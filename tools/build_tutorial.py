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
 "hopper":("Funil","Hopper"),"lectern":("Atril","Lectern"),"bell":("Sino","Bell"),"lead":("Laço","Lead"),
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
help["merger"] = (
 "Une as duas entradas laterais.\nSaída única pela frente.\nAlterna A e B, até 5 itens/s.\nSem energia elétrica.\nUse a chave para girar.",
 "Merges the two side inputs.\nSingle outlet at the front.\nAlternates A/B, up to 5/s.\nNeeds no electricity.\nTurn with the wrench.")
# Trabalhadores (frente Cloud): posto agrícola com aldeão contratado.
label("block.veio.farm_post", "Posto do lavrador", "Farmhand Post")
help["hauler_post"] = (
 "Esmeralda: contrata aldeão.\nAbastece máquinas a até 6\nblocos com os insumos e\nrecolhe os produtos delas.\nNão mexe em esteiras.",
 "Emerald: hires a villager.\nFeeds machines within 6\nblocks from the supplies\nand collects their products.\nLeaves belts alone.")
help["shepherd_post"] = (
 "Esmeralda: contrata aldeão.\nRecolhe ovos, penas e lã,\ntosquia ovelhas e dá ração\naos pares, a até 6 blocos.\nNão ordenha nem abate.",
 "Emerald: hires a villager.\nPicks up eggs, feathers and\nwool, shears sheep and feeds\npairs within 6 blocks.\nNo milking or culling.")
help["foreman_desk"] = (
 "Esmeralda: contrata o capataz.\nCuida de até 6 postos a\naté 12 blocos: leva comida\ne recontrata com esmeraldas\nda mesa. Monta a fila.",
 "Emerald: hires the foreman.\nManages up to 6 posts within\n12 blocks: brings food and\nrehires with the desk's\nemeralds. Queues transfers.")
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
categories=[
 ("Extração e processamento","Mining and processing",["drill","drill_mk2","drill_mk3","press","assembler","cutter","integrator","crusher","alloy_furnace","electronics","hauler_post"]),
 ("Lavoura e comida","Farming and food",["harvester","planter","grow_lamp","tree_farm","ranch","farm_post","shepherd_post","mill","electric_furnace","kitchen","pantry"]),
 ("Energia","Power",["generator","biomass_generator","water_wheel","wind_turbine","steam_engine","geothermal","solar","battery"]),
 ("Logística","Logistics",["belt","fast_belt","splitter","merger","cable","pole","steel_pole","wrench"]),
 ("Projetos e defesa","Projects and defense",["contract_hub","tesla","foreman_desk"]),
]
order=[name for _,_,names in categories for name in names]
assert set(order)==set(help), set(order)^set(help)
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
 "Saqueadores\n\nO chat avisa quando a fumaça chama atenção. Um dia depois, com poluição alta, eles atacam à noite as máquinas que poluem.\n\nMáquina danificada para: use um kit de reparo. Muros e torres Tesla defendem.",
 "Raiders\n\nChat warns when the smoke draws attention. A day later, with high pollution, they attack polluting machines at night.\n\nDamaged machines stop: use a repair kit. Walls and Tesla towers defend.")
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
 "Trabalhadores\n\nNo começo você opera a lavoura. Depois, contrate um aldeão: esmeralda no posto do lavrador.\n\nEle anda até a planta madura, colhe e traz ao posto. Máquinas seguem automáticas.\n\nComida no posto paga o trabalho.",
 "Workers\n\nAt first you run the farm. Later, hire a villager: emerald on a farmhand post.\n\nThey walk to ripe crops, harvest and bring them back. Machines stay automatic.\n\nFood in the post pays for the work.")
page(
 "Carregador e capataz\n\nPosto do carregador: insumos à esquerda; o aldeão abastece as máquinas perto e traz os produtos.\n\nMesa do capataz: comida e esmeraldas; ele alimenta e recontrata os postos a até 12 blocos.",
 "Hauler and foreman\n\nHauler post: supplies on the left; the villager feeds nearby machines and brings products back.\n\nForeman desk: food and emeralds; it feeds and rehires posts within 12 blocks.")
page(
 "Fila e pastor\n\nCom o capataz presente, a mesa monta uma fila: produtos de um posto ou máquina para outra que os aceite. Carregadores perto pegam a fila primeiro.\n\nPosto do pastor: ração, tosquia e coleta de ovos e lã.",
 "Queue and shepherd\n\nWith the foreman present, the desk queues transfers: products from a post or machine to another that accepts them. Nearby haulers take the queue first.\n\nShepherd post: feed, shearing, eggs and wool.")
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
category_pages=[]
for _ in categories:
 category_pages.append(len(pages)+1);pages.append(None)
recipe_index=len(pages)+1;pages.append(None)
references={}
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
  extra.extend([{"translate":f"{kind}.veio.{name}","color":"dark_blue","underlined":True,
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
book={"minecraft:written_book_content":{"title":{"raw":"Veio 0.12.0"},"author":"Veio",
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
from build_operations import add_labels
add_labels(langs)
for code,data in langs.items():write(f"{A}/lang/{code}.json",data)
print("Guide pages:",len(pages))
