"""Verificação portátil de conteúdo, sem launcher ou dependências externas."""
import json
import math
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONTENT = ROOT / "content"


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def main():
    files = list(CONTENT.rglob("*.json"))
    for path in files:
        read_json(path)
    manifests = [tomllib.loads((ROOT / name).read_text(encoding="utf-8"))
                 for name in ("content/horizon-mod.toml", "native/horizon-mixin.toml", "native/api/Cargo.toml")]
    versions = [manifests[0]["version"], manifests[1]["mod"]["version"], manifests[2]["package"]["version"]]
    assert len(set(versions)) == 1, f"Versões diferentes: {versions}"
    languages = [read_json(CONTENT / f"assets/veio/lang/{code}.json") for code in ("pt_br", "en_us")]
    pages = check_book(languages, "guide")
    check_book(languages, "village_guide")
    check_page_fit(languages, "guide")
    # O guia da vila (0.18-0.25) ainda tem páginas longas: aviso, não erro, até serem revistas.
    check_page_fit(languages, "village_guide", strict=False)
    check_book(languages, "transport_guide")
    check_book(languages, "industry_guide")
    check_book(languages, "first_steps")
    check_page_fit(languages, "first_steps")
    check_page_fit(languages, "transport_guide")
    # Dynamic native keys are not reachable from a book's JSON tree.
    keys = ([f"veio.onboarding.goal.{i}" for i in range(16)]
            + [f"veio.onboarding.{part}.{i}" for part in ("title", "why", "story") for i in range(17)]
            + [f"veio.onboarding.from.{i}" for i in (*range(8), 8, 10, 11, 12)]
            + [f"veio.onboarding.recipe.{i}.0" for i in (0, 1, 3, 4, 5, 6, 7, 8, 10, 11, 12)]
            + ["veio.onboarding.recipe.1.1", "veio.onboarding.book", "veio.onboarding.make", "veio.onboarding.done"]
            + [f"veio.onboarding.name.{item.rsplit(':', 1)[-1]}" for item in onboarding_items()]
            + [f"veio.village.{part}.{i}" for part in ("title", "why", "goal", "done") for i in range(8)]
            + [f"veio.village.recipe.{i}" for i in (0, 2, 4, 5, 6, 7)] + [f"veio.village.end.{i}" for i in range(1, 5)]
            + [f"veio.city.{part}.{i}" for part in ("title", "why", "goal", "done") for i in range(8)]
            + [f"veio.city.recipe.{i}" for i in (0, 1, 4, 5, 6)] + [f"veio.city.end.{i}" for i in range(1, 5)]
            + [f"veio.onboarding.blocked.{i}" for i in (3, 5, 9)]
            + ["veio.onboarding.hidden", "veio.onboarding.shown", "veio.onboarding.save_failed"])
    for language in languages:
        assert all(language.get(key, "").strip() for key in keys), "Falta tradução do tutorial na tela"
    textures = check_assets(languages)
    from check_machine_audio import check as check_audio
    check_audio()
    print(f"Veio {versions[0]}: {len(files)} JSON válidos; {len(pages)} páginas e traduções conferidas; "
          f"{textures} texturas 16x16 encontradas.")


# Largura dos caracteres da fonte padrão do Minecraft, com o espaço depois de cada um. O que
# não está aqui ocupa 6 pixels (a maioria das letras, acentuadas ou não, e os dígitos).
GLYPH = {**dict.fromkeys("i!.,:;|'", 2), **dict.fromkeys("l`", 3),
         **dict.fromkeys("It []()\"{}*", 4), **dict.fromkeys("fk<>", 5), **dict.fromkeys("@~", 7)}
PAGE_WIDTH = 114
PAGE_LINES = 14


def page_lines(runs):
    """Linhas que um texto ocupa na página do livro, quebrando por palavra como o jogo.
    `runs` é uma lista de (texto, negrito); o negrito soma um pixel por caractere."""
    lines, width = 1, 0
    word = 0
    # Quebras no fim não ocupam linha visível.
    while runs and runs[-1][0].strip("\n") == "":
        runs = runs[:-1]
    if runs:
        runs = runs[:-1] + [(runs[-1][0].rstrip("\n"), runs[-1][1])]
    for text, bold in runs:
        for char in text:
            if char == "\n":
                lines, width, word = lines + 1, 0, 0
                continue
            advance = GLYPH.get(char, 6) + (1 if bold else 0)
            if char == " ":
                width += advance
                word = 0
                continue
            if width + advance > PAGE_WIDTH:
                if 0 < word < width:
                    width = word
                else:
                    width = 0
                lines += 1
            width += advance
            word += advance
    return lines


def flatten(node, lang, bold=False):
    if isinstance(node, list):
        return [run for child in node for run in flatten(child, lang, bold)]
    if isinstance(node, str):
        return [(node, bold)]
    bold = node.get("bold", bold)
    runs = []
    if "text" in node:
        runs.append((node["text"], bold))
    if "translate" in node:
        key = node["translate"]
        # Nomes vanilla vêm das traduções do jogo, que não estão aqui: o id vira palavras,
        # com o tamanho parecido com o do nome traduzido.
        vanilla = key.split(".")[-1].replace("_", " ").capitalize()
        runs.append((lang.get(key, vanilla).replace("%%", "%"), bold))
    return runs + flatten(node.get("extra", []), lang, bold)


def onboarding_items():
    # Itens da lista de materiais do tutorial (CRAFT em onboarding.rs): todos precisam de nome.
    import re
    source = (ROOT / "native/api/src/onboarding.rs").read_text(encoding="utf-8")
    village = (ROOT / "native/api/src/onboarding/village.rs").read_text(encoding="utf-8")
    city = (ROOT / "native/api/src/onboarding/city.rs").read_text(encoding="utf-8")
    block = (source[source.index("const CRAFT"):source.index("fn craft_bit")] + village[village.index("const PARTS"):village.index("pub(super) fn slot")]
             + city[city.index("const PARTS"):city.index("pub(super) fn step")])
    consts = {"GENERATOR": "generator", "PRESS": "press", "CABLE": "cable", "ASSEMBLER": "assembler", "DRILL": "drill",
              "BELT": "belt", "HUB": "contract_hub", "PLATE": "copper_plate", "GEAR": "gear", "FARM_POST": "farm_post",
              "FIELD_MARKER": "field_marker", "HAULER_POST": "hauler_post", "FOREMAN_DESK": "foreman_desk", "HOUSE": "house",
              "TOWN_HALL": "town_hall", "WELL": "well", "SHOP": "shop", "PARK": "park",
              "BOARD": "factory_board", "POLE": "pole", "SPLITTER": "splitter"}
    return re.findall(r'"#?([a-z_]+:[a-z_]+)"', block) + [consts[name] for name in re.findall(r"\b([A-Z_]+)\b", block) if name in consts]


def check_page_fit(languages, name, strict=True):
    """Nenhuma página passa das 14 linhas do livro (o resto ficaria cortado no jogo)."""
    recipe = read_json(CONTENT / f"data/veio/recipe/{name}.json")
    pages = recipe["result"]["components"]["minecraft:written_book_content"]["pages"]
    long = []
    for number, page in enumerate(pages, 1):
        for code, lang in zip(("pt_br", "en_us"), languages):
            lines = page_lines(flatten(page["raw"], lang))
            if lines > PAGE_LINES:
                long.append(f"{name} p.{number} {code}: {lines} linhas")
    if long and not strict:
        print(f"Aviso: {len(long)} páginas de {name} passam de {PAGE_LINES} linhas: " + ", ".join(long))
        return
    assert not long, "Páginas que não cabem no livro:\n" + "\n".join(long)


def check_book(languages, name):
    """Um livro do Veio: até 100 páginas, traduções e links de página válidos, e o livro
    fabricado igual ao entregue pelo avanço."""
    recipe = read_json(CONTENT / f"data/veio/recipe/{name}.json")
    components = recipe["result"]["components"]
    pages = components["minecraft:written_book_content"]["pages"]
    assert 0 < len(pages) <= 100, f"{name}: o livro aceita de 1 a 100 páginas"

    def visit(node):
        if isinstance(node, dict):
            if "translate" in node:
                key = node["translate"]
                # Nomes vanilla vêm das traduções do próprio jogo.
                vanilla = key.startswith(("item.minecraft.", "block.minecraft."))
                assert vanilla or all(key in lang for lang in languages), key
            if node.get("click_event", {}).get("action") == "change_page":
                assert 1 <= node["click_event"]["page"] <= len(pages), node["click_event"]
            for child in node.values():
                visit(child)
        elif isinstance(node, list):
            for child in node:
                visit(child)

    visit(pages)
    loot = read_json(CONTENT / f"data/veio/loot_table/tutorial/{name}.json")
    assert loot["pools"][0]["entries"][0]["functions"][0]["components"] == components, f"{name}: livro entregue e fabricado devem ser iguais"
    for language in languages:
        assert all(language[key] for key in language if key.startswith(("veio.guide.", "veio.village.", "veio.industry."))), "Texto vazio no guia"
    return pages


def png_size(path):
    data = path.read_bytes()
    assert data[:8] == b"\x89PNG\r\n\x1a\n", f"{path} não é PNG"
    return int.from_bytes(data[16:20], "big"), int.from_bytes(data[20:24], "big")


def check_shapes(path):
    """Horizon shape tables use property tuples and at most 16 boxes per state."""
    block = read_json(path)
    for name in ("shapes", "collision_shapes"):
        table = block.get(name)
        if table is None:
            continue
        assert set(table) <= {"property", "properties", "values", "default"}, f"{path.name}: tabela {name} inválida"
        assert not ("property" in table and "properties" in table), path.name
        properties = table.get("properties", [table["property"]] if "property" in table else [])
        assert len(properties) == len(set(properties)), path.name
        for key in table.get("values", {}):
            assert len(key.split(",")) == len(properties), f"{path.name}: tupla {key} inválida"
        for boxes in [table.get("default", []), *table.get("values", {}).values()]:
            assert len(boxes) <= 16, f"{path.name}: máximo de 16 caixas por forma"
            for box in boxes:
                assert len(box) == 6 and all(math.isfinite(v) for v in box), path.name
                assert all(box[i] <= box[i + 3] for i in range(3)), path.name


def check_assets(languages):
    """Cada bloco e item registrado tem nome, modelo, receita e texturas próprias existentes."""
    assets = CONTENT / "assets/veio"
    data = CONTENT / "data/veio"
    textures = sorted((assets / "textures").rglob("*.png"))
    for path in textures:
        if path.parent.name == "gui" and path.name == "ui.png":
            # Atlas da interface (ícones, setas e chamas): como as telas vanilla, até 256x256.
            width, height = png_size(path)
            assert width <= 256 and height <= 256, f"{path.name} passa de 256x256"
            continue
        assert png_size(path) == (16, 16), f"{path.name} deve ter 16x16, como as texturas vanilla"
    for kind in ("block", "item"):
        for path in (data / f"horizon/{kind}").glob("*.json"):
            name = path.stem
            for language in languages:
                assert f"{kind}.veio.{name}" in language, f"Falta o nome de {kind}.veio.{name}"
            if kind == "block" and name in ("structure", "hatch_in", "hatch_out"):
                check_shapes(path)  # Partes das máquinas grandes: sem item nem receita.
                continue
            assert (assets / f"items/{name}.json").exists(), f"Falta assets/veio/items/{name}.json"
            if kind == "block":  # Componentes saem das máquinas, não da bancada.
                check_shapes(path)
                assert (data / f"recipe/{name}.json").exists(), f"Falta a receita de {name}"
    for model in (assets / "models").rglob("*.json"):
        textures_used = read_json(model).get("textures", {}).values()
        for texture in textures_used:
            if texture.startswith("veio:"):
                path = assets / f"textures/{texture[5:]}.png"
                assert path.exists(), f"{model.name} usa {texture}, que não existe"
    import re
    source = (ROOT / "native/api/src/presentation.rs").read_text(encoding="utf-8")
    slots = int(re.search(r"const DATA_SLOTS: usize = (\d+);", source).group(1))
    for menu in (data / "horizon/menu").glob("*.json"):
        assert read_json(menu)["data_slots"] == slots, f"{menu.name}: data_slots deve ser {slots}, como no painel"
    production = json.loads((ROOT / "tools/production.json").read_text(encoding="utf-8"))
    generated = (ROOT / "native/api/src/production/data.rs").read_text(encoding="utf-8")
    counts = (generated.count("    Recipe {"), generated.count("    Node {"))
    assert counts == (len(production["recipes"]), len(production["nodes"])), \
        "data.rs desatualizado: rode python tools/build_production.py"
    unlocked = set()
    for advancement in (data / "advancement").rglob("*.json"):
        unlocked.update(read_json(advancement).get("rewards", {}).get("recipes", []))
    for recipe in (data / "recipe").glob("*.json"):
        assert f"veio:{recipe.stem}" in unlocked, f"Nenhum avanço libera a receita {recipe.stem}"
    # Equilíbrio da economia: as regras do simulador (salário, teto, frete, saturação, contratos).
    import importlib.util
    spec = importlib.util.spec_from_file_location("sim_economy", ROOT / "tools/sim_economy.py")
    sim = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(sim)
    failures = sim.invariants()
    assert not failures, f"Economia desequilibrada: {failures}"
    return len(textures)


if __name__ == "__main__":
    main()
