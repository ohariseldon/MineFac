"""Distinct industrial silhouettes. Coordinates are pixels in one placement cell.

The same description generates model, oriented hitboxes and native lamp/motion
anchors, so changing a machine cannot leave its light hovering over an old cube.
"""
from pathlib import Path

GROUPS = {
    "drill": "rig", "drill_mk2": "rig", "drill_mk3": "rig", "press": "press",
    "assembler": "assembly", "integrator": "assembly", "cutter": "saw", "crusher": "crusher",
    "alloy_furnace": "oven", "electric_furnace": "oven", "electronics": "electronics",
    "mill": "mill", "kitchen": "kitchen", "pantry": "counter", "contract_hub": "desk",
    "generator": "boiler", "biomass_generator": "boiler", "steam_engine": "boiler", "geothermal": "tanks",
    "planter": "planter", "harvester": "harvester", "tree_farm": "saw", "ranch": "ranch",
    "farm_post": "desk", "hauler_post": "loading", "foreman_desk": "desk", "shepherd_post": "ranch",
    "control_center": "console", "freight_station": "loading", "drone_port": "landing",
    "caravan_station": "hitch", "barn": "shelves", "silo": "silo", "battery": "cells",
}


def form(name, tier, box):
    family = GROUPS.get(name)
    if family is None:
        return None
    side = {"copper": "cut_copper", "iron": "iron_block", "steel": "polished_deepslate", "wood": "barrel_side"}.get(tier, "iron_block")
    accent = ("green_terracotta" if family in ("planter", "harvester", "ranch", "mill")
              else "blue_terracotta" if family in ("electronics", "console", "landing")
              else "red_terracotta" if family in ("oven", "boiler", "tanks") else "cut_copper")
    if name == "biomass_generator": accent = "green_terracotta"
    textures = {"base": "minecraft:block/smooth_stone", "base_side": "minecraft:block/smooth_stone_slab_side",
                "body": "minecraft:block/" + side, "frame": "minecraft:block/polished_deepslate",
                "metal": "minecraft:block/iron_block", "accent": "minecraft:block/" + accent,
                "wood": "minecraft:block/stripped_spruce_log", "glass": "minecraft:block/tinted_glass",
                "front": f"veio:block/{name}_front" if name != "battery" else "veio:block/battery_side",
                "port": "veio:block/port", "particle": "minecraft:block/" + side}
    parts, hits = [], []
    motion, center = "None", [8, 8, 8]
    lamp_y = 9

    def part(lo, hi, material="body", hit=True, faces=None):
        parts.append(box(lo, hi, faces or "#" + material))
        if hit: hits.append([*lo, *hi])

    def base(): part([0, 0, 0], [16, 1.5, 16], "base")
    def legs(top=7, wood=False):
        for x in (1, 13):
            for z in (2, 12): part([x, 1.5, z], [x+2, top, z+2], "wood" if wood else "frame")
    def panel(y, x=5, z=14):
        part([x, y, z], [x+6, y+4, z+0.25], hit=False, faces={"south": "#front", "*": "#frame"})
    base()
    if family == "rig":
        for x in (1, 12): part([x, 1.5, 3], [x+3, 14, 13], "frame")
        part([1, 13, 3], [15, 16, 13], "body")
        part([4, 10, 5], [12, 14, 11], "accent")
        part([5, 1.5, 5], [11, 3, 11], "metal")
        if name != "drill": part([0.75, 6, 2.75], [4.25, 9, 13.25], "metal")
        if name == "drill_mk3": part([11.75, 6, 2.75], [15.25, 9, 13.25], "accent")
        panel(9, 5, 11); motion, lamp_y = "Drill", 13
        hits.append([5, 2, 5, 11, 14, 11])
    elif family == "press":
        part([2, 1.5, 2], [14, 4, 14], "metal")
        for x in (1, 12): part([x, 3, 3], [x+3, 14, 13], "frame")
        part([1, 13, 2], [15, 16, 14], "body")
        part([5, 14, 5], [11, 16, 11], "accent")
        panel(8, 1, 13); motion, lamp_y = "Press", 13
        hits.append([5, 4, 5, 11, 14, 11])
    elif family in ("assembly", "electronics", "kitchen"):
        legs()
        part([0, 6, 1], [16, 8, 15], "metal")
        part([2, 8, 3], [14, 9, 13], "accent")
        if family == "electronics":
            part([2, 9, 2], [14, 10, 5], "frame")
            part([3, 10, 2], [13, 13, 3], "glass")
            panel(3, 5, 14)
        elif family == "kitchen":
            for x in (3, 9): part([x, 9, 5], [x+4, 10, 10], "frame")
            part([1, 8, 1], [15, 12, 3], "body")
            panel(2, 5, 14)
        else:
            part([1, 8, 2], [4, 12, 5], "body")
            part([12, 8, 2], [15, 12, 5], "body")
            if name == "integrator": part([1, 12, 2], [15, 14, 5], "frame")
            panel(2, 5, 14); motion, center = "Turntable", [8, 9.5, 9]
        if family == "electronics": motion, center = "Scanner", [8, 10, 9]
        elif family == "kitchen": motion, center = "Mixer", [5, 11, 8]
        lamp_y = 4
    elif family == "saw":
        legs(6)
        part([0, 5, 1], [16, 7, 15], "metal")
        part([2, 7, 2], [4, 9, 14], "accent")
        part([12, 7, 2], [14, 9, 14], "accent")
        part([5, 2, 4], [11, 5, 12], "body")
        if name == "tree_farm": part([1, 9, 2], [4, 15, 5], "wood")
        panel(2, 5, 12); motion, center, lamp_y = "Saw", [8, 9, 8], 5
        hits.append([3.5, 4.5, 7, 12.5, 13.5, 9])
    elif family in ("crusher", "mill", "planter"):
        part([3, 1.5, 3], [13, 9, 13], "body" if family == "crusher" else "wood")
        part([2, 9, 2], [14, 11, 14], "frame")
        # Four hopper lips make a visible opening, rather than a painted cube.
        for lo, hi in [([1,11,1],[15,14,3]), ([1,11,13],[15,14,15]), ([1,11,3],[3,14,13]), ([13,11,3],[15,14,13])]:
            part(lo, hi, "accent")
        part([4, 9, 4], [12, 9.5, 12], "frame")
        if family == "mill":
            part([13, 3, 6], [15, 11, 10], "wood")
            part([13, 5, 4], [15, 9, 12], "wood")
            part([15, 6, 7], [16, 8, 9], "metal")
        panel(3, 5, 13)
        motion = "Roller" if family == "crusher" else "Mill" if family == "mill" else "Seeder"
        center, lamp_y = [8, 10, 8], 9
    elif family == "oven":
        part([1, 1.5, 2], [15, 12, 14], "frame")
        part([2, 12, 3], [14, 14, 13], "body")
        part([4, 3, 14], [12, 10, 15], "accent")
        panel(4, 5, 15)
        part([10, 14, 3], [13, 16, 6], "metal")
        if name == "alloy_furnace": part([3, 14, 3], [6, 16, 6], "metal")
        motion = "Heat"
        lamp_y = 10
    elif family in ("boiler", "tanks", "cells", "silo"):
        if family == "tanks" or family == "cells":
            for x in (2, 9):
                part([x, 2, 3], [x+5, 14, 13], "accent" if family == "tanks" else "body")
                part([x, 14, 4], [x+5, 16, 12], "frame")
            part([6, 5, 6], [10, 8, 10], "metal")
        else:
            part([4, 2, 2], [12, 14, 14], "accent" if family == "boiler" else "metal")
            part([2, 3, 4], [14, 13, 12], "accent" if family == "boiler" else "metal")
            part([3, 13, 3], [13, 15, 13], "frame")
            if family == "boiler": part([10, 14, 4], [13, 16, 7], "metal")
            if name == "steam_engine": part([1, 4, 4], [3, 13, 7], "metal")
            if name == "biomass_generator": part([13, 7, 9], [15, 12, 13], "wood")
        panel(5, 5, 14); lamp_y = 11
        if family in ("boiler", "tanks"): motion, center = "Motor", [8, 8, 14.5]
    elif family in ("counter", "desk", "console", "loading", "landing", "hitch"):
        legs(7, wood=family in ("desk", "counter", "hitch"))
        part([0, 6, 0], [16, 8, 16], "wood" if family in ("desk", "counter", "hitch") else "metal")
        if family == "console":
            part([1, 8, 2], [15, 11, 8], "accent")
            part([2, 11, 2], [14, 15, 3], "glass")
            panel(9, 5, 8)
        elif family == "loading":
            part([1, 8, 3], [6, 12, 13], "body")
            part([10, 8, 3], [15, 12, 13], "wood")
            panel(8, 5, 13)
        elif family == "landing":
            part([2, 8, 2], [14, 9, 14], "accent")
            part([6, 9, 7], [10, 9.25, 9], "metal", False)
            panel(2, 5, 14)
        elif family == "hitch":
            for x in (1, 13): part([x, 8, 2], [x+2, 15, 4], "wood")
            part([1, 12, 2], [15, 14, 4], "wood")
            part([2, 8, 8], [7, 11, 14], "accent")
            panel(2, 5, 14)
        else:
            part([1, 2, 3], [15, 6, 13], "body")
            part([2, 8, 3], [9, 10, 11], "wood")
            if name in ("foreman_desk", "contract_hub"): part([10, 8, 3], [14, 13, 6], "frame")
            panel(2, 5, 13)
        lamp_y = 4
    elif family == "shelves":
        for x in (1, 13): part([x, 1.5, 2], [x+2, 16, 14], "wood")
        for y in (2, 8, 14): part([1, y, 2], [15, y+1.5, 14], "wood")
        part([3, 3.5, 4], [9, 8, 12], "body")
        part([8, 9.5, 3], [13, 14, 11], "body")
        panel(9, 3, 12); lamp_y = 12
    elif family in ("ranch", "harvester"):
        part([2, 1.5, 3], [14, 6, 13], "wood")
        for x in (1, 13): part([x, 6, 3], [x+2, 12, 13], "frame")
        part([1, 12, 2], [15, 14, 14], "accent")
        part([4, 6, 3], [12, 8, 11], "body")
        if family == "harvester": motion, center = "Harvester", [8, 8, 9]
        elif name == "ranch": motion, center = "Feed", [8, 9, 9]
        panel(2, 5, 13); lamp_y = 10
    else:
        raise ValueError(family)

    if motion == "Roller": hits.append([4, 7.5, 5.5, 12, 12.5, 10.5])
    elif motion in ("Turntable", "Mill", "Seeder"):
        x, y, z = center
        hits.append([x-4.25, y, z-4.25, x+4.25, y+1.5, z+4.25])

    extra_sweeps = {"Scanner": [3,9,7,13,12,12], "Mixer": [3,9,5,8,13,11],
                    "Heat": [5,4,15.3,11,9,15.6], "Motor": [5,5,14.3,11,11,14.7],
                    "Harvester": [4,6.5,7,12,9.5,11], "Feed": [4,8,7,12,11,12]}
    if motion in extra_sweeps: hits.append(extra_sweeps[motion])

    # One fixed north outlet and a small south-facing light keep the direction legible.
    non_output = {"generator", "biomass_generator", "steam_engine", "battery", "planter", "foreman_desk", "control_center", "barn", "silo"}
    connector = "#metal" if name in non_output else "#port"
    part([6, 4, 0], [10, 8, 2], faces={"north": connector, "*": "#frame"})
    # Module badge sits on its own front clip, never hidden inside a new roof.
    housing = {face: "#frame" for face in ("north", "east", "west", "up", "down")}
    part([1.5, 1.5, 12.5], [4.5, 5.5, 16], faces=housing)
    part([1.5, 1.5, 14.9], [4.5, 5.5, 15], hit=False, faces={"south": "#frame"})
    part([11.5, lamp_y-0.25, 12.5], [14.5, lamp_y+1.75, 16], faces=housing)
    part([11.5, lamp_y-0.25, 14.9], [14.5, lamp_y+1.75, 15], hit=False, faces={"south": "#frame"})
    lamp = [12, lamp_y+0.25, 15.85, 14, lamp_y+1.25, 16]
    assert len(hits) <= 16, (name, len(hits))
    assert all(0 <= value <= 16 for part_ in parts for key in ("from", "to") for value in part_[key]), name
    marker_y = {"loading": 12.25, "landing": 9.25, "hitch": 8.25}.get(family, max(p["to"][1] for p in parts) + .25)
    return {"marker": [8, marker_y, 8], "textures": textures, "elements": parts, "hits": hits,
            "lamp": lamp, "badge": [2, 3, 15.85], "motion": motion, "center": center, "family": family}


def shapes(profile):
    canonical = [[v / 16 for v in hit] for hit in profile["hits"]]
    values = {}
    for facing in ("south", "west", "north", "east"):
        def orient(x, z):
            return {"south": (x, z), "west": (1-z, x), "north": (1-x, 1-z), "east": (z, 1-x)}[facing]
        rotated = []
        for x0, y0, z0, x1, y1, z1 in canonical:
            corners = [orient(x, z) for x in (x0, x1) for z in (z0, z1)]
            rotated.append([min(p[0] for p in corners), y0, min(p[1] for p in corners),
                            max(p[0] for p in corners), y1, max(p[1] for p in corners)])
        values[facing] = rotated
    return {"property": "facing", "values": values, "default": canonical}


def native_profiles(root, blocks, box):
    lines = ["// Generated by tools/build_content.py from tools/machine_forms.py.\n",
             "use super::{Profile, Motion};\n",
             "pub(super) fn profile(block: &str) -> Option<Profile> {\n    Some(match block {\n"]
    def numbers(values): return "[" + ", ".join(f"{v:.3f}" for v in values) + "]"
    for name, (_, tier, *_) in blocks.items():
        profile = form(name, tier, box)
        if profile:
            lines.append(f'        "veio:{name}" => Profile {{ lamp: {numbers(profile["lamp"])}, badge: {numbers(profile["badge"])}, motion: Motion::{profile["motion"]}, center: {numbers(profile["center"])}, marker: {numbers(profile["marker"])} }},\n')
    lines.append("        _ => return None,\n    })\n}\n")
    target = Path(root).parent / "native/api/src/appearance/data.rs"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text("".join(lines), encoding="utf-8")
