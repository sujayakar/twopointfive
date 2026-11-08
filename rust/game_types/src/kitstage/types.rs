//! THE PLAN OF ONE WAY IN, as the Team & kit picture stages it: plain numbers and names, filled by rust/game_start/src/kit_plan.rs.
use alloc::string::String;
use alloc::vec::Vec;

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
