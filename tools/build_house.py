"""Casa do fundador: gera `function/home/build.mcfunction` a partir da planta abaixo.

A casa segue o estilo das casas de planície da vila (fundação de pedregulho, cantos de tronco,
paredes de tábua de carvalho, telhado de escadas) e já vem mobiliada: centro de controle (painel),
lareira, sofá, tapete, mesa com cadeiras, estante, cama ao lado do painel, criado-mudo com velas,
lanternas e um mural de 2x2 mapas na parede oeste.

O Rust roda a função uma vez por mundo com `execute positioned X Y Z run function veio:home/build`;
(X, Y, Z) é o canto noroeste da planta, com Y no nível do piso. As posições que o Rust usa
(painel, cama, mural) estão em `native/api/src/home/mod.rs` e são conferidas por teste contra o
arquivo gerado.

Uso: python tools/build_house.py
"""
from pathlib import Path

root = Path(__file__).resolve().parents[1]
OUT = root / "content/data/veio/function/home/build.mcfunction"

# Planta: 13 x 11 colunas (x para leste, z para o sul), uma camada por altura a partir do piso.
# Paredes em x 1..11 e z 1..9; a borda é o beiral do telhado. "." não coloca nada (o volume é
# limpo antes). A porta fica no sul, em x = 6.
LAYERS = [
    [  # 0: piso
        "ggggggggggggg",
        "gCCCCCCCCCCCg",
        "gCSSSSSSSSSCg",
        "gCSSSSSSSSSCg",
        "gCSSSSSSSSSCg",
        "gCSSSSSSSSSCg",
        "gCSSSSSSSSSCg",
        "gCSSSSSSSSSCg",
        "gCSSSSSSSSSCg",
        "gCCCCCCCCCCCg",
        "ggggggqgggggg",
    ],
    [  # 1: móveis
        ".............",
        ".LPPPPBPPPPL.",
        ".Pk..BFB...P.",
        ".P...rrr...P.",
        ".P...rrr...P.",
        ".P..annnb.KP.",
        ".P.........P.",
        ".P.hTj....fP.",
        ".PX......OeP.",
        ".LPPPPdPPPPL.",
        ".....!.......",
    ],
    [  # 2
        ".............",
        ".LP-PPBPP-PL.",
        ".Pk..B.B...P.",
        ".P.........|.",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        ".|..p......P.",
        ".P.......c.P.",
        ".LP--PuP--PL.",
        ".....l.......",
    ],
    [  # 3
        ".............",
        ".LP-PPBPP-PL.",
        ".Py..BBB...P.",
        ".P.........|.",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        ".|.........P.",
        ".P.........P.",
        ".LP--PPP--PL.",
        ".............",
    ],
    [  # 4: lanternas penduradas, meia luz
        ".............",
        ".LPPPPBPPPPL.",
        ".P....B....P.",
        ".P.........P.",
        ".P.........P.",
        ".P.......H.P.",
        ".P.........P.",
        ".P..H......P.",
        ".P.........P.",
        ".LPPPPPPPPPL.",
        ".............",
    ],
    [  # 5: forro
        "NNNNNNNNNNNNN",
        ".SSSSSBSSSSS.",
        ".SSSSSSSSSSS.",
        ".SSSSSSSSSSS.",
        ".SSSSSSSSSSS.",
        ".SSSSSSSSSSS.",
        ".SSSSSSSSSSS.",
        ".SSSSSSSSSSS.",
        ".SSSSSSSSSSS.",
        ".SSSSSSSSSSS.",
        "MMMMMMMMMMMMM",
    ],
    [  # 6: telhado
        ".............",
        "NNNNNNBNNNNNN",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        "MMMMMMMMMMMMM",
        ".............",
    ],
    [  # 7
        ".............",
        "......B......",
        "NNNNNNNNNNNNN",
        ".P.........P.",
        ".P.........P.",
        ".|.........|.",
        ".P.........P.",
        ".P.........P.",
        "MMMMMMMMMMMMM",
        ".............",
        ".............",
    ],
    [  # 8
        ".............",
        "......B......",
        ".............",
        "NNNNNNNNNNNNN",
        ".P.........P.",
        ".P.........P.",
        ".P.........P.",
        "MMMMMMMMMMMMM",
        ".............",
        ".............",
        ".............",
    ],
    [  # 9
        ".............",
        "......B......",
        ".............",
        ".............",
        "NNNNNNNNNNNNN",
        ".P.........P.",
        "MMMMMMMMMMMMM",
        ".............",
        ".............",
        ".............",
        ".............",
    ],
    [  # 10: cumeeira
        ".............",
        ".............",
        ".............",
        ".............",
        ".............",
        "RRRRRRRRRRRRR",
        ".............",
        ".............",
        ".............",
        ".............",
        ".............",
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
    "F": "minecraft:campfire[lit=true,facing=south,signal_fire=false]",
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
    "X": "minecraft:chest[facing=east]",
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
}

WIDTH, DEPTH, HEIGHT = 13, 11, len(LAYERS)
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
