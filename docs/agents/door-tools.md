# Door tools and entries

What the squad does at a door on order: where each man stands, how the door gets open, what goes in first, who crosses and where each ends. The leaf itself is
`doors.md`, the orders ring `squad.md`, the gunfight during an entry `team-feel.md`, wall charges `destruction.md`, the wand's look `stealth.md`.

## What the player gets

- On a door the ring offers stack up, the wand and a page of door tools: open and clear, bang and clear, a charge, frag and clear on G, clear from outside, and
  on H a hammer or breaching shotgun if somebody carries one.
- A **stack** is the men on their marks at a **jamb**, the wall beside the frame. F, **the word**, sends a stacked team through by plain open and clear.
- A door found locked stops the order at the handle. The man says what he found, and the same row then says kick or pick.
- A bang goes off in the air inside, and nobody crosses until the blind burst has run dry. A charge waits for the player to blow it.
- Inside, each man goes to his **point of domination** (the place he holds in the room) and checks his corners, and they look over the whole room before they call it.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `doortools/` | the orders: `DoorOp` and its phases, the counts, the word, Sam's own charge. Most numbers: `DoortoolsK` |
| `orders/rows/` | one file a row (`openclear.rs`, `bangclear.rs`, `charge.rs`, `fragclear.rs`, `bigclear.rs`, `threshold.rs`, `go.rs`, `stack.rs`): label, `valid`, `issue` |
| `orders/` | `squad_orders_stack_posts` and the marks behind it, `squad_orders_pair` (who takes which mark), `squad_orders_assign` |
| `stackroom.rs`, `stackline.rs` | marks kept on the door's own face, in no other door's way; `stack_line_pace`, a man slowing in view of an open door |
| `handleside.rs` | `tool_spot`: where a lead opens a door by hand |
| `shotgun.rs`, `bigkit.rs` | the lock shell and the hammer: `breach_spot`, `big_spots`, `gun_up`, `hammer_blow` |
| `doorpath.rs` | who may cross when, and each man's walk round the leaves |
| `entrypoints.rs` | which openings the squad has been beyond; `entry_points_clear_of` moves a point off an uncleared one |
| `roomread.rs`, `roomfit.rs` | the space round a spot; marks for the move-there order and what a rear man can watch. A door order does not use them |
| `rearwatch.rs` | who keeps the way behind a stack (`rear_watch_tick`); a taken room's other door as the squad leaves (`room_keep_tick`) |
| `wandorder.rs` | the wand on order: its row is `wand.rs`, on V (`stealth.md`) |

Other systems use the same machine through `door_tools_order`: `rescue/` starts its doors on a `DoorGate`, `qrf/` runs a response team with its own counts. The page has no
rule here: the ring and its marks come through `view/`, and `door_tools_draw_charge` shows a set charge.

## How it works

**An order** is a `DoorOp` row in `g.tab.doortools_door_op`, listed in `g.door_tools.ops`; each seat has its own `door_tools`. It holds the door, the face, its men, a
lead, each man's mark (`post`) and his current assignment (`mine`). `tick_op` drops any man whose assignment is no longer the one the order gave him, so any
other order takes a man off by being given. It moves its men with `door_tools_assign`. Each step logs a beat (`door_tools_beat`), which tests read.

**The phases** (`Phase`; `door_tools_step` has a case for each):

- open, bang or frag: `Stack`, then `Ready` if it waits on the word, `Kick` if the door needs a tool, `Toss`, `Fuse`, `Wait`, `Enter`. Open and clear goes from the door to `Enter`.
- charge: `Place`, `Standoff`, `Armed`, then `Wait` and `Enter` behind the blast.
- from outside: `Stack`, `Thr` on the jambs, `Pocket` while cans go in, then `Enter`.

**Marks.** `door_tools_deal_stack` takes `squad_orders_stack_posts` and tucks each mark `tuck` further along the wall. `door_tools_start` shapes them by the tool: open and
threshold keep both jambs; behind a bang or a charge the men file up `bang_file` apart behind the opener. The lead is the man on the lead mark furthest toward the latch.
Teammates already stacked there step back (`door_tools_step_backs`, `room_back`). `door_tools_lane_cover` sends one man to cover a doorway whose
lane the stack stands in. Rear men are dealt by `squad_orders_spare_rear` and `rear_watch_tick`.

**Getting it open** is `door_tools_open`, asked again after each tool fails:

1. Open already: a shove if it stands short, and on.
2. Not locked against the team: a big-slot row's tool, else `door_tools_to_handle`, then `door_tools_shove`.
3. Locked: `door_tools_halt_at` (below). Steel ends the order. A silent team picks. On the face the door opens toward, a boot only drives it onto its frame, so a tool that
   takes it, else `door_tools_frame_answer` (pick, or refuse). Else `door_tools_to_gun` (shotgun first, hammer next; a shell cannot open a wedge), then the boot from
   `door_tools_boot_spot`: beside the latch jamb, or square on where glass or no floor rules that out.

**What the team knows.** `door_tools_test` is a hand on the handle: it writes that face into `tried` and rattles a locked leaf. `door_tools_test_stacks` makes a plain
stack try it and `door_tools_walk_tried` a move through the door. A door the team did not know is shut halts
the order once with "holding on it — your call." and leaves the men stacked. After that `door_tools_known_shut` holds, `door_tools_entry_verb` makes the row say pick or
kick, and the same row goes on through.

**The can.** A bang order takes its can as it is given (`door_tools_take`), and `door_tools_end` refunds one never thrown. `step_toss` asks again as it leaves the hand: ours
beyond the door or a leaf in the way (`frag_way_open`, `frag_way_ajar`) keep it in the kit. The thrower is the man nearest the frame who is not first in, within
`throw_reach`. `door_tools_bang_spot` picks the point inside that lights the most floor with none of ours in its flash; `step_fuse` fires it in flight at that point, or
`fuse_after_land` after it lands. `door_tools_bang` spares the entry's men. `door_tools_burst_dry` says when they may cross (`after_bang`, `quiet`, `entry_cap`). A can
from an ended order goes on a short fuse (`door_tools_defuse`). A frag has its own fuse and depth (`k.frag`) and comes back if hostages or ours are in its reach.

**The entry** is `door_tools_enter`. Points come from `door_tools_pods`: the level's `door_plan` from that face, else the first floor to each hand, then
`entry_points_clear_of`. `door_tools_first_in` picks among the men at the front of each file, leaving out the thrower and the rear watcher, by `gun_first` and
`door_tools_seen_most`. Three or more go in turn off alternate jambs (`gun_turn`, `rear_last`); a man with no point stays out and leaves the order. The queue lets each go
by `door_path_may_go`, and of two men of one entry the man named first has the frame (`named_before`).
On his point a man works through a known enemy's place (`door_tools_threat_step`), his corners (`door_tools_corner_step`), his sweep or
`door_tools_scan` of unseen floor, then his sector. `door_tools_move_on` and `door_tools_flow_in` send men on to floor nobody has seen. The order ends "in" when they have
swept it or at `search_cap`, and the lead says the room's call (`squadnet.rs`).

**After the entry.** `door_tools_keep_sectors` swings a man's sector off a teammate in it. `door_tools_other_doors` turns one man's gun onto each other door
of the room nobody has been beyond (`other_door_m`). With three or more in, one man looks back at the door they came by (`last_out_m`). `room_keep_tick` keeps a man on
such a door as the squad falls in.

**The word.** `door_tools_input` reads the key (`blow_key`) before the night breach does: a row hot on the menu, a held threshold, stacks and orders waiting, then a set charge.
`door_tools_go_men` makes a `Word`; a plain stack becomes an order by `door_tools_from_stack`. `door_tools_word_tick` holds two moments, before anybody makes a sound
(up to `word_stack`) and before anybody crosses (up to `word_in`), then goes without whoever is late. Orders from code wait on a `DoorGate` instead.

**From outside** (`door_tools_to_threshold`; row `threshold.rs`). The lead and the far man take the jambs and the rest are rear men. Only the lead cuts the opening, and he
leans (`door_tools_pie_set`, `thr_pie_arc`). When it has gone quiet (`thr_min`, `thr_quiet`, `thr_cap`) he says "holding on the door." The rows then read go in, or bang then
go in (`door_tools_threshold_then`). The word steps in with no can unless `door_tools_then_on_word` asked for one, and waits for the cut (`door_tools_cuts_owe`). Stepping in
uses `door_tools_shallow_pods`, and the room's other doors are held after (`door_tools_hold_doors`). An order with hold off goes on by itself and may throw first
(`door_tools_can_first`); the ring always gives hold, so that path is the response team's.

**A charge** (`door_tools_start_charge`). The nearest man sets it for `set_secs` and it leaves the count only then. The rest stand on `door_tools_standoff_posts`. The key calls
`door_tools_blow_raw`: the leaf goes, and within `stun_m` with a line through the opening men on the far side go down inside `knock_m` and are dazzled and staggered beyond it;
anyone on the near side inside `backblast_m` is dazzled. A teammate on the far side is only dazzled in a squad of four, and Sam only ever is. A charge on a door that comes
open is refunded (`door_tools_tick_set`) and its men hold (`door_tools_short`). Sam's own is `door_tools_sam_tick`, and the `charge` row then reads "stand off" for the team.

**Big tools.** `door_tools_gun_step` puts up to `shells` into the lock, then gives up to the boot. `door_tools_hammer_step` takes one blow, or `k.bigslot.swings` where the
frame would hold a boot.

**A silent team** (`silent.rs`, `squad_silent`) picks (`door_tools_pick_step`), eases the leaf (`silent_ease`) and throws no can unless told (`silent_no_cans`). It does not halt at
a shut door. A door no pick opens ends the order until the squad is caught (`silent_boot`). A rescue's last warning (`urgent`) lifts the pick and the ban
on the boot. Giving a row marked `Is::Loud` ends silence (`silent_tool`).

**A grenade on the stack.** `door_tools_frag_step` holds the order until the men it scattered are back, up to `frag_hold_cap`. A second word overrides it (`door_tools_frag_held`).

## Rules that hold

- **The word is a plain entry.** "let's also remove the Q vs. F for deciding how to go through a room. F always just does a basic clear".
- **A door is opened from its handle.** "open has to be on the handle side for AI".
- **Doors are realistic.** "for doors just make it realistic": nobody boots a lock on the face it opens toward.
- **A tool works on any shut door.** "these things should work for unlocked doors too".
- **A can never goes where ours are.** "the second team threw a flashbang when our guys were already on the inside".
- **No can at a leaf in the way.** "the squad does waste a lot of bangs". It comes back to the count and they go in without it.
- **From outside, no can unless asked.** "the men shouldn't throw a bang unless i tell them to".
- **One man cuts, from outside.** "men should try to pie from the outside as much as they can before considering their job done ... really only one
  guy should be doing it ... just do the lean, not the crouch".
- **The man who has seen the room goes first.** "i'm going to lead with this man because he's already seen most of the room".
- **Corners.** "teammates clearing rooms seem to miss enemies. they need to check their corners".
- **The whole room, if it is a room.** "in corner fed rooms like this, the squad should really clear the whole thing without being told", and "i like that they try to fully
  clear rooms now, but it doesn't make sense for outdoor spaces or very large rooms": `open_floor` and `flow_max` are the limits.
- **Known floor first.** "they'll go through uncleared rooms to get there!": a stack is dealt the face whose floor the team knows.
- **Nobody holds the back of a leaf.** "guy working the door from the other side is blocked by the door."
- **A frag on an open door walks nobody across.** "after being ordered to frag after externally clearing, blue one crosses across a dangerous threshold".
- **Men stay out of a neighbor's charge.** `door_tools_out_of_blast` re-deals men standing off another door, and `in_anothers_blast` rules out such marks.
- **One man's job starts over.** If the man setting a charge, picking or breaching is lost or re-ordered, `door_tools_new_lead` gives it to the next man.

**What is refused**, in these words. A row's `valid` shows them on the ring.

- Any entry (`door_tools_stackable`): "no room on that door.", "steel, and locked.", and where a frame holds a boot (`door_tools_no_boot`): "opens toward us. no boot, no charge."
- Bang: "you're behind that door." Frag: "not with hostages on the floor."
- Charge: "steel. a charge won't cut it.", "it's open.", "no charges left.", "already on it.", "no room to stand off."
- Big tool: "nobody here carries one." Held door: "already holding it." Word: "nobody waiting."
- Spoken at the door: "it's steel. no way through.", "can't get on that door.", "no way in."

## To change it

- **A number:** `rust/game_types/src/knobs/doortools.rs`, also `doorpath.rs`, `stackroom.rs`, `shotgun.rs`. A panel or `?knob.doortools.after_bang=1` turns one.
- **A row:** a file in `orders/rows/` with its `Row`, registered in `door_tools_new` or, if it hangs on the cast, `door_tools_sync`. `Is` says what it lifts or breaks.
  No key is a digit above 5, so a late row gets a letter of its own.
- **Where a man stands:** the door tools' shaping is in `door_tools_start` and `door_tools_standoff_posts`; the chain every stack shares is `squad_orders_stack_posts`.
- **A new tool on a lock:** a branch in `door_tools_open`, a state under `DoorOp.kick`, a step called from `step_kick`, and on through `door_tools_opened`. Move the leaf through
  the calls `doors.md` names.
- **A new kind of order:** a `DoorToolKind`, its case in `door_tools_opened`, a `Phase` if it needs one. New state in `DoorOp` is also written in `door_tools_new_op`.
- Run `gate tests squad_doors` and the files below, then `gate`: its pins move if a door order changes what the men do.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `door_tools_work.rs` | a charge waits and takes the door; the counts; a can before the first man; a can at a pulled door |
| `squad_doors.rs` | who goes first, files, marks |
| `held_doors.rs`, `refused.rs`, `stack_doors.rs`, `offers.rs` | found shut and the same row then boots; refusals; a lock on the face it opens toward; a keypad |
| `threshold_held.rs` | one man cuts, a can first on a loud floor, men scattered by a grenade |
| `door_hands.rs`, `two_doors.rs`, `partner.rs` | nobody behind a leaf, no boot beside glass; neighboring doors; one word for two doors |
| `room_marks.rs`, `from_his_recordings.rs` | stand-off marks; no entry point in another door's frame |
| `held_bigslot.rs`, `held_charges.rs`, `silent_squad.rs`, `order_keys.rs`, `taken.rs` | the tools; Sam's charge; silence; the word; a stack's rear |

`gate` plays door runs (`night-breach`, `compound-four`) against its pins. `tools/qa/checks.ts` and `tools/qa/pages.ts` have no row for this.

## Not seen

All of this is held by numbers. How a stack or a thrown can looks has to be judged on a GPU. The `Outside` kind with `Phase::Slice` and `Phase::Held`, and the
`held` row's page with `thrWord`, cannot be reached from the ring.
