# The AI and the data it runs on: one design

Written 2026-11-07 on the tree where the Rust game first stood alone, after two reviews (section 13). Built on it so far: the list and its
arbiter, the four old body rules in the list's vocabulary, `update_guard` cut into parts, one worked example of a new rule (section 12).
Counts are greps over `rust/game/src` body files unless said. Where each per-man table goes: `docs/internal/ai-rows.md`.

## 1. What is wrong today, measured

The Rust game is the TypeScript design carried over line for line. It plays right. It is hard to change, for
five reasons that can each be counted.

1. **A man is spread over about 80 modules.** `Guard` has 130 fields. Beside it, 269 tables keyed by a man live
   in 80 modules' own state (`doortools` 28, `wallcharge` 20, `lanes` 15, `guards` 13, `enemysquads` 12, ...).
   A room fight adds a `Def` or an `Op` row, an order an `Assignment`, the press a `Man`.
2. **Everyone writes his mind.** `awareness` is assigned at about 126 places in 31 modules, his fix
   (`last_known`) in 18 modules, `state` in about 35, `react_t` in 25. No one place says what he believes or why.
3. **Who has him is answered six ways.** (a) dead or held, read off his fields; (b) four scripts lent in a fixed
   order at `guards/p_07.rs:100-103` (grenade, ran, shaken, startled); (c) his own `script`, set through
   `squad::order` from 17 modules and ended from 15; (d) the flags `hold` and `pinned`, written in 24 and 19
   modules, by which the room fight and `update_guard` hand him back and forth; (e) the claims word of `owner/`,
   15 rules, 73 `has_him` asks in 24 modules, each asker with its own list of whom it gives way to, and those
   lists disagree in 11 places (the notes on who has a man, `docs/agents/enemies.md`); (f) values that mean ownership without saying so
   (`rally_at == INFINITY` is "the mission's man", `lost_at` ahead of the clock is "held to his room").
4. **One function is sense, belief, decision, act and speech at once.** `update_guard` is 887 lines and calls
   30 other modules by name. Seeing a body, inside perception, also writes belief, counts the tally, files the
   chronicle, picks the words, plays the radio, raises the alarm and hands every other man a duty.
5. **A new behaviour is many edits.** `startle` is the smallest whole reaction: 267 lines of its own, a per-man
   row in its own table, six hooks placed by hand inside `update_guard`, a bit in `owner`, and asks in other
   rules. A new squad order needs an arm in up to eight enums in `orders/types.rs` and a match arm for each.

What is already right, and stays: the tick's top level is one explicit list (`game/p_06.rs`
`step_seats_on_first`, about 120 calls in order); things are named by typed ids in tables (`ids.rs`);
`GuardScript` is already "what his body should do this tick" as plain data; `owner/` already has one ordered
table of rules; `frag` already separates dealing (`frag_tick`) from answering (`frag_flee`).

## 2. The design on one page

**A man** is an id and a small fixed set of records: body, condition, kit, senses, belief, membership,
assignment, control, nav, and one row for each rule that keeps something about him. A guard, a squad mate, a
hostage and the player's own man are the same kind of thing.

**A tick runs five layers, in this order, and each layer writes only its own records.**

| Layer | Reads | Writes | Today's code that becomes it |
|---|---|---|---|
| SENSE | the world, bodies | `Senses`: this tick's percepts | `can_see`, `hearing_check`, `acquire_foe`, `body_in_view`, the light meters |
| BELIEVE | percepts, words on the net | `Belief` | the 126 writes of `awareness`, `belief/`, `attention`, `witness` |
| GROUPS | members' beliefs, the world | `Assignment`: a man's task and its limits; the group's own plan | `enemysquads`, `pressure`, `laststand`, `lanes`, `tactics`, `qrf`, `rescue*`, `powerout`, `netquiet`; `orders`, `doortools`, `pairorders`, `wallcharge`, `cover`, `rearwatch` |
| DECIDE | belief, assignment, condition; a rule's own row | `Control`: who has his legs, his aim and his hands, and the intent | the lent scripts, `gd.script`, `hold` / `pinned`, the modes of `Def` and `Op`, the cascade of `update_guard` |
| ACT | the intent, the world | body, nav, kit; events | `run_guard_script`, `follow_path`, `go_to`, aim, `burst_trigger`, the pose |

Then the world's own systems (doors, props, glass, fire, weather, fog), then REPORT, which turns the tick's
events into the log, sounds, the chronicle and the tally.

**Two questions were one, and are now two** (section 5). *Whose man is he*: one group at a time has given him
a task; a ranked list of taskers with incumbency decides who may take him from whom. *Who is moving him now*:
every rule that wants his body this tick proposes, and one arbiter grants his legs, his aim and his hands, by
one ordered list. The two lists together replace the six answers of section 1.3.

**Adding a behaviour is one module and one line** (section 7).

**Groups are the same thing on both sides** (section 6): a squad is members, a lead, a commander (a seat or an
AI) and an order book.

## 3. A man in data

`ManId` is the one identity (today's `CharId`: every body has one). Records are `ManMap<T>`, a dense
`Vec<Option<T>>` walked in id order. Three records are new ideas; the rest gather `Guard`'s 130 fields and the
269 tables by kind (body, condition, kit, senses, membership, nav: `docs/internal/ai-rows.md` places every one).

- **`Belief`**: awareness; an alert level that is LATCHED (calm, suspicious, hunting, engaged), entered by
  thresholds and left only by named events (`Lost`, `Cleared`, `Calmed`); his foe as a `ManId`; the fix with its
  source (sight, lead, sound, word) and age; what he has been told and found. Private to `belief`.
- **`Assignment`**: the task his group gave him, who gave it, since when, whether it may be interrupted, and
  its LIMITS (below). At most one. Reservations (a spot, a door, a throw) live here too, dealt in id order.
- **`Control`**: for each of legs, aim and hands: the rule that has it this tick, why, since when; the intent;
  the rules that asked and lost.

**A task limits; a proposal moves.** A task carries a fixed vocabulary of limits that every rule below reads in
one place: `post` (do not leave this spot), `bounds` (stay in these rooms), `route` (do not cross this mask),
`arc` (this is your sector), and `fire`, a policy asked as `may_fire(man, target)` (free; held until noticed;
not through glass). Today the same facts are 40 questions asked of 15 modules (`squad_stays_in`, `lane_bars`,
`silent::round_held`, `silent_holds_glass`, `room_fight_cover_passes`, `press_keeps_home`, ...).

**A per-man table is placed before it is moved**: belief; a clock (a named cooldown in a rule's row); plan
progress (a rule's row); physical state; a memo (derived, out of saves and digests); telemetry (out of the
simulation's state). 269 tables do not become 269 components.

## 4. Sense and belief

`Percept`: `Seen { who, at, rate }`, `Heard { kind, at, loud, t }`, `Told { at, by, t }`, `FoundBody`,
`LightOut`, `DoorForced`, `Hit { from }`, and the exits a behaviour reports (`Lost`, `Cleared`). SENSE makes
them and writes nothing else. A shout another man hears is a `Heard`, neither speech state nor an effect.

**Every living man senses every tick, whoever has his body.** Today a man on a lent script returns before
perception (`guards/p_07.rs:100-116`), so `morale` and `room_fight_tick_move` each call `hearing_check`
themselves to give him ears back.

BELIEVE folds a man's percepts into his `Belief` and is the only code that may. The 31 modules that write
`awareness` today mostly mean "he is told" or "he notices": each becomes a percept or one call
(`belief::raise(man, floor, why)`) with its reason kept, so "why is he alert" has an answer.

`state` is three things in one field: life (to `Condition`); the alert level (to `Belief`, latched as above,
because today Alert is entered at 0.85 and never leaves by awareness: it ends through `drop_to_search`, 14 s
at a body, or an order's cold timer, `orders/p_05.rs:95-104`); and the search WALK, which is a behaviour that
runs while he is hunting. Other code reads Search as "roused, contact lost" (`lanes::roused`): that is the level.

A squad has knowledge of its own: contacts with their time and source. Members add to it only by reporting,
and take from it as `Told`. `enemysquads`' net and radio check, `netquiet`, the fog of war's `known_t`,
`tactics`' danger records and the room fight's `reports` are this, kept five times.

## 5. Whose man he is, and who moves him

### Assignment: who may give him a task

One ranked list of taskers. A tasker may take a man only from a tasker below it, or from nobody, and only if the
task he holds says it may be interrupted (a man digging in may be taken while he thinks, not once he has begun:
`digin/p_02.rs:583-588`). The loser is told and may ask again. This is the list the notes on who has a man recommended, and the decision
held there is taken: yes, one list.

Strongest first, with why each stands above the next:

1. **the mission's script** (rescue crew, a staged lesson): authored, must happen.
2. **last stand**: the side's last decision; a squad's plan is moot by then.
3. **sent by his squad**, then **his squad's word pending**: a squad that has decided outranks one man's act.
4. **the last man at his act**: his own end; a group may not press him into company (disagreement 8).
5. **holding a lane**: a covered corridor is worth more than one more body in a push (9).
6. **digging in**: begun work is finished; the press finds another man (4).
7. **walking to a silent mate**, **walking to a lit room**, **sent to the breaker**, **a buddy on his way**:
   chores, in the order they cost the side least to delay.
8. **his squad's man at rest**: he belongs to a squad, so a fight next door may not take him (the press and
   the rally sit below), but a chore may (the four above). This is the one place taste decides; it is what most
   rules do today.
9. **the press**, then **the rally**: opportunist; they take whoever is free.
10. **a need** (coffee): only a man nobody wants.

For a team: his part in a door or wall drill, then a pair's job, then a plain order, then fall in.

In code an assignment is three calls, and a tasker is whoever makes them (not built yet; step E makes the
first, for `hold` and `pinned`):

```rust
assign(man, Tasker::Lane, Task { post: Some(mouth), arc: Some(door), interruptible: false, until: None })  // Err(who has him) if a tasker above holds him
release(man, Tasker::Lane, Why::LaneLost)       // the tasker below, if any, has him again
task_of(man) -> Option<&Task>                   // what every rule below reads: his post, bounds, route, arc, fire
```

A lane that wants a holder calls `assign`; if the press has the man it gets him (the lane is above), if his
squad has sent him it gets `Err(SquadLeg)` and picks another. The lane's own rule then proposes "aim at the
door" as a default each tick. Nothing asks the lane whether it has him: his `Task` says so.

Being his squad's man at rest is an assignment and proposes nothing: it stops the press and the rally from
taking him and leaves him free to fight. (Draft 1 made it a rule that moved him; the review showed that would
stop 55% of man-ticks from engaging.)

### Control: who moves him this tick

```rust
pub struct Proposal { pub rule: RuleId, pub legs: Want<Legs>, pub aim: Want<Aim>, pub hands: Want<Hands>, pub why: u8 }
pub enum Want<T> { No, Default(T), Demand(T) }
```

Three channels. For each, a demand beats any default; among demands, and among defaults, the higher rule wins.
An order's "aim at the door" is a default; the fight's "aim at that man" is a demand, so an ordered man turns to
a foe without the order saying so. A reload demands hands and nothing else. A rule chooses what it wants per
tick, not once: a startle demands aim for its beat, then legs too if he runs.

The list, strongest first. A rule may stand at two places with two conditions.

| Band | Rules |
|---|---|
| INCAPABLE | dead · down · held · dazzled or staggered (aim and hands only) |
| STAGED | a script that must not be interrupted |
| REFLEX | a live grenade · broke and ran · shaken by a blast · ducking under fire |
| SCRIPTED | a mission's script · startled |
| CLOSE | a foe inside arm's reach of his path (the fight, first standing): `room_fight_tick_move`'s plant, `lanes::close_fight` |
| TASK | whatever `Assignment` holds: the drill, the lane, the leg, the order |
| INITIATIVE | fight (second standing) · standoff · search walk · look into a fix · a found body · check on a man · force a door · covering fire |
| ROUTINE | a need · doze · a lockdown post or pair · his patrol route |

A demand from any band beats a default from any band: that is what lets a fight (INITIATIVE) turn an ordered
man's gun (TASK, a default aim) to a foe. Between two demands, or two defaults, the list decides.

Order between rules is the list, never a score. A rule's `propose` returns its own best choice. No planner, no
behaviour tree: multi-step drills (stack, breach, bound) are small state machines in a rule's row.

**Suspend and resume belong to the arbiter.** A rule that loses a channel it held is told so, keeps its row,
and gets the channel back when the winner stops asking. Today each rule hand-rolls this (`squad_lend` and
`squad_hand_back`, an order's `yield_t`, `Op.step`). A grant holds for a minimum time unless a higher band
asks, so men do not flicker between two proposers.

**The list above is the design. The code today asks in the old code's order**, which differs in three places,
each a step with tallies: startled is asked before his script and gives way to it (so the outcome is the
table's); ducking is asked below his script and only of an alert man, where the table has it a reflex above
scripts; and granting by channel, the minimum time and reservations are not built (a rule that takes cover
today takes all of him: his gun is lowered and he fires no round).

A rule is three functions:
- `transition(row, belief, events)`: move the rule's own state. Runs for every man it has a row for.
- `propose(view, man, row) -> Option<Proposal>`: pure. Reservations it needs were dealt in `transition`.
- `commit(row, outcome)`: what it was granted, and what the body did with it (`arrived`).
- and it hears `lost` when it had him last tick and does not this tick (built).

### One man, one tick

A defender holds lane 2 (assignment: `lanes`, limits: post P, arc on the door). He is engaged, mid-reload. A
grenade lands beside him.

- SENSE: `Seen` the foe in the door; `Heard` the grenade land.
- BELIEVE: still engaged; the fix moves to the door.
- GROUPS: `frag`'s `transition` deals him a flight spot and reserves it. Nobody outranks `lanes`: he is still
  the lane's man.
- DECIDE: `frag` demands legs (run to the spot) and aim (gun down). `reload` demands hands. The fight demands
  aim (the foe) and offers legs by default (stand, since the post limits him). The lane offers aim by default
  (the door).
- Granted: legs `frag`, aim `frag` (REFLEX above INITIATIVE), hands `reload`. The fight and the lane are told
  they lost.
- ACT: he runs, gun down, and the reload goes on in his hands.
- The grenade bursts. `frag` stops asking. Legs fall back to the task's post, aim to the fight: he walks back
  to the lane with his gun on the door, magazine in.

Today that is four modules each asking three others whether they have him.

## 6. Groups: a squad is a squad

```rust
pub struct Squad { side, commander: Commander, lead: Option<ManId>, members: Vec<ManId>, book: OrderBook, tools: DoorTools, walls: WallCharge, knows: SquadKnowledge }
pub enum Commander { Seat(SeatId), Ai(AiCommander) }
```

Today a seat's order book, door tools, wall charges and 15 more fields are swapped in and out of `Game` around
each call (`seats::with_seat`), 18 per-man tables are rekeyed by view, and the response team runs the squad's
code a second time by swapping two fields (`squadcast::with_cast`). The swap is the hack; the fact under it is
right: the squad's code already serves two commanders. So the squad becomes a value passed by id, and `Seat`
keeps only a player's hands and eyes (input, camera, aim point, hover, HUD).

Three things come first, each small: (1) `Foe` names a man (`Foe::Player` carries no id today, so an enemy
facing "the player" cannot say which); (2) a squad has a `lead` (orders measure from him, `orders/p_05.rs:52`);
(3) sense takes the man looked at as a parameter (today an enemy is stepped "under the seat he faces",
`seats::view_for`, which is a view standing in for an argument). After them the 17 functions that ask the bare
game for seat 0 (found by the co-op work) stop compiling and are each answered once.

An enemy squad, the press, the last stand and the response team are then taskers with an AI commander. What
they share with the player's team from the start is `Assignment`, the limits and the arbiter.

**Rules co-op puts on this** (from the co-op work, which found each as a fault):
- The tick is a function of the game and the seats' records, nothing else. A seat's aim point, hover and
  nearby things are derived inside the tick from his man and his record, for every seat on every machine. The
  camera and the HUD are the host's and are not in `Seat`. No field of the game means "the seat this machine shows".
- A squad's code touches only its own members. Nothing sweeps all the world's men from inside one squad's
  tick, and nothing works only because of the order the seats are stepped in. What every squad must agree on
  lives on the man or the world and is swept once. Budgets count seats, never "1 player".
- No host writes the game between ticks: what a player can do is a key in his record. An empty seat keeps its
  man and its squad; a commander does not become an AI because a player left, unless the tick decides it.
- The whole game is one value: cloned, hashed, nothing the tick reads outside it. Rule rows and `Control` are in
  the clone; they are not yet in a checkpoint written to a file, and must be before a rule's row matters to play.
- Moving ONE man's legs under a record, on a scratch copy of that man with the world read-only, stays possible
  (local prediction). The arbiter is not the only door to a body: ACT takes `&World` and one `&mut Body`.
- A rule of the world that says "the player is close, seen, down" means any player's man or each, never the first.

## 7. Adding a behaviour: one module, one line

This is built (`rust/game/src/control/`), for the two seams the code has today. A rule is one file and one line.

```rust
// control/mod.rs: THE LIST. The order of the lines is the rank.
rules! {
    frag: "a live grenade", Body;
    runner: "broke and ran", Body;
    morale: "shaken by a blast", Body;
    startle: "startled", Body;
    duck: "head down under fire", Cover;
    reload_in_cover: "changing magazine in cover", Cover;
}

// control/reload_in_cover.rs: the whole rule
pub struct Row { pub plan: Plan, pub t: f64 }                         // what it keeps of one man
pub fn transition(g: &mut Game, gd: GuardId, dt: f64) { ... }         // decide once as the reload begins; notice he has arrived
pub fn propose(g: &Game, gd: GuardId) -> Option<Intent> { ... }       // go to the spot, kneel, or None. Reads only.
pub fn commit(g: &mut Game, gd: GuardId, o: Outcome) { ... }          // arrived / going nowhere
pub fn lost(g: &mut Game, gd: GuardId) { ... }                        // something else took him
```

A line names the rule's file, the words that name it, and the stage of a man's frame at which it is asked.
`Body`: before he senses, and its intent is a script his whole body follows. `Cover`: in the fight, and its
intent is `TakeCover`: down where he is or off to a spot crouched, gun lowered toward the danger, no round
fired, hands free to change magazine. The line makes the rule's module, its id, the table of its rows
(`g.ctl.rows.reload_in_cover`) and its place in the list. Nothing else names the rule: the commit that added
the pilot is its file and two lines.

Each man's `Control` says who has him, since when, and which rules were asked and said no. `control_line`
prints it (`Alvarez: changing magazine in cover for 1.2 s · said no: a live grenade, ...`), and the native
runner ends a run with a CONTROL line: the man-ticks each rule had a man.

**What the pilot taught.** Its first version was 60 lines and wrong in ten ways a review found by reading: a
flashed man still walked to cover; "he is there" was remembered and never checked; he often could not reach
the spot before the magazine was in; two men could pick one spot; it took the player's own men out of their
file. Mended, it is 110 lines, and a third of them are one function, `may`, whose every line is a question the
game cannot yet answer in one place: is he on a post, is he kept to his rooms, is he anybody's. Those lines are
what `Assignment` and its limits are for. A rule is one file today; it is a SHORT file once step E is in.

**Not there yet.** A rule's rows are cloned with the game (a kept game keeps them) but are not in a checkpoint
written to a file. A rule cannot yet take his aim or his hands alone. A test cannot yet stand a room, a door
and two men (another part of this work is building that), so the pilot's tests cover its gates and its row,
not a whole fight.

An order, the same way: one module exposes `can`, `valid`, `issue`, `tick` and a data row, and one line
registers it. The eight enums of `orders/types.rs` stay for the orders not yet moved; the pilot (the 40 mm
shell, `squad40`, called from 3 modules) shows the other path. It waits for the types to be hand-owned.

**The bar:** someone who did not do this work adds one enemy reaction and one order by touching one module and
one line each, with a native test.

## 8. The tick as a schedule

The top level is already a list. Hidden below it: `room_clear_scenario_tick` runs the enemy squads;
`rescue_tick` runs three more rules; `room_fight_tick` runs every defender (digging in inside that), every
operative, then the press, the last stand and the buddy rule; and `update_guard` is all five layers for one man
before the next man senses anything, with decide BEFORE sense for a man on a lent script.

First cut (no change of play): `update_guard` becomes six named parts in today's order: incapable, lent,
perceive, transition, behave, pose. Decide and act have no seam yet (Search's `arrived` and pie speed feed the
same branch), so `behave` stays one part until its rules move out one by one. Later, once, with tallies: all
men sense, then believe, then groups, then decide, then act.

No scheduler framework: the schedule stays a function read top to bottom.

## 9. What stops being `&mut Game`

`Game` is split into parts that can be borrowed apart: `world`, `men`, `squads`, `seats`, `rules`, `events`,
`host`. A view is a plain struct of references to what a layer may touch (`DecideView` is read-only, so
`propose` cannot write). The 174,000 lines that take `&mut Game` are not rewritten: new code takes views, and
each rule moved is a rule whose reads and writes the compiler checks.

Dice: a rule draws from its own stream, keyed by (rule, man). That change lands alone, before any change of
play, because one shared die makes any added draw reshuffle a mission.

Events: a rule does not call the HUD, the mixer or the chronicle; it emits, and REPORT applies. What a man is
saying is a FACT others read the same tick (`radio_recent`, startle's `call`), so the bubble is state, written
at once; the sound and the log are effects.

## 10. Where the time goes

Measured on b874b654 with lap clocks in a scratch copy of the native runner, release build, one core, replaying
whole missions (so the game is the same game, tick for tick). A tick is about 1.2 ms (room-clear-four 1.19 ms
over 3,006 ticks with 9 men; compound-four 1.22 ms over 7,576 ticks with 15 men).

| Where | room-clear-four | compound-four |
|---|---|---|
| posing the men (`character_update`: animation, bones) | 27% | 28% |
| a scripted man's step (`run_guard_script`, with his pose) | 17% | 12% |
| the light meter (`light_estimate_in`) | 11% | 8% |
| lines of sight (`segment_blocked`, about 130 a tick) | 6% | 6% |
| the player (aim, update) | 12% | 13% |
| the fog of war | 6% | 6% |
| ALL of sense, belief, state and behaviour inside `update_guard` | 3% | 5% |
| every group rule together (room fight, lanes, squads, press, tactics, morale, last stand) | 6% | 9% |

**The AI's deciding is under a tenth of the tick.** The men's update is 40% of it, but that is their bodies:
animation and light. So nothing in this design is bought with speed, and the design may spend freely on
clarity: asking every rule every tick, granting by channel, writing the record. Speed is the bodies' matter
(pose only men a camera can see; a cheaper light meter) and the page's (the hand-over to the picture costs more
than the tick: `tools/blitz/PAGE.md`).

## 11. Special cases today, and the general rule that replaces each

| Today | Becomes | Does play change |
|---|---|---|
| four lent scripts tried in a fixed order, then his own | proposals through the arbiter | no (step A) |
| each rule's own give-way list (73 asks) | the tasker list with incumbency | yes, 11 places, each with tallies |
| `hold` / `pinned` written by 24 modules | a `post` limit given and released by a named tasker | no, then yes where two givers fought |
| `rally_at == INFINITY`, `lost_at` ahead of the clock | the mission's script as tasker; `bounds` with an expiry | no |
| 40 questions about what he may do | five limits on his task | no |
| a man on a lent script is deaf unless his rule remembers to listen | every man senses every tick | yes |
| each rule's own suspend and resume | the arbiter's | yes, small |
| a question that writes (`press_keeps_home` stops his rally clock) | `transition` moves clocks; questions are pure | yes, small |
| `startle::runs` tries a break-off for real and undoes it (`room_fight_bolt`) | plan, then commit | no if done with care |
| `with_seat`, `with_cast`, view-keyed tables | a `Squad` passed by id | no |
| "the player" read off the game | a man or a seat as a parameter | yes in a game of two (it is wrong today) |
| act reorders the man mid-step | a request for next tick | yes, one tick later |
| speech, tally, alarm inside perception | percepts and events | no for the effects |

## 12. Steps, each keeps the game playing

E = no change of play, proved by the same per-tick series as b874b654 on the kept runs (21 missions, default
and Hard, at the least). P = play changes on purpose, shown by the mission tallies before and after.

- A (E, BUILT) The list and the arbiter for the body scripts, asked lazily in today's order. His own script
  stays in `gd.script`: 71 files read it.
- A2 (E, BUILT) The alert part's first test (has he his head down) asked through the same arbiter: head-down
  is a rule.
- B (E, BUILT) The four body rules as transition / propose / commit, `propose` on `&Game`; a rule's transition
  still runs only where its old function ran.
- C (E, BUILT) `update_guard` cut by program into incapable, lent, perceive, alertness, patrol / suspicious /
  alert / search, pose. What passes between them is one `Frame` of 18 fields, which is honest about how much
  they share: narrowing it (what he sensed, read-only; what his behaviour decided, returned) is the next cut.
- A3 (E, BUILT) One list with one vocabulary (section 7), `lost`, the record written once a tick.
- Pilot reaction (P, BUILT, held until the keyed dice can judge it): a defender changes magazine behind cover.
- NEXT, in this order: (1) rule rows in file saves; the native room-door-two-men test, and the pilot put under
  it (flash, grenade, time to the spot, two men). (2) F (P, alone): keyed dice; then the pilot's tallies.
  (3) E below, and with it the alert part's legs cut from its fire, so that a rule may move a man who shoots.
- NOT YET: running every rule's transition every tick so that losers show. `startle_transition` ends a run
  whenever he has a script, and its stuck clock would run while a grenade has him: it needs "paused" first.
- Pilot order: `squad40` as one module and one line, once the types are hand-owned.
- E (E) `hold`, `pinned`, `script` given and released through `control::` by a named rule; the hidden
  sentinels named. Before G: ranks need named givers.
- G (P) the tasker list decides; the has_him lists go, one disagreement at a time.
- D (E) belief the sole writer, with reasons. Before H and J. Fields private once the alert level is a latch.
- H (P) body-found, contact and the other inline reports become percepts and events; every man senses.
- I (E) `Foe(ManId)`, a squad's lead, sense by parameter; then a squad as a value.
- J (P) all sense, then believe, groups, decide, act.
- K (P), the largest: the room fight's `Def` and `Op` become taskers and rules. A `Def` holding emits no intent
  today; it sets `pinned`, `react_t`, `skill` and a point that the Alert branch reads.

Each BUILT step is one commit on b874b654; what each was held to is in its commit.

## 13. The reviews, point by point

Two reviews, read-only on the code. **The first**, of the outline; its claims that
were checked on the code are marked.

| Its point | Answer |
|---|---|
| A claim is not a proposal: a squad's man at rest is claimed to refuse the press, not to move him (checked: `enemysquads/p_02.rs:153`, `pressure/p_02.rs:443`) | TAKEN. Two lists: assignment and control (section 5). |
| `over` is not a gun half; every script has a default aim; looks have a third order (checked: `guards/p_07.rs:876-886`) | TAKEN. Three channels, demand or default. |
| Hands are real (throws, reloads, a locked sidearm) | TAKEN. The third channel. |
| Three limits miss fire per target, glass, crossing a mask, an arc | TAKEN. Five limits; fire is a policy. |
| `propose` cannot be pure where a rule tries for real and undoes (checked: `room_fight_bolt`), draws a die while choosing, or deals spots one man at a time | TAKEN. Plan and commit apart; reservations dealt in `transition` in id order; keyed dice first. |
| Rank depends on the situation: dig-in, an order against a hot fight (checked: `orders/p_05.rs:95-104`), a move against contact | TAKEN. A rule may stand twice; a task says whether it may be interrupted. |
| Alert is a latch, not a threshold (checked: no awareness exit in `update_guard`) | TAKEN. Latched levels with named exits. |
| `with_cast` swaps two fields, not 19; enemies run under the seat they face; `Foe::Player` has no id; orders need a lead (checked) | TAKEN. Section 6's three first things. |
| The list missed ducking, door force, covering fire, dazzle | TAKEN into the list. `tactics_alert` is a tasker (to place with `docs/internal/ai-rows.md`). |
| Suspend and resume, reservations, minimum commitment, senses for controlled men | TAKEN (sections 4, 5). |
| C is bigger than it looks; cut six parts, not four | TAKEN. |
| Order A, B(E), F, then any P; D not before G | TAKEN. |
| Pilot reaction: change magazine behind cover | TAKEN. |
| Cut `score`; cut "enemies that stack are a row in a table"; move the data section out; add one worked tick | TAKEN. The data and speed notes move to appendix B. |
| "Off for a coffee" moved without saying so | It is said now: a need is the weakest tasker and its walk is ROUTINE. In the old table it sat above a squad's man at rest; here a squad's man does not wander off for coffee. P, with tallies. |

**The second review**, of draft 3 and of the code as built (the five no-change commits and the pilot):

| Its point | Answer |
|---|---|
| The five commits that claim no change of play: read line by line against what they replaced, none found | Recorded. The proof is the replay of 46 missions, the same after every tick. |
| The pilot, as first written: ten faults (a flashed man walks; `there` never checked; not in time; already in cover; yo-yo; two men one spot; no live threat needed; it pre-empts door forcing; it ignores his squad's rooms; it takes teammates) | ALL TAKEN: rewritten (section 7), defenders only, each fault a gate or a check with a unit test where one can be written without a level. |
| "While his fight keeps his aim" is false: the cover block lowers his gun and fires no round | TAKEN. The intent is named `TakeCover` and the record says the rule has all of him. |
| "Cloned and saved": cloned, not saved to a file | TAKEN, said in sections 6 and 7; the save is another piece of this work. |
| The list in section 5 is not the order the code asks in | TAKEN: said under the table, each difference a step. |
| Demand against default across bands was not stated | TAKEN: stated. |
| Assignment is clear but has no interface to act on | TAKEN: three calls and the lane example in section 5. |
| One rule id from one macro; one vocabulary; `lost`; the record written once; drop the second copy of `since` | TAKEN and built (A3). |
| "One small Frame" is 18 fields under blanket allows | TAKEN: said in step C; narrowing it is the next cut. |
| Put rows on the wire by the macro | NOT TONIGHT: the wire is being frozen as the old schema and new state is to get its own save; it is counted there. |
| Do not run every transition every tick yet | TAKEN (section 12). |

Not taken: nothing outright. One caution kept from draft 1 against the review's "use latched levels ... as
percepts": the exits are percepts from a man's own behaviour into his own belief, which is a loop through one
tick. It is honest (he notices he has lost the man) but it must not become a back door for behaviours to write belief.

## 14. Decided here, for the maintainer to overrule

- One ranked list decides who may give a man a task (the held rank decision): yes. A squad's man at rest above
  the press and the rally, below the chores; a need below everything.
- Changes of play land with tallies, not behind switches.
- No planner, no behaviour tree, no archetype ECS.
- The squad's code is one code for both sides, reached in steps; tonight only the shared records and arbiter.
