# Trabalhadores (frente Cloud)

Primeira etapa da ordem em [frentes/CLOUD.md](frentes/CLOUD.md): um aldeão contratado trabalha numa lavoura. Integrada na 0.10.0 com a interface de postos do Codex e o lote 1 do Grok.

## Para o jogador

1. Obtenha a colheitadeira. O avanço **Lavoura mecanizada** passa a liberar a receita do **Posto do lavrador** (composteira, enxada de ferro, esmeralda e barril, em qualquer posição).
2. Coloque o posto no meio de uma lavoura de até 9x9, como a colheitadeira. A camada do posto, a de baixo e a de cima são lidas.
3. Ponha comida no slot da esquerda. O valor é o do armazém, e cada tarefa gasta **2 pontos**: um trigo não paga a colheita; duas cenouras ou batatas pagam uma; pão (6) paga 3; carne assada (8) paga 4.
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

## 0.19.0: contas separadas, impostos e preços locais

- **Companhia x prefeitura:** o centro de controle ganhou um 5º espaço, o cofre da companhia. Ele paga os salários dos trabalhadores do jogador e os impostos. A prefeitura tem conta própria (verba inicial de 300 moedas, impostos, aportes) e paga o prefeito, a manutenção dos poços (1 moeda a cada 2 turnos por poço) e a assistência às famílias pobres.
- **Impostos:** 0%, 8%, 12% e 16% (por fase) da produção e das vendas do turno na área. Falta de saldo vira dívida; dívida acima de um dia de imposto suspende as licenças.
- **Preços locais:** fator por item em cada loja (50% a 200%), guiado pela cobertura do estoque frente à procura média do dia e pela falta que as famílias sentiram. Detalhes em [frentes/SOCIEDADE.md](frentes/SOCIEDADE.md).
- **Mais leve e organizado:** `society/` em quatro arquivos; o PIB virou soma por turno na conta da prefeitura (sem lista de eventos); a idade dos moradores só é relida quando a família muda (ou uma vez por dia); vizinhança e alvarás numa passada só; sem-teto com `HashSet`; Gini em O(n log n); todas as chaves NBT novas na tabela rápida (o teste de alocação zero continua passando). Morador agora é identificado por mundo e aldeão.

## 0.18.0: sociedade da vila

Casas e famílias, salário, consumo, lojas, prefeito, fases e indicadores. Regras e números no
[README](README.md#sociedade-da-vila) e o desenho em [frentes/SOCIEDADE.md](frentes/SOCIEDADE.md).

- **Código:** `native/api/src/workforce/society.rs`. Casa e poço são blocos sem trabalhador (como armazéns); loja e prefeitura são postos (comerciante e prefeito ficam no posto, como o administrador do centro de controle).
- **Leveza:** a casa trabalha uma vez por turno (1200 ticks), lê o censo de posições e no máximo 4 NBT (idade dos moradores). A prefeitura soma registros em memória uma vez por turno. A loja só reescreve as ofertas do comerciante quando o estoque muda. Nada roda por aldeão a cada tick.
- **Troca com o jogador:** as ofertas vão para o comerciante com `entity_merge_nbt` (`Offers`), e o evento `trade_with_villager` tira o item do estoque e põe as esmeraldas no caixa. Um aldeão que já tinha profissão perde as trocas vanilla enquanto for comerciante.
- **Limites indiretos:** `zones::right_click` consulta `society::zone_cap`; `village::observe` consulta `society::birth_allowed`.
- **Integração pendente:** a frente de obras da vila (outro agente: prestígio, prefeitura como obra, escola, mercado, refeitório) deve se juntar a esta prefeitura e somar serviços ao IDH.

## 0.17.0: o inimigo e o quartel

Partindo do que o Grok fez (ondas e quartel), o inimigo ficou mais justo e mais leve:

- **Ondas com frente:** chegam num arco de 90° a 26 blocos da máquina mais poluída, de um lado diferente a cada noite. Da 3ª noite de ataque vêm esqueletos arqueiros (um quarto da onda); da 5ª, pelo menos um **Bruto** (`veio:raider_brute`: 40 de vida, velocidade 0,2, dano 6, resistência a empurrão 0,6).
- **Combate vanilla:** saqueadores e brutos são zumbis com visão de 40 blocos; esqueletos, aranhas e parentes ganham o alvo vanilla "aldeão mais próximo" (`NearestTarget`). O dano é o do jogo, sem contadores falsos e sem varrer aldeões por mob em todo tick. A sabotagem só procura máquina a cada 20 chamadas.
- **Quartel:** os golpes do soldado tiram vida de verdade (`set_mob_health`; no último, `kill_mob`, com saque). Milícia 7 de dano, guarda 5 e se cura 3 a cada golpe, arqueiro 5 a 16 blocos, mago 10 e metade num segundo monstro. Trocar de classe: **agachar com uma esmeralda na mão** (a mão vazia nunca chegava ao quartel). O soldado fica de plantão à noite e, enquanto tem ordem, esquece o medo (as memórias de susto do cérebro são limpas antes dele pensar), então não foge do monstro.

## 0.16.1: o trabalhador segura o que carrega

- Enquanto leva algo (trigo, lã, minério, insumos, a esmeralda do capataz), o aldeão segura na mão o primeiro item da carga; ao entregar, à noite e ao ser dispensado, a mão fica vazia.
- É só visual: a carga continua no NBT do posto (`Carried`), então salvar, descarregar o chunk ou trocar de trabalhador não perde nem duplica nada. A chance de o item da mão cair na morte é zerada (`HandDropChances`), para a morte do aldeão não duplicar o que já está no posto.
- O posto só chama o jogo quando o item muda (`Held` no NBT do posto). Um aldeão vindo de outro posto ou de um save antigo tem a mão acertada na primeira vez.
- API: `Host::living` e `Host::living_set_equipment(living, EquipmentSlot::MainHand, stack)`, com `LivingRef` e `EquipmentSlot` (`horizon_world::player_inventory`), conferidos no Horizon `66f3981` (`game/native_living_refs.rs`) e acrescentados ao substituto offline.
- Teste: `worker_holds_what_it_carries_and_empties_the_hand_after_delivery`.

## Sexta entrega: aldeões guiados pelo cérebro, noite, spawn conferido e vilas grandes

Feita sem teste no jogo, a partir do código do Horizon `66f3981` (`mobs/ai.rs`, `mobs/villager.rs`, `mobs/villager_brain.rs`, `game/integrated.rs`, `game/commands*`).

### Aldeões andando até a tarefa

- **O que o Horizon faz:** os objetivos registrados por mods (`register_goals`) rodam também nos aldeões, antes do cérebro, então o `Work` roda. O cérebro do aldeão roda depois e, sempre que tem um alvo de caminhada, refaz o caminho (`MoveToTargetSink`). Se o `Work` só usasse a navegação, os dois disputariam o caminho e o aldeão ficaria indo e voltando.
- **Agora:** o `Work` grava a ordem como alvo de caminhada no cérebro (`villager::ext_mut(m).brain.mem.walk_target`) e o próprio cérebro leva o aldeão. Passear, ir ao sino e procurar emprego só escolhem alvo quando não há um, então não atrapalham. Quando a ordem acaba, o objetivo apaga só o alvo que ele mesmo deu.
- **Segurança:** em pânico (zumbi), ataque, preparo de ataque, esconderijo ou sono, o objetivo não mexe no cérebro: o aldeão se protege e o posto espera.
- **Noite:** a partir de 12000 (a hora em que o aldeão vanilla vai dormir), o posto mostra **Descansando (noite)**, retira a ordem e guarda carga e alvo; de manhã o trabalho continua. O centro de controle mantém o painel e o relatório durante a noite.

### Spawn na vila, conferido no Horizon

- `run_command` só roda onde há servidor (`lan`). No singleplayer, o servidor integrado roda numa thread própria com `lan` ativo, e os ganchos de login e renascimento disparam nele: o spawn funciona.
- Os comandos usados existem: `execute ... as ... unless entity @s[tag=...] run`, `tp`, `setworldspawn`, `spawnpoint`, `tag`, `tellraw` e `place structure` (este ignora o bioma, como o vanilla, e exige os chunks carregados, o que vale em volta do jogador).
- **Corrigido:** o tick também roda na janela, onde a lista de jogadores do servidor pode vir vazia; antes a chegada era descartada. Agora ela espera o servidor.

### Desempenho com vilas grandes (medido)

`bench_large_village` (teste ignorado; `cargo test --release -p veio-check bench_large_village -- --ignored --nocapture`): 150 aldeões, 300 animais, 20.000 ticks, ordens vivas para 1 a cada 7 aldeões. Mede só o código do Veio no substituto.

| | Antes | Depois |
| --- | --- | --- |
| µs por tick do jogo (mediana de 5) | ~14,7 | ~3,5 |

- **Registros de mobs por amostragem:** cada mob entra nos registros de defesa e rancho em 1 de cada 7 ticks do relógio do mod (as consultas aceitam marcas de até 40-60 ticks). Antes eram duas travas e várias comparações de texto por mob, por tick.
- **`Work` consulta as ordens a cada 10 chamadas**, não em todo tick.
- **Correção:** o relógio do mod pode avançar de 2 em 2 (janela e servidor). Com o intervalo par de 6000, metade dos aldeões nunca caía na fase de natalidade; agora são 5999 ticks. A amostragem usa 7 pelo mesmo motivo.

### Contratos tocados

- `lib.rs`: `entity_tick_pre` amostra os registros (`OBSERVE_EVERY = 7`).
- Substituto: `mobs::villager::{Ext, ext, ext_mut}`, `mobs::villager_brain::{Brain, Memories, Slot, WalkTarget, Tracker, Act}` (só os campos usados) e `Mob::extra`, conferidos no Horizon `66f3981`; construtores de medição `EntityTickPreEvent::preview`, `TickEvent::preview` e `MobWorld::preview` (só no substituto).
- `presentation.rs`: estado 28 (`rest`); `operations.rs`: sem bloqueio.

## Quinta entrega: vilas até 150, spawn na vila e Guia da Vila Operária

### População que cresce com a vila

O teto de aldeões de cada vila passou a depender do desenvolvimento a até 64 blocos: **30** de base, mais **3** por posto com trabalhador, **5** por nível de zona, **2** por armazém e **20** por um centro de controle com administrador, até **150**. A natalidade é a mesma (1 chance em 6 a cada 5 min por adulto), contando os aldeões a até 48 blocos. Uma fábrica espalhada em vários lugares tem uma vila em cada lugar, cada uma com o seu teto.

**Desempenho com vilas grandes:** as contagens de aldeões (natalidade, níveis de zona, contratação, procura do trabalhador, armazém da vila) usam um **censo** das posições (`ranch::villager_positions`), refeito a cada 100 ticks ou quando o conjunto de aldeões muda. Consultas no mesmo intervalo compartilham a lista, sem copiar as 150 posições. A lista de aldeões não passa pelos animais. O teto lê os registros de postos, zonas e armazéns, sem varrer blocos.

### Spawn no centro da vila

- **Mundo novo e jogador novo:** no login, o Veio procura uma vila a até 96 blocos pelos aldeões registrados. O centro é a média dos aldeões em volta daquele com mais vizinhos (onde a vila se reúne). O jogador é levado para lá, e o spawn do mundo e o dele ficam nesse ponto. Se não houver vila em 30 s, o Veio gera uma vila de planície onde o jogador está (`/place structure minecraft:village_plains`) e procura de novo; se ainda assim nada aparecer, avisa no chat.
- **Centro de controle:** com administrador, o centro de controle passa a ser o centro da vila. O spawn do mundo e o de todos os jogadores vão para cima dele, e quem renasce é levado para lá, mesmo tendo cama em outro lugar.
- Cada jogador recebe a tag `veio_village` depois da primeira chegada; os comandos de chegada só valem para quem não tem a tag, então voltar ao mundo não muda nada.
- Usa os ganchos `player_logged_in` e `player_respawned` e `Host::run_command` (console do servidor), conferidos no Horizon `66f3981`. Só no Overworld.

### Guia da Vila Operária

O guia principal estava com 99 de 100 páginas. Os sistemas da vila foram para um **segundo livro**, o Guia da Vila Operária (23 páginas): roteiro em 7 passos, expansão das zonas, crescimento da vila, problemas comuns, comida e a consulta de todos os blocos da vila. O guia principal ficou com 86 páginas e uma página que aponta para o novo livro.

- Entregue por uma aba de avanços nova, **Vila operária**, que funciona como roteiro: contratar o primeiro trabalhador, demarcar a lavoura, cercado, celeiro, carregador, capataz e centro de controle.
- Livro + trigo fabricam outro guia da vila.
- `tools/check_project.py` confere os dois livros (páginas, traduções, links e livro fabricado igual ao entregue).

## Quarta entrega: zonas, armazenagem, centro de controle e vilas maiores

### Zonas de lavoura e de cercado

- **Marco de lavoura** e **marco de cercado** (estaca com placa verde ou marrom) ficam no centro da área. A borda aparece desenhada no chão, com estacas nos cantos.
- Tamanhos por nível: 9x9, 13x13, 17x17 e 21x21 (o último é cerca de cinco vezes uma horta de vila). Sempre três camadas: a do marco, a de baixo e a de cima.
- **Expansão aos poucos:** com uma esmeralda na mão, o clique no marco libera o próximo nível se a zona tiver trabalho feito e a vila tiver aldeões suficientes a até 48 blocos:

| Nível | Esmeraldas | Trabalho feito na zona | Aldeões |
| --- | --- | --- | --- |
| 2 (13x13) | 8 | 32 | 6 |
| 3 (17x17) | 16 | 128 | 12 |
| 4 (21x21) | 32 | 384 | 20 |

- **O jogador escolhe o tamanho:** a chave (sem agachar) alterna entre os níveis já liberados.
- **Grupos:** todo posto do tipo certo a até 3 blocos da borda trabalha na zona inteira. Um aldeão pode cuidar de uma lavoura, ou vários postos podem dividir a mesma lavoura (as reservas evitam disputa).
- **Lavoura:** além de colher, o lavrador **ara** grama e terra com ar em cima e água a até 4 blocos, e **planta** em terra arada vazia as sementes que colheu (cenoura e batata servem de semente; fungo do Nether vai em areia das almas). A semente no slot do marco escolhe a cultura daquela lavoura: assim há campos variados. Plantar e arar não gastam comida.
- **Cercado:** o pastor trabalha no cercado inteiro. A ração no slot do marco define o cercado: trigo para vacas, ovelhas e cabras; cenoura para porcos e coelhos; sementes para o galinheiro. Galinheiro coberto, área aberta e cercas são construídos pelo jogador; o marco define a área de trabalho.
- O painel do marco mostra nível, área, trabalho feito e o que falta para o próximo nível.

### Celeiro e silo

- **Celeiro:** 27 espaços, qualquer item. **Silo:** 18 espaços, só grãos, sementes, ração e comida.
- Funis põem e tiram itens como num baú; o bocal não esvazia o armazém numa esteira.
- Na fila do capataz, os armazéns são o último destino (recebem o que nenhuma máquina, a mesa ou o pastor quis) e a última origem (abastecem máquinas, a mesa e a ração do pastor quando os produtos novos não bastam). Estoque não vai de um armazém para outro.

### Centro de controle

- Contrate um **administrador** com uma esmeralda; ele fica ao lado do centro. A comida dele é gasta a cada 2 minutos de painel atualizado.
- O painel mostra, para os blocos a até 64 blocos: trabalhadores contratados e quantos estão ativos, quantos esperam (saída cheia, sem trabalhador, ausente, sem comida, caminho obstruído, sem esmeralda) e quantos estão sem comida, lavouras e cercados, ocupação dos armazéns, tarefas na fila dos capatazes e até **3 metas**.
- **Metas:** um item num slot de meta pede `quantidade x 16` daquele item guardados nos armazéns da área; a barra mostra o progresso.
- **Relatório:** clicar no centro com um livro manda ao chat os postos que precisam de atenção (posição e motivo), cada zona com o que falta para o próximo nível, os armazéns e as metas.
- Os números vêm dos registros em memória que postos, zonas e armazéns atualizam no próprio tick; o centro não varre blocos.

### Vilas maiores

- A cada 5 minutos (fase espalhada pelo id), cada aldeão adulto tem 1 chance em 6 de ter um filho, se houver pelo menos 2 aldeões perto e menos que o teto da vila. O vanilla limita a vila pelas camas; o teto começa em 30 (o triplo de uma vila grande) e, a partir da quinta entrega, cresce com o desenvolvimento até 150. O bebê nasce no tick seguinte de um bloco do Veio na mesma dimensão.

## Terceira entrega: fila de tarefas e pastor

### Fila de tarefas do capataz

Com o capataz contratado e presente, a mesa mantém uma **fila de transferências** (a primeira volta da varredura da rede leva cerca de 7,5 s, conforme o [lote 3 do Grok](frentes/GROK-LOTE3.md)) entre os blocos da sua rede: até 6 postos e as máquinas (estações de trabalho da interface de postos) a até 12 blocos, no máximo 24 blocos. A cada 40 ticks ela refaz a fila, com até 8 tarefas:

- **Origem:** saídas das máquinas e dos postos (colheita do lavrador, produtos do carregador e do pastor, esmeraldas do armazém).
- **Destino:** entradas das máquinas que aceitam o item (`operations::accepts_material`), até 16 itens por entrada; a própria mesa (comida quando guarda menos de 16, e esmeraldas); a ração do posto do pastor (até 32). A comida dos trabalhadores continua sendo levada pelo capataz, e o estoque do carregador continua sendo do jogador.
- Um destino com entrega a caminho (reservado) não recebe outra tarefa.

Os **carregadores** a até 16 blocos da mesa pegam tarefas da fila antes das suas, se origem e destino estiverem a até 12 blocos do posto deles. A tarefa fica marcada com a posição do carregador; se a marca não for renovada em 200 ticks (carregador sumiu, posto descarregado), ela volta a ficar livre. O carregador anda até a origem, pega os itens (só o item combinado, só o que ainda estiver lá), tira a tarefa da fila, leva ao destino e insere ao lado dele. A sobra volta ao estoque do posto do carregador.

Exemplos: trigo do lavrador → moinho; farinha → cozinha; pão → armazém ou mesa; esmeraldas do armazém → mesa, que paga as recontratações; sementes do lavrador → ração do pastor.

O painel da mesa mostra quantos postos atende e o tamanho da fila.

### Posto do pastor (setor rancho)

Receita: tesoura, laço, fardo de feno e esmeralda. Liberada pelo avanço **Mão de obra**, junto do carregador.

- Contrate com uma esmeralda. Comida no slot 0; ração (trigo, cenoura, batata, beterraba, sementes, dente-de-leão) no slot 1.
- A até 6 blocos, em ordem: recolhe do chão os produtos de animais (os mesmos do rancho: ovos, penas, couro, lã, carnes cruas); tosquia ovelhas adultas; dá ração a um animal adulto por vez, só quando há dois prontos da mesma espécie (ou o par de um que já comeu) e a espécie tem menos de 10 animais.
- Tudo exige o aldeão ao lado do animal ou do item; o produto vai na mão até o posto.
- Não ordenha nem abate: isso continua com o rancho.

## Segunda entrega: indústria e coordenação

### Posto do carregador (setor indústria)

Receita: baú, funil, esmeralda e chapa de ferro. Liberada pelo avanço **Mão de obra** (obter o posto do lavrador).

- Contrate com uma esmeralda, como no posto do lavrador. Comida no slot da esquerda; cada viagem com entrega gasta 2 pontos (pão paga 3 viagens).
- Os três slots de **insumos** são o estoque: o carregador leva até 16 itens por viagem a uma máquina que os aceite, mantendo cada entrada com até 16.
- Os três slots de **saída** recebem o que ele recolhe das máquinas (até 32 itens por viagem).
- Atende até 12 máquinas a até 6 blocos (13x13, três camadas). Prioridade: máquina parada por falta de insumo ou combustível; depois produtos prontos; depois completar entradas.
- Só atende estações de trabalho da [interface de postos](frentes/INTERFACE_POSTOS.md) (`operations::profile` = `Workstation`): máquinas, geradores, central de projetos, armazém. Nunca esteiras, divisores, unificadores, postes, cabos, acumuladores nem outros postos de trabalho. Os filtros de entrada são os de `operations::accepts_material`, incluindo o pedido atual da central.
- Pega o insumo no posto (precisa estar ao lado dele), anda até a máquina e só insere ao lado dela. Se a máquina não aceitar mais (o jogador trocou o material, a entrada encheu), a sobra volta ao estoque do posto.

### Mesa do capataz (coordenação)

Receita: atril, sino e duas esmeraldas. Liberada pelo avanço **Carregadores** (obter o posto do carregador).

- Contrate o capataz com uma esmeralda na mesa. Os três primeiros slots guardam comida (dele e para os postos); o último guarda esmeraldas.
- Cuida de até 6 postos (lavrador ou carregador) a até 12 blocos:
  - quando o trabalhador de um posto tem menos de 4 pontos de comida e o posto guarda menos de 4 itens de comida, o capataz leva até 8 itens da mesa (sempre guardando um para si);
  - quando um posto fica sem trabalhador (morto, removido, dispensado ou desfeito por ausência), ele leva uma esmeralda até lá e contrata o aldeão adulto livre mais próximo do posto; sem candidato, a esmeralda volta à mesa.
- O capataz não faz o trabalho dos postos nem escolhe as tarefas deles.

### Coordenação sem disputa

Cada posto reserva o alvo atual (planta, máquina ou posto atendido) num registro em memória (`RESERVED`), renovado a cada ciclo e vencido em 100 ticks. Outro posto não escolhe um alvo reservado. Assim dois carregadores não atendem a mesma máquina, dois lavradores não vão à mesma planta e dois capatazes não atendem o mesmo posto ao mesmo tempo.

### Novos estados

| Estado | Significado |
| --- | --- |
| Nada a fazer | Carregador ou capataz sem tarefa nesta volta. O aldeão fica livre (vanilla). |
| Sem esmeralda para contratar | O capataz achou um posto vazio, mas a mesa não tem esmeralda. |

## Regras implementadas

- **Presença física.** Colher exige o aldeão a até 1,8 bloco (horizontal) e 2 blocos (vertical) do centro da planta. Entregar exige até 2,5 blocos do posto. Não há teletransporte de itens nem de aldeões; o produto fica "na mão" do trabalhador (`Carried` no NBT do posto) entre a planta e o posto.
- **Atribuição exclusiva.** Um aldeão pertence a no máximo um posto, e um posto tem no máximo um trabalhador. Bebês não são contratados.
- **Conservação.** O espaço no posto é conferido antes da colheita. A carga só entra no posto quando cabe inteira. Dispensar com carga a mantém no posto até haver espaço; nada é destruído nem duplicado. A comida é consumida item a item até cobrir 2 pontos; cada tarefa concluída desconta esses 2 pontos.
- **Saves.** O posto guarda o UUID do aldeão (`Worker`), a comida (`Ration`), o alvo (`Target`), a carga (`Carried`) e os contadores. Ao reabrir o mundo, o posto reencontra o aldeão pelo UUID entre os aldeões próximos, lendo no máximo 4 NBT a cada 10 ticks.
- **Comportamento vanilla.** O aldeão só obedece enquanto o posto renova a ordem (a cada 10 ticks; ela vence em 60). Sem ordem — sem tarefa, sem comida, posto descarregado ou removido, ou dispensado — o objetivo de trabalho fica inativo e o cérebro vanilla continua no comando, incluindo comércio e profissão. Nenhum dado do aldeão é alterado.
- **Desempenho.** O posto age a cada 10 ticks, com fase espalhada pela posição. Lavrador: 9 blocos/ciclo, com ou sem zona, e no máximo um teste de água por ciclo; carregador: 16 blocos e 3 inspeções de máquina; capataz: 25 blocos de varredura e 3 inspeções de posto; pastor: 4 NBT de animais, e o alvo atual não relê o rebanho. A fila relê cada bloco da rede uma vez a cada 40 ticks. Aldeões e animais vêm do registro do rancho (`entity_tick_pre`). Sem ordem, reserva, mesa, zona ou armazém, os objetivos não disputam os mapas.

## Código

- `native/api/src/workforce/mod.rs`: trabalhador, posto, atribuição, reservas, ordens de caminhada, objetivo `Work` registrado nos aldeões, varredura incremental de blocos, ciclo genérico (carga → comida → tarefa) e painel.
- Interface de tarefas: um adaptador devolve um `Plan`. `Visit { spot, reach }` manda o trabalhador ao alvo e chama o `perform` do adaptador ao chegar; o que ele devolver é carregado até o posto. `Bring { from, count, to, into, kind }` faz o trabalhador pegar itens no posto e levá-los ao slot `into` de outro bloco (`Carry::Supply`) ou contratar lá (`Carry::Wage`). Um setor novo é um submódulo com `plan` (e `perform`, se usar `Visit`) e um caso nos dois `match` de `work`.
- `farmhand.rs` (lavoura), `hauler.rs` (indústria), `shepherd.rs` (rancho) e `foreman.rs` (coordenação e fila).
- `Plan::Fetch` (terceira entrega): tarefa da fila; buscar itens num bloco e levar a outro.
- `native/api/src/workforce/tests.rs`: 17 testes. Além dos 11 da primeira entrega: abastecer e recolher uma prensa, ignorar esteiras e outros postos, reserva entre dois carregadores, capataz alimentando e recontratando, capataz sem esmeralda e esmeralda devolvida, e insumo devolvido ao estoque quando a máquina deixa de aceitar.

### Contratos tocados (para Codex e Grok)

Quinta entrega:

- `lib.rs`: ganchos `player_logged_in` e `player_respawned`; o `tick` chama `workforce::spawn::tick`.
- Substituto offline: `Host::run_command(&mut self, line: &str) -> bool` e os eventos `PlayerLoggedInEvent`/`PlayerRespawnedEvent` (`player: u128`, `name: &str`), copiados do Horizon `66f3981` (`native_server.rs`, `native_hooks.rs` e o gerador `horizon-mod-api-gen`).
- `production/ranch.rs`: `villager_positions` (censo) e `villagers_near` passa a usá-lo.
- Registros (`ZONES`, `STORES`, `POSTS`): leituras sem efeito colateral; a limpeza de vencidos ficou só nas escritas. O atalho de registro vazio do lote 6 do Grok continua.
- `tools/build_tutorial.py` gera o Guia da Vila Operária e a aba `veio:village/*`; `reference_labels` gera uso e receita para os dois livros.

Quarta entrega:

- `lib.rs`: `workforce::owns` (postos, marcos e armazéns) substitui `is_post` no registro, layout, saídas, entradas e tick; `entity_tick_pre` chama `workforce::village::observe` para aldeões; a área de desenho dos marcos cobre a zona; armazéns contam como entrada para as curvas das esteiras.
- `presentation.rs`: estados 26 (`zone`) e 27 (`storing`); centro de controle e marcos publicam os próprios 13 valores do painel; o centro tem tela própria (como a central de projetos); marcos desenham a borda no lugar das luzes.
- `operations.rs`: `StationKind::Storage` (celeiro e silo, setor logística); centro de controle no setor de coordenação.
- Registros novos em memória: `ZONES`, `STORES`, `POSTS` e a fila de nascimentos.
- O livro do guia está com 99 páginas; o limite é 100.

Terceira entrega:

- `production/ranch.rs`: `VILLAGER`, `WOOL`, `collectable` e `tag_int` passam a `pub(crate)` para o pastor.
- `operations.rs`: posto do pastor no setor agricultura.
- A mesa publica o tamanho da fila no slot 7 (`Utilization`), que só tinha uso em máquinas elétricas.
- A fila move itens entre máquinas com `change_mod_block_entity` (retirada na origem e entrega no destino, com o carregador ao lado de cada um). Ela não usa esteiras nem a inserção por face.

Segunda entrega:

- `operations.rs`: `Sector::Workforce` (mesa do capataz); posto do carregador no setor indústria; estados 24 (`idle_work`, sem bloqueio) e 25 (`no_wage`, entrada) em `blockage`; dicas em `tools/build_operations.py`.
- `presentation.rs`: `STATUS_KEYS` com 26 estados. O capataz e o carregador publicam no slot 6 (`Rate`) quantos postos ou máquinas atendem; esse slot só tinha uso em máquinas elétricas.
- `tutorial.rs`: um rótulo seguido de papéis vazios (`""`) passa a ser centralizado no grupo de três slots, para qualquer bloco. Os grupos existentes (colheitadeira, silvicultora, rancho) ficam iguais.
- O carregador só move itens com `change_mod_block_entity` estando ao lado da máquina, revalidando `accepts_material` e o espaço no momento da entrega, como pede a interface de postos. Ele insere direto no slot, sem passar pela face.

Primeira entrega:

- `lib.rs`: `is_factory_block`, `layout`, `output_slots`, `accepts` e `tick_factory` consultam `workforce::is_post`; `right_click_block` passa esmeralda e agachar+chave ao posto; `register_goals` registra `workforce::Work` nos aldeões.
- `production/ranch.rs`: nova função `recent_villagers(dimension)`, sobre o registro existente.
- `presentation.rs`: estados 18 a 23 (`no_worker`, `absent`, `no_task`, `no_food`, `path`, `walking`) em `STATUS_KEYS`; o posto não mostra energia nem barra de progresso. O slot 12 dos dados do painel (`Info`) carrega a comida guardada.
- `tutorial.rs`: rótulos dos slots do posto.
- Conteúdo: bloco `veio:farm_post` em `tools/build_content.py` e `tools/build_textures.py`; receita `farm_post.json`; recompensa no avanço `farming.json` e avanço `workforce.json`; textos, ajuda e uma página do guia em `tools/build_tutorial.py`.

IDs, saves e campos existentes não mudaram. O posto é um bloco novo, sem migração.

## Números para revisão (Grok)

Vila: teto 30 + 3/posto com trabalhador + 5/nível de zona + 2/armazém + 20/centro de controle, máximo 150, desenvolvimento contado a 64 blocos e aldeões a 48. Spawn: procura a 96 blocos, mínimo 3 aldeões, espera 30 s antes de gerar uma vila.

Zonas: lados 9/13/17/21; níveis 2-4 custam 8/16/32 esmeraldas, 32/128/384 tarefas na zona e 6/12/20 aldeões a 48 blocos; postos a até 3 blocos da borda usam a zona. Centro: raio 64, metas x16, comida do administrador a cada 120 ciclos. Natalidade: 1/6 a cada 6.000 ticks por adulto, mínimo 2 e teto 30 aldeões a 32 blocos.

Fila: rede de até 24 blocos a 12 da mesa, 8 tarefas, refeita a cada 40 ticks, marca vence em 200 ticks; carregadores a 16 da mesa, origem e destino a 12 do carregador; mesa pede comida abaixo de 16. Pastor: raio 6, lê até 12 animais por ciclo, lã 1 a 3 por ovelha. Receita do pastor: tesoura, laço, fardo de feno, esmeralda.

Carregador: 16 itens por viagem de abastecimento, 32 por recolhida, entradas mantidas com até 16, raio 6, até 12 máquinas. Capataz: raio 12, até 6 postos, reabastece abaixo de 4 e leva até 8 itens de comida, 1 esmeralda por recontratação. Receitas: carregador (baú, funil, esmeralda, chapa de ferro); capataz (atril, sino, 2 esmeraldas).

Lavrador: contratação: 1 esmeralda. Comida: 2 pontos por tarefa, valores de `tools/production.json` (`food`). Área 9x9x3. Tempo de desistência: 200 ticks. Ausência máxima: 6.000 ticks. A colheitadeira gasta 120 FE por colheita e varre a área em 4 s; o trabalhador depende da caminhada. Calibração do Grok: [GROK-LOTE3.md](frentes/GROK-LOTE3.md).

## Verificações e limitações

- Feito: `python tools/check_project.py`, `python tools/check_rust.py --preview` (51 testes na primeira entrega, 61 na 0.10.0, 67 na 0.11.0, 78 na 0.12.0, 90 na 0.13.0, 93 na 0.14.0, 134 na 0.16.0, 135 na 0.16.1, 138 na 0.17.0, 149 na 0.18.0 e 160 na 0.19.0 (com a logística do Codex e os lotes 3 a 7 do Grok); prévia dos painéis novos).
- **Não feito**: compilação pelo launcher e teste no jogo. As assinaturas usadas (`entity`, `entity_nbt`, `server_players`, `action_bar`, `take_items`, `give_items`, `register_goals`, `nav_move_to`) já existiam no substituto e no Veio; nenhuma assinatura nova foi acrescentada.
- A verificar no jogo: (1) se o Horizon chama `register_goals` para aldeões, que usam o cérebro vanilla e não o seletor de objetivos — se não chamar, o aldeão nunca chega e o posto mostra "Caminho obstruído"; (2) se o cérebro do aldeão disputa a navegação com o objetivo `Work` (o objetivo refaz o caminho a cada segundo); (3) a leitura do `UUID` e do `Age` pelo `entity_nbt`.
- Não distingue morte, remoção e chunk descarregado: todos aparecem como "ausente" até o limite.
- O aldeão não segura o produto visualmente; a carga existe apenas no NBT do posto.
- O carregador não respeita o tamanho máximo de pilha de cada item (usa 64, como o resto do Veio); itens de pilha 16 ou 1 podem ficar acima do vanilla nas máquinas.
- O capataz não tem hierarquia acima dele. A fila só é executada por carregadores; lavradores e pastores seguem com as próprias tarefas.
- Dois pastores podem perseguir o mesmo animal (a reserva é por bloco, e o animal anda); o segundo só encontra o animal já atendido e procura outro.
- A rede da mesa guarda os primeiros 24 blocos encontrados; numa base muito densa, postos podem ficar de fora.
- O carregador, depois da primeira volta, ainda inspeciona até 12 máquinas a cada 10 ticks quando escolhe a próxima viagem; um teto de inspeções por ciclo fica para um lote seguinte.

## Teste sugerido ao usuário (0.18.0, sociedade)

1. Faça um posto do lavrador (libera as receitas) e depois casa, poço, loja e prefeitura.
2. Contrate o prefeito e o comerciante com esmeraldas; ponha esmeraldas no cofre da companhia (5º espaço do centro de controle) e pão na loja.
3. Ponha uma casa perto de 2 a 4 aldeões. Em até 1 minuto o painel mostra os moradores, a carteira e a despensa; de dia a família compra pão, e o caixa da loja enche.
4. Clique no comerciante: a tela de troca mostra o pão à venda. Compre e veja o estoque baixar.
5. Livro na prefeitura: relatório com IDH, Gini, PIB, bairros e o que falta para a próxima fase.

## Teste sugerido ao usuário (0.17.0, inimigo)

1. Polua bastante (geradores) e espere o aviso no chat; na noite seguinte, veja a onda chegar por um lado.
2. Monte um quartel, contrate com esmeralda e ponha pão. Agache com uma esmeralda para trocar a classe.
3. À noite, confira que o soldado não vai dormir nem foge, e que o monstro morre depois de alguns golpes (e solta saque).
4. Sobreviva até a 3ª e a 5ª noites de ataque: esqueletos e o Bruto aparecem.

## Teste sugerido ao usuário (sexta entrega)

1. Contrate um lavrador e acompanhe: ele deve ir direto às plantas maduras, sem ficar indo e voltando para o sino ou passeando.
2. Solte um zumbi perto: o trabalhador deve fugir, e voltar ao trabalho depois.
3. Espere a noite: o painel mostra "Descansando (noite)" e o aldeão vai para a cama; de manhã ele volta.
4. Mundo novo no singleplayer: confira se você aparece no centro da vila.
5. Vila com 100+ aldeões: compare o desempenho com a 0.15.0.

## Teste sugerido ao usuário (quinta entrega)

1. Mundo novo: você deve aparecer no meio de uma vila (ou de uma vila de planície gerada) e receber o Guia da Vila Operária. Morra longe e veja onde renasce.
2. Construa um centro de controle com administrador: o spawn passa para cima dele; renascer leva você para lá.
3. Com vários postos, zonas e um centro, deixe o jogo correr perto da vila: a população deve passar de 30. Compare o desempenho com a vila pequena.
4. Siga a aba de avanços Vila operária e o guia novo: diga o que ficou confuso.

## Teste sugerido ao usuário (quarta entrega)

1. Coloque um marco de lavoura com um posto do lavrador perto da borda, água e grama dentro da área, e trigo no posto. O lavrador deve arar perto da água, plantar e colher. Ponha uma cenoura no marco: só cenoura deve ser plantada.
2. Com trabalho feito e 6 aldeões perto, clique o marco com 8 esmeraldas: a borda cresce para 13x13. A chave alterna entre 9x9 e 13x13.
3. Marco de cercado com cenoura no slot, um pastor, porcos e vacas: só os porcos recebem ração.
4. Celeiro perto da mesa do capataz: sobras devem ir para ele.
5. Centro de controle com administrador: confira os números e use um livro no centro para o relatório. Ponha 4 pães numa meta.
6. Numa vila, espere alguns minutos de jogo perto de blocos do Veio: devem nascer bebês até cerca de 30 aldeões.

## Teste sugerido ao usuário (terceira entrega)

1. Mesa do capataz com capataz contratado, um posto do lavrador com trigo nas saídas, um moinho e um posto do carregador perto. O carregador deve levar o trigo ao moinho sem nada no estoque dele.
2. Um armazém vendendo comida perto da mesa: as esmeraldas devem ir para a mesa.
3. Posto do pastor com trigo perto de duas vacas e de ovelhas: ele deve tosquiar, recolher a lã e alimentar o par uma vez.

## Teste sugerido ao usuário (segunda entrega)

1. Perto de uma prensa com energia, coloque um posto do carregador, contrate um aldeão e ponha cobre bruto nos insumos. Ele deve levar o minério e trazer as chapas.
2. Ponha uma esteira com itens perto: ele não deve mexer nela.
3. Coloque uma mesa do capataz perto de dois postos, contrate o capataz e ponha pão e esmeraldas na mesa. Deixe um posto sem comida e dispense o trabalhador de outro: o capataz deve levar pão e recontratar.

## Teste sugerido ao usuário (primeira entrega)

1. Num mundo de teste com uma vila, plante trigo em volta de um posto do lavrador e acelere com farinha de osso.
2. Coloque pão no posto, clique com uma esmeralda perto de um aldeão adulto e acompanhe o painel.
3. Encha os slots de saída, depois esvazie; dispense com carga; salve, saia e volte ao mundo; afaste o aldeão ou cerque a planta para ver "Caminho obstruído".
4. Confira que outros aldeões continuam comerciando normalmente.
