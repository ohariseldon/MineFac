"""Prévias do mapa fixo do Veio (15.000 x 15.000 blocos, centro em 0,0).

Cada conceito desenha os campos de clima do gerador do Minecraft 1.18+ (continentalidade,
temperatura, umidade, montanha, erosão e "estranheza") em escala de continente e aplica uma
tabela de biomas parecida com a do vanilla. Isso dá transições graduais: o bioma muda porque o
clima muda devagar, não porque alguém pintou uma fronteira.

Uso: python3 tools/mapa/preview_maps.py [pasta de saída]
Gera mapa_1.png .. mapa_5.png, vila.png e cobertura.txt.
Não é o gerador do jogo: é a planta que o gerador vai seguir.
"""
import math
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

HALF = 7500
N = 1000                       # pixels por lado (15 blocos por pixel)
BLOCK = 2 * HALF / N
xs = (np.arange(N) + 0.5) * BLOCK - HALF
X, Z = np.meshgrid(xs, xs)     # X: leste, Z: sul
U, V = X / HALF, Z / HALF

# ---------- ruído ----------
_rng = np.random.default_rng(7)
_TABLE = _rng.uniform(-1, 1, (16, 512, 512)).astype(np.float32)


def value_noise(x, z, cell, seed):
    t = _TABLE[seed % 16]
    fx, fz = x / cell + seed * 37.1, z / cell + seed * 11.7
    ix, iz = np.floor(fx).astype(np.int64), np.floor(fz).astype(np.int64)
    ux, uz = fx - ix, fz - iz
    ux, uz = ux * ux * (3 - 2 * ux), uz * uz * (3 - 2 * uz)
    a = t[ix % 512, iz % 512]; b = t[(ix + 1) % 512, iz % 512]
    c = t[ix % 512, (iz + 1) % 512]; d = t[(ix + 1) % 512, (iz + 1) % 512]
    return (a * (1 - ux) + b * ux) * (1 - uz) + (c * (1 - ux) + d * ux) * uz


def fbm(x, z, cell, seed, octaves=5):
    total, amp, norm = 0.0, 1.0, 0.0
    for o in range(octaves):
        total = total + amp * value_noise(x, z, cell / 2 ** o, seed * 7 + o)
        norm += amp; amp *= 0.5
    return total / norm


def warp(seed, strength=900, cell=2600):
    """Coordenadas deformadas: as bordas dos climas ficam orgânicas."""
    return (X + strength * fbm(X, Z, cell, seed + 1, 4), Z + strength * fbm(X, Z, cell, seed + 2, 4))


def seg_distance(px, pz, pts):
    """Distância de cada pixel até uma linha (lista de pontos em blocos)."""
    best = np.full(px.shape, 1e9, dtype=np.float32)
    for (ax, az), (bx, bz) in zip(pts, pts[1:]):
        dx, dz = bx - ax, bz - az
        t = np.clip(((px - ax) * dx + (pz - az) * dz) / (dx * dx + dz * dz), 0, 1)
        best = np.minimum(best, np.hypot(px - ax - t * dx, pz - az - t * dz))
    return best


def ridge(px, pz, pts, width, seed):
    """Uma serra ao longo da linha: alta no eixo, com picos e colos irregulares."""
    px = px + 700 * fbm(px, pz, 1600, seed + 40, 4)
    pz = pz + 700 * fbm(px, pz, 1600, seed + 41, 4)
    d = seg_distance(px, pz, pts)
    w = width * (0.75 + 0.5 * (fbm(px, pz, 3000, seed, 3) + 1) / 2)
    crest = 0.65 + 0.55 * (1 - np.abs(fbm(px, pz, 900, seed + 5, 4)))
    return np.exp(-(d / w) ** 2) * crest


def blob(cx, cz, r, rough=0.0, seed=0):
    """Mancha redonda; com `rough`, a borda fica irregular como um contorno natural."""
    d = np.hypot(X - cx, Z - cz)
    if rough:
        d = d * (1 + rough * fbm(X, Z, r * 1.2, seed + 900, 4))
    return np.exp(-(d / r) ** 2)


# ---------- biomas (nome, cor, nome em português) ----------
BIOMES = [
    ("ocean", (35, 70, 160), "Oceano"), ("deep_ocean", (20, 45, 120), "Oceano profundo"),
    ("warm_ocean", (40, 140, 190), "Oceano quente"), ("lukewarm_ocean", (45, 110, 180), "Oceano morno"),
    ("deep_lukewarm_ocean", (30, 85, 150), "Oceano morno profundo"), ("cold_ocean", (50, 70, 140), "Oceano frio"),
    ("deep_cold_ocean", (30, 45, 110), "Oceano frio profundo"), ("frozen_ocean", (120, 140, 200), "Oceano congelado"),
    ("deep_frozen_ocean", (80, 95, 160), "Oceano congelado profundo"), ("mushroom_fields", (200, 90, 200), "Campos de cogumelos"),
    ("beach", (235, 220, 150), "Praia"), ("snowy_beach", (235, 235, 215), "Praia nevada"), ("stony_shore", (140, 140, 130), "Costa rochosa"),
    ("river", (60, 110, 220), "Rio"), ("frozen_river", (170, 190, 240), "Rio congelado"),
    ("plains", (140, 190, 90), "Planície"), ("sunflower_plains", (180, 205, 80), "Planície de girassóis"),
    ("snowy_plains", (240, 245, 250), "Planície nevada"), ("ice_spikes", (185, 225, 245), "Picos de gelo"),
    ("desert", (230, 200, 120), "Deserto"), ("swamp", (75, 100, 70), "Pântano"), ("mangrove_swamp", (60, 110, 60), "Manguezal"),
    ("forest", (60, 130, 50), "Floresta"), ("flower_forest", (110, 160, 60), "Floresta florida"),
    ("birch_forest", (110, 165, 90), "Floresta de bétulas"), ("old_growth_birch_forest", (90, 150, 80), "Bétulas antigas"),
    ("dark_forest", (40, 80, 30), "Floresta escura"), ("taiga", (60, 110, 85), "Taiga"),
    ("old_growth_spruce_taiga", (75, 95, 70), "Taiga de abetos antigos"), ("old_growth_pine_taiga", (90, 100, 75), "Taiga de pinheiros antigos"),
    ("snowy_taiga", (155, 185, 175), "Taiga nevada"), ("savanna", (190, 180, 90), "Savana"),
    ("savanna_plateau", (170, 160, 85), "Planalto de savana"), ("windswept_savanna", (200, 185, 120), "Savana fustigada"),
    ("windswept_hills", (125, 140, 120), "Colinas fustigadas"), ("windswept_gravelly_hills", (145, 145, 140), "Colinas de cascalho"),
    ("windswept_forest", (85, 115, 85), "Floresta fustigada"), ("jungle", (40, 150, 40), "Selva"),
    ("sparse_jungle", (90, 165, 60), "Selva esparsa"), ("bamboo_jungle", (110, 185, 50), "Selva de bambu"),
    ("badlands", (210, 110, 60), "Terras áridas"), ("eroded_badlands", (225, 90, 50), "Terras áridas erodidas"),
    ("wooded_badlands", (175, 120, 70), "Terras áridas arborizadas"), ("meadow", (150, 200, 130), "Prado"),
    ("cherry_grove", (240, 170, 200), "Cerejeiral"), ("grove", (190, 215, 205), "Bosque nevado"),
    ("snowy_slopes", (220, 230, 240), "Encostas nevadas"), ("frozen_peaks", (200, 215, 240), "Picos congelados"),
    ("jagged_peaks", (230, 235, 245), "Picos irregulares"), ("stony_peaks", (160, 155, 150), "Picos de pedra"),
]
CAVES = [("dripstone_caves", (150, 110, 80), "Cavernas de espeleotemas"), ("lush_caves", (90, 200, 90), "Cavernas exuberantes"),
         ("deep_dark", (20, 40, 50), "Escuridão profunda")]
INDEX = {name: i for i, (name, _, _) in enumerate(BIOMES)}
COLORS = np.array([c for _, c, _ in BIOMES], dtype=np.float32)


def classify(C, T, H, M, E, W, mushroom):
    """Tabela de biomas no espírito do vanilla: oceanos pela temperatura, costa, montanhas
    pela altura e pelo frio, e as terras baixas por temperatura x umidade. As variantes raras
    (picos de gelo, girassóis, cerejeiral...) ficam com uma fatia fixa da região-mãe, para
    nenhum bioma sumir nem dominar."""
    t = np.digitize(T, [-0.45, -0.15, 0.2, 0.55])           # 0 gelado .. 4 quente
    h = np.digitize(H, [-0.35, -0.1, 0.1, 0.3])             # 0 seco .. 4 úmido
    out = np.full(C.shape, INDEX["plains"], dtype=np.int16)
    def put(mask, name): out[mask] = INDEX[name]
    def share(mask, name, part, field=W, low=False):
        if not mask.any(): return
        cut = np.quantile(field[mask], part if low else 1 - part)
        put(mask & ((field <= cut) if low else (field >= cut)), name)

    land = C >= -0.12
    table = {
        0: ["snowy_plains", "snowy_plains", "snowy_plains", "snowy_taiga", "snowy_taiga"],
        1: ["plains", "plains", "forest", "taiga", "old_growth_spruce_taiga"],
        2: ["plains", "plains", "forest", "birch_forest", "dark_forest"],
        3: ["savanna", "savanna", "forest", "sparse_jungle", "jungle"],
        4: ["desert", "desert", "desert", "desert", "desert"],
    }
    for ti, row in table.items():
        for hi, name in enumerate(row):
            put(land & (t == ti) & (h == hi), name)
    flat = land & (M <= 0.30)
    share(flat & (t == 0) & (h <= 2), "ice_spikes", 0.35)
    share(flat & (t == 2) & (h <= 1), "sunflower_plains", 0.30)
    share(flat & (t == 2) & (h == 1), "flower_forest", 0.35, low=True)
    share(flat & (t == 2) & (h == 2), "flower_forest", 0.25)
    share(flat & (t == 2) & (h == 3), "old_growth_birch_forest", 0.40)
    share(flat & (t == 1) & (h == 4), "old_growth_pine_taiga", 0.50)
    share(flat & (t == 3) & (h == 4), "bamboo_jungle", 0.40)
    share(flat & (t == 4) & (h >= 3), "savanna", 0.0)        # (deserto inteiro, como no vanilla)
    hills = land & (M > 0.30) & (M <= 0.52)
    put(hills & (t <= 1), "windswept_hills")
    share(hills & (t <= 1), "windswept_gravelly_hills", 0.35)
    share(hills & (t <= 1), "windswept_forest", 0.30, field=H)
    put(hills & (t == 2), "meadow")
    share(hills & (t == 2), "windswept_forest", 0.30, field=H)
    put(hills & (t == 3), "windswept_savanna")
    share(hills & (t == 3), "savanna_plateau", 0.50, low=True)
    put(hills & (t == 4), "badlands")
    share(hills & (t == 4), "eroded_badlands", 0.35)
    share(hills & (t == 4), "wooded_badlands", 0.30, field=H)
    slopes = land & (M > 0.52) & (M <= 0.74)
    put(slopes & (t <= 1), "snowy_slopes")
    share(slopes & (t <= 1), "grove", 0.45, field=H)
    put(slopes & (t == 2), "meadow")
    share(slopes & (t == 2), "cherry_grove", 0.50)
    put(slopes & (t == 3), "savanna_plateau")
    put(slopes & (t == 4), "badlands")
    share(slopes & (t == 4), "eroded_badlands", 0.40)
    peaks = land & (M > 0.74)
    put(peaks & (t <= 1), "frozen_peaks")
    share(peaks & (t <= 2), "jagged_peaks", 0.45)
    put(peaks & (t >= 3), "stony_peaks")
    share(peaks & (t == 2), "stony_peaks", 0.35, low=True)
    coast = (C >= -0.12) & (C < -0.04)
    put(coast & (t >= 1), "beach")
    put(coast & (t == 0), "snowy_beach")
    share(coast, "stony_shore", 0.22, field=-E)
    put(coast & (M >= 0.3), "stony_shore")
    sea = C < -0.12
    deep = np.zeros_like(sea)
    for band in range(5):                                    # cada oceano tem a sua parte funda
        m = sea & (t == band)
        if m.any(): deep |= m & (C <= np.quantile(C[m], 0.45))
    put(sea & (t == 4), "warm_ocean")
    put(sea & (t == 3), "lukewarm_ocean"); put(deep & (t == 3), "deep_lukewarm_ocean")
    put(sea & (t == 2), "ocean"); put(deep & (t == 2), "deep_ocean")
    put(sea & (t == 1), "cold_ocean"); put(deep & (t == 1), "deep_cold_ocean")
    put(sea & (t == 0), "frozen_ocean"); put(deep & (t == 0), "deep_frozen_ocean")
    put(mushroom & (C > -0.25), "mushroom_fields")
    return out, t


def equalize(field, mask, cuts):
    """Reposiciona o campo para que cada faixa (entre os cortes da tabela) cubra a mesma fração
    da terra. A ordem dos valores não muda: o norte continua mais frio que o sul."""
    q = np.quantile(field[mask], np.linspace(0, 1, len(cuts) + 2))
    targets = np.concatenate([[cuts[0] - 0.4], cuts, [cuts[-1] + 0.4]])
    return np.interp(field, q, targets)


# ---------- os cinco conceitos ----------
# Cada conceito devolve os campos (C, T, H, M, E, W), a máscara do cogumelo, os rios e os nomes
# no mapa. A vila fica sempre no centro, numa planície temperada.

def village_patch(C, T, H, M):
    """Planície temperada em volta da vila, floresta a oeste, rio a leste."""
    w = blob(0, 0, 650, 0.55, 3)
    C = C * (1 - w) + 0.32 * w
    T = T * (1 - w) + 0.02 * w
    H = H * (1 - w) + (-0.2) * w
    M = M * (1 - blob(0, 0, 700, 0.5, 4))
    H = H + 0.32 * blob(-600, 80, 420, 0.6, 5)  # floresta a oeste
    return C, T, H, M


def base_noise(seed):
    wx, wz = warp(seed)
    n1 = 0.65 * fbm(wx, wz, 2200, seed + 3) + 0.35 * fbm(wx, wz, 800, seed + 13)
    n2 = 0.65 * fbm(wx, wz, 2600, seed + 4) + 0.35 * fbm(wx, wz, 800, seed + 14)
    return wx, wz, n1 * 1.3, n2 * 1.3, fbm(X, Z, 1500, seed + 5), fbm(X, Z, 700, seed + 6) * 1.4


def edge_ocean(wx, wz, width=0.13, p=6):
    """Continentalidade de um continente quadrado arredondado, com o mar junto à muralha."""
    r = ((np.abs(wx / HALF)) ** p + (np.abs(wz / HALF)) ** p) ** (1 / p)
    return np.clip((1 - width - r) * 4.5, -1, 1)


def concept_1():
    """Continente clássico: frio ao norte, quente ao sul, seco a oeste, úmido a leste."""
    s = 10
    wx, wz, n1, n2, e, w = base_noise(s)
    C = edge_ocean(wx, wz) + 0.25 * fbm(wx, wz, 1800, s + 8) + 0.2
    T = 0.85 * (wz / HALF) + 0.4 * n1
    H = 0.75 * (wx / HALF) + 0.4 * n2
    M = (ridge(X, Z, [(-3200, -1300), (-1500, -1150), (0, -1250), (1800, -1600), (3600, -2400)], 520, s)
         + ridge(X, Z, [(2600, -5200), (4200, -4300), (5200, -2800)], 650, s + 1)
         + ridge(X, Z, [(-5600, -4000), (-4200, -5600)], 500, s + 2)
         + 0.9 * ridge(X, Z, [(1800, 3300), (3500, 4200), (4800, 5600)], 520, s + 3)
         + 0.8 * ridge(X, Z, [(-5400, 3000), (-3600, 4300)], 700, s + 4)
         + 0.55 * ridge(X, Z, [(-4800, -800), (-3800, 300)], 600, s + 5))
    mush = blob(6950, 1500, 360, 0.7, 6) > 0.30
    C = np.where(mush, -0.2, C)
    rivers = [[(300, -1250), (220, -500), (180, 0), (320, 900), (700, 2200), (1400, 3400), (2200, 5600), (2400, 7000)],
              [(-1500, -1150), (-2200, -200), (-3300, 900), (-4600, 1500), (-6900, 1900)],
              [(4200, -4300), (5000, -3000), (6000, -1800), (7000, -1500)],
              [(-4200, -5600), (-2500, -6000), (-1200, -6900)],
              [(-1000, 2500), (-1800, 4000), (-1500, 6900)]]
    labels = [("Serra do Veio", 0, -1650), ("Montes Gelados", 4000, -4600), ("Selva", 4700, 1800),
              ("Terras áridas", -4500, 3600), ("Deserto", -1800, 4400), ("Cogumelos", 6500, 2150)]
    return dict(C=C, T=T, H=H, M=M, E=e, W=w, mush=mush, rivers=rivers, labels=labels,
                title="1 · Continente clássico", text="Frio ao norte, quente ao sul, seco a oeste, úmido a leste; mar em volta. "
                "A Serra do Veio fecha o vale da vila ao norte.")


def concept_2():
    """Vale entre duas serras: a vila no fundo de um vale norte-sul; atrás de cada serra, outro clima."""
    s = 20
    wx, wz, n1, n2, e, w = base_noise(s)
    C = edge_ocean(wx, wz, 0.10) + 0.2 * fbm(wx, wz, 1800, s + 8) + 0.25
    T = 0.9 * (wz / HALF) + 0.35 * n1
    # sombra de chuva: oeste da serra oeste é seco, leste da serra leste é úmido, o vale é ameno
    H = np.where(wx < -2600, -0.55 + 0.25 * n2, np.where(wx > 2600, 0.45 + 0.25 * n2, 0.05 + 0.25 * n2))
    H = 0.6 * H + 0.4 * np.clip(wx / 3500, -1, 1) * 0.6
    west = [(-2400, -6400), (-2700, -3500), (-2200, -1000), (-2600, 1500), (-2300, 4200), (-2800, 6300)]
    east = [(2500, -6300), (2300, -3600), (2700, -800), (2200, 1800), (2700, 4400), (2300, 6300)]
    M = ridge(X, Z, west, 620, s) + ridge(X, Z, east, 620, s + 1)
    M = M * (1 - 0.8 * blob(-2400, -200, 420)) * (1 - 0.8 * blob(2500, 600, 420))   # passos (colos)
    M = M + 0.6 * ridge(X, Z, [(-6000, -3000), (-4500, -1800)], 600, s + 2) + 0.6 * ridge(X, Z, [(5200, 2600), (6200, 4600)], 600, s + 3)
    mush = blob(-6500, -5900, 320, 0.7, 6) > 0.30
    C = np.where(mush, -0.2, C)
    rivers = [[(0, -6200), (250, -3500), (180, 0), (-200, 3000), (100, 7000)],
              [(-2400, -200), (-4300, -100), (-6900, 300)],
              [(2500, 600), (4300, 1200), (6900, 900)],
              [(-2600, 4000), (-4800, 5200), (-6900, 5600)]]
    labels = [("Serra Oeste", -2600, -2500), ("Serra Leste", 2500, -2700), ("Passo do Oeste", -2400, -500),
              ("Terras secas", -5200, 2500), ("Terras úmidas", 5000, -500), ("Vale", 0, -2500)]
    return dict(C=C, T=T, H=H, M=M, E=e, W=w, mush=mush, rivers=rivers, labels=labels,
                title="2 · Vale entre serras", text="A vila no fundo de um vale; duas serras com passos. "
                "Atrás da serra oeste a chuva não chega (deserto, terras áridas); atrás da leste, selva.")


def concept_3():
    """Ilha-mãe e arquipélago: a vila numa ilha grande; os biomas extremos em ilhas por mar."""
    s = 30
    wx, wz, n1, n2, e, w = base_noise(s)
    def island(cx, cz, rx, rz):
        return ((wx - cx) / rx) ** 2 + ((wz - cz) / rz) ** 2
    main = 1 - np.sqrt(island(0, 0, 3600, 3300))
    isles = [(0, -5600, 2600, 1300), (5300, -3800, 1500, 1900), (5600, 3400, 1500, 2200), (0, 5700, 2900, 1200),
             (-5400, 3200, 1500, 2100), (-5400, -3200, 1500, 2000)]
    C = main * 1.2
    for cx, cz, rx, rz in isles:
        C = np.maximum(C, (1 - np.sqrt(island(cx, cz, rx, rz))) * 1.2)
    C = np.clip(C, -1, 1) - 0.12 + 0.25 * fbm(wx, wz, 1500, s + 8)
    C = np.minimum(C, edge_ocean(wx, wz, 0.03) + 0.4)
    T = 0.85 * (wz / HALF) + 0.4 * n1
    H = 0.7 * (wx / HALF) + 0.4 * n2
    M = (ridge(X, Z, [(-1800, -1500), (0, -1300), (1900, -1700)], 500, s)
         + ridge(X, Z, [(-1500, -5800), (1500, -5500)], 450, s + 1)
         + 0.8 * ridge(X, Z, [(5200, -4800), (5500, -2900)], 450, s + 2)
         + 0.8 * ridge(X, Z, [(-5600, 2400), (-5200, 4000)], 500, s + 3)
         + 0.6 * ridge(X, Z, [(5500, 2600), (5700, 4500)], 450, s + 4)
         + 0.5 * ridge(X, Z, [(-2600, 1200), (-2000, 2600)], 450, s + 5))
    mush = blob(-6300, 6300, 380, 0.7, 6) > 0.30
    C = np.where(mush, -0.15, C)
    rivers = [[(150, -1300), (180, 0), (300, 1500), (600, 3000)], [(-1800, -1500), (-2700, -300), (-3300, 500)],
              [(1900, -1700), (2900, -800), (3500, 200)], [(0, -5600), (-900, -5100), (-2300, -4900)],
              [(-5500, 3000), (-6200, 3900)]]
    labels = [("Ilha-mãe", 0, 2200), ("Ilha do Gelo", 0, -6300), ("Ilha Fria", 5300, -5900), ("Ilha da Selva", 5600, 4800),
              ("Ilha do Deserto", 0, 6500), ("Ilha Árida", -5600, 4700), ("Ilha da Taiga", -5400, -5000), ("Cogumelos", -6000, 6900)]
    return dict(C=C, T=T, H=H, M=M, E=e, W=w, mush=mush, rivers=rivers, labels=labels,
                title="3 · Ilha-mãe e arquipélago", text="A vila numa ilha grande e temperada; os climas extremos ficam "
                "em ilhas do outro lado do mar. Chegar lá pede barco: a distância vira dificuldade.")


def concept_4():
    """Anéis: o centro é ameno; quanto mais longe, mais extremo. A serra em anel tem passos."""
    s = 40
    wx, wz, n1, n2, e, w = base_noise(s)
    r = np.hypot(wx, wz) / HALF
    ang = np.arctan2(wz, wx)
    C = edge_ocean(wx, wz, 0.11) + 0.2 * fbm(wx, wz, 1800, s + 8) + 0.2
    ext = np.clip((r - 0.28) / 0.5, 0, 1)                     # 0 no centro, 1 na borda
    T = ext * 0.95 * np.sin(ang) + 0.3 * n1                  # norte (z<0) frio, sul quente
    H = ext * 0.85 * np.cos(ang) + 0.35 * n2                  # oeste seco, leste úmido
    ring = np.exp(-((np.hypot(X, Z) - 2300) / 380) ** 2) * (0.7 + 0.5 * (1 - np.abs(fbm(X, Z, 900, s + 5, 4))))
    for a in (0.6, 2.3, 3.9, 5.4):                            # quatro passos no anel
        ring = ring * (1 - 0.9 * blob(2300 * math.cos(a), 2300 * math.sin(a), 380))
    M = 1.05 * ring + 0.8 * ridge(X, Z, [(-5600, -4300), (-3600, -5800)], 600, s + 1) \
        + 0.8 * ridge(X, Z, [(4300, -5400), (5800, -3800)], 600, s + 2) + 0.8 * ridge(X, Z, [(-5800, 3600), (-4200, 5600)], 600, s + 3) \
        + 0.6 * ridge(X, Z, [(4400, 4800), (5900, 3400)], 600, s + 4)
    mush = blob(6600, 6600, 360, 0.7, 6) > 0.30
    C = np.where(mush, -0.2, C)
    rivers = [[(200, -2300), (180, 0), (250, 1200), (1600, 1650)], [(-300, -2700), (-700, -4600), (-400, -6900)],
              [(1600, 1650), (3500, 2600), (5200, 3000), (6900, 3000)],
              [(-1600, -1600), (-3600, -2400), (-6900, -2600)],
              [(-1700, 1700), (-3000, 3800), (-3600, 6900)]]
    labels = [("Anel da Serra", -1200, -2300), ("Centro ameno", 0, 900), ("Norte gelado", 0, -5800),
              ("Sul quente", 0, 5600), ("Oeste seco", -5600, 0), ("Leste úmido", 5300, 0)]
    return dict(C=C, T=T, H=H, M=M, E=e, W=w, mush=mush, rivers=rivers, labels=labels,
                title="4 · Anéis de dificuldade", text="O centro é ameno e cada anel para fora é mais extremo. Uma serra em "
                "anel, com quatro passos, separa o começo do resto do mundo.")


def concept_5():
    """Costa oeste: mar a oeste, muralha de montanhas a leste; clima em diagonal."""
    s = 50
    wx, wz, n1, n2, e, w = base_noise(s)
    coast = -2600 + 700 * fbm(Z, Z * 0 + 3, 2500, s + 9, 4)
    C = np.clip((wx - coast) / 1800, -1, 1) * 0.9 + 0.15 * fbm(wx, wz, 1500, s + 8)
    C = np.minimum(C, edge_ocean(wx, wz, 0.05) + 0.5)
    T = 0.75 * (wz / HALF) + 0.25 * (wx / HALF) + 0.35 * n1
    H = -0.9 * np.clip((wx - 1200) / 4200, -0.7, 1) + 0.35 * n2
    wall = [(5600, -7200), (5200, -4500), (5800, -1500), (5300, 1500), (5900, 4600), (5500, 7200)]
    M = 1.15 * ridge(X, Z, wall, 900, s) + 0.75 * ridge(X, Z, [(800, -1500), (2500, -1800), (3800, -2900)], 480, s + 1) \
        + 0.6 * ridge(X, Z, [(2000, 3800), (3600, 5200)], 520, s + 2) + 0.5 * ridge(X, Z, [(-1200, -5200), (900, -6200)], 500, s + 3)
    mush = blob(-6400, 400, 380, 0.7, 6) > 0.30
    C = np.where(mush, -0.2, C)
    rivers = [[(5300, -2600), (3200, -1200), (1500, -700), (200, -500), (180, 0), (-900, 600), (-2600, 900)],
              [(5200, 2400), (3200, 1900), (1400, 2600), (-600, 3200), (-2700, 3500)],
              [(5000, -5600), (2800, -5000), (500, -4300), (-2400, -4400)],
              [(5400, 6000), (2600, 6400), (-1000, 6600), (-2600, 6800)]]
    labels = [("Mar do Oeste", -5000, -2500), ("Muralha de Pedra", 5600, -400), ("Costa", -2300, 1800),
              ("Interior seco", 3600, 3800), ("Ilha dos cogumelos", -6300, 1000)]
    return dict(C=C, T=T, H=H, M=M, E=e, W=w, mush=mush, rivers=rivers, labels=labels,
                title="5 · Costa oeste", text="Mar a oeste, uma muralha de montanhas a leste fecha o mundo. Rios descem "
                "da muralha ao mar; o litoral é úmido e o interior, atrás das serras, seco.")


# ---------- desenho ----------

def font(size):
    for path in ("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", "/usr/share/fonts/dejavu/DejaVuSans.ttf"):
        if Path(path).exists():
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def to_px(x, z):
    return ((x + HALF) / BLOCK, (z + HALF) / BLOCK)


def jitter(points, seed):
    """Rio com curvas: subdivide e desloca os pontos com ruído."""
    out = []
    for (ax, az), (bx, bz) in zip(points, points[1:]):
        steps = max(2, int(math.hypot(bx - ax, bz - az) / 120))
        nx, nz = -(bz - az), bx - ax
        norm = math.hypot(nx, nz) or 1
        for i in range(steps):
            t = i / steps
            x, z = ax + (bx - ax) * t, az + (bz - az) * t
            off = 260 * float(value_noise(np.array(x), np.array(z), 700, seed)) + 90 * float(value_noise(np.array(x), np.array(z), 200, seed + 1))
            out.append((x + nx / norm * off, z + nz / norm * off))
    out.append(points[-1])
    return out


def caves(C, H, M, E, cls):
    """Biomas do subsolo, no espírito do vanilla: espeleotemas longe do mar, exuberantes onde é
    úmido, a escuridão profunda sob as serras (e longe da vila, para a cidade ancestral)."""
    far = np.hypot(X, Z) > 1500
    drip = (C > 0.55) & (fbm(X, Z, 700, 91) > 0.15)
    lush = (H > 0.32) & (fbm(X, Z, 700, 92) > 0.1) & (C > -0.1)
    dark = (M > 0.55) & far & (fbm(X, Z, 800, 93) > -0.1)
    return drip, lush, dark


def render(concept, out_dir, index):
    C, T, H, M, E, W = (concept[k] for k in ("C", "T", "H", "M", "E", "W"))
    # serras com bordas irregulares e colinas soltas pelo continente (longe da vila)
    hills = 0.8 * np.clip(fbm(X, Z, 1300, 500 + index) - 0.12, 0, 1) * (1 - blob(0, 0, 900))
    M = M * (0.8 + 0.4 * fbm(X, Z, 500, 600 + index, 3)) + hills
    # várzeas: faixa baixa e úmida ao longo dos rios, onde nascem pântanos e manguezais
    flood = Image.new("L", (N, N), 0)
    fd = ImageDraw.Draw(flood)
    for i, pts in enumerate(concept["rivers"]):
        fd.line([to_px(x, z) for x, z in jitter(pts, 300 + index * 10 + i)], fill=255, width=40)
    plain = (np.array(flood) > 0) & (M < 0.3) & (np.hypot(X, Z) > 700) & (fbm(X, Z, 600, 700 + index) > -0.15)
    land = C >= -0.12
    T = equalize(T, land, [-0.45, -0.15, 0.2, 0.55])
    H = equalize(H, land, [-0.35, -0.1, 0.1, 0.3])
    C, T, H, M = village_patch(C, T, H, M)
    cls, tband = classify(C, T, H, M, E, W, concept["mush"])
    h = np.digitize(H, [-0.35, -0.1, 0.1, 0.3])
    swamp = plain & (C >= -0.04) & (h >= 2) & (tband >= 1)
    cls = np.where(swamp & (tband <= 2), INDEX["swamp"], np.where(swamp & (tband >= 3), INDEX["mangrove_swamp"], cls))
    # rios
    river_mask = Image.new("L", (N, N), 0)
    draw = ImageDraw.Draw(river_mask)
    river_lines = []
    for i, pts in enumerate(concept["rivers"]):
        line = [to_px(x, z) for x, z in jitter(pts, 300 + index * 10 + i)]
        river_lines.append(line)
        draw.line(line, fill=255, width=3 if i == 0 else 2)
    rmask = (np.array(river_mask) > 0) & (C >= -0.12)
    cls = np.where(rmask & (tband == 0), INDEX["frozen_river"], np.where(rmask, INDEX["river"], cls))
    # relevo para o sombreamento
    height = np.where(C < -0.12, -30 + 60 * C, 10 + 30 * np.clip(C, 0, 1)) + 160 * np.clip(M, 0, 1.3) ** 1.4 \
        + 6 * fbm(X, Z, 300, 77, 3) + np.where(rmask, -6, 0)
    gz, gx = np.gradient(height)
    shade = np.clip(0.82 + (-gx * 0.6 - gz * 0.6) / 18, 0.45, 1.25)
    rgb = COLORS[cls] * shade[..., None]
    rgb = np.clip(rgb, 0, 255).astype(np.uint8)
    img = Image.fromarray(rgb, "RGB")
    d = ImageDraw.Draw(img)
    # zona protegida da vila (sem ravinas nem bocas de caverna) e marcos
    cx, cz = to_px(0, 0)
    r = 600 / BLOCK
    d.ellipse([cx - r, cz - r, cx + r, cz + r], outline=(255, 255, 255), width=2)
    d.regular_polygon((cx, cz, 9), 5, fill=(255, 215, 0), outline=(0, 0, 0))
    f = font(15)
    for text, x, z in concept["labels"]:
        px, pz = to_px(x, z)
        d.text((px, pz), text, fill=(255, 255, 255), font=f, anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0))
    d.text((cx + 14, cz - 14), "Vila", fill=(255, 235, 120), font=font(16), anchor="lm", stroke_width=3, stroke_fill=(0, 0, 0))
    d.rectangle([0, 0, N - 1, N - 1], outline=(30, 30, 30), width=4)   # muralha de bedrock
    # quadro final: mapa + painel lateral
    panel = Image.new("RGB", (N + 420, N + 90), (24, 26, 30))
    panel.paste(img, (0, 90))
    pd = ImageDraw.Draw(panel)
    pd.text((16, 14), concept["title"], fill=(255, 255, 255), font=font(30))
    pd.text((16, 54), concept["text"], fill=(200, 205, 215), font=font(15))
    # escala
    pd.line([(30, N + 60), (30 + 1000 / BLOCK, N + 60)], fill=(255, 255, 255), width=4)
    pd.text((30, N + 66), "1.000 blocos", fill=(255, 255, 255), font=font(14))
    # subsolo em miniatura
    drip, lush, dark = caves(C, H, M, E, cls)
    sub = np.full((N, N, 3), 60, dtype=np.uint8)
    sub[C < -0.12] = (25, 35, 60)
    sub[drip] = CAVES[0][1]; sub[lush] = CAVES[1][1]; sub[dark] = CAVES[2][1]
    small = Image.fromarray(sub, "RGB").resize((380, 380), Image.NEAREST)
    sd = ImageDraw.Draw(small)
    sd.ellipse([190 - 600 / BLOCK * 0.38, 190 - 600 / BLOCK * 0.38, 190 + 600 / BLOCK * 0.38, 190 + 600 / BLOCK * 0.38], outline=(255, 255, 255))
    panel.paste(small, (N + 20, 130))
    pd.text((N + 20, 100), "Subsolo (cavernas especiais)", fill=(255, 255, 255), font=font(17))
    y = 520
    for _, color, name in CAVES:
        pd.rectangle([N + 20, y, N + 36, y + 16], fill=color, outline=(200, 200, 200)); pd.text((N + 44, y), name, fill=(220, 220, 220), font=font(14)); y += 24
    pd.text((N + 20, y + 4), "Fora delas: cavernas, minérios e\naquíferos normais do Minecraft.", fill=(170, 175, 185), font=font(13))
    # cobertura
    present = set(np.unique(cls).tolist())
    counts = np.bincount(cls.ravel(), minlength=len(BIOMES))
    missing = [BIOMES[i][2] for i in range(len(BIOMES)) if counts[i] == 0]
    cave_ok = [drip.any(), lush.any(), dark.any()]
    total = len(present) + sum(cave_ok)
    y += 60
    pd.text((N + 20, y), f"Biomas: {total} de {len(BIOMES) + 3}", fill=(140, 230, 140) if not missing else (240, 180, 90), font=font(20))
    y += 30
    rare = sorted(((counts[i] * BLOCK * BLOCK / 1e6, BIOMES[i][2]) for i in range(len(BIOMES)) if counts[i] > 0))[:6]
    pd.text((N + 20, y), "Os menores (km²):", fill=(200, 200, 200), font=font(14)); y += 20
    for area, name in rare:
        pd.text((N + 28, y), f"{name}: {area:.1f}", fill=(170, 175, 185), font=font(13)); y += 18
    if missing:
        pd.text((N + 20, y + 6), "Faltam: " + ", ".join(missing), fill=(240, 150, 90), font=font(13))
    pd.text((N + 20, N + 40), "Estrela: a vila. Círculo branco: 600 blocos\nsem ravinas nem bocas de caverna.\nBorda escura: a muralha de bedrock.",
            fill=(170, 175, 185), font=font(13))
    panel.save(out_dir / f"mapa_{index}.png")
    return counts, missing, cave_ok


def legend(out_dir):
    cols, rowh = 3, 26
    rows = math.ceil(len(BIOMES) / cols)
    img = Image.new("RGB", (cols * 330 + 20, rows * rowh + 70), (24, 26, 30))
    d = ImageDraw.Draw(img)
    d.text((14, 12), "Legenda: os 50 biomas da superfície (mais 3 do subsolo em cada mapa)", fill=(255, 255, 255), font=font(20))
    for i, (_, color, name) in enumerate(BIOMES):
        c, r = divmod(i, rows)
        x, y = 14 + c * 330, 54 + r * rowh
        d.rectangle([x, y, x + 18, y + 18], fill=color, outline=(200, 200, 200))
        d.text((x + 26, y), name, fill=(220, 220, 220), font=font(15))
    img.save(out_dir / "legenda.png")


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("mapa_previas")
    out_dir.mkdir(parents=True, exist_ok=True)
    report = []
    for i, make in enumerate([concept_1, concept_2, concept_3, concept_4, concept_5], 1):
        concept = make()
        counts, missing, cave_ok = render(concept, out_dir, i)
        report.append(f"{concept['title']}: {np.count_nonzero(counts)}/{len(BIOMES)} superfície, subsolo {sum(cave_ok)}/3"
                      + (f"; faltam {', '.join(missing)}" if missing else ""))
        print(report[-1], flush=True)
    legend(out_dir)
    # folha com os cinco lado a lado (só os mapas)
    thumbs = [Image.open(out_dir / f"mapa_{i}.png").crop((0, 0, N, N + 90)).resize((480, 523)) for i in range(1, 6)]
    sheet = Image.new("RGB", (480 * 5 + 40, 560), (24, 26, 30))
    for i, thumb in enumerate(thumbs):
        sheet.paste(thumb, (8 + i * 488, 20))
    sheet.save(out_dir / "comparacao.png")
    (out_dir / "cobertura.txt").write_text("\n".join(report) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
