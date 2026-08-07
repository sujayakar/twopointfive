# Character / prop viewer

A second page for placing things on the mannequin — the trifocal night-vision goggles on the head, the Five-seveN with
its under-barrel rail light in the right hand, the guards' hand torch — and for eyeballing the animation clips with the
props attached. Every placement number lives in one object, `RIG` in `src/game/rigProps.ts`; the game (`game.ts
handProps`, the rail-light pose, the guards' flashlight origin) and the viewer both go through `rigBoxes()` /
`rigLightPose()` / `rigTorchPos()`, so what you tune is what the game draws.

```
bun run dev      →  http://localhost:5173/viewer      (the game stays at /; /viewer.html redirects)
bun run build    →  dist/viewer.html next to dist/index.html
```

`?quality=low|medium|high|ultra` pins a preset for the visit as in the game (otherwise the shared 'auto' mode applies). Needs the same WebGPU browser.

## What you are looking at

The game's `Engine` (same passes, cascades, denoisers, smoke sim) rendering a procedural stage (`src/scene/stage.ts`:
a 12 m concrete floor with 1 m marks, a back wall 3 m behind the figure, a 0.5 m cube and a 1.8 m post for scale, a soft
key / cool fill / warm rim light) with one `Character` on it. No `Game`, no AI, no audio. Each frame the viewer writes the
same fields `game.ts` writes (`anim.speed / crouchTarget / reverse / upper`, `aimYaw / aimPitch`, `lookYawExtra`), calls
`char.update(dt)`, emits the props with `rigBoxes()` into the engine's dynamic boxes, poses the rail light with
`rigLightPose()` (or the torch with `rigTorchPos()`), uploads the skin instance and the capsule shadow proxies, and renders
through a `FollowCamera` driven as an orbit camera.

## Controls

| | |
|---|---|
| mouse | **drag** orbit · **right-drag** pan (frees the focus) · **wheel** zoom |
| `F` `H` | frame the body · head close-up (panel: body / back / head / hand / feet, and *track* = what the orbit centre follows) |
| `T` `P` | turntable · pause the character |
| `K` | shoot (Pistol_Shoot layer + muzzle flash lights + flame card + gun smoke) |
| `N` `L` | night vision (goggles down + blazing / up + dim) · rail light or torch on/off |
| `G` `1` `2` | show as guard (tint + torch or drawn pistol) · pistol slot · empty hand |
| `Tab` `` ` `` `[` `]` | panel · debug views · exposure |

## Panel

* **Animation** — *locomotion* (idle / walk / jog / sprint / crouch idle / crouch walk / backpedal) × *speed*; *path*:
  treadmill or walk a 2 m circle (real foot planting against the floor marks — with *anim walk / jog speed* this is the
  foot-slide tuning loop); *upper body* layer (none / relaxed / aim / shoot-loop / torch); *aim pitch*, *aim yaw offset*
  (spine twist), *look yaw* (neck — the goggles must follow the skull); one-shots (shoot / reload / hit / throw /
  die (shot) (`X`) — the game's ragdoll, shoved along the camera's view like a round, colliding with the stage floor, wall,
  cube and post / die (takedown) — folds forward / respawn); *raw clip*: bypass the graph and loop any clip
  of the library over the rest pose (props stay on the bones,
  the upper-body layer and one-shots are off there), *scrub* (pause first), *playback rate*, *turntable* and *body yaw*
  (treadmill only — on the circle the heading is the tangent).
* **Kit** — goggles worn, NV on, slot, light on, show as guard + torch / pistol carry, draw props off (bare mannequin),
  gun smoke, rail light / torch intensity, optional NV screen effect.
* **Rig constants** — `copy constants` puts the complete `export const RIG: Rig = {…};` statement (numbers rounded to
  3 decimals, same layout as the file) on the clipboard and into the box bottom-left (`show` / `hide`; if the browser
  blocks the clipboard the text is left selected there). **Paste it over the block between the `paste target` markers in
  `src/game/rigProps.ts`.** `reset` restores the shipped numbers. Below it, one slider per number, 1 mm steps:
  * `RIG.goggles` — `on.fwd / on.up` (lens cluster base relative to the head joint when NV is on), `off.*` (parked on the
    forehead), `lensSide / lensDown / lensUp` (the trifocal triangle), `lensHalf / lensThick`, `housing.back / .half.xyz`,
    `glowOn.rgb / glowOff.rgb` (screen glow + bloom only — the lit lens boxes are raster-only, so they never light the
    face), and `followPitch`: **1** (shipped) makes the offsets and the boxes ride the skull's full rotation (crouch idle,
    aiming down, a ragdolled body all keep the cluster on the face); **0** lays them out in a yaw-only head frame (the old
    behaviour). The boxes take the frame's full orientation either way.
  * `RIG.pistol` — `slide`, `suppressor`, `rail` (+ `under`: drop below the bore along `gunUnder()`), `lens` (+ glow), and
    `lightUnder / lightAhead` = where the actual spot light sits. Turn haze up a little (Renderer) to see the beam leave
    the lens; `debug view → lighting only` shows the spot on the wall without albedo.
  * `RIG.torch`, `RIG.guardPistol` (collapsed) — the guard kit; switch *show as guard* on to see them.
* **Stage & camera** — framing buttons, track, distance / pitch / yaw / fov, back wall on/off (it also drops to a lip by
  itself when you orbit behind it), key / fill / rim intensity, sky brightness.
* **Renderer** — quality preset, adaptive resolution, exposure, debug view, haze, bloom, emissive scale, fxaa, GPU pass times.

Console handles: `__char`, `__rig` (live `RIG`), `__rigSource()`, `__viewer` (panel state), `__engine`, `__cam`, `__gizmo`,
`__step(n)` (advance n frames when the tab is throttled), `__cans.{smokeCan,stunCan}`, `__shot(name)` (readback → capture sink on :5174).

More panel sections since the first cut: **HIERARCHY** (click a prop or a row: node tree + off/rot sliders for the selected
node's Adj; `U` parent, `Esc` deselect; gizmo opacity), **Effects (shot: K)** (emitter gizmos — `emitters ▸ bore jet / side ports /
muzzle wisp` ride the muzzle frame and edit `fxm.shot.jetOff / portOff / wispOff` — plus shot light / gas / spark numbers), and
**Canisters** (smoke plume or vent and the stun burst on the floor mark 1.6 m ahead, optional loop, all their numbers), and
**Tactical (procedural)** — the animator's procedural vocabulary layered on the clips (`character.ts`): *stance* (high ready / low ready /
stack + which side the wall is on), one-shot buttons *kick* (the HUD flashes at the impact instant, `KICK_IMPACT`; 1.0 s in all, foot planted
by 0.72 s) / *hold* / *go* / *rally* / *flinch*, a *lockpick* toggle (forces the crouch; once the hands are half way onto the tools the animator
raises `hideHeldItem` and the viewer draws an empty hand with the sidearm holstered — the game's hand props are meant to key off the same flag),
and a *loop* box that re-fires the selected one-shot every N s. Same controls for the guard kit (`G`). Console: `__tac.stance('stack', -1)`,
`__tac.kick()`, `__tac.signal('go')`, `__tac.lockpick(true)`, `__tac.loop(true, 0, 1.5)`; `__char.anim.kickTime` / `.hideHeldItem` to read
back. Stills at an exact instant of a one-shot: `__tac.at(0.38, 'kick')` (also `'hold' | 'go' | 'rally' | 'flinch'`, or no shot to just settle
the current stance) pauses the character clock, runs out whatever is playing, lets the layers settle, fires the shot and steps the animator alone
to that time; the loop keeps rendering the frozen pose, so orbit / `__shot` next and `__tac.resume()` (or `P`) to carry on. (`__step(n)` does
not help here: it advances the viewer, whose character clock is what pause stops.)

## Workflow

1. `/viewer`, `H` for the head close-up, toggle `N` a few times, drag `RIG.goggles` sliders until the lenses sit on the
   eyes / forehead from all sides (turntable helps), check with *look yaw* and a crouch that they follow the skull.
2. *frame → hand*, upper body *aim*, sweep *aim pitch*: the slide, can, rail body and lit lens should stay rigid on the
   hand and the light's hot spot on the wall should sit on the bore line. Repeat with *relaxed* and *sprint* (gun hanging: the
   boxes follow the true direction).
3. `copy constants`, paste over the block in `src/game/rigProps.ts`, `bun run typecheck`, look at it in the game.
