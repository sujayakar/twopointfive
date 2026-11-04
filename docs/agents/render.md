# Renderer

One of the area files CLAUDE.md indexes. Everything below was moved here word for word from CLAUDE.md on 2026-10-13, in its old order;
"the bullet above / below" in a bullet means its neighbour there (each block says which lines it was), and a bullet named by its bold
title may now live in another file of this folder: `grep -l '<title>' docs/agents/*.md`. New bullets for this area go at the foot of this file.

<!-- was CLAUDE.md lines 470–505 -->
## Judging renderer output

Whole-frame captures downscaled into a chat hide exactly the defects that matter (penumbra stipple, ragged
iso-contours at a spot edge, probe-sized blotches, sparkle near small emitters, discs around dim lights). Crop
200–400 px tiles around shadow edges / light-pool boundaries / emitters, upscale 3–4×, compare before/after tiles,
and look at the debug views (`engine.settings.debugView`: direct-only, indirect-only) as well as the composite.
`sips -c H W --cropOffset Y X` + `sips -Z` does it on macOS. For "lossless" claims use the pixel A/B harness:
`?paused` boot, identical stepping, `__png` both variants, diff (max/mean/>2/>8 counts) against a same-variant
noise floor captured the same way; when A/B-ing smoke shading freeze the field with `smoke.params.frozen = true` (holds
the field AND still renders it — `enabled=false` renders no smoke at all, so any smoke A/B passes vacuously).
Determinism checklist for captures (learned the hard way): zero every character's `anim.phase` (it is seeded with
`Math.random()`; the idle / breathing / weight-shift clocks are desynced from the character id — deterministic, leave them), park the guards AND their torches somewhere they light nothing (or disable the torches), seed
`Math.random` if emitters are involved, never let the rAF loop run between boot and capture (`?paused` + a
screenshot to give the pane a size, then only `__runFrames`), and for views with MEN ON SCREEN call `game.restartEncounter()` right
after seeding `Math.random` and before staging (the guards' construction-time dice — glance clocks, waits — were drawn unseeded at boot,
and a staged man's head otherwise sits differently boot to boot). With that, two boots are bit-identical in every view;
`tools/qa/rig-identity.js` (`__idrun`) is the recipe end to end (corridor / lobby / cubicles stills with 2–4 men, hashable across boots at every preset).
**ONE CONFIG PER FRESH BOOT** for every identity pass and every look A/B (learned again on the smoke lab): two stagings inside one page never compare — the cubicles'
flickering fixtures, the haze clock, the TAAU phase, the storm and the wind all key on the game / render clocks and the frame index, which the first run has advanced by
the time the second starts — so navigate afresh (`?paused`), run the SAME call sequence from boot for each build / setting (cross-build claims: main served BESIDE the lane on
its own port, both driven identically) and compare the hashes / PNGs left on the sink by name (`__rf.identity / pngs`, `__idrun` are that shape).
Numbers and hashes recorded before 08-12 reproduce only with that day's new defaults off (`?nogatherstrata&nomotioncalm`, and `?noiq` for anything from before the image-quality
program); harnesses that hash against main served beside stay valid, since both sides boot the same defaults.

## GPU timing — known traps

`engine.timer` timestamps are per pass. **Render-pass** timestamps (gbuffer/composite/post/fxaa) on Metal span the
wait for the drawable and read ≈ one frame — ignore them (the HUD lists compute passes only). (2026-11-04: of the four, the g-buffer's
does NOT read so on the one Mac whose takes were looked at: on an M4 Max it moved with g-buffer work and not with the frame's length, and
read 0.33 ms on a page with 21 ms of waiting a frame, where `post` and `fxaa` read 5. So the benchmark's card gives it its own column,
apart from its GPU number: `docs/agents/qa.md`, what a line of the card prices. The totals here still leave all four out.) Scripted
`__runFrames` stepping returns stale timer values UNLESS each frame is followed by `await device.queue.onSubmittedWorkDone()`
+ a MessageChannel yield: that serializes frames (no cross-frame overlap inflating passes) and lets the readbacks resolve
even with the pane hidden — two 30-frame baselines then agree to ~0.02 ms and match the live HUD (this is how the trade-off
options were timed). Without it, hidden-pane numbers are garbage. Interleave A/B/A/B either way. Reference numbers (M4 Max, high, calm office, renderer v3): ≈ 10–11 ms union at 1368×770 with the
08-12 defaults (sharp direct + GBAA, motion-calm, gatherStrata, near mesh shadows at high — docs/internal/trace-perf.md §0 / render-defects-0812.md carry those takes; ≈ 8–10 ms before them) — see docs/perf/v3-gate-phase4.json and
docs/internal/p2a-restir.md for per-pass tables (the pre-v3 frame was ~20 ms: direct ≈7, softblur ≈3, gather ≈6, cascades ≈2, volumetrics ≈1). **The GPU is SHARED with every other lane's page** (and the
maintainer's own tab): a take that overlaps someone else's measurement is worthless — `sh tools/qa/gpu-quiet.sh` waits for `ioreg` Device Utilization to sit under 8 % before a take, rounds are
interleaved and filtered to their quiet minimum, and a row whose p75 sits far above its median is quoted as contended, never as a result.

<!-- was CLAUDE.md lines 2403–2406 -->
- **Markings = object legibility** — palette-indexed bitmaps on box faces, packed in the tail of `boxMat` (no texture); `settings.markings` default ON, `?nomarkings` (+ `?level=relief` = main's frame), `__mark`; emissive markings are LIGHTS; the art rules (flat colour per cell, ≤ 3 entries, no gradients, no baked lighting) — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **Materials, RT reflections, the detail layer** (renderer v3 phase 4) — every box carries a finish (`Box.rough / metal / aniso`), `reflect.wgsl` traces under `settings.rtReflect` (`?nortreflect`, `frame.rtCfg.w` 0 = off), the detail layer under `settings.detail` (`?detail=0‥1`; art rule: variation from geometry, never a painted texture); `__rt`, `__hotReflect` — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **The ultra-only extras** (renderer v3 phase 5) — `settings.giResample` / `rtMeshHits` / `rtHitReuse`, only the ultra preset raises them, `?nop5` vetoes; `__gi`, `__hotGather`; TRAP kept here: `reflect_p5.wgsl` is a separate program from `reflect.wgsl` on purpose (main's text) — keep the two files in step — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **Character proxies = the SDF character rig** — every mannequin's 15 shadow / reflection proxies are round cones fitted at load (`render/sdfrig.ts`, `shaders/sdfrig.wgsl`), no switch; extras in `frame.rigCfg` (`settings.charSelfShadow` default ON, `?noselfshadow`; `?sdfsmooth` / `?charsoft` / `?charsoftmarch` experiments); `__sdfrig`, `__rigTrace` — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.

<!-- was CLAUDE.md lines 2658–2728 -->
- **Effects numbers** are tuned by the user in the viewer and pasted back; treat `effects.ts` / `RIG` values as
  authored data — change them only from viewer readouts or explicit notes. (`effects.ts tracerFx` — tracer speed / shutter /
  thickness / emissive / head-light intensity + range / ember / pool caps — ships UNTUNED: viewer Guard ▸ carbine: tracers (K),
  or Tab ▸ Effects ▸ tracer rows in the corridor at night, then paste back.)
- **No dollhouse wall cutaway** in the game — it was removed by user decision (they rotate the camera instead — Ctrl + mouse orbits and Ctrl + wheel zooms now, host/loop.ts; Q / E are the wall hold and the holster);
  don't bring it back. The `Cutaway` box flag + `cutHeight` primitive survives only for the viewer stage's back wall.
  Debug/sandbox actions are debugger (tweak panel) buttons, not hotkeys.
- **Quality**: `ui/quality.ts` presets (ultra / high / medium / low / verylow) set resolution scale + internal-height cap, beam steps,
  cache grid, smoke iterations, the direct-light ray budget (8/4/3 · 8/4/3 · 6/3/3 · 4/2/2 · 1/1/2), `directHistory` (0.5; high and verylow 0.7)
  AND the lossy trade-offs below (ultra none · high checker + third-res gather + dimRays · medium/low/verylow those + quarter-res beams, with
  staggered cascade refresh `rcPeriods` in place of `rcHalfRate`; low/verylow with `dimRaysBelow` 0.02) AND the final gather's relative firefly bound `settings.giFireflyK` (4 on high‥verylow; 0 = off on ultra:
  the bound is a one-sided clamp — it cut the boil round a torch on a wall by ~40 % but costs up to a quarter of the bounce within ~0.6 m
  of an intense hotspot — so the reference preset runs without it; Tab ▸ Indirect light slider for A/B) — applying a preset writes those
  settings exactly as the menu rows do; a row flipped by hand sticks until the next preset change. `auto` first CALIBRATES behind the start card (a couple of seconds of GPU compute-pass timings → a per-pass
  cost model, `COST`/`REF`/`FACTOR` in quality.ts: per-pixel and fixed factors per preset → preset AND scale in one step, remembered
  per machine; ladder verylow‥high, never ultra; 'auto' picked later on the pause card / Tab runs the SAME path — `Quality.startAuto`:
  from this machine's remembered pick + scale, re-measured behind that card, the card's note refreshed when it settles), then a runtime ladder trims (proportional scale steps toward LADDER.aimFps 50, cut below 45 / restore above 58, ≥ 2 s apart on a 0.05 grid, preset drop
  once a real cut is off, floor SCALE_FLOOR 0.5). THOSE ARE THE 'AS NOW' BANDS: since 10-10 the ladder's default is HOLD 60 (`settings.ladderAim` 60, the maintainer's pick; `?aim=asnow` or `?aim=0` = the bands above for the visit, `?aim=90|120|display` other targets, Tab ▸ Frame ▸ 'ladder: frame rate to hold'; `ui/quality.ts HOLD / ladderBands`): the same climb, but two 1-s windows under ~58 fps with the GPU the limit cut the scale straight to what the cost model says holds 60 (to the floor, then the preset under auto), and pixels come back on measured GPU room; a page something else paces is left alone (as now: `settings.ladderPaced`, `?ladderpaced=0`; under an aim, since 10-14: `settings.ladderHeld`, default on, `?ladderheld=0` or Tab ▸ Frame ▸ 'ladder: an aim leaves a held page alone' = an aim as it was. The maintainer's Mac on a low battery, 10-14: the browser held the page at 30 fps, the GPU clocked down and read ~1.5× longer, and hold 60 stepped High from 770 to 600 rows for a rate that stayed 30. What cut it (read off the code and the replay, not the budget rule first blamed): the union at High's start read 13.9 ms on the slow clock, over the 13.33 ms back-off band, so the `hot` rule took a step every two windows, and the strict restore (the next step priced under 12.0 ms) never gave one back. On, the as-now paced check runs under an aim too: (a) a slow window is PROVEN paced when the union fits the aim's frame less HOLD.pacedMs (10.67 ms at 60) and the frame is more than those 6 ms over it; (b) slow, not GPU-bound, not proven, no pace known: over the back-off band the band's own step goes first as ever (and is judged like any cut); inside it, a judged cut to the highest scale the cost model prices under that floor (none where the scale floor cannot reach it: a small window, a union far over; a step further if it lands short). Proven there, the frame decides: no shorter = the pixels straight back to where the chain began, the failed-rung marks it left restored, that rate remembered as the page's pace; shorter by PACED.probeGain (15 %: a missed vsync bought back is a whole refresh, less is the rate's own wobble) = the cut stays and the level it left is a failed rung; no step up while a cut waits for its verdict; (c) a cut that proves nothing by the end of its 12 s trial goes back to where it started with no pace learnt, the next held off 60 s, doubling; after a judged take-back the next cut comes no sooner than the re-test would; (d) at the pace: no cut for the union's size, no `hot` step, no preset starved, and a scale under the tier's start comes back a grid step at a time on the frame's own room, each step judged (the frame grew = the step off and the pace forgotten); (e) the pace is dropped for a re-test after PACED.recheckSecs (120 s, doubling), which is those same judged cuts again (about 10 s under the tier's start). Check: `ladder-replay.ts --selftest`, the 'page something else holds' block (sixteen cases: held five minutes, the hold lifting, arriving cut, a fight on a held page, a fixed 60 Hz display missing vsync under and over the band, real load, the re-test, the marks, four never-held machines with other overheads, auto, a cut that tells nothing, a machine the cost model is wrong about, the failed rung after a cut that helped, a CPU-bound frame that wobbles, nothing holding); `LADDERHELD_BREAK=1` in front must FAIL eight of its lines (exit 1). Measured 10-14, headless only, after two reviews (the second found a never-held fixed 60 Hz machine left 25 % short of pixels, a cut / step up / cut loop where the model is wrong, a sawtooth after a cut that helped, and cuts repeating on a wobbling rate: all four mended and now cases above): off = the tree before it move for move (600 fuzz machines, as now either way and an aim with it off, four sets: no outside holds / real ones × overhead inside 6 ms / up to 9); on, machines nothing ever held spend the same seconds under 58 fps with overhead inside 6 ms (within 0.1 s of 400) and fewer past it (a fixed display: 116 → 105 of 400, the judged cut finds a missed vsync the old rule let be); with holds coming and going, about 0.5 s more per machine under 58 fps outside the held spells, fixed 60 Hz +4.6 s of 197 (the limit below). 84 steadily held machines whose GPU time follows the pixels more or less than the model says: 26 keep more pixels, 50 the same, 8 lose under 0.015 of mean scale (the trial dips). His take 5 replayed (`--aim 60 --sweeps`, a sweep take, so read the rows only): mean 573 rows off, 730 on. His two F9 takes: identical moves off and on. The real page on the software adapter at his window size, fed held windows through its own ladder (`Quality.simulate`): ×0.75 at 3 s, ×0.70 at 6 s, ×0.55 at 9 s, ×0.77 again at 12 s and for the rest of two minutes; `?ladderheld=0` ends ×0.70. KNOWN LIMITS: a fight that starts on a held page before its pace is known, or a re-test inside one, is cut as before until it is over (the floor cannot bring 23 ms under 10.7); where the model says the scale floor cannot reach the proof's floor, nothing changes; a hold lifting mid-fight on a fixed 60 Hz display, with no fast second between, waits for the re-test; a hold that lifts and comes back inside the hold-off sits at ×0.70 as before until the next judged cut; a frame rate that wobbles 15 % a second by itself can read as a cut that helped (one step under the tier's start then stays held off); everything rests on HOLD.pacedMs = 6 as the frame's overhead past the union. The replay tool's own held-frame test (`heldOut`) stays apart from `holdProof` on purpose: it re-prices a RECORDED frame and knows the take's display rate, which the ladder does not. Never seen on a GPU or a held browser). `bun run tools/qa/ladder-replay.ts --selftest | <F9 take> --check | --aim 60` = the ladder replayed headless over a recording; quality-plan.ts's ladder checks pin as-now. Since 08-12 the ladder also CLIMBS on measured headroom (`settings.ladderClimb`, default on; `capClimb` off): union-driven,
  0.05 a rung up to ×0.95 (1.0 would drop sharp direct), then gather ⅓ → ½ and beams ¼ → ½, backing off newest-first with doubling holds; 'cruising' is judged from
  the GPU union priced at the next preset by the cost model (`Quality.hopRatio`), no longer from an fps threshold a 60 Hz display could never reach; `?quality=` holds
  the climb off, and `?adapt=0` (2026-11-04, `ui/quality.ts visit`, the one reader of the address's two ladder words, called at boot, by Reset and by the viewer) opens the
  visit with NOTHING ADAPTING: the row 'Hold frame rate' off, and where no tier is named the tier the page stands on held at its own start size, unmeasured,
  as `?quality=` would hold it. Nothing moves the picture, which is what two benchmark runs need to be held against each other (`docs/agents/qa.md`, the
  benchmark's carried words, which also says what the word leaves to a hand in Options and to the pacer). The row alone was not that: under auto with the
  row off sharp direct is still shed, the tier still given up and the last visit's size resumed (found by reading, 11-04). `?nomotioncalm` puts off what is now (08-12, the maintainer's call) the DEFAULT: Tab ▸ Frame / `__gimo.calm()` = the gi-motion lane's bundle (restirSpatialAll + restirPenRays + taauMotionAlpha 0.25 +
  taauSharpenMotion 0.15) that calms the DIRECT chain under camera motion at the price of contact-shadow softness — the maintainer's call (docs/internal/gi-motion.md). Since 08-11 `renderScale` is RELATIVE TO THE CAP: internal h = min(maxInternalHeight, cssH·dpr) × renderScale, so 1.0 =
  the preset's rows on any window tall enough and every step below it is real (`capScale`, `effectiveScale` = the true internal/css·dpr ratio for the cost model); render
  targets are pooled and temporal histories RESAMPLED across size changes (render/texpool.ts, `settings.keepHistories`), cascade instances parked per grid (`engine.rebuildRc` /
  `prewarmRc`). GPU cost: read the timer's UNION (`engine.timer.unionMs()`, HUD 'gpu ∪', recorder `gpuUnion`) — per-pass sums over-state a saturated frame by 35–55 %
  because independent compute passes overlap (docs/internal/p0a-cascades.md). `plan(gpuMs, fixedMs, preset, eff, cssH, dpr)` is the pure decision (exported; also
  `__quality.plan`); `bun run tools/qa/quality-plan.ts` prints the cost table and plan()'s picks for synthetic machines (M4 Max →
  high ×1.0, M1-class HiDPI → verylow, …) — re-run it after correcting REF/FACTOR from measured pass timings. Under automation set
  `window.__testVisible = true` or calibration/sampling wait forever on the hidden pane.
- **Auto lets sharp direct go before it shrinks the picture** (10-25, the maintainer's pick off his own takes: 'Auto drops it'; `settings.ladderShed`, on in the page, `?laddershed=0` or Tab ▸ Frame ▸ 'ladder: auto drops sharp direct before pixels' = the ladder as it was; `ui/quality.ts SHED`, the block above the constant is the rule in full). Sharp direct draws the g-buffer, the direct light and the composite at the OUTPUT size, so it is the one part of a frame a smaller render scale does not make cheaper: on his Mac at High ×0.5 it was 4.9 of 15.5 ms, High could not hold 60 in a fight even at the scale floor, and with it off he saw no difference. UNDER AUTO ONLY, wherever the ladder would take pixels off the tier's own picture (the cut by rate, the step for the back-off band, the cut taken with a load spike's rungs, an aim's judged cut on a display missing its vsync, a preset starved away) and sharp direct is drawing and the tier's to place (no `?sharp` / `?nosharp` / `?noiq`), it turns sharp direct off INSTEAD, with its two riders (the decoupled jitter, GBAA); rungs the climb had put on top of the tier's start still come off first. The shed stays across auto's preset hops (`hopRatio` prices the next preset without it) and ends with a pin, auto chosen again, or the switch. It comes back when the tier is at its own scale, the rate at the aim, no preset hop about to be taken, and the union PRICED WITH IT ON THE TIER'S OWN PICTURE clears the same 12.0 ms a restored scale step must, six windows running; no sooner than 60 s after it went, doubling each time it has to go again soon after coming back. Rungs the climb took while it was off are priced off and come off with that move: the tier's own row outranks the extras. ITS PRICE is the cost model's share, or what this machine's union showed if that is more (capped at 3× the model): off a shed, what the union gave back, trusted for twice the hold-off only (a spike that ended in that second reads as its price, and a reading nothing could test again would keep it off for the visit); on the way back, what the union took on in a calm scene, which replaces the other and stands 15 minutes. So a machine the model underprices tries it once after a fight, reads the price and leaves it. Where it does not fit the climb spends the room on pixels, to ×0.95. Put back on by hand (the Tab row) while shed, the ladder forgets its shed. The benchmark and the F8 sweep keep a standing shed across their own preset flips, and an F9 take begun mid-shed says so (`shedAtStart`), so the replay tool does not read it as a row put off by hand. A shed is JUDGED by the paced check like the cut it replaces: on a page something else holds to a low frame rate it buys nothing and goes back with the chain's pixels. Words in a take: `shed ↓ sharp direct off (…)` / `shed ↑ sharp direct back (…)` as `ladder` events, `__quality.shedState` in the console. NO `FROZEN_HOLDS` ROW and no knob table: the ladder exists only in the page (`main.ts` builds `Quality`), the headless Game has none on either stand, and nothing in the update reads `sharpDirect` (the one reader, `Game.viewPx`, sizes thin lines in the picture and already follows the render scale). The check: `bun run tools/qa/ladder-replay.ts --selftest` (26 lines of its 110; `LADDERSHED_BREAK=1` fails twenty; the same counts under `TPS_PROFILE=shipped`), and `--shed on|off [--sharpk N]` replays a take with the rule forced (a take from before the switch replays with it off). WHAT WAS MEASURED: nothing on a GPU. On paper, two of his Auto takes replayed with sharp direct priced as his sweep measured it (`--sharpk 2.2`): a 110 s fight that the page spent 88 % on Medium at a mean 582 rows stays on High at a mean 770 rows, at the aim 100 % of the time (98 % before); a 179 s take that spent 92 % on Low at 479 rows stays on Medium at 590. The replay is a response model, not a render. NOBODY HAS SEEN: the rule on a GPU at all; what a shed looks like at the moment it happens (the targets are re-made, as when the Tab row is flipped); the picture without sharp direct on Medium and Low, which no take or eye has compared. NOT COVERED, on purpose: a pinned preset keeps sharp direct whatever its frame rate (the pick was about Auto), and the boot's `plan()` still prices every tier with it on, so a machine that boots onto a lower preset is not offered the higher one without it.
- **An even pace on a fast display, and a frame rate readout that is true** (11-03; `src/host/pace.ts` `EVENPACE {on, edge 0.05, slack 0.4, gaps 24,
  least 6, near 1.12, window 1}`, ON in the page, `?pace=0` = every display frame worked, as it was; OFF in the module and on the frozen stand,
  `TPS_EVENPACE=1`, a `FROZEN_HOLDS` row and a stand word kept only so the stands' books are whole: no display asks a harness for a frame. The page's alone
  as `FIXEDSTEP` is: on `VISIT_OWN`, its word on `VISIT_FLAGS` and on a benchmark's `CARRIED`). WHY: the maintainer's report of 11-03, 'despite saying
  >100 fps it feels kinda laggy', with two recordings off a 120 Hz laptop. On High with the GPU at about 17 ms a frame the gaps between frames ran 3, 30,
  3, 29, 4, 27 ms: about 54 frames a second in pairs, three quarters of the time spent in gaps of 26 ms or more, a third of the frames under 6 ms after
  the one before, while the HUD said 166 to 200. With the GPU at 10 ms it was 5, 20, 5, 20 (76 a second, the HUD 126); only with the GPU under a display
  frame (very low, 5.6 ms) was the pace even. The display asks for a frame every 8.3 ms, nothing in the loop waits for the GPU, so two frames go out back
  to back and the browser then holds the page. The look prototype's run and the plain one were alike in this. THE RULE: while the ladder holds a frame
  rate (`quality.adaptive || quality.auto`; `quality.bands.target`, 60 unless the player picked another under the debugger's 'ladder: frame rate to hold'
  or `?aim=`) the page draws at that rate: `paceEvery(displayHz, target)` = floor(hz / target + edge) display frames go to one drawn frame. Every second
  at 120, 144 and 165 Hz, every third at 180, every fourth at 240; every one at 60 to 116, at an aim of the display's own rate (`?aim=120` on a 120 Hz
  display draws every frame), at 'as now', with the ladder off (a pinned preset with Options ▸ Hold frame rate off), and while the page measures the
  machine (`quality.calibrating`, `bench.active`, `perfsweep.active`: no rate is held there; a benchmark and a sweep still wait for the GPU, below). Never under `?kitview`, which
  has its own pacer. Only a frame that comes EARLY is passed over (`ts - lastWorked < (every - slack) · period`, on the frame callback's own stamps): one
  that is due or late is always worked, so a machine that cannot hold the pace is never slowed further and a late frame moves the beat. It sits in
  `main.ts frame()` before the time is taken (the step takes the real time since the last WORKED frame, and the ladder samples that gap, so it reads a
  paced 120 Hz display as it reads a 60 Hz one, the case it was built on). THE DISPLAY'S PERIOD is the pacer's own reading off every callback's stamp
  (`periodOf`: the fastest three gaps that agree, then every gap a whole number of those; the kit view's `fastest`), not `quality.displayHz`, which is only
  ever raised and under-reads frames in pairs: the short gap of a pair shows the period, a paced page shows it at every frame passed over, and a window
  dragged to a slower display is read again inside a quarter of a second. A page slow on EVERY frame reads a slower display than it has and is given every
  frame, which is what it had. WHY NOT `kitViewPacer`: it steps a slow machine down to an even 45 or 30 and passes frames over to look at the period (a
  probe at boot, a peek every so often); in the game the ladder is what brings the work inside the frame and a frame passed over to look is a hitch.
  THE READOUT: `fpsSm` (the HUD row, the debug strip, the perf recorder's `fps`) was a smoothed 1 / dt, the mean of the rates, which counts a 3 ms gap as
  333; it is now the frames of the last second over the time they took (`rateMeter`). It is not switched: `?pace=0` shows the old pace with the true
  number, and a perf take's `fps` column from this step on is not an older take's. KNOWN LIMITS: at 144 and 165 Hz the paced frame (72 and 82 a second)
  is shorter than the ladder's 60th, so work between the two still misses a display frame now and then, as it did (the ladder does not hold the paced
  rate there); an aim that is no whole part of the display's rate is not paced (90 on 120 Hz); a pinned preset with the hold off is not paced, and there is
  no step down for a machine that cannot hold the aim at the ladder's floor (frames come as the GPU finishes them in both cases since the bullet after
  next, 'the page waits for the GPU', and no longer in pairs); a
  machine that held an even 120 on auto under the top of the ladder is now drawn at 60 (`?aim=120` or the debugger's row is the way back: Options has no
  row for the aim); telemetry's per-minute rows from a 120 Hz machine now read 16.7 ms a frame at aim 60 while `hz` says 120. `window.__pace()` = `{on,
  hz, every, skipped, worked, target, fps}` and the give-way fields of the next bullet. Check: `tools/qa/pace-check.ts` (stamps played through the pacer at 60, 100, 120, 144 and 240 Hz with a
  wobble, with frames held back, in pairs from the first frame, across a change of display both ways; the readout on the recording's own gaps, on a hitch
  and at two frames a second; the wiring read off the source; the switch on the stand; 54 lines on each stand with the next two bullets'; `EVENPACE_BREAK=1` fails 9 on each, two of them lines of the wait for
  the GPU that stand on the even pace). On
  the real page (headless Chromium, software adapter): boots with no error; on the browser's own 60 Hz frames nothing is passed over; handed callbacks
  stamped as a 120 Hz display stamps them it reads 120 and passes over every second one. MEASURED: nothing on a display. The cause is read off two
  recordings from one machine; the mend is arithmetic on stamps. NEVER SEEN ON A GPU OR A FAST DISPLAY. Whether an even 60 feels better to the maintainer
  than what he had, and whether he would rather hold 120 at a lower preset, is his to say after playing.
- **A window nobody is at gives way** (11-03; `src/host/pace.ts` `EVENPACE.giveway` with `share 0.5`, `longest 50` ms and `wobble 0.1`, ON in the page,
  `?giveway=0` = an unfocused window draws as a focused one, as it was; OFF in the module and on the frozen stand, `TPS_GIVEWAY=1`, a `FROZEN_HOLDS`
  row; the word is on `VISIT_FLAGS` and a benchmark's `CARRIED` beside `pace`). WHY: the maintainer's report of 11-03 from a co-op try on one laptop,
  'seems to just stuck on setting up the GPU when i open two windows on my m1'. A browser has one process that talks to the GPU for all its windows, and
  nothing in the frame loop waits for the GPU, so a window hands over frames as fast as the browser takes them whoever is looking. The even pace above
  does not cover it: it holds the ladder's rate only on a fast display and only once the boot's calibration is done, and a frame that is due is always
  worked, so a window whose work fills its frame keeps the GPU full. REPRODUCED IN KIND, on the software adapter only (two windows of one headless
  Chromium 153, 640 x 360, very low, the first on its title, neither driven): the second window's `requestAdapter` took 49 s and its `requestDevice`
  196 s (34 ms and 5 ms alone), so it sat on 'starting the GPU…' for four minutes and had compiled none of its 98 programs ten minutes in; alone the
  same page was ready in 120 s. THE RULE: a window without the system's focus has ONE frame with the GPU at a time (the next is not begun until
  `queue.onSubmittedWorkDone` for the last has answered), and takes at most `share` of the GPU's time: it begins the next no sooner than that frame's
  work took, over `share`, after the last was begun, and never more than `longest` after it (the loop's step takes a twentieth of a second at the most,
  so a longer wait would run that window's clock slow; so past 25 ms of work a frame it is one at a time and no longer half). Work under half the frame
  is under the share already and nothing changes; work that fills the frame is drawn about every second frame while nobody is at the window (11 ms of
  GPU work: 30 a second on a 60 Hz display, 40 on a 120 Hz one), and the frame the focus comes back is worked at once. The window somebody is at is
  never held by it. Not while the page measures the machine (`quality.calibrating`, a benchmark, a sweep) or records a take (`perfrec.recording`).
  Under `?kitview` too (the kit picture is a live window of its own, whose pacer knows nothing of who looks; the game's pacer holds no rate there and
  only gives way). THE MEASURE of a frame's work is the GPU's own timing of its passes (`engine.timer.unionMs(GPU_RENDER_PASSES)`, the ladder's number)
  and never more than the queue's answer took; with no timings it is the answer alone, which also counts the other windows' work ahead in the queue, so
  there a window beside a busy one gives more than half. The wait runs from the last worked frame's own stamp (`begun`), which a stall does not forget.
  WHO IS AT IT (`main.ts attended`): `window.top.document.hasFocus()`, asked of the top window because the game's page is a frame of the kept page and
  a frame's own document has the focus only once it is clicked into; a top of another site cannot be asked, counts as somebody and is asked once; a page
  check says it with `window.__testFocus`. So a click into the browser's own bar or its developer tools is also 'nobody at it' for as long as it lasts.
  THE LADDER and the visit's per-frame records skip a frame whose gap held a display frame passed over for giving way (`rested`): that gap is the
  rule's, not the machine's. An unfocused window with room to spare passes nothing over and is read as ever. `__pace()` adds `{giveway, attended, gave, gpuMs}`. Checks: `tools/qa/pace-check.ts` (a GPU of 5 to 70 ms a
  frame at 60 and 120 Hz, another window's work ahead in the queue with and without timings, callbacks that never come, one answer of three seconds, the
  focus going and coming back, the wiring off the source; `GIVEWAY_BREAK=1` fails 8 on each stand). ON THE REAL PAGE: the same two windows with the first told nobody is at it (`__testFocus`; the browser's own
  answer was read too, in a bare page with a frame: the frame's own document never has the focus, its top's follows the window in front). With the
  rule: the second's two asks took 23 s and 7 s (most of it behind the three frames the first had handed over before it was told) and it was ready in
  196 s; the first drew 17 frames meanwhile and passed 154 display frames over, with no error on either page. With `?giveway=0`: 37 s and 164 s, and
  not ready four minutes in. Not run: `kitview-page.cjs` (with somebody at the window the pacer's call under `?kitview` holds no rate, waits for no frame and
  passes nothing over, read off the code). KNOWN LIMITS: a
  window somebody IS at keeps the GPU as full as its work makes it (two frames with it at the most since the next bullet, where it was as many as the
  browser took), so a second window opened while the first keeps the focus, or the first clicked back into while the second starts, still waits behind
  it; a window still on
  its boot's calibration does not give way; the share has a step in it (at 60 Hz, work a hair over 9 ms a frame halves the unfocused rate), since a frame
  is drawn or not; an unfocused window that is the only thing on the GPU still gives way, since nothing says whether another window wants the GPU; the
  queue's answer is asked for beside the one the visit's records ask for (`cloud.ts lagAsk`), not through it. NEVER SEEN ON A GPU. On a real card the
  browser's GPU process does not run the kernels itself, so how much of the maintainer's stall this is, is a guess from the software adapter until he
  tries two windows again; whether he wants the window he is not driving at half pace in a co-op try on one laptop is his to say.
- **The page waits for the GPU before it starts a frame** (11-03; `src/host/pace.ts` `EVENPACE.flight`, 2 in the page, `?flight=1` = one, `?flight=0` =
  nothing waits, as it was; 0 in the module and on the frozen stand, `TPS_FLIGHT=1`, a `FROZEN_HOLDS` row; the word is on `VISIT_FLAGS` and a
  benchmark's `CARRIED`). WHY: the maintainer on 11-03, of the look page's trial on his 120 Hz laptop: '&pace looks good, let's make it the default'.
  That trial (the look branch's `src/host/gpuPace.ts`, its `&pace`) passed a display frame over whole while two frames were still with the GPU, told by
  `queue.onSubmittedWorkDone`. It is the cure for the sentence both bullets above turn on, 'nothing in the frame loop waits for the GPU', and the build's
  pacer had it only for a window nobody is at. THE RULE, in the build's ONE pacer: a frame is not begun while `flight` frames are still with the GPU
  (`pacer.sent` at every worked frame, `done` at the queue's answer, oldest first). Two = one being drawn and one waiting behind it: the GPU never idles,
  so no frame a second is lost to the wait, and a frame is at most two old. One = a frame is begun only when the last has left: the freshest picture,
  the GPU idle until the display's next tick, and fewer frames a second wherever page and GPU work used to overlap (17 ms of GPU work on 120 Hz: 40 a
  second, where two gives 59). A window giving way keeps to one, as above. WHAT HE APPROVED AND WHAT THE BUILD ADDS: the look page had no even pace (it
  is 300 steps older), so under `&pace` it took every display frame the GPU had room for. The build keeps its even pace UNDER the new rule: the even
  pace says which display frames are asked for (every second one at 120 Hz while the ladder holds 60), this rule says whether the GPU has room for the
  one asked. With work inside the paced frame nothing is ever held and the two builds draw the same even 60; with work over it both draw as the GPU
  finishes, one or two display frames apart, never back to back; with work under HALF a paced frame on a fast display the look page drew more than 60
  (uneven: gaps of 8 and 17 ms mixed, at 11 ms of work) and the build draws the even 60 the ladder holds. `?pace=0` with the rule on is the look page's
  `&pace` to the letter; `?aim=120` holds 120. Which of the two he wants under 8 ms of work is his to say; it was said to him. A FRAME HELD FOR THE GPU
  IS THE MACHINE'S OWN PACE: the ladder and the visit's records read its gap (`held`, not `gave`). Not during the boot's calibration by itself
  (`room` 0, frames go as they always did); a benchmark and a sweep wait since 11-04 (`pageRoom`, further down); on while a take is recorded. NOT under `?kitview`: the
  kit picture's own pacer reads how late its frames come and steps down to an even pace by that, and a frame held here after that pacer had taken it
  would hide the lateness from it (a window of it nobody is at still gives way). THE QUEUE IS ASKED once for every frame that DREW
  (`behind.drawn`): a card that hides the picture hands the GPU nothing and asks nothing. The visit's records ask the same queue the same thing on
  some of the same frames (`cloud.ts lagAsk`); the two are not one asker yet. A FRAME THE QUEUE NEVER ANSWERS FOR is given up after `lost` (10 s) and
  the page goes on without it (`__pace().lost`): held at its limit for good by one lost answer, a window would be a frozen picture with no card. Ten
  seconds, because a frame on the software adapter is seconds long and is not lost. Each answer is its own frame's (`sent` gives a word, `done` takes
  it), and with two in flight and no pass timings a frame's time is counted from when the GPU was free of the one before it. The debug strip has a
  `pace` row (the display as read, every which frame, frames on the GPU of how many, the share of the frames asked for since the strip was last written
  that found no room; 'not waiting for the gpu' where the rule is not acting); `__pace()` adds `{flight, flying, held, lost}`. Checks:
  `tools/qa/pace-check.ts` (120 Hz with no rate held and the GPU at 17 ms: two with the GPU at the most and the GPU's own 58.8 frames a second; the
  ladder's 60 with the work at 15 and at 20 ms; a 60 Hz display at 5 and 25 ms; at one; who waits (`pageRoom`); the boot's calibration (nothing held, frames pile up); a benchmark at the card's own pace; each answer its own frame's; two frames
  never answered for; A TAB THAT SLEEPS with two frames in flight, four ways: asleep 3 s with the answers come meanwhile it draws every frame at once;
  asleep 30 s and never answered both are given up on the first frame back; asleep 3 s with the GPU still at the two it holds six display frames and
  draws on; asleep 3 s and never answered it waits, ten seconds from the hand-over at the most; the wiring; 54 lines on each stand; `FLIGHT_BREAK=1`
  fails 11 on each stand: frames pile up without end). A BENCHMARK AND AN F8 SWEEP WAIT TOO (2026-11-04, `host/pace.ts pageRoom`, the one rule the
  frame loop and the readout read; until then every measuring page was left out, 'which wants its frames as they come'). On a display the browser
  holds a page's frames back by itself, and a sweep on a 120 Hz laptop with 15 ms of GPU work repeated to a tenth of a millisecond over three
  rounds. A headless browser on a cloud card has no display: on a scene over 16.7 ms the page handed over a frame every 16.7 ms whatever the card
  did, the queue grew for as long as the scene was held (found by the pricing run, which measured the GPU's 'done' 115 to 629 frames late), a row
  flipped in a sweep reached the passes' stamps in another row's turn, and the card's frame rate read the display's 60. Two benchmark cards of
  one build on two such boxes agreed on GPU time to 2 % on eight lines of nine and disagreed on their sweep rows (mesh shadows off 'gave back'
  1.1 ms on one and cost 0.6 on the other). With the wait a measuring page is at most two frames ahead of the GPU, as a player's is, and where
  the card keeps up nothing is held. A benchmark's own calibration waits with it, so a run opens with no pile behind it; only the boot's
  calibration by itself (a second and a half that counts frames as they come: on the software adapter the wait would stretch it without end) and
  `?kitview` still say room 0. The even pace and giving way stay off while measuring, as before. A result says what it ran under (`wait`; absent
  = from before). WHAT THIS CHANGES ELSEWHERE: a benchmark's frame rate is now the pace a player's page gets, so on a fast display where the
  queue answers late it can read under what the card could do (the limit named above, for play), and on a slow headless card a scene runs in
  slow motion against its wall-clock cap as it does on a slow machine with a display. On an adapter slower than five seconds a frame the wait
  gives frames up (`lost`) and nothing is bounded: the software adapter's page tools prove mechanics there, never a number. MEASURED ON A
  CLOUD CARD (an NVIDIA A10, Vulkan, headless Chrome 157, 60 Hz, High with the size held, the benchmark run twice on two boxes before and
  twice after; relative only, never on a Mac or a Windows card): the held picture went from '60 fps, a frame every 16.6 ms' beside 17.8 ms of
  GPU work to 50 to 53 fps (a frame every 16.8, 95th centile 33.5); GPU time and the g-buffer's the same as before on every line; and the
  sweep's rows, which had disagreed between the boxes, agree in sign and near enough in size (the whole other preset 7.8 and 8.2, mesh
  shadows off 3.2 and 3.9, sharp direct off 2.0 and 1.9, the top of the climb -3.6 and -3.9). WHAT THE WAIT COSTS THERE: the queue's answer
  comes late on that headless browser, so 'as it opened' reads 57 fps with 7.3 ms of GPU work (60 before) and the heaviest scene 33 fps where
  frames had been handed over at 47: a benchmark's frame rate on such a card is the pace a player's page gets on it, under what the card could
  do, and a card from before the step is not to be held against one from after by its frame rates. Its GPU times can be. ON THE REAL PAGE (headless Chromium 153,
  software adapter, not driven):
  the title at 640 x 360, very low, watched for seven minutes after it came ready. It came ready as ever, drew 20 frames in
  those minutes, never had more than two with the GPU, and printed no error. A frame takes about 11 s on this adapter, so every one of them outlasted
  `lost` and was given up (20 of 20): here the rule comes down to the page as it was, which is what a page check on this adapter should see. On the
  code before the review's mends (no `lost`): the title drew 19 frames, never more than two with the GPU, 9 display frames held, no error; under
  `?flight=0` nothing was held; the compound's own address came ready with two frames in flight and no error. That is a boot and a count, not a pace:
  at 0.09 frames a second nothing here says how it feels. THE PAGE CHECK, `PORT=<port> node tools/qa/pace-page.cjs` (2026-11-03; nine minutes to half an hour:
  frames come very unevenly on the software adapter, and the first ones after ready take minutes with the rule on or off): the one page tool whose frame
  loop is LIVE. Every other page tool opens the page `?paused` and steps it with `__runFrames`, which calls the step and never meets the pacer, so a red
  line in one of them cannot be a pacing rule's. It opens the title undriven, reads that the three rules are on at the page's numbers, watches four
  minutes after ready (frames keep being worked, some are seen with the GPU, never more than two), then PUTS THE TAB TO SLEEP for fifteen seconds with
  frames in flight and wakes it: the page's own `freeze` and `resume` events say that it slept and worked no frame asleep, and awake it draws on.
  `PACE_PAGE_BREAK=1` opens with `?flight=0` and its lines 2 and 3 fail (nothing is ever with the GPU). IT SPEAKS TO THE BROWSER OVER ITS DEBUGGING
  PORT, with no driver library, because UNDER THE DRIVER THE OTHER PAGE TOOLS USE A PAGE NEVER SLEEPS: its focus emulation holds the page focused and
  visible, and hiding the tab, minimising the window and the protocol's freeze all left a plain page's frame callbacks running with no `freeze` event
  (so a first draft of this line passed twice without a sleep in it). The sleep here is that emulation off, then `Page.setWebLifecycleState` frozen
  (the page reads hidden and its callbacks stop), and back. One run on this box: 4 frames in the four minutes, 2 with the GPU at the most, 4 given up
  unanswered; asleep with 1 in flight, 6 frames worked at the freeze and 6 at the resume, 2 given up over the sleep, 3 worked after it; no error. Not
  in any gate (it wants a browser and a server). It says that a page waiting on the GPU's word boots, draws, sleeps and wakes, never a pace. AT THE LOOK'S JOIN the trial goes: `src/host/gpuPace.ts`, its three lines in `main.ts frame()` and its
  HUD row are replaced by this (its `&pace` = the default here, `&pace=1` = `?flight=1`, no word = `?flight=0`). KNOWN LIMITS: how late a real browser's
  queue answers is not known here. An answer that comes later than a paced frame after the work is done costs frames (the pacer counts a frame as with
  the GPU until it is told otherwise), and the ladder then reads slow frames with the GPU inside its budget, which its paced check is there to tell
  apart; at 120 frames a second and more it bites sooner than at 60. His try of the trial at 120 Hz is the only measure, and it was at 12 to 15 ms of
  GPU work. The model in the check has one GPU serving frames in order and answers that arrive at once, nothing like a real card's own queueing. SEEN ON A GPU ONLY AS THE TRIAL, on the look page, by the maintainer; this code never.
- **One cache grid at every preset** (10-23; `settings.oneCacheGrid`, on in the page and the workbench, `?onegrid=0` puts it off; no Tab row). WHY: the light meter the
  guards and teammates see by (`probe.wgsl`) is the shadowed direct light plus the cache's dice read trilinearly at the asked point, so the probe lattice is part of the
  reading. low and verylow ran a coarser lattice (32 × 3 × 22 against 40 × 4 × 28) and read tight night shadow two to three times darker: sneaking was easier on the low
  presets, and the auto ladder moved the rule under a mission whenever it stepped between medium and low. WHAT: every preset's `rc` row is the one grid; low and verylow
  keep the old one as `rcCoarse`, and `rcGridOf(preset, one)` is the only reader (the preset put on, the grids parked at boot, the cost model: `presetCost(p, oneGrid)`,
  `costTable`, `plan(…, oneGrid)`), so off = the grids AND the ladder's prices as they were. With it on no preset step rebuilds or swaps the cache, and nothing is parked
  at boot. No `FROZEN_HOLDS` row and no journal line: nothing in a step reads it (the meter reaches the game through `Game.sensors`, and the headless stand-in has no cache).
  MEASURED on the software adapter (the pair mission at night, the meter asked at 11 fixed points at 0.8 m and 1.5 m, ten drawn frames, one run each; E, then the HUD's
  LIGHT): in the gap between the containers verylow 0.015 / 0.018 (LIGHT 0) on the coarse grid, 0.046 / 0.057 (13) on the one grid, high 0.049 / 0.063 (15); behind the
  low wall at 0.8 m 0.045 → 0.103 against high's 0.113; the road in shadow LIGHT 0 → 9 (high 9); the partner's start 0 → 6 (high 7); points in the moon 46 to 48 on all
  three. What is left between verylow and high is under 10 % at nine of the eleven points and 14 % at a tenth (0.017 against 0.020), verylow mostly the lower: its far
  field refreshes over more frames and ten had not settled it (inferred). The eleventh sits beside a standing man at 0.8 m: 0.117 on both verylow grids, 0.052 on high. Not the cache: `occluded()` tests a character's skinned mesh where the preset has mesh shadows (ultra, high near) and his
  capsules elsewhere, so the meter's direct light in a man's own shadow is still the preset's (read in the code, one point; the meter's owner's, not mended).
  NOT MENDED: the read itself. One lattice makes the presets agree; it does not make the meter right — it still blends the cells around the asked point, some of them
  inside the wall or the container beside it (the bullet on BOUNCE WALLS says the same of a lit room's wall). A read of the meter's own (its own short fan of rays from the
  asked point, the cache only at their hits) would take the lattice out of the reading at every preset and cost a few thousand rays a frame, but it moves every LIGHT
  number the levels were tuned on. PRICE: low and verylow trace 2.1 × the probes at their own refresh periods; in the cost model their fixed part goes 1.9 → 2.4 and
  1.4 → 1.9 model-ms (`quality-plan.ts`: an M1 / M2-base-class machine, 4.5 × the reference, boots at verylow ×0.50 where it had low ×0.50; a 5.5 × one no longer fits
  the budget at the floor), about +0.2 ms a frame on the reference machine by the cache's measured 1.05 ms at high and the level shares. NEVER TIMED ON A GPU, and never
  seen on one: what low and verylow look like on the finer cache is not known. CHECK: `bun run tools/qa/probes.ts ladder` (family `perf`; the ladder's self-test, which
  now runs the page's grid, with five lines of this rule; `ONEGRID_BREAK=1` fails four; both stands). `rfix-proofs.ts r101` (the parked-grid hop) boots with `&onegrid=0`
  so it still crosses grids; that edit was not run. Page tools that pin `?quality=verylow` on the software adapter now trace the finer cache too.
- **One smoke pressure solve at every preset** (10-23; `settings.oneSmokeSolve`, on in the page and the workbench, `?onesolve=0` = each preset's own `rbIters` as it was:
  ultra 5, high 4, medium, low and verylow 3; no Tab row beside the solver's own 'pressure: rb pairs' slider; render-side, so no game table, no journal line, no
  `FROZEN_HOLDS` row: the headless stands solve no smoke on a GPU). WHY: a sight line through smoke (`game/sensors.ts` → `probe.wgsl` kind 1) is the optical depth of the
  SOLVER's density, and the pressure pairs decide how a plume rolls and spreads. With the pairs a preset row, the cloud that hides a man had another shape at medium than at
  high, and the auto ladder changed it under a fight each time it stepped: a game rule reading the graphics setting, as the meter's cache grid was. Every preset now runs high's
  4 (`rbItersOf(preset, one)` in ui/quality.ts, asked where a preset is applied). PRICE, by reading only: one pair more (two dispatches over the live bricks) a solver step
  on medium, low and verylow, one fewer on ultra; the cost model does not price the solver, so the ladder's price for a step down no longer includes a cheaper solve. The one figure on record is the old
  box solver's on the maintainer's own machine: 0.044 ms a pair a domain (`docs/internal/smoke-perf.md` §1b). NOT MEASURED: how far a plume's shape moved between 3, 4 and 5 pairs, so how much
  sight ever differed; and the cost. NOT MADE ONE by this: the density a sight line reads is still the page's solver, and the headless stand-in is another model
  (`docs/agents/replays.md`). CHECK: `bun run tools/qa/probes.ts ladder`, three lines (`ONESOLVE_BREAK=1` fails one), both stands. PAGE (the software adapter, the final code, `?paused&quality=medium&mute`):
  booted with 4 pairs at medium; low, verylow and medium applied in turn each left 4; with the setting put off and the same three applied, 3 each; back on, 4; a canister solved
  six frames at `stats.rbPairs` 4 with no GPU or page error. The `?onesolve=0` word itself was not opened on a page (one line beside `?onegrid=0`, read before the first
  preset). Never timed or seen on a GPU.
- **The smoke keeps high's look down the ladder** (10-23; `settings.smokeKeep` 0 | 1 | 2, 1 in the page and the workbench, `?smokekeep=0|1|2`, Tab ▸ Frame 'smoke under
  high'). The maintainer, with auto on medium: "a ton of the smoke became very blocky … the smoke looks so sick in our game that i'd rather we keep it closer to high but
  then tradeoff other things in the frame budget". WHAT WAS BLOCKY: under high the march ran at quarter size (`lossy.volQuarter`: one march texel under 4 × 4 internal
  pixels; high's half-size march has 2 × 2), lit by fewer lights a step (medium 3 / 2, low and verylow 2 / 0 = `caps.volK` / `caps.marchLights`, high 4 / 3) and on low and
  verylow with a 2.0 step that combs a plume's shadow edges. WHAT 1 DOES: on medium and low the smoke's rows are the better of the rung's own and high's (`withSmoke(preset, keep)`
  in quality.ts: the march at half size, high's lights a step, shadow refresh and step length), on the preset's OWN path — the froxel haze stays, beams are not the smoke —
  and everything else the rung sheds is still shed (rays, reflections, mesh shadows, bounce cards, denoiser passes, the cache's refresh, the pixel count). The ladder pays
  the difference where it always does first: in render scale. 2 = high's whole volumetric path as well (`caps.vol` 'classic': the per-pixel march for haze AND smoke,
  high's 20 beam steps): the smoke exactly as high draws it at the rung's pixel count, there so the two can be compared by eye. 0 = every preset's own rows, as it was.
  verylow keeps its own smoke at every value (`smokeOwn` on its row): it is the floor, with no rung beneath it to pay, so the smoke is the LAST thing the ladder sheds.
  The solver's iterations (`rbIters`) are not a look row and this switch does not move them: ONE SMOKE SOLVE, the bullet above. high and ultra are the same rows at 0, 1 and 2. With 1 or 2 on, the climb's
  'volumetrics ¼ → ½' rung has nothing to do above verylow, and the caps bullet's medium and low rows below are what 0 runs. `withSmoke` is the only
  reader (the rows put on, the live preset, `costTable(oneGrid, smokeKeep)`, `plan`), so 0 is the ladder as it was, prices included; `Quality.resmoke()` is the Tab row's
  by-hand step. No `FROZEN_HOLDS` row and no journal line: nothing in a step reads it (smoke SIGHT lines are the solver's density, not the march).
  WHAT IT COSTS — on the maintainer's Mac at high in a fight with one dust cloud live (the F8 sweep of 10-10, three rounds; that take's pass was beams and smoke on the
  classic path): the march at quarter size gave back 0.5 to 0.9 ms (0.4 in a calm view with no cloud), 2 lights a step where there were 4 gave 1.5 to 1.65, the froxel path
  1.5 to 1.85 — against 2.3 ms for render scale ×0.77 → ×0.60 and about 7.6 ms for all of medium. So at 1 a rung under high is dearer by roughly a millisecond at high's
  pixel count in a smoky fight (less at its own, about one 0.05 step of render scale on that Mac — INFERRED from those rows, not timed), and at 2 by about 1.5 more. The
  cost model only knows the calm still (medium's per-pixel part 13.3 → 13.5 → 14.0 model-ms at 0 · 1 · 2; no synthetic machine of `quality-plan.ts` changes its pick).
  OTHER THINGS A RUNG COULD GIVE UP INSTEAD, with that Mac's rows at high: the cache at half rate 0.9 ms, mesh shadows 0.8 (medium has none already), audit rays 0.5,
  direct light at 3 rays 0.26, held-light rays at 2 0.25 to 0.38; the reflections' passes read 3.2 ms but the frame moved 0.14. None is taken: by eye, the maintainer's.
  At 2 the model prices medium a shade UNDER 1 (it drops the froxel term and knows no fight): a setting for the eye, not for auto's arithmetic.
  NEVER TIMED OR SEEN ON A GPU. CHECK: `bun run tools/qa/probes.ts ladder` (family `perf`; nine lines of this rule in the ladder's self-test; `SMOKEKEEP_BREAK=1` fails
  two; both stands).
- **Worst-case caps (renderer v3 Phase 0)** — the loud minute (QRF weapon lights + rain + a floor full of smoke + a firefight's particles) bounded with no look change
  at rest; every tier runs the same capped paths, scaled by `PRESETS[*].caps` in quality.ts (ultra 6/2/1.5 · 512, high 4/2/1.5 · 384 — both on the CLASSIC per-pixel path since 08-12, `Caps.vol` —, medium 3/2/1.5 · 256,
  low 2/2/2.0 · 192, verylow 2/2/2.0 · 128 = volK/volEvery/volStep · births): (1) `settings.volLightCap` K (`frame.volCfg.x`; **0 = the volumetric
  pass bit for bit as it was**, proven on the vol texture) — volumetrics.wgsl marches each of the smoke's BOUNDS' own stretch of the ray front to back at that lattice's step (its comments still say 'domain')
  (merged where they overlap) instead of the hull of all of them at the finest voxel, builds a per-stretch shortlist (exact culls: range, the 0.03 floor, cone, panel
  facing; ≤ 16 sorted by unshadowed bound — never 'every light'), gives a shadow ray + self-shadow to the first K lights that reach each step (refreshed every
  `volShadowEvery` steps = `volCfg.y`, reused between; march step `volStepVoxels` = `volCfg.z` tenths — 2.0+ combs shadow edges in a plume, low/verylow only), caches the
  ambient's wall test per dice cell; haze beams keep the K strongest per ray by unshadowed in-scatter (64-bit mask; kind-2 always). Tab ▸ Direct light ▸ CAP rows.
  (2) `settings.particleBirths` → `Particles.prepare(dt, on, cap)`: births a frame; records that do not fit WAIT in the queue (long-lived kinds first once anything
  waits, arrival order untouched otherwise; an oversize record is born alone; a spark record shed after `SPARK_MAX_WAIT` 6 frames; queue ≤ `MAX_QUEUE` 512) —
  `probes.ts particles-queue` holds the defer contract. (3) the smoke solver's cap is no longer a preset's: which bricks step is the pool's own rule on every tier (calm and far bricks wait: the smoke bullet); `Caps.smokeFull` /
  `fullRateDomains` went with the boxes. Measure with `tools/qa/stress-scene.js` (`await import('/qa/stress-scene.js')`
  → `__stress.setup()` / `.record(n)` / `.ab(applyA, applyB)` / `.caps(on)` / `.table(r)`, serialized frames, orbiting camera) and `__hotVol(src?)` (recompile
  volumetrics.wgsl from disk — the dev server serves src/render/shaders under /qa/shader/ — and swap the pipeline in without rebooting a staged scene).
- **Froxel volumetrics** (renderer v3 Phase 2b) — `settings.froxelVol` per tier (`quality.ts Caps.vol`: ultra / high 'classic', medium and below 'froxel'), `?nofroxel` / `?froxel` force either; `settings.smokeMarchLights`; `__froxel`, `__hotFroxel` — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **Lossy options** (engine settings, default OFF in `defaultSettings`, switched ON by the presets as above; pause menu *Graphics —
  trade-offs* + `LOSSY:` rows in the Tab panel for A/B): `settings.checkerDirect` (frame flag 65536: shadow rays on a checkerboard
  half per frame, direct.wgsl), `gatherThird` / `volQuarter` (gather at 1/3, volumetrics at 1/4 res: `passes.setDivisors`, divisors
  in `frame.lossyCfg.xy`), `rcHalfRate` (`rc.halfRate`: uppers even frames, c0 + bake odd), `dimRays` + `dimRaysBelow`
  (`frame.lossyCfg.z`: one emitter sample per leader under that unshadowed luminance). `engine.render` mirrors the booleans every
  frame, so they are the single source of truth. Off must stay the untouched path — A/B them one at a time (pin `?quality=ultra` AND set `settings.volLightCap = 0`
  — every preset, ultra included, carries the Phase 0 volumetric cap; `__stress.caps(false)` puts both caps back to the untouched paths).
- **ReSTIR DI = the direct-light estimator** (renderer v3 phase 2a) — `settings.restir` default ON, `?norestir` = direct.wgsl's leader / tail path; the hybrid held-light path `settings.heldRays`; the motion twin `settings.restirMotion`, `?norestirmotion`; `__hotRestir` / `__hotDirect`, debug views 11 / 12 — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **TAAU** (renderer v3 phase 1) — `settings.taau` default ON, `?notaau` = the old chain; `__taa`; RULE kept here: `frame.viewProj / invViewProj / prevViewProj` ARE the jittered matrices — never build a screen ray from `cam.*` on the GPU side — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **Sharp direct** (decoupled resolution) — `settings.sharpDirect`, switched on per tier by `quality.ts Preset.iq`, `?sharp` / `?nosharp` force it; live only under TAAU below scale 1.0; `__sharp` — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **The resolve configuration per tier** (image-quality program) — `quality.ts IqTier` on `Preset.iq` (sharp direct + decoupled jitter + GBAA + history de-blur), `?noiq` = the resolve as it was, `?gbaa`, `?nomv` (`settings.motionIds`); harness `__iq` — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.
- **Stratified gather directions + the trace lab** (ported from exp/trace-perf; the notebook is docs/internal/trace-perf.md, the page docs/review/trace-perf.html): `settings.gatherStrata`
  (default ON since 08-12 — the maintainer liked it; `?nogatherstrata` = the iid gather, `__gs {on, summary()}`, Tab ▸ Indirect light ▸ 'EXP: stratified gather directions') draws the final gather's k-th ray of four from
  STRATUM (k + frame) mod 4 of the cosine hemisphere (fgather.wgsl `cosineDirK`: azimuth half × cos²θ half; mainGi's two fresh rays a Latin square over the same cells; the young-history
  extras a complete stratified set of their own) instead of iid — UNBIASED (tile means within ±0.5 %, temporal sd equal or lower) but a whole 8×8 tile's k-th rays share a quarter of direction
  space and walk the grid coherently: gather 2.03 → 1.70 ms at high 1368×770 (−16 %), −0.8 ms at ultra. passes.ts compiles the gather module TWICE behind a prepended `const GATHER_STRATA`
  (`pFG/pFGR` false = the module exactly as it was, `__png`-hash-identical to main; `pFGs/pFGRs` true) and the encode picks the pair each frame, so it flips live; NOT hash-identical when on
  (a different sample pattern). THE TRACE LAB (`tools/qa/trace-lab.js` → `__tl`, kernels `src/render/shaders/trace_lab.wgsl` served from disk, `tools/qa/gpu-quiet.sh`):
  `?trace=N` → `render/frame.ts traceConfig` → `const TRACE_VARIANT` at the head of the scene prelude (only 0 = today's walk exists on main — the branch's V1‥V8 / H1 traversal variants were
  negative results and were not merged; a new experiment adds `…V<N>` copies of occluded / occludedT / traceClosest + three thin `switch (TRACE_VARIANT)` dispatchers and the lab finds them);
  `__hotCommon` / `__scenePrelude` / `__traceConfig` / `__commonSource` / `__daylight` in main.ts, `rc.hotSwap`, `particles.hotSwapSim`; `__tl.scene('B1'‥'B5') / points / bench / exact /
  profile / ab / aa / idHash / matrix` = Mrays/s of one ray class through one query per variant, bit-exactness counters, the reference walk's per-ray counters, interleaved frame A/Bs.
  THE GATHER'S BOUNCE READ IS BOUNDED (08-12, docs/internal/render-defects-0812.md §3, rig `tools/qa/rd-scenes.js`): every secondary-hit cache read in fgather.wgsl goes through
  `lighting_common diceIrradianceFrom(Ph, n, P0, N0)` — one probe spacing off the hit face like any dice read, but never past the plane of the surface the ray LEFT (P0, N0), and no trilinear
  neighbour from behind that plane (they collapse to the home cell) — the fix for indirect light 'leaking' behind the manager's shelves (the cartons' backs 12 cm off the wall read the lit
  storage room next door: 26–50 → 5); a new bounce read at a hit takes the receiver's plane the same way, never the bare `diceIrradiance(Ph + n·1 m)`.
- **The denoiser** (renderer v3 phase 3) — `settings.denoiser` `'v3'` default | `'legacy'`, `?legacydenoise`; one core (`render/shaders/denoise.wgsl`), three clients (shadow / diffuse / specular); `__dn`, `__hotDenoise`, debug view 13 — the full bullet moved word for word to `docs/internal/claude-md-renderer-systems.md`.

<!-- was CLAUDE.md lines 2979–3015 -->
- **Smoke = one pool of bricks** (THE smoke solver since 2026-10-10, no switch; the maintainer on the eight 64³ boxes it replaced: "let's just remove the box smoke if this is the way we want to do it" · "we can always revive from
  git history" — the last commit with both solvers is 1530281c, the pool there behind `?sparsesmoke`; notebook docs/internal/sparse-smoke.md, page docs/review/sparse-smoke.html; the names `smoke.sparse`, `SPG_* / spg*`, sparse-lab.js are the prototype's, kept):
  `src/smoke/smoke.ts SmokeSystem` + `smoke.wgsl` + `smokeGrid.ts`: the boxes' solver step on ONE level-wide lattice per voxel size (fine 2.2 cm gun gas, coarse 5.5 cm cans and dust) cut into 8³-cell
  BRICKS that exist only where smoke is (+ `ring` cells of air), one pool of 4096 (`?smokepool=N`, 256‥8192, read once at boot; `?sparsesmoke=N` still reads as that, bare it does nothing), page table / free list / live list / 27-neighbour tables all made on the GPU, 17 + 2 × pairs dispatches a step whatever the
  plume count; every reader goes through `shaders/smoke_read.wgsl`, appended to the scene prelude (r32uint page texture at scene binding 14 + an rgba16float atlas of 10³ blocks at 8; binding 7 = `array<SmokeBound, 8>`, GPU-made; binding 11 is free; shadow rays + the probe walk bricks, the view
  marches walk ≤ 4 bounds per level). No domain count, no faces, no edge fade, nothing for game code to count (`stats.live` = the pool in 512-brick units for the recorder's column; no rule reads it); the limit is total VOLUME (soft ceiling from 60 % of the pool, sooner and harder far from the camera's focus: `viewBias` / `viewR` round `engine.viewFocus`).
  WHAT WENT WITH THE BOXES: `pressure_half.wgsl`, `smoke_lab.wgsl` + tools/qa/smoke-lab.js (`__sl`), `dust_look_sparse.wgsl` (dust_look.wgsl is that text now), FrameFlags `SmokeRenderSkip` + `settings.smokeRenderSkip`, `Caps.smokeFull` / `fullRateDomains`, `CHAOS.puffDomains`, `SmokeSystemLike.unbounded`,
  seven Tab rows (CAP domains, both skip-empty-bricks, flush ε, pressure at half res, fine / coarse voxel).
  ROUND TWO (10-10, each a knob of `__smoke.sparse` and a row of
  Tab ▸ Volumetrics ▸ Smoke pool, each off = the pool as first benched at 3781d45; `__spl.asBenched()` = all off; notebook §9): WHO STEPS = per BRICK, a brick whose fastest air crosses ≤ `params.calmCells` cells (`farCells` 1 out of sight; 2 left nearly twice the smoke hanging at 7 s: a brick that steps less often loses less, notebook §9.1)
  waits 1 / 2 / 4 frames (`calmMax`; 1 = off) and then integrates what it sat out, never past `dtCap` 1/12 s, the shortest wait of its 27 neighbours, a source wakes what it reaches; the whole pool is dispatched directly from `directAt` of it
  stepping. READS HOP (`readSkip`, SHIPS OFF since the Mac's take: eight pairs, no time saved): shadow rays, the lit marches and the froxels skip stretches the page says are clear (`smokeClearRun` / `smokeAnyIn` over a macro map) — same samples, same answers. WHO GIVES WAY (`fresh` 0.15; 0 = first
  come): as the last 2 × fresh of the pool fills, what counts as thin rises, haze is refused the last slots, keeps less air and is let go sooner; nothing evicted, nothing for game code to count. WHAT IS IN A CELL: one word per
  coarse cell (share not canister smoke, 1 − tint) carried along the density's backtrace; `publish` lays `densityScale` / `fxDensityScale` and the tint into the atlas texel, so the two Tab rows act on smoke in the air (frozen too:
  `refreshDomains` → `publishAll`), every reader incl. the guards' probe takes the same `.x`, and the dust twin reads `shaders/dust_look.wgsl`. GUN GAS LETS GO (`fineDecay` 3 /s, `fineShare` 0.25; 0 and 1 = as benched; §9.8): a fine brick is a fifteenth of a coarse one's volume and nothing eats gun gas any more, so 1.7 s on AUTO
  held two thirds of the pool; now thin gas on the fine lattice and the heat in it fade faster, its floors are higher and its air ring smaller (`fineEps / fineHeat / fineRing`, live only while `fineDecay` > 0), and that lattice has a share of the pool that
  stretches to what the coarse one leaves under `softLo` (haze refused from 0.7 of it, all but sources at all of it; counted apart from a dry pool's refusals: `stats.fineHeld`). `tools/qa/sparse-reads.js` (`__spr.run()` = page loads / atlas fetches / march turns per ray hopping or not, `__spr.at(points)` = what a reader is handed). GPU time (the maintainer's Mac, 10-10, high 734 × 770, one run a solver; notebook §5, docs/perf/sparse-lab-*-mac-1010.json): solver ≈ 0.37 ms + 0.45 µs a brick, pool full 2.16 ms / +3.00 union against eight boxes' 2.83 / +3.68, one can in the open peaks at 2029 bricks = 1.31 ms, ≈ +1.5 ms of union over its box; the look: 'big plumes of smoke are sick' (the maintainer, the compound on hard, four cans). `tools/qa/sparse-lab.js` (`__spl.run()`, a fresh boot a run: 1 / 4 / 8 / 12 cans, then one can through its 32 s burn) is the bench; `__spl.round2()` flips each round-two lever in place on five standing scenes, then stages the dry pool, then a magazine on AUTO with the gun-gas rule and without. `__smoke.sparse` = its knobs. Round two on his Mac (10-10, notebook §9.7, docs/perf/sparse-lab2-*-mac-1010.json): solver 0.43 ms + 0.39 µs a brick that STEPS, twelve cans 1.66–1.70 ms / +2.2 to +2.6 union (round one 2.16 / +3.00), a can late in its burn 0.98–1.05 (1.25–1.31); 8192 = 81 % more smoke for +3.85 to +4.02 and 255.5 MiB, so the pool stays 4096; four cans at once already sit on the soft ceiling (~3,150 bricks). A solver-only run on a page that
  has rendered a frame still carries that frame's WIND records (`smoke.setWind([], 3)` first).
- **Prototype branches — NOT merged** (as of August, except the particle rain: `game/rainFx.ts`, `?prain`, `settings.particleRain` default off and the Frame's 832 B ARE in the build now; fire and water are still not in `src/`; each in its worktree with its own dev server; flag absent = main hash for hash, proven in its notebook; review page published as an Artifact):
  `exp/fire` (.worktrees/fire, :5216 `?pfire`; `game/fire.ts` nodes → a `'fire'` emitter kind through the solver's OWN temperature + buoyancy + an up-jet in the wind field, flames = blackbody
  emission inside `smokeMarchJ` behind prelude const `FIRE_ON`, two flickering 0.25 m sphere lights per fire, capped 3 live / 4 spreads, `hash01(serial)`; `FIRE` knobs Tab ▸ Sandbox ▸ fire,
  `__fire`, `tools/qa/fire-scenes.js` `__fs`, docs/internal/pfire.md, docs/review/pfire.html ← pfire-page.py; the maintainer: 'looks good', plume density 6 (c03b6f8); +3.8 ms union per fire at
  high is its LIGHTS + soot on shadow rays, not the flames; men do not fear fire; flammables keyed off cardboard albedo — wants a level flag) · `exp/water` (.worktrees/water, :5218 `?pwater` =
  the break room flooded / `?pflood` = the whole floor with sets through the frontage, rafts, drifting chairs; `game/water.ts` + `render/water.ts` + `shaders/water*.wgsl`, CPU-only
  `BoxFlag.Water`, shared-shader hunks inside `if (WATER)` blocks — identity rests on const-folding, re-hash after edits inside them; v2 @ baa2a84 per the maintainer's review: REST IS A STILL
  MIRROR, ONE dispersive (η, φ) field drives normal / displaced waterline / caustic (∇²η only), wind stress off `wind.at()` the only ambient source, characters lit in the reflection, a `drop`
  particle kind re-poking the field + spray + foam channel, throwables and the stun blast disturb it, wet particles damp and sleep, rounds die in water; `WATER` knobs Tab ▸ Sandbox ▸ water,
  `__water`, `__hotWater`, `tools/qa/water2-harness.js` `__w2`, docs/internal/pwater.md §v2, docs/review/pwater.html) · `exp/particle-rain` (.worktrees/prain, :5208 `?prain`; `game/rainFx.ts`,
  rain = particle kind 6 there in a THIRD ring over the kept lattice; v2 = drops lit as water (phase function + strand glint) refracting through the SAME refract_common.wgsl, wind-driven
  slant / gust sheets; `PRAIN` rows Tab ▸ Weather ▸ p-rain (+ 'v1 look'), `__rain`, `__hotRain`, prain-harness.js, docs/internal/prain.md, docs/review/prain.html; its Frame grows to 832 B
  there; verdict pending) · `exp/gunsmoke-particles` (.worktrees/gunsmoke, :5206 `?gunsmokep`) SHELVED — 'gun smoke as particles looks lame' · `exp/trace-perf` (.worktrees/trace, :5212) =
  the traversal variants V1‥V8 / H1, all negative (the lab + gatherStrata are on main) · `exp/kimodo`, `exp/sdf`, `exp/textures`, `design/second-space` = parked, awaiting the maintainer.
  Design docs on main awaiting one-line decisions: docs/design/level-scripting.md (levels as FOLDERS with a `script.ts` per level against a typed Director API — generator beats, name-bound
  marks / volumes with define-time lint, stimulus verbs that hand destruction a CAUSE never a result, showcase-only camera / puppet verbs — a workbench Director mode, M1 = the breach as a
  script) and docs/research/destruction-survey.md (glass that GIVES from a per-pane impact Voronoi, a fixture swinging on its cable, jointed furniture; beats scripted Seoul-style, breakage
  only from real rounds). NEXT per the maintainer ('later'): fire propagation + extinction by water / rain AS PHYSICS — species + Arrhenius heat release + droplet evaporation inside the
  existing solver domains (Fire-X, TOG 2025, itself wgpu / WGSL); water bodies stay heightfields, airborne water = particles / SPH-lite with a conservative hand-off (Chentanez & Müller 2010).

<!-- added 2026-10-13, after the cut -->
- **Wet ground breaks up into pools and damp aggregate** (`settings.wetBreakup` 0‥1, default 1; `?nowetbreak` = 0 = the frame as it was; Tab ▸ Materials ▸ '· wet ground break-up'; the small flaws
  goal, commit 2a41e5b5; PICTURE ONLY): under the storm every open ground slab took one finish (roughness 0.22, stretched), which the reflection pass drew as a near mirror across the view. The
  g-buffer now varies that roughness over the ground in world space (`detail.wgsl wetGround`, read in gbuffer.wgsl whatever the detail layer's amount): about +0.04 in the low spots, +0.15 on the
  stone between them. Dry gloss floors are untouched. The amount rides `frame.rtCfg.z` as the anisotropy + 2 × tenths (reflect.wgsl / reflect_p5.wgsl take the anisotropy back out: keep the two
  in step). The sender's claim, not re-checked here: seen only on the software adapter.
- **Lasers and tracers show in the ray-traced reflections** (`settings.beamReflect`, default on, acts only with `rtReflect`; `?nobeamreflect` = no segment handed over, the pass's frame as it
  was; Tab ▸ Materials ▸ '· lasers & tracers in reflections'; the small flaws goal, commit 29dd44b8; PICTURE ONLY): a beam is a NoShadow box no ray can meet, so the reflection pass never saw
  one. `engine.ts` hands `passes.setBeams(world.dynamics, thinIs, viewFocus)` the frame's thin lines (the boxes `game/thinline.ts thinNote` noted: combat.ts `beamBoxes`, tracers.ts) as line
  segments — up to eight, nearest the view — in a small UNIFORM at the reflection pass's binding 11 (not a storage buffer: the scene group's budget is untouched), and reflect.wgsl /
  reflect_p5.wgsl add each one's image where the mirror ray passes it, behind one occlusion ray. No beam on the frame = a zero count = nothing added. A beam is still never a light and still
  in nobody's eye or meter (the one-model rule of the IR lasers bullet stands).
- **Squad lasers are light in the air** (`settings.beamAir`, PICTURE ONLY, default ON; `?nobeamair` / Tab '· squad lasers as light in the air' = the glowing boxes exactly as before; debug views show the
  boxes; the maintainer, 10-13, floodlit at night in rain under the tube: "the lasers do feel a bit nonphysical", then "it's a bit bright given the brightness of the IR beam"). Cause: a laser was an
  emissive BOX floored at a pixel wide, which the tube's log curve clips to white: every beam one flat 4 px white bar end to end, its 2 cm bead lost in it, blind to rain and smoke. The game's boxes
  are untouched (`thinline.ts thinAir` marks a weapon laser's line and its bead; `passes.ts setBeams` hides them from the primary view through `BoxWorld.hidePrimary` and hands ≤ 8 segments to
  composite group 1 binding 15); `composite.wgsl beamAir` adds a Gaussian line (σ = max(2.2 mm, 0.55 px)) of the beam's light × an AIR SHARE, a dot where it lands (gain 3.5, r 9 mm, halo 0.07,
  not scaled by the air) and a glint at the emitter. Air share = `beamAirK` × (0.03 + 0.09 × haze/0.035, haze capped at 2×) = 0.12 by default, + 0.25 × rain and a sparkle on drops outdoors, + up
  to 3 in smoke (read per pixel). A line whose far end is under the ground is cut AT the ground and given its dot (the game's `laserReach` does not meet the yard's floor: such a beam had no dot
  in either look; the IR laser's owner's, not mended in the game). No `FROZEN_HOLDS` row and no journal line: nothing in a step reads it. MEASURED on the software adapter (SwiftShader, verylow,
  floodlit's storm, tube on, 5 lasers, live flips on one page): boxes = core 255, +79 over the wet ground, 94 % of the line ≥ 250; air share 0.565 (the first cut) = the same level, 69 % ≥ 250; air
  share 0.265 (about what ships in that rain) = peak 226, +54, 9 % ≥ 250 (the sparkle beads), dot 10–12 px at half height against a 2 px line. Pipelines 112/112, 0 GPU errors at verylow and high.
  NOT measured: clear air (estimate +25–40), the red visible beam, a beam across a depth edge, ultra / medium, motion (the sparkle's flicker). Never seen on a GPU: the maintainer's to judge.
- **Wet ground mirrors alike at every tier; a laser's mirror image follows the beam** (`settings.wetMirror`, PICTURE ONLY, default ON; `?nowetmirror` / Tab '· wet ground mirrors the men'; the
  maintainer, same shot: "we don't really see reflections of the characters"). Men ARE in the traced reflections (round-cone proxies in `traceClosest`, shaded with direct light and a shadow ray,
  the first 8 men only: `BoxWorld.addCharacterCapsules`), but `reflect.wgsl` gave the wet lot a dry finish's Fresnel (4 % from this camera) where the gloss path of low / verylow multiplies it by
  (1 + 2.5 × wetness) × 0.85, and the laser's image had a floor of its own (35 % × 1.5): a bright beam image beside men too faint to read. Now traced hits on `aniso` (wet) ground take
  min(F × (1 + 2 × wetness), 1) (the lamps' analytic glints are as they were), `beams.k.y` carries the 2 (0 = off: the arithmetic as it was), and an air-look laser mirrors at
  max(that, `BEAM_REFL_AIR` 0.25) × its air share, gain 1. KEEP IN STEP: reflect.wgsl ≡ reflect_p5.wgsl. MEASURED (SwiftShader, high, 6–12 frames of history only, 115–140 s a frame): the old
  beam image +11 over ground, the new one at the ground's own noise; NO man-shaped image readable with the switch on or off. So the men are 3× stronger than they were and still faint under
  the tube: dark wet kit (albedo 0.12) × 12 % against lit ground, then the log curve. Making them READ needs a drawn floor like the laser's had (a taste call, put to the maintainer), not
  more physics. Defenders past the 8-proxy table have no image and no shadow at all (a separate, bigger job). Never seen on a GPU.
- **The scene group's binding 11 is the fog of war's sight table, and `evalLight` asks it (10-14, `FOG.full` — `stealth.md`).** `common.wgsl`: `evalLight` =
  `evalLightRaw` (the old body) × `lightFogK` for a light whose kind word carries bit 25 / 26 (none while the rule or the fog is off: one `and` a light);
  `evalLightTrue` = the light as it is, for the game's readings (`probe.wgsl` alone). A NEW PLACE THAT WORKS A LIGHT OUT WITHOUT `evalLight` MUST MULTIPLY BY
  `lightFogK` (today: the glints in `reflect.wgsl` ≡ `reflect_p5.wgsl`, `cards.wgsl`, `froxel.wgsl`, the march's K-beam pick) — `probes.ts fog-full` reads the
  text for these and fails on any other file calling `evalLightRaw`. `fogSeenAt` walks the grid, so the VERTEX stage uses `fogSeenBins`. The table is a 58 KB (41 KB until 10-24, when the fog took five eyes; `FOG.eyes`, `docs/agents/stealth.md`)
  uniform (the 5th of 12 a stage may bind with the composite group's) written as storage by `fogsight.wgsl` at the head of the frame, bound through a second
  scene group (`sceneBGPlain`, a zero stand-in at 11). Sampled textures: the composite stage is at 15 of 16 (13 + the smoke atlas + the smoke page — the note
  at `passes.ts lComp` still says 14). Compiles on the software adapter (105 programs, 0 failed); never run on a GPU.

- **A shader module's text is cut to what it uses (2026-10-19, a play test: fifteen seconds and more of shader compiles on a fast Windows machine, "compiling the same shaders over
  and over", hitches while it does).** `src/gpu/wgslTrim.ts SHADER_TRIM` (ON; `?shadertrim=0` = every module whole, as it was — a switch of the page, not a rule of the game: no knob,
  no `FROZEN_HOLDS` row, both stands boot the same Engine). Every scene module is compiled behind `render/frame.ts scenePrelude()` — some 150 KB and 350 declarations in front of a
  kernel that may read a twentieth of it — and the head of that prelude holds the level's compile-time switches (the weather kinds, snow, the glass, the cut boxes), so a level of
  another family changed the TEXT of every module though most kernels never read the switch. `gpu/device.ts makeShader` (and the one boot module `passes.ts` makes itself, the dust
  volume's) now hands the browser `shaderText(code)`: the module-scope declarations an entry point reaches by name, in their order and character for character (comments aside);
  every entry point, `override`, directive and `const_assert` always stays, and text with no entry point comes back whole. Hot-swap and viewer modules are left whole.
  **Counted** (`tools/qa/shader-count-page.cjs`, the real page on the software adapter at medium, a walk of five pages: the title, the compound's kit picture, the compound, the
  block, the pass): the title is 66 modules, 12.1 MB of WGSL before and 3.3 MB after (73 % less); the same 150 pipelines either way (130 asynchronous + the three radiance-cascade
  ones before ready, 17 after), 118 of 118 counted in by the card, none failed, no GPU error on the console of any page. Module texts NEW to the browser on each page of the walk
  (not made, character for character, by an earlier page), before → after: the title 64 → 63 · the kit picture of a first mission 57 of 64 → 26 of 63 · that mission 2 → 2 · a
  second level of its family 0 → 0 · a snow level 58 of 66 → 13 of 65. Pipelines whose every stage an earlier page had made: 39 of 142 → 104 of 142 for the first mission, 41 of
  146 → 129 of 146 for the snow level. What still differs is what really reads a switch: thirteen modules read `WX_KINDS` (the title's office is the one family with it off), six
  to thirteen the snow's, one carries the level's own mesh-tree sizes. A quality change (medium → high on the title) made no module and no pipeline in the six seconds watched.
  `tools/qa/shadertrim-check.ts` (the Engine booted on a recording stand-in, whole and cut: same modules, order kept, every root kept, nothing left that mentions a dropped name,
  cutting twice changes nothing; a level whose weather switch differs is 10 new texts of 35 where it was 30; `SHADERTRIM_BREAK=1` must fail) — one Engine, so one stand's worth; it
  passes under `TPS_PROFILE=shipped` the same. `window.__boot.shaderText {modules, before, after, ms}` and `.slowest` (the six boot pipelines that came back last) are on the boot
  line; the benchmark's text carries them under `load`, with a `loads` row of every scene's own load (the first compiles cold, the later ones show what the browser kept).
  **The picture, cut against whole** (`tools/qa/draw-equal.cjs` on one build, the switch passed through its quality word: `--quality 'medium&shadertrim=0'`; 320×180, medium, the
  software adapter): the rain view and the squad view are EQUAL pixel for pixel at both checkpoints. One trap in reading it: the fog-of-war pass makes its pipeline on first use,
  so on the squad view the second drawn frame has that pass or not by how fast the compile came back (one cut run had it, a second cut run and the whole run did not: those two
  are equal); `&nofow` on both sides takes the race out, and is equal at both checkpoints.
  **Never seen on a GPU.** That a driver's compile gets shorter, that the browser's shader cache now answers a second level's modules, and that the hitches go with the text are INFERENCES — the next
  benchmark text from the Windows machine is the measure (`load`, `loads`, `longest freeze`).
- **A MODULE'S TEXT IS ALSO CUT TO WHAT ITS CONSTANT SWITCHES LEAVE ON** (11-03, the maintainer's pick on the join of the look: "Fold first, then join").
  `src/gpu/wgslTrim.ts SHADER_FOLD`, `foldWgsl` (ON; `?shaderfold=0` = no branch is cut, the trim's own rule alone, as it was; nothing is cut with the
  trim off; a switch of the page, not a rule of the game: no knob, no `FROZEN_HOLDS` row, both stands boot the same Engine). The scene prelude's switches
  are constants of the text (`const WX_KINDS: bool = false;`) and the kernels branch on them. The trim keeps a declaration that anything kept MENTIONS, so
  the dead side of such a branch kept everything it calls, and Chrome on Windows pastes every call into its caller before it drops the branch. WHAT
  BROUGHT IT UP: the look's prototype is switched at run time and two of its trials (hills that rays see, the surface try) stand behind such constants;
  joined with the look off, the page's programs came to 16 % more pasted text than the build's (29.9 MB against 25.8 in today's writing) and about thirty
  size rows went red. THE RULE: in a function's body an `if` statement is read whole, with its `else if`s and its `else`, and each test is read left to
  right with the module's bool constants in hand (`!`, `&&`, `||`, parentheses; a constant written as a test of other constants counts, `ROUND_RAY`): an
  arm whose test is false goes, an arm whose test is true becomes the statement's `else` and what stood after it goes, and where every arm before it went
  the statement is that block alone (a test may stand with or without its outer parentheses). What follows a statement that cannot be passed (a return,
  break or continue; a block or an `if` with an `else` whose every arm ends in one) goes too, up to a loop's `continuing` block, which runs after a
  `continue`: it never runs, and the browser's compiler warns of every such line (the first writing left them in: thirty warnings a boot, each one filed
  as a report). A `discard` IS passed: the function runs on after it and still owes its return. THE CONSTANTS STAY: a constant that decided a branch of
  a kept function is kept with it, as its mention kept it before the fold, so the text still says which switches it was cut under and the compile queue
  still finds the cut set by its line (`gpu/device.ts installPipeQueue` gives a module that holds `const CUT_BOXES: bool = true;` the background lane; the
  second writing dropped that line from all twenty and a review caught it). A function is folded when the trim first keeps it, not before (most of the
  prelude is kept by no one module: the cut of a boot's 64 modules took 67 ms with the trim alone and takes 87 with the fold, on a busy machine).
  LEFT AS WRITTEN: a test the constants do not decide BEFORE anything else is asked (`x && OFF` runs x
  first), a name inside a comparison or a call, a `select`, an `if` with an attribute in front of it, and a function that has a parameter or a local of the
  constant's name. MEASURED ON THE BUILD ALONE (`shaderinline-check --json`, the shipped page, no browser; text handed to the browser, then the same with
  every call pasted into its caller): the title, the page's own start (the compact set with separate shadow rays) 3.62 to 3.24 MB and 15.0 to 11.1 MB
  pasted, today's writing 3.56 to 3.23 MB and 25.8 to 20.2 MB; a snow level in its weather, the page's start 3.62 to 3.39 MB and 15.1 to 12.0 MB,
  today's writing 25.8 to 21.4 MB (the composite 977 to 620 KB). SINCE THE FOLD A KERNEL'S SIZE FOLLOWS THE SWITCHES, so `tools/qa/shaderinline-check.ts`
  holds its rows to two pages, the title and a snow level in its weather (every switch a level raises; the listing is the snow level's, the heavier; a
  second run of the file sizes the title's, where the arms a level's switch turns OFF stand), and its rows were brought level with that in the same
  change; what a word on the address adds (a trial, `?look`) is in neither, and `--setting look` lists that page. ONE BOOT FOR THE TOOLS
  (`tools/qa/pageboot.ts`): the page's boot over a recording stand-in, the Engine, the smoke solver, the Game's own modules and the cut-box set, with the
  switches `main.ts` raises, under one setting (`title`, `snow`, `look`) a process; the size check, the fold's check and the dump all read it (the
  second writing's checks booted a bare Engine with the page's switches down, so the page's own folded texts were neither compiled nor read).
  CHECKS: `tools/qa/shaderfold-check.ts` (hand-written texts of every form, cut as written by hand or left alone; the page's own modules under each
  setting: no test left that the constants decide, the dead side of every plain branch of the unfolded text gone and its constant still there, both read
  off the unfolded text and not by the fold, the queue's line still in every module that had it, nothing left standing after a jump, a second fold cuts
  nothing but the constants' lines, every root kept, nothing left that mentions a dropped name, no module bigger; `SHADERFOLD_BREAK=1` turns every
  constant over before the fold reads it and must fail; a fold that keeps the wrong side of a plain branch fails the dead-side line; run it on both
  stands). `shadertrim-check` and `shaderrows-check` read unfolded texts (their rules are of declarations as written). `bench-check` and `cardreplay`
  hold that a benchmark and a replay carry the word. `tools/qa/wgsl-compile.cjs` compiles every distinct folded text of the page's boot in the browser's
  own compiler, under each setting and with the shader rows as written and as the page opens (211 texts with the look in), and is red on a warning too. THE SAME FRAME: `tools/qa/draw-equal.cjs capture … --with shaderfold=0`
  against a capture without, then `compare` (the software adapter). NEVER SEEN ON A GPU, and no driver's compiler has read a folded text: what the
  software adapter draws is the same WGSL through another back end.
- **How big each kernel is once every call is pasted into its caller: `bun run tools/qa/shaderinline-check.ts [--top N] [--kernel <module>/<entry>] [--json f]` (2026-10-26).**
  Chrome on Windows compiles a pipeline's WGSL through DXC, which pastes every call into its caller before it optimizes, so a kernel pays for each CALL SITE of a heavy
  function, not for the text its module was handed. The tool makes that a number with no browser and no Windows machine: the page's boot run over a recording stand-in
  device (the Engine, the smoke solver, the Game's mesh shadows / particles / light probe / skinned raster, then `cutSwap.build()`; the shipped stand, cut boxes, the
  glass fracture and the dust twin on, as a plain address opens), every `createShaderModule` text and every pipeline descriptor kept, and each distinct (module text,
  entry point) sized by `pasted(f) = f's own text + for each call site in f, pasted(the callee)` in KB of WGSL as `wgslTrim.ts wgslDecls` reads it (comments out). It
  prints every kernel biggest first (pasted, the same functions once each, the ratio, boot or cut set, the functions pasted most into it), the totals of the two
  sets, the functions pasted most over all kernels, and the entry points nobody asked a pipeline for; `--kernel fgather/main` lists every function in one kernel with
  what ONE call site of it costs. Nothing in `src/` changed; no switch. **Today's kernels** (the same texts at 13164d24 and at the head this was written on): 64
  modules, 144 pipelines (119 the boot's — the card's 118 and the dust twin — and 25 the cut set's), 127 kernels; the boot's set is 13.1 MB pasted where 1.9 MB is each function once (×7.0), the cut set
  12.1 MB, both 25.2 MB. Biggest: the four composites 978 KB each (`gpHash` ×766, `fireHu` ×384), `fgather` `mainGi` 842, `main` 780 (62 KB once, its module 92 KB;
  `occluded` ×16 and `occludedTBoxes` ×15, the two counts the Windows measure named — its "~680 KB from 95 KB" was read another way and is not this tool's number),
  the dust twin 715, the volumetric march 667, `reflectP5` 639, the particle update 585, direct 405, restir 347. Over all kernels: `isectBox` ×1462 copies (1.9 MB),
  `gpHash` ×7268 (1.6 MB), `isectCut` 1.2 MB, `occludedTBoxes` ×389 (1.0 MB), `occluded` ×284 (0.8 MB). **On the tree with the one-call-site rewrites** (the
  Windows pull request's head, 71a73c1c; the tool copied into a checkout of it): the boot's set 13.1 → 8.2 MB (×0.63), the cut set 12.1 → 7.3, both 25.2 → 15.5.
  Per kernel, pasted KB before → after, beside that tree's own offline DXC times: `fgather` `main` 780 → 228 (×0.29; DXC 7725 → 2223 ms, ×0.29) · `mainGi` 842 →
  294 (×0.35; ×0.40) · `reflectP5` 639 → 243 (×0.38; ×0.34) · composite 978 → 574 (×0.59; ×0.65) · direct 405 → 260 (×0.64; ×0.63) · particle update 585 → 361
  (×0.62; ×0.62) · restir 347 → 308 (×0.89; ×0.89) · reflect 274 → 199 (×0.73; ×0.79) · sharpUE 222 → 126 (×0.57; ×0.58) · the volumetric march 667 → 455 (×0.68;
  DXC ×0.30) and its dust twin 715 → 503 (×0.70; ×0.46). So the pasted size tracks what DXC took out of a kernel within a few points, except the two marches,
  where the rewrite turned repeated statements into turns of a loop and DXC gained more than the text did. That tree's boot going 26 s → 9 s is more than its text:
  its LATER twins leave the boot's wait too, and this tool counts a pipeline in the set it was ASKED in, not when a queue hands it to the browser. **Held against
  the real page** (`shader-count-page.cjs --json`, the title on the software adapter at medium, its text hashes beside this tool's `--json`): 62 of the page's 65
  module texts are this tool's character for character; the page also makes the outline's and the save picture's small modules (first use, no tracing), and the one
  mesh-shadow module differs (it carries the character mesh's own sizes); the page asked 146 pipelines, 118 of them counted by the card, this tool 144. **The
  budget** (the table at the head of the file): each kernel over 200 KB has its own row within a twentieth of today, every other kernel 200 KB; red names the
  kernel and both numbers. A kernel that grows on purpose has its row raised in the same change, a new big kernel gets a row, a row with a twentieth to spare is
  printed to be lowered, and a row no kernel answers to is red — on the rewrites' tree it is red for the one new kernel (`volumetricsF / main`, 455 KB, no row) and
  names eleven rows to lower, which is the edit that tree's landing owes this file. (That tree's kernels are in this one as the row Compact shaders, below: `TPS_COMPACTSHADERS=1` in front sizes them against a second table of their own, `BUDGET_COMPACT`, and today's rows stand as they are.) A second line is red when the recording is short of a boot (under 60 modules,
  118 boot pipelines or 24 of the cut set's), so a part of the page that falls out of the stand-in's boot cannot leave the budget green on the rest. `SHADERINLINE_BREAK=1` writes one more call site into the biggest kernel, of the lightest function it reaches whose
  call site is a twentieth of it (today `fireAir`, 76 KB, into `compositeS / cs`: 1054 of 1010 KB): FAILS, as it must (it would not while that row had a
  twentieth to spare, and the plain run names such a row). Two seconds a run; `typecheck:qa` clean. The boot it runs is main.ts's by hand (the six inits, the
  switches a plain address writes before the Engine): a GPU module main.ts gains is not sized until it is added here — the page's hashes are the check on that.
  **What it is not**: a time (DXC's grows faster than the text; a loop round one call site is one copy here, as there, whatever the loop's body costs); a list for
  another level (a snow level or a picked weather turns consts in the prelude: other texts, never sized here); a count of first-use pipelines made after the boot.
  **Nobody has run it beside a Windows compile of the same tree**: the agreement above is with the times the pull request's own notes give.
- **What is left of the shader wait, and needs a GPU to do (2026-10-19).** (1) Compile first what the opened quality draws (2026-10-26: the twins no first frame draws are LATER — the Windows bullets below): the boot waited for all ~133 pipelines though a tier
  takes a part of them (the legacy and v3 denoiser chains, every à-trous variant, both composite kinds are all made). The count tool prints which boot pipelines a pass took in the
  seconds after ready, but on the software adapter a frame is half a minute, so its list is a floor, not the set — the split needs a machine that draws, and every pipeline left
  for later needs its pass to stand down until it is in. (2) Module creation is one burst: sixty `createShaderModule` calls in one task, parsed back to back on the browser's GPU
  thread (the likely "longest freeze" of the benchmark's load row; the cut makes that burst a quarter of the text); spreading them over frames means an `await` between passes in
  `Engine.init`. (3) `render/rc.ts rebuildRc` makes its three pipelines synchronously at boot and at every quality change (identical text: the implementation's cache answers a
  repeat). (4) The Game's own build is one block of the page's thread (0.9 s here in the browser): the mesh shadows' BVH and the reach derivation are most of it
  (`render/meshShadows.ts`, `game/reach.ts`), with `math/hyp.ts`'s trial at module load beside them; the benchmark's `game built in` says what it is on a real machine.
- **The kept page: shaders compile once a visit (2026-10-27; `src/gpu/keep.ts`, `src/host/keeppage.ts`; the maintainer: "compiling shaders once would be
  great").** A mission start is a page load (a pick, the loadout's Deploy, Quit to the list, a tape's page), and the GPU device goes with a page, so every
  mission compiled the renderer's programs again: on the Windows machine of the hosted build's records 8.9 s of a 9.5 s load, each time (Chrome keeps only a
  few megabytes of compiled shaders between loads, so a reload answers few). NOW the page a visitor opens is a SHELL: it owns the device and shows the game
  in a same-origin frame of its own address that fills the window (`showKeptFrame`). Every load the game asks for is as it was written and loads the FRAME,
  so a mission is still born in a JavaScript world of its own: no module state, timer, listener or record of the last one, by construction (this is why the
  game is not torn down and rebuilt in one page: 140 side tables and every flag read at boot would each have to be put back by hand). What stays is the
  shell's: the device, handed to each load on a LEASE (`Keeper.take`, `initGpu`'s `keeper`), and on it the things that never change once made. The device's
  `createShaderModule`, `createBindGroupLayout`, `createPipelineLayout` and the four pipeline makers answer a descriptor they have answered before with the
  same object (keyed by the whole descriptor as JSON, a GPU object in it by the number it got when first seen: so a pipeline asked for through a layout the
  new load made anew is still the first load's pipeline, because that layout is the first load's layout). **Nothing moves into play:** a load asks for what
  it always asked for, before its first frame, and its card counts them in as ever; a program the visit has not built (the registry's `volDust`, another
  level family's text, another weather's) compiles behind that mission's load as it always did. One path on every platform. A LEASE ENDS with its page
  (`pagehide`, or the next `take`): the buffers, textures and query sets it made are destroyed there and then (listed weakly as they were made: two missions'
  memory is never held waiting on the collector), its listeners on the device are taken off, an error scope it left open is closed, any method it put
  over the device's own is deleted (the compile queue's `installPipeQueue` wraps the keeper's makers afresh each load and never stacks), and its `lost`
  was a promise of its own. **A thing the device called invalid is not kept:** a module whose `getCompilationInfo` has an error, a layout or a pipeline
  made at once that raises a validation error (the keeper makes each NEW one a second time inside an error scope of its own to ask; the page's scopes and
  listeners hear the first as ever), a pipeline whose compile failed and the stand-in its caller then makes at once. The next load asks the device again
  and hears what a whole page hears. At most 4096 things are kept on a device; past that a new one is made and not kept. **A compile still in
  flight when a load ends does not hold that load:** each ask is handed a promise of its own, reached from the compile only through the lease's table,
  which the lease's end empties (`settle`; so is the lease's `lost`). Found with a heap snapshot: without it the 24 pipelines the page asks for behind its
  first frame held the whole gone mission (its `Game`, its document) until they landed, two minutes on the software adapter; the next load's ask for the
  same pipeline still joins the one compile. **One device is one queue (2026-10-28):** the frames a mission had submitted when its page went are still
  the GPU's to do, where a whole page load drops them with its device. A load that took the device at once then FROZE at its first big upload until the
  GPU had caught up: seen on the software adapter as 72 s and 32 s inside the Game's constructor on the fourth and fifth start of a tab (a CPU profile
  put all of it in one native call, `queue.writeBuffer` under `attachAir`, with the queue 75 s and 35 s behind at the pick; the same starts took 1.3 and
  1.8 s once the queue was let run dry first). So `Keeper.take` waits for the queue (`drained`: `queue.onSubmittedWorkDone`) before it hands a used
  device over: the page is live while it waits, and the wait is the load's device time in its record, said again as `waited` on kept 2 and up (no
  word = it did not wait). A slow GPU's queue is waited out, since past any cap the load would only freeze for the rest; a device LOST in the wait ends
  it and the load is handed a device opened for it; `Keeper.drainMs` (five minutes) is only against a queue that never answers. Two loads at once: the
  later one holds the device and the earlier one's `take` is refused. It costs a start whatever its page could have fetched and parsed meanwhile (the
  wait is in front of the whole hand-over, because the first big upload anywhere is what froze). On a real GPU the wait is however far the queue is
  behind when a mission is left: the records will say. `?keepdrain=0` on the tab's first address = the device at once, as the kept page first was.
  **The load's clock on its GPU calls (2026-10-28, `startCallClock` in `gpu/keep.ts`, EVERY game page, kept or whole):** the page's time INSIDE
  `createBuffer`, `createTexture` and the queue's `writeBuffer`, `writeTexture` and `submit`, from the moment it has a device to the boot's freeze, is in
  its record as `calls` { n, ms, mb, worst, what, by: { call: [n, ms, mb] } } (`KeepCalls`; `__keptPage.calls` on the page). It is there because the first
  read on a GPU had the game's build step at 2.5 to 3.4 s where the same Mac had it at 0.25 s a day before on a whole page of an older build, with parse
  and the first frame's encode unchanged: a page thread that waits on the GPU process waits inside these calls, and nothing else in the record can tell
  that from script. So a kept load and a `?keeppage=0` load of one machine now compare by `calls.ms` beside `spans.gameCtor`. Software adapter, the
  title at verylow: 119 calls, 16 ms, 12 MB. The five calls are put over the device's and the queue's own in the page's window only while the clock runs
  and what was there is put back at the freeze (on a kept page the keeper's makers), so the load's clock is off every call after it (the play total, next, then goes over the queue's three); a kept load that ends
  first loses them with its lease (`dress` strips the queue as it strips the device). `?callclock=0` on the page's address = not timed, no word. Never
  seen on a GPU. **The play total (`CALLS.play`, 2026-10-28):** from the boot's freeze on, the queue's `writeBuffer`, `writeTexture` and `submit` stay
  under a lighter clock (two reads of the time a call, every argument named, nothing made per call) that keeps running totals { n, ms }: a frame's time
  inside the queue's calls is the difference of two reads of `CALLS.play.ms` (`__keptPage.play` on the page). It is for the minute's perf record (the
  cloud's to read: `cpu.render` there is wall time and holds these calls). It was added for a round on that Mac whose frames took 47 ms with 10 ms of
  GPU work: a page blocked on the GPU process shows as a large share. A SMALL share proves nothing: a page also blocks inside an encoder's call when the
  command buffer to the GPU process is full, and those are not timed (too many a frame), nor is a wait in `createBuffer` or `createTexture`, and the total has no bytes, so a big copy and a
  blocked call read alike. The wrappers stay for the page's life (`CALLS.endPlay` takes them off, and a clock started again does); a kept page's go
  with its lease, and a kept load whose lease is over by its freeze puts none. A tool that puts its own function over one of the three and later
  takes it off with `delete` takes the total's off with it: the total then stops counting that call and says nothing. `?playclock=0` = no play total (`?callclock=0` puts both off). Never seen on a GPU.
  A device really lost is dropped: the frame's retry (`GPU_LOST`) loads the frame and the next lease opens another. The address
  bar follows the frame (on each frame load, and on a `history.replaceState` / `pushState` the page makes), so a reload or a copied link is the mission's.
  **Where it is off** (`keepRole`): `?keeppage=0` = every load a whole page, as it was, kept for the tab in `sessionStorage` (`twopointsix.keeppage`;
  `?keeppage=1` takes it back, and asks for the shell where it is off by itself). OFF BY ITSELF, `?keeppage=1` asks: on a page a script drives or steps
  (`navigator.webdriver`, or `?paused` on the address: EVERY PAGE TOOL, and the verify recipe's page, loads the page as it always was and reads the page's
  own window), and in a browser that is not of the Chromium family (no `Chrome/` in its name: a frame drawing with its parent's device has been seen
  nowhere else, and a browser that took the device and drew nothing would be a black page with no error to catch). ALWAYS OFF: under `?bench` and on the
  benchmark's scene pages (their loads are what it times: a frame sent to one sends the tab there, `leave`), in the kit picture's window (a frame of the
  game's page, with a device of its own as before) and on the one-file page. A frame under a shell of another build (a deploy mid-visit: the module
  scripts the two pages name differ) sends the tab to its address, once a minute at most (a tab whose two pages can never agree plays on a device of its
  own). ANOTHER PAGE of the site asked for in the frame (the replay viewer, the workbench, the listening pages) is loaded by the TAB (the frame stops the
  load with the Navigation API and the tab makes it; without that API the shell sees the frame land there and follows): such a page has the address bar
  to itself, and coming back to the game from it is a new shell and one more compile. The keys are the frame's own window's (`KEPT.shelled`,
  `game/input.ts`: no sink text box as in somebody else's frame, so F7, F8 and a paste into a field work as on a whole page). A browser that will
  not draw into a frame's canvas with its parent's device throws at `configure`: that load and the ones after open their own device (`Keeper.refused`, one
  warning record `gpu.keep.refused`). **The load's record** gets `kept` (which load on this page: 1 = the one that compiled, 2 and up = a start on the kept
  page; absent = a plain page), `made` { pipelines, modules, asked, and on kept 2+ the first labels compiled }, counted up to the boot's own end
  (`keptFreeze`, beside the card's own freeze) and on kept 1 `shell` (ms the shell's page took before the frame: the record's clock is the frame's, so the
  true first load is `whole + shell`) and `shellNav` (how the TAB was loaded: the frame's own `nav` is always a plain navigation, and `net` counts the
  frame's downloads), on kept 2 and up `waited` (ms the load waited for the last one's queue). `calls` (the load's clock on its GPU calls, above) is on every game page's record, kept or whole. Hooks: `__pageKeeper` on the shell (`leases`, `opened`,
  `refused`), `__keptPage` on every page (`role`, `lease`, `calls`, `play`). No knob and no frozen-stand row: a switch of the page (the harness has no page), like the
  compile queue. **Checks:** `bun run tools/qa/keeppage-check.ts` (the keeper over a stand-in device: one compile for equal descriptors within and across
  leases, the lease's end, what is invalid is not kept, a compile in flight at a lease's end (the next load gets it, the gone one hears nothing and what
  it hung there is collected), a lost device, the wait for the last load's queue (its cap, its switch, two loads at once, a device lost in it), `keepRole`'s
  table, the record's words, the load's clock on a whole page's device and on a kept one's (what it counts, what it leaves at the freeze and at a lease's
  end, its switch);
  `KEEPPAGE_BREAK=1` = nothing kept, no wait and no clock, must fail) and `PORT=… node tools/qa/keeppage-page.cjs` (headless Chromium, the software adapter: a pick and a reload on the kept page against a whole load
  by digest, Restart / checkpoint / Quit / pick again, the records at a stand-in backend, leaving for `?keeppage=0`, who is a shell by itself, the replay
  viewer taking the tab, the load's clock on a kept page (`kept`) and on a whole one (`stepped`, `driven`), and `queue`: a playing page loaded again with the GPU behind waits for the queue and not inside the game's build step (35 s behind, waited 79 s, built in 0.8 s; it proves the wait, not the freeze: its mission built in 1.1 s under `?keepdrain=0` with the queue still behind); `JOBS=` picks jobs, `KEEPPAGE_BREAK=1` opens under `?keeppage=0` (`queue` under `?keepdrain=0`), must fail; a whole run is about 25 minutes there).
  **Measured, software adapter only (2026-10-27, verylow, `?paused`):** the depot's room clear opened first: 144 pipelines and 66 modules compiled, 164 to
  172 s to the start card; the registry's rescue picked on the same page: 1 of 145 asked compiled (`volDust`), about 2 s to its card; its reload 0 of 145;
  the tab's document the same one throughout (one top-level load, one device opened). The second mission's game after 120 frames is the game a whole load
  of that address plays, by digest, and the first mission's `Game` is collected once the collector runs. **Costs and seams:** the first load parses the
  bundle twice (the shell's page runs the same script and stops at the top of `main()`; `shell` in the record is what that costs on a real machine); the
  kit picture still compiles its own copy of the shaders in its own window (it could take a lease too: two pages on one device at once, not built); an
  uncaught GPU error raised by the last load's work after its lease ended would be reported by the next; in a browser that does not make equal pipelines
  one (Chromium does) the keeper's second making of a pipeline asked for AT ONCE is a second compile (the game asks for nearly all of them the waiting
  way, which is never made twice). **First read on a GPU (2026-10-28, the hosted build's records: an Apple M4 Max, Chrome, three starts in one tab):**
  the title compiled 119 of 119, the pass road after it 26 of 120 (the snow level's boxes and the haze Medium uses), the tunnel after that 0 of 119.
  The starts were still 5.9, 6.2 and 4.8 s, because the Game's constructor was one task of 2.6, 3.0 and 3.4 s on that machine. That is NOT explained:
  on the software adapter the same step is 1.2 to 1.9 s on a kept page, all of it the game's own work by CPU profile (no keeper frame in it, a whole page
  no faster), 0.3 to 0.5 s warm over the stub device, and another Mac's whole pages had no task over half a second the day before. No load of that
  machine with `?keeppage=0` has been read, so the kept page is neither cleared nor blamed for it. The queue wait above was written after that read
  and has NOT been seen on a GPU. Otherwise **never seen on a GPU**, and never in a browser other than Chromium: that a frame may draw with its parent's device is
  how Chromium behaves, proven here on the software adapter; Safari and Firefox keep a whole page per load until somebody has seen `?keeppage=1` draw
  there (the refusal path is for a browser that throws; one that draws nothing has no path), and the seconds saved on a real machine are the Windows
  records' to show.
- **Windows: why the shader wait came back on every load, and the compile queue (2026-10-26, measured on a Windows machine: RTX 3080, a 16-thread desktop CPU,
  Chrome 155, a 4K 240 Hz display at 150 %).** Chrome on Windows runs WebGPU on D3D12 and compiles with DXC (`use_dxc` is on; FXC, `--disable-dawn-features=use_dxc`,
  had not finished the boot after 16 minutes). DXC inlines every call, so the big kernels are 5–25 s each (fgather's `main` grows from 95 KB of WGSL to ~680 KB inlined:
  `occluded` ×16, the fog's `occludedTBoxes` ×15 through `evalLight`): 118 s of compiles for the title's boot one at a time, 26 s on the card (15 workers), the
  compound and the pass 30–60 s. (1) **Chrome's cache never answers it.** The compiled blobs ARE written to disk (`DawnWebGPUCache`, ~44 MB for the title), but the GPU
  process keeps them in an in-memory LRU of 6 MB (`DawnCachingInterface` → `GetDefaultGpuDiskCacheSize`, `kDefaultMaxProgramCacheMemoryBytes`), so the set thrashes it:
  a trace of a warm load showed 131 DXC compiles and 12 cache hits, and a reload in the same tab took as long as the first load. `--gpu-disk-cache-size-kb` raises the
  size but is not passed to the GPU process; with `--in-process-gpu` it is read, and a warm load is 1.5–2.3 s to interactive (pipelines 0.7–1.2 s) against 26–60 s.
  The game ships no launcher and asks for no switches: plain Chrome is what it is played and measured in, so those two words are a measuring aid only. On a Mac the same 6 MB thrash happens, but the Metal compiler is fast and keeps its
  own cache. `d3d_disable_ieee_strictness`, `disable_robustness` and `d3d_skip_shader_optimizations` change nothing worth having (the GPU union within 0.2 ms; skipped
  optimizations make DXC slower, 43 s). (2) **The burst starved the browser's own drawing.** Chrome makes every …Async pipeline on one worker pool in the GPU process,
  and Skia Graphite (the page's HTML: HUD, cards) makes its pipelines on the same pool. The cut-box set compiling after the boot held all 15 workers for 10–25 s, Graphite's
  pipelines asked for meanwhile waited behind it, and the raster's flush blocked (traces: 3.1 s, 6.7 s; a 15 s frozen picture in one take; the fog pass's own first-use
  pipelines waited too). `gpu/device.ts PIPE_QUEUE` (a switch of the page, ON ONLY IN A BROWSER ON WINDOWS, where it was measured: `?pipequeue=0` = straight to the browser as before,
  `?pipequeue=1` = on wherever the page is, which is how a Mac or a page tool tries it; no knob, every stand compiles the same programs): at most threads − 2 in flight at boot, biggest module first; after the boot (`pipeQueue.settle()`, main.ts) at most 4, of them at most 2 of the background
  set (modules behind `const CUT_BOXES: bool = true;`), the foreground first. **Measured** (`?bench`'s first floor, plain Chrome, cold, full screen at 4K, 40 s from
  interactive, presented frames from the trace's `Display::DrawAndSwap`): without the queue 69.6 frames/s presented, the longest gap 3.5 s, 11 gaps over 50 ms; with it
  76.5/s, the longest 163 ms, 5 over 50 ms; boot 32.5 → 30.3 s (one run each, a second pair agreed: a 3.6 s freeze → 133 ms). With those two switches (not shipped), warm, with the queue: boot
  1.6 s, 76.9/s, the longest gap 130 ms. The frame itself is the GPU's on that floor (union 13–18 ms at `high`, the ladder down to scale 0.5; `low` pinned: 9.9 ms,
  97 fps); the calm office at high is 10.6 ms at 1507×780, about the M4 Max reference above. Never measured on a Mac with the queue on (the boot order changes there too), which is why a Mac does not get it. On the software adapter the page tools run on, with the queue on, the boot made the same 146 pipelines with no GPU error (118 of 118 awaited), but the background set was not ready 21 minutes after the page was, where with the queue off it was ready 389 s after (one run each): two at a time is far slower there, and six page tools wait for that set. **The check**:
  `bun run tools/qa/pipequeue-check.ts` (the queue's own logic over a stand-in device, no browser: the limits, biggest first, the background cap, a failed, thrown
  or broken creation gives its slot back; where it is on is one pure function, `pipeQueueFor`; `PIPEQUEUE_BREAK=1` fails twenty-one, with the later lane's lines below, the same on both stands). A pipeline's time in `pipelineProgress.times` (the boot
  line's and the benchmark's "slowest") is counted from its turn in the queue, not from the asking, or biggest-first would name the smallest shaders the slowest.
  Read in Chrome's and Dawn's source the same day (not run): the 6 MB is one cache for a whole Chrome profile, every site together, oldest out first; on D3D12
  both the DXC bytecode of each stage and the finished pipeline's blob go into it; on Metal only the generated MSL text does, and the compiled pipelines are kept by
  the system's own Metal cache outside Chrome's 6 MB, which is why a Mac's second load is cheap. **Known, left as they are:** the cut-box set is ready later than it
  was (two at a time), so a wall cut in the first seconds draws whole for longer; the Team & kit picture is a second page with a queue of its own, at its boot limit
  while the menu is up, so that step can still freeze on Windows; a 4-thread machine's limits (4 and 4) are no smaller than its pool; `?cutprograms=boot` marks the
  live set background (the text sniff); the workbench never calls `settle()` and stays at the boot limit; `__hotCommon`'s build ms includes the wait in the queue.
- **Windows: the cold compile cut from ~26–30 s to ~9 s (2026-10-26, the same Windows machine; Chrome 155, D3D12, DXC 1.9.2607 as Chrome ships it) — BOTH LEVERS ARE ROWS ON
  TRIAL NOW, OFF BY DEFAULT (the next bullet): every number here is with both on, on the tree they were written in.** Measured with a
  browser already started (a fresh profile's first navigation waits 2.5–4.6 s on the browser's own start-up before it asks for the page: not the page's) and an EMPTY shader
  cache, `__boot` to interactive, cold every time: the title 26.0–26.5 s → 8.4–10.4 s (pipelines 25.0–25.4 → 7.4–9.4 s; four runs of the final build's shaders), the
  compound's room clear 29.4–29.7 → 8.6–8.7 s (three each), the pass (snow) 29.9–30.0 → 9.8 s (two each; one run in four of the same build takes a few seconds more — the
  GPU process's own scheduling).
  **Why it was slow**: DXC inlines every call into one function before it optimizes, so a kernel pays
  for each CALL SITE of a heavy function (the tracer, `occluded`, `evalLight` with the fog's walk) — fgather's `main` reached ~680 KB of inlined text from 95 KB of WGSL —
  and its time grows faster than the text. A small loop round a heavy call is NOT unrolled (tried: an opaque trip count changed nothing), so a loop of turns round ONE call
  site is the tool. **Two levers**: (1) THE ONE-CALL-SITE REWRITES, each the old statements in the old order (the same `rnd()` draws), so the picture is the same
  bits: fgather `main` and `mainGi` trace every ray (the gather's, the young tile's extras, the fresh candidates, the gloss floor's and the pane's mirror) through one
  `traceShade` in a loop of turns; `directAtPoint` / `directOnScreen` are one `directAt` with one `evalLight` and one `occluded` (also reflectP5's two-part hit and
  `giReshadeDirect`); reflectP5 takes the two halves and `traceClosestMesh` on every path (bit for bit `traceClosest` with its bits down); the volumetric rain walks its
  open-air pieces through one `rainAlong`, the sun and a lamp share one beam march, the fog's three looks and the sun's two rays are turns; the composite runs its beams /
  fire / volumetrics / veil tail once for sky and surface and the glass cracks draw one line list through one `gbEdge` (max / min only, so order-free); the particle update's
  three light walks are one `pLight`, and the drop's crown and head, the snow grain and the wisp share one `rainShadeV`; the direct pass's leaders and tail, the sharp
  pass's edge and sliver rays, restir's glass and target reads, the rig normal's four taps likewise. (2) LATER TWINS (`gpu/device.ts laterPipelineAsync`,
  `PIPE_QUEUE.later`, the row Start on first frame shaders; off = the boot waits for every one as before; a switch of the page, no knob): the boot's set leaves out what no first frame draws with the
  default settings — restir's motion twin (restir meanwhile), the iid gathers and the resampling pair (the strata pair / the plain gather meanwhile; the reservoirs start clean
  as when the row is switched on), reflectP5 (main's reflections), the old direct estimator (restir), the dust twin (plain smoke), the fragment composites (sharp direct is
  held live until one is in — engine.render, even at scale 1), the non-refracting compute composite (compSR, the same picture with no pane crossed), the froxel kernels and
  the volumetric pass's froxel twin (FrameFlags.FroxelVol held down: the per-pixel march meanwhile) — and compiles them after it in the queue's background lane; each lands
  through its cut set's `patch`. The K = 0 smoke march, which no preset runs, lives only in a twin built the first frame `volLightCap` is 0 (`volumetrics.wgsl VOL_K0`).
  **Offline DXC, per kernel, the title's text (ms, one at a time; HLSL dumped from Chrome with `dump_shaders`, timed with Chrome's own dxcompiler.dll)**: volDust 9268 → 4274,
  fgatherGi 9079 → 3658, vol 8626 → 2560 (+ the froxel twin), fgather 7725 → 2223, reflectP5 7396 → 2514, compositeS/SR 5265 → 3400, direct 4072 → 2579, particlesUpdate
  3488 → 2164 (the pass's 5914 → 3678), restir 3181 → 2847, reflect 2422 → 1906, sharpUE 1734 → 1009; the queue's 15 workers get ~4.5× out of the 16 threads (DXC alone
  scales 6× at best), so the boot is about its set's total ÷ 4.5. **The picture** (a GPU twin of `tools/qa/draw-equal.cjs`: its five views, plus the pass and the compound on
  their own camera, with `&cutprograms=boot` so the cut rule is in every program, every LATER twin waited for): EQUAL pixel for pixel on D3D12 against the tree before at
  high (all seven), medium (the five) and ultra (the five and the compound); the pass's second checkpoint at ultra differs only through the frames drawn before the twins
  landed (the resampling gather's reservoirs start clean when its twin lands, where the tree before carried the boot frame's) and is EQUAL with every twin waited for. A check WITHOUT `cutprograms=boot` must not wait for the queue: the cut set landing mid-capture changes the firefight view by design. **The
  frame** (the compound's room clear at ultra, scale 0.75, warm): union 13.65 → 13.85 ms (gather 1.37 → 1.50, compS 0.55 → 0.66; the loops' registers). **Not seen**:
  any of it on Metal (the rewrites are the same operations, but a fast-math compiler may schedule them to last-bit differences, as reflect_p5.wgsl's history notes), and the
  fallback frames themselves (a `?nogatherstrata` / `?norestir` / `?norefract` visit, ultra's phase-5 pair, a froxel tier, scale 1 — for the seconds before their twin is in).
  Left: vol's two smoke marches (the classic and the froxel-lit one are one march apart from their light — ~1.2 s of each program), the rain splash's second `rainLight`,
  restir's two `traceSample`s across its barriers, and the page script's own ~0.7 s.
- **START ON FIRST FRAME SHADERS STARTS ON IN EVERY BROWSER A PERSON SITS AT (2026-10-27, the third row that starts on, `on: 'person'`; the maintainer:
  "should we do background compiles on all platforms? i'd generally prefer to have a single path when possible", and "i think we shouldn't do truly lazy
  compilation where then have stutters during gameplay").** (10-28: a mission's brief no longer goes live on that card's terms. Its Start waits for the
  later set, the cut set and a drawn, measured picture, `docs/agents/menus.md` "The brief's Start waits for the picture", so what this row still
  buys is the title up sooner and the rest compiling while the menus are read.) The start card waits only for what the first frame draws and the LATER set (the bullet "Shader rows
  on trial" below lists it: the gathers' later twin, the fragment composites, the froxel kernels, reflectP5, direct, restirM, and volFroxel with the compact
  set) is asked for at the boot as before and compiled behind the card, each pass on its stand-in until its own is in. Nothing is built on demand by it.
  **Off:** `?firstframe=0`, or the row in Options (kept off). A load as it was is now THREE words: `?compactshaders=0&splitrays=0&firstframe=0`.
  **WHERE IT STARTS OFF:** the harness (both stands, as before; `TPS_FIRSTFRAME=1` raises it) and A BROWSER A SCRIPT DRIVES (`navigator.webdriver`, so every
  page tool): some forty tools poll for a hook and begin at once, and their first picture and first timing must be the settings' own. A tool that wants the
  player's start asks for it (`?firstframe=1`) and waits on `__ready`. The Team & kit picture (`?kitview`, a still drawn once) and the workbench (debug views,
  no stand-in) lower `PIPE_QUEUE.later` before they build, whatever the row says. **What moved with it:** (1) `window.__started` is the moment the card is
  live; a script's `__ready` waits for the later set too (`gpu/device.ts laterIdle`, a tick after the count reaches 0) and `__boot.marks.laterReady` is when
  it was in (`docs/agents/qa.md`). (2) The benchmark waits while `Engine.standsIn` (`ui/bench.ts standing`, 180 s in all on a page; past that its lines are
  timed on the picture as it is and the report says so once). (3) Auto quality's first measure waits for the later set, 30 s of frames at most
  (`quality.ts STANDS_IN`, the ladder's own cap; before, with the row off by default, that wait had no end), and a page that hides the view is not drawn
  behind while it waits (`Quality.measuring`, main.ts `viewHidden`). **What it rests on:** the headless arms (`shaderrows-check` `first` and `both`: the later
  set out, the stand-ins, all of it in, `laterIdle`; its `driven` case and the wiring pins for main.ts and the workbench), `quality-plan` (D1's two lines on
  the boot's measure, `LADDERHOLD_BREAK=1` must fail) and the page walks on the software adapter (`shaderrows-page.cjs starts`: a person's default load from
  the moment the card is live until every later pipeline is in, then the page as a tool opens it). **What nobody has:** any time on a real GPU for this row in
  this build. On its author's Windows machine the row's first writing ran 2 to 5 s at about 5 fps some 8 s after the start; the likeliest cause was mended when
  it came over (the restir stand-in, "Shader rows on trial" below) and nobody has gone back to see it gone. How long the stand-ins draw on a Windows machine
  and on a Mac is not measured, and the load's record says it only by luck: `laterReady` is among the marks it carries only when the set was in before the record was made
  (`cloud/loadtime.ts` waits for `gpuReady`, not for it). A player sees the picture change when
  the later set lands (sharp direct to the fragment composite, the per-pixel march to the froxels). **Known and left:** (a) on Windows the later set goes
  ahead of the wall-cut set in the compile queue's background lane (two at a time), so the cut set is ready later than it was and a wall cut in the first
  seconds draws whole for longer; (b) a debug view or TAAU off while no fragment composite is in draws black until one lands (no preset and no Options row
  does either); (c) what a page tool draws is no longer the first seconds a player sees: only `shaderrows-page.cjs` looks at those; (d) a later pipeline that
  fails is one error record (the console's line) and its pass keeps the stand-in for the visit, counted nowhere in `__boot.pipelines`.
- **TWO SHADERS ROWS START ON IN EVERY BROWSER (2026-10-27; `src/ui/shaderrows.ts`, a row's `on: true`): Compact shaders and Separate shadow rays.** The
  maintainer's pick, asked whether to turn them on in a Windows browser only, everywhere, or not yet ("we pretty much have no one using ultra, so i think it's
  fine?" of Ultra losing its resampling gather under the split, below). **Off:** `?compactshaders=0&splitrays=0` on the address = a load as it was (the address
  for a PLAIN load to set a wait against; with the bullet above, `&firstframe=0` too), or either row turned off in Options > Graphics > Shaders, which is KEPT off (the kept line holds a row only while it
  is not where it starts; Reset to defaults puts each back where it starts). **Both stands of the harness boot with every row off, as before**: the harness is
  no browser (a row starts on only where `navigator.userAgent` is a browser's), so no frozen digest, tally or headless check moves and there is no
  `FROZEN_HOLDS` row; `TPS_COMPACTSHADERS=1` / `TPS_SPLITRAYS=1` raise one there as before. EVERY PAGE TOOL NOW DRAWS WITH BOTH ON (headless Chromium is a
  browser): a page check that wants today's kernels puts the two words at 0 on its address. **What it rests on:** one Windows machine (RTX 4080, a 4K screen,
  Chrome 155) loaded the two together fourteen times with no error record and every pipeline made (read off the build's backend by another session, not by
  this one); a Mac (M4 Max) drew each row ALONE at the plain build's frame cost within noise; the software-adapter pictures below. **What nobody has:** a plain
  load of THIS build from a Windows machine to set the wait against (the one-call-site rewrite that Compact shaders carries was timed by its author on a Windows
  machine before it came over: the compile queue's bullet has his numbers; Separate shadow rays has no Windows time at all); the two together timed on a Mac; Compact shaders' own gather under the
  Windows compiler (with both on the gather is the split's); any AMD or Intel card. **A refused points' buffer is heard:** the split's buffer (60 bytes a
  rendered pixel) refused by a GPU draws no bounce light; it was a console line only, and is now also one of the visit's error records
  (`gatherSplit.ts splitRefused` calls the page's reporter, `reportError('gpu.splitrays.buffer', …)`, `src/cloud/report.ts`; the maintainer: "let's just make sure it's logged"; the
  grid and the size ride beside the record as its `data`, and one machine is one error whatever its window). No GPU that refuses it has been seen. A browser that may
  not send yet (its first visit ever, opened straight onto a mission before the sharing notice) keeps the report and sends it once the notice has been read; the
  row is not turned off for that machine's next load, so it draws no bounce light until its player turns the row off. **Known seams:** the SHIPPED stand of the
  harness also boots with the rows off, so there it is not the page as it opens for these two (`TPS_COMPACTSHADERS=1 TPS_SPLITRAYS=1` in front of a headless
  tool is; `splitrays-check` sizes the two together); on Ultra the Tab panel and the preset still read resampling on while the split draws the plain gather; a
  browser with no storage shows a row turned off as still on. **Checks:** `shaderrows-check.ts` (cases `starts`, `offkept`, `plain` beside the harness's; `SHADERROWS_BREAK=1` fails `starts` and `offkept`, and `plain`, a load with both words at 0, reads the same under it),
  `splitrays-check.ts` (the refusal's line, and the size tool with both rows on), on the page `shaderrows-page.cjs` (walk `starts`). The ten kernels still exist twice: taking today's writing out
  is the next step, not done here.
- **Shader rows on trial: Compact shaders and Start on first frame shaders (2026-10-26; Options > Graphics > Shaders, `src/ui/shaderrows.ts`).** The two levers of the
  bullet above, each a row that was OFF by default when this was written (Compact shaders has started on in a browser since 2026-10-27: the bullet above) and is the NEXT load's (read once, when its module is imported: the pipelines are made before a menu can open). Kept
  under `twopointsix.shaderrows`; a URL word wins for the visit and is never written; a row is listed only in a build whose code reads it (`useShaderRow`). Switches of
  the page, not rules of the game: no knob, no `FROZEN_HOLDS` row, off on BOTH stands; the harness's words are `TPS_COMPACTSHADERS=1` / `TPS_FIRSTFRAME=1` in front of
  any headless tool.
  **Compact shaders (`?compactshaders=1`, `render/kernels.ts`)**: TEN KERNELS EXIST TWICE. `src/render/shaders/compact/` holds fgather, gather_common, reflect_p5,
  volumetrics, composite, particles, direct, sharp_direct, restir and sdfrig written a second way BY HAND (one call site per heavy function; no tool makes a twin from
  its original), and `KERNELS` hands passes.ts, particles.ts and frame.ts one set or the other. With the row off nothing of the second set reaches the browser. The
  compact volumetrics.wgsl alone needs TypeScript the other does not, behind `COMPACT.on` in passes.ts: its two programs (`const VOL_FROXEL_MARCH`: 'vol' false,
  'volFroxel' true) and the K = 0 twin (`const VOL_K0`, 'volK0', built the first frame `volLightCap` is 0). **THE SECOND COPY IS A PROTOTYPE: one set goes when the
  default is decided.** Until then a change to one of the ten is made in BOTH files, or the row draws the picture as it was before the change:
  `git diff <before> <after> -- src/render/shaders/<k>.wgsl | sed 's#shaders/<k>#shaders/compact/<k>#g' | git apply --3way` carries a change that is outside the
  rewritten lines, then `bun run tools/qa/shaderrows-check.ts --stamp`. The check is RED while one of today's ten differs from what it was when its twin was last
  brought level (`tools/qa/fixtures/compact-kernels.json`), and names the kernel. Every tool that reads a kernel by its path (`dust-look-check`, the snow and fog
  probes …) reads TODAY's; `shadertrim-check` and any other harness tool takes the compact set under `TPS_COMPACTSHADERS=1`.
  **Start on first frame shaders (`?firstframe=1`, `gpu/device.ts PIPE_QUEUE.later`)**: the start waits only for what the first frame draws; the LATER set (direct,
  restirM, fgather, fgatherGi, fgatherGiS, reflectP5, composite, compositeR, compositeS, the five froxel kernels, the dust twin where dust is drawn, and volFroxel with
  the compact set) is not counted on the start card and is compiled behind it. With the compile queue on (a Windows browser) it is held until the boot's own are in, one
  at a time until the page settles, then in the queue's background lane (two at a time, shared with the cut set's twins); with the queue off (everywhere else) every
  creation goes straight to the browser, so the set is only not WAITED for. Each pass takes its stand-in meanwhile (the list above), and WHAT THE ENGINE WRITES IS FOR
  THE PROGRAM THAT RUNS: `engine.render` asks `passes.restirStandsIn` and `passes.restirMReady` before it writes the frame's flags and the restir uniform. As the row
  first came over, a frame the camera moved ran main's restir while `restirM` was out with the motion taps still packed into `cfg.y`, the word main's program reads as
  its spatial count: 0x603, about 1,500 taps a pixel, for as long as the twin took. That is the likeliest cause of the 2 to 5 s at about 5 fps seen on the Windows
  machine some 8 s after the start, and nobody has gone back to that machine to see it gone. A LATER pipeline that fails to compile hands back null (logged; the pass
  keeps its stand-in, and a froxel set that failed leaves the per-pixel march drawing); one that arrives after its place was filled (a composite retune, a hot swap)
  leaves the place alone (`ProgramSet.baseOf`). Auto quality's boot calibration waits while `laterProgress.pending` is above 0 (it would measure the stand-ins and keep
  the answer); the ladder after it does not wait. With the row off `engine.render`'s `sharp` is `sharpAsked`, the line as it always was, the estimator is the setting,
  and the froxel kernels join the cut swap after their build, in the order they always did. **Left as they came, and known**: a debug view or TAAU off while no
  fragment composite is in clears the picture (black until one lands: no preset does either); the set compiles in the order it is asked for (the froxel kernels
  first, the composites last), not the order a tier needs it; a page that never settles the queue (the workbench, the editor) gets it one at a time; the compact K = 0
  twin is built from the boot's prelude alone, outside the cut swap, and is picked ahead of the dust and froxel twins (no preset runs K = 0).
  **The checks**: `bun run tools/qa/shaderrows-check.ts` (the Engine and the particle system on a stand-in device that holds every async creation, four arms each a
  process of its own: off, compact, first, both, plus the rows' own module on a stand-in page; it reads which program each pass is given and what the engine wrote
  for it, with the later set out and in: 136 lines; `SHADERROWS_BREAK=1` hands each arm another's rows and fails 27; the same on both stands); `pipequeue-check` has
  the later lane (49 lines in all with the gentle load's and a lost device's, `docs/agents/qa.md`; `PIPEQUEUE_BREAK=1` fails 31); `TPS_COMPACTSHADERS=1 bun run tools/qa/shaderinline-check.ts` sizes the compact set against its own table
  (`BUDGET_COMPACT`: the boot's set 8.2 MB pasted where today's is 13.1, the cut set 7.3 where today's is 12.1, 129 kernels; composite 574 KB where today's is 978,
  `fgather / main` 228 where 780; `SHADERINLINE_BREAK=1` with it fails as it must; with the row off the tool's own table is green on this tree, as before);
  `node tools/qa/shaderrows-page.cjs` is the real page on the software adapter (the rows on the screen before and after the renderer is up, a boot with both rows
  kept, a boot on `?firstframe=1`; the queue off unless `QUEUE=1`).
  **Off is the build as it was**: a recorded boot (every module's text by digest, every pipeline with its module, entry point and constants, the cut set's build, the
  program each pass is given over twelve settings, and each of those frames' uniform block and restir uniform by digest) is byte for byte that of the build head it was last joined to, on
  both stands: 57 modules, 110 pipelines, 974 picks (the Engine and the particle system; the smoke solver, the light probe and the mesh shadows are not in that
  recording and none of their files changed).
  **What the trial costs while both writings are in the tree**: every page's bundle carries the second set's text (about half a megabyte) whether the row is on or
  not; `src/cloud/prefs.ts` skips `twopointsix.shaderrows` (a row is this machine's).
  **On the real page (headless Chromium, the software adapter; nothing here is a time on a real machine)**: `tools/qa/draw-equal.cjs` (`--with compactshaders=1`, a
  new word of the tool: a query added to every view's address) at 640x360, high, the compact set against today's on the same commit: the rain at dusk EQUAL at both
  checkpoints, the firefight EQUAL at its first (230,400 pixels each, by sha256). The firefight's second checkpoint has no pair: TODAY's page lost the software adapter's device there three times out of three (`A valid external Instance reference no longer exists`, with the queue on and off, once before its first checkpoint), where the compact page drew all seven frames; that is this box's adapter under today's kernels and says nothing of the row. The compact page booted with 119 of 119 pipelines and no GPU error
  both times. `shaderrows-page.cjs`: with nothing kept and the renderer still compiling, Options > Graphics has Shaders with both rows Disabled; both put on are kept;
  the next load (medium, the queue off) counted 104 pipelines on its start card where a load with the rows off counts 118, every later pipeline came in, frames after it
  march the froxels, none failed and nothing was on the console; Options then shows both Enabled and Reset to defaults puts both off and takes the kept line away. With
  the queue off this adapter had the later set in before the start resolved (nothing holds it back; it is only not waited for). `QUEUE=1 … first` (`?firstframe=1` at
  high, the queue on, as a Windows browser gets it): ready with no fragment composite and no froxel kernels in and 40 creations in the queue, frames meanwhile drawn by
  sharp direct with the froxel flag down, everything in 22 minutes later on this adapter (the later set and the wall-cut twins share the two background slots), no
  error, and the address's word not kept.
  **Not seen**: either row on a GPU from this side (the numbers and the pixel-for-pixel word of the bullet above are the Windows machine's, on the tree they were
  written in); the slow seconds after the start gone on that machine; any of it on a Mac.
- **The two rows that built shaders on demand are out (2026-10-27).** "Wall cut shaders when needed" held the cut-box set back until the first cut, and "Other
  shaders when needed" held the later set back until a frame asked for each program. Both were a day old, off by default and never seen on a GPU. The maintainer:
  "i think we shouldn't do truly lazy compilation where then have stutters during gameplay" and "i'd generally prefer to have a single path when possible". Their
  code, their Options rows, their URL and harness words and their checks are gone; a choice kept under either is dropped the next time a row is moved. What stays
  is the one path: the later set compiled behind the start (the first-frame row above), the cut set built in the background after the boot (`cutSwap.start()`), and
  the ladder's hold while a later program compiles (`ui/quality.ts STANDS_IN`, at most 30 s in one wait; `quality-plan` case D1, `LADDERHOLD_BREAK=1`). **Checked:**
  `shaderrows-check.ts --dump` on the off, compact, first, both and all-three arms is line for line that of the tree before. Nothing of it was run on a page.
- **Separate shadow rays (a Shaders row, 2026-10-26; OFF on both stands of the harness and, since 2026-10-27, ON in every browser, the bullet above; the next load's; `?splitrays=1`, `TPS_SPLITRAYS=1` in front of a headless
  tool; `render/gatherSplit.ts`, `shaders/gather_split.wgsl`).** A trial for the Windows shader wait: the final gather stops being one megakernel. Today
  `fgather.wgsl`'s `main` lights every point its rays land on inside itself, so `directAtPoint` (the light loop and its three shadow rays, 123 KB a copy as DXC
  pastes it) is written at four places and `main` is 780 KB out of a 92 KB module. With the row on the same frame is two programs made from the same helpers:
  `trace` (pass `gather`) casts the rays and writes each point that wants light to a buffer (the point, its normal, its bounce-card set, its paint, the radiance it
  already has), `light` (a pass of its own, `gatherLight`, so the Tab list and the F9 recorder price the two halves apart) lights them in one loop round ONE
  `directAtPoint`, takes the firefly bound, and stores what the gather stores. The cut-box set gets the same two (they are made in `traceJobs`, like every traced
  pipeline). **Sizes (the size tool, `TPS_SPLITRAYS=1 bun run tools/qa/shaderinline-check.ts`):** `trace` 249 KB and `light` 125 KB where `main` is 780 KB and
  `mainGi` 842 KB; a load's boot set goes from 13,110 KB to 10,490 KB and both sets together from 25,175 KB to 19,935 KB. The split set has budget rows of its own
  (261 and 131 KB, in place of the megakernel gathers' rows only while the row is on); no row of today's table moves. What is left in `trace`: `occluded` ×4
  (the cards' term, the glints, a pane's dust), `traceClosest` ×3, `glossGlints` ×2 and a pane's shading at 97 KB.
  **Row off = the build as it was:** every one of the 64 module texts a load compiles is the same text, hash for hash, as on the build's head without this change.
  **Not the same random numbers as today, by design:** in `main` the light loop draws from the texel's one stream (one number a light that reaches the point), so
  the next ray's direction depends on a count only the light loop knows; the split's rays use `main`'s stream without those draws and each point's light pick has a
  stream of its own (`gxFork`). The estimate is the same one, a frame is other noise of the same size. So the split is not compared with today's picture bit for
  bit but with a TWIN: today's megakernel drawing from the split's streams (`tools/qa/splitrays-twin.cjs`, loaded through the gather's hot hook by
  `DRAW_EQUAL_AFTER_READY=tools/qa/splitrays-twin.cjs node tools/qa/draw-equal.cjs capture …`), against `--with splitrays=1`. **Pictures (software adapter, Chrome for Testing 153, 640×360, high, drawn frames
  3 and 7, two views with no cut piece in them: `office-night` and `glass-broken`):** the row's picture against the twin's differs in 0.23–0.51 % of the pixels, by
  at most 2 of 255 levels (mean 0.001–0.002 levels): what the buffer rounds (paint and unlit radiance as f16, a slanted normal as 16 + 16 bits) and the order two
  sums are taken in. Against TODAY's build the same frames differ in 23–29 % of the pixels by up to 11 levels (ten pixels over 8 in `office-night`, none in
  `glass-broken`; mean 0.13–0.17 levels, the pictures' mean brightness the same to three decimals) — and the twin differs from today by the same amounts to the
  pixel count, so that distance is the other random draws, not the split. Both pages booted with every pipeline made (117 with the row on, 118 without) and no console or GPU error; `gatherLight` is in the
  frame's pass list. The views' own expectations were unmet the same way on every arm (two of three torches counted in frame; no pane broken in `glass-broken`), so
  the pane's mirror ray was drawn through whole panes only. After the review's changes the row's first two frames were drawn again and are the same pixels, and the same again after a hot reload of the gather with the
  row on (`__hotGather()`: the three pipelines rebuilt, no error).
  **What the row does not cover yet (say it when it is tried):** the resampling gather (`mainGi`, ultra's) is not built while the row is on: the plain split
  gather draws ultra's bounce and no reservoirs are kept (`setGi`), so ultra looks and costs different under the row (the row's own words say so; the debugger's
  resampling line still reads the setting, not what is drawn); the young-history rays (a Tab row, in no preset) are not traced; a hot reload (`__hotGather`,
  `__hotCommon`) rebuilds the split's three pipelines from the given texts' helpers, so an edit inside `main`'s own body is not drawn and the answer says so; the
  shadow rays of the cards' term, the glints and a pane's dust stay in `trace`. `trace`, `glossReflectionX` and `glassShadeX` are hand copies of `main`,
  `glossReflection` and `glassShade` (the check pins the originals by hash, it cannot prove the copies follow): making them from `fgather.wgsl` by a text swap, as
  the twin is made, is the next step if the row is kept. Reflect, restir, particles, the
  marches and the composites are untouched. **The buffer** is 240 bytes a gather texel (60 a rendered pixel at gather /2: 55 MB at 1280×720 inside, 124 MB at
  1920×1080), written and read once a frame, grown with the gather grid and let go for a smaller one when the grid is under half of it; a grid whose buffer the adapter
  cannot bind, or the GPU cannot make (an out-of-memory scope round its creation), is refused with one console error, the bounce is not drawn, and that size is
  not tried again in the visit. That is the frame cost to measure on a real GPU beside the two passes' times. **With Compact shaders also on, the split wins for the
  gather:** it is made from today's `fgather.wgsl` and `gather_common.wgsl`, imported by `gatherSplit.ts` itself, whatever other writing of those kernels the load
  compiles. **Check:** `bun run tools/qa/splitrays-check.ts` (probe `splitrays`, family `shaders`; eleven lines: `trace` reaches no light loop, `light` reaches
  `directAtPoint` through one call site, the buffer's layout is one number in the shader and the engine, the three functions of `fgather.wgsl` the split was
  copied from are pinned by hash — a change to `main`, `glossReflection` or `glassShade` goes into `gather_split.wgsl` too — the twin builds, and what a load
  compiles with the row off and on); `SPLITRAYS_BREAK=1` (the row left off where the lines turn it on) fails three, the same on both stands.
  **Never seen on a GPU; nobody has timed a Windows compile with it; no frame time measured anywhere.**
- **The outline in the dark** (`render/outline.ts`, `shaders/outline.wgsl`; 2026-10-20): one scissored, blended triangle onto the composite after the fog of
  war and before the resolve, reading the G-buffer's id and depth targets, made only while the player's Options row is on. What it is for, what it costs
  and what nobody has seen: `docs/agents/menus.md`, "Finding your own man". A pass that wants whose-pixel-is-this can take it as its model: the owner byte
  is bits 20 to 27 of the id target on a character's mesh and on every box emitted for him.
- **The bounce is never read through a wall off a man (`BOUNCE_WALLS`, the scene prelude's const — `render/frame.ts bounceWallsConfig`, ON; `?nobouncewalls` at boot = every
  read as it was; no game table, nothing for a recording to carry, the same on both stands).** The maintainer, 10-20, the pass at night, the squad in the snow against
  the tea room's outer wall, the debugger's direct / specular / indirect views: "i think there's some leak on direct and specular and indirect with the wall here", and
  "also happens on this side too". WHAT IT WAS: the radiance cache (`diceIrradiance`, a 1 m lattice) is read a probe spacing along a face's normal — sound for a wall,
  and for a man stood by a wall the room behind it. (1) Indirect: a gather ray that HIT a man read the room off his side that faces the wall; a ray that LEFT a man (or
  the lip of a boot print: a shading normal well off a box face's own) for the wall was bounded by `diceIrradianceFrom` as if the plane it left were a wall, and
  "the cell centre is behind it: step to the cell on its side" stepped the read into the room. (2) Specular: a coat's sheen (`composite.wgsl wetCoatSpec`) mirrored
  the cache a spacing along the man's normal: the room's lamps on his rims. (3) Direct: nothing found — the reach masks drop the room's lights from the cells outside
  (dumped for the pass: level 0 in every cell beyond the walls), what his direct view shows is the moon; the few warm specks on the men's rims there are NOT explained.
  THE RULE: `lighting_common.wgsl diceWalledPoint(Ph, n, skip)` — the read point a spacing along n, stopped 5 cm short of the NEAREST box on that line (`occludedTBoxes`
  answers the first box it meets, the ground before a wall: it is asked again short of each answer; men's proxies are not asked, and `skip` names up to two men whose
  carried boxes are not either — his own gun must not hide the air from him), then a box ray to the centre of each of its four plan cells: the read keeps all four,
  else the one column / row / cell it sees, and where it sees none there is NO read (dark, never the room). Asked where the hit is a man or a box he carries
  (`bounceHitMan`: the gather's rays, the ray-traced reflections' hit shading, the cache's own probes in `hitRadiance`; NOT the gather program's own gloss and pane
  rays, which keep the old read: below), where the surface a gather ray
  left is no wall (`bounceLeftNoWall`: a man's pixel; within 2 m, a normal more than a dozen degrees off every axis), and for the sheen. Everything else reads as before (since 10-23 a metal's reflection rays ask it too: REFL WALLS, the next bullet).
  THE SOFTWARE ADAPTER'S ROOM (10-20, the same day: the build with the rule lost its GPU device — "A valid external Instance reference no longer exists", the GPU
  process dead of a segfault — on the first frame a CUT piece was alive: a wall charge in `soundlanes-page.cjs`'s walk, a bang entry that broke cover). The software
  adapter builds a copy of a function for every place it is written, the read wrote `occludedTBoxes` in three places and the gather asked the read in five: fifteen
  more box walks in the gather's program, each with the cut test inside in the cut twin (`render/cutSwap.ts`), and that twin no longer ran. Bisected on the blown-wall
  still (`node tools/qa/cut-stills.cjs --root . --out <dir> --tag after --cut --scene wall --cam near --quality verylow --w 320 --h 180 --settle 3 --physics 30`, ten
  minutes a run, one run each): rule in the probes alone clean, probes + sheen clean, gather + reflections dead; the walk written ONCE (five more walks in the gather)
  clean at verylow and high; a second written walk (ten) dead. So the read's loop asks ONE written walk up to eight times, and the gather's gloss and pane rays (a man
  mirrored in a floor or a pane) went back to the old read: three more walks. THE RULE FOR WHOEVER EDITS `fgather.wgsl` NEXT: its cut twin is within a handful of
  written box walks of what the software adapter runs; after adding a trace, draw that still. Nobody knows whether any real GPU has such a limit (none was reported).
  `?nobouncewalls` rides a replay page's address (`ui/replaykey.ts VISIT_FLAGS`).
  NOT MENDED: the light meter (`probe.wgsl`) reads the cache AT the man, blended over the cells around him — by a lit room's wall some are the room's (the meter's
  owner's; untouched here). The firefly cap of the reflections (`Eref`) still reads the old point. COST: five to eight short box rays per asked read — every pixel of a
  man for the sheen, his gather rays, and near rays off steep snow or yawed boxes; never timed on a GPU.
  CHECK: `bun run tools/qa/probes.ts bouncewalls` (family `render`; `BOUNCEWALLS_BREAK=1` fails 3 of its first 5 lines; its last 3 are REFL WALLS'; both stands): the shader's rule and every site held by
  their text, and a TypeScript copy (its ray is the nearest box, as the shader's ask-again loop) weighed for men stood 0.33 to 0.9 m off every ceiling-high wall of all
  50 levels, a shin's down-turned skin among the points — reads that blend a cell beyond a wall 42.8 % → 14 of 1.46 M (the pass's tea room: 44.5 % of skin points and
  40.5 % of rays to the wall → 0; a print's lip at the wall's foot 165 of 500 → 0, the 100 that lean 0.15 clean either way and not asked). No shader runs there.
  PAGE: the final text compiled on the software adapter (the pass, squad of four, hard, medium and high quality: 119 of 119 pipelines, no GPU error on the console). STILLS there (one seed, 783×500 at half scale,
  five men stood at the tea room's west wall, the lean-to's bulb put out so the yard has the moon alone, `?nobouncewalls` against the page as it opens, the same
  frames each): the indirect view's men had orange flanks on the side that faces the wall and the snow at the wall's foot an orange streak — pixels redder than blue
  by 6 of 255 on the men's wall side 1623 → 1, on the strip of snow 592 → 1; the spec view's orange rims 1261 → 15; the men stay moon-blue, not black. Eight frames,
  not converged. Never seen on a GPU: the picture in motion, the cost, and whether a man's dark side reads too dark where the walled point sees one cell.
- **A metal never mirrors the room behind a wall (`REFL_WALLS`, the scene prelude's const — `render/frame.ts reflWallsConfig`, ON; `?noreflwalls` at boot = the rule
  off; no game table, no journal line, no `FROZEN_HOLDS` row: nothing in the update reads it and the headless stands draw nothing)** (10-23). The maintainer, the solo
  night's manager's office at high, one desk lamp lit: "some weird light leakage on bookshelf? seems like irradiance cache leak through wall? and showing up in
  specular?" — the steel shelving on the partition to the lit storage room glowed a dull orange, and only debug view 10 (specular) showed it. CAUSE: a ray-traced
  reflection shades its hit with `diceIrradiance(Ph + hit.n × c0Spacing(), hit.n)`. For a hit on a wall the spacing leads into the room. For a hit whose normal runs
  ALONG a partition within half a cell of it (a shelf board, the floor at the wall's foot, the ceiling over it, an upright's side) the read stays by the wall and its
  four plan cells straddle it: with the wall on the metre lattice the share of the cells beyond is 0.5 less the distance to the wall's line, 0.1 to 0.27 on that unit's
  boards and 0.44 at the wall's foot. A dielectric shows a few per cent of its mirror at this camera; a metal shows all of it (F0 = its paint, and composite steps a
  metal's diffuse back 70 %), so the steel glowed and the cartons on it did not. The 08-12 mend (`diceIrradianceFrom`) is the gather's and bounds a read by the plane
  the ray LEFT; it was never asked by the reflections ("listed for a follow-up if a case shows") and would not have held this one: the wall is neither plane.
  THE RULE: a metal pixel's reflection rays (`metalness > 0.25` in `reflect.wgsl` / `reflect_p5.wgsl` main) shade every hit with the walled read
  (`diceWalledPoint`: stopped short of the nearest box along the normal, the footprint held to the cells the point can see, no read where it sees none), the reflecting
  pixel's owner in the skip word; every other pixel's rays keep the read they had. The box walk is written no more often than before (the sites that asked it for a
  hit on a man now also take the metal's word), so the software adapter's program is no bigger. `?nobouncewalls` alone no longer gives the picture from before both
  rules where a man is mirrored in steel: that takes both words. `?noreflwalls` rides a replay page's address (`VISIT_FLAGS`).
  NOT MENDED: the same blend in the DIFFUSE gather's hits for rays that did not leave that wall (× the look's indirect scale 0.35, and not seen in his indirect view);
  the gather's gloss and pane mirror reads (low and verylow trace no reflections; that program has no room for a fourth walled site); a pane's mirror
  (`diceMeanRadiance(Pg + Rg × 0.6)`); the light meter (`probe.wgsl` reads the cache at the man: no LIGHT number moves with this switch, the reflections are picture
  only); phase 5's hit-lighting reuse (`rtHitReuse`, off on every preset) keys its record by box and place, not by which read made it. THE GENERAL MEND, proposed and
  not built: the cache writes, per cell, which of its plan neighbours its centre can see (a few short box rays where it already traces hundreds, so doors and blown
  walls follow), and `diceIrradiance` itself holds its footprint to those — every reader mended with no rays at the read, this rule and BOUNCE WALLS' rays retired.
  COST: five to eight short box rays more for each ray of a metal pixel (steel shelving, lockers, door leaves, the GWOT kit's sheet metal; two rays a pixel on ultra);
  never timed on a GPU. RISK, from the review's count on the check's own model: away from walls the walled read also moves 15 % of floor reads that never leaked
  (stopped short of furniture, or one line to a cell blocked by a desk leg) and 3 % of wall-face reads see no cell and go dark, so a steel face may show a step or a
  dark patch in its mirror of clutter. Never seen on a GPU.
  CHECK: `bun run tools/qa/probes.ts bouncewalls` (family `render`), its last three lines; `REFLWALLS_BREAK=1` fails two of them; both stands. The text of the rule in
  both passes, the prelude and the flag; and the same TypeScript copy of the read weighed for faces that look up (the floor, a board's height) or down (the ceiling)
  6 to 40 cm off every ceiling-high wall of all 56 levels, the boxes turned a quarter put back so the shelving's own boards stop the walk: reads that blend a cell
  beyond a wall 195,120 of 292,051 (66.8 %) → 0, with 1.0 % seeing no cell (dark); at the office's partition 397 of 508 → 0, 24 dark. No shader runs there.
  PAGE (the software adapter, one seed, 640×360 with 492×277 inside at high, `?solo&paused&quality=high&mute`, the camera pinned on the unit from the office side, 24
  frames, `?noreflwalls` against the page as it opens): the final text compiled and drew with no GPU or page error. The specular view at exposure × 4, in display
  levels of 255: the uprights' inner faces 19.4 → 0.7 (their brightest point 53 → 2), the lowest board's top 6.3 → 1.0, the next 1.6 → 0.7, the top board 4.7 → 3.4,
  the unit's whole rectangle on screen 4.08 → 1.70 in the mean and 11.7 → 2.4 at its ninetieth percentile; in the final frame the uprights 3.1 → 0.3 and the lowest
  board 1.4 → 0.3; the indirect view is the same to the digit at every point (the diffuse is not touched). What stays, the same with the switch either way, is a warm
  streak on the front edges of the top two boards: read as the desk lamp's own highlight (inferred, not traced). No step or dark patch showed in that one view. Never
  seen on a GPU: the picture in motion, the cost, and steel anywhere but this unit.
- **The flame's sample count is a number** (`FIRE.look.steps`, 40; the look row's fifteenth float, `K.d.z`; `?firesteps=N`, 4 to 64; a row in `/firelab`;
  `LOOK='steps=12'` in `tools/qa/fire-lab.cjs`). No rule and no switch: 40 is the count the shader had, and a recording from before the key gets 40. It is
  there for a TIMING: the flame's price is the count × the pixels it covers, at sigma 2.5 no ray goes opaque so none stops early, and a model (nobody's
  timing) puts the flame over its bar at the closest zoom at High. On a real GPU, compare the page as it opens with `&firesteps=18` and `&firesteps=24`
  for the price and for bands in the flame. Checked: the shader compiles and draws at 40 and at 12 on the software adapter, shader alone (the same flame, a
  faint stipple at 12); `fire-check`'s look line carries the count. NEVER SEEN OR TIMED ON A GPU; the whole page was not booted.
- **An open fire burns (`src/game/fire.ts FIRE`, `render/shaders/fire.wgsl`; 2026-10-23, the maintainer: "we gotta improve our fire! we have barrels with fire on them …
  great fire with smoke and crackles and dynamic lighting would look sick").** Until now such a drum was a glowing lid and a steady lamp. A level light whose id starts
  with `brazier` is a fire (`pass.json` and `tut-sky.json` have one each; `docs/level-format.md`, the lights row). SWITCH: `FIRE.on`, ON in the page (`?fire=0` = the
  lid and the steady lamp), OFF in the module and on the frozen stand (`TPS_FIRE=1`); its table rides the journal and a recording from before the key replays with it off.
  THE FLUID SOLVER IS USED FOR THE SMOKE AND NOT FOR THE FLAME: a flame is 40 cm across and changes shape in a tenth of a second, the solver's cells are 5.5 cm and it
  publishes density, not heat, to the picture — a flame made of it would be seven cells wide, cost bricks at every preset and still need a shader to colour it; smoke is
  what the solver is for (it climbs, leans in the wind, pools under a roof, is lit by every lamp and shadowed).
  THE LIGHT is the level's own lamp, no new light: its strength × `fireFlicker` (smooth noise at five rates 0.43‥8.3 Hz + a kick with each crack of the wood: 0.69 to
  1.37 of the authored 9 over twenty seconds, mean 0.985, RMS about 0.15, never under `least` 0.6 or over `most` 1.6), its place lifted `lift` 0.25 m into the flame (the
  level hangs it 16 cm over a lid 58 cm wide: from there the lid is lit white and a flame is lost on it) and wandering 6 cm, and while it burns the emitter is `size`
  1.1 of the mouth's radius, the flame's own reach (0.24 m on the pass's drum; the level's 6 cm pinpoint drew the drum's own shadow as a black disc a metre round it:
  SEEN ON A GPU, below; `size` 0 = the level's, `?firesize=0` in the page, to compare on a GPU; the level's size is back the frame the fire is out or the switch goes
  off, and a recording from before the key replays with `size` 0. A level's own radius is kept when it is the bigger). WHO READS IT: the light meter of a man
  beside it and every caller of `lightEstimateAt` that does not ask for the steady light (the review of the second cut listed three that ask what a PLACE lights and
  so read the flicker: `guards.ts litPlace`'s dark test, `doortools.ts thrLitCell`, kept for the order, and `nvg.ts nvgPlaceLight` — a cell near `LIGHTS.litAt`
  beside a fire can fall either side by the flicker's phase; they also count flashes, so handing them `steadyOnly` is their owners' call and was not made here).
  The steady readers — `lightEstimateAt(…, steadyOnly)` (the calm torch, the dark-floor and dark-door rules) is handed the AUTHORED lamp where the level hung
  it and the size the level gave it through `RtLight.steady` (guards.ts; the fog of war's remembered lamp likewise), the same number every frame as with the switch off (the check's `steady` line, bit for bit).
  THE FLAME, THE BED OF COALS AND THE EMBERS are drawn by the composite (`fireOver`, beside the lasers' light in the air; records in the beams uniform, `passes.ts
  setFires`, four fires at most, the nearest the view): a marched volume 24 steps deep that hides what is behind it within three fingers (`FIRE_SIGMA` 14 /m; 18 steps at 20 /m was a stipple, a pixel against its four
  neighbours 0.036 in the lab where the first cut was 0.015: this is 0.019), each tongue yellow at its root, orange up its length and red
  at its tip (`fireColor`, written as scene radiance and kept under the tone curve's shoulder: red from 0.4, orange 1.6, the yellow heart 2.1, a tenth more when the
  light kicks); a
  disc of char and deep orange gaps over the lid; ten embers climbing and six thrown by each crack, closed forms of the game's clock (nothing kept between frames: a replay
  draws the same fire). PICTURE ONLY.
  THE THIRD CUT (2026-10-23, the maintainer's second look on a GPU, with stills of a fluid-simulated camp fire as the target: "still very ugly. needs more
  dynamic range, smoke, embers, movement, could we do heat distortion?"; then "embers can be GPU particles though") REPLACES the paragraph above where they
  differ. The second cut had held the flame's radiance under the tone curve's shoulder, level with the snow it lights: that was the flat pale peach he saw.
  Now the flame is HDR inside and saturated at its skin (`fireColor`: dark red 0.07 at the tips and edge, through 0.3, 0.6, 1.05 and 1.9 of red, to a heart of
  9 × `core`; AgX at the page's exposure puts saturated red and orange between 0.1 and 1 of scene radiance and everything above pale, and the bloom, which has
  no threshold, spreads the heart), marched 32 steps and nearly opaque (`sigma` 12 /m), with distinct tongues (two slow noise reads lick the column sideways,
  a third cuts deep gaps between tongues and streaks the heat), rings that swell and climb about twice a second (`breathe`), a sooty cool edge (`soot`), and a
  column of SMOKE DRAWN STRAIGHT OFF ITS TIPS in the same march (`plume`, ten steps to 3.6 m, lit orange from below: the solver's own plume is look-only, thin
  and starts over a man's head, so by itself it never read as the fire's smoke; `?fireplume=0`). HEAT SHIMMER (`shimmer`, 1.2 cm; `?fireheat=0`): the composite's
  pixel is a pure function of where it is asked (`compositePixel`), so it is asked a little to one side: `fireWarp` sums `fireShimmer` over the fires (the ray's
  closest approach to the column over the flame, two noise reads that climb), nothing when the pixel's own surface is nearer than the column, and never in a
  debug view. THE EMBERS ARE GPU PARTICLES now (`render/particles.ts EmitOpts.ember`: a spark whose colour word's top byte is not 255 — `particles_common.wgsl
  emberIs`; lifted by `EMBER_LIFT` 3.2 m/s² and swirled, carried by the wind in full, drawn by `emberRadiance`, in the bloom like any spark): about `embers` 24
  alight over a fire within 30 m of the view, `emberCrack` 10 more thrown by a crack; filed by the host's book with a seed made by hand, so the game's own die
  and state are untouched (the check's `look` line: a game with a pool and one without differ in nothing); `?fireembers=0` = none; they are not drawn for a
  fire the fog remembers. The record is 64 bytes (no pop list); the look's ten numbers ride behind the four records (`passes.ts setFireLook`, `FireLook`).
  KNOWN, kept: no embers at all where the page has no GPU particles (`?noparticles`: the closed-form embers the composite drew are gone); embers are drawn
  after the composite, so the flame does not hide one behind it and the shimmer does not move them; an ember asks the wind as a puff of dust does (no wind
  record names sparks); the shimmer reads nothing that stands nearer than the column, here or where it would read; the flame thins out under the top of what
  is marched (2.3 × its height) so a kicked tongue is not cut flat; THE DRAWN SMOKE AND THE FLAME ARE PICTURE ONLY (no sight line or sensor knows of them:
  a man behind the column is hidden from the player a little and from nobody else, which bends "one model for picture and AI"; the flame as gas in the smoke
  solver, the maintainer's pick and the next step, is what makes the smoke real; until then `plume` is a slider and `?fireplume=0` a word).
  A recording from before `FIRE.look` replays with no shimmer, no drawn smoke and no embers. THE FIRE LAB: `/firelab` on the dev server
  (`?levelFile=pass.json&roomclear&squad4&nofow&firelab`, `src/host/firelab.ts`, on the sneak lab's pattern): you beside the brazier, nobody able to notice or
  hurt you, every number of the look, the embers, the light and the drifting smoke as a live slider, the build's back by a button, the lot copied out as one line.
  NEVER SEEN ON A GPU: the look was set on the software adapter's stills (`tools/qa/fire-lab.cjs`, which now emulates the bloom and the tone curve: `LOOK='k=v,…'`).
  THE FLAME AS GAS (`FIRE.gas`, OFF in the page until it has been seen: `?firegas=1`, and a row in the fire lab; 2026-10-23, the maintainer on a card asking
  how much of the flame should be real fluid sim: "Drive it from our 3D smoke solver", then "i'm curious how far we can push our existing smoke solver.
  curious if we can make the finer cells work well with our existing brick stuff"). A fire within `viewR` 12 m of the view files a second look-only source
  every half second on the solver's FINE lattice (2.2 cm cells, the breath's and the gun gas's, the same brick pool; `lattice: 'fine'`) at the coals: HEAT
  with next to no smoke in it (`temp` 22, `density` 0.3 a second, leaving at 1.8 m/s). The solver lifts it by its own buoyancy, curls it and leans it in the wind, and the
  picture draws the flame from that heat (`fire.wgsl fireFlameG` through `composite.wgsl fireGas` = `smoke_read.wgsl spgFineHot`): white-hot at `ref`, no
  flame under `lo` of it, the noise only tearing the skin finer than a cell. A fire's record carries how far it has eased to its gas (`FireRecord.gas`, the
  host's book), so a fire far from the view, on a preset that sheds fire smoke or with no solver is the flame of noise; while it is gas the drawn smoke is
  off. TWO CHANGES TO THE SOLVER (`src/smoke`, no thread's own: the last change there was the breath's, 10-20), both nothing while the switch is off:
  (1) HOT GAS ON THE FINE LATTICE IS KEPT — `sparse.fineKeep` (0 = no such rule, the solver as it was; the uniform's spare word): the fine lattice fades
  gas under `thinBelow` 0.25 and the heat in it at up to 3 /s (THE FINE LATTICE LETS GO), which made the first gas flame (drawn from density) a lick whose
  length hung on how dense it was and which lost its lift and lay over 45° in the yard's 0.7 m/s; with the rule a cell hotter than `fineKeep` is not thin
  and cools only at the solver's own 1.2 /s, so the flame's height is its cooling time. The fire raises it (`SmokeSystemLike.keepHot`, `FIRE.gas.keep` 0.8)
  while a fire's flame is gas and puts it back to 0; WHILE IT IS UP gun gas and breath that hot linger as long too (nobody has measured how hot they run).
  (2) A FINE BRICK'S HEAT IS PUBLISHED in its atlas texel's last channel (`publish`; nobody read that channel of a fine brick: `smoke_read.wgsl` is the
  atlas's only reader and takes `.x` of a fine brick), always, switch or no switch: a brick that holds heat and no density was already kept and published
  (its box counts either), so the page and the marches see what they saw. WHAT THE GAS IS TO THE GAME: its little density is the solver's own, so in the
  page a sight line through the flame loses what that smoke takes (thin by design; `density` is a lab slider), and the headless stands have no solver.
  FIRST CUT, kept in the log (d0e94537): the flame drawn from the fine lattice's DENSITY with no solver change; measured in the page (software adapter,
  lines asked of the game's sensors): the published gas ran about a fifth of the source's `density` at the heart, 0.8 a second was a half-metre lick and
  1.6 a metre and a half, the pool read 787 to 896 bricks three seconds in. CHECK: the fire check's `gas` line (heat sources as the table says, the solver
  told to keep hot gas and told to stop, the record's share, none without a solver or put off, an old recording off). IN THE PAGE (software adapter, the
  pass, the storm 0.5 m/s at the drum, five stills): a white heart, orange body, red tip, no haze; one streamer about a metre and a half long that the wind
  leans some 45° and curls (one streamer, not a field of tongues: one soft source, two-centimetre cells); a sensor's line through its heart keeps 0.90 of
  its light, every other line through the flame 0.97 or more; the pool 802 bricks three seconds in (275 fine) and 1,374 six seconds in with `temp` 22
  (445 fine; the pass's own cold smoke and the fire's coarse plume are in those). A gas flame is marched 56 steps (40 and 16 more) in a cylinder up to 0.8 m wider than the
  noise's (`fireGasWide`), so it costs half as much again a pixel over more pixels: not timed. NEVER SEEN ON A GPU, never seen moving.
  UNDER THE FOG OF WAR A FIRE IS DRAWN AS THE TEAM LAST SAW IT (`FIRE.fog`, on; `?firefog=0`; a recording from before the key replays with it
  off; 2026-10-23, the maintainer, playing the pass in plan mode on hard: "weird that fire doesn't show up in fog of war … I think we should at least show
  everything in the environment visible at the start of the level all the time?"). The first cut hid the flame over ground the fog hides, by the fog's rule for
  smoke and flakes; but a fire is part of the place, and the fog's picture (the full picture: the live frame dimmed) already had its lamp lighting the yard
  round a drum with a bare lid. Now a record carries the fog's own memory of the fire's lamp (`FireRecord.fog`, `RtLight.fog`'s numbers; `fireOver` asks
  `fogSeenAt` at the coals only for 1 and 2): burning and known to burn = 0, drawn wherever it stands (the fog pass dims it with its ground); lit where nobody saw
  (`RtLight.fog` 1) = 1, only where the team sees; put out where nobody saw (`fogGhost` up) = a record at full heat for where the team does NOT see, 2, beside the
  lamp's own ghost (which is the level's pinpoint where the level hung it: its shadow has the drawn edge). The records are built by `fireDraws`, which
  game.ts calls AGAIN right behind the fog's turn (`updateFire` runs before it): the frame a fire is put out unseen already draws it burning, not one frame of
  bare lid. KNOWN LIMITS, kept: the fog test is ONE per fire, at the coals, while the fog pass dims a pixel at a time (a flame astride the edge of what the team
  sees is drawn or not as a whole); a fire in the fog still leans on the LIVE wind (a door's gust the team did not see moves it) and flickers on the clock; a
  remembered fire counts toward the four drawn; `fireSummary`'s "drawn" counts it too; `FIRE.fog` has no stand word of its own (like `FIRE.smoke.on`, a
  picture-only key under the held `FIRE.on`: the frozen stand never draws a fire). NOT IN IT: THE FIRE'S SMOKE — `common.wgsl fogSmoke`
  hides all smoke over floor nobody sees and cannot tell a fire's plume from a grenade's, so a fire in the fog has no plume (the fog's rule, not this file's);
  and the fog's OTHER pictures (`?fogfull=0`: the paint under the remembered lamps) draw nothing of the composite, so no flame there, as before. The check's
  `fog` line (the real fog raised in the rig, the squad at its start, the brazier broken: `2@1.00` that very frame); drawn once in the page, the software adapter at Low, the squad left at its start on hard (`AWAY=1 QUERY='roomclear&squad4&difficulty=hard'`).
  NEVER SEEN ON A GPU. (The fog's remembered lamp, `?fogfull=0`'s, is the authored one, where the level hung it: fogofwar.ts `litBlock`.) Not in the ray-traced reflections; the level's glowing lid under it is dimmed to
  `coals` 0.42 of its authored glow (its bounce is that much less).
  THE SMOKE is a `look` source on the coarse lattice (`FIRE.smoke`), filed every second while the solver draws and the fire is within `viewR` 18 m of the view.
  LEARNT IN THE PAGE: a source two cells wide does NOT climb whatever its heat (temp 2.6 to 30: it hung within 0.3 to 0.6 m of where it was filed, an invisible haze
  at head height that cut a line along the wind at 1.6 m to 0.26); one five cells wide (radius 0.26, temp 30) climbs — so it is as wide as the drum and its FOOT is
  over a standing man's head (2.05 m; the check holds 1.9). MEASURED IN THE PAGE (`fire-page.cjs`, the pass, the software adapter at Low, the yard's 0.5 to 0.8 m/s of
  wind at the drum, a plume fifteen seconds old; the light each line keeps, a line starts to count smoke under 0.8): across the plume at 1.3, 1.5 and 1.7 m, over
  the drum and 0.8 and 2 m downwind, 1.00 every one; at 1.9 m 0.94 over the drum, 1.00 downwind; ALONG the wind (2 m upwind to 6 m down) 0.98 at 1.3 m, 1.00 at
  1.6 m, 0.92 at 1.9 m; overhead 0.55 at 2.6 m and 0.23 at 3.4 m (the smoke is there); with `?fire=0` every one of them 1.00. ONE wind, one level, open sky: a fire
  under a low roof or in still air may pool lower, and nobody measured that. `?firesmoke=0` (`FIRE.smoke.on`) is the fire with no smoke.
  THE SOUND: `fireBed` (a 7 s loop held as a follow voice, handed on every 12 s, asked again when a fire is lit again) and `fireCrack` with each pop — the mixer's
  alone, no event: nobody in the game hears a fire.
  WHAT IT COSTS, by the model and timed on no GPU: no light is added. The flame, coals and embers run in the composite, which every preset draws as its compute twin
  at OUTPUT size (timer row `compS`; the ladder counts it fixed, so the render scale cannot pay for it, only a preset hop): against High's 1000 rows Ultra × 1.44,
  Medium × 0.81, Low × 0.52, Very low × 0.36. Only pixels inside a fire's bound pay (a cylinder 1.3 m round it: with the game's 28° camera at 55° of pitch 5.5 % of a
  16:9 screen at the 21 m the game opens at, 14 % at 13 m, 68 % at the closest zoom, 6 m): about 2,300 operations a pixel there (ten ember tests, up to 24 more for
  1.5 s after a crack), and about 15,500 where the ray crosses the flame's marched cylinder (18 samples of six noise reads; 1.3 %, 3.3 % and 15.5 % of the screen —
  the cylinder is 1.9 mouth radii wide, the flame itself lives inside 1.1). (THE SECOND CUT MARCHES 24 SAMPLES, not 18: a third more on the flame's own pixels, none on the embers'.) Set against the resolve's row (0.25 ms at High for work of that order: a count, good to about
  three times either way) that was 0.20 ms at High at 21 m, 0.53 at 13 m and 2.5 at 6 m; since `fireFlame` returns before any read for a point the wobble cannot bring
  inside the rim, and right behind its two wobble reads for one that ends outside it (exact: the flame came out 0 there anyway, and `fire-lab.cjs`'s 24 frames are
  byte for byte the same, in still air and in wind), about 0.13, 0.35 and 1.6. The ember tests are most of the
  closest zoom's price and are as they were. THE SMOKE IS THE REAL COST and the same at every preset: the solver stays awake while a
  fire is within 18 m of the view — the plume alone holds about 1,100 bricks after twenty seconds when the solver is only
  stepped (497, 942, 1,119, 1,216 at 5, 10, 15 and 20 s, against 110 to 140 with the fire off) and the DRAWN page's pool read 2,242 fifteen seconds in against 901 with
  the fire off (the pass's own cold-weather smoke), so about 1,300 bricks: by the solver's own figures on the maintainer's Mac (0.39 µs a stepping brick, and 0.43
  ms for being awake at all on a level where nothing else smokes) 0.5 ms on the pass and about 0.95 ms where the fire is the only smoke. The plume was still
  growing slowly at twenty seconds. `?firesmoke=0` is the same fire without it, to weigh that on a GPU. BY PRESET, and the
  one rung that draws the fire without its smoke: FIRE SMOKE SHED, the next bullet.
  THE THIRD LOOK ON A GPU (2026-10-23, the maintainer playing the pass with the third cut: "this is looking better but still seems way too dark, especially
  relative to the other lights in the scene"; "i'd also expect the color from the fire to be more warm"; "would also expect the fire to be affected by the wind,
  looks like only the smoke is affected? fire sound can also be more prominent and crackly"; "shadows cast by the fire should be more dynamic i think. the fire as
  an emitter should be shifting continuously over time"; "fire getting put out by a grenade is weird"). WHAT WAS FOUND, and it explains every GPU look of the
  flame so far (pale blob, static, dim and see-through): THE TEMPORAL RESOLVE AVERAGED IT. `taau.wgsl` blends this frame in at a tenth, and its anti-flicker
  term quarters that where a neighbourhood is all contrast, which a flame on dark ground is: some forty frames of forty different flames. Particles, rain and
  smoke had reactive inputs; the composite's flame had none. Now the composite writes `1 − fireReact` into the HDR target's alpha (it was a constant 1 nobody
  read; `fireReact` = the opacity of the flame and its drawn smoke, `fire.wgsl fireVeil`, not its bed of coals, × `FIRE.look.react`), `gbaa.wgsl` carries that alpha through its rewrite, the fog of war, the outlines and the
  particle splat already kept it, and the resolve takes `react` from it like any other reactive pixel. The software adapter's stills were one frame at a time
  and never showed any of this. `?firereact=0` = averaged, as it was. THE LAMP (`FIRE.lift` 0.5, `.reach` 1.4, `.warm` [1, 0.72, 0.51], `.bright` 1.25;
  `?firebright=0` = 0.25, 1, no tint, 1): the level's lamp is 9 where a room's bulb is 26 and the gate's flood 70, and it hung a hand over the lid, where the
  drum's rim shadowed the ground a metre and a half round it (the dark ring in his shot, which ate most of the fire's pool). MORE STRENGTH IS NOT THE ANSWER:
  the tone curve pales a bright orange on snow to cream (lit snow a metre off is 0.95, 0.88, 0.77 at strength 9), so strength reads as whiter, not warmer, and
  a flame cannot look bright over ground brighter than itself. Tried at × 3 in the real page (software adapter): the yard washed out and the flame went pale
  against it. So the lamp is lifted into the flame, tinted deeper, carries 1.4 times as far (6.5 m to 9.1 m on the pass) and is only a quarter stronger, and
  the brightness is the FLAME's: `look.gain` 2 (was 1), `look.core` 1.4 (was 1), chosen on `fire-lab.cjs`'s stage from the play camera on snow (`LAMPK`,
  `LAMPC`, `LAMP` are that tool's words for the lamp). THE GAME READS THE LAMP: the check's `lamp` line reads the light meter three metres off at 0.544 by
  the level's lamp and 0.884 by a fire's at × 2 (the line's own number, not the page's), and the steady readers are told the lamp as it is; the fog of war's
  remembered picture packs a burning fire's lamp at that same steady strength (`fogofwar.ts`, one expression: its colour and reach were the live lamp's already).
  THE LAMP SHIFTS (`FIRE.wander` 0.14, `.wanderUp` 0.10,
  were 0.06 and 0.04; `?firesway=0`): how far a shadow's edge moves; whether the shadow history averages that motion away is the direct light's question,
  not looked at. THE WIND LEANS THE FLAME (`FIRE.look.lean` 0.2, was a constant 0.07 in the shader; `?firelean=0` = 0.07): `fireBend` = lean · y^1.5
  through the flame's body (y in flame heights, to 1.5) and on at two fifths of that slope above it, for the flame, its smoke off the tips and the shimmer, so
  the flame lies over some 45° in the pass's storm (the wind handed on is capped at `windMax` 3 m/s) and the marched bound grows to about 2.1 m round the
  drum from 1.6 (a straight y^1.5 at this lean made it 3 m and more). With `?firelean=0` the smoke over y 1.5 leans a little less than it did (the same
  curve at the old number): the picture only. `FIRE_STEPS` 40, was 32. SOUND: `bed` 0.85
  (was 0.5), `crack` 0.9 (was 0.55), `popRate` 1.1 a second (was 0.55): the fire's own levels, no change to the sound table or the mixer; more cracks is also
  more flashes of the lamp and more throws of embers. A FIRE IS NOT PUT OUT BY A ROUND OR A BLAST (`FIRE.hardy`, `?firehardy=0`): `combat.ts`'s fixture rule
  and `destruction.ts`'s (`fixtureLive`, the blast's fixture loop, `fixtureRound`) take a fire that `fireHolds` for no fixture at all, so a lamp behind one can
  still be shot, and call `fireStir` where the hit lands (a crack at rate 0.7 and a throw of embers on the fire's next frame from the host's book, one in
  `stirGap` 0.3 s however long the burst); nothing is counted against the encounter's fixtures or `AIMCOST_STATS.lampHits`, no light event goes to the
  guards. Its own box blown away (`breakFixtureBox`) still takes it, and so does a level's own switch or script. Driven through the real `bang` beside the
  pass's brazier on the shipped stand: out with the rule off, burning and stirred once with it on.
  THIS REMOVES A WAY TO MAKE DARK: a brazier cannot be shot out any more, which is the maintainer's to weigh for the quiet missions. A recording from before any
  of these keys replays with the level's lamp, no reactive flame, the 0.07 lean and a fire a round puts out (`journal.ts`). The lab (`/firelab`) has a slider for
  each: how fresh each frame of the flame is, how far the wind leans it, how orange / strong / far-carrying / high the light is, how far it shifts, and a
  section for the sound. NOT MEASURED: tallies on the shipped stand with the brighter lamp (guards by a fire are better lit and see further into its pool);
  no soak or tour run. NEVER SEEN ON A GPU: the reactive flame (the whole point of it), the lamp's colour, the lean, the shifting shadows.
  THE MAINTAINER'S OWN NUMBERS (2026-10-24, from the fire lab's "copy them" line on his GPU, taken as sent and made the table's defaults): `look` gain 3, core 4
  (both the top of their sliders, which now go to 8 and 10), sigma 2.5 (a thin veil where it was 12), speed 0.7, tall 0.95, torn 1.1, shimmer 0.019, soot 0.5;
  `height` 1.15 (was 0.75: the smoke's foot, the gas's and the embers' heights go up with it), `coals` 0.26 (the lid's glow, which the GPU meter reads through
  the bounce), `depth` 0.4 and `kick` 0.6, 60 embers of 1.7 s and 4 mm, and the drifting smoke at density 1, heat 21, speed 0.9 (thinner and slower than the
  2.2, 30, 1.5 whose sight lines are measured above: less of it, nothing measured again). THE GAME READS depth and kick: with the level's lamp (the check's
  `meter` line, frozen stand, one seed, twenty seconds) the light estimate 1.25 m off swings 1.785 to 4.701 about a steady 3.350 (mean 7.1 % under) and 3 m
  off 0.338 to 0.806 about 0.533 (0.6 % under), where the figures quoted further up were for depth 0.3 and kick 0.25; the lamp's own long-run mean is about a
  twentieth over the authored (the kick's share at 1.1 cracks a second), so the check's two mean bars are a tenth now, were a twentieth. HE SET THESE ON THE
  BUILD BEFORE the rules of the paragraph above, so the picture with them is not the one he tuned: the nearest to that is
  `?firereact=0&firelean=0&firebright=0&firesway=0` (twice the cracks and `FIRE_STEPS` 40 have no word). TWO MENDS FOR HIS NUMBERS: the reactive mark is
  `min(1, 4 × fireVeil)`, since at sigma 2.5 ten centimetres of flame is only a fifth opaque and its tongues would have gone back to the history; and the
  lamp's shift up and down is held to `wanderUp` (the kick is past the swing, and at 0.6 over 0.4 it snapped the lamp 15 cm in a frame with each crack).
  NOT his: `wander` (he sent 0.07; his note asking for shadows that move came with it, so it stays 0.14). COST, not timed: at sigma 2.5 next to no ray goes
  opaque, so nearly every flame pixel runs all 40 samples, over a bound a sixth taller. `fire-lab.cjs`'s look, height, kick, swing and lid glow follow.
  NOT SEEN by anyone with `react`, the lifted lamp and the lean together.
  FOUR MENDS FROM A REVIEWER'S READ OF THE CODE (2026-10-24; a fifth read, that the lamp's newer rows ride `FIRE.on` on the frozen stand with no row of their
  own, is true and as meant: no frozen digest or fixture is recorded with `TPS_FIRE=1`, and the check pins them itself). (1) A FIRE PUT OUT WHERE NOBODY SEES is
  remembered in the fog as a fire's light: `fire.ts held` keeps its tint and reach and its steady strength while the fog holds its ghost, and `fogofwar.ts`
  takes the ghost's strength from that steady record (it was the level's, a fifth dimmer, paler and shorter, at the lid). It is held from the very frame it
  goes out, one frame ahead of the fog's say. ONLY under the fog's full picture with `FIRE.fog` on: with `?fogfull=0` or `?firefog=0` the fog remembers the
  level's lamp, as before. NEVER SEEN ON A GPU. (2) `fireStir`'s gap is each fire's own: a grenade between two braziers stirs both. (3) The player's round stirs a fire on the older lamp
  search too (`AIMCOST.lamp` off, or a show on the stage), and that search goes on past the fire to a lamp behind it. (4) A floor with no fire tells the solver
  to let hot gas go (`keepHot(0)`); the page stands one game on a fresh solver a level, so nothing reaches this today. The check's fog, hardy and gas
  lines hold each; (3) stirs on a round that STOPPED in the drum, the newer search's own rule. RUN SINCE, and it found the rule itself short: a fire's own box is its glowing lid, a finger thick, and the drum is four boxes of the level's that name no fire, so a round into the drum stirred nothing on any path (the line of October 23 called `fireStir` itself and never fired a round). `fire.ts fireStruck` now says a round stopped in a fire when it stopped in the fire's box OR in a box that stands on the fire's axis under its mouth, within the drum's wall (1.7 lids, 0.3 m at least; a wall behind the drum or the ground beside it is no drum); the player's two lamp searches and `destruction.ts fixtureRound` (everyone else's rounds) ask it. `fire-check`'s hardy line fires a real round into the drum from three metres by each search: one stir each, still burning.
  CHECK: `bun run tools/qa/probes.ts fire` (`tools/qa/fire-check.ts`, families `render`, `all`; twenty-three lines, both stands; `FIRE_BREAK=1` fails twenty-one): the lamp's swing
  and place, a second game the same frame for frame, the bed and the cracks, out and lit again, a checkpoint load (and the bed asked for again behind it; kept through frames that move no clock; asked for again behind a restart), the smoke's sources (look-only, coarse, foot over
  heads; none without a solver, on a preset that sheds it, or with the smoke put off; the game that smoked for twelve seconds and the one nothing drew differ in
  nothing of the fire's state), the steady readers, the lamp's size (0.24 m burning, the steady readers told the level's 0.06 and reading the same to the bit a hand from the lamp; the level's
  size with `size` 0, with the switch off and in a recording without the key), the CPU estimate (1.25 m off: steady 3.350, burning 2.02 to 4.24, mean −9.3 %; 3 m: 0.533 against 0.35 to 0.71,
  mean −3.1 %; with `lift` 0.3 the near one read −11.9 % and the line was red, which is why the lamp stands at 0.25), off = the build as it was, an old recording. THE SHADER ALONE: `node tools/qa/fire-lab.cjs` (seconds; where the look was set). THE REAL PAGE:
  `node tools/qa/fire-page.cjs` (software adapter, minutes; EVERY FIGURE AND STILL FROM HERE TO "NOT DONE" IS THE FIRST CUT'S, lamp 6 cm and `lift` 0.2: of the
  second cut the page was drawn at High from the still's own view, `QUALITY=high VIEWS=shot W=640 H=350`, 119 of 119 pipelines, two stills: a red and orange flame
  with a yellow root standing clear of the snow, the drum's shadow a soft dark foot round it — on the octagon drum, before ROUND joined): it boots with the switch either way (119 of 119 pipelines, no page error) and
  the stills are on the fire's page (rebuild/fire/): a flame on a bed of coals with the man beside it, the wall and the snow lit warm, a pale plume leaning downwind — against
  a white lid. THE GPU METER there, the same sixteen frames of the same game with the fire and without (the player's `visibility`): at 1.25 m 0.941 to 0.949
  burning against 0.952 to 0.955 steady; 3 m 0.635 to 0.717 against 0.605 to 0.631; 5 m 0.375 to 0.674 against 0.398 to 0.608 (the storm's lightning is in both). Half
  a second a stand: enough to say the flicker moves the meter by hundredths, not to give its swing.
  SEEN ON A GPU, ONCE (2026-10-23, the maintainer's Mac at High on the pass, one still; his words: "fire looks a bit jank tbh"). What the still showed, none of
  which the software adapter at Low had: (1) THE FLAME WAS A PALE SOFT BLOB with no tongues and no red. It was thin (`FIRE_SIGMA` 8) and ADDED to snow its own lamp
  lights as bright as it is, and its heart was radiance 5: the tone curve (AgX at exposure 2) turns a red above about 1 and a yellow above about 2.5 to cream, so
  brighter only read as paler. (2) A HARD BLACK DISC A METRE ROUND THE DRUM: the drum's own shadow from a lamp 6 cm in radius 36 cm over its rim. (3) THE COALS
  READ GREY: their gaps were radiance 3 and up, cream by the same curve. THE SECOND CUT (the same day) is the numbers above: the flame hides what is behind it and its
  colours stay under the curve's shoulder, the lamp is the flame's size and a little higher, the coals' gaps are 1.15 × a deep orange. It was set in `fire-lab.cjs`
  from his camera (`DIST=7 PITCH=50 ALB=0.85`: snow; the stage's lamp now casts the soft shadow, `LAMP` and `SIZE`) and
  `fire-page.cjs` has his view (`VIEWS=shot`, `DPR=2`). THE SECOND CUT HAS NOT BEEN SEEN ON A GPU.
  NOT DONE, KNOWN: the flame's clock wraps
  every 68 minutes (one frame's jump in its shape); `brazier*` is a name rule, not a level key; a brazier can still be shot out or jammed and the flame dies with
  its lamp. NEVER SEEN ON A GPU OR HEARD: the flame in motion through the temporal resolve (its 18-step dither, one-pixel embers under the clip), the flicker's
  feel on the yard, the plume, the sound, and every cost above.
- **THE LOOK PAGE'S KIT** (11-01; `src/main.ts`, the block after `loadSandbox`; the maintainer: "for the look lab, can you make it so we have unlimited
  ammo and all equipment? want to play with smoke grenades", "turning off fog of war would also be good for /look too"). On the look page alone: the
  unlimited ammunition row is up from the start (so a thrown can goes back on the belt and a magazine never empties), Sam is never without two cans of
  smoke, two stun charges and two frags whatever card he was dealt, and the fog of war is down unless the link asks for it by name (`?fow`).
  `?lookkit=0` = the kit as dealt, the sandbox row as saved and the fog as the link has it. Not topped up: the rifle's rings and shells where it has no
  tube, sticky cameras, the wand, wall charges. CHECKED: the look page booted on the software adapter with the kit lines (ammunition up, two of each
  can, no error); the fog line was added after that boot and is typechecked only. Never played.
- **THE LOOK PANEL'S "FIRE SMOKE" ROW** (10-28; `lookConfig.fireSmoke`, 1 on the look page, 0 = the fire's smoke as the build files it;
  `game/fire.ts FIRE_LOOK_SMOKE`; the maintainer, of a drum fire on the look page by day, on a GPU: "smoke plume from fire looks a bit weird,
  disconnected from fire. should be billowing more"). TWO CAUSES, both in `FIRE.smoke`. DISCONNECTED: the build files the gas with its middle 1.9 flame
  heights over the coals (2.2 m over them: its foot is above a standing man's head so that no sight line near a fire is cut, the bullet above), and a
  camera that looks down on the yard shows that height as a gap beside the flame. With the row up the steady source's middle is `up` 0.8 of the flame's
  height over the coals, a little narrower and thinner, so the gas climbs out of the flame's tips. NOT BILLOWING: a steady source makes a smooth column.
  With the row up the fire also PUFFS: every `puffEvery` 0.42 s a burst of thicker, hotter gas for `puffSecs` 0.26 s, a little off the middle and never
  twice alike (the game's clock and the fire's own seed, no die), with a divergence of its own (`puffExpand`, smoke/types.ts `expand`: it pushes the air
  round it apart, which the solver rolls up). All of them LOOK sources (`look: true`), so the headless stand's own smoke hears of none of them.
  **THE PRICE, to be settled before any of it goes to the build:** in the page the men's sight lines are read off the solver, so a line that passes
  within a hand of the drum between chest and head height now meets smoke; nobody has measured how much (`fire-page.cjs` asks those lines).
  SEEN on the software adapter only (the pass at dusk, the solver stepped 10 s with the row up, then 14 s with it at 0; 480 × 300, a play view and a
  close one): with the row up a pale column leaves the flame's own tips, warm at its foot, and widens as it climbs; with it at 0 the gas shows first well
  over the flame. Whether it BILLOWS cannot be judged from a still at that size. NEVER SEEN ON A GPU; the numbers are a first setting.
- **A FIRE AS ONE COLUMN, ON THE LOOK PAGE** (10-29; `lookConfig.fireFit`, on with the Fire smoke row up, `?firefit=0` = the pieces as they were;
  `game/fire.ts FIRE_LOOK_SMOKE` and `axisAt`). The maintainer on a GPU, with a simulated fire beside his shot of the pass's drum: "interaction of smoke
  with embers, heat distortion, hot part of the plume doesn't seem to look right. they should fit together more naturally?" READ OFF HIS SHOT: the gas over
  the flame is a pale glowing haze (white solver smoke a hand from the fire's own lamp); the soot the flame draws is dark and stands on the flame's
  leaning line while the gas was filed straight over the mouth; the embers leave the coals in a wide cone. WITH IT ON: (1) the steady source and the
  puffs are born on the flame's own middle as `fire.wgsl fireBend` leans it (`axisAt` is its CPU twin: the capped wind x the look's lean) and leave along
  that line's slope, not the old lean on top of it; (2) the gas is SOOT (`soot` [0.34, 0.31, 0.29], an emitter `tint`), so the volumetric pass's dust twin
  draws it: lit warm from below by the fire, grey above, thinned at its edge and broken into billows with a lit side; (3) the drawn soot is only the
  flame's dark tips: x 0.6 and gone by two flame heights, by each fire's OWN share (`FireRec d.z`, `FireShown.sootMix`, eased 0.5 a second and up only
  while that fire's gas is filed, so a fire past `smoke.viewR` or on the rung that sheds its smoke keeps its drawn plume); (4) embers leave the upper
  flame (`emberUp` 0.72 of its height) on the leaning middle, along it, in a cone of 0.2 rad at 1.5 x the table's speed; a crack's throw is wider from
  the same place. The shimmer already followed the lean. No knob, no journal key: host's book and look sources only (`look: true`), nothing the game
  reads. `tools/qa/fire-lab.cjs` takes `LOOK=fit=1` for the handover alone.
  **THE PRICE, to be settled before any of it goes to the build:** a tinted source keeps `SmokeBinding.dusty` up, so the dust twin of the volumetric
  march (three noise reads and four more density taps a sample in the soot, four more a light for its self shadow) runs on every frame a fire is in
  range, where it ran for seconds after a blast; nobody has timed it. The twin draws a cloud's edge thinner than a sight line reads it
  (`dust_look.wgsl`), on top of the Fire smoke row's own price above.
  CHECKED: typecheck; `fire-check` 23 of 23 on both stands; `shaderrows-check`, `dice-check`, `rollback-check --static`; the fire's shader alone
  (`fire-lab.cjs`, with and without the handover) and the look page (119 of 119 programs, no error) on the software adapter.
  SEEN on the software adapter only (the pass at dusk, 18 drawn frames a tenth of a second apart and four more, 480 x 300: a play view and two from the
  side; a frame there is over a minute): the embers climb in one narrow stream off the flame's tips along its lean; the flame's tips go dark red into a
  thin grey column on the same line; from above the soot stands over the flame as a soft dark cloud, bluish under the dusk sky, with little of the
  fire's own light in it (whether the lamp warms it on a GPU, where it made the white haze, is not known, nor whether 0.34 is too dark). NEVER SEEN ON A
  GPU, never in motion; the numbers are a first setting.
- **THE FIRE ON THE LOOK PAGE, AFTER HIS GPU LOOK OF 10-30** ("embers not visible in fog of war", "overall fire looks better but i can't really see the
  smoke now", "would it be possible to have the fire fluid sim influence the snow particles?"). Three things, all look page only, in `game/fire.ts
  FIRE_LOOK_SMOKE`:
  (1) EMBERS UNDER THE FOG (`emberFog`): `FIRE.fog` keeps a known fire's flame in the fog's picture and the particle draw cut its embers there like
  anything the team did not make, so the drum behind Sam's back burned without sparks. An ember of a fire whose flame the fog keeps (`K.fog` and the lamp's
  class is not 1) is now filed with the particles' `ours` bit up, as the team's own effects are; a fire never seen lit keeps neither. The build cuts them
  still (the fog's owner's call). It follows the fog's own row: with `FOG.ours` off (`?fogours=0`, an old tape) no ember is kept. `?emberfog=0` = as it was.
  (2) THE SMOKE WAS ERASED BY THE DUST DRAWING. `dust_look.wgsl` thins a cloud within 0.2 to 0.5 m of where it ends, right for a room of dust; a fire's
  column is 0.2 m wide, so every sample of it was an edge and it was drawn at about a tenth. The software stills of 10-29 did not show it because they
  were taken in still air after the gas had pooled round the drum. Now a cell whose tint's blue is over its red (`DUST_FIRE_AT`) is SMOKE OF A FIRE: it
  keeps the tint, the billows and the lit side and is neither thinned at its edge nor frayed (and skips the four edge taps). Every dust in the game is
  warm or neutral (`game/dustLook.ts`, the burst's `smoke`), so none is read as that. `soot` is now a cool grey [0.46, 0.48, 0.53] (was 0.34);
  `?firesoot=0` = the warm soot, so the column thinned away as it was. THE RULE RESTS ON A COLOUR: the first cool dust filed on the coarse lattice
  would lose its edge too, and a `soot` retuned to neutral loses the column again. A bit of its own in the cell's word is the mend, and there is no spare.
  (3) THE FALLING SNOW (`lookConfig.fireSnow`, `?firesnow=0`; table `snow`). The flakes are GPU particles carried by the wind's records and the air
  field; the smoke solver's velocity is not published to them. So each fire near the view is filed with the particles as one more record of moving air
  that only the falling snow reads (`render/particles.ts setHeat`, a jet record with iflags bit 6, which `windAir` skips; `particles.wgsl heatAir`, in the
  compact kernel too): a jet up the flame's leaning middle (2.6 m/s at the mouth, halved 2.2 m up), a draught in toward the fire round it (0.3 of that,
  fastest 0.9 m out), and a flake in the hot of the column melts (`HEAT_MELT_AT`, `HEAT_MELT_RATE`; how hot is the air's speed against `HEAT_FULL`, a
  full fire's, so a fire burning low melts less). A closed form beside the solver: the two agree on
  where the column stands and how it leans; a puff's roll does not move a flake. Publishing the solver's own velocity to the particles would be the real
  thing and is not built. The kernel's text is behind the prelude's `LOOK`, so the build's kernel is as it was. The records ride behind the wind's in the
  same block and are the FIRST LEFT OUT when it is full (eight records, less a blast frame's impulses). A fire put off in a running page clears them.
  THE CHECK: `tools/qa/fire-check.ts`, the line 'look page' (kept embers, one jet up the fire's middle, none off the look page, with the rows off or
  with the fire put off; `FIRE_BREAK=1` fails it), on both stands. SEEN on the software adapter only (480 × 300, the pass, the page booted
  with the fog on and Sam in a room away from the yard): at dusk the embers are drawn over the flame in the picture taken with the fog on, and with the
  fog put off the smoke stands as a pale column from the flame's tip to about two metres over it, as wide as the flame, faint against the wall; whether
  the drum stood on floor the fog hid in that picture was not proved (the check line holds the rule). AT MIDDAY THE DRUM IS OUT: `game/hours.ts` puts the
  level's lamps out by day and a fire is a lamp, so there is no flame, smoke or hot air to see then. The flakes round the fire were never seen moving,
  on any adapter. NEVER SEEN ON A GPU.
- **The floor rung draws an open fire without its smoke (FIRE SMOKE SHED: `settings.fireSmokeShed`, `ui/quality.ts fireSmokeOf`; 2026-10-23).** Asked when the fire
  was merged: price it by preset, and say whether a preset should drop its smoke. The maintainer's rule stands (the smoke stays near High and other things pay the frame
  budget first), so this is the one rung with nothing left to pay. THE PRICE, by the cost model and timed on no GPU: the fire's smoke is the solver's fixed 0.5 ms (other
  smoke already keeps the solver awake, as the men's breath does on the pass) to 0.95 ms (the fire is the only smoke) at EVERY preset — 1,300 bricks at 0.39 µs a
  stepping brick, + 0.43 ms awake; every preset solves with high's four pairs since ONE SMOKE SOLVE, which is what that figure was measured at. Against each rung's
  modelled compute at its own scale on the maintainer's Mac, and what the ladder's render scale gives up on a machine that only just holds the rung at its 15 ms:
  ultra 31.3 ms, 2 to 3 %, 1 to 2 % of its rows; high 14.5, 3.5 to 6.5 %, 3 to 5 %; medium 8.7, 6 to 11 %, 5 to 10 %; low 4.6, 11 to 20 %, 12 to 24 % (past the
  0.5 scale floor where the fire is the only smoke: that machine hops to verylow beside a fire); verylow 2.9, 17 to 32 %, and it cannot be paid — its whole per-pixel
  part is 1.0 model-ms on the scale floor, 15 ms becomes 17.6 to 19.8. The flame's own price is the bullet above. THE RULE: a preset with `smokeOwn` (verylow: the rung that
  already keeps its own cheap smoke under SMOKE KEEP, for the same reason) draws the fire with its flame, embers, light and sound and files no smoke source; every
  rung above keeps the smoke and the scale pays. UP FROM THE FLOOR THE SHED STANDS ONE RUNG (`fireSmokeFor`): auto prices the hop verylow → low off a frame measured
  with no fire smoke, so a low that put the smoke back would be 0.5 to 0.95 ms short beside a fire, fall to the floor, find room and hop again, the plume coming and
  going each time. While AUTO moves the preset, low reached from verylow keeps the shed; the smoke is back at medium (where the scale has the room: the hop low →
  medium is priced without it too, and medium's scale has 6 to 11 % to give) or when a preset is put on by hand; a machine that came DOWN to low keeps its smoke. HOW: `Quality.apply` writes `settings.fireSmoke` (render-side, never `FIRE.smoke.on`: that table rides the journal
  and a graphics preset must not write it), `game/fire.ts updateFire` reads it beside the solver's `drawing`; a fire that gets its smoke back (a hop up the ladder)
  smokes the same frame, and one that loses it just stops filing: the plume there thins out by itself and the solver sleeps when its bricks are let go. SWITCH:
  `settings.fireSmokeShed`, default on, `?fireshed=0` = every preset keeps the fire's smoke, as it was (render-side like ONE SMOKE SOLVE: no `FROZEN_HOLDS` row, no
  journal line; `?firesmoke=0` still puts the smoke off at every preset). WHAT IT IS NOT: a game rule on any stand (the source is look-only, and no headless stand
  has a solver that draws) — but in the PAGE a sight line reads the solver, so beside a fire a line over a standing man's head (the fire's notes: 0.92 to 0.94 kept
  at 1.9 m, 0.55 at 2.6 m, all of it at 1.7 m and under) reads the plume on low and up and none on verylow. Nobody in the game looks along such a line today.
  ALSO RENDER-SIDE BY WAIVER, to be confirmed: the key is read inside the game's update (beside `drawing`, itself a render setting). What it
  decides is kept out of the game's state: the fire's book of which smoke it filed and which bed voices it asked for is on the HOST, beside the list the
  renderer is handed (`engine.hostBooks.fire`, `fire.ts FireShown`; `host/api.ts HostBooks` is the one bag for such books, dropped whole by the put-back's one line, so a book kept there cannot be forgotten; `Game.engine` is in no snapshot), and `fire.states` holds only what the clock alone decides — so a
  preset or a solver moves no state hash through the fire (the fire check's line 'the host's, not the game's': the game that smoked for twelve seconds and the
  one nothing drew differ in no row of the fire's, and the lamp and the coals are the same to the bit). WHO ABANDONS THE MOMENT DROPS THE BOOK: a restart, and
  with it a file's or a checkpoint's load (`game.ts restartEncounter`, beside the line that empties the solver), and a save in memory put back
  (`host/putback.ts`); a fresh one is begun on the next frame that moves the clock, the smoke is filed and the bed asked for on that frame, and the abandoned
  moment's voice lets go — where before a load into an earlier twelve seconds left a burning fire silent until the next mark (the check's second load line,
  which runs the host's own `hostAfterPutBack`, `FIREBOOK_BREAK=1` undoes the put-back's line and fails it; and its 'restart' line: the clock does not move
  with a restart, a source and a bed on the next frame, once). A FRAME THAT MOVES NO CLOCK IS NOT A LOAD: the page updates the game with `dt` 0 for the kit
  page's still and a card's frozen still (a pause skips the update), and the first cut of this rule took every such frame for a load — the bed's voice was let
  go and asked for again and a source filed on the next frame (found by a review; the check's 'still' line and the preset line's last count were red on it).
  A second cut read the clock instead (a book is another moment's when the clock is not where it left it) and lost the restart, which does not move the
  clock: found by the next review before it left the branch. The book is kept by a fire's place among the level's, a frame stamp each, with no limit on how
  many. NOT KNOWN TO IT: whether the mixer still holds the bed (a page frozen past the mixer's own hold, `audio.ts FOLLOW_SECS`, resumes with no bed until
  the next twelve-second mark). A page replay under another preset
  still files other look sources (the men's breath kept its own word for the solver in the game's state until the step after: its book is the host's too now, `engine.hostBooks.breath`, `weather.md`). NOT DONE: the
  other look sources (the men's breath, the cigarette) still wake the solver on verylow, so on a level with them the shed saves the plume's bricks (0.5 ms) and not
  the 0.43 ms of being awake; `settings.fireSmoke` is written when a preset is put on, so the switch moved by hand on a running page waits for the next preset.
  CHECK: `bun run tools/qa/probes.ts ladder` (four lines: by hand, under auto up from the floor, the switch off, a bare mock; `FIRESHED_BREAK=1` fails two) and
  `probes.ts fire`'s line 'on the graphics preset that sheds it', the real `Quality` over the game's own settings (verylow put on: no source in four seconds; low put
  on: a source that frame; both stands; `FIRESHED_BREAK=1` fails it). THE REAL PAGE (the software adapter, the pass, 640 × 360, the player beside the brazier, the camera at 13 m; a
  frame is twenty seconds and more there, so this is half a second of the game): verylow pinned, on this text, boots 119 of 119 pipelines with no page error, `fireSmoke`
  false, the flame drawn over the drum (the shader's two early returns compiled into the composite), and in sixteen frames no source filed and no brick in the solver's
  pool; low put on in that page: a source that same frame, 64 bricks, then 151 fifteen frames on. With `?fireshed=0` (a page from before the auto rule and the
  shader's first bound were written; stopped early, so its error list was never read): verylow boots 119 of 119, `fireSmoke` true, a source filed within the first
  eight frames, 125 bricks. NEVER SEEN ON A GPU, and nobody has watched the ladder move beside a fire: the auto rule is the self-test's alone. A take that would settle the prices: the pass and mission 1's level by the fire, F8 at high and low, the page / `?firesmoke=0` / `?fire=0`, at the zoom the
  game opens at and at the closest; and a fire under a roof, where the plume pools (never measured).
- **SURFACES, A TRY (`?surf` at boot; `render/frame.ts surfTryConfig`, OFF; the scene prelude's const `SURF_TRY`, `shaders/surface.wgsl`, read in `gbuffer.wgsl fsBox`
  after the detail layer and before the wet ground and the markings):** an answer to "what would textures and normal maps buy on today's boxes", NOT the look
  (`STYLE.md` rule 1 stands until the maintainer says otherwise; the picking of it is theirs). With the word, a box's PAINT is classed against `GWOT_PALETTE`
  (`gbuffer.ts surfaceWgsl` writes `surfClass`: an exact test per palette paint, then a darkened-paint test for the classed ones; eight classes: mud wall, plaster, block,
  earth, gravel, slab, wood, sheet) and the class's written pattern changes the pixel's albedo, normal and roughness in the g-buffer: no image, no binding, no
  level data. Capped faces, glowing boxes, turned boxes and paints of no class are as they were. THE RELIEF IS HELD UNDER `SURF_TILT` 0.18 so the shading normal stays
  inside `bounceLeftNoWall`'s 0.98 (a normal further off an axis asks the walled bounce read); detail finer than the pixel's footprint is faded out. WHAT SEES IT:
  the g-buffer's readers only (direct, the filters' edge stops, composite). Rays, the light cache, reflection hits, the cards and the LIGHT METER read the box's
  flat paint as ever, so nothing a man senses moves. Without the word the const is false and the branch is dead code, but it is still TEXT: the prelude gains the one const
  line and the two g-buffer modules carry `surface.wgsl` and the generated `surfClass` (about 16 KB each) on every page. `surf` rides a replay page's address
  (`ui/replaykey.ts VISIT_FLAGS`). KNOWN WRONG, left so because it is a try: THE CLASS IS A GUESS FROM THE COLOUR, so a paint outside the GWOT palette that is
  a darker shade of a classed one takes its class (on the office: lino, stone and concrete draw as block, cartons as mud wall; try it on the GWOT levels), and
  rain-darkened earth keeps or changes class while wet gravel and slab go plain; the pattern is laid in WORLD space for six of the eight classes, so it stays
  put while a pushed prop slides under it; the footprint is the across-the-ray pixel size, too fine on a wall seen at a low angle (expect crawl there); relief past
  `SURF_TILT` is clipped to a facet (the ribs of sheet metal most of all); a box yawed a few degrees can still lean past 0.98. A cut box's own faces keep the
  pattern, its cut faces are bare. No check holds any of this. SEEN: on the software adapter only (the compound, squad of four, very low, 800×500, one seed,
  night and midday, 118 of 118 pipelines, no GPU or console error, the same two views with and without the word). Never seen on a GPU: the look at the play
  resolution, motion, shimmer under the temporal blend, the noise it adds to bounce light, and the cost (it is raster work, which no trusted timer covers).
- **THE LOOK PROTOTYPE** (`?look` at boot, or the page `/look` = the game's page with `look` on the pass at night unless the link names its own `levelFile`; `?look=relief:1.4,air:2` opens it with those
  amounts, each held to 0‥4, and `?look=plain|0|off` stays the GWOT dress's word and does not raise it; `render/frame.ts lookConfig`,
  `shaders/look.wgsl`, `render/lookTex.ts`, the panel `ui/lookpanel.ts`; OFF on every other page). A place to find out how far materials carry a world of boxes,
  NOT the look when it was made; the maintainer said yes to a material look on 11-01 and `STYLE.md`'s rule 1 and Surfaces are rewritten for it on this branch (the bullet THE MATERIAL MAPS ARE THE LOOK NOW, below, has what changed with that). Where the surfaces try above writes a pattern in the shader, this one samples MAPS:
  eleven tiling materials (the try's eight classes and three more for what stood untouched beside them: cloth, bare metal and painted metal; the class is
  `lookClass`, the try's table with those rows added, `gbuffer.ts LOOK_OF`) made on the CPU at boot, 512 × 512 over 2 m, as two mipped `rgba8unorm` texture
  arrays (A = paint multiplier ÷ 2 + height, B = tangent normal + roughness + cavity; no image file ships). They sit at the g-buffer object group's bindings
  11 to 14 with a repeating, anisotropic sampler and a 96-byte uniform block, ALL FOUR ONLY ON THAT PAGE: without the word the g-buffer modules get a stub of a few lines and the bind group is the page's as it was. What stays on every page: the prelude's `LOOK` / `LOOK_BOUNCE` consts, `lookGeoN` in `common.wgsl`, and one
  flat `u32` varying on the box vertex output (zero). FOUR LAYERS, each a slider on the panel (`gbuffer.ts lookAmounts`, 0 = that layer off, all 0 = the boxes
  as the game draws them, by an early return: none of the reads run): the maps (paint, relief, finish, cavity); the repeat broken for the four materials without courses (a second read, turned and
  shifted, cross-faded by a slow noise and by height); weather that follows the BUILDING (a wall's foot damp to a ragged line with its tide mark, its head
  bleached with runs under it, slow blotches, and on the ground the level's traffic map: walked earth pale and flat, stones against the walls); and small
  boxes' edges rounded in the normal and rubbed pale. THE NORMAL: `fsBox` stores the tilted normal with the normal target's alpha 0 (a code no other writer
  uses for a surface EXCEPT ON A SNOW LEVEL, where the alpha is the snow's mark in four steps: 1 plain, 2/3 thin snow, 1/3 snow, and 0 was only the clear.
  There the tilt takes 0 where no snow lies, a snow pixel keeps the snow's normal and its mark, and on the look page `composite.wgsl snowGlowAmt` reads an alpha
  under 0.17 as no snow. Every read and write of that alpha the prototype adds is a line of its own tagged THE LOOK PROTOTYPE in its comment, and
  `snowglow-check` and `snowsparkle-check`, which hold the mark's writers and readers by their text, drop tagged lines before they read). `gdown` hands the working-size targets the box's own
  FACE (`lookGeoN`: the box axis the normal leans on most, exact while the tilt is under `LOOK_MAX_TILT` 0.48; a mark laid over the relief, `decals.wgsl`, is laid on the face and the two are held under the cap together; a tilt under 0.02 is stored as the face, unmarked), so the reservoirs, the filters, the gather,
  the reflections and `bounceLeftNoWall` read the flat wall they always read. Three readers see the relief: `sharp_direct.wgsl` scores each light that reaches the cell a second
  time on the tilted normal and re-weighs each class's result by that ratio over the live lights' sum (capped at 4; a light the face itself turns from gives
  nothing), `composite.wgsl` leans the bounced light by how far the facet looks up (`LOOK_BOUNCE` 0.7), and the fog of war's picture (`fogofwar.wgsl`, which
  has no id to find the face by) shades what it remembers with it (the cap keeps a wall's facet under its `N.y > 0.5` test for a top). IT NEEDS SHARP DIRECT (every preset has it): with `?nosharp`
  the working-size passes read the tilted normals themselves. Rays, the light cache, reflection hits and the LIGHT METER read the flat paint as ever, so
  nothing a man senses moves. KNOWN WRONG, left so because it is a prototype (the first two and the last were mended on 11-01, the bullet below): the class was a guess from the colour a corner was handed; the maps were
  laid in world space for six of eleven classes, so a pushed prop slid under them; relief past the tilt cap is clipped; the bounce's lean assumes its light
  comes from above, and on the ground a facet can only lose bounce, never gain it; a facet lit by a light that only grazes the face gets at most four times
  that light, and a light that reaches the cell but is shadowed at the pixel still has its say in which way the relief reads; the edges rule is by size (a box
  under 1.3 m on its long side and 1.6 m tall), so two small boxes laid edge to edge show a groove; the maps cost about 5 s of the boot and 31 MB of texture. No check holds any of this.
  ON A SNOW LEVEL (the second round, made for the pass; the panel's 'Snow that clings', 0 = none of it): `look.wgsl lookSnow`, called from `fsBox` for the
  level file's own boxes, reads the snow field's cover. A TOP left out under the sky (a crate, a stack of planks, a sill, a gatepost) takes a cap of the
  ground's own snow with a ragged, domed edge. A FACE that stands on open ground takes a bank against its foot, higher on the side the wind blows at,
  with the snow's normal and mark; above the bank a thin rime and round clots that fade a stride up, which are paint and finish only. Never on bare metal
  (stoves, flues and braziers are warm), on a lit, turned or glass box, or on a box painted as snow or as a path (a marking on a face is covered like the face). BARE EARTH AND GRAVEL within 0.3 m of lying snow are
  frozen: frost grains, and dark ice in the low places, smooth enough to show a lamp. LYING SNOW (`look.wgsl lookGround`, one tagged line in `fsBox` and one in `snow.wgsl fsSnow`, after `snowShade`): its whole relief (the drifts' slopes,
  the wind's ridges, the prints and their rims) × the panel's 'Snow relief' (1 = as the build draws it; the page opens at 1.5), and 'Snow crust': fresh snow
  the wind has glazed, in streaks drawn out along it, takes roughness 0.56, which is over every preset's `rt.maxRough` so no glaze asks for a reflection ray.
  The steeper normals are the snow's own, marked as snow: the working-size passes read them as they read the build's (the tilt is not handed back as a
  face), so more relief is also more for the filters to hold still. AIR (the panel's row, `lookConfig.air`; 1 = the level's own haze, 0 = none) is × `settings.hazeDensity`
  in the frame block and in the beams' air line. Known wrong here: a cut piece (`gbuffer_cut.wgsl`) takes no clinging snow, so a wall loses its bank where
  a blast cut it; the frost reads the field, which knows no roof, so an earth floor inside an outer wall takes a hand's width of it along the wall; the
  bank and the caps are paint: no ray, bounce or meter knows them, as with the snow's own hills.
  SEEN: on the software adapter only (round one: the compound at midday; round two: the pass at night; squad of four, very low, one seed, 119 of 119 pipelines, no GPU or console error). Never seen on
  a GPU: the look at the play resolution, motion, shimmer, and the cost (anisotropic array reads are raster work, which no trusted timer covers; read the
  page's fps with the panel's "stay plain" box on and off: that measures everything but the class test a corner and the wider bind group).
- **A GEOMETRY TRIAL** (`src/scene/geoTry.ts`; on only on the look page, `?look`, with cut boxes on; `?geo=0` or the look panel's Shapes button
  leaves it out; no switch table and no state of its own: `main.ts geoTryOn` reads the link). Asked 2026-10-23: "how viable other modeling systems
  (low poly triangles, analytic surfaces, SDFs) are for geometry", then the trial picked from a card. The answer tried is the CUT BOX
  (`scene/cut.ts`), because the raster, every GPU ray and every CPU ray already meet it as one solid: nothing new to keep in step. It runs once,
  after the level file loads and before its boxes go to the world (`bootLevel`), and sets `cutConfig.swap` false at boot (its cut boxes stand for
  the whole visit, so the one set of ray programs with the cut rule is compiled from the start).
  ROUND THINGS (`roundStacks`): the kit's octagon (`gwotKit.ts round`: four slabs 45° apart) becomes ONE regular twelve-sided prism, the box's four
  faces + eight planes (the record's whole room), with `Box.cutRound = 2` (packed in the core row's w as 1024 × (1 + the axis); never with glass. Until the merge of 10-25 it was 128 ×: the build's ROUND THINGS, `scene/round.ts`, took 128 as their own bit, a cylinder about the box's y that every ray meets; `gbuffer_cut.wgsl cutRoundAxis` reads both and shades both with the one block, so on the look page a level's drum is the build's round thing and the trial's prisms are the lorry's wheels and the logs).
  The other three slabs stay in the list, hidden (`NoPrimary | NoShadow`, shrunk): a level file's box indices are names. In `gbuffer_cut.wgsl
  fsBoxCut` a round's side faces are one face: the normal turns with the point, marks run round it (the long face's band is put on all four sides;
  a lid's one-paint patch is drawn again as a disc; a drum's two-paint lid is let go and the shader draws a rolled rim). The look's paint and its clinging snow
  are rules that run along a flat face, so each is asked a quarter of the round at a time (the box face the normal leans to) and what it leans the
  normal by is added back onto the round; such a pixel is never marked as a tilted normal (`common.wgsl lookGeoN` would snap it to a box axis). The
  silhouette, the shadow and every ray keep the twelve flat faces. The snow hook knows a round about y only. On the pass: 19 octagons (drums, tyres, two stoves, the oven), so 57 fewer boxes for a ray to meet.
  A BROKEN WALL TOP (`breakWallTops`): a yard wall gets one bite where nothing tall stands against it and no lamp or alarm hangs, no nearer than 10 m to
  another bite (`BITE_APART`). A YARD WALL IS A WALL WITH THE SKY ON BOTH HANDS (changed 11-02; asked that day: "new assets or things for other
  levels", because the tests will run on more than the pass): a plain upright box 3.4 m or longer and under 0.4 m thick, standing on the ground, with
  free floor under the open sky two cells out on both sides of it the bite's whole length, on the renderer's own floor plan (`render/placeField.ts
  skyPlan`: the plan the light of a place is read on, made of the level's boxes before the trial). A house's wall has a roof on one hand and is never
  one. Until then the test was a coping slab of the wall's own length on its top, which only `gwotKit.ts` lays, and "something tall against it" was
  asked by a thing's LONGER side both ways, so a wall ten metres long five metres off kept every bite away: the rule fired on the pass alone (2), and
  on no level built before the kit. Now, from the files as they are: the pass 3, the compound 4, the crossing 6, the police post 5, the bazaar 3, the
  depot 2; the block, the records office, the warehouse and the barracks none (they have no wall in the open). A coping is one thin plain slab in
  the wall's line that runs on past the bite both ways, whatever its length (one slab may cap several walls), and is broken with the wall, as is the
  snow a level laid on it; anything else lying on the top there, or drawn on the wall's face by it (an alarm's housing is two boxes no ray meets),
  keeps the bite off. A BITE IS THREE TO FIVE STRETCHES, NONE UNDER 0.42 m (`PIECE_MIN`, `BITE_LINE`): the floor plan takes an upright box under
  0.4 m long for a post, and the five stretches a bite was until then were five posts to it, a gap in the wall on the plan `wallSides.ts` keeps the
  light of two places apart by (so on the pass's two bites for as long as the trial has stood). `breakWallTops` returns the bites it made.
  The check: `bun run tools/qa/geotry-check.ts` (11 lines: a yard built by hand, a roof on one hand, one coping and its snow over two walls, posts,
  lamps and alarms, every level's counts, no slab over either hand of a bite, the 2 m line, nothing left in a gap, the floor plan's wall cells;
  `GEOTRY_BREAK=1` takes every spot for open sky and fails three). A LEVEL'S COUNTS ARE A FILE OF ITS OWN since the join (11-04):
  `tools/qa/fixtures/geotry/<level>.txt`, "<round things> <bites>", no file = none of either, written in the commit that changes the level (the
  folder's README). Until then a table of ten levels stood in the check's source, every level commit met in it, and it was stale the day the kit's
  tyres and drums became round things (the depot: 10 for 0). EVERY level file is asked now, 44 of them and 106 bites where ten levels gave 32, which
  found one fault of the check's own: it measured a turned box by its unturned size, so a container stood across the gwot yard read as hanging in a
  bite three metres away. THE FINGERPRINT MOVED on every level that gained or lost a bite, the pass among
  them (one of its two bites moved 4.9 m along the road wall and a third came in the west wall): look-page checkpoints and recordings of those
  levels from before are refused, and `?geo=0` still takes the round things out with the bites. Seen: stills of the compound and the pass on the software
  adapter at midday (the bite by the pass's road gate reads as a notch with its coping and snow broken), each booted with no GPU error. NEVER SEEN ON A GPU, and nobody has played
  a level with them (a bite's top is above a standing eye and the 2 m line a sound is carried over; no run tally was taken).
  ROUND THINGS on the other levels are what their kit says: the crossing 28, the police post 19, the bazaar 6, the compound 3; the levels furnished
  by the older kit (`tools/levels/livedKit.ts`: a drum and a tyre are ONE square box there, and a loose drum, a bucket and a stool are props) have
  NONE, and nothing in a level file says such a box is round. Making them round is a change to that kit and a new export of those levels (the build
  branch's, not done). Of a bite:
  the wall and coping are split at the bite and three to five cut boxes with slanted tops, each a step off its
  neighbour, fill it; the level's snow caps on the coping are cut back to both sides of it; five picture-only lumps lie at the foot. The pieces are `Box.cutOld`
  (512 in the core row's w): on the look page their cut faces take the look's paint as a face of the core's stuff, and the snow lies on the ones
  that look up. A blast's fresh cut never sets it. A bite's middle stops at 2.0 m and its lowest edge at 1.98 m: no standing eye gains a line, and the
  wall still stands over the 2 m line a sound is carried across (`collision.ts`). New boxes are APPENDED (nothing is removed), so `levelFileOf(level).boxes` is shorter than `level.boxes` on that page.
  Known wrong: what pushes against a box meets the UNCUT box, so a man slides round a drum's square, a hand's width out from the round at its four
  corners (the octagon was nearer); the bank of clinging snow steps at the four quarter lines of a round; the authored snow caps on a drum and a
  tyre stack are still the kit's squares; a blast next to a round cannot cut it further (its record is full), and a wall charge on a piece of the bite
  rebuilds it with a square top (`wallcharge.ts` does not read `cut`); the lumps cast no shadow. The trial changes what a ray and a body meet with no
  knob and no journal key: only the level's fingerprint tells a save or a recording made with it from one made without (each is refused on the other).
- **GROUND THAT RAYS CAN SEE, A TRIAL** (`src/scene/groundField.ts`; asked 10-23 after the geometry trial: "do both that and rays on the ground"). On
  THE LOOK PROTOTYPE's page only (`?hills=0`, or the look panel's Ground button, puts it off; the page loads again). The snow hills were PICTURE ONLY: a ray
  met the level's snow boxes under them, never the hill. Here the hills' height, as their mesh draws it (`snowField.ts` `mound`, 10 texels a metre, a texel
  with nothing on it a finger under the ground as the mesh's corners are), rides in `boxMat`'s LAST rows (after the cut table: no new binding, 452 KB),
  with a head row and the highest hill of each 1 m plan cell in front. `common.wgsl ghHit` marches the stretch of a ray that runs between the ground and
  the highest hill, 0.1 m a step and 40 steps at most (a longer stretch in longer steps), one row a step over a cell with no hill, and answers the last
  place the ray stood above the surface (four halvings), so what is shaded or traced on from a hit starts outside the hill. A ray that STARTS under the
  surface meets nothing: the hills are a skin seen from above. It is asked by `occluded` (after the boxes leave the ray clear), `occludedT` and the
  direct light's soft-character path (the hill is the cap a nearer box still beats) and both closest-hit walks (the hit is the ground slab's, its normal
  the hill's slope; `fgather.wgsl giPointOnBox` lets such a sample keep its history). `occludedTBoxes` itself is untouched, so the fog of war's sight
  and the walled cache read are as they were. Every pass that traces light takes it: the direct light's shadows, the gather, the cascades, reflections,
  the volumes, the particles AND THE LIGHT METER (`probe.wgsl`): on that page a man behind a bank is darker to the meter than the headless stand-in
  says. The game hands the rows over where it hands the field to the g-buffer (`game.ts syncSnowLook`), and whole again after a blast digs a hill
  (`stepSnowTracks`).
  Switch: the prelude's `const GROUND_RAYS` from `groundRayConfig.on`, raised by `main.ts` at boot; off = the reads are dead code and the buffer no longer
  (the prelude gains one line of text on every page, the same on each). Not a rule of the game: no knob table, no journal key, no check of its own.
  THE OPEN GROUND ROLLS (`?roll=<m>`, 0 to 0.25, off unless asked; `buildSnowField`'s `roll`): the lying snow under the open sky is baked with a slow
  swell that deep at most, thinner where it is trodden or old (a path reads as a trench through it), none on cleared ground, under a roof or in a wall.
  The mesh then covers the whole yard (88 k quads on the pass at 0.14, 40 k without) and the rays meet it. Nothing in the update reads it: a man's feet
  stand on the level's flat ground, up to the swell's depth under the picture's snow.
  Not in it: the hills' fine relief (the swell, the wind's ridges) and the boot prints, which stay a matter of the shading normal; a hill's own pixels
  start their rays a finger and a half off the mesh and `GH_BIAS` (8 mm) is all that keeps the mesh's triangles and the march's four-texel blend apart.
- **THE LOOK PANEL IS A PAGE OF THE DEBUGGER** (10-31; `ui/lookpanel.ts buildLookPanel(panel, …)`, `LOOK_SYSTEM` = 'Materials prototype', 'Look' on the
  rail; `ui/panel.ts` is not changed). The maintainer: "there's also some flickering where the debug panel would help -- could you make the materials
  prototype menu just another tab in the debug panel?" The look's rows were a fixed box of their own in the debugger's corner and over it, so on the look
  page the debugger (the debug views, the denoiser, the gather) could not be used. Now they are sections of one more system of the debugger, made with its
  own rows: Layers (a slider a row, a dot puts one back, the section's ↺ puts them all back; the frame's cost on top), Compare (hold = plain boxes, a switch
  to stay plain), Hour, sun and moon, Quality, Trials, and Performance and Character lab once the page has made their engines. The debugger's search finds
  a look row by its name or its tip, and backslash shuts and opens the lot. The page still OPENS with the panel up on that system (`panel.show`), as the box
  was always up; so on the look page a key pressed with the pointer over the panel is the debugger's (a letter goes to its search), where the old box let
  it through to the game. A row follows `window.__look.set` now (the panel reads its rows back each refresh); the old sliders did not. SEEN: the look
  sheet's boot on the software adapter (`look-sheet.cjs --eval`, below): the system is on the rail and shown, a slider moves `lookAmounts`, its dot puts it
  back, no page error. Never seen on a GPU or driven by hand.
- **THE LOOK SHEET** (10-31; `node tools/qa/look-sheet.cjs`, its head has the flags). The maintainer, after finding a fifth day case himself: "overall i
  worry that we're finding these cases one by one and not fixing them systematically. is that accurate?" It is: the day picture is the night renderer plus
  a stack of corrections, each tuned on one view. The sheet is one boot of the look page on the software adapter that draws five places on the pass (the
  yard with Sam in it; the yard with Sam indoors; a room with windows with Sam in it; that room with Sam in the yard; a windowless room with one bulb) at
  every hour, as one picture with a table of how bright and how wide each cell is shown. DRAW IT BEFORE AND AFTER ANY CHANGE TO THE DAY'S LIGHT OR
  EXPOSURE AND LOOK AT EVERY CELL before the change is called done. A row can carry look rows of its own (`overcast+lamps:0`). About an hour for the
  whole sheet (a boot of six minutes, then some 25 s a frame); `--first` draws the rows that matter first and the sheet is rewritten after each row.
- **THE LOOK PANEL'S SUN AND MOON ROWS** (`src/ui/lookpanel.ts`; the maintainer, 10-23: "do we model a sun at all for dawn/midday? i think that's a big
  reason why it looks flat. and perhaps we should also model a moon on night levels (and can configure it)"). Both were there: every hour but night
  puts in a sun (`game/hours.ts`, a kind-2 light with traced shadows where the level's moon stood), and every level is authored with a moon. What made
  his picture flat: the pass is a snow level, and the snow's cloud (0.9) leaves the sun 9 % of the hour's own (`cloudLight`); the pass's moon is 0.06,
  where the level kit's own is 0.9, so his night shadows were the gate lamp's. The rows set both for the VISIT: Sun = `__hour.sun(k)` (the share of the
  hour's own sun; it then stands against the cloud, the sky still follows it), Moon / Moon height / Moon bearing = `window.__moon.set({ k, elev,
  bearing })` (`main.ts weatherHourHooks`: the level's steady directional light; `?moon=k,elev,bearing` sets it at boot). A row is greyed while its light
  is not up. THE MOON AND PLAY: the sun's strength is carried by a checkpoint and the journal (the hour's record) and the meter's CPU stand-in reads it
  (`hourLight`); the moon's is NOT carried, and `guards.ts lightEstimateAt` skips every directional light but the hour's sun, so a raised moon moves the
  picture and the GPU's meter and not the headless stand-in or the team's light rules. A moon worth playing under needs that stand-in to count it (the
  sun's one segment test is the model) and a level's own number: a rule of the game, for a switch of its own.
- **THE LOOK PANEL'S CLARITY ROW** (`render/gbuffer.ts setLook`; the maintainer on his GPU, 10-25: "it does feel a bit too "high frequency" for me, like
  edges are cranked up (e.g. "clarity" knob in lightroom)"). One number over the four layers that set a surface against its own neighbourhood: Relief,
  Cavities, Edges and Snow relief are each sent × Clarity (their rows keep their own numbers). Default 0.7; 1 = the layers as their rows say. Plain boxes
  set it to 1 with the snow's relief, and Snow relief at exactly 1 (the game's own snow, what the sweep's row flips to) is sent as 1 whatever Clarity
  says. Nobody has seen 0.7 on a GPU.
- **THE LOOK PANEL'S SKY LIGHT ROW** (`lookConfig.sky`, `engine.ts` where the frame block is written; a MEASURING row, asked for by the maintainer's
  midday note, 10-25: "we need much more ambient light coming from the sky. it shouldn't be so dark underneath the shed here"). The sky's zenith and
  horizon as the picture and its rays have them × the row (0 to 4, default 1). It is the same lever as the debugger's sky brightness, put where the look
  is judged, and it reaches the GPU only: what the game reads off the sky on the CPU (`hours.ts skyLum`, so the headless light meter) does not see it.
  Not a fix and not a default to move: MEASURED with it (the plain page, the pass at midday, snow weather, software adapter): the lorry bay's floor
  holds 3 to 12 % of the open yard's light, which is what one open side lets in (a view factor of 0.03 to 0.05 at five metres in, 0.12 at three), so the
  sky's light arrives about right. The bay read black for three other reasons: one exposure set for sunlit snow (1.7), the tone curve's toe (scene ×
  exposure 0.01 comes out at code 18, 0.05 at 59), and dirt whose paint is very dark (luminance about 0.1). Hence the next row.
- **THE LOOK PANEL'S "EYES ADJUST TO SHADE" ROW** (`lookConfig.shade`, 0.7 on the look page, 0 = off; `engine.ts lookShade` writes `frame.shadeCfg`
  @832, `composite.wgsl lookShadeGain` reads it; the maintainer, 10-25: "it shouldn't be so dark underneath the shed here", then "let's make sure the
  ambient occlusion fix is general and fits nicely with our lighting pipeline. i want it to feel quite realistic!"). No light is added. By day, the
  light a pixel SENDS BACK (its diffuse and its mirror share) is shown up to ×6 brighter where the place it is in holds less light than open air, as an
  eye opens up looking into a shed. THE PLACE is read off the radiance cache (`diceMeanRadiance`, no ray) against what open air holds from the sky
  alone, raised to the row's amount. It is the MOST of five reads (here and 0.6 of a cell to each side in plan), because a probe buried in a container
  or a thick wall holds nothing and would make the ground beside it read as a dark place: a glow round every big thing. The price is on the kinder
  side: within a cell of a bright neighbour (the inside of a shed's outer wall, a doorway) less is brought up, which reads as the shade a wall's foot
  has anyway; a room under two cells wide with a bright yard on both sides is hardly brought up at all. THE PIXEL's own light bounds it: a pixel at or
  over what open ground gets from the sky alone is left as it is (sunlit ground, a sun patch on a shed's floor, the pool under a lamp), and under that
  the gain rolls off as the pixel nears it (place ∕ (1 + (place − 1)·Epix ∕ w)), so brighter stays brighter and nothing lands on one flat level. A
  lamp's own face, a section cap, snow sparkle, beams, fire, the air, the weather and the lit GPU particles (casings, chips) are NOT scaled: a chip on a
  shed's floor reads darker than the floor by the gain. It fades out as the sky's mean radiance falls from 0.35 to 0.08: dusk gets a little, night gets
  none (the dark is the game's). "Hold: plain boxes" and the sweep's `look.all` row put it at 0. The two sky constants in `lookShade` (0.8 / 0.2 and
  0.947 / 0.053) are the hemisphere integrals of `common.wgsl skyRadiance`'s ramp: retune one, redo the other.
  **THE PICTURE ONLY**: guards, teammates and the light meter read the light as it is, so by day a shed LOOKS brighter than it hides you. That is a
  break of "what the AI senses is what the picture shows" by the amount of the gain, ON by default on the prototype page because the maintainer asked
  to see it, and to be settled before any of it goes to the build (the honest version is one exposure for the picture, chosen by where the camera
  looks, which the game can read too). Seen on the software adapter only (the bay at midday); never seen on a GPU, never played.
- **THE LOOK PAGE OPENS A SNOW LEVEL UNDER BROKEN CLOUD, AND ITS PANEL HAS A CLOUD ROW** (`ui/lookpanel.ts LOOK_SNOW_COVER` 0.5, `main.ts`
  `__hour.cover(c)`; the maintainer, 10-25, of the pass at midday on a GPU: "i'd still expect a stronger directional light from the sun in midday").
  What he saw was the game's own rule, not a fault of the hour: the snow weather's cloud is 0.9 (`weatherKinds.ts`), and under that `hours.ts`
  leaves 0.09 of the hour's sun (0.54 of midday's 6), spreads its disc to 0.23 rad and greys the sky: an overcast, with no shadows. (So on his page a
  lantern at a metre was eight times the sun, not equal to it.) `__hour.cover(c)` writes the picked weather's cover for the VISIT and hands the sun
  back to it, so the sun, its disc and the sky follow by the same rule (Kasten and Czeplak's fits): at 0.5 midday's sun is 0.70 of its own (4.2), the
  disc the hour's small one (hard shadows) and the sky 1.66 of its own. It moves the real light: the guards and both meters read it. The night is not
  touched by cloud. NOT carried by a save or a recording (a take made on the look page re-runs elsewhere under the game's 0.9). Whether the GAME's
  snow should fall from broken cloud by day is the weather's owner's and the maintainer's call; nothing outside the look page has moved. Checked on
  the software adapter: the page's boot check reads the sun at midday under 0.5 and 0.9, and one still each of the gate at midday
  (`rebuild/graphics-next/gate-midday-cloud.png`): under 0.9 open snow code 146 / 161 / 184 and the gate's shadow a vague 109 / 127 / 159; under 0.5
  snow 205 / 210 / 219 (not clipped) and a hard-edged shadow 101 / 122 / 159. Never seen on a GPU.
- **THE LOOK PANEL'S "EYES FOLLOW SAM" ROW** (ITS EYE BY SAM'S METER, `render/lookeye.ts` AND `lookeye-check` ARE GONE SINCE 11-04, see THE ROWS THE ONE EXPOSURE MODEL REPLACED ARE DELETED below: the row now says how far the model's own eye goes to his place; what follows is how it was. 10-28; `lookConfig.eyes`, 1 on the look page, 0 = the shade row alone as it was; `render/lookeye.ts
  lookEyeFrame` writes `frame.eyeCfg` @880 (`FRAME_BYTES` 896), `composite.wgsl lookShadeGain` reads it; the maintainer, on a GPU: "overall i think light
  levels looks really good. i wonder if we need to adjust our tone mapping based on where the characters eyes are though. like in this setting on overcast
  the room should probably be a bit brighter (and then it's okay for outside to be a bit overexposed?)", and two minutes later, in a room with every door
  shut: "it doesn't really _look_ like all the light is coming from the window? feels flat"). The shade row brings every place up by ITS OWN darkness,
  wherever Sam stands, which evens out the very fall-off that says where a room's light comes in. THE EYE: the light on Sam as his meter has it
  (`lookConfig.eye`, the page's word: `game.player.irradiance`) against what open ground gets from the sky alone (`shadeCfg.w`), eased over about a second
  into the dark and a third of that back out (`LOOK_EYE.open` / `.close`; the first frame takes the reading outright: `Engine.lookEye.snap`). With him in
  a place darker than the open: every place NO DARKER than his is brought up as his is, by one gain (so the light still falls off from the window, and
  his place is `more` 0.4 of the way further toward the open air's brightness than the shade row took it: "a bit brighter"); a place darker still gets
  only the share 1 − `drop` × … of its own extra; a pixel as bright as open ground, which the shade row left alone, is shown his gain to the
  `out` 0.6 × the "Glare outside" row (`lookConfig.glare`, 0.1) times brighter (the yard through the door, about a tenth over); and the most a place
  is brought up rises from 6 to 9. BOTH ROWS ARE FOR PLACES UNDER A ROOF (10-29; the maintainer on a GPU, Version 24: "indoors look better, but
  outdoors looks very flat when eyes are indoors. seems like the "HDR" effect isn't quite set up right?", and of midday "a little flat"). Sam's
  gain went on every pixel that was not in full light, the open yard's included: a shaded wall there and a man's dark side came up about twice
  while the snow beside them, near the top of the curve, could only go to white (it was shown his gain to the 0.3, up to × 1.9). And the shade row
  itself read a wall's shadow on the snow as a dark place (beside a wall the air holds about two thirds of the open air's light) and brought it up
  by a third. Now `roof` = how far a place is under one, by what it holds against the open air (none over 0.8 of it, all under 0.4; a doorway and
  a porch in between): the shade row's own gain and Sam's both go by it, so out in the open nothing is brought up for either. With him in the open the
  picture is the shade row's exactly. By day only, as the shade row is. THE PICTURE ONLY, like the shade row: the same break of "what the AI senses is
  what the picture shows", by the amount of the gain. Check: `tools/qa/lookeye-check.ts` (12 lines, the shader's gain mirrored on the CPU;
  `LOOKEYE_BREAK=1` fails 9). The meter's 0 before its first answer is no reading (the eye would open wide on a level's first frames), and a restart
  takes the first reading outright again (`game/airSim.ts`). SEEN on the software adapter only (the pass under overcast, 320 × 200): with the row at 0
  the tea room and the kitchen are brought up in the middle of the floor and stay dark along their walls; with it up both rooms read evenly and clearly
  and the snow outside the wall goes to white. NOT SETTLED: what the eye should do at night; a camera that looks at a place Sam is not in.
- **THE LOOK PANEL'S GRADING ROWS** (10-29; Exposure, Contrast, Highlights, Shadows, Colour: `lookAmounts.gexp / contrast / highs / lows / colour`,
  each 1 at rest but Contrast; `gbuffer.ts lookSync` → `lookConfig.grade*` → `engine.ts` → `passes.grade*` → `post.wgsl agx`; the maintainer on a
  GPU, of midday: "image looks a little flat in midday, i think it looked better before. maybe needs some tuning for exposure/contrast/etc.? that's
  what i'd reach for in lightroom"). MEASURED on stills of the pass's road at midday (Version 24, the powder on and off: the two are the same to a
  code value, so the powder was not it): sunlit snow 215, a wall's shadow on it 140, and the darkest of Sam's kit in the sun 110. The tone curve
  spreads sixteen and a half stops over the screen, which a night needs; a day on snow lives in the top four, so with the snow near white a black
  rifle in full sun, four stops under it, lands on middle grey and the whole picture is pale. CONTRAST (`render/lookgrade.ts lookGrade`): BY DAY ONLY
  (it fades out with the sky's own light over the shade row's span; a night is the game's), the picture's stops are turned, before the curve, about
  the stop SUNLIT GROUND sits on at that hour (level ground of albedo 0.85 under that sky and sun at that exposure: 0.841 of the range at midday on
  the pass), so the snow keeps the place the hour's exposure gave it and whatever is darker goes down: at 1.4 the rifle is 5.6 stops under the
  snow where it was 4, a shadow on the snow 1.8 where it was 1.3. EXPOSURE is × the hour's own (0‥2 = two stops under to two over; it goes on
  `frame.exposure`, so every reader of that follows). HIGHLIGHTS and SHADOWS add up to a stop and a half to, or take it from, what is over a stop
  above middle grey, or under two below it. COLOUR is × the look's saturation. With every row at rest the tone map does not run the line (the
  build's picture to the bit); `PostParams` grew four floats (the buffer is 64 bytes). THE DAY'S CONTRAST IS THE OPEN AIR'S: seen on a still
  of the kitchen under overcast at 1.4, the turn pushed the room the eye rows had just brought up back down (a third of its distance under the
  snow). So the composite gives a place under a roof back what the turn takes from it: `frame.gradeCfg` @896 (`FRAME_BYTES` 912;
  `lookgrade.ts lookGradeFrame`: 1 ∕ the contrast, the radiance the turn is about) and `lookShadeGain`'s own `roof` (it now returns it), on the
  pixel's reflected light under the turn's radiance, eight times at most: a room is shown as it was with the row at 1, a porch in between. A
  pixel in full light is left (it sits at the turn already), and so is a lamp's own face. THE DEFAULT IS 1.35, picked on stills of the pass's
  road at midday at 1, 1.3 and 1.6 (640 × 400, the software adapter): sunlit snow 212 / 208 / 203, a wall's shadow on it 140 / 110 / 82, the
  darkest of Sam's kit 109 / 78 / 46; at 1.6 the shadows are cobalt (the turn is per channel, so it adds colour as a photo editor's contrast
  does; Colour takes it back). Check: `tools/qa/lookgrade-check.ts` (7 lines; `LOOKGRADE_BREAK=1` turns about middle grey and fails 4).
  Never seen on a GPU at the time of writing.
- **A ROOM OF THE PASS AT MIDDAY, MEASURED (10-30; the rows it measured are gone since 11-04; its points (1) and (3) were WRONG and (2) is not the whole of it, see the next bullet).** The maintainer on a GPU, the look page, the tea room: "lighting still looks
  very flat indoors during midday (see flat shading on chair). also very odd pooling of shadow under window". Read on the software adapter, the same room.
  HOW: debug views 3 (direct), 4 (the gather), 7 (the cache's irradiance) and 8 (both) are LINEAR on screen in a room's range (the Window light row at 0.5
  halves view 3's pixels), so their pixels can be set against each other. Move the squad out first (a man at a window shadows half the room), and a pin's
  pitch is held to about 60 degrees. WHAT IS THERE: the midday sun (56 degrees up) reaches the floor only as one sharp patch 0.4 by 1 m; all the other
  direct light is the window lamps', 45 near a window, 22 a stride along the wall, 4 at the far wall. The gather's bounce is EVEN, 3 to 7 everywhere, and
  a flux sum for that room (two lamps, dark floor, stone walls) gives about the same, so the amounts are right. The cache holds 40 to 90 in the room (sky
  it leaks in); the picture does not draw it, the shade row and the gather's ray bound read it. RULED OUT, each by a still: the shade row's roll-off (a
  curve linear up to open ground's light drew the same picture and was dropped), the Eyes follow Sam row, the gather's ray bound (`giFireflyK` 0 drew the
  same), the Window light row (at 0 the room is darker and as flat). SO: (1) THE POOL is the lamp's dead ground. A window lamp is a Lambert disk on the
  wall: it lights nothing beside or under itself, so the wall round a window and the floor at its foot have only the even bounce, a fifth of the lit
  floor beside them, and the grade takes that to black. (2) THE CHAIR: from above its seat and the faces toward the window are lit alike by one wide soft
  lamp, its shadow is soft, and the far lamps fill it. (3) The window's fall-off, 4.7 times in light from the near floor to the far, lands as about 2.4 on
  screen; the sun's patch is drawn hardly brighter than the floor round it (the shade row leaves it and brings the room up). NOT BUILT, put to him: a
  contrast of the room's own (the day's contrast is undone under a roof), the sun's patch left bright, and a window's and an open door's light as an
  opening that sends sky down and the lit ground up. (11-01: a window, an open door and a blown hole ARE openings now, by one rule, each one light of the
  mean of the two: THE LOOK PANEL'S "DAYLIGHT AT OPENINGS" ROW. The two-tone light is still not built.)
- **A ROOM'S LIGHT IS READ ON THIS SIDE OF ITS WALLS, AND A SUN PATCH INDOORS KEEPS THE ROOM'S CONTRAST** (BOTH HALVES ARE GONE SINCE 11-04: the sun patch half with the rows the model replaced, and the read on this side of the walls, `lookPlaceHere`, `?footing` and `lookplace-check`, with the guess it served: A PIXEL'S PLACE IS READ OFF THE FLOOR PLAN, below. What follows is the record. 10-30; `lookConfig.footing`, 1 on the look
  page, `?footing=0` = as it was; `frame.gradeCfg.z`, written by `engine.ts` on the look page whatever the Contrast row says; `composite.wgsl
  lookPlaceHere` and the head of `lookShadeGain`, and the compact twin). The maintainer on a GPU: "very odd pooling of shadow under window". THE BULLET
  ABOVE HAD THE CAUSE WRONG. The window lamp's dead ground only says why no direct light falls there. The black was the SHADE ROW not bringing that floor
  up: `lookShadeGain` took the place's light as the most of five cache reads, 0.6 of a cell to each side, and the cache blends a point with the four
  probes about it, so within a stride of any OUTER wall the open air beyond it was in the read, the floor was taken for a place in the day's light and
  was shown at the yard's exposure and contrast: a black band along the foot of every outer wall, most plain beside a window where the lit floor is next
  to it. (The old comment called it "the shade a wall's foot has anyway".) NOW the read is the blend of the four probes the point can SEE: a box ray to
  each one's column at the point's height (the pixel's own man's gun is skipped, men's bodies are not asked), a probe beyond a wall or inside a crate
  is left out and the others share its part; where the four hold about the same (the most under 1.3 times the least) no ray is asked and the read is the
  plain one. The four side reads are gone with the row on (they were for a probe buried in a container, and that one is not seen). ONE box walk in the
  text, ONE call site: the composite's rows in `shaderinline-check` went up by its 15 KB (the look's text is in the build's kernel, dead under `LOOK`).
  SECOND, THE SUN PATCH: a pixel in full light was returned as the open air with no read, so the day's contrast was given back to the patch's soft edge
  and not to the patch (a bright rim round a middle hardly brighter than the floor). Such a pixel is now asked where it is too, only when the contrast
  row would touch it (darker than sunlit ground), and it is under a roof by a stricter span (the place holds under a quarter of the open air's light:
  yes; over a half: no), so a sunlit wall's foot out of doors is the open air as it was. KNOWN AND LEFT: in a doorway or a porch (a place between a
  quarter and 0.8 of the open air's light) the step at a shadow's edge is still there; the place's light steps where a probe comes into sight (a door's
  jamb), by what that probe weighs there. Also wrong above: "4.7 times lands as 2.4" compared light with ENCODED pixels; in shown light the fall-off is
  kept. THE CHECKS: `lookplace-check` mirrors the read over a cache of two values (this side 1, the far side 20) at 268,576 floor points within 0.9 m of
  a ceiling-high wall in 57 levels: the old reads are over twice this side's light at 93.3 % of them (11 times on the mean), the new at 0.9 % (1.18
  times), 2.2 box rays a point; `LOOKPLACE_BREAK=1` fails it. `lookgrade-check` is as it was (the caller's line is the same). COST: up to four box rays
  of a cell's length for a pixel by a wall or a doorway, none in the middle of a room or in the open; five cache reads for a sunlit pixel darker than
  sunlit ground: NEVER TIMED ON A GPU, never seen on one. NOT MENDED BY THIS: the chair (the next bullet). What stood here about it was
  WRONG: "2.2 times across the seat is shown as 1.36, a white thing sits high on the tone curve, a matter of the grade". Compared light with encoded
  pixels again, and the Highlights row at 0.3 and 0 gave the chair nothing back. And a window is still a lamp on the wall and not an opening that sends
  the sky down and the lit ground up.
  A point that sees none of its four probes (a face in a crack of the lorry) takes the brightest of them, so it is never a dark place. SEEN, the
  software adapter, one page, the tea room from above, the row on against off: the middle of the room is the same picture (mean pixel difference
  under 2 of 255); the floor under the window is 2.3 times brighter in shown light, the foot of the window wall 28 times and the wall behind the
  table 51 times (they were black); the sun patch's middle is 1.8 times brighter and stands 2.1 times over the floor beside it where it stood 1.2
  times. The yard from above, on against off: one pixel in a hundred differs by over 8 of 255, nearly all of them
  brighter: slivers in the lorry's recesses (between cab and bed, under the cab's corners), which are places the old reads took for the open air.
  The final text (a blind point takes the brightest) boots 119 of 119 and drew the yard again; its room stills are the text before that last line.
- **A ROOM HAS THE DAY'S CONTRAST TOO, ABOUT ITS OWN LEVEL** (GONE SINCE 11-04: the Room contrast row, `lookConfig.roomContrast`, `frame.gradeCfg.w`'s use and `lookroom-check` went with the rows the one exposure model replaced (below); what follows is how it was. 10-30; the look panel's "Room contrast" row `roomc`, 0.6 on the look page (the maintainer's number, 10-31, from his GPU: "indoor looks a lot better, i set room contrast to 0.6"; it shipped at 1), 0 = as it
  was, `?look=roomc:0`; `lookConfig.roomContrast` → `frame.gradeCfg.w`; `composite.wgsl` under `lookShadeGain`'s call, and that function's z and w. Off
  too with `?footing=0` and with the Contrast row at 1. PICTURE ONLY, the look page only, so no stand and no knob table). The maintainer on a GPU, the
  tea room of the pass at midday: "lighting still looks very flat indoors during midday (see flat shading on chair)". THE CAUSE, found the long way:
  the Contrast row (1.35, `render/lookgrade.ts`) turns the day's stops about sunlit ground, and the composite takes the WHOLE turn back under a roof so
  that a room is not pushed down. So the yard has the row and a room has none of it: a room is drawn with its light as it is, beside a yard drawn a
  third steeper. WHAT IT WAS NOT, each by stills of the white chair (seat against the face of its back, shown light): not the tone curve (above); not
  the shade row's roll-off (a trial that raised the roll-off's top under a roof took the seat from 2.92 times the back to 3.36 at its most and doubled
  the sun patch: the roll-off squeezes the chair by about 1.3, and the trial was dropped); and "the seat takes 14 times its back's light", read off
  the direct and bounce debug views, was WRONG (those two views are not on one scale with the picture; by how the gain answered the trial the seat
  holds about 3 times the back's light, which is about what the picture showed). THE RULE: a pixel under a roof is turned by the same contrast about
  the level its place is shown at: × (l ∕ level)^((1 − x)·row), l = what the surface sends back after the shade gain, no further than three stops
  under the level and two over it. The level is `ROOM_LEVEL` (0.5, set by eye from three stills of this one room) × what a surface sends back, after
  the gain, in the light of the air its place is exposed for: Sam's room's for every place no darker than his (one level across his room, so the
  fall-off from the window is turned about it too), a place's own with him out of doors. So the level is NOT the same for one room with Sam in it and
  out of it (about a stop apart on paper, eased with his eye): never looked at. Under a roof by one span for a pixel in light and in shade (nothing
  where the place holds half the open air's light or more, whole from a third down), so the row draws no rim at a shadow's edge. A PIXEL IN FULL
  LIGHT IS TURNED TOO: a sun patch comes up by up to two fifths, so a sun stripe through a door is drawn brighter inside than on the ground it came in
  over. THE CHECK: `lookroom-check` mirrors the lines as numbers with the tone map's turn as a power (two faces 3 times apart are shown 3.00 apart at
  0 and 4.41 at 1, the yard's own; a surface at the level stays put; never under 0.58 or over 1.43 of what it was; no step at sunlit ground's
  brightness; one weight in light and shade; one level across Sam's room); `LOOKROOM_BREAK=1` fails it. It does not mirror the tone curve's shoulder
  or lookPlaceHere. COST: with the row up EVERY pixel in full light takes lookPlaceHere's read (five cache reads, and up to four box rays by a wall),
  the sunlit yard too, where none was made: NEVER TIMED ON A GPU. SEEN, the software adapter only, one page, the tea room at midday, the row at 1
  against 0 on one boot: the chair's seat stands 3.80 times over the face of its back where it stood 3.15; the chair's shadow on the floor is 0.64 of what it was, the floor beside it 0.78, the stone wall 0.81, the dark carpet 0.44, the sun patch 1.09 times; the still's mean pixel goes from 77.7 to 71.2. The room from above: the fall-off from the window, the shadows under the tables and the sun patch read where the picture was even, and it is DARKER, mean pixel 56.8 to 48.1 (most of this room sits under the level). The yard from above: mean pixel 205.7 both ways, 0.7 % of pixels differ by over 8 of 255. The page booted 119 of 119 with no page error. SEEN ON HIS GPU 10-31 ("indoor looks a lot better", the row at 0.6); never timed there, and the stills above are the row at 1.
- **ONE EXPOSURE: ONE GAIN A PLACE, AND THE ROWS IT REPLACES** (SINCE 11-04 THE ROWS IT REPLACED ARE DELETED and `expo` at 0 is no gain by the place at all, see the bullet after ONE EXPOSURE AT EVERY HOUR. 10-31; the look panel's "One exposure" row `expo`, 1 on the look page, 0 = the rows as
  they were, `?look=expo:0`; `lookConfig.expo`; `render/lookexpo.ts lookExpoFrame` → `frame.eyeCfg` with w under 0, which is `composite.wgsl
  lookExpoGain`'s word; the place field `render/placeField.ts` + `shaders/place_field.wgsl`. PICTURE ONLY, the look page only, BY DAY ONLY (it rides
  on the shade row), so no stand and no knob table: the guards and the light meter read the light as it is). The maintainer, 10-31, after a fifth day
  case found on his GPU (a store with one bulb and no window, black on a grey day while the bulb glowed): "overall i worry that we're finding these
  cases one by one and not fixing them systematically", "compare to night and dusk", and on the card he chose "Sheet, then one exposure model". WHAT
  WAS THERE: Eyes adjust to shade (a gain by the place, capped at 6), Eyes follow Sam (four numbers, a cap of 9), a roll-off by the pixel's own light,
  the day's contrast taken back out under a roof, Room contrast putting a share back about a room's level; each tuned on one room. THE MODEL, said
  once: every PLACE gets one gain, from how many times darker than the open air it is (rp) and how many times darker the place Sam stands in is (rs).
  His place: × rs (shown as the open air is with him out in it). A darker place: his gain × the shade row's share of its own extra dark, (rp ∕ rs)^0.7.
  A brighter place: exposed for itself and shown over by at most rs^(0.6 × the Glare outside row). NO CAP ON THE PLACE AND NOTHING BY THE PIXEL short of the shoulder below: a
  shadow's edge and a man's own shading keep the light's proportions. A place holding 0.8 of the open air's
  light or more IS the open air (`EXPO_BAND` 1.25, `LOOK_EXPO.band`), and a darker one is counted from there, so nothing steps and a darker place is
  never shown brighter than a lighter one. THE DAY'S CONTRAST is the yard's, in the place's own exposure: the place is toned as the open air is and
  then shown as much darker as it is to be, which is one more gain for the whole place, (rp ∕ base)^(1 − 1∕contrast). The old rows' AMOUNTS still say
  how much (shade = the share, eyes = how far the eye goes to his place, glare, contrast); Room contrast does nothing with the row up.
  THE OPEN AIR IS WHAT THE CELLS UNDER THE OPEN SKY HOLD ON THIS MAP, not what open ground would hold in theory (the pass's walled yard holds 0.64 of
  the theory's light and the snow past its wall 1.6 times it; counted from the theory the yard came out a fifth too bright). The map marks each free
  cell that has no roof over it (`placeRoof`: a slab over head height, 0.9 m wide both ways, `weather.ts roofsOf`'s own test) and the field's third
  word is the level a tenth of those cells are darker than (`PLACE.skyShare`, eased over half a second so the picture is never handed a step; a level
  with no cell under the sky takes its brightest place). So every place under the sky is × 1 and a room is counted from the darkest open air there is.
  A place is followed 14 stops down and no further (`EXPO_MOST`). THE SHOULDER
  (`lookRoll`): what a place's gain hands on brighter than sunlit ground is rolled off toward `EXPO_TOP` 4 times it, in full from a gain of 1.5: a sun
  patch through the door of a room shown hundreds of times brighter, the floor under its bulb, the yard mirrored in its floor (reflections and
  highlights take the same roll). A glowing face and a lamp's own haze and beams take NO gain: a bulb's own glow is as bright as it was. (Since 11-02 the
  shoulder is one part of the one rule for a place seen from another, and smoke's light takes the place's gain too: the bullet "A PLACE SEEN FROM ANOTHER".)
  A PLACE IS WHAT WALLS, WINDOWS AND SHUT DOORS ENCLOSE: the place field, 100 × 70 cells of 0.4 m over the cache's floor plan, one number a cell: log2
  of the light the air of the place holds. THE MAP is drawn from the level's boxes and again when one changes (`placeMask`): a cell is WALL when boxes
  fill it from 0.45 m to 1.9 m, one box or several (`placeWall` gives a box's height slices as bits: a sill, its pane and the lintel over it make a
  wall between them, so A WINDOW IS A WALL, whole or shot out, and the light through it is the room's own reading; a crate, a post under 0.6 m long, a
  tilted box and a box a blast has hidden are not; a dynamic box shaped like a door's leaf counts where it hangs). A DOORWAY is a free run of 4 cells
  or fewer whose way through opens out within 3 cells on both sides: its cells are marked, the pull across them is a hundredth, and two rooms through
  an open door stay two places with a soft step at the threshold. A corridor is as narrow but stays narrow: one place along its length. An opening
  over 1.6 m joins. Each free cell is pulled to what the radiance cache's up-facing slab holds over it (the light that falls on a floor there, not the mean light in the air) (only the probes it can see along the map; a probe
  holding nothing counts a fiftieth) and, hard, to its free neighbours (red-black over-relaxation in place, 4 sweeps a frame, 160 when told to settle
  outright: a first frame, a restart, a still). FOUR WORDS FOLLOW THE CELLS: Sam's eye (the field under his feet, eased on the GPU, 0.9 s to open,
  0.35 s to close: there is no second eye by his meter), 1 once the field is live, the open air's LOW level (a tenth of the cells under the sky are darker:
  what a room is counted from) and its MIDDLE level (what the key is held to, so a sunlit yard beside a shaded one is not shown at the shade's exposure).
  NEVER BRIGHTER THAN THE NIGHT: no place's gain passes the night's exposure over the hour's (`eyeCfg.x`, `photometry.ts hourExposure`: 1,152 on the grey
  day, 5,051 at midday), so a room lit by its lamps alone is shown as the night shows it and a room with no light is dark, not its noise. THE COMPOSITE READS THE FIELD AS A
  UNIFORM BUFFER (`@group(1) @binding(17)`, 28 KB): its fragment stage has no sampled texture left. `lookPlaceField` never weighs in a cell across a
  wall; in a wall's own cell with a different place each side (a wall's top, the floor at its foot) the side is picked by what the cache holds at the
  pixel (`lookPlaceHere`, the kernel's one box walk, also the answer before the field's two programs are compiled and off the map). The composite's
  budget row went 1024 → 1032 KB on purpose. NOT DONE HERE: `placeMask`'s stamp is a second writing of `game/airSim.ts`'s; a level whose walls are
  under 1.9 m has no places.
  THE CHECKS: `bun run tools/qa/lookexpo-check.ts` mirrors the gain as numbers on the kernel's own constants (the open air with Sam in it
  is × 1 exactly; his room 700 times under the open has its pale floor shown 1.44 times as bright as the yard's with him out in it; order kept and no step over every pair of places, rows, day and dusk,
  open country and walled yard; the shoulder; a room toned as the yard; the two kernels word for word; windows, doorways, corridors and roofs on the
  map: 28 lines),
  `LOOKEXPO_BREAK=1` (the old cap of 6) fails 8; `node tools/qa/place-field-page.cjs` runs the field ALONE on the software adapter over a stand-in
  cache (a room whose light spans 2.6 stops has one level to 0.08 of a stop, the room behind its wall keeps its own, an open door moves it 0.06 of a
  stop, a wide opening joins, a window leaves no hole, at rest nothing creeps, a change of light is followed inside half a second, the eye closes
  faster than it opens: 15 of 15), `PLACEFIELD_BREAK=1` (no walls) fails 11. THE SHEET draws old and new on one boot and prints the field's own
  levels a cell: `node tools/qa/look-sheet.cjs --rows overcast,overcast+expo:0,…` (`expo:0` drew the rows before the model beside it until 11-04, when they and the row went; `overcast+shade:0` now draws the hour's one exposure with no place brought up). COST: 9 small compute dispatches a frame and a 28 KB uniform; two
  more pipelines, compiled after the page is up, the old rows run until they are: NEVER TIMED ON A GPU. SEEN: software adapter only. One boot of the code as pushed, midday and night
  (picture means; lamp room · yard · room with windows): midday 0.142 · 0.594 · 0.097, night 0.224 · 0.140 · 0.240. On earlier boots of this row: the
  lamp room on the grey day 0.118 and at dusk 0.037. WHAT THE STILLS SAY THAT NOBODY HAS RULED ON: a room with windows at midday is DARK with Sam in it
  (median 0.06, its windows and their patches bright); the lamp room at dusk is about a fifth as bright as at night (Sam's place gain there is 13.6 and
  the night bound 68.87 is not reached). NOT DRAWN on the code as pushed: dawn, dusk, the grey day, the two looking-in and looking-out views, and no
  before row (`expo:0`, gone since 11-04 with the rows it drew) beside any cell. KNOWN AND NOT MENDED (a review of the diff): a wall cell takes its brighter neighbour's level inside half a stop;
  reflections are multiplied by the place gain and then rolled again, and the wet coat's highlight is not rolled; the roll's knee when the Contrast row
  gives no turn; the night bound ignores the Real light values amount; the eye stays as it was when the row goes back to 0, and the field's buffers are
  never released; the cascade overlap is along x only; `placeField.maskKey` walks every box a frame. NEVER SEEN ON A GPU, AND NEVER TIMED ON ONE.
- **THE BOUNCE HOLDS STILL BY DAY** (11-01; the look panel's row `steady`, "Steady bounce", 1 on the look page, `?look=steady:0` = both halves below as they
  were; it sits under the `rooms` row and does nothing with that row down, at night, or off the look page. The maintainer on his GPU, the pass at midday
  with doors open, two recordings: "lots of temporal instability for midday, also strange shimmering on metallic barrels", "lots of temporal instability on
  the guy's back too", "overall night, dawn, and dusk look pretty good. midday and overcast are the broken ones"). ONE CAUSE, TWO PLACES IT SHOWS: a pass made
  for nights judges light against a night-sized window or a night-sized number, and by day a room is lit through a door or a window by something small and
  thousands of times brighter than the room. (1) THE CACHE'S MEAN (`render/diceRing.ts`, `shaders/dice_ring.wgsl`, `engine.ts`). The radiance cache shifts its
  directions every frame (`frame.rcJitter`) and its dice was a running mean of about three frames (`rc.setDiceBlend(0.3)`). MEASURED on the software adapter,
  the store with its door open at midday, Sam still: the light meter at Sam (the dice there) went 0.00094 to 0.0014 from frame to frame, a fifth either way,
  and every reader of the dice went with it (the gather's second bounce and its outlier bound, a wet coat's sheen, the smoke, the meter, the One exposure
  row's level of the place: the whole store's picture pulsed together). NOW the shift is one of a few fixed points taken in turn and the dice is the plain mean
  of the cache's last bake at each: the cache bakes whole into the dice (its own mean off), `DiceRing.step` copies that bake into its slot of a texture eight
  dice tall and one small program writes the mean of the slots held back into the dice, which everything binds as before. In a still scene each frame
  replaces one bake with the same bake, so nothing changes; a real change of light is all in after one round. Points by `ringPoints(lamp share, row)`: none
  at night, four at dawn and dusk, eight on a grey day and at midday (four or eight only: a round must be a whole number of the cache's own refresh rounds,
  `settings.rcPeriods`). A re-primed cache drops what was held (`rc.reprimed`) and the ring fills as a plain mean. One more compute pipeline, made at the first
  day frame with the row up and tried under an error scope; until it is ready, or on a cache too tall for the texture, the cache's own mean as it was.
  (2) THE REFLECTION PASS'S BOUNDS (`lighting_common.wgsl dayBoundK` + `dayBound`, set in the three reflection programs: `reflect.wgsl`, `reflect_p5.wgsl` and
  its compact twin). A ray may bring back 60 and the analytic highlights 160, numbers that at night are some hundreds of times a lit wall. In the store at
  midday a wall sends back about 0.00016, so the bounds held nothing and a ray off a steel drum that left by the door was a white streak, somewhere else the
  next frame. By day both are now x a quarter of the light the cache says the point receives (E / pi, taken as no less than the hour's lamp share, never over
  1): about thirty times a mid-grey wall beside it for a ray. `frame.lampCfg.z` carries the row. LEFT AS IT WAS: the gather's own gloss ray and glints
  (`GLOSS_RAY_MAX`, `GLOSS_GLINT_MAX`, read only on tiers without ray-traced reflections), the gather's 24 beside its relative bound, the denoiser. THE CHECK:
  `bun run tools/qa/steadybounce-check.ts` (the points by hour, the slots, what is held as a ring fills and starts over, and a model probe in a still scene:
  the ring's mean is one number every frame where the running mean swings 8 times over; `STEADYBOUNCE_BREAK=1` leaves the row at nothing and fails). The
  sheet tool measures it: `node tools/qa/look-sheet.cjs --rows midday+steady:0,midday --views lamp-open,drums --settle 4 --flicker 10` (the door-open views;
  `--flicker N` prints each frame's picture mean and light meter and saves a map of what changed). SEEN (software adapter, 640 x 400, Sam and the scene
  still, the store's door open at midday): the light meter over ten frames with the row up 0.00117, 0.00116, 0.00116, 0.00114, 0.00113, 0.00112, 0.00112,
  0.00112, 0.00112, where with it down it was 0.00103, 0.0011, 0.0014, 0.00119, 0.00094, 0.00113, 0.00096, 0.00124, 0.00099, 0.00095; the picture's 8 px
  blocks changed by a middle 0.33 display levels where they changed by 1.13, and the store's floor is no longer lit up on the map of change. What still
  changes there is Sam's rifle and a few thin edges, 15 levels at most, the same with the row down: a thing a pixel or two wide at this size.
  The same on view `drums` (the store's steel drums against the lit wall, Sam beside them): with the row down the WHOLE picture pulsed, its mean 87.6 to
  93.2 of 255 over ten frames, blocks' middle change 1.83 levels, 7.7 % of blocks over 3 levels, drum rims and crate tops lit up on the map of change; with
  the row up the mean holds 90.8 to 91.2, the middle is 0.33, 2.2 % over 3 levels, the largest single change 23.5 against 26.3 (Sam's rifle and the drums'
  rims at the bright wall). Both boots, and one more of the tree as committed, compiled 119 of 119 programs with no errors, the ring filling
  a point a frame.
  NOT SEEN: a man in motion (the flicker frames stand still), the overcast hour, the stagger and half-rate cache options with the ring (stagger gives the
  ring two bakes a point, which can leave a two-frame ripple an eighth the size), frame time (one copy of the dice and one pass over it a frame, untimed),
  any level but the pass. The light meter and the smoke read the dice, so on the look page the row also steadies what guards see by. An old red on this
  branch, not from this change: `shaderinline-check`'s budget line (fgather, volumetrics, reflectP5 and particles over their rows since the look rows of
  10-31). NEVER SEEN ON A GPU.
- **THE PLACE MAP IS TOLD THE LEVEL'S DOORS** (11-01; `scene/placePlan.ts placeMask` `gaps` + `PLACE.gapNear`, `PLACE.minWide` 0.3 to 0.2,
  `lookConfig.doorGaps` set in `main.ts` from `game.doors.list`, `engine.ts` hands it to `PlaceField.setBoxes`. The maintainer on his GPU, of the pass at
  midday on the One exposure row: "it seems easy to get into a state where the exposure is off, like moving between these two rooms", with a shot of Sam in
  the kitchen, the kitchen nearly black, his light meter at 0.121). THE FAULT, found on the CPU's map and not on a GPU: a doorway was whatever the SHAPES
  said (a free run of four cells or fewer between walls that opens out on both sides), and the shapes change with how a leaf hangs. With the kitchen's yard
  door wide open its opening and half the window beside it read as one run five cells wide (the 0.5 m pier between them was "a post" at `minWide` 0.3, and
  one pane's half-width is a hair under 0.3), so the kitchen was ONE PLACE WITH THE OPEN YARD: its level was the yard's, the eye stood in the open, and the
  room was shown hardly brought up at all. The bunk room's leaf at about 65 degrees did the same. THE MEND: each door is a line on the floor plan (hinge to
  far jamb) and the free cells within `gapNear` of it are doorway cells outright; the shapes still find an opening no door hangs in. And a wall piece is
  0.4 m long or more (`minWide` 0.2). THE CHECK: `bun run tools/qa/placedoors-check.ts` (the pass, headless: each of the seven doors through its whole
  travel every 0.05 rad, 539 positions, and all open at once: no room reaches another room or the open sky but through a doorway cell;
  `PLACEDOORS_BREAK=1`, no doors told and the old width, joins 25 and fails). `lookexpo-check` is as it was (28 lines). SEEN (software adapter, one still,
  midday, the kitchen's three doors standing open, Sam in it): the eye at -5.1 stops against the open air's 0.12, the picture's middle 0.115, the slate
  floor grey and read; with the doors as the sheet had them before (the yard door shut) it was -5.4 and 0.125. NOT DRAWN: the black kitchen itself on the
  code before (the map's check stands for it), any other level. Levels whose door lists are empty or whose openings have no door are as they were.
  NEVER SEEN ON A GPU.
- **LIGHT STAYS IN ITS ROOM, AND A LAMP KEEPS ITS LIGHT BY DAY** (10-31; the look panel's row `rooms`, 1 on the look page, 0 = every number below as it was,
  `?look=rooms:0`; `lookConfig.rooms`. IT CHANGES THE LIGHT ITSELF on the look page, so the light meter and the guards' light read change with it there; no
  stand and no knob table, the look page is not the build. The maintainer, 10-31, of the pass's store, one bulb and no window: black on a grey day while
  the bulb glowed, then "compare to night and dusk", then of the cases found one by one "not fixing them systematically"). ONE STATEMENT OF THE FAULT: by day
  the Real light values row draws a lamp at the hour's exposure share of its night strength (`photometry.ts`: × 0.00087 on the grey day, × 0.0002 at
  midday), and the open air is thousands of times a lamp's light. Every place where the renderer let a LITTLE light through a wall, or tested a lamp's
  light against a FIXED level tuned at night, was invisible at night and is the whole picture of a shut room by day. THE MENDS, all under the one row:
  (1) THE CACHE: the cascades' hand-over overlap (`rc.setOverlap`), the secondary threshold `secMinE` and the dim-pixel rule held at the day's lamp share,
  and a bounce read that keeps to the hit's side of a wall (`FLAG_RC_ROOM`, `lighting_common.wgsl hitRadiance`: no ray, a plane test against the face that
  was hit). (2) `frame.lampCfg` (the frame's last member, zeros off the look page): x = the day's lamp share `lampK` = dayExp ^ the Real light values
  amount, y = what a glowing face is worth to a RAY (`boxEmissiveRay`: its night light × lampK; a mirror keeps `boxEmissive`). `lampFloorK()` holds every
  fixed night floor a lamp's light is tested against at that share: a light's score floor `1e-4` in the direct estimators, ReSTIR, sharp direct, the
  composite's pane light and the gather kernels, and the gather's firefly floor `0.35`. (3) THE WINDOW LIGHTS GET REACH MASKS (then `game.ts reachWindows`; since 11-01 the
  engine decides them itself and the game has no part in it, the DAYLIGHT AT OPENINGS bullet below): the engine made them a frame after the level gave its own lights theirs, so each was a candidate in every room. (4) THE FIREFLY FLOOR was the midday
  fault: a few gather rays in a shut room come back with daylight (the stills show a strip of it at the store's door; a CPU ray test of 60,000 rays from
  inside the room found none that left it, so the strip is on this side), the bound let each in whole (0.35 is thousands of times a day-drawn bulb's
  wall), and the filter spread it flat and white over the room. NOT HELD AT THE SHARE (a lamp's light in haze, smoke, particles and glass by day is
  still tested against a night's level): `HAZE_MIN_E`, `SMOKE_MIN_E`, `P_MIN_E`, `CARD_MIN_E`, `GLASS_MIN_E`. WHAT WAS RULED OUT ON THE WAY, each by a
  fresh-boot still (a row drawn after a row that switched a pass off is NOT evidence: its history is stale): the hidden sun, the sun's reach mask,
  reflections, the bulb's own glowing fitting, the windows of other rooms. THE SHEET: `node tools/qa/look-sheet.cjs --rows midday,midday+rooms:0 --views
  lamp`; its engine words `x.indirect:0`, `x.view:N`, `x.s.<setting>:<n>` and the `lit` print (which lights weigh at Sam, with their reach) are how each
  cause was tested. SEEN: software adapter only, each on a fresh boot. The pass's store, one bulb: at midday
  0.632 flat and white before the firefly floor, 0.142 after, warm, with Sam's shadow on the floor and the crates' on the wall; the grey day 0.118;
  night 0.224 (0.194 on a boot with the rows in another order); dusk 0.037. STILL WRONG IN THE STILLS: by day a white strip of daylight is drawn at the
  store's door on its right wall (and a small streak on the left wall on the grey day). The review's two mends (the bounce read's fallback cell, window
  lights dropped from the masked list when the engine replaces them) changed no number at midday. NOT DRAWN: a before row (`rooms:0`) on the code as
  pushed, any other shut room, any other level. NEVER SEEN ON A GPU.
- **WALLS HAVE TWO SIDES: ONE RULE FOR EVERY READ OF THE CACHE** (11-01; the look page's row `walls`, read at BOOT: `?look=walls:0` and a reload = the older
  guards; `render/frame.ts wallSidesConfig` → the prelude's `const WALL_SIDES`; `render/wallSides.ts`; `shaders/common.wgsl sideAt`, `sideOut`;
  `lighting_common.wgsl diceTapsBy`, `diceFaceBy`). The maintainer on his GPU, the pass's store by day: "also lots of light leak when doors are closed", and of the work as a
  whole: "i'd like us to try to remove as many "hacks" or special cases as possible -- I think having strong general systems is the core of this project". THE
  FAULT: the cache is probes a metre apart and a read blends the four columns round its point; within a metre of a wall one of them is the next place, and a
  face reads a spacing out along its normal, which by a wall is the next place too. At night that is a little lamp light in the wrong room; by day the next place is
  the open air, thousands of times a shut room. THE RULE: the level's floor plan (the exposure's place map, 0.4 m cells, `placeField.ts placeMask`) is flood-filled
  into places and handed to every shader as one u32 a cell at the end of the Frame uniform (`sideCfg` @928, `sides` @944, 28 KB: the scene group has no texture or
  storage slot left, and the fog's sight table is a uniform of twice that already), with the place of each probe column of the first cascade after it, a nibble a
  column (`sideCols` @28944, `wallSides.ts sideCols`: room for 4,480 columns, the finest cascade the debugger offers; a bigger one leaves the flag down). The two
  tables are 30 KB of the block and are sent to the GPU only when they have changed (`FRAME_HEAD_BYTES`, `engine.ts sidesSent`); every other page sends the
  head alone. A free cell's word is its place (0‥13, never shared by two places with cells
  within three cells of each other; 15 = a doorway, every place's; +16 = nothing but this place within three cells, the plain read); a wall cell's word is the
  wall's own line across x and across z (from the box's centre line, in 2.5 cm steps) and the place of each quarter, so a point 3 cm off a wall's face is on its
  own side though the cell is 0.4 m. A read leaves out the columns that stand in another place (one tap along a wall, two at a corner), and a face whose point a
  spacing out is another place reads half as far, else a hand off the face. WRITTEN TO BE PASTED ONCE: a face asks the plan in one loop (the air at the face, a
  spacing out, half, a hand off: `diceFaceBy`) and hands the point with its word to `diceTapsBy`, whose four columns come from the table, and the slab reads run in
  a loop over the taps (`diceIrradianceBy`). A face's read pastes `sideAt` into a program once; the first writing pasted it four times and sampled every slab twice. IT STANDS IN PLACE OF FOUR OLDER GUARDS, each of which met one case of the same fault
  by means of its own and all of which are dead code under the const (their text is still in the modules: the build and every other page read as they did):
  `diceIrradianceFrom` (08-12, the plane of the surface a gather ray left), `diceMeanRadianceWalled` and the view march's cached copy of it (08-09, an occlusion ray
  for a point in smoke), BOUNCE WALLS `diceWalledPoint` (10-20, five to eight box rays off a man) and LIGHT KEEPS TO ITS ROOM in `hitRadiance` (10-31, the cache's
  own nearest-cell read). WHY THE CONST AND NOT A LIVE ROW: the four compiled out are what pays for the rule in the final gather, which the software adapter
  builds only while its routine's stack frame is under 524,288 bytes: 521,176 before this row, 519,976 with it, and 530,008 as first written, when the page booted
  clean and the GPU process died at the gather's first dispatch (the bullet on the software adapter's limit below; `node tools/qa/kernel-stack.cjs`). WHAT IS A WALL'S LINE: the centre line of what makes the cell a wall on the map, one whole-height box or the boxes of one line that fill the height between
  them (a sill, its pane, its lintel). A box that fills part of the height on a line of its own (a counter against the wall, a bench, a board) lays none and
  stops none: as first written it did, and half the cells that said nothing on five levels were a wall with furniture against it. A wall cell with no line that
  stands inside ONE place (a square pier, a tall stack) is that place's. WHAT IT DOES NOT KNOW: anything inside one place (a shelf unit, a crate: a read behind
  one is the same room's, as it should be), a second storey (there is none: the lattice is the storey), and A WALL BUILT ASKEW BETWEEN TWO PLACES: those cells
  say nothing, and a read there is the plain blend WITH NO GUARD AT ALL, since the older guards are compiled out. Counted on six levels (cells that say nothing
  of all wall cells): the pass 25 of 641 (all the lorry's, askew in the yard; 58 before), bazaar 50 of 1250 (224 before), office 5 of 1049 (115 before), depot
  14 of 876, barracks 5 of 886, consulate 6 of 977. WHEN THE WORDS ARE MADE AGAIN (`wallSides.ts sidesKey`): a wall, a pane's box, a door's line, and a leaf
  COMING TO REST OR LEAVING IT, by the very test the words make (`SIDE_TURNED`). The map's own key steps every 0.04 rad, at other angles: a leaf that stopped
  inside the step it was last keyed in kept the words of a leaf still ajar (a shut door that said nothing), and a swing made the words on every frame, 2 ms
  each on the main thread. A leaf in mid-swing is keyed by a sixth of a quarter turn. The map itself is drawn once a frame for both its readers
  (`placeField.ts PlaceMap`). THE CHECK: `bun run tools/qa/wallsides-check.ts` (the pass,
  headless, the shader's read mirrored in TS: 11 places, 5674 pairs within three cells and none sharing a number; 318 face points 3 cm off six faces all on their
  own side; 65964 floor points and none with a column across a wall; 28 slow closings from four angles and none left stale, the words made 122 times in 11,228
  frames; the column table right at 40 × 28 and at 80 × 56; `WALLSIDES_BREAK=1` leaves one wall out of the map, skips the last rebuilds of each closing, and
  fails three lines). The look page is
  on neither stand, so there is no word for either. COST: `shaderinline-check` IS RED ON THIS BRANCH and its rows were not raised: it counts text and cannot see code a const compiles out, so it reads
  higher than what is built (composite 1060 of 1032 KB, 1027 before this work, the smoke lab's two pictures and the sided reads; `fgather / main` 936 of 805
  and `mainGi` 990 of 870, which were 894 and 957 and already over). Whoever takes the row to the build either cuts the dead guards' text or raises those rows
  on purpose. The uniform array's cost on a GPU is not measured (`?look=walls:0` and a reload is the A/B; the look panel's cost sweep leaves the row out,
  since moving it draws nothing). WHAT THE STILLS SAY (software adapter, the pass, High): midday and overcast, the shut store: the white floor behind the
  drums, the red streak up the drum and the white speckle on the man in the corner are gone, and Sam throws a shadow from the bulb; the back door's wall is
  clean. Night, two fresh boots with the row up and down: the same picture (mean difference 0.14 and 0.22 of 255 over two views), so the one rule does at
  night what the four guards did there. A white band on the kitchen's side of the store wall by day is the same before and after: light on that face, not
  this fault. NEVER SEEN ON A GPU.
- **THE PLACE MAP AT A SHUT DOOR AND AT A WALL'S FOOT** (SINCE 11-04 a shut door is its line's own wall with no leaf asked, `PLACE.shutNear` is gone, and the composite's side of a wall is `lookPlaceLevels` by `common.wgsl sideLine`: A PIXEL'S PLACE IS READ OFF THE FLOOR PLAN, below. The record: 11-01; `placeField.ts PLACE.shutNear`, `gapNear`; `composite.wgsl lookPlaceField`, `sideOut`). The
  maintainer, the pass's back door in overcast: "weird artifacts to the left of the door in overcast too", "doesn't show up in night but does for dawn and dusk".
  Hard-edged blocks one map cell wide on the wall and the floor beside the door. Two causes. A shut leaf's crack counted as a doorway, so cells at the jamb were
  every place's: a shut door's line is now wall from jamb to jamb (`placedoors-check`, its last line). And a point in a wall's own cell (the floor at a wall's
  foot, a wall's top) had no place and took its level from the cache at the pixel: it now takes the place on its own side of the wall's line (`sideOut`, up to four
  cells out), under the row above.
- **STONES GATHER ALONG A LONG FACE, NOT ROUND A THING PUT DOWN** (11-04, in place of 11-03's shelter rule; `render/wear.ts buildWearMap(level, nav, longFoot)`,
  `faceRuns`, `bandByFaces`). THE SWITCH: on the look's page only (`game.ts` asks for it when `lookConfig.on`), and `?longfoot=0` there is the band as every
  other page has it. Every other page has the traffic map's wall channel byte for byte as it always was (`bandByOccupancy`, the old code whole; checked
  against the old file on all 58 level files, and `probes.ts materials-wear` holds its sum on the office), so `detail.wgsl`'s grime band in the build does
  not move. The maintainer, 11-03 on the smoke lab's shed with the look on: "circles on ground near table legs are weird"; his 11-04 shots of a dressed yard
  (said of the paint, next bullet) show the same dark ring of stones at every crate, bench foot and stall post. The channel is 1 at the foot of ANYTHING
  standing on the floor, to 0 at 16 cm, and `look.wgsl` brings the gravel up in it and darkens it. On the look's page each FACE of a standing box now lays its
  band × how long the straight line of faces is that it is a piece of: faces that look the same way (3°), lie within 12 cm of this face's plane (a wall's
  pillar, a board leant on it) and meet end to end (3 cm). Nothing under 1.25 m, the whole band from 2.5 m. Asked face by face, never as groups of boxes; each
  box lays its band exact from its own rectangle (no occupancy, no distance transform: a turned wall has no steps) and a texel keeps the most any box gives
  it. A wall keeps its band at any yaw, beside a crate pushed against it, and along a front of piers and window sills whichever way it lies; a table leg, a
  post, a pier, a crate, a crate against a wall or touching a corner pillar have none; a block or a row of crates has it along its long sides and not at its
  ends. A band is a strip along something; round a prop it was a halo. KNOWN: a piece of wall under 1.25 m between two doorways has none (a lintel does not
  carry the line). TWO WAYS TO MAKE ONE CHANNEL until the look is the page: then `bandByOccupancy` goes. THE THREE TRIES BEFORE THIS, for whoever is tempted:
  (1) the share of floor stood on within 0.3 m (11-03, commit 693b7403) cleared legs and left every crate its ring; (2) the longest straight run of stood-on
  texels lost the band at the steps of a turned wall, cut a wall's band where a crate stood against a block, and gave a long block a band at its corners;
  (3) joining BOXES that lie in line into one foot (a union) joined a crate to a wall through the wall's pillar, measured a segmented front across itself
  when a cabinet stood end-on to it, and depended on which way a square box was described. `WEARFOOT_BREAK=1` (the band as every other page has it) fails the
  probe's line. On the dressed yard's map 31 % of the band's texels are gone, on the crossing 30 %; 4 ms a level, 14 at most (2000 boxes at random yaws:
  about 0.15 s). Seen as a map; the still drawn was of try (3); never seen on a GPU.
- **OLD PAINT IS CHIPPED, NOT BLOTCHED** (11-04; `render/lookTex.ts paint`, the look's layer for drums, gates, painted crates, a stall's sheet, a painted door; the
  look's page only). The maintainer on a dressed yard at midday, of dark islands on a green sheet, blue crates and a green door: "splotches on here look a bit
  weird, overdone". The tile peeled in ragged patches reaching 36 cm (5 % of it bare) with a pale rim; at the game's two centimetres a pixel a patch a hand
  across is a blot on a crate's lid. Now: chips, the shards of a scattered lattice (1 to 3 cm), gathered in the few places that took the knocks, and here and
  there a flake up to 10 cm with a lifted rim; 1.1 % bare. Whether a cell has a chip is asked once, where the cell's own point is (asked at each texel, a
  chip that straddled a worn place's edge was sliced along it). The chalking, the dents and the long scuffs are as they were. AND EVERY BOX HAS ITS OWN PLACE
  ON ITS MAP: `look.wgsl` shifted a box's own map (wood, sheet, cloth, metal, paint, anything that moves) by its number modulo 7, so two lids in a yard wore
  one blot; it is the whole number now (91 places; two unkeyed movers of one size still share one: `gbuffer.ts lookWordOf`). `lookmaps-check.ts` holds the
  tile (the widest bare patch reaches 12 cm at most, 0.2 to 2 % bare; the old tile fails that line). Not changed: what is under the paint is still a
  multiplier of the paint (dark and warm), so it cannot be rust on blue. Seen as a tile and in a software still; never seen on a GPU.
- **FOUR OFFICE STUFFS, ROOM FOR 31, AND A LEVEL HAS THE MAPS IT DRAWS AND NO OTHERS** (11-04; `render/lookTex.ts`, `render/gbuffer.ts`, `shaders/look.wgsl`; the
  look's page only). The maintainer, of a desk and a swivel chair in a dressed office at midday: "nicer materials on the chair and table". The look had one
  wood (weathered planks with gaps), one cloth (a sack's weave) and one metal (iron gone to rust), so a desk drew as a pallet, a chair as a sandbag and steel
  shelving as a wreck. NEW LAYERS, things made in a factory and kept indoors: `veneer` (leaves 40 cm wide, fine straight grain, pores, flush joints, no gaps or
  knots; laid the long way like boards), `plastic` (one colour, a pebbled skin, hairline scratches, rubbed smooth where it is handled), `fabric` (a tight
  two-yarn melange with a faint twill and pills where it is sat on), `steel` (a baked coat with the spray's orange peel, fine scratches, no rust and no chips:
  `paint` is the coat left out). They take no damp foot and no bleached head (`kept` in `lookSurface`), and a third of the slow blotches. NO PALETTE ROW NAMES
  THEM on this branch: a level's own table of paint names does (the dress goal's `scene/stuff.ts`), asking `lookStuffOf(name)` for the number. A STUFF'S
  NUMBER is its place in `LOOK_NAMES` and one; new stuffs go in FRONT of the two masonry layers, whose places the shader is told (`LOOK_L_STONE`, `LOOK_L_BRICK`
  in the prelude `gbuffer.ts lookWgsl` writes, with each layer's base roughness as a const array: the block's rows c, d, e.xyz are no longer read). THE WORD
  (`lookWordOf`, `LOOK_WORD`; `common.wgsl boxStuff`): five bits of stuff, one of moving, then the box's own number, now 0 to 44 (it was four bits and 0 to
  90): the word rides one float above the finish and the float had no bit to spare, so 31 stuffs are paid for by the number's range alone. THE MAPS: a layer
  is 2.8 MB and about half a second of arithmetic, so a layer is made the first time a box IS that stuff (`lookWordOf` notes it; `main.ts` asks every static and
  door leaf its word and names the props' paints before the first frame, and waits: `needLook`), with the layers its drawing reads (`LOOK_WITH`: a mud wall's stone and brick, earth's
  gravel), into the next SLOT of arrays that are only as long as the slots in use (grown by a copy on the GPU). The block carries the table from layer to
  slot (`LookCfg.s`, 272 bytes now); a stuff with no slot draws plain. The boot makes the men's cloth alone. A layer is made in SLICES (`lookLayerSliced`:
  30 ms at the boot, 3 ms in play, the event loop between; one body with `lookLayer`, the same bytes), so a thing that first appears in play as a stuff no
  static, door or prop was (a held thing, a thrown one) costs no frame: it draws plain for the second or so its map takes. The `even` row is read once, at the
  making of the arrays. They take the `kept` rule BY NAME: a stuff added later says in `look.wgsl` how it is laid and whether the weather touches it. `lookmaps-check.ts` holds the names against the
  shader's, the float, the slots and the block. Drawn as tiles (rebuild pictures); a software boot; never seen on a GPU.
- **CAR PAINT IS A STUFF; ITS SHINE IS THE BOX'S** (11-04; `render/lookTex.ts carpaint`, stuff 16, `look.wgsl LOOK_CARPAINT`; the look's page only). The
  maintainer, of the rounded cars in the dressed showroom: "i think the materials could use some work though. we already have some metal shaders right? maybe
  we want a car paint material too?". A truck's body drew as old `paint` (chips and rust specks all over), a saloon's as nothing. The layer is an enamel
  under a clear coat: one colour, the spray's orange peel as a wobble of the shine only, a film of road dust in soft patches that dulls the coat where it
  lies, the runs a rain leaves down through it, hairline cloth marks, a stone chip the size of a lentil here and there (0.1 % bare). In colour it is nearly
  flat ON PURPOSE: what it draws is where the coat is clean and where it is dulled (roughness 0.17 clean to 0.43 under dust). A MAP ONLY VARIES THE FINISH A
  BOX WAS GIVEN (`rough + dr`), so the gloss is whoever makes the car's to say on each box, and smooth finishes are what the ray-traced reflections trace
  (a cost to measure before a car park of them is a default). Agreed words, proposed to the cars part: body `carpaint`, roughness about 0.2, not metal (a
  metallic paint: metal, 0.3); glass plain, 0.05 to 0.1; bumpers and arch trim `plastic`, 0.5; chrome and hubcaps `steel` with metal, 0.25; tyres plain, 0.9.
  It takes the `kept` rule (no damp foot, no bleached head). Drawn as a tile; never seen on a page or a GPU.
- **DAYLIGHT COUNTED ONCE: AN OPENING IS A PORTAL** (THE ROW IS GONE SINCE 11-04 AND THIS IS THE ONLY WAY AN OPENING IS LIT: the maintainer played with
  it up on his M4 and chose to keep it on a card at 15:18Z, after saying of it that morning "my preference would be that we use this sampling
  unconditionally and then have no asymmetry between night and day". Deleted in one commit: the `portal` row and `lookConfig.portal`, the disk
  each opening was (`scene/openings.ts openingRadiance`, its fade out at night, `LOOK_OPENINGS.most` and `least`, the engine's `openAsPortal` and
  every `portal ?` branch of `engine.ts lookOpeningLights`). `&look=portal:..` on an address is ignored. The "Daylight at openings" row stays, as
  the switch it already was with this row up. Where this bullet says "with the row up" read "always", and "with the row at 0" read "until 11-04".
  `openings-check` (its disk lines are gone: an opening is lit at every hour, a night's sky is a sky), `portal-check` (`PORTAL_BREAK=1` fails 2),
  `lookexpo-check`. The fold's proof in a picture, software adapter, two boots a pair with the row on the old page's address: the white roof at midday and the block at night are each 0 of 2,304,000 bytes off the page as it drew with `&look=portal:1` (the night pair on the final tree; the midday pair on the fold before a review's tidy-ups, which touch no arithmetic: `Opening.area` and its work deleted, the portal record made with the light). The record: 11-04; the look panel's row `portal` "Daylight counted once", live, 0 ON
  THE LOOK'S PAGE UNTIL IT HAS BEEN SEEN ON A GPU, `?look=portal:1` = on; `render/lights.ts LIGHT_PORTAL` (bit 28 of a kind 3 light's kind word, `RtLight.portal`),
  `common.wgsl` `portalAt` / `portalRay` / `portalOutside` / `portalGround` / `portalPoint` / `portalFlux` / `portalFaces` / `evalLightDirect` /
  `lightPower` / `skyKeep` / `skyThrough`, `direct_common.wgsl shadowSample`, `rc.wgsl rcDice`, `place_field.wgsl reading`, `scene/openings.ts
  openingLooks`, `ajarOf` and `Opening.mid` / `half` / `ajar`, `engine.ts lookOpeningLights`, `frame.placeCfg.y`). The maintainer, of the nine special cases counted on 11-04: "I
  think we should pretty much fix all 9 things you found"; of this one, "this feels like tweaking the importance sampling for restir, right?" and "my
  preference would be that we use this sampling unconditionally and then have no asymmetry between night and day"; and of a door opened on a dark
  room in his two videos of 11-04, "I was still seeing some artifacts with the door and there's some odd brightness swinging". WHAT IT WAS: by day
  each opening was a Lambert DISK hung 5 cm inside the wall, as bright as a guess at what is outside (`LOOK_OPENINGS.up`, `.ground`), capped at
  `most` 2.5 m² because the bounce brought the same daylight in a second time, off at night by a fade, lit in one frame once a quarter of a square
  metre stood open by the straight way through, and hung in front of a leaf that then swung through it (read frame by frame on a GPU, a lounge door
  seen from above: the picture's mean goes 0.020, 0.028, 0.068, 0.092, 0.063, 0.039 over the five frames after the disk comes on, as the leaf
  uncovers and covers it; the pool on the floor comes and goes twice more; the leaf glows). WHAT A PORTAL IS: what is seen through the hole, at
  every hour: the sky itself, and the ground outside at the engine's one guess.
  ITS LIGHT at a point is WHAT THE POINT LOOKS OUT ON (`portalOutside`: above the horizon `skyRadiance`, the function a ray that misses asks, no
  second writing of it; under the horizon the ground outside, `portalGround`; no ramp between the two) × both cosines ÷ the distance squared, added up over `PORTAL_POINTS` 8
  points of the opening's rectangle (`portalFlux`: a lattice, one point to each eighth of the height; the distance squared is held off by a point's
  own patch over 2π, `portalSoft`). So a floor takes the sky, a ceiling the ground, and a wall across the room that looks out level takes the
  ground and next to no sky (with the sky alone, as this was first drawn, the lounge's far wall went from 0.30 to 0.03 on the software adapter and
  showed the gather's blotches: by day most of a room's daylight is the sunlit ground's). ITS SHADOW RAYS run ON THROUGH the opening: `PORTAL_SKY`
  40 m where they look up, to find that sky, so a wall across the yard, a shutter, the reveal of its own hole and A DOOR'S LEAF AT ANY ANGLE are in
  the light; where they look down, until they are `PORTAL_OUT` 1 m out from the opening's plane or at the ground, whichever is first
  (`portalPast`: clear of a leaf swung either way and of the reveal; the ground is what such a ray looks out on and not a thing in its way; what
  the ground is lit by is the guess). EACH RAY IS AIMED AS THE LIGHT COMES: at one of the eight points (the lattice moved round by the sample's two numbers), picked in proportion
  to what each sends the receiver. So a ray's answer is a plain yes or no like any light's (restir keeps a visibility in eighths; the denoiser reads
  "clear" as lit in full; the sharp pass asks for the emitter's middle with no numbers and gets the point half the light is under), and the share of
  a pixel's rays that get through is the share of the light that does. A blocker PAST the opening hands the denoiser the opening's own size for a
  penumbra: it makes the hole smaller and casts no edge. Any other light's ray is written as it was, to the last rounding.
  THE BOUNCE GIVES UP THE SAME SKY. A ray of the gather that leaves by a lit portal takes none of it (`skyThrough`, at the gather's three diffuse
  misses, the split gather's and the compact gather's; a mirror ray keeps the sky, it is a picture of it). THE CACHE gives it up at the dice bake,
  the probe the light is for: the cascades' record carries, in the fourth number nothing used, how much of a direction's light is the sky's (the top
  cascade adds the sky from a probe metres away, and sixteen directions are too few for a probe to find a doorway by), and `rcDice` takes out of
  each of a probe's faces what the portal would send it with nothing in the way (`portalFaces`, the same eight points) × the share of the opening that
  is open, never more than the sky the probe's rays really brought that face, in the hue of the sky toward the opening, and nothing where the
  portal's reach mask leaves the probe's cell out. THE PLACE FIELD reads the cache's +y face as the light that falls on a floor, and must read the
  same light whoever carries it: what the bake took out of that face rides in that slab's fourth number, eased as the light is, and
  `place_field.wgsl reading` adds it back, so the row moves no exposure (0 with no portal lit; `tools/qa/place-field-entry.ts` writes 0 there; the
  steady bounce's ring, `dice_ring.wgsl`, wrote 1 in every fourth number and now carries all four: with a 1 there every place read a whole unit
  brighter and a quarter of the lounge went black on the software adapter, which the row-off picture caught). THE GROUND OUTSIDE IS STILL COUNTED BY BOTH, as it was with the disk: the portal sends it at the guess,
  smooth and with the leaf in its way, and the gather's rays that hit it bring it again at the bounce's third (below, not done). THE TWO SHARES OF
  THE SKY ADD UP: a portal's light fades toward its range, and the ray keeps the rest of that window; an opening that is not in the list this
  frame is no portal and takes nothing. `frame.placeCfg.y` = the slot of the list's first portal + 1 (the walk starts there).
  WHO ASKS WHAT, because the final gather has no stack to spare (below): `evalLightRaw` is untouched and knows nothing of a portal. Only the picture's
  direct light (restir, direct, sharp_direct: `evalLightDirect`, then `shadowSample`) takes the rectangle itself and traces through it (since 11-04 what of
  it shows the outside past the hole's far rim: THE HOLE IS AS DEEP AS ITS WALL, below). To every other
  pass (the cache's `directIrradiance`, a bounce's hit off the screen, the light meter, the beams, a mote, a man's rim) a portal is an area light AS
  BRIGHT AS WHAT IS OPEN OF IT NOW, with a ray that stops short of it. THE RECORD: colour = what an opening looks out on as one radiance
  (`scene/openings.ts openingLooks`: half the low sky, half the ground, which is `LOOK_OPENINGS.ground` 0.5 of the sun's and the whole sky's light
  on it; the disk's own radiance without its amount and its fade at dusk) × the open area; `cosInner` = the open share of the rectangle; `cosOuter`
  its half width, `radius` its half diagonal. The picture's direct light reads the ground back out of it (twice the reference less the low sky) and
  takes the sky from the sky, so the whole rectangle's light has neither the share nor the sky's half of the reference in it, and restir's
  frame-to-frame ratio weighs a portal by the whole (`lightPower`: a leaf swinging is no change of the light a pixel's history was built for). So a leaf ajar dims the other
  passes by the record, and a wall across the yard is not in their light (11 × too much at the check's points behind a 3 m yard wall: the coarse
  end, as the disk was).
  WHAT IS OPEN (`Opening.ajar`, `ajarOf`): the narrowest way through the opening in plan, a height at a time. The two jambs (the footprints of the
  boxes that end the opening, `cut.jambs`: the tallest box the wall's own depth is crossed by there, so a crate against the wall is not one) and the
  boxes that hang in it are a chain across the doorway, and what gets through is what the gaps of the SHORTEST chain from jamb to jamb leave, each
  gap less the `play` 7 cm a shut leaf has in its frame (at one jamb or shared); what a leaf leaves over its head is a slit, as in a wall. A leaf is open by the gap at its
  free edge whichever way it swings and however thick the wall, a pair of leaves by the gap between them, and nothing there knows what a door is.
  The office's tea door: nothing to 0.2 rad (its edge is still in the wall's depth, then under the play), 0.02 m² at 0.25, 0.10 at 0.3, 0.61 at
  0.6, 1.55 of 1.70 at 1.5; the disk's `area` there is 0 to 0.5 rad on the frozen stand and jumps to 0.5 m² at 0.3 on the shipped one. A portal's
  rectangle is the bounding box of what the walls leave open, about its own middle (`Opening.mid`; the disk hangs at the open part's centre of
  weight). LIT BY WHAT THE PICTURE WOULD SHOW: `LOOK_OPENINGS.seen` 0.008 against what it looks out on × the frame's exposure, no hour and no area
  in it. The reach masks are found from the rectangle's middle and width (a disk's from its radius: the row flipped = found again, the old ones
  serving meanwhile).
  THE CHECKS. `bun run tools/qa/probes.ts portal` (`tools/qa/portal-check.ts`, family `render`, `PORTAL_BREAK=1` leaves the bounce and the dice
  their sky and fails): in a room built by hand with a 3 m wall across the yard, under a sky eight times brighter low down than overhead, against
  the same light added up over the doorway with a ray a point (48 by 48), the portal is 1.20 × a hand inside the sill and 0.95 to 1.04 × from 1 m
  in to the side wall and a crate's top; with the yard open 0.71 × at the sill and 0.90 to 1.03 × elsewhere, a crate's top at 1.6 m (the middle of
  the opening is below it) among them; nothing of that share is left to the gather's rays, 0.0 % and 6.1 % of it in the dice of three probes, the
  yard keeps every ray's sky, and the yard wall takes the rays that reach sky from 83 % to 16 %. With the ground (0.465 against a horizon of 1, a
  sun of 3 on ground that sends back half): the far wall at 1.5 m gets 0.029 where the sky alone gave 0.012 with the yard open, a ceiling 0.049
  where it gave nothing, a floor nothing more, each 0.98 to 1.01 × the sum over the doorway. A shut leaf reads 0 with a crate or a cabinet by
  the jamb listed before the walls, and 0.10, 0.59, 1.14, 1.53 of 1.80 m² at 0.3, 0.6, 1.0, 1.5 rad. (Two points up the opening, as it was first
  written, were out by 0.6 to 2 × on a crate's top and at the sill; a ray aimed evenly over the rectangle with a yes or no was 1.9 × behind the
  yard wall.) `openings-check` on both stands: the engine's line (a rectangle no smaller than its opening about its middle, the record's two
  numbers, the door by angle, lit under a night's sky, out under a black one, the masks found again), and EVERY LEVEL'S DOORS: 60 levels, 202
  openings a leaf hangs in, none reads open when shut, none under three fifths at 1.5 rad, every one found its jambs (`AJAR_BREAK=1` = no play
  allowed: all 202 read open, the line fails).
  THE STACK (`tools/qa/kernel-stack.cjs`): `fgatherS / main` 521,912 of 524,288 bytes (520,328 before: `skyKeep` at its misses is all the gather
  carries of this), `fgather / main` 522,216, restir 241,912, direct 283,896, the sharp pass 175,688, the beams and the meter as they were. The first
  writing, with the portal's line in `evalLightRaw`, was 536,648 and lost the device on the first frame: it is why the rule is opt-in a pass.
  ON THE SOFTWARE ADAPTER (the white roof at midday, its service door and its lounge door open, 960 by 600): with the row at 0 both
  pictures are the preview head's byte for byte (0 of 2,304,000). With it on, taken in turn with the disk in one boot, four frames each, while
  the room's exposure was still settling after its door opened: the lounge's far wall 0.298 and 0.206 with the disk, 0.181 with the portal, 0.158
  with the disk, 0.154 with the portal, on the one falling curve, and the place's level -4.98, -4.81, -4.69, -4.64 through the same turns (the row
  does not move it); no blotches. The service door from outside: the open leaf 0.137 with the disk, the brightest orange in the frame, and 0.085
  with the portal, darker than the sunlit dado; the room past it 0.163 and 0.107; the sun's pool on its floor 0.73 and 0.70. The lounge door at
  0.45 rad with the portal: the far wall 0.026. 118 pipelines, 0 page or GPU errors. What a software draw of four frames cannot say is how the
  light moves in time: that is the GPU's frame-by-frame recording to show.
  NOT DONE, known: the disk's path and its numbers stay until the row has been seen on a GPU and the maintainer has said which stays (with the row
  on, the `windows` row is a switch: any amount is all of it); THE GROUND OUTSIDE IS ONE GUESS A LEVEL (half of the sun's and the sky's light on
  it comes back, whatever the ground is and whether a wall shades it) AND IS COUNTED BY BOTH (the gather's hits past a portal want dropping, which
  is a line at each of the gather's hit sites and it has no stack for one; the mend for both is a pass of its own that asks the level's own boxes
  what an opening looks out on); a bounce's hit lit through a portal has two values, the exact one on the screen and the record's off it; a sky
  sample kept in the gather's reservoir keeps the share it was traced with (a hole blown, the 12 nearest changing: tens of frames); the penumbra
  hint for a blocker past the opening is the largest of a pixel's rays' and may smear a contact shadow in the same light; the sharp pass's one ray
  jumps between points along a line on the floor; the soft character path (off) stops a man's shadow at the opening's plane; the night's cost of a
  dozen more lights is not measured; a pixel's place near a door is still the floor plan's guess. Never seen on a GPU.
- **DOORS OPEN BY DEGREES IN THE FLOOR PLAN** (THE ROW IS GONE SINCE 11-04 15:18Z AND THIS IS THE ONLY MAP: the maintainer played it on his M4 and chose
  to keep it on a card, so the `doors` row, `lookConfig.doorsBy`, `PlaceMap.update`'s `byDegrees`, the map that stamped a leaf as wall cells and the
  engine's second map for the wall sides (`sidesMap`, `sidesKey`) are deleted; `&look=doors:...` on an address is ignored. Where this bullet says "with
  the row up" read "always". `DOORSBY_BREAK=1` now tells the map every door stands wide open and fails 3 lines. The record: 11-04; the look panel's row
  `doors` "Doors open by degrees", `&look=doors:1`, 0 in the page until it has been seen on a GPU; `scene/placePlan.ts placeMask` `steps`, `doorSteps`, `doorwayOpen`, `doorJambs`, `placeJamb`, `PlaceMap.update` `byDegrees`;
  `scene/wayThrough.ts`; `shaders/place_field.wgsl hand`, `doorShut`). The maintainer, of a door opened on a dark room: "some odd brightness
  swinging". FOUND in frame-by-frame recordings of the page on a real GPU (an NVIDIA card driven headless, the row not yet built) and worked out
  again on the CPU: a leaf was stamped into the map as wall cells wherever it stood, so while a door swung the map's walls moved under the place
  field (the pass's seven doors draw 120 different wall sets over their travel). On the white roof's lounge door the floor inside is held to the
  yard's level from 0.40 to 0.64 rad, falls back to the dark room's own from 0.68 to 0.80 (the leaf's cells wall it off from the doorway, and the
  floor is drawn six stops brighter for five frames: the recording's tiles there go 0.017 to 0.117 and back), and is held again from 0.84. WITH THE
  ROW: no leaf is a wall where it hangs; each door's line carries HOW OPEN IT IS in the map's bits above the low three, as a step of 1 to
  `DOOR_STEPS` (31), `DOOR_PER_STOP` (6) to a stop under wide open, so a step is the same sixth of a stop wherever it falls (0 = the map does not
  say: a doorway by its shape, wide open); a door at step 0 is shut and its line a wall from jamb to jamb. HOW OPEN is the narrowest way through in
  plan (`wayThrough.ts narrowest`, the measure a portal's open share is taken by, `openings.ts ajarOf`): the shortest chain of gaps from one end of
  the door's line to the other over what a leaf shuts against there (`doorJambs`: the level's own boxes that stand from the floor to a door's head
  within `WAY.play` of an end, `placeJamb`: a pier counts, a crate or a lintel does not) and the leaves near it, each gap less `WAY.play` (ONE number
  now, `wayThrough.ts WAY`: `LOOK_OPENINGS.play` is gone), over the same chain with nothing hung. So the wall's own depth counts: the pass's tea room
  door is shut to 0.25 rad, 0.26 of its doorway at 0.5 (the portal's measure says 0.26 there), 0.73 at 1.0, whole from about 1.4. IN THE FIELD the
  pull across a doorway's cell is × that share, and the brightest place a stride away is handed out of the doorway dimmer by it ONCE, on the hand
  that leaves the door's line (`max(0, giver's stops − taker's)`: two doors whose lines touch in a row are charged as the more shut of the two), a
  stop for each halving, however many cells deep the line is. The floor inside a door gets that share of the yard's light and is shown by the level
  of that share, so it should hold its brightness through the swing. A map with no such bits is every doorway as it was: `hand`'s arithmetic is the
  old expression. THE WALL SIDES still read a map with the leaves in it (`engine.ts sidesMap`, drawn only while the row and the field are both on):
  they were made for it, their check is red on the shipped stand at the base, and they are not this step's. COST: `PlaceMap.update` finds the jambs
  again only when the walls' key moves and the steps only when a leaf has moved, so a level at rest pays two keys. ROW AT 0 IS THE BASE: on 60
  levels with every door at seven positions the map, its key and `PlaceMap.update` are the head's byte for byte (840 maps, 420 updates), every
  opening's open area is the head's (2807), and with every door shut the by-degrees walls are the old map's on all 60. CHECKS: `placedoors-check`
  (both stands; its last three lines: every room its own place by degrees too; one wall set a door over the whole swing, the step never falls as a
  door opens and is 31 at both ends of the travel of the pass's doors, the tea door's share pinned at five angles, the map the engine keeps equal to
  a fresh one at 550 positions, after a wall goes and with the row put off and on; on every level, 580 doors, shut is shut, either end of a travel
  is 30 or 31, and all 1160 line ends have a jamb; `DOORSBY_BREAK=1` draws those lines' map without the row and fails, `PLACEDOORS_BREAK=1` fails
  too), `lookexpo-check` (the rule's text, the two constants against the plan's, and the rule written again: a stop under for a door half open,
  through a wall one cell deep and through two), `place-field-page.cjs` on the software adapter (19 of 19: the second block is the rule's own answer
  in every cell of seven scenes to 0.02 of a stop; the room's first cell is held to what a wide door gives less the stops the rule says; room A's
  own level rises from shut to clear and is one level across at every step). MEASURED on the software adapter and the CPU only: THE ROW ON HAS NEVER
  BEEN SEEN ON A GPU. NOT DONE: two leaves that meet in one doorway are two doors, each measured to the point they meet at (no level has such a
  pair); a door whose travel stops short of about 1.4 rad rests a step or two under whole (the barracks' n3 at 30), which is what its leaf leaves of
  the doorway; the threshold's own cells still hold a level of their own the moment a door leaves its frame (as they do without the row); the wall
  sides, above; the eye's own easing is untouched.
- **ONE EXPOSURE AT EVERY HOUR** (THE ROW IS GONE SINCE 11-04 15:18Z AND THIS IS THE ONLY PATH: the maintainer played the block at dusk and the white
  roof at midday with it up on his M4 ("overall 2 looks a lot better!") and chose to keep it on a card, so `hours`, `lookConfig.hours`, `everyHour`
  in `engine.ts lookShade` and `lookExpoFrame`, the shade row's fade into a night, the day's share and the shader's two tests of `placeCfg.z`
  with the older night bound behind them are deleted in one commit; `&look=hours:...` on an address is now ignored. The `dark` row stays. What
  follows is the bullet as written with the row, for its reasons and its numbers; where it says "with the row up" read "always", and where it
  says "with the row at 0" read "until 11-04". `HOURS_BREAK=1` still leaves the eyes' widest out of the check's model and fails 6 lines. The
  fold was NOT drawn byte for byte against the row at 1 (a dusk and a night want two boots each): its proof is the check, and the midday boot of
  the step after it (A PIXEL'S PLACE IS READ OFF THE FLOOR PLAN). 11-04; the look panel's rows `hours` "One exposure at every hour" and `dark` "Eyes
  open in the dark", `&look=hours:1`, `hours` 0 in the page until it has been seen on a GPU; `render/lookexpo.ts lookExpoFrame` `everyHour`, `lookExpoBound`,
  `lookNightSkyE`, `LOOK_EXPO.dim` / `.dimSky` / `.dark` / `.read` / `.lamp`; `engine.ts lookShade` `everyHour`, `eyesWidest`;
  `shaders/place_common.wgsl lookExpoFor`'s last lines, `lookRoll`, `EXPO_READ`, `EXPO_LAMP`; frame.placeCfg z and w; the page's
  `lookConfig.nightSky`). The maintainer ruled every special case of the light fixed (11-04, "I think we should pretty much fix all 9 things you
  found"). THE ONE EXPOSURE MODEL ASKED THE HOUR: the shade row it rides on faded out as the sky dimmed (`smoothstep(0.08, 0.35)` of the sky's
  mean radiance: 0.76 of itself at dawn, 0.58 at dusk, 0 on a night), the model's own powers took that share, and on a night the frame got no
  model and no gain of any kind, so the picture was exposed for nobody: a team that starts in an unlit room was faint outlines on black (seen on
  a real GPU: the block, the consulate, the depot, the glasshouse). WITH THE ROW the model asks no hour. The day's share is 1 and the shade row
  has no fade; the place field runs at night; and what holds a night's picture is THE EYES' WIDEST, the bound the model already had, said at
  every hour: no place is shown by more than the night's exposure over the hour's (1 on a night, 69 at dusk, 5,050 at midday), except a place
  whose floor holds less than the DIM LEVEL, which is brought up by the shade row's share of its dark under that level, `LOOK_EXPO.dark` stops
  (2) × the `dark` row at the most: `min(gain, nightOver × clamp((dim ∕ here)^share, 1, 2^stops))`. It is the rule the model has by day for a
  place darker than the open air, about the level a night's exposure leaves off at, and it is the same WHEREVER THE EYES ARE: no two dark places
  are drawn at one level, a darker corner stays darker, and the spill inside a dark room's doorway keeps its fall-off. (Two shapes were tried
  first and dropped after review: a plain cap at the dim level drew every place between a quarter of it and all of it at one grey; bringing the
  eyes' own place up whole and the rest by the share did the same for every place between the eyes' level and the dim level.) THE DIM LEVEL is
  `LOOK_EXPO.dim` (0.03, the place field's own measure, the night's units) and never over `dimSky` (0.4) of what the night's sky puts on open
  ground (`lookNightSkyE`: the sky the frame draws on a night, whatever a weather or a slider has done to it; the level file's own while an
  hour is up, by the page's word `level.sky`; a level that is its own day has none and takes 0.03). On the block the yard holds 0.071 of a sky
  that gives 0.082, the lit rooms 0.14 to 0.38, the far end of the weakest lit hall 0.031, the unlit room the team starts in 0.004 to 0.009
  (shown × 2.6 to × 3.8); under the solo tutorial's sky, six times blacker, the dim level is 0.0054 and its yard about 0.012. So A NIGHT'S YARD
  AND ITS LIT ROOMS ARE THE NIGHT'S OWN, wherever the eyes are. BY DAY the dim level in the hour's units is at the field's read floor
  (`EXPO_READ`, three stops over the solver's 1e-6): at midday the row moves no place and no light, and a black room is not shown by its noise
  any further than it was. UNDER THE GOGGLES nothing is brought up (`lookExpoBound`'s `nv`, all gone by halfway down, where `post.wgsl` takes
  the tube's picture): the tube is the eye, and `game/nvg.ts` models it on the light as it is. Before the field is live the bound takes no
  number from a place. THE SHOULDER (`lookRoll`) is never about less than THE NIGHT'S WHITE with the row up (`EXPO_LAMP` 1.23: a pale floor
  under a room's own 15 lux bulb as a night shows it), so a room brought to the night's own picture at dusk or on a grey day is rolled off where
  the night rolls it off and not at that hour's dimmer sunlit ground; at midday sunlit ground is over it and nothing moves. A lamp-lit room at
  dusk is shown as the night shows it, or where the open air's middle is if that is darker (the day's fade had left it at a third of the
  night's). WHY THE DIM LEVEL IS A NUMBER AND NOT THE YARD'S OWN LEVEL: the field holds the sky's light and the bounce, not a lamp's own beam,
  so a lit hall with dark walls can hold less than the yard. Set at the yard's level (tried first), such a hall counted as a dark place and was
  shown brighter the moment the eyes were in it. The honest measure of "dark to the eye" is what a place's floor sends back, which carries the
  lamps; the field has no such block yet, and with it the dim level could be the yard's own and the unlit room could come up further. ROWS AT 0
  ARE THE BASE, and so is a night with the `dark` row at 0: on the software adapter the block's night opening is the head's picture byte for
  byte with the rows as the page opens, and again with `hours` at 1 and `dark` at 0 (the model runs and every gain is 1); a midday with `hours`
  at 1 is the midday with it at 0, byte for byte. A device that cannot build the field brings no place of a night up (the bound waits for a live
  field; what else it draws is under THE ROWS THE ONE EXPOSURE MODEL REPLACED ARE DELETED, below). CHECKS:
  `lookexpo-check` on both stands (eleven lines under ONE EXPOSURE AT EVERY HOUR; `HOURS_BREAK=1` leaves the bound out and fails six), with
  `lookgrade`, `lookunits`, `frame-layout`, `shaderrows`, `knobs`; all 68 shader texts compile. WHAT MOVES WITH THE ROW ON, the other
  rows as the page opens: a night only where a place is under the dim level; dawn and dusk (the model at its full share, the night's white);
  the grey day in places under the dim level and in the shoulder; midday nothing. (With the Sky light row well under 1 the shade row's fade was
  acting at midday too, and its going moves that.) COST, not measured: at night the frame runs the field's sweeps and the composite's place
  read, as a day does, also where every gain comes out 1 (the `dark` row at 0, the goggles down). NOT DONE: seen from inside a dark room, a
  moon patch on its floor is shown brighter than the moonlit ground outside the door (the Light in order row is bounded by the eyes' own
  level, by day too); the lamps' haze and beams take no gain (as by day: `volumetrics.wgsl airShow`'s note; lit particles do since 11-04: A LIT
  PARTICLE IS SHOWN BY ITS PLACE), so a torch beam
  in a room brought up reads weaker against it; the light debug views are × the gain; on a switch of hour from the panel the field holds the
  old hour's units for the frames it takes to settle; the comments in the composite and the smoke march that say "1 at night" are true of the
  page as it opens and are to be mended when `hours` is its default; the Contrast row still fades by day (`lookgrade.ts`); the Room brightness
  row's light is not a night's (`lampCfg`); lamps are still the game's by night and real by day (`photometry.ts`). (The rows the model
  replaced went in the step after this one: the next bullet.) MEASURED on the software adapter and the CPU only: THE ROWS ON
  HAVE NEVER BEEN SEEN ON A GPU.
- **A PIXEL'S PLACE IS READ OFF THE FLOOR PLAN, NEVER GUESSED** (11-04; the look page only, no row: it is the only path. (LATER THE SAME DAY the
  rule's text, `lookPlaceLevels` with `placeBlend` and `placeSideCell`, moved word for word from the composite's two writings into
  `place_common.wgsl`, the one text they are built with, when the particles' splat came to ask it: A LIT PARTICLE IS SHOWN BY ITS PLACE.) `composite.wgsl` and
  `compact/composite.wgsl lookPlaceLevels`, `placeBlend`, `placeAcross`, `placeSideCell`; `common.wgsl sideLine`, `sideIsDoor`;
  `place_field.wgsl placeSweep`, `hand`, `placeEye`; `render/wallSides.ts placeSides` kind 3; `scene/placePlan.ts placeMask`,
  `PlaceMap.shape`). The maintainer on his M4, the white roof at midday: "weird flash on the door when opening", "there's a dark band right by the
  entry of the door threshold and door", two dark squares on the floor at a door's jambs in a still, and of the depot at midday "lots of bleeding in
  that level"; before those, 10-31: "i worry that we're finding these cases one by one and not fixing them systematically". ONE CAUSE: each was a pixel
  shown by the gain of a place it is not in, and each came from the step that ran where the place field had no plain answer. In a wall's cell, a
  doorway's, and a door mid swing the place was GUESSED from the radiance cache near the pixel (`lookPlaceHere`: five reads, a box ray each, then
  "whichever of the two levels is nearer"). The flash was a door that had begun to swing: the engine kept two maps, the field's (doors by degrees)
  and an older one for the wall sides that stamped the turned leaf as wall cells, so for the frames between shut and the first step the door's cells
  were wall in one and said nothing in the other, and the floor at the door's line went to the guess. The band and the squares were a doorway's cell
  holding a level of its own, between its two rooms', and the strip of floor at a jamb's foot taking one cell's level unblended beside floor that was
  blended. The depot's white lines were sunlit wall tops shown by the gain of the dark room under them.
  THE RULE, as `lookPlaceLevels` asks it: a wall's top (a face that looks up above 1.88 m within 0.14 m of a wall's line, the part that overhangs the
  room's own cell too; a tall cabinet's top further from any line is its room's) is of neither room and takes THE BRIGHTER of the two its line parts,
  or of the four places round a corner where it is over both lines of a corner's cell;
  a free cell answers with its own level, blended with the free cells beside it; a wall's cell and a doorway's answer by the plan's own line through
  them (the wall's centre line, the door's line shut or open and at every angle of its leaf; to 2.5 cm), with the first cell that holds a level on
  the point's side, blended as any cell of that room is; a cell with no line, or with nothing its walk could reach, takes the brightest of the nearest cells
  that hold a level (on the point's own side of its cell's lines first); only the middle of a mass wider than 3 m, with no place in four cells, is shown by its own light. THE WALK to the answering cell
  (`placeSideCell`) turns at a doorway it meets from the side, to the point's side of that doorway's line, and stops at another wall's line: the
  floor at the foot of a wall that ends in a doorway is of the room on its side of the door, and the pocket behind a shelf's end panel is never the
  room across the partition. NOTHING ELSE is by the pixel, no read of the cache says a place, and the composite carries one box walk less
  (`shaderinline-check`: 1070 to 1052 KB a module, still over its row of 1030 as it was).
  WHAT ELSE MOVED, all on the look page: (1) A DOORWAY'S CELL HOLDS NO LEVEL in the field, and no level is pulled through a doorway (`PLACE.link`
  is gone): two rooms through an open door are two places to the door's line, each with the level its own floor holds, and what joins them in the
  picture is the Light in order row, which shows the floor a bright room lights inside a dark one's door by the level of the light on it. A room's
  level no longer moves when its door opens unless its light does. (2) ONE MAP for both readers: `PlaceMap` by degrees; `WallSides` is made again
  when the map's SHAPE changes (which cells are wall, which doorway: `PlaceMap.shape`) and not at each of a swing's 31 steps (49 makings in the
  check's 28 closings, where keying it by every drawing made 500). THE WORDS RIDE WHENEVER THE PLACE MODEL IS ON, whatever the Walls have two sides
  row says: with that row at 0 only the cache reads' flag is down (`frame.ts`: `sideCfg.x` is up with the column table, the words are written with
  either reader). A doorway's word (kind 3) carries its lines and no places. (3) A DOORWAY WITH NO DOOR has ONE LINE A RUN: the line of a door
  whose cells it touches, else the centre line of the walls that are its jambs, else its middle along the way through, which is the axis whose two
  hands no wall touches (a line a cell made a doorway two cells deep four bands of its two rooms: 158 runs on 29 levels, found by review before it
  was drawn). (4) A LINE ON A CELL'S EDGE is written a step outside the cell (it lay between two steps of the 2.5 cm grid and float noise put a
  hairline of the wrong room along 45 doors). (5) THE AIR (`placeAir`) is as it was, one answer a cell, because the march keeps a cell's answer for
  its steps there: a doorway's cell, which holds no level now, takes the brightest free cell within two as a wall's cell does, so smoke on a
  threshold is shown as the brighter room's. (6) THE EYE where Sam's cell holds no level (a threshold, a short passage, against a wall) takes the
  NEAREST cell that holds one, by where he stands in his cell (it took the brightest beside him, which shut the eye on a dark room he had not
  left). (7) THE FIELD IS BUILT WITH THE BOOT'S PROGRAMS (`engine.ts buildPlace`, awaited by `init`), since before it is live, and on a device that
  cannot build it, every place is now the open air and no gain is taken: there is no second model to fall back on. A device with no field shows the
  look's interiors unexposed, with a console warning.
  DELETED with it: `lookPlaceHere`, `lookPlaceField`, `sideOut`, `lookConfig.footing` and `?footing`, `frame.gradeCfg.z`'s use, the `doors` row,
  `PLACE.link`, `PLACE.shutNear`, `PLACE.leafStep`, `lighting_common.wgsl diceMeanRadiancePlain`, `tools/qa/lookplace-check.ts`.
  CHECKS, no GPU: `wallsides-check` section 8 copies the shader's rule line for line and holds (a) every cell of a door's line knows the line at every
  angle of its leaf (1,694 cell positions of the pass's seven doors); (b) a walk through an open door a millimetre a step is one room to the line
  and the other from it, at three angles; (c) 318 points at the foot and on the faces of the store's walls are their own room's; (d) a wall top is
  the brighter room's from edge to edge, and a cabinet top inside the room or against the wall is the room's; (e) both writings of the kernel carry
  the same text and no read of the cache; (f) ON ALL 60 LEVELS, doors wide and shut: 2,119 doorway runs change rooms once; of 206,990 points in the
  cells with lines that their side answers, 80 (0.04 %, the check's bar is 0.05) are answered by a room they cannot see past things a wall's height
  (corners where masses stand three deep, most in one warehouse), and 1,858 more, in pockets between tall things with nothing on their side, take the
  brightest cell near them and are not judged; of 362,475 points on the tops of upright walls, a millimetre in from each edge, 0.27 % are shown as
  one room's (walls beside another tall thing whose line the cell holds; bar 0.5); of 24,884 points on the tops of tall things 0.3 m from every wall
  box 0.27 % are taken for a wall's top (beside a window wall, which is no one box; bar 0.5). `SIDEDOOR_BREAK=1` (the words made with
  no door told) fails 4 lines and `WALLSIDES_BREAK=1` 4. `placedoors-check` (6 lines; `DOORSBY_BREAK=1` fails 3, `PLACEDOORS_BREAK=1` 6),
  `lookexpo-check` (both stands; `HOURS_BREAK=1` 6, `LOOKEXPO_BREAK=1` 13), `openings-check`, `smokelab-check`, `place-field-page.cjs` on the software
  adapter (19 lines: a doorway's cell holds no level, an open door does not move a room's level by itself, the brightest place a stride away still
  comes in with the swing), `wgsl-compile` all texts, both typechecks. Two reviews; the second's finds are in the rule above.
  DRAWN, software adapter only, and thin: the white roof at midday boots on the final tree with all 120 programs (the field's two are in the boot's
  set now), no page error and no GPU error, and the door between its two rooms swung through five angles draws as it did before, frame for frame by
  eye. His flash does not show on this adapter before the change either (it is a matter of which frame a map was drawn on), nor do the depot's white
  wall tops in its opening view, so THIS STEP'S PROOF IS ITS CHECKS, line (a) for the flash and (d) and (f) for the tops, and the picture wants his
  GPU.
  NEVER SEEN ON A GPU. Not drawn at all: a doorway with no door; smoke on a threshold; the eye on a threshold; a device with no field; the page with
  the Walls have two sides row at 0. A wall top between two indoor rooms takes the brighter room's level whatever light is on it: by eye. NOT THIS
  STEP'S, seen in its stills before and after alike: a cell-sized grey square on the floor at a door's jamb (the maintainer's still of 11-04). It
  is in the light, not the exposure: the cache's read by place (`lighting_common.wgsl diceTaps`) at the cells beside a doorway. Open.
- **THE HUE SPLIT ASKS THE BELIEF THE HUE WAS BUILT BY** (11-04; EVERY PAGE, no switch: it is the direct light estimator's own rule put right.
  `shaders/restir.wgsl` and `compact/restir.wgsl`: the per-pixel dark memory `darkSlot` is moved AFTER the shade). The maintainer on his M4, the
  depot at midday: "red rim on shadow also looks odd". Every shadow the sun threw, a man's and a wall's, was edged in saturated red. FOUND BY
  SWAPPING ONE THING AT A TIME on software stills of one shadow edge, in the direct light's own debug view: the rim stayed with a pure white sun,
  with the contrast row at rest, with the temporal resolve's clip opened and its sharpening off, with the spatial reuse off and with the other
  shadow denoiser; it went with the mast's red obstruction lamp's colour set to black, and with every sample shaded in its own lamp's hue
  (`settings.restirSampleHue`). That lamp is some 26 m off: at its night strength it would send that ground under a ten-thousandth of the sun's
  light, and by day a fixture is drawn at a share of that. THE FAULT: the estimator shades a class's light in the hue of the lamps the pixel BELIEVES visible (`Uv`), and a
  pixel leaves out the lamp its own ray last found dark (`darkSlot`, six bits a pixel). The HUE SPLIT lets a sample of a lamp that is not
  believed keep its own hue. `Uv` was summed by the memory as the frame found it, and the memory was then moved by this frame's rays BEFORE the
  split asked it. In a half shadow a pixel's ray is blocked one frame and through the next: `Uv` had left the sun out, the ray came through and
  cleared the memory, the split called the sun's sample believed, and the sun's light was painted in the hue of what was left in `Uv`. So ANY far
  dim lamp of another colour edged every half shadow of a strong one in its own colour, at any hour; the split had thinned the fringes it was
  written for (the green exit sign's) and not ended them. THE MEND IS AN ORDER, not a second word: the memory's two lines run after both classes
  are shaded, for the next frame, so the sum and the split read one value. Nothing else read the moved value (the blur hint's gate asks the tile's
  belief; `U.a` is `Uv`'s). WHAT ELSE IT MOVES: a pixel one lamp reaches, on the frame its fresh ray is first blocked while its history's sample is
  lit, is shaded by `Uv` where it was shaded by the lamp's nominal hue; the same to rounding for a lamp, and for an opening's daylight the exact
  hue of what it sends in place of its record's. Check: `bun run tools/qa/probes.ts huesplit` (family `render`; the order held by its text in
  both writings, and a one-ray model of the half shadow that takes its order from that text; `HUESPLIT_BREAK=1` moves the memory first and must
  fail; with the kernel's text as it was both lines fail), `shaderrows-check` (the compact estimator re-stamped), all 69 shader texts compile.
  DRAWN, software adapter, the depot at midday, one wall's shadow on the dust: across its edge in the direct light's view red over green ran
  from 1.00 up to 1.6 and 2.7 before, and stays at 1.00 to 1.02 with the mend; in the picture the edge peaked at 1.63 on dust that is 1.25 lit and
  1.28 in shade, and peaks at 1.39 now (what the tone curve does to a tan as it darkens, and no more). NEVER SEEN ON A GPU, and not timed there:
  the kernel does the same work in another order.
- **A LIT PARTICLE IS SHOWN BY ITS PLACE** (11-04; the look page only, under the one exposure model, with no word of its own: it rides the shade
  row as the floor's gain does, and off the look page the splat's text never compiles the call in (`const LOOK`). `particles_draw.wgsl partShow`,
  `vs` / `vsKind`, `partAt`; `place_common.wgsl lookPlaceLevels`, `lookGainsOwn`, `lookShowBy`, `lookAirGains`; `render/particles.ts setPlace`;
  `render/placeField.ts placeNone`). The maintainer on his M4, the white roof at midday: "GPU particles (like casings) look like they're not lit".
  THEY WERE LIT AND SHOWN WITH NO GAIN: the composite shows each place's surfaces by the place's gain and the smoke march its air, and the splat is
  drawn after both. Brass on the floor of a room shown thirty times brighter was drawn at a thirtieth of the floor it lies on. A PARTICLE NOW ASKS
  WHAT A POINT OF THE FLOOR ASKS, the same three functions in the same order: its place off the floor plan (so at a wall's foot it is of the room on
  its side of the wall's line, in a doorway of the side of the door's line, and between two cells the blend), the Light in order row with the light
  that falls on it, and the one rule for what a place sends the eye. For that the floor plan's rule moved, word for word, out of the composite's two
  writings into `place_common.wgsl`, the one text the composite, every volumetric march and now the splat are built with. ONE CALL, where the vertex
  stage hands its answer on (`vs` wraps the kinds' own writings, `vsKind`), so a new kind cannot miss it; the splat's module grew by one call's
  text. Over the roofs and past the map's edge a particle is in the open air, as the air's own rule has it. The splat binds the place field at
  group 1 binding 7 (a uniform in the vertex stage, 56 KB, under the default limit), handed each frame by the engine with the composite's; off the
  look page both get the same zeros (`placeNone`, one a device). The smoke march's inline copy of the air's gains is `lookAirGains` now, the same
  arithmetic. WHAT IS NOT IN IT, on purpose: A SPARK'S OWN LIGHT is sent on as it was. What a glowing thing is shown by (a lamp's face, a screen, a
  spark, a fire) is one question and is not asked here: put through the rule as it stands, a spark in a dark room is held at the shoulder's top
  through nearly all of its cooling and then cuts off. THE LIGHT ON A PARTICLE IS AN ESTIMATE: the sim hands the splat what the particle sends on,
  in which the bounce is already × `indirectScale`, and the record has no free word for the light itself. The splat works it back (÷ the albedo's
  brightness × π) and puts the bounce back at its true strength as far as the place's own level says it. Deep in a room that is the floor's own
  number to the bit; where the bounce at the particle is three times its place's level and no lamp lights it, the particle reads 1.49 times over the
  floor's share (`indirectScale` 0.35), and a case's glint reads the other way. Both only where a brighter place is a stride away, which is the only
  place the row asks. The lasting mend is a word in the particle's record for the light on it. Check: `bun run tools/qa/lookexpo-check.ts` (the
  splat's lines: the text of the call, of its three builds and of the field's hand-over, and the arithmetic of the estimate, mirrored;
  `PARTSHOW_BREAK=1` shows a particle with no gain and must fail: 3 lines, on both stands), `tools/qa/wallsides-check.ts` (the rule is one text and
  each of its three askers calls it once), all 69 shader texts compile, `shaderrows-check` (the compact march re-stamped). DRAWN, software adapter, the white roof's service room at midday with its door open (a room the
  model shows by its own level, far over the frame's), 48 casings put on its floor: they read 161 of 255 by luminance on a floor at 217 before, dark
  grey dashes, and 233 with the mend, pale gold (154, 161, 174 and 239, 233, 213); the page booted and drew with no page or GPU error. NEVER SEEN
  ON A GPU. NOT DONE: a spark and the other glowing things; the lamps' haze and beams; a particle's own light carried from the sim.
- **THE HOLE IS AS DEEP AS ITS WALL, AND AN OPENING'S DAYLIGHT KNOWS IT** (11-04; the look's page, where the openings' lights are; ON, the address's
  `&holedepth=0` = the whole rectangle as it was, `settings.portalHole`; no shader variant: a depth of 0 is the old light. `common.wgsl portalSeen`,
  `portalDeep`, `portalRims`, `PORTAL_RIM`; `scene/openings.ts Opening.deep` / `.rims`; `render/lights.ts portalHoleBits`). The maintainer on his M4,
  the white roof's service door at midday: "boiling video has boiling in the sun on the ground through the door, especially along left edge". THE
  CAUSE, found on software stills in the direct light's own view: the fan of daylight a doorway lays on the floor ends at its sides where the wall
  the doorway is cut in hides the outside. From a point off to one side most of the rectangle looks onto the hole's own far side, 0.29 m of wall. The
  light was the whole rectangle and each ray found that out for itself, one yes or no a pixel a frame: in the check's room 15 to 65 % of a pixel's
  rays died on the jamb across the fan, and in a room the one exposure model shows hundreds of times brighter that chance was the picture. THE RULE:
  the far rim of the hole is the same rectangle a wall's depth further out, so what a point sees the outside through is the overlap of two
  rectangles, which is a rectangle. The light is that part: `portalFlux` (what the picture's direct light weighs), `portalFaces` (what the cache's
  dice give up) and `shadowSample` (where the ray is aimed) lay the same eight points in it and take its share of the whole, so the hole's sides are
  in the sum and a ray is left to find what really stands in the way (a leaf, a man, the wall across the yard). DEPTH: from where the light hangs
  back to where the boxes that END the opening reach out (the wall's own, as `cut.jambs` finds them; the shallower end; `proud` alone where neither
  end has one), not the wall's depth as `cut.out` has it, which a fence or a shutter near the line moves. In the kind word's bits 11 to 20, mm,
  where a lamp's marking rides. RIMS: an end has a far rim where the opening itself ends, not where a long strip of glass goes on as its next
  light, whatever post or bar stands between (bits 21 and 22; a light that says nothing of its rims has none, and a ray finds them). The head and
  the foot always have one, a doorway's foot too: the threshold in the wall's depth is floor the picture draws and the bounce lights, and the ground
  a portal stands for begins past the wall (the check's reference sum says the same now; before, that strip was counted as open ground by the
  portal and again by the bounce). THE RIM IS TAKEN HALF A SAMPLE WIDER than the rectangle (`PORTAL_RIM`, pinned to `LOOK_OPENINGS.ds` and `.dy`):
  the walls' real edge can stand that far outside the rectangle the samples gave, the bounce gives up its sky by the whole rectangle (`skyKeep`),
  and what the rule left out no ray would look for; what it leaves in, a ray finds as before. WHAT ELSE IT MOVES: beyond the fan's sides the
  portal now sends nothing where it sent a light no ray confirmed, and the estimator's picture of that floor loses a blue cast (drawn below); a
  floor point a hand in front of the light's plane gets its full light where the eight points' cosine faded it over 25 cm (nearer the sum over the
  doorway; a step at the plane is the limit, never seen on a GPU); a shadow's blur hint is still the whole rectangle's size (`li.radius`) where the
  part seen is a sliver, as it always was. Checks: `bun run tools/qa/probes.ts portal` on both stands (its mirror lays the points as the shader does;
  line 4z: across the fan 94 to 97 % of a pixel's rays reach sky where 35 to 85 % did, the light within 4 % of the sum over the doorway at every
  point; line 4y: a strip of glass 4 m long in each of the room's four walls is two lights with rims 1 and 2 in the shader's order, each 0.95 to
  0.97 × the sum over its own rectangle, 0.94 × with a rim at both ends; the ground line asks three points near the door now, 0.97 to 1.05 ×; the
  text pins. `PORTALDEEP_BREAK=1` = the whole rectangle, fails 3; `PORTALRIMS_BREAK=1` = a rim at both ends whatever stands there, fails 1). The
  mirror's rays went from 24 by 24 pixels to 48 by 48: at 24 a point 4 % of the rays reach sky from read 0.78 × or 0.97 × by the count alone. The
  older figures in the portal's bullet above now read: 1.22 × a hand inside the sill, 0.96 to 1.05 × from 1 m in; 0.0 % and 5.6 % left in the dice.
  `openings-check` both stands, `shaderrows-check`, all 69 shader texts compile, `tsc` both. The frozen stand has no openings' lights: nothing of it
  moves. DRAWN, software adapter, the service door from inside at midday: the same build with `&holedepth=0` and without. With the
  old light the floor beside the sun's patch is a cloud of blotches, in the picture and in the direct light's own view; with the new it is one
  smooth wedge with a soft edge where the fan ends. Between two frames a patch of the fan's side changes by 0.44 of 255 in the mean with the old
  light and by 0.26 with the new. Both pages booted (120 pipelines) and drew with no page or GPU error. NEVER SEEN ON A GPU, and not timed there (a few dozen
  arithmetic operations a portal sample; `kernel-stack.cjs` not run again: the page booted and drew on the software adapter with no error).
- **THE ROWS THE ONE EXPOSURE MODEL REPLACED ARE DELETED, AND ITS OWN ROW WITH THEM** (11-04; `composite.wgsl` and `compact/composite.wgsl
  lookShadeGain`, their caller, `engine.ts`, `ui/lookpanel.ts`; no switch: nothing a page draws with its rows as it opens was touched). The
  maintainer chose the model over mending the rows it replaced (10-31), and ruled every special case of the light fixed (11-04): the first of the
  two in the exposure was that two models stood side by side. The older one was reachable only with the One exposure row at 0 (`?look=expo:0`)
  and on a device that could not build the place field. GONE: the tail of `lookShadeGain` (a gain by the place capped at six and nine times, the
  roll-off by a pixel's own light, the sun patch indoors, the level a room is turned about) and its head comments; the caller's two lines under
  it (the day's contrast given back under a roof; the Room contrast turn); the Room contrast row `roomc`, `lookConfig.roomContrast` and
  `frame.gradeCfg.w`'s use (the slot is spare); the eye by Sam's meter, `render/lookeye.ts`, `lookConfig.eye` and the page's word for it;
  `lookeye-check` and `lookroom-check`; `frame.shadeCfg.z` (spare, 0); and THE ONE EXPOSURE ROW ITSELF (`expo`, `lookConfig.expo`): with nothing
  behind it, the row at 0 drew exactly what the Eyes adjust to shade row at 0 draws, the hour's one exposure with no place brought up, so
  `shade:0` is that word now and `expo` on an address is ignored like any word the page does not know (`roomc` too). `lookShadeGain` is the
  model's two reads and returns the gain alone: the field's answer, or where the field has none or two (a wall's own cell) the cache's read on
  this side of the wall. A DEVICE THAT CANNOT BUILD THE FIELD keeps the model on the cache's own read at the pixel, the eyes taken to stand in
  the open: what every page shows for the frames before its field is live, and what a device whose field pipelines came back invalid already
  showed (`engine.ts placeDead`: the constructor is asked once and no more; before, the row was put down and the older rows ran). The engine's
  `lookEye` is `{ snap }` alone (the stills tools and `game/airSim.ts` set it). STILL TWO, and not this step's (BOTH GONE the same day: A PIXEL'S PLACE IS READ OFF THE FLOOR PLAN, below): `lookPlaceHere`'s five reads
  behind `?footing=0`; the wall cell's side by what the cache holds where the plan does not say (the floor plan's silent cells, 5.8 % of 48,877
  wall cells over 60 levels with doors shut, most of them on four levels); the corrections on the model named under ONE EXPOSURE AT EVERY HOUR.
  CHECKS: `lookexpo-check` (the kernel's text in both writings, that nothing of the rows is left in either, that the model has no row),
  `lookgrade-check` (its last line is the frame's numbers and the model's own toning line), `shaderrows-check` (the compact twin took the same
  change; fixture stamped), all 68 shader texts compile, both typechecks, `bench-check`; the checks that read the composite and are red at the
  base are red on the same lines (`scenetweak`, `snowglow`, `snowsparkle`, `round-check`, `shaderinline`: the composite's kernels read 1072 KB
  with the every-hour lines and 1070 after this, against a budget now 1030, the Room contrast row's 1.5 KB taken back out of both rows).
  MEASURED: on the software adapter the look page with its rows as it opens draws the pictures it drew, byte for byte: the block at
  night and at dusk and the white roof at midday (two boots each, this tree against the one before it: 0 of 2,304,000 bytes), and a grey day on
  the tree one review before this one. NOBODY HAS SEEN: any of it on a GPU (the model's own lines are untouched, so the default page should not need
  it); a device that cannot build the field, which is drawn differently than before; the wall cell's side on a native driver's compile of the
  new kernel text.
- **PAST THE LEVEL'S EDGE IS BLACK, RULED TWICE** (10-15, the maintainer: "we should just have black outside the edges of the map, not extend";
  11-04: asked again with the other answer built for him to look at, he kept black). At midday the edge is a black wall near where a team
  starts, a quarter to a half of some opening frames; that is known and is the ruling. WHAT WAS TRIED AND TAKEN OUT (11-04, in and out within
  the hour, never in the build): the level's own ground slab laid once more, 160 m wider, as a box no ray meets, following the slab's look at every pack; and
  with it a rule that let a level's maps (the walked ways, the floor's wear, the place field) go past the map's edge instead of clamping to
  the rim's last cell. On the software adapter it took the depot's midday opening from 0.37 of the frame black to 0.09 with no line at the
  rim. Do not propose it again; a level whose world should end at a wall, a drop or water is a matter for that level's own file.
- **LIGHT KEEPS ITS ORDER WHERE TWO PLACES MEET** (11-04; the look panel's row `order` "Light in order", live, 1 on the look's page, 0 = as it was to
  the bit; `frame.placeCfg.x`, a new member, the last of the frame's head; `place_common.wgsl lookGainsOwn`, called by `composite.wgsl` and
  `compact/composite.wgsl lookExpoGain`; the place field's SECOND BLOCK, `render/placeField.ts PLACE_NEAR`, `place_field.wgsl hand`). The
  maintainer, of the white roof at midday on his GPU: "is there some nonphysical stuff going on here with doors? door looks like it's lit even though
  the ground immediately in front of it is in shadow", then "is there a way we can make the exposure stuff more general and not special case doors?
  this should hold for holes from wall charges too right?", "or even areas with very different brightnesses outdoors". CAUSE, read from the code:
  the One exposure row gives a place ONE gain from the light on its floor, and a dark place's is large (9 stops under the yard is shown 3 under).
  What stands at its mouth in the light of the place beside it (an open leaf, a sill, the reveal of a hole a charge blew) got the dark place's
  whole gain and was drawn brighter than the lighter place it is lit by. Only sunlit ground's brightness stopped it (`lookRoll`). His "odd blocky
  artifact near doors when they open and close" is likely the same: a leaf's place is read 30 cm off its face on 40 cm cells and flips between
  yard and room as it swings. THE RULE NAMES NO OPENING: a pixel that holds more light than its place is shown as a place holding its own light
  would be, AS FAR AS THE BRIGHTEST PLACE A STRIDE AWAY HOLDS, and as far as the light of the place the eyes are in (`lookEyeRs`: with Sam in the
  room nothing moves). THE BRIGHTEST PLACE A STRIDE AWAY is a second number a cell of the place field, solved in the same sweep: the most of the
  cell's own level and what its four neighbours hand it, each `PLACE.near` (half a stop) dimmer for the cell of floor between and `PLACE.nearDoor` (an
  eighth) across a doorway's own cells, never through a wall or a pane. So the first cell inside a door holds the yard's level all but whole, a room
  five stops under its yard is its own again four metres in, and DEEP IN A ROOM EVERY PIXEL IS TO THE BIT AS IT WAS. EVERY STEP LOSES SOMETHING, a
  doorway's too: with none there, a doorway's two cells kept the brightest answer either ever held (a yard gone dim), a review's find. THE FALL-OFF IS
  SLOWER THAN A DOORWAY'S LIGHT FALLS INTO A ROOM: at a stop a cell the gain rose inward faster than the light fell, and a software draw had a strip of
  floor a stride inside the door brighter than the threshold. In a wall's own cell with no side known the bound is of the cell the pixel's level was taken from, never
  of the other side's (a review asked for no bound there at all, and a draw showed that undid the leaf: a leaf hanging open IS such a cell). Where the field is smooth (the open air) the two
  numbers hardly differ and nothing moves: the rule acts where the gain steps, which is where order was lost. THE MEASURE: the pixel's own light is
  what is drawn whole (the sun's, a lamp's, an opening's) plus the bounce over the scale it is drawn by (`indirectScale`, 0.35): the field's.
  By the model's arithmetic (`lookexpo-check`): a room 30 times under the open air, a leaf a cell and a half in holding 0.4 of the open air's
  light: far end 0.11, leaf 0.65, the shade outside 0.35; the leaf was 3.7, ten times the shade. ON THE SOFTWARE ADAPTER (the white roof at midday,
  its service door open, the row off and on, at a stop a cell): the leaf 0.30 with a white rim to 0.15 beside a sunlit dado of the same paint at
  0.40; the floor inside the door 0.91 to 0.77, the sunlit deck outside 0.77 both times; nothing outside the doorway moved. THE TWICE THAT IS LEFT is not the exposure's: the
  leaf holds LESS light than the shaded ground (0.4 against 1) and is drawn with more, because an opening's light is drawn whole and the sky's ×
  0.35 (the task "Count daylight once indoors" takes that out; with one scale the same leaf lands at two thirds of the shade).
  THROWN OUT ON THE WAY, each by a still or on paper: THE RULE WITH NO BRIGHTEST-PLACE BOUND (every pixel an opening lights holds more than its
  room's level, so a whole daylit room seen from the yard went dark and flat: the hall's wall 0.026 to 0.006; an earlier still had passed it
  because the member sat past the frame's head and never reached the GPU); a cap with no limit at the eyes' light (it flattened the bright half of
  the room Sam stands in); the member at the frame's END (`frame-layout` now holds it in the head); the pixel's light as drawn over the bounce's
  scale (what is drawn whole was counted three times over). NOT REACHED, on purpose: the bound does not pass a pane (a window's light is the
  room's own, so a sill in it keeps the room's gain), and out of doors a sunlit thing in a deep shade is as the field's own smoothing leaves it.
  KNOWN: the bound is read by the field's own bilinear weights, so it is as coarse as
  the field (40 cm); with the Room brightness row over about 1.3 a gain falls faster than the light rises; the air (`volumetrics.wgsl`) keeps
  its place's gain; a leaf's place is still read off its face. THE BUFFER: `PlaceBuf` is 56,016 bytes (was 28,016), still one uniform. CHECK
  `tools/qa/lookexpo-check.ts` (the opening's line, the solver's rule written again on a made-up plan, the texts; `LOOKEXPO_BREAK=1` puts the row
  down and both fail), both stands; `frame-layout`. Software draws of a doorway and a room with the row off and on. Never seen on a GPU.
- **A SMOOTH FINISH CATCHES THE SUN** (11-04; the scene prelude's `SUN_GLINT`, `render/frame.ts sunGlintConfig`, `rtGlints` in the three reflection
  passes `shaders/reflect.wgsl`, `reflect_p5.wgsl`, `compact/reflect_p5.wgsl`; on on the look's page, `?nosunglint` at boot = as it was, a visit
  flag; every other page as it was). The maintainer, of a car's gloss paint beside two pictures of the real thing: "mm doesn't look good. we don't
  need to go full hyperrealistic but we should sell it more". Every lamp's highlight on a smooth finish is added analytically, and the one light the
  loop left out is the brightest: a directional one. The sky a reflection ray comes back with (`common.wgsl skyRadiance`) is a gradient with no sun
  in it. So by day NOTHING smooth showed the sun: no streak along a car's wing, no glint on a pipe or on the lumps of a gloss-painted wall. With the
  word a directional light (THE SUN, AND BY NIGHT THE MOON) goes through the same loop, from its own direction, as wide as its disc, following the
  shading normal (a relief catches it), with rules of its own, all in the function's head: it is a FIFTH CANDIDATE outside the lamps' contest for
  their four shadow rays, with a fifth turn of the one shadow loop (so no kernel gains a call site: `render/kernels.ts`, a call is pasted whole;
  two earlier shapes were thrown out, one that took a lamp's ray on every sunlit pixel and one that pasted 48 KB into each reflection kernel on
  every page); NEVER CREDITED UNSHADOWED (a sun let in at half through a rough lobe's skirt would light a shut room); its ray stops at the
  world's top; it FADES IN over `RT_SUN_CUT` (0.02) to three times that of the light's own irradiance, whatever the exposure, and under the cut
  throws no ray; it is BOUNDED BY ITS OWN LIGHT at `RT_SUN_PEAK` (2, about ten white walls: the lobe alone is 80 at a grazing eye and 130 on steel,
  a pixel-wide line on every round rim); and its lobe is never under `RT_SUN_ALPHA` (0.07, a finish of about 0.26), against sparks off a relief's
  normals under the jitter (the maintainer's midday fireflies). Without the word the flag is a constant false: the lines are the lines they were,
  plus one empty turn. Where to expect it: a face that looks half way between the eye and the sun. From the play camera with the sun behind it
  that is a round thing's flank, a curved panel, a lump's shoulder; a flat floor shows it only when the camera looks toward the sun, and then all
  of it does. By the arithmetic a finish of 0.2 at the mirror is about four white walls and one at 0.45 under half of one. ONLY WHERE THE
  REFLECTION PASS RUNS: a finish under the preset's roughness cut (0.55 ultra, 0.45 high, 0.35 medium, none on low), at that pass's resolution
  and through its denoiser. No game table. KNOWN, LEFT: one ray at the disc's middle, so under cloud its edge in a shadow is harder than the
  shade's; untinted through glass, as a lamp's; with rays traced the denoiser places it at the first ray's hit. CHECK `tools/qa/probes.ts
  sunglint` (family `render`; `SUNGLINT_BREAK=1` must fail), both stands; `shaderrows-check` holds the compact pass's text. Never timed, never
  seen on a GPU, night included: the three numbers are first guesses.
- **NO LAMP'S HIGHLIGHT IS CREDITED UNSEEN** (11-04; the scene prelude's `GLINT_PICK`, `render/frame.ts glintPickConfig`, `rtGlints` in the
  three reflection passes; the look's page only, `&glintpick=1`, OFF unless asked; a const of the shaders, so a reload of the address is the
  A/B). One of the nine special cases of the light the maintainer ruled fixed (11-04). THE RULE AS IT WAS: of the lamps whose highlight
  reaches a smooth pixel the four strongest get a shadow ray and every other is added at HALF strength with no ray ("off the lobe, dim by
  construction"). So a fifth lamp behind a wall glints at half, and a fifth in plain sight is drawn at half. WITH THE WORD the three
  strongest keep their rays, the rest are summed, and the fourth ray goes to ONE lamp picked from the rest by its weight: THE REST'S OWN SUM
  is drawn if that one is seen, through the smoke on its segment, and nothing if it is not. The pick is a reservoir with one number handed
  on, so the lamps are still walked once, the first of the rest is taken without asking the number, and no weight is divided by itself.
  The number is the caller's: the pixel's own lobe offset (seeded by the pixel, the same every frame whatever the jitter does to the point)
  stepped by 0.7548777 each frame (not the golden ratio, whose step nearly repeats every eight frames, the jitter's cycle: a review's model had
  each jitter phase meet almost the same pick for 140 frames), so a still pixel walks through its lamps; no draw is added to the kernel's random stream. NO RAY
  MORE and no call site more (`shaderinline`: the reflection kernels read 743 and 327 KB, as they did). A PIXEL FOUR LAMPS OR FEWER REACH
  DRAWS WHAT IT DREW: the same slots in the same order, the fourth's own highlight as the sum of one. WITH THE REST ALL IN SIGHT AND NO SMOKE BETWEEN every frame
  is the frame before, colours and all, and the old sum whole (through smoke the rest's sum is dimmed by the picked lamp's own segment, so the
  frames differ and their mean is right). WITH SOME HIDDEN the frames differ and their mean is the seen share of the
  rest's light, in the rest's mean colour and not the seen ones' own. KNOWN AND NOT MENDED (a review's models, not seen): the highlights
  are NOT NOISE-FREE with the word, as the pass's comments said of them: the specular denoiser clamps its history into the frame's
  neighbourhood, which is not linear in sparse bright frames, so where one hidden lamp holds most of the rest's weight the seen remainder may
  settle dim and flicker (modelled 5 to 15 % under); the luminance bound `RT_GLINT_MAX` is then on a frame's sample and not on the sum, so
  the mean falls short wherever the three strongest and the WHOLE rest together pass the bound, though their true sum is under it (a model: 22 % short with the true sum at
  62 % of the bound); under rays set to 0 the returned distance is still chosen by the lamps' own weights (the pick keeps its
  own). THE ROUGHNESS LIMIT, the other half of that special case as first listed, WAS ALREADY A FADE: `composite.wgsl` weighs the reflection
  term down over the 0.12 of roughness under the preset's limit (`reflK`), so the pass's own `rough > maxR` return is where that weight is
  already nothing; the limit itself is a preset's budget for rays, and stays. THE SAME HALF CREDIT STANDS ELSEWHERE and is not this step's:
  `composite.wgsl paneLight` (the cracks' light, one ray), `fgather.wgsl glassShade` (the dust glow, two rays) and its twin in
  `gather_split.wgsl`; the particles credit 0.35 past their three strongest; the froxels drop what they do not trace. ITS END: on the
  maintainer's yes the word goes, `GLINT_PICK` with it, and the four slots and the half are deleted from the three texts in the same commit.
  THE CHECK: `probes.ts sunglint` lines 4 and 5, both stands (the word ships down and is wired; one text in the three passes, its lines and
  the caller's number word for word, no `rnd()` or hash in the function; the arithmetic written again in colour: four lamps or fewer in
  every order and with a tie, 165 cases, the same as before and as the truth; six in sight steady and whole; the mean over 4096 frames of
  the shader's own numbers within half a percent of the truth for two hidden and eight mixed in either order, 2.1 % off over 16 frames; the
  half's own error printed: 6.9 % short with six in sight, 8.1 % bright with two hidden; `GLINTPICK_BREAK=1` asks the same of the half and
  fails). DRAWN, software adapter, the library hall at night: without the word the page is 0 of 2,304,000 bytes from the build before it; with
  `&glintpick=1` it boots with all 120 programs and no error, and its opening view has the same mean and the same 95th percentile as without (0.111
  and 0.465): no view was drawn that shows the difference, so the trial's proof is the check's arithmetic. Never seen on a GPU: whether the pick's noise where lamps are partly hidden settles under the denoiser, still or
  in motion, is the page's to show.
- **USE SHOWS ON A KEPT PLACE: `worn`, A STUFF'S OWN WORD** (11-04; `render/lookTex.ts STUFFS`, `shaders/look.wgsl`; the look's page only, under its
  Weather row, 0 = none). The maintainer, of the white roof on his GPU: "directionally nice but still doesn't have enough "personality / cm^2"". His
  pictures' kept walls and floors are not weathered, but every patch says it was made and used: tie holes, a chipped corner, paint knocked off a
  wall's foot, a changed paver. A kept stuff (`fades` absent) had none of it, since the weather's rules skip it. `worn` (0 to 1, absent on steel,
  enamel, carpet) is how hard use shows, and the shader gives it two things BY WHERE IT IS IN THE WORLD, never in the map (a tile's sixteen pavers
  with one chip between them is a chip every two metres): (1) `lookPiece` (was `lookPieceTone`): a piece in four has a history, one of three by a
  die of its own: changed for a new one (a clear step of shade), stained (a soft patch inside the piece), chipped at a corner (3 to 8 cm on the
  slant, dark and rough; metres, so a chip is a chip on a paver and on a block). Less use = fewer such pieces, never another history for the same
  piece. (2) A kept wall that stands on the floor has its coat knocked off its foot in stretches, to a ragged line a finger to a hand high, each
  face's plane its own run. No edge is drawn finer than the pixel. (The dirt in a floor's crease and along its walls was already the detail
  layer's, `detail.wgsl`, on every page: the look adds none.) In the maps themselves: a cast panel has six tie holes 4 cm across (four of 2 cm
  were specks at a pixel of a centimetre and a half). Enamel stays a clean coat: a chip in its map would come round every two metres along a
  painted line. `lookmaps-check` line 11 holds the row (use on kept stuffs only, none on the sixteen older ones, so every
  other level's picture is as it was). Shader texts compile (`wgsl-compile`); NEVER SEEN ON A GPU, and the amounts are a first guess.
- **HOW A LEVEL IS WALKED SHOWS IN ITS SURFACES, AND ONE LAYER WEARS A PIXEL** (11-04; `src/scene/ways.ts levelWays`; `render/wear.ts buildWearMap`'s
  `walked`; `shaders/look.wgsl` USE AND DIRT, `lookWears`, `lookWay`; `detail.wgsl detailSurface`'s `byLook`; the look's page only, `?walked=0` at
  boot = the traffic map as every other page has it). The maintainer: "i think there's a lot of room to add personality to textures. wear, paths in
  the ground, different materials can all tell a story for how people walk through the level, its logic, leading the player for game play". THE MAP
  OF WAYS is one function of what a level already says: four counts on the walk grid's own cells (patrol = its routes; team = every start of every
  scenario to each room and thing it goes for; doors = both faces of every frame and every pair of outer doors, door to door; hint = the author's own by name, the level file's
  `scenarios.walked`, `docs/level-format.md`), each the number of ways over the cell, not summed and not saturated; beside them `paths`, the same
  ways as the lines they were traced along, and `lost`, the ways the search found no path for (a straight line then; no level has one). A name is
  looked for as a door's id first. Pure, no module state; its
  path searches are on tables of its own (`findPathAside`); nothing in `src/game` imports it. Cost: 2 to 7 ms on `block`, 3 on `compound`, under 1
  on the office. THE READER: on the look's page the traffic map's lanes are those paths, traced as a route's legs always were, by `wear.ts WAY` (a route
  1, a team's way 0.45, door to door 0.3, a hint its own weight); a level with no routes had no lane at all, and a raid's yard now has the trodden line from where the team comes in to the doors it
  goes for. Every other page's map is to the byte what it was. WHAT A SURFACE SHOWS OF IT: wild ground is beaten flat and pale along a lane (as it
  was, now with lanes to show); on a floor somebody keeps a lane is a shade darker (a fifth at the busiest before the tone curve, about a tenth on a pale deck in the sun; by the stuff's `worn`) and a piece's history (changed, stained, chipped) is likelier where feet go, asked at the
  piece's own middle so a lane's edge never cuts a history in two; a kept wall's foot is knocked in more stretches beside a lane; paint on the floor
  (a hatched bay, a stencil) goes first where feet go. ONE ANSWER A MARK: the detail layer (`detail.wgsl`, every page) and the look layer both read the
  traffic map and both ran, so a wild floor's band at a wall was double and so was a wild wall's foot. Three marks come of the map and each now has
  one answer on the look's page, by what the surface is. A LANE: on a floor with a finish to wear the detail layer's scuff; on wild ground with none
  the weather's (beaten pale); on a kept floor the dirt feet bring in on top, by the stuff's `worn`. THE BAND AT A WALL'S FOOT: on wild ground the
  weather's; on any other floor the detail layer's dust. THE SKIRTING: on a wild wall the weather's damp foot; on any other the detail layer's
  shoe marks. HOW: the g-buffer asks `lookWears` once a pixel and tells both layers (a cut face and a round's side have the detail layer's wear from
  neither, as before); the detail layer then leaves its two terms, and the look layer CALLS them, `detail.wgsl detailFloorWear` and
  `detailSkirtWear`, one copy of the numbers, on the sample it read itself with what the weather answers for left out. The look layer reads the map
  ONCE a pixel (a floor where it is, a kept wall a stride out from its foot; a pieced floor asks once more at its piece's middle), whoever dresses
  the pixel. So a kept floor and a finished floor are what they were; wild ground and wild walls lose the second copy of their band. The white roof's six ways (the
  stair's landing to the lounge's door, to the service door and to the plant; the service door to the plant; through the service room and the
  lounge) are in its generator, with three named places for their ends (`docs/agents/levels.md`). CHECK
  `bun run tools/qa/probes.ts ways` (family `render`; `WAYS_BREAK=1` fails the reader's line): the map on `block`, the white roof's names, the
  office's patrol, the lane in the traffic map, the wiring by its text. NEVER SEEN ON A GPU, and what a surface shows of a lane is a first guess.
- **NO SURFACE REPEATS AT ANY SIZE: THE DEAL, BAYS, AND A STUFF'S OWN `untiles`** (11-04; `shaders/look.wgsl lookDeal`, `lookPiece`; `render/lookTex.ts
  STUFFS` words `untiles` and `bay`, the prelude's `LOOK_LAY`; the look's page only, under its No repeat row, 0 = the page as it was). The maintainer,
  of the white roof on his GPU: "this is starting to look very good btw! I think the main thing to iterate on is making it look less repetitive at the
  different scales". A map is two metres and a deck is forty, and three sizes repeated: the map itself (every piece of a paver deck came round every
  two metres, grain and all), the field of pieces (one even grid to the walls), and the whole face (forty metres of one shade). ONE ANSWER A SIZE,
  each by where the surface is in the WORLD and none in a map. (1) A STUFF MADE OF PIECES DEALS ITS MAP'S PIECES: the piece of the world at a place is
  drawn from another piece of the map, picked by a die of the world piece's own and of the face it is on (the face's plane FROM THE BOX, one number
  a face: taken from the pixel it could flip a centimetre and speckle the wall); a bond keeps a course's parity. Exact: one
  read, no cross-fade, the caller's own slopes; a piece's edge is a joint in every piece, so nothing shows where two dealt pieces meet. A map may now
  hold pieces that differ. (2) PIECES ARE LAID IN BAYS (`bay`: pieces along and up to a movement joint; pavers six by six, a cast wall in lifts of
  three panels): a bay has a shade its own on top of its pieces', and the joint between two bays is wider and darker than a piece's (under the row,
  as the deal is). A bonded stuff
  has no bay along (its courses are half a piece out of step). (3) A KEPT PLACE DRIFTS: three in a hundred of shade over ten metres. (Bays and the drift go on to twice their strength with the row past 1, for
  the page to try; the deal is on or off.) AND THE SHADER
  NAMES NO STUFF: which stuffs are read twice and cross-faded (the older rule for what has no courses) was four class names in `look.wgsl`; it is the
  row's `untiles` now, and stucco, skim and carpet have it. CHECK `lookmaps-check` line 11: the four older stuffs still untile and no other of the
  sixteen; nothing made of pieces untiles; a bay is of whole pieces; a bonded map has an even count of courses. Shader texts compile. NEVER SEEN ON A
  GPU; a software draw of the white roof's deck with the row off and on is what was looked at.
- **TEN KEPT STUFFS, AND A STUFF SAYS ITS OWN ROW** (11-04; `render/lookTex.ts` stucco, skim, gloss block, board concrete, cast panel, paver, brushed,
  louvre, enamel, carpet; the level `white-roof`, `docs/agents/levels.md`). The maintainer, of a game whose pictures he sent: "it has some very
  tastefully done high frequency detail but overall quite flat", "an antidote to our repeating textures that look generated", and, after trying the
  Paint row down on a GPU, "it all still just looks pretty flat and \"slop\" ... it's all quite intentionally dressed", "i'm referring specifically to
  the texture work". So these are drawn the way such surfaces are MADE: one colour (the box's) with next to nothing added, and everything in the
  relief, the finish and the pieces. Soft lumps of stucco under gloss paint; a rolled skim; painted block in courses; concrete that kept its form
  boards, their grain and its air holes; concrete cast in metre panels with a struck joint, four fixing holes and a little dirt in the joints and
  under their lower lip; half-metre pavers; brushed steel; a louvre; plain gloss enamel; a loop-pile carpet. THREE RULES CAME WITH THEM.
  (1) A MAP HOLDS NOTHING BETWEEN A HAND AND A DOOR WIDE BUT ITS OWN PIECES: soft tone or sheen inside the two-metre tile is seen to repeat at the
  game's size (brushed steel's first smears and the skim's first shiny patches did). (2) A STUFF MADE OF PIECES says so (`unit`: a piece's size, its
  bond, how far a piece's shade may sit from the next's). The map draws the joints at that size (`piece`) and gives the pieces no shade; `look.wgsl
  lookPiece` gives each piece of the WORLD its shade by where it is and which face it is on, so a wall of eighty panels is not a tile's eight
  over and over and two walls that look the same way are not one pattern twice. (3) WHAT A STUFF IS TO THE SHADER IS ITS ROW, by name, in
  `lookTex.ts STUFFS` (a name without a row does not compile): `rough`, `relief`, `built` (laid in place: the map lies by the world, so courses run on
  from one box of a wall to the next), `fades` (how far the weather blotches it; absent = a place somebody keeps: no blotch, no drift of hue, none of
  the traffic's dirt), `soaks` (a damp foot, a bleached head), `unit`. The shader's lists of stuffs by name for these are gone (`kept`, the classes
  laid in a box's frame); the sixteen older stuffs' rows give the statements they had (× 1, or the same 0.3). Lumps and joints are drawn larger than
  life: at the game's zoom a pixel is about 1.5 cm, and relief finer than 3 cm reads only as finish. A level's door row (`scene/look.ts LOOK_DOORS`)
  may say what its leaves are made of (`stuff`). NOT DONE: a piece is not fitted to a box's face (the level stands on a half-metre grid instead; a
  40 cm block does not divide it), the older `block` keeps its tile's own shades. (By day no smooth finish had a sun glint when this was
  written: "A SMOOTH FINISH CATCHES THE SUN" above.) CHECK `tools/qa/lookmaps-check.ts` (every layer: no seam, as bright
  as its paint, the word's room; the rows: line 11). The tiles were judged as software pictures under one raking light and the level as software
  stills. Never seen on a GPU.
- **A WET COAT DOES NOT MIRROR A LIGHT IT CANNOT SEE** (11-01; `composite.wgsl wetCoatSpec`, `WET_HIDDEN` 8). The maintainer: "still lots of fireflies/sparkles
  in that room", "some on the shadowed side too", "lots of temporal instability on the guy's back too". The sheen scaled every light's lobe by its class's shadowed
  over unshadowed light; in a shut room by day the hidden sun is the whole unshadowed sum, so its lobe was drawn at the bulb's strength. A light whose
  unshadowed light is more than eight times the class's shadowed light is not there, faded out from four times so a shadow's soft edge is not a line where the
  glint ends. ON THE LOOK PAGE BY DAY ONLY, under the Steady bounce row (`frame.lampCfg.z`; `?look=steady:0` is the A/B, and every other page and every night
  draws the highlight as it did). A threshold, not a general rule: the general one is a shadowed evaluation of each light, which the composite does not have.
  The coat's MIRROR of the room (its sheen) reads the cache like any face under the wall sides row (`diceFaceBy`), where it asked BOUNCE WALLS' box rays.
- **ROOM BRIGHTNESS** (11-01; the look page's row `roomb`, live, 1 = as it is; `frame.lampCfg.w`, `composite.wgsl lookExpoGain`). The maintainer: "exposure does
  seem a bit too bright when indoors but we can tweak it". A power on a place's gain where the gain is over 1: 0.9 is about half a stop off a room with windows
  and a stop off a shut one. A knob for him to find the number with; the number then belongs in the exposure model, and the row goes.
- **THE LIGHT DEBUG VIEWS SHOW A ROOM BY DAY** (11-01; `composite.wgsl`, views 3, 4, 7, 8). The maintainer: "i went through all the buffers in the debug view and
  couldn't see what was causing the light leak really". The views drew scene light at the yard's exposure, so a shut room was black in every one of them by day.
  On the look page they are now shown by each place's own gain, as the picture is.
- **THE SOFTWARE ADAPTER REFUSES A BIG KERNEL WITHOUT SAYING SO: A CLEAN BOOT, THEN THE DEVICE LOST ON THE FIRST FRAME** (11-01; `node tools/qa/kernel-stack.cjs`).
  All 119 pipelines made and no error, then `A valid external Instance reference no longer exists`: the GPU process had died (exit 11) the first time one pass was
  dispatched. THE CAUSE, read in a debugger: SwiftShader pastes every call of a kernel into one routine, and its compiler (Subzero) gives the routine up when its
  stack frame passes 524,288 bytes (`Ice::Cfg::setError("Stack size limit exceeded")` from `TargetX8664::addProlog`); the pipeline is still handed back, and the
  first dispatch calls a null routine (`rr::Nucleus::invokeCoroutineBegin`, a read at address 0x10). THE FINAL GATHER LIVES AT THAT LIMIT: `fgatherS / main` on the
  look page is 99.2 % of it (519,976 bytes; 521,176 at the commit before the wall sides), so a few lines added to anything it calls are enough, and what was added
  need not be at fault: with the cut boxes compiled out the same kernel is 465,624 (they cost 60 KB of frame: `isectCut` is pasted 70 times). It is a limit of
  the adapter the cloud tests on and no GPU's, but the frame grows with the same pasted size a Windows shader compiler pays for. WHAT TO DO: after a change to
  anything the gather calls, `node tools/qa/kernel-stack.cjs` (40 s: the page's own text of one kernel, made and dispatched alone on zeros, with the compiler's
  frame and limit read at its own check when gdb is on the machine; `--text` tries a cut before it is made; `--list`, `--kernel a,b`). Its texts are THE PAGE'S:
  `wgsl-dump.ts`'s stand-in leaves `GROUND_RAYS`, `WX_KINDS`, `GLASS_BREAK` and `CUT_BOXES` down and the same kernel fits there with room. What costs frame: every
  pasted call (a helper called at eleven places is eleven copies of everything it calls), a second texture read at each of them, a loop's own variables. What
  bought it back here: one look at the plan a read where there were four, the columns from a table, the taps in a loop. Also measured: `compositeS / cs` on the
  look page, 277,096 (52.9 %). ULTRA'S RESAMPLING GATHER HAS NEVER RUN ON THE SOFTWARE ADAPTER: `fgatherGiS / mainGi` is over the limit at the commit before
  this work (109.8 %) and after it (111.2 %), so a software page at Ultra loses its device on the first frame and no sheet or still was ever drawn there. Not
  measured: the other kernels' frames (the page draws at High, so each is under), and nothing here says how a GPU's compiler takes the same text.
- **THE DAY PATH'S SPECIAL CASES, COUNTED** (11-01; the maintainer: "overall i worry that we're finding these cases one by one and not fixing them systematically", and
  "i'd like us to try to remove as many "hacks" or special cases as possible -- I think having strong general systems is the core of this project"). Two faults
  made nearly every day defect so far, and each wants ONE rule every pass obeys. (1) LIGHT THROUGH A WALL: now one rule, the floor plan (the bullet above), in
  place of four guards. (2) A NUMBER MADE FOR A NIGHT: a bound or a threshold in absolute light, right when a lit room is near 1 and meaningless when the lamp
  is a few thousandths of the sun. That one is NOT yet one rule. What still answers it case by case: `common.wgsl lampFloorK` (the gather's firefly floor held
  at the day's lamp share; read in the direct pass, sharp direct, ReSTIR, both gathers and the composite); `lighting_common.wgsl dayBoundK` (reflection and
  highlight bounds from the cache's own light at the point, `DAY_BOUND_SHARE` 0.25; both reflection passes); `composite.wgsl WET_HIDDEN` (a wet
  coat's highlight of a light the point cannot see, bounded at 8 times what its class delivered). THE ONE RULE THEY WANT: every such bound is a multiple of the
  light level of the PLACE the point is in, and the exposure already makes that number a cell (`render/placeField.ts`, the One exposure row): the three could
  read it and their own constants go. Not built. ALSO STILL SPECIAL: a wall built askew between two places knows no line, so its cells read the cache unguarded and their place for the exposure is still
  guessed from their neighbours (25 of the pass's 641 wall cells, the lorry's; `composite.wgsl lookPlaceField`'s last lines); a roofed lamp always asks its shadow ray (the rain leak's mend,
  `reach` masks are a floor plan); Room contrast and Room brightness are taste rows over the place's gain; the older stack of day corrections is still in the
  modules behind `?look=expo:0` for the A/B (DELETED 11-04, the row and the Room contrast row with them: THE ROWS THE ONE EXPOSURE MODEL REPLACED ARE DELETED); and the four wall guards' text is still in the modules because the build's page reads them: they are deleted when
  the row goes to the build, not before.
- **A PLACE SEEN FROM ANOTHER IS THE PICTURE IT IS TO EYES IN IT, SHOWN DARKER** (11-02; look page only, part of the One exposure row `expo`, so
  `?look=expo:0` is the rows before it (was: they and the row are gone since 11-04); the maintainer on his GPU at midday on the pass, his eyes in the yard: the men in the bulb-lit store white at their
  edges on a near-black floor, "weirdly not able to see where the glow is coming from here by flipping through the debug views", "it subsides a lot once i
  shoot out the one light in the room"; and "weird that smoke looks black indoors but white outside"). Both were one fault: the exposure was a gain on the
  DIFFUSE light of a place, and every other kind of light a place sends the eye was handled apart, or not at all. A room seen from the yard had its floor
  taken down twelve to thirty times while a bulb's highlight on a man's kit kept the whole of its strength (the shoulder only rolled it off above sunlit
  ground), so highlights read as a glow from nowhere; and smoke's light took no gain at all, so in a room shown a few hundred times brighter the smoke sat
  hundreds of times under the floor behind it, black. THE ONE RULE (`shaders/place_common.wgsl`, one text pasted in front of the composite and of every
  volumetric march by `passes.ts`): a place has three gains from the light it holds (`lookGains`: the gain it is shown by now; the gain it is shown by to eyes
  that are in it, never under the first; the gain the open air is shown by now, never over the first), and ANY light of the place goes through `lookShowBy`:
  exposed as for eyes in the place, rolled off there above sunlit ground (`lookRoll`, as it was), then taken down by the ratio of the first two gains, and
  never darker than the open air shows the same light (a sun patch through a door of a room shown dark is as bright as the ground it came in over). To eyes
  in the place it is the gain and the shoulder exactly as they were. What goes through it: the diffuse, the ray-traced mirrors, the gloss, glass's
  highlight, the men's finish (`composite.wgsl lookShow`, both twins), and the smoke march's light step by step (`volumetrics.wgsl airShow`, both twins: the
  air's place is the place field's cell under the step, `placeAir`; in a wall's cell the brightest free cell within two; over the ceiling's height the open
  air). The volumetric programs bind the place field at group 1 binding 5. NOT YET THROUGH IT: a lamp's own haze and marched beams and the dust volume
  (they still take no gain), a glowing face, a spark's own light (lit particles go through it since 11-04: A LIT PARTICLE IS SHOWN BY ITS PLACE).
  Debug view 10 (specular) is where the men's highlight shows. Check: `bun run
  tools/qa/lookexpo-check.ts` (its CPU mirror of the three gains; a store 4,000 times under the yard seen from the yard: a glint keeps its ratio to the
  floor as from inside and is never over 0.20 of sunlit ground, where a glint a hundred times the floor used to be handed on at 1.23 of it beside a floor at
  0.012; the volumetric texts and the wiring are read; `LOOKEXPO_BREAK=1` fails). Seen on the software adapter only: one boot of the look page (119 programs,
  none failed, no GPU error) and stills of the pass's store with two men under its bulb and a can of smoke, from the yard and from inside, at midday and at
  night, beside the same two midday stills of the tree before. From the yard the glints on the men's hands and legs are down with the floor as it was (the
  brightest pixel 224 of 255 before, 172 after; the middle pixel 12 both times) and the smoke is a pale puff lit like the room; from inside and at night
  the two trees draw the same picture. NEVER SEEN ON A GPU. Cost not measured: the smoke march works a place's three gains (about fifteen `pow`) once a cell
  of the place field a ray, not once a step (`airShow` keeps the last cell's answer). STILL OPEN from the same notes:
  blotches on a room's floor in overcast, and a room with its doors open at midday (pale floor with the eyes in it, sun patches at the shoulder's white).
- **THE SMOKE LAB** (11-01; `/smokelab` on the dev server = the look page on the lab's own floor with `&smokelab`; `src/host/smokelab.ts`, on the fire lab's
  pattern; the floor is `docs/agents/levels.md`, THE SMOKE LAB'S FLOOR). The
  maintainer: "could we make a separate smoke lab that's more suited to playing with it? would love to be able to visualize the bricks, the curls (could we turn
  these on and off in the debug panel?), tweak the parameters etc." Its system in the panel: what the pool holds, a can at your feet or at the cursor, all of it
  cleared, the field held still; TWO PICTURES of the solver, both the composite's (`smokeLabOver`) off one word the solver hands the readers (`SmokeSystem.show` →
  `SP.show` → `smoke[0].show`): the BRICKS as wire (a walk of the page texture along the pixel's ray, the coarse lattice's bricks blue, the fine one's amber,
  hidden behind what is solid) and the SPIN (while it is up `publish` leaves density × |curl| ÷ the row's full-colour number in a coarse texel's tint channel, and
  the smoke is drawn again blue to red by it; the dust twin is held off meanwhile, since that channel is the dust's); each kind of curl on a box of its own
  (vorticity confinement, the source's gusts, the look row `stir`); every number of `smoke.params` and of the pool as sliders with the build's numbers back by a
  button and the numbers copied out as a line. It writes the tables the solver already reads; nothing is made without the word, and the two pictures are in the
  composite only on a page opened with it (`render/frame.ts smokeLabConfig` → the prelude's `const SMOKE_LAB`, read in `main.ts` before the engine is made): every
  other page's composite is the program it was. Fine-lattice gas is not coloured by spin. `/smokelab?levelFile=pass.json&roomclear&squad4` is the lab on
  another level (a `levelFile` on the address wins). Seen on the software adapter only: never on a GPU.
- **A ROUND THROUGH SMOKE IS ONE LINE** (11-02; the maintainer in the smoke lab: "also still can't see bullets leaving trails through the somke"). A round
  used to be a row of soft shoves along its path (`combat.ts`: every 0.28 m, 7 cm across = a cell and a bit of the 5.5 cm lattice, a blend of 0.4 toward 5.5 m/s
  for ONE step, at most twenty a frame with the bodies' shoves counted in), which moved nothing anyone could see. Now the solver has a LINE emitter
  (`smoke.wgsl`, mode 3: a stretch of half length `tall` along `dir`) and `SmokeSystem.round(from, dir, len)` sends each straight leg of a round's path as a
  `push` with `line`. THE DISTANCE: the shove, the swelling and the feed all measure an emitter through `emR2`, which for a line is the distance to the stretch
  (round ends). THE BRICKS: the bricks an emitter wakes (`markEmit`) and the emitters a brick lists both ask `emInBox`, which for a line takes the point of the
  stretch nearest the brick's middle, so a line reaches the bricks along it and not a sphere of its length; `emHalf` gives `markEmit` the stretch's own box of
  bricks to walk. THE CUT (`smoke.ts push`): a leg is cut to each lattice's loose bounds and three radii past them, overlapping stretches of one lattice are
  one, two plumes apart stay two (the clear air between is not sent), and a stretch goes in pieces of `LINE_MOST` 4 m at most. THE ENDS: the line stands two
  radii inside each end of the leg, so its round ends do not shove the air behind what the round hit or came out of; a round that flies within about a radius
  ALONG a thin partition still stirs the far side, as a walking body's shove does. THE LAW: the air along the path is brought UP to `wake.speed` 9 m/s along it,
  `u += dir * max(speed * clamp(exp(-r²) * push, 0, 1) - dot(u, dir), 0)`: no frame length in it (so it is the same at 30 and at 144 frames a second), air
  already that fast is left alone, nothing is slowed or turned, and a second round down the same line adds nothing while the first's jet stands. `wake.push` 4
  = how flat the jet's top is (full speed out to 1.18 of `wake.radius` 0.12 m). A long thin jet is nearly free of divergence, so the pressure step leaves it
  standing: it drags smoke after the round out of the far side of a plume, pulls clear air in at the near side, and its edges roll up. LOOK PAGE ONLY
  (`combat.ts` asks under `lookConfig.on` with the look panel's Stirred smoke row up; anywhere else, with that row at 0, and with the lab's row "how fast a
  round drags the air" at 0, the dots as they were), so the build's page is as it was. No frozen row: a headless run has no smoke system, and nothing here is
  the simulation's state. It does change what a guard sees through smoke on the look page (the same field is the sight line). Check:
  `bun run tools/qa/smokeline-check.ts` (the cut run as it stands over a system that is only its lists, the distance and the brick reach mirrored, the brick
  reach against 400 lines and bricks at random, the texts read; `SMOKELINE_BREAK=1` measures a line as a point, sends a leg uncut and wakes bricks by its
  middle, and fails). SEEN, software adapter only, the lab's can at dusk, the solver stepped at 1/30 s: a round through a young plume at knee
  height throws a tongue of smoke about 0.6 m out of the far side within 1/15 s and a broad lobe by a third of a second; after a second it has merged with the
  plume. An earlier boot (the law then a blend, equal to this one in still air at that step): a burst of five pushed the plume's bound out by 3.5 m, and a round
  through the middle or just under the top of a 2 m plume showed nothing along its path from above, only the puff at its far side. No GPU error on any frame.
  NEVER SEEN ON A GPU, and never with
  a gun fired in play (the stills call `round` by hand): the path from `combat.ts` to the solver is under the text check alone. NOT DONE: a round does not part
  the smoke up to the plume's top, so from the play camera a round through the middle of a tall plume shows at its far side and not along its path.
- **HOW SMOKE IS DRAWN, AS ROWS: the smoke lab's "How it's drawn" and "Streaks the air can fold"** (11-01; `src/host/smokelab.ts`; `Engine.smokeDraw` →
  `FrameParams.smokeDraw` → `frame.sideCfg.y / .z` (two spare words of the look prototype's block), read in `vol_common.wgsl` under that heading;
  `passes.smokeDiffuseKeep` → `post.smokeKeep`;
  `SmokeSystem.streak`). The maintainer, of the lab at dusk: "we *have* a lot of interesting high frequency detail in the sim now but it ends up looking flat
  in the final image. 1. is this a lighting problem? 2. it also seems like we have some screen space smoothing on the large smoke volumes", then "let's make
  it so we can play with the rendering stuff in a section", and of rounds pushing smoke, "it ends up not being legible". Read from the code: (1) yes. The
  soft light on a plume is ONE ambient the froxels hand the march with no smoke in its way, so with little direct light a plume is one even tone; the
  self-shadow is four taps 0.18 m apart, coarser than the curls. (2) yes, three: the march is at half size and scaled up by a 3 × 3 kernel, TAAU averages
  frames, and `post.wgsl`'s SMOKE DIFFUSION mixes the whole pixel, the plume's own light included, toward the frame at a quarter and an eighth of the size
  by the smoke looked through (up to 0.96 past optical depth 3). NOTHING IS DECIDED: every new row starts at the picture as it was (both frame words 0,
  `smokeKeep` 0, no tint in a can's smoke, so the plain volumetric program runs), and no other page writes them. THE NEW ROWS: **creases go dark**
  (`smokeCreaseIn`: the ambient at a sample × exp(−the smoke crowding round it), four taps on a tetrahedron 0.2 m out, asked every `volShadowEvery` steps,
  in both lit marches: the classic one that ultra and high run and `smokeMarchJ`, where with `smokeMarchLights` 0 the lamps are in that light too and are
  dimmed with it; the compact twin has the same lines); **how dark the far side of a fold is**
  (`SMOKE_SELF_K` as a number, 6 = as it was) and **how far a fold's shadow looks** (0.72 m = as it was and the row's top: the march gathers the smoke within 0.75 m for those taps), both self-shadows alike, the dust twin's and the
  froxels' too; **the smoke's own light stays sharp** (the volumetric pass now writes the smoke's own in-scatter luminance to `volReact.b`, always; post
  reads it five taps wide and lets the diffusion go by its share of the blurred pixel: a sign through thin smoke still blurs, a thick lit plume does
  not). THE REST of that section are the debugger's old rows gathered (multiple scattering, the diffusion and where it starts, TAAU's smoke reactivity, lamps
  per step, the march step, quarter size, the upsample's depth guide). STREAKS: "yeah 1 makes sense" (of five ideas: a can that burns unevenly, so the
  flow has something to fold). **dark bands** = a can's emitter takes a tint that comes and goes with its age (`pack`: the K word every coarse cell
  already carries for dust, canister share still 0, so the published density and every sight line are what they were); the air carries it, the dust twin
  draws it, and `sideCfg.z` bit 0 (up while the bound system's `streak.bands` is, `SmokeBinding.streaks`) tells `dust_look.wgsl` a tint is a streak: no edge thinning, no fray, no billow noise, so the field drawn is the
  solver's own. Said plainly: while that row is up real dust is drawn as a plain tint too, because a texel does not say whose tint it is; the build
  would want the canister share published. A band's times come from the can's age and where it lies. With `?nodustlook` no tint is kept and the row does
  nothing. **puffs** = the can's output × a slow wave, the one row that moves the smoke itself. Costs: creases 4 texel
  reads a refresh; bands run the kind transport in a can's bricks (it is skipped for plain canister smoke). CHECKED: `wgsl-compile` (68 texts),
  `shaderrows-check` twins re-stamped (the compact march has the same two lines), `dust-look-check`, both typechecks, and one boot of `/smokelab` on the
  software adapter with every row up (below). NEVER SEEN ON A GPU, and whether any of it reads well is his to say: the stills are 640 px on a software
  adapter. Not built: idea 2 (a round carves a tube: the fine lattice in its wake), smoke reprojected by the solver's own velocity instead of averaged.
  WHAT THE ROWS DO TOGETHER (11-02, the maintainer's lab at dusk with `densityScale` 3: "curious why there's so much orange in the smoke, also seems
  nonphysical" and "horizontal striations that seem aligned with the grid"; one software boot, each row alone). ORANGE = "creases go dark" turned up. The row
  dims the sky's soft light on smoke by an absolute optical depth 0.2 m out, so at a density scale of 3 nearly the whole plume counts as a crease: the
  blue-violet sky light goes and what is left is the dusk sun's direct light, which is orange. At 1.5 alone the plume is orange with no bands. FINE BANDS =
  the two fold shadow rows turned up (15 and 0.3 m): the four shadow taps then stand 7.5 cm apart, closer than the solver's 5.5 cm cells can resolve
  smoothly, and the lattice shows as lines; at the defaults (6, 0.72 m) the same plume is smooth. The can's own uneven feed is NOT the cause (with it taken
  out the lattice pattern is plainer). SIGNS STAY CRISP behind smoke only while "smoke blurs what is behind it" is at 0 (the page opens with it at 1). The
  lab's "copy them" carries every drawing row now (`draw`, `streak`, `wake` beside the solver's numbers), so a pasted set says which of these were up. Not
  done, and not without his word: the creases row measured against the plume's own thickness, the shadow taps jittered (it would grain the calm look).
- **THE SMOKE LAB'S "A LAMP BEHIND SMOKE GLOWS THROUGH IT" ROW** (11-02; `Engine.smokeDraw.ahead`, bits 24‥31 of `frame.sideCfg.y`; `vol_common.wgsl
  smokeAheadW` / `smokeAheadLobe`, the three marches of `volumetrics.wgsl` and its compact twin; 0 on every page = the picture as it was, a lab row only).
  Smoke scatters most of a lamp's light on along the line it came by. The march's one lobe (g 0.35) is wide; under the row a NARROW one (g 0.8) is ADDED
  for each light a step lights itself, through half the smoke both ways, as the many-scatter octave is: so looking at a lamp through thin smoke the smoke
  before it glows with the lamp's colour, where blurring what is behind only smears the lamp. A lamp is a disc, not a point: the lobe is asked at the
  disc's nearest edge to the line of sight and capped at 1 ∕ the disc's solid angle (a big panel close by is not a point's spike). The same lobe weighs
  the froxel march's choice of its K strongest lights (`smokeMarchJ`; the two older per-pixel marches keep their order by bound), so the lamp one looks
  toward is not the one dropped there. A MINOR light, which a step credits without a ray of its own, gets none of it: the lobe's peak is sixty times the
  wide lobe's sideways value, and an untraced lamp would shine through a wall. Software smoke lab at dusk, a can before the blue
  vending front (after the `face` class below, without which the front's light was nothing): the plume crossing the front goes from 162, 135, 149 to
  167, 141, 158 at 0.5 and 172, 146, 164 at 1 (of 255), nothing else moves. A small gain there; whether it is worth a row is the maintainer's call on
  his GPU. NOT SEEN: a GPU, a lamp seen low through a thin edge (where it should count most), a flashlight aimed at the camera through smoke.
- **THE GLOW THROUGH SMOKE CROSSES THE SMOKE AS ITS FORWARD LIGHT DOES** (11-03; `vol_common.wgsl smokeAheadT`, used by the three marches of
  `volumetrics.wgsl` and its compact twin; nothing with the row above at 0, so the plain build is as it was). The maintainer, of a sign behind a puff:
  "i'm still not exactly sure why the vending machine textures coming through the smoke so clearly looks so nonphysical to me". MEASURED on a
  capture of that view (software adapter): the picture is the wall times the smoke's transmittance plus the smoke's own light, and that held to the
  digit, so nothing skipped the smoke. The reason it reads wrong is the ratio: the sign's face gives 2 to 2.7, the lit smoke in front of it 0.08 to
  0.2, so through an optical depth of 4.2 (1.5 % let through) a white bar of the sign still adds a third to the pixel, and it adds it SHARP. Real
  smoke that thick in front of a sign is lit up by the sign: the light it scatters forward hides the pattern behind a soft glow of the sign's colour.
  The row "a lamp behind smoke glows through it" was that term, but it was cut down by the same thickness as light scattered sideways, on the way in
  (the lamp to the sample) and on the way out (the sample to the eye), so where the smoke was thick enough to matter it gave almost nothing. Now the
  narrow lobe crosses smoke as (1 - g) as thick, g the lobe's own 0.8: light scattered nearly straight on is nearly unscattered light, the textbook
  reduction for a forward-peaked phase function. Both legs and the step's own share use it. WHAT A PICTURE SHOWS (the smoke lab at dusk on the software adapter, his rows; the row at 0 and at 0.65 INSIDE ONE BOOT, on
  the tree before and the tree after, because the plume a can grows differs from boot to boot by as much as the change, which a first pair of
  stills from two boots did not allow for): at 0 the sign's pattern and its lower edge stand sharp through the smoke, on both trees alike (within
  1 of 255 where the plumes agree). The row at 0.65 lifted the smoke over the red face by 18 before and 30 after, over the blue face by 7 and 16,
  the plume's middle by 4 and 13; over the whole picture a mean of 3.0 before and 4.4 after. So the old term was NOT nothing, as this bullet first
  said: it gave about two thirds of what the new one gives near the sign and a third of it deep in the plume. After, the glow hides the sign's
  lower edge and its small bars; before, both still showed. IT CHANGES THE PICTURE UNDER HIS TUNED ROW, so the row may want another number now.
  Never seen on a GPU. A MARCH STOPS IN THICK SMOKE ONLY WHEN THE GLOW IS DONE
  TOO (`smokeMarchDone`, the three marches' early-outs): with a lamp straight behind, every step of the plume adds about the same glow, so stopping
  where the picture's transmittance fell under 2 to 4 % dropped the far part of a deep plume, at a depth dithered a pixel. With the row on, a march
  now goes on until the glow's own weight is under the cut, which in practice is the plume's far side. THAT COSTS STEPS in thick smoke on the look
  page (a thin plume was always marched whole): NOT MEASURED ON A GPU. On the software adapter, the smoke lab's close-up with smoke over most of the
  picture, inside one boot: a whole frame took 82 to 90 s with the row at 0 and 83 to 92 s at 0.65, so nothing a software frame can show. With the row at 0 the test is the old one and `smokeAheadT` returns before its `pow`.
- **THE LOOK PAGE DRAWS SMOKE BY THE LAB'S TUNED TABLE** (11-03; `render/smokeLook.ts SMOKE_LOOK`, put on by `main.ts` on the look page only,
  `?smokelook=0` there = smoke as the build draws it; a benchmark asked for on the look page carries the word). The maintainer handed the smoke lab's
  numbers back ("latest settings we should make these default"). Of that paste, the solver's, the pool's, the can's, the stir's, the streaks' and the
  wake's numbers WERE the build's own already, so the smoke itself and what anyone sees through it did not move. Nine rows of the PICTURE differ and are
  the table: creases 0.6, the far side of a fold 10.5, a fold's shadow reach 0.58 m, a lamp behind smoke glows through it 0.65, the smoke's own light
  stays sharp 0, light glows through the body 0.15, smoke blurs what is behind it 0 (so the post blur is not run), its start 0, a moving plume leans on
  the new frame 1, and the march's step 1 cell AS TUNED ON HIGH. The step is the quality preset's to write (`ui/quality.ts putSmoke`, its one writer,
  asks `smokeLookStep`): high, medium and ultra go from 1.5 to 1, low and very low from 2 to 1.33, never under a cell, so the ladder's cheaper presets
  keep their longer step. Lamps that light each step, a quarter size and free scaling stay the preset's (his paste had high's). Every other page draws
  smoke as it did: the plain build's picture is not touched. The lab's "How it's drawn" ↺ now puts back what the page opened with. CHECK:
  `bun run tools/qa/smokelook-check.ts` (7 lines: the table is the paste row for row, the paste's solver numbers are `defaultSmokeParams()`, each row
  lands where the renderer reads it, the step off and on, the wiring; `SMOKELOOK_BREAK=1` fails). Software boot of the lab: the rows open on the table.
  NOT MEASURED: what the shorter step costs (the march is 1.5 times the samples); his numbers were set by eye at dusk in the lab on his GPU.
- **EVERY MAN DRAWN HAS HIS SHADOW PROXIES** (11-03; `scene/boxes.ts BoxWorld.MAX_CAST` = 16, the skinning's own count; `common.wgsl` and
  `smoke.wgsl SceneInfo.charBounds`, `meshShadowT`, `traceCapsulesFrom`; shared renderer code, so every page, no switch: it is a capacity, not a
  rule). The maintainer on a thirteen-man mission: "odd that the two guys don't have shadows from the window, the chimney does". The proxy list
  (the round cones every shadow, bounce and reflection ray sees a man as) stopped at EIGHT characters, in list order: the player, his four and the
  first three defenders. A ninth man was skinned and lit like anybody and threw no shadow of any kind, stood in no mirror and bounced no light.
  Now the list holds every drawn man. The info block's word offsets are named (`INFO_BOUNDS`, `INFO_GLOBALS_Y`, `INFO_GRID`, `INFO_GLOBALS_B`,
  `GLASS_WORD`), since the bounds array grew in place. THE MESH TREES STILL HOLD EIGHT (`render/meshbvh.ts MESH_MAX_CHARS`, 7.4 MB): a man past
  them is his round cones to every ray. `meshShadowT` used to stop at the trees' count, which would have dropped his traced shadow again under
  mesh shadows; it now walks the whole list, and the reflection tracer's mesh branch adds the cones of the men past the trees. COST: a shadow
  ray tests up to sixteen cull spheres where it tested eight, and the soft shadows walk the cones of whoever is near. NOT MEASURED. The game's
  budgets written for "eight cast shadows" (`guards.ts MAX_CHARACTERS`, the room fight's five defenders, the patrol routes, the QRF's count) are
  untouched: they are rules of play now, their owners' to keep or lift. Checked on the software adapter, two boots that differ in this change: the pass at midday with
  thirteen men, the sixth and the tenth of the list stood in a window's light. Before, the page's list reads 8 and only the sixth throws a shadow;
  after, it reads 13 and both do; no GPU error. `probes.ts foursquad-budget` now holds the new statement on both stands (ten men, a place each, in
  the floor's order; a man down goes last and keeps his cones), and `tools/qa/grid-globals.ts` reads the info block by the named offsets. Never
  seen on a GPU. THE OFF WORD (11-03, at the join into the build): `?allproxies=0` in the page (`main.ts`, `BoxWorld.castCap` down to the mesh trees' eight) = the
  list stops at the first eight men drawn, as it did; the list's room and the info block's layout do not move with it. `foursquad-budget` holds it
  (`ALLPROXIES_BREAK=1` must fail one line). No game table: nothing in the update reads the list. The word never seen on a GPU.
- **FIREFLIES ON MEN BY DAY: THE SUN'S REACH MASK HAD LOOKS STANDING INSIDE WALLS** (11-01; `render/reach.ts reachMaskForDirectional`'s fifth argument,
  `game.ts buildDirReach` `free`, `REACH_OPTS.solidLooks`, `?sunlooks=old` = the looks as they were). The maintainer, the pass at midday, men in lamp-lit
  rooms speckled white: "still lots of fireflies", "seems to be a direct light problem". CAUSE: under ReSTIR the hour's sun has a mask of the roofed cells
  it can reach, and a cell was lit when one of five looks × three heights had a clear path to the sun. The corner looks sit 9 cm inside the cell; a wall on
  a cell line is 12 cm to either side; a look that started INSIDE the wall left through its outer face. So every cell along the inside of an outer wall
  read "sun reaches" (the pass at midday: all of column 12 and 34, most of row 10). There the sun, thousands of times a bulb, is in every pixel's fresh
  draw, and each pick of the bulb is a white sun-sized speckle; men stack at walls. Print the mask before blaming an estimator (the first guess here was
  that the sun had no mask at all). MEND, two halves: (1) a look stands in open air: one inside an upright static shadow caster (turned or not; not tilted,
  round or cut) is moved out through the nearest side face that stays in its own cell and in no other box, with air beyond it (a cabinet 2 cm off a
  wall put the look ON the wall's face, and a ray that starts on a face does not hit it: found on the page's stand, where rooms are furnished), or not asked. (2) A directional light's mask
  has 37 looks a cell, the five and a 6 × 6 grid (`DIR_SUBS`), at five heights (`DIR_HEIGHTS`): the looks inside walls had been hiding that five miss a
  sun patch a fifth of a cell wide, and a cell wrongly DARK has no sun in its draw, so the patch a window throws is cut at the cell's line (the review
  counted 9 such cells on the pass at dawn with (1) alone; three heights missed a band under a window's head on the compound at dawn, lit at 1.6 m and
  dark at 0.8 and 1.7). A build is 35 to 120 ms on the pass, the office and the compound, on a busy machine (it was 4 to 36). The masks are the picture's alone. CHECK: `bun run
  tools/qa/sunreach-check.ts` (both stands; 58 levels with a roof, 232 level hours: every lit cell has an open-air point that sees the sun on a 3 cm grid
  against every shadow caster; on five levels every dark cell is asked on a grid of its own and holds a sliver of sun at most, under a twentieth of its
  open air; the pass's wall cells are dark and its window cells lit; a level that does not stand up is a FAIL; `SUNREACH_BREAK=1`, the five looks where
  they fall, fails on hollow cells). STILL TRUE: a sliver under a twentieth of a cell can be cut at a cell's line (66 of 8,911 dark cells on the five
  levels, the widest 4.5 % of its cell; 11 on the pass at midday, as many as before the mend), and a sliver a centimetre wide (over a wall's top, under
  the edge of a roof that sits above it) still lights its whole cell. WHAT IS LEFT, said to him: a cell a window or an open door REALLY lets the sun into keeps the sun in its draw, and a
  man standing in that cell but out of the patch still speckles. The general answer is the sun as a HELD stratum light (its own rays every frame, like a
  carried torch), not built: its cost and its soft overcast disc are unknown without a GPU. Software page, the pass at midday, men at the kitchen's wall,
  direct-light view, old mask against new in one boot: it showed nothing either way (the camera's own rule hid the men under the top of the wall they
  stood at), so the mend rests on the mask, not on a picture. NEVER SEEN ON A GPU.
- **THE LOOK PANEL'S "STIRRED SMOKE CURLS" ROW** (10-31; `lookConfig.stir`, row `stir`, 1 on the look page, 0 = as it was, `?look=stir:0`; `game.ts
  smokeWake` + `SMOKE_STIR`, `smoke/smoke.wgsl emBody`, `SmokePush.body`, `SmokeSystem.setStir`, `SmokeParams.stirConf`. The maintainer, 10-31, of a
  smoke can in the yard: "i do wonder if there could be more turbulence or vorticity near the edges, especially when it's driven by player movement",
  "the smoother flow when gently pushed by the wind like against the wall does look great", with the wavelet turbulence paper, whose idea is taken: detail
  only where the flow already has energy). WHAT WAS THERE: a moving man was two soft shoves along his heading (hips 0.30 m, chest 0.26 m, reaching three
  radii), and he is no obstacle: the smoke bulged ahead of him and closed behind with nothing shed. The solver's vorticity confinement was one strength
  everywhere. THE ROW, three things: (1) HIS BODY is a standing capsule from 12 cm over the floor to half a metre over his hips, 0.24 m in radius (emitter
  mode 2, `tall` = its half height, `emReach` = what wakes and lists a brick): the air inside goes with him, the pull is gone by 1.8 radii, and the
  pressure step sends what he displaces round his sides. (2) HIS STRIDE: a kick just behind him, sideways,
  that changes side with his own walk cycle (`c.anim.phase`, one cycle is two steps, so nothing is kept between frames and a turn does not
  scramble it), at 0.9 of his speed and never over 2.2 m/s: a lattice
  of 5.5 cm with this advection would smooth a walker's wake into two still eddies, and the kick is what sheds them. (3) CONFINEMENT IS RAISED ROUND A
  BODY: × (1 + `stirConf` 2.0) at his capsule, back to the plain strength by three radii (the debugger's "curl round a moving body ×"). Smoke nobody walks
  through is not touched: the wind's relax and a can's own plume are as they were. REMOVED: the solver's own uniform draught (`windX`, `windZ` and their
  two sliders, which nothing set since `game/wind.ts`); its two parameter words carry the row. SIGHT: the guards' probe reads the same solver density, so
  on the look page a wake changes sight lines a little; the maintainer the same day: "i'm actually fine with vision being a bit different than the
  visuals, can be just the brick level stuff", so finer picture-only detail is allowed next. The headless stand-in drops a push: no digest moves.
  Check: `bun run tools/qa/smokestir-check.ts` (what a moving man HANDS the solver, over one stride: with the row up one upright body a frame moving as
  he moves and no soft shove, a kick across his way 7 frames to one side and 7 to the other of a 20 frame stride, growing with his speed to 2.2 m/s and
  with the row; a man who stands is not told of; with the row at nothing, or off the look page, the build's two shoves: 5 lines), `SMOKESTIR_BREAK=1`
  fails 3. SEEN: the look page boots with the solver's new kernels on the software adapter (119 of 119 pipelines, none failed, no page or GPU error).
  NOTHING OF THE SMOKE ITSELF: a step of the solver takes seconds there and a can's cloud did not finish growing in eleven minutes, twice, so no still of
  a wake was drawn. Whether a walker's wake curls, whether the kick reads as his legs or as a pulse, and whether calm smoke is untouched are for a GPU:
  NEVER SEEN ON ONE, in a still or in motion. The numbers to turn there: `SMOKE_STIR` in `game.ts` (the kick's 0.9 and 2.2, the body's 0.24), the
  debugger's "curl round a moving body ×", and the row itself (0 to 2).
- **THE LOOK PANEL'S "DAYLIGHT AT OPENINGS" ROW** (SINCE 11-04 EACH OPENING IS A PORTAL AND NOTHING ELSE: see DAYLIGHT COUNTED ONCE. What this bullet says of a
  light's STRENGTH and HOURS is the record of the disk it was and no longer holds: `openingRadiance`, `LOOK_OPENINGS.most` and `least`, `Opening.area`,
  "× the row", "nothing is looked for at night" and "light goes straight through a doorway or not at all" are gone; the row is a switch, an opening is
  lit at every hour, and a door's from the first hand's width. How openings are FOUND, kept, masked and paced is as written. 11-01, in place of the 10-28 "Window light" row and its `render/lookwindow.ts`; the row's key is still
  `windows`: `lookConfig.windows`, 1 on the look page, `?look=windows:0` = none; `scene/openings.ts`, its numbers in `LOOK_OPENINGS`; `engine.ts lookOpeningLights` and
  its `dayLights`, `render/lights.ts upload`'s `own`). The 10-28 row, after the maintainer in a room with every door shut under an overcast sky ("it doesn't really _look_ like all the light is
  coming from the window? feels flat"): a shut room's daylight arrives through the radiance cache alone, probes a metre apart that hold a few broad
  directions each, so it comes spread over the room with no way it came from. So by day each window was ALSO an area light (kind 3, `broad`, group `sky`): a
  one-sided disk on the wall's inside face, its strength the glass's area × the radiance it looks out on. A window was an upright Glass box found once a
  level, so an open door and a blown wall were nothing. The maintainer, 11-01, of windows and open doors as real openings: "yep let's do it. blowing holes
  in walls should work too!". THE RULE NOW, one for all of them, from the level's boxes as they stand: AN OPENING IS WHERE FLOOR UNDER A ROOF MEETS FLOOR
  UNDER THE SKY WITH NO WALL BETWEEN THEM, AND WALL ON BOTH HANDS. Glass is no wall to light; what is left of a wall round a hole is one. HOW
  (`planOpenings`, statics only): a plan on the place map's cells but cut in eighths from the floor to the ceiling (the place map's own is cut between a
  man's shin and his head, and a strip of glass over a door's height is nothing to it), with each cell's roof height; an edge is two cells side by side,
  one roofed and one not, that light passes at a shared height, where a short flood under the roof finds `LOOK_OPENINGS.deep` 2 cells a man could stand on (so
  not the strip under an eave, not a lorry's bed under its canvas); edges in a line are a stretch; each stretch is then MEASURED: the wall's plane is
  sampled every 5 cm along and 10 cm up to the roof, a sample open when no solid box (not glass, not NoShadow) stands in the straight way through the
  wall, from `reach` 0.5 m outside to the light's own plane (sandbags against the outside take what they cover; a cupboard in the room is in front of the
  light and throws its own shadow, as does a leaf swung into the room). Open samples make stretches that end where the wall is shut from floor to roof for `jamb` 0.2 m or more (less is a mullion or a post). A
  stretch that is still open at the end of what was looked at has no wall on that hand: an OPEN SIDE, no light (a stall's awning on posts, a canopy with
  one wall behind it; the cache carries what comes in there). Slits under `crack` 0.12 m are shut, and so is a gap under `slit` 0.25 m high (a shed's wall that stops a hand under its roof is still a wall: with the
  gap open no wall would be shut from floor to roof and the shed would have no jambs); less than `small` 0.25 m² is no opening; a stretch
  longer than `span` 2 m and than twice its height is several lights side by side; one found twice within `same` 0.6 m is kept once. The light hangs 5 cm
  inside the wall's inside face at the middle of what is open. A DOOR (`openAreas`, the light half, made again when a leaf swings): the doorway is always
  in the plan, and its leaf, with its true shape, takes out the samples it stands across where it hangs, so a shut door's light is 0, one half way round
  is part, and THE LIGHT NEVER MOVES while the door swings (its reach mask is made once). Light goes straight through or not at all: a leaf swung OUT a quarter
  round still stands across the whole doorway and gives 0; one swung IN covers only what of it is still in the wall's depth. NOT AN OPENING: a door or a pane between two roofed places (a door under a canopy too), an
  open side, glass out of doors. WHAT IS ONE that the old row never lit: an open outside door, a hole a charge blew, a window hole with no glass, a strip
  of high glass, a passage's mouth and a garage bay's open side between their walls (the pass's bay and lean-to; the smoke lab's tunnel). STRENGTH:
  `openingRadiance` × the open area, the area counted at `most` 2.5 m² at most: THE CACHE BRINGS THE SAME DAYLIGHT IN A SECOND TIME, and the bigger the
  opening the more of it the probes resolve, so past a big window's size the light would only double the room; it is there to give the daylight a
  direction (the old row counted a window's whole glass). The radiance is as it was: half the sky low down, half the ground lit by the sky and the sun
  (`LOOK_OPENINGS.ground` 0.5), × the row, fading out as the sky's mean radiance falls from 0.12 to 0.04, so a night's rooms are as they were and nothing is
  looked for then. THE LIGHTS ARE THE PICTURE'S ALONE (`engine.ts lookOpeningLights`): one light an opening, in the engine's own list (`dayLights`), handed to
  the upload each frame (`lights.ts upload(time, own)`) and uploaded after the game's lights and the infrared ones. The light set itself is the
  game's and goes into every checkpoint, so they are NOT kept in it. NOTHING THE GAME READS OR SAVES HOLDS ONE: the CPU's light estimates, the fog's
  lamp list, the game's reach masks, the count a new man's torch is held to (`spawnUnit` throws at 64 steady lights; a first cut that kept 32 spare
  lights in the game's list broke spawning on the block, the warehouse and the office), a checkpoint, and `stateHash`, which is the same to the bit
  with every opening lit and masked as before the first frame (a second cut that kept them in the light set was in every checkpoint). So the update
  with the row on is the update with it off; the GPU's light meter sees them as it sees everything in the picture, and the headless stand-in does
  not, as it sees no bounce. Openings are found again when the level's boxes have changed and then stood still for `settle` 4 drawn frames (never on
  the frame of a blast), at once on a level's first frame: the world's `geomSerial` says when to look, and every eighth frame besides, at
  `openingsKey`, every static box's shape mixed number by number (a lintel and a frame's bar count; a box that turns wet does not). What is open
  of each is measured again only when a leaf has moved (`leavesKey`). An opening that is where it was, or moved within `same`, or grew (a hole
  blown beside it), keeps its light. ITS REACH MASK (where the `rooms` row is on, a light with none is a candidate in every room, so it is out
  until it has one): decided by the engine against its own raster of the walls (`reach.ts buildWallRaster`, 0.4 to 2 ms, made at each finding), with
  the jump field kept by the engine and no note in `reach.ts`'s side tables (`ReachJob.own`: they are in a checkpoint). A FIRST mask of a light that
  would be lit now is decided whole, at once: a level's first frame (7 on the pass: 1.27 million sight-line steps, about 50 ms, once; 12 at the
  most), a new hole, the rooms row put on in the middle of a level. A mask is decided AGAIN only where the wall raster itself has changed
  (`wallRasterDiff`: a pane shot out or a crate gone redoes none), at the game's own pace for a lamp's (`REACH_PACE.steps` 60,000 steps a drawn
  frame, about 2 ms; a mask is 130,000 to 310,000), the nearest first, the old one serving until the new is whole. It runs in the drawn frame, so the
  lights come on behind the start card and on a paused page. A light's range is where its light, wide open, falls to nothing by the hour's own
  radiance (a swinging leaf does not move it). More than `lit` 12 open at once: the nearest to SAM are lit (`lookConfig.eyeAt`, the camera's look-at
  where nobody says). `photometry.ts lightClass` calls the group `sky` the hour's own; `engine.ts glassLightMasks` gives them their panes as it does
  a lamp. LIMITS: A WALL MORE THAN ABOUT A DEGREE OFF THE GRID'S SQUARE GIVES NO OPENING (the hand-built room turned 0.02 rad keeps its window, turned 0.1 rad
  loses it: the finder walks the grid's two axes; the old row found glass at any yaw; no level has such a wall today); the two-tone light (sky down onto the floor, lit ground up onto the ceiling)
  is NOT built: one disk carries the mean (the light record has no spare word, so it is two lights an opening or a shader change); the double count
  above stands until the cache's own rays stop bringing the open air into a roofed place; a wall thicker than 0.30 m has its light hang inside the
  reveal (the way through is asked no deeper than `min(face, 0.45)`); a wall that stops more than `slit` under its roof all the way along is an open
  side; a tilted or cut box blocks as the box round it; which twelve are lit moves with Sam, so on a floor with more than twelve a far room's daylight
  loses its direction (the cache still lights it); a wall put back (a save put back, a reset) keeps its hole's light for the `settle` frames; when the
  light buffer's 64 slots are full these are the first left out (they are uploaded last). WHERE A WALL SITS ON THE 0.4 m GRID CAN LOSE ITS OPENINGS
  (a review's sweep of a room with a door and a window moved in 1 cm steps across a cell): a wall 0.24 m thick with the roof to its outer face loses
  them at 1 place in 40, 0.30 m at 5, 0.40 m at 17; a roof that hangs 0.30 m past the wall at 19 and 0.45 m at 34 (the way through is asked no
  deeper than the line where roof meets sky, and the samples all read open, an open side). Every outside door (199) and every unboarded window of
  the shipped levels is found; a new level with an eave or a thick wall may get none. A thin panel within 0.3 m inside a window is taken for the
  wall's own face (it shuts the window); a bookcase flush on the wall moves the light to its front. An opening in a wall on the map's last cell is
  not found (no open ground beyond it on the grid). The door numbers below are the frozen stand's, where the tea door's leaf swings out; on the
  shipped stand it swings in and 0.05 / 0.4 / 0.8 / 1.2 / 1.5 rad give 0 / 0.80 / 1.40 / 1.60 / 1.70 m². The engine keeps a second wall raster
  beside the game's (0.4 to 2 ms a finding): the price of the game not knowing. Check: `tools/qa/openings-check.ts` (27 lines on both stands;
  `OPENINGS_BREAK=1`, glass taken for wall, fails 10): a room built by hand (a 4 m window as two lights, high glass, a porch, a shed whose walls stop
  10 cm under its roof open by its doorway alone, and a shut leaf a hand shorter than that doorway leaves it shut; an awning, a canopy, glass between rooms and out of
  doors are none); the pass: 4 windows (tea 1.53 and 1.53 of 1.71 m² of glass, kitchen 0.99 of 1.14) and the bay's and lean-to's
  open sides with every door shut, each outside door open 1.6 to 1.7 m², the tea door at 0.05 / 0.4 / 0.8 / 1.2 / 1.5 rad 0 / 0 / 0.50 / 1.20 / 1.60 m²
  with no light moved, 11 reach masks none dark, a real breaching charge's hole in the store's outer wall 4.47 m² (asked on the shipped stand: the frozen
  one holds a charge on an outer wall off), and the engine's own method on a stand-in with the game standing still (11 lights, the 7 open lit on the first frame with whole masks, all 11
  masks whole 25 frames on at no more than the pace a frame and no work after; the door's light on and out with its place, range and mask unmoved;
  `stateHash` the same before the first frame and after the last mask, the game's light list, reach lists and steady count as with the row off, and
  a man still spawned; with `lit` at 3 the three nearest Sam are lit wherever the camera looks; a wall holed is a new light 4 frames after it stands
  still, lit with its mask whole while the others stay lit by their old masks, and gone when the wall is put back; the rooms row off, on, and on for
  the first time in the middle of a level never leaves a frame with fewer lit; the row at 0 takes all out); the office 13 with doors shut (7 over 2.5 m², 5 over head height; the old row
  found 9); gwot-yard 1 and tut-quiet 0 (awnings); the smoke lab 2 shut and 6 open; radiance 0.51 grey, 0.83 midday, 0.26 dusk, 0 night. Every level
  swept by hand: 1 to 27 openings, `planOpenings` 0.6 to 13 ms, `openAreas` under 0.5 ms. SEEN, the software adapter only, one page of
  the text before the lights' list left the light set (the same shaders; 640 by 400, the look page on the pass under the grey sky with a squad of four, paused and stepped; 119 of 119 programs, no GPU error, no
  page error): none of these lights in the game's list, 10 in the picture's (the look page's dressing boards the tea house's first window, shuts a
  shutter on its second and nails a blanket over the kitchen's: 0.86, 0.77 and 0 m² there, so 6 are lit with every door shut). The kitchen with its
  back door shut is lit warm by its bulb; the door open, its light comes on (0.87) and the room turns to cool daylight, brightest at the doorway, with
  the men's shadows falling away from it; with the row at 0 the list is empty and only a patch by the doorway is daylit. The bay's own floor is nearly
  black in its still (the eye is set for the yard there: not looked into). The store, shut, is lit by its lamp; holed by the game's own breaching
  charge it floods with daylight through the hole, an eleventh light at 1.35 with its mask, debris still in the air. The final text booted 119 of 119 and drew the kitchen again, door shut and open, with the same
  lights and no error. NEVER SEEN ON A GPU. Not measured: what the lights cost in the direct pass, the grain of a small disk in ReSTIR, the mask work's 2 ms a frame on his machine.
  HOW MUCH IS COUNTED TWICE, MEASURED (11-02, after the maintainer's "midday still quite rough. also with doors open"; the pass's kitchen at midday, its two
  outside doors and the store's door open, Sam in it, software adapter, `tools/qa/look-sheet.cjs --rows midday,midday+windows:0 --views kitchen`). The level
  the place map holds for the room is −3.91 (log2) with the row on and −5.82 with it off: the opening lights are about three quarters of the room's light.
  A test tree with the sky taken out of the cascades and of every gather ray (so no daylight came in a second time by the sky; the ground outside seen
  through a door still did) held −4.21: the second helping is about a fifth of the room's light, 0.3 of a stop. So counting daylight once (a probe and a
  gather ray that leave by a lit opening carry nothing, and `most` goes) would be the cleaner rule and would take one hack out, but it is NOT what makes that
  room read flat, and it was not built. What the stills do show there: the kitchen's two outside doors face each other across 6 m with a window beside one,
  so its floor is lit from both sides (2.5 to 1 from a door's pool to the middle of the room, before any shadow) on a pale concrete tile; the tea room on
  the same sheet, a dirt floor lit from one side, reads deep. After the one-rule mend above (a place seen from another, and the roll-off in `lookShowBy`)
  no cell of the midday sheet has a burnt area. The store's patch of door light, blotchy in his shot, cannot be judged on a software still three frames old.
- **SHARP DIRECT NEVER GIVES A CLASS MORE THAN ITS LIVE LIGHTS SEND UNSHADOWED** (11-02, `sharp_direct.wgsl` and its compact twin; every page, no row). The
  maintainer on his GPU, midday on the pass: "still seeing lots of splotches/glowing", "it seems like it's screen space, seems invariant when zooming", and on
  11-02 earlier "not able to see where the glow is coming from here by flipping through the debug views", "it subsides a lot once i shoot out the one light in
  the room". The pass draws each class as U at the output pixel times the ratio S / U of the internal texels round it, and a light walled off from the cell
  stays in U on both sides so that the ratio carries across. Along such a light's terminator on anything round or in relief (a man's sleeve, a strap, a drum)
  the pixel faces the light and its texels do not: their ratio is the room's lamp's and the pixel's U holds the walled-off light, which was then drawn at the
  lamp's visibility. By night that light is another lamp behind a wall. By day it is the sun over the roof, thousands of times the lamp, so the pixel went
  white and the bloom spread it. A debug view turns the pass off (engine.resize), which is why no view showed it. THE RULE: a walled-off light is dark by
  proof (the edge rays and the slivers already took it so), so a class is bounded by its live lights' unshadowed sum times the ratio's own most. Nothing the
  internal estimator measured is taken away: it never sends a ray to a walled-off light. SEEN, software adapter, two boots that differ in this line alone,
  `look-sheet.cjs --rows midday --views lamp-out --settle 2`: the man in the shut bulb room had 98 pixels over half white (the brightest 184 of 255), after
  none (the brightest 61), and no pixel of the picture outside him moved by more than 10 levels. NOT MENDED BY IT: a light that does reach the cell (a man
  beside a sun patch, in a cell the sun's reach mask calls lit) can still be lent a neighbour's ratio. NEVER SEEN ON A GPU. His other word that day, "periodic
  flashing of the glows as if it's throwing away the temporal buffer every so often": in his recording the edges of chairs, drums and wall feet near the lamp
  go bright for about five frames and fade over the next ten to thirty, twice in 0.4 s after the camera stops. What drops the history was not found (the
  recording does not show the HUD's res line; a scale step of the ladder remakes every target, but its cooldown is 2 s). THE OFF WORD (11-03, at the join into the build):
  `?sharpcap=0` in the page (`render/frame.ts sharpCapConfig`, the scene prelude's `const SHARP_CAP`, read once at boot; both writings of the kernel keep the
  bound behind it) = a class takes the ratio its taps carry with no bound, as it was. `tools/qa/sharpcap-check.ts` holds the texts, the prelude under each
  setting and the page's line (`SHARPCAP_BREAK=1` must fail). The word never seen on a GPU.
- **THE LOOK PANEL'S "REAL LIGHT VALUES" ROW** (10-28; `lookConfig.lamps`, 1 on the look page, 0 = as authored; `render/photometry.ts` holds the table and
  the rule, `engine.ts lookExposure` sets `LightSet.dayExp` / `dayAmount`, `render/lights.ts upload` applies it; it REPLACES the 10-25 "Lamps by day" row, one
  number for every lamp at every day hour. The maintainer, 10-25: "we don't have consistent, physically accurate light values (lumens?) across the image +
  the right HDR mapping to screen … a lamp on the ground looks way too bright in daytime mode … same for the flashlight"; 10-28, of fixing it properly: "I
  think my answer would be yes"). WHAT THE NUMBERS ARE: every level is authored as a night and its FIXTURES are real with one unit = 3.3 lux (a store's bulb
  26 = 86 candela, 15 lux on the floor; a street lamp 260 from 5 m = 34 lux; a lantern 4.5 = 15 candela). What the night holds DOWN is whatever a real night
  would white the picture out with: a weapon light is 10,000 candela where the game's is 200, a headlamp 15,000 against 400 to 500, a flash or a blast more
  again, lamp glass 10,000 cd/m² against 10. An hour's sun and sky are not in the night's units: they are the real ones already × that hour's exposure
  (a real sun is 30,000 units, the hour's is 1 to 6), and they stand in a fair proportion to each other (midday: the sky gives 0.29 of what the sun puts
  on the ground; a real clear noon 0.2 to 0.25), so they are left as authored. THE RULE: `HOUR_SUN_LUX` says what each hour's sun stands for (midday
  100,000 lux, overcast 3,800 as the bright patch of a 10,000 lux day, dawn 600 and dusk 500 as the first and last minutes of sun with the photocell's
  lamps burning); from it the hour's exposure against the night's (`hourExposure`: midday 1/5,051, overcast 1/1,152, dawn 1/70, dusk 1/69, night 1);
  every source that is not the hour's own is uploaded at min(as authored, its real strength × that exposure), by what it is (`lightClass`): a FIXTURE's real
  strength is its authored one; a HAND light's (group `flashlights`, or `card`: the torches, the rail light) is 10,000 candela and a VEHICLE's lamp's
  15,000 (`TRUE_CD`); a FLASH's (ttl, a tracer, a light `game/fire.ts` flickers) is 6 × its authored one (`FLASH_TRUE`); a glowing FACE (the frame's
  `emissiveScale`, so boxes and emissive markings alike) 1,000 × (`EMISSIVE_TRUE`); the LIGHT THAT STANDS FOR A GLOWING FACE (kind 3: a ceiling panel's, a
  screen's, a lit sign's; class `face`, 11-02, the bullet below) 50 × (`FACE_TRUE`). So at midday a bulb is 1/5,051, the rail light 1/100 (1.1 % of the sun
  on a wall 3 m off; a real one about 1 %), lamp glass 0.20 of its night glow; at dusk a bulb is 1/69, the torch 0.63 of its night strength and glass in
  full. The `min` is the night's hold: nothing is drawn brighter than the night draws it. The weather's cloud dims sun and sky under the hour's one
  exposure (`hours.ts`), so lamps count for more under rain by themselves. The row's amount is in stops (k^amount). NOT scaled: the sun and the moon, a
  lightning stroke (`transient`), an infrared illuminator, a device's sight cone (`group 'devices'`) and the Team & kit figure's staging (`kitview…`).
  Applied to the UPLOADED record and the frame's scale alone, so the picture, the bounce, the beams and the GPU's light meter read one number;
  `RtLight.intensity` and `settings.emissiveScale` are never written. WHERE NOBODY SAYS THE HOUR (`lookConfig.exposure` null: a night, or a level lit as a
  day by its own sky) the exposure runs from 1 to midday's by that sky's mean radiance 0.08 to 0.35, read without the Sky light row, as before. A night is
  left exactly as it was. "Hold: plain boxes" and the sweep's `look.all` do NOT move it (it is the light, not a layer). CHECK: `bun run
  tools/qa/lookunits-check.ts` (17 lines: the table against `HOURS`, each class at midday and dusk on the renderer's own light set, the hold, the
  proportions against the sun, nothing written, a fading torch, the row at a half, the night word for word; `LOOKUNITS_BREAK=1` fails 8). KNOWN BREAKS, to be settled before any of it goes to
  the build: the game's CPU readers of the list (`guards.ts lightEstimateAt` and its callers: calm torch, power out, morale, night vision) still read the
  authored strength, so by day a man can judge a lamp-lit room as lit that the picture and the GPU meter have as shade; the lit fog's own lamp list is
  authored strength too (not read under the full picture, `FOG.full`); a lamp at 1/5,000 still holds its slot and its range; a glowing face lights its
  surroundings through the bounce by the eye's factor (0.20 at midday) where its real share is the exposure's; ONE flash factor serves a rifle (× 3) and
  a stun grenade (× 13); particles that glow of themselves (sparks, flame) are not scaled; indoors by day the only adaptation is the shade row's × 6
  where a real room is some hundreds under the open air. The night's own moons are the levels' (41 of 53 are 0.9 = 3 lux, ten full moons): not touched.
  IN THE PAGE on the software adapter (the pass, high × 0.5, 119 of 119 pipelines, no console or GPU error; four stills, each five frames after the change):
  the engine handed the light set 1/69 at dusk with the row on and 1 with it off; at MIDDAY a test flashlight's pool on a floor in shade fell from 0.33 to
  0.026 of the picture's white (the floor beside it 0.011) and two flashlight beams on open snow went from over the snow's own brightness to nothing to see;
  at DUSK the same pool went 0.39 to 0.28 (a flashlight keeps 0.63 there). NOT SEEN: a fixture at dusk (none stood in that view); overcast and dawn; a
  flash or a fire by day; the night compared in pictures (the check has the upload word for word); any of it on a GPU, moving, or played.
- **A LIGHT THAT STANDS FOR A GLOWING FACE IS A CLASS OF ITS OWN: `face`, 50 × ITS AUTHORED STRENGTH** (11-02; `render/photometry.ts lightClass` /
  `FACE_TRUE`; part of the Real light values row above, so `?look=lamps:0` is the page without it and no other page has it. The maintainer on his GPU, the
  smoke lab at dusk with its blur row at 0: "something about the emissive on the vending machine still doesn't look right, and blur behind isn't the right
  solution"). THE FAULT: `game.ts` gives every glowing marking and every thin glowing box (a ceiling panel, a screen, a sign) a kind-3 light of its
  radiance × its area, and the row classed that light a FIXTURE, authored = real. But these are held down by the night like a torch: a vending machine's
  front is 7 candela in the game and some 350 real, a 4,000 lumen panel 20 against 1,300, a monitor 1 against 40. So at dusk the sign's FACE was drawn in
  full (`emissiveDayK`) and its LIGHT went up at 1/69, 0.027 units, under every floor a light is culled by (0.03): it lit no ground and no smoke, and a
  bright sign with dead smoke over it reads as pasted on. Read off the software smoke lab, one boot: the sign's light put to 0 by hand changed nothing in
  the picture. THE RULE: kind 3 that is not the sky's (`group 'sky'`, the look page's opening lights, are the hour's own and stay `none`) is class `face`,
  real = 50 × authored: 0.73 of its night strength at dusk and dawn, 1/23 under overcast, 1/101 at midday (a panel puts 0.15 % of the sun on the floor
  under it; a real one about 0.1 %). The face is still drawn by `EMISSIVE_TRUE` (0.20 at midday), which is the glass of a lamp and too much for a panel:
  that is the frame's one scale for every glowing box, left alone. WHICH LEVELS: the pass has no such light, so its day look does not move; the smoke lab 2
  (the vending fronts), the compound 15 and the records office 18 (ceiling panels 5 to 8 units, monitors 0.3), where rooms by day take their panels at 50
  times what they did, still a hundredth of the night. CHECK: two lines in `lookunits-check` (the class at midday and at dusk), `openings-check` (an
  opening's light is not one). SEEN on the software smoke lab at dusk, the same boot script before and after: the ground before the red front takes
  its red, the smoke crossing the blue front is lit by it (brighter and cooler there than the plume's far side). NOT SEEN: a GPU; the compound or
  the records office by day; midday or overcast anywhere.
- **THE LOOK PAGE WITH A GEOMETRY TRIAL ON IS COMPILED WITHOUT THE ROUND TEST** (`main.ts urlFlagsBeforeEngine`: `roundConfig.gpu` is false when
  `lookConfig.on` and a trial has put `cutConfig.swap` off; `?round=1` compiles it in; 10-25). After the build's round things came in by merge, the look
  page lost its SOFTWARE device on its first drawn frame ("A valid external Instance reference no longer exists") where the plain page at the same
  quality drew. Runs that settled it: look with every trial off and `round=0` draws; look with every trial on and `round=0` draws; look with every
  trial on and rounds compiled in dies. The reading (the software adapter's room, above; not bisected further): a trial keeps the cut twin of every ray
  walk live from the first frame and `ROUND_BOXES` adds the round test to each copy. `ROUND.on` (the game's own switch, the debugger's row) is NOT
  moved: only the shader's half. On the pass nothing is lost, because the trial turns the kit's octagons into its own cut prisms before a round thing
  is looked for. NOT so everywhere: the trial takes only octagons stepped 2 mm ± 1 mm (`geoTry.ts octagonAt`) where `round.ts` takes 0.5 to 4.5 mm, so
  on a level whose octagons are stepped otherwise (block, consulate, records) and wherever the panel's Shapes row leaves an octagon alone, a round
  thing is drawn and traced as its square box on this page while the game plays it round. The one-file hosted page takes no address flags, so it
  cannot ask for `round=1`. Whether `?look&round=1` draws on a real GPU: never tried.
  SINCE 10-26 the cut set traces a round as the picture's prism and its box walks no longer hold the round test (`docs/agents/levels.md`, the round
  things' bullet, THE CUT SET): `?look&round=1` with every trial on DREW on the software adapter (119 of 119 pipelines) and lost its device again with
  `&roundexact=1`. The exclusion in `main.ts` is LEFT IN all the same: taking it out changes what this page draws on levels other than the pass, and
  that has not been looked at.
- **THE LOOK PANEL'S PERFORMANCE ROWS** (`ui/lookpanel.ts perfRows`, `ui/perfsend.ts`; the maintainer, 10-25: "it'd be good to have a way to collect a lot
  of perf data so you can see how it runs on my laptop", "i do worry that we have a few stuttering frames even if overall fps is pretty high"). The
  measuring is the F9 recorder and the F8 sweep as they are (`ui/perfrec.ts`, `ui/perfsweep.ts`); the panel adds two buttons and a way out of a page that
  cannot download. Record play = `rec.start` / `rec.stop` round his own play. Price the layers = the sweep over its own rows plus the look's:
  `PerfSweep.extra` is a list of rows a PAGE owns (`{ row, flip }`, the flip made when the sweep arms, undone by its own undo when the sweep is kept
  back), and the panel gives it `look.all` (plain boxes) and one `look.<row>` a layer (not Size, Sky light, Air, Clarity or Untile; Snow relief flips
  to 1, its plain value). On a stop the recorder's own way out runs first, then `takeRead` makes the short read (frame time at the middle, the 95th and
  99th and the worst; counts over 20 / 25 / 33 / 50 / 100 ms; the forty slowest frames outside a sweep's warm-up and settles, each with its CPU parts,
  its GPU cost, the passes that ran over their own middle and what was logged within 0.4 s; every pass as mean and 95th) and shows it in a box with a
  Copy button. A sweep's rows are NOT priced on the page: `tools/qa/perf-report.ts` prices them off the whole take. `sendTake` hands the read and the
  whole take (gzipped, base64, 180,000-character parts) to `window.__takeStore.put(path, doc)` as `takes/<id>` and `takes/<id>/parts/<n>`, one take at a
  time, a failed write tried once more. No page of this repo sets `__takeStore`: a page that hosts the one-file build hands it in, and without it nothing
  is sent and the read sits in the box. `takeRead` was tried headless on made-up takes only; no take has left a page yet.
  Where a store is handed in the recorder's own download is NOT tried (a hosted page starts none, and the try only raises the host's notice over a
  take that did get out). FIRST TAKE FROM THE MAINTAINER'S MAC (10-25, the pass at night, high at half scale, world frozen, 20 rows × 3 rounds): base 15.5
  ms of GPU; every layer of the look together gives back 0.8 ms (5 %), no single layer is outside the noise, the shade row is free. A third button, **Sharp direct**, is `?nosharp` from the panel (`sharpForce.v = false`, then the current preset put on again so its other resolve rows follow; again = as the preset): a hosted page's address takes no flags, and the debugger's own row only holds until the ladder's next preset. Asked for by the performance lane, 10-25, for a take on Auto with it off; not while a take or a sweep runs.
- **POSTS SEATED IN A WALL'S END** (`scene/lookMend.ts seatPosts`, the look page only, `?seat=0` off; the maintainer, 10-25: "seems like a sorting issue
  that's flickering to right of the red door"). The pass's gate post (0.34 m square) stands with one face exactly in the end face of the yard wall it
  closes, so two faces shared a plane and the depth test flipped between the post's render and the wall's stone plinth. On the look page a post (tall,
  8 to 30 cm half both ways, square to the world, drawn, no glass) with a face in the END face on the SAME side of a wall whose section is inside the
  post's grows 4 mm in x and z when the level loads, and rises 4 mm where the two tops are level. A post that only stands AGAINST a wall is not matched.
  Over every level file: 12 posts (pass 2, gwot-yard 2, police-post 2, tut-sky 2, snatch 2, depot 1, depot-gwot 1); the console says how many. The level
  file is not touched: the right mend is in the level's own numbers, found by `tools/qa/zfight-check.ts` (`levels.md`). Never seen on a GPU.
- **THE CHARACTER LAB** (`host/charlab.ts`, opened from the look panel; the maintainer, 10-25: "men up close look pretty good, but not perfect. IMO we
  should just iterate on the characters in a character lab or the workbench"). Not a page of its own: on the running look page the camera is pinned on
  YOU at a man's height (`FollowCamera.setPin` every frame; Angle, Height, Distance, Looks at, or turning slowly), you wear any variant of the cast
  (`cast.ts castWear`, two arrows) and are painted as the SIDE of that outfit (`charLook.ts`), except in a mission that has you in the sneaking suit,
  where every outfit is the suit. Nobody notices or hurts you while it is open: `playerInvisible` and `godMode` are raised once on Open, only where they
  were down, and Leave puts down only what the lab raised, with your own outfit and the free camera back. You walk the mannequin into the light you want
  to judge him in. THE EYE STAYS ON YOUR SIDE OF A WALL (`charLabFrame`: `col.segmentBlocked` from the look point to the eye and to a point 0.25 m to
  either side of it, the distance bisected and 0.25 m kept off the wall); where that leaves it nearer than three quarters of the distance asked (3.8 m by default: the lens is a long one, 28°, and shows a man whole from about 3.6 m) it looks from the roomiest of the twelve
  angles a twelfth of a turn apart, a nearer one preferred. Coming in is at once, going back out is eased. `quietUtility` is raised with the two
  flags, as a show has it, so NO CHECKPOINT is written while the lab is open (`savestate.ts runNo`): a file with the lab's unseen player in it could
  be loaded later with nothing to put the flag down. `window.__charlab` = `{ state, open, leave, wear, worn, outfits }`. Sam keeps his own
  weapon in any outfit. Never seen on a GPU.
- **A HOUSE FRONT DRESSED BY HAND, A TRIAL** (`src/scene/frontTry.ts dressFront`; the look page only; the maintainer, 10-23, of the pass's tea house seen
  from over its yard: "this looks a bit too "regular" -- especially the face of the building we're looking at", and a minute before "the art style just
  needs a lot of iteration. right now it does look a bit procedural"). The answer tried is AUTHORING, not another layer of noise: one wall gone over
  piece by piece after the level loads (as the geometry trial does; `levels/pass.json` is not touched), every number placed by hand. The three windows
  differ (boards, shutters, a blanket nailed over), the two red panels differ (one worn, one plastered over), mends of other mud, a spout with its
  streak and ice, beam ends under the roof line, split wood under a window, a ladder leaned over the level's own planks, a plank bench with a can and
  a pot, a few mud bricks at the wall's end. Made of what the engine has: plain and tilted boxes (`Box.rot`), twelve-sided prisms for round timber
  (`cutRound` about z, a thin picture-only disc for the end grain) and thin cut boxes for flat shapes with an uneven outline. ONE MODEL: what closes a
  window is a REAL box (sight, a round and the room's lamp stop at a board, a shutter, the blanket and pass between the boards), as is what stands on
  the ground (none within a stride of a door; the swung shutter stands out at head height beside the planks) and the beam ends over every head; only
  what lies flat on the wall is picture only, as the level's own panels are. COLOURS are the level palette's rows darkened in one piece (`of(row, k)`,
  0.55 < k < 0.99), so `lookClass` gives each piece its map: a hand-picked colour between two rows gets none. Traps met: the wall past x = 28 at that
  z is a partition INSIDE the bunk room, and the yard wall there has a door between its two panels; the level already stacks planks at x 21.4 to 22.85.
  KNOWN: a tilted box takes no map and no clinging snow from the look (`lookSurface` leaves `BOX_ROT` alone) and a cut piece takes snow only about y;
  a wall charge there leaves the pieces above a hand's height hanging (`scene/lookTidy.ts` takes a decal with its wall only from the foot). It knows
  the house by three of its file's boxes (the two red panels, the second window's pane) and does nothing on any other level. Switch: `?front=0` or the
  look panel's 'Dressed by hand' button (kept on the machine as `twopointsix.fronttry`; the page loads again); needs cut boxes on, and holds the cut
  programs in from boot as the geometry trial does. Not a rule of the game: no knob, no journal key; the men's routes and sight along that wall change
  on that page. If the look goes to the build this belongs in the level's file or its kit, not in code. Checked: 98 pieces placed, none malformed,
  none in the level's own solids or by a door (a script over the loaded level); drawn on the software adapter only.
- **A LORRY WITH CURVES, A TRIAL** (`src/scene/lorryTry.ts dressLorry`; the look page only, with cut boxes on; `?lorry=0` = the lorry as its file has
  it, or the look panel's 'Curved' button, kept on the machine as `twopointsix.lorrytry`; the page loads again). The maintainer, 10-24, on a GPU picture of the pass's yard: "some curved surfaces on the truck would go a long way". The
  pass's lorry (twenty boxes under four flat slabs of snow) is gone over by hand after the level loads, as the house front is; `levels/pass.json` is not
  touched and the pieces are found by where the generator put them (`tools/levels/pass.ts LORRY`): any one missing = nothing is touched. The tilt is bowed
  over four hoops with its snow on the bow; the cab's roof is an arch from a leaned windscreen back; the bonnet is a narrow engine cover with a round top
  between two fenders that arch over the front wheels; wheels, hubs, lamp pods and a tank under the bed each side are round. 21 boxes added, 16 changed.
  AN ARC (`Box.cutArc`, new): a round prism (`cutRound`) turns its normal about the box's own middle, which makes a drum but not a roof. With `cutArc` =
  k metres the axis lies k UNDER the box's centre, so a box whose top is cut to seven facets of a wide circle (its top face + six planes) is shaded as
  that circle. It rides in the record's unused LAST row (`cut.ts packCut`: `CUT_PAD`'s w + k; every reader of a plane stops at the first zero normal),
  so an arc carries seven planes at most (an eighth plane drops the arc without a word); about x or z only. In `gbuffer_cut.wgsl` its facets and top
  turn (the box's own upright faces and underside stay flat), the look's paint is read straight down each quarter (no unrolling), its ends get no drum's
  rim, and an arc in the snow paint gets `snowBox`'s snow (grain, glow, the wandering edge) asked as the box's top face, and is drawn as a SHEET: its
  upright faces and ends are discarded (they stood as straight lines where the wandering edge drew back), so a cap lies two centimetres over its roof.
  Picture only: every ray meets the facets. A round's rolled rim is now on both end faces (a wheel shows one to each side).
  WHAT A RAY AND A BODY MEET CHANGES on that page: the bonnet (1.5 m high, 0.98 m a side) is a hump 0.56 m a side that tops out at 1.46 m, with a
  fender of at most 1.08 m beside it and, for 0.63 m in front of each fender, only the 0.78 m bumper; the tilt's eaves come down from 1.95 m to 1.6 m;
  a tank closes a metre of the gap under the bed each side. No knob and no journal key, as the geometry trial.
  KNOWN: no clinging snow from the look on an arc (the round's snow hook is about y only), so the authored caps are all the snow there is; the level's banks against the lorry are as they were; the lorry is found by twenty
  numbers copied from the generator, and the console says so when one misses.
  Checked: `dressLorry` on `levels/pass.json` under bun (21 added, 16 changed, every record seven planes or fewer where it is an arc); the page booted
  on the software adapter with 0 console errors; four pictures (play camera, the nose, from above, dusk). Never seen on a GPU.
- **THE LORRY'S FITTINGS, A TRIAL ON THE TRIAL** (`src/scene/lorryTry.ts kitLorry`; the look page only, with the lorry's curves on; `?lorrykit=0` = the
  curved lorry without them; no panel button). The maintainer, 10-30, on a GPU picture of it at midday: "truck could use some dressing up. but overall the
  round surfaces look nice". 43 boxes added after the curves (twelve of them cut prisms), found by the chassis rail: an exhaust stack in a heat shield up the cab's back corner,
  a mirror on an arm each side, a beacon, a step, a door seam and a handle a side, an air cleaner on the cowl, a winch drum between two cheeks, two
  hooks and a plate on the bumper, four stakes down each sideboard, three jerry cans racked on one of them, mud flaps, tail lamps, and the spare wheel
  upright on the tailboard. Rounds are prisms of twelve sides (`ring`), colours are palette rows (none under 0.55 of its row; the beacon's orange and
  the cans' olive are `gwotAs(…, 'metal')`, so a round into them throws metal, not a plant's or a cloth's debris). ONE MODEL: each is a real
  box to rays and bodies; the spare stands 0.3 m off the tailboard from 0.75 to 1.71 m, a mirror as far out as the bank of snow along that side.
  SEEN on the software adapter only (midday, 480 × 300, from over the cab and from behind): the stack, the beacon, the mirrors, the stakes, the air
  cleaner and the racked cans read; the winch, the steps and the spare were not in those two frames. NEVER SEEN ON A GPU; its cost there is not measured
  (`?lorrykit=0` against the page as it opens is the pair).
- **THE PASS DRESSED WITH ROUND THINGS, A TRIAL** (`src/scene/passTry.ts dressPass`; the look page only, with cut boxes on; `?dress=0` = the pass as its
  file has it, or the look panel's 'Dressed' button, kept on the machine as `twopointsix.passtry`; the page loads again). The maintainer, 10-29, asking what
  rounds cost and whether to use them through a level: "dressing up the pass level we have would be another good experiment". Gone over by hand after the
  level loads, as the lorry is; `levels/pass.json` is not touched and the chopping block is the test that this is the pass (not found = nothing touched).
  Three pipes stacked along the bunk room's wall (two iron, one rust) with snow along their tops; three fuel drums standing in the pocket of yard by the
  lean-to and one on its side; the chopping block made round, four logs by it and two along the wood pile; a loose tyre by the cart. 35 boxes added: the
  page as it opens (the shapes, house front and lorry trials on too) goes from 607 boxes with 131 cut to 642 with 167, none with more than eight planes.
  WHAT STANDS IS A CUT PRISM (`cutRound` about y, `lorryTry.ring(r, 2)`, as `geoTry.ts roundStacks` makes one), NOT `Box.round`: on the look page with a
  geometry trial on, `roundConfig.gpu` is off (main.ts) and a `Box.round` is drawn and traced as the square box round it, which the first stills showed
  (three square drums). So a body meets the box round each new piece and a ray meets the prism. Paints are the palette's own rows as they are (a row
  darkened under 0.55 of itself is no row to the look's material maps, `gbuffer.ts classWgsl`, nor to what a round throws off it); the olive drum is
  `gwotAs('olive', 'metal')`. No knob and no journal key, as the other trials.
  TRIED AND TAKEN OUT: the file's 93 low heaps of snow with their tops cut to arcs (the page's snow surface already draws a smooth hill over each, so
  nothing showed and 93 cut boxes bought nothing), and discs of snow on the drums' lids (a round's own end gets a drum's dark rim and no snow finish).
  KNOWN: the snow along a lying round is a hand-fitted arc each time, because the cut pass's clinging snow knows a round about y only (the lorry's bullet
  says the same); the look panel's trials are five positional arguments and four copies of one predicate.
  SEEN on the software adapter only (midday, 480 x 300 drawn up to 960 x 600): the page boots with every program (119 of 119) and no error; the logs and
  the lying drum read as round with their snow on the top line; the standing drums are round and show their twelve sides close up; the pipes read as
  pipes. NEVER SEEN ON A GPU and NOT TIMED ANYWHERE: the page's fps with `?dress=0` and without is the reading asked of the maintainer.
- **THE MEN ON THE LOOK PAGE, A FIRST ROUND** (`shaders/look.wgsl lookMan`, called from `gbuffer.wgsl fsSkin`; the maintainer, 10-23: "we need to spend a
  LOT of time using these layers to make the characters look awesome. I think that's the next big frontier"). A man was one flat colour a material
  slot with the mesh's creases; here each slot is drawn as the stuff it is, in his BIND pose (nothing swims as he moves), by his side and a number of
  his own: cloth (thread from the cloth map round each limb, folds that ring the limb and gather at knee, hip, ankle and elbow as a relief the lamps
  light, an operator's print cut from his own colour, dust on the knees, age), webbing (rows of tape, rubbed edges), helmet (cover, chips), metal
  (bright edges, an oiled sheen), boots, skin (tone, the cold in the face, a day's growth). No snow is drawn on a man: a CAP on helmets and shoulders was here until 10-24 and snow caked on the boots and hems until 11-02, both out on the maintainer's word ("let's also remove the snow on boots"). Carried by one float: `vsSkin` packs slot + 16 x side (and, since round four, + 128 x the cast part) into `fin.w` on that page only (`charMaterials.ts charLookSides`, raised by
  main.ts with the look), where the build keeps 1. Amount: the panel's Men row (`look=men:0` = the men as the build draws them; `window.__look.set({
  men })` for a scripted picture). KNOWN GAPS: the tilted normal is stored as a man's normal always was (alpha 1), so the working-size passes read
  the folds too: on a GPU look for crawl on cloth under the denoiser; the print and folds are noise, which is the "procedural" he named, so authored
  maps for the men are the next round; nothing here has been seen on a GPU.
- **THE MEN, ROUND TWO: TAILORING** (`shaders/look.wgsl manCut` and the slots' own blocks in `lookMan`; amount `lookCfg.g.y`, the panel's Kit detail row,
  `look=tailor:0` = round one alone). Round one was print, folds and wear, all of it noise, and he had called the page "a bit procedural": round two
  draws what a person CUT AND SEWED, placed by hand on the body in the bind pose. The cast stands in a T: an arm runs out along x round (y 1.435,
  z -0.068) from |x| 0.22 to 0.70, a leg up along y round (|x| 0.093, z -0.045 at the shin to 0.01 at the thigh), an operator's helmet is y 1.65 to 1.86,
  his boots y 0.03 to 0.27, his knee pads y 0.47 to 0.60 (measured from the cast, not guessed: see the preview below). OPERATORS: a combat shirt (the
  body under the plates plain knit, only the sleeves printed; collar and short zip), a sleeve pocket with its flap and a loop field carrying a patch
  and a tape, doubled elbows and knees and seat, the out seam stitched twice, cuffs and hems; the webbing's tape sewn down every 38 mm (a bar tack in
  colour and relief: this is what makes a vest read as load-bearing kit); a helmet cover with four seams from the crown, a band with two cat's eyes at
  the back, loop squares on the sides and a rubber rim; sand-coloured rough-out boots with laces between eyelets, a padded collar and a welt (side 0
  only: the dark suit keeps dark boots and gloves); knee pads as hard caps scraped pale; gloves with a sand back, a dark palm and a knuckle bar. THE
  OTHER SIDE: a shirt's placket with buttons, collar band and cuffs; loose trousers and the shirt's skirt hang in long vertical folds (relief); the
  waistcoat is wool in a twill; headgear is felted wool. The shader has slot and side but NOT the part, so a shawl or scarf in `cloth_plain` gets the
  shirt's lines too, and every hat is "wool": a part id in `fin` is the next step if that shows (round four carries it, for the webbing). WGSL TRAP: `patch` and `loop` are reserved words.
  A PREVIEW WITHOUT THE PAGE made this round possible (a still of the men costs ten minutes here): rasterise the cast in its bind pose under bun
  (`tools/qa/headless.ts loadCastCharacter`, `charMaterials.ts charSlots`) into point / normal / slot / cavity buffers for a front, back, side and top
  view, then shade them with a numpy stand-in of `lookMan` (two seconds a man); design there, port to WGSL after. The stand-in is not in the repo (it
  would drift): the shader is the truth. Never seen on a GPU; on the page only the software renderer has drawn it.
- **THE MEN, ROUND THREE: THE PRINT, AND CLOTHES THAT WERE WORN** (`shaders/look.wgsl manCamo2`, `manPrint`, `manWorn`; the print's amount is
  `lookCfg.g.z`, the panel's Printed cloth row, `look=print:0` = the first print; the worn clothes ride the Kit detail row). The maintainer's second
  point was "we also need to get away from the pixel art textures", and the first print was that on a sleeve: one value noise cut at a level, whose
  blobs come out square-shouldered along the lattice and read as big pixels up close. `manCamo2` is the print as it is on the bolt: a ground that
  drifts from tan to sage with no edge, hard-edged shapes of green and of brown over it, cream flecks, dark twigs in the green; each field is three
  octaves and every octave is turned off the last one's lattice (`manRot0` / `manRot1`), which is what takes the squares out. Sixteen noise taps a
  printed pixel where the first print took six. THE OTHER SIDE'S one set of clothes (`manWorn`, `cloth_plain` on every side but the operators'): the
  road in the trouser legs to a ragged line below the knee and on the seat, grime at the cuffs, the collar and where a man wipes his hands, rubbed
  elbows, sweat down the back, a breast pocket with its flap (`manCut`), and on about half the men a stripe in the weave; a responder's duty shirt
  takes under half the dirt and no stripe. The dirt is the side's own grime colour at half strength, so it darkens a pale shirt and dusts a dark one.
  NO SNOW LIES ON A MAN (the maintainer on a GPU, 10-24: "weird that the men are covered in snow, shouldn't happen"): the cap on helmets and
  shoulders is out, after three tries at making it smaller; only the cake on boots and hems stays. Do not put it back. (He saw the cap on a GPU; the build without it booted on the
  software adapter with no console errors and has not been seen on a GPU.)
  The print and the worn clothes were designed in the bind-pose preview, then ported; never seen on a GPU.
- **THE MEN, ROUND FOUR: A PLATE CARRIER THAT IS MADE, AND THE PART REACHES THE PIXEL** (`shaders/look.wgsl manWeb`, `manVest`, `manRigPouch`; rides
  the Kit detail row; `look=tailor:0` = round one's webbing with every other sewn thing gone too: no setting shows round two's carrier beside this). A close picture of the squad showed
  the weakest thing on the page: rounds one and two taped EVERY webbing pixel, so a carrier was a wall of bricks from collar to belt, shoulders,
  packs and straps alike, the "procedural" look he had named. THE PART ID NOW RIDES TO THE FRAGMENT (round two's known gap): `gbuffer.wgsl vsSkin`
  adds 128 × the vertex's cast part to `fin.w` on the look page (`code` = slot + 16 × side + 128 × part, 4043 at most, exact in the float), and
  the page says which parts matter once its cast is loaded (`gbuffer.ts lookParts`: 1 + the index of `op_suit` and of `chest_rig`, 0 = this cast
  has none, sent as `lookCfg.g.w`). `manWeb` sorts a webbing pixel four ways. (1) THE OPERATORS' CARRIER (the webbing of `op_suit`, side 0 only:
  the dark suit keeps its black harness as it was): tape rows only on the front panel (to 1.345 m), the back panel (to 1.385 m) and the
  cummerbund, lower in relief (1.1 mm, was 2.2) and drawn as paler tape on the cloth, not as dark gaps; above them a plain plate bag with a
  seam and the plate's clipped corners stitched through; a loop field on the chest with a name tape (its letters only marks) and a flag in two
  drab tones; on the back a drag handle under the collar (a doubled strap boxed and crossed at each end, standing 2.8 mm off) and a loop field
  with a call-sign tape and a glint square; shoulder straps padded between two rows of stitching, a buckle where each meets the front; a bound
  hem. (2) HIS OTHER KIT (the `kit_*` packs, the chin strap): plain cloth, no tape. (3) A DEFENDER'S CANVAS CHEST RIG (`chest_rig`): no tape; a
  twill, each pouch under a flap whose edge is stitched across at the toggle and stands 1.6 mm proud, a stitched hem. (0) Everything else (the
  responders' kit, a cast with no such parts) as before. Everything on the carrier is placed by bind-pose coordinates on the one mesh the cast
  has, and the part is found by its NAME in the page's boot (`main.ts`): another carrier mesh needs its own numbers, and nothing checks them.
  KNOWN: the pouches on the carrier's front are `op_suit` too, so the tape rows run over them and the front seam crosses the chest pouch; the canvas rig's pouch mask is "the front of him between two heights", so the belt's front arc takes the hem line; a
  pack on a man's back hides the handle and the back's loop field from most angles. SEEN: the numpy preview of `operator_2` and `defender_4`
  (bind pose, four views) and the squad of four on the pass at dawn on the software adapter (very low, one seed, 119 of 119 pipelines, no
  errors). Never seen on a GPU; the defenders' rig never seen on the page at all.
- **BIG HAND-MADE RELIEF: STONE, MUD BRICK, PLANKS, DAUBED MUD** (`render/lookTex.ts`; the look page only; the maintainer, 10-23, sent two stills of a
  2005 stealth game, a stone stairwell and a dark brick room, as the materials reference: "this looks pretty good and it's pretty much all materials,
  right?"). What those stills have that noise does not: FEATURES A PERSON CAN NAME, each a few centimetres to half a metre, under one raking light.
  So the maps were redrawn as things, not spectra: `wood` = eight rough-sawn boards of their own widths with wandering gaps, raised hard grain, knots
  with swirl, checks from the butts, saw strokes on some, grey boards beside brown, nails with rust runs; `mud` = hand daubs with finger strokes and lap
  ridges, a flaked last coat with a crisp ragged edge, one long crack net and fine crazing, straw, pebbles and the holes they left. Two NEW layers, 11
  `field stone` (a jittered lattice of stones each proud by 2 to 5 cm with its own lean, clefts, chips, four rock colours, lichen) and 12 `mud brick`
  (sixteen courses, a wandering bond, washed-out joints, crumbled and missing bricks). They are on no class: `look.wgsl lookSurface` lays them INTO a
  mud wall wider than 0.6 m and taller than 0.24 m, stacked pieces included (the MASONRY block): a plinth of field stone to a wandering line about 0.35 m up with a
  drip shadow under the render's lip, and mud brick showing in ragged holes where the render has fallen, more of them low on the wall, each with a
  lit lip. Panel row Stonework (`look=stone:0` = plain mud walls). `LookCfg` grew to 7 vec4 (f.w stone, g.x marks); `LOOK_ROUGH` is read for the first
  eleven layers only (the two new ones carry their base roughness in the block). TRAPS: a coursed map must never sit on a layer `untile` reads twice
  (the second read is turned 0.61 rad: courses cross); the plinth's line is in world y, so a mud wall on a slope would step. A map can be judged
  without the page: bake `lookLayer(n)` under bun and shade it flat, raked and top-lit (2 s a layer) before any still is asked for. Nothing here has
  been seen on a GPU.
- **THE MATERIAL MAPS ARE THE LOOK NOW** (the look page only still; 11-01, after the maintainer's yes to walking back "no image textures"). `STYLE.md` says what a
  map may do (rule 1, Surfaces, Not the look, Holding new work to it), and four things the prototype had wrong are rules with a check. (1) WHAT A BOX IS MADE OF IS THE
  BOX'S OWN WORD. `render/gbuffer.ts lookStuff(albedo)` is the rule `lookClass` wrote out for the shader (the palette's row, then a classed paint darkened in one piece;
  the numbers are `LOOK_MATCH`), asked on the CPU of the box's paint AS IT WAS DRY. `gbuffer.ts lookWordOf(dry)` makes the box's look word: bits 0 to 3 the stuff, bit 4
  "it moves" (a box past the level's statics), bits 5 up a number of the box's own, 0 to 90 (from its motion key or size if it moves, from where it stands if not).
  `BoxWorld.setStuffOf` (the engine on the look page; null anywhere else, so the mat word on every other page and every headless stand is as it was) puts 8 × the word
  above the finish in the mat record's last float (`common.wgsl boxLook` / `boxStuff` read it, `boxFinish` takes it off under `LOOK`; a finish there is good to a
  500th). The shader used to ask the colour it was handed, which is the paint × the rain's `wetK` 0.5: a wet concrete slab was nobody's colour and drew flat, wet
  tin was block. THE BIG LOT IN THE RAIN is soaked by overwriting its `albedo` (`weather.ts soak`), so the page hands `lookWordOf` the weather's own record
  (`Weather.dryPaint`, wired in `main.ts` once the game stands; a box with `aniso` set is asked there first). `lookClass(colour)` is left for one caller, an old
  break's core in `gbuffer_cut.wgsl`, which is a colour in a cut's record and not a box. STILL BY PAINT: a level file's `material` name is thrown away at load and
  the stuff is found again from the colour, as it is for a round; ten pairs of the palette collide when one is worn dark to exactly the other (the check counts
  them: mud × 0.95 is coyote, tin × 0.70 is block …). Landing wants the name kept.
  (2) A MAP KEEPS ITS PAINT'S VALUE (`lookTex.ts lookLayer`, the row `even`): two things in a map multiply the paint. The multiplier of a layer under
  `LOOK_PAINTED` is scaled by ONE number so its mean value (luminance) is 1; one number and not one a channel, which gave clean metal a blue cast in place of a
  rust one. The hollows were `0.5 + ½ open`, never over 1, so every map was darkened a second time (gravel drew a tenth under its paint): they are stored as
  ½ open + what puts their mean at ½, the mean weighed by the multiplier (worn wood's high places are its pale ones: counted texel by texel the two together
  drew 3 % over), and the shader draws `0.5 +` the channel. Drawn together a painted layer is its paint within 0.2 % at any Cavities setting. The two masonry
  layers are another stuff than the paint over them and keep their colour; their hollows are evened too. WHAT MOVES IN THE PICTURE HE HAS SEEN (value, at the
  page's rows): cloth and bare metal about a sixth brighter (18 %), sheet an eighth, gravel and wood a tenth, painted metal 6 %, slab and plaster 3 to 4 %, mud walls 2 %;
  earth 4 % DARKER, block as it was. On the software page (the pass's kitchen under the grey sky, one frame with the row and one without) a floor slab read
  4 % brighter and a mud wall 3 %, a wooden table 17 % and a crate 23 % (dark things also take the brighter room's bounce), and the two men within half a per cent. THE MEN DO NOT MOVE: their cloth was tuned on his GPU against the cloth layer as it fell, so `lookMan` multiplies the layer's
  tone back in and takes its hollow back out (`lookCfg.h.x`, `.y`, written by the fill when layer 8 is made). OFF WORD: `?look=even:0` and a reload = every map's
  texels as its material wrote them (read when the maps are made; the panel's row "Maps keep their paint" says so and draws nothing when moved). It is the maps
  as they were and NOT the whole picture before 11-01: (1) and (4) are mends of wrong answers and have no row (a wet slab is classed, a mover is in its own
  frame, a static's shift is by where it stands), so the page before them is a checkout of the commit before.
  (3) THE MAPS ARE MADE WHILE THE SHADERS COMPILE (`GBuffer.fillLook`, one of `Engine.init()`'s jobs, a turn of the event loop between layers): 6.5 s of arithmetic
  under bun that used to run in the constructor, in front of the first compile, with the start card frozen. No frame is drawn before the last layer is in. It is
  its own boot mark (`pipelines_materials`), so on a warm boot the card can stand at N of N pipelines for a few seconds while the last layers are made; a layer
  is still half a second in one piece (a Worker would end that; not done). `tools/qa/wgsl-dump.ts` waits 180 s for init because of it.
  (4) A THING THAT MOVES CARRIES ITS MAP: a box whose word says it moves is laid in its own frame whatever its stuff, shifted by its own number, and takes no
  masonry. By the box's place in the list (as the shift was, `bi % 7`) every door's boards jumped in the frame a breach appended a static. Known, left: a device's
  boxes are keyed by their order among the devices' boxes, so a tripwire's beam coming or going still moves the number of the device boxes after it (most carry a
  turn and take no map); two unkeyed movers of one size share a number.
  CHECK: `bun run tools/qa/lookmaps-check.ts`, 14 lines, the same on both stands (it stands no rule up); `LOOKMAPS_BREAK=1` = the row at 0 and four lines
  fail (the value, paint × hollows, the masonry's hollows, and the rain's line, which under the word asks a wet thing's stuff of the colour it is handed).
  THE RAIN'S LINE AND WHAT IS WET (11-04): since everything flat under open sky is wet in rain (`game/wetGround.ts`), a wet thing may be plain paint laid on the
  ground (a weed, a rug, a mat, a stain: 36 of the compound's 177 wet things, 41 of 186 on the page's stand), of no stuff dry or wet. The level is
  packed dry and each thing's word kept, then the rain is picked and it is packed again: every wet thing carries the word it carried dry, its paint's;
  some of them are of a stuff, and the ground's own slabs always; a wet metal thing is handed its own colour. The lines: the value by one number with the hue kept; paint × hollows at three Cavities settings;
  the masonry's hollows; the row's off word and the men's cloth handed back to a texel's rounding; the masonry's own colour; no seam; normals, finishes and far
  copies; every palette paint its own row's stuff; worn paints and the counted collisions; the word through the real pack on the smoke lab's floor (stuff, the
  moves bit on dynamics only, no mover's word changed by an appended static); the rain through the game's own weather on consulate-gwot's gravel lot; the
  shaders' and the page's text; the fill's 260 writes with none at the making and a wait on a message before each layer (counted by the channels the fill makes: bun hands a port a
  thousand messages in one go, so a count of the loop's turns from outside said nothing); the block's size and its last row. Only the three `even` lines have a
  break path. No `FROZEN_HOLDS` row: the maps are a page's
  picture and none of this is the game's state. SEEN: one boot on the software adapter. Nothing of this on a GPU. NOT DONE: a material name kept from the level
  file, which is the root of what is left (the stuff is re-read from the colour at every pack through a cache, the weather is asked through a hook, any other
  writer of a box's paint still changes its stuff, and `lookStuff` and the generated `lookClass` are two writings of one rule that no check holds together);
  the cut core's class on the CPU; the maps made off the main thread or kept between visits; the far copies' cost (31 MB became 36 with thirteen layers); relief past the tilt cap.
- **A PAINT'S NAME SAYS WHAT IT IS MADE OF, FIRST SLICE** (`src/scene/stuff.ts PAINT_STUFF`, `render/gbuffer.ts lookNamed` and `lookWordOf`; the
  maintainer on the showroom, 11-04: "nicer materials on the chair and table"). The look decided what a box is made of from its COLOUR, so the
  office's paints, which are the simulation's own names and no palette row, were nobody's: a desk, a chair and a carpet drew flat. Now ONE TABLE
  says what each paint NAME the kits write is made of: a material of the look (`lookTex.ts LOOK_NAMES`), `plain` (meant flat: plastic, laminate,
  rubber, a screen), `waits` (no material yet: card, foliage, a lawn, a clean steel; drawn plain) or `mixed`. The palette's rows moved into it
  (checked against `GwotMat` by the compiler) and `LOOK_OF` is read off it, the same classes in the same order, so the shader's text is unchanged.
  THE LEVEL'S OWN ROWS carry the names: the loader keeps a file's `materials` beside the level (`levelLoad.ts levelFileOf`) and the page makes the
  look's word for a box with them (`lookWordOf(dry, rows)`): a paint a row names is its name's stuff, asked before any colour is weighed. A box
  and a loose prop's part are written under the FIRST row that has their colour exactly, so the row is found by the colour and no box carries a
  word. THAT IS ALSO ITS LIMIT, and what `mixed` is for: a file's writer names a box by its colour, and the kits have used one colour for two
  things. The showroom's car body is written as the blue partition, a hut's stucco as the tan one, the lived kit's iron fittings as the car park
  (277 boxes to 20 slabs), an outside door's leaf has the window frame's numbers. Those six names (`partRed`, `partBlue`, `partTan`, `red`,
  `asphalt`, `glassFrame`) are left to the colour rule, as before there was a table, and so is every paint written as numbers (door leaves and
  frames, the lived kit's toned paints, cars). A NAME NEVER CHANGES A COLOUR: the simulation reads colours and reads nothing here. `?names=0` on
  the look page = by colour alone, as it was (a word of the look page: a benchmark carries it). BOTH STANDS: nothing, it is no rule of play and
  the harness cannot see it; no knob, no journal key. No shader text, no level file moved. CHECK: `bun run tools/qa/stuff-check.ts` (`STUFF_BREAK=1`
  must fail): every paint name in every level file has a row (90 names in 59 files: 57 a material, 21 plain, 6 waiting, 6 mixed); the palette's
  classes as they were; on every file a box wearing a row's paint is drawn as the first row of its colour says, standing or moving (3948 rows,
  649 of them something else by colour alone); the office's paints; the car body, the leaf, the iron and the stucco are the colour's; the page's
  wiring. `lookmaps-check`'s text test of main.ts takes the second argument. NOT YET (the second slice, `stuff-by-name` in the dressing notes): a
  word on a box, which is what parts the mixed names and names a paint written as numbers; the level format's `stuff` on a box, a part and a row
  (a level's accent as a material); the cut core's class in `gbuffer_cut.wgsl` (still `lookClass` of the colour: an old break in an office
  partition draws plain); `?surf` retired; a row on the look panel. THE LOOK'S FOUR OFFICE MATERIALS (veneer, plastic, fabric, steel; asked for this, 11-04) have their rows where a
  name is one thing: `chair` is fabric, the four `plastic*` plastic, the office's `metal` steel. KNOWN: `wood` stays planks, because the name is a
  bench, a crate and a pallet as well as a desk: a desk says veneer itself, with the second slice's word on a box. The class of a stuff is asked of
  `lookTex.ts lookStuffOf`, the one table. Seen on the software adapter, and the first three commits of it by the maintainer on his GPU through the preview.
- **A BOX SAYS WHAT IT IS MADE OF, SECOND SLICE** (`Box.stuff`, a level file's `stuff` on a box or a prop's part, `src/scene/stuff.ts BOX_STUFFS`;
  the maintainer, 11-04, of the showroom's desk and chair: "nicer materials on the chair and table"). A paint's NAME cannot part a desk from a
  bench (both `wood`) or a car body from a partition (a level file names a box by its colour), so the PIECE says it: `lookWordOf` asks the box's
  own word first, then its paint's name, then its colour. One of the look's materials that has a number (`lookStuffOf`: the stone and brick
  under a mud wall are refused), `plain`, or `waits`. PICTURE ONLY: no rule of the game reads it, a level's fingerprint does not, and
  `lived-check` reads a base's boxes with today's words. `?words=0` = the boxes' own words off and the names on (the picture before this
  slice); `?names=0` puts both off. Written in the file only where it was said (after `metal`, before `flags`).
  THE RULE: A WORD BELONGS TO THE PAINT IT WAS SAID WITH (`stuff.ts saidFor`). (1) A piece says a word for its OWN paints only: the kit's desk
  is veneer in the kit's two wood paints and says nothing when a caller hands it steel or white (a steel bench, a white table: that paint's
  name speaks); a chair says nothing itself (the kit's chair paint is fabric by name) and a caller that hands it a paint of its own says
  what that is (the manager's dark chair). A piece that is one thing in any colour says so always (a rug is cloth; floor paint and a dirt
  track are NOT rugs and are laid as their own flat boxes). (2) Whoever REPAINTS a box drops its word: the look's skin
  (`tools/levels/gwotSkin.ts`, boxes and prop parts; `look-check` counts `stuff` as paint), the editor's paint commands, the cars trial.
  (3) Whoever COPIES a box's paint copies its word: a holed wall's standing and fallen pieces (`wallbreak.ts`). A loose prop's part carries
  it to the box the game makes (`props.ts`), and the page asks those boxes their word before the first frame so the map is made (`main.ts`).
  SAID SO FAR, by the office kit (`officeGen.ts`, `levelKit.ts`): desks, the reception counter, low table, conference table and credenza
  veneer; cubicle partitions, couch and sofa fabric; cabinets steel; rugs cloth. NO DOOR FRAME says a word: a frame goes with its leaf, and
  both wait for the doors' own slice. The level files of the generators in step with their files are re-written by words alone.
  CHECK: `stuff-check` lines 7 and 8 (`STUFF_BREAK=1` red on 7): a word over a name and a colour, through the file and back, on the box the
  game makes of a prop's part, refused on a box, a part and a gear screen's box; the desk as itself, as a steel bench and as a white table;
  no frame with a word in any level file; no word kept on a box a look's file repainted; the editor's paint drops it. NOT YET: a cut core
  takes its box's word; the lived kit, the compound's stucco and door leaves and frames say theirs; a row on the look panel; the editor
  shows no word. Seen on the software adapter (the showroom and the office); never on a GPU by us: the maintainer has it on the preview.
- **A THING IS MADE OF WHAT IT SAYS: NOTHING READS A MATERIAL OFF A COLOUR ANY MORE** (11-04; `src/scene/stuff.ts`, `scene/levelLoad.ts`,
  `render/gbuffer.ts lookWordOf`; the maintainer: "remove as many hacks or special cases as possible"). This ENDS the three bullets above it
  (the name table's first slice, the box's own word, `lookStuff` and the generated `lookClass`): they are the record of how it came about.
  THE RULE, WHOLE: a level file's material ROW says `stuff`; a box or a prop's part is made of its own word, else its row's, else it is plain
  (the loader puts the word on the box; a box written as numbers has no row and says its own); the look reads `Box.stuff` and nothing else.
  GONE: `gbuffer.ts classWgsl`, `lookStuff`, `lookNamed`, the palette match and its darkened test (CPU and the generated WGSL `lookClass` /
  `surfClass`), `levelKit` and `officeGen FINISH_BY_ALBEDO`, `?names=0`, `?words=0`, and SURFACES, A TRY (`?surf`, `SURF_TRY`, `surfTryConfig`;
  the look page replaced it; `shaders/surface.wgsl`, the last of it, went with the next bullet).
  IN CODE A PAINT IS KNOWN BY THE ARRAY HANDED OVER, never by its numbers (`stuff.ts namePaints`, `paintOf`): a kit's palette row carries its
  name, its finish and its word; a COPY of the numbers is nobody's paint, matte and plain, and nothing in the types says so: hand the row, tone
  it with `toned` / `tonedAs` (a tone of a paint is that paint's stuff), stand in for it with `gwotAs`, repaint a box with `repainted`, and
  copy a box's paint to a piece with `boxes.ts wordOf` (debris, a broken wall's cells, a blown table, a vehicle's parts). A piece's own word
  goes with its own paints by the same rule (`saidFor`: the row, not its numbers). A colour written as numbers says its word beside it:
  `madeOf('cloth', [...])` where somebody chose, `saidAs('stucco', C.partTan)` for one paint that is a second thing, and
  `drawnAs('slab', [...])` where NOBODY chose and the word is only what the colour rule made of those numbers (grep `drawnAs(` for the
  list still owed a decision: 51 in 17 files, officeGen 15; none of them is a word a round reads falsely, below). `PAINT_STUFF` has no
  'mixed': a name says one thing, and a row's word is what the row is (asphalt says plain). A door's leaf says its paint's word
  (`lookTidy.ts lookDoors`; a `look.ts LOOK_DOORS` row says one a side). An old cut's core says its stuff in the cut record (`cut.ts
  CUT_CORE_STUFF`, bits 12..16). A SHAPE'S PIECE (`scene/shapes.ts`, a box's `shape.with`) says its word as a box does, its own or its
  paint's row's, and the box stood up for it carries it. A TONE OF A PALETTE ROW is made of what the row is: the kit's `toned` (`gwotKit.ts`)
  and the snow levels' rock (`tools/levels/snowRock.ts`) hand their numbers through `tonedAs`, which is how ground dressing, stains, worn
  paths and cut rock keep their earth, gravel and slab. The look page's hand-built pieces (`frontTry`, `lorryTry`, `passTry`) are made of
  palette rows and say their words. The editor keeps, copies, sets and shows a paint's word (`editor/paint.ts`, the paint panel's 'made of' row), a
  said plain included; a blank document's paints say theirs.
  LEVEL FILES: every level file of the build is its generator's output, so each was written again by its generator
  (`tools/levels/export.ts`, `tools/level-export.ts`). `tools/levels/stuff-bake.ts` brings forward a file laid WITHOUT its generator (the
  old rule frozen in `tools/levels/stuffWas.ts`, for that tool and `tools/qa/stuff-audit.ts`'s before side only: delete the three when
  nobody holds a file from before; `tools/levels/paintUsers.ts` stays).
  WHAT WAS CHANGED ON PURPOSE, because a word is now read by more than the look (next paragraph) and these were the colour rule's
  mistakes, each read by the level's own task or the hit table's: THE CAR PARK's paint, `asphalt`, says plain (no stuff the look has is
  tarmac yet; its grey was drawn as bare iron, and the lived kit's iron fittings in the same numbers say metal themselves); the compound's
  packed-earth yard says earth (drawn as wood) and its house's walls stucco (the partition's tan, drawn as cloth); the lot's kerb is slab
  (`KERB_GREY`, drawn as sheet); the vending machine's body is paint, the dumpster's lids plastic, a sofa fabric, the anti-fatigue mat and
  the lived kit's rubber ink rubber, a quarry-tile floor slab, a gravel hardstanding gravel, sandbags cloth, an exhaust metal; a picture on
  a wall, black file folders, mugs, a planter trough, pull strips and three vehicles' bodies say nothing (the vehicles' words are the
  vehicles task's); the registry's lacquered leaves say paint and the consulate's varnished ones wood.
  NO SWITCH: it is not a rule of play and both stands draw the same. THE CHECK: `bun run tools/qa/stuff-check.ts` (14 lines;
  `STUFF_BREAK=1` strikes the rows' words and fails lines 2, 3, 4, 7, 13 and 14; line 14 is a shape's piece; line 10 holds that in `src/game` a box's word is READ in at
  most one place, the picture of a hit) and `bun run tools/qa/stuff-audit.ts --before <ref>`, which prints what every box was and is, by
  kind and file, and exits 1 where one lost or moved its material: against the head before this series it lists the 1,046 below and exits
  1, by design.
  MEASURED ON THE BUILD: the 58 level files as they stood, read by the old rule, against their generators' new output: 41,285 boxes and
  parts, 40,239 drawn as before and 1,046 not. What the kit and the generators SAY now, where the colour rule guessed: 286 desk and table
  tops veneer where their paint's name said wood, 73 cabinets and lockers steel where their white was taken for paint, 130 chairs and bins
  plastic, 54 seats fabric, 82 crate slats wood where their colour was a mud wall's, 81 tyres and mats rubber; 138 dark things drawn as
  bare iron by their colour say nothing (black plastic, file folders, the asphalt a level stands on); and the single pieces named above.
  A paint handed over by lineage (toned, stood in for) keeps its word: 209 pieces of ground dressing and cut rock kept theirs only once the
  kit's `toned` handed its numbers through `tonedAs`. No box's colour, size or place moved (`lived-check` holds every level's own part to
  its base with the words set aside).
  A ROUND'S HIT WILL READ THE WORD, AND IS NOT THE LOOK: the hit picture's rule (HITSTUFF, `game/particleFx.ts saidSurf`; it joins after
  this series, with its check) reads `Box.stuff`. Measured where both stood together, on the look's preview: with every box saying its
  row's word that rule heard 18,701 of 37,425 standing boxes (389 before), and on 4,295 of them another surface than the paint's colour
  gave: sandbags, tarps, rugs and carpets the soft thing where they threw concrete chips, mud walls concrete where drywall, wood furniture
  wood where concrete, iron and sheet things sparks; picture only, the run's digest the same with the rule up as down. That is why the
  words above were corrected and not kept as drawn. Of the 51 words nobody chose, a round read 13 otherwise than the colour and each read
  true (duckboards and pallets wood, a coffee machine metal); the depot's walls in the partition's tan still say cloth. NOT MEASURED ON
  THIS BUILD: the rule is not in it yet. On the page every leaf in one paint is one stuff (leaves whose own shade was at or above their
  paint's were plain before).
  NOT COVERED: thrown and dropped kit, a gun model's parts with computed colours and a magazine in the hand say nothing and are plain
  (not measured: whatever their colour happened to match before is gone). No debris is written to a checkpoint; a broken wall's or a blown
  table's slice is re-applied after a restore by the code that copies the word. An editor document kept in a browser from before says
  nothing in its rows and draws plain on the look page until it is brought forward (`stuff-bake.ts <file>`). Still read off a colour, and
  OUT of this: what a round, a charge and a blast make of a box (`STYLE.md`: the game reads paint). NEVER SEEN ON A GPU by a person. On
  the look's preview a cloud card drew eight scenes before and after with no error, the same pictures but for the words changed on
  purpose (an office car park no longer rusted iron); as joined to the build it has been drawn by nothing but the shader compiler.
- **THE LOOK'S SHADER NAMES NO STUFF: WHAT IT DID BY NAME IS A WORD IN THE STUFF'S ROW** (11-04; `render/lookTex.ts STUFFS`, `gbuffer.ts
  lookWgsl`, `shaders/look.wgsl lookSurface` and `lookSnow`). `look.wgsl` had eleven constants that were one stuff's number each
  (`SURF_MUD` … `LOOK_CARPAINT`) and seven tests of a box's class against them; a new stuff could not be a mud wall's cousin without a
  line of shader. Each is now a row word, read through the prelude: `runs` (which way a thing's grain or ribs run: 'long' on every face,
  boards and a veneered top; 'flat' along a lying face and up a standing one, a sheet), `least` (half a face's width under which the map
  is not shown: a bucket is not a roof sheet), `under` (what it is made of under its coat, with the least wall that shows it: a mud wall's
  field-stone plinth and mud-brick core, MASONRY), `foot` (what works up through it at a wall's foot: gravel through earth), `ground`
  (bare ground that freezes by lying snow), `sheds` (snow does not lie on it: bare iron). `LOOK_WITH`, the layers a layer needs made
  with it, is derived from `under` and `foot`. Two tables that are not a stuff's (a plain box is rubbed too): `LOOK_HANDLED`, what a hand
  moves has its arrises rubbed (the "edges under 1.3 m" rule: 1.3 long, 1.6 tall, 4 cm thin), and `LOOK_GROUND`, where the ground is for
  what the weather does (a top under 0.25 m, the snow field's 0.06, a foot under 0.12 stands on it), metres above the level's floor.
  The shader keeps the stuff's row at the head of its stuff branch (`let row = …; let thing = LOOK_THING[row]; …`) and tests a class only
  against plain. `shaders/surface.wgsl` is deleted. STILL IN THE SHADER'S OWN WORDS, not this slice's: the heights of USE AND DIRT, the
  one answer a mark (a lane, the band at a wall's foot, the skirting, the knocked coat: 0.25, 0.3, 0.15, 0.14 m, the look task's block),
  the shapes of the effects themselves (how high a damp line wanders), and `LOOK_L_CLOTH`, the layer a man's clothes are woven from.
  NO SWITCH and NO CHANGE OF PICTURE meant: every number is the number it was. THE CHECK: `lookmaps-check` line 9 holds that the shader's
  text has no stuff's constant and no class test but against plain, and that for each of the 28 stuffs of that day the row says exactly
  what the named rules said, six words each, the old rules written out in the check (`was`: a row changed on purpose changes it there);
  `wgsl-compile`, all 68 texts. Never seen on a GPU by a person.
- **MARKS AS PAINT, A FIRST TRY** (`shaders/markings.wgsl markSoft`, behind `lookMarkSoft()` = `lookCfg.g.x`, 0 off the look page by gbuffer.ts's
  stub; the maintainer, 10-23: "we also need to get away from the pixel art textures"). A marking is still the level's 2-bit cell bitmap (2.5 cm
  cells), but on the look page it is not drawn as squares: the coverage is read over a one-cell footprint, cut back at half cover along a ragged line
  (noise by box and marking, so two signs do not share an edge), the shares sharpened so two colours meet in a line and not a blur, and thin rubbed
  patches let the surface through. An emissive entry keeps its cells (a lamp's face must not lose light), and past about 1.75 cells a pixel it is the
  plain coverage again (nothing to draw). Panel row Painted marks (`look=marks:0` = square cells; what the row does today: THE ROW ON A GPU, below). The bitmaps themselves are still the level's: a
  mark that is a coarse picture stays a coarse picture with soft edges, so authored marks at a finer cell are the round after. Never seen on a GPU.
  THE SOFT READ IS ITS OWN (`markSoftRead`, 10-24), one read of the four cells around the pixel. (1) A PLAIN BILINEAR RAMP between cell centres.
  The first try read `markTaps` at a one-cell footprint, which is already a quarter into its minified mean: 0.75 of the ramp + 0.25 of the nearest
  cell, a step on every cell edge, so the cut sat on the cell edges and a staircase stayed a staircase with round corners. (2) A STROKE THAT RUNS
  CORNER TO CORNER IS JOINED: where two cells of one paint meet only at a corner and the other two of the block are bare, the ramp is exactly half
  cover at the corner (a stepped diagonal was a string of beads). The bridge adds cover there for that paint (a neck about 0.8 of a cell, as wide as
  the stroke), so the pass gate's diamond is one painted line. NOT joined: two solid blocks meeting at a corner (the rugs' chequers of 2 by 2 squares;
  the same rule leaves two bare cells meeting inside a painted field alone), a chequer of single cells, a paint that burns. Counted over
  `levels/*.json`: 6,903 such corners in 8,075 marks; 4,608 are block corners and 36 a single-cell chequer (bazaar.json), left as they were; 2,259
  are joined, in 748 marks. Among those are specks that touch at a corner (the dots of writing on a paper, rust speckle), which now fuse. KNOWN: it
  reads its own record, so a corner on the seam between two records of one face is not joined (347 of them, most on rugs laid as four records, where
  the block rule skips them anyway); the bridge is paint the far read (the plain coverage past 1.75 cells a pixel) and a reflection (`markHit`) do
  not have, so a joined stroke is about a tenth heavier up close than far off. A wider smooth read (a B-spline over the cells, cut at 0.4) was tried
  first and DROPPED: it filled one-cell holes (the police post's lattice), washed out a lone cell of one paint inside another, and did not keep the
  cover. Seen on the software adapter at the pass gate (the diamond is one line, the bars have a brushed edge), a rug (the lozenges are slanted and
  round, no steps) and a crate; never on a GPU. TRAP: the corner test first read its eight cells in two chains of `&&` over `markCell` calls, and
  with that the software adapter never finished the look page's first frame (three boots of three; every pipeline compiled, no error). It is one
  call site in a loop over `MK_BRIDGE` now. A page boot that reports its pipelines says nothing until a frame is drawn.
  **THE ROW ON A GPU (10-25).** First: "turning on and off painted marks doesn't seem to do anything?" A test picture at the top of the row (red = how
  near the soft read counts the pixel, green = its cover, blue = an entry that burns) showed the soft read working on his GPU: red patches, smooth green
  lines, the diamond one slanted line. Then, with the read proven: "painted marks still doesn't look like it does anything until 2 where it's really
  overdone" (2 was that test picture, read as the effect). CAUSE: at the distance the game is played from a cell is about two pixels, so a rounded
  corner or a ragged edge is under a pixel; only a change of tone or of a whole stroke shows. THE ROW NOW (`markSoft`, 0 to 2, 1 the default; the test
  picture is out): the SHAPE as before (all of it by 1); the HAND, the cut wandering slowly along the stroke (`0.5 + 0.32 × row × (noise - 0.5)` over
  20 cm, clamped 0.22 to 0.82: a stripe a cell wide is 0.85 to 1.15 of one at 1); the WEAR, paint rubbed thin in patches of 9 and 30 cm measured in
  METRES on the face (`pm = cell × texel`: a fine mark and a coarse one wear alike), a quarter of it touched at 1 and half at 2, up to 0.85 gone, each
  patch fading at its rim; the AGE (`markTone`), the paint's own colour 0.82 of the frame's marking contrast at 1 and 0.64 at 2. A burning entry keeps
  cells and colour. JUDGED BY EYE on the software adapter, the pass gate at midday, 480×300 at scale 1 from 5.2 m (his cells to a pixel): 0 = crisp
  stripes and a stepped diamond; 1 = stripes nearly straight with faded lengths and round ends, the diamond a drawn line; 2 = stripes half gone. A
  first cut swung the stroke's width more than twice as far over half the length (0.7, 11 cm): a row of stripes read as dripped wax, so it was halved.
  KNOWN: the wear is noise by box and marking, not by where a thing is rubbed (a gate's foot, a rug's middle); writing made of single cells loses
  letters at 2; the far read and a reflection (`markHit`) have none of it. The reworked row has never been seen on a GPU.
- **A CAPTURE IS EVERY TARGET OF A FEW FRAMES, AND THE TRACE OF WHAT THE RENDERER CHANGED** (11-02; `src/render/framecap.ts`, wired in `main.ts`; every
  page, nothing read by the game. The maintainer: "any telemetry or like tracing we can add to the graphic system to make it easier to debug the drops …
  being able to get all the frame buffers … in a recording mode or something so we can debug stuff together and maybe also even try to optimize"; before
  it, a flash at midday "as if it's throwing away the temporal buffer every so often" that showed in no debug view). HOW TO TAKE ONE: **F6** = THE WHOLE PICTURE over 2 frames, from the frame
  after the key (since 11-03; the maintainer, of the window at the cursor it was: "can you make the captures for the entire frame? i don't want to have to
  think about the cursor. and you can make it just start recording when we start it, it doesn't need to go back in history"; his first two captures had the
  cursor on bare ground and not on the man he meant); **Shift+F6** = a window of the picture, 256 output pixels square at the cursor, over 8 frames (a tenth
  the size: for a thing that flickers in one place); the look panel's Quality section
  has both and "at next change"; by console `__framecap.start({ frames, px, whole, at: [u, v] })` (`whole: false` = the window), `__framecap.arm({…}, kinds?)` (waits and starts on
  the frame the renderer next changes something, or only on the named kinds, e.g. `['long']` for a long frame, `['targets']`, `['scale']`),
  `__framecap.stop()`, `__framecap.trace()`. The game's message line and the HUD's first lines say what it is doing and where the file went: `captures/<name>.tpscap.gz` under the dev server's checkout
  (`tools/serve.ts` POST `/capture`, asked only by a page on a loopback host and answered only to this server's own page with the capture's header, as
  the level API is; never over a file of the same name; the folder is gitignored) or, on a page served from anywhere else, out by the page's one save door (`host/savefile.ts`, kind `capture`: to whoever keeps the page's files, the size rule theirs; a download only with `?downloads=1`; the key's line says which in the door's own words). A CAPTURE OVER 24 MB IS ALSO WRITTEN
  IN PIECES beside it, `<name>.part1of6.tpscap.gz` … (11-03: his first whole-picture capture was 137 MB and could not be handed on in one file; the
  message line says how many; `tools/qa/capture-pieces.ts`: a piece is a plain cut of the gzipped file, the reader takes any one piece with the others
  beside it and joins them, and names a piece that is missing or cut short; `bun run tools/qa/capture-pieces.ts captures/<name>.tpscap.gz` cuts one
  that is already on disk; `framecap-check` holds all of it). HOW TO READ ONE: `bun run
  tools/qa/capture-read.ts captures/<name>.tpscap.gz` writes beside it `report.txt` (the trace; each frame's length and state; for every target and
  frame the share of texels that moved, by how much, the share that are zero, mean and max, NaN count; the frames that stand out), `strips/<target>.png`
  (the target's window over the frames, left to right, one exposure for the run) and `index.html`. NO LISTS: `watchTextures` wraps the device's
  `createTexture` once, so a capture takes every live 2D texture that may be copied from, named by its label (a pooled target is relabelled when it is
  taken: `passes.ts take`, `gbuffer.ts`; parked ones are left out, `TexturePool.isParked`), and names what it did not take and why. The render targets
  ask for COPY_SRC where they are made (`passes.ts SU` already did; `RT` and the g-buffer do now). `?framecap=0` (`framecap.ts FRAMECAP`, lowered in
  `main.ts` before the Engine is built) = the targets made without that bit, as they were, no texture watched and the key dead: a usage is a texture's
  from the moment it is made, so the capture cannot ask for it at the key, and the word is how a machine is held against the bit (a replay and a
  benchmark carry it; `framecap-check` holds the switch). The trace has no list either: the page hands
  `state()`, a flat record (internal and output size, scale, quality and the ladder's last line, taau, sharp, the debug view, the targets' re-make count
  and line, hidden), and a change of ANY key is an event with its frame; a frame over 50 ms and 2.5 times the running length is a `long` event; the
  last 900 frames' lengths and CPU milliseconds ride in every capture, with the passes' GPU timings. A new target or a new key of state is in the next
  capture with nothing written for it. A window is the same part of the picture in a target of any size (`cropRect`); a depth target is copied whole
  and cut on the CPU (WebGPU's rule). The copies are made after the frame is submitted. BESIDE THE F9 RECORDER (`ui/perfrec.ts`, the performance
  thread's): that one is timings over minutes, with `scale`, `ladder` and `targets` events of its own; this one is the picture's numbers over frames.
  CHECKS: `bun run tools/qa/framecap-check.ts` (the window, the file there and back, the reader naming the one history that restarts in a made-up
  capture and the one re-made at another size, the trace, an armed start, a stop; `FRAMECAP_BREAK=1` fails 3) and, in the page on the software adapter, `PORT=<port> node
  tools/qa/framecap-page.cjs` (the look page on the pass at midday, high: 91 targets over 3 frames in 4.8 MB, the whole picture once in 14.6 MB at
  640 x 400, no GPU error, both read back; strips of the g-buffer, each light class, the histories and the picture looked at and right). NOT TAKEN
  YET: volumes and layer stacks (the radiance cache, the smoke atlas, the look's maps) and seven small maps made without COPY_SRC (wear, snow, air
  cells, place map, `hdrCopy`). ON A GPU (11-03, an M4 Max in Chrome, 1654 x 1000, high at x 0.70): the first two, windows of 8 frames, were 34 MB
  each with 91 targets and read back whole; F6 reaches the page. A frame's targets there are 286 MB whole and 13 MB as a window, and gzip took a window
  to about a third, so the whole picture over 2 frames should be near 190 MB (NOT RUN: that size, what it costs while it is taken, whether a file that
  big is easy to hand over; `whole: false` and Shift+F6 are the small ones).
- **FRAMES REACH THE GPU IN BURSTS, AND A TRIAL THAT HOLDS THEM BACK: `&pace`** (11-03; `src/host/gpuPace.ts`, `main.ts frame()`; OFF unless the page's
  address has the word, on any page; nothing in the game reads it). The maintainer on his 120 Hz display, the HUD reading over 100: "despite saying
  >100 fps it feels kinda laggy". WHAT HIS RECORDINGS AND CAPTURES SHOW: the page's frame callback runs at the display's rate whatever the GPU is
  doing (the CPU side is about 1 ms), so with the GPU at 13 to 17 ms a frame, where the hold-60 ladder parks it, frames queue and are shown in pairs:
  captured frame lengths 23.8, 6.6, 23.4, 6.5, 22.4, 2.4, 30.5, 2.3 ms, the last 900 frames' median 8.4 ms and 95th 43.6, over 120 frames of 50 to
  60 ms in two and a half minutes; in a benchmark's recording the gaps of 26 ms or more hold 77 to 79 % of the time at a GPU union of 17 ms, and 27
  to 47 % even at 8.8 ms. THE HUD's `fps` IS THE MEAN OF 1 ∕ dt, which a burst inflates: it read 170 to 188 where the frames a second were 53 to 55.
  The same on the plain page and the look page: it is the frame loop's, not the look's. THE TRIAL: a display frame is passed over while N frames
  handed to the GPU are not yet done (`queue.onSubmittedWorkDone`): `&pace` = 2 in flight at most, `&pace=1` = one. The HUD gains a `pace` row: the
  frames a second by the mean frame length, how many are on the GPU, the share of display frames passed over. Checked on the software adapter only:
  the page boots with the word with no GPU error, hands the GPU two frames and then waits for it (a software frame takes longer than the minute
  watched, so no third frame and no `pace` row was seen). NEVER RUN ON A GPU: whether either feels better is the maintainer's to say. THE LASTING
  MEND IS THE QUALITY LADDER'S OWNER'S: the build has its own pacing under the same word (on by default there, `?pace=0` off), and that replaces this
  trial when the look joins the build. AT THE JOIN (11-03) THE TRIAL'S FILE AND ITS LINES IN `main.ts` WERE LEFT OUT, the one place the joined tree is not the
  look branch's: the maintainer had said of it "&pace looks good, let's make it the default", and its rule (a display frame passed over while two are
  still with the GPU) goes into the build's own pacer as one path (`src/host/pace.ts`, by its owner's change), not as a second pacer beside it. What
  this bullet measured stands; `src/host/gpuPace.ts` and the HUD row described above are not in the build.
