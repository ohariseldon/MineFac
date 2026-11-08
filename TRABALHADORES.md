# Trabalhadores (frente Cloud)

Primeira etapa da ordem em [frentes/CLOUD.md](frentes/CLOUD.md): um aldeão contratado trabalha numa lavoura. Integrada na 0.10.0 com a interface de postos do Codex e o lote 1 do Grok.

## Para o jogador

1. Obtenha a colheitadeira. O avanço **Lavoura mecanizada** passa a liberar a receita do **Posto do lavrador** (composteira, enxada de ferro, esmeralda e barril, em qualquer posição).
2. Coloque o posto no meio de uma lavoura de até 9x9, como a colheitadeira. A camada do posto, a de baixo e a de cima são lidas.
3. Ponha comida no slot da esquerda. O valor é o do armazém, e cada tarefa gasta **2 pontos**: um trigo não paga a colheita; duas cenouras ou batatas pagam uma; pão (6) paga 3; carne assada (8) paga 4.
4. Com uma **esmeralda** na mão, clique com o botão direito no posto. Um aldeão adulto livre a até 8 blocos é contratado e a esmeralda é gasta: primeiro os sem profissão e os nitwits, depois comerciantes de nível 1, e os experientes só se não houver outro; no empate, o mais próximo. Sem candidato, a esmeralda fica com o jogador.
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

## 0.25.0: balanceamento da economia

Números, princípios e resultados em [frentes/ECONOMIA.md](frentes/ECONOMIA.md).

- **Preço-âncora único:** o preço base local de um produto da bolsa é o valor de referência mundial (base x complexidade); comida fora da bolsa vale pontos x 1,5 (preparada) ou x 1 (crua). Antes, pão valia 6 na loja e 9 lá fora: a indústria exportava tudo e a vila passava fome.
- **Salário = 2 x custo de vida / 20** (4 por turno), custo de vida 40/dia.
- **Teto do preço local** = custo de importar + 10% de margem, contando o imposto da fase (`market::import_ceiling`); atacado a 75%; lote ao jogador = o maior que cabe em 1 esmeralda (6 pães).
- **Dinheiro circula:** lucro do comerciante acima da reserva de giro (600) vai para a família dele; poupança acima de 2 dias de custo de vida vira compra de bens.
- **Comércio exterior:** absorção maior (256/512/512/256/128) com pressão = metade da média recente + hoje, piso de sucata de 25%, frete por pilha (porto 6, estação 12), contrato pago a preço cheio + 50% sem saturar o mercado à vista, camada aberta que nunca fecha (formato de salvamento v3). Preços em centésimos.
- **Defeitos corrigidos:** a camada conquistada podia fechar com a queda de reputação.
- **Simulador** `tools/sim_economy.py` com 4 cenários e regras de equilíbrio, que também rodam em `check_project.py`; 4 testes `balance_*` com o código do jogo.

## 0.24.0: acabamento do comércio exterior

- Avisos no chat (contrato novo, cumprido com prêmio, perdido, mercado aberto) e partidas com buzina ou trilho, fumaça e aviso na barra para quem está perto.
- Estado *Mercado fechado* para carga de camada ainda não liberada; a aba mostra a reputação que abre a próxima camada.
- Contratos sem produto repetido; prêmio em centésimos (antes sumia em itens de 1 moeda: defeito corrigido); bolsa (saturação) e reputação salvas no centro de controle, com versão do formato.
- Estação também longe do centro da vila; importação da comida mais vendida; as 5 melhores cotações com tendência no relatório; uma trava por pilha exportada.
- Testes novos: contratos sem repetição e avisos, prêmio e mercado aberto, carga que espera e reputação que volta ao reabrir.

## 0.23.0: comércio exterior refeito

Desenho, problema e porquê em [frentes/SOCIEDADE.md](frentes/SOCIEDADE.md#comércio-exterior-023-no-estilo-das-conexões-externas-de-cities-skylines).

- `society/trade.rs` substitui o porto da 0.22: conexões externas físicas (porto de carga exige água; estação de carga, fronteira), capacidade, frete e horário; bolsa com índice diário determinístico e saturação pelas exportações recentes; contratos com prazo, prêmio e reputação (salvos no centro de controle); importação automática de pão pelas lojas com falta, com tarifa para a prefeitura e teto do preço local na paridade de importação.
- Nova aba *Comércio exterior* no centro de controle (11 abas).
- Testes: bolsa e camadas, porto que exige mar e parte no horário com frete, estação na fronteira, contrato com prêmio e reputação, importação com a identidade contábil fechando.

## 0.22.0: comércio exterior

- **Porto de exportação** (`society/export.rs`, empresa de logística): 9 espaços, um navio por turno. Preço mundial por complexidade (1x a 4x) e saturação diária por item; sem centro de controle (cofre da companhia), o navio não sai.
- A venda é moeda de fora: entra como borda (contada à parte como exportação), com imposto na fonte, e a receita vai para a empresa que faz o item. O PIB da prefeitura passa a ser consumo + exportação.
- O relatório do centro de controle sugere os 3 itens guardados que mais rendem lá fora hoje; a visão geral da holding mostra a exportação do dia.
- Testes: preços por camada e saturação; o navio sai na hora certa, leva tudo e a identidade contábil fecha com a moeda de fora.

## 0.21.0: economia fechada

- **Imposto na fonte:** cada venda (varejo e atacado) já manda a parte da prefeitura (4%, 8%, 12%, 16%). Saíram a dívida de imposto, a verba inicial da prefeitura e a manutenção dos poços: nada cria nem destrói moeda.
- **Comerciantes independentes:** o estoque entregue pela indústria vira atacado (70% do preço local) devido à companhia, por empresa; a loja paga com o caixa (fiado quando falta) e paga o próprio comerciante. O antigo caixa em esmeraldas virou só retirada.
- **Três livros + famílias** em `society/ledger.rs`, com as bordas (esmeraldas que entram e saem) contadas. Teste `money_circulates_without_appearing_or_vanishing`: a soma de todos os saldos não muda dentro da economia.
- **Saque:** clique na linha da companhia no painel do centro de controle (botão de menu); a esmeralda sai no novo 6º espaço.
- **Holding:** cada aba mostra receita (atacado dos itens da empresa), gastos (salários dos postos da empresa e imposto) e lucro do dia; o relatório também.

## 0.20.0: holding no centro de controle

- O centro de controle virou a sede da companhia como **holding**. Setas `<` `>` no cabeçalho do painel trocam a aba (clique na tela → botão de menu no servidor): visão geral e nove empresas — mineração, manufatura, energia, logística (inclui armazéns e capatazes), agro (fazendas, cercados, comida), extrativismo (fazendas de árvores: madeira e folhas), urbanização (casas e poços), comércio (lojas) e gestão e segurança (centro, quartel, Tesla). A prefeitura é pública e fica fora.
- Cada aba mostra unidades, ativas, trabalhadores, folha estimada, produção do dia e o estado parado mais comum, mais números próprios (zonas no agro; casas, moradores e alvarás na urbanização; lojas e preços no comércio). O livro no centro dá uma linha por empresa.
- **Leve:** `holding.rs` registra cada bloco uma vez a cada 100 ticks (fase pela posição) num mapa por dimensão; a produção é o aumento do contador do bloco, somado por turno e virando média do dia. A aba aberta é recalculada a cada 4 ciclos do centro, nunca por tick. O teste de alocação zero continua passando.

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

## Sétima entrega: revisão de aldeões e mobs (com o quartel do Grok)

Revisão feita sobre a main `cf37004` com a casa do fundador (sexta entrega) e a branch `cursor/grok-quartel-c7d9` integrada (mobs caçando aldeões, saqueadores atrás do jogador, quartel). Só ajustes; nada foi reescrito.

### Correções

- **Relógio do mod em dobro no jogo de um jogador só.** O gancho `tick` roda no servidor interno e também na janela (o Horizon chama os mods dos dois lados). O Veio avançava o relógio duas vezes por tick, em threads diferentes: tudo medido nele (espera do spawn, ausência do trabalhador, ordens, reservas, natalidade, bônus da casa e da comida) corria no dobro da velocidade, e metade dos aldeões nunca tinha filhos (o sorteio `(relógio + id) % 6000` só cai em números pares). As cotas do orçamento global também eram zeradas no meio do tick do servidor, e um passo da poluição podia ser gasto na janela, onde não há jogadores nem `summon`: a onda daquela noite se perdia. Agora o gancho sai logo quando `Host::window()` é verdadeiro. Os tempos documentados (5 min, 30 s...) passam a valer de fato; no servidor dedicado nada muda.
- **Renascer no centro da vila não funcionava.** `run_command` falha dentro de `player_respawned` (o estado do jogador está trocado). O teleporte vai para uma fila e sai no tick seguinte.
- **Onda sem saqueadores.** Se nenhum saqueador cabe (chunks em volta descarregados), a noite não conta, nada é anunciado e o passo seguinte tenta de novo. Eles nascem no chão, não em cima de árvores, e o som de invasão só toca se alguém apareceu.
- **Alvos de outra dimensão.** Saqueadores só miram máquinas do Overworld (antes, uma máquina do Nether com as mesmas coordenadas servia de alvo).
- **Limpeza dos registros de mobs e animais** (`defense`, `production::ranch`): a marca de limpeza era uma só para todas as dimensões, então só uma dimensão era limpa; agora todas são limpas juntas.

### Ajustes no trabalho do Grok

- Zumbis, esqueletos e aranhas só caçam aldeões **à noite** (o código caçava a qualquer hora; o texto já dizia "à noite"). Saqueadores caçam sempre.
- A presa precisa estar a até 6 blocos de altura: um monstro da caverna não tenta subir até a vila.
- Esqueletos, strays, bogged e pillagers com o jogador a até 16 blocos continuam atirando nele em vez de largar a luta para ir atrás de aldeões (os de corpo a corpo seguem com 6 blocos).
- Aldeões feridos se recuperam depois de 30 s sem apanhar (antes, os golpes ficavam para sempre).
- Os registros de jogadores e aldeões dos mobs se refazem a cada 10 ticks, não a cada tick.
- O quartel ignora monstros mais de 8 blocos acima ou abaixo dele, e o soldado corre (velocidade 0,9 em vez de 0,6) e refaz o caminho duas vezes por segundo atrás do alvo.

### Aldeões

- **Contratação** prefere quem não faz falta no comércio (sem profissão ou nitwit, depois nível 1, por último os experientes).
- **Lavradores e pastores dormem à noite.** Das 13000 às 23000, depois de entregar o que carregam, a ordem é retirada e o aldeão volta à vida vanilla (ir para a cama). O painel mostra o estado novo 29, **Descansando (noite)**. Carregador, capataz, administrador e soldados seguem trabalhando. Dormir numa cama longe do posto não conta como ausência (antes, a noite de 8 min passava do limite de 5 min e o posto dispensava o trabalhador).

### Contratos tocados

- `lib.rs`: saída antecipada do gancho `tick` na janela; `note_players` e `harm_villagers` recebem o relógio.
- `presentation.rs`: `STATUS_KEYS` com 30 entradas (`freight_route` do Codex em 28, `resting` em 29, cor azul).
- `pollution.rs`: `night_at`, `is_night`; `spawn_wave` devolve quantos nasceram.
- Substituto offline: `Host::window`.

### Verificação

Substituto offline: 135 testes, entre eles onda que não nasce, contratação por profissão, descanso noturno e o teste do quartel do Grok. Conteúdo conferido. Pendentes: compilação pelo launcher e teste no jogo (prioridade: confirmar que `window()` é falso no servidor interno e verdadeiro na janela, e que `host.entity` funciona no tick global, que o censo de aldeões dos mobs usa).

## Sexta entrega: casa do fundador (missão de controle + casa)

### Para o jogador

- **A casa já vem pronta.** Uma vez por mundo, quando o centro da vila é conhecido (ou cerca de um minuto depois do primeiro jogador entrar), o Veio constrói a casa do fundador no terreno mais plano ao norte do centro, com a porta virada para a vila. Estilo das casas de planície: fundação de pedregulho, cantos de tronco, tábuas de carvalho, telhado de escadas, chaminé de tijolo.
- **Dentro:** o centro de controle (o painel), lareira com fogueira acesa, sofá de frente para o fogo com tapete, mesa com duas cadeiras, estante, baú, cama ao lado do painel com criado-mudo e velas, duas lanternas penduradas. Meia luz.
- **Mural:** quatro mapas (2 x 2) na parede oeste mostram 256 x 256 blocos em volta da casa. Só aparece o que já foi visto (o que está ou esteve carregado); o resto fica como papel. Máquinas do Veio em laranja, a casa em vermelho. O mural se refaz em cerca de 25 s.
- **Em casa: +15%.** Com o jogador dentro da casa (até 9 blocos do painel, debaixo do telhado), os ciclos das máquinas ficam 13% mais curtos (1 / 0,87 = 15% mais produtos por minuto). A energia por tick não muda, então cada produto sai mais barato. A barra de ação avisa ao entrar e ao sair.
- **Duas camas.** A primeira cama de cada jogador tem de ser uma da casa (a que vem ao lado do painel serve). Depois disso vale só mais uma fora. Para trocar a de fora, basta quebrá-la. Quem tem cama renasce nela; quem ainda não tem continua indo ao centro da vila.
- **Comida.** Refeição (carnes e peixes assados, pão, batata assada, torta): pressa I e máquinas +5% por 5 min. Banquete (ensopados, sopas, cenoura dourada): pressa I, um pouco de regeneração e máquinas +10% por 8 min. Comer dentro de casa dobra a duração. Lanches (cru, frutas, biscoito) só matam a fome. Entre jogadores vale o maior bônus, sem somar. Junto com a casa, o máximo é 0,87 x 0,91: 26% mais produção.
- O Guia da Vila Operária ganhou as páginas "Sua casa" e "Refeições" (25 páginas no total).

### Código e contratos tocados

- Novo módulo `native/api/src/home/`: `mod.rs` (construção, presença, camas), `mural.rs`, `food.rs`, `tests.rs`.
- Nova planta `tools/build_house.py` → `content/data/veio/function/home/build.mcfunction` (251 comandos `fill`/`setblock`/`summon`, coordenadas relativas). O Rust roda a função uma vez com `execute in minecraft:overworld positioned X Y Z run function veio:home/build`. A marca `veio:home built` no armazenamento de comandos do mundo impede uma segunda casa. Um teste confere painel, cama e molduras do arquivo gerado contra as constantes do Rust.
- `lib.rs`: `mod home`; ganchos novos `player_spawn_point_set` e `player_bed_enter` (contrato `cancel`) e `living_entity_use_item_finish` (`listen`); `home::tick` no tick global; `player_respawned` passa o UUID; chaves NBT `Home` e `HomeBeds` na tabela de chaves.
- `upgrades::effect`: multiplica `time` por `home::time_factor`. Atômicos, sem trava no caminho das máquinas. Módulos, energia por tick e poluição não mudam.
- `workforce/mod.rs`: o tick do centro de controle chama `home::center_tick` (só o painel da casa faz algo). `workforce/spawn.rs`: `respawned` não teleporta quem tem cama registrada.
- O painel da casa guarda `Home = 1` e as camas em `HomeBeds` (texto curto). Nada novo no menu nem nos 13 valores do painel.
- APIs novas, conferidas no código do Horizon `66f3981`: `Host::send_map` (API 132), `Host::acting_player` (API 21), ganchos `player_spawn_point_set` (277), `player_bed_enter` (251), `living_entity_use_item_finish` (90). Substituto offline atualizado.

### Desempenho

- O painel da casa lê uma linha do mural a cada 2 ticks: uma consulta de 256 alturas e 256 leituras de bloco. Custo fixo, um painel só no mundo; não entra no orçamento global (`frentes/ORCAMENTO-GLOBAL.md`) porque não cresce com a fábrica. Os 64 KiB de pixels ficam na memória.
- Presença: a cada 20 ticks, uma consulta de altura só para os jogadores perto do painel.
- Envio do mural: 4 mapas de 16 KiB a quem chega perto da casa e a cada 10 s para quem está nela.
- A construção é uma chamada de função única, uma vez por mundo, e a escolha do terreno faz duas consultas de 450 colunas.

### Números para revisão (Grok)

`HOME_TIME 0,87`, `MEAL_TIME 0,95` por 5 min, `FEAST_TIME 0,91` por 8 min, comer em casa dobra. Todos em `home/mod.rs` e `home/food.rs`.

### Limitações conhecidas

- O bônus da casa vale para todas as máquinas do mundo, não só as perto da casa (o efeito do módulo não conhece a posição da máquina).
- Quebrar o painel da casa desliga o bônus; recolocado no mesmo lugar ou perto (até 9 blocos), na mesma sessão, ele volta a ser o da casa.
- Mundos antigos também recebem a casa (a marca no armazenamento ainda não existe neles), perto do centro de controle com administrador ou do primeiro jogador.
- A comida dos aldeões continua a do administrador e dos postos (pontos por item); o próximo passo é a refeição em casa valer para eles também.
- Pendente: teste no jogo de `/function` com bloco do Veio e molduras com `minecraft:map_id`, de `send_map` nas molduras e da ordem cama → ponto de renascimento.

### Teste sugerido ao usuário

1. Mundo novo: espere aparecer a mensagem da casa e vá até ela. Confira lareira, sofá, cama, mural.
2. Fique dentro perto do painel: a barra de ação avisa o +15%. Saia: avisa de novo.
3. Tente dormir numa cama fora antes da da casa: recusada. Durma na da casa, depois numa fora, depois numa terceira: a terceira é recusada até você quebrar a segunda.
4. Coma pão fora e um ensopado em casa: veja as mensagens e a pressa.
5. Ande pela região e volte: o mural mostra o que você viu.
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

## Décima entrega: funcionários, centro de pesquisa, ruínas e território

### Funcionários e pesquisa

Com o prédio de pé, o canteiro vira o posto do prédio (`staff.rs`): esmeralda contrata o funcionário e a comida vai no slot da esquerda. O canteiro tem **3 slots**: comida, entrada do funcionário e produto.

| Prédio | Funcionário | O que faz |
| --- | --- | --- |
| Refeitório | cozinheira | Carne, peixe e batata crus da entrada saem prontos |
| Escola | professor | Na hora da aula as crianças vão às carteiras e crescem mais rápido |
| Ferraria | ferreiro | 2 chapas de ferro viram 2 kits de reparo |
| Biblioteca | bibliotecário | 3 papéis viram 1 livro |
| Correio | carteiro | Ronda pelas casas e postos |
| Estalagem | estalajadeiro | À noitinha os hóspedes vêm às mesas |
| Centro de pesquisa | cientista | Pesquisa melhorias para a vila |

A praça funciona com 2 lojas nos lugares reservados e a prefeitura com o bloco dela. O efeito da obra só vale com o prédio **funcionando**; de pé e parado, o centro mostra "de pé · parado".

**Centro de pesquisa** (obra `lab`, nível Vila): o cientista pesquisa em ordem, com os materiais entregues na entrada e 24 tarefas cada: **Agronomia** (zonas +1 de trabalho por tarefa), **Logística** (carregadores +8 itens por viagem), **Nutrição** (tarefas gastam 1 ponto de comida a menos), **Pedagogia** (aula em dobro na escola) e **Engenharia** (módulo na entrada: o cientista instala em máquinas do Veio a até 12 blocos). A mensagem de pesquisa concluída usa `veio.research.lab_done` (a chave `veio.research.done` é da pesquisa da grande indústria).

### Ruínas da vila

Na primeira chegada a um mundo novo (até o fim do primeiro dia), quando o centro da vila é achado, o Veio põe até **3 ruínas** de obras sem exigência de prestígio (refeitório, escola, ferraria, praça) em volta do centro (`ruins.rs`):

- lugares a 50, 56 ou 62 blocos do centro, em 16 direções (o anel de 48–80 blocos, limitado a 62 para o centro de controle no meio da vila alcançar a obra a até 64 blocos);
- terreno firme e plano sob toda a planta: `MotionBlockingNoLeaves` igual a `OceanFloor` (sem água nem lavoura irrigada) e desnível de até 3 blocos (casas e árvores ficam de fora); sem sobrepor outra ruína, com 3 blocos de folga;
- o canteiro (`veio:site`, virado para o centro) só entra onde o bloco é substituível (`execute if block ... #minecraft:replaceable run setblock`);
- no primeiro tick, o canteiro pega a planta escolhida e fica com a **planta fixa** (`Fixed`; a chave não troca) e carimba a ruína: as 4 camadas de baixo, um subconjunto que depende só do lugar (90%, 70%, 45% e 20% por camada), sem móveis, sem blocos com propriedades e sem tocar em blocos protegidos que já estejam ali.

O jogador paga a obra no centro e o mestre de obras reconstrói por cima da ruína. `<mundo>/veio/ruins.txt` guarda que as ruínas já foram postas e quais canteiros ainda não pegaram a planta (chunk descarregado ou mundo fechado no meio): reabrir o mundo ou outro jogador chegar não repete nada. Mundos que já existiam não ganham ruínas.

### Território das obras

Ruínas e obras em construção ou de pé protegem o **território**: a planta mais 3 blocos em todos os eixos (`territory.rs`, ganchos `destroy_block`, `left_click_block`, `use_item_on` e `explosion_detonate`).

- Livre: terra, grama, flores, folhas, areia, cascalho, neve e plantas naturais; jogadores no criativo passam em tudo.
- Quebrar: recusado no resto. O canteiro do jogador pode sair (o prédio deixa de valer na hora); o da ruína, não. Numa ruína ainda à espera da obra, os blocos que o mestre de obras não derruba (baús, camas, máquinas) podem sair, para a conferência do terreno não parar para sempre.
- Usar item num bloco: comida, esmeralda, chave, livros, bússola, armas e outros itens de uso passam; o bloco do lugar reservado (loja na praça, bloco da prefeitura) pode ser posto; abrir portas, baús e blocos do Veio sem agachar passa. Pôr blocos, baldes, isqueiro, machado, enxada e afins são recusados (barra de ação: "Território da obra").
- Explosões não levam os blocos protegidos (o chão natural ainda voa).

O registro `BUILDINGS` ganhou a área, a planta fixa e se a entrada **conta** em `site::near()`: a ruína à espera publica só o território, sem ocupar a obra na vila; as obras em andamento e de pé contam como antes.

### Contratos tocados

- Substituto do Horizon: `DestroyBlockEvent`, `LeftClickBlockEvent`, `UseItemOnEvent<'_>`, `ExplosionDetonateEvent<'_>` e os quatro ganchos, com as assinaturas do handoff (a compilação pelo launcher confere).
- `lib.rs`: os quatro ganchos chamam `workforce::territory`; `server_about_to_start` zera as ruínas; chave NBT `Fixed` na tabela rápida.
- Centro de controle: rótulos dos slots 6 (obra) e 7 (decreto).
- Testes: `ruins_pick_firm_flat_spots_in_the_ring_facing_the_center`, `ruin_stamp_keeps_low_layers_without_furniture_and_is_deterministic`, `ruins_are_placed_once_per_new_world_and_their_sites_keep_a_fixed_plan`, `territory_covers_the_plan_plus_margin_only_for_ruins_and_real_works`, `protection_frees_natural_ground_creative_players_and_food` e `protection_hooks_cancel_breaking_building_and_explosions_inside_the_grounds`.

### A conferir no jogo

- Se `run_command` com `execute if block ... run setblock` devolve `false` quando o bloco não é substituível (hoje a ruína fica pendente no arquivo, sem efeito).
- Se os ganchos de bloco do Horizon têm exatamente as assinaturas do substituto (em especial o retorno de `explosion_detonate` e o tipo de `face`).
- Se cancelar `use_item_on` recusa também o uso do item (por isso comida e itens de uso passam).
- O carimbo das ruínas em chunks recém-gerados e o mestre de obras reconstruindo por cima.

## Nona entrega: plantas prontas e canteiro de obras

As obras com prédio (refeitório, escola, ferraria, praça do mercado, biblioteca, correio, estalagem e prefeitura) agora têm **plantas prontas**, desenhadas em texto camada por camada em `tools/blueprints.py` (o mesmo jeito das texturas). `python tools/build_blueprints.py` confere e gera `native/api/src/workforce/blueprint_data.rs`. A festa continua sendo só festa.

### As plantas

| Obra | Tamanho | Personalidade |
| --- | --- | --- |
| Refeitório | 9x7, 9 de altura | Salão de tijolos, mesa comprida para 10, cozinha com defumadores e caldeirão, chaminé com fogueira no alto |
| Escola | 9x9, 11 | Bétula com telhado vermelho, quadro-negro, púlpito, dez carteiras e campanário com sino |
| Ferraria | 9x7, 10 | Pedra e ardósia, alto-forno, caldeirão de lava, bigorna e rebolo na frente aberta, chaminé alta |
| Praça do mercado | 11x11, 4 | Calçamento xadrez, chafariz, quatro bancas de toldo colorido com um lugar para **loja** em cada, bancos e postes |
| Biblioteca | 9x9, 6 | Carvalho-escuro forrado de estantes, mesas com velas, tapete vermelho, lanternas e claraboia |
| Correio | 7x8, 8 | Tijolo e abeto, faixa amarela e azul, balcão, barris de triagem, caixa de correio e bandeira |
| Estalagem | 11x9, 10 | Enxaimel: salão com lareira, mesa e barris; dormitório com **8 camas**; chaminé |
| Prefeitura | 11x11, 9 | Pedra e quartzo, pórtico de colunas, escadaria, frontão, tapete até a mesa, campanário e o lugar do **bloco da prefeitura** (0.18) |

### Como se constrói

1. **Pagar:** os materiais vão ao slot Obra do centro, como antes. Paga, a obra aparece como "paga · faça o canteiro" e ainda **não tem efeito**.
2. **Canteiro de obras** (`veio:site`, receita: bancada, cerca, papel e corante azul, liberada com o centro de controle): o jogador põe onde quer o prédio. A borda azul aparece no chão e a seta amarela mostra a entrada, virada para onde o canteiro olha. O canteiro fica dentro do prédio, no lugar marcado na planta. **Chave** troca a planta.
3. **Terreno:** com a obra paga no centro a até 64 blocos, o canteiro confere a área (48 blocos por ciclo, cota de trabalhadores). Terra, pedra, grama, árvores e água saem; baús, camas, portas, máquinas e outros blocos do jogador param a obra, e o painel mostra onde está o bloco (borda vermelha). Conferido, a obra começa.
4. **Mestre de obras:** contratado com esmeralda (o capataz também o alimenta e recontrata). Anda até cada parte e assenta 10 blocos por tarefa (cada tarefa gasta comida). Primeiro o chão embaixo do piso é completado com pedregulho onde houver ar, água ou plantas; depois, de baixo para cima, as paredes, o telhado e o ar a limpar; por último os móveis e enfeites (portas, camas, lanternas, vasos, sinos). Uma parte que ele não alcança andando é feita do próprio canteiro.
5. **De pé:** o mestre é dispensado, quem está perto recebe o aviso, e o centro passa a contar o prédio: o efeito da obra liga. Os lugares reservados (lojas da praça, bloco da prefeitura) piscam até o jogador pôr o bloco do Veio ali.

O prédio vale enquanto o canteiro existir. Dois canteiros não constroem o mesmo prédio na mesma vila. Na página Atenção, canteiro sem mestre ou com terreno ocupado vira pendência (e entra no foco da bússola).

### Contratos tocados

- `projects.rs`: `Built` passa a significar **paga**; o efeito usa `standing` (paga e com prédio de pé, `Standing` atualizado pelo centro a partir de `site::near`). `Village.funded` publica as obras pagas.
- `site.rs` (novo) é um posto (`is_post`) com fases em `ContractStage`; registro em memória `BUILDINGS`. `blueprints.rs` gira plantas e propriedades (`facing`, `axis`).
- `presentation.rs`: estados 36-39 (`site_plan`, `site_blocked`, `site_building`, `site_done`; 32-35 ficaram com as conexões externas da 0.23); `operations.rs`: canteiro no setor de coordenação, bloqueio "ambiente" no terreno ocupado.
- Mundo: só `edit_blocks` (com o filtro `#minecraft:replaceable` no alicerce) e `set_block_property`, que já existiam.

### A conferir no jogo

- Se `edit_blocks` cria os blocos com entidade (baús, barris, púlpitos, sinos, camas, fogueiras) e se `set_block_property` acerta portas e camas de duas partes.
- Cercas e vidros não se conectam sozinhos depois de `edit_blocks` (sem atualização de vizinhos); por isso as plantas usam vidro inteiro e cercas só como postes.
- O mestre de obras andando dentro do prédio em construção.

### Números para revisão (Grok)

10 blocos por tarefa; conferência 48 blocos por ciclo; reconferência de terreno ocupado a cada 200 ticks; alcance do mestre 5 blocos; alicerce até 3 blocos. As plantas têm de 300 a 700 células (4.090 no total).

## Oitava entrega: prestígio, pedidos, decretos, boletim, obras maiores e banquete

Continuação direta da sétima entrega, feita sem teste no jogo e sem assinaturas novas do Horizon (além das usadas antes: `summon`, `entity_nbt`, `give_items`/`take_items`, `run_command`).

### Prestígio da vila

Pontos guardados no centro (`Prestige`): pedido atendido +10, obra pronta +25, festa +10, banquete no lar +5, dia em ordem no lar +2, manhã sem pendências +3 (prefeitura: +50%). Níveis: **Aldeia** (0), **Vila** (60), **Vila próspera** (200), **Cidade operária** (500). Cada nível soma 5 ao teto da vila, libera obras e deixa os pedidos mais ricos. Subir de nível é anunciado a quem está na vila.

### Pedidos da vila (página 4 do centro)

- Toda manhã a vila tem 3 pedidos abertos (4 com o correio), sorteados pelo dia e pela posição do centro, sem repetir item: pão, chapas, vidro, tijolos, lã, tochas... e, com prestígio, engrenagens, aço, fiação, lanternas, circuitos, módulos, bolos, cenouras douradas, trilhos, maçãs douradas.
- Entrega pelo slot **Obra** do centro; os pedidos têm prioridade sobre as obras. Cada pedido atendido paga esmeraldas ao **caixa** da vila (correio: +50%) e +10 de prestígio.
- **Nada entregue se perde:** um pedido espera até ser atendido. Na manhã seguinte, os atendidos viram pedidos novos; um pedido sem nenhuma entrega há 3 dias é trocado.

### Caixa da vila

- Recebe as recompensas dos pedidos e as esmeraldas que sobram no slot Obra.
- Paga sozinho as esmeraldas das obras (praça do mercado) e as **expansões de zona** a até 64 blocos do centro (o jogador completa o que faltar; se não completar, o caixa é devolvido).
- **Agachar + esmeralda no centro:** saca até 16. Com a prefeitura, rende 1 a cada 16 por dia (até 8).

### Decretos (slot novo do centro)

O item fica no slot e não é gasto; vale enquanto estiver lá e o administrador estiver contratado.

| Item | Decreto | Efeito |
| --- | --- | --- |
| Sino | Convocação | Postos contratam aldeões a até 16 blocos (em vez de 8) |
| Papel | Racionamento | Tarefas custam 1 ponto de comida; ninguém nasce |
| Fardo de feno | Fartura | Tarefas custam 3 pontos; zonas ganham o dobro de trabalho; natalidade 1/4 |

Decretos de horário (turno estendido) ficaram de fora de propósito: à noite o cérebro vanilla entra em descanso e o objetivo `Work` não interfere nele.

### Obras maiores (liberadas pelo prestígio)

| Obra | Prestígio | Materiais | Efeito |
| --- | --- | --- | --- |
| Biblioteca | Vila | 8 estantes, 16 livros, 4 lanternas | Zonas ganham o dobro de trabalho por tarefa |
| Correio | Vila | 16 papéis, 4 baús, 8 chapas de cobre | 4 pedidos por dia e +50% de esmeraldas |
| Estalagem | Vila próspera | 8 camas brancas, 32 tábuas de carvalho, 4 chapas de aço | Teto +20; toda manhã chega um viajante se houver lugar |
| Prefeitura | Cidade operária | 4 blocos de ouro, 1 sino, 8 circuitos | Prestígio +50%; juros no caixa |

Material de uma obra bloqueada fica no slot até ela liberar. A página Obras mostra primeiro as em andamento, depois as liberadas, as bloqueadas (com o prestígio exigido) e uma linha com as prontas (cursor: efeitos de todas).

### Boletim da manhã

Quando o dia vira, quem está a até 64 blocos do centro recebe no chat: dia da vila e nível, tarefas de ontem, aldeões e teto (com a variação), pedidos atendidos, juros e viajantes, os pedidos de hoje e as pendências (ou "vila em ordem", +3 de prestígio).

### Banquete no lar

**Bolo no lar**, com a lareira acesa, uma vez por dia: até 6 aldeões adultos livres a até 32 blocos vêm à sua casa por um minuto (as mesmas ordens dos postos; o cérebro vanilla os leva). Aldeões contratados não são chamados, e quem for contratado durante o banquete vai trabalhar. Quem oferece ganha **Herói da Vila** por 20 minutos (descontos no comércio vanilla; nível II na casa de engenheiro) e a vila ganha +5 de prestígio. O escritório do lar (nível 3) passa a mostrar também os pedidos abertos.

### Contratos tocados

- Centro de controle: **6 slots** (menu e `build_content.py`; o 6º é o decreto). Dados da página 4 em `DD*`; página Obras com 9 obras, os efeitos e o prestígio.
- `projects::Village` ganhou `requests`; efeitos novos no mesmo registro (`PERKS`): bits de obra, decreto e nível de prestígio.
- `PostInfo.total` (tarefas do posto) para o boletim.
- `mod.rs`: custo de comida por `projects::food_cost` (comer antes da tarefa cobre esse custo; na festa, 0); raio de contratação por `projects::hire_radius`; agachar + esmeralda no centro saca o caixa.
- `zones.rs`: crédito de trabalho por `projects::zone_credit`; expansão paga primeiro pelo caixa. `village.rs`: racionamento impede nascimentos.
- Chaves NBT novas na tabela de `nbt_key` (o teste de alocação continua zerado).

### Números para revisão (Grok)

Prestígio e níveis acima; 27 itens de pedido com faixas e recompensas em `requests.rs`; pedido parado troca em 3 dias; decretos 1/2/3 pontos; juros 1/16 até 8; banquete 6 convidados, 32 blocos, 1.200 ticks, herói por 1.200 s.

### Teste sugerido ao usuário

1. Centro com administrador: chave até a página 4. Entregue um pedido pelo slot Obra e veja o caixa e o prestígio. Agachar + esmeralda para sacar.
2. Durma uma noite perto do centro: confira o boletim da manhã.
3. Sino no slot Decreto: contrate um aldeão a ~12 blocos de um posto. Papel: confira que ninguém nasce.
4. Lareira acesa, bolo no lar perto de aldeões: eles devem vir até a casa; veja os preços com o efeito de herói.
5. Junte 60 de prestígio e confira a biblioteca e o correio liberados.

## Sétima entrega: lar do jogador, painel executivo, foco e obras da vila

Feita sem teste no jogo. Nenhuma assinatura nova do Horizon: usa `tell`, `action_bar`, `run_command`, `server_players`, `day_time`, `text_width` e as partículas `Flame`, `Smoke` e `HappyVillager`, que já estavam no substituto.

### Centro de controle: painel executivo em três páginas

A **chave** (sem agachar) troca a página; o título mostra qual (ex.: "Atenção 2/3"). O painel atualiza também à noite, sempre que há administrador contratado.

1. **Resumo:** aldeões perto e teto da vila, trabalhadores e ativos, esperas e sem comida, **comida guardada em tarefas** (soma dos armazéns), lavouras/cercados e quantas zonas **já podem crescer**, armazéns e fila, e as três metas.
2. **Atenção:** pendências da vila em ordem de urgência (sem comida, ausente, sem trabalhador, sem esmeralda, caminho obstruído, saída cheia, zona pronta para crescer, armazém com 90%+), com distância e direção a partir do centro. Cursor na linha: o que fazer ali e o deslocamento x/y/z.
3. **Obras:** cada obra da vila com o progresso e o próximo material que falta. Cursor na linha: efeito e lista de materiais.

Cada linha do Resumo também explica o número ao passar o cursor.

### Foco (bússola)

- **Bússola no centro:** o jogador recebe a pendência mais urgente; cliques seguidos passam para a próxima. **Agachar + bússola** limpa.
- A barra de ação mostra "➤ Lavrador: sem comida · 23 m NE"; partículas verdes marcam o bloco. Perto dele (3,5 blocos), a barra mostra a dica do que fazer ali.
- Quando o problema some (o posto voltou a trabalhar, a zona subiu de nível, o armazém esvaziou), toca um sinal e vem a próxima pendência da mesma vila, até "Vila em ordem". O foco vence em 10 minutos.
- Só lê os registros em memória (`POSTS`, `ZONES`, `STORES`), a cada 10 ticks do relógio e só no lado do servidor (a lista de jogadores vazia da janela não consome a vez).

### Obras da vila (slot **Obra** do centro)

O centro ganhou um 5º slot (o canteiro). Cada material vai para a primeira obra que ainda o pede; sobras ficam no slot. As obras valem para tudo a até 64 blocos do centro enquanto ele tiver administrador contratado.

| Obra | Materiais | Efeito |
| --- | --- | --- |
| Refeitório | 24 tijolos, 8 chapas de ferro, 1 cozinha | Toda 4ª tarefa dos postos não gasta comida |
| Escola | 24 papéis, 6 livros, 16 tijolos | Natalidade 1/3 (era 1/6) e teto +15 |
| Ferraria | 8 chapas de aço, 8 engrenagens, 1 bigorna | Carregador leva 24 itens e recolhe 48 |
| Praça do mercado | 16 vidros, 16 chapas de cobre, 8 esmeraldas | Expandir zonas custa 25% menos |
| Festa da colheita (repetível) | 1 bolo, 16 biscoitos, 16 pães | Hoje e amanhã: tarefas sem comida, natalidade 1/2 |

### Lar (`veio:home`)

Receita (liberada desde o começo, pela aba Vila operária): fogueira, barril, vaso de flor e tijolos. Slots: lenha, despensa (3) e obra.

- **Dono e renascer:** o primeiro jogador a até 8 blocos vira o dono (`spawnpoint`); ao renascer, ele vai para o lar em vez do centro da vila. Agachar + chave passa o lar para quem clicou.
- **Lareira:** carvão/carvão vegetal 4 min, biomassa 2 min, troncos 80 s, tábuas 30 s, bloco de carvão e de alga seca. Fumaça e chamas quando acesa.
- **Refeição em casa:** tigela no lar. Escolhe o prato mais valioso que não foi comido nas últimas 5 refeições. Sempre enche a fome; com a lareira acesa: regeneração (mais longa para comida melhor), **pressa** com 3 pratos diferentes, **resistência** com 5. Cenoura dourada dá visão noturna; maçã dourada, absorção. Duração x (0,5 + conforto/100), x1,5 a partir do nível 2. Uma refeição a cada 2 min.
- **Conforto (0-100):** lê 9x9, de 1 abaixo a 3 acima, 27 blocos por ciclo (cota `Workforce` do orçamento global): cama 15, porta 10, janela 10, luz 10, baú 10, fogão 10, bancada 10, enfeites até 10 (3), acabamento até 15 (24 blocos de tábuas/tijolos/polidos/terracota).
- **Descanso:** se a lareira esteve acesa durante a noite e o dono está a até 12 blocos de manhã: regeneração e velocidade (e pressa no nível 4).
- **Dia em ordem:** comeu em casa, lareira acesa e 3+ comidas na despensa → experiência (10 + 5 por dia de sequência, até 40) e a sequência sobe.
- **Melhorias:** nível 2 (24 tijolos, 12 vidros, 8 chapas de cobre: a lareira cozinha carne/peixe/batata crus da despensa e efeitos +50%); nível 3 (8 fiações, 2 circuitos, 8 chapas de ferro: **escritório** — o painel mostra aldeões, teto e pendências do centro mais próximo, e a bússola no lar dá o foco); nível 4 (4 chapas de aço, 4 engrenagens, 1 módulo: pressa II nas refeições e pressa no descanso).
- O painel lista o que fazer: lareira, despensa, refeição de hoje, o que falta no conforto, a próxima melhoria e as pendências da vila.

### Atalhos da fábrica

Receitas sem posição, liberadas ao obter chapas, engrenagens ou circuitos (`advancement/shortcuts.json`): balde, funil, lanternas, para-raios, pistão, **bússola** (circuito + chapa), relógio, lâmpadas de redstone e trilhos (aço + graveto). Página nova no Guia da Vila Operária, junto de "Foco e obras" e "Seu lar" (29 páginas).

### Contratos tocados

- `lib.rs`: o `tick` chama `workforce::world_tick` (spawn e foco) no lugar de `spawn::tick`; chaves NBT novas na tabela de `nbt_key` (painel, obras, lar) para o publish continuar sem alocação.
- `presentation.rs`: tela e dados próprios do lar; o lar não desenha luzes de máquina. `tutorial.rs`: `workforce::screen_tooltip` antes das dicas genéricas.
- Centro de controle: 5 slots (menu e `build_content.py`); dados do painel por página (`DA*`, `DB*`, `DC*`; índice 12 = página). Saves antigos: o 5º slot entra vazio; `DWorkers` e afins ficam sem uso.
- `village.rs`: teto + escola; sorteio da natalidade com a chance da obra (o 1/2 da festa filtra antes de ler a posição). **Conflito previsto** com `cursor/grok-vilas-c7d9` (camas): as duas mudanças são pequenas e independentes.
- `hauler.rs`: tamanho da viagem por `projects::haul_batch`. `zones.rs`: preço com desconto (`Market` no NBT do marco). `spawn.rs`: renascer no lar.
- Registros novos em memória: `projects::PERKS` (centros com administrador, efeitos e resumo), `home::HOMES` (lar por dono, sem vencimento: um lar quebrado continua valendo para renascer até reabrir o mundo), `focus::FOCUS`.

### Números para revisão (Grok)

Obras e efeitos acima; refeição a cada 2.400 ticks; lenha 4.800/2.400/1.600/600; conforto 9x9x5 a 27 blocos/ciclo; cozinhar 1 item a cada 400 ticks; XP diário 10 + 5/dia até 40.

### Teste sugerido ao usuário

1. Fabrique um lar, ponha dentro de casa com carvão e pão: confira dono, fumaça e o painel. Tigela no lar; varie a comida e veja os efeitos. Morra e veja se renasce em casa.
2. Construa cama, porta, janela, luz etc. e acompanhe o conforto subir. Ponha tijolos, vidro e chapas no slot Obra.
3. No centro: chave troca as páginas; deixe um posto sem comida e use uma bússola no centro; siga a barra e resolva.
4. Ponha tijolos, chapas de ferro e uma cozinha no slot Obra do centro (refeitório) e um bolo, biscoitos e pão (festa).

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

- A cada 5 minutos (fase espalhada pelo id), cada aldeão adulto tem 1 chance em 6 de ter um filho, se houver pelo menos 2 aldeões perto, menos que o teto da vila e uma cama livre. O teto começa em 30 e cresce com o desenvolvimento até 150. A casa é a cama do Minecraft: conta a cabeceira, não o pé. Sem cama livre, ninguém nasce e o jogador ouve “construa mais casas”. A vila gerada no começo já traz camas para quem está lá. O bebê nasce no tick seguinte de um bloco do Veio na mesma dimensão.

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

- Feito: `python tools/check_project.py`, `python tools/check_rust.py --preview` (51 testes na primeira entrega, 61 na 0.10.0, 67 na 0.11.0, 78 na 0.12.0, 90 na 0.13.0, 93 na 0.14.0, 134 na 0.16.0, 135 na 0.16.1, 138 na 0.17.0, 149 na 0.18.0, 160 na 0.19.0, 162 na 0.20.0, 164 na 0.21.0, 166 na 0.22.0, 169 na 0.23.0, 172 na 0.24.0 e 176 na 0.25.0 (com a logística do Codex e os lotes 3 a 7 do Grok); prévia dos painéis novos).
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
