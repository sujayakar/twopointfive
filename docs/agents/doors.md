# Doors and furniture

What a door is, what moves it, and what it does to whoever is in its way. What the squad does at a door on order is `door-tools.md`; blowing
one open is `destruction.md`.

## What the player gets

- A door opens one way. From one face you push it, from the other you pull it, and its row says which.
- You can ease it open in silence, shut it, pick its lock, use a key, kick it in, look under it with the wand, or run at it.
- It stays where you leave it. Nothing swings shut by itself.
- It has weight. A leaf pushed into a man moves him, one kicked into him throws him, and one that opens toward you is solid to you.
- A kick is heard across the floor. Twelve rifle rounds take a wooden leaf apart. A steel one is never opened by force.
- Enemies lock doors, wedge them, shut them on their rounds, fire through them and boot them.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `doors/` | a door: its leaf, lock, latch and boxes; who is locked out; the boot; the hand; a man's path threaded through a frame |
| `doorswing.rs` | which way each door opens, dealt once as the level is made |
| `doorway.rs` | manners: who waits at a frame, who goes round whom, where nobody stands about |
| `handleside.rs` | a man opens a door from where its handle is, off the floor the leaf sweeps |
| `sprintdoor.rs` | Sam running at a door |
| `doorthrow.rs` | what a leaf driven into a man does to him |
| `doorblast.rs` | a leaf off its hinges, splintered by gunfire or torn by a blast, and the slab it becomes |
| `doornames.rs` | what a door is called out loud |
| `shutdoor.rs` | the squad's order to shut one |
| `doorforce.rs`, `darkdoor.rs`, `digin/` | defenders: firing and booting through a shut door, staying out of a dark room, wedging (`enemies.md`) |
| `props.rs` | furniture: pushed, shoved by a leaf, thrown by a blast |

The step itself is `rust/sim/src/doors.rs` (and `rust/sim/src/props.rs`): every leaf for one tick, on flat columns. `game_update_doors` in
`rust/game_rules/src/tick/game/doors.rs` gathers who is near a leaf, hands it over, and takes back where everybody ended. Sam's rows are in
`rust/game_rules/src/sam/player/interactables.rs`. The level file's record is `DoorData` in `tools/levels/lib/levelRows.ts`; a doorway's size is
`tools/levels/lib/doorSize.ts`. A leaf is drawn as any box is; `rust/game_draw/src/doorways.rs` adds the doorways the light's reach asks for.

## How it works

**A leaf** is an angle and a turn rate between two stops, one of which is its frame. Its numbers are columns of the game's one block, so they are in
every save and replay with nothing to add. A leaf off its hinges (`door_off`) reads to everybody as an open doorway.

**A tick.** Each man, Sam and each body near a leaf is a contact (`DoorContact`): where he is, how he moves, what keys he has, whether he is quiet.
The step turns leaves under them, moves men out of a leaf's way, latches, and lists the noises. Then `door_throw_after` makes what it did to a man,
and `door_slabs` moves the loose leaves.

**Which way it opens** is the first of these that speaks (`deal_swings`): written in the level; a stop that only folds one way; a drill's plan; into the
room it serves (the smaller of two); into the hostages' room; into the mission's; away from its lock; the way it stands open; the side with
less floor; the wider stop. Then a door whose leaf would cut off floor on that side, and not on the other, turns round.

**The lock** is on one face (`keySide`). `doors_locked_out` is the one question: latched, locked, on the keyed face, and nothing on your ring fits.
Every lock has an id; a man has a ring of them. Picking takes `DOORS_PICK_SECS`; left alone for two seconds it slips back.

**The boot** (`doors_kick_in`) breaks the lock and flings the leaf. From the face a door opens toward, the frame holds it (`doors_boot_holds`): a thud, and
nothing gives. Steel only rings.

**Through a leaf** men hear less and a round may go on (`wallpen.rs`). A leaf is in the dynamic boxes, so sight, light, smoke and sound all ask the leaf where it
stands.

## Rules that hold

- **One way, always.** An angle written on the wrong side is mirrored (`door_keep`), whoever writes it.
- **Real size.** 0.9 by 2.05 m in every level, its middle on a walk-grid cell. "having it feel realistic here would be good."
- **Doors stay as left.** Only a hand, a body, a boot or a blast moves one. Calm staff on patrol shut their own keyed and outside doors behind them.
- **The handle side.** "open has to be on the handle side for AI." A man goes to where the handle is and stands off the sweep.
- **A door opened into a man alerts him; a door kicked into him throws him.** "opening a door into an enemy standing on the other side should just alert
  him, it's not like we're slamming these doors", and "kicking down a door when an enemy on the side should stun or knock them out". An opened
  leaf moves him aside and startles a defender. One slammed at a run folds him for a moment, a booted one for longer. It hurts nobody. An eased leaf, a
  pulled one, one opened by a calm man, or opened short of a boot by his own side does nothing.
- **It is solid to Sam as it opens toward him.** "it's very easy to clip through a door when walking towards it and it swings out towards you, player should have
  to open it and walk aroudn it". He is carried back or along it, and stands clear before he goes through.
- **A sprint has weight.** From the push face it is a shoulder: the leaf slams and he keeps most of his speed. From the pull face it is a boot that takes the leaf
  off. Locked, the boot works from one face and the other is a wall.
- **Shooting one out is a cost.** "it should be a real gameplay tradeoff (requires a lot of ammo? maybe like 12 shots?)". The same count at any destruction
  scale; a pistol asks twice as many. The lock rail is left lying by the frame.
- **A wedge holds a door only toward the man who set it** (`doors_wedge_holds`). From inside, a hand opens it.
- **Every room they can shut has another way in.** "i worry that it effectively requires the player to blow open the door with a door charge or 40mm. maybe we
  just need to design levels so there's always another route in?" A level that breaks it hangs a door to open out, or gets a window a man steps through.
- **Nobody stands in a doorway.** A man waiting keeps off frames, sweeps and the lane through an open door; two men never meet in one.
- **A teammate goes round a leaf standing open**, and pushes it only once he has walked on the spot against it.
- **Furniture** slides when shoved, scrapes and is heard; a man low makes half the noise, whoever he is; an office chair rolls; a thing shoved through a frame
  is steered to its middle so that it cannot jam.
- **No compass word in a door's name.** A level names its own doors; a shorthand id is never said.

## To change it

- **A number:** `rust/game_types/src/knobs/` (`doors.rs` for a driven leaf, `doorblast.rs`, `doorway.rs`, `sprintdoor.rs`, `handleside.rs`, `props.rs`). A panel
  or `?knob.doorthrow.kick_secs=3` turns it. `?doorcost=0` is the cheap count for shooting one out.
- **How a leaf moves:** `rust/sim/src/doors.rs`. A column it keeps from tick to tick is added to the list at its head. What it needs of the game is an
  argument of the one call; it keeps nothing else. Its constants stand a second time in `rust/game_types/src/doors/`, for the rules that read them.
- **A new way to work a door:** a row in `rust/game_rules/src/sam/player/interactables.rs` for Sam; for the squad an order (`door-tools.md`). Move the leaf through
  `doors_use`, `doors_crack`, `doors_pull`, `doors_pull_to` or `doors_kick_in`, never by its angle, so that the noise, the blame and who it struck follow.
- **A noise:** `doors_sound`, which is heard by the rules and played by the mixer alike.
- **A door in a level:** its generator; `closer`, `exterior`, `locked`, `lock`, `keySide`, `swing` and `steel` are the words. Another size is `DOOR_SIZE` and every
  level exported again.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `held_leaves.rs` | a leaf stops at a man or carries him, a boot throws him, a pulled one passes |
| `held_sprint.rs` | a sprint at each face, locked and not |
| `door_stays.rs` | a door stands where left, a keyed one locks again only when shut |
| `door_leaves.rs` | the handle side on every door of every level; what a blast leaves |
| `door_body.rs` | a man who opens a door on Sam does not stand in the frame |
| `door_hands.rs` | who stands where when a leaf opens toward the team |
| `held_doors.rs`, `stack_doors.rs`, `two_doors.rs`, `squad_doors.rs` | the team at a door: found shut, locked, wedged, two teams at two doors |
| `held_doorforce.rs`, `fight_doors.rs`, `dark_rooms.rs` | men who fire, boot and wait at one |
| `furniture.rs`, `pair.rs` | furniture and blasts, the noise of a shoved chair |

`levels doors` and `levels second-way` (`rust/game_start/src/levels.rs`) hold each level's doors to their size and its rooms to a second way in.
`gate`'s pins move if a leaf stands anywhere else.

## Not seen

How a thrown man, a carried man or a rolling chair looks has been checked by numbers only. A corridor narrower than an open leaf and a man still makes the
team push the leaf to and open it again.
