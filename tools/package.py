"""Monta o pacote jogável do Veio para Windows: `dist/Veio-<versão>.zip`.

Conteúdo do pacote:
- `launcher.exe`: compila o código nativo pelo Horizon Launcher, instala `content/` na instância
  e abre o launcher (código em `launcher/`);
- `content/`: assets e dados do mod;
- `native/`: `horizon-mixin.toml` e o código em `native/api`, sem a verificação offline;
- `LEIA-ME.txt`.

O executável é gerado, nunca versionado. Em Linux, a compilação cruzada requer o alvo
`x86_64-pc-windows-gnu` (rustup target add) e `x86_64-w64-mingw32-gcc`. No Windows, o alvo
padrão já serve. Use `--exe caminho` para empacotar um launcher.exe já compilado.
"""
import platform
import subprocess
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
README = """Veio {version}

1. Instale o Horizon Launcher, entre com a conta e faça o Prepare na página Play (uma vez).
2. Feche o jogo e abra launcher.exe nesta pasta.
3. O launcher cria a instância Veio se ela não existir, compila o código nativo (só quando a
   versão muda; a primeira vez leva de 20 a 30 minutos), instala o conteúdo e abre o jogo.
4. Na primeira partida, ative o Veio em Opções > Mods e reinicie o jogo.

Opções: launcher.exe --rebuild (compila de novo), --skip-build (só copia o conteúdo).
Mantenha launcher.exe, content e native juntos nesta pasta.
"""


def version():
    for line in (ROOT / "content/horizon-mod.toml").read_text(encoding="utf-8").splitlines():
        if line.startswith("version"):
            return line.split("=", 1)[1].strip().strip('"')
    raise SystemExit("Versão não encontrada")


def build_launcher():
    windows = platform.system() == "Windows"
    command = ["cargo", "build", "--release"] + ([] if windows else ["--target", "x86_64-pc-windows-gnu"])
    subprocess.run(command, cwd=ROOT / "launcher", check=True)
    folder = "release" if windows else "x86_64-pc-windows-gnu/release"
    return ROOT / "launcher/target" / folder / "launcher.exe"


def files(folder, skip=()):
    for path in sorted((ROOT / folder).rglob("*")):
        relative = path.relative_to(ROOT)
        if path.is_file() and not any(part in skip for part in relative.parts) and "__pycache__" not in relative.parts:
            yield path, relative


def main():
    exe = Path(sys.argv[sys.argv.index("--exe") + 1]) if "--exe" in sys.argv else build_launcher()
    current = version()
    out = ROOT / "dist" / f"Veio-{current}.zip"
    out.parent.mkdir(exist_ok=True)
    top = f"Veio-{current}"
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as archive:
        archive.write(exe, f"{top}/launcher.exe")
        archive.writestr(f"{top}/LEIA-ME.txt", README.format(version=current).replace("\n", "\r\n"))
        archive.write(ROOT / "CREDITS.md", f"{top}/CREDITS.md")
        for path, relative in files("third_party/audio/kenney"):
            if path.name in {"LICENSE.txt", "SOURCES.json"}:
                archive.write(path, f"{top}/{relative.as_posix()}")
        count = 0
        for path, relative in [*files("content"), *files("native", skip={"offline-check", "target"})]:
            archive.write(path, f"{top}/{relative.as_posix()}")
            count += 1
    print(f"{out.relative_to(ROOT)}: launcher.exe e {count} arquivos ({out.stat().st_size // 1024} KB)")


if __name__ == "__main__":
    main()
