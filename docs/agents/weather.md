# The hour, the weather and the air

What time of day a level stands at, what falls from its sky, what blows through it, and which of that moves a rule. Shaders and the smoke solver are
`render.md`; a mission's row is `missions.md`; a level's boxes and vents are `levels.md`.

## What the player gets

- Any level can be played at dawn, midday, a gray afternoon, dusk or night, with no change to the level. By day the sun and sky light the yards, and a
  shut room is dark.
- Rain, a downpour, fog, dust, overcast or clear can be picked at any hour. A few missions fix their own.
- **Rain is cover for your ears.** Small sounds carry about half as far outdoors, and a downpour drowns more. Gunfire is still placed as in dry air.
- **Fog and dust are cover for your eyes.** They stand between any eye and what it looks at, for men and cameras alike. Dust comes in at a door you
  open.
- **Lightning is real light.** Under a picked weather a stroke can show you to a man. The next stroke's way is read off the storm's own die before it comes
  (`weather_next_way`), so the picture makes its reach mask ahead (`render/reachKeep.ts coming`). A stroke asked for by hand, or one that comes as the level moves, has its mask made in its own frame.
- **Snow is a place, not a pick.** A few levels are built for it. It hides nobody and slows nobody. A thrown thing sticks where it lands in deep snow.
  Boots leave prints.
- Men get wet under open sky and dry slowly. Sam drips and leaves wet prints indoors. An open fire lights its yard, leans in the wind and takes a
  bullet without going out.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `hours/` | the hours (`hours()`), one laid over the level and taken off, cloud on the light, the sky's light on a man |
| `weather_kinds.rs` | the seven kinds (`weather_presets`), which a player may pick (`WEATHER_ROW`), the snow levels (`snow_levels`) |
| `weather.rs` | what is in force: kind, roofs, storm, strokes, thunder, the wet lot; `hear_path`, `loud_len`, `weather_air_trans` |
| `air_dust.rs` | the fog's share in each half-meter cell, carried through openings and settling back |
| `wind.rs` | the sources of moving air: storm, vents, gusts at outside doors, broken windows |
| `wet.rs` | who is soaked and who dries; Sam's drips and prints |
| `fire.rs` | an open fire: lamp, flicker, coals, cracks, embers, plume, hot air |
| `moths.rs` | moths round the lit yard lamps; dress |
| `snow_bog.rs`, `render_snow_field.rs` | how deep snow takes a thing that lands; which box is lying snow, by its word |
| `snow_blast.rs` | only what is told to the picture |
| `dress/` | the rows a host with no picture puts down (`DressModDRESS`) |

Hot loops are in `rust/sim/src/`: `weather.rs` (the roofs), `air.rs`, `wind.rs`, `snow.rs` (with `bog_of`), `glow.rs`, `dome.rs`.
`rust/game_draw/src/` tells the picture: `sky.rs`, `winds.rs`, `fires.rs`, `snow.rs`, `wet.rs`.

The page's half reads those rows and writes nothing back:

| file | what it does |
|---|---|
| `src/draw/sky.ts`, `src/draw/winds.ts` | sky and weather onto the engine (`skyFrom`); the wind's lists to the solver and the particles |
| `src/picture/weather.ts` | `WeatherPicture`: holds all below, and runs either the snow or the rain |
| `src/picture/rain.ts` | near drops and drips off edges (`PRAIN`) |
| `src/picture/air.ts`, `src/render/airField.ts` | the picture's own fluid of air, for snow to ride |
| `src/picture/fire.ts` | which fires are near enough to draw; soot handing over to the plume |
| `src/picture/snow/` | prints, marks, craters, breath, flakes, wisps (`snow.ts` holds them) |
| `src/render/snowField.ts`, `src/render/powderField.ts` | lying snow baked as one surface; loose powder in the air |
| `src/render/shaders/` | `volumetrics.wgsl` (far rain), `particles_draw.wgsl`, `composite.wgsl` (fog, flame), `snow.wgsl`, `fire.wgsl` |

## How it works

**An hour is laid over a level** (`apply_hour`, `put_in`). Night is no record: the level as built. Any other hour puts the moon out, puts a sun in at
the moon's place in the light list, gives `told` sky, haze and exposure from its `HourDef`, and at midday and on the gray day opens the switch of every
`exterior` lamp (`lamps_out`). `take_out` puts everything back.

**Cloud moves light.** A picked kind's `cloud` takes light off the ground (`cloud_sun`), moves what is left from the sun to the sky (`cloud_light`),
and past half cover spreads the disc and grays both (`under_weather`). `HourState.sun_k` is the sun's one number. A strength set by hand
(`set_sun_strength`) stands against the cloud until `sun_to_cloud`. At night none of this runs.

**Which weather is in force** is settled as the game is made (`weather_new`):

1. A level named in `snow_levels` is born in snow and refuses any other kind (`weather_level_kind`, `set_kind_at`).
2. A picked kind: its preset's storm, rain, wind and fog, under the level's own roofs. `Weather.live` is up, and only then do ears and eyes change.
3. No pick, on a level with quiet missions of its own (`own_floor`): dry and still (`weather_own_night`).
4. No pick, otherwise: the old storm (`legacy`), rain and lightning, cut by the level's own roofs (`Weather.own`, `sync_roofs`), or by the fixed
`interior` and `annex` rectangles if those fit it (`weather_office_fit`). It wets men, sounds and is drawn, and moves no sense. An hour other than
night holds it off (`under_weather`).

**Roofs** are a level's ceiling slabs as flat rectangles, the biggest four (`roofs_of`, `ROOFS`). Rain, wet men, the rain's sound, wind, fog and bog
ask one question, `under_roof`.

**The tick** runs `air_dust_update`, `update_fire`, `weather_update`, `update_wetness`, then the dress rows `wind_update` and `moths_update`
(`STEPS`). `weather_update` steps strokes and thunder and writes `WeatherFrame`. The storm has its own die (`rnd`), so a stroke moves no
other roll.

**What moves a rule:**

| what | who reads it | the road |
|---|---|---|
| rain's noise | every ear (`hearing_check`, `whistle.rs`, `roomfight/`…) | `hear_path` stretches a path by `weather_hear_mask`: less under a roof, more after thunder. `loud_len` undoes it when loud |
| fog, dust | men, teammates, civilians, cameras, the team's fog of war | `clear_between` (`sense.rs`): `weather_air_trans` over `smoke_between`: fog blinds by smoke's own thresholds (`sight_gain`) |
| lightning | the light meter's stand-in | `stroke_light`, under a picked weather only |
| sun, sky, cloud | every question of light | `light_in` adds `sun_light`, `dome_light`; which ways from a place reach the sky is `rust/sim/src/dome.rs`, kept by half metre, in no save |
| the hour | `darkdoor.rs`, `calmtorch.rs`, the kit picture's porch lamp | `hour_of`, `hour_has_lamps_out` |
| storm wind | the rules' smoke grid | `wind_storm_drift` into `grid_step`, from `sense.rs`. No other wind reaches a rule |
| deep snow | a thrown thing's bounce and slide, the squad's forecast of a throw | `snow_bog_at`, `bog_run`; `throwables/`, `throwplan.rs`, `squadsmoke.rs` |
| a fire's lamp | the light meter | a real light, moved by `update_fire` |

Snow, rain and wind do not slow feet: no rule of a man's speed reads them.

**What is only dress:** rain and flakes, splashes; wet ground (`rust/game_draw/src/wet.rs` writes the rows as wet and leaves the box dry); a man's
coat (`wet_step` keeps it, and only `rust/game_draw/src/men.rs` reads it); drips and wet prints (`wet_footfall`); all snow on the ground and in the
air; the picture's air; a fire's flame and plume; moths.

**How the split is held.** `game_draw` takes the game shared and `src/picture/` writes nothing back, so drawing cannot change a game
(`the_snow_rows_change_nothing`). `tools/draw/ban.ts` keeps rules out of `src/`. A host says `StartOpts.dress`: a page has it up, a host with no
picture down. This system reads two rows of `DressModDRESS`, `wind` and `moths`, both in the tick, because the moths are boxes of the frame's list. A
cut (a load, a restart) starts each `src/picture/` part again from the rows.

**Wind.** `wind_new` makes the sources: one storm, each vent, each outside door; a broken window adds one (`glassbreak.rs`). `wind_update` works each
one's speed for the tick (ramps, `vent_due`, gusts off `vnoise`) and hands them to the picture's solver. `wind_reroof` cuts the storm by the roofs.
Fires and moths read `wind_at`.

**Dust and fog indoors.** `rust/sim/src/air.rs` steps `AirDust`: for each half-meter cell, what share of the open air's fog stands there, only under a picked
kind with fog. Open a door and it spreads in (`airdust.diffuse`); a room settles back to the kind's `leak` (`airdust.settle`). `weather_air_trans` walks that
field along the line. No section of the draw carries it: the shaders' `wxGrid` reads rows of the box table that nothing fills.

**Fire.** A level light whose name starts with a word in `fire.names` is a fire (`is_fire`). `update_fire` flickers and moves its lamp, feeds its
coals, and files its smoke, marked as look so that no man sees it, and a hot-air source that parts falling snow. Rounds and blasts leave it burning
(`fire_holds`); a hit is a crack and embers (`fire_stir`). It goes out only as its lamp does (`burns`).

**Snow.** What lies is the level's own boxes, said by their word (`SnowKind`), and they are what a boot, a round and a ray meet. `buildSnowField`
bakes them once into one drawn surface. The page presses prints from the men's rows, and a man the fog of war hides puts nothing on the map while he
is hidden. `snow_bog_at` answers 1 in deep snow, less on packed and old snow, 0 on cleared ground or under a roof.

## How a mission says its hour and weather

A row of the mission list names `hour` and `weather` on its `ScenarioEntry` (`all_missions`); `row_settings` writes them on the address over whatever
it carried, and the start page hides that row. A player picks on the start page's Hour and Weather rows (`hour_row`, `weather_row`), and the pick goes
on the link (`hour_href`, `weather_search`). A snow level has no Weather row (`own_weather`). `address_before_game` reads the address before the game
is made and sets `StoodWith`, the kind locked for that game.

| word | what it does |
|---|---|
| `?hour=dusk` | `dawn`, `midday`, `overcast`, `dusk`, `night`, and the `alias` words; anything else is night |
| `?sun=0.5` | the sun's strength 0 to 1, held against the cloud |
| `?weather=fog` | `clear`, `overcast`, `rain`, `downpour`, `fog`, `dust`. Snow is refused |
| `?noweather` | the level's own storm off from the start |
| `?firesway=0`, `?firebright=0`, `?firesize=0`, `?fireheat=0`, `?fireplume=0`, `?fireembers=0`, `?firereact=0`, `?firelean=0` | a fire's parts |
| `?knob.wx_sim.rain_k=0` | any number, by its table's name |
| `?snowlook=0`, `?snowwisps=0`, `?snowtracks=0`, `?snowbreath=0`, `?snowblast=0`, and the rest of `snowLookOfQuery` | a layer of the snow's look, off |
| `?firesnow=0`, `?norainshadow`, `?norainwalls`, `?norainscale`, `?nowallfog`, `?flatfog`, `?norain2`, `?rainv1` | picture only (`src/host/pictureopens.ts`, `src/ui/rainrows.ts`) |

## Rules that hold

- **Snow is curated.** "i think we should probably curate snow levels and have the art be custom. seems like a lot to have it just be slapped on". It
  is never a pick.
- **Rain drowns small sounds, not loud ones.** A rule that hears asks `hear_path`, never the plain path. The mixer's rain and the ear's mask are one
  number (`rain_bed`).
- **No kind picked changes no sense.** `weather_hear_mask` and `weather_air_trans` answer exactly 1.
- **A storm is cut by the level's own roofs.** "rain is busted on records office".
- **Vents are wind.** "for hvac vents, that should model wind which then also influences the smoke solver".
- **The picture's air is a fluid.** "the nice thing of integrating with the fluid system is that you'll naturally get neat interactions where the wind
  hits a wall". Nothing reads it back.
- **Rain is not lit through a wall or a shut door.** "looks like some light leakage for the low res rain". `rainLight` asks a ray where the reach mask
  is unsure and for any lamp under a ceiling.
- **A blast marks the snow.** "grenades/he don't seem to really affect snow". Every blast told makes a crater, a throw, a puff and a shove.
- **Breath is real steam, near the view.** "the smoke from the gun muzzle is fine so maybe breath should just work like that".
- **A fire's light moves.** "shadows cast by the fire should be more dynamic i think. the fire as an emitter should be shifting continuously over
  time".

## To change it

- **A number:** `rust/game_types/src/knobs/`: `weather.rs` (`WxSimK`), `wet.rs`, `wind.rs`, `air_dust.rs`, `fire.rs`, `snow_bog.rs`.
- **A kind:** its row in `WeatherKindsMod::blank`. A new one is also a word of `WeatherKind`, `weather_kinds()`, `ALL_KINDS`, `WEATHER_ROW` and
  `ScenarioEntryWeather`.
- **An hour:** its row in `hours()`; a new one also `HourId`, `HOUR_ROW` and `HOURS_OF` in `src/draw/sky.ts`.
- **A snow level:** its name in `snow_levels()`, its boxes painted by word (the `level` skill).
- **A rule that hears or sees:** call `hear_path`, and ask `clear_between` of the air.
- **A new source of air:** a `WindWhy`, its speed in `wind_update`.
- **A thing the picture needs:** a row in `rust/game_draw/src/sky.rs` or `winds.rs`, then `tools/draw/gen.ts`, then the reader.

## What holds it

Native tests in `rust/game_start/tests/`:

| file | what it holds |
|---|---|
| `weather.rs` | fog between eyes, rain on a footfall and not on a shot, dust at a door, `snow_touches_no_ear`, a pick at any hour |
| `weather_rows.rs` | what the picture is told: a fire, the winds, a snow level, a blast; `wet_ground_is_the_rows_and_not_the_box` |
| `level_rules.rs` | `a_storm_is_cut_by_the_levels_own_roofs` |
| `senses.rs` | `by_day_a_room_is_lit_by_what_it_is_open_to`, `a_lamp_is_a_nights_lamp_by_night_and_next_to_nothing_at_noon` |
| `smoke_said.rs`, `fire_look.rs` | a fire's plume is seen by no man; its hot air reaches the page |
| `marks.rs`, `kit_stage.rs` | `a_wet_print_dries_by_itself`; `by_day_the_porch_lamp_is_out_with_the_levels_own` |

`hours/`, `wind.rs` and `fire.rs` have unit tests beside their code. `levels rain` holds every level: open ground is rained on, ground under a ceiling is
not. `gate`'s pins cover `weather` and `air_dust`. Checks: `bun run tools/qa/checks.ts rainscale`, `rainwalls`, `sunglint` read the shaders' text
and run no shader. Page script: `bun run tools/qa/pages.ts weather-page` (on the software adapter).

## Not seen

Everything drawn is the maintainer's to judge on a GPU: a page check here proves only that the rows arrive. No check hears thunder or the rain. No
native test asks `snow_bog_at` directly; only `gate`'s pins move if a landing changes. The picture's fog of war asks no air: past what the weather's fog
lets an eye through (about 20 m) the floor is drawn as seen, under the weather's own haze, with nobody on it.
