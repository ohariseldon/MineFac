"""Peças para montar os prédios da vila em blocos do Minecraft.

Um `Model` guarda blocos em coordenadas locais: x para leste, y para cima (0 é o nível do piso, que
substitui a camada de grama), z para o sul. As peças (paredes, telhado de duas águas, janela,
porta, cama, mesa...) escrevem estados de bloco válidos do Minecraft 1.21. Depois:

- `rotated(quartos)` gira o modelo de 90 em 90 graus (sentido horário visto de cima), girando
  também as propriedades `facing` e `axis` dos blocos;
- `connected()` liga cercas, muros e vidraças aos vizinhos (o comando `setblock` não faz isso);
- `commands()` devolve os comandos `fill`/`setblock` relativos (~x ~y ~z) para uma função do jogo.
"""
import re

DIRS = ["north", "east", "south", "west"]
DELTA = {"north": (0, -1), "south": (0, 1), "east": (1, 0), "west": (-1, 0)}

# Blocos que não ocupam o bloco inteiro: não servem de apoio para cerca/vidraça ligar
THIN = ("glass_pane", "_fence", "_wall", "iron_bars")
NON_SOLID = ("air", "torch", "lantern", "flower", "potted", "carpet", "candle", "rail", "button", "pressure_plate", "sign",
             "banner", "bed", "door", "trapdoor", "ladder", "vine", "short_grass", "tall_grass", "fern", "dandelion", "poppy",
             "tulip", "oxeye", "cornflower", "azure_bluet", "allium", "lily", "bush", "scaffolding", "cobweb", "slab", "stairs",
             "campfire", "chain", "bell", "pot", "cake", "flower_pot", "snow", "moss_carpet", "pink_petals", "sapling", "head")


def base_id(state):
    return state.split("[", 1)[0]


def props(state):
    if "[" not in state:
        return {}
    inner = state.split("[", 1)[1].rstrip("]")
    return dict(item.split("=") for item in inner.split(",") if item)


def with_props(block, values):
    if not values:
        return block
    return block + "[" + ",".join(f"{k}={v}" for k, v in values.items()) + "]"


def turn(direction, quarters):
    if direction not in DIRS:
        return direction
    return DIRS[(DIRS.index(direction) + quarters) % 4]


def rotate_state(state, quarters):
    """Gira as propriedades de direção de um estado de bloco."""
    quarters %= 4
    if quarters == 0 or "[" not in state:
        return state
    block, p = base_id(state), props(state)
    out = {}
    for key, value in p.items():
        if key == "facing":
            out[key] = turn(value, quarters)
        elif key == "axis" and value in ("x", "z") and quarters % 2:
            out[key] = "z" if value == "x" else "x"
        elif key in DIRS:
            out[turn(key, quarters)] = value
        elif key == "rotation":                                  # banners e placas em pé: 0..15
            out[key] = str((int(value) + 4 * quarters) % 16)
        else:
            out[key] = value
    return with_props(block, out)


def solid(state):
    b = base_id(state)
    return not any(word in b for word in NON_SOLID) and not any(b.endswith(word) for word in THIN) and not b.endswith("_fence_gate")


class Model:
    def __init__(self, name, width, depth, title=None):
        self.name, self.width, self.depth = name, width, depth
        self.title = title or name
        self.blocks = {}
        self.notes = []          # (x, y, z, texto): o que é cada coisa, para a prévia
        self.entrance = None     # (x, z, lado) da porta principal, depois da rotação
        self.core = (0, 0, width - 1, depth - 1)   # o retângulo das paredes (sem beiral nem quintal)

    # ---------- básico ----------
    def set(self, x, y, z, state):
        self.blocks[(x, y, z)] = state
        return self

    def get(self, x, y, z):
        return self.blocks.get((x, y, z))

    def clear(self, x, y, z):
        self.blocks.pop((x, y, z), None)

    def fill(self, x0, y0, z0, x1, y1, z1, state):
        for x in range(min(x0, x1), max(x0, x1) + 1):
            for y in range(min(y0, y1), max(y0, y1) + 1):
                for z in range(min(z0, z1), max(z0, z1) + 1):
                    self.blocks[(x, y, z)] = state
        return self

    def carve(self, x0, y0, z0, x1, y1, z1):
        """Esvazia (ar de verdade: o comando coloca ar)."""
        return self.fill(x0, y0, z0, x1, y1, z1, "minecraft:air")

    # ---------- estrutura ----------
    def floor(self, x0, z0, x1, z1, state, y=0):
        return self.fill(x0, y, z0, x1, y, z1, state)

    def walls(self, x0, z0, x1, z1, y0, y1, state, corner=None):
        for y in range(y0, y1 + 1):
            for x in range(x0, x1 + 1):
                self.set(x, y, z0, state); self.set(x, y, z1, state)
            for z in range(z0, z1 + 1):
                self.set(x0, y, z, state); self.set(x1, y, z, state)
        if corner:
            for x, z in ((x0, z0), (x1, z0), (x0, z1), (x1, z1)):
                self.fill(x, y0, z, x, y1, z, corner)
        return self

    def beams(self, x0, z0, x1, z1, y, state_x, state_z):
        """Vigas no alto da parede (enxaimel): toras deitadas."""
        for x in range(x0, x1 + 1):
            self.set(x, y, z0, state_x); self.set(x, y, z1, state_x)
        for z in range(z0 + 1, z1):
            self.set(x0, y, z, state_z); self.set(x1, y, z, state_z)
        return self

    def gable(self, x0, z0, x1, z1, y, wood, gable_fill, ridge="x", overhang=1, trim=None):
        """Telhado de duas águas sobre o retângulo de paredes (x0..x1, z0..z1), começando na altura y.
        `wood` é o nome da madeira (spruce, oak, dark_oak...) ou um bloco com escadas/lajes
        (`stone_brick`, `brick`, `mud_brick`...). A cumeeira corre ao longo de `ridge`."""
        stairs = f"minecraft:{wood}_stairs"
        slab = f"minecraft:{wood}_slab"
        if ridge == "x":
            lo, hi = z0 - overhang, z1 + overhang
            k = 0
            while lo + k < hi - k:
                for x in range(x0 - overhang, x1 + overhang + 1):
                    self.set(x, y + k, lo + k, f"{stairs}[facing=south,half=bottom,shape=straight]")
                    self.set(x, y + k, hi - k, f"{stairs}[facing=north,half=bottom,shape=straight]")
                for z in range(lo + k + 1, hi - k):          # empena: parede triangular
                    if z0 <= z <= z1:
                        self.set(x0, y + k, z, gable_fill); self.set(x1, y + k, z, gable_fill)
                k += 1
            if lo + k == hi - k:
                for x in range(x0 - overhang, x1 + overhang + 1):
                    self.set(x, y + k, lo + k, f"{slab}[type=bottom]")
            if trim:
                for x in (x0 - overhang, x1 + overhang):
                    for kk in range(k):
                        pass
        else:
            lo, hi = x0 - overhang, x1 + overhang
            k = 0
            while lo + k < hi - k:
                for z in range(z0 - overhang, z1 + overhang + 1):
                    self.set(lo + k, y + k, z, f"{stairs}[facing=east,half=bottom,shape=straight]")
                    self.set(hi - k, y + k, z, f"{stairs}[facing=west,half=bottom,shape=straight]")
                for x in range(lo + k + 1, hi - k):
                    if x0 <= x <= x1:
                        self.set(x, y + k, z0, gable_fill); self.set(x, y + k, z1, gable_fill)
                k += 1
            if lo + k == hi - k:
                for z in range(z0 - overhang, z1 + overhang + 1):
                    self.set(lo + k, y + k, z, f"{slab}[type=bottom]")
        return self

    def flat_roof(self, x0, z0, x1, z1, y, state, rim=None):
        self.fill(x0, y, z0, x1, y, z1, state)
        if rim:
            self.walls(x0, z0, x1, z1, y + 1, y + 1, rim)
        return self

    # ---------- aberturas ----------
    def door(self, x, z, facing, wood="oak", hinge="left", main=True):
        """Porta de duas metades; `facing` é para onde quem sai olha."""
        self.set(x, 1, z, f"minecraft:{wood}_door[facing={facing},half=lower,hinge={hinge},open=false]")
        self.set(x, 2, z, f"minecraft:{wood}_door[facing={facing},half=upper,hinge={hinge},open=false]")
        if main:
            self.entrance = (x, z, facing)
        return self

    def window(self, x, y, z, glass="minecraft:glass_pane", height=1):
        for dy in range(height):
            self.set(x, y + dy, z, glass)
        return self

    def shutters(self, x, y, z, side, wood="spruce"):
        """Venezianas abertas ao lado de uma janela na parede que dá para `side`."""
        dx, dz = DELTA[side]
        if dx == 0:
            for sx in (-1, 1):
                self.set(x + sx, y, z + dz, f"minecraft:{wood}_trapdoor[facing={side},half=bottom,open=true]")
        else:
            for sz in (-1, 1):
                self.set(x + dx, y, z + sz, f"minecraft:{wood}_trapdoor[facing={side},half=bottom,open=true]")
        return self

    # ---------- móveis ----------
    def bed(self, x, z, facing, color="red", y=1):
        """Cama: o pé em (x, z), a cabeceira um bloco na direção `facing`."""
        dx, dz = DELTA[facing]
        self.set(x, y, z, f"minecraft:{color}_bed[facing={facing},part=foot]")
        self.set(x + dx, y, z + dz, f"minecraft:{color}_bed[facing={facing},part=head]")
        return self

    def table(self, x, z, y=1, wood="spruce", cover=None):
        self.set(x, y, z, f"minecraft:{wood}_fence")
        self.set(x, y + 1, z, cover or "minecraft:white_carpet")
        return self

    def chair(self, x, z, facing, y=1, wood="spruce"):
        """Cadeira de escada: o encosto fica no lado oposto a `facing`."""
        back = DIRS[(DIRS.index(facing) + 2) % 4]
        return self.set(x, y, z, f"minecraft:{wood}_stairs[facing={back},half=bottom,shape=straight]")

    def note(self, x, y, z, text):
        self.notes.append((x, y, z, text))
        return self

    # ---------- saída ----------
    def bounds(self):
        xs = [p[0] for p in self.blocks]; ys = [p[1] for p in self.blocks]; zs = [p[2] for p in self.blocks]
        return min(xs), min(ys), min(zs), max(xs), max(ys), max(zs)

    def rotated(self, quarters):
        """Gira de `quarters` x 90 graus no sentido horário (visto de cima: norte vira leste)."""
        quarters %= 4
        out = Model(self.name, self.width, self.depth, self.title)
        if quarters % 2:
            out.width, out.depth = self.depth, self.width

        def rot(x, z):
            for _ in range(quarters):
                x, z = -z, x
            return x, z
        moved = {}
        for (x, y, z), state in self.blocks.items():
            nx, nz = rot(x, z)
            moved[(nx, y, nz)] = rotate_state(state, quarters)
        # O zero é o canto noroeste das paredes (não o do beiral): assim um acabamento girado junto
        # cai no mesmo lugar que o prédio.
        corners = [rot(self.core[0], self.core[1]), rot(self.core[2], self.core[3])]
        mx = min(c[0] for c in corners); mz = min(c[1] for c in corners)
        out.blocks = {(x - mx, y, z - mz): s for (x, y, z), s in moved.items()}
        out.notes = [(rot(x, z)[0] - mx, y, rot(x, z)[1] - mz, t) for x, y, z, t in self.notes]
        out.core = (0, 0, max(c[0] for c in corners) - mx, max(c[1] for c in corners) - mz)
        if self.entrance:
            dx, dz = rot(self.entrance[0], self.entrance[1])
            out.entrance = (dx - mx, dz - mz, turn(self.entrance[2], quarters))
        return out

    def connected(self):
        """Liga cercas, muros, barras e vidraças aos vizinhos (ao mesmo tipo ou a bloco sólido)."""
        out = Model(self.name, self.width, self.depth, self.title)
        out.notes, out.entrance, out.core = list(self.notes), self.entrance, self.core
        for (x, y, z), state in self.blocks.items():
            b = base_id(state)
            kind = next((t for t in THIN if b.endswith(t)), None)
            if kind is None:
                out.blocks[(x, y, z)] = state
                continue
            p = props(state)
            for d, (dx, dz) in DELTA.items():
                other = self.blocks.get((x + dx, y, z + dz), "minecraft:air")
                ob = base_id(other)
                links = ob.endswith(kind) or solid(other) or (ob.endswith("_fence_gate") and kind == "_fence")
                if kind == "_fence" and ob.endswith("_wall"):
                    links = False
                if kind == "_wall":
                    p[d] = "low" if links else "none"
                else:
                    p[d] = "true" if links else "false"
            if kind == "_wall":
                p["up"] = "true"
            out.blocks[(x, y, z)] = with_props(b, p)
        return out

    def commands(self):
        """Comandos relativos ao canto noroeste (y = nível do piso). Junta blocos iguais seguidos."""
        lines = []
        by_row = {}
        for (x, y, z), state in self.blocks.items():
            by_row.setdefault((y, z), []).append((x, state))
        for (y, z) in sorted(by_row):
            row = sorted(by_row[(y, z)])
            start = 0
            while start < len(row):
                end = start
                while end + 1 < len(row) and row[end + 1][1] == row[start][1] and row[end + 1][0] == row[end][0] + 1:
                    end += 1
                x0, state = row[start]; x1 = row[end][0]
                if x0 == x1:
                    lines.append(f"setblock ~{x0} ~{y} ~{z} {state}")
                else:
                    lines.append(f"fill ~{x0} ~{y} ~{z} ~{x1} ~{y} ~{z} {state}")
                start = end + 1
        return lines


# ---------- checagem dos estados de bloco ----------
KNOWN_PROPS = {
    "stairs": {"facing", "half", "shape", "waterlogged"}, "slab": {"type", "waterlogged"},
    "door": {"facing", "half", "hinge", "open", "powered"}, "trapdoor": {"facing", "half", "open", "powered", "waterlogged"},
    "bed": {"facing", "part", "occupied"}, "_log": {"axis"}, "_wood": {"axis"}, "hay_block": {"axis"},
    "glass_pane": {"north", "south", "east", "west", "waterlogged"}, "fence_gate": {"facing", "open", "in_wall", "powered"},
    "fence": {"north", "south", "east", "west", "waterlogged"}, "_wall": {"north", "south", "east", "west", "up", "waterlogged"},
    "iron_bars": {"north", "south", "east", "west", "waterlogged"},
    "lantern": {"hanging", "waterlogged"}, "campfire": {"lit", "facing", "signal_fire", "waterlogged"},
    "barrel": {"facing", "open"}, "chest": {"facing", "type", "waterlogged"}, "furnace": {"facing", "lit"},
    "smoker": {"facing", "lit"}, "blast_furnace": {"facing", "lit"}, "anvil": {"facing"}, "grindstone": {"face", "facing"},
    "loom": {"facing"}, "lectern": {"facing", "has_book", "powered"}, "bell": {"attachment", "facing", "powered"},
    "candle": {"candles", "lit", "waterlogged"}, "ladder": {"facing", "waterlogged"}, "wall_torch": {"facing"},
    "vine": {"north", "south", "east", "west", "up"}, "decorated_pot": {"facing", "waterlogged", "cracked"},
    "water_cauldron": {"level"}, "scaffolding": {"bottom", "distance", "waterlogged"}, "tall_grass": {"half"},
    "large_fern": {"half"}, "lilac": {"half"}, "rose_bush": {"half"}, "peony": {"half"}, "wheat": {"age"}, "carrots": {"age"},
    "potatoes": {"age"}, "farmland": {"moisture"}, "composter": {"level"}, "cartography_table": set(), "pink_petals": {"facing", "flower_amount"},
    "banner": {"rotation"}, "wall_banner": {"facing"}, "chain": {"axis", "waterlogged"}, "beehive": {"facing", "honey_level"},
    "stonecutter": {"facing"}, "pillar": {"axis"}, "basalt": {"axis"}, "end_rod": {"facing"}, "lightning_rod": {"facing", "powered", "waterlogged"},
}
STATE_RE = re.compile(r"^minecraft:[a-z_]+(\[[a-z_]+=[a-z0-9_]+(,[a-z_]+=[a-z0-9_]+)*\])?$")


def check_state(state):
    """Erro de sintaxe ou propriedade estranha num estado de bloco (None se está certo)."""
    if not STATE_RE.match(state):
        return f"sintaxe: {state}"
    b = base_id(state).split(":", 1)[1]
    p = props(state)
    if not p:
        return None
    family = None
    for key in sorted(KNOWN_PROPS, key=len, reverse=True):
        if (key.startswith("_") and b.endswith(key)) or b == key or b.endswith("_" + key):
            family = key
            break
    if family is None:
        return f"propriedades num bloco sem regra conhecida: {state}"
    extra = set(p) - KNOWN_PROPS[family]
    if extra:
        return f"propriedade inválida {extra} em {state}"
    return None
