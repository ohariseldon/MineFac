"""Player-facing text for the server-observed workshop tutorial."""
def add_labels(label):
    goals=[
      ('1/8 | Coloque um gerador na oficina', '1/8 | Place a workshop generator'),
      ('2/8 | Leve energia até a prensa', '2/8 | Supply power to the press'),
      ('3/8 | Produza chapas de cobre: %s/4', '3/8 | Produce copper plates: %s/4'),
      ('4/8 | Produza uma engrenagem: %s/1', '4/8 | Produce a gear: %s/1'),
      ('5/8 | Extraia cobre com a máquina: %s/1', '5/8 | Extract copper with a machine: %s/1'),
      ('6/8 | Abasteça a prensa automaticamente: %s/2', '6/8 | Automatically feed the press: %s/2'),
      ('7/8 | Guarde chapas automaticamente: %s/4', '7/8 | Automatically store plates: %s/4'),
      ('8/8 | Central: entregue 24 chapas e 8 engrenagens', '8/8 | Hub: deliver 24 plates and 8 gears'),
    ]
    # Cada etapa: título, por quê, passos numerados (DO_LINES em onboarding.rs) e o kit.
    steps=[
      (('Etapa 1 de 8 · Energia', 'Step 1 of 8 · Power'),
       ('Toda máquina do Veio precisa de energia. O gerador queima carvão e manda energia pelos cabos.', 'Every Veio machine needs power. The generator burns coal and sends power through cables.'),
       [('1. Junte os materiais da ficha: a lista acima da barra de itens marca em verde o que você já tem.', '1. Gather the materials on the card: the list above your hotbar turns green as you get them.'),
        ('2. Fabrique o gerador na bancada e coloque-o onde vai ficar a sua oficina.', '2. Craft the generator at a crafting table and place it where your workshop will be.'),
        ('3. Abra o gerador com a mão vazia e ponha carvão no espaço de combustível.', '3. Open the generator with an empty hand and put coal in its fuel slot.')]),
      (('Etapa 2 de 8 · Ligar uma máquina', 'Step 2 of 8 · Power a machine'),
       ('A prensa transforma cobre bruto em chapas, mas só trabalha com energia chegando.', 'The press turns raw copper into plates, but only works with power arriving.'),
       [('1. Fabrique a prensa e os cabos (fichas acima; siga a lista acima da barra de itens).', '1. Craft the press and the cables (cards above; follow the list above your hotbar).'),
        ('2. Coloque a prensa e ligue-a ao gerador com cabos encostados: gerador > cabo > cabo > prensa.', '2. Place the press and connect it with touching cables: generator > cable > cable > press.'),
        ('3. Veja as marcas da prensa: faixas VERDES recebem itens; a moldura VERMELHA (atrás) solta a produção.', '3. Look at the press marks: GREEN strips take items; the RED frame (back) releases products.'),
        ('4. A etapa avança quando a prensa receber energia. Abra-a para ver a barra de energia.', '4. The step advances when the press receives power. Open it to see the power bar.')]),
      (('Etapa 3 de 8 · Produzir à mão', 'Step 3 of 8 · Produce by hand'),
       ('Antes de automatizar, veja como uma máquina trabalha: entrada, progresso e saída.', 'Before automating, see how a machine works: input, progress and output.'),
       [('1. Minere cobre bruto: minério de cobre com picareta de pedra ou melhor.', '1. Mine raw copper: copper ore with a stone pickaxe or better.'),
        ('2. Abra a prensa e ponha o cobre BRUTO no espaço da esquerda (entrada).', '2. Open the press and put RAW copper in the left slot (input).'),
        ('3. Espere a seta encher e retire as chapas da direita (saída).', '3. Wait for the arrow to fill and take the plates from the right (output).'),
        ('4. Faça 4 chapas novas. Se a máquina parar, passe o mouse no estado do painel: ele diz o motivo.', '4. Make 4 new plates. If the machine stops, hover over the panel status: it tells you why.')]),
      (('Etapa 4 de 8 · Uma cadeia de produção', 'Step 4 of 8 · A production chain'),
       ('Fábricas encadeiam máquinas: a chapa da prensa vira engrenagem no montador.', 'Factories chain machines: the press plate becomes a gear in the assembler.'),
       [('1. Fabrique o montador (ficha acima) com 2 das chapas que você fez.', '1. Craft the assembler (card above) with 2 of your plates.'),
        ('2. Coloque-o, ligue energia e ponha 2 chapas na entrada.', '2. Place it, power it and put 2 plates in its input.'),
        ('3. Retire a engrenagem pronta da saída.', '3. Take the finished gear from the output.')]),
      (('Etapa 5 de 8 · Extração automática', 'Step 5 of 8 · Automatic extraction'),
       ('Minerar à mão não escala. A extratora tira minério da jazida sozinha, sem acabar.', 'Mining by hand does not scale. The extractor mines the deposit on its own, forever.'),
       [('1. Fabrique a extratora (ficha acima).', '1. Craft the extractor (card above).'),
        ('2. Coloque-a EM CIMA de minério de cobre, sem quebrá-lo. Veio maior embaixo = mais rápido.', '2. Place it ON TOP of copper ore, without mining it. Bigger vein below = faster.'),
        ('3. Leve energia até ela. O cobre aparece na saída (moldura vermelha).', '3. Bring power to it. Copper appears at the output (red frame).')]),
      (('Etapa 6 de 8 · Esteiras', 'Step 6 of 8 · Belts'),
       ('Agora a extratora alimenta a prensa: o cobre vai sozinho, sem você carregar.', 'Now the extractor feeds the press: copper travels on its own.'),
       [('1. Fabrique esteiras (ficha acima: cada receita rende 16).', '1. Craft belts (card above: each recipe makes 16).'),
        ('2. A primeira encosta na moldura VERMELHA da extratora. A esteira anda para LONGE de você.', '2. The first touches the extractor RED frame. A belt runs AWAY from you.'),
        ('3. A última entra numa face VERDE da prensa (frente ou lado; nunca na vermelha).', '3. The last one enters a GREEN face of the press (front or side; never the red one).'),
        ('4. Entregue 2 cobres pela esteira e deixe a prensa fazer 1 chapa com eles.', '4. Deliver 2 copper by belt and let the press make 1 plate from them.')]),
      (('Etapa 7 de 8 · Saída e estoque', 'Step 7 of 8 · Output and storage'),
       ('Uma linha completa também leva a produção embora. Senão a saída enche e a máquina para.', 'A complete line also carries products away. Otherwise the output fills and the machine stops.'),
       [('1. Fabrique um baú e coloque-o perto da prensa.', '1. Craft a chest and place it near the press.'),
        ('2. Coloque esteiras saindo da moldura VERMELHA da prensa até o baú: ela despeja nele sozinha.', '2. Run belts from the press RED frame into the chest: it unloads by itself.'),
        ('3. Guarde 4 chapas feitas pela linha. Braços, funis e carregadores também valem.', '3. Store 4 plates made by the line. Arms, hoppers and haulers also count.')]),
      (('Etapa 8 de 8 · A primeira encomenda', 'Step 8 of 8 · The first order'),
       ('A central recebe as encomendas da vila e paga em materiais: é a meta da sua fábrica.', 'The hub takes village orders and pays in materials: it is your factory goal.'),
       [('1. Faça uma engrenagem no montador e fabrique a central (ficha acima).', '1. Make a gear in the assembler and craft the hub (card above).'),
        ('2. Junte 24 chapas e 8 engrenagens (2 chapas cada).', '2. Gather 24 plates and 8 gears (2 plates each).'),
        ('3. Na central: chapas na entrada 1, engrenagens na entrada 2; deixe as saídas livres.', '3. In the hub: plates in input 1, gears in input 2; keep outputs clear.'),
        ('4. Recompensa: 64 esteiras e 32 cabos. Dica: ligue a esteira da prensa à central.', '4. Reward: 64 belts and 32 cables. Tip: run the press belt into the hub.')]),
      (('Tutorial concluído!', 'Tutorial complete!'),
       ('Você aprendeu energia, máquinas, cadeias, extração, esteiras e estoque.', 'You learned power, machines, chains, extraction, belts and storage.'),
       [('Próximo: o Quadro da fábrica mostra as fases e as próximas metas.', 'Next: the Factory board shows the phases and next goals.'),
        ('Ampliar: uma extratora comum alimenta uma prensa. Mais máquinas pedem mais energia.', 'Expanding: one basic extractor feeds one press. More machines need more power.'),
        ('Consulta: livro de receitas (verde), guia da fábrica e guia da vila. /veiotutorial reiniciar refaz.', 'Reference: recipe book (green), factory guide and village guide. /veiotutorial reiniciar restarts.')]),
    ]
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
    plan={0:['generator'],1:['press','cable'],3:['assembler'],4:['drill'],5:['belt'],7:['contract_hub']}
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
    label('veio.onboarding.book','Livro de receitas: no inventário, clique no livro verde. A receita da etapa já está lá; clique nela e a grade se monta sozinha.',
          'Recipe book: in your inventory, click the green book. This step\'s recipe is there; click it and the grid fills itself.')
    label('veio.onboarding.make','Fabrique: %s','Craft: %s')
    label('veio.onboarding.done','Etapa concluída!','Step complete!')
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
