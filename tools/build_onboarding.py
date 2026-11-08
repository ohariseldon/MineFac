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
    hints=[
      ('Oficina da vila: precisamos de peças e de uma produção que continue enquanto você explora. Primeiro, coloque um gerador. Receita: fornalha + lingote de ferro + lingote de cobre. O objetivo fica acima da barra de itens. /veiotutorial ocultar esconde; mostrar retoma.',
       'Village workshop: we need parts and production that keeps going while you explore. First, place a generator. Recipe: furnace + iron ingot + copper ingot. Your objective stays above the hotbar. /veiotutorial ocultar hides it; mostrar restores it.'),
      ('Agora faça uma prensa: lingote de cobre + lingote de ferro + pistão + pedra lisa. Ligue gerador > cabo > prensa, encostados. Cabo: cobre + graveto rende 16. Coloque carvão no gerador. O objetivo avança quando a prensa realmente recebe energia.',
       'Now make a press: copper ingot + iron ingot + piston + smooth stone. Connect touching generator > cable > press. Cable: copper + stick makes 16. Fuel the generator with coal. The goal advances when the press actually receives power.'),
      ('Energia confirmada. Abra a prensa com mão vazia: ponha cobre BRUTO na entrada esquerda. Veja o progresso e retire as chapas à direita. Precisamos de 4 ciclos novos; chapas já guardadas não completam esta etapa. Se parar, passe o cursor no estado do painel.',
       'Power confirmed. Open the press with an empty hand: put RAW copper in the left input. Watch progress and collect plates on the right. Complete 4 fresh cycles; stored plates do not complete this goal. If it stops, hover over its panel status.'),
      ('Quatro chapas produzidas! Use 2 chapas + pistão + bancada para fazer o montador. Coloque-o e ligue energia; ponha as outras 2 chapas na entrada. O objetivo pede uma engrenagem realmente produzida. Por enquanto, transporte os materiais à mão.',
       'Four plates produced! Use 2 plates + piston + crafting table to make an assembler. Place and power it; put the other 2 plates in its input. The goal requires a newly produced gear. Carry materials by hand for now.'),
      ('A oficina ampliou o pedido: 24 chapas e 8 engrenagens. Vamos eliminar viagens. Faça uma extratora com 2 cobres + 1 ferro (lingotes) + pistão. Coloque SOBRE minério de cobre e ligue energia. Preserve a jazida. Ela precisa extrair cobre de verdade.',
       'The workshop now needs 24 plates and 8 gears. Let us remove trips. Craft an extractor with 2 copper ingots + 1 iron ingot + piston. Place it ON copper ore and supply power. Keep the vein. It must actually extract copper.'),
      ('Extração funcionando! Abasteça a prensa com esteiras, braços, funis ou um carregador contratado. Você pode partir de um estoque que abasteceu à mão. Só contam cobres que o transporte realmente entregar e ciclos novos da prensa. Exemplo simples: extratora > esteira > prensa. Braços precisam de energia; carregadores, comida e acesso.',
       'Extraction works! Feed the press with belts, arms, hoppers or a hired hauler. A player-stocked input buffer is allowed. Only actual transport deliveries and fresh press cycles count. Simple example: extractor > belt > press. Arms need power; haulers need food and access.'),
      ('A prensa já recebe cobre automaticamente. Agora guarde 4 chapas produzidas com esse abastecimento: use esteiras, braços, funis, carregadores ou misture os meios. Valem baú, barril, funil, celeiro, silo e estoque do posto do carregador. A carga do aldeão só conta ao ser descarregada. Chapas antigas colocadas à mão não contam.',
       'The press is automatically supplied. Now store 4 plates made with that supply using belts, arms, hoppers, haulers or a mixture. Chests, barrels, hoppers, barns, silos and hauler post storage count. Worker cargo counts only after unloading. Old manually added plates do not count.'),
      ('Seu transporte guardou 4 chapas automaticamente! Reserve 24 chapas e use outras 16 para fazer 8 engrenagens. Central: baú + engrenagem + lingote de ferro + lingote de cobre. Na primeira encomenda, ponha chapas na entrada 1 e engrenagens na 2; saídas livres. Recompensa: 64 esteiras + 32 cabos.',
       'Your transport automatically stored 4 plates! Reserve 24 plates and use another 16 for 8 gears. Hub: chest + gear + iron ingot + copper ingot. For its first order put plates in input 1 and gears in input 2; leave outputs clear. Reward: 64 belts + 32 cables.'),
      ('Oficina abastecida! Retire as recompensas da central. Você aprendeu energia, ciclos, fluxo e estoque. Agora escolha o próximo projeto. Observe o motivo de espera antes de ampliar a linha. /veiotutorial reiniciar refaz os exercícios; os livros ficam como consulta opcional.',
       'Workshop supplied! Collect the hub rewards. You learned power, cycles, flow and storage. Choose your next project. Read waiting reasons before expanding. /veiotutorial reiniciar restarts the exercises; books remain optional references.'),
    ]
    for i,(pt,en) in enumerate(goals): label(f'veio.onboarding.goal.{i}',pt,en)
    for i,(pt,en) in enumerate(hints): label(f'veio.onboarding.hint.{i}',pt,en)
    label('veio.onboarding.hidden','Objetivo oculto. /veiotutorial mostrar volta a exibi-lo; sua fábrica continua funcionando.', 'Objective hidden. /veiotutorial mostrar shows it again; your factory keeps working.')
    label('veio.onboarding.shown','Tutorial ativado. Coloque ou abra a máquina que deseja usar nos exercícios.', 'Tutorial enabled. Place or open the machine you want to use for the exercises.')
    label('veio.onboarding.save_failed','Não foi possível salvar o tutorial. O progresso continua nesta sessão; confira o acesso à pasta do mundo.', 'Could not save the tutorial. Progress continues this session; check access to the world folder.')
    label('veio.onboarding.blocked.3','A saída está cheia. Esvazie-a ou confira a direção e o espaço livre no próximo destino.', 'The output is full. Clear it or check direction and available space at the next destination.')
    label('veio.onboarding.blocked.5','A máquina está sem energia. Confira combustível no gerador e cabos encostando nas máquinas.', 'The machine has no power. Check generator fuel and cables touching the machines.')
    label('veio.onboarding.blocked.9','Redstone está pausando a máquina. Retire o sinal para retomar o exercício.', 'Redstone is pausing the machine. Remove the signal to resume the exercise.')
