// A MAN'S STATE, FIELD BY FIELD — THE ONE SCHEMA. Every man of a game (a guard, a teammate, the player, a civilian) has one row of the `men` part of the game's block
// (rust/sim/src/state.rs, src/sim/state.ts), and this table says what the columns of that row are. `bun run tools/sim/men.ts` writes the two sides from it:
//   src/sim/men/cols.ts     the TypeScript game's accessors (a man reads and writes himself through them, with no copy)
//   rust/sim/src/men.rs     the Rust struct of columns (a rule there reads the same bytes, with no copy)
// Both are written, never edited. tools/qa/men-check.ts is red when they are not this table, and every game holds the two sides' hashes to each other as it starts.
//
// A ROW OF THE TABLE:
//   name    what the game calls it. A `vec3` makes three columns, `<name>X`, `<name>Y`, `<name>Z`, and an accessor for each; `vec3` and `vec3?` also read as a whole
//           `<name>`, a fresh array each read (never a view: nothing keeps a view of the block).
//   on      whose accessor it is: 'man' = every man (the player and every Guard record, through his Character too); 'guard' or 'player' = that record's; 'anim' = the
//           animator's kept state (CharacterAnimator's clocks, weights and phases, a group moved by the part that moves the animator).
//   group   'hot' read every tick by the fights (position, facing, velocity, stance, health, alert state, goal) · 'cold' the rest · 'animator' the animator's. The
//           columns are laid out hot before cold, so a walk over the men reads the fewest lines.
//   type    'f64' a double · 'u32' / 'i32' a word · 'bool' a bit of a word of truths · 'vec3' three doubles · 'vec3?' three doubles and a bit that says there are any ·
//           an array of strings, a word holding the place in that list.
//   digest  game/digest.ts reads it, so a run's digest holds it.
//   init    what a new man's column starts at, when it is not zero (false, the first word of a list).
//   why     what it is, when its name does not say.
// A NUMBER A DIGEST READS IS A DOUBLE: the game's rules are worked out in doubles, a single would round them (rust/sim/src/lib.rs says the same of the crate).
//
// STAYS WITH THE OBJECT: `KEPT` names a field that belongs to a man and is NOT a column, and why: 'dressing' only the picture, the sound or the HUD reads it · 'object' it
// holds what a column cannot (a list, a closure, a record that has its own family to move with).
export type ColType = 'f64' | 'u32' | 'i32' | 'bool' | 'vec3' | 'vec3?' | readonly (string | null)[];
export type ColOn = 'man' | 'guard' | 'player' | 'anim';
export type ColGroup = 'hot' | 'cold' | 'animator';
export interface MenField { readonly name: string; readonly on: ColOn; readonly group: ColGroup; readonly type: ColType; readonly digest: boolean; readonly why?: string; readonly init?: number | boolean | string }
export interface MenKept { readonly name: string; readonly on: ColOn; readonly stays: 'dressing' | 'object'; readonly why?: string }

/** rows of the part: twice the men a game can hold at once (game/levelfit.ts menHeld, its `manrows` row: the doors' and the furniture's hand-over say 32), a multiple of four */
export const MEN_ROWS = 64;

export const MEN_FIELDS: readonly MenField[] = [
  // WHERE HE IS AND HOW HE IS MOVING
  { name: 'pos', on: 'man', group: 'hot', type: 'vec3', digest: true },
  { name: 'vel', on: 'man', group: 'hot', type: 'vec3', digest: true },
  { name: 'bodyYaw', on: 'man', group: 'hot', type: 'f64', digest: true },
  { name: 'aimYaw', on: 'man', group: 'hot', type: 'f64', digest: true },
  { name: 'aimPitch', on: 'man', group: 'hot', type: 'f64', digest: true },
  { name: 'radius', on: 'man', group: 'hot', type: 'f64', digest: false, init: 0.3 },
  { name: 'stepPhase', on: 'man', group: 'hot', type: 'f64', digest: false, why: 'where his walk stands in its cycle' },
  { name: 'speed', on: 'guard', group: 'hot', type: 'f64', digest: true, why: 'how fast his feet are going (m/s)' },
  { name: 'lastPos', on: 'guard', group: 'hot', type: 'vec3', digest: false, why: 'where his watchdog last saw him' },
  { name: 'speedSm', on: 'player', group: 'hot', type: 'f64', digest: false, why: 'his speed, eased' },
  { name: 'stepTravel', on: 'player', group: 'cold', type: 'f64', digest: false, init: 1, why: 'how far the walk cycle has come since the last footfall filed, in turns, either way added up, 1 at the most (player.ts FOOTFALL.once; 1 = far enough, which is where it starts, and where it stays while the key has never been on)' },
  { name: 'stepOwed', on: 'player', group: 'cold', type: 'f64', digest: false, why: 'how far the walk cycle has come at a footfall\'s speed since the last edge of it passed or footfall filed, in turns, either way added up (player.ts FOOTFALL.owed: at half a turn a footfall is owed; 0 where it starts, when he slows, after frames the footfall test was not asked, and while the key is down)' },
  { name: 'stepAt', on: 'player', group: 'cold', type: 'f64', digest: false, init: -1, why: 'the game clock the footfall test was last asked at with the key up (-1: never)' },
  // WHO HAS THIS MAN: the one record the "does X have this man?" questions become. Written and read by the game's owner rules (they are not moved here: they arrive empty).
  { name: 'owner', on: 'man', group: 'cold', type: 'u32', digest: false, why: 'which rule has him (0: nobody)' },
  { name: 'rank', on: 'man', group: 'cold', type: 'u32', digest: false, why: 'how hard that rule holds him against another' },
  { name: 'reason', on: 'man', group: 'cold', type: 'u32', digest: false, why: 'why it has him' },
  { name: 'since', on: 'man', group: 'cold', type: 'f64', digest: false, why: 'the game time it took him' },
  // HOW HE IS: alive, hurt, down, his stance and his hands
  {"name": "alive", "on": "man", "group": "hot", "type": "bool", "digest": false, "init": true},
  {"name": "state", "on": "guard", "group": "hot", "type": ["patrol", "suspicious", "alert", "search", "dead"], "digest": true},
  {"name": "down", "on": "player", "group": "hot", "type": "bool", "digest": false, "why": "one 9 mm hit and you are down, same as the guards; Enter restarts the encounter"},
  {"name": "crouch", "on": "player", "group": "hot", "type": "bool", "digest": true},
  {"name": "sprinting", "on": "player", "group": "hot", "type": "bool", "digest": false},
  {"name": "backpedal", "on": "player", "group": "hot", "type": "bool", "digest": false},
  {"name": "aimHeld", "on": "player", "group": "hot", "type": "bool", "digest": false},
  {"name": "holstered", "on": "player", "group": "hot", "type": "bool", "digest": true, "init": true, "why": "he starts with the pistol on the thigh (playtest call: hands free by default — Space grabs, a click behind a man is the takedown); E puts the selected item away / draws it again (Chaos Theory: equip / unequip); holstered + LMB behind a guard in reach = the silent takedown (its one and only input), the pistol's paths all check it"},
  {"name": "hitsLeft", "on": "player", "group": "hot", "type": "f64", "digest": false, "init": 1, "why": "one 9 mm hit and you are down, same as the guards; Enter restarts the encounter"},
  {"name": "hurtAt", "on": "man", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "game time a blast left him hurt (condition.ts: -1 = never; read through isHurt)"},
  {"name": "staggerUntil", "on": "man", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "a ring airfoil took him in the body (launcher.ts staggerGuard): doubled over till this time — rooted like the dazzle (reactT), no round from him, but he can SEE, and he comes out of it alert toward the side it came from"},
  {"name": "dazzledUntil", "on": "man", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "stun canister with line of sight: no vision, no shooting, rooted to the spot until this time (see Game.detonateStun)"},
  {"name": "fireCd", "on": "man", "group": "hot", "type": "f64", "digest": false, "why": "What he carries and its state (weapons.ts: the row from his route — the 9 mm service pistol unless the level says otherwise — with its chamber and magazine; guards.ts fires it and reads its cadence, fireWeapon its report / flash / spread, killGuard what falls out of the hand). fireCd: seconds until the trigger will go again (the AI's beat: cadence, the reaction on going alert, the shield's floor)."},
  {"name": "muzzleAhead", "on": "man", "group": "cold", "type": "f64", "digest": false, "init": 0.26, "why": "metres along gunDir from the wrist to the muzzle: the row's (weapons.ts WeaponDef.muzzleAhead — 0.26 for a pistol; whoever hands him a weapon writes it — or, for a man dealt the carbine over his sidearm row (game.ts, tune.carbineGuards), rigProps.carbineMuzzleAhead's 0.5 off the drawn prop). The round, the flash, the laser and armThrough all start at `muzzle`."},
  {"name": "diedAt", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1},
  {"name": "downKind", "on": "guard", "group": "cold", "type": [null, "shot", "struck", "choked", "wounded"], "digest": false, "why": "(dead) how he went down and by whose hand — set once by combat.ts killGuard; null while he lives. isBreathing() reads it: struck / choked = out cold."},
  {"name": "downBy", "on": "guard", "group": "cold", "type": [null, "player", "guard", "world"], "digest": false, "why": "(dead) how he went down and by whose hand — set once by combat.ts killGuard; null while he lives. isBreathing() reads it: struck / choked = out cold."},
  {"name": "killerId", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "(dead) the id of the character whose round put him down, −1 = not by a round / unknown (combat.ts fireWeapon stamps it; the engagement record names him)"},
  {"name": "found", "on": "guard", "group": "cold", "type": "bool", "digest": false, "why": "(dead) somebody has discovered this body — counted once for the debrief"},
  {"name": "missedAt", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "(dead, not found) the net has called this man and got nothing back — when (guards.ts updateRollcall; -1 = never): from then on the others count him as missing (superviseLockdown stops dealing his corpse a role) — and whether the floor has already been raised once on his account (it is, once, when the man sent round his route comes back with nothing)"},
  {"name": "missingRaised", "on": "guard", "group": "cold", "type": "bool", "digest": false, "why": "(dead, not found) the net has called this man and got nothing back — when (guards.ts updateRollcall; -1 = never): from then on the others count him as missing (superviseLockdown stops dealing his corpse a role) — and whether the floor has already been raised once on his account (it is, once, when the man sent round his route comes back with nothing)"},
  {"name": "muzzleDown", "on": "guard", "group": "hot", "type": "bool", "digest": false, "why": "muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said "},
  {"name": "muzzleClearT", "on": "guard", "group": "cold", "type": "f64", "digest": false, "why": "muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said "},
  {"name": "muzzleEvalT", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said "},
  {"name": "lineBarkT", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -100, "why": "muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said "},
  {"name": "lineInT", "on": "guard", "group": "cold", "type": "f64", "digest": false, "why": "alert, licensed to shoot: seconds a colleague has stood in the line of the shot itself (a man crossing is waited out for a beat; one standing there is stepped round)"},
  {"name": "gunLostAt", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "a LIVING man whose gun was knocked from his hand at the grab (HOLD.dropGun): dropped = that gun, this = when — his hands are empty (guards.ts inHand) and its light dies on the floor; −1 again once he has it back (player.ts handBack)"},
  {"name": "torchLostAt", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "a LIVING man whose torch was knocked from his hand (the grab): dropped = that torch, this = when — the beam follows it down and sputters out (guards.ts updateFlashlight); on patrol afterwards his hand is empty"},
  {"name": "lightDeadUntil", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1},
  {"name": "lightLitAtLoss", "on": "guard", "group": "cold", "type": "bool", "digest": false, "init": true, "why": "was his light burning the moment it left his hand (killed / grabbed)? a dark weapon light must not sputter ON as it tumbles (review 08-10 #7: the breach hostiles' unlit lights swept the sunlit room the frame each died)"},
  {"name": "doorDownUntil", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "a booted door met him where he stood right at it (doorthrow.ts): on a knee till this time, inside the fold the same blow gave him"},
  {"name": "trained", "on": "guard", "group": "cold", "type": "bool", "digest": false, "why": "a trained man (the response team, game/qrf.ts): the aim-settle model lines him up quicker and his hands are steadier (aim.ts TRAINED — × the settle time, × his row's cone), and qrf.ts keeps his reaction beats short. False for the night staff and the showcase's men: their numbers are untouched."},
  {"name": "god", "on": "guard", "group": "cold", "type": "bool", "digest": false, "why": "scripted, SHOWCASE CAST ONLY (director.ts LoadoutSpec.god — the day showcase's two operatives): a round that would have found him passes through him as it passes a colleague (combat.ts fireWeapon). The show's doctrine keeps the opposition's rounds off them by construction (laid on their cover, traced: tools/qa/day-probes.ts); this is the net under a stray, never the mechanism, and nothing the play"},
  {"name": "carbine", "on": "guard", "group": "cold", "type": "bool", "digest": false, "why": "rifles (docs/internal/sc20k-design.md §3): dealt the carbine by the sandbox knob (tune.carbineGuards, written by Game.update each frame) — and the deal IS the weapon: while it stands, `weapon` is the AKS-74U row (bursts, the 30-round box, the harder report, the rifle that drops — guards.ts burstTrigger, combat.ts), the RIG in his hands the carbine and his poses the long gun's; dealt away again, hi"},
  {"name": "drawn", "on": "guard", "group": "hot", "type": "bool", "digest": false, "why": "alarm escalation (guards.ts updateEscalation): drawn = the sidearm is out on patrol (level ≥ 1) — handProps / attachFlashlight / killGuard read it where they used to read 'patrol ⇒ torch'; leader = lockdown pairing: trail this man while calm (followT: re-plan clock, ≤ 2 path plans a second; followGoal: his station behind him); post = lockdown: hold this corridor junction instead of walking the rou"},
  {"name": "reloadT", "on": "guard", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "What he carries and its state (weapons.ts: the row from his route — the 9 mm service pistol unless the level says otherwise — with its chamber and magazine; guards.ts fires it and reads its cadence, fireWeapon its report / flash / spread, killGuard what falls out of the hand). fireCd: seconds until the trigger will go again (the AI's beat: cadence, the reaction on going alert, the shield's floor)."},
  {"name": "lastFireT", "on": "player", "group": "cold", "type": "f64", "digest": false, "init": -10},
  {"name": "friendLow", "on": "player", "group": "cold", "type": "bool", "digest": false, "why": "friends.ts: his raised gun is down for a teammate in its line (FRIENDS.lowReady), and how long that line has been clear"},
  {"name": "friendT", "on": "player", "group": "cold", "type": "f64", "digest": false, "init": -10, "why": "friends.ts: his raised gun is down for a teammate in its line (FRIENDS.lowReady), and how long that line has been clear"},
  {"name": "noise", "on": "player", "group": "hot", "type": "f64", "digest": false, "why": "HUD sound meter: the loudest thing you did lately (hearing-model level, decays)"},
  {"name": "hitFlash", "on": "player", "group": "cold", "type": "f64", "digest": false},
  {"name": "stunPosed", "on": "player", "group": "cold", "type": "bool", "digest": false, "why": "a blast that left him up (samstun.ts): folded until / blind until, on the game clock, and where the flash was — the same two stamps a Guard carries"},
  {"name": "doorBashAt", "on": "player", "group": "cold", "type": "f64", "digest": false, "init": -10, "why": "a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock"},
  {"name": "doorBashLean", "on": "player", "group": "cold", "type": "f64", "digest": false, "why": "a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock"},
  {"name": "doorRipUntil", "on": "player", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock"},
  {"name": "doorKickTail", "on": "player", "group": "cold", "type": "bool", "digest": false, "why": "a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock"},
  {"name": "doorHead", "on": "player", "group": "cold", "type": "f64", "digest": false, "why": "the look his head was last given against that turn, and the game time it was given on (player.ts's sprint facing; game.ts takes it back off the frame nobody renews it)"},
  {"name": "doorHeadT", "on": "player", "group": "cold", "type": "f64", "digest": false, "init": -1, "why": "the look his head was last given against that turn, and the game time it was given on (player.ts's sprint facing; game.ts takes it back off the frame nobody renews it)"},
  {"name": "dragYaw", "on": "player", "group": "cold", "type": "f64", "digest": false, "why": "the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor"},
  {"name": "dragNoiseT", "on": "player", "group": "cold", "type": "f64", "digest": false, "why": "the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor"},
  {"name": "dragStuckT", "on": "player", "group": "cold", "type": "f64", "digest": false, "why": "the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor"},
  {"name": "equipT", "on": "player", "group": "cold", "type": "f64", "digest": false, "why": "seconds left of the rifle coming off the sling into the hands (or of any change of what is in them that involves it): no round or ring meanwhile"},
  {"name": "pace", "on": "player", "group": "cold", "type": "f64", "digest": false, "init": 1, "why": "walking-pace multiplier set with the mouse wheel, PACE_MIN‥1 of the walk / crouch-walk speed (sprint is its own held modifier); shown as ticks in the kit column"},
  {"name": "paceShownT", "on": "player", "group": "cold", "type": "f64", "digest": false, "init": -10, "why": "walking-pace multiplier set with the mouse wheel, PACE_MIN‥1 of the walk / crouch-walk speed (sprint is its own held modifier); shown as ticks in the kit column"},
  {"name": "nv", "on": "player", "group": "cold", "type": "bool", "digest": false},
  {"name": "nvAmount", "on": "player", "group": "cold", "type": "f64", "digest": false},
];

export const MEN_KEPT: readonly MenKept[] = [];
