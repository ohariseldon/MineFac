"""A vila no estágio 1 ("a chegada": a vila em declínio), vista de cima bloco a bloco.

Cada bloco é um quadrado de 6 x 6 pixels com a cor do bloco do Minecraft que vai estar ali, a
sombra do sol da tarde (vindo do noroeste) e o volume das casas e das copas. A área vai da borda da
mata, a oeste, até a outra margem do Rio Veio (x -130 a 140, z -95 a 105, em blocos a partir do
centro da praça).

Terraços: a praça no nível 64; a leste, o terraço do meio (62) e a margem do rio (60), com escadas
de pedra onde as ruas descem; a oeste, o chão sobe 1 bloco até a mata. Ao norte começa a encosta do
Morro da Mina.

Uso: python3 tools/mapa/vila_estagio1.py [pasta de saída]  -> vila_estagio1.png
"""
import math
import random
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

X0, X1, Z0, Z1 = -130, 140, -95, 105
WB, HB = X1 - X0, Z1 - Z0
S = 6
FONT_DIR = Path("/usr/share/fonts/truetype/dejavu")
STEP_MID, STEP_BANK, STEP_WEST = 34, 74, -70     # quebras dos terraços (x médio)


def font(size, bold=False):
    path = FONT_DIR / ("DejaVuSans-Bold.ttf" if bold else "DejaVuSans.ttf")
    return ImageFont.truetype(str(path), size) if path.exists() else ImageFont.load_default()


COLORS = {
    "grass": (106, 160, 68), "grass_dry": (128, 160, 72), "grass_dark": (86, 138, 56), "tallgrass": (80, 138, 52),
    "fern": (70, 120, 50), "bush": (60, 110, 42), "flower_y": (232, 212, 64), "flower_r": (204, 58, 48),
    "flower_w": (238, 238, 232), "flower_b": (92, 122, 222), "flower_p": (190, 110, 200), "dirt": (128, 92, 60),
    "path": (154, 128, 82), "coarse": (112, 82, 56), "cobble": (124, 124, 124), "mossy": (100, 118, 92),
    "stonebrick": (140, 140, 140), "cracked": (114, 112, 110), "gravel": (140, 130, 124), "sand": (216, 206, 158),
    "water": (54, 92, 198), "water_sh": (74, 124, 214), "lily": (40, 120, 40), "cane": (140, 190, 90),
    "farmland": (102, 68, 40), "wheat": (210, 186, 76), "wheat_young": (120, 168, 62), "carrot": (90, 150, 50),
    "oak_roof": (166, 130, 80), "spruce_roof": (106, 80, 50), "dark_roof": (74, 54, 36), "deepslate": (80, 80, 86),
    "terracotta": (154, 96, 70), "brick": (152, 80, 64), "hay": (206, 174, 64), "leaves": (40, 98, 30),
    "leaves_hi": (74, 138, 46), "dark_leaves": (28, 72, 22), "birch_leaves": (120, 162, 74), "leaf_rim": (24, 60, 20), "log": (98, 78, 50),
    "fence": (144, 114, 74), "scaffold": (224, 194, 114), "plank": (172, 138, 88), "bell": (234, 202, 62),
    "lantern": (252, 224, 122), "anvil": (68, 68, 74), "stairs": (152, 152, 152), "kiln": (162, 86, 66),
    "clay_rack": (178, 164, 142), "charcoal": (40, 38, 36), "ash": (92, 90, 88), "barrel": (122, 90, 54),
    "chest": (158, 114, 54), "rubble": (102, 98, 94), "vine": (60, 100, 40), "awning_r": (192, 62, 52),
    "awning_w": (236, 232, 222), "copper_ore": (176, 112, 78), "door": (122, 92, 56), "composter": (110, 84, 50),
    "stone": (126, 126, 126), "mud": (92, 76, 62), "trough": (120, 110, 100), "leaf_rim": (24, 60, 20),
}
# altura dos objetos acima do chão (para as sombras)
HEIGHT = {"oak_roof": 6, "spruce_roof": 6, "dark_roof": 6, "deepslate": 6, "terracotta": 6, "cobble_wall": 3,
          "leaves": 7, "leaves_hi": 7, "dark_leaves": 8, "birch_leaves": 7, "scaffold": 4, "fence": 1, "kiln": 3,
          "hay": 1, "barrel": 1, "chest": 1, "bush": 1, "log": 2, "bell": 3, "cane": 2, "clay_rack": 2, "lantern": 2}


def wobble(z, base, amp, seed):
    return base + amp * math.sin(z / 37 + seed) + amp * 0.5 * math.sin(z / 13 + seed * 2)


def river_x(z):
    return 112 + 8 * math.sin(z / 40) + 4 * math.sin(z / 15)


def noise2(x, z, cell, seed):
    rng = np.random.default_rng(seed)
    g = rng.random((int(WB / cell) + 3, int(HB / cell) + 3))
    fx, fz = (x - X0) / cell, (z - Z0) / cell
    ix, iz = int(fx), int(fz)
    ux, uz = fx - ix, fz - iz
    ux, uz = ux * ux * (3 - 2 * ux), uz * uz * (3 - 2 * uz)
    return (g[ix, iz] * (1 - ux) + g[ix + 1, iz] * ux) * (1 - uz) + (g[ix, iz + 1] * (1 - ux) + g[ix + 1, iz + 1] * ux) * uz


class Village:
    def __init__(self):
        self.kind = np.full((HB, WB), "grass", dtype=object)
        self.level = np.zeros((HB, WB), dtype=np.float32)
        self.extra = np.zeros((HB, WB), dtype=np.float32)       # altura do objeto
        self.roof = np.zeros((HB, WB), dtype=np.int8)           # 1 água norte do telhado, -1 água sul
        self.rng = random.Random(7)

    def inside(self, x, z):
        return X0 <= x < X1 and Z0 <= z < Z1

    def set(self, x, z, kind, height=None):
        x, z = int(math.floor(x)), int(math.floor(z))
        if self.inside(x, z):
            self.kind[z - Z0, x - X0] = kind
            self.extra[z - Z0, x - X0] = HEIGHT.get(kind, 0) if height is None else height

    def get(self, x, z):
        return self.kind[z - Z0, x - X0] if self.inside(x, z) else None

    def rect(self, x0, z0, x1, z1, kind, height=None):
        for z in range(int(z0), int(z1)):
            for x in range(int(x0), int(x1)):
                self.set(x, z, kind, height)

    def outline(self, x0, z0, x1, z1, kind, height=None):
        for x in range(int(x0), int(x1)):
            self.set(x, z0, kind, height); self.set(x, z1 - 1, kind, height)
        for z in range(int(z0), int(z1)):
            self.set(x0, z, kind, height); self.set(x1 - 1, z, kind, height)

    def disc(self, cx, cz, r, kind, keep=(), height=None):
        for z in range(int(cz - r) - 1, int(cz + r) + 2):
            for x in range(int(cx - r) - 1, int(cx + r) + 2):
                if (x + 0.5 - cx) ** 2 + (z + 0.5 - cz) ** 2 <= r * r and self.get(x, z) not in keep:
                    self.set(x, z, kind, height)

    def line(self, pts, width, kind, keep=()):
        for (ax, az), (bx, bz) in zip(pts, pts[1:]):
            steps = int(max(abs(bx - ax), abs(bz - az)) * 2) + 1
            for k in range(steps + 1):
                t = k / steps
                self.disc(ax + (bx - ax) * t, az + (bz - az) * t, width / 2, kind, keep, 0)


# ---------- chão ----------

def terrain(v):
    for z in range(Z0, Z1):
        rx = river_x(z)
        for x in range(X0, X1):
            if abs(x + 0.5 - rx) < 6.5:
                kind, lvl = ("water" if abs(x + 0.5 - rx) < 4 else "water_sh"), -6
            elif abs(x + 0.5 - rx) < 8.5:
                kind, lvl = ("sand" if (x * 7 + z * 3) % 5 else "gravel"), -5
            elif x > rx:
                kind, lvl = "grass", -3
            elif x > wobble(z, STEP_BANK, 4, 2.6):
                kind, lvl = "grass", -4
            elif x > wobble(z, STEP_MID, 4, 1.3):
                kind, lvl = "grass", -2
            elif x < wobble(z, STEP_WEST, 5, 0.4):
                kind, lvl = "grass", 1
            else:
                kind, lvl = "grass", 0
            slope = max(0.0, (-72 - z + 5 * math.sin(x / 18)) / 4)      # encosta do Morro da Mina
            v.kind[z - Z0, x - X0] = kind
            v.level[z - Z0, x - X0] = lvl + min(slope, 7)
            if slope > 2.5 and kind == "grass" and v.rng.random() < 0.18:
                v.kind[z - Z0, x - X0] = v.rng.choice(["stone", "gravel", "coarse"])
    # o capim tem manchas: mais seco aqui, mais escuro ali; flores em canteiros naturais
    for z in range(Z0, Z1):
        for x in range(X0, X1):
            if v.get(x, z) != "grass":
                continue
            n = noise2(x, z, 9, 3)
            f = noise2(x, z, 5, 4)
            r = v.rng.random()
            if n > 0.72: v.set(x, z, "grass_dry")
            elif n < 0.25: v.set(x, z, "grass_dark")
            if r < 0.05: v.set(x, z, "tallgrass")
            if f > 0.88 and r < 0.22:
                v.set(x, z, ["flower_y", "flower_r", "flower_w", "flower_b", "flower_p"][int(noise2(x, z, 14, 6) * 5) % 5])
    # na margem: juncos (cana) e vitórias-régias
    for z in range(Z0, Z1):
        rx = river_x(z)
        for side in (-1, 1):
            if v.rng.random() < 0.25:
                v.set(rx + side * 7.2, z, "cane")
            if v.rng.random() < 0.08:
                v.set(rx + side * 5, z, "lily")


def tree(v, x, z, kind="oak"):
    leaves = {"oak": "leaves", "birch": "birch_leaves", "dark": "dark_leaves"}[kind]
    r = {"oak": 2.6, "birch": 2.1, "dark": 3.4}[kind]
    keep = ("water", "water_sh", "path", "cobble", "stairs", "farmland", "wheat", "wheat_young", "oak_roof", "spruce_roof",
            "dark_roof", "deepslate", "terracotta", "plank", "fence", "sand", "gravel")
    v.disc(x, z, r + 0.6, "leaf_rim", keep, height=6)
    v.disc(x, z, r, leaves, keep)
    v.disc(x - 0.8, z - 0.8, r * 0.45, "leaves_hi" if kind != "birch" else "birch_leaves", keep)


def vegetation(v):
    # a mata a oeste, densa, com borda irregular e alguns carvalhos-escuros
    for _ in range(260):
        z = v.rng.uniform(Z0 - 2, Z1 + 2)
        edge = -112 + 6 * math.sin(z / 18)
        x = v.rng.uniform(X0 - 3, edge)
        tree(v, x, z, v.rng.choice(["oak", "dark", "dark", "birch"]))
    for _ in range(60):                                   # arbustos e samambaias sob a mata
        z = v.rng.uniform(Z0, Z1); x = v.rng.uniform(X0, -108)
        if v.get(int(x), int(z)) in ("grass", "grass_dark", "grass_dry", "tallgrass"):
            v.set(x, z, v.rng.choice(["bush", "fern"]))
    # árvores soltas e bosquetes pela vila e pela outra margem
    for cx, cz, k in [(-60, -60, 4), (50, -78, 3), (100, 80, 3), (130, -40, 5), (130, 30, 4), (-48, 90, 3), (60, 92, 2),
                      (-80, -10, 2), (8, -70, 2), (-100, 30, 2), (40, 75, 1), (-30, 88, 2)]:
        for _ in range(k):
            tree(v, cx + v.rng.uniform(-8, 8), cz + v.rng.uniform(-6, 6), v.rng.choice(["oak", "oak", "birch"]))
    for _ in range(45):                                   # pedras e árvores na encosta norte
        x, z = v.rng.uniform(X0, X1 - 30), v.rng.uniform(Z0, -78)
        if v.rng.random() < 0.55:
            tree(v, x, z, "oak")
        else:
            v.disc(x, z, v.rng.uniform(0.8, 1.6), v.rng.choice(["stone", "cobble"]), height=1)


ROADS = [
    ([(0, -9), (0, -26)], 3), ([(0, -38), (-4, -70), (-10, -96)], 3),                  # casa do fundador; estrada da mina
    ([(12, 0), (20, 0)], 3), ([(-12, 2), (-24, 3)], 3),                                # ferreiro, lavradora
    ([(32, 0), (58, 6), (84, 24)], 3),                                                 # olaria (desce dois degraus)
    ([(-24, 6), (-46, 32), (-60, 46)], 2), ([(-60, 46), (-70, 54)], 2),                 # pasto
    ([(-10, -6), (-50, -28), (-98, -40)], 2),                                         # carvoaria
    ([(4, 10), (12, 16)], 3), ([(58, 6), (57, -14)], 2),                              # vendinha, oficina
    ([(-2, 10), (-6, 40), (0, 70), (8, 106)], 2),                                     # estrada do sul (Brejo)
    ([(84, 24), (100, 26)], 2),                                                        # trilha até a beira do rio
]


def roads(v):
    for pts, width in ROADS:
        v.line(pts, width, "path", keep=("water", "water_sh"))
        v.line(pts, width + 1.4, "coarse", keep=("water", "water_sh", "path"))       # beiradas gastas
    for z in range(Z0, Z1):                                                              # tira a beirada onde não é borda
        for x in range(X0, X1):
            if v.get(x, z) == "coarse" and v.rng.random() < 0.55:
                v.set(x, z, "path" if v.rng.random() < 0.4 else "grass_dry")
    # escadas de pedra onde a rua desce um degrau
    for pts, width in ROADS:
        for (ax, az), (bx, bz) in zip(pts, pts[1:]):
            for base, seed in ((STEP_MID, 1.3), (STEP_BANK, 2.6), (STEP_WEST, 0.4)):
                for k in range(300):
                    t = k / 300
                    x, z = ax + (bx - ax) * t, az + (bz - az) * t
                    if abs(x - wobble(z, base, 4 if base != STEP_WEST else 5, seed)) < 0.5:
                        v.disc(x, z, width / 2 + 0.8, "stairs", height=0)
                        break


# ---------- construções ----------

def house(v, x, z, w, h, roof, state="ok", door="s", garden=True):
    x0, z0 = int(x - w / 2), int(z - h / 2)
    x1, z1 = x0 + w, z0 + h
    ridge = (z0 + z1) // 2
    if state in ("ok", "ruina"):
        v.rect(x0, z0, x1, z1, roof)
        for zz in range(z0, z1):
            for xx in range(x0, x1):
                if v.inside(xx, zz):
                    v.roof[zz - Z0, xx - X0] = 1 if zz < ridge else -1
                    v.extra[zz - Z0, xx - X0] = 4 + (3 - abs(zz - ridge + 0.5)) * 0.6
        v.rect(x0, ridge, x1, ridge + 1, "log", height=7)
        v.set(x0 + 1, z0 + 1, "cobble", height=8)                 # chaminé
        dz = z1 if door == "s" else z0 - 1
        v.set((x0 + x1) // 2, dz, "door", height=0)
        v.set((x0 + x1) // 2, dz + (1 if door == "s" else -1), "path", height=0)
        if garden and state == "ok":
            for k in range(x0, x1):
                if v.rng.random() < 0.55:
                    v.set(k, z1 + 1 if door == "s" else z0 - 2, v.rng.choice(["flower_r", "flower_y", "flower_p", "bush"]))
            v.set(x1, z1, "composter", height=1)
        if state == "ruina":
            for _ in range(w * h // 2):
                v.set(v.rng.randint(x0, x1 - 1), v.rng.randint(z0, z1 - 1), v.rng.choice(["rubble", "coarse", "tallgrass", "vine", "mossy"]), height=1)
            for k in range(z0, z1):
                if v.rng.random() < 0.6:
                    v.set(x0 - 1, k, "vine", height=2)
    elif state == "obra":
        v.rect(x0, z0, x1, z1, "dirt", height=0)
        v.outline(x0, z0, x1, z1, "cobble", height=3)
        for k in range(x0 + 1, x1 - 1, 2):
            v.set(k, z0 + 1, "plank", height=2)
        v.outline(x0 - 1, z0 - 1, x1 + 1, z1 + 1, "scaffold")
        v.rect(x1 + 1, z0, x1 + 4, z0 + 2, "plank", height=1)
        v.rect(x1 + 1, z0 + 3, x1 + 3, z0 + 5, "cobble", height=1)
    elif state == "lote":
        v.outline(x0, z0, x1, z1, "gravel", height=0)
        v.rect(x0 + 1, z0 + 1, x1 - 1, z1 - 1, "tallgrass", height=0)
        v.set(x0 + 2, z0 + 2, "fence"); v.set(x1 - 3, z1 - 3, "fence")


# As construções do estágio 1 (centro x, z, largura, fundo). Os estágios seguintes
# (`vila_crescimento.py`) partem destas posições e só acrescentam: nada é derrubado.
BUILDINGS = {
    "Casa do fundador": (0, -32, 15, 11), "Ferreiro · Seu Bento": (25, -3, 12, 10), "Lavradora · Dona Cida": (-30, 1, 11, 9),
    "Vendinha · Seu Zé": (15, 20, 10, 8), "Olaria · Dona Zefa": (86, 32, 12, 9), "Carvoaria · Seu Lino": (-104, -44, 10, 8),
    "Vaqueiro · Seu Tonho": (-70, 46, 9, 7),
}
HOUSES = [(-16, 30, "oak_roof"), (20, -24, "dark_roof"), (-22, -22, "oak_roof"), (-14, 52, "spruce_roof"),
          (24, 50, "oak_roof"), (-48, -18, "spruce_roof"), (52, -42, "dark_roof")]
WORKS = [(46, 26), (-2, 74)]
RUINS = [(28, -54)]
LOTS = [(-38, -40), (50, 70), (-34, 42), (16, 70)]
# o que mais ocupa chão: praça, roça, pasto, terreno da oficina, poço, forno, varais, monte de carvão
GROUNDS = {"Praça": (0, 0.5, 27, 19), "Roça": (-45, 19, 20, 20), "Pasto": (-75, 76, 42, 40), "Terreno da oficina": (57.5, -20.5, 11, 11),
           "Poço": (-20, 13, 4, 4), "Forno da olaria": (97, 41, 6, 6), "Varais": (82, 43, 9, 5), "Carvão": (-93, -32, 8, 8),
           "Forja": (33.5, -3.5, 4, 6)}

LABELS = []


def label(x, z, text, kind="name"):
    LABELS.append((x, z, text, kind))


def places(v):
    # a praça: pedregulho com falhas e musgo, a fonte seca e quebrada, o coreto sem telhado, o sino
    v.rect(-13, -9, 14, 10, "cobble", height=0)
    for _ in range(90):
        v.set(v.rng.randint(-13, 13), v.rng.randint(-9, 9), v.rng.choice(["mossy", "gravel", "tallgrass", "cracked", "mossy"]), height=0)
    v.disc(0, 0.5, 4.6, "stonebrick", height=1)
    v.disc(0, 0.5, 3.3, "coarse", height=0)
    for x, z in [(4, 2), (3, 4), (-4, -1), (1, -4), (-3, 3)]:
        v.set(x, z, "rubble", height=1)
    v.disc(0, 0.5, 1.1, "cracked", height=2)
    v.rect(-12, -8, -4, -1, "plank", height=0)
    for x, z in [(-12, -8), (-5, -8), (-12, -2), (-5, -2)]:
        v.set(x, z, "log", height=4)
    for x, z in [(-13, -5), (-3, -6), (-8, 0), (-6, -9), (-10, 1)]:
        v.set(x, z, "brick", height=1)
    v.set(7, -7, "bell")
    v.set(-13, 9, "barrel"); v.set(13, 9, "barrel"); v.set(-3, 8, "plank", height=1); v.set(3, 8, "plank", height=1)
    v.set(10, -8, "lantern")
    label(-1, -12.5, "Praça", "place"); label(-19, -14.5, "coreto sem telhado", "small"); label(0, 12.5, "fonte seca e quebrada", "small")
    # casa do fundador (norte): telhado de abeto e a horta
    house(v, 0, -32, 15, 11, "spruce_roof")
    v.rect(10, -36, 15, -28, "farmland", height=0); v.rect(11, -35, 14, -29, "carrot", height=0)
    label(0, -40, "Casa do fundador")
    # ferreiro: ardósia, a forja e a bigorna
    house(v, 25, -3, 12, 10, "deepslate", door="s", garden=False)
    v.rect(32, -6, 35, -1, "cobble", height=2); v.set(33, -4, "charcoal", height=2)
    v.set(32, 3, "anvil", height=1); v.set(34, 3, "barrel"); v.set(34, 4, "chest")
    label(28, -11, "Ferreiro · Seu Bento")
    # lavradora: casa, roça com o canal de água, feno
    house(v, -30, 1, 11, 9, "oak_roof")
    v.rect(-54, 10, -36, 28, "farmland", height=0)
    for zz in range(10, 28):
        if zz % 2 == 0:
            v.rect(-54, zz, -36, zz + 1, "wheat" if zz < 18 else "wheat_young", height=0)
    v.rect(-46, 10, -45, 28, "water", height=0)
    v.set(-35, 8, "hay"); v.set(-34, 8, "hay"); v.set(-35, 9, "hay")
    v.outline(-55, 9, -35, 29, "fence")
    label(-30, -6, "Lavradora · Dona Cida"); label(-45, 31, "roça de trigo", "small")
    # vendinha: toldo listrado, caixotes vazios
    house(v, 15, 20, 10, 8, "oak_roof", garden=False)
    for k in range(10, 20):
        v.set(k, 25, "awning_r" if k % 2 else "awning_w", height=3)
    v.set(9, 26, "barrel"); v.set(21, 26, "chest"); v.set(20, 27, "barrel")
    label(15, 31, "Vendinha · Seu Zé")
    v.rect(22, 15, 26, 18, "plank", height=2); v.set(22, 18, "log", height=1); v.set(25, 18, "log", height=1)   # carroça velha
    # olaria na margem: forno de tijolo rachado, varais de barro
    house(v, 86, 32, 12, 9, "terracotta", garden=False)
    v.disc(97, 41, 2.6, "kiln"); v.set(97, 41, "rubble", height=2); v.set(98, 40, "cracked", height=2)
    v.rect(78, 41, 86, 42, "clay_rack"); v.rect(78, 44, 86, 45, "clay_rack")
    label(86, 24, "Olaria · Dona Zefa")
    # carvoaria na borda da mata: o monte de carvão apagado, a lenha empilhada
    house(v, -104, -44, 10, 8, "spruce_roof", garden=False)
    v.disc(-93, -32, 3.4, "ash", height=1); v.disc(-93, -32, 2.0, "charcoal", height=2)
    v.rect(-100, -30, -96, -28, "log"); v.rect(-100, -27, -97, -26, "log"); v.rect(-110, -36, -107, -33, "log")
    label(-104, -52, "Carvoaria · Seu Lino")
    # vaqueiro: casa, celeiro aberto, pasto grande com uma vaca só, bebedouro
    house(v, -70, 46, 9, 7, "oak_roof")
    v.outline(-96, 56, -54, 96, "fence")
    v.set(-72, 56, "path", height=0); v.set(-71, 56, "path", height=0)
    v.rect(-92, 60, -88, 63, "hay"); v.rect(-82, 90, -78, 91, "trough", height=1)
    label(-70, 40, "Vaqueiro · Seu Tonho")
    # casas da vila
    for x, z, roof in HOUSES:
        house(v, x, z, 10, 8, roof)
    for x, z in WORKS:
        house(v, x, z, 10, 8, "oak_roof", "obra")
    for x, z in RUINS:
        house(v, x, z, 10, 8, "oak_roof", "ruina")
    for x, z in LOTS:
        house(v, x, z, 10, 8, "", "lote")
    label(46, 33, "em obra", "small"); label(-2, 81, "em obra", "small"); label(28, -61, "abandonada", "small")
    for x, z in LOTS:
        label(x, z, "lote", "small")
    # poço seco
    v.disc(-20, 13, 1.6, "cobble", height=1); v.set(-20, 13, "coarse", height=0)
    label(-20, 17.5, "poço seco", "small")
    # o terreno da oficina fica livre (marcado só no desenho) e o cobre aflora ao lado
    v.rect(52, -26, 63, -15, "grass", height=0)
    for dx, dz in [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2), (3, 2), (2, 3)]:
        v.set(64 + dx, -40 + dz, "copper_ore", height=1)
    label(66, -44, "cobre aflorando", "small")


# ---------- desenho ----------

def render_map(v):
    """Só o mapa (sem painel e sem rótulos), para os planos de crescimento desenharem por cima."""
    return _render(v, labels=False)


def render(v, out):
    im = _render(v, labels=True)
    _panel(im, out)


def _render(v, labels=True):
    rng = np.random.default_rng(11)
    base = np.zeros((HB, WB, 3), np.float32)
    for zi in range(HB):
        for xi in range(WB):
            base[zi, xi] = COLORS[v.kind[zi, xi]]
    base *= rng.uniform(0.95, 1.05, (HB, WB, 1))
    # telhados: a água norte pega sol, a sul fica na sombra
    base[v.roof == 1] *= 1.10
    base[v.roof == -1] *= 0.84
    # sombra do sol do noroeste (casas, árvores e degraus do terreno)
    elev = v.level + v.extra
    shadow = np.zeros((HB, WB), np.float32)
    for k in range(1, 7):
        nb = np.full_like(elev, -99)
        nb[k:, k:] = elev[:-k, :-k]
        shadow = np.maximum(shadow, (nb - elev - k * 0.9 > 0).astype(np.float32) * (1 - k * 0.08))
    light = 1 - 0.32 * shadow
    up = np.roll(v.level, 1, axis=0); left = np.roll(v.level, 1, axis=1)
    light *= 1 + 0.09 * np.clip((v.level - up) + (v.level - left), -4, 4)
    water = np.isin(v.kind, ["water", "water_sh"])
    rx = np.array([river_x(z) for z in range(Z0, Z1)])[:, None]
    depth = np.clip(1 - np.abs(np.arange(X0, X1)[None, :] + 0.5 - rx) / 6.5, 0, 1)
    base[water] *= (1 - 0.25 * depth[water])[:, None]
    img = base * light[..., None]
    big = np.kron(img, np.ones((S, S, 1)))
    # textura dentro de cada bloco, como os pixels de uma textura do jogo
    big *= rng.uniform(0.92, 1.06, big.shape[:2])[..., None]
    big[::S, :, :] *= 0.97
    big[:, ::S, :] *= 0.97
    im = Image.fromarray(np.clip(big, 0, 255).astype(np.uint8), "RGB")
    d = ImageDraw.Draw(im, "RGBA")
    P = lambda x, z: ((x - X0) * S, (z - Z0) * S)
    # ondinhas no rio
    for _ in range(140):
        z = rng.uniform(Z0, Z1); x = river_x(z) + rng.uniform(-4, 4)
        a, b = P(x, z)
        d.line([(a, b), (a + 7, b)], fill=(150, 185, 240, 150), width=1)
    # terreno da oficina, tracejado
    a, b = P(52, -26); c, e = P(63, -15)
    for k in range(int(a), int(c), 10):
        d.line([(k, b), (k + 5, b)], fill=(150, 255, 150, 255), width=2); d.line([(k, e), (k + 5, e)], fill=(150, 255, 150, 255), width=2)
    for k in range(int(b), int(e), 10):
        d.line([(a, k), (a, k + 5)], fill=(150, 255, 150, 255), width=2); d.line([(c, k), (c, k + 5)], fill=(150, 255, 150, 255), width=2)
    # lugar da roda d'água
    a, b = P(river_x(-20) - 5, -20)
    d.ellipse([a - 7, b - 7, a + 7, b + 7], outline=(255, 255, 255, 255), width=2)
    # a Mimosa no pasto
    a, b = P(-74, 78)
    d.rectangle([a, b, a + 2 * S, b + S], fill=(245, 245, 240, 255), outline=(50, 35, 25, 255))
    d.rectangle([a + 3, b + 2, a + 7, b + 5], fill=(40, 30, 25, 255))
    d.rectangle([a + 2 * S, b + 1, a + 2 * S + 4, b + S - 1], fill=(235, 225, 215, 255))
    # o prefeito e os moradores
    for px, pz, gold in [(4, 4, True), (20, 3, False), (-24, 7, False), (15, 27, False), (88, 38, False), (-98, -40, False), (-66, 52, False)]:
        a, b = P(px, pz)
        d.ellipse([a - 5, b - 5, a + 5, b + 5], fill=(255, 215, 0, 255) if gold else (225, 225, 240, 255), outline=(0, 0, 0, 255), width=2)
    if not labels:
        return im
    # rótulos
    for x, z, text, kind in LABELS + [(13, 6.5, "Prefeito Otávio", "small"), (-74, 76, "Mimosa", "small"),
                                       (57, -29, "Terreno da oficina", "small"), (river_x(-20) - 5, -26, "lugar da roda d'água", "small"),
                                       (river_x(-80) + 12, -80, "Rio Veio", "place"), (-120, 0, "Mata", "place"),
                                       (10, -91, "↑ Mina do avô, no Morro da Mina (300 blocos)", "small"),
                                       (30, 101, "↓ Brejo do Rio, onde está a argila (~1.200 blocos)", "small"),
                                       (-6, 94, "praça: nível 64", "level"), (54, 94, "terraço do meio: 62", "level"),
                                       (92, 94, "margem: 60", "level"), (-86, 12, "+1", "level")]:
        a, b = P(x, z)
        if kind == "name":
            d.text((a, b), text, fill=(255, 255, 255, 255), font=font(17, True), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0, 255))
        elif kind == "place":
            d.text((a, b), text, fill=(255, 245, 210, 255), font=font(18, True), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0, 255))
        elif kind == "level":
            d.text((a, b), text, fill=(210, 235, 255, 255), font=font(13, True), anchor="mm", stroke_width=3, stroke_fill=(10, 30, 60, 255))
        else:
            d.text((a, b), text, fill=(255, 240, 205, 255), font=font(13), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0, 255))
    return im


def _panel(im, out):
    W = im.width
    side = 500
    canvas = Image.new("RGB", (W + side, im.height + 104), (22, 24, 28))
    canvas.paste(im, (0, 104))
    cd = ImageDraw.Draw(canvas)
    cd.text((18, 14), "A vila no estágio 1: a chegada", fill=(255, 255, 255), font=font(34, True))
    cd.text((18, 62), "Como ela está quando você abre o jogo · 270 x 200 blocos · 1 quadradinho = 1 bloco · norte para cima · sol da tarde a noroeste",
            fill=(200, 205, 215), font=font(15))
    a, y0 = 24, canvas.height - 26
    cd.line([(a, y0), (a + 20 * S, y0)], fill=(255, 255, 255), width=4)
    cd.text((a + 20 * S + 8, y0 - 9), "20 blocos", fill=(255, 255, 255), font=font(13))
    x0, y = W + 22, 122
    cd.text((x0, y), "Estágio 1 · a vila em declínio", fill=(255, 215, 0), font=font(20, True)); y += 34
    notes = [
        "Praça de pedregulho com musgo e falhas. A fonte",
        "está seca e quebrada: o pedregulho que o prefeito",
        "pede é para consertá-la.",
        "O coreto perdeu o telhado: as telhas estão no",
        "chão (o pedido da Dona Zefa).",
        "7 casas habitadas, 2 em obra (andaime, pilhas de",
        "tábuas e pedra), 1 abandonada (telhado caído,",
        "trepadeira) e 4 lotes vazios, com capim alto.",
        "A vendinha tem caixotes vazios; a carvoaria está",
        "apagada (cinza fria); o forno da olaria rachou; o",
        "pasto é grande para uma vaca só, a Mimosa.",
        "Poço seco e só uma lanterna na praça: de noite",
        "a vila fica escura.",
        "Ruas de terra gasta; escadas de pedra onde a",
        "rua desce para o terraço do meio e para a margem.",
        "O terreno da oficina é o chão plano que o",
        "prefeito separa, perto do cobre e da água.",
    ]
    for line in notes:
        cd.text((x0, y), line, fill=(205, 208, 215), font=font(14)); y += 20
    y += 16
    cd.text((x0, y), "Blocos", fill=(255, 255, 255), font=font(18, True)); y += 28
    items = [("cobble", "pedregulho"), ("mossy", "pedregulho com musgo"), ("stonebrick", "tijolo de pedra (fonte)"),
             ("path", "caminho de terra"), ("stairs", "escada de pedra"), ("spruce_roof", "telhado de abeto"),
             ("oak_roof", "telhado de carvalho"), ("dark_roof", "telhado de carvalho-escuro"), ("deepslate", "ardósia (ferreiro)"),
             ("terracotta", "terracota (olaria)"), ("scaffold", "andaime"), ("wheat", "trigo"), ("farmland", "terra arada"),
             ("hay", "fardo de feno"), ("fence", "cerca"), ("leaves", "copa de carvalho"), ("dark_leaves", "copa de carvalho-escuro"),
             ("birch_leaves", "copa de bétula"), ("water", "rio"), ("cane", "cana na margem"), ("sand", "areia e cascalho"),
             ("copper_ore", "minério de cobre"), ("ash", "carvoaria apagada"), ("kiln", "forno da olaria")]
    for k, (kind, text) in enumerate(items):
        cx = x0 + (k % 2) * 240; cy = y + (k // 2) * 22
        cd.rectangle([cx, cy, cx + 15, cy + 15], fill=COLORS[kind], outline=(170, 170, 170))
        cd.text((cx + 23, cy - 1), text, fill=(210, 210, 210), font=font(13))
    canvas.save(out)


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("docs/mapa/costa-oeste")
    out_dir.mkdir(parents=True, exist_ok=True)
    v = Village()
    terrain(v)
    roads(v)
    places(v)
    vegetation(v)
    render(v, out_dir / "vila_estagio1.png")


if __name__ == "__main__":
    main()
