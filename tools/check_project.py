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
                assert all(node["translate"] in lang for lang in languages), node["translate"]
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
    print(f"Veio {versions[0]}: {len(files)} JSON válidos; {len(pages)} páginas e traduções conferidas.")


if __name__ == "__main__":
    main()
