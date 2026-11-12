"""Os prédios da vila, estágio por estágio. Cada função devolve um `Model` com a porta para o sul
(a vila gira o modelo para a porta dar para a rua). Paredes de 3 blocos (y 1..3), piso em y 0.

Estágio 1 (a chegada, a vila em declínio): casas habitadas mas simples, algumas com sinais de
abandono (musgo, vidro faltando), a praça gasta, a fonte seca, o coreto sem telhado.
"""
from modelo import Model

M = "minecraft:"


def mc(name, **p):
    if not p:
        return M + name
    return M + name + "[" + ",".join(f"{k}={v}" for k, v in p.items()) + "]"


def pot(flower):
    return mc(f"potted_{flower}")


def lantern(hanging=False):
    return mc("lantern", hanging=str(hanging).lower())


def planter(m, x, z, side, flowers=("red_tulip", "dandelion"), wood="spruce"):
    """Jardineira sob a janela, do lado de fora: alçapão de pé e flores."""
    m.set(x, 1, z, mc(f"{wood}_trapdoor", facing=side, half="top", open="false"))
    m.set(x, 2, z, pot(flowers[(x + z) % len(flowers)]))


# =====================================================================================
# Casas habitadas (7 modelos, nenhum igual)
# =====================================================================================

def casa_pedra_tabua():
    """Casa da família Souza: embasamento de pedregulho, tábuas de carvalho, telhado de abeto."""
    m = Model("casa_pedra_tabua", 10, 8, "Casa de pedra e tábua")
    m.floor(0, 0, 9, 7, mc("spruce_planks"))
    m.walls(0, 0, 9, 7, 1, 1, mc("cobblestone"))
    m.walls(0, 0, 9, 7, 2, 3, mc("oak_planks"), corner=mc("oak_log", axis="y"))
    m.fill(0, 1, 0, 0, 1, 0, mc("oak_log", axis="y")); m.fill(9, 1, 0, 9, 1, 0, mc("oak_log", axis="y"))
    m.fill(0, 1, 7, 0, 1, 7, mc("oak_log", axis="y")); m.fill(9, 1, 7, 9, 1, 7, mc("oak_log", axis="y"))
    m.carve(1, 1, 1, 8, 3, 6)
    m.door(4, 7, "south")
    for x in (2, 7):
        m.window(x, 2, 7); planter(m, x, 8, "south")
    for x in (2, 4, 7):
        m.window(x, 2, 0)
    m.window(0, 2, 3); m.window(9, 2, 4)
    m.gable(0, 0, 9, 7, 4, "spruce", mc("spruce_planks"), ridge="x")
    # dentro: cama de casal, mesa com duas cadeiras, fogão, baú, estante, tapete, lanterna
    m.bed(1, 2, "north", "red"); m.bed(2, 2, "north", "red")
    m.set(1, 1, 4, mc("chest", facing="east")); m.set(1, 1, 5, mc("crafting_table"))
    m.set(8, 1, 1, mc("furnace", facing="west")); m.set(8, 1, 2, mc("barrel", facing="up"))
    m.set(8, 2, 1, mc("candle", candles="2", lit="false"))
    m.table(6, 4); m.chair(6, 5, "north"); m.chair(5, 4, "east")
    m.set(4, 1, 1, mc("bookshelf")); m.set(5, 1, 1, mc("bookshelf"))
    m.set(4, 2, 1, pot("fern"))
    m.fill(3, 1, 3, 4, 1, 5, mc("red_carpet")); m.clear(4, 1, 5); m.set(3, 1, 5, mc("red_carpet"))
    m.set(5, 3, 3, lantern(True))
    m.set(8, 1, 6, mc("flower_pot"))
    m.note(2, 1, 2, "cama de casal"); m.note(6, 1, 4, "mesa"); m.note(8, 1, 1, "fogão")
    return m


def casa_enxaimel():
    """Casa enxaimel: estrutura de madeira escura aparente e painéis claros; telhado no outro sentido."""
    m = Model("casa_enxaimel", 10, 8, "Casa enxaimel")
    post, panel = mc("stripped_dark_oak_log", axis="y"), mc("calcite")
    m.floor(0, 0, 9, 7, mc("dark_oak_planks"))
    m.walls(0, 0, 9, 7, 1, 3, panel, corner=post)
    for x in (3, 6):
        m.fill(x, 1, 0, x, 3, 0, post); m.fill(x, 1, 7, x, 3, 7, post)
    m.fill(0, 1, 4, 0, 3, 4, post); m.fill(9, 1, 4, 9, 3, 4, post)
    m.walls(0, 0, 9, 7, 1, 1, mc("stone_bricks"))
    m.beams(0, 0, 9, 7, 4, mc("stripped_dark_oak_log", axis="x"), mc("stripped_dark_oak_log", axis="z"))
    m.carve(1, 1, 1, 8, 3, 6)
    m.door(5, 7, "south", wood="dark_oak")
    for x in (2, 7):
        m.window(x, 2, 7); m.shutters(x, 2, 7, "south", "dark_oak")
    m.window(2, 2, 0); m.window(7, 2, 0); m.window(0, 2, 2); m.window(9, 2, 2); m.window(9, 2, 6)
    m.gable(0, 0, 9, 7, 5, "dark_oak", panel, ridge="z")
    m.fill(1, 4, 1, 8, 4, 6, mc("air"))
    # dentro: duas camas de criança, tear, mesa, baú, caldeirão de água, quadro de flores
    m.bed(1, 1, "south", "light_blue"); m.bed(3, 1, "south", "yellow")
    m.set(2, 1, 1, mc("chest", facing="south"))
    m.set(8, 1, 1, mc("loom", facing="west")); m.set(8, 1, 2, mc("barrel", facing="up"))
    m.set(8, 1, 6, mc("water_cauldron", level="3")); m.set(7, 1, 6, mc("smoker", facing="north"))
    m.table(5, 4, wood="dark_oak", cover=mc("yellow_carpet")); m.chair(5, 5, "north", wood="dark_oak"); m.chair(4, 4, "east", wood="dark_oak")
    m.fill(1, 1, 4, 2, 1, 6, mc("light_gray_carpet"))
    m.set(4, 3, 3, lantern(True))
    m.set(1, 2, 6, pot("blue_orchid"))
    m.note(2, 1, 1, "quarto das crianças"); m.note(8, 1, 1, "tear")
    return m


def casa_taipa():
    """Casa de taipa: tijolo de barro, pequena e fresca, telhado de barro."""
    m = Model("casa_taipa", 8, 7, "Casa de taipa")
    m.floor(0, 0, 7, 6, mc("packed_mud"))
    m.walls(0, 0, 7, 6, 1, 3, mc("mud_bricks"), corner=mc("stripped_oak_log", axis="y"))
    m.carve(1, 1, 1, 6, 3, 5)
    m.door(3, 6, "south", wood="oak")
    m.window(1, 2, 6); m.window(5, 2, 6); m.window(3, 2, 0); m.window(0, 2, 3); m.window(7, 2, 3)
    m.gable(0, 0, 7, 6, 4, "mud_brick", mc("mud_bricks"), ridge="x")
    m.set(5, 1, 7, mc("decorated_pot", facing="south")); m.set(1, 1, 7, pot("cactus"))
    # dentro: cama, defumador, barril, pote, rede (tapete) e vasos
    m.bed(1, 1, "south", "orange")
    m.set(6, 1, 1, mc("smoker", facing="south")); m.set(5, 1, 1, mc("barrel", facing="up"))
    m.set(6, 1, 5, mc("decorated_pot", facing="west")); m.set(6, 2, 5, mc("candle", candles="1", lit="false"))
    m.table(4, 3, wood="oak", cover=mc("orange_carpet")); m.chair(4, 4, "north", wood="oak")
    m.fill(1, 1, 4, 2, 1, 5, mc("orange_carpet"))
    m.set(3, 3, 3, lantern(True))
    m.note(1, 1, 1, "cama"); m.note(6, 1, 1, "defumador")
    return m


def sobrado():
    """Sobrado estreito: térreo de pedra (cozinha), andar de cima de abeto (quartos)."""
    m = Model("sobrado", 7, 9, "Sobrado")
    m.floor(0, 0, 6, 8, mc("stone_bricks"))
    m.walls(0, 0, 6, 8, 1, 3, mc("cobblestone"), corner=mc("spruce_log", axis="y"))
    m.fill(0, 4, 0, 6, 4, 8, mc("spruce_planks"))
    m.walls(0, 0, 6, 8, 5, 7, mc("spruce_planks"), corner=mc("spruce_log", axis="y"))
    m.carve(1, 1, 1, 5, 3, 7); m.carve(1, 5, 1, 5, 7, 7)
    for y in (1, 2, 3, 4):                                    # escada de mão até o andar de cima
        m.set(1, y, 1, mc("ladder", facing="east"))
    m.door(3, 8, "south", wood="spruce")
    m.window(5, 2, 8); m.window(1, 6, 8); m.window(5, 6, 8); m.window(3, 6, 0); m.window(0, 6, 4); m.window(6, 2, 4)
    m.shutters(1, 6, 8, "south"); m.shutters(5, 6, 8, "south")
    planter(m, 5, 9, "south", ("red_tulip", "oxeye_daisy"))
    m.gable(0, 0, 6, 8, 8, "spruce", mc("spruce_planks"), ridge="z")
    # térreo: cozinha
    m.set(5, 1, 1, mc("smoker", facing="south")); m.set(4, 1, 1, mc("barrel", facing="up")); m.set(3, 1, 1, mc("crafting_table"))
    m.table(4, 5, wood="spruce", cover=mc("white_carpet")); m.chair(4, 6, "north"); m.chair(5, 5, "west")
    m.set(3, 3, 4, lantern(True))
    # em cima: duas camas, baú, estante, vaso
    m.bed(5, 5, "north", "green", y=5); m.bed(3, 7, "east", "lime", y=5)
    m.set(5, 5, 7, mc("chest", facing="north")); m.set(2, 5, 7, mc("bookshelf")); m.set(2, 6, 7, pot("allium"))
    m.set(3, 7, 4, lantern(True))
    m.note(4, 1, 5, "cozinha no térreo"); m.note(5, 5, 4, "quartos em cima")
    return m


def casa_varanda():
    """Casa com varanda: bétula clara, alpendre de frente para a rua com banco e flores."""
    m = Model("casa_varanda", 10, 8, "Casa com varanda")
    m.floor(0, 0, 9, 5, mc("birch_planks"))
    m.walls(0, 0, 9, 5, 1, 3, mc("birch_planks"), corner=mc("oak_log", axis="y"))
    m.walls(0, 0, 9, 5, 1, 1, mc("stone_bricks"))
    m.carve(1, 1, 1, 8, 3, 4)
    m.door(4, 5, "south", wood="birch")
    m.window(2, 2, 5); m.window(7, 2, 5); m.window(2, 2, 0); m.window(7, 2, 0); m.window(0, 2, 2); m.window(9, 2, 3)
    # varanda: piso, guarda-corpo, colunas, cobertura de lajes, banco, flores
    m.fill(0, 0, 6, 9, 0, 7, mc("oak_planks"))
    for x in (0, 9):
        m.fill(x, 1, 7, x, 3, 7, mc("oak_fence"))
    for x in range(1, 9):
        if x not in (4,):
            m.set(x, 1, 7, mc("oak_fence"))
    m.fill(0, 4, 6, 9, 4, 7, mc("oak_slab", type="bottom"))
    m.set(1, 1, 6, mc("oak_stairs", facing="north", half="bottom", shape="straight"))
    m.set(2, 1, 6, mc("oak_stairs", facing="north", half="bottom", shape="straight"))
    m.set(8, 1, 6, pot("poppy")); m.set(7, 1, 6, mc("composter"))
    m.set(4, 3, 6, lantern(True))
    m.gable(0, 0, 9, 5, 4, "oak", mc("birch_planks"), ridge="x")
    # dentro: cama, estante com atril (leitor), mesa, baú, tapete
    m.bed(8, 2, "north", "white")
    m.set(1, 1, 1, mc("bookshelf")); m.set(2, 1, 1, mc("bookshelf")); m.set(3, 1, 1, mc("lectern", facing="south"))
    m.table(5, 2, wood="birch", cover=mc("white_carpet")); m.chair(5, 3, "north", wood="birch")
    m.set(1, 1, 4, mc("chest", facing="east")); m.set(8, 1, 4, mc("barrel", facing="up"))
    m.fill(2, 1, 3, 4, 1, 4, mc("light_blue_carpet"))
    m.set(5, 3, 2, lantern(True))
    m.note(4, 1, 6, "varanda"); m.note(3, 1, 1, "atril")
    return m


def casa_jardim():
    """Casa do jardim: abeto e andesito, telhado no outro sentido, horta e composteira na frente."""
    m = Model("casa_jardim", 9, 8, "Casa do jardim")
    m.floor(0, 0, 8, 5, mc("spruce_planks"))
    m.walls(0, 0, 8, 5, 1, 3, mc("spruce_planks"), corner=mc("spruce_log", axis="y"))
    m.walls(0, 0, 8, 5, 1, 1, mc("polished_andesite"))
    m.carve(1, 1, 1, 7, 3, 4)
    m.door(2, 5, "south", wood="spruce")
    m.window(5, 2, 5); m.window(6, 2, 5); m.window(4, 2, 0); m.window(0, 2, 2); m.window(8, 2, 2)
    m.gable(0, 0, 8, 5, 4, "spruce", mc("spruce_planks"), ridge="z")
    # jardim na frente: canteiro de flores e horta cercados
    m.fill(3, 0, 6, 8, 0, 7, mc("farmland", moisture="7"))
    for x in range(3, 9):
        m.set(x, 1, 6, mc("carrots", age="5") if x % 2 else mc("potatoes", age="4"))
        m.set(x, 1, 7, mc("oak_fence"))
    m.set(0, 1, 7, mc("composter", level="0")); m.set(0, 1, 6, mc("rose_bush", half="lower")); m.set(0, 2, 6, mc("rose_bush", half="upper"))
    m.set(1, 1, 7, mc("oxeye_daisy"))
    # dentro: duas camas, baú, bancada, mesa redonda, vasos
    m.bed(6, 1, "south", "brown"); m.bed(7, 1, "south", "brown")
    m.set(1, 1, 1, mc("crafting_table")); m.set(2, 1, 1, mc("chest", facing="south"))
    m.table(4, 3, wood="spruce", cover=mc("green_carpet")); m.chair(4, 4, "north")
    m.set(1, 1, 4, pot("azure_bluet")); m.set(4, 3, 2, lantern(True))
    m.note(5, 1, 6, "horta"); m.note(6, 1, 1, "quarto")
    return m


def casa_pedra(name="casa_pedra", title="Casa de pedra", roof="dark_oak"):
    """Casa de pedra: tijolo de pedra com musgo, a casa mais antiga da vila, telhado escuro."""
    m = Model(name, 10, 8, title)
    m.floor(0, 0, 9, 7, mc("oak_planks"))
    m.walls(0, 0, 9, 7, 1, 3, mc("stone_bricks"), corner=mc("chiseled_stone_bricks"))
    for x, y, z in [(1, 1, 0), (5, 1, 0), (9, 2, 3), (0, 1, 5), (3, 3, 7), (8, 1, 7)]:
        m.set(x, y, z, mc("mossy_stone_bricks"))
    m.set(6, 2, 0, mc("cracked_stone_bricks")); m.set(0, 3, 2, mc("cracked_stone_bricks"))
    m.carve(1, 1, 1, 8, 3, 6)
    m.door(4, 7, "south", wood="dark_oak")
    m.window(2, 2, 7); m.window(7, 2, 7); m.window(2, 2, 0); m.window(7, 2, 0); m.window(0, 2, 4); m.window(9, 2, 4)
    m.gable(0, 0, 9, 7, 4, roof, mc("stone_bricks"), ridge="x")
    m.set(1, 1, 8, mc("moss_carpet")); m.set(8, 1, 8, mc("moss_carpet"))
    # dentro: cama, fornalha com chaminé, barril, estante, poltrona, tapete
    m.bed(8, 5, "north", "blue")
    m.fill(1, 1, 1, 3, 3, 1, mc("bricks")); m.set(2, 1, 1, mc("furnace", facing="south")); m.set(2, 1, 2, mc("red_carpet"))
    m.set(4, 1, 1, mc("bookshelf")); m.set(5, 1, 1, mc("bookshelf")); m.set(8, 1, 1, mc("barrel", facing="up"))
    m.chair(3, 4, "west", wood="dark_oak"); m.table(2, 4, wood="dark_oak", cover=mc("brown_carpet"))
    m.set(1, 1, 6, mc("chest", facing="east"))
    m.set(5, 3, 4, lantern(True))
    m.note(2, 1, 1, "lareira"); m.note(8, 1, 5, "cama")
    return m


HOUSES = [casa_pedra_tabua, casa_enxaimel, casa_taipa, sobrado, casa_varanda, casa_jardim, casa_pedra]


# =====================================================================================
# Casas em obra, abandonada e lotes
# =====================================================================================

def em_obra(base):
    """A casa `base` pela metade: embasamento e meia parede, andaime em volta, material empilhado."""
    full = base()
    m = Model(full.name + "_obra", full.width, full.depth, full.title + " (em obra)")
    for (x, y, z), s in full.blocks.items():
        if y == 0:
            m.set(x, y, z, s)
        elif y <= 2 and ("cobble" in s or "stone" in s or "brick" in s or "log" in s or "mud" in s or "planks" in s or "calcite" in s):
            if (x + z) % 4 or y == 1:
                m.set(x, y, z, s)
    xs = [p[0] for p in full.blocks if p[1] == 0]; zs = [p[2] for p in full.blocks if p[1] == 0]
    x0, x1, z0, z1 = min(xs), max(xs), min(zs), max(zs)
    for x in range(x0 - 1, x1 + 2):
        for z in (z0 - 1, z1 + 1):
            if (x - x0) % 3 == 0:
                m.fill(x, 1, z, x, 4, z, mc("scaffolding", bottom="false", distance="0"))
    for z in range(z0, z1 + 1, 3):
        for x in (x0 - 1, x1 + 1):
            m.fill(x, 1, z, x, 4, z, mc("scaffolding", bottom="false", distance="0"))
    m.fill(x1 + 2, 1, z0, x1 + 3, 1, z0 + 1, mc("oak_planks")); m.set(x1 + 2, 2, z0, mc("oak_slab", type="bottom"))
    m.fill(x1 + 2, 1, z0 + 3, x1 + 3, 1, z0 + 3, mc("cobblestone")); m.set(x1 + 2, 1, z0 + 4, mc("crafting_table"))
    m.entrance = (full.entrance[0], full.entrance[1], "south") if full.entrance else None
    m.note(x0, 1, z0, "em obra")
    return m


def ruina():
    """A casa de pedra abandonada: telhado caído, parede rachada, trepadeira, teia, entulho."""
    full = casa_pedra()
    m = Model("ruina", full.width, full.depth, "Casa abandonada")
    import random
    rnd = random.Random(5)
    for (x, y, z), s in full.blocks.items():
        if "stairs" in s or "slab" in s:
            if rnd.random() < 0.45 and not (3 <= x <= 6):
                m.set(x, y, z, s)
            continue
        if any(w in s for w in ("bed", "bookshelf", "furnace", "barrel", "chest", "carpet", "lantern", "door", "glass")):
            continue
        if y >= 3 and rnd.random() < 0.4:
            continue
        if "stone_bricks" in s and rnd.random() < 0.35:
            s = mc("mossy_stone_bricks") if rnd.random() < 0.6 else mc("cracked_stone_bricks")
        m.set(x, y, z, s)
    for x, z in [(2, 2), (6, 4), (4, 5), (7, 2)]:
        m.set(x, 1, z, mc("cobblestone") if (x + z) % 2 else mc("mossy_cobblestone"))
    m.set(5, 1, 3, mc("dark_oak_slab", type="bottom")); m.set(3, 1, 4, mc("cobweb")); m.set(8, 2, 1, mc("cobweb"))
    for z in range(1, 7):
        if rnd.random() < 0.6:
            m.set(-1, 2, z, mc("vine", east="true")); m.set(-1, 1, z, mc("vine", east="true"))
    m.set(4, 1, 6, mc("short_grass")); m.set(6, 1, 1, mc("short_grass")); m.set(2, 1, 5, mc("fern"))
    m.entrance = full.entrance
    m.note(4, 1, 3, "telhado caído")
    return m


def lote():
    """Lote vazio: marcas de fundação no capim alto e uma estaca com placa."""
    m = Model("lote", 10, 8, "Lote vazio")
    for x in range(10):
        for z in (0, 7):
            if x % 2 == 0:
                m.set(x, 0, z, mc("gravel"))
    for z in range(8):
        for x in (0, 9):
            if z % 2 == 0:
                m.set(x, 0, z, mc("gravel"))
    for x, z in [(2, 2), (5, 3), (7, 5), (3, 5), (6, 1), (8, 3), (1, 4), (4, 6)]:
        m.set(x, 1, z, mc("tall_grass", half="lower")); m.set(x, 2, z, mc("tall_grass", half="upper"))
    for x, z in [(3, 2), (4, 4), (6, 5), (2, 6), (8, 1), (5, 1)]:
        m.set(x, 1, z, mc("short_grass"))
    m.set(0, 1, 7, mc("oak_fence")); m.set(0, 2, 7, mc("oak_fence"))
    m.entrance = (5, 7, "south")
    return m


# =====================================================================================
# Moradores
# =====================================================================================

def ferreiro():
    """A oficina do Seu Bento: forja de pedra aberta para a rua, bigorna, mesa de ferraria, e o
    quarto nos fundos. Pouco ferro: a mina está fechada."""
    m = Model("ferreiro", 12, 10, "Ferreiro · Seu Bento")
    m.floor(0, 0, 11, 9, mc("stone_bricks"))
    m.walls(0, 0, 11, 9, 1, 3, mc("cobblestone"), corner=mc("dark_oak_log", axis="y"))
    m.carve(1, 1, 1, 10, 3, 8)
    # a oficina (sul) é aberta: colunas e meia parede
    m.carve(1, 1, 9, 10, 3, 9)
    for x in (0, 4, 7, 11):
        m.fill(x, 1, 9, x, 3, 9, mc("dark_oak_log", axis="y"))
    m.fill(1, 1, 9, 3, 1, 9, mc("cobblestone_wall")); m.fill(8, 1, 9, 10, 1, 9, mc("cobblestone_wall"))
    m.entrance = (5, 9, "south")
    # divisória do quarto (norte)
    m.fill(1, 1, 4, 10, 3, 4, mc("spruce_planks")); m.door(3, 4, "south", wood="spruce", main=False)
    m.window(2, 2, 0); m.window(8, 2, 0); m.window(0, 2, 2); m.window(11, 2, 2)
    m.gable(0, 0, 11, 9, 4, "deepslate_tile", mc("cobbled_deepslate"), ridge="x")
    # chaminé da forja
    m.fill(9, 1, 6, 10, 9, 7, mc("bricks")); m.carve(9, 1, 6, 9, 2, 6); m.set(9, 1, 6, mc("campfire", lit="false"))
    # oficina
    m.set(6, 1, 6, mc("anvil", facing="east")); m.set(4, 1, 6, mc("smithing_table")); m.set(2, 1, 6, mc("grindstone", face="floor", facing="north"))
    m.set(10, 1, 8, mc("blast_furnace", facing="west")); m.set(1, 1, 8, mc("barrel", facing="up")); m.set(1, 1, 7, mc("chest", facing="east"))
    m.set(7, 1, 8, mc("water_cauldron", level="3"))
    m.set(5, 3, 6, lantern(True)); m.set(2, 3, 8, lantern(True))
    m.set(8, 1, 6, mc("iron_ore")); m.set(8, 2, 6, mc("raw_iron_block"))
    # quarto
    m.bed(9, 1, "south", "gray"); m.set(10, 1, 3, mc("chest", facing="west")); m.set(1, 1, 1, mc("crafting_table"))
    m.table(5, 2, wood="spruce", cover=mc("gray_carpet")); m.chair(6, 2, "west")
    m.set(5, 3, 2, lantern(True))
    m.note(6, 1, 6, "bigorna"); m.note(4, 1, 6, "mesa de ferraria"); m.note(9, 1, 6, "forja")
    return m


def lavradora():
    """A casa da Dona Cida: carvalho e pedregulho, cozinha com pão, despensa, composteira na porta."""
    m = Model("lavradora", 11, 9, "Lavradora · Dona Cida")
    m.floor(0, 0, 10, 8, mc("oak_planks"))
    m.walls(0, 0, 10, 8, 1, 1, mc("cobblestone"))
    m.walls(0, 0, 10, 8, 2, 3, mc("oak_planks"), corner=mc("oak_log", axis="y"))
    for x, z in ((0, 0), (10, 0), (0, 8), (10, 8)):
        m.set(x, 1, z, mc("oak_log", axis="y"))
    m.carve(1, 1, 1, 9, 3, 7)
    m.door(5, 8, "south")
    for x in (2, 8):
        m.window(x, 2, 8); planter(m, x, 9, "south", ("dandelion", "poppy"), "oak")
    m.window(3, 2, 0); m.window(7, 2, 0); m.window(0, 2, 4); m.window(10, 2, 4)
    m.gable(0, 0, 10, 8, 4, "oak", mc("oak_planks"), ridge="x")
    m.set(4, 1, 9, mc("composter")); m.set(6, 1, 9, mc("hay_block", axis="y"))
    # dentro: cama, mesa com pão, despensa (barris), fogão, estante de sementes
    m.bed(1, 1, "south", "yellow")
    m.set(9, 1, 1, mc("smoker", facing="west")); m.set(9, 1, 2, mc("barrel", facing="up")); m.set(9, 1, 3, mc("barrel", facing="up"))
    m.set(9, 2, 2, mc("hay_block", axis="y"))
    m.table(5, 4, wood="oak", cover=mc("yellow_carpet")); m.chair(5, 5, "north", wood="oak"); m.chair(4, 4, "east", wood="oak")
    m.set(6, 1, 1, mc("crafting_table")); m.set(5, 1, 1, mc("chest", facing="south"))
    m.set(1, 1, 6, pot("dandelion"))
    m.set(5, 3, 4, lantern(True))
    m.note(5, 1, 4, "mesa com pão"); m.note(9, 1, 2, "despensa")
    return m


def vendinha():
    """A vendinha do Seu Zé: balcão, prateleiras quase vazias, caixotes, toldo listrado, quarto atrás."""
    m = Model("vendinha", 10, 8, "Vendinha · Seu Zé")
    m.floor(0, 0, 9, 7, mc("spruce_planks"))
    m.walls(0, 0, 9, 7, 1, 3, mc("oak_planks"), corner=mc("spruce_log", axis="y"))
    m.walls(0, 0, 9, 7, 1, 1, mc("cobblestone"))
    m.carve(1, 1, 1, 8, 3, 6)
    m.door(2, 7, "south", wood="spruce")
    m.carve(4, 2, 7, 7, 2, 7)                                       # vitrine grande
    m.window(4, 2, 7); m.window(5, 2, 7); m.window(6, 2, 7); m.window(7, 2, 7)
    for x in range(-1, 11):
        m.set(x, 3, 8, mc("red_wool") if x % 2 else mc("white_wool"))
    m.window(0, 2, 3); m.window(9, 2, 3)
    m.gable(0, 0, 9, 7, 4, "spruce", mc("spruce_planks"), ridge="x")
    # dentro: balcão (lajes sobre barris), prateleiras (estantes com alçapões), caixotes vazios
    for x in range(3, 8):
        m.set(x, 1, 4, mc("spruce_slab", type="top") if x != 5 else mc("barrel", facing="up"))
    m.set(6, 2, 4, mc("candle", candles="1", lit="false"))
    for x in range(1, 7):
        m.set(x, 1, 1, mc("barrel", facing="up") if x % 3 else mc("chest", facing="south"))
        m.set(x, 2, 1, mc("spruce_trapdoor", facing="south", half="top", open="false"))
    m.set(2, 3, 1, mc("hay_block", axis="y")); m.set(5, 3, 1, mc("barrel", facing="up"))
    m.fill(7, 1, 1, 7, 3, 3, mc("spruce_planks")); m.carve(7, 1, 2, 7, 2, 2)      # biombo do quarto
    m.set(8, 1, 6, mc("barrel", facing="up")); m.set(8, 2, 6, mc("barrel", facing="up"))
    m.set(1, 1, 3, mc("smoker", facing="east"))
    # quarto do Seu Zé atrás de um biombo
    m.bed(8, 2, "north", "brown")
    m.set(5, 3, 3, lantern(True))
    m.set(-1, 1, 8, mc("barrel", facing="up")); m.set(10, 1, 8, mc("barrel", facing="up"))
    m.note(5, 1, 4, "balcão"); m.note(4, 1, 1, "prateleiras vazias")
    return m


def olaria():
    """A olaria da Dona Zefa: terracota e tijolo, roda de oleiro (cortador de pedra), potes, varais."""
    m = Model("olaria", 12, 9, "Olaria · Dona Zefa")
    m.floor(0, 0, 11, 8, mc("bricks"))
    m.walls(0, 0, 11, 8, 1, 3, mc("terracotta"), corner=mc("bricks"))
    m.walls(0, 0, 11, 8, 1, 1, mc("bricks"))
    m.carve(1, 1, 1, 10, 3, 7)
    m.door(6, 8, "south", wood="jungle")
    m.window(2, 2, 8); m.window(9, 2, 8); m.window(3, 2, 0); m.window(8, 2, 0); m.window(0, 2, 4); m.window(11, 2, 4)
    m.gable(0, 0, 11, 8, 4, "brick", mc("terracotta"), ridge="x")
    # dentro: roda de oleiro (cortador de pedra), potes prontos e quebrados, bancada, cama
    m.set(5, 1, 3, mc("stonecutter", facing="south")); m.note(5, 1, 3, "roda de oleiro")
    for x, f in [(1, "north"), (2, "east"), (3, "south")]:
        m.set(x, 1, 1, mc("decorated_pot", facing=f))
    m.set(1, 2, 1, mc("decorated_pot", facing="south"))
    m.set(10, 1, 1, mc("barrel", facing="up")); m.set(9, 1, 1, mc("clay")); m.set(9, 2, 1, mc("clay"))
    m.bed(10, 5, "north", "orange"); m.set(10, 1, 7, mc("chest", facing="north"))
    m.table(7, 5, wood="jungle", cover=mc("orange_carpet")); m.chair(7, 6, "north", wood="jungle")
    m.set(6, 3, 4, lantern(True)); m.set(2, 3, 5, lantern(True))
    m.set(1, 1, 7, pot("orange_tulip"))
    # lá fora: o forno rachado e os varais de secar barro
    for x, z in [(13, 2), (14, 2), (15, 2), (13, 3), (15, 3), (13, 4), (14, 4), (15, 4)]:
        m.fill(x, 1, z, x, 2, z, mc("bricks"))
    m.set(14, 3, 3, mc("brick_slab", type="bottom")); m.set(14, 1, 3, mc("campfire", lit="false"))
    m.set(15, 2, 4, mc("cracked_stone_bricks")); m.set(13, 3, 2, mc("bricks")); m.set(16, 1, 4, mc("brick_stairs", facing="west", half="bottom", shape="straight"))
    for z in (6, 8):
        m.set(13, 1, z, mc("oak_fence")); m.set(16, 1, z, mc("oak_fence"))
        for x in range(13, 17):
            m.set(x, 2, z, mc("oak_slab", type="bottom"))
    m.note(14, 1, 3, "forno rachado"); m.note(14, 2, 7, "varais")
    return m


def carvoaria():
    """A carvoaria do Seu Lino: cabana de toras, monte de carvão apagado, lenha empilhada."""
    m = Model("carvoaria", 10, 8, "Carvoaria · Seu Lino")
    m.floor(0, 0, 7, 6, mc("coarse_dirt"))
    m.walls(0, 0, 7, 6, 1, 3, mc("spruce_log", axis="y"))
    for z in range(1, 6):
        for y in (1, 2, 3):
            m.set(0, y, z, mc("spruce_log", axis="z")); m.set(7, y, z, mc("spruce_log", axis="z"))
    for x in range(1, 7):
        for y in (1, 2, 3):
            m.set(x, y, 0, mc("spruce_log", axis="x")); m.set(x, y, 6, mc("spruce_log", axis="x"))
    m.carve(1, 1, 1, 6, 3, 5)
    m.door(3, 6, "south", wood="spruce")
    m.window(5, 2, 6); m.window(0, 2, 3); m.window(7, 2, 3)
    m.gable(0, 0, 7, 6, 4, "spruce", mc("spruce_planks"), ridge="x")
    # dentro: cama, alto-forno (do carvoeiro), barril de carvão, mesa
    m.bed(1, 1, "south", "black"); m.set(6, 1, 1, mc("blast_furnace", facing="west")); m.set(6, 1, 2, mc("barrel", facing="up"))
    m.set(6, 1, 5, mc("coal_block")); m.table(4, 3, wood="spruce", cover=mc("black_carpet")); m.chair(4, 4, "north")
    m.set(3, 3, 3, lantern(True))
    # lá fora: o monte de carvão apagado (terra sobre lenha) e a lenha
    for x in range(8, 13):
        for z in range(0, 5):
            d = abs(x - 10) + abs(z - 2)
            if d <= 2:
                m.set(x, 1, z, mc("coarse_dirt"))
            if d <= 1:
                m.set(x, 2, z, mc("dirt") if (x + z) % 2 else mc("coarse_dirt"))
    m.set(10, 3, 2, mc("campfire", lit="false"))
    m.fill(8, 1, 6, 9, 2, 7, mc("oak_log", axis="x")); m.set(8, 3, 6, mc("oak_log", axis="x"))
    m.set(10, 1, 7, mc("oak_log", axis="y")); m.set(10, 2, 7, mc("oak_slab", type="bottom"))
    m.note(10, 1, 2, "carvoaria apagada"); m.note(8, 1, 6, "lenha")
    return m


def vaqueiro():
    """A casa do Seu Tonho: pequena, couro e feno, caldeirão do curtidor, sela na parede."""
    m = Model("vaqueiro", 9, 7, "Vaqueiro · Seu Tonho")
    m.floor(0, 0, 8, 6, mc("spruce_planks"))
    m.walls(0, 0, 8, 6, 1, 3, mc("stripped_spruce_log", axis="y"))
    m.walls(0, 0, 8, 6, 1, 1, mc("cobblestone"))
    m.carve(1, 1, 1, 7, 3, 5)
    m.door(4, 6, "south", wood="spruce")
    m.window(2, 2, 6); m.window(6, 2, 6); m.window(4, 2, 0); m.window(0, 2, 3)
    m.gable(0, 0, 8, 6, 4, "spruce", mc("spruce_planks"), ridge="x")
    m.bed(1, 1, "south", "brown"); m.set(7, 1, 1, mc("cauldron")); m.set(6, 1, 1, mc("barrel", facing="up"))
    m.set(7, 1, 5, mc("hay_block", axis="y")); m.set(7, 2, 5, mc("hay_block", axis="y")); m.set(5, 1, 5, mc("chest", facing="north"))
    m.set(4, 3, 3, lantern(True))
    m.set(9, 1, 2, mc("hay_block", axis="x")); m.set(9, 1, 3, mc("hay_block", axis="x")); m.set(9, 2, 2, mc("hay_block", axis="y"))
    m.note(7, 1, 1, "caldeirão do curtidor")
    return m


def pasto():
    """O pasto do Seu Tonho: cerca com porteira, abrigo de feno, cocho de água. Grande para uma vaca só."""
    m = Model("pasto", 42, 40, "Pasto")
    for x in range(42):
        for z in (0, 39):
            m.set(x, 1, z, mc("oak_fence"))
    for z in range(40):
        for x in (0, 41):
            m.set(x, 1, z, mc("oak_fence"))
    m.set(20, 1, 0, mc("oak_fence_gate", facing="north", open="false", in_wall="false")); m.entrance = (20, 0, "north")
    # abrigo: quatro postes, telhado de lajes, feno
    for x, z in [(3, 3), (9, 3), (3, 8), (9, 8)]:
        m.fill(x, 1, z, x, 3, z, mc("spruce_fence"))
    m.fill(2, 4, 2, 10, 4, 9, mc("spruce_slab", type="bottom"))
    m.fill(4, 1, 4, 6, 1, 5, mc("hay_block", axis="x")); m.set(4, 2, 4, mc("hay_block", axis="y"))
    for x in range(16, 20):
        m.set(x, 1, 34, mc("water_cauldron", level="3") if x % 2 else mc("cauldron"))
    for x, z in [(25, 20), (30, 12), (12, 25), (33, 30), (20, 15)]:
        m.set(x, 1, z, mc("short_grass")); m.set(x + 1, 1, z, mc("dandelion"))
    m.note(6, 1, 5, "abrigo e feno"); m.note(17, 1, 34, "cocho")
    return m


def roca():
    """A roça da Dona Cida: terra arada com o canal de água no meio, trigo em vários pontos, cerca."""
    m = Model("roca", 20, 20, "Roça de trigo")
    for x in range(20):
        for z in range(20):
            if x in (0, 19) or z in (0, 19):
                m.set(x, 1, z, mc("oak_fence"))
                continue
            if x in (5, 14):                                     # dois canais: a terra fica úmida até a cerca
                m.set(x, 0, z, mc("water"))
                continue
            m.set(x, 0, z, mc("farmland", moisture="7"))
            age = 7 if z < 8 else (4 if z < 14 else 1)
            m.set(x, 1, z, mc("wheat", age=str(age)))
    m.set(10, 1, 19, mc("oak_fence_gate", facing="south", open="false", in_wall="false")); m.entrance = (10, 19, "south")
    m.set(10, 0, 19, mc("dirt_path"))
    m.note(5, 0, 10, "canais de água")
    return m


# =====================================================================================
# Praça, fonte, coreto, poço
# =====================================================================================

def praca(stage=1):
    """A praça: pedregulho com musgo e falhas (estágio 1), a fonte, o coreto, o sino, bancos."""
    import random
    rnd = random.Random(3)
    m = Model("praca" if stage == 1 else f"praca_{stage}", 27, 19, "Praça")
    for x in range(27):
        for z in range(19):
            r = rnd.random()
            if stage == 1:
                s = mc("cobblestone") if r < 0.62 else mc("mossy_cobblestone") if r < 0.8 else mc("gravel") if r < 0.9 else mc("grass_block")
            else:
                s = mc("stone_bricks") if r < 0.7 else mc("polished_andesite") if r < 0.9 else mc("cobblestone")
            m.set(x, 0, z, s)
            if stage == 1 and s == mc("grass_block") and rnd.random() < 0.5:
                m.set(x, 1, z, mc("short_grass"))
    # a fonte no centro (13, 9)
    cx, cz = 13, 9
    for x in range(cx - 4, cx + 5):
        for z in range(cz - 4, cz + 5):
            d = ((x - cx) ** 2 + (z - cz) ** 2) ** 0.5
            if 3.2 < d <= 4.4:
                m.set(x, 1, z, mc("stone_brick_wall") if stage == 1 and (x + z) % 5 else mc("stone_brick_wall"))
                m.set(x, 0, z, mc("stone_bricks"))
            elif d <= 3.2:
                m.set(x, 0, z, mc("coarse_dirt") if stage == 1 else mc("water"))
                m.clear(x, 1, z)
    m.fill(cx, 0, cz, cx, 2, cz, mc("chiseled_stone_bricks"))
    # no alto da coluna: a laje quebrada (estágio 1) ou uma bacia com água (água solta escorreria pela praça)
    m.set(cx, 3, cz, mc("stone_brick_slab", type="bottom") if stage == 1 else mc("water_cauldron", level="3"))
    if stage == 1:
        for x, z in [(cx + 4, cz + 1), (cx - 4, cz), (cx + 1, cz - 4), (cx - 2, cz + 4)]:
            m.clear(x, 1, z)
            m.set(x, 0, z, mc("cracked_stone_bricks"))
        m.set(cx + 1, 1, cz + 1, mc("cobblestone")); m.set(cx - 1, 1, cz - 2, mc("cobweb") if False else mc("dead_bush"))
        m.note(cx, 1, cz, "fonte seca e quebrada")
    else:
        m.note(cx, 1, cz, "fonte com água")
    # o coreto (a oeste): piso de tábuas, quatro colunas; no estágio 1, sem telhado e telhas no chão
    m.fill(1, 0, 1, 8, 0, 8, mc("spruce_planks"))
    for x, z in [(1, 1), (8, 1), (1, 8), (8, 8)]:
        m.fill(x, 1, z, x, 3, z, mc("oak_log", axis="y"))
    if stage == 1:
        for x, z in [(0, 4), (9, 2), (4, 9), (6, 0), (3, 3)]:
            m.set(x, 1, z, mc("brick_stairs", facing="north", half="bottom", shape="straight") if (x + z) % 2 else mc("brick_slab", type="bottom"))
        m.note(4, 1, 4, "coreto sem telhado")
    else:
        m.fill(0, 4, 0, 9, 4, 9, mc("brick_slab", type="top"))
        m.gable(1, 1, 8, 8, 5, "brick", mc("bricks"), ridge="x")
        m.set(4, 3, 4, lantern(True)); m.set(5, 3, 5, lantern(True))
        for x in (2, 7):
            m.set(x, 1, 4, mc("spruce_stairs", facing="west" if x == 7 else "east", half="bottom", shape="straight"))
        m.note(4, 1, 4, "coreto com telhado")
    # o sino num poste, bancos, uma lanterna só (estágio 1) ou várias
    m.set(20, 1, 3, mc("oak_fence")); m.set(20, 2, 3, mc("bell", attachment="floor", facing="south"))
    for x in (11, 15):
        m.set(x, 1, 17, mc("spruce_stairs", facing="south", half="bottom", shape="straight"))
    for x in (11, 15):
        m.set(x, 1, 1, mc("spruce_stairs", facing="north", half="bottom", shape="straight"))
    lamps = [(23, 2)] if stage == 1 else [(23, 2), (2, 16), (23, 16), (13, 1), (13, 17)]
    for x, z in lamps:
        m.set(x, 1, z, mc("oak_fence")); m.set(x, 2, z, mc("oak_fence")); m.set(x, 3, z, lantern(False))
    m.set(25, 1, 17, mc("barrel", facing="up")); m.set(1, 1, 17, mc("barrel", facing="up"))
    m.note(20, 1, 3, "sino")
    return m


def poco(stage=1):
    """O poço da praça: seco no estágio 1 (telhadinho caído), com água a partir do 4."""
    m = Model("poco" if stage == 1 else f"poco_{stage}", 4, 4, "Poço")
    m.walls(0, 0, 3, 3, 1, 1, mc("cobblestone") if stage == 1 else mc("stone_bricks"))
    m.fill(1, 0, 1, 2, 0, 2, mc("coarse_dirt") if stage == 1 else mc("water"))
    for x, z in ((0, 0), (3, 3)):
        m.fill(x, 2, z, x, 3, z, mc("oak_fence"))
    if stage == 1:
        m.set(0, 4, 0, mc("oak_slab", type="bottom")); m.set(1, 4, 0, mc("oak_slab", type="bottom"))
        m.note(1, 1, 1, "poço seco")
    else:
        m.fill(0, 4, 0, 3, 4, 3, mc("oak_slab", type="bottom")); m.set(1, 3, 1, mc("oak_fence")); m.set(1, 2, 1, mc("lantern", hanging="true"))
        m.note(1, 1, 1, "poço com água")
    m.entrance = (1, 3, "south")
    return m


# =====================================================================================
# Prefeitura por estágio
# =====================================================================================

def prefeitura(stage=3):
    """A prefeitura cresce com a vila: estágio 3 é o gabinete modesto (o secretário, a mesa, o livro);
    estágio 4 ganha andar, relógio de sol e varanda; estágio 6 vira o paço municipal de pedra, com
    torre do sino, salão do conselho e brasão."""
    if stage <= 3:
        m = Model("prefeitura_3", 12, 9, "Prefeitura (estágio 3): o gabinete")
        m.floor(0, 0, 11, 8, mc("spruce_planks"))
        m.walls(0, 0, 11, 8, 1, 3, mc("oak_planks"), corner=mc("stripped_oak_log", axis="y"))
        m.walls(0, 0, 11, 8, 1, 1, mc("stone_bricks"))
        m.carve(1, 1, 1, 10, 3, 7)
        m.door(5, 8, "south", wood="oak"); m.door(6, 8, "south", wood="oak", hinge="right", main=False)
        for x in (2, 9):
            m.window(x, 2, 8)
        m.window(3, 2, 0); m.window(8, 2, 0); m.window(0, 2, 4); m.window(11, 2, 4)
        m.gable(0, 0, 11, 8, 4, "spruce", mc("oak_planks"), ridge="x")
        m.set(5, 3, 9, mc("white_wall_banner", facing="south")); m.set(6, 3, 9, mc("yellow_wall_banner", facing="south"))
        # gabinete: a mesa do secretário (atril com o livro), cadeiras de quem é atendido, arquivo
        m.set(5, 1, 2, mc("lectern", facing="south", has_book="false")); m.chair(5, 1, "south")
        m.set(4, 1, 2, mc("spruce_slab", type="top")); m.set(6, 1, 2, mc("spruce_slab", type="top"))
        for x in (4, 6):
            m.chair(x, 4, "north")
        for x in (1, 2, 9, 10):
            m.set(x, 1, 1, mc("bookshelf")); m.set(x, 2, 1, mc("bookshelf"))
        m.set(10, 1, 7, mc("barrel", facing="up")); m.set(1, 1, 7, pot("red_tulip"))
        m.set(3, 1, 6, mc("cartography_table")); m.fill(4, 1, 5, 7, 1, 7, mc("green_carpet"))
        m.set(5, 3, 4, lantern(True)); m.set(2, 3, 2, lantern(True)); m.set(9, 3, 2, lantern(True))
        m.note(5, 1, 2, "mesa do secretário"); m.note(1, 1, 1, "arquivo")
        return m
    if stage == 4:
        m = Model("prefeitura_4", 12, 9, "Prefeitura (estágio 4): sobrado com varanda")
        m.floor(0, 0, 11, 8, mc("stone_bricks"))
        m.walls(0, 0, 11, 8, 1, 3, mc("stone_bricks"), corner=mc("chiseled_stone_bricks"))
        m.fill(0, 4, 0, 11, 4, 8, mc("spruce_planks"))
        m.walls(0, 0, 11, 8, 5, 7, mc("white_terracotta"), corner=mc("stripped_spruce_log", axis="y"))
        m.carve(1, 1, 1, 10, 3, 7); m.carve(1, 5, 1, 10, 7, 7)
        m.door(5, 8, "south", wood="spruce"); m.door(6, 8, "south", wood="spruce", hinge="right", main=False)
        for x in (2, 9):
            m.window(x, 2, 8, height=2)
        for x in (2, 4, 7, 9):
            m.window(x, 6, 8)
        m.window(0, 2, 4, height=2); m.window(11, 2, 4, height=2); m.window(0, 6, 4); m.window(11, 6, 4)
        # varanda do andar de cima, sobre a porta
        m.fill(3, 4, 9, 8, 4, 10, mc("spruce_slab", type="top"))
        for x in range(3, 9):
            m.set(x, 5, 10, mc("spruce_fence"))
        m.set(5, 5, 8, mc("spruce_door", facing="south", half="lower", hinge="left", open="false"))   # porta da varanda
        m.set(5, 6, 8, mc("spruce_door", facing="south", half="upper", hinge="left", open="false"))
        m.gable(0, 0, 11, 8, 8, "spruce", mc("white_terracotta"), ridge="x")
        m.set(5, 10, 4, mc("bell", attachment="floor", facing="south"))
        for y in (1, 2, 3, 4):
            m.set(10, y, 6, mc("ladder", facing="west"))
        m.carve(10, 4, 6, 10, 4, 6); m.set(10, 4, 6, mc("ladder", facing="west"))
        # térreo: atendimento, bancos; em cima: salão do conselho, mesa comprida
        m.set(5, 1, 2, mc("lectern", facing="south", has_book="false")); m.chair(5, 1, "south")
        for x in (3, 4, 7, 8):
            m.chair(x, 5, "north")
        for x in (1, 2, 9):
            m.set(x, 1, 1, mc("bookshelf")); m.set(x, 2, 1, mc("bookshelf")); m.set(x, 3, 1, mc("bookshelf"))
        for x in range(3, 9):
            m.table(x, 4, y=5, wood="dark_oak", cover=mc("green_carpet"))
            m.chair(x, 3, "south", y=5, wood="dark_oak"); m.chair(x, 5, "north", y=5, wood="dark_oak")
        m.set(5, 7, 4, lantern(True)); m.set(5, 3, 4, lantern(True)); m.set(2, 3, 5, lantern(True)); m.set(9, 3, 5, lantern(True))
        m.note(5, 1, 2, "atendimento"); m.note(5, 5, 4, "salão do conselho")
        return m
    m = Model("prefeitura_6", 16, 12, "Prefeitura (estágio 6): o paço municipal")
    m.floor(0, 0, 15, 11, mc("polished_andesite"))
    m.fill(1, 0, 1, 14, 0, 10, mc("dark_oak_planks"))
    m.walls(0, 0, 15, 11, 1, 3, mc("stone_bricks"), corner=mc("chiseled_stone_bricks"))
    m.fill(0, 4, 0, 15, 4, 11, mc("stone_brick_slab", type="double"))
    m.walls(0, 0, 15, 11, 5, 7, mc("stone_bricks"), corner=mc("chiseled_stone_bricks"))
    m.carve(1, 1, 1, 14, 3, 10); m.carve(1, 5, 1, 14, 7, 10)
    m.fill(1, 4, 1, 14, 4, 10, mc("dark_oak_planks"))
    for x in (3, 6, 9, 12):
        m.window(x, 1, 11, height=3); m.window(x, 5, 11, height=3); m.window(x, 5, 0, height=2)
    m.window(0, 2, 5, height=2); m.window(15, 2, 5, height=2)
    m.carve(7, 1, 11, 8, 3, 11)
    m.door(7, 11, "south", wood="dark_oak"); m.door(8, 11, "south", wood="dark_oak", hinge="right", main=False)
    # escadaria e colunas na frente
    m.fill(5, 0, 12, 10, 0, 12, mc("stone_brick_stairs", facing="north", half="bottom", shape="straight"))
    for x in (5, 10):
        m.fill(x, 1, 12, x, 3, 12, mc("quartz_pillar", axis="y"))
    m.fill(4, 4, 12, 11, 4, 12, mc("stone_brick_slab", type="bottom"))
    m.gable(0, 0, 15, 11, 8, "deepslate_tile", mc("stone_bricks"), ridge="x")
    # a torre do sino no meio
    # a torre do sino no meio, acima da cumeeira: arcos abertos nos quatro lados e o sino no meio
    m.walls(6, 4, 9, 7, 8, 18, mc("stone_bricks"), corner=mc("chiseled_stone_bricks"))
    m.fill(7, 8, 5, 8, 14, 6, mc("air"))
    m.carve(7, 16, 4, 8, 17, 7); m.carve(6, 16, 5, 9, 17, 6)
    m.fill(7, 15, 5, 8, 15, 6, mc("stone_brick_slab", type="top"))
    m.set(7, 16, 5, mc("bell", attachment="floor", facing="south"))
    m.fill(5, 19, 3, 10, 19, 8, mc("deepslate_tile_slab", type="bottom"))
    m.fill(6, 20, 4, 9, 20, 7, mc("deepslate_tile_slab", type="bottom")); m.set(7, 21, 5, mc("lightning_rod", facing="up"))
    for x in (2, 13):
        m.set(x, 3, 12, mc("lantern", hanging="true"))
    m.set(6, 3, 12, mc("yellow_wall_banner", facing="south")); m.set(9, 3, 12, mc("yellow_wall_banner", facing="south"))
    # térreo: saguão com o atendimento e o mural; em cima: o salão do conselho
    m.set(7, 1, 3, mc("lectern", facing="south", has_book="false")); m.set(8, 1, 3, mc("lectern", facing="south", has_book="false"))
    for x in (2, 3, 12, 13):
        m.set(x, 1, 1, mc("bookshelf")); m.set(x, 2, 1, mc("bookshelf")); m.set(x, 3, 1, mc("bookshelf"))
    for x in (4, 11):
        for z in (6, 8):
            m.chair(x, z, "north")
    m.fill(6, 1, 5, 9, 1, 9, mc("red_carpet"))
    for x in range(4, 12):
        m.table(x, 5, y=5, wood="dark_oak", cover=mc("red_carpet"))
        m.chair(x, 4, "south", y=5, wood="dark_oak"); m.chair(x, 6, "north", y=5, wood="dark_oak")
    for x, z in [(4, 3), (11, 3), (4, 8), (11, 8)]:
        m.set(x, 3, z, lantern(True)); m.set(x, 7, z, lantern(True))
    for y in (1, 2, 3, 4):
        m.set(14, y, 9, mc("ladder", facing="west"))
    m.carve(14, 4, 9, 14, 4, 9); m.set(14, 4, 9, mc("ladder", facing="west"))
    m.note(7, 1, 3, "saguão"); m.note(7, 5, 5, "salão do conselho"); m.note(7, 16, 5, "torre do sino")
    return m


# =====================================================================================
# Ruas por estágio
# =====================================================================================

STREET_STYLES = {
    1: ("terra com mato", [("dirt_path", 0.62), ("coarse_dirt", 0.14), ("grass_block", 0.16), ("gravel", 0.08)]),
    2: ("cascalho batido", [("gravel", 0.5), ("dirt_path", 0.3), ("coarse_dirt", 0.12), ("cobblestone", 0.08)]),
    4: ("calçamento de pedra", [("cobblestone", 0.55), ("andesite", 0.25), ("stone", 0.12), ("gravel", 0.08)]),
    6: ("pedra com meio-fio", [("stone_bricks", 0.6), ("polished_andesite", 0.25), ("cracked_stone_bricks", 0.1), ("andesite", 0.05)]),
}
STREET_EDGE = {1: None, 2: None, 4: "andesite", 6: "polished_andesite"}
STREET_LAMPS = {1: 0, 2: 28, 4: 20, 6: 14}       # uma lanterna a cada tantos blocos de rua (0: nenhuma)


# =====================================================================================
# Variações: a mesma planta com outra madeira, outra pedra e outras cores (casas novas do estágio 3)
# =====================================================================================
import re as _re


def variant(base, name, title, swaps):
    """`base` com trocas de material: (velho, novo) casam com o nome inteiro do material, depois de
    `minecraft:` ou de `stripped_` (assim `oak` não pega `dark_oak`)."""
    full = base()
    m = Model(name, full.width, full.depth, title)
    pattern = [(_re.compile(r"(?<=minecraft:)(stripped_)?" + old + r"(?=_|\[|$)"), new) for old, new in swaps]
    for k, s in full.blocks.items():
        for rx, new in pattern if not s.startswith("minecraft:potted_") else ():
            s = rx.sub(lambda mt: (mt.group(1) or "") + new, s)
        m.blocks[k] = s
    m.notes, m.entrance, m.core = list(full.notes), full.entrance, full.core
    return m


def casa_lima():
    return variant(casa_pedra_tabua, "casa_lima", "Casa dos Lima",
                   [("oak", "birch"), ("spruce", "dark_oak"), ("red", "cyan"), ("cobblestone", "mossy_cobblestone")])


def casa_alaranjada():
    return variant(casa_varanda, "casa_alaranjada", "Casa da varanda de acácia",
                   [("birch", "acacia"), ("oak", "spruce"), ("white", "orange"), ("light_blue", "yellow")])


def casa_tijolo():
    return variant(casa_taipa, "casa_tijolo", "Casa de tijolo",
                   [("mud_bricks", "bricks"), ("mud_brick", "brick"), ("packed_mud", "terracotta"), ("orange", "red")])


def sobrado_escuro():
    return variant(sobrado, "sobrado_escuro", "Sobrado de pedra e carvalho escuro",
                   [("spruce", "dark_oak"), ("cobblestone", "stone_bricks"), ("green", "purple"), ("lime", "magenta")])


def casa_cerejeira():
    return variant(casa_jardim, "casa_cerejeira", "Casa da cerejeira",
                   [("spruce", "cherry"), ("polished_andesite", "polished_granite"), ("brown", "pink")])


def casa_enxaimel_nova():
    """A casa enxaimel que estava em obra, pronta no estágio 2 (abeto e terracota branca)."""
    return variant(casa_enxaimel, "casa_enxaimel_nova", "Casa enxaimel nova",
                   [("dark_oak", "spruce"), ("calcite", "white_terracotta"), ("light_blue", "lime"), ("yellow", "pink")])


def casa_varanda_nova():
    """A casa com varanda que estava em obra, pronta no estágio 2 (carvalho e abeto)."""
    return variant(casa_varanda, "casa_varanda_nova", "Casa da varanda nova",
                   [("oak", "spruce"), ("birch", "oak"), ("white", "light_gray")])


NEW_HOUSE_MODELS = [casa_lima, casa_alaranjada, casa_tijolo, sobrado_escuro, casa_cerejeira]


def casa_reformada():
    """A casa abandonada, reformada no estágio 2: telhado novo de abeto, musgo limpo."""
    m = casa_pedra("casa_reformada", "Casa de pedra reformada", roof="spruce")
    for k, s in list(m.blocks.items()):
        if "mossy_stone_bricks" in s or "cracked_stone_bricks" in s:
            m.blocks[k] = mc("stone_bricks")
        elif "moss_carpet" in s:
            m.blocks[k] = pot("red_tulip") if k[0] < 5 else mc("rose_bush", half="lower")
    m.set(8, 2, 8, mc("rose_bush", half="upper"))
    return m


def all_models():
    """Todos os modelos, para gerar as funções e as prévias."""
    out = [f() for f in HOUSES]
    out += [em_obra(casa_enxaimel), em_obra(casa_varanda), ruina(), lote()]
    out += [ferreiro(), lavradora(), vendinha(), olaria(), carvoaria(), vaqueiro(), pasto(), roca()]
    out += [praca(1), praca(2), poco(1), poco(4), prefeitura(3), prefeitura(4), prefeitura(6)]
    out += [f() for f in NEW_HOUSE_MODELS] + [casa_reformada()]
    return out


# =====================================================================================
# Acabamento por estágio: o desgaste do estágio 1 e o capricho que chega depois
# =====================================================================================

FLOWERS = ["poppy", "dandelion", "cornflower", "azure_bluet", "oxeye_daisy", "allium", "red_tulip", "pink_tulip"]


def _front(m):
    """A parede da frente (a da porta, no sul): z, janelas da frente, x da porta."""
    a0, b0, a1, b1 = m.core
    door_x = m.entrance[0]
    windows = sorted(x for (x, y, z), s in m.blocks.items() if z == b1 and y == 2 and "glass_pane" in s)
    return a0, a1, b1, door_x, windows


def _free(m, x, y, z):
    return m.get(x, y, z) is None or m.get(x, y, z) == mc("air")


def wear(m, seed):
    """Estágio 1, a vila em declínio: uma vidraça quebrada, teia no beiral, mato junto das paredes.
    Devolve o modelo mudado (o mesmo objeto) e o que mudou (para o estágio 2 desfazer)."""
    import random
    rnd = random.Random(seed)
    a0, a1, b1, door_x, windows = _front(m)
    changed = []
    if windows:
        x = rnd.choice(windows)
        changed.append(((x, 2, b1), m.get(x, 2, b1)))
        m.set(x, 2, b1, mc("air"))
    for x in (a0, a1):
        if _free(m, x, 3, b1 + 1) and not _free(m, x, 4, b1 + 1):
            changed.append(((x, 3, b1 + 1), None)); m.set(x, 3, b1 + 1, mc("cobweb"))
            break
    for x in range(a0 - 1, a1 + 2):
        for z in (b1 + 1,):
            if abs(x - door_x) > 1 and _free(m, x, 1, z) and rnd.random() < 0.45:
                plant = mc("short_grass") if rnd.random() < 0.7 else mc("fern")
                changed.append(((x, 1, z), None)); m.set(x, 1, z, plant)
    return changed


def finish(m, level, seed, worn=()):
    """O que o acabamento do estágio põe: um modelo só com os blocos novos (nada é limpo).
    `level` 2: conserta o desgaste, lanterna na porta, jardineiras e flores na frente.
    `level` 4: venezianas, banco ao lado da porta, vasos nas janelas dos lados."""
    import random
    rnd = random.Random(seed * 7 + level)
    a0, a1, b1, door_x, windows = _front(m)
    out = Model(m.name + f"_acabamento_{level}", m.width, m.depth, m.title + f" (acabamento do estágio {level})")
    out.core, out.entrance = m.core, m.entrance
    if level == 2:
        for (x, y, z), old in worn:
            out.set(x, y, z, old if old else mc("air"))
        for x in (door_x + 1, door_x - 1):
            if _free(m, x, 3, b1 + 1) and not _free(m, x, 4, b1 + 1):
                out.set(x, 3, b1 + 1, lantern(True)); break
        for x in windows:
            if abs(x - door_x) > 1 and _free(m, x, 1, b1 + 1) and _free(m, x, 2, b1 + 1):
                out.set(x, 1, b1 + 1, mc("spruce_trapdoor", facing="south", half="top", open="false"))
                out.set(x, 2, b1 + 1, pot(rnd.choice(["red_tulip", "dandelion", "poppy", "blue_orchid", "allium"])))
        for x in range(a0 - 1, a1 + 2):
            if abs(x - door_x) > 1 and _free(m, x, 1, b1 + 1) and (x, 1, b1 + 1) not in out.blocks and rnd.random() < 0.6:
                out.set(x, 1, b1 + 1, mc(rnd.choice(FLOWERS)))
    else:
        for x in windows:
            for sx in (-1, 1):
                if _free(m, x + sx, 2, b1 + 1) and abs(x + sx - door_x) > 0:
                    out.set(x + sx, 2, b1 + 1, mc("spruce_trapdoor", facing="south", half="bottom", open="true"))
        for x in (door_x - 2, door_x + 2):
            if a0 < x < a1 and _free(m, x, 1, b1 + 1):
                out.set(x, 1, b1 + 1, mc("spruce_stairs", facing="north", half="bottom", shape="straight")); break
        a0_, b0_, a1_, b1_ = m.core
        for (x, y, z), s in m.blocks.items():
            if "glass_pane" in s and y == 2 and (x in (a0_, a1_)) and _free(m, x + (1 if x == a1_ else -1), 1, z):
                bush = rnd.choice(["rose_bush", "lilac", "peony"])
                out.set(x + (1 if x == a1_ else -1), 1, z, mc(bush, half="lower"))
                out.set(x + (1 if x == a1_ else -1), 2, z, mc(bush, half="upper"))
    return out


def worn(factory, seed):
    """O prédio do estágio 1 com o desgaste e a lista do que mudou."""
    m = factory()
    return m, wear(m, seed)
