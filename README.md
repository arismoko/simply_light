# Simply Light for Sunrise

160 light blocks in every dye colour: lamps, panels, slabs, bulbs, rods, posts and edge lights.

A port of [Simply Light](https://github.com/Flanks255/simplylight) by Flanks255 to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3). The blocks are content; the illuminant blocks' redstone switch is a small native part.

## What it does

- A lamp post is three blocks tall and only its top block emits light; hover a light and hold Shift for its detail text; every light can be worn on the head; new players get the mod's recipes.
- Sneak-click an edge light to open its side editor: four switches, one per side, written to the block as you press them.
- Ten families in 16 colours: illuminant blocks, slabs and panels, wall lamps, light bulbs, rod lamps, lamp posts and edge lights.
- Illuminant blocks switch with redstone as in the original: a plain one lights when powered, an inverted one goes dark, and redstone dust turns toward them.
- Panels, slabs, wall lamps and bulbs face the way you place them and can be waterlogged.
- The original's recipes (including off and on toggles), tags and creative tab.

## Differences from the original

- The optional "Simply Light Full block CTM" resource pack is bundled: turn it on in Resource Packs and neighbouring illuminant blocks of the same kind join into one panel. The original's pack was written for the Connected Textures mod; here the same pictures are in OptiFine's connected-texture format, which Sunrise reads (`tools/gen_ctm_pack.py`).
- A lamp post is placed and removed as its three-block column; its partner blocks are removed at the end of the tick the post is broken ([sunrise-org/sunrise#5286](https://github.com/sunrise-org/sunrise/issues/5286)).
- The server edits an edge light from the side editor only when the player is within 8 blocks of it; the original accepts any position.

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK) and Rust:

```sh
cargo sunrise-mod build    # compiles against the pinned game, packs target/sunrise-mod/simplylight.smod
cargo sunrise-mod run      # builds it and starts the game with it
cargo test
```

`sunrise-mod.toml` pins the Sunrise Dev build `dev-2026-11-25-bdad86d` (game commit `bdad86d2d3`). Sunrise builds native mods from source on the player's computer, against the exact game build.

## Licence and credits

Simply Light is by Flanks255 (https://github.com/Flanks255/simplylight). Its textures, models, recipes and strings are Flanks255's and are used here under the **MIT licence**, the original's; `LICENSE` holds the text and its copyright notice, and `LICENSE-NOTICE.md` says what this port carries.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/simply_light](https://github.com/AdLf0208/simply_light)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
