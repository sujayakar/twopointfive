# What breaks

Rounds and blasts on walls, cover, furniture, glass and fixtures: what goes, what is left, who is caught. A door's leaf is `doors.md`; what a thing is
made of is `levels.md` (`MATERIALS`); the fog's dust (`air_dust.rs`) is `weather.md`; how it is drawn is `render.md`; the door charge and the stun can
on order are `door-tools.md`; the row that orders a wall charge is `squad.md`; a man running from a grenade is `enemies.md`.

## What the player gets

- A plasterboard wall gives under rifle fire, a column at a time. It shows wear first, and the hole is real to feet, eyes and rounds alike.
- A rifle round goes on through thin things with what it has left. Low cover gets chewed away, and men know it.
- A wall charge opens a hole a man fits through, in a partition or a building's outer wall, never in steel or concrete. It kills whoever stands at it,
  blinds whoever is close, and shocks the room behind it, and for a few seconds nobody sees through its smoke.
- A frag or a 40 mm round breaks what is around it and raises the same cloud. A stun can tears a leaf and drops tiles, and opens no wall.
- Glass breaks as its kind does: float glass runs and drops daggers, toughened glass goes all at once, a laminated sheet holds. A bar goes with one
  round. What falls is heard, and broken glass underfoot makes a footstep louder.
- Furniture is thrown, comes apart or shatters by how near it stood. A restart puts it all back; a load gives it as it stood.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `destruction/` | the way in: `destruction_blast`, `destruction_round`, `destruction_bang`; chaos, heat, papers, clutter, tiles, fixtures, the pall |
| `destruction_switch/` | the numbers of kicked rubble (`RubbleK`) and a piece's landing (`DebrisK`) |
| `wallbreak/` | the wall engine: what can break, its cells, what stands afterward, a blast's plan, the change list |
| `wallcharge/` | the wall charge: where it may go, Sam's and the squad's, the safety radius, the blow |
| `wallpen.rs`, `coverfire.rs` | a round through thin things; men who fire through what hides a known enemy |
| `wallreact.rs` | what men make of a hole or of lost cover |
| `wallwear.rs`, `wallskin.rs`, `edge_look.rs` | a wall's look under fire; what is painted on it; ragged edges, soot and litter |
| `break_table/`, `debris/`, `falling.rs` | what a broken thing throws and its shapes (`Mat`, `break_row`); the pool of pieces; where a thing comes down |
| `furnblast.rs` | furniture and a blast |
| `frag/`, `he40.rs` | a burst and whom it takes (`blast`); the 40 mm's flight, arming and dud |
| `glass.rs`, `glassbreak.rs`, `glasspen.rs`, `pattern.rs` | a blow on a pane; its breaking; a round through it; the cracks |
| `shards/`, `winbars.rs` | what a falling piece of glass becomes; a window's bars |
| `fixture_break.rs`, `blast_look.rs`, `blast_feel/`, `dust_look.rs`, `floormess.rs` | for the picture: a lamp's tray; the charge's flash; report and kick; dust tints; floor films |
| `marks.rs`, `render_decals/` | who asks for a mark, the list of marks |
| `scene_cut.rs` | planes that shave a box, for a ragged end or a torn piece |
| `scene_materials.rs`, `scene_stuff.rs` | `does_of` and `BoxStuff`, the two words a box says |

The game's block keeps, in `rust/sim/src/`: `panes.rs`, `glass.rs`, `pattern.rs`, `pen.rs` for glass and bars.
A leaf in the air is on its door, rules in `slab.rs`. Landings and litter are `g.falls`, rules in
`rust/sim/src/falls.rs`. The page's half: `src/render/decals.ts`, `src/render/particles.ts`, `src/draw/marks.ts`,
`rust/game_draw/src/marks.rs`, `src/scene/cut.ts`. `blastcost` (`rust/game_start/src/bin/blastcost.rs`) times a blast's tick.

## How it works

**Chaos.** `chaos_of` is one number from 0 to 1, `tune.chaos`, set from the knob `chaos.def` as the game is made. No panel row or word turns it. At 0
most of this page is off: `destruction_round`, `destruction_bang`, `wall_charge_on`, `glass_break_live` and `chew_live` all ask it.

**Heat.** A round or a bang pours heat into a coarse floor grid that decays (`destruction_stoke`, `heat_tau`), and heat × 2 × chaos is what the
thresholds read (`t_decal` up to `t_tile`). One round marks a wall; a beaten spot loses its papers, its clutter, then a tile. Each kind has a rate cap
(`destruction_take`). `loud_now` (a scenario is up) raises a round's heat (`heat_k`), and only then do fixtures die to stray fire and blasts.

**A breakable.** `wall_break_index` walks the level's boxes once. A wall is thin, full height, square to the world, roofed on both sides, and its
`does` says wall: a row of columns (`col`) of two cells each, split at the `sill`. Cover is a low thing with `does.cover`, a screen or a low board, in
a grid of cells. `wall_break_adopt` makes an outer wall a breakable only as a charge is about to open it: that row is `hard`, rounds only scar it, and
a blast opens it only by naming it as its `breach`.

**A cell opens** by one road, `wall_break_open_cells`. `wall_break_rebuild` hides the box and stands the rest as added static boxes, with studs where
metal uprights are left, so every line and ray simply meets those. The same call brings down what stood on it or hung from it (`wall_break_settle`,
`wall_break_hangers`), drops the sound paths (`sound_paths_invalidate`), rebakes the walk grid there (`nav_grid_rebake`), tells the look
(`edge_look_opened`, `wall_skin_opened`), and writes a row of the change list (`wall_break_note`), which `wall_react_take` reads. Past `max_pieces` or
the box world's room, the cells stay shut and it answers no.

**A round.** `thin_path` is the flight: each thin thing takes a toll (`thin_toll`), the round turns a little (`deflect`), and it ends under
`min_energy`. A pane on the way is priced by `pen.rs`; a bar is met by `bar_meet` and broken by `bars_settle`. `commit_walls` lays both holes,
`wall_break_round` counts on the cell it stopped in (rifle 1, pistol `pistol_k`, × chaos × 2), and a cell goes at its `tough`. A sill never goes from
under a standing upper. `destruction_round` does the rest. `coverfire.rs` lets a man spend rounds on what hides an enemy he knows of, by asking the same
engine.

**A blast.** `destruction_blast` runs its listeners (`BlastListener`: the glass and the bars), snow in the way, `doors_blast`, `wall_break_blast`,
`furniture_blast`; then, for a burst but not a charge (the option `chaos` false), `destruction_bang`, `destruction_pall_burst`, `mess_burst`.
`wall_break_plan_of` is the plan: columns in line within `hole_k` × radius go whole, within `port_k` lose their upper part, cover cells within
`cover_k` go, and a charge within `contact_m` widens its hole to `min_hole`. A man can ask first (`wall_break_would_open`).

**A wall charge.** `wall_at` finds the wall under the point, or its reason in words. `wall_charge_settle` slides the place to floor on both sides.
Sam's row is `wall_charge_rows`, his job `wall_charge_sam_tick`. The squad's order is `wall_charge_order`, stepped through `Phase` (`Place`,
`Standoff`, `Armed`, `Wait`, `Enter`) by `wall_charge_step`. The count is `carried` where the card (`kit/`) or the level's `wallCharges`
(`issue_stage`) gave one, else `cost` door charges out of the bag (`wall_charge_price`).

**The blow** (`wall_charge_blow`). Who is caught on the charge's side is asked first (`wall_charge_blast_share`). Then the blast, the pall, and a
borrowed stun can's burst (`detonate_stun`). On his own side a man inside `wallkill.near_m` dies; others go blind (`safe_secs`). Beyond the wall, only
if a hole opened (a wall that held is still a wall), and with a line: inside `knock_m` he is struck down, inside `stun_m` blind and folded. A defender
in that room with no line is folded out to `room_m`. A teammate past the wall is dazzled for `ours_secs`.

**Who stands back.** `safe_radius` gives a stand-off and a reach, larger indoors (`safe_standoff`, `safe_reach`). `wall_charge_too_close` names who is
inside. `wall_charge_clear_of` moves men on fall in to posts from `wall_charge_safe_posts`, away from `wall_charge_known_foes`.

**A burst** (`blast_raw`). The glass first, then each man's line before anything broke (`before`), then `destruction_blast`, then `judge`: hidden
before and shown after is stunned. The 40 mm calls `blast` too (`burst`), on the frag's numbers; under `arm_m` of flight it is a `dud`. A stun can
(`detonate_stun_raw`) tears leaves by `tear_m`, then `destruction_bang` with stun, where only fixtures within `stun_fixture_m` go.

**Furniture** (`furniture_blast`). Reaches scale by what went off (`BlastSource`). It blows from the charge and from the far mouth of each hole just
opened, and a thing needs a line from one of them (`furniture_exposed_box`). Fixed furniture shatters inside `shatter_m` or comes apart inside
`apart_m` (`furn_break_row`); props are thrown inside `throw_m`. `FurnKMouth` clears a hole's way, whoever owns what stands there.
`furniture_hung_goes` tears boards hung under a charge.

**Pieces.** `debris_spawn` first calls `debris_thrown`: `flight_of` says where it comes down and the landing is kept in `g.falls`.
`debris_landings` tells the floor when it is due. The pool (`DEBRIS_CAP`) then tumbles what is drawn. Full, it takes the smallest resting
piece's slot (`breaks_pick`); with all in the air it refuses. `debris_feet` shoves rubble. `destruction_bodies` keeps a piece off a man down;
`way_off` slides it the shortest free way clear of all of him.

**Glass.** A pane's state is a row in the block (`panes.rs`). `glass_impact` is the way in for a blow, `glass_break_blast` for a blast,
`glass_break_bang` for a stun. `place_rule` deals its kind: narrow is laminated; a foot near the floor or glass by a door is toughened; the rest is
float. The rule lists the pieces that let go; `glass_break_drain` hands each to `glass_shards_release`, the picture's half. Each landing and the glass
it leaves are kept in `falls.rs`, where `step` makes a footfall louder and grinds the litter. A pane is a wall to feet until what stands below
`walk_clear_h` has gone (`glass_break_walkable`).

**Dust and smoke.** `destruction_pall_wall` and `destruction_pall_burst` start wide slow emitters in the smoke solver, plus particles. The solver's
density is real, so men's eyes read it as the picture does. `destruction_dust_puff` is a round's thin kind.


**Marks.** `bullet_mark`, `bang_scorch` and `chip_mark` only ask. `marks_settle`, near the tick's end, answers, and `lay` keeps the list (`CAP`).
`wallwear.rs` sizes a round's own mark by how far gone its cell is.

**Cut boxes.** A box carries up to `MAX_CUT_PLANES` planes. The rules make them in `shape_box` and `breaks_ragged`. The game's rays meet the cut solid
(`cut_met`, `cut_clip`).

**Pacing.** On the tick a wall goes it gets its cells, soot and marks only. `edge_look_dress` queues the rest: the next tick dresses the wall
(`edge_look_flush`), then `spread` cells a tick are shed. Puffs wait in `dust_queue` under `puffs_per_frame`, and `feel_stages` lays out the feel over
time.

**Restart and load.** A restart runs `glass_break_reset`, `destruction_reset` (it calls `wall_break_reset`, `furniture_reset`) and `decals_clear`. A
load puts back a clone of the whole game and its block (`savestate/`), so nothing has to be told.

## Rules that hold

- **A blast does the same to anyone.** "blowing a wall charge with me right next to it should kill me". Sam, teammates, defenders and civilians alike
  (`guard_caught`, `civilian_caught`, `player_caught`).
- **It catches whom the warning named.** `wall_charge_too_close` and the blow ask the same thing, before the blast, so furniture it smashes exposes
  nobody late.
- **The men step back.** "after i place a wall charge, my guys should move away with standoff". A man told to hold stays, and the row says so.
- **A wall shot shocks the room.** "the smoke after a wall shot makes it too hard -- we should probably (1) turn down its density a little and (2)
  make it so everyone in the room gets more of a shock" (`WallshotK`).
- **A blast raises real dust.** "i'd also expect much more dust/smoke from a wall charge"; "it should feel pretty chaotic with HE going off".
  `per_window` palls in `window_secs`, and a wall charge is never the one refused.
- **A hole can be walked through.** "couldn't path through a hole from a wall breach". What stands in either mouth goes, up to `top` and `vol`.
- **Stunned, not killed, behind a wall the burst itself broke.** "Stun, like doors". The same for a leaf it tore (`torn`).
- **A round goes on.** "a rifle round carries on through drywall, partitions and wooden doors with the energy it has left, as it already does through
  glass". Steel, glowing boxes and fixtures stop it (`stuff_of`).
- **One round, one bar.** "for gameplay it'd be nice if they broke off with the glass and just as fast. so each bar breaks up with just a single
  bullet?"
- **Pieces are dressing; their landings are not.** "let's make sure we're very explicit about what's gameplay affecting (and also run on the server)
  and what's just client side only dressing (debris, smoke details)". A full pool hears the same landings. No rule reads a mark.
- **A floor keeps what happened.** "maybe we've just blown up too much stuff but the office floor looks too clean" (`floormess.rs`).
- **Pieces are made of the thing** (`mat_of_surf`), and a hit goes by `does`, never by a color (`surf_at`).
- **A level's places for a charge are counted.** `levels charges`: a loss is fine "so long as it's deliberate", and a line in the count's file says
  why.

## To change it

- **A number:** `rust/game_types/src/knobs/`: `destruction.rs` (`ChaosK`), `wallbreak.rs` (`WallsK`), `wallcharge.rs` (`WallchargeK`, `WallshotK`,
  `WallkillK`), and files of the module's name. A panel or `?knob.walls.blast.radius=3` turns one.
- **What a stuff does:** its `MATERIALS` row in `src/scene/materials.ts` with the twin `does_of` reads (`levels.md`); a toll in `WallpenK`, a `tough`
  row, a `break_row` in `game_types/src/break_table/`.
- **A new blast:** `blast` if it judges men, else `destruction_blast`, with `who`, `radius`, `power`. Open cells only through `wall_break_open_cells`.
- **Something that reacts to a hole:** read `wall_break_since`. Something a restart must clear joins `destruction_reset`.
- **The glass rule:** `rust/sim/src/panes.rs`. A fragment's look is `shards/`.
- **Wall places on a level:** `levels charges LEVEL --write`, in `tools/levels/counts/charges`, with a line that starts # to say why.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `wall_breaks.rs`, `blast_holes.rs` | a hole agrees for feet, eyes and rounds; chewed cover; a restart; asking first; a man walks the hole |
| `wallcharge.rs`, `wall_kills.rs` | it catches whom the warning named; at it he dies, past it he is blind; men step back out of known fire |
| `held_charges.rs`, `guns_and_debris.rs` | both hands, the count, whose key; a piece is heard where it lands, a boot files nothing |
| `rounds_and_saves.rs` | what a round goes through; hits by `does`; a save carries the broken level; one flood for two walls |
| `breaks.rs`, `debris_pool.rs`, `furniture.rs` | pieces of the thing, off a man down; a full pool; a wall shields furniture, a restart mends it |
| `glass.rs`, `glass_open.rs`, `burst_panes.rs` | a pane broken early; what is open to feet, rays and sound; a burst and the glass between |
| `held_frag.rs`, `held_forty.rs`, `marks.rs`, `restarts.rs` | whom a burst takes; the 40 mm's arming; marks put back; a restart is a fresh staging |

`levels charges` holds each level, and `hold` holds the marks to no rule.

## Not seen

How a pall, a ragged end or a film looks is judged on the page or not at all. `blast_feel/` plays its report at once, with no delay for distance.
