# Gerado por tools/vila/build_village.py. Não edite à mão.
# Poço: rode com `execute positioned <canto> <piso> <canto> run function veio:village/poco_4`.
fill ~0 ~1 ~0 ~3 ~5 ~3 minecraft:air
fill ~0 ~0 ~0 ~3 ~0 ~3 minecraft:grass_block
kill @e[type=minecraft:item,x=~0,y=~-1,z=~0,dx=4,dy=7,dz=4]
fill ~-2 ~0 ~-2 ~5 ~12 ~5 minecraft:air replace #minecraft:logs
fill ~-2 ~0 ~-2 ~5 ~12 ~5 minecraft:air replace #minecraft:leaves
fill ~0 ~1 ~0 ~3 ~6 ~3 minecraft:air
fill ~0 ~0 ~0 ~3 ~0 ~3 minecraft:grass_block replace #minecraft:replaceable
fill ~0 ~-6 ~0 ~3 ~-1 ~3 minecraft:dirt replace #minecraft:replaceable
fill ~0 ~-4 ~0 ~3 ~-1 ~0 minecraft:cobblestone replace minecraft:dirt
fill ~0 ~-4 ~3 ~3 ~-1 ~3 minecraft:cobblestone replace minecraft:dirt
fill ~0 ~-4 ~0 ~0 ~-1 ~3 minecraft:cobblestone replace minecraft:dirt
fill ~3 ~-4 ~0 ~3 ~-1 ~3 minecraft:cobblestone replace minecraft:dirt
fill ~1 ~0 ~1 ~2 ~0 ~1 minecraft:water
fill ~1 ~0 ~2 ~2 ~0 ~2 minecraft:water
fill ~0 ~1 ~0 ~3 ~1 ~0 minecraft:stone_bricks
setblock ~0 ~1 ~1 minecraft:stone_bricks
setblock ~3 ~1 ~1 minecraft:stone_bricks
setblock ~0 ~1 ~2 minecraft:stone_bricks
setblock ~3 ~1 ~2 minecraft:stone_bricks
fill ~0 ~1 ~3 ~3 ~1 ~3 minecraft:stone_bricks
setblock ~3 ~2 ~0 minecraft:oak_fence[north=false,south=false,east=false,west=false]
setblock ~1 ~2 ~2 minecraft:lantern[hanging=true]
setblock ~0 ~2 ~3 minecraft:oak_fence[north=false,south=false,east=false,west=false]
setblock ~3 ~3 ~0 minecraft:oak_fence[north=false,south=false,east=false,west=false]
setblock ~1 ~3 ~2 minecraft:oak_fence[north=false,south=false,east=false,west=false]
setblock ~0 ~3 ~3 minecraft:oak_fence[north=false,south=false,east=false,west=false]
fill ~0 ~4 ~0 ~3 ~4 ~0 minecraft:oak_slab[type=bottom]
fill ~0 ~4 ~1 ~3 ~4 ~1 minecraft:oak_slab[type=bottom]
fill ~0 ~4 ~2 ~3 ~4 ~2 minecraft:oak_slab[type=bottom]
fill ~0 ~4 ~3 ~3 ~4 ~3 minecraft:oak_slab[type=bottom]
