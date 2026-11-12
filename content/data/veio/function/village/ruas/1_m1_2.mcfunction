# Gerado por tools/vila/build_village.py. Ruas do estágio 1 (terra com mato), relativas ao centro da vila.
fill ~-2 ~-8 ~64 ~-2 ~24 ~64 minecraft:air replace #minecraft:logs
execute positioned ~-2 ~ ~64 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:gravel replace #veio:road_ground
execute positioned ~-2 ~ ~64 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
fill ~-2 ~-8 ~65 ~-2 ~24 ~65 minecraft:air replace #minecraft:logs
execute positioned ~-2 ~ ~65 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:dirt_path replace #veio:road_ground
execute positioned ~-2 ~ ~65 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
fill ~-1 ~-8 ~64 ~-1 ~24 ~64 minecraft:air replace #minecraft:logs
execute positioned ~-1 ~ ~64 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:dirt_path replace #veio:road_ground
execute positioned ~-1 ~ ~64 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
fill ~-1 ~-8 ~65 ~-1 ~24 ~65 minecraft:air replace #minecraft:logs
execute positioned ~-1 ~ ~65 positioned over motion_blocking_no_leaves run fill ~ ~-1 ~ ~ ~-1 ~ minecraft:gravel replace #veio:road_ground
execute positioned ~-1 ~ ~65 positioned over motion_blocking_no_leaves run fill ~ ~ ~ ~ ~1 ~ minecraft:air replace #minecraft:replaceable_by_trees
