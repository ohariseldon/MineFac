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
      ('11/16 | Postes: leve energia longe, %s/2 postes', '11/16 | Poles: carry power far, %s/2 poles'),
      ('12/16 | Coloque um posto do carregador perto da prensa', '12/16 | Place a hauler post near the press'),
      ('13/16 | Contrate um aldeão: esmeralda na mão, clique no posto', '13/16 | Hire a villager: emerald in hand, click the post'),
      ('14/16 | Ponha comida no 1º espaço do posto', '14/16 | Put food in the post first slot'),
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
       [('1. Fabrique a prensa e os cabos (fichas acima; siga a lista acima da barra de itens).', '1. Craft the press and the cables (cards above; follow the list above your hotbar).'),
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
      (('Etapa 11 de 16 · Energia longe', 'Step 11 of 16 · Power far away'),
       ('Cabo serve para perto. Para levar energia até outra parte do terreno, use postes: o fio vai pelo ar.', 'Cables are for short runs. To carry power to another part of the plot, use poles: the wire goes through the air.'),
       [('1. Fabrique postes de madeira (ficha acima: cada receita rende 2).', '1. Craft wooden poles (card above: each recipe makes 2).'),
        ('2. Ponha o primeiro a até 2 blocos do gerador: ele puxa a energia de lá.', '2. Put the first within 2 blocks of the generator: it pulls power from there.'),
        ('3. Ponha o segundo a até 8 blocos do primeiro: o fio aparece sozinho. Máquinas a 2 blocos dele recebem energia.', '3. Put the second within 8 blocks of the first: the wire appears by itself. Machines within 2 blocks of it get power.')]),
      (('Etapa 12 de 16 · Um posto de trabalho', 'Step 12 of 16 · A work post'),
       ('Aldeões trabalham em postos. O carregador abastece e esvazia as máquinas a até 6 blocos do posto.', 'Villagers work at posts. The hauler fills and empties the machines within 6 blocks of the post.'),
       [('1. Chapa de ferro: ferro bruto na prensa. Esmeralda: troque com os aldeões da vila (o flecheiro compra gravetos).', '1. Iron plate: raw iron in the press. Emerald: trade with the village folk (the fletcher buys sticks).'),
        ('2. Fabrique o posto do carregador (ficha acima). Junte outra esmeralda para o salário.', '2. Craft the hauler post (card above). Get another emerald for the wage.'),
        ('3. Coloque o posto a até 6 blocos da prensa e de um baú.', '3. Place the post within 6 blocks of the press and a chest.')]),
      (('Etapa 13 de 16 · Contratar', 'Step 13 of 16 · Hiring'),
       ('Um aldeão contratado trabalha sem você: carrega, colhe e cuida das máquinas enquanto você cresce a fábrica.', 'A hired villager works without you: hauls, harvests and looks after machines while you grow the factory.'),
       [('1. Traga um aldeão livre para perto (a até 8 blocos do posto), ou vá até ele.', '1. Bring a free villager close (within 8 blocks of the post), or go to him.'),
        ('2. Com uma esmeralda na mão, clique no posto: o aldeão livre mais perto é contratado.', '2. Holding an emerald, click the post: the nearest free villager is hired.'),
        ('3. Ninguém perto? O posto espera 30 s: clique, ainda com a esmeralda, no aldeão que quiser.', '3. Nobody close? The post waits 30 s: click, still holding the emerald, the villager you want.')]),
      (('Etapa 14 de 16 · Comida para quem trabalha', 'Step 14 of 16 · Food for workers'),
       ('Cada tarefa gasta comida. Posto sem comida para, e esse é um gargalo que você vai ver muito.', 'Every task costs food. A post with no food stops, and that is a bottleneck you will see a lot.'),
       [('1. Faça pão (3 trigos na bancada) ou asse batatas. A vila tem plantações.', '1. Make bread (3 wheat at the crafting table) or bake potatoes. The village has farms.'),
        ('2. Abra o posto e ponha a comida no 1º espaço.', '2. Open the post and put the food in the first slot.'),
        ('3. Mais tarde, o posto do fazendeiro colhe sozinho e um carregador leva a comida.', '3. Later, the farm post harvests on its own and a hauler brings the food.')]),
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
       ('Você aprendeu energia, máquinas, esteiras, o Quadro, postes, trabalhadores, comida e a vila.', 'You learned power, machines, belts, the board, poles, workers, food and the village.'),
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
      ('Um dia a sua fábrica vai ser grande demais para cabos. Os postes levam energia de um lado ao outro da vila.', 'One day your factory will be too big for cables. Poles carry power from one side of the village to the other.'),
      ('Você não precisa carregar tudo sozinho. A vila tem gente querendo trabalho.', 'You do not need to carry everything yourself. The village has people who want work.'),
      ('Quem trabalha recebe salário, uma esmeralda, e passa a cuidar do posto.', 'A worker gets a wage, one emerald, and looks after the post.'),
      ('Trabalhador com fome não trabalha. Comida também é parte da fábrica.', 'A hungry worker does not work. Food is part of the factory too.'),
      ('E você também precisa comer! Uma refeição quente em casa e você trabalha o dobro.', 'And you need to eat too! A hot meal at home and you work twice as hard.'),
      ('Daqui de casa você vê a vila toda. É o seu escritório de engenheiro.', 'From home you see the whole village. It is your engineer office.'),
      ('Você aprendeu o que eu sabia ensinar. Venha me ver na praça.', 'You learned all I could teach. Come see me in the square.'),
    ]
    for step,(pt,en) in enumerate(stories): label(f'veio.onboarding.story.{step}',pt,en)
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
      'hauler_post':('Posto do carregador','Hauler post'), 'hopper':('Funil','Hopper'),
      'emerald':('Esmeralda','Emerald'), 'iron_plate':('Chapa de ferro','Iron plate'),
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
          8:['factory_board'],10:['pole'],11:['hauler_post']}
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
      (10,'De onde vem: graveto = 2 tábuas · lingote de cobre na fornalha.',
          'Where from: stick = 2 planks · copper ingot from the furnace.'),
      (11,'De onde vem: funil = 5 lingotes de ferro + baú · chapa de ferro = ferro bruto na prensa · esmeralda = comércio com os aldeões.',
          'Where from: hopper = 5 iron ingots + chest · iron plate = raw iron in the press · emerald = trading with villagers.')]:
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
      ('mayor.hello.2','Embaixo destes morros tem cobre e ferro. Com máquinas, isso vira chapa, engrenagem, trabalho e comida na mesa.',
       'Under these hills there is copper and iron. With machines it becomes plates, gears, jobs and food on the table.'),
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
      ('mayor.machine.1','Pronto, aqui está o seu gerador. E as receitas da prensa e dos cabos: estão no seu livro de receitas.',
       'Done, here is your generator. And the press and cable recipes: they are in your recipe book.'),
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
      ('mayor.next.1','Olhe só o que você fez: energia, uma linha, o plano da vila, gente trabalhando e casa arrumada. Parabéns, engenheiro!',
       'Look at what you did: power, a line, the village plan, people working and a tidy home. Congratulations, engineer!'),
      ('mayor.next.2','Agora a vila vai pedir mais, e cada pedido maior trava em algum lugar: falta minério, energia, carvão, comida ou gente.',
       'Now the village will ask for more, and every bigger order gets stuck somewhere: not enough ore, power, coal, food or people.'),
      ('mayor.next.3','A próxima fase do Quadro pede 32 engrenagens. Uma prensa só não dá conta: vai precisar de mais extratoras e mais prensas.',
       'The next board phase asks for 32 gears. One press will not keep up: you will need more extractors and more presses.'),
      ('mayor.next.4','Todo dia o Quadro traz um pedido da vila que paga em esmeraldas. Com elas, você contrata mais gente.',
       'Every day the board brings a village order that pays in emeralds. With them, you hire more people.'),
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
