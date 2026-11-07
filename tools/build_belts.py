"""Belt paths: quarter turns and continuous 45 degree ramps."""
import math


def build(root, model, box, write):
    for name in ("belt", "fast_belt"):
        textures, straight = model("belt", name, None)
        shapes = {"default": [[0, 0, 0, 1, .25, 1]]}
        variants = {}
        for path in ("straight", "left", "right", "up", "down"):
            elements = straight
            if path in ("left", "right"):
                elements = []
                cells = []
                for x in range(0, 16, 2):
                    for z in range(0, 16, 2):
                        radius = math.hypot(x + 1 if path == "left" else 15 - x, z + 1)
                        if 2 <= radius <= 14:
                            top = "#rail" if radius < 4 or radius > 12 else "#belt"
                            elements.append(box([x, 0, z], [x + 2, 3, z + 2], {"up": top, "*": "#iron"}))
                            cells.append([x / 16, 0, z / 16, (x + 2) / 16, .1875, (z + 2) / 16])
                # Merge each row so Horizon's 16-box limit is respected.
                rows = []
                for z in range(0, 16, 2):
                    row = [cell for cell in cells if cell[2] == z / 16]
                    if row:
                        rows.append([min(c[0] for c in row), 0, z / 16,
                                     max(c[3] for c in row), .1875, (z + 2) / 16])
                shapes[f"path={path}"] = rows
            elif path in ("up", "down"):
                origin = [8, 3, 16 if path == "up" else 0]
                rotation = {"origin": origin, "axis": "x", "angle": 45 if path == "up" else -45}
                start, end = (16 - math.sqrt(512), 16) if path == "up" else (0, math.sqrt(512))
                uv = {"up": [0, 0, 16, 16], "side": [0, 0, 16, 3]}
                elements = [
                    box([0, 0, start], [16, 2, end], {"up": "#base", "*": "#base_side"}, rotation=rotation, uv=uv),
                    box([2, 2, start], [14, 3, end], "#belt", rotation=rotation, uv=uv),
                    box([0, 2, start], [2, 4, end], {"up": "#rail", "*": "#iron"}, rotation=rotation, uv=uv),
                    box([14, 2, start], [16, 4, end], {"up": "#rail", "*": "#iron"}, rotation=rotation, uv=uv),
                ]
                shapes[f"path={path}"] = [[0, 0, z / 16, 1,
                    (19 - z if path == "up" else 3 + z + 2) / 16, (z + 2) / 16]
                    for z in range(0, 16, 2)]
            model_name = f"{name}_{path}"
            write(root / f"assets/veio/models/block/{model_name}.json", {
                "parent": "minecraft:block/block", "textures": textures, "elements": elements})
            for facing, yaw in (("south", 0), ("west", 90), ("north", 180), ("east", 270)):
                variants[f"facing={facing},path={path}"] = {"model": f"veio:block/{model_name}", "y": yaw}
        write(root / f"assets/veio/blockstates/{name}.json", {"variants": variants})
        # Collision and outline follow facing too, just like the model variants.
        oriented = {}
        for selector, boxes in shapes.items():
            if selector == "default":
                oriented[selector] = boxes
                continue
            for facing in ("south", "west", "north", "east"):
                def turn(x, z):
                    return {"south": (x, z), "west": (1-z, x),
                            "north": (1-x, 1-z), "east": (z, 1-x)}[facing]
                out = []
                for x1, y1, z1, x2, y2, z2 in boxes:
                    corners = [turn(x, z) for x in (x1, x2) for z in (z1, z2)]
                    out.append([min(c[0] for c in corners), y1, min(c[1] for c in corners),
                                max(c[0] for c in corners), y2, max(c[1] for c in corners)])
                oriented[f"{facing},{selector.split('=')[1]}"] = out
        table = {"properties": ["facing", "path"],
                 "values": {key: value for key, value in oriented.items() if key != "default"},
                 "default": oriented["default"]}
        write(root / f"data/veio/horizon/block/{name}.json", {
            "template": "minecraft:stonecutter", "item": True,
            "properties": {"hardness": 1.5, "occludes": False},
            "state_properties": {"path": {"type": "enum", "values": ["straight", "left", "right", "up", "down"]}},
            "default_state": {"path": "straight"}, "shapes": table, "collision_shapes": table})
