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

`check_performance.py`: "checks identical" em todos os commits. Os 7 cenários (8 desde a 0.26.0, com o ambiente: veja [VISUAL-E-CLIMA.md](VISUAL-E-CLIMA.md)) continuam iguais ou melhores dentro do ruído; `nbt_keys` cai porque o painel lê menos chaves.

## Trocas

- Esteira parada com bússola ou relógio como carga: o ponteiro fica parado enquanto ela está parada (efeito do `keep`, como já acontece nas máquinas).
- Nenhuma outra troca perceptível.

## O que ainda custa mais

1. **Sincronias das esteiras** (22 de 28,6 por tick). Na 0.17.2 a espera da vez passou a contar como andando (1), a pedido do usuário: a luz não pisca mais laranja a cada item. Medido depois: as sincronias continuam em 28,6 por tick, porque a esteira ainda vai de vazia (0) a carregada e volta, e o item desenhado muda junto. Para cortar mais, só deixando de mostrar a carga, o que não vale a pena.
2. Postes: 393 `change_mod_block_entity` por tick (distribuição a cada consumidor, mesmo cheio) e a reconstrução dos fios a cada 40 ticks, que relê os postes vizinhos.
3. `neighbor_signal` em todo consumidor, todo tick (resposta da redstone em 1 tick).
4. Buscas restantes no NBT: `set_number_at` (12%) e a primeira assinatura visual (10%).
5. `neighbor_changed` existe no Horizon e poderia substituir a releitura de 13 blocos das esteiras. Não cobre as diagonais de rampa e exige um gancho novo no substituto; fica para depois.

## Depois da consolidação (PR #47, 8 de novembro de 2026)

Medido no checkout de `claude/project-thread-l94frq` (`80980db`), Linux x86_64, Rust 1.97,
callgrind sobre `bench_factory` (480 blocos, 600 + 4000 ticks). O resumo
`CHECK factory_digest=6241941769725dda gears=1024` é o mesmo antes e depois de cada mudança.

| Medida | Antes | Depois |
|---|---|---|
| Instruções (callgrind) | 17,88 bi | **16,51 bi (−7,7%)** |

A consolidação subiu o custo de 12,58 bi para 17,88 bi (+42%): sons, censo do quadro,
holding, indústria, medidor de produção e poluição passaram a rodar em todo bloco, todo tick.

1. **Sons só perto de alguém.** `soundscape::after` lia o NBT de todo bloco antes de saber se
   havia ouvinte. Agora pergunta primeiro se algum jogador está a até 12 blocos (a mesma conta
   que `admit` já fazia para recusar o som). Drones seguem o caminho antigo, porque o som sai de
   onde eles voam. Nenhum som que tocava deixa de tocar; o teste
   `far_blocks_are_skipped_before_reading_nbt_exactly_where_admit_would_refuse` confere isso.
2. **Mural da casa mais lento quando ninguém olha.** Eram 256 leituras de bloco a cada 2 ticks,
   para sempre, por casa. Sem jogador a 24 blocos, passa a uma linha por segundo
   (`IDLE_SCAN_EVERY`, 13 leituras por tick). Com alguém perto, volta ao ritmo de antes.
3. **Medição mais fiel.** `change_mod_block_entity` do mundo simulado agora copia e compara os
   itens do alvo, como o Horizon faz em `native_with_mod_block_entity`. Custa +1,4% no total
   medido; os números de antes e depois acima não incluem essa cópia.

### Escala e memória (`bench_factory_scale`)

```
cargo test --offline --release -p veio-check bench_factory_scale -- --ignored --nocapture --test-threads=1
```

A mesma linha de produção repetida. `memoria_mb` são bytes vivos depois do aquecimento,
incluindo os mapas do mundo simulado: é um teto para o que o Veio ocupa.

| Blocos | µs por tick | µs por bloco | % de um tick de 50 ms | Memória |
|---|---|---|---|---|
| 480 | 329 | 0,69 | 0,7% | 0,8 MB |
| 4.080 | 3.731 | 0,91 | 7,5% | 6,9 MB |
| 16.080 | 17.328 | 1,08 | 34,7% | 27,4 MB |

Conclusões para um PC com 8 GB:

- **Memória não é o gargalo do Veio**: cerca de 1,8 KB por bloco, 27 MB para 16 mil blocos.
  A RAM do jogo vem dos chunks carregados e da renderização do Horizon, não da simulação.
- **CPU é**: uma fábrica de 16 mil blocos carregados usa cerca de um terço do tick nesta
  máquina de nuvem (ruidosa). O µs por bloco sobe com o tamanho por cache, não por algoritmo:
  as instruções por bloco ficam quase iguais (8,1 mil a 480 blocos, 8,9 mil a 4.080).

### Custo por tipo de bloco (480 blocos, medição temporária)

| Bloco | ns por tick | Quantidade |
|---|---|---|
| poste | 1.358 | 80 |
| extratora | 1.325–1.366 | 16 |
| prensa | 1.235 | 16 |
| montadora | 1.114 | 16 |
| gerador | 939 | 16 |
| esteira | 690–706 | 256 |
| solar | 476 | 16 |
| cabo | 258 | 64 |

Postes e esteiras somam quase 90% do tempo. Próximos passos, que mudam o comportamento e
por isso precisam de decisão antes:

- **Postes** chamam `change_mod_block_entity` para cada bloco da área em todo tick, mesmo
  cheio ou com o poste vazio. No Horizon, cada chamada copia os itens do alvo e marca o chunk
  como modificado. Distribuir a cada N ticks com N vezes a vazão, ou guardar o tipo de cada
  alvo junto da área para pular consumidores quando o poste está sem energia, corta isso.
- **Máquinas paradas** (esteira cheia, montadora cheia) refazem todo o tick. Uma máquina que
  não mudou em vários ticks poderia dormir até um vizinho mudar (`neighbor_changed`).

## Segunda rodada: métodos do Factorio (8 de novembro de 2026)

A pedido do Luiz: inspirar-se em como Factorio e Satisfactory deixam fábricas grandes leves.
A produção nunca para por estar longe do jogador; o que muda é não gastar CPU onde o tick
não faria nada, e entregar energia em lotes.

1. **Esteira inativa (como as entidades inativas do Factorio).** Uma esteira comum fora da
   sua fase de transferência, vazia (0), travada com carga (3) ou desligada por redstone (9),
   não muda nada no tick: `update_status` manteria o estado e a transferência só roda na fase.
   Esse tick agora só redesenha a carga (se um vizinho colocou item) e mantém os observadores
   (censo, holding, indústria, som). Resultado **idêntico bit a bit**:
   16,83 → 14,59 bi de instruções (−13%). Divisores, unificadores e túneis seguem completos.
   O painel de uma esteira parada pode atualizar até 7 ticks depois de uma mudança de
   configuração feita por fora; a carga e a luz atualizam na hora.
2. **Rede elétrica em lotes (como a rede do Factorio).** Cada poste atende área e fios a cada
   4 ticks, com fase pela posição, movendo até 4× a vazão. O buffer do poste sobe na mesma
   proporção (madeira 1.000 → 4.000 FE, aço 4.000 → 16.000 FE) para a vazão por tick não cair.
   As máquinas guardam 2.000 FE e recebem o mesmo total. Chamadas ao mundo por tick:
   393 → 170. Instruções: 14,59 → **13,18 bi** (−22% sobre a medição fiel de 16,83 bi).
   Muda o resumo da fábrica (energia guardada e fase de produção de poucos ticks); a
   produção é a mesma: 1.024 engrenagens nos dois casos, prensas 150 contra 148 itens,
   extratoras 228 contra 229. Os geradores queimam 3 carvões a mais para encher os buffers
   maiores, energia que fica guardada nos postes.

Escala depois das duas mudanças:

| Blocos | µs por tick | µs por bloco | % de um tick de 50 ms | Memória |
|---|---|---|---|---|
| 480 | 291 | 0,61 | 0,6% | 0,8 MB |
| 4.080 | 3.118 | 0,76 | 6,2% | 6,9 MB |
| 16.080 | 13.653 | 0,85 | 27,3% | 27,4 MB |

`check_performance.py` voltou aos controles históricos (`power_energy=129500
callbacks=33792`) com os cenários de poste rodando num tick de entrega do lote.
`prices_follow_local_supply_and_demand` falhou uma vez numa execução completa e passou em
todas as outras, isolado ou junto; parece depender da ordem dos testes que dividem estado
global da vila, e não toca no código desta mudança.
