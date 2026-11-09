"""Wiki do Veio: dados da tela da wiki no jogo (native/api/src/wiki/data.rs) e textos.

Uma fonte só: as receitas de bancada (content/data/veio/recipe), as receitas de máquina
(tools/production.json, lidas no jogo direto de production::data), as abas do criativo, os
avanços que liberam cada receita e os textos abaixo. Chamado por build_tutorial.py, que escreve
os arquivos de idioma; o site da wiki escrita sai de tools/wiki_site.py com os mesmos dados.
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "content/data/veio"
OUT = ROOT / "native/api/src/wiki/data.rs"

WOODS = {"oak": ("carvalho", "Oak"), "spruce": ("pinheiro", "Spruce"), "birch": ("bétula", "Birch"),
         "jungle": ("selva", "Jungle"), "acacia": ("acácia", "Acacia"), "dark_oak": ("carvalho-escuro", "Dark oak"),
         "mangrove": ("mangue", "Mangrove"), "cherry": ("cerejeira", "Cherry"), "pale_oak": ("carvalho-pálido", "Pale oak")}

# Nomes do Minecraft que a tabela do guia (build_tutorial.vanilla) ainda não tem.
VANILLA = {
    "amethyst_block": ("Bloco de ametista", "Block of amethyst"), "amethyst_shard": ("Fragmento de ametista", "Amethyst shard"),
    "ancient_debris": ("Detritos ancestrais", "Ancient debris"), "apple": ("Maçã", "Apple"),
    "baked_potato": ("Batata assada", "Baked potato"), "bamboo": ("Bambu", "Bamboo"), "bamboo_block": ("Bloco de bambu", "Block of bamboo"),
    "beef": ("Bife cru", "Raw beef"), "beetroot": ("Beterraba", "Beetroot"), "beetroot_seeds": ("Sementes de beterraba", "Beetroot seeds"),
    "beetroot_soup": ("Sopa de beterraba", "Beetroot soup"), "blackstone": ("Pedra-negra", "Blackstone"), "blaze_rod": ("Vara de blaze", "Blaze rod"),
    "blue_dye": ("Corante azul", "Blue dye"), "bone": ("Osso", "Bone"), "bone_meal": ("Farinha de osso", "Bone meal"),
    "bowl": ("Tigela", "Bowl"), "bread": ("Pão", "Bread"), "brick": ("Tijolo", "Brick"), "brown_mushroom": ("Cogumelo marrom", "Brown mushroom"),
    "budding_amethyst": ("Ametista em broto", "Budding amethyst"), "cactus": ("Cacto", "Cactus"), "cake": ("Bolo", "Cake"),
    "carrot": ("Cenoura", "Carrot"), "charcoal": ("Carvão vegetal", "Charcoal"), "chicken": ("Frango cru", "Raw chicken"),
    "chiseled_stone_bricks": ("Tijolos de pedra talhados", "Chiseled stone bricks"), "clay": ("Argila", "Clay"),
    "clay_ball": ("Bola de argila", "Clay ball"), "clock": ("Relógio", "Clock"), "coal_block": ("Bloco de carvão", "Block of coal"),
    "coal_ore": ("Minério de carvão", "Coal ore"), "cobbled_deepslate": ("Ardósia em pedregulho", "Cobbled deepslate"),
    "cocoa_beans": ("Sementes de cacau", "Cocoa beans"), "cod": ("Bacalhau cru", "Raw cod"), "compass": ("Bússola", "Compass"),
    "cooked_beef": ("Bife", "Steak"), "cooked_chicken": ("Frango assado", "Cooked chicken"), "cooked_cod": ("Bacalhau assado", "Cooked cod"),
    "cooked_mutton": ("Carneiro assado", "Cooked mutton"), "cooked_porkchop": ("Costeleta assada", "Cooked porkchop"),
    "cooked_rabbit": ("Coelho assado", "Cooked rabbit"), "cooked_salmon": ("Salmão assado", "Cooked salmon"), "cookie": ("Biscoito", "Cookie"),
    "copper_ore": ("Minério de cobre", "Copper ore"), "crying_obsidian": ("Obsidiana chorona", "Crying obsidian"),
    "dead_bush": ("Arbusto morto", "Dead bush"), "deepslate": ("Ardósia", "Deepslate"), "diamond_ore": ("Minério de diamante", "Diamond ore"),
    "dried_kelp": ("Alga seca", "Dried kelp"), "dried_kelp_block": ("Bloco de alga seca", "Dried kelp block"), "egg": ("Ovo", "Egg"),
    "emerald_ore": ("Minério de esmeralda", "Emerald ore"), "end_stone": ("Pedra do End", "End stone"), "fern": ("Samambaia", "Fern"),
    "gold_block": ("Bloco de ouro", "Block of gold"), "gold_nugget": ("Pepita de ouro", "Gold nugget"), "gold_ore": ("Minério de ouro", "Gold ore"),
    "golden_apple": ("Maçã dourada", "Golden apple"), "golden_carrot": ("Cenoura dourada", "Golden carrot"), "gravel": ("Cascalho", "Gravel"),
    "green_dye": ("Corante verde", "Green dye"), "honey_bottle": ("Garrafa de mel", "Honey bottle"), "iron_ore": ("Minério de ferro", "Iron ore"),
    "kelp": ("Alga", "Kelp"), "lantern": ("Lanterna", "Lantern"), "lapis_lazuli": ("Lápis-lazúli", "Lapis lazuli"),
    "lapis_ore": ("Minério de lápis-lazúli", "Lapis lazuli ore"), "large_fern": ("Samambaia grande", "Large fern"),
    "light_blue_dye": ("Corante azul-claro", "Light blue dye"), "lily_pad": ("Vitória-régia", "Lily pad"),
    "melon_seeds": ("Sementes de melancia", "Melon seeds"), "melon_slice": ("Fatia de melancia", "Melon slice"),
    "minecart": ("Carrinho", "Minecart"), "moss_carpet": ("Tapete de musgo", "Moss carpet"), "mushroom_stew": ("Ensopado de cogumelos", "Mushroom stew"),
    "mutton": ("Carneiro cru", "Raw mutton"), "nether_brick": ("Tijolo do Nether", "Nether brick"),
    "nether_gold_ore": ("Minério de ouro do Nether", "Nether gold ore"), "nether_quartz_ore": ("Minério de quartzo do Nether", "Nether quartz ore"),
    "netherite_scrap": ("Fragmento de netherita", "Netherite scrap"), "netherrack": ("Netherrack", "Netherrack"),
    "oak_boat": ("Barco de carvalho", "Oak boat"), "oak_door": ("Porta de carvalho", "Oak door"), "oak_stairs": ("Escada de carvalho", "Oak stairs"),
    "porkchop": ("Costeleta crua", "Raw porkchop"), "potato": ("Batata", "Potato"), "pumpkin": ("Abóbora", "Pumpkin"),
    "pumpkin_pie": ("Torta de abóbora", "Pumpkin pie"), "pumpkin_seeds": ("Sementes de abóbora", "Pumpkin seeds"),
    "quartz": ("Quartzo do Nether", "Nether quartz"), "rabbit": ("Coelho cru", "Raw rabbit"), "rail": ("Trilho", "Rail"),
    "raw_copper": ("Cobre bruto", "Raw copper"), "raw_gold": ("Ouro bruto", "Raw gold"), "raw_iron": ("Ferro bruto", "Raw iron"),
    "red_mushroom": ("Cogumelo vermelho", "Red mushroom"), "red_sand": ("Areia vermelha", "Red sand"),
    "redstone_lamp": ("Lâmpada de redstone", "Redstone lamp"), "redstone_ore": ("Minério de redstone", "Redstone ore"),
    "salmon": ("Salmão cru", "Raw salmon"), "sand": ("Areia", "Sand"), "seagrass": ("Grama marinha", "Seagrass"),
    "short_grass": ("Grama", "Short grass"), "soul_sand": ("Areia das almas", "Soul sand"), "soul_soil": ("Terra das almas", "Soul soil"),
    "sponge": ("Esponja", "Sponge"), "stone": ("Pedra", "Stone"), "sugar": ("Açúcar", "Sugar"), "sugar_cane": ("Cana-de-açúcar", "Sugar cane"),
    "sweet_berries": ("Frutas doces", "Sweet berries"), "tall_grass": ("Grama alta", "Tall grass"), "torch": ("Tocha", "Torch"),
    "vine": ("Trepadeira", "Vines"), "wet_sponge": ("Esponja molhada", "Wet sponge"), "wheat": ("Trigo", "Wheat"),
    "written_book": ("Livro escrito", "Written book"), "flower_pot": ("Vaso de flor", "Flower pot"), "powered_rail": ("Trilho elétrico", "Powered rail"),
    "chest_minecart": ("Carrinho com baú", "Minecart with chest"), "lava_bucket": ("Balde de lava", "Lava bucket"),
    "obsidian": ("Obsidiana", "Obsidian"), "glass_pane": ("Painel de vidro", "Glass pane"), "planks": ("Tábuas", "Planks"),
}
for wood, (pt, en) in WOODS.items():
    VANILLA[f"{wood}_log"] = (f"Tronco de {pt}", f"{en} log")
    VANILLA[f"stripped_{wood}_log"] = (f"Tronco de {pt} descascado", f"Stripped {en.lower()} log")
    VANILLA[f"{wood}_leaves"] = (f"Folhas de {pt}", f"{en} leaves")
    VANILLA[f"{wood}_sapling"] = (f"Muda de {pt}", f"{en} sapling")
for ore in ("coal", "copper", "diamond", "emerald", "gold", "iron", "lapis", "redstone"):
    pt, en = VANILLA.get(f"{ore}_ore", (ore, ore))
    VANILLA[f"deepslate_{ore}_ore"] = (pt + " de ardósia", "Deepslate " + en.lower())

# Descrições da wiki (as que o "?" das máquinas ainda não tem, e as que pedem mais contexto).
TEXT = {
    "copper_plate": ("Chapa de cobre: a peça básica da fábrica. Sai da prensa (cobre bruto ou lingote). Vira engrenagem no montador e fiação na cortadora.",
                     "Copper plate: the factory's basic part. Made by the press (raw copper or ingot). Becomes gears in the assembler and wire in the cutter."),
    "iron_plate": ("Chapa de ferro: prensa com ferro bruto. Vira moldura na cortadora e aço no forno de ligas.",
                   "Iron plate: press with raw iron. Becomes frames in the cutter and steel in the alloy furnace."),
    "steel_plate": ("Chapa de aço: 2 chapas de ferro + carvão no forno de ligas. 16 delas abrem a fase do aço.",
                    "Steel plate: 2 iron plates + coal in the alloy furnace. 16 of them open the steel phase."),
    "gear": ("Engrenagem: 2 chapas de cobre no montador. 32 abrem a oficina; entra em módulos, centrais e receitas.",
             "Gear: 2 copper plates in the assembler. 32 open the workshop; used in modules, hubs and recipes."),
    "wire": ("Fiação de cobre: chapa de cobre na cortadora. Base dos circuitos e dos motores.",
             "Copper wire: copper plate in the cutter. Base of circuits and motors."),
    "frame": ("Moldura: chapa de ferro na cortadora. Com engrenagem, vira módulo no integrador.",
              "Frame: iron plate in the cutter. With a gear it becomes a module in the integrator."),
    "silicon": ("Pastilha de silício: quartzo no triturador, ou 4 areias + carvão no forno de ligas (sem ir ao Nether).",
                "Silicon wafer: quartz in the crusher, or 4 sand + coal in the alloy furnace (no Nether trip)."),
    "circuit": ("Placa de circuito: 2 fiações + silício no montador de circuitos. 16 abrem a eletrônica.",
                "Circuit board: 2 wires + silicon in the circuit assembler. 16 open electronics."),
    "module": ("Módulo mecânico: moldura + engrenagem no integrador. Usado em melhorias e pedidos.",
               "Mechanical module: frame + gear in the integrator. Used in upgrades and orders."),
    "speed_module": ("Módulo de velocidade: ciclo em 60% do tempo, 180% de energia por tick e 150% de poluição. Botão direito na máquina.",
                     "Speed module: 60% cycle time, 180% power per tick, 150% pollution. Right-click a machine."),
    "efficiency_module": ("Módulo de eficiência: 60% da energia e metade da poluição, na mesma velocidade.",
                          "Efficiency module: 60% power and half the pollution, same speed."),
    "productivity_module": ("Módulo de produtividade: um lote extra a cada cinco ciclos (+20%), com ciclo um pouco mais lento.",
                            "Productivity module: one extra batch every five cycles (+20%), with a slightly slower cycle."),
    "motor": ("Motor: aço + 4 fiações no integrador. Peça da automação avançada.",
              "Motor: steel + 4 wires in the integrator. Part of advanced automation."),
    "processor": ("Processador: 2 circuitos + 2 silícios no montador de circuitos.",
                  "Processor: 2 circuits + 2 silicon in the circuit assembler."),
    "automation_core": ("Núcleo de automação: motores, processadores e aço na fabricadora. Pedido pela pesquisa de escala.",
                        "Automation core: motors, processors and steel in the fabricator. Required by the scale research."),
    "mk2_kit": ("Kit Mk.II: transforma máquinas em versões mais rápidas.", "Mk.II kit: turns machines into faster versions."),
    "permit_workshop": ("Licença da oficina: entregue pelo Quadro da fábrica ao fechar a fase das engrenagens.",
                        "Workshop permit: given by the Factory board when the gear phase closes."),
    "permit_steel": ("Licença do aço: fase das 16 chapas de aço.", "Steel permit: the 16 steel plate phase."),
    "permit_electronics": ("Licença da eletrônica: fase dos 16 circuitos.", "Electronics permit: the 16 circuit phase."),
    "route_card": ("Cartão de rota: liga terminais, portos e caravanas pela mesma cor.",
                   "Route card: links terminals, ports and caravans by colour."),
    "clipboard": ("Prancheta: agachado copia filtro e prioridade de um braço ou divisor; em pé, cola em outro.",
                  "Clipboard: sneak to copy an arm's or splitter's filter and priority; stand to paste."),
    "flour": ("Farinha: trigo no moinho. Vira pão, biscoito e bolo na cozinha.", "Flour: wheat in the mill. Becomes bread, cookies and cake in the kitchen."),
    "biomass": ("Biomassa: plantas, folhas e troncos no moinho. Combustível do gerador a biomassa.",
                "Biomass: plants, leaves and logs in the mill. Fuel for the biomass generator."),
    "repair_kit": ("Kit de reparo: botão direito numa máquina danificada por Saqueadores recupera 50% da integridade.",
                   "Repair kit: right-click a machine damaged by Raiders to restore 50% integrity."),
}


def names(langs, vanilla):
    """Nome de cada item que a wiki mostra, em pt e en."""
    out = {}
    for code in ("pt_br", "en_us"):
        for key, value in langs[code].items():
            match = re.fullmatch(r"(?:block|item)\.veio\.([a-z0-9_]+)", key)
            if match:
                out.setdefault("veio:" + match[1], {})[code] = value
    for name, pair in {**VANILLA, **vanilla}.items():
        out["minecraft:" + name] = {"pt_br": pair[0], "en_us": pair[1]}
    return out


def load():
    recipes = {}
    for path in sorted((DATA / "recipe").glob("*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        if data.get("type") != "minecraft:crafting_shapeless":
            continue
        recipes.setdefault(data["result"]["id"], []).append((data["ingredients"], data["result"].get("count", 1), path.stem))
    tabs = []
    for name in ("industry", "power", "logistics", "world"):
        data = json.loads((DATA / f"horizon/creative_tab/{name}.json").read_text(encoding="utf-8"))
        tabs.append((name, data["title"], data["icon"], data["items"]))
    unlocks = {}
    for path in sorted((DATA / "advancement").rglob("*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        title = data.get("display", {}).get("title", {}).get("translate")
        for recipe in data.get("rewards", {}).get("recipes", []):
            if title:
                unlocks.setdefault(recipe, title)
    production = json.loads((ROOT / "tools/production.json").read_text(encoding="utf-8"))
    return recipes, tabs, unlocks, production


def rust(text):
    return json.dumps(text, ensure_ascii=False)


def build(label, langs, vanilla):
    from wiki_articles import ARTICLES
    recipes, tabs, unlocks, production = load()
    known = names(langs, vanilla)
    used = set()
    entries = []
    for index, (_, _, _, items) in enumerate(tabs):
        for item in items:
            craft = recipes.get(item, [([], 0, "")])[0]
            name = item.split(":")[1]
            if item.startswith("veio:"):
                text = f"veio.wiki.text.{name}" if name in TEXT else f"veio.help.{name}"
            else:
                text = ""
            unlock = unlocks.get(craft[2] and "veio:" + craft[2], "") if craft[2] else ""
            entries.append((item, index, craft[0], craft[1], text, unlock))
            used.add(item)
            used.update(craft[0])
    for recipe in production["recipes"]:
        used.add("veio:" + recipe["machine"])
        used.add(recipe["out"][0])
    # Os itens das receitas de máquina vêm de data.rs (grupos já expandidos).
    data = (ROOT / "native/api/src/production/data.rs").read_text(encoding="utf-8")
    used.update(re.findall(r'"((?:minecraft|veio):[a-z0-9_]+)"', data.split("pub(crate) static NODES")[0]))
    for item, pair in sorted(known.items()):
        if item in used and "pt_br" in pair:
            label(f"veio.wiki.name.{item.split(':')[1]}", pair["pt_br"], pair.get("en_us", pair["pt_br"]))
    for name, (pt, en) in TEXT.items():
        label(f"veio.wiki.text.{name}", pt, en)
    label("veio.wiki.title", "Wiki do Veio", "Veio Wiki")
    label("veio.wiki.hint", "J ou Esc fecha · roda do mouse rola · clique num item para abrir", "J or Esc closes · scroll to move · click an item to open")
    label("veio.wiki.tab.guide", "Sistemas", "Systems")
    label("veio.wiki.craft", "Fabricação (bancada)", "Crafting (table)")
    label("veio.wiki.made_in", "Produzido em", "Made in")
    label("veio.wiki.processes", "Processa", "Processes")
    label("veio.wiki.used_in", "Usado em", "Used in")
    label("veio.wiki.unlock", "Liberado por: %s", "Unlocked by: %s")
    label("veio.wiki.per", "%s s · %s FE/t", "%s s · %s FE/t")
    label("veio.wiki.vanilla", "Item do Minecraft", "Minecraft item")
    label("veio.wiki.open", "Wiki do Veio: aperte J para abrir (receitas, máquinas e sistemas).",
          "Veio Wiki: press J to open (recipes, machines and systems).")
    for article in ARTICLES:
        label(f"veio.wiki.article.{article['id']}.title", *article["title"])
        label(f"veio.wiki.article.{article['id']}.body", "\n".join(article["game"][0]), "\n".join(article["game"][1]))
    lines = ["// @generated por tools/build_wiki.py (chamado por build_tutorial.py); não edite.",
             "use super::{Article, Entry};", "",
             "pub(crate) static TABS: &[(&str, &str)] = &["]
    for _, title, icon, _ in tabs:
        lines.append(f"    ({rust(title)}, {rust(icon)}),")
    lines += ["    (\"veio.wiki.tab.guide\", \"minecraft:book\"),", "];", "",
              "pub(crate) static ENTRIES: &[Entry] = &["]
    for item, tab, craft, makes, text, unlock in entries:
        parts = ", ".join(rust(part) for part in craft)
        lines.append(f"    Entry {{ id: {rust(item)}, tab: {tab}, craft: &[{parts}], makes: {makes}, text: {rust(text)}, unlock: {rust(unlock)} }},")
    lines += ["];", "", "pub(crate) static ARTICLES: &[Article] = &["]
    for article in ARTICLES:
        lines.append(f"    Article {{ id: {rust(article['id'])}, icon: {rust(article['icon'])} }},")
    lines += ["];", ""]
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines), encoding="utf-8")
    try:
        from wiki_site import build as site
        site(langs, known, recipes, tabs, unlocks, production, entries)
    except ImportError:
        pass
    return entries
