# Track B3 — light-based detection & guard AI

Owner branch: `claude/detection`, worktree `~/src/twopointfive-wt/detection`.
Base: `claude/campaign` @ the sha you were given.

Read first: `README.md`, `docs/campaign/STATUS.md`, `tools/headless/README.md`,
then `src/game/guards.ts` (all of it — the "deliberately not AI" comments are
the design you are graduating past), `src/game/visibility.ts`,
`src/game/raycast.ts`, `src/game/player.ts`, `src/game/character.ts`
(shared-rig hazard: a pose is only valid between one Character's update and
the next — `repose()` before reading bones), `src/shaders/probe.wgsl` and
its driver in `src/engine/renderer.ts` (`setProbes`, readback) and consumer
in `src/main.ts` (`visibility.update`), `src/ui/gauge.ts` + `panel.ts`, and
`src/scene/level.ts` for the collider list and layout.

## Why this track exists

This is the demo's thesis made playable: **detection is the lighting you
are looking at**, not a separate system that disagrees with it. The tracer
already computes how lit the player is (the probe pass → LIGHT gauge);
guards already carry real spotlights whose beams the renderer traces. Your
job is to make those beams *be* the guards' eyes and the light meter *be*
your danger, and to give the loop a fail state so it means something.

## What to build

### Perception
- **Vision** per guard, evaluated at a modest CPU cadence (10–20 Hz is
  plenty): the player is seen when inside the guard's view cone (align the
  cone with the torch beam direction — the beam IS the eye), within range,
  with clear line of sight (`raycaster.blocked` eye→target with a fixture
  margin; static geometry only, which is correct — bodies don't hide you),
  AND the detection signal is strong enough. Signal = a function of the
  player's smoothed light level (`visibility.level`, already 0..1), distance,
  whether the player is inside this guard's *own* beam (a beam hit at close
  range detects fast regardless of ambient), and stance (crouch behind a
  1.35 m partition should break LOS: use a lower target height when
  crouched — resolve today's mismatch where the probe sits at +1.15 whether
  or not the player crouches).
- **Hearing** keeps today's gunshot broadcast, plus: sprinting footsteps in a
  radius, takedown thud in a small radius.
- **Bodies**: a guard whose LOS ray reaches a dead, un-carried body goes to
  alert with that body as the search point (bodies never leave the dynamic
  box list, so the data is free).
- Make the probe honest for gameplay: include the player's own flashlight
  (uniform, not in `lights[]` — probe.wgsl already flags this "happy
  accident") and read the radiosity indirect at the player's floor patch if
  cheap. Torch-on should raise your own exposure. State what you did and
  why in the report.

### Suspicion → states
Per guard, a suspicion accumulator in [0,1] driven by the signal, decaying
slowly; states `patrol → suspicious → alert → search → patrol`. Suspicious:
stop, aim the torch at the stimulus, sweep. Alert: LOS + weapon = fire (wake
the dormant `Guard.fire()`, give it spread and a cadence; hits kill the
player → fail state), no LOS = move to last-known position. Search: visit
last-known, do a sweep pattern, time out to patrol via the route's nearest
point. Guards call out (broadcast alert to guards within earshot) when they
go alert.

### Navigation (needed for alert/search — there is none today)
A coarse XZ grid (0.5 m cells over the level bounds ±26 × ±18) rasterized
from `level.colliders` plus the static box footprints tall enough to block
(desks, crates, columns), a plain A*, and path following that reuses the
existing yaw-easing/speed-scaling locomotion. Guards never leave the floor
plate; no doors exist (gaps in walls). Keep it deterministic and small.
Patrols stay authored polylines; the grid is only for excursions.

### The lighting tells the story
- An alert guard's torch light colour warms toward amber/red (per-light RGB
  exists; the *volumetric* shaft tint is currently one shared uniform — do
  not fight that here, another track is replacing the volumetric channel;
  the beam colour on surfaces is what changes).
- HUD: a detection element in the `gauge.ts` pattern — the strongest
  suspicion across guards, with the guard state as a label
  (HIDDEN / SUSPICIOUS / SEEN / HUNTED). Optional: a subtle screen-edge
  vignette pulse when SEEN (compose it in `post.wgsl` from a uniform, not
  DOM).
- A fail state: shot by a guard → fade + "COMPROMISED" + press R to restart
  (respawn the level state cleanly — dead guards, ammo, position). And a
  quiet success beat: all guards down or exfil zone reached → "GHOST" /
  "CLEAN" rating readout. Keep it two sentences of UI, not a menu system.

### Tuning surface
Everything numeric (FOV, ranges, rise/fall rates, band thresholds, hearing
radii, fire cadence) lives in one exported `detectionTuning` object with a
tweak-panel group, exactly like `movementTuning` in player.ts.

## Verification you must do

- `npm run typecheck`, `npm run build` clean.
- Headless (Bash sandbox off — `dangerouslyDisableSandbox: true`, reason
  "chromium needs its own namespaces"): a scenario script that places the
  player in a guard's beam and asserts (via new `__guards` /
  `__detection` debug fields you expose) that suspicion rises and the guard
  transitions to alert; another with the player crouched behind a partition
  in the beam's direction asserting LOS is blocked; another confirming a
  gunshot alerts guards within range and not beyond. Return numbers, not
  vibes. Screenshot the SEEN state at 384×240 and Read it: describe the
  amber beam and HUD state.
- Determinism: `npm run build` output must not depend on Date/Math.random
  outside seeded paths (the level RNG is seeded — keep new randomness on a
  seeded stream so scenario asserts are stable).

## Rules

- Work only inside your worktree. Never touch `~/src/twopointfive` or other
  worktrees. Commit to `claude/detection` at every green step, at least
  hourly. Never `git stash`, never rewrite history.
- Do not modify the tracer's estimator (pathtrace/radiosity/reproject WGSL)
  — other tracks own those. `probe.wgsl` is yours; `post.wgsl` is yours for
  the vignette only; new HUD/DOM is yours; `guards.ts`, `player.ts` (target
  heights, noise events), `main.ts` wiring are yours. If you need a renderer
  change beyond probe/post inputs, write it in the report as a request
  rather than making it.
- Comments: short, why-not-what, no edit narration. Follow existing idiom.

## Report

`docs/campaign/tracks/B3-detection.md` on your branch: what changed and
why (≤20 lines), verification evidence with the actual numbers from your
scenarios and the screenshot descriptions, a "How to feel it" section (30
seconds of what to do in-game to see the system working), Findings, merge
notes. Final message = same content + branch HEAD sha + anything below
100% with the reason.
