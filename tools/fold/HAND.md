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

Left where it is, for whoever reworks those files: with BELIEF.on settled, `fix_stamp`, `pt_stamp`, `roomfight::def_fix_is_pt` and
`fix_is_pt` (the stand-ins for the TypeScript's object identity of a man's fix) are still written and read by `def_move_pt`,
`move_pt` and the senses. The program cannot call them dead: they are reached from code that never tested the row.
