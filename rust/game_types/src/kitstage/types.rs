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
/// A light the staging turned, and the intensity it is held at
#[derive(Clone, Copy, Debug)]
pub struct LightAs { pub light: LightId, pub intensity: f64 }

/// The mission as its start left it, before anything of it was staged: the game's own value, its block, and the stage as it then was
#[derive(Clone)]
pub struct Base(pub alloc::sync::Arc<(alloc::boxed::Box<crate::game::Game>, Vec<u8>, KitStage)>);
impl core::fmt::Debug for Base { fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result { f.write_str("Base") } }

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
    /// what the staging wrote each time, so that the tick cannot move them: the moon, the fill, Sam's mark and his aim's point on the wall, the sky
    pub moon_at: Option<LightAs>, pub fill_at: f64, pub sam_on: bool, pub sam_intensity: f64, pub sam_inner: f64, pub sam_outer: f64, pub sam_volumetric: f64, pub sam_aim: V3, pub sky_at: Option<(V3, V3)>,
    /// THE QUIET PICTURE: it is staged · the pool lamp as a fixture of the game's · the lamps put out · the lamps dimmed, and as they were · the share the room burns at
    pub quiet: bool, pub pool: Option<FixtureId>, pub out: Vec<String>, pub lit_in: Vec<LampAs>, pub share: f64,
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
    /// what every staging starts from
    pub base: Option<Base>,
    /// THE WAY IN AS THE ACTS READ IT: the leaf (none: a wall or a gap) · a blank wall · the opening's width · the floor a can or a round has behind the anchor's line · where Sam's frag is aimed and how far in its arc must land · a wall anchor stands blown open
    pub door: Option<DoorId>, pub walled: bool, pub width: f64, pub reach_in: f64, pub frag_m: f64, pub frag_land: f64, pub hole: bool,
    /// the door's face the team is on · the leaf's angle shut and at rest · along the wall toward the stack, out on the team's side, the anchor's middle · which way along the wall the leaf's free edge is
    pub side: f64, pub leaf_shut: f64, pub leaf_rest: f64, pub s: V2, pub n: V2, pub mid: V2, pub latch: f64,
    /// the floor spots inside: a throw's, a teammate's smoke, Sam's smoke, his stun, his frag
    pub spots_in: Vec<V3>, pub spots_smoke: Vec<V3>, pub sam_smoke: V3, pub sam_stun: V3, pub sam_frag: V3,
    /// Sam's rest, and his place a step back while a partner's card is open · the fourth man's place while his card is open, and that he is on it
    pub sam_rest: V2, pub sam_back: Option<V2>, pub act4: V2, pub four: bool,
    /// the team indexes by place, kept through a restart
    pub keep: Option<Vec<usize>>,
    /// THE DISSOLVE: copies asked of the page · when the last was asked (none: none is out) · the page has laid it · a blank wall's hole is not stood again by what it covers
    pub veil: u32, pub veil_asked: Option<f64>, pub veil_laid: bool,
    /// the link's `stillkit`, or a check's own: nothing is directed, the men stand on their places
    pub still: bool,
    /// the page has shown its first picture: no act and no life before it
    pub shown: bool,
    /// THE ACTS: the director's own (acts.rs)
    pub a: Acts,
}

/// A governed yaw: its value and speed, what the game last wrote, and, owned by the director, the ease it is on
#[derive(Clone, Copy, Debug, Default)]
pub struct Chan { pub v: f64, pub w: f64, pub want: f64, pub own: bool, pub from: f64, pub d: f64, pub v0: f64, pub t0: f64, pub t: f64, pub to: f64 }
/// a leg of a quiet row's aim: the yaw, the most speed to it (rad/s), seconds the ease takes at least (under 0: the calls' own for the angle), seconds held there
#[derive(Clone, Copy, Debug)]
pub struct Leg { pub yaw: f64, pub cap: f64, pub min_t: f64, pub hold: f64 }
/// a teammate's walk: the way points and the one he is on, his pace, when it starts, he is on the yard lane, the point last sent, his gun's watch, when it was dealt, the yaw he is sent with, a card switch dealt it, he holds it on a knee
#[derive(Clone, Debug)]
pub struct Walk { pub man: GuardId, pub pts: Vec<V2>, pub k: usize, pub pace: f64, pub at: f64, pub lane: bool, pub sent: Option<V2>, pub watch: Option<V3>, pub t0: f64, pub face: Option<f64>, pub dealt: bool, pub low: bool }
/// the row a beat is playing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Act { #[default] Gun, GunDoor, Light, BangOrder, FragOrder, ThrowSmoke, ThrowFlash, ThrowFrag, ChargeMate, ChargeSam, WandSam, WandMate, Cam, Shell, Ring, WallCharge, BigMate, BigSam, Turn, Swap, QuietGun, QuietLean, QuietLight }
/// the gun row's shot: where he stands for it and the corner his walk turns at, the point the rounds go to, through the opened doorway (no pane)
#[derive(Clone, Copy, Debug)]
pub struct Fire { pub post: V2, pub via: Option<V2>, pub at: V3, pub door: bool }
/// A beat under way: which one, the thing, whose card, seconds in, where it is, what has happened; its row, the man (none: Sam) and his body, where he lives and its yaw, when it and its phase began, when its mark fired, a count from before it
#[derive(Clone, Debug)]
pub struct Beat {
    pub n: u32, pub field: &'static str, pub i: u32, pub time: f64, pub phase: &'static str, pub marks: Vec<&'static str>,
    pub row: Act, pub prelude: bool, pub on_span: bool, pub man: Option<GuardId>, pub ch_id: CharId, pub home: V2, pub yaw: f64, pub t0: f64, pub pt: f64, pub mark_t: f64, pub base: f64,
    /// where he steps to and throws to · his light is the row's · the can has left Sam's hand · the squad's order was given · what Sam had in his hands · the thing thrown, what lay there before it, where it last lay
    pub post: Option<V2>, pub spot: Option<V3>, pub lit: bool, pub thrown: bool, pub given: bool, pub hand: Option<crate::game::SlotKind>, pub item: Option<ItemId>, pub old: Vec<ItemId>, pub rest: Option<V3>,
    /// the aim: the channel of a teammate (Sam's is the stage's), the legs and the one it is on, when its hold began, the aim is the director's, the body turns with it, the beam's pitch and that it has risen, the height the aim point is put at
    pub ch: Chan, pub legs: Vec<Leg>, pub leg: usize, pub leg_t: f64, pub quiet: bool, pub turn: bool, pub pitch: Option<Chan>, pub rose: bool, pub h_to: f64,
    /// the OTHER men it took off their places · they have been let go of the squad's order · the gun's shot, its rounds, the last one's time, the corner of his walk
    pub moved: Vec<GuardId>, pub loose: bool, pub fire: Option<Fire>, pub shots: u32, pub shot_t: f64, pub via: Option<V2>,
    /// a bystander's furthest drift
    pub drift: f64,
}
/// A beat that is over, as a check reads it
#[derive(Clone, Debug, Default)]
pub struct BeatOver { pub n: u32, pub field: &'static str, pub i: u32, pub t: f64, pub phase: &'static str, pub marks: Vec<&'static str>, pub how: &'static str, pub rest: Option<V3>, pub drift: f64 }
/// a teammate's gun on its way in
#[derive(Clone, Copy, Debug)]
pub struct GunSwap { pub man: GuardId, pub id: crate::weapons::WeaponId, pub at: f64 }
/// a man's idle wander: where in the list he is, the ease, when the next is due, where it is, he is wandering
#[derive(Clone, Copy, Debug)]
pub struct Scan { pub key: Option<GuardId>, pub k: usize, pub from: f64, pub to: f64, pub t0: f64, pub next: f64, pub v: f64, pub on: bool }
/// loud kit's cost as it last played
#[derive(Clone, Copy, Debug, Default)]
pub struct CostSeen { pub field: &'static str, pub i: u32, pub door: bool, pub leaf: f64, pub up: bool, pub s: f64, pub how: &'static str, pub moved: f64, pub pool: bool }

/// The director's own state
#[derive(Clone, Debug)]
pub struct Acts {
    /// the thing under the highlight · the thing asked for now · the running beat is cut for it · the gun and the light, turn about · no row before this · a rest owed after a mended lamp · beats begun · a row has been waited for since
    pub hi: Option<&'static str>, pub asked: Option<&'static str>, pub cut_now: bool, pub flip: u32, pub rest_until: f64, pub rest_next: f64, pub beat_n: u32, pub wait_t: f64,
    /// the beat under way · what the last row left only a restage mends · the men a whole-team row left apart go back onto their places
    pub beat: Option<Beat>, pub dirty: bool, pub apart: bool,
    /// a restage for a man off his place is out, and the row that waited begins at once · one has been tried and nobody seen home since · at a wall left open: the men the next row waits for · and Sam and every man, for the row after the charge
    pub seated: bool, pub seat_tried: bool, pub hole_men: Vec<GuardId>, pub hole_all: bool,
    /// the leaf last pulled · Sam's card is owed him · the men the last row had off their places · the hand a row cut this tick owes Sam · a leaf asked shut until · the last row's end
    pub pulled: f64, pub sam_card_due: bool, pub home_men: Vec<GuardId>, pub hand_due: Option<crate::game::SlotKind>, pub leaf_ask: f64, pub last_end: f64,
    /// smoke: when the vent is shut · a cut can vents nothing until · the game's clock at the last tick and since it
    pub vent_at: f64, pub vent_kill: f64, pub step_t: f64, pub step_dt: f64,
    /// THE HOLD through Sam's frag: the beat it is on for · since when the team is on a knee · each man's knee as it last stood · a row of Sam's has waited for the men since · the beat that has the team down
    pub pin: Option<u32>, pub pin_t: f64, pub kneel_t: f64, pub knees: Vec<(GuardId, f64)>, pub home_t: f64, pub kneel: Option<u32>,
    /// the game's rows as they were, held for a row: FRAG's flee, SAMSTUN's own, the dead lamp's lead
    pub flee_m: Option<f64>, pub seen_was: Option<bool>, pub lead_was: Option<bool>,
    /// Sam's weapon light as the game has it · the lit man's beam · the wand's row has the fog of war on for the game · Player.wand as it found it · a teammate's card and what it carried · the load a launcher's row found on the tube
    pub sam_cone: Option<(f64, f64, f64)>, pub beam: Option<V3>, pub fog_up: bool, pub wand_was: bool, pub wand_kit: Option<(GuardId, bool)>, pub tube_was: Option<i32>,
    /// teammates' guns on their way in · each member's colours as the scenario staged him
    pub swaps: Vec<GunSwap>, pub issued_suit: Vec<(u32, V3)>, pub issued_trim: Vec<(u32, V3)>,
    /// the walks · this way's wall would not open on its span, and why · the wall charge's own tile is not played here again, and why · the order as start() would have given it
    pub walks: Vec<Walk>, pub shut_wall: Option<String>, pub shut_own: Option<String>, pub wall_ask: Option<(crate::orders::OrderTarget, Vec<GuardId>)>,
    /// the picture's solver has been told of smoke since the air was last cleared
    pub smoke_told: bool,
    /// a round has gone and nothing is mended yet · when · bursts looked past through a wall left open
    pub shot: bool, pub shot_at: f64, pub hole_shots: u32,
    /// Sam's aim as the director has it, and its height · his body and aim as drawn · his rest aim (NaN: a row of his own hands has it) · when his yaws last stepped
    pub sam_ch: Chan, pub sam_h: f64, pub body: Chan, pub aim: Chan, pub sam_at: f64, pub yaw_t: f64,
    /// idle life · the half step home stuck at, since when
    pub scans: Vec<Scan>, pub stuck_d: f64, pub stuck_t: f64,
    /// THE QUIET PICTURE: since when the dark's fill is lifted · the half crouch · his gun last put away at · why the quiet row gave way · the blade's share
    pub lift_at: f64, pub rise: f64, pub stow_t: f64, pub kept: Option<String>, pub blade: f64,
    /// LOUD KIT'S COST: since when (NaN: none up) · its lamp · the pool lamp's own · his feet under it · played · cut short · the last
    pub cost_t: f64, pub cost_lamp: Option<LightId>, pub cost_pool: Option<(f64, V3)>, pub cost_feet: V2, pub cost_n: u32, pub cost_cuts: u32, pub cost_last: Option<CostSeen>,
    /// LIFE AT REST: since when (NaN: none up) · the next · how many · the men hold still · its lamp, hung for it, and home · a check raised it
    pub life_t: f64, pub life_next: f64, pub life_n: u32, pub life_still: bool, pub life_lamp: Option<LightId>, pub life_hung: bool, pub life_home: V3, pub life_on: Option<bool>, pub life_seen: u32, pub life_cuts: u32,
    /// the cards as last told
    pub cards: Vec<crate::kit::MemberKit>,
    /// this tick's hand on Sam's own controls, and his cursor
    pub press: Vec<&'static str>, pub clicked: f64, pub cursor: Option<V3>,
    /// FOR THE CHECKS: the last beat over · why the last one asked for did not begin · beats cut · rounds fired · Sam staggered, a teammate dazzled · men the hold put right · the plain restages · a pick waited on
    pub last: Option<BeatOver>, pub refused: Option<String>, pub cuts: u32, pub shots: u32, pub hits_sam: u32, pub hits_team: u32, pub pins: u32, pub restages: u32,
}
impl Default for Acts {
    fn default() -> Self {
        let n = f64::NAN;
        Acts {
            hi: None, asked: None, cut_now: false, flip: 0, rest_until: 0.0, rest_next: n, beat_n: 0, wait_t: n,
            beat: None, dirty: false, apart: false,
            seated: false, seat_tried: false, hole_men: Vec::new(), hole_all: false, smoke_told: false,
            pulled: -1e9, sam_card_due: false, home_men: Vec::new(), hand_due: None, leaf_ask: -1.0, last_end: -1e9,
            vent_at: -1.0, vent_kill: -1.0, step_t: 0.0, step_dt: 0.0,
            pin: None, pin_t: 0.0, kneel_t: 0.0, knees: Vec::new(), home_t: n, kneel: None,
            flee_m: None, seen_was: None, lead_was: None,
            sam_cone: None, beam: None, fog_up: false, wand_was: false, wand_kit: None, tube_was: None,
            swaps: Vec::new(), issued_suit: Vec::new(), issued_trim: Vec::new(),
            walks: Vec::new(), shut_wall: None, shut_own: None, wall_ask: None,
            shot: false, shot_at: -1e9, hole_shots: 0,
            sam_ch: Chan::default(), sam_h: 0.0, body: Chan::default(), aim: Chan::default(), sam_at: n, yaw_t: 0.0,
            scans: Vec::new(), stuck_d: f64::INFINITY, stuck_t: 0.0,
            lift_at: n, rise: 0.0, stow_t: -1e9, kept: None, blade: 1.0,
            cost_t: n, cost_lamp: None, cost_pool: None, cost_feet: [0.0; 2], cost_n: 0, cost_cuts: 0, cost_last: None,
            life_t: n, life_next: n, life_n: 0, life_still: false, life_lamp: None, life_hung: false, life_home: [0.0; 3], life_on: None, life_seen: 0, life_cuts: 0,
            cards: Vec::new(),
            press: Vec::new(), clicked: 0.0, cursor: None,
            last: None, refused: None, cuts: 0, shots: 0, hits_sam: 0, hits_team: 0, pins: 0, restages: 0,
        }
    }
}
