"""A vila do Veio: põe os modelos de `predios.py` nos lugares do plano e gera o que o jogo usa.

- `content/data/veio/function/village/<peça>.mcfunction`: uma função por prédio (já girado para a
  porta dar para a rua), rodada pelo Rust com `execute positioned <canto> <piso> <canto> run function`.
  Cada função limpa o volume, tira as árvores em volta, faz a fundação (pedregulho onde o chão
  desce, terra embaixo), põe os blocos e chama os moradores (um aldeão por casa habitada).
- `content/data/veio/function/village/ruas/<estágio>_<i>.mcfunction`: as ruas em pedaços de 32 x 32,
  relativas ao centro da vila; cada bloco cai no chão do lugar (`positioned over`).
- `native/api/src/workforce/town_plan.rs`: as peças, os pedaços de rua e os lugares dos moradores
  com nome, para o construtor (`workforce/town.rs`).

As posições vêm de `tools/mapa/vila_estagio1.py` (estágio 1) e `tools/mapa/vila_crescimento.py`
(estágios seguintes), relativas ao centro da vila (a fonte da praça). x para leste, z para o sul.

Uso: python3 tools/vila/build_village.py [--check]
"""
import hashlib
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "tools/mapa"))

import predios as P                                          # noqa: E402
from modelo import check_state, DELTA, base_id               # noqa: E402
import vila_estagio1 as V1                                    # noqa: E402
import vila_crescimento as VC                                 # noqa: E402

FUNCTIONS = ROOT / "content/data/veio/function/village"
PLAN_RS = ROOT / "native/api/src/workforce/town_plan.rs"
ROAD_TAG = ROOT / "content/data/veio/tags/block/road_ground.json"
NAMESPACE = "veio:village/"
TILE = 32
MAX_FILL = 32768

# A casa do fundador é construída por `tools/build_house.py` (13 x 11, porta no sul). Aqui só
# reservamos o lugar: canto noroeste relativo ao centro.
FOUNDER_CENTER = V1.BUILDINGS["Casa do fundador"][:2]
FOUNDER_SIZE = (13, 11)
FOUNDER_NW = (round(FOUNDER_CENTER[0] - FOUNDER_SIZE[0] / 2 + 0.5) - 1, round(FOUNDER_CENTER[1] - FOUNDER_SIZE[1] / 2))
# O terreno da oficina (o jogador nivela): centro e lado
PLOT_CENTER = (57, -21)


# =====================================================================================
# As peças
# =====================================================================================

class Piece:
    def __init__(self, key, factory, center, stage=1, until=99, site=None, people=0, rotate=None, extras=(), overlay_of=None):
        self.key, self.factory, self.center = key, factory, center
        self.overlay_of = overlay_of       # acabamento: só acrescenta blocos ao prédio `overlay_of`
        self.stage, self.until = stage, until
        self.site = site or key            # peças no mesmo lugar (as trocas de estágio) dividem o piso
        self.people = people               # aldeões que a função chama (um por cama, até este número)
        self.rotate = rotate               # None: a porta vira para a rua mais perto
        self.extras = extras               # comandos a mais no fim da função (relativos ao canto)


PIECES = []
_NAMES = ["Ferreiro · Seu Bento", "Lavradora · Dona Cida", "Vendinha · Seu Zé", "Olaria · Dona Zefa",
          "Carvoaria · Seu Lino", "Vaqueiro · Seu Tonho"]
_NPC_MODELS = [("ferreiro", P.ferreiro), ("lavradora", P.lavradora), ("vendinha", P.vendinha), ("olaria", P.olaria),
               ("carvoaria", P.carvoaria), ("vaqueiro", P.vaqueiro)]

PIECES.append(Piece("praca", lambda: P.praca(1), (0, 0.5), until=2, site="praca", rotate=0,
                    extras=["summon minecraft:iron_golem ~20.5 ~1 ~14.5 {PersistenceRequired:1b,PlayerCreated:0b}"]))
PIECES.append(Piece("praca_2", lambda: P.praca(2), (0, 0.5), stage=2, site="praca", rotate=0))
SEEDS = {}


def _worn(factory, seed):
    """Fábrica do prédio do estágio 1 com o desgaste (sempre o mesmo, pela semente)."""
    return lambda: P.worn(factory, seed)[0]


def _finish(factory, level, seed, worn):
    def make():
        base, changes = P.worn(factory, seed) if worn else (factory(), ())
        return P.finish(base, level, seed, changes)
    return make


def finishing(base_key, factory, center, seed, worn, levels):
    site = next(p.site for p in PIECES if p.key == base_key)
    for level in levels:
        PIECES.append(Piece(f"{base_key}_acabamento_{level}", _finish(factory, level, seed, worn), center, stage=level,
                            site=site, overlay_of=base_key))


for i, ((key, factory), name) in enumerate(zip(_NPC_MODELS, _NAMES)):
    x, z = V1.BUILDINGS[name][:2]
    PIECES.append(Piece(key, _worn(factory, 100 + i), (x, z)))
    finishing(key, factory, (x, z), 100 + i, True, (2, 4))
PIECES.append(Piece("roca", P.roca, V1.GROUNDS["Roça"][:2], rotate=0))
PIECES.append(Piece("pasto", P.pasto, V1.GROUNDS["Pasto"][:2], rotate=0,
                    extras=['summon minecraft:cow ~25.5 ~1 ~22.5 {PersistenceRequired:1b,CustomName:"Mimosa"}']))
PIECES.append(Piece("poco", lambda: P.poco(1), V1.GROUNDS["Poço"][:2], until=4, site="poco"))
PIECES.append(Piece("poco_4", lambda: P.poco(4), V1.GROUNDS["Poço"][:2], stage=4, site="poco"))
for i, ((x, z, _), factory) in enumerate(zip(V1.HOUSES, P.HOUSES)):
    extras = ['summon minecraft:cat {livre} {PersistenceRequired:1b,variant:"minecraft:tabby"}'] if factory is P.casa_jardim else []
    PIECES.append(Piece(factory.__name__, _worn(factory, i), (x, z), people=2 if factory in (P.casa_enxaimel, P.sobrado) else 1,
                        extras=extras))
    finishing(factory.__name__, factory, (x, z), i, True, (2, 4))
for (x, z), base in zip(V1.WORKS, [P.casa_enxaimel, P.casa_varanda]):
    key = base.__name__ + "_obra"
    PIECES.append(Piece(key, lambda b=base: P.em_obra(b), (x, z), until=2, site=key))
for (x, z), factory in zip(V1.WORKS, [P.casa_enxaimel_nova, P.casa_varanda_nova]):
    n = factory.__name__
    PIECES.append(Piece(n, factory, (x, z), stage=2, site=n.replace("_nova", "") + "_obra", people=1))
    finishing(n, factory, (x, z), 50, False, (4,))
for x, z in V1.RUINS:
    PIECES.append(Piece("ruina", P.ruina, (x, z), until=2, site="ruina"))
    PIECES.append(Piece("casa_reformada", P.casa_reformada, (x, z), stage=2, site="ruina", people=1))
    finishing("casa_reformada", P.casa_reformada, (x, z), 51, False, (4,))
for i, ((x, z), factory) in enumerate(zip(V1.LOTS, P.NEW_HOUSE_MODELS)):
    PIECES.append(Piece(f"lote_{i + 1}", P.lote, (x, z), until=3, site=f"lote_{i + 1}"))
    PIECES.append(Piece(factory.__name__, factory, (x, z), stage=3, site=f"lote_{i + 1}", people=1))
    finishing(factory.__name__, factory, (x, z), 60 + i, False, (4,))
_extra = [(s, x, z) for s, x, z in VC.NEW_HOUSES if s == 3 and (x, z) not in V1.LOTS]
for (s, x, z), factory in zip(_extra, P.NEW_HOUSE_MODELS[len(V1.LOTS):]):
    PIECES.append(Piece(factory.__name__, factory, (x, z), stage=3, people=1))
    finishing(factory.__name__, factory, (x, z), 70, False, (4,))
_hall = next((x, z) for s, name, x, z, w, h in VC.NEW if name == "Prefeitura")
PIECES.append(Piece("prefeitura_3", lambda: P.prefeitura(3), _hall, stage=3, until=4, site="prefeitura"))
PIECES.append(Piece("prefeitura_4", lambda: P.prefeitura(4), _hall, stage=4, until=6, site="prefeitura"))
# o paço é maior (16 x 12): cresce para leste, longe da casa enxaimel
PIECES.append(Piece("prefeitura_6", lambda: P.prefeitura(6), (_hall[0] + 3, _hall[1]), stage=6, site="prefeitura"))


# =====================================================================================
# Ruas
# =====================================================================================

def streets(stage):
    """As ruas que existem no estágio: (pontos, largura)."""
    out = list(V1.ROADS)
    out += [(pts, w) for s, _, pts, w in VC.STREETS if s <= stage]
    return out


def seg_distance(p, a, b):
    ax, az = a; bx, bz = b; px, pz = p
    dx, dz = bx - ax, bz - az
    L = dx * dx + dz * dz
    t = 0 if L == 0 else max(0, min(1, ((px - ax) * dx + (pz - az) * dz) / L))
    return math.hypot(px - ax - t * dx, pz - az - t * dz)


def road_cells(stage):
    cells = {}
    for pts, width in streets(stage):
        half = width / 2
        for a, b in zip(pts, pts[1:]):
            x0, x1 = int(min(a[0], b[0]) - width - 1), int(max(a[0], b[0]) + width + 1)
            z0, z1 = int(min(a[1], b[1]) - width - 1), int(max(a[1], b[1]) + width + 1)
            for x in range(x0, x1 + 1):
                for z in range(z0, z1 + 1):
                    d = seg_distance((x + 0.5, z + 0.5), a, b)
                    if d <= half:
                        edge = d > half - 1 and width >= 3
                        cells[(x, z)] = cells.get((x, z), True) and edge
    return cells          # (x, z) -> é borda?


def h01(*parts):
    """Número de 0 a 1 repetível a partir das coordenadas (sem depender do `random` do Python)."""
    digest = hashlib.blake2b(repr(parts).encode(), digest_size=4).digest()
    return int.from_bytes(digest, "big") / 2 ** 32


def style_for(stage):
    keys = sorted(k for k in P.STREET_STYLES if k <= stage)
    return keys[-1]


def street_block(stage, x, z, edge):
    style = style_for(stage)
    if edge and P.STREET_EDGE[style]:
        return "minecraft:" + P.STREET_EDGE[style]
    r = h01("rua", style, x, z)
    acc = 0.0
    for block, share in P.STREET_STYLES[style][1]:
        acc += share
        if r < acc:
            return None if block == "grass_block" else "minecraft:" + block
    return "minecraft:" + P.STREET_STYLES[style][1][0][0]


# =====================================================================================
# Montagem
# =====================================================================================

class Placed:
    """Uma peça posta no mundo (coordenadas relativas ao centro da vila)."""

    def __init__(self, piece, model, quarters, origin):
        self.piece, self.model, self.quarters, self.origin = piece, model, quarters, origin

    def world(self, x, z):
        return self.origin[0] + x, self.origin[1] + z

    def own_area(self):
        x0, _, z0, x1, _, z1 = self.model.bounds()
        return (self.origin[0] + x0, self.origin[1] + z0, self.origin[0] + x1, self.origin[1] + z1)

    @property
    def area(self):
        """O que a peça ocupa, com os acabamentos dos estágios seguintes (o acabamento ocupa o
        mesmo que o prédio)."""
        base = self.piece.overlay_of or self.piece.key
        rects = [placed_of(p).own_area() for p in PIECES if p.key == base or p.overlay_of == base]
        return (min(r[0] for r in rects), min(r[1] for r in rects), max(r[2] for r in rects), max(r[3] for r in rects))

    @property
    def core(self):
        a0, b0, a1, b1 = self.model.core
        return (self.origin[0] + a0, self.origin[1] + b0, self.origin[0] + a1, self.origin[1] + b1)

    def front(self):
        """A célula na frente da porta (fora do prédio)."""
        if not self.model.entrance:
            return None
        ex, ez, side = self.model.entrance
        dx, dz = DELTA[side]
        return self.world(ex + dx, ez + dz)


_PLACED = {}


def placed_of(piece):
    """A posição de cada peça é decidida uma vez, com as ruas do estágio em que ela nasce; o
    acabamento vai junto com o prédio."""
    if piece.key in _PLACED:
        return _PLACED[piece.key]
    if piece.overlay_of:
        base = placed_of(next(p for p in PIECES if p.key == piece.overlay_of))
        out = Placed(piece, piece.factory().rotated(base.quarters), base.quarters, base.origin)
    else:
        cells = road_cells(piece.stage)
        plaza = [(x, z) for x in range(-13, 14) for z in range(-9, 10)]
        out = place(piece, list(cells) + plaza)
    _PLACED[piece.key] = out
    return out


def place(piece, avoid_cells):
    base = piece.factory()
    options = [piece.rotate] if piece.rotate is not None else [0, 1, 2, 3]
    best = None
    for q in options:
        model = base.rotated(q)
        a0, b0, a1, b1 = model.core
        ox = math.floor(piece.center[0] - (a0 + a1 + 1) / 2 + 0.5)
        oz = math.floor(piece.center[1] - (b0 + b1 + 1) / 2 + 0.5)
        placed = Placed(piece, model, q, (ox, oz))
        front = placed.front()
        if front is None:
            return placed
        score = min((math.hypot(front[0] - x, front[1] - z) for (x, z) in avoid_cells), default=0)
        if best is None or score < best[0]:
            best = (score, placed)
    return best[1]


def layout(stage, overlays=False):
    """As peças de pé no estágio (as de estágios anteriores que não foram trocadas)."""
    return [placed_of(p) for p in PIECES if p.stage <= stage < p.until and (overlays or not p.overlay_of)]


def rect_overlap(a, b, gap=0):
    return not (a[2] + gap < b[0] or b[2] + gap < a[0] or a[3] + gap < b[1] or b[3] + gap < a[1])


def founder_rect():
    x, z = FOUNDER_NW
    return (x, z, x + FOUNDER_SIZE[0] - 1, z + FOUNDER_SIZE[1] - 1)


def plot_rect():
    return (PLOT_CENTER[0] - 5, PLOT_CENTER[1] - 5, PLOT_CENTER[0] + 5, PLOT_CENTER[1] + 5)


def problems():
    """Prédios sobrepostos, ruas passando por dentro de prédio, estados de bloco inválidos."""
    out = []
    for stage in (1, 2, 3, 4, 6):
        placed = layout(stage)
        fixed = [("casa do fundador", founder_rect()), ("terreno da oficina", plot_rect())]
        rects = [(p.piece.key, p.area) for p in placed] + fixed
        for i in range(len(rects)):
            for j in range(i + 1, len(rects)):
                if rect_overlap(rects[i][1], rects[j][1]):
                    out.append(f"estágio {stage}: {rects[i][0]} encosta em {rects[j][0]}")
        for p in placed:
            if p.front() is None:
                continue
            for _, r in rects:
                fx, fz = p.front()
                if r[0] <= fx <= r[2] and r[1] <= fz <= r[3] and r != p.area:
                    out.append(f"estágio {stage}: a porta de {p.piece.key} dá para dentro de outro prédio")
    seen = set()
    for p in PIECES:
        model = p.factory()
        if model.name in seen:
            continue
        seen.add(model.name)
        for s in set(model.blocks.values()):
            err = check_state(s)
            if err:
                out.append(f"{model.name}: {err}")
    return out


# =====================================================================================
# Saída: funções
# =====================================================================================

def fills(x0, y0, z0, x1, y1, z1, what, replace=None):
    """Um `fill` (ou vários, se o volume passa do limite do comando)."""
    lines = []
    span_x = x1 - x0 + 1
    span_z = z1 - z0 + 1
    layer = span_x * span_z
    step = max(1, MAX_FILL // layer)
    for y in range(y0, y1 + 1, step):
        top = min(y1, y + step - 1)
        tail = f" replace {replace}" if replace else ""
        lines.append(f"fill ~{x0} ~{y} ~{z0} ~{x1} ~{top} ~{z1} {what}{tail}")
    return lines


def building_function(placed, previous=()):
    """`previous`: as peças que esta troca (o mesmo lugar, estágio anterior); a área delas é limpa
    antes, e o piso delas volta a ser grama."""
    model = placed.model.connected()
    x0, y0, z0, x1, y1, z1 = model.bounds()
    a0, b0, a1, b1 = model.core
    lines = [
        f"# Gerado por tools/vila/build_village.py. Não edite à mão.",
        f"# {model.title}: rode com `execute positioned <canto> <piso> <canto> run function {NAMESPACE}{placed.piece.key}`.",
    ]
    for old in previous:
        ox0, oz0, ox1, oz1 = old.area
        top = old.model.bounds()[4]
        dx, dz = placed.origin
        lines += fills(ox0 - dx, 1, oz0 - dz, ox1 - dx, top + 1, oz1 - dz, "minecraft:air")
        lines += fills(ox0 - dx, 0, oz0 - dz, ox1 - dx, 0, oz1 - dz, "minecraft:grass_block")
        lines.append(f"kill @e[type=minecraft:item,x=~{ox0 - dx},y=~-1,z=~{oz0 - dz},dx={ox1 - ox0 + 1},dy={top + 3},dz={oz1 - oz0 + 1}]")
    lines += [
        # árvores em volta (as folhas soltas caem sozinhas sem tronco perto)
        *fills(x0 - 2, 0, z0 - 2, x1 + 2, y1 + 8, z1 + 2, "minecraft:air", "#minecraft:logs"),
        *fills(x0 - 2, 0, z0 - 2, x1 + 2, y1 + 8, z1 + 2, "minecraft:air", "#minecraft:leaves"),
        # o volume do prédio, do piso para cima (e a camada de cima do morro, se o chão sobe)
        *fills(x0, 1, z0, x1, y1 + 2, z1, "minecraft:air"),
        # chão: grama onde falta (buraco, água, planta) e terra embaixo; pedregulho na borda das paredes
        *fills(x0, 0, z0, x1, 0, z1, "minecraft:grass_block", "#minecraft:replaceable"),
        *fills(x0, -6, z0, x1, -1, z1, "minecraft:dirt", "#minecraft:replaceable"),
    ]
    if placed.piece.key not in ("roca", "pasto", "lote_1", "lote_2", "lote_3", "lote_4"):
        for (rx0, rz0, rx1, rz1) in ((a0, b0, a1, b0), (a0, b1, a1, b1), (a0, b0, a0, b1), (a1, b0, a1, b1)):
            lines.append(f"fill ~{rx0} ~-4 ~{rz0} ~{rx1} ~-1 ~{rz1} minecraft:cobblestone replace minecraft:dirt")
    body = model.commands()
    lines += body
    # moradores: um aldeão perto de cada cama, até `people`
    beds = sorted((x, y, z) for (x, y, z), s in model.blocks.items() if base_id(s).endswith("_bed") and "part=head" in s)
    for (x, y, z) in beds[:placed.piece.people]:
        lines.append(f"summon minecraft:villager ~{x + 0.5} ~{y} ~{z + 0.5} "
                     '{PersistenceRequired:1b,VillagerData:{type:"minecraft:plains",profession:"minecraft:none",level:1}}')
    free = free_cell(model)
    lines += [extra.replace("{livre}", free) for extra in placed.piece.extras]
    return lines


def free_cell(model):
    """Um lugar livre no chão de dentro (para um gato, por exemplo)."""
    a0, b0, a1, b1 = model.core
    cx, cz = (a0 + a1) / 2, (b0 + b1) / 2
    cells = sorted(((x, z) for x in range(a0 + 1, a1) for z in range(b0 + 1, b1)),
                   key=lambda c: (c[0] - cx) ** 2 + (c[1] - cz) ** 2)
    for x, z in cells:
        if all(model.get(x, y, z) in (None, "minecraft:air") for y in (1, 2)):
            return f"~{x + 0.5} ~1 ~{z + 0.5}"
    return f"~{cx} ~1 ~{cz}"


def overlay_function(placed, base):
    """O acabamento de um estágio: só os blocos novos, ligados aos vizinhos do prédio."""
    whole = type(base.model)(base.model.name, base.model.width, base.model.depth, base.model.title)
    whole.blocks = dict(base.model.blocks)
    whole.blocks.update(placed.model.blocks)
    whole.core = base.model.core
    linked = whole.connected()
    only = type(base.model)(placed.model.name, placed.model.width, placed.model.depth, placed.model.title)
    only.blocks = {k: linked.blocks[k] for k in placed.model.blocks}
    # vidraças vizinhas das que voltaram também mudam de ligação
    for (x, y, z) in placed.model.blocks:
        for dx, dz in DELTA.values():
            k = (x + dx, y, z + dz)
            if k in base.model.blocks and "glass_pane" in base.model.blocks[k]:
                only.blocks[k] = linked.blocks[k]
    return [f"# Gerado por tools/vila/build_village.py. Não edite à mão.",
            f"# {placed.model.title}: só acrescenta (roda no mesmo canto e piso de {NAMESPACE}{base.piece.key})."] + only.commands()


def road_functions(stage, placed):
    """Os pedaços de rua do estágio: {nome: (linhas, retângulo)}."""
    cells = road_cells(stage)
    blocked = [p.area for p in placed] + [founder_rect(), plot_rect()]
    # caminhos da porta até a rua: em L, um bloco de largura
    paths = set()
    for p in placed:
        front = p.front()
        if front is None or front in cells:
            continue
        target = min(cells, key=lambda c: abs(c[0] - front[0]) + abs(c[1] - front[1]))
        x, z = front
        while (x, z) != target and abs(x - front[0]) + abs(z - front[1]) < 30:
            paths.add((x, z))
            if x != target[0]:
                x += 1 if target[0] > x else -1
            else:
                z += 1 if target[1] > z else -1
    for c in paths:
        cells.setdefault(c, False)

    def free(x, z):
        return not any(r[0] <= x <= r[2] and r[1] <= z <= r[3] for r in blocked)
    style = style_for(stage)
    tiles = {}
    for (x, z), edge in sorted(cells.items()):
        if not free(x, z):
            continue
        block = street_block(stage, x, z, edge)
        tile = (x // TILE, z // TILE)
        lines = tiles.setdefault(tile, [])
        at = f"execute positioned ~{x} ~ ~{z} positioned over motion_blocking_no_leaves run"
        if block is None:                                   # mato crescendo no meio da rua
            continue
        if stage == 1:
            # tronco no meio da rua (o chão da rua fica embaixo dele): sai a coluna toda
            lines.append(f"fill ~{x} ~-8 ~{z} ~{x} ~24 ~{z} minecraft:air replace #minecraft:logs")
        lines.append(f"{at} fill ~ ~-1 ~ ~ ~-1 ~ {block} replace #veio:road_ground")
        lines.append(f"{at} fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees")
    # lanternas: a cada tantos blocos de rua, um poste na borda (estágio 2 em diante)
    every = P.STREET_LAMPS[style]
    if every:
        count = 0
        for (x, z), edge in sorted(cells.items(), key=lambda kv: (kv[0][1], kv[0][0])):
            if not edge or not free(x, z):
                continue
            count += 1
            if count % every:
                continue
            for dx, dz in DELTA.values():
                nx, nz = x + dx, z + dz
                if (nx, nz) not in cells and free(nx, nz):
                    at = f"execute positioned ~{nx} ~ ~{nz} positioned over motion_blocking_no_leaves run"
                    tiles.setdefault((nx // TILE, nz // TILE), []).append(
                        f"{at} fill ~ ~ ~ ~ ~1 ~ minecraft:oak_fence")
                    tiles[(nx // TILE, nz // TILE)].append(f"{at} setblock ~ ~2 ~ minecraft:lantern[hanging=false]")
                    break
    out = {}
    for (tx, tz), lines in sorted(tiles.items()):
        name = f"ruas/{stage}_{tx}_{tz}".replace("-", "m")
        rect = (tx * TILE, tz * TILE, tx * TILE + TILE - 1, tz * TILE + TILE - 1)
        out[name] = ([f"# Gerado por tools/vila/build_village.py. Ruas do estágio {stage} ({P.STREET_STYLES[style][0]}),"
                      f" relativas ao centro da vila."] + lines, rect)
    return out


ROAD_STAGES = (1, 2, 3, 4, 6)

# O verde da vila: árvores e canteiros naturais entre as casas, longe das ruas e dos lugares
# de todos os estágios (o que vai nascer depois não encontra árvore no caminho).
TREES = ["minecraft:oak", "minecraft:birch", "minecraft:oak", "minecraft:fancy_oak"]
PATCHES = ["minecraft:flower_default", "minecraft:flower_plain", "minecraft:patch_grass"]
GREEN_BOX = (-118, -70, 100, 104)


def greenery():
    """{nome: (linhas, retângulo)}: árvores (`place feature`) e canteiros de flores e capim."""
    taken = []
    for p in PIECES:
        taken.append(placed_of(p).area)
    taken += [founder_rect(), plot_rect()]
    taken += [(-14, -10, 14, 10)]                                    # a praça fica aberta
    # o que o plano de crescimento ainda vai construir (parque, quartel, distrito, casas novas)
    taken += [(x - w / 2, z - h / 2, x + w / 2, z + h / 2) for _, _, x, z, w, h in VC.NEW]
    taken += [(x - 5, z - 4, x + 5, z + 4) for _, x, z in VC.NEW_HOUSES]
    for a, b2 in zip(VC.WALL, VC.WALL[1:]):                           # a muralha do estágio 2
        steps = int(math.hypot(b2[0] - a[0], b2[1] - a[1]) // 4) + 1
        for k in range(steps + 1):
            x = a[0] + (b2[0] - a[0]) * k / steps; z = a[1] + (b2[1] - a[1]) * k / steps
            taken.append((x - 3, z - 3, x + 3, z + 3))
    roads = set()
    for stage in ROAD_STAGES:
        roads |= set(road_cells(stage))

    def clear_of(x, z, gap_house, gap_road):
        if any(r[0] - gap_house <= x <= r[2] + gap_house and r[1] - gap_house <= z <= r[3] + gap_house for r in taken):
            return False
        return not any((x + dx, z + dz) in roads for dx in range(-gap_road, gap_road + 1) for dz in range(-gap_road, gap_road + 1))
    trees, patches = [], []
    X0, Z0, X1, Z1 = GREEN_BOX
    for x in range(X0, X1, 3):
        for z in range(Z0, Z1, 3):
            jx, jz = x + int(h01("jx", x, z) * 3), z + int(h01("jz", x, z) * 3)
            r = h01("verde", jx, jz)
            # mais árvores na borda da vila, poucas no meio
            edge = min(1.0, math.hypot(jx / 90, jz / 75))
            if r < 0.05 + 0.10 * edge and clear_of(jx, jz, 4, 4) and all(math.hypot(jx - a, jz - b) >= 7 for a, b in trees):
                trees.append((jx, jz))
            elif r > 0.94 and clear_of(jx, jz, 2, 2) and all(math.hypot(jx - a, jz - b) >= 6 for a, b in patches):
                patches.append((jx, jz))
    tiles = {}
    for x, z in trees:
        feature = TREES[int(h01("arvore", x, z) * len(TREES))]
        tiles.setdefault((x // TILE, z // TILE), []).append(
            f"execute positioned ~{x} ~ ~{z} positioned over motion_blocking_no_leaves if block ~ ~-1 ~ minecraft:grass_block "
            f"run place feature {feature} ~ ~ ~")
    for x, z in patches:
        feature = PATCHES[int(h01("canteiro", x, z) * len(PATCHES))]
        tiles.setdefault((x // TILE, z // TILE), []).append(
            f"execute positioned ~{x} ~ ~{z} positioned over motion_blocking_no_leaves run place feature {feature} ~ ~ ~")
    out = {}
    for (tx, tz), lines in sorted(tiles.items()):
        name = f"verde/{tx}_{tz}".replace("-", "m")
        rect = (tx * TILE, tz * TILE, tx * TILE + TILE - 1, tz * TILE + TILE - 1)
        out[name] = ([f"# Gerado por tools/vila/build_village.py. Árvores e canteiros da vila, relativos ao centro."] + lines, rect)
    return out, trees, patches


def build():
    files = {}
    pieces = {}
    for stage in (1, 2, 3, 4, 6):
        for placed in layout(stage, overlays=True):
            pieces.setdefault(placed.piece.key, placed)
    for key, placed in pieces.items():
        if placed.piece.overlay_of:
            files[f"{key}.mcfunction"] = overlay_function(placed, pieces[placed.piece.overlay_of])
            continue
        previous = [p for p in pieces.values() if p.piece.site == placed.piece.site and p.piece.stage < placed.piece.stage
                    and not p.piece.overlay_of]
        files[f"{key}.mcfunction"] = building_function(placed, previous)
    roads = {}
    for stage in ROAD_STAGES:
        later = [s for s in ROAD_STAGES if s > stage]
        until = later[0] if later else 99
        for name, (lines, rect) in road_functions(stage, layout(stage)).items():
            files[name + ".mcfunction"] = lines
            roads[name] = (stage, until, rect)
    green, _, _ = greenery()
    for name, (lines, rect) in green.items():
        files[name + ".mcfunction"] = lines
        roads[name] = (1, 99, rect)
    return files, pieces, roads


# =====================================================================================
# Saída: plano para o Rust
# =====================================================================================

def spots(placed):
    """Quatro lugares em frente à porta: o da frente, os dois lados e um passo adiante."""
    front = placed.front()
    side = placed.model.entrance[2]
    dx, dz = DELTA[side]
    lx, lz = -dz, dx
    fx, fz = front
    return [(fx + dx, fz + dz), (fx + dx + lx * 2, fz + dz + lz * 2), (fx + dx - lx * 2, fz + dz - lz * 2), (fx + 2 * dx, fz + 2 * dz)]


def rust_plan(pieces, roads):
    p1 = {p.piece.key: p for p in layout(1)}
    lines = [
        "// Gerado por tools/vila/build_village.py a partir de tools/vila/predios.py. Não edite à mão.",
        "//! O plano da vila do Veio: as peças (prédios, praça, roça, pasto), os pedaços de rua e os",
        "//! lugares dos moradores com nome, relativos ao centro da vila (a fonte da praça).",
        "use super::town::{Piece, Road};",
        "",
        f"pub(crate) const PIECES: [Piece; {len(pieces)}] = [",
    ]
    order = sorted(pieces.values(), key=lambda p: (p.piece.stage, math.hypot(*p.piece.center)))
    for p in order:
        a0, b0, a1, b1 = p.core
        x0, z0, x1, z1 = p.area
        lines.append(
            f'    Piece {{ key: "{p.piece.key}", site: "{p.piece.site}", at: [{p.origin[0]}, {p.origin[1]}], '
            f"core: [{a0}, {b0}, {a1}, {b1}], area: [{x0}, {z0}, {x1}, {z1}], "
            f"stage: {p.piece.stage}, until: {p.piece.until}, overlay: {'true' if p.piece.overlay_of else 'false'} }},")
    lines.append("];")
    lines.append(f"pub(crate) const ROADS: [Road; {len(roads)}] = [")
    for name, (stage, until, rect) in sorted(roads.items(), key=lambda kv: (kv[1][0], kv[0])):
        lines.append(f'    Road {{ name: "{name}", stage: {stage}, until: {until}, area: [{rect[0]}, {rect[1]}, {rect[2]}, {rect[3]}] }},')
    lines.append("];")
    lines.append(f"/// Canto noroeste da casa do fundador (`home::FOOTPRINT`).")
    lines.append(f"pub(crate) const FOUNDER: [i32; 2] = [{FOUNDER_NW[0]}, {FOUNDER_NW[1]}];")
    lines.append(f"/// Centro do terreno da oficina.")
    lines.append(f"pub(crate) const PLOT: [i32; 2] = [{PLOT_CENTER[0]}, {PLOT_CENTER[1]}];")
    lines.append("/// O prefeito: na praça, entre a fonte e o coreto.")
    lines.append("pub(crate) const MAYOR: [(i32, i32); 4] = [(-3, -6), (-3, -4), (-2, -7), (-4, -8)];")
    for const, key in [("SMITH", "ferreiro"), ("FARMER", "lavradora"), ("GROCER", "vendinha"), ("POTTER", "olaria"),
                       ("COLLIER", "carvoaria"), ("COWHERD", "vaqueiro")]:
        s = spots(p1[key])
        lines.append(f"pub(crate) const {const}: [(i32, i32); 4] = [{', '.join(f'({x}, {z})' for x, z in s)}];")
    return "\n".join(lines) + "\n"


def road_tag():
    """O chão que a rua pode trocar: terra, grama e as ruas dos estágios anteriores."""
    import json
    values = ["#minecraft:dirt", "minecraft:dirt_path", "minecraft:farmland"]
    for _, blocks in P.STREET_STYLES.values():
        values += ["minecraft:" + b for b, _ in blocks if "minecraft:" + b not in values and b != "grass_block"]
    values += ["minecraft:" + e for e in P.STREET_EDGE.values() if e and "minecraft:" + e not in values]
    return json.dumps({"replace": False, "values": values}, indent=2) + "\n"


def main():
    check = "--check" in sys.argv
    issues = problems()
    for issue in issues:
        print("PROBLEMA:", issue)
    files, pieces, roads = build()
    for name, lines in files.items():
        for line in lines:
            if line.startswith(("fill ", "setblock ")) or " run fill " in line or " run setblock " in line:
                words = line.split()
                state = words[-1] if " replace " not in line else words[words.index("replace") - 1]
                if not state.startswith("#"):
                    err = check_state(state)
                    if err:
                        issues.append(f"{name}: {err}")
                        print("PROBLEMA:", name, err)
    plan = rust_plan(pieces, roads)
    files_tag = road_tag()
    if check:
        stale = [n for n, l in files.items() if not (FUNCTIONS / n).exists() or (FUNCTIONS / n).read_text(encoding="utf-8") != "\n".join(l) + "\n"]
        if not PLAN_RS.exists() or PLAN_RS.read_text(encoding="utf-8") != plan:
            stale.append(str(PLAN_RS.relative_to(ROOT)))
        if not ROAD_TAG.exists() or ROAD_TAG.read_text(encoding="utf-8") != files_tag:
            stale.append(str(ROAD_TAG.relative_to(ROOT)))
        if stale:
            print("Desatualizado (rode tools/vila/build_village.py):", ", ".join(stale[:8]))
        sys.exit(1 if stale or issues else 0)
    # limpa as funções antigas da vila (menos a de boas-vindas, que é de outra ferramenta)
    for old in list(FUNCTIONS.glob("*.mcfunction")) + list(FUNCTIONS.glob("ruas/*.mcfunction")) + list(FUNCTIONS.glob("verde/*.mcfunction")):
        if old.name != "welcome.mcfunction" and old.relative_to(FUNCTIONS).as_posix() not in files:
            old.unlink()
    for name, lines in files.items():
        path = FUNCTIONS / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    PLAN_RS.write_text(plan, encoding="utf-8")
    ROAD_TAG.parent.mkdir(parents=True, exist_ok=True)
    ROAD_TAG.write_text(files_tag, encoding="utf-8")
    total = sum(len(l) for l in files.values())
    print(f"Vila: {len(pieces)} peças, {len(roads)} pedaços de rua, {total} comandos; {len(issues)} problemas.")
    sys.exit(1 if issues else 0)


if __name__ == "__main__":
    main()
