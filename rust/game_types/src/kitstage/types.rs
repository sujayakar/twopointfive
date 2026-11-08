//! THE PLAN OF ONE WAY IN, as the Team & kit picture stages it: plain numbers and names, filled by rust/game_start/src/kit_plan.rs.
use alloc::string::String;
use alloc::vec::Vec;
use crate::ids::*;

pub type V2 = [f64; 2];
pub type V3 = [f64; 3];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Src { Authored, #[default] Derived }
impl Src { pub fn word(self) -> &'static str { match self { Src::Authored => "authored", Src::Derived => "derived" } } }
/// What the team is stood at. `a` → `b` are its two ends on the wall's line (a door: hinge → latch jamb), `t` the unit vector a → b, `n` the unit normal out of the wall toward the team, `door`
/// the leaf's id (none: a wall or a gap, no leaf to push, crack or charge)
#[derive(Clone, Debug, Default)]
pub struct Anchor { pub kind: String, pub door: Option<String>, pub a: V2, pub b: V2, pub mid: V2, pub t: V2, pub n: V2, pub width: f64, pub steel: bool, pub locked: bool, pub exterior: bool }
/// The wall at the anchor, measured: its faces as distances from the anchor's line along `n`, its thickness, its top over the anchor (0: no wall there), the boxes it was read off
#[derive(Clone, Debug, Default)]
pub struct Wall { pub face: f64, pub back: f64, pub thickness: f64, pub top: f64, pub boxes: Vec<String> }
/// The lane beside the stack for the card switch: `out` m from the wall's line; `free` = every step of it is free floor, `clear_m` = how far along it is
#[derive(Clone, Debug, Default)]
pub struct Lane { pub out: f64, pub a: V2, pub b: V2, pub free: bool, pub clear_m: f64 }
/// The pane the gun row shoots through: the box, its middle, width and the unit normal toward the man, where he stands, how far that is from the nearest of the five places, whether it is in
/// the anchor's own wall and how far along it (+ toward the stack). Not always the one the child's gun row ends on: that one also tries each pane from the man's own place, and has a line of fire.
#[derive(Clone, Debug, Default)]
pub struct Pane { pub name: String, pub index: usize, pub at: V3, pub width: f64, pub n: V2, pub stand: V3, pub from: f64, pub on_wall: bool, pub along: f64 }
#[derive(Clone, Debug, Default)]
pub struct Occluder { pub id: String, pub index: usize, pub group: String, pub why: &'static str }
#[derive(Clone, Debug, Default)]
pub struct Fixture { pub at: V3, pub pos: V3 }
#[derive(Clone, Debug, Default)]
pub struct Porch { pub offset: V3, pub aim: V3, pub pos: V3, pub fixture: Option<Fixture> }
#[derive(Clone, Debug, Default)]
pub struct Fill { pub offset: V3, pub aim: V3, pub pos: V3 }
#[derive(Clone, Debug, Default)]
pub struct Lamps { pub porch: Porch, pub fill: Fill }
/// The camera's pin, before the page's frame: the child keeps this yaw and pitch and then slides and scales, so do not ray-test from `pos` what the page shows
#[derive(Clone, Debug, Default)]
pub struct Cam { pub look: V3, pub yaw: f64, pub pitch: f64, pub dist: f64, pub pos: V3 }
/// One way, planned.
#[derive(Clone, Debug, Default)]
pub struct Planned {
    pub anchor: Anchor,
    /// a point on the team's side
    pub ref_: V2,
    /// which way along the wall the stack forms: 1 = past `b`, -1 = past `a` · `s` the unit vector from the anchor toward the stack
    pub side: f64, pub s: V2,
    /// where the stack falls on screen beside the anchor under `cam` ('right': the child mirrors its door / left / right)
    pub hand: String,
    pub wall: Wall,
    /// the five places, final: [x, z, yaw°]: [0] Sam, [1‥3] the stack from the anchor outward, [4] the fourth teammate behind Sam
    pub slots: Vec<V3>, pub slot_src: Vec<Src>,
    /// how many of the five stand clear · places the worked-out lines stepped over for the men they place · the spacing the authored stack was laid out at
    pub fit: usize, pub stepped: usize, pub slot_gap: f64,
    pub lane: Lane,
    /// free floor in front of the face · behind the anchor's line · behind a blank wall's far face
    pub out: f64, pub inside: f64, pub beyond: f64,
    pub pane: Option<Pane>,
    /// what stands between the camera and the men, the anchor or their floor, with the rest of each thing it is a piece of · everything out of the picture, as indexes into the boxes
    pub occluders: Vec<Occluder>, pub hidden: Vec<usize>,
    /// the loose props that go with them, as indexes into the props
    pub hidden_props: Vec<usize>,
    pub lamps: Lamps,
    /// light groups left lit with the room lights out
    pub keep_lights: Vec<String>,
    /// the team stands under a roof · how many of the five places are
    pub indoors: bool, pub covered: usize,
    pub cam: Cam,
    pub src: Vec<(&'static str, Src)>,
    /// what is wrong with this way, in plain words (empty: it stages)
    pub problems: Vec<String>,
    /// the set it was planned on (`plan_way`; none: the level itself)
    pub set: Option<String>,
}

/// What a way's `light` writes over the picture's table, key by key (absent: the table's).
#[derive(Clone, Debug, Default)]
pub struct LampOver {
    pub kind: Option<f64>, pub color: Option<V3>, pub intensity: Option<f64>, pub range: Option<f64>, pub inner_deg: Option<f64>, pub outer_deg: Option<f64>, pub radius: Option<f64>, pub volumetric: Option<f64>, pub broad: Option<bool>,
    /// the porch lamp's brightness with the level's outside lamps out
    pub day: Option<f64>,
    /// the way wrote an aim of its own
    pub aim: bool,
    /// its glowing box's half size and glow
    pub fix_half: Option<V3>, pub fix_glow: Option<f64>,
}
#[derive(Clone, Debug, Default)]
pub struct SamOver { pub on: Option<bool>, pub intensity: Option<f64>, pub inner_deg: Option<f64>, pub outer_deg: Option<f64>, pub volumetric: Option<f64>, pub aim: Option<V3> }
#[derive(Clone, Debug, Default)]
pub struct WayLights { pub porch: LampOver, pub fill: LampOver, pub sam: SamOver, pub moon_scale: Option<f64>, pub sky_scale: Option<f64>, pub room_lights: Option<f64> }

/// where the page wants the picture's men in its view. All in the view's own -1 to 1.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameAsk {
    pub cx: f64, pub cy: f64, pub h: f64, pub w: f64, pub man: f64,
    /// hung on the door: its line, the room to the left and right of it, the least share of the men's size kept
    pub hung: Option<[f64; 4]>,
    /// the clear column's own top and foot
    pub column: Option<[f64; 2]>,
}
/// A room's lamp the quiet picture dimmed, as the mission had it
#[derive(Clone, Copy, Debug)]
pub struct LampAs { pub t: FixtureId, pub intensity: f64, pub emissive: V3 }
/// A light the staging turned up or down, at the intensity the game had it
#[derive(Clone, Copy, Debug)]
pub struct LightAs { pub light: LightId, pub intensity: f64 }

/// THE KIT PICTURE'S STAGE: all a host keeps to hold a game as the picture of the team at its way in. No rule of the game reads it.
#[derive(Clone, Debug, Default)]
pub struct KitStage {
    /// the plan of the way (none: nothing to stage), the way's index
    pub plan: Option<Planned>, pub way: usize,
    /// what the way writes of the light · the way's own lamps the hour has out · the link's quiet kind (1 a quiet mission, 2 the solo night, 0 loud) · the link asks for no mission
    pub over: WayLights, pub way_room_lights: Option<f64>, pub day_out: Vec<String>, pub quiet_kind: u8, pub asks_none: bool,
    /// the men by slot · their places and yaws · Sam's place and yaw · a partner's rest · the foot of the door · the men already sent to their places
    pub order: Vec<GuardId>, pub spots: Vec<V2>, pub yaws: Vec<f64>, pub sam: V2, pub sam_yaw: f64, pub mate: Option<(V2, f64)>, pub foot: V3, pub sent: Vec<GuardId>,
    /// the lights it hung and the porch lamp's box
    pub porch: Option<LightId>, pub fill: Option<LightId>, pub inside: Option<LightId>, pub fixture: Option<BoxId>,
    /// the lights it dimmed, the moon's, the moon's colour and the sky as they were
    pub dimmed: Vec<LightAs>, pub moon: Option<LightAs>, pub moon_color: Option<V3>, pub sky: Option<(V3, V3)>,
    /// what the staging wrote each time, so that the tick cannot move them: the moon, the fill, Sam's mark and his aim's point on the wall, the sky
    pub moon_at: Option<f64>, pub fill_at: f64, pub sam_on: bool, pub sam_intensity: f64, pub sam_inner: f64, pub sam_outer: f64, pub sam_volumetric: f64, pub sam_aim: V3, pub sky_at: Option<(V3, V3)>,
    /// THE QUIET PICTURE: it is staged · the pool lamp as a fixture of the game's, for the game's life · on the list now · the lamps put out · the lamps dimmed and as they were · the slabs drawn · the share the room burns at
    pub quiet: bool, pub pool_kept: Option<FixtureId>, pub pool: Option<FixtureId>, pub out: Vec<String>, pub lit_in: Vec<FixtureId>, pub lamp_as: Vec<LampAs>, pub roofs: Vec<BoxId>, pub share: f64,
    /// where the camera is framed: the plan's pin as it was asked · the anchor's two ends · the places · the stack falls right of the anchor · the anchor's framed height · the fixture and Sam for one subject
    pub look: V3, pub yaw: f64, pub pitch: f64, pub dist: f64, pub doorway: Vec<V2>, pub stand: Vec<V2>, pub hand_right: bool, pub top_y: f64, pub fit: Option<(V3, V2)>,
    /// the page's ask, and its view's aspect (0: as the camera has it) · the pin worked out
    pub frame: Option<FrameAsk>, pub aspect: f64, pub pinned: Option<([f64; 3], f64, f64, f64)>,
    /// whose card is open (0 the player) and the row under its highlight · how many come, as last asked
    pub who: u32, pub field: String, pub size: Option<u32>,
    /// the card asked for, whose man may not have come yet · staged again by the next tick
    pub want: u32, pub due: bool,
    /// the boxes it sank, as places in the level's list · the level's men found roused by a park, and the alarm's marks it dropped: 0 for good is what the picture is held to
    pub sunk: Vec<usize>, pub roused_men: u32, pub roused_alarms: u32,
    /// nothing to stage
    pub none: bool,
}
