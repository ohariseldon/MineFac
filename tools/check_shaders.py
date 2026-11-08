"""Confere o pacote de shaders do Veio (`visuals/veio-industrial`) sem abrir o jogo.

Monta cada estágio como o Horizon monta (prelúdio da API 0.1, arquivos `shared`, o arquivo do
estágio e os pontos de entrada) e valida o WGSL com o `naga` (`cargo install naga-cli`).
Também confere o que o `horizon-shader check` recusa e é fácil de esquecer: toda opção precisa
de um `override` do mesmo nome, nenhum nome começa com `horizon_`, nada declara `@group`, e as
páginas alcançam todas as opções.

O prelúdio em `tools/horizon_shader_0_1/` foi gerado do código MIT do Horizon
(`crates/horizon-shader-api`). Isto não substitui o `horizon-shader check` do próprio Horizon,
que também traduz para Vulkan, Metal e DirectX: é a conferência rápida, sem o motor.

    python tools/check_shaders.py [pasta-do-pacote]
"""
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PRELUDE = ROOT / "tools/horizon_shader_0_1"
LIMIT_POST = 12
# O Horizon recusa o pacote inteiro com mais saídas: "a pack may write 4".
LIMIT_OUTPUTS = 4


def piece(stage, part):
    return (PRELUDE / f"{stage}.{part}.wgsl").read_text(encoding="utf-8")


def targets(manifest):
    names = []
    for entry in manifest.get("post", []):
        if "output" in entry and entry["output"] not in names:
            names.append(entry["output"])
    return "".join(f"@group(3) @binding({32 + i}) var {name}: texture_2d<f32>;\n" for i, name in enumerate(names))


def compose(pack, manifest, stage, file, extra_files=()):
    shared = [name for name in manifest.get("shared", []) if name != file]
    texts = [piece(stage, "prelude")]
    if stage == "post":
        texts.append(targets(manifest))
    texts += [(pack / name).read_text(encoding="utf-8") for name in shared]
    texts.append((pack / file).read_text(encoding="utf-8"))
    entries = piece(stage, "entries")
    for name in extra_files:
        texts.append((pack / name).read_text(encoding="utf-8"))
    if stage == "material" and extra_files:
        entries += piece("vertex", "entries")
    texts.append(entries)
    return "\n".join(texts)


def own_rules(pack, manifest):
    problems = []
    sources = {path.name: path.read_text(encoding="utf-8") for path in pack.glob("*.wgsl")}
    for name, text in sources.items():
        code = re.sub(r"//[^\n]*", "", text)
        if "@group" in code or "@binding" in code:
            problems.append(f"{name}: declara binding")
        if re.search(r"@(vertex|fragment|compute)\b", code):
            problems.append(f"{name}: declara ponto de entrada")
        for found in re.findall(r"\b(horizon_\w+)\s*\(", code):
            if not re.search(rf"fn\s+{found}\b", piece("material", "prelude") + piece("post", "prelude")
                             + piece("material", "entries")):
                problems.append(f"{name}: {found} não existe no prelúdio")
        for declared in re.findall(r"\b(?:fn|let|var|const|override|struct)\s+(horizon_\w+)", code):
            problems.append(f"{name}: nome reservado {declared}")
    every = "\n".join(sources.values())
    options = [option["id"] for option in manifest.get("options", [])]
    for option in options:
        if not re.search(rf"\boverride\s+{option}\s*:", every):
            problems.append(f"opção {option} sem override")
    pages = manifest.get("pages", [])
    if pages:
        seen, reached, todo = set(), {pages[0]["id"]}, [pages[0]["id"]]
        by_id = {page["id"]: page for page in pages}
        while todo:
            for item in by_id[todo.pop()].get("items", []):
                if item.startswith("page:"):
                    target = item[5:]
                    if target not in by_id:
                        problems.append(f"página {target} não existe")
                    elif target not in reached:
                        reached.add(target)
                        todo.append(target)
                elif item:
                    seen.add(item)
        problems += [f"página {page['id']} inalcançável" for page in pages if page["id"] not in reached]
        problems += [f"opção {option} fora das páginas" for option in options if option not in seen]
    used = set(manifest.get("shared", []))
    for stage in ("sky", "material", "vertex", "weather"):
        if stage in manifest:
            used.add(manifest[stage]["file"])
    used.update(entry["file"] for entry in manifest.get("post", []))
    for name in sorted(sources):
        if name not in used:
            problems.append(f"{name} não é usado por nenhum estágio nem em shared (o Horizon recusa)")
    if len(manifest.get("post", [])) > LIMIT_POST:
        problems.append(f"mais de {LIMIT_POST} passes")
    outputs = [entry["output"] for entry in manifest.get("post", []) if "output" in entry]
    if len(outputs) > LIMIT_OUTPUTS:
        problems.append(f"{len(outputs)} saídas de [[post]]; o Horizon aceita {LIMIT_OUTPUTS}")
    if len(set(outputs)) != len(outputs):
        problems.append("saída de [[post]] repetida")
    return problems


def main():
    pack = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "visuals/veio-industrial"
    manifest = tomllib.loads((pack / "horizon-shaders.toml").read_text(encoding="utf-8"))
    naga = shutil.which("naga")
    if naga is None:
        raise SystemExit("Instale o validador: cargo install naga-cli")
    stages = []
    vertex = [manifest["vertex"]["file"]] if "vertex" in manifest else []
    if "sky" in manifest:
        stages.append(("sky", manifest["sky"]["file"], ()))
    if "material" in manifest:
        stages.append(("material", manifest["material"]["file"], vertex))
    elif vertex:
        stages.append(("vertex", vertex[0], ()))
    if "weather" in manifest:
        stages.append(("weather", manifest["weather"]["file"], ()))
    for entry in manifest.get("post", []):
        stages.append(("post", entry["file"], ()))
    problems = own_rules(pack, manifest)
    with tempfile.TemporaryDirectory() as folder:
        for index, (stage, file, extra) in enumerate(stages):
            path = Path(folder) / f"{index:02}-{stage}-{Path(file).stem}.wgsl"
            path.write_text(compose(pack, manifest, stage, file, extra), encoding="utf-8")
            result = subprocess.run([naga, str(path)], capture_output=True, text=True)
            label = f"{stage:9} {file}"
            if result.returncode:
                problems.append(f"{label}:\n{result.stdout}{result.stderr}")
            else:
                print(f"ok  {label}")
    if problems:
        print("\n".join(problems))
        raise SystemExit(1)
    print(f"{manifest['id']} {manifest['version']}: {len(stages)} estágios válidos")


if __name__ == "__main__":
    main()
