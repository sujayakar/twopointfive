# Writing for the player

How the words a player reads are written and where each kind lives. `CLAUDE.md` has the short rules ("Writing for the player"): this page shows them on the game's own text
and says what each kind of text needs. Who says a spoken line and when is `dialogue.md`. The cards and rows as a screen is `menus.md`.

## What it is for

A player reads these at a glance, in the middle of a fight, and they should sound like a friend beside them. It covers the lesson's big line and its cards, a mission's opening scene and
"You will learn" list, blurbs, menus, HUD rows, the end card, and what a teammate, a guard or a hostage says. It is not for notes in this folder, commit messages or the debugger's rows. Do not
carry the voice of these notes into the game. They run noun first with no contractions, and the game does not.

## Where the code is

| text | where it is written | how it is read |
|---|---|---|
| a lesson's lines and cards | `tools/levels/tut-*.ts`, written out to `levels/tut-*.json` as `line`, `after`, `explain` | `lesson_line`, then `capsPlain` in `src/ui/tabcard.ts` |
| a mission's row: `blurb`, `scene`, `teaches`, `only`, `place` | `rust/game_types/src/scene_scenario_list/rows.rs` | `src/ui/loadbrief.ts`, `start_list` |
| the objective | `mission::objective` and the modules it asks | the view |
| a row beside what Sam can use | `interact_view`, `player/offers/` | the view |
| squad rows and answers | `orders/`, `orders/rows/`, `doortools/` (`squad.md`) | the view |
| spoken lines | `rust/game_types/src/dialogue/types.rs` and each rule's own tables (`dialogue.md`) | the view |
| menus | `src/ui/`, for instance `startpage.ts` | the page |
| what a key is called | `src/host/keymap.ts` | `keyText`, `capsPlain` |

`tools/qa/writing-check.ts` and `tools/qa/mystery-check.ts` are the checks; `tools/qa/lessonwords.ts` writes a token out for them.

## How it works

**A key is written as its code in brackets.** The page draws the player's own key. [B] is the squad's key and [Space] the hand's. [X] means the squad's key held, and is drawn "Hold Q" at
the start of a sentence, so it may open one ("[X] on the wall"), which no other key does.

**A move the keys can change is a token**, so write the sentence around it. There are four: {grab him}, {grab him from behind}, {knock him out} and {take him out}, with any words for
"him" ({grab the near guard}). A capital inside the braces begins a sentence. `token_line` in `lesson.rs` writes them out by the keys the build has: "press [Space] to grab him",
"right-click to knock him out". Read the line aloud each way it can come out, and never spell the grab's key yourself. The same list stands in `tools/qa/lessonwords.ts`; change both.

**Each kind of text has a job:**

| kind | what it is | from the game |
|---|---|---|
| the big line | one short sentence that says what to do, not why | "Press [C] to crouch, and the shots will go over you" |
| a line after a press | what they're doing now (`after`) | "They're stacking up on the door" |
| a card's title | what the card is about | "Picking a lock" |
| a card | two or three sentences: what's about to happen, what to do, which key | "This door is locked. Stand at the lock and press [Space] to pick it." |
| an opening scene | two sentences, like a briefing: where you are and why | "A fuel convoy was hit on the pass road, and its escort stopped answering the radio." |
| a "You will learn" row | finishes that sentence, or names the thing | "Take cover on a wall and lean out" |
| a blurb | one or two full sentences | "The building is laid out as a ring, so whichever door you fight at, someone can walk around behind you." |
| a menu row, a button | a label stays a label until it has a verb | "Team & kit", "fall in" |
| a radio line | short, because people on a radio are short | "stacking up.", "on it.", "set.", "room clear." |
| a verdict | a word or two in capitals, never a sentence | "WIPED" |

## Rules that hold

Each pair is an old line and the line that stands in the game now.

**The test is out loud.** The maintainer's model, on the whistle card, as it now reads:

| Before | After |
|---|---|
| Where the sound was he stops, his back to you. | He'll check out the sound and then stop for a smoke, with his back to you. |

The subject comes first, "He'll" says what is about to happen, "check out" is what you'd say, and "and then" and "with" do the work of a comma and a verbless phrase.

**Who, then what they do, then the rest.** A plain "when", "if" or "once" in front is normal speech.

| Before | After |
|---|---|
| Stacked on a door, they wait for your word. | They'll wait there until you say go. |
| Kept down, the same key opens a ring with the rest of them. | Hold [Space] and a ring will come up with everything you can do there. |

**You press the key.** A key never does anything by itself, and a key followed by a colon is not a sentence.

| Before | After |
|---|---|
| Go round the wall, come up behind him, and [E] takes hold of him. | Come around the wall, go up behind him, and {grab him}. |
| [Q] on the door: stack up | Press [B] on the door to stack them up |
| [N]: night vision | Press [N] to turn on night vision |
| Right button: out cold. Left: the knife | {Knock him out} |

**Contractions, "will", plain orders.** No sayings in a timeless present.

| Before | After |
|---|---|
| He heard that, and he is coming to look. | He heard that, and he's coming to look. |
| Men fight by what they can see. In the dark they are slow, and they come with torches. | Enemies can only fight what they can see. In the dark they'll slow down and turn on flashlights. |
| A whistle brings one man | Whistle to lure a guard |

**Join ideas with words.** "So" and "because" are for a real reason ("Press [1] to switch to your pistol, because it's much quieter"). Two things that are simply both true are two sentences.

| Before | After |
|---|---|
| When he stops: up behind him. [E] grabs him | When he lights a smoke, sneak up behind him and {grab him} |
| They come with torches: the light gives them away | Two more are hiding in the yard, and you can see their flashlights |
| Hide. When he works at the box: strike | Stay hidden until he's working at the breaker box, then take him from behind |

**Short is good, clipped is not.** Cut ideas, never the small words. If a line is too long, teach less in it or split the step.

| Before | After |
|---|---|
| On, past the wall | Keep going past the wall |
| To the back door | Go to the back door |
| Lights out: what the dark does to men | Cut the lights and fight in the dark |

**Everyday American words.** One word for one thing, every time.

| Not | But |
|---|---|
| take hold of, strike | grab, take down |
| torch | flashlight |
| mains box, fuse box | breaker box |
| round, towards, whilst | around, toward, while |
| a man, men, a man of theirs | an enemy, a guard, enemies |
| your word, the word | say go, until you say go |
| out cold, as a verb | knock him out |

The squad "stacks up" on a door and goes when you "say go". An enemy is "watching" a doorway, and the enemy at a machine gun is "the gunner". Call a thing what the rest of the screen calls it.
A line that stands alone names who it is about ("your partner", "the gunner"), because a bare "him" has nothing to point at.

**A voice keeps its brevity, not odd grammar.** A radio line drops the subject and stays in the order a person says it. Squad and defender lines are lower case and a defender's may join with a
dash ("lost them — holding the mess"), because that is someone on a net. A frightened man may stammer. A teammate sounds like a present-day American soldier. The office night's staff, the
pairs and the scripts have a British voice of their own ("They've put a sensor in the gents."), and whether that changes is the maintainer's call.

**A character cap is for layout.** If a natural sentence doesn't fit, say less or raise the cap openly. `SAM_MAX_CHARS` is the only cap on words that the game enforces: a longer line of Sam's
is not said. The bans stand as `CLAUDE.md` says: no compass word, no digit above five on a menu. So does `mystery-check.ts`: no count of who is inside.

## To change it

- **A lesson's line or card:** its `tools/levels/tut-*.ts`, then `bun run tools/levels/export.ts <name>`. Then `bun run tools/qa/writing-check.ts`.
- **A mission row:** `rust/game_types/src/scene_scenario_list/rows.rs`. The checks read it off the module, so make that first.
- **A rule in the check:** `faults` in `tools/qa/writing-check.ts`. Its `BEFORE` and `AFTER` lists are what the rules must catch and pass; `WRITING_BREAK=1` reads the old sentences as a level's.
- **A spoken line:** `dialogue.md`, To change it.
- **Whatever you change:** read each changed line aloud as the player reads it, keys drawn and tokens written out. Then have a second reader who did not write it do the same before calling it
  final. A pattern catches a colon after a key or a missing contraction, and only a reader catches a sentence nobody would say.

## What holds it

| | |
|---|---|
| `writing-check.ts` | every lesson's lines and cards as written out, the four tokens, every row's `blurb`, `scene`, `only` and `teaches`, by the list below |
| `mystery-check.ts` | no row says its own count, and `src/ui` reads no count; `MYSTERY_BREAK=1` must fail |
| `held_words.rs` | the same count on the Rust list, order rows on digits to five, door names with no compass word |
| `bun run tools/qa/checks.ts words` | both checks |

`writing-check.ts` is red on a key with a colon, a sentence opening on a key but [X], a colon in a line, a bad opener or verbless tail, a word of the "not" list and an uncontracted
pronoun. The checks read data alone. Menus, HUD rows, a row's `place` and spoken lines rest on the reading above.

## Not seen

No new player has been watched reading these. The strings have been read in the source and in the checks. Some text the rules cover does not obey them: a row's
`place` can say "round" ("three huts round a floodlit yard"), and the can has four names (`dialogue.md`).
