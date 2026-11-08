# Look sheets of the cast

Pictures of every kind of person the game puts on a level, for judging clothes, kit and colours before and after a change.
It needs no GPU.

## The cast shaded with the game's own resolved colours

    REPO=<tree> OUT=<folder for the PNGs> [PY=<python with bpy>] [SPP=48] [SHOTS="swatches lineup topdown close head"] \
      sh tools/chars/look/run_blender_look.sh

- `dump_look.ts` (bun) imports the TREE's own `charMaterials` and cast loader (who wears what tints is a table in it, from the Rust
  rules it names) and writes `look.json` (per variant: the representative man, his resolved slot rows, slot shares, picks) and `look.bin` (bind positions, then per variant and vertex the linear albedo a DRY
  pixel stores after `fsSkin`'s crease / wear / dust block, the lobe's roughness and gain).
- `render_look.py` (Blender as the `bpy` module, Cycles on the CPU) reads the tree's `cast.gltf` and those two files. Colours
  go to Principled Base Color as they are (linear, no sRGB decode), roughness to Roughness, 0.25 × gain to Specular.
  Neutral white three-light studio, view transform Standard, exposure 0 (`LOOK_EXPOSURE=<stops>`). It refuses a `look.json`
  made from another cast, so do not rebuild the cast under a running set.
- `REPO` defaults to this checkout. `PY` defaults to `python3`: point it at a Python 3.11 with `bpy` 5.0 installed
  (`pip install bpy`, plus `apt install libegl1` on a bare Linux box). `WORK` is a local scratch folder.

Sheets: `swatches.png` (every man's eleven slot rows as flat chips, no render) · `lineup-front.png` / `lineup-back.png` ·
`topdown-near.png`, `topdown-far.png` and its ×3 crops (the game's camera distance) · `close-<variant>.png` (ten of the
fourteen men by default: both pairs' second men and the second responder differ from the first only in back kit; any
man by hand with `render_look.py <work> close --only operator_4,responder_2`) · `head-<side>.png` (the FIRST man of each
side only: defender_1, the cleaner. A hat, a rig or a tie another man of that side wears is not on it: ask for him with
`head --only defender_2,civilian_2`, which writes `head-<variant>.png`).

Representative men (the first man the game itself would deal each variant; the table is in `look.json`): operator_1 Sam,
operator_2‥5 the squad of four with their pair tints, defender_1‥3 the office shift with its per-route tints, defender_4 / 5
hostile units, responder_1 / 2 the response element, civilian_1 / 2 the cleaner and the late worker.

What the picture is and is not: albedo, roughness and gain are the game's rows per VERTEX (the game evaluates crease, wear
and dust per pixel); the light is a studio and the view transform is Standard, so cloth reads lighter and more saturated
than the game's AgX frame at night. Judge hue and the relations between men, not brightness. No guns, no rig props, no wet
coat. Lineup / close / head stand in `Idle_Loop` frame 1; the top-down sheets mix idle and walk. Helmets and goggles are
skinned wholly to the head bone, so a clip the game never plays on these men can show them off the head: that is the
sheet, not the game.

`render_look.py` began as a copy of `../render_cast.py` (its glTF reader, skinning, studio and cameras): a change to the
cast's format has to be made in both. The wrapper exits 1 if the dump or a sheet family failed and keeps each family's
full output in `WORK/_<family>.log`.

Timings, 4 cores shared with other work: dump 2 s, lineup 2‥4 min, topdown 2‥3 min, head 3 min, close 10‥15 min at
`SPP=28`. Stop a run by PID.
