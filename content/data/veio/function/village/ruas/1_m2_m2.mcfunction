# Gerado por tools/vila/build_village.py. Ruas do estágio 1 (terra com mato), relativas ao centro da vila.
fill ~-39 ~-8 ~-34 ~-39 ~24 ~-34 minecraft:air replace #minecraft:logs
execute positioned ~-39 ~ ~-34 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:dirt_path replace #veio:road_ground
execute positioned ~-39 ~ ~-34 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
fill ~-38 ~-8 ~-36 ~-38 ~24 ~-36 minecraft:air replace #minecraft:logs
execute positioned ~-38 ~ ~-36 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:dirt_path replace #veio:road_ground
execute positioned ~-38 ~ ~-36 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
