# Grok — lote 5: fila do capataz

Base: 0.12.0 (`f8b0bfd`). O lote 4 (inspeções do carregador e NBT do pastor) segue em PR separado e não está nesta branch; este lote não depende dele. Medições com o substituto offline. Não alegam FPS no Horizon.

## Mapa de custos (antes)

| Caminho | Quando | Custo observado |
|---|---|---|
| `foreman::organize` | a cada 40 ticks, rede cheia | 24 `change_mod_block_entity` para fontes **e de novo** 24 para destinos |
| `foreman::plan` | cada 10 ticks, depois da 1ª volta | até 24 `block()` só para saber quais nós da Área são postos, depois até 6 `look` |
| `hauler::plan` / pastor | lote 4 | fora deste PR |

Uma mesa no meio de uma linha relia cada máquina duas vezes para montar a fila, e o capataz relia a Área inteira a cada ciclo só para achar os 6 postos.

## Correções

1. **Fila.** Um recorte da rede: uma leitura por bloco. Fontes e destinos saem dessa cópia. Teste: `queue_reads_each_network_block_once_per_refresh` (8 edições, não 16).
2. **Postos.** A lista `Posts` sai do mesmo recorte. O ciclo seguinte não chama `block()` na Área para filtrar. Teste: `idle_foreman_inspects_a_few_posts_per_cycle` (25 `block()` da varredura, não 25+24).
3. **Capataz.** `LOOKS = 3` inspeções de posto por ciclo, no mesmo rodízio de `Pick`. Posto sem trabalhador ou sem comida continua tendo prioridade dentro do lote.

Troca perceptível: notar um posto recém-faminto entre 6 pode levar até 2 s. A fila continua a ser refeita a cada 40 ticks; o recorte tem o estado daquele instante, como as duas passadas antigas. Raio 12, teto de 24 blocos / 6 postos / 8 tarefas e o ciclo de 10 ticks não mudam.

## Balanceamento

Sem mudança de `TASK_COST`, comida, vazão ou limiar de reabastecimento (`LOW_FOOD` 4). Mk.III vs esteira comum continua hipótese para o jogo.

Não alterados: varredura de 25 blocos/ciclo até a primeira volta (~7,5 s), inspeções do carregador e NBT do pastor (lote 4).

## Teste pelo usuário

1. Fila já conhecida (trigo do lavrador → moinho; esmeraldas da despensa → mesa): o carregador ainda pega a tarefa.
2. Seis postos ociosos: o capataz declara “Nada a fazer”; ao acabar a comida de um, reage em no máximo ~2 s.
3. Máquina nova na rede: entra na fila na próxima volta da Área (~7,5 s) e no recorte seguinte (até 40 ticks depois).

Sem migração de save. Sem bump de versão (continua 0.12.0).
