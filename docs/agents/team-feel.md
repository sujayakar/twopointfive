# Team feel

How teammates fight, look around and keep quiet without being told. What each order does, the orders menu and how fast a teammate walks are
`squad.md`; working a door is `door-tools.md`; what a defender does in the same fight is `enemies.md`; the fog and Sam's own stealth are `stealth.md`.

## What the player gets

- **A teammate fights by what he sees.** He makes a man out, brings his gun round, settles and fires a pair. He calls contacts, kills, reloads and a
  man down.
- **He looks where it matters.** A shot, a mate's call, a footfall or a door coming open turns him. Told to cover, he keeps his angle.
- **He never shoots through Sam.** His gun comes down, he waves, says so, and steps aside.
- **He checks his corners.** Through a door, his gun goes to the near corner, then the deep one, then over the floor nobody has looked at yet.
- **He keeps out of rooms nobody has been in** when there is another way.
- **The team goes dark.** With Sam's goggles down or nobody found, they use goggles and no light. Compromised, each man's light is his own to use.
- **Nobody fires first.** While nobody has found the squad, no teammate starts the fight until Sam shoots or gives an order that means it.
- **Silent is an order.** It means walking quietly, no light, easing doors, picking locks and holding fire on men who have not noticed.
- **When Sam sneaks, they sneak.** They go low when he does and keep his pace. A knee his work puts him on (the wand, a lock, a body) is not that: they
  keep what they were doing. On a quiet mission his partner reads the light and the defenders' eyes before each step.

## Where the code is

In `rust/game_rules/src/`, with types and numbers of the same name in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `roomfight/` | the fight: `operative.rs` is a teammate, `fight.rs` the tick and what he hears, `line.rs` who stands in a round's way, `rooms.rs` its rooms |
| `cleared/` | the level's rooms, flooded from each door, and which the team has stood in |
| `roomread.rs` | the space round a point and its ways out, for `roomfit.rs` |
| `seenwalk.rs` | what lit floor in view of an unknown room costs a walk |
| `coverfire.rs` | rounds into breakable cover at a man known to be behind it |
| `nvg.rs`, `irlight.rs` | whether the team is dark and each man's goggles; the infrared flood on the goggles |
| `irlaser.rs` | Sam's infrared laser |
| `silent.rs` | the silent posture, and `round_held`, the one question every fire hold goes through |
| `holdfire.rs` | nobody fires first, whatever the posture |
| `unseen.rs`, `sneak/` | the team sneaking because Sam is; the partner's careful follow |
| `spotted.rs` | what a silent man does when a defender is onto him |
| `pairup.rs` | the switch that turns on `pairorders.rs`, `sneak/` and `spotted.rs` together |
| `showwhy.rs` | waves, fists and lines that say why a teammate is waiting |

Beside these: weapon lights are `team_lit`, `light_ruled`, `lit_place` and `wall_dark` in `rust/game_rules/src/people/guards/search.rs`, numbers in `LightsK`.
Corners and the floor search are in `rust/game_rules/src/entries/doortools/threats.rs`, `flow.rs` and `flow_in.rs`. Openings dealt to stopped men are `cover/`, the team's pace
`teampace.rs` (`squad.md`).

The page draws: `rust/game_draw/src/lights.rs` hands over the infrared lights, `src/render/lights.ts` caps them.

## How it works

**Order in the tick** (`STEPS`): `silent_tick`, `spotted_tick`, `squad_orders_tick`, `room_fight_tick`, `show_why_tick`, and later
`nvg_tick`, `coverfire_tick`, `ir_light_tick`. The posture stands before the orders pose the men.

**The fight runs on top of the order.** `room_fight_tick` gives each teammate an `Op` and calls `room_fight_tick_op`. An order owns his legs and the
default for his gun. The fight lays `over` on his script (`GuardScriptOver`): `room_fight_point` turns the gun and `room_fight_restore` takes it off,
and nothing the order holds is written. A side step is the fight's own goal (`own_step_back`); a new order wins over it.

**His frame** (`tick_op_in_view`):

1. `his_eyes`: every hostile through `can_see_char` and `room_fight_rate_on`, into a `Meter` a man. He may take a man with a real sighting now, once
   the meter passes `notice` or the floor is hot. `pair_passes`, `work_passes` and `room_fight_cover_passes` can each say the man is not his.
2. `muzzle_to`: a new man sets when he is ready. Inside `sector_deg` of his gun that is `react` × `ready`, outside it `react` + `surprise`, from
   another man `shift`.
3. Nobody: `nobody_in_sight` asks `room_fight_cue` for a look, then `cover_op`, then takes the overlay off.
4. A man: `engage` turns the gun, `tracked` asks whether a round may go, `fires` sends it.

**Hot** means loud reached his ears. Gunfire, a bang or a kick stamps `hot_t` in `room_fight_tick`, and `room_fight_hot_for` holds for `team_hot`.
While hot, a sighting alone is a man. The fire holds count it as found.

**What turns him**, the first that speaks (`room_fight_cue`): a hostile's round from where he has a line, a mate's target he can see, a footfall
(`hear_step` gives it to the best-placed man), a door an enemy opened in his sight (`hear_door`), rounds through an open doorway
(`room_fight_door_shot`). A held arc limits all but the first to `arc_half_deg` (`in_arc`). A look only brings the gun round: no round goes at a
noise.

**The gates on a round** (`tracked`, `fires`): on the man; not at a run (`team_sprinting`); settled (`acquire_min`); gun not in a wall; `round_held`;
a clear shot at chest or head, or a man behind breakable cover (`cover_seen`); `silent_holds_glass`; no friend in the line or the cone it could go
(`room_fight_friend_in_line`, `friend_in_cone`). Its spread is the gun's `ai_spread` × `spread_k` × how settled, plus suppression and his feet.

**When he cannot shoot.** A friend in the line: `room_fight_side_step` once he has stood `line` seconds, and `room_fight_let_go` drops the masked man
after `let_go` for someone shooting from elsewhere. No shot and a walk to finish: `room_fight_walk_on`, saying "no shot — moving.". Stopped in a
doorway: `room_fight_doorway`. Dark on a lit man, Sam in the line: `room_fight_hold_short` stops his feet. Dry: `gunslots.rs` draws the
sidearm on a close man, else `begin_reload` and "reloading.".

**Holding fire.** `round_held` is `silent_holds_round`, `pair_holds_round` or `spotted_holds_round`. Off the silent posture, `silent_holds_round` asks
`unseen_holds`, then `hold_fire_holds`. The silent, unseen and hold-fire answers free the round at a man alert, searching or past `noticed`, and when
a captor is at his last warning (`urgent`). `hold_fire_lifted` holds for `lift_secs` after a player's round (`hold_fire_shot`) or an order that means
it (`hold_fire_order`), and while a door order or rescue clock runs (`working`).

**Dark or free** (`nvg_tick`), the first that speaks: Sam's light on, free; Sam's goggles down, dark; else dark after `quiet_secs` with nothing loud
and no alarm (`loud_t`). `read_place` reads his place twice a second. Goggles come down (`nvg_down`) where it is dim and his light is off. A dark
man's light stays off, unless nobody leads and the tube shows nothing (`nvg_black`).

**The weapon light** (`pose_flashlight`): the man's card (`unit.light`), `team_lit`, and neither `nvg_dark` nor `silent_dark`. `team_lit` wants it
with a target or a look, just after loud, searching a room, after a footfall, or more than `on_m` from his mark or `alone_m` from Sam. `lit_place`,
`gun_on_lit_man` and `wall_dark` put it off.

**Through goggles.** `sight_gain_on` takes the most of the plain reading, `lamp_glare` and `nvg_sight`, which is the tube's curve on the man's light
plus `ir_on_man`. The flood (`ir_light_tick`) burns only where `burn` finds floor too dark for the tube (`dark_at`, `fan_dark`), and dims against a
near wall (`near_k`).

**Silent** (`squad_silent`, `under_silence`): `k.silent.pace`, `silent_step`, `silent_hush`, `silent_ease`, `silent_dark`, `silent_pick` before a
boot, `silent_no_cans`. Loud reaching a man (`reached`) frees his rounds and the word says compromised, but the squad stays silent until told or until
a loud tool is picked (`silent_tool`). A door no pick opens is booted only then (`silent_boot`).

**Unseen** (`unseen_live`) needs no order: Sam low, on a wall or slow (`sam_sneaks`) and nobody found (`squad_found`). It gives `unseen_low`,
`unseen_follow`, `unseen_step`, `unseen_holds` and a place along Sam's wall (`unseen_wall_slot`), to all but men working a door (`entering`).
Whether he is low by his own key is `sam_low`; `kneel` keeps on him (`knee`) how he stood before his work, until he is up again.

**The partner** is on only under `k.pairup.on`, which a quiet mission with a squad sets at its start (`pair_up_apply`; `quiet.rs` keeps the old switches
in `Held`), with `k.silent.start`. `sneak_follow` walks Sam's own trail (`track`) and reads each step by the defenders' arithmetic (`exposure`,
`lit_seen`, `eye_gains`, `cross_cost`). `spotted_tick` sets `SpottedMode`: `Wary` holds still or slips out of the line (`hide`), `Caught` shoots on
sight and finds a shot (`spotted_keen`, `cover`), and at arm's length a roused man is taken by hand (`hands`).

**Corners and floor.** `door_tools_corner_step` works through two points from `door_tools_corners_of`. One counts once the gun has been within
`corner_rad` for `corner_secs`, or after `corner_cap` a point (time in a fight not counted), or with him a meter from it. Then `door_tools_search_of`
makes a `RoomSearch` of every floor cell: `door_tools_look` marks what a gun has crossed with a line or is close by him, `door_tools_scan` turns the
gun to the unseen cell of least turn, `door_tools_send_on` sends a man to floor nobody can see. A room itself flips in `room_clear_scenario_tick`,
when its own men are down.

**Known rooms.** `cleared_rooms_mark` runs off everyone's feet. `cleared_rooms_dear_for` prices the cells of unknown rooms for a route, bar his own
and his mark's, and `seen_way` in `rust/game_rules/src/squad_orders/orders/routes.rs` adds `seen_price`.

**Cover fire** (`cover_op`) is for a man with nobody in sight: `choose` picks a place and a piece, and `cover_lane` and `round_held` can hold it. A
piece is chewed only where cover breaks (`wall_charge_on`).

**Why** (`show_why_tick`): masked by Sam, a wave and "you're in my line." (`masked`, one voice: `line_said_by`); a stack waiting, squeezes up the file
then the lead's fist (`stacks`); "after you."; "I'm hit."; "I can't keep up."; "I've got this door.".

## Rules that hold

- **Nobody fires first.** "while nobody has found the squad, nobody fires until you do or you order it, whatever your posture". The order to go loud
  lifts it only for a squad that was silent (`hold_fire_order`).
- **Night vision follows the lead or the secret.** "the squad members should have night vision and use it accordingly. this could either be following
  your lead or knowing when they haven't been compromised and flashlight would ruin that". A card can leave a man without goggles (`kit_goggles`).
- **Enemies have no goggles.** "we can assume enemies never have nvg". `nvg_worn` asks for a teammate.
- **The flood is for what is too dark.** "is it realistic to only have them turn on when it's actually too dark to see?" and "ideally it's just enough
  to be able to see".
- **No laser line on a teammate's gun.** "why do we have the lasers for the guys? i think we can disable". Sam's shows only through a tube, gun up
  (`sam_laser_on`).
- **No light on a wall.** "weird that they are shining the flashlight on the wall immediately in front of them" (`wall_dark`).
- **Silent, and sneaking.** "some way of telling the squad to be silent or not. silent implies walking quietly, no flashlight, etc." and "if we're not
  compromised and i'm sneaking around to get into a better position, my teammates should do the same"
- **A man told to cover keeps his angle.** "doesn't watch his angle even though red team has it covered". He passes over a man outside
  `cover_half_deg` while a mate has a round at him. Inside `cover_near_m`, or with a muzzle on him, the man is his.
- **He checks his corners.** "teammates clearing rooms seem to miss enemies. they need to check their corners"
- **Rounds through a doorway turn the men who can see it.** "enemy was immediately in front of them and shooting, nothing happened"
- **Routes keep to known rooms.** "they'll route to the room through a different door … even worse they'll go through uncleared rooms to get there"
- **Lit is lit to somebody.** "not following me here". A dim step is seen only where that defender's eye would gain on a man there (`lit_seen`).
- **Fear is for defenders.** `morale.rs` asks only of defenders and night staff (`ours`). A teammate's nerve is suppression and being hurt:
  `condition_sup` widens his rounds, `condition_pace` caps his legs.

## To change it

- **A number:** the table in `rust/game_types/src/knobs/` (`roomfight.rs`, `nvg.rs`, `irlight.rs`, `silent.rs`, `sneak.rs`, `unseen.rs`,
  `seenwalk.rs`, `spotted.rs`, `coverfire.rs`, `showwhy.rs`, `LightsK` in `guards.rs`). A panel or `?knob.roomfight.team_hot=20` turns it.
  `?seenrate=0.1` is `sneak.seen_rate`.
- **How quick or wide he is:** the `Operative` row of `RoomFight::blank`, in `rust/game_types/src/roomfight/types.rs`. Its `Hostile` row is the
  defenders'.
- **Who he takes, what turns him:** the score in `his_eyes`; `room_fight_cue`, in order, with new sounds read off `g.events` in `room_fight_tick`.
- **A new hold on a round:** an `||` in `round_held`, so that the fight and the cover fire ask it alike, and an answer in `masked`, which must not
  show a line for a round held anyway.
- **When his light burns:** the `want` list in `team_lit`. The goggles follow by themselves.
- **A word or a wave:** a section of `show_why_tick`, or `room_fight_speak`. Player text follows `writing.md`.
- **On for a mission:** the quiet mission's start in `rust/game_rules/src/missions/quiet.rs`. A new part is a field of `Held`.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `team_lights.rs` | a light off on a lit man, dark until compromised, following Sam's lead |
| `tube.rs` | no half-dark place blinds a goggled man |
| `silent_squad.rs` | picks the lock, holds fire until noticed, ends only by a loud tool |
| `partner_sneak.rs` | low when Sam is, out of a door's line, off a walker's way, taken by hand, which rounds are held |
| `pair.rs` | the squad line's words, a blown job, a way across his eyes, a man's line |
| `rooms_held.rs` | a room flips when its men are down; a man told to cover keeps his angle |
| `fight_doors.rs` | rounds through a doorway keep the team's guns on it |

`gate`'s pins move with the fight. `tools/qa/sweep-ir-check.ts` holds the page's cap on infrared lights.

## Not seen

How a teammate's look or side step reads in play, and the wave and the fist, have been checked by numbers only. A goggled team's flood has not been
judged on a GPU. No native test names `holdfire.rs`, `unseen.rs` or the corners' measure.
