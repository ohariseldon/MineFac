# Grok — lote 4: inspeções do carregador e NBT do pastor

Base: 0.12.0 (`f8b0bfd`), com fila do capataz, posto do pastor e o lote 3 já integrado. Medições com o substituto offline. Não alegam FPS no Horizon.

## Mapa de custos (depois do lote 3, antes deste)

| Caminho | Quando | Custo observado |
|---|---|---|
| `hauler::plan` | cada 10 ticks, depois da 1ª volta | até 12 `operations::inspect` + `change_mod_block_entity` |
| `foreman::claim_task` | cada ciclo de cada carregador | `DESKS.lock()` mesmo sem mesa no mundo |
| `shepherd::plan` | cada 10 ticks, inclusive andando | até 12 `entity_nbt` de animais + `item_entities` |
| `foreman::organize` | a cada 40 ticks | lê até 24 blocos da rede de novo (fica para depois) |

O carregador ocioso no meio de uma fábrica era o pico: 12 inspeções a cada 10 ticks, cada uma alocando a vista da estação. O pastor relia o rebanho inteiro enquanto já tinha um alvo.

## Correções

1. **Carregador.** `LOOKS = 3` inspeções por ciclo, em rodízio (`Pick`). Máquina com falta de insumo ou produto continua tendo prioridade dentro do lote inspecionado. Teste: `idle_hauler_inspects_a_few_stations_per_cycle` (3 edições, não 12).
2. **Pastor.** `MAX_READS` 12 → 4 NBT por ciclo, em rodízio (`Herd`). A população (teto do rancho) continua sendo a lista no raio, sem NBT. Com alvo de tosquia ou ração, só relê esse animal. Ovos no chão continuam primeiro. Teste: `walking_shepherd_follows_the_target_without_rereading_the_herd`.
3. **Fila.** `DESKS_PENDING`: carregador sem mesa por perto não disputa o mutex nem parseia `Queue`.

Troca perceptível: notar uma máquina recém-parada entre 12 pode levar até 4 ciclos (2 s) em vez de 10 ticks. Raio 6, teto de 12 máquinas, 16/32 itens por viagem e o ciclo de 10 ticks não mudam. Tosquiar uma ovelha no fim da fila do rebanho pode esperar a volta de 4 NBT (ainda segundos, o aldeão anda mais que isso).

## Balanceamento

Sem mudança de `TASK_COST`, valores de comida ou vazão. Ovos (1 ponto) não pagam a tarefa (2). Lã não é comida. Ração do pastor é item de reprodução, não fecha ciclo com a colheita.

Não alterados: fila do capataz (recalc a cada 40 ticks, até 24 blocos), Mk.III vs esteira comum, limiar de reabastecimento.

## Teste pelo usuário

1. 12 prensas ociosas em volta de um carregador com estoque vazio: o aldeão fica em “Nada a fazer”; ao faltar cobre numa delas, ele reage em no máximo ~2 s.
2. Pastor andando até uma ovelha: não abandona o alvo porque há vacas no caminho.
3. Carregador longe de qualquer mesa: comportamento próprio, sem custo da fila.

Sem migração de save. Sem bump de versão (continua 0.12.0).
