"""Render a geometry review sheet. Pillow is optional, only used by this tool.

Own pixel-art textures are sampled; Minecraft materials use approximate colors.
This is an asset preview, not a screenshot or a Horizon lighting simulation.
"""
import argparse
import json
from pathlib import Path
import subprocess
from PIL import Image, ImageDraw, ImageFont, ImageEnhance
from machine_forms import GROUPS, form
from build_content import BLOCKS, box

ROOT = Path(__file__).resolve().parents[1]
PALETTE = {"cut_copper": "#b97855", "iron_block": "#b0b5b5", "polished_deepslate": "#41464b",
           "smooth_stone": "#aaa9a3", "smooth_stone_slab_side": "#95958e", "barrel_side": "#795a3a",
           "barrel_top": "#916c43", "stripped_spruce_log": "#6a5036", "green_terracotta": "#626d3a",
           "blue_terracotta": "#515b70", "red_terracotta": "#925447", "tinted_glass": "#555465",
           "bricks": "#875c4e", "furnace_top": "#747779"}


def font(size):
    try: return ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", size)
    except OSError: return ImageFont.load_default()


def texture(model, name):
    for _ in range(8):
        if not name.startswith("#"): break
        name = model["textures"].get(name[1:], "minecraft:block/iron_block")
    if name.startswith("veio:"):
        file = ROOT / "content/assets/veio/textures" / (name[5:] + ".png")
        if file.exists(): return Image.open(file).convert("RGBA")
    return Image.new("RGBA", (16, 16), PALETTE.get(name.split("/")[-1], "#747b7f"))


def render(canvas, model, origin, scale=4.0):
    def project(p):
        x, y, z = p
        return (origin[0] + (x-z)*scale*.8, origin[1] + (x+z)*scale*.4-y*scale)
    faces = []
    for element in model["elements"]:
        a,b,c = element["from"]; d,e,f = element["to"]
        options = {
            "south": ([(a,e,f),(d,e,f),(d,b,f),(a,b,f)], .95),
            "east": ([(d,e,f),(d,e,c),(d,b,c),(d,b,f)], .70),
            "up": ([(a,e,c),(d,e,c),(d,e,f),(a,e,f)], 1.12),
        }
        for direction,(points, shade) in options.items():
            if direction not in element["faces"]: continue
            depth = sum(x+z+y*.8 for x,y,z in points)/4
            faces.append((depth, points, texture(model, element["faces"][direction]["texture"]), shade))
    draw = ImageDraw.Draw(canvas)
    ground = [project(p) for p in [(0,0,0),(16,0,0),(16,0,16),(0,0,16)]]
    draw.polygon(ground, fill="#d7d4cb", outline="#bbb7ad")
    for _, points, tex, shade in sorted(faces, key=lambda f:f[0]):
        p = [project(v) for v in points]
        left, top = int(min(v[0] for v in p)), int(min(v[1] for v in p))
        right, bottom = int(max(v[0] for v in p))+2, int(max(v[1] for v in p))+2
        if right <= left or bottom <= top: continue
        q = [(x-left,y-top) for x,y in p]
        ux,uy = q[1][0]-q[0][0],q[1][1]-q[0][1]
        vx,vy = q[3][0]-q[0][0],q[3][1]-q[0][1]
        determinant = ux*vy-vx*uy
        if abs(determinant)<.01: continue
        w,h=tex.size
        coefficients=(w*vy/determinant,-w*vx/determinant,w*(-vy*q[0][0]+vx*q[0][1])/determinant,
                      -h*uy/determinant,h*ux/determinant,h*(uy*q[0][0]-ux*q[0][1])/determinant)
        patch=tex.transform((right-left,bottom-top),Image.Transform.AFFINE,coefficients,Image.Resampling.NEAREST)
        patch=ImageEnhance.Brightness(patch).enhance(shade)
        mask=Image.new("L",patch.size); ImageDraw.Draw(mask).polygon(q,fill=255)
        canvas.paste(patch,(left,top),mask)
        draw.line(p+[p[0]],fill="#4f5559",width=1)


def pose(name, profile):
    """Approximate resting working parts, for geometry review only."""
    model={"textures":profile["textures"],"elements":list(profile["elements"])}
    def add(a,b): model["elements"].append(box(a,b,"#metal"))
    if profile["motion"] == "Press": add([5,11,5],[11,13,11]); add([7,13,7],[9,14,9])
    elif profile["motion"] == "Drill": add([7,3,7],[9,13,9]); add([5.5,3,6.5],[10.5,3.6,9.5])
    elif profile["motion"] == "Saw": add([4,8,7.5],[12,10,8.5]); add([7,5,7.5],[9,13,8.5]); add([5,6,7.5],[11,12,8.5])
    elif profile["motion"] in ("Turntable","Roller"):
        x,y,z=profile["center"];add([x-3,y,z-3],[x+3,y+.6,z+3])
    return model


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before-ref", help="Git ref for original models")
    parser.add_argument("--out",type=Path,default=ROOT/".build/visual")
    args=parser.parse_args();args.out.mkdir(parents=True,exist_ok=True)
    names=list(GROUPS)
    language=json.loads((ROOT/"content/assets/veio/lang/pt_br.json").read_text())
    def title(name):return language.get("block.veio."+name,name)
    featured=["press","cutter","crusher","drill","electronics","alloy_furnace","steam_engine","geothermal","mill","harvester","barn","caravan_station"]
    sheet=Image.new("RGB",(1320,1070),"#efeee9");d=ImageDraw.Draw(sheet)
    d.text((32,18),"VEIO  /  Identidade das máquinas",font=font(27),fill="#273338")
    d.text((32,56),"Prévia de geometria • materiais vanilla com cores aproximadas • não é captura do jogo",font=font(15),fill="#656d70")
    before={}
    if args.before_ref:
        for name in featured:
            result=subprocess.run(["git","show",f"{args.before_ref}:content/assets/veio/models/block/{name}.json"],cwd=ROOT,check=True,capture_output=True,text=True)
            before[name]=json.loads(result.stdout)
    elif (args.out/"before.json").exists():before=json.loads((args.out/"before.json").read_text())
    for i,name in enumerate(featured):
        x,y=20+(i%3)*440,93+(i//3)*240
        d.rounded_rectangle((x,y,x+420,y+226),radius=10,fill="#ffffff")
        d.text((x+16,y+12),title(name),font=font(18),fill="#273338")
        d.text((x+58,y+39),"ANTES",font=font(12),fill="#777f80")
        d.text((x+258,y+39),"AGORA",font=font(12),fill="#6a7c4d")
        if name in before:render(sheet,before[name],(x+110,y+147),4.1)
        render(sheet,pose(name,form(name,BLOCKS[name][1],box)),(x+310,y+147),4.1)
    sheet.save(args.out/"machines-before-after.png")
    rows=(len(names)+5)//6
    catalog=Image.new("RGB",(1440,80+rows*230),"#efeee9");d=ImageDraw.Draw(catalog)
    d.text((25,18),f"VEIO  /  {len(names)} modelos • um ponto de colocação",font=font(25),fill="#273338")
    for i,name in enumerate(names):
        x,y=10+(i%6)*240,70+(i//6)*230
        d.rounded_rectangle((x,y,x+228,y+218),radius=8,fill="#ffffff")
        d.text((x+12,y+11),title(name),font=font(14),fill="#273338")
        render(catalog,pose(name,form(name,BLOCKS[name][1],box)),(x+114,y+134),4.6)
    catalog.save(args.out/"machine-catalog.png")
    print(args.out/"machines-before-after.png");print(args.out/"machine-catalog.png")


if __name__ == "__main__":main()
