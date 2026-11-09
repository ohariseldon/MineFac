"""Máquinas grandes: o bloco da máquina (núcleo) e a estrutura que ele monta em volta.

Cada máquina é desenhada como **um modelo só**, em pixels, sobre a planta inteira (tanques
redondos, torres treliçadas, passarelas, canos e telhados contínuos). O gerador corta esse
modelo em blocos: cada célula da planta vira um estado de `veio:structure` com o seu pedaço
(`piece = <máquina>_<n>`). As faces que só existem por causa do corte são retiradas, então as
peças se juntam sem costura.

O núcleo fica na fileira da frente, com a frente livre (painel, luz de estado, entrada). As
escotilhas (`veio:hatch_in` e `veio:hatch_out`) são blocos à parte, com inventário: a de saída
fica na fileira de trás, com a boca para fora.

Coordenadas do desenho: x da esquerda para a direita de quem olha a frente da máquina, y para
cima, z de trás (0) para a frente (16 x profundidade). Uma célula tem 16 px.

A mesma descrição gera os modelos, as colisões e `native/api/src/frames/data.rs`; não editar os
arquivos gerados à mão. Prévia: `python3 tools/preview_machine_frames.py`.
"""
from math import cos, sin, pi
from pathlib import Path

STRUCTURE, HATCH_IN, HATCH_OUT = "structure", "hatch_in", "hatch_out"
TIERS = ("copper", "iron", "steel")
U = 16

TIER_TEXTURES = {
    "copper": {"body": "minecraft:block/cut_copper", "trim": "minecraft:block/polished_deepslate",
               "deck": "minecraft:block/smooth_stone", "shell": "minecraft:block/copper_block"},
    "iron": {"body": "minecraft:block/iron_block", "trim": "minecraft:block/polished_deepslate",
             "deck": "minecraft:block/polished_andesite", "shell": "minecraft:block/light_gray_concrete"},
    "steel": {"body": "minecraft:block/polished_deepslate", "trim": "minecraft:block/iron_block",
              "deck": "minecraft:block/deepslate_tiles", "shell": "minecraft:block/gray_concrete"},
}
COMMON_TEXTURES = {
    "base": "minecraft:block/smooth_stone", "base_side": "minecraft:block/smooth_stone_slab_side",
    "metal": "minecraft:block/iron_block", "pipe": "minecraft:block/cut_copper",
    "joint": "minecraft:block/copper_block", "brick": "minecraft:block/bricks",
    "dark": "minecraft:block/black_concrete", "glow": "minecraft:block/blast_furnace_front_on",
    "wood": "minecraft:block/stripped_spruce_log", "plank": "minecraft:block/spruce_planks",
    "glass": "minecraft:block/tinted_glass", "screen": "minecraft:block/blue_terracotta",
    "warn": "minecraft:block/yellow_terracotta", "light": "minecraft:block/redstone_lamp_on",
    "port": "veio:block/port", "rust": "minecraft:block/weathered_cut_copper",
    "steelbar": "minecraft:block/polished_basalt_side", "roof": "minecraft:block/deepslate_tiles",
}


def textures(tier, particle="body"):
    values = {**COMMON_TEXTURES, **TIER_TEXTURES[tier]}
    values["particle"] = values[particle]
    return values


class Machine:
    """Uma máquina desenhada em pixels sobre a planta."""

    def __init__(self, name, tier, width, depth, core, hatches=()):
        self.name, self.tier, self.width, self.depth = name, tier, width, depth
        self.core = (core, depth - 1)  # coluna e fileira (de trás para a frente) do núcleo
        self.hatches = list(hatches)   # (coluna, fileira, "in"|"out")
        self.boxes = []                # (lo, hi, faces)

    # ---- primitivas ----
    def box(self, lo, hi, material="body", faces=None):
        lo, hi = list(lo), list(hi)
        if all(hi[i] - lo[i] > 1e-6 for i in range(3)):
            self.boxes.append((lo, hi, faces or "#" + material))

    def plinth(self):
        """Laje de pedra contínua sob a planta, com a faixa de aviso na borda da frente."""
        W, D = self.width * U, self.depth * U
        self.box([0, 0, 0], [W, 1.5, D], faces={"up": "#base", "down": "#base", "*": "#base_side"})
        for x in range(0, W, 4):
            self.box([x, 1.5, D - 1.2], [x + 2, 1.7, D - 0.2], "warn")
            self.box([x + 2, 1.5, D - 1.2], [x + 4, 1.7, D - 0.2], "dark")

    def round_v(self, cx, cz, r, y0, y1, material="body"):
        """Cilindro em pé feito de três caixas cruzadas (oito lados)."""
        k = 0.42
        self.box([cx - r, y0, cz - r * k], [cx + r, y1, cz + r * k], material)
        self.box([cx - r * k, y0, cz - r], [cx + r * k, y1, cz + r], material)
        self.box([cx - r * 0.78, y0, cz - r * 0.78], [cx + r * 0.78, y1, cz + r * 0.78], material)

    def round_h(self, axis, c1, cy, r, a0, a1, material="body"):
        """Cilindro deitado ao longo de x (`c1` é z) ou de z (`c1` é x)."""
        k = 0.42
        for (du, dv) in ((r, r * k), (r * k, r), (r * 0.78, r * 0.78)):
            if axis == "x":
                self.box([a0, cy - dv, c1 - du], [a1, cy + dv, c1 + du], material)
            else:
                self.box([c1 - du, cy - dv, a0], [c1 + du, cy + dv, a1], material)

    def tank(self, cx, cz, r, y0, y1, bands=2, material="shell", legs=False):
        top = y1
        self.round_v(cx, cz, r, y0, top, material)
        for i in range(bands):
            y = y0 + (top - y0) * (i + 1) / (bands + 1)
            self.round_v(cx, cz, r + 0.45, y, y + 1, "trim")
        self.round_v(cx, cz, r * 0.8, top, top + 1.5, material)
        self.round_v(cx, cz, r * 0.5, top + 1.5, top + 2.5, material)
        self.box([cx - 1, top + 2.5, cz - 1], [cx + 1, top + 4, cz + 1], "joint")  # válvula
        self.box([cx - r - 0.6, y0 + 2, cz - 0.6], [cx - r + 0.4, top - 1, cz + 0.6], "metal")  # visor
        if legs:
            for dx in (-r * 0.6, r * 0.6):
                for dz in (-r * 0.6, r * 0.6):
                    self.box([cx + dx - 1, 1.5, cz + dz - 1], [cx + dx + 1, y0, cz + dz + 1], "trim")

    def drum(self, axis, c1, cy, r, a0, a1, material="shell"):
        """Caldeira ou tanque deitado, com cintas e tampos."""
        self.round_h(axis, c1, cy, r, a0 + 1, a1 - 1, material)
        self.round_h(axis, c1, cy, r * 0.8, a0, a1, "trim")
        n = max(1, int((a1 - a0) // 10))
        for i in range(1, n + 1):
            a = a0 + (a1 - a0) * i / (n + 1)
            self.round_h(axis, c1, cy, r + 0.45, a - 0.5, a + 0.5, "trim")
        # berços
        for a in (a0 + 3, a1 - 5):
            if axis == "x":
                self.box([a, 1.5, c1 - r * 0.8], [a + 2, cy - r * 0.5, c1 + r * 0.8], "trim")
            else:
                self.box([c1 - r * 0.8, 1.5, a], [c1 + r * 0.8, cy - r * 0.5, a + 2], "trim")

    def chimney(self, cx, cz, s, y0, y1):
        h = s / 2
        self.box([cx - h - 1, y0, cz - h - 1], [cx + h + 1, y0 + 4, cz + h + 1], "brick")
        self.box([cx - h, y0, cz - h], [cx + h, y1, cz + h], "brick")
        y = y0 + 12
        while y < y1 - 4:
            self.box([cx - h - 0.4, y, cz - h - 0.4], [cx + h + 0.4, y + 1, cz + h + 0.4], "metal")
            y += 12
        self.box([cx - h - 1, y1, cz - h - 1], [cx + h + 1, y1 + 2, cz + h + 1], "brick")
        self.box([cx - h + 0.5, y1 + 1.95, cz - h + 0.5], [cx + h - 0.5, y1 + 2.05, cz + h - 0.5], "dark")

    def truss(self, x0, z0, x1, z1, y0, y1, material="body", step=12):
        """Torre treliçada: quatro pernas, anéis e contraventamento em X nas faces."""
        p = 2
        for x in (x0, x1 - p):
            for z in (z0, z1 - p):
                self.box([x, y0, z], [x + p, y1, z + p], material)
        y = y0
        while y <= y1 - 1:
            self.box([x0 + p, y, z0], [x1 - p, y + 1, z0 + 1], "trim")
            self.box([x0 + p, y, z1 - 1], [x1 - p, y + 1, z1], "trim")
            self.box([x0, y, z0 + p], [x0 + 1, y + 1, z1 - p], "trim")
            self.box([x1 - 1, y, z0 + p], [x1, y + 1, z1 - p], "trim")
            y += step
        # diagonais em degraus nas quatro faces
        y = y0
        while y + step <= y1 + 0.01:
            n = max(6, int(max(x1 - x0, z1 - z0) / 1.3))
            for i in range(n):
                t0 = i / n
                ya = y + step * t0
                xa = x0 + p + (x1 - x0 - 2 * p - 1.5) * t0
                xb = x1 - p - 1.5 - (x1 - x0 - 2 * p - 1.5) * t0
                za = z0 + p + (z1 - z0 - 2 * p - 1.5) * t0
                zb = z1 - p - 1.5 - (z1 - z0 - 2 * p - 1.5) * t0
                hh = step / n + 0.3
                self.box([xa, ya, z0 + 0.2], [xa + 1.5, ya + hh, z0 + 1], "trim")
                self.box([xb, ya, z1 - 1], [xb + 1.5, ya + hh, z1 - 0.2], "trim")
                self.box([x0 + 0.2, ya, za], [x0 + 1, ya + hh, za + 1.5], "trim")
                self.box([x1 - 1, ya, zb], [x1 - 0.2, ya + hh, zb + 1.5], "trim")
            y += step

    def crown(self, x0, z0, x1, z1, y):
        """Topo da sonda: plataforma, roldana e cabo descendo pelo meio."""
        cx, cz = (x0 + x1) / 2, (z0 + z1) / 2
        self.box([x0 - 1, y, z0 - 1], [x1 + 1, y + 2, z1 + 1], "trim")
        self.box([cx - 3, y + 2, cz - 1.5], [cx + 3, y + 5, cz + 1.5], "body")
        self.gear("z", cz, cx, y + 8, 4.5, "metal", thick=1.5)
        self.box([cx - 0.5, y + 12.5, cz - 0.5], [cx + 0.5, y + 15, cz + 0.5], "light")

    def deck(self, x0, z0, x1, z1, y, rails="nsew"):
        """Passarela: chapa, guarda-corpo com montantes, corrimão e rodapé."""
        self.box([x0, y, z0], [x1, y + 1.5, z1], faces={"up": "#deck", "*": "#trim"})
        top = y + 9
        sides = {"n": ((x0, z0), (x1, z0 + 1)), "s": ((x0, z1 - 1), (x1, z1)),
                 "w": ((x0, z0), (x0 + 1, z1)), "e": ((x1 - 1, z0), (x1, z1))}
        for side in rails:
            (ax, az), (bx, bz) = sides[side]
            self.box([ax, top, az], [bx, top + 1, bz], "warn")
            self.box([ax, y + 5, az], [bx, y + 5.7, bz], "trim")
            length = (bx - ax) if side in "ns" else (bz - az)
            n = max(1, int(length // 8))
            for i in range(n + 1):
                t = min(length - 1, i * length / n)
                if side in "ns":
                    self.box([ax + t, y + 1.5, az], [ax + t + 1, top, bz], "trim")
                else:
                    self.box([ax, y + 1.5, az + t], [bx, top, az + t + 1], "trim")

    def ladder(self, x, z, y0, y1, along="x"):
        w = 5
        if along == "x":
            self.box([x, y0, z], [x + 1, y1, z + 1], "trim")
            self.box([x + w, y0, z], [x + w + 1, y1, z + 1], "trim")
            y = y0 + 2
            while y < y1:
                self.box([x + 1, y, z + 0.2], [x + w, y + 0.6, z + 0.8], "metal")
                y += 3
        else:
            self.box([x, y0, z], [x + 1, y1, z + 1], "trim")
            self.box([x, y0, z + w], [x + 1, y1, z + w + 1], "trim")
            y = y0 + 2
            while y < y1:
                self.box([x + 0.2, y, z + 1], [x + 0.8, y + 0.6, z + w], "metal")
                y += 3

    def pipe(self, points, r=1.4, material="pipe"):
        """Cano por segmentos retos, com flanges nas pontas e juntas nas curvas."""
        for a, b in zip(points, points[1:]):
            lo = [min(a[i], b[i]) - r for i in range(3)]
            hi = [max(a[i], b[i]) + r for i in range(3)]
            self.box(lo, hi, material)
        for p in points:
            self.box([p[0] - r - 0.5, p[1] - r - 0.5, p[2] - r - 0.5], [p[0] + r + 0.5, p[1] + r + 0.5, p[2] + r + 0.5], "joint")

    def gear(self, plane, at, c1, cy, r, material="metal", thick=2.0):
        """Engrenagem ou volante: disco de oito lados, dentes e cubo. `plane` é o eixo do giro
        (x: disco no plano yz, com `at` = x e `c1` = z; z: disco no plano xy, `at` = z, `c1` = x)."""
        k = 0.42

        def flat(u0, u1, v0, v1, t0, t1, mat):
            if plane == "x":
                self.box([t0, v0, u0], [t1, v1, u1], mat)
            else:
                self.box([u0, v0, t0], [u1, v1, t1], mat)
        t0, t1 = at - thick / 2, at + thick / 2
        for du, dv in ((r, r * k), (r * k, r), (r * 0.78, r * 0.78)):
            flat(c1 - du, c1 + du, cy - dv, cy + dv, t0, t1, material)
        for i in range(8):
            a = i * pi / 4
            u, v = c1 + cos(a) * (r + 0.6), cy + sin(a) * (r + 0.6)
            flat(u - 1, u + 1, v - 1, v + 1, t0 + 0.2, t1 - 0.2, material)
        flat(c1 - 1.5, c1 + 1.5, cy - 1.5, cy + 1.5, t0 - 0.6, t1 + 0.6, "joint")

    def cabinet(self, x0, z0, x1, z1, y0, y1, material="body", face="s", screen=True, vents=True):
        """Gabinete: corpo, cantoneiras, costura, grelha, tela e luzes na face indicada."""
        self.box([x0 + 0.5, y0, z0 + 0.5], [x1 - 0.5, y1 - 1, z1 - 0.5], material)
        self.box([x0, y1 - 1, z0], [x1, y1, z1], "trim")
        for x in (x0, x1 - 1):
            for z in (z0, z1 - 1):
                self.box([x, y0, z], [x + 1, y1 - 1, z + 1], "trim")
        w = (x1 - x0) if face in "ns" else (z1 - z0)
        mid = (y0 + y1) / 2

        def on(u0, u1, v0, v1, mat, depth=0.4):
            if face == "s":
                self.box([x0 + u0, v0, z1 - 0.5], [x0 + u1, v1, z1 - 0.5 + depth], mat)
            elif face == "n":
                self.box([x0 + u0, v0, z0 + 0.5 - depth], [x0 + u1, v1, z0 + 0.5], mat)
            elif face == "e":
                self.box([x1 - 0.5, v0, z0 + u0], [x1 - 0.5 + depth, v1, z0 + u1], mat)
            else:
                self.box([x0 + 0.5 - depth, v0, z0 + u0], [x0 + 0.5, v1, z0 + u1], mat)
        if screen and y1 - y0 >= 8:
            on(2, min(w - 2, 10), mid, min(y1 - 3, mid + 5), "screen")
            on(1.5, min(w - 1.5, 10.5), mid - 0.6, mid, "trim")
        if vents and y1 - y0 >= 6:
            v = y0 + 2
            while v < mid - 1.5:
                on(w - 7 if w > 9 else 2, w - 2, v, v + 0.8, "dark")
                v += 1.8
        if y1 - y0 >= 6:
            on(w - 3, w - 2, y1 - 3, y1 - 2, "light", 0.5)

    def conveyor(self, x0, z0, x1, z1, y, axis="z"):
        """Esteira de roletes entre trilhos."""
        self.box([x0, 1.5, z0], [x1, y - 1.5, z1], "trim")
        if axis == "z":
            self.box([x0, y - 1.5, z0], [x0 + 1.5, y + 1.5, z1], "body")
            self.box([x1 - 1.5, y - 1.5, z0], [x1, y + 1.5, z1], "body")
            z = z0 + 1
            while z < z1 - 1:
                self.box([x0 + 1.5, y - 0.5, z], [x1 - 1.5, y + 0.5, z + 1.6], "metal")
                z += 3
        else:
            self.box([x0, y - 1.5, z0], [x1, y + 1.5, z0 + 1.5], "body")
            self.box([x0, y - 1.5, z1 - 1.5], [x1, y + 1.5, z1], "body")
            x = x0 + 1
            while x < x1 - 1:
                self.box([x, y - 0.5, z0 + 1.5], [x + 1.6, y + 0.5, z1 - 1.5], "metal")
                x += 3

    def hopper(self, x0, z0, x1, z1, y0, h=12):
        """Funil: bocal estreito embaixo, abrindo em degraus para cima."""
        steps = 4
        for i in range(steps):
            inset = (steps - 1 - i) * 1.8
            ya, yb = y0 + h * i / steps, y0 + h * (i + 1) / steps
            a0, b0, a1, b1 = x0 + inset, z0 + inset, x1 - inset, z1 - inset
            self.box([a0, ya, b0], [a1, yb, b0 + 1.5], "body")
            self.box([a0, ya, b1 - 1.5], [a1, yb, b1], "body")
            self.box([a0, ya, b0 + 1.5], [a0 + 1.5, yb, b1 - 1.5], "body")
            self.box([a1 - 1.5, ya, b0 + 1.5], [a1, yb, b1 - 1.5], "body")
        self.box([x0 - 0.5, y0 + h, z0 - 0.5], [x1 + 0.5, y0 + h + 1, z1 + 0.5], "trim")
        self.box([x0 + 6, y0, z0 + 6], [x1 - 6, y0 + 0.2, z1 - 6], "dark")

    def stripes(self, x0, x1, y, z, h=1.6):
        """Faixa amarela e preta na face sul em z."""
        x = x0
        while x < x1:
            self.box([x, y, z], [min(x + 2, x1), y + h, z + 0.3], "warn")
            self.box([min(x + 2, x1), y, z], [min(x + 4, x1), y + h, z + 0.3], "dark")
            x += 4

    def sawtooth(self, x0, z0, x1, z1, y):
        """Telhado de fábrica em dentes de serra, com vidraças voltadas para trás."""
        self.box([x0, y, z0], [x1, y + 1.5, z1], faces={"up": "#roof", "*": "#trim"})
        z = z0
        while z + 12 <= z1:
            for i in range(4):
                self.box([x0, y + 1.5 + i * 1.6, z + i * 3], [x1, y + 3.1 + i * 1.6, z + 12], faces={"up": "#roof", "*": "#trim"})
            self.box([x0 + 1, y + 1.5, z], [x1 - 1, y + 7.5, z + 0.6], "glass")
            z += 12

    # ---- corte em células ----
    def blocked(self):
        cells = {self.core}
        cells.update((i, j) for i, j, _ in self.hatches)
        return cells

    def slice(self):
        """{(i, y, j): [elementos locais]} e a altura da planta."""
        out = {}
        reserved = self.blocked()
        for lo, hi, faces in self.boxes:
            lo = [max(lo[0], 0), max(lo[1], 0), max(lo[2], 0)]
            hi = [min(hi[0], self.width * U), hi[1], min(hi[2], self.depth * U)]
            if any(hi[k] - lo[k] <= 1e-6 for k in range(3)):
                continue
            for i in range(int(lo[0] // U), int((hi[0] - 1e-6) // U) + 1):
                for y in range(int(lo[1] // U), int((hi[1] - 1e-6) // U) + 1):
                    for j in range(int(lo[2] // U), int((hi[2] - 1e-6) // U) + 1):
                        if y == 0 and (i, j) in reserved:
                            continue
                        base = [i * U, y * U, j * U]
                        a = [max(lo[k], base[k]) - base[k] for k in range(3)]
                        b = [min(hi[k], base[k] + U) - base[k] for k in range(3)]
                        if any(b[k] - a[k] <= 1e-6 for k in range(3)):
                            continue
                        # Faces criadas pelo corte não existem no modelo inteiro.
                        cut = {"west": lo[0] < base[0] - 1e-6 and a[0] < 1e-6,
                               "east": hi[0] > base[0] + U + 1e-6 and b[0] > U - 1e-6,
                               "down": lo[1] < base[1] - 1e-6 and a[1] < 1e-6,
                               "up": hi[1] > base[1] + U + 1e-6 and b[1] > U - 1e-6,
                               "north": lo[2] < base[2] - 1e-6 and a[2] < 1e-6,
                               "south": hi[2] > base[2] + U + 1e-6 and b[2] > U - 1e-6}
                        out.setdefault((i, y, j), []).append(
                            ([round(v, 4) for v in a], [round(v, 4) for v in b], faces, cut))
        height = max(y for _, y, _ in out) + 1 if out else 1
        return out, height


def element(lo, hi, faces, cut, box):
    e = box(lo, hi, faces)
    e["faces"] = {k: v for k, v in e["faces"].items() if not cut.get(k)}
    return e


# --------------------------------------------------------------------------------------
# As máquinas. Larguras e fundos em blocos; núcleo na coluna indicada da fileira da frente.
# --------------------------------------------------------------------------------------

def drill():  # 2 x 2: sonda leve sobre plataforma
    m = Machine("drill", "copper", 2, 2, 0, [(0, 0, "out")])
    m.plinth()
    m.cabinet(17, 1, 31, 15, 1.5, 14, face="e")                      # motor
    m.gear("x", 31.6, 8, 8, 4.5, thick=1.2)
    m.tank(24, 24, 5.5, 1.5, 12, bands=2)
    m.deck(0, 0, 32, 32, 16, rails="sew")
    m.ladder(26, 30.6, 1.5, 17.5)
    m.truss(2, 2, 14, 14, 17.5, 41, "body", step=8)
    m.crown(2, 2, 14, 14, 41)
    m.box([7.5, 1.5, 7.5], [8.5, 41, 8.5], "metal")                  # haste da sonda
    m.pipe([(16, 10, 8), (14.5, 10, 8)])
    return m


def drill_mk2():  # 3 x 3: torre de aço com casa de máquinas
    m = Machine("drill_mk2", "steel", 3, 3, 1, [(1, 0, "out")])
    m.plinth()
    m.truss(8, 2, 40, 34, 1.5, 58, "body", step=14)
    m.crown(14, 8, 34, 28, 58)
    m.box([23.5, 1.5, 17.5], [24.5, 58, 18.5], "metal")
    for y in (12, 26, 40):
        m.round_v(24, 18, 2.5, y, y + 2, "joint")                     # luvas da haste
    m.tank(7, 22, 5.5, 1.5, 26, bands=3)
    m.cabinet(37, 4, 47, 28, 1.5, 20, face="e")
    m.gear("x", 47.4, 16, 10, 6, thick=1.2)
    m.pipe([(40, 14, 16), (34, 14, 16), (34, 30, 16)])
    m.pipe([(7, 27.5, 22), (7, 32, 22), (10, 32, 22)], r=1.2)
    m.deck(0, 32, 16, 48, 14, rails="sw")
    m.deck(32, 32, 48, 48, 14, rails="se")
    m.stripes(8, 40, 3, 34.2)
    return m


def drill_mk3():  # 4 x 4: grande sonda com dois tanques e radiadores
    m = Machine("drill_mk3", "steel", 4, 4, 1, [(2, 0, "out")])
    m.plinth()
    m.truss(12, 4, 52, 44, 1.5, 76, "body", step=16)
    m.crown(18, 10, 46, 38, 76)
    m.box([31.5, 1.5, 23.5], [32.5, 76, 24.5], "metal")
    for y in (14, 30, 46, 62):
        m.round_v(32, 24, 3, y, y + 2.5, "joint")
    m.deck(12, 4, 52, 44, 30, rails="nsew")
    m.ladder(52.2, 20, 1.5, 31.5, along="z")
    m.tank(6, 14, 5.5, 1.5, 34, bands=3)
    m.tank(58, 14, 5.5, 1.5, 34, bands=3)
    for x0 in (2, 50):
        for x in range(int(x0), int(x0) + 12, 2):
            m.box([x, 2, 50], [x + 1, 14, 62], "trim")               # aletas do radiador
        m.box([x0, 14, 49], [x0 + 12, 15.5, 63], "body")
    m.pipe([(6, 36, 14), (6, 40, 14), (14, 40, 14)])
    m.pipe([(58, 36, 14), (58, 40, 14), (50, 40, 14)])
    m.stripes(12, 52, 3, 44.2)
    return m


def press():  # 2 x 3: prensa hidráulica com unidade de força atrás
    m = Machine("press", "copper", 2, 3, 0, [(1, 0, "out")])
    m.plinth()
    for cx in (4, 28):
        m.round_v(cx, 24, 3.5, 1.5, 46, "metal")
        m.round_v(cx, 24, 4.3, 1.5, 5, "trim")
        m.round_v(cx, 24, 4.3, 36, 38, "trim")
    m.cabinet(7, 17, 25, 31, 1.5, 12, "trim", face="s", screen=False)  # mesa
    m.box([8, 12, 18], [24, 13, 30], "dark")
    m.box([7, 40, 17], [25, 48, 31], "body")                           # cabeçote
    m.box([0, 46, 18], [32, 50, 30], "trim")
    m.round_v(16, 24, 3, 22, 40, "metal")                              # êmbolo
    m.box([8, 20, 18], [24, 24, 30], "trim")                           # martelo
    m.stripes(8, 24, 21, 30.2)
    m.round_v(16, 24, 4.5, 50, 58, "joint")
    m.drum("x", 8, 7, 5.5, 1, 15)                                      # unidade de força
    m.pipe([(15, 10, 8), (16.5, 10, 8)], r=1.2)
    m.pipe([(8, 13, 8), (8, 54, 8), (12, 54, 8), (12, 54, 22)], r=1.2)
    m.cabinet(17, 33, 31, 47, 1.5, 15, face="s")                       # console ao lado do núcleo
    return m


def assembler():  # 3 x 3: célula de montagem com pórtico e braços
    m = Machine("assembler", "copper", 3, 3, 1, [(1, 0, "out")])
    m.plinth()
    for x in (1, 45):
        for z in (1, 45):
            m.box([x, 1.5, z], [x + 2, 34, z + 2], "trim")
    m.box([1, 32, 1], [47, 34, 3], "body")
    m.box([1, 32, 45], [47, 34, 47], "body")
    m.box([1, 32, 3], [3, 34, 45], "trim")
    m.box([45, 32, 3], [47, 34, 45], "trim")
    m.box([20, 30, 3], [28, 34, 45], "body")                           # ponte rolante
    m.box([21, 24, 20], [27, 30, 28], "trim")                          # carro
    m.box([23.5, 14, 23.5], [24.5, 24, 24.5], "metal")
    m.box([21, 12, 21], [27, 14, 27], "joint")
    m.stripes(20, 28, 30.5, 45)
    m.conveyor(2, 18, 46, 30, 9, axis="x")
    for cx in (8, 40):                                                 # braços robóticos
        m.round_v(cx, 39, 4, 1.5, 6, "trim")
        m.round_v(cx, 39, 2.5, 6, 14, "body")
        m.box([cx - 1.5, 13, 30], [cx + 1.5, 16, 41], "body")
        m.box([cx - 1, 9, 29], [cx + 1, 16, 32], "joint")
    m.cabinet(2, 2, 14, 14, 1.5, 20, face="s")
    m.cabinet(34, 2, 46, 14, 1.5, 20, face="s")
    return m


def cutter():  # 2 x 4: mesa de roletes comprida e serra circular com capô
    m = Machine("cutter", "iron", 2, 4, 0, [(0, 0, "out")])
    m.plinth()
    m.conveyor(3, 2, 29, 62, 9, axis="z")
    m.box([2, 11, 22], [30, 13, 42], "trim")                           # capô da serra
    m.box([2, 13, 24], [30, 22, 40], "body")
    m.box([6, 22, 26], [26, 24, 38], "body")
    m.gear("x", 16, 32, 13, 7.5, "metal", thick=1)
    m.stripes(2, 30, 15, 40.1)
    m.pipe([(16, 24, 32), (16, 30, 32), (16, 30, 14), (24, 30, 14)], r=1.6, material="metal")
    m.cabinet(17, 49, 31, 63, 1.5, 16, face="e")                       # motor ao lado do núcleo
    m.gear("x", 31.4, 56, 9, 4, thick=1)
    return m


def integrator():  # 3 x 4: galpão com telhado em dente de serra
    m = Machine("integrator", "iron", 3, 4, 1, [(1, 0, "out"), (0, 2, "in")])
    m.plinth()
    for x in (0, 46):
        for z in (0, 16, 32, 48, 62):
            m.box([x, 1.5, z], [x + 2, 32, min(z + 2, 64)], "trim")
    for x0, x1 in ((0, 2), (46, 48)):
        m.box([x0, 22, 0], [x1, 30, 64], "body")
        m.box([x0 + 0.2, 24, 2], [x1 - 0.2, 28, 62], "glass")
    m.box([2, 22, 0], [46, 30, 2], "body")
    m.sawtooth(0, 0, 48, 64, 32)
    m.conveyor(18, 4, 30, 44, 9, axis="z")
    m.cabinet(2, 2, 16, 14, 1.5, 22, face="s")
    m.cabinet(32, 2, 46, 14, 1.5, 22, face="s")
    m.cabinet(34, 34, 46, 46, 1.5, 15, face="w")
    m.tank(40, 25, 4.5, 1.5, 14, bands=1)
    m.box([46.5, 40, 6], [47.5, 52, 7], "metal")                       # antena
    m.box([43, 50, 6], [48, 51, 7], "metal")
    m.stripes(2, 46, 3, 62.5)
    return m


def crusher():  # 2 x 2: corpo alto com funil e volante
    m = Machine("crusher", "iron", 2, 2, 0, [(0, 0, "out")])
    m.plinth()
    m.cabinet(17, 1, 31, 31, 1.5, 26, face="e", screen=False)
    m.box([1, 16, 1], [31, 26, 31], "body")
    m.box([0, 26, 0], [32, 28, 32], "trim")
    m.hopper(2, 2, 30, 30, 28, 14)
    m.gear("x", 31.4, 16, 12, 8, "metal", thick=1.5)
    m.box([31, 11.5, 15.5], [32, 12.5, 16.5], "joint")
    m.stripes(0, 32, 26.2, 31.9)
    m.ladder(1, 31, 16, 41)
    return m


def alloy_furnace():  # 3 x 3: alto-forno de tijolo com chaminé e cadinhos
    m = Machine("alloy_furnace", "iron", 3, 3, 1, [(1, 0, "out")])
    m.plinth()
    m.box([2, 1.5, 2], [46, 6, 32], "trim")
    m.box([4, 6, 4], [44, 28, 30], "brick")
    m.box([8, 28, 7], [40, 38, 27], "brick")
    m.box([12, 38, 10], [36, 44, 24], "brick")
    for y in (12, 22, 34):
        m.box([3.5, y, 3.5], [44.5, y + 1, 30.5], "metal")
    for x in (10, 34):                                                 # bocas da fornalha
        m.box([x - 4, 8, 30], [x + 4, 16, 30.6], "glow")
        m.box([x - 5, 16, 30], [x + 5, 17.5, 31], "trim")
    m.chimney(8, 8, 8, 44, 66)
    m.pipe([(36, 30, 17), (46, 30, 17), (46, 30, 40), (40, 30, 40)], r=1.8, material="metal")
    m.tank(8, 40, 5, 1.5, 12, bands=1, material="rust")              # cadinho
    m.tank(40, 40, 5, 1.5, 12, bands=1, material="rust")
    m.stripes(4, 44, 3, 30.2)
    return m


def electric_furnace():  # 2 x 2: forno a arco com três eletrodos e transformador
    m = Machine("electric_furnace", "copper", 2, 2, 0, [(0, 0, "out")])
    m.plinth()
    m.tank(24, 10, 7, 1.5, 15, bands=1, material="shell")
    m.round_v(24, 10, 7.5, 15, 17, "trim")
    for i, (dx, dz) in enumerate(((-3, -2), (3, -2), (0, 3))):
        m.round_v(24 + dx, 10 + dz, 1.2, 17, 34, "metal")
        m.box([24 + dx - 2, 30, 10 + dz - 2], [24 + dx + 2, 32, 10 + dz + 2], "joint")
    m.box([14, 34, 4], [30, 36, 16], "trim")
    m.cabinet(17, 18, 31, 31, 1.5, 15, face="e", screen=False)
    for x in range(18, 31, 2):
        m.box([x, 15, 19], [x + 1, 22, 30], "trim")                    # radiador
    m.pipe([(20, 22, 24), (20, 34, 24), (20, 34, 16)], r=0.9, material="dark")
    m.box([1, 1.5, 17], [15, 3, 31], "trim")
    return m


def electronics():  # 3 x 2: sala de montagem fina com bastidores e antena
    m = Machine("electronics", "steel", 3, 2, 1, [(1, 0, "out")])
    m.plinth()
    for x0 in (2, 34):
        m.cabinet(x0, 2, x0 + 12, 14, 1.5, 28, face="s")
        for y in (6, 12, 18):
            m.box([x0 + 2, y, 13.6], [x0 + 10, y + 3, 14.2], "screen")
            m.box([x0 + 8, y + 1, 14.2], [x0 + 9, y + 2, 14.5], "light")
    m.cabinet(2, 18, 14, 30, 1.5, 12, face="s")
    m.cabinet(34, 18, 46, 30, 1.5, 12, face="s")
    m.box([0, 28, 0], [48, 30, 16], "trim")
    m.box([2, 30, 2], [12, 34, 12], "body")                            # ar-condicionado
    m.gear("z", 12.2, 7, 32, 3, "dark", thick=0.6)
    m.box([24, 30, 7], [25, 40, 8], "metal")
    m.round_v(24.5, 7.5, 4, 40, 41, "metal")                           # prato
    m.pipe([(40, 30, 8), (40, 34, 8), (30, 34, 8)], r=0.8, material="dark")
    return m


def mill():  # 2 x 2: moinho de madeira com roda d'água lateral e telhado
    m = Machine("mill", "copper", 2, 2, 0, [(0, 0, "out")])
    m.plinth()
    for x in (16, 23):
        for z in (1, 28):
            m.box([x, 1.5, z], [x + 3, 30, z + 3], "wood")
    m.box([17, 1.5, 2], [25, 28, 30], "plank")
    m.box([0, 16, 0], [26, 18, 32], "wood")
    m.box([1, 18, 1], [25, 28, 31], "plank")
    m.box([0, 28, 0], [26, 30, 32], "wood")
    for i in range(4):                                                  # telhado de duas águas
        m.box([0, 30 + i * 2.5, i * 3.5], [27, 32.5 + i * 2.5, 32 - i * 3.5], faces={"up": "#plank", "*": "#wood"})
    m.box([25, 14, 15], [32, 16, 17], "trim")                           # eixo
    m.gear("x", 29, 16, 15, 11.5, "wood", thick=1.5)                    # roda d'água
    for i in range(12):
        a = i * pi / 6
        y, z = 15 + sin(a) * 12, 16 + cos(a) * 12
        m.box([27.5, y - 1.6, z - 1.6], [31, y + 1.6, z + 1.6], "plank")  # pás
    m.box([26, 1.5, 4], [32, 3.5, 28], "dark")                          # canal
    m.hopper(4, 4, 14, 14, 18, 8)
    return m


def kitchen():  # 3 x 2: forno de tijolo, coifa e despensa de barris
    m = Machine("kitchen", "iron", 3, 2, 1, [(1, 0, "out")])
    m.plinth()
    m.box([2, 1.5, 2], [15, 14, 14], "brick")
    m.box([4, 3, 13.9], [13, 9, 14.3], "glow")
    m.chimney(8, 8, 6, 14, 34)
    for x in (34, 41):
        m.tank(x + 3, 8, 3.2, 1.5, 12, bands=1, material="plank")
    m.cabinet(0, 17, 15, 31, 1.5, 10, "trim", face="s", screen=False)
    m.cabinet(33, 17, 48, 31, 1.5, 10, "trim", face="s", screen=False)
    m.box([0, 22, 16], [48, 24, 32], "body")                           # coifa
    m.box([4, 24, 18], [44, 28, 30], "body")
    m.pipe([(24, 28, 24), (24, 32, 24), (24, 32, 8), (12, 32, 8)], r=1.6, material="metal")
    for x in (2, 46):
        m.box([x - 1 if x > 2 else x, 1.5, 30], [x + 1 if x > 2 else x + 2, 22, 32], "trim")
    return m


def fabricator():  # 4 x 4: fábrica do fim do jogo com ponte rolante e anexo
    m = Machine("fabricator", "steel", 4, 4, 1, [(2, 0, "out"), (0, 2, "in")])
    m.plinth()
    for x in (0, 62):
        for z in (0, 20, 40, 62):
            m.box([x, 1.5, z], [x + 2, 40, min(z + 2, 64)], "trim")
    for x0, x1 in ((0, 2), (62, 64)):
        m.box([x0, 1.5, 0], [x1, 24, 64], "body")
        m.box([x0 + 0.2, 26, 2], [x1 - 0.2, 34, 62], "glass")
        m.box([x0, 34, 0], [x1, 40, 64], "body")
    m.box([2, 1.5, 0], [62, 40, 2], "body")
    m.sawtooth(0, 0, 64, 64, 40)
    m.box([2, 34, 28], [62, 37, 32], "body")                           # ponte rolante
    m.box([28, 28, 26], [36, 34, 34], "trim")
    m.box([31.5, 16, 29.5], [32.5, 28, 30.5], "metal")
    m.stripes(2, 62, 34.5, 32)
    m.conveyor(20, 6, 44, 18, 9, axis="x")
    m.conveyor(36, 18, 46, 52, 9, axis="z")
    m.tank(12, 28, 6, 1.5, 22, bands=2)
    m.cabinet(48, 20, 62, 52, 1.5, 22, face="w")
    m.cabinet(48, 52, 62, 62, 1.5, 16, face="s")
    for x in (14, 54):
        m.box([x, 48, 6], [x + 1, 62, 7], "metal")
        m.box([x - 3, 58, 6], [x + 4, 59, 7], "metal")
    m.stripes(2, 62, 3, 62.5)
    return m


def generator():  # 2 x 2: caldeira deitada, fornalha e chaminé
    m = Machine("generator", "copper", 2, 2, 0)
    m.plinth()
    m.box([17, 1.5, 2], [31, 8, 30], "brick")
    m.box([20, 2.5, 29.9], [28, 7, 30.3], "glow")
    m.drum("z", 24, 15, 7, 2, 30)
    m.chimney(8, 8, 7, 1.5, 44)
    m.pipe([(24, 23, 10), (24, 26, 10), (12, 26, 10)], r=1.4, material="metal")
    m.pipe([(24, 23, 22), (24, 28, 22), (24, 28, 18)], r=1.0)
    m.box([1, 1.5, 17], [15, 3.5, 31], "trim")
    return m


def biomass_generator():  # 2 x 3: funil de biomassa, fornalha, caldeira e chaminé
    m = Machine("biomass_generator", "copper", 2, 3, 0)
    m.plinth()
    m.chimney(8, 8, 7, 1.5, 48)
    m.box([1, 1.5, 17], [15, 14, 31], "brick")
    m.box([4, 3, 30.9], [12, 9, 31.3], "glow")
    m.drum("z", 24, 14, 7, 2, 46)
    m.box([17, 1.5, 33], [31, 6, 47], "trim")
    m.hopper(2, 18, 14, 30, 14, 10)
    m.pipe([(24, 22, 10), (24, 26, 10), (12, 26, 10)], r=1.4, material="metal")
    m.tank(8, 24, 3, 24, 32, bands=1)
    return m


def steam_engine():  # 3 x 5: caldeira, cilindro, biela e grande volante
    m = Machine("steam_engine", "steel", 3, 5, 1)
    m.plinth()
    m.drum("z", 9, 12, 8, 2, 58)
    m.box([1, 1.5, 4], [17, 4, 56], "brick")
    m.box([4, 2, 55.9], [14, 3.8, 56.3], "glow")
    m.chimney(9, 6, 7, 20, 52)
    m.round_h("z", 30, 10, 4, 18, 34, "body")                          # cilindro de vapor
    m.round_h("z", 30, 10, 4.6, 17, 19, "trim")
    m.round_h("z", 30, 10, 4.6, 33, 35, "trim")
    m.box([29, 9, 34], [31, 11, 46], "metal")                          # biela
    m.gear("x", 40, 50, 22, 18, "trim", thick=2.5)                     # volante
    for a in (0, pi / 4, pi / 2, 3 * pi / 4):
        for t in range(-16, 17, 2):
            m.box([41.2, 22 + sin(a) * t - 0.8, 50 + cos(a) * t - 0.8], [41.8, 22 + sin(a) * t + 0.8, 50 + cos(a) * t + 0.8], "metal")
    m.box([31, 21, 49], [48, 23, 51], "joint")                         # eixo
    m.box([44, 1.5, 44], [48, 22, 56], "trim")
    m.box([24, 1.5, 44], [28, 22, 56], "trim")
    m.pipe([(9, 21, 30), (9, 24, 30), (30, 24, 30), (30, 14.5, 30)], r=1.6, material="metal")
    m.tank(40, 10, 5, 1.5, 18, bands=2)
    m.stripes(20, 48, 3, 79.5)
    return m


def geothermal():  # 3 x 3: poço, trocadores e torre de resfriamento
    m = Machine("geothermal", "steel", 3, 3, 1)
    m.plinth()
    for r, y0, y1 in ((12, 1.5, 10), (10, 10, 20), (9, 20, 28), (10, 28, 34)):
        m.round_v(24, 14, r, y0, y1, "shell")                          # torre de resfriamento
    m.round_v(24, 14, 10.5, 33, 34.5, "trim")
    m.box([18, 34.4, 8], [30, 34.6, 20], "dark")
    m.tank(6, 38, 4.5, 1.5, 20, bands=2, material="rust")
    m.tank(42, 38, 4.5, 1.5, 20, bands=2, material="rust")
    for x0 in (36,):
        for x in range(x0, x0 + 10, 2):
            m.box([x, 2, 2], [x + 1, 14, 12], "trim")
    m.pipe([(6, 22, 38), (6, 26, 38), (14, 26, 38), (14, 26, 26)], r=1.4, material="metal")
    m.pipe([(42, 22, 38), (42, 26, 38), (34, 26, 38), (34, 26, 26)], r=1.4, material="metal")
    m.round_v(24, 40, 3, 1.5, 4, "joint")
    return m


def research_lab():  # 3 x 3: laboratório com cúpula, bancadas e antena
    m = Machine("research_lab", "iron", 3, 3, 1)
    m.plinth()
    m.box([1, 1.5, 1], [47, 18, 31], "shell")
    m.box([0, 18, 0], [48, 20, 32], "trim")
    for x in range(4, 44, 8):
        m.box([x, 8, 30.9], [x + 5, 15, 31.3], "glass")
    for r, y0, y1 in ((14, 20, 24), (12, 24, 28), (9, 28, 31), (5, 31, 33)):
        m.round_v(24, 16, r, y0, y1, "body")                           # cúpula
    m.box([23, 22, 1], [25, 34, 3], "dark")                            # fenda do observatório
    m.box([38, 20, 4], [39, 36, 5], "metal")
    m.round_v(38.5, 4.5, 4, 36, 37, "metal")
    m.tank(6, 40, 4, 1.5, 14, bands=1, material="glass")
    m.tank(42, 40, 4, 1.5, 14, bands=1, material="glass")
    m.stripes(2, 46, 3, 31.4)
    return m


def manufacturer():  # 3 x 3: combinadora com quatro tremonhas sobre a mesa e saída de sobra
    m = Machine("manufacturer", "steel", 3, 3, 1, [(1, 0, "out"), (0, 1, "in"), (2, 1, "in")])
    m.plinth()
    for x in (0, 46):
        for z in (0, 23, 46):
            m.box([x, 1.5, z], [x + 2, 26, z + 2], "trim")
    m.box([2, 1.5, 2], [46, 12, 46], "body")                           # mesa de mistura
    m.box([0, 12, 0], [48, 14, 48], "trim")
    for x0, z0 in ((6, 6), (28, 6), (6, 28), (28, 28)):                # uma tremonha por entrada
        m.hopper(x0, z0, x0 + 14, z0 + 14, 14, h=10)
    m.box([0, 26, 0], [48, 28, 48], "body")                            # pórtico
    m.box([22, 24, 2], [26, 26, 46], "trim")
    m.conveyor(18, 2, 30, 14, 9, axis="z")                             # saída
    m.cabinet(34, 2, 46, 10, 1.5, 12, face="s")                        # painel da receita
    m.tank(40, 40, 3.5, 14, 24, bands=1, material="rust")              # sobra (escória)
    m.stripes(2, 46, 3, 47.5)
    return m


MACHINES = [drill, drill_mk2, drill_mk3, press, assembler, cutter, integrator, crusher, alloy_furnace,
            electric_furnace, electronics, mill, kitchen, fabricator, manufacturer, generator, biomass_generator,
            steam_engine, geothermal, research_lab]
LAYOUTS = {}


def plan(name):
    """{tier, size [w, h, d], cells [([dx, dy, dz], tipo, peça)], pieces {peça: elementos}}."""
    if name in LAYOUTS:
        return LAYOUTS[name]
    raise KeyError(name)


def compile_all(box):
    for make in MACHINES:
        m = make()
        sliced, height = m.slice()
        cx, cz = m.core
        cells, pieces = [], {}
        for index, ((i, y, j), parts) in enumerate(sorted(sliced.items(), key=lambda kv: (kv[0][1], kv[0][2], kv[0][0]))):
            piece = f"{m.name}_{index}"
            pieces[piece] = [element(a, b, faces, cut, box) for a, b, faces, cut in parts]
            cells.append(([i - cx, y, j - cz], STRUCTURE, piece))
        for i, j, kind in m.hatches:
            cells.append(([i - cx, 0, j - cz], HATCH_OUT if kind == "out" else HATCH_IN, None))
        LAYOUTS[m.name] = {"tier": m.tier, "size": [m.width, height, m.depth], "cells": cells, "pieces": pieces}
    return LAYOUTS


def hatch_elements(kind, box):
    """Escotilha: gabinete com a boca para trás (saída) ou para os lados (entrada)."""
    parts = [box([0, 0, 0], [16, 1.5, 16], {"up": "#base", "down": "#base", "*": "#base_side"}),
             box([0.5, 1.5, 0.5], [15.5, 15, 15.5], "#body"), box([0, 15, 0], [16, 16, 16], "#trim")]
    for x in (0, 15):
        for z in (0, 15):
            parts.append(box([x, 1.5, z], [x + 1, 15, z + 1], "#trim"))
    if kind == HATCH_OUT:
        parts.append(box([3, 3, 0], [13, 13, 0.5], "#warn"))
        parts.append(box([4, 4, -0.0], [12, 12, 0.6], {"north": "#port", "*": "#trim"}))
    else:
        for x0, x1, face in ((0, 0.6, "west"), (15.4, 16, "east")):
            parts.append(box([x0, 3, 3], [x1, 13, 13], "#screen"))
            parts.append(box([x0 - 0.01 if x0 == 0 else x0, 4, 4], [x1, 12, 12], {face: "#port", "*": "#trim"}))
    for p in parts:
        for key in ("from", "to"):
            p[key] = [min(16, max(0, v)) for v in p[key]]
    return parts


def hits(elements):
    """Colisão da peça: a caixa que envolve o que há nela (mínimo de meio pixel)."""
    lo = [min(e["from"][k] for e in elements) for k in range(3)]
    hi = [max(e["to"][k] for e in elements) for k in range(3)]
    for k in range(3):
        if hi[k] - lo[k] < 0.5:
            hi[k] = min(16, lo[k] + 0.5)
            lo[k] = hi[k] - 0.5
    return [[round(v / 16, 5) for v in (*lo, *hi)]]


def build(root, box, write):
    """Modelos, blockstates e registros das peças e escotilhas."""
    compile_all(box)
    assets, data = Path(root) / "assets/veio", Path(root) / "data/veio"
    folder = assets / "models/block/structure"
    if folder.exists():
        for old in folder.glob("*.json"):
            old.unlink()
    rotations = {"south": 0, "west": 90, "north": 180, "east": 270}
    variants, shapes, names = {}, {}, []
    for name, layout in LAYOUTS.items():
        tex = textures(layout["tier"])
        for piece, elements in layout["pieces"].items():
            names.append(piece)
            write(folder / f"{piece}.json", {"parent": "minecraft:block/block", "textures": tex, "elements": elements})
            for facing, y in rotations.items():
                variant = {"model": f"veio:block/structure/{piece}"}
                if y: variant["y"] = y
                variants[f"facing={facing},piece={piece}"] = variant
                shapes[f"{facing},{piece}"] = rotate_shape(hits(elements), facing)
    write(assets / "blockstates/structure.json", {"variants": variants})
    shape = {"properties": ["facing", "piece"], "values": shapes, "default": [[0, 0, 0, 1, 1, 1]]}
    write(data / "horizon/block/structure.json", {"template": "minecraft:black_glazed_terracotta", "item": False,
        "properties": {"hardness": 2.0, "occludes": False},
        "state_properties": {"piece": {"type": "enum", "values": names}},
        "default_state": {"piece": names[0]},
        "shapes": shape, "collision_shapes": shape})
    for kind in (HATCH_IN, HATCH_OUT):
        hatch = {}
        for tier in TIERS:
            write(folder / f"{kind}_{tier}.json", {"parent": "minecraft:block/block",
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


def rotate_shape(boxes, facing):
    """Gira as caixas (em blocos) como o `y` do blockstate gira o modelo."""
    out = []
    for x0, y0, z0, x1, y1, z1 in boxes:
        def orient(x, z):
            return {"south": (x, z), "west": (1 - z, x), "north": (1 - x, 1 - z), "east": (z, 1 - x)}[facing]
        corners = [orient(x, z) for x in (x0, x1) for z in (z0, z1)]
        out.append([round(min(p[0] for p in corners), 5), y0, round(min(p[1] for p in corners), 5),
                    round(max(p[0] for p in corners), 5), y1, round(max(p[1] for p in corners), 5)])
    return out


def native_layouts(root):
    """`native/api/src/frames/data.rs`: a planta de cada máquina para o módulo nativo."""
    lines = ["// Generated by tools/build_content.py from tools/machine_frames.py.\n",
             "use super::{Cell, Layout, Part};\n\n",
             "pub(super) fn layout(block: &str) -> Option<&'static Layout> {\n    Some(match block {\n"]
    for name, p in LAYOUTS.items():
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
