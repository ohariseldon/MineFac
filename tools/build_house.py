"""Casa do fundador: gera `function/home/build.mcfunction` a partir da planta abaixo.

A casa segue o estilo das casas da cidade (fundação de pedregulho, cantos de tronco, paredes de
tábua de carvalho, telhado de duas águas com chaminé) e já vem mobiliada, em dois ambientes:
- sala com cozinha (oeste): o Lar das obras no lugar da lareira, sofá, tapete, estante, mesa com
  cadeiras, defumador, caldeirão, barril, bancada e o mural de 2x2 mapas na parede oeste;
- escritório com quarto (leste): o centro de controle (painel), atril, estante, baú e a cama ao
  lado do painel, com criado-mudo e vela;
- varanda coberta com bancos virados para a praça, lanternas no parapeito e jardim florido.

O Rust roda a função uma vez por mundo com `execute positioned X Y Z run function veio:home/build`;
(X, Y, Z) é o canto noroeste da planta, com Y no nível do piso. As posições que o Rust usa
(painel, cama, mural) estão em `native/api/src/home/mod.rs` e são conferidas por teste contra o
arquivo gerado.

Uso: python tools/build_house.py
"""
from pathlib import Path

root = Path(__file__).resolve().parents[1]
OUT = root / "content/data/veio/function/home/build.mcfunction"

# Planta: 15 x 14 colunas (x para leste, z para o sul), uma camada por altura a partir do piso.
# Paredes em x 1..13 e z 1..10; a parede interna em x = 9 separa a sala com cozinha (oeste) do
# escritório com quarto (leste). Varanda coberta em z 11..12 e jardim em z 13. A porta fica no
# sul, em x = 7, de frente para a praça. "." não coloca nada (o volume é limpo antes).
LAYERS = [
    [  # 0: piso, varanda e jardim
        "ggggggggggggggg",
        "gCCCCCCCCCCCCCg",
        "gCSSSSSSSSSSSCg",
        "gCSSSSSSSSSSSCg",
        "gCSSSSSSSSSSSCg",
        "gCSSSSSSSSSSSCg",
        "gCSSSSSSSSSSSCg",
        "gCSSSSSSSSSSSCg",
        "gCSSSSSSSSSSSCg",
        "gCSSSSSSSSSSSCg",
        "gCCCCCCCCCCCCCg",
        "gSSSSSSSSSSSSSg",
        "gSSSSSSSSSSSSSg",
        "gggggggqggggggg",
    ],
    [  # 1: móveis
        "...............",
        ".LPPBPPPPLPPPL.",
        ".Pk.F...yPk.cP.",
        ".P.rrr...P...P.",
        ".P.rrr...PE..P.",
        ".Pannnb..P..KP.",
        ".P...........P.",
        ".PhTj....P..XP.",
        ".PswO....P..eP.",
        ".PQQQ....P.OfP.",
        ".LPPPPPdPLPPPL.",
        ".TVV.......VVT.",
        ".TTTTTT.TTTTTT.",
        ".YZY.......ZYZ.",
    ],
    [  # 2: janelas
        "...............",
        ".LP-BPP-PLP-PL.",
        ".Pk......Pk..P.",
        ".|.......P...|.",
        ".P.......P...P.",
        ".P.......P...P.",
        ".P...........P.",
        ".|.p.....P...|.",
        ".P.......P...P.",
        ".P.c.....P.c.P.",
        ".LP-P-PuPLP-PL.",
        ".T...........T.",
        ".T.l.......l.T.",
        "...............",
    ],
    [  # 3: lanternas penduradas
        "...............",
        ".LPPBPPPPLPPPL.",
        ".P.......P...P.",
        ".P.......P.H.P.",
        ".P.......P...P.",
        ".P...H...P...P.",
        ".P.......P...P.",
        ".P.......P...P.",
        ".P.......P...P.",
        ".P.......P...P.",
        ".LPPPPPPPLPPPL.",
        ".T...........T.",
        ".T...........T.",
        "...............",
    ],
    [  # 4: forro, beirais e telhado da varanda
        "NNNNNNNNNNNNNNN",
        ".SSSBSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        ".SSSSSSSSSSSSS.",
        "MMMMMMMMMMMMMMM",
        "GGGGGGGGGGGGGGG",
        "...............",
    ],
] + [
    # 5 a 9: as duas águas do telhado, com a empena de tábuas e a chaminé
    [
        ("NNNNNNNNNNNNNNN" if z == level - 4 else
         "MMMMMMMMMMMMMMM" if z == 15 - level else
         ".P...........P." if level - 4 < z < 15 - level else
         "...............")
        if not (z == 1 and level <= 7) else
        ("NNNNBNNNNNNNNNN" if level == 5 else "....B.........." if level == 6 else "....A..........")
        for z in range(14)
    ]
    for level in range(5, 10)
] + [
    [  # 10: cumeeira
        "..............." if z not in (5, 6) else "RRRRRRRRRRRRRRR" for z in range(14)
    ],
]

BLOCKS = {
    "g": "minecraft:grass_block",
    "q": "minecraft:dirt_path",
    "C": "minecraft:cobblestone",
    "S": "minecraft:spruce_planks",
    "L": "minecraft:oak_log",
    "P": "minecraft:oak_planks",
    "B": "minecraft:bricks",
    # A lareira da casa é o Lar das obras (`veio:home`): lareira, despensa e melhorias da casa.
    "F": "veio:home[facing=south]",
    "k": "minecraft:bookshelf",
    "y": "minecraft:potted_red_tulip",
    "r": "minecraft:red_carpet",
    # Sofá: assento de escada com encosto para o sul (de frente para a lareira), braços de alçapão.
    "n": "minecraft:spruce_stairs[facing=south,half=bottom]",
    "a": "minecraft:spruce_trapdoor[facing=west,open=true,half=bottom]",
    "b": "minecraft:spruce_trapdoor[facing=east,open=true,half=bottom]",
    "K": "veio:control_center[facing=west]",
    "f": "minecraft:red_bed[facing=south,part=foot]",
    "e": "minecraft:red_bed[facing=south,part=head]",
    "X": "minecraft:chest[facing=west]",
    "O": "minecraft:barrel[facing=up]",
    "c": "minecraft:candle[candles=3,lit=true]",
    "h": "minecraft:oak_stairs[facing=west,half=bottom]",
    "j": "minecraft:oak_stairs[facing=east,half=bottom]",
    "T": "minecraft:oak_fence",
    "p": "minecraft:oak_pressure_plate",
    "d": "minecraft:oak_door[facing=north,half=lower,hinge=left]",
    "u": "minecraft:oak_door[facing=north,half=upper,hinge=left]",
    "-": "minecraft:glass_pane[east=true,west=true]",
    "|": "minecraft:glass_pane[north=true,south=true]",
    "!": "minecraft:oak_fence",
    "l": "minecraft:lantern[hanging=false]",
    "H": "minecraft:lantern[hanging=true]",
    "N": "minecraft:oak_stairs[facing=south,half=bottom]",
    "M": "minecraft:oak_stairs[facing=north,half=bottom]",
    "R": "minecraft:oak_slab[type=bottom]",
    # Varanda: bancos virados para a praça, telhado de meia laje e jardim.
    "V": "minecraft:spruce_stairs[facing=north,half=bottom]",
    "G": "minecraft:spruce_slab[type=bottom]",
    "Y": "minecraft:poppy",
    "Z": "minecraft:dandelion",
    # Escritório e cozinha.
    "E": "minecraft:lectern[facing=east]",
    "s": "minecraft:smoker[facing=east]",
    "w": "minecraft:water_cauldron[level=3]",
    "Q": "minecraft:smooth_stone_slab[type=top]",
    "A": "minecraft:campfire[lit=true,signal_fire=false]",
}

WIDTH, DEPTH, HEIGHT = 15, 14, len(LAYERS)
# Mural: quatro molduras na parede oeste (x = 2, encostadas na parede x = 1), viradas para leste.
# Quem olha para oeste tem o sul à esquerda: a moldura de z maior mostra a metade oeste do mapa.
MURAL_ID = 1_970_000
MURAL = [  # (dy, dz, índice do mapa: linha * 2 + coluna)
    (3, 5, 0), (3, 4, 1),
    (2, 5, 2), (2, 4, 3),
]


def runs(row):
    """Trechos seguidos do mesmo bloco numa linha: (x inicial, x final, símbolo)."""
    out, start = [], 0
    for x in range(1, len(row) + 1):
        if x == len(row) or row[x] != row[start]:
            if row[start] != ".":
                out.append((start, x - 1, row[start]))
            start = x
    return out


def commands():
    lines = [
        "# Gerado por tools/build_house.py. Não edite à mão.",
        "# Casa do fundador: rode com `execute positioned <canto noroeste, nível do piso> run function veio:home/build`.",
        f"kill @e[type=minecraft:item_frame,tag=veio_mural,dx={WIDTH},dy={HEIGHT},dz={DEPTH}]",
        f"fill ~ ~1 ~ ~{WIDTH - 1} ~{HEIGHT + 1} ~{DEPTH - 1} minecraft:air",
        # Fundação: tapa buracos e água embaixo da casa e do terreiro.
        f"fill ~ ~-4 ~ ~{WIDTH - 1} ~-1 ~{DEPTH - 1} minecraft:dirt replace minecraft:air",
        f"fill ~ ~-4 ~ ~{WIDTH - 1} ~-1 ~{DEPTH - 1} minecraft:dirt replace minecraft:water",
    ]
    for y, layer in enumerate(LAYERS):
        assert len(layer) == DEPTH, f"camada {y}: {len(layer)} linhas"
        for z, row in enumerate(layer):
            assert len(row) == WIDTH, f"camada {y}, linha {z}: {len(row)} colunas"
            for x0, x1, symbol in runs(row):
                block = BLOCKS[symbol]
                if x0 == x1:
                    lines.append(f"setblock ~{x0} ~{y} ~{z} {block}")
                else:
                    lines.append(f"fill ~{x0} ~{y} ~{z} ~{x1} ~{y} ~{z} {block}")
    for dy, dz, index in MURAL:
        item = f'{{id:"minecraft:filled_map",count:1,components:{{"minecraft:map_id":{MURAL_ID + index}}}}}'
        lines.append(
            f"summon minecraft:item_frame ~2.5 ~{dy}.5 ~{dz}.5 "
            f'{{Facing:5b,Fixed:1b,Invulnerable:1b,Tags:["veio_mural"],Item:{item}}}'
        )
    return lines


def main():
    lines = commands()
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Casa do fundador: {len(lines)} comandos em {OUT.relative_to(root)}")


if __name__ == "__main__":
    main()
