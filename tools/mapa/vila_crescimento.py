"""O plano de crescimento da vila: o que cada estágio da história acrescenta, sem derrubar nada.

A vila nasce orgânica (estágio 1, `vila_estagio1.py`): ruas de terra saindo da praça. Ela cresce
por anéis irregulares que ligam essas ruas e formam quarteirões tortos, como as vilas antigas que
viram cidade. Cada construção do estágio 1 fica onde está (pode ganhar andar, telhado novo, toldo),
e o novo entra nos espaços livres ao longo das ruas novas.

Estágios (a história em `frentes/HISTORIA.md`):
1. A chegada: a vila em declínio.
2. A vila respira (os pedidos da vila e a noite, capítulo 2): fonte com água, coreto com telhado,
   casas em obra prontas, a abandonada reformada, o Anel da Praça em cascalho, lanternas, a muralha.
3. A vila operária (capítulo 3): a prefeitura, os postos de trabalho, casas nos lotes vazios, o
   Anel de Fora começa.
4. A vila cresce (capítulo 4, fase Vila): o Anel de Fora fecha os quarteirões, poço novo, parque,
   a vendinha vira a loja, mais casas.
5. A grande indústria (capítulo 5): o distrito industrial, o quartel e a torre Tesla.
6. A cidade (capítulo 6, fase Cidade): ruas de pedra, a ponte e o bairro da margem, a praça cívica.

Uso: python3 tools/mapa/vila_crescimento.py [pasta de saída]  -> vila_crescimento.png
A saída lista conflitos (rua ou lote novo em cima de algo que já existe); deve sair vazia.
"""
import math
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageEnhance

sys.path.insert(0, str(Path(__file__).parent))
import vila_estagio1 as v1  # noqa: E402

STAGE_COLOR = {2: (110, 200, 255), 3: (255, 170, 60), 4: (255, 225, 90), 5: (200, 200, 210), 6: (255, 110, 150)}
STAGE_NAME = {1: "A chegada", 2: "A vila respira", 3: "A vila operária", 4: "A vila cresce", 5: "A grande indústria", 6: "A cidade"}

# Ruas novas: (estágio, nome, pontos, largura). Passam entre as construções do estágio 1.
STREETS = [
    # o Anel da Praça passa entre a praça e a casa da lavradora (a roça fica de fora)
    (2, "Anel da Praça", [(-24, -15), (-8, -16), (10, -16), (30, -14), (39, -10), (40, 0), (39, 13), (33, 22), (28, 36),
                          (8, 38), (-8, 39), (-26, 38), (-26, 26), (-15, 16), (-16, 6), (-19, -8), (-24, -15)], 3),
    (3, "Rua da Prefeitura", [(30, -14), (34, -18)], 3),
    (3, "Anel de Fora (norte)", [(-62, -28), (-46, -50), (-20, -62), (4, -64), (20, -66), (42, -62), (64, -48), (69, -30)], 3),
    (4, "Anel de Fora (leste e sul)", [(69, -30), (70, -4), (68, 22), (62, 44), (64, 62), (52, 84), (20, 86), (-14, 86), (-46, 74),
                                       (-48, 58), (-56, 44), (-62, 10), (-62, -28)], 3),
    (4, "Travessa do Parque", [(8, 38), (6, 52)], 2),
    (4, "Travessa da Ruína", [(10, -16), (14, -40), (22, -66)], 2),
    (6, "Rua da Ponte", [(68, 22), (96, 22), (126, 24)], 4),
    (6, "Rua do Sul", [(-14, 86), (-12, 104)], 4),
]
# Construções novas: (estágio, nome, x, z, largura, fundo)
NEW = [
    (2, "Coreto com telhado", -8, -4.5, 9, 8), (2, "Fonte com água", 0, 0.5, 9, 9),
    (3, "Prefeitura", 34, -26, 12, 9), (3, "Posto de lavoura", -48, 4, 5, 5), (3, "Posto de pastor", -62, 52, 5, 5),
    (3, "Mesa do capataz", -10, -58, 5, 4),
    (4, "Parque", 6, 58, 12, 8), (4, "Poço novo", -20, 13, 4, 4),
    (5, "Quartel", -20, -50, 12, 9), (5, "Torre Tesla", 66, -8, 3, 3), (5, "Distrito industrial", 58, -10, 18, 14),
    (6, "Praça cívica", 40, -86, 16, 12), (6, "Ponte", 112, 26, 18, 6), (6, "Bairro da Margem", 132, 20, 12, 30),
]
# Casas novas: (estágio, x, z) — 10 x 8, viradas para a rua
NEW_HOUSES = [
    (3, -38, -40), (3, 50, 70), (3, -34, 42), (3, 16, 70),                    # os lotes vazios do estágio 1
    (3, -36, -24),
    (4, -34, -68), (4, 8, -52), (4, 36, -72), (4, 56, -70), (4, 78, -12), (4, 78, 12), (4, 72, 54), (4, 28, 96),
    (4, -30, 96), (4, -36, 64), (4, -48, 40), (4, -76, 4), (4, -76, -18), (4, -52, -64),
    (6, 4, 96), (6, -42, 98), (6, 90, 60), (6, 90, -40), (6, -14, -78),
]


# A muralha (estágio 2, o capítulo da noite): de margem a margem do rio, por fora do pasto e da carvoaria.
WALL = [(95, -94), (60, -91), (0, -93), (-60, -89), (-110, -68), (-118, -20), (-112, 40), (-104, 101), (-60, 104), (0, 105)]
LABEL_AT = {"Travessa da Ruína": (26, -50), "Rua da Prefeitura": None, "Travessa do Parque": (20, 46)}


def rect_of(x, z, w, h):
    return (x - w / 2, z - h / 2, x + w / 2, z + h / 2)


def overlap(a, b, gap=1.0):
    return not (a[2] + gap <= b[0] or b[2] + gap <= a[0] or a[3] + gap <= b[1] or b[3] + gap <= a[1])


def seg_rect_distance(p, q, rect):
    """Distância do segmento ao retângulo (0 se cruzam), por amostragem fina."""
    best = 1e9
    steps = int(max(abs(q[0] - p[0]), abs(q[1] - p[1]))) * 2 + 1
    for k in range(steps + 1):
        t = k / steps
        x, z = p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t
        dx = max(rect[0] - x, 0, x - rect[2]); dz = max(rect[1] - z, 0, z - rect[3])
        best = min(best, math.hypot(dx, dz))
    return best


def existing():
    """Tudo o que o estágio 1 ocupa: (nome, retângulo)."""
    out = [(name, rect_of(x, z, w, h)) for name, (x, z, w, h) in v1.BUILDINGS.items()]
    out += [(f"casa {x},{z}", rect_of(x, z, 10, 8)) for x, z, _ in v1.HOUSES]
    out += [(f"obra {x},{z}", rect_of(x, z, 12, 10)) for x, z in v1.WORKS]
    out += [(f"ruína {x},{z}", rect_of(x, z, 10, 8)) for x, z in v1.RUINS]
    out += [(name, rect_of(x, z, w, h)) for name, (x, z, w, h) in v1.GROUNDS.items() if name != "Praça"]
    return out


def check():
    """Nada novo em cima do que já existe; casas novas não se sobrepõem; ruas novas não cortam casas."""
    problems = []
    old = existing()
    lots = [rect_of(x, z, 10, 8) for x, z in v1.LOTS]
    upgrades = ("Coreto com telhado", "Fonte com água", "Poço novo")
    news = [(f"{name} ({stage})", rect_of(x, z, w, h)) for stage, name, x, z, w, h in NEW if name not in upgrades]
    houses = [(f"casa nova {x},{z} ({stage})", rect_of(x, z, 10, 8)) for stage, x, z in NEW_HOUSES]
    for name, rect in news + houses:
        for other, orect in old:
            if overlap(rect, orect) and not any(rect == lot for lot in lots) \
                    and not (name.startswith("Distrito") and other == "Terreno da oficina"):
                problems.append(f"{name} em cima de {other}")
    every = news + houses
    for i, (a, ra) in enumerate(every):
        for b, rb in every[i + 1:]:
            if overlap(ra, rb, 0.5) and not {a.split(" (")[0], b.split(" (")[0]} == {"Torre Tesla", "Distrito industrial"}:
                problems.append(f"{a} encosta em {b}")
    for stage, sname, pts, width in STREETS:
        for name, rect in old + news + houses:
            if any(seg_rect_distance(p, q, rect) < width / 2 + 0.5 for p, q in zip(pts, pts[1:])):
                if name.startswith(("Distrito", "Ponte", "Bairro", "Praça cívica", "Prefeitura")) and sname in ("Rua da Ponte", "Rua da Prefeitura"):
                    continue
                problems.append(f"rua {sname} ({stage}) passa por {name}")
    return problems


def render(out):
    v = v1.Village()
    v1.terrain(v); v1.roads(v); v1.places(v); v1.vegetation(v)
    base = v1.render_map(v)
    base = ImageEnhance.Color(base).enhance(0.35)
    base = ImageEnhance.Brightness(base).enhance(0.7)
    im = base.convert("RGBA")
    over = Image.new("RGBA", im.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(over)
    S = v1.S
    P = lambda x, z: ((x - v1.X0) * S, (z - v1.Z0) * S)
    # a muralha (capítulo 2): contorna a vila por terra e fecha nas margens do rio, que defende o leste
    wall = WALL
    for p, q in zip(wall, wall[1:]):
        steps = int(math.hypot(q[0] - p[0], q[1] - p[1]) / 4)
        for k in range(0, steps, 2):
            a = (p[0] + (q[0] - p[0]) * k / steps, p[1] + (q[1] - p[1]) * k / steps)
            b = (p[0] + (q[0] - p[0]) * (k + 1) / steps, p[1] + (q[1] - p[1]) * (k + 1) / steps)
            d.line([P(*a), P(*b)], fill=STAGE_COLOR[2] + (235,), width=5)
    # ruas novas
    for stage, name, pts, width in STREETS:
        d.line([P(*p) for p in pts], fill=STAGE_COLOR[stage] + (235,), width=int(width * S), joint="curve")
    # construções novas e casas novas
    font = v1.font
    for stage, name, x, z, w, h in NEW:
        a = P(x - w / 2, z - h / 2); b = P(x + w / 2, z + h / 2)
        d.rectangle([a, b], fill=STAGE_COLOR[stage] + (150,), outline=STAGE_COLOR[stage] + (255,), width=3)
    for stage, x, z in NEW_HOUSES:
        a = P(x - 5, z - 4); b = P(x + 5, z + 4)
        d.rectangle([a, b], fill=STAGE_COLOR[stage] + (120,), outline=STAGE_COLOR[stage] + (255,), width=2)
        d.text(P(x, z), str(stage), fill=(20, 20, 20, 255), font=font(14, True), anchor="mm")
    im = Image.alpha_composite(im, over)
    d = ImageDraw.Draw(im)
    for stage, name, x, z, w, h in NEW:
        d.text(P(x, z + h / 2 + 2.5), f"{name} ({stage})", fill=(255, 255, 255), font=font(13, True), anchor="mm",
               stroke_width=3, stroke_fill=(0, 0, 0))
    for stage, name, pts, width in STREETS:
        mid = LABEL_AT.get(name, pts[len(pts) // 2])
        if mid is None:
            continue
        d.text(P(*mid), name, fill=STAGE_COLOR[stage], font=font(13, True), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0))
    for name, (x, z, w, h) in v1.BUILDINGS.items():
        d.text(P(x, z - h / 2 - 2.5), name, fill=(255, 255, 255), font=font(13), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0))
    d.text(P(-96, -86), "muralha (cap. 2)", fill=STAGE_COLOR[2], font=font(13, True), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0))
    # painel
    W, side = im.width, 520
    canvas = Image.new("RGB", (W + side, im.height + 104), (22, 24, 28))
    canvas.paste(im.convert("RGB"), (0, 104))
    cd = ImageDraw.Draw(canvas)
    cd.text((18, 14), "A vila cresce: o plano dos estágios", fill=(255, 255, 255), font=font(34, True))
    cd.text((18, 62), "Por baixo, apagado, o estágio 1 como ele é. Por cima, em cores, o que cada estágio acrescenta. Nada é derrubado.",
            fill=(200, 205, 215), font=font(15))
    x0, y = W + 22, 122
    for stage in range(1, 7):
        color = (180, 180, 180) if stage == 1 else STAGE_COLOR[stage]
        cd.rectangle([x0, y + 3, x0 + 16, y + 19], fill=color)
        cd.text((x0 + 26, y), f"Estágio {stage} · {STAGE_NAME[stage]}", fill=(255, 255, 255), font=font(17, True)); y += 26
        text = {
            1: ["A vila em declínio (a planta do estágio 1)."],
            2: ["Fonte com água e coreto com telhado; as 2 casas", "em obra ficam prontas e a abandonada é", "reformada; Anel da Praça em cascalho;", "lanternas; a muralha da noite."],
            3: ["Prefeitura, postos de lavoura e de pastor,", "mesa do capataz; casas nos 4 lotes vazios;", "começa o Anel de Fora."],
            4: ["O Anel de Fora fecha os quarteirões; poço novo,", "parque; a vendinha vira loja (no mesmo lugar);", "14 casas novas."],
            5: ["Distrito industrial no terraço do meio (a oficina", "cresce em volta do terreno), quartel e Tesla."],
            6: ["Ruas de pedra; a ponte e o bairro da margem;", "a Rua do Sul para a expansão; a praça cívica."],
        }[stage]
        for line in text:
            cd.text((x0 + 26, y), line, fill=(200, 205, 215), font=font(13)); y += 18
        y += 8
    y += 6
    cd.text((x0, y), "Regras", fill=(255, 255, 255), font=font(17, True)); y += 26
    for line in ["O que existe no estágio 1 fica no lugar: pode", "ganhar andar, telhado novo ou toldo, nunca sumir.",
                 "Ruas novas passam entre as casas e ligam as", "ruas de terra do começo: quarteirões irregulares.",
                 "Casas novas viradas para a rua, 10 x 8 blocos,", "nos espaços livres, sem tocar nas antigas.",
                 "O número em cada casa nova é o estágio dela.",
                 "As trocas acontecem longe dos olhos do jogador", "(a mais de 32 blocos) ou com um efeito de obra."]:
        cd.text((x0, y), line, fill=(200, 205, 215), font=font(13)); y += 18
    canvas.save(out)


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("docs/mapa/costa-oeste")
    out_dir.mkdir(parents=True, exist_ok=True)
    problems = check()
    print("conflitos:", "nenhum" if not problems else len(problems))
    for line in problems:
        print("  -", line)
    render(out_dir / "vila_crescimento.png")


if __name__ == "__main__":
    main()
