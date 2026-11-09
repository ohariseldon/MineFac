"""Monta o pacote de texturas realistas do Realismo: `visuals/texturas/Veio-Texturas-JG-128x.zip`.

Fonte: **PBR JG** 1.1.0 (Jason J. Gardner, conversão Java do JG RTX), 256x com mapas LabPBR
(`_n` normal/AO/altura, `_s` brilho/metal/emissão). Licença: Creative Commons BY-NC-SA 4.0
(https://github.com/jasonjgardner/jg-rtx). Pode ser redistribuído sem fins comerciais, com
crédito e na mesma licença, indicando o que mudou. O Veio é gratuito.

O que esta ferramenta muda: reduz cada textura pela metade (256x → 128x por padrão), para caber
na meta de PCs de 8 GB, acerta o brilho médio de cada textura com a do Minecraft 26.3 (o JG é
calibrado para shaders PBR e fica até 2x mais escuro com a luz do jogo: pedra 57%, grama 76%),
mantém minérios, plantações, itens, criaturas e água como no Minecraft (só blocos do cenário
ficam realistas) e troca a descrição do `pack.mcmeta`. O ZIP não vai para o
git (passa de 100 MB): rode esta ferramenta antes do `tools/package.py`.

    python tools/build_realistic_textures.py            # 128x
    python tools/build_realistic_textures.py --size 64  # mais leve ainda

O download é conferido pelo SHA-512 que o Modrinth publica. Para medir o brilho original, a
ferramenta baixa também o client.jar do 26.3 da Mojang (conferido pelo SHA-1 da Mojang) e só lê
as médias; nenhuma textura da Mojang entra no pacote.
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
VANILLA_URL = ("https://piston-data.mojang.com/v1/objects/"
               "e877b6a07acd633fb3bb475002175cec036e7b87/client.jar")
VANILLA_SHA1 = "e877b6a07acd633fb3bb475002175cec036e7b87"
# Expoente da curva de luz (em linear): 0.35 clareia o cascalho do JG até o do jogo.
GAIN_RANGE = (0.3, 2.5)
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
    # Folhas: os furos finos do 256x viram chuvisco de céu a distância; ficam como no jogo.
    "leaves",
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
{size}x, brilho médio de cada textura acertado com o do Minecraft original, só texturas de
blocos do cenário mantidas (sem minérios, plantações, folhas, itens, criaturas e água) e descrição do pack.mcmeta trocada. Esta adaptação segue a mesma licença.
Uso não comercial.
"""


def download(cache, url=URL, digest=SHA512, algorithm="sha512"):
    check = lambda data: hashlib.new(algorithm, data).hexdigest() == digest
    if cache.is_file() and check(cache.read_bytes()):
        return cache.read_bytes()
    print(f"Baixando {url} ...")
    data = urllib.request.urlopen(url, timeout=600).read()
    if not check(data):
        raise SystemExit(f"O download de {url} não confere com o hash publicado; nada foi gerado.")
    cache.parent.mkdir(parents=True, exist_ok=True)
    cache.write_bytes(data)
    return data


TO_LINEAR = [((i / 255) / 12.92) if i <= 10 else (((i / 255) + 0.055) / 1.055) ** 2.4 for i in range(256)]


def to_srgb(v):
    v = min(max(v, 0.0), 1.0)
    return round(255 * (v * 12.92 if v <= 0.0031308 else 1.055 * v ** (1 / 2.4) - 0.055))


def mean_light(image):
    """Luminância linear média dos pixels visíveis (alfa acima da metade)."""
    rgba = image.convert("RGBA")
    mask = rgba.getchannel("A").point(lambda a: 255 if a > 127 else 0)
    total = 0.0
    for channel, weight in zip(rgba.split()[:3], (0.2126, 0.7152, 0.0722)):
        hist = channel.histogram(mask=mask)
        count = sum(hist)
        if count == 0:
            return 0.0
        total += weight * sum(h * TO_LINEAR[i] for i, h in enumerate(hist)) / count
    return total


def match_light(image, target):
    """Curva de luz (expoente em linear, sem estourar o branco) até a média bater com a do jogo.

    Uma curva e não um ganho: a pedra do JG precisa de quase 3x de luz, e um ganho assim
    queimaria os claros. O expoente clareia os escuros e guarda os detalhes.
    """
    current = mean_light(image)
    # Muito longe (fora de 1/6..6x em luz linear) é desenho diferente do original (caldeirão, funil): fica.
    if current < 0.003 or target < 0.003 or not 1 / 6 < target / current < 6.0:
        return image
    rgba = image.convert("RGBA")
    bands = rgba.split()

    def curved(power):
        lut = [to_srgb(TO_LINEAR[i] ** power) for i in range(256)]
        return Image.merge("RGBA", [b.point(lut) for b in bands[:3]] + [bands[3]])

    low, high = GAIN_RANGE  # expoente: abaixo de 1 clareia, acima escurece
    for _ in range(14):
        power = (low * high) ** 0.5
        if mean_light(curved(power)) < target:
            high = power
        else:
            low = power
    adjusted = curved((low * high) ** 0.5)
    return adjusted if image.mode == "RGBA" else adjusted.convert(image.mode)


def shrink(name, data, factor, vanilla):
    image = Image.open(io.BytesIO(data))
    width, height = image.size
    data_map = name.endswith(("_n.png", "_s.png"))
    target = None if data_map else vanilla.get(name)
    resize = not (factor == 1 or width < 32 or height < 32 or width % factor or height % factor)
    if not resize and target is None:
        return data
    mode = image.mode
    if mode not in ("RGB", "RGBA", "L", "LA"):
        image = image.convert("RGBA")
    # Mapas de dados (_n, _s) com média simples; a cor com Lanczos, que guarda a nitidez.
    if resize:
        image = image.resize((width // factor, height // factor),
                             Image.Resampling.BOX if data_map else Image.Resampling.LANCZOS)
    if target is not None and image.mode in ("RGB", "RGBA"):
        image = match_light(image, target)
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
    jar = zipfile.ZipFile(io.BytesIO(download(args.cache.parent / "client-26.3.jar",
                                              VANILLA_URL, VANILLA_SHA1, "sha1")))
    # Só a média de brilho de cada textura original; as imagens da Mojang não são copiadas.
    vanilla = {}
    for name in jar.namelist():
        if name.startswith(BLOCKS) and name.endswith(".png") and name in source.NameToInfo:
            vanilla[name] = mean_light(Image.open(io.BytesIO(jar.read(name))))
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
                data = shrink(info.filename, data, factor, vanilla)
                count += 1
            out.writestr(info.filename, data, compress_type=zipfile.ZIP_STORED if info.filename.endswith(".png") else zipfile.ZIP_DEFLATED)
        out.writestr("LEIA-ME-LICENCA.txt", NOTICE.format(size=args.size))
    size = target.stat().st_size / 1e6
    print(f"{target.relative_to(ROOT)}: {count} texturas em {args.size}x, {size:.0f} MB")


if __name__ == "__main__":
    sys.exit(main())
