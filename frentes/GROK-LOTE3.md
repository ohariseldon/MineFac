# Grok — lote 3: postos da 0.11.0, comida e varredura

Base: 0.11.0 (`4f1f984`), com carregador e capataz. O lote 2 foi escrito sobre a 0.10.0 e ficou em conflito; este lote leva a calibração e os cortes de tick para a interface nova (`Plan`, reservas, três postos). Medições com o substituto offline. Não alegam FPS no Horizon.

## Mapa de custos (0.11.0, antes)

| Caminho | Quando | Custo observado |
|---|---|---|
| `farmhand::plan` | cada 10 ticks, sem alvo | 81 `Host::block` (camada 9×9) |
| `hauler::plan` | cada 10 ticks até a 1ª volta | 64 `block()`; área 13×13×3 = 507 |
| `foreman::plan` | cada 10 ticks até a 1ª volta | 150 `block()`; área 25×25×3 = 1875 |
| `Work::can_use` | todo aldeão, todo tick | `ORDERS.lock()` mesmo sem ordem |
| `reserved_by_other` | cada célula/estação da busca | `RESERVED.lock()` mesmo vazio |
| `work` | cada ciclo | `carried()` monta um `Vec` só para saber se há carga |
| `has_item_inputs` | curva de esteira | postos caíam em `production::has_inputs` → `false` |

## Correções

1. **Lavrador.** `SCANS` 81 → 9. Teste: `idle_farmhand_scans_one_row_per_cycle`.
2. **Carregador.** `SCAN` 64 → 16 (uma fileira do 13×13). Primeira volta 8 ciclos (~0,8 s) → 32 ciclos (~3,2 s). Teste: `idle_hauler_scans_one_row_per_cycle`.
3. **Capataz.** `SCAN` 150 → 25 (uma fileira do 25×25). Primeira volta ~13 ciclos (~1,3 s) → 75 ciclos (~7,5 s). Teste: `idle_foreman_scans_one_row_per_cycle`.
4. **Mutexes.** `ORDERS_PENDING` e `RESERVED_PENDING`: aldeão ocioso e busca sem reserva não disputam os mapas.
5. **Carga.** `has_load` olha a lista `Carried` sem montar o `Vec`.
6. **Curvas.** `has_item_inputs` trata os três postos como entrada. Testes: `workforce_posts_count_as_item_input_for_belt_curves`, `belt_curves_into_a_farm_post`.

Trocas perceptíveis (separadas dos mutexes): lavrador declara “nada maduro” em ~13,5 s; carregador e capataz demoram a primeira descoberta da vizinhança. Área, raio, teto de 12 máquinas / 6 postos e ciclo de 10 ticks não mudam.

## Balanceamento

Cada tarefa concluída (colheita, viagem com entrega, recontratação) gasta `TASK_COST = 2`. O trabalhador come até cobrir (`feed`). O painel mostra `Ration / 2` tarefas restantes.

| Comida | Valor | Tarefas pagas |
|---|---|---|
| Trigo, beterraba | 1 | 0 |
| Duas cenouras ou batatas | 2 | 1 (empate) |
| Pão | 6 | 3 |
| Carne assada | 8 | 4 |

Testes: `raw_wheat_does_not_pay_a_full_harvest`; ciclo do lavrador (`Info` 3 → 2); carregador desconta 2 por viagem (ração 10 → 8).

O capataz ainda reabastece abaixo de 4 pontos (2 tarefas). Não alterados: 16/32 itens por viagem, TOP_UP 16, contratação, valores de `production.json`, vazão das esteiras, Mk.III vs esteira comum.

## Teste pelo usuário

1. Esteira para o posto: a curva vira sozinha.
2. Pão no lavrador: painel mostra 3 tarefas; depois da primeira, 2.
3. Um trigo só: “Trabalhador sem comida”.
4. Carregador novo: a primeira viagem espera a volta da varredura (~3 s).
5. Capataz novo: os postos entram na lista em ~7 s.

Sem migração de save. Sem bump de versão (continua 0.11.0).
