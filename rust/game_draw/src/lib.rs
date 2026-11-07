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

/// THE SECTIONS, declared: each with its place (its bit in a host's `want`), its row and where its rows are kept
macro_rules! sections {
    ($($i:literal $name:ident: $t:ty = $($at:ident).+;)*) => {
        /// where each section lies, in their order: its address and how many rows
        fn table(d: &Drawn, out: &mut Vec<u32>) { out.clear(); $({ let v: &Vec<$t> = &d.$($at).+; out.push(v.as_ptr() as usize as u32); out.push(v.len() as u32); })* }
        /// the declarations as text, a line a section: what the page's offsets were made from is held to it as the module is loaded
        pub fn layout() -> String {
            let mut s = String::new();
            $(s.push_str(stringify!($name)); for (f, t) in <$t as Row>::FIELDS { s.push(' '); s.push_str(f); s.push(':'); for c in t.chars() { if !c.is_whitespace() { s.push(c); } } } s.push('\n');)*
            s
        }
        /// each section's bit
        #[allow(non_upper_case_globals)]
        pub mod want { $(pub const $name: u32 = 1 << $i;)* }
    };
}

pub mod boxes;
pub mod doorways;
pub mod events;
pub mod fog;
pub mod lights;
pub mod men;
pub mod rims;
pub mod sky;
pub mod view;

sections! {
    0 fog: fog::Fog = fog;
    1 view: view::View = view;
    2 sky: sky::Sky = sky;
    3 head: boxes::Head = boxes.head;
    4 geo: boxes::Geo = boxes.geo;
    5 mat: boxes::Mat = boxes.mat;
    6 cut: boxes::Cut = boxes.cut;
    7 visible: boxes::Word = boxes.visible;
    8 marks: boxes::Row4 = boxes.marks.rows;
    9 cells: boxes::Word = boxes.grid.cells;
    10 items: boxes::Word = boxes.grid.items;
    11 info: boxes::Word = boxes.grid.info;
    12 panes: boxes::Row4 = boxes.glass.cells;
    13 leaves: boxes::Leaf = boxes.leaves;
    14 lights: lights::Light = lights.rows;
    15 men: men::Man = men.men;
    16 insts: men::Inst = men.insts;
    17 bones: men::Bones = men.bones;
    18 caps: men::Cap = men.caps;
    19 doorways: doorways::Doorway = doorways;
    20 emits: events::Emit = events.emits;
    21 kicks: events::Kick = events.kicks;
    22 pushes: events::AirPush = events.pushes;
    23 sources: events::SmokeSource = events.sources;
    24 anchors: events::Anchor = events.anchors;
    25 beams: boxes::Beam = boxes.beams;
    26 sounds: events::Sound = events.sounds;
    27 footfalls: events::Footfall = events.footfalls;
    28 rolls: events::Roll = events.rolls;
    29 heard: events::Heard = events.heard;
    30 rims: rims::Rim = rims;
}

/// the numbers of a page's look (src/draw/look.ts writes them in this order): the boxes', then the men's
pub const LOOK: usize = boxes::LOOK + 4;

/// what one host of one game draws it from
#[derive(Default)]
pub struct Drawn { pub fog: Vec<fog::Fog>, pub view: Vec<view::View>, pub sky: Vec<sky::Sky>, pub boxes: boxes::Boxes, pub lights: lights::Lights, pub men: men::Men, pub doorways: Vec<doorways::Doorway>, pub events: events::Events, pub rims: Vec<rims::Rim>, tick: Option<u32>, stood: u32,
    /// the distance the host's camera was on its way to as the step was handed it
    pub desired: f64, had: u32, want: u32, table: Vec<u32> }
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
        if want & want::geo != 0 { let night = g.pl[game::seats::host_man(g)].nv_amount > 0.5; self.boxes.take(g, night, again && self.had & want::geo != 0); }
        if want & want::lights != 0 { self.lights.take(g, again && self.had & want::lights != 0); }
        if want & want::men != 0 { self.men.take(g, again && self.had & want::men != 0); }
        if want & want::rims != 0 { self.rims.clear(); rims::fill(g, &mut self.rims); }
        if want & want::emits != 0 { self.events.take(g); }
        self.had = if again { self.had | want } else { want };
        if want & want::doorways != 0 && self.doorways.len() != g.level.doors.len() { self.doorways.clear(); doorways::fill(g, &mut self.doorways); }
    }
    /// A FRAME IS DRAWN, `a` of the way from the tick before to the last (1 = the last as it is), looking at `focus`
    pub fn frame(&mut self, g: &Game, a: f64, focus: [f64; 3]) {
        if self.want & want::geo != 0 { self.boxes.frame(g, a, focus); }
        if self.want & want::lights != 0 { self.lights.frame(a); }
        if self.want & want::men != 0 { self.men.frame(a); }
    }
    /// the game is another than the one last drawn (a start, a game put back, another seat): nothing is drawn through it
    pub fn cut(&mut self) { self.boxes.cut(); self.lights.cut(); self.men.cut(); self.events.cut(); self.tick = None; self.stood += 1; }
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
