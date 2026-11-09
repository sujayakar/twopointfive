# Sounds

What the player hears: how a rule asks for a sound, how it reaches the ear through the walls, what it is made of, and the score. What a man does
with what he hears is `enemies.md`; how a replay plays is `replays.md`; the rain itself is `weather.md`.

## What the player gets

- Every sound is synthesized on the page. No audio file ships.
- A sound comes from a place. Through open air it is clear; round a corner it comes out of the doorway, dulled; behind a shut door it is duller
  still; through solid wall only the low part gets through. A big bare room rings more than a small one.
- What is underfoot changes a step: carpet and grass scuff, hard floors click, and the rules hear that step at the same level.
- A rolling chair sounds for as long as it rolls. Rain has a bed.
- The score follows how roused the defenders are. Each level has two pieces of its own; the main menu has the theme alone.
- Options has four volumes.

## Where the code is

The rules' half, in `rust/`:

| module | what it does |
|---|---|
| `audio_iface/` | the calls a rule makes: `game_audio_like_play`, `game_audio_like_footstep`, `game_audio_like_rain`; `PlayOpts` |
| `out_events/` | what a tick tells its host (`OutEvent`) |
| `floor/` | what is underfoot; `tread`, one footfall for the mixer and the hearing rule alike |
| `scene_collision/` | `sound_paths_propagate` for the rules, `heard_shared_by` for a question that changes nothing, `Propagation` |
| `rust/sim/src/hear.rs` | the two halves of a sound's path: `direct` and `route` |
| `ask/` | `hear_kept`, the mixer's question, reached by `face_hear` in `rust/game_face/src/ask.rs` |
| `game/` (`p_05.rs`) | `game_roll_tick` and `game_on_prop_event`: furniture's noise, the chair's voice |
| `rust/game_draw/src/events.rs` | the rows the page reads: `Sound`, `Footfall`, `Roll`, `Heard`; `tension` for the score |

The page's half, in `src/audio/` unless it says:

| file | what it does |
|---|---|
| `audio.ts` | `AudioEngine`, the mixer: the budget, followed voices, hums, the room; `SFX_TABLE`, `SFX_WARM` |
| `mixlaw.ts` | the law for one placed voice, read by the mixer and the sounds page; the default volumes |
| `backend.ts`, `webaudio.ts` | what plays a buffer: `AudioBackend`, `WebAudioBackend`, `masterChain`, the rain bed |
| `loops.ts`, `floors.ts` | room hums and the canister's hiss as data; `FLOOR`, which sample and how bright |
| `sfx.ts`, `synth.ts` | the sound table (`buildSfx`, `SfxName`, `HEARD`) and its instruments (`Voice`) |
| `sfxWas.ts`, `sfxAlt.ts` | the older table, frozen (`WAS_IN_GAME`); other takes to choose from (`altTake`) |
| `music.ts`, `pieces.ts`, `setlist.ts` | the score's sequencer (`Music`), its pieces (`PIECES`), each level's list (`SETLISTS`) |
| `musicBackend.ts`, `acoustic.ts` | the score's voices (`WebAudioMusicBackend`); plucked and struck models |
| `src/draw/events.ts`, `src/draw/ask.ts` | a tick's rows handed to the mixer once each; `hear` asked of the module |
| `src/sounds/main.ts`, `src/sounds/catalog.ts` | the sounds page, `sounds.html` at /sounds |
| `src/music/main.ts` | the score's page, `music.html` at /music |
| `soundlanes.ts`, `soundtune.ts`, `soundfilm.ts` | in `src/tape/`: a replay's sound lanes, a voice tuned on its card, the picture while a stretch is heard again |
| `tools/draw/sound-tapes.cjs`, `public/assets/replays/` | the replays that cover the table, and their `index.json` |

## How it works

**The road of a sound.** A rule calls `game_audio_like_play` with a name, a place (or none: a sting, the radio), a gain and perhaps `PlayOpts`. That
pushes an `OutEvent` on `g.out`. `rust/game_draw/src/events.rs` turns the tick's events into rows, each with its tick and a serial. `DrawEvents` plays a
row once (`Played`), so a tick run again does not sound twice, and calls `AudioEngine.play`. No rule reads the mixer back.

**Two acts, one place.** A noise a man can hear is two things: the voice above, and a `GameEvent` on `g.events`, which each man's `hearing_check` reads.
`doors_sound`, `tread` and `game_on_prop_event` do both side by side. A voice with no event is heard by the player alone.

**The path** (`sim::hear`). A line runs from source to listener, neither end lower than 0.9 m.
- Clear: direct, muffle 0. Over a meter long, each ray from the source's sides that is blocked adds a little muffle, which softens a doorway's edge.
- Blocked, but clear at 2 m up (only chest-high things between): `OVER`, direct and slightly muffled.
- Walled off and over 28 m straight, or with no way round: `sealed`, through the mass. Muffle 1, a quarter carried, and the path taken as half again
  the straight distance.
- Otherwise `route` walks the floor (`find_path`, from listener to source). The sound seems to come from the last waypoint the listener still has a straight
  line to, at the path's whole length (`apparent`). Muffle is 0.35, or 0.85 if a shut leaf lies across the way. A leaf is `sound_seal` while nearly
  shut (`SOUND_SEAL` in `rust/sim/src/doors.rs`).

`carry` is what the rules take from it. Whole glass stops a sound and a hole lets it through, where the level's pane rule is up. Rules ask through
`hear_path` in `weather/`, which also lengthens the path in rain (`weather_hear_mask`), and keep a routed answer half a second. The mixer asks `hear_kept`:
no weather, and one search a tick between two cells of the floor. Both come to the same two calls of `sim::hear`.

**The mixer's law** (`mixlaw.ts`, `AudioEngine.play`). From `muffle` and `pathLen`: a low-pass (`cutoffOf`), less gain (`occludedGain`), more reverb
(`sendOf`). The voice stands at the apparent place and the panner's distance law (`REF_DIST`, `ROLLOFF`, `MAX_DIST`) does the rest. `hrtfFor` gives
the costly panner to what is near or loud.

**The budget.** A placed one-shot quieter at the ear than `CULL_LEVEL` is not played. At most `MAX_POSITIONAL` are alive: a louder newcomer takes the
quietest one's place. Loops and placeless sounds are outside this. Pitch jitter comes from `sysRandom`, never the game's die.

**The room.** Four times a second `AudioEngine.update` casts rays round the listener; their mean length sizes the reverb, and loops are re-aimed along their
paths. The listener is the view's place (`ear` in `src/host/drawpage.ts`).

**Loops.** Room hums (`AMBIENCE_OFFICE`, `ambienceLoop`) on the ambience bus. The canister's hiss, made by the mixer when asked for `smokeHiss` (`HISS`).
The rain bed: `weather_rain_bed`, sent as `Heard.rain` each tick. A followed voice, the chair: `game_roll_tick` measures a chair's real movement, so a pinned
chair is silent, and sends one `PlayOpts::ChairRoll` with a token; the mixer then asks the `Roll` row each frame where it is and how hard it goes (`FollowSrc`,
`followGain`). `MAX_FOLLOW` and `FOLLOW_SECS` bound it. The roll files no event; a rolling chair's scrape plays no `propScrape` but still files one.

**Floors.** `floor_of` asks the level's `surfaces` block, or the office's own map. `tread` multiplies a step's level by `loud` and hands the mixer that level
and the material. The mixer picks `footHard` or `footSoft` by `hard` and nudges the rate.

**The table.** Each row of `buildSfx` is one voice with its own seed and no other input, so it is the same on every boot, in any order. Most are built with
`make` from `synth.ts` and set to the loudness in `HEARD`. `gameSfx` is what the game plays: that table with the `WAS_IN_GAME` voices laid over it.
`?oldsounds` plays all of the older table, `?newsounds` all of the newer. `warmSfx` makes the voices one at a time in idle moments before the first click
(`gameSfxLater`); a voice asked for early is made on the spot (`warmOne`). `?sfxwarm=0` makes the table at the click. `?mute` never starts the mixer.

**The score.** `Music` schedules sixteenths on the audio clock, off its own die. Its `intensity` is `Heard.tension`, 0 to 1: the most roused defender, or what
the alarm and the mains say; a teammate counts only while he is in a fight of his own. The theme is `Music.tick`. A `Piece` is data plus a `tick`, played
through `Hands`, with `levels` setting pad, sub and bass by intensity. The kit and strings are rendered once (`struckAt`, `tunedAt`). Which piece:
`audio.scoreSet`, said once at boot from the address (`setlistOf`: level, then the kind the desk says). Play opens on the list's first piece. After `MUSIC.minutes`
the next takes over at the end of a four-bar phrase while the floor is calm, or at twice that whatever the floor is doing. `audio.scoreMenu` holds the theme
while the title is up.

**The listening pages.** Neither loads game code. /sounds has a row a voice from `SOUNDS`, played through the real graph and `mixlaw.ts` from a place you
pick (`PLACES` stand in for the path). A voice with an older self plays before and after, other takes beside them. You vote, mark a toy, write a note and
copy the marks as text. Its In play links open a replay with lanes: "replays should just play in the current window, not open a new tab". /music has a
row a piece with a tension slider and keep or cut marks, then a row a level with a picker for each of its two pieces.

**Sound lanes** (`?playtape=…&soundlanes`). `soundLanes` sets `audio.tap`, told of every sound asked for: a lane a sound, a mark a time. A card picks a take,
sets pitch, tone and level (`audio.trims`, `loadTake`), flags a moment, and `copyText` says it all in one line. A checkpoint or a drag on the ruler sets a
stretch; `relisten` pauses the tape and plays its sounds through `again`, each with the mix it had, under the takes as they stand now, while `SoundFilm` shows
the recorded picture. On a voice built from instruments the card lists its layers (`TUNING`) with dials, mute, solo and seed; `renderTune` makes a take of it.

## Rules that hold

- **The mixer is the listener's alone.** Takes, trims, tables and volumes change nothing in a run. A tape holds no sound and asks for them anew as it plays.
- **A voice keeps its level.** Gains in the game are set against `HEARD`, so a new take is set to the same number.
- **No hitch on the first click.** "a hitch in the audio right when it gets started".
- **The game plays what was picked.** `WAS_IN_GAME` is the maintainer's list of voices better before. A picked take is moved into `sfx.ts` by hand and its place in
  `sfxAlt.ts` becomes `PICKED`, so the take beside it keeps its number.
- **The theme is the menu's.** "we should only have the main theme on the main menu". It is on no list.
- **Each level's music is chosen.** "we should curate the music for each level, not do it randomly". Two pieces a row. A retry keeps the music it has.
- **The band is acoustic.** "more diverse instrumentation and making them sound more acoustic rather than synth (specifically the accents and percussion,
  the long drawn out glissando pads and the fat synth bass are great)". So pad, sub and reese stand under every piece, and no piece plays the theme's
  oscillator drums (`drums: 0` in every `levels`).
- **The lanes are a debug mode.** "a special replay viewer mode where we have the audio tracks in the timeline below". Without the word there is no dock, no
  tap and no `__sndlanes`. What the browser kept is never trusted (`pickOf`, `tuneOf`).
- **A change to a level parts its replays,** which are hands played again: `--check` follows it.

## To change it

- **A number in a rule:** `rust/game_types/src/knobs/` (`fire.rs` has the fire's gains), `roll_voice` for the chair; many are bare gains at the call. `SOUNDS`
  in `src/sounds/catalog.ts` copies gains from those calls: move it too.
- **The law, the budget:** `mixlaw.ts`, the head of `audio.ts`.
- **A new voice:** its name in `SfxName`, a row in `buildSfx` with a seed of its own, its level in `HEARD`, a row in `SOUNDS` (the typecheck is red without
  it), then the rule's call. A name not in the table plays nothing and says nothing. `bun run tools/qa/checks.ts sound`, then
  `node tools/draw/sound-tapes.cjs --cover`, which says what no replay reaches and what was asked for that is no row.
- **Another take:** a row of `TAKES` in `sfxAlt.ts`. An older voice back: a name in `WAS_IN_GAME`.
- **Something new to say of a sound:** an arm of `PlayOpts`, `play_opts`, `OutPlayOpts`, the `Sound` row, `tools/draw/gen.ts`, then the reading in
  `src/draw/events.ts`. Rebuild with `bun run tools/blitz/face-build.ts`.
- **A floor:** `floor()` in `rust/game_types/src/floor/` for the rules, `FLOOR` in `floors.ts` for sample and brightness. Change both; nothing compares them.
- **A piece:** a `Piece` in `pieces.ts`, added to `PIECES`.
- **A level:** a row in `SETLISTS`, `<level>:<kind>` when a mission kind wants its own. `tools/qa/level-needs.ts` asks for it.
- **A replay that covers a sound:** `PORT=<dev server> node tools/draw/sound-tapes.cjs --add <tape> <id> <title>`. `--check` says which part
  from its tape; `--again` plays the same hands anew and says what they no longer reach.

## What holds it

Native tests in `rust/game_start/tests/`; `p.sounds` in `play.rs` is every sound a step asked for:

| file | what it holds |
|---|---|
| `ask.rs` | the shared question answers as the game's own does, sound paths with it |
| `glass_open.rs` | `through_glass_a_sound_goes_the_long_way_round` |
| `behaviour.rs`, `readme.rs` | a shot behind a shut door is heard and gone to |
| `weather.rs`, `guns_and_debris.rs` | `rain_masks_a_sound`, `debris_is_heard_where_it_lands` |
| `pair.rs` | `a_man_hears_the_other_sides_chair_and_door_and_not_his_own` |
| `the_grab.rs`, `kit.rs` | the sounds of a gun taken off a man; a refusal is `deny` |
| `hud.rs` | the word for the floor underfoot; `floor/` has tests of its own |

`bun run tools/qa/checks.ts sound` runs three: `gunfire-table-check.ts` (the three tables), `sfxwarm-check.ts` (a voice made alone is the same voice, in another
order, at two rates) and `sound-takes-check.ts` (every take neither silent nor clipped and another sound; a tune that changes nothing is the table's voice
sample for sample). The red paths are `GUNFIRE_BREAK=1`, `SFXWARM_BREAK=1` and `TAKES_BREAK=1`.

Page scripts in `tools/draw/`, run by hand on a dev server and in no row of `tools/qa/pages.ts`: `sound-tapes.cjs --check`, `soundlanes-page.cjs`,
`soundfilm-page.cjs`.

The one check on the score is `level-needs.ts`'s row. No test asks for `tension`.

## Not seen

No check hears anything: numbers hold the takes and pieces, and nobody in the cloud has heard one. The lanes' card has been seen only on the software adapter.

Faults that stand:
- Every level gets the office's hums at the office's places (`setupAmbience`), and the room's outdoors test is the office's outline (`update`).
- `fireBed` is one voice asked for every `bed_secs`, longer than the voice, so the bed has gaps. `FOLLOW_LOOPS` names it, but only the chair asks to be followed.
- `floorAt` is never set on the mixer, so a footfall with no material (a civilian's) always plays the hard sample.
