# Artwork

The optional **Veio — Realismo leve 1.1** shader in `visuals/veio-industrial` is original WGSL under its included MIT license. Its industrial visual direction is inspired by Satisfactory; it includes no code, art or assets from Satisfactory or other shader packs. Its material lighting, weather, analytical sky and depth-based water use the documented Horizon shader API 0.1 and existing renderer resources.

Since Veio 0.7, every texture in `content/assets/veio/textures` is original 16x16 pixel art made for this project. The drawings live as text in `tools/build_textures.py`, which writes the PNG files.

The block models also reference textures from Minecraft itself, such as `minecraft:block/iron_block`, `minecraft:block/cut_copper` and `minecraft:block/daylight_detector_top`. Those files are not distributed here: the game resolves them from the player's own installation.

Veio 0.4 to 0.6 used sprites from **Platformer Pack: Industrial** by Kenney (CC0). They were replaced in 0.7 to match Minecraft's style and are no longer included.

# Audio

Machine feedback uses recordings from **Kenney**, under **CC0 1.0**:

- [Impact Sounds](https://kenney.nl/assets/impact-sounds): metal/wood impacts and grass rustles.
- [RPG Audio](https://kenney.nl/assets/rpg-audio): cloth, metal utensils and blade scrapes.
- [Sci-Fi Sounds](https://kenney.nl/assets/sci-fi-sounds): motor, mechanism and electronic textures.

Selected originals, unmodified license texts and SHA-256 provenance are in
`third_party/audio/kenney/`. Veio's derivatives in `content/assets/veio/sounds/machines/`
are trimmed, converted to mono, filtered, pitched, faded and sometimes arranged
as repeated mechanical impacts by `tools/build_machine_audio.py`. They retain
CC0 permissions. No Minecraft sound recordings are redistributed.

## Texturas realistas (Realismo)

`visuals/texturas/Veio-Texturas-JG-128x.zip` é uma adaptação de **PBR JG** 1.1.0, de Jason J. Gardner
([Modrinth](https://modrinth.com/resourcepack/pbr-jg), [GitHub](https://github.com/jasonjgardner/jg-rtx)),
sob [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). O Veio reduz as texturas de
256x para 128x e usa só as de blocos do cenário (sem minérios, plantações, itens, criaturas e água) (`tools/build_realistic_textures.py`); a adaptação segue a mesma licença, para uso não comercial.
