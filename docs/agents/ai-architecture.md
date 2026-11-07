# The AI and the data it runs on: one design

Written 2026-11-07 on the tree where the Rust game first stood alone, after two reviews (section 13). Built on it so far: the list and its
arbiter, the four old body rules in the list's vocabulary, `update_guard` cut into parts, the fight cut into aim, legs and hands, one door each
for a man's post flags, his mind and his dice, one worked example of a new rule and one of an order, each with a whole-fight test (sections 7 and 12).
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

**Adding a reaction or an order is one file and one line** (section 7); a behaviour of a man's own is not yet.

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

## 7. Add a reaction, an order, a behaviour

**A reaction**: something happens to an enemy and it takes him for a while (a grenade lands by him, a blast shakes
him, rounds pass close). One file and one line.

1. `rust/game/src/control/<name>.rs`, five things and nothing else (`control/duck.rs`, 41 lines, is one to copy):
   `pub struct Row` what it keeps of one man (`pub type Row = ();` if nothing) · `transition(g, man, dt)` move
   its own state · `propose(&Game, man) -> Option<Intent>` what it asks of him this tick, reading only ·
   `commit(g, man, Outcome)` what his body made of it · `lost(g, man)` a higher rule took him.
2. One line in `rules!` of `control/mod.rs`: `<name>: "the words that name it", Body;` or `Cover;`. Its place
   in the list is its rank. `Body` rules are asked before he senses and hand him a script. `Cover` rules are
   asked of a man whose state is Alert and who has no script of his own, and hand him `TakeCover` (down where
   he is, or off to a spot). ALERT IS NOT "HAS A MAN TO FIGHT": a bang alone makes a man Alert, so a rule
   that means "in a fight" asks for itself (`g.time - g.gd[man].last_seen_t`). RANK IT AS LOW AS IT CAN BE AND
   STILL BE RIGHT: the rule that has a man is the only one that thinks about him that tick, so every rule
   below it finds its own row stale when it gets him back.
3. A test that is a fight: a file in `rust/game_start/tests/` that stands a few rooms and men, makes the thing
   happen, and asserts the rule had him (`game::control::control_line`). `cargo test --release` there; no page.
   Write a control beside it (the same scene where the thing does not happen, or happens to a calm man), and
   break the rule once (`propose` returns `None`) to see the test go red.

Worked example, held back until the dice are keyed: `control/reload_in_cover.rs` (a defender who runs dry kneels or
goes to cover to change magazine), the line `reload_in_cover: "changing magazine in cover", Cover;`, and the test
`a_guard_who_runs_dry_in_the_open_kneels_to_change_magazine` in `tests/control.rs`.

**An order**: a row of the squad's menu. One file and one line.

1. `rust/game/src/orders/rows/<name>.rs` with `pub const ORDER: Row = Row { id, label, can, only, valid, issue,
   tick, frame }`: its id on the menu and the functions the menu and the tick call (`orders/rows/mod.rs` says
   what each is for). Two of them are the order. `issue` gives each man his task with
   `squad_orders_assign(g, man, &SquadOrdersAssignA { order, post, face, stance, upper, crouch, going, there, .. })`:
   where he stands, which way he faces, how he carries his gun, and the two lines he says. `frame` runs once a
   frame and is where the order puts its row on the menu once (`g.tab.orders_order_def.push(rows::def(&ORDER,
   data))` then `squad_orders_register`), or takes it off.
2. One line in `rows!` of `orders/rows/mod.rs`: `<name>;`.
3. A test that is a fight, the same way, giving the order by the menu's own call (`Play::order`). IT MUST
   INCLUDE CONTACT: the men under the order meet an enemy and the test says who fell and when. A test of
   posture alone was green on an order that got two of four men killed (the pose trap, below).

Worked examples: a whole small order is printed below ("take a knee", 42 lines, in this page only). In the
build: `orders/rows/shell40.rs` ("put a 40 mm shell there") with the two tests of `tests/orders.rs`; it shows
the `Row` and the list but hands its three functions on to an older module, so it is not the one to copy.

**Before either lands** (none of this is in the file or the line):
- `bun run tools/sim/build.ts` after any change under `rust/`.
- An order's id also goes into `dealtRows` of `tools/blitz/schema.json`, or the page's export of a game that
  holds the row stops with "no closures by id for the row" (`tools/blitz/export.ts`). Reading a game back is
  already by the list (`rows::by_id`).
- A dozen modules know orders BY ID and a new order is in none of their lists: `silent` (loud rows),
  `holdfire` (lift and blow orders), `sneak` and `unseen` (posted), `orders::lane_orders`, `squadthrow`
  (resumed), `rails` (a lesson's list of rows hides a row it does not name), `cmdring` (the ring's seats),
  `cover` and `roomfight` (`order == "hold"`, `"cover"`, `"fallIn"`). Read each name once and decide; say in
  the commit which the order joined.
- The menu's digit: `squad_orders_register` deals a row with no key the first free digit (from 5, or from 1
  for a row on a page), and none when all five are taken: the wheel still reaches it.
- A switch or not: section 14 lands changes of play with tallies and no switch, the repo's standing rule says
  a new rule is a switch. Until that is ruled, say in the commit which was done.
- The build: a change to a rule or an order is a build of the game crate, about ten minutes on a small
  machine; a change to a test is fifteen seconds. Ask your questions of the game from the test first.
- Searching for a field of a man: leave out `io.rs` and `puppet.rs`, whose lines are several thousand
  characters each.

**That nothing else moved**: before the change, `cd rust/game_start && cargo run --release --bin hold -- keep
/tmp/held`; after it, `… -- check /tmp/held`. 72 native runs, every part of the game hashed every frame; it names
the first frame and the parts that differ. A new reaction or order is meant to change play where it acts, so read
it this way: every run in which it never acted must still be the same (each run's line says how long each rule
had a man), and what it does where it acts is for the mission tallies to judge.

**What is NOT one file and one line today**, plainly. *A behaviour of a man's own* (how he patrols, searches,
fights when nothing has taken him) is still a function of the guard's update in `guards/p_07.rs`, and the room
fight's defenders and operatives are still their own module: neither is in a list. A reaction can take a man
whole, not his legs alone while he goes on shooting. A reaction's file is longer than its idea, because it must
ask for itself whether a man is on a post, kept to his rooms or somebody's (a third of the worked example).
A reaction that draws from the die re-deals the rest of the mission until the keyed draws land. An order that
shows a line in the squad's HUD block still needs an arm of `OrderNoteFn`, which the page reads over the wire,
and an order that keeps state of its own still needs a field on the game. About thirty older orders are still
arms of six enums. The rest of this section is the detail of each of these.

### The pieces in detail

The list of reactions as it stands (`rust/game/src/control/`), with the held worked example's line last:

```rust
// control/mod.rs: THE LIST. The order of the lines is the rank.
rules! {
    frag: "a live grenade", Body;
    runner: "broke and ran", Body;
    morale: "shaken by a blast", Body;
    startle: "startled", Body;
    duck: "head down under fire", Cover;
    reload_in_cover: "changing magazine in cover", Cover;   // held: lands after the keyed draws
}

// control/reload_in_cover.rs: the whole rule
pub struct Row { pub plan: Plan, pub t: f64 }                         // what it keeps of one man
pub fn transition(g: &mut Game, gd: GuardId, dt: f64) { ... }         // decide once as the reload begins; notice he has arrived
pub fn propose(g: &Game, gd: GuardId) -> Option<Intent> { ... }       // go to the spot, kneel, or None. Reads only.
pub fn commit(g: &mut Game, gd: GuardId, o: Outcome) { ... }          // arrived / going nowhere
pub fn lost(g: &mut Game, gd: GuardId) { ... }                        // something else took him
```

A line names the rule's file, the words that name it, and the stage of a man's frame at which it is asked.
`Body`: before he senses, and its intent is a script his whole body follows. `Cover`: asked at the top of
`alert` (`guards/p_07.rs`), that is of a man whose state is Alert and whom no script has, before his reaction
time and before his fight; its intent is `TakeCover`: down where he is or off to a spot crouched, gun lowered
toward the danger, no round fired, hands free to change magazine. The line makes the rule's module, its id,
the table of its rows (`g.ctl.rows.reload_in_cover`) and its place in the list. Nothing else names the rule:
the commit that added the pilot is its file and one line.

Four things the list does not yet do for a rule, each found by a reader who added one from this page alone:
- **An intent of the wrong kind is dropped without a word.** A `Cover` rule that returns `Intent::Script`
  (or the other way) is ignored: no panic, no record, not even "said no".
- **`TakeCover` cannot say "hands idle".** Any `Cover` rule lets the reload run on, so a blinded man held
  down by a rule finishes changing magazine.
- **One unit test pins the first `Cover` rule by name** (`control/mod.rs`,
  `the_games_list_is_the_order_update_guard_asked_them_in`): a rule ranked above `duck` must change it.
- **The list holds 32 rules** (the "said no" record is 32 bits).

What a rule reads of a man today, with no door: `dazzled_until` (is he blind: written by a flash, and also
by a wall charge, a thrown door and the door tools, so it does not say WHAT blinded him), `last_seen_t` (when
he last had his man in his eyes), `team` and `civ` (is he an enemy), `standoff` (has he a hostage in his arms),
`pinned` and `hold_room` (through `control::set_pinned` / `set_hold` only), `reload_t`, `sup.cover`.

Each man's `Control` says who has him, since when, and which rules were asked and said no. `control_line`
prints it (`Alvarez: changing magazine in cover for 1.2 s · said no: a live grenade, ...`), and the native
runner ends a run with a CONTROL line: the man-ticks each rule had a man.

**What the pilot taught.** Its first version was 60 lines and wrong in ten ways a review found by reading: a
flashed man still walked to cover; "he is there" was remembered and never checked; he often could not reach
the spot before the magazine was in; two men could pick one spot; it took the player's own men out of their
file. Mended, it is 110 lines, and a third of them are one function, `may`, whose every line is a question the
game cannot yet answer in one place: is he on a post, is he kept to his rooms, is he anybody's. Those lines are
what `Assignment` and its limits are for. A rule is one file today; it is a SHORT file once step E is in.

**What a rule calls, today.** Four doors are built, each a hand-written file a
person can read in a few minutes, and a rule uses them instead of writing a man's fields. (The fight's three
parts, `fight_sight`, `fight_legs` and `fight_trigger` in `guards/p_07.rs`, are NOT a door: they are private
to the guard's update and take `&mut Game`, so a rule cannot call them.)

| to do this | call | file |
|---|---|---|
| make him more or less aware, or change his state | `mind::raise(g, man, floor, "my_rule")`, `cap`, `set`, `set_state` | `rust/game/src/mind.rs` |
| tell him where his man is | `mind::fix(g, man, Some(at), "my_rule")` (a new fix), `fix_keep` (put back, not news) | `rust/game/src/mind.rs` |
| hold him to his post or pin him | `control::set_hold(g, man, on, "my_rule")`, `set_pinned` | `control/mod.rs` |
| draw from the die for him | `dice::draw_for(g, "my_rule::what", Who::Man(man))` | `rust/game/src/dice.rs` |

Every old write of awareness and state (189), of his fix (54 calls now, and eight private one-line helpers
gone) and of the two post flags (81) was moved to these by a program, so there is no second way in the body
files (the programs' `--check` is red on one), and each door keeps who last moved him and when: `mind::why` and
`control_line` print it (`Alvarez: alert at 0.92 · alert since 3.0 s ago by roomfight (was patrol) · raised to
0.92 by pressure 3.0 s ago · his fix by guards 0.4 s ago`).

**An order is the same shape** (`rust/game/src/orders/rows/`). An order of the squad's menu is a `Row`: its id
and its `label`, `can`, `only`, `valid`, `issue`, `tick` and `frame` functions, in one file, and one line:

```rust
// orders/rows/mod.rs: THE LIST
rows! {
    shell40;
}
// orders/rows/shell40.rs
pub const ORDER: Row = Row { id: "shellThere", label, can: None, only: Some(only), valid: Some(valid), issue,
                             tick: Some(squad40::turn_tick), frame: Some(squad40::squad40_tick) };
```

A whole order, as a reader who had only this page wrote it (it is printed here and is not in the game):

```rust
// orders/rows/knee.rs  TAKE A KNEE: the men it is given to stop where they are and crouch, until their next order
const ID: &str = "takeKnee";
pub const ORDER: Row = Row { id: ID, label, can: None, only: None, valid: None, issue, tick: None, frame: Some(frame) };

fn label(_: &mut Game, _: &OrderTarget) -> String { String::from("take a knee") }

fn issue(g: &mut Game, men: &[GuardId], _: &OrderTarget, _: Option<&GiveOpts>) -> Option<String> {
    for &gd in men {
        let c = g.gd[gd].ch;
        let (p, face) = (g.ch[c].pos, g.ch[c].body_yaw);
        crate::orders::squad_orders_assign(g, gd, &SquadOrdersAssignA {
            order: String::from(ID), post: Some([p[0], 0.0, p[2]]), face: Some(face),
            stance: Some(Stance::None), upper: Some(UpperMode::Aim),   // gun up: see the pose trap
            crouch: Some(true),
            going: String::from("taking a knee"), there: String::from("on a knee"),
            ..Default::default()
        });
    }
    None                                                               // or Some(why it was refused)
}

/// its row goes on the menu once there is a squad to give it to
fn frame(g: &mut Game) {
    let listed = g.orders.defs.iter().any(|&d| g.tab.orders_order_def[d].id == ID);
    if listed || !crate::orders::squad_orders_active(g) { return; }
    let data = OrderDef { targets: vec![TargetKind::Floor, TargetKind::Door, TargetKind::None],
                          ack: Some(Vec::from([String::from("taking a knee.")])), ..Default::default() };
    let id = g.tab.orders_order_def.push(def(&ORDER, data));
    crate::orders::squad_orders_register(g, id, None);
}
```

Three things about an order that its file does not show:
- **The pose trap.** An order that crouches its men with the gun carried low (`Stance::LowReady`,
  `UpperMode::None`, the pattern the older orders show) gave men who knelt and did not fire for about three
  seconds at a defender they could see: on three seeds two of four fell, where the same men with no order, on
  `hold`, standing in the same pose, or crouched with `UpperMode::Aim` lost nobody. The order's id made no
  difference; it is the crouch with the lowered gun. Found on a trial, cause not traced. Until it is, a crouched
  order carries the gun up.
- **`tick` does not run in a hot fight.** `squad_orders_tick` (`orders/p_05.rs`) skips a man who is Alert with
  no script until his fight has gone cold, so "once a frame for each man under the order" is true only out of
  contact; and the fight drops the order's crouch while it has him.
- **The id is typed twice.** `Row.id` and the `order` of `SquadOrdersAssignA` are the same word written by
  hand, and nothing ties them.

`frame` runs once a frame after the menu's input and is where an order puts its row on the menu or takes it off
(`rows::def(&ORDER, data)` makes the row from what only it knows: its page, its targets, its key). The older
orders are still arms of six enums in `orders/types.rs` with bodies in the matches of `orders/p_03.rs`; each
enum has ONE arm, `Row`, through which every order written this way is called, so moving an older order
deletes its arms and adds none. The 40 mm shell is moved (its five arms, six bodies, its line of the frame and
its row of the read-a-game table are gone); its fight test is `rust/game_start/tests/orders.rs`.

**The test of a rule is a fight.** `rust/game_start/tests/control.rs` stands a room, a guard and the player's
men, runs the fight, and asserts the rule had the man (`cargo test` in `rust/game_start`, no page, no
TypeScript). The pilot's test: a guard who runs dry in the open kneels to change magazine, on three seeds.

**What is still long-hand, plainly.**
- A rule is one file, not yet a short one. A third of the pilot is `may`: is he on a post, kept to his rooms,
  anybody's. Those lines go when `Assignment` holds them (step G).
- There are two seams, `Body` and `Cover`. The fight is now three functions (aim, legs, hands), but a rule
  cannot yet be granted one of them alone: the list still grants a man whole.
- A rule's `transition` runs only where its old function ran, so a rule that lost this tick did not think.
- The doors record; they do not yet decide. `mind::raise` writes what its caller asked, and `mind::fix` whatever
  point it is handed: belief does not yet weigh a word against his own eyes in one place.
- The keyed draw is built and every draw is moved to it by a program, but its output is NOT in the build: it
  re-deals every mission once, so it lands alone with the mission tallies before and after. Until it lands, a
  new rule that draws moves every later draw of the mission, and a tally cannot judge it. The pilot waits on it.
- A rule's rows are cloned with the game (a kept game keeps them) but are not in a checkpoint written to a file.
- Orders: one is moved, about thirty are not. The one long-hand left for an order that has a line in the
  squad's HUD block: that line is still an arm of `OrderNoteFn`, an enum the page reads by its tag over the
  wire. It ends when a `Row` has a `notes` function and the page is handed the lines as text, which is a change
  of the wire's table and of the HUD's reader, not of the game. An order's own state is still a field of the
  game's `Mods` (`g.m.squad40`).
- No-change proofs are native now (`rust/game_start`, `hold keep` / `hold check`: 24 levels by three scripted
  hands, every part of the game hashed every frame), but the hands reach three things: a pushed fight, cans
  among the enemy, a floor at rest. No hand gives a door order or walks Sam.

**The bar:** someone who did not do this work adds one enemy reaction and one order by touching one module and
one line each, with a native test.

**The bar, tried.** A reader with this page and no other help added a reaction (an enemy blinded in a fight
gets down where he stands until he can see) and an order (take a knee). Both compiled the first time and both
tests passed on three seeds, with a control each and a broken build that went red: 36 and 42 lines of game
code, one line each in the two lists, 71 and 88 lines of test. The claim held for what compiles and runs
natively and failed at its edges, and each edge is now said above: one pinned unit test, the wire's list of
order ids, the pose trap, what Alert means, what a rank costs the rules below. The code was about a tenth of
the work; learning those was the rest. What would make each a quarter of an hour is a list for the code, not
for this page: `Intent` typed by stage, `TakeCover::down()` with a word for the hands, defaults for the empty
functions, a small file of questions a rule may ask of a man (enemy, in contact, blind, stopped), a `Row` that
lists itself on the menu, an assign that takes its id from the `Row`, and readers in the test's `Play`
(`rule(man)`, `flash_lands`, `on_menu`, `red()`, `blue()`).

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

E = no change of play. The steps up to A4 were proved by the same per-tick series as the last tree with a
TypeScript tick, on 46 recorded missions (23 by default and Hard). When the wire changed under those
recordings the proof became `hold` (section 7): 72 native runs kept on the tree before a step and checked on
the tree after it, every part of the game hashed every frame. P = play changes on purpose, shown by the mission
tallies before and after.

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
- A4 (E, BUILT) The fight's shooting branch is three functions in the order it ran: `fight_sight` (aim),
  `fight_legs`, `fight_trigger` (hands). The seam a rule needs to move a man who goes on shooting.
- E, first half (E, BUILT) `hold` and `pinned` written through `control::set_hold` / `set_pinned` with the
  writer's name: 81 writes, by `tools/blitz/sweeps/post-flags.py`. Not yet: `script` given and released by a
  named rule; the hidden sentinels named. Before G: ranks need named givers.
- D, first half (E, BUILT) awareness, state and the fix written through `mind::` with the writer's name: 189
  writes of the first two, 54 calls for the fix, 39 second names for a man and eight one-line helpers gone, by
  `tools/blitz/sweeps/mind-door.py`. The stand-ins for "his fix is his watched point" that the settled belief
  row left dead are out by hand. Not yet: belief deciding what a raise or a word does; the fields private.
- Pilot reaction (P, BUILT, held until the keyed dice can judge it): a defender changes magazine behind cover,
  with a whole-fight test (`rust/game_start/tests/control.rs`).
- F (P, BUILT AS TWO PROGRAMS, not in the build): keyed dice. `dice-who.py` hands each helper that draws the
  man it draws for (9 helpers, 44 calls: no change of play by itself). `keyed-dice.py` then moves the 88 draws
  of the one die to keyed draws: 64 about a man or a squad, 24 the world's own (a prop, a thrown thing's
  tumble, a sound's pitch, a deal of the whole floor), and takes out 4 draws made only to keep a place in the
  old sequence. After it, the order men are walked in is not play for any draw about a man. It re-deals every
  mission once, so it lands alone, after the no-change steps, with the 84 tallies before and after.
- NEXT, in this order: (1) F lands; the pilot's tallies. (2) rule rows in file saves. (3) the list grants a
  channel (legs alone), so head-down and the pilot stop silencing the gun by a floor on its clock. (4) the
  other orders moved to `orders/rows/`, door tools first (twelve of the arms are theirs).
- NOT YET: running every rule's transition every tick so that losers show. `startle_transition` ends a run
  whenever he has a script, and its stuck clock would run while a grenade has him: it needs "paused" first.
- Pilot order (E, BUILT): the 40 mm shell as `orders/rows/shell40.rs` and one line of `rows!`, with a fight
  test. One `Row` arm in each of six enums is the seam for the rest.
- G (P) the tasker list decides; the has_him lists go, one disagreement at a time.
- H (P) body-found, contact and the other inline reports become percepts and events; every man senses.
- I (E) `Foe(ManId)`, a squad's lead, sense by parameter; then a squad as a value.
- J (P) all sense, then believe, groups, decide, act.
- K (P), the largest: the room fight's `Def` and `Op` become taskers and rules. A `Def` holding emits no intent
  today; it sets `pinned`, `react_t`, `skill` and a point that the Alert branch reads.

Each BUILT step is one commit; what each was held to is in its commit.

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

## 15. Plan: the fog of war keeps two records (not built)

The fog's record (`fogofwar::FogOfWar`, one part of the game's state) holds two different things. WHAT THE
TEAM KNOWS is the game's: who has been seen and where, which doors, lamps and loose things are remembered and
how, who is left out. Rules read it, so every machine of one game must hold it the same. THE PICTURE'S BLOCK
is one machine's: the renderer's rows, the lamps picked for the lit fog, the switches of a viewer's lifted
draw. Two machines showing different seats need not agree on it, and a host that draws nothing need not fill it.

Today they are one record and one word of the state, and two places cross the line:

- `fog_of_war_lamp_take` measures each lamp from `seats::host_man`, the seat this machine shows; when a level
  has more lamps than `FOG_LIGHTS` the list is sorted by that distance and cut, so `lamp_pick`, `frame.lights`,
  `frame.lights_u`, `frame.light_count` and `lit_stats.lights` differ by machine, inside the state.
- `fog_of_war_hidden` and `fog_of_war_air_hides` ask `!bared` first, the switch of a viewer's lifted draw:
  a rule's answer goes through the picture. (`bared` is only ever true between two steps, so no tick has
  read it true; the mend changes nothing played.)

### Which field goes where

`g.fog_of_war` keeps the name and becomes the team's record. The picture's block is a new part,
`g.fog_pic: fogofwar::FogPicture`, named on the scratch list (`tools/blitz/schema.json`, `game::Game.fog_pic`)
so it has no word of the state.

| stays in `FogOfWar` (the team's; a word of the state) | why |
|---|---|
| `on`, `eyes`, `mates` | the switch a scenario sets; this tick's eyes, which `spot_seen`, the doors and the sticky camera read |
| `recs`, `shown`, `hidden_chars` | who is seen, framed, scripted in; who is left out: read by some forty rules through `hidden`, `seen`, `sighting`, `drawn` |
| `fired_n`, `fired_at`, `fire_clock` | the squad's fire shows the man it is at |
| `door_recs` (`shown` and `ghost`), `doors_held`, `door_from`, `door_n` | a door as last seen, and its copies on the frame's list |
| `lamp_recs` | a lamp as last seen |
| `thing_recs`, `thing_out`, `thing_since`, `thing_stats`, `cell_seen`, `span_x`, `span_z`, `rig_from`, `rig_to`, `our_chunks`, `our_landed` | loose things as last seen, and the step's own working values |
| `frame_no`, `turn`, `asked`, `pending` | the pass's count and its smoke asks |
| `call_clock`, `last_call`, `called_t`, `known_t`, `told_step`, `call_n`, `call_log`, `call_stats` | the calls a teammate makes of what he sees |
| `stats` | the pass's counts |
| `full_on` | read by rules (`ours_on`, `ours_id`, `air_hides`); renamed `live`, see below |
| `loaded`, `props_loaded` | nothing sets either true since saves went; left for whoever brings a saved fog back (`FogSnap`) |

| moves to `FogPicture` (the host's; no word) | why |
|---|---|
| `frame` (all of `render_fogofwar::FogFrame`) | the renderer's block |
| `lamp_pick`, `lamp_n`, `lit_stats` | the lit fog's list, cut by distance from the shown seat |
| `face_recs`, `full_stats` | a lamp's glow as last seen, which only `lamp_face` reads to paint `fog_emit`; the count of lights by class |
| `viewer`, `bared`, `raw_list`, `raw_at` | a viewer's lifted draw |
| `look_frames` | counted down and read by nothing; goes with the picture until it is deleted |
| from `Game`: `fog_held`, `men_bare`, `men_all`, `men_statured` | what `game_fog_lift` keeps to hand back; become `fog_pic.held`, `.men_bare`, `.men_all`, `.men_statured` |

### Which function goes where

`fogofwar/picture.rs` (new, the host's side; may read all of the game, writes only `g.fog_pic` and the
renderer's own fields named below): `fog_of_war_lit_block`, `fog_of_war_lamp_take`, `fog_of_war_thin_block`,
`fog_of_war_lamp_face`, `face_of`, `fog_of_war_face_seen`, `fog_of_war_faces_lift`, `fog_of_war_faces_back`,
`fog_of_war_raw`, `fog_of_war_filter`, `fog_of_war_everybody`, `fog_of_war_page_look`, the tail of
`fog_of_war_pass` from "the renderer's block" down (as `picture::fill`), and the half of `fog_of_war_full_block`
that paints (`tab.light[..].fog`, `.fog_ghost`, a box's `fog_emit` and `fog_mark`: by their own notes the
upload alone reads these). `game_fog_lift` and `game_fog_back` already are the host's (they are called between
two steps, never in one) and move beside them.

Everything else stays the team's. Four cuts make the line clean:

1. **A lamp's knowledge is written by the pass, not by the picture.** `fog_of_war_lamp_known` is the only
   writer of `lamp_recs`, and today it is called from inside `full_block` and again from `lit_block`. The pass
   gets its own step before the block, one call a fixture that has a light, in the order of `g.targets`; the
   picture then reads `lamp_recs` and asks nothing. (The second call a frame goes. It could only count a test
   twice: `thing_stats.tests` is zeroed by `fog_of_war_things` later in the same frame, so no word kept at a
   frame's end moves. By reading; `hold` says.)
2. **Who the team is belongs to the team.** `full_block` fills `engine.lights.fog_team` (its `owners` are read
   by `things_ours` and `ours_id`) and sets `full_on`. That loop moves into the pass with the lamp step.
   `full_on` is true exactly while the fog is active and the pass has run, so it becomes the team's `live`,
   and the pass's own test "was I on last frame" reads `live`, not `frame.on` as it does now.
3. **`hidden` and `air_hides` drop `!bared`.** Every caller found is a rule or a step of the tick (the AI's
   picture, aim, orders, markers, cues, props on the frame's list). A host reader that wants "as drawn now"
   gets `picture::hidden_as_drawn` (`!fog_pic.bared && hidden`); none is known, so it is written only if the
   grep at the time finds one.
4. **The list before the fog cut it is kept by the picture.** The first lines of `fog_of_war_things`, under
   `viewer`, copy the frame's list into `raw_list`. They become `picture::keep_raw(g, &list)`, called by
   `game_draw_characters` just before `things`; `things` then never reads `viewer`.

`picture::fill` stays where the tail of the pass is today, called once a tick on every machine. It is the
picture's by what it may write, not by where it runs; a host that draws nothing may later leave it out, and
the check below is what makes that safe.

### The ghosts: does a machine with no viewer make the same rows?

Yes for `viewer`, by reading. `viewer` gates only `raw_list` and `raw_at`. The copies the fog keeps of a door
moved unseen (`fog_of_war_door_boxes`, `copy_box`) and of a thing moved unseen (`thing_take`, `ghost_box`,
`box_let_go`) are made by every machine alike, in the order of the frame's list, whenever the fog is active and
there is an eye. And they are the game's, not the picture's: they go on the frame's list, which becomes
`world.all`, and `box_world_raycast` over `world.all` is what a seat's cursor, an order's click, a weapon
laser's reach and a device's beam meet. Two seats aim at the same remembered chair. So the ghosts stay rows
of the game's table, and `thing_recs` and `door_recs.ghost` stay the team's.

Not the same for a host with dressing rows down, also by reading. The list handed to `things` is gathered by
`game_step_fx_and_dyn_boxes`, where three dressing steps add boxes: moths (their key is one the rule passes
over), one-frame effects (no key was found on them: they only move `cell_seen` and `thing_stats.out`), and a broken fixture's
pieces (`fixture_break`, keyed in the fixture range, so each gets a `thing_recs` row and, out of sight, ghost
rows). With `fixtures` down those rows are never made, the box table hands out other ids from there on, and
the state parts from a host with the row up. The dressing check's five missions are the same with the rows
down, so either none of them breaks a fixture out of the team's sight or this reading is wrong: the new test
below plays it both ways and says which. If it parts, the fog's rule passes over the fixture range as it does
the moths' (a broken lamp's pieces are then drawn live or not at all, never remembered).

### Order of work, and what proves each

1. **The cuts, with every field where it is.** The four cuts above and the move of functions into
   `picture.rs`; the record and the wire are untouched. Proof: `hold check`, the same after every frame on all
   72 runs, every word of the state compared.
2. **The fields move**, by a program (`g.fog_of_war.X` to `g.fog_pic.X` for the second table, the four
   fields of `Game`, the scratch row, the types made again). The fog's word and the part count change, so
   `hold` cannot compare the trees whole: it gains `--but PART,…` (parts left out by name on both sides), and
   the check is every other word the same on the 72 runs, plus the two-seat kept games.
3. **`lamp_take` measures from the picture's own seat** (it already does; it is now out of state), and one
   native test stands two games of one seed on a level with more lamps than `FOG_LIGHTS`, one with a viewer
   set (and, where the start seats two, another seat shown), one without, and compares every state word
   after every frame. The same test once more with the dressing rows down in one game, for the fixtures
   question.

Not part of this: what the smoke along a sight line reads. `fog_of_war_pass` asks the sensors (`sensors_read`),
and a machine whose senses are the host's probe may answer differently from one whose senses are the game's
own. That is a question about which senses a networked game stands on, not about this record.
