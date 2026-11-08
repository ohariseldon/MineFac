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
 "oak_planks":("Tábuas de carvalho","Oak planks"),"oak_boat":("Barco de carvalho","Oak boat"),"oak_door":("Porta de carvalho","Oak door"),"glass_pane":("Painel de vidro","Glass pane"),
 "stone_bricks":("Tijolos de pedra","Stone bricks"),"cobblestone":("Pedregulho","Cobblestone"),
}
def item_name(item,code):
 ns,name=item.split(":")
 if ns=="minecraft": return vanilla.get(name,(name,name))[0 if code=="pt_br" else 1]
 for kind in ("block","item"):
  key=f"{kind}.{ns}.{name}"
  if key in langs[code]: return langs[code][key]
 return name

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
 "Cobre/ferro bruto vira chapa.\nOuro bruto vira lingote.\nEntrada no slot esquerdo.\nSaída no direito. Use energia.",
 "Raw copper/iron makes plates.\nRaw gold makes ingots.\nInput in the left slot.\nOutput on the right. Use power."),
 "assembler":(
 "Recebe 2 chapas de cobre.\nProduz 1 engrenagem.\nEntrada à esquerda.\nSaída à direita. Use energia.",
 "Takes 2 copper plates.\nMakes 1 gear.\nInput on the left.\nOutput on the right. Use power."),
 "cutter":(
 "Chapa de ferro vira moldura.\nChapa de cobre vira fiação.\nEntrada à esquerda.\nSaída à direita. Use energia.",
 "Iron plates make frames.\nCopper plates make wiring.\nInput on the left.\nOutput on the right. Use power."),
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
label("veio.tutorial.merger.description", "Conecte as entradas A e B pelas laterais. Você recebeu o guia atualizado.", "Connect inputs A and B from the sides. You received the updated guide.")
label("block.veio.inserter", "Braço de transferência", "Transfer Arm")
label("veio.hint.inserter", "Traseira > frente: 40 FE/item", "Rear > front: 40 FE/item")
label("veio.hint.inserter_wait", "Confira origem e destino", "Check source and target")
label("veio.hint.fast_inserter", "Traseira > frente: 64 FE/item", "Rear > front: 64 FE/item")
label("veio.hint.long_inserter", "Alcance 2; 60 FE/item", "Reach 2; 60 FE/item")
help["inserter"] = (
 "Puxa de trás, entrega à frente.\nBaús, esteiras e máquinas.\nAté 1 item/s; 40 FE por item.\nLigue um cabo ou poste.\nRedstone pausa; chave gira.",
 "Takes from rear, sends ahead.\nChests, belts and machines.\nUp to 1 item/s; 40 FE/item.\nConnect a cable or pole.\nRedstone pauses; wrench turns.")
label("block.veio.fast_inserter", "Braço rápido", "Fast Transfer Arm")
label("block.veio.long_inserter", "Braço longo", "Long Transfer Arm")
help["fast_inserter"] = (
 "Até 2,5 itens/s; 64 FE/item.\nPuxa atrás, entrega à frente.\nAlcance de 1 bloco por lado.\nAgachar + item: filtro.\nAgachar + mão vazia: limpar.",
 "Up to 2.5 items/s; 64 FE/item.\nRear to front, 1-block reach.\nSneak + item: set filter.\nSneak + empty hand: clear.")
help["long_inserter"] = (
 "Até 1 item/s; 60 FE/item.\nAlcance de 2 blocos por lado.\nIgnora inventário intermediário.\nAgachar + item: filtro.\nAgachar + mão vazia: limpar.",
 "Up to 1 item/s; 60 FE/item.\nReaches 2 blocks on each side.\nSkips intermediate inventory.\nSneak + item: set filter.\nSneak + empty hand: clear.")
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
 "Centro da lavoura: 9x9 no\nnível 1, até 21x21. Lavradores\nperto aram, plantam e colhem.\nSemente no slot: só ela.\nEsmeralda expande; chave escolhe.",
 "Field center: 9x9 at level 1,\nup to 21x21. Nearby farmhands\ntill, plant and harvest.\nSeed in the slot: only that.\nEmerald expands; wrench sizes.")
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
 "Esmeralda: contrata o\ncomerciante (independente).\nO que você entrega ele compra\nno atacado (70%) e paga com\nas vendas. Imposto na fonte.",
 "Emerald: hires the merchant\n(independent). What you\ndeliver they buy wholesale\n(70%) and pay from sales.\nTax withheld at source.")
help["town_hall"] = (
 "Esmeralda: contrata o\nprefeito. Vive do imposto na\nfonte (4% a 16%); paga o\nprefeito e a assistência.\nMede IDH e libera as fases.",
 "Emerald: hires the mayor.\nLives on tax at source (4% to\n16%); pays the mayor and\nassistance. Measures HDI and\nunlocks phases.")
help["well"] = (
 "Saneamento das casas a até\n24 blocos. Balde vazio no\nslot vira balde d'água.",
 "Sanitation for homes within\n24 blocks. An empty bucket\nin the slot fills with water.")
help["export_dock"] = (
 "Vende ao mercado externo:\num navio por minuto leva\ntudo. Mais complexo, mais\nvale (até 4x). O mesmo item\nno mesmo dia vale menos.",
 "Sells to foreign markets:\na ship each minute takes\neverything. More complex,\nmore value (up to 4x). Same\nitem, same day: worth less.")
help["farm_post"] = (
 "Esmeralda: contrata o aldeão\nadulto livre mais próximo.\nEle colhe o 9x9 e traz ao posto.\nCome 2 pontos por colheita.\nAgachar + chave: dispensa.",
 "Emerald: hires the nearest\nfree adult villager.\nHarvests the 9x9, brings it back.\nEats 2 points per harvest.\nSneak + wrench: dismiss.")
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
 ("veio.worker.hired", "Aldeão contratado. Ele vai até as plantações maduras.", "Villager hired. They will walk to ripe crops."),
 ("veio.worker.staffed", "Este posto já tem um trabalhador.", "This post already has a worker."),
 ("veio.worker.none", "Nenhum aldeão adulto livre a até 8 blocos.", "No free adult villager within 8 blocks."),
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
 ("veio.status.rest", "Descansando (noite)", "Resting (night)"),
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
categories=[
 ("Extração e processamento","Mining and processing",["drill","drill_mk2","drill_mk3","press","assembler","cutter","integrator","crusher","alloy_furnace","electronics"]),
 ("Lavoura e comida","Farming and food",["harvester","planter","grow_lamp","tree_farm","ranch","mill","electric_furnace","kitchen","pantry"]),
 ("Energia","Power",["generator","biomass_generator","water_wheel","wind_turbine","steam_engine","geothermal","solar","battery"]),
 ("Logística","Logistics",["belt","fast_belt","splitter","merger","inserter","fast_inserter","long_inserter","cable","pole","steel_pole","wrench"]),
 ("Projetos e defesa","Projects and defense",["contract_hub","tesla"]),
]
order=[name for _,_,names in categories for name in names]
# Blocos da vila operária: consulta no Guia da Vila Operária (fim deste arquivo).
village_order=["farm_post","field_marker","shepherd_post","pen_marker","barn","silo","hauler_post","foreman_desk","control_center","site","barracks","house","shop","town_hall","well","export_dock","home"]
assert set(order)|set(village_order)==set(help), (set(order)|set(village_order))^set(help)
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
 "shop":["Comida do comerciante.","Entrega da indústria: o comerciante paga 70% no atacado.","Entrega da indústria.","Entrega da indústria.","Entrega da indústria.","Entrega da indústria.","Entrega da indústria.","Antigo caixa (até a 0.20): retire as esmeraldas."],
 "town_hall":["Comida do prefeito.","Aporte voluntário à conta pública (60 moedas por esmeralda)."],
 "well":["Balde vazio: sai cheio de água."],
 "export_dock":["Carga do navio: produtos para exportar."]*9,
 "control_center":["Comida do administrador.","Meta: o item e a quantidade x16 a guardar.","Meta: o item e a quantidade x16 a guardar.","Meta: o item e a quantidade x16 a guardar.","Cofre da companhia: esmeraldas viram moedas (60 cada).","Saque: clique na linha da companhia no painel (1 esmeralda).","Obra: entregas dos pedidos, materiais das obras; esmeraldas vão para o caixa.","Decreto: sino (convocação), papel (racionamento) ou fardo de feno (fartura)."],
 "site":["Comida do mestre de obras: cada leva de blocos custa uma tarefa."],
 "home":["Lenha: carvão, carvão vegetal, biomassa, troncos ou tábuas.","Despensa: comida para as refeições em casa.","Despensa: comida para as refeições em casa.","Despensa: comida para as refeições em casa.","Obra: materiais da próxima melhoria da casa."],
 "foreman_desk":["Comida do capataz e dos postos.","Comida do capataz e dos postos.","Comida do capataz e dos postos.","Esmeraldas para recontratar."],
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
 "shop":["Merchant food.","Industry delivery: the merchant pays 70% wholesale.","Industry delivery.","Industry delivery.","Industry delivery.","Industry delivery.","Industry delivery.","Old till (up to 0.20): take the emeralds out."],
 "town_hall":["Mayor food.","Voluntary contribution to public funds (60 coins per emerald)."],
 "well":["Empty bucket: comes out full of water."],
 "export_dock":["Ship cargo: goods to export."]*9,
 "control_center":["Administrator food.","Goal: the item and its count x16 to store.","Goal: the item and its count x16 to store.","Goal: the item and its count x16 to store.","Company vault: emeralds become coins (60 each).","Cash out: click the company line on the panel (1 emerald).","Build: request deliveries, project materials; emeralds go to the treasury.","Decree: bell (call-up), paper (rationing) or hay bale (plenty)."],
 "site":["Master builder food: each batch of blocks costs one task."],
 "home":["Fuel: coal, charcoal, biomass, logs or planks.","Pantry: food for meals at home.","Pantry: food for meals at home.","Pantry: food for meals at home.","Build: materials for the next house upgrade."],
 "foreman_desk":["Food for the foreman and the posts.","Food for the foreman and the posts.","Food for the foreman and the posts.","Emeralds for rehiring."],
 "contract_hub":["First ingredient in the displayed order.","Second ingredient in the displayed order.","First reward.","Second reward."],
}
for block,lines in slothelp.items():
 for i,line in enumerate(lines): label(f"veio.slothelp.{block}.{i}",line,slot_en[block][i])
label("veio.slothelp.merger.0", "A: lateral amarela. Olhando para a saída, fica à esquerda.", "A: yellow side. Left when facing the outlet.")
label("veio.slothelp.merger.1", "B: lateral azul. Olhando para a saída, fica à direita.", "B: cyan side. Right when facing the outlet.")

label("veio.guide.title","Guia do Veio","Veio Field Guide")
label("veio.guide.help","Ajuda ?","Help ?")
label("veio.guide.open","Selecione o guia e use o botão direito.","Select the guide and right-click.")
label("veio.guide.tip","Passe o cursor no ? dos painéis.","Hover over ? in machine panels.")
label("veio.guide.welcome","Você recebeu o Guia do Veio. Selecione o livro e abra com o botão direito.","You received the Veio Field Guide. Select it and right-click to begin.")
label("veio.guide.back","Sumário","Contents")
label("veio.guide.recipe_link","Receita","Recipe")
label("veio.guide.role.input","Entrada","Input")
label("veio.guide.role.output","Saída","Output")
label("veio.guide.role.fuel","Combust.","Fuel")
label("veio.guide.role.input1","Ent.1","In 1")
label("veio.guide.role.input2","Ent.2","In 2")
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
 "wire":("Chapa de cobre na cortadora. Usada em circuitos e cabos.","Copper plate in a cutter. Used for circuits and cables."),
 "silicon":("Quartzo do Nether no triturador. Usado em circuitos.","Nether quartz in a crusher. Used for circuits."),
 "circuit":("Duas fiações e um silício no montador de circuitos.","Two copper wiring and one silicon in a circuit assembler."),
}
components.update({
 "flour":("Trigo no moinho. Vai para pão, biscoito e bolo na cozinha.","Wheat in the mill. Used for bread, cookies and cake."),
 "biomass":("Plantas ou troncos no moinho. Combustível do gerador a biomassa.","Plants or logs in the mill. Fuel for the biomass generator."),
 "repair_kit":("Botão direito numa máquina danificada: recupera 50% da integridade.","Right-click a damaged machine: restores 50% integrity."),
 "speed_module":("Botão direito numa máquina: 40% mais rápida, mais energia e poluição.","Right-click a machine: 40% faster, more power and pollution."),
 "efficiency_module":("Botão direito numa máquina: 40% menos energia e metade da poluição.","Right-click a machine: 40% less power and half the pollution."),
 "productivity_module":("Botão direito numa máquina: +10% de produtos, ciclo mais lento.","Right-click a machine: +10% products, slower cycle."),
})
for name,texts in components.items():label(f"veio.componenthelp.{name}",*texts)

pages=[None] # Contents filled after page numbers are assigned.
def page(pt,en):
 key=f"veio.guide.page.{len(pages)+1:02}"
 label(key,pt,en)
 pages.append({"translate":key})
 return len(pages)
intro=page(
 "Primeira fábrica\n\nA meta é obter uma engrenagem.\nVocê vai extrair cobre, fazer chapas e montar peças.\n\nAvance uma página por vez. O restante fica como consulta.",
 "First factory\n\nYour goal is one gear.\nExtract copper, press plates and assemble parts.\n\nFollow one page at a time. Later pages are a reference.")
materials=page(
 "Prepare os materiais\n\n6 lingotes de cobre\n3 lingotes de ferro\n3 pistões\n1 bancada extra\n1 fornalha\n1 pedra lisa\n3 gravetos\n1 baú e carvão\n\nSão peças já prontas.",
 "Gather the parts\n\n6 copper ingots\n3 iron ingots\n3 pistons\n1 spare crafting table\n1 furnace\n1 smooth stone\n3 sticks\n1 chest and coal\n\nThese are finished parts.")
page(
 "Como fabricar\n\nUse uma bancada. As receitas do Veio aceitam os ingredientes em qualquer posição.\n\nO livro verde da bancada mostra as receitas liberadas.\nAs páginas de consulta mostram os ingredientes.",
 "How to craft\n\nUse a crafting table. Veio recipes accept ingredients in any arrangement.\n\nThe green recipe book lists unlocked recipes.\nReference pages list ingredients.")
page(
 "1. Prepare energia\n\nFabrique um gerador e postes de madeira. Abra o gerador e coloque carvão ou madeira.\n\nPonha um poste a até 2 blocos do gerador e das máquinas. Postes se ligam sozinhos por fios.",
 "1. Prepare power\n\nCraft a generator and wooden poles. Open the generator and add coal or wood.\n\nPlace a pole within 2 blocks of the generator and machines. Poles wire themselves together.")
page(
 "2. Ache uma jazida\n\nEncontre minério de cobre no mundo. Coloque a extratora diretamente SOBRE ele.\n\nEncoste um cabo energizado na extratora.\n\nO painel deve mostrar atividade.",
 "2. Find a vein\n\nFind copper ore in the world. Put the extractor directly ON TOP of it.\n\nTouch it with a powered cable.\n\nThe panel should show activity.")
layout=page(
 "3. Monte a linha\n\nExtratora > esteira > prensa > baú\n\nColoque tudo na mesma altura. Olhe para o destino ao colocar os blocos.\n\nO bocal indica a saída. Faça um ramo de cabos até a prensa.",
 "3. Build the line\n\nExtractor > belt > press > chest\n\nPlace all endpoints at the same height. Face the destination when placing blocks.\n\nThe port is the output. Branch cables to the press.")
page(
 "4. Veja a primeira chapa\n\nA prensa recebe cobre BRUTO à esquerda. A chapa sai à direita.\n\nPode colocar cobre à mão.\n\nRetire uma chapa no painel ou no baú.",
 "4. Your first plate\n\nThe press takes RAW COPPER in the left slot. The plate appears on the right.\n\nTry inserting raw copper by hand.\n\nTake a plate from the panel or chest.")
page(
 "5. Faça engrenagens\n\nFabrique um montador. Ligue energia e leve chapas até ele.\n\nCada engrenagem usa DUAS chapas de cobre.\n\nColoque um baú na saída e recolha a primeira peça.",
 "5. Make gears\n\nCraft an assembler. Connect power and feed it copper plates.\n\nEach gear uses TWO copper plates.\n\nPut a chest at the output and take your first gear.")
page(
 "6. Seu próximo passo\n\nFabrique a chave para girar blocos com botão direito.\n\nDepois monte a linha de ferro: prensa > cortadora > moldura.\n\nNo integrador, moldura + engrenagem viram módulo.",
 "6. Next steps\n\nCraft the wrench to turn blocks with right-click.\n\nBuild an iron line: press > cutter > frame.\n\nThe integrator turns a frame and gear into a module.")
troubleshoot=page(
 "Se a máquina parar\n\nSem energia: confira carvão e cabos.\n\nSem jazida: confira o bloco abaixo.\n\nFalta material: veja os slots.\n\nSaída cheia: libere espaço no destino.",
 "When a machine stops\n\nNo power: check fuel and cables.\n\nNo ore: check the block underneath.\n\nMissing material: inspect the slots.\n\nOutput full: make room downstream.")
page(
 "Controles e ajuda\n\nBotão direito abre o painel.\n\nChave + botão direito gira.\n\nPasse o cursor no ? ou slot vazio para ajuda.\n\nRedstone pausa a produção.",
 "Controls and help\n\nEmpty hand + right-click opens the panel.\n\nWrench + right-click turns it.\n\nHover over ? or an empty slot for help.\n\nPowered redstone pauses production.")
page(
 "Seu roteiro\n\nAbra Avanços > Primeira fábrica. Os marcos contam materiais obtidos.\n\nRecolha os produtos para marcar progresso.\n\nO painel de cada máquina confirma seu funcionamento.",
 "Your checklist\n\nOpen Advancements and find First Factory. Milestones track obtained materials.\n\nPick up products to mark progress.\n\nInspect machine panels to confirm operation.")
wiring=page(
 "Postes e fios\n\nCada poste alimenta as máquinas a até 2 blocos (aço: 3) e se liga aos postes vizinhos.\n\nMadeira: fios de 8 blocos, 120 FE/t.\nAço: 16 blocos, 400 FE/t.\n\nQuando faltar energia longe do gerador, o fio é o gargalo.",
 "Poles and wires\n\nEach pole powers machines within 2 blocks (steel: 3) and links to nearby poles.\n\nWood: 8-block wires, 120 FE/t.\nSteel: 16 blocks, 400 FE/t.\n\nIf far machines starve, the wire is the bottleneck.")
smog=page(
 "Poluição\n\nMáquinas trabalhando poluem o chunk; o painel mostra quanto por minuto. A poluição se espalha e some aos poucos.\n\nLavouras colhidas a absorvem. Módulos de eficiência cortam pela metade.",
 "Pollution\n\nWorking machines pollute their chunk; the panel shows how much per minute. It spreads and slowly fades.\n\nHarvested crops absorb it. Efficiency modules halve it.")
page(
 "Saqueadores\n\nO chat avisa um dia antes. Na noite seguinte uma onda chega por um lado, com som de invasão. Cada noite vem maior: da 3ª vêm esqueletos, da 5ª o Bruto, que quebra máquinas rápido.\n\nAranhas e esqueletos caçam aldeões. Quartel, muros e torre Tesla.",
 "Raiders\n\nChat warns a day ahead. The next night a wave comes from one side with a raid horn. Each night is bigger: skeletons from the 3rd, the Brute from the 5th, who wrecks machines fast.\n\nSpiders and skeletons hunt villagers. Barracks, walls and Tesla towers.")
upgrades=page(
 "Módulos\n\nBotão direito numa máquina instala um. Chave agachado retira.\n\nVelocidade: mais produção, mais energia e poluição.\nEficiência: menos energia e poluição.\nProdutividade: +10% de produtos.",
 "Modules\n\nRight-click a machine to install one. Sneak with the wrench to remove it.\n\nSpeed: more output, power and pollution.\nEfficiency: less power and pollution.\nProductivity: +10% products.")
world=page(
 "Fábrica e mundo\n\nColheitadeira: plante ao redor dela e ligue uma esteira.\n\nTorre Tesla: protege a base à noite. Junte acumuladores ao solar.\n\nOssos dos monstros viram farinha no triturador.",
 "Factory and world\n\nHarvester: build the farm around it and belt the crops away.\n\nTesla tower: guards the base at night. Pair solar with batteries.\n\nMonster bones become bone meal in the crusher.")
page(
 "Comércio\n\nDepois dos quatro projetos, a central exporta sem fim: 4 circuitos e 4 chapas de aço viram 4 esmeraldas e frascos de experiência.\n\nUse as esmeraldas com os aldeões da sua vila.",
 "Trade\n\nAfter the four projects, the hub keeps exporting: 4 circuits and 4 steel plates become 4 emeralds and experience bottles.\n\nSpend emeralds with your villagers.")
production=page(
 "Escada de produção\n\nCada melhoria resolve um gargalo e revela o próximo: jazida, energia, fio, poluição.\n\nVeios grandes rendem mais: a extratora lê o 3x3 abaixo dela. Jazida pura dobra a velocidade.",
 "Production ladder\n\nEach upgrade fixes one bottleneck and reveals the next: vein, power, wire, pollution.\n\nBig veins pay off: the extractor reads the 3x3 below. A pure vein doubles its speed.")
page(
 "Energia\n\nRoda d'água e cata-vento: grátis e limpos; 8 FE/t por face ou no nível do mar.\nGerador e biomassa: 40 FE/t; biomassa polui menos.\nVapor: 120 FE/t, precisa de água.\nGeotérmico e solar: escalam.\n\nAcumuladores seguram a noite.",
 "Power\n\nWater wheel and wind: free and clean; 8 FE/t per face or at sea level.\nGenerator and biomass: 40 FE/t; biomass pollutes less.\nSteam: 120 FE/t, needs water.\nGeothermal and solar: scale up.\n\nBatteries carry the night.")
food=page(
 "Comida e povoado\n\nTrigo > moinho > farinha > cozinha > pão.\nCarnes do rancho > fornalha elétrica.\n\nO armazém paga esmeraldas pela comida, mais por comida pronta, e 50% a mais com aldeões perto.",
 "Food and village\n\nWheat > mill > flour > kitchen > bread.\nRanch meat > electric furnace.\n\nThe pantry pays emeralds for food, more for cooked food, and 50% more with villagers near.")
page(
 "A vila operária\n\nAldeões contratados trabalham por você: lavoura, animais, máquinas, armazéns e um centro de controle.\n\nTudo isso está no Guia da Vila Operária, entregue no começo. Livro + trigo fabricam outro.",
 "The worker village\n\nHired villagers work for you: fields, animals, machines, storage and a control center.\n\nIt is all in the Worker Village Guide, given at the start. Book + wheat craft another.")
help["belt"] = (
 "Curvas e rampas automáticas.\nConecte esteiras vizinhas.\nRampas vencem 1 bloco.\nChave fixa a direção.\nAgachar + chave: automático.",
 "Automatic turns and ramps.\nConnect neighboring belts.\nRamps span one block up.\nWrench locks the direction.\nSneak + wrench: automatic.")
help["fast_belt"] = (
 "Mesmas curvas e rampas.\nTransporta até 5 itens/s.\nUse na saída do unificador.\nChave fixa a direção.",
 "Same turns and ramps.\nCarries up to 5 items/s.\nUse after a merger.\nWrench locks direction.")
for name in ("belt", "fast_belt"):
 label(f"veio.help.{name}", *help[name])
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
 "Rede de logística\n\nEsteiras levam itens longe. Braços ligam baús e máquinas. Unificadores juntam; divisores distribuem.\n\nUse rápidas depois de juntar linhas.",
 "Logistics network\n\nBelts carry over distance. Arms link chests and machines. Mergers join; splitters distribute.\n\nUse fast belts after merging.")
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
for _ in categories:
 category_pages.append(len(pages)+1);pages.append(None)
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
 pages.append({"text":"","extra":[{"translate":f"{kind}.veio.{name}","bold":True},{"text":"\n"},
  {"translate":usage_key},{"text":"\n"},{"translate":craft_key,"color":"dark_gray"},{"text":"\n"},
  {"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}}]})
for g,(pt,en,members) in enumerate(categories):
 key=f"veio.guide.category.{g}"
 label(key,pt,en)
 extra=[{"translate":key,"bold":True},{"text":"\n"}]
 for name in members:
  kind="item" if name=="wrench" else "block"
  short_key=f"veio.guide.short.{name}"
  title_key=short_key if short_key in langs["pt_br"] else f"{kind}.veio.{name}"
  extra.extend([{"translate":title_key,"color":"dark_blue","underlined":True,
                 "click_event":{"action":"change_page","page":references[name]}},{"text":"\n"}])
 extra.append({"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}})
 pages[category_pages[g]-1]={"text":"","extra":extra}

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
PER_PAGE=4
for machine in machines:
 entries=[r for r in production_data["recipes"] if r["machine"]==machine]
 for start in range(0,len(entries),PER_PAGE):
  chunk=entries[start:start+PER_PAGE]
  extra=[{"translate":f"block.veio.{machine}","bold":True}]
  if len(entries)>PER_PAGE:
   extra.append({"text":f" {start//PER_PAGE+1}/{(len(entries)+PER_PAGE-1)//PER_PAGE}"})
  extra.append({"text":"\n"})
  for r in chunk:
   extra.append({"text":"\n"})
   for i,(item,count) in enumerate(r["in"]):
    if i: extra.append({"text":" + "})
    extra.extend([{"text":f"{count} "},name_component(item)])
   extra.extend([{"text":" > ","color":"dark_gray"},{"text":f"{r['out'][1]} "},name_component(r["out"][0])])
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
# O sumário cabe numa página: 12 entradas. "Resolver falhas" e "Fábrica e mundo" seguem na leitura.
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
book={"minecraft:written_book_content":{"title":{"raw":"Veio 0.22.0"},"author":"Veio",
 "generation":0,"resolved":True,"pages":[{"raw":p} for p in pages]},
 "minecraft:custom_name":{"translate":"veio.guide.title","italic":False}}
write(f"{D}/loot_table/tutorial/guide.json",{"type":"minecraft:generic",
 "pools":[{"rolls":1,"entries":[{"type":"minecraft:item","name":"minecraft:written_book",
 "functions":[{"function":"minecraft:set_components","components":book}]}]}]})
write(f"{D}/recipe/guide.json",{"type":"minecraft:crafting_shapeless","category":"misc",
 "ingredients":["minecraft:book","minecraft:paper"],
 "result":{"id":"minecraft:written_book","count":1,"components":book}})
label("veio.tutorial.title","Primeira fábrica","First Factory")
label("veio.tutorial.start","Abra o Guia do Veio no inventário e siga a primeira linha de cobre.","Open the Veio Field Guide in your inventory and follow the copper line.")
start={"display":{"icon":{"id":"minecraft:written_book"},"title":{"translate":"veio.tutorial.title"},
 "description":{"translate":"veio.tutorial.start"},"background":"minecraft:gui/advancements/backgrounds/stone",
 "announce_to_chat":False},
 "criteria":{"welcome":{"trigger":"minecraft:tick"}},
 "rewards":{"loot":["veio:tutorial/guide"],"recipes":["veio:guide","veio:generator","veio:cable","veio:drill","veio:belt","veio:press"],
 "function":"veio:tutorial/welcome"}}
write(f"{D}/advancement/tutorial/start.json",start)
label("veio.guide.path_welcome", "Guia atualizado: esteiras agora fazem curvas e rampas. Consulte as páginas antes das categorias.", "Updated guide: belts now turn and climb. Read the pages before the categories.")
write(f"{D}/advancement/tutorial/belt_paths.json", {
 "parent":"veio:tutorial/start",
 "criteria":{"obtain":{"trigger":"minecraft:inventory_changed","conditions":{"items":[{"items":"veio:belt"}]}}},
 "rewards":{"loot":["veio:tutorial/guide"],"function":"veio:tutorial/path_welcome"}})
p=root/D/"function/tutorial/path_welcome.mcfunction"
p.parent.mkdir(parents=True,exist_ok=True)
p.write_text('tellraw @s {"translate":"veio.guide.path_welcome","color":"gold"}\n',encoding="utf-8")
write(f"{D}/advancement/tutorial/merger.json", {
 "parent":"veio:tutorial/start",
 "display":{"icon":{"id":"veio:merger"},"title":{"translate":"veio.tutorial.merger.title"},
 "description":{"translate":"veio.tutorial.merger.description"},"announce_to_chat":False},
 "criteria":{"obtain":{"trigger":"minecraft:inventory_changed","conditions":{"items":[{"items":"veio:merger"}]}}},
 "rewards":{"loot":["veio:tutorial/guide"]}})
p=root/D/"function/tutorial/welcome.mcfunction";p.parent.mkdir(parents=True,exist_ok=True)
p.write_text('tellraw @s {"translate":"veio.guide.welcome","color":"gold"}\n',encoding="utf-8")
stages=[
 ("generator","veio:generator","1. Gerador em mãos","1. Get a generator","Coloque combustível e um poste perto das máquinas.","Add fuel and place a pole near the machines."),
 ("drill","veio:drill","2. Preparar extração","2. Prepare extraction","Coloque a extratora sobre cobre e dê energia.","Place the extractor on copper ore and supply power."),
 ("plates","veio:copper_plate","3. Recolher uma chapa","3. Collect a plate","A prensa usa cobre bruto. Recolha sua primeira chapa.","The press uses raw copper. Pick up your first plate."),
 ("gears","veio:gear","4. Primeira engrenagem","4. Your first gear","Leve duas chapas ao montador e recolha uma engrenagem.","Feed two plates to the assembler and collect a gear."),
 ("frames","veio:frame","5. Produzir molduras","5. Make frames","Na linha de ferro, a cortadora transforma chapas em molduras.","On the iron line, the cutter turns plates into frames."),
 ("modules","veio:module","6. Juntar as linhas","6. Join the lines","Entregue moldura e engrenagem ao integrador.","Supply a frame and gear to the integrator."),
 ("hub","veio:contract_hub","7. Escolher um projeto","7. Start a project","Consulte o pedido da central. O guia tem suas receitas.","Read the hub order. The guide contains its recipes."),
 ("repair","veio:repair_kit","8. Preparar reparos","8. Prepare repairs","A poluição atrai Saqueadores. Tenha kits de reparo à mão.","Pollution draws Raiders. Keep repair kits at hand."),
 ("defense","veio:tesla","9. Defender a fábrica","9. Defend the factory","Uma torre Tesla abastecida protege a fábrica à noite.","A powered Tesla tower guards the factory at night."),
]
parent="veio:tutorial/start"
for name,item,pt,en,pdesc,edesc in stages:
 key="veio.tutorial."+name;label(key+".title",pt,en);label(key+".description",pdesc,edesc)
 obj={"parent":parent,"display":{"icon":{"id":item},"title":{"translate":key+".title"},
 "description":{"translate":key+".description"},"announce_to_chat":False,"show_toast":False},
 "criteria":{"obtain":{"trigger":"minecraft:inventory_changed","conditions":{"items":[{"items":item}]}}}}
 write(f"{D}/advancement/tutorial/{name}.json",obj);parent="veio:tutorial/"+name

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
 "1. Primeiro trabalhador\n\nColoque um posto do lavrador perto da plantação e de aldeões.\n\nPonha pão no slot da esquerda e clique no posto com uma esmeralda.\n\nO aldeão colhe e traz a colheita ao posto.",
 "1. First worker\n\nPlace a farmhand post near crops and villagers.\n\nPut bread in the left slot and click the post with an emerald.\n\nThe villager harvests and brings the crop back.")
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
 "A vila cresce\n\nFilhos até o teto: 30, mais 3 por posto com trabalhador, 5 por nível de zona, 2 por armazém e 20 pelo centro de controle.\n\nO prefeito limita: 30 sem prefeitura, depois 60, 100 e 150. Com ele, bebê só nasce com vaga em casa.",
 "The village grows\n\nChildren up to the cap: 30, plus 3 per staffed post, 5 per zone level, 2 per storage and 20 for the control center.\n\nThe mayor limits it: 30 without a town hall, then 60, 100 and 150. With a mayor, babies need a free home.")
v_trouble=vpage(
 "Quando um aldeão para\n\nNoite: ele dorme e volta de manhã.\nSem comida: ponha pão.\nAusente: traga-o ou dispense (agachar + chave).\nCaminho obstruído: abra passagem.\nSaída cheia: esvazie ou use um celeiro.\nCursor no estado: dicas.",
 "When a villager stops\n\nNight: they sleep and return in the morning.\nNo food: add bread.\nMissing: bring them back or dismiss (sneak + wrench).\nPath blocked: clear the way.\nOutput full: empty it or use a barn.\nHover the status for tips.")
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
 "9. Loja\n\nO comerciante é independente. O que você entrega no estoque ele compra no atacado (70% do preço) e paga com as vendas; sem caixa, fica fiado.\n\nAs casas compram de dia; você compra trocando com ele.",
 "9. Shop\n\nThe merchant is independent. What you deliver to the stock they buy wholesale (70% of price) and pay from sales; without cash, on credit.\n\nHomes buy during the day; you buy by trading with them.")
v_mayor=vpage(
 "10. Prefeito e contas\n\nCompanhia (sua): cofre do centro. Paga seus trabalhadores; recebe o atacado.\nPrefeitura: imposto retido na fonte em cada venda (4%, 8%, 12%, 16%). Paga o prefeito e a assistência aos pobres.",
 "10. Mayor and accounts\n\nCompany (yours): center vault. Pays your workers; receives wholesale.\nTown hall: tax withheld at source on every sale (4%, 8%, 12%, 16%). Pays the mayor and assistance.")
v_holding=vpage(
 "Holding\n\nO centro de controle é a sede da companhia. Setas < > trocam a empresa: mineração, manufatura, energia, logística, agro, extrativismo, urbanização, comércio e gestão.\n\nCada aba: unidades, ativas, trabalhadores, produção do dia e o que está parado.",
 "Holding\n\nThe control center is the company headquarters. The < > arrows switch business: mining, manufacturing, power, logistics, agriculture, forestry, urban, retail and management.\n\nEach tab: units, active, workers, daily output and what is stopped.")
v_money=vpage(
 "A moeda\n\nEsmeralda é a moeda: 60 moedas cada. O dinheiro só circula:\nindústria → salários → famílias → lojas → atacado → indústria.\n\nSó entra ou sai em esmeraldas de verdade: depósito, saque e suas compras. O relatório da prefeitura confere a soma.",
 "Money\n\nThe emerald is the currency: 60 coins each. Money only circulates:\nindustry → wages → families → shops → wholesale → industry.\n\nIt only enters or leaves as real emeralds: deposits, cash outs and your purchases. The town hall report checks the sum.")
v_export=vpage(
 "Comércio exterior\n\nO porto de exportação vende ao mercado externo: um navio por minuto. Matéria-prima 1x, processado 1,5x, componente 2x, avançado 3x, módulo 4x.\n\nO mesmo item no mesmo dia vale menos. É moeda nova na sua companhia.",
 "Foreign trade\n\nThe export dock sells abroad: one ship per minute. Raw 1x, processed 1.5x, component 2x, advanced 3x, module 4x.\n\nThe same item on the same day pays less. It is new money in your company.")
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
v_blocks=len(vpages)+1;vpages.append(None)
v_manage=len(vpages)+1;vpages.append(None)
v_reference=len(vpages)+1;vpages.append(None)
v_planlist=len(vpages)+1;vpages.append(None)
vrefs={}
for name in village_order:
 reference_labels(name)
 vrefs[name]=len(vpages)+1
 vpages.append({"text":"","extra":[{"translate":f"block.veio.{name}","bold":True},{"text":"\n"},
  {"translate":f"veio.guide.use.{name}"},{"text":"\n"},{"translate":f"veio.guide.craft.{name}","color":"dark_gray"},{"text":"\n"},
  {"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}}]})
label("veio.village.blocks","Blocos da vila","Village blocks")
extra=[{"translate":"veio.village.blocks","bold":True},{"text":"\n"}]
for name in village_order:
 extra.extend([{"translate":f"block.veio.{name}","color":"dark_blue","underlined":True,
  "click_event":{"action":"change_page","page":vrefs[name]}},{"text":"\n"}])
extra.append({"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}})
vpages[v_blocks-1]={"text":"","extra":extra}
def subindex(page,key,pt,en,links):
 label(key,pt,en)
 extra=[{"translate":key,"bold":True},{"text":"\n"}]
 for n,(lpt,len_,target) in enumerate(links):
  k=f"{key}.{n}";label(k,lpt,len_)
  extra.extend([{"translate":k,"color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":target}},{"text":"\n"}])
 extra.append({"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}})
 vpages[page-1]={"text":"","extra":extra}
MANAGE=[("Começar","Start here",v_intro),("Holding","Holding",v_holding),("A moeda","Money",v_money),("Exportação","Exports",v_export),("Preços","Prices",v_prices),("Foco e obras","Focus and projects",v_focus),("Obras da vila","Village projects",v_projects),
 ("Pedidos e prestígio","Requests and prestige",v_requests),("Decretos e caixa","Decrees and treasury",v_decrees),
 ("Obras maiores","Bigger projects",v_projects2),("Fases e indicadores","Phases and indicators",v_phases)]
REFERENCE=[("Expandir zonas","Grow zones",v_expand),("A vila cresce","Village growth",v_grow),("Quartel","Barracks",v_barracks),
 ("Uma casa de verdade","A real house",v_home2),("Banquete","Banquet",v_banquet),("Problemas e comida","Problems and food",v_trouble),
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
extra=[{"translate":"veio.village.title","bold":True},{"text":"\n"}]
for n,(pt,en,target) in enumerate(vlinks):
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
label("veio.spawn.village","Você está no centro da vila. Ela vai crescer com a sua fábrica.","You are at the village center. It will grow with your factory.")
label("veio.spawn.none","Nenhuma vila encontrada por perto. Construa um centro de controle para fixar o centro da sua vila.","No village found nearby. Build a control center to set your village center.")
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
from build_operations import add_labels
add_labels(langs)
for code,data in langs.items():write(f"{A}/lang/{code}.json",data)
print("Guide pages:",len(pages))
