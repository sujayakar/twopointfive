# Track B3 — light-based detection & guard AI

Branch `claude/detection`, forked from `claude/campaign` @ `230295a`.
Everything below is measured on the headless SwiftShader harness at
384×240 (`tools/headless/run.py`, sandbox off) with the game clock owned
by the scenario: `__pause(true)` stands the rAF loop down and every
`__renderStill(n, 50)` frame is exactly 50 ms of game time, so the numbers
are game seconds, not this machine.

## What changed and why

- `detection.ts` (new): per-guard perception at 15 Hz — the view cone is the
  torch beam axis, LOS is `raycaster.blocked` on static geometry, and the
  signal is `litF·distF·seenRate + beamF·beamRate` where `lit` is the same
  probe value the LIGHT gauge shows. Eyes are on the player only while the
  signal is non-zero (`sees`); a clear ray in the dark carries nothing.
  Suspicion accumulator → patrol / suspicious / alert / search; hearing
  (gunshot broadcast, sprint footsteps, takedown thud, one-hop callout);
  bodies alert whoever's beam reaches them; seeded fire cadence and spread;
  a hit ends the run. One `detectionTuning` object on the tweak panel.
- `guards.ts`: brain state on the guard, mode API (`route`/`hold`/`follow`),
  yaw-eased locomotion reused for nav paths, per-guard torch tint that
  reddens with state at constant luminance, `reset()`; corridor route
  shortened so its beam never reaches the spawn.
- `nav.ts` (new): 0.5 m XZ raster from colliders + tall boxes, A* with
  string-pulling, deterministic. Excursions only; patrols stay authored.
- `probe.wgsl`: the player's own flashlight (direct + one backscatter tap)
  counts; shadow rays to a spot light stand 0.3 m off the lens so the
  carrier's fist stops occluding his own beam. `post.wgsl`: red edge pulse
  from `seenPulse` (one new `FrameState` field, `pp[15]`).
- `player.ts`: `kill()` / `reset()`, stance-following LOS target; `main.ts`:
  wiring, fail state (COMPROMISED + R restart, which also restores shot
  lamps and the OCP), GHOST / CLEAN success card, `__pause` and fixed-step
  `__renderStill` hooks; `overlay.ts`, `gauge.ts`: end card, objective line,
  DETECTION meter (HIDDEN / SUSPICIOUS / SEEN / HUNTED).

## Verification

`npm run typecheck` and `npm run build` clean. Nine scenario scripts under
`tools/headless/scenarios/`, each returning numbers plus an `ok` verdict the
runner enforces (`run.py` fails on `ok: false`). All green at HEAD:

**beam-detect** — player 4.0 m down guard 0's beam axis, feet frozen. LOS ✓,
inBeam ✓, signal 1.67 → 2.65 as suspicion feeds the torch tint. Suspicion
0 → 1 monotone; suspicious at t = 0.20 s, **alert at t = 0.30 s** of game
time. HUD `DETECTION | SEEN`, `LIGHT | EXPOSED`, probe illuminance 13.1.

**dark-los** — the thesis check both reviewers broke. Guard pinned facing
the corridor, player 16 m down its cone past beamRange, torch off, `litMin`
forced to 1 so the visual signal is exactly 0 while the ray is clear: over
0.5 s idle `hasLOS true, signal 0, sees false, suspicion 0, stimulus null`.
Seed suspicion 0.4 + stimulus at the player's spot (a heard noise): state →
suspicious and suspicion **decays** with the ray still clear (0.4 → 0.31 in
1.5 s). Player moves 3 m: stimulus stays on the noise. Force suspicion 0.9
(what a gunshot or a colleague's shout does): guard goes alert in `nav`
mode chasing the last-known point — **0 rounds fired, player alive** after
3 s of a clear ray to a target he has no light on.

**los-crouch** — guard 1 pinned facing a 1.35 m cubicle panel, player 2.9 m
behind it. Raw raycast: standing target (1.5 m) clear, crouched target
(0.85 m) blocked. Crouched, 16 ticks: LOS false, sees false, suspicion
stays 0.000 with the meter itself at 0.364 (dim). Standing: LOS true, beam
true, signal 2.29, suspicious at 0.15 s, alert at 0.35 s.

**hearing** — three phases through the real trigger (mousedown → shot):
A) callouts silenced, shot at (−25.2, −16.2): guards at 30.7 / 11.9 /
13.3 m go alert with stimulus on the shot; the server guard at **41.2 m
stays patrol, suspicion 0** (gunshotRange 34). B) callouts on, natural
layout: alerted set = shot ∪ one shout hop (all four here). C) hand-placed
relay line, shot at 29 m from g0, g1 16 m from g0, g2/g3 within 15 m of g1
only: g0 alert (shot), g1 alert (g0's shout), **g2 and g3 patrol,
suspicion 0** — a shout is one hop, never a chain.

**probe-check** — 4 m down the beam, frozen: TODO

**fail-state** — TODO

**death-card** — TODO

**soak** — TODO

**spawn-afk** — TODO

Screenshots at 384×240, read back: TODO

## How to feel it

Load the game and stand still: the corridor guard walks his beat to the
east and back and never looks your way — HIDDEN, meter dark. Now walk east
down the polished strip toward him with the flashlight **off** (F). When
his beam pool comes back down the corridor toward you, step into it: LIGHT
jumps to EXPOSED, DETECTION climbs through SUSPICIOUS (his torch goes
amber and starts sweeping) to SEEN in about a third of a second at close
range, the frame edge starts breathing red, and 0.7 s later he opens fire.
Break line of sight sideways behind a crate: his beam turns red, he runs to
where you were, and if he loses you he stands there sweeping the torch
before walking back to his route (HUNTED, then decaying to HIDDEN). Crouch
(C) behind a cubicle partition while he sweeps and the meter and his eyes
agree you are not there. Fire a shot (2, click) and every guard in earshot
converges on the sound, plus whoever the nearest of them shouts to. Get
shot: COMPROMISED, press R — the room, the lamps and the guards come back
exactly as they opened. Slip out the east end untouched for GHOST.

## Findings

- The probe read the guard's own torch through the guard's fist: shadow rays
  end at a 6 cm jitter sphere around a 1.2 cm lens sitting on a hand and a
  pistol slide, so it flipped between fully lit and near-black on
  alternate frames while a player stood dead centre in a beam. Fixed at the
  probe (0.3 m spot standoff); the general fix — a per-light "carrier group"
  the shadow test skips, mirroring `skipGroup` — needs the GPU `Light`
  struct, which is renderer territory. **Renderer request** if the standoff
  ever shows an artefact.
- Torch-on-raises-exposure is real but conditional: pointed down an empty
  corridor your own beam lands 9 m away and returns nothing measurable;
  facing a wall a metre off it lifts the meter (numbers above). One
  backscatter tap under-counts wide spill by design.
- The light-meter's HIDDEN band and the eye's floor were different numbers
  (0.25 vs 0.15), so "hidden" on the HUD was still slowly visible. They are
  one constant now (`litMin = 0.25`).
- Callouts as first written relayed guard to guard, so one shot's real radius
  was the whole level. A shout is now heard once.

## Merge notes

- New files: `src/game/detection.ts`, `src/game/nav.ts`, `src/ui/overlay.ts`,
  `tools/headless/scenarios/*.js`, this report. Touched: `guards.ts`,
  `player.ts`, `main.ts`, `gauge.ts`, `probe.wgsl`, `post.wgsl`, and two
  lines of `renderer.ts` (`FrameState.seenPulse` → `pp[15]`, a post-pass
  *input*, no estimator change).
- `main.ts` will conflict with any track touching the frame loop or the debug
  hook table; the detection block is contiguous (guards.update →
  detection.update → checkOutcome) and the hooks are three added lines.
- Do not merge a `node_modules` symlink: an earlier commit on this branch
  tracked one; it is deleted here and `.gitignore` now names the symlink
  form too. The trunk clone currently holds a self-referential
  `node_modules/node_modules` symlink from that history — worth deleting
  there (not this track's directory to touch).

## Review resolution

Two adversarial reviews of `805ca17` + the dirty tree. Verdicts are mine after
reading the code; every real one is fixed at HEAD.

| # | Finding | Verdict | Resolution |
|---|---|---|---|
| 1 | Report missing, scenarios untracked, tuning uncommitted; HEAD ≠ what was verified | **Real** | Committed everything (`d1c3433`), wrote this report, and every number here is from the committed HEAD's build. |
| 2 | `node_modules` absolute symlink committed | **Real** | `git rm --cached node_modules`; `.gitignore` now lists both `node_modules` and `node_modules/` so the symlink form cannot be re-added. |
| 3 | Scenarios assert nothing; wall-clock timebase | **Real** | `__pause(true)` + `__renderStill(n, dtMs)` fixed-step clock (rAF stands down); every scenario returns `ok` + `failures`, and `run.py` exits non-zero on `ok: false`. All numbers in this report are game-time. |
| 4 | Geometric LOS alone drives suspicion rise, stimulus, alert steering and firing — a zero-signal player is tracked and shot (both reviewers' HIGH) | **Real, the central defect** | `perceive()` now derives `sees = hasLOS && signal > 0`; suspicion rises only on `sees` and *decays otherwise even with a clear ray*; `stimulus` is written only on `sees`; alert steer/fire, `aimTime`, alert→search exit and the HUD's `seen` all gate on `sees`. In the dark an alert guard chases the last lit position and finds you with his torch, not with a hidden ray. Locked by `dark-los.js`. |
| 5 | Idle player at spawn detected and executed with no input (HEAD route −11.5; dirty tree −6 still raked by the beam) | **Real** | Three coupled fixes: corridor route's west end moved to x = 0 (13 m from spawn, outside `beamRange`); `litMin` raised to the HUD's HIDDEN edge (0.25); ambient term reshaped to `litF·distF` (light and closeness multiply, no 0.4 range floor). Locked by `spawn-afk.js`: TODO. |
| 6 | Gunshot "within range and not beyond" false — callouts chain across the map (guard at 41.3 m alert) | **Real** | Callout recipients are marked `calloutDone`, so a shout is heard once and never relayed. `hearing.js` measures shot-only (41.2 m guard untouched), one-hop, and a hand-placed relay line where the second hop stays silent. A colleague's shout reaching a guard just past gunshot range is intended (brief: "broadcast alert to guards within earshot") and now bounded to one hop. |
| 7 | Restart leaves shot-out lamps dark and the OCP spent | **Real** | `Equipment.reset()` restores every OCP-disabled and shot lamp (CPU intensity, GPU intensity, fixture emissive) and refills the charge; `restart()` calls it. Verified in `fail-state.js` by reading the probe under a lamp before/after (TODO numbers). |
| 8 | Light probe bimodal (13.4 ↔ 0.44 lx) standing motionless in a beam | **Real (attribution confirmed)** | The shadow ray toward a spot light now stops 0.3 m short of the lens (`SPOT_LENS_STANDOFF`), clearing the carrier's hand and slide; `probe-check.js` frame-to-frame max/min went from 33.7× (one lagged frame) / bimodal to ≤1.06× steady across every phase. |
| 9 | `aimTime += 1/perceptionHz` regardless of the real tick | **Real (low)** | `aimTime += dtP`; the 0.7 s reaction beat is now in elapsed game time whatever the frame rate. `fail-state.js` asserts the first shot is not before `fireReaction`. |
| 10 | `body.reported` latches on a finder that dies before its alert tick | **False positive** | `checkBodies` runs inside `perceive`, and `transition` runs in the same call: the finder's suspicion is raised past `alertAt` and it enters alert (and shouts) synchronously in that tick. There is no window in which it can be killed between the two. |
| 11 | Crouch triad: probe 0.75 / target 0.9 / capsule 1.05 disagree behind ~0.8–0.9 m cover | **Real** | LOS target and probe now share one crouch height (0.85): the eye and the meter measure the same body, so cover cannot read dark to one and clear to the other. The 1.05 capsule top only matters to a guard already firing, i.e. already seeing. |
| 12 | Post-win sim keeps running: guards perceive and fire at a GHOST | **Real (low)** | The brain no longer ticks once the run is won (`ended === "win"`); bodies keep animating. R-after-win reload was already inert (confirmed by the reviewer). |
| 13 | Torch tint feeds the detection signal (red beam dimmer at HEAD; luma-hold brighter) | **Real, half of it** | The luma-hold is kept and is the fix: intensity is rescaled so the beam's *luminance* is constant across tints — `probe-check.js` measures the meter at TODO of calm when the beam is fully red. The reviewer's second scenario (thud → suspicious guard's beam settles on you → alert) is not tint coupling but the mechanic itself: a suspicious guard aims his torch at the noise, and standing in a torch is being seen. |
| 14 | `pursue()` repaths every frame when `findPath` returns null | **Real (latent)** | The 0.8 s repath timer now paces every replan including the unreachable-goal path; entering alert zeroes it so the first chase still paths immediately. |
| 15 | Guards interpenetrate each other and the player | Not addressed | Bodies are deliberately absent from the nav raster and the LOS test (the brief: bodies do not hide you). Cosmetic; a separation impulse is the right follow-up, out of scope here. |
| 16 | Renderer touched without a note | **Real (bookkeeping)** | Recorded above: `FrameState.seenPulse` + `pp[15]`, a post-pass input, within the brief's "post inputs" allowance. No estimator change. |
