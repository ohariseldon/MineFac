"""Prévias das telas do Veio em PNG, sem abrir o jogo.

    python tools/preview_screens.py            # todas as telas, em pt-BR
    python tools/preview_screens.py --lang en_us
    python tools/preview_screens.py --only barn,export_dock

Roda o desenho real do mod (`presentation::screen_draw`) contra o substituto offline do Horizon,
grava o que ele desenhou e pinta em `native/offline-check/target/screens/`, com uma página
`index.html` lado a lado. A fonte é um desenho próprio com as larguras da fonte do Minecraft; os
ícones de itens do Veio usam as texturas do mod e os do Minecraft aparecem como plaquinhas
coloridas (a prévia não tem os arquivos do jogo). Serve para conferir diagramação, textos que
estouram e cores; a referência final continua sendo o jogo.
"""
import argparse
import hashlib
import html
import json
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ui_font  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "content/assets/veio"
TARGET = ROOT / "native/offline-check/target"
OUT = TARGET / "screens"
SCALE = 3

# Nomes do Minecraft que as telas citam; a prévia não tem as traduções do jogo.
VANILLA = {
    "pt_br": {
        "color.minecraft.white": "Branco", "color.minecraft.orange": "Laranja", "color.minecraft.magenta": "Magenta", "color.minecraft.light_blue": "Azul-claro",
        "color.minecraft.yellow": "Amarelo", "color.minecraft.lime": "Verde-limão", "color.minecraft.pink": "Rosa", "color.minecraft.gray": "Cinza",
        "color.minecraft.light_gray": "Cinza-claro", "color.minecraft.cyan": "Ciano", "color.minecraft.purple": "Roxo", "color.minecraft.blue": "Azul",
        "color.minecraft.brown": "Marrom", "color.minecraft.green": "Verde", "color.minecraft.red": "Vermelho", "color.minecraft.black": "Preto",
        "container.inventory": "Inventário",
        "item.minecraft.coal": "Carvão", "item.minecraft.charcoal": "Carvão vegetal",
        "item.minecraft.raw_copper": "Cobre bruto", "item.minecraft.raw_iron": "Ferro bruto",
        "item.minecraft.raw_gold": "Ouro bruto", "item.minecraft.copper_ingot": "Barra de cobre",
        "item.minecraft.iron_ingot": "Barra de ferro", "item.minecraft.gold_ingot": "Barra de ouro",
        "item.minecraft.wheat": "Trigo", "item.minecraft.wheat_seeds": "Sementes de trigo",
        "item.minecraft.bread": "Pão", "item.minecraft.emerald": "Esmeralda",
        "item.minecraft.water_bucket": "Balde de água", "item.minecraft.lava_bucket": "Balde de lava",
        "item.minecraft.bucket": "Balde", "item.minecraft.bone_meal": "Farinha de osso",
        "item.minecraft.quartz": "Quartzo do Nether", "item.minecraft.redstone": "Pó de redstone",
        "item.minecraft.white_dye": "Corante branco", "item.minecraft.red_dye": "Corante vermelho",
        "item.minecraft.blue_dye": "Corante azul", "item.minecraft.carrot": "Cenoura",
        "item.minecraft.potato": "Batata", "item.minecraft.beef": "Carne crua",
        "item.minecraft.cooked_beef": "Bife", "item.minecraft.leather": "Couro",
        "item.minecraft.milk_bucket": "Balde de leite", "item.minecraft.egg": "Ovo",
        "block.minecraft.white_wool": "Lã branca", "block.minecraft.oak_log": "Tronco de carvalho",
        "block.minecraft.oak_sapling": "Muda de carvalho", "block.minecraft.cobblestone": "Pedregulho",
        "block.minecraft.gravel": "Cascalho", "block.minecraft.sand": "Areia",
        "block.minecraft.hay_block": "Fardo de feno", "block.minecraft.chest": "Baú",
    },
    "en_us": {
        "container.inventory": "Inventory",
        "color.minecraft.white": "White", "color.minecraft.orange": "Orange", "color.minecraft.magenta": "Magenta",
        "color.minecraft.light_blue": "Light Blue", "color.minecraft.yellow": "Yellow", "color.minecraft.lime": "Lime",
        "color.minecraft.pink": "Pink", "color.minecraft.gray": "Gray", "color.minecraft.light_gray": "Light Gray",
        "color.minecraft.cyan": "Cyan", "color.minecraft.purple": "Purple", "color.minecraft.blue": "Blue",
        "color.minecraft.brown": "Brown", "color.minecraft.green": "Green", "color.minecraft.red": "Red",
        "color.minecraft.black": "Black",
    },
}

# Cor das plaquinhas dos itens do Minecraft (por palavra no nome).
CHIP_COLORS = [
    ("coal", 0x2b2b2b), ("copper", 0xc9734a), ("iron", 0xd8d8d8), ("gold", 0xf2d24b),
    ("wheat", 0xd9b84a), ("hay", 0xd9b84a), ("bread", 0xb8843f), ("emerald", 0x3fbf5f),
    ("water", 0x3f76e4), ("lava", 0xe0561a), ("seed", 0x6b8f3a), ("wool", 0xe8e8e8),
    ("log", 0x8a6a3a), ("planks", 0xb08850), ("sapling", 0x4f8a2a), ("bone", 0xefefef),
    ("redstone", 0xc02020), ("quartz", 0xeee6da), ("lapis", 0x2f4fb0), ("diamond", 0x5fe0e0),
    ("carrot", 0xe08a20), ("potato", 0xc8a050), ("beef", 0xc04848), ("leather", 0x9a5a30),
    ("milk", 0xf4f4f4), ("egg", 0xf0e0c0), ("bucket", 0xa0a0a0), ("dye", 0x9a50c0),
    ("cobble", 0x7a7a7a), ("gravel", 0x8a8078), ("sand", 0xe0d4a0), ("chest", 0xa0702a),
    ("book", 0x8a5a30), ("paper", 0xf0f0e0), ("barrel", 0x8a6030), ("boat", 0xb08850),
]

# Casos: rótulo, bloco e os valores do menu (os 13 do painel e a máscara de slots vazios).
# Estado (0), progresso (1), duração (2), energia (3), queima (4), capacidade (5), ritmo (6),
# uso (7), receita/fase/água (8), total (9), dano (10), módulo (11), informação (12), vazios (13).
CASES = """
press_trabalhando        veio:press            1 30 40 1500 0 2000 30 75 2 412 0 0 0 0 0=raw_iron*12 1=veio:iron_plate*5 t35
press_sem_material       veio:press            2 0 40 1800 0 2000 0 0 0 0 0 0 0 3
press_sem_energia        veio:press            5 12 40 0 0 2000 0 0 0 12 0 0 0 2
assembler                veio:assembler        1 20 60 900 0 2000 20 60 0 88 0 1 0 2
cutter                   veio:cutter           3 40 40 2000 0 2000 0 100 1 90 0 0 0 0
crusher                  veio:crusher          1 10 30 1200 0 2000 40 90 0 300 0 0 0 2
integrator               veio:integrator       1 50 100 1600 0 2000 12 80 0 30 0 0 0 4
alloy_furnace            veio:alloy_furnace    1 70 160 1600 0 2000 7 95 0 30 0 0 0 4
electronics              veio:electronics      2 0 120 1600 0 2000 0 0 0 0 0 0 0 7
electric_furnace         veio:electric_furnace 1 30 100 1600 0 2000 12 70 0 30 0 0 0 2
mill                     veio:mill             1 30 100 1600 0 2000 12 70 0 30 0 0 0 2
kitchen                  veio:kitchen          1 30 80 1500 0 2000 12 70 1 40 0 2 0 4 0=veio:flour*20 1=egg*6
fabricator               veio:fabricator       2 0 400 1600 0 2000 0 0 0 0 0 0 0 15
drill                    veio:drill            1 20 40 1500 0 2000 30 100 0 900 0 0 3 0 0=raw_copper*21
drill_mk2                veio:drill_mk2        4 0 20 2000 0 2000 0 0 0 0 0 0 0 1
drill_mk3                veio:drill_mk3        1 4 5 1800 0 2000 240 98 0 900 0 1 3 1
generator                veio:generator        1 600 1600 1500 600 2000 0 0 0 0 0 0 40 0 0=coal*14
generator_sem_carvao     veio:generator        6 0 1 200 0 2000 0 0 0 0 0 0 0 1
geothermal               veio:geothermal       1 300 3200 1800 300 2000 0 0 -3000 0 0 0 80 2
solar                    veio:solar            1 0 1 900 0 2000 0 0 0 0 0 0 12 0
solar_noite              veio:solar            7 0 1 100 0 2000 0 0 0 0 0 0 0 0
battery                  veio:battery          8 0 1 24000 0 30000 0 0 0 0 0 0 0 0
biomass_generator        veio:biomass_generator 1 100 400 1200 100 2000 0 0 0 0 0 0 30 0
steam_engine             veio:steam_engine     14 0 266 0 0 2000 0 0 0 0 30 0 0 1
water_wheel              veio:water_wheel      1 0 1 1200 0 2000 0 0 0 0 0 0 16 0
wind_turbine             veio:wind_turbine     1 0 1 1200 0 2000 0 0 0 0 0 0 16 0
hydro_turbine            veio:hydro_turbine    1 0 1 1200 0 2000 0 0 6 0 0 0 48 0
pump                     veio:pump             1 0 1 1200 0 2000 0 0 2400 0 0 0 20 0
sprinkler                veio:sprinkler        1 0 1 1200 0 2000 0 0 800 34 0 0 60 0
belt                     veio:belt             1 0 1 0 0 2000 150 0 0 0 0 0 0 0
splitter                 veio:splitter         1 0 1 0 0 2000 150 0 0 0 0 0 1 0
merger                   veio:merger           1 0 1 0 0 2000 150 0 0 0 0 0 0 0
belt_tunnel              veio:belt_tunnel      1 0 1 0 0 2000 150 0 0 0 0 0 0 0
cargo_lift               veio:cargo_lift       1 0 1 0 0 2000 150 0 0 0 0 0 0 0
inserter                 veio:inserter         1 0 1 900 0 2000 40 0 0 0 0 0 4 0
tesla                    veio:tesla            11 0 1 1500 0 2000 0 0 0 0 0 0 30 0
harvester                veio:harvester        1 0 1 1500 0 2000 12 0 0 0 0 0 0 1
planter                  veio:planter          1 0 1 1500 0 2000 0 0 0 30 0 0 0 0
grow_lamp                veio:grow_lamp        1 0 1 1500 0 2000 0 0 0 30 0 0 0 0
tree_farm                veio:tree_farm        1 20 100 1500 0 2000 0 0 0 6 0 0 0 14
ranch                    veio:ranch            1 20 100 1500 0 2000 0 0 0 6 0 0 5 28
pantry                   veio:pantry           1 42 60 0 0 2000 0 0 0 5 0 0 4 0
research_lab             veio:research_lab     1 20 100 1500 0 2000 0 0 0 0 0 0 0 0
contract_hub             veio:contract_hub     2 0 1 0 0 2000 0 0 0 0 0 0 0 15
contract_hub_projeto     veio:contract_hub     1 0 1 0 0 2000 0 0 5 120 0 0 40 15
factory_monitor          veio:factory_monitor  1 17268826 33800267 50454558 84869300 134279168 184631320 0 0 0 12 3 1 0
chunk_anchor             veio:chunk_anchor     1 0 1 1500 0 2000 0 0 0 0 0 0 0 0
farm_post                veio:farm_post        23 0 1 0 0 2000 0 0 0 12 0 0 5 12 0=bread*8 1=wheat*14
hauler_post              veio:hauler_post      24 0 1 0 0 2000 3 0 0 40 0 0 7 0
foreman_desk             veio:foreman_desk     25 0 1 0 0 2000 4 3 0 9 0 0 11 0
shepherd_post            veio:shepherd_post    17 0 1 0 0 2000 0 0 0 6 0 0 4 0
barracks                 veio:barracks         1 0 1 0 0 2000 0 0 0 6 0 0 4 0
field_marker             veio:field_marker     26 2 2 13 140 128 14 12 16 140 0 0 0 0
pen_marker               veio:pen_marker       26 2 2 13 140 128 14 12 16 140 0 0 0 0
barn                     veio:barn             27 0 1 0 0 2000 4 27 0 0 0 0 0 0 0=oak_log*64 1=veio:iron_plate*40 2=wheat*30 9=cobblestone*64
silo                     veio:silo             3 0 1 0 0 2000 18 18 0 0 0 0 0 0
export_dock              veio:export_dock      1 9 2400 0 0 2000 128 0 0 340 0 0 75 504 0=veio:gear*64 1=veio:gear*64 2=veio:circuit*20 t120
freight_station          veio:freight_station  1 6 2400 0 0 2000 64 0 0 -12 0 0 20 510 0=veio:steel_plate*64
rail_terminal            veio:rail_terminal    1 0 1 1500 0 2000 0 0 0 0 0 0 3 0
drone_port               veio:drone_port       1 0 1 1500 0 2000 0 0 1 0 0 0 3 0
caravan_station          veio:caravan_station  1 0 1 0 0 2000 16 0 0 0 0 0 3 0
cargo_station            veio:cargo_station    1 0 1 0 0 2000 0 0 0 0 0 0 0 0
unloading_station        veio:unloading_station 1 0 1 0 0 2000 0 0 0 0 0 0 0 0
control_center_vila      veio:control_center   1 786441 131073 196609 1 54 4 2687038 96 62 100 -1 0 255
control_center_atencao   veio:control_center   101 2 0 0 0 0 0 0 0 0 0 0 0 255
control_center_obras     veio:control_center   201 228 228 8192 8754 524288 8401182 8404992 20979712 29364224 3 180 2 255
control_center_pedidos   veio:control_center   301 180 66 37 82945 98699 211 1550 2 0 0 0 3 255
control_center_comercio  veio:control_center   9001 7 2 0 0 0 0 0 0 0 0 0 0 255
town_hall                veio:town_hall        1 2 41 12 80 72 31 340 120 10 34 2003 0 3
home                     veio:home             1 72 95 2 215 5 24 40 97 3 3 141630 0 31
house                    veio:house            29 0 1 0 0 2000 0 0 0 0 0 0 0 0
shop                     veio:shop             1 0 1 0 0 2000 0 0 0 0 0 0 0 0
well                     veio:well             1 0 1 0 0 2000 0 0 0 0 0 0 0 0
site_plano               veio:site             38 512 1271 0 0 0 0 0 2 380 0 0 12 0
site_bloqueado           veio:site             37 0 1271 0 0 0 3 33153153 1 0 0 0 0 0
factory_board            veio:factory_board    1 0 8 0 20 0 3 3 1 0 0 7 5 3
factory_board_aco        veio:factory_board    1 2 5 4 10 1 9 6 0 0 0 15 6 3
"""


def language(code):
    data = json.loads((ASSETS / f"lang/{code}.json").read_text(encoding="utf-8"))
    merged = dict(VANILLA.get("pt_br" if code == "pt_br" else "en_us", {}))
    if code != "pt_br":
        merged.update({k: v for k, v in VANILLA["pt_br"].items() if k not in merged})
    merged.update(data)
    return merged


def write_inputs(code, only):
    TARGET.mkdir(parents=True, exist_ok=True)
    lang = language(code)
    (TARGET / "preview_lang.txt").write_text(
        "\x1e".join(f"{k}\x1f{v}" for k, v in lang.items()), encoding="utf-8")
    lines = [line for line in CASES.strip().splitlines()
             if not only or line.split()[0] in only or line.split()[1].removeprefix("veio:") in only]
    (TARGET / "preview_cases.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")
    return lang


def run_preview():
    subprocess.run(["cargo", "test", "--offline", "-q", "-p", "veio-check", "preview", "--", "--ignored"],
                   cwd=ROOT / "native/offline-check", check=True)


def argb(value):
    return ((value >> 16) & 255, (value >> 8) & 255, value & 255, (value >> 24) & 255)


class Canvas:
    def __init__(self, width, height, margin):
        self.margin = margin
        size = (int((width + 2 * margin) * SCALE), int((height + 2 * margin) * SCALE))
        self.image = Image.new("RGBA", size, (24, 22, 30, 255))
        # Fundo escurecido do jogo atrás da tela, com um degradê leve.
        shade = ImageDraw.Draw(self.image)
        for row in range(size[1]):
            tone = 16 + int(14 * row / size[1])
            shade.line([(0, row), (size[0], row)], fill=(tone, tone - 2, tone + 6, 255))
        self.textures = {}

    def to_px(self, x, y):
        return ((x + self.margin) * SCALE, (y + self.margin) * SCALE)

    def fill(self, rect, color):
        x, y, w, h = rect
        if w <= 0 or h <= 0:
            return
        r, g, b, a = argb(color)
        if a == 0:
            return
        x0, y0 = self.to_px(x, y)
        x1, y1 = self.to_px(x + w, y + h)
        box = (round(x0), round(y0), round(x1), round(y1))
        if box[2] <= box[0] or box[3] <= box[1]:
            return
        layer = Image.new("RGBA", (box[2] - box[0], box[3] - box[1]), (r, g, b, a))
        self.image.alpha_composite(layer, (box[0], box[1]))

    def quad(self, points, color):
        r, g, b, a = argb(color)
        layer = Image.new("RGBA", self.image.size, (0, 0, 0, 0))
        ImageDraw.Draw(layer).polygon([self.to_px(x, y) for x, y in points], fill=(r, g, b, a))
        self.image.alpha_composite(layer)

    def glyphs(self, text, x, y, color):
        r, g, b = (color >> 16) & 255, (color >> 8) & 255, color & 255
        draw = ImageDraw.Draw(self.image)
        cursor = x
        for char in text:
            rows, above = ui_font.glyph(char)
            for offset, row_list in ((0, rows), (-len(above), above)):
                for row, line in enumerate(row_list):
                    for column, pixel in enumerate(line):
                        if pixel == "#":
                            px, py = self.to_px(cursor + column, y + row + offset)
                            draw.rectangle([px, py, px + SCALE - 1, py + SCALE - 1], fill=(r, g, b, 255))
            cursor += ui_font.advance(char)
        return cursor - x

    def text(self, text, at, color, shadow):
        x, y = at
        if shadow:
            dark = ((color >> 16 & 255) // 4 << 16) | ((color >> 8 & 255) // 4 << 8) | ((color & 255) // 4)
            self.glyphs(text, x + 1, y + 1, dark)
        self.glyphs(text, x, y, color)

    def texture(self, path):
        if path not in self.textures:
            self.textures[path] = Image.open(path).convert("RGBA") if path and path.exists() else None
        return self.textures[path]

    def sprite(self, name, src, dst):
        path = None
        if ":" in name:
            namespace, rest = name.split(":", 1)
            path = ROOT / f"content/assets/{namespace}/textures/{rest}.png"
        image = self.texture(path)
        x0, y0 = self.to_px(dst[0], dst[1])
        size = (max(1, round(dst[2] * SCALE)), max(1, round(dst[3] * SCALE)))
        if image is None:
            tile = Image.new("RGBA", size, (255, 0, 255, 255))
            ImageDraw.Draw(tile).rectangle([0, 0, size[0] // 2, size[1] // 2], fill=(0, 0, 0, 255))
        else:
            sx, sy, sw, sh = src
            tile = image.crop((round(sx), round(sy), round(sx + sw), round(sy + sh))).resize(size, Image.NEAREST)
        self.image.alpha_composite(tile, (round(x0), round(y0)))

    def item(self, item, at, size, lang):
        namespace, name = item.split(":", 1) if ":" in item else ("minecraft", item)
        path = None
        if namespace == "veio":
            for candidate in (f"item/{name}", f"block/{name}_front", f"block/{name}", f"block/{name}_top",
                              f"block/{name}_side", f"block/{name}_rail"):
                if (ASSETS / f"textures/{candidate}.png").exists():
                    path = ASSETS / f"textures/{candidate}.png"
                    break
        image = self.texture(path)
        x0, y0 = self.to_px(at[0], at[1])
        pixels = max(1, round(size * SCALE))
        if image is not None:
            frame = image.crop((0, 0, image.width, min(image.height, image.width)))
            self.image.alpha_composite(frame.resize((pixels, pixels), Image.NEAREST), (round(x0), round(y0)))
            return
        color = next((c for word, c in CHIP_COLORS if word in name), int(hashlib.md5(name.encode()).hexdigest()[:6], 16))
        r, g, b = color >> 16 & 255, color >> 8 & 255, color & 255
        chip = Image.new("RGBA", (pixels, pixels), (0, 0, 0, 0))
        inset = max(1, pixels // 8)
        ImageDraw.Draw(chip).rounded_rectangle([inset, inset, pixels - inset - 1, pixels - inset - 1],
                                               radius=pixels // 5, fill=(r, g, b, 255), outline=(30, 30, 30, 255))
        self.image.alpha_composite(chip, (round(x0), round(y0)))
        if size >= 12:
            label = (lang.get(f"item.minecraft.{name}") or lang.get(f"block.minecraft.{name}") or name)[:2]
            ink = 0x101010 if (r * 299 + g * 587 + b * 114) / 1000 > 128 else 0xf0f0f0
            self.glyphs(label, at[0] + size / 2 - sum(ui_font.advance(c) for c in label) / 2 + 0.5,
                        at[1] + size / 2 - 3.5, ink)

    def tooltip(self, lines, mouse):
        if not lines:
            return
        width = max(sum(ui_font.advance(c) for c in line) for line in lines)
        height = len(lines) * 10 + 1
        x, y = mouse[0] + 12, mouse[1] - 12
        self.fill([x - 3, y - 4, width + 6, height + 6], 0xf0100010)
        self.fill([x - 4, y - 3, width + 8, height + 4], 0xf0100010)
        self.fill([x - 3, y - 3, width + 6, 1], 0x505000ff)
        self.fill([x - 3, y + height, width + 6, 1], 0x5028007f)
        self.fill([x - 3, y - 3, 1, height + 4], 0x505000ff)
        self.fill([x + width + 2, y - 3, 1, height + 4], 0x505000ff)
        for index, line in enumerate(lines):
            self.text(line, [x, y + index * 10 + (2 if index else 0)], 0xffffff if index == 0 else 0xaaaaaa, True)


def stacks(line):
    """Itens nos slots de um caso: `slot=item*quantidade` (o jogo os desenha por cima da tela)."""
    found = {}
    for word in line.split()[2:]:
        if "=" in word:
            slot, item = word.split("=", 1)
            item, _, count = item.partition("*")
            found[int(slot)] = (item if ":" in item else f"minecraft:{item}", int(count or 1))
    return found


def render(record, lang, items=None):
    width, height = record["size"] or [194, 236]
    canvas = Canvas(width, height, 6)
    for op in record["ops"]:
        if "fill" in op:
            canvas.fill(op["fill"], op["argb"])
        elif "text" in op:
            canvas.text(op["text"], op["at"], op["rgb"], op.get("shadow", False))
        elif "sprite" in op:
            canvas.sprite(op["sprite"], op["src"], op["dst"])
        elif "item" in op:
            canvas.item(op["item"], op["at"], op["size"], lang)
        elif "quad" in op:
            canvas.quad(op["quad"], op["argb"])
    slots = record.get("slots") or []
    for index, (item, count) in (items or {}).items():
        if index < len(slots) and slots[index]:
            x, y = slots[index]
            canvas.item(item, [x, y], 16, lang)
            if count > 1:
                text = str(count)
                canvas.text(text, [x + 17 - sum(ui_font.advance(c) for c in text), y + 9], 0xffffff, True)
    canvas.tooltip(record.get("tooltip", []), record.get("mouse", [-100, -100]))
    return canvas.image


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--lang", default="pt_br", choices=["pt_br", "en_us"])
    parser.add_argument("--only", default="", help="rótulos ou blocos separados por vírgula")
    parser.add_argument("--out", type=Path, default=OUT)
    parser.add_argument("--no-run", action="store_true", help="só pinta o screens.jsonl existente")
    args = parser.parse_args()
    only = {name.strip() for name in args.only.split(",") if name.strip()}
    lang = write_inputs(args.lang, only)
    if not args.no_run:
        run_preview()
    args.out.mkdir(parents=True, exist_ok=True)
    cards = []
    items = {line.split()[0]: stacks(line) for line in CASES.strip().splitlines()}
    for line in (TARGET / "screens.jsonl").read_text(encoding="utf-8").splitlines():
        record = json.loads(line)
        image = render(record, lang, items.get(record["label"]))
        name = f"{record['label']}.png"
        image.save(args.out / name)
        cards.append((record["label"], record["block"], name))
    page = ["<!doctype html><meta charset='utf-8'><title>Telas do Veio</title>",
            "<style>body{background:#1b1a20;color:#ddd;font:14px sans-serif;margin:16px}"
            "div{display:inline-block;margin:8px;vertical-align:top}img{image-rendering:pixelated;width:388px}"
            "p{margin:4px 0}</style>"]
    for label, block, name in cards:
        page.append(f"<div><p>{html.escape(label)} <small>{html.escape(block)}</small></p><img src='{name}'></div>")
    (args.out / "index.html").write_text("\n".join(page), encoding="utf-8")
    print(f"{len(cards)} telas em {args.out}")


if __name__ == "__main__":
    main()
