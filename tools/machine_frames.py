"""Máquinas grandes: o bloco da máquina (núcleo) e a estrutura que ele monta em volta.

Cada máquina tem uma planta própria (largura x profundidade x altura). O núcleo fica na
fileira da frente, com a frente livre: é onde o jogador abre o painel e onde a esteira da
frente entrega. O resto da planta são peças de estrutura (`veio:structure`, sem item, sem
inventário) e escotilhas (`veio:hatch_in` e `veio:hatch_out`), que passam itens ao núcleo.

A mesma descrição gera os modelos, as formas de colisão e a tabela nativa
`native/api/src/frames/data.rs`; não editar os arquivos gerados à mão.

Planta: camadas de baixo para cima; em cada camada, fileiras de trás (longe do jogador) para
a frente (a última fileira é a da frente, a do núcleo); em cada fileira, da esquerda para a
direita de quem olha a frente da máquina.
"""
from pathlib import Path

STRUCTURE, HATCH_IN, HATCH_OUT = "structure", "hatch_in", "hatch_out"
TIERS = ("copper", "iron", "steel")

# Material de cada nível: corpo, acabamento e chapa de piso.
TIER_TEXTURES = {
    "copper": {"body": "minecraft:block/cut_copper", "trim": "minecraft:block/polished_deepslate",
               "deck": "minecraft:block/smooth_stone"},
    "iron": {"body": "minecraft:block/iron_block", "trim": "minecraft:block/polished_deepslate",
             "deck": "minecraft:block/polished_andesite"},
    "steel": {"body": "minecraft:block/polished_deepslate", "trim": "minecraft:block/iron_block",
              "deck": "minecraft:block/deepslate_tiles"},
}
COMMON_TEXTURES = {
    "base": "minecraft:block/smooth_stone", "base_side": "minecraft:block/smooth_stone_slab_side",
    "metal": "minecraft:block/iron_block", "pipe": "minecraft:block/cut_copper",
    "joint": "minecraft:block/copper_block", "brick": "minecraft:block/bricks",
    "dark": "minecraft:block/black_concrete", "glow": "minecraft:block/blast_furnace_front_on",
    "wood": "minecraft:block/stripped_spruce_log", "plank": "minecraft:block/spruce_planks",
    "glass": "minecraft:block/tinted_glass", "screen": "minecraft:block/blue_terracotta",
    "warn": "minecraft:block/yellow_terracotta", "port": "veio:block/port",
}

# Peças: letra da planta -> nome da peça. Maiúsculas ficam no chão (têm o rodapé de pedra
# das máquinas); minúsculas vão em cima de outra peça.
PIECES = {
    "C": "casing", "c": "casing_upper", "V": "vent", "F": "frame", "f": "frame_upper",
    "D": "deck", "T": "tank", "u": "tank_upper", "t": "tank_dome", "S": "chimney",
    "s": "chimney_upper", "z": "chimney_cap", "h": "hopper", "P": "pipes", "G": "gearbox",
    "R": "rollers", "K": "derrick_base", "k": "derrick", "y": "derrick_crown", "B": "boiler",
    "W": "flywheel", "O": "firebox", "M": "console", "L": "column", "l": "column_upper",
    "x": "crossbeam", "N": "radiator", "J": "timber", "j": "timber_upper", "Q": "wheel",
    "E": "rack", "a": "antenna", "r": "roof",
}
PORTS = {">": HATCH_OUT, "<": HATCH_IN}
CORE = "@"

# nome: (nível da estrutura, planta). As medidas ficam escritas ao lado de cada máquina.
LAYOUTS = {
    # Extração: torres de sondagem que crescem com o nível.
    "drill": ("copper", [  # 2 x 2 x 3
        [">G",
         "@F"],
        ["k.",
         "D."],
        ["y.",
         ".."],
    ]),
    "drill_mk2": ("steel", [  # 3 x 3 x 4
        ["G>P",
         "FKF",
         "F@F"],
        ["fkf",
         "fkf",
         "D.D"],
        [".k.",
         ".k.",
         "..."],
        [".y.",
         "...",
         "..."],
    ]),
    "drill_mk3": ("steel", [  # 4 x 4 x 5
        ["GP>G",
         "FKKF",
         "FKKF",
         "N@VN"],
        ["fkkf",
         "fkkf",
         "fkkf",
         "D.DD"],
        [".kk.",
         ".kk.",
         ".kk.",
         "...."],
        [".kk.",
         ".kk.",
         "....",
         "...."],
        [".yy.",
         ".yy.",
         "....",
         "...."],
    ]),
    # Prensa: dois pilares e a cabeça hidráulica por cima da mesa.
    "press": ("copper", [  # 2 x 3 x 3
        ["L>",
         "LC",
         "@L"],
        ["lc",
         "lc",
         ".l"],
        ["xx",
         "xx",
         "xx"],
    ]),
    # Montadora: bancada larga com pórtico.
    "assembler": ("copper", [  # 3 x 3 x 2
        ["G>G",
         "RRR",
         "F@F"],
        ["fDf",
         "...",
         "f.f"],
    ]),
    # Cortadora: mesa comprida de roletes, baixa.
    "cutter": ("iron", [  # 2 x 4 x 1
        [">G",
         "RR",
         "RR",
         "@R"],
    ]),
    # Integradora: a maior das bancadas, com cabine de controle.
    "integrator": ("iron", [  # 3 x 4 x 3
        ["E>E",
         "RRR",
         "<RM",
         "F@F"],
        ["cDc",
         "f.f",
         "f.f",
         "f.f"],
        ["a.r",
         "rrr",
         "rrr",
         "rrr"],
    ]),
    # Britador: boca de alimentação no alto, motor ao lado.
    "crusher": ("iron", [  # 2 x 2 x 3
        [">G",
         "@C"],
        ["cc",
         ".c"],
        ["hh",
         "hh"],
    ]),
    # Fundição de liga: fornalha de tijolo e chaminé alta.
    "alloy_furnace": ("iron", [  # 3 x 3 x 4
        ["S>P",
         "OOO",
         "C@C"],
        ["sTc",
         "cuc",
         "c.c"],
        ["s.t",
         "rtr",
         "..."],
        ["z..",
         "...",
         "..."],
    ]),
    # Forno elétrico: corpo compacto, radiador e cabos.
    "electric_furnace": ("copper", [  # 2 x 2 x 2
        [">N",
         "@C"],
        ["cc",
         ".c"],
    ]),
    # Eletrônica: sala limpa com bastidores e antena.
    "electronics": ("steel", [  # 3 x 2 x 2
        ["E>E",
         "M@M"],
        ["cac",
         "r.r"],
    ]),
    # Moinho: estrutura de madeira e roda lateral.
    "mill": ("copper", [  # 2 x 2 x 3
        [">J",
         "@Q"],
        ["jj",
         ".j"],
        ["hh",
         "rr"],
    ]),
    # Cozinha: fogões, coifa e chaminé.
    "kitchen": ("iron", [  # 3 x 2 x 2
        ["S>O",
         "C@C"],
        ["zrr",
         "r.r"],
    ]),
    # Fabricadora: a grande fábrica do fim do jogo.
    "fabricator": ("steel", [  # 4 x 4 x 3
        ["EG>E",
         "RRRR",
         "<RRM",
         "C@VC"],
        ["cDDc",
         "ffff",
         "ffff",
         "c.cc"],
        ["arra",
         "rrrr",
         "rrrr",
         "rrrr"],
    ]),
    # Energia: caldeiras, tanques e volante.
    "generator": ("copper", [  # 2 x 2 x 3
        ["SB",
         "@B"],
        ["sc",
         ".c"],
        ["z.",
         ".."],
    ]),
    "biomass_generator": ("copper", [  # 2 x 3 x 3
        ["SP",
         "BB",
         "@O"],
        ["sc",
         "uc",
         ".c"],
        ["z.",
         "t.",
         ".."],
    ]),
    "steam_engine": ("steel", [  # 3 x 5 x 2
        ["SPT",
         "BBT",
         "BBG",
         "WWG",
         "C@C"],
        ["sru",
         "rrt",
         "rr.",
         "...",
         "c.c"],
    ]),
    "geothermal": ("steel", [  # 3 x 3 x 2
        ["TPT",
         "NCN",
         "T@T"],
        ["tct",
         ".r.",
         "t.t"],
    ]),
    # Laboratório: bancadas, armários e antena.
    "research_lab": ("iron", [  # 3 x 3 x 2
        ["EME",
         "RRR",
         "T@T"],
        ["cac",
         "...",
         "t.t"],
    ]),
}


def parse(layers):
    """Células da planta: ([dx, dy, dz], tipo, peça) com o núcleo em (0, 0, 0), x para a
    direita de quem olha a frente e z negativo para trás."""
    ground = layers[0]
    depth, width = len(ground), len(ground[0])
    core = [(row.index(CORE), z) for z, row in enumerate(ground) if CORE in row]
    assert len(core) == 1, "uma planta tem um núcleo"
    cx, cz = core[0]
    assert cz == depth - 1, "o núcleo fica na fileira da frente"
    cells = []
    for y, layer in enumerate(layers):
        assert len(layer) == depth and all(len(row) == width for row in layer), "camadas do mesmo tamanho"
        for z, row in enumerate(layer):
            for x, char in enumerate(row):
                if char in ".@":
                    assert char == "." or y == 0, "núcleo só no chão"
                    continue
                at = [x - cx, y, z - cz]
                if char in PORTS:
                    assert y == 0, "escotilhas ficam no chão"
                    cells.append((at, PORTS[char], None))
                else:
                    cells.append((at, STRUCTURE, PIECES[char]))
    return {"size": [width, len(layers), depth], "cells": cells}


def plan(name):
    tier, layers = LAYOUTS[name]
    return {"tier": tier, **parse(layers)}


def piece_elements(piece, box):
    """Caixas (pixels) de uma peça, olhando para o sul como o núcleo."""
    parts = []

    def part(lo, hi, material="body", faces=None, uv=None):
        parts.append(box(lo, hi, faces or "#" + material, uv=uv))

    def plinth():
        part([0, 0, 0], [16, 1.5, 16], faces={"up": "#base", "down": "#base", "*": "#base_side"})

    def posts(low=0, high=16, size=2, material="trim"):
        for x in (0, 16 - size):
            for z in (0, 16 - size):
                part([x, low, z], [x + size, high, z + size], material)

    def rails(y):
        for lo, hi in [([0, y, 0], [16, y + 1, 1]), ([0, y, 15], [16, y + 1, 16]),
                       ([0, y, 1], [1, y + 1, 15]), ([15, y, 1], [16, y + 1, 15])]:
            part(lo, hi, "trim")

    if piece in ("casing", "casing_upper"):
        low = 1.5 if piece == "casing" else 0
        if piece == "casing": plinth()
        part([0.5, low, 0.5], [15.5, 16, 15.5], "body")
        posts(low, 16, 1.5)
        for y in (6, 11):  # costuras das chapas
            part([0, y, 0.25], [16, y + 0.75, 15.75], "trim")
    elif piece == "vent":
        plinth()
        part([1, 1.5, 1], [15, 16, 15], "body")
        posts(1.5, 16, 1.5)
        for y in (4, 6.5, 9, 11.5):  # aletas da grelha na frente e atrás
            part([3, y, 0], [13, y + 1, 16], "dark")
    elif piece in ("frame", "frame_upper"):
        low = 1.5 if piece == "frame" else 0
        if piece == "frame": plinth()
        posts(low, 16)
        for y in ((low, 14) if piece == "frame" else (0, 14)):
            for lo, hi in [([2, y, 0], [14, y + 2, 2]), ([2, y, 14], [14, y + 2, 16]),
                           ([0, y, 2], [2, y + 2, 14]), ([14, y, 2], [16, y + 2, 14])]:
                part(lo, hi, "trim")
        part([2, 7, 7], [14, 9, 9], "metal")  # travessa
    elif piece == "deck":
        part([0, 0, 0], [16, 2, 16], faces={"up": "#deck", "*": "#trim"})
        for x in (0, 15):
            part([x, 2, 0], [x + 1, 9, 1], "trim")
            part([x, 2, 15], [x + 1, 9, 16], "trim")
        rails(8)
    elif piece in ("tank", "tank_upper"):
        low = 1.5 if piece == "tank" else 0
        if piece == "tank": plinth()
        part([3, low, 1], [13, 16, 15], "body")
        part([1, low, 3], [15, 16, 13], "body")
        part([2, low, 2], [14, 16, 14], "body")
        for y in (5, 12):
            part([0.75, y, 2.75], [15.25, y + 1, 13.25], "trim")
            part([2.75, y, 0.75], [13.25, y + 1, 15.25], "trim")
    elif piece == "tank_dome":
        part([2, 0, 2], [14, 3, 14], "body")
        part([4, 3, 4], [12, 5, 12], "body")
        part([7, 5, 7], [9, 8, 9], "metal")
    elif piece in ("chimney", "chimney_upper"):
        low = 1.5 if piece == "chimney" else 0
        if piece == "chimney":
            plinth()
            part([1, 1.5, 1], [15, 5, 15], "brick")
        part([3, low, 3], [13, 16, 13], "brick")
        part([2.5, 9, 2.5], [13.5, 10, 13.5], "metal")
    elif piece == "chimney_cap":
        part([3, 0, 3], [13, 6, 13], "brick")
        part([2, 6, 2], [14, 8, 14], "brick")
        part([4, 7.9, 4], [12, 8.05, 12], "dark")
    elif piece == "hopper":
        part([1, 0, 1], [15, 3, 15], "trim")
        for lo, hi in [([0, 3, 0], [16, 9, 2]), ([0, 3, 14], [16, 9, 16]), ([0, 3, 2], [2, 9, 14]), ([14, 3, 2], [16, 9, 14])]:
            part(lo, hi, "body")
        part([2, 2.9, 2], [14, 3.05, 14], "dark")
    elif piece == "pipes":
        plinth()
        part([1, 1.5, 1], [15, 16, 8], "body")
        for x in (2, 7, 12):
            part([x, 1.5, 9], [x + 3, 16, 12], "pipe")
            part([x - 0.5, 4, 8.5], [x + 3.5, 5, 12.5], "joint")
            part([x - 0.5, 12, 8.5], [x + 3.5, 13, 12.5], "joint")
        part([1, 14, 8], [15, 15.5, 13], "trim")
    elif piece == "gearbox":
        plinth()
        part([1, 1.5, 1], [15, 14, 15], "body")
        part([0, 14, 0], [16, 16, 16], "trim")
        for x0, x1 in ((-0.01, 0.99), (15.01, 16.01)):  # engrenagem em relevo nas laterais
            lo, hi = min(x0, x1), max(x0, x1)
            part([max(lo, 0), 4, 3], [min(hi, 16), 12, 13], "metal")
            part([max(lo, 0), 3, 6], [min(hi, 16), 13, 10], "metal")
        part([0, 7, 7], [16, 9, 9], "joint")
    elif piece == "rollers":
        plinth()
        part([0, 1.5, 0], [2, 6, 16], "trim")
        part([14, 1.5, 0], [16, 6, 16], "trim")
        for z in range(1, 16, 3):
            part([2, 4, z], [14, 5.5, z + 2], "metal")
        part([2, 1.5, 0], [14, 3, 16], "dark")
    elif piece in ("derrick_base", "derrick"):
        low = 1.5 if piece == "derrick_base" else 0
        if piece == "derrick_base": plinth()
        posts(low, 16, 2, "body")
        for y in (low + 1, 8):
            for lo, hi in [([2, y, 0.5], [14, y + 1, 1.5]), ([2, y, 14.5], [14, y + 1, 15.5]),
                           ([0.5, y, 2], [1.5, y + 1, 14]), ([14.5, y, 2], [15.5, y + 1, 14])]:
                part(lo, hi, "trim")
        part([7, low, 7], [9, 16, 9], "metal")  # haste da sonda
    elif piece == "derrick_crown":
        posts(0, 4, 2, "body")
        part([0, 4, 0], [16, 6, 16], "trim")
        part([4, 6, 7], [12, 10, 9], "metal")  # roldana
        part([7, 6, 4], [9, 10, 12], "joint")
        part([7.5, 10, 7.5], [8.5, 12, 8.5], "warn")
    elif piece == "boiler":
        plinth()
        for z in (2, 12):  # berços
            part([2, 1.5, z], [14, 4, z + 2], "trim")
        part([1, 5, 0], [15, 15, 16], "body")
        part([3, 3, 0], [13, 16, 16], "body")
        for z in (3, 12.5):
            part([0.75, 4.75, z], [15.25, 15.25, z + 1], "trim")
    elif piece == "flywheel":
        plinth()
        part([1, 1.5, 3], [3, 10, 13], "trim")
        part([13, 1.5, 3], [15, 10, 13], "trim")
        part([5, 1.5, 1], [11, 16, 15], "metal")
        part([5, 3, 0], [11, 14, 16], "metal")
        part([6, 6.5, 6.5], [10, 9.5, 9.5], "joint")
        part([0, 7, 7], [16, 9, 9], "joint")  # eixo
    elif piece == "firebox":
        plinth()
        part([0.5, 1.5, 0.5], [15.5, 14, 15.5], "brick")
        part([4, 3, 15.5], [12, 9, 16], "glow")
        part([3, 2.5, 15.4], [13, 3, 16], "trim")
        part([0, 14, 0], [16, 16, 16], "trim")
    elif piece == "console":
        plinth()
        part([1, 1.5, 2], [15, 8, 14], "body")
        part([1, 8, 2], [15, 10, 9], "trim")
        part([2, 10, 2], [14, 15, 4], "trim")
        part([3, 10.5, 4], [13, 14.5, 4.25], "screen")
        part([3, 8.2, 10], [13, 8.6, 13], "warn")
    elif piece in ("column", "column_upper"):
        low = 1.5 if piece == "column" else 0
        if piece == "column":
            plinth()
            part([2, 1.5, 2], [14, 4, 14], "trim")
        part([4, low, 4], [12, 16, 12], "metal")
        part([5, low, 3], [11, 16, 13], "metal")
        part([3, low, 5], [13, 16, 11], "metal")
        part([3.5, 10, 3.5], [12.5, 11, 12.5], "trim")
    elif piece == "crossbeam":
        part([0, 0, 1], [16, 6, 15], "body")
        part([0, 6, 0], [16, 8, 16], "trim")
        part([5, 8, 5], [11, 12, 11], "joint")  # cilindro hidráulico
    elif piece == "radiator":
        plinth()
        part([1, 1.5, 6], [15, 15, 10], "trim")
        for x in range(1, 15, 2):
            part([x, 2, 1], [x + 1, 14.5, 15], "body")
    elif piece in ("timber", "timber_upper"):
        low = 1.5 if piece == "timber" else 0
        if piece == "timber": plinth()
        posts(low, 16, 3, "wood")
        part([0.5, low, 1], [15.5, 16, 15], "plank")
        part([0, 7, 0], [16, 9, 16], "wood")
    elif piece == "wheel":
        plinth()
        part([1, 1.5, 6], [6, 10, 10], "wood")  # mancal
        part([6, 0.5, 1], [9, 15.5, 15], "plank")
        part([6, 3, -0.5], [9, 13, 16.5], "plank")
        for y in (2, 7, 12):
            part([5.5, y, 0.5], [9.5, y + 1.5, 15.5], "wood")  # pás
        part([1, 7, 7], [16, 9, 9], "metal")
    elif piece == "rack":
        plinth()
        part([1, 1.5, 1], [15, 16, 15], "trim")
        for y in (3, 7, 11):
            part([2, y, 15], [14, y + 3, 15.5], "screen")
            part([11, y + 1, 15.5], [13, y + 2, 15.6], "warn")
    elif piece == "antenna":
        part([3, 0, 3], [13, 2, 13], "trim")
        part([7, 2, 7], [9, 14, 9], "metal")
        part([3, 11, 7.5], [13, 12, 8.5], "metal")
        part([6, 14, 6], [10, 16, 10], "joint")
    elif piece == "roof":
        part([0, 0, 0], [16, 2, 16], faces={"up": "#deck", "*": "#trim"})
        for z in (2, 7, 12):  # telhado de chapas com nervuras
            part([0, 2, z], [16, 3, z + 1.5], "trim")
        part([10, 2, 9], [14, 5, 13], "body")  # exaustor
    else:
        raise ValueError(piece)
    for p in parts:
        for key in ("from", "to"):
            p[key] = [min(16, max(0, v)) for v in p[key]]
    return parts


def hatch_elements(kind, box):
    """Escotilha: caixa da estrutura com a boca para trás (saída) ou para o lado (entrada)."""
    parts = [box([0, 0, 0], [16, 1.5, 16], {"up": "#base", "down": "#base", "*": "#base_side"}),
             box([0.5, 1.5, 0.5], [15.5, 16, 15.5], "#body")]
    for x in (0, 14.5):
        for z in (0, 14.5):
            parts.append(box([x, 1.5, z], [x + 1.5, 16, z + 1.5], "#trim"))
    color = "#warn" if kind == HATCH_OUT else "#screen"
    if kind == HATCH_OUT:  # boca de saída voltada para trás, como o bico das máquinas
        parts.append(box([4, 4, 0], [12, 12, 0.5], {"north": "#port", "*": "#trim"}))
        parts.append(box([3, 3, 0], [13, 4, 0.6], color))
    else:  # entrada nas duas laterais
        parts.append(box([0, 4, 4], [0.5, 12, 12], {"west": "#port", "*": "#trim"}))
        parts.append(box([15.5, 4, 4], [16, 12, 12], {"east": "#port", "*": "#trim"}))
        parts.append(box([0, 3, 3], [0.6, 4, 13], color))
        parts.append(box([15.4, 3, 3], [16, 4, 13], color))
    return parts


def textures(tier, particle="body"):
    values = {**COMMON_TEXTURES, **TIER_TEXTURES[tier]}
    values["particle"] = values[particle]
    return values


def hits(piece):
    """Colisão de cada peça, em blocos: cheia, salvo as baixas e as pontas de cima."""
    low = {"rollers": 6, "deck": 2, "tank_dome": 5, "chimney_cap": 8, "hopper": 9,
           "derrick_crown": 6, "antenna": 2, "roof": 3, "crossbeam": 8}
    if piece == "chimney_upper":
        return [[3/16, 0, 3/16, 13/16, 1, 13/16]]
    if piece in low:
        return [[0, 0, 0, 1, low[piece] / 16, 1]]
    return [[0, 0, 0, 1, 1, 1]]


def used_pieces():
    return sorted({piece for name in LAYOUTS for _, kind, piece in plan(name)["cells"] if kind == STRUCTURE})


def build(root, box, write):
    """Modelos, blockstates e registros das peças e escotilhas."""
    assets, data = Path(root) / "assets/veio", Path(root) / "data/veio"
    pieces = used_pieces()
    rotations = {"south": 0, "west": 90, "north": 180, "east": 270}
    variants = {}
    for tier in TIERS:
        for piece in pieces:
            write(assets / f"models/block/structure/{piece}_{tier}.json", {"parent": "minecraft:block/block",
                "textures": textures(tier), "elements": piece_elements(piece, box)})
            for facing, y in rotations.items():
                variant = {"model": f"veio:block/structure/{piece}_{tier}"}
                if y: variant["y"] = y
                variants[f"facing={facing},piece={piece},tier={tier}"] = variant
    write(assets / "blockstates/structure.json", {"variants": variants})
    shape = {"properties": ["piece"], "values": {piece: hits(piece) for piece in pieces}, "default": [[0, 0, 0, 1, 1, 1]]}
    write(data / "horizon/block/structure.json", {"template": "minecraft:black_glazed_terracotta", "item": False,
        "properties": {"hardness": 2.0, "occludes": False},
        "state_properties": {"piece": {"type": "enum", "values": pieces}, "tier": {"type": "enum", "values": list(TIERS)}},
        "default_state": {"piece": pieces[0], "tier": "iron"},
        "shapes": shape, "collision_shapes": shape})
    for kind in (HATCH_IN, HATCH_OUT):
        hatch = {}
        for tier in TIERS:
            write(assets / f"models/block/structure/{kind}_{tier}.json", {"parent": "minecraft:block/block",
                "textures": textures(tier), "elements": hatch_elements(kind, box)})
            for facing, y in rotations.items():
                variant = {"model": f"veio:block/structure/{kind}_{tier}"}
                if y: variant["y"] = y
                hatch[f"facing={facing},tier={tier}"] = variant
        write(assets / f"blockstates/{kind}.json", {"variants": hatch})
        write(data / f"horizon/block/{kind}.json", {"template": "minecraft:black_glazed_terracotta", "item": False,
            "properties": {"hardness": 2.0, "occludes": False},
            "state_properties": {"tier": {"type": "enum", "values": list(TIERS)}},
            "default_state": {"tier": "iron"},
            "shapes": {"default": [[0, 0, 0, 1, 1, 1]]}, "collision_shapes": {"default": [[0, 0, 0, 1, 1, 1]]}})


def native_layouts(root):
    """`native/api/src/frames/data.rs`: a planta de cada máquina para o módulo nativo."""
    lines = ["// Generated by tools/build_content.py from tools/machine_frames.py.\n",
             "use super::{Cell, Layout, Part};\n\n",
             "pub(super) fn layout(block: &str) -> Option<&'static Layout> {\n    Some(match block {\n"]
    for name in LAYOUTS:
        p = plan(name)
        cells = []
        for at, kind, piece in p["cells"]:
            what = {HATCH_IN: "Part::HatchIn", HATCH_OUT: "Part::HatchOut"}.get(kind, f'Part::Piece("{piece}")')
            cells.append(f"Cell {{ at: [{at[0]}, {at[1]}, {at[2]}], part: {what} }}")
        w, h, d = p["size"]
        lines.append(f'        "veio:{name}" => &Layout {{ tier: "{p["tier"]}", size: [{w}, {h}, {d}], cells: &[\n            '
                     + ",\n            ".join(cells) + ",\n        ] },\n")
    lines.append("        _ => return None,\n    })\n}\n")
    target = Path(root).parent / "native/api/src/frames/data.rs"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text("".join(lines), encoding="utf-8")
