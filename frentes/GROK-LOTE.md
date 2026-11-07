# Grok — lote 1: diagnóstico, desempenho e energia

Base: 0.9.1 + coordenação (`9e8b601`). Medições com o substituto offline (`python tools/check_rust.py`). Não alegam FPS no Horizon.

## Mapa de custos (antes)

| Caminho | Quando | Custo observado |
|---|---|---|
| `block_entity_tick` | cada máquina, cada tick | poluição, postes, logística, produção, `publish` |
| Esteiras | a cada 8 ticks | `incoming` em 4 lados × 3 alturas + `belt_target` + `layout` alocado |
| Cabos / geradores | cada tick | 6× `change_mod_block_entity` mesmo com 0 FE |
| Poluição `update` | cada máquina, cada tick | `rem_euclid`; a cada 64 ticks, N máquinas disputam o mutex |
| Solar | cada tick | `day_time` + `light` + `weather` |
| `production::tick` / `produce` | cada tick | clone de `entity.id`; `Sense` escrito em queimadores que não usam sensor |
| `publish` | cada tick | `Vec` novo + `DisplayItem` em máquinas que não desenham carga |
| Postes, fazenda, rancho, Tesla | já escalonados | sem mudança neste lote |

Por quadro: desenho de esteiras anima; fios e rotores usam `keep`/`spin`. Sem medição de GPU aqui.

## Correções (com evidência)

1. **Poluição.** O tick do mundo também chama `update`. Por dimensão, `world.updated` evita um segundo passo no mesmo `game_time` (Overworld e Nether não compartilham o atalho).
2. **Cabos ociosos.** Sem carga depois de gerar, não há visita a vizinhos. Teste: `idle_cables_skip_neighbor_lookups` (0 visitas vazias, 6 com 80 FE).
3. **Curvas de esteira.** Assinatura da vizinhança (`PathSig`). Segunda passagem na mesma linha: só a assinatura (≤14 `block()`), sem reescrever `path`. Vizinho novo invalida. Teste: `stable_neighborhood_skips_incoming_scans`.
4. **Sensores.** Água, vento, vapor e solar relêem o mundo a cada ~40 ticks. Queimadores a combustível não escrevem `Sense`.
5. **Painel.** Reusa `entity.data`; `DisplayItem` só em esteiras, divisor e unificador, e só se a carga mudou.
6. **Outros.** Sem clone de id no tick de produção; `has_item_inputs` sem alocar `layout`; lâmpada sem energia não varre a lavoura.

Limitações: consultas `Host::block` ainda alocam `String` (API do Horizon). A assinatura da esteira relê 13 blocos a cada 8 ticks para não congelar curvas; a resposta a uma esteira nova continua em até 0,4 s. O substituto offline não mede FPS.

Conservação: unificador, rampas, combustível com buffer cheio e colheita sem espaço continuam nos testes já existentes.

## Balanceamento (lote pequeno)

Problema demonstrado nos números, não no jogo:

- Uma linha inicial (2 extratoras a 6 FE/t + 2 prensas a 8 FE/t) pede 28 FE/t.
- Antes: 1 face de roda = 6 FE/t; cata-vento no nível do mar = 1 FE/t; biomassa = 30 FE/t contra carvão = 40 FE/t. Água e vento não fechavam a linha sem dezenas de blocos; biomassa era carvão pior.
- Depois: 8 FE/t por face de água (máx. 32); 8 FE/t de vento no nível do mar (y 110 continua 16, teto 32); biomassa 40 FE/t, mesma energia por combustível, poluição 2 contra 6 do carvão.

Exemplo: 2 rodas com 2 faces cada = 32 FE/t cobrem a linha. 4 cata-ventos no spawn também. Nenhum dos dois chega aos 120 FE/t da caldeira. Carvão continua mais simples de abastecer; renovável ganha terreno e poluição.

Não alterados neste lote: vazão das esteiras (2,5 / 5 itens/s), tempos de receita, pureza, módulos, contratos, comida, capacidade NBT. Mk.III pura (4 itens/s) continua acima da esteira comum — hipótese para o usuário: a esteira rápida deve acompanhar a Mk.III.

### Trabalhadores (só parâmetros, sem implementar)

O Cloud ainda não entregou o sistema. Números para quando a interface existir, não uma segunda IA:

| Parâmetro | Proposta | Por quê |
|---|---|---|
| Desbloqueio | avanço de lavoura (depois da colheitadeira) | o jogador já operou a fazenda |
| Custo de atribuição | 1 pão (consumido uma vez) | usa a cadeia trigo → moinho → cozinha |
| Ritmo | 1 ação agrícola a cada 40 ticks | mais lento que a colheitadeira (3 varreduras/tick), substitui clique, não a máquina |
| Posto inicial | semeadora ou colheitadeira | um ciclo completo visível |
| Coordenador | 1 para até 8 trabalhadores | fila curta; custo 1 circuito + 1 módulo, a discutir com o Cloud |
| Energia do posto | a mesma da máquina | não criar um segundo orçamento de FE |

## Teste pelo usuário

1. Fábrica 0.9.1 existente: curvas e rampas ainda viram sozinhas ao colocar uma esteira vizinha.
2. Cabo/gerador parado: máquinas vizinhas ainda recebem quando o gerador tem combustível.
3. Duas rodas d'água com duas faces de fluxo cada uma: 2 extratoras + 2 prensas aguentam.
4. Cata-vento no y 64 com céu aberto: ~8 FE/t no painel.
5. Gerador a biomassa: 40 FE/t, fumaça menor que o de carvão.

Sem migração de save. Sem bump de versão.
