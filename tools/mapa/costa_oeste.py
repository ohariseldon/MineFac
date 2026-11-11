"""Mapa 5, "Costa oeste": a grade de 9 blocos e o bloco central (C) em detalhe.

O mapa de 15.000 blocos se divide em 3 x 3 blocos de 5.000 x 5.000. Os biomas do zoom usam os
mesmos cortes de clima do mapa inteiro (`preview_maps.CUTS`): o recorte mostra os mesmos biomas, só
com mais detalhe de relevo e de borda.

Uso: python3 tools/mapa/costa_oeste.py [pasta de saída]  -> grade.png, bloco_C.png
"""
import math
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

sys.path.insert(0, str(Path(__file__).parent))
import preview_maps as pm  # noqa: E402

CELL = 5000
# (código, nome, coluna, linha); coluna 0 = oeste, linha 0 = norte
GRID = [("NO", "Noroeste", 0, 0), ("N", "Norte", 1, 0), ("NE", "Nordeste", 2, 0),
        ("O", "Oeste", 0, 1), ("C", "Centro", 1, 1), ("L", "Leste", 2, 1),
        ("SO", "Sudoeste", 0, 2), ("S", "Sul", 1, 2), ("SE", "Sudeste", 2, 2)]
SUMMARY = {
    "NO": "mar frio", "N": "neve, taiga e o rio do norte", "NE": "a Muralha de Pedra gelada",
    "O": "o Mar do Oeste e a ilha dos cogumelos", "C": "a vila, o Morro da Mina e a Serra do Veio",
    "L": "a Muralha de Pedra e os passos para o interior", "SO": "mar morno e quente",
    "S": "litoral quente: selva, savana e deserto", "SE": "o interior seco: deserto e terras áridas",
}


def bounds(col, row):
    x0, z0 = -pm.HALF + col * CELL, -pm.HALF + row * CELL
    return x0, z0, x0 + CELL, z0 + CELL


def label(d, xy, text, size=15, color=(255, 255, 255), anchor="mm"):
    d.text(xy, text, fill=color, font=pm.font(size), anchor=anchor, stroke_width=3, stroke_fill=(0, 0, 0))


def grid_image(out_dir, overview):
    """O mapa 5 inteiro com a grade dos 9 blocos; o central em destaque."""
    img = Image.fromarray(overview)
    d = ImageDraw.Draw(img, "RGBA")
    for code, name, col, row in GRID:
        x0, z0, x1, z1 = bounds(col, row)
        a0, b0 = pm.to_px(x0, z0); a1, b1 = pm.to_px(x1, z1)
        d.rectangle([a0, b0, a1 - 1, b1 - 1], outline=(255, 255, 255, 220), width=2)
        if code == "C":
            d.rectangle([a0 + 2, b0 + 2, a1 - 3, b1 - 3], outline=(255, 215, 0), width=5)
        label(d, (a0 + 12, b0 + 12), code, 30, (255, 215, 0) if code == "C" else (255, 255, 255), "la")
        label(d, ((a0 + a1) / 2, b1 - 22), SUMMARY[code], 13)
    cx, cz = pm.to_px(0, 0)
    d.regular_polygon((cx, cz, 9), 5, fill=(255, 215, 0), outline=(0, 0, 0))
    panel = Image.new("RGB", (pm.N + 440, pm.N + 90), (24, 26, 30))
    panel.paste(img, (0, 90))
    pd = ImageDraw.Draw(panel)
    pd.text((16, 14), "Mapa 5 · Costa oeste · a grade dos 9 blocos", fill=(255, 255, 255), font=pm.font(28))
    pd.text((16, 54), "Cada bloco tem 5.000 x 5.000 blocos. O primeiro a detalhar é o C, onde está a vila.",
            fill=(200, 205, 215), font=pm.font(15))
    y = 110
    for code, name, col, row in GRID:
        x0, z0, x1, z1 = bounds(col, row)
        pd.text((pm.N + 20, y), f"{code} · {name}", fill=(255, 215, 0) if code == "C" else (255, 255, 255), font=pm.font(17))
        pd.text((pm.N + 20, y + 22), f"x {x0} a {x1}, z {z0} a {z1}", fill=(170, 175, 185), font=pm.font(13))
        pd.text((pm.N + 20, y + 40), SUMMARY[code], fill=(200, 205, 215), font=pm.font(13))
        y += 72
    pd.text((pm.N + 20, y + 10), "Situação: C em detalhe; os outros 8\naguardam (só a planta geral).",
            fill=(170, 175, 185), font=pm.font(13))
    panel.save(out_dir / "grade.png")


# ---------- o bloco C ----------

PLACES = [  # (nome, x, z, tamanho)
    ("Planície do Veio", 110, 175, 14), ("Morro da Mina", -320, -560, 16), ("Serra do Veio", 1550, -1820, 20),
    ("Rio Veio", 1300, -960, 15), ("Passo do Rio", 420, -560, 13), ("Mata Escura", -1250, 180, 17),
    ("Pinhal do Norte", -1500, -1500, 18), ("Bosque das Bétulas", 1250, 250, 16), ("Bosques do Sul", 0, 1650, 17),
    ("Brejo do Rio", -1350, 830, 15), ("Foz", -2330, 1180, 14), ("Mar do Oeste", -2150, 1950, 13),
    ("Taiga Velha", 1500, 900, 15), ("Fenda da Serra", 2050, -1150, 13),
]
# Recursos garantidos perto da vila (o subsolo é o do Minecraft; estes são reservados pelo plano)
RESOURCES = [
    ("cobre", 330, -160, (215, 120, 70), "Afloramento de cobre: a primeira extratora (etapa 5)"),
    ("ferro", 120, -470, (200, 190, 180), "Ferro do Morro: a mina do avô, reaberta"),
    ("carvão", -300, -170, (40, 40, 40), "Carvão da Mata: veio raso junto da carvoaria"),
    ("argila", 250, 360, (150, 165, 190), "Barreiro: argila na curva do rio (Dona Zefa)"),
    ("argila", -2250, 1050, (150, 165, 190), "Argila da foz"),
    ("pedra", -470, -470, (150, 150, 150), "Pedreira do Morro: pedra para a fonte e a prefeitura"),
]
ROADS = [  # (nome, pontos, cor)
    ("Estrada da Mina", [(0, -30), (-20, -200), (-40, -300), (100, -440)], (210, 175, 120)),
    ("Estrada velha do Rio", [(60, 60), (200, 300), (100, 600), (-300, 820), (-1000, 940), (-1700, 1000), (-2420, 1010)], (210, 175, 120)),
    ("Trilha da Serra", [(120, -40), (420, -540), (1000, -900), (1500, -1300)], (190, 160, 110)),
    ("Trilha da Mata", [(-30, 10), (-140, -60), (-300, -170)], (190, 160, 110)),
]
ZONES = [  # (nome, polígono, cor) — onde a vila cresce depois
    ("Distrito industrial (cap. 1 a 5)", [(300, -380), (850, -420), (900, 150), (330, 180)], (255, 170, 60)),
    ("Expansão da cidade (cap. 4 e 6)", [(-450, 420), (180, 470), (420, 950), (-380, 1050)], (120, 200, 255)),
    ("Lavouras e pastos", [(-420, 110), (-60, 110), (-40, 400), (-430, 390)], (230, 220, 90)),
]
POINTS = [  # marcos com número no mapa
    (1, -40, -300, "Mina do avô (fechada)"),
    (2, -2440, 990, "Cais velho: por onde o minério descia o rio (o mundo lá fora, cap. 7)"),
    (3, 78, -22, "Terreno da oficina (terraço da margem)"),
    (4, 175, -60, "Lugar da roda d'água"),
    (5, 1900, -1600, "Escuridão profunda sob a serra (cidade ancestral, longe da vila)"),
    (6, -140, -60, "Carvoaria do Seu Lino, na borda da mata"),
]


def block_c(out_dir):
    pm.REPLAY = True
    pm.set_view(-2500, -2500, 5000, 1250)
    concept = pm.concept_5()
    f = pm.prepare(concept, 5, detail=True)
    rgb = pm.shaded(f["cls"], f["height"], 2.2)
    # curvas de nível suaves
    level = np.floor(f["height"] / 6)
    edge = (np.diff(level, axis=0, prepend=level[:1]) != 0) | (np.diff(level, axis=1, prepend=level[:, :1]) != 0)
    rgb[edge & (f["C"] >= -0.12)] = (rgb[edge & (f["C"] >= -0.12)] * 0.82).astype(np.uint8)
    img = Image.fromarray(rgb)
    d = ImageDraw.Draw(img, "RGBA")
    P = pm.to_px
    # quadriculado de 500 blocos
    for k in range(-2000, 2500, 500):
        a, _ = P(k, 0); _, b = P(0, k)
        d.line([(a, 0), (a, pm.N)], fill=(255, 255, 255, 45)); d.line([(0, b), (pm.N, b)], fill=(255, 255, 255, 45))
        d.text((a + 3, 3), f"x {k}", fill=(255, 255, 255, 160), font=pm.font(11))
        d.text((3, b + 3), f"z {k}", fill=(255, 255, 255, 160), font=pm.font(11))
    # zonas de crescimento
    for name, poly, color in ZONES:
        pts = [P(x, z) for x, z in poly]
        d.polygon(pts, fill=color + (40,), outline=color + (230,), width=2)
        cx = sum(p[0] for p in pts) / len(pts); cz = sum(p[1] for p in pts) / len(pts)
        label(d, (cx, cz), name, 12, color)
    # estradas
    for name, pts, color in ROADS:
        d.line([P(x, z) for x, z in pts], fill=color + (255,), width=3, joint="curve")
    # zona protegida (sem ravinas nem bocas de caverna)
    cx, cz = P(0, 0); r = 600 / pm.BLOCK
    for k in range(0, 360, 8):
        a0, a1 = math.radians(k), math.radians(k + 4)
        d.line([(cx + r * math.cos(a0), cz + r * math.sin(a0)), (cx + r * math.cos(a1), cz + r * math.sin(a1))], fill=(255, 255, 255, 230), width=2)
    # recursos
    for kind, x, z, color, _ in RESOURCES:
        a, b = P(x, z)
        d.polygon([(a, b - 9), (a + 8, b), (a, b + 9), (a - 8, b)], fill=color, outline=(255, 255, 255), width=2)
        label(d, (a + 12, b), kind, 12, (255, 255, 255), "lm")
    # marcos numerados
    for n, x, z, _ in POINTS:
        a, b = P(x, z)
        d.ellipse([a - 10, b - 10, a + 10, b + 10], fill=(30, 30, 30), outline=(255, 215, 0), width=2)
        d.text((a, b), str(n), fill=(255, 215, 0), font=pm.font(13), anchor="mm")
    # a vila
    d.regular_polygon((cx, cz, 12), 5, fill=(255, 215, 0), outline=(0, 0, 0))
    label(d, (cx, cz - 16), "Vila", 18, (255, 235, 120), "mb")
    for name, x, z, size in PLACES:
        label(d, P(x, z), name, size)
    # quadro
    W = pm.N
    panel = Image.new("RGB", (W + 470, W + 90), (24, 26, 30))
    panel.paste(img, (0, 90))
    pd = ImageDraw.Draw(panel)
    pd.text((16, 14), "Mapa 5 · bloco C (Centro) · x -2.500 a 2.500, z -2.500 a 2.500", fill=(255, 255, 255), font=pm.font(26))
    pd.text((16, 54), "A vila numa planície entre o Morro da Mina e o rio; a Serra do Veio a nordeste; o rio desce até a foz, no canto sudoeste.",
            fill=(200, 205, 215), font=pm.font(14))
    pd.line([(24, W + 70), (24 + 500 / pm.BLOCK, W + 70)], fill=(255, 255, 255), width=4)
    pd.text((24, W + 74), "500 blocos", fill=(255, 255, 255), font=pm.font(12))
    x0, y = W + 20, 104
    pd.text((x0, y), "Biomas no bloco (km²)", fill=(255, 255, 255), font=pm.font(17)); y += 26
    counts = np.bincount(f["cls"].ravel(), minlength=len(pm.BIOMES))
    shown = sorted(((counts[i] * pm.BLOCK ** 2 / 1e6, i) for i in range(len(counts)) if counts[i]), reverse=True)
    for area, i in shown:
        pd.rectangle([x0, y, x0 + 14, y + 14], fill=pm.BIOMES[i][1], outline=(200, 200, 200))
        pd.text((x0 + 22, y - 1), f"{pm.BIOMES[i][2]}  {area:.2f}", fill=(210, 210, 210), font=pm.font(12)); y += 18
    y += 10
    pd.text((x0, y), "Marcos", fill=(255, 255, 255), font=pm.font(17)); y += 24
    for n, _, _, text in POINTS:
        pd.text((x0, y), f"{n}. {text}", fill=(210, 210, 210), font=pm.font(12)); y += 18
    y += 8
    pd.text((x0, y), "Recursos garantidos (losangos)", fill=(255, 255, 255), font=pm.font(17)); y += 24
    for _, _, _, color, text in RESOURCES:
        pd.polygon([(x0 + 7, y), (x0 + 14, y + 7), (x0 + 7, y + 14), (x0, y + 7)], fill=color)
        pd.text((x0 + 22, y - 1), text, fill=(210, 210, 210), font=pm.font(12)); y += 18
    y += 8
    pd.text((x0, y), "Tracejado branco: 600 blocos sem ravinas\nnem bocas de caverna. Estradas de terra\nem bege; zonas coloridas: onde a vila\ncresce nos próximos capítulos.",
            fill=(170, 175, 185), font=pm.font(12))
    panel.save(out_dir / "bloco_C.png")
    return shown


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("mapa_previas")
    out_dir.mkdir(parents=True, exist_ok=True)
    pm.REPLAY = False
    pm.set_view(-pm.HALF, -pm.HALF, 2 * pm.HALF, 1000)
    f = pm.prepare(pm.concept_5(), 5)
    grid_image(out_dir, pm.shaded(f["cls"], f["height"]))
    shown = block_c(out_dir)
    print("bloco C:", ", ".join(f"{pm.BIOMES[i][2]} {a:.2f}" for a, i in shown))


if __name__ == "__main__":
    main()
