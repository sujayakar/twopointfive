//! THE GAME'S MAIN SWITCHES AND FIRST NUMBERS: which systems run at all, how fast people move, the lights they carry, how the light meter reads, when the response
//! team comes.
//! The rules read nearly all of it; `device_cones`, `moths` and `tracers` are the picture alone, and the lights are lights in the world like any other.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TuneK {
    /// cameras and tripwires sense at all (off: they still pan and lose power, and report nobody)
    #[k(true)] pub devices: bool,
    /// tripwires sense, given `devices` (off: the cameras still do)
    #[k(true)] pub lasers: bool,
    /// what a camera has seen, in lit-seconds (a man in full light, upright, inside 3 m fills one a second, less when farther off, low or in smoke; it drains once he
    ///  is gone):
    ///  at the first it turns suspicious and starts to blink, and its LED is brightest at 1.2 of it · at the second it reports him
    #[k(0.8)] pub camera_suspicion_secs: f64,
    #[k(2.0)] pub camera_alarm_secs: f64,
    /// the picture: a working camera shows its cone as a light (off: no cone, and it still sees)
    #[k(true)] pub device_cones: bool,
    /// m/s, the legs of every AI man: a patrol's walk (× 1.15 once the shift has escalated; also a squad script's default pace and the brake at a doorway) · a search or a man
    ///  moved by fire · a chase or a fight
    ///  (some rows run a little past the chase, as the chase + their own share)
    #[k(1.15)] pub guard_walk: f64,
    #[k(1.75)] pub guard_investigate: f64,
    #[k(2.7)] pub guard_chase: f64,
    /// m/s, Sam: his walk (a teammate's walk is this × teampace.k) · his sprint (what teammates run at beside him, and the pace a guard's search reckons how far he
    ///  can have got by) ·
    ///  NOT his crouched walk, which is twogears.crouch: his pace crouched along a wall, and low with a body (× the haul's pace; × 0.6 on a tow with no haul)
    #[k(1.55)] pub player_walk: f64,
    #[k(4.7)] pub player_sprint: f64,
    #[k(1.3)] pub player_crouch: f64,
    /// the intensity of the rail light on the gun in Sam's hands, kept to this every frame
    #[k(60.0)] pub player_light: f64,
    /// the torch on every guard and spawned unit, kept to these every frame: intensity · half-angle of its full-strength core · half-angle of the whole beam
    ///  (degrees; the panel keeps the core at least 2 under the beam)
    #[k(70.0)] pub flash_intensity: f64,
    #[k(7.0)] pub flash_inner: f64,
    #[k(19.0)] pub flash_outer: f64,
    /// awareness a second a guard gains on a man in full light, upright, inside 3 m: × light^1.5, distance, smoke, how he moves and the side of the eye he is on (×
    ///  0.75 low, × 2.2 for a man hunting).
    ///  The same number starts the sum for a pair in a hold (× PAIR_LEGIBLE) and for a sneaking teammate
    #[k(1.6)] pub detect_rate: f64,
    /// seconds the OCP blinds a fixture or a camera; a guard's torch stays out for 0.7 of it and a breaker's blackout lasts 2.5 × it
    #[k(9.0)] pub ocp_duration: f64,
    /// a guard not yet suspicious, near a lamp the OCP put out: Glance = a small look and a mutter, his awareness kept under 0.28 · Investigate = as for any other
    ///  lamp that goes out
    #[k(crate::game::GameTuneOcpNotice::Glance)] pub ocp_notice: crate::game::GameTuneOcpNotice,
    /// THE LIGHT METER: visibility = slope × ln(irradiance / floor), held to 0‥1. This is the slope: at 0.2 full visibility is about 150 × the floor. A carried
    ///  lamp's glare in the room fight falls off by it too
    #[k(0.2)] pub vis_slope: f64,
    /// … and the floor: the irradiance that reads 0. Morale calls a man in the dark when the steady light on his chest and on his head, his own lamp left out, are
    ///  both under it
    #[k(0.03)] pub vis_floor: f64,
    /// what a floor is made of sets how loud a step is, to the ear and to the guards, and the HUD says what is underfoot (off: the guards hear every floor alike, and
    ///  the ear gets a soft floor at 0.8)
    #[k(true)] pub floor_materials: bool,
    /// guards don't fire over a hostage: none shoots while Sam has a living man up as a shield, nor does a guard in a standoff unless his hold has a clear line, and
    ///  when the held man drops he waits a beat
    ///  (off: they keep firing, and that beat is only 0.15‥0.45 s)
    #[k(true)] pub muzzle_discipline: bool,
    /// fire that passes close shakes a man, makes him flinch and duck and can pin him (off: it does nothing to him)
    #[k(true)] pub suppression: bool,
    /// who among the level's guards carries the AKS-74U over his route's gun: nobody · the one called Reyes · all of them. Spawned units keep their own, and it is
    ///  dealt again every frame
    #[k(crate::game::CarbineDeal::None)] pub carbine_guards: crate::game::CarbineDeal,
    /// a guard's first burst in a while at a far foe goes wide of him on purpose, to the side his cover is nearest (the distances are the SUPPRESS_FIRST_ consts)
    #[k(true)] pub suppress_first: bool,
    /// guns with a tracer row show their streaks (off: none go, and any in flight are cleared). Their numbers are the `tracers` table
    #[k(true)] pub tracers: bool,
    /// guards aim by a track that builds on a foe: it has to reach a bar before the first shot, they brake to shoot, and they learn when their line is fouled by a
    ///  frame (off: a flat spread from the gun's numbers, no bar, no brake)
    #[k(true)] pub aim_model: bool,
    /// sides exist (off: no unit can be spawned, the room fight doesn't run and a mission that needs a side refuses to start)
    #[k(true)] pub teams: bool,
    /// guards at their ease get up for a need, one at a time (off: no trips, and any under way are broken off). A level's script can set this for its run, and `qrf`
    ///  and `civilians` too
    #[k(true)] pub needs: bool,
    /// hunting guards work together: plans with flankers, gates at doors, doors marked as dangerous and looks at them (off: each chases on his own)
    #[k(true)] pub tactics: bool,
    /// seconds the man holding the target kneels with no flanker still running before he goes in anyway
    #[k(6.0)] pub flank_wait: f64,
    /// seconds a man alone on a target kneels and calls for help before he goes on hunting (with a friend near, 1 is enough)
    #[k(8.0)] pub lone_wait: f64,
    /// seconds a door stays marked as dangerous to a side after a man of theirs drops there in view, a body is found by it or rounds come through it (at least 1)
    #[k(25.0)] pub door_danger: f64,
    /// seconds the first man on a door's jamb waits for a second man to stack behind him before he opens up anyway
    #[k(6.0)] pub door_wait: f64,
    /// the picture: moths gather at lit lamps, at night, in calm air, with no snow or downpour
    #[k(true)] pub moths: bool,
    /// panes can break (it also needs destruction's chaos above 0)
    #[k(true)] pub glass_break: bool,
    /// a breaking pane throws shards and plates, given `glass_break` (off: it breaks without them)
    #[k(true)] pub glass_shards: bool,
    /// walls can break, under rounds and charges
    #[k(true)] pub wall_break: bool,
    /// blasts and blows push what lies loose (it also needs destruction's chaos above 0)
    #[k(true)] pub debris_impulses: bool,
    /// the response team comes when the shift has escalated and gunfire is heard or two bodies are found (off: only the panel can call it)
    #[k(true)] pub qrf: bool,
    /// seconds it takes to arrive, for a panel call and on a level whose response row gives no time
    #[k(75.0)] pub qrf_eta: f64,
    /// men in it, rounded and held to 2‥4, for a panel call and on a level whose response row gives no number
    #[k(4.0)] pub qrf_size: f64,
    /// the second tier: with the team deployed, one of theirs down, enough men found down or a running gunfight, control locks the place down and men go to the
    ///  outside posts. Only on a level with posts, once a night
    #[k(true)] pub perimeter: bool,
    /// seconds until they are posted (at least 5); control's radio line says this number as it stands
    #[k(45.0)] pub perimeter_eta: f64,
    /// men wanted, rounded and held to 2‥4, and never more than there are posts
    #[k(2.0)] pub perimeter_size: f64,
    /// gunfire, then every man of the shift off the net: control asks for a radio check, then calls the team on the silence alone (off: the silence means nothing)
    #[k(true)] pub qrf_silent: bool,
    /// a level's civilians stand up (never on a fight stage)
    #[k(true)] pub civilians: bool,
    /// killing a civilian fails the mission (off: a teammate's line, and the mission goes on)
    #[k(false)] pub civilian_kill_fails: bool,
    /// a body has to be lit enough for a guard to make it out, by the `body_light` table (off: any body in view is seen)
    #[k(true)] pub body_light: bool,
}
