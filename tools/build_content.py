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
    "merger": ("merger", None, 2, [], 1.5),
    "inserter": ("inserter", None, 1, [], 1.5),
    "fast_inserter": ("inserter", None, 1, [], 2.0),
    "long_inserter": ("inserter", None, 1, [], 1.5),
    "cable": ("cable", None, 0, [], 1.0),
    "pole": ("pole", None, 0, [], 1.0),
    # Produção (0.9): extração, processamento, energia, lavoura e comida.
    "drill_mk3": ("machine", "steel", 1, [0], 2.5),
    "electric_furnace": ("machine", "copper", 2, [1], 1.5),
    "mill": ("machine", "copper", 2, [1], 1.5),
    "kitchen": ("machine", "iron", 3, [2], 1.5),
    "pantry": ("machine", "wood", 2, [1], 1.5),
    "biomass_generator": ("generator", "copper", 1, [], 1.5),
    "steam_engine": ("generator", "steel", 1, [], 2.0),
    "water_wheel": ("water_wheel", None, 1, [0], 1.5),
    "wind_turbine": ("wind_turbine", None, 1, [0], 2.0),
    "grow_lamp": ("grow_lamp", None, 1, [0], 1.0),
    "planter": ("machine", "iron", 1, [], 1.5),
    "tree_farm": ("machine", "iron", 4, [1, 2, 3], 2.0),
    "ranch": ("machine", "wood", 5, [2, 3, 4], 1.5),
    "steel_pole": ("steel_pole", None, 0, [], 2.0),
    # Trabalhadores: posto onde um aldeão contratado trabalha.
    "farm_post": ("machine", "wood", 4, [1, 2, 3], 1.5),
    "hauler_post": ("machine", "iron", 7, [4, 5, 6], 1.5),
    "foreman_desk": ("machine", "wood", 4, [], 1.5),
    "shepherd_post": ("machine", "wood", 5, [2, 3, 4], 1.5),
    # Zonas, armazenagem e gestão da vila.
    "field_marker": ("marker", None, 1, [], 1.0),
    "pen_marker": ("marker", None, 1, [], 1.0),
    "barn": ("machine", "wood", 27, [], 2.0),
    "silo": ("machine", "iron", 18, [], 2.0),
    "control_center": ("machine", "steel", 4, [], 2.0),
    "barracks": ("machine", "iron", 2, [], 2.0),
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
    "repair_kit": ("minecraft:iron_ingot", 64),
    "speed_module": ("minecraft:echo_shard", 16),
    "efficiency_module": ("minecraft:echo_shard", 16),
    "productivity_module": ("minecraft:echo_shard", 16),
    "flour": ("minecraft:sugar", 64),
    "biomass": ("minecraft:wheat", 64),
}

TABS = {
    "industry": ("veio:press", ["drill", "drill_mk2", "drill_mk3", "press", "assembler", "cutter", "integrator",
        "crusher", "alloy_furnace", "electronics", "electric_furnace", "contract_hub", "wrench", "copper_plate", "iron_plate",
        "steel_plate", "gear", "wire", "frame", "silicon", "circuit", "module", "speed_module",
        "efficiency_module", "productivity_module"]),
    "power": ("veio:generator", ["generator", "biomass_generator", "water_wheel", "wind_turbine", "steam_engine",
        "geothermal", "solar", "battery", "pole", "steel_pole", "cable",
        "minecraft:coal", "minecraft:lava_bucket"]),
    "logistics": ("veio:belt", ["belt", "fast_belt", "splitter", "merger", "inserter", "fast_inserter", "long_inserter", "wrench"]),
    "world": ("veio:harvester", ["harvester", "planter", "grow_lamp", "tree_farm", "ranch", "farm_post", "shepherd_post", "hauler_post", "foreman_desk",
        "field_marker", "pen_marker", "barn", "silo", "control_center", "mill", "kitchen",
        "pantry", "flour", "biomass", "tesla", "barracks", "repair_kit", "minecraft:bone_meal"]),
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
    if kind == "inserter":
        motor = {"fast_inserter": "minecraft:block/prismarine_bricks", "long_inserter": "minecraft:block/red_terracotta"}.get(name, "minecraft:block/cut_copper")
        return {"iron": "minecraft:block/iron_block", "base": "minecraft:block/smooth_stone",
                "motor": motor, "port": "veio:block/port", "particle": motor}, [
            box([0, 0, 0], [16, 2, 16], "#base"),
            box([2, 2, 4], [14, 4, 12], "#iron"),
            box([5, 4, 5], [11, 8, 11], "#motor"),
            box([6, 2, 0], [10, 4, 4], {"up": "#port", "*": "#motor"}),
            box([6, 2, 12], [10, 3, 16], "#iron"),
        ]
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
    if kind in ("belt", "splitter", "merger"):
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
        if kind == "merger":
            textures["junction"] = "veio:block/merger_top"
            textures["lane_b"] = "veio:block/fast_belt_rail"
            # Open both sides. The tapered forward nose marks the single outlet.
            elements = [elements[0], elements[3],
                box([0, 2, 5], [16, 3.25, 11], {"up": "#belt", "*": "#iron"}),
                box([4, 3.25, 4], [12, 5, 12], {"up": "#junction", "*": "#iron"}),
                box([6, 3, 0], [10, 4, 4], {"up": "#rail", "*": "#iron"}),
                box([0, 3.25, 5], [2, 3.75, 11], {"up": "#rail", "*": "#iron"}),
                box([14, 3.25, 5], [16, 3.75, 11], {"up": "#lane_b", "*": "#iron"}),
                box([0, 2, 14], [16, 5, 16], {"up": "#rail", "*": "#iron"})]
        return textures, elements
    if kind == "marker":
        sign = "minecraft:block/brown_wool" if name == "pen_marker" else "minecraft:block/lime_wool"
        textures = {"wood": "minecraft:block/stripped_oak_log", "sign": sign, "particle": "minecraft:block/stripped_oak_log"}
        return textures, [
            box([7, 0, 7], [9, 14, 9], "#wood", uv={"side": [7, 2, 9, 16], "up": [7, 7, 9, 9]}),
            box([3, 9, 7.5], [13, 15, 8.5], "#sign", uv={"side": [3, 1, 13, 7], "up": [3, 7, 13, 8]}),
        ]
    if kind == "pole":
        textures = {"wood": "minecraft:block/stripped_spruce_log", "glass": "minecraft:block/copper_block",
                    "particle": "minecraft:block/stripped_spruce_log"}
        tall = {"side": [7, 0, 9, 16], "up": [7, 7, 9, 9]}
        return textures, [
            box([7, 0, 7], [9, 22, 9], "#wood", uv=tall),
            box([3, 19, 7.5], [13, 20.5, 8.5], "#wood", uv={"side": [0, 6, 16, 7.5], "up": [0, 7, 16, 8]}),
            box([3.5, 20.5, 7.5], [4.5, 21.5, 8.5], "#glass", uv={"side": [0, 0, 1, 1], "up": [0, 0, 1, 1]}),
            box([11.5, 20.5, 7.5], [12.5, 21.5, 8.5], "#glass", uv={"side": [0, 0, 1, 1], "up": [0, 0, 1, 1]}),
            box([7.5, 22, 7.5], [8.5, 23, 8.5], "#glass", uv={"side": [0, 0, 1, 1], "up": [0, 0, 1, 1]}),
        ]
    if kind == "steel_pole":
        textures = {"base": "minecraft:block/smooth_stone", "base_side": "minecraft:block/smooth_stone_slab_side",
                    "mast": "minecraft:block/iron_block", "arm": "minecraft:block/polished_deepslate",
                    "glass": "minecraft:block/copper_block", "particle": "minecraft:block/iron_block"}
        return textures, [
            box([5, 0, 5], [11, 2, 11], {"up": "#base", "down": "#base", "*": "#base_side"}),
            box([7, 2, 7], [9, 29, 9], "#mast", uv={"side": [7, 0, 9, 16], "up": [7, 7, 9, 9]}),
            box([2, 26, 7.5], [14, 27.5, 8.5], "#arm", uv={"side": [2, 6, 14, 7.5], "up": [2, 7, 14, 8]}),
            box([2.5, 27.5, 7.5], [3.5, 29, 8.5], "#glass", uv={"side": [0, 0, 1, 1.5], "up": [0, 0, 1, 1]}),
            box([12.5, 27.5, 7.5], [13.5, 29, 8.5], "#glass", uv={"side": [0, 0, 1, 1.5], "up": [0, 0, 1, 1]}),
            box([7.5, 29, 7.5], [8.5, 31, 8.5], "#glass", uv={"side": [0, 0, 1, 2], "up": [0, 0, 1, 1]}),
        ]
    if kind == "water_wheel":
        # As pás são desenhadas pelo bloco e giram com a água; o modelo traz o suporte e o eixo.
        textures = {"post": "minecraft:block/stripped_oak_log", "axle": "minecraft:block/oak_log",
                    "particle": "minecraft:block/oak_planks"}
        return textures, [
            box([0, 0, 6], [2, 10, 10], "#post"),
            box([14, 0, 6], [16, 10, 10], "#post"),
            box([2, 7, 7], [14, 9, 9], "#axle"),
        ]
    if kind == "wind_turbine":
        textures = {"mast": "minecraft:block/iron_block", "body": "minecraft:block/polished_deepslate",
                    "hub": "minecraft:block/copper_block", "particle": "minecraft:block/iron_block"}
        return textures, [
            box([7, 0, 7], [9, 24, 9], "#mast", uv={"side": [7, 0, 9, 16], "up": [7, 7, 9, 9]}),
            box([6, 24, 5], [10, 30, 10], "#body", uv={"side": [3, 5, 8, 11], "up": [6, 5, 10, 10]}),
            box([7, 27, 10], [9, 29, 11], "#hub", uv={"side": [0, 0, 2, 2], "up": [0, 0, 2, 1]}),
        ]
    if kind == "grow_lamp":
        textures = {"post": "minecraft:block/iron_block", "glow": "minecraft:block/glowstone",
                    "top": TIER_TOP["steel"], "particle": "minecraft:block/glowstone"}
        return textures, [
            box([7, 0, 7], [9, 12, 9], "#post"),
            box([4, 12, 4], [12, 15, 12], {"up": "#top", "down": "#glow", "*": "#glow"}),
        ]
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
    "inserter": [[0, 0, 0, 1, 0.25, 1], [.3125, .25, .3125, .6875, .625, .6875]],
    "machine": [[0, 0, 0, 1, 1, 1]],
    "generator": [[0, 0, 0, 1, 1, 1]],
    "tesla": [[0, 0, 0, 1, 1, 1]],
    "belt": [[0, 0, 0, 1, 0.25, 1]],
    "splitter": [[0, 0, 0, 1, 0.375, 1]],
    "merger": [[0, 0, 0, 1, 0.3125, 1]],
    "cable": [[0, 0, 0.375, 1, 0.25, 0.625], [0.375, 0, 0, 0.625, 0.25, 1]],
    "solar": [[0, 0, 0, 1, 0.34375, 1]],
    "battery": [[0.0625, 0, 0.0625, 0.9375, 1.025, 0.9375]],
    "pole": [[0.375, 0, 0.375, 0.625, 1.5, 0.625]],
    "steel_pole": [[0.3125, 0, 0.3125, 0.6875, 1.5, 0.6875]],
    "water_wheel": [[0, 0, 0, 1, 1, 1]],
    "wind_turbine": [[0.4375, 0, 0.4375, 0.5625, 1.5, 0.5625]],
    "grow_lamp": [[0.25, 0, 0.25, 0.75, 0.9375, 0.75]],
    "marker": [[0.375, 0, 0.375, 0.625, 0.9375, 0.625]],
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
        # Sem menu, um template sem tela própria: a terracota esmaltada só gira com o jogador.
        template = "minecraft:black_glazed_terracotta" if kind in ("cable", "pole", "steel_pole") else "minecraft:stonecutter"
        shape = {"default": SHAPES[kind]}
        properties = {"hardness": hardness, "occludes": False}
        if kind == "grow_lamp":
            properties["light"] = 15  # A lâmpada ilumina a lavoura à noite.
        write(D / f"horizon/block/{name}.json", {"template": template, "item": True,
            "properties": properties,
            "shapes": shape, "collision_shapes": shape})
        if slots:
            write(D / f"horizon/menu/{name}.json", {"slots": slots, "data_slots": 13, "results": results})
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
    # Saqueadores: zumbis com mais alcance de visão, para acharem a fábrica e os aldeões.
    write(D / "horizon/entity_type/raider.json", {"template": "minecraft:zombie",
        "attributes": {"minecraft:max_health": 24.0, "minecraft:movement_speed": 0.25,
            "minecraft:follow_range": 40.0}})
    # O Bruto (da 5ª onda): lento, duro, quebra máquinas rápido e empurra pouco.
    write(D / "horizon/entity_type/raider_brute.json", {"template": "minecraft:zombie",
        "attributes": {"minecraft:max_health": 40.0, "minecraft:movement_speed": 0.2,
            "minecraft:attack_damage": 6.0, "minecraft:knockback_resistance": 0.6,
            "minecraft:follow_range": 40.0}})
    write(D / "loot_table/entities/raider_brute.json", {"type": "minecraft:entity", "pools": [
        {"rolls": 1, "entries": [{"type": "minecraft:item", "name": "minecraft:iron_ingot",
            "functions": [{"function": "minecraft:set_count", "count": {"type": "minecraft:uniform", "min": 1, "max": 3}}]}]},
        {"rolls": 1, "entries": [{"type": "minecraft:item", "name": "minecraft:emerald",
            "functions": [{"function": "minecraft:set_count", "count": {"type": "minecraft:uniform", "min": 0, "max": 1}}]}]},
    ], "random_sequence": "veio:entities/raider_brute"})
    write(D / "loot_table/entities/raider.json", {"type": "minecraft:entity", "pools": [
        {"rolls": 1, "entries": [{"type": "minecraft:item", "name": "minecraft:rotten_flesh",
            "functions": [{"function": "minecraft:set_count", "count": {"type": "minecraft:uniform", "min": 0, "max": 2}}]}]},
        {"rolls": 1, "entries": [{"type": "minecraft:item", "name": "minecraft:iron_nugget",
            "functions": [{"function": "minecraft:set_count", "count": {"type": "minecraft:uniform", "min": 1, "max": 3}}]}]},
    ], "random_sequence": "veio:entities/raider"})
    write(ROOT / "data/minecraft/tags/block/mineable/pickaxe.json", {"values": [f"veio:{name}" for name in BLOCKS]})
    for tab, (icon, members) in TABS.items():
        write(D / f"horizon/creative_tab/{tab}.json", {"title": f"itemGroup.veio.{tab}", "icon": icon,
            "items": [name if ":" in name else f"veio:{name}" for name in members]})
    print(f"{len(BLOCKS)} blocos e {len(ITEMS)} itens gerados")

if __name__ == "__main__":
    main()
    from build_belts import build
    build(ROOT, model, box, write)
