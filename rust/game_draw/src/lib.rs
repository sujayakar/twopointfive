//! WHAT THE PICTURE IS TOLD, AND NOTHING ELSE OF THE GAME. A page that draws this game holds no game: after a tick it is told what to draw as a list of
//! SECTIONS, each a run of flat rows in this module's memory, read where they lie. A section is declared once (`section!`) and filled by a function that
//! takes the game shared, so drawing cannot change it; this crate is above the rules and no rule can name it. The page's offsets into a row are made from
//! the same declaration by the program `half` (game_start/src/bin/half).
//!
//! To add a section: a file beside this one with its `section!` and its `fill`, a line in `sections!` below, then `half`.
//!
//! - What a row's field may be: `game_types::rows`.
//! - A row whose only reader is the GPU is written here as the GPU reads it: the page hands the rows over and packs nothing.
//! - A row holds values, never a reference into the game: what it says of another row is that row's place in its own section, or a key that means one
//!   thing for as long as the thing stands.
#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec};
use game::game::Game;

pub use game::rows::{Row, RowTable};

/// A SECTION'S ROW, declared (`game_types::rows`)
macro_rules! section { ($($t:tt)*) => { game::row! { $($t)* } }; }

/// AN EVENT'S ROW, declared: a section's, behind the id that names it. The tick it happened in, its kind (which section it is in) and its place among its kind in
/// that tick: the same id whenever that tick is run, so a host plays it once, and one that does not come again un-happened.
macro_rules! event {
    ($(#[doc = $d:expr])* pub struct $name:ident { $($(#[doc = $fd:expr])* pub $f:ident: $t:ty,)* }) => {
        section! { $(#[doc = $d])* pub struct $name { pub tick: u32, pub serial: u32, $($(#[doc = $fd])* pub $f: $t,)* } }
    };
}

/// WHAT A HOST MAY ASK FOR, each with a bit of its `want`: what is made together is asked for together
macro_rules! groups {
    ($($name:ident)*) => {
        #[allow(non_camel_case_types, dead_code)]
        enum Group { $($name,)* }
        #[allow(non_upper_case_globals)]
        pub mod want { $(pub const $name: u32 = 1 << super::Group::$name as u32;)* }
        /// their names, in the order of their bits
        pub const GROUPS: &[&str] = &[$(stringify!($name)),*];
    };
}

/// THE SECTIONS, declared: each with its row and where its rows are kept, in the order the table says where they lie
macro_rules! sections {
    ($($name:ident: $t:ty = $($at:ident).+;)*) => {
        /// where each section lies, in their order: its address and how many rows
        fn table(d: &Drawn, out: &mut Vec<u32>) { out.clear(); $({ let v: &Vec<$t> = &d.$($at).+; out.push(v.as_ptr() as usize as u32); out.push(v.len() as u32); })* }
        /// each section by its name, with its row: what the page's offsets are written from (game_start/src/bin/half)
        pub fn sections() -> Vec<(&'static str, RowTable)> { alloc::vec![$((stringify!($name), game::rows::table::<$t>())),*] }
        /// the declarations as text, a line a section: what the page's offsets were written from is held to it as the module is loaded
        pub fn layout() -> String {
            let mut s = String::from("want");
            for g in GROUPS { s.push(' '); s.push_str(g); }
            s.push('\n');
            $(s.push_str(stringify!($name)); for (f, t) in <$t as Row>::FIELDS { s.push(' '); s.push_str(f); s.push(':'); for c in t.chars() { if !c.is_whitespace() { s.push(c); } } } s.push('\n');)*
            s.push_str(&gpu::text());
            s
        }
    };
}

pub mod boxes;
pub mod doorways;
pub mod dust;
pub mod events;
pub mod fires;
pub mod fog;
pub mod gpu;
pub mod lead;
pub mod watched;
pub mod lights;
pub mod marks;
pub mod men;
pub mod resets;
pub mod rims;
pub mod sky;
pub mod smoke;
pub mod traffic;
pub mod snow;
pub mod view;
pub mod wet;
pub mod winds;

groups! { fog view sky boxes lights men doorways rims events fires decals air level_look snow smoke }

sections! {
    fog: fog::Fog = fog;
    view: view::View = view;
    sky: sky::Sky = sky;
    head: boxes::Head = boxes.head;
    geo: boxes::Geo = boxes.geo;
    mat: boxes::Mat = boxes.mat;
    cut: boxes::Cut = boxes.cut;
    visible: boxes::Word = boxes.visible;
    marks: boxes::Row4 = boxes.marks.rows;
    cells: boxes::Word = boxes.grid.cells;
    items: boxes::Word = boxes.grid.items;
    info: boxes::Info = boxes.grid.info;
    panes: boxes::Row4 = boxes.glass.cells;
    leaves: boxes::Leaf = boxes.leaves;
    lights: lights::Light = lights.rows;
    coming: lights::Coming = lights.coming;
    men: men::Man = men.men;
    insts: men::Inst = men.insts;
    bones: men::Bones = men.bones;
    caps: men::Cap = men.caps;
    doorways: doorways::Doorway = doorways;
    emits: events::Emit = events.emits;
    kicks: events::Kick = events.kicks;
    pushes: events::AirPush = events.pushes;
    sources: events::SmokeSource = events.sources;
    anchors: events::Anchor = events.anchors;
    beams: boxes::Beam = boxes.beams;
    sounds: events::Sound = events.sounds;
    footfalls: events::Footfall = events.footfalls;
    lasting: events::Lasting = events.lasting;
    heard: events::Heard = events.heard;
    rims: rims::Rim = rims;
    fires: fires::Fire = fires;
    fire_table: fires::FireTable = fire_table;
    decals: boxes::Word = marks.words;
    decal_keys: marks::DecalKey = marks.keys;
    decal_head: marks::DecalHead = marks.head;
    winds: winds::Wind = winds.active;
    gusts: winds::Wind = winds.gusts;
    heat: winds::Wind = winds.heat;
    air: winds::Air = winds.air;
    wind_roofs: winds::Rect = winds.wind_roofs;
    roofs: winds::Rect = winds.roofs;
    ceilings: winds::Rect = winds.ceilings;
    door_boxes: boxes::Leaf = winds.leaves;
    open_panes: boxes::Word = winds.open;
    resets: resets::Reset = resets;
    dust: dust::DustLook = dust;
    feet: snow::Foot = snow.feet;
    things: snow::Thing = snow.things;
    blasts: snow::SnowBlast = snow.blasts;
    snow_words: snow::SnowWord = snow.words;
    smoke: smoke::SmokeHead = smoke.head;
    smoke_at: boxes::Word = smoke.at;
    smoke_bricks: smoke::SmokeBrick = smoke.bricks;
}

/// the numbers of a page's look (src/draw/look.ts writes them in this order): the boxes', then the men's
pub const LOOK: usize = boxes::LOOK + 5;

/// what one host of one game draws it from
#[derive(Default)]
pub struct Drawn {
    pub fog: Vec<fog::Fog>,
    pub view: Vec<view::View>,
    pub sky: Vec<sky::Sky>,
    pub boxes: boxes::Boxes,
    pub lights: lights::Lights,
    pub men: men::Men,
    pub doorways: Vec<doorways::Doorway>,
    pub events: events::Events,
    pub marks: marks::Marks,
    pub dust: Vec<dust::DustLook>,
    pub rims: Vec<rims::Rim>,
    tick: Option<u32>,
    run: Option<u32>,
    stood: u32,
    /// the distance the host's camera was on its way to as the step was handed it
    pub desired: f64,
    had: u32,
    want: u32,
    table: Vec<u32>,
    pub fires: Vec<fires::Fire>,
    pub fire_table: Vec<fires::FireTable>,
    pub winds: winds::Winds,
    pub resets: Vec<resets::Reset>,
    pub snow: snow::Snow,
    pub smoke: smoke::Smoke,
    /// the host's own man, where he is drawn ahead of the tick
    pub lead: lead::Lead,
    /// the picture is a tube's, whatever the run's was
    tube: Option<bool>,
    watched: Vec<game::rig_props::CarriedBox>,
}
impl Drawn {
    /// A TICK HAS RUN: the sections of `want`, made again from the game as it stands
    pub fn take(&mut self, g: &Game, want: u32) { self.taken(g, want, false) }
    /// …of several a host steps before it draws a frame, one after the first: what moves is blended over them all
    pub fn over(&mut self, g: &Game, want: u32) { self.taken(g, want, true) }
    fn taken(&mut self, g: &Game, want: u32, over: bool) {
        // (another run of the mission is another game, whoever began it)
        if self.run.replace(g.mission.serial).is_some_and(|r| r != g.mission.serial) { self.cut(); }
        self.want = want;
        // (the same tick asked again, or asked for other sections, is packed again and is no step between two)
        let now = tick(g);
        let again = self.tick == Some(now);
        self.tick = Some(now);
        if want & want::fog != 0 { self.fog.clear(); fog::fill(g, &mut self.fog, self.men.bared); }
        if want & want::view != 0 { self.view.clear(); view::fill(g, self.stood, self.desired, &mut self.view); }
        if want & want::sky != 0 { self.sky.clear(); sky::fill(g, &mut self.sky); }
        if want & want::boxes != 0 { let night = self.tube.unwrap_or(g.pl[game::seats::host_man(g)].nv_amount > 0.5); self.boxes.take(g, night, over || again && self.had & want::boxes != 0, &self.watched); }
        if want & want::lights != 0 { self.lights.take(g, again && self.had & want::lights != 0); }
        if want & want::men != 0 { self.men.take(g, over || again && self.had & want::men != 0); }
        if want & want::rims != 0 { rims::fill(g, &mut self.rims); }
        if want & want::events != 0 { self.events.take(g); resets::fill(g, &mut self.resets); }
        if want & want::level_look != 0 { self.dust.clear(); dust::fill(g, &mut self.dust); }
        if want & want::fires != 0 { self.fires.clear(); fires::fill(g, &mut self.fires, &mut self.fire_table, self.men.bared); }
        if want & want::air != 0 { self.winds.take(g, again && self.had & want::air != 0); }
        if want & want::snow != 0 { self.snow.take(g); }
        if want & want::smoke != 0 { self.smoke.take(g); }
        self.had = if again { self.had | want } else { want };
        if want & want::doorways != 0 && self.doorways.len() != g.level.doors.len() { self.doorways.clear(); doorways::fill(g, &mut self.doorways); }
    }
    /// A FRAME IS DRAWN, `a` of the way from the tick before to the last (1 = the last as it is), looking at `focus`
    pub fn frame(&mut self, g: &Game, a: f64, focus: [f64; 3]) {
        let lead = self.lead.at(a);
        if self.want & want::boxes != 0 { self.boxes.frame(g, a, focus, lead.as_ref()); }
        if self.want & want::lights != 0 { self.lights.frame(a); }
        if self.want & want::men != 0 { self.men.frame(a); }
        if let Some(by) = &lead {
            if self.want & want::rims != 0 { rims::fill(g, &mut self.rims); }
            self.lights.lead(by);
            self.men.lead(by);
            for r in &mut self.rims { if r.id == by.id { r.at = by.pt32(r.at); } }
        }
        if self.want & want::decals != 0 { self.marks.frame(g, &self.boxes); }
    }
    /// does this picture show what no list of the game's holds (watched.rs)
    pub fn watches(&self) -> bool { self.men.bared || self.tube == Some(true) }
    /// …that, of the game as it stands: before a take
    pub fn watch(&mut self, g: &Game) { self.watched = if self.watches() { watched::boxes(g, self.men.bared && game::fogofwar::fog_of_war_active(g), self.tube == Some(true)) } else { Vec::new() }; }
    /// A VIEWER'S OWN PICTURE of a run: with the fog off it, or through a tube the run's was not (nothing said: as the run had it)
    pub fn view_as(&mut self, bared: bool, tube: Option<bool>) { self.tube = tube; self.boxes.bare(bared); self.men.bared = bared; self.lights.bared = bared; }
    /// a host's tick came and the game was not stepped (a networked host with nothing confirmed): what moves stands where the last tick left it, and is blended from there
    pub fn hold(&mut self) { self.boxes.hold(); self.lights.hold(); self.men.hold(); }
    /// the game is another than the one last drawn (a start, a game put back, another seat): nothing is drawn through it
    pub fn cut(&mut self) { self.boxes.cut(); self.lights.cut(); self.men.cut(); self.events.cut(); self.marks.cut(); self.smoke.cut(); self.lead.rest(); self.tick = None; self.stood += 1; }
    /// what the page says of its look
    pub fn look(&mut self, v: &[f64]) { let (boxes, men) = v.split_at(boxes::LOOK); self.boxes.set_look(boxes::Look::read(boxes)); self.men.swaps = core::array::from_fn(|i| men[i] as u32); self.men.dry_look = men[4] == 0.0; }
    pub fn table(&mut self) -> &[u32] {
        let mut t = core::mem::take(&mut self.table);
        table(self, &mut t);
        self.table = t;
        &self.table
    }
}

/// the tick a game is at: sixtieths of a second on its clock
pub fn tick(g: &Game) -> u32 { game::m::round(g.time * 60.0) as u32 }
/// where the picture looks: the host's camera, or its man where it has none
pub fn looks_at(g: &Game) -> [f64; 3] {
    match g.host_cam { Some(c) => g.tab.camera[c].target, None => g.player_pos(game::seats::host_man(g)) }
}
/// what the picture leaves out of a fire's embers and hot air: anything further from where it looks than it would ever show
pub fn far_for_a_fire(g: &Game, p: [f64; 3]) -> bool {
    let f = looks_at(g);
    !(game::v3::dist_xz(p, f) < g.k.fire.ember_r)
}

/// the short way round from one angle to another
pub fn turn(from: f64, to: f64) -> f64 {
    let full = 2.0 * core::f64::consts::PI;
    let d = (to - from) % full;
    if d > full / 2.0 { d - full } else if d < -full / 2.0 { d + full } else { d }
}
/// between two turns, the short way round (a tick's turn is a few degrees)
pub fn between(p: [f64; 4], q: [f64; 4], a: f64) -> [f64; 4] {
    let s = if p[0] * q[0] + p[1] * q[1] + p[2] * q[2] + p[3] * q[3] < 0.0 { -1.0 } else { 1.0 };
    let r: [f64; 4] = core::array::from_fn(|i| p[i] + (q[i] * s - p[i]) * a);
    let n = game::m::sqrt(r[0] * r[0] + r[1] * r[1] + r[2] * r[2] + r[3] * r[3]);
    if n > 1e-9 { r.map(|x| x / n) } else { q }
}
