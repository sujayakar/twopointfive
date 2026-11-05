# Run, check, drive headlessly

One of the area files CLAUDE.md indexes. Everything below was moved here word for word from CLAUDE.md on 2026-10-13, in its old order;
"the bullet above / below" in a bullet means its neighbour there (each block says which lines it was), and a bullet named by its bold
title may now live in another file of this folder: `grep -l '<title>' docs/agents/*.md`. New bullets for this area go at the foot of this file.

<!-- was CLAUDE.md lines 244–260 -->
- **THE HARNESS HAS TWO STANDS, AND ONE LIST OF WHAT SEPARATES THEM** (`src/host/profile.ts`). FROZEN (the default) = the stand every frozen digest, tally, probe bound and session
  fixture was recorded on: `src/host/standup.ts standGame({harness})` and `tools/qa/headless.ts` hold the newer rules off, each line behind `pinned('<its env word>')`. SHIPPED
  (`TPS_PROFILE=shipped` in front of any headless tool) = the page as it opens with no URL flag on a machine that has kept nothing: none of those lines runs, `shippedBoot()` calls
  `pageSwitches('')` — THE function `src/main.ts` calls with the page's own query for the switches its boot writes (destruction, furniture, wall reactions, wall charge, the door
  walk) — reads the kit from an empty store, takes the squads' page-only `?radiocheck=call`, and the cast is loaded. `FROZEN_HOLDS` there lists every row the two stands differ in (73 when this line was last touched; the table is the count, not this line)
  — since 2026-11-04 NOT the game's own maths or a row of the Rust module: both are on on both stands (`standup.ts pinMaths`, `pinRust`; `docs/agents/rust.md`, "All on"), and since 2026-11-05 neither is a
  switch: those words stop a tool that says them (`docs/agents/rust.md`, "Not switches"; a check's own arm is `QA_HOLD`) —
  with the word that raises each on a frozen run (`TPS_MOVES=1`, `GLASSFX=on`, `SLOTS=on`, `SQUADS=radiocheck-call`, …); under shipped a pin's own word still holds
  (`TPS_PROFILE=shipped TPS_MOVES=0`), except that a PLAY-THROUGH writes the door rules and chaos itself on either stand (`--set` there). Held by: `probes.ts shipped`
  (shipped-check.ts: the two stands differ in exactly those rows, plain and with a mission staged; every pin's word is in `src/host/standEnv.ts STAND_ENV`) and
  `PORT=<dev server> node tools/qa/shipped-page.cjs [name=query …]` (the REAL page on the software adapter, `window.__knobs` against the shipped stand: 0 rows may differ; ~3 min
  a page — run it when a boot switch or a page-only module default is added). `tools/qa/stand-dump.ts [--profile shipped|frozen] [--query q] [--out f] | --diff a b` = every knob
  table, `Game.tune` and the sandbox flags of a stand-up as JSON. `tools/qa/ships.ts` = the play-through under the shipped profile (`--base` = frozen, `--partial` = the five rows
  it raised before the profile existed — frags, `GLASSFX=on`, `TPS_MOVES=1`, `TPS_TURN=1`, `TPS_CAST=1` — which six `*-check.ts` scripts still ask for: their bounds were read on
  it). A plain night or a quiet mission alone stood on the shipped stand has the page's `?solo` Sam (one of each can, one round: `kit.ts kitSoloStand`, asked by `standGame` and `startFromQuery`; frozen = as built, nothing written) — and the PAGE's plain night is that Sam whichever way it was started: main.ts asks `kitSoloIf` for `?solo` and for a page with no mission, show or quiet run in its URL, so the card's own Start no longer hands him the eleven things (`PORT=<dev server> node tools/qa/start-kit-page.cjs`: Start and `?solo` carry the same, after a restart too). What shipped still is not: the player (a scripted commander who cannot be hurt), the GPU's light meter and smoke lines (headless = the CPU stand-in), localStorage, V8.
  STILL ON THE OLD FIVE ROWS and calling it 'shipped' / 'the page': kit-cells.ts, snap-probes.ts `PAGE` (framecost.ts's shipped arms and frame-spikes.ts `--ship` stand on `TPS_PROFILE=shipped` now; the five-row arm is kept there as `shipped5` / `pair5` / `--ship5`).
- **One knob registry** (`src/game/knobs.ts knobRegistry()` — built lazily: the import cycle): every module knob table by name. The session journal (`JOURNAL_KNOBS` = the subset a
  recording carries), `playthrough.ts --set` and its put-back, the probe runner's copy / put-back round every probe (`knobsCopy / knobsBack`, in place, every depth; it also puts back `KIT_SOLO.on`, which is in no table) and the page's
  `__knobs` all read it. A new table = one line there; `bun run tools/qa/knobs-check.ts` holds the round trip and is red on an exported table with an `on` switch that is neither registered nor on that file's `UNREGISTERED` list with its reason (six today: four look tables, the kit's two).
  (Added 10-13, the switches goal's step c28ba4fb:) A TABLE THE PAGE'S BOOT WRITES — any `NAME.key = …` in `host/profile.ts pageSwitches` — differs between the page and the stand a
  replay is stood on, so it is carried by a recording (its name at the END of `JOURNAL_KNOBS`) or excused on knobs-check's `PAGE_ONLY` list with how a replay gets it or why it need
  not (five today: DESTRUCTION and LIVED by a recording's env, LOOK and PLAN picture only, RECAP reads only); knobs-check is red on a written table that is neither — and on a table `journal.ts` asks a FILE for (`k.tables.X`, the rules that put a key off for a recording from before it) that no recording carries: such a rule reads every file as one from before the key (PAIR's four keys and ROOMFIGHT.doorEars stood so until 10-22).

<!-- was CLAUDE.md lines 319–323 -->
- **The gates.** `bun run tools/qa/missions-gate.ts` (`probes.ts missions-shipped`) plays every squad mission of the Missions list × 2 seeds to its end AS SHIPPED and is red on a run
  that throws or stalls (≈ 4 min at four jobs; not judged: who won); the QUIET rows (`QUIET_MISSIONS`) are played by `solo-night.ts`'s ghost on that tool's own stand, × 2 seeds — for
  them the gate holds 'starts and runs to the debrief', not 'as shipped'. `tools/qa/expected-red.ts` = the reds somebody has explained, by probe name; probes.ts prints its reds against it
  after the verdict (EXPECTED RED · NEW RED · GREEN AGAIN) and gate.sh carries the lines — the exit code is unchanged, red is red. `EXPECTED_RED_SHIPPED` in the same file =
  the rows that hold under `TPS_PROFILE=shipped` only (read on that stand alone, so a probe green on the frozen stand never reads GREEN AGAIN there). tools/qa is typechecked by its own
  `tools/qa/tsconfig.json` (`bun run typecheck:qa`). That program (every tool, `tools/mp` and the whole of `src/` behind them) takes about 2.4 GB to check, past node's default heap on a small box, so the script gives node 8 GB (`--max-old-space-size=8192`); with a system `tsc` say `NODE_OPTIONS=--max-old-space-size=8192` in front, or it aborts out of memory with no type error to show.

<!-- was CLAUDE.md lines 398–468 -->
## Run · build · check

```bash
bun run dev            # tools/serve.ts → http://localhost:5173  (loopback only, HOST=… overrides; game at /, the WORKBENCH at /workbench = level editor + character/prop viewer in one shell; /viewer /editor /gait /index.html forward; GET/POST /levels/ — POST wants X-Level-Editor: 1)
bun run build          # tools/build.ts → dist/ (Bun HTML bundler; WGSL imported as text)
bun run typecheck      # node vendor/typescript/bin/tsc --noEmit -p tsconfig.json   ← run before every commit (vendor/typescript is gitignored: in a cloud checkout run a system `tsc --noEmit -p tsconfig.json` — 6.0.2 passes on src/)
bun run tools/shot-server.ts   # :5174 capture sink (loopback only; CORS granted to loopback pages on any port): __shot() → docs/shots/<name>.jpg, __png() → docs/shots/ab/<name>.png
```
There are no unit tests, but there IS a headless QA harness under `tools/qa/` (the whole Game over a Proxy GPU device, ~1 s
of wall per simulated minute): `bun run tools/qa/tour.ts --runs 3 --quiet` (the 13-stop tour with the by-hand checks as
assertions, exit 1 on failure), `bun run tools/qa/soak.ts --seeds 1,2,3 --minutes 12 --quiet` (seeded randomized soak with
per-frame invariants — among them: the player never crosses a static between frames and never ends a frame inside one), `bun run tools/qa/
probes.ts <name>` (single-situation repros; `player` = the player-side PASS/FAIL set incl. the wall-clip brute force, `wall-clip` = that
family in full with its 4-minute door sweep, `attract` = tools/qa/attract.ts, the start card's attract mode without a page). THE EXIT CODES ARE VERDICTS:
probes.ts runs every probe through one `run(name, fn)` (failed = returned false / threw / printed a FAIL or error line; the rest still run) and exits 1 naming
the failed ones, 2 on an unknown name (the known list printed, nothing ran), 0 with an `=== probes.ts <name>: ALL PASS ===` banner (`NO VERDICT` when what ran
checked nothing); soak.ts exits 1 on any exception / invariant violation and 2 on an unknown `--flag` (the valid ones listed); `sh tools/qa/gate.sh fam1 fam2 …`
prints every sub-family verdict, counts both check-line shapes (`- PASS —` and the editor / level probes' two-space `  PASS`), and is red (exit ≠ 0) on any FAIL /
error line, a non-zero probes.ts exit, or a family that printed neither a verdict nor one counted check (`probes.ts selftest` / `selftest-quiet` exercise the red
paths); `bun run tools/qa/replay.ts <session.json[.gz]> | --all [--strict] [--invariants]` replays SESSION JOURNALS recorded with F10 (the *Session journal* bullet: exit 0 =
identical, 1 = diverged with the frame / field / sensor key named, 2 = bad file; `probes.ts journal` = its self-test + the fixtures in `tools/qa/sessions/`). Run tour + soak after every gameplay merge, and `probes.ts player` after touching movement,
doors' contacts or props. Since 08-12 the runner returns a real verdict (`bun run tools/qa/probes.ts <name|family|all>`: unknown name exits 2 with
the list, any failed probe exits 1, a probe that checks nothing is 'NO VERDICT'; `tools/qa/gate.sh <families…>` prints every sub-verdict, counts both check formats and
exits non-zero on a failure / unknown family / silent probe; it finds a time cap it can use — timeout, gtimeout, a perl alarm — so a stock macOS runs it, `GATE_TIMEOUT` overrides) and the soak rejects unknown `--flags` and exits 1 on any violation. `probes.ts all-quick` (`sh tools/qa/gate.sh all-quick`; ~20 min on the maintainer's Mac before the rebuilt families joined it, an hour on a busy four-core cloud box with them: 170 probes, red today only in `expected-red.ts`'s rows) = the union of the gate lists a lane runs before handing back (perf · player · doors · qrf · checkpoints · journal · ui + every identity card, and since 10-09 the rebuilt game's: roomclear · threshold · silent · lineoffire · doorpath · kit · missions · depot · registry · shipped) where `all` is hours; `probes.ts repros` = the informational who-does-what tables (NO VERDICT by design, out of `all`); `probes.ts list` = the name index. The runner snapshots the module knob tables (CIV, WHISTLE, NEEDS, AIM …) round every probe, so a probe cannot tint the next one's identity digest. The deep-review fixes carry their
own staged families: `review-a` (perception / doors / rings / items — docs/internal/review-ledger.md passes 6–8) and `review-b` (checkpoints × civilians, score, UI —
passes 3 / 9); soak `--body-light on|off` A/Bs the bodies-by-light rule. "Verified" still also means driven in the browser (below) with no console/GPU errors and the thing visibly working,
plus a clean typecheck — the harness cannot see rendering, the light meter, smoke sight-lines or audio.

- **The frame cost tool's marks follow the call (2026-11-03): `tools/qa/framecost.ts --marks`.** `framecost.ts` splits `Game.update` by system with
  marks that `framecost-hooks.ts` writes into `game.ts` as it loads. A mark was found by how its line STARTED, in the step and the private methods written
  straight after it; the step's re-write to seats (`this.` to a seat's view, loops over the seats, parts moved below other methods) lost sixteen of them
  with nobody told but the table's last section, and their time fell on the bucket of the last mark found: a run read 46 to 74 ms worst frames as 'smoke
  pushes'. Now a mark names the CALL (`updatePlayer(`, `.orders.tick()`), and the statement that holds it is found whatever stands in front
  (`statementWith`: a loop or a guard and its brace, a `const`, the thing a `.call(` is made on, the guard it is the test of; never an `else` arm, a
  comment, the middle of an expression, or a line the hooks wrote). Behind a brace or a `;` the mark goes IN the line, so the first call of a loop over
  the seats is marked inside the loop for every seat. The step's own table (`MARKS`) is in the order the tick runs and each mark is looked for at or
  below the one before it, so a mark that would land on another call of the same name is reported, not written; the marks inside a method the step calls
  (`INNER`: tracers, the smoke's wake) are looked for in every method of the class the step reaches by `this.<name>(`, wherever it stands
  (`stepRanges`). New marks: the senses' step on the CPU, the checkpoints' tick, the journal's close, the death recap. `--marks` plays nothing: it loads
  the two files under the hooks, prints where each mark went (method, line, in the line or over it; `--gpu` with it = the harness as that mode writes it)
  and is red (exit 1) when one found no statement; `FC_MARKS_BREAK=1` adds a mark for a call the step does not make and must fail. A RUN with a lost mark
  is red too now, and says so above its tables. RUN `--marks` AFTER A CHANGE TO THE STEP'S OWN LINES. With the marks back (51 of 51, the harness's 4;
  `--verify` on five missions: the digests with and without the hooks agree) those worst frames are the CHECKPOINT'S TAKE (`this.saves.tick`): one frame
  a run, 68 ms on the compound at 163 s, 60 on the warehouse at 188 s, 50 on the pass at 10 s, 51 on the rescue at 4 s (seed 1, this box, bun; under
  node the same runs' worst frames are 111, 98, 83 and 88 ms with no collector pause in them, by `--v8`, which has no split). Every other frame of that
  bucket is under 0.03 ms. It is the stall the save's own bullets already name (a whole take of a team on the floor, read as 23 to 26 ms in the
  maintainer's Chrome), not a new one: the first save in memory of a run is a whole take, and `SAVETIMELINE.since` shortens only the ones after it
  (since 11-03 that whole take is the run's first tick's and the page runs the take's code behind the brief: `SAVETIMELINE.base`, the checkpoints' bullet).
  block-squads' worst frame is another thing: 35 ms at 16.4 s, 16 of it path finding and 8 the scenario. The probe's CPU stand-in is spelt two ways
  and has one site of each today (four sites before the senses had rows of their own): a spelling with no site is what is reported.
- **What a tick costs (2026-11-02): `tools/qa/tickcost.ts`.** The simulation's own CPU time, tick by tick, on the stand a game server plays (the shipped stand, the senses
  on the CPU, the game's own maths): the clock is round `Game.update` alone, the box list's pack (`BoxWorld.upload`) is timed apart, the first 120 ticks of a run are left
  out. Five scripted squad runs by default (`compound-four:1, warehouse-four:1, pass-four:1, hostage-rescue:1, block-squads:1`, 38 634 ticks), each in its own process;
  `--rounds n` keeps the quietest round, `--node` plays the same under V8 through `framecost-v8.mjs`, `--json` files the numbers and `--compare a.json b.json` prints the
  before / after table. After every update the raw bits of what moves are chained (`game/digest.ts rawBits`) outside the clock: `--compare` is red (exit 1) when a run's
  bits, tick count or digest differ, so a speed-up is only ever quoted for THE SAME RUN TO THE BIT, and the bits are the same under bun and under node. Red path:
  `TICKCOST_BREAK=1` flips the last bit of one float once, after the warm-up, and the comparison must fail (as it does on a run that is in one file only). Take times with nothing else running on the machine and
  never with `--prof` on (bun's sampling profiler, one `.md` a run, is for reading where the time goes, not how much). Under node two cores are a quarter slower than four
  (the engine compiles and collects on other threads). At the head it was written on: 3.48 ms a tick under bun 1.4, 4.34 under node 22 (mean of all five; p99 about 10
  and 12.5 ms); the slowest ticks of a squad run are not the simulation but the save in memory (`savestate.ts writeMem`, 70 to 150 ms headless) written inside the step.
  THE PACK'S CADENCE is the play-through's by default (every 4th tick, as a page drawing at 15 Hz would); `--each` makes the list after EVERY update, as a networked
  game's tick does (another run: its bits compare with another `--each` run's only), and `--list` makes it by `BoxWorld.relist()` (the same run to the bit as without
  it). The json says which (`pack`), and the comparison's header prints both sides'. `bun run tools/qa/relist-check.ts [--runs a:1,b:1]` is the check that relist
  leaves the game and the device's bytes as the whole pack did (`docs/agents/overview.md`, the list without the pack; `RELIST_BREAK=1` must fail).
  The machine these numbers were first taken on exports `BUN_OPTIONS=--smol` (a small heap, about 2 % slower): the 3.48 above is with it, the net-cadence numbers in
  `overview.md` without (`BUN_OPTIONS= bun run …`). Say which when quoting one.

## Driving the app headlessly (Browser pane / any automation)

`src/main.ts` exposes hooks on `window`: `__game __engine __cam __demo __input __smoke __menu __audio __quality __marks __particles __weather __findme __glass __wet __froxel __rt __gi __mark __save __sharp __civ __sdfrig __wind __qrf __needs __destr __taa __dn __attract __stage __meshTrace __boot __ready __perfrec __journal __replay __prewarm __anim __killGuard __readout`
(+ since the rebuild, the squad game's: `__orders __roomclear __nightbreach __rescue __scenarios __picks __doortools __doors __doorblast __fight __wallcharge __glassbreak __director __pair __wounds __hour __fow __extra`)
(+ the hot-swap hooks `__hotVol __hotDirect __hotRestir __hotFroxel __hotDenoise __hotReflect __hotGather`: recompile a pass from source WITH its real preludes — prefer them to `__wgsl` for the big kernels;
`__hotCommon(src?, variant?)` swaps an edited common.wgsl / `TRACE_VARIANT` into EVERY tracing pipeline at once — the ray-traversal lab's hook, see the *Stratified gather directions + the trace lab* bullet)
(+ URL flags `?sharp` / `?nosharp` / `?noiq` / `?gbaa` / `?nomv` from the image-quality program, `?sdfsmooth` / `?charsoft` / `?charsoftmarch` + `__rigTrace(rays)` from the SDF character rig),
`__runFrames(n, dt=1/60, keys=[], draw=true)` (fixed-step frames with synthetic keys held; `draw` false = only the burst's last step is drawn, the others pack the world as a draw would and go on — what a script on the software adapter wants, and NOT a check of the drawn-every-step path: the GPU's senses then answer once a burst), `__mouse(x, y, buttons?)`,
`__click`, `__pauseLoop` (stop the rAF loop; step only via `__runFrames`), `__resetClock()`, `__shot(name, q, scale)`
(JPEG to the shot server), `__panelShot(name, {frame?, quality?, scale?})` (the tweak PANEL (the debugger, on the backslash key: `player.md`, empty hands are a slot) is HTML, outside the frame `__shot` reads back: `#panel` rasterised through
editor/domshot.ts and laid over the frame + the overlay's canvases; native sliders in the raster), `__png(name)` (lossless readback for pixel A/B), `__timeScale`, `__wgsl(src, {prelude:'scene'|'lighting'})`
(compile-check WGSL against the live device with the same prelude the scene passes get — 'scene' for direct/softblur/
temporal, 'lighting' adds lighting_common for gather/rc/probe/volumetrics — `naga` is not installed here, so Chrome/Tint through the page is the WGSL validator; a Linux sandbox could run naga if ever
needed).
URL flags: `?keeppage=0` (every load a whole page, as before the kept page — `docs/agents/render.md`, the kept page; a DRIVEN page and a `?paused` one are whole by themselves, so a tool reads `window.__game` as ever, and `?keeppage=1` asks such a page for the shell: the game's hooks are then on `document.getElementById('kept-page').contentWindow`), `?keepdrain=0` (on the tab's first address: a load on the kept page does not wait for the last mission's GPU queue before it takes the device), `?callclock=0` (a load's buffer, texture and submit calls are not timed, and its record has no `calls`), `?playclock=0` (after the load, a frame's time inside the GPU queue's calls is not totalled; `?callclock=0` puts both off), `?paused` (deterministic boot, nothing runs until stepped), `?quality=verylow|low|medium|high|ultra` (pins a preset,
auto off), (`?demo` and `?breach` started the night and day showcases; both were removed with `__night` / `__breach` — docs/agents/showcases.md — and the flags now do nothing; the old tour is `__demo.start()` only), `?mute` (or `localStorage twopointsix.testMute=1` once in the automation
profile — **keep test pages silent**, the user listens to their own audio), `?nodecals` / `?noparticles` / `?noweather` / `?noglass` / `?nocivilians` (each merged experiment off from boot — its untouched path), `?nortreflect` (+ `?detail=0‥1`; `?nortreflect&detail=0` = renderer v3 phase 4 off: main's frame bit for bit), `?nomarkings` / `?level=relief` / `?levelFile=<name>.json` (the markings bullet), `?decalStress=N`, `?level=local` (boot on the level EDITOR's current document: the workbench's ▶ *play from here*
stashes it under localStorage `twopointsix.editorLevel` and opens this — an unreadable stash falls back to the bundled office with a console error), `?nogatherstrata` / `?nomotioncalm` (the two 08-12 defaults off), `?hud` (the debug read-out under the HUD on
from boot — otherwise opt-in per machine, Tab ▸ Frame ▸ debug read-out), `?nop5` (renderer v3 phase 5's ultra extras off from boot AND vetoed for the presets — ultra's frame as it was before the
phase; the Tab rows still flip them), `?reachUniform` (A/B only: force the reach masks' uniform binding instead of the storage buffer). The viewer = the workbench's Characters / Effects modes (`src/viewer/` runtime + chrome housed by `src/workbench/main.ts`, VIEWER.md;
`/workbench#mode=chars&scene=guard/carbine-shoulder`; `/viewer#guard/carbine-shoulder` and `/gait` forward with the hash translated) has
`__viewer __char __step __gizmo __rig __rigSource __toss __cans __vtime __shot` (+ `__viewer.scene(name, variant)`, `__tac __hold __long __M`, `__vui` the chrome, `__vrt` the viewer Runtime — NOT `__rt`, which is the game page's RT-reflection knobs).
The level editor = the workbench's Level / Materials modes (`src/editor/app.ts` housed by `src/workbench/main.ts`, docs/level-format.md 'Editor'; `/workbench#mode=level&file=<name>` (`&autosave` / `&blank`);
`/editor#file=…` forwards; `__wb` {mode, setMode, scene(id|null, variant), staged} switches modes / stands a staging figure in the level) has `__ed` (every edit as a
function over the same undo stack: `__ed.box.add/move/resize/…`, `light`, `door` (+ `makeFrame` / `cutOpening`), `route.draw`, `volume`, `mark`, `select`,
`undo/redo`, `validate()`, `lint()`, `serialize()`, `load(text, {lenient?})`, `camera()/pin()`, `step(n)`, `shot(name)`; every multi-op verb inside `doc.transact` = all-or-nothing), `__edshot(name)` (view + overlay), `__tools __view __scene`;
its rAF loop also stops on a hidden pane — dispatch mouse events then `__ed.step(n)`. QA: `probes.ts editor` + `probes.ts level-lint`; `bun run tools/qa/level-lint.ts levels/office.json`.
`__stage(pin | null, { park, guards:[{i,pos,yaw,torch,aimPitch}], player:{pos,yaw,crouch,holstered,invisible} })` pins the camera VERBATIM
(`FollowCamera.setPin` → `pin`: look-at, yaw, pitch, dist — no damping, no aim lead; the numbers must be finite, dist is clamped ≥ near + 0.05 and |pitch| ≤ 89°, the applied pin comes back
in the result; null releases it AND puts the free camera's yaw / pitch / zoom back) and stages actors in one call — use it for every capture: the follow
camera drifts under scripts and framed walls instead of the subject for a whole day of renderer notes. Move the player with `__game.teleportPlayer([x,0,z])`, never by assigning `player.char.pos` (his collision is a sweep from the last settled spot, so a raw
write becomes a walk that stops at the first wall). **The boot is asynchronous** (every pipeline through `createComputePipelineAsync`, `Engine.init()` awaited
behind a start card that counts them in): `window.__ready` is a Promise that exists from the first tick and resolves once every hook above is assigned, the
card is live AND every kernel is in (rejects if the boot dies) — `await window.__ready` is the one line a script needs. Since 2026-10-27 the start card of a
PERSON'S browser does not wait for the heaviest kernels (the row Start on first frame shaders starts on there: `docs/agents/render.md`). A browser a script
drives (`navigator.webdriver`: every page tool) starts with that row OFF, so there the card still waits for everything and `__ready`, the hooks and the first
picture are as they were. `window.__started` is true from the moment the card is live (on a mission's page a person sits at the brief's block is live later, at `__boot.marks.settled`: `docs/agents/menus.md`, the brief's Start waits for the picture; `?settle=0` off); with the row on (`?firstframe=1` on a tool's address) the later set
may still be compiling then and its passes drawing stand-ins, the hooks are already assigned, and `__ready` resolves only once the set is in (`gpu/device.ts
laterIdle`; `__boot.marks.laterReady` is that moment): a tool that asks for the row and compares pictures waits on `__ready`, never on a hook. `window.__boot` = the boot timeline (`ui/boot.ts`: marks, `pipelines {done,total,failed}`, long tasks, frame gaps,
first-frame GPU) and one `boot … ms to interactive · …` console line prints at GPU-ready. Typical script: navigate with `?paused&quality=medium`, `await __ready`, wait for `#view.clientWidth > 0`
(a hidden pane gives a 0×0 canvas, rAF stops and timers throttle), `__menu.close()`, then step with `__runFrames`.
**`__runFrames` draws every frame it steps (unless its fourth argument is false) and does not wait for the GPU** (only the last of a burst is put on the canvas). On a GPU slower than the script the
frames pile up behind it. Measured 10-17 on the software adapter, 1280×720, Very low, `?roomclear`: a drawn frame is about 30 s of GPU (the first one out from
under the menu 3½ min) and a burst of 60 is queued in under a second, so three bursts leave an hour and a half of drawing queued, and a later call does not come
back until the browser takes more (a page check's fourth `__runFrames(60)` not back inside four minutes, no error anywhere: it reads as a hang in the game and is
not one; that it is the browser holding the call is read off the numbers, not seen). So on that adapter a tool that only needs the GAME to have run steps with the
draw stubbed — `const e = __engine, real = e.render; e.render = () => { e.world.upload(); }; try { __runFrames(n); } finally { e.render = real; }`, the stub packing
the world as replays.md's 'an undrawn step must pack the world' says, thirty-odd page tools do — and draws the frames it wants to look at one at a time, each waited
for (`__runFrames(1); await __engine.device.queue.onSubmittedWorkDone()`). Same page, same day: eight stubbed bursts of 60 back in 0.2–0.5 s each, the game's clock
eight seconds on, then one drawn frame through the GPU in 32 s. What lives in the draw stands still while it is stubbed: the smoke solver, the GPU particles, the
light meter's answers (and its asks queue up from the first stubbed ticks — draw a frame or two before reading it; read off render/probe.ts `put`, not measured).
The start card has an idle **attract mode** (`src/game/attract.ts`, `__attract`): 6 s without input and the world starts running behind the card with the
camera cutting round the level — never under `?paused`, `__testVisible`, or once any of `__runFrames/__mouse/__click` has been called, but a plain page left
on the start card WILL start it (any key / click / mouse move puts the still view and a fresh encounter back; `__attract.start()/.stop()` by hand).
Synthetic input goes through `input.keys/pressed/clicked`; during the tour real input is locked (`Input.lockExcept`)
but synthetic writes still work.

<!-- was CLAUDE.md lines 2730–2764 -->
- **Checkpoints + autosave** (STATUS G5, `src/game/savestate.ts` — the schema, what is NOT saved and why, the determinism statement and the autosave rules are the
  file's head comment; keep it current): `saveState(g, {label, kind, force}) → SaveState v1` (plain JSON: Sam's kit / place, every ROSTER man's state / fix / clocks /
  errand / body script / pair / weapon / witness + talk, bodies as SETTLED ragdolls at their particles (`Ragdoll.settleAt`), items on the floor (in-flight / on a fuse
  → written spent), doors, furniture, fixtures + blackout, devices, the mission and the CHRONICLE (so the debrief survives a load; a 'checkpoint' row on save and on
  load, loads cost SCORE.load), escalation + rollcall, needs / whistle / tactics / dialogue ledgers (small `*Snapshot/*Restore` pairs beside their state), the CIVILIANS
  (`civilians.ts civiliansSnapshot/Restore`: place, gait, mark, fright, target / teller by callsign, corner + promise, dice, a downed one's particles, the floor's queue of
  words due as data — restored AFTER the men and after `eventSeq`, ears re-stamped even for a file without the slice), the storm with its dice, wet coats, the decal ring;
  NOT smoke, sparks, tracers, particles, search plans / pies / standoffs / flank roles (re-dealt inside a frame), scripts, a whistle event in the air (resolved once per
  object — dropped on load)), `loadState(g, s)` = `restartEncounter()` (the clean slate: fresh Player / Guards / civilians, units despawned, level as authored, nothing in
  the air; `resetGuards` also drops the tactics' memory) then every slice applied and the derived state rebuilt (poses, torch lights, leaf boxes, `updateTargets(g, 0)`);
  **`Game.time` comes back as saved** (every stamp is against it; nothing outside the Game reads it). `canSave(g, force)` / `canLoad(g, s)` return the reason: refused
  under the tour / attract / breach / any puppet, daylight, a unit on the floor or `qrf.state !== 'idle'` (no checkpoint inside the response), Sam down, the run OVER
  (stage failed / done — canLoad refuses such a file too, never rewritten to exfil); a scripted man / the clearing pair / Sam mid-action refuse a plain save and FILE under `force`
  (let go of in the file — the running game is never touched: saving is read-only but for the chronicle row, and a `kind: 'harness'` snapshot writes not even that);
  a plain save also waits for a BODY STILL FALLING (it is filed settled); a file of another level (`levelFingerprint`: door + route names, box / target / prop counts, every
  static's centre + size to the cm) or version is refused. A load takes daylight out first, pre-marks the restored hearing events as seen by the glass / the QRF, and
  forgets the floor-wide bark clocks (`forgetGuardClocks` / `forgetSuppressionClocks`) — Game.time has gone BACK, and any new module clock keyed by the Game must
  tolerate that (compare like `wet.ts`, or reset in loadState). DETERMINISM: with the harness's seeded Math.random re-seeded after the save and after the load, the
  next 10 s digest identically (continue ≡ load into the same game ≡ load into a fresh game — `probes.ts checkpoint-identity`); in the browser only the dice differ.
  A MAN CAUGHT MID-TURN (10-24; `SAVETIMELINE.turn`, savestate.ts: ON in the page, `?saveturn=0`; OFF in the module and on the frozen stand,
  `TPS_SAVETURN=1`; a `FROZEN_HOLDS` row; the journal carries the table, and a recording from before the key replays with it off — its go-backs started a
  turning man from rest; its own key, whatever `SAVETIMELINE.on` says): with the turn weight on (`character.ts TURN`, the page and the shipped stand) a
  guard's turn was not in the file, so after a load he started from rest facing where his weight had let him get to and his AI eased from there, not from
  its wish: continue and load parted on his first frame (`checkpoint-identity` on the SHIPPED stand was red from the build step that moved its staging so
  the save lands while a man turns; `TPS_TURN=0` passed). The turn is filed now (`GuardSnap.turn`, nine numbers: where he has got to, the speeds his weight
  carries, where his AI wants him, where he stood, the neck's lead; `character.ts turnSnap` / `turnPut`, which reads the nine as untrusted — finite or
  nothing, the speeds held to a turn's top rates and the lead to the neck's reach — and marks him weighed for the loader's bake: game.ts `dealWeapons`
  writes `turn.on` only on his next frame; with the weight off at the load, `?turn=0`, nothing is put back and he is the file's yaws). The loader bakes a living man with `bakeLoaded`: posed where his weight had got him, his yaws then handed back
  AS FILED, so a wish written after his own update on the frame of the save (two men turning to talk: `dialogue.ts updateAmbient`) is still what his next
  frame eases from. No key with the weight or the switch off (those files are byte for byte what they were), on a body, or on a scripted man; a file from
  before the key loads as it did. CHECK, both stands: `probes.ts checkpoint-turn` (in `checkpoints`; it raises `TPS_TURN=1 TPS_SAVETURN=1` itself), three
  lines: the patrols walk until a man has over 1 rad/s in his feet and over 0.3 rad to come round, saved there, every man's place and yaws the same frame
  for frame over the first 30 frames of A, B and C and one digest over the 3 s after, and a file made after a load holds the same turns and yaws; the same
  again with his body yaw written 0.4 rad on after his update (the loader with a plain `update(0)` fails this line); with the switch off no file says a
  turn, the rest of the guards' slice is the same, and a file that has one loads frame for frame as the same file with the key taken out (the load's
  own reading of the switch). `CHECKPOINT_TURN_BREAK=1` (the key taken out of the file before the load) fails the first two on both
  stands, from frame 1. `savetimeline-check.ts` no longer puts TURN off for its file comparison (it did on the grounds that no file held a turn). One
  seed, the office; in play the gap it closes was small (a turn restarted). The page, headless on the software adapter (the office, a man handed a turn of 1.6 rad and saved three frames in, 1.9 rad/s in his feet): the turn straight after the load is the saved one and four frames on load and continue agree to the bit, 0 console errors. Never seen on a GPU, and nobody has watched a load land on a turning man.
  `Game.saves: Checkpoints` (ticked LAST in update): AUTOSAVE requests on the stage advancing, first entry into a room region (chronicle.ts `roomOf`), a hauled body
  let go of ≥ `CKPT.bodyHideM`, every `CKPT.quietSecs` 90 of quiet play; written once the gates pass (missionLive, AI on, nobody alert / searching / awareness >
  `awareMax` 0.4, no civilian startled / frightened / running / telling / hiding from him untold and no word of theirs pending — `civilianCompromise`, `playerBusy`
  null, nothing on a fuse, plus canSave) after `debounceSecs` 2, never within `minGapSecs` 10 of the last, dropped after `pendingSecs`
  25; an autosave followed inside `regretSecs` 8 by a man going alert / an alarm episode is REGRETTED (`SlotInfo.regret`: kept, but `newest()` passes over it —
  a checkpoint a stride ahead of being seen is a death loop); three rotating localStorage slots `twopointsix.save.auto.N` + `…manual` + `…index` (`store` is a `SaveStore`: `localStore` / `memoryStore()` — headless.ts
  hands the Map one), `enabled` persisted `twopointsix.autosave`; `write(g, slot, label, force)`, `load(g, key|state)`, `slots()/newest(preferRun)/read/clear`,
  `importText`, `ageWords` ('40 s ago' off the mission clock for this run's, wall-clock otherwise), `flash` (overlay.ts: the CHECKPOINT tag top-right, `#checkpoint`),
  `summary/log/stats`, `onSaved` (QA). UI: the DOWN card's default is 'Restart from checkpoint (N s ago)' (menu.ts `checkpointButton`, `MenuDeps.loadCheckpoint` =
  main.ts: show stopped, camera cut), the pause card offers it beside Resume + Settings ▸ Game ▸ Autosave, a bare Enter once down loads this run's newest (player.ts;
  Shift+Enter restarts as ever), Tab ▸ Checkpoints (save now / force / why not, load newest / chosen, slot list, export .json / import / clear; THE EXPORT MAKES NO DOWNLOAD since 10-27, see below), `__save` {save,
  load, slots, newest, export, import, clear, auto, summary, canSave, snapshot}. Sizes: ~26 KB fresh, ~30–90 KB in play, ≈ 215 KB only in a synthetic worst case
  (512 marks + 520 rows + every prop moved). QA: `W.snapshot(label, force)` / `W.restore(s)` in headless.ts (start a probe from a staged file), `probes.ts
  checkpoints` (identity — civilians in the digest — / autosave / death / slots / refuse / civilians), `probes.ts review-b` (the deep review's game-systems fixes: run-over
  refusal, tactics reset, the civilians' gate term, laser cut by a civilian, whistle not re-answered, score counters past the timeline cap, rain bed in the annex, glass
  reset), `soak.ts --checkpoint-at MIN` (+ the always-on autosave invariants and the `checkpoints` smell line).
- **An exported checkpoint is not downloaded** (2026-10-27; the maintainer: "we should get rid of local downloads entirely", and for a file that cannot go his
  pick "No downloads, editor keeps files"). Tab ▸ Checkpoints ▸ 'export chosen' (`ui/bindings.ts`) saves the slot's text through the one door
  (`src/host/savefile.ts`, kind 'checkpoint', off `SAVE_UP.local` since this step): the browser is handed nothing, the file goes to the build's backend where
  there is one and this browser sends (a local build's own deployment under `bun run dev`), and THE ROW'S LINE SAYS WHAT BECAME OF IT in the door's shared
  words (`savedWords('The checkpoint', …)`): "is being sent to the developer", "will be sent" (held in memory until the sharing notice has been read),
  "wasn't sent" (no backend, sharing off, `?cloud=0`); a second line follows a file that was taken (`savedEnd`): "was sent", or "couldn't be sent." The
  file's name and size go to the console. IMPORT STAYS ('import…': a way in, not a download; an old export, or a file written back out by
  `bun run tools/cloud/pull.ts [--local] --out DIR`), and so does `__save.export()` (it hands the console the text). The F10 journal went the same way
  in the step before (`ui/sessionrec.ts`; replays.md). SWITCH: the door's own, a page-only table with no row on the frozen stand and no knob:
  `?downloads=1` (or `__saveUp.on = false` in the console) = the file downloaded and the line the old "exported <name> (N KB)"; no game state reads it,
  so both stands play the same. CHECKS: `bun run tools/qa/savefile-check.ts` (the kinds that still download; the row read in source;
  `SAVEFILE_BREAK=1` exit 1), `probes.ts panel-inventory` (two rows reworded in the fixture), and on a real page `tools/qa/savedoor-page.cjs`
  (Chromium, software adapter, a stand-in backend): with a backend no download, both lines, and the stand-in holds
  `twopointsix-checkpoint-infiltrate-0s.json` (32,831 bytes); under `?downloads=1&cloud=0` one download and the old line; under `?cloud=0` no download and
  "The checkpoint wasn't sent." One run of each, 10-27. NOT CHECKED: the 'held' line on a page (the door's answers are driven headless only); a real
  backend; `SAVEDOOR_BREAK=1` with the export in it; a file past the backend's bound (a checkpoint is 30 to 90 KB). WHAT IT COSTS, by reading: with
  sharing off or no backend an export is kept nowhere unless the address carries `?downloads=1`. Never seen on a GPU.
- **Saves keep a timeline** (`SAVETIMELINE`, `src/game/savestate.ts`, 10-19; the maintainer, from a play test: one quick save is awkward, "especially in the tutorial";
  "a rolling buffer"; "maybe we have a screenshot"; "let those have a timeline"). ON in the page (`?savetimeline=0` puts it off), OFF in the module and on the frozen
  stand; the word `TPS_SAVETIMELINE=1`; a `FROZEN_HOLDS` row; in the knob registry and the journal (a recording from before the key replays with it off). OFF = the build
  as it was: three autosaves and the one manual slot, no F5, no strip, no pictures. ON:
  · **Two rings that never push each other out**: the newest FIVE autosaves (`auto.0‥4`, was three) and the newest FIVE made by hand (`manual`, `manual.1‥4`: the first empty
    slot, then the oldest). An old build's slots read and list as they are (`manual` IS hand slot 0), and with the switch back off every slot written under it still loads.
  · **F5 saves by hand.** Read in `updatePlayer` (`input.hit('F5')` → `Checkpoints.askSave`, written at the end of that update), so a run tape replays it like any press; the
    page keeps a bare F5 from the browser while a run is up (`Input.saveKey`, the page's to say: not on the start page, behind a mission's brief, on a replay,
    in the kit picture's window, under a show's key lock (`Input.lockExcept`: no F5 reaches the game there) or over a broken frame). ON THE PAUSE CARD the world is
    held and no update reads a key, so the page keeps the press (main.ts `savePend`, the loop's hotkeys; only on the card itself, not with the Options or the
    mission list over it) and hands it over as a press of F5 on every live frame until a tick has moved the clock — the save stays the recording's own input —
    while the mission and `Game.time` are the ones of the press (a restart or a save loaded with the card up drops it). The card's strip shows a dashed tile
    where the save will stand (`Menu.saveAsked` lays the strip alone, the keys' place on the card kept; `MenuDeps.saveDue`; `.tile.due`), and none when the
    save would be refused (`Checkpoints.handNo` asks, writes nothing): the refusal is then said on the feed at that tick as ever. The F5 line of `input.ts`
    stands BEFORE the typing-target return, so an Options slider that kept the focus does not let a bare F5 reload the page under a run. On the end cards F5
    stays kept and does nothing: a reload there would throw away the saves the DOWN card is showing, and there is nothing to save. Known: with five saves by
    hand the tile to come is a sixth until the oldest is replaced. What a file cannot
    hold is refused with its reason on the feed (`not saved: …`; `canSave`: his hands full, a can on its fuse, a body still falling …). A man alerted does NOT refuse
    a save by hand and a save by hand is never marked regretted: it is the player's word, and the strip is how he goes further back than it. The Tab ▸ Checkpoints
    buttons go on as before (`bindings.ts`, straight to `Checkpoints.write`; its slot picker still reaches four).
  · **A team on the floor saves in memory.** A file holds no unit (`canSave` refuses), so Training five to seven and every squad mission never had a checkpoint. With the
    switch on a save made with a unit up is a rollback snapshot (`rollback.ts takeSnapshot`, kept behind a closure on the `Checkpoints` so no snapshot walks into another):
    its own two rings of five, `SlotInfo.mem`, key `twopointsix.save.mem.<slot>.<i>`, nothing in the store. It lasts THIS RUN ON THIS PAGE: a restart, another mission or a
    reload of the tab drops it. Its gate is `canSnap` (no show or puppet, the Director not blocking, Sam up, the run not over, the response not out) and the soft autosave
    gates as ever; the REGRET rule under the switch counts any man against him going alert (a mission's defenders are units too: `!x.unit` never saw them), and the
    rooms an autosave is asked at and named for are the level's own on every level (`roomHere`: the office's table named a lesson's street 'cubicles'). Loading one
    restores the Game in place (`loadMem`). NOT taken back, as a file's load leaves them: the run's save and load counts, the controller's log and stats, the autosave
    switch, the three hooks, the sandbox row, the tune rows; any OTHER setting kept on the Game goes back with the save. The page's run books start over (main.ts
    `vRun = null`: the Mission is the same object, and a load is another try). Then it calls
    `Checkpoints.onPutBack` — `src/host/putback.ts`: the books of "what was last sent to the GPU" roll back with the Game while the GPU's buffers do not, so the host
    forgets them (`BoxWorld.forgetSent`, `LightSet.forgetSent`, `CharacterRenderer.forgetSent`, the marks, and `Game.rehand()` for the snow, the wind's map and the dust grid,
    smoke and particles cleared, the TAA history cut, the world cache dropped) — A HAND-KEPT LIST: a new "sent only when changed" book inside the Game's reach needs
    its line there, or is a `HostOwned` no snapshot touches (the GPU light meter's buffers and count book are: `render/probe.ts` `Plumbing`, `docs/agents/replays.md`; its line here only drops the answers still out, `IrradianceProbe.putBack`). A check of 'a save put back drops X' runs the real hook headless through `tools/qa/putback-host.ts hostPutBack(g)` (it stands in the GPU calls the harness's engine lacks), never a hand on the host's field. (`putback.ts` imports game modules as TYPES only, and `player.ts` imports nothing of `savestate.ts` — `askSave` reads the switch: a value
    import in either was one more door into the game's import cycle and broke a tool whose first import was `digest.ts`.) The page adds the camera cut and drops held keys. Enter once down goes through `loadInStep` (a memory save is put back at the END of that update,
    never under the modules still running; one that will not load is refused in the step and the restart happens, as with a file).
  · **A picture per save**, taken by the page, never by the game: `Checkpoints.picture(info)` is a seam the page fills (main.ts: after the next PRESENTED frame, that
    frame 256 px wide as a JPEG, `setPicture` — a few frames later on the page: see below). A file save's picture is one more store key (`twopointsix.save.pic.<slot>`, stamped with its save's time so a stale one is
    never shown); a memory save's stays in memory. Every store write is in a `try` (`put`): a full store drops the pictures and writes the save again, a store that throws
    leaves the game running with the save refused. No picture (the harness, a hidden tab) = a plain tile.
    THE PICTURE DOES NOT MAKE THE PAGE WAIT (10-25; `SAVEPIC.on`, `src/host/savepic.ts`, the page's alone: ON in the page, `?savepic=0`; OFF in the module and on the
    frozen stand, `TPS_SAVEPIC=1`, a `FROZEN_HOLDS` row and a stand word kept only so the stands' books are whole — no picture is taken headless, nothing in
    the simulation reads it and a run is the same run either way. Its table is on `knobs.ts VISIT_OWN` and NOT in the journal's: the canvas is configured for it
    once at boot, so no recording may set it, and its address word is on `ui/replaykey.ts VISIT_FLAGS`: a page that plays a tape keeps the viewer's
    `?savepic=0` and drops the tape's — `cardreplay-check` reads every word on that list). WHY: on the maintainer's machine a save with a squad up was a 64 ms frame, twice in a
    minute of play, of which 38 to 46 ms lay outside the update and the render (a performance take of 10-25, read by the performance work: an inference
    from the timings, not yet a `save` row in a take). The picture was `drawImage(the WebGPU canvas)` into a 2D canvas straight after the frame was drawn,
    which makes the browser read the WHOLE canvas back and wait for the frame it was just handed. NOW (`render/savepic.ts SavePicture`): one small pass
    samples the canvas's texture down to the picture's size (4 × 4 taps a pixel), the small target is copied to a buffer and mapped, and the JPEG is made
    from those pixels when they arrive, a frame or a few later; `setPicture` already drops a picture whose save has left its slot. For that the canvas is
    configured so a pass may sample it (`gpu/device.ts initGpu({ canvasRead })`: asked only with the switch on, the save timeline on and not in the kit picture's window — otherwise the
    canvas is configured exactly as it was; a `?bench` page is the page as it opens, so `?bench&savepic=0` against `?bench` is how a machine says what the wider
    canvas costs it). The pass is made inside an error scope: one the device refuses is counted lost and leaves no black picture — the page says so once in the console,
    takes that save's picture from the next presented frame the old way, and stays on the old road until it is loaded again (`__save.pic().late` reads false).
    The pass's pipeline is built by the device's own call, off the boot's count of pipelines, and the JPEG is made on a small canvas kept on the CPU. Until the pass is built, or with the switch off, the old road. `window.__save.pic()` reads which road is up
    and how many were taken / lost / are in flight. CHECK: `PORT=5199 node tools/qa/savepic-page.cjs` (headless Chromium, the software adapter): on, the
    picture is not there when the frame's own task ends, is there once the GPU hands it back, is the size asked and the old road's picture of the same
    frame, a second save with the camera turned has its own frame's picture and the first still reads the first's, a pause card laid before a picture arrives
    shows it on the same tile, and a late picture made to come to nothing (`__save.picLose`) drops the page to the old road with that save's picture the next frame's; off, the old road; `SAVEPIC_BREAK=1` fails. ON THE KEPT PAGE (10-28; `docs/agents/render.md`, the kept page): `SAVEPIC_KEPT=1` in front of the same check opens `?keeppage=1` and drives the game's FRAME under the shell, in place of the two pages above — the tab's page is a shell with no game, the frame a kept one on lease 1 and every `on` line holds in it; then the frame is loaded again (it must come up on lease 2 with NO program compiled and no shader module made, the small pass's own handed back by the shell) and a save's picture there is still the late road's and the old road's picture of the same frame; `SAVEPIC_BREAK=1` fails under the word too. Software adapter only: a save's picture on a kept page has never been seen on a GPU, and a save taken while a load of the frame is on its way was not tried. NOT KNOWN: what either road costs on a GPU — the late road has never run on one, and whether a
    canvas a pass may sample presents any slower there is not measured (it should not: the usage is a permission, not a copy). A card showing the saves when a picture arrives is handed it for its tile (`menu.ts pictureCame`: the tile's picture alone, nothing laid again). A page
    closed or loaded again inside the few frames between a file save and its picture keeps the save with a plain tile (the old road wrote both in one frame).
    A SQUAD SAVE COPIES ONLY WHAT CHANGED (10-25; `SAVETIMELINE.since`: ON in the page, `?savesince=0`; OFF in the module and on the frozen stand,
    `TPS_SAVESINCE=1`, a `FROZEN_HOLDS` row; carried by the journal, a recording from before the key replays with it off). WHY: with the picture off the
    hot frame, what a save with a team up still costs is the whole-game snapshot inside the update — 23 to 26 ms on the maintainer's machine (a take of
    10-25 with the recorder's `save` event beside each of three autosaves, 92 s apart: a frame of about 40 ms where the usual one is 16). NOW `writeMem`
    takes the snapshot SINCE the last one taken or put back (`rollback.ts takeSnapshot(g, since)`: what that one held is gone through and only what changed
    is copied, the rest shared; the game is not walked). The first save of a run is a whole take (the run's first tick's own under `SAVETIMELINE.base`, the next bullet), and `rollback.ts` walks the whole game again when the
    chain has grown stale (`WALK_AGAIN`: a save taken since its base stands on the base's own chain, a whole walk begins a new one). The base is the shelf's `last`, and the shelf is now a `HostOwned`
    class (`savestate.ts MemShelf`; it was closures): a snapshot's walk reads every key the Game reaches, and the first cut of this, a shelf that handed
    the walk a snapshot, put the chain inside the next save and a load rolled the chain itself back — the check threw on exactly that. The base is
    DROPPED (the next save a whole take) at a new run (whose first tick takes its own under `SAVETIMELINE.base`) and at a file's load (`adopt` — a recording begins on one and so does its replay, so the first
    save in memory of both is whole). WHAT IS NOT THE SAME: a save taken since another also holds,
    and a load writes back, what its base held that the game has since let go of — objects nothing in the game reaches (a closure or the host still holding one would see it
    rewound: none found, not checked). One snapshot holds at most a quarter more than a whole one and 2,048 objects of these before the game is walked
    whole again (`WALK_AGAIN`); the chain itself — every object any save of it met — lives as long as any save of it is on a ring or is the base, so a
    run's saves hold more dead objects alive than whole takes did (bounded by count, 8 snapshots' worth and 65,536; NOT MEASURED over a long mission). What the game reaches is put back the same either way, and the run does not change. `rollback.ts`: a take that throws half way leaves its
    chain marked (`Chain.busy`) and no snapshot of that chain is taken since again — before this every save had a chain of its own. MEASURED headless on
    the cloud box, the orders yard's pair, saves 90 s apart: 11 to 13 ms since the last against 23 to 54 ms whole (three saves); the FIRST save taken
    since another in a process was slower than a whole take (67 ms once, the path cold), so the second save of a visit may be the slowest. NOT KNOWN: the cost in Chrome
    on the maintainer's machine, where a whole take is the 23 to 26 ms above — a take with the recorder's `save` event says, and `?savesince=0` is the
    other arm. CHECK: `probes.ts savetimeline`
    line (5b), both stands (`SAVESINCE_BREAK=1`, one man nudged after each put-back, fails on both): seven saves into the ring of five with a team up, each of the five kept loads and plays
    on as its save did with the snapshots it was taken since gone, a save made after a put-back loads and so do an older and a newer one, the run and
    the seconds after every load are, by digest, those of the same script with whole takes, and the moment before each save, taken since the shelf's own base and put back beside a
    whole take of that moment, gives the same state word (`stateHash`: to the bit, of everything the game reaches; seven of them, one after a put-back —
    the save's own snapshot cannot stand beside a whole take, the controller writes its books on both sides of it inside the one call); line (5)
    runs with the stand's own setting, and the whole rollback check's stand-in host puts the shelf's base back with its entries. A take made to throw half way is followed by a whole one on a new chain that is the game as it stands. NOT CHECKED: a save
    after a file's load, and one across a whole walk the chain forced (the quiet yard reaches neither).
    A RUN'S SAVES STAND ON A BASE TAKEN IN ITS FIRST TICK, AND THE PAGE RUNS THE TAKE'S CODE BEHIND THE BRIEF (11-03; `SAVETIMELINE.base`: ON in the
    page, `?savebase=0`; OFF in the module and on the frozen stand, `TPS_SAVEBASE=1` beside `TPS_SAVETIMELINE=1 TPS_SAVESINCE=1`, a `FROZEN_HOLDS` row;
    carried by the journal, a recording from before the key replays with it off). The maintainer dislikes "the autosave stutter"; to the idea of the
    performance read (the worst frame of a squad run is its FIRST autosave: the framecost bullet above) he said "sure, snapshot behind start card sgtm".
    WHY THAT FRAME: it held the run's one whole take, AND the take's code had never run. `rollback.ts` has three roads (the whole walk; going through
    what a snapshot held and finding it unchanged; copying what changed) and each costs two to four times its later self the first time it runs. NOW,
    two things under the one switch. (1) `Checkpoints.prime` (private; the tick that begins a run, where the shelf is cleared): the whole take, kept as
    the shelf's `last`, so the first save of play is a take since. Under one gate (`basedNow`: the timeline and `since` on, AUTOSAVE ON — a machine
    with it off, or the kit picture's window, pays for no take, and a save by hand there is the whole take as it was —, a unit on the floor,
    `canSnap`): a run whose program is running in its first tick has none (tut-squad, tut-kit; tut-sky, tut-team and tut-queue do get one, their
    program not yet running then), nor has a run whose team comes up later. (2) `Checkpoints.warm` (the HOST's: main.ts, after the boot's own marks,
    while the mission's brief is up and not started): a whole take and one since it, kept nowhere (`shelf.warmed` counts it), so the first tick's
    take is not the page's first. Same gate. Only where a brief is up: a page a script drives, a replay and every headless tool warm nothing, and
    their first tick's take is a cold one. A walk that throws costs the base or the warm-up, never the frame or the boot (a warning on the console).
    WHAT IS NOT WARMED: the third road cannot run before the game has changed, so the first save of play is still about a third dearer than a later
    one. NOT THE END OF WHOLE TAKES IN PLAY: the chain's stale limit is a quarter over the BASE's count, and the first tick is the smallest a run
    ever is. A run that grows past it walks whole again at the save after: the compound's play-through (seed 1) holds 29,256 objects at its base and
    43,383 at its own first autosave, 163 s in, so its SECOND save is a whole walk — a warm one, where before the first was the page's first whole
    walk and the second its first take since. With no base the chain begun at that autosave is good to 52,594. The limit measures growth over the
    first snapshot, not what the chain holds that the game has dropped: the mend is in `rollback.ts` (a size that follows the game, or the walk taken
    off a frame of play), not built. THE FIRST CUT TOOK THE BASE AT THE STAGING (Game.restartEncounter's last
    act) and was wrong twice, both found by review: every tutorial threw on its first tick (the walk met the beat context's showcase-only `view`
    getter with the program live: the authored gate lets a program by because an authored save STOPS it first) — `TPS_PROFILE=shipped bun run
    tools/qa/rails-check.ts` and `lesson-run.ts tut-road` are the checks that see it, the save's own did not; and the chain's stale limit
    (`WALK_AGAIN` × the base's count) counted the game before its first tick (24,058 objects on the compound, 27,875 a tick later), so a fight forced
    a whole walk in play two saves sooner than no base did. From the first tick: five saves, one every 30 s, over a fight that kills ten men in
    150 s with him stood still are all takes since (32,452 to 34,653 objects held). MEASURED 11-03, this box (4 cores), one seed, fresh process each, a scratch script (three squad missions,
    two runs each, saves asked at 10 / 40 / 70 s; not in the repo): the first save of play 30 to 35 ms where it was 74 to 91 under bun, 38 to 45
    where it was 143 to 148 under node's V8; a later save 21 to 26 (bun), 30 to 36 (V8), as before. WHAT IT COSTS: 115 to 124 ms (bun) / 146 to 158
    (V8) behind the brief, and the run's first tick 42 to 57 / 66 to 76 ms longer — the frame the brief closes on, which was already the slowest of
    any run (150 to 166 ms here, everything in it running for the first time). ON THE PAGE (headless Chromium, the software adapter, nothing drawn,
    the compound with four, two runs each): the first save of play 49 and 57 ms with the switch on against 139 and 132 off, the run's first update
    336 and 319 ms against 252 and 242. A restart's first tick pays a warm whole take. SAME GAME: four missions' play-throughs (compound, warehouse, pass, rescue; seed 1; shipped
    stand) digest the same on the build before, with the switch on and with it off (`PT_DIGEST=1`). CHECK: `probes.ts savetimeline` line (5c), both
    stands (it raises the switches itself; `SAVEBASE_BREAK=1`, the base dropped after the first tick, fails on both): the first tick leaves a base,
    the first save is a take since it and to the bit a whole take of its moment, it loads and plays on, a restart gives the next run its own, the
    run and its loads are those of a run with no base, `warm` made before the first tick keeps and changes nothing, and with autosave off no base is
    taken; (5b) reads the first save's kind from what its run's first tick left on the shelf. The tutorials are NOT in it: run
    `TPS_PROFILE=shipped bun run tools/qa/rails-check.ts` after a change to when a take is made.
    `PORT=5199 node tools/qa/savebase-page.cjs` (two boots, about five minutes each; needs no `?mute`: that flag makes the page a script's and the
    brief goes): behind the brief with the first tick not run the code has been warmed once and nothing kept, Enter, the first tick leaves the base,
    the first save a take since (one whole walk later is let by); `?savebase=0` = nothing warmed, no base, the first save WHOLE; it prints what each
    save cost and does not judge it;
    `SAVEBASE_BREAK=1` fails. NOT KNOWN: anything on a GPU or in the maintainer's Chrome (never seen there; `?savebase=0` is the other arm, and a
    take with the recorder's `save` event beside an autosave says); more than one seed; whether a warm save (20 to 35 ms here) still shows as a
    dropped frame on his 120 Hz display — that is the take itself, which this did not touch.
  · **The strip** (`menu.ts saveStrip`, `#menu .saves` in index.html): on the pause card and the DOWN card, this run's saves in time order — picture, age, a small diamond
    on one made by hand, a regretted autosave dimmed, one that is ahead of the mission clock marked. One stop for the keys (Left / Right or A / D walk it, Enter loads),
    or a click. Fewer than two saves = no strip. `Restart from checkpoint`, Enter once down and `newest()` are as they were: the newest sound one.
  Check: `probes.ts savetimeline` (families `checkpoints`, `all`; `SAVETIMELINE_BREAK=1` fails; green on BOTH stands): the rings roll at five and five and neither kind
  pushes out the other; every file entry loads and the next 10 s digest as the run that simply went on (the camera put back and MOVES off for the comparison: the
  camera is the host's and the animator's momentum is in no file, as the head of savestate.ts says; a turn's weight is in one since 10-24, `SAVETIMELINE.turn`); an old build's slots list and load, on and back off; a store that
  refuses, throws or is full; Training five with its pair: memory saves, each loads and plays on to the same digest with the animator as the stand has it, the store
  untouched, F5, Enter once down, a restart drops them. `checkpoint-slots` / `checkpoint-autosave` read the ring's size from the stand. ONE SAVE WEIGHS (headless, one
  seed): the plain night's file 28 KB frozen / 31 KB shipped; Training one's 9 / 11 KB; Training five in memory 5.0 / 6.9 MB (14–19 thousand objects, about 30 ms to
  take here: a hitch of a frame or three at each such save on a real machine is likely and is NOT timed on the page), so ten of those are 50–70 MB of the tab's
  heap; a picture is 256×144 and about 4 KB as stored (the software adapter's frame at 1024×576), so ten files with pictures are near 350 KB of the store. `tools/qa/savetimeline-page.cjs` drives
  the real page on the software adapter (Chromium, SwiftShader, `verylow`; four jobs, each a boot of several minutes): the plain night — F5 as a real keydown, kept
  from the browser, three times → three tiles in time order with pictures that are not flat, the middle tile loads to its own mission clock, the DOWN card shows
  them; Training five — four saves in memory on the strip with pictures, then a bulb put out AFTER a save and that save loaded from its tile: the bulb glows again
  on the frame after the load (as first measured, with nobody in the room: 259 nearly white pixels at the save, 0 with it out, 258 after the load), and with `SAVEPUTBACK_BREAK=1` (the host not told) it stays
  dark there (0) though the game has it lit, so that line fails as it must; `?savetimeline=0` — F5 writes nothing and is the browser's, no strip; `card` (10-20, the page's own loop let go with the pause card up) — F5 is kept, nothing is written and the world does not move while the card is up, the strip shows the tile to come, the card closed writes ONE save by hand with its picture, the button the keys had walked to keeps the focus, a save that would be refused shows no tile, a restart made with the card up drops the press, and under a key lock F5 is the browser's; `SAVECARD_BREAK=1` (the kept press not handed over) writes none and fails as it must. No page error. THE YARD JOB STANDS HIM UNDER THE BULB AND SWITCHES IT OFF since 10-26. From the street nobody on the team has a look at that bulb, and since the fog's kept faces (`FOG.kept`, on in the page since 10-24) a light put out unseen rightly goes on showing lit in the picture: the job read 259 → 259 → 258 and said "the check cannot tell" on every page from then on (it had not been run on one between 10-23 and 10-25; headless on the shipped stand the bulb put out from the street has emissive 0 and a kept face of its full glow, and with him stood in the kitchen the team has a look at it and no face is kept). So the job, on the page as it opens with no rule off, stands him under the bulb before the save (`__stage`'s `player`), reads `fogOfWar.lampSeen` and fails by name, drawing nothing, if the team has no look. It switches the bulb off (`on = false`) where it used to break it, because a light the team SEES break arcs (`fixtureBreak.ts`) and its sparks, which no undrawn step moves on, were 213 to 221 nearly white pixels where the bulb had been. And it counts the nearly white pixels within a twelfth of the frame's height of the bulb's own place on the frame (`FollowCamera.project`), because the two men now in the room, lit by the bulb's light (which every frame uploads, told or not), were half the frame's count. Measured on the software adapter: 299 around the bulb at the save, 0 with it off, 272 after the load (570, 0 and 501 on the whole frame; the bar is the old one, within 0.35 of the drop); with `SAVEPUTBACK_BREAK=1` 299, 0 and 0 (593, 0 and 0 on the whole frame: nothing in the room was lit again in those five draws), and the job fails as it must, exit 1. NOT CHECKED on any adapter since: a light the team sees BREAK, whole again after a load with its sparks gone; and a face the fog keeps ANOTHER way than the world has it, across a save in memory that is put back (the fog's own `probes.ts fog-filed` reads a file's load, headless).
  NEVER SEEN ON A GPU, never played. Known and open: smoke hanging at a memory save is gone after its load (the fluid lives on the GPU) and a sound that was playing
  plays out; a load from a card is between steps and cuts the run tape, as a card's load always has; an F10 recording ENDS at a load or a restart made between frames and a new one is asked for (10-22, `SessionRecorder.cut`, called by `Checkpoints.load` / `loadMem` and `Game.restartEncounter`; `docs/agents/replays.md`, the journal bullet; check `recordcut`), while Enter once down is inside an update and on the tape, and the recording runs on over it: it replays for a file written inside the recording, for a file from before it (the file carries the store's rows and that slot's text) and for a save in memory made under the tape (the sensors' stream is the host's); a save in memory from before the recording ends the recording with that frame, and a save or an import made between frames (the panel) ends it before the save (`docs/agents/replays.md`, the journal bullet; check `recordload`); the Tab list shows a memory
  save as 0 KB.
- **A save carries what his body is holding** (`SAVETIMELINE.holds`, 10-20; `?saveholds=0`, `TPS_SAVEHOLDS=1`; its own key, whatever `SAVETIMELINE.on` says): Sam's
  wall hold with the walk-in take's clock, and the close-in mode's held gait with the man it is held on. The rule, the cause (`savetimeline` (2) red on the
  page's stand once walls were taken by walking) and the checks: `docs/agents/stealth.md`, the wall taken without the key.
- **A stand-up sets every held rule again from its word** (10-13, the switches owner's line, after a check's pin was found undone). `src/host/standup.ts standGame` writes each
  held table on EVERY stand-up — `if (pinned('TPS_X')) X.on = <the word TPS_X is '1'>` — so on the frozen stand a check that sets `X.on = true` and THEN stands a game up gets it
  put back off, silently. A check's own pin goes AFTER each stand-up (each one: a second stand-up in the same process undoes it again), or the run uses the word (`TPS_X=1`),
  which every stand-up then honours. By design: one stand-up = one known stand. On the shipped stand a table is written only where its word is given (`pinned(env)`), so nothing
  is claimed here about a pin made before a stand-up there. Read in `standup.ts` on the rows around the opening, the kit and the dust; not every row was read, nothing run.
- **Which rule turns a line red: `bun run tools/qa/stand-bisect.ts [--line REGEX] [--green] -- <command …>`** (2026-11-04). A check that is green on the frozen
  stand and red on the page's is red because of a row the frozen stand holds off, usually one or two. The tool raises subsets of `FROZEN_HOLDS`' words on
  the frozen stand until the fewest that flip the line are left (delta debugging: about a dozen runs of the check for one word, some eighty for three) and
  prints them with the rows each raises. `--line` makes the verdict one line's (a check with several red lines has a rule for each); `--green` asks the other
  way round, of a line that is red on the plain frozen stand: which rule of the page's mends it. READ THE ANSWER THIS WAY: the row's own check says what the
  rule does, and the line is then a promise the rule ended (mend the line to read the rule, as `selector-sustained` and `devices` (d) now ask "stays" where
  doors stay as they are left) or a fault the rule brought (its owner's). Found with it on 11-04: `kickrubble-pool` (`CUTBOXES=1`: a blast breaks the piece),
  `orders-cover` and `orders-stack` (`TPS_WORDS=1 TPS_DOORNAMES=1`: the HUD's name of a door), `teams-dark` (`TPS_IRLIGHT=1` flips both its lines: a teammate's
  tube sees a dark man), `devices` (e) (`TPS_HANDS=1`), `controls-behind` (d) and (f) (`TPS_KITANIM=1`), `selector-rows` (`TPS_CLOSE=1`, `TPS_HANDSLOT=1`,
  `TPS_LIGHTSWITCH=1`, a line each), `workstow-mate` (`TPS_SQUADDEAL=1 TPS_TEAMPACE=1 TPS_PACING=1`, all three, and
  there the answer was the CHECK'S fault: it asked "was his gun hung on the frame before the round left", and with the squad's kit and its pace the gun comes
  back up and fires inside one tick. The game's own count of rounds out of a hung gun, `WORKSTOW_STATS.mateFired`, stayed at the one the check forces; the
  line reads that count now). So a named rule is where to look, not the verdict: trap the thing the line is about before calling it a fault. A line no set of words flips is not a held row's doing: the tool says so, and what the shipped stand itself makes of it.
- **Retiring a switch: `bun run tools/retire/retire.ts ROW=value … | --rows rows.json [--write]`** (2026-11-04; the maintainer: "can you look for switches we
  haven't flipped in at least 3 days and clean all of those up?"). A settled rule's switch is a second way through the code that nobody takes. The tool takes
  rows with the value each has IN THE PAGE and leaves one way: every read becomes the value and the code is folded round it; the page's address word, the
  stand's hold, the journal's line for an old file, the debugger's toggle and a tool's own raise or put-back go; the row goes from its table, from
  `FROZEN_HOLDS`, its stand word from `STAND_ENV`, its address word from the lists of address words, its name from a panel's list of rows and from what a
  machine keeps; and what only the dropped branch used goes with it, to a fixed point. It reads the tree with TypeScript's own language service
  (`vendor/typescript`, or `TS_LIB=<path to typescript.js>` in a cloud checkout) and is a dry run without `--write`. IT REFUSES, with the place: a row the game
  writes as it runs (state, not a switch), a row Options writes (the player's), a getter and setter, a row an object literal or a map of tables' rows holds
  at another value, a row the game reads through a list of keys (or writes through a list something else reads too), a row read as its module loads where
  its own value is not the page's (the page has written nothing yet: at a file's top level, or in anything a file's top level calls, makes, reads through a getter or
  hands to a function of the game's own, followed to the end by the declaration each name leads to; what is handed to a listener, a timer or a promise
  waits, and the file a page's script tag names is the program that boots the game after the switches, not walked), a row that is the subject of a `switch`, the same key read on another thing of the table's type (a parameter, a kept copy that
  is not taken at load: a copy taken at load, `X_DEFAULTS`, is read as the row's OWN value), a row read through a list of keys handed to a call with the table, a row the game writes through a list of keys or
  through a pattern, a held number whose old value something read as its module loaded, a table typed by an index, a value that is not of the row's
  kind, and the rows of the Rust module's twins. AND A VALUE THAT IS NOT THE PAGE'S, where the page itself says what that is: `--page page.json` is every
  table as a stand-up of the page left it (`TPS_PROFILE=shipped bun run tools/qa/stand-dump.ts --out page.json`, on the tree about to be folded), and a row
  the dump gives another value, or does not hold, is refused; so is a row whose write in the page rides a row that stays (`A = B && !q.has('noa')`: with
  B's word on the address A was off too). `--trust ROW,ROW` takes the caller's word for those. `--late ROW,ROW` takes it for a row the walk of load-time calls reached (the refusal names every such read and the chain of calls to each from the top-level statement: the walk follows every call a function could make, not the ones it does make), never for a read at a file's own top level; each read it was taken for is printed. THE JOURNAL'S LINE for a recording from before a row, `if (the recording has no word for ROW) ROW = off`, goes whole with the row, and only where everything under that `if` is the write of a row retiring in the same run: once the row is gone no recording has its word, so anything else under it would run for every recording made from then on, and the row is refused (as it is where the word is read into a name first). A row read on an untyped thing under the table's name anywhere else in the game is refused too: that may be a recording's words or the table itself. (Reading the page's writes off the code was tried first and
  dropped: their order, an early return and a helper are the code's to run.) Exit 3 = some rows were refused and the rest done. A CALL IS NEVER FOLDED AWAY FOR ITS NAME: `x.get(k) && ROW` loses the call only where `get` is the language's own
  (a Map's), the function's body only reads, or the caller vouches for it (`--pure name,name`; the command line is then the record of what was vouched for).
  A game object's `get`, a getter (also under a type that calls it plain data, or reached by a key), a method some other class overrides, a function a `let`
  holds and a default worked out at the call all stay. A method of the language's own that is not a reading one (`sort`, a Map's `delete`) is never asked of the game's same-named bodies. An import whose last use as a value went stays as a bare import, where it was: its module still loads
  (asked twice: of what the run dropped as it rendered the file, and once more of the finished text by its words alone). A number of a knob table whose last reader went stays where its name is still a quoted word somewhere (a panel's list of rows). IT SAYS, for a hand: a constant left in a
  condition beside a call it cannot see to the bottom of, a value nothing reads any more whose making still happens (a die drawn, a call made: the name
  goes, the call stays), a loop left with nothing in it, and every place a CHECK asked for the retired path. `--drop file.ts:name` takes a declaration out
  with the statements that call it; `--drop-at file.ts:line[:words | :column]` one statement of a check, and only a statement that is nothing but the call
  (a verdict kept in a name, returned, passed on by a helper, or a call that prints a line a case is said as NOT FOUND; `lines.ts diff` does not place a
  line whose place printed other lines of the run too). THE PROOF OF A RUN is not the tool's: both
  typechecks; the page's stand run for run the same before and after (`TPS_PROFILE=shipped PT_DIGEST=1 bun run tools/qa/playthrough.ts --seeds 1-2 --json f`:
  42 digests); the frozen stand after = the frozen stand before with the retired rows' words raised; and the family's checks run before and after on the
  page's stand, line by line, where a line that went from PASS to FAIL is an arm of the retired path and is taken out by the place it was printed from:
  `bun run tools/retire/lines.ts run <dir> [--tree <the tree before>] <the check files the fold's report names>` before and after, `lines.ts diff <before>
  <after> --drop places.json`, `retire.ts --drop-lines places.json --write`, and `run` once more. (`diff` also says a check that DIED after the fold, one
  that throws exits 1 as a red check does, and does not count a line that was red before under other words.) After a fold that took a row with a slider
  out: `bun run tools/gen-scene-notes.ts` (the scene panel's notes are keyed by row). WHAT GOES THAT WAY, by kind: the stand's and the journal's
  lines, the off arm, the on arm measured against the off arm ("a quarter of those metres"), and the lines of a check that held a newer rule off for its
  length; in the first fifteen checks of the squad's family that was 65 of 147 lines. A check left saying nothing goes whole (`--drop`). THE TOOL'S OWN CHECK:
  `bun run tools/retire/retire-check.ts` runs it on a small tree of its own (`tools/retire/fixture`) and holds that each of its cases there (110 today) answers the same,
  draws the same dice and makes the same calls after the fold as before it, that the folded tree compiles, and that what names a retired row is gone while
  what only looks like it is not; `RETIRE_BREAK=1` folds one row at the wrong value and must fail. A shape the tool folds wrongly goes into the fixture as a
  case first. Memory: a hundred rows at once is about a minute and 4 GB; all of a family's rows go in one run, not all families in one.
- **A switch that only moves WHEN something lands is still a switch** (11-05; the planners' pace, after the multiplayer goal's read). `PLAN_PACE`
  (`src/render/planPace.ts`, the page's `?planpace=old`) was first read by `main.ts` alone and kept out of the registry as a picture's pace. That is a
  special case with a cost: a word `main.ts` reads by itself is in no recipe and no recording, and a server's stand (`tools/mp/stand.ts`:
  `shippedBoot` and `pageSwitches`) never reads it, so a host's page and its server can stand differently. So it is registered like any other: read in
  `pageSwitches`, a `FROZEN_HOLDS` row (`TPS_PLANPACE=1 (a picture only)`), `STAND_ENV`, the knob registry, `JOURNAL_KNOBS`, and `applyKnobs` puts it
  down for a file with no table for it; `openings-check` holds all of that on both stands. THE RULE FOR THE NEXT ONE: any word that moves when
  something runs or lands, in the tick or in the picture, goes through `pageSwitches` and the registry; `main.ts` reads no such word by itself.
  `REACH_PACE` (`src/render/reach.ts`, `?reachpace=old`) was the same case and is registered the same way, in a commit of its own: the game's tick
  reads it (to pace the lamps' reach masks, which are the picture's: no sense, rule or run digest reads them, and the 42 play-through digests are the
  same with it forced either way on both stands), so a host's page and its server paced that bookkeeping differently. It is the same on both stands
  with no word (the shares), so it has no `FROZEN_HOLDS` row. A PAGE RIG RUNS THE PAGE, where the planners' pace is on and its allowance is the
  clock's: after the first six drawn frames a plan lands when the clock lets it. A rig that reads a frame a fixed number of frames after a door or a
  wall changes puts `planpace=old` on its address (`tools/qa/draw-equal.cjs` does). Neither word is a visit's flag: a joiner's page takes the
  recipe's. `docs/agents/render.md` has both rules.
- **Not every `FROZEN_HOLDS` word is a `TPS_` word** (10-13, read off `src/host/profile.ts`; the knob registry's owner's note). Raised by a word of their own: the door rules
  (`ONEWAY=1`, `QA_FRAMEHOLDS=1`, `QA_DOORMANNERS=1`, `QA_DOORBLAST=1`, `QA_DOORSTAY=1`), `SLOTS=on`, `CUTBOXES=1`, `GLASSFX=on` (three rows), `SQUADS=radiocheck-call` (three
  rows). No word under probes.ts at all: `DESTRUCTION.on`, `RUBBLE.on`, `EVICT.on` (`PT_DESTR` / `PT_KICK` / `PT_POP`, playthrough.ts alone), `FRAG.defenders` (`--set`),
  `tune.chaos` (the tool's own), `AIM.player.lightSnapPx`, `SILENT.on` and `ROOMCLEAR.fresh` (each set by its own probe). So "the frozen stand with X raised" is the row's own
  word in `FROZEN_HOLDS`, never a guessed `TPS_X=1` (nothing reads a guessed word: the rule stays off and no line says so).
- **`laststand-callout`'s last-man line is two lines, each staged for one rule (10-13, tools/qa/laststand-probes.ts; the probe only, nothing in `src/`).** The line kept the
  office post's first man and wanted `named` = the room he stood in. On the FROZEN stand that man is out hunting when he says he is the last (his squad is in contact the frame
  a mate of it drops: the squads' radio check is held off there), and laststand.ts names NO room for a man out hunting (`tellOnly` / `tickCheck`: `free ? null : w`, 'a man out
  hunting is not a room to go to'). It only held because he said it in the corridor; since the compound's second hand-placement pass (a620967d: rooms listed house first, the
  office post's mark) he says it walking through the bunk room → `truly in bunk then, named ''`, red, and not a rule's. Now: (a) THE GUARD HUT'S man, a squad of one, so no mate of
  his drops and he holds his post on both stands: he must say the hut, be holding, and the net must name the hut; (b) the office's man as before, read by what he is doing the
  frame he says it (`g.fight.defs` mode): he says where he truly is (room, or the level's place word), and the net names his room only if he HOLDS one, nothing if he is out
  hunting (frozen: out hunting in the bunk room, named nothing; shipped: holding the office, named office). `LASTSTAND_BREAK=1` = call out left off for those two runs: both lines
  fail on both stands (run), pass without it. A re-placed compound that sends the hut's man walking turns (a) red on purpose: restage it on a man who holds, do not loosen it.
  Not done: the mode is read the frame he speaks, the rule reads it when the check closes (the same in every run here). The shipped stand's 'the answers reach the radio strip'
  is an older red, untouched. Headless only.
- **`scenario-level-probes`' commander hunts a defender who has left his room (10-13, tools/qa/scenario-level-probes.ts `Clear.hunt` / `left` / `run`; the probe only,
  nothing in `src/`).** A room is not called clear while a man dealt to it lives (roomclear.ts tick), and a flanker can be left holding a door on floor nobody holds
  (enemysquads.ts: 'holds where he is', then at home where he ended up). The commander walked his empty room three times and waited out the cap: 'finished: every seed ran to
  the scenario's own end inside the cap' red on barracks (shipped, the four's seed 1: the day room's man in the wash room), compound (frozen, the pair's seed 1: the shed's man
  in the pantry) and compound (shipped, the four's seed 1). Now: from the second pass a room with no live defender in it is not taken again, its men are walked to (a move
  order onto each man's spot), and twice more after the passes; the first pass and any room with a man in it are played as before. IT READS THE DEFENDER'S TRUE POSITION,
  which a player does not have: the line is a liveness check, not a claim that a player finds him. `HUNT_BREAK=1` skips it: the barracks' and the compound's seeds above stall
  again and the line fails (run), passes without it. Run on both stands over compound, barracks, mousehole, warehouse, glasshouse, depot, office: the three stalls are taken and no
  other seed line moved (51 seed lines a stand, against the same run with the hunt only after the passes). Still red with or without it, older: 'it can be won' for the pair on mousehole and warehouse (shipped), the office's rescue data
  (roamer 2 in furniture). Tried and dropped: a hunt after EVERY pass (the warehouse's frozen pair seed 1 turned from taken to wiped). Headless only.
- **The one-button hold probes hold the close-in mode off** (`tools/qa/grab-probes.ts oneButtonRows`, 2026-10-15): `grab-gate`, `grab-release` and
  `controls-behind` were written before the close-in mode (`CLOSE.on`) and read the rows of the build without it, so on the shipped stand they were ten red
  lines that said nothing (the knife / knockout pair where they wanted the one takedown, the take from in front, the alert man still in reach). They now lay
  `CLOSE.on = false` after each stand-up and put it back when done: on the shipped stand they are the check of the `?noclose` page; the mode's own rows are
  family `close`. Measured on the build head 275316ac, headless: family `grab` shipped 13 FAIL lines → 3, frozen 55 PASS 0 FAIL before and after, `close`
  shipped 0 FAIL. The three left are NOT the close-in mode's (red with every close-in switch off): `controls-behind` (d) the card counts no shot on a
  pistol click's frame, (f) a can's click is no throw on its frame; `guard-key` (b) the closer never latches the server door again (reads as doors stay
  where left, inferred). No single `TPS_<NAME>=0` turns any of the three green (every word in `STAND_ENV` tried but the snow, music, picture and words ones).
- **THE BENCHMARK (`src/ui/bench.ts`, `src/ui/benchrun.ts`, `src/host/benchScenes.ts`, `src/host/handsoff.ts`, 2026-10-17; title ▸ Extras ▸ Benchmark, or `?bench`
  on the game's address; `__bench`).** For a machine nobody on the project can sit at, and for tuning the quality levels: one press, hands off, about seven
  minutes. ITS SCENES ARE REAL FLOORS (the set `floors-1`; the day showcase it once played was removed with both showcases): the pass in night snow, the block
  with the mains thrown by a teammate (red emergency lights, the men's torches), the compound in a storm; four teammates, hard, each a mission of the list on
  its own deal and die. A LEVEL IS ONLY LOADED AT BOOT, so a run is A PAGE PER SCENE: `benchBegin` writes a small record to the tab's session store and loads
  the first scene's address (the mission's own query with `roomclear=<seed>&seed=<seed>&difficulty=hard&noverdict&nokeep&benchrun=<i>`); each page finds the
  record (`benchLegOfPage`), plays its part (`BenchLeg`) and hands its lines on with the next page's load; the last page shows the card, and Close (or an Esc
  with nothing to show) loads the address the run was asked on. Of the visitor's flags only `mute`, `nomusic`, `quality`, `pipequeue`, `shadertrim`, `shaderfold`, `savepic` (2026-11-02: until then
  `?bench&savepic=0` against `?bench`, the pair the save picture's bullet names, was one run twice), `pace` (11-03: a benchmark draws every display frame whatever the word says, as it
  always did, so the word only rides along rather than be named as left behind: `docs/agents/render.md`), `adapt` (11-04: `?bench&quality=high&adapt=0`
  is a run at ONE SIZE: `?adapt=0` opens the visit with nothing adapting, `ui/quality.ts visit`. With the ladder on, a heavier picture is paid for in
  resolution, so two runs show the same frame rate and the same GPU milliseconds at different sizes; with the word nothing moves, and the lines of two runs
  are the same pixels. With no tier named the word holds the tier the page stands on (the one this browser's auto last had, High where it never ran), at
  that tier's start size and unmeasured, since no calibration runs: the row 'Hold frame rate' alone left auto free to shed sharp direct, give the tier up
  and resume the last visit's size. NAME THE TIER where two machines or two days are to be compared. THE CARD SAYS WHAT ITS OWN LINES SHOW (`ui/bench.ts
  sizeNote`), not what the switches were: 'held still' where the page's lines (as it opened, each scene, the held picture) were all drawn at one size, and
  after it 'the address named no tier' where it did not; 'more than one size' where they were not (a window resized, the resolution moved by hand); and
  where the row was put off BY HAND under auto (Options, not the address) it asks for a pinned tier. With the row on it says nothing. Like every word of the
  address it is how the visit OPENS: a tier or Auto picked in Options afterwards is the player's, and Reset puts the defaults back under the address's
  words. With the ladder off the page has no frame rate to hold, so outside a benchmark the even pace has no target and the page draws every display frame
  (a pinned tier with the row off always did; a benchmark draws every frame anyway). The kit view's frame is opened without the word (it is pinned to a
  tier and its pacer counts on its ladder), the viewer reads the address by the same reader, and `ladder-replay.ts` replays a take with the row as the take
  had it. Asked for by the look's join to read 'within a tenth of the GPU time with the look off'; whether a machine HOLDS 60 is still read on a run
  without the word. `bench-check` has the word on both addresses, the page's wiring (one reader of the two words, before a frame is drawn; Reset and the
  viewer by the same reader) and the card's lines (a whole run each, the unnamed tier among them); `ladder-replay.ts --selftest` plays a 36 fps fight with
  the word under hold 60 and as now, through a pinned High and through auto with sharp direct its own to shed and a size left behind, and then the page's
  own order (a ladder made from what a browser remembers, the address read, a hand's pick, Reset): nothing moves and nothing is written as the machine's
  choice; `ADAPT_BREAK=1` leaves the word off and the seven lines that play it fail. Run on a cloud NVIDIA card (two benchmark runs, every
  High line of the three scenes at one size, the notes as said); never on a Mac or a Windows card), the backend's own words and the
  shader rows' words (`ui/shaderrows.ts SHADER_ROW_WORDS`, e.g. `compactshaders`) ride along: anything else would make it another fight than the set's, and
  the report has a note naming the words that were not kept. THE ROWS WERE DROPPED UNTIL 2026-10-27, so `?bench&compactshaders=1` measured the plain build
  on every scene (found by the maintainer, who saw no difference on a second run); a row set in Options always applied. Since then the report has a
  `shaders` line naming the rows the page was built with as Options names them, or saying none is on (`BenchResult.rows` keeps their URL words;
  `BenchDeps.rows` = `shaderRowsOn()`: a row can be on from Options with nothing on the address). THE BENCHMARK WAITS FOR SHADERS STILL BUILDING BEHIND THE START
  (2026-10-27, with the first-frame start on in a person's browser): while `Engine.standsIn` the wait before the run starts over
  (`Bench.standing`, as for a tab that was away; every page of a set comes through that wait, and main.ts holds auto quality's measure back with it), `standInMs` (180 s) in all on a page, counted from the first look that found one and never started
  again; past it the lines are timed as they are and the report says so once, however many pages the set has ("some shaders were still building 180 s in …"). bench-check has both cases on its fake page. Not covered: a wall cut before the cut set is in still draws
  its whole box under a timed line, as it always did (that set is not in `standsIn`). NOT DONE either: the engine replay's address (`ui/replaykey.ts
  VISIT_FLAGS`) drops the same words. Checked by `bench-check` (the fake page and tab) and once on a page on the software adapter: never run on a GPU. THE LOOK PROTOTYPE (11-02) rides along too: a run asked for on the look's page keeps the look (`?bench&look`): every
  `look=key:value` row and the page's other boot words (`scene/look.ts lookPageWord`: hills, geo, front, lorry, dress, seat, the cut boxes' switch and the
  rest; never the dress's `look=plain|0|off`) ride along, since the page reads them at boot into its shader text and its level. Before, such a run timed
  the plain build on every scene and said nothing. A TRIAL THE LOOK PANEL'S BUTTON KEEPS OFF on the machine goes on the address as its own word
  (`lookTrialsKept`, `hills=0`; the kept cut boxes' switch with them), so the run times the look the page it was asked on shows and the report's address says which. A SCENE'S ADDRESS IS
  NOBODY'S TO TUNE (`main.ts`, on the word `benchrun`, as the load brief and the GPU retry are; so also a scene's address opened with no run under way): the
  look page's kit and its fog put down stand off (the set's own), and the look panel is not built (no panel over the picture, none of its timers, the take
  leaves by the recorder's own way). So off against on is two runs, `?bench` and `?bench&look`. WHAT THE PAIR DOES NOT HOLD ALIKE: the look's own geometry
  (broken wall tops, round stacks, the pass's dressing) is part of the level a look run plays, so its fight is not the plain run's frame for frame and its
  heavy moment may be another; the first scene's page writes that into the report's notes. The report is still headed with the set's name, `floors-1`: the
  note and the address row are what tell a look run apart. The look's amounts are the defaults unless a `look=key:value` row is on the address (a slider
  moved on the panel is kept nowhere). `bench-check` has the three lines and reads the page's gates in `main.ts` by text; the page's half (kit, fog, no panel,
  the note, a kept-off trial off) was read off the first scene's page on the software adapter, where very low to high also each drew frames under the
  look with no GPU error (the four presets the last page holds: `AUTO_LADDER`). ULTRA IS NOT ON THAT LADDER, and on the software adapter the look page
  LOSES THE DEVICE within ten seconds of ultra being applied (the GPU process exits on signal 11 while it draws the first frame; no program is compiled
  then). NARROWED, a boot a row: of ultra's rows it is the gather's reservoir resampling alone (`giResample 'temporal'`, `fgather.wgsl mainGi`), and it
  needs the cut rule compiled into the ray programs from boot, which any geometry trial of the look asks for (`cutConfig.swap = false`). With the look's
  trials off the look page draws ultra clean; hill rays are not it; and THE PLAIN PAGE WITH `?cutprograms=boot` DIES THE SAME WAY, so it is not the
  look's. IT IS THIS BRANCH'S OLD BASE, NOT THE BROWSER (11-02 late): the build's head, unpacked beside this tree and put through the same script on the
  same box and the same browser (Chromium 141), draws ultra on the plain page with `?cutprograms=boot` with no device loss and no GPU error, where this
  tree's plain page loses the device every time; `&round=0` on this tree changes nothing, so the round test in the cut set's walks is not the whole of
  it. Which of the build's steps since the base mends it was not found (the build's change to how round things are traced in the cut set does not apply
  to this tree, which has its own), so nothing was ported: it is expected to go at the join, and that is one run to make then. Never tried on a GPU. `window.__look.parts()` says which trials a look page asked for at boot. NOT CARRIED: the dress's off words beside a look row
  (`?look=plain&look=…`: the scenes are the set's dressed levels). NEVER PLAYED TO ITS CARD, and never on a GPU. A reload in the middle is a plain mission page (the page marks the record before it plays); `?bench` on the address
  goes to the first scene BEFORE the game boots (`main()`'s first lines, a `replace`: that load is the machine's first and the one the report's `load` line
  times, and Back does not ask again), the row is a second load and the report says so. EACH SCENE IS PLAYED BY THE HANDS-OFF PLAYER (`HandsOff`, the page
  twin of `command-tally.ts`'s hand `b`): W A S D along the nav path, the cursor on a door, the quick key B (stack, then B again when the men have arrived),
  the rifle on whoever the fog shows on a clear line after a rolled beat, then a hunt for whoever is left; all of it written onto the host `Input` in the
  loop's `beforeTick`, before the tick samples the command, so the game is played by its own keys and its own AI on both sides. Sam cannot go down while it
  plays (`godMode`, put back after; a scene's page does not read the machine's own sandbox row since 2026-11-02 — `benchSceneWaits`, asked before the mission is staged: with
  `no AI` kept the room clear refused to start and the run ended on 'the mission did not start', and `unlimited ammo` kept was another fight than the set's; mortal he went down inside half a minute in a quarter of the dark runs), the real keyboard is locked out of the scene
  (`input.lockExcept`), and its one direct write is the door verb in the no-teammates-left path (`selectVerb('door','kick')`). A scene plays at most 75 s of
  game time. In order: page one holds the world while auto quality finishes measuring and times that opening picture AS THE PAGE OPENED, then plays its
  scene live with auto quality doing what it does in play; page two its scene; page three its scene until ITS HEAVY MOMENT (a round fired in the last 0.4 s,
  a dozen seconds in; none by 45 s = held where it stands, and the report says so), the world held still and that picture timed as it stood, then under each
  preset of `AUTO_LADDER` from the bottom up (a preset under eight frames a second is the last one tried), then, everything put back, with each heavy feature
  flipped alone (the F8 sweep, one round, `SweepOpts.inside`), then the fight plays out. It ends on a card (the world held under it) with a text report (the
  GPU by name off WebGL's unmasked renderer, the browser, the system, the screen, on battery or not; the first page's BOOT timings; a line per window and per
  scene; `floors`: how each fight ended, so two machines' numbers are known to be of the same fight only where those agree; the ten features that move the
  frame most) and a Copy button, and the LAST page's take goes out as the F9 recorder's json (no download: the one door, `host/savefile.ts`) with one `bench` event carrying all of it (`set`, `scenes[]`
  with each scene's address, end and boot; the earlier scenes' frames are not in the file, their lines are). `tools/qa/perf-report.ts` prints the report
  first. THE RUN ITSELF SENDS NOTHING, except to a listener at the door (`host/savefile.ts`, 2026-10-27: on a build with a backend whose browser sends, the
  result goes as a small file of its own, kind `bench`, and the take as `benchperf`). NO FILE LANDS IN THE DOWNLOADS (the maintainer, 2026-10-27: "we
  should get rid of local downloads entirely"; `?downloads=1` = as it was), so THE CARD'S LINE SAYS WHERE THE RUN WENT (`benchSaid`, `#bench-said`): "Your
  results are being sent to the developer." where the listener is sending the result or the take now (the take holds the result), "Your results will be
  sent to the developer." where it holds them (a browser that would send and has not been shown the notice, a page opened straight onto `?bench`), "Your
  results weren't sent, so press Copy if you want to share them." where nobody took them (no backend, sharing off), and "The full recording wasn't sent."
  after the first two where the take alone was refused (too large). THE LINE FOLLOWS THE FILES TO THEIR END (`BenchView.update`, the door's `done`): "Your
  results were sent to the developer." once what was going has arrived, and the not-sent line where it was let go after all. Unless all of it is already going the card carries the notice's line (`benchNotice`,
  `cloudNote`, ui/cloudnote.ts): two seconds on screen and a held file goes up, before "back" loads another page and loses it. `tools/qa/bench-check.ts`
  holds every line and the calls in the source. CLOSE WAITS for a file that is on its way or held (`benchGoing`): the button reads "Sending…" until
  `cloud.filesIdle(BENCH_SEND_MS)` answers, then leaves, and a second press leaves at once (Close loads another page, which would cut the only copy off).
  The visitor's sending words (`cloud`, `cloudfiles`, `cloudstats`, `cloudtapes`, `cloudprefs`) and `downloads` ride along the scenes' addresses: before,
  a run opened under `?cloud=0` lost the word at the first scene. `tools/qa/savedoor-page.cjs` (a stand-in backend, `npm ci` in `cloud/` first) holds on
  a real page: a take with no download that the backend has, the notes, the card's lines, Close's wait, `?downloads=1`, and a page with nobody
  listening. `tools/qa/bench-page.cjs` expects no download and reads the take off `__lastPerfDump` (run whole on the software adapter 2026-11-04 with the
  benchmark's wait in: 28 lines, all pass). A DEVELOPER'S WAY TO A FILE: `?downloads=1`, or after the fact `__saveUp.on = false` in the console, then
  `__journal.save()` or `__perfrec.deliver(__lastPerfDump)`. Never seen on a GPU. Esc stops a run and puts everything back. HANDS OFF: the page's Esc and hotkeys stand down while a run is on, no attract
  reel starts. `?bench=N` = N seconds a window; `?benchcap=N` = N seconds a scene (the page check's). A tab hidden during a run starts that window over, or
  leaves that frame out of the scene's line. While a window is timed on a held picture the page's quality model does not run. A scene that will not start is
  said in the report and the next page loads all the same. SEEDS: `bun run tools/qa/bench-seeds.ts` plays each scene headless (the shipped stand) over a range
  of seeds and picks, among the runs that take the floor, the one nearest the set's medians in seconds, rounds fired and teammates lost (each over its MAD);
  `--check` holds each kept seed to still taking its floor, `--twice scn:seed` to the same digest twice. KEPT (12 seeds a
  scene, the 75 s cut): snow 10 (the floor taken in 63 s, 21 rounds, one teammate lost; medians 61 s, 25.5 rounds, one lost; 10 of 12 took it inside the cut),
  dark 1 (the mains out 5 s in, two rooms banged and cleared, 6 rounds, nobody lost, 4 of 8 defenders left; medians 8.5 rounds, none lost, 4 left), storm 6
  (22 rounds, one lost, 4 of 11 left; medians 23.5 rounds, one lost, 4 left).
  THE PAGE'S FIGHT IS NOT PROMISED TO BE THE HEADLESS ONE: the page steps the game at a fixed 60 Hz, the camera with the tick, and draws a blend between
  ticks (`FIXEDSTEP.on`, `TWEEN.on`: docs/agents/replays.md; the hands-off player's keys go on before each tick), so a display's rate alone no longer
  makes another fight. But no check has shown a benchmark scene on a page equal to the headless run of its seed (the page's men read the GPU's light and
  smoke, the headless ones their stand-ins), nor two machines equal; the `floors` line is how a report says which fight it saw. The seeds are picked
  headless. Checks: `bun run tools/qa/bench-check.ts` (a fake page and a fake tab:
  the acts on each of three pages, the lines carried, everything back at the end and from every stop, the slow-machine cuts, a tab hidden, the ring full, no
  heavy moment, the addresses, the reload guard, the report's text; `BENCH_BREAK=1` must fail), `tools/qa/perf-sweep-logic.ts` (the sweep inside another
  take), and on a real page `PORT=<port> node tools/qa/bench-page.cjs` (the software adapter, quality pinned: `?bench` goes to the first scene and Esc comes
  back, then the row's run start to finish over three pages with the hands-off player seen giving its orders on each and walking on the first, keys
  pressed into a scene, the game's clock read every frame, the card and its line, no download, Close (opened under `cloud=0`, which rides along the scenes: nobody listens at the door); the page's own frames play each scene under the fixed step,
  a frame there taking seconds and the wall's cut put out of reach, so a scene ends by the player's own cut after `CAP` game seconds and the last page's
  tail is the fake page's to prove; `STEP=script` has the check step the scenes with `__runFrames` instead; 154 minutes at 320x180 on a four-core box on 2026-11-04, so its `DEADLINE` allows 240 by default). NEVER RUN ON A GPU: no number of its report has been read on a real machine, the seven
  minutes are an estimate, and nobody has watched a scene played or looked at the card.
- **THE PRICING RUN (`src/ui/pricerun.ts`, `src/host/runpage.ts`, 2026-11-04; `localhost:5173/?look&pricelevels&downloads=1`, hands off)** walks every level of the mission list
  (`=a,b` for some), a page load a state: the look page (played four seconds of the game's clock, the picture held on one step, timed with the look's
  switchable layers off and on in turn) and then the same mission WITHOUT the look, held on the same step. One file, `price-levels-<date>.json` (version 6),
  a row a level, each with the run's own verdict against the bar (`priceOver`, the one place a GPU number of it is judged; the file's `bar.ruled` says
  which rows of the bar the maintainer has ruled on). A ROW THAT IS NOT ONE PICTURE IS NOT JUDGED (`apart`: the picture moved between its turns, its two
  pages stood at another size, rung or step, or frames were queued ahead of the GPU in a state, `queuedWords`): its numbers stand and no verdict is read off
  them. The GPU's lag is no line of the verdict (it reads 2 to 3 wherever the page waits for the GPU, more on a 120 Hz display). TWO NUMBERS a state: the
  compute passes' busy span (the game's own "GPU ms") and that span with the g-buffer's pass in it (`all`: every stamped pass but the ones that draw to the
  display, whose stamps span the display's wait; the g-buffer's was read against recorded frames on an M4 Max and follows its own work — one chip, so each
  state is still checked, `spansWait`, which wants half a frame of g-buffer before it calls it a wait). EVERY PAGE STANDS ON ITS PRESET'S OWN ROWS with the
  quality ladder off from before its first frame and gives way to no window in focus elsewhere, BY ITS ADDRESS'S OWN WORDS (`quality=<preset>&adapt=0`
  and `giveway=0`, which `priceSearch` writes: no rule of the page knows a priced page; asked with `quality=auto` the ladder and its pace are left to the
  page and the row says so). It is not counted as measuring: it still waits for the GPU as a player's page does, and a frame's time is the time between two frames it DREW (`window.__pace`; the frames a
  turn waits for are counted the same way), so a level the GPU cannot hold sixty on reads as that and queues nothing ahead; a row says its rung, its size
  and the step it was held at. ONE WAY TO PLAY, on every page: the loop is held from before the boot, the run asks for the page's warm-up (`__prewarm`) and
  lets the page draw with the clock held until the page says the warm-up's clear is over (`window.__prewarmed`), and then the mission is played by the
  page's scripted step, a step a display frame, every step drawn, with no more frames with the GPU than the page's own pacer keeps there. So a level's two
  pages have run the same steps, drawn the same frames and stand on the same render clock at any display rate. (Stepped with no warm-up first, a snow
  level had no snow in its air: the warm-up clears the particles' ring some frames after the page's first frame of its own, which then came after the
  play. Played by the page's own loop instead, two pages stood on render clocks a few milliseconds apart and a 120 Hz display ran other frames than a
  60 Hz one.) A PAGE WHOSE PART THROWS, or whose loop has stopped, is one page not priced, with its cause in the file's notes, and the run goes on.
  The file names the build its pages were (`commit`, the page's own word), and the table merges rows only of one build. IT IS A RUN'S PAGE
  (`host/runpage.ts`: `runPage` stands the load brief, the lost-device reload, the sandbox row and the look page's kit down, as on a benchmark's scene;
  `ownPage` keeps its own page under the kept-page rule, since a run walks by loading its next address). ITS FILE GOES OUT BY THE SAVE DOOR (kind `price`):
  to whoever listens, and to the browser as a download only with `downloads=1`, a flag the run carries with the benchmark's other carried words
  (`ui/benchrun.ts carriedWords`); where nobody took it the card holds the file's text under the table for Copy. A word of the asking address that no
  page carries is named in the file's notes. Words: `quality=` (default high), `pricesecs=`, `pricepage=0` (look pages only), `pricelive=N` (below); a mission's link sheds them
  all (`PRICE_ASK_WORDS`). Checked by `bun run tools/qa/pricerun-check.ts` (no page: the verdicts, the wait's rule, frames queued ahead, the report, the
  addresses, on made-up rows; `PRICERUN_BREAK=1` must fail) and by
  `node tools/qa/pricerun-page.cjs [level …]` (the real page on the software adapter, as a person's page runs it: the chain, one rung, one step, the file
  down; no number there is a price, and its frames are seconds long, so it proves nothing of the pacing) and with `--real` on a machine with a GPU (`--look "<words>" --look-only` prices a look page under other words). Run on
  a rented NVIDIA card through Vulkan only (11-04); NEVER RUN ON METAL OR ON A DISPLAY (a headless card has none, so nothing of a 120 Hz display's frames has been seen), and nobody has read a row of it against the bar. The level tools below read its file.
  A LONG WALK TIRES (11-05, the rented card, 29 levels in one browser): from about the 15th level the GPU's answer came later and later (to 57 ms for 10 ms
  of passes) while the pages' boots kept their time, and the last seven rows read the look page CHEAPER than the page without it with every check green;
  the same seven alone read sane. Why it tires is NOT known. Two mends. The driver walks in even parts on one dev server, a fresh browser every five levels
  (`--each N`; ten until the running read doubled a level's time; files under `<out>/part-<k>`, every part's price file copied to `<out>`). And the run sets a row apart whose answer came late against the
  run's own least (`PRICE.lateMs`, `lateRows`, asked once when the file is made: a state's `work` over the frames of its own passes the page keeps with
  the GPU plus the least any state of the run took beyond its passes, by more than 12 ms; the number is set by that one walk, where it sets apart exactly
  the seven; 0 = not asked, as on the software page). Checked on made-up rows with that walk's numbers by `pricerun-check` (`PRICERUN_BREAK=late` must fail
  on those cases alone). THE RULE HAS NEVER RUN IN A PAGE ON A GPU, and a walk in parts has not run on one either: the rule was read against that walk's
  file after the fact. A person's run is one tab and cannot start a browser: its late rows are what this rule is for.
  THE WORLD RUNNING (11-05, `PRICE.runS` 20, `pricelive=N` on the address, `--live N` on the driver; 0 = held pictures only). A held picture steps
  nothing: on a step of no length the smoke solver encodes nothing (`smoke.ts encode`), the wind over the floor and the loose snow are not stepped
  (`engine.ts`, `dt > 0`) and a flake is handed back as it was, so the walk of 29 had no `smoke` stamp in any row and its `air` read 0. After a page's
  held turns the same scripted steps play on to second 20 of the game's clock, every step drawn, and one more turn is timed with the world still
  stepping: a sixtieth of the game's clock a frame, the frames paced to sixty by THE PAGE'S OWN PACER (`host/pace.ts pacer`, a second one of the turn's:
  `turn`'s `drive`), so the world runs at the game's rate on a 60 or a 120 Hz display. It is NOT the page's own loop, which a card freezes (a lesson's
  card on a tutorial nobody plays, `main.ts frozen`): a scripted step is the one way the world runs on every level. The row keeps the running read BESIDE
  the held one (`run`, `page.run`: the turn, the clock where it ended, programs made while it ran) with a verdict of its own (`overRun`, `priceOverRun`:
  the bar's own lines by `barLines`, which the held verdict shares; NOBODY HAS RULED ON A RUNNING READ) and its own reasons not to be judged (`runApart`:
  frames queued, a late answer, a clock that did not run, SOMEONE WENT TO ALERT since the mission began or its stage moved, by the mission's own word; a
  row whose held read is apart is not judged running either). What the page stood on and what the switch compiled are taken BEFORE the world runs and are
  the held read's; a throw in the running part leaves the held read standing. `rowRun` and `judgeRow` write and judge a row and the check asks them. THE
  LATE ANSWER'S RULE READS EACH KIND AGAINST THE LEAST OF ITS OWN KIND (`leastAnswer` held, `leastAnswerRun` running): a running state is paced to sixty,
  so on a fast display it answers sooner than a held state, and one least for both would call every held state late. Where a held state says the
  g-buffer's stamp holds the display's wait, the running read judges nothing by it either. A `pricelive` that is no second is said in the file's notes;
  one not past the held second is no running read; the file says `bar.runS` only where a page took one. Both pages take the same steps to second 20 and
  then step on through their own turn, so the two running reads are not one picture to the step as the held ones are.
  LIGHTS IN FOG ARE IN THE HELD READ; what running adds is the stepping (solver, wind, powder, flakes and drops) and a grown plume's share of the fog
  march. NO FIGHT is in either read: a fight's smoke, grenades and charges are above both. TWENTY IS ONE MEASURE (the rented card, the snow road with
  three fires near the player, stepped as the run steps for 60 s): 1,289 bricks of smoke after 5 s, 1,947 after 10, 2,141 after 15, then 1,850 to 2,310
  to 60 s; the stamped passes 21.7 ms over the first 5 s and 23.6 to 24.4 from 15 s on; the same picture held at 60 s read 22.7 (no solver stamp). One
  level, one card. Of the thirteen levels kept by the 11-05 plan, by the headless stand: snow, wind and open fires on tut-road (three fires), tut-team
  and the pass (one each); rain on the compound, the consulate, the depot, Mousehole and the office; nothing in the air on tut-kit, tut-queue, the vault,
  the bazaar and Lights out. A PERSON'S RUN IS ABOUT TWICE AS LONG WITH IT, in one tab, where a long walk tires: `pricelive=0` is the run as it was.
  Checked on made-up rows by `pricerun-check` (`PRICERUN_BREAK=run` must fail on the running read's cases alone) and end to end by the driver on the
  software page (twelve more steps, then the turn; there the pacer reads no display, so every callback is a step). THE RUNNING READ HAS NEVER RUN ON A
  GPU as the run takes it (the measure above stepped a page by hand), and the late answer's 12 ms was set by held states. `level-table.ts` shows it in
  three columns on the rented card's table (ranked by it where every level has one), lists the Mac's running verdict apart from "over a bar", shows a
  running read only at the newest running file's second (a row keeps its own file's, `ranTo`), keeps a level's newest running read on its line when a
  later run took held pictures only, and prints the run's own reasons where it set one apart. The report prints no share the running verdict did not
  judge (`addedRun`, `waitsOn`), and a wait for the GPU's queue in the play and the run-up has the run's own limit (`capS`).
- **THE LEVEL TOOLS (2026-11-04; files only, no GPU, no page of the game's changed).** `bun run tools/qa/level-audit.ts [level …]` counts every level the mission
  list draws as the look page draws it (boxes, cut planes, round things, lamps burning at its hour, how much of the picture is each material and how much
  has none: a box is made of what it says, `Box.stuff`, read as the renderer reads it), about a second a level; `--check <level>` holds the engine's caps and the PROPOSED bars of `level-count.ts` (`LEVELAUDIT_BREAK=1` must fail),
  `--counts` gives a level's row for its dressing file, `--write-base` the baseline (`fixtures/level-audit-base.json`: the levels before they were
  dressed). `node tools/qa/level-cost-page.cjs <level>` is one boot of the look page on the software adapter: what it compiled, layers off against on, a
  still; a relative read, and the GPU's particles are never stepped in it, so a snow or rain level is read without what is in its air.
  `bun run tools/qa/level-table.ts --dir <folder>` makes one page of every level from those files, a GPU's price file and each dressing row (the folder's `notice.txt` is printed first and marks a level whose price no longer stands; `cloud/before/` holds the run the newest one takes the place of, printed beside it). "Which levels
  the list draws" is one answer (`scene/scenarioList.ts listedLevels`) and the render passes are one list (`gpu/timer.ts RENDER_PASSES`): both tools ask those.
- **A picture fault is debugged from a frame capture** (F6 on the page: every render target over 8 frames and the trace of what the renderer changed;
  `tools/qa/capture-read.ts` reads it; `docs/agents/render.md`, "A CAPTURE IS EVERY TARGET OF A FEW FRAMES").
- **What a line of the benchmark's card prices** (2026-11-04, `ui/bench.ts benchStats`). `frame ms` is the gap between frames: wherever the
  display's pace is held it reads 16.7 whatever the picture cost, so it answers 'is 60 held' and prices nothing. `GPU ms` is the COMPUTE passes'
  union, the number the quality ladder is fed (`ui/perfrec.ts`); every drawn pass is left out of it, because a drawn pass's stamps are not to be
  trusted on Metal. So work that lands in the g-buffer (more boxes, a richer material, a layer of the look) was in no number of a line: the first
  run on a real GPU that priced it (a cloud NVIDIA card, Vulkan) had the look's layers at 3 % by `GPU ms` and 5 to 6 % with the g-buffer counted.
  `g-buf ms` (`BenchStep.gbuf`) is the g-buffer pass's OWN stamp over the line's frames, in its own column and never folded into `GPU ms`; n/a
  where no frame carried one, and on a result saved before it was kept. It is the mean of the middle nine tenths, not a median: a browser rounds a
  stamp to a tenth of a millisecond, and of a pass this small only a mean reads finer than that. HOW FAR TO BELIEVE IT ON METAL: far
  enough, on one chip. Seven takes of an M4 Max (2026-10-14 and 10-25, about 50,000 frames): in an F8 sweep of the look's prototype, 'every layer off' took the pass's stamp from
  1.85 to 1.15 ms and 'sharp direct off' to 0.81, the same in each of three rounds, while 'fog of war off' (no g-buffer work in it, the frame
  2.9 ms shorter) left it at 1.80; on a page held at 30 frames a second, with 21 ms of waiting in every frame, it read 0.33 ms. `post` and
  `fxaa` followed the frame's length in the same takes: THEY span the wait for the display (`docs/agents/render.md`, timing traps), the
  g-buffer does not. Not shown: any other Apple chip, or a build newer than those takes. In that sweep the look gave back 0.69 ms in the
  g-buffer and 0.85 in the compute union, so `GPU ms` alone saw about half of it. HOLDING TWO RUNS AGAINST EACH OTHER: add the two columns, as an ESTIMATE (two separate statistics, and any time a driver ran the g-buffer beside a
  compute pass is counted twice), and only between lines with the same quality rows: under sharp direct the pass draws at the OUTPUT size and its
  downsample is not stamped, so it does not follow a line's 'drawn at'. Not changed: the card's 'heaviest GPU passes' still lists every stamped
  pass, drawn ones too (on Metal `post` and `fxaa` can lead it and mean nothing; on other machines it is the only place `composite` shows), and
  the sweep's 'GPU ms given back' and the ladder's own number are the compute passes alone, so a feature that moves g-buffer work is under-priced
  there. `bench-check` has the mean on a rounded clock, a hitch in a long and in a short window, the missing stamp (none, or nothing but the 0
  the timer gives a pair never written), the column and its line on a whole run's card, a browser with no timings, an old result, and the pass's
  name against the engine's. Run on a cloud NVIDIA card (the column on every line of four cards, 0.39 to 0.79 ms on the High lines, rising
  with the size over the presets); never on a Mac or a Windows card with this build. A BENCHMARK WAITS FOR THE GPU as a player's page does
  (`docs/agents/render.md`, the page waits for the GPU; a result's `wait`): a card from before that, made on a machine with no display, has
  frame rates and 'one feature changed' rows that are not to be believed on any line over the display's frame.
- **Stalls: the recorder watches the page's own thread** (`ui/perfrec.ts`, from 10-25). A frame's marks see only inside the frame, and a stall is nearly
  always between two frames. While a take runs ON THE PAGE (never headless: a headless take is the file it was; `perfrec.watch = false` leaves it off on
  a page too) a 4 ms timer answers, and each frame's `beat` is the longest it went silent in the GAP before the frame: from the end of the step before to
  the start of this one, so neither frame's own work is in it. What the browser itself reports as a long task or a long frame becomes a `long` event at the
  moment it began, with the script it blames by file name (300 of a kind a take, then one `longcap` line; they start at 50 ms, so the beat is the finer of
  the two; on a page whose usual frame is itself long only one well over it is kept), and the tab going out of sight and coming back are `tab` events. A hold of 20 ms or more outside the frame is a `held` event (300 a take), with whose it was where the recorder can tell: for the
  length of a take two passive listeners hear each wheel, pointer and key event first and last, so the time the page's own handlers took is known (the
  browser names a script only from 50 ms up, and the holds seen on the maintainer's display were 26 to 41 ms). `settings` events leave the two eased
  keys alone (the goggles count as on or off, a hit's flash is left out). A checkpoint written or put back during a take is a `save` / `load` event of the step it
  happened in (a save with a squad up walks the whole game and takes its picture off the canvas), and the report sets what the take says happened in a
  stall's own stretch beside it (a save, the ladder's step, a resize). Mind the frame time's own shape: a frame's `dt` runs from the start of the
  step before to the start of this one, so a slow step shows in the NEXT frame's `dt`, and the reader looks at the row before for it. Reading: a long
  frame after a long step, or under a long GPU frame = IN THE FRAME; with a beat over three tenths of the gap = the thread was HELD between frames (a timer, a collection, another
  script; also what the page itself does in its callback after the step is recorded: a sweep's flip, a benchmark's scene change); with a short beat = the thread was FREE and the browser handed the page no frame (its compositor, the GPU's queue, the page around an embedded
  one). `tools/qa/perf-report.ts` prints a STALLS block (`tools/qa/perf-stalls.ts`: frames over 35 ms and over twice the frames around them, so a slow
  stretch is not a stall; bursts and how often they come; each stall read as above with the long lines that fall in it; long frames with the tab hidden
  counted apart; and the slowest ten seconds of the take where they are well under its usual frame, since a slow stretch reads as stutter with no stall
  in it). It does not guess the display's refresh: frame times on the maintainer's 120 Hz display are not whole refreshes (a take at 45 fps reads a steady
  22 ms, not 16.7 and 25), and two ways of guessing both gave wrong answers on real takes. The watch is on for every take on a page, the sweep's and the benchmark's too (a timer of three
  operations 250 times a second; its cost was never measured on a GPU, `watch = false` is the way to an A/B). Check: `bun run tools/qa/perfrec-check.ts`
  (`PERFREC_BREAK=1` must fail; one part of it runs the real timer on the real clock, one the wiring on a stand-in observer). Why: the maintainer's first take from an embedded page had 40 stalls of about 50 ms in
  three minutes with the GPU and the frame's CPU as ever, and nothing in the file could say whose they were. Run on a real page on the software adapter;
  no take from a GPU carries a beat yet.
- **A page's own rows in the F8 sweep** (`ui/perfsweep.ts PerfSweep.extra`, a list of `SweepExtra { id, label, flip, liveOnly?, warm? }`; the page's
  sweep is `window.__perfsweep`). For a switch that is a page's own and no row of `SWEEP_ROWS` (a prototype page's panel): `flip({ live })` is asked once
  as the take begins and answers `{ from, to, apply: () => undo, settleMs?, ready? }` or, as a string, why the row is skipped here (`ready` = a `warm`
  row's own picture draws). Added rows are flipped in the same rounds as the build's own (after them in the first round; later rounds shuffle all the
  take's rows together, so the build's rows are not in the order of a take without added rows) and named in the take's begin line like any row, so
  `tools/qa/perf-report.ts` prints them with no change (`from` / `to` are printed as the row's change: small plain values). Skipped, each with its
  reason under the name `added row '<id>'`, never stopping the take: an id already taken (`base` too), a flip that throws or is not one, values a line
  cannot carry. A flip must not go through `Quality` (that reads as a hand change and stops the take). The benchmark's sweep (`SweepOpts.inside`) takes
  the build's rows alone. None added (the game's page) = the take as it was. With these the sweep also runs EVERY undo when one throws (it stopped at
  the first) and its closing line then says one row's way back failed. The finished take is on `window.__lastPerfDump`, and `serializeDump`
  (`ui/perfrec.ts`) gives its JSON in parts for a page that cannot download. Check: `bun run tools/qa/perf-sweep-logic.ts` (the hand-change line for
  a page's own row and the block at its end). Run once on a real page on the software adapter with added rows; never on a GPU.
- **The missions gate's commander hunts first (tools/qa/missions-gate.ts; the gate only).** It passes `PT_HUNT=first` to its `ships.ts` run: with no room held the scripted
  commander goes for whoever is left before he walks the rooms again. `playthrough.ts`'s default (every room first) is unchanged, so tallies made with it still compare, and
  `ships.ts --seeds 1-2` by itself still stalls where the gate does not. Why: with the last defender standing out in the open the default ran the compound's seed 2 to the 300 s
  cap with the team 4 of 4 (seven rooms walked from 171 s, the hunt begun at 281 s); an earlier build stalled the warehouse's seed 2 the same way. The cap stayed 300 then (the squad rows' is 420 since 2026-10-26: the bullet at this file's foot). Read on
  one build, 42 squad runs as shipped: every row ends, the longest at 195 s; 3 rows reach a different end than under the default (the compound 2 stalled → taken, the warehouse
  2 taken → wiped, the two charges 2 taken at 178 s → at 120 s). The gate judges a stall, never who won; its report line says which commander ran, and a row is played again
  with `PT_HUNT=first bun run tools/qa/ships.ts --trace <id>:<seed>`. `PT_HUNT=rooms` in the caller's env = the default order under the gate. Headless only.
- **A mission's page waits for Start; a script's page does not** (2026-10-19, `src/ui/loadbrief.ts`; `docs/agents/menus.md` has the screen). A page
  whose address names a mission resolves `__ready` with the mission's brief up and the world held: nothing ticks until the brief is started. A tool is
  never held by it: no brief is made under `?paused`, a replay, `?kitview` or a benchmark scene, `?mute` (or the test profile's mute) drops it at the
  boot's end, and a hook that drives the game (`__runFrames`, `__mouse`, `__click`, a save or a load, `__journal.start`) drops it at the loop's next
  step. A tool that loads a mission with none of those and only watches or shoots the page (`__shot`, `__png`, `__demo.start`) is looking at a held
  world under the brief: it presses Enter, or loads the page with `?loadbrief=0`.

- **A lost GPU device: quiet when the page is leaving, one fresh load for a real loss, then a card that says what was lost (2026-10-19, a play test: the game did not come up on an
  AMD machine; its notes hold "GPU device lost: A valid external Instance reference no longer exists.").** `src/gpu/device.ts GPU_LOST` (ON; `?gpuretry=0` = the red card at once
  for any loss, as it was). That message is the one Chrome gives a device whose page is going — a load to another address, a closed tab, the Team & kit picture's window dropped
  (reason `destroyed`) — and it used to put the card up on the page being left and print an error for every kit picture closed: now it is one `console.info`. A real loss on the
  game page loads the page again (twice a tab at most, never twice inside a minute; never on a benchmark page, a driven page (`navigator.webdriver`) or the kit picture's window),
  and the card that follows carries the adapter, the browser's reason and where the boot stood (`gpuStory`: "while compiling shaders, 37 of 119"). `tools/qa/gpulost-check.ts`
  (the rule as a table; `GPULOST_BREAK=1` must fail) and `tools/qa/shaderwait-page.cjs` (the real page, on `?paused` pages that draw nothing: once the browser has TOLD the loss,
  `device.destroy()` puts no card up and prints no error; under `?gpuretry=0` the old one-line card. On the software adapter the loss is told by the browser's GPU side, minutes
  late behind a drawn frame: a check that waits a second for it passes "no card" without the loss ever having arrived). NOT FOUND: why the AMD machine failed. Nothing here has been run on one; the other line in its notes ("invalid: resource doesn't exist")
  is not a string of the page. What would find it: the card's new second line from that machine, its `chrome://gpu` text, and whether the title drew before the card.
- **A gentle second try: a browser that loses its device while shaders compile compiles them one at a time from then on (2026-10-29, the AMD machine's first records).**
  `src/gpu/device.ts GENTLE` (ON; `?gentle=0` = no note written or read, as it was; `?gentle=1` = this load is gentle; a `?pipequeue` word wins; the page's own, no stand
  differs in it). WHAT THE RECORDS SHOWED (a Radeon RX 9070 XT, Windows, Chrome 156, 16 threads, the hosted build): three losses in a minute, all "A valid external Instance
  reference no longer exists." with reason unknown: one 7.5 s into the boot's compiles with 17 of 105 in, one 8.1 s into a mission's eight with the five heaviest kernels
  (`vol`, `restir`, `reflect`, `compositeSR`, `gatherTraceS`) and `particlesUpdate` still out, and after the third the browser gave no adapter and no WebGL (which is what
  the "no webgpu adapter available" of 10-27 on that machine was: the browser's GPU side switched off after its process went down repeatedly). One load in between compiled all
  105 in 10.2 s, so the same programs come through or do not. In hand each time: the burst the queue allows at the boot or under the rush, the threads less two (fourteen).
  WHY the process goes is NOT known: its memory under fourteen big compiles, or the driver under several at once, are guesses. THE RULE: `onLost` writes
  `localStorage['twopointsix.gentle']` ('count:when'; a machine key, on the settings sync's skip list) when a REAL loss comes while shaders compile (the boot's count
  open, anything in hand or waiting, the later set out) on a page that retries (so never a benchmark, the kit picture or a page a script drives) and whose load was not
  gentle already. A load that finds the note young enough turns the queue on wherever it is with ONE in flight at the boot, after it and under the rush (`gentleQueue`),
  says so in a warning record (`gpu.gentle`, with `why`) and on the card of a further loss ("shaders compiled one at a time"). Young enough: ONE day after a first loss
  (one loss a healthy machine had for its own reasons must not cost it a month of slow loads), THIRTY once the burst has lost the device again within 180 days. So that
  machine pays one more lost load the day after its first, and is gentle for a month from then. `watchPipes` counts every `…Async` creation of the device as in hand
  from the browser being given it to its answer (beneath the queue, so at a creation's turn), and the loss's error and fatal records carry `flying` (the labels in hand
  when the device went, as up to three strings of 80 characters, `flying2` / `flying3`, and `flyingOf` the count; kept from the first rejection of the burst, since a
  lost device rejects what is in flight before it says it is lost and the queue then hands it what waited) and `gentle`. So a gentle load that dies too names the one
  program it died on. CHECKS: `tools/qa/gpulost-check.ts` (the note, its count and its age as tables; what a record keeps of the labels), `tools/qa/pipequeue-check.ts`
  (one in flight in every phase; what a loss had in hand, with the queue draining onto the dead device), `GENTLE_BREAK=1` must fail in both; and
  `tools/qa/gentle-page.cjs` on the real page (headless Chromium launched so the page sees no driven browser, software adapter: the browser's GPU process is killed
  through the debugging protocol while the page shows "compiling shaders 3 / 105"; the page is told the loss, notes it, loads itself again, and on that load never has
  more than one program with the browser while over a hundred wait, prints the warning, shows no card; `GENTLE_WAIT_S=N` also waits for that load to come ready;
  `GENTLE_BREAK=1` runs it under `?gentle=0` and must fail). `?gentle` rides a replay's and a benchmark's addresses (`VISIT_FLAGS`, `CARRIED`). PRICE: a gentle load
  waits for the heavy kernels' sum, about a minute on that machine by its own per-kernel times where the burst took ten seconds, and each mission's start waits
  likewise. KNOWN SEAMS: a benchmark on a noted browser reads the note and times a gentle load (its records hold the `gpu.gentle` warning; nothing on the report says
  so); a loss told more than ten seconds after the creations failed is not counted as one while compiling unless the boot's count is still open.
  NOT SEEN: any of it on a GPU. Whether one at a time gets that machine through is what its next visit's records say (a `gpu.gentle` warning and then a boot record, or a
  loss with `gentle` and one label in `flying`). Still what would settle the cause: that machine's `chrome://gpu` log lines after a loss (the GPU process's exit code).
- **Plain air: a driver that stops on the volumetric program is not handed it (2026-10-30, the AMD machine's second visit).** `src/gpu/device.ts PLAIN_AIR` (ON;
  `?plainair=0` = no note written or read and no adapter's row, as it was; `?plainair=1` = this load is in plain air; the page's own, no stand differs in it).
  WHAT THE RECORDS SHOWED (the same Radeon RX 9070 XT, on the build with the gentle try): the burst lost the device again starting The compound, with `compositeSR
  restir reflect vol gatherTraceS particlesUpdate` in hand; the load after it was gentle, made `compositeSR`, `particlesEmit`, `particlesUpdate`, `restir` and `reflect`
  one at a time (4.9, 5.6 and 4.2 s for the three heavy ones) and lost the device 23 s in with ONE program in hand, `vol`. `vol` was in hand at every loss of that machine
  whose record names anything (four of four), alone at the last. So it is not how many compile at once. NOT known: why the driver's process goes on that program,
  whether it does every time (one title load of the first visit made all 105, `vol` among them), and whether `vol` is the only one (the two gathers behind it in the
  gentle order have never been tried by themselves there). THE RULE: a load in plain air gives the volumetric pass `shaders/vol_plain.wgsl` under the pass's own label
  (`render/passes.ts traceJobs`, so the cut set's build too) and makes none of the real march's twins (`volFroxel`, `volDust`, `volK0`). The plain module has no
  function in it that walks the boxes, a light, a shadow or the fog of war's sight (11 KB of text where the real march's module is 74): the smoke's density marched
  across the hull of the live domains and lit by the radiance cache's mean radiance alone, a picked weather's air marched the same way with the open sky's share. Its
  program draws no beam, so plain air forces the froxel path on every tier as `?froxel` does (`main.ts`; `?nofroxel` still wins, and then there are no beams):
  the haze's cones are the froxel kernels'. LOST there: the lamp-lit, self-shadowed plume (it is lit by the cache alone, on the froxel tiers too), rain streaks and
  splashes of this pass, the lit sky under lightning, the sun's shafts in a weather's air; dust is drawn as plain smoke; smoke is drawn under the fog of war whatever
  `FOG.air` says (the page ships it so). KEPT: smoke and a weather's air stand where the men's eyes are cut by them, and a torch's cone is in the air. A load is in
  plain air when the page says so, when the browser noted it (`localStorage['twopointsix.plainAir']`, 'count:when' as the gentle note, a machine key on the settings
  sync's skip list: written by `onLost` for a REAL loss on a page that retries with exactly ONE program in hand, that one of `PLAIN_AIR.labels`, on a load not plain
  already; it holds ONE day after a first such loss and THIRTY once the real march has lost the device again within 180 days, then the real program is tried
  again), or when the adapter is one the records name (`PLAIN_AIR.adapters`: a browser on Windows whose adapter's name starts `amd rdna-4`). THE ROW IS A GUESS FROM
  ONE MACHINE and has no second look of its own: a machine it names is never handed the program, so only a `?plainair=0` load of such a machine can say the row is
  no longer needed; it spares that machine the lost loads a note costs (two before the note, and its third lost GPU process switches the browser's GPU off). The card
  of the loss that wrote the note ends "Load the page again and the game will skip the shader your graphics driver stopped on. Smoke and fog will look plainer." (only
  when the note was written); a plain load says so in a warning record (`gpu.plainair`: `why`, and `row` or `noted` / `hours`), on a further loss's card, in that
  loss's facts (`plainAir`) and in a benchmark's notes, so a machine that still loses its device names the NEXT program with this one out of the way. CHECKS, each on
  both stands: `tools/qa/gpulost-check.ts` (the note, the row, the word, the three reasons and the card as tables; `PLAINAIR_BREAK=1` fails seven lines);
  `tools/qa/plainair-check.ts` (the Engine built on a stand-in device as any load and in plain air: one plain module, no text of the real march, no twin asked for,
  every other program as ever; the plain module AS CUT declares no walking function and is a third of the real one or less; the numbers it copies from the real march
  are still the real march's, and on any load the weight of a lamp's glow through smoke (the look's row, `Engine.smokeDraw.ahead`) reaches the frame at 0 with
  only its two writers naming it, so the real march's cut is the plain one's; `TPS_COMPACTSHADERS=1` in front for the page's writing; `PLAINAIR_BREAK=1` fails five
  lines, seven with the compact set); and
  `tools/qa/plainair-page.cjs` on the real page (headless Chromium, software adapter, `?plainair=1`: every program made, 117 where the page makes 118; the browser is
  handed `volumetricsPlain` and no text of the real march, no twin is asked for, the warning is printed, a canister's plume shows in the picture and in the volumetric
  pass's own, no GPU error, no card; `PLAINAIR_BREAK=1` runs it under `?plainair=0` and fails four lines; eighteen minutes on this box, most of it the cut set
  compiling behind the page). `?plainair` rides a replay's and a benchmark's addresses (`VISIT_FLAGS`, `CARRIED`). NOT SEEN: anything on a GPU; the plain plume beside
  the real one on anything but the software adapter; the froxel path forced on high or ultra together with the plain program; a driver that stops on the real program
  getting into the game this way (that machine's next visit says: a `gpu.plainair` warning and then a boot record with frames = it was `vol`; a lost device with
  `plainAir` and other labels in `flying` = it was not only `vol`).
- **A start with no GPU says what the machine said, and asks twice (2026-10-27; the maintainer: a friend's AMD machine "is still crashing with failed to create WebGPU context
  provider / no webgpu adapter available", "anything we can collect from his computer?").** `src/gpu/adapter.ts GPU_ASK` (ON; `?gpuask=0` = one ask and the one line "No WebGPU
  adapter available.", as it was; like `GPU_LOST` it is the page's own, no stand differs in it). `initGpu` asked once, for `powerPreference: 'high-performance'`, and an empty
  answer was that one line: nothing told a browser with no WebGPU from one that keeps the card back, and nothing was sent, because the backend boots after the GPU. Now an empty
  (or thrown) first ask is followed by an ask with NO preference, and the game starts on the card that gives, with one `console.info` and " (given to the second ask)" after the adapter's
  name (`gpuStory.adapter`, so the benchmark's report and a visit's record carry it and such a machine can be found); the browser's software adapter is never taken from that second ask (a browser whose FIRST answer is that adapter starts on it as before: a test box). With no card from either, the
  software adapter is asked for, for the record only, and the start throws `NoGpuError` whose `facts` are plain JSON: each ask's answer, WebGL's vendor and renderer (the card and
  driver by name where the browser tells), the user agent, the system and full browser version where the browser tells, secure address or not, threads, memory, screen. The red
  card (`showFatalError`, gpu/device.ts) says in a sentence what to try (no WebGPU in the browser · WebGL drawn in software: turn graphics acceleration on · else restart the
  browser and update the driver), lists the facts under it and has a Copy button (the copy is made inside the click; a browser that refuses leaves the lines selected). The second ask gives up
  after `GPU_ASK.waitMs`, the software ask and the system's name after `softMs`, side by side. EVERY RED CARD now tells a listener (`FATAL.tell` in gpu/device.ts: the plain error,
  its stack, `stage` 'adapter' or 'device', where the boot stood, the adapter, and for this card the facts) and hands it the card. NOBODY SETS IT in this step: sending is the
  backend's side (`src/cloud`), which boots after the GPU and so needs its own way in. `tools/qa/nogpu-check.ts` (a stand-in browser that answers each ask as the case says,
  and the card on a stand-in page; `NOGPU_BREAK=1` = the switch off, must fail) and `tools/qa/nogpu-page.cjs` (the real page with `requestAdapter` stubbed; `NOGPU_BREAK=1`
  must fail). MEASURED on that page (Chromium 141, the software adapter): the card with its lines, the Copy button on top and its text on the clipboard, `?gpuask=0`
  the old card after one ask, an empty first ask refused on this box's software adapter, a plain boot with no card and a take made with no download (nobody listens on that page). NOT KNOWN: whether any real machine answers the second ask
  after an empty first one (nobody here has one; on the friend's machine it may change nothing), what Chrome's "Failed to create WebGPU Context Provider" means there (it is a
  console line of the browser, not something the page can read), and the card has never been seen on a machine that really has no adapter. The three sentences of advice are
  guesses from the facts, not a diagnosis.
- **How much a page compiles: `node tools/qa/shader-count-page.cjs [--port] [--settle S] [--json f] [address …]`** (2026-10-19). Wraps `createShaderModule`, the four pipeline
  makers and `setPipeline` before the page's script runs and prints, per page of a walk (default: the title, a mission's kit picture, the mission, a second mission): modules,
  distinct texts and their size; pipelines asynchronous and synchronous, before and after ready; how many texts and pipelines an EARLIER page of the walk had made character for
  character (the most a browser's shader cache can hand back); which boot pipelines a pass took after ready; the boot line. Exit 1 on a page that did not come ready or printed a
  GPU error. The counts are the page's; no time it prints is a price on a real machine (a page is three to seven minutes on the software adapter).
- **How big each kernel is once every call is pasted into its caller: `bun run tools/qa/shaderinline-check.ts [--top N] [--kernel <module>/<entry>] [--json f]`**
  (2026-10-26; what Chrome's Windows compiler does to a kernel, as a number: `docs/agents/render.md` has the rule, today's sizes and what it cannot see). The page's
  boot and the cut-box set over a recording stand-in device, always on the shipped stand; two seconds. Exit 1 when a kernel is over its row of the budget table at
  the head of the file, a row has no kernel left, or the stand-in's boot did not finish or came up short; `SHADERINLINE_BREAK=1` must fail. Run it after a change to a `.wgsl` file or to what a module is
  built from, and move the kernel's row in the same change when the growth is meant.
  THE CUT TEST IS COUNTED APART (2026-11-05): a kernel is held to its row without the text of `isectCut` and `cutFour`; that text has its own number
  (`CUT_TEST_KB`, 3.1 of 3.2 KB at one copy of the box test); and every kernel of the cut set is held to its boot twin with the round's own test out and the
  cut test in, a fiftieth of the twin at most on top (`docs/agents/destruction.md`, THE BOX TEST HAS NO LOOP). The JSON's `cut` and `round` are those texts in
  a kernel; `SHADERINLINE_BREAK=1` measures the same way.
- **The night breach's probes on the SHIPPED stand** (2026-10-25; tests only). `nightbreach-stage` and `nightbreach-stop` read 'tune CHANGED' there: the
  rig (`tools/qa/nightbreach-probes.ts rig`) took its `before.tune` and THEN wrote `game.tune.chaos` itself, and the shipped stand stands the game up at
  chaos 0.5 where the frozen one has 0 — the rig's own write read as the staging's. The write is before the snapshot now; both are green on both stands.
  STILL RED THERE AND KNOWN, not mended: `nightbreach-contested` ('the man Sam's rifle pins fires no blind round; the two it cannot reach fire theirs')
  and `nightbreach-restart` (its last line wants `blindRounds > 0`). Both are bars set on the frozen stand's fight; on the shipped stand a run has 0 to 2
  blind rounds (`nightbreach-fight`, seeds 11‥20: none in two of ten, the room cleared in all ten), so a line that needs one fails by the seed. Whether
  dazzled defenders SHOULD fire blind more than that on the page is the room fight's question, not a probe's. The mission itself is off the list
  (`docs/agents/menus.md`, its foot); the staging under it is the room clear's and the rescue's.
- **The missions gate's squad rows have a 420 s cap** (2026-10-26; `tools/qa/missions-gate.ts`, the gate only; a loosened bar, said as one). It was
  300 for every row. As shipped the compound's seed 1 is TAKEN at 311.9 s with three of four up, 12 s past the old cap, so the gate read a slow win as a
  stall. Measured with the gate's commander (`PT_HUNT=first bun run tools/qa/ships.ts --cap 900`): all 42 squad runs end, the other 41 by 180 s; the
  compound's seeds 2 to 8 end between 91 and 206 s (five taken, two wiped). The same seed is taken at 170.6 s on a build of 2026-10-19 and at 311.9 s on the
  four builds tried from 2026-10-23 on, and on that older build the compound's wins ran 127 to 185 s over eight seeds: the mission as shipped got slower for the scripted
  commander between those dates (which step did it was not looked for; one run is no measure, the sim is not the same game after any change). The quiet
  rows keep 300. `--cap <s>` sets both, and `--cap 300` gives the old gate, red on that one row. What the gate still says: a run that has not ended in
  seven minutes is stuck. What it does not say: that five minutes for the compound is good play; that is the tallies' and a playtest's question.
  WHY THAT RUN IS LONG (found by the squad's own tallies, the two words run again here on the build head, seed 1, the gate's commander): 311.9 s as
  shipped; 250.5 s with `TPS_DOORFORCE=0` (an alerted defender no longer waits at a shut door: the office man who wedged his door now leaves by the
  back instead of pulling it open onto the squad); 179.5 s with `TPS_TEAMPACE=0` (teammates walk slower than the player, which is most of the
  mission's general slowing since 2026-10-19). Neither is the squad's work at a door: its entry on the wedged door is as quick as it was. On top of
  those the SCRIPTED COMMANDER idles about 50 s of that run where a player would not (`tools/qa/playthrough.ts`): he sets off for a room by its
  defender's home after the man has left it (`heldRooms`); Sam pushes 17 s at a wedged door before `go` takes the way round, though the team had
  tried it; and he stands 28 s short of his post behind an open leaf with a teammate in the doorway. Mending those three is what would let the cap go
  back toward 300. Not done: the default commander is every tally's commander, so a mend would be a word of the gate's own like `PT_HUNT=first`, and
  it buys this gate nothing but the tighter number.
- **`PT_STILL=1`: the scripted player stands off his men's line of fire** (2026-10-26; `tools/qa/playthrough.ts heldOff`; a word of the harness, OFF by
  default, the game untouched). The commander side-steps out of the line between a teammate and the man he is on (`offLine`: inside 0.9 m of it he goes
  for a spot 1.3 m off). The frame after he is 0.9 m clear nothing sends him off any more, so he steps back toward his goal, into the line, and out again
  the next frame: 2.6 cm each way for as long as the line lasts. The squad's measure of the player reads that as a walk, so nothing that waits on him
  standing runs while he does it (the fall-in rest among them; found by the squad's own tallies). With the word, just outside the line and with his way
  on closing on it, he stands until it clears; `go`'s stuck watch and the smells count that wait as time off the line, as they do the side-step.
  Default off because he is every tally's commander: unset, a run is the run it was (the compound's eight seeds below, to the tenth). MEASURED, the
  compound as shipped, the gate's commander, seeds 1 to 8, plain → with the word: 5 of 8 runs change; taken 6 → 7 of 8; teammates up at the end 8 → 12;
  seed 1 311.9 → 334.1 s, so it does NOT shorten the gate's slow run and the gate does not pass it. Eight seeds of one mission is no tally. Not measured
  here: the 2.6 cm itself (the squad's finding), any other mission, the frozen stand. Tallies that want a player who really stands put the word in front.
  THE SMELL IT ENDS, by the squad's own tally of the same eight seeds on a later join: 'in the doorway for N s · fallIn (on his mark)' 3 lines without the
  word, 0 with it. Making it the default is open: 84 files of `tools/qa` lean on this commander, and every table before it was made with the walker.
- **Every shader text compiled in twenty seconds: `node tools/qa/wgsl-compile.cjs`** (11-01; `tools/qa/wgsl-dump.ts` writes the texts). The Engine and the smoke
  solver are built on a stand-in device that records every module's text, as a plain boot makes them and again as the look page builds them (its prelude consts
  up), in TWO runs: bare, and with every shader row's word in the environment (`TPS_COMPACTSHADERS=1` and the rest: the rows are read once at import, and a first
  writing that flipped the row after the import compiled none of the compact writings and said nothing). Each distinct text (68 today) goes to headless
  Chromium's `createShaderModule` and its errors are printed with the line. Parse and type errors only: it makes no pipeline and draws nothing, so it does not replace a page boot (see the first-frame bullet in `render.md`).
  `WGSL_BREAK=1` adds a function that names nothing declared and must fail.
- **One kernel alone on the software adapter, with its stack frame against the adapter's limit: `node tools/qa/kernel-stack.cjs`** (11-01; 40 s a kernel;
  default `fgatherS/main` on the look page, `--kernel a,b`, `--list`, `--plain` = the game page, `--query '&look=walls:0'`, `--text file.wgsl`). A page that boots
  clean and loses its device on the first frame after a shader change is this: `docs/agents/render.md`, the software adapter's limit. Exit 0 built and ran, 1 not
  built (it prints the compiler's own words), 2 no browser or no such kernel. The numbers need gdb and the symbols Playwright's Chromium ships in
  `libvk_swiftshader.so`; without them it still says built or not. It attaches to its own browser's GPU process (found by a word it puts on that browser's
  command line), so another page check may run beside it.
- **A check that needs a wall stands one: `tools/qa/standwall.ts`** (2026-11-05; tests only, no rule of play). A check used to lay a function of its own over a
  game's line test (`g.col.segmentBlocked = () => true`, `solidBlocked` the same). Only the TypeScript that calls the method met it: a question answered
  inside the Rust module (an eye on a man, the lamps on a point, the smoke along a line, a sound's way, a way across the floor, the fog's pass) reads the
  module's copy of the BOXES, so the game kept a guard (`collision.ts ownLines`) that hands every such question back to the TypeScript while a stand-in is
  laid, and that guard is the last thing in the tools that needs the TypeScript sides. A wall that is a box is met by every reader in either language.
  `standWall(g, walls)` puts boxes on the end of the box world's list (`collision.ts addStatic`, the road a holed wall's pieces take) and returns the
  function that cuts the list back (it refuses a list that is not the one it left: nothing else may add a static while a wall stands);
  `wallBetween(a, b, at, half, top)` is a thin upright wall across a line; `clearLine(g, a, b)` hides the statics one line meets and nothing else
  (`BoxFlag.NoShadow` raised, as on a box the game itself has hidden), so a game may be played under it; `noWalls(g)` hides every static, the floor
  too, for a question asked at once. Each returns the function that puts back what it changed and no more (a flag is lowered only where it was
  raised here). A door's leaf is no static: neither hides one. The floor's grid is not baked again: a man meets a wall and does not plan round it.
  Take walls down last up first; stand none across a restart.
  THE FIVE CHECKS THAT LAID A STAND-IN NOW STAND WALLS, and each reads what it read (both stands, the same day, the lines compared with the tree before):
  `autocam-check` (a wall half way to a man at his side; a wall 0.9 m toward the lens as wide as hid everything within acos 0.9 of that way, stood
  again where he stops since a wall does not follow a man as the function did; a pillar half way to the locked man): every line of its output the same;
  `stalk-check` (a wall half way between Sam and his man where 'something solid comes between them'): every line the same, and under `STALK_BREAK=1`;
  `deathrecap-check` (`RECAP_BREAK=seen` hides what stands between his mate or Sam and the man, where he lies and at his chest: every line the same, and the break fails the lines it failed; its output the same but for a digest one byte longer on the page's stand); `moonlight-check`
  (`MOONLIGHT_BREAK=1` asks of a level with no static standing): every line the same, the break fails `shadow` as it did, to the digit;
  `squaddoors-check` (the teammate who must not see the leaf, at a door with no place behind a wall of the level's: a wall is stood between a place in
  front of the door and the leaf, the first that hides every point of the leaf by the check's own lines; before, every line from within 0.6 m of
  where he stood was shut by hand): the one staged line reads its new place (0.75 m from the leaf on the frozen stand, 1.50 m on the page's, for 0.46
  and 1.20) and passes, `SQUADDOORS_BREAK=1` fails the two lines it failed, and with the wall left out the same man does see the leaf (shown by hand).
  No tool lays a function over the line test any more (a grep of `tools/` for `segmentBlocked = ` and `solidBlocked = ` finds two comments), and once
  the senses' TypeScript sides are deleted `ownLines` false is an error said by name, so that a sixth tool that tries it is told. NOT TOUCHED:
  `morale-check` lays a function over `col.raycast`, which is the rays' and no part of that guard.
- **Two measuring tools for a machine with a GPU: a page's repeat visits, and where a frame's CPU goes** (2026-11-05; neither is a check, neither has a
  PASS line, no probe runs them; both want `PORT=<a dev server's port>` and the Playwright the other page tools use).
  `node tools/qa/warmboot-page.cjs [--query …] [--plan '<name>:<visits>:<stay>[:<cache KB>][:restart] …'] [--out file.json]` opens one address again and
  again in ONE browser the way a player comes back to it (a new tab each visit, the old one closed; `restart` closes the browser itself between visits and
  starts it again on the same profile) and prints each visit's boot record: ms to interactive, the three slowest pipelines by name, every pipeline group
  over a second. `stay` is `full` (until every shader built behind the start is in) or seconds after interactive; `cache KB` is Chrome's own size for its
  store of compiled shaders. Each scenario starts on a fresh browser profile.
  `node tools/qa/cpuprofile-page.cjs --query … --do '<js>'` (or the script in `PAGE_DO`) runs a script in the page and takes Chrome's own sampling CPU
  profile over the stretches the script names (`await __prof.start()` … `await __prof.stop('a name')`): the call tree from the top down, every branch
  over 2 % in ms, and the functions that cost most themselves, by the names the dev server's bundle keeps (it is not minified). What a frame recorder
  cannot say: WHAT in `engine.render` or `game.update` a dear frame was. First used on the frames a wall comes down under the look.
  WHAT THE FIRST TOOL FOUND ABOUT THE RENTED CLOUD CARD, a caution for every "warm" number taken there (an NVIDIA A10G, Vulkan, headless Chrome 157;
  five boxes of this tool, after eight of a rig that reloaded the page in a frame of itself and showed the same three programs): (1) what makes a second visit fast there is THE CARD DRIVER'S OWN CACHE on the box's disk, not Chrome's store. With the driver's cache
  switched off (its own environment word) every one of five later visits compiled for 24 to 28 s, as the cold one did, with Chrome's store at its own
  size and at 100 MB alike. With it on, a brand new browser profile's first visit was 4.1 to 4.2 s. (2) THREE PROGRAMS ARE COMPILED A SECOND TIME before
  that cache keeps them: `compositeR`, `compositeS` and `particlesUpdate`, 7 to 9 s each, each once more after the cold visit, on whichever later visit
  it falls (the first, second or third; with Chrome's store at 100 MB too). After that every visit was 3.9 to 4.7 s, in a new tab or after a restart of
  the browser, short stays or full. Why those three, nobody knows; it is the driver's and nothing of the game's. So: A WARM BOOT TIMED ON A CLOUD BOX
  IS NOT TO BE TRUSTED BEFORE ITS FOURTH VISIT, and it says how the driver's cache behaves there, NOT whether a player's browser keeps its shaders. On
  Windows Chrome's own store is what has to hold them and is too small for this game (the shader rows above); on an Apple chip nobody has looked.
- **A game stood up has done with the last one: `standUp` asks for a full collection first** (2026-11-05; `tools/qa/headless.ts STANDUP_COLLECTS`; the
  harness only, nothing of the game). FOUND by the merge: the workers of `bang-tally.ts` (the `flashweight` family's tally, missions back to back in one
  process) grew about 1.6 GB a minute on the page's stand and two died at a 12 GB limit; old, on every build looked at. Reproduced here: one worker from
  0.4 GB to 6.4 GB in two and a half minutes. WHAT IT IS NOT: a mission's world being held, or the tally. After ONE forced full collection a process that
  has played forty-two missions over twenty-one scenarios is 350 to 450 MB, the size of one that has played two. WHY THE GAMES STAY: these tools run a
  whole batch without once going back to the event loop (every `await` is on work already done), and that is where bun's collector does its full
  collections from; so a dropped game (30 to 45 MB of heap a mission on the page's stand, about 7 on the frozen one) stays as garbage, and the heap
  itself grows, not only what lies outside it. Shown both ways: with no collection asked, ONE turn of the event loop between bare stand-ups holds the
  heap as well as the collection does (growth 29 to 38 MB over 25 games against 141 to 184); between missions it gives two to five times less growth,
  the collection ten times less.
  THE CURE is in the one place every tool stands a game up: `Bun.gc(true)` at the head of `standUp`, from a process's second stand-up on. The same
  worker then stays under 0.8 GB (nine minutes, four workers at once). WHAT IT COSTS: about 50 ms a stand-up, more with a bigger live heap (0.2 s at
  worst seen): the doors family, 247 stand-ups, paid 12 s of 381; a loop of bare stand-ups is 6 to 10 % slower; missions of a second or more are not
  slower for it (32 tally missions: 91 s with it, 93 without). `QA_STANDUP_GC=0` = the stand-up as it was, for measuring; any other value but `1`
  stops the tool. It does nothing for garbage that is not a dropped game (a single long game's own: one game restarted in place thirty times grows
  77 MB uncollected on the frozen stand and is 112 MB after one collection).
  **Check**: `bun run tools/qa/standupgc-check.ts` (`probes.ts standupgc`, family `perf`, run as a process of its own), both stands, about 8 s:
  twenty-five games stood up back to back, twenty steps each, nothing collected by the script, the word unset as every tool runs; the heap may grow
  60 MB from the second game to the last. Measured: 28 MB (frozen) and 32 (the page's stand) with the collection, 141 and 184 without. A second line:
  a spelling of the word the harness does not know stops the tool by name. `STANDUPGC_BREAK=1` (the script under `QA_STANDUP_GC=0`) must fail.
  WITH IT IN, green on this machine: the `checkpoints` (13 probes), `replay` and `journal` families on the frozen stand and `rollback-check` whole on
  both. Nothing a game can observe depends on when a collection runs (the one `WeakRef` user in the simulation's folders, `math/side.ts`, is read
  through reachability by the rollback).
  THE PAGE: not this. In a browser a mission is a page load and its world goes with the page; the page's own way to play again without a load, the
  restart in place, keeps nothing (one game restarted thirty times with four seconds of play between: 116 MB to 121 after a full collection, the
  page's stand). Nobody has watched a browser's heap through a long session; nothing here says it grows. A TOOL THAT KEEPS GAMES ALIVE ON PURPOSE
  (two stood side by side, a list of them) is not helped by this and is as big as its games.
