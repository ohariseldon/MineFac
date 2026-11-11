"""Mapa 5, "Costa oeste": a planta final do mundo inteiro (15.000 x 15.000 blocos).

Uma costa de Pacífico a oeste (o "Chile" da vila) e uma muralha de gelo a leste (os "Andes"); do
outro lado da muralha, uma bacia úmida em que os rios correm para leste (a "Amazônia" espelhada).

- Costa: fiordes e penhascos ao norte, dunas com lagoas (os "Lençóis") descendo até a foz do Rio
  Veio, que entra na terra como uma fenda larga com um delta e uma ilha grande; ao sul, a Ponta do
  Paredão e o Mar das Ilhas (um Caribe de águas rasas, recifes e ilhotas); a foz do rio do sul é
  uma lagoa atrás de uma restinga. A oeste, no mar aberto, a Ilha da Bruma.
- Rios em árvore: cada afluente recebe os seus; a largura cresce com a vazão acumulada. Nascentes
  nas geleiras e nas serras, cachoeiras onde o rio desce um degrau, leitos secos onde um rio morreu.
- Clima: frio ao norte, quente ao sul; a Serra do Veio é a divisa entre a Mata Escura e o Pinhal
  do Norte; depois das praias do sul vem o deserto; a bacia leste é úmida.

Os biomas saem da mesma tabela das prévias (`preview_maps.classify`), com o equilíbrio por faixa de
clima. Uso:

    python3 tools/mapa/costa_oeste.py [pasta de saída] [pixels]

Gera mapa_final.png (o mapa inteiro, 4 blocos por pixel com 3750) e mapa_final_previa.jpg; no repositório
fica uma cópia em JPEG (docs/mapa/costa-oeste/mapa_final.jpg).
É a planta que o gerador do jogo vai seguir; não é o gerador.
"""
import math
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFilter, ImageFont

sys.path.insert(0, str(Path(__file__).parent))
import preview_maps as pm  # noqa: E402

HALF = pm.HALF
FONT_DIR = Path("/usr/share/fonts/truetype/dejavu")


def font(size, kind="sans"):
    name = {"sans": "DejaVuSans.ttf", "bold": "DejaVuSans-Bold.ttf", "serif": "DejaVuSerif.ttf",
            "serifb": "DejaVuSerif-Bold.ttf"}[kind]
    path = FONT_DIR / name
    return ImageFont.truetype(str(path), size) if path.exists() else ImageFont.load_default()


# ====================================================================================================
# Geografia (blocos: x leste, z sul)
# ====================================================================================================

# O continente só tem mar a oeste: ao norte, a leste e ao sul ele segue até a borda do mundo (a
# muralha de bedrock corta a terra). A costa é uma linha aberta, do sul ao norte; só ela tem contorno
# irregular. No noroeste fica o Golfo do Norte.
COASTLINE = [
    (-1400, 7700), (-1450, 7350),
    # o Mar das Ilhas (Caribe): costa recortada de baías pequenas
    (-1700, 6700), (-2050, 6350), (-2300, 6050), (-2150, 5650), (-2250, 5300), (-2600, 4950), (-2450, 4550),
    (-2400, 3950), (-2650, 3600), (-2800, 3250),
    # a Ponta do Paredão, esticada para o mar
    (-2950, 2880), (-3600, 2720), (-4300, 2640), (-4800, 2470), (-5050, 2260), (-4800, 2060), (-4150, 1990),
    (-3450, 1910), (-3000, 1760),
    # a foz do Rio Veio: uma fenda larga que entra na terra até perto da vila
    (-2850, 1520), (-2350, 1360), (-1950, 1190), (-1600, 1050), (-1350, 965), (-1520, 880), (-1950, 770), (-2420, 630), (-2900, 460),
    # os Lençóis: costa reta de dunas
    (-3000, 250), (-3060, -400), (-3150, -1100), (-3110, -1800),
    # penhascos e fiordes
    (-3250, -2600), (-3420, -3300), (-3420, -4000), (-3520, -5000), (-3460, -6000), (-3380, -6600),
    # o Golfo do Norte; a leste dele, terra até a borda
    (-3000, -6760), (-2250, -6830), (-1500, -6980), (-1150, -7300), (-1050, -7700),
]
# Fora do mapa: fecha o continente sem mexer (a muralha corta a terra antes daqui).
OUTSIDE = [(-1050, -9000), (9000, -9000), (9000, 9000), (-1400, 9000)]

# fiordes: (de, até, largura na boca) — água entrando no planalto
FJORDS = [((-3600, -3720), (-2350, -3860), 170), ((-3650, -4470), (-2080, -4420), 260),
          ((-3600, -5480), (-2500, -5720), 150), ((-3500, -6350), (-2700, -6480), 120)]

# ilhas: (centro, raios, irregularidade, nome ou None)
ISLANDS = [
    ((-5750, -520), (720, 520), 0.45, "bruma"),          # a Ilha da Bruma (cogumelos, misteriosa)
    ((-2780, 1000), (230, 150), 0.35, "grande"),         # a Ilha Grande, na boca da foz
    ((-1720, 960), (95, 60), 0.4, None), ((-2080, 1060), (120, 70), 0.4, None), ((-2230, 860), (90, 60), 0.4, None),
    ((-1950, 950), (70, 45), 0.4, None),                  # ilhas do delta
    ((-4150, 4300), (430, 300), 0.4, "tartarugas"),      # Mar das Ilhas
    ((-3420, 5520), (170, 140), 0.4, "farol"),
    ((-3700, 3700), (70, 55), 0.5, None), ((-4820, 5180), (95, 70), 0.5, None), ((-3120, 4920), (55, 45), 0.5, None),
    ((-4520, 3480), (45, 40), 0.5, None), ((-5200, 4420), (75, 55), 0.5, None), ((-4650, 4800), (35, 30), 0.5, None),
    ((-3950, 5950), (60, 40), 0.5, None),
    ((-4120, -5000), (130, 90), 0.4, None), ((-3900, -6200), (90, 70), 0.4, None), ((-4300, -3150), (65, 50), 0.4, None),
]
LAGOON = ((300, 6660), (950, 240))      # a Lagoa da Restinga (foz do rio do sul)
# o Pantanal do Leste: onde os rios da bacia leste terminam, num lago raso cercado de brejo
PANTANAL = ((6450, 420), (420, 230))
PANTANAL_WET = ((6350, 450), (1000, 650))
LAKES = [((600, -5300), (320, 200)), ((-1720, -720), (130, 90)), ((1800, 5220), (55, 40)),
         ((6250, -1550), (80, 35)), ((6100, 2700), (90, 40)), ((5500, -4200), (110, 60)), ((-300, -2900), (140, 90))]
# os Lençóis: faixa de dunas com lagoas entre a foz e os penhascos
DUNES = dict(x0=-3200, x1=-2350, z0=-1750, z1=300)

# ---------- serras (pontos, largura, altura, semente) ----------
RIDGES = [
    ([(4500, -7200), (4300, -5600), (4700, -4000), (4350, -2400), (4650, -800), (4300, 800), (4700, 2400),
      (4350, 4000), (4700, 5600), (4400, 7200)], 600, 1.25, 501),                        # Muralha de Gelo
    ([(700, -1650), (1600, -1800), (2600, -1700), (3600, -1900), (4300, -1800)], 400, 0.9, 502),   # Serra do Veio
    ([(-650, -420), (250, -450), (800, -520)], 170, 0.55, 57),                            # Morro da Mina
    ([(-1500, -5900), (0, -6200), (1500, -5900)], 450, 0.75, 503),                      # Montes do Norte
    ([(1500, 3900), (2600, 4700), (3300, 5800)], 420, 0.8, 504),                         # Serra do Sul
    ([(5300, -6000), (5400, -3000), (5250, 0), (5400, 3000), (5300, 6000)], 300, 0.35, 505),   # contrafortes leste
    ([(-4800, 2250), (-3200, 2280)], 170, 0.55, 506),                                    # Ponta do Paredão
]

# ---------- rios: (nome, pontos, rio-pai) — o último ponto de um afluente é um ponto do pai ----------
RIVERS = [
    # Rio Veio: da geleira até o delta
    # O Rio Veio corre entre a Serra do Veio e o Morro da Mina e atravessa o morro no Passo do Rio
    ("Rio Veio", [(4250, -1350), (3600, -1250), (3200, -1200), (2400, -1080), (1500, -1000), (900, -860), (450, -700),
                  (230, -480), (110, 0), (170, 420), (-300, 820), (-900, 940), (-1350, 965)], None),
    ("Ribeirão da Serra", [(2600, -1560), (2300, -1330), (2000, -1180), (1500, -1000)], "Rio Veio"),
    ("Córrego do Gelo", [(3350, -1620), (2800, -1420), (2300, -1330)], "Ribeirão da Serra"),
    ("Rio Claro", [(4300, 600), (3600, 300), (2800, 100), (1900, -40), (1000, 180), (170, 420)], "Rio Veio"),
    ("Arroio da Pedra", [(4400, -250), (3800, -100), (3600, 300)], "Rio Claro"),
    ("Arroio Fundo", [(3500, 1200), (3000, 700), (2800, 100)], "Rio Claro"),
    ("Córrego do Morro", [(-300, -700), (0, -640), (230, -480)], "Rio Veio"),
    ("Rio das Bétulas", [(1800, 2000), (1200, 1600), (500, 1200), (-300, 820)], "Rio Veio"),
    ("Sanga Clara", [(2100, 2380), (1600, 2150), (1000, 1850), (500, 1200)], "Rio das Bétulas"),
    ("Riacho da Mata", [(-1300, -1600), (-1200, -800), (-1050, 0), (-950, 500), (-900, 940)], "Rio Veio"),
    ("Grota Escura", [(-2000, -1350), (-1650, -1150), (-1200, -800)], "Riacho da Mata"),
    # Rio do Sul: da muralha até a Lagoa da Restinga
    ("Rio do Sul", [(4300, 2600), (3600, 2500), (2900, 2700), (2000, 3000), (1000, 3500), (300, 4300), (-100, 5200),
                    (0, 6000), (200, 6450)], None),
    ("Rio Seco de Cima", [(4400, 3600), (3700, 3500), (2900, 2700)], "Rio do Sul"),
    ("Córrego Vermelho", [(4600, 4300), (4000, 4100), (3700, 3500)], "Rio Seco de Cima"),
    ("Rio Manso", [(3600, 1600), (2800, 2000), (1800, 2750), (1000, 3500)], "Rio do Sul"),
    ("Sanga do Oásis", [(1800, 5200), (1200, 4700), (300, 4300)], "Rio do Sul"),
    ("Arroio das Palmas", [(-500, 3700), (-1300, 3600), (-2300, 3750)], None),     # desce direto ao Mar das Ilhas
    # Rio do Norte: da geleira até o fiorde (cachoeira)
    ("Rio do Norte", [(4300, -5200), (3200, -5000), (2000, -4800), (800, -4500), (-500, -4400), (-1500, -4450), (-2080, -4420)], None),
    ("Riacho dos Montes", [(2600, -6300), (2000, -5600), (2000, -4800)], "Rio do Norte"),
    # Grande Rio do Leste: a bacia do outro lado da muralha, correndo para leste
    ("Grande Rio do Leste", [(5200, -4600), (5600, -3000), (5800, -1500), (5700, -300), (6060, 300)], None),
    ("Rio do Sul do Leste", [(5200, 5200), (5600, 3800), (5800, 2200), (5700, 1000), (6060, 300)], "Grande Rio do Leste"),
    ("Igarapé do Gelo", [(4800, -3800), (5300, -3500), (5600, -3000)], "Grande Rio do Leste"),
    ("Igarapé Alto", [(4800, -2200), (5300, -1900), (5800, -1500)], "Grande Rio do Leste"),
    ("Igarapé Fino", [(5000, -2700), (5200, -2300), (5300, -1900)], "Igarapé Alto"),
    ("Igarapé da Névoa", [(4900, -900), (5300, -700), (5700, -300)], "Grande Rio do Leste"),
    ("Rio do Norte do Leste", [(6600, -4600), (6000, -3600), (5600, -3000)], "Grande Rio do Leste"),
    ("Igarapé das Palmeiras", [(4900, 1700), (5300, 1900), (5800, 2200)], "Rio do Sul do Leste"),
    ("Igarapé do Jacaré", [(6700, 2900), (6200, 2500), (5800, 2200)], "Rio do Sul do Leste"),
    ("Igarapé da Onça", [(4900, 3300), (5300, 3500), (5600, 3800)], "Rio do Sul do Leste"),
    ("Igarapé Miúdo", [(6600, 4800), (6100, 4300), (5600, 3800)], "Rio do Sul do Leste"),
]
# braços do delta: do ápice até a água da foz
DELTA = [[(-1350, 965), (-1650, 900), (-1980, 800)], [(-1350, 965), (-1700, 1040), (-2050, 1150)], [(-1350, 965), (-1620, 965), (-1850, 975)]]
# leitos secos: rios que morreram
DRY = [[(2600, 5600), (1600, 5900), (800, 6350)], [(4200, -6250), (3400, -6000), (2800, -6150)], [(-2000, 5200), (-1300, 5600), (-900, 6300)]]

# ---------- marcos ----------
WATERFALLS = [
    ("Cataratas do Fiorde", -2080, -4420), ("Véu da Serra", 2550, -1500), ("Salto da Muralha", 4950, -2170),
    ("Salto das Palmeiras", 5000, 1760), ("Cachoeira do Passo", 230, -480), ("Cascata do Paredão", -4250, 2420),
    ("Salto Vermelho", 4350, 3560), ("Queda do Gelo", 4150, -5150), ("Cachoeira dos Montes", 2500, -6150),
]
RAVINES = [  # (nome, centro, comprimento, ângulo em graus)
    ("Fenda da Serra", (2100, -1150), 320, 35), ("Cânion Vermelho", (2700, 4950), 700, 60),
    ("Fenda Gelada", (1200, -5550), 380, 10), ("Garganta do Paredão", (-4000, 2300), 220, 80),
    ("Fenda do Pinhal", (-1800, -3400), 300, 120), ("Grotão do Leste", (6500, -2400), 280, 20),
]
# Estruturas: (nome, x, z, tipo, biomas aceitos). Tipos: "mc" (do Minecraft), "veio" (estrutura
# própria do Veio, ligada à história; ideias de estruturas da comunidade adaptadas) e "sub" (no
# subsolo). Os biomas seguem as regras do Minecraft: `place_structures` confere cada uma e, se o
# ponto não serve, procura o lugar válido mais perto (até 1.500 blocos).
VILLAGE_BIOMES = ("plains", "sunflower_plains", "meadow")
SNOWY = ("snowy_plains", "snowy_taiga", "snowy_slopes", "ice_spikes")
OCEANS = ("ocean", "deep_ocean", "cold_ocean", "deep_cold_ocean", "lukewarm_ocean", "deep_lukewarm_ocean", "warm_ocean",
          "frozen_ocean", "deep_frozen_ocean")
DEEP = ("deep_ocean", "deep_cold_ocean", "deep_lukewarm_ocean", "deep_frozen_ocean")
OUTPOST = ("plains", "sunflower_plains", "desert", "savanna", "taiga", "snowy_plains", "meadow", "grove", "snowy_slopes", "cherry_grove")
TAIGAS = ("taiga", "snowy_taiga", "old_growth_pine_taiga", "old_growth_spruce_taiga", "old_growth_birch_forest", "jungle")
LAND = None
STRUCTURES = [
    # vilas do Minecraft (vizinhas para o comércio do capítulo 7), todas a mais de 1.500 blocos da vila
    ("Vila dos Campos", 1300, -3000, "mc", VILLAGE_BIOMES), ("Vila do Planalto", 2800, -2600, "mc", VILLAGE_BIOMES),
    ("Vila do deserto", 2300, 6000, "mc", ("desert",)), ("Vila do Oásis", 1500, 4900, "mc", ("desert",)),
    ("Vila da savana", 1700, 2900, "mc", ("savanna", "savanna_plateau")), ("Vila da Chapada", 3300, 3200, "mc", ("savanna", "savanna_plateau")),
    ("Vila da taiga", -700, -3800, "mc", ("taiga", "snowy_taiga")), ("Vila do Leste", 6300, -3300, "mc", ("taiga", "snowy_taiga")),
    ("Vila nevada", 2100, -5900, "mc", ("snowy_plains",)),
    # templos, mansão, cabanas
    ("Templo do deserto", 1300, 5650, "mc", ("desert",)), ("Pirâmide do Sol", 3600, 6400, "mc", ("desert",)),
    ("Templo da selva", 6250, 4300, "mc", ("jungle", "bamboo_jungle")), ("Templo do Bambu", 6400, 2000, "mc", ("jungle", "bamboo_jungle")),
    ("Mansão da floresta", -1200, -1500, "mc", ("dark_forest",)),
    ("Cabana da bruxa", -1300, 1100, "mc", ("swamp", "mangrove_swamp")), ("Cabana do Pantanal", 6100, 700, "mc", ("swamp", "mangrove_swamp")),
    ("Iglu", -300, -5650, "mc", SNOWY), ("Iglu do Planalto", 2700, -4800, "mc", SNOWY),
    ("Posto dos pillagers", 3400, -2300, "mc", OUTPOST), ("Posto da savana", 2900, 2350, "mc", OUTPOST),
    ("Ruínas de trilha", -1600, -3200, "mc", TAIGAS), ("Ruínas da selva", 6500, 3000, "mc", TAIGAS),
    ("Portal em ruínas", 3300, -3700, "mc", LAND), ("Portal da Serra do Sul", 2400, 5200, "mc", LAND), ("Portal do Fiorde", -2600, -5200, "mc", LAND),
    ("Poço do deserto", 600, 5300, "mc", ("desert",)), ("Poço das Dunas", 2800, 6600, "mc", ("desert",)),
    # mar
    ("Monumento oceânico", -6500, -2100, "mc", DEEP), ("Monumento do Sul", -6200, 5600, "mc", DEEP),
    ("Ruínas submersas", -4550, 3850, "mc", OCEANS), ("Ruínas do Golfo", -3200, -7100, "mc", OCEANS),
    ("Navio naufragado", -3600, 4900, "mc", OCEANS), ("Naufrágio da Bruma", -5000, -900, "mc", OCEANS),
    ("Naufrágio na praia", -2600, 5600, "mc", ("beach",) + OCEANS), ("Tesouro enterrado", -4150, 4560, "mc", ("beach",)),
    # subsolo (sem bioma de superfície)
    ("Cidade ancestral (subsolo)", 4400, -3000, "sub", LAND), ("Câmaras de desafio (subsolo)", 1500, 1400, "sub", LAND),
    ("Câmaras do Norte (subsolo)", -500, -4600, "sub", LAND),
    ("Fortaleza do End (subsolo)", -1700, 1600, "sub", LAND), ("Fortaleza do End (subsolo)", 1800, 1250, "sub", LAND),
    ("Fortaleza do End (subsolo)", 300, -2350, "sub", LAND),
    ("Mina abandonada", 3800, 1200, "sub", LAND), ("Geodo de ametista (subsolo)", -2100, -3000, "sub", LAND),
    ("Fósseis (subsolo)", 900, 6100, "sub", LAND),
    # estruturas do Veio (a história e as ideias da comunidade adaptadas)
    ("Acampamento dos Saqueadores", 2100, 1500, "veio", LAND), ("Cais velho", -1450, 1060, "veio", LAND),
    ("Mina do avô", -40, -300, "veio", LAND),
    ("Farol abandonado", -3420, 5520, "veio", LAND), ("Forte do Paredão (ruínas)", -4600, 2250, "veio", LAND),
    ("Torre de vigia da Serra", 1900, -1950, "veio", LAND), ("Mosteiro da Muralha", 4100, -500, "veio", LAND),
    ("Vila de pescadores abandonada", -2800, -1300, "veio", LAND), ("Ponte de pedra antiga", 900, 300, "veio", LAND),
    ("Moinho velho", 700, 1200, "veio", LAND), ("Observatório dos Montes", 200, -6200, "veio", LAND),
    ("Taverna da Encruzilhada", 600, 2300, "veio", LAND), ("Catacumbas da Mata (subsolo)", -1700, -400, "veio", LAND),
    ("Templo submerso", -4900, 4700, "veio", OCEANS), ("Santuário da Bruma", -5800, -560, "veio", LAND),
]
FAUNA = [  # (bichos, x, z)
    ("lobos e raposas", -1700, -2500), ("ursos-polares", -2800, -6400), ("cabras", 4500, -3600), ("lhamas", 3000, -2300),
    ("pandas", 6300, 3300), ("papagaios e jaguatiricas", 6050, 1500), ("tartarugas", -2600, 5300), ("golfinhos", -5000, 3200),
    ("peixes tropicais", -3700, 4300), ("camelos", 900, 6200), ("tatus", 2400, 3500), ("sapos", -1900, 650),
    ("abelhas", 900, -1300), ("coelhos", -2750, -800), ("mooshrooms", -5750, -300), ("lulas brilhantes", -2700, -3800),
    ("cavalos", -600, 1900), ("axolotes (cavernas)", 5950, -150),
]
PLACES = [  # (nome, x, z, tamanho, estilo)
    ("MAR DO OESTE", -5800, -3200, 34, "water"), ("MAR DAS ILHAS", -4300, 6100, 26, "water"), ("Golfo do Norte", -2300, -7250, 16, "water"),
    ("Pantanal do Leste", 6450, 900, 15, "land"), ("Lago do Pantanal", 6450, 430, 11, "water"), ("Ilha da Bruma", -5750, -1150, 20, "land"), ("Ilha das Tartarugas", -4150, 4750, 15, "land"),
    ("Ilhota do Farol", -3420, 5780, 12, "land"), ("Ilha Grande", -2780, 1230, 13, "land"), ("Ponta do Paredão", -4300, 2150, 15, "land"),
    ("Foz do Veio", -2200, 1450, 15, "water"), ("Lençóis", -2780, -800, 18, "land"), ("Fiordes", -2950, -5000, 18, "land"),
    ("Penhascos do Norte", -2600, -2700, 15, "land"), ("Pinhal do Norte", -1200, -2800, 24, "land"), ("Mata Escura", -1450, -400, 24, "land"),
    ("Serra do Veio", 2300, -2050, 22, "mount"), ("MURALHA DE GELO", 4520, -4700, 22, "mount"), ("Montes do Norte", 0, -6450, 18, "mount"),
    ("Serra do Sul", 2700, 5250, 17, "mount"), ("Lago Gelado", 600, -5000, 13, "water"), ("Planície do Veio", 150, 230, 13, "land"),
    ("Brejo do Rio", -700, 1260, 14, "land"), ("Bosque das Bétulas", 1450, 250, 16, "land"), ("Savana do Sul", 1300, 2600, 18, "land"),
    ("Deserto do Sol", 900, 5900, 24, "land"), ("Terras Vermelhas", 3500, 4650, 18, "land"), ("Lagoa da Restinga", 300, 6420, 13, "water"),
    ("Praias do Sul", -1700, 5600, 15, "land"), ("BACIA DO LESTE", 6250, -800, 18, "land"), ("Selva do Leste", 6250, 3600, 18, "land"),
    ("Grande Rio do Leste", 5450, -2200, 12, "water"), ("Planalto Gelado", 1500, -4300, 18, "land"), ("Taiga Velha", 1600, 900, 14, "land"),
]
RESOURCES = [  # (tipo, x, z, cor) — o subsolo é o do Minecraft; estes o plano reserva
    ("cobre", 66, -38, (215, 120, 70)), ("ferro", 120, -470, (205, 195, 185)), ("carvão", -96, -30, (35, 35, 35)),
    ("argila", -880, 1000, (150, 165, 195)), ("argila", -1900, 900, (150, 165, 195)), ("pedra", -470, -470, (150, 150, 150)),
]

VILLAGE_SAFE = 600


# ====================================================================================================
# Ferramentas
# ====================================================================================================

def fractal(points, seed, rough=0.22, depth=6, closed=True):
    """Contorno natural: cada lado ganha pontos no meio, empurrados para fora ou para dentro."""
    rng = np.random.default_rng(seed)
    pts = list(points)
    for level in range(depth):
        out = []
        n = len(pts) if closed else len(pts) - 1
        for i in range(n):
            a, b = pts[i], pts[(i + 1) % len(pts)]
            out.append(a)
            dx, dz = b[0] - a[0], b[1] - a[1]
            length = math.hypot(dx, dz)
            off = rng.normal(0, rough) * length * (0.6 ** level) if length > 25 else 0
            mx, mz = (a[0] + b[0]) / 2 - dz / max(length, 1) * off, (a[1] + b[1]) / 2 + dx / max(length, 1) * off
            out.append((mx, mz))
        if not closed:
            out.append(pts[-1])
        pts = out
    return pts


def blob_poly(center, radii, rough, seed, n=28):
    cx, cz = center
    rx, rz = radii
    rng = np.random.default_rng(seed)
    pts = []
    for k in range(n):
        a = 2 * math.pi * k / n
        r = 1 + rough * (rng.random() - 0.5)
        pts.append((cx + rx * r * math.cos(a), cz + rz * r * math.sin(a)))
    return fractal(pts, seed + 1, 0.12, 4)


def meander(points, seed, amp=0.18):
    """Rio com curvas naturais; os pontos de controle (nascentes e confluências) ficam fixos."""
    out = []
    for i, ((ax, az), (bx, bz)) in enumerate(zip(points, points[1:])):
        length = math.hypot(bx - ax, bz - az)
        steps = max(3, int(length / 45))
        nx, nz = -(bz - az) / max(length, 1), (bx - ax) / max(length, 1)
        for k in range(steps):
            t = k / steps
            env = math.sin(math.pi * t)
            x, z = ax + (bx - ax) * t, az + (bz - az) * t
            wiggle = float(pm.value_noise(np.array(x), np.array(z), 420, seed)) * 0.7 + float(pm.value_noise(np.array(x), np.array(z), 140, seed + 1)) * 0.3
            off = amp * length * env * wiggle
            out.append((x + nx * off, z + nz * off))
    out.append(points[-1])
    return out


class Canvas:
    """Desenho em coordenadas de blocos sobre uma imagem de N x N."""
    def __init__(self, n, mode="L", fill=0):
        self.img = Image.new(mode, (n, n), fill)
        self.draw = ImageDraw.Draw(self.img)

    def poly(self, pts, fill):
        self.draw.polygon([pm.to_px(x, z) for x, z in pts], fill=fill)

    def line(self, pts, fill, width_blocks):
        self.draw.line([pm.to_px(x, z) for x, z in pts], fill=fill, width=max(1, int(round(width_blocks / pm.BLOCK))), joint="curve")

    def array(self):
        return np.array(self.img)


def logit_distance(mask_img, sigma_blocks):
    """Distância aproximada até a costa (blocos, positiva em terra) a partir de uma borra gaussiana."""
    b = np.array(mask_img.filter(ImageFilter.GaussianBlur(sigma_blocks / pm.BLOCK)), dtype=np.float32) / 255
    b = np.clip(b, 0.002, 0.998)
    return sigma_blocks * np.log(b / (1 - b)) / 1.702


def smooth(arr, radius_blocks):
    """Suaviza um campo: reduz e amplia de novo (o modo de ponto flutuante não aceita borra)."""
    img = Image.fromarray(arr.astype(np.float32), "F")
    n = arr.shape[0]
    k = max(2, int(n * pm.BLOCK / max(radius_blocks, 1)))
    k = min(k, n)
    return np.array(img.resize((k, k), Image.BOX).resize((n, n), Image.BILINEAR), dtype=np.float32)


# ====================================================================================================
# Montagem
# ====================================================================================================

def build(n):
    pm.REPLAY = False
    pm.CUTS.clear()
    pm.set_view(-HALF, -HALF, 2 * HALF, n)
    X, Z = pm.X, pm.Z

    # ---------- terra e água ----------
    land = Canvas(n)
    land.poly(fractal(COASTLINE, 11, 0.16, 6, closed=False) + OUTSIDE, 255)
    for (a, b, w) in FJORDS:
        line = fractal([a, ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2 + 60), b], 21, 0.1, 4, closed=False)
        for k in range(len(line) - 1):
            t = k / max(1, len(line) - 2)
            land.line([line[k], line[k + 1]], 0, w * (1 - 0.75 * t))
    lagoon = blob_poly(*LAGOON, 0.25, 31)
    land.poly(lagoon, 0)
    land.poly(blob_poly(*PANTANAL, 0.35, 33), 0)
    for i, (c, r_) in enumerate([((6000, 800), (120, 60)), ((6800, 0), (150, 80)), ((6150, 120), (90, 50)), ((6900, 900), (110, 70))]):
        land.poly(blob_poly(c, r_, 0.4, 34 + i), 0)
    for i, (c, r) in enumerate(LAKES):
        land.poly(blob_poly(c, r, 0.4, 40 + i), 0)
    rng = np.random.default_rng(5)
    dune_lakes = []
    for _ in range(60):     # lagoas dos Lençóis
        c = (rng.uniform(DUNES["x0"] + 150, DUNES["x1"]), rng.uniform(DUNES["z0"], DUNES["z1"]))
        r = (rng.uniform(15, 55), rng.uniform(10, 35))
        dune_lakes.append((c, r))
        land.poly(blob_poly(c, r, 0.5, 900 + _, 12), 0)
    isl = Canvas(n)
    bruma = Canvas(n)
    for i, (c, r, rough, name) in enumerate(ISLANDS):
        poly = blob_poly(c, r, rough, 60 + i)
        land.poly(poly, 255)
        if name == "bruma":
            bruma.poly(poly, 255)
    mask_img = land.img
    is_land = land.array() > 127
    d_near = logit_distance(mask_img, 40)
    d_mid = logit_distance(mask_img, 500)
    d_far = logit_distance(mask_img, 2200)
    w_mid = np.clip((np.abs(d_near) - 80) / 40, 0, 1)
    w_far = np.clip((np.abs(d_mid) - 900) / 400, 0, 1)
    d = d_near * (1 - w_mid) + (d_mid * (1 - w_far) + d_far * w_far) * w_mid
    d = np.where(is_land, np.maximum(d, 0.5), np.minimum(d, -0.5))

    # ---------- relevo das serras e do planalto ----------
    M = np.zeros_like(X, dtype=np.float32)
    for pts, width, amp, seed in RIDGES:
        M += amp * pm.ridge(X, Z, pts, width, seed)
    rolling = pm.fbm(X, Z, 700, 77, 4)
    hills = 0.75 * np.clip(pm.fbm(X, Z, 1300, 505) - 0.15, 0, 1) * (1 - pm.blob(0, 0, 1100))
    M = M * (0.85 + 0.3 * pm.fbm(X, Z, 500, 605, 3)) + hills
    # planalto costeiro ao norte (penhascos), a Ponta do Paredão e a Ilha da Bruma são altos à beira-mar
    plateau = np.clip((-z_curve(X) - Z - 1800) / 900, 0, 1) * np.clip((-1000 - X) / 1500, 0, 1)
    plateau = np.maximum(plateau, np.clip(1 - np.hypot((X + 4100) / 900, (Z - 2280) / 300), 0, 1))
    is_bruma = np.array(bruma.img) > 127
    plateau = np.maximum(plateau, np.where(is_bruma, np.clip(1 - np.hypot((X + 5750) / 700, (Z + 520) / 500), 0, 1), 0))

    # ---------- continentalidade ----------
    C = np.where(d >= 0, -0.12 + np.minimum(d, 30) / 30 * 0.08 + np.clip((d - 30) / 2600, 0, 1) * 0.92,
                 -0.12 + np.maximum(d, -3500) / 3500 * 0.88 + 0.12 * pm.fbm(X, Z, 900, 815, 4) * np.clip(-d / 400, 0, 1))
    # ---------- relevo de base e os vales que os rios cavam ----------
    base = np.where(is_land, 64 + 26 * np.clip(C, 0, 1) + 9 * rolling + 170 * np.clip(M, 0, 1.4) ** 1.45 + 34 * plateau
                    + 3 * pm.fbm(X, Z, 70, 811, 3), 63 + np.maximum(d, -2500) / 2500 * 45).astype(np.float32)
    lakes_c = Canvas(n)
    lakes_c.poly(lagoon, 255); lakes_c.poly(blob_poly(*PANTANAL, 0.35, 33), 255)
    for i, (c, r_) in enumerate(LAKES):
        lakes_c.poly(blob_poly(c, r_, 0.4, 40 + i), 255)
    is_lake = (lakes_c.array() > 127) & ~is_land
    river_img, river_info = draw_rivers(n)
    height, profiles, falls = carve(base, river_info, is_land, is_lake)
    river_img = Canvas(n)
    for name, (curve, widths) in river_info.items():
        for k in range(len(curve) - 1):
            river_img.line([curve[k], curve[k + 1]], 255, widths[min(k, len(widths) - 1)])
    river_img = river_img.img
    # onde o vale corta a serra, o bioma é o do vale (encosta, prado), não o do pico
    M = M * np.clip(1 - (base - height) / 140, 0.25, 1)
    dunes = (X > DUNES["x0"] - 300) & (X < DUNES["x1"] + 150) & (Z > DUNES["z0"]) & (Z < DUNES["z1"])
    dune_band = dunes & is_land & (d < 330 + 120 * pm.fbm(X, Z, 300, 801, 3))
    C = np.where(dune_band, np.minimum(C, -0.06), C)

    # ---------- clima ----------
    wx, wz = pm.warp(51)
    n1 = 0.65 * pm.fbm(wx, wz, 2200, 53) + 0.35 * pm.fbm(wx, wz, 800, 63)
    n2 = 0.65 * pm.fbm(wx, wz, 2600, 54) + 0.35 * pm.fbm(wx, wz, 800, 64)
    south = np.clip((Z - 1800) / 3000, 0, 1)
    inland = np.clip((d - 500) / 2500, 0, 1)
    east_basin = np.clip((X - 4950) / 500, 0, 1)
    north_of_serra = 1 / (1 + np.exp((Z - serra_line(X)) / 220)) * (X < 4400)
    T = 0.95 * (wz / HALF) + 0.32 * n1 + 0.35 * south * inland * (1 - east_basin) - 0.32 * east_basin * np.clip((Z + 500) / 3000, 0, 1) \
        - 0.18 * north_of_serra - 0.45 * south * np.clip(1 - np.abs(d) / 1300, 0, 1) * (X < 500)
    coast_wet = np.clip(1 - (X + 3000) / 5500, 0, 1)
    dark = np.exp(-(((X + 1300) / 1150) ** 2 + ((Z + 750) / 950) ** 2))
    dry_south = south * np.clip(1 - np.abs(X - 1300) / 3600, 0, 1) * inland
    carib = south * np.clip(1 - np.abs(d) / 1500, 0, 1) * np.clip((-1200 - X) / 1500, 0, 1)
    H = 0.5 * coast_wet + 0.75 * dark + 0.9 * east_basin - 0.85 * dry_south + 0.8 * carib + 0.32 * n2 - 0.15 * north_of_serra / (1 + np.exp(-wx / 500))
    E = pm.fbm(X, Z, 1500, 55)
    W = pm.fbm(X, Z, 700, 56) * 1.4
    landmask = C >= -0.12
    T = pm.equalize(T, landmask, [-0.45, -0.15, 0.2, 0.55], "T")
    H = pm.equalize(H, landmask, [-0.35, -0.1, 0.1, 0.3], "H")
    # o mar do sudoeste é quente e raso; o norte, gelado
    T_sea = 0.72 * (wz / HALF) + 0.22 * pm.fbm(wx, wz, 1400, 57, 4) + 0.6 * carib
    T = np.where(~landmask, T_sea, T)
    C, T, H, M = pm.village_patch(C, T, H, M)
    calm = pm.blob(0, 0, 1000, 0.4, 8) > 0.45

    # ---------- biomas ----------
    cls, tband = pm.classify(C, T, H, M, E, W, is_bruma)
    for rare, common in pm.CALM.items():
        cls = np.where(calm & (cls == pm.INDEX[rare]), pm.INDEX[common], cls)
    hband = np.digitize(H, [-0.35, -0.1, 0.1, 0.3])
    coast = landmask & (d < 40)
    cliffs = coast & (plateau > 0.35) & (pm.fbm(X, Z, 250, 807, 3) > -0.35)
    cls = np.where(cliffs, pm.INDEX["stony_shore"], cls)
    cls = np.where(dune_band & ~cliffs, pm.INDEX["beach"], cls)
    paradise = landmask & (d < 70 + 40 * pm.fbm(X, Z, 200, 813, 3)) & (Z > 2600) & (X < 600) & ~cliffs
    cls = np.where(paradise, pm.INDEX["beach"], cls)
    cls = np.where(is_bruma & landmask, pm.INDEX["mushroom_fields"], cls)
    # as ilhotas do Mar das Ilhas: coqueiros por dentro, praia em volta
    keys = landmask & (X < -2700) & (Z > 3200)
    cls = np.where(keys & (d > 22), np.where(pm.fbm(X, Z, 120, 817, 3) > 0.1, pm.INDEX["jungle"], pm.INDEX["sparse_jungle"]), cls)
    cls = np.where(keys & (d <= 22), pm.INDEX["beach"], cls)
    # rios (máscara), várzeas e delta
    river = np.array(river_img) > 0
    flood_c = Canvas(n)
    for name, pts, parent in RIVERS:
        flood_c.line(meander(pts, hash_seed(name)), 255, 520)
    for pts in DELTA:
        flood_c.line(pts, 255, 700)
    flood = np.array(flood_c.img.filter(ImageFilter.GaussianBlur(120 / pm.BLOCK))) > 110
    plain = flood & (M < 0.32) & (np.hypot(X, Z) > 700) & (pm.fbm(X, Z, 500, 809) > -0.2) & landmask & ~dune_band
    swamp = plain & (hband >= 2) & (tband >= 1)
    cls = np.where(swamp & (tband <= 2), pm.INDEX["swamp"], np.where(swamp & (tband >= 3), pm.INDEX["mangrove_swamp"], cls))
    lag_ring = (np.hypot((X - LAGOON[0][0]) / (LAGOON[1][0] + 350), (Z - LAGOON[0][1]) / (LAGOON[1][1] + 260)) < 1) & landmask & ~coast
    cls = np.where(lag_ring, pm.INDEX["mangrove_swamp"], cls)
    pantanal = (np.hypot((X - PANTANAL_WET[0][0]) / PANTANAL_WET[1][0], (Z - PANTANAL_WET[0][1]) / PANTANAL_WET[1][1])
                * (1 + 0.25 * pm.fbm(X, Z, 300, 819, 3)) < 1) & landmask
    cls = np.where(pantanal & ~coast, np.where(tband >= 3, pm.INDEX["mangrove_swamp"], pm.INDEX["swamp"]), cls)
    delta_zone = (np.hypot((X + 1900) / 700, (Z - 960) / 420) < 1) & landmask
    cls = np.where(delta_zone & ~coast, pm.INDEX["swamp"], cls)
    water_land = river & landmask
    cls = np.where(water_land & (tband == 0), pm.INDEX["frozen_river"], np.where(water_land, pm.INDEX["river"], cls))

    # ---------- altura (para o sombreamento): nível do mar = 63 ----------
    height = np.where(water_land, height - 2, height)
    drip, lush, dark_cave = pm.caves(C, H, M, E, cls)
    return dict(C=C, T=T, H=H, M=M, cls=cls, tband=tband, d=d, height=height, land=landmask, river=water_land,
                river_info=river_info, dunes=dune_band, dune_lakes=dune_lakes, caves=(drip, lush, dark_cave),
                carib=carib, plateau=plateau, lake=is_lake, profiles=profiles, falls=falls, base=base)


def finish(f):
    f["structures"] = place_structures(f)
    return f


def sample(grid, x, z):
    px, pz = pm.to_px(x, z)
    i, j = int(min(max(pz, 0), pm.N - 1)), int(min(max(px, 0), pm.N - 1))
    return grid[i, j]


# Perfil do vale: (distância da margem em blocos, quanto o chão sobe acima da água ali).
VALLEY = [(0, 0.0), (8, 2.0), (20, 5.0), (45, 11.0), (90, 24.0), (160, 45.0), (260, 75.0)]


def carve(base, river_info, is_land, is_lake):
    """Cada rio só desce: o perfil da água é o mínimo do terreno da nascente até ali, encaixado
    na confluência abaixo do rio que o recebe e no nível do mar (ou do lago) na foz. O terreno em
    volta é rebaixado num vale: estreito e fundo nas serras, largo e raso nas planícies. Onde o
    perfil cai depressa, fica uma cachoeira."""
    parents = {name: parent for name, _, parent in RIVERS}
    ends = {name: pts[-1] for name, pts, _ in RIVERS}
    profiles = {}
    falls = []
    for name, _, parent in RIVERS:
        curve, widths = river_info[name]
        h = [float(sample(base, x, z)) for x, z in curve]
        # A água não sobe: se o caminho passa por um morro mais alto que a nascente, a nascente de
        # verdade é o ponto mais alto antes dele (o trecho de antes corre para o outro lado).
        start = 0
        for _ in range(8):
            low, deepest, at = h[start], 0.0, start
            for k in range(start, len(h)):
                low = min(low, h[k])
                if h[k] - low > deepest:
                    deepest, at = h[k] - low, k
            if deepest <= 90 or at <= start:
                break
            start = max(range(start, at + 1), key=lambda k: h[k])
        if start:
            curve, widths, h = curve[start:], widths[start:], h[start:]
            river_info[name] = (curve, widths)
        prof, low = [], h[0]
        for value in h:
            low = min(low, value)
            prof.append(low)
        # a foz: no mar ou num lago, a água fica no nível do mar; numa confluência, abaixo do pai
        if parent:
            pc = river_info[parent][0]
            k = min(range(len(pc)), key=lambda i: math.hypot(pc[i][0] - ends[name][0], pc[i][1] - ends[name][1]))
            outlet = profiles[parent][k] + 0.5
        else:
            outlet = 62.0
        # se a água chega acima da saída, só o trecho final desce em rampa até ela
        excess = prof[-1] - outlet
        if excess > 0:
            total = sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(curve, curve[1:]))
            reach = min(1500.0, total / 2)
            dist = 0.0
            for k in range(len(prof) - 1, -1, -1):
                if k < len(prof) - 1:
                    dist += math.hypot(curve[k + 1][0] - curve[k][0], curve[k + 1][1] - curve[k][1])
                if dist > reach:
                    break
                prof[k] -= excess * (1 - dist / reach)
        for k in range(1, len(prof)):
            prof[k] = min(prof[k], prof[k - 1])
        profiles[name] = prof
        # cachoeiras: queda de mais de 9 blocos em menos de 60 blocos de rio
        k = 0
        while k < len(prof) - 1:
            j, run = k, 0.0
            while j < len(prof) - 1 and run < 60:
                run += math.hypot(curve[j + 1][0] - curve[j][0], curve[j + 1][1] - curve[j][1]); j += 1
            if prof[k] - prof[j] > 9 and h[k] > 70:
                falls.append((name, curve[k], prof[k] - prof[j]))
                k = j + 8
            else:
                k += 1
    # o vale: para cada faixa de distância, desenha os rios com a altura do chão naquela distância
    # (do mais alto para o mais baixo, para o mais baixo valer onde se cruzam) e fica com o mínimo
    segs = []
    for name, (curve, widths) in river_info.items():
        if name.startswith("delta"):
            prof = [62.0] * len(curve)
        else:
            prof = profiles[name]
        for k in range(len(curve) - 1):
            segs.append(((prof[k] + prof[k + 1]) / 2, curve[k], curve[k + 1], widths[min(k, len(widths) - 1)]))
    segs.sort(key=lambda item: -item[0])
    height = base.copy()
    for off, rise in VALLEY:
        img = Image.new("F", (pm.N, pm.N), 1e9)
        dr = ImageDraw.Draw(img)
        for level, a, b, w in segs:
            dr.line([pm.to_px(*a), pm.to_px(*b)], fill=level + rise, width=max(1, int(round((w + 2 * off) / pm.BLOCK))))
        height = np.minimum(height, np.array(img, dtype=np.float32))
    height = np.where(is_land, height, base)
    # tira cachoeiras repetidas (mais de uma no mesmo trecho de 350 blocos)
    kept = []
    for name, (x, z), drop in sorted(falls, key=lambda f: -f[2]):
        if all(math.hypot(x - kx, z - kz) > 350 for _, (kx, kz), _ in kept):
            kept.append((name, (x, z), drop))
    return height, profiles, kept


def z_curve(x):
    """Onde começa o planalto do norte (z), um pouco mais ao norte no meio do mapa."""
    return 0 * x


def serra_line(x):
    """A Serra do Veio como divisa: ao norte dela, o Pinhal; ao sul, a Mata Escura."""
    return -1750 + 120 * np.sin(x / 900)


def hash_seed(name):
    return sum(ord(c) * (i + 1) for i, c in enumerate(name)) % 997


def draw_rivers(n):
    """A rede de rios, com a largura pela vazão: cada afluente soma o seu comprimento e o dos seus
    afluentes; o rio engrossa ao longo do caminho e em cada confluência."""
    by_name = {name: (pts, parent) for name, pts, parent in RIVERS}
    lengths = {name: sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(pts, pts[1:])) for name, pts, _ in RIVERS}
    children = {}
    for name, pts, parent in RIVERS:
        if parent:
            children.setdefault(parent, []).append(name)

    def total(name):
        return lengths[name] + sum(total(c) for c in children.get(name, []))

    img = Canvas(n)
    info = {}
    for name, pts, parent in RIVERS:
        curve = meander(pts, hash_seed(name))
        joins = {}
        for c in children.get(name, []):
            joins[by_name[c][0][-1]] = total(c)
        flow, done = 120.0, 0.0
        widths = []
        for a, b in zip(curve, curve[1:]):
            for jp, jf in joins.items():
                if math.hypot(a[0] - jp[0], a[1] - jp[1]) < 30:
                    flow += jf
            flow += math.hypot(b[0] - a[0], b[1] - a[1])
            w = 1.5 + 0.33 * math.sqrt(flow)
            widths.append(w)
            img.line([a, b], 255, w)
        info[name] = (curve, widths)
    for k, pts in enumerate(DELTA):
        curve = meander(pts, 700 + k, 0.12)
        img.line(curve, 255, 30)
        info[f"delta{k}"] = (curve, [30] * (len(curve) - 1))
    return img.img, info


# ====================================================================================================
# Desenho
# ====================================================================================================

def colors(f):
    """Cor de cada pixel: bioma, árvores, areia, água rasa e funda, neve."""
    cls, land, d = f["cls"], f["land"], f["d"]
    rgb = pm.COLORS[cls].copy()
    # água do mar: mais clara no raso; o Mar das Ilhas é turquesa
    sea = ~land
    depth = np.clip(-d / 1600, 0, 1)[..., None]
    base = pm.COLORS[cls]
    shallow = np.clip(base * 1.25 + 25, 0, 255)
    rgb = np.where(sea[..., None], shallow * (1 - depth) + base * depth, rgb)
    turq_shallow, turq_deep = np.array([70, 215, 215], np.float32), np.array([25, 120, 175], np.float32)
    tq = (f["carib"] > 0.02) & sea
    mix = np.clip(-d / 900 + (1 - f["carib"]) * 0.8, 0, 1)[..., None]
    blend = np.clip(f["carib"] / 0.25, 0, 1)[..., None] * sea[..., None]
    rgb = rgb * (1 - blend) + (turq_shallow * (1 - mix) + turq_deep * mix) * blend
    # recifes de coral: manchas coloridas no raso do Mar das Ilhas
    reef = tq & (d > -260) & (pm.fbm(pm.X, pm.Z, 60, 821, 3) > 0.32)
    rgb = np.where(reef[..., None], np.array([235, 120, 140], np.float32), rgb)
    # espuma na arrebentação
    near_land = np.zeros_like(land)
    for dz, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        near_land |= np.roll(land, (dz, dx), axis=(0, 1))
    foam = sea & near_land
    rgb = np.where(foam[..., None], np.array([235, 245, 250], np.float32), rgb)
    # rios e lagoas
    river = f["river"]
    frozen = river & (f["tband"] == 0)
    rgb = np.where(river[..., None], np.array([55, 115, 220], np.float32), rgb)
    rgb = np.where(frozen[..., None], np.array([160, 195, 240], np.float32), rgb)
    # árvores: pontinhos escuros com densidade por bioma
    density = np.zeros(len(pm.BIOMES), np.float32)
    for name, dens in {"forest": 0.35, "flower_forest": 0.28, "birch_forest": 0.3, "old_growth_birch_forest": 0.38,
                       "dark_forest": 0.6, "taiga": 0.33, "old_growth_spruce_taiga": 0.45, "old_growth_pine_taiga": 0.42,
                       "snowy_taiga": 0.25, "jungle": 0.55, "sparse_jungle": 0.22, "bamboo_jungle": 0.5, "swamp": 0.18,
                       "mangrove_swamp": 0.45, "windswept_forest": 0.3, "grove": 0.25, "savanna": 0.05, "savanna_plateau": 0.06,
                       "windswept_savanna": 0.05, "wooded_badlands": 0.12, "plains": 0.012, "meadow": 0.01, "cherry_grove": 0.3,
                       "desert": 0.004}.items():
        density[pm.INDEX[name]] = dens
    rng = np.random.default_rng(3)
    trees = (rng.random(cls.shape) < density[cls]) & land & ~river
    shade = np.where(cls == pm.INDEX["cherry_grove"], 0.85, 0.62)
    rgb = np.where(trees[..., None], rgb * shade[..., None], rgb)
    # areia das dunas com ondulação
    dune = f["dunes"] & land & ~river
    ripple = (np.sin((pm.X * 0.8 + pm.Z * 0.35) / 22 + 3 * pm.fbm(pm.X, pm.Z, 120, 823, 2)) > 0.6)
    rgb = np.where((dune & ripple)[..., None], rgb * 0.9, rgb)
    return rgb


def hillshade(height):
    gz, gx = np.gradient(height)
    gx, gz = gx / pm.BLOCK * 5, gz / pm.BLOCK * 5
    light = (-gx - gz) / math.sqrt(2)
    shade = np.clip(0.88 + light * 0.08, 0.55, 1.28)
    return shade


def render(f, out_dir, n):
    rgb = colors(f)
    shade = hillshade(f["height"])
    rgb = rgb * np.where(f["land"], shade, 1.0)[..., None]
    # curvas de nível suaves em terra, a cada 12 blocos
    level = np.floor(f["height"] / 12)
    edge = ((np.diff(level, axis=0, prepend=level[:1]) != 0) | (np.diff(level, axis=1, prepend=level[:, :1]) != 0)) & f["land"] & ~f["river"]
    rgb = np.where(edge[..., None], rgb * 0.88, rgb)
    img = Image.fromarray(np.clip(rgb, 0, 255).astype(np.uint8), "RGB")
    d = ImageDraw.Draw(img, "RGBA")
    P = pm.to_px
    s = n / 1000                                  # escala dos símbolos
    # leitos secos
    for k, pts in enumerate(DRY):
        curve = meander(pts, 300 + k)
        for i in range(0, len(curve) - 1, 2):
            d.line([P(*curve[i]), P(*curve[i + 1])], fill=(205, 175, 125, 230), width=max(1, int(2 * s)))
    # ravinas
    for name, (cx, cz), length, ang in RAVINES:
        a = math.radians(ang)
        pts = []
        for k in range(9):
            t = k / 8 - 0.5
            off = (12 if k % 2 else -12)
            pts.append((cx + math.cos(a) * length * t - math.sin(a) * off, cz + math.sin(a) * length * t + math.cos(a) * off))
        d.line([P(*p) for p in pts], fill=(40, 25, 20, 255), width=max(2, int(3 * s)))
    # zona protegida em volta da vila
    vx, vz = P(0, 0)
    r = VILLAGE_SAFE / pm.BLOCK
    for k in range(0, 360, 6):
        a0, a1 = math.radians(k), math.radians(k + 3)
        d.line([(vx + r * math.cos(a0), vz + r * math.sin(a0)), (vx + r * math.cos(a1), vz + r * math.sin(a1))], fill=(255, 255, 255, 210), width=max(1, int(1.5 * s)))
    # nascentes e cachoeiras
    for name, (curve, widths) in f["river_info"].items():
        if name.startswith("delta"):
            continue
        x, z = curve[0]
        px, pz = P(x, z)
        d.ellipse([px - 3 * s, pz - 3 * s, px + 3 * s, pz + 3 * s], fill=(120, 200, 255, 255), outline=(255, 255, 255, 255))
    for river_name, (x, z), drop in f["falls"]:
        if all(math.hypot(x - wx_, z - wz_) > 400 for _, wx_, wz_ in WATERFALLS):
            px, pz = P(x, z)
            d.polygon([(px, pz + 5 * s), (px - 4 * s, pz - 4 * s), (px + 4 * s, pz - 4 * s)], fill=(235, 250, 255, 255), outline=(30, 90, 170, 255))
    for name, x, z in WATERFALLS:
        px, pz = P(x, z)
        d.polygon([(px, pz + 7 * s), (px - 6 * s, pz - 5 * s), (px + 6 * s, pz - 5 * s)], fill=(235, 250, 255, 255), outline=(30, 90, 170, 255))
    # recursos garantidos
    for kind, x, z, color in RESOURCES:
        px, pz = P(x, z)
        d.polygon([(px, pz - 6 * s), (px + 5 * s, pz), (px, pz + 6 * s), (px - 5 * s, pz)], fill=color + (255,), outline=(255, 255, 255, 255))
    # estruturas
    STYLE = {"mc": (120, 40, 160), "veio": (215, 150, 30), "sub": (70, 70, 90)}
    for name, x, z, kind, biome, moved in f["structures"]:
        px, pz = P(x, z)
        d.rectangle([px - 4 * s, pz - 4 * s, px + 4 * s, pz + 4 * s], fill=STYLE[kind] + (255,), outline=(255, 255, 255, 255))
    # bichos
    for text, x, z in FAUNA:
        px, pz = P(x, z)
        d.ellipse([px - 2.5 * s, pz - 2.5 * s, px + 2.5 * s, pz + 2.5 * s], fill=(255, 190, 60, 255))
        d.text((px + 5 * s, pz), text, fill=(255, 235, 190, 255), font=font(int(9 * s)), anchor="lm", stroke_width=max(1, int(1.5 * s)), stroke_fill=(0, 0, 0, 255))
    # nomes
    for name, x, z, size, style in PLACES:
        px, pz = P(x, z)
        if style == "water":
            d.text((px, pz), name, fill=(215, 235, 255, 255), font=font(int(size * s * 0.55), "serif"), anchor="mm", stroke_width=max(1, int(2 * s)), stroke_fill=(20, 50, 100, 255))
        elif style == "mount":
            d.text((px, pz), name, fill=(255, 255, 255, 255), font=font(int(size * s * 0.55), "bold"), anchor="mm", stroke_width=max(1, int(2 * s)), stroke_fill=(70, 60, 50, 255))
        else:
            d.text((px, pz), name, fill=(255, 255, 255, 255), font=font(int(size * s * 0.55), "bold"), anchor="mm", stroke_width=max(1, int(2 * s)), stroke_fill=(0, 0, 0, 255))
    for name, x, z in WATERFALLS:
        px, pz = P(x, z)
        d.text((px + 8 * s, pz - 2 * s), name, fill=(220, 240, 255, 255), font=font(int(8 * s)), anchor="lm", stroke_width=max(1, int(1.5 * s)), stroke_fill=(20, 50, 100, 255))
    for name, (cx, cz), length, ang in RAVINES:
        px, pz = P(cx, cz)
        d.text((px, pz + 10 * s), name, fill=(255, 220, 200, 255), font=font(int(8 * s)), anchor="mt", stroke_width=max(1, int(1.5 * s)), stroke_fill=(0, 0, 0, 255))
    for name, x, z, kind, biome, moved in f["structures"]:
        px, pz = P(x, z)
        d.text((px + 7 * s, pz), name, fill=(240, 215, 255, 255), font=font(int(8 * s)), anchor="lm", stroke_width=max(1, int(1.5 * s)), stroke_fill=(0, 0, 0, 255))
    for kind, x, z, color in RESOURCES:
        px, pz = P(x, z)
        if math.hypot(x, z) < 520:
            continue                        # perto da vila, os nomes ficam na planta da vila
        d.text((px + 7 * s, pz), kind, fill=(255, 255, 255, 255), font=font(int(8 * s)), anchor="lm", stroke_width=max(1, int(1.5 * s)), stroke_fill=(0, 0, 0, 255))
    # rios com nome (no meio do curso)
    for name, (curve, widths) in f["river_info"].items():
        if name.startswith("delta") or name not in ("Rio Veio", "Rio do Sul", "Rio do Norte", "Grande Rio do Leste", "Rio Claro", "Rio das Bétulas", "Rio do Sul do Leste"):
            continue
        mid = curve[len(curve) // 3]
        px, pz = P(*mid)
        d.text((px, pz - 8 * s), name, fill=(200, 225, 255, 255), font=font(int(9 * s), "serif"), anchor="mm", stroke_width=max(1, int(1.5 * s)), stroke_fill=(20, 50, 100, 255))
    # a vila
    d.regular_polygon((vx, vz, 8 * s), 5, fill=(255, 215, 0, 255), outline=(0, 0, 0, 255))
    d.text((vx, vz - 11 * s), "Vila", fill=(255, 235, 120, 255), font=font(int(13 * s), "bold"), anchor="mb", stroke_width=max(1, int(2 * s)), stroke_fill=(0, 0, 0, 255))
    # muralha de bedrock (a borda do mundo)
    d.rectangle([0, 0, n - 1, n - 1], outline=(20, 20, 22, 255), width=max(3, int(5 * s)))
    return img


def panel(img, f, n, out_dir):
    s = n / 1000
    side = int(470 * s)
    top = int(95 * s)
    canvas = Image.new("RGB", (n + side, n + top), (22, 24, 28))
    canvas.paste(img, (0, top))
    d = ImageDraw.Draw(canvas)
    d.text((int(18 * s), int(14 * s)), "Mapa 5 · Costa oeste", fill=(255, 255, 255), font=font(int(34 * s), "bold"))
    d.text((int(18 * s), int(58 * s)), "15.000 x 15.000 blocos · a vila no centro (0, 0) · norte para cima · 4 blocos por pixel na imagem cheia",
           fill=(200, 205, 215), font=font(int(15 * s)))
    # escala
    x0 = int(30 * s); y0 = n + top - int(30 * s)
    d.line([(x0, y0), (x0 + 1000 / pm.BLOCK, y0)], fill=(255, 255, 255), width=max(2, int(4 * s)))
    d.text((x0, y0 + int(4 * s)), "1.000 blocos", fill=(255, 255, 255), font=font(int(13 * s)))
    # legenda
    cls = f["cls"]
    counts = np.bincount(cls.ravel(), minlength=len(pm.BIOMES))
    X0 = n + int(18 * s); y = top + int(12 * s)
    drip, lush, dark = f["caves"]
    present = int(np.count_nonzero(counts))
    caves_ok = int(drip.any()) + int(lush.any()) + int(dark.any())
    d.text((X0, y), f"Biomas: {present + caves_ok} de {len(pm.BIOMES) + 3}", fill=(140, 230, 140) if present == len(pm.BIOMES) and caves_ok == 3 else (240, 180, 90),
           font=font(int(20 * s), "bold")); y += int(30 * s)
    order = sorted(range(len(pm.BIOMES)), key=lambda i: -counts[i])
    col_w = int(225 * s)
    for k, i in enumerate(order):
        cx = X0 + (k % 2) * col_w
        cy = y + (k // 2) * int(16 * s)
        d.rectangle([cx, cy, cx + int(11 * s), cy + int(11 * s)], fill=pm.BIOMES[i][1], outline=(180, 180, 180))
        area = counts[i] * pm.BLOCK ** 2 / 1e6
        d.text((cx + int(16 * s), cy - int(2 * s)), f"{pm.BIOMES[i][2]} {area:.1f}", fill=(205, 205, 205), font=font(int(10 * s)))
    y += (len(order) + 1) // 2 * int(16 * s) + int(10 * s)
    d.text((X0, y), "Subsolo: cavernas de espeleotemas, exuberantes e a escuridão profunda;\nfora delas, cavernas, minérios e aquíferos normais do Minecraft.",
           fill=(170, 175, 185), font=font(int(10 * s))); y += int(34 * s)
    d.text((X0, y), "Símbolos", fill=(255, 255, 255), font=font(int(17 * s), "bold")); y += int(24 * s)
    def sym(draw_fn, text):
        nonlocal y
        draw_fn(X0 + int(7 * s), y + int(7 * s))
        d.text((X0 + int(20 * s), y), text, fill=(210, 210, 210), font=font(int(11 * s))); y += int(18 * s)
    sym(lambda a, b: d.regular_polygon((a, b, 7 * s), 5, fill=(255, 215, 0), outline=(0, 0, 0)), "a vila (círculo tracejado: 600 blocos sem ravinas nem cavernas)")
    sym(lambda a, b: d.ellipse([a - 3 * s, b - 3 * s, a + 3 * s, b + 3 * s], fill=(120, 200, 255), outline=(255, 255, 255)), "nascente")
    sym(lambda a, b: d.polygon([(a, b + 6 * s), (a - 5 * s, b - 4 * s), (a + 5 * s, b - 4 * s)], fill=(235, 250, 255), outline=(30, 90, 170)), "cachoeira")
    sym(lambda a, b: d.line([(a - 7 * s, b - 2 * s), (a - 2 * s, b + 2 * s), (a + 2 * s, b - 2 * s), (a + 7 * s, b + 2 * s)], fill=(40, 25, 20), width=max(2, int(3 * s))), "ravina / cânion")
    sym(lambda a, b: d.line([(a - 7 * s, b), (a - 2 * s, b)], fill=(205, 175, 125), width=max(1, int(2 * s))), "leito seco (rio que morreu)")
    sym(lambda a, b: d.polygon([(a, b - 6 * s), (a + 5 * s, b), (a, b + 6 * s), (a - 5 * s, b)], fill=(215, 120, 70), outline=(255, 255, 255)), "recurso garantido perto da vila")
    sym(lambda a, b: d.rectangle([a - 4 * s, b - 4 * s, a + 4 * s, b + 4 * s], fill=(120, 40, 160), outline=(255, 255, 255)), "estrutura do Minecraft (bioma conferido)")
    sym(lambda a, b: d.rectangle([a - 4 * s, b - 4 * s, a + 4 * s, b + 4 * s], fill=(215, 150, 30), outline=(255, 255, 255)), "estrutura do Veio (história e extras)")
    sym(lambda a, b: d.rectangle([a - 4 * s, b - 4 * s, a + 4 * s, b + 4 * s], fill=(70, 70, 90), outline=(255, 255, 255)), "estrutura no subsolo")
    sym(lambda a, b: d.ellipse([a - 3 * s, b - 3 * s, a + 3 * s, b + 3 * s], fill=(255, 190, 60)), "onde vivem os bichos da região")
    sym(lambda a, b: d.rectangle([a - 6 * s, b - 4 * s, a + 6 * s, b + 4 * s], fill=(235, 120, 140)), "recife de coral (Mar das Ilhas)")
    y += int(8 * s)
    d.text((X0, y), "Rios: a largura cresce com os afluentes;\nnascem como fios nas geleiras e serras.", fill=(170, 175, 185), font=font(int(10 * s)))
    canvas.save(out_dir / "mapa_final.png", optimize=True)
    preview = canvas.resize((1700, int(1700 * canvas.height / canvas.width)), Image.LANCZOS)
    preview.save(out_dir / "mapa_final_previa.jpg", quality=90)
    return present, caves_ok, counts


STORY = ("Mina do avô", "Cais velho")


def place_structures(f):
    """Confere o bioma de cada estrutura (as regras de onde o Minecraft as gera) e, se o ponto não
    serve, leva a estrutura ao ponto válido mais perto. Longe da vila (700 blocos), menos as da
    história. Devolve [(nome, x, z, tipo, bioma, quanto andou)]."""
    out = []
    X, Z, cls, land, river = pm.X, pm.Z, f["cls"], f["land"], f["river"]
    far = np.hypot(X, Z) > 700
    for name, x, z, kind, need in STRUCTURES:
        if need is None:
            ok = land & ~river
        else:
            ok = np.isin(cls, [pm.INDEX[b] for b in need]) & ~river
        if name not in STORY:
            ok &= far
        if not sample(ok, x, z):
            px, pz = pm.to_px(x, z)
            r = int(1500 / pm.BLOCK)
            i0, i1 = max(0, int(pz) - r), min(pm.N, int(pz) + r)
            j0, j1 = max(0, int(px) - r), min(pm.N, int(px) + r)
            window = ok[i0:i1, j0:j1]
            if window.any():
                ii, jj = np.nonzero(window)
                k = np.argmin((ii + i0 - pz) ** 2 + (jj + j0 - px) ** 2)
                nx = float(X[ii[k] + i0, jj[k] + j0]); nz = float(Z[ii[k] + i0, jj[k] + j0])
                out.append((name, nx, nz, kind, pm.BIOMES[int(sample(cls, nx, nz))][0], math.hypot(nx - x, nz - z)))
                continue
            out.append((name, x, z, kind, pm.BIOMES[int(sample(cls, x, z))][0], -1.0))
            continue
        out.append((name, x, z, kind, pm.BIOMES[int(sample(cls, x, z))][0], 0.0))
    return out


def segments_cross(a, b, c, d):
    def orient(p, q, r):
        return (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])
    return (orient(a, b, c) * orient(a, b, d) < 0) and (orient(c, d, a) * orient(c, d, b) < 0)


def review(f):
    """Revisão física dos rios: cada um desce da nascente à foz (o terreno da planta não sobe mais
    de 8 blocos acima do ponto mais baixo já passado), não atravessa o mar no caminho e não cruza
    outro rio fora da confluência. Devolve a lista de problemas."""
    problems = []
    info = {name: curve for name, (curve, _) in f["river_info"].items() if not name.startswith("delta")}
    parents = {name: parent for name, _, parent in RIVERS}
    ends = {name: pts[-1] for name, pts, _ in RIVERS}
    for name, curve in info.items():
        prof = f["profiles"][name]
        if any(b > a + 1e-6 for a, b in zip(prof, prof[1:])):
            problems.append(f"{name}: a água sobe em algum trecho")
        deepest = max(float(sample(f["base"], x, z)) - p for (x, z), p in zip(curve, prof))
        if deepest > 140:
            problems.append(f"{name}: cânion de {deepest:.0f} blocos (fundo demais)")
        for k, (x, z) in enumerate(curve[:-3]):
            if not sample(f["land"], x, z) and not sample(f["lake"], x, z):
                problems.append(f"{name}: passa pelo mar em ({x:.0f}, {z:.0f})")
                break
    names = list(info)
    for a_i, a in enumerate(names):
        for b in names[a_i + 1:]:
            # a confluência legítima: um é o pai do outro (ou irmãos que chegam ao mesmo ponto)
            join = None
            if parents.get(a) == b: join = ends[a]
            elif parents.get(b) == a: join = ends[b]
            elif parents.get(a) and parents.get(a) == parents.get(b) and ends[a] == ends[b]: join = ends[a]
            elif ends[a] == ends[b]: join = ends[a]
            ca, cb = info[a], info[b]
            found = False
            for p1, p2 in zip(ca, ca[1:]):
                for q1, q2 in zip(cb, cb[1:]):
                    if segments_cross(p1, p2, q1, q2):
                        if join and math.hypot(p1[0] - join[0], p1[1] - join[1]) < 120:
                            continue
                        problems.append(f"{a} cruza {b} perto de ({p1[0]:.0f}, {p1[1]:.0f})")
                        found = True
                        break
                if found:
                    break
    return problems


def export(f, out_dir):
    """A planta em dados, para o gerador e para o mod: costa, ilhas, lagos, serras, rios (com o perfil
    da água), cachoeiras, ravinas, estruturas, recursos, nomes e o acampamento dos Saqueadores."""
    import json
    def pts(curve, step=4):
        return [[round(x), round(z)] for x, z in curve[::step]] + [[round(curve[-1][0]), round(curve[-1][1])]]
    rivers = []
    for name, (curve, widths) in f["river_info"].items():
        if name.startswith("delta"):
            continue
        parent = next((p for n_, _, p in RIVERS if n_ == name), None)
        prof = f["profiles"][name]
        rivers.append({"nome": name, "afluente_de": parent, "pontos": pts(curve),
                       "largura": [round(w, 1) for w in widths[::4]], "nivel_agua": [round(v, 1) for v in prof[::4]]})
    data = {
        "mapa": "Costa oeste", "versao": 2, "tamanho": 15000, "centro": [0, 0], "nivel_do_mar": 63,
        "nota": "Planta do mapa fixo; o gerador segue a planta e a seed fixa. Coordenadas em blocos (x leste, z sul).",
        "costa": [[round(x), round(z)] for x, z in COASTLINE], "fiordes": [[list(a), list(b), w] for a, b, w in FJORDS],
        "ilhas": [{"centro": list(c), "raios": list(r_), "nome": name} for c, r_, _, name in ISLANDS],
        "lagos": [{"centro": list(c), "raios": list(r_)} for c, r_ in LAKES] + [{"centro": list(LAGOON[0]), "raios": list(LAGOON[1]), "nome": "Lagoa da Restinga"},
                  {"centro": list(PANTANAL[0]), "raios": list(PANTANAL[1]), "nome": "Lago do Pantanal"}],
        "serras": [{"pontos": [list(p_) for p_ in pts_], "largura": w, "altura": a} for pts_, w, a, _ in RIDGES],
        "rios": rivers, "delta": [[list(p_) for p_ in d_] for d_ in DELTA], "leitos_secos": [[list(p_) for p_ in d_] for d_ in DRY],
        "cachoeiras": [{"nome": n_, "x": x, "z": z} for n_, x, z in WATERFALLS]
                      + [{"rio": r_, "x": round(x), "z": round(z), "queda": round(q)} for r_, (x, z), q in f["falls"]],
        "ravinas": [{"nome": n_, "centro": list(c), "comprimento": l, "angulo": a} for n_, c, l, a in RAVINES],
        "estruturas": [{"nome": n_, "x": round(x), "z": round(z), "tipo": k, "bioma": b} for n_, x, z, k, b, _ in f["structures"]],
        "recursos": [{"tipo": k, "x": x, "z": z} for k, x, z, _ in RESOURCES],
        "acampamento_saqueadores": next([round(x), round(z)] for n_, x, z, *_ in f["structures"] if n_ == "Acampamento dos Saqueadores"),
        "vila": {"centro": [0, 0], "sem_ravinas_raio": VILLAGE_SAFE, "planta": "vila_estagio1.png"},
        "nomes": [{"nome": n_, "x": x, "z": z} for n_, x, z, _, _ in PLACES],
    }
    (out_dir / "plano.json").write_text(json.dumps(data, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("docs/mapa/costa-oeste")
    n = int(sys.argv[2]) if len(sys.argv) > 2 else 3750
    out_dir.mkdir(parents=True, exist_ok=True)
    f = finish(build(n))
    problems = review(f)
    for name, x, z, kind, biome, moved in f["structures"]:
        if moved != 0:
            print(f"  estrutura {name}: {'movida ' + str(round(moved)) + ' blocos' if moved > 0 else 'SEM LUGAR VÁLIDO'} -> ({x:.0f}, {z:.0f}) {biome}")
    export(f, out_dir)
    print("revisão dos rios:", "sem problemas" if not problems else f"{len(problems)} problema(s)")
    for line in problems:
        print("  -", line)
    img = render(f, out_dir, n)
    present, caves_ok, counts = panel(img, f, n, out_dir)
    missing = [pm.BIOMES[i][2] for i in range(len(pm.BIOMES)) if counts[i] == 0]
    small = sorted(((counts[i] * pm.BLOCK ** 2 / 1e6, pm.BIOMES[i][2]) for i in range(len(pm.BIOMES)) if counts[i]))[:8]
    print(f"biomas {present}/{len(pm.BIOMES)} + cavernas {caves_ok}/3", "faltam: " + ", ".join(missing) if missing else "")
    print("menores:", ", ".join(f"{name} {area:.2f}" for area, name in small))


if __name__ == "__main__":
    main()
