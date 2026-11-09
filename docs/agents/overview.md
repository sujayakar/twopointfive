# Overview

What this game is, how a tick becomes a picture, and where everything stands. Each other page in this folder is one system.

## What it is

A top-down squad game seen from above at a tilt, in a browser on raw WebGPU. The player is Sam, with up to four teammates who take orders
from a ring. Levels are boxes; every light, shadow and wisp of smoke is live. A mission is a room clear, a rescue, a quiet night, the night
breach or a tutorial (`missions.md`). Two people can play one mission together (`net.md`).

Two ideas hold it together:

- **One model for the picture and the AI.** What a man sees and hears is what the player is shown: the same light, the same smoke, the same
  fog, the same sound paths. No rule may know what the picture hides, and none reads a number off the GPU.
- **General systems.** The maintainer: "i'd like us to try to remove as many "hacks" or special cases as possible -- I think having strong
  general systems is the core of this project".

## The two halves

**The game is Rust** (`rust/`), built to one WebAssembly module with one memory. It holds every rule and all of the state. It has no clock,
no files, no threads and no die but its own, so the same inputs on the same start make the same game, to the bit, on every machine.

**The page draws it** (`src/`, TypeScript, no npm dependencies). The page holds a game as a number. It hands over what the player did, asks for
a tick, and draws the rows the tick left in the module's memory. No rule of play is written in `src/`: `tools/draw/ban.ts` is the first step of
the build and is red on one.

The same module runs under the page, the game server (`tools/mp/server.ts`) and the tools. Native tests and the gate link the same crates
with no module at all.

## One tick

1. **The hand.** `src/draw/hand.ts` samples keys, mouse and camera; `face_hand` takes it. What a player or a panel does that is no input
   (a setting, a kit, an order from a card) goes as a command: `src/draw/commands.ts`, read by `rust/game_run/src/command.rs`.
2. **The step.** `face_step` runs `game_update`, which steps every seat. The tick is one list of calls in order,
   `step_seats_on_first` in `rust/game_rules/src/game/p_06.rs`: the seats' aim and orders, the missions, the squad's rules, the rules that act
   on one man, then every man (`update_guard`), doors, furniture, destruction, air, lights, fire, devices, weather, items, glass, the fog,
   the drawn bodies, the light meter. A new system is a line in that list.
3. **What is drawn.** `rust/game_draw` fills sections: flat rows of numbers, one run a kind (men, lights, boxes, marks, fog, smoke, sky, view).
   Filling takes the game shared, so drawing cannot change it. `tools/draw/gen.ts` makes the page's offsets from the same declaration.
4. **What is told.** The HUD's record comes from `rust/game_rules/src/view/`, sounds and one-shot effects from
   `rust/game_rules/src/out_events/`.
5. **The picture.** `src/host/drawpage.ts` draws at the display's rate between two ticks (`src/host/between.ts`); the tick's rate is fixed
   (`src/host/fixedstep.ts`). `src/engine.ts` runs the renderer.

A run's tape is the start and every hand, so a replay is the run stepped again (`replays.md`). A networked game is the same thing over a wire:
records go up, never state.

**A fixture's light and a mark are the game's, whatever the picture shows.** A fixture's light for a question is a fact of the level (`al.base`); the
panel's emissive slider moves only the surfaces the shader draws. The marking lights are always registered, at the level's `markTexel` or
`MARKING_TEXEL`, and `?nomarkings` is the picture's alone. Marks are always laid; the panel's switch stops the upload
(`src/render/decals.ts`). A wet character's look is the last number of a look (`src/draw/look.ts`). Never seen on a GPU.

**No rule reads a picture's setting: the game tells the picture its sky.** `told` (`rust/game_rules/src/hours/told.rs`) works out the sky, the haze, the
exposure and the wet gloss from state whenever it is asked, and keeps nothing: the kit stage's own sky if it stands one, else the show's day, else the
hour under its weather (`sky_dome`), else the level's night. `rust/game_draw/src/sky.rs` reads it. `src/draw/sky.ts` keeps the rule at the page's end: a
number the game has not moved writes nothing, a hand's turn stands until the game says another, and a turn of the exposure is carried to that as a
factor. `rust/game_start/tests/told.rs` holds 23 stands against `told_numbers.json`, the numbers the settings held on the last build that wrote them.
Never seen on a GPU.

## Where things are

### `rust/`

| crate | what it is |
|---|---|
| `rust/game_types/` | the data: every module's types, typed ids, and the numbers people tune (`rust/game_types/src/knobs/`) |
| `rust/game_rules/` | the rules, a folder a module; its crate name is `game` |
| `rust/sim/` | the world's hot loops on flat columns: lines and collision, light, fog, sound, smoke, doors, furniture, glass, weather, poses |
| `rust/game_draw/` | what the picture is told |
| `rust/game_run/` | what a host does round a step: commands, the panel's knobs, the cards before a game, address words (`rust/game_run/src/words.rs`) |
| `rust/game_start/` | a game from a level file, and the native tests, the gate and the level tool |
| `rust/game_face/` | the module's face: the calls a host makes, with each file's own calls at its head |
| `rust/game_derive/`, `rust/prim/` | a declaration's bytes and knobs, and the four operations the core library lacks |

Where a system's rules are, by its page:

| page | modules of `rust/game_rules/src/` |
|---|---|
| `squad.md` | `orders/`, `pairorders/`, `pairup/`, `squadthrow/`, `squadsmoke/`, `squad40/`, `shutdoor/`, `cover/`, `rearwatch/`, `teampace/`, `planmode/` |
| `door-tools.md` | `doortools/`, `doorpath/`, `stackroom/`, `stackline/`, `entrypoints/`, `roomread/`, `roomfit/`, `bigkit/`, `wand/` |
| `team-feel.md` | `roomfight/`, `cleared/`, `nvg/`, `irlaser/`, `irlight/`, `silent/`, `sneak/`, `unseen/`, `spotted/` |
| `enemies.md` | `guards/`, `enemysquads/`, `digin/`, `pressure/`, `lanes/`, `laststand/`, `tactics/`, `qrf/`, `netquiet/`, `difficulty/`, `doorforce/`, `darkdoor/`, `needs/`, `civilians/` |
| `ai-architecture.md` | `control/`, `owner/`, `belief/`, `sense/`, `attention/`, `morale/`, `startle/`, `runner/` |
| `stealth.md` | `fogofwar/`, `interact/`, `takedown/`, `close/`, `wallsnap/`, `stickycam/`, `devices/`, `lightswitch/`, `lighting/`, `whistle/` |
| `player.md` | `player/`, `aim/`, `combat/`, `weapons/`, `kit/`, `loadout/`, `hands/`, `handwork/`, `friendlyfire/` |
| `doors.md` | `doors/`, `doorswing/`, `doorway/`, `handleside/`, `sprintdoor/`, `doorthrow/`, `doorblast/`, `props/` |
| `destruction.md` | `destruction/`, `wallbreak/`, `wallcharge/`, `glass/`, `glassbreak/`, `furnblast/`, `debris/`, `frag/`, `he40/`, `falling/` |
| `weather.md` | `weather/`, `weather_kinds/`, `wind/`, `wet/`, `hours/`, `fire/`, `air_dust/`, `snow_bog/` |
| `missions.md` | `mission/`, `roomclear/`, `rescue/`, `quiet/`, `nightbreach/`, `lesson/`, `rails/`, `par/`, `verdict/` |
| `dialogue.md` | `words/`, `dialogue/`, `doornames/`, `squadnet/` |
| `anim.md` | `character/`, `anim_skeleton/`, `ragdoll/`, `stealthpose/`, `kitanim/` |

A module the table lacks says what it is at its head; `grep -rl` over `rust/game_rules/src` for a rule's own words finds the rest. A module's types
are under the same name in `rust/game_types/src/`, and its numbers in `rust/game_types/src/knobs/`.

### `src/`

| folder | what it is |
|---|---|
| `src/host/` | the page: boot, the frame (`drawpage.ts`), the game it holds (`drawgame.ts`), input, camera, settings, the save door |
| `src/draw/` | the page's half of what a tick left: each section read and handed to the renderer |
| `src/render/`, `src/engine.ts` | passes, WGSL (`src/render/shaders/`), lights, the G-buffer, the denoiser (`render.md`) |
| `src/picture/`, `src/smoke/`, `src/anim/` | the picture's own rain, snow, air and fires; the smoke solver; glTF and skinning |
| `src/ui/`, `src/view/` | menus, HUD, the debugger; the view's types as the page reads them (`menus.md`) |
| `src/audio/`, `src/sounds/`, `src/music/` | the mixer, synthesized effects, the score, and the two listening pages (`sounds.md`) |
| `src/tape/` | kept runs, the replay page, its map and sound lanes (`replays.md`) |
| `src/net/`, `src/cloud/` | a networked game; the hosted page's side of its backend (`cloud/` is the backend) |
| `src/scene/` | the box world as it is drawn, the level format's records, the mission lists |
| `src/gpu/`, `src/math/` | the device, shader text trimming, timers; vectors and the exact maths |

### The rest

`levels/` is every level as JSON, made by `tools/levels/` (`levels.md`). `tools/qa/` is the checks and page scripts (`qa.md`). `public/assets/` is
models and animations. `STYLE.md` is the look as rules, `docs/level-format.md` the level file.

## To change it

- **A rule:** its module in `rust/game_rules/src/`, its numbers as `#[k(value)]` beside their fields, and a native test that fails without it
  (`rust/game_start/README.md` has a ten-line example). It lands on.
- **Something new to draw:** a section in `rust/game_draw/src/`, then `tools/draw/gen.ts`, then a reader in `src/draw/`.
- **A command or a setting:** `rust/game_run/src/command.rs`, then `tools/draw/gen.ts`.
- **A level:** its generator under `tools/levels/`, then `tools/levels/export.ts`.
- After Rust changes a page needs the module built: `bun run tools/blitz/face-build.ts`. The dev server serves the one it finds.

## What holds it

| | |
|---|---|
| `gate tests` | the native tests: what the rules do (`rust/game_start/tests/`) |
| `gate` | every mission started and a few played, held to `rust/game_start/pins.txt` |
| `bun run tools/qa/checks.ts` | the picture's code, shaders, page, menus, sounds, level files, words, these pages |
| `bun run tools/qa/pages.ts` | the real page in a browser on the software adapter |
| `bun run tools/draw/held.ts`, `bun run tools/draw/unused.ts` | what the pages are built from, and what nothing reaches |

None of them sees a GPU, hears a sound or knows whether a fight is fun. Anything about how it looks, sounds or plays is said as unseen until
somebody has played it.
