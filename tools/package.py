"""Monta o pacote jogável do Veio para Windows: `dist/Veio-<versão>.zip`.

Conteúdo do pacote:
- `launcher.exe`: compila o código nativo pelo Horizon Launcher, instala `content/` na instância
  e abre o launcher (código em `launcher/`);
- `content/`: assets e dados do mod;
- `native/`: `horizon-mixin.toml` e o código em `native/api`, sem a verificação offline;
- `visuals/`: shader industrial leve original;
- `Jogar-Leve.cmd` e `Escolher-Visual.cmd`: escolha do visual antes da partida;
- `LEIA-ME.txt`.

O executável é gerado, nunca versionado. Em Linux, a compilação cruzada requer o alvo
`x86_64-pc-windows-gnu` (rustup target add) e `x86_64-w64-mingw32-gcc`. No Windows, o alvo
padrão já serve. Use `--exe caminho` para empacotar um launcher.exe já compilado.
"""
import argparse
import platform
import subprocess
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
README = """Veio {version}

1. Instale o Horizon Launcher, entre com a conta e faça o Prepare na página Play (uma vez).
2. Feche o jogo e abra launcher.exe nesta pasta.
3. Antes de abrir o jogo, escolha Realismo leve (padrão pensado para 8 GB) ou Leve (sem shaders).
   Jogar-Leve.cmd inicia sem shaders desde a primeira partida; Escolher-Visual.cmd muda a escolha.
4. O launcher cria a instância Veio se ela não existir, compila o código nativo otimizado (só quando a
   versão muda; a primeira vez leva de 20 a 30 minutos), instala o conteúdo e abre o jogo.
5. Na primeira partida, ative o Veio em Opções > Mods e reinicie o jogo.

Realismo: sol e sombras próximos, céu natural, metais e água com iluminação suave.
Leve: sem pacote de shaders e com distância de visão reduzida.
Os 8 GB são uma meta para a memória total do PC; o desempenho também depende da placa gráfica.
As configurações originais ficam em options.txt.antes-do-veio e horizon-graphics.txt.antes-do-veio.
Depois de escolher, ajustes feitos dentro do jogo são preservados nas próximas aberturas.

Opções: launcher.exe --rebuild (compila de novo), --skip-build (só instala),
--graphics (escolhe o visual), --visual leve ou --visual realismo (escolhe sem perguntar).
Mantenha launcher.exe, content, native e visuals juntos nesta pasta.
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
    parser = argparse.ArgumentParser(description="Monta o pacote jogável do Veio para Windows.")
    parser.add_argument("--exe", type=Path, help="Launcher já compilado")
    parser.add_argument("--out", type=Path, help="Arquivo ZIP de saída (padrão: dist/Veio-<versão>.zip)")
    args = parser.parse_args()
    exe = args.exe if args.exe is not None else build_launcher()
    current = version()
    out = args.out.resolve() if args.out is not None else ROOT / "dist" / f"Veio-{current}.zip"
    out.parent.mkdir(parents=True, exist_ok=True)
    top = f"Veio-{current}"
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as archive:
        archive.write(exe, f"{top}/launcher.exe")
        for name in ("Jogar-Leve.cmd", "Escolher-Visual.cmd"):
            archive.writestr(f"{top}/{name}", (ROOT / name).read_text(encoding="utf-8").replace("\n", "\r\n"))
        archive.writestr(f"{top}/LEIA-ME.txt", README.format(version=current).replace("\n", "\r\n"))
        notes = ROOT / "CONSOLIDACAO.md"
        if notes.exists():
            archive.write(notes, f"{top}/CONSOLIDACAO.md")
        count = 0
        for path, relative in [*files("content"), *files("native", skip={"offline-check", "target"}), *files("visuals")]:
            archive.write(path, f"{top}/{relative.as_posix()}")
            count += 1
    print(f"{out}: launcher.exe e {count} arquivos ({out.stat().st_size // 1024} KB)")


if __name__ == "__main__":
    main()
