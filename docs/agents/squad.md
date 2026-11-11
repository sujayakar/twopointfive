# Squad orders

How Sam orders his teammates and what they do with it. Door orders are `door-tools.md`, fighting in a room is `team-feel.md`, what the men
say is `dialogue.md`.

## What the player gets

- **Two keys.** Q tells the squad to do the obvious thing with whatever the cursor is on. F is the word: it sends whoever is waiting for it.
- **Tap Q** on floor and they move there, on a door and they stack up, on Sam and they fall in, on one teammate and he alone falls in. No menu opens.
- **Hold Q** for the ring at the cursor. It carries every order for that thing and stays up after the key comes up. A row they cannot do says why.
- **Shift + Q** sends part of the squad and leaves the rest on what they were doing. Shift with Q kept down is the ring for that part.
- **Tap Q twice** on one place (`twice_secs`) and every teammate drops what he has and falls in.
- Left alone they walk at Sam's pace, stay out of doorways, never shove him, and keep guns on the ways in.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `orders/` | the book: targets, the menu, giving an order, each man walked and posed. `orders/ring.rs` is the ring's input, `orders/tick.rs` the tick |
| `orders/rows/` | every order: a file, a `Row` |
| `cmdring.rs`, `ringlone.rs` | the ring's arithmetic: which wedge the cursor lights, which row sits on it, a ring with one row |
| `autopilot.rs` | a tap: `auto_pick`, `quick` |
| `planmode.rs` | Shift: `goal_men` says who takes it. There is no plan mode |
| `pairorders.rs`, `pairup.rs` | quiet orders for one partner, and the switch that raises them with `sneak/` |
| `squadthrow/`, `squadsmoke.rs`, `squad40.rs` | frag and stun, smoke, a 40 mm shell on order |
| `shutdoor.rs`, `wandorder.rs`, `workthings.rs` | one man shuts a door, looks under one, works a thing |
| `teampace.rs`, `noshove.rs` | how fast a teammate goes, and giving way to Sam |
| `cover/`, `rearwatch.rs`, `cleared/looked.rs` | angles covered unasked, the way behind a stack, a taken room's other door |
| `squadcast.rs` | `squad_cast`: a second book and door tools, for the response team (`qrf/`), with `every_key` |

The page draws what the tick tells it: `view/rings.rs` and `view/plan.rs` go to `src/ui/cmdring.ts` and `src/ui/planfloor.ts`, `orders/hud.rs` to the
block in `src/ui/overlay.ts`. `src/host/keymap.ts` turns Q into the code the game asks for, `KeyB` (`?bkey` leaves it on B).

## How it works

**A target** (`OrderTarget`) is `Floor`, `Door`, `None` (Sam, or `man`: a teammate) or `Foe`. `target_under_of` reads the cursor's ray: what is drawn
under it wins, and a door counts by the doorway the ray looks through.

**The book** is `SquadOrders`, one a seat, its `orders`. `defs` is the menu: `OrderDef` rows with their `targets`, `page`, words of assent, and
`row`, the `Row` of functions (`can`, `only`, `valid`, `issue`, `tick`, `frame` and more, with the flags in `Is`). `asg` is each man's one
`Task`: `post` is his mark and none means he follows, with `watch`, `door`, `via`, `group` and his status words `going` and `there`.

**The key** (`squad_orders_ring_input`). Down starts `cmd_press` on what the cursor is on. Up within `stay_hold_secs` is a tap
(`squad_orders_tap_give`). Still down, or a digit pressed, and the ring goes up, over that place. A wedge lights by digit or by the cursor going
`flick_px` out. `ring_latch` gives it as the key comes up pointing at one; otherwise the ring stays until a click, the key again or a digit. A right
click turns back from more, or puts it away. So does Tab.

**The ring's rows** (`squad_orders_flat_rows`) are one list: the rows of each tool a listed row opens, plus the squad's posture. `flat_seats` seats them: wedge 1 is what a
tap would give, `flat_top` holds fixed places (floor: cover, fall in, hold; door: cover, open and clear, bang and clear), and the rest fill up behind
a last wedge, more.

**Giving** (`squad_orders_give`) is one path for tap, ring and word: `only` narrows who is asked, `squad_orders_for_men` picks the face of a door,
`can` and `valid` may refuse, `issue` hands each man his `Task` (`squad_orders_assign`, which finds his way), `squad_orders_same_ways` puts men
sent together on one way, then one man assents. A refusal is said by the man it is about, and "nobody's free for that." by the nearest (`say_why`).

**The tick** (`squad_orders_tick`) runs after the door tools, `rear_watch_tick`, `cover_tick` and `pair_orders_tick`. For each man: give way to the
fight if it has him, run the row's `tick`, then the leg, to his mark by `via` or to his place in the file. Then cover, `spotted.rs` and `fireline.rs` may
take his gun or feet, the muzzle drops off Sam, and `order` writes it all. When the last man of a `group` arrives he says "set."

**Who takes it.** Every order of Sam's goes to the whole squad (`squad_orders_sams`: no key picks a pair), unless the cursor is on one teammate or
Shift picks a part. `goal_men` decides: the man under the cursor; everyone, on Sam himself; the men already on that door; with nothing going, half
(`goal_half`: a whole pair if it can, else the nearer men); else the free men; else half of the biggest group. Nobody to spare says so (`GOAL_NONE`).
`roster_of` makes the team: one or two men are one pair, three or four are Red and Blue (`unit.pair`). `squad_orders_men` is where to ask for men. In
a game of two, `deal_seats` deals them out, and each seat has its own men (`in_seat`), book and share (`seat_share`).

**A way** (`squad_orders_route`) goes round doors locked against Sam's keys, rooms nobody has been in (`cleared_rooms_dear_for`, `round_cap`), lanes,
and lit floor an uncleared room can see (`seenwalk.rs`). His steps go round furniture (`prop_way`). A door nobody has tried is walked up to and tried
(`squad_orders_walk_try`).

### The orders

| row | what it does |
|---|---|
| `movethere` | a place and a watch each that fit the room (`fit_spots`), or the line abreast (`squad_orders_spots_at`), dealt for the least walk (`squad_orders_pair`) |
| `stack` | marks on the door (`door-tools.md`), the back men spare for the rear (`squad_orders_spare_rear`) |
| `cover` | stays where he is, aiming at it |
| `fallin` | a place in the file, nearer men first, three or more by pairs (`fall_in_cmp`) |
| `hold` | stops a stride on; in a squad of three or more the men share out the ways in (`squad_orders_hold_arcs`) |
| `go`, `pairgo`, `thrword` | the word for a stack, a mark or a held door. F: `pair_orders_input`, then `door_tools_input` |
| `openclear`, `bangclear`, `charge`, `fragclear`, `bigclear`, `threshold`, `held`, `tools` | the door's tools (`door-tools.md`) |
| `shutdoor` | in the charge's place on an open door: the shortest walk shuts it (`plan`, `why_not_shut`), then back to the file if he came from it (`finish`) |
| `wand` | a shut door, a man with a wand, a fog: he kneels and looks (`pick`, `tick`), then holds by it (`settle`) |
| `work` | in move's place on the breaker or the mission's thing (`work_things_keep`): the nearest works it, another covers him (`work_things_issue`) |
| `throwopen`, `fragthere`, `stunthere` | one plan (`ThrowPlan`): the nearest man with a throw onto the spot, else the shortest walk to one (`walk_spot`). A frag that catches our own is refused |
| `smokethere` | only with cans issued (`squad_smoke_stage`): the nearest man with an arc |
| `shell40` | a man with a launcher up, a round and a shot (`ShellPlan`) turns and fires (`turn_tick`). No rounds, no row |
| `silent`, `loud` | the whole squad, one in the other's place (`silent.rs`) |
| `pairmark`, `pairtake`, `pairwatch`, `pairdark` | below |
| `wallrow`, `walltools` | wall charge (`destruction.md`) |

**One partner.** The pair rows are up only with one teammate and `k.pair.on` (`pair_orders_up`); a quiet mission with a squad raises that through
`pair_up_apply`. "Take him on your mark" sends him round to the man's back to wait for F, "take him now" needs no word, "watch him" holds fire until
the man notices (`pair_holds_round`), "hold in the dark" looks for dark floor (`pair_orders_dark_spot`). A job not done in `take_secs` is called off
by the man himself.

**The answer.** A refused order is said by one man and changes nothing (`squad_orders_why_not`). An accepted one is answered by one man, in turn. A
tap on a door they stand stacked on is answered and moves nobody (`squad_orders_again`). The block shows each man's `going`, then `there`
(`squad_orders_status_of`); a follower's word is latched (`squad_orders_follow_word`). `orders.log` keeps what was ordered.

**How it ends.** Another order replaces it, and a job that finishes hands him a plain hold or the file. A man stopped short of his mark for
`stuck_secs` takes where he stands as his mark. If that is a metre or more off he says "can't get through." and his group calls no "set."
`squad_orders_drop` lets go of a dead man or one a drill takes (`mission::has`). In a fight the AI has him, a posted man pinned to his post
(`duty::pin`); `cold_secs` after he last saw or heard anything, never mid-reload, he walks back by his order's route.

### The file

**His place** is on Sam's trail (`squad_orders_crumb`, `squad_orders_slot`), each man further back. When Sam stops: no place stays in a doorway
(`squad_orders_clear_back`), no two men share a spot (`squad_orders_apart_slot`), a man near enough stays put (`near_m`), and nobody is left in a
frame or a leaf's sweep (`squad_orders_rest_clear`). Sam walking at a man moves him `aside_off` off the line (`squad_orders_aside`). A man far from
his place goes round unknown rooms or by the doors Sam took (`squad_orders_round_rooms`, `squad_orders_trail_way`).

**Pace** (`teampace.rs`). To a mark he goes at Sam's walk, slower if Sam walks after him from behind, faster if Sam is ahead of him and bound for the
same mark (`team_walk_of`). A follower closes at `follow_k` of Sam's walk and runs only while Sam sprints (`team_follow`). At `run_at` his gun goes
low. A run needs a reason (`RunWhy`): a grenade, cover (`team_dash`), following, a mission's order (`team_told`). A man bound for a stack stays under
the run (`door_brisk`). `sneak/` and `unseen.rs` cap all of it while the squad is quiet.

**Sam is not pushed.** A man under fall in yields (`falling_in_yields`): he goes round, stops short or squeezes past (`no_shove_keep`). Sam is put
back only by what his own legs closed on him (`no_shove_back`). `walkahead.rs` is a networked client's guess at its own man: `net.md`.

### Covering without being told

**`cover/`** lists the level's openings (`cover_build`: doors and gaps) and every `deal_secs` deals free men one each (`cover_deal`). A man is free if
his order names nothing to watch and no door, on the walk to any mark and on the mark under `Is::LooksAbout` (`cover_eyes_free`), or if it left him
spare (`cover_spare`). Nearer weighs more, a shut door less, one under Sam's gun or a posted man's little (`sam_k`, `held_k`). A dealt man moves his
gun and not his feet (`cover_pose_for`). One man steps aside to see through an open door nobody has a line into (`cover_step_deal`), and his mark
stays. A man whose walk takes him into a room with floor nobody has looked over is dealt nothing until he is in: he looks where he goes (`walks_in`).
A man with nothing dealt him turns to floor of his room that nobody of the team has looked over (`unlooked_by`), off the record `look_over` keeps:
a mark a cell within 11 m of whoever has it in his eyes.

**`rearwatch.rs`** keeps a stack's back man on the way behind it (`rear_watch_tick`, `rear_ways`). With no door behind, he watches a way in
(`cover_read_rear`). On an entry of three or more he goes in last (`rear_last`). As the squad leaves a taken room one man keeps its other door, then
follows last (`room_keep_tick`, `room_kept`, `room_secs`).

## Rules that hold

- **A tap always sends.** Passed on by the maintainer: "tapping Q just always sends the order".
- **Nothing above 5.** "it's hard to hit numbers bigger than 5 when left hand is on wasd".
- **Shift gives what the key gives.** "i think shift Q on a point should just move the squad there, should match what regular Q does. you should need
  to open the radial menu to get covering a point".
- **Shift splits.** "shift Q on a second door will naturally split the squad in two, and F tells all of them to go in", and the note the maintainer
  passed on: "first shift Q should just send half the squad, whoever's closest".
- **Two taps.** "double tap Q -> cancel everything and fall in".
- **They think for themselves.** "we expect squad mates to be very smart on their own and not need micromanaging like this". "men are walking past the
  breach looking away from it" is why any walk to a mark is dealt cover. And: "weird that these guys are covering a wall".
- **A teammate has what Sam has.** "should be symmetric between self and teammates (e.g. 40mm should work for squad)".
- **Sam sets the pace.** "maybe we do something where if you're walking with the squad but behind them, they slow down to let you catch up (and match
  your speed when you're ahead of them?)". And to a door: "weird really fast squad movement / sprinting that didn't make sense".
- **An enemy the picture hides is no target and gives nothing away** (`pair_orders_is_foe`, `in_the_way`).

## To change it

- **A number:** `rust/game_types/src/knobs/`; `?knob.cmdring.twice_secs=0.6` turns one. Words: `?qtwice=0`, `?goalhalf=0`, `?doorbrisk=0`,
  `?pacing=0`. `?squad=4` is the team's size.
- **A new order:** a file in `orders/rows/` with `pub const ORDER: Row`, a line in `rows!` in its `mod.rs`, and its row on the menu: `rows::def` in
  `squad_orders_new`, or `list` from its `frame`. What other rules need to know of it goes in `Is`.
- **A place on the ring:** `flat_top` and `flat_left` in `cmdring.rs`.
- **What a tap means:** `auto_pick`. The HUD's word reads the same pick (`auto_hint`).
- **A man's mark:** `squad_orders_assign`, never his script, or the route and the status are left behind.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `order_keys.rs` | move, hold, cover, fall in, a stacked door, a man back from the fight |
| `orders.rs`, `menu.rs`, `refused.rs` | every row's order and words; one row however often listed; what they refuse |
| `goal_keys.rs`, `shift.rs`, `two_seats.rs` | Shift on every target, the part's ring, two taps, the second seat |
| `rings.rs`, `squad.rs` | what a seat is told of its ring, plan and block |
| `partner.rs`, `partner_sneak.rs`, `pair.rs` | the partner's orders; two pairs, one word |
| `throw.rs`, `throw_walk.rs`, `throws.rs`, `squad_smoke.rs` | the throw shown is the one given, a walked throw, a shell, smoke |
| `rooms_held.rs` | cover keeps its angle, an order under fire stands, shutting a door |
| `held_team.rs`, `men_by_sam.rs` | the team's pace, going round Sam and a chair |
| `corners.rs` | a squad that follows looks over the whole room |
| `held_cover.rs`, `passing.rs`, `taken.rs` | openings dealt, cover on the walk, an order taken away |

`squadkeys-check.ts` (a `tools/qa/checks.ts` row) holds the controls list to the keys.

## Not seen

How the ring, the file and the plan on the floor look and feel has been checked by numbers only,. `who_key`, and a menu with no ring, still stand in
`squad_orders_input`; no key on the page reaches them.
