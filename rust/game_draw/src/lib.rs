//! WHAT THE PICTURE IS TOLD, AND NOTHING ELSE OF THE GAME. A page that draws this game holds no game: after a tick it is told what to draw as a list of
//! SECTIONS, each a run of flat rows in this module's memory, read where they lie. A section is declared once (`section!`) and filled by a function that
//! takes the game shared, so drawing cannot change it; this crate is above the rules and no rule can name it. The page's offsets into a row are made from
//! the same declaration by `bun run tools/draw/gen.ts`.
//!
//! To add a section: a file beside this one with its `section!` and its `fill`, a line in `sections!` below, then `tools/draw/gen.ts`.
//!
//! - A field is a word or a run of words: `f32`, `u32`, `i32`, or an array of one. So a row has no padding and lies as it is declared.
//! - A row whose only reader is the GPU is written here as the GPU reads it: the page hands the rows over and packs nothing.
//! - A row holds values, never a reference into the game: what it says of another row is that row's place in its own section, or a key that means one
//!   thing for as long as the thing stands.
#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec};
use game::game::Game;

/// a type that is nothing but words
pub trait Words: Copy { const ZERO: Self; }
impl Words for f32 { const ZERO: f32 = 0.0; }
impl Words for u32 { const ZERO: u32 = 0; }
impl Words for i32 { const ZERO: i32 = 0; }
impl<T: Words, const N: usize> Words for [T; N] { const ZERO: [T; N] = [T::ZERO; N]; }

/// a row of a section: its fields in their order, as the page's offsets are made from them
pub trait Row: Copy { const FIELDS: &'static [(&'static str, &'static str)]; const ZERO: Self; }

/// A SECTION'S ROW, declared: the struct as it lies in memory and the names its offsets are made from
macro_rules! section {
    ($(#[$m:meta])* pub struct $name:ident { $($(#[$fm:meta])* pub $f:ident: $t:ty,)* }) => {
        $(#[$m])* #[repr(C)] #[derive(Clone, Copy, Debug, PartialEq)] pub struct $name { $($(#[$fm])* pub $f: $t,)* }
        impl crate::Row for $name {
            const FIELDS: &'static [(&'static str, &'static str)] = &[$((stringify!($f), stringify!($t)),)*];
            const ZERO: Self = $name { $($f: <$t as crate::Words>::ZERO,)* };
        }
    };
}

/// AN EVENT'S ROW, declared: a section's, behind the id that names it. The tick it happened in, its kind (which section it is in) and its place among its kind in
/// that tick: the same id whenever that tick is run, so a host plays it once, and one that does not come again un-happened.
macro_rules! event {
    ($(#[$m:meta])* pub struct $name:ident { $($(#[$fm:meta])* pub $f:ident: $t:ty,)* }) => {
        section! { $(#[$m])* pub struct $name { pub tick: u32, pub serial: u32, $($(#[$fm])* pub $f: $t,)* } }
    };
}

/// WHAT A HOST MAY ASK FOR, each with a bit of its `want`: what is made together is asked for together
macro_rules! groups {
    ($($name:ident)*) => {
        #[allow(non_camel_case_types, dead_code)]
        enum Group { $($name,)* }
        #[allow(non_upper_case_globals)]
        pub mod want { $(pub const $name: u32 = 1 << super::Group::$name as u32;)* }
        const WANTS: &str = concat!("want" $(, " ", stringify!($name))*);
    };
}

/// THE SECTIONS, declared: each with its row and where its rows are kept, in the order the table says where they lie
macro_rules! sections {
    ($($name:ident: $t:ty = $($at:ident).+;)*) => {
        /// where each section lies, in their order: its address and how many rows
        fn table(d: &Drawn, out: &mut Vec<u32>) { out.clear(); $({ let v: &Vec<$t> = &d.$($at).+; out.push(v.as_ptr() as usize as u32); out.push(v.len() as u32); })* }
        /// the declarations as text, a line a section: what the page's offsets were made from is held to it as the module is loaded
        pub fn layout() -> String {
            let mut s = String::from(WANTS);
            s.push('\n');
            $(s.push_str(stringify!($name)); for (f, t) in <$t as Row>::FIELDS { s.push(' '); s.push_str(f); s.push(':'); for c in t.chars() { if !c.is_whitespace() { s.push(c); } } } s.push('\n');)*
            s
        }
    };
}

pub mod boxes;
pub mod doorways;
pub mod events;
pub mod fires;
pub mod fog;
pub mod lights;
pub mod marks;
pub mod men;
pub mod resets;
pub mod rims;
pub mod sky;
pub mod view;
pub mod winds;

groups! { fog view sky boxes lights men doorways rims events fires decals air }

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
    info: boxes::Word = boxes.grid.info;
    panes: boxes::Row4 = boxes.glass.cells;
    leaves: boxes::Leaf = boxes.leaves;
    lights: lights::Light = lights.rows;
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
    rolls: events::Roll = events.rolls;
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
}

/// the numbers of a page's look (src/draw/look.ts writes them in this order): the boxes', then the men's
pub const LOOK: usize = boxes::LOOK + 4;

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
    pub rims: Vec<rims::Rim>,
    tick: Option<u32>,
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
}
impl Drawn {
    /// A TICK HAS RUN: the sections of `want`, made again from the game as it stands
    pub fn take(&mut self, g: &Game, want: u32) {
        self.want = want;
        // (the same tick asked again, or asked for other sections, is packed again and is no step between two)
        let now = tick(g);
        let again = self.tick == Some(now);
        self.tick = Some(now);
        if want & want::fog != 0 { self.fog.clear(); fog::fill(g, &mut self.fog); }
        if want & want::view != 0 { self.view.clear(); view::fill(g, self.stood, self.desired, &mut self.view); }
        if want & want::sky != 0 { self.sky.clear(); sky::fill(g, &mut self.sky); }
        if want & want::boxes != 0 { let night = g.pl[game::seats::host_man(g)].nv_amount > 0.5; self.boxes.take(g, night, again && self.had & want::boxes != 0); }
        if want & want::lights != 0 { self.lights.take(g, again && self.had & want::lights != 0); }
        if want & want::men != 0 { self.men.take(g, again && self.had & want::men != 0); }
        if want & want::rims != 0 { self.rims.clear(); rims::fill(g, &mut self.rims); }
        if want & want::events != 0 { self.events.take(g); resets::fill(g, &mut self.resets); }
        if want & want::fires != 0 { self.fires.clear(); fires::fill(g, &mut self.fires, &mut self.fire_table); }
        if want & want::air != 0 { self.winds.take(g, again && self.had & want::air != 0); }
        self.had = if again { self.had | want } else { want };
        if want & want::doorways != 0 && self.doorways.len() != g.level.doors.len() { self.doorways.clear(); doorways::fill(g, &mut self.doorways); }
    }
    /// A FRAME IS DRAWN, `a` of the way from the tick before to the last (1 = the last as it is), looking at `focus`
    pub fn frame(&mut self, g: &Game, a: f64, focus: [f64; 3]) {
        if self.want & want::boxes != 0 { self.boxes.frame(g, a, focus); }
        if self.want & want::lights != 0 { self.lights.frame(a); }
        if self.want & want::men != 0 { self.men.frame(a); }
        if self.want & want::decals != 0 { self.marks.frame(g, &self.boxes); }
    }
    /// the game is another than the one last drawn (a start, a game put back, another seat): nothing is drawn through it
    pub fn cut(&mut self) { self.boxes.cut(); self.lights.cut(); self.men.cut(); self.events.cut(); self.marks.cut(); self.tick = None; self.stood += 1; }
    /// what the page says of its look
    pub fn look(&mut self, v: &[f64]) { let (boxes, men) = v.split_at(boxes::LOOK); self.boxes.set_look(boxes::Look::read(boxes)); self.men.swaps = core::array::from_fn(|i| men[i] as u32); }
    pub fn table(&mut self) -> &[u32] {
        let mut t = core::mem::take(&mut self.table);
        table(self, &mut t);
        self.table = t;
        &self.table
    }
}

/// the tick a game is at: sixtieths of a second on its clock
pub fn tick(g: &Game) -> u32 { game::m::round(g.time * 60.0) as u32 }

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
