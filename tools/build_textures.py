"""Gera as texturas 16x16 do Veio no estilo pixel art do Minecraft.

Cada textura é desenhada em texto: uma letra por pixel, com a paleta abaixo.
Rode `python tools/build_textures.py` depois de mudar um desenho e inclua os PNG no commit.
Não precisa de bibliotecas externas.
"""
import struct
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "content/assets/veio/textures"

PALETTE = {
    ".": None,
    # Ferro, do mais claro ao buraco escuro.
    "a": "e6e6e6", "b": "c8c8c8", "c": "a9a9a9", "d": "8d8d8d", "e": "6f6f6f", "f": "525252",
    "g": "363636", "h": "1d1d1d", "k": "0e0e0e", "w": "ffffff",
    # Cobre, nas cores do bloco de cobre vanilla.
    "p": "fc9982", "q": "e77c56", "r": "c15a36", "s": "9c4529", "t": "6d3421",
    # Ouro e latão.
    "y": "fcee4b", "Y": "d8af23", "z": "8f6a10",
    # Fogo e lava.
    "O": "ffe27a", "o": "ff9b26", "R": "d9381e", "X": "8a1d0e",
    # Placa de circuito.
    "G": "2f7a34", "H": "47a24c", "J": "1a4a1e",
    # Silício e detalhes azuis.
    "U": "41506e", "u": "7a90bd", "v": "b7c9f0",
    # Plantação e madeira.
    "W": "e4cf6e", "V": "a8892e", "L": "62b23c", "K": "3c7d26", "n": "a07a46", "N": "674b29",
    # Aço escuro.
    "S": "5b6170", "T": "3d424e", "M": "7c8394",
    # Vermelho de sinalização e ciano da esteira rápida.
    "x": "c22a2a", "C": "3fd0d8",
}

def hex_rgb(value):
    return tuple(int(value[i:i + 2], 16) for i in (0, 2, 4))

def jitter(x, y, salt):
    """Variação fixa por pixel, como o ruído suave das texturas vanilla."""
    n = (x * 73856093) ^ (y * 19349663) ^ (salt * 83492791)
    n = (n ^ (n >> 13)) * 1274126177 & 0xFFFFFFFF
    return (n % 9) - 4

def render(rows, salt=0, noise=True, mapping=None):
    assert len(rows) == 16, f"{len(rows)} linhas"
    pixels = []
    for y, row in enumerate(rows):
        assert len(row) == 16, f"linha {y} com {len(row)} colunas: {row!r}"
        for x, char in enumerate(row):
            char = (mapping or {}).get(char, char)
            color = PALETTE[char]
            if color is None:
                pixels.append((0, 0, 0, 0))
                continue
            r, g, b = hex_rgb(color)
            d = jitter(x, y, salt) if noise and char not in "kwhyO" else 0
            pixels.append((max(0, min(255, r + d)), max(0, min(255, g + d)), max(0, min(255, b + d)), 255))
    return pixels

def write_png(path, pixels, size=16):
    raw = b"".join(b"\x00" + bytes(c for p in pixels[y * size:(y + 1) * size] for c in p) for y in range(size))
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0))
    png += chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b"")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(png)

# ---------------------------------------------------------------- faces das máquinas
# A face frontal mostra as colunas 1-14 e as linhas 2-13: o resto fica sob a tampa e a base.
# 1 claro, 2 médio, 3 borda e 4 visor, trocados pela cor de cada nível.
FRONT = [
    "2222222222222222",
    "2222222222222222",
    "2333333333333332",
    "231a11111111a132",
    "2312222222222132",
    "2312444444442132",
    "2312444444442132",
    "2312444444442132",
    "2312444444442132",
    "2312444444442132",
    "2312444444442132",
    "2312444444442132",
    "231a44444444a132",
    "2333333333333332",
    "2222222222222222",
    "2222222222222222",
]
TIERS = {
    "copper": {"1": "q", "2": "r", "3": "t", "4": "h"},
    "iron": {"1": "b", "2": "c", "3": "e", "4": "h"},
    "steel": {"1": "M", "2": "S", "3": "g", "4": "k"},
}

MOTIFS = {
    "drill": [
        ".ebbbbe.",
        "..cYYc..",
        "..bccb..",
        "..cYYc..",
        "...bc...",
        "...cY...",
        "...bc...",
        "...c....",
    ],
    "press": [
        "...ee...",
        "...ce...",
        "...ce...",
        "bbbbbbbb",
        "eccccccd",
        "........",
        ".pqqqqq.",
        ".rssssr.",
    ],
    "assembler": [
        "...qq...",
        ".q.qq.q.",
        "..qrrq..",
        "qqr..rqq",
        "qqr..rqq",
        "..qrrq..",
        ".q.qq.q.",
        "...qq...",
    ],
    "cutter": [
        ".b.bb.b.",
        "..cccc..",
        "bcceeccb",
        ".ce..ec.",
        ".ce..ec.",
        "bcceeccb",
        "..cccc..",
        ".b.bb.b.",
    ],
    "crusher": [
        "cccccccc",
        "dbdbdbdb",
        ".d.d.d.d",
        "........",
        "........",
        "d.d.d.d.",
        "bdbdbdbd",
        "cccccccc",
    ],
    "alloy_furnace": [
        "eeeeeeee",
        "e......e",
        "e......e",
        "e.R..R.e",
        "eRoRRoRe",
        "eoOooOoe",
        "eOyOOyOe",
        "eeeeeeee",
    ],
    "electronics": [
        "HGGGGGGH",
        "GyyyyG.G",
        "GG.Gy.yG",
        "GkkkkGyG",
        "GkkkkGyG",
        "GyGyGGyG",
        "GyyyyyyG",
        "HGGGGGGH",
    ],
    "integrator": [
        "yYyYyY..",
        "Y....e..",
        "y....e..",
        "Y...cbc.",
        "y...c.c.",
        "Y.......",
        "y..qqqq.",
        "zz.rrrr.",
    ],
    "generator": [
        "eeeeeeee",
        "e......e",
        "eR.R.R.e",
        "eRoRoRoe",
        "eoOooOoe",
        "eOyOOyOe",
        "eeeeeeee",
        "ekkkkkke",
    ],
    "geothermal": [
        "eeeeeeee",
        "eRoOoRoe",
        "eoOyOoRe",
        "eRoOooOe",
        "eeeeeeee",
        "eoRoOoRe",
        "eOooRoOe",
        "eeeeeeee",
    ],
    "contract_hub": [
        "nnnnnnnn",
        "nwwwwwwn",
        "nwkkkkwn",
        "nwwwwwwn",
        "nwkkkwwn",
        "nwwwwwwn",
        "nwkkwxwn",
        "nnnnnnnn",
    ],
    "harvester": [
        "W.W..W.W",
        "VWV.WVW.",
        ".V..V.V.",
        ".L..L.L.",
        ".K.LK.L.",
        "bbbbbbbb",
        "ecececec",
        "........",
    ],
}
MOTIFS.update({
    "electric_furnace": [
        "eeeeeeee",
        "e..y...e",
        "e.yy...e",
        "eyyyyy.e",
        "e...yy.e",
        "e...y..e",
        "eoOOOOoe",
        "eeeeeeee",
    ],
    "mill": [
        "W.cccc.W",
        ".cbbbbc.",
        "cbbeebbc",
        "cbeWWebc",
        "cbeWWebc",
        "cbbeebbc",
        ".cbbbbc.",
        "W.cccc.W",
    ],
    "kitchen": [
        "........",
        "..nnnn..",
        ".nWWWWn.",
        "nWVWVWWn",
        "nWWWWWWn",
        ".NNNNNN.",
        "eeeeeeee",
        ".e....e.",
    ],
    "biomass_generator": [
        "...LL...",
        "..LKKL..",
        ".LKLLKL.",
        "..LKKL..",
        "...KK...",
        "eR.R.R.e",
        "eoOooOoe",
        "eeeeeeee",
    ],
    "steam_engine": [
        ".w.w.w..",
        "w.w.w.w.",
        "eeeeeeee",
        "ebbbbbbe",
        "ecccccce",
        "e.eeee.e",
        "eRoOoRoe",
        "eeeeeeee",
    ],
    "planter": [
        "...ee...",
        "..eYYe..",
        "...ee...",
        "...V....",
        ".....V..",
        "..V.....",
        "NnNnNnNn",
        "nNnNnNnN",
    ],
    "tree_farm": [
        "..LLLL..",
        ".LKLLKL.",
        "LLKLLKLL",
        ".LLKKLL.",
        "...nn...",
        "...nn..b",
        "...nn.bb",
        "NNNNNNNN",
    ],
    "ranch": [
        "........",
        ".k.kk.k.",
        ".wkwwkw.",
        ".wwkwww.",
        ".kwwwkk.",
        "..pppp..",
        "..pkkp..",
        "........",
    ],
    "house": [
        "...xx...",
        "..xxxx..",
        ".xxxxxx.",
        "xxxxxxxx",
        ".nnnnnn.",
        ".nvvnkn.",
        ".nvvnkn.",
        ".nnnnkn.",
    ],
    "shop": [
        "xwxwxwxw",
        "xwxwxwxw",
        ".n....n.",
        ".n.yO.n.",
        ".n....n.",
        "nnnnnnnn",
        "nWWWWWWn",
        "nnnnnnnn",
    ],
    "town_hall": [
        "...yy...",
        "..yYYy..",
        ".aaaaaa.",
        "aaaaaaaa",
        ".b.bb.b.",
        ".b.bb.b.",
        ".b.bb.b.",
        "aaaaaaaa",
    ],
    "well": [
        "..nnnn..",
        ".n....n.",
        "..n..n..",
        "..n..n..",
        "bcbcbcbc",
        "cuuuuuuc",
        "cuuuuuub",
        "bcbcbcbc",
    ],
    "export_dock": [
        "...w....",
        "...ww...",
        "...www..",
        "...w....",
        "nnnnnnnn",
        ".nnnnnn.",
        "uuvuuvuu",
        "vuuvuuvu",
    ],
    "barracks": [
        "..xx....",
        ".xRRx...",
        "xRRRRx.a",
        ".xRRx.aa",
        "..xx.aa.",
        "....aa..",
        "....aa..",
        "....a...",
    ],
    "farm_post": [
        "..VWWV..",
        ".VWWWWV.",
        "VVVVVVVV",
        "..nnnn..",
        "..nkkn..",
        "..nnnn..",
        "LKLKLKLK",
        "NnNnNnNn",
    ],
    "hauler_post": [
        "........",
        ".nnnnnn.",
        ".nNNNNn.",
        ".nnnnnn.",
        "..e..e..",
        ".eeeeee.",
        ".e.ee.e.",
        "........",
    ],
    "shepherd_post": [
        "........",
        ".wwwww..",
        "wwwwwwwk",
        "wwwwwwwk",
        ".wwwww..",
        ".k.k.k..",
        ".k.k.k..",
        "NnNnNnNn",
    ],
    "barn": [
        "...xx...",
        "..xxxx..",
        ".xxxxxx.",
        "xxnnnnxx",
        ".xnwwnx.",
        ".xnwwnx.",
        ".xnnnnx.",
        "........",
    ],
    "silo": [
        "..dddd..",
        ".dbbbbd.",
        ".dcccdd.",
        ".dbbbbd.",
        ".dcccdd.",
        ".dbbbbd.",
        ".dWWWWd.",
        "........",
    ],
    "control_center": [
        "eeeeeeee",
        "eGHGHGHe",
        "eHyGGuHe",
        "eGGyuGGe",
        "eHGHuyHe",
        "eeeeeeee",
        "...ee...",
        "..eeee..",
    ],
    "foreman_desk": [
        "...yy...",
        "..yYYy..",
        "..yYYy..",
        "...zz...",
        "nnnnnnnn",
        "nwkwkwkn",
        "nnnnnnnn",
        ".N....N.",
    ],
    "pantry": [
        "nnnnnnnn",
        "nWWnooRn",
        "nWWnRoon",
        "nnnnnnnn",
        "nqqnGHGn",
        "nqqnHGHn",
        "nnnnnnnn",
        "........",
    ],
})
MACHINE_TIER = {
    "drill": "copper", "press": "copper", "assembler": "copper", "generator": "copper",
    "contract_hub": "copper", "cutter": "iron", "integrator": "iron", "crusher": "iron",
    "alloy_furnace": "iron", "harvester": "iron", "electronics": "steel", "geothermal": "steel",
    "drill_mk2": "steel",
    "drill_mk3": "steel", "electric_furnace": "copper", "mill": "copper", "kitchen": "iron", "pantry": "copper",
    "biomass_generator": "copper", "steam_engine": "steel", "planter": "iron", "tree_farm": "iron", "ranch": "copper",
    "farm_post": "copper", "hauler_post": "iron", "foreman_desk": "copper", "shepherd_post": "copper",
    "barracks": "iron", "house": "copper", "shop": "copper", "town_hall": "steel", "well": "iron", "export_dock": "steel",
    "barn": "copper", "silo": "iron", "control_center": "steel",
}

def front(name):
    motif = MOTIFS["drill" if name in ("drill_mk2", "drill_mk3") else name]
    rows = [list(row) for row in FRONT]
    for y, line in enumerate(motif):
        for x, char in enumerate(line):
            if char != ".":
                rows[5 + y][4 + x] = char
    if name == "drill_mk2":  # Faixa azul: a versão avançada.
        rows[3][3:13] = list("uuuuuuuuuu")
    if name == "drill_mk3":  # Faixa dourada: a última extratora.
        rows[3][3:13] = list("yYyYyYyYyY")
    return render(["".join(r) for r in rows], salt=len(name), mapping=TIERS[MACHINE_TIER[name]])

TOP = [
    "3333333333333333",
    "3111111111111113",
    "31a2222222222a13",
    "3122222222222213",
    "3122hhhhhhhh2213",
    "3122222222222213",
    "3122hhhhhhhh2213",
    "3122222222222213",
    "3122hhhhhhhh2213",
    "3122222222222213",
    "3122hhhhhhhh2213",
    "3122222222222213",
    "3122222222222213",
    "31a2222222222a13",
    "3111111111111113",
    "3333333333333333",
]

# Saída: abertura com faixa de segurança.
PORT = [
    "cccccccccccccccc",
    "cccccccccccccccc",
    "ceeeeeeeeeeeeeec",
    "cebbbbbbbbbbbbec",
    "cebcccccccccbbec",
    "cebceeeeeeeecbec",
    "cebcehhhhhhecbec",
    "cebcehkkkkhecbec",
    "cebcehkkkkhecbec",
    "cebcehhhhhhecbec",
    "cebceeeeeeeecbec",
    "cebbbbbbbbbbbbec",
    "ceykykykykykykec",
    "ckykykykykykykyc",
    "cccccccccccccccc",
    "cccccccccccccccc",
]

BELT = [
    "gggggggggggggggg",
    "gggggggffggggggg",
    "ggggggfggfgggggg",
    "gggggfggggfggggg",
    "ggggfggggggfgggg",
    "gggggggggggggggg",
    "gggggggggggggggg",
    "gggggggffggggggg",
    "ggggggfggfgggggg",
    "gggggfggggfggggg",
    "ggggfggggggfgggg",
    "gggggggggggggggg",
    "gggggggggggggggg",
    "gggggggffggggggg",
    "ggggggfggfgggggg",
    "gggggfggggfggggg",
]

def stripes(light, dark="k"):
    return ["".join(light if (x + y) % 8 < 4 else dark for x in range(16)) for y in range(16)]

SPLITTER_TOP = [
    "eeeeeeeeeeeeeeee",
    "ebbbbbbbbbbbbbbe",
    "ebccccccccccccbe",
    "ebccccccccccccbe",
    "ebcccxccccxcccbe",
    "ebccxxccccxxccbe",
    "ebcxxxxxxxxxxcbe",
    "ebxxxxxxxxxxxxbe",
    "ebcxxxxxxxxxxcbe",
    "ebccxxccccxxccbe",
    "ebcccxccccxcccbe",
    "ebccccccccccccbe",
    "ebccccccccccccbe",
    "ebbbbbbbbbbbbbbe",
    "eeeeeeeeeeeeeeee",
    "eeeeeeeeeeeeeeee",
]

BATTERY = [
    "TTTTTTTTTTTTTTTT",
    "TqqqqqqqqqqqqqqT",
    "TrrrrrrrrrrrrrrT",
    "TSSSSSSSSSSSSSST",
    "TSMMMMMMMMMMMMST",
    "TSMkkkkkkkkkkMST",
    "TSMkHHHHHHHHkMST",
    "TSMkkkkkkkkkkMST",
    "TSMkHHHHHHHHkMST",
    "TSMkkkkkkkkkkMST",
    "TSMkGGGGGGGGkMST",
    "TSMkkkkkkkkkkMST",
    "TSMkJJJJJJJJkMST",
    "TSMkkkkkkkkkkMST",
    "TSSSSSSSSSSSSSST",
    "TTTTTTTTTTTTTTTT",
]

COIL = [
    "tqqqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
    "tqpqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
    "tqpqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
    "tqpqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
    "tqpqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
    "tqpqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
    "tqpqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
    "tqpqqqqqqqqqqqqt",
    "tsrrrrrrrrrrrrst",
]

# ---------------------------------------------------------------- itens
def plate(light, mid, dark, edge):
    """Chapa quadrada em perspectiva, com espessura, diferente dos lingotes vanilla."""
    rows = [["."] * 16 for _ in range(16)]
    top, bottom = 4, 10
    for y in range(top, bottom + 1):
        shift = (bottom - y) // 2  # A face superior inclina para a direita.
        left, right = 2 + shift, 11 + shift
        for x in range(left, right + 1):
            border = x in (left, right) or y in (top, bottom)
            tone = edge if border else (light if x - shift < 6 and y < 7 else mid if x - shift < 9 else dark)
            rows[y][x] = tone
    for y in (bottom + 1, bottom + 2):  # Espessura.
        for x in range(2, 12):
            rows[y][x] = edge if y == bottom + 2 or x in (2, 11) else dark
    rows[top + 1][5] = rows[top + 1][6] = "w"
    return ["".join(r) for r in rows]

def gear(light, mid, dark, edge):
    rows = [["."] * 16 for _ in range(16)]
    for y in range(16):
        for x in range(16):
            dx, dy = x - 7.5, y - 7.5
            r2 = dx * dx + dy * dy
            tooth = (abs(dx) < 1.6 or abs(dy) < 1.6 or abs(abs(dx) - abs(dy)) < 1.2)
            if r2 <= 2.2 ** 2:
                continue  # Furo central.
            if r2 <= 5.3 ** 2:
                rows[y][x] = light if dx + dy < -2 else mid if dx + dy < 3 else dark
            elif r2 <= 7.4 ** 2 and tooth:
                rows[y][x] = mid
    for y in range(16):
        for x in range(16):
            if rows[y][x] != "." and any(
                not (0 <= x + i < 16 and 0 <= y + j < 16) or rows[y + j][x + i] == "."
                for i, j in ((1, 0), (-1, 0), (0, 1), (0, -1))
            ):
                rows[y][x] = edge
    return ["".join(r) for r in rows]

WIRE = [
    "................",
    "................",
    "...NNNNNNNNNN...",
    "...NnnnnnnnnN...",
    "....tqpqpqpt....",
    "....trqrqrqt....",
    "....tqpqpqpt....",
    "....trqrqrqtq...",
    "....tqpqpqptrq..",
    "....trqrqrqt.rq.",
    "....tqpqpqpt..r.",
    "....trqrqrqt....",
    "...NnnnnnnnnN...",
    "...NNNNNNNNNN...",
    "................",
    "................",
]

CIRCUIT = [
    "................",
    "................",
    ".JJJJJJJJJJJJJJ.",
    ".JHGGGGGGGGGGHJ.",
    ".JGyyyGkkkGyGGJ.",
    ".JGGGyGkkkGyyGJ.",
    ".JGyGyGkkkGGyGJ.",
    ".JGyyyyyyyyyyGJ.",
    ".JGGGGyGGGyGGGJ.",
    ".JHxGGyGGGyGuHJ.",
    ".JJJJJJJJJJJJJJ.",
    "..Y.Y.Y..Y.Y.Y..",
    "................",
    "................",
    "................",
    "................",
]

SILICON = [
    "................",
    ".....hhhhhh.....",
    "...hhUUuuUUhh...",
    "..hUUUuvuUUUUh..",
    "..hUUuvuUUUUUh..",
    ".hUUuvuUUUUUUUh.",
    ".hUuvuUUUUUUUUh.",
    ".hUUuUUUUUUUuUh.",
    ".hUUUUUUUUUuvUh.",
    ".hUUUUUUUUuvuUh.",
    "..hUUUUUUuvuUh..",
    "..hUUUUUUUuUUh..",
    "...hhUUUUUUhh...",
    ".....hhhhhh.....",
    "................",
    "................",
]

FRAME = [
    "................",
    "................",
    "..eeeeeeeeeeee..",
    "..eaaaaaaaaaae..",
    "..eabbbbbbbbce..",
    "..eab.e..e.bce..",
    "..eab..ee..bce..",
    "..eab..ee..bce..",
    "..eab.e..e.bce..",
    "..eabbbbbbbbce..",
    "..eacccccccccce.",
    "..eeeeeeeeeeeee.",
    "................",
    "................",
    "................",
    "................",
]

MODULE = [
    "................",
    "..eeeeeeeeeeee..",
    "..eaaaaaaaaaae..",
    "..eab..qq..bce..",
    "..eab.qqqq.bce..",
    "..eabqqrrqqbce..",
    "..eabqr..rqbce..",
    "..eabqr..rqbce..",
    "..eabqqrrqqbce..",
    "..eab.qqqq.bce..",
    "..eab..qq..bce..",
    "..eacccccccccce.",
    "..eeeeeeeeeeeee.",
    "...YzY....YzY...",
    "................",
    "................",
]

WRENCH = [
    "................",
    "...........ee...",
    "..........eabe..",
    "...........eabe.",
    ".........e..ebe.",
    "........ecbbce..",
    ".......ecbcee...",
    "......ecbce.....",
    ".....ecbce......",
    "....xcbce.......",
    "...xxxce........",
    "..xXxxe.........",
    "..xxXe..........",
    "...ee...........",
    "................",
    "................",
]

REPAIR_KIT = [
    "................",
    "................",
    "......eeee......",
    ".....e....e.....",
    "..xxxxxxxxxxxx..",
    ".xXXXXXXXXXXXXx.",
    ".xXxxxxxxxxxxXx.",
    ".xXxxxxbbxxxxXx.",
    ".xXxxxxbbxxxxXx.",
    ".kkkkkkbbkkkkkk.",
    ".xXxxxxxxxxxxXx.",
    ".xXxxxxxxxxxxXx.",
    ".xXXXXXXXXXXXXx.",
    "..xxxxxxxxxxxx..",
    "................",
    "................",
]

def module_card(light):
    """Placa de módulo com a luz da cor do efeito, como os módulos do Factorio."""
    rows = [
        "................",
        "................",
        "..TTTTTTTTTTTT..",
        "..TSSSSSSSSSST..",
        "..TS11111111ST..",
        "..TS12222221ST..",
        "..TS12kkkk21ST..",
        "..TS12kkkk21ST..",
        "..TS12kkkk21ST..",
        "..TS12222221ST..",
        "..TS11111111ST..",
        "..TSSSSSSSSSST..",
        "..TTTTTTTTTTTT..",
        "...Y.Y.Y.Y.Y....",
        "................",
        "................",
    ]
    return render(rows, salt=len(light), mapping={"1": light[0], "2": light[1]})

FLOUR = [
    "................",
    "................",
    "......nnnn......",
    ".....n....n.....",
    "......nnnn......",
    ".....bwwwwb.....",
    "....bwwwwwwb....",
    "...bwwwWwwwwb...",
    "...bwwwwwwwwb...",
    "...bwwwwwWwwb...",
    "...bwwwwwwwwb...",
    "...cbwwwwwwbc...",
    "....ccbbbbcc....",
    "................",
    "................",
    "................",
]

BIOMASS = [
    "................",
    "................",
    "................",
    "...KKKKKKKKKK...",
    "..KLLKLLLKLLLK..",
    "..KLVLLKLLVLLK..",
    "..KLLLKLLLLKLK..",
    "..KKLLLLVLLLLK..",
    "..KLLKLLLLKLLK..",
    "..KLLLLVLLLLKK..",
    "..KVLLKLLLKLLK..",
    "...KKKKKKKKKK...",
    "................",
    "................",
    "................",
    "................",
]

def material():
    """Textura clara e uniforme que o bloco tinge nas luzes e peças animadas."""
    rows = []
    for y in range(16):
        rows.append("".join("a" if jitter(x, y, 99) == 4 else "w" for x in range(16)))
    return render(rows, noise=False)

def main():
    out = {}
    for name in MACHINE_TIER:
        out[f"block/{name}_front"] = front(name)
    for tier, mapping in TIERS.items():
        out[f"block/machine_top_{tier}"] = render(TOP, salt=7, mapping=mapping)
    out["block/port"] = render(PORT, salt=3)
    out["block/belt"] = render(BELT, salt=5)
    out["block/belt_rail"] = render(stripes("y"), noise=False)
    out["block/fast_belt_rail"] = render(stripes("C"), noise=False)
    out["block/splitter_top"] = render(SPLITTER_TOP, salt=9)
    out["block/merger_top"] = render([
        "gggggggyyggggggg",
        "ggggggyyyygggggg",
        "gggggyyyyyyggggg",
        "gggggggyyggggggg",
        "gggggggyyggggggg",
        "gggggggyyggggggg",
        "gggggggyyggggggg",
        "gggggggyyggggggg",
        "ggyyyyyyyyyyyygg",
        "ggyyyyyyyyyyyygg",
        "gggggggggggggggg",
        "gggggggggggggggg",
        "gggggggggggggggg",
        "gggggggggggggggg",
        "gggggggggggggggg",
        "gggggggggggggggg",
    ], noise=False)
    out["block/battery_side"] = render(BATTERY, salt=11)
    out["block/tesla_coil"] = render(COIL, salt=13)
    out["gui/material"] = material()
    out["item/copper_plate"] = render(plate("p", "q", "r", "t"), salt=1)
    out["item/iron_plate"] = render(plate("a", "b", "c", "e"), salt=2)
    out["item/steel_plate"] = render(plate("M", "S", "T", "g"), salt=3)
    out["item/gear"] = render(gear("p", "q", "r", "t"), salt=4)
    out["item/wire"] = render(WIRE, salt=5)
    out["item/circuit"] = render(CIRCUIT, salt=6)
    out["item/silicon"] = render(SILICON, salt=7)
    out["item/frame"] = render(FRAME, salt=8)
    out["item/module"] = render(MODULE, salt=9)
    out["item/wrench"] = render(WRENCH, salt=10)
    out["item/repair_kit"] = render(REPAIR_KIT, salt=11)
    out["item/speed_module"] = module_card("uv")
    out["item/efficiency_module"] = module_card("HG")
    out["item/productivity_module"] = module_card("oO")
    out["item/flour"] = render(FLOUR, salt=12)
    out["item/biomass"] = render(BIOMASS, salt=13)
    for name, pixels in out.items():
        write_png(ROOT / f"{name}.png", pixels)
    print(f"{len(out)} texturas 16x16 em {ROOT.relative_to(ROOT.parents[3])}")

if __name__ == "__main__":
    main()
