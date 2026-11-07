# Trabalhadores — primeira entrega (frente Cloud)

Primeira etapa da ordem em [frentes/CLOUD.md](frentes/CLOUD.md): um aldeão contratado trabalha numa lavoura. Integrada na 0.10.0 com a interface de postos do Codex e o lote 1 do Grok.

## Para o jogador

1. Obtenha a colheitadeira. O avanço **Lavoura mecanizada** passa a liberar a receita do **Posto do lavrador** (composteira, enxada de ferro, esmeralda e barril, em qualquer posição).
2. Coloque o posto no meio de uma lavoura de até 9x9, como a colheitadeira. A camada do posto, a de baixo e a de cima são lidas.
3. Ponha comida no slot da esquerda. O valor é o do armazém, e cada colheita gasta **2 pontos**: um trigo não paga a colheita; duas cenouras ou batatas pagam uma; pão (6) paga 3; carne assada (8) paga 4; torta de abóbora (10) paga 5.
4. Com uma **esmeralda** na mão, clique com o botão direito no posto. O aldeão adulto livre mais próximo, a até 8 blocos, é contratado e a esmeralda é gasta. Sem candidato, a esmeralda fica com o jogador.
5. O trabalhador anda até uma planta madura, colhe (a planta fica replantada no estágio inicial, como na colheitadeira) e leva o produto de volta ao posto. Retire pelos três slots da direita ou ligue uma esteira no bocal.
6. **Agachar + chave** no posto dispensa o trabalhador. Ele volta ao comportamento normal de aldeão.

Plantas colhidas: trigo, cenoura, batata, beterraba, fungo do Nether, bagas doces e cacau. Cana, cacto, bambu, abóbora e melancia continuam com a colheitadeira.

### O que continua automático e o que muda

Máquinas industriais, a colheitadeira e as demais máquinas de produção continuam funcionando sem trabalhadores; nada foi desativado. O posto é uma alternativa sem energia elétrica: em vez de FE, ele gasta comida e uma esmeralda de contratação, e é mais lento, porque o aldeão precisa andar. A colheitadeira não substitui o posto de vez nem o contrário: Grok deve calibrar essa escolha.

### Estados do painel

| Estado | Significado |
| --- | --- |
| Sem trabalhador | Ninguém contratado. Use uma esmeralda. |
| Trabalhador a caminho | Andando até a planta ou de volta ao posto. |
| Trabalhando | Colheu ou entregou neste ciclo. |
| Nada maduro | Uma volta completa pela área sem planta madura. O aldeão fica livre (vanilla) até haver trabalho. |
| Trabalhador sem comida | Slot de comida vazio e nenhuma colheita paga. Nada é colhido. |
| Saída cheia | O produto não cabe no posto. Nada é colhido; o que ele já carrega espera. |
| Caminho obstruído | 10 s andando sem chegar. A planta é pulada até a próxima volta; com carga, ele continua tentando chegar ao posto. |
| Trabalhador ausente | Aldeão não encontrado (longe, em chunk descarregado, morto). Depois de 5 minutos com o posto carregado, a atribuição é desfeita. |

O painel mostra também as colheitas feitas e quantas colheitas a comida guardada ainda paga.

## Regras implementadas

- **Presença física.** Colher exige o aldeão a até 1,8 bloco (horizontal) e 2 blocos (vertical) do centro da planta. Entregar exige até 2,5 blocos do posto. Não há teletransporte de itens nem de aldeões; o produto fica "na mão" do trabalhador (`Carried` no NBT do posto) entre a planta e o posto.
- **Atribuição exclusiva.** Um aldeão pertence a no máximo um posto, e um posto tem no máximo um trabalhador. Bebês não são contratados.
- **Conservação.** O espaço no posto é conferido antes da colheita. A carga só entra no posto quando cabe inteira. Dispensar com carga a mantém no posto até haver espaço; nada é destruído nem duplicado. A comida é consumida item a item até cobrir 2 pontos; cada colheita desconta esses 2 pontos.
- **Saves.** O posto guarda o UUID do aldeão (`Worker`), a comida (`Ration`), o alvo (`Target`), a carga (`Carried`) e os contadores. Ao reabrir o mundo, o posto reencontra o aldeão pelo UUID entre os aldeões próximos, lendo no máximo 4 NBT a cada 10 ticks.
- **Comportamento vanilla.** O aldeão só obedece enquanto o posto renova a ordem (a cada 10 ticks; ela vence em 60). Sem ordem — sem tarefa, sem comida, posto descarregado ou removido, ou dispensado — o objetivo de trabalho fica inativo e o cérebro vanilla continua no comando, incluindo comércio e profissão. Nenhum dado do aldeão é alterado.
- **Desempenho.** O posto age a cada 10 ticks, com fase espalhada pela posição. Com alvo conhecido, lê um bloco; procurando, lê uma fileira de 9 posições por ciclo (volta completa em 27 ciclos). Aldeões vêm do registro já mantido pelo rancho (`entity_tick_pre`); não há varredura de entidades. Sem ordem viva, o objetivo `Work` não disputa o mapa de destinos.

## Código

- `native/api/src/workforce/mod.rs`: trabalhador, posto, atribuição, ordens de caminhada, objetivo `Work` registrado nos aldeões, ciclo genérico (carga → comida → tarefa) e painel.
- `native/api/src/workforce/farmhand.rs`: a tarefa agrícola (`next`, `perform`, `skip`). Um novo setor acrescenta um submódulo com essas três funções e um caso no `match` de `work`.
- `native/api/src/workforce/tests.rs`: testes de atribuição exclusiva, ciclo físico, comida, saída cheia, caminho obstruído, conservação na dispensa, restauração pelo UUID, ausência, reivindicação vencida, orçamento da varredura e custo de 2 pontos por colheita.

### Contratos tocados (para Codex e Grok)

- `lib.rs`: `is_factory_block`, `layout`, `output_slots`, `accepts` e `tick_factory` consultam `workforce::is_post`; `right_click_block` passa esmeralda e agachar+chave ao posto; `register_goals` registra `workforce::Work` nos aldeões.
- `production/ranch.rs`: nova função `recent_villagers(dimension)`, sobre o registro existente.
- `presentation.rs`: estados 18 a 23 (`no_worker`, `absent`, `no_task`, `no_food`, `path`, `walking`) em `STATUS_KEYS`; o posto não mostra energia nem barra de progresso. O slot 12 dos dados do painel (`Info`) carrega a comida guardada.
- `tutorial.rs`: rótulos dos slots do posto.
- Conteúdo: bloco `veio:farm_post` em `tools/build_content.py` e `tools/build_textures.py`; receita `farm_post.json`; recompensa no avanço `farming.json` e avanço `workforce.json`; textos, ajuda e uma página do guia em `tools/build_tutorial.py`.

IDs, saves e campos existentes não mudaram. O posto é um bloco novo, sem migração.

## Números para revisão (Grok)

Contratação: 1 esmeralda. Comida: 2 pontos por colheita, valores de `tools/production.json` (`food`). Área 9x9x3. Tempo de desistência: 200 ticks. Ausência máxima: 6.000 ticks. A colheitadeira gasta 120 FE por colheita e varre a área em 4 s; o trabalhador depende da caminhada. Calibração do Grok: [GROK-LOTE2.md](frentes/GROK-LOTE2.md).

## Verificações e limitações

- Feito: `python tools/check_project.py`, `python tools/check_rust.py --preview` (51 testes offline na entrega; 61 depois da integração da 0.10.0).
- **Não feito**: compilação pelo launcher e teste no jogo. As assinaturas usadas (`entity`, `entity_nbt`, `server_players`, `action_bar`, `take_items`, `give_items`, `register_goals`, `nav_move_to`) já existiam no substituto e no Veio; nenhuma assinatura nova foi acrescentada.
- A verificar no jogo: (1) se o Horizon chama `register_goals` para aldeões, que usam o cérebro vanilla e não o seletor de objetivos — se não chamar, o aldeão nunca chega e o posto mostra "Caminho obstruído"; (2) se o cérebro do aldeão disputa a navegação com o objetivo `Work` (o objetivo refaz o caminho a cada segundo); (3) a leitura do `UUID` e do `Age` pelo `entity_nbt`.
- Não distingue morte, remoção e chunk descarregado: todos aparecem como "ausente" até o limite.
- O aldeão não segura o produto visualmente; a carga existe apenas no NBT do posto.
- Ainda não há outros setores nem coordenador. As etapas 4 e 5 da ordem (interfaces para outros setores e coordenadores) ficam para as próximas entregas, partindo de `work` e `farmhand.rs`.

## Teste sugerido ao usuário

1. Num mundo de teste com uma vila, plante trigo em volta de um posto do lavrador e acelere com farinha de osso.
2. Coloque pão no posto, clique com uma esmeralda perto de um aldeão adulto e acompanhe o painel.
3. Encha os slots de saída, depois esvazie; dispense com carga; salve, saia e volte ao mundo; afaste o aldeão ou cerque a planta para ver "Caminho obstruído".
4. Confira que outros aldeões continuam comerciando normalmente.
