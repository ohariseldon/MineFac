"""Gera os registros e modelos de blocos e itens do Veio a partir de uma lista única.

Saídas: modelos de bloco e item, blockstates, definições de item do cliente, registros
`data/veio/horizon/{block,item,menu}`, tabelas de saque, a tag de picareta e as abas criativas.
Rode `python tools/build_content.py` ao adicionar uma máquina ou mudar um modelo.
Os modelos usam texturas do próprio Minecraft (resolvidas pela instalação do jogador)
e as texturas 16x16 de `tools/build_textures.py`.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "content"
A, D = ROOT / "assets/veio", ROOT / "data/veio"

# Lados de cada nível de máquina, com texturas vanilla.
TIER_SIDE = {
    "copper": "minecraft:block/cut_copper",
    "iron": "minecraft:block/iron_block",
    "steel": "minecraft:block/polished_deepslate",
    "wood": "minecraft:block/barrel_side",
}
TIER_TOP = {
    "copper": "veio:block/machine_top_copper",
    "iron": "veio:block/machine_top_iron",
    "steel": "veio:block/machine_top_steel",
    "wood": "minecraft:block/barrel_top",
}

# nome: (tipo de modelo, nível, slots do menu, saídas do menu, dureza)
BLOCKS = {
    "drill": ("machine", "copper", 1, [0], 1.5),
    "press": ("machine", "copper", 2, [1], 1.5),
    "assembler": ("machine", "copper", 2, [1], 1.5),
    "generator": ("generator", "copper", 1, [], 1.5),
    "contract_hub": ("machine", "wood", 4, [2, 3], 1.5),
    "cutter": ("machine", "iron", 2, [1], 1.5),
    "integrator": ("machine", "iron", 3, [2], 1.5),
    "crusher": ("machine", "iron", 2, [1], 1.5),
    "alloy_furnace": ("machine", "iron", 3, [2], 2.0),
    "harvester": ("machine", "iron", 4, [1, 2, 3], 1.5),
    "electronics": ("machine", "steel", 3, [2], 2.0),
    "geothermal": ("machine", "steel", 2, [1], 2.0),
    "drill_mk2": ("machine", "steel", 1, [0], 2.0),
    "tesla": ("tesla", "steel", 1, [0], 2.5),
    "belt": ("belt", None, 1, [], 1.5),
    "fast_belt": ("belt", None, 1, [], 1.5),
    "splitter": ("splitter", None, 1, [], 1.5),
    "cable": ("cable", None, 0, [], 1.0),
    "solar": ("solar", None, 1, [0], 2.5),
    "battery": ("battery", None, 1, [0], 2.5),
}

# Itens sem bloco: modelo vanilla de referência para os componentes de dados e tamanho da pilha.
ITEMS = {
    "copper_plate": ("minecraft:copper_ingot", 64),
    "iron_plate": ("minecraft:iron_ingot", 64),
    "steel_plate": ("minecraft:iron_ingot", None),
    "gear": ("minecraft:iron_nugget", 64),
    "wire": ("minecraft:copper_ingot", None),
    "circuit": ("minecraft:redstone", None),
    "silicon": ("minecraft:quartz", None),
    "frame": ("minecraft:quartz", 64),
    "module": ("minecraft:echo_shard", 64),
    # A chave copia a picareta de ferro: também quebra as máquinas rapidamente.
    "wrench": ("minecraft:iron_pickaxe", None),
}

TABS = {
    "industry": ("veio:press", ["drill", "drill_mk2", "press", "assembler", "cutter", "integrator",
        "crusher", "alloy_furnace", "electronics", "contract_hub", "wrench", "copper_plate", "iron_plate",
        "steel_plate", "gear", "wire", "frame", "silicon", "circuit", "module"]),
    "power": ("veio:generator", ["generator", "geothermal", "solar", "battery", "cable",
        "minecraft:coal", "minecraft:lava_bucket"]),
    "logistics": ("veio:belt", ["belt", "fast_belt", "splitter", "wrench"]),
    "world": ("veio:harvester", ["harvester", "tesla", "minecraft:bone_meal"]),
}

FACES = ("down", "up", "north", "south", "west", "east")

def box(start, end, textures, uv=None, **extra):
    """Elemento com as faces indicadas; `textures` aceita uma chave para todas ou um dicionário.
    `uv` fixa a área da textura de lados (`side`) e topo (`up`) em peças acima de 16 px,
    onde a área automática sairia da textura."""
    faces = {}
    for face in FACES:
        texture = textures if isinstance(textures, str) else textures.get(face, textures.get("*"))
        if texture:
            faces[face] = {"texture": texture}
            if uv:
                faces[face]["uv"] = uv["up"] if face in ("up", "down") else uv["side"]
    element = {"from": start, "to": end, "faces": faces}
    element.update(extra)
    return element

def plinth():
    return box([0, 0, 0], [16, 2, 16], {"up": "#base", "down": "#base", "*": "#base_side"})

def machine(name, tier):
    textures = {
        "base": "minecraft:block/smooth_stone", "base_side": "minecraft:block/smooth_stone_slab_side",
        "side": TIER_SIDE[tier], "top": TIER_TOP[tier], "front": f"veio:block/{name}_front",
        "port": "veio:block/port", "particle": TIER_SIDE[tier],
    }
    elements = [
        plinth(),
        # Frente ao sul (voltada ao jogador ao colocar); saída ao norte.
        box([1, 2, 1], [15, 14, 15], {"north": "#port", "south": "#front", "*": "#side"}),
        box([0, 14, 0], [16, 16, 16], {"up": "#top", "*": "#side"}),
        # Bico de saída, para ler a direção de longe.
        box([6, 5, 0], [10, 9, 1], {"north": "#port", "*": "#side"}),
    ]
    return textures, elements

def model(kind, name, tier):
    if kind == "machine":
        return machine(name, tier)
    if kind == "generator":
        textures, elements = machine(name, tier)
        textures["chimney"] = "minecraft:block/bricks"
        elements.append(box([10, 16, 10], [13, 20, 13], {"up": "#chimney_top", "*": "#chimney"},
                            uv={"side": [0, 0, 3, 4], "up": [0, 0, 3, 3]}))
        textures["chimney_top"] = "minecraft:block/furnace_top"
        return textures, elements
    if kind == "tesla":
        textures = {
            "base": "minecraft:block/smooth_stone", "base_side": "minecraft:block/smooth_stone_slab_side",
            "body": TIER_SIDE["steel"], "top": TIER_TOP["steel"], "coil": "veio:block/tesla_coil",
            "crown": "minecraft:block/iron_block", "tip": "minecraft:block/copper_block",
            "particle": "veio:block/tesla_coil",
        }
        return textures, [
            plinth(),
            box([2, 2, 2], [14, 6, 14], {"up": "#top", "*": "#body"}),
            box([5, 6, 5], [11, 13, 11], {"up": "#tip", "down": "#tip", "*": "#coil"}),
            box([4, 13, 4], [12, 15, 12], "#crown"),
            box([6, 15, 6], [10, 16, 10], "#tip"),
        ]
    if kind in ("belt", "splitter"):
        rail = "veio:block/fast_belt_rail" if name == "fast_belt" else "veio:block/belt_rail"
        textures = {
            "base": "minecraft:block/smooth_stone", "base_side": "minecraft:block/smooth_stone_slab_side",
            "belt": "veio:block/belt", "rail": rail, "iron": "minecraft:block/iron_block",
            "particle": "veio:block/belt",
        }
        elements = [
            box([0, 0, 0], [16, 2, 16], {"up": "#base", "down": "#base", "*": "#base_side"}),
            box([0, 2, 0], [2, 4, 16], {"up": "#rail", "*": "#iron"}),
            box([14, 2, 0], [16, 4, 16], {"up": "#rail", "*": "#iron"}),
            box([2, 2, 0], [14, 3, 16], {"up": "#belt", "*": "#belt"}),
        ]
        if kind == "splitter":
            textures["divider"] = "veio:block/splitter_top"
            elements.append(box([5, 3, 5], [11, 6, 11], {"up": "#divider", "*": "#iron"}))
        return textures, elements
    if kind == "cable":
        textures = {"wire": "minecraft:block/black_wool", "joint": "minecraft:block/copper_block",
                    "particle": "minecraft:block/black_wool"}
        return textures, [
            box([6, 0, 0], [10, 3, 16], "#wire"),
            box([0, 0, 6], [16, 3, 10], "#wire"),
            box([5, 0, 5], [11, 4, 11], "#joint"),
        ]
    if kind == "solar":
        textures = {"frame": "minecraft:block/iron_block", "cells": "minecraft:block/daylight_detector_top",
                    "edge": "minecraft:block/daylight_detector_side", "particle": "minecraft:block/daylight_detector_top"}
        legs = [box([x, 0, z], [x + 2, 3, z + 2], "#frame") for x in (2, 12) for z in (2, 12)]
        return textures, legs + [
            box([0, 3, 0], [16, 4, 16], "#frame"),
            box([1, 4, 1], [15, 5.5, 15], {"up": "#cells", "*": "#edge"}),
        ]
    if kind == "battery":
        textures = {"side": "veio:block/battery_side", "cap": "minecraft:block/polished_deepslate",
                    "terminal": "minecraft:block/copper_block", "particle": "veio:block/battery_side"}
        return textures, [
            box([1, 0, 1], [15, 16, 15], {"up": "#cap", "down": "#cap", "*": "#side"}),
            box([3, 16, 7], [5, 16.5, 9], "#terminal", uv={"side": [0, 0, 2, 1], "up": [0, 0, 2, 2]}),
            box([11, 16, 7], [13, 16.5, 9], "#terminal", uv={"side": [0, 0, 2, 1], "up": [0, 0, 2, 2]}),
        ]
    raise ValueError(kind)

SHAPES = {
    "machine": [[0, 0, 0, 1, 1, 1]],
    "generator": [[0, 0, 0, 1, 1, 1]],
    "tesla": [[0, 0, 0, 1, 1, 1]],
    "belt": [[0, 0, 0, 1, 0.25, 1]],
    "splitter": [[0, 0, 0, 1, 0.375, 1]],
    "cable": [[0, 0, 0.375, 1, 0.25, 0.625], [0.375, 0, 0, 0.625, 0.25, 1]],
    "solar": [[0, 0, 0, 1, 0.34375, 1]],
    "battery": [[0.0625, 0, 0.0625, 0.9375, 1.025, 0.9375]],
}

def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

def main():
    for name, (kind, tier, slots, results, hardness) in BLOCKS.items():
        textures, elements = model(kind, name, tier)
        write(A / f"models/block/{name}.json", {"parent": "minecraft:block/block", "textures": textures, "elements": elements})
        write(A / f"blockstates/{name}.json", {"variants": {
            "facing=south": {"model": f"veio:block/{name}"},
            "facing=west": {"model": f"veio:block/{name}", "y": 90},
            "facing=north": {"model": f"veio:block/{name}", "y": 180},
            "facing=east": {"model": f"veio:block/{name}", "y": 270},
        }})
        write(A / f"items/{name}.json", {"model": {"type": "minecraft:model", "model": f"veio:block/{name}"}})
        template = "minecraft:black_glazed_terracotta" if kind == "cable" else "minecraft:stonecutter"
        shape = {"default": SHAPES[kind]}
        write(D / f"horizon/block/{name}.json", {"template": template, "item": True,
            "properties": {"hardness": hardness, "occludes": False},
            "shapes": shape, "collision_shapes": shape})
        if slots:
            write(D / f"horizon/menu/{name}.json", {"slots": slots, "data_slots": 10, "results": results})
        write(D / f"loot_table/blocks/{name}.json", {"type": "minecraft:block", "pools": [{"rolls": 1,
            "entries": [{"type": "minecraft:item", "name": f"veio:{name}"}],
            "conditions": [{"condition": "minecraft:survives_explosion"}]}],
            "random_sequence": f"veio:blocks/{name}"})
    for name, (template, stack) in ITEMS.items():
        parent = "minecraft:item/handheld" if name == "wrench" else "minecraft:item/generated"
        write(A / f"models/item/{name}.json", {"parent": parent, "textures": {"layer0": f"veio:item/{name}"}})
        write(A / f"items/{name}.json", {"model": {"type": "minecraft:model", "model": f"veio:item/{name}"}})
        entry = {"template": template}
        if stack:
            entry["max_stack_size"] = stack
        write(D / f"horizon/item/{name}.json", entry)
    write(ROOT / "data/minecraft/tags/block/mineable/pickaxe.json", {"values": [f"veio:{name}" for name in BLOCKS]})
    for tab, (icon, members) in TABS.items():
        write(D / f"horizon/creative_tab/{tab}.json", {"title": f"itemGroup.veio.{tab}", "icon": icon,
            "items": [name if ":" in name else f"veio:{name}" for name in members]})
    print(f"{len(BLOCKS)} blocos e {len(ITEMS)} itens gerados")

if __name__ == "__main__":
    main()
