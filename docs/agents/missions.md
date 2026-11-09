# Missions

What is played: the list, how each kind of mission is staged, won, lost and started over, the tutorials and their level scripts. The screens are
`menus.md`; how men fight, and the response team, `enemies.md` and `team-feel.md`; sneaking and fog `stealth.md`; what a level file holds `levels.md` and the
`level` skill; saves `replays.md`.

## What the player gets

- **Room clear:** take a building, room by room, with a squad. When every room's men are down or gone and nobody is still coming, it's yours.
- **Hostage rescue:** the clock starts when you do. The captor counts, then shoots a hostage. Lead them out, or clear the building.
- **Quiet, and the solo night:** reach the place, take the thing, and leave by any way out. Being seen ends nothing; it only makes it harder.
- **Par:** one room clear is also a race.
- **Tutorials:** one big line says what to press, and it moves on when you've done it. What you haven't been taught doesn't work yet.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `scene_scenario_list/` | an address read (`spec_of`), a row's link (`pick_search`), the short list (`PICKS`) |
| `start_list/` | the rows in their groups, a mission's sheet and settings, as the desk (`rust/game_start/src/host/desk.rs`) answers the page |
| `scenario_run/` | the one way from an address to a staged game; what is up; quit |
| `roomclear/`, `squaddeal/` | rooms, marks and the two ends; squads cut along a house whose file names none |
| `nightbreach/`, `breach/` | the staging a room clear stands on; F's drill |
| `rescue/`, `rescueend/` | a rescue's cast, F's entry, the captor's clock; the hostages led, and the win |
| `rescueflow/`, `rescuecheck/`, `rescueflush/` | who covers the openings Sam runs past; their radio check; the men left coming for the hostages |
| `quiet/`, `quietfog/`, `takecheck/` | a quiet mission's deal and partner, its fog lever, whether anybody is told the thing is gone |
| `mission/` | the stealth stages, the run's counts, the one objective line (`objective`) |
| `verdict/`, `par/` | how a run ended, latched; the medal clock |
| `lesson/`, `rails/` | a tutorial's steps and tests; the frame over them: which keys work, the view, what is said |
| `director/`, `script_tut_*/` | level scripts, and the six that exist |

The table of missions is `scenarios`, `quiet_missions` and `tutorial_missions` in `rust/game_types/src/scene_scenario_list/rows.rs`, and there is no other: the page asks the desk
(`missions` every row, `kind` what an address starts, `pick` where a short-list name alone goes, `leave`, `brief`), and a tool asks the same desk through `tools/desk.ts`. A level's
mission data is its file's `scenarios` block (`LevelScenarios`). Page half: `src/ui/missions.ts`, `startpage.ts`, `verdictCard.ts`, `parBest.ts`, `railcard.ts`. The tutorials'
scripted players: `rust/game_start/src/lessons/`.

## How it works

**A row is a link.** `pick_search` makes the address: `?levelFile=<file>&roomclear`, or `rescue`, `quiet`, `nightbreach`; `?solo` for the solo night. A pasted address starts the same
run. `offList` rows keep their level and address and aren't drawn (`?offlist=0` draws them). With no `group` of its own, a row's kind and team
place it (`group_of`). `?pick=compound` goes to that short-list row, and Next on the cards walks `PICKS`.

| group | rows drawn |
|---|---|
| Tutorial | `tut-road`, `tut-team`, `tut-kit`, `tut-queue` |
| Raids | `room-clear-pair`, `records-pair`, `depot-four` |
| Strongholds | `block-four`, `warehouse-four`, `compound-four`, `mousehole-four`, `pass-four` |
| On the clock | `hostage-rescue`, `rescue-consulate`, `killhouse-four` |
| Quiet, alone | the solo night, `bazaar-quiet`, `vault-quiet` |
| Quiet, as two | `lights-out-two` |

**The one path.** `start_at_address` → `start_from_query`: the row's hour (`row_settings`), fog, weather, `spec_of`, then `start_scenario`, which calls the kind's own start. A refusal comes
back in words and leaves the plain floor. Each tick, `step_seats_on_first` runs `rails_gate` first, then `director_tick`, the four kinds' ticks, and later `update_mission`, `par_tick`,
`lesson_tick` and `rails_tick`. `verdict_tick` follows the whole step.

**Room clear** is the night breach's staging on the level's own layout: `room_clear_scenario_start` calls `night_breach_start`. `StructureData` holds the rooms (bounds, doors, `garrison`,
more marks than that), `team`, other `starts`, `maxMen`, `squads`. A room is cleared while none of its men holds (`holds`: not down, not fled) and nobody loose stands in it; it can be
held again. **Taken:** nobody holds and `response_up` is false. **Wiped:** every player down. The squad falling leaves Sam playing on.

**Night breach:** office only (`scenarios.nightBreach`). F stacks the pair (`night_breach_order`), F again sends it. It ends when every defender is down or fled.

**Rescue** (`RescueData`): a captor, a door man, room guards, roamers, hostages. `rescue_start_clock` fires on Sam's first move, shot or order (`k.rescue.start`). The captor's clock
(`rescue_clock`) runs `resolve_secs` to the first hostage and `second_hostage_secs` to the next; a flash holds it, and his death stops it. Each defender takes `set_secs` to be ready after he
learns of the assault. F is the entry on the hostage room (`rescue_order_door`). The room held is only "secured". **Won** (`rescue_end_won`): nobody of theirs left for `clear_secs`, or every
living hostage within `out_m` of the team's start, having been led (`rescue_end_word`). **Lost** (`rescue_outcome`): all hostages dead, one hit by your own fire, or Sam down.

**Quiet** (`QuietData`): `starts`, `spots`, `exits`, `patrols`, `thing`. `quiet_mission_before_restart` writes the deal into `g.level.mission` and benches the routes not manned. The stages run
in `update_mission`: Infiltrate, Drive (hold on the holder for `DRIVE_SECS`; walking off, sprinting or hauling a body lets go), Exfil, Done past any exit (`out_now`). Every player down is
Failed. `take_told` is true where the spot is `wired`, and then a calm man is sent to look. `squad=1` adds a partner: pair rules and silence held on (`Held`, put back by
`quiet_mission_release`), staff hostile to him. The **solo night** is those stages on the office's own mission with no scenario up.

**How it ended** (`ended` in `verdict/b_01.rs`): DOWN, CLEAR, WIPED, RESCUED, LOST, OUT or FAILED. `verdict_tick` latches one `RunVerdict`, which never changes for that run.
The objective line is one read, first speaker wins: script, lesson, room clear with its par, rescue, night breach, stages.

**Starting over.** Enter while down loads this run's newest checkpoint if there is one, else restarts; Shift+Enter always restarts (`rust/game_rules/src/player/p_02.rs`). Inside
`game_restart_encounter`, the first act is the quiet deal and the last is each scenario's staging again. A tutorial step with `checkpoint` keeps one as it comes up (`checkpoints_keep`). A save
carries the quiet deal, the lesson's step and a script's place in its beat.

**Deals and ways in.** Every try starts the same. `k.roomclear.seed` is 0, which `place_defenders` reads as each room's first `garrison` marks in file order; `?roomclear=3` shuffles them off that
seed. A rescue stands its first roamer marks. A quiet mission with no pin takes the first of each list; `?quiet=3` deals off that number (`quiet_mission_deal_one`). `squad_deal` cuts a house
with no `squads` block by head count (`k.squaddeal`), never on a lesson. A way in is an entry of `starts`. The link's `way=N`
picks one: none, 1, or no number is the level's own, past the last is the last. One reader (`roomclear` `way_asked`, `way_taken`) serves the staging, the sheet and the kit picture; only a room
clear has ways. `g.stood.way` keeps it through a restart. `tests/way_in.rs` stands every way from the sheet's link.

**Par** (`ParData`, `scenarios.par`; only `killhouse.json` has one). `par_tick` starts on the first order, a stride of `PAR_START_M` off the mark, or a round, and stops at the room clear's own
end; only a taken house gets a time. Each teammate down adds `loss_secs`, and the medal is judged in whole seconds, as shown.

**The response team as a clock.** A level with `scenarios.response` has one. It keeps a room clear from being Taken while its men are up. The rest is `enemies.md`.

**A tutorial** is a `scenarios.lesson` block on a mission of any kind: `steps` and a `rails` record. `lesson_tick` asks the step's test (`done`, a `LessonTest`), holds it `hold` seconds, waits
`settle`, then moves on. `rails_gate` takes out of the tick every key the step hasn't opened (`rails_allowed`: `rails.allow`, each step's `allow` and `deny`), and `rails_order_rows` trims the menu.
`rails_answer`, once as a step comes up, sets its fog, signals its `cue` and keeps its `checkpoint`. A step's `explain` is the card that holds the world. With `respawn`, a teammate who falls is
spared on one knee (`rails_spare`). While steps remain, a room clear ends only by every player falling (`rails_holds_end`); past the last it's Taken. A lesson's address reads no difficulty lever.

**Level scripts.** `level_script_of` names a script by the level's file name, and only the six tutorials have one. `manifest_of` makes a manifest of marks, doors and programs, `lint_manifest`
checks it against the level, and a bad one isn't armed. `rails_start_program` starts the program named `tutorial` at each run; `director_tick` resumes its beats. A beat is a step function over a plain
frame (`BeatFrame`, `beat_next`), yielding a `Cond`, so it saves whole. A `cue` reaches a beat as the signal `lesson:<cue>`, and a beat says it has played out as `done:<name>` (`rails_cued`).

**The breach drill** (`breach_drill_tick`): wait, approach, stack, breach, kick, bang, enter, clear, restack. `stage_checked` starts one where the level has `plans.breach` and there are two
teammates. Only `office.json` has that plan. Elsewhere every teammate is the order menu's from the start.

## Rules that hold

- **Every try is the same try.** "i'd also prefer we make the puzzles more curated and interesting rather than relying too much on randomness". A restart equals a fresh staging.
- **A rescue ends one of two ways.** "the hostage rescue missions should have you safely extract the hostage or fully clear the map." A room stood in wins nothing.
- **No head counts on a row.** "we should also remove all defender counts from the menus, etc., it should be a mystery!"
- **Standard or Hard is the one choice a short-list row offers.** No switches "other than easy/hard".
- **The response team is level design.** "let's make it part of the level design and always make sense (ideally w.r.t. the concept/design of the level) and not all levels
  should have it".
- **A tutorial has a level of its own.** "each tutorial level should have its own awesome level".
- **A step moves on when the thing is done,** never on a timer, and a key isn't named before its step opens it (`rails_hushes`).
- **A cut row stays a row.** "let's also remove "night breach" since it's unnecessary now": no list draws it, and its address and checks still stage it.
- **A level has no rooms in code.** `check_structure`, `check_squads`, `check_rescue` and `check_quiet` hold the file's rooms and marks to the built floor.

## To change it

- **A number:** `rust/game_types/src/knobs/` (`roomclear.rs`, `rescue.rs`, `rescueend.rs`, `rails.rs`, `squaddeal.rs`), or `?knob.rescue.resolve_secs=40`. Medal times: the level.
- **A new mission of a kind that exists:** follow the `level` skill: a generator in `tools/levels/`, `tools/levels/export.ts`, the `scenarios` block. Then the row in
  `rows.rs`, the module made, and `bun run tools/qa/level-needs.ts <name>`. A room clear also goes in `LEVELS` in `rust/game_start/src/bin/missions.rs`. The gate's `played` rows find it themselves.
- **Cut one:** `off_list` on its row. A level whose mission's own rules left stays a plain room clear, as `snatch.json` is.
- **A new kind:** `ScenarioEntryKind` and `ScenarioSpecKind`, `kind_of`, `SCENARIO_KEYS`, `start_scenario`, `active_scenario`, an arm in `ended` and in `objective`,
  a tick line in `step_seats_on_first`, and its staging at the end of `game_restart_encounter`.
- **A tutorial step:** the level's generator (`tools/levels/tut-road.ts` and its like), then its deed in the scripted player (`road.rs`). A new kind of test is an arm of `LessonTest` and `test_ok`.
- **A new script:** a `script_tut_*/` folder, an arm of `LevelScriptFn`, `level_script_of`, `beat_frame_new` and `beat_next`.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `lessons.rs` | each tutorial played by keys to its end (`cargo test --release --test lessons -- --ignored`); each block on its floor; a checkpoint carries the step |
| `lesson_view.rs`, `tut_queue.rs` | what the HUD is told of a step; the keys a step opens |
| `quiet_mission.rs` | a quiet run to each way out; walking out early ends nothing; a save carries the deal |
| `rescue_clock.rs`, `rescue_ends.rs`, `rescue_orders.rs` | the captor's clock; the wins and the room just stood in; F and the openings |
| `par.rs`, `cut_missions.rs` | the clock's start, stop, losses and medal; levels whose mission left |
| `response_team.rs`, `last_men.rs` | what calls the team and its clock; when the last few count as down |
| `restarts.rs`, `endcard.rs`, `debrief.rs` | a restart equals a fresh staging; the end card comes once, with its record |
| `does_and_stages.rs`, `owes.rs`, `hands_off.rs` | every file that names a mission stages it; what a whole run owes; a room clear with nobody at the keys |
| `desk.rs`, `program_moment.rs` | the list and sheet as the page has them; a checkpoint in the middle of a beat changes nothing |

`gate long` adds a `played` row for each staged mission and a `lesson` row for each tutorial; the quick `gate` has `par` and `start` rows. `missions` plays the room clears under a dull commander, and
`missions diff BEFORE AFTER` compares builds. `levels second-way`, `stands`, `walked` and `stacks` ask what a level owes. Checks: `mystery-check.ts`, `writing-check.ts`. Page scripts:
`tut-road-page.cjs`, `startpage-look.cjs`, `cards-page.ts`.

## Not seen

- **The other ways in** have stood only natively: nobody has played one on a GPU.
- **Left over:** `fresh_seed_of` and `g.room_clear.fresh` decide nothing. `rails.sky` is on no level. The Director's show path (`director_enter_show`) has no script. `set_daylight` is
  only ever called to turn daylight off, so `level.daylight` is never seen.
- Most rows carry `untuned`. The tutorials are checked mostly by the scripted player, not by hand on a GPU.
