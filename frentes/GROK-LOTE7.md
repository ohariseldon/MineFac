# Grok — lote 7: tick sem alocação repetida

Base: 0.14.0 (`a2f2d99`), com o lote 6 já integrado. Medições com o substituto offline. Não alegam FPS no Horizon.

Nada de raio, vazão, comida ou frequência muda. O jogador vê a mesma fábrica; o tick deixa de montar as mesmas strings.

## Mapa de custos (antes)

| Caminho | Quando | Custo observado |
|---|---|---|
| `number` / `set_number` | cada máquina, cada tick, várias vezes | `JavaString` novo (UTF-16) por chave: `"Status"`, `"energy"`, … |
| `process` | cada máquina de receita, cada tick | `"Recipe".into()` + `to_string_lossy` da receita atual |
| `publish` | cada esteira, a cada tick | `String` do item mesmo quando a carga não mudou |
| `visual_signature` | cada bloco, cada tick | clona o `ItemStack` do primeiro slot, inclusive em máquina que não desenha carga |
| `pollution::update` | cada bloco, no tick múltiplo de 64 | mutex da dimensão, mesmo depois que o primeiro bloco já difundiu |
| `ranch::recent` / `defense::recent` | cada consulta | `retain` do mapa inteiro, mesmo com a lista já filtrada por `FRESH` |

## Correções

1. **Chaves NBT.** As chaves do tick ficam prontas na primeira leitura e são reaproveitadas. `set_number` só grava quando o valor muda. Comparar um `JavaString` com texto não monta `String`.
2. **Esteira.** O nome da carga só é copiado quando o item ou a quantidade mudam.
3. **Sincronia visual.** O clone do item entra na assinatura só de esteira, divisor e unificador — os blocos que desenham a carga. Luzes continuam a depender de `Status`, `Module` e `Info`.
4. **Poluição.** Overworld, Nether e End têm cada um o seu passo. No tick da difusão, a primeira chamada trabalha; as outras não pegam o mutex. O estado por dimensão (`world.updated`) continua valendo.
5. **Registros de mobs.** A limpeza das marcas velhas roda no máximo uma vez a cada 40 ticks. Quem consulta continua vendo só o que está dentro de `FRESH` (60 ticks no rancho, 40 na Tesla). Um mob novo entra no mesmo tick.

## O que não muda

Área das zonas, raios, 16/32 itens por viagem, `TASK_COST`, vazão das esteiras, FE por tick, cadência de 10 ticks dos postos, aviso e onda de poluição. Sem bump de versão (continua 0.14.0). Sem migração de save.

## Teste pelo usuário

A fábrica deve se comportar como na 0.14.0: curvas, energia, trabalhadores, poluição e Tesla. Não há atraso novo para notar. Se uma luz ou um item na esteira atrasar um tick em relação ao painel, é regressão.

## Verificação

- `python tools/check_project.py`
- `cargo +1.90.0 test --offline -p veio-check` — 93 testes.
- Sem compilação pelo launcher e sem teste no jogo.
