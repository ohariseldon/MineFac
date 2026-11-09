import json
from pathlib import Path
from collections import Counter
root=Path(__file__).resolve().parents[1]/"content"; A="assets/veio"; D="data/veio"
def read(path): return json.loads((root/path).read_text(encoding="utf-8"))
def write(path,value):
 p=root/path;p.parent.mkdir(parents=True,exist_ok=True)
 p.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
langs={code:read(f"{A}/lang/{code}.json") for code in ("pt_br","en_us")}
def label(key,pt,en):
 langs["pt_br"][key]=pt;langs["en_us"][key]=en
vanilla={
 "campfire":("Fogueira","Campfire"),"flower_pot":("Vaso de flor","Flower pot"),"bricks":("Tijolos","Bricks"),
 "copper_ingot":("Lingote de cobre","Copper ingot"),"iron_ingot":("Lingote de ferro","Iron ingot"),
 "piston":("Pistão","Piston"),"smooth_stone":("Pedra lisa","Smooth stone"),
 "furnace":("Fornalha","Furnace"),"stick":("Graveto","Stick"),
 "crafting_table":("Bancada","Crafting table"),"redstone":("Redstone","Redstone"),
 "stonecutter":("Cortador de pedra","Stonecutter"),"chest":("Baú","Chest"),
 "coal":("Carvão","Coal"),"glass":("Vidro","Glass"),"gold_ingot":("Lingote de ouro","Gold ingot"),
 "bucket":("Balde","Bucket"),"book":("Livro","Book"),"paper":("Papel","Paper"),
 "iron_hoe":("Enxada de ferro","Iron hoe"),"composter":("Composteira","Composter"),
 "lightning_rod":("Para-raios","Lightning rod"),"copper_block":("Bloco de cobre","Block of copper"),
 "grindstone":("Rebolo","Grindstone"),"smoker":("Defumador","Smoker"),"barrel":("Barril","Barrel"),
 "dispenser":("Ejetor","Dispenser"),"white_wool":("Lã branca","White wool"),"iron_axe":("Machado de ferro","Iron axe"),
 "shears":("Tesoura","Shears"),"hay_block":("Fardo de feno","Hay bale"),"glowstone":("Pedra luminosa","Glowstone"),
 "diamond":("Diamante","Diamond"),"emerald":("Esmeralda","Emerald"),
 "hopper":("Funil","Hopper"),"lectern":("Atril","Lectern"),"bell":("Sino","Bell"),"lead":("Laço","Lead"),"oak_fence":("Cerca de carvalho","Oak fence"),"wheat_seeds":("Sementes de trigo","Wheat seeds"),
 "oak_planks":("Tábuas de carvalho","Oak planks"),"oak_sapling":("Muda de carvalho","Oak sapling"),"poppy":("Papoula","Poppy"),"dandelion":("Dente-de-leão","Dandelion"),"oak_fence":("Cerca de carvalho","Oak fence"),"lantern":("Lanterna","Lantern"),"oak_stairs":("Escada de carvalho","Oak stairs"),"gold_block":("Bloco de ouro","Block of gold"),"chiseled_stone_bricks":("Tijolos de pedra talhados","Chiseled stone bricks"),"oak_boat":("Barco de carvalho","Oak boat"),"rail":("Trilho","Rail"),"minecart":("Carrinho","Minecart"),"oak_door":("Porta de carvalho","Oak door"),"glass_pane":("Painel de vidro","Glass pane"),
 "stone_bricks":("Tijolos de pedra","Stone bricks"),"cobblestone":("Pedregulho","Cobblestone"),
}
def item_name(item,code):
 ns,name=item.split(":")
 if ns=="minecraft": return vanilla.get(name,(name,name))[0 if code=="pt_br" else 1]
 for kind in ("block","item"):
  key=f"{kind}.{ns}.{name}"
  if key in langs[code]: return langs[code][key]
 return name

label("veio.contract.optional_beacon", "Farol: estrela no slot 1", "Beacon: star in slot 1")
label("advancement.veio.freight.title", "Entre zonas", "Between zones")
label("advancement.veio.freight.description", "Defina carga e retorno em um terminal de transporte.", "Define dispatch and return cargo at a freight terminal.")
label("advancement.veio.drone_transport.title", "Entrega aérea", "Air delivery")
label("advancement.veio.drone_transport.description", "Conecte dois portos e alimente os drones com energia.", "Connect two ports and power their drones.")
label("block.veio.caravan_station", "Terminal de caravanas", "Caravan terminal")
label("advancement.veio.caravan.title", "Estradas de carga", "Freight roads")
label("advancement.veio.caravan.description", "Vincule uma montaria a uma rota terrestre.", "Assign a mount to a ground freight route.")
label("veio.status.caravan_animal", "Vincule um animal com laço", "Assign an animal with a lead")
label("veio.status.caravan_feed", "Confira a ração do animal", "Check the animal feed")
label("veio.status.caravan_busy", "Animal ocupado ou preso", "Animal occupied or leashed")
label("veio.freight.caravan", "Caravana: ração e caminho livre", "Caravan: feed and a clear road")
label("veio.guide.role.cargo", "Carga", "Cargo")
label("block.veio.rail_terminal", "Terminal ferroviário", "Rail freight terminal")
label("block.veio.drone_port", "Porto de drones", "Drone port")
label("item.veio.route_card", "Cartão de rota", "Route card")
label("veio.status.freight_route", "Confira o par e o canal", "Check pair and channel")
label("veio.freight.rail", "Vagões: use funis", "Minecarts: use hoppers")
label("veio.freight.outbound", "Drone levando a carga", "Drone delivering cargo")
label("veio.freight.return", "Drone retornando", "Drone returning")
label("veio.freight.ready", "Aguardando carga para o drone", "Waiting for drone cargo")
label("veio.freight.route_color", "Rota: %s", "Route: %s")
label("veio.freight.disabled", "desligada", "disabled")
label("veio.freight.capacity", "Carga por viagem: até %s", "Cargo per trip: up to %s")
label("veio.freight.caravan_outbound", "Animal levando a carga", "Animal delivering cargo")
label("veio.freight.caravan_return", "Animal voltando à estação", "Animal returning home")
label("veio.freight.channel", "Canal: %s (0 desliga)", "Channel: %s (0 disables)")
label("veio.guide.role.dispatch", "Enviar", "Dispatch")
label("veio.guide.role.received", "Recebido", "Received")
label("veio.guide.role.inflight", "Voo", "Flight")

vanilla.update({"ender_pearl":("Pérola do Fim","Ender pearl"),"obsidian":("Obsidiana","Obsidian"),
 "glass_pane":vanilla.get("glass_pane",("Painel de vidro","Glass pane")),"book":vanilla.get("book",("Livro","Book")),
 "glass":vanilla.get("glass",("Vidro","Glass"))})
# Every help entry describes the actual slot order and material conversion.
help={
 "generator":(
 "Queima combustível vanilla:\ncarvão, madeira, tábuas,\nbloco de carvão, vara de blaze.\nLigue cabos a cada máquina.",
 "Burns vanilla fuels:\ncoal, logs, planks,\ncoal blocks, blaze rods.\nRun cables to each machine."),
 "drill":(
 "Coloque sobre um minério.\nCobre, ferro, ouro, carvão,\nredstone, lápis e quartzo.\nBocal: saída para a esteira.",
 "Place directly on an ore.\nCopper, iron, gold, coal,\nredstone, lapis and quartz.\nThe port feeds a conveyor."),
 "drill_mk2":(
 "Extrai duas vezes mais rápido.\nPrecisa do dobro de potência.\nSó ela extrai diamante e\nesmeralda, bem devagar.",
 "Extracts twice as fast.\nNeeds twice the power.\nOnly it mines diamond and\nemerald, very slowly."),
 "press":(
 "Cobre/ferro bruto vira chapa.\nUma extratora normal pede duas.\nOuro bruto vira lingote.\nEntrada à esquerda. Use energia.",
 "Raw copper/iron makes plates.\nA normal extractor wants two.\nRaw gold makes ingots.\nInput on the left. Use power."),
 "assembler":(
 "Recebe 2 chapas de cobre.\nProduz 1 engrenagem.\nEntrada à esquerda.\nSaída à direita. Use energia.",
 "Takes 2 copper plates.\nMakes 1 gear.\nInput on the left.\nOutput on the right. Use power."),
 "cutter":(
 "Chapa de ferro vira moldura.\nChapa de cobre: 2 fiações.\nEntrada à esquerda.\nSaída à direita. Use energia.",
 "Iron plates make frames.\nCopper plate: 2 wiring.\nInput on the left.\nOutput on the right. Use power."),
 "integrator":(
 "Entrada 1: moldura.\nEntrada 2: engrenagem.\nProduz módulo mecânico.\nRequer um de cada e energia.",
 "Input 1: frame.\nInput 2: gear.\nMakes a mechanical module.\nNeeds one of each and power."),
 "crusher":(
 "Quartzo vira silício.\nOsso vira 4 farinhas de osso.\nPedregulho vira cascalho e\ncascalho vira areia.",
 "Quartz makes silicon.\nA bone makes 4 bone meal.\nCobblestone makes gravel,\ngravel makes sand."),
 "alloy_furnace":(
 "Entrada 1: 2 chapas de ferro.\nEntrada 2: 1 carvão.\nProduz 1 chapa de aço.\nTambém aceita carvão vegetal.\nPrecisa de energia.",
 "Input 1: 2 iron plates.\nInput 2: 1 coal.\nMakes 1 steel plate.\nCharcoal works too.\nRequires electricity."),
 "electronics":(
 "Entrada 1: 2 fiações.\nEntrada 2: 1 silício.\nProduz 1 placa de circuito.\nPrecisa de energia.",
 "Input 1: 2 copper wiring.\nInput 2: 1 silicon wafer.\nMakes 1 circuit board.\nRequires electricity."),
 "geothermal":(
 "Entrada: balde de lava.\nDevolve o balde vazio.\nSaída à direita e pelo bocal.\nEnvia energia aos vizinhos.",
 "Input: lava bucket.\nReturns the empty bucket.\nCollect on the right or port.\nSupplies adjacent machines."),
 "solar":(
 "Gera energia com luz solar.\nColoque sob céu aberto.\nFunciona no Overworld.\nCombine com acumuladores.",
 "Generates power in sunlight.\nNeeds open sky above.\nWorks in the Overworld.\nCombine with accumulators."),
 "battery":(
 "Guarda energia para depois.\nLigue por cabos ao gerador\ne às máquinas.\nA carga aparece no painel.",
 "Stores energy for later.\nConnect cables to generators\nand machines.\nRead its charge in the panel."),
 "cable":(
 "Leva energia aos vizinhos.\nEncoste nas máquinas.\nConecta pelas seis faces.\nFaça ramais para cada máquina.",
 "Transfers energy to neighbors.\nTouch the machines with it.\nConnects on all six faces.\nBranch to each machine."),
 "belt":(
 "Leva itens pelo bocal.\nOlhe para o destino ao colocar.\nMantenha a mesma altura.\nGire curvas com a chave.",
 "Moves items toward its port.\nFace the destination to place.\nKeep endpoints at one height.\nTurn corners with the wrench."),
 "fast_belt":(
 "Use como a esteira comum.\nTransporta o dobro de itens.\nÚtil ao juntar várias linhas.\nTem acionamento mecânico.",
 "Use like the basic conveyor.\nCarries twice as many items.\nUseful for merging lines.\nUses mechanical drive."),
 "splitter":(
 "Divide entre saídas laterais.\nAlterna se ambas estão livres.\nDesvia se um lado encher.\nUse a chave para orientar.",
 "Splits into the side outputs.\nAlternates when both are free.\nDiverts if one side fills.\nOrient it with the wrench."),
 "cargo_lift":(
 "Empilhe para subir itens.\nO último entrega pela frente.\n5 itens/s, sem energia.\nAgache + chave: sobe/desce.\nChave: gira a saída.",
 "Stack them to lift items.\nThe last one outputs forward.\n5 items/s, no power.\nSneak + wrench: up/down.\nWrench: turns the outlet."),
 "blueprint_designer":(
 "Prancheta: o quadrado acima.\nPapel: copia a obra.\nAgache + chave: tamanho.\nCopia blocos e ajustes,\nnunca inventários.",
 "Its board: the square above.\nPaper: copies the build.\nSneak + wrench: size.\nCopies blocks and settings,\nnever inventories."),
 "build_marker":(
 "Onde a cópia vai nascer.\nMão vazia: orçamento.\nAgache + chave: constrói\ncom o seu inventário.\nChave: gira a obra.",
 "Where the copy will rise.\nEmpty hand: materials list.\nSneak + wrench: builds from\nyour inventory.\nWrench: turns the build."),
 "factory_board":(
 "Mostra a fase, o que falta\ne o que ela libera.\nEntregue a pilha da fase ou\no pedido do dia na entrada.\nAvisa máquinas paradas.",
 "Shows the phase, what is left\nand what it unlocks.\nDeliver the phase stack or\nthe daily order to the input.\nFlags stalled machines."),
 "contract_hub":(
 "Entradas 1 e 2 à esquerda.\nRecompensas 1 e 2 à direita.\nEntregue o pedido exibido.\nRetire sobras ao mudar pedido.",
 "Inputs 1 and 2 on the left.\nRewards 1 and 2 on the right.\nDeliver the displayed order.\nClear leftovers for the next."),
 "harvester":(
 "Colhe e replanta em 9x9.\nTrigo, cenoura, batata,\nbeterraba, fungo e frutas.\nFarinha de osso acelera.",
 "Harvests and replants 9x9.\nWheat, carrots, potatoes,\nbeetroot, wart and berries.\nBone meal speeds growth."),
 "tesla":(
 "Abate monstros a 10 blocos.\n2.400 FE por disparo.\nÀ noite, use acumuladores.\nRedstone desliga a torre.",
 "Stops monsters within 10.\n2,400 FE per shot.\nAt night, use batteries.\nRedstone turns it off."),
 "pole":(
 "Liga-se sozinho a postes\na até 8 blocos por fio aéreo.\nAlimenta máquinas num raio\nde 2 blocos. 120 FE/t.",
 "Links itself to poles within\n8 blocks by overhead wire.\nPowers machines within\n2 blocks. 120 FE/t."),
 "steel_pole":(
 "Fios de até 16 blocos.\nAlimenta num raio de 3.\n400 FE/t por fio: resolve\no gargalo da madeira.",
 "Wires up to 16 blocks.\nPowers within 3 blocks.\n400 FE/t per wire: fixes\nthe wooden pole bottleneck."),
 "drill_mk3":(
 "Quatro vezes a Mk.I.\nÚnica que extrai detritos\nancestrais. Gasta 30 FE/t.\nVeios puros dobram o ritmo.",
 "Four times the Mk.I.\nThe only one that mines\nancient debris. 30 FE/t.\nPure veins double the pace."),
 "electric_furnace":(
 "Funde minérios, areia,\npedra, argila e troncos.\nAssa carnes e batatas.\n2,5x a fornalha comum.",
 "Smelts ores, sand, stone,\nclay and logs.\nCooks meat and potatoes.\n2.5x a regular furnace."),
 "mill":(
 "Trigo vira farinha.\nCana vira 2 de açúcar.\nPlantas e troncos viram\nbiomassa.",
 "Wheat becomes flour.\nSugar cane gives 2 sugar.\nPlants and logs become\nbiomass."),
 "kitchen":(
 "Dois ingredientes.\nPão, biscoito, bolo, torta,\nsopa e cenoura dourada.\nVeja as receitas no guia.",
 "Two ingredients.\nBread, cookies, cake, pie,\nsoup and golden carrots.\nSee recipes in the guide."),
 "pantry":(
 "Compra comida da fábrica\ne paga esmeraldas.\nComida pronta vale mais;\num povoado dá +50%.",
 "Buys the factory's food\nand pays in emeralds.\nCooked food is worth more;\na village adds 50%."),
 "biomass_generator":(
 "Queima biomassa, plantas\ne feno. 40 FE/t, polui menos\nque o carvão. Ligue à\nlavoura e ao moinho.",
 "Burns biomass, plants\nand hay. 40 FE/t, less\npollution than coal. Feed\nit from farms and the mill."),
 "steam_engine":(
 "Precisa de água parada ao\nlado. 120 FE/t e o dobro\nde energia por combustível.\nUsa os do gerador.",
 "Needs still water beside\nit. 120 FE/t and double\nthe energy per fuel.\nTakes generator fuels."),
 "water_wheel":(
 "Encoste em água corrente.\n8 FE/t por lado com fluxo,\naté 32. Não polui e não\nusa combustível.",
 "Place by flowing water.\n8 FE/t per side with flow,\nup to 32. No pollution,\nno fuel."),
 "wind_turbine":(
 "Céu aberto acima.\n8 FE/t no nível do mar;\ny 110 dá 16. Chuva e\ntempestade ajudam.",
 "Needs open sky.\n8 FE/t at sea level;\ny 110 gives 16. Rain and\nstorms help."),
 "grow_lamp":(
 "Instale acima da lavoura.\nAdianta um estágio a cada\n4 ticks, 50 FE cada.\nTambém ilumina a noite.",
 "Place above your crops.\nAdvances a stage every\n4 ticks, 50 FE each.\nAlso lights the night."),
 "planter":(
 "Sementes no slot. Planta\nem terra arada vazia num\n9x9. Fungo do Nether vai\nem areia das almas.",
 "Seeds in the slot. Plants\non empty farmland in 9x9.\nNether wart goes on\nsoul sand."),
 "tree_farm":(
 "Mudas no slot. Planta a\ncada 3 blocos em volta e\ncorta árvores crescidas.\nReplanta a mesma espécie.",
 "Saplings in the slot.\nPlants every 3 blocks and\nfells grown trees.\nReplants the same kind."),
 "ranch":(
 "Ração: reproduz até 10 por\nespécie. Tosquia ovelhas,\nordenha com baldes e\nrecolhe ovos e penas.",
 "Feed: breeds up to 10 per\nkind. Shears sheep, milks\nwith buckets and picks up\neggs and feathers."),
 "wrench":(
 "Selecione na barra rápida.\nBotão direito gira máquinas\ne esteiras.\nConserva estoques e progresso.",
 "Select it in the hotbar.\nRight-click to turn machines\nand conveyors.\nKeeps contents and progress."),
}
label("block.veio.merger", "Unificador de esteiras", "Conveyor Merger")
label("veio.guide.role.left", "Ent. A", "In A")
label("veio.guide.role.right", "Ent. B", "In B")
label("veio.hint.merger", "Duas laterais > uma frente", "Two sides > one front")
label("veio.hint.water_wheel", "8 FE/t por lado com água corrente", "8 FE/t per side with flowing water")
label("veio.hint.wind", "8 FE/t no nível do mar; mais alto e na chuva", "8 FE/t at sea level; more up high and in rain")
label("veio.tutorial.merger.title", "Duas linhas, uma saída", "Two lines, one outlet")
label("veio.tutorial.merger.description", "Conecte as entradas A e B pelas laterais. Consulte Logística no guia.", "Connect inputs A and B from the sides. See Logistics in the guide.")
label("block.veio.inserter", "Braço de transferência", "Transfer Arm")
label("veio.hint.inserter", "Traseira > frente: 40 FE/item", "Rear > front: 40 FE/item")
label("veio.hint.inserter_wait", "Confira origem e destino", "Check source and target")
label("veio.hint.fast_inserter", "Traseira > frente: 64 FE/item", "Rear > front: 64 FE/item")
label("veio.hint.long_inserter", "Alcance 2; 60 FE/item", "Reach 2; 60 FE/item")
help["inserter"] = (
 "Puxa de trás, entrega à frente: baús, esteiras e máquinas.\nAté 1 item/s; 40 FE por item.\nCabo ou poste. Redstone pausa; chave gira.",
 "Takes from rear, sends ahead: chests, belts and machines.\nUp to 1 item/s; 40 FE/item.\nCable or pole. Redstone pauses; wrench turns.")
label("block.veio.fast_inserter", "Braço rápido", "Fast Transfer Arm")
label("block.veio.long_inserter", "Braço longo", "Long Transfer Arm")
help["fast_inserter"] = (
 "Até 2,5 itens/s; 64 FE/item.\nDe trás para a frente, 1 bloco.\nAgachar + item: filtro.\nAgachar + mão vazia: limpar.",
 "Up to 2.5 items/s; 64 FE/item.\nRear to front, 1-block reach.\nSneak + item: set filter.\nSneak + empty hand: clear.")
help["long_inserter"] = (
 "Até 1 item/s; 60 FE/item.\nAlcance 2, pula o bloco do meio.\nAgachar + item: filtro.\nAgachar + mão vazia: limpar.",
 "Up to 1 item/s; 60 FE/item.\nReach 2, skips the middle block.\nSneak + item: set filter.\nSneak + empty hand: clear.")
for key, pt, en in [
 ("balanced", "Distribuição equilibrada", "Balanced routing"),
 ("left", "Prioridade: esquerda / A", "Priority: left / A"),
 ("right", "Prioridade: direita / B", "Priority: right / B"),
 ("sorted", "Filtro: igual à esquerda", "Sort: matching goes left"),
 ("filtered", "Filtro ativo: ícone no bloco", "Filter set: icon on block"),
 ("any", "Filtro: qualquer item", "Filter: any item"),
]: label("veio.routing."+key, pt, en)
for name, pt, en in [
 ("belt", "Esteira", "Conveyor"), ("fast_belt", "Esteira rápida", "Fast conveyor"),
 ("splitter", "Divisor", "Splitter"), ("merger", "Unificador", "Merger"),
 ("inserter", "Braço básico", "Basic arm"), ("fast_inserter", "Braço rápido", "Fast arm"),
 ("long_inserter", "Braço longo", "Long arm"),
]: label("veio.guide.short."+name, pt, en)
help["merger"] = (
 "Une as duas entradas laterais.\nSaída única pela frente.\nAlterna A e B, até 5 itens/s.\nSem energia elétrica.\nUse a chave para girar.",
 "Merges the two side inputs.\nSingle outlet at the front.\nAlternates A/B, up to 5/s.\nNeeds no electricity.\nTurn with the wrench.")
# Trabalhadores (frente Cloud): posto agrícola com aldeão contratado.
label("block.veio.farm_post", "Posto do lavrador", "Farmhand Post")
label("block.veio.barracks", "Quartel", "Barracks")
help["hauler_post"] = (
 "Esmeralda: contrata aldeão.\nAbastece máquinas a até 6\nblocos com os insumos e\nrecolhe os produtos delas.\nNão mexe em esteiras.",
 "Emerald: hires a villager.\nFeeds machines within 6\nblocks from the supplies\nand collects their products.\nLeaves belts alone.")
help["shepherd_post"] = (
 "Esmeralda: contrata aldeão.\nRecolhe ovos, penas e lã,\ntosquia ovelhas e dá ração\naos pares, a até 6 blocos.\nNão ordenha nem abate.",
 "Emerald: hires a villager.\nPicks up eggs, feathers and\nwool, shears sheep and feeds\npairs within 6 blocks.\nNo milking or culling.")
help["field_marker"] = (
 "Centro da lavoura: 9x9 no nível 1, até 21x21. Lavradores perto aram, plantam e colhem.\nSemente no slot: só ela.\nEsmeralda expande; chave escolhe.",
 "Field center: 9x9 at level 1, up to 21x21. Nearby farmhands till, plant and harvest.\nSeed in the slot: only that.\nEmerald expands; wrench sizes.")
help["pen_marker"] = (
 "Centro do cercado: 9x9 a\n21x21. O pastor perto cuida\ndos animais dentro dele.\nRação no slot: só ela e as\nespécies que a comem.",
 "Pen center: 9x9 to 21x21.\nA nearby shepherd tends the\nanimals inside it.\nFeed in the slot: only that\nand the kinds that eat it.")
help["barn"] = (
 "27 espaços para qualquer item.\nA fila do capataz guarda\nsobras aqui e tira daqui\npara as máquinas. Funis\npõem e tiram.",
 "27 slots for any item.\nThe foreman's queue stores\nsurplus here and feeds\nmachines from it. Hoppers\nwork too.")
help["silo"] = (
 "18 espaços: grãos, sementes,\nração e comida. Também faz\nparte da fila do capataz.",
 "18 slots: grain, seeds, feed\nand food. Also part of the\nforeman's queue.")
help["control_center"] = (
 "Sede da companhia e da vila:\nresumo, pendências, obras,\npedidos e empresas (setas).\nCofre, saque, obra, decreto.\nLivro no centro: relatório.",
 "Company and village HQ:\nsummary, issues, projects,\nrequests, businesses (arrows).\nVault, cash out, build, decree.\nBook on the center: report.")
help["home"] = (
 "Sua casa na vila. Lenha:\nlareira; comida: despensa.\nTigela: refeição em casa.\nMelhorias com itens da fábrica.\nVocê renasce aqui.",
 "Your house in the village.\nFuel: hearth; food: pantry.\nBowl: eat at home.\nUpgrade with factory goods.\nYou respawn here.")
help["site"] = (
 "Canteiro: o prédio da obra\nsai do papel em volta dele.\nChave: trocar a planta.\nEsmeralda: mestre de obras.\nComida no slot.",
 "Site: the project's building\nrises around it.\nWrench: switch blueprint.\nEmerald: master builder.\nFood in the slot.")
help["foreman_desk"] = (
 "Esmeralda: contrata o capataz.\nCuida de até 6 postos a\naté 12 blocos: leva comida\ne recontrata com esmeraldas\nda mesa. Monta a fila.",
 "Emerald: hires the foreman.\nManages up to 6 posts within\n12 blocks: brings food and\nrehires with the desk's\nemeralds. Queues transfers.")
help["barracks"] = (
 "Esmeralda: contrata o aldeão.\nAgachar com esmeralda troca\na classe: milícia, guarda,\narqueiro ou mago. Treinar\ngasta a esmeralda do slot.\nComida: cada golpe é uma tarefa.",
 "Emerald: hires the villager.\nSneak with an emerald to\ncycle militia, guard, archer\nor mage. Training spends the\nemerald in the slot.\nFood: each blow is one task.")
# Sociedade (0.18): casas, lojas, prefeitura e poço.
help["house"] = (
 "Lar de até 4 aldeões a até 10\nblocos. Recebe o salário dos\nque trabalham, come da\ndespensa e compra na loja.\nO prefeito dá o alvará.",
 "Home for up to 4 villagers\nwithin 10 blocks. Gets the\nworkers' wages, eats from the\npantry and buys at the shop.\nThe mayor grants the permit.")
help["shop"] = (
 "Esmeralda: contrata o\ncomerciante (independente).\nO que você entrega ele compra\nno atacado (75%) e paga com\nas vendas. Imposto na fonte.",
 "Emerald: hires the merchant\n(independent). What you\ndeliver they buy wholesale\n(75%) and pay from sales.\nTax withheld at source.")
help["town_hall"] = (
 "Esmeralda: contrata o\nprefeito. Vive do imposto na\nfonte (4% a 16%); paga o\nprefeito e a assistência.\nMede IDH e libera as fases.",
 "Emerald: hires the mayor.\nLives on tax at source (4% to\n16%); pays the mayor and\nassistance. Measures HDI and\nunlocks phases.")
help["well"] = (
 "Saneamento das casas a até\n24 blocos. Balde vazio no\nslot vira balde d'água.",
 "Sanitation for homes within\n24 blocks. An empty bucket\nin the slot fills with water.")
help["export_dock"] = (
 "Conexão externa no mar ou\nrio largo. Um navio a cada\n2 min leva 9 pilhas; frete\nbarato. Exporta e importa.\nAba Comércio exterior.",
 "Outside connection on the sea\nor a wide river. A ship every\n2 min takes 9 stacks; cheap\nfreight. Exports and imports.\nForeign trade tab.")
help["freight_station"] = (
 "Conexão externa na fronteira\n(96 blocos da sede). Trem a\ncada minuto, 6 pilhas, frete\nmais caro. Exporta e importa.\nAba Comércio exterior.",
 "Outside connection at the\nfrontier (96 blocks away).\nA train every minute, 6\nstacks, pricier freight.\nForeign trade tab.")
help["park"] = (
 "Lazer para as casas a até\n32 blocos. Os visitantes\npedem parques nos bairros\nsem lazer; o prefeito paga\na obra à companhia.",
 "Leisure for homes within\n32 blocks. Visitors ask for\nparks in districts without\nleisure; the mayor pays the\ncompany for the work.")
help["plaza"] = (
 "Praça com coreto: lazer a\n32 blocos e o lugar onde a\nvila se reúne nos feriados.\nPedida pela arquiteta.",
 "Plaza with a bandstand:\nleisure within 32 blocks and\nwhere the village gathers on\nholidays. The architect asks.")
help["monument"] = (
 "Monumento: lazer a 32\nblocos e mais ânimo. Pedido\npelo escultor; a prefeitura\npaga a obra mais cara.",
 "Monument: leisure within 32\nblocks and more morale. The\nsculptor asks; the town hall\npays the priciest work.")
help["farm_post"] = (
 "Esmeralda: contrata um aldeão adulto livre por perto.\nEle colhe o 9x9 e traz ao posto; 2 pontos de comida por colheita.\nAgachar + chave: dispensa.",
 "Emerald: hires a free adult villager nearby.\nHarvests the 9x9 and brings it back; 2 food points per harvest.\nSneak + wrench: dismiss.")
for key, pt, en in [
 ("veio.status.no_worker", "Sem trabalhador", "No worker"),
 ("veio.status.absent", "Trabalhador ausente", "Worker missing"),
 ("veio.status.no_task", "Nada maduro", "Nothing ripe"),
 ("veio.status.no_food", "Trabalhador sem comida", "Worker has no food"),
 ("veio.status.path", "Caminho obstruído", "Path blocked"),
 ("veio.status.walking", "Trabalhador a caminho", "Worker on the way"),
 ("veio.metric.tasks", "Colheitas feitas: %s", "Harvests done: %s"),
 ("veio.metric.ration", "Comida: %s tarefas", "Food: %s tasks"),
 ("veio.hint.farm_post", "Esmeralda: contratar aldeão", "Emerald: hire a villager"),
 ("veio.hint.barracks.militia", "Milícia: rápida, 7 de dano", "Militia: quick, 7 damage"),
 ("veio.hint.barracks.guard", "Guarda: tanque, se cura a cada golpe", "Guard: tank, heals on every blow"),
 ("veio.hint.barracks.archer", "Arqueiro: atira de longe", "Archer: shoots from far away"),
 ("veio.hint.barracks.mage", "Mago: 10 de dano e fere outro", "Mage: 10 damage and wounds another"),
 ("veio.barracks.militia", "Milícia pronta.", "Militia ready."),
 ("veio.barracks.guard", "Guarda pronto. Ele aguenta a linha de frente.", "Guard ready. They hold the front line."),
 ("veio.barracks.archer", "Arqueiro pronto.", "Archer ready."),
 ("veio.barracks.mage", "Mago pronto.", "Mage ready."),
 ("veio.barracks.need_emerald", "Uma esmeralda no quartel treina a classe.", "An emerald in the barracks trains the class."),
 ("veio.worker.hired", "Aldeão contratado. Ele vai colher as plantas maduras em volta.", "Villager hired. They will harvest the ripe crops around."),
 ("veio.worker.staffed", "Este posto já tem um trabalhador.", "This post already has a worker."),
 ("veio.worker.none", "Ninguém livre a até 8 blocos. Clique, com a esmeralda, no aldeão que quiser (30 s).", "No one free within 8 blocks. Click the villager you want with the emerald (30 s)."),
 ("veio.worker.dismissed", "Trabalhador dispensado. Ele volta à vida de aldeão.", "Worker dismissed. They return to village life."),
 ("advancement.veio.workforce.title", "Mão de obra", "Hired Hands"),
 ("advancement.veio.workforce.description", "Contrate um aldeão para colher a lavoura.", "Hire a villager to harvest your fields."),
 # Segunda entrega: indústria e coordenação.
 ("block.veio.hauler_post", "Posto do carregador", "Hauler Post"),
 ("block.veio.foreman_desk", "Mesa do capataz", "Foreman Desk"),
 ("veio.status.idle_work", "Nada a fazer", "Nothing to do"),
 ("veio.status.no_wage", "Sem esmeralda para contratar", "No emerald to hire"),
 ("veio.metric.trips", "Viagens feitas: %s", "Trips done: %s"),
 ("veio.metric.posts", "Postos: %s · fila: %s", "Posts: %s · queue: %s"),
 # Quarta entrega: zonas, armazenagem e centro de controle.
 ("block.veio.field_marker", "Marco de lavoura", "Field Marker"),
 ("veio.status.zone", "Zona ativa", "Zone active"),
 ("veio.status.storing", "Guardando", "Storing"),
 ("veio.status.resting", "Descansando (noite)", "Resting (night)"),
 ("block.veio.pen_marker", "Marco de cercado", "Pen Marker"),
 ("block.veio.barn", "Celeiro", "Barn"),
 ("block.veio.silo", "Silo", "Silo"),
 ("block.veio.control_center", "Centro de controle", "Control Center"),
 ("veio.zone.size", "Nível %s de %s · área %sx%s", "Level %s of %s · area %sx%s"),
 ("veio.zone.next", "Próximo: %s esm. · %s/%s · aldeões %s/%s", "Next: %s emer. · %s/%s · villagers %s/%s"),
 ("veio.zone.max", "Nível máximo liberado", "Highest level unlocked"),
 ("veio.zone.level1", "Zona em 9x9", "Zone set to 9x9"),
 ("veio.zone.level2", "Zona em 13x13", "Zone set to 13x13"),
 ("veio.zone.level3", "Zona em 17x17", "Zone set to 17x17"),
 ("veio.zone.level4", "Zona em 21x21", "Zone set to 21x21"),
 ("veio.zone.need_work", "Falta trabalho feito na zona para o próximo nível.", "The zone needs more work done for the next level."),
 ("veio.zone.need_villagers", "A vila precisa de mais aldeões para o próximo nível.", "The village needs more villagers for the next level."),
 ("veio.zone.need_emeralds", "Faltam esmeraldas para o próximo nível.", "Not enough emeralds for the next level."),
 ("veio.hint.field_marker", "Esmeralda expande · chave: tamanho", "Emerald expands · wrench: size"),
 ("veio.hint.pen_marker", "Ração no slot define o cercado", "Feed in the slot sets the pen"),
 ("veio.hint.barn", "Sobras da fila e estoque", "Queue leftovers and stock"),
 ("veio.hint.silo", "Grãos, sementes, ração, comida", "Grain, seeds, feed, food"),
 ("veio.metric.storage", "Espaços usados: %s de %s", "Slots used: %s of %s"),
 ("veio.center.workers", "Trabalhadores: %s · ativos: %s", "Workers: %s · active: %s"),
 ("veio.center.waiting", "Esperando: %s · sem comida: %s", "Waiting: %s · no food: %s"),
 ("veio.center.zones", "Lavouras: %s · cercados: %s", "Fields: %s · pens: %s"),
 ("veio.center.storage", "Armazéns: %s%% · fila: %s", "Storage: %s%% · queue: %s"),
 ("veio.center.no_storage", "Sem armazéns · fila: %s", "No storage · queue: %s"),
 ("veio.center.goal_empty", "meta", "goal"),
 ("veio.center.need_admin", "Contrate um administrador e deixe-o no centro para ver o relatório.", "Hire an administrator and keep them at the center to see the report."),
 ("veio.report.title", "Relatório da vila: %s postos, %s trabalhando.", "Village report: %s posts, %s working."),
 ("veio.report.issue.blocked", "Saída cheia: posto em %s", "Output full: post at %s"),
 ("veio.report.issue.no_worker", "Sem trabalhador: posto em %s", "No worker: post at %s"),
 ("veio.report.issue.absent", "Trabalhador ausente: posto em %s", "Worker missing: post at %s"),
 ("veio.report.issue.no_food", "Sem comida: posto em %s", "No food: post at %s"),
 ("veio.report.issue.path", "Caminho obstruído: posto em %s", "Path blocked: post at %s"),
 ("veio.report.issue.no_wage", "Sem esmeralda para contratar: mesa em %s", "No emerald to hire: desk at %s"),
 ("veio.report.issue.idle", "Parado: posto em %s", "Stopped: post at %s"),
 ("veio.report.field", "Lavoura em %s: nível %s de %s (%sx%s)", "Field at %s: level %s of %s (%sx%s)"),
 ("veio.report.pen", "Cercado em %s: nível %s de %s (%sx%s)", "Pen at %s: level %s of %s (%sx%s)"),
 ("veio.report.next", "  Próximo nível: %s esmeraldas · trabalho %s/%s · aldeões %s/%s", "  Next level: %s emeralds · work %s/%s · villagers %s/%s"),
 ("veio.report.storage", "Armazéns: %s, com %s de %s espaços usados.", "Storage: %s, %s of %s slots used."),
 ("veio.report.goal", "Meta %s (%s): %s de %s", "Goal %s (%s): %s of %s"),
 ("advancement.veio.zoning.title", "Terra demarcada", "Marked Land"),
 ("advancement.veio.zoning.description", "Demarque uma lavoura para seus lavradores plantarem e colherem.", "Mark a field for your farmhands to plant and harvest."),
 ("advancement.veio.granary.title", "Celeiro cheio", "Full Barn"),
 ("advancement.veio.granary.description", "Guarde as sobras da fazenda num celeiro ou silo.", "Store the farm's surplus in a barn or silo."),
 ("advancement.veio.headquarters.title", "Centro de controle", "Headquarters"),
 ("advancement.veio.headquarters.description", "Acompanhe toda a vila operária num só painel.", "Watch the whole worker village from one panel."),
 # Terceira entrega: fila de tarefas e pastor.
 ("block.veio.shepherd_post", "Posto do pastor", "Shepherd Post"),
 ("veio.hint.shepherd_post", "Ração, tosquia e coleta", "Feed, shearing and pickup"),
 ("veio.metric.chores", "Tarefas feitas: %s", "Chores done: %s"),
 ("advancement.veio.herding.title", "Pastoreio", "Herding"),
 ("advancement.veio.herding.description", "Um aldeão alimenta, tosquia e recolhe os produtos dos animais.", "A villager feeds and shears animals and picks up their products."),
 ("veio.hint.hauler_post", "Abastece e recolhe máquinas", "Feeds and empties machines"),
 ("veio.hint.foreman_desk", "Comida e contratos dos postos", "Food and hiring for posts"),
 ("veio.guide.role.supply", "Insumos", "Supplies"),
 ("veio.guide.role.wage", "Esmer.", "Emerald"),
 ("advancement.veio.hauling.title", "Carregadores", "Haulers"),
 ("advancement.veio.hauling.description", "Um aldeão abastece as máquinas e recolhe os produtos.", "A villager feeds machines and collects their products."),
 ("advancement.veio.coordination.title", "Capataz", "Foreman"),
 ("advancement.veio.coordination.description", "Um capataz alimenta e recontrata os trabalhadores dos postos.", "A foreman feeds and rehires the workers of your posts."),
]:
 label(key, pt, en)
help["caravan_station"] = (
 "Cor igual liga os terminais.\nLaço vincula animal próximo.\nAgachar + item: o que sai.\nColoque ração no slot de baixo.",
 "Match terminal dye colors.\nLead assigns a nearby animal.\nSneak + item: outgoing cargo.\nPut feed in the bottom slot.")
help["rail_terminal"] = (
 "Corante igual liga a rota.\nAgachar + item: envio.\nItem sem agachar: recebimento.\nFunis ligam os vagões.",
 "Match terminal dye colors.\nSneak + item: outgoing filter.\nItem without sneak: receive.\nHoppers connect minecarts.")
help["drone_port"] = (
 "Ligue com corantes iguais.\n64 blocos; 16 itens/voo.\nEnergia e trajeto livre.",
 "Match port dye colors.\n64-block range, 16 items/flight.\nNeeds power and a clear path.\nSneak + wrench: recall.")
# Água (0.18): bomba, cano, aspersor, turbina hidrelétrica e rios das nascentes.
help.update({
 "pump":(
 "Encoste em água parada.\n25 mB/t por fonte vizinha,\naté 4. Gasta 6 FE/t.\nEmpurra a água aos canos.",
 "Place it by still water.\n25 mB/t per touching source,\nup to 4. Uses 6 FE/t.\nPushes water into pipes."),
 "pipe":(
 "Leva água da bomba.\nConecta nas seis faces.\nNivela entre canos vizinhos.\nNão precisa de energia.",
 "Carries pump water.\nConnects on all six faces.\nLevels out between pipes.\nNeeds no power."),
 "sprinkler":(
 "No meio da lavoura 9x9,\nna altura das plantas. Molha\na terra e rega cada estágio.\nAdubo no slot: 3x mais rápido.",
 "In the middle of a 9x9 field,\nat crop height. Wets the soil\nand waters every stage.\nFertilizer in the slot: 3x faster."),
 "hydro_turbine":(
 "Na parede da represa: água\natrás, saída livre na frente.\nQueda de 3 a 32 blocos e\nespelho largo. Até 320 FE/t.",
 "In the dam wall: water\nbehind, free outlet in front.\nDrop of 3 to 32 blocks and\na wide lake. Up to 320 FE/t."),
})
for key, pt, en in [
 ("block.veio.pump", "Bomba d'água", "Water Pump"),
 ("block.veio.pipe", "Cano d'água", "Water Pipe"),
 ("block.veio.sprinkler", "Aspersor", "Sprinkler"),
 ("block.veio.hydro_turbine", "Turbina hidrelétrica", "Hydro Turbine"),
 ("veio.water", "Água %s / %s mB", "Water %s / %s mB"),
 ("veio.metric.water", "Água %s de %s mB", "Water %s of %s mB"),
 ("veio.metric.pumping", "Bombeando %s mB/t", "Pumping %s mB/t"),
 ("veio.metric.fertility", "Adubo no solo: %s", "Soil fertility: %s"),
 ("veio.metric.head", "Queda d'água: %s blocos", "Water drop: %s blocks"),
 ("veio.hint.pump", "Água parada ao lado; canos levam", "Still water beside; pipes carry it"),
 ("veio.hint.sprinkler", "Água pelos canos; adubo acelera", "Water by pipe; fertilizer speeds up"),
 ("veio.hint.hydro", "Represa atrás, saída na frente", "Dam behind, outlet in front"),
 ("veio.hint.steam", "Água parada ao lado ou por cano", "Still water beside or by pipe"),
 ("veio.guide.short.hydro_turbine", "Turbina hidro", "Hydro turbine"),
 ("veio.season.0", "Primavera", "Spring"),
 ("veio.season.1", "Verão", "Summer"),
 ("veio.season.2", "Outono", "Autumn"),
 ("veio.season.3", "Inverno", "Winter"),
 ("veio.climate", "%s · %s °C", "%s · %s °C"),
 ("veio.climate.frozen", "%s · %s °C: água congelada", "%s · %s °C: water frozen"),
 ("veio.climate.frost", "%s · %s °C: geada, lavoura parada", "%s · %s °C: frost, crops stopped"),
 ("veio.climate.unknown", "Medindo o clima...", "Reading the weather..."),
 ("advancement.veio.hydro.title", "Força da queda d'água", "Power of the Falls"),
 ("advancement.veio.hydro.description", "Represe um rio e gere energia com a queda.", "Dam a river and make power from the drop."),
]:
 label(key, pt, en)
help["steam_engine"] = (
 "Água parada ao lado ou por\ncano: 5 mB/t. 120 FE/t e o\ndobro de energia por\ncombustível do gerador.",
 "Still water beside it or by\npipe: 5 mB/t. 120 FE/t and\ndouble the energy per\ngenerator fuel.")
for key, pt, en in [
 ('block.veio.house', 'Casa', 'House'),
 ('block.veio.shop', 'Loja', 'Shop'),
 ('block.veio.town_hall', 'Prefeitura', 'Town Hall'),
 ('block.veio.well', 'Poço', 'Well'),
 ('veio.status.housed', 'Família morando', 'Family living'),
 ('veio.status.no_permit', 'Sem alvará', 'No permit'),
 ('veio.status.empty_home', 'Casa vazia', 'Empty house'),
 ('veio.house.residents', 'Moradores: %s de %s · IDH %s', 'Residents: %s of %s · HDI %s'),
 ('veio.house.wallet', 'Carteira: %s moedas · despensa: %s', 'Wallet: %s coins · pantry: %s'),
 ('veio.house.need0', 'Tudo em ordem', 'All good'),
 ('veio.house.need1', 'Fome: abasteça uma loja perto com comida', 'Hunger: stock a nearby shop with food'),
 ('veio.house.need2', 'Renda baixa: mais empregos na família', 'Low income: more jobs in the family'),
 ('veio.house.need3', 'Falta saneamento: poço a até 24 blocos', 'No sanitation: a well within 24 blocks'),
 ('veio.house.need4', 'Falta energia: poste a até 12 blocos', 'No power: a pole within 12 blocks'),
 ('veio.house.need5', 'Falta comércio: loja a até 40 blocos', 'No commerce: a shop within 40 blocks'),
 ('veio.house.need6', 'Ar poluído: fábrica perto demais', 'Polluted air: factory too close'),
 ('veio.house.need7', 'Apertada: espalhe o bairro', 'Crowded: spread the district'),
 ('veio.house.need8', 'Sem alvará do prefeito', 'No permit from the mayor'),
 ('veio.house.need9', 'Sem prefeitura: sem serviços nem assistência', 'No town hall: no services or assistance'),
 ('veio.shop.sales', 'Vendas hoje: %s · caixa: %s moedas', 'Sales today: %s · till: %s coins'),
 ('veio.shop.stock', 'Estoque: %s itens · preços %s%%', 'Stock: %s items · prices %s%%'),
 ('veio.hint.shop', 'Troque com o comerciante para comprar', 'Trade with the merchant to buy'),
 ('veio.hint.well', 'Saneamento a até 24 blocos', 'Sanitation within 24 blocks'),
 ('veio.mayor.phase_name1', 'Fase 1: Povoado', 'Phase 1: Hamlet'),
 ('veio.mayor.phase_name2', 'Fase 2: Vila', 'Phase 2: Village'),
 ('veio.mayor.phase_name3', 'Fase 3: Cidade', 'Phase 3: Town'),
 ('veio.mayor.phase_name4', 'Fase 4: Metrópole', 'Phase 4: City'),
 ('veio.mayor.phase2', 'O prefeito declarou: agora somos uma Vila!', 'The mayor declares: we are now a Village!'),
 ('veio.mayor.phase3', 'O prefeito declarou: agora somos uma Cidade!', 'The mayor declares: we are now a Town!'),
 ('veio.mayor.phase4', 'O prefeito declarou: agora somos uma Metrópole!', 'The mayor declares: we are now a City!'),
 ('veio.mayor.population', 'Pop. %s · famílias %s · emprego %s%%', 'Pop. %s · families %s · jobs %s%%'),
 ('veio.mayor.employment', 'Emprego: %s%% dos adultos', 'Employment: %s%% of adults'),
 ('veio.mayor.idh', 'IDH %s · Gini %s', 'HDI %s · Gini %s'),
 ('veio.mayor.gdp', 'PIB %s/dia · imposto %s%%', 'GDP %s/day · tax %s%%'),
 ('veio.mayor.suspended', 'Licenças suspensas: o bem-estar caiu', 'Permits suspended: well-being dropped'),
 ('veio.mayor.request0', 'O prefeito está satisfeito', 'The mayor is satisfied'),
 ('veio.mayor.request1', '%s famílias com salário atrasado: cofre da companhia', '%s families with overdue wages: company vault'),
 ('veio.mayor.request2', '%s famílias passam fome', '%s families are hungry'),
 ('veio.mayor.request3', '%s aldeões sem casa', '%s villagers without a home'),
 ('veio.mayor.request4', '%s casas sem saneamento (poço)', '%s homes without sanitation (well)'),
 ('veio.mayor.request5', '%s casas sem loja perto', '%s homes without a shop nearby'),
 ('veio.mayor.request6', '%s casas sem energia (poste)', '%s homes without power (pole)'),
 ('veio.mayor.request7', '%s adultos sem emprego', '%s adults without a job'),
 ('veio.mayor.request8', '%s casas com ar poluído', '%s homes with polluted air'),
 ('veio.mayor.request9', '%s casas apertadas: crie bairros', '%s crowded homes: build districts'),
 ('veio.mayor.request10', 'Meta: chegar à fase %s (livro: relatório)', 'Goal: reach phase %s (book: report)'),
 ('veio.mayor.need_mayor', 'Contrate um prefeito na prefeitura para ver o relatório.', 'Hire a mayor at the town hall to see the report.'),
 ('veio.mayor.report.people', 'População %s: %s adultos, %s crianças, %s famílias, %s sem casa.', 'Population %s: %s adults, %s kids, %s families, %s homeless.'),
 ('veio.mayor.report.economy', 'PIB %s/dia (produção %s, comércio %s) · emprego %s%% · custo de vida %s%%.', 'GDP %s/day (production %s, commerce %s) · employment %s%% · cost of living %s%%.'),
 ('veio.mayor.report.idh', 'IDH %s · Gini %s', 'HDI %s · Gini %s'),
 ('veio.mayor.report.district', 'Bairro %s (%s): %s moradores, IDH %s', 'District %s (%s): %s residents, HDI %s'),
 ('veio.mayor.report.next', 'Fase %s pede: %s moradores, %s famílias, IDH %s, PIB %s, emprego %s%%, Gini até %s, %s bairros.', 'Phase %s needs: %s residents, %s families, HDI %s, GDP %s, employment %s%%, Gini up to %s, %s districts.'),
 ('veio.mayor.report.global', 'Global: %s prefeituras, %s pessoas, IDH %s', 'Global: %s town halls, %s people, HDI %s'),
 ('veio.center.zones', 'Lavouras: %s · cercados: %s · companhia: %s', 'Fields: %s · pens: %s · company: %s'),
 ('veio.company.report', 'Companhia: %s moedas no cofre · folha %s/dia · impostos %s/dia.', 'Company: %s coins in the vault · payroll %s/day · taxes %s/day.'),
 ('veio.mayor.public', 'Cofre público %s · dívida %s', 'Public funds %s · debt %s'),
 ('veio.mayor.request11', 'Cofre público vazio: prefeito e poços sem pagamento', 'Public funds empty: mayor and wells unpaid'),
 ('veio.mayor.request12', 'Dívida de impostos da companhia: %s moedas', 'Company tax debt: %s coins'),
 ('veio.mayor.report.public', 'Contas públicas: saldo %s · imposto %s%% (%s/dia) · salários %s/dia · poços %s/dia · assistência %s/dia · dívida da companhia %s.', 'Public accounts: balance %s · tax %s%% (%s/day) · wages %s/day · wells %s/day · assistance %s/day · company debt %s.'),
 ('veio.house.need10', 'Salário atrasado: abasteça o cofre da companhia', 'Wages overdue: fill the company vault'),
 ('veio.holding.tab0', 'Holding: visão geral', 'Holding: overview'),
 ('veio.holding.tab1', 'Mineração', 'Mining'),
 ('veio.holding.tab2', 'Manufatura', 'Manufacturing'),
 ('veio.holding.tab3', 'Energia', 'Power'),
 ('veio.holding.tab4', 'Logística', 'Logistics'),
 ('veio.holding.tab5', 'Agro (fazendas)', 'Agriculture'),
 ('veio.holding.tab6', 'Extrativismo', 'Forestry'),
 ('veio.holding.tab7', 'Urbanização', 'Urban development'),
 ('veio.holding.tab8', 'Comércio', 'Retail'),
 ('veio.holding.tab9', 'Gestão e segurança', 'Management and security'),
 ('veio.holding.units', 'Unidades: %s · ativas: %s', 'Units: %s · active: %s'),
 ('veio.holding.workers', 'Trabalhadores: %s · folha ~%s/dia', 'Workers: %s · payroll ~%s/day'),
 ('veio.holding.made1', 'Minério extraído: %s/dia', 'Ore mined: %s/day'),
 ('veio.holding.made2', 'Peças fabricadas: %s/dia', 'Parts made: %s/day'),
 ('veio.holding.made3', 'Produção registrada: %s/dia', 'Recorded output: %s/day'),
 ('veio.holding.made4', 'Itens movidos: %s/dia', 'Items moved: %s/day'),
 ('veio.holding.made5', 'Colheitas e produtos: %s/dia', 'Harvests and goods: %s/day'),
 ('veio.holding.made6', 'Madeira e folhas: %s/dia', 'Wood and leaves: %s/day'),
 ('veio.holding.made7', 'Moradia: números abaixo', 'Housing: see below'),
 ('veio.holding.made8', 'Vendas: números abaixo', 'Sales: see below'),
 ('veio.holding.made9', 'Tarefas: %s/dia', 'Tasks: %s/day'),
 ('veio.holding.extra1', 'Extratoras sobre jazidas', 'Extractors on ore veins'),
 ('veio.holding.extra2', 'Prensas, montadoras, carregadores', 'Presses, assemblers, haulers'),
 ('veio.holding.extra3', 'Geradores, painéis, baterias, postes', 'Generators, panels, batteries, poles'),
 ('veio.holding.extra4', 'Esteiras, braços, armazéns, capatazes', 'Belts, arms, storage, foremen'),
 ('veio.holding.extra5', 'Lavouras %s · cercados %s · níveis %s', 'Fields %s · pens %s · levels %s'),
 ('veio.holding.extra6', 'Fazendas de árvores', 'Tree farms'),
 ('veio.holding.extra7', 'Casas %s · moradores %s · sem alvará %s', 'Homes %s · residents %s · no permit %s'),
 ('veio.holding.extra8', 'Lojas %s · abertas %s · preços %s%%', 'Shops %s · open %s · prices %s%%'),
 ('veio.holding.extra9', 'Centro, quartel e torres Tesla', 'Center, barracks and Tesla towers'),
 ('veio.holding.stopped', 'Parado mais comum: %s (%s)', 'Most common stop: %s (%s)'),
 ('veio.holding.all_good', 'Nada parado', 'Nothing stopped'),
 ('veio.holding.report', '%s: %s unidades, %s ativas, %s trabalhadores, %s/dia', '%s: %s units, %s active, %s workers, %s/day'),
 ('veio.center.company', 'Companhia: %s moedas (clique: sacar 1 esm.)', 'Company: %s coins (click: cash out 1 emerald)'),
 ('veio.holding.units', 'Unidades %s · ativas %s · trabalhadores %s', 'Units %s · active %s · workers %s'),
 ('veio.holding.profit', 'Receita %s · gastos %s · lucro %s /dia', 'Revenue %s · costs %s · profit %s /day'),
 ('veio.holding.report', '%s: %s unidades, %s ativas, %s trabalhadores, %s/dia · receita %s, gastos %s, lucro %s', '%s: %s units, %s active, %s workers, %s/day · revenue %s, costs %s, profit %s'),
 ('veio.company.report', 'Companhia: %s moedas · receita %s/dia · folha %s/dia.', 'Company: %s coins · revenue %s/day · payroll %s/day.'),
 ('veio.mayor.public', 'Cofre público %s · imposto %s/dia', 'Public funds %s · tax %s/day'),
 ('veio.mayor.report.economy', 'PIB %s/dia (consumo final) · atacado da indústria %s/dia · emprego %s%% · custo de vida %s%%.', 'GDP %s/day (final consumption) · industry wholesale %s/day · employment %s%% · cost of living %s%%.'),
 ('veio.mayor.report.public', 'Contas públicas: saldo %s · imposto na fonte %s%% (%s/dia) · salários %s/dia · assistência %s/dia.', 'Public accounts: balance %s · tax at source %s%% (%s/day) · wages %s/day · assistance %s/day.'),
 ('veio.mayor.report.money', 'Moeda em circulação %s = companhia %s + prefeitura %s + lojas %s + famílias %s. Esmeraldas que entraram: %s; que saíram: %s.', 'Money in circulation %s = company %s + town hall %s + shops %s + families %s. Emeralds in: %s; out: %s.'),
 ('veio.shop.owes', 'Deve à indústria: %s (fiado)', 'Owes the industry: %s (credit)'),
 ('veio.hint.shop', 'Troque com o comerciante para comprar', 'Trade with the merchant to buy'),
 ('block.veio.export_dock', 'Porto de exportação', 'Export Dock'),
 ('veio.status.no_company', 'Sem companhia (centro de controle)', 'No company (control center)'),
 ('veio.dock.today', 'Exportado hoje: %s itens · %s moedas', 'Exported today: %s items · %s coins'),
 ('veio.dock.next', 'Próximo navio em %s s', 'Next ship in %s s'),
 ('veio.hint.export_dock', 'Mais valor agregado, mais lucro', 'More added value, more profit'),
 ('veio.export.best', 'Exportar hoje: %s rende %s moedas por unidade', 'Export today: %s pays %s coins per unit'),
 ('veio.center.company', 'Companhia: %s · exporta %s/dia (clique: sacar)', 'Company: %s · exports %s/day (click: cash out)'),
 ('veio.mayor.report.economy', 'PIB %s/dia = consumo %s + exportação %s · atacado da indústria %s/dia · emprego %s%% · custo de vida %s%%.', 'GDP %s/day = consumption %s + exports %s · industry wholesale %s/day · employment %s%% · cost of living %s%%.'),
 ('veio.mayor.report.money', 'Moeda em circulação %s = companhia %s + prefeitura %s + lojas %s + famílias %s. Entrou %s (exportação %s); saiu %s.', 'Money in circulation %s = company %s + town hall %s + shops %s + families %s. In %s (exports %s); out %s.'),
 ('block.veio.export_dock', 'Porto de carga', 'Cargo Port'),
 ('block.veio.freight_station', 'Estação de carga', 'Freight Station'),
 ('veio.status.no_water', 'Sem mar em volta', 'No sea around'),
 ('veio.status.not_frontier', 'Fora da fronteira', 'Not at the frontier'),
 ('veio.dock.today', 'Hoje: %s itens · líquido %s moedas', 'Today: %s items · net %s coins'),
 ('veio.dock.next', 'Partida em %s s · frete até %s', 'Departs in %s s · freight up to %s'),
 ('veio.hint.export_dock', 'Valor agregado paga mais lá fora', 'Added value pays more abroad'),
 ('veio.hint.freight_station', 'Trem na fronteira: frete maior', 'Frontier train: higher freight'),
 ('veio.holding.tab10', 'Comércio exterior', 'Foreign trade'),
 ('veio.trade.reputation', 'Reputação %s · o mundo compra até a camada %s', 'Reputation %s · the world buys up to tier %s'),
 ('veio.trade.flows', 'Exporta %s/dia · importa %s/dia · saldo %s', 'Exports %s/day · imports %s/day · balance %s'),
 ('veio.trade.contract', '%s: %s/%s · %s dias', '%s: %s/%s · %s days'),
 ('veio.export.best', 'Bolsa hoje: %s paga %s moedas por unidade', 'Market today: %s pays %s coins per unit'),
 ('veio.status.locked_market', 'Mercado fechado (reputação)', 'Market closed (reputation)'),
 ('veio.dock.next', 'Partida em %s s · %s pilhas · frete até %s', 'Departs in %s s · %s stacks · freight up to %s'),
 ('veio.trade.reputation', 'Reputação %s · camada %s aberta · próxima com %s', 'Reputation %s · tier %s open · next at %s'),
 ('veio.trade.reputation_max', 'Reputação %s · todos os mercados abertos', 'Reputation %s · every market open'),
 ('veio.trade.board', 'Bolsa: %s a %s moedas (%s hoje)', 'Market: %s at %s coins (%s today)'),
 ('veio.trade.news.offered', 'Novo contrato: %s %s em %s dias, pagando %s%% a mais. Veja a aba Comércio exterior.', 'New contract: %s %s in %s days, paying %s%% more. See the Foreign trade tab.'),
 ('veio.trade.news.fulfilled', 'Contrato cumprido: %s. Prêmio de %s moedas e +1 de reputação.', 'Contract fulfilled: %s. %s coins premium and +1 reputation.'),
 ('veio.trade.news.lost', 'Contrato perdido: %s. Reputação -1.', 'Contract lost: %s. Reputation -1.'),
 ('veio.trade.news.opened2', 'Mercado aberto: componentes (engrenagens, fios, molduras, silício, aço).', 'Market open: components (gears, wires, frames, silicon, steel).'),
 ('veio.trade.news.opened3', 'Mercado aberto: eletrônicos (circuitos, módulos mecânicos, kits de reparo).', 'Market open: electronics (circuits, mechanical modules, repair kits).'),
 ('veio.trade.news.opened4', 'Mercado aberto: módulos de melhoria.', 'Market open: upgrade modules.'),
 ('veio.trade.departed.ship', 'Navio partiu: %s itens, %s moedas líquidas', 'Ship departed: %s items, %s net coins'),
 ('veio.trade.departed.train', 'Trem partiu: %s itens, %s moedas líquidas', 'Train departed: %s items, %s net coins'),
 ('block.veio.park', 'Parque', 'Park'),
 ('block.veio.plaza', 'Praça', 'Plaza'),
 ('block.veio.monument', 'Monumento', 'Monument'),
 ('entity.veio.visitor', 'Visitante', 'Visitor'),
 ('veio.status.holiday', 'Feriado', 'Holiday'),
 ('veio.agenda.founding.name', 'Dia da Fundação', 'Founding Day'),
 ('veio.agenda.founding.visitor', 'Seu Anselmo, o primeiro morador', 'Old Anselmo, the first settler'),
 ('veio.agenda.founding.hello', 'Prefeito, lembra quando isto aqui era só um punhado de casas?', 'Mayor, remember when this was just a handful of houses?'),
 ('veio.agenda.founding.ask', 'No Dia da Fundação a vila para. Queria bolo, pão e bandeiras para a festa.', "On Founding Day the village stops. I'd like cake, bread and banners for the party."),
 ('veio.agenda.founding.thanks', 'Agora sim, a vila vai comemorar como merece!', 'Now the village will celebrate as it deserves!'),
 ('veio.agenda.flowers.name', 'Festa das Flores', 'Flower Festival'),
 ('veio.agenda.flowers.visitor', 'Dona Margarida, das floristas', 'Margarida, of the florists'),
 ('veio.agenda.flowers.hello', 'Que dia lindo! A primavera pede flores.', 'What a lovely day! Spring calls for flowers.'),
 ('veio.agenda.flowers.ask', 'Para a Festa das Flores preciso de papoulas, dentes-de-leão e um pouco de mel.', 'For the Flower Festival I need poppies, dandelions and some honey.'),
 ('veio.agenda.flowers.thanks', 'Que perfume! A praça vai ficar linda.', 'What a scent! The square will look lovely.'),
 ('veio.agenda.saint_john.name', 'Festa de São João', "Saint John's Festival"),
 ('veio.agenda.saint_john.visitor', 'Seu Benedito, o sanfoneiro', 'Benedito, the accordion player'),
 ('veio.agenda.saint_john.hello', 'Arraiá chegando, prefeito!', 'The festival is coming, mayor!'),
 ('veio.agenda.saint_john.ask', 'Para o São João: batata assada, fogueiras e biscoitos para o povo.', "For Saint John's: baked potatoes, campfires and cookies for everyone."),
 ('veio.agenda.saint_john.thanks', 'Vai ter quadrilha até o sol nascer!', "We'll dance until sunrise!"),
 ('veio.agenda.harvest.name', 'Festa da Colheita', 'Harvest Festival'),
 ('veio.agenda.harvest.visitor', 'Tia Ceres, dos lavradores', 'Aunt Ceres, of the farmers'),
 ('veio.agenda.harvest.hello', 'A colheita foi boa este ano.', 'The harvest was good this year.'),
 ('veio.agenda.harvest.ask', 'Para a Festa da Colheita: trigo, tortas de abóbora e pão.', 'For the Harvest Festival: wheat, pumpkin pies and bread.'),
 ('veio.agenda.harvest.thanks', 'Fartura na mesa de todo mundo!', "Plenty on everyone's table!"),
 ('veio.agenda.lights.name', 'Festa das Luzes', 'Festival of Lights'),
 ('veio.agenda.lights.visitor', 'Mestre Lumen, o vidreiro', 'Master Lumen, the glassmaker'),
 ('veio.agenda.lights.hello', 'O inverno escurece cedo...', 'Winter gets dark early...'),
 ('veio.agenda.lights.ask', 'Para a Festa das Luzes: lanternas, velas e vidro para as luminárias.', 'For the Festival of Lights: lanterns, candles and glass for the lamps.'),
 ('veio.agenda.lights.thanks', 'A vila vai brilhar no frio!', 'The village will shine in the cold!'),
 ('veio.agenda.new_year.name', 'Virada do Ano', "New Year's Eve"),
 ('veio.agenda.new_year.visitor', 'Pequena Aurora', 'Little Aurora'),
 ('veio.agenda.new_year.hello', 'Moço, o ano tá acabando!', 'Hey, the year is almost over!'),
 ('veio.agenda.new_year.ask', 'Na Virada do Ano eu quero fogos, um bolo e biscoitos!', "For New Year's Eve I want fireworks, a cake and cookies!"),
 ('veio.agenda.new_year.thanks', 'Oba! Vai ter fogos!', 'Yay! There will be fireworks!'),
 ('veio.agenda.fair.name', 'Feira de domingo', 'Sunday Fair'),
 ('veio.agenda.fair.visitor', 'Dona Lia, da feira', 'Lia, of the market fair'),
 ('veio.agenda.fair.hello', 'Domingo é dia de feira!', 'Sunday is fair day!'),
 ('veio.agenda.fair.ask', 'Para as barracas: lã, papel e pão.', 'For the stalls: wool, paper and bread.'),
 ('veio.agenda.fair.thanks', 'A feira vai lotar!', 'The fair will be packed!'),
 ('veio.agenda.dance.name', 'Baile de domingo', 'Sunday Dance'),
 ('veio.agenda.dance.visitor', 'Juca, do baile', 'Juca, of the dance hall'),
 ('veio.agenda.dance.hello', 'E aí, prefeito, bora um baile no domingo?', 'Hey mayor, how about a dance on Sunday?'),
 ('veio.agenda.dance.ask', 'Preciso de blocos musicais, lanternas e biscoitos.', 'I need note blocks, lanterns and cookies.'),
 ('veio.agenda.dance.thanks', 'Vai ser o baile do ano!', "It'll be the dance of the year!"),
 ('veio.agenda.fishing.name', 'Torneio de pesca', 'Fishing Contest'),
 ('veio.agenda.fishing.visitor', 'Pescador Tião', 'Tião the fisherman'),
 ('veio.agenda.fishing.hello', 'Os peixes estão mordendo, prefeito.', 'The fish are biting, mayor.'),
 ('veio.agenda.fishing.ask', 'Para o torneio de pesca: varas, bacalhau assado e pão.', 'For the fishing contest: rods, cooked cod and bread.'),
 ('veio.agenda.fishing.thanks', 'Que vença o melhor pescador!', 'May the best angler win!'),
 ('veio.agenda.park.name', 'Parque', 'Park'),
 ('veio.agenda.park.visitor', 'Guarda-parque Silvano', 'Silvano the park ranger'),
 ('veio.agenda.park.hello', 'Este bairro não tem uma árvore para descansar.', 'This district has no tree to rest under.'),
 ('veio.agenda.park.ask', 'Construa um parque (bloco Parque) neste bairro.', 'Build a park (Park block) in this district.'),
 ('veio.agenda.park.thanks', 'Agora as crianças têm onde brincar.', 'Now the children have somewhere to play.'),
 ('veio.agenda.plaza.name', 'Praça', 'Plaza'),
 ('veio.agenda.plaza.visitor', 'Arquiteta Iara', 'Iara the architect'),
 ('veio.agenda.plaza.hello', 'A vila precisa de um ponto de encontro.', 'The village needs a meeting point.'),
 ('veio.agenda.plaza.ask', 'Construa uma praça (bloco Praça) neste bairro.', 'Build a plaza (Plaza block) in this district.'),
 ('veio.agenda.plaza.thanks', 'Uma praça de verdade! A vila vai se encontrar aqui.', 'A real plaza! The village will meet here.'),
 ('veio.agenda.monument.name', 'Monumento', 'Monument'),
 ('veio.agenda.monument.visitor', 'Escultor Bento', 'Bento the sculptor'),
 ('veio.agenda.monument.hello', 'Uma cidade sem monumento não tem memória.', 'A town without a monument has no memory.'),
 ('veio.agenda.monument.ask', 'Erga um monumento (bloco Monumento) neste bairro.', 'Raise a monument (Monument block) in this district.'),
 ('veio.agenda.monument.thanks', 'Ficará para as próximas gerações.', 'It will stand for generations.'),
 ('veio.agenda.says', '[%s] %s', '[%s] %s'),
 ('veio.agenda.need', 'Faltam: %s de %s %s · %s dias', 'Still needed: %s of %s %s · %s days'),
 ('veio.agenda.need_civic', 'Construa um bloco %s no bairro (%s) · %s dias', 'Build a %s block in the district (%s) · %s days'),
 ('veio.agenda.ready', 'Tudo pronto! Faltam %s dias para o evento.', 'All set! The event is in %s days.'),
 ('veio.agenda.arrived', '%s chegou à prefeitura para falar com o prefeito. Clique no visitante para ouvir o pedido.', '%s arrived at the town hall to see the mayor. Click the visitor to hear the request.'),
 ('veio.agenda.prepared', '%s: tudo pronto! A prefeitura reembolsou %s moedas à companhia.', '%s: all set! The town hall refunded %s coins to the company.'),
 ('veio.agenda.civic_done', '%s: obra pronta! A prefeitura pagou %s moedas à companhia.', '%s: the work is done! The town hall paid %s coins to the company.'),
 ('veio.agenda.holiday_great', 'Hoje é %s! A vila para de trabalhar para comemorar.', 'Today is %s! The village stops working to celebrate.'),
 ('veio.agenda.holiday_poor', 'Hoje é %s, mas a festa não foi preparada. A vila para, desanimada.', 'Today is %s, but nothing was prepared. The village stops, disheartened.'),
 ('veio.agenda.sunday_great', 'Domingo de %s! A vila se diverte.', 'Sunday %s! The village has fun.'),
 ('veio.agenda.today', 'Hoje: %s, dia %s · %s', 'Today: %s, day %s · %s'),
 ('veio.agenda.daykind0', 'dia útil', 'workday'),
 ('veio.agenda.daykind1', 'domingo', 'Sunday'),
 ('veio.agenda.daykind2', 'feriado', 'holiday'),
 ('veio.agenda.morale', 'Ânimo da vila: %s/100', 'Village morale: %s/100'),
 ('veio.agenda.next', 'Próximo feriado: %s, em %s dias', 'Next holiday: %s, in %s days'),
 ('veio.agenda.line', '%s: %s/%s %s · %s d', '%s: %s/%s %s · %s d'),
 ('veio.agenda.line_civic', '%s: construir no bairro · %s d', '%s: build in the district · %s d'),
 ('veio.agenda.line_ready', '%s: pronto · %s d', '%s: ready · %s d'),
 ('veio.agenda.amenity', 'Lazer para %s casas', 'Leisure for %s homes'),
 ('veio.hint.amenity', 'Lazer para casas a até 32 blocos', 'Leisure for homes within 32 blocks'),
 ('veio.mayor.request13', '%s casas sem lazer (parque, praça ou monumento)', '%s homes without leisure (park, plaza or monument)'),
 ('veio.zone.need_mayor', 'O plano diretor não permite: suba a fase da vila na prefeitura.', 'The master plan forbids it: raise the village phase at the town hall.'),
 ('veio.zone.suspended', 'Licenças suspensas pelo prefeito: melhore o bem-estar da vila.', "Permits suspended by the mayor: improve the village's well-being."),
 ('veio.guide.role.output', 'Saída', 'Output'),
]:
 label(key, pt, en)

# ---------- Painel executivo, foco, obras da vila e lar (frente Cloud) ----------
for key, pt, en in [
 ("block.veio.home", "Lar", "Home Hearth"),
 ("veio.center.page.0", "Resumo", "Summary"),
 ("veio.center.page.1", "Atenção", "Attention"),
 ("veio.center.page.2", "Obras", "Projects"),
 ("veio.center.page.3", "Pedidos", "Requests"),
 ("veio.center.prestige", "Prestígio: %s (%s/%s)", "Prestige: %s (%s/%s)"),
 ("veio.center.prestige_max", "Prestígio: %s (%s)", "Prestige: %s (%s)"),
 ("veio.center.treasury", "Caixa: %s esm. · decreto: %s", "Treasury: %s em. · decree: %s"),
 ("veio.center.served", "Pedidos atendidos hoje: %s", "Requests served today: %s"),
 ("veio.center.hint.deliver", "Entregas no slot Obra", "Deliver in the Build slot"),
 ("veio.center.hint.cash", "Agachar + esmeralda: sacar", "Sneak + emerald: withdraw"),
 ("veio.center.project.locked", "%s: exige %s", "%s: needs %s"),
 ("veio.center.project.built", "Prontas: %s (cursor: efeitos)", "Built: %s (hover: effects)"),
 ("veio.center.tip.prestige", "Prestígio: pedidos +10, obras +25, festa +10,\nbanquete +5, dia em ordem no lar +2, manhã sem\npendências +3. Cada nível: teto +5, obras novas\ne pedidos mais ricos.", "Prestige: requests +10, projects +25, festival +10,\nbanquet +5, tidy day at home +2, morning with\nno issues +3. Each tier: cap +5, new projects\nand richer requests."),
 ("veio.center.tip.treasury", "O caixa da companhia recebe as recompensas (pagas\npela prefeitura) e as esmeraldas que sobram no slot\nObra. Paga obras e zonas. Agachar + esmeralda: sacar 16.", "The company cash gets rewards (paid by the town\nhall) and leftover emeralds from the Build slot.\nIt pays projects and zones. Sneak + emerald: withdraw 16."),
 ("veio.center.tip.served", "Atendidos hoje. Amanhã, viram pedidos novos.", "Served today. Tomorrow they become new requests."),
 ("veio.request.empty", "(sem pedido)", "(no request)"),
 ("veio.request.line", "%s/%s %s → %s esm.", "%s/%s %s → %s em."),
 ("veio.request.line_done", "Pronto: %s %s (novo amanhã)", "Done: %s %s (new tomorrow)"),
 ("veio.request.tip", "Recompensa: %s esmeraldas (pagas pela prefeitura) e %s de prestígio.", "Reward: %s emeralds (paid by the town hall) and %s prestige."),
 ("veio.request.tip_keep", "Entregue pelo slot Obra. Nada entregue se perde:\no pedido espera até ser atendido.", "Deliver through the Build slot. Nothing delivered\nis lost: the request waits until served."),
 ("veio.request.done", "Pedidos atendidos: %s. A prefeitura pagou %s esmeraldas à companhia.", "Requests served: %s. The town hall paid the company %s emeralds."),
 ("veio.treasury.out", "Sacou %s esmeraldas do caixa.", "Withdrew %s emeralds from the treasury."),
 ("veio.treasury.empty", "O caixa está vazio.", "The treasury is empty."),
 ("veio.prestige.up", "A vila agora é %s! Teto maior, obras e pedidos novos.", "The village is now a %s! Higher cap, new projects and requests."),
 ("veio.prestige.tier.0", "Aldeia", "Hamlet"), ("veio.prestige.tier.1", "Vila", "Village"),
 ("veio.prestige.tier.2", "Vila próspera", "Thriving village"), ("veio.prestige.tier.3", "Cidade operária", "Worker city"),
 ("veio.decree.0", "nenhum", "none"), ("veio.decree.1", "Convocação", "Call-up"),
 ("veio.decree.2", "Racionamento", "Rationing"), ("veio.decree.3", "Fartura", "Plenty"),
 ("veio.decree.0.effect", "Decreto: ponha um sino, papel ou fardo de feno no slot Decreto.", "Decree: put a bell, paper or hay bale in the Decree slot."),
 ("veio.decree.1.effect", "Convocação (sino): postos contratam aldeões a até 16 blocos.", "Call-up (bell): posts hire villagers within 16 blocks."),
 ("veio.decree.2.effect", "Racionamento (papel): tarefas custam 1 ponto de comida, mas ninguém nasce.", "Rationing (paper): tasks cost 1 food point, but nobody is born."),
 ("veio.decree.3.effect", "Fartura (feno): tarefas custam 3 pontos; zonas ganham o dobro e nascem mais.", "Plenty (hay): tasks cost 3 points; zones gain double and more are born."),
 ("veio.bulletin.title", "☀ Bom dia! Dia %s da vila (%s).", "☀ Good morning! Village day %s (%s)."),
 ("veio.bulletin.yesterday", "Ontem: %s tarefas · aldeões %s de %s (%s) · %s pedido(s) atendido(s).", "Yesterday: %s tasks · villagers %s of %s (%s) · %s request(s) served."),
 ("veio.bulletin.extra", "Caixa rendeu %s esm. · viajantes chegando: %s.", "Treasury earned %s em. · travelers arriving: %s."),
 ("veio.bulletin.requests", "Pedidos de hoje: %s.", "Today's requests: %s."),
 ("veio.bulletin.calm", "Nenhuma pendência. Vila em ordem (+3 de prestígio).", "No issues. Village in order (+3 prestige)."),
 ("veio.bulletin.issues", "Pendências: %s. Bússola no centro para resolver.", "Issues: %s. Compass on the center to solve them."),
 ("veio.project.tier", "Exige prestígio: %s.", "Needs prestige: %s."),
 ("veio.project.library.name", "Biblioteca", "Library"),
 ("veio.project.library.effect", "Zonas ganham o dobro de trabalho por tarefa.", "Zones gain double work per task."),
 ("veio.project.post_office.name", "Correio", "Post office"),
 ("veio.project.post_office.effect", "4 pedidos por dia e +50% de esmeraldas.", "4 requests a day and +50% emeralds."),
 ("veio.project.inn.name", "Estalagem", "Inn"),
 ("veio.project.inn.effect", "Teto +20; um viajante chega toda manhã se houver lugar.", "Cap +20; a traveler arrives each morning if there is room."),
 ("veio.project.hall.name", "Prefeitura", "Town hall"),
 ("veio.project.hall.effect", "Prestígio +50% e a prefeitura paga 1 a cada 16 do caixa (até 8/dia).", "Prestige +50% and the town hall pays 1 per 16 of the cash (up to 8/day)."),
 ("veio.home.banquet", "Banquete! %s aldeões vêm jantar na sua casa. Você é o herói da vila.", "Banquet! %s villagers come to dinner at your house. You are the village hero."),
 ("veio.home.banquet_cold", "Acenda a lareira antes de receber convidados.", "Light the hearth before hosting guests."),
 ("veio.home.banquet_done", "Já houve banquete hoje. Amanhã tem mais.", "There was a banquet today already. More tomorrow."),
 ("veio.home.banquet_alone", "Nenhum aldeão livre por perto para convidar.", "No free villager nearby to invite."),
 ("veio.center.population", "Aldeões: %s de %s · comida: %s", "Villagers: %s of %s · food: %s"),
 ("veio.center.crew", "Trabalhadores: %s · ativos: %s · esperando: %s", "Workers: %s · active: %s · waiting: %s"),
 ("veio.center.zones_ready", "Lavouras: %s · cercados: %s · prontas: %s", "Fields: %s · pens: %s · ready: %s"),
 ("veio.center.tip.company", "Caixa da companhia e exportação média do dia.\nClique na linha: saca 1 esmeralda para o slot Saque.", "Company cash and average daily exports.\nClick the line: cash out 1 emerald to the Cash out slot."),
 ("veio.center.food", "Comida guardada: %s tarefas", "Food stored: %s tasks"),
 ("veio.center.issues", "Pendências: %s", "Issues: %s"),
 ("veio.center.no_issues", "Vila em ordem: nada pendente", "Village in order: nothing pending"),
 ("veio.center.issue", "%s: %s · %s m %s", "%s: %s · %s m %s"),
 ("veio.center.projects", "Obras da vila", "Village projects"),
 ("veio.center.project.done", "%s: pronta", "%s: built"),
 ("veio.center.project.active", "%s: ativa por %s dia(s)", "%s: on for %s day(s)"),
 ("veio.center.project.progress", "%s: %s%% · falta %s %s", "%s: %s%% · %s %s left"),
 ("veio.center.hint.compass", "Bússola no centro: ir até lá", "Compass on center: go there"),
 ("veio.center.hint.hover", "Cursor na linha: detalhes", "Hover a line: details"),
 ("veio.center.hint.site", "Materiais no slot Obra →", "Materials in the Build slot →"),
 ("veio.center.hint.page", "Chave: próxima página", "Wrench: next page"),
 ("veio.center.page_title", "%s %s/%s", "%s %s/%s"),
 ("veio.center.tip.population", "Aldeões a até 48 blocos e o teto da vila.\nO teto cresce com postos, zonas, armazéns,\no centro e a escola (+15). Comida: tarefas\nguardadas (2 pontos cada); menos de 16, abasteça.", "Villagers within 48 blocks and the village cap.\nThe cap grows with posts, zones, storage,\nthe center and the school (+15). Food: stored\ntasks (2 points each); under 16, restock."),
 ("veio.center.tip.workers", "Postos com trabalhador e quantos estão\nandando ou trabalhando agora.", "Staffed posts and how many are walking\nor working right now."),
 ("veio.center.tip.waiting", "Postos parados por algo que você resolve.\nVeja a página Atenção (chave).", "Posts stopped by something you fix.\nSee the Attention page (wrench)."),
 ("veio.center.tip.food", "Comida nos armazéns da área, em tarefas\n(2 pontos cada). Menos de 16: abasteça.", "Food in the area's storage, in tasks\n(2 points each). Under 16: restock."),
 ("veio.center.tip.zones", "Zonas e quantas já têm trabalho e aldeões\npara crescer: leve esmeraldas ao marco.", "Zones and how many have the work and villagers\nto grow: bring emeralds to the marker."),
 ("veio.center.tip.storage", "Ocupação dos celeiros e silos e tarefas\nna fila dos capatazes.", "Barn and silo usage and tasks\nin the foremen's queues."),
 ("veio.center.tip.offset", "Posição a partir do centro: x %s, y %s, z %s", "Offset from the center: x %s, y %s, z %s"),
 ("veio.center.problem.zone_ready", "pode crescer", "ready to grow"),
 ("veio.center.problem.store_full", "quase cheio", "almost full"),
 ("veio.center.kind.0", "Bloco", "Block"), ("veio.center.kind.1", "Lavrador", "Farmhand"),
 ("veio.center.kind.2", "Carregador", "Hauler"), ("veio.center.kind.3", "Capataz", "Foreman"),
 ("veio.center.kind.4", "Pastor", "Shepherd"), ("veio.center.kind.5", "Centro", "Center"),
 ("veio.center.kind.6", "Lavoura", "Field"), ("veio.center.kind.7", "Cercado", "Pen"),
 ("veio.center.kind.8", "Celeiro", "Barn"), ("veio.center.kind.9", "Silo", "Silo"),
 ("veio.dir.0", "N", "N"), ("veio.dir.1", "NE", "NE"), ("veio.dir.2", "L", "E"), ("veio.dir.3", "SE", "SE"),
 ("veio.dir.4", "S", "S"), ("veio.dir.5", "SO", "SW"), ("veio.dir.6", "O", "W"), ("veio.dir.7", "NO", "NW"),
 ("veio.report.compass", "Dica: use uma bússola no centro para ir até cada pendência.", "Tip: use a compass on the center to walk to each issue."),
 # Foco.
 ("veio.focus.set", "Foco %s de %s: %s em %s. Siga a barra de ação.", "Focus %s of %s: %s at %s. Follow the action bar."),
 ("veio.focus.bar", "➤ %s · %s m %s", "➤ %s · %s m %s"),
 ("veio.focus.here", "Aqui: %s", "Here: %s"),
 ("veio.focus.resolved", "Resolvido: %s", "Solved: %s"),
 ("veio.focus.next", "Próxima de %s: %s", "Next of %s: %s"),
 ("veio.focus.all_clear", "Vila em ordem. Bom trabalho!", "Village in order. Well done!"),
 ("veio.focus.none", "Nada pendente na vila agora.", "Nothing pending in the village right now."),
 ("veio.focus.cleared", "Foco desligado.", "Focus cleared."),
 ("veio.focus.expired", "O foco acabou. Use a bússola no centro de novo.", "Focus ended. Use the compass on the center again."),
 ("veio.focus.advice.zone_ready", "Clique no marco com esmeraldas para o próximo nível.", "Click the marker with emeralds for the next level."),
 ("veio.focus.advice.store_full", "Esvazie, use as sobras ou construa outro celeiro.", "Empty it, use the surplus or build another barn."),
 # Obras.
 ("veio.project.done", "Obra pronta: %s. %s", "Project built: %s. %s"),
 ("veio.project.needs", "Materiais: %s", "Materials: %s"),
 ("veio.project.repeat", "Pode ser feita de novo.", "Can be done again."),
 ("veio.project.canteen.name", "Refeitório", "Canteen"),
 ("veio.project.canteen.effect", "Toda 4ª tarefa dos postos não gasta comida.", "Every 4th post task costs no food."),
 ("veio.project.school.name", "Escola", "School"),
 ("veio.project.school.effect", "Nascem mais aldeões (1/3) e o teto sobe 15.", "More villagers are born (1/3) and the cap rises by 15."),
 ("veio.project.smithy.name", "Ferraria", "Smithy"),
 ("veio.project.smithy.effect", "Carregadores levam 24 itens e recolhem 48 por viagem.", "Haulers carry 24 items and collect 48 per trip."),
 ("veio.project.market.name", "Praça do mercado", "Market square"),
 ("veio.project.market.effect", "Expandir zonas custa 25% menos esmeraldas.", "Growing zones costs 25% fewer emeralds."),
 ("veio.project.festival.name", "Festa da colheita", "Harvest festival"),
 ("veio.project.festival.effect", "Hoje e amanhã: tarefas sem comida e nascem\nmais aldeões (1/2).", "Today and tomorrow: tasks cost no food and\nmore villagers are born (1/2)."),
 # Lar.
 ("veio.home.claimed", "Este é o seu lar. Você renasce aqui.", "This is your home. You respawn here."),
 ("veio.home.lit", "Lareira acesa · %s", "Hearth lit · %s"),
 ("veio.home.cold", "Lareira apagada: ponha lenha", "Hearth out: add fuel"),
 ("veio.home.comfort", "Conforto %s/100 · nível %s: %s", "Comfort %s/100 · level %s: %s"),
 ("veio.home.level.1", "Casa simples", "Simple house"),
 ("veio.home.level.2", "Casa de tijolos", "Brick house"),
 ("veio.home.level.3", "Casa elétrica", "Electric house"),
 ("veio.home.level.4", "Casa de engenheiro", "Engineer's house"),
 ("veio.home.perk.2", "A lareira cozinha carne, peixe e batata crus.\nEfeitos das refeições e do descanso +50%.", "The hearth cooks raw meat, fish and potatoes.\nMeal and rest effects +50%."),
 ("veio.home.perk.3", "Escritório: o painel mostra a vila e a bússola\nno lar dá o foco das pendências.", "Office: the panel shows the village and a compass\non the hearth gives issue focus."),
 ("veio.home.perk.4", "Refeições variadas dão pressa II e o descanso\ntambém dá pressa.", "Varied meals give haste II and resting\nalso gives haste."),
 ("veio.home.level_up", "Sua casa subiu para o nível %s: %s.", "Your house reached level %s: %s."),
 ("veio.home.today", "Hoje: %s", "Today: %s"),
 ("veio.home.task.meal", "coma em casa (tigela)", "eat at home (bowl)"),
 ("veio.home.task.fire", "acenda a lareira", "light the hearth"),
 ("veio.home.task.pantry", "abasteça a despensa (3+)", "stock the pantry (3+)"),
 ("veio.home.day_done", "Dia em ordem · sequência: %s dia(s)", "Day in order · streak: %s day(s)"),
 ("veio.home.day_done_chat", "Dia em ordem em casa! Sequência: %s dia(s).", "A tidy day at home! Streak: %s day(s)."),
 ("veio.home.meal_ready", "Refeição pronta · despensa %s · cardápio %s/5", "Meal ready · pantry %s · menu %s/5"),
 ("veio.home.meal_wait", "Próxima refeição em %s s · cardápio %s/5", "Next meal in %s s · menu %s/5"),
 ("veio.home.meal_wait_bar", "Ainda satisfeito. Próxima refeição em %s s.", "Still full. Next meal in %s s."),
 ("veio.home.meal", "Refeição em casa: %s · cardápio %s/5 · conforto %s", "Meal at home: %s · menu %s/5 · comfort %s"),
 ("veio.home.meal_cold", "Comida fria (%s). Acenda a lareira para os efeitos.", "Cold food (%s). Light the hearth for the effects."),
 ("veio.home.no_food", "A despensa está vazia.", "The pantry is empty."),
 ("veio.home.missing", "Falta: %s", "Missing: %s"),
 ("veio.home.cozy", "Casa completa e aconchegante", "Complete, cozy house"),
 ("veio.home.upgrade", "Nível %s: %s%% · falta %s %s", "Level %s: %s%% · %s %s left"),
 ("veio.home.max", "Casa no nível máximo", "House at top level"),
 ("veio.home.village", "Vila: %s/%s · %s pendência(s) · %s pedido(s)", "Village: %s/%s · %s issue(s) · %s request(s)"),
 ("veio.home.office_locked", "Nível 3: escritório da vila", "Level 3: village office"),
 ("veio.home.no_village", "Sem centro de controle por perto", "No control center nearby"),
 ("veio.home.hint", "Tigela: comer · bolo: banquete · bússola: foco", "Bowl: eat · cake: banquet · compass: focus"),
 ("veio.home.rested", "Você acordou descansado em casa (conforto %s).", "You woke up rested at home (comfort %s)."),
 ("veio.home.cat.bed", "cama", "bed"), ("veio.home.cat.door", "porta", "door"),
 ("veio.home.cat.window", "janela", "window"), ("veio.home.cat.light", "luz", "light"),
 ("veio.home.cat.storage", "baú", "chest"), ("veio.home.cat.stove", "fogão", "stove"),
 ("veio.home.cat.workshop", "bancada", "workbench"), ("veio.home.cat.decor", "enfeites", "decor"),
 ("veio.home.cat.finish", "acabamento", "finish"),
 ("veio.home.tip.fire", "Carvão e carvão vegetal: 4 min; biomassa: 2 min;\ntroncos: 80 s. Acesa à noite = acordar descansado.", "Coal and charcoal: 4 min; biomass: 2 min;\nlogs: 80 s. Lit at night = wake up rested."),
 ("veio.home.tip.comfort", "O lar lê a casa em 9x9, de 1 abaixo a 3 acima:", "The hearth reads the house in 9x9, 1 below to 3 above:"),
 ("veio.home.tip.today", "Comer em casa, lareira acesa e 3+ comidas\nna despensa: dia em ordem, experiência e sequência.", "Eat at home, hearth lit and 3+ food\nin the pantry: day in order, XP and streak."),
 ("veio.home.tip.meal", "Tigela no lar: enche a fome. Com a lareira acesa:\nregeneração; 3 pratos diferentes: pressa; 5: resistência.\nConforto e nível aumentam a duração.", "Bowl on the hearth: fills hunger. With the hearth lit:\nregeneration; 3 different dishes: haste; 5: resistance.\nComfort and level extend the duration."),
 ("veio.home.tip.build", "Ponha os materiais no slot Obra, um de cada vez.", "Put the materials in the Build slot, one at a time."),
 ("veio.home.tip.office", "No nível 3, o lar acompanha o centro de controle\nmais próximo: aldeões, pendências e pedidos abertos.\nBússola no lar: ir até a pendência.", "At level 3 the hearth follows the nearest control\ncenter: villagers, issues and open requests.\nCompass on the hearth: go to the issue."),
 ("veio.home.tip.hint", "Bolo no lar (1 por dia, lareira acesa): até 6 aldeões\nlivres vêm jantar; você vira herói da vila (descontos\nno comércio) e a vila ganha prestígio.\nBússola (nível 3): foco nas pendências da vila.\nAgachar + chave: o lar passa a ser seu.", "Cake on the hearth (once a day, lit): up to 6 free\nvillagers come to dinner; you become the village hero\n(trade discounts) and the village gains prestige.\nCompass (level 3): focus on village issues.\nSneak + wrench: make this hearth yours."),
 ("advancement.veio.homestead.title", "Lar, doce lar", "Home Sweet Home"),
 ("advancement.veio.homestead.description", "Faça um lar na vila: lareira, despensa e uma casa de verdade.", "Make a home in the village: hearth, pantry and a real house."),
]:
 label(key, pt, en)
# Grande indústria (0.22): blocos do Guia da Grande Indústria, o terceiro livro.
help.update({
 "research_lab":(
 "Pesquisa da fábrica: entregue\ncentenas de peças. Consome 2/s\nde cada entrada com 24 FE/t.\nVários laboratórios somam.",
 "Factory research: deliver\nhundreds of parts. Eats 2/s\nper input with 24 FE/t.\nSeveral labs add up."),
 "fabricator":(
 "Três entradas: motor,\nprocessador e aço ou\nengrenagens. Faz o núcleo de\nautomação e o kit Mk.II.",
 "Three inputs: motor,\nprocessor and steel or\ngears. Makes the automation\ncore and the Mk.II kit."),
 "factory_monitor":(
 "Estatísticas da dimensão:\nmáquinas paradas e itens\nproduzidos e gastos por\nminuto. Avisa os problemas.",
 "Dimension statistics:\nidle machines and items\nmade and used per minute.\nWarns about problems."),
 "chunk_anchor":(
 "Mantém o chunk carregado\nmesmo longe de você.\nGasta 32 FE/t; sem energia\nsolta o chunk em 5 s.",
 "Keeps its chunk loaded\neven when you are far.\nUses 32 FE/t; without power\nreleases it after 5 s."),
 "express_belt":(
 "Mesmas curvas e rampas.\nTransporta até 10 itens/s,\no dobro da rápida.\nChave fixa a direção.",
 "Same turns and ramps.\nCarries up to 10 items/s,\ntwice the fast belt.\nWrench locks direction."),
 "cargo_station":(
 "No posto distante: recebe\ndas esteiras e manda ao\ncanal (corante). Longe de\nvocê, segue no ritmo medido.",
 "At the distant outpost: takes\nfrom belts and ships to its\nchannel (dye). While you are\naway it keeps the measured rate."),
 "unloading_station":(
 "Na base: entrega o que\nchega pelo canal da mesma\ncor. Bocal na frente: ligue\numa esteira ou um baú.",
 "At the base: delivers what\narrives on the same color\nchannel. Port in front: add\na belt or a chest."),
 "belt_tunnel":(
 "Coloque dois, na mesma\ndireção, até 6 blocos.\nA entrada passa os itens\npor baixo até a saída.",
 "Place two facing the same\nway, up to 6 blocks apart.\nThe entrance passes items\nunderneath to the exit."),
})
# Terminais de transporte: consulta no livro Transportes (build_transport_guide.py).
transport_order=["rail_terminal","drone_port","caravan_station"]
industry_order=["research_lab","fabricator","factory_monitor","cargo_station","unloading_station","chunk_anchor","express_belt","belt_tunnel","blueprint_designer","build_marker"]

# Quadro da fábrica (milestones/): fases, pedido do dia, censo de gargalos e ritmo do dia.
for key, pt, en in [
 ("block.veio.factory_board", "Quadro da fábrica", "Factory board"),
 ("item.veio.permit_workshop", "Licença da oficina", "Workshop permit"),
 ("item.veio.permit_steel", "Licença do aço", "Steel permit"),
 ("item.veio.permit_electronics", "Licença da eletrônica", "Electronics permit"),
 ("veio.componenthelp.permit_workshop", "Dada pelo quadro: abre a chave, o triturador e o forno de ligas.", "From the board: unlocks the wrench, crusher and alloy furnace."),
 ("veio.componenthelp.permit_steel", "Dada pelo quadro: abre a caldeira, os acumuladores e a geotermia.", "From the board: unlocks steam, batteries and geothermal."),
 ("veio.componenthelp.permit_electronics", "Dada pelo quadro: abre a Mk.II, o solar e a torre Tesla.", "From the board: unlocks Mk.II mining, solar and the Tesla tower."),
 ("veio.board.phase", "Fase %s de %s: %s", "Phase %s of %s: %s"),
 ("veio.board.phase.first", "Primeira fábrica", "First factory"),
 ("veio.board.phase.workshop", "Oficina", "Workshop"),
 ("veio.board.phase.steel", "Aço", "Steel"),
 ("veio.board.phase.electronics", "Eletrônica", "Electronics"),
 ("veio.board.phase.depth", "Profundidade", "Depth"),
 ("veio.board.deliver", "%s/%s %s", "%s/%s %s"),
 ("veio.board.reward", "Recompensa: %sx %s", "Reward: %sx %s"),
 ("veio.board.unlocks", "Libera: %s", "Unlocks: %s"),
 ("veio.board.next", "Depois: %s", "Next: %s"),
 ("veio.board.setup.power", "Energia", "Power"),
 ("veio.board.setup.drill", "Extratora", "Extractor"),
 ("veio.board.setup.press", "Prensa", "Press"),
 ("veio.board.setup.assembler", "Montador", "Assembler"),
 ("veio.board.hint.workshop", "Próximo gargalo: energia. Uma linha de 30 engrenagens por minuto pede um gerador inteiro.", "Next bottleneck: power. A 30 gears a minute line needs a whole generator."),
 ("veio.board.hint.steel", "Próximo gargalo: carvão. Cada forno de ligas queima 15 por minuto.", "Next bottleneck: coal. Each alloy furnace burns 15 a minute."),
 ("veio.board.hint.electronics", "Próximo gargalo: silício. Quartzo do Nether, ou areia no forno de ligas.", "Next bottleneck: silicon. Nether quartz, or sand in the alloy furnace."),
 ("veio.board.hint.depth", "Próximo gargalo: transporte. Uma Mk.III num veio de 16+ blocos passa de uma esteira rápida.", "Next bottleneck: transport. A Mk.III on a 16+ block vein outruns a fast belt."),
 ("veio.board.done", "Fases concluídas. A vila segue com pedidos.", "All phases done. The village keeps ordering."),
 ("veio.board.order", "Dia %s: %s/%s %s", "Day %s: %s/%s %s"),
 ("veio.board.order_reward", "Pedido da vila: %s esmeraldas até amanhecer.", "Village order: %s emeralds until dawn."),
 ("veio.board.order_paid", "Pedido da vila pago. Outro amanhã.", "Village order paid. A new one tomorrow."),
 ("veio.board.order_elsewhere", "Pedido de hoje já pago em outro quadro.", "Today's order was paid at another board."),
 ("veio.board.no_machines", "Nenhuma máquina perto do quadro.", "No machines near the board."),
 ("veio.board.ok", "%s de %s máquinas trabalhando.", "%s of %s machines working."),
 ("veio.board.issues", "Paradas: %s energia, %s cheias, %s danificadas", "Stalled: %s power, %s full, %s damaged"),
 ("veio.board.complete", "Fase concluída: %s! Retire a recompensa no quadro.", "Phase complete: %s! Take the reward from the board."),
 ("veio.board.order_complete", "Pedido do dia entregue: %s esmeraldas no quadro.", "Daily order delivered: %s emeralds on the board."),
 ("veio.alert.near", "Máquinas paradas perto: %s sem energia, %s sem combustível, %s com saída cheia, %s danificadas", "Stalled nearby: %s no power, %s no fuel, %s output full, %s damaged"),
 ("veio.day.dawn", "Dia %s. Pedido da vila no quadro: %s %s, paga %s esmeraldas até o próximo amanhecer.", "Day %s. Village order at the board: %s %s, pays %s emeralds until next dawn."),
 ("veio.day.dusk", "Fim do dia %s: a fábrica produziu %s itens.", "End of day %s: the factory made %s items."),
 ("veio.day.dusk_order", "Pedido da vila: %s/%s %s. Termina ao amanhecer.", "Village order: %s/%s %s. Ends at dawn."),
 ("veio.day.dusk_paid", "Pedido da vila: pago.", "Village order: paid."),
 ("veio.day.dusk_issue", "Para amanhã: %s máquinas sem energia, %s com saída cheia, %s danificadas.", "For tomorrow: %s machines without power, %s with full output, %s damaged."),
]: label(key, pt, en)

# Megafábricas: copiadora, marco de obra, elevador de carga e andares por escada.
for key, pt, en in [
 ("block.veio.cargo_lift", "Elevador de carga", "Cargo lift"),
 ("block.veio.blueprint_designer", "Copiadora", "Blueprint copier"),
 ("block.veio.build_marker", "Marco de obra", "Build marker"),
 ("advancement.veio.vertical.title", "Para cima", "Going up"),
 ("advancement.veio.vertical.description", "Leve itens entre andares com um elevador de carga.", "Move items between floors with a cargo lift."),
 ("advancement.veio.blueprints.title", "Copiar e colar", "Copy and paste"),
 ("advancement.veio.blueprints.description", "Fabrique uma copiadora e multiplique suas linhas.", "Build a copier and multiply your lines."),
 ("veio.lift.up", "Sentido: subindo", "Direction: up"),
 ("veio.lift.down", "Sentido: descendo", "Direction: down"),
 ("veio.hint.lift", "Agache com a chave para inverter.", "Sneak with the wrench to reverse."),
 ("veio.blueprint.copied", "Projeto copiado: %s blocos de %s materiais, prancheta de %s de lado e %s de altura.", "Design copied: %s blocks of %s materials, a %s-wide board, %s high."),
 ("veio.blueprint.skipped", "%s blocos ficaram de fora: minério, fluido, portal ou acima do limite.", "%s blocks left out: ores, fluids, portals or over the limit."),
 ("veio.blueprint.size", "Prancheta: %s de lado, até %s de altura.", "Board: %s wide, up to %s high."),
 ("veio.blueprint.designer", "Copiadora: prancheta de %s de lado e até %s de altura. Projeto guardado: %s blocos. Clique com papel para copiar.", "Copier: a %s-wide board up to %s high. Stored design: %s blocks. Click with paper to copy."),
 ("veio.blueprint.no_design", "Nenhuma copiadora com projeto a até %s blocos.", "No copier with a design within %s blocks."),
 ("veio.blueprint.budget", "Projeto da copiadora em %s: %s blocos, %s de lado, %s de altura, girado %s graus.", "Design from the copier at %s: %s blocks, %s wide, %s high, turned %s degrees."),
 ("veio.blueprint.cost", "  %sx %s", "  %sx %s"),
 ("veio.blueprint.more", "  e mais %s materiais.", "  and %s more materials."),
 ("veio.blueprint.blocked", "Obra bloqueada em %s. Libere o espaço.", "Build blocked at %s. Clear the space."),
 ("veio.blueprint.clear", "Espaço livre. Agache com a chave para construir.", "Space is clear. Sneak with the wrench to build."),
 ("veio.blueprint.built", "Obra concluída: %s blocos.", "Build complete: %s blocks."),
 ("veio.blueprint.missing", "Faltam: %s. Nada foi cobrado.", "Missing: %s. Nothing was charged."),
]: label(key, pt, en)
categories=[
 ("Extração e processamento","Mining and processing",["drill","drill_mk2","drill_mk3","press","assembler","cutter","integrator","crusher","alloy_furnace","electronics"]),
 ("Lavoura e comida","Farming and food",["harvester","planter","sprinkler","grow_lamp","tree_farm","ranch","mill","electric_furnace","kitchen","pantry"]),
 ("Energia","Power",["generator","biomass_generator","water_wheel","wind_turbine","steam_engine","geothermal","solar","battery","hydro_turbine","pump","pipe"]),
 ("Logística","Logistics",["belt","fast_belt","splitter","merger","inserter","fast_inserter","long_inserter","cargo_lift","cable","pole","steel_pole","wrench"]),
 ("Projetos e defesa","Projects and defense",["factory_board","contract_hub","tesla"]),
]
order=[name for _,_,names in categories for name in names]
# Blocos da vila operária: consulta no Guia da Vila Operária (fim deste arquivo).
village_order=["farm_post","field_marker","shepherd_post","pen_marker","barn","silo","hauler_post","foreman_desk","control_center","site","barracks","house","shop","town_hall","well","park","plaza","monument","export_dock","freight_station","home"]
assert set(order)|set(village_order)|set(industry_order)|set(transport_order)==set(help), (set(order)|set(village_order)|set(industry_order)|set(transport_order))^set(help)
for name,(pt,en) in help.items():
 label(f"veio.help.{name}",pt,en)

slothelp={
 "drill":["Produto extraído. Retire ou ligue uma esteira."],
 "drill_mk2":["Produto extraído. Retire ou ligue uma esteira."],
 "generator":["Coloque carvão, madeira ou outro combustível."],
 "harvester":["Farinha de osso: acelera as plantações.","Colheita.","Colheita.","Colheita."],
 "geothermal":["Coloque um balde de lava.","Recolha os baldes vazios."],
 "press":["Coloque cobre, ferro ou ouro BRUTO.","Recolha as chapas ou os lingotes."],
 "assembler":["Coloque DUAS chapas de cobre.","Recolha as engrenagens."],
 "cutter":["Coloque chapa de ferro ou de cobre.","Recolha molduras ou fiação."],
 "integrator":["Coloque UMA moldura.","Coloque UMA engrenagem.","Recolha o módulo mecânico."],
 "crusher":["Coloque quartzo, osso, pedregulho ou cascalho.","Recolha o produto triturado."],
 "alloy_furnace":["Coloque DUAS chapas de ferro.","Coloque UM carvão ou carvão vegetal.","Recolha as chapas de aço."],
 "electronics":["Coloque DUAS fiações de cobre.","Coloque UMA pastilha de silício.","Recolha as placas de circuito."],
 "belt":["Itens guardados neste trecho da esteira."],
 "fast_belt":["Itens guardados neste trecho da esteira."],
 "splitter":["Itens aguardando as saídas laterais."],
 "drill_mk3":["Produto extraído. Retire ou ligue uma esteira."],
 "electric_furnace":["Coloque o que deseja fundir ou assar.","Recolha o produto."],
 "mill":["Trigo, cana, plantas ou troncos.","Recolha farinha, açúcar ou biomassa."],
 "kitchen":["Ingrediente principal: farinha, abóbora...","Segundo ingrediente: cacau, açúcar, ouro...","Recolha a comida pronta."],
 "pantry":["Coloque comida para vender.","Recolha as esmeraldas."],
 "biomass_generator":["Biomassa, plantas ou feno."],
 "steam_engine":["Carvão, madeira ou outro combustível."],
 "sprinkler":["Adubo: farinha de osso ou biomassa."],
 "planter":["Sementes, cenouras, batatas ou fungo do Nether."],
 "tree_farm":["Mudas de carvalho, bétula, pinheiro, selva, acácia ou cerejeira.","Produto.","Produto.","Produto."],
 "ranch":["Trigo, cenoura, batata, beterraba ou sementes.","Baldes vazios para ordenhar.","Produto.","Produto.","Produto."],
 "farm_post":["Comida do trabalhador: pão vale 3 colheitas.","Colheita entregue.","Colheita entregue.","Colheita entregue."],
 "hauler_post":["Comida do trabalhador: pão vale 3 viagens.","Insumo para as máquinas.","Insumo para as máquinas.","Insumo para as máquinas.","Produto recolhido.","Produto recolhido.","Produto recolhido."],
 "shepherd_post":["Comida do trabalhador.","Ração: trigo, cenoura, sementes...","Produto.","Produto.","Produto."],
 "field_marker":["Semente desta lavoura (vazio: qualquer uma)."],
 "pen_marker":["Ração deste cercado (vazio: qualquer uma)."],
 "barn":["Estoque."]*27,
 "silo":["Grãos, sementes, ração ou comida."]*18,
 "house":["Despensa: comida que a família comprou (ou que você doa).","Despensa.","Despensa."],
 "shop":["Comida do comerciante.","Entrega da indústria: o comerciante paga 75% no atacado.","Entrega da indústria.","Entrega da indústria.","Entrega da indústria.","Entrega da indústria.","Entrega da indústria.","Antigo caixa (até a 0.20): retire as esmeraldas."],
 "town_hall":["Comida do prefeito.","Aporte voluntário à conta pública (60 moedas por esmeralda).","Agenda: materiais dos eventos pedidos pelos visitantes."],
 "park":["Enfeite: mudas, flores ou lanterna."],
 "plaza":["Enfeite: mudas, flores ou lanterna."],
 "monument":["Enfeite: mudas, flores ou lanterna."],
 "well":["Balde vazio: sai cheio de água."],
 "export_dock":["Carga do navio: produtos para exportar."]*9,
 "freight_station":["Carga do trem: produtos para exportar."]*9,
 "control_center":["Comida do administrador.","Meta: o item e a quantidade x16 a guardar.","Meta: o item e a quantidade x16 a guardar.","Meta: o item e a quantidade x16 a guardar.","Cofre da companhia: esmeraldas viram moedas (60 cada).","Saque: clique na linha da companhia no painel (1 esmeralda).","Obra: entregas dos pedidos, materiais das obras; esmeraldas vão para o caixa.","Decreto: sino (convocação), papel (racionamento) ou fardo de feno (fartura)."],
 "site":["Comida do mestre de obras: cada leva de blocos custa uma tarefa."],
 "home":["Lenha: carvão, carvão vegetal, biomassa, troncos ou tábuas.","Despensa: comida para as refeições em casa.","Despensa: comida para as refeições em casa.","Despensa: comida para as refeições em casa.","Obra: materiais da próxima melhoria da casa."],
 "foreman_desk":["Comida do capataz e dos postos.","Comida do capataz e dos postos.","Comida do capataz e dos postos.","Esmeraldas para recontratar."],
 "cargo_lift":["Itens a caminho. Entram por qualquer lado."],
 "factory_board":["Entrega: pilha da fase ou pedido do dia. Aceita esteira.","Recompensas: licenças, esteiras, esmeraldas."],
 "contract_hub":["Primeiro material do pedido exibido.","Segundo material do pedido exibido.","Primeira recompensa do pedido.","Segunda recompensa do pedido."],
}
slot_en={
 "drill":["Extracted product. Take it or connect a belt."],
 "drill_mk2":["Extracted product. Take it or connect a belt."],
 "generator":["Insert coal, wood or another fuel."],
 "harvester":["Bone meal: speeds up the crops.","Harvest.","Harvest.","Harvest."],
 "geothermal":["Insert a lava bucket.","Collect the empty buckets."],
 "press":["Insert RAW copper, iron or gold.","Collect plates or ingots."],
 "assembler":["Insert TWO copper plates.","Collect the gears."],
 "cutter":["Insert iron or copper plates.","Collect frames or wiring."],
 "integrator":["Insert ONE frame.","Insert ONE gear.","Collect the mechanical module."],
 "crusher":["Insert quartz, bone, cobblestone or gravel.","Collect the crushed product."],
 "alloy_furnace":["Insert TWO iron plates.","Insert ONE coal or charcoal.","Collect the steel plates."],
 "electronics":["Insert TWO copper wiring.","Insert ONE silicon wafer.","Collect the circuit boards."],
 "belt":["Items held by this conveyor segment."],
 "fast_belt":["Items held by this conveyor segment."],
 "splitter":["Items waiting for the side outputs."],
 "drill_mk3":["Extracted product. Take it or connect a belt."],
 "electric_furnace":["Insert what you want to smelt or cook.","Collect the product."],
 "mill":["Wheat, sugar cane, plants or logs.","Collect flour, sugar or biomass."],
 "kitchen":["Main ingredient: flour, pumpkin...","Second ingredient: cocoa, sugar, gold...","Collect the finished food."],
 "pantry":["Insert food to sell.","Collect the emeralds."],
 "biomass_generator":["Biomass, plants or hay."],
 "steam_engine":["Coal, wood or another fuel."],
 "sprinkler":["Fertilizer: bone meal or biomass."],
 "planter":["Seeds, carrots, potatoes or nether wart."],
 "tree_farm":["Oak, birch, spruce, jungle, acacia or cherry saplings.","Product.","Product.","Product."],
 "ranch":["Wheat, carrots, potatoes, beetroot or seeds.","Empty buckets for milking.","Product.","Product.","Product."],
 "farm_post":["Worker food: bread pays for 3 harvests.","Delivered harvest.","Delivered harvest.","Delivered harvest."],
 "hauler_post":["Worker food: bread pays for 3 trips.","Supplies for the machines.","Supplies for the machines.","Supplies for the machines.","Collected product.","Collected product.","Collected product."],
 "shepherd_post":["Worker food.","Feed: wheat, carrots, seeds...","Product.","Product.","Product."],
 "field_marker":["Seed for this field (empty: any)."],
 "pen_marker":["Feed for this pen (empty: any)."],
 "barn":["Stock."]*27,
 "silo":["Grain, seeds, feed or food."]*18,
 "house":["Pantry: food the family bought (or you donate).","Pantry.","Pantry."],
 "shop":["Merchant food.","Industry delivery: the merchant pays 75% wholesale.","Industry delivery.","Industry delivery.","Industry delivery.","Industry delivery.","Industry delivery.","Old till (up to 0.20): take the emeralds out."],
 "town_hall":["Mayor food.","Voluntary contribution to public funds (60 coins per emerald).","Agenda: materials for events visitors ask for."],
 "park":["Decoration: saplings, flowers or a lantern."],
 "plaza":["Decoration: saplings, flowers or a lantern."],
 "monument":["Decoration: saplings, flowers or a lantern."],
 "well":["Empty bucket: comes out full of water."],
 "export_dock":["Ship cargo: goods to export."]*9,
 "freight_station":["Train cargo: goods to export."]*9,
 "control_center":["Administrator food.","Goal: the item and its count x16 to store.","Goal: the item and its count x16 to store.","Goal: the item and its count x16 to store.","Company vault: emeralds become coins (60 each).","Cash out: click the company line on the panel (1 emerald).","Build: request deliveries, project materials; emeralds go to the treasury.","Decree: bell (call-up), paper (rationing) or hay bale (plenty)."],
 "site":["Master builder food: each batch of blocks costs one task."],
 "home":["Fuel: coal, charcoal, biomass, logs or planks.","Pantry: food for meals at home.","Pantry: food for meals at home.","Pantry: food for meals at home.","Build: materials for the next house upgrade."],
 "foreman_desk":["Food for the foreman and the posts.","Food for the foreman and the posts.","Food for the foreman and the posts.","Emeralds for rehiring."],
 "cargo_lift":["Items on the way. They enter from any side."],
 "factory_board":["Delivery: the phase stack or the daily order. Belts welcome.","Rewards: permits, belts, emeralds."],
 "contract_hub":["First ingredient in the displayed order.","Second ingredient in the displayed order.","First reward.","Second reward."],
}
for name in ("rail_terminal", "drone_port", "caravan_station"):
 slothelp[name] = ["Carga para enviar. O filtro é copiado com agachar + item."]*3 + ["Carga recebida. O carregador pode levar às máquinas."]*3
 slot_en[name] = ["Dispatch stock. Sneak + item copies its filter."]*3 + ["Received stock. Haulers can feed it to machines."]*3
 if name in ("drone_port", "caravan_station"):
  slothelp[name].append("Carga em voo, conservada aqui até a entrega. Retirar impede a entrega.")
  slot_en[name].append("Flight cargo is kept here until delivery. Taking it prevents delivery.")
slothelp["caravan_station"][6] = "Carga em trânsito: conservada na estação até o animal chegar ao destino."
slot_en["caravan_station"][6] = "Transit cargo: kept at the station until the animal reaches its destination."
slothelp["caravan_station"].append("Ração: cavalo/burro aceitam trigo, maçã ou feno; porco aceita cenoura, batata ou beterraba.")
slot_en["caravan_station"].append("Feed: horse/donkey eat wheat, apples or hay; pig eats carrots, potatoes or beetroots.")
slothelp.update({
 "research_lab":["Primeira peça da pesquisa atual.","Segunda peça da pesquisa atual."],
 "fabricator":["Coloque DOIS motores.","Coloque processadores.","Aço (núcleo) ou engrenagens (kit Mk.II).","Recolha o produto."],
 "express_belt":["Itens guardados neste trecho da esteira."],
 "belt_tunnel":["Itens a caminho da saída do túnel."],
 "cargo_station":["Itens para mandar pelo canal."]*4,
 "unloading_station":["Itens que chegaram pelo canal."]*4,
})
slot_en.update({
 "research_lab":["First part of the current research.","Second part of the current research."],
 "fabricator":["Put TWO motors.","Put processors.","Steel (core) or gears (Mk.II kit).","Collect the product."],
 "express_belt":["Items held by this conveyor segment."],
 "belt_tunnel":["Items on their way to the tunnel exit."],
 "cargo_station":["Items to ship on the channel."]*4,
 "unloading_station":["Items that arrived on the channel."]*4,
})
for block,lines in slothelp.items():
 for i,line in enumerate(lines): label(f"veio.slothelp.{block}.{i}",line,slot_en[block][i])
label("veio.slothelp.merger.0", "A: lateral amarela. Olhando para a saída, fica à esquerda.", "A: yellow side. Left when facing the outlet.")
label("veio.slothelp.merger.1", "B: lateral azul. Olhando para a saída, fica à direita.", "B: cyan side. Right when facing the outlet.")

label("veio.guide.title","Guia do Veio","Veio Field Guide")
label("veio.guide.help","Ajuda ?","Help ?")
label("veio.guide.open","Selecione o guia e use o botão direito.","Select the guide and right-click.")
label("veio.guide.tip","Passe o cursor no ? dos painéis.","Hover over ? in machine panels.")
label("veio.guide.welcome","A vila precisa de uma oficina. Siga o objetivo acima da barra de itens. O caderno recebido é uma consulta opcional; o progresso do tutorial acompanha máquinas funcionando.","The village needs a workshop. Follow the objective above the hotbar. The workbook is an optional reference; tutorial progress follows working machines.")
label("veio.guide.back","Sumário","Contents")
def back_mark():
 """Volta ao sumário na mesma linha do título: poupa a última linha da página."""
 return {"text":"  «","color":"dark_blue","click_event":{"action":"change_page","page":1},
  "hover_event":{"action":"show_text","value":{"translate":"veio.guide.back"}}}
label("veio.guide.recipe_link","Receita","Recipe")
label("veio.guide.role.input","Entrada","Input")
label("veio.guide.role.output","Saída","Output")
label("veio.guide.role.fuel","Combust.","Fuel")
label("veio.guide.role.input1","Ent.1","In 1")
label("veio.guide.role.input2","Ent.2","In 2")
label("veio.guide.role.input3","Ent.3","In 3")
label("veio.guide.role.output1","Sai.1","Out 1")
label("veio.guide.role.output2","Sai.2","Out 2")
label("veio.guide.role.fertilizer","Adubo","Fertilizer")
label("veio.guide.role.pantry","Despensa","Pantry")
label("veio.guide.role.build","Obra","Build")
label("veio.guide.role.decree","Decreto","Decree")
for key, pt, en in [
 ("block.veio.site", "Canteiro de obras", "Construction Site"),
 ("veio.status.site_plan", "Preparando a obra", "Preparing the project"),
 ("veio.status.site_blocked", "Terreno ocupado", "Ground occupied"),
 ("veio.status.site_building", "Construindo", "Building"),
 ("veio.status.site_done", "Prédio de pé", "Building standing"),
 ("veio.site.picked", "Planta: %s", "Blueprint: %s"),
 ("veio.site.started", "A obra já começou: a planta não muda mais.", "Construction has started: the blueprint is locked."),
 ("veio.site.plan", "%s · %sx%s, %s de altura", "%s · %sx%s, %s tall"),
 ("veio.site.reserved", "Falta: %s x %s nos lugares que piscam", "Missing: %s x %s at the blinking spots"),
 ("veio.site.standing", "De pé! O efeito vale para a vila.", "Standing! The effect applies to the village."),
 ("veio.site.progress", "Construído: %s%% · %s blocos assentados", "Built: %s%% · %s blocks placed"),
 ("veio.site.blocker", "Bloco do jogador em x %s, y %s, z %s do canteiro", "Player block at x %s, y %s, z %s from the site"),
 ("veio.site.surveying", "Conferindo o terreno...", "Surveying the ground..."),
 ("veio.site.wait.0", "Pronto para começar", "Ready to start"),
 ("veio.site.wait.1", "Sem centro de controle a até 64 blocos", "No control center within 64 blocks"),
 ("veio.site.wait.2", "Pague a obra no centro (slot Obra)", "Pay for the project at the center (Build slot)"),
 ("veio.site.wait.3", "Este prédio já está de pé na vila", "This building already stands in the village"),
 ("veio.site.wait.4", "Outro canteiro já constrói este prédio", "Another site is already building it"),
 ("veio.site.done", "Obra de pé: %s! %s", "Building complete: %s! %s"),
 ("veio.hint.site", "Chave: planta · esmeralda: mestre de obras", "Wrench: blueprint · emerald: master builder"),
 ("veio.center.project.paid", "%s: paga · faça o canteiro", "%s: paid · set up a site"),
 ("veio.project.building", "Paga, a obra vira prédio num canteiro de obras.", "Once paid, it becomes a building at a construction site."),
 ("veio.center.kind.10", "Canteiro", "Site"),
]:
 label(key, pt, en)


components={
 "copper_plate":("Feita na prensa com cobre bruto. Vai para engrenagens e fiação.","Press raw copper. Used for gears and wiring."),
 "iron_plate":("Feita na prensa com ferro bruto. Vai para molduras e aço.","Press raw iron. Used for frames and steel."),
 "gear":("Duas chapas de cobre no montador. É o começo das melhorias.","Two copper plates in an assembler. Starts the upgrades."),
 "frame":("Chapa de ferro na cortadora. Vai para módulos mecânicos.","Iron plate in a cutter. Used for mechanical modules."),
 "module":("Moldura e engrenagem no integrador. Usado na expansão.","Frame and gear in an integrator. Used for expansion."),
 "steel_plate":("Duas chapas de ferro e carvão no forno de ligas.","Two iron plates and coal in an alloy furnace."),
 "wire":("Chapa de cobre na cortadora rende duas. Usada em circuitos e cabos.","A copper plate in a cutter makes two. Used for circuits and cables."),
 "silicon":("Quartzo do Nether no triturador. Usado em circuitos.","Nether quartz in a crusher. Used for circuits."),
 "circuit":("Duas fiações e um silício no montador de circuitos.","Two copper wiring and one silicon in a circuit assembler."),
}
components.update({
 "route_card":("Use no terminal para escolher canal 0-16; agachado volta um. Dois terminais do mesmo tipo por canal. Não gasta o cartão.","Use on a terminal to select channel 0-16; sneak goes back one. Two terminals of the same type per channel. Card is kept."),
 "flour":("Trigo no moinho. Vai para pão, biscoito e bolo na cozinha.","Wheat in the mill. Used for bread, cookies and cake."),
 "biomass":("Plantas ou troncos no moinho. Combustível do gerador a biomassa.","Plants or logs in the mill. Fuel for the biomass generator."),
 "repair_kit":("Botão direito numa máquina danificada: recupera 50% da integridade.","Right-click a damaged machine: restores 50% integrity."),
 "speed_module":("Botão direito numa máquina: ciclo 40% mais curto, mais energia e poluição.","Right-click a machine: 40% shorter cycle, more power and pollution."),
 "efficiency_module":("Botão direito numa máquina: 40% menos energia e metade da poluição.","Right-click a machine: 40% less power and half the pollution."),
 "productivity_module":("Botão direito numa máquina: +20% de produtos, ciclo mais lento.","Right-click a machine: +20% products, slower cycle."),
})
for name,texts in components.items():label(f"veio.componenthelp.{name}",*texts)

pages=[None] # Contents filled after page numbers are assigned.
def page(pt,en):
 key=f"veio.guide.page.{len(pages)+1:02}"
 label(key,pt,en)
 pages.append({"translate":key})
 return len(pages)
intro=page(
 "Manual de consulta\n\nPara aprender automação, use Sua primeira oficina: livro + pedregulho.\n\nO Quadro da fábrica acompanha fases e entregas. Aqui ficam máquinas, energia, logística e receitas.",
 "Technical reference\n\nTo learn automation, use Your first workshop: book + cobblestone.\n\nThe Factory board tracks phases and deliveries. Find machines, power, logistics and recipes here.")
troubleshoot=page(
 "Se a máquina parar\n\nSem energia: confira carvão e cabos.\n\nSem jazida: confira o bloco abaixo.\n\nFalta material: veja os slots.\n\nSaída cheia: libere espaço no destino.",
 "When a machine stops\n\nNo power: check fuel and cables.\n\nNo ore: check the block underneath.\n\nMissing material: inspect the slots.\n\nOutput full: make room downstream.")
page(
 "Controles e ajuda\n\nBotão direito abre o painel.\n\nChave + botão direito gira.\n\nPasse o cursor no ? ou slot vazio para ajuda.\n\nRedstone pausa a produção.",
 "Controls and help\n\nEmpty hand + right-click opens the panel.\n\nWrench + right-click turns it.\n\nHover over ? or an empty slot for help.\n\nPowered redstone pauses production.")
page(
 "Seu roteiro\n\nAbra Avanços > Primeiros pedidos. Os marcos contam materiais obtidos.\n\nRecolha os produtos para marcar progresso.\n\nO painel de cada máquina confirma seu funcionamento.",
 "Your checklist\n\nOpen Advancements and find First requests. Milestones track obtained materials.\n\nPick up products to mark progress.\n\nInspect machine panels to confirm operation.")
wiring=page(
 "Postes e fios\n\nCada poste alimenta máquinas a até 2 blocos (aço: 3) e liga aos vizinhos.\n\nMadeira: fio de 8, 120 FE/t.\nAço: fio de 16, 400 FE/t.\n\nSem energia longe? O fio é o gargalo.",
 "Poles and wires\n\nEach pole powers machines within 2 blocks (steel: 3) and links to nearby poles.\n\nWood: 8-block wire, 120 FE/t.\nSteel: 16-block wire, 400 FE/t.\n\nStarving far away? The wire is the limit.")
smog=page(
 "Poluição\n\nMáquinas trabalhando poluem o chunk; o painel mostra quanto por minuto. A poluição se espalha e some aos poucos.\n\nLavouras colhidas a absorvem. Módulos de eficiência cortam pela metade.",
 "Pollution\n\nWorking machines pollute their chunk; the panel shows how much per minute. It spreads and slowly fades.\n\nHarvested crops absorb it. Efficiency modules halve it.")
page(
 "Saqueadores\n\nO chat avisa um dia antes. Na noite seguinte vêm com som de invasão, atrás de você e dos aldeões.\n\nQuatro golpes param uma máquina ou matam um aldeão. Muros, espada, quartel e torre Tesla.",
 "Raiders\n\nChat warns a day ahead. The next night they come with a raid horn, for you and the villagers.\n\nFour hits stop a machine or kill a villager. Walls, a sword, barracks and Tesla towers.")
upgrades=page(
 "Módulos\n\nBotão direito numa máquina instala um. Chave agachado retira.\n\nVelocidade: mais produção, mais energia e poluição.\nEficiência: menos energia e poluição.\nProdutividade: +20% de produtos.",
 "Modules\n\nRight-click a machine to install one. Sneak with the wrench to remove it.\n\nSpeed: more output, power and pollution.\nEfficiency: less power and pollution.\nProductivity: +20% products.")
world=page(
 "Fábrica e mundo\n\nColheitadeira: plante ao redor dela e ligue uma esteira.\n\nTorre Tesla: protege a base à noite. Junte acumuladores ao solar.\n\nOssos dos monstros viram farinha no triturador.",
 "Factory and world\n\nHarvester: build the farm around it and belt the crops away.\n\nTesla tower: guards the base at night. Pair solar with batteries.\n\nMonster bones become bone meal in the crusher.")
page(
 "Comércio\n\nDepois do projeto do povoado, a central exporta sem fim: 4 circuitos e 4 chapas de aço viram 4 esmeraldas e 2 frascos de experiência.",
 "Trade\n\nAfter the village project, the hub keeps exporting: 4 circuits and 4 steel plates become 4 emeralds and 2 experience bottles.")
production=page(
 "Escada de produção\n\nCada melhoria resolve um gargalo e revela o próximo: jazida, energia, fio, poluição.\n\nVeio conectado rende mais: 7+ blocos de minério ligados dobram a velocidade, 16+ quadruplicam.",
 "Production ladder\n\nEach upgrade fixes one bottleneck and reveals the next: vein, power, wire, pollution.\n\nConnected veins pay off: 7+ linked ore blocks double the speed, 16+ quadruple it.")
page(
 "Farol opcional\n\nDepois do povoado, esvazie a central. Ponha 1 estrela do Nether na entrada 1 e depois 48 circuitos na 2.\n\nGanhe 1 farol e 2 extratoras Mk.II. Retire as saídas: depois vem o grande projeto.",
 "Optional beacon\n\nAfter the village project, empty the hub inputs. Put 1 Nether star in input 1 first, then 48 circuits in input 2.\n\nReceive 1 beacon and 2 Mk.II extractors. Clear the outputs: the grand project comes next.")
page(
 "Energia\n\nRoda d'água e cata-vento: limpos, 8 FE/t.\nGerador e biomassa: 40 FE/t (biomassa polui menos).\nVapor: 120 FE/t, com água.\nGeotérmico e solar escalam.\nAcumuladores seguram a noite.",
 "Power\n\nWater wheel and wind: clean, 8 FE/t.\nGenerator and biomass: 40 FE/t; biomass pollutes less.\nSteam: 120 FE/t, needs water.\nGeothermal and solar scale up.\n\nBatteries carry the night.")
water=page(
 "Água e irrigação\n\nBomba ao lado de água parada enche os canos; eles levam a água ao aspersor e ao vapor.\n\nCada planta bebe por estágio: cenoura pouco, beterraba muito. Sem água, a lavoura cresce devagar.",
 "Water and irrigation\n\nA pump by still water fills the pipes. Pipes carry water to sprinklers and steam engines.\n\nEach crop drinks per stage: carrots little, potatoes more, beetroot a lot. Without water, crops grow as usual, slowly.")
page(
 "Rios e represas\n\nNo mundo novo, nascentes nas montanhas formam rios que descem ao mar, engrossam e caem.\n\nFeche um vale e ponha a turbina na parede: mais queda, lago largo ou rio grande rendem mais.",
 "Rivers and dams\n\nIn new terrain, springs high in the mountains form rivers that run to the sea, join and grow, with falls on the way.\n\nClose a valley and set the turbine in the wall: a higher drop and a wider lake pay more. So does a big river.")
food=page(
 "Comida e povoado\n\nTrigo > moinho > farinha > cozinha > pão.\nCarnes do rancho > fornalha elétrica.\n\nO armazém paga esmeraldas pela comida, mais por comida pronta, e 50% a mais com aldeões perto.",
 "Food and village\n\nWheat > mill > flour > kitchen > bread.\nRanch meat > electric furnace.\n\nThe pantry pays emeralds for food, more for cooked food, and 50% more with villagers near.")
page(
 "A vila operária\n\nAldeões contratados trabalham por você: lavoura, animais, máquinas, armazéns e um centro de controle.\n\nTudo isso está no Guia da Vila Operária, entregue ao obter um posto. Livro + trigo fabricam outro.",
 "The worker village\n\nHired villagers work for you: fields, animals, machines, storage and a control center.\n\nIt is all in the Worker Village Guide, given when you obtain a post. Book + wheat craft another.")
help["belt"] = (
 "Anda para longe de você.\nLeva 5 minérios ou 1 produto.\nNada entra pela frente.\nChave fixa a direção.\nAgachar + chave: automático.",
 "Runs away from you.\nCarries 5 ores or 1 product.\nNothing enters its front.\nWrench locks the direction.\nSneak + wrench: automatic.")
help["fast_belt"] = (
 "Mesmas curvas e rampas.\nTransporta até 5 itens/s.\nUse na saída do unificador.\nChave fixa a direção.",
 "Same turns and ramps.\nCarries up to 5 items/s.\nUse after a merger.\nWrench locks direction.")
for name in ("belt", "fast_belt"):
 label(f"veio.help.{name}", *help[name])
page(
 "Entrada e saída\n\nVERDE: a máquina recebe.\nVERMELHO: a produção sai.\n\nEsteira: nada entra pela frente. Leva 5 minérios ou 1 produto (rápida 2, expressa 4).",
 "Input and output\n\nGREEN (front and sides): the machine takes items.\nRED (back): products leave; nothing enters.\n\nA belt accepts nothing from its front and holds up to 5 ores or 1 finished product (fast 2, express 4).")
page(
 "Curvas de esteira\n\nConecte uma entrada e uma saída vizinhas. A curva se ajusta sozinha.\n\nEm cruzamentos, use a chave para fixar a saída. Agache com a chave para voltar ao automático.",
 "Conveyor turns\n\nConnect one incoming and one outgoing belt. The corner adjusts automatically.\n\nAt junctions, use the wrench to lock the outlet. Sneak with it to restore automatic routing.")
page(
 "Subir e descer\n\nColoque esteiras sobre blocos em degraus, com diferença de 1 bloco.\n\nA peça mais baixa vira rampa. Deixe espaço acima.\n\nMáquinas, divisores e unificadores conectam na mesma altura.",
 "Up and down\n\nPlace belts on stair-stepped blocks, one block of height per step.\n\nThe lower belt becomes a ramp. Leave headroom.\n\nMachines, splitters and mergers connect at the same height.")
help["splitter"] = (
 "Equilibra duas saídas.\nAgachar + chave: prioridade.\nAgachar + item: filtro.\nIguais à esquerda; outros à direita.\nMão vazia agachado: limpar.",
 "Balances two outlets.\nSneak + wrench: priority.\nSneak + item: sorting filter.\nMatching left; other right.\nSneak + empty hand: clear.")
help["merger"] = (
 "Duas entradas, uma saída.\nAgachar + chave: prioridade\nequilibrada, A ou B.\nVazão até 5 itens/s.\nRedstone pausa o fluxo.",
 "Two inputs, one outlet.\nSneak + wrench: balanced,\nA or B priority.\nUp to 5 items/s.\nRedstone pauses the flow.")
for name in ("splitter", "merger"):
 label(f"veio.help.{name}", *help[name])
page(
 "Megafábricas\n\nCopiadora: monte a linha no quadrado acima dela e clique com papel.\nMarco de obra: mão vazia mostra o orçamento; agachado com a chave, constrói.\n\nAndares: escada de mão até o piso de cima, ou elevador de carga.",
 "Megafactories\n\nCopier: build a line on the square above it and click with paper.\nBuild marker: empty hand lists materials; sneak with the wrench to build.\n\nFloors: a ladder up to the next floor, or a cargo lift.")
page(
 "Configurar filtros\n\nAgache e use um item no braço ou divisor: ele vira filtro sem gastar a amostra.\n\nAgache com mão vazia para limpar. O ícone fica no bloco.",
 "Set a filter\n\nSneak and use an item on an arm or splitter to copy its filter. The sample is kept.\n\nSneak with an empty hand to clear. See the icon on the block.")
page(
 "Prioridades\n\nAgache com a chave no divisor ou unificador: equilibrado, esquerda/A, direita/B.\n\nPrioridade usa a outra via se precisar. O filtro nunca mistura vias.",
 "Priorities\n\nSneak with a wrench on a splitter or merger: balanced, left/A, right/B.\n\nPriority falls back if needed. Sorting filters never mix lanes.")
page(
 "Parar e medir\n\nRedstone pausa braços e esteiras. Esteiras reagem em até 8 ticks.\n\nPainéis mostram itens/min. Divisor filtrado: iguais à esquerda, outros à direita.",
 "Stop and measure\n\nRedstone pauses arms and belts. Belts respond within 8 ticks.\n\nPanels show items/min. A sorting splitter sends matching left, other right.")
category_pages=[]
category_parts=[]
for _,_,members in categories:
 parts=[]
 for start in range(0,len(members),10):
  parts.append((len(pages)+1,members[start:start+10]));pages.append(None)
 category_parts.append(parts);category_pages.append(parts[0][0])
label("veio.guide.more","Mais >","More >")
recipe_index=len(pages)+1;pages.append(None)
references={}
def reference_labels(name):
 """Uso e receita de um bloco, nos dois idiomas (consulta dos dois guias)."""
 title={code:item_name("veio:"+name,code) for code in langs}
 usage_key=f"veio.guide.use.{name}"
 recipe=read(f"{D}/recipe/{name}.json")
 ingredients=Counter(recipe["ingredients"])
 made=recipe["result"].get("count",1)
 for code in langs:
  pt=code=="pt_br"
  langs[code][usage_key]=help[name][0 if pt else 1]
  lines=[("Fabricar " if pt else "Craft ")+title[code],""]
  lines += [f"{count} x {item_name(item,code)}" for item,count in ingredients.items()]
  lines += ["",("Resultado: " if pt else "Makes: ")+str(made)]
  langs[code][f"veio.guide.make.{name}"]="\n".join(lines)
  parts=", ".join(f"{count} {item_name(item,code)}" for item,count in ingredients.items())
  langs[code][f"veio.guide.craft.{name}"]=("Fabricar: " if pt else "Craft: ")+parts+(f" ({made})" if made>1 else "")
for name in order:
 kind="item" if name=="wrench" else "block"
 title={code:item_name("veio:"+name,code) for code in langs}
 usage_key=f"veio.guide.use.{name}"
 recipe=read(f"{D}/recipe/{name}.json")
 ingredients=Counter(recipe["ingredients"])
 made=recipe["result"].get("count",1)
 recipe_key=f"veio.guide.make.{name}"
 craft_key=f"veio.guide.craft.{name}"
 for code in langs:
  pt=code=="pt_br"
  langs[code][usage_key]=help[name][0 if pt else 1]
  lines=[("Fabricar " if pt else "Craft ")+title[code],""]
  lines += [f"{count} x {item_name(item,code)}" for item,count in ingredients.items()]
  lines += ["",("Resultado: " if pt else "Makes: ")+str(made)]
  langs[code][recipe_key]="\n".join(lines)
  parts=", ".join(f"{count} {item_name(item,code)}" for item,count in ingredients.items())
  langs[code][craft_key]=("Fabricar: " if pt else "Craft: ")+parts+(f" ({made})" if made>1 else "")
 references[name]=len(pages)+1
 pages.append({"text":"","extra":[{"translate":f"{kind}.veio.{name}","bold":True},back_mark(),{"text":"\n"},
  {"translate":usage_key},{"text":"\n"},{"translate":craft_key,"color":"dark_gray"}]})
for g,(pt,en,members) in enumerate(categories):
 key=f"veio.guide.category.{g}"
 label(key,pt,en)
 for part,(target,chunk) in enumerate(category_parts[g]):
  extra=[{"translate":key,"bold":True},back_mark(),{"text":"\n"}]
  for name in chunk:
   kind="item" if name=="wrench" else "block"
   short_key=f"veio.guide.short.{name}"
   title_key=short_key if short_key in langs["pt_br"] else f"{kind}.veio.{name}"
   extra.extend([{"translate":title_key,"color":"dark_blue","underlined":True,
                  "click_event":{"action":"change_page","page":references[name]}},{"text":"\n"}])
  if part+1<len(category_parts[g]):
   extra.append({"translate":"veio.guide.more","color":"dark_blue","click_event":{"action":"change_page","page":category_parts[g][part+1][0]}})
  pages[target-1]={"text":"","extra":extra}

# Receitas das máquinas, geradas de tools/production.json. Os nomes vanilla usam as chaves do
# próprio jogo, então aparecem no idioma do jogador.
production_data=json.loads((root.parent/"tools/production.json").read_text(encoding="utf-8"))
BLOCK_ITEMS={"sand","red_sand","gravel","cobblestone","stone","smooth_stone","cobbled_deepslate","deepslate",
 "glass","netherrack","cactus","wet_sponge","sponge","ancient_debris","clay","pumpkin","brown_mushroom",
 "red_mushroom","sugar_cane","kelp","cake","coal_block","dried_kelp_block","hay_block","bamboo"}
label("veio.guide.group.logs","Qualquer tronco","Any log")
label("veio.guide.group.plants","Plantas, folhas ou mudas","Plants, leaves or saplings")
label("veio.guide.group.coal","Carvão","Coal")
label("veio.guide.recipes","Receitas das máquinas","Machine recipes")
label("veio.guide.alt","alternativa","alternate")
label("veio.guide.per_minute"," /min · %ss"," /min · %ss")
label("veio.metric.utilization_max","Tempo ativo %s%% · máx. %s ciclos/min","Active %s%% · max %s cycles/min")
def name_component(item):
 if item.startswith("@"):
  return {"translate":f"veio.guide.group.{item[1:]}"}
 ns,path=item.split(":")
 if ns=="veio":
  return {"translate":f"item.veio.{path}"}
 return {"translate":f"{'block' if path in BLOCK_ITEMS else 'item'}.minecraft.{path}"}
machines=[]
for recipe in production_data["recipes"]:
 if recipe["machine"] not in machines: machines.append(recipe["machine"])
recipe_pages={}
# Receitas por página: 4, menos onde os nomes longos quebram linha (14 linhas por página).
RECIPES_PER_PAGE={"electronics":2,"kitchen":3}
for machine in machines:
 entries=[r for r in production_data["recipes"] if r["machine"]==machine]
 PER_PAGE=RECIPES_PER_PAGE.get(machine.split(":")[-1],4)
 for start in range(0,len(entries),PER_PAGE):
  chunk=entries[start:start+PER_PAGE]
  extra=[{"translate":f"block.veio.{machine}","bold":True}]
  if len(entries)>PER_PAGE:
   extra.append({"text":f" {start//PER_PAGE+1}/{(len(entries)+PER_PAGE-1)//PER_PAGE}"})
  extra.append({"text":"\n"})
  for r in chunk:
   extra.append({"text":"\n"})
   # Como no Satisfactory: cada item com o ritmo de uma máquina sem módulo, em itens por
   # minuto, e o ciclo no fim. Duas prensas de 40/min enchem um montador que pede 80/min.
   def rate(count):
    per_minute=count*1200/r["ticks"]
    return f"{per_minute:.0f}" if per_minute>=10 else f"{per_minute:.1f}".replace(".",",").replace(",0","")
   for i,(item,count) in enumerate(r["in"]):
    if i: extra.append({"text":" + "})
    extra.extend([{"text":f"{rate(count)} "},name_component(item)])
   extra.extend([{"text":" > ","color":"dark_gray"},{"text":f"{rate(r['out'][1])} "},name_component(r["out"][0])])
   seconds=r["ticks"]/20
   cycle=f"{seconds:.0f}" if seconds==int(seconds) else f"{seconds:.1f}".replace(".",",")
   extra.append({"translate":"veio.guide.per_minute","with":[cycle],"color":"dark_gray"})
   if r.get("alt"):
    extra.extend([{"text":" ("},{"translate":"veio.guide.alt","italic":True},{"text":")"}])
  extra.extend([{"text":"\n"},{"translate":"veio.guide.recipes","color":"dark_blue","click_event":{"action":"change_page","page":recipe_index}}])
  recipe_pages.setdefault(machine,len(pages)+1)
  pages.append({"text":"","extra":extra})
extra=[{"translate":"veio.guide.recipes","bold":True},{"text":"\n"}]
for machine in machines:
 extra.extend([{"translate":f"block.veio.{machine}","color":"dark_blue","underlined":True,
  "click_event":{"action":"change_page","page":recipe_pages[machine]}},{"text":"\n"}])
extra.append({"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}})
pages[recipe_index-1]={"text":"","extra":extra}
# O sumário cabe numa página: 13 entradas. "Resolver falhas" e "Fábrica e mundo" seguem na leitura.
links=[
 ("Começar","Start here",intro),
 ("Escada de produção","Production ladder",production),("Comida e povoado","Food and village",food),
 ("Postes e fios","Poles and wires",wiring),("Poluição e defesa","Pollution and defense",smog),
 ("Módulos","Modules",upgrades),
]+[(pt,en,category_pages[g]) for g,(pt,en,_) in enumerate(categories)]+[("Receitas","Recipes",recipe_index)]
extra=[{"translate":"veio.guide.title","bold":True},{"text":"\n"}]
for n,(pt,en,target) in enumerate(links):
 key=f"veio.guide.contents.{n}";label(key,pt,en)
 extra.extend([{"translate":key,"color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":target}},{"text":"\n"}])
pages[0]={"text":"","extra":extra}
book={"minecraft:written_book_content":{"title":{"raw":"Veio 0.31.0"},"author":"Veio",
 "generation":0,"resolved":True,"pages":[{"raw":p} for p in pages]},
 "minecraft:custom_name":{"translate":"veio.guide.title","italic":False}}
write(f"{D}/loot_table/tutorial/guide.json",{"type":"minecraft:generic",
 "pools":[{"rolls":1,"entries":[{"type":"minecraft:item","name":"minecraft:written_book",
 "functions":[{"function":"minecraft:set_components","components":book}]}]}]})
write(f"{D}/recipe/guide.json",{"type":"minecraft:crafting_shapeless","category":"misc",
 "ingredients":["minecraft:book","minecraft:paper"],
 "result":{"id":"minecraft:written_book","count":1,"components":book}})
from build_first_steps import build as build_first_steps
build_first_steps(label, read, write, A, D)
label("veio.tutorial.title","Primeiros pedidos","First requests")
label("veio.tutorial.start","A oficina precisa de 4 chapas. Abra Sua primeira oficina e monte seu primeiro ciclo.","The workshop needs 4 plates. Open Your first workshop and build your first cycle.")
start={"display":{"icon":{"id":"minecraft:written_book"},"title":{"translate":"veio.tutorial.title"},
 "description":{"translate":"veio.tutorial.start"},"background":"minecraft:gui/advancements/backgrounds/stone",
 "announce_to_chat":False},
 "criteria":{"welcome":{"trigger":"minecraft:tick"}},
 "rewards":{"loot":["veio:tutorial/first_steps","veio:tutorial/board"],"recipes":["veio:first_steps","veio:guide","veio:generator","veio:cable","veio:drill","veio:belt","veio:press","veio:factory_board"],
 "function":"veio:tutorial/welcome"}}
write(f"{D}/advancement/tutorial/start.json",start)
label("veio.guide.path_welcome", "Esteiras são uma opção: extratora > esteira > prensa na mesma altura. Braços, funis e carregadores também contam no tutorial.", "Belts are one option: extractor > belt > press at one height. Arms, hoppers and haulers also count in the tutorial.")
write(f"{D}/loot_table/tutorial/board.json",{"type":"minecraft:generic","pools":[{"rolls":1,"entries":[{"type":"minecraft:item","name":"veio:factory_board"}]}]})
write(f"{D}/advancement/tutorial/belt_paths.json", {
 "parent":"veio:tutorial/start",
 "criteria":{"obtain":{"trigger":"minecraft:inventory_changed","conditions":{"items":[{"items":"veio:belt"}]}}},
 "rewards":{"function":"veio:tutorial/path_welcome"}})
p=root/D/"function/tutorial/path_welcome.mcfunction"
p.parent.mkdir(parents=True,exist_ok=True)
p.write_text('tellraw @s {"translate":"veio.guide.path_welcome","color":"gold"}\n',encoding="utf-8")
write(f"{D}/advancement/tutorial/merger.json", {
 "parent":"veio:tutorial/start",
 "display":{"icon":{"id":"veio:merger"},"title":{"translate":"veio.tutorial.merger.title"},
 "description":{"translate":"veio.tutorial.merger.description"},"announce_to_chat":False},
 "criteria":{"obtain":{"trigger":"minecraft:inventory_changed","conditions":{"items":[{"items":"veio:merger"}]}}},
 "rewards":{}})
p=root/D/"function/tutorial/welcome.mcfunction";p.parent.mkdir(parents=True,exist_ok=True)
p.write_text('tellraw @s {"translate":"veio.guide.welcome","color":"gold"}\n',encoding="utf-8")
stages=[
 ("bread","minecraft:bread",3,"Opcional: reserva de comida","Optional: food reserve","Junte 3 pães; replante o trigo. A reserva continua com você.","Gather 3 bread; replant the wheat. Keep your reserve."),
 ("generator","veio:generator",1,"1. Energia para a oficina","1. Workshop power","Abasteça o gerador e ligue uma prensa com cabos.","Fuel the generator and connect a press with cables."),
 ("press","veio:press",1,"2. Monte o teste","2. Build the test","Gerador > cabo > prensa. Confirme energia; depois coloque cobre bruto.","Generator > cable > press. Confirm power; then add raw copper."),
 ("plates","veio:copper_plate",4,"3. Quatro chapas","3. Four plates","Coloque cobre bruto na prensa e recolha 4 chapas. Leve à mão.","Put raw copper in the press and collect 4 plates. Carry them by hand."),
 ("gears","veio:gear",1,"4. Primeira engrenagem","4. First gear","Duas chapas no montador energizado fazem uma engrenagem.","Two plates in a powered assembler make a gear."),
 ("hub","veio:contract_hub",1,"5. A encomenda da oficina","5. The workshop order","Abra a central: 24 chapas e 8 engrenagens. Esteiras podem poupar viagens.","Open the hub: 24 plates and 8 gears. Belts can save trips."),
 ("drill","veio:drill",1,"6. Uma tarefa a menos","6. One less chore","Coloque sobre cobre, ligue energia e confira a saída antes de conectar esteiras.","Place on copper, connect power and check its output before adding belts."),
 ("stock","veio:copper_plate",24,"7. Reserva da oficina","7. Workshop stock","Tenha também 8 engrenagens. Entregue na central para receber esteiras e cabos.","Also hold 8 gears. Deliver to the hub to receive belts and cables."),
 ("frames","veio:frame",1,"8. Produzir molduras","8. Make frames","A cortadora transforma chapas de ferro em molduras.","A cutter turns iron plates into frames."),
 ("modules","veio:module",1,"9. Juntar as linhas","9. Join the lines","Moldura e engrenagem no integrador: expanda no seu ritmo.","Frame and gear in the integrator: expand at your pace."),
 ("repair","veio:repair_kit",1,"Preparar reparos","Prepare repairs","A poluição atrai Saqueadores. Tenha kits de reparo à mão.","Pollution draws Raiders. Keep repair kits at hand."),
 ("defense","veio:tesla",1,"Defender a fábrica","Defend the factory","Uma torre Tesla abastecida protege a fábrica à noite.","A powered Tesla tower guards the factory at night."),
]
next_hints={
 "bread":("Reserva de comida pronta. Guarde para explorar ou alimentar trabalhadores; o pedido da oficina continua no seu caderno.","Food reserve ready. Keep it for exploring or workers; the workshop request is in your workbook."),
 "generator":("Teste de energia: coloque gerador > cabo > prensa encostados e carvão no gerador. Abra a prensa e confirme a energia subindo. Cabo transporta energia; esteira transporta itens.","Power test: place generator > cable > press touching and fuel the generator. Open the press and confirm rising energy. Cables carry power; belts carry items."),
 "press":("Teste da prensa: ponha 4 cobres brutos na entrada esquerda. Veja o progresso e recolha 4 chapas à direita. Se parar, leia o estado: energia, material ou saída cheia.","Press test: put 4 raw copper in the left input. Watch progress and collect 4 plates on the right. If it stops, read its status: power, material or full output."),
 "drill":("Teste da extração: coloque SOBRE minério de cobre e ligue energia. Confira cobre saindo sozinho. Depois ligue extratora > esteira > prensa e veja o cobre chegar sem sua ajuda.","Extraction test: place ON copper ore and supply power. Check that copper appears on its own. Then connect extractor > belt > press and watch copper arrive without your help."),
 "hub":("Encomenda: 24 chapas + 8 engrenagens. Isso exige 40 chapas ao todo, além das peças de construção. Sua primeira oficina ensina a automatizar coleta, transporte e armazenamento antes da entrega.","Order: 24 plates + 8 gears. That needs 40 plates in total, besides construction parts. Your first workshop teaches automated extraction, transport and storage before delivery."),
 "plates":("Chapas prontas! Faça um montador com 2 chapas, pistão e bancada. Mais 2 chapas na máquina energizada fazem sua primeira engrenagem.","Plates ready! Craft an assembler with 2 plates, a piston and crafting table. Feed it 2 more plates and power for your first gear."),
 "gears":("A oficina precisa de um lote: 24 chapas e 8 engrenagens. Abra uma central de projetos. Experimente automatizar uma viagem de cada vez.","The workshop needs a batch: 24 plates and 8 gears. Open a project hub. Try automating one trip at a time."),
 "stock":("Reserva reunida. A central recebe chapas na entrada 1 e engrenagens na 2. A entrega consome os itens; retire esteiras e cabos das saídas.","Stock gathered. The hub takes plates in input 1 and gears in input 2. Delivery consumes the items; collect belts and cables from its outputs."),
}
parent="veio:tutorial/start"
for name,item,count,pt,en,pdesc,edesc in stages:
 key="veio.tutorial."+name;label(key+".title",pt,en);label(key+".description",pdesc,edesc)
 items=[{"items":item,"count":{"min":count}}]
 if name=="stock": items.append({"items":"veio:gear","count":{"min":8}})
 obj={"parent":"veio:tutorial/start" if name=="bread" else parent,"display":{"icon":{"id":item},"title":{"translate":key+".title"},
 "description":{"translate":key+".description"},"announce_to_chat":False,"show_toast":False},
 "criteria":{"obtain":{"trigger":"minecraft:inventory_changed","conditions":{"items":items}}}}
 if name in next_hints:
  label(key+".next",*next_hints[name])
 write(f"{D}/advancement/tutorial/{name}.json",obj)
 if name!="bread": parent="veio:tutorial/"+name

# ---------- Guia da Vila Operária ----------
# Segundo livro: o guia principal está perto do limite de 100 páginas. Roteiro da vila, problemas,
# comida e consulta dos blocos da vila. Entregue pela aba de avanços "Vila operária".
vpages=[None]
def vpage(pt,en):
 key=f"veio.village.page.{len(vpages)+1:02}"
 label(key,pt,en)
 vpages.append({"translate":key})
 return len(vpages)
v_intro=vpage(
 "A vila operária\n\nVocê começa no centro de uma vila. Os aldeões passam a trabalhar por você, e a vila cresce com a fábrica.\n\nSiga os passos em ordem. A aba de avanços Vila operária marca o progresso.",
 "The worker village\n\nYou start at the center of a village. Villagers start working for you, and the village grows with the factory.\n\nFollow the steps in order. The Worker Village advancement tab tracks progress.")
v_hire=vpage(
 "1. Primeiro trabalhador\n\nPosto do lavrador perto da plantação e de aldeões.\n\nPão no slot da esquerda; clique no posto com uma esmeralda.\n\nO aldeão colhe e traz a colheita.",
 "1. First worker\n\nFarmhand post near crops and villagers.\n\nBread in the left slot; click the post with an emerald.\n\nThe villager harvests and brings the crop back.")
v_field=vpage(
 "2. Demarque a lavoura\n\nColoque o marco de lavoura no centro do campo, com água perto.\n\nPostos a até 3 blocos da borda trabalham na área toda: aram, plantam e colhem.\n\nSemente no marco: só ela é plantada.",
 "2. Mark the field\n\nPlace the field marker at the center of the field, with water nearby.\n\nPosts within 3 blocks of the border work the whole area: till, plant and harvest.\n\nSeed in the marker: only that is planted.")
v_pen=vpage(
 "3. Cercados\n\nCerque os animais e ponha o marco de cercado no meio. Um posto do pastor perto cuida deles.\n\nRação no marco define o cercado: cenoura para porcos, sementes para galinhas, trigo para vacas e ovelhas.",
 "3. Pens\n\nFence the animals and put the pen marker in the middle. A nearby shepherd post tends them.\n\nFeed in the marker sets the pen: carrots for pigs, seeds for chickens, wheat for cows and sheep.")
v_store=vpage(
 "4. Celeiro e silo\n\nO celeiro guarda qualquer item; o silo, grãos e comida.\n\nFunis põem e tiram itens. Com um capataz por perto, as sobras vão para eles sozinhas.",
 "4. Barn and silo\n\nThe barn keeps any item; the silo, grain and food.\n\nHoppers insert and extract. With a foreman nearby, surplus goes there by itself.")
v_haul=vpage(
 "5. Carregadores\n\nO posto do carregador abastece máquinas a até 6 blocos com os insumos da esquerda e traz os produtos para a direita.\n\nEle não mexe em esteiras.",
 "5. Haulers\n\nThe hauler post feeds machines within 6 blocks from the left slots and brings products to the right.\n\nIt leaves belts alone.")
v_foreman=vpage(
 "6. Capataz e fila\n\nA mesa do capataz cuida de 6 postos a até 12 blocos: leva comida e recontrata com esmeraldas.\n\nEla monta uma fila de entregas entre postos, máquinas e armazéns, feita pelos carregadores.",
 "6. Foreman and queue\n\nThe foreman desk manages 6 posts within 12 blocks: brings food and rehires with emeralds.\n\nIt queues deliveries between posts, machines and storage, done by haulers.")
v_center=vpage(
 "7. Centro de controle\n\nContrate um administrador. Três páginas (a chave troca): Resumo, Atenção e Obras.\n\nItem num slot de meta: x16 guardados. Livro no centro: relatório. O spawn vai para o centro.",
 "7. Control center\n\nHire an administrator. Three pages (wrench switches): Summary, Attention and Projects.\n\nItem in a goal slot: x16 stored. Book on the center: report. Spawn moves to the center.")
v_focus=vpage(
 "Gerenciar de perto\n\nA página Atenção lista o que parou, da mais urgente. Cursor na linha: o que fazer.\n\nBússola no centro: a barra de ação leva você até lá e explica o que fazer. Resolvido, vem a próxima.",
 "Manage up close\n\nThe Attention page lists what stopped, most urgent first. Hover a line: what to do.\n\nCompass on the center: the action bar takes you there and explains. Once solved, the next one comes.")
v_requests=vpage(
 "Pedidos e prestígio\n\nToda manhã a vila faz 3 pedidos (página 4 do centro). Entregue no slot Obra: esmeraldas para o caixa e +10 de prestígio.\n\nNíveis: Aldeia, Vila (60), Vila próspera (200), Cidade operária (500).",
 "Requests and prestige\n\nEach morning the village makes 3 requests (center page 4). Deliver in the Build slot: emeralds to the treasury and +10 prestige.\n\nTiers: Hamlet, Village (60), Thriving (200), Worker city (500).")
v_decrees=vpage(
 "Decretos e caixa\n\nItem no slot Decreto:\nSino: contrata a 16.\nPapel: tarefa a 1 ponto, sem bebês.\nFeno: tarefa a 3, zonas em dobro, mais bebês.\n\nO caixa paga zonas e obras. Agachar + esmeralda: sacar.",
 "Decrees and treasury\n\nItem in the Decree slot:\nBell: hire within 16.\nPaper: tasks at 1, no babies.\nHay: tasks at 3, zones double, more babies.\n\nThe treasury pays zones and projects. Sneak + emerald: withdraw.")
v_projects2=vpage(
 "Obras maiores\n\nVila: Biblioteca (zonas em dobro) e Correio (4 pedidos, +50%).\nPróspera: Estalagem (teto +20, viajantes).\nCidade: Prefeitura (prestígio +50%, juros).",
 "Bigger projects\n\nVillage: Library (zones double) and Post office (4 requests, +50%).\nThriving: Inn (cap +20, travelers).\nCity: Town hall (prestige +50%, interest).")
v_projects=vpage(
 "Obras da vila\n\nMateriais no slot Obra:\nRefeitório: 4ª tarefa grátis.\nEscola: mais bebês, teto +15.\nFerraria: carregador leva 24.\nMercado: zonas -25%.\nFesta: 2 dias sem gastar comida.",
 "Village projects\n\nMaterials in the Build slot:\nCanteen: every 4th task free.\nSchool: more babies, cap +15.\nSmithy: haulers carry 24.\nMarket: zones -25%.\nFestival: 2 days of free food.")
v_home=vpage(
 "8. Seu lar\n\nPonha o lar dentro da sua casa: você renasce ali.\n\nLenha na lareira, comida na despensa. Tigela no lar: refeição em casa, com efeitos maiores se a comida variar e a casa for confortável.",
 "8. Your home\n\nPlace the hearth inside your house: you respawn there.\n\nFuel in the hearth, food in the pantry. Bowl on the hearth: meal at home, with stronger effects for varied food and a cozy house.")
v_banquet=vpage(
 "Banquete\n\nBolo no lar, com a lareira acesa, uma vez por dia: até 6 aldeões livres vêm jantar na sua casa por um minuto.\n\nVocê vira herói da vila (descontos no comércio) e a vila ganha prestígio.",
 "Banquet\n\nCake on the hearth, with the fire lit, once a day: up to 6 free villagers come to dinner at your house for a minute.\n\nYou become the village hero (trade discounts) and the village gains prestige.")
v_home2=vpage(
 "Uma casa de verdade\n\nConforto: cama, porta, janela, luz, baú, fogão, bancada, enfeites, acabamento.\n\nNoite em casa, lareira acesa: acorda descansado.\n\nMelhore com tijolos, circuitos e aço.",
 "A real house\n\nComfort: bed, door, window, light, chest, stove, workbench, decor, finish.\n\nA night home, hearth lit: wake up rested.\n\nUpgrade with bricks, circuits and steel.")
v_expand=vpage(
 "Expandir as zonas\n\nEsmeralda no marco libera o próximo nível:\n13x13: 8 esm., 32 tarefas, 6 aldeões\n17x17: 16, 128, 12\n21x21: 32, 384, 20\n\nA chave escolhe o tamanho entre os liberados.",
 "Grow the zones\n\nEmerald on the marker unlocks the next level:\n13x13: 8 emer., 32 tasks, 6 villagers\n17x17: 16, 128, 12\n21x21: 32, 384, 20\n\nThe wrench picks the size among unlocked ones.")
v_grow=vpage(
 "A vila cresce\n\nFilhos até o teto: 30, mais 3 por posto com trabalhador, 5 por nível de zona, 2 por armazém e 20 pelo centro de controle.\n\nO prefeito limita: 30 sem prefeitura, depois 60, 100 e 150. Sem cama livre ninguém nasce; com prefeito, também é preciso vaga em casa.",
 "The village grows\n\nChildren up to the cap: 30, plus 3 per staffed post, 5 per zone level, 2 per storage and 20 for the control center.\n\nThe mayor limits it: 30 without a town hall, then 60, 100 and 150. No free bed, no birth; with a mayor, a free home too.")
v_lan=vpage(
 "Na mesma rede\n\nAnfitrião: Esc, Abrir para a LAN, Iniciar mundo LAN.\n\nQuem entra: mesmo Wi-Fi, Multijogador; o mundo aparece na lista. Mesma versão do Veio.\n\nAs máquinas são as do anfitrião.",
 "On your network\n\nHost: Esc, Open to LAN, Start LAN World.\n\nJoin: same Wi-Fi, Multiplayer; the world shows in the list. Same Veio version.\n\nThe machines are the host's.")
v_trouble=vpage(
 "Quando um aldeão para\n\nSem comida: ponha pão.\nAusente: traga-o ou dispense (agachar + chave).\nCaminho obstruído: abra passagem.\nSaída cheia: esvazie ou use um celeiro.\nPasse o cursor no estado do painel para dicas.",
 "When a villager stops\n\nNo food: add bread.\nMissing: bring them back or dismiss (sneak + wrench).\nPath blocked: clear the way.\nOutput full: empty it or use a barn.\nHover over the panel status for tips.")
v_barracks=vpage(
 "Quartel\n\nEsmeralda contrata. Agachar com esmeralda troca a classe (com soldado, gasta a do slot).\n\nMilícia: 7 de dano.\nGuarda: tanque, se cura.\nArqueiro: atira de longe.\nMago: 10 e fere outro.\n\nPlantão à noite, sem fugir. Cada golpe come uma tarefa.",
 "Barracks\n\nAn emerald hires. Sneak with an emerald to change class (with a soldier, spends the slot one).\n\nMilitia: 7 damage.\nGuard: tank, heals.\nArcher: shoots from afar.\nMage: 10 and hits another.\n\nNight duty, never flees. Each blow eats one task.")
v_food=vpage(
 "Comida\n\nCada tarefa gasta 2 pontos. Trigo vale 1, batata e cenoura 1, pão 6 (3 tarefas), carne assada 8 (4 tarefas), torta 10.\n\nComida pronta rende mais: monte moinho e cozinha cedo.",
 "Food\n\nEach task costs 2 points. Wheat is worth 1, potato and carrot 1, bread 6 (3 tasks), cooked meat 8 (4 tasks), pie 10.\n\nCooked food goes further: build a mill and kitchen early.")
v_society=vpage(
 "8. Casas e famílias\n\nCasa: até 4 aldeões a até 10 blocos. Os que trabalham ganham salário; a família come da despensa e compra na loja.\n\nSem comida, sem serviços ou sem renda, o IDH cai. Espalhe bairros: casa apertada vive pior.",
 "8. Homes and families\n\nHouse: up to 4 villagers within 10 blocks. Workers earn wages; the family eats from the pantry and buys at the shop.\n\nWithout food, services or income, HDI drops. Spread districts: crowded homes live worse.")
v_shop=vpage(
 "9. Loja\n\nO comerciante é independente. O que você entrega no estoque ele compra no atacado (75% do preço) e paga com as vendas; sem caixa, fica fiado.\n\nAs casas compram de dia; você compra trocando com ele.",
 "9. Shop\n\nThe merchant is independent. What you deliver to the stock they buy wholesale (75% of price) and pay from sales; without cash, on credit.\n\nHomes buy during the day; you buy by trading with them.")
v_mayor=vpage(
 "10. Prefeito e contas\n\nCompanhia (sua): cofre do centro. Paga seus trabalhadores; recebe o atacado.\nPrefeitura: imposto retido na fonte em cada venda (4%, 8%, 12%, 16%). Paga o prefeito e a assistência aos pobres.",
 "10. Mayor and accounts\n\nCompany (yours): center vault. Pays your workers; receives wholesale.\nTown hall: tax withheld at source on every sale (4%, 8%, 12%, 16%). Pays the mayor and assistance.")
v_agenda=vpage(
 "Agenda e feriados\n\nEstações de 6 dias; o 6º é domingo. Em 6 feriados por ano a vila para e se reúne na praça.\n\nDois dias antes, um visitante vem à prefeitura: clique nele e entregue o pedido no espaço Agenda.",
 "Calendar and holidays\n\n6-day seasons; the 6th is Sunday. On 6 holidays a year the village stops and gathers at the plaza.\n\nTwo days before, a visitor comes to the town hall: click them and deliver in the Agenda slot.")
v_civic=vpage(
 "Ânimo e obras\n\nFesta preparada: ânimo sobe. Feriado sem festa: cai. O ânimo muda o IDH (0,9x a 1,1x).\n\nPedidos de parque, praça ou monumento: construa o bloco no bairro. A prefeitura paga.",
 "Morale and works\n\nPrepared festival: morale up. Empty holiday: down. Morale scales HDI (0.9x to 1.1x).\n\nPark, plaza or monument requests: build the block in the district. The town hall pays.")
v_holding=vpage(
 "Holding\n\nO centro de controle é a sede da companhia. Setas < > trocam a empresa: mineração, manufatura, energia, logística, agro, extrativismo, urbanização, comércio e gestão.\n\nCada aba: unidades, ativas, trabalhadores, produção do dia e o que está parado.",
 "Holding\n\nThe control center is the company headquarters. The < > arrows switch business: mining, manufacturing, power, logistics, agriculture, forestry, urban, retail and management.\n\nEach tab: units, active, workers, daily output and what is stopped.")
v_money=vpage(
 "A moeda\n\nEsmeralda é a moeda: 60 moedas cada. O dinheiro só circula:\nindústria → salários → famílias → lojas → atacado → indústria.\n\nSó entra ou sai em esmeraldas de verdade: depósito, saque e suas compras. O relatório da prefeitura confere a soma.",
 "Money\n\nThe emerald is the currency: 60 coins each. Money only circulates:\nindustry → wages → families → shops → wholesale → industry.\n\nIt only enters or leaves as real emeralds: deposits, cash outs and your purchases. The town hall report checks the sum.")
v_export=vpage(
 "Comércio exterior\n\nConexões externas: porto de carga (no mar ou rio largo) e estação de carga (na fronteira, a 96 blocos da sede). Leve a carga até elas; cada viagem tem capacidade e frete.\n\nQuando falta comida, as lojas importam por elas.",
 "Foreign trade\n\nOutside connections: cargo port (on the sea or a wide river) and freight station (at the frontier, 96 blocks from HQ). Haul cargo to them; each trip has capacity and freight.\n\nWhen food runs short, shops import through them.")
v_market=vpage(
 "Bolsa e contratos\n\nCotação = valor x complexidade x bolsa do dia; cai se você inunda um item.\n\nContratos têm prazo e pagam 50% a mais; o chat avisa. Reputação 1 abre componentes, 3 eletrônicos, 6 módulos. Contrato perdido: -1.",
 "Market and contracts\n\nQuote = value x complexity x the day's market; it drops if you flood an item.\n\nContracts have deadlines and pay 50% more; chat tells you. Reputation 1 opens components, 3 electronics, 6 modules. A lost contract: -1.")
v_prices=vpage(
 "Preços locais\n\nCada loja tem seus preços. Muita procura e pouco estoque: até o dobro. Estoque parado: barateia até 70%. Família sem achar comida empurra o preço.\n\nCada vila tem seu mercado: abasteça onde está caro.",
 "Local prices\n\nEach shop sets its own prices. High demand and low stock: up to double. Idle stock: drops to 70%. Families who find no food push prices up.\n\nEach village has its own market: supply where it is expensive.")
v_phases=vpage(
 "Fases\n\n1 Povoado: zona nível 2, 30 aldeões, 4 casas.\n2 Vila: 12 moradores, 3 famílias, IDH 0,40.\n3 Cidade: 36, 9, IDH 0,55, PIB 400, emprego 60%.\n4 Metrópole: 80, 20, IDH 0,70, Gini 0,45, 3 bairros.\n\nCasas pedem poço (fase 2), poste (3) e loja (4).",
 "Phases\n\n1 Hamlet: zone level 2, 30 villagers, 4 homes.\n2 Village: 12 residents, 3 families, HDI 0.40.\n3 Town: 36, 9, HDI 0.55, GDP 400, jobs 60%.\n4 City: 80, 20, HDI 0.70, Gini 0.45, 3 districts.\n\nHomes need a well (phase 2), pole (3) and shop (4).")
v_numbers=vpage(
 "Indicadores\n\nIDH: comida, moradia, serviços e renda de cada família (0 a 1).\nGini: desigualdade da renda.\nPIB: tarefas feitas e vendas do último dia.\nEmprego: adultos com posto.\n\nLivro na prefeitura: relatório com bairros e pedidos.",
 "Indicators\n\nHDI: food, housing, services and income of each family (0 to 1).\nGini: income inequality.\nGDP: tasks done and sales in the last day.\nEmployment: adults with a post.\n\nBook on the town hall: report with districts and requests.")
import blueprints as _plans
v_site=vpage(
 "Canteiro de obras\n\nPaga a obra no centro, ponha o canteiro onde quer o prédio: a borda azul mostra o tamanho e a seta, a entrada.\n\nChave troca a planta. Contrate um mestre de obras e dê comida: ele ergue o prédio.",
 "Construction site\n\nPay at the center, then place the site where you want the building: the blue border shows its size, the arrow the door.\n\nWrench switches blueprint. Hire a master builder and feed him: he raises it.")
v_plans={}
for _key in ["canteen","school","smithy","market","library","post_office","inn","hall"]:
 _p=_plans.BLUEPRINTS[_key]; _l=_p["layers"]
 _size=(len(_l[0][0]),len(_l[0]),len(_l))
 _res=("\n\nLugares para: "+{"veio:shop":"loja","veio:town_hall":"prefeitura (bloco)"}[_p["reserved"]]) if "reserved" in _p else ""
 _res_en=("\n\nSpots for: "+{"veio:shop":"a shop","veio:town_hall":"the town hall block"}[_p["reserved"]]) if "reserved" in _p else ""
 v_plans[_key]=vpage(f"Planta: {_p['name'][0]}\n\n{_p['about'][0]}\n\n{_size[0]}x{_size[1]}, {_size[2]} de altura.{_res}",
  f"Blueprint: {_p['name'][1]}\n\n{_p['about'][1]}\n\n{_size[0]}x{_size[1]}, {_size[2]} tall.{_res_en}")
v_shortcuts=vpage(
 "Atalhos da fábrica\n\n3 chapas de ferro: balde\n4 chapas + baú: funil\nchapa + tocha: lanternas\n3 chapas de cobre: para-raios\ncircuito + chapa: bússola\ncircuito + ouro: relógio\naço + graveto: 16 trilhos",
 "Factory shortcuts\n\n3 iron plates: bucket\n4 plates + chest: hopper\nplate + torch: lanterns\n3 copper plates: lightning rod\ncircuit + plate: compass\ncircuit + gold: clock\nsteel + stick: 16 rails")
v_house=vpage(
 "Casa do fundador\n\nA vila já vem com ela: painel, o Lar no lugar da lareira, sofá, cama e mural.\n\nO Lar dá a refeição e os bônus de casa.\n\nA primeira cama é a da casa; fora, só mais uma.",
 "Founder's house\n\nThe village comes with it: panel, the Hearth as the fireplace, sofa, bed and mural.\n\nThe Hearth gives home meals and bonuses.\n\nYour first bed is the one at home; outside, just one more.")
v_meals=vpage(
 "Refeições fora de casa\n\nRefeição (assados, pão, torta): trabalho mais rápido e fábricas +5% por 5 min.\n\nBanquete (sopas, cenoura dourada): +10% por 8 min e regeneração.\n\nEm casa, coma no Lar.",
 "Meals away from home\n\nMeal (roasts, bread, pie): faster work, factories +5% for 5 min.\n\nFeast (stews, soups, golden carrot): +10% for 8 min, regeneration.\n\nAt home, eat at the Hearth.")
v_blocks=len(vpages)+1;vpages.append(None)
v_manage=len(vpages)+1;vpages.append(None)
v_reference=len(vpages)+1;vpages.append(None)
v_planlist=len(vpages)+1;vpages.append(None)
vrefs={}
for name in village_order:
 reference_labels(name)
 vrefs[name]=len(vpages)+1
 vpages.append({"text":"","extra":[{"translate":f"block.veio.{name}","bold":True},back_mark(),{"text":"\n"},
  {"translate":f"veio.guide.use.{name}"},{"text":"\n"},{"translate":f"veio.guide.craft.{name}","color":"dark_gray"}]})
label("veio.village.blocks","Blocos da vila","Village blocks")
extra=[{"translate":"veio.village.blocks","bold":True},back_mark(),{"text":"\n"}]
for name in village_order:
 extra.extend([{"translate":f"block.veio.{name}","color":"dark_blue","underlined":True,
  "click_event":{"action":"change_page","page":vrefs[name]}},{"text":"\n"}])
vpages[v_blocks-1]={"text":"","extra":extra}
def subindex(page,key,pt,en,links):
 label(key,pt,en)
 extra=[{"translate":key,"bold":True},{"text":"\n"}]
 for n,(lpt,len_,target) in enumerate(links):
  k=f"{key}.{n}";label(k,lpt,len_)
  extra.extend([{"translate":k,"color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":target}},{"text":"\n"}])
 extra.append({"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}})
 vpages[page-1]={"text":"","extra":extra}
MANAGE=[("Começar","Start here",v_intro),("Holding","Holding",v_holding),("A moeda","Money",v_money),("Agenda e feriados","Calendar",v_agenda),("Obras cívicas","Civic works",v_civic),("Comércio exterior","Foreign trade",v_export),("Bolsa e contratos","Market and contracts",v_market),("Preços","Prices",v_prices),("Foco e obras","Focus and projects",v_focus),("Obras da vila","Village projects",v_projects),
 ("Pedidos e prestígio","Requests and prestige",v_requests),("Decretos e caixa","Decrees and treasury",v_decrees),
 ("Obras maiores","Bigger projects",v_projects2),("Fases e indicadores","Phases and indicators",v_phases)]
REFERENCE=[("Expandir zonas","Grow zones",v_expand),("A vila cresce","Village growth",v_grow),("Quartel","Barracks",v_barracks),
 ("Uma casa de verdade","A real house",v_home2),("Banquete","Banquet",v_banquet),("Problemas e comida","Problems and food",v_trouble),
 ("Casa do fundador","Founder's house",v_house),("Refeições fora","Meals away",v_meals),("Na mesma rede","On your network",v_lan),
 ("Atalhos da fábrica","Factory shortcuts",v_shortcuts),("Blocos","Blocks",v_blocks)]
MANAGE.append(("Plantas das obras","Project blueprints",v_planlist))
subindex(v_planlist,"veio.village.plans","Plantas das obras","Project blueprints",[("Canteiro de obras","Construction site",v_site)]+
 [(_plans.BLUEPRINTS[k]["name"][0],_plans.BLUEPRINTS[k]["name"][1],v_plans[k]) for k in ["canteen","school","smithy","market","library","post_office","inn","hall"]])
subindex(v_manage,"veio.village.manage","Gestão da vila","Running the village",MANAGE)
subindex(v_reference,"veio.village.reference","Consulta","Reference",REFERENCE)
vlinks=[("1. Trabalhador","1. Worker",v_hire),("2. Lavoura","2. Field",v_field),
 ("3. Cercados","3. Pens",v_pen),("4. Armazéns","4. Storage",v_store),("5. Carregadores","5. Haulers",v_haul),
 ("6. Capataz","6. Foreman",v_foreman),("7. Centro","7. Center",v_center),("8. Casas","8. Homes",v_society),
 ("9. Loja","9. Shop",v_shop),("10. Prefeito e contas","10. Mayor and accounts",v_mayor),("11. Seu lar","11. Your home",v_home),
 ("Gestão da vila","Running the village",v_manage),("Consulta","Reference",v_reference)]
label("veio.village.title","Guia da Vila Operária","Worker Village Guide")
label("veio.village.steps","Passos:","Steps:")
extra=[{"translate":"veio.village.title","bold":True},{"text":"\n"}]
for n,link in enumerate(vlinks):
 if link is None:
  extra.append({"translate":"veio.village.steps"})
  for step,target in enumerate(vsteps,1):
   extra.extend([{"text":" "},{"text":str(step),"color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":target}}])
  extra.append({"text":"\n"})
  continue
 pt,en,target=link
 key=f"veio.village.contents.{n}";label(key,pt,en)
 extra.extend([{"translate":key,"color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":target}},{"text":"\n"}])
vpages[0]={"text":"","extra":extra}
assert len(vpages)<=100
vbook={"minecraft:written_book_content":{"title":{"raw":"Vila Operária"},"author":"Veio",
 "generation":0,"resolved":True,"pages":[{"raw":p} for p in vpages]},
 "minecraft:custom_name":{"translate":"veio.village.title","italic":False}}
write(f"{D}/loot_table/tutorial/village_guide.json",{"type":"minecraft:generic",
 "pools":[{"rolls":1,"entries":[{"type":"minecraft:item","name":"minecraft:written_book",
 "functions":[{"function":"minecraft:set_components","components":vbook}]}]}]})
write(f"{D}/recipe/village_guide.json",{"type":"minecraft:crafting_shapeless","category":"misc",
 "ingredients":["minecraft:book","minecraft:wheat"],
 "result":{"id":"minecraft:written_book","count":1,"components":vbook}})
# Aba de avanços "Vila operária": o roteiro do livro, marcado ao obter cada bloco.
label("veio.village.tab","Vila operária","Worker Village")
label("veio.village.tab.description","Abra o Guia da Vila Operária e contrate seu primeiro aldeão.","Open the Worker Village Guide and hire your first villager.")
label("veio.village.welcome","Você recebeu o Guia da Vila Operária: aldeões podem trabalhar por você.","You received the Worker Village Guide: villagers can work for you.")
label("veio.village.need_houses","%s aldeões e %s camas. Construa mais casas.","%s villagers and %s beds. Build more houses.")
label("veio.spawn.village","Você está no centro da vila. Ela vai crescer com a sua fábrica.","You are at the village center. It will grow with your factory.")
label("veio.spawn.none","Nenhuma vila encontrada por perto. Construa um centro de controle para fixar o centro da sua vila.","No village found nearby. Build a control center to set your village center.")
label("veio.world.requires_normal","O mapa completo do Veio precisa de geração Normal e estruturas ativadas.","Veio's complete world requires Normal generation with structures enabled.")
label("veio.world.generation_failed","Não foi possível preparar um mundo com todo o conteúdo. Tente criar outro mundo.","Could not prepare a world with all required content. Please try creating another world.")
label("veio.world.portal_blocked","Nenhuma saída segura encontrada. Você permaneceu na origem.","No safe portal exit was found. You stayed at the origin.")
label("veio.world.outside","Esse destino fica além da muralha do mapa.","That destination is beyond the world wall.")
label("veio.home.built","Sua casa está pronta, com o painel em %s: lareira, sofá, cama e o mural com o mapa. Durma lá primeiro.","Your house is ready, with the panel at %s: fireplace, sofa, bed and the map mural. Sleep there first.")
label("veio.home.inside","Em casa, com o painel: as fábricas rendem 15% mais.","At home with the panel: factories produce 15% more.")
label("veio.home.left","Fora de casa: as fábricas voltam ao ritmo normal.","Away from home: factories are back to normal pace.")
label("veio.home.bed_first","Sua primeira cama é a da casa, ao lado do painel. Durma nela antes.","Your first bed is the one at home, next to the panel. Sleep there first.")
label("veio.home.bed_limit","Você já tem duas camas: a da casa e uma fora. Quebre a de fora para usar esta.","You already have two beds: the one at home and one outside. Break the outside one to use this.")
label("veio.home.bed_home","Cama da casa: você renasce aqui.","Home bed: you respawn here.")
label("veio.home.bed_away","Segunda cama marcada. Você tem a da casa e esta.","Second bed set. You have the one at home and this one.")
label("veio.food.meal","Refeição: você trabalha mais rápido e as fábricas rendem 5%% mais por %s min.","Meal: you work faster and factories produce 5%% more for %s min.")
label("veio.food.feast","Banquete: fábricas +10%%, trabalho mais rápido e regeneração por %s min.","Feast: factories +10%%, faster work and regeneration for %s min.")
label("veio.food.home","Em casa, coma no Lar: tigela na mão e clique nele.","At home, eat at the Hearth: hold a bowl and click it.")
write(f"{D}/advancement/village/root.json",{
 "display":{"icon":{"id":"minecraft:bell"},"title":{"translate":"veio.village.tab"},
 "description":{"translate":"veio.village.tab.description"},"background":"minecraft:gui/advancements/backgrounds/husbandry",
 "announce_to_chat":False,"show_toast":False},
 "criteria":{"welcome":{"trigger":"minecraft:tick"}},
 "rewards":{"loot":["veio:tutorial/village_guide"],"recipes":["veio:village_guide","veio:home"],"function":"veio:village/welcome"}})
p=root/D/"function/village/welcome.mcfunction";p.parent.mkdir(parents=True,exist_ok=True)
p.write_text('tellraw @s {"translate":"veio.village.welcome","color":"gold"}\n',encoding="utf-8")
vstages=[
 ("hire","veio:farm_post","1. Primeiro trabalhador","1. First worker","Posto do lavrador, pão e uma esmeralda perto de aldeões.","Farmhand post, bread and an emerald near villagers."),
 ("field","veio:field_marker","2. Lavoura demarcada","2. Marked field","Marco no centro do campo, água perto, postos na borda.","Marker at the field center, water nearby, posts at the border."),
 ("pen","veio:pen_marker","3. Cercado","3. Pen","Marco no meio do cercado e um posto do pastor.","Marker in the pen and a shepherd post."),
 ("store","veio:barn","4. Celeiro","4. Barn","Guarde as sobras num celeiro ou silo.","Keep surplus in a barn or silo."),
 ("haul","veio:hauler_post","5. Carregador","5. Hauler","Um aldeão abastece máquinas e recolhe produtos.","A villager feeds machines and collects products."),
 ("foreman","veio:foreman_desk","6. Capataz","6. Foreman","A mesa alimenta, recontrata e monta a fila de entregas.","The desk feeds, rehires and queues deliveries."),
 ("center","veio:control_center","7. Centro de controle","7. Control center","Painel, metas e relatório. O spawn vai para o centro.","Panel, goals and report. Spawn moves to the center."),
 ("home","veio:house","8. Casa","8. House","Uma família mora, ganha salário e compra comida.","A family lives, earns wages and buys food."),
 ("market","veio:shop","9. Loja","9. Shop","Um comerciante vende comida e bens às casas e a você.","A merchant sells food and goods to homes and to you."),
 ("mayor","veio:town_hall","10. Prefeitura","10. Town hall","O prefeito paga salários, mede o IDH e libera as fases.","The mayor pays wages, measures HDI and unlocks phases."),
 ("hearth","veio:home","11. Seu lar","11. Your home","Lareira, despensa e uma casa de verdade. Você renasce ali.","Hearth, pantry and a real house. You respawn there."),
]
vparent="veio:village/root"
for name,item,pt,en,pdesc,edesc in vstages:
 key="veio.village.stage."+name;label(key+".title",pt,en);label(key+".description",pdesc,edesc)
 write(f"{D}/advancement/village/{name}.json",{"parent":vparent,"display":{"icon":{"id":item},"title":{"translate":key+".title"},
  "description":{"translate":key+".description"},"announce_to_chat":False,"show_toast":False},
  "criteria":{"obtain":{"trigger":"minecraft:inventory_changed","conditions":{"items":[{"items":item}]}}}})
 vparent="veio:village/"+name
print("Village guide pages:",len(vpages))
from build_transport_guide import build as build_transport_guide
build_transport_guide(write, langs)
# ---------- Guia da Grande Indústria ----------
# Terceiro livro: pesquisa, razões, cadeia funda, monitor, âncora, logística nova e o grande
# projeto. Entregue pelo avanço do aço, quando a fábrica passa a crescer por volume.
for key, pt, en in [
 ("block.veio.research_lab", "Laboratório de pesquisa", "Research Lab"),
 ("block.veio.fabricator", "Fabricadora", "Fabricator"),
 ("block.veio.factory_monitor", "Monitor da fábrica", "Factory Monitor"),
 ("block.veio.chunk_anchor", "Âncora de chunk", "Chunk Anchor"),
 ("block.veio.express_belt", "Esteira expressa", "Express Conveyor"),
 ("block.veio.belt_tunnel", "Túnel de esteira", "Conveyor Tunnel"),
 ("block.veio.cargo_station", "Estação de carga", "Cargo Station"),
 ("block.veio.unloading_station", "Estação de descarga", "Unloading Station"),
 ("veio.station.channel", "Canal %s: %s de %s em trânsito", "Channel %s: %s of %s in transit"),
 ("veio.station.rate", "Ritmo medido: %s itens/min", "Measured rate: %s items/min"),
 ("veio.hint.cargo", "Longe de você, segue nesse ritmo", "While you are away, keeps this rate"),
 ("veio.hint.unloading", "Corante na estação escolhe o canal", "Dye on the station picks the channel"),
 ("item.veio.motor", "Motor elétrico", "Electric Motor"),
 ("item.veio.processor", "Processador", "Processor"),
 ("item.veio.automation_core", "Núcleo de automação", "Automation Core"),
 ("item.veio.mk2_kit", "Kit Mk.II", "Mk.II Kit"),
 ("item.veio.clipboard", "Prancheta", "Clipboard"),
 ("veio.guide.short.express_belt", "Esteira expressa", "Express conveyor"),
 ("veio.guide.short.belt_tunnel", "Túnel", "Tunnel"),
 ("veio.mk2", "Mk.II", "Mk.II"),
 ("veio.research.title", "Pesquisa %s/%s: %s", "Research %s/%s: %s"),
 ("veio.research.metallurgy", "Metalurgia", "Metallurgy"),
 ("veio.research.electronics", "Eletrônica", "Electronics"),
 ("veio.research.automation", "Automação", "Automation"),
 ("veio.research.scale", "Escala", "Scale"),
 ("veio.research.metallurgy.unlocks", "Libera bateria, vapor e túnel", "Unlocks battery, steam, tunnel"),
 ("veio.research.electronics.unlocks", "Libera solar, Tesla e âncora", "Unlocks solar, Tesla, anchor"),
 ("veio.research.automation.unlocks", "Libera fabricadora e broca III", "Unlocks fabricator, drill III"),
 ("veio.research.scale.unlocks", "Libera a esteira expressa", "Unlocks the express belt"),
 ("veio.research.complete", "Todas as pesquisas concluídas.", "All research complete."),
 ("veio.research.done", "Pesquisa concluída: %s. Novas receitas no livro.", "Research complete: %s. New recipes in the book."),
 ("veio.monitor.counts", "%s ativas · %s esperando · %s paradas", "%s active · %s waiting · %s stuck"),
 ("veio.monitor.header", "Item          produz  gasta /min", "Item           made   used /min"),
 ("veio.monitor.empty", "Nenhuma máquina medida ainda.", "No machine measured yet."),
 ("veio.monitor.alert", "Veio: %s máquinas paradas (saída cheia, energia ou desgaste). Uma em %s %s %s.", "Veio: %s machines stuck (full output, power or wear). One at %s %s %s."),
 ("veio.hint.anchor", "Mantém o chunk carregado: %s FE/t", "Keeps the chunk loaded: %s FE/t"),
 ("veio.anchor.on", "Chunk carregado", "Chunk loaded"),
 ("veio.anchor.off", "Sem energia: o chunk será solto", "No power: the chunk will unload"),
 ("veio.hint.tunnel", "Par na mesma direção, até %s blocos", "Pair facing the same way, up to %s blocks"),
 ("veio.lava", "Lava %s / %s mB", "Lava %s / %s mB"),
 ("veio.metric.lava", "Lava %s de %s mB", "Lava %s of %s mB"),
 ("veio.project.phase1", "Torre: fundação de aço", "Tower: steel foundation"),
 ("veio.project.phase2", "Torre: fiação e motores", "Tower: wiring and motors"),
 ("veio.project.phase3", "Torre: central de controle", "Tower: control room"),
 ("veio.project.phase4", "Torre: transmissor", "Tower: transmitter"),
 ("veio.project.stage", "Grande projeto: fase %s de %s", "Grand project: phase %s of %s"),
 ("veio.project.done", "A torre de transmissão está pronta. A central volta a exportar.", "The transmission tower is done. The hub goes back to exporting."),
 ("advancement.veio.research.metallurgy.title", "Metalurgia", "Metallurgy"),
 ("advancement.veio.research.metallurgy.description", "Pesquise com 128 aços e 128 engrenagens.", "Research with 128 steel and 128 gears."),
 ("advancement.veio.research.electronics.title", "Eletrônica", "Electronics"),
 ("advancement.veio.research.electronics.description", "Pesquise com 128 circuitos e 128 molduras.", "Research with 128 circuits and 128 frames."),
 ("advancement.veio.research.automation.title", "Automação", "Automation"),
 ("advancement.veio.research.automation.description", "Pesquise com 64 motores e 64 processadores.", "Research with 64 motors and 64 processors."),
 ("advancement.veio.research.scale.title", "Escala", "Scale"),
 ("advancement.veio.research.scale.description", "Pesquise com 32 núcleos de automação e 256 aços.", "Research with 32 automation cores and 256 steel."),
 ("advancement.veio.grand_project.title", "Torre de transmissão", "Transmission Tower"),
 ("advancement.veio.grand_project.description", "Conclua as quatro fases do grande projeto.", "Finish the four phases of the grand project."),
]:
 label(key, pt, en)
for name,texts in {
 "motor":("Aço e quatro fiações no integrador. Base da automação.","Steel and four wires in an integrator. Base of automation."),
 "processor":("Dois circuitos e dois silícios no montador de circuitos.","Two circuits and two silicon in a circuit assembler."),
 "automation_core":("Motores, processadores e aço na fabricadora. Pesquisa e grande projeto.","Motors, processors and steel in a fabricator. Research and grand project."),
 "mk2_kit":("Botão direito numa máquina: ciclo na metade, dobro de energia por tick. Agachar + chave devolve.","Right-click a machine: half the cycle, double power per tick. Sneak + wrench returns it."),
 "clipboard":("Agachado num braço, divisor ou unificador: copia filtro e prioridade. Em pé: cola.","Sneak on an arm, splitter or merger: copies filter and priority. Standing: pastes."),
}.items(): label(f"veio.componenthelp.{name}",*texts)
for name in industry_order:
 label(f"veio.help.{name}", *help[name])
ipages=[None]
def ipage(pt,en):
 key=f"veio.industry.page.{len(ipages)+1:02}"
 label(key,pt,en)
 ipages.append({"translate":key})
 return len(ipages)
i_intro=ipage(
 "A grande indústria\n\nCom o aço, a fábrica passa a crescer por volume: as receitas novas saem de pesquisas que pedem centenas de peças.\n\nMonte linhas que rodem sozinhas e meça o ritmo de cada uma.",
 "Large-scale industry\n\nWith steel, the factory grows by volume: new recipes come from research that asks for hundreds of parts.\n\nBuild lines that run on their own and measure each one.")
i_research=ipage(
 "Laboratório\n\nEntregue as duas peças da pesquisa atual. Ele gasta 2 de cada por segundo com 24 FE/t.\n\nO progresso é do mundo: vários laboratórios somam e quebrar um não perde nada. Concluída, todos ganham as receitas.",
 "Research lab\n\nDeliver both parts of the current research. It eats 2 of each per second with 24 FE/t.\n\nProgress belongs to the world: several labs add up and breaking one loses nothing. When done, everyone gets the recipes.")
i_tree=ipage(
 "Pesquisas\n\n1 Metalurgia: 128 aço, 128 engr.\nbateria, vapor, túnel, monitor\n2 Eletrônica: 128 circ., 128 mold.\nsolar, Tesla, módulos, âncora\n3 Automação: 64 motores, 64 proc.\nfabricadora, broca III\n4 Escala: 32 núcleos, 256 aço\nesteira expressa",
 "Research\n\n1 Metallurgy: 128 steel, 128 gears\nbattery, steam, tunnel, monitor\n2 Electronics: 128 circ., 128 frames\nsolar, Tesla, modules, anchor\n3 Automation: 64 motors, 64 proc.\nfabricator, drill III\n4 Scale: 32 cores, 256 steel\nexpress belt")
i_ratio=ipage(
 "Razões\n\nAs receitas mostram itens por minuto de uma máquina sem módulo, e o ciclo.\n\nPrensa: 40 cobre > 40 chapas\nMontador: 60 chapas > 30 engr.\n\nTrês prensas alimentam duas montadoras (120 = 2 x 60).",
 "Ratios\n\nRecipes show items per minute of one machine without modules, and the cycle.\n\nPress: 40 copper > 40 plates\nAssembler: 60 plates > 30 gears\n\nThree presses feed two assemblers (120 = 2 x 60).")
i_panel=ipage(
 "No painel\n\nAcima de cada slot, o painel mostra quanto a máquina pede e entrega por minuto, já com módulo e Mk.II.\n\nEsteiras: comum 150/min, rápida 300/min, expressa 600/min. Some as máquinas até encher a esteira.",
 "On the panel\n\nAbove each slot the panel shows what the machine asks and gives per minute, with module and Mk.II.\n\nBelts: basic 150/min, fast 300/min, express 600/min. Add machines until the belt is full.")
i_chain=ipage(
 "Cadeia funda\n\nMotor: aço + 4 fiações (integrador)\nProcessador: 2 circuitos + 2 silícios\nNúcleo: 2 motores, 2 proc., 4 aços (fabricadora)\nKit Mk.II: 2 motores, 1 proc., 8 engr.\n\nO núcleo alimenta a pesquisa de escala e o grande projeto.",
 "Deep chain\n\nMotor: steel + 4 wires (integrator)\nProcessor: 2 circuits + 2 silicon\nCore: 2 motors, 2 proc., 4 steel (fabricator)\nMk.II kit: 2 motors, 1 proc., 8 gears\n\nThe core feeds the scale research and the grand project.")
i_mk2=ipage(
 "Kit Mk.II\n\nBotão direito com o kit numa máquina de receita ou broca: ciclo na metade, dobro de energia por tick e 25% mais poluição.\n\nSoma-se ao módulo. Agachar com a chave tira o módulo, depois o kit.",
 "Mk.II kit\n\nRight-click a recipe machine or drill with the kit: half the cycle, double power per tick and 25% more pollution.\n\nIt stacks with the module. Sneak with the wrench to remove the module, then the kit.")
i_monitor=ipage(
 "Monitor da fábrica\n\nMostra as máquinas da dimensão: ativas, esperando insumo e paradas.\n\nLista os 9 itens de maior fluxo: produção e gasto por minuto. Gasto em vermelho falta na linha.\n\nA cada 30 s avisa onde há máquina parada.",
 "Factory monitor\n\nShows the machines in the dimension: active, waiting for input and stuck.\n\nLists the 9 busiest items: made and used per minute. Red usage is short on the line.\n\nEvery 30 s it warns where a machine is stuck.")
i_stations=ipage(
 "Postos distantes\n\nNo posto: estação de carga no fim da linha. Na base: estação de descarga.\n\nClique as duas com o mesmo corante: é o canal. Até 1.200 itens/min, 10 FE por item.\n\nO canal guarda até 4.096 itens em trânsito.",
 "Distant outposts\n\nAt the outpost: a cargo station at the end of the line. At the base: an unloading station.\n\nClick both with the same dye: that is the channel. Up to 1,200 items/min, 10 FE per item.\n\nThe channel holds up to 4,096 items in transit.")
i_away=ipage(
 "Longe de você\n\nA estação de carga mede quanto manda por minuto. Quando o posto descarrega, ela continua mandando nesse ritmo, sem carregar nada.\n\nAo voltar, o posto segue de onde parou. Mude a linha perto dela: o ritmo novo vale após 1 minuto.",
 "While you are away\n\nThe cargo station measures what it ships per minute. When the outpost unloads it keeps shipping at that rate, loading nothing.\n\nWhen you return, the outpost resumes. Change the line nearby: the new rate counts after 1 minute.")
i_anchor=ipage(
 "Âncora de chunk\n\nPara manter um chunk vivo de verdade (máquinas reagindo, aldeões, redstone): com 32 FE/t ele fica carregado.\n\nPesa mais que as estações. Sem energia por 5 s, solta. Quebrada, solta em 20 s.",
 "Chunk anchor\n\nTo keep a chunk truly alive (machines reacting, villagers, redstone): with 32 FE/t it stays loaded.\n\nHeavier than stations. Without power for 5 s it releases. Broken, in 20 s.")
i_logistics=ipage(
 "Logística nova\n\nTúnel: dois na mesma direção, até 6 blocos. Itens passam por baixo de muros e trilhos.\n\nPrancheta: agachado copia filtro e prioridade de braço ou divisor; em pé, cola em outro.\n\nEsteira expressa: 10 itens/s.",
 "New logistics\n\nTunnel: two facing the same way, up to 6 blocks. Items pass under walls and rails.\n\nClipboard: sneak to copy an arm or splitter's filter and priority; stand to paste onto another.\n\nExpress belt: 10 items/s.")
i_lava=ipage(
 "Lava encanada\n\nA bomba encostada em lava parada (sem água ao lado) puxa 5 mB/t por fonte.\n\nOs canos levam a lava ao gerador geotérmico: 1.000 mB valem um balde, sem baldes vazios. Cada cano leva um fluido só.",
 "Piped lava\n\nA pump touching still lava (and no water) draws 5 mB/t per source.\n\nPipes carry it to the geothermal generator: 1,000 mB is one bucket, with no empty buckets. Each pipe holds one fluid.")
i_project=ipage(
 "Grande projeto\n\nDepois dos quatro contratos, a central pede a torre de transmissão em quatro fases:\n\n1 200 aço, 100 módulos\n2 300 circuitos, 100 motores\n3 150 proc., 50 núcleos\n4 200 núcleos, 50 kits Mk.II\n\nEla recolhe aos poucos e paga cada fase.",
 "Grand project\n\nAfter the four contracts, the hub asks for the transmission tower in four phases:\n\n1 200 steel, 100 modules\n2 300 circuits, 100 motors\n3 150 proc., 50 cores\n4 200 cores, 50 Mk.II kits\n\nIt collects slowly and pays each phase.")
i_climate=ipage(
 "Clima e estações\n\nO ano tem 4 estações de 6 dias. Solar rende mais no verão, vento no inverno, rios cheios na primavera.\n\nNa geada a lavoura para e a roda d'água congela; com chuva neva e os lagos congelam. Telhado sobre a plantação vira estufa: 20 °C.",
 "Weather and seasons\n\nThe year has 4 seasons of 6 days. Solar yields more in summer, wind in winter, rivers run high in spring.\n\nIn frost crops stop and the water wheel freezes; rain turns to snow and lakes freeze. A roof over the field makes a greenhouse: 20 °C.")
i_visual=ipage(
 "Veio Realismo\n\nO pacote de shaders do jogo mostra as estações nas folhas, a fumaça das fábricas no céu e a luz colorida das máquinas quentes.\n\nPC de 8 GB: escolha Leve nas opções do pacote.",
 "Veio Realism\n\nThe game's shader pack shows the seasons on the leaves, factory smoke in the sky and the coloured glow of hot machines.\n\n8 GB PC: pick \"Leve (8 GB)\" in the pack options.")
i_blocks=len(ipages)+1;ipages.append(None)
irefs={}
for name in industry_order:
 reference_labels(name)
 irefs[name]=len(ipages)+1
 ipages.append({"text":"","extra":[{"translate":f"block.veio.{name}","bold":True},{"text":"\n"},
  {"translate":f"veio.guide.use.{name}"},{"text":"\n"},{"translate":f"veio.guide.craft.{name}","color":"dark_gray"},{"text":"\n"},
  {"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}}]})
label("veio.industry.blocks","Blocos","Blocks")
extra=[{"translate":"veio.industry.blocks","bold":True},{"text":"\n"}]
for name in industry_order:
 extra.extend([{"translate":f"block.veio.{name}","color":"dark_blue","underlined":True,
  "click_event":{"action":"change_page","page":irefs[name]}},{"text":"\n"}])
extra.append({"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}})
ipages[i_blocks-1]={"text":"","extra":extra}
ilinks=[("Começar","Start here",i_intro),("Laboratório","Research lab",i_research),("Pesquisas","Research tree",i_tree),
 ("Razões","Ratios",i_ratio),("Razões no painel","Ratios on the panel",i_panel),("Cadeia funda","Deep chain",i_chain),
 ("Kit Mk.II","Mk.II kit",i_mk2),("Monitor","Monitor",i_monitor),("Postos distantes","Distant outposts",i_stations),("Longe de você","While away",i_away),("Âncora de chunk","Chunk anchor",i_anchor),
 ("Logística nova","New logistics",i_logistics),("Lava encanada","Piped lava",i_lava),("Grande projeto","Grand project",i_project),("Clima","Weather",i_climate),("Visual","Visuals",i_visual),("Blocos","Blocks",i_blocks)]
label("veio.industry.title","Guia da Grande Indústria","Large Industry Guide")
extra=[{"translate":"veio.industry.title","bold":True},{"text":"\n"}]
for n,(pt,en,target) in enumerate(ilinks):
 key=f"veio.industry.contents.{n}";label(key,pt,en)
 extra.extend([{"translate":key,"color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":target}},{"text":"\n"}])
ipages[0]={"text":"","extra":extra}
assert len(ipages)<=100
ibook={"minecraft:written_book_content":{"title":{"raw":"Grande Indústria"},"author":"Veio",
 "generation":0,"resolved":True,"pages":[{"raw":p} for p in ipages]},
 "minecraft:custom_name":{"translate":"veio.industry.title","italic":False}}
write(f"{D}/loot_table/tutorial/industry_guide.json",{"type":"minecraft:generic",
 "pools":[{"rolls":1,"entries":[{"type":"minecraft:item","name":"minecraft:written_book",
 "functions":[{"function":"minecraft:set_components","components":ibook}]}]}]})
write(f"{D}/recipe/industry_guide.json",{"type":"minecraft:crafting_shapeless","category":"misc",
 "ingredients":["minecraft:book","veio:gear"],
 "result":{"id":"minecraft:written_book","count":1,"components":ibook}})
label("veio.industry.welcome","Você recebeu o Guia da Grande Indústria: pesquisa, razões e o grande projeto.","You received the Large Industry Guide: research, ratios and the grand project.")
write(f"{D}/advancement/tutorial/industry.json",{"parent":"veio:steel",
 "criteria":{"steel":{"trigger":"minecraft:inventory_changed","conditions":{"items":[{"items":"veio:steel_plate"}]}}},
 "rewards":{"loot":["veio:tutorial/industry_guide"],"recipes":["veio:industry_guide"],"function":"veio:tutorial/industry_welcome"}})
p=root/D/"function/tutorial/industry_welcome.mcfunction";p.parent.mkdir(parents=True,exist_ok=True)
p.write_text('tellraw @s {"translate":"veio.industry.welcome","color":"gold"}\n',encoding="utf-8")
print("Industry guide pages:",len(ipages))
from build_operations import add_labels
add_labels(langs)
from build_machine_audio import add_labels as add_sound_labels
add_sound_labels(langs)
from build_onboarding import add_labels as add_onboarding_labels
add_onboarding_labels(label)
from build_wiki import build as build_wiki
build_wiki(label, langs, vanilla)
for code,data in langs.items():write(f"{A}/lang/{code}.json",data)
print("Guide pages:",len(pages))
