# Stealth and what you see

Seeing, being seen and working in the dark: the fog of war, the light and sound meters, lights as things, Sam's close work, the wall hold, the whistle
and the tools that look where he is not. What a defender senses is `enemies.md`; how a teammate sneaks is `team-feel.md`; the poses are `anim.md`; the
fog's kernels are `render.md`; the end card is `menus.md`.

## What the player gets

- A floor he cannot see all of. Enemies and what they do show only where the team has a line of sight. The building is always shown.
- Two meters: how lit Sam is, the same number every enemy eye and camera reads, and how much noise he is making.
- Lights he can use: shoot a lamp or a car's lamp out, flip a room's switch, throw or shoot the breaker, jam a lamp or a flashlight.
- His own hands. Hold the left button on a man's back and Sam creeps up and grabs him, then questions him, chokes him out or puts the pistol to his
  head. The right click knocks a man out from any side. Bodies can be dragged and carried.
- A wall hold taken by walking into a wall, with a peek round its end. A low whistle that brings one calm man. Cameras he sticks to walls, and a wand
  under a shut door.
- Enemy cameras, laser beams and keypads. A stealth score on the end card.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `fogofwar/`, `peeksight/` | who is an eye, who is seen, what is kept as last seen (`picture.rs`: what the renderer is told); the eye moved by a peek |
| `quietfog/`, `told/` | the fog lever's rungs on a quiet mission; how much the player is told |
| `sense/`, `sim_lightshome/`, `lighting/` | smoke; fixtures and the mains' columns; carried lights, the blackout, the jammer |
| `lightswitch/`, `carlamps/`, `powerout/` | wall switches; a vehicle's lamps; a garrison's answer to the dark (`enemies.md`) |
| `interact/`, `handring/` | one key and a ring over the rows `build_interactables` lists |
| `close/`, `stalk/`, `takedown/`, `disarm/` | what Sam can do to a man from where he stands; the creep; blows and falls; the gun out of his hand |
| `hands/`, `handwork/` | a body hauled, a door by hand; hands on a keypad or a lever |
| `wallsnap/`, `samlook/`, `stealthfeed/` | the wall hold's numbers; Sam's head and his room round a man; the word to his animator |
| `whistle/`, `attention/` | the whistle and who answers; a calm man's one attention |
| `stickycam/`, `wand/`, `wandorder/` | stuck cameras; Sam's wand; a teammate's on order |
| `devices/`, `chronicle/` | cameras, beams, keypads; the run's record, score and badges |
| `unseen/`, `sneak/` | the team going low because Sam is (`team-feel.md`) |

The hot loops are `rust/sim/src/fog.rs` (eyes against men), `rust/sim/src/light.rs` (lamps on a point) and `rust/sim/src/lights.rs` (fixtures, the
mains, the machines of cameras and beams). The hold and the wall are in `player/`, the meters in `game/p_07.rs`.

The page's half: `rust/game_draw/src/fog.rs` fills the fog's section, `src/draw/fog.ts` reads it into `src/render/fogofwar.ts` (kernels
`fogofwar.wgsl`, `fogsight.wgsl`). `src/ui/overlay.ts` draws `marks_view` (meters, marks, lenses) and `deriveInteractions`. `src/host/keymap.ts` gives
the keys.

## How it works

**The fog's one question.** The rule the maintainer agreed: "The fog hides other people. It never hides the place, and it never hides your own team."
A new effect asks whose it is. The place is live: walls, weather, a lamp's glow, smoke. What the team does always shows (`fog_of_war_ours_id`,
`fog_of_war_doing`). Everybody else stands as the team last saw him.

**The pass.** `fog_of_war_pass`, late in the tick, while `fog_of_war_active`. The eyes, in order: each seat's man (or his wand's lens or peek), living
teammates not blind, live stuck cameras, then a tutorial's eye (`rails_eye`, `rails_step_eye`), up to `fog_eye_count` and never past `FOG_EYES`. Under
`k.fog.own` a teammate counts only by an ordered lens. Each man off the team is asked of them all (`fog_of_war_sees`, smoke by `smoke_between`) and
`judge` writes his `Rec`.

**A man is drawn** (`fog_of_war_drawn`) if seen, seen within `k.fog.linger`, fired on by a teammate (`fog_of_war_fired_on`), shown by a script, or a
body the team has seen, which stays for good. Otherwise he is in `hidden_chars`, and what he breathes or fires files no air.

**Things are kept.** A door's leaf stands at `door_recs` until an eye or a teammate beside it has it (`fog_of_war_door_seen`). Loose things keep their
place (`fog_of_war_things`), lamps their state and face (`fog_of_war_lamp_known`, `fog_of_war_lamp_face`). Another man's flash shows only where its
spot is seen (`fog_of_war_cell_seen_at`).

**Two records.** `g.fog_of_war` is the game's and every machine holds the same. `g.fog_picture` is one machine's, and no rule reads it.

**Who plays under it.** `fog_of_link`: a room clear, a rescue, `?squad4` or `?fow`, a quiet mission whose rung is not off; never `?nofow`. `QuietRung`
is `Off`, `Team` (the default, `k.quietfog.def`), `Cones` or `Own`: `quiet_fog_start` lays it on `k.fog.cone` and `k.fog.own` for the visit,
`?quietfog=` picks it. The cone also comes with Hard (`set_difficulty`, `?fogcone`): a wedge of `cone_deg` about `gaze_of`, and `cone_near_m` all
round. Then teammates call what the player has not got (`fog_of_war_tell_calls`). A quiet mission under the fog hands out `k.sticky.issue` cameras
(`?cams=`).

**The light meter.** `light_at` sums lamps on six faces of a point with one shadow line each, then sun, sky and lightning. `light_on` takes the
brighter of chest and head. `game_meter_read` maps it by `game_visibility_of` and eases it into `visibility`, which guards' sight, `lens_gain` and
`close_near_k` read.

**The sound meter** is the player's `noise`: the loudest level he has just made, falling away. A noisy rule raises it where it files its event. Only a
lesson's step reads it; men hear the events.

**Lights.** A fixture is a row of `sim_lightshome` (`FixtureDef`) and every change goes in by `light_home_in`. `shoot_out` breaks one, if `lamp_live`.
A round in the breaker ends the power for good; `breaker_start` and `breaker_tick` throw it by hand, and it can be reset. `set_blackout` sends the
nearest calm man, unless `power_out_rules` gives the walk to a garrison's party. The jammer is the code's OCP (`ocp`): the right button with the
pistol up, on what the reticle has (`ocpable`). A flashlight dies, a fixture or device goes dark for `ocp_duration`, a breaker in sight trips.

**Wall switches** are in no level file. `build` finds plates on a room's wall by each door. `flip_switch` works one; a calm defender at it puts the
room back (`light_switch_tick`, `puts_back`). **Car lamps** are found from the boxes (`find_car_lamps`); one already dark breaks and tells nobody
(`dead_lamp`).

**One key and a ring.** `build_interactables` makes the rows (`Interactable`, each with a `Verb`). `update_interact` reads the key: a tap does the
highlighted row; kept down for `k.handring.hold_secs` it puts up a ring, with the whistle as its last wedge (`whistle_wedge`). What is done is `g.act`
for that frame, read by the rule that owns it.

**Close in.** `close_at` is the one test for rows, clicks and the ready pose. `close_windows` gives a man a rear arc, sides and a front, each with a
reach. Only the rear grabs, and not an alert man; every window strikes. `close_rmb` reads the right button: a tap, or kept down into the choke.
`start_takedown` begins the blow, `take_fall` drops him, `close_noise` is its sound.

**The stalk.** `stalk_step` takes Sam's feet while the left button stays down on a man whose back he is behind: toward `station_m` behind him at the
man's speed and a little. `out_of` ends it; `stalk_grab` hands on the grab. On a wall the held button is a wait (`corner_gate`, `corner_man`) and
takes the man who comes past. Off a wall the `Grab` row ends it as well; on one, where the wait ends
by itself, the button is the grab (`stalk_click_only`).

**The hold.** `update_hold` runs `HoldPhase`: the gun comes out first (`disarm_begin`, `disarm_tick`), then `Held`, where `ask_held` and `answer_held`
question him, `Choke` runs to `choke_out`, and `swap_hold` brings the pistol up for `Whip` or a round past him. `free_held` ends it.

**Bodies.** `toggle_drag`, `haul_begin`: lifted where there is room at the hips, otherwise `drag_haul` along the floor. A sprint drops him.

**The wall.** `find_wall` picks a face, `wall_snap` takes it, `move_on_wall` slides, peeks at an open end and lets go when held away. `wall_press`
takes it from a move key held into a face (`press_secs`, longer by anything to use). A peek moves the fog's eye (`peek_eye`).

**The whistle.** `try_whistle` files one event; `resolve` works out once who heard it along the sound's path (`hear_path`, `reach_k`) and that the
nearest who `can_answer` comes. A second inside `twice_window` is a search; a man already hunting takes it as a fix (`hear_whistle`). **Attention**
(`attention_tick`, `able`): a small thing holds a calm man's eye for `secs_for` and dulls what is off it (`attention_side_k`, `attention_ear_k`);
`attention_view` gives the HUD its thread.

**Stuck cameras** ride the rifle's tube (`fire_cam`). `settle` sticks one or drops it to the floor. `sticky_live` is the newest `k.sticky.live`, which
send (`sticky_eyes`); `take_cam` takes one back.

**The wand** is kit (`wand_offered`: carried, under the fog, a door `wand_fits`). `start_wand`, then `wand_through`; `wand_eye` is a wedge past the
leaf that the cursor turns (`wand_turn`). A teammate looks only by order (`wand_order_key`), his lens sweeping by itself (`mate_wand_eye`).

**Devices** are level data (`docs/level-format.md`). `update_camera` tests a lens by `eye_contact` and `lens_gain`, then `camera_suspect` (a man sent)
and `camera_alarm` (the floor up). `update_laser` breaks on a stranger's body or a flying item (`body_breaks`, `laser_trip`). A round, the jammer or
the mains stop them (`alive`). `sensing` is off under a night breach and a rescue. Keypads: `pad_state`; codes come by `learn_code` and `overhear`.

**The record.** Each `chron_*` adds a row. `stealth_score` takes `k.score`'s prices off a hundred, `rating_of` words it, `badges_of` adds badges,
`debrief_of` hands them to the card. `told_now` sets how long a last-seen mark stays (`fog_of_war_last_seen`).

## Rules that hold

- **Enemies never read the fog.** A sensor under it asks the floor and not the picture (`update_laser`).
- **To see more, get more eyes on it.** A camera, a wand and a peek are each a `FogEye`.
- **The squad's doing is seen.** "in fog of war mode we should still see enemies when your squad is actually shooting at them. otherwise it feels a
  bit weird to see them shooting at nothing".
- **What moves unseen stays where it was.** "seeing the furniture move in rooms behind fog of war also gives away information".
- **A teammate's eye passes what the fog's lines pass.** "teammates should see whatever we see". One mask: `fog_through`. Every other side's eye stops
  at thin things (`eye_through`).
- **Solo nights have the fog.** "for the solo night missions we should have fog of war! then you really need the sticky cams".
- **Nothing is done to a man the team cannot see** (`close_subject`).
- **Sam's hands knock out; they do not kill.** "remove the knife, only rmb for non lethal CQC when nothing equipped".
- **Behind a man, the button says how.** "make it possible to just right click to knock him out quickly but it's a bit louder, or holding right click
  does a choke." The tap is `rmb_loud` louder.
- **The stalk is the game's walk.** "the game also controls the trajectory and speed and makes things smooth. it ends with you grabbing him".
- **A held man has no gun.** "held enemies should drop their gun".
- **The wall has no key on the page.** "snapping feels good, we can remove Q". The rules still read `KeyQ`, and a player may bind it.
- **Attention must read.** "we can try to play with some mechanics where guards' attention could be redirected elsewhere (e.g. apollo robbins
  psychology stuff) but we should make sure it's legible and fun". It stands off a live squad fight except on a quiet mission (`able`).
- **Scores only take away.** Nothing in `k.score` adds; a shot lamp costs a capped little.

## To change it

- **A number:** `rust/game_types/src/knobs/` (`fogofwar.rs`, `close.rs`, `stalk.rs`, `wallsnap.rs`, `whistle.rs`, `attention.rs`, `stickycam.rs`,
  `wand.rs`, `chronicle.rs`). `?knob.fog.linger=1` turns one. Words: `?wallpress=0`, `?nowallsnap`, `?noleangun`.
- **A new effect under the fog:** ask whose it is. The team's: `fog_of_war_doing`. Another man's: `fog_of_war_air_hides` at the source, or a place
  kept like `thing_recs`. The place's: nothing to do. No fog exception of its own, and no rule reads `g.fog_picture`.
- **A new eye:** a `FogEye` in `fog_of_war_pass`, under the cap. More room means `FOG_EYES`, `MAX_EYES`, `rust/game_draw/src/fog.rs` and the kernels.
- **A new row for Sam:** a `Verb`, a row in `build_interactables` or a `*_rows` call beside it, and the owner reads `g.act`.
- **A new device:** a `CameraDef2` or `LaserDef2` row in the level; a new kind goes in `devices/` and `rust/sim/src/lights.rs`, drawn by
  `device_boxes`.
- **A price:** `ScoreK`; a new count is a `chron_*` and a `RunCounts` field.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `fog_eyes.rs`, `fog.rs` | whose eyes show whom, the cone, what is kept; the game's record against the picture's |
| `hud_marks.rs`, `hud.rs` | what the HUD is told: marks, last sightings, meters, rows |
| `senses.rs`, `eyes.rs` | smoke and the light on a point; a man's level look |
| `the_grab.rs`, `held_close.rs`, `held_takedown.rs` | where a grab is offered, the choke, how a hold ends, the gun; the right click; the falls |
| `held_wallhold.rs`, `held_lights.rs` | taking and sliding a wall; a gun light at a wall; a room's switch |
| `whistles.rs`, `attention.rs` | who hears and who comes; a held eye and its thread |
| `lenses.rs`, `lenses_mate.rs` | stuck cameras and the wand, Sam's and a teammate's |
| `yard_devices.rs`, `mains_and_falling_in.rs`, `dark_rooms.rs` | beams, cameras and the mains; who walks to the breaker |
| `quiet_mission.rs`, `held_aim.rs`, `debrief.rs` | a quiet mission's end; a lamp needs a real hit; the card |

`gate`'s rows run the whole tick, the fog with it (`qa.md`).

## Not seen

- The fog's picture on a GPU: the levels of `dim`, `lit`, `flat` and `edge`, and how a never-seen room should look, are the maintainer's to judge.
- Whether `attention/` and `powerout/` read well in play has been checked by numbers only.
- The knife is dead code: `close_bare` is true outside a show, so `start_kill`, `kill_out` and the lethal blow in `game/p_07.rs` cannot happen in
  play.
