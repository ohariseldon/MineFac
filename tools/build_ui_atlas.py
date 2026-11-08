"""Gera o atlas de ícones das telas do Veio e as coordenadas usadas pelo código nativo.

    python tools/build_ui_atlas.py

Escreve `content/assets/veio/textures/gui/ui.png` (desenho próprio, pixel a pixel) e
`native/api/src/ui/icons.rs`. As telas desenham cada ícone com `Host::sprite("veio:gui/ui", ...)`.
Para mudar ou acrescentar um ícone, edite a grade abaixo e rode de novo.
"""
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
PNG = ROOT / "content/assets/veio/textures/gui/ui.png"
RUST = ROOT / "native/api/src/ui/icons.rs"
SIZE = 256

PALETTE = {
    "k": 0x1b1b1b, "K": 0x3a3a3a, "g": 0x8b8b8b, "G": 0xc6c6c6, "w": 0xffffff, "W": 0xe8e8e8,
    "y": 0xf8d040, "Y": 0xc89a10, "o": 0xe8761c, "O": 0xa04a10, "r": 0xd83a2a, "R": 0x8a1e14,
    "b": 0x3f76e4, "B": 0x234a9a, "c": 0x9fd0ff, "e": 0x3cc864, "E": 0x1e7a3a, "l": 0x6ab04a,
    "L": 0x3e7a2a, "n": 0x9a6a3a, "N": 0x5e3e1e, "t": 0xe0c060, "T": 0xa88a30, "p": 0x9a6ad8,
    "P": 0x5a3a9a, "s": 0xd8a070, "S": 0xa06a40, "m": 0xd07a3a, "M": 0x8a4a1e, "i": 0xe0e0e0,
    "I": 0x9a9a9a, "v": 0x7ad0c0, "V": 0x2e8a7a, "u": 0xfff0a0,
}

ICONS = {
    "bolt": """
....kkkk
...kyyyk
...kyyk.
..kyyk..
.kyyykkk
kyyyyyyk
kkkkyyk.
...kyyk.
..kyyk..
..kyk...
.kyk....
.kk.....""",
    "flame": """
....k....
...kok...
...kook..
..koyok..
..koyyok.
.koyyyok.
.koyyuyok
koyyuuyok
koyuuuyok
.koyuuok.
..kkkkk..""",
    "drop": """
...k...
...k...
..kbk..
..kbk..
.kbbbk.
kbbbbbk
kbcbbbk
kbcbbbk
.kbbbk.
..kkk..""",
    "lava": """
...k...
...k...
..kok..
..kok..
.kooOk.
kooooOk
koyooOk
koyooOk
.kooOk.
..kkk..""",
    "clock": """
..kkkkk..
.kWWkWWk.
kWWWkWWWk
kWWWkWWWk
kWWWkkkWk
kWWWWWWWk
kWWWWWWWk
.kWWWWWk.
..kkkkk..""",
    "gauge": """
...kkkkk...
.kkWWWWWkk.
kWWWWWWWkWk
kWWWWWWkWWk
kWWWWWkWWWk
kWWWWkkWWWk
kkkkkkkkkkk""",
    "sun": """
.....y.....
.y...y...y.
..y.....y..
....ooo....
...oyyyo...
yy.oyuyo.yy
...oyyyo...
....ooo....
..y.....y..
.y...y...y.
.....y.....""",
    "moon": """
..kkkk...
.kWWWk...
kWWWk....
kWWk.....
kWWk.....
kWWWk..k.
.kWWWkkWk
..kWWWWk.
...kkkk..""",
    "wind": """
.......BB..
..BBBBB..B.
.........B.
BBBBBBBBB..
...........
.BBBBBBB...
........B..
......BB...""",
    "emerald": """
...kkk...
..keeek..
.kewEeek.
kewEeeeEk
keeeeeeEk
keeeeeEEk
keeeeeEEk
.keeeEEk.
..keEEk..
...kkk...""",
    "wheat": """
....t....
...tTt...
..tTtTt..
...tTt...
..tTtTt..
...tTt...
..tTtTt..
....T....
....T....
...lTl...
....T....
....T....""",
    "bread": """
..kkkkkkk..
.knnttnnnk.
knntnnttnnk
knnnnnnnnnk
kNnnnnnnnNk
.kNNNNNNNk.
..kkkkkkk..""",
    "person": """
.kkkkkkkk.
kssssssssk
kssssssssk
kKKKKKKKKk
kswesswesk
kssssSsssk
kssssSsssk
kssSSSSssk
.kkSSSSkk.
...kkkk...""",
    "house": """
.....k.....
....krk....
...krrrk...
..krrrrrk..
.krrrrrrrk.
kkkkkkkkkkk
.kWWWWWWWk.
.kWnnWkkWk.
.kWnnWkcWk.
.kkkkkkkkk.""",
    "chest": """
.kkkkkkkkk.
knnnnnnnnnk
kNNNNNNNNNk
kkkkkGkkkkk
knnnnGnnnnk
knnnnnnnnnk
knnnnnnnnnk
kNNNNNNNNNk
.kkkkkkkkk.""",
    "ship": """
......k.......
......kW......
......kWW.....
......kWWW....
......kWWWW...
......kWWWWW..
......k.......
kkkkkkkkkkkkkk
.knnnnnnnnnnk.
..kNNNNNNNNk..
...kkkkkkkk...""",
    "train": """
..kk..........
..kk....kkkkk.
.kkkkkkkkcccck
.krrrrrrkcccck
.krrrrrrkkkkkk
.krrrrrrrrrrrk
kkkkkkkkkkkkkk
.kIk..kIk.kIk.
..k....k...k..""",
    "drone": """
kkk.....kkk
..k.....k..
..kkkkkkk..
.kbbbbbbbk.
.kbcbbbbbk.
..kkkkkkk..
....k.k....""",
    "llama": """
kk........
kWk.......
kWWk......
.kWk......
.kWkkkkkk.
.kWWWWWWWk
.kWWWWWWWk
.kWkkkkWk.
.kWk..kWk.
.kk....kk.""",
    "hammer": """
.kkkk......
kIiiIk.....
kIiiIIk....
kIIIInk....
.kkknnk....
....knnk...
.....knnk..
......knnk.
.......knnk
........kk.""",
    "gear": """
....kkk....
..k.kIk.k..
.kIkkIkkIk.
..kIIIIIk..
kkkIIkIIkkk
kIIIk.kIIIk
kkkIIkIIkkk
..kIIIIIk..
.kIkkIkkIk.
..k.kIk.k..
....kkk....""",
    "warning": """
.....k.....
....kyk....
....kyk....
...kyyyk...
...kykyk...
..kyykyyk..
..kyykyyk..
.kyyyyyyyk.
.kyyykyyyk.
kkkkkkkkkkk""",
    "check": """
.......EE
......EeE
E....EeE.
EE..EeE..
EeEEeE...
.EeeE....
..EE.....""",
    "cross": """
RR...RR
RrR.RrR
.RrRrR.
..RrR..
.RrRrR.
RrR.RrR
RR...RR""",
    "heart": """
.kk...kk.
krrk.krrk
krwrkrrrk
krrrrrrrk
.krrrrrk.
..krrrk..
...krk...
....k....""",
    "star": """
.....k.....
....kyk....
....kyk....
kkkkyyykkkk
.kyyyyyyyk.
..kyyyyyk..
...kyyyk...
..kyykyyk..
.kyk...kyk.
.kk.....kk.""",
    "scroll": """
.kkkkkkkk.
kWWWWWWWWk
.kWkkkkWk.
.kWWWWWWk.
.kWkkkkWk.
.kWWWWWWk.
.kWkkkWWk.
.kWWWWWWk.
kWWWWWWWWk
.kkkkkkkk.""",
    "chart": """
k.........
k.......ee
k.......ee
k....bb.ee
k....bb.ee
k.rr.bb.ee
k.rr.bb.ee
k.rr.bb.ee
kkkkkkkkkk""",
    "pickaxe": """
.kkkkkk....
kiiiiiIk...
.kkk.kIIk..
....knkIk..
...knk.kIk.
..knk...kk.
.knk.......
knk........
kk.........""",
    "sapling": """
....lll..
...lLlll.
..lLllll.
..lLlll..
...lLl...
....n....
.ll.n....
lLlln....
.ll.n....
....n....""",
    "cow": """
kk......kk
kWkkkkkkWk
.kWWnnWWk.
.kWknnkWk.
.kWnnnnWk.
.kWWWWWWk.
.ksSSSSsk.
.ksksskSk.
..kkkkkk..""",
    "shield": """
kkkkkkkkk
kIIIrIIIk
kIIIrIIIk
krrrrrrrk
kIIIrIIIk
.kIIrIIk.
.kIIrIIk.
..kIrIk..
...kIk...
....k....""",
    "smoke": """
...KKK.....
..KggKK.KK.
.KggggKKggK
KgggggggggK
KggggggggK.
.KKgggKKK..
...KKK.....""",
    "hourglass": """
kkkkkkk
kWWWWWk
.kttWk.
..ktk..
...k...
..kWk..
.kWtWk.
ktttttk
kkkkkkk""",
    "flag": """
kk......
kkrrrrr.
kkrrrrrr
kkrrrrr.
kk......
kk......
kk......
kk......
kk......
kk......""",
    "lock": """
..kkkk..
.kk..kk.
.k....k.
kkkkkkkk
kyyyyyyk
kyyykyyk
kyyykyyk
kyyyyyyk
kkkkkkkk""",
    "info": """
..kkkkk..
.kbbwbbk.
kbbbbbbbk
kbbbwbbbk
kbbbwbbbk
kbbbwbbbk
kbbbwbbbk
.kbbbbbk.
..kkkkk..""",
    "cart": """
k..........
.k.........
..kkkkkkkkk
..knnnnnnnk
..kNNNNNNNk
...kkkkkkk.
...kk...kk.
...kk...kk.""",
    "battery": """
...kkk...
.kkkkkkk.
.kIIIIIk.
.keeeeek.
.keeeeek.
.keeeeek.
.kEEEEEk.
.kEEEEEk.
.kEEEEEk.
.kkkkkkk.""",
    "factory": """
...........kk.
.k...k.....kk.
.kk..kk....kk.
.kkk.kkk...kk.
.kIkkkIkkkkkk.
.kIIIIIIIIIIk.
.kIcIcIIcIcIk.
.kIIIIIIIIIIk.
.kkkkkkkkkkkk.""",
    "leaf": """
.....kkk.
...kklllk
..klllLlk
.kllLlllk
.klLllllk
kllLllk..
k.Lkkk...
.k.......""",
    "pot": """
...W.W...
..W.W.W..
.........
kkkkkkkkk
.kIIIIIk.
.kIiIIIk.
.kIIIIIk.
..kkkkk..""",
    "plate": """
...........
..kkkkkkk..
.kmmmmmmmk.
kmmmmmmmmMk
kMMMMMMMMMk
.kkkkkkkkk.""",
    "saw": """
...k.k.k...
..kkIkIkk..
.kIIIIIIIk.
kkIIkkkIIkk
.kIIkWkIIk.
kkIIkkkIIkk
.kIIIIIIIk.
..kkIkIkk..
...k.k.k...""",
    "crush": """
.kkk..kkk..
kIIIkkIIIk.
kIkIkkIkIk.
kIIIkkIIIk.
.kkk..kkk..
...k..k....
...........""",
    "chip": """
.k.k.k.k.
kkkkkkkkk
.kVVVVVk.
kkVvvvVkk
.kVvVvVk.
kkVvvvVkk
.kVVVVVk.
kkkkkkkkk
.k.k.k.k.""",
    "flask": """
..kkk..
...k...
..kWk..
..kWk..
.kWWWk.
kWppppk
kppPppk
kppppPk
.kkkkk.""",
    "anvil": """
kkkkkkkkkk.
kIIIIIIIIIk
.kkIIIIkkk.
...kIIk....
..kIIIIk...
.kkkkkkkk..""",
    "pickup": """
...k...
..kyk..
.kyyyk.
kyyyyyk
..kyk..
..kyk..
..kkk..""",
    "dropoff": """
..kkk..
..kyk..
..kyk..
kyyyyyk
.kyyyk.
..kyk..
...k...""",
    "people": """
..kkk..kkk..
.ksssk.kssk.
.ksesk.kesk.
.kssskkkssk.
kkkkkkkkkkkk
kbbbbkkrrrrk
kbbbbk.krrrk
kkkkkk.kkkkk""",
    "crown": """
k...k...k
kk.kyk.kk
kykyyykyk
kyyyyyyyk
kyryyyryk
kkkkkkkkk""",
    "wrench": """
.kk...kk.
kIIk.kIIk
kIIIkIIIk
.kIIIIIk.
..kIIIk..
...kIk...
...kIk...
...kIk...
...kIk...
...kkk...""",
    "bed": """
k.........
kk........
kWkkkkkkkk
kWWrrrrrrk
kkkkkkkkkk
k........k""",
    "tree": """
...kkkk...
..klllLk..
.kllLlllk.
kllllLlllk
kLlllllLlk
.kLLlLLLk.
..kkknkk..
....knk...
....knk...
...kknkk..""",
    "seeds": """
..l....
.lLl.l.
..l.lLl
l....l.
Ll..l..
.l.lLl.
....l..""",
}

# Peças maiores para as telas: seta de progresso e chama da queima, vazias e cheias.
def arrow(fill, edge):
    rows = []
    for y in range(15):
        row = ""
        for x in range(22):
            inside = (x < 14 and 5 <= y <= 9) or (x >= 13 and abs(y - 7) <= (21 - x) * 7 / 8)
            row += fill if inside else "."
        rows.append(row)
    # Contorno: ponto aceso vizinho de ponto apagado.
    out = []
    for y, row in enumerate(rows):
        line = ""
        for x, c in enumerate(row):
            if c == ".":
                line += "."
                continue
            near = [rows[y + dy][x + dx] if 0 <= y + dy < 15 and 0 <= x + dx < 22 else "."
                    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))]
            line += edge if "." in near else c
        out.append(line)
    return "\n" + "\n".join(out)


ICONS["arrow_empty"] = arrow("g", "K")
ICONS["arrow_full"] = arrow("w", "G")
ICONS["burn_empty"] = """
......K......
.....KgK.....
.....KggK....
....KgggK....
....KggggK...
...KgggggK.K.
..KggggggKKgK
.KggggggggggK
KgggggggggggK
KgggggggggggK
KggggggggggK.
.KgggggggggK.
..KKKKKKKKK..""".replace("K", "K")
ICONS["burn_full"] = """
......k......
.....kok.....
.....kook....
....kooyk....
....koyyok...
...koyyyok.k.
..koyyuyokkok
.koyyuuyyyyok
koyyuuuuyyyok
koyuuuuuuyyok
koyuuuuuuyok.
.koyyuuyyyok.
..kkkkkkkkk..""".replace("k", "O")


def parse(art):
    rows = [row for row in art.strip("\n").split("\n")]
    width = max(len(row) for row in rows)
    return [row.ljust(width, ".") for row in rows]


def main():
    sheet = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    x = y = 1
    shelf = 0
    lines = ["//! Ícones do atlas `veio:gui/ui`. Gerado por `tools/build_ui_atlas.py`; não edite à mão.",
             "use super::Icon;", ""]
    for name, art in ICONS.items():
        rows = parse(art)
        width, height = len(rows[0]), len(rows)
        if x + width + 1 > SIZE:
            x, y, shelf = 1, y + shelf + 1, 0
        for row, line in enumerate(rows):
            for column, char in enumerate(line):
                if char == ".":
                    continue
                color = PALETTE[char]
                sheet.putpixel((x + column, y + row), ((color >> 16) & 255, (color >> 8) & 255, color & 255, 255))
        lines.append(f"pub(crate) const {name.upper()}: Icon = Icon {{ u: {x}.0, v: {y}.0, w: {width}.0, h: {height}.0 }};")
        x += width + 1
        shelf = max(shelf, height)
    assert y + shelf <= SIZE, "o atlas encheu"
    PNG.parent.mkdir(parents=True, exist_ok=True)
    sheet.save(PNG)
    RUST.parent.mkdir(parents=True, exist_ok=True)
    RUST.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"{len(ICONS)} ícones em {PNG.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
