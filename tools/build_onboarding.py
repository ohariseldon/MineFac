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
       [('1. Coloque o gerador no chão, onde vai ficar a sua oficina.', '1. Place the generator where your workshop will be.'),
        ('2. Abra o gerador (clique com a mão vazia) e ponha carvão no espaço de combustível.', '2. Open the generator (click with an empty hand) and put coal in the fuel slot.'),
        ('3. Guarde os cabos: na próxima etapa eles ligam o gerador a uma máquina.', '3. Keep the cables: next step they connect the generator to a machine.')],
       ('Kit recebido: 1 gerador, 16 carvões e 8 cabos. Para fazer mais gerador: fornalha + lingote de ferro + lingote de cobre.', 'Kit received: 1 generator, 16 coal and 8 cables. To craft more generators: furnace + iron ingot + copper ingot.')),
      (('Etapa 2 de 8 · Ligar uma máquina', 'Step 2 of 8 · Power a machine'),
       ('A prensa transforma cobre bruto em chapas, mas só trabalha com energia chegando.', 'The press turns raw copper into plates, but only works with power arriving.'),
       [('1. Coloque a prensa a alguns blocos do gerador.', '1. Place the press a few blocks from the generator.'),
        ('2. Ligue os dois com cabos encostados um no outro: gerador > cabo > cabo > prensa.', '2. Connect them with touching cables: generator > cable > cable > press.'),
        ('3. Veja as marcas da prensa: faixas VERDES recebem itens; a moldura VERMELHA (atrás) é por onde a produção sai.', '3. Look at the press marks: GREEN strips take items; the RED frame (back) is where products come out.'),
        ('4. A etapa avança quando a prensa receber energia. Abra-a para ver a barra de energia.', '4. The step advances when the press receives power. Open it to see the power bar.')],
       ('Kit recebido: 1 prensa. Receita: lingote de cobre + lingote de ferro + pistão + pedra lisa.', 'Kit received: 1 press. Recipe: copper ingot + iron ingot + piston + smooth stone.')),
      (('Etapa 3 de 8 · Produzir à mão', 'Step 3 of 8 · Produce by hand'),
       ('Antes de automatizar, veja como uma máquina trabalha: entrada, progresso e saída.', 'Before automating, see how a machine works: input, progress and output.'),
       [('1. Abra a prensa com a mão vazia.', '1. Open the press with an empty hand.'),
        ('2. Ponha cobre BRUTO no espaço da esquerda (entrada).', '2. Put RAW copper in the left slot (input).'),
        ('3. Espere a seta de progresso encher e retire as chapas da direita (saída).', '3. Wait for the progress arrow and take the plates from the right (output).'),
        ('4. Faça 4 chapas novas. Se a máquina parar, passe o mouse no estado do painel: ele diz o motivo.', '4. Make 4 new plates. If the machine stops, hover over the panel status: it tells you why.')],
       ('Kit recebido: 12 cobres brutos.', 'Kit received: 12 raw copper.')),
      (('Etapa 4 de 8 · Uma cadeia de produção', 'Step 4 of 8 · A production chain'),
       ('Fábricas encadeiam máquinas: a chapa da prensa vira engrenagem no montador.', 'Factories chain machines: the press plate becomes a gear in the assembler.'),
       [('1. Coloque o montador e leve energia até ele (cabo encostado).', '1. Place the assembler and bring power to it (touching cable).'),
        ('2. Ponha 2 chapas na entrada do montador.', '2. Put 2 plates in the assembler input.'),
        ('3. Retire a engrenagem pronta da saída.', '3. Take the finished gear from the output.')],
       ('Kit recebido: 1 montador. Receita: 2 chapas + pistão + bancada.', 'Kit received: 1 assembler. Recipe: 2 plates + piston + crafting table.')),
      (('Etapa 5 de 8 · Extração automática', 'Step 5 of 8 · Automatic extraction'),
       ('Minerar à mão não escala. A extratora tira minério da jazida sozinha, sem acabar.', 'Mining by hand does not scale. The extractor mines the deposit on its own, forever.'),
       [('1. Ache minério de cobre (aparece em encostas e cavernas).', '1. Find copper ore (in hillsides and caves).'),
        ('2. Coloque a extratora EM CIMA do minério, sem quebrá-lo. Veio maior embaixo = mais rápido.', '2. Place the extractor ON TOP of the ore, without mining it. Bigger vein below = faster.'),
        ('3. Leve energia até ela. O cobre aparece na saída (moldura vermelha).', '3. Bring power to it. Copper appears at the output (red frame).')],
       ('Kit recebido: 1 extratora. Receita: 2 lingotes de cobre + 1 de ferro + pistão.', 'Kit received: 1 extractor. Recipe: 2 copper ingots + 1 iron ingot + piston.')),
      (('Etapa 6 de 8 · Esteiras', 'Step 6 of 8 · Belts'),
       ('Agora a extratora alimenta a prensa: o cobre vai sozinho, sem você carregar.', 'Now the extractor feeds the press: copper travels on its own.'),
       [('1. Coloque a primeira esteira encostada na moldura VERMELHA da extratora.', '1. Place the first belt against the extractor RED frame.'),
        ('2. A esteira anda para LONGE de você: coloque cada uma olhando na direção da prensa.', '2. A belt runs AWAY from you: place each one looking toward the press.'),
        ('3. A última esteira entra numa face VERDE da prensa (frente ou lado; nunca na vermelha).', '3. The last belt enters a GREEN face of the press (front or side; never the red one).'),
        ('4. Entregue 2 cobres pela esteira e deixe a prensa fazer 1 chapa com eles.', '4. Deliver 2 copper by belt and let the press make 1 plate from them.')],
       ('Kit recebido: 16 esteiras. Cada esteira leva até 5 minérios ou 1 produto pronto. A chave inverte a direção.', 'Kit received: 16 belts. Each belt carries up to 5 ores or 1 finished product. The wrench reverses it.')),
      (('Etapa 7 de 8 · Saída e estoque', 'Step 7 of 8 · Output and storage'),
       ('Uma linha completa também leva a produção embora. Senão a saída enche e a máquina para.', 'A complete line also carries products away. Otherwise the output fills and the machine stops.'),
       [('1. Coloque uma esteira saindo da moldura VERMELHA da prensa.', '1. Place a belt leaving the press RED frame.'),
        ('2. Leve essa esteira até o baú: ela despeja nele sozinha.', '2. Run that belt into the chest: it unloads by itself.'),
        ('3. Guarde 4 chapas feitas pela linha. Braços, funis e carregadores também valem.', '3. Store 4 plates made by the line. Arms, hoppers and haulers also count.')],
       ('Kit recebido: 1 baú e 8 esteiras.', 'Kit received: 1 chest and 8 belts.')),
      (('Etapa 8 de 8 · A primeira encomenda', 'Step 8 of 8 · The first order'),
       ('A central recebe as encomendas da vila e paga em materiais: é a meta da sua fábrica.', 'The hub takes village orders and pays in materials: it is your factory goal.'),
       [('1. Junte 24 chapas e faça 8 engrenagens no montador (2 chapas cada).', '1. Gather 24 plates and make 8 gears in the assembler (2 plates each).'),
        ('2. Coloque a central: chapas na entrada 1, engrenagens na entrada 2.', '2. Place the hub: plates in input 1, gears in input 2.'),
        ('3. Deixe as saídas livres: a recompensa (64 esteiras e 32 cabos) aparece nelas.', '3. Keep the outputs clear: the reward (64 belts and 32 cables) appears there.'),
        ('4. Dica: ligue a esteira da prensa à central e a encomenda se enche sozinha.', '4. Tip: run the press belt into the hub and the order fills itself.')],
       ('Kit recebido: 1 central. Receita: baú + engrenagem + lingote de ferro + lingote de cobre.', 'Kit received: 1 hub. Recipe: chest + gear + iron ingot + copper ingot.')),
      (('Tutorial concluído!', 'Tutorial complete!'),
       ('Você aprendeu energia, máquinas, cadeias, extração, esteiras e estoque.', 'You learned power, machines, chains, extraction, belts and storage.'),
       [('Próximo: o Quadro da fábrica mostra as fases e as próximas metas.', 'Next: the Factory board shows the phases and next goals.'),
        ('Ampliar: uma extratora comum alimenta uma prensa. Mais máquinas pedem mais energia.', 'Expanding: one basic extractor feeds one press. More machines need more power.'),
        ('Consulta: guia da fábrica e guia da vila. /veiotutorial reiniciar refaz os exercícios.', 'Reference: factory guide and village guide. /veiotutorial reiniciar restarts the exercises.')],
       None),
    ]
    for i,(pt,en) in enumerate(goals): label(f'veio.onboarding.goal.{i}',pt,en)
    for i,(title,why,lines,kit) in enumerate(steps):
        label(f'veio.onboarding.title.{i}',*title)
        label(f'veio.onboarding.why.{i}',*why)
        for n,line in enumerate(lines,1): label(f'veio.onboarding.do.{i}.{n}',*line)
        if kit: label(f'veio.onboarding.kit.{i}',*kit)
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
