# Track B4 — temporal hardening (dynamic reprojection audit)

Owner branch: `claude/temporal`, worktree `~/src/twopointfive-wt/temporal`.
Base: `claude/campaign` @ the sha you were given.

Read first: `README.md` (note "Next steps" item 2 — you will be correcting
it), `docs/campaign/STATUS.md`, `tools/headless/README.md`,
`src/shaders/reproject.wgsl` (all), `src/shaders/pathtrace.wgsl` around the
G-buffer emit (~lines 640-700: dynamic hits are transformed into the
PREVIOUS frame's box frame from `prevDynBoxes`, so `gPos.xyz` already holds
last frame's world position — the README's claim that reprojection is
static-only is stale), `src/engine/renderer.ts` `updateDynamic` /
`prevDynData` / `prevDynBuffer`, `src/game/particles.ts` (swap-remove
eviction reorders indices → a particle's "previous" box may be a different
particle), `src/game/character.ts` box packing order, and `composite.wgsl`
debug views (view 3 = variance/history length).

## Why this track exists

Temporal accumulation is what lets this tracer live at 1 spp. Someone
already did the hard part — per-box rigid previous transforms feed
reprojection for the animated character — but nobody has *verified* it
holds up in motion, the docs claim the opposite, and there are known holes
(particles). This is a measurement-and-hardening track, not a build track.
Your output is evidence and small surgical fixes.

## Deliverables

1. **Measure history survival on the moving character.** Build a repeatable
   headless probe: `__renderMotion(n)` at ~384×240 with the debug view set
   to variance/history, screenshot, Read it, and quantify (add a small
   readback if you must — e.g. average `momentsHist` history-length over the
   character's screen bounds via a debug hook `__historyStats()` you add).
   Do the same for a static wall as the control. State the numbers.
2. **If history sheds on limbs**, find why (`validTap`'s normal threshold
   0.90 ≈ 26° vs a forearm rotating at 60 Hz; depth test; the joint-overlap
   sub-pixel error) and propose+implement the minimal fix — e.g. a normal
   test relaxation gated on the tap being on dynamic geometry, or history
   length caps for high-motion pixels — WITHOUT reintroducing ghosting.
   Every threshold change ships with before/after history numbers and a
   moving-character screenshot pair. If it does not shed, say so with
   evidence and change nothing.
3. **Particles**: write current-as-previous for the particle box range in
   `updateDynamic` (their identities reshuffle under swap-remove, so their
   reprojection is currently against the wrong particle) unless you find a
   cheaper identity-preserving fix. Justify in a comment (why, not what).
4. **Docs**: fix README "Next steps" item 2 and the reproject.wgsl header
   comment to describe what the code actually does (per-box rigid
   reprojection for dynamic geometry via `prevDynBoxes` → `gPos`), and
   record the packing-order invariant (`prevDynBoxes[i]` is the same box
   only while ordering is stable: player group 0, guards in fixed order,
   particles last) next to the code that depends on it.
5. **Probe stance** (small, adjacent): the light probe sits at +1.15 m
   regardless of crouch — make it follow stance (crouched ≈ 0.75) so the
   LIGHT gauge matches what a crouched player is actually exposed to. Note
   in the report if Track B3 (detection) collides — coordinate via the
   report, don't edit their files.

## Verification you must do

- `npm run typecheck` + `npm run build` clean after each step.
- Headless runs (Bash sandbox off — `dangerouslyDisableSandbox: true`,
  reason "chromium needs its own namespaces") at ~384×240; every claim in
  the report is backed by a number from a scenario or a Read screenshot
  described in words. State what the tap/history evidence can and cannot
  prove (SwiftShader is bit-different from Metal but the reprojection logic
  is deterministic — say which class of bug your check would miss).
- `__compareToReference` is NOT expected to move (you are not touching the
  estimator); if you change `validTap`, run it at small resolution before
  and after and quote both.

## Rules

- Work only inside your worktree; commit to `claude/temporal` at every
  green step, at least hourly; never `git stash`; never rewrite history.
- Yours: `reproject.wgsl`, the `updateDynamic` particle-range fix in
  `renderer.ts`, the probe placement line in `main.ts`, README/comment
  fixes, debug hooks. Not yours: the estimator (pathtrace radiance math),
  guards/detection, volumetrics, radiosity. If a fix wants to leave your
  lane, write it up as a request in the report instead.
- Comments: short, why-not-what, no edit narration.

## Report

`docs/campaign/tracks/B4-temporal.md` on your branch: findings first
(measured history numbers with the exact scenario), then changes and their
before/after, then Findings-out-of-scope, then merge notes. Final message =
same content + branch HEAD sha + anything below 100% with the reason.
