# twopointsix — context for agents

**This file is loaded into every session: it is the index and the rules every session needs, under 12 KB.** What a system is, where its code
is, its rules and how to change it stand in one page a system under `docs/agents/`, true to the code today. Read the pages your work needs,
and mend a page in the commit that makes it false. `bun run tools/qa/docs-check.ts` holds every name in them, and their size.

## What this is

A top-down **squad game** on raw WebGPU: Sam (the player) and up to four AI teammates under an orders ring, box levels where every light is
live, enemies who see and hear by the same light, smoke and sound paths the player is shown. **The game is Rust** (`rust/`), one WebAssembly
module; **the page draws it** (`src/`, TypeScript on Bun, **no npm dependencies**). `docs/agents/overview.md` has the whole map.

## Run, build, check

```bash
bun run dev            # tools/serve.ts: the game at http://localhost:5173, also /sounds and /music
bun run build          # tools/build.ts: the static build
bun run typecheck      # src/ (with no vendored compiler: a system tsc --noEmit -p tsconfig.json) · typecheck:qa for tools/qa
(cd rust/game_start && cargo run --profile gate --bin gate -- tests [BINARY [TEST]])   # the native tests: the game's rules
(cd rust/game_start && cargo run --profile gate --bin gate)                            # what a build is held to (gate pin, breaks, long)
bun run tools/qa/checks.ts [group|name …]     # picture, shaders, page, menus, sounds, levels, words, these pages: a minute
bun run tools/qa/pages.ts [group|name …]      # the real page in a browser on the software adapter, one script at a time
bun run tools/blitz/face-build.ts             # the module, after a change under rust/
(cd rust/game_start && cargo run --profile gate --bin half)    # the page's half, after a change to what is drawn, told or commanded
```

In full, with exit codes, hooks and address words: `docs/agents/qa.md`. A page driven from scratch: `.claude/skills/verify/SKILL.md`. A level,
from its idea to its row: `.claude/skills/level/SKILL.md`.

## Rules every session needs

- **Where a rule goes.** A rule of play is Rust, in its module under `rust/game_rules/src/`, with its numbers under
  `rust/game_types/src/knobs/`. None is written in `src/`: the page asks the game and draws the answer. `tools/draw/ban.ts`, the
  build's first step, is red on a file that makes the game do something and isn't one of those it names, or that writes its memory.
- **A new rule lands on,** with a native test that fails without it. It gets an off word only if somebody asks for one.
- **Strong general systems.** No special case for one level, one door or one man where a system can carry it.
- **The same inputs make the same game.** No clock, no die but the game's own (`rust/game_rules/src/tick/dice.rs`), no order that is not written
  down, all state reachable from `Game`.
- **One model for the picture and the AI.** What a man senses is what the picture shows. No number off the GPU reaches a rule.
- **No GPU here.** Native tests see no picture and hear nothing; a page script runs on the software adapter. Say what was checked and how,
  and "never seen on a GPU" of the rest. "Verified" in full means driven in a real browser with no console or GPU errors.
- **For the player.** Show, not tell. No digit above 5 on a menu. No compass words on the HUD or in a door's name. Harder by smarter enemies,
  never by hit points. Teammates walk at the player's pace and run only for a reason (`teampace.rs`). Nothing is painted on the world for the player.
- **Small commits.** Build it, drive it, type check, commit. A change that moves play says what moved and what was measured; one that moves a
  pin says why (`rust/game_start/pins.txt` is made by `gate pin`, never by hand).
- **Git.** Work on your own branch off `twopointsix`, whose head is the build to play and moves only by the merge of a finished branch. Never
  push to it or to `main`; no force-push; never `git stash`. Unsigned commits are fine (`git -c commit.gpgsign=false commit`). Human-style
  messages: no attribution lines, no tool trailers, no model names.
- **This repo is public.** Nothing from outside the game goes into a file or a message: no private links or ids, no names, no machine paths.
- **Servers and measurements.** Never `pkill -f tools/serve.ts`: stop a server by pid or port. Do not edit `src/` under a running measurement.

## Writing for the player

Every word a player reads or hears (tutorial lines and cards, mission blurbs, menus, HUD rows, radio lines) is written the way you'd
say it out loud to a friend who is playing next to you. Read each line aloud before you commit it: if you wouldn't say it, rewrite it.
The maintainer's model, from a tutorial card:

> Before: "Where the sound was he stops, his back to you."\
> After: "He'll check out the sound and then stop, with his back to you."

- **Who, then what they do, then the rest.** Subject, verb, object, and only then where, when or how. Don't open on a place or a
  participle ("Stacked on a door, they wait") and don't tack on a verbless phrase ("his back to you"). A plain "when" or "if"
  clause in front is fine: "When he stops, sneak up behind him."
- **Talk to "you", and you press the key.** "Press [E] to grab him", not "[E] takes hold of him" and not "[E]: grab".
- **Use contractions** (he'll, you'll, don't, it's, they're), "will" for what is about to happen, and an order for what to do.
  No sayings: "Men fight by what they can see" is "Enemies can only fight what they can see."
- **Join ideas with words** (and, then, so, because, with, to), not with a colon, a dash or a bare comma.
- **Everyday American words**: grab, flashlight, breaker box, around, enemy, guard. Not "take hold of", "torch", "mains box",
  "round", "a man", "your word".
- **Short is good, clipped is not.** Cut ideas, never the small words (the, a, to, you, press). "Show, not tell" means fewer
  screens of text, not telegrams. A character cap is there for layout: if a natural sentence doesn't fit, say less or raise the
  cap openly. Never squeeze the sentence.
- **A voice keeps its brevity, not odd grammar.** A radio call can be "stacking up." or "room clear."; it still runs in the order a
  person would say it.

The banned words stand (no compass words, no digit above 5) and a key is still written as its code in brackets (`[B]`). Longer
notes and before and after pairs from the game's own text: `docs/agents/writing.md`.

## The pages (`docs/agents/`)

- `overview.md` — what the game is, the two halves, one tick, where everything stands, how to change it, what holds it.
- `qa.md` — run, build and check; the module; `gate`, `hold`, the checks and page scripts; the address words and `window` hooks; what "verified" means.
- `missions.md` — the list, each kind of mission staged, won and lost, par, the tutorials and their scripts.
- `menus.md` — the title, start page, kit, HUD, rings, cards, Options and keys, the debugger.
- `squad.md` — the order keys and the ring, every order, who takes it, parts of the squad, following, pace, covering unasked.
- `door-tools.md` — what the squad does at a door on order: stacks, cans, charges, locks, the entry, corners, after it, what is refused.
- `team-feel.md` — how teammates fight, look and keep quiet unasked: the room fight, night vision, lights, silent, sneaking.
- `enemies.md` — defenders, night staff and civilians: senses, states, difficulty, squads, pursuit, nerve, the dark, the response team.
- `dialogue.md` — who says what, how a line is picked and reaches the screen, door and place names.
- `stealth.md` — the fog of war, the meters, lights as things, Sam's close work, the wall hold, the whistle, stuck cameras, the wand, devices.
- `player.md` — Sam's feet, aim, guns, throws, kit, rows; friends in the line; being hit; restarts.
- `doors.md` — doors and furniture: which way one opens, locks, the boot, a leaf's weight, who works one from where, the level rules.
- `destruction.md` — what breaks: rounds, blasts and wall charges on walls, furniture and glass; debris, marks, restarts.
- `weather.md` — hours, weather kinds, roofs, wind, fire, snow, wet men; what of it moves a rule and what is dress.
- `levels.md` — the level file, its generators and kits, rooms, marks, lamps, paint and the look, what a level owes and what checks it.
- `anim.md` — the skeleton, the layers that pose a man, the turn, falls, outfits and materials.
- `render.md` — the frame, the box world, smoke as drawn, exposure, fog, glass, presets and the ladder, shader text, measuring, judging a picture.
- `render-light.md` — direct light, the gather, the cascades, the denoiser, reflections, TAAU and sharp direct, men in rays.
- `replays.md` — the tape, why a run is the same again, the kept runs, the replay page, saves.
- `net.md` — two players: the wire, the server, the lobby, seats in the rules; the hosted build and its backend.
- `rust.md` — writing Rust here: the crates, determinism, state, borrowing, asking the world, adding anything, leftovers.
- `ai-architecture.md` — a man as data, the order he is thought in, adding a reaction or an order, where it is going. Read it before adding a behavior.
- `sounds.md` — how a sound is asked for, its path to the ear, the mixer, the table, the score, the listening pages and sound lanes.
- `rulings.md` — the maintainer's rulings that stand on no system's page.
- `writing.md` — how player text is written, by kind of text, on the game's own lines.

Other maps: `STYLE.md` (the look as rules), `docs/level-format.md` (a level file, member by member), `docs/library.md` (every piece and paint a
level can use), `rust/game_start/README.md` (writing a native test, the gate), `tools/blitz/FACE.md` (the module), `cloud/README.md` (the
backend), `tools/chars/README.md` (the cast).
