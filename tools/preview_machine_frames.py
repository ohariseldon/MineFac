"""Folha de conferência das máquinas grandes: núcleo + estrutura, em perspectiva.

Precisa do Pillow. Usa as mesmas cores aproximadas de `preview_machine_forms.py`: é uma
prévia da geometria, não uma captura do jogo.

    python3 tools/preview_machine_frames.py            # grava .build/visual/machine-frames.png
"""
import argparse
from pathlib import Path
from PIL import Image, ImageDraw
from preview_machine_forms import render, font, PALETTE
from machine_forms import form
from build_content import BLOCKS, box, model as block_model
import machine_frames as frames

ROOT = Path(__file__).resolve().parents[1]
PALETTE.update({"copper_block": "#c07a56", "polished_andesite": "#868b89", "deepslate_tiles": "#37373a",
                "black_concrete": "#141519", "blast_furnace_front_on": "#d9792c", "spruce_planks": "#73553a",
                "yellow_terracotta": "#b98a24", "polished_deepslate": "#45494e",
                "light_gray_concrete": "#8e8e86", "gray_concrete": "#3e4247", "weathered_cut_copper": "#6c9a7b",
                "redstone_lamp_on": "#f2c27a", "polished_basalt_side": "#5a5a5e"})
NAMES = {"drill": "Extratora", "drill_mk2": "Extratora Mk.II", "drill_mk3": "Extratora Mk.III",
         "press": "Prensa", "assembler": "Montadora", "cutter": "Cortadora", "integrator": "Integradora",
         "crusher": "Britador", "alloy_furnace": "Fundição de liga", "electric_furnace": "Forno elétrico",
         "electronics": "Eletrônica", "mill": "Moinho", "kitchen": "Cozinha", "fabricator": "Fabricadora", "manufacturer": "Combinadora", "paint_mixer": "Misturador de tintas", "paint_booth": "Cabine de pintura",
         "generator": "Gerador", "biomass_generator": "Gerador a biomassa", "steam_engine": "Máquina a vapor",
         "geothermal": "Geotérmica", "research_lab": "Laboratório"}


def combined(name):
    """Um modelo só com o núcleo e as peças, deslocadas em pixels."""
    kind, tier = BLOCKS[name][0], BLOCKS[name][1]
    core_textures, core_elements = block_model(kind, name, tier)
    core = {"textures": core_textures, "elements": core_elements}
    p = frames.plan(name)
    texture_sets = {"core": core["textures"], "frame": frames.textures(p["tier"])}
    elements = [dict(e, faces={k: {"texture": "@core" + v["texture"]} for k, v in e["faces"].items()})
                for e in core["elements"]]
    for (dx, dy, dz), kind, piece in p["cells"]:
        parts = (p["pieces"][piece] if kind == frames.STRUCTURE
                 else frames.hatch_elements(kind, box))
        for e in parts:
            elements.append({"from": [e["from"][0] + dx*16, e["from"][1] + dy*16, e["from"][2] + dz*16],
                             "to": [e["to"][0] + dx*16, e["to"][1] + dy*16, e["to"][2] + dz*16],
                             "faces": {k: {"texture": "@frame" + v["texture"]} for k, v in e["faces"].items()}})
    # Resolve as texturas de cada conjunto para nomes finais.
    flat = {}
    for e in elements:
        for face in e["faces"].values():
            tag = face["texture"]
            group, key = ("core", tag[5:]) if tag.startswith("@core") else ("frame", tag[6:])
            textures = texture_sets[group]
            for _ in range(8):
                if not key.startswith("#"): break
                key = textures.get(key[1:], "minecraft:block/iron_block")
            face["texture"] = key
    return {"textures": flat, "elements": elements}, p


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", default=str(ROOT / ".build/visual/machine-frames.png"))
    parser.add_argument("--only", nargs="*")
    parser.add_argument("--zoom", type=float, default=1.0)
    args = parser.parse_args()
    frames.compile_all(box)
    names = args.only or list(frames.LAYOUTS)
    cols = 4
    cell_w, cell_h = int(420 * args.zoom), int(420 * args.zoom)
    rows = (len(names) + cols - 1) // cols
    sheet = Image.new("RGBA", (cols*cell_w, rows*cell_h + 60), "#f2efe8")
    draw = ImageDraw.Draw(sheet)
    draw.text((20, 18), "Veio — máquinas grandes (prévia de geometria, cores aproximadas)", fill="#2b2f33", font=font(22))
    for index, name in enumerate(names):
        model, p = combined(name)
        w, h, d = p["size"]
        span = max(w, d, h*0.9)
        scale = 3.4 if span <= 2 else 2.6 if span <= 3 else 2.1 if span <= 4 else 1.75
        scale *= args.zoom
        tile = Image.new("RGBA", (cell_w, cell_h), "#f2efe8")
        render(tile, model, (cell_w*0.5, cell_h*0.62), scale)
        ox, oy = (index % cols)*cell_w, 60 + (index // cols)*cell_h
        sheet.paste(tile, (ox, oy))
        draw.text((ox + 16, oy + 8), NAMES.get(name, name), fill="#1f2326", font=font(20))
        draw.text((ox + 16, oy + 34), f"{w} × {d} de chão, {h} de altura", fill="#5a6066", font=font(15))
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    sheet.convert("RGB").save(out)
    print(out)


if __name__ == "__main__":
    main()
