# How a man moves and looks

The skeleton, the clips, the layers that pose a man, how he falls, what he wears, and how he reaches the GPU. Who decides to crouch, aim or
reload is each rule's page (`player.md`, `squad.md`, `enemies.md`); takedowns and the wall hold are `stealth.md`; how he is lit is `render.md`.

## What the player gets

- Everybody walks, jogs, sprints and crouches on one skeleton, and the feet match the ground's speed.
- A teammate sinks onto bent knees and leans into the gun. A defender walks upright with his elbow out.
- An armed AI man's turn has weight. A heavy gun comes round slower, and his head gets to the new point first.
- A shot kicks, a close round makes a man duck, a surprise makes him start, a hit knocks his trunk, a flash doubles him up.
- His hands do the work you see: the magazine in the well, the door, the goggles, the radio.
- Sam carries himself as a sneak when his hands are empty, and bows his head to the floor to look under a door.
- A man a blast left standing stays hurt: he hunches and limps. A man under fire pulls his head down.
- The dead have weight. A takedown lays a man down along a path, and a man out cold breathes.
- Fourteen outfits on four sides, a little difference in height from man to man. Rain soaks them and dust climbs from the boots.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `character/` | a man: his animator (`Anim`), the order of its layers, his turn, his bones, his falls |
| `anim_skeleton.rs`, `posehome.rs` | poses, masks, sampling a clip, blending, joint matrices; a man's poses kept in the game's pool |
| `ragdoll.rs` | the dead body: sixteen weighted points, sticks, floor and walls |
| `condition.rs` | hurt, shaken and folded, told to the figure every frame (`condition_pose`) |
| `stealthfeed.rs`, `kitanim.rs`, `handwork.rs` | what the game tells Sam's stealth poses; his errands (draw, stow, reload, goggles, sling); where hands go on a thing |
| `rig_props/`, `gunlook.rs` | what he holds and wears as boxes (`rig_boxes`), a gun's grip and muzzle; which models a man has (`gun_look_of`) |
| `cast.rs`, `anim_gltf.rs`, `stature.rs`, `char_look.rs` | his outfit and part mask; his drawn height; which side's dress a team wears |

| file | what it does |
|---|---|
| `rust/sim/src/layers.rs` | the layers themselves, each pose written in place |
| `rust/sim/src/skeleton.rs`, `poses.rs`, `clips.rs` | blends and the two-bone solve; the pose pool in the game's block; the clip store |
| `rust/sim/src/knobs.rs`, `rustknobs.rs` | the numbers and named nodes a layer reads, and who writes them there |
| `rust/game_start/src/gltf.rs` | the character files read for the game: skeleton, clips, the cast's parts and variants |
| `rust/game_draw/src/men.rs` | what the picture is told of a man: sections `Man`, `Inst`, `Bones`, `Cap` |
| `src/draw/men.ts`, `src/anim/characters.ts`, `src/anim/gltf.ts` | those sections handed to the GPU; the mesh's buffers; the mesh read for the page |
| `src/render/charMaterials.ts`, `src/render/sdfrig.ts` | a man shaded by material slot; the round cones the rays meet of him |
| `public/assets/ual/`, `public/assets/chars/` | the mannequin with every clip; `cast.gltf` and `cast-gwot.gltf` |
| `tools/chars/` | the cast's builder and checks (its `README.md` holds the file's contract), look sheets in `tools/chars/look/` |

`rust/sim/src/falls.rs` is falling glass and litter, not men (`destruction.md`).

## How it works

**The inputs.** A rule poses nobody. It writes the animator's fields (`speed`, `crouch_target`, `stance`, `upper`, `long_gun`, `hold_pose`, `kit`) or calls a
one-shot (`anim_fire`, `anim_flinch_from`, `anim_startle_from`, `anim_reload`, `anim_throw_item`). `game_deal_weapons` deals what comes from his gun and side
each frame: the grip, the reload's beat, `anim.trained`, the turn.

**A man's frame.** His own update ends in `character_update`: his turn, `anim_update`, the skeleton, `character_read_bones`, `character_in_hand_pass`. Called with
no time (a re-bake) it moves no clock, which is how a man moved by something else gets bones where he stands.

**The layers.** `anim_update_standing` is the whole order, and a later layer writes over an earlier one: gaits by speed, the crouch, breath and feet; upper-body
clips; stance, wall, kick, carry, long gun; lock work, the hold and the held; the door hand; the aim clips; signals, tells, hands at work; creep, watch, stealth
poses; ready pose, kit errands, reload, throw, blow, hit; the big tool, a lean; the reactions; a whole-body clip (`anim_act_layer`); fingers (`anim_hands_pass`);
goggles and a smoke. Then `twists` carry the aim down the spine. Each layer stands down, by an eased weight, for whatever else has the arms.

**A layer has two halves.** An `anim_*` function in `character/` puts its numbers and kept springs on the bench's `args` and calls the layer in `layers.rs`, which
works on the pose where it lies. What a layer keeps stays in `Anim` and crosses as columns (`lean_cols_get`). It reads numbers by `k(NAME)` and nodes by `role(NAME)`.

**A pose** is ten columns of singles, a row a node. A man's poses live in the game's block (`Poses`), so a rollback restores them. `pose_home_tend` rebuilds the pool when
a man has left, and `stage_open` lends a place to a man made inside the tick. Clips are authored data outside any game, one copy a process.

**Poses feed the game.** `character_read_bones` writes `bones`, `gun_dir` and `muzzle`, and rounds leave that muzzle. `character_capsules` is what a round
(`character_ray_hit`) meets, and the same fifteen capsules are the proxies he casts shadows with. Arm layers that only look (the reload hand, bound wrists, the clutch) keep the
pose under them (`anim_under_pose`), so the game reads joints with `character_joint_under` and a ragdoll starts from that pose.

**The turn.** Between a man's updates `body_yaw` and `aim_yaw` hold the weighed yaws. `character_turn_begin`, at the head of `update_guard`, puts his AI's wish back. Then
`character_turn_weigh` steps body and gun toward the wish with acceleration, a top speed and a brake (`k.turn`, `handling_of`, `anim.trained`). It is on for the guards list but
a held man. Sam and civilians have no turn.

**The long gun.** A set of springs toward one of `LongStance` (`long_targets`). `anim_long_body` sets the trunk and `anim_long_arms` puts both hands on the grip. A pistol's
reload is its clip. A long gun plays none: `anim_reload_carry` brings the gun and head round and `anim_reload_arm` works the support hand, by `reload_at` where the game deals it.

**Reactions.** `mv_pre`, `mv_post_body`, `anim_mv_arms` and their neighbours: the carry, recoil, flinch, startle, hit, dazzle, fold, hurt and shaken. `k.moves` has a switch
for each part (`mv_on`); `k.mv` and `mv2` have the numbers.

**How he falls.** `character_ragdollize` seeds the points from his pose and his joints' last speed, capped so that a pop in a clip is not momentum. A loud death shoves him one of
three ways by his own die (`draw_body`). A quiet one with `fall_as` set follows `character_fall_guide`: pins move along a path for each `FallStyle`, and anything else that pins
him ends it. `ragdoll_step` runs until the body sleeps. Asleep he costs nothing, unless `breathe` is set (`character_breath_frame`).

**What he wears.** `cast_variant_of` picks an outfit by side, roster place or id. `cast_parts` makes the frame's mask (`cast_suit`, `pack_mask`, `cast_goggles`). The game reads the
files for skeleton and clips; the page reads them for the mesh (`loadCharacterSet`), and `castFile` picks the look's file. `?mannequin` draws everybody bare.

**To the GPU.** `Men::take` copies each shown man's bones after a tick, and `Men::frame` blends two ticks, snapping past `SNAP`. Stature scales his matrices about his feet here.
`DrawMen` sends `Inst` and `Bones` as they lie, draws by the mask (`drawRuns`), and works out materials only when what he wears changes. `vsSkin` and `fsSkin` draw him.

**Materials.** A vertex has a slot and a cavity baked from the mesh's curvature (`charSlots`, `bakeCavity`). A man has rows a slot (`writeCharPalette`) from his side's table
(`CHAR_MATS`, `CHAR_MATS_LOOK`, `SNEAK_SUIT`). `engine.settings.charMats` goes on for a mesh that keeps the slot names. A finish goes through the wet coat's channel (`packWet`,
`wetCoatSpec`). Bounces read one colour a proxy (`proxyAlbedos`). A wet man's numbers are rows of `Inst`, from `k.wet` (`weather.md`).

**The shadow rig.** `fitRig` fits the fifteen proxies round cones against the mesh, once an outfit, and `applyRig` puts the radii on the frame's list (`render.md`).

## Rules that hold

- **A pose is the game's.** What must not move the game goes under `anim_under_pose`, or stays off the arm and trunk a rule reads.
- **The turn's two reads.** Inside his own update a fire test reads `character_gun_yaw`, because `aim_yaw` there is where he wants the gun. An easing of his yaw from outside
  starts from `character_want_body`.
- **A re-bake takes nothing:** no one-frame flag (`kick_hit`), no carry word (`swap_clock`).
- **No die but his own.** A fall's die is `draw_body`; an outfit and a height come from his id (`hash01`).
- **The crouch walk is carried level.** "the crouch walk animation looks like waddling" (`steady_layer`, `k.moves.steady`), by the crouch walk's share of the pose alone. A value
  over 1.5 also raises the pelvis, which shows more of him over cover, so that is not the default.
- **The rifle sprint stays on the gun.** "sprint animation still has some waddling" (`k.moves.steady_run`), by the sprint's share alone.
- **Run arms come in.** "sprint looks dainty when viewed from the back, maybe should have arms closer in?" (`run_arms`, `k.runarms`).
- **Sam sprints a pistol at low ready.** "sprinting with pistol animation is weird, should keep two hands on it and at low ready, like rifle sprinting animation" (`run_ready`).
- **A man who has run breathes hard.** "characters should breathe more forcefully when sprinting (e.g. on snow levels)" (`k.hardbreath`). Effort is ground speed: no stamina exists.
- **The rifle hangs by the mission.** "unequipping rifle should be putting it down in a sling, not over the back, right?" In front on a loud mission, on his back on a quiet
  one (`kit_sling_front`, `k.kitanim.sling`).
- **A long gun's reload runs the clip's clock and plays no clip,** so whatever asks `anim_reloading` hears the same.
- **Everyone favors his right when hurt** (`hurt_side`). Only a blast hurts a man (`condition_hurt`). A round kills or knocks out: there are no wounds.
- **Off by default:** `k.moves.entry`, a teammate's muzzle dipped past a friend's back. It moves the bore, the rounds and the weapon light.
- **Stature is looks only.** Bones, capsules and boxes stay where the game has them. A player's man is 1, and two bodies that must meet are both 1 (`paired`).
- **Slots are the contract with the meshes:** the names of `CHAR_SLOTS`, whose order never changes. Another material name stops the load. No file has a texture.
- **Limits.** 64 joints a man (`JOINT_STRIDE`), 16 men drawn (`MOST`), 8 with mesh shadows (`MESH_MAX_CHARS`), 32 parts to a cast.

## To change it

- **A number:** `knobs/character.rs` (`hold`, `moves`, `mv`, `turn`, `runarms`, `runready`, `hardbreath`, `kit`), `knobs/stealthpose.rs`, `knobs/kitanim.rs`, `knobs/condition.rs`. The panel or
  `?knob.moves.steady=0` turns one. Tables with no panel are functions in `rust/game_types/src/character/types.rs`: `stances`, `blend`, `sneak`, `creep`, `life`, `mv2`, `long_reload`.
- **A number for a layer:** a line in `rust/sim/src/knobs.rs` and its `k[…]` line in `rustknobs.rs`.
- **A new layer:** a `pub fn` in `layers.rs`; an `anim_*` and `anim_*_ask` pair (copy `anim_lean_body`); a line in `anim_update_standing` where it should write over what is
  under it; its kept springs in `Anim`, with columns. Add it to the stand-down lists of what it gives way to.
- **A one-shot:** an `anim_*` function that starts a clock in `Anim` (`sig_t`), advanced and read in the layer's place. Rules call that function, never the layer.
- **A new clip:** put the animation in `AnimationLibrary_Godot_Standard.gltf`. `gltf.rs` takes every animation by name. For a whole-body clip a rule sets `anim.act` to an `AnimAct`
  with its name. In a layer, call `sample_clip` with `clip_of` and blend under a mask. A clip the game cannot do without goes in `need` in `anim_new`, which panics without it.
  The cast files carry no clips: they use the mannequin's by joint name.
- **A part or an outfit:** `PARTS` and `VARIANTS` in `tools/chars/build_cast.py`, then `python3 tools/chars/build_cast.py && python3 tools/chars/check_cast.py`, the same with `--look gwot`,
  then `cast-probes.ts`. Both files keep the same fourteen names on the same sides.
- **A colour or a finish:** the tables in `charMaterials.ts`; `tools/chars/look/README.md` makes sheets. A new slot goes last in `CHAR_SLOTS`, `SLOT_NAMES` and `tools/chars/kit.py`,
  and `CHAR_MAT_ROWS` has no row spare.
- After Rust changes a page needs `bun run tools/blitz/face-build.ts`.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `clip_store.rs` | forty games hold one set of clips |
| `eyes.rs` | his pose turns his eyes and does not tip them |
| `men_under_fire.rs` | the duck at a close round, the start of a surprised man |
| `hands.rs` | a magazine change goes on whoever has the rest of him |
| `held_takedown.rs`, `the_grab.rs` | a blow lands inside its takedown, a choke lowers a man to the floor |
| `kit.rs` | what a player's hands do, which carriage a gun shows |
| `draw.rs` | drawing changes nothing, and what is drawn between two ticks stands between them |

Unit tests stand beside the code (`swap_clock_beats`, `blend_is_the_typescripts_bits`, `a_blast_leaves_a_man_hurt_and_writes_his_stamp`). `gate`'s pins move when a pose moves what the game reads.
Rows of `tools/qa/checks.ts`: `cast-probes.ts`, `charmats-probes.ts`, `sdfrig-probes.ts`, `frame-layout.ts`. By hand: `tools/chars/check_cast.py`.
No script of `tools/qa/pages.ts` is this system's own.

## Not seen

No check looks at a man. A pose is judged by its numbers, by offline renders (`tools/chars/render_cast.py`, the clips and not the layers) or by playing. How the limp reads at
game size and how the falls look in motion are unseen on a GPU.

Known faults: `k.turn.on` is read by nothing, and `k.moves.on` reaches `layers.rs` but not `mv_on`, so neither switches its whole pass off. The sections of `tools/chars/README.md` after
its file contract name checks that are gone. On the meshes, the shawl lies close as a yoke, short sleeves have a squared cap, and a shirt's hem does not swing.
