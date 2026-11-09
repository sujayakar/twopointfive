# twopointsix

A top-down squad tactics and stealth game on raw WebGPU. You lead up to four teammates through an orders menu,
or go in alone and unseen. Levels are built from boxes, and every light, shadow and wisp of smoke is live.

| | |
|---|---|
| ![Two men cross lights and lasers on a doorway](docs/shots/board_room_clear.jpg) | ![The boot goes into a door](docs/shots/breach2_kick.jpg) |
| ![A door opening throws a wedge of light into the dark](docs/shots/01_fire_exit_wedge.jpg) | ![Smoke from a canister in a torch beam](docs/shots/04_smoke_canister.jpg) |
| ![Emergency beacons sweep the haze in a blackout](docs/shots/05_blackout_beacons.jpg) | ![A muzzle flash lights the room](docs/shots/07_muzzle_flash.jpg) |
| ![Soft shadows under the car park lamps](docs/shots/06_parking_soft_shadows.jpg) | ![Night vision](docs/shots/08_night_vision.jpg) |

## Run

You need [Bun](https://bun.sh) 1.3 or newer and a WebGPU browser (Chrome or Edge stable, Safari 26+). There are no npm dependencies.

```bash
bun run dev      # the game at http://localhost:5173
bun run build    # static bundle in ./dist
```

## Play

Pick a mission, pick who comes and what they carry, deploy. Every key is also listed in the game under Controls.

| | |
|---|---|
| `W A S D` | move · `Shift` sprint · `C` crouch · wheel sets your walking pace |
| mouse | aim · `LMB` fire or throw what is in hand · `RMB` the pistol's light jammer, the rifle's launcher |
| `Space` | do the highlighted action · hold to pick another |
| `1` to `5` | pistol · rifle · smoke · stun · frag |
| `` ` `` | empty hands: what is in them goes away · a digit draws |
| a wall | walk into it and your back goes to it · a step away lets go |
| `R` · `L` · `N` · `V` · `Z` | reload · weapon light · night vision · whistle · semi or auto |
| `X` · `Q` · `T` · `F` | squad: order menu · the usual order · who takes it · give the word |
| `Ctrl` + mouse | orbit the camera · `Ctrl` + wheel zooms |
| `Esc` · `\` | pause and settings · the debugger, with every knob |

Every run you play is kept as a replay. `F7` saves it right away, `Shift+F7` watches it, and Extras ▸ Replays on the title lists them.

## What is in it

- **One model for the picture and the AI.** Enemies see by the same light meter and through the same smoke you are shown,
  and hear by the same sound paths the mixer plays.
- **Light.** Ray-traced direct light from sized emitters, a traced final gather over a radiance cache, volumetric beams.
  Shoot out a lamp or throw the breaker and the room really goes dark.
- **Smoke.** A GPU fluid solver. Smoke blocks sight, drifts with the vents and the wind, and stops at a shut door.
- **A squad.** Stack on a door, bang and clear, breach with a charge, hold, cover, go silent. Teammates fight on their own judgement.
- **Doors, glass and walls.** Doors swing one way, lock, get picked, kicked or blown. Glass cracks and falls. Walls and furniture come apart.
- **Levels are data.** JSON files under `levels/`, made by the generators under `tools/levels/`.
- **A look.** Plain boxes under real light: [STYLE.md](STYLE.md).

## Tools

| | |
|---|---|
| `/?bench`, or Extras ▸ Benchmark on the title | hands off, about seven minutes: three floors played by the game itself (night snow, a block with the lights out, a storm), then every quality level and heavy feature timed on your machine. It ends on a short text to copy (your GPU, load times, frames a second) with no file saved. A build with a backend also sends the numbers to the developer, which you can turn off in Options; a build with none sends nothing |
| `cargo test -p game_start` (in `rust/`) | the game's rules, asked of the game itself |
| `bun run tools/qa/checks.ts` | the picture's code, the page, the menus, the sounds and the level files, without a GPU |
| `bun run qa` | typecheck, the checks above, and `gate`: every mission stood and eight played, each held to its pins (`rust/game_start/README.md`, "The gate") |

`bun run typecheck` and `bun run qa` expect a TypeScript compiler unpacked at `vendor/typescript`, which is not in the repo.
Any recent `tsc` does the typecheck: `tsc --noEmit -p tsconfig.json`.

## Layout

```
rust/        the game: its rules, its world, what a page is told to draw
src/render   passes, WGSL shaders, lights      src/draw    the page's half of what the game hands it
src/smoke    the smoke solver                  src/scene   level format, mission lists, the box world as it is drawn
src/anim     glTF loader, skinning             src/audio   mixer, score, synthesized effects
src/ui       menus, HUD, the debugger          src/host    the page: its boot, frame and cards
src/tape     replays: the tape, its bar, the map
levels/      every level as JSON               tools/      dev server, build, level generators, qa/
public/      models and animations             docs/       level format, notes, captures
```

## Credits

Base character mesh and animations: **Universal Animation Library (Standard)** by Quaternius, CC0
(https://quaternius.com). Everything else was made here.
