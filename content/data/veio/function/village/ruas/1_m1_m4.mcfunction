# Gerado por tools/vila/build_village.py. Ruas do estágio 1 (terra com mato), relativas ao centro da vila.
fill ~-11 ~-8 ~-97 ~-11 ~24 ~-97 minecraft:air replace #minecraft:logs
execute positioned ~-11 ~ ~-97 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:coarse_dirt replace #veio:road_ground
execute positioned ~-11 ~ ~-97 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
fill ~-10 ~-8 ~-97 ~-10 ~24 ~-97 minecraft:air replace #minecraft:logs
execute positioned ~-10 ~ ~-97 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:dirt_path replace #veio:road_ground
execute positioned ~-10 ~ ~-97 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
