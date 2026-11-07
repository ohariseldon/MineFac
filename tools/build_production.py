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
    for recipe in data["recipes"]:
        inputs = ", ".join(
            f"Input {{ items: {items(expand(name))}, count: {count} }}" for name, count in recipe["in"]
        )
        item, count = recipe["out"]
        lines.append(
            f'    Recipe {{ machine: "veio:{recipe["machine"]}", inputs: &[{inputs}], '
            f'output: ("{item}", {count}), ticks: {recipe["ticks"]}, power: {recipe["power"]}, '
            f'alt: {str(recipe.get("alt", False)).lower()} }},'
        )
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
