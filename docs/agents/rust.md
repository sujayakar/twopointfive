# The simulation in Rust

The maintainer's ask (2026-11-02): "we should just write this in rust so we have good control over everything (and aren't subject to the vagaries of each JS
engine). and then we can also use wasm simd for accelerating things (e.g. ECS layout)", and earlier that a Rust simulation "might even be desirable for us
running this elsewhere". The game's update (`Game.update`, about 100,000 lines over `src/game`) is moving into Rust, built to WebAssembly, a system at a
time. What is built so far is the ground: the crate, its build, its one way into the page, the server and the tools, the switch table, the checks, and the
first system across (the game's own maths). No rule of play has changed and nothing a player sees is different.

- **Where it lives.** `rust/` is a Cargo workspace (`rust-toolchain.toml` names the compiler and the WebAssembly target; no crate from outside, so no network
  at build time). `rust/sim` is the simulation: `src/lib.rs` (what every line owes the game, and the C exports, named `<module>_<thing>`), `src/maths.rs`,
  `src/pow10.rs` (the decimal powers as the language reads them, written by `bun run tools/sim/pow10.ts`: the Rust side never parses a decimal). `rust/prim`
  is three functions (√, floor, trunc): the simulation is `no_std` and the core library lacks them. `rust/dress` is the other side of the line (below,
  gameplay and dressing): its own module, `src/dress/built.ts` and `src/dress/dress.ts`.
  `src/sim/built.ts` is the module's bytes as text, WRITTEN BY `bun run tools/sim/build.ts` and checked in. `src/sim/sim.ts` is the one TypeScript file that
  loads the module (`sim()`, through `src/sim/load.ts`), names its exports (`SimExports`, `SIM_EXPORTS`, `SIM_ABI`), hands out its memory (`f64()`, `u8()`) and holds the switch table
  `RUST`. Like `src/math`, `src/sim` is below the game: it imports `built.ts` and `src/math` and nothing else.
- **Nobody needs Rust to play, or to write TypeScript.** The page, the game server (`tools/mp/`), the hosted build and every headless tool load the module
  from `built.ts` as they load any source file. The price: `built.ts` must follow `rust/`. AFTER ANY CHANGE UNDER `rust/`: `bun run tools/sim/build.ts`, and
  commit `built.ts` with it. `bun run tools/sim/build.ts --check` (the `built` lines of `tools/qa/rust-check.ts`) is red when the hash of the crate's sources
  in `built.ts` is not their hash now (no compiler needed), when the module is over 6 MB, when it carries a path of the machine that built it, when the
  powers' table is not what its tool writes, and, where
  there is a compiler, when the crate built again is not the checked-in bytes (another builder than the file's `SIM_BUILDER`, its compiler and kind of
  machine, is told, not failed: nobody has built this on a Mac yet, so whether the bytes are the same there is not known). The build passes cargo its own
  flags and its own output folder, whatever the shell's `RUSTFLAGS` or `CARGO_TARGET_DIR` say; `.gitattributes` keeps `rust/` in LF, since the hash is of bytes.
- **One way in, and it is synchronous.** `sim()` compiles and starts the module in one step on first use (2 to 3 ms headless, 1.5 to 11 ms over five boots in
  Chromium for the 32 KB of today), so nothing that calls it is written around a promise and there is no second path for the page. Chrome allows that on the page's own
  thread up to 8 MB (measured: 6 MB compiles, 9 MB is refused with a RangeError); the build is red at 6 MB. If the whole update ever outgrows that, the
  answer is a boot step that waits, decided then. The page starts the module at boot and says so once on the console (`rust simulation: …`, `simSay`).
- **A browser with no WebAssembly.** A browser whose policy turns its JIT off has no `WebAssembly` at all. `simWhyNot()` is the one answer to "can this
  runtime run the module" (no WebAssembly, a compile the page's policy refuses, a `built.ts` of another version: asked once, by starting it); where it
  cannot, the page says NOT RUNNING on its console and plays on the TypeScript simulation, and a row of `RUST` asked for there stays off (`rust-page.cjs nojit`). THE DAY THE UPDATE IS IN RUST ALONE
  SUCH A PAGE CANNOT PLAY. That is a cost of the goal the maintainer should know before the TypeScript update is taken out.
- **`RUST`: a row a system** (`src/sim/sim.ts`). On = that system of the update runs in the module; off = in TypeScript as it always did. THE TWO ARE THE SAME
  GAME TO THE BIT, which is what lets a system move at all. So a row is never a rule of play: it joins no pin list and moves no digest (the frozen stand
  leaves every row off; the rows of `RUST_ON` are on in the page and so on the shipped stand, with a `FROZEN_HOLDS` line each: "Rows that are on", below),
  a recording made with one on replays the same with it off, and the journal carries the table so that a recording says which ran (a file from
  before the table replays with every row off, as it was made: `journal.ts applyKnobs`). It is in the knob registry, so a recipe carries it and a server and its clients run the same rows, unless a client's own address says
  otherwise (`profile.ts pageRows`: what the address says of the rows is written again over a recipe's). A row
  goes ON in the page (`RUST_ON`) once its system is proven and costs nothing there; until then a check raises it. Rows are setters: writing one moves the system.
  - `maths` — the game's own maths (`src/math/exact.ts`, when `EXACT.on` has put them in `Math`'s place) from the module: `exact.ts mathsFrom` takes the
    same fourteen functions and the three lengths of `math/hyp.ts` from elsewhere. `?rustmaths` in the page, `TPS_RUSTMATHS=1` headless. OFF EVERYWHERE, and
    meant to stay off: from TypeScript the crossing costs more than the dozen operations it saves (bun 1.4: sin 27 ns in TypeScript, 44 ns across; exp 34 and
    83; a length 17 and 22. node 22: level, within 2 ns either way except sin, 46 and 55). The row exists as the proof that the two arithmetics are one,
    in a whole game and not only on a stream of inputs; the Rust systems that follow call the Rust maths with no crossing.
  - `lines` — the box world's line tests (`collision.ts segmentBlocked`, `segmentBlockedDynamic`: every sight line, round and reach) from `rust/sim/src/lines.rs`,
    over the module's copy of the boxes (next bullet), inside a tick. `?rustlines`, `TPS_RUSTLINES=1`. The length is worked out in TypeScript and handed
    over, so the row answers under either maths wherever no pane's rule stands. GLASS: whether a pane is open where a line crosses it is answered in
    the module (`glass.rs`, the bullet "A pane's open question" below), by the game's own maths, so with glass breaking on the row answers under the
    game's own maths alone (`collision.ts paneAsk`; under the engine's the TypeScript walks any line that may meet a pane). Only a line of fire still
    has the walk LIST the glass it crosses that is not open, for the penetration rule (`glasspen.ts`), which is TypeScript. THE RAY IS A ROW OF ITS OWN, `rays` (`?rustrays`, `TPS_RUSTRAYS=1`; `collision.ts raycast` →
    `lines.rs ray`, 2026-11-03: the nearest thing a ray meets, how far along, and the normal of the face it enters by — `boxes.ts rayBox` whole, over
    plain, tilted, round and cut boxes; the leaves standing in `dynamicBoxes` first, then the grid in the same half-metre steps). Every rule of the
    things that flies something through a level asks it (a round, a thrown can, a torn leaf, a piece of debris: 102 callers), so it had to be in the
    module before any of them could be. A PANE MET WHERE IT IS OPEN IS PASSED IN THE MODULE and the ray flies on to what stands beyond, as the
    TypeScript's walk does (until the pane's open question moved there the module took every pane as shut and the host walked the ray again:
    6,500 rays in twelve mission runs). `raycastHere` is the TypeScript's own. `RAYS.n` counts the
    module's, `RAYS.host` the ones the host walks (a ray that is not a number, which the host's walk fails on loudly). The ray rode the lines' row for a
    day, on no named build; it has its own so that the lines' row can be on in the page while the ray waits its turn, and a rule stepped in the
    module (a slab, a thrown thing) asks its rays there whatever this row says. THE LINES ARE ON IN THE PAGE AND ON THE SHIPPED STAND since 2026-11-03
    (`RUST_ON`: the bullet "Rows that are on"), off on the frozen stand; THE RAY'S ROW IS OFF EVERYWHERE. Measured 2026-11-03 on a 4-core cloud
    box under bun: a sight line 1.81 → 1.08 µs on the office and 1.03 → 0.77 µs on the compound (`tools/qa/segment-bench.ts`). THE TICK, with `lines`,
    `circles` and `fog` up together against none (`tools/qa/tickcost.ts --each --list --rounds 2`, two runs, one seed each, the same bits in every table;
    the rows-off table taken before and after): 4.81 → 4.59 ms mean over 13,676 ticks, and 4.81 again off. pass-four 5.28 / 5.13 → 4.68 ms (its ticks
    over 8 ms 594 / 363 → 260); compound-four 4.48 / 4.58 → 4.52 ms, which is nothing: that machine's two rows-off tables differ by as much. So about
    5 % of a tick, where the profile gave collision and the lines 12 %: the rest of that share is what calls the lines, not the walk.
  - `circles` — a circle against the boxes (`collideCircle`, `circleTouches`, `gather`: a body pushed out of the statics, what a prop or a rag doll rests
    against) from `collide.rs`, the same copy. `?rustcircles`, `TPS_RUSTCIRCLES=1`. No gain of its own was measured (the number above is the three
    rows together). ON where `lines` is (`RUST_ON`).
  - `fog` — the fog of war's sight pass (`fogofwar.ts update`: which of the team's eyes has a line to each man and each body) from `fog.rs`: the eyes are
    handed over once a pass (`collision.ts sightOpen`, `sightEye`) and a man is then ONE question (`eyesSee`) where he was two lines an eye. `?rustfog`,
    `TPS_RUSTFOG=1`. It works its lengths out in the module, so IT ANSWERS ONLY WHERE THE GAME'S OWN MATHS STAND (`EXACT.on`: the page, the server, the
    shipped stand); on the frozen stand, whose lengths are the engine's, the TypeScript answers whatever the row says. That is the rule for every system to
    come that takes a length or an angle. What the fog draws with (`things`, the picture's blocks) is not in the row, and that is where the fog's
    time is: the sight pass was its lines, so the row has no gain of its own. ON where `lines` is (`RUST_ON`): it rides the lines' copy and costs nothing of its own.
  - `poses` — two things under one row. THE POSES A CHARACTER KEEPS from one tick to the next (`src/anim/skeleton.ts Pose`, made through its animator's
    `PoseSet`: the blend's stages, the layers' own buffers) live in the game's block of the module's memory (`rust/sim/src/poses.rs`, a tenant:
    `src/game/posehome.ts`; the bullet "State in the module is state of the game"). `Pose.cols` is the floats wherever they are (ten columns, a node a row: the bullet "The data's layout"), asked for at each
    use and never kept in a field, so the layers of `game/character.ts` (TypeScript still) read and write the same expression either way. And THE POSING
    FUNCTIONS (`sampleClip`, `blendPose`, `PoseFK.run` / `runFrom` / `parentRot`, `rotateLocal`, `rotateModel`, `setModelRot`, `twoBoneIK`) are asked of `skeleton.rs`. A
    question says where its pose is: in game `g`'s pool, where it is worked on in place, or (`g` = −1: a function's scratch pose, a pose whose block
    stands parked) on the module's BENCH, written there and read back. One body a question in Rust, one export, whichever it is; an answer of "not a
    pose" throws in the host. `?rustposes`, `TPS_RUSTPOSES=1`. STILL A PROOF, NOT A SPEED-UP: the layers that ask are TypeScript, so a question is a
    call, and a call costs more than a blend of fifty nodes. What it proves is that the two are one arithmetic in a whole game AND that the pose
    store is right through every road a snapshot takes, which is what a layer written in Rust needs to find. The questions are asked only where the
    game's own maths stand (`benchFor`: `EXACT.on`); where the poses live does not depend on it. A pose is stored in singles as the Float32Array stores
    it; `rust/sim/src/single.rs` is the one file that may name a single (the build's rule names it as its exception), and nothing is worked out in
    singles. THE POOL HOLDS THE POSES OF THE CHARACTERS WHO ARE IN THE GAME AND NO OTHERS (`posehome.ts`): the players' men, the shift, the civilians.
    As a tick opens, a set at home that is on none of the game's lists any more (a restart made the guards anew, a man was taken off the floor)
    means everybody comes out, the pool is emptied and those who are there go back in: one rule for whoever left, however he left, and nothing in
    `resetGuards` or anywhere else has to tell the pool. A pose the pool has no room for stays an array of its own (the same game) and is tried again
    each tick. Not ported yet: `SkeletonInstance.update` (the twists, the rag doll's
    overrides, the matrices), and the run-forward bones (`PoseFK.rot` / `.pos` are the host's arrays, copied to the bench and back a question).
    OFF EVERYWHERE.
  - `walk` — THE ANIMATOR'S LAYERS (`game/character.ts CharacterAnimator`: what turns a blend of clips into a man who leans into his walk, breathes,
    keeps his soles where he put them and holds a rifle steady) run in the module, a layer at a time (`rust/sim/src/layers.rs`, a `pub fn` each;
    the bullet "A layer of the animator" below is how one moves). Each is the TypeScript method statement for statement, on the pose where it lives,
    so the row needs `poses` at home and the game's own maths (`layerSlot` answers −1 otherwise and the TypeScript method runs: the frozen stand, a
    pose whose block stands parked). The method stays, behind the row, as `<name>Own`. `?rustwalk`, `TPS_RUSTWALK=1`. OFF EVERYWHERE.
    - MOVED SO FAR. THE BODY (pelvis, legs, trunk, head), twenty-four: `standPose` (the blend of clips: the idle and the cycle over it, the
      walk, the jog and the sprint, each run shaped before the blend; `jogShape` and `sprintShape` are steps of it, not layers asked alone),
      `crouchPose` (the crouch laid over that), `mvPre` (the recoil bled off, the muzzle's dip, the trained glide's pelvis), `mvTrunk` (the
      trunk's share of MOVES), `mvPostBody` (the flinch, the start, the hit, the blinded hunch, the fold, a hurt and a shaken man's carriage,
      the toss's wind-up), `longBody` (the long gun's trunk set), `lifeLayer` (the breath, the body's mass, the weight shift, the standing
      foot lock), `stanceBody`, `runArms` (a run's arms carried close, RUNARMS: `docs/agents/player.md`), `softKnees`,
      `runPlant`, `steadyLayer` (the crouch walk carried level), `sneakBody`, `creepBody`, `watchSlip`
      and `watchLook`, `kickBody`, `toesUp`, `wallBody` (the wall hold: its feet, its sidle's gait, its peek), `leanBody`, `actPose` (a
      one-shot action's clip over the pose, and the slump), `woundBody` (the wounded man's clip, his head up to the wound), `gazeKeep` (it turns
      nothing: it ANSWERS the yaw the host's twists take out, and an answer is compared like a column), and one RULE OF THE CHARACTER'S THAT
      HAS NO POSE, `Character.turnWeigh` (asked through the same bench with no pose handed: `anim.ruleSlot` / `ruleRan` / `ruleBoth`).
      THE ARMS AND HANDS: `sneakArms`, `carryArms`, `wallArms`, `woundLimbs`, `longArms`, `mvPostArms` (the start's
      shoulders, a blinded man's gun and hand, the toss), `watchArm`, `creepArms` (the close-in carriage's, with the sheath's fist),
      `clutchArm`, `boundArms`, `reloadArm` (the long gun's change: box, belt and shells), `pistolReloadArm`, `signalLayer`, `tellLayer`,
      `smokePose` and `nvPose` (one function, a hand brought to the head as it will be drawn: `head_hand`), `torchUpLayer`, `radioPose`,
      `lockpickLayer`, `haulLayer`, `swapPose`, `stealthLayer` (the crouch's own arms, with its seven columns), `cablePose` (the look under
      a door), `takenLayer` (a man taken from behind), the blow (`blowLegs`, `blowTrunk`, `blowArms`, and the knife's `knifePose`),
      `readyLayer` (the ready pose: thirty-nine columns), `kitLayer` (a change of kit, with the spot it goes to), `doorLayer` (a hand on a
      door: six columns), `workLayer` (hands at work on keys, a charge, a lever, a thing taken: nine columns), the hand shape they share
      (`hand`, below), `handsPass`, the hand pass itself, and the last five that posed in TypeScript alone: `holdLayer` (Sam holding a man
      from behind: thirty columns) with `catchArm` (`catchFix`'s arm), `heldLayer` (the man he holds: twenty-six), `bigLayer` (the big
      slot's hammer or shotgun: four columns, and the pose kept under the hammer, which the dispatcher makes before either language is
      asked so that it is made on the same frame both ways) and `throwLayer` (the throw at every size: eight). STILL TYPESCRIPT ONLY:
      `update` itself, which weighs the layers and calls them.
    - A POSE'S SINGLES ARE READ AND WRITTEN IN ONE PLACE in `layers.rs`: `ch(d, node, channel)` and `set_ch` (0‥2 the origin, 3‥6 the
      rotation, 7‥9 the scale). No layer writes where a single lies; a table handed from TypeScript names a NODE (the jog's seam), never
      a place in a pose. A pose laid out another way is those two lines.
    - A METHOD THAT POSED BODY AND ARMS IS SPLIT, body first where the order allows (`mvPost` → `mvPostBody` + `mvPostArms`, with what the body
      hands the arms kept as columns; `watchLayer` → `watchSlip`, `watchArm`, `watchLook`). A METHOD THAT KEPT A CLOCK AND POSED keeps the
      clock and hands the pose to a layer (`actLayer` → `actPose`, `woundLayer` → `woundBody` + `woundLimbs`): which clip plays, its time and
      its weight are fields of the animator until `update` itself moves. A LAYER THAT IS A GATE AND A POSE (`creepLayer`) keeps the gate and
      hands the pose on (`creepArms`). A LAYER WITH NOTHING TO POSE this frame may answer before it asks for a slot (`mvPostArms`,
      `watchArm`): one test in the dispatcher, the same for both sides. A CLOCK WITH A POSE UNDER IT keeps the clock (`smokeLayer` →
      `smokePose`, `nvLayer` → `nvPose`, `radioLayer` → `radioPose`, `swapLayer` → `swapPose`, `stealthPicture` → `cablePose`). A LAYER
      NEED NOT POSE `pA`: `cablePose` is asked on the picture's own pose (`layerSlot(ch, pic, …)`, `layerBoth('cablePose', pic, …)`), and
      a pose a layer KEEPS for a later reader (`swapPose`'s pose as it stood) is another pose handed by its place and compared as `also`.
      NOT moved yet: the two holds (with the catch's arm), the big tools, the throw, and `update`, which is the order the layers run in,
      their weights and the springs under them.
    - AN ARM LAYER works as the TypeScript does: the body copied into a scratch pose (`ask_blend` at weight 1 is a copy), the solve there,
      and the blend back through a mask the module writes on the bench (`ask_mask` = `buildMask`). A pose it reads besides (the aim clip's,
      the rest pose) is another pose of the same pool, handed by its place.
    - THE HAND SHAPE (`CharacterAnimator.hand`) is one function of `layers.rs`. It needs what the hands pass keeps: each side's digits as the
      locomotion left them and each side's notes. Those are THE HAND PASS'S COLUMNS, handed after a layer's own (`HAND_COLS_AT`, 130
      doubles; `handsAsk` on the host) to every layer that shapes a hand, and the notes come back; `layerBoth` compares them. The digits
      are two lists of the rig (`digitsL`, `digitsR`).
    - THE HAND PASS (`handsPass` → `hands_pass`) turns digit joints alone. Its own state is five columns (the grips eased, seconds since he
      fired, the finger off the trigger, the gun eased); it reads the hand pass's columns; and A MAN'S OWN TABLES (the library's fist, the
      turn to his open hand, the crouch's own hand, his thumb, the straight trigger finger: 286 doubles, built once a man by `handsBuild`,
      which stays the host's) are handed where a path of keys rides (below), kept as one run of doubles on the animator (`handTables`).
      They belong in a character's columns when those are drawn.
    - A PATH OF KEYS A LAYER IS HANDED (the stations of a magazine change: `gunreload.ts feedPath`, the game's own timing, which the sound
      beats read too and which is not posing) rides the bench after the hand pass's columns: `LAYER_KEYS_AT` 264, `LAYER_KEY` 7 doubles a
      key, `LAYER_KEYS_MAX` 48 (`skeleton.ts`; `KEYS_AT`, `KEY`, `KEYS_MAX` in `layers.rs`; the bench's `ARGS` is 1,024). A path longer
      than the room is a mistake, said by name where the layer is asked (`skeleton.ts layerFits`): no second way poses it. Two more things ride the same place: the times of the
      blows of a big tool's beat that the layer reads (one double each, at most two: the last landed and the next coming, `character.ts blowsNear`) and the throw's seventeen tables of keys (`throwLayerAsk`:
      seventeen counts, then eleven tables of two doubles a key and six of four, `THROW_KEYS_MAX` 16 keys a table), which are the
      TypeScript's own `THR` / `THR_LOW` handed over at each ask, so there is one copy of every key.
    - A CLIP A LAYER NEEDS it samples itself (`sample_on_rest`: the rest pose copied into a scratch pose and the clip over it; `clip_at`: the
      clip into a pose as it stands). The layer is handed the module's NUMBER for the clip with its own numbers (`skeleton.ts clipNo`, which
      writes the clip into the module if it never was), and the host asks such a layer only when the module holds every clip it samples
      (`clipsIn`; otherwise the TypeScript runs it). A SCRATCH POSE A LAYER LEAVES FOR THE NEXT (the cycle's own sample in `pC`, which
      `steadyLayer` reads) is compared too: `layerBoth`'s `also`. An OBJECT A LAYER MAY OR MAY NOT HAVE is a "there" column and noughts.
    - A RIG'S OWN TABLE OF NUMBERS (what the animator worked out once from the character file and reads every frame: the jog's seam) is a
      line of `knobs.rs` under `Num` and an entry of the animator's `cast.nums`; the host writes it with the rig's named nodes and lists
      (`layerSlot`), the layer reads `nums(JOG_SEAM)`. A COPY (`build.ts STATICS`: `NUM`, `NUM_LEN`), like the roles. What is still handed
      a call at a time and belongs there when the characters' columns are drawn: the hands' calibration (`handFix`), `gripUp`,
      `chestRestInv` and the sheath's rest inverse, `gazeLocal`, the idle's feet, the run's pelvis heights, a man's hand tables.
    - A CURVE OR A CONSTANT OF THE ANIMATOR'S is read where the TypeScript keeps it, never copied: it is a line of `knobs.rs` under `ANIM.`
      (`character.ts animTables()` names the tables: `ANIM.MV_FLINCH_ENV.1.0` is a key's time), a curve's keys named two by two
      (`FLINCH_KEYS`), so the red path bends it too.
    - KNOBS OUTSIDE A TICK. The module's copy of the knobs is written as a tick opens. A pose made outside one (a teleport's re-pose) would
      read the last tick's: `CharacterAnimator.update` begins with `LAYERS.knobs?.()` (`rustknobs.ts knobsNow`: hands them over unless a tick
      already has). Found by the wall hold's probe, which flips a knob and teleports. `layer-ident`'s `changed` line holds it.
    - WHO IS ASKED: `layerSlot` answers a slot for a man whose every pose stands in the module (`PoseSet.stray` is 0: a layer is handed
      its other poses by their places; the stage sees to it for an update outside a tick). A name the animator lacks, or a list of the
      rig's nodes or a table of its numbers longer than the module holds, is a mistake and throws by name, before a word of the rig is
      written (`layerFits`). So does everything else of a fixed length a layer is handed: a clip the module does not hold or one that
      writes a node its pose lacks (`clipNo`), a reload of more stations than `LAYER_KEYS_MAX` (where
      the layer is asked), a knife blow of more than 4 keys or a table of a throw outside 2 to 16 (`character.ts layerTablesFit`, read
      as an animator is made, so a table that grew is found as the game loads; `throwTables` names a swing's tables once, for the layer
      and for this). NO QUIET WAY ROUND ANY OF THEM: until 2026-11-04 each was a man or a layer the TypeScript posed instead (the rig
      and the clip counted under `walk`, the other four not counted at all). A layer that answers 0 was handed something the module
      does not hold all the same: it throws. `layer-ident`'s `whole` line holds the pose astray, the rig's table and the clips (each of
      the character file's is held; one of no tracks and one that writes a node past the pose are refused by name); `caps` holds the
      knife's and the throw's tables, the check's own edges and every gun's reload. A RECORD THAT PLAY MAKES IS NOT A TABLE: a teammate's
      hammer keeps every blow it landed until it is stowed (`bigkit.ts hammerOut`), over as many doors as he breaks, and a layer that
      took the whole record had room for 64. So the big tool's layer is handed what it reads, in both languages: the last blow landed
      and the next one coming (`character.ts blowsNear`), two at most whatever the record holds. `layer-ident`'s `blows` line holds it:
      on every call of the big tool's drills the module handed the whole record leaves the pose the two blows leave, and a hammer's
      record of 70 blows is one more drill of `both` (`BLOWS_BREAK=1`, the coming blow not handed over, is red). A layer that reads a record takes the part it reads, never the record.
    - THE CHECK: `bun run tools/qa/layer-ident.ts` (shipped stand; `rust-check`'s `layers` section runs it). `both` plays `warehouse-four:1`
      and then the drives a mission deals too little of: a quiet walk round a guard (80 s, crouch and close-in coming and going, with and without the knife and with the kit's close-in pose
      off for a part), four kicks,
      the reactions (Sam and a guard each ducked, started and hit from three sides, blinded, staggered, hurt, shaken, all at once; Sam tosses
      and walks hurt), a lean dealt to Sam's animator, the wounds (the guard laid down on a knee with a hand to his ribs, on his back with
      a hand to his thigh, and as a save restores him), a man kept down (Sam's hands behind his back, then on his head), the magazine
      changes (Sam's rifle and pistol each topped up, emptied with one in the chamber and dry; a guard dealt a short rifle, a marksman's
      rifle, a small gun, a belt gun and two shotguns in turn), what a free left hand does (every signal, an aside to each side; the two
      tells; a smoke brought up, half let down and put away; the goggles put down and up by hand, twice; a hand to the radio), four things
      the game deals to a state this mission never reaches, for which THE ANIMATOR'S OWN INPUT IS HELD at a value whatever the game writes
      to it meanwhile (`hold` in the check: the flashlight brought up in the gun hand, the lock worked, a body hauled three ways, a pistol
      guard changing what he carries six times), the look under a door (in, held and out, with a goggle nod as it comes in and another while
      it is held), the takedowns (each of the five blows and both knife blows, with and without a point to strike at and a reach under
      them; a man taken three ways), the ready pose (a strike and a bare-handed grab, with the knife's hilt and close in), the changes
      of kit (rifle, pistol and empty hands each way), a hand on a door (pushing with either hand or the one the layer picks, and
      reaching), hands at work (a code of one and of four keys, a charge, a lever each way, a thing taken, with the right hand free and
      not, and put away after), a man held from behind (Sam's layer and the guard's, each animator's input held as `player.ts` deals it:
      every phase and out of each (grab, held, choke, whip, kill), the arm and the gun and a change between them, with no knife, with
      `HOLD.catchShape` up (the yank, the step in, the man's place, the haul, the reach's hand brought up first), lowered, hauled, the
      kill and its beats shown and not, the knife stripped with and without a place said, a shot, an armed and a restless man), the big
      slot's tool (the record the game writes, held on Sam's animator and its clock run: the hammer fetched, three blows of a beat, a
      blow asked for before it is out, a blow past the record's end, the point near, far and not said; the shotgun and its shells;
      each with `snug` and without, standing and walking, his chest turned and pitched to an aim and not, with and without a rifle
      under it on his back), the throw (the full overhand, shorter ones, a hard throw two steps off, the toss at several sizes;
      standing, crouched and walking; a swing begun inside the last; the free hand's throw with a long gun kept in the other; a toss
      with a hand up on a gun; a blast that folds him mid-swing and a swing begun already folded; `THROW_BY` off), a run with its arms
      carried close (`RUNARMS` raised for the drive, the animator's ground speed HELD: up from a walk through the jog to a full sprint and
      back, a jog held, a sprint that comes and goes; nothing drawn, the pistol out, the rifle coming up into both arms and put away
      mid-run) and the wall-pose probe's two drives. Every call both ways from the same start: pose singles, bones, columns, answers and
      the scratch poses compared. A layer asked under 100 times fails the line. Measured 2026-11-04, every row on, one seed, 0 differ of:
      standPose 495,915 · lifeLayer 495,915 · mvPre 495,915 · handsPass 495,915 · gazeKeep 495,915 · turnWeigh 273,404 ·
      mvPostBody 109,340 · stealthLayer 79,577 · longBody 73,198 · longArms 73,044 · mvTrunk 72,665 · crouchPose 69,724 ·
      toesUp 69,377 · clutchArm 63,227 · actPose 62,301 · watchSlip 35,967 · watchLook 35,967 · stanceBody 24,791 · wallBody
      15,591 · wallArms 15,591 · carryArms 15,247 · sneakBody 15,144 · sneakArms 13,490 · runPlant 10,072 · holdLayer 10,044
      · heldLayer 9,972 · softKnees 8,208 · creepBody 8,162 · steadyLayer 5,924 · runArms 4,112 · creepArms 3,606 ·
      workLayer 3,595 · radioPose 3,391 · readyLayer 3,234 · bigLayer 2,764 · throwLayer 2,267 · reloadArm 1,608 · doorLayer
      1,601 · mvPostArms 1,092 · watchArm 1,070 · kitLayer 1,026 · swapPose 742 · lockpickLayer 716 · blowLegs 610 ·
      blowTrunk 610 · blowArms 610 · woundBody 600 · tellLayer 580 · knifePose 556 · haulLayer 540 · woundLimbs 534 ·
      boundArms 522 · signalLayer 450 · cablePose 446 · catchArm 317 · pistolReloadArm 297 · smokePose 271 · kickBody 240 ·
      takenLayer 236 · leanBody 234 · nvPose 216 · torchUpLayer 174.
      WHAT NO DRIVE REACHES (counted once with a counter on each branch, then taken out): a thing in the right fist during the close-in
      carriage and its wide pose's own pole; a magazine change with the gear rows off (the marks from the hip) and with the hip line off;
      in the hand pass, a layer's note smaller than 0.03; a smoke or a goggle flip with no twist on the head; a haul with no twist.
      For the nine newest layers (the takedowns, the ready pose, the kit change, the door hand, hands at work) the counters were read as
      each was moved and a drive written for what they showed unreached, but the list of what then stayed unreached was not kept: count
      again before trusting a branch of those that a drive above does not name.
      For the last five (the hold, the held man, the big tool, the throw) the counters named every input the drives above list, by phase
      and variant, and each was reached with the layer live and easing out; what they did NOT split, and so nobody counted: the hold's
      inner tests on its own clocks (the sway, a spring's rest), the big tool's point exactly on his pelvis, a chest pitched but not
      turned, and what was once a fall-back to the TypeScript (a hammer's record longer than the module's room; a throw table of more than 16 keys is refused as the game loads). `runArms`,
      counted the same way: the jog alone, the sprint's share part and whole, the run's share part and whole, each hand behind, through
      and past its lift, a long gun part of the way up; unreached, the two shoulder joints on one spot (no rig has them there). A review
      read two more that the counters had not split, each driven since: the disarm begun WITH the grab (the game begins every one
      there; the pistol hand's rap on his gun) and a blast's fold with `THROW_BY` off.
      `caps`: a size both languages write down (the most keys of a knife blow, blows of a beat, keys of a throw table; where the keys
      ride the bench, how many keys a path may be) is the same number in `character.ts` / `skeleton.ts` and in `layers.rs`, read from the
      sources; the crate itself refuses to build if the blows or the throw's tables at their caps would not fit the bench's own numbers.
      Its second line holds the game to them: the knife's blows and the throw's swings fit as they stand, a jab lengthened to one key
      past its layer is refused by name, and every row of the weapons table with a model of its own, at every count of rounds its gun
      holds and one more, makes no more stations than a path may be (34 of 48 at the most: the M1014, eight shells and the bolt run);
      `layerFits` refuses a length under its least, over its most, or that is no number; a door's blows by the kit's own numbers
      (`BIGSLOT.swings`, `.shells`) are within a beat's 64.
      `RUST_BREAK=1`: every knob handed a hundredth large AND the first of each layer's own numbers bent the same way: every layer differs
      somewhere, `rust-check` wants each to.
      THE MISSIONS, all of the above in: `rust-check --missions --rows poses,walk`, five runs, 38,634 ticks, the same game after every
      update with the rows on and off, under bun and under node (and every quick section green beside them). THE ROW OFF is the build
      before the layers to the bit, and the row on is too (`tickcost.ts --compare`, the five runs: 3.85 ms a tick before, 3.83 with the
      row off, 3.68 with `poses` and `walk` on, one round each on a shared box, so no more than "no slower seen"). WITH 61 OF THE 62 LAYERS
      IN (2026-11-04, the build to play before the run's arms joined it, `tickcost.ts --each --list --rounds 2`, the five runs, 30,071
      ticks, every run the same game to the bit): rows off 6.35 ms a tick, `poses` and `walk` on 6.40; a run by itself lies between 5 %
      under and 9 % over (compound-four 5.91 → 6.07, warehouse-four 6.47 → 6.60, pass-four 6.20 → 5.86, hostage-rescue 6.06 → 5.76,
      block-squads 7.95 → 8.63), which is the box's own noise. So the layers buy NO time while each ask crosses to the bench and back
      (the next item); the box was slower than at the first measure, so the two sittings do not compare. SNAPSHOTS with the
      rows up: `TPS_RUSTPOSES=1 TPS_RUSTWALK=1 bun run tools/qa/rollback-check.ts` on both stands (a layer's state is the animator's own
      fields, so this is what holds that no layer keeps anything a snapshot cannot reach).
    - WHAT THE ROW COSTS WHILE THE ANIMATOR'S STATE IS STILL TYPESCRIPT'S (all of it goes when `update` moves): a layer's columns cross to the
      bench and back on every ask, and some `put`s make a new array where the TypeScript writes in place; the knob list is written whole as
      every tick opens and at every update outside one; the bench's layout (where columns, the hand pass's columns and a path of keys
      stand, a key's width, each layer's own numbers) is written by hand on both sides, held only by `both` on the branches a drive
      reaches. `src/sim/built.ts` is 920 KB of text with every layer in (203 KB before the first; 741 KB with the first fifty-six), loaded
      and started by every page with the row off. It is the layers' own arithmetic (the module's code section; some 10 KB a layer), and
      no build setting takes it back: measured on a scratch copy, the module is 664 KB at opt-level 3, 633 at 2, 601 at s, 595 at z.
    - NOBODY COMPARED: `steadyLayer`'s rise (`MOVES.steady` = 2) is reached by no drive; the wall hold's arms were compared on whichever of
      their two forms (`WALLSNAP.pose` on or off) the shipped stand plays, not both. The layers were never seen on a GPU; a pose is
      compared as numbers, not looked at.
  - `dice` — the game's own die (`math/dice.ts SimDice`: the mulberry32 word every draw of the simulation comes off, and the count of draws) kept in the
    game's block of the module's memory and rolled there (`rust/sim/src/dice.rs`; `src/sim/dicehome.ts` is its TypeScript half, so `math/dice.ts` stays a
    leaf that names no module: `simRandom()` → the die's `home.roll()` → `dice_roll`). `?rustdice`, `TPS_RUSTDICE=1`. THE FIRST
    STATE OF THE GAME THAT LIVES IN THE MODULE (the bullet "State in the module is state of the game" is what it was built to prove). A PROOF, NOT A
    SPEED-UP, like `maths`: a draw from TypeScript then costs a call; but the die has to be in the module before any rule that rolls it is, and it puts
    real state through every road a snapshot takes. Either maths (it works out no length). It moves only a game that HAS a die of its own (the page
    always; the harness with `TPS_SEED`). `g.dice.s` / `.n` read and write whichever place holds them, so a checkpoint file is the same file with the
    row on or off. OFF EVERYWHERE.
  - THE SENSES, five rows, each a WHOLE QUESTION of the game's asked once where the TypeScript asked its lines one by one (`src/scene/collision.ts`, the
    block headed "THE SENSES IN THE MODULE"; `senseSet(bit)` is the one gate: inside a tick, the copy standing, the row on, `EXACT.on`, and the
    instance's line tests the class's own: a check that puts its own `segmentBlocked` over one instance gets ITS lines, from the TypeScript). Each works its
    lengths and angles out in the module, so each answers only where the game's own maths stand. Each returns "ask the TypeScript" for what it cannot
    judge (a length that is no length of this world's; until the pane's open question moved to the module, a line that met a pane), and the caller's own statements, kept whole as
    `…Here`, then answer. What they write to is SCRATCH of one question (the lamps, the blobs, the nav grid's shut cells, a way's waypoints): nothing of the
    game is kept in the module by any of them. The numbers the host shares with these files (doubles a lamp, the answers' codes) are `boxMirror.ts FACE`,
    under the Rust's own names; `rust-check` reads the Rust sources and is red where one differs.
    - `sight` — an eye on a man (`game/guards.ts eyeContact`: the cone in plan, the lens's elevation, then the line to his chest and, behind a blocked
      chest, to his head) from `sight.rs`. `?rustsight`, `TPS_RUSTSIGHT=1`. Only for an eye whose line passes glass (every eye's does today).
    - `light` — the lamps on a point (`guards.ts lightEstimateAt`) from `light.rs`. The host keeps the gates that are about WHOSE lamp it is (enabled, the
      owner skipped, steady or not), in their order, writes a row a lamp that passed and whose reach can hold the point, and asks once; the hour's sun and
      sky and a stroke of lightning are added to the same six faces in TypeScript afterwards, as they were (`hours.ts hourLight` keeps its own memory of
      rays, which is state of the host's). `?rustlight`, `TPS_RUSTLIGHT=1`.
    - `smoke` — the game's own smoke along a line (`game/sense.ts CpuSmoke.trans`, what `SENSE.cpu` sees through; now asked as `CpuSmoke.along(col, a, b)`)
      from `smoke.rs`: the live blobs are written for the one question and the sum, the erf and the wall test of each blob are the module's.
      `?rustsmoke`, `TPS_RUSTSMOKE=1`. Idle where the senses are the renderer's (nothing asks it).
    - `sound` — how a sound reaches a listener (`collision.ts SoundPaths.propagate`) from `hear.rs`, in two questions with the TypeScript's half-second
      memory of routed answers between them: the straight lines (clear, grazed, over the top, walled off, too far), then the way round by the floor (the
      search of `nav.rs`, each leg measured, the last waypoint the listener has a straight run to, a shut leaf across it). Sound stops at glass and
      passes a hole in it: the lines' own answer in the module (`glass.rs`); before that a line that met a pane sent the whole question back to TypeScript.
      `?rustsound`, `TPS_RUSTSOUND=1`.
    - `paths` — a way across the floor (`collision.ts NavGrid.findPath` with NO penalty: the A* over the half-metre grid, `nearestFree`, the greedy
      smoothing and `walkable`) from `nav.rs`. The grid of shut cells stays the host's (it bakes and re-bakes it) and is written to the module for each
      question, as it stands. The scores are KEPT in singles as the TypeScript keeps them in Float32Arrays (`single.rs`), and the search's tables stand
      blank between searches on both sides, so a search the module ran leaves the TypeScript's tables as it found them. A search under a penalty
      (`tactics.ts`) was TypeScript's until the senses' one-way-in car (below: the search waits for the caller's prices). `?rustpaths`,
      `TPS_RUSTPATHS=1`.
    OFF EVERYWHERE, all five senses. Measured 2026-11-03 on a 4-core cloud box under bun; never seen on a GPU, and in a browser only by the page check below.
    A QUESTION (`sense-ident --bench`, TypeScript → the module, µs): on the office sight 1.87 → 1.51, light 16.9 → 12.9, smoke 4.44 → 3.41, sound 164 →
    188, paths 108 → 121; on the compound sight 2.07 → 1.34, light 13.5 → 11.4, smoke 4.94 → 4.02, sound 82 → 92, paths 46 → 54. So `sound` and `paths`
    are SLOWER in the module (the search is a scan of the open list for its least score, which bun's compiler does as fast, and the grid is handed over
    each question): they are there because the server's model is to be one model, not for speed, and stay off as proofs, like `maths`, until what calls
    them moves. THE TICK (`tickcost.ts --each --list --rounds 2`, five missions, one seed each, 38,634 ticks, the same bits in every table): rows off
    3.53 and 3.43 ms (two tables; another job ran beside the first), the five senses up alone 3.41 ms, which is nothing, and with `lines`, `circles` and
    `fog` up too 3.29 ms, 4 % under the quiet table. A mission asks the module some 400 thousand eyes, 170 thousand lights, up to 60 thousand lines of
    smoke, 1,500 sounds and 350 ways. The five missions are the same game to the bit with all ten rows up and with none, under bun and under node;
    `--both` answered 16.6 million questions twice with none differing; on the page (`rust-page.cjs systems`, headless Chromium, frames not drawn) a
    mission's first 240 frames with these rows raised are the game in TypeScript by digest and by every frame's bits (the smoke row is idle there: the
    page's senses are the renderer's). Nobody has played with one of these rows up.
  - `doors` — THE FIRST RULE OF THE THINGS: the doors' step (`game/doors.ts Doors.update` and `Door.place` at its end: what was written onto each leaf
    since the last tick, the men and bodies in its swing, its swing, stops, closer, latch and lock's clocks, where its box stands) from
    `rust/sim/src/doors.rs`, statement for statement. `?rustdoors`, `TPS_RUSTDOORS=1`. The leaves' own numbers are COLUMNS (`src/sim/doorshome.ts DC`,
    48 of them, a door an index), a part of the game's block from the moment the doors are made (bit 4; "State in the module"), on either maths;
    the STEP runs in the module only inside a tick, where the game's own maths stand, the box world's copy stands with the game's own leaves in it, and the
    Game's two floor functions are still the Game's (`Doors.there`). Anywhere else the TypeScript steps the same columns where they are (`updateHere`): a
    tick whose box world changed shape before the doors' turn (a static added: `wroteGrid`), the frozen stand, a level of more than 64 doors (those keep
    their columns in TypeScript altogether). What the step needs of the rest of the game is an ARGUMENT, written for the one call (`Doors.handOver`: the
    level's facts of each door, each contact, what `Doors.man` answers of each living contact and of the hand last on a leaf that was written to, the
    switches, the world's edge), and what it answers comes back the same way (`takeBack`); the two floor questions (`standable`, `settleSpot` with its
    `bodyR`) are asked of the statics in the module. It writes each leaf's ROW of the box world itself (next bullet but one). OFF EVERYWHERE. The door
    VERBS (`use`, `pull`, `pullTo`, `crack`, `pick`, `kickIn`, `rattle`) are still TypeScript, writing the columns through `Door`'s accessors: they move
    with what calls them (the door tools, the men).
  - `falls` — WHAT FALLS AND WHAT IT LEAVES (`src/sim/fallshome.ts FallStore`, one a game: `game.falls`): the landings the simulation has scheduled for
    things that let go (when and where each reaches the floor, whether the floor hears it, the glass it leaves) and the LITTER map (m² of
    glass in each half-metre cell of the world's floor, which makes a footfall louder and is ground down by it) from `rust/sim/src/falls.rs`: six small
    rules (`schedule`, `deposit`, `due`, `step`, `heard`, `clear`), the store 6,790 doubles of the block (bit 8; a landing is twelve columns, the last the dust it raises). The landings wait in
    LISTS, one a kind of thing (`FALL_GLASS`: fragments of a pane; `FALL_PIECES`: the pieces a broken thing throws, `debris.ts` under `DEBRIS.dress`),
    each with its own count and its own memory of when the floor was last told, so one kind's cap, gap or going inert never moves another's; a new kind
    of falling thing is a new list, not a new store. `?rustfalls`,
    `TPS_RUSTFALLS=1`. It works out no length and no angle, so it answers under either maths and on both stands. It is the GAMEPLAY half of broken
    glass, cut out of `game/shards.ts` (whose plates and spray are dressing under `SHARDS.byFall`, `docs/agents/destruction.md`): `shards.ts` calls
    the store, and the store takes each rule there, here, or both ways (`FALLS.both`). The litter was a `Map` keyed by cell; it is a grid over the
    world's floor now, with the row on or off (glass laid off the slab is not kept: nobody can walk there), which the shards probes hold to the build
    before by digest. A call a footfall on glass and a call a tick with landings waiting: A PROOF AND A FOUNDATION, not a speed-up — it is here so that
    the breaks, when they move, lay their glass and schedule their landings with no crossing. OFF EVERYWHERE.
  - `props` — THE FURNITURE MEN SHOVE (`game/props.ts PropSystem.update`: every chair, carton, bin and plant coasting on its friction, set going by the
    men who walk into it and holding back the man it cannot yield to, knocking its neighbours, swept by a swinging leaf, guided through a door frame,
    stopped by the statics, and the scrapes, knocks and castor rolls of all that) from `rust/sim/src/props.rs`, statement for statement. `?rustprops`,
    `TPS_RUSTPROPS=1`. A thing's own numbers are COLUMNS laid for the cache (`src/sim/propshome.ts PropStore`,
    `props.rs Props`: see the data layout below), a part of the game's block from the moment the furniture is made (bit 32), on either maths; the STEP runs in the module only inside a tick, where the
    game's own maths stand and the box world's copy stands with the game's own leaves in it (`PropSystem.there`); anywhere else the TypeScript steps
    the same columns (`updateHere`). Handed in for the call (`handOver`): the level's facts of each thing by row, the men, which of the box world's leaves may sweep (a leaf off its hinges is
    none), the door frames, the switches (`CASTOR`, the friction, the system frozen). Back (`takeBack`): where the furniture leaves each man, and the
    noises in order. IT WRITES NO ROW OF THE BOX WORLD: furniture is not in it (nothing sees, hears or paths by a chair), so the boxes the picture
    draws are placed from the columns by `Prop.place`, after the step, both ways; the statics and the leaves are READ from the copy
    (`collide.rs gather`, list 0 of the leaves). A level of more than 128 things keeps them in TypeScript. OFF EVERYWHERE.
    Measured 2026-11-03 on a 4-core cloud box under bun, alone on the box; never seen on a GPU, and in a browser only by the page check (`rust-page.cjs
    systems`). THE STEP ALONE (the office, 61 things, 12 men walking among them, 30,000 updates, µs an update): the build before 56.9; these columns
    with the row off 59.1 (+4 %: the TypeScript step reads a column by row where it read a field; with nothing moving 22.6 → 21.6); the row on 27.3.
    THE RAY with its row off 2.17 → 2.12 µs. THE TICK (`tickcost.ts --each --list --rounds 2`, five missions, one seed each, 38,634 ticks, the same
    bits in every table): the build before 3.52 ms, this with every row off 3.51, with `props`, `lines` and `circles` up 3.34 (−5 %; a mission's p99
    10 to 23 % lower). The five missions are the same game to the bit with all fourteen rows up and with none, under bun and under node (26,583
    ticks); `--both --rows props,doors,lines` (the rays were the lines' row's then: `rays` is in the list now) answered 14.2 million questions twice (646,786 rays, the furniture's step on every one of 38,634 ticks)
    with none differing; `rollback-check` whole on both stands with the rows up. WHAT IT COSTS A SNAPSHOT, not measured as time: with the row off the
    columns are ONE array of 13,328 bytes and the clock in its head is written every tick, so a snapshot after every tick copies it whole where it
    copied the fields that changed; with the row on they are in the game's block, which is shared a 4 KB piece at a time. (ONE array is the store's
    field, the words a view asked of it, `src/sim/colstore.ts wordsOf`: two views of one buffer kept as two fields are the same bytes copied twice a
    snapshot, since the walk takes every typed array it meets.) Nobody has played with this row up.
  - `slabs` — A LEAF OFF ITS HINGES, IN THE AIR (`game/doorblast.ts stepSlab`: the loose slab a blast, a charge or a boot at a run sends out of its frame,
    flying, dragging its foot, toppling, stopped dead by what its rays find, pushed out of what its toppling reaches, slamming flat, sliding out and
    coming to rest clear of every static) from `rust/sim/src/slabs.rs`, statement for statement. `?rustslabs`, `TPS_RUSTSLABS=1`. A slab's numbers are
    A ROW of the slabs' columns (`src/sim/slabshome.ts SlabStore`, `slabs.rs Slabs`; a part of the game's block from the moment the doors are made, bit 64):
    152 bytes a row, the eleven doubles a slab in the air reads every substep first, six cold ones, four words. `Door.slab` READS THE ROW AS IT STANDS
    (`doors.ts SlabRow`, read-only by its type: one held across a step is the slab after the step) and is set whole. ONE SLAB A CALL: the host steps them in the order of their
    doors and poses each one's box before the next is stepped, as it always did, so a slab's rays meet the ones before it where this tick left them;
    the module writes no row of the box world. ITS OWN BOX IS NO WALL TO IT: `lines.rs ray_but` leaves one leaf of the list out (the TypeScript
    takes the box out of the list for the step, `withoutLeaf`). The checkpoint's run-on of a slab still in the air (`slabSnapshot`) is the
    TypeScript's, on a copy, whatever the row says. OFF EVERYWHERE.
    Measured 2026-11-03 on a 4-core cloud box under bun; never seen on a GPU. `slab-ident`: 24,000 steps over six levels, two of them with
    panes a ray may pass (1,576 leaves torn out, 450 of them falling back through their doorway, 476 thrown at a pane, 498 begun inside a wall,
    180 at the six-second cap; 1,358 steps handed back for a pane), 30,818 steps of a slab in the module, 2.36
    million rays, none differing; `SLABS_BREAK=1` red. NO RUN OF THE FIVE MISSIONS TAKES A LEAF OFF, so `rust-check`'s set has a sixth,
    `warehouse-four:1@frag` (`@frag` = `PT_FRAG=1`: 'frag and clear' ordered at a held room's door): with all fifteen rows up it and
    `compound-four:1@frag` are the game with none to the bit (13,110 ticks; 61 and 43 steps of a slab in the module, one slam each), and `--both`
    answered their 101 slab steps both ways with none differing. WITH THE ROW OFF IT IS THE BUILD AS IT WAS, AND A MISSION'S BITS DO NOT SAY SO
    (`digest.ts rawBits` folds a door's angle and nothing of its slab): `tools/qa/slab-asitwas.ts` flies 900 torn leaves over four levels by the
    game's own entry and folds every number of every slab, its boxes, the noises, the tallies, the log and what a checkpoint would file after each
    tick, 5.5 million numbers, against the words the tree before the columns folded; `rust-check`'s slabs section runs it. A WORD THAT FOLDS A LEVEL'S
    NUMBERS NAMES THE LEVEL'S BYTES: the check reads each level by the git blob id of its file at the recording, so a level dressed since leaves
    it alone (it went red on the first build that dressed the compound, with nothing of the slabs changed). A SLAB'S STEP WITH ITS
    POSE (the office, every wooden leaf torn out from both faces, 182,700 steps a side): 162 µs in TypeScript, 67 µs in the module. A tick has a
    slab in the air for a second or two after a blast and at no other time, so this row is a rule moved, not a tick made shorter. Nobody has
    played with this row up.
  - `items` — WHAT IS THROWN OR DROPPED (`game/throwables.ts Throwables.update`: a can, a grenade, a magazine, a gun out of a dying hand, ageing,
    falling, meeting walls, leaves and furniture by a ray from its middle, bouncing, handing a chair a share of what it lost, coming to rest, and
    dropping off a thing that moved from under it) from `rust/sim/src/items.rs`, statement for statement. `?rustitems`, `TPS_RUSTITEMS=1`. AN ITEM
    IS A HANDLE ON A ROW (`throwables.ts Item`, `src/sim/itemshome.ts ItemStore`, `items.rs Items`; a part of the game's block from the moment the pool is made,
    bit 128): 184 bytes a row, the twelve doubles read for every row on every tick first, then what a bounce reads and writes, then the fuse, the
    venting and what a magazine holds (other rules' own, carried), six words. `it.pos`, `it.fuse`, `it.settled` read and write the row whoever
    asks; `pos` and `vel` are given as new arrays and taken whole. THE ROWS ARE THE GAME'S LIST IN ITS ORDER (the order is the rule's: a full
    pool gives up the first spent thing, the boxes are listed in it, the step walks it from the newest down), and the list stays what it always
    was, a plain array any rule may splice: the pool keeps the list its rows were last laid in and lays them anew where it has changed
    (`Throwables.sync`), and an item out of the list keeps its numbers as they were. ONE CALL WALKS THE POOL; what a thing does to anything else
    (a bounce said by its callback, a share of momentum for a thing of furniture, its time up) is LISTED and done by the host when the whole
    pool has been stepped, in both languages: a callback is handed what the item was at the bounce, and the item as the step left it. A ROW
    THAT BOUNCES ON GROUND THAT MAY BE LYING SNOW IS HANDED BACK (the snow's map is the host's; a ray that ended on a pane was too, until the
    pane's open question moved to the module): the module writes and lists nothing of that row and the TypeScript steps it. A POOL PAST THIRTY-TWO ROWS (every one a can on
    its fuse: never, in play) grows wider columns and is stepped in TypeScript from there on. THE FURNITURE'S RAY came with it
    (`props.ts PropSystem.raycast` | `lines.rs ray_things`, over the box world's copy of the furniture's parts: the bullet "The box world's copy").
    OFF EVERYWHERE. What it taught: "WHAT THE THROWN THINGS ADDED" below.
    Measured 2026-11-03 on a 4-core cloud box under bun, one seed a run; nobody has played with this row up; never seen on a GPU. `item-ident`:
    28,000 steps over seven levels (one with panes a ray may pass, one under snow, one with the pool driven past thirty-two rows), 3,437 things
    thrown, dropped or let go, 115,326 rows stepped in the module and 390 handed back (506 rays ended on a pane, 1,649 bounces asked of the snow),
    8,824 bounces, 1,766 shares handed to furniture, 494 landings on a thing and 381 drops off one, 1,953 writes of the pool from inside a
    callback, 168,000 furniture rays asked both ways: none differing; `ITEMS_BREAK=1` red. THE MISSIONS: the five and the two `@frag` runs with
    all sixteen rows up are the game with none to the bit under bun and under node (39,693 ticks; the module walked 258,928 rows of thrown
    things, 178 things said or done after a walk, 67 come to rest, 24 rows handed back, all in the one snow level), and `--both` answered 58,369
    walks of the pool both ways with none differing and no stale row of the furniture's copy. WITH THE ROW OFF the five missions are the build
    before it to the bit (the handle and the pool are in force either way: 3.55 → 3.48 ms a tick, inside the noise); with it on, against the
    same build under `TPS_EXACT=1`, 3.50 → 3.53 ms, to the bit. ONE WALK ALONE: a pool of six in the air 12 µs in the module, 26 µs in
    TypeScript. WHAT THE HANDLE COSTS WITH THE ROW OFF (thirty-two lying still, the quietest of four runs): the step 1.6 µs where the build
    before took 0.9, the boxes' listing 14.7 where it took 12.3, the fuses' pass 3.0 where it took 1.1; a field read is a column read now,
    about 5 µs a tick with a full pool, on a tick of 3.5 ms. The rollback check passes on both stands with this row, the slabs', the furniture's, the
    doors', the falls' and the lines' up. THE PAGE (headless Chromium on the software adapter, `tools/qa/rust-page.cjs`): 240 frames with every
    row up, this one among them, are the frames with none, to the bit. THE PROBES (26 families of the doors, the kit, the blasts, the devices
    and the squad's throws, on both stands with the rows off, and ten of them again on the shipped stand with this row, the slabs', the
    furniture's, the lines' and the circles' up): no red that the commit before does not have with the same digest. NOT MEASURED: a real page on a GPU, a second seed, more than one player.
- **Posing: what is left, and it is most of it.** Posing is gameplay, whole (the bullet "Gameplay and dressing"), and about a sixth of the tick. The time is not in the
  functions above but in the layers that call them, and in the arrays they allocate on the way. Those layers move to Rust only WITH THEIR DATA. The
  poses are there now (the `poses` row: a part of `State`, a pool, and nothing in `game/rollback.ts` changed for it); the run-forward bones and each
  layer's own numbers (its springs, its clocks, its targets) go into the block as the layer that owns them moves, as columns of a character. Then the
  layers, one at a time (the walk and the trunk; the arms and the hands), each held by `tools/qa/pose-ident.ts` and the missions as the functions
  are now. A layer written in Rust is handed the slot and where each pose starts, and asks `skeleton.rs` with no crossing.
- **A clip is sampled in the module, and a sample finds its place once** (`rust/sim/src/clips.rs`, `skeleton.rs ask_sample`, `anim/skeleton.ts sampleClip`,
  `ClipSide`; under the `poses` row). THE CLIPS are authored data, not state: a character file's animations are copied into the module once, each as the
  host first samples it (`clip_begin` / `clip_track` / `clip_end`; the host writes a track's times and values where `clip_track` says), outside the
  games' blocks, with no part and no word of the layout: a COPY (`tools/sim/build.ts STATICS`). A clip the table has no room for, or that is not one it
  takes (no track, a track with no key, values that are not its keys', a node no pose has), is refused whole and sampled in TypeScript, the same pose.
  A CLIP WRITTEN TWICE IS ONE CLIP: the host knows a clip by its object (`ClipSide`, a memo in a side table over the character file: gone with the
  clip, in no snapshot) and a process may load the file again (a game stood up a second time), so `clip_end` finds a clip already held with the same
  length, tracks and singles (by a folded word, then compared whole) and gives its number, keeping nothing of the second writing. The table grows with
  the clips there are (the rig's 46 take 310,527 of its 1,048,576 singles), not with how often they are loaded. To be found, a clip has to be written
  again, so the table ALWAYS KEEPS ROOM FOR ITS LARGEST CLIP: a new clip that would leave less is refused, and a full table still knows what it holds.
  A clip left half written (a throw between `clip_begin` and `clip_end`) is dropped by the next `clip_begin`. A CLIP IS NEVER WRITTEN AFTER IT IS LOADED
  (`anim/gltf.ts Clip` says so): the memo and the copy are made once and nothing compares them again while a game runs.
  `pose_sample(g, at, nodes, clip, t, looped)` writes straight into the pose where it lives (`g = -1`: the bench); `ask_sample` is the `pub fn` a layer
  written in Rust calls with no crossing. There is NO CLIP PLAYER in `skeleton.ts`: which clip plays and its clocks are fields of the animator
  (`game/character.ts`) and go into a character's columns with the layer that owns them. ONCE A LINE, in BOTH languages: where a time falls among a
  track's keys depends on the track's times alone, and a clip's tracks share theirs (the rig: 7,314 tracks, up to 159 a clip, on at most TWO sets of
  times a clip), so every track is given the number of its LINE (the first track of its clip with the same times, single for single) and a sample
  searches each line once where it searched every track. The same place, so the same pose: this changed the TypeScript the game plays on today, with
  every row off. Past eight lines a clip, a track is searched by itself. WHAT HOLDS IT: `pose-ident.ts` asks every sample THREE ways (the TypeScript,
  the module, and `plain`, the definition kept in the check: every track searched), at random times, on a key's own time, past either end, looped and
  held, the pose on the bench and at home; samples every clip at every key's own time (1,649), which reads the copy back single for single; samples four
  clips the module must refuse and four made there for what the rig never has (twelve sets of times, sets that differ in one single, the same values
  written twice, one key among many); writes every clip a second time as new objects (the same 46 numbers), one a single off a held clip (another
  number) and one that keys a node past the pose (taken by the module, sampled by the host); leaves a clip half written a hundred times; then fills the
  table each of the three ways it fills (clips of 150 tracks until the tracks run out, long ones until the singles do, short ones until the clips do:
  the rest sampled in TypeScript, and one of five thousand tracks turned away by `clip_begin`) and after each filling writes every clip AGAIN (the
  same numbers still, 138 of 138) and asks a held clip of the module. It counts the differing answers by kind (`sample`, `lines`, `blend`, `run`,
  `turn`, `after` = the run made from a turn, `ik`; a label of no kind throws) and the calls AT THE MODULE'S DOOR by export: in an at-home round each
  export is asked of a pose where it lives an exact number of times (`ROUND`), and a count that is off is red. Under `RUST_BREAK=1` it is exit 3
  when a kind stays green, and each kind has a break of its own: the rig's wrong parent for the whole run (and the solve), a hair on a turn's first
  number and on a solve's target, on a blend's weight, on a sample's time, and the plain sampler asked a hair off; every question then starts from
  the TypeScript's pose and run, so nothing is red by what an earlier answer left. Seen to bite, one temporary edit each: lines told apart by their
  length alone (492 of 13,253 questions, in either language), the places not cleared between samples (4,149), the ninth line given the first's place
  (167), no clip ever found again (0 of 46 written twice are one), the singles left out of the comparison (the near clip taken for the held one, 428
  samples differ), no room kept for tracks (0 of 138 known once full), a half-written clip not dropped (nothing taken after it), a turn at home sent
  by the bench (`pose_turn 500 of 750`), the rig's break left out under the break word (exit 3: `NOT RED FOR run`), a turn's hair left out (76
  turns differ where 11,997 did: only the wrong parent's own rotation still shows it). MEASURED (bun, this box, the game's rig, a pose at home): a sample of each of the 46 clips in
  turn 2,798 ns before, 1,916 ns in TypeScript now and 1,592 ns in the module; the walk clip 1,644, 1,407 and 1,162 ns. A tick samples some fifteen, so
  this is under one per cent of a tick either way, below what `tickcost.ts` can tell from noise: NOT measured on a tick. It is groundwork: the layers
  that sample (the walk, the arms) can now do so from Rust.
- **The box world's copy** (`src/scene/boxMirror.ts`, `rust/sim/src/world.rs`). The rules that move a box (doors, blasts, cuts, glass, props) are still
  TypeScript, so a `Box` object is the truth and the module holds a COPY: sixteen doubles a box, its flags, its cut planes, the grid's cells. It is not state
  of the game: nothing is snapshotted, and a copy thrown away is made again from the objects. What makes it safe is that it is PROVEN, not trusted:
  - the module is asked only INSIDE A TICK (`Game.stepSeats` → `col.tickOpen()` / `tickShut()`). As a tick opens every static is compared value for value
    with its row and rewritten where it differs, so whatever happened between ticks (a level edit, a load, a snapshot put back: `PUT_BACKS` forces all of
    it) costs nothing to get right. Outside a tick (the page's draw, a tool) the TypeScript answers.
  - A RULE THAT WRITES A STATIC INSIDE A TICK SAYS SO: `col.wrote()` (its shape, flags or cut; a pane a walker now passes). The next question then compares
    the statics again first. `addStatic` / `truncateStatics` say it themselves (the grid changed: TypeScript answers until the next tick). Leaves: the
    three places that write a leaf say so themselves (`boxMirror.ts leafWrote`, called by `Door.place`, and so `Door.reset`, and by `doorblast.ts
    poseSlab`), whoever calls them and when; a list a question swaps in for itself is copied at every question.
    THE GLASS IS SAID BY ITS WORK, NOT BY ITS WRITES (`col.glassBegin()` / `glassEnd()` round a blow, a blast and the whole of the glass's step):
    the rule that breaks a pane asks lines and rays of its own between its writes (a fragment's way out), so from begin to end
    every question writes the panes' rows again first, and so does the first one after. A mark at each write was tried first and was wrong
    within a day's check: a question in the middle paid the debt and the writes after it went unsaid.
    Today's writers of statics: `destruction.ts`, `furnblast.ts`, `wallbreak.ts`, `winbars.ts`, `glassbreak.ts`. THIS IS THE ONE PART THAT RESTS ON EVERY
    WRITER REMEMBERING, and it ends when those rules are Rust and write the rows themselves. Until then `rust-check --both` is the net: it found the three
    writers nobody had listed (door leaves, a flag weather writes, the breakers' hides).
  - the copy never outlives what it was proven against: the tick shuts it however the step ends (a `finally` in `Game.stepSeats`), and a snapshot put back
    shuts every game's copy there and then (`rollback.ts restoreSnapshot` → `mirrorsShut`: a checkpoint loads inside the step's last lines).
  - the copy keeps the flags a question reads (`MIRRORED`: NoShadow, Dynamic, and bits 16 up); a question asked with any other flag goes to TypeScript, as
    does a level too big for the tables (8,192 statics, 256 leaves a list, 16,384 cells). Four games' worlds are held at once; a fifth takes the one stepped
    longest ago.
  - THE FURNITURE'S PARTS ARE A THIRD LIST OF IT (2026-11-03, with the thrown things; `boxMirror.ts things`, `world.rs THING_SET`, `thing`,
    `thing_live`). Furniture is not in the box world (a ray meets it through `PropSystem.raycast`, a man through the furniture's own step), so a
    rule in the module that must meet it reads a copy: every part box of every thing in the level, a thing's parts together and the things in the
    furniture's own order, as a list of the leaves' kind (at most 256 parts: the largest level has 220), with where each thing stands and the
    radius of the cylinder that holds it, by its id. `Prop.place` is the one place a part is written and says so (`thingWrote`; so do a thing
    taken out of the level and the level put back); the copy is made at the tick's first question that needs it and again after a write, ROW BY
    ROW: a part is compared with its row and written only where it differs, so a chair shoved across a room costs its own boxes. `lines.rs
    ray_things` is `PropSystem.raycast` over it, the cylinder's test and all; under `rust-check --both` every furniture ray of a mission is asked
    both ways and the copy compared with the furniture (`collision.ts thingsBoth`). No TypeScript caller is sent to the module for this ray: the
    walk of the thrown things (`items.rs`) is its one user, and the guns' rounds will be the next.
  Checks: `tools/qa/segment-ident.ts --rust` (the `lines, circles` line of `rust-check`: 40,000 segments and 10,000 circles a level over four levels with
  leaves swung, walls broken and re-shaped, against the TypeScript walk as it is and as it first was; `RUST_BREAK=1` leaves a re-shape unsaid and is red);
  `rust-check --missions` (rows on against off, raw bits after every tick) and `--both` (every question the module is sent is answered by TypeScript too,
  and every row of the copy is compared with its box at every question: `collision.ts BOTH`).
  Run 2026-11-03, five squad missions, one seed each, every row up: 38,634 ticks the same raw bits after every update under bun and under node;
  `--both` 14,283,676 questions (385,530 of them the fog's), none different, no stale row; `RUST_BREAK=1` red on both; `tools/qa/dress-check.ts` green
  with the rows up; `tools/qa/rust-page.cjs` (Chromium, the software adapter, frames not drawn): 240 frames of a squad mission with lines, circles, fog
  and poses up are the TypeScript game by digest and raw bits. NOBODY HAS SEEN any of it on a GPU or played it: the rows are off wherever a person
  plays, and the frozen stand runs `lines` and `circles` only (the other two need the game's own maths). Probe families with the rows up: destruction,
  furnblast, breaks, winbars, wallbreak and wallcharge were run both ways before the last review's mends (3.3 million questions, none different); the
  door, glass and frag families were not.
- **A pane's open question** (`rust/sim/src/glass.rs`; the copy's panes, `boxMirror.ts panes` / `PaneSource`; 2026-11-04). Whether a pane of glass
  is open where a line or a ray meets it (`glassbreak.ts openAt(bi, X) || also(bi, X)`, what `col.glassOpen` answers) is answered IN THE MODULE.
  Before it, every rule there that met a pane stopped and handed the question back: the ray was walked again in TypeScript (6,500 rays in twelve
  mission runs), a slab's or a thrown thing's step was handed back whole, a sound's line that met glass sent the whole question back, and a line
  listed its panes for the host to judge. One move ended all of them, where a loop in the host that asks again with the pane left out would have
  ended one. It is no row of `RUST`: the lines' and the ray's rows answer by it, and the TypeScript rule is its twin until the twins go.
  - THE STATE IS STILL TYPESCRIPT'S. Breaking glass (`glassbreak.ts`) owns each pane's cracks and pieces. The copy holds a ROW A PANE THAT IS NO
    LONGER WHOLE (a piece out, a piece queued to leave, toughened glass coming down, gone): eighty doubles (nine a site: where it is, the bearing of
    radial 0, the four base radii, the ring zone, the merged share; then the dice's blow, half sides, and the key and number of the last clump out) and
    sixty-four words (the static, flags and site count, the dice's lattice and seed, each site's radials and seed, a bit a piece that is out, a bit
    a piece queued to leave, a bit a site whose crater is). A pane with cracks and nothing out answers as a whole one and has no row. The rows are
    written WHOLE every time the copy is proven: a row is a few hundred numbers, a level's broken panes are few, and nobody has to say which pane
    changed (a put-back and a pane made whole are no case). 256 rows; past that `paneAsk` is −1 and the TypeScript answers what may meet a pane
    (counted: `fell`, with the reason).
  - THE RULE AND ITS ROWS ARE HUNG TOGETHER, in one call (`col.glassRule(rule, rows)`; `glassbreak.ts` hangs itself as both). A rule hung alone
    (`col.glassOpen = …`: a check's made-up closure, a mode's own) has no rows, and the TypeScript answers by it (`paneAsk` −1): the module
    never judges by rows that were another rule's. The collision keeps the rows' source and hands it to the copy each time it is stood up, as it
    hands the walk; a copy that is shut holds nothing of the game (a world outlives its game). The source is hung as a FUNCTION, like every hook
    the game hangs on the collision: an object kept there was a second road to the game's glass, and the restart check, which names what may
    differ by its road from the game, went red on it.
  - A DICING PANE NEEDS NO BIT A CLUMP. The clumps of toughened glass let go in one order (their keys, a tie by clump number: the list
    `glassbreak.ts` sorted once), so "this clump is out" is "(its key, its number) is no later than the last one out": the row holds that one
    pair and the module works a clump's key out again (`gpDiceKey`, the same arithmetic). The row is written from the clumps that ARE out, not
    from the step's counter: a clump's release is heard by the game before the counter moves, and a question asked from inside that hearing (a
    shard's ray, a sound's line) must find the clump out. A review found the first version one clump late there; `glass-ident` now asks from
    inside every release.
  - A BURST'S VIEW IS DATA. A blast judges its lines by glass as it is to a burst (`frag.ts throughGlass`), which was a closure hung on the glass
    for the length of one judgment. `GlassBreak.seeThrough(view, from, size, fn)` now says it twice: the closure for this side's walks, and rows
    for the module's (`boxMirror.ts PaneView`). The rows are made WHEN THE MODULE IS FIRST ASKED under the view (`GlassBreak.view`, kept for the
    judgment's length), so a view under which nothing is asked of the module costs nothing. `blown` is answered there from the pieces queued to
    leave; for `wary` and `counted` the rule that would break the glass is worked out for every sheet of the level at once and handed over as
    "this pane gives whole" or "it gives within this far of its nearest point". That rule is stated ONCE (`glassbreak.ts give`): `wouldGive`
    asks it of a point and the view's rows are its answers for a pane, so the two cannot drift. THE PRICE, measured by review on the office (22
    sheets) before the rows were made from the level's glass alone and not from every box: a burst's line by a pane 10 µs wary and 8 µs
    counted, against 0.9 µs blown, because a view's rows are worked out for every sheet where the closure asked only the pane the line met. A
    man scoring spots to flee to asks one view a spot. If that shows in a tick, keep the rows for the same burst across the questions of a tick.
  - THE GLASS IS SAID BY ITS WORK (`col.glassBegin()` / `glassEnd()`; the bullet "The box world's copy"). The copy's debt names WHAT is owed
    (the boxes, the panes, the view), so glass at work has only the panes written again before each question and a view only its own rows; the
    whole of glass's step is one piece of work, and a question from inside it finds the rows as the glass stands at that statement. A write
    with no question of its own between (a pane made another glass, every pane made whole) says `col.glassWrote()`. THE PRICE, measured by
    review on the office with 15 broken panes: a solid line asked while the glass is worked on is 2.3 µs against 0.53, and once a pane is
    touched every tick writes the rows once more (the glass's step is bracketed whole). A count bumped at each write of the glass, as the
    leaves and the furniture have, would write nothing when nothing changed, but rests on every write site being found, which the bracket does
    not; the tick's cost did not move (below), so the bracket stands until the glass's rule is Rust and writes the rows itself.
  - IT NEEDS THE GAME'S OWN MATHS WHERE AN ANGLE OR A LENGTH DECIDES. A piece is found by an angle and a clump by a length, so there the answer is
    the TypeScript's only where both sides have the same `atan2`, `cos`, `sin` and `hypot`. `paneAsk` is 1 under `EXACT.on`, and under the
    engine's maths while no row holds a site or a dice and no view gives by a length (`glassPlain`: panes gone and nothing else). A sight line
    that passes glass by its flag never meets the question and is the module's under either maths. Otherwise, under the engine's maths, the
    pane's question is IDLE and the TypeScript answers what may meet a pane (`paneAsk` −2). That is the rule of the rows that work a length or
    an angle out (`sim.ts OWN_MATHS`), kept at the question because the rows that ask it, `lines` and `rays`, answer boxes under either maths;
    an idle question's reach of the TypeScript is NOT counted as a fall (`paneFell` counts −1 alone: no rows behind the rule, or more than the
    tables hold), so the sealed rows stay sealed on a stand whose maths are the engine's.
  - MEASURED 2026-11-04 on a 4-core cloud box under bun, against the tree before it: the twelve mission runs of both stands (`playthrough.ts
    --seeds 1-2`, `PT_DIGEST=1`) the same by digest, 42 of 42 on each; with `TPS_FELL=1` the count "rays · the nearest thing is a pane"
    (10,638 over the shipped stand's runs) is gone and no line names a pane. The tick (`tickcost.ts --each --list --rounds 2`, five missions,
    30,071 ticks): 2.11 → 2.11 ms mean, every run to the bit; the same tree measured twice differs by 1 %. Never seen on a GPU; the page was
    driven in headless Chromium on the software adapter (`rust-page.cjs`).
  - A RULE'S STEP ASKS ONCE (`col.rulesPanes()`): a slab's and a thrown thing's rays ask the pane's rule, so their step takes the answer
    (0 or 1) from one place, which also counts a −1 as the TypeScript's under the rules' own row. The thrown things ask again before each walk:
    a row this side stepped in between may have broken more glass than the copy holds.
  - WHAT HOLDS IT: `tools/qa/glass-ident.ts` (`GLASS_BREAK=1` must fail it; the section `glass` of `rust-check`, which runs it ON BOTH STANDS:
    toughened glass comes down in a wave on the frozen stand and drops after a beat on the page's, and the two reach different rows). The glass
    of four levels, and one block of solid glass a level, broken by the game's own entry points (rounds hard and soft, pellets, blasts, the
    glass's step, a pane made another glass, every pane made whole); points on and off every pane asked of the rule and of the module with no
    view and under each of the three; a solid line and a ray through each asked of both walks; the same asked FROM INSIDE A RELEASE (a piece
    leaving, a clump leaving, the game hearing either); the copy's rows compared with the glass after every round. Fair only if it met float
    glass holed and with pieces queued, cracks run, a pane part way down, panes gone, a web, a pane with all eight sites, open and shut under
    every view, glass on its way out, a pane that gives whole and one that gives near, solid glass, and a question from inside each kind of
    release. The tool takes its stand from `TPS_PROFILE` given OUTSIDE it (a default written in the tool's first line is read after the harness
    has stood up, and gives a half stand: the pins lifted, the page's switches not written). `segment-ident --rust` breaks real glass too (it
    runs under the game's own maths for that), and `slab-ident` / `item-ident` say their made-up panes as rows as well as a rule.
- **A rule in the module WRITES the box world: whose a row is, for every writer** (decided 2026-11-03 with the doors' step as its first tenant; agreed
  with the senses' owner, whose smoke grid reads the leaves). The families still to move (blasts, breaks, cuts, glass, props) follow this, not a way of
  their own:
  - **A write owns a row, not a side.** ONE INVARIANT: outside a call into the module, a Box object and its row are the same box (in everything the copy
    keeps), whichever side wrote last. A TypeScript rule writes the OBJECT and says so, as before (`col.wrote()`, `leafWrote()`; `addStatic` says it
    itself), and the row is made the object before the next question. A Rust rule writes the ROW: it is the original from that statement on, so the
    rule's own later questions meet it with no crossing; and the host function that made the call makes the object FOLLOW the row before it returns to
    any TypeScript (`boxMirror.ts followLeaf`, called by `doors.ts takeBack`). Nobody else writes a Box from a row.
  - **What is state.** While a table of boxes still has a TypeScript writer (any twin standing), the OBJECTS are what a snapshot carries and what the
    tick's opening compare proves the rows against, exactly as before: a row is not state, nothing more is snapshotted, and a row put down and raised
    again under a running game costs nothing, because the objects never stopped being current. The writer's OWN state (a door's angle and clocks) is in
    the block under its bit; its box is worked out from that state by the same statements on both sides (`place`). The day a table's last TypeScript
    writer is gone its rows become a part of the block and the objects picture-only followers: not yet, and not a change to this rule.
  - **How the picture and the ray world hear.** They read the Box objects, as ever (a leaf is tested object by object in `raycast`; the upload reads
    them every frame), so the follow IS the telling, and it happens inside the same door step the TypeScript's write happened in. A static's follow, when
    the first static writer moves, must also make the calls a TypeScript writer makes beside its write: the smoke grid's `gridDirty` for the old and the
    new footprint (the senses' owner's ask: its faces cache must equal a fresh ask), the nav grid's rebake, `sound.invalidate()`.
  - **Noises are heard as they were made.** A step's noises come back as a list. `takeBack` hands them to the game DOOR BY DOOR, each door's before its
    own box is placed, so a listener that asks the box world while it hears (the page's mixer asks a sound's way at once) finds the leaves as the
    TypeScript's step had them at that statement: the doors before it placed, the rest not yet. While that goes on the leaves' list is marked to be
    copied from the objects again (`leavesTurned`); a step with no noise (nearly all) leaves the rows standing, and the tick's later questions copy no
    leaf (the TypeScript's `place` moved the count on every tick, so they all did).
  - **The leaves' list.** The module writes list 0 at the door's own index, so the game's own list must be the doors' leaves in the doors' order
    (`Doors.there` looks; any other list in its place and the TypeScript steps). The same index for the same leaf from tick to tick is also what the
    smoke grid's kept copy of the leaves compares against.
- **A family whose RULES move: what the doors add to the recipe below** ("A NEW FAMILY, step by step" is the state's half).
  - Its fields become ACCESSORS over columns, in TypeScript too (`Door.angle` → `DoorStore`: an array of the game's, or the block): one read for every
    reader, at home or not, so a checkpoint file, the digest, a probe's `Object.assign(d, …)` and the family's own TypeScript twin need no second way. A
    record that hung off a field (`Door.drive`) is a view of its columns, written through, never swapped for another object (`driveOf` copies one out).
    A RECORD THAT IS OVER LEAVES ZEROS. `drive = null` zeroes the drive's columns (`DRIVE_COLS`, taken from DC's own names; `doors.rs drive_off`
    is the same line, its list held to DC by `rust-check`'s `doors` section): cleared by its flag alone a record left numbers standing that no rule
    reads and the state's word does, so the same game restarted, loaded or played on was not the same columns, where the object a drive used to be
    was simply gone. `tools/qa/cleanrestart-check.ts` (line 5: a wrecked level restarted against its untouched twin) was RED ON THE SHIPPED STAND
    ONLY for it, behind the step that named the doors' columns: the frozen stand drives no leaf. So that check runs with the `doors` and
    `destruction` families of `probes.ts` now, and THIS FAMILY'S GATE IS RUN ON BOTH STANDS (`TPS_PROFILE=shipped` too). NOT MENDED, older than the
    columns and a rule of play if it were: a restart keeps a lock's click count (`pickN`, which times the next click) and the tally of leaves
    turned round (`mirrored`); the check's wreck works no lock, so it does not see them.
  - Its step has ONE door in the host (`Doors.update`): there, here, or both ways. `updateBoth` is the differential in play (`collision.ts BOTH`: the
    module's step with nothing kept, then the TypeScript's from the same columns, every column, contact, noise and box compared); `tools/qa/door-ident.ts`
    is the same on made-up men, with the game's own road (`twin`) every third step.
  - What it reads of the game that is not its own is handed in for the call and kept nowhere (scratch with a row in `STATICS`); a callback of the
    TypeScript's (`Doors.man`) is asked BEFORE the call for whoever the step can ask about, and the step says which answers it used (`manAsked`) so
    the host leaves on the game's objects exactly what the TypeScript's lazy ask left. A step that asks for one the host did not hand in is an error
    (`P_WRONG`), never a guess.
  - Its numbers are the TypeScript's: `rust-check`'s `doors` line reads every shared constant out of both sources (191 of them) and is red where one
    differs. A CHANGE TO `Doors.update`, `Door.place`, `settleSpot`, `bodyR` OR THE DOORS' `standable` IS A CHANGE TO `doors.rs` TOO.
  - WHAT THE FURNITURE ADDED (the second family; `props.rs`, `propshome.ts`, `tools/qa/prop-ident.ts`), each a rule for the next one:
    - **Nothing a step keeps may hide on an object.** `Prop` kept the cos and sin of its yaw in private fields, remade when `yaw !== yawCached`: a yaw
      that goes from −0 to +0 is the same yaw to that test, so the kept sin keeps its sign, and a module that worked them out afresh would part from
      the TypeScript by the sign of a zero. They are columns now (`cs`, `sn`) BESIDE THEIR SOURCE AND WRITTEN WHERE IT IS WRITTEN: every write of a
      yaw goes through one function on each side (`Prop`'s setter and `turnRow`; `props.rs turn`), which works the two out again when the yaw is
      another value than it was. No reader asks whether they are fresh. (A yaw that crosses from −0 to +0 with a write of another value between is
      the one case the old memo answered otherwise; no level authors a thing at −0, 851 things read.) Look for a memo before a family moves; what a
      memo is keyed on is part of the rule.
    - **Whoever hears a noise hears it AFTER the step, so the noise carries its own facts.** In TypeScript a listener ran in the middle of the step and
      could read the thing as it stood then (`Game.onPropEvent` read `e.prop.rolling` and `e.prop.x`); the module lists its noises and the host says
      them once it is back. So an event says where the thing was and whether it was rolling AS IT WAS MADE (`PropEvent.pos`, `rolling`), and the
      listener reads the event, never the thing. ONE HEARING ORDER FOR BOTH SIDES: the TypeScript step lists its noises too and says them after
      its last substep (`PropSystem.say`, `updateHere`), in the order they were made. So nothing a listener does (ask the module and move its
      memory, put the game's block out of it, shove a thing) reaches into a step half taken, on either side, and the step may keep the columns'
      arrays it asked for as it opened. `prop-ident`'s listener puts the block out of the module every fourth noise to hold that.
    - **THE DATA LAYOUT (the first part born to the layout rules; a part that follows copies this, not the doors' or the falls').** DENSE ROWS: row
      k of every column is `PropSystem.props[k]`, the things in the level now, in the order they are stepped, rows 0 to n−1 with the count in the
      head; a thing a blast takes out goes behind the live rows, those after it closing up in their order (`PropStore.leave`), and the level put back
      is every thing in the row of its id again (`homeRows`). `id` (row → thing) and `row` (thing → row) are the two tables; `Prop`'s own fields find
      their row through `row`, the step never does. So the order is ONE fact, in the rows: `remove` and `reset` are the only writers of the list, and
      `update` throws if the list and the rows ever count differently. NARROW COLUMNS: eleven doubles (x, z, yaw, vx, vz, w, cs, sn — the hot eight,
      read for every row every tick — then idle, pushT, noiseT), then four words: `flags` (asleep, dirty, rolling: three truths, one column of bits),
      `who` (a man's id), `id`, `row`. 104 bytes a row against 128 as sixteen doubles; a step's pass over the awake rows reads 8 doubles and a word.
      Every column starts on a 16-byte boundary (the head is 16 bytes: the clock, then n and the total; 128 rows a column). Clocks, places and
      angles stay doubles: the rules of play are doubles. THE STEP READS BY ROW ON BOTH SIDES (`d[X + r]`), the level's facts of a thing
      (half-sizes, mass, tune) from the hand-over's table by row there and from `props[r]` here. LANES a kernel could take, none taken yet: the
      integrate pass (x, z, yaw by vx, vz, w: two doubles a lane, awake rows by a mask of `flags`) and the pair pass's first test (the squared
      distance against the two radii). The solver proper is a branch a contact.
    - **The step counts what it met** (`PP.swept`, `guided`, `pairs`, `walls`, `held`, `substeps`: scratch, the host's tallies), so the differential
      can say its run was fair without a twin count in the TypeScript.
    - **A check written for one family is written for the next**: `simstate-check`'s three lines (the row on, off and flipped; a snapshot put back and
      into a fresh game; across a row change) are `tenantLines(family)` now, called for the doors and for the furniture.
    A CHANGE TO `PropSystem.step`, `solvePairs`, `solveStatics`, `circleContact`, `satRects`, `rollNote` OR `noise` IS A CHANGE TO `props.rs` TOO.
  - WHAT THE LEAVES OFF THEIR HINGES ADDED (the third family; `slabs.rs`, `slabshome.ts`, `tools/qa/slab-ident.ts`):
    - **Rows in the order they are stepped, THE STEPPED ONES FIRST.** Rows 0 to fly−1 are the slabs in the air, in the order of their doors (the
      order `doorSlabs` always took them in); rows fly to n−1 the ones lying at rest, which nothing steps again. A slab that comes to rest moves
      behind the ones in the air (`SlabStore.rest`), a leaf torn off enters in its door's place among them (`enter`), a door hung again gives its
      row up and the row is zeros (`leave`). `row[door]` is the row PLUS ONE: a part of zeros is no slab at all, which is what a part nobody is at
      home in must mean. The host moves rows, on both roads; no rule in the module does.
    - **A step that may have to be handed back works on a copy of its row.** Until the pane's open question was in the module a ray that ended
      on a pane was the host's, so a step whose ray met one could not go on: IT WORKS ON A COPY OF ITS ROW, writes the row back only if the whole
      step was its to take, and otherwise answers HOST with nothing written, and the TypeScript takes that slab's step for that tick
      (`SLABS_STEPPED.host`). The pane is no such case any more (a ray flies on through a hole in the module); what is left is a ray of no
      length or no number. The TypeScript step works on the row read out and written back in the same way, so nothing reads a slab half stepped.
    - **A memo keyed on an object is a column, and WHAT THE KEY WAS is part of the rule.** `wedged` (a slab with no clear spot in reach is not
      searched for again for a quarter second) was a side table keyed on the slab OBJECT. The checkpoint's run-on stepped a COPY, another object,
      which therefore had no memory. It is a column now, and the copy starts with it at 0, as it always in effect did.
    - **What a step met is said after the step, with the boxes as the step had them.** The slam was said in the middle of the step, with the slab's
      own box out of the dynamic list. Both sides now list what the step met (`SlabMet`: the slam and where, coming to rest, what the settling
      did) and `flySlab` says it once the step is over, the slam with that box out of the list again.
    - **A store of columns is ONE class** (`src/sim/colstore.ts ColStore`: the game's block and where the family's part of it starts, a swap of two
      rows, the bytes taken and put, the diff in words). `PropStore`, `SlabStore` and `ItemStore` are that plus what their ROWS mean (which are
      live, in what order, which tables say who is where). The next family extends it; the doors' and the falls' stores are older than it and are not
      on it yet (they are parts of the block the same way: "A game's block is taken as the game is made").
    - **The numbers written in the rule itself are held too.** A rung of a ladder, a reach, a margin (`0.45`, `1.2`, `6`) has no table to compare, and
      the differential meets some of them once in ten thousand steps. `rust-check`'s slabs section reads the numbers out of each function's body on
      both sides (`stepSlab` | `step`, `pushOut` | `push_out`, …) and is red where the two lists differ.
    - **The differential has to MAKE what play rarely does**: `slab-ident` throws slabs at panes, flags three statics in ten as panes with a quarter of
      them gone (`:panes`: a made-up state of the glass, said twice as the game's own is, as the rule its walk asks and as the rows the copy holds), buries slabs in the widest walls (the settling's whole ladder, the doorway it falls back to) and ages one to its
      six-second cap. A one-sided change to any of those was invisible before it did.
    - The lead that stood here (the copy should carry a pane's holes, so that a step need not be handed back for a pane) is done: "A pane's open
      question" below.
    - **A mission set must DO the thing a row moves**: see `@frag` above. A row whose rule no run reaches is held by its differential alone, and
      `rust-check` now says so of the slabs if its set has no such run.
    A CHANGE TO `stepSlab`, `pushOut`, `unstick`, `settle`, `overlaps`, `freeSpot`, `reachAlong`, `support` OR `acrossOf` IS A CHANGE TO `slabs.rs` TOO.
  - WHAT THE THROWN THINGS ADDED (the fourth family; `items.rs`, `itemshome.ts`, `throwables.ts`, `tools/qa/item-ident.ts`):
    - **A thing with an object of its own all over the game becomes A HANDLE, and the test of it is the compiler.** Forty places read an item's
      fields and a handful write them. With `pos` and `vel` typed read-only for one compile, every write of an ELEMENT of either showed as an
      error: there were none outside the step itself, only whole assignments, so the two are accessors that give a new array and take one whole.
      Do that compile before choosing the handle's shape for the next family.
    - **A public list stays a public list.** The first cut made `items` read-only and gave the pool `remove` and `clear`; a review found six
      places that still spliced it (one in the page, behind a cast; five in tools typed `any`), each a thrown tick. The list is the game's again
      and the rows follow it (`sync`: a compare of at most thirty-two identities a tick). A store that must be told of every outside write is a
      rule every caller can break; one that looks is not.
    - **What a step says is said when the WHOLE family has stepped, on both sides.** Said row by row in TypeScript and after the walk in the
      module, a callback that wrote the pool gave two games (the review's probe; no callback of the game's does). Both now list and say at the
      end, and `item-ident`'s callback stands another item up and takes one out of the list to hold it.
    - **An item that leaves keeps its numbers in itself** (`Item.kept`, a plain array by the columns' places): a door op holds its can and a guard
      the gun he dropped long after the pool gave the row up.
    - **No pool outgrows its rows** (until 2026-11-04 `ColStore.widen` took the columns out of the block, twice as long, and the TypeScript stepped
      them from there on): the thrown things' columns have twice the game's rule of thirty-two, and at that many live cans the oldest gives way
      (`itemshome.ts`). `item-ident` drives a pool past thirty-two (`:many`).
    - **What the module does not have is asked for by handing the row back, and the host says up front whether it can come up**: the snow's map
      is the host's, so a bounce on ground is the host's wherever `BogGround.may()` says snow may lie, and nowhere else. ONE hook answers both
      questions (`may`, `at`), so the two cannot disagree.
    - **One helper asks a rule's ray** (`lines.rs ray_rule`: a ray of no length or no number → the host's; a pane is judged where it is met).
      The slabs' step and the pool's walk both call it; the next thing that flies does too.
    - **The furniture's parts are a third list of the box world's copy** (the bullet "The box world's copy"): copied row by row where a part
      differs, at the tick's first question that needs them and after `thingWrote` (said by `Prop.place`, the one place a part is written, and
      where a thing leaves the level or the level is put back). `collision.ts thingsBoth` asks every furniture ray both ways under `--both`.
    - LEAD, not done: a bounce in a snow level is always the host's. The lays and banks `snowBog.ts` reads are boxes of the level; a table of
      them in the copy would let the module answer. LEAD: `frag.ts comeDown`, `rollOn` and `restOfThrow` forecast this flight for the men who
      flee it, with their own copy of the arithmetic; they are the frag rule's and move with it.
    A CHANGE TO `Throwables.stepRow`, `ray` OR `poseKey`, OR TO `PropSystem.raycast`, IS A CHANGE TO `items.rs` / `lines.rs ray_things` TOO.
- **What Rust code owes the game, and who holds it** (the head of `rust/sim/src/lib.rs`). One arithmetic: doubles, and only + − × ÷ √ and what the standard makes exact of them;
  never `f64::sin` and friends (`maths.rs`), never `mul_add`, never an `f32` where the TypeScript holds a double, no fast-math flag. One order: nothing whose
  order is not written down (no `HashMap` / `HashSet` iteration, no pointer as a key, no threads). No world outside: no clock, no random source of its own
  (the game's die is handed in), no files. A port is STATEMENT FOR STATEMENT: `(a - b) - c` is not `a - (b + c)`, and a term that looks like it cancels is
  there because the other side has it. Speed comes after the bits match, and SIMD after that, each held by the same checks.
  THE COMPILER holds the world outside and the order: `rust/sim` is `#![no_std]`, so `std::collections`, a thread, a clock, a file do not resolve in it.
  THE BUILD'S `rules` LINE (`tools/sim/build.ts RULES`, red in `rust-check`) holds what the compiler cannot once any crate links the standard library: its
  float methods (`.sin()`, `.powf()`, `.mul_add()`, and `.sqrt()` / `.floor()` as methods: the exact ones are `prim`'s functions), an `f32`, relaxed SIMD,
  `extern crate std`. The order of the arithmetic is the reader's and the differential's.
- **What holds a moved system** (`tools/qa/rust-check.ts`; `RUST_BREAK=1` must fail it). `built` and `face` (the module exports what `sim.ts` names, at its
  version). `maths`: one fixed stream (`tools/qa/rust-fold.js`, 14.2 million answers over the fourteen functions, the corners a port gets wrong among them)
  through the TypeScript functions and the Rust ones (`sim.ts mathsOfRust()`, the very set the row installs), every single answer the same bits. `engines`: the Rust functions fold that stream to one word under bun
  and under node, the TypeScript functions' word. `switch`, `stand`. `server`: the game server's own stand (`tools/mp/stand.ts`) takes the row from its recipe, ticks 600 times with the
  module's maths in `Math`'s place, and ends on the word of the same setup without the row (a whole server and its clients on the row:
  `tools/qa/mpserver-check.ts --setup '{"scenario":"?roomclear&squad4","seed":7,"knobs":{"RUST":{"maths":true}}}'`). `--missions`: five scripted squad
  runs on the server's stand (shipped, the senses on the CPU), each played twice, a row on and off, and the chained raw bits of the state after EVERY update
  (`game/digest.ts rawBits`) are the same; `--node` plays them under V8. Measured 2026-11-02: 38,634 ticks, every one the same bits, and the five end words
  are the same under bun and under node. `tools/qa/rust-page.cjs` is the page's half in Chromium: `loads` (the stream in the page, Chrome's V8 against the module, and the module's
  word against bun's), `rows` (a mission's first 240 frames with `?rustmaths` are the game without it, by digest and by every frame's bits), `plain` (the page as a player
  opens it: the rows of `RUST_ON` on, the same game), `systems`, `words` (`?rust<row>=0`), `nojit`.
  `senses` (`tools/qa/sense-ident.ts`): random questions over whole levels, doors swung, a share of the statics re-shaped half way and said, a dozen lamps
  of every kind put in, two dozen blobs; each asked of the module and of the TypeScript (the same function with the copy's rows put down for the one
  call), every number of every answer the same bits, and red if under half of a sense's questions went to the module, or if a level did not stand up.
  The floor's grid is baked again over the re-shaped boxes. Under `RUST_BREAK=1` the re-shape goes unsaid and the new grid is not handed over: it must
  fail, and EVERY sense must differ somewhere (exit 3 names one that did not). In `--missions` a run is a fair pair only if each raised sense answered
  there (`collision.ts SENSED` counts them: some hundred thousand eyes and lights a run, some thousand sounds, some hundred ways; the smoke has no
  floor a run, since not every run throws a canister, and a floor over the runs together), and `--both` answers every one of them twice. The game
  server's 600 ticks ask no smoke: the missions hold that row.
  A NEW SYSTEM GETS THE SAME THREE: a differential of its own on recorded inputs (`tools/cpu-frame/raycast-diff.ts` is that for the collision world), the
  missions with its row on and off, and the page.
  - **Every row has a section, and a red path that reaches it** (`rust-check.ts HELD`, `RED_SECTIONS`; 2026-11-03). `HELD` names, for each row of
    `RUST`, the section of the quick half that asks that system's own answers of both sides; `rows` is red on a row with no line there, or whose
    section did not run. Each differential ends on a line of its counts BY ROW (`SEGMENT-IDENT`, `SENSE-IDENT`, `POSE-IDENT`; the die's own line of
    `simstate-check`). Under `RUST_BREAK=1` every section that asks the module must be red AND every row's own answers must have differed: the run
    ends `the red path is red for every section that ran … and for every row's own answers …` and exit 1, or `THE RED PATH IS NOT RED FOR …` and
    EXIT 3. A break word that does not reach a row leaves that row's check proving nothing, and a red neighbour hides it. Three rows were like that
    until this was counted: THE CIRCLES (`segment-ident --rust` asked them after the copy was made good again: on the red path 0 of 40,000 circle
    answers differed; the panes walked through now go unsaid too, 11,430 differ beside 842 of the lines'), THE DIE (`SIMSTATE_BREAK=1` broke the
    block, not the die: its own line stayed PASS; a bit of its word is now turned there) and THE FOG (next).
  - **The fog's pass has a differential of its own, cones and all** (`tools/qa/sense-ident.ts`, the sixth kind of question). Every eye of the
    comparison missions sees all round (`FOG.fovDeg` 360; the cone is Hard's, the lens the wand's), so the wedge and the near field of `fog.rs`
    were asked by NO check, and the red path this row had (every eye handed over as seeing all round) was no break at all: with that row alone
    the five missions were the same game to the bit and `--both` found 0 of 18,830 answers differing. Now: one to seven eyes with wedges and
    near fields of their own or the team's, handed over by the game's own `fogofwar.ts handEyes` (what `update()` calls), on men, men down and
    bodies, tried in an order until so many see; 8,000 questions over four levels, each the module's answer against `seesHere`, none differing.
    Tried on it, by hand, and put back: an eye's cone handed over a tenth narrow, 73 of 4,000 answers differ; its near field 0.3 m long, 20; the
    red path's unsaid re-shape, 127 of 8,000.
  - **What the red path reaches INSIDE A MISSION, a row at a time** (measured 2026-11-03: pass-four and block-squads, `RUST_BREAK=1`, that row
    alone against the TypeScript): the game differs for `maths` (the engine's cosine), `fog` (one answer in seven turned) and `dice` (a bit of
    the word turned). For `lines`, `circles`, `sight`, `light`, `smoke`, `sound` and `paths` it is THE SAME GAME TO THE BIT: their break
    is the copy not told what was written, a mission's copy is made good as every tick opens, and no answer of those two runs fell between a
    write and the next tick. So `RUST_BREAK=1 --missions` is red by the maths, and `--missions
    --rows lines` under the break word now says so itself (`THE RED PATH IS NOT RED FOR the section missions`, exit 3). What holds those seven
    rows on the red path is their differential, by row. THE POSES had no break in a mission until they lived in the block; now, from the fiftieth
    tick that ends with poses at home, a pose's floats are handed to the TypeScript ten singles off (`rust-check --run`), and `RUST_BREAK=1
    --missions --rows poses` differs (hostage-rescue: 2,192 ticks against 3,766, other bits; exit 1). NOT DONE: a break that turns each row's own answers where they come back from the
    module (as the fog's does), so that a mission under the break word is red for every row raised alone.
  - **What `missions` holds of the fog, measured** (that row alone, the turned answer above): the bits differ on four of the five runs (not
    hostage-rescue), so a man shown who should not be reaches the state; with only "nobody sees" one call in seven, hostage-rescue and
    compound-four were still the same to the bit (the next pass sees him). The bits hold some of a system's mistakes; the answers asked both ways
    hold every one. A NEW ROW'S FINAL SAYS BOTH: its section's count by row on the red path, and whether `RUST_BREAK=1 --missions --rows <row>`
    differs.
  - **What ran for this** (2026-11-03, a cloud box, bun 1.4, no GPU, nothing drawn): the quick half (every section PASS but this box's own three:
    it has no linker for the crate, so `bytes` twice, and its node child does not answer, so `engines`); `RUST_BREAK=1` exit 1 with every
    section and row red; the same with the circles' question put back as it was, exit 3 naming the circles; `--missions` (five runs, every row,
    26,583 ticks to the bit); `--both` on two runs (2,662,497 answers, none differing, no stale row); `RUST_BREAK=1 --missions --rows fog` red
    on four runs; `typecheck` and `typecheck:qa`. Not run: node, the page, the whole `--both`. `fogofwar.ts` changed by one extracted function
    (`handEyes`), no statement of it moved.
- **State in the module is state of the game** (`rust/sim/src/state.rs`, `src/sim/state.ts`, `game/rollback.ts`). What a game keeps in the module from
  one call to the next is ONE BLOCK of bytes a game, and nothing else in the module is state:
  - **Rust.** `state.rs State` is a `#[repr(C)]` struct: a head (the layout's word, the homes) and then A FIELD A FAMILY of the update, each a struct
    of flat fixed-size columns, with its row in `PARTS` (where, how long, a version). Four rules: NO ADDRESS in a block, only indices (it is the same
    bytes in any slot, in the host's memory, in a file, on another machine); NO PADDING the compiler chose (each part asserts its size is the sum of its
    fields); ZEROS are a game with nothing in it; no allocation (`GAMES` = 4 blocks, the memory never grows). `tools/sim/build.ts STATICS` names
    every `static mut` of the crate as STATE (the blocks), SCRATCH (one call's) or COPY (the box world) and is red on one with no row, and on a
    static written through a cell.
  - **A pool.** A part may be a POOL (`Part.pool`; the poses): sized for the largest game, its first word the bytes of it in use. Only those are state:
    the host carries, compares and folds that much (`SimState.live`), a take's first piece of a pool says how much it holds, and what lies past is
    read by nobody (whoever hands room out zeroes it). So a part can be megabytes long and a game pays for what it uses. A POOL IS THE LAST PART of the
    block (`State`'s last field): a parked block ends where its last part's use does, so a fixed part put after the pool makes every parked block the
    pool's whole length (the doors' columns and the falls' store went in BEFORE it for that; `simstate-check`'s `parked poses` line is red otherwise).
  - **TypeScript.** A Game has one `SimState` (`game.sim`). Its block is BOUND to one of the module's slots or PARKED in a buffer of its own when a
    fifth game needs the slot (the one bound longest ago gives way; nothing is lost). `st.w` / `st.d` are the block and nothing else, wherever it
    stands: a read or a write from TypeScript is this game's and costs no call; a call into the module passes `st.bind()`. `st.f32(at, n)` is `n`
    singles of it as an array for a read or a write NOW (the SimState's own, made again when the block moves). A block is BOUND FOR THE LENGTH OF A TICK
    (`settle` brings a parked one in), so such an array holds until the tick ends; outside a tick, code that may be holding one asks `st.where` (the
    slot now, −1 = parked) and never moves the block. A parked block is ONE RUN of bytes from its start to its last word in use, not as long as the layout: the
    parts before its last pool are in it whole (the smoke grid's bullet has what that costs since its part came to stand before the poses).
  - **Tables made from the block are not in it.** The block is state and nothing else. A family that keeps tables DERIVED from its state (and from the
    box world) keeps them as a COPY with its row in `STATICS`, beside the `st.stamp` they were made at: a number no other block has had, given anew
    whenever the block's bytes were written wholesale or it changed place (a take put back, parked, brought into a slot, born, shut), and never for a
    family's own writes. A different stamp means: make them again. NO OBJECT OF THE GAME
    KEEPS A TYPED ARRAY OVER THE MODULE'S MEMORY (never `st.w` in a field): a snapshot put into another game would clone the buffer under it, the
    whole memory. The snapshot counts any it meets (`Snapshot.simViews`, carried on by every take since), `restoreSnapshot` THROWS on such a snapshot
    (in place, its bytes would land on whichever game holds the slot now) and `rollback-check` is red on one. A slot that is not one traps in the module
    (`state.rs state`): it never answers for another game's block.
  - **A game's block is taken as the game is made** (2026-11-05; `SimState.keep`, `sim/colstore.ts`, the five stores beside it). A family whose state
    is COLUMNS — the doors, what falls, the furniture, the leaves off their hinges, the thrown things — is made OVER the game's `SimState`
    (`new PropStore(game.sim, …)`: `Game` hands `this.sim` to `FallStore`, `PropSystem`, `Throwables` and `Doors`) and its columns are that family's
    part of the block from the store's first line to the game's last. THE STORE HAS NO ARRAY OF ITS OWN: no `own`, no `into` / `outOf`, no
    `home === null`, and "the store is not at home" is not a way back to the TypeScript any more — it cannot happen. Which side STEPS the columns
    (the module's rule, or its TypeScript twin over the same bytes) is the family's row of RUST and nothing else. `keep(bit, name)` takes the block
    if the game had none — zeros, PARKED in the SimState's own buffer, no call into the module — and raises the family's bit in the head for good,
    so `settle` never shuts a block under a store (a second store over the same part of one block throws by name). A block is BOUND as its first
    tick or its first question of the module opens, like any parked block. A SYSTEM MADE WITH NO GAME (a check's lone `PropSystem`, the level
    editor's lone `Door`) has a `SimState`, and so a block, of its own: the constructors' last argument, a fresh one by default.
    WITH NO WEBASSEMBLY there is no module to say how a block is laid out, and the game plays on the TypeScript for as long as there is one, so
    a block is then laid out from what the stores say (`simRoom(part, bytes)`, a top-level statement of each store's file: the head, then each
    part end to end in `SIM_PARTS`' order, a part no store lives in nothing) and is never bound. Where the module runs, its layout is held to
    those same numbers as it is first asked. That road goes when a page with no WebAssembly shows a card instead of playing.
    HELD BY: `simstate-check` on both stands — `tenantLines` for each of the four stepped families (the columns in the game's block at every
    tick with the row up, down, and flipped every 40; a snapshot put back, put into a fresh game, and put back across a change of the row) and
    `a crowd`: one game more than the module has slots, every family worked in each, stepped turn about, each the game one is alone after every
    tick though each opens every tick parked (`SIMSTATE_BREAK=park`, a parked block left behind, must fail `slots`, `parked poses` and
    `a crowd`); `falls-ident`, whose store's block is parked under the run 2,692 times and found bound again by its next rule in the module;
    the 42 by digest on both stands; `rollback-check` whole; the page with no WebAssembly (`rust-page.cjs nojit`). Still moving: the die, the
    poses, the smoke grid (below).
  - **At home, or not: the systems that still move.** A system with a TypeScript twin that keeps its state in fields of its own objects (the die, the
    poses) has it in the block only with its row on. Which, for THIS game, is a bit in the
    block's head (state like the rest), and as a tick opens every such system whose row and bit disagree is moved (`SimState.settle`, called by
    `Game.stepSeats`, inside the tick's own `try`). A system registers `simTenant({ name, bit, wants, into, outOf, tend? })`. `wants(g)` is the system's own
    reason, whole: a system with a TypeScript twin says its row of RUST and that THIS game has such state at all (the die: `RUST.dice` and a die of its
    own, so a game with none is never given a block for it); a rule that lives only in the module says its own switch (the smoke grid, the tenant in
    `game/sense.ts`, whose `into` is an empty grid and whose `outOf` is zeros, there being no TypeScript copy to move). `tend`, if it has one, is called
    each tick the system is at home, for what has come or gone since (the poses' is the rule of who is in the pool). So a row may be raised or dropped under a running game, a snapshot
    holds where things were. (A `SimState` nothing was ever kept or moved into has no block at all: an empty take, nothing in a slot. No game is such a one.)
  - **The snapshot** knows a `SimState` by its make (`K_SIM`): taken as the block in 4 KB pieces, a piece unchanged since the last take shared;
    put back into the block where it stands; put into ANOTHER game's own block; folded into `stateHash` and laid out by `stateDiff` a part at a time
    (`sim.dice[2]: … | …`). The saves kept in memory are snapshots, so they carry it with no line of their own.
  - **A checkpoint FILE and the raw bits do not carry bytes of the module, on purpose.** A row is never a rule of play: a file written with a row
    on must load with it off (a browser with no WebAssembly plays the TypeScript), and `rawBits` must give one word either way. So both go on
    reading the game's VALUES, and a system at home in the block answers for its values there (the die: `SimDice.s` / `.n`). The day a family has
    no TypeScript twin, its part of the block is what a file holds; the layout's word (`state_layout`) is there for that.
  - **A NEW FAMILY, step by step** (the die is the worked example: `rust/sim/src/dice.rs`, `src/sim/dicehome.ts`). It writes no rollback code.
    1. RUST: a file of its own with a `#[repr(C)]` struct of flat columns (numbers and fixed arrays of numbers; an index where another language
       would hold a pointer; a `spare` field wherever the compiler would pad, and a `const` assert that its size is the sum of its fields).
    2. A field of `State` for it in `state.rs`, after the others, and its row in `PARTS` (`offset_of!`, `size_of`, a version to raise whenever the
       columns change). No other `static mut`: scratch of one call is the only other kind, and it needs its row in `tools/sim/build.ts STATICS`.
    3. Its functions take the slot (`state(g)`) and are exported from `lib.rs`; `SIM_ABI` goes up, and the name joins `SIM_EXPORTS` in `src/sim/sim.ts`.
    4. TYPESCRIPT: its name in `SIM_PARTS` and a `P_<NAME>` in `src/sim/state.ts`, in the order of `PARTS`.
    5. A row of `RUST` (off in the module and on both stands; `?rust<name>`, `TPS_RUST<NAME>=1`, the word in `src/host/standEnv.ts STAND_ENV`).
    6. A file that registers `simTenant({ name, bit, wants, into, outOf })` with a bit nobody has used, imported once by `game.ts` (in `src/sim/` when it
       names no game module, like the die's; in `src/game/` when it walks the game, like `posehome.ts`): `into` copies the game's values from the
       TypeScript objects into the block, `outOf` back, to the bit, and leaves its part zeros. State whose owners come and go gets a `tend`.
    7. Accessors for what TypeScript still reads or writes (`st.w` / `st.d` at `simPart(P_<NAME>)`, never a view kept in a field), so a checkpoint
       file and `rawBits` see the same values with the row on or off. A call into the module passes `st.bind()`.
    8. `bun run tools/sim/build.ts`; then its lines: a differential on its own state in `rust-check`, its `fair()` counts in `--missions --rows <name>`
       (both arms on the same stand), `TPS_RUST<NAME>=1` in front of `rollback-check`, `savetimeline-check` and `seats-check`, a `_BREAK` path, the
       page (`rust-page.cjs`), and what a tick costs with the row off and on. The row's line in `rust-check.ts HELD`, and its own answers differing
       under `RUST_BREAK=1` (exit 3 names a row the red path left green).
    It gets rollback, the ring, a fresh game, two games and the saves in memory for that. Pieces are compared in JavaScript, word by word, at every
    take: fine for megabytes a save; a ring that takes one a tick over a block that changes all over (smoke that decays) will want the compare in
    the module.
  - **A KNOB, A NAMED NODE OR A LIST OF NODES THAT RUST CODE READS is a line of `rust/sim/src/knobs.rs`, and nothing else is written for it.** Three
    lists there, each a Rust name beside the name the TypeScript has: a KNOB by its path in the game's tables (`GAIT_LEAN = "MV.gait.lean"`; read
    `k(GAIT_LEAN)`, a switch `on(…)`), a ROLE by the animator's name for a node of the rig (`SPINE1 = "spine1"`; `role(SPINE1)`), a LIST by the
    animator's name for some nodes to run forward (`FK_LEGS = "fkLegs"`; `list(FK_LEGS)`). The module hands the names out (`knob_addr`, `knob_len`)
    and the host fills the values in: `src/game/rustknobs.ts` finds each knob's path once, in the registered knob tables (`game/knobs.ts`) or, for a path that starts
    `ANIM.`, in the animator's own tables of authored numbers (`character.ts animTables`: add a table there when a layer reads one that is no knob), and WRITES EVERY
    VALUE AS A TICK OPENS (`Game.stepSeats`, straight after the block is settled and before anything of the tick runs), so a knob turned in the
    panel, by a recording's `applyKnobs`, by a recipe or by a check between two ticks is the module's by the next, and the Rust code of a tick reads
    what the TypeScript of that tick reads. The roles and lists go with the rig (`anim/skeleton.ts layerSlot`, when another rig comes onto the
    bench). A knob is CONFIG, the same for every game in the process: it is in no game's block and no snapshot (the two tables are `COPY` rows of
    `tools/sim/build.ts STATICS`). RED: a path that is not a number of those tables (an error with its name the first time the list is read, and
    the `list` line of `tools/qa/layer-ident.ts`); a name of the lists that no line of the crate reads (its `read` line); a knob changed between two
    ticks that is not the module's by the next (its `changed` line). Never a second copy of a number in Rust: a constant the TypeScript has a name
    for is read from that name.
  - **A LAYER OF THE ANIMATOR, step by step** (`lifeLayer` is the worked example; the row is `walk`).
    1. RUST: a `pub fn <name>(g, at) -> u32` in `layers.rs`, the TypeScript method statement for statement and operand for operand: the same order
       of every sum and product, `maths::` for every `Math.` function, the helpers there for `Math.min` / `max` / `sign` (JavaScript's noughts and
       no-numbers), a product with an axis's nought written out (`0.0 * s`: −0 is a number too). `with_pose` hands it the pose, the rig and the
       run-forward bones (`Fk`: `part(list)` is `fkPart`). Its own numbers are the first sixty-four doubles of the bench's `args`, in an order its
       comment gives; the knobs, roles and lists it reads are lines of `knobs.rs`.
    2. STATE IT KEEPS BETWEEN TICKS (a phase, a spring, where a foot was put down) follows on `args` from the sixty-fourth double, in and out: the
       layer's COLUMNS of one character, their order in its comment. The host hands them over and takes them back through one pair of functions
       (`LayerCols`: `get` out of the animator's fields, `put` back in), so today they are kept where they always were, the TypeScript object every
       save and checkpoint already carries, and a save is the same bytes with the row on or off. THE TABLE IS THE CLASS'S (`private static
       readonly`, its two functions called ON the man: `colsToBench(CharacterAnimator.lifeCols, this)`), never a field of the man with
       arrow functions that have caught him: a function in the state is kept as it is by a snapshot, so a man who joined the mission after
       its start, put into a second game that has no such man, was handed the FIRST game's functions and wrote its state (found by
       `rollback-check` with the rows up on the shipped stand, `hostage-rescue:1`, the lines `fresh game` and `two games`). (When a character's columns have a part of
       `State` of their own, the pair goes and the slice is the block's.)
    3. WHAT IT ANSWERS: 0 = not a pose of the pool (the host throws) · else it ran, bit 1 = all of the run-forward bones on the bench are current
       for the pose (it solved a limb), a bit from bit 2 for each list it ran forward. `layerRan` brings back those nodes and no others, because the
       TypeScript after it may read the bones a layer left and nothing else of them may move.
    4. An export in `lib.rs`, its name in `SIM_EXPORTS`, `SIM_ABI` up.
    5. TYPESCRIPT: the method becomes three. `<name>(…)` asks `layerSlot` and runs `<name>Own` (the old body, untouched) or `<name>Ask` (the numbers
       onto `args`, the columns over, the export, `layerRan`, the columns back); with `LAYERS.both` up it runs BOTH from the same start
       (`layerBoth`) and compares pose, bones and columns.
    6. Its name in `LAYERS_MOVED` of `tools/qa/layer-ident.ts`: every call of it in a mission both ways, at least 100 of them, none differing.
       Then `rust-check --missions --rows poses,walk` to the bit under bun and node, the break (`RUST_BREAK=1` hands every knob a hundredth large:
       `both` and `changed` red, and `rust-check`'s `layers` section red), `rollback-check` with the rows up, the page, and what a tick costs.
    A branch a mission never takes is a branch nobody compared: say which run reached it (the sprint's share of `longBody` is reached by
    `warehouse-four`, not by `hostage-rescue`).
  - **The check**: `tools/qa/simstate-check.ts` (the `state, dice` line of `rust-check`, both stands): the layout; the die in a block against
    mulberry32, 4,000 seeds × 250 draws; nine blocks over four slots rolled turn about (2,696 parks, every die its own); take and put, here and
    into another block; a seeded game with the row on against off after every tick, and with the row flipped every 40 ticks under it; a snapshot
    put back (the state's word after each of 60 ticks) and put into a fresh game (the raw bits); snapshots put back across a change of the row; a
    planted view named and its snapshot refused; THE POSE POOL (a seeded game whose guards are made anew sixty times, `resetGuards`: rows on
    against off the same raw bits, after every restart the sets at home are exactly those of the characters on the game's lists with every pose
    of theirs in the block, and the pool never over one and a half times its first size: 6 characters, 92,416 bytes); PARKED POSES (a block
    with poses in it parked by four other games is read and written where it stands and not moved by it, is the parts before the pool and what is in use of the pool
    (318,944 bytes: the fixed parts' 78,928, the smoke grid's part of 235,600 that stands before the pool, and the 4,416 of the pool in use), takes a longer take put back, and has a new stamp each time it changes place or is put back and not for a write). `SIMSTATE_BREAK=1`
    (a parked block comes back zeros, a put back writes nothing, a bit of each die's word is turned, and a system at home is not tended as a
    tick opens) fails `dice`, `slots`, `take and put`, `row`, `snapshot`, `another game`, `across a row change`, `the pose pool` and `parked
    poses`; in front of `rollback-check` with `TPS_RUSTDICE=1` or `TPS_RUSTPOSES=1` it must fail the put-back lines there.
  - **What ran (2026-11-03, the cloud box, no GPU).** `rust-check --missions --rows dice` under bun and under node: the five runs, 26,583 ticks, to the
    bit with the die in the module (both arms stood up on `TPS_SEED`, since the row moves only a game's own die: these are other runs than the
    unseeded 38,634 ticks of the other rows); `RUST_BREAK=1` red. `rollback-check` on both stands with `TPS_RUSTDICE=1` (8 scenarios) and without;
    `savetimeline-check`, `dice-check` and `seats-check` (its `twin` line: the look ahead draws no die) with the row up; the page check
    (`tools/qa/rust-page.cjs`, headless Chromium on the software adapter: `systems` raises `?rustdice` on a seeded page, 240 frames to the bit, and
    `nojit` plays the TypeScript). With every row off a tick costs what it did: 3.44 → 3.41 ms over three runs, the same runs to the bit
    (`tickcost --each --list`, the quieter of two pairs). NEVER SEEN ON A GPU, never played; one seed a run.
  - **What ran for the poses at home (2026-11-03, the cloud box, no GPU).** `rust-check --missions --rows poses` under bun and under node: the
    five runs, 38,634 ticks, to the bit with every listed character's poses in the block (compound-four 8,060 ticks, warehouse-four 12,611,
    pass-four 5,616, hostage-rescue 3,766, block-squads 8,581); `--missions` with every row, 26,583 ticks to the bit; `RUST_BREAK=1 --missions
    --rows poses` red (above). `pose-ident`: 20,000 rounds, 260,000 calls answered by the module, 130,000 of them of a pose where it lives in a
    game's block, none differing; red under `RUST_BREAK=1`. `rollback-check` on both stands with `TPS_RUSTPOSES=1 TPS_RUSTDICE=1 TPS_SEED=1` (8
    scenarios each) and without, and red with `SIMSTATE_BREAK=1` in front; `savetimeline-check`, `seats-check`, `dice-check` and
    `statediff-check` with the rows up; `rollback-check --static`; the page check (`rust-page.cjs`: `systems` raises the row, 240 frames to the
    bit by digest and by every frame's bits). WHAT A TICK COSTS (`tickcost --each --list --rounds 3`, three missions, the three stands run SIDE
    BY SIDE on one box so each is slower than alone): 4.72 ms before the change, 4.68 with the row off, 4.73 with it on, every run to the bit.
    So the poses at home buy no time by themselves: about 500 posing questions a tick still cross, and `Pose.cols` under the row is a lookup;
    the time comes when a layer that reads them runs in the module. The module's memory is 2 MB a slot larger (the pool, four slots), on every
    stand, row on or off. NOT DONE, and only with the row on: more games alive than the module has slots (a tool, never the page or a server
    of one game) park and bring a whole live block a tick each; a man who leaves re-packs every pose of the game (rare: a restart, a bench).
    NEVER SEEN ON A GPU, never played with the row up; one seed a run.
  - **What ran for the doors and the falls (2026-11-03, the cloud box, no GPU; one seed a run; in full on the build's steps 1222 and 1226, and the quick half, the missions under both engines, the red path and the page again on step 1229, module version 13).** `door-ident`: 24,000 steps
    over four levels, 0 differ, with a listener that asks the module at every noise (4,446 times; without the read-out in `takeBack` the same run parts
    813 times). `falls-ident`: 200,000 rules in both lists, 0 differ. `rust-check`: the quick half all pass; `--missions` with all thirteen rows under
    bun and under node, 26,583 ticks, the same game after every update and the same end words under both engines (every door step in the module:
    26,583 of them, 36,167 leaf turns, 19 noises; 1,192 rules of the falls); `--both` 14.5 million questions answered both ways, 0 differ, 0 stale rows
    (run on the join with step 1217). `RUST_BREAK=1` is red for the doors' and the falls' own answers (a tick one unit in the last place too long; a
    landing's glass as much too heavy), quick and with two missions. `rollback-check` on both stands with `TPS_RUSTDOORS=1 TPS_RUSTFALLS=1` (78 lines;
    `SIMSTATE_BREAK=1` red), `savetimeline-check` both stands, `seats-check` both stands with the rows up and the frozen one without (17 lines each),
    `dress-check` with the rows and the two rules up, the static pass, `knobs-check`, `dice-check`, `shipped-check`, `boxshape-check`. The page
    (`rust-page.cjs`, headless Chromium on the software adapter): four jobs pass, `systems` with `?rustdoors&rustfalls` among the rows is 240 frames to
    the bit. TICK COST (`tickcost --each --rounds 2`, alone on the box, `BUN_OPTIONS` cleared, 38,634 ticks): the build 3.68 ms; this tree with the
    two rules' words off 3.61 ms and EVERY RUN TO THE BIT (so the doors' columns and the falls' store cost nothing a tick and change nothing); with
    the page's rules on 3.55 ms (another game: the rules); with the two rows up 3.60 ms, the same game as 3.55. So the doors' row does NOT pay yet
    (its hand-over asks the game about every man every tick): it stays off. NEVER SEEN ON A GPU, never played.
- **The game's own smoke on a coarse grid** (`src/game/sense.ts SENSE.grid`, `SmokeGrid`; `rust/sim/src/bricks.rs`; the maintainer, 2026-11-02: "rework
  smoke to be on a much simpler simulation on CPU (coarser grid with bricks?) and then have the visuals driven by the current pipeline"). A NEW RULE, not a
  moved one: there is no TypeScript twin and no row of `RUST`. Read only under `SENSE.cpu`. DEFAULT OFF everywhere: the page's `?smokegrid` (with
  `?cpusense`), the harness's `TPS_SMOKEGRID=1` on either stand; off is the blobs, the build as it was. No `FROZEN_HOLDS` row while it is down in the
  plain page too (the `STALK.drag` precedent); the journal carries `SENSE` whole and a file from before the key replays with it off.
  - WHAT IT IS. Density in half-metre cells, held in bricks of 4 × 4 × 8 cells (2 m square, floor to 4 m), 160 bricks a game, stepped ten times a
    second inside the tick (`Game.senseStep`). Fed by the emitters the picture's solver is handed (`CpuSmoke.emit` passes each on: its place, its
    width, what it lets out) and by nothing else of the solver: no velocity, no pressure, no wind, so `smoke.push` and the wind stay dressing. Smoke
    passes between two cells only where nothing solid and no leaf stands between their centres (the box world's lines, asked both ways), so a canister
    fills its room, stops at a shut door and goes out by an open one. It decays at the solver's rate and spreads at the blobs' rate. A sight line's
    transmittance is the density of the cell each quarter metre of it lies in (`grid_trans`, read from the state alone: it answers outside a tick too).
  - THE PICTURE IS NOT DRIVEN BY IT AND DOES NOT DRIVE IT. The GPU solver and its renderer are untouched. How far the two are apart is what
    `?modelerr` measures (the next bullet); the grid's numbers (`SENSE.smoke`: spread, decay, rise) have NOT been tuned against a GPU, and a doorway's
    haze after nine seconds is thinner than the picture's is likely to be.
  - STATE IS A PART OF THE GAME'S BLOCK (`bricks.rs Smoke`, the part `smoke` of `state.rs State`; 2026-11-03, it was an array of the game's own with a
    proven copy in the module before a game had a block). One row of doubles after the part's first word: a head with the plan in bricks, the emitters
    letting out, which brick is where, the light model's columns, and LAST the cells, a brick after another. The part is a POOL: its first word is the
    row up to the end of the highest brick that stands (`relive`), so a snapshot of a game with one canister out carries 92 KB of the part's 236 KB
    and an empty grid 72 KB, and what lies past is read by nobody (a brick's cells are written zeros when it is made). Nothing is copied in or out:
    a step, a sight line and a lamp's shadow name the block's slot (`st.bind()`), and TypeScript reads the row where the block stands
    (`SmokeGrid.at`, `row`, `glowAt`: `game.sim.d`, bound or parked). `SmokeGrid` keeps the game's `SimState` and never a view of it; what else it
    keeps is the host's half of the beat (the seconds owed, the emitters handed over since the last step, which go to the step itself in the
    scratch: one that finds no grid standing has taken none).
    AT HOME by the tenant in `sense.ts` (bit 16): `wants` is the rule's own switches, `SENSE.cpu && SENSE.grid` and a module that runs. It
    has no TypeScript copy, so `into` is an empty grid and `outOf` is zeros (`grid_reset`), as a tick opens; a new encounter and the senses coming
    to the game `renew` the grid where it stands. ONE THING PLAYS DIFFERENTLY FOR IT, with the switch up: an emitter handed over between
    `restartEncounter` and the next tick's `senseStep` used to reach the blobs alone (the game had no grid object until that step made one) and now
    waits for the grid's first step too, as it does for the blobs. No scripted mission has such an emitter: sixty runs with the switch up (below)
    are the same games by digest. A TOOL'S OWN GRID (`SmokeGrid.make`: the error tool's, a check's) is in a block of its own, a `SimState` nobody
    settles; it holds one of the module's four slots while it stands there, so its maker gives it back (`SmokeGrid.free`, as `ModelErr` does when
    its walk ends).
    WHAT THE MODULE KEEPS BESIDE IT is derived, a set of tables A SLOT OF THE BLOCKS: where each brick is (`INDEX`), which faces between cells are
    open (`FACES`), the leaves as the last step found them. WHOSE THEY ARE is three numbers (`MADE`) a call names as they stand now: the block's
    `stamp` (`SimState.stamp`: another number whenever a block was put back, parked, brought into a slot or born) and, for the faces, the box world
    and the NUMBER OF ITS COPY (`boxMirror.ts gridCopy`: another whenever every row is written afresh). Where one differs the tables are made again
    before anything is read. So a put-back, another game in the slot and a block parked and brought back need no line of the grid's. The faces are
    asked once and kept, asked again round every leaf that has moved since the last step and wherever the host rewrites a static's row (`statics()`
    says the rectangle to every set of tables asked of that world; a row past the end of the list as it was is always written), so they are at
    every step what asking them all afresh gives: a game put back from a snapshot and one that never was step alike. NOT DONE, and a cost: a
    put-back, and a block parked and brought back, ask every face again (which brick a cell's faces are kept under goes with the block). Measured
    here at 21 bricks: a tick that steps the grid goes from 0.75 ms to 3.6 ms when its block was parked since the last one, and a sight line or a
    lamp's shadow asked between ticks brings a parked block in (`bind`) and costs the same. It was so before the move too (a world's copy made
    afresh forgot them). Faces kept by their place on the plan would outlive both, which a client that rolls back every tick with smoke out will
    want. A FILE checkpoint keeps no smoke in the air (`savestate.ts`, as ever): a load clears the
    grid with the blobs. A clear of a grid nothing was let out into does nothing at all, and no clear moves the steps' beat (a page's warm-up clears,
    a tape of it played again does not, and the two step alike).
    THE ORDER OF THE PARTS IS THE PARKED BLOCK'S COST (`state.rs PARTS`): the host keeps a parked block as one run of bytes to its last word in
    use, so the room a pool does not use is paid for by every part after it. Fixed parts first, then the pools, the largest last: the smoke stands
    BEFORE the pose pool. THE COST, measured on the layout of module version 14 (head, dice, doors, falls, smoke, poses): every parked block is now at least 314,536
    bytes where it was 78,936 (the fixed parts, the doors' columns and the falls' store among them, are carried whole either way; the 235,600 more
    are the smoke grid's part), and one with poses is that much longer than it was (`simstate-check` `parked poses`: 318,944 bytes where it was
    83,344; its bar follows the layout: the fixed parts, plus the pools that stand before the last, which have a budget of their own of a quarter
    megabyte, plus the 16 KB the last pool's own use had; `slots` there parks 2,700 blocks and so allocates some 850 MB over its run where the
    fixed parts alone made it 210 MB). Only a process with more games than slots parks at all: checks, and never a
    page or a game server. A HELD BAR, agreed with the state's owner (the game rules' task), not the end state: a parked block kept as each
    part's live words back to back, so that an empty pool takes no room, is that task's to do in `state.ts`, and the bar goes back to under 16 KB
    with it. MENDED WITH THE MOVE, at that owner's asking: `SimState.take` matched an earlier take's pieces by one count over all the parts, so
    when the smoke's part grew by a piece every piece of the poses after it was copied anew; it now walks the earlier take a part at a time by
    its own lengths, as `put` does (`simstate-check` `pieces`: two pools at home, the smoke's part grown by a piece between two takes, the poses'
    eleven pieces all shared; none were with the one count).
    KNOBS: the step is still handed `SENSE.smoke` and `SENSE.glow` as arguments by the host. When the list of knobs Rust code reads lands
    (`rust/sim/src/knobs.rs`, the game rules' task), the step reads them there.
    WHAT RAN FOR THE MOVE (2026-11-03, a cloud box, bun, no GPU). THE SAME GAMES: the five missions of `rust-check --missions`, seeds 1 to 6, on the
    server's stand with `TPS_SMOKEGRID=1` and with `TPS_CPULIGHT=1` beside it (`tools/qa/playthrough.ts`), on the build before the move and after
    it: sixty runs, each the same digest, end and second. With the switches down, `tickcost --each --list --compare` against the build: 38,634
    ticks to the bit (2.16 → 2.14 ms a tick, nothing). `rust-check` quick and `--missions`; `smokegrid-check` and `cpulight-check` on both stands
    with their red paths; `simstate-check` on both stands; `modelerr`; `sense-check`, `shipped-check`, `seats-check`; `dress-check` with both
    switches up; `rollback-check` whole on both stands plain and with `TPS_CPUSENSE=1 TPS_SMOKEGRID=1 TPS_CPULIGHT=1` (8 scenarios each), and two
    scenarios with the die, the poses, the grid and the light all at home in the block. A PAGE AGAINST A SERVER (`mpclient-page.cjs`, headless
    Chromium on the software adapter, knobs `SENSE` cpu, grid, bounce): 27 of 27 state words over 818 ticks, the smoke's part of the block among
    what a word folds, so V8 and bun keep the same bytes there. `rust-page.cjs`, every job. THE PAGE BY ITSELF with `?cpusense&smokegrid&cpulight`:
    no page error, the grid in the game's own block (slot 0), a plan of 40 × 28 metres with the lamps' light in 757 and the bounce in 1,101, and a
    canister at Sam's feet in 15 bricks a few seconds on, the metre beyond it letting through a ten-millionth.
    AGAIN ON THE BUILD THAT HAS THE DOORS AND THE FALLS IN THE MODULE (module version 14, the same day and box): the sixty runs against that
    build, each the same digest, end and second; the switches down, 38,634 ticks to the bit; `rust-check` quick, `--missions` and its red path;
    `smokegrid-check` and `cpulight-check` on both stands with their red paths; `simstate-check` on both stands and its red path; `rollback-check`
    whole on both stands, plain and with the three switches up, and two scenarios with the die, the poses, the doors, the falls, the grid and the
    light all at home; `modelerr`, `sense-check`, `sense-ident`, `shipped-check`, `seats-check`, `dress-check` plain and with both switches up,
    `door-ident`, `falls-ident`, `glassfall-check`, `debrisfall-check`, `mpserver-check`; the page against a server with the three knobs, 32 of 32
    state words over 968 ticks and a watcher on bun 20 of 20; `rust-page.cjs`, every job; the page by itself, the same figures (slot 0, 757, 1,101, 15 bricks, a ten-millionth) and no
    page error. Nobody has played it. Never seen on a GPU.
  - THE BOOK. The head keeps three sums that thin at the cells' own rate: all that was let out, what thin bricks held when they were given back, and
    what an emitter could not let out because every brick was taken. What the cells hold + given back + refused = let out, at every step, to
    rounding: passing smoke between cells makes none and loses none. A brick is MADE where a cell at a neighbour's edge holds smoke and the way out of
    that cell is open (a wall along a brick's edge costs no brick behind it) and GIVEN BACK when it is thin, no emitter stands in or beside it and no
    neighbour holds smoke at the shared edge: the same test, so a brick is never made and given back in one step. A thin canister's fringe is given
    back as it goes (with no decay a tenth-strength canister has given back 28 % of itself after four minutes, a full one 4 %; with the game's decay
    the smoke is gone long before).
  - THE COPY IS THE GRID'S TO STAND UP (`collision.ts gridStep`): where no row of `RUST` has the box world's copy standing for the tick, the step makes
    it the boxes as they are, steps, and lets it go, with no question of the game's sent to it. Nobody who opens a tick has to know a grid is there.
  - WHERE NO GRID STANDS the blobs answer, as they did: a runtime with no WebAssembly (`simWhyNot()`), a level whose plan is over 8,192 bricks, a tick
    with more leaves than a list holds, and WHILE THERE IS MORE SMOKE THAN BRICKS (the last step wanted a brick there was no slot for, or what was
    refused is still more than nothing: `grid_trans` answers −1, and the grid answers again when it holds everything it was asked to). The blobs are
    still stepped beside the grid (they hold the emitter budget's count), so the fall back is whole.
  - `tools/qa/smokegrid-check.ts` holds it, on both stands: `stands` (and `sense.ts GRID`, the numbers this side reads the state by, against
    `bricks.rs` name for name), `mass` (the book balances to a part in 10⁹: a full canister and a tenth of one, no decay and the game's, at 14 s and
    at four minutes), `full` (sixty canisters: every brick taken, the grid says it cannot answer and the game's smoke along a line is the blobs'; one
    more where no brick is lets nothing out and the book says how much), `door` (a shut door holds, an opened one lets through), `block` (the part is in use up to the highest brick,
    the grid's object keeps no typed array, the snapshot met no view of the module's memory), `put back`, `another game`, `faces` and `parked` (the
    same state word straight · put back from a snapshot · the snapshot put into a fresh game · with the box world's copy another copy before every
    tick · stepped turn about with four other games over four slots, 386 blocks parked on the way; a door swinging and a wall moved and said on the
    way; `parked` is a fresh game handed the snapshot, so it is never put back while it plays). `SMOKEGRID_BREAK=1` is two breaks, each also a
    word of its own: `=leaves` asks the grid with no leaves (`door` must fail, and nothing else does) and `=stamp` pins every block's stamp, so
    the tables are never made again (`put back` and `parked` must fail, `door` must pass; `faces` goes with them). The check judges its own red
    path: exit 1 when every line a break must reach failed and none it must not, exit 3 naming the line otherwise.
  - Measured 2026-11-03 on a cloud box (bun, no GPU), ONE SEED A MISSION, the five missions of `rust-check --missions`. With the switch down every
    mission is the game it was, to the bit. With it up on the frozen stand (`TPS_SMOKEGRID=1`) three of the five are still the same game to the bit
    (the pass, the hostage rescue, the block: no line through smoke changed an answer in them); the compound is another game (taken at 10,870 ticks
    where the blobs' game is wiped at 8,060) and the warehouse another with the same end (taken, 12,611 ticks both). One seed says the rule moves a
    fight where smoke is thrown, not which way it moves fights: the tally over seeds is under the light model, below. A tick costs the same within
    the noise of a run (3.54 ms over the five with the grid, 3.60 without; the three missions of equal bits 3.73 / 3.43 / 3.62 against 3.80 / 3.47 /
    3.53), and the check's own `cost` line reads 1.12 ms a tick while a canister burns against 0.92 quiet. Nobody has played it. Never seen on a GPU,
    and the picture does not read it: the smoke drawn is the solver's, as it was.
- **The light model** (`src/game/sense.ts SENSE.bounce`, `SENSE.glow`; `rust/sim/src/glow.rs`, `light.rs`; `guards.ts lightEstimateAt`, `lightField`,
  `steadyLamps`; the maintainer, 2026-11-02, of the coarse smoke grid: "same goes for lighting too, right?"). A NEW RULE, not a moved one: no TypeScript
  twin, no row of `RUST`. It stands on the coarse grid, so it is read only where `SENSE.cpu` and `SENSE.grid` have one standing. DEFAULT OFF everywhere:
  the page's `?cpulight` (with `?cpusense&smokegrid`), the harness's `TPS_CPULIGHT=1` on either stand; off is the estimate as it was, the build as it
  was. No `FROZEN_HOLDS` row while it is down in the plain page too; the journal carries `SENSE` whole and a file from before the key replays with it off.
  - WHAT IT IS: the two things the estimate of the light on a man never had. THE SMOKE'S SHADOW: a lamp no wall hides is dimmed by the grid's smoke
    along the very line the wall test took: the game's own smoke on that line, one answer (`CpuSmoke.along`: the grid's cells, the blobs while the
    grid holds more smoke than bricks; the module hands such a question back). A grid nothing was let out into is not asked. THE LIGHT THAT COMES ROUND: one
    number A SQUARE METRE OF THE PLAN, in four columns after the grid's cells (the lamps' light at the metre's middle at chest height, the bounce, and
    whether the way to the next metre toward +x and toward +z is open: a clear line between the two middles, asked both ways, leaves and all; a
    pane lets light by, as the lamps' own lines and a reader's do). A thirty-second of the plan is asked again every step of the grid (ten a second),
    so a door that opens or a lamp that moves is in the field within 3.2 s; and THE WHOLE PLAN in the step that finds other lamps than it was last
    swept whole under (`light.rs word`: which lamps and how strong, nothing of where: a new field, a lamp shot out, a breaker thrown), so the light
    that comes round does not outlast its lamp (about 12 ms, once, on the office). Then two passes of one rule over the whole plan: a metre's bounce is `albedo` of what it keeps of its own light and what each open side
    hands it (`share` a side). At rest that is the rule's own fixed point, and no metre holds more than a closed room of the brightest lamp would.
    A POINT reads the nearest metre whose middle it SEES (its own; else across the nearer edge, the other, the corner), so a man against a wall reads
    his own room and never the one behind it; `lightEstimateAt` adds that to the brightest face.
  - WHICH LAMPS. `steadyLamps`: every lamp that is on, has no owner and is not a flash, a beacon or the moon, at its steady strength. A torch a man
    carries, a muzzle flash and lightning light a man directly (the estimate has them) and bounce nothing. A lamp that sweeps (the office has two)
    is followed a sweep behind.
  - WHAT IT LEAVES OUT, named: colour (one number), height (the plan is flat: a mezzanine reads the floor's bounce), the sun's and the sky's bounce,
    bodies' shadows, and any tuning against a GPU: `SENSE.glow` (albedo 0.3, share 0.1) is two numbers picked against ONE software-adapter picture.
    Every level's plan today is within 2,048 square metres; a larger one has no bounce (the smoke's shadow still stands).
  - STATE. The four columns and the lamps' word are in the grid's row, before the cells (64 KB; the part `smoke` of the game's block, above), so a
    snapshot takes them and a put-back writes them with the cells; a clear of the smoke keeps them, they are the level's and not the smoke's. A NEW
    ENCOUNTER TAKES A NEW GRID (`Game.restartEncounter`, and whenever the senses come to the game: `SmokeGrid.renew`): its beat and the sweep's turn
    start with the encounter, so a file loaded into a game that has run an hour and into a fresh one go on alike (this holds the smoke grid's beat
    too). A step with the model down zeroes a field that was swept: nothing old is read the day it is raised again. `glow.rs NEXT` is scratch.
  - `tools/qa/cpulight-check.ts` holds it, on both stands: `stands` (up, the plan is swept and the estimate has the bounce in it; a new encounter starts a new
    grid; down, or with no grid, the columns stay zeros and the estimate is the plain one to the bit; `sense.ts GRID` against the Rust sources), `rest` (the rule worked out
    in TypeScript from the columns, and the bound), `rooms` (light comes round by an open way, a metre with no open way holds its own alone, and
    over the whole floor a point reads exactly the metre the rule says, worked out in the check), `lamp` (the lamps a switch holds put out: a
    quarter of a second on no metre keeps their light), `door` (no open way joins the two sides of a shut door; opened, within a sweep, one does),
    `shadow`, `put back` (straight on against put back), `cost`. `CPULIGHT_BREAK=1` sweeps with no leaves and `door` must fail. `rust-check --both` with the switch up asks
    every light question of the module and of `lampsHere` and they agree to the bit (below).
  - Measured 2026-11-03 on a cloud box (bun, no GPU), ONE SEED A MISSION. With the switch down the five missions of `rust-check --missions` are the
    game they were, to the bit. With it up over the grid (`tickcost --each --list`, `TPS_SMOKEGRID=1 TPS_CPULIGHT=1` against `TPS_SMOKEGRID=1`)
    ALL FIVE ARE ANOTHER GAME WITH THE SAME END: the compound taken at 6,733 ticks where the grid alone takes it at 10,870, the warehouse taken at
    13,195 against 12,611, the pass taken at 5,537 against 5,616, the hostage rescued at 3,781 against 3,766, the block wiped at 4,501 against 8,581.
    That is the rule doing what it says (men in a room lit round a corner are seen sooner) and one seed does not say which way it moves a fight: a
    tally over seeds is the next line. A TICK: other fights do not compare tick for tick, and the two tables were made side by
    side on a busy box (the pass 5.17 ms against 5.90, the hostage rescue 5.16 against 4.69: noise both ways); the check's own `cost` line on the
    still office reads 1.01 ms a tick up against 0.89 down, and the whole plan swept at once (a lamp shot out) is 11 to 17 ms, once. BOTH WAYS
    (`TPS_RUSTLIGHT=1 TPS_RUSTSIGHT=1 TPS_RUSTSMOKE=1 TPS_SMOKEGRID=1 TPS_CPULIGHT=1 rust-check --both`, the compound, the hostage rescue and the
    block): 306,585 light questions answered by the module and by `lampsHere`, none differ, no stale row; that run is red on its own fairness floor
    for `smoke` and should be, since with a grid standing a sight line's smoke is the grid's and the blobs' row is asked nothing. Against the one
    kept picture (the software adapter, the office, 24 light points): `lit` is off by 0.025 of the meter at the middle where the plain estimate is
    off by 0.055, 0.107 at the 95th (0.105), leaning 0.027 darker (0.075), 0.77 stops at the 95th (1.23), the other side of the lit bar on the same
    4.2 %. `BUDGET.lit` is that with room, provisional as the others. THE PAGE (headless Chromium on the software adapter, the office,
    `?cpusense&smokegrid&cpulight`): it boots with no page error or warning, five seconds in the field stands on 40 × 28 metres (lamps' light in
    756, bounce in 1,101), Sam's meter reads 0.150 where the same page with the model down reads 0.077, and with a canister at his feet 0 where
    down it reads 0.081. Nobody has played it. Never seen on a GPU.
  - THE TALLY OVER SEEDS, the grid and the light (2026-11-03, a cloud box, bun, no GPU; the game code of the step that brought the light model). The
    server's stand (shipped, `TPS_CPUSENSE=1`), `tools/qa/playthrough.ts` and its scripted commander, the five missions of `rust-check --missions`,
    seeds 1 to 16, the 300 s cap, three arms: the blobs · `TPS_SMOKEGRID=1` · `TPS_SMOKEGRID=1 TPS_CPULIGHT=1`. 80 runs an arm, on each difficulty.
    STANDARD, blobs / grid / light: missions the squad won 51 / 53 / 58 of 80, its men lost 172 / 167 / 156 of 288, theirs left standing 155 / 155 /
    125 of 704; won of 16 by mission, the compound 8 / 12 / 13, the warehouse 8 / 10 / 11, the pass 15 / 13 / 16, the hostage rescue 16 / 16 / 16, the
    block's three squads 4 / 2 / 2. HARD (`--set '{"DIFFICULTY":{"level":"hard"}}'`): 54 / 60 / 55 won (one stall at the cap among the blobs' losses,
    one hostage lost among the light's), men lost 167 / 163 / 168, theirs left 158 / 122 / 141; the compound 12 / 14 / 11, the warehouse 10 / 11 / 10,
    the pass 13 / 16 / 16, the rescue 16 / 16 / 15, the block 3 / 3 / 3. SEED BY SEED. Blobs to grid: the same game by digest on 36 of 80 on each
    difficulty (no line through smoke changed an answer there); an end turned for the squad on 7 seeds and against it on 5 (standard), 7 and 1
    (hard). Grid to light: never the same game (every meter reads another number); an end turned for the squad on 9 and against on 4 (standard), 5
    and 10 (hard). WHAT IT SAYS: the grid leans to the squad, 14 ends for and 6 against over both difficulties, which a fair coin does about one
    time in nine; the light leans NEITHER way, 14 for and 14 against, its lean on standard undone on hard. Neither rule moves who wins a mission by
    more than sixteen seeds can tell from chance, so neither is a rule that makes the game harder or easier that this can see. What the light does
    move is pace where men are seen sooner: the pass, won, takes 74 s on standard where the grid alone takes 89 (81 against 89 on hard). WHAT IT DOES
    NOT SAY: anything of a person commanding, of the page (its senses are the renderer's), or of the watcher's smells, which were counted (340 / 352 /
    341 standard, 368 / 382 / 393 hard) and not read. Both switches stay down: the light has no TypeScript twin, and what it should weigh against
    the picture waits for a report from a real GPU.
- **The night's light from the sky in the estimate** (`src/game/sense.ts SENSE.moon`; `src/game/hours.ts moonLight`, `skyLight`, `skyLampOf`,
  `inEstimate`; `guards.ts lightEstimateAt`). THE SWITCH: OFF everywhere today (the page's `?moonlight`, the harness's `TPS_MOONLIGHT=1`; carried
  by the knob registry in `SENSE`, a recording from before the key replays with it off). Off = the estimate as it was, to the bit. AN EXCEPTION TO
  "A NEW RULE IS ON IN THE PAGE", SAID: it has no row of `FROZEN_HOLDS` because both stands have it off, as `SENSE.cpu`, `grid` and `bounce` are,
  and it is to come on with them. On by itself it would move every tally of the shipped stand at night and change what men do on the page (below).
  - WHAT IT IS. The estimate of the light on a man had no moon ("unshadowed it would light everybody"). The first run of the error tool on a GPU
    (the next bullet) read what that costs: under the open sky the picture's meter had a man lit and the estimate had nothing, the other side of the
    lit bar on 42 % of the points. ONE READING FOR THE LIGHT FROM THE SKY, AN HOUR OR NONE: `hours.ts skyLight` is what `hourLight` did for an hour —
    its one directional light where a point has a line to it past the statics and the door leaves, on the meter's cosine; the sky through whatever
    is open above the point's half-metre cell; one bounce off ground they light — and `moonLight` hands it the level with no hour laid over it: the
    level's own light from the sky (`skyLampOf`: its brightest steady directional light that is up — the moon as authored, or the sun a show put in
    its place, `lighting.ts setDaylight`) and THE LEVEL'S OWN SKY (`level.sky`, a show's `level.daylight.sky`), never the engine's settings: the
    page writes the level's sky there and a headless stand does not, and a rule may not differ by host. With the moon turned out, the sky alone.
    Nothing at another hour: the moon is out and the hour's sun has the same reading. Nothing of the level is touched. What it keeps a game
    (`nights`: the box and the rays of the cells asked) is made again when the lamp, its direction, the sky or the casters change (counted once a
    tick) and dropped by `hourWorldChanged` / `hourAfterReset` as an hour's cells are. It is state a snapshot carries, as an hour's is.
  - NOT IN IT: the edge of a shadow is hard here and soft in the picture (the points left over are in the building's moon shadow, where the picture
    reads 0.2 to 0.4 W/m² and this 0.08); a second bounce; glass is whatever `segmentBlocked` makes of a pane. The light that comes round
    (`SENSE.bounce`) does not carry the moon.
  - IT ACTS WHEREVER THE ESTIMATE IS READ, ON EVERY HOST, the page with the picture's senses too: the meter's stand-in (the senses on the CPU, the
    headless stand, the page's first frames) and every rule that asks what a PLACE lights — the team's weapon lights (`guards.ts LIGHTS`), the
    goggles and the IR illuminator (`nvg.ts`, `irlight.ts`), the calm torch, a search in the dark (`darkfoe.ts`), `pressure.ts`, the pair's
    orders, the lights-out mission. NOBODY ADDS THE MOON TWICE: `hours.ts inEstimate(g, lamp)` says whether a directional light is already in the
    estimate (an hour's sun; the night's under the switch), and whoever adds one on top asks it (`nvg.ts nvgPlaceLight`, `tools/qa/lesson-run.ts`
    `--moon`). OTHER STAND-INS OF THE MOON IN THE TOOLS ARE NOT GATED: `PT_METER=sky` (`hour-lib.ts meterAt`, its own sum from the lamps up),
    `pair-sneak.ts PS_MOON`, `vault-run.ts` and `lights-out-run.ts --moon` add theirs whatever the switch says, so a run with `TPS_MOONLIGHT=1`
    takes none of them. They were written because the estimate had no moon; when the switch comes on they go.
  - AGAINST THE GPU'S PICTURE (`tools/qa/modelerr.ts` row `moonlit` = the estimate with the light that comes round and the night's sky, asked on
    the SHIPPED stand of the kept report's questions; the frozen stand asks other questions and judges no `moonlit` row): under the open sky off by
    0.010 of the meter at the middle (0.355 without), 0.214 at the 95th, the other side of the lit bar on 1.1 % (42.0 %); under a roof 0.027, 0.113
    and 1.1 % (the worst roofed point was moonlight through an opening: 0.565 without, 0.175 with); 1.49 stops at the 95th over all 176 points
    where it was 15.70. `BUDGET` rows `moonlit:roof`, `moonlit:sky` and plain `moonlit` hold it there. The picture is one run on one adapter (Apple
    Metal), one level whose sky is the engine's default, one night.
  - THE CHECK: `bun run tools/qa/moonlight-check.ts` on both stands (the office, 81 points round the error tool's stations): `switch` (down = the
    estimate asked with no moon, to the bit; up = asked with it), `sky` (a point with a line to the moon is never under the moon's luminance ×
    intensity × the best face's cosine, 0.459), `shadow` (ON THE OFFICE: behind a wall or under a roof the switch adds under a quarter of that, at
    most 0.084 and 0.017 — a level with a dim moon under a bright sky would want another bar), `twice` (up, the goggles' light of a place is the
    estimate's own at every point and the moon is said to be in the estimate; down, the goggles add their own as ever), `hour` (at dusk the switch
    adds nothing; the night put back, what it added), `put back` (a snapshot put back reads every point as it did, the state's word the same).
    `MOONLIGHT_BREAK=1` (nothing stands in anything's way while the moon is asked) fails `shadow`. THAT DOWN IS THE BUILD AS IT WAS is held by
    the missions: the five of `rust-check --missions` with the switch down against the tree before it, 36,859 ticks, every run to the bit
    (`tickcost --each --list --compare`).
  - MEASURED (2026-11-04, a cloud box, bun, no GPU; the server's stand with the smoke grid and the light model up, `TPS_PROFILE=shipped TPS_CPUSENSE=1
    TPS_SMOKEGRID=1 TPS_CPULIGHT=1`, the five squad missions, seeds 1 to 16, `tools/qa/playthrough.ts`, with and without `TPS_MOONLIGHT=1`): won
    57 of 80 either way; men lost 154 of 288 without and 166 with; theirs left 125 and 116 of 704; a mean 96 s and 100 s. By mission (won · men
    lost, without → with): compound 12 → 12 · 34 → 37, warehouse 11 → 11 · 41 → 41, pass 16 → 16 · 15 → 21 (72 s → 83 s), rescue 16 → 16 ·
    3 → 6, block 2 → 2 · 61 → 61. Seed by seed 26 of 80 are the same game by digest (warehouse 10 of 16 and block 13 of 16; compound none), 76
    end the same way, 2 better, 2 worse. So where a fight crosses open ground at night the men in the moon are seen, on both sides, and it costs
    the squad a man more here and there; no mission is won or lost by it. `rollback-check` whole with the switch up on both stands (with the
    senses on the CPU, the grid and the light model on the shipped one): all pass. `probes.ts hours`, `nvg`, `hourwx` and `breach-daylight` with
    the switch down: as on the tree before (two of them red there already on the shipped stand: `breach-daylight`'s round trip and
    `hourwx-sheet`). WITH THE SWITCH UP `probes.ts hours` is red on `hours-light` and `hours-team`, which say what a night reads with no moon in
    it: they are to be told the night's sky when the switch comes on. The quiet missions, the lessons and the solo night have NOT been tallied
    with it, and they are where it matters most (Sam in a moonlit yard): that is the tally the switch owes before it comes on. Nobody has
    played it. Never seen on a GPU: the only picture it has been held against is the one kept report.
- **The coarse grid's smoke drifts with the wind and stays low in it** (`src/game/sense.ts SENSE.drift`; `rust/sim/src/bricks.rs` steps 3b and 4,
  `K.wind`, the scratch table `WIND`; `src/game/wind.ts stormDrift`; `src/scene/collision.ts gridStep`, `GridWind`). THE SWITCH: OFF everywhere
  today (the page's `?smokedrift`, the harness's `TPS_SMOKEDRIFT=1`; carried by the knob registry in `SENSE`, a recording from before the key
  replays with it off; it is the grid's, so it acts only with `?cpusense&smokegrid`). Off, a level with no wind, or under a roof = the step as it
  was, to the bit. No row of `FROZEN_HOLDS`, as `SENSE.grid`, `bounce` and `moon` have none: both stands have it off and it is to come on with
  them. A rule NEW in Rust: no TypeScript twin.
  - WHY. The first run of the error tool on a GPU (the next bullet) read the grid against the picture under the open sky: off by 0.883 of the light
    at the middle, deciding the other way on 64.8 % of the lines. The picture's solver pulls the smoke's whole velocity toward the storm's flat
    wind about three times a second (`smoke/wind_force.wgsl windForce`), the rise too: out of doors its smoke blows along and stays under head
    height (on the page's software adapter a canister's smoke stood 1.3, 1.8 and 2.6 m tall at 1, 3 and 6 s under a roof and 1.3, 1.3 and 1.8 m
    under the sky). The grid's stood over the canister and rose as it does in a room. THE MAINTAINER'S CALL, 2026-11-04, asked whether smoke out
    of doors should rise instead: "let's just leave it as is and have a smoke tuning pass before the release". So the grid follows the picture as
    it is, and if that pass lets outdoor smoke rise, step 4's damping is what to take out again.
  - WHAT IT IS, two things by one share. THE DRIFT (3b): before the spread, each step carries smoke across the open faces in plan out of the cell
    upwind (a donor's own density, so nothing is made), at the wind's speed at that height times the share of it the two squares have. IT DOES
    LITTLE OF ITS OWN (4): a face passes less of the smoke's own spread by that share. Upward, none in the open (the wind is flat), so out of
    doors the smoke stays in the layers it was let out into; sideways, down to `SENSE.smoke.inWind` of it (0.15), enough to get round what the
    wind pushes it against, as the picture's does by its pressure. (With the same 0.15 kept upward the grid was off again against the picture:
    0.213 at the middle and 20.5 % the other way.) THE WIND is the weather's own vector as the game holds it (`weather.ts wind`), wherever `Wind.blows()`
    says the weather blows at all — the one test the picture's storm uses too — and NONE OF A MACHINE'S OWN WIND SETTINGS (the master,
    `WIND.storm`, `WIND.stormK`, and the weather's own on / off, `?noweather` and the panel's box, which is the session's and which no snapshot
    or recording carries: what a player turns down is his picture, and with the storm switched off on his page the game's smoke still drifts),
    no gust, no vent, no draught at a door. So it reads the hour, the kind and the weather's numbers and nothing else. NOTE: the rest of the
    weather acts on the simulation only under a picked kind (`weather.ts simLive`); this acts under the office's unpicked night storm too,
    because that storm moves the picture's smoke and the picture's smoke is what the page's senses read. THE SHARE of a square metre is
    `stormShare` (0 under a roof, easing to 1 over the lee off it), from the footprints as the weather's roofs and `WX_OWN.wind` give them at the
    moment of asking, not from the storm's record (which the frame's dressing step keeps fresh, and a host with the wind turned down or no
    picture does not take). ITS RISE WITH HEIGHT is the storm field's (`WIND_FIELD.stormLow`, `stormGain`, `stormFullAt`: 0.55 of it at the
    ground, all of it by 2.5 m; `wind-probes.ts` holds the shader's own three numbers to them). A pass carries at most a quarter of a cell across
    a face, so a faster wind takes more passes of the step (eight at most: 10 m/s along an axis; faster than that it is slowed whole, both axes by
    one number, never bent). What goes over the plan's edge has left the level and is in the book as given back (`H_GIVEN` now holds both). The
    host writes the shares, a number a square metre, into the module's scratch table before a step, and only while there is smoke to move
    (`SmokeGrid.smoky`, read off the game's block).
  - THE CHECK: `bun run tools/qa/smokedrift-check.ts` on both stands (`SMOKEDRIFT_BREAK=1` tells the game a wind of nothing and must fail `switch`,
    `drift`, `calm`, `low`, `mass`, `bearing` and `back`). On the office: the switch, the drift downwind, no change in a room or with no wind in the weather, the same
    drift with the weather switched off in the session, the book with what went over the edge, a put-back and a fresh game handed the snapshot. On `gwot-yard.json`: the dust's wind (past one pass a step)
    carries the smoke on its own bearing (1.9° off; with each axis capped by itself, as it was first written, it went 20° to the side), of one of
    the game's canisters in the rain's wind none is above 1.5 m at 8 s where 17 % is in still air, a wind toward −x and −z goes its way and over
    the near edges, and the bricks are said.
  - THE BRICKS, SAID. In still air one of the game's canisters takes about 15 bricks. In the rain's wind (1.1, 0.45 m/s) on the yard it takes 62
    and three take 92 of the 160, because a plume is long; nothing was refused in 45 s. A level with more open ground or more canisters can still
    take them all: the grid then says it cannot answer and the blobs do (`smokegrid-check.ts full`), which neither drift nor stay low nor know
    walls, so a line's answer would jump between two smokes: this is not to come on before there are bricks enough that it does not happen. More
    bricks (the index is a byte: 255 at most without widening it) is a change to the state's layout and the Rust goal's to place.
  - AGAINST THE PICTURE (the kept report; `adrift` in the tool's rows, `adrift:roof@1.5` and `adrift:sky@1.5` in `BUDGET`: rows of the 1.5 m
    lines, the only height that report has): under the open sky 0.000 at the
    middle, 0.250 at the 95th and 2.7 % deciding the other way on 443 lines, where the grid in still air has 0.883, 1.000 and 64.8 %; under a
    roof the two are one smoke (0.019, 0.526, 10.3 %). WHAT THAT DOES NOT SAY: every line of that report runs at 1.5 m, and out of doors the
    picture's smoke stays under it, so the agreement is that neither hides a standing man, and smoke gone
    altogether or blown the wrong way would read as well (the check above is what holds those). AND IT IS AN AGREEMENT ON A KNIFE'S EDGE (found
    2026-11-05): 1.5 m is a face between two layers of the grid's half-metre cells, a level line on it reads the layer above alone, and this
    smoke fills the layer below — the same lines asked one centimetre lower let 0.21 of the light through at 8 s, not 0.93, and hide 74 % of
    them, not 4 %. So the 2.7 % says the grid's smoke stops at 1.5 m and nothing about whether the picture's is there under it. What a line
    at a standing man's chest or a crouching man's eye reads through the plume in the picture, nobody has measured: the plan now asks those
    heights (the model against the picture, LINES AT FOUR HEIGHTS), and a run of it on a GPU is wanted before this comes on. The report is one
    machine's (Apple Metal), made before the walk waited for its last reading.
  - MEASURED (2026-11-04, a 4-core cloud box, headless). THE SWITCH DOWN IS THE BUILD AS IT WAS: `tickcost.ts --each --list` over five missions,
    36,859 ticks, every run to the bit against the tree before, and the same 80 games by digest on the server's stand (below). WITH IT UP, the
    server's stand with the grid and the light model (`TPS_PROFILE=shipped TPS_CPUSENSE=1 TPS_SMOKEGRID=1 TPS_CPULIGHT=1`), compound-four,
    warehouse-four, pass-four, hostage-rescue and block-squads, 16 seeds each: 76 of the 80 games are the same game; won 57 → 56, men lost 154 →
    156 of 288. The four that moved are two on the compound and two on the pass, where smoke is thrown out of doors in a wind (one squad that
    lost three came through whole, one that came through whole was wiped): too few to say it helps or hurts. `rollback-check` whole with the
    switch up on both stands: all pass. `smokegrid-check`, `cpulight-check`, `modelerr.ts`, `rust-check`, `simstate-check`, `knobs-check`,
    `dice-check`, `rollback-check --static` on the tree: pass; `probes.ts wind` is red on `wind-identity` on both stands, as on the tree before.
    THE PAGE, headless Chromium on the software adapter, frames not drawn: it boots plain, with the grid, and with the switch up, with no page
    error, and six seconds after one of the game's canisters is let out in the office's north strip the grid's smoke has its middle 0.25 m from
    the canister with the switch down and 2.0 m downwind with it up; `modelerr-page.cjs` walks its smallest plan with the new row in the report.
    NOT MEASURED: the quiet missions and the lessons with it up; a level under the dust or the downpour in play; what a step costs with a long
    plume standing; smoke meeting a wall in the wind (it has a sideways spread to get round by, and nobody has looked at what it does there).
    Nobody has played it. Never seen on a GPU: the only picture it has been held against is the one kept report.
- **The model against the picture** (`src/host/modelerr.ts`; the page's `?modelerr`, `tools/qa/modelerr.ts`, `tools/qa/modelerr-page.cjs`; the
  maintainer, 2026-11-02: "we'd want some way during development of bounding the error between the two but then just let them proceed independently
  when actually running the game"). A DEVELOPMENT TOOL: nothing of it is made without the word and the game never reads it. The senses' CPU models
  and the picture are two models of one world and neither drives the other; this says how far apart they stand.
  - WHAT IT ASKS. One plan walked over whichever level is open: stations picked from the free floor (the one nearest the start, then each the farthest
    from those picked), at each a set of LIGHT POINTS at chest and head height and, round a canister it sets off there, level SIGHT LINES with
    nothing across them AT EACH HEIGHT OF THE PLAN (`EYES`: 1.6, 1.2, 0.95 and 0.4 m — LINES AT FOUR HEIGHTS, below), read at fixed ages of the smoke. Every sample is one question put to both sides: the probe's meter or its transmittance
    (`Game.probe`, asked under the tool's own keys at the last rank, eight places left to the game) and each model's answer (`MODELS`, a row a model:
    `estimate` = `lightEstimateAt`, `blobs`, `grid`; a new model is a row there and a row of `BUDGET`). The models are stepped on a fixed sixtieth
    whatever the page's frame rate, and the tool keeps its own smoke, so the game's is as it was whatever `SENSE` says. The player is stood at each
    station unseen and unhurt with the AI held (nothing moves a door between the page and a headless run) and the three flags are put back at the end.
    Not made on a tape's page or a networked one.
  - WHAT IT REPORTS, a model: the spread of model − picture in the unit the game decides by (light: the meter's 0‥1 scale, `Game.visibilityOf`; smoke:
    the share of light a line lets through): the middle, the 95th percentile, the worst, the lean; for light also stops at the 95th; and HOW OFTEN THE
    TWO DECIDE DIFFERENTLY (lit or dark at `LIGHTS.litAt`; hidden by the plume or not at 0.3). `BUDGET` holds a 95th percentile and a share of
    opposite decisions a model; a model past either is red. It bounds getting WORSE on the recorded scenes; it does not say the numbers are good.
    EVERY SAMPLE SAYS WHERE IT STANDS (`Sample.o`: under the open sky or a roof, by `weather.ts underRoof`, the footprints the rain and the wind go
    by — the light point; a line is under a roof only when its canister and both its ends are), and a report that says so is judged A PART AT A TIME
    by the rows of that part (`partsOf`: the light there, `estimate:roof`; the sight lines there a height at a time, `grid:sky@1.5`), its whole
    said and not judged: the first GPU run was the two places apart, and a whole would hide either in the other. A report that does not say is judged whole by the plain rows, and one where some samples say and some do
    not is a fault.
  - THE WALK CHANGES THE PICTURE ONLY WHEN THE PICTURE HAS ANSWERED (`ModelErr.changes`, `frame`): before the tick that lets the canister out, and
    before the one that clears the smoke and moves on, the plan's clock stands (the models are not stepped either) until everything asked has come
    back. An answer comes frames after its question. THE FIRST WALKS DID NOT WAIT: a station's last age was asked in the tick that cleared the room
    and moved on, so at every station but the last (nothing is cleared after it) its lines read 'no smoke' (the first GPU run: 1.00 on every 25 s
    line of the two stations under a roof, where the same lines had read 0.07 and 0.11 ten seconds before). Such a report is version 1 (`Report.v`)
    and those lines are in the file and not counted (`counted`); a walk that waits writes 2. Held headless by a stand-in picture (`waits`, below)
    and on the page by `PLAN=quick tools/qa/modelerr-page.cjs` (two stations: 14 of the first station's 16 last-age lines came back with smoke on
    them, on the software adapter). No walk that waits has been run on a GPU.
  - LINES AT FOUR HEIGHTS (2026-11-05; `EYES`, `Plan.eyes`, `linesOf`). Every line of the first walks was at 1.5 m, and that said little: out of
    doors the picture's smoke stays under a standing head (the smoke grid's drift bullet above), so the lines that decide whether a crouching man
    is hidden were never asked; AND 1.5 m IS A FACE BETWEEN TWO LAYERS OF THE GRID'S HALF-METRE CELLS, where a level line reads the layer above
    alone (`bricks.rs rho_at`), so the grid's answer there was not the answer of the air a centimetre under it (under the sky, `adrift` at 8 s:
    0.93 of the light at 1.5 m, 0.21 at 1.49). A plan now asks its lines at 1.6 m and 1.2 m (a standing man's head and chest, the heights of the
    light points: a guard's eye on a standing man runs between the two), 0.95 m (a crouching man's eye: `fogofwar.ts EYE_CROUCH`) and 0.4 m (low
    over the floor) — one height inside each of the grid's four lowest layers, none on a face — the same count at each. They are dealt by one
    die, each height keeping the first pairs clear AT IT (of the full plan's 144 lines a height on the office, 131 to 143 are shared by two
    heights), asked both ways round and kept inside the level (the first walks kept 3 of their 192 lines an age with an end inside a box,
    lines through a canister near the level's edge that ended off the map, and 8 pairs dealt twice: none now, and `walks` is red on a line
    asked twice), and put to the picture a line of each height by turns, so that no
    height is asked later than another when an age does not fit one frame of the probe (`waits` holds it with places for a third of an age a
    frame: the first line of each height, the second of each and so on reach the picture within a frame of each other — in the same frame,
    measured; `MODELERR_BREAK=turns` queues a height at a time, the lowest height's lines are then two frames behind the highest's, and it must
    fail). The spread is said A HEIGHT AT A TIME and `BUDGET`'s smoke rows are rows of a
    height (`grid:roof@1.5`): every row there is from the 1.5 m lines of the first GPU run and judges that kept report alone; THE FOUR HEIGHTS
    OF A PLAN HAVE NO ROW, said and not judged, until a walk that asks them is run on a GPU. MEASURED HEADLESS, the models alone (no picture:
    this says the lines are worth asking, not that a model is right): the full plan on the office asks 576 lines at each height on both stands,
    552 to 556 of each with smoke on them by the grid. On the page's stand, seed 1, UNDER THE SKY the grid with the wind in it (`adrift`) lets
    0.97 to 0.90 of the light along the 1.6 m lines from 3 to 25 s and hides 3 to 7 % of them; at 1.2 m it lets 0.58 through at 3 s and 0.24
    from 8 s on and hides 34 % and then 71 %; at 0.95 m 0.29 to 0.23 and 67 to 75 %; at 0.4 m 0.23 and 74 %. So in the model a standing
    man's head shows over the smoke and his chest does not, and whether the picture's plume reaches a chest is the first thing a GPU run of
    this plan will say. Under a roof the grid is thinner high up early (0.45 of the light at 1.6 m at 3 s, 0.31 at 1.2 m, 0.23 at 0.95 m,
    0.21 at 0.4 m: each over that height's own lines) and near one smoke at every height by 15 s (0.12 to 0.07). ON THE PAGE, THE SOFTWARE ADAPTER ONLY
    (`PLAN=quick tools/qa/modelerr-page.cjs`, headless Chromium, Very low, seed 1; never seen on a GPU): the picture answered all 46 light
    points and all 192 lines, 48 at each height, with smoke on 16, 26, 30 and 30 of them from the highest down, 44 of them at the last age of
    the station the walk moved on from; no page error. Its second station is under the sky, 12 lines a height at 3 and 6 s, the mean light let
    through, picture / `adrift` / `grid`: at 1.6 m 1.00 / 1.00 / 0.68 and 1.00 / 1.00 / 0.37; at 1.2 m 1.00 / 0.72 / 0.49 and 0.77 / 0.45 /
    0.27; at 0.95 m 0.74 / 0.44 / 0.35 and 0.58 / 0.43 / 0.23; at 0.4 m 0.58 / 0.37 / 0.30 and 0.50 / 0.40 / 0.22. So that picture does have
    smoke under a standing head out of doors, thinning upward, the drifting grid has the shape and is thicker than it at every height below
    the head (deciding the other way on 0 %, 19 %, 12 % and 4 % of the 26, 26, 26 and 24 lines under the sky), and the grid in still air is far thicker (38 %,
    38 %, 31 %, 38 %). One station, one adapter at Very low, two early ages: a first look and no row. NOT DONE: no GPU has been asked a line at any of the four heights; their rows are to be set from the first such run.
  - A KEPT REPORT IS ASKED AGAIN BY ITS OWN QUESTIONS (2026-11-05; `ModelErr`'s `again`, `planAgain`, `Report.asked`, `Report.look`, `lookOf`).
    A report used to be held to the models as they are now by walking its plan again and finding the walk dealt the very same questions (the
    `world` word), so any change to what a plan deals (a height, a count, the rule for a line) left every kept report judged 'as recorded' and
    still green. A walk handed a report now asks ITS stations, points, lines and ages again, in its order, dealing nothing: all of that is read
    from the report's own samples, and from the plan's row it carries (`asked`) only how long a station is stood at first.
    WHERE IT IS ASKED AGAIN: where its questions look out on the world they did. A report carries a word for that (`look`, `lookOf`), read off
    the box world by the CPU's own lines, and the tool works the word out again for the report's questions on its own stand: the lamps that
    are things of the level (an emitting surface or a lamp with its fixture: where each hangs, how far it reaches); for every light point,
    which of those lamps in reach a wall stands before; for every sight line, whether it is a line still, each way; for every station, how far
    a level ray runs before a wall, eight ways round at two heights, to a quarter metre and no further than 6 m. So a wall moved, cut or
    opened near a question moves the word, and a lamp moved, and a door stood otherwise where a question passes; and what no line of these
    meets does not: a box drawn and nothing else, what a box is made of, a car dressed at the far end of the yard, a door two rooms off, a
    mark a rule sets on a box for another reader. Nor the hour, the weather, whether a lamp is on or who carries one where: the stand's rules
    and the seed decide those, so the tool also wants the report's scene to name a seed and nothing but the plain page's words (`seed`,
    `solo`, `paused`, `mute`, `quality`: it stands up no other world). The word says 'these questions look out on the same walls and lamps',
    not 'the same picture' (the picture also reads paint), and a report asked again prints how far each model's answers have moved from the
    ones it recorded. The word is steady while no door moves (the same after 3 updates and after 300, for two seeds, on both stands).
    THE TWO STANDS: the frozen stand's office has two doors standing otherwise (the manager's, the break room's). The full report's
    questions meet them and it is asked again on the page's stand alone; the one-station software report's do not, and it is asked again on
    both, as it was before any of this. A REPORT THAT IS NOT ASKED AGAIN reads AS RECORDED ONLY on its verdict line, the reason is printed,
    and the tool's last line counts the reports that hold nothing here (frozen: `1 of 2`; the page's stand: `2 of 2`).
    ON THE PAGE'S STAND A REPORT KEPT UNDER `tools/qa/fixtures/modelerr/` MUST BE ASKED AGAIN, OR THE TOOL IS RED, a report at a time
    (`MODELERR_BREAK=look` reads every report as looking out on another world, and there each kept report must fail; on the frozen stand
    nothing is owed and the tool says the word fails nothing): a kept picture holds the models on that stand or nowhere, and a level stood
    otherwise must not empty the check without a word. What mends that red is a new report from a GPU (`?modelerr&seed=1&solo`), or taking
    the stale report out of the folder: a diff somebody reads. A report of a scene the tool never stands up is not asked by design and is no
    fault. A word for `MODELERR_BREAK` that is no red path is refused (exit 2).
    HOW IT CAME TO THIS (the first day, three words tried): a word of every box of the level with its whole flags moved with the wet ground's
    shine; one on the checkpoint's fingerprint (`savestate.ts levelFingerprint`) moved when the office's two cars gained 28 boxes of
    drawn-only trim and their bodies were cut to their lines (811 boxes for 783 on the page's stand), and both kept reports held nothing on
    either stand with the tool green; one of the whole floor plan, blind to shape, was blind to a cut wall beside a station and to a lamp
    moved. Each was a guess at what matters over the whole level. The word as it is asks only what the report's own questions meet, of the
    box world itself. On the tree before the cars changed and on the one after, both reports have the same words, and asked again give every
    model the very answers they gave before, to the last digit printed (no end of a line and no light point of the full report is within
    3.7 m of a car). WHAT THE PICTURE WOULD SAY OF THE CURVED CARS NOBODY HAS MEASURED: the kept pictures are of the cars as whole boxes.
    The two reports kept from before had their row and their word written into the file by hand (the rows as the plans stood: 6 stations,
    40 points, 32 lines at 1.5 m, ages 3, 8, 15 and 25 s, settle 2 s; 1, 24, 16, 3 and 6 s, 0.5 s; the words worked out on the page's stand),
    nothing else in them touched; asked again by their own questions, every model answers what it answered when the plan was dealt again
    (the same verdict lines, but for the height in a part's name). ON THE PAGE (the software adapter only, `modelerr-page.cjs` on
    the smallest plan: 24 light points and 96 lines answered, no page error): the report the page made says its word at the end of its walk,
    and the headless tool, handed that file on the page's stand, works out the same word in another engine at the start of its own and asks
    it again. No page on a GPU has said a word yet. `--keep` is gone: it added where each sample
    stands to a report from before the walk said, and a page's report is now a fixture as it comes. The `world` word is still written (a
    word for the questions) and decides nothing.
  - THE PAGE: `localhost:5173/?modelerr&seed=1&solo` (`=quick` two stations, `=tiny` one; `solo` is the solo night straight in — any mission's own
    words do as well). The walk does not begin under a menu (`ModelErrPage.held`: the start page, with the title's reel playing behind it, is one),
    and on an address that opens the start page the tool says so on a note at the foot of the page that takes no click (seen in a headless
    Chromium on the software adapter: the note up, and after ten seconds of the page's clock, the reel running, no walk begun; not seen by a person).
    Hands off
    for about three minutes: a card shows the table, `window.__modelerr` is the report, and it goes out by the page's door as kind `modelerr`
    (`host/savefile.ts`: to the backend where the page has one, no download).
  - HEADLESS: `bun run tools/qa/modelerr.ts` on both stands walks the plan with the models alone (`walks`: lines at every height of the plan, each
    height with smoke on some of them, no line twice, every model answers and a second walk is the first to the bit; the grid against the blobs is said), plants pictures and reads them back exactly (`planted`), judges a second walk against the
    first under a tight budget (`budget`; `MODELERR_BREAK=1` makes that walk's smoke three times as thick and it must fail), walks once with a
    stand-in picture that answers 45 frames late, has places for a third of an age's lines a frame and knows only whether the walk's smoke is in the
    room (`waits`: no light point read with smoke out, no line read from a cleared room, no height's lines put to the picture later than
    another's, the models' answers those of a walk with no picture to the bit; `MODELERR_BREAK=move` = the walk does not wait
    and it must fail, `=turns` = it queues its lines a height at a time and it must fail), and judges every report
    under `tools/qa/fixtures/modelerr/` (and `--report f`): as recorded, and, where its questions look out on the world they did, WITH THE
    MODELS AS THEY ARE NOW against the recorded picture, the report's own questions asked again (the sub-bullet above), and on the page's
    stand is red for a kept report that was not (`=look`). So a picture recorded once on a GPU holds the models to their budget from then on
    with no GPU.
  - MEASURED ON A GPU, 2026-11-04 (the maintainer's run: Apple Metal, the office at night, `?modelerr&seed=1&solo`, plan full, build aa4360cd; kept
    as `tools/qa/fixtures/modelerr/office-full-metal.json`, a version 1 report with where its samples stand added by a flag since removed; its questions look out on the
    SHIPPED stand's office, where the models answer as the page's did — smoke to the bit, light within 0.004 — and on the frozen stand's they
    meet two doors that stand otherwise, and it is read as recorded there). All 944 answered. UNDER A ROOF the models are close: `estimate` off by 0.076 of the meter at the middle, 0.145 at
    the 95th, the other side of the lit bar on 4.5 %; `lit` (the light that comes round) 0.029, 0.136 and 2.3 %; smoke at 3, 8 and 15 s over 165
    lines of the two roofed stations, `blobs` 0.009 of the light at the middle, 0.903 at the 95th, the other side of the hiding bar on 13.9 %;
    `grid` 0.019, 0.526 and 10.3 % (their 25 s lines are not counted, so no picture of a room's smoke at 25 s has been measured and the two smoke
    rows under a roof are provisional until a walk that waits is run; the mean light let through at the first station, picture / blobs / grid: 0.33 / 0.19 / 0.52 at 3 s, 0.16 / 0.10 / 0.19 at 8 s,
    0.07 / 0.10 / 0.12 at 15 s). UNDER THE OPEN SKY they are far off. LIGHT: the estimate has no moon, so 88 points read nothing where the picture
    reads 0.07 to 0.6 W/m² — off by 0.355 at the middle, the other side of the lit bar on 42.0 %; a man outside at night is lit to the picture and
    dark to the model. SMOKE: the picture showed NO smoke at any of the four stations under the sky at 3 and 8 s (1.00 on every line, those through
    the canister too) and a trace at 15 s (and at the last station at 25 s, 0.95 of the light on average), where the models keep a thick cloud
    (`grid` off by 0.883 at the middle and the other side on 64.8 %, `blobs` 0.958 and 77.7 %, over 443 lines). WHY THE PICTURE
    HAS NONE THERE IS NOT KNOWN (the outdoor wind, or the solver): nobody has looked at it, and the smoke under the sky has no budget row until
    somebody has. THE SOFTWARE ADAPTER SAYS THE SAME OF THE SKY (the page check's two stations at Very low, default settings, a walk that waits, same
    day: at the station under the sky 1.00 on all 32 lines at 3 and 6 s, and 21 light points off by 0.431 at the middle), so it is the picture and
    not the maintainer's machine or settings; under its one roofed station the smoke reads further from the models than on Metal (`grid` 0.098 at
    the middle and 20 % over 30 lines, past the rows below: another adapter at Very low, as its kept report). `BUDGET`'s rows by part are this run
    with room over them; under the sky the rows of `estimate` and `lit` are wide because neither has the moon, and `moonlit` (the bullet above) is
    the row that has. The page does not move onto these senses until the smoke under the sky is settled and a second run says so.
  - Measured 2026-11-03 ON THE SOFTWARE ADAPTER ONLY (a headless browser on a cloud box; never seen on a GPU): the office, plan `tiny` (one station),
    Very low, seed 1, kept as `tools/qa/fixtures/modelerr/office-tiny-software.json`. The picture answered all 24 light points and all 32 sight lines
    (26 with smoke on them). `estimate`: off by 0.055 of the meter at the middle, 0.105 at the 95th, 0.559 at worst, 1.23 stops at the 95th, leaning
    0.075 darker than the picture, the other side of the lit bar on 4.2 %. `blobs`: off by 0.181 of the light let through at the middle, 0.988 at the
    95th, leaning 0.331 thicker, the other side of the hiding bar on 37.5 %. `grid`: 0.038 at the middle, 0.947 at the 95th, leaning 0.206 thicker,
    25.0 %. So in that one room the grid is much nearer the picture than the blobs on most lines, and both are far off on the worst ones (a line
    through the edge of the cloud, where the solver's smoke has climbed and spread and the model's has not). (Its one station is the last, so its
    last age was read with the smoke still there: every line counts. Against the GPU's two rooms it is the thicker disagreement — 25.0 % against
    10.3 % for the grid — on another adapter, at Very low, at 3 and 6 s: nobody has said why.) Headless, the full plan on both stands,
    the grid against the blobs over 768 lines: 0.038 apart at the middle, 0.681 at the 95th, deciding differently on 13.0 %. The plain rows of
    `BUDGET` are that one report with room to spare, and judge a report that does not say where its samples stand.
- **Gameplay and dressing** (the maintainer, 2026-11-02: "let's make sure we're very explicit about what's gameplay affecting (and also run on the server) and
  what's just client side only dressing (debris, smoke details) and uses relaxed simd and/or gpu"). THE RULE: a step of the tick is DRESSING only if the game
  is the same game to the bit without it. Everything else is GAMEPLAY: it runs on the server and on every client, in one arithmetic. Dressing only makes the
  picture; the server need not run it, two machines need not agree on it, and it can read the game and never write it.
  - **The list is a table, `src/game/dress.ts DRESS`**: a row per picture-only step of the tick, and `game.ts` takes the step only with its row up (ONE EXCEPTION: `wetGround`'s step is always taken and
    reads its row itself, because with the row down it must put back what it wrote on the statics: `wetGround.ts follow`). Every row
    is up in the page and on both stands. It is not a rule of play: no `FROZEN_HOLDS` line, not a knob table, no recording carries it. The rows: `air`
    (airSim), `moths`, `snow` (the lying snow's surface, prints, marks, a blast's dig), `rain` (rainFx), `wetGround` (the open ground a rain wets, as the renderer's words of the statics: `docs/agents/weather.md`), `fires` (fireDraws; the lamp's flicker is a light and
    is gameplay), `wake` (men pushing the solver's smoke), `men` (skin instances, materials, shadow proxies), `cues`, `recap`, `blastFeel`, `fixtures`, `fx`,
    `stature`, `cases`, `net`, `wind`, and `debris` (the pool of rigid pieces: dressing only under `DEBRIS.dress`, `docs/agents/destruction.md`; without
    that switch the pool is the game's and is kept whatever the row says): eighteen. `dressAll(false)` puts them all down: a host that draws nothing may. NOBODY
    DOES YET: the game server still takes every step (measured below; its owner's call).
  - **`tools/qa/dress-check.ts` holds it**: the five comparison missions of `rust-check --missions` (the server's stand: shipped, the senses on the CPU), each
    played with every row down and with every row up, and the chained raw bits after EVERY update (`game/digest.ts rawBits`), the end and the run digest are
    the same. `--each` plays a row at a time (which row, when the set is red); `--rows a,b`; `DRESS_TRACE=<file>` in front of `--run` writes each tick's bits
    by part (`rawBitsParts`), which says what a step touched. `DRESS_BREAK=1` puts a gameplay step down with the rows (the doors) and must fail.
    A ROW THAT WORKS ONLY IN A WEATHER (`rain`, `wetGround`, `wind`) does nothing on a dry night, and the plain run says nothing of it: it is held by
    `--weather <kind>` (the missions played in that picked weather; RED when no run had anything wet with the rows up). The check is in no gate or
    probe family: it is run by hand.
    Measured 2026-11-02: 38,634 ticks, the same bits with the sixteen rows down, and the five end words are `rust-check --missions`' own; on the frozen
    stand (`TPS_PROFILE=frozen` in front) two of the missions, 6,080 ticks, the same; the red path parts on both of its runs, and a red path whose runs leave
    nothing is said to be NOT red. What putting the rows down saves a headless host: compound-four played alone, twice each way, 35.1 and 35.0 s with the rows
    up, 33.6 and 34.4 s with them down, about 3 % (a headless engine draws nothing, so the hand-over of the men is most of it). The page with every row up
    is the page it was: `rust-page.cjs`' three jobs pass on this tree with the digest and the bits of the tree before it. Not run under node; no page has
    played with a row down (the table is not on `window`).
  - **A NEW ROW** is a step whose code has been READ for the four things dressing may not do (push a sound a man hears, draw from the game's die, move a
    clock a rule reads, write a field a rule reads: a pose, a light, a box a ray meets; and one more, put smoke where the senses read it: `smoke.emit` feeds the
    CPU's own smoke and its budget, `sense.ts ownSmoke`, where `smoke.push` and the wind move only the solver's) and that then passes the check. The check alone is not enough: it only
    sees the paths the five missions walk (no takedown, no scripted smoke break, no GPU particles).
  - **WHAT THE CHECK'S STAND DOES NOT SAY.** The rows are dressing where the senses are on the CPU (the server, the headless tools). On the page as it opens
    today the guards' eyes read the GPU's light probe and the GPU's smoke, and there `men` (their shadow proxies), `wake` and `wind` (the smoke they move) reach
    a rule through the picture. That ends when the page plays on the CPU's senses (the maintainer's steer, the part "Run smoke, light and senses on the CPU");
    until then a page with a row down is not promised to be the page with it up.
  - **Tried, and NOT dressing today** (their steps have no row and run everywhere): the tracers (`tracers.ts`: a streak's head carries a light, the light set
    is what the CPU's meter reads: one mission parted); what the men carry in hand (`handProps`: two missions parted, Sam's own numbers first); wetness
    (`wet.ts`: the raw bits did not move and the run digest did, which holds it); `threatTick` (it writes the pose's `deliberate`, and bones are what sight, the
    meter and rounds read); `takedownTick` and `smokebreak.ts` (each writes a pose; the missions exercise neither). THREE PASSED THE CHECK AND ARE NOT DRESSING, BY READING: the
    breath on a snow level (`snowBreath.ts` emits smoke, which the CPU's senses read and which counts against the smoke budget a grenade is emitted under),
    the sparks (`sparks.ts`: a stun's smoke trail emitters ride the pool's live positions) and the look of a wall charge (`blastLook.ts` warms the colour of
    the stun flash's light, and the meter drops a light by its brightest channel). And inside `drawMen`: WHAT A MAN WEARS
    (`c.parts`, `cast.ts castParts`) is dealt there, to the men the fog lets be drawn, and his kit's boxes are placed off it (`rigProps.ts packOf`); so that
    one statement stays with the `men` row down. That a man's kit depends on whether he was drawn is found, not changed.
  - **Found, for the maintainer's call** (the page of this work puts them to him; nothing changed): debris (`debris.ts`) is gameplay three ways today: a
    landing files a `'prop'` sound the men hear, the pool's size is `settings.debrisCap` (64 on the two top graphics tiers, 32 below: two machines on different
    tiers do not play the same game), and its chunks are in the raw bits. Glass: where shards land becomes the litter that makes a footstep louder
    (`shards.ts` underfoot).
  - **In Rust**: `rust/sim` is gameplay, `rust/dress` is dressing, `rust/prim` is for both (`tools/sim/build.ts SIDES`; a crate with no row is red). They are
    TWO MODULES with a memory each (`src/sim/built.ts`, `src/dress/built.ts`; `src/sim/load.ts` starts either, handing in nothing), so dressing has no road to
    the simulation's memory: what it needs of the game is copied in (`src/dress/dress.ts dressIn()`). The build reads the BYTES that ship
    (`tools/sim/wasm.ts`, every instruction decoded): `sides` (no simulation crate links a dressing one, nor the other way), `alone and strict` (neither module
    imports anything; the simulation's code holds no relaxed SIMD and no threads instruction; an instruction the reader does not know is red until somebody
    names its family), `reader` (that reading tried on hand-made modules that break each rule). Fixed-width SIMD is allowed in the simulation (the same
    answer everywhere); relaxed SIMD only in dressing. `rust/dress` holds nothing but its hand-over today: no dressing system has moved, nothing calls it, and
    no relaxed-SIMD target feature is turned on (which browsers run those instructions was not looked up).
- **What a tick costs with the rows up, and where it goes** (the hill-climb, 2026-11-03; a 4-core cloud box, bun 1.4 and node 24, one seed a
  mission, `tickcost --each --list --rounds 2`, the five runs, 38,634 ticks, the same bits in every table; nothing drawn, never on a GPU).
  - A tick's update, ms: bun 2.02 with every row down · 1.92 with `lines`, `circles`, `fog`, `sight`, `light`, `smoke` up (−4.8 %) · `sound` and
    `paths` add nothing a mean shows. node 2.23 down · 2.17 up (−3 %). A row raised ALONE is inside the noise (1 %); `poses` alone is +6 %
    (its bench copies a pose in and out) until the animator's data is at home in the module.
  - A question, TypeScript → the module, µs (`sense-ident --bench`): an eye 1.1 → 0.8 · the light on a point 8–11 → 5–9 · smoke along a line
    3.3–7.3 → 2.6–5.1 · a sound's way 92–136 → 91–127 · a way across the floor 47–85 → 47–79.
  - WHERE A FRAME GOES (`tools/qa/framecost.ts --scn compound-four,warehouse-four,pass-four,hostage-rescue,block-squads --seeds 1`: the page's
    stand and cadence, every row down, the services timed apart; 42,205 frames, 2.44 ms a frame): THE ANIMATOR IS A QUARTER OF IT (0.60 ms,
    8 to 16 updates a frame); the box list's pack for the picture 18 % (the page's every frame; a server's tick lists instead, 0.02 ms); the
    box world's lines 9 % (0.23 ms, 300 to 460 a frame; the walk is the one hot function of a profile, 6.5 % of a mission's samples in
    TypeScript, 5.0 % as the module's); the fog's pass under 2 %; a sound's way and a way across the floor a few hundredths of a ms a frame,
    and the worst frame of one mission in five (25 ms of paths). The worst frame of the other four (46 to 74 ms) is what stands at the end
    of the update, the checkpoints among it. The tool's marks in `Game.update` have gone stale (sixteen not found): its split BY RULE is
    coarse until they are put back; the services' rows are exact.
  - So the size is in the animator, then in what only a whole tick in the module removes (the small vectors and the garbage they are). The
    rows that stand are near their floor: a tenth off the line walk is half a percent of a tick.
  - Done: `sim.ts` keeps its views over the module's memory and tests their length (a view over a detached buffer has none) where every ask
    fetched `memory.buffer` from the engine, a thousand times a tick with the senses up: 0.6 % of a mission's samples, 1.2 % of the tick with
    the six rows up under bun, nothing under node. `rust-check`'s `views` line grows the memory by a page and holds every view remade.
    `SimState.bind` (state.ts) and `dressIn` (dress.ts) still fetch the buffer on every ask: the same test would do for them.
  - Tried and dropped: fixed SIMD switched on for the whole crate (`+simd128`): no question got faster, nothing in these kernels
    vectorises by itself. A build rule against `memory.grow`: the module has held ONE since its first build (the standard library's
    allocator, linked through `rust/prim` and called by nothing), so the views are held by the length test, not by a rule.
- **Rows that are on** (2026-11-03; the maintainer, of the release: "let's also get the rust rewrite into this build too"). `src/sim/sim.ts RUST_ON`
  = the rows that are ON WHERE THE GAME IS PLAYED: `lines`, `circles`, `fog`. The page's boot raises them (`profile.ts pageRows`, from `pageSwitches`), so the
  shipped stand has them and a game server's recipe carries them; the frozen stand holds every row off (`FROZEN_HOLDS` `RUST.lines`, `RUST.circles`,
  `RUST.fog`, made from the list; `TPS_RUST<ROW>=1` raises one). OFF WORDS: `?rust=0` = none of the default rows, the game in TypeScript as it was (a row
  named beside it is still raised: `?rust=0&rustmaths`); `?rust<row>=0` / `TPS_RUST<ROW>=0` = that one off. THE ADDRESS WINS OVER A RECIPE, in two places, because a replay's
  or a networked game's page meets its recipe twice. First it is SENT ON to the recipe's address (`engineplay.ts recipeSearch`), which keeps of the visit
  only the visit's words: `rust` and every `rust<row>` are among them (`sim.ts rustWords`, spread into `ui/replaykey.ts VISIT_FLAGS`), so a typed `?rust=0`
  is still there when the page reads it and a recipe recorded under such a word does not hand it on. Then the page lays the recipe's knob tables over its
  own, `RUST` among them, and `main.ts recipeTables` writes what the address SAYS of the rows again (`pageRows(q, true)`). An address with no word keeps
  the recipe's rows. (The first cut had only the second half, and the word was gone before it was read: found by the multiplayer task, who ran it.) A ROW IS NEVER A
  RULE OF PLAY: on or off it is the same game to the bit, so nothing here joins a pin list and no digest moves; a recording from before the rows replays
  with them off, one made with them on carries them (`journal.ts applyKnobs`). A runtime with no WebAssembly keeps every row off whatever the list says.
  WHAT IS ON THE LIST AND WHY: a row goes on when it is proven the same game AND costs nothing in the module, measured. `lines` is faster there; `circles`
  and `fog` ask the same copy of the boxes and measured neither faster nor slower (below). Every other row is a proof and stays off because ON IT IS
  SLOWER TODAY: its state or its layers are right in the module, but while what calls them is TypeScript every question crosses (`maths`, `poses`,
  `dice`, `doors`, `falls`, and `walk`, the animator's layers). The senses' rows answer only on the CPU-sense stand and were not turned on here.
  THE MAINTAINER RULED the same day that the game may REQUIRE WebAssembly: a rule that exists only in Rust may be on in the page, and a moved family's
  TypeScript twin is deleted in a change of its own once its row has been on in a named build and played. Nothing was deleted here, and the page still
  falls back silently. The fog's pass has the guard the senses have (`collision.ts ownLines`): a check that puts its own line test over one instance
  gets the TypeScript pass, which obeys it.
  WHAT WAS RUN, on the build's head of that evening with this change, a 4-core cloud box, bun 1.4.4 and node 24:
  - The same game: `rust-check.ts --missions --rows lines,circles,fog`, six runs, 41,393 ticks, the chained bits after every update the same with the
    rows on and off, under bun and under node. With the poses' and the walk's rows raised beside the three (`--rows lines,circles,fog,poses,walk`), the same
    six runs are the same game too. The sixth run is new: `hostage-rescue:1:hard`, on Hard, where every eye is a cone (2,759 ticks, every one
    with `FOG.cone`), because the five standard runs ask no wedge and Hard is how the maintainer plays. `--both` (every question answered both ways,
    every row of the copy compared): 0 differ, 0 stale; on the Hard run 773,068 questions, 13,795 of them the fog's with cone eyes. The wand's lens is
    asked by `sense-ident`'s random questions only.
  - The stands: `rust-check.ts` quick half 0 FAIL (`stand`: frozen none, shipped the three, each word, the `FROZEN_HOLDS` rows, eleven cases of the
    address's words as a page opens and over a recipe's rows; `server as the page`: a recipe that says nothing carries the three and 600 ticks are the
    word of the game with every row off; a page sent on to a recipe's address keeps the visit's words about the rows and drops the recipe's own).
    `RUST_BREAK=1`: exit 1, red for every section and every row; the page-rows stand has a red path of its own
    (every line through the statics answered "clear": a stale leaf and a turned answer of the fog's moved no word in 600 ticks, tried first).
    `probes.ts shipped`, `shipped-check`, `knobs-check`, `rollback-check` (static, and both stands), `seats-check`, `cleanrestart-check`: no FAIL.
  - The page: `rust-page.cjs` in headless Chromium on the software adapter (frames not drawn), seven jobs ok. `plain`, the address as a player opens it:
    240 ticks on the module's lines, 240 on its circles, 240 with its sight pass (1,200 men asked), digest and every frame's bits those of the page with
    `?rust=0`. `words`: `?rustlines=0` leaves circles and fog. `replay`: `plain`'s 240 frames as a replay file, opened as `?playtape=`: with no word the page stands on the recipe's three rows, with `?rust=0` the word
    is on the address it is sent on to, no row is on, and the tape plays 240 of 240 steps with no man off his trace either way (a game recorded in the
    module, played back in TypeScript). `nojit`: no WebAssembly, every row off, the same game. `RUST_BREAK=1`: `loads`, `plain`
    and `systems` red. The first 240 frames of a fresh page, three pages each way: no difference a row makes shows over the page's own spread (first
    frame 169 to 226 ms either way, median 3.5 to 4.3 ms).
  - What it costs (`tickcost.ts --rounds 2`, five runs, 38,634 ticks, nothing else running; the table now says which rows ran): every row off 2.09 and
    2.07 ms a tick; `lines` alone 2.05; `lines` and `circles` 2.03; the three 2.04 and 2.02. So the three are about 2 % of a tick (3 % on the build
    before, 2.05 to 1.98), all of it the lines; circles and fog are inside the spread between two runs of one setting (0.02 ms). A first pass of these
    numbers was taken while a page check ran beside it and read 3 % SLOWER with the rows on: thrown away, and the reason for "nothing else running".
  - On the build this was first cut from (the step before the pacer's), with the three rows on by default: the gate's families on both stands, the old
    tour and a soak gave the build's own reds and no other (shipped squadplay 1, lineoffire 2, aim 5, doorpath 2, teams 6; frozen doorpath 3, teams 4;
    the tour 0 of 3; the soak's 15 violations with the same tail), line for line the clean copy's.
  NOT DONE: never seen on a GPU, and nobody has played with a row on. No NETWORKED page was opened with an off word in a browser (the replay's page was;
  a client takes the same two steps, and a server with the rows on and a seat with them off agreed on 30 of 30 state words in one headless run by the
  multiplayer task). `tour` and `soak` were not run again after the review's mends.
- **All on** (2026-11-04; the maintainer: "the plan is rust/wasm only", "let's get the rust merged asap and delete the typescript and we can always
  look at git history if needed"). `RUST_ON` is now EVERY ROW BUT `maths`: lines, circles, fog, poses, dice, sight, light, smoke, sound, paths, doors,
  falls, walk, props, slabs, items, rays. The bullet above is the list as it was the day before and why; this one is what stands.
  THE TWO STANDS NO LONGER DIFFER IN A ROW OR IN THE MATHS. Until this step the frozen stand held every row off and played on the engine's own
  `Math` (`EXACT.on` off), because every frozen digest was recorded so. But the rows that work lengths and angles out answer only under the game's own
  maths, and the TypeScript behind them is going: a stand that cannot run the game the way it will be kept holds nothing. So `standup.ts pinMaths` and
  `pinRust` put the game's own maths and the rows of `RUST_ON` on on BOTH stands, the `FROZEN_HOLDS` lines `EXACT.on` and `RUST.<row>` are gone, and the
  frozen stand's pins that fold a run were recorded again, once (below).
  THE STAND AS IT WAS RECORDED IS STILL ASKED FOR BY WORDS, in this step: `TPS_EXACT=0 TPS_RUST=0` in front of a frozen tool. `TPS_RUST=0` = every row
  down unless its own word raises it (the page's `?rust=0`); `TPS_RUST<ROW>=0|1` = that one (`?rust<row>`). The deletions end that: with the TypeScript
  twins gone a row cannot be put down, `?rust=0` does nothing and the old frozen stand cannot be raised.
  A ROW IS STILL NEVER A RULE OF PLAY: the page's stand reads the digests it read (42 of 42 missions, seeds 1 and 2 of 21). What moved the frozen
  stand's digests is the MATHS under it (the engine's sine against the game's own, in the last place), not a row.
  NOT ALL OF IT IS CHEAPER YET. A row whose callers are TypeScript pays a crossing a question. Measured with nothing else running (`tickcost.ts --each
  --list`, one round, five missions, a 4-core cloud box, this step's own head): a tick is 2.23 ms with lines, circles and fog alone and 2.14 ms with
  every row on the frozen stand, 2.24 and 2.13 ms on the page's, every run the same to the bit; the same measurement taken twice differs by 1 %. The
  rows are on because the game must be played the way it will be kept, not because each one pays today.
  `SENSE.*` (gameplay smoke and light on the coarse CPU model) is a rule of play and is NOT part of this: off, as it was.
  THE 42 (seeds 1 and 2 of 21 missions, `playthrough.ts` with digests), this step against the build before it: the page's stand 42 of 42 equal; the
  frozen stand under `TPS_EXACT=0 TPS_RUST=0` 42 of 42 equal; the frozen stand as it now is, 30 equal and 12 on other words (no ending changes kind;
  compound-four:2 is taken at 139.2 s where it was 153.8, barracks-four:1 at 144 for 143.4, mousehole-four:2 is wiped at 145.3 for 145.6).
  WHAT WAS RECORDED AGAIN (the frozen stand):
  - `tools/qa/fixtures/threshold-identity.json`: five of fourteen runs fold to other words, with the same endings at the same seconds; the other nine
    read the words they had. Under `TPS_EXACT=0 TPS_RUST=0` the fixture as it was is still green.
  - `tools/qa/slab-asitwas.ts`: the pin of the torn leaves follows its stand and keeps the recorded one askable (four levels' words).
  CHECKS THAT STOOD ON A ROW BEING OFF were mended to say their own words: `simstate-check` (stands on no row by its own word), `sense-ident` and
  `posemath-check` (put the rows of the copy down for their TypeScript arm: a red path had stopped going red once the lines' row was up by default),
  `rust-check` (its `stand` section holds both stands to the one list, each row's own word, `TPS_RUST=0` and the stand as recorded; the torn leaves'
  pin is read five ways: each stand by the module and by the TypeScript, and the frozen stand as recorded). `RUST_BREAK=1` is red for every section and row.
  THE FAMILY SWEEPS (`probes.ts all-quick`, 424 probes, each stand): on the page's stand every failing line reads word for word what it read without
  this step; on the frozen stand the one probe this step turned red is `threshold-identity`, recorded again as above.
  THE PAGE (`rust-page.cjs`, headless Chromium on the software adapter; never seen on a GPU): `plain` and `systems` play with the seventeen rows up,
  240 frames, the game in TypeScript to the bit; `nojit` still boots and plays with every row off. THE DAY THE TWINS GO, A PAGE WITH NO WEBASSEMBLY
  CANNOT PLAY, and says so on a card instead.
- **One way in** (2026-11-04; the maintainer: "let's get the rust merged asap and delete the typescript"). A row that is on sends its questions to the
  module only where the module can answer; everywhere else the TypeScript still did, quietly, which is why a twin could not be deleted. This is the step
  that finds those places and closes them, a family at a time. NO RULE OF PLAY MOVES: every answer is the same answer, by digest (the 42 on both stands).
  THE COUNT (`src/sim/sim.ts FELL`, `fell(row, why)`): `TPS_FELL=1` in front of any headless tool counts every question the TypeScript answered while its
  row was up and prints `FELL <n> × <row> · <why>` as the tool ends (`FELL none`); a play-through prints the sum of its runs. `TPS_FELL=who` adds the
  first two callers (`who6`: six), `=throw` stops at the first. On the page: `?fell`, read by `window.__sim.fell()`. Off it costs one test at a place that
  was falling back anyway. A fallback site calls `if (FELL.on) fell('<row>', '<why in plain words>')`: A FALLBACK THAT IS NOT COUNTED IS NOT KNOWN.
  THE SEAL (`sim.ts SEALED`): the rows no run may reach the TypeScript of. A tool that counted one prints `FELL SEALED ROW REACHED` and exits 1, and
  `rust-page.cjs`'s `systems` job fails on one in the real page. `FELL_BREAK=1` with `TPS_FELL=1` is the red path (questions outside a tick go back to
  the TypeScript; it must end red — and the first step of each rule of the things, and every hundredth after, is sent to its TypeScript and counted
  under its own row, `sim.ts ruleRed`, so every one of those rows that steps in the run is reached by name), and `FELL_BREAK=<row>` counts one question of that row by hand as the tool opens: red for a sealed row, not red
  for another (a family's proof that its row is on the list; a name that is no row of `RUST` is refused, and under `TPS_FELL=throw` the count
  throws like any other). SEALED TODAY: `lines`, `circles`, `fog`, `poses`, `rays`, `walk`, and the senses' `sight`, `light`, `smoke`, `sound`, `paths`, and the rules of the things: `doors`,
  `falls`, `props`, `slabs`, `items` and `a rule's step (doors, furniture, leaves, thrown things)` (`sim.ts RULES_ROW`: the copy refused to one of
  them). A family adds its row in the step that closes its last fallback, and its TypeScript can go in the step after. `rays` joined with "A pane's open question": until then a ray whose nearest thing was a pane the
  host said was open there flew on in the TypeScript walk (some five hundred a mission with glass in it).
  THE COUNT'S OWN RULES (a review's mends): `TPS_FELL=0` and `?fell=0` are off, as every word of this family; `?fell` is a word of the visit, so a
  page sent on to a recipe's address keeps it. A WINDOW IS PROVEN BY ITS FIRST QUESTION (`col.window(fn)`: one that asks nothing costs nothing), and
  the page opens one round the camera's step of a frame and round the sound's probe, which ask their lines between ticks; the `ALONE` line says how
  many questions proved the copy for themselves and how many windows a question proved. A copy that did not stand because the level does not fit is
  not asked for again until the next tick or window (the TypeScript answers, as it always did there: slower, never wrong). A game's pose pool with no
  room does not throw: the pose stays the host's own array, that man's layers are the TypeScript's and counted under `walk`, and it is tried again
  at every update (when the TypeScript goes, the cast's size is a level's limit: game/levelfit.ts has no row for it yet). Counted since: the fog's
  pass handed back by the module, a rig the bench did not take, a blend of poses of different sizes.
  WHAT WAS CLOSED FOR THE SEALED ROWS:
  - A QUESTION OUTSIDE A TICK. The module answers only over a copy of the box world that has been PROVEN (every static compared with its row), and a
    tick was the only stretch with a proof. `StaticCollision.window(fn)` proves the copy once and runs `fn` with its questions going to the module; a
    single line, circle or ray asked outside every window proves the copy for itself and shuts it behind the answer (`collision.ts lone`; `ANYTIME.proofs`
    counts those: code that asks many in a row opens a window). A tick's open is still a tick's: the window and the lone question stand the copy by
    a step of their own (`stand`), so a check that counts ticks by `tickOpen` counts ticks.
  - THE GRID WRITTEN INSIDE A TICK (`boxMirror.ts wroteGrid`). A wall holed (`addStatic`) or the list cut back used to SHUT the copy for what was
    left of the tick, and every later question of that tick, the senses' and the rules' steps among them, was the TypeScript's. It is a debt now, as
    `wrote` is, and the compare that pays it copies the cells first.
  - A RAY OF NO LENGTH meets nothing, said once at the top of `collision.ts raycast` for both walks.
  - THE STAGE (`anim/skeleton.ts stageOpen`, `stageShut`; `Character.update` and `CharacterAnimator.update` open it). An animator's layers are handed
    their poses by their places in one pool of the module, so an update's poses must STAND IN A BLOCK. A man of a ticking game has his in the game's
    block. Every other update had none and was posed by the TypeScript: a man posed as he is spawned, before any tick has opened over him; a menu's
    figure, who is in no game; a game whose block another game's tick has parked. Now a set at home has its block bound as the update opens, and a set
    with no home is lent the stage, a block of that file's own, for the one update. The same floats either way. A pool with no room is an error.
  - THE ANIMATOR'S LAYERS (the row `walk`, sealed with them). What was left under it after the stage were sizes: a rig with a list or a table
    longer than the module holds, a layer's clip the module does not hold, a reload of more stations than a path may be, a knife blow of more
    than 4 keys, a big tool's record of more than 64 blows, a table of a throw outside 2 to 16 keys. Each was a quiet "the TypeScript poses this one", and
    the last four were not even counted. None of them happens in the game (the count read none over the 42, and `layer-ident`'s `caps` holds
    the tables and every gun's reload to their sizes), so each is now a mistake said by name through one check (`anim/skeleton.ts layerFits`;
    the row's bullet, "WHO IS ASKED"), but for the big tool's record, which play makes and no table bounds: its layer is handed the two blows it reads
    (`character.ts blowsNear`), so no size is left to it. What `layerSlot` still counts: a pose the pool had no
    room for, a pose not at home, the module not answering posing questions — each only with the `poses` row up, since a row put down by its
    own word (`?rustposes=0`) makes every layer the TypeScript's by that word, which is no way back in. `layer-ident`'s `whole` line walks
    those ways out on purpose and holds the count down while it does.
  WHAT STILL LEANED ON A TWIN'S BODY WITH NO CHOICE MADE, MOVED BEFORE ANY TWIN IS DELETED (2026-11-05). Sealing the rows proved that no RUN reaches a
  twin's STEP through the place a rule chooses its side. It could not see a twin's body reached where nothing chooses: no `ruleFell` says it, so
  no count has it. Such a place is counted under a row of its own, `sim.ts TWIN_ROW` ("a twin's body, no choice made"), until it has a choice like
  any other; the 42, the soak and the tour must read none of it before a twin goes.
  - THE FURNITURE'S SETTLE AS A LEVEL IS STOOD UP IS THE MODULE'S (`props.ts settle`, `props.rs settle`, export `props_settle(g, w)`). The twelve
    passes that resolve a level's furniture out of its authoring overlaps (every thing against the statics, then thing against thing, nothing
    answered and nothing heard), and make where it comes to stand HOME, were the step's own two halves called straight (`solveStatics(r, false)`,
    `solvePairs(false)`) on every load of every level. They are those halves in `props.rs` with the same word (`dynamic`), over the things' columns
    in the game's block and the box world's copy, proven once for the lot (`collision.ts rulesStatics`: the statics alone, no leaf is asked for).
    The way back (no window on the copy: the red path, a level that does not fit) is `ruleFell('props', …)`, a sealed row. `SETTLE_PASSES` is one
    number in both sources. HELD BY `prop-ident`: for each of its five levels a second set of the furniture is made over the same statics with the
    both-ways switch up, and EVERY LEVEL FILE'S furniture is settled both ways, as it loads and as it loads lived in (111 settles; 99 things moved
    and a thing pushed out of a static 2,292 times; 0 differ); `PROPS_BREAK=1` hands the module every thing a hair wider for its settle and
    `settles differ` must not be 0 (4). The 42 by digest on both stands. A step while a level is still being settled cannot happen and throws.
  - A RAY INTO THE FURNITURE IS THE MODULE'S (`props.ts PropSystem.raycast`, about thirty callers: rounds, a device's beam, the launcher, debris,
    marks, morale, cover fire, the sticky camera, a throw's plan — 35,135 over four missions on the page's stand, 1,533 on the frozen one, every
    one answered in TypeScript until now, with `lines_ray_things` only compared under the both-ways switch). It asks `collision.ts thingsRay`:
    the copy's list of the furniture's parts as they stand (`boxMirror.ts things`: copied where a part was written since), inside a tick or a
    window as it stands, outside both proven for the one question (`lone`). This file's walk (`raycastHere`) is the both-ways arm and the counted
    way back (`ruleFell('props', …)`). WHERE A THING STANDS, WRITTEN BY ANYBODY BUT THE STEP, IS SAID (`Prop.x` and `.z` say `thingWrote`): the copy
    keeps each thing's place beside its parts for the ray's first test, and a ray asked after a rule stood a thing somewhere else and before it
    was placed again must read the place it was just given, as the walk here does (without it 1 ray in 30,000 of the check's meets another thing).
    HELD BY `prop-ident`: four rays a step, the game's road against `raycastHere` — two outside the tick straight after the rules' writes, two
    inside it, one of those after a thing was moved under it and not placed (30,000 rays at `--n 1500`: 7,631 met a thing, 15,000 outside a tick,
    2,241 things moved under one; 0 differ; `PROPS_BREAK=1` asks the module from a hair to one side: 2,052 of 12,000 differ) — and by
    `rust-check --both` over the missions as before. No question more is asked alone: the missions' count of lone proofs did not move (17,270
    over four on the page's stand). `TWIN_ROW` has no place left and is SEALED: a place found later is red from the day it is counted.
  - NO RAY IS HANDED BACK. Two kinds were (`lines.rs ray_rule`, `collision.ts raycastAsk`; a slab's step and a thrown thing's walk handed their
    whole step or row back with them): a ray "a thousand kilometres long or longer", and a ray that is no number.
    NO LENGTH IS TOO LONG: the walk of the cells ends where the ray has left the level for good — the cells round a sample lie off the grid on
    the side the ray is going, or it does not go that way at all, and a sample's cell moves one way only along a ray — the same statement in
    `lines.rs ray_but` and `collision.ts raycastHere`. The samples are where they were, so every answer is the same to the bit; a long ray that
    misses is walked as far as the level reaches (it was walked to its end, half a metre a sample). A ray that goes nowhere in plan (straight up
    or down) never leaves that way, and ends after its first sample: every later sample's cells are the first's, whose statics have each been
    asked the whole ray. One that crawls in plan is walked until it has left: long, never wrong. The gate at 1e6 is gone from both sides.
    A RAY THAT IS NO NUMBER HAS NO ANSWER, here or there: where it starts, the way it goes, or a length that is not finite (its samples are none).
    The TypeScript walk never answered one: it reads a cell that is no cell and fails. So the module still reports it (`RAY_HOST`, a rule writing
    nothing of its step), and the host NAMES THE MISTAKE where it is reported (`collision.ts noNumber`: a level ray, a slab's step, a thrown
    thing's walk) instead of handing it to a walk that fails on it without a name. `flySlab`, `runOn` and the pool's walk have no second half
    behind the module's step any more. Nothing of play asks either kind (the count over the missions, the soak and the tour read none).
    HELD BY `segment-ident --rust`: one ray in twenty-five with plan to it is asked for two thousand or a million kilometres (7,672 on the
    office alone), each the same answer both ways, and a run with none is not a fair run; the 42 by digest.
  - MORE ROOFS OVER THE SNOW THAN A WALK IS HANDED CANNOT HAPPEN, AND IS ONE NUMBER. The roofs a thrown thing's walk is handed are the WEATHER'S
    (`weather.ts roofsNow`: the roofs in force, which change with the weather picked — the weather's state, not the level's statics, so they are
    handed with each walk and are no table of the copy). The weather keeps at most four (`roofsOf`: a level's biggest; the picture's frame has a
    slot each) and a walk is handed four, which were two literals. `weatherKinds.ts MAX_ROOFS`, `itemshome.ts ITEM_ROOFS` and `items.rs MAX_ROOFS`
    are held to one number by `rust-check` (`items`), and the way back is a throw by name.

  WHAT WAS CLOSED FOR THE RULES OF THE THINGS (doors, what falls, furniture, a leaf off its hinges, thrown things; 2026-11-04):
  - EVERY WAY BACK IS SAID. Each rule's choice between the module and this side (`doors.ts there`, `props.ts there`, `doorblast.ts slabThere` and
    `flySlab`, `throwables.ts world` and its walk, `fallshome.ts rule`) names its reason through `sim.ts ruleFell(row, why)`. Not counted, because
    nothing is stepped anywhere: a level with no door, a level with no loose furniture, and a row that is idle by its own rule (the engine's maths:
    `OWN_MATHS`).
  - A GAME'S STATE IS AT HOME FROM THE MOMENT THE GAME IS MADE (`host/standup.ts standGame`, the one place a `Game` is made, says `sim.settle`
    once the die is hung: the statement every tick's open makes). Until then the stores moved into the game's block at its FIRST tick, so every
    rule that acted before it was the TypeScript's: a level staged or restarted (`FallStore.clear`, four a run), a leaf shot apart on a game just
    loaded, a checkpoint filed before a tick, a roll of the die. A change to WHEN a block is born, for every tenant, the die and the poses among
    them; no answer changes (the 42 by digest, both stands; a snapshot taken before any tick and put back replays to the same bits).
    (2026-11-05: for the five families of columns this is no longer a move at all — their stores are made over the block, "A game's block is
    taken as the game is made" — and the statement in `standGame` is the die's and the poses'.)
  - THE LEAVES' LIST IS ONE ARRAY FOR THE GAME'S LIFE (`savestate.ts`, where a state is put back). The copy knows the game's own leaves by the array
    that stood in `col.dynamicBoxes` as the tick opened; a state put back made a new array of the same boxes, and a restart taken INSIDE a tick left
    every rule of that tick's rest without the copy (the doors', the furniture's, the thrown things'). No run of the 42 does it; a twelve-minute soak
    did, ten ticks in three seeds ("another list in the leaves' place"). The list is filled anew in place now. THE SOAK AND THE TOUR ARE RUN WITH
    THE COUNT, as the 42 are: a way back that only chaos finds is still a way back.
  - LYING SNOW IS ASKED IN THE MODULE (`rust/sim/src/items.rs bog_at` = `snowBog.ts snowBogAt`). A thrown thing that bounced on ground that might be
    snow handed its row back (60 over the 42 runs, all on the snow level). The snow is data now: what snow each box of the level is, a byte a box
    (`snowBog.ts SNOW_KIND`, the world's table `snow`, compared and written by `boxMirror.ts snow` at a walk that may meet snow), read with the
    statics' own rows for where each box is and how it is turned, so nothing is kept twice; the roofs in force (at most four: `weather.ts
    roofsNow`) and the rule's numbers (`SNOWBOG`) are handed over with each walk. Held by `item-ident` on the two snow levels that have furniture,
    with the game's own rule on this side and four roofs in force (`ITEMS_BREAK=snow`: the module told another snow, which must differ).
    `ITEMS_STEPPED.host` is now a failure of that check: no row is handed back.
  - A RAY OF NO LENGTH meets nothing in a rule's ray too (`lines.rs ray_rule`), as in `collision.ts raycast`.
  - THE LANDING A CHECKPOINT FILES (`doorblast.ts slabSnapshot` → `runOn`). A slab still in the air was run on to where it lands by 480 steps of the
    TypeScript's step on a copy of its row. It is the module's step now, on the slab's own row, with the slabs' columns put back as they were and the
    box world's copy proven once for all of it. `slab-asitwas` files hundreds a run and folds them, and asks each landing again between two ticks
    (the road an autosave takes): its words did not move on either stand, and no landing differs.
  - ONE WINDOW A CALLER AS A LEVEL IS STOOD UP: `doorswing.ts dealSwings` (every leaf's room: some 1,370 circles on the office), `props.ts settle`
    (829) and `destruction.ts index` (a roof's height a thing: 265) each proved the copy a question. 2,501 lone proofs a load of the office are 38
    (the fixtures' two lines each in `Game`'s constructor, which are not this family's).
  - A LEVEL TOO BIG FOR A POOL DOES NOT LOAD (`game/levelfit.ts levelOver`, the first statement of `Game`'s constructor). A level of more than 64
    doors, 128 loose things of furniture (or 256 boxes of them), or 256 sheets of glass was stepped, or judged, in TypeScript without a word. It is
    said now before anything of the level is built, the stores refuse more rows than they have (`doorshome.ts`, `slabshome.ts`, `propshome.ts`),
    and the level validator says the same of the document whole (`levelData.ts`: doors, things, the things' boxes). THE COUNT IS ONE SOURCE:
    `levelfit.ts fitRows` is every count of a stood game against what holds it, the pools and every table of the box world's copy (`BoxMirror.tables`),
    with the share a table that grows in play may fill as a level loads; `copyfit-check` is that function over every level and is red when the copy
    has a table no row of it is held by. It reads: doors 27 of 64, things 61 of 128, sheets of glass 22 of 256, and THE FURNITURE'S BOXES 220 OF 512: they were
    220 of 256 on office-relief, with levels being dressed that week, so the copy's leaf lists are twice as long now (`world.rs MAX_LEAVES`,
    `propshome.ts PROP_PARTS`; `boxMirror.ts boot` holds the two together).
  - THE THROWN THINGS' POOL GROWS INSIDE THE MODULE. The game's rule is untouched: thirty-two things, one given up for each new one, and a can on
    its fuse or venting never given up, so thirty-two live cans grow the pool (the code's own word for that: "never, in play"). A pool that grew
    used to leave the module for good. Its columns have 64 rows now (`itemshome.ts ITEM_ROWS`, `items.rs CAP`), so it is stepped where every pool
    is. AT SIXTY-FOUR LIVE CANS THE OLDEST GIVES WAY (`throwables.ts spawn`), where the pool used to grow without end: a limit of the build in a
    corner twice as far as the one the code calls "never, in play", with no switch; nobody has played it and no run reaches it. `colstore.ts
    widen` is gone.
  STILL COUNTED WHERE THEY HAPPEN, and none of the 42 reaches one: more men at the doors, or men, leaves or door frames at the furniture, than a
  step is handed (256 each); a ray that is no number; the model viewer's stage, whose pool has no game and no block (the workbench goes before
  the TypeScript does). Each wants its answer before the twin is deleted.
  EVERY LEVEL AGAINST THE COPY'S CAPS: `bun run tools/qa/copyfit-check.ts` (`COPYFIT_BREAK=1` its red path) stands each level of `levels/` up and reads
  what it holds against the module's tables: statics (the fullest level 16 % of 8,192 on the page's stand), cells, the cells' entries, tilted statics,
  the cut rows the copy spent, leaves, furniture (61 of 128) and its part boxes on the leaves' table (220 of 512), the tightest. A level that does not fit would be a level every question of which is the
  TypeScript's, and the count says so by its reason.
  WHAT THE COUNT READS, the 42 on the page's stand (the frozen stand has only the first): `paths · a search with a penalty has no Rust` (some 8,500);
  `light · outside a tick` (5,730, one caller: an order given between ticks places the calm torch); `rays` at an open pane. Each is its family's.
  WHAT WAS RUN: the 42 on both stands with the count, digests equal to the build before; `rust-check` quick on both stands and its red path; `--both` on
  runs that hole walls inside ticks (0 answers differ, 0 stale rows); layer-ident, simstate, rollback static, knobs, dice; the page check with `?fell`
  (headless Chromium, software adapter; never seen on a GPU).
  THE SENSES' CAR (sight, light, smoke, sound, paths; the same day). No rule of play moves: the 42 on both stands are the build before, whole result
  files the same.
  - A SEARCH WITH A PENALTY IS THE MODULE'S (`rust/sim/src/nav.rs find_priced`; `collision.ts findPathThere`, `priceThese`). A penalty is a function
    of the caller's (a guard's plan, an order's cost, a lane's stepper: 31 call sites, 17 of them reached over the 42) and the module imports
    nothing, so THE SEARCH WAITS. It is written once over a source of prices (`trait Prices`: `Flat`, every price 1, is the search with no penalty;
    `Asked` reads a table the host fills). Where it comes to a cell it has no price for, it stops (`NAV.NEED`), lists the cells (the neighbours of
    the cell it is about to open, eight at most; a leg's cells in the smoothing), and goes on from where it stood once the host has written them
    (`nav_need`, `nav_resume`; `nav_drop` when the price throws). So the caller's function is called for exactly the cells this file's search
    reads, in the order it first reads them, ONCE a cell where the TypeScript calls it at every visit. That is the same search only because every
    penalty gives a cell one price all through a search: A NEW PENALTY OWES THAT, and the both-ways check holds it (below). A search that waits is
    scratch of the one call (`STATICS` rows; no snapshot can fall inside one) and the module holds one: a way or a sound asked from inside a price
    is an error while the module's search waits (`PRICED.waits`; `findPath` and `propagate` say so at their top). No penalty in the game asks one.
    A price the search finds missing where it has asked for it is the host's search (a counted hand-back), never a guess of 1.
  - BETWEEN TICKS: the light estimate (`guards.ts lightEstimateAt`: an order given between ticks places the calm torch) and the smoke's line
    (`sense.ts CpuSmoke.along`), each one question in several calls, stand in a `col.window`; an eye on a man (`collision.ts eyeContact`) is one
    call and proves the copy for itself, as a line does. Inside a tick each is as it was.
  - EVERY HAND-BACK OF THE FIVE IS COUNTED where its caller's own statements then run: a sight line that stops at glass or names a flag the copy
    does not keep, an eye handed back; the lamps handed back (a line too long to be one: more lamps than a part carries is no hand-back since
    THE LAMPS IN PARTS, below); the smoke's line handed back; the sound's straight lines and its way round handed back, a floor larger than the module's table;
    a search's cap that is no whole number, the search handed back.
  - THE LAMPS IN PARTS, AND THE SMOKE'S SHADOW IN THE MODULE (2026-11-05, the first of the roads closed so that `lampsHere` can go; the rule for
    all of them, the Rust goal's: CONTENT IS NEVER AN ERROR — what a level or a fight can hold is asked in parts or is a level's limit said before
    play, and only a mistake is an error by name). The module's table holds 256 lamps (`light.rs MAX_LAMPS`, `boxMirror.ts FACE`). More than that
    in reach of a point (a level of many lamps, a fight's flashes) used to hand the whole question back. Now `collision.ts lamp`, meeting a full
    table, sends it as a PART and begins it again, and `light.rs estimate` sums a later part (`more`) onto the six faces the one before left: one
    sum in `lampsHere`'s order, its bits, for any number of lamps. The faces are kept from part to part in a static of the light's own (`light.rs
    SUM`, a `STATICS` row), not in the shared scratch, so a question a caller's gate asks of the module between two parts cannot spoil the sum;
    the grid's block is bound again a part. `LAMPS.parts` counts the parts sent ahead. THE SHADOW (`SENSE.bounce`): where the smoke grid could not answer a lamp's line (more smoke
    than bricks; no grid standing) the module handed the question back and `lampsHere` asked the blobs. Now the host writes the field's blobs
    before the question and `light.rs` asks `smoke.rs` over them inside the lamp loop, under the model's numbers, told whether a grid stands
    (`sense.ts SmokeGrid.along`'s own first test). THE BLOBS ARE NOT CONTENT: a game keeps `SENSE.smoke.cap` of them (48) and never more than
    the module's table holds (`smoke.rs MAX_BLOBS` 64, `FACE.MAX_BLOBS`: `CpuSmoke.emit` takes the smaller), because a question carries every
    blob at once; a list longer than the table is somebody else's doing and an error by name (`collision.ts blobsFit`), and `rust-check` is red if the knob
    as it ships is past the table. The knob registry keeps no most for a knob and the debugger shows no row of `SENSE`. HELD by `sense-ident`
    (a level's last line): some six hundred lamps put in reach of forty points, the module in three parts a point against the TypeScript's one
    loop, the same to the bit; then twenty-four points through smoke with the grid answering, with no grid standing, and with more smoke than
    bricks, every point to the module, dimmed, the same to the bit. `LAMPS_BREAK=1` = every part begins its sum again: the lamps in parts must
    differ and no sense of the questions may. `LAMPS_BREAK=shade` = the module is handed no blobs: the two cases where the grid cannot answer
    must differ, and neither the parts with no smoke nor the grid that answers may (`rust-check`'s senses line runs both). NEVER ASKED: a
    question handed back in a LATER part (a line too long to be one, after a part has summed): the caller's loop then runs from the start and
    reads nothing of the module's sum, by the code and by no check. NOT A HAND-BACK AND STILL A LIMIT: THE GRID'S SWEEP OF THE
    LIGHT THAT COMES ROUND writes the level's steady lamps into the same table (`guards.ts steadyLamps`) and takes no more than 256: past that
    the field stands as it was and the bounce light stops, with no word. The most a level has today is 33 (the office; every other level 24 or
    fewer). It is to be a row of the levels' fit (`src/game/levelfit.ts`) with the deletion of `lampsHere`; until then whoever dresses a level
    with hundreds of fixtures is told here.
  WHAT THE COUNT READS NOW, the 42: `paths · a search with a penalty` and `light · outside a tick` are GONE (8,962 and 5,688 on the page's stand).
  Of the five senses: NOTHING on either stand, in the tour or in the soak, and ALL FIVE ARE SEALED (the soak's watcher asks two eyes between
  ticks: that was `sight · outside a tick` until the eye proved the copy for itself). `sound` and `paths` were counted for the first time by this
  car on a tree without the panes in the copy (the straight lines handed back 493, frozen stand 1,989; the way round 102, 178; the search 32, 5):
  each was a line that came to a pane, and behind "A pane's open question" (`collision.ts paneAsk`) each reads zero. What the module can still
  hand back of them is a line too long to be one, a point that is no number, a way of more waypoints than its table; none is met. The senses'
  TypeScript (`eyeContactHere`, `lampsHere`, the smoke's own loop, `propagateHere`, `findPathHere`) can go in the step after.
  THE CHECKS: `tools/qa/sense-ident.ts` asks half its ways with a price a cell (1, 6, 20, 0.5 or 200 by a hash, in blocks of one cell or four) and
  compares the way AND the cells each side priced in the order it first asked for them; `PRICED_BREAK=1` prices the module's cells last first and
  must end with `paths` differing and nothing else (`rust-check` runs it and is red if it is not). `rust-check --both` holds the same in the
  missions (`collision.ts findPathAsk`): there this file's search asks the caller's function at every visit, as it does with the row off, and a
  price that is not the one the module was given for that cell is a differ, which is the one-price rule as a standing check (`sense-ident` runs
  its two paths on every level: an honest price is the same answer, a price that changes at every ask is caught). The numbers a search answers
  that are no count (`boxMirror.ts NAV`) are held to `nav.rs` with `FACE`. `TPS_FELL=1` prints `PRICED <n> searches … came back for prices <k>
  times … <c> cells`.
  WHAT IT COSTS: 8,962 priced searches over the 42 came back for prices 1.42 million times for 2.08 million cells (232 cells a search; the frozen
  stand 2,746 searches, 608 cells each). A tick: no difference `tickcost` can see (five runs, base then car, measured three times: the mean of all five +5 %, +2 %, −2 %, and on the car as it went 3.77 → 3.78 ms with each
  run between −2 % and +3 %; the same tree
  measured twice differs by 3 %).
  WHAT WAS RUN (headless, a cloud box, no GPU; NEVER SEEN ON A GPU and nobody has played it). ON THE CAR BEFORE THE PANES WERE UNDER IT: the 42 on
  both stands with the count, each stand's result file the base's to the byte; `sense-ident` on every level, ALL PASS, `RUST_BREAK=1` red for
  every sense, `PRICED_BREAK=1` red for the paths and nothing else, the two together refused; `FELL_BREAK=1` red; `rust-check` on both stands and
  `RUST_BREAK=1` red for every section; `rust-check --both`, 8 runs, 28.0 million questions, 0 differ, 0 stale; `rollback-check` whole and
  `--static`, `simstate-check`, `knobs-check`, `dice-check`, `copyfit-check`, both typechecks, `build.ts --check`; the tour (3 runs) and the soak
  (seeds 1,2,3, 12 minutes) on both stands with the count: green on the frozen stand, and on the page's stand the reds of the base, the same ones
  (the tour's fire exit, stun and smoke stops; the soak's 15 violations). The page itself in headless Chromium on the software adapter
  (`rust-page.cjs`, the jobs loads, plain and systems; its frames are not drawn): the game in TypeScript to the bit with every row up, and with
  `?fell` the TypeScript answered none. BEHIND THE PANES, AS IT IS CUT: the 42 on both stands with the count, the same result files to the byte and
  `FELL none` on both; the tour and the soak on both stands, nothing of a sense counted (the soak's one line is a rule's step, 22 on the page's
  stand and 6 on the frozen one); a run on the stand as it was recorded (`TPS_EXACT=0 TPS_RUST=0`) counts nothing; `FELL_BREAK=sound` and
  `FELL_BREAK=paths` each end red; `sense-ident` and its two red paths; both typechecks.
- **The data's layout** (2026-11-03; the maintainer: "we should also start changing our data layout to be more cache and simd friendly"). MEASURED
  FIRST, one blend of two 55-node poses in a module, an x86 cloud box, bun 1.4 | node 24: as it stood (ten singles a node, worked in doubles) 1334 |
  1391 ns · the same arithmetic over columns 1334 | 1390 · columns, two lanes of doubles, THE SAME BITS 1204 | 964 · doubles, the length a plain root
  832 | 916 · singles, four lanes, over the store as it stood (four nodes turned into columns on the way in and back on the way out) 730 | 287 ·
  singles, four lanes, columns, the compensated length 299 | 291 · singles, four lanes, columns, a plain root 157 | 166. WHAT IT SAYS: this game's
  data is small (a pose 2 KB, a level's boxes 100 KB, 16 men), so it sits in the nearest caches however it is laid out and COLUMNS ALONE GAIN
  NOTHING AND COST NOTHING; lanes of doubles gain little where the work is divides and a root (the compensated length is eight divides and a root a
  node); columns and singles pay only TOGETHER. So the layout is what lets a lane kernel pay, and whether a kernel may be worked in singles is a
  rule of play (it moves the last bits: a switch, the maintainer's call). Not measured: Apple silicon; the sampler and the run forward.
  - THE RULES for a family's state (every part written from here on; a part already in the build is re-laid with its owner):
    1. ROWS ARE DENSE: a thing is a row 0..n-1, the live ones first, and a count; whoever names a row from outside names it by an id that does not
       move and a small id-to-row table.
    2. A COLUMN IS AS NARROW AS ITS VALUES: a family's truths are bits of one `u32`, an id or an index a `u16` or `u32`, a kind a `u8`; a clock, a
       length or an angle stays a double while its rule is worked in doubles.
    3. HOT BEFORE COLD, and what changes every tick apart from what seldom does (a snapshot shares the 4 KB pieces that stood still).
    4. WHOLE VECTORS: a column starts on a 16-byte boundary and holds a multiple of 4 rows, so a kernel has no tail. The rows past the last thing
       are zeros and a kernel that works them must leave them zeros (they are state: a snapshot folds them).
    5. A COMPONENT A COLUMN (x[], y[], z[]), wherever a kernel does one thing to many rows; what it needs worked out (the cos and sin of a yaw) is a
       column beside its source.
    6. WALK ORDER: what a kernel visits together lies together in the order it visits.
    7. Fixed-width SIMD only (the line, above), and a rule of play is not re-defined in singles to get four lanes without the maintainer's word.
  - **A POSE IS TEN COLUMNS** (`src/anim/skeleton.ts Pose`, `rust/sim/src/skeleton.rs rows`): channel c of node i is `cols[c * rows + i]`, `rows` =
    the nodes rounded up to 4 (55 → 56), where it was `data[i * 10 + c]`. In an array of the pose's own and in the game's block alike (the pool hands
    out `singles(nodes)`, and every pose starts on a 16-byte boundary: `state.rs` asserts the part does). The accessor was RENAMED (`data` → `cols`)
    so that the type check names every reader: a reader left on the old index would have compiled and read another node's numbers. A rotation's
    four numbers stand `rows` apart, so the helpers that take "where a rotation is" take that stride (`character.ts qAng`, `qTurnBy`, `qTurnTo`;
    `skeleton.rs store_unit`). A clip's track keeps its own number of where it writes (node × 10 + channel: what a clip is known by) and a sample
    turns it into the place in the pose it is writing (`Track.node`, `Track.chan`). WHAT COLUMNS TOOK AWAY, and what stands in its place: a node
    past the pose used to fall off the end of the array and write nothing; in columns that place is another node's, so `sampleClip` skips a
    track of a node the pose has not, `setRot` / `setPos` / `rotateLocal` / `rotateModel` / `setModelRot` write nothing for a node that is none
    (as `skeleton.rs ask_turn` always did), `restPose` and `Pose.copy` throw on a pose of another size, `character.ts sampleAt0` keeps the row
    count its slots were found for, and the bench reads back only the nodes' rows (the rows past them are another question's leavings there).
    The index is WRITTEN OUT at each site (`d[3 * N + i]`), not asked of a helper: a call a single is what the TypeScript twin pays for most.
    The price is that a tool which reads a pose through `any` is not named by the type check: `grep -n 'cols\b' tools/qa` finds them
    (`pose-ident`, `simstate-check`, `stealthpose-check`, `moves-probes`, `pose-fold`). The poses' part of the block is version 2 and the
    module's version went up with it, so a block or a built.ts from before is refused, not read sideways.
  - THE ANIMATOR'S LAYERS (`rust/sim/src/layers.rs`, the walk's row) reach a pose's single through `ch` / `set_ch` and nothing else, so the
    columns were those two lines there (`c * (d.len() / NODE) + i`: a layer is handed exactly a pose's singles, `skeleton.rs with_pose`, so its
    rows are their count over NODE), the slice `with_pose` hands over (`singles(n)`), and the stride in `set_unit` and `Fk::part`. On the
    TypeScript side `skeleton.ts layerBoth` compares `pose.cols` and names a differing single by node and column, the jog's seam keeps a node and
    no offset, and `handsPassOwn` reads `cols` with `N = rows`. Nothing inside a layer changed.
  - NO RULE CHANGED and no switch: the same game to the bit. WHAT WAS RUN, all headless (bun 1.4 and node 24, a cloud box), on the build's head of
    2026-11-03 with the rows that are on: `pose-ident` (the module against the TypeScript on the functions the module answers: 204,131 questions,
    none differing, red under `RUST_BREAK=1`; its node-past-the-pose case now holds the pose it must give); `layer-ident` (every layer the same
    pose both ways, red under the break word); `rust-check --missions --rows poses,walk` to the bit under bun and node, and with every row;
    `tickcost --each --list` on the five missions against the build before, 38,634 ticks TO THE BIT with the rows as the page has them and with
    `TPS_RUSTPOSES=1 TPS_RUSTWALK=1`, 2.01 and 2.02 ms a tick (and 1.97 and 1.98 with those two rows up): nothing, as measured before it was
    built; and, because the run's bits fold where a man is and not how he is posed, `tools/qa/pose-fold.ts` (NEW: every kept pose, skinning
    matrix, bone and ragdoll particle of every character after every tick, in one word) on hostage-rescue:1 and compound-four:1 (shipped stand;
    the first also with the poses' row up, and with the poses' and the walk's) and warehouse-four:1 (frozen): the build before and this one print
    the same three words. The animator's families on BOTH stands (`moves`, `hands`, `kitanim`, `stealthpose`, `wall-pose`, `gunhand`,
    `handwork`: no FAIL line), `rollback-check` on both stands with `TPS_RUSTPOSES=1 TPS_RUSTWALK=1`, `seats-check`, `simstate-check`,
    `savefile-check` on both stands (a save holds no pose: a man is posed again on a load), `rust-check` (red under the break word), and the
    page (`rust-page.cjs`, Chromium on the software adapter, the frames not drawn: seven jobs, 240 frames the same by digest and bits with every
    row up, as a player opens it, with the maths alone, and with no WebAssembly). NEVER SEEN ON A GPU: `SkeletonInstance.update` (the skinning
    matrices the picture draws from) was re-indexed, and what holds it is the fold above, not an eye. One seed a run.
- **A pose's bulk arithmetic in singles** (2026-11-04; the maintainer on 2026-11-03, asked which arithmetic the kernels a pose is blended and sampled by
  should use: "Singles, plain root for lengths"). THE SWITCH: `src/anim/skeleton.ts POSEMATH.singles`. On, `blendPose` and the step `sampleClip` takes
  between two keys are worked in SINGLES, every operation rounded once, and a rotation is made a unit again by the plain root of its squares; off, in
  doubles with the game's compensated length, the build as it was to the bit. A pose's numbers are stored as singles either way, so what moves is their
  LAST PLACE: it is a rule of play, not a row. ON in the page (`?posesingles=0` = doubles), OFF in the module and on the frozen stand
  (`TPS_POSESINGLES=1`; `FROZEN_HOLDS` `POSEMATH.singles`); registered in `knobs.ts` and carried by a recording, and one from before the table replays
  in doubles (`journal.ts applyKnobs`, `recipestand.ts`, `rerun.ts`; `replay.ts` says no word for it, the line below is why). Not in the
  frozen-digest pin lists: off in the module. WHAT AN ANIMATOR MEASURES OFF ITS CLIPS AS IT IS MADE (the run clip's pelvis table, the stances, the
  stride's middle) IS MEASURED IN DOUBLES UNDER EITHER RULE (the animator's constructor puts the rule off while it measures): those tables are authored data read off the file, so nothing
  kept from the build stands on the other arithmetic, and the rule written on a standing game before its first tick is the game stood with it so,
  both ways (`posemath-check late`). So a recording's rule is written by `applyKnobs` after the game is stood, like any knob; the stands pin it
  before the game all the same (`standup.ts pinPoseMath`), which is what a stand's own word means.
  - WHY: four singles fit a lane of a vector where two doubles do, and the compensated length is eight divides and a root a node. Measured before it
    was built (the layout bullet above): one blend of two 55-node poses 1334 ns in the module as it stood, 157 ns in singles over columns, four nodes a
    step. Singles change the last place, which is why it needed his word.
  - BOTH LANGUAGES, ONE RULE. The TypeScript twin rounds each step with `Math.fround` (an add, a subtract, a multiply, a divide and a root of singles
    worked in doubles and rounded once are the single's own answer: the standard's, and 20,000 blends said so before the build). The module's are
    singles to begin with: `rust/sim/src/single.rs`, the one file that may name a single, now also the one place anything is WORKED in singles
    (`unit_step`, the rule one rotation at a time; `step3`, `step4` for a sample's step; `blend4`, the blend: four rows a step in fixed-width SIMD,
    `core::arch::wasm32`, the module's first vector instructions, with a one-at-a-time body where the crate is compiled for anything else). A blend by
    nought or by one with no mask is a copy (a layer's "blend by one" is how it copies a pose); only the nodes' rows are ever written. The ORDER of the
    operations is the rule (in TypeScript written out at its two sites, `sampleClip` and `blendPose`: one helper for both doubled a blend's cost): a dot product is ((ax·bx + ay·by) + az·bz) + aw·bw,
    a length the root of ((x² + y²) + z²) + w². Where a time falls among a clip's keys is found in doubles as ever; the fraction is rounded to a
    single once.
  - WHO SAYS THE RULE TO THE MODULE: whoever asks. The host's own blend and sample read the switch as they ask and hand it in the call
    (`pose_blend`, `pose_sample`: bit 2 of `how`), so a question asked outside a tick, by a tool or by a page with only the poses' row up, is never
    answered by a stale copy. A LAYER of the module (`layers.rs`) blends and samples from inside the module and reads the tick's knob
    (`knobs.rs POSE_SINGLES`, handed over as the tick opens with every other knob). Both read the one table as the tick stands.
  - WHAT IT IS NOT: `character.ts addPoseDelta`, the turns, the solve and the run forward stay in doubles. The run forward is a chain (a child after
    its parent), so lanes do not fall out of it; it would move every bone the game reads, and is its own question.
  - THE CHECK: `tools/qa/posemath-check.ts`, on BOTH stands (`stand`, `old`, `twin`, `rule`, `late` on both; `layers` and `game` on the shipped stand,
    where the module answers); `POSEMATH_BREAK=1` must exit 1 (the module is asked without the rule's bit while the twin works in singles). It is in no
    runner: run it by name, as `rust-check` is. `rust-check`'s `poses` section asks `pose-ident` under both rules, and `pose-ident` holds the sample
    three ways under each (its plain definition has the rule in its own words: every result put through a Float32Array of one).
  WHAT WAS RUN (2026-11-04, headless, a 4-core cloud box, bun 1.4 and node 24; one seed a run):
  - The two languages: `pose-ident` with `TPS_POSESINGLES=1`, 204,131 questions, 0 differ (the TypeScript twin, the module with its lanes, and the
    plain definition), and red under either break word; without the word, 0 differ as ever. `layer-ident` under both rules: every layer the same pose.
  - Whole games: `rust-check --missions` with the rule on, six runs (one on Hard), 35,957 ticks, the poses' and the walk's rows up against every row
    off: to the bit under bun and under node, and the six end words are the same on the two engines. `posemath-check` on both stands: ALL PASS, and
    exit 1 under `POSEMATH_BREAK=1` on both.
  - OFF IS THE BUILD AS IT WAS: with `TPS_POSESINGLES=0` the five missions of `tickcost` are the build before to the bit (38,634 ticks).
  - ON IS ANOTHER GAME, as a last place moved must be: the five missions end differently (block-squads is over at tick 3,145 where it ran 8,581), so
    a tick's cost before and after cannot be read off them, and was not. What a call costs, ns (bun | node): the TypeScript blend 1322 to 871 | 1451
    to 1686, masked 957 to 661 | 1429 to 1414, a sample 1592 to 1394 | 2613 to 2679; the module with the pose at home, blend 1460 to 456 | 1418 to
    362, masked 1112 to 469 | 1064 to 379, a sample 1221 to 952 | 1189 to 951. So in TypeScript the rule is a gain under bun (the plain root) and
    about nothing under V8; in the module a blend is a third to a quarter of what it was, most of what is left being the call itself (157 ns
    inside the module). A mission asks about 73 pose questions a tick, so today this is about 1 % of a tick where the poses' row is up and
    nothing where it is not. It pays as the animator's own blends move into the module.
  NOT DONE: a clip's keys are still a track after a track, so a sample's step is one track at a time (in singles, but not in lanes); laying keys
  out as columns moves no bit and is the next step. Never seen on a GPU: the picture's skinning reads these poses, a last place apart. Nobody has
  played it. The rule written on a game in the middle of a fight is not held by a check: only before its first tick.
- **Found, not changed.** `tools/qa/exact-check.ts` is red on the build this was cut from, on three sites that are nobody's Rust: a `Math.tan` worked out as
  `scene/round.ts` loads, and a cube and a power of two written with `**` in `game/stalk.ts` and `game/showwhy.ts`. Each is the engine's arithmetic inside
  the game, the thing this work exists to end; they are their owners' to mend and a port of those files must not copy them.
  Also: `exact.ts log10` gives the exponent exactly for every decimal power that is a normal double (1e-307 … 1e308); for the subnormal ones
  (1e-308 and smaller) its own guard misses by a few 1e-9 and the answer stands a hair off. The Rust port does the same to the bit; nothing in the game takes
  the logarithm of a number that small.
- **Not seen.** Nothing here draws, so nothing here was seen on a GPU; the page check runs the bundle on the software adapter with the frames undrawn. Nobody
  has built the crate on a Mac or on Windows.
