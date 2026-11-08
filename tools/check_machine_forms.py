"""Check player-facing model contracts: placement, rotation and moving clearances."""
import json
from pathlib import Path
from machine_forms import GROUPS, form
from build_content import BLOCKS, box

ROOT = Path(__file__).resolve().parents[1]


def contains(outer, inner):
    return all(outer[i] <= inner[i] + 1e-6 and outer[i+3] >= inner[i+3] - 1e-6 for i in range(3))


def main():
    unique = set()
    # The outlet must stay opposite facing, at the existing north connection.
    outlets = {"south": [.375,.25,0,.625,.5,.125], "west": [.875,.25,.375,1,.5,.625],
               "north": [.375,.25,.875,.625,.5,1], "east": [0,.25,.375,.125,.5,.625]}
    for name in GROUPS:
        profile = form(name, BLOCKS[name][1], box)
        model = json.loads((ROOT/f"content/assets/veio/models/block/{name}.json").read_text())
        block = json.loads((ROOT/f"content/data/veio/horizon/block/{name}.json").read_text())
        for part in model["elements"]:
            assert all(0 <= v <= 16 for key in ("from", "to") for v in part[key]), name
        unique.add(json.dumps([(p["from"], p["to"]) for p in model["elements"]]))
        for field in ("shapes", "collision_shapes"):
            table = block[field]
            assert table["property"] == "facing" and set(table["values"]) == set(outlets), name
            for facing, expected in outlets.items():
                hits = table["values"][facing]
                assert 0 < len(hits) <= 16, name
                assert expected in hits, (name, facing, "connection moved")
                assert [0,0,0,1,1,1] not in hits, (name, "old invisible cube")
                assert all(0 <= value <= 1 for hit in hits for value in hit), name
        hits = block["collision_shapes"]["values"]["south"]
        lamp = [v/16 for v in profile["lamp"]]
        bx, by, bz = profile["badge"]
        badge = [bx/16,by/16,bz/16,(bx+2)/16,(by+2)/16,(bz+.15)/16]
        assert any(contains(hit,lamp) for hit in hits), (name,"light has no housing")
        assert any(contains(hit,badge) for hit in hits), (name,"badge is floating")
        # Flat indicator faces must not fight an opaque face at the same plane.
        for indicator in (profile["lamp"], [bx,by,bz,bx+2,by+2,bz+.15]):
            for part in model["elements"]:
                lo, hi = part["from"], part["to"]
                if "south" not in part["faces"] or abs(hi[2]-indicator[5]) > 1e-5: continue
                overlap = min(hi[0],indicator[3]) > max(lo[0],indicator[0]) and min(hi[1],indicator[4]) > max(lo[1],indicator[1])
                assert not overlap, (name,"indicator has an opaque coplanar face")
        motion = profile["motion"]
        sweep = {"Drill": [5,3,5,11,13,11], "Press": [5,7,5,11,14,11],
                 "Saw": [3.75,4.75,7.5,12.25,13.25,8.5],
                 "Roller": [4,7.5,5.5,12,12.5,10.5]}.get(motion)
        if motion == "Turntable":
            x,y,z = profile["center"]; sweep = [x-4.25,y,z-4.25,x+4.25,y+1.5,z+4.25]
        if sweep: assert any(contains(hit,[v/16 for v in sweep]) for hit in hits), (name,"moving part escapes collision")
    core = ["press","cutter","crusher","drill","electronics","integrator","mill"]
    silhouettes = {json.dumps([(p["from"],p["to"]) for p in form(name,BLOCKS[name][1],box)["elements"]]) for name in core}
    assert len(silhouettes) == len(core), "Core machines still share one silhouette"
    print(f"{len(GROUPS)} machine forms: placement, four facings, indicators and moving clearances verified; {len(unique)} distinct silhouettes.")


if __name__ == "__main__": main()
