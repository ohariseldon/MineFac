"""Focused freight handbook; generated with the main tutorial's languages."""


def build(write, languages):
    entries = [
        ("Escolher transporte", "Choose transport",
         "Escolha pela função:\n\nPorco: carga inicial.\nCavalo: leve e rápido.\nBurro: carga maior.\nDrone: ligação curta.\nTrem: volume alto.\n\nCaravanas usam estrada.",
         "Choose by purpose:\n\nPig: early cargo.\nHorse: light and fast.\nDonkey: heavier cargo.\nDrone: short air route.\nRail: high volume.\n\nCaravans need a road."),
        ("Ligar duas zonas", "Connect two zones",
         "Use o MESMO CORANTE em dois terminais do mesmo tipo. A cor liga a rota, sem gastar o corante.\n\nExatamente duas pontas por cor.\n\nO cartão permite escolher o canal por número; 0 desliga.",
         "Use the SAME DYE on two terminals of the same type. Color links the route; dye is kept.\n\nExactly two ends per color.\n\nThe card selects a numbered channel instead; 0 disables it."),
        ("Envio e recebimento", "Dispatch and receive",
         "Agachar + produto define o ENVIO, à esquerda.\n\nÀ direita chega a carga da outra ponta.\n\nClique com item sem agachar para fixar o recebimento manual.\n\nMão vazia abre o painel.",
         "Sneak + product copies outgoing cargo. Left slots hold dispatch stock.\n\nRight slots receive the other end's product. Use an item without sneaking to fix a manual receiving filter.\n\nEmpty hand opens the panel."),
        ("Trabalho dos aldeões", "Villager jobs",
         "Ponha uma mesa de capataz e um posto do carregador perto das máquinas e do terminal. Contrate e alimente os dois.\n\nO capataz cria a fila. O carregador leva o envio e usa o recebido nas máquinas compatíveis.",
         "Place a foreman desk and hauler post near machines and terminal. Hire and feed both.\n\nThe foreman plans the queue. The hauler fills dispatch and uses received goods in compatible machines."),
        ("Ferrovia", "Rail freight",
         "Funis ligam o terminal ao vagão com baú.\n\nSepare a carga da descarga ou alterne os funis com redstone. Evite retirar o que acabou de carregar.\n\nVocê define trilhos e paradas.",
         "Hoppers link terminals and chest minecarts.\n\nSeparate loading from unloading, or alternate hoppers with redstone. Avoid pulling new cargo straight back out.\n\nYou define rails and stops."),
        ("Drones", "Drones",
         "Até 64 blocos e 16 itens por voo. Energize os portos.\n\nDeixe livre o trajeto direto, três blocos acima deles. Obstáculos e chunks descarregados pausam o voo.\n\nAgachar + chave chama o drone de volta.",
         "Up to 64 blocks and 16 items per flight. Power both ports.\n\nClear the direct path three blocks above them. Obstacles and unloaded chunks pause travel.\n\nSneak + wrench recalls the drone."),
        ("Preparar o animal", "Prepare an animal",
         "Traga um adulto livre a até 8 blocos.\n\nCavalo: domado e selado.\nBurro: domado, com baú.\nPorco: selado.\n\nUse laço no terminal para vincular, sem gastar o laço.",
         "Bring a free adult animal within 8 blocks.\n\nHorse: tamed and saddled.\nDonkey/mule: tamed with chest.\nPig: saddled.\n\nUse a lead on the caravan terminal to assign it. The lead is kept."),
        ("Ração e capacidade", "Feed and capacity",
         "Uma ração paga ida e volta. Slot de baixo.\n\nCavalo: 16 itens.\nBurro/mula: 64 itens.\nTrigo, maçã ou feno.\n\nPorco: 8 itens.\nCenoura, batata ou beterraba. Sem energia.",
         "One feed item pays for both legs. Bottom slot.\n\nHorse: 16 items.\nDonkey/mule: 64 items.\nWheat, apple or hay.\n\nPig: 8 items.\nCarrot, potato, beetroot. No power."),
        ("Estrada e retorno", "Road and return",
         "Caravanas aceitam destinos a até 512 blocos. O animal caminha de verdade; só entrega ao chegar.\n\nEstrada larga e sem quedas ajuda. Montar ou prender o animal pausa seu trabalho.\n\nAgachar + chave manda voltar.",
         "Caravans accept targets up to 512 blocks away. The animal really walks; delivery requires arrival.\n\nUse a wide road without drops. Riding or leashing pauses its job.\n\nSneak + wrench recalls it."),
        ("Recuperar uma carga", "Recover cargo",
         "Sem destino, espaço ou animal, a carga fica na estação.\n\nAgachar + laço desvincula e devolve a carga não entregue, se houver espaço.\n\nNão teleporta o animal.",
         "No destination, room or animal: cargo stays at the terminal.\n\nSneak + lead releases the animal and restores undelivered cargo if room allows.\n\nNo animal teleportation."),
        ("Áreas descarregadas", "Unloaded areas",
         "Caravanas precisam de origem e animal ativos. Descarregar a origem pausa a viagem.\n\nO destino precisa estar ativo para receber.\n\nAlcance de 512 blocos não carrega o mundo.",
         "Caravans need an active origin and animal. Unloading the origin pauses travel.\n\nDelivery needs an active destination.\n\n512-block range does not force world loading."),
    ]
    def label(key, pt, en):
        languages["pt_br"][key] = pt
        languages["en_us"][key] = en
    label("veio.transport.title", "Guia de Transportes", "Freight Guide")
    contents = [{"translate": "veio.transport.title", "bold": True}, {"text": "\n\n"}]
    pages = []
    for index, (pt_title, en_title, pt, en) in enumerate(entries):
        key = f"veio.transport.page.{index + 1}"
        # Conservative readability budget; this is not a rendered-font measurement.
        import textwrap
        for text in (pt, en):
            lines = sum(max(1, len(textwrap.wrap(line, width=20))) for line in text.split("\n"))
            assert lines <= 14, (key, lines)
        label(key, pt, en)
        title = f"veio.transport.contents.{index + 1}"
        label(title, pt_title, en_title)
        contents.extend([{"translate": title, "color": "dark_blue", "underlined": True,
                          "click_event": {"action": "change_page", "page": index + 2}}, {"text": "\n"}])
        pages.append({"translate": key})
    pages.insert(0, {"text": "", "extra": contents})
    components = {
        "minecraft:written_book_content": {"title": {"raw": "Transportes"}, "author": "Veio", "generation": 0,
                                            "resolved": True, "pages": [{"raw": page} for page in pages]},
        "minecraft:custom_name": {"translate": "veio.transport.title", "italic": False},
    }
    write("data/veio/recipe/transport_guide.json", {"type": "minecraft:crafting_shapeless", "category": "misc",
          "ingredients": ["minecraft:book", "minecraft:rail"],
          "result": {"id": "minecraft:written_book", "count": 1, "components": components}})
    write("data/veio/loot_table/tutorial/transport_guide.json", {"type": "minecraft:generic", "pools": [{
        "rolls": 1, "entries": [{"type": "minecraft:item", "name": "minecraft:written_book",
                                  "functions": [{"function": "minecraft:set_components", "components": components}]}]}]})
    criteria = {name: {"trigger": "minecraft:inventory_changed", "conditions": {"items": [{"items": f"veio:{name}"}]}}
                for name in ("freight_station", "drone_port", "caravan_station")}
    write("data/veio/advancement/transport_guide.json", {"parent": "veio:root", "criteria": criteria,
          "requirements": [list(criteria)], "rewards": {"recipes": ["veio:transport_guide"], "loot": ["veio:tutorial/transport_guide"]}})
    print("Transport guide pages:", len(pages))
