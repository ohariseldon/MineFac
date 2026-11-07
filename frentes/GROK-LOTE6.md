# Grok — lote 6: zonas da 0.13.0 e registros

Base: 0.13.0 (`d01f57e`), com zonas, celeiro, silo, centro de controle e os lotes 4 e 5 já integrados. O merge da 0.13.0 tirou o `DESKS_PENDING` do lote 4; este lote devolve e corta o pico novo da lavoura. Medições com o substituto offline. Não alegam FPS no Horizon.

## Mapa de custos (0.13.0, antes)

| Caminho | Quando | Custo observado |
|---|---|---|
| `farmhand::plan` numa zona 13×13/21×21 | cada 10 ticks, fileira de grama seca | `zone.side` `block()` **e** até 32 `block()` de `watered` **por** célula de terra |
| `zones_near` / `zone_for` | cada ciclo de lavrador, pastor e centro | `ZONES.lock()` mesmo sem marco |
| `stores_near` / `posts_near` / `claim_task` | centro e carregador | mutex mesmo sem celeiro, posto publicado ou mesa |
| `register` / `report` / `STORES` | a cada publicação | `retain` de toda a lista + `push`, em vez de atualizar o mesmo bloco |

Uma lavoura nível 2 só de grama sem água era o pico: 13 células × (1 + 1 ar + 32 água) ≈ 400 `block()` a cada 10 ticks. Sem zona, o lote 3 já limitava a 9.

## Correções

1. **Lavrador.** Na zona, a fileira continua `SCANS = 9` (a área 9/13/17/21 não muda). Um `watered` por ciclo. Teste: `idle_farmhand_in_a_dry_field_scans_one_row` (≤50 `block()`, não centenas).
2. **Registros.** `ZONES_PENDING`, `STORES_PENDING`, `POSTS_PENDING` e de novo `DESKS_PENDING`: consulta vazia não disputa o mutex. Atualização no mesmo bloco, sem reconstruir a lista.
3. Testes da zona grande: a primeira volta de um 13×13×3 com 9 células/ciclo cabe em 80 ciclos (~8 s), não em 40.

Troca perceptível: a primeira volta de uma lavoura 21×21 leva ~14,7 s em vez de ~6,3 s. Arar um bloco seco no fim da fileira espera o próximo ciclo (o aldeão anda mais que isso). Raio da zona, teto de 30 aldeões, metas do centro e vazão não mudam.

## Balanceamento

Sem mudança de `TASK_COST`, comida, custos de nível da zona ou vazão. Mk.III vs esteira comum continua hipótese para o jogo.

## Teste pelo usuário

1. Lavoura nível 2 só de grama, sem água: o lavrador declara “nada a fazer” sem travar o tick; com água a 4 blocos, ara em poucos segundos.
2. Trigo longe do posto, dentro da zona: ainda colhe (pode levar ~8 s na primeira volta).
3. Carregador sem mesa: comportamento próprio, como no lote 4.

Sem migração de save. Sem bump de versão (continua 0.13.0).
