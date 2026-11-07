"""Verificação portátil de conteúdo, sem launcher ou dependências externas."""
import json
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
    recipe = read_json(CONTENT / "data/veio/recipe/guide.json")
    components = recipe["result"]["components"]
    pages = components["minecraft:written_book_content"]["pages"]
    assert 0 < len(pages) <= 100, "O livro aceita de 1 a 100 páginas"

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
    loot = read_json(CONTENT / "data/veio/loot_table/tutorial/guide.json")
    assert loot["pools"][0]["entries"][0]["functions"][0]["components"] == components, "Guia entregue e fabricado devem ser iguais"
    for language in languages:
        assert all(language[key] for key in language if key.startswith("veio.guide.")), "Texto vazio no guia"
    textures = check_assets(languages)
    print(f"Veio {versions[0]}: {len(files)} JSON válidos; {len(pages)} páginas e traduções conferidas; "
          f"{textures} texturas 16x16 encontradas.")


def png_size(path):
    data = path.read_bytes()
    assert data[:8] == b"\x89PNG\r\n\x1a\n", f"{path} não é PNG"
    return int.from_bytes(data[16:20], "big"), int.from_bytes(data[20:24], "big")


def check_assets(languages):
    """Cada bloco e item registrado tem nome, modelo, receita e texturas próprias existentes."""
    assets = CONTENT / "assets/veio"
    data = CONTENT / "data/veio"
    textures = sorted((assets / "textures").rglob("*.png"))
    for path in textures:
        assert png_size(path) == (16, 16), f"{path.name} deve ter 16x16, como as texturas vanilla"
    for kind in ("block", "item"):
        for path in (data / f"horizon/{kind}").glob("*.json"):
            name = path.stem
            for language in languages:
                assert f"{kind}.veio.{name}" in language, f"Falta o nome de {kind}.veio.{name}"
            assert (assets / f"items/{name}.json").exists(), f"Falta assets/veio/items/{name}.json"
            if kind == "block":  # Componentes saem das máquinas, não da bancada.
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
    return len(textures)


if __name__ == "__main__":
    main()
