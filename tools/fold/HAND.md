# What a hand did beside the program

The program's own report lists what it met and would not take. This file is the edits a hand made after it, so they can be made
again when the program is run on a later tree.

## With the rows settled on 11-07 (BELIEF.on, SNOW.on and the rest of that commit)

The program folds a body's reads; it does not judge a unit test. A test that set a folded row to its OTHER value was a test of the
side that is gone and could only fail now:

- `belief/p_01.rs`, tests: `stand(on)` is `stand()` (it set `g.k.belief.on`); the test `rule_off_one_field_and_duties_stay` is
  deleted; the last two lines of `a_word_stands_against_what_is_heard_of_then_or_before` (rule off, no word stands) are deleted.
- `roomfight/p_02.rs`, tests: `rule_off_the_fix_is_pt_until_somebody_else_writes_it` is deleted.
- `roomfight/p_03.rs`, tests: `the_post_moved_takes_what_is_on_it` ends before its rule-off half (`cover_pt` as a new fix once).
- `weather_kinds/p_01.rs`, tests: the last two lines of the kinds test (`SNOW.on` off, no level is a snow level) are deleted.

A row read through a parameter typed with its table's struct is not folded by the program (a caller may hand in a table of its own,
and a unit test does). Where every caller hands in the game's own table, it is folded by hand and the test's other side goes:

- `wallsnap/p_01.rs`: `take_secs` (WALLSNAP.slow, yes) and `press_near` (WALLSNAP.pressRun, yes) take the yes side; the halves of
  `take_secs_bits` and `press_near_bits` that set the row to no are deleted.

A settled row a body writes is kept by the program. Where the write lays the page's own value over itself it goes by hand, and the
program then folds the row:

- `fogofwar/p_02.rs`, the renderer's block: for a few frames after the engine replay's word (`fog_of_war_page_look`) the look's rows
  FOG.desat, .edge, .air and .kept were laid from the module's copy of the page's values. Once the dead code is gone that lay is the
  rows' only writer, and what it writes is what they hold: the lay is deleted (the count of frames still runs down), and FOG.air and
  FOG.kept fold (four reads in `fogofwar/p_02.rs` and `p_03.rs`). FOG.desat and FOG.edge are numbers and stay numbers.

READING TWINS, for the readers pass (tools/fold/README.md, "Readers"). A block of the sim was handed out as `&'static mut` from a slot
number whether the caller read it or wrote it, so no function that touched one could be called a reader. Beside each such function now
stands a twin that hands the same thing out shared, named `…_read`; the pass moves every call that compiles onto the twin.
- rust/sim: `state::state_read`, `weather::of_read`, `world::world_read`, `skeleton::bench_read`, `clips::table_read`.
- the bodies' helpers: `lights_read` (sim_lightshome), `cols_read` (sim_slabshome, props), `io_read` (props), `wx_read` (weather),
  `air_read` (air_dust), `args_read` (character p_08, p_10, p_11), `world_read` (scene_box_mirror), `block_d_read` (sim_weatherhome),
  `step_io_read` (doors), `slab_par_read` (doorblast), `pool_at_read`, `cols_at_read`, `cols_of_read` (anim_skeleton).
Not given a twin: `panes::pn` (it tidies an empty pool's counts as it hands it out) and `panes::row_ref` with glassbreak's `row_at`
(a row is reached through `&mut self`); `air::plan` (it makes the plan it hands out).
STILL UNSOUND, and not mended here: the `mut` functions can hand out two live `&'static mut` to one block (two `state(slot)` in one
expression already do), and a `…_read` held across a call that writes the same part aliases a shared reference with a write. One
thread and "take it, use it, let it go" is the only rule, held by habit. Who owns a block is for the data design.

Left where it is, for whoever reworks those files: with BELIEF.on settled, `fix_stamp`, `pt_stamp`, `roomfight::def_fix_is_pt` and
`fix_is_pt` (the stand-ins for the TypeScript's object identity of a man's fix) are still written and read by `def_move_pt`,
`move_pt` and the senses. The program cannot call them dead: they are reached from code that never tested the row.

## With the settings the shader cleanup removed (the drop of 49 fields of `RenderSettings`)

Two settings were always false and still read: `rtMeshHits` and `sdfRigSmooth`. By hand, in TypeScript: the one read in
`src/game/game.ts` (`oneColourHits`, now the plain `false` handed to `looks.proxies`), the same expression in `src/viewer/runtime.ts`,
their two lines of the interface and of `defaultSettings()` in `src/settings.ts`, their rows of `tools/qa/settings-check.ts` (with
`meshShadows`, whose only read in game.ts was in that expression) and the key `smooth` of `tools/qa/rig-identity.js`. After that
`fold fields-gone` lists them with the other 47 and the three commands of the README drop all 49.
Left: `whole`, the last parameter of `charLook.ts proxies` and of `render/charMaterials.ts proxyAlbedos`, is `false` at every call
now; the comments at `src/ui/quality.ts:215` and `src/render/frame.ts:142` still name the two settings.

## Before the dead-code pass on the fourth batch (the wrappers that took a query as text or as words)

`spec_of_query`, `weather_of_search`, `squad_of_query` and `row_of_query` each only chose between a text and words already read and
called the function under it; nothing but two unit tests called them. Those two tests (`a_query_naming_no_scenario_is_none`,
`kinds_of_search`) test live code through the wrapper, and the pass would have deleted them with it. By hand: each test calls the
live function on `Query::parse(text)` itself, and the wrapper's sentence of what it answers moved onto the function under it.
The pass then deletes the four wrappers by name and no test.

