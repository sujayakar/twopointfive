# Look sheets of the cast

Pictures of every kind of person the game puts on a level, for judging clothes, kit and colors before and after a change. It needs no GPU.

## The cast shaded with the game's own resolved colors

    REPO=<tree> OUT=<folder for the PNGs> CAST_GLTF=<tree>/public/assets/chars/cast-gwot.gltf \
      [PY=<python with bpy>] [SPP=48] [SHOTS="swatches lineup topdown close head"] sh tools/chars/look/run_blender_look.sh

- `dump_look.ts` (bun) imports the tree's own `charMaterials.ts` and cast loader (who wears what tints is a table in it, from the Rust rules it names) and
  writes look.json (per variant: the representative man, his resolved slot rows, slot shares, picks) and look.bin (bind positions, then per variant and
  vertex the linear albedo a DRY pixel stores after `fsSkin`'s crease / wear / dust block, the lobe's roughness and gain).
- **Which cast.** The dump takes the look's dress and its file, `cast-gwot.gltf`, unless `TPS_LOOK=0` is in front (the office's dress, `cast.gltf`).
  `render_look.py` reads `cast.gltf` unless `CAST_GLTF` names another, and neither the wrapper nor the dump sets it. So say `CAST_GLTF` as above, or give
  `TPS_LOOK=0` and no `CAST_GLTF`: with neither, the sheets stop with "look.json is of another cast". That refusal is also why you do not rebuild the
  cast under a running set.
- `render_look.py` (Blender as the `bpy` module, Cycles on the CPU) reads the cast and those two files. Colors go to Principled Base Color as they are
  (linear, no sRGB decode), roughness to Roughness, 0.25 × gain to Specular. Neutral white three-light studio, view transform Standard, exposure 0
  (`LOOK_EXPOSURE=<stops>`).
- `REPO` defaults to this checkout. `PY` defaults to `python3`: point it at a Python with `bpy` installed (`pip install bpy`, plus
  `apt install libegl1` on a bare Linux box). `WORK` is a local scratch folder.

Sheets: swatches.png (every man's eleven slot rows as flat chips, no render) · lineup-front.png and lineup-back.png · topdown-near.png, topdown-far.png
and its ×3 crops (the game's camera distance) · close-<variant>.png (ten of the fourteen men by default; any man by hand with
`render_look.py <work> close --only operator_4,responder_2`) · head-<side>.png (the FIRST man of each side only. A hat, a rig or a tie another man of that
side wears is not on it: ask for him with `head --only defender_2,civilian_2`, which writes head-<variant>.png).

Representative men (the first man the game itself would deal each variant; the table is in look.json): operator_1 Sam, operator_2‥5 the squad of four
with their pair tints, defender_1‥3 the office shift with its per-route tints, defender_4 and defender_5 hostile units, responder_1 and responder_2 the
response team, civilian_1 and civilian_2 the cleaner and the late worker.

What the picture is and is not: albedo, roughness and gain are the game's rows per VERTEX (the game evaluates crease, wear and dust per pixel); the light is
a studio and the view transform is Standard, so cloth reads lighter and more saturated than the game's AgX frame at night. Judge hue and the relations
between men, not brightness. No guns, no rig props, no wet coat. Lineup, close and head stand in `Idle_Loop` frame 1; the top-down sheets mix idle and
walk. Helmets and goggles are skinned wholly to the head bone, so a clip the game never plays on these men can show them off the head: that is the sheet,
not the game.

`render_look.py` began as a copy of `render_cast.py` (its glTF reader, skinning, studio and cameras): a change to the cast's format has to be made in both.
The wrapper exits 1 if the dump or a sheet family failed and keeps each family's full output in a _<family>.log file in `WORK`. The close sheets are the
slowest family. Stop a run by PID.
