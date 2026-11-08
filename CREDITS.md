# Artwork

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
