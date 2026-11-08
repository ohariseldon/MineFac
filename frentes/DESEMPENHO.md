# Desempenho: fábrica fluida e leve

Base: 0.16.1 (`a5f0eb5`), branch `perf/fluid`. Medições no substituto offline; não alegam FPS no Horizon. Nada de alcance, ritmo, receita, energia ou save muda: a fábrica simulada termina no mesmo estado, bit a bit, antes e depois de cada commit.

## Como medir

```
cd native/offline-check
cargo test --offline --release -p veio-check bench_factory -- --ignored --nocapture
cargo test --offline --release -p veio-check bench_frame -- --ignored --nocapture
cargo test --offline --release -p veio-check bench_large_village -- --ignored --nocapture
python3 tools/check_performance.py        # "checks identical"
```

- `bench_factory` (novo, `native/api/src/perf/factory.rs`): 480 blocos (16 linhas de extratora comum ou Mk.III → 8 esteiras → prensa → 8 esteiras → montadora, 5 postes e um gerador por linha, 64 cabos, 16 solares) pelo **mesmo** `block_entity_tick` do jogo, 600 ticks de aquecimento + 4000 medidos. Imprime µs por tick, chamadas ao Host por tick, sincronias por tipo e `CHECK factory_digest`: um resumo de NBT, itens, painel e propriedades de todos os blocos. Se uma mudança alterar o jogo, o resumo muda. O resumo ignora `PathSig`, que é só um cache.
- `bench_frame` (novo): desenha a mesma fábrica 3 vezes por tick e reaproveita o que pediu `keep` até a próxima sincronia, como o jogo faz.
- Instruções (callgrind): `valgrind --tool=callgrind <binário de teste> bench_factory --ignored`. É o número estável; o relógio desta máquina oscila ±20%.

## Mapa de custos (antes)

Fábrica simulada, callgrind, 19,67 bilhões de instruções no total:

| Caminho | Parte | Por quê |
|---|---|---|
| `nbt_key` | 19% | `match` em ~75 textos a cada leitura de NBT (≈45 leituras por bloco por tick) |
| `presentation::publish` | 23% (com filhos) | 12 buscas no mapa por bloco, todo tick |
| `visual_signature` | 15% | 2×3 buscas e 2 clones do `ItemStack` da carga por tick |
| `power::tick` / reconstrução dos fios | 26% | 49 consultas SipHash por chamada de `versions` |
| `refresh_path` das esteiras | 9% | metade das esteiras nunca acertava o cache (abaixo) |

Host por tick: 844 `block()`, 123 `block_property()`, 393 `change_mod_block_entity()`, 80 `neighbor_signal()`, 28,6 sincronias, 1151 alocações.
Quadro: 257 blocos redesenhados, 1223 alocações, 67 µs.

## Mudanças

1. **Medição** (`669674b`). O corpo do gancho vira `block_entity_tick(host, pos, entity)`, sem mudar nada, para as medições usarem o mesmo caminho.
2. **Sincronia pela carga desenhada** (`93e63ed`). O desenho da esteira lê só o nome da carga (`DisplayItem`). `publish` agora avisa quando o nome muda. Mudar só a quantidade já não pede sincronia, e a assinatura não clona mais o `ItemStack`. `DisplayCount` continua no NBT.
3. **Chaves resolvidas na compilação** (`d0ee742`). `nbt_key`, `number` e `set_number` são `#[inline(always)]`. Com o nome literal, o `match` some na compilação. A busca no mapa (`number_at`) e a chave rara (`rare_nbt_key`, fria) ficam fora, para o código não crescer em cada chamada (+2% no binário de teste).
4. **Quadro e tick sem alocações** (`9c94c55`).
   - Esteira bloqueada (3) ou desligada (9) não anda; agora pede `keep`.
   - O item da carga e o do filtro vêm de um cache por nome (`with_drawn_item`).
   - `generation::draw` não copia a orientação de toda máquina desenhada.
   - As chaves montadas com `.into()` a cada tick vão para a tabela fixa: `Info` dos sensores e da despensa, `BusyAt`, `Worker`, `Carried`, `Held`, `Villagers` e outras.
   - O substituto ganha `BlockEntitySubmitEvent::preview` e `preview_kept`, só para medir.
5. **Assinatura pelo painel e hash rápido** (`84ca145`).
   - A segunda assinatura do tick lê `data[0]`, `data[11]` e `data[12]`, que `publish` acabou de copiar. Cabos, postes, Tesla, centro, marcos e braços continuam lendo o NBT.
   - O registro de postes por chunk e a fila do `budget` usam `FastHash`, um hash multiplicativo, no lugar do SipHash. Esses mapas só são consultados por chave.
6. **Painel numa passada** (`a433cb4`, teste em `6f0b970`). `numbers_sorted` percorre o `BTreeMap` uma vez com as 14 chaves do painel em ordem, em vez de 12 buscas.
7. **Uma leitura de `facing` por transferência** (`7e6f13a`). `refresh_path` devolve a saída; a transferência não relê.
8. **Cache da esteira com hash negativo** (`ce64fff`). Bug antigo: `PathSig` é um i32 e `number()` devolve no mínimo 0, então metade das esteiras nunca acertava o cache do lote 1 e refazia entradas a cada 8 ticks. A comparação agora usa o valor guardado. A assinatura passa a incluir a orientação dos vizinhos do Veio, que as entradas leem. Assim, girar uma esteira vizinha com a chave continua refazendo a curva, agora em todas as esteiras (antes, só por acaso na metade sem cache).

## Resultado

| Medida (fábrica simulada) | Antes | Depois |
|---|---|---|
| Instruções (callgrind, 4600 ticks) | 19,67 bi | **12,58 bi (−36%)** |
| µs por tick (mediana de 5, máquina ruidosa) | ~285 | ~190–225 |
| `block()` por tick | 844 | **694** |
| `block_property()` por tick | 123 | 100 |
| Alocações por tick | 1151 | 936 |
| Sincronias por tick | 28,6 | 28,6 (veja abaixo) |
| Quadro: blocos redesenhados | 257 | **224** |
| Quadro: alocações | 1223 | **0** |
| Quadro: µs | ~67 | ~37 |
| Vila grande (150 + 300) | 3,2 µs/tick | 3,3 µs/tick (sem mudança; já estava barata) |

`check_performance.py`: "checks identical" em todos os commits. Os 7 cenários continuam iguais ou melhores dentro do ruído; `nbt_keys` cai porque o painel lê menos chaves.

## Trocas

- Esteira parada com bússola ou relógio como carga: o ponteiro fica parado enquanto ela está parada (efeito do `keep`, como já acontece nas máquinas).
- Nenhuma outra troca perceptível.

## O que ainda custa mais

1. **Sincronias das esteiras** (22 de 28,6 por tick). Na 0.17.2 a espera da vez passou a contar como andando (1), a pedido do usuário: a luz não pisca mais laranja a cada item. Medido depois: as sincronias continuam em 28,6 por tick, porque a esteira ainda vai de vazia (0) a carregada e volta, e o item desenhado muda junto. Para cortar mais, só deixando de mostrar a carga, o que não vale a pena.
2. Postes: 393 `change_mod_block_entity` por tick (distribuição a cada consumidor, mesmo cheio) e a reconstrução dos fios a cada 40 ticks, que relê os postes vizinhos.
3. `neighbor_signal` em todo consumidor, todo tick (resposta da redstone em 1 tick).
4. Buscas restantes no NBT: `set_number_at` (12%) e a primeira assinatura visual (10%).
5. `neighbor_changed` existe no Horizon e poderia substituir a releitura de 13 blocos das esteiras. Não cobre as diagonais de rampa e exige um gancho novo no substituto; fica para depois.
