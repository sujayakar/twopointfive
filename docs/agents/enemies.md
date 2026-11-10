# Defenders, night staff and civilians

The men Sam is up against and the people caught in it: what they sense, believe and do. `ai-architecture.md` holds the design of a man's records
(`control/`, `owner.rs`, `belief.rs`). `stealth.md` holds attention and the fog, `team-feel.md` the room fight, `missions.md` a rescue's crew,
`dialogue.md` the cues and words.

## What the player gets

- Everybody sees and hears by the light, smoke and sound paths the picture shows. Stay dark and quiet and nobody knows.
- Night staff walk rounds, go for coffee, doze in chairs, sit at screens and call in by radio. A body in the dark stays hidden; a missed radio check
  does not.
- Defenders hold rooms in squads, talk on a net you can hear, wedge doors, go round by another door, respect a corridor that cost them, and throw
  grenades with a warning.
- They jump when surprised, duck at near rounds, fear the dark and a silent net, and a green man may drop his gun and run.
- Civilians notice you, scream, run to a phone or a guard, and tell what they saw. Told to stay down, they stay down.
- Hard means better trained men who act sooner. It never means more hit points.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/`, `knobs/`:

| module | what it does |
|---|---|
| `guards/` | one man's tick (`frame.rs`), `senses/`, `behave/`, the floor's alarm and radio check |
| `mind.rs`, `belief.rs`, `control/`, `owner.rs` | the one door to his mind, what his gun covers, who has his body, whose man he is |
| `difficulty.rs` | the levels and levers, written over the `pressure/` and `digin/` tables |
| `enemysquads/`, `squaddeal.rs`, `stayask.rs` | squads, their net, choices and radio check; squads cut from rooms; squads told to stay |
| `pressure/`, `digin/`, `laststand.rs` | a room fight's press, furniture and wedges, the last few |
| `tactics/`, `lanes/`, `doorforce.rs` | pursuit and doorways, covered lanes, a shut door between him and his man |
| `suppression.rs`, `flinch.rs`, `startle.rs`, `morale.rs`, `runner.rs`, `netquiet/` | nerve: pinned, ducked, surprised, shaken, gone, and a net that does not answer |
| `darkdoor.rs`, `darkfoe.rs`, `calmtorch.rs`, `powerout.rs` | men and the dark |
| `gunhold.rs`, `gunroles.rs`, `frag/` (`frag_throw_tick`) | Sam's pistol in a hold; who carries which gun; their grenades |
| `qrf/`, `squadcast.rs` | the response team, and its room drill by the squad's own code |
| `needs.rs`, `doze.rs`, `smokebreak.rs` | men at their ease |
| `civilians/`, `rescuecheck.rs`, `rescueflush.rs` | civilians and hostages; a rescue's men checking in and coming for the hostages |

The start card's difficulty row is `difficulty_row` in `start_list/`.

## How it works

**The tick.** `step_seats_on_first` runs the floor's rules (`update_suppression`, `room_fight_tick`, `morale_tick` and the rest, to `frag_tick`), then
`update_guard` for each man, then `door_force_tick`, `update_civilians`, `update_escalation`, `update_qrf`. `Team::Guard` is night staff on routes;
`Team::Hostile` is defenders, posted by the room fight. Both run `update_guard`, whose head lists its parts.

**Senses.** `sense` runs every tick whoever has his body; `kept_elsewhere` names the two kinds of man sensed elsewhere (a teammate under an order, a
response man under his script). `can_see` asks reach, cone, line and smoke, and `sight_gain` turns the light on the target into awareness
(`GuardEyeK`, with `far_range` for a lit man past `range`). `hearing_check` takes each event once, by the way it came (`hear_path`). `body_in_view`
asks the light on a body (`BodyLightK`), so a body in the dark is found only from close.

**Mind.** All of it goes through `mind`: `raise`, `cap`, `set`, `set_state`, `fix`, `foe`, each with who wrote it. Awareness is 0 to 1: at 0.3 a calm
man turns suspicious, at 0.85 alert, and under 0.12 a suspicious man goes back to patrol. His fix (`last_known`) is where he thinks his man is.
`gun_point` says where his gun goes with no man to shoot: his eyes this tick, else his duty, else his belief. `word` places a report.

**States**, one file each under `behave/`. `patrol`: hold, lockdown duties, a need, a doze, the route. `suspicious`: a task, a found body, a walk to
his fix and a look round. `alert`: a cover rule, his reaction beat, a standoff, shooting, then `cover_blind`, `door_force`, `tactics_alert` or
`chase_fix`. `search`: `search_plan_for` deals spots round the fix, dark and covered ones first.

**The floor's alarm.** `escalation` is 0, 1 or 2 (`raise_escalation`, `step_down_escalation`). From 1 men hear keener and walk brisker; at 2
`supervise_lockdown` deals followers and posts. `update_rollcall` is the staff's radio check, which is how a death nobody found comes out.

**Who has him.** The body list in `control/` is `frag`, `runner`, `morale`, `startle`, `duck`, `lost`, `rally`; the first to answer wins. The first four
are asked before his own script and state, the last two (his room is lost; a fight next door) at his post.
Rules that task him set a bit in `owner.rs` (`taskers!` in its types, strongest first), and a rule asks `kept` before
it takes him. That order is why a squad's man is not the press's.

**Difficulty.** `tables_of` writes `PressureK` and `DiginK` from a level and the levers. Hard's AI turns on the press (`on`, `warned`, `second`,
`by_way`, `rescue_crew`, `way_out`, `drilled`, `sweep`) and `digin/`, with `keeper` off and `reach_m` from `HardK`. The levers (`preset_levers`) are
`?fog=`, `?enemyai=`, `?enemykit=`, `?told=`; Hard's fog stands on cones. `HardAtK` (a stun charge each, by structure, through `difficulty_here`) and
`DrillAtK` (rows of `SquadsK`) hold what is one map's own. `training_of` reads the mix off `morale_at`: at standard a band's first man is steady and
the rest green; at hard only the last of three or more is green. A squad's `training` comes first.

**Squads.** `squads_stage` makes them from the level's `SquadData`, or from rooms when a room clear names none (`squad_deal`). After a thinking beat
`decide` picks `Barricade`, `Flank` or `Help` by the situation, with no die. Those who stay dig in (`digs_in`). Hard here is `squad_drill`: longer
beats, so a pair is set while the team goes in (`drill_net`), and a pair sent to look when it goes quiet (`look_out`). Men called to a room of theirs
are there together (`TOGETHER_SECS`), and one alone stays on his door when nobody of theirs is up in it (`tick_due`). `radio_check` finds a drop
nobody saw; where all squads stay, they still miss a silent room (`stay_ask_here`).

**The room fight's other rules.** `consider` and `tick_press` take men to a fight together. `press_keeps_home` keeps a man, `stand_to` is surprise
lost, `last_man` and `tick_out` give ground, `go_look` sends a pair. `dig_in_tick` runs `begin`, `start_brace`, `haul`, `wedge` and `to_cover`, each
heard (`scrape`, `shout`). `step_stand` starts when the last few are up (`few`, a third of those staged at most): `start_check`, `tick_check`, and
each says where he is (`tick_moved`).

**Pursuit.** `tick_plans` and `plan_approach` deal a presser, seconds and flankers on one fix. `mark_danger` makes a doorway somebody fell in dear
(`tactics_penalty`), and `door_gate` makes the next man take it as a drill.

**Lanes.** `cost` opens one when fire drops or passes a man. It goes on the net (`become_known`) and its floor costs more to walk (`mask_of`). Men are
held at its mouth (`held_on`), `back_out` of its line, are sent `round`, set and fire from an angle (`pin_start`), throw (`frag_out`, `stun`), or look
(`peek_start`). `lane_bars` bars other rules. Room fight only, at any difficulty.

**Nerve.** `note_round` adds to a man's suppression, `flinch_round` ducks him, `head_down` pins him and `cover_from` finds him cover. `startle_sight`
gives a beat at first sight, then `choose` draws or, for a green man alone inside `run_m`, runs. Morale is shock plus standing terms (alone, dark) as
dread: `rung_now` gives jumpy and broken, and a blast (`feed`) starts a duck, a scrabble back or a blind burst. `ripe` and `lost` hand a broken man to
`runner_tick`: a shout, the gun dropped (`drop`), a way out (`plan_out`). From there he counts for nothing (`runner_nobody`). `flinch_tick` sends a
green man alone under fire too. `net_quiet_asked`, `net_quiet_body` and `net_quiet_fall` shock the men, once a dead man (`pay`), and worsen the calls;
a whip turns his gun to a noise; the last man (`last_kind`) goes low on `low_spot`.

**The dark.** `dark_door_gate` stops a green man alone short of a dark room (`dark_room`), calling a mate if one is near. In `dark_foe_tick` a man who
knows (`learns`) or has the word (`has_word`) and is blind (`dark_foe_blind`) lights his lamp if he carries one (`carry`), else walks where he can see
(`dark_foe_spot`). `calm_torch_dark` puts a calm torch out where the place is lit. `power_out` starts `decide_out`: their word on the net, then a few
men walk to the lever and back.

**Doors, rounds, grenades, guns.** `door_force_hear` builds sureness of a man behind a leaf; `door_force` fires through it (`round_for`), then
`kick_step` boots it. `round_past_colleague` holds a round whose line passes a mate. `deal_carriers` gives a grenade to a share of them (`def_share`,
`def_max`). A throw has a wind-up, a longer fuse (`def_fuse`) and a HUD line (`sam_cue`), and none goes near a hostage (`hostage_near`). `gun_deal`
deals guns by post at staging.

**Sam with a man in his arm.** In `standoff_alert` men ring and talk. Once the pistol is known `gunhold.rs` has them: `pick_cover`, `plan_round`, peeks,
and `gun_hold_free` opens the clean line at Sam's back once he has fired from behind a man who is no civilian.

**Response team.** A level has one only if its file has a `ResponseData` block (`has_response`). `update_qrf` calls it on a locked-down floor that
heard gunfire or found two bodies, or on a silent net (`silent_net`). `dealt` draws way, size and time; `read_net` keeps what the net says, its word
of the team until it sees him. `QrfState` runs on: `arriving` musters the men, `deploy` throws the door, then `pick_target`, stack and breach or
`start_drill`, and `maybe_perimeter`. There is one wave (`request_qrf`). A block's `raid` brings it on the defenders' side in a room clear.

**At ease.** `needs_tick` fills four kinds of `NeedKind` and sends one man at a time after `announce`; a shot ends it (`cancel_trip`). In
`doze_patrol` a calm staff man with a wait of `min_wait` to `max_wait` sits. `doze_deaf` sleeps him with dull ears and no eyes; noise, light on his
face or a shoved chair wakes him. At a screen (`screen_seat`) `doze_ear` dulls him. `update_smoke_break` is a man whose script says so.

**Civilians.** `update_one` runs `CivState`. `see` and `hear` fill her awareness; `pick_target` chooses a guard, a phone or the way out; then
`to_report`, `to_phone`, `scream`, `to_cower`. `civilian_grabbed` and `civilian_comply` are Sam's hand on her. `spawn_civilians` puts none on a night
breach or a rescue, and `spawn_hostage` makes a rescue's.

## Rules that hold

- **Never harder by hit points.** "harder settings have better trained guys, more proactive, higher morale, better kit". `tables_of` writes behavior
  and kit, and the mix follows the AI lever.
- **One model.** Their knowledge is what reached them. A lit man is seen farther ("yeah raise the range").
- **A man who cannot see goes nowhere.** Dazzled or staggered, he gets down where he is, whatever spot a cover rule names (`alert`).
- **Nobody walks out at a noise he cannot place.** `decide` puts every squad on its doors for it.
- **A lane that cost them.** "enemies do seem to run into held corridors a bit too eagerly, especially when they've already taken fire". "enemies try
  to hold you at some particular angles with fire while their teammates maneuver around you".
- **A gun in a hold.** "they should probably run for cover when you take out a gun and actively work on flanking you". Men fire nothing at a pair
  while the man in the arm is up, unless `gun_hold_free` gives a line.
- **A shut door.** "and if they're alerted they should behave realistically, like shooting through the door or eventually kicking through it".
- **Startle and flinch.** "a quick beat and then they pull out their gun or run". "so very inexperienced characters get freaked out by gunfire and may
  even run away".
- **A man who gives up.** "if a guy gives up he should drop his gun (including pistol) and just run for it no matter what". He leads and answers no
  check after that.
- **The quiet net.** "Dread from unanswered radio checks and bodies found. Heard in the calls getting worse, seen in beams whipping to noises.
  Done-test: the last man never dies standing upright without firing". One dead man is one shock to one man.
- **Cover** stays in rooms his squad stays in, never behind a latched door (`cover_from`).
- **The response team is level design.** "let's make it part of the level design" and "not all levels should have it".
- **Warnings come first.** A wedge, a grenade, a charge and a press each have a sound or a line first (`cues.rs`).

## To change it

- **A number:** the module's file in `rust/game_types/src/knobs/`. `?knob.morale.half_life=40` turns one, `?training=` and `?nerve=` pick rows, and
  `?hard`, `?difficulty=`, `?pressure`, `?digin`, `?fogcone` lay a level.
- **How he patrols, searches or fights:** his state's file under `guards/behave/`.
- **A new sound:** its kind's function in `senses/hearing.rs` and its line in `heard`. Write his mind through `mind`.
- **A rule that takes his body:** a file in `control/` and a line of `rules!` at its rank (`ai-architecture.md`). A rule that tasks him: a line of
  `taskers!`.
- **What Hard is:** a row of `PressureK`, set in `tables_of` and added to what `pressure_same` compares. On one map: `HardAtK` or `DrillAtK`.
- **A level:** `SquadData` (`rooms`, `rally`, `may_leave`, `training`), `way_out`, `ResponseData`. `docs/level-format.md` has the words.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `behaviour.rs`, `senses.rs`, `eyes.rs`, `ears.rs`, `witness.rs` | who sees what by light and smoke; who knows what |
| `attention.rs`, `night_men.rs`, `men_at_rest.rs`, `smoke_break.rs` | calm men: a far glimpse, a missed man, sleep, trips, smoke |
| `darkness.rs`, `dark_rooms.rs` | the search; the dark door, lamps, the breaker |
| `men_under_fire.rs`, `blind.rs`, `cover.rs`, `net_quiet.rs`, `last_men.rs` | nerve, cover, the quiet net, the last few |
| `squads.rs`, `squads_radio.rs`, `come_together.rs`, `hard.rs`, `kept.rs`, `owner.rs` | squads, Hard by address, the tasker list |
| `pursuit.rs`, `held_doorforce.rs`, `held_fragtheirs.rs`, `standoff_held.rs` | two on a fix, a shut door, grenades, a man in Sam's arm |
| `response_team.rs`, `civilians_held.rs` | what calls the team and its one wave; a civilian |

`gate` pins every mission.

## Not seen

The numbers of `morale.rs`, `runner.rs`, `flinch.rs`, `netquiet/`, `darkdoor.rs`, `darkfoe.rs`, `doze.rs` and `powerout.rs` are first numbers, never tuned by play.
Beams whipping, a lit face at a screen, a man dropping his gun and a smoke break are unseen on a GPU.
