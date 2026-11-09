"""Player-facing text for the server-observed workshop tutorial."""
def add_labels(label):
    goals=[
      ('1/8 | Ponha o gerador no terreno e abasteça com carvão', '1/8 | Put the generator on the plot and fuel it with coal'),
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
       [('1. O prefeito montou o seu gerador. Para fazer outros, a receita está no livro de receitas (verde).', '1. The mayor built your generator. To make more, the recipe is in the recipe book (green).'),
        ('2. Coloque o gerador no terreno da oficina, o que você planificou.', '2. Place the generator on the workshop plot you flattened.'),
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
       [('Próximo: volte ao prefeito, na varanda da sua casa. Ele conta o que vem a seguir.', 'Next: go back to the mayor, on your house porch. He tells you what comes next.'),
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
      ('mayor.next.1','A primeira encomenda chegou e a vila já fala da sua fábrica. Parabéns!',
       'The first order arrived and the village is already talking about your factory. Congratulations!'),
      ('mayor.next.2','Agora a vila vai pedir mais, e cada pedido maior trava em algum lugar: falta minério, energia, carvão ou esteira.',
       'Now the village will ask for more, and every bigger order gets stuck somewhere: not enough ore, power, coal or belts.'),
      ('mayor.next.3','Faça um Quadro da fábrica (baú, 2 lingotes de cobre e graveto) no terreno: ele mostra a fase, o pedido do dia e onde a fábrica travou.',
       'Make a Factory board (chest, 2 copper ingots and a stick) on the plot: it shows the phase, the daily order and where the factory got stuck.'),
      ('mayor.next.4','Contrate aldeões: com uma esmeralda na mão, clique num posto de trabalho. Eles carregam, colhem e cuidam das máquinas.',
       'Hire villagers: holding an emerald, click a work post. They haul, harvest and look after the machines.'),
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
        ('3. Contrate um aldeão para o gabinete com uma esmeralda (no painel ele aparece como prefeito).', '3. Hire a villager for the office with an emerald (the panel calls them mayor).'),
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
    label('veio.village.start_title', 'Capítulo 2: a vila', 'Chapter 2: the village')
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
      # (título, fala do prefeito, passos, objetivo da barra, comentário ao concluir)
      (('Cidade 1 de 8 · Saneamento', 'Town 1 of 8 · Sanitation'),
       ('Uma cidade começa pela água limpa. Sem saneamento, as famílias adoecem e o IDH não sobe; da fase Vila em diante, casa nova só ganha alvará com um poço a até 24 blocos.',
        'A town starts with clean water. Without sanitation families fall ill and HDI stays low; from the Village phase on, a new house only gets a permit with a well within 24 blocks.'),
       [('1. Fabrique um poço (ficha acima).', '1. Craft a well (card above).'),
        ('2. Coloque-o a até 24 blocos das casas: um poço atende o bairro inteiro.', '2. Place it within 24 blocks of the houses: one well serves the whole district.'),
        ('3. O espaço do poço enche baldes: balde vazio entra, balde de água sai.', '3. The well slot fills buckets: an empty bucket goes in, a water bucket comes out.'),
        ('4. Abra uma casa: a linha de serviços mostra o saneamento.', '4. Open a house: the services line shows sanitation.')],
       ('Uma casa com saneamento (poço a até 24 blocos)', 'A house with sanitation (well within 24 blocks)'),
       ('Água limpa. Agora as famílias precisam onde comprar.', 'Clean water. Now families need somewhere to shop.')),
      (('Cidade 2 de 8 · Comércio', 'Town 2 of 8 · Trade'),
       ('As famílias ganham salário e gastam na loja mais próxima. O comerciante é independente: compra de você no atacado (75% do preço) e vende no varejo.',
        'Families earn wages and spend at the nearest shop. The merchant is independent: buys from you wholesale (75% of the price) and sells retail.'),
       [('1. Fabrique uma loja (ficha acima).', '1. Craft a shop (card above).'),
        ('2. Coloque-a perto das casas (da fase Cidade em diante, casa nova pede loja a até 40 blocos).', '2. Place it near the houses (from the City phase on, a new house needs a shop within 40 blocks).'),
        ('3. Contrate um comerciante com uma esmeralda, como nos postos.', '3. Hire a merchant with an emerald, as with the posts.'),
        ('4. Clique no comerciante: é a tela de troca do Minecraft, com o estoque real e o preço local.', '4. Click the merchant: it is the Minecraft trade screen, with real stock and the local price.')],
       ('Contrate um comerciante na loja', 'Hire a merchant at the shop'),
       ('A loja abriu as portas. Falta mercadoria.', 'The shop is open. It needs goods.')),
      (('Cidade 3 de 8 · A primeira venda', 'Town 3 of 8 · The first sale'),
       ('O que você entrega no estoque o comerciante compra fiado e paga com as vendas. Os preços são da vila: sobem quando falta e caem quando sobra.',
        'What you put in the stock the merchant buys on credit and pays from sales. Prices are local: they rise when goods are short and fall when they pile up.'),
       [('1. Abasteça o estoque da loja com comida (pão, batata assada) e bens (lã, couro, velas).', '1. Stock the shop with food (bread, baked potatoes) and goods (wool, leather, candles).'),
        ('2. Funil e carregador também abastecem: a loja aceita entregas automáticas.', '2. Hoppers and haulers also stock it: the shop takes automatic deliveries.'),
        ('3. De dia, as famílias vêm comprar. A venda paga 4% de imposto na fonte para a prefeitura.', '3. During the day families come to buy. Each sale withholds 4% tax for the town hall.'),
        ('4. Abra a loja para ver o caixa, os preços e o que mais vende.', '4. Open the shop to see the till, the prices and what sells most.')],
       ('Primeira venda da loja (caixa acima de zero)', 'First shop sale (till above zero)'),
       ('O dinheiro começou a girar. Vamos ver de onde ele vem.', 'Money has started to move. Let us see where it comes from.')),
      (('Cidade 4 de 8 · A conta da companhia', 'Town 4 of 8 · The company account'),
       ('A companhia é você: paga os salários dos seus trabalhadores e recebe o atacado das lojas. O dinheiro só entra na vila em esmeraldas de verdade, pelo cofre do centro de controle.',
        'The company is you: it pays your workers\' wages and receives wholesale from the shops. Money only enters the village as real emeralds, through the control center vault.'),
       [('1. Vá ao centro de controle (o painel da sua casa).', '1. Go to the control center (the panel in your house).'),
        ('2. Ponha esmeraldas no cofre: cada uma vira 60 moedas na conta da companhia.', '2. Put emeralds in the vault: each one becomes 60 coins in the company account.'),
        ('3. Sem saldo, os trabalhadores ficam sem salário e as famílias empobrecem.', '3. Without a balance, workers go unpaid and families get poorer.'),
        ('4. Clique na linha da companhia no painel para sacar de volta, se precisar.', '4. Click the company line on the panel to withdraw, if you need to.')],
       ('Esmeraldas no cofre do centro de controle', 'Emeralds in the control center vault'),
       ('Salários garantidos. Agora, conforto nas casas.', 'Wages secured. Now, comfort at home.')),
      (('Cidade 5 de 8 · Energia nas casas', 'Town 5 of 8 · Power at home'),
       ('Energia é serviço público: entra no IDH junto com saneamento, comércio e lazer. Da fase Cidade em diante, casa nova pede um poste a até 12 blocos.',
        'Power is a public service: it counts in HDI with sanitation, trade and leisure. From the City phase on, a new house needs a pole within 12 blocks.'),
       [('1. Fabrique postes (ficha acima; a receita rende 2).', '1. Craft poles (card above; the recipe makes 2).'),
        ('2. Leve uma linha de postes da fábrica até o bairro: eles se ligam sozinhos a até 8 blocos.', '2. Run a pole line from the factory to the district: they link on their own within 8 blocks.'),
        ('3. Um poste a até 12 blocos da casa já conta.', '3. One pole within 12 blocks of the house is enough.')],
       ('Uma casa com energia (poste a até 12 blocos)', 'A house with power (pole within 12 blocks)'),
       ('Luz nas casas. Falta um lugar para descansar.', 'Lights at home. We still need a place to relax.')),
      (('Cidade 6 de 8 · Lazer', 'Town 6 of 8 · Leisure'),
       ('Gente que só trabalha não é feliz. Parque, praça e monumento dão lazer a 32 blocos: é o quarto serviço do IDH, e o ânimo da vila sobe nas festas.',
        'People who only work are not happy. Parks, plazas and monuments give leisure within 32 blocks: the fourth HDI service, and village morale rises at festivals.'),
       [('1. Fabrique um parque (ficha acima).', '1. Craft a park (card above).'),
        ('2. Coloque-o a até 32 blocos das casas.', '2. Place it within 32 blocks of the houses.'),
        ('3. Mais tarde eu vou pedir obras cívicas para os bairros sem lazer: a prefeitura paga a companhia.', '3. Later I will ask for civic works for districts without leisure: the town hall pays the company.'),
        ('4. Visitantes também vão chegar pedindo festas. Entregue o pedido no espaço Agenda da prefeitura.', '4. Visitors will also come asking for festivals. Deliver the request in the town hall Agenda slot.')],
       ('Uma casa com lazer (parque a até 32 blocos)', 'A house with leisure (park within 32 blocks)'),
       ('Um parque! Agora precisamos de mais gente.', 'A park! Now we need more people.')),
      (('Cidade 7 de 8 · Bairro', 'Town 7 of 8 · A district'),
       ('Uma vila de verdade tem várias famílias. Cada casa abriga até 4 aldeões; bebês nascem onde há vaga. Casas muito juntas ficam apertadas: espalhe o bairro.',
        'A real village has several families. Each house holds up to 4 villagers; babies are born where there is room. Houses packed too tight are crowded: spread the district.'),
       [('1. Fabrique mais casas (ficha acima).', '1. Craft more houses (card above).'),
        ('2. Coloque-as perto dos aldeões, com até 6 vizinhas a 16 blocos.', '2. Place them near villagers, with at most 6 neighbours within 16 blocks.'),
        ('3. Camas livres fazem a vila crescer; comida na despensa mantém as famílias.', '3. Free beds make the village grow; food in the pantry keeps families going.'),
        ('4. Abra a prefeitura: ela mostra famílias, emprego e IDH da vila.', '4. Open the town hall: it shows families, jobs and the village HDI.')],
       ('Famílias na vila: %s/3', 'Families in the village: %s/3'),
       ('Três famílias! Estamos perto da primeira fase.', 'Three families! We are close to the first phase.')),
      (('Cidade 8 de 8 · Subir de fase', 'Town 8 of 8 · Move up a phase'),
       ('O plano diretor tem quatro fases: Povoado, Vila, Cidade e Metrópole. Para virar Vila: 12 moradores, 3 famílias e IDH 0,40. Eu confiro no fim de cada dia.',
        'The master plan has four phases: Hamlet, Village, City and Metropolis. To become a Village: 12 residents, 3 families and HDI 0.40. I check at the end of each day.'),
       [('1. Abra a prefeitura ou clique com um livro: o relatório diz o que falta.', '1. Open the town hall or click it with a book: the report says what is missing.'),
        ('2. IDH baixo? Comida na despensa, salário em dia, serviços (poço, poste, loja, lazer) e casas sem aperto.', '2. Low HDI? Food in the pantry, wages paid, services (well, pole, shop, leisure) and uncrowded houses.'),
        ('3. Moradores sobem com camas livres e casas com vaga.', '3. Residents grow with free beds and houses with room.'),
        ('4. Na fase Vila, a zona pode ir ao nível 3, a vila a 60 aldeões e 10 casas, e o imposto sobe para 8%.', '4. In the Village phase, zones can reach level 3, the village 60 villagers and 10 houses, and tax rises to 8%.')],
       ('Fase Vila · moradores %s/12 · famílias %s/3 · IDH %s/0,40', 'Village phase · residents %s/12 · families %s/3 · HDI %s/0.40'),
       ('Somos uma Vila! O plano diretor avançou.', 'We are a Village! The master plan moved forward.')),
    ]
    for i, (title, why, lines, goal, done) in enumerate(steps):
        label(f'veio.city.title.{i}', *title)
        label(f'veio.city.why.{i}', *why)
        for n, line in enumerate(lines, 1): label(f'veio.city.do.{i}.{n}', *line)
        label(f'veio.city.goal.{i}', *goal)
        label(f'veio.city.done.{i}', *done)
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
    label('veio.city.intro.1', 'A vila trabalha. Agora vamos fazer dela um lugar bom de morar: água, comércio, luz, lazer e mais famílias.',
          'The village works. Now let us make it a good place to live: water, trade, light, leisure and more families.')
    label('veio.city.intro.2', 'No fim, a vila sobe de fase, e o plano diretor libera mais aldeões, zonas maiores e mais casas.',
          'At the end the village moves up a phase, and the master plan unlocks more villagers, bigger zones and more houses.')
    label('veio.city.intro.3', 'Começamos pela água.', 'We start with water.')
    label('veio.city.bar', 'Cidade %s/8 | %s', 'Town %s/8 | %s')
    label('veio.city.start_title', 'Capítulo 3: a cidade', 'Chapter 3: the town')
    label('veio.city.start_sub', 'Serviços, comércio, dinheiro e a primeira fase', 'Services, trade, money and the first phase')
    label('veio.city.end_title', 'Uma cidade de verdade', 'A real town')
    label('veio.city.end_sub', 'Tutorial da cidade concluído', 'Town tutorial complete')
    label('veio.city.end.1', 'Água, comércio, salário, luz, lazer e famílias: você sabe fazer uma cidade.',
          'Water, trade, wages, light, leisure and families: you know how to build a town.')
    label('veio.city.end.2', 'Para Cidade: 36 moradores, 9 famílias, IDH 0,55, PIB de 400 por dia e 60% de emprego. Para Metrópole: 80 moradores, 3 bairros e Gini até 0,45.',
          'For City: 36 residents, 9 families, HDI 0.55, GDP of 400 a day and 60% employment. For Metropolis: 80 residents, 3 districts and Gini up to 0.45.')
    label('veio.city.end.3', 'Com o porto ou a estação de carga, a vila vende para o mundo: contratos pagam mais e abrem mercados novos.',
          'With the cargo port or station, the village sells to the world: contracts pay more and open new markets.')
    label('veio.city.end.4', 'Eu sigo na prefeitura. Clique em mim quando quiser saber o próximo passo da fábrica.',
          'I will be at the town hall. Click me whenever you want the factory\'s next step.')
