"""Gera a tabela de produção do Veio a partir de `tools/production.json`.

Saída: `native/api/src/production/data.rs`, com receitas, jazidas, combustíveis e valores de
comida em arrays estáticos (sem custo de leitura durante o jogo). O guia usa o mesmo JSON.
Rode `python tools/build_production.py` ao mudar o JSON e inclua o .rs gerado no commit.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "tools/production.json"
TARGET = ROOT / "native/api/src/production/data.rs"
# Entradas por máquina (o padrão é 2) e máquinas com segundo slot de saída para subprodutos.
INPUT_SLOTS = {"press": 1, "assembler": 1, "cutter": 1, "crusher": 1, "electric_furnace": 1, "mill": 1,
               "fabricator": 3, "manufacturer": 4}
BYPRODUCT_MACHINES = {"manufacturer"}


def load():
    data = json.loads(SOURCE.read_text(encoding="utf-8"))
    groups = data["groups"]

    def expand(name):
        if name.startswith("@"):
            return groups[name[1:]]
        return [name]

    return data, expand


def items(names):
    return "&[" + ", ".join(f'"{name}"' for name in names) + "]"


def main():
    data, expand = load()
    lines = [
        "// @generated por tools/build_production.py a partir de tools/production.json; não edite.",
        "use super::{Fuel, Input, Node, Recipe};",
        "",
        "pub(crate) static RECIPES: &[Recipe] = &[",
    ]
    ids = set()
    for recipe in data["recipes"]:
        # Nome estável: o do JSON ou máquina + produto (+ "_alt", "_alt2"... nas repetidas).
        base = recipe.get("id") or f'{recipe["machine"]}_{recipe["out"][0].split(":")[1]}'
        rid, n = base, 1
        while rid in ids:
            rid = f"{base}_alt" + ("" if n == 1 else str(n))
            n += 1
        assert not recipe.get("id") or rid == base, f"id repetido: {base}"
        ids.add(rid)
        assert len(recipe["in"]) <= INPUT_SLOTS.get(recipe["machine"], 2), f"{rid}: entradas demais"
        by = recipe.get("by")
        assert not by or recipe["machine"] in BYPRODUCT_MACHINES, f"{rid}: {recipe['machine']} não tem slot de subproduto"
        byproduct = f'Some(("{by[0]}", {by[1]}))' if by else "None"
        inputs = ", ".join(
            f"Input {{ items: {items(expand(name))}, count: {count} }}" for name, count in recipe["in"]
        )
        item, count = recipe["out"]
        lines.append(
            f'    Recipe {{ id: "{rid}", machine: "veio:{recipe["machine"]}", inputs: &[{inputs}], '
            f'output: ("{item}", {count}), byproduct: {byproduct}, ticks: {recipe["ticks"]}, power: {recipe["power"]}, '
            f'alt: {str(recipe.get("alt", False)).lower()} }},'
        )
    # No fabricante cada ingrediente tem um slot fixo: a esteira entrega no primeiro slot que
    # aceita o item, então o mesmo item em dois slots travaria a receita que o espera no outro.
    slot_of = {}
    for recipe in data["recipes"]:
        if recipe["machine"] in BYPRODUCT_MACHINES:
            for slot, (name, _) in enumerate(recipe["in"]):
                for item in expand(name):
                    assert slot_of.setdefault(item, slot) == slot, \
                        f"{item} aparece nos slots {slot_of[item] + 1} e {slot + 1} do {recipe['machine']}"
    lines += ["];", "", "pub(crate) static NODES: &[Node] = &["]
    for node in data["nodes"]:
        item, count = node["out"]
        lines.append(
            f'    Node {{ blocks: {items(node["blocks"])}, output: ("{item}", {count}), '
            f'tier: {node["tier"]}, slow: {node["slow"]} }},'
        )
    lines += ["];", ""]
    for machine, fuels in data["fuels"].items():
        lines.append(f"pub(crate) static FUELS_{machine.upper()}: &[Fuel] = &[")
        for name, ticks in fuels:
            lines.append(f"    Fuel {{ items: {items(expand(name))}, ticks: {ticks} }},")
        lines += ["];", ""]
    lines.append("pub(crate) static FOOD: &[(&str, u32)] = &[")
    for name, value in data["food"]:
        lines.append(f'    ("{name}", {value}),')
    lines.append("];")
    TARGET.parent.mkdir(parents=True, exist_ok=True)
    TARGET.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"{len(data['recipes'])} receitas, {len(data['nodes'])} jazidas e "
          f"{len(data['food'])} alimentos em {TARGET.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
