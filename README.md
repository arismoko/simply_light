# Simply Light for Sunrise

160 light blocks in every dye colour: lamps, panels, slabs, bulbs, rods, posts and edge lights.

A port of [Simply Light](https://github.com/Flanks255/simplylight) by Flanks255 to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3). The blocks are content; the illuminant blocks' redstone switch is a small native part.

## What it does

- Ten families in 16 colours: illuminant blocks, slabs and panels, wall lamps, light bulbs, rod lamps, lamp posts and edge lights.
- Illuminant blocks switch with redstone as in the original: a plain one lights when powered, an inverted one goes dark, and redstone dust turns toward them.
- Panels, slabs, wall lamps and bulbs face the way you place them and can be waterlogged.
- The original's recipes (including off and on toggles), tags and creative tab.

## Differences from the original

- An illuminant block switches one tick after its signal changes, not in the same block update.
- The lamp post is placed as one block, not as its three-block column, and the edge-light editor and the blocks' tooltips are not ported.

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK) and Rust:

```sh
cargo sunrise-mod build    # compiles against the pinned game, packs target/sunrise-mod/simplylight.smod
cargo sunrise-mod run      # builds it and starts the game with it
cargo test
```

`sunrise-mod.toml` pins the Sunrise Dev build `dev-2026-11-22-7cfdf81` (game commit `7cfdf81463`). Sunrise builds native mods from source on the player's computer, against the exact game build.

## Licence and credits

Simply Light is by Flanks255 (https://github.com/Flanks255/simplylight). Its textures, models, recipes and strings are Flanks255's and are used here under the **MIT licence**, the original's; `LICENSE` holds the text and its copyright notice, and `LICENSE-NOTICE.md` says what this port carries.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/simply_light](https://github.com/AdLf0208/simply_light)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
