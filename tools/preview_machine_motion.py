"""Render eight poses captured by the real Veio callbacks in the offline probe.
First export: VEIO_MOTION_PREVIEW=$PWD/.build/machine-motion.json python tools/check_rust.py
This is a geometry study, not Horizon lighting, textures or an in-game recording.
Requires Pillow only for this optional preview.
"""
import itertools
import json
import math
from pathlib import Path
from PIL import Image, ImageDraw
from preview_machine_forms import font, PALETTE

ROOT=Path(__file__).resolve().parents[1]


def rotate(p, axis, angle, pivot):
    a=math.radians(angle); c=math.cos(a); s=math.sin(a)
    q=[p[i]-pivot[i] for i in range(3)]
    cross=[axis[1]*q[2]-axis[2]*q[1],axis[2]*q[0]-axis[0]*q[2],axis[0]*q[1]-axis[1]*q[0]]
    dot=sum(q[i]*axis[i] for i in range(3))
    return [pivot[i]+q[i]*c+cross[i]*s+axis[i]*dot*(1-c) for i in range(3)]


def transforms(ops,time):
    current=[]; stack=[]
    for op in ops:
        if op[0]=='push': stack.append(current[:])
        elif op[0]=='pop': current=stack.pop()
        elif op[0]=='still': current=[o for o in current if o[0]!='spin']
        else: current.append(op)
    def apply(p):
        for op in reversed(current):
            if op[0]=='translate': p=[p[i]+op[1][i] for i in range(3)]
            elif op[0]=='scale': p=[p[i]*op[1][i] for i in range(3)]
            elif op[0]=='rotate': p=rotate(p,op[1],op[2],op[3])
            elif op[0]=='spin': p=rotate(p,op[1],op[3]*time+op[4],op[2])
        return [n*16 for n in p]
    return apply


def vertices(lo,hi): return list(itertools.product(*[(lo[i]/16,hi[i]/16) for i in range(3)]))


def base(name):
    model=json.loads((ROOT/f'content/assets/veio/models/block/{name}.json').read_text())
    parts=[]
    for element in model.get('elements',[]):
        face=next(iter(element['faces'].values()))
        tex=face.get('texture','#body')
        for _ in range(6):
            if not tex.startswith('#'): break
            tex=model.get('textures',{}).get(tex[1:],'minecraft:block/iron_block')
        color=PALETTE.get(tex.split('/')[-1],'#747c78')
        parts.append(([tuple(n*16 for n in p) for p in vertices(element['from'],element['to'])],color))
    return parts


def paint(canvas,parts,origin,scale=3.4):
    faces=[]
    indices=[(2,6,7,3),(1,5,7,3),(4,6,7,5),(0,2,6,4),(0,1,3,2),(0,4,5,1)]
    for points,color in parts:
        center=[sum(p[i] for p in points)/8 for i in range(3)]
        if isinstance(color,int): color=f'#{color:06x}'
        rgb=tuple(int(color[i:i+2],16) for i in (1,3,5))
        for inds in indices:
            face=[points[i] for i in inds]
            mid=[sum(p[i] for p in face)/4 for i in range(3)]
            normal=[mid[i]-center[i] for i in range(3)]
            if normal[0]+normal[2]+normal[1]*.8<=0: continue
            depth=mid[0]+mid[2]+mid[1]*.8
            shade=.8 if abs(normal[0])>abs(normal[1])+abs(normal[2]) else 1.08 if abs(normal[1])>abs(normal[2]) else .94
            fill=tuple(min(255,int(c*shade)) for c in rgb)
            poly=[(origin[0]+(p[0]-p[2])*scale*.8,origin[1]+(p[0]+p[2])*scale*.35-p[1]*scale) for p in face]
            faces.append((depth,poly,fill))
    draw=ImageDraw.Draw(canvas)
    for _,poly,fill in sorted(faces): draw.polygon(poly,fill=fill,outline='#333f39')


def main():
    machines=json.loads((ROOT/'.build/machine-motion.json').read_text())
    columns=6; width=252*columns; rows=math.ceil(len(machines)/columns)
    frames=[]
    for index in range(8):
        canvas=Image.new('RGB',(width,rows*282+110),'#edf0e8');d=ImageDraw.Draw(canvas)
        d.text((25,15),'Veio · movimento mecânico',font=font(25),fill='#253a2e')
        d.text((25,51),'8 poses capturadas dos callbacks · estudo de geometria, sem iluminação/texturas do Horizon',font=font(15),fill='#526458')
        d.text((width-135,22),f'Pose {index+1}/8',font=font(16),fill='#526458')
        for n,machine in enumerate(machines):
            x=(n%columns)*252; y=(n//columns)*282+90
            d.rounded_rectangle((x+8,y+3,x+244,y+274),radius=12,fill='#dfe5da')
            d.text((x+19,y+14),machine['name'],font=font(15),fill='#253a2e')
            parts=base(machine['name']); frame=machine['frames'][index]
            for cube in frame['cubes']:
                apply=transforms(cube['ops'],frame['time'])
                parts.append(([apply(p) for p in vertices(cube['lo'],cube['hi'])],cube['color']))
            paint(canvas,parts,(x+126,y+224))
        frames.append(canvas)
    output=ROOT/'.build/machine-motion.gif'
    frames[0].save(ROOT/'.build/machine-motion.png')
    frames[0].save(output,save_all=True,append_images=frames[1:],duration=200,loop=0,optimize=False)
    print(output)


if __name__=='__main__': main()
