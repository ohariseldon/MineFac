"""Player-facing text for the server-observed workshop tutorial."""
def add_labels(label):
    goals=[
      ('1/16 | Ponha o gerador no terreno e abasteça com carvão', '1/16 | Put the generator on the plot and fuel it with coal'),
      ('2/16 | Leve energia até a prensa', '2/16 | Supply power to the press'),
      ('3/16 | Produza chapas de cobre: %s/4', '3/16 | Produce copper plates: %s/4'),
      ('4/16 | Produza uma engrenagem: %s/1', '4/16 | Produce a gear: %s/1'),
      ('5/16 | Extraia cobre com a máquina: %s/1', '5/16 | Extract copper with a machine: %s/1'),
      ('6/16 | Abasteça a prensa automaticamente: %s/2', '6/16 | Automatically feed the press: %s/2'),
      ('7/16 | Guarde chapas automaticamente: %s/4', '7/16 | Automatically store plates: %s/4'),
      ('8/16 | Central: entregue 24 chapas e 8 engrenagens', '8/16 | Hub: deliver 24 plates and 8 gears'),
      ('9/16 | Coloque o Quadro da fábrica perto das máquinas', '9/16 | Place the Factory board near the machines'),
      ('10/16 | Quadro: entregue chapas de cobre %s/16', '10/16 | Board: deliver copper plates %s/16'),
      ('11/16 | Dobre o minério: coloque outra extratora %s/1', '11/16 | Double the ore: place another extractor %s/1'),
      ('12/16 | Coloque um divisor na esteira', '12/16 | Place a splitter on the belt'),
      ('13/16 | Postes: leve energia longe, %s/2 postes', '13/16 | Poles: carry power far, %s/2 poles'),
      ('14/16 | Pedido do dia no Quadro: %s entregues', '14/16 | Daily order at the board: %s delivered'),
      ('15/16 | Coma em casa: tigela na mão, clique no Lar', '15/16 | Eat at home: bowl in hand, click the Home'),
      ('16/16 | Abra o centro de controle da sua casa', '16/16 | Open the control center in your house'),
    ]
    # Cada etapa: título, por quê, passos numerados (DO_LINES em onboarding.rs) e o kit.
    steps=[
      (('Etapa 1 de 16 · Energia', 'Step 1 of 16 · Power'),
       ('Toda máquina do Veio precisa de energia. O gerador queima carvão e manda energia pelos cabos.', 'Every Veio machine needs power. The generator burns coal and sends power through cables.'),
       [('1. O prefeito montou o seu gerador. Para fazer outros, a receita está no livro de receitas (verde).', '1. The mayor built your generator. To make more, the recipe is in the recipe book (green).'),
        ('2. Coloque o gerador no terreno da oficina, o que você planificou.', '2. Place the generator on the workshop plot you flattened.'),
        ('3. Abra o gerador com a mão vazia e ponha carvão no espaço de combustível.', '3. Open the generator with an empty hand and put coal in its fuel slot.')]),
      (('Etapa 2 de 16 · Ligar uma máquina', 'Step 2 of 16 · Power a machine'),
       ('A prensa transforma cobre bruto em chapas, mas só trabalha com energia chegando.', 'The press turns raw copper into plates, but only works with power arriving.'),
       [('1. A prensa e os cabos vêm do Seu Bento, o ferreiro, em troca de um favor (siga a barra). Depois você pode fabricar mais.', '1. The press and the cables come from Seu Bento, the smith, in exchange for a favour (follow the bar). Later you can craft more.'),
        ('2. Coloque a prensa e ligue-a ao gerador com cabos encostados: gerador > cabo > cabo > prensa.', '2. Place the press and connect it with touching cables: generator > cable > cable > press.'),
        ('3. Veja as marcas da prensa: faixas VERDES recebem itens; a moldura VERMELHA (atrás) solta a produção.', '3. Look at the press marks: GREEN strips take items; the RED frame (back) releases products.'),
        ('4. A etapa avança quando a prensa receber energia. Abra-a para ver a barra de energia.', '4. The step advances when the press receives power. Open it to see the power bar.')]),
      (('Etapa 3 de 16 · Produzir à mão', 'Step 3 of 16 · Produce by hand'),
       ('Antes de automatizar, veja como uma máquina trabalha: entrada, progresso e saída.', 'Before automating, see how a machine works: input, progress and output.'),
       [('1. Minere cobre bruto: minério de cobre com picareta de pedra ou melhor.', '1. Mine raw copper: copper ore with a stone pickaxe or better.'),
        ('2. Abra a prensa e ponha o cobre BRUTO no espaço da esquerda (entrada).', '2. Open the press and put RAW copper in the left slot (input).'),
        ('3. Espere a seta encher e retire as chapas da direita (saída).', '3. Wait for the arrow to fill and take the plates from the right (output).'),
        ('4. Faça 4 chapas novas. Se a máquina parar, passe o mouse no estado do painel: ele diz o motivo.', '4. Make 4 new plates. If the machine stops, hover over the panel status: it tells you why.')]),
      (('Etapa 4 de 16 · Uma cadeia de produção', 'Step 4 of 16 · A production chain'),
       ('Fábricas encadeiam máquinas: a chapa da prensa vira engrenagem no montador.', 'Factories chain machines: the press plate becomes a gear in the assembler.'),
       [('1. Fabrique o montador (ficha acima) com 2 das chapas que você fez.', '1. Craft the assembler (card above) with 2 of your plates.'),
        ('2. Coloque-o, ligue energia e ponha 2 chapas na entrada.', '2. Place it, power it and put 2 plates in its input.'),
        ('3. Retire a engrenagem pronta da saída.', '3. Take the finished gear from the output.')]),
      (('Etapa 5 de 16 · Extração automática', 'Step 5 of 16 · Automatic extraction'),
       ('Minerar à mão não escala. A extratora tira minério da jazida sozinha, sem acabar.', 'Mining by hand does not scale. The extractor mines the deposit on its own, forever.'),
       [('1. Fabrique a extratora (ficha acima).', '1. Craft the extractor (card above).'),
        ('2. Coloque-a EM CIMA de minério de cobre, sem quebrá-lo. Veio maior embaixo = mais rápido.', '2. Place it ON TOP of copper ore, without mining it. Bigger vein below = faster.'),
        ('3. Leve energia até ela. O cobre aparece na saída (moldura vermelha).', '3. Bring power to it. Copper appears at the output (red frame).')]),
      (('Etapa 6 de 16 · Esteiras', 'Step 6 of 16 · Belts'),
       ('Agora a extratora alimenta a prensa: o cobre vai sozinho, sem você carregar.', 'Now the extractor feeds the press: copper travels on its own.'),
       [('1. Fabrique esteiras (ficha acima: cada receita rende 16).', '1. Craft belts (card above: each recipe makes 16).'),
        ('2. A primeira encosta na moldura VERMELHA da extratora. A esteira anda para LONGE de você.', '2. The first touches the extractor RED frame. A belt runs AWAY from you.'),
        ('3. A última entra numa face VERDE da prensa (frente ou lado; nunca na vermelha).', '3. The last one enters a GREEN face of the press (front or side; never the red one).'),
        ('4. Entregue 2 cobres pela esteira e deixe a prensa fazer 1 chapa com eles.', '4. Deliver 2 copper by belt and let the press make 1 plate from them.')]),
      (('Etapa 7 de 16 · Saída e estoque', 'Step 7 of 16 · Output and storage'),
       ('Uma linha completa também leva a produção embora. Senão a saída enche e a máquina para.', 'A complete line also carries products away. Otherwise the output fills and the machine stops.'),
       [('1. Fabrique um baú e coloque-o perto da prensa.', '1. Craft a chest and place it near the press.'),
        ('2. Coloque esteiras saindo da moldura VERMELHA da prensa até o baú: ela despeja nele sozinha.', '2. Run belts from the press RED frame into the chest: it unloads by itself.'),
        ('3. Guarde 4 chapas feitas pela linha. Braços, funis e carregadores também valem.', '3. Store 4 plates made by the line. Arms, hoppers and haulers also count.')]),
      (('Etapa 8 de 16 · A primeira encomenda', 'Step 8 of 16 · The first order'),
       ('A central recebe as encomendas da vila e paga em materiais: é a meta da sua fábrica.', 'The hub takes village orders and pays in materials: it is your factory goal.'),
       [('1. Faça uma engrenagem no montador e fabrique a central (ficha acima).', '1. Make a gear in the assembler and craft the hub (card above).'),
        ('2. Junte 24 chapas e 8 engrenagens (2 chapas cada).', '2. Gather 24 plates and 8 gears (2 plates each).'),
        ('3. Na central: chapas na entrada 1, engrenagens na entrada 2; deixe as saídas livres.', '3. In the hub: plates in input 1, gears in input 2; keep outputs clear.'),
        ('4. Recompensa: 64 esteiras e 32 cabos. Dica: ligue a esteira da prensa à central.', '4. Reward: 64 belts and 32 cables. Tip: run the press belt into the hub.')]),
      # Os sistemas da vila: o que faz a fábrica crescer depois da primeira linha.
      (('Etapa 9 de 16 · O Quadro da fábrica', 'Step 9 of 16 · The Factory board'),
       ('O Quadro é o plano da vila: diz a fase, o pedido do dia e onde a sua fábrica travou.', 'The board is the village plan: it shows the phase, the daily order and where your factory got stuck.'),
       [('1. Fabrique o Quadro da fábrica (ficha acima).', '1. Craft the Factory board (card above).'),
        ('2. Coloque-o perto das máquinas: ele enxerga tudo a até 96 blocos.', '2. Place it near the machines: it sees everything within 96 blocks.'),
        ('3. Abra o Quadro: o painel conta máquinas trabalhando, sem energia, travadas e sem montar.', '3. Open the board: the panel counts machines working, unpowered, blocked and not set up.')]),
      (('Etapa 10 de 16 · Uma fase do plano', 'Step 10 of 16 · A phase of the plan'),
       ('Cada fase do Quadro pede uma entrega maior e libera máquinas novas. É assim que a vila cresce.', 'Each board phase asks for a bigger delivery and unlocks new machines. That is how the village grows.'),
       [('1. A primeira fase pede 16 chapas de cobre no espaço de entrada do Quadro.', '1. The first phase asks for 16 copper plates in the board input slot.'),
        ('2. Melhor ainda: leve uma esteira da prensa até o Quadro. Ele só aceita o que a fase pede.', '2. Even better: run a belt from the press into the board. It only accepts what the phase asks for.'),
        ('3. Recompensa: 32 esteiras na saída do Quadro, e o divisor, o montador e a central liberados.', '3. Reward: 32 belts at the board output, and the splitter, assembler and hub unlocked.')]),
      (('Etapa 11 de 16 · Dobrar o minério', 'Step 11 of 16 · Double the ore'),
       ('A prensa faz mais do que uma extratora comum consegue tirar. Quando falta entrada, o gargalo é o minério.', 'The press makes more than one basic extractor can mine. When input runs short, the bottleneck is the ore.'),
       [('1. Fabrique outra extratora (ficha acima).', '1. Craft another extractor (card above).'),
        ('2. Coloque-a em cima de outro minério de cobre e leve energia até ela.', '2. Place it on top of other copper ore and bring power to it.'),
        ('3. Ligue a saída dela com esteira na mesma linha que vai para a prensa.', '3. Run its output by belt into the same line that goes to the press.')]),
      (('Etapa 12 de 16 · O divisor', 'Step 12 of 16 · The splitter'),
       ('Com minério de sobra, uma prensa só vira o gargalo. O divisor reparte uma esteira em duas.', 'With ore to spare, one press becomes the bottleneck. The splitter shares one belt between two.'),
       [('1. Fabrique o divisor (ficha acima: engrenagem, 2 esteiras e redstone rendem 2).', '1. Craft the splitter (card above: gear, 2 belts and redstone make 2).'),
        ('2. Ponha o divisor no meio da esteira do cobre: ele manda metade para cada lado.', '2. Put the splitter in the middle of the copper belt: it sends half to each side.'),
        ('3. Cada saída leva a uma prensa. Agachado com a chave, ele dá prioridade a um lado.', '3. Each output goes to a press. Sneaking with the wrench, it gives one side priority.')]),
      (('Etapa 13 de 16 · Energia longe', 'Step 13 of 16 · Power far away'),
       ('Cabo serve para perto. Para levar energia até outra parte do terreno, use postes: o fio vai pelo ar.', 'Cables are for short runs. To carry power to another part of the plot, use poles: the wire goes through the air.'),
       [('1. Fabrique postes de madeira (ficha acima: cada receita rende 2).', '1. Craft wooden poles (card above: each recipe makes 2).'),
        ('2. Ponha o primeiro a até 2 blocos do gerador: ele puxa a energia de lá.', '2. Put the first within 2 blocks of the generator: it pulls power from there.'),
        ('3. Ponha o segundo a até 8 blocos do primeiro: o fio aparece sozinho. Máquinas a 2 blocos dele recebem energia.', '3. Put the second within 8 blocks of the first: the wire appears by itself. Machines within 2 blocks of it get power.')]),
      (('Etapa 14 de 16 · O pedido do dia', 'Step 14 of 16 · The daily order'),
       ('Todo dia a vila faz um pedido no Quadro e paga em esmeraldas. É o dinheiro para contratar gente.', 'Every day the village places an order at the board and pays in emeralds. It is the money to hire people.'),
       [('1. Abra o Quadro: o painel mostra o pedido de hoje (o que e quanto).', '1. Open the board: the panel shows today order (what and how much).'),
        ('2. Entregue no espaço de entrada do Quadro, de preferência por esteira, junto com a fase.', '2. Deliver it at the board input slot, by belt if you can, alongside the phase.'),
        ('3. Completo o pedido, as esmeraldas aparecem na saída do Quadro. Guarde: o prefeito vai pedir.', '3. Once the order is complete, the emeralds appear at the board output. Keep them: the mayor will ask.')]),
      (('Etapa 15 de 16 · Em casa', 'Step 15 of 16 · At home'),
       ('O Lar, dentro da sua casa, é onde você descansa. Comer em casa dá mais força para o trabalho.', 'The Home block, inside your house, is where you rest. Eating at home gives you more strength for work.'),
       [('1. No Lar: lenha ou carvão na lareira (1º espaço) e comida na despensa (espaços 2 a 4).', '1. At the Home: wood or coal in the fireplace (first slot) and food in the pantry (slots 2 to 4).'),
        ('2. Com uma tigela na mão, clique no Lar: refeição em casa. Com a lareira acesa, ganha efeitos.', '2. Holding a bowl, click the Home: a home meal. With the fire lit, you get effects.'),
        ('3. Pratos variados e uma casa confortável (cama, luz, janela) deixam os efeitos mais fortes.', '3. Varied dishes and a comfortable house (bed, light, window) make the effects stronger.')]),
      (('Etapa 16 de 16 · A vila inteira', 'Step 16 of 16 · The whole village'),
       ('O centro de controle, na sua casa, mostra a vila de uma vez: gente, comida, dinheiro e obras.', 'The control center, in your house, shows the whole village at once: people, food, money and works.'),
       [('1. Vá até a sua casa e abra o centro de controle (o painel ao lado da cama).', '1. Go to your house and open the control center (the panel beside the bed).'),
        ('2. Ali ficam a comida da vila, as metas, o caixa da empresa e as obras.', '2. It holds the village food, the goals, the company cash and the works.'),
        ('3. A wiki do Veio (tecla J) explica cada sistema quando você quiser saber mais.', '3. The Veio wiki (J key) explains each system whenever you want to know more.')]),
      (('Tutorial concluído!', 'Tutorial complete!'),
       ('Você aprendeu energia, máquinas, esteiras, o Quadro, gargalos, o divisor, postes, o pedido do dia e a casa.', 'You learned power, machines, belts, the board, bottlenecks, the splitter, poles, the daily order and home.'),
       [('Próximo: volte ao prefeito, na praça da vila. Ele conta o que vem a seguir.', 'Next: go back to the mayor, in the village square. He tells you what comes next.'),
        ('Ampliar: uma extratora comum alimenta uma prensa. Mais máquinas pedem mais energia.', 'Expanding: one basic extractor feeds one press. More machines need more power.'),
        ('Consulta: livro de receitas (verde), wiki (J), guia da fábrica e guia da vila. /veiotutorial reiniciar refaz.', 'Reference: recipe book (green), wiki (J), factory guide and village guide. /veiotutorial reiniciar restarts.')]),
    ]
    # A voz do prefeito em cada etapa: por que isso importa para a vila.
    stories=[
      ('A primeira chama da fábrica! Guarde carvão sempre: gerador apagado é fábrica parada.', 'The factory first flame! Always keep coal: a cold generator is a stopped factory.'),
      ('O ferreiro da vila faz chapa no martelo, uma por dia. A prensa faz uma a cada segundo e meio.', 'The village smith hammers one plate a day. The press makes one every second and a half.'),
      ('Veja a máquina trabalhar uma vez com as suas mãos. Depois você nunca mais vai querer fazer isso.', 'Watch the machine work once with your own hands. After that you will never want to do it again.'),
      ('Uma máquina sozinha é ferramenta. Duas encadeadas já são uma fábrica.', 'One machine alone is a tool. Two chained together are already a factory.'),
      ('Minério debaixo da terra não acaba: a extratora tira dele para sempre, se tiver energia.', 'Ore under the ground never runs out: the extractor mines it forever, as long as it has power.'),
      ('Aqui começa a mágica: o cobre anda sozinho até a prensa. Você só olha.', 'This is where the magic starts: copper walks to the press on its own. You just watch.'),
      ('Uma máquina com a saída cheia para. Sempre pergunte: para onde vai o que ela faz?', 'A machine with a full output stops. Always ask: where does what it makes go?'),
      ('A vila precisa de chapas e engrenagens para as primeiras obras. É a sua primeira encomenda!', 'The village needs plates and gears for its first works. It is your first order!'),
      ('Agora que a linha anda, vamos fazer a vila crescer. O Quadro é o plano que a gente segue junto.', 'Now that the line runs, let us grow the village. The board is the plan we follow together.'),
      ('Cada fase do plano pede mais do que a anterior. Quando a entrega demorar, olhe o Quadro: ele mostra o gargalo.', 'Each phase asks for more than the last. When a delivery takes long, look at the board: it shows the bottleneck.'),
      ('Fábrica boa não é a que trabalha mais, é a que não para. Descubra o que falta e dobre só essa parte.', 'A good factory is not the one that works hardest, it is the one that never stops. Find what is missing and double only that part.'),
      ('Duas prensas com um minério só, ou duas extratoras com uma prensa só: sempre tem um lado esperando. O divisor equilibra.', 'Two presses with one ore, or two extractors with one press: one side always waits. The splitter balances it.'),
      ('Um dia a sua fábrica vai ser grande demais para cabos. Os postes levam energia de um lado ao outro da vila.', 'One day your factory will be too big for cables. Poles carry power from one side of the village to the other.'),
      ('O pedido do dia é como a vila paga a fábrica. Com essas esmeraldas, a gente contrata gente para trabalhar com você.', 'The daily order is how the village pays the factory. With those emeralds, we hire people to work with you.'),
      ('E você também precisa comer! Uma refeição quente em casa e você trabalha o dobro.', 'And you need to eat too! A hot meal at home and you work twice as hard.'),
      ('Daqui de casa você vê a vila toda. É o seu escritório de engenheiro.', 'From home you see the whole village. It is your engineer office.'),
      ('Você aprendeu o que eu sabia ensinar. Venha me ver na praça.', 'You learned all I could teach. Come see me in the square.'),
    ]
    for step,(pt,en) in enumerate(stories): label(f'veio.onboarding.story.{step}',pt,en)
    # Os 4 atos da fábrica (4 etapas cada): nome, subtítulo e a abertura no chat.
    acts=[
      (('Ato I · A primeira chama','Act I · The first flame'),('Energia e a primeira máquina','Power and the first machine'),
       ('─── Ato I · A primeira chama ─── energia, a prensa e a primeira peça.','─── Act I · The first flame ─── power, the press and the first part.')),
      (('Ato II · A linha que anda sozinha','Act II · The line that runs itself'),('Extrair, transportar, guardar, entregar','Mine, carry, store, deliver'),
       ('─── Ato II · A linha que anda sozinha ─── o minério vai sozinho até a peça pronta.','─── Act II · The line that runs itself ─── ore travels on its own to the finished part.')),
      (('Ato III · O plano da vila','Act III · The village plan'),('O Quadro e os gargalos','The board and the bottlenecks'),
       ('─── Ato III · O plano da vila ─── o Quadro mostra o que travou; você dobra só essa parte.','─── Act III · The village plan ─── the board shows what got stuck; you double only that part.')),
      (('Ato IV · A vila conta com você','Act IV · The village counts on you'),('Energia longe, o pedido do dia e a casa','Power far away, the daily order and home'),
       ('─── Ato IV · A vila conta com você ─── a fábrica começa a pagar a vila.','─── Act IV · The village counts on you ─── the factory starts paying the village.')),
    ]
    for n,(title,sub,line) in enumerate(acts):
        label(f'veio.onboarding.act.{n}',*title); label(f'veio.onboarding.act.{n}.sub',*sub); label(f'veio.onboarding.act.{n}.line',*line)
    label('veio.onboarding.act.end','A fábrica está de pé!','The factory stands!')
    label('veio.onboarding.act.end.sub','Volte ao prefeito na praça','Go back to the mayor in the square')
    label('veio.onboarding.far','%s  ·  %s %s blocos','%s  ·  %s %s blocks')
    # Dica do prefeito quando o jogador empaca (3 min na mesma etapa) e ao clicar nele.
    label('veio.onboarding.hint.head','Faz um tempo que você está nessa etapa. Uma dica:','You have been on this step for a while. A tip:')
    hints=[
      ('Abra o gerador com a mão VAZIA (com um item na mão você coloca o bloco). Sem carvão? Carvão vegetal serve: tora na fornalha.',
       'Open the generator with an EMPTY hand (holding an item places the block). No coal? Charcoal works: a log in the furnace.'),
      ('Os cabos precisam se ENCOSTAR, sem buraco: gerador, cabo, cabo, prensa. E o gerador precisa estar queimando carvão.',
       'Cables must TOUCH, with no gap: generator, cable, cable, press. And the generator must be burning coal.'),
      ('Cobre BRUTO, não lingote: o minério quebrado com picareta de pedra. Tem nas encostas e logo abaixo da grama nas cavernas.',
       'RAW copper, not ingots: the ore broken with a stone pickaxe. It is on hillsides and just below the grass in caves.'),
      ('O montador precisa de energia também: um cabo encostado nele. Duas chapas na entrada fazem uma engrenagem.',
       'The assembler needs power too: a cable touching it. Two plates in the input make one gear.'),
      ('A extratora vai EM CIMA do minério, sem quebrá-lo. Se a barra dela não enche, falta energia ou o bloco de baixo não é minério.',
       'The extractor goes ON TOP of the ore, without mining it. If its bar does not fill, it lacks power or the block below is not ore.'),
      ('A esteira anda para longe de você. Coloque-a olhando da extratora para a prensa, e a última numa face VERDE da prensa.',
       'A belt runs away from you. Place it looking from the extractor towards the press, and the last one on a GREEN face of the press.'),
      ('A esteira de saída começa encostada na moldura VERMELHA (atrás da prensa) e termina dentro do baú.',
       'The output belt starts against the RED frame (behind the press) and ends inside the chest.'),
      ('Para as 8 engrenagens, ponha chapas no montador e deixe trabalhar. Chapas na entrada 1 da central, engrenagens na 2.',
       'For the 8 gears, put plates in the assembler and let it work. Plates in hub input 1, gears in input 2.'),
      ('O Quadro já está liberado no livro de receitas (verde): baú, 2 lingotes de cobre e um graveto.',
       'The board is already unlocked in the recipe book (green): chest, 2 copper ingots and a stick.'),
      ('O Quadro só aceita chapa de COBRE na fase 1. Veja o número no painel dele subir; uma esteira da prensa até ele faz isso sozinha.',
       'The board only accepts COPPER plates in phase 1. Watch the number on its panel go up; a belt from the press does it by itself.'),
      ('Qualquer extratora nova conta, até em outro minério. Ponha energia nela e uma esteira até a linha.',
       'Any new extractor counts, even on another ore. Power it and run a belt into the line.'),
      ('O divisor está no livro depois da fase 1 do Quadro. Precisa de engrenagem, 2 esteiras e redstone (minério vermelho nas cavernas).',
       'The splitter is in the book after board phase 1. It needs a gear, 2 belts and redstone (red ore in caves).'),
      ('O primeiro poste fica a até 2 blocos do gerador; o segundo, a até 8 do primeiro. Se o fio não aparece, aproxime os postes.',
       'The first pole stands within 2 blocks of the generator; the second within 8 of the first. If no wire appears, move them closer.'),
      ('O pedido muda todo dia e é da vila toda: abra o Quadro para ver o que é. Às vezes é chapa de ferro: ferro bruto na prensa.',
       'The order changes every day and belongs to the whole village: open the board to see it. Sometimes it is iron plates: raw iron in the press.'),
      ('A tigela é de madeira: 3 tábuas em V. O Lar fica dentro da sua casa, ao norte da praça; ponha comida na despensa dele.',
       'The bowl is wooden: 3 planks in a V. The Home block is inside your house, north of the square; put food in its pantry.'),
      ('O centro de controle é o painel da sua casa. Clique nele com a mão vazia.',
       'The control center is the panel in your house. Click it with an empty hand.'),
    ]
    for step,(pt,en) in enumerate(hints): label(f'veio.onboarding.hint.{step}',pt,en)
    village_hints=[
      ('Esmeralda: troque com os aldeões da vila. O flecheiro compra gravetos; o fazendeiro, trigo, batata ou cenoura.',
       'Emerald: trade with the village folk. The fletcher buys sticks; the farmer buys wheat, potatoes or carrots.'),
      ('Ninguém contratado? O aldeão precisa estar a até 8 blocos e sem posto. Se não houver, o posto espera 30 s: clique no aldeão com a esmeralda.',
       'Nobody hired? The villager must be within 8 blocks and without a post. If none, the post waits 30 s: click the villager with the emerald.'),
      ('O marco precisa ficar dentro das faíscas verdes em volta do posto, na mesma altura (um bloco a mais ou a menos).',
       'The marker must stand inside the green sparkles around the post, at the same height (one block up or down at most).'),
      ('Trigo cru quase não alimenta: ponha pão no posto. E a plantação precisa estar madura para ele colher.',
       'Raw wheat barely feeds: put bread in the post. And the crops must be ripe to be harvested.'),
      ('O carregador só atende máquinas a até 6 blocos do posto. Ponha insumos à esquerda para ele levar às máquinas.',
       'The hauler only serves machines within 6 blocks of the post. Put supplies on the left for him to bring to the machines.'),
      ('A mesa do capataz precisa de um aldeão livre por perto e uma esmeralda na mão para contratar.',
       'The foreman desk needs a free villager nearby and an emerald in hand to hire.'),
      ('A casa só ganha moradores se houver aldeões a até 10 blocos dela. Ponha comida na despensa.',
       'The house only gets residents if there are villagers within 10 blocks. Put food in its pantry.'),
      ('A prefeitura precisa de um aldeão livre e uma esmeralda, como os postos.',
       'The town hall needs a free villager and an emerald, like the posts.'),
    ]
    for step,(pt,en) in enumerate(village_hints): label(f'veio.village.hint.{step}',pt,en)
    # Diário do Prefeito Otávio (livro dado com o gerador): a história e o caminho.
    diary=[
      ('Diário de Otávio\nPrefeito da vila\n\n\nPara o engenheiro que chegou do caminho de terra.',
       'Otávio\'s Diary\nMayor of the village\n\n\nFor the engineer who came down the dirt road.'),
      ('O veio\n\nMeu avô abriu a primeira mina destes morros. Dizia que lá embaixo corre um veio de cobre e ferro que não acaba nunca.\n\nA picareta não deu conta. A mina fechou, e a vila voltou a plantar.',
       'The vein\n\nMy grandfather opened the first mine in these hills. He said a vein of copper and iron runs down there and never ends.\n\nThe pickaxe could not keep up. The mine closed, and the village went back to farming.'),
      ('O trato\n\nA vila dá o chão, os trabalhadores e as encomendas. Você monta a fábrica, e a fábrica paga a vila.\n\nMáquina não cansa. Uma extratora segue o veio para sempre, se tiver energia.',
       'The deal\n\nThe village gives the land, the workers and the orders. You build the factory, and the factory pays the village.\n\nMachines do not tire. An extractor follows the vein forever, as long as it has power.'),
      ('O caminho\n\nAto I · A primeira chama\nAto II · A linha que anda sozinha\nAto III · O plano da vila\nAto IV · A vila conta com você\n\nDepois: a vila operária e a cidade.',
       'The road\n\nAct I · The first flame\nAct II · The line that runs itself\nAct III · The village plan\nAct IV · The village counts on you\n\nThen: the working village and the town.'),
      ('Conselhos\n\nMáquina parada tem motivo: passe o mouse no estado.\n\nSaída cheia para a linha.\n\nEmpacou? Fale comigo.\n\nA tecla J abre a wiki.',
       'Advice\n\nA stopped machine has a reason: hover over its status.\n\nA full output stops the line.\n\nStuck? Talk to me.\n\nThe J key opens the wiki.'),
    ]
    for n,(pt,en) in enumerate(diary): label(f'veio.onboarding.diary.{n}',pt,en)
    label('veio.onboarding.diary.title','Diário do Prefeito Otávio','Mayor Otávio\'s Diary')
    # Festas do capítulo da vila (como os eventos do Stardew Valley): data marcada, contagem, festa.
    festivals={
        'harvest':('Festa da Colheita','Harvest Festival',
            ('Daqui a %s dias é a Festa da Colheita! A vila inteira vai à praça ver o primeiro campo da fazenda.',
             'The Harvest Festival is in %s days! The whole village will come to the square to see the farm\'s first field.'),
            ('Até lá quero o posto do lavrador, alguém contratado, o campo marcado e a primeira colheita. Não me deixe passar vergonha!',
             'By then I want the farmer post, someone hired, the field marked and the first harvest. Don\'t let me down!'),
            ('A vila comemorou! Pão quente para todos, e um saco para os seus postos.',
             'The village celebrated! Warm bread for everyone, and a sack for your posts.'),
            ('Ouviu os sinos? É isso que o veio faz quando a vila trabalha junta.',
             'Hear the bells? That is what the vein does when the village works together.'),
            ('Pão para os postos e experiência','Bread for the posts and experience'),
            'O campo não ficou pronto e a festa foi fraca. Vamos remarcar: mais 2 dias.',
            'The field wasn\'t ready and the festival fell flat. Let\'s move it: 2 more days.',
            'Remarquei a Festa da Colheita para daqui a %s dias. Desta vez vai!',
            'I moved the Harvest Festival to %s days from now. This time it will work!'),
        'governor':('Visita do Governador','Governor\'s Visit',
            ('O governador vem nos visitar em %s dias! Ele decide se a vila merece estrada e comércio.',
             'The governor visits in %s days! He decides whether the village deserves a road and trade.'),
            ('Ele vai querer ver o carregador, o capataz, uma casa nova e a prefeitura de pé. Mãos à obra!',
             'He will want to see the hauler, the foreman, a new house and the town hall standing. Get to work!'),
            ('O governador saiu impressionado! Mandou um bolo para o Lar: hoje tem banquete.',
             'The governor left impressed! He sent a cake for the Hearth: tonight there is a feast.'),
            ('Ele disse que vai falar da nossa vila na capital. Isso é obra sua.',
             'He said he will talk about our village in the capital. That is your doing.'),
            ('Um bolo para o banquete e experiência','A cake for the feast and experience'),
            'O governador viu a vila pela metade e torceu o nariz. Ele volta em 2 dias.',
            'The governor saw a half-built village and frowned. He comes back in 2 days.',
            'O governador volta daqui a %s dias. Vamos terminar o que falta!',
            'The governor returns in %s days. Let\'s finish what is left!'),
    }
    for key,(name_pt,name_en,a1,a2,g1,g2,sub,weak_pt,weak_en,retry_pt,retry_en) in festivals.items():
        label(f'veio.festival.{key}.name',name_pt,name_en)
        label(f'veio.festival.{key}.when','em %s dias','in %s days')
        for n,(pt,en) in enumerate([a1,a2],1): label(f'veio.festival.{key}.announce.{n}',pt,en)
        for n,(pt,en) in enumerate([g1,g2],1): label(f'veio.festival.{key}.good.{n}',pt,en)
        label(f'veio.festival.{key}.good_sub',*sub)
        label(f'veio.festival.{key}.weak',weak_pt,weak_en)
        label(f'veio.festival.{key}.retry',retry_pt,retry_en)
    label('veio.festival.countdown','%s em %s dia(s)','%s in %s day(s)')
    label('veio.festival.reminder','Não esqueça: %s.','Don\'t forget: %s.')
    # O favor do primeiro ato (favors.rs): a prensa do Seu Bento em troca do campo da Dona Cida.
    favor=[
      ('smith.name','Seu Bento, o ferreiro','Seu Bento, the smith'),
      ('farmer.name','Dona Cida, a lavradora','Dona Cida, the farmer'),
      ('start.1','O gerador está de pé, mas sozinho ele não faz nada. Falta uma máquina para ligar nele: uma prensa.',
       'The generator is up, but alone it does nothing. It needs a machine to power: a press.'),
      ('start.2','O Seu Bento, o ferreiro, tem uma prensa velha sobrando na oficina dele, a leste da praça. Vá falar com ele.',
       'Seu Bento, the smith, has an old spare press in his workshop, east of the square. Go and talk to him.'),
      ('mayor.1','A prensa está com o Seu Bento, o ferreiro, a leste da praça. Siga a seta.','The press is with Seu Bento, the smith, east of the square. Follow the arrow.'),
      ('mayor.2','O Seu Bento pediu um favor, não foi? Ele é assim. Fale com a Dona Cida, a oeste da praça.','Seu Bento asked a favour, didn\'t he? That\'s him. Talk to Dona Cida, west of the square.'),
      ('mayor.3','Um campo de trigo ao lado da Dona Cida: isso ajuda a vila inteira. Use a farinha de osso para crescer rápido.','A wheat field next to Dona Cida: that helps the whole village. Use the bone meal to grow it fast.'),
      ('mayor.4','Colha o trigo e leve para a Dona Cida. Trigo maduro é o dourado.','Harvest the wheat and take it to Dona Cida. Ripe wheat is the golden one.'),
      ('mayor.5','A Dona Cida já está sorrindo. Agora volte ao Seu Bento: palavra dele vale.','Dona Cida is smiling already. Now go back to Seu Bento: his word is good.'),
      ('goal.1','Fale com o Seu Bento, o ferreiro','Talk to Seu Bento, the smith'),
      ('goal.2','Fale com a Dona Cida, a lavradora','Talk to Dona Cida, the farmer'),
      ('goal.3','Plante trigo ao lado da Dona Cida: %s/12','Plant wheat next to Dona Cida: %s/12'),
      ('goal.4','Colha 12 trigos e entregue à Dona Cida','Harvest 12 wheat and give it to Dona Cida'),
      ('goal.5','Volte ao Seu Bento para pegar a prensa','Go back to Seu Bento for the press'),
      ('bar','Favor · %s','Favour · %s'),
      ('smith.ask.1','Prensa? Tenho, sim. Estava encostada desde que a mina fechou.','A press? Yes, I have one. It\'s been sitting there since the mine closed.'),
      ('smith.ask.2','Mas olha, estou com a cabeça em outra coisa: o pão aqui está pela hora da morte.','But look, my head is somewhere else: bread here costs a fortune.'),
      ('smith.ask.3','A Dona Cida não dá conta da roça sozinha, então tem pouco pão e ela cobra caro. Meus filhos comem pouco.','Dona Cida can\'t keep up with the fields alone, so there\'s little bread and she charges a lot. My kids eat little.'),
      ('smith.ask.4','Ajuda ela com um campo de trigo e a prensa é sua. Com os cabos, de quebra.','Help her with a wheat field and the press is yours. The cables too, as a bonus.'),
      ('farmer.ask.1','O Bento te mandou? Ai, que bom. Minhas costas não aguentam mais a enxada.','Bento sent you? Oh, good. My back can\'t take the hoe anymore.'),
      ('farmer.ask.2','Lavre a terra aqui do lado de casa e plante %s pés de trigo. Perto da água a terra fica boa.','Till the soil next to my house and plant %s wheat. Near water the soil stays good.'),
      ('farmer.ask.3','Tome sementes, farinha de osso para crescer logo e uma enxada. Depois me traga o trigo colhido.','Take seeds, bone meal so it grows fast and a hoe. Then bring me the harvested wheat.'),
      ('farmer.gift','Você recebeu 16 sementes de trigo, 32 farinhas de osso e uma enxada de pedra.','You got 16 wheat seeds, 32 bone meal and a stone hoe.'),
      ('farmer.plant','Estou contando: %s de %s pés plantados aqui do lado.','I\'m counting: %s of %s planted right here.'),
      ('farmer.planted','Que campo bonito! Agora deixe crescer (farinha de osso ajuda) e me traga %s trigos.','What a lovely field! Now let it grow (bone meal helps) and bring me %s wheat.'),
      ('farmer.more','Ainda faltam %s trigos. O maduro é o dourado, quebre e pegue.','Still %s wheat to go. The ripe one is golden: break it and pick it up.'),
      ('farmer.thanks.1','Olha só quanto trigo! Com esse campo eu consigo fazer pão para a vila toda.','Look at all this wheat! With this field I can bake bread for the whole village.'),
      ('farmer.thanks.2','Antes eu vendia 2 pães por esmeralda. Agora, com mais trigo, vendo 6. Pão barato é família de barriga cheia.','I used to sell 2 bread for an emerald. Now, with more wheat, I sell 6. Cheap bread means full families.'),
      ('farmer.thanks.3','Vá lá contar para o Bento. E volte quando quiser pão: é só clicar em mim.','Go tell Bento. And come back whenever you want bread: just click me.'),
      ('farmer.after','Pão fresquinho, 6 por esmeralda! O campo que você plantou rende todo dia.','Fresh bread, 6 for an emerald! The field you planted yields every day.'),
      ('farmer.wait','Fale com o Bento primeiro, ele está te esperando.','Talk to Bento first, he\'s waiting for you.'),
      ('smith.thanks.1','A Cida veio aqui toda contente. Pão barato na minha mesa: obrigado, de verdade.','Cida came by all happy. Cheap bread on my table: thank you, truly.'),
      ('smith.thanks.2','Aqui está a prensa, e 16 cabos de cobre. Também te ensinei a fazer os dois: estão no seu livro de receitas.','Here is the press, and 16 copper cables. I also taught you to make both: they\'re in your recipe book.'),
      ('smith.thanks.3','Ligue a prensa no gerador com os cabos. E se a vila precisar, ajude de novo: aqui um favor puxa o outro.','Connect the press to the generator with the cables. And if the village needs it, help again: here one favour leads to another.'),
      ('smith.after','A prensa está te servindo? Cuida bem dela, era do meu pai.','Is the press serving you well? Take good care of it, it was my father\'s.'),
      ('smith.wait','Ajude a Dona Cida com o campo e a prensa é sua.','Help Dona Cida with the field and the press is yours.'),
      ('done','Favor feito!','Favour done!'),
      ('done.sub','A prensa e os cabos são seus','The press and the cables are yours'),
    ]
    for key,pt,en in favor: label(f'veio.favor.{key}',pt,en)
    # Aldeões comuns: só um resmungo curto. Quem conversa de verdade é o prefeito.
    label('veio.villager.who','Aldeão','Villager')
    label('veio.villager.says','§7§o%s: %s','§7§o%s: %s')
    barks={
        'mayor':[('Hrm! O prefeito está na praça.','Hrm! The mayor is in the square.'),
                 ('Fala com o prefeito, ele sabe das coisas.','Talk to the mayor, he knows things.'),
                 ('Hmm. Você é o novo do veio?','Hmm. You\'re the new one for the vein?'),
                 ('O prefeito? Ali, o de casaco de pele.','The mayor? Over there, the one in the fur coat.')],
        'gather':[('Hrm. Madeira tem no bosque.','Hrm. There\'s wood in the grove.'),
                  ('Pedra? É só cavar um pouco.','Stone? Just dig a bit.'),
                  ('O prefeito quer tudo de uma vez, hein.','The mayor wants it all at once, huh.'),
                  ('Hmm! Cobre aparece nos morros.','Hmm! Copper shows up in the hills.')],
        'plot':[('Hrm. Terreno torto não serve pra oficina.','Hrm. Crooked ground won\'t do for a workshop.'),
                ('Aplana bem, hein.','Level it well, huh.'),
                ('Uma fábrica! Na nossa vila!','A factory! In our village!'),
                ('Hmm. Onze por onze, foi o que ele disse.','Hmm. Eleven by eleven, that\'s what he said.')],
        'festival':[('A festa é logo! Hrm!','The festival is soon! Hrm!'),
                    ('Já enfeitei a minha porta.','I already decorated my door.'),
                    ('Hmm, tomara que a festa saia bonita.','Hmm, hope the festival turns out nice.'),
                    ('O prefeito está nervoso com a festa.','The mayor is nervous about the festival.')],
        'favor':[('Hrm. A Dona Cida não dá conta da roça.','Hrm. Dona Cida can\'t keep up with the fields.'),
                 ('O pão anda caro, hein.','Bread is pricey lately, huh.'),
                 ('O Bento tem uma prensa velha, sabia?','Bento has an old press, did you know?'),
                 ('Hmm! Um campo novo ia bem.','Hmm! A new field would do nicely.')],
        'factory':[('Hrm. Que barulho é esse da oficina?','Hrm. What\'s that noise from the workshop?'),
                   ('As esteiras andam sozinhas! Hmm!','The belts move on their own! Hmm!'),
                   ('O prefeito diz que o veio não acaba.','The mayor says the vein never ends.'),
                   ('Hmm. Cuidado com os dedos na prensa.','Hmm. Mind your fingers on the press.')],
        'village':[('Hrm! Tem trabalho pra mim?','Hrm! Got work for me?'),
                   ('Um posto e eu trabalho, hein.','Give me a post and I\'ll work, huh.'),
                   ('Hmm. A vila está crescendo.','Hmm. The village is growing.'),
                   ('O prefeito está contente com você.','The mayor is happy with you.')],
        'free':[('Hrm.','Hrm.'),('Hmm! Bom dia.','Hmm! Good day.'),
                ('A vila está bonita, hein.','The village looks nice, huh.'),
                ('Hrm. O veio ainda dá cobre.','Hrm. The vein still gives copper.')],
    }
    for topic,lines in barks.items():
        for n,(pt,en) in enumerate(lines,1): label(f'veio.villager.{topic}.{n}',pt,en)
    # Nomes dos materiais (lista acima da barra de itens e fichas).
    names={
      'generator':('Gerador','Generator'), 'press':('Prensa','Press'), 'cable':('Cabo','Cable'),
      'assembler':('Montador','Assembler'), 'drill':('Extratora','Extractor'), 'belt':('Esteira','Belt'),
      'contract_hub':('Central','Hub'), 'chest':('Baú','Chest'), 'raw_copper':('Cobre bruto','Raw copper'),
      'furnace':('Fornalha','Furnace'), 'iron_ingot':('Lingote de ferro','Iron ingot'),
      'copper_ingot':('Lingote de cobre','Copper ingot'), 'piston':('Pistão','Piston'),
      'smooth_stone':('Pedra lisa','Smooth stone'), 'stick':('Graveto','Stick'),
      'crafting_table':('Bancada','Crafting table'), 'copper_plate':('Chapa de cobre','Copper plate'),
      'gear':('Engrenagem','Gear'), 'planks':('Tábuas','Planks'),
      'factory_board':('Quadro da fábrica','Factory board'), 'pole':('Poste de madeira','Wooden pole'),
      'splitter':('Divisor','Splitter'), 'redstone':('Redstone','Redstone'),
    }
    for key,(pt,en) in names.items(): label(f'veio.onboarding.name.{key}',pt,en)
    # Fichas de receita, geradas das receitas do jogo (nunca ficam desatualizadas).
    import json
    from pathlib import Path
    recipes=Path(__file__).resolve().parent.parent/'content/data/veio/recipe'
    def recipe_line(item):
        data=json.loads((recipes/f'{item}.json').read_text(encoding='utf-8'))
        counts={}
        for ingredient in data['ingredients']:
            key=ingredient.split(':')[-1]; counts[key]=counts.get(key,0)+1
        def part(lang):
            return ' + '.join((f'{n}× ' if n>1 else '')+names[k][lang] for k,n in counts.items())
        out=data['result'].get('count',1)
        return (f"Receita · {names[item][0]} (bancada, em qualquer posição): {part(0)} = {out}× {names[item][0]}",
                f"Recipe · {names[item][1]} (crafting table, any position): {part(1)} = {out}× {names[item][1]}")
    plan={0:['generator'],1:['press','cable'],3:['assembler'],4:['drill'],5:['belt'],7:['contract_hub'],
          8:['factory_board'],10:['drill'],11:['splitter'],12:['pole']}
    for step,items in plan.items():
        for index,item in enumerate(items): label(f'veio.onboarding.recipe.{step}.{index}',*recipe_line(item))
    label('veio.onboarding.recipe.6.0','Receita · Baú (bancada): 8 tábuas em volta, com o meio vazio = 1 baú',
          'Recipe · Chest (crafting table): 8 planks around, center empty = 1 chest')
    sources=[
      ('De onde vem: lingotes = minério de cobre/ferro na fornalha · fornalha = 8 pedregulhos · carvão = minério de carvão.',
       'Where from: ingots = copper/iron ore in a furnace · furnace = 8 cobblestone · coal = coal ore.'),
      ('De onde vem: pistão = 3 tábuas + 4 pedregulhos + ferro + redstone · pedra lisa = pedregulho na fornalha 2 vezes · graveto = 2 tábuas.',
       'Where from: piston = 3 planks + 4 cobblestone + iron + redstone · smooth stone = cobblestone smelted twice · stick = 2 planks.'),
      ('De onde vem: cobre bruto = minério de cobre (encostas e cavernas), com picareta de pedra ou melhor.',
       'Where from: raw copper = copper ore (hillsides and caves), with a stone pickaxe or better.'),
      ('De onde vem: chapa de cobre = a prensa da etapa anterior · bancada = 4 tábuas.',
       'Where from: copper plate = the press from the last step · crafting table = 4 planks.'),
      ('De onde vem: lingotes na fornalha; o pistão você já conhece da prensa.',
       'Where from: ingots from the furnace; you know the piston from the press.'),
      ('De onde vem: lingote de cobre na fornalha · graveto = 2 tábuas.',
       'Where from: copper ingot from the furnace · stick = 2 planks.'),
      ('De onde vem: tábuas = 1 tora rende 4.',
       'Where from: planks = 1 log makes 4.'),
      ('De onde vem: engrenagem = 2 chapas no montador · baú = 8 tábuas.',
       'Where from: gear = 2 plates in the assembler · chest = 8 planks.'),
    ]
    for step,(pt,en) in enumerate(sources): label(f'veio.onboarding.from.{step}',pt,en)
    # Etapas dos sistemas com receita (as outras não têm ficha).
    for step,pt,en in [
      (8,'De onde vem: baú = 8 tábuas · lingote de cobre na fornalha · graveto = 2 tábuas.',
         'Where from: chest = 8 planks · copper ingot from the furnace · stick = 2 planks.'),
      (10,'De onde vem: lingotes na fornalha; o pistão você já fez para a prensa.',
          'Where from: ingots from the furnace; you already made the piston for the press.'),
      (11,'De onde vem: engrenagem = 2 chapas no montador · esteira = a da etapa 6 · redstone = minério de redstone, nas cavernas.',
          'Where from: gear = 2 plates in the assembler · belt = the one from step 6 · redstone = redstone ore, in caves.'),
      (12,'De onde vem: graveto = 2 tábuas · lingote de cobre na fornalha.',
          'Where from: stick = 2 planks · copper ingot from the furnace.')]:
        label(f'veio.onboarding.from.{step}',pt,en)
    label('veio.onboarding.book','Livro de receitas: no inventário, clique no livro verde. A receita da etapa já está lá; clique nela e a grade se monta sozinha.',
          'Recipe book: in your inventory, click the green book. This step\'s recipe is there; click it and the grid fills itself.')
    label('veio.onboarding.make','Fabrique: %s','Craft: %s')
    # Prólogo (onboarding/intro.rs): casa, prefeito e terreno da oficina; e a volta ao prefeito.
    for key,(pt,en) in {'n':('ao norte','north'),'ne':('a nordeste','northeast'),'e':('a leste','east'),
            'se':('a sudeste','southeast'),'s':('ao sul','south'),'sw':('a sudoeste','southwest'),
            'w':('a oeste','west'),'nw':('a noroeste','northwest')}.items():
        label(f'veio.onboarding.dir.{key}',pt,en)
    intro=[
      ('mayor.name','Prefeito Otávio','Mayor Otávio'),
      # Chegada: nasce perto da vila e segue a seta.
      ('intro.welcome','Veio','Veio'),
      ('intro.welcome_sub','Uma vila perto daqui precisa de você','A village nearby needs you'),
      ('intro.village.1','Você chegou a uma terra sem indústria. Há uma vila aqui perto, e ela precisa de alguém que saiba montar máquinas.',
       'You arrived in a land with no industry. There is a village nearby, and it needs someone who can build machines.'),
      ('intro.village.2','Siga a seta acima da barra de itens até a vila.','Follow the arrow above your hotbar to the village.'),
      ('intro.goal.village','Vá para a vila %s  %s blocos','Go to the village %s  %s blocks'),
      ('intro.goal.village_wait','Procurando a vila mais próxima... explore um pouco.','Looking for the nearest village... explore a little.'),
      ('intro.arrived','A vila','The village'),
      ('intro.arrived_sub','Procure o prefeito na praça','Find the mayor in the square'),
      ('intro.mayor.1','O prefeito está na praça, com o nome em cima da cabeça. Clique nele com o botão direito.',
       'The mayor is in the square, with his name above his head. Right-click him.'),
      ('intro.goal.mayor','Fale com o prefeito %s  %s blocos','Talk to the mayor %s  %s blocks'),
      ('intro.goal.mayor_here','Fale com o prefeito: clique nele com o botão direito','Talk to the mayor: right-click him'),
      # A história e o pedido de recursos.
      ('mayor.hello.1','Um forasteiro! Seja bem-vindo. Aqui a gente vive da terra, mas a terra sozinha não basta mais.',
       'A stranger! Welcome. We live off the land here, but the land alone is no longer enough.'),
      ('mayor.hello.2','Embaixo destes morros corre um veio de cobre e ferro que não acaba. Meu avô seguia o veio na picareta; com máquinas, ele vira chapa, engrenagem, trabalho e comida na mesa.',
       'Under these hills runs a vein of copper and iron that never ends. My grandfather followed it with a pickaxe; with machines it becomes plates, gears, jobs and food on the table.'),
      ('mayor.hello.3','Proponho um trato: a vila dá o chão, os trabalhadores e as encomendas. Você monta a fábrica e ela paga a vila.',
       'Here is my deal: the village gives the land, the workers and the orders. You build the factory, and it pays the village.'),
      ('mayor.hello.4','Nossa oficina tem as peças de um gerador, a primeira máquina. Faltam os materiais. Traga para mim:',
       'Our workshop has the parts of a generator, the first machine. We lack the materials. Bring me:'),
      ('mayor.hello.5','(A casa ao norte da praça é sua: a cama ao lado do painel é onde você renasce.)',
       '(The house north of the square is yours: the bed beside the panel is where you respawn.)'),
      ('mayor.need','· %s %s','· %s %s'),
      ('mayor.how.1','Como conseguir: tábuas das árvores, bancada e picareta de madeira; com ela, quebre pedra para ter pedregulho.',
       'How to get them: planks from trees, a crafting table and a wooden pickaxe; with it, break stone to get cobblestone.'),
      ('mayor.how.2','Com pedregulho, faça uma picareta de pedra e uma fornalha. Minério de ferro e de cobre ficam nas encostas e cavernas.',
       'With cobblestone, make a stone pickaxe and a furnace. Iron and copper ore are on hillsides and in caves.'),
      ('mayor.how.3','Derreta o minério na fornalha, com carvão ou madeira, e terá os lingotes. Guarde carvão: o gerador queima carvão.',
       'Smelt the ore in the furnace, with coal or wood, to get ingots. Keep some coal: the generator burns coal.'),
      ('intro.gather_title','Junte os materiais','Gather the materials'),
      ('intro.gather_sub','Para o prefeito montar o gerador','So the mayor can build the generator'),
      ('intro.goal.gather','Para o prefeito:','For the mayor:'),
      ('intro.goal.deliver','Tudo pronto! Leve ao prefeito %s  %s blocos','All set! Take it to the mayor %s  %s blocks'),
      ('mayor.missing','Quase! Ainda faltam: %s.','Almost! Still missing: %s.'),
      # A primeira máquina e o terreno.
      ('intro.machine_title','Primeira máquina!','First machine!'),
      ('intro.machine_sub','O gerador é seu','The generator is yours'),
      ('mayor.machine.1','Pronto, montei o seu gerador com as minhas mãos. A prensa e os cabos, isso eu não tenho: na hora certa eu te digo com quem estão.',
       'Done, I built your generator with my own hands. The press and the cables I don\'t have: when the time comes I\'ll tell you who has them.'),
      ('mayor.machine.2','Mas máquina não fica em qualquer lugar. Máquinas, cabos e esteiras precisam de chão plano.',
       'But machines cannot go just anywhere. Machines, cables and belts need flat ground.'),
      ('mayor.plot','Separei o terreno da sua oficina, %s blocos %s: %s x %s blocos marcados com faíscas verdes. Deixe tudo plano.',
       'I set aside your workshop plot, %s blocks %s: %s x %s blocks marked with green sparkles. Make it all flat.'),
      ('intro.plot_title','Planifique o terreno','Flatten the plot'),
      ('intro.plot_sub','O chão da sua oficina','Your workshop floor'),
      ('intro.plot.1','Terreno da oficina: centro em %s %s %s, %s blocos de lado. O nível é o do meio do terreno.',
       'Workshop plot: centre at %s %s %s, %s blocks wide. The level is the middle height of the plot.'),
      ('intro.plot.2','Chamas pequenas: terra ou bloco a mais, cave (pá ou picareta). Faíscas azuis: buraco ou água, tape com terra ou pedra.',
       'Small flames: extra dirt or blocks, dig them (shovel or pickaxe). Blue sparks: holes or water, fill them with dirt or stone.'),
      ('intro.plot.3','Árvores no terreno também saem. A barra acima dos itens conta o que falta; quando acabar, o prefeito avisa.',
       'Trees on the plot must go too. The bar above your hotbar counts what is left; when you finish, the mayor lets you know.'),
      ('intro.goal.plot_far','Terreno da oficina %s  %s blocos','Workshop plot %s  %s blocks'),
      ('intro.goal.plot','Planifique: faltam %s colunas · cavar %s · aterrar %s','Flatten: %s columns left · dig %s · fill %s'),
      ('mayor.plot_again','O terreno fica a %s blocos %s. Ainda faltam %s colunas para ficar plano: chamas, cave; faíscas azuis, aterre.',
       'The plot is %s blocks %s. Still %s columns to flatten: flames, dig; blue sparks, fill.'),
      ('intro.plot_done','Terreno pronto!','Plot ready!'),
      ('mayor.plot_done.1','Que chão bonito! Aqui vai ser a sua oficina.','What a fine floor! This is your workshop.'),
      ('mayor.plot_done.2','Ponha o gerador no terreno e a prensa ao lado, ligada por cabos. Depois a extratora no minério, e esteiras levando o cobre até a prensa.',
       'Put the generator on the plot and the press beside it, linked by cables. Then the extractor on the ore, and belts carrying the copper to the press.'),
      ('mayor.plot_done.3','O tutorial mostra cada passo. E quando não souber o que fazer, venha falar comigo.',
       'The tutorial shows each step. And whenever you do not know what to do, come talk to me.'),
      ('mayor.now','Sua tarefa agora: %s','Your task now: %s'),
      # Fim do tutorial e, depois, as fases do Quadro da fábrica (gargalos).
      ('outro.goal','Volte ao prefeito %s  %s blocos','Go back to the mayor %s  %s blocks'),
      ('outro.title','Fábrica fundada!','Factory founded!'),
      ('outro.sub','A vila conta com você','The village counts on you'),
      ('mayor.next.1','Olhe só o que você fez: energia, duas linhas, o plano da vila, o primeiro pedido pago e casa arrumada. Parabéns, engenheiro!',
       'Look at what you did: power, two lines, the village plan, the first paid order and a tidy home. Congratulations, engineer!'),
      ('mayor.next.2','Agora a vila vai pedir mais, e cada pedido maior trava em algum lugar: falta minério, energia, carvão, comida ou gente.',
       'Now the village will ask for more, and every bigger order gets stuck somewhere: not enough ore, power, coal, food or people.'),
      ('mayor.next.3','A próxima fase do Quadro pede 32 engrenagens. Uma prensa só não dá conta: o Quadro mostra onde a linha espera.',
       'The next board phase asks for 32 gears. One press will not keep up: the board shows where the line waits.'),
      ('mayor.next.4','E você não precisa fazer tudo sozinho: com as esmeraldas do pedido, a vila contrata gente para trabalhar com você.',
       'And you do not have to do it all alone: with the order emeralds, the village hires people to work with you.'),
      ('mayor.next.5','Volte aqui quando quiser: eu digo o que a vila precisa e o que vai travar a seguir.',
       'Come back any time: I will tell you what the village needs and what will get stuck next.'),
      ('mayor.board.none','Ainda não vi o seu Quadro da fábrica. Faça um (baú, 2 lingotes de cobre e graveto) e ponha perto das máquinas.',
       'I have not seen your Factory board yet. Make one (chest, 2 copper ingots and a stick) and put it near the machines.'),
      ('mayor.board.phase','A vila está na fase %s: entregue %s %s no Quadro da fábrica.','The village is in the %s phase: deliver %s %s at the Factory board.'),
      ('mayor.board.grow','Se a linha não dá conta, não é falta de esforço: é gargalo. Veja no Quadro o que parou e dobre essa parte.',
       'If the line cannot keep up, it is not lack of effort: it is a bottleneck. See on the board what stopped and double that part.'),
      ('mayor.board.done','Você levou a vila até o fim do plano. Agora é crescer: mais linhas, mais gente, mais cidade.',
       'You took the village to the end of the plan. Now it is about growing: more lines, more people, more town.'),
    ]
    for key,pt,en in intro: label(f'veio.onboarding.{key}',pt,en)
    label('veio.onboarding.done','Etapa concluída!','Step complete!')
    for i,(pt,en) in enumerate(goals): label(f'veio.onboarding.goal.{i}',pt,en)
    for i,(title,why,lines) in enumerate(steps):
        label(f'veio.onboarding.title.{i}',*title)
        label(f'veio.onboarding.why.{i}',*why)
        for n,line in enumerate(lines,1): label(f'veio.onboarding.do.{i}.{n}',*line)
    label('veio.onboarding.tip.faces','Dica · Entrada e saída: faixas VERDES (frente e laterais) recebem itens; a moldura VERMELHA (atrás) solta a produção e não aceita nada. A máquina nunca devolve o produto na esteira que a alimenta.',
          'Tip · Input and output: GREEN strips (front and sides) take items; the RED frame (back) releases products and accepts nothing. A machine never pushes its product onto the belt that feeds it.')
    label('veio.onboarding.tip.belt','Dica · Esteira: anda para longe de você ao ser colocada e não aceita nada pela frente. Leva até 5 minérios ou 1 produto pronto por bloco (2 na rápida, 4 na expressa). A chave inverte a direção.',
          'Tip · Belt: runs away from you when placed and accepts nothing from its front. Carries up to 5 ores or 1 finished product per block (2 on fast, 4 on express). The wrench reverses it.')
    label('veio.onboarding.hidden','Objetivo oculto. /veiotutorial mostrar volta a exibi-lo; sua fábrica continua funcionando.', 'Objective hidden. /veiotutorial mostrar shows it again; your factory keeps working.')
    label('veio.onboarding.shown','Tutorial ativado. Coloque ou abra a máquina que deseja usar nos exercícios.', 'Tutorial enabled. Place or open the machine you want to use for the exercises.')
    label('veio.onboarding.save_failed','Não foi possível salvar o tutorial. O progresso continua nesta sessão; confira o acesso à pasta do mundo.', 'Could not save the tutorial. Progress continues this session; check access to the world folder.')
    label('veio.onboarding.blocked.3','A saída está cheia. Leve a produção embora pela moldura VERMELHA (esteira, braço ou baú) ou esvazie a máquina.', 'The output is full. Carry products away from the RED frame (belt, arm or chest) or empty the machine.')
    label('veio.onboarding.blocked.5','A máquina está sem energia. Confira combustível no gerador e cabos encostando nas máquinas.', 'The machine has no power. Check generator fuel and cables touching the machines.')
    label('veio.onboarding.blocked.9','Redstone está pausando a máquina. Retire o sinal para retomar o exercício.', 'Redstone is pausing the machine. Remove the signal to resume the exercise.')
    add_village_labels(label, names, recipe_line)
    add_city_labels(label, names, recipe_line)
    add_night_labels(label)
    add_forge_labels(label, names, recipe_line)
    add_metro_labels(label)


def add_village_labels(label, names, recipe_line):
    """Capítulo 2 do tutorial: a vila operária, contada pelo prefeito Otávio (onboarding/village.rs)."""
    names.update({
        'farm_post': ('Posto do lavrador', 'Farmer post'), 'field_marker': ('Marco de lavoura', 'Field marker'),
        'hauler_post': ('Posto do carregador', 'Hauler post'), 'foreman_desk': ('Mesa do capataz', 'Foreman desk'),
        'house': ('Casa', 'House'), 'town_hall': ('Prefeitura', 'Town hall'), 'composter': ('Composteira', 'Composter'),
        'iron_hoe': ('Enxada de ferro', 'Iron hoe'), 'emerald': ('Esmeralda', 'Emerald'), 'barrel': ('Barril', 'Barrel'),
        'oak_fence': ('Cerca de carvalho', 'Oak fence'), 'wheat_seeds': ('Sementes de trigo', 'Wheat seeds'),
        'hopper': ('Funil', 'Hopper'), 'iron_plate': ('Chapa de ferro', 'Iron plate'), 'lectern': ('Atril', 'Lectern'),
        'bell': ('Sino', 'Bell'), 'oak_door': ('Porta de carvalho', 'Oak door'), 'glass_pane': ('Painel de vidro', 'Glass pane'),
        'stone_bricks': ('Tijolos de pedra', 'Stone bricks'), 'oak_planks': ('Tábuas de carvalho', 'Oak planks'),
    })
    for key, (pt, en) in names.items(): label(f'veio.onboarding.name.{key}', pt, en)
    steps = [
      # (título, fala do prefeito, passos, objetivo da barra, comentário ao concluir)
      (('Vila 1 de 8 · Um posto de trabalho', 'Village 1 of 8 · A workplace'),
       ('A fábrica já anda sozinha, mas uma vila precisa de mãos. Um aldeão contratado trabalha sem energia: ele come e anda. Comece pela comida: um posto do lavrador no meio de uma plantação.',
        'The factory runs on its own, but a village needs hands. A hired villager works without power: they eat and walk. Start with food: a farmer post in the middle of a crop field.'),
       [('1. Fabrique o posto do lavrador (ficha acima). A esmeralda vem de troca com aldeões.', '1. Craft the farmer post (card above). Emeralds come from trading with villagers.'),
        ('2. Ache ou plante uma lavoura de trigo, cenoura ou batata (até 9x9) perto da vila.', '2. Find or plant a wheat, carrot or potato field (up to 9x9) near the village.'),
        ('3. Coloque o posto no meio da lavoura: ele trabalha a camada dele, a de baixo e a de cima.', '3. Place the post in the middle of the field: it works its own layer, the one below and the one above.'),
        ('4. Abra o posto com a mão vazia: comida à esquerda, colheita à direita.', '4. Open the post with an empty hand: food on the left, harvest on the right.')],
       ('Coloque um posto do lavrador numa lavoura', 'Place a farmer post in a field'),
       ('Bom lugar. Agora falta quem trabalhe nele.', 'Good spot. Now it needs someone to work it.')),
      (('Vila 2 de 8 · Contratar', 'Village 2 of 8 · Hiring'),
       ('Contratar custa uma esmeralda, e o trabalhador come: cada tarefa gasta 2 pontos de comida. Pão vale 6 e paga 3 colheitas; trigo cru vale 1 e não paga nenhuma.',
        'Hiring costs one emerald, and the worker eats: each task costs 2 food points. Bread is worth 6 and pays for 3 harvests; raw wheat is worth 1 and pays for none.'),
       [('1. Ponha comida no espaço da esquerda do posto (pão, cenoura, batata, carne assada).', '1. Put food in the post\'s left slot (bread, carrots, potatoes, cooked meat).'),
        ('2. Traga um aldeão adulto para até 8 blocos do posto (os sem profissão são escolhidos primeiro).', '2. Bring an adult villager within 8 blocks of the post (jobless ones are chosen first).'),
        ('3. Com uma esmeralda na mão, clique com o botão direito no posto. O mais próximo é contratado.', '3. With an emerald in hand, right-click the post. The nearest one is hired.'),
        ('4. Sem ninguém livre, a esmeralda fica com você. Agachar + chave dispensa o trabalhador.', '4. If nobody is free, you keep the emerald. Sneak + wrench dismisses the worker.')],
       ('Contrate um aldeão no posto (esmeralda + botão direito)', 'Hire a villager at the post (emerald + right-click)'),
       ('Temos o primeiro trabalhador da vila! Agora vamos dizer onde ele trabalha.', 'Our first village worker! Now let us tell them where to work.')),
      (('Vila 3 de 8 · Demarcar a lavoura', 'Village 3 of 8 · Mark the field'),
       ('Sem marco, o lavrador cuida só de um quadrado em volta do posto. Com um marco de lavoura, a zona fica sua: começa em 9x9 e cresce até 21x21 conforme a vila trabalha nela.',
        'Without a marker, the farmer only tends a square around the post. With a field marker the zone is yours: it starts at 9x9 and grows to 21x21 as the village works it.'),
       [('1. Fabrique o marco de lavoura (ficha acima).', '1. Craft the field marker (card above).'),
        ('2. Coloque o marco no centro da plantação, dentro das faíscas verdes em volta do posto.', '2. Place the marker at the centre of the crop field, inside the green sparks around the post.'),
        ('3. Ponha no marco a semente que deve ser plantada: ele é o filtro dos lavradores.', '3. Put the seed to plant in the marker: it is the farmers\' filter.'),
        ('4. Mais tarde, a chave no marco escolhe o tamanho entre os níveis liberados.', '4. Later, the wrench on the marker picks the size among the unlocked levels.')],
       ('Coloque um marco de lavoura perto do posto', 'Place a field marker near the post'),
       ('A zona está demarcada. Vamos ver o trabalho render.', 'The zone is marked. Let us see the work pay off.')),
      (('Vila 4 de 8 · A primeira colheita', 'Village 4 of 8 · The first harvest'),
       ('O trabalhador anda até a planta madura, colhe, replanta e traz a colheita ao posto. É mais lento que uma máquina, mas não gasta energia.',
        'The worker walks to a ripe plant, harvests, replants and brings the crop to the post. Slower than a machine, but no power needed.'),
       [('1. Garanta comida no posto e plantas crescendo na zona.', '1. Keep food in the post and plants growing in the zone.'),
        ('2. Abra o posto: o painel mostra o estado do trabalhador e quantas colheitas a comida paga.', '2. Open the post: the panel shows the worker\'s state and how many harvests the food pays for.'),
        ('3. Se ele parar, passe o mouse no estado: "sem comida", "nada maduro", "caminho obstruído"...', '3. If they stop, hover over the status: "no food", "nothing ripe", "path blocked"...')],
       ('Colheitas entregues no posto: %s/4', 'Harvests delivered at the post: %s/4'),
       ('A comida da vila está garantida. Agora vamos poupar suas viagens.', 'The village food is secured. Now let us save you some trips.')),
      (('Vila 5 de 8 · O carregador', 'Village 5 of 8 · The hauler'),
       ('Um carregador abastece as máquinas a até 6 blocos do posto dele com o que você deixar à esquerda, e traz os produtos para a direita. Ele é a esteira que anda.',
        'A hauler feeds the machines within 6 blocks of their post with what you leave on the left, and brings products to the right. A belt that walks.'),
       [('1. Fabrique o posto do carregador (ficha acima).', '1. Craft the hauler post (card above).'),
        ('2. Coloque-o a até 6 blocos das máquinas que ele vai atender (a prensa, por exemplo).', '2. Place it within 6 blocks of the machines it will serve (the press, for example).'),
        ('3. Comida e insumos à esquerda; contrate com uma esmeralda, como no lavrador.', '3. Food and inputs on the left; hire with an emerald, as with the farmer.'),
        ('4. Ele não mexe em esteiras: use os dois juntos onde fizer sentido.', '4. They do not touch belts: use both where it makes sense.')],
       ('Contrate um carregador perto das máquinas', 'Hire a hauler near the machines'),
       ('Agora as máquinas têm quem as abasteça. Falta quem cuide dos trabalhadores.', 'Now the machines have someone to feed them. We still need someone to look after the workers.')),
      (('Vila 6 de 8 · O capataz', 'Village 6 of 8 · The foreman'),
       ('O capataz cuida de até 6 postos a até 12 blocos da mesa: leva comida a quem está ficando sem, recontrata quem perdeu o trabalhador e organiza o que os carregadores levam de um lugar a outro.',
        'The foreman looks after up to 6 posts within 12 blocks of the desk: brings food to those running low, rehires empty posts and plans what haulers carry from place to place.'),
       [('1. Fabrique a mesa do capataz (ficha acima).', '1. Craft the foreman desk (card above).'),
        ('2. Coloque a mesa a até 12 blocos dos postos (lavrador e carregador).', '2. Place the desk within 12 blocks of the posts (farmer and hauler).'),
        ('3. Deixe na mesa pão e algumas esmeraldas: são para os postos e para recontratar.', '3. Leave bread and a few emeralds on the desk: they are for the posts and for rehiring.'),
        ('4. Contrate o capataz com uma esmeralda.', '4. Hire the foreman with an emerald.')],
       ('Contrate um capataz perto dos postos', 'Hire a foreman near the posts'),
       ('A vila operária anda sozinha. Agora, a vila das pessoas.', 'The worker village runs itself. Now, the village of people.')),
      (('Vila 7 de 8 · Uma casa', 'Village 7 of 8 · A house'),
       ('Aldeões que moram perto de uma casa viram uma família: quem trabalha recebe salário, a família come da despensa e compra nas lojas. Coloque casas onde quiser; bairros espalhados vivem melhor.',
        'Villagers living near a house become a family: workers earn wages, the family eats from the pantry and shops. Place houses wherever you like; spread districts live better.'),
       [('1. Fabrique uma casa (ficha acima).', '1. Craft a house (card above).'),
        ('2. Coloque-a a até 10 blocos de onde os aldeões ficam: até 4 moram nela.', '2. Place it within 10 blocks of where villagers stay: up to 4 live in it.'),
        ('3. Ponha comida na despensa (3 espaços). Abra a casa para ver moradores, renda e IDH.', '3. Put food in the pantry (3 slots). Open the house to see residents, income and HDI.'),
        ('4. Evite mais de 6 casas vizinhas a 16 blocos e poluição perto: piora a moradia.', '4. Avoid more than 6 neighbour houses within 16 blocks and nearby pollution: it hurts housing.')],
       ('Coloque uma casa com moradores: %s', 'Place a house with residents: %s'),
       ('Nossa primeira família! Agora a vila precisa de governo.', 'Our first family! Now the village needs a government.')),
      (('Vila 8 de 8 · A prefeitura', 'Village 8 of 8 · The town hall'),
       ('Eu cuido da vila, mas preciso de um gabinete. A prefeitura mede população, emprego, PIB, IDH e Gini, cobra o imposto na fonte e libera as fases: Povoado, Vila, Cidade e Metrópole.',
        'I look after the village, but I need an office. The town hall measures population, jobs, GDP, HDI and Gini, collects tax at source and unlocks the phases: Hamlet, Village, City and Metropolis.'),
       [('1. Fabrique a prefeitura (ficha acima).', '1. Craft the town hall (card above).'),
        ('2. Coloque-a no centro da vila: ela vê 128 blocos em volta.', '2. Place it at the village centre: it sees 128 blocks around.'),
        ('3. Contrate um aldeão como meu secretário com uma esmeralda: ele faz o trabalho duro do gabinete.', '3. Hire a villager as my clerk with an emerald: they do the office\'s hard work.'),
        ('4. Clique com um livro na prefeitura: o relatório mostra os problemas e a meta da próxima fase.', '4. Click the town hall with a book: the report shows the problems and the next phase goal.')],
       ('Contrate o funcionário da prefeitura', 'Hire the town hall clerk'),
       ('Agora sim, temos um governo.', 'Now we have a government.')),
    ]
    for i, (title, why, lines, goal, done) in enumerate(steps):
        label(f'veio.village.title.{i}', *title)
        label(f'veio.village.why.{i}', *why)
        for n, line in enumerate(lines, 1): label(f'veio.village.do.{i}.{n}', *line)
        label(f'veio.village.goal.{i}', *goal)
        label(f'veio.village.done.{i}', *done)
    blocks = {0: 'farm_post', 2: 'field_marker', 4: 'hauler_post', 5: 'foreman_desk', 6: 'house', 7: 'town_hall'}
    for step, block in blocks.items(): label(f'veio.village.recipe.{step}', *recipe_line(block))
    sources = {
      0: ('De onde vem: esmeralda = troca com aldeões · enxada = 2 ferros + 2 gravetos · composteira = 7 lajes de madeira · barril = 6 tábuas + 2 lajes.',
          'Where from: emerald = trade with villagers · hoe = 2 iron + 2 sticks · composter = 7 wooden slabs · barrel = 6 planks + 2 slabs.'),
      2: ('De onde vem: cerca = tábuas e gravetos · sementes = quebrar grama.', 'Where from: fence = planks and sticks · seeds = break grass.'),
      4: ('De onde vem: chapa de ferro = ferro bruto na prensa · funil = 5 ferros + baú.', 'Where from: iron plate = raw iron in the press · hopper = 5 iron + chest.'),
      5: ('De onde vem: atril = 4 lajes + estante · sino = troca com aldeões ou vilas.', 'Where from: lectern = 4 slabs + bookshelf · bell = village trade or villages.'),
      6: ('De onde vem: tábuas = troncos · porta = 6 tábuas · painel de vidro = 6 vidros.', 'Where from: planks = logs · door = 6 planks · glass pane = 6 glass.'),
      7: ('De onde vem: tijolos de pedra = 4 pedras (pedregulho na fornalha) · atril e sino como na mesa do capataz.',
          'Where from: stone bricks = 4 stone (smelted cobblestone) · lectern and bell as for the foreman desk.'),
    }
    for step, (pt, en) in sources.items(): label(f'veio.village.from.{step}', pt, en)
    label('veio.village.intro.1', 'A fábrica está de pé. Agora vem a parte de que eu mais gosto: a vila. Gente trabalhando, famílias, um governo.',
          'The factory is up. Now comes my favourite part: the village. People working, families, a government.')
    label('veio.village.intro.2', 'São 8 passos. Eu explico cada um, e você vê no mundo. Clique em mim quando quiser que eu repita.',
          'Eight steps. I explain each one and you see it in the world. Click me whenever you want me to repeat.')
    label('veio.village.intro.3', 'Vamos começar pela comida.', 'Let us start with food.')
    label('veio.village.offer', 'Quando quiser aprender a vila (contratar, zonas, casas e prefeitura), fale comigo de novo.',
          'When you want to learn the village (hiring, zones, houses and the town hall), talk to me again.')
    label('veio.village.again', 'Onde paramos:', 'Where we were:')
    label('veio.village.bar', 'Vila %s/8 | %s', 'Village %s/8 | %s')
    label('veio.village.far', '%s · %s blocos ao %s', '%s · %s blocks %s')
    label('veio.village.start_title', 'Capítulo 3: a vila operária', 'Chapter 3: the worker village')
    label('veio.village.start_sub', 'Contratar, demarcar, morar e governar', 'Hire, mark, live and govern')
    label('veio.village.end_title', 'A vila é sua', 'The village is yours')
    label('veio.village.end_sub', 'Tutorial da vila concluído', 'Village tutorial complete')
    label('veio.village.end.1', 'Pronto: você contrata, demarca, organiza, abriga e governa. O resto é crescer.',
          'Done: you hire, mark, organize, house and govern. The rest is growing.')
    label('veio.village.end.2', 'Quando quiser transformar a vila em cidade (serviços, comércio, dinheiro e a primeira fase), fale comigo de novo.',
          'When you want to turn the village into a town (services, trade, money and the first phase), talk to me again.')
    label('veio.village.end.3', 'Eu vou receber visitantes pedindo festas e obras. Fique de olho no chat e na prefeitura.',
          'I will receive visitors asking for festivals and works. Keep an eye on the chat and the town hall.')
    label('veio.village.end.4', 'Dúvidas? Aperte J: a wiki tem tudo. E o livro da prefeitura mostra o que falta para a próxima fase.',
          'Questions? Press J: the wiki has everything. And the town hall book shows what the next phase needs.')


def add_city_labels(label, names, recipe_line):
    """Capítulo 3 do tutorial: a cidade, contada pelo prefeito Otávio (onboarding/city.rs)."""
    names.update({
        'well': ('Poço', 'Well'), 'shop': ('Loja', 'Shop'), 'pole': ('Poste de madeira', 'Wooden pole'), 'park': ('Parque', 'Park'),
        'cobblestone': ('Pedregulho', 'Cobblestone'), 'bucket': ('Balde', 'Bucket'), 'oak_sapling': ('Muda de carvalho', 'Oak sapling'),
        'poppy': ('Papoula', 'Poppy'), 'dandelion': ('Dente-de-leão', 'Dandelion'), 'lantern': ('Lanterna', 'Lantern'),
        'copper_ingot': ('Lingote de cobre', 'Copper ingot'), 'stick': ('Graveto', 'Stick'),
    })
    for key, (pt, en) in names.items(): label(f'veio.onboarding.name.{key}', pt, en)
    steps = [
      # (título, fala do prefeito, passos, objetivo da barra, comentário ao concluir, dica quando trava)
      (('Crescer 1 de 10 · Saneamento', 'Growth 1 of 10 · Sanitation'),
       ('Uma cidade começa pela água limpa. Sem saneamento as famílias adoecem e o IDH não sobe; da fase Vila em diante, casa nova só ganha alvará com um poço a até 24 blocos.',
        'A town starts with clean water. Without sanitation families fall ill and HDI stays low; from the Village phase on, a new house only gets a permit with a well within 24 blocks.'),
       [('1. Fabrique um poço (ficha acima).', '1. Craft a well (card above).'),
        ('2. Coloque-o a até 24 blocos das casas: um poço atende o bairro inteiro.', '2. Place it within 24 blocks of the houses: one well serves the whole district.'),
        ('3. O espaço do poço enche baldes: balde vazio entra, balde de água sai.', '3. The well slot fills buckets: an empty bucket goes in, a water bucket comes out.'),
        ('4. Abra uma casa: a linha de serviços mostra o saneamento.', '4. Open a house: the services line shows sanitation.')],
       ('Casas com saneamento: %s de %s', 'Houses with sanitation: %s of %s'),
       ('Água limpa. Agora as famílias precisam onde comprar.', 'Clean water. Now families need somewhere to shop.'),
       ('O poço só conta para casas com moradores a até 24 blocos dele. Leve o poço para o meio do bairro, ou espere a família se formar na casa.',
        'The well only counts for houses with residents within 24 blocks. Move the well to the middle of the district, or wait for the family to form.')),
      (('Crescer 2 de 10 · Comércio', 'Growth 2 of 10 · Trade'),
       ('As famílias ganham salário e gastam na loja mais próxima. O comerciante é independente: compra de você no atacado (75% do preço) e vende no varejo.',
        'Families earn wages and spend at the nearest shop. The merchant is independent: buys from you wholesale (75% of the price) and sells retail.'),
       [('1. Fabrique uma loja (ficha acima).', '1. Craft a shop (card above).'),
        ('2. Coloque-a perto das casas (da fase Cidade em diante, casa nova pede loja a até 40 blocos).', '2. Place it near the houses (from the City phase on, a new house needs a shop within 40 blocks).'),
        ('3. Contrate um comerciante com uma esmeralda, como nos postos.', '3. Hire a merchant with an emerald, as with the posts.'),
        ('4. Clique no comerciante: é a tela de troca do Minecraft, com o estoque real e o preço local.', '4. Click the merchant: it is the Minecraft trade screen, with real stock and the local price.')],
       ('Contrate um comerciante na loja', 'Hire a merchant at the shop'),
       ('A loja abriu as portas. Falta mercadoria.', 'The shop is open. It needs goods.'),
       ('Para contratar, precisa de um aldeão adulto livre a até 8 blocos da loja e uma esmeralda na mão. Traga um aldeão para perto, ou libere um trabalhador.',
        'To hire you need a free adult villager within 8 blocks of the shop and an emerald in hand. Bring a villager closer, or free up a worker.')),
      (('Crescer 3 de 10 · A primeira venda', 'Growth 3 of 10 · The first sale'),
       ('O que você entrega no estoque o comerciante compra fiado e paga com as vendas. Os preços são da vila: sobem quando falta e caem quando sobra.',
        'What you put in the stock the merchant buys on credit and pays from sales. Prices are local: they rise when goods are short and fall when they pile up.'),
       [('1. Abasteça o estoque da loja com comida (pão, batata assada) e bens (lã, couro, velas).', '1. Stock the shop with food (bread, baked potatoes) and goods (wool, leather, candles).'),
        ('2. Funil e carregador também abastecem: a loja aceita entregas automáticas.', '2. Hoppers and haulers also stock it: the shop takes automatic deliveries.'),
        ('3. De dia, as famílias vêm comprar. A venda paga 4% de imposto na fonte para a prefeitura.', '3. During the day families come to buy. Each sale withholds 4% tax for the town hall.'),
        ('4. Abra a loja para ver o caixa, os preços e o que mais vende.', '4. Open the shop to see the till, the prices and what sells most.')],
       ('Primeira venda: abasteça a loja e espere o dia', 'First sale: stock the shop and wait for daytime'),
       ('O dinheiro começou a girar. Vamos ver de onde ele vem.', 'Money has started to move. Let us see where it comes from.'),
       ('Famílias só compram de dia e com dinheiro na carteira. Sem salário (conta da companhia vazia) ninguém compra: ponha esmeraldas no cofre do centro de controle.',
        'Families only buy during the day and with money in their wallet. Without wages (empty company account) nobody buys: put emeralds in the control center vault.')),
      (('Crescer 4 de 10 · A conta da companhia', 'Growth 4 of 10 · The company account'),
       ('A companhia é você: paga os salários dos seus trabalhadores e recebe o atacado das lojas. O dinheiro só entra na vila em esmeraldas de verdade, pelo cofre do centro de controle.',
        'The company is you: it pays your workers\' wages and receives wholesale from the shops. Money only enters the village as real emeralds, through the control center vault.'),
       [('1. Vá ao centro de controle (o painel da sua casa).', '1. Go to the control center (the panel in your house).'),
        ('2. Ponha esmeraldas no cofre: cada uma vira 60 moedas na conta da companhia.', '2. Put emeralds in the vault: each one becomes 60 coins in the company account.'),
        ('3. Sem saldo, os trabalhadores ficam sem salário e as famílias empobrecem.', '3. Without a balance, workers go unpaid and families get poorer.'),
        ('4. Clique na linha da companhia no painel para sacar de volta, se precisar.', '4. Click the company line on the panel to withdraw, if you need to.')],
       ('Conta da companhia: %s moedas (ponha esmeraldas no cofre do centro de controle)', 'Company account: %s coins (put emeralds in the control center vault)'),
       ('Salários garantidos. Agora, conforto nas casas.', 'Wages secured. Now, comfort at home.'),
       ('O cofre é o espaço de esmeralda do centro de controle. Esmeraldas vêm de troca com aldeões, da central de contratos e da exportação.',
        'The vault is the emerald slot of the control center. Emeralds come from villager trades, the contract hub and exports.')),
      (('Crescer 5 de 10 · Energia nas casas', 'Growth 5 of 10 · Power at home'),
       ('Energia é serviço público: entra no IDH junto com saneamento, comércio e lazer. Da fase Cidade em diante, casa nova pede um poste a até 12 blocos.',
        'Power is a public service: it counts in HDI with sanitation, trade and leisure. From the City phase on, a new house needs a pole within 12 blocks.'),
       [('1. Fabrique postes (ficha acima; a receita rende 2).', '1. Craft poles (card above; the recipe makes 2).'),
        ('2. Leve uma linha de postes da fábrica até o bairro: eles se ligam sozinhos a até 8 blocos.', '2. Run a pole line from the factory to the district: they link on their own within 8 blocks.'),
        ('3. Um poste a até 12 blocos da casa já conta.', '3. One pole within 12 blocks of the house is enough.')],
       ('Casas com energia: %s de %s', 'Houses with power: %s of %s'),
       ('Luz nas casas. Falta um lugar para descansar.', 'Lights at home. We still need a place to relax.'),
       ('O poste precisa estar a até 12 blocos de uma casa com moradores. Não precisa estar ligado a um gerador para contar, mas a fábrica agradece.',
        'The pole must be within 12 blocks of a house with residents. It does not need a generator to count, but the factory will thank you.')),
      (('Crescer 6 de 10 · Lazer', 'Growth 6 of 10 · Leisure'),
       ('Gente que só trabalha não é feliz. Parque, praça e monumento dão lazer a 32 blocos: é o quarto serviço do IDH, e o ânimo da vila sobe nas festas.',
        'People who only work are not happy. Parks, plazas and monuments give leisure within 32 blocks: the fourth HDI service, and village morale rises at festivals.'),
       [('1. Fabrique um parque (ficha acima).', '1. Craft a park (card above).'),
        ('2. Coloque-o a até 32 blocos das casas.', '2. Place it within 32 blocks of the houses.'),
        ('3. Mais tarde eu vou pedir obras cívicas para os bairros sem lazer: a prefeitura paga a companhia.', '3. Later I will ask for civic works for districts without leisure: the town hall pays the company.'),
        ('4. O painel do parque mostra quantas casas ele atende.', '4. The park panel shows how many houses it serves.')],
       ('Casas com lazer: %s de %s', 'Houses with leisure: %s of %s'),
       ('Um parque! Agora precisamos de mais gente.', 'A park! Now we need more people.'),
       ('O parque leva alguns segundos para se registrar e só atende casas a até 32 blocos. Coloque-o no meio do bairro.',
        'The park takes a few seconds to register and only serves houses within 32 blocks. Place it in the middle of the district.')),
      (('Crescer 7 de 10 · Bairro', 'Growth 7 of 10 · A district'),
       ('Uma vila de verdade tem várias famílias. Cada casa abriga até 4 aldeões; bebês nascem onde há vaga. Casas muito juntas ficam apertadas: espalhe o bairro.',
        'A real village has several families. Each house holds up to 4 villagers; babies are born where there is room. Houses packed too tight are crowded: spread the district.'),
       [('1. Fabrique mais casas (ficha acima).', '1. Craft more houses (card above).'),
        ('2. Coloque-as perto dos aldeões, com até 6 vizinhas a 16 blocos.', '2. Place them near villagers, with at most 6 neighbours within 16 blocks.'),
        ('3. Camas livres fazem a vila crescer; comida na despensa mantém as famílias.', '3. Free beds make the village grow; food in the pantry keeps families going.'),
        ('4. Abra a prefeitura: ela mostra famílias, emprego e IDH da vila.', '4. Open the town hall: it shows families, jobs and the village HDI.')],
       ('Famílias na vila: %s/3', 'Families in the village: %s/3'),
       ('Três famílias! A vila já tem bairro.', 'Three families! The village has a district.'),
       ('Família se forma com aldeões a até 10 blocos da casa. Sem aldeões sobrando, a vila cresce com camas livres: ponha camas perto das casas.',
        'A family forms with villagers within 10 blocks of the house. With no spare villagers, the village grows with free beds: put beds near the houses.')),
      (('Crescer 8 de 10 · Impostos e a conta pública', 'Growth 8 of 10 · Taxes and the public account'),
       ('Eu não fabrico nada: a prefeitura vive do imposto retido em cada venda (4% agora, até 16% na Metrópole). Com ele pago os funcionários, os poços, a assistência aos pobres e as obras.',
        'I make nothing: the town hall lives on the tax withheld from each sale (4% now, up to 16% in a Metropolis). With it I pay clerks, wells, aid to the poor and public works.'),
       [('1. Vá à prefeitura e abra o painel: ele mostra a conta pública.', '1. Go to the town hall and open the panel: it shows the public account.'),
        ('2. Clique nela com um livro: o relatório traz a moeda em circulação por conta.', '2. Click it with a book: the report shows the money in circulation per account.'),
        ('3. Cada venda das lojas deixa uma parte aqui. Nada some: o dinheiro só muda de conta.', '3. Every shop sale leaves a share here. Nothing vanishes: money only changes account.'),
        ('4. O espaço de esmeralda da prefeitura aceita um aporte voluntário seu.', '4. The town hall emerald slot takes a voluntary contribution from you.')],
       ('Conta pública: %s moedas (o imposto chega a cada venda)', 'Public account: %s coins (tax arrives with every sale)'),
       ('A conta pública tem dinheiro. É assim que a vila se mantém.', 'The public account has money. That is how the village keeps going.'),
       ('A conta pública só recebe quando a loja vende. Confira se a loja tem estoque e se as famílias têm salário.',
        'The public account only receives when the shop sells. Check that the shop has stock and families have wages.')),
      (('Crescer 9 de 10 · Uma festa', 'Growth 9 of 10 · A festival'),
       ('O ano tem estações, domingos e feriados. Dois dias antes de cada um, um visitante vem à prefeitura pedir o que a festa precisa. Festa preparada sobe o ânimo, e o ânimo pesa no IDH.',
        'The year has seasons, Sundays and holidays. Two days before each one, a visitor comes to the town hall asking for what the festival needs. A prepared festival raises morale, and morale weighs on HDI.'),
       [('1. Quando o chat avisar que um visitante chegou, vá à prefeitura e clique nele.', '1. When chat says a visitor arrived, go to the town hall and click them.'),
        ('2. Ele diz o que precisa: a barra mostra o pedido e o que falta.', '2. They tell you what they need: the bar shows the request and what is missing.'),
        ('3. Entregue os itens no 3º espaço da prefeitura (Agenda). A prefeitura reembolsa a companhia.', '3. Deliver the items in the town hall\'s 3rd slot (Agenda). The town hall reimburses the company.'),
        ('4. A aba Agenda do painel (setas no título) mostra o calendário, o ânimo e os pedidos.', '4. The panel Agenda tab (arrows in the title) shows the calendar, morale and requests.')],
       ('Festa: %s', 'Festival: %s'),
       ('Festa preparada! A vila vai comemorar no dia.', 'Festival ready! The village will celebrate on the day.'),
       ('Os visitantes chegam 2 dias antes de cada domingo ou feriado. Enquanto isso, junte comida pronta: quase toda festa pede pão, bolo ou biscoito.',
        'Visitors arrive 2 days before each Sunday or holiday. Meanwhile, gather cooked food: almost every festival asks for bread, cake or cookies.')),
      (('Crescer 10 de 10 · Subir de fase', 'Growth 10 of 10 · Move up a phase'),
       ('O plano diretor tem quatro fases: Povoado, Vila, Cidade e Metrópole. Para virar Vila: 12 moradores, 3 famílias e IDH 0,40. Eu confiro no fim de cada dia.',
        'The master plan has four phases: Hamlet, Village, City and Metropolis. To become a Village: 12 residents, 3 families and HDI 0.40. I check at the end of each day.'),
       [('1. Abra a prefeitura ou clique com um livro: o relatório diz o que falta.', '1. Open the town hall or click it with a book: the report says what is missing.'),
        ('2. A barra mostra a parte mais fraca do IDH: comida, moradia, serviços ou renda.', '2. The bar shows the weakest part of HDI: food, housing, services or income.'),
        ('3. Moradores sobem com camas livres e casas com vaga.', '3. Residents grow with free beds and houses with room.'),
        ('4. Na fase Vila, a zona pode ir ao nível 3, a vila a 60 aldeões e 10 casas, e o imposto sobe para 8%.', '4. In the Village phase, zones can reach level 3, the village 60 villagers and 10 houses, and tax rises to 8%.')],
       ('Fase Vila · moradores %s/12 · famílias %s/3 · IDH %s/0,40%s', 'Village phase · residents %s/12 · families %s/3 · HDI %s/0.40%s'),
       ('Somos uma Vila! O plano diretor avançou.', 'We are a Village! The master plan moved forward.'),
       ('A fase só muda no fim do dia, com o funcionário da prefeitura presente. Se tudo já passou da meta, durma uma noite e confira de novo.',
        'The phase only changes at the end of the day, with the town hall clerk present. If everything is past the goal, sleep a night and check again.')),
    ]
    for i, (title, why, lines, goal, done, hint) in enumerate(steps):
        label(f'veio.city.title.{i}', *title)
        label(f'veio.city.why.{i}', *why)
        for n, line in enumerate(lines, 1): label(f'veio.city.do.{i}.{n}', *line)
        label(f'veio.city.goal.{i}', *goal)
        label(f'veio.city.done.{i}', *done)
        label(f'veio.city.hint.{i}', *hint)
    label('veio.city.goal.8_wait', 'Festa: o próximo visitante chega em %s dia(s) (%s). Enquanto isso, junte comida pronta', 'Festival: the next visitor arrives in %s day(s) (%s). Meanwhile, gather cooked food')
    label('veio.city.goal.8_soon', 'Festa: um visitante chega em breve à prefeitura', 'Festival: a visitor will soon arrive at the town hall')
    label('veio.city.bar_prefix', 'Crescer %s/10 |', 'Growth %s/10 |')
    for key, pt, en in [
        ('open', 'comerciante no balcão', 'merchant at the counter'), ('day', 'é dia', 'daytime'),
        ('wages', 'salários pagos', 'wages paid'), ('wallets', 'famílias com dinheiro', 'families with money'),
        ('clerk', 'funcionário na prefeitura', 'town hall clerk'), ('residents', 'moradores %s/12', 'residents %s/12'),
        ('families', 'famílias %s/3', 'families %s/3'), ('idh', 'IDH %s/0,40', 'HDI %s/0.40'),
    ]: label(f'veio.city.check.{key}', pt, en)
    label('veio.city.hint_ready', ' · o prefeito tem uma dica', ' · the mayor has a tip')
    more = [
      ('Saiba mais: J → Sistemas → Sociedade da vila (serviços e IDH).', 'Learn more: J → Systems → Village society (services and HDI).'),
      ('Saiba mais: J → Sistemas → Sociedade da vila (lojas e preços locais).', 'Learn more: J → Systems → Village society (shops and local prices).'),
      ('Saiba mais: J → Sistemas → Economia e moeda (o caminho do dinheiro).', 'Learn more: J → Systems → Economy and money (where money goes).'),
      ('Saiba mais: J → Sistemas → Economia e moeda (as quatro contas).', 'Learn more: J → Systems → Economy and money (the four accounts).'),
      ('Saiba mais: J → Sistemas → Energia (postes e alcance).', 'Learn more: J → Systems → Power (poles and reach).'),
      ('Saiba mais: J → Sistemas → Agenda, feriados e obras (lazer e ânimo).', 'Learn more: J → Systems → Calendar, holidays and civic works (leisure and morale).'),
      ('Saiba mais: J → Sistemas → Sociedade da vila (casas e famílias).', 'Learn more: J → Systems → Village society (houses and families).'),
      ('Saiba mais: J → Sistemas → Economia e moeda (imposto na fonte).', 'Learn more: J → Systems → Economy and money (tax at source).'),
      ('Saiba mais: J → Sistemas → Agenda, feriados e obras (o calendário e os visitantes).', 'Learn more: J → Systems → Calendar, holidays and civic works (calendar and visitors).'),
      ('Saiba mais: J → Sistemas → Sociedade da vila (fases do plano diretor).', 'Learn more: J → Systems → Village society (master plan phases).'),
    ]
    for i, (pt, en) in enumerate(more): label(f'veio.city.more.{i}', pt, en)
    weak = [('comida', 'food'), ('moradia', 'housing'), ('serviços', 'services'), ('renda', 'income')]
    for i, (pt, en) in enumerate(weak): label(f'veio.city.weak.{i}', f' · mais fraco: {pt}', f' · weakest: {en}')
    fixes = [
      ('Comida: ponha comida pronta na despensa das casas e na loja; pão e batata assada valem mais que trigo cru.',
       'Food: put cooked food in house pantries and in the shop; bread and baked potatoes are worth more than raw wheat.'),
      ('Moradia: casas apertadas e poluição pioram. Espalhe as casas e afaste o bairro das máquinas que poluem.',
       'Housing: crowded houses and pollution hurt. Spread the houses and keep the district away from polluting machines.'),
      ('Serviços: confira poço, poste, loja e lazer perto de cada casa. Abra uma casa para ver qual falta.',
       'Services: check well, pole, shop and leisure near each house. Open a house to see which is missing.'),
      ('Renda: mais aldeões empregados e salário em dia. Contrate em postos e mantenha esmeraldas no cofre da companhia.',
       'Income: more employed villagers and wages paid. Hire at posts and keep emeralds in the company vault.'),
    ]
    for i, (pt, en) in enumerate(fixes): label(f'veio.city.fix.{i}', pt, en)
    for step, block in {0: 'well', 1: 'shop', 4: 'pole', 5: 'park', 6: 'house'}.items():
        label(f'veio.city.recipe.{step}', *recipe_line(block))
    sources = {
      0: ('De onde vem: pedregulho = minerar pedra · balde = 3 lingotes de ferro.', 'Where from: cobblestone = mine stone · bucket = 3 iron ingots.'),
      1: ('De onde vem: barril = 6 tábuas + 2 lajes · esmeralda = troca com aldeões.', 'Where from: barrel = 6 planks + 2 slabs · emerald = trade with villagers.'),
      4: ('De onde vem: graveto = 2 tábuas · lingote de cobre = minério de cobre na fornalha.', 'Where from: stick = 2 planks · copper ingot = copper ore in a furnace.'),
      5: ('De onde vem: mudas = folhas de carvalho · flores = campos · cerca = tábuas e gravetos · lanterna = pepitas de ferro e tocha.',
          'Where from: saplings = oak leaves · flowers = meadows · fence = planks and sticks · lantern = iron nuggets and a torch.'),
      6: ('De onde vem: tábuas = troncos · porta = 6 tábuas · painel de vidro = 6 vidros.', 'Where from: planks = logs · door = 6 planks · glass pane = 6 glass.'),
    }
    for step, (pt, en) in sources.items(): label(f'veio.city.from.{step}', pt, en)
    label('veio.city.intro.1', 'A vila trabalha. Agora vamos fazer dela um lugar bom de morar: água, comércio, dinheiro, luz, lazer, festas e mais famílias.',
          'The village works. Now let us make it a good place to live: water, trade, money, light, leisure, festivals and more families.')
    label('veio.city.intro.2', 'São 10 passos. No fim, a vila sobe de fase, e o plano diretor libera mais aldeões, zonas maiores e mais casas.',
          'Ten steps. At the end the village moves up a phase, and the master plan unlocks more villagers, bigger zones and more houses.')
    label('veio.city.intro.3', 'Começamos pela água.', 'We start with water.')
    label('veio.city.bar', 'Crescer %s/10 | %s', 'Growth %s/10 | %s')
    label('veio.city.start_title', 'Capítulo 4: a vila cresce', 'Chapter 4: the village grows')
    label('veio.city.start_sub', 'Serviços, comércio, dinheiro, festas e a primeira fase', 'Services, trade, money, festivals and the first phase')
    label('veio.city.end_title', 'Uma cidade de verdade', 'A real town')
    label('veio.city.end_sub', 'Tutorial da cidade concluído', 'Town tutorial complete')
    label('veio.city.end.1', 'Água, comércio, salário, imposto, luz, lazer, festa e famílias: você sabe fazer uma cidade.',
          'Water, trade, wages, tax, light, leisure, festivals and families: you know how to build a town.')
    label('veio.city.end.2', 'Para Cidade: 36 moradores, 9 famílias, IDH 0,55, PIB de 400 por dia e 60% de emprego. O relatório da prefeitura acompanha cada número.',
          'For City: 36 residents, 9 families, HDI 0.55, GDP of 400 a day and 60% employment. The town hall report tracks each number.')
    label('veio.city.end.3', 'Para Metrópole: 80 moradores, 20 famílias, IDH 0,70, 3 bairros e Gini até 0,45: renda bem distribuída entre as famílias.',
          'For Metropolis: 80 residents, 20 families, HDI 0.70, 3 districts and Gini up to 0.45: income well spread across families.')
    label('veio.city.end.4', 'Com o porto ou a estação de carga, a vila vende para o mundo: contratos pagam mais e abrem mercados novos. Veja a aba Comércio exterior no centro de controle.',
          'With the cargo port or station, the village sells to the world: contracts pay more and open new markets. See the Foreign trade tab at the control center.')
    label('veio.city.end.5', 'Eu sigo na prefeitura. Clique em mim quando quiser saber o próximo passo da fábrica.',
          'I will be at the town hall. Click me whenever you want the factory\'s next step.')


def add_night_labels(label):
    """A noite (onboarding/night.rs): a primeira noite, o anoitecer de todo dia e o capítulo 2."""
    label('veio.night.villager', 'Um morador', 'A villager')
    label('veio.night.first_title', 'A primeira noite', 'The first night')
    label('veio.night.first_sub', 'A vila fecha as portas', 'The village shuts its doors')
    label('veio.night.first.1', 'Escureceu! Aqui ninguém fica na rua de noite: os monstros saem e caçam quem estiver fora.',
          'It is dark! Nobody stays outside here at night: monsters come out and hunt whoever is outdoors.')
    label('veio.night.first.2', 'Vá para a sua casa, ao norte da praça. Durma na cama ao lado do painel e a noite passa num instante.',
          'Go to your house, north of the plaza. Sleep in the bed next to the panel and the night passes in a moment.')
    label('veio.night.first.3', 'O trabalho espera o amanhecer. Amanhã a gente conversa sobre deixar a vila mais segura.',
          'Work can wait for dawn. Tomorrow we will talk about making the village safer.')
    label('veio.night.goal.home', 'Noite! Volte para casa %s  %s blocos', 'Night! Go home %s  %s blocks')
    label('veio.night.goal.inside', 'Em casa e a salvo. Durma na cama para a noite passar', 'Home and safe. Sleep in the bed to skip the night')
    label('veio.night.goal.shelter', 'Noite! Abrigue-se: feche-se num lugar com porta e luz até o amanhecer', 'Night! Take shelter: stay somewhere with a door and light until dawn')
    label('veio.night.dawn_title', 'Amanheceu', 'Morning')
    label('veio.night.dawn_sub', 'Você passou a primeira noite', 'You made it through the first night')
    label('veio.night.dawn', 'Bom dia! Viu como a vila se recolhe? Cada noite é assim. Um dia vamos ter muralha e guardas; até lá, ao escurecer, para casa.',
          'Good morning! See how the village shuts in? Every night is like that. One day we will have walls and guards; until then, home at dusk.')
    dusk = [
        ('O sino da vila tocou: está escurecendo. Os moradores fecham as portas.', 'The village bell rang: it is getting dark. Villagers shut their doors.'),
        ('Anoitece. Lá fora, os monstros já se mexem. Hora de voltar.', 'Night is falling. Out there, monsters are stirring. Time to head back.'),
        ('O sol se pôs. Quem trabalha na rua volta para casa; a fábrica segue sozinha.', 'The sun has set. Those working outside go home; the factory runs on its own.'),
        ('Escureceu. Luz e muralha são a melhor companhia de uma vila à noite.', 'It is dark. Light and walls are a village\'s best company at night.'),
    ]
    for i, (pt, en) in enumerate(dusk, 1): label(f'veio.night.dusk.{i}', pt, en)
    label('veio.night.offer', 'Quando quiser, fale comigo de novo: antes de crescer, a vila precisa perder o medo da noite.',
          'Whenever you like, talk to me again: before growing, the village needs to lose its fear of the night.')
    label('veio.night.start_title', 'Capítulo 2: a noite', 'Chapter 2: the night')
    label('veio.night.start_sub', 'Luz, muralha e uma noite em paz', 'Light, walls and a peaceful night')
    label('veio.night.intro.1', 'A vila tem medo da noite, e com razão: monstros caçam quem está fora, e com a fábrica crescendo vão aparecer coisas piores.',
          'The village fears the night, and rightly so: monsters hunt whoever is outside, and as the factory grows worse things will come.')
    label('veio.night.intro.2', 'Antes de crescer, vamos fazer a vila dormir em paz: luz, muralha e cama. Depois, a fábrica: o que ela polui, o muro dela e o nosso plano de defesa.',
          'Before growing, let us make the village sleep in peace: light, walls and beds. Then the factory: what it pollutes, its wall and our defense plan.')
    steps = [
      (('Noite 1 de 6 · Luz na vila', 'Night 1 of 6 · Light in the village'),
       ('Monstro nasce no escuro. Uma vila iluminada é uma vila onde eles não aparecem: cada tocha é um pedaço de chão seguro.',
        'Monsters spawn in the dark. A lit village is a village where they do not appear: every torch is a patch of safe ground.'),
       [('1. Faça tochas (carvão ou carvão vegetal + graveto) ou lanternas.', '1. Make torches (coal or charcoal + stick) or lanterns.'),
        ('2. Coloque-as na praça, nas ruas e em volta das casas (até 40 blocos do centro da vila).', '2. Place them in the plaza, the streets and around the houses (within 40 blocks of the village centre).'),
        ('3. Espalhe: uma tocha a cada 6 ou 7 blocos não deixa canto escuro.', '3. Spread them out: a torch every 6 or 7 blocks leaves no dark corner.'),
        ('4. A fábrica também: monstro perto de máquina é problema.', '4. The factory too: a monster near machines is trouble.')],
       ('Luzes na vila: %s/8', 'Lights in the village: %s/8'),
       ('Que vila bonita de noite! E bem mais segura.', 'What a lovely village at night! And much safer.')),
      (('Noite 2 de 6 · A muralha', 'Night 2 of 6 · The wall'),
       ('Luz afasta, muralha segura. Monstros andam, não escalam: um muro em volta da vila e da fábrica para quase tudo, inclusive os Saqueadores que um dia vão vir.',
        'Light keeps them away, a wall keeps them out. Monsters walk, they do not climb: a wall around the village and the factory stops almost everything, including the Raiders that will come one day.'),
       [('1. Construa um muro em volta da vila: muros de pedra, cercas ou blocos de pedra (entre 12 e 64 blocos do centro).', '1. Build a wall around the village: stone walls, fences or stone blocks (between 12 and 64 blocks from the centre).'),
        ('2. Dois blocos de altura bastam para o que anda; muros e cercas contam como altos.', '2. Two blocks high is enough for walkers; walls and fences count as tall.'),
        ('3. Deixe um portão (de cerca) ou uma porta para você e os aldeões passarem.', '3. Leave a gate (fence gate) or a door for you and the villagers.'),
        ('4. Inclua a fábrica: é ela que vai atrair os Saqueadores quando poluir.', '4. Include the factory: it is what will attract Raiders once it pollutes.')],
       ('Muralha: %s/64 blocos · portão: %s', 'Wall: %s/64 blocks · gate: %s'),
       ('Uma muralha! Agora sim a vila respira à noite.', 'A wall! Now the village can breathe at night.')),
      (('Noite 3 de 6 · Uma noite em paz', 'Night 3 of 6 · A peaceful night'),
       ('A noite não é para trabalhar na rua. A fábrica segue sozinha; você descansa e acorda com o turno pronto. Dormir também faz a noite passar.',
        'Night is not for working outside. The factory runs on its own; you rest and wake up to the work done. Sleeping also makes the night pass.'),
       [('1. Ao escurecer, volte para dentro da muralha.', '1. At dusk, get back inside the wall.'),
        ('2. Durma na cama da sua casa (ou em qualquer cama, à noite).', '2. Sleep in your house bed (or any bed, at night).'),
        ('3. Os trabalhadores civis também dormem; o quartel, quando houver, fica de plantão.', '3. Civilian workers sleep too; the barracks, once built, stays on watch.')],
       ('Uma noite em paz', 'A peaceful night'),
       ('Bom dia! Assim que se faz: a vila dormiu em paz.', 'Good morning! That is how it is done: the village slept in peace.')),
      (('Noite 4 de 6 · O preço da fábrica', 'Night 4 of 6 · The price of the factory'),
       ('Toda máquina que trabalha polui o chunk em que está, e o gerador mais que todas. A poluição se espalha e só uma parte some. É ela que um dia vai chamar os Saqueadores.',
        'Every working machine pollutes its chunk, and the generator most of all. Pollution spreads and only part of it fades. It is what will one day call the Raiders.'),
       [('1. Abra uma máquina que trabalha (o gerador, por exemplo) com a mão vazia: o painel mostra a poluição por minuto.', '1. Open a working machine (the generator, for example) with an empty hand: the panel shows pollution per minute.'),
        ('2. A barra mostra a poluição do chunk da oficina: o aviso chega em 400; as ondas, acima de 1.200.', '2. The bar shows the workshop chunk pollution: the warning comes at 400; waves above 1,200.'),
        ('3. Lavouras absorvem poluição a cada colheita; o módulo de eficiência corta pela metade.', '3. Crops absorb pollution with every harvest; the efficiency module cuts it in half.'),
        ('4. Não dá para não poluir: dá para planejar.', '4. You cannot avoid polluting: you can plan for it.')],
       ('Poluição da oficina: %s · aviso em %s · ondas acima de %s · abra uma máquina que polui', 'Workshop pollution: %s · warning at %s · waves above %s · open a polluting machine'),
       ('Agora você lê o perigo antes de ele chegar.', 'Now you read the danger before it arrives.')),
      (('Noite 5 de 6 · A muralha da fábrica', 'Night 5 of 6 · The factory wall'),
       ('Os Saqueadores vêm atrás de quem está na fábrica e, sem ninguém por perto, golpeiam a máquina que polui. Quatro golpes param uma máquina. A oficina também precisa de muro.',
        'Raiders go after whoever is at the factory and, with nobody around, strike the polluting machine. Four blows stop a machine. The workshop needs a wall too.'),
       [('1. Cerque a oficina: muros, cercas ou pedra entre 4 e 24 blocos do gerador.', '1. Fence the workshop: walls, fences or stone between 4 and 24 blocks from the generator.'),
        ('2. Deixe um portão para você e os carregadores passarem.', '2. Leave a gate for you and the haulers.'),
        ('3. Esteiras que saem da oficina passam pelo portão ou, mais tarde, por baixo do muro com o túnel de esteira.', '3. Belts leaving the workshop go through the gate or, later, under the wall with the belt tunnel.'),
        ('4. Ilumine dentro do muro: monstro não nasce em chão claro.', '4. Light up inside the wall: monsters do not spawn on lit ground.')],
       ('Muralha da fábrica: %s/32 blocos · portão: %s', 'Factory wall: %s/32 blocks · gate: %s'),
       ('A oficina está cercada. Venha conversar comigo: tenho um plano.', 'The workshop is walled. Come talk to me: I have a plan.')),
      (('Noite 6 de 6 · O conselho de defesa', 'Night 6 of 6 · The defense council'),
       ('Muralha e luz seguram o começo. Quando a fábrica crescer, vamos precisar de mais. Venha à praça: quero te mostrar o plano, e onde cada defesa falha.',
        'Walls and light hold the beginning. When the factory grows, we will need more. Come to the plaza: I want to show you the plan, and where each defense fails.'),
       [('1. Fale com o prefeito Otávio na praça.', '1. Talk to mayor Otávio in the plaza.'),
        ('2. Ele explica as ondas, o quartel e a torre Tesla: o que cada um custa e onde falha.', '2. He explains the waves, the barracks and the Tesla tower: what each costs and where it fails.'),
        ('3. Ainda não dá para construir: o quartel chega com os circuitos, e a Tesla com a pesquisa de eletrônica.', '3. You cannot build them yet: the barracks comes with circuits, and the Tesla with the electronics research.')],
       ('Fale com o prefeito na praça: o conselho de defesa', 'Talk to the mayor in the plaza: the defense council'),
       ('Conselho dado. Que a vila durma em paz.', 'Advice given. May the village sleep in peace.')),
    ]
    for i, (title, why, lines, goal, done) in enumerate(steps):
        label(f'veio.night.title.{i}', *title)
        label(f'veio.night.why.{i}', *why)
        for n, line in enumerate(lines, 1): label(f'veio.night.do.{i}.{n}', *line)
        label(f'veio.night.goal.{i}', *goal)
        label(f'veio.night.done.{i}', *done)
    label('veio.night.goal.2_day', 'Uma noite em paz: quando escurecer, durma na cama', 'A peaceful night: when it gets dark, sleep in the bed')
    label('veio.night.goal.2_night', 'Noite! Volte para dentro da muralha e durma na cama', 'Night! Get inside the wall and sleep in the bed')
    label('veio.night.gate_yes', 'sim', 'yes')
    label('veio.night.gate_no', 'falta', 'missing')
    label('veio.night.bar', 'Noite %s/6 | %s', 'Night %s/6 | %s')
    more = [
      ('Saiba mais: J → Sistemas → Poluição e defesa.', 'Learn more: J → Systems → Pollution and defense.'),
      ('Saiba mais: J → Sistemas → Poluição e defesa (as ondas e os muros).', 'Learn more: J → Systems → Pollution and defense (waves and walls).'),
      ('Saiba mais: J → Sistemas → Vila operária (descanso dos trabalhadores).', 'Learn more: J → Systems → Worker village (workers resting).'),
      ('Saiba mais: J → Sistemas → Poluição e defesa (o medidor e as ondas).', 'Learn more: J → Systems → Pollution and defense (the meter and the waves).'),
      ('Saiba mais: J → Sistemas → Poluição e defesa (o que os Saqueadores atacam).', 'Learn more: J → Systems → Pollution and defense (what Raiders attack).'),
      ('Saiba mais: J → Sistemas → Poluição e defesa (quartel, Tesla e seus gargalos).', 'Learn more: J → Systems → Pollution and defense (barracks, Tesla and their bottlenecks).'),
    ]
    council = [
      ('Sente aqui. Para atender a vila e, um dia, o mundo, a fábrica vai poluir; não tem outro jeito. E poluição traz Saqueadores.',
       'Sit down. To supply the village and, one day, the world, the factory will pollute; there is no other way. And pollution brings Raiders.'),
      ('O sinal: acima de 400 eu aviso no chat. Um dia depois, nas noites em que a poluição passa de 1.200, vêm as ondas: 4 Saqueadores na primeira, até 12.',
       'The signal: above 400 I warn you in chat. A day later, on nights when pollution passes 1,200, the waves come: 4 Raiders the first time, up to 12.'),
      ('Da 3ª noite de ataque vêm arqueiros; da 5ª, o Bruto: lento, duro, vai direto às máquinas e bate o dobro. Quatro golpes param uma máquina, até um kit de reparo.',
       'From the 3rd attack night come archers; from the 5th, the Brute: slow, tough, goes straight for the machines and hits twice as hard. Four blows stop a machine, until a repair kit.'),
      ('Com os circuitos, você ergue o quartel: um soldado por prédio. Milícia chega rápido, guarda aguenta e se cura, arqueiro atira de longe, mago acerta dois.',
       'With circuits you raise the barracks: one soldier per building. Militia arrives fast, the guard holds and heals, the archer shoots from afar, the mage hits two.'),
      ('Fraqueza do quartel: o soldado come a cada golpe. Sem comida no quartel, ninguém sai do posto. E trocar de classe custa esmeralda.',
       'The barracks weakness: the soldier eats with every blow. With no food in the barracks, nobody leaves their post. And changing class costs an emerald.'),
      ('Mais adiante, com a pesquisa de eletrônica, vem a torre Tesla: abate sozinha o monstro mais perto, a até 10 blocos. Mas cada tiro gasta 2.400 FE, e ela guarda 20.000: são 8 tiros, um a cada 2 segundos.',
       'Later, with the electronics research, comes the Tesla tower: it kills the nearest monster on its own, within 10 blocks. But each shot costs 2,400 FE and it stores 20,000: that is 8 shots, one every 2 seconds.'),
      ('Fraqueza da Tesla: de noite o solar não rende. Sem acumuladores ou gerador, ela apaga no meio da onda. E gerador a carvão polui... e chama mais Saqueadores.',
       'The Tesla weakness: at night solar gives nothing. Without batteries or a generator it goes dark mid-wave. And a coal generator pollutes... and calls more Raiders.'),
      ('É esse o jogo da defesa: muralha, comida no quartel e energia guardada para a noite. Eu vou te lembrando. Por hoje, a vila está pronta.',
       'That is the defense game: walls, food in the barracks and energy saved for the night. I will keep reminding you. For today, the village is ready.'),
    ]
    for i, (pt, en) in enumerate(council, 1): label(f'veio.night.council.{i}', pt, en)
    for i, (pt, en) in enumerate(more): label(f'veio.night.more.{i}', pt, en)
    label('veio.night.end_title', 'A vila perdeu o medo', 'The village lost its fear')
    label('veio.night.end_sub', 'Capítulo da noite concluído', 'Night chapter complete')
    label('veio.night.end.1', 'Luz, muralha e cama: a vila dorme tranquila. Mas lembre: quanto mais a fábrica produz, mais ela polui.',
          'Light, walls and beds: the village sleeps soundly. But remember: the more the factory makes, the more it pollutes.')
    label('veio.night.end.2', 'Com poluição alta, vêm os Saqueadores, de noite e em ondas. Não tem jeito: para atender a vila e o mundo, a fábrica vai poluir.',
          'With high pollution come the Raiders, at night and in waves. There is no way around it: to supply the village and the world, the factory will pollute.')
    label('veio.night.end.3', 'Por isso a defesa cresce com a fábrica. Agora, fale comigo quando quiser: a vila precisa de mãos.',
          'That is why defense grows with the factory. Now talk to me whenever you like: the village needs hands.')


def add_chapter(label, key, steps, extra):
    """Textos de um capítulo do motor comum (onboarding/chapter.rs)."""
    for i, step in enumerate(steps):
        title, why, lines, done, more = step[:5]
        label(f'veio.{key}.title.{i}', *title)
        label(f'veio.{key}.why.{i}', *why)
        for n, line in enumerate(lines, 1): label(f'veio.{key}.do.{i}.{n}', *line)
        label(f'veio.{key}.done.{i}', *done)
        label(f'veio.{key}.more.{i}', *more)
    for name, pair in extra.items(): label(f'veio.{key}.{name}', *pair)


def add_forge_labels(label, names, recipe_line):
    """Capítulo 5: a grande indústria (onboarding/forge.rs)."""
    names.update({
        'drill_mk2': ('Extratora Mk.II', 'Extractor Mk.II'), 'barracks': ('Quartel', 'Barracks'), 'tesla': ('Torre Tesla', 'Tesla tower'),
        'battery': ('Acumulador', 'Battery'), 'repair_kit': ('Kit de reparo', 'Repair kit'), 'drill': ('Extratora', 'Extractor'),
        'steel_plate': ('Chapa de aço', 'Steel plate'), 'circuit': ('Placa de circuito', 'Circuit board'), 'module': ('Módulo mecânico', 'Mechanical module'),
        'iron_ingot': ('Lingote de ferro', 'Iron ingot'), 'smooth_stone': ('Pedra lisa', 'Smooth stone'), 'lightning_rod': ('Para-raios', 'Lightning rod'),
        'copper_block': ('Bloco de cobre', 'Block of copper'), 'redstone': ('Redstone', 'Redstone'), 'wire': ('Fiação de cobre', 'Copper wire'),
        'gear': ('Engrenagem', 'Gear'), 'iron_plate': ('Chapa de ferro', 'Iron plate'),
    })
    for key, (pt, en) in names.items(): label(f'veio.onboarding.name.{key}', pt, en)
    more = lambda pt, en: ('Saiba mais: J → Sistemas → ' + pt, 'Learn more: J → Systems → ' + en)
    steps = [
      (('Indústria 1 de 8 · Oficina e aço', 'Industry 1 of 8 · Workshop and steel'),
       ('O Quadro da fábrica é o plano da indústria: cada fase pede uma pilha que só uma linha funcionando entrega. A Oficina pede 32 engrenagens; o Aço, 16 chapas de aço.',
        'The Factory board is the industry plan: each phase asks for a stack only a working line delivers. The Workshop asks for 32 gears; Steel, 16 steel plates.'),
       [('1. Faça o Quadro da fábrica (baú, 2 lingotes de cobre e graveto) e ponha perto das máquinas.', '1. Make the Factory board (chest, 2 copper ingots and a stick) and place it near the machines.'),
        ('2. Entregue 32 engrenagens: libera chave, triturador, forno de ligas e kit de reparo.', '2. Deliver 32 gears: unlocks the wrench, crusher, alloy furnace and repair kit.'),
        ('3. Forno de ligas: 2 chapas de ferro + carvão = 1 chapa de aço. Entregue 16.', '3. Alloy furnace: 2 iron plates + coal = 1 steel plate. Deliver 16.'),
        ('4. Gargalo: a linha de engrenagens puxa 40 FE/t, mais que uma roda d\'água. Planeje a energia.', '4. Bottleneck: the gear line draws 40 FE/t, more than one water wheel. Plan the power.')],
       ('Aço! A fábrica virou indústria.', 'Steel! The factory became an industry.'), more('Fases e pesquisa.', 'Phases and research.')),
      (('Indústria 2 de 8 · Eletrônica', 'Industry 2 of 8 · Electronics'),
       ('Circuito é cobre virando inteligência: fiação na cortadora, silício no triturador ou no forno de ligas, e o montador de circuitos.',
        'A circuit is copper turned smart: wire in the cutter, silicon in the crusher or alloy furnace, and the circuit assembler.'),
       [('1. Cortadora: chapa de cobre vira fiação. Triturador: quartzo vira silício (ou 4 areias + carvão no forno de ligas).', '1. Cutter: copper plate becomes wire. Crusher: quartz becomes silicon (or 4 sand + coal in the alloy furnace).'),
        ('2. Montador de circuitos: 2 fiações + silício = 1 circuito.', '2. Circuit assembler: 2 wires + silicon = 1 circuit.'),
        ('3. Entregue 16 circuitos no Quadro: libera solar, Mk.II, módulos e o quartel.', '3. Deliver 16 circuits at the board: unlocks solar, Mk.II, modules and the barracks.'),
        ('4. Gargalo: silício. Sem ir ao Nether, a rota da areia é mais longa, mas não para.', '4. Bottleneck: silicon. Without the Nether, the sand route is longer, but it works.')],
       ('Circuitos! Agora a fábrica pensa.', 'Circuits! Now the factory thinks.'), more('Fases e pesquisa.', 'Phases and research.')),
      (('Indústria 3 de 8 · Extratora Mk.II', 'Industry 3 of 8 · Extractor Mk.II'),
       ('A Mk.II extrai uma unidade por segundo, o dobro da comum, e chega a diamante e esmeralda. Com ela a linha dobra, e a poluição também: 4 por segundo.',
        'The Mk.II mines one unit per second, twice the basic one, and reaches diamond and emerald. With it the line doubles, and so does pollution: 4 per second.'),
       [('1. Fabrique a Mk.II (ficha acima): ela usa uma extratora comum.', '1. Craft the Mk.II (card above): it uses a basic extractor.'),
        ('2. Troque a extratora de um veio bom pela Mk.II.', '2. Swap the extractor on a good vein for the Mk.II.'),
        ('3. Dobre o que vem depois: duas prensas por Mk.II, esteira rápida se precisar.', '3. Double what comes after: two presses per Mk.II, a fast belt if needed.'),
        ('4. Veja no painel quanto ela polui por minuto.', '4. See on the panel how much it pollutes per minute.')],
       ('O dobro de minério. A vila agradece, e o ar sente.', 'Twice the ore. The village is grateful, and the air feels it.'), more('Jazidas e extração.', 'Deposits and extraction.')),
      (('Indústria 4 de 8 · O primeiro aviso', 'Industry 4 of 8 · The first warning'),
       ('Com a fábrica maior, a poluição passa de 400 e eu aviso no chat. Um dia depois, nas noites em que passar de 1.200, as ondas começam. É o preço de crescer.',
        'With a bigger factory, pollution passes 400 and I warn you in chat. A day later, on nights it passes 1,200, the waves begin. It is the price of growing.'),
       [('1. Deixe a fábrica trabalhar: a barra mostra o chunk mais poluído.', '1. Let the factory work: the bar shows the most polluted chunk.'),
        ('2. Quando o aviso vier, confira a muralha, a luz e o portão da fábrica.', '2. When the warning comes, check the wall, the light and the factory gate.'),
        ('3. Lavouras perto absorvem; o módulo de eficiência corta metade. Mas para atender a vila, vai poluir.', '3. Nearby crops absorb; the efficiency module halves it. But to supply the village, it will pollute.'),
        ('4. Prepare-se: as próximas etapas são a defesa.', '4. Get ready: the next steps are defense.')],
       ('Aí está o aviso. Agora, defesa.', 'There is the warning. Now, defense.'), more('Poluição e defesa.', 'Pollution and defense.')),
      (('Indústria 5 de 8 · O quartel', 'Industry 5 of 8 · The barracks'),
       ('Muralha segura quem anda; o quartel luta. Um soldado por prédio, de plantão também à noite. Ele come a cada golpe: sem comida, ninguém sai do posto.',
        'Walls stop walkers; the barracks fights. One soldier per building, on watch at night too. They eat with every blow: with no food, nobody leaves their post.'),
       [('1. Fabrique o quartel (ficha acima) e coloque dentro da muralha, perto da fábrica.', '1. Craft the barracks (card above) and place it inside the wall, near the factory.'),
        ('2. Comida no espaço da esquerda; uma esmeralda no da direita, para o salário.', '2. Food in the left slot; an emerald in the right one, for wages.'),
        ('3. Contrate com uma esmeralda; agache com uma esmeralda para trocar a classe.', '3. Hire with an emerald; sneak with an emerald to change class.'),
        ('4. Milícia chega rápido, guarda aguenta e se cura, arqueiro atira de longe, mago acerta dois.', '4. Militia is fast, the guard holds and heals, the archer shoots from afar, the mage hits two.')],
       ('Temos um soldado. A vila dorme mais tranquila.', 'We have a soldier. The village sleeps easier.'), more('Vila operária (quartel).', 'Worker village (barracks).')),
      (('Indústria 6 de 8 · A torre Tesla', 'Industry 6 of 8 · The Tesla tower'),
       ('A Tesla defende sozinha: abate o monstro mais perto a 10 blocos. Mas cada tiro gasta 2.400 FE e ela guarda 20.000, só 8 tiros. Energia é o gargalo dela.',
        'The Tesla defends on its own: it kills the nearest monster within 10 blocks. But each shot costs 2,400 FE and it stores 20,000, just 8 shots. Power is its bottleneck.'),
       [('1. A receita vem da pesquisa de eletrônica: 128 circuitos e 128 molduras no laboratório.', '1. The recipe comes from the electronics research: 128 circuits and 128 frames in the lab.'),
        ('2. Coloque a torre perto das máquinas que poluem: é para lá que os Saqueadores vão.', '2. Place the tower near the polluting machines: that is where Raiders go.'),
        ('3. Ligue-a na rede (cabo ou poste): ela só atira com energia guardada.', '3. Connect it to the grid (cable or pole): it only fires with stored energy.'),
        ('4. Ela não acerta aldeões, animais nem chefes.', '4. It does not hit villagers, animals or bosses.')],
       ('A Tesla está carregada. Agora, a noite.', 'The Tesla is charged. Now, the night.'), more('Poluição e defesa (gargalos).', 'Pollution and defense (bottlenecks).')),
      (('Indústria 7 de 8 · Energia para a noite', 'Industry 7 of 8 · Power for the night'),
       ('De noite o solar não rende. Sem reserva, a Tesla apaga no meio da onda. Cada acumulador guarda 30.000 FE; o gerador a carvão segura, mas polui e chama mais ondas.',
        'At night solar gives nothing. Without a reserve, the Tesla goes dark mid-wave. Each battery stores 30,000 FE; a coal generator holds, but it pollutes and calls more waves.'),
       [('1. Fabrique acumuladores (a receita rende 4).', '1. Craft batteries (the recipe makes 4).'),
        ('2. Ligue na mesma rede da Tesla, com cabo ou poste.', '2. Connect them to the Tesla grid, with cable or pole.'),
        ('3. Carregue de dia, com sobra de energia.', '3. Charge them by day, with spare power.'),
        ('4. Conta rápida: uma onda de 12 pede 12 tiros, 28.800 FE. Uma Tesla sozinha não dá conta.', '4. Quick math: a wave of 12 needs 12 shots, 28,800 FE. One Tesla alone is not enough.')],
       ('Energia guardada para a noite.', 'Power saved for the night.'), more('Energia.', 'Power.')),
      (('Indústria 8 de 8 · Reparos', 'Industry 8 of 8 · Repairs'),
       ('Quatro golpes param uma máquina, e o Bruto bate o dobro. Máquina parada não perde estoque: um kit de reparo, com o botão direito, devolve 50%.',
        'Four blows stop a machine, and the Brute hits twice as hard. A stopped machine keeps its stock: a repair kit, right-clicked, restores 50%.'),
       [('1. Fabrique kits de reparo (a receita rende 4).', '1. Craft repair kits (the recipe makes 4).'),
        ('2. Guarde alguns perto da fábrica.', '2. Keep some near the factory.'),
        ('3. Depois de uma onda, ande pela fábrica: o painel mostra a integridade de cada máquina.', '3. After a wave, walk the factory: the panel shows each machine\'s integrity.')],
       ('Pronto para o que vier.', 'Ready for whatever comes.'), more('Poluição e defesa.', 'Pollution and defense.')),
    ]
    extra = {
        'intro.1': ('A vila cresceu e pede mais: mais chapas, mais engrenagens, mais comida. A oficina de hoje não dá conta.', 'The village grew and asks for more: more plates, more gears, more food. Today\'s workshop cannot keep up.'),
        'intro.2': ('Vamos levar a fábrica ao aço e à eletrônica, com a extratora Mk.II. Mas produzir mais é poluir mais.', 'Let us take the factory to steel and electronics, with the Mk.II extractor. But making more means polluting more.'),
        'intro.3': ('E poluição traz Saqueadores. Desta vez a defesa vem junto: quartel, torre Tesla e energia para a noite.', 'And pollution brings Raiders. This time defense comes along: barracks, Tesla tower and power for the night.'),
        'end.1': ('A fábrica virou indústria, e a vila tem defesa.', 'The factory became an industry, and the village has a defense.'),
        'end.2': ('As ondas vão voltar, maiores com a poluição: até 12 Saqueadores, arqueiros e o Bruto. Mais Tesla, mais energia, mais soldados.', 'Waves will return, bigger with pollution: up to 12 Raiders, archers and the Brute. More Teslas, more power, more soldiers.'),
        'end.3': ('Com a indústria de pé, a vila pode virar cidade. Fale comigo quando quiser.', 'With industry standing, the village can become a city. Talk to me whenever you like.'),
        'end.4': ('E lembre: o Quadro da fábrica sempre diz o próximo gargalo.', 'And remember: the Factory board always tells you the next bottleneck.'),
        'start_title': ('Capítulo 5: a grande indústria', 'Chapter 5: heavy industry'),
        'start_sub': ('Aço, eletrônica, Mk.II e a defesa da fábrica', 'Steel, electronics, Mk.II and the factory defense'),
        'end_title': ('A vila tem uma indústria', 'The village has an industry'),
        'end_sub': ('Capítulo da grande indústria concluído', 'Heavy industry chapter complete'),
        'bar': ('Indústria %s/8 | %s', 'Industry %s/8 | %s'), 'bar_prefix': ('Indústria %s/8 |', 'Industry %s/8 |'),
        'goal.1': ('Entregue 16 circuitos no Quadro da fábrica (fase Eletrônica)', 'Deliver 16 circuits at the Factory board (Electronics phase)'),
        'goal.2': ('Coloque uma extratora Mk.II num veio', 'Place an Mk.II extractor on a vein'),
        'goal.3': ('Poluição mais alta: %s · o aviso vem em %s', 'Highest pollution: %s · the warning comes at %s'),
        'goal.7': ('Tenha um kit de reparo no inventário', 'Have a repair kit in your inventory'),
        'check.board': ('Quadro da fábrica', 'Factory board'), 'check.workshop': ('fase Oficina', 'Workshop phase'),
        'check.steel': ('fase Aço', 'Steel phase'), 'check.barracks': ('quartel colocado', 'barracks placed'),
        'check.soldier': ('soldado contratado', 'soldier hired'), 'check.rations': ('comida no quartel', 'food in the barracks'),
        'check.tesla': ('torre colocada', 'tower placed'), 'check.charge': ('tiros guardados: %s/%s', 'shots stored: %s/%s'),
        'check.battery': ('acumulador colocado', 'battery placed'), 'check.stored': ('energia guardada: %s FE', 'energy stored: %s FE'),
        'wave': ('Os Saqueadores vieram! É o preço da fábrica. Muralha, quartel e Tesla seguram; no dia seguinte, reparos.', 'The Raiders came! That is the price of the factory. Walls, barracks and Tesla hold; the next day, repairs.'),
    }
    add_chapter(label, 'forge', steps, extra)
    for step, block in {2: 'drill_mk2', 4: 'barracks', 5: 'tesla', 6: 'battery', 7: 'repair_kit'}.items():
        label(f'veio.forge.recipe.{step}', *recipe_line(block))
    sources = {
      2: ('De onde vem: chapa de aço = forno de ligas · circuito = montador de circuitos · módulo = moldura + engrenagem no integrador.', 'Where from: steel plate = alloy furnace · circuit = circuit assembler · module = frame + gear in the integrator.'),
      4: ('De onde vem: lingotes de ferro na fornalha · pedra lisa = pedregulho na fornalha 2 vezes · esmeralda = troca com aldeões.', 'Where from: iron ingots from the furnace · smooth stone = cobblestone smelted twice · emerald = villager trades.'),
      5: ('De onde vem: para-raios = 3 lingotes de cobre · bloco de cobre = 9 lingotes · circuitos e aço da fábrica.', 'Where from: lightning rod = 3 copper ingots · copper block = 9 ingots · circuits and steel from the factory.'),
      6: ('De onde vem: redstone das minas · fiação na cortadora · aço no forno de ligas.', 'Where from: redstone from mines · wire in the cutter · steel in the alloy furnace.'),
      7: ('De onde vem: chapas de ferro na prensa · engrenagem no montador.', 'Where from: iron plates in the press · gear in the assembler.'),
    }
    for step, (pt, en) in sources.items(): label(f'veio.forge.from.{step}', pt, en)


def add_metro_labels(label):
    """Capítulo 6: a cidade (onboarding/metro.rs)."""
    more = lambda pt, en: ('Saiba mais: J → Sistemas → ' + pt, 'Learn more: J → Systems → ' + en)
    steps = [
      (('Cidade 1 de 6 · Nove famílias', 'City 1 of 6 · Nine families'),
       ('Uma cidade precisa de gente. Cada casa abriga até 4; bebê nasce onde há cama livre e vaga em casa. Na fase Vila, a vila pode ter até 60 aldeões e 10 casas.',
        'A city needs people. Each house holds up to 4; babies are born where there is a free bed and room at home. In the Village phase, the village can have up to 60 villagers and 10 houses.'),
       [('1. Faça mais casas, em bairros espalhados (até 6 vizinhas a 16 blocos).', '1. Build more houses, in spread districts (at most 6 neighbours within 16 blocks).'),
        ('2. Camas livres perto das casas fazem a vila crescer.', '2. Free beds near houses make the village grow.'),
        ('3. Casa nova pede poço a 24 blocos (o alvará da fase Vila).', '3. A new house needs a well within 24 blocks (the Village phase permit).'),
        ('4. Comida nas despensas mantém as famílias.', '4. Food in the pantries keeps families going.')],
       ('Nove famílias! A vila já parece cidade.', 'Nine families! The village already looks like a city.'), more('Sociedade da vila (casas e famílias).', 'Village society (houses and families).')),
      (('Cidade 2 de 6 · Emprego', 'City 2 of 6 · Jobs'),
       ('Família sem salário empobrece, e o IDH cai. A Cidade pede 60% dos adultos empregados: postos, lojas, quartel e a prefeitura contam.',
        'A family without wages gets poorer, and HDI falls. The City asks for 60% of adults employed: posts, shops, barracks and the town hall count.'),
       [('1. Contrate em postos de trabalho: lavrador, pastor, carregador, capataz.', '1. Hire at work posts: farmer, shepherd, hauler, foreman.'),
        ('2. Lojas e o quartel também empregam.', '2. Shops and the barracks employ too.'),
        ('3. A conta da companhia precisa pagar: esmeraldas no cofre.', '3. The company account must pay: emeralds in the vault.'),
        ('4. Abra a prefeitura: ela mostra o emprego da vila.', '4. Open the town hall: it shows village employment.')],
       ('Gente trabalhando, dinheiro girando.', 'People working, money moving.'), more('Economia e moeda (salários).', 'Economy and money (wages).')),
      (('Cidade 3 de 6 · Serviços para todos', 'City 3 of 6 · Services for all'),
       ('Na Cidade, toda casa conta. Saneamento, energia, comércio e lazer para cada família: é o que mais pesa no IDH. Da fase Cidade, casa nova pede também poste a 12 blocos.',
        'In a City, every house counts. Sanitation, power, trade and leisure for every family: it weighs most on HDI. From the City phase, a new house also needs a pole within 12 blocks.'),
       [('1. Abra cada casa: a linha de serviços diz o que falta.', '1. Open each house: the services line says what is missing.'),
        ('2. Poço a 24, poste a 12, loja a 40 e lazer a 32 blocos.', '2. Well within 24, pole within 12, shop within 40 and leisure within 32 blocks.'),
        ('3. Bairros afastados precisam dos próprios serviços.', '3. Far districts need their own services.'),
        ('4. A barra mostra quantas casas já têm tudo.', '4. The bar shows how many houses have everything.')],
       ('Toda família bem servida.', 'Every family well served.'), more('Sociedade da vila (serviços e IDH).', 'Village society (services and HDI).')),
      (('Cidade 4 de 6 · Uma obra cívica', 'City 4 of 6 · A civic work'),
       ('Bairro sem lazer pede obra. A cada 4 dias eu peço um parque, uma praça ou um monumento para o bairro mais carente, e a prefeitura paga a obra à companhia.',
        'A district without leisure asks for a work. Every 4 days I ask for a park, a plaza or a monument for the neediest district, and the town hall pays the company for it.'),
       [('1. Quando o pedido chegar, a barra mostra o que e onde.', '1. When the request arrives, the bar shows what and where.'),
        ('2. Construa o bloco cívico a até 32 blocos do bairro, em 6 dias.', '2. Build the civic block within 32 blocks of the district, within 6 days.'),
        ('3. Parque 240, praça 420, monumento 720 moedas, pagos à companhia.', '3. Park 240, plaza 420, monument 720 coins, paid to the company.'),
        ('4. Obra pronta: ânimo +10 e lazer permanente no bairro.', '4. Work done: morale +10 and permanent leisure in the district.')],
       ('Obra entregue. A cidade tem a sua cara.', 'Work delivered. The city has its own face.'), more('Agenda, feriados e obras.', 'Calendar, holidays and civic works.')),
      (('Cidade 5 de 6 · O PIB', 'City 5 of 6 · GDP'),
       ('O PIB da vila é o que as famílias compram por dia, mais o que a vila exporta. Para Cidade: 400 por dia. Mais gente com salário, mais loja abastecida, mais PIB.',
        'The village GDP is what families buy per day, plus what the village exports. For a City: 400 a day. More people with wages, more stocked shops, more GDP.'),
       [('1. Lojas sempre abastecidas: estoque vazio não vende.', '1. Shops always stocked: an empty stock sells nothing.'),
        ('2. Salários em dia: família sem dinheiro não compra.', '2. Wages paid: a family without money does not buy.'),
        ('3. Bens além de comida (lã, couro, velas, livros) aumentam o que cada casa gasta.', '3. Goods beyond food (wool, leather, candles, books) raise what each house spends.'),
        ('4. Mais tarde, o porto ou a estação de carga somam a exportação.', '4. Later, the cargo port or station adds exports.')],
       ('A economia da vila tem tamanho de cidade.', 'The village economy is city-sized.'), more('Economia e moeda.', 'Economy and money.')),
      (('Cidade 6 de 6 · A fase Cidade', 'City 6 of 6 · The City phase'),
       ('Tudo junto, no fim do dia: 36 moradores, 9 famílias, IDH 0,55, PIB 400 e 60% de emprego, com o meu secretário no gabinete. Aí eu declaro a Cidade.',
        'Everything together, at the end of the day: 36 residents, 9 families, HDI 0.55, GDP 400 and 60% employment, with my clerk in the office. Then I declare the City.'),
       [('1. A barra lista cada meta com ✔ ou ✘.', '1. The bar lists each goal with ✔ or ✘.'),
        ('2. O livro da prefeitura mostra os bairros e os problemas.', '2. The town hall book shows districts and problems.'),
        ('3. Na fase Cidade: zonas nível 4, 100 aldeões, 24 casas e imposto de 12%.', '3. In the City phase: zone level 4, 100 villagers, 24 houses and 12% tax.'),
        ('4. Se tudo estiver verde, durma uma noite: a fase muda no fim do dia.', '4. If everything is green, sleep a night: the phase changes at the end of the day.')],
       ('Somos uma Cidade!', 'We are a City!'), more('Sociedade da vila (fases).', 'Village society (phases).')),
    ]
    extra = {
        'intro.1': ('A indústria trouxe trabalho, e o trabalho trouxe gente. Chegou a hora de a Vila virar Cidade.', 'Industry brought work, and work brought people. It is time for the Village to become a City.'),
        'intro.2': ('Para Cidade: 36 moradores, 9 famílias, IDH 0,55, PIB de 400 por dia e 60% de emprego.', 'For a City: 36 residents, 9 families, HDI 0.55, GDP of 400 a day and 60% employment.'),
        'intro.3': ('Uma coisa de cada vez. Comecemos pelas famílias.', 'One thing at a time. Let us start with families.'),
        'end.1': ('Uma Cidade! Do punhado de casas que eu governava para isto.', 'A City! From the handful of houses I governed to this.'),
        'end.2': ('A Metrópole pede 80 moradores, 20 famílias, IDH 0,70, 3 bairros e Gini até 0,45: renda bem distribuída.', 'A Metropolis asks for 80 residents, 20 families, HDI 0.70, 3 districts and Gini up to 0.45: income well spread.'),
        'end.3': ('A vila agora produz mais do que consome. Lá fora tem gente querendo comprar.', 'The village now makes more than it consumes. Out there, people want to buy.'),
        'end.4': ('Quando quiser saber o próximo passo, fale comigo.', 'Whenever you want the next step, talk to me.'),
        'start_title': ('Capítulo 6: a cidade', 'Chapter 6: the city'), 'start_sub': ('De Vila a Cidade', 'From Village to City'),
        'end_title': ('Uma Cidade de verdade', 'A real City'), 'end_sub': ('Capítulo da cidade concluído', 'City chapter complete'),
        'bar': ('Cidade %s/6 | %s', 'City %s/6 | %s'), 'bar_prefix': ('Cidade %s/6 |', 'City %s/6 |'),
        'goal.0': ('Famílias: %s/9 · moradores: %s', 'Families: %s/9 · residents: %s'),
        'goal.1': ('Emprego: %s%% (%s de %s adultos) · meta 60%%', 'Jobs: %s%% (%s of %s adults) · goal 60%%'),
        'goal.3': ('Obra: %s', 'Work: %s'),
        'goal.3_wait': ('Obra cívica: o próximo pedido chega em até 4 dias (casas com lazer: %s de %s)', 'Civic work: the next request comes within 4 days (houses with leisure: %s of %s)'),
        'goal.4': ('PIB do dia: %s/400 moedas', 'GDP of the day: %s/400 coins'),
        'check.water': ('água %s/%s', 'water %s/%s'), 'check.power': ('energia %s/%s', 'power %s/%s'), 'check.all': ('tudo %s/%s', 'everything %s/%s'),
        'check.residents': ('moradores %s/36', 'residents %s/36'), 'check.families': ('famílias %s/9', 'families %s/9'),
        'check.idh': ('IDH %s/0,55', 'HDI %s/0.55'), 'check.gdp': ('PIB %s/400', 'GDP %s/400'), 'check.jobs': ('emprego %s%%/60%%', 'jobs %s%%/60%%'),
    }
    add_chapter(label, 'metro', steps, extra)
