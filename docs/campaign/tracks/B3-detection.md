# Track B3 — light-based detection & guard AI

Branch `claude/detection`, forked from `claude/campaign` @ `230295a`.
Every number below is game time from the headless SwiftShader harness at
384×240 (`tools/headless/run.py`, sandbox off), one Chromium at a time,
with the clock owned by the scenario: `__pause(true)` stands the rAF loop
down and each `__renderStill(n, 50)` frame is exactly 50 ms, so a ramp is
seconds of game, not of this machine.

## What changed and why

- `detection.ts` (new): per-guard perception at 15 Hz. Cone = torch beam
  axis, LOS = `raycaster.blocked` on static geometry, signal =
  `litF·distF·seenRate + beamF·beamRate` where `lit` is the same probe value
  the LIGHT gauge draws. Eyes are on the player only while signal > 0
  (`sees`); a clear ray to a dark body carries nothing, and suspicion
  integrates net of a forgetting rate. States patrol / suspicious / alert /
  search; hearing (gunshot, sprint steps, thud, one-hop callout); found
  bodies alert; seeded fire cadence, one-hit fail state. Tuning lives in one
  `detectionTuning` object on the tweak panel.
- `guards.ts` brain state + mode API (`route` / `hold` / `follow`), beam
  tint eases with mood at constant luminance, `reset()`; corridor route now
  patrols the east half so its beam never scores the spawn. `nav.ts`
  (new): 0.5 m raster + A* with string-pull, deterministic, excursions
  only.
- `probe.wgsl`: the player's own flashlight counts (direct + one
  backscatter tap); shadow rays to a spot light stand 0.3 m off the carried
  lens. `post.wgsl`: red edge pulse (`FrameState.seenPulse` → `pp[15]`).
- `player.ts` kill / reset / stance target, torch starts off; `main.ts`
  wiring, COMPROMISED + R restart (restores shot lamps and the OCP too),
  GHOST / CLEAN card; `overlay.ts`, `gauge.ts` end card, objective,
  DETECTION meter; `__pause` + fixed-step `__renderStill` for asserts.

## Verification

`npm run typecheck`, `npm run build` clean. Ten scripts under
`tools/headless/scenarios/`; each returns numbers plus `ok`, and `run.py`
exits non-zero on `ok: false`. All green at HEAD (`tools/headless/run.py
--scenario <name>.js`):

**beam-detect** — 4.0 m down guard 0's beam, feet frozen. LOS ✓, inBeam ✓,
signal 1.67 rising to 2.65 as his torch reddens onto the player; suspicion
0 → .16 → .40 → .66: suspicious at t = 0.20 s, **alert at t = 0.30 s**,
pinned 1.0 by 0.50 s. Probe 13.6 lux, meter 1.00; HUD `DETECTION | SEEN`,
`LIGHT | EXPOSED`.

**dark-los** — the thesis check both reviewers broke. Guard pinned looking
down the corridor, player 16 m in his cone past beamRange, torch off, the
eye's floor forced to 1 so the signal is exactly 0 with the ray clear. Idle
0.5 s: `hasLOS true, signal 0, sees false, suspicion 0, stimulus null`.
Seed suspicion 0.40 + stimulus at the player's feet (a heard noise):
suspicious, and suspicion **decays** to 0.31 in 1.5 s with the ray still
clear. Move the player 3 m: stimulus stays on the noise (−12, 0.8). Force
suspicion 0.9 (any gunshot or shout): alert, mode `nav`, chasing the noise
— after 3 s of a clear ray to a target he has no light on, **0 rounds
fired, player alive**.

**los-crouch** — guard 1 pinned 2.9 m from a player behind a 1.35 m
partition, in his beam. Raw raycast: 1.5 m target clear, 0.85 m target
blocked. Crouched, 16 ticks: LOS false, sees false, suspicion 0.000, meter
0.36 (dim). Standing: LOS, beam, signal 2.29, suspicious at 0.15 s, alert
at 0.35 s.

**hearing** — real trigger (mousedown → shot at (−25.2, −16.2)). A, callouts
silenced: guards at 11.9 / 13.3 m alert with stimulus on the shot; guards
at **38.5 and 41.2 m stay patrol, suspicion 0** (gunshotRange 34). B,
callouts on, same shot: alerted = shot ∪ one shout hop (the two in-range
guards; the two beyond are 30 m from either, untouched). C, hand-placed
line — g0 29 m from the shot, g1 16 m from g0, g2/g3 within 15 m of g1
only: g0 alert (shot), g1 alert (g0's shout), **g2 and g3 patrol,
suspicion 0**. One hop, never a chain.

**probe-check** — 4 m down the beam, world frozen, 8–10 frames per phase.
Frame-to-frame max/min: stand 1.05, crouch 1.06, calm 1.05, alert 1.06 —
**no bimodal reading** (was 13.4 ↔ 0.44). Beam fully red on alert
((1, .18, .06) at intensity 460.6 vs 170 calm): meter 13.58 → 13.50 lux,
ratio **0.994** — the mood colour is not an input. Own torch: pointed down
the empty corridor +0.18 lux (+1.3%); 0.9 m off the conference wall, aimed
at it (yaw within 5° of east), 1.39 → **3.94 lux (×2.8)**.

**fail-state** — lamp #20 shot out through `shootOut`: probe under it
0.517 → 0.233. Standing 5 m down guard 0's beam: first shot at **t = 1.10
s** (0.30 s ramp + 0.70 s reaction, seeded cadence), a hit, `COMPROMISED |
PRESS R TO RESTART`. `__restart()` (what R calls): player at (−13, 0.5)
alive, 11 rounds, spares [10, 10], OCP 1.0, all four guards
patrol/route/suspicion 0, card hidden, shot lamp back on the CPU list and
the probe under it reading 0.535 again.

**soak** — server guard alerted with the player in its beam, player then
teleported to spawn: alert/`nav` (pursue last-known) → search/`nav` →
search/`hold` at (13.55, 6.24) sweeping → patrol/`route` at 3.75 s
(searchTime shortened to 2 s), suspicion drained to 0.12 on giving up. Its
one callout reached the corridor guard 18 m away, still hunting the point
when the scenario ends — one hop, one guard.

**spawn-afk** — the opening promise, world live: `__restart()`, no input,
**75 s** (two corridor laps). Worst suspicion across all four guards
**0.000**; player meter peaks 0.35 (the corridor beam washing the spawn
from ≥ 22 m at his west turn) and returns to 0.11. Coda: torch on at spawn
for 20 s → meter **0.84** (EXPOSED, off the west wall) — no guard has the
spawn in range, so still 0 suspicion, but the meter says what stepping
into anyone's view would cost.

**smoke** — cold start: 33 lights on the probe, spawn meter 0.11 (HIDDEN),
nav raster 104×72 with 1995 / 7488 cells blocked, four guards on route.

Screenshots at 384×240 internal (page 1280×657), read back:

- `seen.png` (beam-detect, 1.5 s after alert): the corridor mid-frame, a
  bright beam pool on the polished strip around the standing player figure
  with the guard at its head up-left; the pool's rim and the whole left,
  right and lower frame breathe deep red (the SEEN pulse plus the reddened
  beam); HUD `LIGHT ▮▮▮▮ EXPOSED` and `DETECTION ▮▮▮▮ SEEN`, objective line
  top-left. Red rather than amber: alert is the state, amber is the sweep
  before it.
- `compromised.png` (death-card): frame dimmed, `COMPROMISED` in red with
  `PRESS R TO RESTART` beneath, the fallen player in the reddish pool below
  the text, guard up-left; DETECTION reads `HUNTED` — a corpse is not seen.

## How to feel it

Load the game and touch nothing: your torch is off, the meter is dark, and
the corridor guard walks his beat out east and back with his beam washing
faintly toward you at the turn — the meter breathes to a third and settles,
DETECTION never leaves HIDDEN. That is the spawn's promise. Now walk east
down the polished strip toward him. When his beam pool comes back down the
corridor, step into it: LIGHT jumps to EXPOSED, DETECTION climbs through
SUSPICIOUS (his torch warms to amber and sweeps) to SEEN in about a third
of a second at close range, the frame edge starts breathing red, and 0.7 s
later he opens fire. Break line of sight sideways behind a crate: the beam
goes red, he runs to where you last stood in light, and if you are gone he
stands there sweeping before walking back while the meter decays to HIDDEN.
Crouch (C) behind a cubicle partition and both his eyes and your meter
agree you are not there. Turn your torch on (F) a metre from a wall and
watch LIGHT. Fire a shot (2, click): everyone in earshot converges on the
sound and the nearest of them shouts to whoever is near him — once. Get
shot: COMPROMISED, press R, and the room, the lamps and the guards come
back as they opened. Slip out the east end untouched for GHOST.

## Findings

- The reviewers' central catch was right and structural: geometric LOS was
  doing the seeing, so the light model was decorative. `sees` (LOS ∧ signal
  > 0) is now the single gate for suspicion rise, stimulus, alert steering,
  firing and the HUD, and darkness is a mechanic again.
- The probe read the guard's own torch through the guard's fist: shadow
  rays end at a 6 cm jitter sphere around a 1.2 cm lens on a hand and a
  slide, so it flipped lit/black on alternate frames. Fixed at the probe
  with a 0.3 m spot standoff. The exact fix — a per-light carrier group the
  shadow test skips, mirroring `skipGroup` — needs the GPU `Light` struct:
  **renderer request**, only worth it if the standoff ever shows.
- Torch-on exposure is real but honest: pointed down an empty corridor your
  own beam lands 9 m away and returns ~1%; a metre off a wall it near
  triples the meter. So the torch is a decision, and it starts off —
  opening with it splashing the west wall read EXPOSED to the corridor guard
  before any input.
- The eye's floor and the HUD's HIDDEN band were different numbers (0.15 vs
  0.25), so "hidden" was still slowly visible. One constant now, and the
  accumulator runs the forgetting rate against the signal so a glow below
  the noise floor never integrates by being stared at.
- Callouts as first written relayed guard to guard, so one shot's real
  radius was the whole floor. A shout is heard once.

## Merge notes

- New: `src/game/detection.ts`, `src/game/nav.ts`, `src/ui/overlay.ts`,
  `tools/headless/scenarios/*.js`, this report. Touched: `guards.ts`,
  `player.ts`, `equipment.ts`, `main.ts`, `gauge.ts`, `probe.wgsl`,
  `post.wgsl`, and two lines of `renderer.ts` (`FrameState.seenPulse` →
  `pp[15]`, a post-pass *input*; no estimator change).
- `main.ts` conflicts with any track touching the frame loop or the hook
  table: the detection block is contiguous (`guards.update` →
  `detection.update` → `checkOutcome`) and the hooks are added lines.
- An earlier commit on this branch tracked a `node_modules` symlink; it is
  removed and `.gitignore` names the symlink form too. The trunk clone
  holds a self-referential `node_modules/node_modules` symlink from that
  history — worth deleting in the trunk, which this track does not touch.

## Review resolution

Two adversarial reviews of `805ca17` plus the then-dirty tree. Every finding
was checked against the code; verdicts are mine. Everything marked real is
fixed at HEAD and re-verified above.

| # | Finding | Verdict | Resolution |
|---|---|---|---|
| 1 | Report missing, scenarios untracked, tuning uncommitted; HEAD ≠ what was verified (both) | **Real** | Everything committed; this report written; all numbers above are from the committed HEAD's build. |
| 2 | `node_modules` absolute symlink committed (both) | **Real** | `git rm --cached node_modules`; `.gitignore` lists `node_modules` and `node_modules/` so neither form can return. |
| 3 | Scenarios assert nothing; wall-clock timebase (R1) | **Real** | `__pause(true)` + fixed-step `__renderStill(n, dtMs)` with the rAF loop stood down; every scenario returns `ok` + `failures` and `run.py` fails on `ok:false`. |
| 4 | LOS alone drives suspicion, stimulus, alert steering and firing — a zero-signal player is tracked and executed (both, HIGH) | **Real — the central defect** | `perceive()` derives `sees = hasLOS && signal > 0`. Suspicion integrates `signal − decay` and never freezes on a bare ray; `stimulus` is written only on `sees`; alert hold/fire, `aimTime`, alert→search exit and HUD `seen` all gate on it. In the dark an alert guard chases the last lit position. Locked by `dark-los.js`. |
| 5 | Idle player at spawn detected and shot; the −6 route still raked it (both) | **Real, deeper than the route** | The corridor is a sightline to the spawn and the guard's own beam lights whatever it points at, so the westward leg now ends at x = 9 (22 m, the edge of vision); `litMin` raised to the HUD's HIDDEN edge 0.25; ambient term reshaped to `litF·distF`; the accumulator forgets against the signal; and the player's torch — which lit him EXPOSED off the west wall — starts off. Locked by `spawn-afk.js` (75 s, worst suspicion 0.000). |
| 6 | Gunshot alerts beyond range: the callout chain hunts the whole level (both) | **Real** | Callout recipients are marked `calloutDone`: heard once, never relayed. `hearing.js` measures shot-only (38.5 / 41.2 m guards untouched), the one-hop set, and a placed relay line where the second hop stays silent. A shout reaching a guard just past gunshot range is the brief's intended "guards within earshot", bounded to one hop. |
| 7 | Restart leaves shot lamps dark and the OCP spent (R1) | **Real** | `Equipment.reset()` restores every OCP-disabled and shot lamp (CPU intensity, GPU intensity, fixture emissive) and refills the charge; `restart()` calls it. `fail-state.js` proves it through the probe: 0.517 → shot 0.233 → restart 0.535. |
| 8 | Probe bimodal in a beam, 13.4 ↔ 0.44 lx (R1) | **Real, attribution confirmed** | Shadow rays toward a spot light stop 0.3 m short of the lens (`SPOT_LENS_STANDOFF`), clearing the carrier's fist and slide; every `probe-check` phase holds max/min ≤ 1.07. |
| 9 | `aimTime += 1/perceptionHz` ignores the real tick (R1) | **Real (low)** | `aimTime += dtP`; the 0.7 s reaction beat is elapsed game time at any frame rate. `fail-state.js` asserts no shot before `fireReaction`. |
| 10 | `body.reported` latches on a finder killed before its alert tick (R1) | **False positive** | `checkBodies` runs inside `perceive` and `transition` runs in the same call: the finder's suspicion is raised past `alertAt` and it enters alert (and shouts) synchronously in that tick — there is no frame in which it can die between the two. |
| 11 | Crouch triad 0.75 / 0.9 / 1.05 disagree behind ~0.85 m cover (R1) | **Real** | Crouched LOS target and probe share one height, 0.85: the eye and the meter measure the same body, so cover cannot read dark to one and clear to the other. The 1.05 capsule top only matters to a guard already seeing you. |
| 12 | Post-win sim runs on: guards perceive and fire at a GHOST (R1) | **Real (low)** | The brain no longer ticks once the run is won; bodies keep animating for the beat. The R-after-win reload was already inert, as the reviewer confirmed. |
| 13 | Torch tint feeds the signal — red beam dimmer at HEAD (R2) | **Real, half of it** | The luma-hold is the fix and is kept: intensity rescales so beam *luminance* is constant across tints — measured 0.994 of calm with the beam fully red at 2.7× intensity. The reviewer's follow-on (thud → suspicious guard's beam settles on you → alert) is not tint coupling but the mechanic: a suspicious guard points his torch at the noise, and standing in it is being seen. |
| 14 | `pursue()` repaths every frame on a null path (R2, low) | **Real (latent)** | The 0.8 s repath timer paces every replan, unreachable goals included; entering alert zeroes it so the first chase paths at once. |
| 15 | Guards interpenetrate each other and the player (R2, low) | **Not addressed** | Bodies are deliberately absent from the raster and the LOS test (brief: bodies do not hide you). Cosmetic; a separation impulse is the right follow-up and out of this brief. |
| 16 | Renderer touched with no note (R2) | **Real (bookkeeping)** | Recorded above: `FrameState.seenPulse` + `pp[15]`, a post-pass input inside the brief's allowance; no estimator change. |
| 17 | Search timeout dumped the guard back into "suspicious" at once (found in my soak run, not by the reviewers) | **Real (low)** | Giving up on a searched spot clamps suspicion to the exit threshold: the guard walks back HIDDEN instead of stopping again to stare at nothing (soak: 0.12 on resume). |

## Below 100%

- The probe standoff is a symptom fix. It is measured clean (max/min ≤
  1.07 over ~90 frames across six poses), but a per-light carrier-group
  skip in the GPU `Light` struct is the mechanism-level fix and needs the
  renderer track's agreement.
- `spawn-afk` covers 75 s — two corridor laps and about 1.5 laps of the
  other three routes — not the joint period of all four loops. The safety
  argument is geometric (spawn ≥ 22 m from the corridor lane's west end,
  walls or off-cone for the rest) rather than exhaustively enumerated.
- Timings from SwiftShader are still not performance numbers; the
  reviewer's `Detection.update` cost (mean 0.05 ms, p95 0.10 ms) stands as
  the CPU-side sanity check and was not re-measured after these changes.
