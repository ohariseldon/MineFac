"""Textos das telas do Veio (`native/api/src/ui`): legenda de cada bloco, a linha de como e por
quê, as famílias e os rótulos dos cartões e palcos, em pt-BR e inglês.

    python tools/build_ui_text.py      # grava nos lang/*.json

`tools/build_tutorial.py` chama `add_labels` também, para quem regenerar tudo não perder estes
textos. A legenda cabe em uma linha do cabeçalho (cerca de 26 letras) e o "como" em duas linhas
do rodapé (cerca de 60 letras); `tools/preview_screens.py` mostra se algo estourou.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
import ui_font  # noqa: E402

# Larguras em pixels da interface (`native/api/src/ui/screens.rs`).
TAG_WIDTH = 144
HOW_WIDTH = 180
CARD_WIDTH = 52

# Bloco: (legenda pt, legenda en, como pt, como en).
BLOCKS = {
    "press": ("Bate minério bruto em chapa", "Hammers raw ore into plates",
              "Bruto entra, chapa sai. A chapa vira engrenagem, fio e moldura.",
              "Raw ore in, plate out. Plates become gears, wire and frames."),
    "assembler": ("Monta engrenagens", "Builds gears",
                  "2 chapas de cobre: 1 engrenagem, peça de módulos e motores.",
                  "2 copper plates make 1 gear, used in modules and machines."),
    "cutter": ("Faz fio e moldura de chapa", "Cuts plates into wire, frames",
               "Cobre vira fiação; ferro vira moldura. A receita segue a chapa.",
               "Copper becomes wire; iron becomes a frame. The plate picks the recipe."),
    "crusher": ("Mói pedra, osso, quartzo", "Crushes stone, bone, quartz",
                "Quartzo vira silício, osso vira farinha e pedregulho vira areia.",
                "Quartz to silicon, bone to meal, cobble down to sand."),
    "integrator": ("Junta peças em módulos", "Joins parts into modules",
                   "Moldura + engrenagem: módulo. Aço + fios: motor. Um de cada slot.",
                   "Frame + gear: module. Steel + wire: motor. One per slot."),
    "alloy_furnace": ("Funde ferro e carvão: aço", "Melts iron and coal: steel",
                      "2 chapas de ferro e 1 carvão por chapa de aço, a base da indústria.",
                      "2 iron plates and 1 coal per steel plate, the base of industry."),
    "electronics": ("Solda circuitos e chips", "Solders circuits and chips",
                    "Fios + silício: circuito. Circuitos + silício: processador.",
                    "Wire + silicon: circuit. Circuits + silicon: processor."),
    "electric_furnace": ("Funde e assa com energia", "Smelts and cooks with power",
                         "Faz o que a fornalha faz, 2,5x mais rápido e sem carvão.",
                         "Does what a furnace does, 2.5x faster and without coal."),
    "fabricator": ("Fabrica núcleos e kits Mk.II", "Makes cores and Mk.II kits",
                   "Motor, processador e aço ou engrenagem: o topo da cadeia.",
                   "Three inputs: motor, processor and steel or gears. Top of the chain."),
    "mill": ("Mói grãos, cana e plantas", "Grinds grain, cane, plants",
             "Trigo vira farinha, cana vira açúcar e plantas viram biomassa.",
             "Wheat becomes flour, cane becomes sugar, plants become biomass."),
    "kitchen": ("Cozinha comida em escala", "Cooks food at scale",
                "Dois ingredientes: pão, bolo, torta, sopa. Comida pronta vale mais.",
                "Two ingredients: bread, cake, pie, soup. Cooked food pays more."),
    "drill": ("Extrai o minério de baixo", "Mines the ore beneath it",
              "Fica sobre a jazida. Pura dobra o ritmo; impura corta pela metade.",
              "Sits on a vein. Pure veins double the pace; impure ones halve it."),
    "drill_mk2": ("Extrai 2x mais rápido", "Mines 2x faster",
                  "O dobro do ritmo e da energia. Só ela tira diamante e esmeralda.",
                  "Twice the pace and the power. Only it mines diamond and emerald."),
    "drill_mk3": ("Extrai 4x, até detrito", "Mines 4x, even debris",
                  "30 FE/t. Única que tira detrito ancestral; veio rico rende 4x.",
                  "30 FE/t. The only one for ancient debris; rich veins yield 4x."),
    "generator": ("Queima carvão: 40 FE/t", "Burns fuel for 40 FE/t",
                  "Carvão, madeira ou tábuas. Cabos levam a energia às máquinas.",
                  "Coal, wood or planks go in. Cables carry the power to machines."),
    "biomass_generator": ("Queima biomassa: 40 FE/t", "Burns biomass for 40 FE/t",
                          "Plantas, feno e biomassa do moinho. Polui menos que o carvão.",
                          "Plants, hay and biomass from the mill. Less smog than coal."),
    "steam_engine": ("Vapor: 120 FE/t com água", "Steam: 120 FE/t with water",
                     "Água parada ao lado ou por cano. Rende o dobro por combustível.",
                     "Still water beside it or piped in. Twice the energy per fuel."),
    "geothermal": ("Lava vira 80 FE/t", "Turns lava into 80 FE/t",
                   "Balde de lava entra, balde vazio sai. Lava encanada também serve.",
                   "Lava bucket in, empty bucket out. Piped lava works too."),
    "solar": ("Luz do sol vira energia", "Turns sunlight into power",
              "Até 16 FE/t ao meio-dia, sob céu aberto. De noite, use acumuladores.",
              "Up to 16 FE/t at noon under open sky. At night, use batteries."),
    "wind_turbine": ("O vento gira as pás", "Wind turns the blades",
                     "8 FE/t no nível do mar; mais alto e com chuva, até 32. Céu aberto.",
                     "8 FE/t at sea level; higher up and in rain, up to 32. Open sky."),
    "water_wheel": ("Água corrente gira a roda", "Running water turns it",
                    "8 FE/t por lado com água corrente, até 32. Sem fumaça.",
                    "8 FE/t per side with flowing water, up to 32. No fuel, no smog."),
    "hydro_turbine": ("Represa vira muita energia", "A dam makes lots of power",
                      "Água atrás, saída livre na frente. Mais queda e espelho, mais FE.",
                      "Water behind, free outlet ahead. More drop and surface, more FE."),
    "battery": ("Guarda energia para depois", "Stores power for later",
                "Enche quando sobra energia e devolve à noite ou nos picos.",
                "Fills when power is spare and gives it back at night or at peaks."),
    "belt": ("Leva itens bloco a bloco", "Carries items block to block",
             "Curvas e rampas sozinhas. Chave fixa a direção. Sem energia.",
             "Turns and ramps by itself. Wrench locks direction. No power."),
    "fast_belt": ("Esteira 2x mais rápida", "Belt at 2x speed",
                  "Até 5 itens/s. Boa na saída do unificador e nas linhas cheias.",
                  "Up to 5 items/s. Good after a merger and on busy lines."),
    "express_belt": ("Esteira 3x mais rápida", "Belt at 3x speed",
                     "Até 10 itens/s, o dobro da rápida. Para as linhas principais.",
                     "Up to 10 items/s, twice the fast belt. For main lines."),
    "belt_tunnel": ("Passa itens por baixo", "Sends items underneath",
                    "Dois túneis na mesma direção, até 6 blocos. Cruza outras linhas.",
                    "Two tunnels facing the same way, up to 6 blocks. Crosses lines."),
    "cargo_lift": ("Sobe ou desce itens", "Lifts items up or down",
                   "Empilhe. O último entrega pela frente. Agache + chave inverte.",
                   "Stack them. The last one delivers forward. Sneak + wrench flips it."),
    "splitter": ("Divide uma esteira em duas", "Splits one belt into two",
                 "Equilibra A e B. Agache + chave: prioridade. Agache + item: filtro.",
                 "Balances A and B. Sneak + wrench: priority. Sneak + item: filter."),
    "merger": ("Junta duas esteiras em uma", "Merges two belts into one",
               "Laterais A e B entram, a frente sai. Agache + chave: prioridade.",
               "Sides A and B in, front out. Sneak + wrench: priority."),
    "inserter": ("Pega atrás, põe na frente", "Takes behind, puts ahead",
                 "Baú, esteira ou máquina. 1 item/s, 40 FE por item. Redstone pausa.",
                 "Chest, belt or machine. 1 item/s, 40 FE per item. Redstone pauses."),
    "fast_inserter": ("Braço 2,5x mais rápido", "Arm at 2.5x speed",
                      "2,5 itens/s, 64 FE por item. Agache + item: filtro.",
                      "2.5 items/s, 64 FE per item. Sneak + item: filter."),
    "long_inserter": ("Braço que alcança 2 blocos", "Arm that reaches 2 blocks",
                      "Pula o bloco do meio. 1 item/s, 60 FE por item.",
                      "Skips the middle block. 1 item/s, 60 FE per item."),
    "planter": ("Planta sementes no 9x9", "Plants seeds in a 9x9",
                "Sementes no slot, terra arada em volta. Fungo vai em areia das almas.",
                "Seeds in the slot, farmland around. Fungus goes on soul sand."),
    "grow_lamp": ("Acelera as plantas abaixo", "Speeds up crops below",
                  "Fica acima da lavoura 9x9 e ilumina a noite. 50 FE por estágio.",
                  "Hangs over a 9x9 field and lights the night. 50 FE per stage."),
    "harvester": ("Colhe e replanta no 9x9", "Harvests, replants a 9x9",
                  "Trigo, cenoura, batata, beterraba e frutas. Farinha de osso acelera.",
                  "Wheat, carrot, potato, beetroot and fruit. Bone meal speeds it up."),
    "tree_farm": ("Planta e corta árvores", "Plants and fells trees",
                  "Mudas no slot. Planta a cada 3 blocos e corta as crescidas.",
                  "Saplings in the slot. Plants every 3 blocks and fells grown trees."),
    "sprinkler": ("Rega a lavoura 9x9", "Waters a 9x9 field",
                  "Água pelos canos molha a terra. Adubo no slot: 3x mais rápido.",
                  "Piped water wets the soil. Fertilizer in the slot: 3x faster."),
    "pump": ("Puxa água para os canos", "Pumps water into pipes",
             "Encoste em água parada: 25 mB/t por fonte, até 4. 6 FE/t.",
             "Touch still water: 25 mB/t per source, up to 4. 6 FE/t."),
    "ranch": ("Cria, tosquia e ordenha", "Breeds, shears and milks",
              "Ração reproduz até 10 por espécie. Balde vira leite; lã e ovos.",
              "Feed breeds up to 10 per kind. Buckets become milk; wool, eggs."),
    "pantry": ("Vende comida ao povoado", "Sells food to the hamlet",
               "Comida entra, esmeralda sai. Pronta vale mais; 3 aldeões: +50%.",
               "Food in, emeralds out. Cooked food pays more; 3 villagers give +50%."),
    "barn": ("Guarda 27 pilhas de tudo", "Holds 27 stacks of anything",
             "O capataz guarda as sobras e abastece as máquinas daqui.",
             "The foreman stores leftovers and feeds machines from here."),
    "silo": ("Guarda grão e ração", "Holds grain, seeds and feed",
             "18 espaços só para o campo. Também entra na fila do capataz.",
             "18 slots just for the farm. Also part of the foreman's queue."),
    "farm_post": ("Lavrador: colhe o 9x9", "Farmhand: harvests a 9x9",
                  "Esmeralda contrata um aldeão. Comida: 2 pontos por colheita.",
                  "An emerald hires a villager. Food in the slot: 2 points per harvest."),
    "hauler_post": ("Carregador: abastece", "Hauler: supplies machines",
                    "Leva os insumos às máquinas a até 6 blocos e traz os produtos.",
                    "Takes supplies to machines within 6 blocks and brings products back."),
    "foreman_desk": ("Capataz: cuida dos postos", "Foreman: runs the posts",
                     "Leva comida a até 6 postos e recontrata com as esmeraldas.",
                     "Feeds up to 6 posts and rehires with the emeralds on the desk."),
    "shepherd_post": ("Pastor: cuida dos animais", "Shepherd: tends the animals",
                      "Tosquia, recolhe ovos e penas e dá ração aos pares por perto.",
                      "Shears, gathers eggs and feathers, feeds pairs nearby."),
    "barracks": ("Quartel: soldados da vila", "Barracks: village soldiers",
                 "Esmeralda contrata e treina. Comida: cada golpe gasta um ponto.",
                 "An emerald hires and trains. Food: each strike costs a point."),
    "field_marker": ("Marca a lavoura da vila", "Marks the village field",
                     "Lavradores perto aram, plantam e colhem aqui. Esmeralda expande.",
                     "Nearby farmhands till, plant and harvest here. Emeralds expand it."),
    "pen_marker": ("Marca o cercado", "Marks the animal pen",
                   "O pastor perto cuida dos animais dentro. A ração escolhe a espécie.",
                   "The shepherd tends animals inside. The feed picks the kind."),
    "export_dock": ("Navio leva e traz do mundo", "Ships trade with the world",
                    "Um navio a cada 2 min leva 9 pilhas. Valor agregado paga mais lá fora.",
                    "A ship every 2 min takes 9 stacks. Added value pays more abroad."),
    "freight_station": ("Trem leva e traz do mundo", "Trains trade with the world",
                        "Um trem por minuto leva 6 pilhas. Frete maior, mas sai sempre.",
                        "A train every minute takes 6 stacks. Pricier freight, but often."),
    "rail_terminal": ("Envia carga por trilho", "Sends cargo by rail",
                      "Dois terminais com o mesmo corante formam a rota. Funis ligam vagões.",
                      "Two terminals with the same dye form a route. Hoppers link carts."),
    "drone_port": ("Drone leva carga pelo ar", "Drones fly cargo",
                   "Mesmo corante liga os portos. 64 blocos, 16 itens por voo, energia.",
                   "Same dye links the ports. 64 blocks, 16 items per flight, power."),
    "caravan_station": ("Animal leva carga por terra", "Animals carry cargo by land",
                        "Laço vincula um animal. Ração no slot de baixo; o caminho livre.",
                        "A lead binds an animal. Feed in the lower slot; keep the path clear."),
    "cargo_station": ("Manda carga ao canal", "Ships to the channel",
                      "Recebe das esteiras e manda ao canal, mesmo longe de você.",
                      "Takes from belts and sends to the channel. Keeps pace while away."),
    "unloading_station": ("Recebe carga na base", "Receives cargo at the base",
                          "Entrega o que chega pelo canal da mesma cor. Ligue esteira ou baú.",
                          "Delivers what arrives on its colour channel. Add a belt or chest."),
    "house": ("Lar de até 4 aldeões", "Home for up to 4 villagers",
              "Moradores trabalham, comem da despensa e compram na loja.",
              "Residents work, eat from the pantry and shop at the store."),
    "shop": ("Loja do comerciante", "The merchant's store",
             "Ele compra no atacado o que você entrega e paga com as vendas.",
             "He buys what you deliver at wholesale and pays from his sales."),
    "well": ("Saneamento das casas", "Sanitation for homes",
             "Atende casas a até 24 blocos. Balde vazio vira balde d'água.",
             "Serves homes within 24 blocks. Fills empty buckets."),
    "tesla": ("Abate monstros a 10 blocos", "Zaps monsters in 10 blocks",
              "2.400 FE por disparo. Fumaça atrai ataques; use acumulador.",
              "2,400 FE per shot. Smog draws raids; at night, use batteries."),
    "chunk_anchor": ("Mantém o chunk carregado", "Keeps its chunk loaded",
                     "A fábrica segue longe de você. Gasta 32 FE/t; sem energia, solta.",
                     "The factory runs while you are away. 32 FE/t, or it lets go."),
    "research_lab": ("Pesquisa novas tecnologias", "Researches new technology",
                     "Entregue centenas de peças; cada pesquisa libera máquinas.",
                     "Deliver hundreds of parts; each study unlocks machines."),
    "control_center": ("Sede da companhia e da vila", "Company and village HQ",
                       "", ""),
    "town_hall": ("Prefeitura: impostos e IDH", "Town hall: taxes and HDI",
                  "O prefeito mede a vila e diz o que falta. Livro: relatório no chat.",
                  "The mayor rates the town and says what it lacks. Book: report."),
    "home": ("Sua casa na vila", "Your home in the village",
             "Lenha na lareira, comida na despensa. Material sobe o nível.",
             "Fuel lights the fire, the pantry feeds. Materials level it up."),
    "site": ("Obra que sai do papel", "A building rising from plans",
             "Chave troca a planta. Esmeralda contrata o mestre; comida no slot.",
             "Wrench picks the plan. An emerald hires the builder; food in the slot."),
    "factory_board": ("Fases e pedidos da fábrica", "Factory stages and orders",
                      "Entregue a peça da fase: ela paga e libera máquinas novas.",
                      "Deliver the stage part: it pays and unlocks new machines."),
    "contract_hub": ("Pedidos com recompensa", "Orders with rewards",
                     "O pedido entra à esquerda; a recompensa sai à direita.",
                     "The order goes in on the left; the reward comes out right."),
    "factory_monitor": ("Resumo da fábrica inteira", "Whole-factory overview",
                        "Vermelho: a linha gasta mais do que produz.",
                        "Red: the line uses more than it makes."),
}

FAMILIES = {
    "industry": ("Indústria", "Industry"), "extraction": ("Extração", "Mining"),
    "energy": ("Energia", "Power"), "logistics": ("Logística", "Logistics"),
    "farming": ("Campo", "Farming"), "food": ("Alimentos", "Food"),
    "storage": ("Armazém", "Storage"), "workforce": ("Trabalho", "Workforce"),
    "society": ("Vila", "Village"), "trade": ("Comércio", "Trade"),
    "management": ("Gestão", "Management"), "defense": ("Defesa", "Defense"),
}

# Nome curto de cada cartão (embaixo do número).
CARDS = {
    "rate": ("produção", "output"), "uptime": ("ativa", "active"), "cycles": ("ciclos", "cycles"),
    "smog": ("fumaça", "smog"), "deposit": ("jazida", "vein"), "fe_tick": ("FE/t", "FE/t"),
    "energy": ("energia", "energy"), "water": ("água mB", "water mB"), "capacity": ("máximo", "maximum"),
    "charge": ("carga", "charge"), "direction": ("sentido", "direction"), "speed": ("ritmo", "speed"),
    "planted": ("plantios", "planted"), "grown": ("estágios", "stages"), "trees": ("árvores", "trees"),
    "harvests": ("colheitas", "harvests"), "pumping": ("mB/t", "mB/t"), "animals": ("animais", "animals"),
    "slots": ("espaços", "slots"), "tasks": ("colheitas", "harvests"), "trips": ("viagens", "trips"),
    "chores": ("tarefas", "chores"), "posts": ("postos", "posts"), "blocks": ("blocos", "blocks"), "plan": ("planta", "blueprint"),
    "kills": ("abates", "kills"), "villagers": ("aldeões", "villagers"), "work": ("trabalho", "work"),
    "departs": ("partida", "departs"), "sold_today": ("vendidos", "sold"),
    "profit_today": ("lucro", "profit"), "route": ("rota", "route"),
    "per_trip": ("por ida", "per trip"), "trips_done": ("viagens", "trips"), "channel": ("canal", "channel"),
    "residents": ("família", "family"), "idh": ("IDH", "HDI"), "wallet": ("carteira", "wallet"),
    "sales": ("vendas", "sales"), "stock": ("estoque", "stock"), "owes": ("fiado", "owed"),
    "cash": ("caixa", "cash"), "transit": ("no canal", "in channel"), "head": ("queda", "drop"), "radius": ("blocos", "blocks"), "sold": ("vendas", "sales"),
    "phase": ("fase", "stage"), "contract": ("contrato", "contract"), "exports": ("entregas", "deliveries"),
    "order": ("pedido", "order"), "prize": ("prêmio", "reward"), "machines": ("máquinas", "machines"),
}

# Rótulos e frases curtas dos palcos e das dicas.
LABELS = {
    "veio.ui.label.unlocks": ("libera", "unlocks"),
    "veio.ui.label.fuel": ("lenha", "fuel"),
    "veio.ui.label.build": ("obra", "build"),
    "veio.ui.label.wage": ("salário", "wage"),
    "veio.ui.board.done": ("Todas as fases concluídas", "All stages complete"),
    "veio.ui.monitor.item": ("Item · por minuto", "Item · per minute"),
    "veio.ui.monitor.made": ("produz", "makes"),
    "veio.ui.monitor.used": ("gasta", "uses"),
    "veio.ui.monitor.counts": ("%s ativas · %s esperam · %s paradas", "%s active · %s waiting · %s stopped"),
    "veio.ui.module.speed": ("Velocid.", "Speed"),
    "veio.ui.module.efficiency": ("Eficiên.", "Effic."),
    "veio.ui.module.productivity": ("Produt.", "Produc."),
    "veio.ui.purity.unknown": ("?", "?"),
    "veio.ui.purity.impure": ("½x", "½x"),
    "veio.ui.purity.normal": ("1x", "1x"),
    "veio.ui.purity.pure": ("2x", "2x"),
    "veio.ui.purity.rich": ("4x", "4x"),
    "veio.ui.burn.left": ("Resta %s", "%s left"),
    "veio.ui.burn.out": ("Sem fogo", "No fire"),
    "veio.ui.max.solar": ("máx. %s ao meio-dia", "max %s at noon"),
    "veio.ui.max.wind": ("máx. %s no alto, com chuva", "max %s up high, in rain"),
    "veio.ui.max.water_wheel": ("máx. %s com 4 lados", "max %s on 4 sides"),
    "veio.ui.max.hydro": ("máx. %s com represa alta", "max %s with a tall dam"),
    "veio.ui.head": ("Queda: %s blocos", "Drop: %s blocks"),
    "veio.ui.tunnel": ("até %s blocos por baixo", "up to %s blocks under"),
    "veio.ui.lift.up": ("sobe", "up"),
    "veio.ui.lift.down": ("desce", "down"),
    "veio.ui.route.balanced": ("Equilibrado", "Balanced"),
    "veio.ui.route.left": ("Primeiro A", "A first"),
    "veio.ui.route.right": ("Primeiro B", "B first"),
    "veio.ui.route.sorted": ("Filtro em A", "Filter on A"),
    "veio.ui.arm.from": ("de trás", "from back"),
    "veio.ui.arm.to": ("para a frente", "to front"),
    "veio.ui.arm.filtered": ("Só o item do filtro", "Filtered item only"),
    "veio.ui.arm.any": ("Qualquer item", "Any item"),
    "veio.ui.pump.rate": ("%s mB/t", "%s mB/t"),
    "veio.ui.label.harvest": ("colheita", "harvest"),
    "veio.ui.label.products": ("produtos", "products"),
    "veio.ui.label.collected": ("recolhido", "collected"),
    "veio.ui.label.leftover": ("sobra", "leftover"),
    "veio.ui.label.supply": ("insumos", "supplies"),
    "veio.ui.label.food": ("comida", "food"),
    "veio.ui.label.pantry": ("despensa", "pantry"),
    "veio.ui.label.stock": ("estoque à venda", "stock for sale"),
    "veio.ui.label.cash": ("caixa", "cash"),
    "veio.ui.label.to_send": ("para enviar", "to send"),
    "veio.ui.label.arrived": ("chegou", "arrived"),
    "veio.ui.foreman.posts": ("%s postos", "%s posts"),
    "veio.ui.foreman.queue": ("fila: %s", "queue: %s"),
    "veio.ui.soldier.1": ("Milícia", "Militia"),
    "veio.ui.soldier.2": ("Guarda", "Guard"),
    "veio.ui.soldier.3": ("Arqueiro", "Archer"),
    "veio.ui.soldier.4": ("Mago", "Mage"),
    "veio.ui.kills": ("%s abates", "%s kills"),
    "veio.ui.storage.used": ("%s de %s espaços", "%s of %s slots"),
    "veio.ui.zone.size": ("Área %sx%s", "Area %sx%s"),
    "veio.ui.zone.level": ("Nível %s de %s", "Level %s of %s"),
    "veio.ui.zone.next": ("Próximo: %s", "Next: %s"),
    "veio.ui.zone.max": ("Nível máximo", "Max level"),
    "veio.ui.port.departs": ("Partida em %s", "Departs in %s"),
    "veio.ui.port.hold": ("%s de %s pilhas", "%s of %s stacks"),
    "veio.ui.shop.prices": ("preços %s%%", "prices %s%%"),
    "veio.ui.pantry.credit": ("%s/%s", "%s/%s"),
    "veio.ui.tesla.smog": ("Fumaça", "Smog"),
    "veio.ui.anchor.on": ("Chunk carregado", "Chunk loaded"),
    "veio.ui.anchor.off": ("Sem energia: solto", "Unpowered: released"),
    "veio.ui.anchor.cost": ("%s FE/t", "%s FE/t"),
    "veio.ui.lab.stage": ("Etapa %s/%s", "Stage %s/%s"),
    "veio.ui.hint.cycles": ("Ciclos completos desde que foi colocada.", "Cycles completed since it was placed."),
    "veio.ui.hint.fe_tick": ("Energia gerada agora, por tick.", "Power generated right now, per tick."),
    "veio.ui.hint.belt1": ("2,5 itens por segundo.", "2.5 items per second."),
    "veio.ui.hint.belt2": ("5 itens por segundo.", "5 items per second."),
    "veio.ui.hint.belt3": ("10 itens por segundo.", "10 items per second."),
    "veio.ui.hint.blocks": ("Blocos assentados de todos os da planta.", "Blocks laid out of the whole plan."),
    "veio.ui.hint.zone_villagers": ("Aldeões na vila e os que o próximo nível pede.", "Villagers in the village and how many the next level needs."),
    "veio.ui.hint.zone_work": ("Trabalho feito na zona e o que o próximo nível pede.", "Work done in the zone and what the next level needs."),
    "veio.ui.hint.sold_today": ("Itens vendidos hoje por esta conexão.", "Items sold today through this link."),
    "veio.ui.hint.profit_today": ("Moedas líquidas hoje: vendas menos compras e frete.", "Net coins today: sales minus purchases and freight."),
    "veio.ui.hint.route": ("Corante no terminal: só o par da mesma cor troca carga.", "Dye on the terminal: only the same-colour pair trades cargo."),
    "veio.ui.hint.trips_done": ("Viagens completas desta rota.", "Trips completed on this route."),
    "veio.ui.hint.idh": ("Bem-estar da casa: comida, renda, saneamento, energia e comércio.", "Home wellbeing: food, income, sanitation, power and trade."),
    "veio.ui.hint.cash": ("Moedas no caixa da loja.", "Coins in the store's till."),
    "veio.ui.hint.pantry_sold": ("Vendas feitas ao povoado.", "Sales made to the hamlet."),
    "veio.ui.hint.energy_in": ("A máquina gasta daqui; cabos e postes enchem.", "The machine draws from here; cables and poles fill it."),
    "veio.ui.hint.energy_out": ("Reserva do gerador; a rede puxa daqui.", "Generator buffer; the grid draws from here."),
    "veio.ui.hint.goes_to": ("Usado em: %s", "Used in: %s"),
    "veio.ui.hint.transit": ("Itens no canal, a caminho da descarga (até %s).", "Items in the channel, on the way to unloading (up to %s)."),
}


def add_labels(langs):
    pt, en = langs["pt_br"], langs["en_us"]
    for name, (tag_pt, tag_en, how_pt, how_en) in BLOCKS.items():
        pt[f"veio.ui.tag.{name}"], en[f"veio.ui.tag.{name}"] = tag_pt, tag_en
        if how_pt:
            pt[f"veio.ui.how.{name}"], en[f"veio.ui.how.{name}"] = how_pt, how_en
    for name, (family_pt, family_en) in FAMILIES.items():
        pt[f"veio.ui.family.{name}"], en[f"veio.ui.family.{name}"] = family_pt, family_en
    for name, (card_pt, card_en) in CARDS.items():
        pt[f"veio.ui.card.{name}"], en[f"veio.ui.card.{name}"] = card_pt, card_en
    for key, (text_pt, text_en) in LABELS.items():
        pt[key], en[key] = text_pt, text_en


def width(text):
    return sum(ui_font.advance(char) for char in text)


def lines(text, limit):
    """Quebra por palavras como `Pen::wrap`."""
    out, line = [], ""
    for word in text.split(" "):
        candidate = f"{line} {word}" if line else word
        if line and width(candidate) > limit:
            out.append(line)
            line = word
        else:
            line = candidate
    return out + [line]


def check():
    """Legenda numa linha, "como" em duas e cartões curtos, nas duas línguas."""
    problems = []
    for name, (tag_pt, tag_en, how_pt, how_en) in BLOCKS.items():
        for tag, how in ((tag_pt, how_pt), (tag_en, how_en)):
            if width(tag) > TAG_WIDTH:
                problems.append(f"legenda de {name} passa de {TAG_WIDTH}px: {tag}")
            if how and len(lines(how, HOW_WIDTH)) > 2:
                problems.append(f"'como' de {name} passa de duas linhas: {how}")
    for name, texts in CARDS.items():
        for text in texts:
            if width(text) > CARD_WIDTH:
                problems.append(f"cartão {name} passa de {CARD_WIDTH}px: {text}")
    return problems


def main():
    problems = check()
    if problems:
        raise SystemExit("\n".join(problems))
    paths = {code: ROOT / f"content/assets/veio/lang/{code}.json" for code in ("pt_br", "en_us")}
    langs = {code: json.loads(path.read_text(encoding="utf-8")) for code, path in paths.items()}
    add_labels(langs)
    for code, path in paths.items():
        path.write_text(json.dumps(langs[code], ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"{len(BLOCKS)} blocos, {len(CARDS)} cartões, {len(LABELS)} rótulos")


if __name__ == "__main__":
    main()
