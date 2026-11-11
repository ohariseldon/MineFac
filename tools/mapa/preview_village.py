"""Planta da vila inicial (estágio 0, "a vila em declínio"), 400 x 400 blocos em volta da praça.

Os terraços descem de oeste para o rio, a leste: degraus de 2 blocos, no máximo 4 entre a praça e
a beira do rio, com escadas onde as ruas atravessam. Cada morador fica onde o ofício dele faz
sentido: a oleira na margem (argila), o carvoeiro na borda da floresta (lenha), o vaqueiro na
planície aberta (pasto), o ferreiro perto da praça e da estrada da serra (minério).

Uso: python3 tools/mapa/preview_village.py [pasta de saída]  -> vila.png
"""
import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

SIZE = 400                      # blocos de lado
S = 3                           # pixels por bloco
W = SIZE * S
PANEL = 470


def font(size):
    for path in ("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", "/usr/share/fonts/dejavu/DejaVuSans.ttf"):
        if Path(path).exists():
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def px(x, z):
    return ((x + SIZE / 2) * S, (z + SIZE / 2) * S)


def wobble(z, base, amp, seed):
    return base + amp * math.sin(z / 37 + seed) + amp * 0.5 * math.sin(z / 13 + seed * 2)


def river_x(z):
    return 150 + 14 * math.sin(z / 45) + 6 * math.sin(z / 17)


# Terraços: oeste (nível 0, a praça), meio (−2), margem (−4). Fronteira = x da quebra por z.
STEPS = [(-48, 0.0, "nível da praça"), (52, 1.3, "−2 blocos"), (118, 2.6, "−4 blocos (margem)")]
TERRACE_COLORS = [(168, 198, 112), (138, 176, 92), (106, 150, 78)]

ROADS = [  # (pontos, largura em blocos)
    ([(0, -6), (0, -34)], 3),                                   # praça → casa do fundador
    ([(0, -34), (-10, -90), (-24, -150), (-30, -172)], 3),       # estrada da serra (mina do avô)
    ([(6, 0), (30, 2)], 3),                                     # praça → ferreiro
    ([(-6, 0), (-36, 6)], 3),                                   # praça → lavradora
    ([(30, 2), (80, 8), (124, 34)], 3),                         # → oleira, cruzando dois degraus
    ([(-36, 6), (-62, 40), (-74, 66)], 2),                      # → pasto
    ([(-20, -20), (-70, -46), (-118, -56)], 2),                 # → carvoaria
    ([(4, 6), (14, 20)], 3),                                    # → vendinha
    ([(80, 8), (78, -18)], 2),                                  # → terreno da oficina
]

# (x, z, largura, profundidade, estado, rótulo)
BUILDINGS = [
    (0, -40, 13, 11, "ok", "Casa do fundador"),
    (34, -4, 12, 10, "ok", "Ferreiro (Seu Bento)"),
    (-42, 0, 11, 9, "ok", "Lavradora (Dona Cida)"),
    (18, 24, 10, 8, "ok", "Vendinha (Seu Zé)"),
    (126, 44, 12, 9, "ok", "Olaria (Dona Zefa)"),
    (-124, -60, 10, 8, "ok", "Carvoaria (Seu Lino)"),
    (-84, 64, 9, 7, "ok", "Vaqueiro (Seu Tonho)"),
    (-22, 30, 8, 8, "ok", ""), (24, -30, 8, 8, "ok", ""), (-30, -28, 8, 8, "ok", ""),
    (44, 26, 8, 8, "obra", ""), (-14, 52, 8, 8, "obra", ""),
    (54, -34, 8, 8, "ruina", ""),
    (-48, -40, 8, 8, "lote", ""), (56, 48, 8, 8, "lote", ""), (-44, 46, 8, 8, "lote", ""), (10, 58, 8, 8, "lote", ""),
]
STATE = {"ok": ((188, 140, 90), "habitada"), "obra": ((230, 200, 120), "em obra (andaime)"),
         "ruina": ((120, 105, 95), "abandonada (telhado caído)"), "lote": (None, "lote vazio (casa futura)")}


def main():
    out_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("mapa_previas")
    out_dir.mkdir(parents=True, exist_ok=True)
    img = Image.new("RGB", (W + PANEL, W + 90), (24, 26, 30))
    d = ImageDraw.Draw(img, "RGBA")
    oy = 90
    def P(x, z):
        a, b = px(x, z)
        return (a, b + oy)
    # serra ao norte (encosta), floresta a oeste, terraços
    for zi in range(-200, 200):
        for xi in range(-200, 200, 2):
            if zi < -150 + 10 * math.sin(xi / 30):
                shade = max(0, min(1, (-150 - zi) / 50))
                c = (int(150 - 40 * shade), int(160 - 50 * shade), int(120 - 20 * shade))
            elif xi < -120 + 12 * math.sin(zi / 25):
                c = (55, 110, 50)
            elif xi > river_x(zi) - 6 and xi < river_x(zi) + 6:
                c = (60, 110, 210)
            elif xi > river_x(zi) + 6:
                c = (120, 165, 85)
            else:
                level = sum(1 for bx, amp, _ in STEPS[:2] if xi > wobble(zi, bx, 6, amp))
                c = TERRACE_COLORS[level]
            d.rectangle([*P(xi, zi), P(xi + 2, zi + 1)[0], P(xi + 2, zi + 1)[1]], fill=c)
    # árvores da floresta e algumas soltas
    import random
    rnd = random.Random(4)
    for _ in range(420):
        x, z = rnd.uniform(-200, -125), rnd.uniform(-150, 200)
        if x < -120 + 12 * math.sin(z / 25) - 4:
            a, b = P(x, z); r = rnd.uniform(4, 7)
            d.ellipse([a - r, b - r, a + r, b + r], fill=(35, 85, 35, 230))
    # quebras dos terraços (linhas) e rótulos
    for bx, amp, label in STEPS[:2]:
        line = [P(wobble(z, bx, 6, amp), z) for z in range(-148, 200, 4)]
        d.line(line, fill=(90, 120, 60), width=3)
    # argila nas margens
    for z in range(-140, 200, 14):
        a, b = P(river_x(z) + rnd.choice([-8, 8]), z)
        d.ellipse([a - 6, b - 4, a + 6, b + 4], fill=(150, 160, 175))
    # ruas, com escadas onde cruzam um degrau
    for pts, width in ROADS:
        d.line([P(x, z) for x, z in pts], fill=(170, 140, 95), width=width * S, joint="curve")
        for (ax, az), (bx_, bz) in zip(pts, pts[1:]):
            for step_x, amp, _ in STEPS[:2]:
                for k in range(60):
                    t = k / 60
                    x, z = ax + (bx_ - ax) * t, az + (bz - az) * t
                    if abs(x - wobble(z, step_x, 6, amp)) < 1.2:
                        a, b = P(x, z)
                        for i in range(-2, 3):
                            d.line([(a - 6, b + i * 3), (a + 6, b + i * 3)], fill=(120, 120, 120), width=1)
                        break
    # pasto cercado com a Mimosa
    fence = [P(-100, 50), P(-60, 50), P(-60, 95), P(-100, 95)]
    d.polygon(fence, outline=(110, 80, 50), width=2)
    a, b = P(-80, 78); d.ellipse([a - 5, b - 4, a + 5, b + 4], fill=(240, 240, 240), outline=(60, 40, 30))
    d.text((a + 8, b - 2), "Mimosa", fill=(255, 255, 255), font=font(12), stroke_width=2, stroke_fill=(0, 0, 0))
    # roça da Dona Cida
    for i in range(6):
        d.rectangle([*P(-62, 12 + i * 3), *P(-44, 13.6 + i * 3)], fill=(205, 185, 90))
    # terreno da oficina (11 x 11) e o lugar da roda d'água
    a1, b1 = P(72, -28); a2, b2 = P(83, -17)
    d.rectangle([a1, b1, a2, b2], outline=(120, 255, 120), width=2)
    d.text(((a1 + a2) / 2, b1 - 10), "Terreno da oficina", fill=(200, 255, 200), font=font(13), anchor="mm", stroke_width=2, stroke_fill=(0, 0, 0))
    a, b = P(river_x(-22), -22); d.ellipse([a - 6, b - 6, a + 6, b + 6], outline=(255, 255, 255), width=2)
    d.text((a + 10, b), "roda d'água", fill=(255, 255, 255), font=font(12), anchor="lm", stroke_width=2, stroke_fill=(0, 0, 0))
    # praça, fonte quebrada, coreto sem telhado
    a1, b1 = P(-12, -10); a2, b2 = P(12, 10)
    d.rectangle([a1, b1, a2, b2], fill=(185, 180, 165))
    a, b = P(0, 0)
    d.ellipse([a - 10, b - 10, a + 10, b + 10], fill=(140, 135, 125), outline=(80, 80, 80), width=2)
    d.line([(a - 8, b - 3), (a + 2, b + 6)], fill=(70, 70, 70), width=2)
    a, b = P(-8, -7); d.rectangle([a - 7, b - 7, a + 7, b + 7], outline=(150, 90, 60), width=3)
    d.text(P(-62, -24), "Praça: fonte seca e quebrada,\ncoreto sem telhado, prefeito", fill=(255, 255, 255), font=font(12), stroke_width=2, stroke_fill=(0, 0, 0))
    # casas
    for x, z, w, h, state, label in BUILDINGS:
        color, _ = STATE[state]
        a1, b1 = P(x - w / 2, z - h / 2); a2, b2 = P(x + w / 2, z + h / 2)
        if color is None:
            d.rectangle([a1, b1, a2, b2], outline=(230, 230, 230), width=1)
            for k in range(0, int(a2 - a1), 6):
                d.line([(a1 + k, b1), (a1 + k, b2)], fill=(230, 230, 230, 90))
        else:
            d.rectangle([a1, b1, a2, b2], fill=color, outline=(60, 40, 30), width=2)
            if state == "obra":
                d.line([(a1, b1), (a2, b2)], fill=(120, 90, 40), width=2); d.line([(a1, b2), (a2, b1)], fill=(120, 90, 40), width=2)
            if state == "ruina":
                d.polygon([(a1, b1), ((a1 + a2) / 2, (b1 + b2) / 2), (a1, b2)], fill=(70, 60, 55))
        if label:
            d.text(((a1 + a2) / 2, b2 + 9), label, fill=(255, 255, 255), font=font(13), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0))
    # mina do avô e rótulos de região
    a, b = P(-30, -175); d.polygon([(a - 9, b + 6), (a, b - 8), (a + 9, b + 6)], fill=(40, 30, 25), outline=(200, 170, 120))
    d.text((a + 14, b), "Mina do avô (fechada)", fill=(255, 230, 180), font=font(13), anchor="lm", stroke_width=3, stroke_fill=(0, 0, 0))
    for text, x, z in [("Serra do Veio (encosta)", 60, -180), ("Floresta", -165, 20), ("Rio", 162, -120), ("Planície", 100, 150)]:
        d.text(P(x, z), text, fill=(255, 255, 255), font=font(15), anchor="mm", stroke_width=3, stroke_fill=(0, 0, 0))
    # cabeçalho, escala e painel
    d.text((16, 14), "A vila no estágio 0: a vila em declínio", fill=(255, 255, 255), font=font(28))
    d.text((16, 54), "400 x 400 blocos em volta da praça · os terraços descem para o rio em degraus de 2 blocos, com escadas nas ruas",
           fill=(200, 205, 215), font=font(15))
    a, b = 20, W + oy - 24
    d.line([(a, b), (a + 50 * S, b)], fill=(255, 255, 255), width=4); d.text((a, b + 4), "50 blocos", fill=(255, 255, 255), font=font(13))
    x0 = W + 24; y = oy + 10
    d.text((x0, y), "Casas", fill=(255, 255, 255), font=font(18)); y += 30
    for state in ("ok", "obra", "ruina", "lote"):
        color, label = STATE[state]
        if color: d.rectangle([x0, y, x0 + 18, y + 14], fill=color, outline=(60, 40, 30))
        else: d.rectangle([x0, y, x0 + 18, y + 14], outline=(230, 230, 230))
        d.text((x0 + 28, y - 1), label, fill=(220, 220, 220), font=font(14)); y += 24
    y += 14
    d.text((x0, y), "Terreno", fill=(255, 255, 255), font=font(18)); y += 30
    for c, label in zip(TERRACE_COLORS, ["praça (nível 0)", "terraço do meio (−2)", "margem do rio (−4)"]):
        d.rectangle([x0, y, x0 + 18, y + 14], fill=c); d.text((x0 + 28, y - 1), label, fill=(220, 220, 220), font=font(14)); y += 24
    d.rectangle([x0, y, x0 + 18, y + 14], fill=(150, 160, 175)); d.text((x0 + 28, y - 1), "argila na margem", fill=(220, 220, 220), font=font(14)); y += 24
    d.line([(x0, y + 7), (x0 + 18, y + 7)], fill=(170, 140, 95), width=6); d.text((x0 + 28, y - 1), "rua de terra (escadas nos degraus)", fill=(220, 220, 220), font=font(14)); y += 40
    notes = [
        "Por que cada um está ali",
        "• Oleira na margem: a argila está no rio.",
        "• Carvoeiro na borda da floresta: a lenha.",
        "• Vaqueiro na planície aberta: o pasto.",
        "• Ferreiro perto da praça e da estrada",
        "  que sobe para a mina do avô.",
        "• Lavradora ao lado da roça, perto da casa.",
        "• Vendinha na praça: onde o povo passa.",
        "• Oficina no terraço da margem: água",
        "  para a roda d'água e chão plano.",
        "",
        "Raio de 600 blocos sem ravinas e sem",
        "bocas de caverna; o subsolo segue o",
        "do Minecraft abaixo de y = 20.",
    ]
    for i, line in enumerate(notes):
        d.text((x0, y), line, fill=(255, 255, 255) if i == 0 else (200, 205, 215), font=font(17 if i == 0 else 14)); y += 22 if i == 0 else 19
    img.save(out_dir / "vila.png")


if __name__ == "__main__":
    main()
