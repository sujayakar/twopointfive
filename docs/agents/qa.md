# Running, building and checking

How to run the game, how its module is built, what holds a change, how to drive the page by hand or by script, and what "verified" may be said of.
`rust/game_start/README.md` has how to write a native test, and the gate, `hold`, `missions` and `levels` in detail. `menus.md` has the debugger panel, `replays.md` tapes.

## What it is for

The command that tells whether a change holds, from one native test up to the real page in a browser. None of it needs a GPU, and none of it can see one.

## Where the code is

| file | what it does |
|---|---|
| `tools/serve.ts`, `tools/build.ts` | the dev server; the static build |
| `tools/blitz/face-build.ts`, `tools/blitz/FACE.md` | where the module's file is, whether it is stale, the build |
| `rust/game_face/prebuilt/` | the checked-in module and `SOURCES`, the hash of what it was built from |
| `rust/game_start/tests/` | the native tests |
| `rust/game_start/src/bin/gate/`, `rust/game_start/pins.txt` | `gate`, and what its rows are held to |
| `rust/game_start/src/bin/` | also `hold.rs` (with `rust/game_start/hands/`), `missions.rs`, `levels.rs`, `blastcost.rs` (what a wall blast's tick costs), `half/` (the page's half) |
| `tools/qa/checks.ts` | every check that stands no game of its own: one table, by group |
| `tools/qa/pages.ts`, `tools/qa/page.cjs` | the page scripts' table and runner, and what each script shares |
| `src/host/drawpage.ts` | the page, and the `window` hooks at the foot of its `main` |
| `src/host/keymap.ts`, `src/host/machine.ts` | the keys; what this machine keeps |

## How it works

**Run and build.**

```sh
bun run dev              # tools/serve.ts with a local backend: http://localhost:5173 (game at /, /sounds, /music, /look)
bun run tools/serve.ts   # the same with no backend; PORT= and HOST= move it (it listens on loopback)
bun run build            # tools/build.ts: tools/draw/ban.ts first, then the dist folder with the module beside the page
bun run typecheck        # src/ · with no vendored compiler, a system tsc --noEmit -p tsconfig.json
bun run typecheck:qa     # tools/qa, by its own tsconfig
bun run qa               # the typecheck, then tools/qa/checks.ts, then the gate
```

A static build with no module says so and goes on, unless `FACE_BUILD=1`.

**The module.** `rust/game_face` is the game as one WebAssembly file. The dev server, the static build and the tools take the first of these (`fresh` in `face-build.ts`):

1. a build of this machine that no source is newer than, the asked profile's and then the other's;
2. the checked-in module, if the hash in `SOURCES` is this checkout's hash (`sourcesHash`);
3. nothing: the dev server starts a build and answers 202 with a line, and the page asks again until it is done. With no cargo it serves an old build, or answers 503 with
   the one command that mends it.

The sources are the Rust files of `FACE_CRATES`, their `Cargo.toml` and `build.rs`, and `rust/rust-toolchain.toml`. A `src/bin` program, a test or a module the wasm build
leaves out moves no hash. Two profiles: `play` (no link-time optimization, about 7 GB of
memory, what the dev server builds) and `release` (the fat link, over 10 GB, what `bun run build` builds, unless `FACE_PROFILE=play`).

```sh
bun run tools/blitz/face-build.ts [--profile play|release]     # build if stale, say where the file is
bun run tools/blitz/face-build.ts --where                      # which module this machine would serve
bun run tools/blitz/face-build.ts --prebuilt                   # make the checked-in one: play, with this machine's paths mapped away
```

`--check` only says if it is stale. `--prebuilt` refuses a file that carries a path of this machine, and needs a new build: if `play` is up to date, touch a source first.

**Native tests.** `cargo test` in `rust/game_start`, or `gate tests [BINARY [TEST]]`, which builds the tests of every crate that has any (the list heads `tests.rs`) and runs a
process a binary. Ask a rule there first; a page script is for what crosses to the page.

**The gate** is one native command with no page, no TypeScript and no module: `cargo build --profile gate --bin gate` in `rust/game_start` makes target/gate/gate there.
It plays every mission's start and a few runs,
each row in a child process, so a panic costs only its own rows. Exit 0 pass, 1 a row failed, 64 a wrong word.

- `gate` is the quick kind: rows `start`, `play`, `again`, `save`, `load`, `draw`, `seats`, `par` and `level file`. `gate save` runs the rows whose name starts with that word.
- `gate long` adds `played` (each room clear, rescue and night breach to its end by the hands-off player), `lesson` (the tutorials by a man at the keys)
  and `level` (each other question of `levels`).
- A pin is the figures' word (`game_start::figures`, what play is in plain numbers). Figures moved is play moved, and red; only the parts moved passes and says so.
- `gate pin [ROW…]` records the pins again and says which moved. `gate breaks` plants each check's fault: the row must pass unbroken, then fail in its check's own words.

**The other programs**, by `cargo run --release --bin <name> --` in `rust/game_start`.

- `hold keep DIR`, then `hold check DIR` on the changed tree: for a change meant to change no play. It names the first frame and the parts that differ.
- `missions > before.tsv`, the same to `after.tsv`, then `missions diff before.tsv after.tsv`: for a change meant to change play, never a claim about how hard a mission is.
- `half`: writes the page's half (`src/draw/sections.ts`, `commands.ts`, `asks.ts`, `src/view/read.ts`) from what Rust declares; `half --check` is red where a file would change, as
  `tests/half.rs` is, and a module of another stamp is refused as a page loads.
- `levels second-way | sight | walked | stacks | stands | doors | charges | names | rain`: the level tool, exit 1 a FAIL line.

**Checks.** `bun run tools/qa/checks.ts [group | name …] [--list]`, groups `picture`, `shaders`, `page`, `menus`, `sound`, `levels`, `words`, `build`. Each is a file run in its
own process, stopped at `CHECK_CAP` minutes. `BY_HAND` names the files no row runs: the backend's, the
game server's, and those of `tools/draw/` that want the module.

**Page scripts.** `bun run tools/qa/pages.ts [group | name …] [--list]`, groups `boot`, `play`, `menus`, `coop`, `saves`, `cloud`, `replay`, `all`, and `tools`, whose scripts run
only by name. It starts one dev server with no backend and runs a script at a time, because a page takes the whole box.
`QA_PAGE_CAP` is a script's minutes. A machine with no GPU needs a module (the checked-in one of these sources, or a build), and (`page.cjs`):

- **A browser:** `CHROMIUM` or `CHROME`, or `/usr/bin/chromium`, or the one Playwright installed.
- **Playwright:** `PLAYWRIGHT_MODULE`, `NODE_PATH`, or `/opt/node-tools/node_modules/playwright`.
- **The software adapter:** `SOFTWARE` is the flags, and `NO_GPU` is no adapter at all for plain DOM (`gpu: false`). `NOISE` skips lines about smoke kernels an adapter refuses.

A drawn frame is very slow there. So a script steps with nothing drawn, and one about keys stubs the renderer with `__engine.render = () => {}`: it saw no picture at all, so say so.
Playwright's clicks hang on the software adapter: click by script.

**`tools/draw/`**, by `bun run`. The build runs `ban.ts`; run the others a change touches.

| file | what it says |
|---|---|
| `ban.ts` | the page imports no game code and nothing a level is made with; the build's first step |
| `held.ts`, `unused.ts` | what the pages are built from and what holds the other files; declarations nothing uses (`TS_LIB` or the vendored compiler) |
| `play-check.ts` | every mission on the module: started, stepped, drawn, saved, loaded, lost to a planted panic, restarted |
| `hands-check.ts`, `net-check.ts`, `keep-check.ts` | the benchmark's scenes played hands-off; a game of two on the page's half; a run kept in beats |
| `reach-check.ts` | storms on six levels: no reach mask made in the frame that shows its light, and the bytes as made whole (`REACH_BREAK`) |
| `tape-check.ts`, `walk-check.ts`, `walk.ts` | a tape walked on another game is the run, at every mark and after a seek; one tape read and walked here |

**The address.** A plain address opens the title (`startPageWanted`). A mission is a page load of its own words, read by the game (`Query`) and by the page.

| words | what they do |
|---|---|
| `?roomclear`, `?squad4`, `?squad=2`, `?nightbreach`, `?rescue`, `?quiet`, `?levelFile=depot.json` | which mission; with no file it is `office.json` |
| `?paused` | a whole page, no load brief, its loop held until stepped: the start of any script |
| `?seed=7`, `?quality=verylow`, `?mute`, `?hud` | the die (no word, a random one); a fixed preset; silence; the numbers on screen |
| `?hard`, `?nofow`, `?noweather`, `?hour=` | difficulty; no fog of war; no weather; the hour |
| `?cards=0`, `?loadbrief=0`, `?nokit`, `?noreplay`, `?cloud=0`, `?downloads=1` | no card waits; no brief; not this machine's kit cards; no tape; nothing sent; files also download |
| `?knob.doorthrow.kick_secs=3` | any number of the rules, by name. The short words are `WORDS` in `rust/game_start/src/host/words.rs` |
| `?playtape=`, `?at=`, `?bench` | a tape's page and the tick to seek to; the benchmark |
| `?keeppage=0`, `?keeppage=1` | the shell that keeps the GPU device. A driven or `?paused` page is whole. A player's Chrome has the game in the frame `kept-page`, hooks on its `contentWindow` |

The picture's own words are in `src/host/pictureopens.ts` and `src/host/engineflags.ts`.

**Hooks** (set when `main` is done; `await window.__ready` first):

| hook | what it is |
|---|---|
| `__draw.run(n, keys, draw)` | `n` ticks of a sixtieth with those codes held, `draw` being `'none'`, `'last'` (its default) or `'all'`; it takes a title or a brief down first |
| `__runFrames(n, dt, keys, draw)` | the same by its older name: `dt` is ignored, and its default draws every frame |
| `__draw.game` | `DrawGame`: `view()` the HUD's record, `map()`, `floor()`, `tally()`, `saves()`, `time`, and `commands` (`god(1)`, `save()`, `load(key)`, `autosave(0)`) |
| `__draw.input`, `.cam`, `.hud`, `.engine` | the page's parts; `__draw.held` holds its loop, as `__pauseLoop` does |
| `__game`, `__engine`, `__quality`, `__panel`, `__boot`, `__perfrec`, `__framecap` | time and stage; the renderer; presets; the debugger (`toggle()`, `visible`); how the boot went; the recorders |
| `__timeScale`, `__testVisible`, `__testFocus` | set by a tool: game time's rate; a hidden page counts as seen; is somebody at this window |

Real keys work too: a keydown on `window`, or Playwright's keyboard.

**Keys for a checker.** F5 saves (only in a run). F6 captures a frame to the captures folder (Shift: a small window), read with `tools/qa/capture-read.ts`. F7 keeps a replay. F8 prices the
heavy features one at a time. F9 records frames, read with `tools/qa/perf-report.ts`. Backslash opens the debugger: find any knob by a word of its name and turn it live. The Cheats on Options are God
mode, enemies that don't think and unlimited ammunition.

**Saves and what a machine keeps.** A save is a `Moment`: a clone of the whole game in the module's memory, so a reload loses it. Saves by hand and autosaves are two rings, and a fresh run starts with
none. In `rust/game_rules/src/tick/savestate/`, `run_no` refuses for two players, Sam down, a run over or inside the response team's call, and `gate` there holds an autosave back while
anybody is alert, searching or above `aware_max`. So a script that sees no autosave may be right. Kept runs are in IndexedDB (`keptStore`).

What keeps a page from playing as it did yesterday is `localStorage`: the difficulty, the kit cards, the Cheats, and panel rows that were turned (`machineKept`, `machineAfter`). A fresh Playwright
context has none of it, and a person's browser may. An address word wins over what is kept (`rust/game_start/src/host/mod.rs`, `lay`), on a native start too.

## Rules that hold

- **"Verified" in full means driven in a real browser with no console or GPU errors.** A page script does that on the software adapter, with no picture worth judging. Say which step was
  reached: a native test, the gate, a check, a page script, played on a GPU. Say "never seen on a GPU" of the rest.
- **A check has a red path.** `<NAME>_BREAK=1` at a file's head, `gate breaks` for the rows. A new check ships with one, and it must fail.
- **A FAIL line is red whatever the exit.** `checks.ts` and `pages.ts` read lines that begin with it. A `*-check.ts` in `tools/qa/` or `tools/draw/` that neither a row nor `BY_HAND` names is red too.
- **`pins.txt` is made by `gate pin`, never by hand.** A commit that moves a pin says why. A move nobody meant is a fault, and `hold` finds its part and frame.
- **A pin holds a game to itself, never that it is any good.** A mistake made at a pin passes.
- **The quick gate stays quick.** "can we slim down the gate? should be no more than 15m". What is slower goes in `gate long`.
- **A file never leaves the page as a download.** "we should get rid of local downloads entirely". It goes through `src/host/savefile.ts`, and its saver says what became of it.
- **The module is never of other sources.** A checked-in module whose hash differs is not served, because the page's half is made from the same declarations and would read garbage.
- **Stop a server by pid or port.** Never `pkill -f tools/serve.ts`. Do not edit `src/` under a running measurement.

## To change it

- **A native test:** the README, "Write a behaviour test". A new crate with tests goes in `CRATES` in `tests.rs`, or the run fails.
- **A gate row:** a kind in `rows.rs` (`all` or `long`), a job in `Job`, and a row in `breaks` with the words its failure must hold. Then a whole `gate pin`, since only that writes the list of rows.
- **A check:** a file in `tools/qa/` with a usage line and a red path, and a row of `ROWS` in `checks.ts`.
- **A page script:** `page({ name })` from `page.cjs`, `P.open`, `check`, `P.end`, and a row of `PAGES` in `pages.ts` with the minutes it took (0 means not read).
- **A section, command or view:** the Rust declaration, then `cargo run --profile gate --bin half` in `rust/game_start`.
- **Rust the page should play with no cargo:** `bun run tools/blitz/face-build.ts --prebuilt` where cargo runs, in the same commit. No check asks this; `--where` says how it stands.
- **A hook or an address word:** the foot of `main` in `drawpage.ts`; a rule's word in `words.rs`. A word about the visit and not the run goes in `VISIT_FLAGS`.

## What holds it

`gate breaks` holds the rows, `gate tests --break=BINARY` shows a red is read, and `DOCS_BREAK=1 bun run tools/qa/docs-check.ts` the docs check. `tools/qa/rust-rules-check.ts` holds `FACE_CRATES`
to the crates' locks. Native tests of the machinery: `readme.rs` (the README's example, word for word), `start.rs`, `restarts.rs`, `saves.rs`, `held_saves.rs`, `draw.rs`, `tick_ends.rs`,
`owes.rs`, `hosts_agree.rs`. Pages: `savetimeline-page.cjs`, `panel-keys.cjs`, `nogpu-page.cjs`, `tape-page.cjs`. `savefile-check.ts`.

## Not seen

No check here has seen a GPU, heard a sound, or judged a fight. No check says that the
checked-in module is of the head of the branch. `menustills-check.ts` and `sweep-ir-check.ts` have no red path.
