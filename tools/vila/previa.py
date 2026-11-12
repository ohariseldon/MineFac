"""Prévias isométricas dos prédios da vila (por fora e em corte, com o interior) e da vila inteira.

Uso: python3 tools/vila/previa.py <pasta de saída>
Gera `predios_1.png`, `predios_2.png`... (cada prédio por fora e por dentro), `vila_estagio1_3d.png` e
`vila_estagio2_3d.png`.
"""
import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import predios as P                                # noqa: E402
import build_village as B                          # noqa: E402
from modelo import base_id, props                  # noqa: E402

FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
FONT_B = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"

# Cores por palavra no nome do bloco (a primeira que casa vale)
PALETTE = [
    ("water", (64, 118, 212)), ("leaves", (70, 125, 45)), ("lantern", (255, 200, 90)), ("campfire", (120, 90, 60)), ("glass_pane", (190, 225, 240)),
    ("bookshelf", (140, 100, 60)), ("_bed", None), ("_carpet", None), ("_wool", None), ("_banner", None),
    ("dark_oak", (66, 43, 21)), ("spruce", (114, 84, 48)), ("birch", (215, 200, 140)), ("jungle", (160, 115, 80)),
    ("acacia", (170, 92, 50)), ("cherry", (226, 178, 172)), ("stripped_oak", (178, 144, 86)), ("oak_log", (102, 81, 50)),
    ("oak", (162, 130, 78)), ("mud_brick", (137, 104, 79)), ("packed_mud", (142, 106, 79)),
    ("deepslate", (70, 70, 76)), ("mossy", (100, 120, 80)), ("cracked", (118, 118, 118)), ("chiseled_stone", (120, 120, 120)),
    ("stone_brick", (128, 128, 128)), ("brick", (150, 74, 60)), ("cobble", (120, 120, 120)), ("andesite", (136, 136, 138)), ("granite", (160, 110, 90)),
    ("calcite", (225, 226, 222)), ("white_terracotta", (210, 178, 161)), ("terracotta", (152, 94, 67)),
    ("quartz", (236, 230, 222)), ("gravel", (130, 124, 122)), ("dirt_path", (148, 122, 66)), ("coarse_dirt", (119, 85, 59)),
    ("farmland", (82, 52, 30)), ("dirt", (134, 96, 67)), ("grass_block", (98, 148, 60)), ("hay", (190, 160, 40)),
    ("wheat", (200, 180, 70)), ("carrots", (230, 130, 30)), ("potatoes", (120, 160, 60)), ("short_grass", (90, 140, 55)),
    ("tall_grass", (90, 140, 55)), ("fern", (80, 130, 50)), ("moss", (90, 120, 50)), ("vine", (60, 110, 40)),
    ("rose_bush", (180, 40, 50)), ("lilac", (190, 140, 200)), ("peony", (230, 170, 200)), ("cornflower", (70, 100, 220)),
    ("azure_bluet", (220, 230, 240)), ("allium", (180, 110, 220)), ("red_tulip", (220, 50, 40)), ("pink_tulip", (240, 160, 190)),
    ("orange_tulip", (240, 140, 40)), ("blue_orchid", (40, 160, 220)), ("cactus", (80, 130, 50)), ("dandelion", (240, 220, 50)), ("oxeye", (240, 240, 230)), ("poppy", (200, 40, 40)),
    ("potted", (150, 80, 60)), ("flower_pot", (150, 80, 60)), ("decorated_pot", (160, 90, 60)), ("clay", (160, 166, 179)),
    ("scaffolding", (190, 160, 90)), ("cobweb", (230, 230, 230)), ("anvil", (60, 60, 64)), ("furnace", (100, 100, 100)),
    ("smoker", (90, 80, 70)), ("barrel", (120, 88, 50)), ("chest", (160, 115, 50)), ("crafting", (140, 100, 60)),
    ("smithing", (60, 60, 70)), ("grindstone", (130, 130, 130)), ("loom", (170, 140, 100)), ("lectern", (150, 110, 60)),
    ("cartography", (140, 110, 80)), ("stonecutter", (120, 120, 120)), ("cauldron", (60, 60, 64)), ("composter", (110, 80, 40)),
    ("bell", (230, 190, 50)), ("candle", (230, 220, 180)), ("iron_ore", (150, 130, 120)), ("raw_iron", (170, 140, 110)),
    ("coal", (30, 30, 30)), ("dead_bush", (130, 100, 60)), ("ladder", (130, 100, 60)), ("cactus", (80, 130, 50)),
]
COLORS = {"white": (233, 236, 236), "light_gray": (160, 160, 154), "gray": (70, 74, 78), "black": (25, 25, 30),
          "brown": (114, 72, 40), "red": (160, 40, 35), "orange": (240, 118, 20), "yellow": (248, 197, 40),
          "lime": (112, 185, 25), "green": (84, 109, 27), "cyan": (21, 137, 145), "light_blue": (58, 175, 217),
          "blue": (53, 57, 157), "purple": (121, 42, 172), "magenta": (189, 68, 179), "pink": (237, 141, 172)}


def color(state):
    b = base_id(state).split(":")[1]
    for word, rgb in PALETTE:
        if word in b:
            if rgb is None:
                for c in sorted(COLORS, key=len, reverse=True):
                    if b.startswith(c + "_"):
                        return COLORS[c]
                return (200, 200, 200)
            return rgb
    return (170, 170, 170)


def thin(state):
    """Blocos que ocupam pouco: desenhados mais baixos ou finos."""
    b = base_id(state)
    if b.endswith(("_carpet", "moss_carpet", "farmland", "_path")):
        return 0.12
    if b.endswith(("_slab",)) and props(state).get("type") == "bottom":
        return 0.5
    if b.endswith("_slab") and props(state).get("type") == "top":
        return -0.5
    if b.endswith(("_bed",)):
        return 0.55
    return 1.0


def shade(rgb, k):
    return tuple(max(0, min(255, int(c * k))) for c in rgb)


class Iso:
    def __init__(self, scale):
        self.a = scale            # meia largura do losango
        self.b = scale / 2        # meia altura do losango
        self.c = scale * 1.0      # altura de um bloco

    def point(self, x, y, z):
        return ((x - z) * self.a, (x + z) * self.b - y * self.c)

    def cube(self, draw, x, y, z, rgb, ox, oy, h=1.0, outline=True):
        bottom = y if h > 0 else y + 1 + h
        top = y + h if h > 0 else y + 1
        P0 = lambda X, Y, Z: (lambda p: (p[0] + ox, p[1] + oy))(self.point(X, Y, Z))
        topf = [P0(x, top, z), P0(x + 1, top, z), P0(x + 1, top, z + 1), P0(x, top, z + 1)]
        south = [P0(x, top, z + 1), P0(x + 1, top, z + 1), P0(x + 1, bottom, z + 1), P0(x, bottom, z + 1)]
        east = [P0(x + 1, top, z), P0(x + 1, top, z + 1), P0(x + 1, bottom, z + 1), P0(x + 1, bottom, z)]
        line = shade(rgb, 0.55) if outline else None
        draw.polygon(south, fill=shade(rgb, 0.78), outline=line)
        draw.polygon(east, fill=shade(rgb, 0.62), outline=line)
        draw.polygon(topf, fill=shade(rgb, 1.08), outline=line)


def render_blocks(blocks, scale, margin=20, ground=None, label=None):
    """blocks: {(x, y, z): estado}. Devolve uma imagem."""
    iso = Iso(scale)
    if not blocks:
        return Image.new("RGB", (100, 100), (245, 242, 235))
    xs = [p[0] for p in blocks]; ys = [p[1] for p in blocks]; zs = [p[2] for p in blocks]
    corners = [iso.point(x, y, z) for x in (min(xs), max(xs) + 1) for y in (min(ys), max(ys) + 1) for z in (min(zs), max(zs) + 1)]
    minx = min(c[0] for c in corners); maxx = max(c[0] for c in corners)
    miny = min(c[1] for c in corners); maxy = max(c[1] for c in corners)
    top_pad = 40 if label else 0
    w, h = int(maxx - minx + 2 * margin), int(maxy - miny + 2 * margin + top_pad)
    if label:
        w = max(w, int(ImageFont.truetype(FONT_B, 18).getlength(label)) + 2 * margin)
    im = Image.new("RGB", (w, h), (245, 242, 235))
    d = ImageDraw.Draw(im)
    ox, oy = margin - minx, margin - miny + top_pad
    order = sorted(blocks, key=lambda p: (p[0] + p[2], p[1], p[0]))
    for (x, y, z) in order:
        s = blocks[(x, y, z)]
        if base_id(s) == "minecraft:air":
            continue
        rgb = color(s)
        hh = thin(s)
        if base_id(s).endswith(("_fence", "_wall", "glass_pane", "ladder", "_door", "_trapdoor", "lantern", "potted", "flower_pot",
                                "short_grass", "tall_grass", "fern", "dandelion", "oxeye_daisy", "rose_bush", "wheat", "carrots",
                                "potatoes", "candle", "vine", "cobweb", "dead_bush", "bell", "_banner", "cornflower", "azure_bluet", "allium",
                                "tulip", "lilac", "peony", "rose_bush", "poppy")) \
                or "potted_" in s:
            # peças finas: um cubo menor no meio
            pad = 0.32
            P0 = lambda X, Y, Z: (lambda p: (p[0] + ox, p[1] + oy))(iso.point(X, Y, Z))
            x0, x1, z0, z1 = x + pad, x + 1 - pad, z + pad, z + 1 - pad
            if base_id(s).endswith(("glass_pane", "_door", "_trapdoor", "_banner")):
                x0, x1, z0, z1 = x + 0.05, x + 0.95, z + 0.05, z + 0.95
            topy = y + (0.9 if not base_id(s).endswith(("wheat", "carrots", "potatoes", "short_grass")) else 0.5)
            topf = [P0(x0, topy, z0), P0(x1, topy, z0), P0(x1, topy, z1), P0(x0, topy, z1)]
            south = [P0(x0, topy, z1), P0(x1, topy, z1), P0(x1, y, z1), P0(x0, y, z1)]
            east = [P0(x1, topy, z0), P0(x1, topy, z1), P0(x1, y, z1), P0(x1, y, z0)]
            d.polygon(south, fill=shade(rgb, 0.8)); d.polygon(east, fill=shade(rgb, 0.65)); d.polygon(topf, fill=shade(rgb, 1.05))
            continue
        iso.cube(d, x, y, z, rgb, ox, oy, h=hh, outline=scale >= 6)
    if label:
        d.text((margin, 8), label, fill=(40, 40, 40), font=ImageFont.truetype(FONT_B, 18))
    return im


def cutaway(model, top=3):
    """O interior: tira tudo acima de `top` e as paredes sul e leste (fica a fiada de baixo)."""
    a0, b0, a1, b1 = model.core
    out = {}
    for (x, y, z), s in model.blocks.items():
        if y > top:
            continue
        if y >= 2 and (x >= a1 or z >= b1):
            continue
        out[(x, y, z)] = s
    return out


def sheet(models, out, per_row=2, scale=9):
    tiles = []
    for m in models:
        ext = render_blocks(m.connected().blocks, scale, label=m.title)
        inside = render_blocks(cutaway(m), scale, label="por dentro")
        upper = None
        if any(y >= 5 and ("_bed" in s or "lectern" in s or "chest" in s or "_fence" in s and y == 5)
               for (x, y, z), s in m.blocks.items() if m.core[0] < x < m.core[2] and m.core[1] < z < m.core[3]):
            upper = render_blocks(cutaway(m, top=7), scale, label="andar de cima")
        parts = [ext, inside] + ([upper] if upper else [])
        w = sum(p.width for p in parts) + 10 * (len(parts) - 1)
        h = max(p.height for p in parts)
        tile = Image.new("RGB", (w, h), (245, 242, 235))
        x = 0
        for p in parts:
            tile.paste(p, (x, h - p.height)); x += p.width + 10
        tiles.append(tile)
    rows = [tiles[i:i + per_row] for i in range(0, len(tiles), per_row)]
    W = max(sum(t.width for t in r) + 30 * (len(r) - 1) for r in rows) + 40
    H = sum(max(t.height for t in r) for r in rows) + 30 * len(rows) + 40
    im = Image.new("RGB", (W, H), (245, 242, 235))
    y = 20
    for r in rows:
        x = 20
        for t in r:
            im.paste(t, (x, y)); x += t.width + 30
        y += max(t.height for t in r) + 30
    im.save(out)
    return out


def village(stage, out, scale=3):
    """A vila inteira no estágio: chão de grama, ruas do padrão do estágio e os prédios."""
    placed = B.layout(stage, overlays=True)
    placed.sort(key=lambda p: (p.piece.overlay_of is not None, p.piece.stage))
    blocks = {}
    X0, Z0, X1, Z1 = -118, -70, 100, 104
    for x in range(X0, X1):
        for z in range(Z0, Z1):
            blocks[(x, -1, z)] = "minecraft:grass_block"
    cells = B.road_cells(stage)
    blocked = [p.area for p in placed] + [B.founder_rect()]
    for (x, z), edge in cells.items():
        if not (X0 <= x < X1 and Z0 <= z < Z1):
            continue
        if any(r[0] <= x <= r[2] and r[1] <= z <= r[3] for r in blocked):
            continue
        b = B.street_block(stage, x, z, edge)
        if b:
            blocks[(x, -1, z)] = b
    for p in placed:
        for (x, y, z), s in p.model.connected().blocks.items():
            key = (p.origin[0] + x, y - 1, p.origin[1] + z)
            if base_id(s) == "minecraft:air":
                blocks.pop(key, None)
                continue
            blocks[key] = s
    # o verde: árvore (tronco e copa) e canteiro (flores soltas)
    _, trees, patches = B.greenery()
    for x, z in trees:
        if not (X0 <= x < X1 and Z0 <= z < Z1):
            continue
        birch = B.h01("arvore", x, z) > 0.5
        for y in range(0, 4):
            blocks[(x, y, z)] = "minecraft:birch_log" if birch else "minecraft:oak_log"
        for dx in range(-2, 3):
            for dz in range(-2, 3):
                for y in (3, 4, 5):
                    if abs(dx) + abs(dz) <= (3 if y < 5 else 1) and (dx, dz) != (0, 0) or y == 5 and (dx, dz) == (0, 0):
                        blocks.setdefault((x + dx, y, z + dz), "minecraft:oak_leaves")
    for x, z in patches:
        for k in range(6):
            fx, fz = x + int(B.h01("fx", x, z, k) * 5) - 2, z + int(B.h01("fz", x, z, k) * 5) - 2
            blocks.setdefault((fx, 0, fz), "minecraft:" + ["poppy", "dandelion", "cornflower", "oxeye_daisy", "short_grass"][k % 5])
    # a casa do fundador como um bloco simples (o modelo dela é de build_house.py)
    fx0, fz0, fx1, fz1 = B.founder_rect()
    for x in range(fx0 + 1, fx1):
        for z in range(fz0 + 1, fz1):
            blocks[(x, -1, z)] = "minecraft:spruce_planks"
            if x in (fx0 + 1, fx1 - 1) or z in (fz0 + 1, fz1 - 1):
                for y in range(0, 3):
                    blocks[(x, y, z)] = "minecraft:oak_planks"
    for k in range(5):
        for x in range(fx0, fx1 + 1):
            for z in (fz0 + k, fz1 - k):
                blocks[(x, 3 + k, z)] = "minecraft:spruce_stairs"
    im = render_blocks(blocks, scale, margin=30, label=f"Vila do Veio · estágio {stage}")
    im.save(out)
    return out


def progression(factories, out, scale=9):
    """O mesmo prédio no estágio 1 (desgaste), 2 e 4 (acabamentos), de frente."""
    rows = []
    for i, f in enumerate(factories):
        base, worn = P.worn(f, i)
        stages = [("estágio 1", dict(base.blocks))]
        two = dict(base.blocks)
        for k, v in P.finish(base, 2, i, worn).blocks.items():
            two[k] = v
        stages.append(("estágio 2", two))
        four = dict(two)
        for k, v in P.finish(base, 4, i).blocks.items():
            four[k] = v
        stages.append(("estágio 4", four))
        imgs = []
        for label, blocks in stages:
            m = type(base)(base.name, base.width, base.depth, base.title)
            m.blocks = {k: v for k, v in blocks.items() if base_id(v) != "minecraft:air"}
            m.core = base.core
            # visto de frente: gira para a porta ficar à esquerda-baixo da imagem
            imgs.append(render_blocks(m.rotated(0).connected().blocks, scale, label=f"{base.title} · {label}"))
        rows.append(imgs)
    W = max(sum(i.width for i in r) + 20 * (len(r) - 1) for r in rows) + 40
    H = sum(max(i.height for i in r) for r in rows) + 20 * len(rows) + 40
    im = Image.new("RGB", (W, H), (245, 242, 235))
    y = 20
    for r in rows:
        x = 20
        for i in r:
            im.paste(i, (x, y)); x += i.width + 20
        y += max(i.height for i in r) + 20
    im.save(out)
    return out


def main():
    out = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
    out.mkdir(parents=True, exist_ok=True)
    houses = [P.worn(f, i)[0] for i, f in enumerate(P.HOUSES)]
    npcs = [P.worn(f, 100 + i)[0] for i, f in enumerate([P.ferreiro, P.lavradora, P.vendinha, P.olaria, P.carvoaria, P.vaqueiro])]
    others = [P.em_obra(P.casa_enxaimel), P.ruina(), P.lote(), P.poco(1), P.casa_reformada(), P.casa_enxaimel_nova()]
    halls = [P.prefeitura(3), P.prefeitura(4), P.prefeitura(6)]
    news = [f() for f in P.NEW_HOUSE_MODELS]
    files = [
        sheet(houses, out / "predios_1_casas.png"),
        sheet(npcs, out / "predios_2_moradores.png"),
        sheet(others + news, out / "predios_3_obras_e_novas.png"),
        sheet(halls, out / "predios_4_prefeitura.png", per_row=1),
        sheet([P.praca(1), P.praca(2)], out / "predios_5_praca.png", per_row=1, scale=8),
        progression([P.casa_pedra, P.casa_taipa, P.vendinha], out / "predios_6_acabamento.png"),
        village(1, out / "vila_estagio1_3d.png"),
        village(2, out / "vila_estagio2_3d.png"),
    ]
    for f in files:
        print(f)


if __name__ == "__main__":
    main()
