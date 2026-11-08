"""Monta o pacote de texturas realistas do Realismo: `visuals/texturas/Veio-Texturas-JG-128x.zip`.

Fonte: **PBR JG** 1.1.0 (Jason J. Gardner, conversão Java do JG RTX), 256x com mapas LabPBR
(`_n` normal/AO/altura, `_s` brilho/metal/emissão). Licença: Creative Commons BY-NC-SA 4.0
(https://github.com/jasonjgardner/jg-rtx). Pode ser redistribuído sem fins comerciais, com
crédito e na mesma licença, indicando o que mudou. O Veio é gratuito.

O que esta ferramenta muda: reduz cada textura pela metade (256x → 128x por padrão), para caber
na meta de PCs de 8 GB, mantém minérios, plantações, itens, criaturas e água como no Minecraft (só blocos do cenário
ficam realistas) e troca a descrição do `pack.mcmeta`. O ZIP não vai para o
git (passa de 100 MB): rode esta ferramenta antes do `tools/package.py`.

    python tools/build_realistic_textures.py            # 128x
    python tools/build_realistic_textures.py --size 64  # mais leve ainda

O download é conferido pelo SHA-512 que o Modrinth publica.
"""
import argparse
import hashlib
import io
import json
import sys
import urllib.request
import zipfile
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
URL = "https://cdn.modrinth.com/data/F2D7CupJ/versions/theqvBoc/JG.zip"
SHA512 = ("04437d7c40dc9ea76d4fe08fed406ce1d258652cede08c9e63f50a49cad9782c"
          "db4db4bb191f41b63da3a3798cb37fed95e794f2eb3606268e027ed6210931bd")
SOURCE_SIZE = 256
OUT = ROOT / "visuals" / "texturas"
NAME = "Veio-Texturas-JG-{size}x.zip"
# Só os blocos do cenário e de construção ficam realistas (pedra, areia, grama, madeira...).
# Ficam como no Minecraft, para quem conhece o jogo reconhecer na hora (pedido do Luiz):
# minérios, plantações, itens, criaturas, interface e partículas, e a água, que o shader desenha.
BLOCKS = "assets/minecraft/textures/block/"
KEEP_ORIGINAL = (
    "_ore", "ancient_debris", "raw_iron_block", "raw_gold_block", "raw_copper_block",
    "wheat_stage", "carrots_stage", "potatoes_stage", "beetroots_stage", "melon_stem",
    "attached_melon_stem", "pumpkin_stem", "attached_pumpkin_stem", "cocoa_stage",
    "nether_wart_stage", "sweet_berry_bush", "torchflower_crop", "pitcher_crop",
    "water_",
)


def realistic(name):
    """Se a textura entra no pacote: blocos do cenário, sem minérios, plantações e água."""
    if not name.startswith(BLOCKS):
        return False
    stem = name[len(BLOCKS):].split("/")[-1]
    return not any(key in stem for key in KEEP_ORIGINAL)

NOTICE = """Veio — Texturas realistas ({size}x)

Este pacote é uma adaptação de "PBR JG" 1.1.0, de Jason J. Gardner
(https://modrinth.com/resourcepack/pbr-jg, https://github.com/jasonjgardner/jg-rtx),
licenciado sob Creative Commons Atribuição-NãoComercial-CompartilhaIgual 4.0
(https://creativecommons.org/licenses/by-nc-sa/4.0/). Os créditos dos materiais de origem
estão em https://github.com/jasonjgardner/jg-rtx (docs).

Alterações feitas pelo Veio (tools/build_realistic_textures.py): texturas reduzidas de 256x para
{size}x, só texturas de blocos do cenário mantidas (sem minérios, plantações,
itens, criaturas e água) e descrição do pack.mcmeta trocada. Esta adaptação segue a mesma licença.
Uso não comercial.
"""


def download(cache):
    if cache.is_file() and hashlib.sha512(cache.read_bytes()).hexdigest() == SHA512:
        return cache.read_bytes()
    print(f"Baixando {URL} ...")
    data = urllib.request.urlopen(URL, timeout=600).read()
    if hashlib.sha512(data).hexdigest() != SHA512:
        raise SystemExit("O download não confere com o SHA-512 publicado; nada foi gerado.")
    cache.parent.mkdir(parents=True, exist_ok=True)
    cache.write_bytes(data)
    return data


def shrink(name, data, factor):
    image = Image.open(io.BytesIO(data))
    width, height = image.size
    if factor == 1 or width < 32 or height < 32 or width % factor or height % factor:
        return data
    mode = image.mode
    if mode not in ("RGB", "RGBA", "L", "LA"):
        image = image.convert("RGBA")
    # Mapas de dados (_n, _s) com média simples; a cor com Lanczos, que guarda a nitidez.
    data_map = name.endswith(("_n.png", "_s.png"))
    image = image.resize((width // factor, height // factor),
                         Image.Resampling.BOX if data_map else Image.Resampling.LANCZOS)
    out = io.BytesIO()
    image.save(out, "PNG", optimize=False, compress_level=9)
    return out.getvalue()


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--size", type=int, default=128, choices=[32, 64, 128, 256])
    parser.add_argument("--cache", type=Path, default=ROOT / ".build" / "pbr-jg-1.1.0.zip")
    args = parser.parse_args()
    factor = SOURCE_SIZE // args.size
    source = zipfile.ZipFile(io.BytesIO(download(args.cache)))
    OUT.mkdir(parents=True, exist_ok=True)
    target = OUT / NAME.format(size=args.size)
    for old in OUT.glob("Veio-Texturas-JG-*.zip"):
        old.unlink()
    count = 0
    with zipfile.ZipFile(target, "w", zipfile.ZIP_DEFLATED) as out:
        for info in source.infolist():
            if info.is_dir():
                continue
            # Fora do pacote: o jogo usa a textura original (a pasta ctm do OptiFine também sai).
            if info.filename.startswith("assets/") and not realistic(info.filename):
                continue
            data = source.read(info)
            if info.filename == "pack.mcmeta":
                meta = json.loads(data)
                meta["pack"]["description"] = f"Veio: texturas realistas {args.size}x (PBR JG, CC BY-NC-SA)"
                data = json.dumps(meta, ensure_ascii=False, indent=2).encode("utf-8")
            elif info.filename.lower().endswith(".png") and info.filename != "pack.png":
                data = shrink(info.filename, data, factor)
                count += 1
            out.writestr(info.filename, data, compress_type=zipfile.ZIP_STORED if info.filename.endswith(".png") else zipfile.ZIP_DEFLATED)
        out.writestr("LEIA-ME-LICENCA.txt", NOTICE.format(size=args.size))
    size = target.stat().st_size / 1e6
    print(f"{target.relative_to(ROOT)}: {count} texturas em {args.size}x, {size:.0f} MB")


if __name__ == "__main__":
    sys.exit(main())
