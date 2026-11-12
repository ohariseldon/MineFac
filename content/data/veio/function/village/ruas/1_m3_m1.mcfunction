# Gerado por tools/vila/build_village.py. Ruas do estágio 1 (terra com mato), relativas ao centro da vila.
fill ~-68 ~-8 ~-32 ~-68 ~24 ~-32 minecraft:air replace #minecraft:logs
execute positioned ~-68 ~ ~-32 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:coarse_dirt replace #veio:road_ground
execute positioned ~-68 ~ ~-32 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
fill ~-67 ~-8 ~-32 ~-67 ~24 ~-32 minecraft:air replace #minecraft:logs
execute positioned ~-67 ~ ~-32 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:coarse_dirt replace #veio:road_ground
execute positioned ~-67 ~ ~-32 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
fill ~-66 ~-8 ~-32 ~-66 ~24 ~-32 minecraft:air replace #minecraft:logs
execute positioned ~-66 ~ ~-32 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:dirt_path replace #veio:road_ground
execute positioned ~-66 ~ ~-32 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
