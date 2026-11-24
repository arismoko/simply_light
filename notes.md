# Simply Light — port notes

Ported from **Simply Light 1.5.3** (`Flanks255/simplylight`, MIT) for Minecraft
Java 26.3. The **current campaign pin is `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`** /
horizon-mixins **`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`** (`api = "0.2"`,
game 26.3, package format 6); the delivered package is **content-only and names no
pin** (no `[native]`), and its evidence is measured at `0a875da` (0.2.1) and
`0b14f8a` (0.2.0). The retired `.hnmod` bundle was built at `1d63e0a5` /
`5a64af938ec86b00d53d16a408aabc0038fb3e57`; its evidence below is labelled
historical.

**Class + tier** (`ports/ledger.csv`; provenance
`ports/_shared/source-map/atm10-sources.csv`): **content / data**. The approach
is the tier's **carry-over** shape (`docs/modpacks.md` §"Registries 0.2: data-driven
blocks, items and mob effects"): the jar's `assets/` and `data/` upgraded to 26.3,
with the 160 blocks declared from vanilla templates in
`data/simplylight/horizon/block/` and one creative tab. There is **no production
code**: the port's only Rust is a test-only census model with zero hooks.

**Licence: MIT → rank 0.** The jar's own `META-INF/neoforge.mods.toml` declares
`license="MIT"` (`authors="Flanks255"`) and the upstream `LICENSE` is
`Copyright (c) 2018 Flanks255`. The port is **local-only until the maintainer
approves publishing** (ARCHITECTURE.md §8). It is being ported now because the
licence-first queue puts a rank-0 content carry first, and the mod is one of the
cleanest data-tier carries left.

> **Status: a content-only `kind = "mod"` package, version 0.2.1 (2026-11-21).** The delivered
> artifact is `simplylight.hmod`, **689,910 B**, SHA-256
> `7bbf9c31943b0641ff313178d0ce479426339879b528bdb47c959ef65b99118b`, 1,455 entries
> (`horizon-mod.toml`, `assets/`, `data/`) — **no `Cargo.toml`, no `src/`, no `[native]`**.
> It is measured in game at `0b14f8a` (the 0.2.0 state/light/drop/placement leg) and at
> `0a875da` (0.2.1, that leg's carried bytes plus the recipe/loot/waterlogging parity leg);
> evidence in `ports/_shared/evidence/simplylight-e2e/`. Read
> [`## Content-only 0.2.0`](#content-only-020--the-registrys-kindmod-rule-2026-11-19) first;
> it supersedes the native 0.2.0 package, which is kept byte for byte in
> `ports/_shared/evidence/simplylight/retired-native-0.2.0/` and is described by
> [`## Migration to the SDK format`](#migration-to-the-sdk-format-020-2026-11-18) (historical:
> that section's package hash and file map describe the retired native package).
> Everything from "Version and source" down to "Reviewer repairs" is the history of the
> retired `.hnmod` bundle at pin `1d63e0a5`.

Status before the migration: **implemented and verified in-game** at pin `1d63e0a5`
(2026-11-09); evidence in
[`ports/_shared/evidence/simplylight/`](../_shared/evidence/simplylight/).

## Version and source

- ATM10 1.21.1 jar: `simplylight-1.5.3+1.21.1-b4.jar`, **1,103,352 B**,
  SHA-1 `d85bfed3824cb66d258acf11c73edaf11d583223`, SHA-256
  `6d791f98671c1a5facc8f240c28944d0c7a2a6a635a5d0c5bf4d97a9f3b7a3a0`.
  Resolved from the pack's Crash Assistant export
  (`AllTheMods/ATM-10/config/crash_assistant/modlist.json`: the entry's
  `modrinthHash` is that SHA-1); the local copy matches. Kept as evidence in
  [`../_shared/carry-work/simplylight/`](../_shared/carry-work/simplylight/)
  with [`SOURCE.md`](../_shared/carry-work/simplylight/SOURCE.md).
- Upstream repo (analysis only): `https://github.com/Flanks255/simplylight`,
  `master` @ `1a465c7`, `gradle.properties` `minecraft_version=1.21.1`,
  `mod_version=1.5.3`. No 26.3 branch exists, so the jar is the data source and
  the port upgrades it (below). The clone was deleted after this section was
  written.

## Upstream analysis (behaviour, not code)

Files read: `SLBlocks.java`, `SimplyLight.java`, `blocks/{LampBase,
RotatableLamp,LampBlock,ThinLamp,Fixture,RodLamp,LightBulb,LampPost,EdgeLight,
BaseBlockItem,LampPostItem}.java`, `data/{BlockStates,SLBlockLoot,SLBlockTags,
SLItemTags,Recipes,LangGen}.java`, `README.md`, `LICENSE`. Behaviours extracted:

- **160 blocks** = 10 families × 16 colours, all registered with a block item;
  the creative tab order is on lamps, off lamps, slabs, panels, rods, bulbs,
  fixtures (wall lamps), posts, edge lights, top edge lights.
- `LampBlock` (`illuminant_block` / `illuminant_block_on`): the block id is the
  on/off variant; the `on` boolean, light 15 when on else 0, map colour black
  (off) / light gray (on); **redstone inverts** the variant
  (`neighborChanged`, `setPlacedBy`, `isSignalSource`).
- `ThinLamp` (panel thickness 4/16, slab 8/16), `Fixture` (wall lamp),
  `LightBulb` and `RodLamp`: `facing` (panels/wall/bulb) or `axis` (rod) ×
  `waterlogged`, light 15 (14 for the bulb), per-facing outline boxes; the
  panel/slab continue an adjacent thin lamp's `facing` when not sneaking;
  `hood` none of them drops differently.
- `LampPost`: `position` (bottom/middle/top) × `waterlogged`, placed as a
  **3-block column** (`setPlacedBy`), removed as a column (`onRemove`), light
  15 **only when `position == top`**.
- `EdgeLight`/`EdgeLightTop`: four side booleans × `waterlogged`, light 14,
  sides set once at placement from the neighbours' sturdy faces
  (`checkSide`), a sneak-use opens a client **edge editor GUI** (network
  packet, screen).
- Loot: `dropSelf` for every block; recipes: 234 total (block, bulb, edge,
  slab, panel, rod, wall lamp, lamp post, per-family dye recipes through the
  `simplylight:any_*` item tags, and off↔on toggle recipes); advancements are
  suppressed (`NoAdvRecipeOutput`); item tags: 10 `any_*` families; block
  tags: `minecraft:mineable/pickaxe` (all 160) and
  `minecraft:wall_post_override` (the white light bulb).
- Tooltips: `BaseBlockItem.appendHoverText` prints `block.simplylight.<id>.info*`
  on shift; the lamp-post item has attack-damage attributes; `BaseBlockItem`
  implements `Equipable` (HEAD).

## Scope: the 160 blocks

`horizon-port-pack check` carried 112 declarations (it guessed templates) and
**left out 48 blocks** whose states no vanilla template has — the 32
`illuminant_block*` (`on`) and the 16 `lamp_post` (`position`) — plus the 83
recipes that named them. `tools/gen_content.py` declares all 160 by hand from
the upstream source and re-shapes those recipes.

| Family | Count | Declared states | Template (26.3) | Light | Notes |
|---|---:|---:|---|---:|---|
| `illuminant_block` | 16 | 2 (`on`) | `minecraft:calcite` | **0** | stateless cube, stone sound, map 29 |
| `illuminant_block_on` | 16 | 2 (`on`) | `minecraft:calcite` | **15** | default `on=true`, map 22 |
| `illuminant_slab` | 16 | 12 (`facing` 6 × `waterlogged` 2) | `minecraft:amethyst_cluster` | 15 | 8/16-thick plate |
| `illuminant_panel` | 16 | 12 | `minecraft:amethyst_cluster` | 15 | 4/16-thick plate |
| `wall_lamp` | 16 | 12 | `minecraft:amethyst_cluster` | 15 | `noCollission()`, empty collision |
| `lightbulb` | 16 | 12 | `minecraft:amethyst_cluster` | **14** | `noCollission()` |
| `rodlamp` | 16 | 6 (`axis` 3 × `waterlogged`) | `minecraft:iron_chain` | 15 | map 36 |
| `lamp_post` | 16 | 6 (`position` 3 × `waterlogged`) | `minecraft:mangrove_roots` | 15 | scalar-light gap (below) |
| `edge_light` | 16 | 32 (4 sides × `waterlogged`) | `minecraft:oak_fence` | **14** | `noCollission()`, 1/16 strips |
| `edge_light_top` | 16 | 32 | `minecraft:oak_fence` | **14** | top strips |
| **Total** | **160** | **2048** | | | |

Templates were chosen from the pin's 26.3 state table
(`.local/horizon-client/26.3/model-states.json`) and interface list, not
guessed: `amethyst_cluster` is `facing` × `waterlogged`, `iron_chain` is
`axis` × `waterlogged`, `oak_fence` is exactly the four side booleans ×
`waterlogged`, `mangrove_roots` is `waterlogged` only (plus our `position`
enum), `calcite` has no states. All four waterlogged templates implement
`SimpleWaterloggedBlock`, so the state and the fluid behaviour come from the
template.

Declarations also set upstream's `Block.Properties.of().strength(1.0f)` as
`hardness: 1.0` + `explosion_resistance: 1.0`, `requires_tool: false`,
`sounds: "minecraft:stone"`, and the upstream `getShape` boxes per
property (`shapes`, twelve to sixteen boxes for the edge lights' side
combinations).

## Behaviour carried, and measured deviations

Carried (proved by the e2e): all 160 ids register, **2048/2048 states bind**,
the light levels are upstream's, every block **drops itself**, six recipes
craft, the block and item tags resolve, all 160 items exist, the creative tab
lists all 160, the items place, and unknown ids/invalid states are refused.

Template-derived deviations, measured in `probe-support.log`:

1. **Panel/slab/wall lamp/bulb need a support and break without it.** They copy
   `amethyst_cluster`, whose `canSurvive`/`updateShape` removes the block when
   the face opposite `facing` loses its sturdy block; upstream's
   `RotatableLamp` has no such rule. Measured: `illuminant_panel[facing=up]`
   on stone survives, removing the stone turns it to `minecraft:air`.
2. **Edge lights follow neighbours after placement.** They copy `oak_fence`,
   whose `updateShape` recomputes the four side booleans; upstream sets them
   once at placement and keeps the shape. Measured: `edge_light` all-false,
   a stone placed east turns `east=true`, removing it turns `east=false` again.
   (This also gives placement the "follows the walls" look for free.)
3. **`lamp_post` emits 15 from every position.** `properties.light` is scalar,
   so bottom/middle are brighter than upstream's 0; measured f/f/f. The e2e and
   the runtime test pin the gap so a per-state light API makes them fail.

## Gaps and deferred work (native slice 2)

1. **Redstone toggle of the illuminant blocks** (`neighborChanged`,
   `setPlacedBy`, `isSignalSource`): the two ids are fixed on/off blocks in
   content; inverting a placed lamp needs a logic half (`neighbor_changed` +
   `native_set_block`).
2. **The 3-block lamp-post column** (place/remove cascade) and its per-position
   light — placement logic, not data.
3. **The edge-light sneak-use editor GUI** (screen + network packet) and the
   "compute sides once" rule; the GUI is client work (`native_screen`).
4. **Tooltips** (`appendHoverText`, the shift hint and `.info*` lang keys) —
   the content schema has no tooltip field; a client half (`native_tooltip`)
   would add them. The lang keys are carried and inert.
5. **Item behaviour**: the lamp-post item's `+4.0` attack damage and the
   `Equipable` HEAD slot on every block item are not carried.
6. **Fluid behaviour was not exercised**: the waterlogged *states* bind and the
   templates implement `SimpleWaterloggedBlock`, but the e2e sets the state
   rather than flowing water into it. **Closed at 0.2.1** —
   `tools/data_parity.sh` places the block into a water cell for all eight
   waterlogged families and reads `waterlogged=true`, with the air differential
   and the stateless `illuminant_block` counterfactual; see
   [`## Slice: carried-data parity`](#slice-carried-data-parity--waterlogging-021-2026-11-21-pin-0a875da).
7. **The optional CTM resource pack** (`optional_fullblock_ctm`, a built-in
   client pack upstream) is carried in OptiFine's `ctm_compact` form: the
   original's CTM-mod sheets (`"type": "ctm"`, version 1) become tiles 1-4 and
   each block's own texture tile 0 (`tools/gen_ctm_pack.py`), since Sunrise
   reads `optifine/ctm` and not the CTM mod's `.png.mcmeta`. Faces join by
   texture (OptiFine's default for `matchTiles`), so lit and unlit blocks of
   one colour stay apart.
8. **No JEI plugin** (upstream ships one); Horizon's recipe viewer is built in.
9. **`c:stones` membership is ours**: 26.3 ships no `c:` tags, so
   `data/c/tags/item/stones.json` lists the 12 vanilla natural stone blocks
   (NeoForge's exact membership may differ). `c:dusts/redstone` and
   `c:dusts/glowstone` are the single vanilla items.
10. **Upstream's `itemGroup.simplylight` lang key is unused**: the tab title is
    the literal `Simply Light`, as upstream's `Component.literal` is.

## Files

```
Cargo.toml, Cargo.lock, compatibility.json, LICENSE, rust-toolchain.toml, .gitignore
simplylight.hnmod                          delivered native bundle (4,651 B)
mixins/simplylight/horizon-mixin.toml      bundle, 0 injections
mixins/simplylight/runtime.rs              10 tests (test-only census model)
tools/gen_content.py                       deterministic content generator
tools/craft_check.py                       the recipe scenario (grid 100..108)
tools/e2e.sh                               the in-game scenario (4 legs)
tools/hmod-load.sh                         .hmod load + no-package control
tools/horizon.py                           stage/check/test/build ladder
tools/package.py                           the .hnmod packager
content/simplylight.hmod                   delivered content package (687,852 B, 1453 entries)
content/horizon-mod.toml                   manifest (API 0.2, side both)
content/data/simplylight/horizon/block/    160 declarations
content/data/simplylight/horizon/creative_tab/lights.json
content/data/simplylight/loot_table/blocks/  160 26.3 loot tables
content/data/simplylight/recipe/           234 26.3 recipes
content/data/simplylight/tags/item/        10 upstream any_* item tags
content/data/minecraft/tags/block/         2 vanilla tag additions
content/data/c/tags/item/                  3 convention tags supplied
content/assets/simplylight/blockstates/    160
content/assets/simplylight/models/         210 block + 160 item
content/assets/simplylight/items/          160 26.3 item definitions
content/assets/simplylight/textures/       152 png + 33 mcmeta
content/assets/simplylight/lang/           7 locales
content/carry/                             the raw `horizon-port-pack check` output
```

## 26.3 carry traps hit

- **Loot conditions** are 26.3's singular typed form already
  (`"condition": {"type": "minecraft:survives_explosion"}`); all 160 verified
  programmatically (no `conditions` list, no untyped condition, every entry the
  block itself).
- **Item definitions** moved to `assets/<ns>/items/<id>.json`; the carry tool
  added 160, each naming the carried `models/item/<id>.json`.
- **`c:` tags do not exist in 26.3**; without them the stone/glowstone recipes
  would expand to empty ingredients and never match — the three tags are
  supplied.
- **A UTF-8 BOM** in upstream's `models/block/floor_lamp.json` breaks the JSON
  reader; the generator strips BOMs.
- **Upstream's dead `edge_light.jsn` / `edge_light_top.jsn`** (typo files in
  the jar, read by no block) are skipped.

Skills: superpowers/using-superpowers,writing-plans,systematic-debugging,verification-before-completion; BeautifulRust/rust-best-practices

Skills: superpowers/using-superpowers,writing-plans,verification-before-completion; BeautifulRust/none (no Rust change) — documentation-consistency job (2026-11-19): the header pin line now states `0b14f8a`/`e59da90` (the retired bundle's `1d63e0a5` is labelled history), no code touched.

## Verification

> **Historical (retired `.hnmod`, pin `1d63e0a5`, version 0.1.0).** Everything in this section
> measured the retired bundle and its `content/` package; the tools it names
> (`tools/horizon.py`, `tools/package.py`, `compatibility.json`, the staged ladder) are gone.
> The flat package's own ladder is in
> [`## Migration to the SDK format`](#migration-to-the-sdk-format-020-2026-11-18), and the
> e2e logs here remain the behaviour's evidence because the carried bytes are identical.

Pin **`1d63e0a5a37a7a708eb0b32b49bf0dfeb28ed8a9`** / horizon-mixins
**`5a64af938ec86b00d53d16a408aabc0038fb3e57`** (schema 6), Rust 1.95.0, 26.3
prepared resources at `/home/adlf/Desktop/horizon/.local/horizon-client/26.3`.
Every cargo/stage/e2e command ran on 2026-11-09 through
`ports/tools/build-slot.sh` (registry `ports/_shared/build-slots`,
`CARGO_INCREMENTAL=0`, build steps in slot 2 because slot 1 was held by another
job); logs are in
[`ports/_shared/evidence/simplylight/`](../_shared/evidence/simplylight/).

### Standalone runtime crate (test-only census)

| Check | Command | Result |
|---|---|---|
| fmt | `cargo fmt --all -- --check` | `FMT_OK` |
| tests | `cargo test --locked --offline` | **10 passed, 0 failed** |
| clippy | `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, **0 warnings** |

`runtime.log`.

### Staged ladder at `1d63e0a`

| Step | Result |
|---|---|
| stage | `stage_exit=0`, `stage_changed: true`, revision `1d63e0a5…`, 88.4 s |
| check | `cargo_exit: 0`, 53.2 s, then cached 0.3 s (`horizon-check.log`) |
| test | `cargo_exit: 0`, **10 passed; 0 failed; 1240 filtered** (`horizon-test.log`) |

`horizon-stage.json`, `horizon-check.json`, `horizon-test.json`, their logs.
(`build` was not run: the bundle has zero injections and the game binary is the
campaign's shared `1d63e0a5` one, which the e2e uses.)

### Packages (delivered in the port, not in `target/`)

| Package | Size | SHA-256 | Entries | Check |
|---|---:|---|---:|---|
| [`content/simplylight.hmod`](../_shared/evidence/simplylight/retired-content-0.1.0/simplylight.hmod) | 687,852 B | `3ab6ee0921c3efed804372b0c605f5c74c3b49b42b54f77049a360f639e5c8a8` | 1455 (882 assets, 570 data, 1 manifest, 2 licence copies written at pack time) | `cargo-horizon-mod check` exit 0: `ok:true`, `side:both`, `no code: content only`; a rebuild reproduces the same SHA-256 |
| [`simplylight.hnmod`](../_shared/evidence/simplylight/retired-hnmod-0.1.0/simplylight.hnmod) | 4,651 B | `159d5a8409baebd0cf7431553ef9aa77a1a0aa2cb47150d08e1ae6f27e4e1981` | 3 (`horizon-native.json`, bundle toml + runtime.rs) | manifest pins `horizon 1d63e0a5…`, `mixins 5a64af93…`, schema 1, game 26.3; repack reproduces the same SHA-256 |

`hmod-build.log`, `hmod-check.log`, `hmod-check-human.log`, `hmod-entries.log`,
`hnmod-package.log`, `hnmod-entries.log`.

### Mode B rebuild (2026-11-09) — artifact made provably current

Pin unchanged; the tree was rebuilt and both packages re-created. Every step
ran through `ports/tools/build-slot.sh run --job MB-simplylight --port
ports/simplylight` with `CARGO_INCREMENTAL=0` (content-only, zero injections:
the e2e legs use the shared `1d63e0a5` game, so no per-port build).

| Step | Result |
|---|---|
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, **0 warnings** |
| `cargo test --locked --offline` | **10 passed**, 0 failed |
| `horizon.py stage` / `check` / `test` | `cargo_exit: 0`, **10 passed / 1240 filtered** |
| `.hmod` rebuild | **687,852 B**, `3ab6ee09…c8a8`, 1455 entries (the retired table's `1453` omitted the two pack-time licence copies) — hash unchanged |
| `.hnmod` repack | **4,651 B**, `159d5a84…1981`, 3 entries — hash unchanged |
| default / counterfactual / negative / selftest | `PASS+EXIT:0` / `PASS+EXIT:0` / `FAIL+EXIT:1` / `FAIL+EXIT:1` |

`check-e2e-logs.py simplylight --strict` → `logs=4 ok=4`, exit 0.
`upstream_ready.py simplylight` → **READY** (exit 0).

### In-game runs (shared unmodded `1d63e0a5` game, `--dev-mod content/`)

| Run | Command | Result |
|---|---|---|
| default | `tools/e2e.sh` | **PASS + `EXIT:0`** — `e2e-default.log` |
| counterfactual | `COUNTERFACTUAL=1 tools/e2e.sh` | **PASS + `EXIT:0`** — `e2e-counterfactual.log` |
| negative | `NEGATIVE=1 tools/e2e.sh` | **FAIL + `EXIT:1`** as required — `e2e-negative.log` |
| selftest | `SELFTEST=1 tools/e2e.sh` | **FAIL + `EXIT:1`** as required — `e2e-selftest.log` |
| `.hmod` load | `tools/hmod-load.sh` | **PASS + `EXIT:0`** — `hmod-load.log` |
| deviation probe | `/tmp/opencode/simplylight-probe.sh` | **`EXIT:0`** — `probe-support.log` |

`python3 ports/tools/check-e2e-logs.py simplylight --strict` → exit 0:
`ok e2e-counterfactual.log PASS + EXIT:0`, `ok e2e-default.log PASS + EXIT:0`,
`ok e2e-negative.log FAIL + EXIT:1`, `ok e2e-selftest.log FAIL + EXIT:1`
(`logs=4 ok=4 missing-exit=0 bad-exit=0`).

The default run's transcript (quoted):

```
160/160 default states bound (10 families × 16 colours)
32/32 lamp states bind (16 blocks × 2)
32/32 lamp states bind (16 blocks × 2)
192/192 slab states bind (16 blocks × 12)
192/192 panel states bind (16 blocks × 12)
192/192 wall-lamp states bind (16 blocks × 12)
192/192 lightbulb states bind (16 blocks × 12)
96/96 rod-lamp states bind (16 blocks × 6)
96/96 lamp-post states bind (16 blocks × 6)
512/512 edge-light states bind (16 blocks × 32)
512/512 top-edge-light states bind (16 blocks × 32)
2048/2048 declared states total
lightbulb=e illuminant_slab=f glowstone=f rodlamp=f stone=0 lamp_post=f wall_lamp=f off=0 on=f illuminant_panel=f edge_light=e edge_light_top=e
15: panel/slab/wall/rod/post/on-lamp; 14: bulb/edge/edge-top; 0: off lamp and stone; the off lamp differs from the vanilla glowstone
{"scenario":"simplylight:illuminant_panel","block":"minecraft:air","block_before":"simplylight:illuminant_panel[facing=up,waterlogged=false]","drops":{"simplylight:illuminant_panel":1}}
… (nine blocks, each dropping itself)
stone/glowstone/redstone -> {"count": 4, "item": "simplylight:illuminant_block", "slot": 200}
stone/glowstone/redstone torch -> {"count": 4, "item": "simplylight:illuminant_block_on", "slot": 200}
glowstone over stone -> {"count": 8, "item": "simplylight:lightbulb", "slot": 200}
8 off-lamps + black dye -> {"count": 8, "item": "simplylight:illuminant_black_block", "slot": 200}
black on-lamp -> off-lamp -> {"count": 1, "item": "simplylight:illuminant_black_block", "slot": 200}
glowstone over sticks -> null
tag PASS
160/160 items give-able; sample items land in the inventory
creative tab lists all 160 items
placed simplylight:illuminant_block -> simplylight:illuminant_block[on=false]
placed simplylight:illuminant_block_on -> simplylight:illuminant_block_on[on=true]
placed simplylight:illuminant_panel -> simplylight:illuminant_panel[facing=up,waterlogged=false]
placed simplylight:edge_light -> simplylight:edge_light[east=false,north=false,south=false,waterlogged=false,west=true]
unknown ids and invalid states are refused
PASS: 2048/2048 states bind …, 9/9 blocks self-drop, six recipes craft with an empty stick control, the block tags hold, 160 items give and the tab lists 160, one item of every family places, and unknown ids are refused
EXIT:0
```

The counterfactual, negative and selftest (quoted):

```
vanilla glowstone light=f
without the content, setblock simplylight:illuminant_block_on reads 'minecraft:air'
the content ids are refused without --dev-mod, and vanilla glowstone still emits 15
PASS: counterfactual (no --dev-mod: the 160 ids are unknown there; a vanilla glowstone emits 15)
EXIT:0

FAIL: NEGATIVE: the deliberately false assertion (the content is loaded without --dev-mod) tripped, as it must; the --dev-mod differential is live (this exit 1 is the proof)
EXIT:1

SELFTEST: read simplylight:illuminant_panel[facing=up,waterlogged=false]
FAIL: SELFTEST: read '…' where the deliberately wrong assertion wanted facing=down; the state read-back is live (this exit 1 is the proof)
EXIT:1
```

The `.hmod` load check (quoted):

```
block 0,80,0 = simplylight:illuminant_block_on[on=true]
block 2,80,0 = simplylight:illuminant_panel[facing=up,waterlogged=false]
block 4,80,0 = simplylight:lamp_post_pink[position=bottom,waterlogged=false]
block 6,80,0 = simplylight:edge_light_top_red[east=false,north=false,south=false,waterlogged=false,west=false]
items simplylight:illuminant_block_on and simplylight:lamp_post_pink = 1
control block 0,80,0 = minecraft:air … control block 6,80,0 = minecraft:air
PASS: the .hmod registers the on lamp, a panel, the pink lamp post and a top edge light and their items; without it the same ids are refused
EXIT:0
```

### Harness facts learned

- Killing the Ender Dragon opens the victory screen, which makes `ctl break`
  refuse (`the win screen is open; close it first`): the e2e closes it after
  the platform setup.
- `ctl place X Y Z --item id` needs the target air cell to be reachable along a
  face; a `amethyst_cluster`-templated block only survives if the face opposite
  `facing` has support, so the placement check puts stone west of each cell.
- `ctl select` picks a **hotbar** slot: ten different items push the tenth into
  the main inventory, so the placement leg clears before each give.
- The mod creative cache escapes the quotes in its item strings
  (`{count:1,id:\"ns:path\"}`), so the tab check greps the bare id, not `"id"`.
- A 32,768-block command cap: the platform's air fill must be split or dropped.

## Reviewer repairs (2026-11-11) — PR review items and what changed


Each item below is quoted from `ports/_shared/horizon-mods/reviews/pr-*-simplylight.md` (the review is the specification), with what this repair did about it.

- **Reviewer:** [P1] the entry pinned `ec720186…` but the downloadable release asset was `3ab6ee09…`
  **Repair:** The source repository's `content/simplylight.hmod` **is** the `ec720186…` file the entry pins, so the entry was right and the release asset was stale. After this repair the intended file is one artifact with the new hash printed in the report; the release asset must be re-uploaded from `content/simplylight.hmod` and the entry updated to match (orchestrator, needs the token).
- **Reviewer:** licence notice
  **Repair:** The rebuilt `.hmod` carries `data/simplylight/license/{license,notice}.txt`.

Skills: superpowers/using-superpowers,systematic-debugging,verification-before-completion; BeautifulRust/rust-best-practices

## Migration to the SDK format (0.2.0, 2026-11-18)

*First publication.* There is no closed PR to supersede — this is the port's first submission —
and the text that goes to the registry is at the end of this section ("Changelog").

**Why.** Horizon's mod-API merge (#16941, launcher ≥ 0.2.70) retired the `.hnmod` container, and
the new unit is **one flat `.hmod`**: content, native code, or both. For this port that is a
simplification — the retired bundle existed only to run the test-only census at the pin, while
the 160 blocks were a *second* package under `content/`. The flat package carries `assets/` and
`data/` itself and keeps the census in `src/`.

- **Pins:** Horizon / SDK **`0b14f8acfd97fa2c750e42795249dcc981178ca4`** (2026-11-18, package
  format **6**, content API **`0.2`**, toolchain rust 1.99.1), horizon-mixins
  **`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`**. The SDK is fetched into `.horizon/sdk/` and
  never packed; this port's copy is the same signed slice verified in
  `ports/_shared/sdk/0b14f8a/`.
- **Version:** `0.1.0` → **`0.2.0`** (a resubmission is always a new version).
- **Class + tier unchanged: class `content`, tier `data`** (`atm10-sources.csv`; the ledger row
  already says `content,data`). The 160 blocks are carried-over content; the only code is the
  census model, and there is still no code half (see "Gaps after the migration").
- **Approach from `docs/modpacks.md`:** §"Registries 0.2: data-driven blocks, items and mob
  effects" (lines 156–260 of the pin's `docs/modpacks.md`) is the whole job — `assets/` and
  `data/` are copied byte for byte, each block declaration naming a vanilla template — plus the
  SDK's §0 layout for the crate (`ports/_shared/AGENT_MOD_STRUCTURE.md` §0). Nothing in the
  retired §4 plan/injection schema survives, and nothing was re-tiered.

### Files created, files decided

| Created | What it holds |
|---|---|
| `horizon-mod.toml` | the only manifest: `id = "simplylight"`, `version = "0.2.0"`, `api = "0.2"`, `side = "both"`, `capabilities = ["registries"]`, `[native]` with the exact three pins |
| `Cargo.toml` | one crate (`mod-simplylight`, `publish = false`); the only dependency is `horizon-mod-api` from `.horizon/sdk/` |
| `src/lib.rs` | `pub mod runtime;` and the one `#[mod_api::horizon_mod] impl Hooks` — **empty**, because the mod has no game moment to answer |
| `src/runtime.rs` | the retired `runtime.rs`, **byte-identical** (`cp`, then `cmp`) |
| `assets/simplylight/**`, `data/**` | the retired content package's own trees, moved to the package root |
| `.gitignore` | `/target`, `/.horizon`, `/Cargo.lock` |
| `README.md` | rewritten for the flat layout **before** the first build (the root is packed, trap 10) |
| `simplylight.hmod` | the delivered package, port root (never `target/`) |

| Retired | Why | Where it went |
|---|---|---|
| `mixins/simplylight/` (`horizon-mixin.toml`, `runtime.rs`) | the plan/module schema is not part of any SDK package; `runtime.rs` moved to `src/` | `ports/_shared/evidence/simplylight/retired-hnmod-0.1.0/mixins/` |
| `simplylight.hnmod` (4,651 B) | unloadable on launcher ≥ 0.2.70 | same folder |
| `compatibility.json`, `rust-toolchain.toml` | `[native]` in the manifest is the pin; the launcher decides the toolchain | same folder |
| `tools/horizon.py`, `tools/package.py`, `tools/stamp_license.py` | the staged ladder, the `.hnmod` packager and the notice stamper belong to the retired format | `retired-hnmod-0.1.0/tools/` |
| `content/` (`horizon-mod.toml`, `simplylight.hmod`, `carry/`, `carry-report.json`, `check.json`, `outside/`) | a second package; its manifest's five fields are all in the root manifest now | trees → the package root; the rest → `retired-content-0.1.0/` |
| `Cargo.lock` | written by `cargo test`; packing it changes the artifact (trap 4) | deleted; `/Cargo.lock` is gitignored |

**Moved before the first build**, as trap 1 demands: the project root may hold only package
paths, and a surviving `mixins/` is refused outright. The retired tree is **kept as evidence**,
never deleted — `retired-hnmod-0.1.0/` (the container), `retired-content-0.1.0/` (the content
artifact, the raw `horizon-port-pack` carry tree, its report and the 83 recipes it left outside).

Kept on purpose: `tools/e2e.sh` (the in-game oracle, repointed at the package),
`tools/hmod-load.sh` (the smaller "does the package register" leg), `tools/craft_check.py`,
`tools/gen_content.py` (repointed at the package root, reading the carry tree from evidence),
`notes.md`, `PLAN.md`, `LICENSE`, `LICENSE-NOTICE.md`.

### Package file map (1,462 entries)

| Path | Entries | Role |
|---|---:|---|
| `horizon-mod.toml` | 1 | the manifest and the only identity |
| `Cargo.toml` | 1 | the crate; `horizon-mod-api` path dependency only |
| `src/lib.rs`, `src/runtime.rs` | 2 | the empty `impl Hooks` + the carried census |
| `assets/simplylight/**` | 882 | 160 blockstates, 210 block models, 160 item models, 160 26.3 item definitions, 152 png + 33 `.mcmeta`, 7 lang |
| `data/simplylight/**` | 572 | 160 block declarations, 1 creative tab, 160 loot tables, 234 recipes, 10 `any_*` item tags, 2 licence copies |
| `data/minecraft/tags/**`, `data/c/tags/**` | 5 | 2 vanilla block-tag additions, 3 convention tags 26.3 does not ship |
| `LICENSE`, `LICENSE-NOTICE.md`, `README.md` | 3 | the MIT text, the provenance notice, the shipping readme |
| `uses.json` | 1 | generated: see "What the package says it uses" |

The packer takes `horizon-mod.toml`, `Cargo.toml`, `src/`, `assets/`, `data/`, `uses.json` and
the root `LICENSE*`/`README.md`/`icon.png`. It does **not** take `notes.md`, `PLAN.md`, `tools/`
or the evidence folder — measured from the entry list, which is why the readme is the only
narrative a player or reviewer sees.

**Manifest schema note.** `id`, `name`, `version`, `description`, `authors`, `api`, `side`,
`capabilities` and `[native]` are the fields this package writes; `license`, `homepage` and
`issues` are **not** fields (other ports carry them and the build drops them), so the licence is
recorded at the root, in `data/simplylight/license/` and in this file. `namespaces` is omitted:
the 0.1.0 package carried its `data/c/` and `data/minecraft/` tag files without it and the tags
resolved in game, and the migrated `cloudglass` does the same — if the launcher ever requires
the list, these two are the additions.

### Runtime carry (the campaign rule)

`src/runtime.rs` is the retired `mixins/simplylight/runtime.rs`, copied **before** that file was
moved to evidence: `cmp` clean, **385 lines**, SHA-256
`f2f2f3a002fce5eacaced5c724cf78415c17600c40592d7d2a4dd62aca87ff2b`, and the entry inside the
delivered `.hmod` hashes to the same value (`unzip -p simplylight.hmod src/runtime.rs |
sha256sum`).

Its **10 tests ran green before any glue existed** (`src/lib.rs` was `pub mod runtime;` alone:
`sdk-unit-before-glue.log`), then again with the `impl Hooks` in place (`sdk-test.log`).
Nothing in the module changed, including the two test-local names that trip the `uses.json`
scanner (trap 3) — the campaign rule outranks a cosmetic metadata entry.

The module is `#[cfg(test)] mod tests` from top to bottom, so the package's non-test Rust is
only the empty `impl Hooks`; `pub mod runtime;` keeps the tests reachable under
`cargo test` without a `dead_code` warning (imfast trap 8).

### Content carry (proof it is mechanical)

The retired package's own tree and the new package root were compared file by file:

- **1,454 `assets/` + `data/` files on both sides** (`diff -r` clean on each tree, no file only
  in one, no differing byte), aggregate SHA-256 over `find assets data -type f | sort | xargs
  sha256sum`: **`2a996fbefe2682f80c590c3a3b212378db0e14e399cab3d2bb33e72fe2dfb649`**.
- The two `data/simplylight/license/` copies were written *at pack time* by the retired
  `tools/stamp_license.py` and were never in the old on-disk tree; they were extracted from the
  retired artifact byte for byte (`license.txt == LICENSE`, `notice.txt == LICENSE-NOTICE.md`),
  and `tools/gen_content.py` now writes them, so a regeneration is complete (trap 6).
- `tools/gen_content.py` was repointed at the package root (and at the evidence carry tree) and
  **reproduces the whole carried tree byte for byte** — `diff -r` against the retired package
  clean on both trees after a fresh run. `sdk-content-carry.log`.

### Helper → hook table

The retired plan had one `[[module]]` and **zero** `[[injection]]`s. Every retired entry is
mapped below; nothing is dropped silently.

| Retired | SDK (0.2) | Where |
|---|---|---|
| `[[module]] id = "runtime"`, `file = crates/horizon-client/src/game.rs`, `source = runtime.rs` | no plan section — `src/runtime.rs` is an ordinary module | `src/lib.rs`: `pub mod runtime;` |
| `[mod] priority = 100` | gone; ordering is `#[listener(priority = …)]` on a hook | — (no hook) |
| `[compatibility] horizon_revision = 1d63e0a5…` | `[native] horizon` / `mixins` / `game` (40-hex, moved to the SDK pin) | `horizon-mod.toml` |
| `horizon-native.json` (schema 1) | gone: the manifest is the only one | — |
| **no `[[injection]]` at all** | one empty `#[mod_api::horizon_mod] impl Hooks for Mod {}` | `src/lib.rs` |
| upstream's redstone toggle, lamp-post column, edge-light editor, tooltips | **no method, no Part**: recorded gaps, not invented hooks | "Gaps after the migration" |
| the 160 blocks themselves | **not Rust**: content declarations (template + properties + shapes) | `data/simplylight/horizon/block/` |
| the creative tab | content | `data/simplylight/horizon/creative_tab/lights.json` |

There is no `Registrar` declaration either: `capabilities = ["registries"]` is a schema
requirement of the manifest, and the package registers its blocks as content, not through
`Registrar::block*`. **Hooks mapped: 0. Parts: 0. Gaps: 4** (the behaviours in the row above).

### Build and check (verbatim, in a build slot)

The build steps ran **locally** (`BUILD_LOCAL=1`): the far box has no
`cargo-horizon-mod` (trap 2). Every command is `bash -c '[ -f Cargo.toml ] || cd ports/simplylight;
…'` because the far runner `cd`s into the port and the local one does not (trap 1).
`CARGO_INCREMENTAL=0`, no `--target-dir`.

```sh
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job simplylight-sdk --port ports/simplylight --step build \
    -- bash -c '[ -f Cargo.toml ] || cd ports/simplylight; rm -f Cargo.lock; CARGO_INCREMENTAL=0 cargo horizon-mod build'
# → SDK 0b14f8a (build 0b14f8acfd97fa2c750e42795249dcc981178ca4) in ./.horizon/sdk
# → Built ./target/horizon-mod/simplylight.hmod

BUILD_LOCAL=1 ports/tools/build-slot.sh run --job simplylight-sdk --port ports/simplylight --step check \
    -- bash -c 'cargo horizon-mod check simplylight.hmod && cargo horizon-mod fits simplylight.hmod --build nightly'
```

```
$ cargo horizon-mod check simplylight.hmod
simplylight 0.2.0 (native-mod, format native-1/0b14f8acfd97fa2c750e42795249dcc981178ca4/e59da9094f0c2fdda6c627fe9eff22dec6a906f0, for 26.3)
  700196 bytes, SHA-256 4fcc82a26bc25522a4a32299a349cf693626e68adc80c7d74a196c64ba7f2d3b, unsigned
  native source: unsandboxed; not executed by the registry check. Compile and test against the exact pinned Horizon and Mixins revisions before review.
CHECK_EXIT:0

$ cargo horizon-mod fits simplylight.hmod --build nightly
Simply Light:
0b14f8a  made for
FITS_EXIT:0
```

`$ cargo horizon-mod check simplylight.hmod --uses` printed the report only on this tool build;
the list is the package's own `uses.json` (`unzip -p simplylight.hmod uses.json`).

### What the package says it uses

`uses.json` (schema 1, `made_for = 0b14f8ac…`) has **30 entries**. Four are real —
`capability:registries`, `macro:horizon_mod`, `part:block`, `trait:Hooks` — and the rest are
recorded by the scanner from **test-only identifiers** in the byte-identical `runtime.rs`:

- a test-local binding named `blocks` (`let mut blocks = Vec::with_capacity(160);`) is read as
  the `blocks` hook, which also pulls in `method:Hooks::blocks`, `type:BlocksEvent`,
  `type:Block`, `BlockProperties`, `BlockShapes`, `StateLight`, `PushReaction`, `RandomTicks`,
  `RenderShape`, `ShapeEntityKind`, `RegistrationContext`/`RegistrationReadError`, `Handles*`,
  `Tag`/`Tag.String` and `handle:Tag::id`;
- `2u32.pow(4)` in one assertion is read as `fn:java_math::pow` / `fn:strict_math::pow`.

**The package answers no hook.** Three probe builds prove the mapping and its cause: (a) the
same package with `data/` deleted — identical list, so content does not produce it; (b) with
`cloudglass`'s `src/` — list falls to the four real entries; (c) with only the two test-local
names renamed (`blocks` → `ledger`) — `hook:blocks` and every entry it dragged in disappear.
The rename is **not** applied here: `runtime.rs` must stay byte-identical in a migration, so
this is a recorded tooling false positive, worth an upstream report (the scanner should skip
`#[cfg(test)]` code); the fix belongs to a slice that is allowed to touch the runtime.

### The e2e (no claim)

No in-game run was made for the flat package: `horizon-launcher` is **not installed** on this
box and there is no `0b14f8a` game build, so nothing can load a `.hmod` that carries native
code. `tools/e2e.sh` is updated for the launcher world:

- `GAME=<launcher-built horizon-client>` is **required** — with `GAME=` unset the script refuses
  with the launcher command to run (measured: `EXIT:1`, it does not silently pass);
- `CONTROL_GAME=<a build without the package>` adds the "the ids are unknown there" leg, and
  `COUNTERFACTUAL=1` keeps the same-game leg (`--dev-mod` absent);
- the launch is `--dev-mod "$PORT/simplylight.hmod"`;
- `SELFTEST=1` (a deliberately wrong panel facing) and `NEGATIVE=1` (the deliberately false
  "loaded without `--dev-mod`") are unchanged and still exit 1, so the run stays fail-capable;
- the 0.1.0 logs (`e2e-default.log`, `e2e-counterfactual.log`, `e2e-negative.log`,
  `e2e-selftest.log`, `hmod-load.log`, `probe-support.log`) are **historical**: they prove the
  carried bytes and the declared behaviour, not the transport.
`tools/hmod-load.sh` defaults at the package too and has the same launcher requirement.

### Gaps after the migration

1. **No in-game run of the flat package** — no `horizon-launcher`, no `0b14f8a` game build. The
   first launcher-capable box runs `GAME=… tools/e2e.sh` (and `CONTROL_GAME=…`) and records it.
2. **The behaviour gaps are unchanged** (the migration adds no code): the redstone toggle of the
   two lamp blocks, the lamp-post column's place/break cascade and per-position light, the
   edge-light editor screen, the shift tooltips, the lamp-post item's `+4.0` attack damage and
   the `Equipable` head slot, water *flowing* into the waterlogged states, the optional CTM
   resource pack, a JEI plugin, and `c:stones` membership (ours, since 26.3 ships no `c:` tags).
   See "Gaps and deferred work (native slice 2)" above — the list did not move.
3. **`uses.json` overstates the surface** (trap 3): the registry's `--uses` review reads
   `hook:blocks` on a package that implements none. Not fixable without editing the
   byte-identical runtime.
4. **The measured template deviations stay** (panel/slab/wall lamp/bulb need support, edge lights
   recompute their sides, `lamp_post` emits 15 from all three positions): a content carry cannot
   express them differently, and the runtime tests pin them so a per-state light API will fail
   loudly. Unchanged by this migration.

### Upstream defects fixed

Carried from the 0.1.0 carry and still true of this package (proved from the artifact):

1. **Two dead blockstate files** — `assets/simplylight/blockstates/edge_light.jsn` and
   `edge_light_top.jsn` are in the jar with a `.jsn` extension, read by no registered block
   (`SLBlocks` names `edge_light`/`edge_light_top`, whose blockstates are the `.json` files). The
   port skips them (`tools/gen_content.py` filters `.jsn`); copying them would ship two files no
   loader can parse. Nothing user-visible.
2. **A UTF-8 BOM** in upstream's `models/block/floor_lamp.json` makes the resource reader fail on
   the model, which would leave the floor lamp with the missing-texture model. The generator
   strips BOMs from every carried JSON. User-visible as "the floor lamp renders".

No further upstream report is warranted for either (both are inert in the Java mod); they are
recorded here because the port is judged on the behaviour the mod means.

### Traps measured in this migration

1. **The far runner `cd`s into the port; the local runner does not.** The brief's command shape
   (`bash -c 'cd ports/simplylight && …'`) fails on the second machine with
   `cd: ports/simplylight: No such file or directory`, because it is already in
   `/home/deepseek/atm10/ports/simplylight`. Use `[ -f Cargo.toml ] || cd ports/<id>` in the
   wrapped command and it works in both.
2. **The far box has no `cargo-horizon-mod`.** `build-slot.sh run --step build` forwards by
   default and answers `error: no such command: 'horizon-mod'` — and the *runner* still exits 0
   on that run, so read the log, not the runner's exit code. Package steps need `BUILD_LOCAL=1`.
3. **`uses.json` records test-only code.** A local binding named `blocks` claims the `blocks`
   hook, and `2u32.pow(4)` claims `java_math::pow`; the scanner does not skip
   `#[cfg(test)]`. Measured by three probe builds (see "What the package says it uses"). Keep
   `runtime.rs` byte-identical and record it; renaming the test locals is a separate slice.
4. **`Cargo.lock` changes the artifact.** `cargo test` writes one; `rm -f Cargo.lock` right before
   the delivery build, and `/Cargo.lock` is in `.gitignore`. Two builds without it are
   byte-identical here (700,196 B, `4fcc82a2…`).
5. **The far box's registry cache was short.** The first ladder with `--offline` failed with
   `no matching package named 'unicode_names2'` (a dependency of the SDK's `horizon-core`); one
   online `cargo test` fetched it. The SDK itself resolves 40 registry packages.
6. **The two licence files are pack-time artifacts.** `data/simplylight/license/{license,notice}.txt`
   existed inside the retired `.hmod` but not in the old on-disk `content/data/`, because
   `tools/stamp_license.py` wrote them at pack time. Moving the tree as-is makes the carry proof
   read 1,452 vs 1,454 files; extract them from the retired artifact (or let
   `tools/gen_content.py` write them, which it now does).
7. **`mv content/data ./data` nests when the destination exists.** Creating
   `data/simplylight/license/` before the move produced `data/data/`; move the trees first, then
   write into them (or move the children). Caught by counting the tree before building.
8. **There is no `todo!()` red to show.** The retired plan has zero injections, so the only glue
   is the empty `impl Hooks`; a `todo!()` would have meant inventing a hook. The campaign rule
   (runtime green before glue) is satisfied by the before-glue run of the same 10 tests, and
   `cloudglass` trap 7 measured that an empty `impl Hooks` is a valid native package.
9. **The root is packed, and only the documented root files.** `README.md` must be the shipping
   text *before* the build (it is in the archive); `notes.md`, `PLAN.md` and `tools/` are not
   packed (measured from the entry list), so they stay on disk without inflating the artifact.
10. **The pins are 40 hex and the mixins pin moved.** `e59da9094f0c…`, not the campaign's
    `5a64af93…`; a short hash reads as `native requires game and exact revisions` without saying
    which one is short.
11. **`cargo fmt --all` formats the fetched SDK** (a path dependency). `cargo fmt --check` only.
12. **`cargo horizon-mod check` with no file compiles wasm**, not the package. Always
    `check <id>.hmod`.

### Changelog (0.2.0) — first publication

> **First publication of Simply Light for Horizon: version 0.2.0, mod API `0.2`.**
> This is the first submission of `simplylight`; there is no earlier PR to supersede.
> The port is a content carry of Simply Light 1.5.3+1.21.1-b4 (MIT, by Flanks255) upgraded to
> Minecraft Java 26.3: 160 light blocks in ten families × sixteen dye colours — off and on lamps,
> slabs, panels, wall lamps, bulbs, rod lamps, three-block lamp posts and edge lights — with 2,048
> states, 234 recipes, twelve tags, models, textures and seven locales.
> **What changed for `api = "0.2"`:** the retired `.hnmod` container plus a separate `content/`
> package became **one flat `.hmod`**. The 160 block declarations, `assets/` and `data/` are now
> the package root instead of a nested content package, the test-only census model moved from
> `mixins/simplylight/runtime.rs` to `src/runtime.rs` **byte-identical**, and the plan/injection
> manifest (`horizon-mixin.toml`, `horizon-native.json`, `compatibility.json`) became a single
> `horizon-mod.toml` with `api = "0.2"` and `[native]` pinned to SDK `0b14f8ac…` / mixins
> `e59da909…` for game 26.3. There is no code half: the `impl Hooks` is empty and the package
> registers nothing from Rust.
> Not ported yet: the redstone toggle of the lamps, the lamp-post column's per-position light, the
> edge-light editor screen, tooltips, and the lamp-post item's attack damage. Local-only until the
> maintainer approves publishing (MIT, rank 0).

Skills: superpowers/using-superpowers,brainstorming,writing-plans,test-driven-development,systematic-debugging,verification-before-completion; BeautifulRust/rust-best-practices,rust-port

### Migration verification (SDK `0b14f8a`)

| Check | Command | Result |
|---|---|---|
| Runtime tests before glue | `cargo test` with only `pub mod runtime;` | **10 passed**, 0 failed (`sdk-unit-before-glue.log`) |
| Runtime tests with glue | `cargo test` | **10 passed**, 0 failed (`sdk-test.log`) |
| Lints | `cargo clippy --all-targets -- -D warnings` | `CLIPPY_EXIT:0`, 0 warnings (`sdk-clippy.log`) |
| Formatting | `cargo fmt --check` | `FMT_EXIT:0` (`sdk-clippy.log`) |
| Package | `cargo horizon-mod build` (no `Cargo.lock`) | `target/horizon-mod/simplylight.hmod`, `BUILD_EXIT:0` (`sdk-build.log`) |
| Reproducible | second build, `cmp` | byte-identical, same SHA-256 (`sdk-build-second.log`, `sdk-sha256-build1.txt`, `sdk-sha256-build2.txt`) |
| Registry check | `cargo horizon-mod check simplylight.hmod` | `simplylight 0.2.0 (native-mod, format native-1/0b14f8ac…/e59da909…, for 26.3)`, **700,196 B**, SHA-256 `4fcc82a2…2d3b`, `CHECK_EXIT:0` (`sdk-check.log`, `sdk-check.json`) |
| Fits the pinned build | `cargo horizon-mod fits simplylight.hmod --build nightly` | `0b14f8a  made for`, `FITS_EXIT:0` (`sdk-fits.log`) |
| Entries | `unzip -l simplylight.hmod` | **1,462**: 882 `assets/`, 572 `data/`, 2 `src/`, 6 root; **no `Cargo.lock`**; `LICENSE`, `LICENSE-NOTICE.md`, `README.md` inside (`sdk-entries.log`) |
| `runtime.rs` untouched | `unzip -p simplylight.hmod src/runtime.rs \| sha256sum` | `f2f2f3a0…ff2b` = the retired file, byte for byte (`sdk-runtime-carry.log`) |
| Content untouched | `diff -r` + aggregate hash, both trees | 1,454 files, `2a996fbe…b649` on both sides (`sdk-content-carry.log`) |
| What it uses | `unzip -p simplylight.hmod uses.json` | 30 entries, 4 real + the test-only scanner artifacts of trap 3 (`sdk-uses.json`) |
| `gen_content.py` regeneration | run, then `diff -r` vs the retired tree | identical (`sdk-content-carry.log`) |
| In-game e2e | — | **not run**: no launcher/no `0b14f8a` game (gap 1); the script refuses without `GAME=` |
| Skills line | `python3 ports/tools/check-skills.py --notes ports/simplylight --strict` | PASS |

## Content-only 0.2.0 — the registry's `kind = "mod"` rule (2026-11-19)

**Why this section exists.** PR #1228 was rejected by the registry reviewer
(`var/review-levzzz/1228.md`) for a **registry-immutability** violation, not a code one:
`mods/simplylight.toml` already exists on `main` as `kind = "mod"` with a published
`0.1.0`, and the PR *removed* that published version and *changed* the kind to
`native-mod`. Published versions are never removed and a live project's kind is never
changed. The reviewer allowed exactly two resolutions: (a) keep `kind = "mod"`,
preserve `0.1.0`, and ship `0.2.0` as a content-only `.hmod`; or (b) publish the native
port under a new id. **This port takes (a)**, because the native package registered
nothing (below), so a content-only package loses no behaviour.

### Hook inventory: the native package answered zero hooks

Proved from the retired native package and its sources, not from the notes:

| Evidence | What it shows |
|---|---|
| `src/lib.rs` (25 lines) | `pub mod runtime;` plus one `#[mod_api::horizon_mod] impl mod_api::Hooks for Mod {}` — **empty**. No `#[listener]`, no method override, no Part. |
| `src/runtime.rs` (385 lines) | `#[cfg(test)] mod tests` from line 10 to the end: the census model (families, ids, template state spaces, light levels, drop rules) runs only under `cargo test`. No game type, no hook. |
| `grep -rn "listener\|fn on_\|Hooks::" src/` | **no matches** |
| `unzip -p simplylight.hmod uses.json` | 30 entries, of which four are real (`capability:registries`, `macro:horizon_mod`, `part:block`, `trait:Hooks`). `hook:blocks` is a **scanner false positive** from the test-local `let mut blocks = Vec::with_capacity(160);` (measured by three probe builds in the migration section: deleting `data/` changes nothing, swapping in `cloudglass`'s `src/` drops the list to the four real entries, renaming the test local removes `hook:blocks`). |
| the 160 blocks, 2,048 states, 234 recipes, 12 tags | declared as **content** (`data/simplylight/horizon/block/`, `creative_tab/`, `loot_table/`, `recipe/`, `tags/`) and drawn from `assets/` — no Rust involved at any point. |

**Hooks the package registers: none.** The behaviours that *would* need hooks — the
redstone toggle of the two lamp blocks, the lamp-post 3-block column and its
per-position light, the edge-light editor screen, the shift tooltips, the lamp-post
item's attack damage and `Equipable` head slot — are **unported gaps** in both packages
(`## Gaps after the migration`), so neither resolution of the reviewer's finding changes
what a player gets. Under (a) a user loses **nothing**; under (b) the same mod would be
listed twice, which is the worse outcome.

**Residual benefit of (a):** the content-only package has no `src/`, so the false
`hook:blocks` claim disappears from its `uses.json` — the registry's `--uses` review
now reads a package that really has no code.

### What changed in the tree

| Before (native 0.2.0) | After (content-only 0.2.0) | Where the old file went |
|---|---|---|
| `horizon-mod.toml` with `[native]` (game/horizon/mixins pins) | same manifest **without `[native]`**, `version = "0.2.0"` | — |
| `Cargo.toml`, `src/lib.rs`, `src/runtime.rs` | gone (a content mod may not carry a crate; `cargo horizon-mod build` refuses one) | `ports/_shared/evidence/simplylight/retired-native-0.2.0/` |
| `simplylight.hmod` 700,196 B `4fcc82a2…2d3b` | `simplylight.hmod` **689,910 B** `433705de…5c50` | `…/retired-native-0.2.0/simplylight.hmod` |
| `.horizon/sdk/`, `target/`, `Cargo.lock` | removed (build scratch; regenerated) | not evidence |
| `README.md` describing the crate | rewritten: content mod, install, no code half | — |
| `tools/e2e.sh` "needs a launcher build" | content leg on a plain `0b14f8a` game with `--dev-mod` | — |

The delivered bytes are `horizon-mod.toml` + 882 `assets/` files + 572 `data/` files =
**1,455 entries**. The licence text and the provenance notice still travel inside the
package (`data/simplylight/license/license.txt`, `notice.txt`), which is what the entry's
description promises; a content build does not pack the root `LICENSE`/`README.md`, so
those two `data/` copies are the in-archive ones.

### Verification (SDK `0b14f8a`, 2026-11-19)

| Check | Command | Result |
|---|---|---|
| Build | `cargo horizon-mod build` (in a build slot, `CARGO_INCREMENTAL=0`) | `BUILD_EXIT:0`; a second build is byte-identical (`sdk-build-content.log`) |
| Registry check | `cargo horizon-mod check simplylight.hmod` | `simplylight 0.2.0 (mod, API 0.2, both)`, **689,910 B**, SHA-256 `433705de1d8467c0ab076ae02a0b7ed447e65b068032414db694dc46992c5c50`, `no code: content only, nothing to run`, `CHECK_EXIT:0` (`sdk-check-content.log`) |
| Fits | `cargo horizon-mod fits simplylight.hmod --build nightly` | "content mod, with no code to check against a build; `fits` doesn't apply" (`sdk-fits-content.log`) |
| In-game default | `GAME=…/0b14f8a/control/horizon-client tools/e2e.sh` | **`PASS` + `EXIT:0`** — 2048/2048 states bind, light 15/14/0 as upstream, 9/9 blocks self-drop, six recipes craft against an empty stick control, tags hold, 160 items give and the tab lists 160, one item of every family places, unknown ids refused (`ports/_shared/evidence/simplylight-e2e/e2e-default.log`) |
| Counterfactual (same game, **no** `--dev-mod`) | `COUNTERFACTUAL=1 tools/e2e.sh` | **`PASS` + `EXIT:0`** — the ids are `minecraft:air`/refused there, vanilla glowstone still emits 15 (`e2e-counterfactual.log`) |
| Negative | `NEGATIVE=1 tools/e2e.sh` | **`FAIL` + `EXIT:1`** as required (`e2e-negative.log`) |
| Selftest | `SELFTEST=1 tools/e2e.sh` | **`FAIL` + `EXIT:1`** as required (`e2e-selftest.log`) |
| Log gate | `python3 ports/tools/check-e2e-logs.py simplylight-e2e --strict` | `logs=4 ok=4 missing-exit=0 bad-exit=0`, exit 0 |
| Package gate | `python3 ports/tools/verify-sdk-package.py simplylight` | 1/1 |

The game used is the campaign's plain `0b14f8a` export
(`~/.cache/horizon-e2e/0b14f8a/control/horizon-client`) with the prepared root the
launcher wrote for that pin (`…/versions/nightly-2026-11-18-0b14f8a/.local/horizon-client/26.3`);
the older prepared root at `/home/adlf/Desktop/horizon/.local/horizon-client/26.3` is
missing `fluid_state_definitions` in `block-behaviours.json` and the game refuses to
start with it ("re-run horizon-prepare") — a trap for the next content leg.

Skills: superpowers/using-superpowers,brainstorming,systematic-debugging,verification-before-completion; BeautifulRust/none (no Rust change) — the native crate was removed, not edited.

### Changelog (0.2.0) — the text the registry PR carries

> **0.2.0 — the port becomes a content-only package.** `simplylight` is a content carry of
> Simply Light 1.5.3+1.21.1-b4 (MIT, by Flanks255) for Minecraft Java 26.3: 160 light blocks
> in ten families × sixteen dye colours — off and on lamps, slabs, panels, wall lamps, bulbs,
> rod lamps, three-block lamp posts and edge lights — with 2,048 states, 234 recipes, twelve
> tags, models, textures and seven locales.
> **What changed for `api = "0.2"`:** the package is now a plain content mod — `assets/` and
> `data/` with a `horizon-mod.toml` (API 0.2, `side = "both"`, `capabilities = ["registries"]`)
> — with **no `Cargo.toml`, no `src/` and no `[native]` block**. The port answers no game
> moment: the 160 blocks are content declarations, and the census model the retired bundle
> carried was test-only. Nothing a player sees changed: the 160 ids, 2,048 states, light
> levels, drops, recipes, tags and creative tab are the same bytes.
> Not ported yet: the redstone toggle of the lamps, the lamp-post column's per-position light,
> the edge-light editor screen, tooltips, and the lamp-post item's attack damage.
> Local-only until the maintainer approves publishing (MIT, rank 0).

## Slice: carried-data parity + waterlogging (0.2.1, 2026-11-21, pin `0a875da`)

**Why this slice.** The port's own record claimed the carried data was proven by "six recipes
craft" and "9/9 blocks self-drop" (above, §"Behaviour carried"), while the package ships **234
recipes** and **160 loot tables**, and its gap list names gap 6: *"Fluid behaviour was not
exercised: the waterlogged **states** bind and the templates implement `SimpleWaterloggedBlock`,
but the e2e sets the state rather than flowing water into it."* The parity ruling (AGENTS.md,
2026-11-20) makes every carried recipe, loot table and tag a required in-game assertion — *"a
carried recipe that is never crafted is dead data"* — so the highest-value remaining work at this
pin was to turn the carried data from **registered** into **proven**. Class and tier are unchanged
(**content / data**): no Rust, no native code, because the registry's `kind = "mod"` keeps a
content-only package for every version (see §"Content-only 0.2.0").

**What changed.** `version = "0.2.1"` in `horizon-mod.toml`; two new tools (`tools/data_parity.py`
reads the recipes, tags and loot tables **out of the delivered `.hmod`** and drives the crafting
grid; `tools/data_parity.sh` is the leg, including the waterlogging proof); the shipped `README.md`
was updated (it is **not** packed by a content build — `cmp` and the same SHA-256 prove the data
bytes did not move). The carried data itself is **unchanged**: a static comparison of every file
against the jar shows **234/234 recipes semantically identical** (pattern, key, ingredients, result
id and count) and **160/160 loot tables** in 26.3's singular typed-condition shape.

### The red, before the fix

The first assembled full sweep went red on **10 of the 234** recipes — every white-dye recolour —
from this port's *harness* (the inventory expectation), not from the data:

```
[8/234] edge_light_dyed: 8 simplylight:edge_light (FAIL)
[24/234] edge_light_top_dyed: 8 simplylight:edge_light_top (FAIL)
[42/234] illuminant_block_dyed: 8 simplylight:illuminant_block (FAIL)
[44/234] illuminant_block_on_dyed: 8 simplylight:illuminant_block_on (FAIL)
[95/234] illuminant_panel_dyed: 8 simplylight:illuminant_panel (FAIL)
[141/234] illuminant_slab_dyed: 8 simplylight:illuminant_slab (FAIL)
[173/234] lamp_post_dyed: 8 simplylight:lamp_post (FAIL)
[189/234] lightbulb_dyed: 8 simplylight:lightbulb (FAIL)
[206/234] rodlamp_dyed: 8 simplylight:rodlamp (FAIL)
[222/234] wall_lamp_dyed: 8 simplylight:wall_lamp (FAIL)
FAIL: edge_light_dyed: the inventory holds 8 simplylight:edge_light after crafting, expected 0 (8 made, 8 consumed as ingredients)
```

`FAIL: the carried-data parity sweep failed` / `EXIT:1` —
`ports/_shared/evidence/simplylight-e2e/e2e-parity-firstrun-harness-red.log`. The bug: a white-dye
recipe uses the result item as its own ingredient, so the inventory reads
`given − consumed + produced = produced`, not `produced − consumed`; the expectation was corrected
to `result.count` (a grid that consumed nothing would read `2 × count`, which the assertion still
catches, and the emptied grid is asserted separately).

**No carried-data defect was found.** All 234 result stacks and all 160 loot drops matched at the
first attempt once the harness was right, so there is nothing to "fix" in the data: inventing a
minimal change would violate the upstream-defect rule (*a defect is a broken behaviour, not a design
choice* — and the white-dye recipes are upstream's own, wasteful but not broken).

Two harness reds that cost iterations (recorded so the next leg does not repeat them):

1. **`ctl place` refuses a fluid cell** — `agent.rs` returns *"…holds minecraft:water; break it
   first"* when the target is neither air nor replaceable, so a `SimpleWaterloggedBlock` can never be
   made with it. The emissive proof must use the vanilla right-click path: select the item and
   `ctl interact <x> 100 <z> up` on the support block under the water.
2. **A wall between the player and the support hides its face** —
   `{"error":"no face of block [21, 99, 0] is visible from here: minecraft:stone at [21, 100, 0] is
   in the way"}`. The cells are therefore 10 blocks apart with no walls; a water source does not
   reach even `x+7` on this flat floor (measured: `block 27 100 0` = air).
3. **The shared `~/.cache/horizon-e2e/0a875da/control` export is not stable during a wave.** Another
   job's `launcher-game.sh ensure` re-cut that directory mid-run and the leg died at recipe 208 with
   `ctl click-slot 1: timeout: failed to run command '…/control/horizon-ctl': No such file or
   directory`. Hardlink a private copy
   (`~/.cache/opencode/simplylight-bin-0a875da/{horizon-client,horizon-ctl}`) and pass `GAME=`/`CTL=`;
   the leg then runs on the same bytes regardless of other jobs.
4. **The End's exit-portal frame sits at (0, ~100, 0) once the dragon is killed**, so the fluid cells
   start at `x = 20` and the negative leg probes `20 120 0`; at `(0, 100, 0)` the cell is bedrock and
   `interact` answers `… is in the way`.

### In-game transcript (quoted, `0a875da` control game, `--dev-mod simplylight.hmod`)

Gap 6 — a real placement into water, and its differentials:

```
+ gap 6: flowing water into the eight waterlogged families (The End, real placement)
  water: simplylight:illuminant_panel -> simplylight:illuminant_panel[facing=up,waterlogged=true]
  water: simplylight:illuminant_slab -> simplylight:illuminant_slab[facing=up,waterlogged=true]
  water: simplylight:wall_lamp -> simplylight:wall_lamp[facing=up,waterlogged=true]
  water: simplylight:lightbulb -> simplylight:lightbulb[facing=up,waterlogged=true]
  water: simplylight:rodlamp -> simplylight:rodlamp[axis=y,waterlogged=true]
  water: simplylight:lamp_post -> simplylight:lamp_post[position=bottom,waterlogged=true]
  water: simplylight:edge_light -> simplylight:edge_light[east=false,north=false,south=false,waterlogged=true,west=false]
  water: simplylight:edge_light_top -> simplylight:edge_light_top[east=false,north=false,south=false,waterlogged=true,west=false]
+ the differential: the same placement in air is waterlogged=false
  air: simplylight:illuminant_panel -> simplylight:illuminant_panel[facing=up,waterlogged=false]
  air: simplylight:rodlamp -> simplylight:rodlamp[axis=y,waterlogged=false]
  air: simplylight:lamp_post -> simplylight:lamp_post[position=bottom,waterlogged=false]
  air: simplylight:edge_light -> simplylight:edge_light[east=false,north=false,south=false,waterlogged=false,west=false]
+ the counterfactual: a stateless block placed into water has no waterlogged property
  water: simplylight:illuminant_block -> simplylight:illuminant_block[on=false]
waterlogging PASS: 8/8 waterlogged families take the water at placement
```

Recipes and loot tables, read from the delivered archive:

```
[1/234] bulb: 8 simplylight:lightbulb (ok)
…
[234/234] walllamp: 6 simplylight:wall_lamp (ok)
recipes PASS: 234/234 carried recipes craft their declared result id and count, every ingredient is consumed, and the output lands in the inventory
[1/160] loot edge_light (ok)
…
[160/160] loot wall_lamp_yellow (ok)
unknown table: {"chat": ["Can't find element 'simplylight:blocks/not_a_block' in registry 'minecraft:loot_table'", "...ot_a_block<--[HERE]"], "ok": true}
loot PASS: 160/160 carried loot tables each give exactly one of their own block, and an unknown table is refused
PASS: 234/234 carried recipes craft their declared result id and count with the
unknown table is refused, and the eight waterlogged families take water at placement
EXIT:0
```

(`EXIT:0`, `ports/_shared/evidence/simplylight-e2e/e2e-parity.log`.) The loot table is executed by
`/loot give @s loot simplylight:blocks/<name>` and the reply is parsed as an inventory set — exactly
one of the block and nothing else; the nine break-based drops in `tools/e2e.sh` remain the
block-break path's evidence.

### Fail-capability (every new assertion can fail)

| leg | what it asserts | result | log |
|---|---|---|---|
| `SELFTEST=1` | the waterlogged panel must read `waterlogged=false` | `FAIL: SELFTEST: the deliberately wrong waterlogged=false assertion tripped on 'simplylight:illuminant_panel[facing=up,waterlogged=true]'` + `EXIT:1` | `e2e-parity-selftest.log` |
| `SELFTEST=recipe` | the first recipe must yield one extra item | `FAIL: SELFTEST: bulb: expected result 9 simplylight:lightbulb, got ('simplylight:lightbulb', 8)` + `EXIT:1` | `e2e-parity-selftest-recipe.log` |
| `NEGATIVE=1` | the same world **without** `--dev-mod`: the ids, the loot table and the item must be absent | `without the package, setblock simplylight:illuminant_panel reads 'minecraft:air'`; `/loot …` → `Can't find element 'simplylight:blocks/illuminant_block' in registry 'minecraft:loot_table'`; `FAIL: NEGATIVE: …` + `EXIT:1` | `e2e-parity-negative.log` |

`python3 ports/tools/check-e2e-logs.py simplylight-e2e --strict` → `logs=9 ok=9
missing-exit=0 bad-exit=0`, exit 0.

### Coverage (every family in the jar)

| family | state | proof |
|---|---|---|
| 160 blocks × 2048 states | **tested** | `e2e-default.log`: 160/160 defaults, 2048/2048 states bind |
| light levels (15/14/0) | **tested** | `e2e-default.log`: the family table against glowstone 15 and stone 0 |
| **234 recipes** | **tested** | `e2e-parity.log`: 234/234 craft the declared id and count, ingredients consumed |
| **160 loot tables** | **tested** | `e2e-parity.log`: 160/160 give exactly their own block; 9 break-based in `e2e-default.log`; unknown table refused |
| 13 tags (10 `any_*`, `c:stones`, 2 `c:dusts`) | **tested** | the recipes resolve every tag member; the 2 block tags in `e2e-default.log` |
| creative tab, 160 items give/place | **tested** | `e2e-default.log` |
| waterlogged states (8 families) | **tested** | `e2e-parity.log`: water at placement, the air differential and the stateless counterfactual |
| redstone toggle of the lamps | **not ported** | needs `neighbor_changed` + a native crate; `kind = "mod"` forbids one |
| lamp-post 3-block column, per-position light | **not ported** | placement logic; same rule |
| edge-light sneak-use editor | **not ported** | client screen + packet; same rule |
| shift tooltips | **not ported** | `native_tooltip`; same rule |
| lamp-post item damage, `Equipable` head slot | **not ported** | item behaviour; same rule |
| optional CTM resource pack | **not ported** | a built-in client pack upstream; not carried |
| JEI plugin | **not ported** | Horizon's recipe viewer is built in |
| worldgen | none | the mod ships none |

### Verification (0.2.1, pin `0a875da` / mixins `e59da909…`)

| Check | Command | Result |
|---|---|---|
| Build ×3 | `cargo horizon-mod build` with `rm -f Cargo.lock` before each | `BUILD_EXIT:0`, **689,910 B**, SHA-256 `7bbf9c31…9118b`, `cmp` byte-identical (`sdk-build-0.2.1.log`) |
| Registry check | `cargo horizon-mod check simplylight.hmod` | `simplylight 0.2.1 (mod, API 0.2, both)`, 689,910 B, `no code: content only`, `CHECK_EXIT:0` (`sdk-check-parity.log`) |
| Fits, the campaign pin | `cargo horizon-mod fits simplylight.hmod --build 0a875da` | `Simply Light is a content mod, with no code to check against a build; \`fits\` doesn't apply.` `FITS_EXIT:2` (`sdk-fits.log`) |
| Fits, a different build | `… --build 0b14f8a` | the same line, `FITS_EXIT:2` (`sdk-fits-other.log`) |
| Entries | `unzip -l simplylight.hmod` | **1,455 files**, no `Cargo.lock` (`sdk-entries-0.2.1.log`) |
| Package gate | `python3 ports/tools/verify-sdk-package.py simplylight` | **1/1** |
| Rust ladder | — | not applicable: a content-only package has **no crate** (no `Cargo.toml`, no `src/`) |

**Why both `fits` lines are exit 2.** `fits` compares a *native* package's `[native]` pin with a
build's surface; a content package carries no pin ("made for" cannot be printed for it, at any
build), which is the tool's own verdict and unchanged from 0.2.0. There is no stale `0b14f8a made
for` line here: the package names no build at all.

### Changelog (0.2.1)

> **0.2.1 — the carried data is proved in game, and gap 6 is closed.** No player-visible data
> changed: the 160 blocks, 2,048 states, 234 recipes, 160 loot tables, 12 tags and seven locales are
> the same bytes as 0.2.0 (a content build does not pack the README, so the archive is
> byte-identical apart from the version). What is new is evidence: all **234 recipes** are crafted
> in a real crafting table from the delivered archive, each asserting its declared result id and
> count and that the grid was consumed; all **160 loot tables** are rolled and must give exactly
> their own block (an unknown table is refused); and the **eight waterlogged families** are placed
> into water — not state-set — and must read `waterlogged=true`, with the air differential and the
> stateless `illuminant_block` counterfactual. Two new tools: `tools/data_parity.py` and
> `tools/data_parity.sh`.
> Still not ported: the redstone toggle of the lamps, the lamp-post column's per-position light and
> place/break cascade, the edge-light editor screen, tooltips, and the lamp-post item's attack
> damage — a content-only package (`kind = "mod"` for every version) cannot carry the crate they
> need.
> Local-only until the maintainer approves publishing (MIT, rank 0).

Skills: superpowers/using-superpowers,brainstorming,writing-plans,test-driven-development,systematic-debugging,verification-before-completion; BeautifulRust/none (no Rust change) — a content-only port: the slice adds two in-game harness tools and a version bump, no crate exists to lint
