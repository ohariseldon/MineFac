"""Confere as plantas de `tools/blueprints.py` e gera `native/api/src/workforce/blueprint_data.rs`.

Cada célula vira um `u32`: x (5 bits), z (5), camada (5) e índice da paleta (bits 15+). A ordem
de construção já sai pronta: primeiro, de baixo para cima, os blocos que se sustentam sozinhos (e
o ar a limpar); depois, também de baixo para cima, os que se apoiam em outros.
"""
from pathlib import Path
import re
from blueprints import BLUEPRINTS, ATTACHED

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "native/api/src/workforce/blueprint_data.rs"
KEYS = ["canteen", "school", "smithy", "market", "library", "post_office", "inn", "hall", "lab"]
VISITORS = {None: 0, "adults": 1, "children": 2}
STATE = re.compile(r"^([a-z0-9_:]+)(?:\[([a-z0-9_=,]+)\])?$")


def parse(spec):
    match = STATE.match(spec)
    assert match, f"bloco inválido: {spec}"
    block, props = match.group(1), match.group(2)
    pairs = [tuple(pair.split("=")) for pair in props.split(",")] if props else []
    return block, pairs


def attached(block):
    return any(part in block for part in ATTACHED)


def check(key, plan):
    layers = plan["layers"]
    depth, width = len(layers[0]), len(layers[0][0])
    assert width <= 31 and depth <= 31 and len(layers) <= 31, key
    sites = []
    for y, layer in enumerate(layers):
        assert len(layer) == depth, f"{key}: camada {y} com {len(layer)} linhas, esperado {depth}"
        for z, row in enumerate(layer):
            assert len(row) == width, f"{key}: camada {y} linha {z} com {len(row)} colunas: {row!r}"
            for x, char in enumerate(row):
                if char == "S":
                    sites.append((x, y, z))
                elif char not in " .*":
                    assert char in plan["legend"], f"{key}: '{char}' sem legenda (camada {y}, linha {z})"
    assert len(sites) == 1 and sites[0][1] == 1, f"{key}: um S na camada 1, achei {sites}"
    assert ("reserved" in plan) == any("*" in row for layer in layers for row in layer), key
    return width, depth, sites[0]


def main():
    assert sorted(KEYS) == sorted(BLUEPRINTS), "uma planta para cada obra com prédio"
    palette = ["minecraft:air"]
    props = [[]]
    index = {"minecraft:air": 0}
    entries = []
    for key in KEYS:
        plan = BLUEPRINTS[key]
        width, depth, site = check(key, plan)
        first, second, reserved = [], [], []
        for y, layer in enumerate(plan["layers"]):
            for z, row in enumerate(layer):
                for x, char in enumerate(row):
                    if char in " S":
                        continue
                    if char == "*":
                        reserved.append((x, y, z))
                        spec = "minecraft:air"
                    elif char == ".":
                        spec = "minecraft:air"
                    else:
                        spec = plan["legend"][char]
                    if spec not in index:
                        block, pairs = parse(spec)
                        index[spec] = len(palette)
                        palette.append(block)
                        props.append(pairs)
                    cell = x | (z << 5) | (y << 10) | (index[spec] << 15)
                    (second if attached(spec) else first).append(cell)
        staff = plan.get("staff", {})
        def spots(chars):
            found = []
            for y, layer in enumerate(plan["layers"]):
                for z, row in enumerate(layer):
                    for x, char in enumerate(row):
                        if char in chars and char not in " .*S":
                            found.append((x, y, z))
            return found
        work, seats = spots(staff.get("work", "")), spots(staff.get("seats", ""))
        assert not staff.get("work") or work, f"{key}: sem lugar de trabalho"
        assert not staff.get("visitors") or seats, f"{key}: visitantes sem assentos"
        entries.append((key, width, len(plan["layers"]), depth, site, first + second, reserved, plan.get("reserved", ""),
                        staff.get("role", ""), work, seats, VISITORS[staff.get("visitors")], staff.get("hours", [0, 0]),
                        staff.get("max", 0)))
    lines = ["// Gerado por tools/build_blueprints.py a partir de tools/blueprints.py. Não edite à mão.",
             "use super::blueprints::Blueprint;", "",
             f"pub(crate) static PALETTE: [&str; {len(palette)}] = ["]
    lines += [f"    \"{block}\"," for block in palette]
    lines += ["];", "", f"pub(crate) static PROPS: [&[(&str, &str)]; {len(props)}] = ["]
    lines += ["    &[" + ", ".join(f"(\"{k}\", \"{v}\")" for k, v in pairs) + "]," for pairs in props]
    lines += ["];", ""]
    for key, width, height, depth, site, cells, reserved, block, *_ in entries:
        lines.append(f"static {key.upper()}: [u32; {len(cells)}] = [")
        for start in range(0, len(cells), 12):
            lines.append("    " + ", ".join(str(cell) for cell in cells[start:start + 12]) + ",")
        lines.append("];")
    lines += ["", f"pub(crate) static BLUEPRINTS: [Blueprint; {len(entries)}] = ["]
    def coords(found):
        return ", ".join(f"[{x}, {y}, {z}]" for x, y, z in found)
    for key, width, height, depth, site, cells, reserved, block, role, work, seats, visitors, hours, most in entries:
        lines.append(f"    Blueprint {{ key: \"{key}\", size: [{width}, {height}, {depth}], site: [{site[0]}, {site[1]}, {site[2]}], "
                     f"cells: &{key.upper()}, reserved: &[{coords(reserved)}], reserved_block: \"{block}\",")
        lines.append(f"        role: \"{role}\", work: &[{coords(work)}], seats: &[{coords(seats)}], visitors: {visitors}, "
                     f"hours: [{hours[0]}, {hours[1]}], max_visitors: {most} }},")
    lines.append("];")
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    total = sum(len(entry[5]) for entry in entries)
    print(f"{len(entries)} plantas, {total} blocos, paleta com {len(palette)} estados")


if __name__ == "__main__":
    main()
