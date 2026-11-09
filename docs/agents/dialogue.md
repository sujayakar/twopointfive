# Dialogue

Who says what, where and to whom, how a line is picked, and how it reaches the screen. How a line should read is `writing.md`. What makes a defender's net go
quiet is `enemies.md`, Sam's arm on a man is `stealth.md`, and an order's row and refusal are `squad.md`.

## What the player gets

- Everybody talks in words on the screen. No line has a voice.
- A line stands over the head of whoever says it, if the player can see him, in a tone that shows how he is: calm, wary or in a fight.
- A teammate off the screen or hidden by the fog is still read: his line sits at the edge of the view on his side, under his name, and the net clicks.
- A teammate who goes down is called from where he fell, with a louder, lower click.
- The defenders' net is on the radio strip, three rows at most, ranked so that a fall-back beats a man repeating that he is holding.
- A shout from a defender the fog hides is a quiet row with no name, and only if it really reached Sam's ears or a living teammate's.
- Sam can question a man in his arm. The man answers from what he saw and how scared he is, and may give up a key, a code, a route or how many are left.
- Two bored guards stop and chat. Sam can overhear a keypad's code that way.
- Hostages plead, civilians gasp and answer, and a handler talks Sam through the office night.

## Where the code is

In `rust/game_rules/src/`, with types and tables under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `guards/p_09.rs` | `say`: the one way a man speaks; `guards/p_08.rs` has `bubble_secs` |
| `dialogue/` | the held man and Sam: `gather`, `pick_line`, `script_for`, `pick_sam_line`; places (`place_name`, `is_office`); `tick_radio`; `update_ambient` |
| `cues/` | the strip's rows (`cues_file`, `radio_rows`), each line as a HUD is told it (`bubbles_view`), the defenders' cues (`cues_tick`, `cues_note`) |
| `squadnet/` | the click, "… is down.", a lead's call at the end of an entry (`room_call`, `plain_call`), a quick pick's reason (`why_words`) |
| `doornames/` | the name a door is said by (`name_doors`, `door_said_name`, `shorthand_id`) |
| `netquiet/` | its speech is `speak`: a man calling a name that does not answer |
| `civilians/` | `hostage_say`, `civ_say`, `civilian_answer`; `rescue/p_03.rs` times the hostages |
| `words/` | `w` and `wg`: each hands back the present line and nothing else |
| `view/` | `tick_view` hands `radio` and `bubbles` to the page |

The page's half: `src/ui/bubbles.ts` puts each line on the screen and `src/ui/netplace.ts` finds its place at the edge. The strip is drawn by `src/ui/overlay.ts`.
`src/view/pagesaid.ts` (`pageSay`) is what the page says for itself, a save's word or a panel's answer, in rows under the strip. No game hears it.

Who says what, and where the words are:

| who | lines | when |
|---|---|---|
| a man in Sam's arm | `lines`, `scripts`, `intel_text_table` in `game_types/src/dialogue/types.rs` | each press of the key |
| Sam | `sam_lines` on the game's table, a script's own beats | before each answer, `sam_key_line_for` as he takes a key, `sam_remark_for` as the pistol comes up |
| two calm guards | `ambient_pairs` | `update_ambient`, on a floor with no alarm |
| staff about their rounds | `needs_lines_table`, and lines in `guards/` | a coffee, a smoke, a call for a missing man |
| the handler and Sam | `handler_lines` | `handler_beat`, the office night only |
| civilians, hostages | `civ_lines`, `hostage_lines_table` | `civ_say`, `hostage_say` by `HostageMoment` |
| a man who finds a hold | `lines_of` in `guards/p_03.rs` | by moment and temperament |
| a defender | `LINES` in `morale/`, `laststand/`, `darkdoor/`; `lines` in `runner/`, `gunhold/`; `SUP_LINES` in `suppression/`; `wary` and its sisters in `attention/` | each rule's own moment |
| a defender at a door, or going around | `gate_call`, `flank_call` in `tactics/` | a pursuit plan |
| a defender's squad | `LINES` in `cues/`, more in `enemysquads/`, `rescueflush/` | `cues_cue`, or the rule itself |
| a teammate | an order's `ack` in `orders/p_02.rs`, `doortools/`, `sneak/`, `showwhy/`, a contact called by `fog_of_war_call_words` | `say`, or `cues_call` for a call through the fog |

## How it works

**A line is said once.** `say` puts it in the man's bubble (`GuardBubble`), for `bubble_secs` by its length. Two lines in one tick queue, and `bubble_tick` raises
the next. A held man says nothing unless forced. A man under the squad's silence (`silent_hush`) speaks on the net, and his free hand goes to his radio.
`game_interrogate` forces the answer up and gives it longer.

**What a HUD is told** is asked of the game with a shared borrow, so reading it changes nothing. `bubbles_view` goes over everybody with a line up:

- A man who is not the seat's own has no line drawn while the fog hides him.
- The seat's own man has `BubblePlace::Edge` while the fog hides him. Otherwise `BubblePlace::OverOrEdge` lets the page choose by its camera.
- The tone (`bubble_tone`) comes from his state. A teammate is never wary.

**The strip.** `radio_rows` gives what `cues_file` kept. A row has a class (`cue_class` by its words, or the kind's own in `cues/`), and a lower class is better. Class 0
is a line that came in `radio: `: control, the staff's net and the handler. A class waits for its gap, the same words are not filed twice, and a full strip gives up
its worst row. A squad's word ("Bravo: …", `squad_word`) outranks a man's in its class. Each part of that is a knob in `CuesK`.

**A defender's line reaches the strip three ways:** `say` on the radio calls `cues_heard`, `cues_net` is a line that is telling only, and `cues_cue` is one of the kinds in
`cues/`. All of them need a room fight to be on (`fight_live`, and `cues_tick` returns without one). `cues_tick` watches each defender and `cues_note` reads the fight's log, in the man's words.

**What the player hears and what is only shown.** Lines are text. The sounds around them are these: `radio` as the defenders' squelch, as the team's higher click and as a teammate's lower "down"
click; `murmur` when Sam remarks; `heldProtest` from a hostage or a man in Sam's arm. A shout (`is_shout`: the writer put a bang in it) that `cues_overheard` carries by `hear_path` to an ear
within `k.digin.shout_m` is the `RadioKind::Heard` row. It takes only a free place and gives it up to the next real row.

**The log is not the strip.** `game_msg` writes `g.messages`, the last five lines. No view and no host reads that list. A line reaches the player from it only if it starts `radio: `,
which `cues_raw` files. The rest are for rules that look back (`guards/frame.rs`, `combat/p_02.rs`, `wallcharge/p_03.rs`) and for tests (`Play::log`). To tell the player
something use a bubble, a strip row, or a row the view already carries (`mission::objective`, `interact_view`).

**A held man.** `gather` builds `DialogueCtx`: what he knows (`knowledge_of`), how rattled (`agitation_of`, `band_of`), his temperament (`temperament_of`), the facts
and the tokens a line can use. `pick_line` then asks, in order:

1. Is this a press where he gives (`gives_at`)? `intel_for` picks the next true thing he has not given. A door's name hands over its key, and a code teaches the keypad's.
2. Does his script (`script_for`, dealt once per man, off the scripts and shapes least played) have a beat for this press?
3. Otherwise the pool for his cell: exact, then any temperament, then any agitation. Another man's knowledge is never used.

A row of a table has its words in layers, and `said_of` takes the first that applies: `bare` for a nameless place, `away` off the office, then `guide`, `now`, `text`. A
token (`expand`) fills a name, and a line whose token has no value cannot be said, so it drops out of the pool.

**A pair's talk can teach.** A pair's line that names the code calls `devices_overhear_said`, and Sam learns it if the sound's path reaches him (`k.overhear`).

**How a line is not repeated:**

| who | what stops it |
|---|---|
| a held man | `said` on his row; a line said in the last `RECENT_SECS` by anybody weighs `RECENT_W`; a repeat only when no fresh line fits, and never the same one twice running |
| Sam | `SAM_RECENT_SECS` on `SamMemory`, then the whole list but the last |
| a script | the fewest played first, of the script and of its shape, on the game's `recent` table |
| a pair | `played`, the fewest first, a pair's talk once only off the office |
| a cue | `man_secs` for one man, `kind_gap` for the floor, `again_secs` for a "still here" |
| a hostage | never the line over her head; `murmur_at` spaces them |
| attention, a net call | a pool and a die, never the one he used last (`line`); a net call by kind, on a die (`speak`) |
| the handler | each beat once a run (`done`), and five seconds between beats; a beat with no quiet moment in twenty is dropped |

**A door's name** (`door_said_name`): what the level wrote (`doorNames` in its `scenarios`), else a name made from the rooms in `names_of`, for a shorthand id (`shorthand_id`) that the
room clear structure lists. Failing both, `door_words` says the id's words through `door_id_words`. The office and a lesson's level make no names.

**A place's name** is `place_name`, from the level's own rooms (`level_rects`). `own_place` is a nearby room, `watched_place` what a man looks at.

## Rules that hold

- **A defender is not staff.** `defender_words` is a hostile unit. He says "them" of the team (`flank_call`), has no name for a mate in a line (`spoken_name`), though his squad's net
  counts them off (`mate`), is `Temperament::Hard` when held, and runs none of the night watchman's lines (`LineOnly::Shift`, `Need::Office`).
- **Office things are said only in the office.** `is_office` asks for the office's routes. Elsewhere `away` and `own` rows name the level's own places.
- **The handler is the office's.** "the 'subtle was twenty years ago' text with the handler keeps popping up -- should be confined to just the office level and honestly a bit
  stale anyways". `radio_live` is false anywhere else. A beat also waits for `radio_clear`: nobody alert and Sam's hands free. His answer is dropped if he cannot give it.
- **No compass word in a door's or a route's name,** and no two doors of a level said the same, the office and the lessons aside. A route is said by the level's place it keeps to
  (`route_name`), else by its own words. The office's response team keeps "west stairwell" (`qrf/p_02.rs`).
- **On a defender's net "they" is the team.** So the hostages are named: "the hostages are gone — they're moving them! cut them off!" (`rescueflush/`).
- **One can, a word for each place.** A squad's rows and mouths say bang ("bang out!", "no bangs left."). Where kit is picked or held it is "stun canister", and a lesson says
  "stun grenade". A mission's blurb and `combat/` say flashbang. A new line takes the word of the place it stands in.
- **The verdict is a word or two in capitals, never a sentence** (`verdict/`): CLEAR, RESCUED, OUT; DOWN, WIPED, LOST, FAILED. The why under it is the mission's own line.
- **The same words on the same floor are one row.** A second man's copy of a line is news to nobody (`dup_secs`).
- **How many are inside is a mystery.** "we should also remove all defender counts from the menus, etc., it should be a mystery!" A man in Sam's arm may say how many, because he gave it up.

## To change it

- **Reword a line:** find its words (`rust/game_types/src/dialogue/types.rs` holds most tables, the rest are in the module that says them). Read `writing.md` first. A reword fails the pin of
  its table at the foot of the module (`lines_match_the_typescript`, `script_lines_are_the_typescripts`, `beats_rows_are_the_typescripts`, `handler_pools_are_the_typescripts`,
  `pairs_are_the_typescripts`): change the pin with it. The words are in the game's state and `bubble_secs` times them, so `gate`'s pins can move too.
- **Words only for one kind of place:** the `away`, `bare`, `guide` and `now` fields of a row; `own` on a pair's line; `h_now` on the handler's. Add no new branch where a line is read.
- **A new line for a held man:** a `Line` in `lines` with `k`, `t`, `s` and `needs` (a `Need`, then its fact in `gather` and `hold_fact`). A new token: `Token`, `DialogueCtxTokens`, and the
  table in `ctx_tokens_map`.
- **A new script:** a `Script` in `scripts`, with all four `ScriptBeats`. `script_fits` drops it for any man it cannot fill.
- **A new defender's line on the strip:** a `CueKind` with its row in `LINES` in `cues/` and a class, or `say` with radio on and words that `CLASSES` already ranks. A line in no row of `CLASSES`
  files as class 4.
- **Say something to the HUD:** `say` for a man. `game_msg` with `radio: ` for control. Anything else in `game_msg` reaches nobody.
- **A strip's numbers:** `rust/game_types/src/knobs/cues.rs`; the edge and the click: `squadnet.rs`. Both turn on a panel or by `?knob.cues.rows=4`.
- **A door's name:** `doorNames` in the level's generator, then `bun run tools/levels/export.ts <name>`.

## What holds it

Native tests in `rust/game_start/tests/`, and each module's own tests, by `gate tests`:

| file | what it holds |
|---|---|
| `radio.rs` | a line as the scene says it, its tone, the strip's ranking, each HUD told its own in a game of two |
| `held_words.rs` | no count in a row, no order row past digit five, no door and no route said by a compass word |
| `net_quiet.rs` | calls that get worse, the last line only for the last man |
| `squads_radio.rs` | a drop found by a radio check, nobody walking out at a noise |
| `civilians_held.rs` | a held civilian talks, told to stay down she stays, and she tells the first man who finds her |
| `standoff_held.rs` | a man who finds a hold calls it in, once, and talks to it |
| `attention.rs` | a guard's lines as his attention moves and breaks |
| `rescue_ends.rs` | a hostage rescue's "secured" before its end |

`bun run tools/qa/checks.ts words` runs `mystery-check.ts` and `writing-check.ts`, which read mission rows and lessons and no spoken line (`writing.md`). `bun run tools/qa/view-check.ts`
holds the HUD to importing nothing of the game.

## Not seen

How often lines come in a fight was read in tests, not felt. No check sees a bubble or the strip on a GPU.
Spoken lines have no lint: a line over length, a banned word in a mouth or a bad token shows only by reading the table. `MAX_CHARS`, `AIM_CHARS` and `originality_blocklist` stand for
that, and nothing reads them. Much that is written to the player through `game_msg` reaches no screen.
