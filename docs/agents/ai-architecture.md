# The AI: a man as data

What a man of the game is made of, the order he is thought in, and how to add to him. Read it before adding a behavior. What defenders do is
`enemies.md`; what teammates do is `squad.md`, `team-feel.md` and `door-tools.md`; attention and the fog are `stealth.md`.

## What the player gets

- Men who only know what they could know, by the light, smoke and sound the player is shown.
- Men who drop everything for what matters: a live grenade beats a blast, a blast beats a startle, and each beats what they were doing.
- Teammates who need no hand-holding: they cover, turn and fight without being told.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`. A man's records:

| record | where | what it holds |
|---|---|---|
| his row | `Guard`, `g.gd[gd]` | every man of any side but Sam (`g.pl`): `state`, `awareness`, `last_known`, `foe`, `script`, `hold`, `pinned`, `task`, his clocks |
| his body | `Character`, `g.ch[c]` | where he is, his bones, his animation |
| his mind, written | `mind.rs` | the one door to awareness, state, fix and foe, and who last moved each (`Note`) |
| his duty's aim | `belief.rs` | `covers`, said anew each tick; `gun_point`; a report that places him (`word`) |
| his senses | `guards/senses/`, `sense.rs`, `guards/feet.rs` | ears and eyes of one man; the game's own smoke and light; `can_see`, `look_of` |
| his eye | `attention.rs` | what small thing holds a calm man's eye (`stealth.md`) |
| how he is | `Cond`, `g.cond[c]`, `condition.rs` | down, hurt, blind, folded, the fire on him: a table by body, for the player, guards and civilians |
| whose man he is | `owner.rs`, `taskers!` in its types | one word on his row of the block (`man_owner`), a bit a tasker |
| who has his body | `control/` | `Control`, the list of rules and the arbiter (`g.ctl`) |
| what he belongs to | `team`, `unit`; `Squad` (`enemysquads/`); `Def`, `Op` (`roomfight/`); `Assignment` (`orders/`) | his side; his squad; his place in a room fight; his post under an order |
| a rule's own row | `g.m.morale.state_of`, `g.m.startle.state_of`; `PerMan` in `Rows` of `control/` | what one module keeps of him |

And `guards/frame.rs` (one man's frame), `guards/behave/` (what he does by his state), `guards/looks.rs` (`LOOKS`, where some module has him look),
`control/man.rs` (what a rule may ask of a man), `control/frag.rs`, `runner.rs`, `morale.rs`, `startle.rs`, `duck.rs` (the rules; their code is in
`frag/`, `runner.rs`, `morale.rs`, `startle.rs`, `suppression.rs`), `squad/` (`order`, `end_guard_script`, `run_guard_script`: a man's own script) and
`dice.rs`.

## How it works

**A man's frame** is `update_guard`, one function a part, in this order:

| part | what it does |
|---|---|
| `incapable` | dead or held: his body settles and nothing below runs |
| `sense` | his ears (`hearing_check`) and his eyes (`foe.rs`), whoever has his body |
| `hands` | a change of magazine goes on, or begins by itself |
| `lent` | a rule of the body stage has him, or his own script: he follows it and nothing below runs |
| `notice` | where his man went, a colleague on the floor, the man he trails, a kicked lock |
| `alertness` | his state follows his awareness; it settles `Sensed` |
| `behave::act` | his state's file, from `Sensed` (which it does not write) to `Acts` (which it returns) |
| `pose` | `looks::lay`, the muzzle and the wall, the animator, his light, his footfall |

A teammate on a script, or a man of the response team on his, has his senses kept by that system (`kept_elsewhere`).

**The layers, as far as they exist.** Sense and belief are one step: a sense moves his mind through `mind.rs` as it runs. Groups think in their own
ticks in `step_seats_on_first` (`game/tick.rs`), before any man's frame: `squads_tick`, `squad_orders_tick`, `room_fight_tick`, `lanes_tick`,
`frag_tick` and others. They answer by setting his script, his flags and their own rows. Then each man's whole frame runs before the next man's.
Decide and act are together in `lent` and `behave::act`.

**His mind.** `mind::raise` (never lowers him), `cap` (never raises him), `set`, `set_state`, `fix` (news: `fix_stamp` counts it), `fix_keep` (not
news), `foe`. Each takes the writer's name. Anybody can read the fields, and no rule writes them any other way.

**Who moves him now: the list.** `rules!` in `control/mod.rs` is one line a rule, and the line's place is its rank: `frag`, `runner`, `morale`,
`startle`, `duck`, `lost`, `rally`. What a rule asks for decides where it is asked, and the compiler holds it to that:

- A rule whose `Ask` is `GuardScript` is `Stage::Body`, asked in `lent` by `grant_body`, above his own script. His whole body follows that script for
  the tick (`run_lent_script`), and his own waits.
- A rule whose `Ask` is `TakeCover` is `Stage::Cover`, asked at the top of `alert` by `grant_cover`, so only of a man who is alert with no script and
  no body rule. He gets down (`TakeCover::down`) or goes crouched to a spot (`TakeCover::to`), gun lowered, firing nothing, hands free to reload. A
  blind or staggered man gets down wherever it sends him.
- A rule whose `Ask` is `LeavePost` is `Stage::Post`, asked by `grant_post` of a defender at his post who can see, once he has looked and reacted. He
  goes on a leg or loose, once: on the leg, his script has him.

`grant_from` is the arbiter: from the top, it runs a rule's `transition`, then asks its `propose`, and the first answer has him. A rule below the
winner is neither moved nor asked. `record` writes the `Holder` once a tick (`Dead`, `Held`, a rule, `Script`, `Own`) and tells a rule that had him
and no longer does (`lost`). The record keeps legs, aim and hands apart, but every grant takes all three.

**A rule** is `impl Reaction for Rule`. It must write `Ask` and `propose` (which takes `&Game`, so asking changes nothing). It may write `Row`,
`transition`, `commit` (what his body made of it: `Outcome`) and `lost`.

**Whose man he is: the taskers.** A second question and a second list: `taskers!`, strongest first, from `FRAG` down to `SQUAD`, `PRESS` and `NEEDS`.
A squad's man at rest is his squad's, and no rule moves him for it. `owner::claim` sets a bit and `release` clears it. `who` is the strongest, and
`has` asks of named taskers. `kept` asks "may this tasker take him?": his body first (dead, held, blind, reeling, a hostage in his arms), then anyone
above it in the list. `claim` itself refuses nobody, so the list decides only where a tasker asks `kept` first. His squad, the press, the lights and
the last man ask it; other rules ask `has` of those they give way to.

**Posts.** `hold` (he stays where he stands) and `pinned` (the room fight has him this tick) go through `set_hold` and `set_pinned`, which note the
writer. A defender leaves his post and comes back by `roomfight/post.rs` alone: `send` (on a `Leg`), `loose` (on the fight's own hunt), `back_on`. No
rule writes his `mode`. A leg keeps who sent him (`on_leg_of`): sent elsewhere, the first errand is over.

**Limits have no one home.** What a man may do is asked of the module that knows: `lane_bars`, `round_held`, `silent_holds_glass`,
`room_fight_cover_passes`, `press_keeps_home`, `stays_in_rooms`. A new rule that moves a man asks them for itself.

**Dice.** `draw_of(g, "module::function#1", gd)` is a fixed function of the seed, the site, the man, the tick and how many times this site has drawn
for him this tick. So a new draw about one man moves no other man's draw, whatever order the men are walked in. The block's die still steps once a
draw, as a witness, so `hold` sees a new draw in its `dice` part.

**Debugging.** `control_line` says it in one line: Alvarez: shaken by a blast for 0.4 s · said no: a live grenade, broke and ran · a covered lane has
him for 3.1 s. A test prints it with `Play::why` and asks the rule's name with `Play::rule`. `hold` and `missions` report how long each rule had a
man. No panel of the page shows any of this: ask from a native test.

## Rules that hold

- **What it is for.** "having a nicer architecture for the AI feels important too, more for understandability and having good emergent behavior, not
  just LOC".
- **One way of sensing.** "we should make one way for sensing". The rules ask the simulation how lit a point is and how much smoke is on a line
  (`sense.rs`, `light_estimate_at`). Every eye looks along `look_of`, which is the fog's `gaze_of`, so what the picture shows of where a man looks is
  what he sees.
- **No micromanaging.** "we expect squad mates to be very smart on their own and not need micromanaging like this". A behavior a player would have to
  order by hand belongs in the man.
- **They must not read as stupid.** "we don't want them to feel dumb". Weigh any new behavior by it.
- **A man's ears and eyes run whoever has his body.** Only `kept_elsewhere` is excused, and a man with headphones on is deaf (`doze_deaf`).
- **The list decides, never a score.** There is no planner and no behavior tree: a drill is a small state machine in a rule's row.
- **Rank a rule as low as it can be and still be right.** The rule that has a man is the only one thinking about him that tick, and those below find
  their rows stale when they get him back.
- **Alert is not "has a man to fight".** A bang alone makes a man alert. A rule that means a fight asks `saw_his_man_within`.
- **A question lives in `control/man.rs`:** one line, takes `&Game`, and its comment says what it does not tell you. Add the missing one there instead
  of reading fields in a rule.
- **A new tasker asks `kept`** and keeps no list of its own. Add no assignment to `awareness`, `state`, `last_known` or `foe`: they go through
  `mind.rs`.

## To change it

**A new reaction** (something happens and it takes him for a while):

1. `control/<name>.rs` with `pub struct Rule` and its `impl Reaction`. Copy `control/duck.rs`, the fullest: `transition` asks `head_down` once,
   because asking moves clocks and may draw, and keeps the answer in its `Row`; `propose` reads the row; `commit` ends the shift on `Outcome::Arrived`
   or `Outcome::GoingNowhere`. `control/startle.rs` shows the other form: an empty `Row`, the state in `startle.rs`, and `commit` handing back the
   script as the walk left it (`startle_script_ran`).
2. A line in `rules!` at its rank: its file's name and the words that name it. Nothing else names it. 
3. If it also tasks him, a line of `taskers!`, with `claim`, `release` and `kept` where it takes and lets go.
4. A test that is a fight (below). Break the rule once, with `propose` answering `None`, and see the test go red.

First count how often the thing happens: a rule almost nobody meets is not worth its row.

**A new order:** `orders/rows/<name>.rs` with `pub const ORDER` a `Row` made by `Row::new`, and a line of `rows!`. Its `issue` hands each man an
`Assignment` (`rows::assign`). `orders/rows/cover.rs` has it all in one file; `orders/rows/shell40.rs` hands its functions on to `squad40.rs`. Read `Is`
once and decide what the rest of the game may ask of it. The test gives it by `Play::order` and must include contact: posture alone proves nothing.

**How he behaves when nothing has taken him:** his state's file in `guards/behave/`. A new sense is a file of `guards/senses/` and a line in `notice`.
A look is a line of `LOOKS`. A new state of a man is the costly one: an arm of `GuardState`, which many modules read, and of `behave::act`.

**A number:** `knobs/`, turned by a panel or `?knob.startle.beat=1`. **A draw:** `draw_of` with a site name no other line uses.

**After the change,** in `rust/game_start`:

- `cargo test --release --test owner`, by your file's name, holds your rule. `gate tests` holds them all, and `gate` the whole game.
- `cargo run --release --bin hold -- keep /tmp/held` before, and `cargo run --release --bin hold -- check /tmp/held` after: every run in which the
  rule never acted must stay the same.
- `cargo run --release --bin missions`, before and after, then `cargo run --release --bin missions -- diff before.tsv after.tsv`, says what a change
  of play came to.
- `bun run tools/blitz/face-build.ts` for the page. A new rule lands on, with a native test that fails without it.

**Writing the test** (`rust/game_start/README.md`): a `Scene` of a few rooms and men, `Play::scene`, three seeds with `nudge`, and a control where the
thing does not happen. Ask through `play.rs`, do through the game's own calls, hold what a man did, and put `p.why(gd)` in the failure's message.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `owner.rs` | the list and nothing else keeps a man from his squad, the lights and the last man's act |
| `ears.rs` | a man running from a grenade still hears and sees |
| `blind.rs` | a man who cannot see goes nowhere, whatever spot a rule names |
| `men_under_fire.rs` | the duck, the startled beat, the blast, the man who drops it and runs |
| `hands.rs` | a reload goes on and begins by itself |
| `attention.rs`, `witness.rs`, `senses.rs` | a calm man's eye, who knows what, smoke and light |
| `post.rs` | a defender sent from his post, and what takes him off it |
| `orders.rs` | the 40 mm order in a fight; `every_row_holds_its_own_order` |
| `squads.rs`, `squads_radio.rs`, `net_quiet.rs`, `pursuit.rs` | groups: the net, the radio check, two men on one fix |

Inside the crate: `control/mod.rs` (the arbiter, the record, the ranks), `owner.rs`, `mind.rs`, `dice.rs`.  `gate`'s pins move if a man does
anything else.

## Where it is going

Agreed and not built. Assume none of it.

- **Belief as one record:** a latched alert level left only by named events, his foe as a man, a fix with its source and age, private to one module.
- **Percepts:** senses hand over what was seen, heard or told, and that module alone decides what it does to him. `state` splits into his life, his
  alert level and the search walk.
- **Five layers for all men:** all sense, then believe, then groups, then decide, then act, with nothing `kept_elsewhere`.
- **A task with limits:** one task a man, given and released by name, carrying his post, bounds, route, arc and fire policy, read in one place. `hold`
  and `pinned` become a post from a named tasker.
- **The tasker list deciding:** a tasker takes a man only from one below it, the loser is told, and each rule's own give-way list goes.
- **Grants by channel:** a rule proposes legs, aim and hands, each as a demand or a default, so a man goes on shooting while a rule moves him.
- **The whole list in bands:** incapable, staged, reflex, scripted, close, task, initiative, routine, with his script, fight, search and patrol as
  rules in it, and the room fight's `Def` and `Op` as taskers and rules.
- **The arbiter's care:** suspend and resume, a minimum grant, reservations of a spot or a door, every `transition` run every tick.
- **A squad as a value:** members, a lead and a commander that is a seat or an AI, one code for both sides, so `with_seat` and `with_cast` go. `Foe`
  names a man (`Foe::Player` does not). A squad's knowledge is one record of contacts.
- **A report step:** a rule emits events, and the sounds, the chronicle and the tally apply them after.

## Not seen

No native test sees how a man reads. Known faults: a rule that has a man has all of him, so a man taking cover cannot go on shooting; and each man's
frame runs before the next man's, so what one man does this tick is seen by those walked later and not by those walked earlier.
