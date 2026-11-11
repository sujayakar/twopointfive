---
name: verify
description: Build/launch/drive recipe for verifying changes to the twopointsix WebGPU game (a page in the browser) in a real browser.
---

# Verifying twopointsix

The game is Rust in `rust/`, built to one WebAssembly module. `src/` only draws it. So a rule is verified by a native test, not a browser, and the page is for what
crosses to the page. "Verified" in full means driven in a real browser with no console or GPU errors. There is no GPU on a cloud machine: say which step you reached
(a native test, the gate, a check, a page script, played on a GPU) and say "never seen on a GPU" of the rest. `docs/agents/qa.md` has every command, address word and
hook. This page is the working recipe and the traps.

## What verifies which change

| change | ask it with |
|---|---|
| a rule of play | a native test in `rust/game_start/tests/` that fails without it, run by `cargo test --test <file>` in `rust/game_start` |
| a change meant to change no play | `hold keep DIR` on the old tree, `hold check DIR` on the new one (`cargo run --release --bin hold --`) |
| a change meant to change play | `missions > before.tsv`, the same to `after.tsv`, `missions diff before.tsv after.tsv`, then `gate` and `gate pin` with the reason in the commit |
| anything in `rust/` | `gate` (`cargo build --profile gate --bin gate` in `rust/game_start`), and the module rebuilt before a page asks it |
| a section, command or view | `cargo run --profile gate --bin half` in `rust/game_start`, then a page: `half --check` is red where a file would change |
| a shader | `bun run tools/qa/checks.ts shaders`, then `node tools/qa/wgsl-compile.cjs` (it parses and type-checks, a minute), then a page boot, which alone makes the pipelines |
| the renderer's look | a `picture` check if one fits, a boot with a screenshot, and "never seen on a GPU" |
| page code or `src/ui/` | `bun run typecheck`, `bun run tools/qa/checks.ts page menus`, the page script nearest it (`bun run tools/qa/pages.ts menus`) |
| words a player reads | `bun run tools/qa/checks.ts words` |
| a sound or the score | `bun run tools/qa/checks.ts sound`, and with a dev server up `PORT=<port> node tools/draw/sound-tapes.cjs --check`. Nobody here hears it |
| networked play | `bun run tools/qa/pages.ts coop`, `bun run tools/draw/net-check.ts` |
| saves or replays | `bun run tools/qa/pages.ts saves replay` |
| a level | `.claude/skills/level/SKILL.md` |

`bun run qa` is the typecheck, every check and the gate. `gate long` also plays each squad mission to its end and the tutorials by keys. A pin holds a game to
itself, never that it is any good.

## The recipe

**1. The module.** `bun run tools/blitz/face-build.ts --where` says what this machine would serve. On a checkout where the checked-in module is "of these sources"
you need no cargo and no build. If you changed a source of `FACE_CRATES`, the dev server builds on the first ask (the `play` profile, about 7 GB of memory, while the
page waits on a 202). To build first: `bun run tools/blitz/face-build.ts --profile play`.

**2. The server.** One on its own port, with no backend. Leave 5173 alone, and stop yours by pid.

```sh
PORT=5297 bun run tools/serve.ts > /tmp/serve.log 2>&1 &  SERVER=$!
# …
kill $SERVER
```

**3. The browser.** `tools/qa/page.cjs` finds Chromium (`CHROMIUM`, `CHROME`, `/usr/bin/chromium` or Playwright's), Playwright (`PLAYWRIGHT_MODULE`, `NODE_PATH` or
`/opt/node-tools/node_modules/playwright`) and the software adapter's flags (`SOFTWARE`). This script ran as it stands on a machine with no GPU:

```js
// PORT=5297 node <this script> <still.png>, from the top of the tree
const { page, NOISE } = require(`${process.cwd()}/tools/qa/page.cjs`);
const P = page({ name: 'drive' }), { check, note } = P;
(async () => {
  const { page: w, errors, go, ready, step } = await P.open({ size: [1280, 720],
    errors: { also: /Invalid|error:/, skip: NOISE },              // WGSL and validation faults come as plain console lines
    init: [() => { window.__testVisible = true; }] });            // a hidden page counts as seen
  await go('paused&quality=verylow&mute&cloud=0&seed=1&roomclear&squad=2&levelFile=killhouse.json', { timeout: 600000 });
  await ready();                                                  // sets the hooks, once the pipelines compile
  const t0 = await w.evaluate(() => window.__draw.game.time);
  await step(120, ['KeyW'], 'last');                              // two seconds with W held, the last frame drawn
  const got = await w.evaluate(() => { const g = window.__draw.game; return { time: g.time, tally: g.tally(), men: g.map().men.length }; });
  note(JSON.stringify(got));
  check(got.time > t0, 'the game moved', `${t0} to ${got.time}`);
  await w.evaluate(() => window.__draw.engine.device.queue.onSubmittedWorkDone());
  await w.screenshot({ path: process.argv[2], timeout: 240000 });
  check(errors.length === 0, 'no page, console or GPU error', errors.join('\n'));
  await P.end();                                                  // closes the browser, exit 1 on a FAIL
})().catch(e => { console.error(e); process.exit(2); });
```

Look at the picture. `P.open` keeps thrown errors and console errors. GPU validation reaches the console through the `uncapturederror` listener in
`src/gpu/device.ts`, as a line that starts "WebGPU error:". A boot that dies raises the red card (`#fatal`), and `window.__ready` rejects.

To keep a script, put it in `tools/qa/` with a row of `PAGES` in `tools/qa/pages.ts`, which starts a server of its own and hands the script `PORT`.

## Driving it

**Address.** `?paused` gives the whole page with its loop held until stepped: the start of any script. A mission is a page load of its own words: `?roomclear`,
`?squad=2`, `?squad4`, `?rescue`, `?quiet`, `?nightbreach`, `?levelFile=depot.json`. `?seed=7` is one die, `?quality=verylow` a fixed preset, `?mute` silence,
`?cards=0` no card waits and `?cloud=0` nothing sent. `?knob.doorthrow.kick_secs=3` sets any number of the rules. A plain address opens the title.

**Hooks** (set when `main` is done):

| hook | use |
|---|---|
| `__draw.run(n, keys, draw)` | `n` ticks of a sixtieth with those codes held. `draw` is `'none'`, `'last'` (default) or `'all'`. Takes a title or a brief down first |
| `__draw.game` | `view()` the HUD's record, `map()` the men and doors, `floor()`, `tally()`, `run()`, `saves()`, `time`, and `commands` (`god(1)`, `save()`, `load(key)`) |
| `__draw.cam` | `cam.project(point, width, height, 1)` is where a world point falls on the page; `cam.pin` holds the camera for a still |
| `__draw.input`, `.engine`, `.quality`, `.hud`, `.pause`, `.smoke` | the page's parts |
| `__engine`, `__quality`, `__panel`, `__boot`, `__game` | the renderer; presets; the debugger (`toggle()`); how the boot went; time and stage |
| `__testVisible`, `__timeScale`, `__testFocus` | set by a tool: a hidden page counts as seen; game time's rate; somebody at the window |

**Keys.** `step` holds `KeyboardEvent.code`s and files no press, so a key read as a press wants `__draw.input.pressed.add(code)` first, or a real key:
`w.keyboard.down('Digit5')`, a step, then `up`. The mouse is real too (`w.mouse.move(x, y)`, `w.mouse.down()`). A real press goes through `src/host/keymap.ts`, and a
code does not. So the squad's command is Q on the keyboard and `'KeyB'` as a code, and B on the keyboard gives nothing. F is the go word, the digits are the kit,
backtick empties the hands and backslash opens the debugger. F5 saves, F6 captures a frame and F7 keeps a replay.

**To aim:** read a place from `__draw.game.map()`, then `__draw.cam.project`, then `w.mouse.move`, then step two frames. `tools/qa/tut-road-page.cjs` has a full
walker: real keys, a route around what stands, a click on a man, the HUD's line read off the DOM (`#railprompt`).

## Traps

- **A drawn frame can take a minute on the software adapter.** Step with `'none'` and draw only the last. `__runFrames` is the old name and draws every frame
  by default, and a script about keys can stub the renderer with `__engine.render = () => {}`. Then it saw no picture at all, so say so.
- **A boot takes a minute or more here.** Give `go` and the wait for `__ready` long timeouts. `QA_PAGE_CAP` is a script's cap under `pages.ts`, 10 minutes unless set.
- **Playwright's clicks hang on the software adapter.** Click by script (`el.click()` in `evaluate`) or by `mouse.down` and `mouse.up` around a step.
- **`NOISE` is an adapter's complaint** about smoke kernels it refuses. Smoke is not tested by a software boot unless the script steps the solver and reads it back,
  as `smokejump-page.cjs` does. Skip nothing else.
- **A driven page stands still.** `navigator.webdriver` or `?paused` makes the page whole, with its hooks on its own `window`. A player's Chrome has the game in a
  frame (`kept-page`), hooks on its `contentWindow`; `?keeppage=1` shows that one. Do not trust a `requestAnimationFrame` wait or wall time: step.
- **A screenshot shows the last frame presented.** Draw one first and give it a long timeout. It says that rows reach the renderer, never how it looks on a card.
- **A fresh context has no `localStorage`, and a person's browser may.** Difficulty, kit cards, cheats and panel rows are kept there, and an address word wins.
- **A save is a clone in the module's memory.** A reload loses it, and an autosave waits while anybody is alert (`run_no`), so a script that sees none may be right.
- **One edit to a source of `FACE_CRATES` and the checked-in module is not served,** because the page's half is made from the same declarations. `--where` then says
  "of OTHER sources", a page's first ask starts a build, and `kitview-plan.ts` finds no module.
- **Comments in Rust and `src/` are partly stale.** Trust statements.
- **Do not edit `src/` under a running measurement.** Never `pkill -f tools/serve.ts`.

## What nothing here sees

A GPU's picture, GPU timings, a sound, and whether a fight is fun. All of that stays unseen until somebody has played it.
