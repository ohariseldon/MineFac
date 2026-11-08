# Artwork

The optional **Veio — Realismo leve 1.1** shader in `visuals/veio-industrial` is original WGSL under its included MIT license. Its industrial visual direction is inspired by Satisfactory; it includes no code, art or assets from Satisfactory or other shader packs. Its material lighting, weather, analytical sky and depth-based water use the documented Horizon shader API 0.1 and existing renderer resources.

Since Veio 0.7, every texture in `content/assets/veio/textures` is original 16x16 pixel art made for this project. The drawings live as text in `tools/build_textures.py`, which writes the PNG files.

The block models also reference textures from Minecraft itself, such as `minecraft:block/iron_block`, `minecraft:block/cut_copper` and `minecraft:block/daylight_detector_top`. Those files are not distributed here: the game resolves them from the player's own installation.

Veio 0.4 to 0.6 used sprites from **Platformer Pack: Industrial** by Kenney (CC0). They were replaced in 0.7 to match Minecraft's style and are no longer included.
