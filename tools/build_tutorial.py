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
 "wrench":(
 "Selecione na barra rápida.\nBotão direito gira máquinas\ne esteiras.\nConserva estoques e progresso.",
 "Select it in the hotbar.\nRight-click to turn machines\nand conveyors.\nKeeps contents and progress."),
}
order=["generator","cable","drill","belt","press","assembler","wrench","cutter","integrator",
 "contract_hub","splitter","fast_belt","alloy_furnace","crusher","electronics","geothermal",
 "solar","battery","drill_mk2","harvester","tesla","pole","steel_pole"]
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
 "contract_hub":["First ingredient in the displayed order.","Second ingredient in the displayed order.","First reward.","Second reward."],
}
for block,lines in slothelp.items():
 for i,line in enumerate(lines): label(f"veio.slothelp.{block}.{i}",line,slot_en[block][i])

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
index_pages=[]
for group in range(5):
 index_pages.append(len(pages)+1);pages.append(None)
references={}
for name in order:
 title={code:item_name("veio:"+name,code) for code in langs}
 usage_key=f"veio.guide.use.{name}"
 for code in langs:
  langs[code][usage_key]=title[code]+"\n\n"+help[name][0 if code=="pt_br" else 1]
 use_index=len(pages)+1; recipe_index=use_index+1
 pages.append({"text":"","extra":[{"translate":usage_key},{"text":"\n\n"},
  {"translate":"veio.guide.recipe_link","color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":recipe_index}},
  {"text":" | "},{"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}}]})
 recipe=read(f"{D}/recipe/{name}.json")
 ingredients=Counter(recipe["ingredients"])
 recipe_key=f"veio.guide.make.{name}"
 for code in langs:
  lines=[("Fabricar " if code=="pt_br" else "Craft ")+title[code],""]
  lines += [f"{count} x {item_name(item,code)}" for item,count in ingredients.items()]
  lines += ["",("Resultado: " if code=="pt_br" else "Makes: ")+str(recipe["result"].get("count",1))]
  langs[code][recipe_key]="\n".join(lines)
 pages.append({"text":"","extra":[{"translate":recipe_key},{"text":"\n\n"},
  {"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}}]})
 references[name]=use_index
for g,index in enumerate(index_pages):
 members=order[g*5:g*5+5]
 key=f"veio.guide.index.{g}"
 label(key,"Máquinas e ferramentas\n\n","Machines and tools\n\n")
 extra=[{"translate":key}]
 for name in members:
  kind="block" if name!="wrench" else "item"
  extra.extend([{"translate":f"{kind}.veio.{name}","color":"dark_blue","underlined":True,
                 "click_event":{"action":"change_page","page":references[name]}},{"text":"\n"}])
 extra.extend([{"text":"\n"},{"translate":"veio.guide.back","color":"dark_blue","click_event":{"action":"change_page","page":1}}])
 pages[index-1]={"text":"","extra":extra}
links=[
 ("Começar","Start here",intro),("Materiais","Gather materials",materials),
 ("Montar a linha","Build the line",layout),("Resolver falhas","Troubleshoot",troubleshoot),
 ("Máquinas A","Machines A",index_pages[0]),("Máquinas B","Machines B",index_pages[1]),
 ("Máquinas C","Machines C",index_pages[2]),("Máquinas D","Machines D",index_pages[3]),
 ("Máquinas E","Machines E",index_pages[4]),("Fábrica e mundo","Factory and world",world),
 ("Postes e fios","Poles and wires",wiring),("Poluição e defesa","Pollution and defense",smog),
 ("Módulos","Modules",upgrades),
]
extra=[{"translate":"veio.guide.title","bold":True},{"text":"\n\n"}]
for n,(pt,en,target) in enumerate(links):
 key=f"veio.guide.contents.{n}";label(key,pt,en)
 extra.extend([{"translate":key,"color":"dark_blue","underlined":True,"click_event":{"action":"change_page","page":target}},{"text":"\n"}])
pages[0]={"text":"","extra":extra}
book={"minecraft:written_book_content":{"title":{"raw":"Veio 0.8"},"author":"Veio",
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
for code,data in langs.items():write(f"{A}/lang/{code}.json",data)
print("Guide pages:",len(pages))
