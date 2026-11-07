# Grok — lote 2: custos do posto e comida que fecha ciclo

Base: 0.10.0 (`ed75b87`), com o posto do lavrador do Cloud e o lote 1 já integrados. Medições com o substituto offline (`python tools/check_rust.py`). Não alegam FPS no Horizon.

## Mapa de custos (depois do lote 1, antes deste)

| Caminho | Quando | Custo observado |
|---|---|---|
| `farmhand::next` | cada 10 ticks, sem alvo | 81 `Host::block` (camada 9×9 inteira) |
| `Work::can_use` | todo aldeão, todo tick | `ORDERS.lock()` mesmo sem nenhum posto com ordem |
| `work` | cada ciclo do posto | `carried()` monta um `Vec` só para saber se há carga |
| `has_item_inputs` | curva de esteira | `farm_post` caía em `production::has_inputs` → `false` |

A colheitadeira lê 3 blocos por tick (volta em 81 ticks). O posto ocioso lia 81 blocos a cada 10 ticks: um pico 27 vezes maior, e a volta vazia em 30 ticks — mais rápido que a máquina, no pior lugar.

## Correções (com evidência)

1. **Varredura incremental.** `SCANS` 81 → 9 (uma fileira). Volta completa em 27 ciclos / 270 ticks. Teste: `idle_farmhand_scans_one_row_per_cycle` (9 `block()` por ciclo ocioso, não 81).
2. **Mutex das ordens.** `ORDERS_PENDING` (`AtomicBool`): `Work::can_use` não disputa o mapa quando nenhum posto tem ordem viva. `cancel` só baixa a flag se o mapa esvaziar. Teste existente: `work_goal_only_runs_with_a_fresh_order`.
3. **Carga.** `has_load` olha se a lista `Carried` existe e não está vazia; o parse completo fica para a entrega.
4. **Curva de esteira.** `has_item_inputs` trata postos de trabalhador como entrada. Testes: `farm_post_counts_as_item_input_for_belt_curves`, `belt_curves_into_a_farm_post`.

Troca perceptível da varredura: declarar “nada maduro” e notar uma planta que acabou de amadurecer no fim da fila leva ~13,5 s em vez de 1,5 s. A área 9×9×3, o alcance e o ciclo de 10 ticks não mudam. O aldeão ainda gasta a maior parte do tempo andando. Documentado aqui, separado da correção do mutex e da curva.

## Balanceamento

Problema demonstrado nos números, não no jogo:

- Colheita de trigo devolve 1 trigo (valor 1) + 1 semente. Com 1 ponto por colheita, o trigo cru paga a próxima colheita: ciclo fechado, moinho e cozinha inúteis para o posto.
- Cenoura e batata devolvem 2 unidades (valor 1 cada). Com custo 2, duas raízes cruas ainda pagam uma colheita.

Depois: `TASK_COST = 2`. O trabalhador come até cobrir o custo (`feed`). O painel mostra `Ration / 2` colheitas restantes (`Info`).

| Comida | Valor | Colheitas pagas |
|---|---|---|
| Trigo, beterraba | 1 | 0 (sobra 1 ponto) |
| Duas cenouras ou batatas | 2 | 1 (empate, ainda sem cozinha) |
| Batata assada | 4 | 2 |
| Pão | 6 | 3 |
| Carne assada | 8 | 4 |

Testes: `raw_wheat_does_not_pay_a_full_harvest` (1 trigo não colhe; o segundo fecha); `farmhand_walks_harvests_and_carries_the_crop_home` (pão → `Info` 3, depois da colheita 2); `two_carrots_cover_one_harvest`.

Não alterados: contratação (1 esmeralda), área, tempos de desistência/ausência, valores de `production.json`, vazão das esteiras, Mk.III vs esteira comum (hipótese do lote 1). Coordenador continua com o Cloud.

## Teste pelo usuário

1. Esteira apontando para o posto: a curva vira sozinha, como nas outras máquinas de entrada.
2. Pão no slot de comida: o painel mostra 3 colheitas; depois da primeira, 2.
3. Só um trigo no slot: estado “Trabalhador sem comida”, planta intacta.
4. Posto ocioso no campo vazio: “Nada maduro” aparece depois de uma volta (~13 s), não em 1,5 s.

Sem migração de save (`Ration` continua em pontos de comida; só o gasto por colheita muda). Sem bump de versão.
