"""Cidade inicial do Veio: gera `function/town/build.mcfunction` e `workforce/town_data.rs`.

A cidade sai pronta, como uma estrutura pré-montada: o gerador limpa todo o volume da cidade
(árvores, galhos, água rasa, morros baixos), nivela o chão e só então constrói. Nada do terreno
original sobra dentro do perímetro.

Plano (x para leste, z para o sul, a partir do centro da praça):

- **Praça** (17 x 17): calçamento de pedra, chafariz ao norte, sino, bancos, árvores e postes.
  O centro fica livre (é onde o jogador renasce e onde o prefeito aparece).
- **Lote do fundador** (norte da praça): terreno reservado e nivelado para a casa do jogador,
  que `home.rs` constrói logo depois. A porta dá para a praça.
- **Avenidas** de pedra (leste-oeste e sul) com postes a cada 8 blocos; **ruas** de terra batida
  num anel em volta do centro; **cerca** no perímetro com portões nas avenidas.
- **Comércio** (leste): praça do mercado com barracas coloridas (cada uma com a estação de um
  ofício) e uma fileira de sobrados com loja embaixo e moradia em cima, de frente para a avenida.
- **Cívico** (noroeste): lote da prefeitura (reservado para o canteiro da obra) e um parque.
- **Residencial** (oeste e sul): casas de três paletas (carvalho, abeto, bétula) e sobrados.
- **Distrito industrial** (leste): lote plano e marcado para a oficina do tutorial e um galpão.
- **Lavouras** (sul): canteiros irrigados com trigo, cenoura, batata e beterraba, e um celeiro.
- **Bairros** (2026-11-09, pedido do dono: a cidade era pequena para a escala do Veio): em volta
  do centro (|x|, |z| <= `CORE`), até a cerca (`HALF`), quadras de quatro casas ou de sobrados com
  casas, praças de bairro com laguinho, lavouras ao sul e galpões a leste, com ruas de terra e
  avenidas até a cerca nos quatro sentidos. Cerca de 140 prédios novos e 300 camas: a cidade
  nasce com 20 moradores e cabe crescer até o teto de 500. Encher as casas é tarefa do jogador.

As casas do centro têm 30 camas (300 com os bairros); a cidade nasce com 20 aldeões (`STARTING_VILLAGERS`), dois deles o casal
fundador protegido (`workforce/founders.rs`). As posições que o Rust usa (lotes, refúgio, portão
e lugares dos moradores) vão para `native/api/src/workforce/town_data.rs`.

O Rust roda `execute positioned X Y Z run function veio:town/build` com (X, Y, Z) no centro da
praça e Y no primeiro bloco de ar acima do chão (o chão fica em Y - 1).

Uso: python tools/build_town.py
"""
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "content/data/veio/function/town/build.mcfunction"
DATA = ROOT / "native/api/src/workforce/town_data.rs"

HALF = 96                 # perímetro em |x| = 96 e |z| = 96 (cerca)
CORE = 44                 # o centro desenhado à mão fica em |x|, |z| <= 44; fora dele, os bairros
CLEAR_UP = 40             # ar limpo do chão até 40 blocos acima
FILL_DOWN = 8             # buracos e água preenchidos até 8 blocos abaixo do chão
STARTING_VILLAGERS = 20
MAX_FILL = 32768          # limite de blocos de um `fill`

DIRS = ["north", "east", "south", "west"]
CW = {"north": "east", "east": "south", "south": "west", "west": "north"}
ATTACHED = ("lantern", "torch", "carpet", "potted_", "_bed", "_door", "bell", "candle", "wheat",
            "carrots", "potatoes", "beetroots", "poppy", "dandelion", "cornflower", "azure_bluet",
            "oxeye_daisy", "allium", "short_grass", "ladder", "_trapdoor", "pressure_plate", "lily_pad")
STATE = re.compile(r"^([a-z0-9_:]+)(?:\[([a-z0-9_=,]+)\])?$")


def attached(spec):
    return any(part in spec for part in ATTACHED)


def rotate_spec(spec):
    """Gira as propriedades de um estado de bloco 90 graus no sentido horário (visto de cima)."""
    match = STATE.match(spec)
    assert match, spec
    block, props = match.group(1), match.group(2)
    if not props:
        return spec
    out = []
    for pair in props.split(","):
        key, value = pair.split("=")
        if key == "facing" and value in CW:
            value = CW[value]
        elif key == "axis" and value in ("x", "z"):
            value = "z" if value == "x" else "x"
        elif key in CW:
            key = CW[key]
        out.append(f"{key}={value}")
    return f"{block}[{','.join(sorted(out))}]"


class Shape:
    """Blocos de um prédio em coordenadas locais: x (0..w-1), y (0 = chão), z (0..d-1).

    A forma canônica tem a frente (a porta) virada para o sul; `rotated` gira no sentido horário:
    1 = frente a oeste, 2 = ao norte, 3 = a leste.
    """

    def __init__(self, w, d):
        self.w, self.d = w, d
        self.blocks = {}
        self.spots = []

    def put(self, x, y, z, spec):
        assert 0 <= x < self.w and 0 <= z < self.d, (x, z, self.w, self.d)
        self.blocks[(x, y, z)] = spec

    def box(self, x1, y1, z1, x2, y2, z2, spec):
        for x in range(min(x1, x2), max(x1, x2) + 1):
            for y in range(min(y1, y2), max(y1, y2) + 1):
                for z in range(min(z1, z2), max(z1, z2) + 1):
                    self.put(x, y, z, spec)

    def spot(self, x, y, z):
        self.spots.append((x, y, z))

    def rotated(self, turns):
        shape = self
        for _ in range(turns % 4):
            out = Shape(shape.d, shape.w)
            for (x, y, z), spec in shape.blocks.items():
                out.blocks[(shape.d - 1 - z, y, x)] = rotate_spec(spec)
            out.spots = [(shape.d - 1 - z, y, x) for x, y, z in shape.spots]
            shape = out
        return shape


# ------------------------------------------------------------------ Paletas

PALETTES = {
    "oak": dict(log="minecraft:oak_log", planks="minecraft:oak_planks", roof="minecraft:spruce",
                floor="minecraft:spruce_planks", door="minecraft:oak_door", bed="minecraft:red_bed",
                trap="minecraft:spruce_trapdoor", base="minecraft:cobblestone"),
    "spruce": dict(log="minecraft:spruce_log", planks="minecraft:spruce_planks", roof="minecraft:dark_oak",
                   floor="minecraft:oak_planks", door="minecraft:spruce_door", bed="minecraft:blue_bed",
                   trap="minecraft:dark_oak_trapdoor", base="minecraft:stone_bricks"),
    "birch": dict(log="minecraft:stripped_dark_oak_log", planks="minecraft:birch_planks", roof="minecraft:brick",
                  floor="minecraft:oak_planks", door="minecraft:birch_door", bed="minecraft:yellow_bed",
                  trap="minecraft:birch_trapdoor", base="minecraft:cobblestone"),
    "stone": dict(log="minecraft:stripped_spruce_log", planks="minecraft:white_terracotta", roof="minecraft:mud_brick",
                  floor="minecraft:spruce_planks", door="minecraft:spruce_door", bed="minecraft:green_bed",
                  trap="minecraft:spruce_trapdoor", base="minecraft:stone_bricks"),
}


def stairs(material, facing):
    return f"{material}_stairs[facing={facing},half=bottom]"


def slab(material):
    return f"{material}_slab[type=bottom]"


def walls(s, x1, z1, x2, z2, y1, y2, v):
    """Paredes de tábua com cantos de tronco."""
    for y in range(y1, y2 + 1):
        for x in range(x1, x2 + 1):
            for z in (z1, z2):
                s.put(x, y, z, v["planks"])
        for z in range(z1, z2 + 1):
            for x in (x1, x2):
                s.put(x, y, z, v["planks"])
        for x, z in ((x1, z1), (x1, z2), (x2, z1), (x2, z2)):
            s.put(x, y, z, f"{v['log']}[axis=y]")


def gable(s, x1, z1, x2, z2, y, v):
    """Telhado de duas águas sobre as paredes x1..x2, z1..z2 (cumeeira leste-oeste), com beiral."""
    roof = v["roof"]
    for x in range(x1, x2 + 1):
        for z in range(z1, z2 + 1):
            s.put(x, y, z, v["floor"])           # forro
    zn, zs = z1 - 1, z2 + 1
    while zs - zn >= 1:
        for x in range(x1 - 1, x2 + 2):
            s.put(x, y, zn, stairs(roof, "south"))
            s.put(x, y, zs, stairs(roof, "north"))
        if y > 0 and zs - zn >= 2:
            for z in range(zn + 1, zs):
                for x in (x1, x2):
                    if s.blocks.get((x, y, z)) is None:
                        s.put(x, y, z, v["planks"])
        zn, zs, y = zn + 1, zs - 1, y + 1
    if zn == zs:
        for x in range(x1 - 1, x2 + 2):
            s.put(x, y - 1, zn, slab(roof))
    return y


def window(s, x, y, z, facing_axis):
    """Vidro: `ew` numa parede norte/sul, `ns` numa parede leste/oeste."""
    s.put(x, y, z, "minecraft:glass_pane[east=true,west=true]" if facing_axis == "ew"
          else "minecraft:glass_pane[north=true,south=true]")


def door(s, x, z, y, v):
    s.put(x, y, z, f"{v['door']}[facing=north,half=lower,hinge=left]")
    s.put(x, y + 1, z, f"{v['door']}[facing=north,half=upper,hinge=left]")


def bed(s, x, y, z, v, facing="north"):
    """Cabeceira em (x, z); o pé fica do lado oposto a `facing`."""
    dx, dz = {"north": (0, 1), "south": (0, -1), "east": (-1, 0), "west": (1, 0)}[facing]
    s.put(x, y, z, f"{v['bed']}[facing={facing},part=head]")
    s.put(x + dx, y, z + dz, f"{v['bed']}[facing={facing},part=foot]")


def floor(s, x1, z1, x2, z2, v, y=0):
    for x in range(x1, x2 + 1):
        for z in range(z1, z2 + 1):
            edge = x in (x1, x2) or z in (z1, z2)
            s.put(x, y, z, v["base"] if edge else v["floor"])


# ------------------------------------------------------------------ Prédios

def casa(p, flowers=("minecraft:poppy", "minecraft:dandelion")):
    """Casa térrea 7 x 7 (9 x 9 com beiral): duas camas, mesa, baú e jardineira na frente."""
    v = PALETTES[p]
    s = Shape(9, 9)
    floor(s, 1, 1, 7, 7, v)
    walls(s, 1, 1, 7, 7, 1, 3, v)
    for x in (3, 5):
        window(s, x, 2, 1, "ew")
    for z in (3, 5):
        window(s, 1, 2, z, "ns")
        window(s, 7, 2, z, "ns")
    for x in (2, 6):
        window(s, x, 2, 7, "ew")
    door(s, 4, 7, 1, v)
    s.put(4, 0, 8, "minecraft:dirt_path")
    for x in range(2, 7):
        for z in range(2, 7):
            for y in range(1, 4):
                s.put(x, y, z, "minecraft:air")
    bed(s, 2, 1, 2, v)
    bed(s, 6, 1, 2, v)
    s.put(4, 1, 2, "minecraft:barrel[facing=up]")
    s.put(4, 2, 2, "minecraft:candle[candles=2,lit=true]")
    s.put(2, 1, 5, "minecraft:chest[facing=east]")
    s.put(2, 1, 6, "minecraft:crafting_table")
    s.put(5, 1, 5, "minecraft:oak_fence")
    s.put(5, 2, 5, "minecraft:oak_pressure_plate")
    s.put(6, 1, 5, stairs("minecraft:oak", "east"))
    s.put(4, 1, 5, stairs("minecraft:oak", "west"))
    s.put(6, 1, 6, "minecraft:potted_red_tulip")
    s.put(4, 3, 4, "minecraft:lantern[hanging=true]")
    s.spot(3, 1, 4)
    s.spot(5, 1, 4)
    # Jardineiras de alçapão com flores dos dois lados da porta.
    for x, flower in ((2, flowers[0]), (6, flowers[1])):
        s.put(x, 0, 8, "minecraft:grass_block")
        s.put(x, 1, 8, flower)
    gable(s, 1, 1, 7, 7, 4, v)
    return s


def sobrado(p, shop, awning="minecraft:spruce_trapdoor"):
    """Sobrado 7 x 9: loja (ou cozinha) embaixo, quarto com duas camas em cima, escada de mão."""
    v = PALETTES[p]
    s = Shape(9, 11)
    floor(s, 1, 1, 7, 9, v)
    walls(s, 1, 1, 7, 9, 1, 7, v)
    for x in range(2, 7):
        for z in range(2, 9):
            for y in (1, 2, 3, 5, 6, 7):
                s.put(x, y, z, "minecraft:air")
            s.put(x, 4, z, v["floor"])
    # Janelas: vitrine embaixo, três janelas em cima.
    for x in (3, 5):
        window(s, x, 2, 1, "ew")
        window(s, x, 6, 1, "ew")
    for z in (4, 7):
        window(s, 1, 2, z, "ns")
        window(s, 7, 2, z, "ns")
        window(s, 1, 6, z, "ns")
        window(s, 7, 6, z, "ns")
    for x in (2, 6):
        window(s, x, 2, 9, "ew")
    for x in (2, 4, 6):
        window(s, x, 6, 9, "ew")
    door(s, 4, 9, 1, v)
    s.put(4, 0, 10, "minecraft:dirt_path")
    # Escada de mão até o andar de cima.
    for y in range(1, 6):
        s.put(6, y, 2, "minecraft:ladder[facing=south]")
    if shop:
        for x in (2, 3, 5, 6):
            s.put(x, 1, 6, "minecraft:smooth_stone_slab[type=top]")
        for x in (2, 3, 4):
            s.put(x, 1, 2, "minecraft:barrel[facing=up]")
        s.put(2, 2, 6, "minecraft:candle[candles=3,lit=true]")
        s.put(6, 2, 6, "minecraft:potted_cactus")
    else:
        s.put(2, 1, 2, "minecraft:smoker[facing=south]")
        s.put(3, 1, 2, "minecraft:barrel[facing=up]")
        s.put(4, 1, 2, "minecraft:water_cauldron[level=3]")
        s.put(4, 1, 6, "minecraft:oak_fence")
        s.put(4, 2, 6, "minecraft:oak_pressure_plate")
        s.put(3, 1, 6, stairs("minecraft:oak", "west"))
        s.put(5, 1, 6, stairs("minecraft:oak", "east"))
    s.put(2, 1, 8, "minecraft:potted_fern")
    s.put(6, 1, 8, "minecraft:potted_azure_bluet")
    s.put(4, 3, 5, "minecraft:lantern[hanging=true]")
    # Andar de cima.
    bed(s, 2, 5, 3, v)
    bed(s, 4, 5, 3, v)
    s.put(3, 5, 3, "minecraft:barrel[facing=up]")
    s.put(3, 6, 3, "minecraft:candle[candles=1,lit=true]")
    s.put(2, 5, 8, "minecraft:chest[facing=east]")
    s.put(6, 5, 8, "minecraft:bookshelf")
    s.put(6, 5, 6, "minecraft:potted_dandelion")
    s.put(4, 7, 6, "minecraft:lantern[hanging=true]")
    s.spot(3, 5, 6)
    s.spot(5, 5, 6)
    # Toldo sobre a porta e lanternas dos lados.
    for x in range(2, 7):
        s.put(x, 3, 10, f"{awning}[facing=north,half=top,open=false]")
    for x in (2, 6):
        s.put(x, 2, 10, "minecraft:lantern[hanging=true]")
    gable(s, 1, 1, 7, 9, 8, v)
    return s


def stall(color, station):
    """Barraca do mercado: toldo listrado sobre quatro postes, balcão com a estação do ofício."""
    s = Shape(5, 5)
    for x in range(5):
        for z in range(5):
            s.put(x, 0, z, "minecraft:stone_bricks" if x in (0, 4) or z in (0, 4) else "minecraft:spruce_planks")
    for x, z in ((1, 1), (3, 1), (1, 3), (3, 3)):
        s.put(x, 1, z, "minecraft:spruce_fence")
        s.put(x, 2, z, "minecraft:spruce_fence")
    for x in range(5):
        for z in range(5):
            stripe = color if x % 2 == 0 else "minecraft:white"
            s.put(x, 3, z, f"{stripe}_wool")
    s.put(2, 1, 3, station)
    s.put(2, 1, 1, "minecraft:barrel[facing=up]")
    s.put(1, 1, 2, "minecraft:hay_block[axis=y]")
    s.put(3, 1, 2, "minecraft:spruce_slab[type=bottom]")
    s.put(3, 2, 2, "minecraft:potted_red_mushroom")
    s.put(2, 2, 2, "minecraft:lantern[hanging=true]")
    return s


def shed():
    """Galpão do distrito industrial: estrutura aberta de troncos, telhado e materiais empilhados."""
    v = dict(PALETTES["spruce"], roof="minecraft:cobblestone", floor="minecraft:spruce_planks")
    s = Shape(13, 11)
    for x in range(1, 12):
        for z in range(1, 10):
            s.put(x, 0, z, "minecraft:gravel" if 1 < x < 11 and 1 < z < 9 else "minecraft:stone_bricks")
    for x in (1, 6, 11):
        for z in (1, 9):
            for y in range(1, 4):
                s.put(x, y, z, "minecraft:spruce_log[axis=y]")
    for z in (5,):
        for x in (1, 11):
            for y in range(1, 4):
                s.put(x, y, z, "minecraft:spruce_log[axis=y]")
    # Paredes só no fundo (norte); o resto fica aberto.
    for x in range(2, 11):
        if x != 6:
            for y in range(1, 4):
                s.put(x, y, 1, "minecraft:spruce_planks")
    for x, z, block in ((3, 2, "minecraft:oak_log[axis=x]"), (4, 2, "minecraft:oak_log[axis=x]"),
                        (3, 3, "minecraft:oak_log[axis=x]"), (8, 2, "minecraft:cobblestone"),
                        (9, 2, "minecraft:cobblestone"), (8, 3, "minecraft:cobblestone"),
                        (5, 2, "minecraft:crafting_table"), (7, 2, "minecraft:anvil[facing=north]"),
                        (9, 3, "minecraft:barrel[facing=up]"), (2, 3, "minecraft:chest[facing=east]")):
        s.put(x, 1, z, block)
    s.put(3, 2, 2, "minecraft:oak_log[axis=x]")
    s.put(8, 2, 2, "minecraft:cobblestone")
    s.put(6, 3, 5, "minecraft:lantern[hanging=true]")
    gable(s, 1, 1, 11, 9, 4, v)
    for x in range(2, 11):
        for z in range(2, 9):
            s.blocks.pop((x, 4, z), None)
            s.put(x, 4, z, "minecraft:air")
    return s


def barn():
    """Celeiro das lavouras: tábuas de carvalho escuro, portão largo e fardos de feno."""
    v = dict(PALETTES["spruce"], planks="minecraft:dark_oak_planks", log="minecraft:dark_oak_log",
             roof="minecraft:dark_oak", base="minecraft:cobblestone", floor="minecraft:spruce_planks")
    s = Shape(11, 11)
    floor(s, 1, 1, 9, 9, v)
    walls(s, 1, 1, 9, 9, 1, 4, v)
    for x in range(2, 9):
        for z in range(2, 9):
            for y in range(1, 5):
                s.put(x, y, z, "minecraft:air")
    for x in (4, 5, 6):
        for y in (1, 2, 3):
            s.put(x, y, 9, "minecraft:air")
    for x in (3, 7):
        window(s, x, 3, 1, "ew")
    for z in (4, 6):
        window(s, 1, 3, z, "ns")
        window(s, 9, 3, z, "ns")
    for x, z in ((2, 2), (3, 2), (2, 3), (7, 2), (8, 2), (8, 3)):
        s.put(x, 1, z, "minecraft:hay_block[axis=y]")
    s.put(2, 2, 2, "minecraft:hay_block[axis=y]")
    s.put(8, 2, 2, "minecraft:hay_block[axis=y]")
    s.put(5, 1, 2, "minecraft:composter")
    s.put(2, 1, 7, "minecraft:barrel[facing=up]")
    s.put(8, 1, 7, "minecraft:chest[facing=west]")
    s.put(5, 4, 5, "minecraft:lantern[hanging=true]")
    s.spot(5, 1, 5)
    for x in range(4, 7):
        s.put(x, 0, 10, "minecraft:dirt_path")
    gable(s, 1, 1, 9, 9, 5, v)
    return s


# ------------------------------------------------------------------ Cidade

class Town:
    def __init__(self):
        self.ground = {}      # (x, z) -> bloco do chão (y = -1)
        self.blocks = {}      # (x, y, z) -> bloco (y relativo ao ar)
        self.owner = {}       # (x, z) -> quem ocupa (para conferir sobreposições)
        self.spots = []
        self.reserved = {}

    def claim(self, x1, z1, x2, z2, name):
        for x in range(x1, x2 + 1):
            for z in range(z1, z2 + 1):
                assert -HALF < x < HALF and -HALF < z < HALF, f"{name} fora da cidade em {x} {z}"
                known = self.owner.get((x, z))
                assert known is None, f"{name} sobrepõe {known} em {x} {z}"
                self.owner[(x, z)] = name

    def floor(self, x1, z1, x2, z2, block, claim=None):
        if claim:
            self.claim(x1, z1, x2, z2, claim)
        for x in range(min(x1, x2), max(x1, x2) + 1):
            for z in range(min(z1, z2), max(z1, z2) + 1):
                self.ground[(x, z)] = block

    def set(self, x, y, z, block):
        if y == -1:
            self.ground[(x, z)] = block
        else:
            self.blocks[(x, y, z)] = block

    def place(self, shape, x0, z0, turns, name, claim=True):
        """Põe um prédio com o canto noroeste (depois de girado) em (x0, z0)."""
        shape = shape.rotated(turns)
        if claim:
            self.claim(x0, z0, x0 + shape.w - 1, z0 + shape.d - 1, name)
        for (x, y, z), spec in shape.blocks.items():
            self.set(x0 + x, y - 1, z0 + z, spec)
        self.spots += [(x0 + x, y - 1, z0 + z) for x, y, z in shape.spots]

    # Detalhes de rua -------------------------------------------------
    def lamp(self, x, z):
        self.set(x, 0, z, "minecraft:spruce_fence")
        self.set(x, 1, z, "minecraft:spruce_fence")
        self.set(x, 2, z, "minecraft:lantern[hanging=false]")

    def tree(self, x, z, log="minecraft:oak_log", leaves="minecraft:oak_leaves"):
        for y in range(0, 5):
            self.set(x, y, z, f"{log}[axis=y]")
        for y, radius in ((3, 2), (4, 2), (5, 1), (6, 1)):
            for dx in range(-radius, radius + 1):
                for dz in range(-radius, radius + 1):
                    if radius == 2 and abs(dx) == 2 and abs(dz) == 2:
                        continue
                    if y == 6 and abs(dx) + abs(dz) > 1:
                        continue
                    if (dx, dz) == (0, 0) and y < 5:
                        continue
                    self.set(x + dx, y, z + dz, f"{leaves}[persistent=true]")

    def bench(self, x, z, facing):
        self.set(x, 0, z, f"minecraft:spruce_stairs[facing={facing},half=bottom]")


def street(town, x1, z1, x2, z2, name, paved):
    town.claim(x1, z1, x2, z2, name)
    for x in range(min(x1, x2), max(x1, x2) + 1):
        for z in range(min(z1, z2), max(z1, z2) + 1):
            if paved:
                along_x = abs(x2 - x1) >= abs(z2 - z1)
                middle = z not in (z1, z2) if along_x else x not in (x1, x2)
                town.ground[(x, z)] = "minecraft:polished_andesite" if middle else "minecraft:stone_bricks"
            else:
                town.ground[(x, z)] = "minecraft:dirt_path"


def plan():
    t = Town()

    # --- Praça (17 x 17) ---
    t.claim(-8, -8, 8, 8, "praça")
    for x in range(-8, 9):
        for z in range(-8, 9):
            border = abs(x) == 8 or abs(z) == 8
            t.ground[(x, z)] = "minecraft:stone_bricks" if border else (
                "minecraft:polished_andesite" if (x + z) % 2 == 0 else "minecraft:andesite")
    for x in range(-1, 2):
        for z in range(-1, 2):
            t.ground[(x, z)] = "minecraft:chiseled_stone_bricks" if (x, z) == (0, 0) else "minecraft:polished_diorite"
    # Chafariz ao norte do centro (o centro fica livre para o renascimento e o prefeito).
    for x in range(-2, 3):
        for z in range(-7, -2):
            rim = abs(x) == 2 or z in (-7, -3)
            if rim:
                t.set(x, 0, z, "minecraft:stone_brick_wall" if abs(x) == 2 and z in (-7, -3) else "minecraft:stone_bricks")
            else:
                t.ground[(x, z)] = "minecraft:water"
    t.set(0, 0, -5, "minecraft:chiseled_stone_bricks")
    t.set(0, 1, -5, "minecraft:stone_brick_wall")
    t.set(0, 2, -5, "minecraft:water")
    for x, z in ((-2, -7), (2, -7), (-2, -3), (2, -3)):
        t.set(x, 1, z, "minecraft:lantern[hanging=false]")
    # Fundo da bacia.
    for x in range(-1, 2):
        for z in range(-6, -3):
            t.blocks[(x, -2, z)] = "minecraft:stone_bricks"
    # Árvores, bancos, postes e o sino (ponto de encontro dos aldeões).
    for x, z in ((-6, -6), (6, -6), (-6, 6), (6, 6)):
        t.tree(x, z, "minecraft:birch_log", "minecraft:birch_leaves")
    for x in (-4, 4):
        t.bench(x, -5, "east" if x < 0 else "west")
        t.bench(x, -4, "east" if x < 0 else "west")
    for x in (-3, -2, 2, 3):
        t.bench(x, 5, "south")
    for x, z in ((-7, -3), (7, -3), (-7, 3), (7, 3), (-3, 7), (3, 7)):
        t.lamp(x, z)
    t.set(5, -1, 3, "minecraft:chiseled_stone_bricks")
    t.set(5, 0, 3, "minecraft:bell[attachment=floor,facing=west]")
    for x, z, flower in ((-5, 2, "minecraft:poppy"), (-5, 3, "minecraft:dandelion"), (-4, 3, "minecraft:cornflower"),
                         (4, -2, "minecraft:oxeye_daisy"), (5, -2, "minecraft:allium")):
        t.ground[(x, z)] = "minecraft:grass_block"
        t.set(x, 0, z, flower)

    # --- Ruas ---
    street(t, -43, -1, -9, 1, "avenida oeste", True)
    street(t, 9, -1, 43, 1, "avenida leste", True)
    street(t, -1, 9, 1, 43, "avenida sul", True)
    street(t, -1, -9, 1, -9, "acesso da casa", True)
    street(t, -29, -40, -27, -2, "rua oeste norte", False)
    street(t, -29, 2, -27, 40, "rua oeste sul", False)
    street(t, 27, -40, 29, -2, "rua leste norte", False)
    street(t, 27, 2, 29, 40, "rua leste sul", False)
    street(t, -40, 23, -30, 25, "rua sul oeste", False)
    street(t, -26, 23, -2, 25, "rua sul centro oeste", False)
    street(t, 2, 23, 26, 25, "rua sul centro leste", False)
    street(t, 30, 23, 40, 25, "rua sul leste", False)
    street(t, -40, -25, -30, -23, "rua norte oeste", False)
    street(t, -26, -25, -9, -23, "rua norte centro oeste", False)
    street(t, 9, -25, 26, -23, "rua norte centro leste", False)
    street(t, 30, -25, 40, -23, "rua norte leste", False)
    # Postes nas avenidas (calçada a 2 blocos do eixo) e nas ruas.
    for x in range(12, 44, 8):
        for side in (-1, 1):
            t.lamp(side * x, 2 if side < 0 else -2)
    for z in range(12, 44, 8):
        t.lamp(-2, z)
    for z in (-36, -12, 12, 36):
        t.lamp(-30, z)
        t.lamp(30, z)

    # --- Lote do fundador (a casa do jogador vem de home.rs) ---
    t.floor(-7, -23, 7, -10, "minecraft:grass_block", claim="lote do fundador")
    t.reserved["founder_lot"] = (-7, -1, -23)

    # --- Cívico (noroeste): lote da prefeitura e parque ---
    t.claim(-22, -22, -10, -10, "lote da prefeitura")
    for x in range(-22, -9):
        for z in range(-22, -9):
            edge = x in (-22, -10) or z in (-22, -10)
            t.ground[(x, z)] = "minecraft:polished_andesite" if edge else "minecraft:gravel"
    for x, z in ((-22, -22), (-10, -22), (-22, -10), (-10, -10)):
        t.lamp(x, z)
    for x, z, block in ((-20, -20, "minecraft:oak_log[axis=x]"), (-19, -20, "minecraft:oak_log[axis=x]"),
                        (-20, -19, "minecraft:cobblestone"), (-19, -19, "minecraft:cobblestone"),
                        (-18, -20, "minecraft:crafting_table")):
        t.set(x, 0, z, block)
    t.reserved["refuge"] = (-16, 0, -16)
    t.claim(-22, -8, -10, -2, "jardim cívico")
    for x in range(-22, -9):
        for z in range(-8, -1):
            t.ground[(x, z)] = "minecraft:grass_block"
    for x in range(-22, -9):
        t.ground[(x, -5)] = "minecraft:dirt_path"
    t.tree(-19, -7)
    t.tree(-13, -3, "minecraft:spruce_log", "minecraft:spruce_leaves")
    for x, flower in ((-21, "minecraft:poppy"), (-17, "minecraft:dandelion"), (-15, "minecraft:azure_bluet"),
                      (-11, "minecraft:cornflower")):
        t.set(x, 0, -3, flower)

    # Parque (noroeste externo) e pomar (atrás da casa).
    t.claim(-40, -40, -30, -26, "parque")
    t.claim(-8, -40, 8, -26, "pomar")
    for x in range(-40, -29):
        t.ground[(x, -33)] = "minecraft:dirt_path"
    for z in range(-40, -25):
        t.ground[(-35, z)] = "minecraft:dirt_path"
    for x in range(-33, -30):
        for z in range(-31, -27):
            t.ground[(x, z)] = "minecraft:water"
            t.blocks[(x, -2, z)] = "minecraft:clay"
    t.set(-32, 0, -29, "minecraft:lily_pad")
    for x, z in ((-38, -38), (-38, -29), (-32, -38)):
        t.tree(x, z)
    for x in (-38, -32):
        t.bench(x, -34, "south")
    t.lamp(-36, -35)
    t.lamp(-34, -31)
    for x in (-5, 0, 5):
        for z in (-37, -30):
            t.tree(x, z)
    for x, z in ((-2, -34), (3, -34), (-7, -33), (7, -33)):
        t.set(x, 0, z, "minecraft:short_grass")

    # --- Comércio: praça do mercado (nordeste interno) ---
    t.claim(10, -22, 26, -3, "praça do mercado")
    for x in range(10, 27):
        for z in range(-22, -2):
            t.ground[(x, z)] = "minecraft:polished_andesite" if (x + z) % 3 else "minecraft:andesite"
    stalls = [("minecraft:red", "minecraft:smoker[facing=south]"), ("minecraft:yellow", "minecraft:composter"),
              ("minecraft:blue", "minecraft:barrel[facing=south]"), ("minecraft:green", "minecraft:fletching_table"),
              ("minecraft:orange", "minecraft:loom[facing=south]"), ("minecraft:purple", "minecraft:cartography_table")]
    positions = [(10, -21, 0), (16, -21, 0), (10, -11, 2), (16, -11, 2)]
    for index, (x0, z0, turns) in enumerate(positions):
        color, station = stalls[index]
        t.place(stall(color, station), x0, z0, turns, f"barraca {index}", claim=False)
    for x, z in ((26, -15), (10, -15)):
        t.lamp(x, z)
    t.tree(16, -14, "minecraft:oak_log", "minecraft:azalea_leaves")
    for index, (x0, z0, turns) in enumerate(((22, -21, 0), (22, -11, 2)), start=4):
        color, station = stalls[index]
        t.place(stall(color, station), x0, z0, turns, f"barraca {index}", claim=False)

    # --- Comércio: fileira de sobrados com loja (sudeste interno), frente para a avenida ---
    t.place(sobrado("oak", True), 9, 3, 2, "loja 1")
    t.place(sobrado("stone", True, "minecraft:dark_oak_trapdoor"), 18, 3, 2, "loja 2")
    t.place(casa("birch"), 3, 14, 0, "casa sudeste 1")
    t.place(casa("spruce", ("minecraft:allium", "minecraft:oxeye_daisy")), 13, 14, 0, "casa sudeste 2")

    # --- Residencial (sudoeste interno) ---
    t.place(casa("oak"), -26, 3, 2, "casa 1")
    t.place(casa("birch", ("minecraft:cornflower", "minecraft:allium")), -17, 3, 2, "casa 2")
    t.place(casa("spruce"), -21, 14, 0, "casa 3")
    t.place(casa("stone", ("minecraft:azure_bluet", "minecraft:poppy")), -11, 14, 0, "casa 4")

    # --- Residencial (oeste externo), de frente para a rua oeste ---
    for index, (z0, p) in enumerate(((-21, "spruce"), (-11, "oak"), (3, "birch"), (13, "stone"))):
        t.place(casa(p), -40, z0, 3, f"casa oeste {index}")

    # --- Residencial (nordeste externo): sobrados de frente para a rua leste ---
    t.place(sobrado("spruce", False), 31, -40, 1, "sobrado nordeste 1")
    t.place(sobrado("oak", False), 12, -39, 0, "sobrado nordeste 2")

    # --- Distrito industrial (leste externo): galpão e lote da oficina ---
    t.place(shed(), 31, -17, 0, "galpão")
    t.claim(31, 4, 43, 18, "lote da oficina")
    for x in range(31, 44):
        for z in range(4, 19):
            edge = x in (31, 43) or z in (4, 18)
            t.ground[(x, z)] = "minecraft:smooth_stone" if edge else "minecraft:grass_block"
    for x, z in ((31, 4), (43, 4), (31, 18), (43, 18)):
        t.lamp(x, z)
    t.reserved["workshop"] = (37, 0, 11)

    # --- Lavouras (sul externo) e celeiro ---
    crops = ["minecraft:wheat[age=7]", "minecraft:carrots[age=7]", "minecraft:potatoes[age=7]",
             "minecraft:beetroots[age=3]", "minecraft:wheat[age=5]", "minecraft:carrots[age=4]"]
    farms = [(-40, 28), (-26, 28), (-12, 28), (2, 28), (13, 28)]
    for index, (x0, z0) in enumerate(farms):
        t.claim(x0, z0, x0 + 9, z0 + 9, f"lavoura {index}")
        crop = crops[index % len(crops)]
        for x in range(x0, x0 + 10):
            for z in range(z0, z0 + 10):
                edge = x in (x0, x0 + 9) or z in (z0, z0 + 9)
                if edge:
                    t.ground[(x, z)] = "minecraft:grass_block"
                    gate = z == z0 and x in (x0 + 4, x0 + 5)
                    if not gate:
                        t.set(x, 0, z, "minecraft:oak_fence")
                elif x == x0 + 5:
                    t.ground[(x, z)] = "minecraft:water"
                else:
                    t.ground[(x, z)] = "minecraft:farmland[moisture=7]"
                    t.set(x, 0, z, crop)
        t.set(x0, 1, z0, "minecraft:lantern[hanging=false]")
    t.place(barn(), 31, 28, 2, "celeiro")
    # --- Bairros de fora (2026-11-09): a cidade cresceu 5 vezes para a escala do Veio ---
    districts(t)

    # Portões da cerca.
    t.reserved["gate"] = (0, 0, HALF - 4)

    # --- Perímetro: cerca com portões nas avenidas e postes nos portões ---
    for i in range(-HALF, HALF + 1):
        for x, z in ((i, -HALF), (i, HALF), (-HALF, i), (HALF, i)):
            opening = (abs(x) == HALF and -1 <= z <= 1) or (abs(z) == HALF and -1 <= x <= 1)
            if not opening:
                t.set(x, 0, z, "minecraft:oak_fence")
    for x, z in ((HALF, -2), (HALF, 2), (-HALF, -2), (-HALF, 2), (-2, HALF), (2, HALF), (-2, -HALF), (2, -HALF)):
        t.set(x, 1, z, "minecraft:lantern[hanging=false]")
    for x in range(-HALF, HALF + 1):
        for z in (-HALF, HALF):
            t.ground[(x, z)] = t.ground.get((x, z), "minecraft:grass_block")
    return t


# ------------------------------------------------------------------ Bairros

# Quadras dos bairros, em cada eixo; entre elas, ruas de terra de 3 blocos. As faixas -2..2 são a
# avenida (calçada nas pontas) e 45..47 é a rua do anel em volta do centro.
LOTS = [(-92, -72), (-68, -48), (-44, -25), (-21, -3), (3, 21), (25, 44), (48, 68), (72, 92)]
PALETTE_ORDER = ["oak", "spruce", "birch", "stone"]
FLOWER_ORDER = [("minecraft:poppy", "minecraft:dandelion"), ("minecraft:cornflower", "minecraft:allium"),
                ("minecraft:azure_bluet", "minecraft:oxeye_daisy"), ("minecraft:allium", "minecraft:poppy")]


def in_core(interval):
    return -CORE <= interval[0] and interval[1] <= CORE


def quarter_kind(ix, iz, xr, zr):
    """Tipo da quadra: casas, sobrados com casas, parque, lavoura ou galpões (leste)."""
    if xr == LOTS[-1] and abs(zr[0]) <= CORE:
        return "industrial"
    if zr == LOTS[-1] and xr in (LOTS[1], LOTS[2], LOTS[5], LOTS[6]):
        return "farm"
    if (ix, iz) in ((0, 0), (7, 0), (0, 7), (3, 0), (4, 7), (0, 4)):
        return "park"
    return "mixed" if (ix * 3 + iz * 5) % 3 == 0 else "houses"


def districts(t):
    """Enche o anel entre o centro (CORE) e a cerca (HALF) com quadras de casas e serviços."""
    count = 0
    for ix, xr in enumerate(LOTS):
        for iz, zr in enumerate(LOTS):
            if in_core(xr) and in_core(zr):
                continue
            kind = quarter_kind(ix, iz, xr, zr)
            name = f"quadra {ix},{iz}"
            seed = ix * 8 + iz
            a, b = xr
            c, d = zr
            w, h = b - a + 1, d - c + 1
            x0 = a + (w - 18) // 2
            p = lambda k: PALETTE_ORDER[(seed + k) % len(PALETTE_ORDER)]
            f = lambda k: FLOWER_ORDER[(seed + k) % len(FLOWER_ORDER)]
            if kind == "houses" or (kind == "mixed" and h < 20):
                # Duas casas de frente para a rua do norte, duas para a do sul, quintal no meio.
                t.place(casa(p(0), f(0)), x0, c, 2, f"{name} casa 1")
                t.place(casa(p(1), f(1)), x0 + 9, c, 2, f"{name} casa 2")
                t.place(casa(p(2), f(2)), x0, d - 8, 0, f"{name} casa 3")
                t.place(casa(p(3), f(3)), x0 + 9, d - 8, 0, f"{name} casa 4")
                count += 4
                if h > 18:
                    t.tree(a + w // 2, c + h // 2, "minecraft:oak_log", "minecraft:oak_leaves")
            elif kind == "mixed":
                # Sobrados com loja para a rua do norte, casas para a do sul.
                t.place(sobrado(p(0), seed % 2 == 0), x0, c, 2, f"{name} sobrado 1")
                t.place(sobrado(p(1), seed % 3 == 0, "minecraft:dark_oak_trapdoor"), x0 + 9, c, 2, f"{name} sobrado 2")
                t.place(casa(p(2), f(2)), x0, d - 8, 0, f"{name} casa 1")
                t.place(casa(p(3), f(3)), x0 + 9, d - 8, 0, f"{name} casa 2")
                count += 4
            elif kind == "industrial":
                t.place(shed(), a + (w - 13) // 2, c + (h - 11) // 2, 0, f"{name} galpão")
                count += 1
            elif kind == "farm":
                farm(t, a, c, b, d, seed, name)
            else:
                park(t, a, c, b, d, seed, name)
            for x, z in ((a - 1, c - 1), (b + 1, c - 1), (a - 1, d + 1), (b + 1, d + 1)):
                if (x, z) not in t.owner and abs(x) < HALF and abs(z) < HALF:
                    t.lamp(x, z)
                    t.owner[(x, z)] = "poste"
    # Avenidas até a cerca, nos quatro sentidos.
    street(t, -HALF + 1, -1, -CORE - 1, 1, "avenida oeste (bairros)", True)
    street(t, CORE + 1, -1, HALF - 1, 1, "avenida leste (bairros)", True)
    street(t, -1, CORE + 1, 1, HALF - 1, "avenida sul (bairros)", True)
    street(t, -1, -HALF + 1, 1, -CORE - 1, "avenida norte", True)
    # O resto do anel de fora que não é quadra vira rua de terra.
    for x in range(-HALF + 1, HALF):
        for z in range(-HALF + 1, HALF):
            if abs(x) <= CORE and abs(z) <= CORE:
                continue
            if (x, z) not in t.owner and (x, z) not in t.ground:
                t.ground[(x, z)] = "minecraft:dirt_path"
    return count


def farm(t, a, c, b, d, seed, name):
    """Lavoura de bairro: quatro canteiros irrigados com cerca e um poste."""
    crops = ["minecraft:wheat[age=7]", "minecraft:carrots[age=7]", "minecraft:potatoes[age=7]", "minecraft:beetroots[age=3]"]
    t.claim(a, c, b, d, name)
    for x in range(a, b + 1):
        for z in range(c, d + 1):
            edge = x in (a, b) or z in (c, d)
            if edge:
                t.ground[(x, z)] = "minecraft:grass_block"
                if not (z == c and abs(x - (a + b) // 2) <= 1):
                    t.set(x, 0, z, "minecraft:oak_fence")
            elif (x - a) % 5 == 0:
                t.ground[(x, z)] = "minecraft:water"
            else:
                t.ground[(x, z)] = "minecraft:farmland[moisture=7]"
                t.set(x, 0, z, crops[(seed + (x - a) // 5 + (z - c) // 10) % len(crops)])
    t.set(a, 1, c, "minecraft:lantern[hanging=false]")


def park(t, a, c, b, d, seed, name):
    """Praça de bairro: grama, caminhos em cruz, árvores, bancos e um laguinho."""
    t.claim(a, c, b, d, name)
    mx, mz = (a + b) // 2, (c + d) // 2
    for x in range(a, b + 1):
        for z in range(c, d + 1):
            t.ground[(x, z)] = "minecraft:dirt_path" if x == mx or z == mz else "minecraft:grass_block"
    for x in range(mx - 2, mx + 3):
        for z in range(mz - 2, mz + 3):
            if abs(x - mx) == 2 or abs(z - mz) == 2:
                t.ground[(x, z)] = "minecraft:stone_bricks"
            else:
                t.ground[(x, z)] = "minecraft:water"
                t.blocks[(x, -2, z)] = "minecraft:clay"
    logs = [("minecraft:oak_log", "minecraft:oak_leaves"), ("minecraft:birch_log", "minecraft:birch_leaves"),
            ("minecraft:spruce_log", "minecraft:spruce_leaves")]
    for k, (x, z) in enumerate(((a + 3, c + 3), (b - 3, c + 3), (a + 3, d - 3), (b - 3, d - 3))):
        log, leaves = logs[(seed + k) % len(logs)]
        t.tree(x, z, log, leaves)
    t.bench(mx - 4, mz + 1, "east")
    t.bench(mx + 4, mz - 1, "west")
    t.lamp(mx + 3, mz + 3)
    t.lamp(mx - 3, mz - 3)


# ------------------------------------------------------------------ Saída

def chunks(x1, y1, z1, x2, y2, z2):
    """Divide um volume em pedaços dentro do limite do `fill`."""
    step = 16
    height = y2 - y1 + 1
    for x in range(x1, x2 + 1, step):
        for z in range(z1, z2 + 1, step):
            xe, ze = min(x + step - 1, x2), min(z + step - 1, z2)
            rows = max(1, MAX_FILL // ((xe - x + 1) * (ze - z + 1)))
            for y in range(y1, y2 + 1, rows):
                yield x, y, z, xe, min(y + rows - 1, y2), ze
    assert height > 0


def runs(cells):
    """Agrupa blocos iguais seguidos no eixo x: [(x1, x2, y, z, bloco)]."""
    out = []
    for (y, z) in sorted({(y, z) for (_, y, z) in cells}):
        row = sorted((x, spec) for (x, yy, zz), spec in cells.items() if yy == y and zz == z)
        start = None
        for x, spec in row + [(None, None)]:
            if start and (x != start[1] + 1 or spec != start[2]):
                out.append((start[0], start[1], y, z, start[2]))
                start = None
            if x is None:
                break
            if start is None:
                start = [x, x, spec]
            else:
                start[1] = x
    return out


def emit(line_list, x1, x2, y, z, spec):
    if x1 == x2:
        line_list.append(f"setblock ~{x1} ~{y} ~{z} {spec}")
    else:
        line_list.append(f"fill ~{x1} ~{y} ~{z} ~{x2} ~{y} ~{z} {spec}")


def commands(t):
    lines = [
        "# Gerado por tools/build_town.py. Não edite à mão.",
        "# Cidade inicial: rode com `execute positioned <centro da praça, primeiro ar acima do chão> run function veio:town/build`.",
        "# 1. Limpa o volume (árvores, galhos, água, morros baixos).",
    ]
    for x1, y1, z1, x2, y2, z2 in chunks(-HALF, 0, -HALF, HALF, CLEAR_UP, HALF):
        lines.append(f"fill ~{x1} ~{y1} ~{z1} ~{x2} ~{y2} ~{z2} minecraft:air")
    lines.append("# 2. Fundação: tapa buracos e água embaixo e nivela o chão.")
    for liquid in ("minecraft:air", "minecraft:water", "minecraft:lava"):
        for x1, y1, z1, x2, y2, z2 in chunks(-HALF, -FILL_DOWN, -HALF, HALF, -2, HALF):
            lines.append(f"fill ~{x1} ~{y1} ~{z1} ~{x2} ~{y2} ~{z2} minecraft:dirt replace {liquid}")
    for x1, y1, z1, x2, y2, z2 in chunks(-HALF, -1, -HALF, HALF, -1, HALF):
        lines.append(f"fill ~{x1} ~{y1} ~{z1} ~{x2} ~{y2} ~{z2} minecraft:grass_block")
    lines.append("# 3. Chão das ruas, praças e lotes.")
    ground = {(x, -1, z): spec for (x, z), spec in t.ground.items() if spec != "minecraft:grass_block"}
    for x1, x2, y, z, spec in runs(ground):
        emit(lines, x1, x2, y, z, spec)
    below = {key: spec for key, spec in t.blocks.items() if key[1] < -1}
    for x1, x2, y, z, spec in runs(below):
        emit(lines, x1, x2, y, z, spec)
    lines.append("# 4. Prédios e detalhes (primeiro o que se sustenta sozinho, depois o que se apoia).")
    above = {key: spec for key, spec in t.blocks.items() if key[1] >= 0}
    solid = {key: spec for key, spec in above.items() if not attached(spec) and spec != "minecraft:air"}
    hanging = {key: spec for key, spec in above.items() if attached(spec)}
    for x1, x2, y, z, spec in runs(solid):
        emit(lines, x1, x2, y, z, spec)
    for x1, x2, y, z, spec in runs(hanging):
        emit(lines, x1, x2, y, z, spec)
    return lines


def rust(t):
    spots = sorted(set(t.spots))
    assert len(spots) >= STARTING_VILLAGERS, f"{len(spots)} lugares para {STARTING_VILLAGERS} moradores"
    beds = sum(1 for spec in t.blocks.values() if "_bed[" in spec and "part=head" in spec)
    assert beds >= 26, f"só {beds} camas"
    fl, refuge, workshop, gate = (t.reserved[k] for k in ("founder_lot", "refuge", "workshop", "gate"))
    fmt = lambda p: f"[{p[0]}, {p[1]}, {p[2]}]"
    lines = [
        "// Gerado por tools/build_town.py. Não edite à mão.",
        "// Posições relativas ao centro da praça, com y = 0 no primeiro bloco de ar acima do chão.",
        "",
        "/// Meio lado da cidade: o perímetro fica em |x| e |z| iguais a isto.",
        f"pub(crate) const HALF: i32 = {HALF};",
        "/// Quantos aldeões a cidade tem ao nascer.",
        f"pub(crate) const STARTING_VILLAGERS: usize = {STARTING_VILLAGERS};",
        "/// Camas (cabeceiras) das casas da cidade.",
        "#[cfg(test)]",
        f"pub(crate) const BEDS: usize = {beds};",
        "/// Canto noroeste do lote do fundador, no nível do piso (o chão).",
        f"pub(crate) const FOUNDER_LOT: [i32; 3] = {fmt(fl)};",
        "/// Onde o casal fundador se recupera quando cai (o lote da prefeitura).",
        f"pub(crate) const REFUGE: [i32; 3] = {fmt(refuge)};",
        "/// Centro do lote da oficina (distrito industrial).",
        f"pub(crate) const WORKSHOP: [i32; 3] = {fmt(workshop)};",
        "/// Por onde chegam os imigrantes (dentro do portão sul).",
        f"pub(crate) const GATE: [i32; 3] = {fmt(gate)};",
        "/// Lugares dos moradores dentro das casas, do centro para fora.",
        f"pub(crate) const SPOTS: [[i32; 3]; {len(spots)}] = [",
    ]
    spots.sort(key=lambda p: (abs(p[0]) + abs(p[2]), p))
    for start in range(0, len(spots), 6):
        lines.append("    " + " ".join(fmt(p) + "," for p in spots[start:start + 6]))
    lines.append("];")
    return lines, beds


def main():
    t = plan()
    lines = commands(t)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    data, beds = rust(t)
    DATA.write_text("\n".join(data) + "\n", encoding="utf-8")
    print(f"Cidade: {len(lines)} comandos em {OUT.relative_to(ROOT)}; {beds} camas, {len(t.spots)} lugares")


if __name__ == "__main__":
    main()
