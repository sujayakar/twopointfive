//! THE MARKS THE GAME HAS: bullet holes, scorches, scuffs, chips, cracks in glass, dust and rubble on a floor, wear on a wall, wet prints. One list, each mark with
//! the key of what made it, kept for the game in a slot (`Game.mark_set`): a clone of the game holds them, so a game put back shows the marks it had, and what happened
//! after it un-happens by itself. No rule reads a mark.
//!
//! A rule ASKS for a mark where something hits something. The asks of a tick are answered in their order as its last step (`settle`), with the world where the tick left
//! it: the arm works out which surface took it, and `lay` is the rule of the list (the cap, what goes first, a mark pulled onto its face, one that rides a box).
//! (rust/game_draw marks.rs is what the picture is told of them.)
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::out_events::*;
use crate::v3::{self, quat, V3};
use alloc::vec::Vec;

/// marks at most: past it the oldest goes, one laid only for the picture first
pub const CAP: usize = 512;
const SCUFF_HALF_W: f64 = 0.016;

/// what a mark's strength was when it was made and how long it lasts, where it fades as it dries
#[derive(Clone, Copy, Debug)]
pub struct Dries { pub born: f64, pub k0: f64, pub lasts: f64 }
impl Dries {
    /// how strong at `time`; a print is told in steps of a quarter of a second, so that a table of them is not sent each frame
    pub fn at(&self, time: f64) -> f64 { self.k0 * (1.0 - m::max(0.0, m::floor(time * 4.0) / 4.0 - self.born) / self.lasts) }
}

#[derive(Clone, Copy, Debug)]
pub struct Mark {
    /// the id of what made it: the tick, then the arm in the top byte and its place among that tick's marks. A tick run again makes the same.
    pub key: [u32; 2],
    pub kind: DecalKind,
    /// where it stands (a mark that rides a box: where it stood when it was laid), its face's normal, and which way it lies in the face
    pub pos: V3,
    pub n: V3,
    pub t: V3,
    pub radius: f64,
    pub param: f64,
    pub seed: u32,
    pub serial: u32,
    /// glass: the static (the level's list) whose pane it is on
    pub pane: Option<u32>,
    /// what it rides, and where on it in that box's own frame
    pub rides: Option<(BoxId, DecalLocal)>,
    /// laid only for the picture
    pub look: bool,
    pub dries: Option<Dries>,
    /// wear: half the wall's thickness where it is drawn on both faces (0: on the face it was hit on), and the stuff's code
    pub wear_half: f64,
    pub wear_mat: f64,
}

/// what the rules asked of the marks
#[derive(Clone, Debug)]
pub enum Ask {
    Add(OutDecalsAdd),
    Bullet(OutBulletMark),
    Chip(OutChipMark),
    Bang(OutBangScorch),
    Kick(OutKickScuff),
    Burst(OutMessBurst),
    Opened(OutMessOpened),
    Wall(OutMessWall),
    Evicted(OutMessEvicted),
    /// (and whether what it throws is the team's own: the fog of war asks, as the round is fired)
    Round(OutWallWearRound, bool),
    Reset,
    /// (no mark: a wall charge's feel, whose start asks the world where the tick leaves it, in its place among the rest)
    Feel(crate::wallcharge::WallSpot),
}
impl Ask {
    /// the arm's number, in a mark's key
    fn arm(&self) -> u32 {
        match self { Ask::Add(_) => 1, Ask::Bullet(_) => 2, Ask::Chip(_) => 3, Ask::Bang(_) => 4, Ask::Kick(_) => 5, Ask::Burst(_) => 6, Ask::Opened(_) => 7, Ask::Wall(_) => 8, Ask::Evicted(_) => 9, Ask::Round(..) | Ask::Reset => 10, Ask::Feel(_) => 11 }
    }
}

/// a wall's cells and the marks each carries, by serial, the oldest cell first; and the newest bullet mark taken as a round's own
#[derive(Clone, Debug, Default)]
pub struct Wear { pub cells: Vec<(f64, Vec<u32>)>, pub taken: u32 }

#[derive(Clone, Debug, Default)]
pub struct MarkSet {
    /// oldest first
    pub list: Vec<Mark>,
    pub serial: u32,
    /// prints laid (a print's look is off how many came before it)
    pub prints: u32,
    pub wear: Wear,
    /// the particles the last tick's marks and feels threw: the picture's alone, and no out event of the page that has a copy of the game
    pub thrown: Vec<OutEvent>,
    asks: Vec<Ask>,
    tick: u32,
    arm: u32,
    in_tick: u32,
}

/// (the picture's: in no bytes)
impl crate::bytes::ToBytes for MarkSet { fn w(&self, _: &mut crate::bytes::W) {} }

/// the marks of a game
pub fn mark_set(g: &Game) -> Option<&MarkSet> { g.mark_set.get() }
pub fn mark_set_mut(g: &mut Game) -> &mut MarkSet { g.mark_set.at() }
/// a rule asks for a mark
pub(in crate::breakage) fn mark_asked(g: &mut Game, a: Ask) { mark_set_mut(g).asks.push(a); }
/// particles thrown for the picture this tick
pub(in crate::breakage) fn thrown(g: &mut Game, e: OutEvent) { mark_set_mut(g).thrown.push(e); }

/// every mark goes, and what was asked before this with them
pub(super) fn marks_clear(g: &mut Game) {
    let s = mark_set_mut(g);
    s.asks.clear();
    s.list.clear();
    crate::blast_feel::feel_cleared(g);
}

/// THE TICK'S LAST STEP: what it asked is answered in its order, with the world where it left it; a feel's stages come due; and a print that has dried goes
pub fn marks_settle(g: &mut Game) {
    let tick = m::round(g.time * 60.0) as u32;
    let s = mark_set_mut(g);
    s.thrown.clear();
    if !s.asks.is_empty() {
        let asks = core::mem::take(&mut s.asks);
        s.in_tick = 0;
        s.tick = tick;
        for a in asks.iter() {
            mark_set_mut(g).arm = a.arm();
            match a {
                Ask::Add(e) => { lay(g, e.kind, e.pos, e.normal, e.size, &e.opts.unwrap_or_default()); }
                Ask::Bullet(e) => crate::marks::lay_bullet(g, e),
                Ask::Chip(e) => crate::marks::lay_chip(g, e),
                Ask::Bang(e) => crate::marks::lay_bang(g, e.at),
                Ask::Kick(e) => crate::marks::lay_kick(g, e.d, e.from),
                Ask::Burst(e) => crate::floormess::lay_burst(g, e.at, e.size),
                Ask::Opened(e) => crate::floormess::lay_opened(g, e.rect),
                Ask::Wall(e) => crate::floormess::lay_wall(g, e.p, e.n, e.thick, e.through),
                Ask::Evicted(e) => crate::floormess::lay_evicted(g, e),
                Ask::Round(e, ours) => crate::wallwear::wear_round(g, e, *ours),
                Ask::Reset => mark_set_mut(g).wear = Wear::default(),
                Ask::Feel(w) => crate::blast_feel::feel_starts(g, w),
            }
        }
    }
    crate::blast_feel::feel_stages(g);
    let t = g.time;
    let s = mark_set_mut(g);
    s.list.retain(|d| !d.dries.is_some_and(|w| w.at(t) <= 0.01));
}

/// a kind's radius and parameter where a rule says neither (render/decals.ts DEFAULTS)
fn defaults(kind: DecalKind) -> (f64, f64) {
    match kind {
        DecalKind::Bullet => (0.03, 1.0),
        DecalKind::Scorch => (0.75, 1.0),
        DecalKind::Scuff => (0.1, 0.16),
        DecalKind::Chip => (0.013, 1.0),
        DecalKind::GlassHit => (0.12, 1.0),
        DecalKind::Wetprint => (0.17, 1.0),
        DecalKind::Damage => (0.05, 0.8),
    }
}

/// THE RULE OF THE LIST. A mark of `kind` centred at `pos` on a surface whose outward normal is `normal`; `size` for the kind's own radius. What it answers is the
/// mark's serial, or none where the list takes no such mark (a full list of play's marks and this only for the picture, a crack on no pane).
pub fn lay(g: &mut Game, kind: DecalKind, pos: V3, normal: V3, size: Option<f64>, o: &DecalOpts) -> Option<u32> {
    let n = v3::normalize(normal);
    let mut t = match o.tangent { Some(t) => v3::sub(t, v3::scale(n, v3::dot(t, n))), None => perpendicular(n) };
    if v3::len(t) < 1e-4 { t = perpendicular(n); }
    let t = v3::normalize(t);
    let (r0, p0) = defaults(kind);
    let param = o.param.unwrap_or(p0);
    let radius = if kind == DecalKind::Scuff { 0.5 * param + SCUFF_HALF_W + 0.06 } else { size.unwrap_or(r0) };
    let p = match o.clamp_to { Some(b) => clamp_to_face(&g.tab.boxes[b], pos, n, if kind == DecalKind::Scuff { 0.5 * param } else { radius }, true), None => pos };
    let rides = o.attach.map(|b| { let x = &g.tab.boxes[b]; (b, DecalLocal { p: to_local(x, p, true), n: to_local(x, n, false), t: to_local(x, t, false) }) });
    let (time, lasts) = (g.time, m::max(1.0, g.k.wet.print_fade_secs));
    let s = mark_set_mut(g);
    let look = o.look == Some(true);
    let full = s.list.len() >= CAP;
    let look_at = if full { s.list.iter().position(|d| d.look) } else { None };
    if full && look && look_at.is_none() { return None; }
    s.serial += 1;
    let serial = s.serial;
    let mut seed = match o.seed { Some(v) => js::to_i32(v) as u32 & 0xfff, None => ((m::imul(serial as i32, js::to_i32(2654435761.0)) as u32) >> 20) & 0xfff };
    let pane = if kind == DecalKind::GlassHit {
        let at = o.pane.unwrap_or(-1.0);
        if at < 0.0 || at >= 65535.0 { return None; }
        Some(at as u32)
    } else { None };
    // (a print's seed says which foot; the rest of it is its look)
    let dries = if kind == DecalKind::Wetprint { seed = seed & 1 | (s.prints.wrapping_mul(37) & 0xffe); s.prints += 1; Some(Dries { born: time, k0: param, lasts }) } else { None };
    if full { match look_at { Some(i) => { s.list.remove(i); } None => { s.list.remove(0); } } }
    let key = [s.tick, s.arm << 24 | s.in_tick & 0xff_ffff];
    s.in_tick += 1;
    s.list.push(Mark { key, kind, pos: p, n, t, radius, param, seed, serial, pane, rides, look, dries, wear_half: 0.0, wear_mat: 0.0 });
    Some(serial)
}

/// a mark by its serial
pub(in crate::breakage) fn mark_of(s: &mut MarkSet, serial: u32) -> Option<&mut Mark> {
    let i = s.list.binary_search_by_key(&serial, |d| d.serial).ok()?;
    s.list.get_mut(i)
}

// ---- a box's own frame (local +X is world (cos yaw, 0, −sin yaw), local +Z is (sin yaw, 0, cos yaw); its rotation where it has one)
fn to_local(b: &crate::scene_boxes::Box, v: V3, point: bool) -> V3 {
    let d = if point { v3::sub(v, b.c) } else { v };
    if let Some(q) = b.rot { return quat::rotate(quat::conj(q), d); }
    let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw));
    [cs * d[0] - sn * d[2], d[1], sn * d[0] + cs * d[2]]
}
/// from a box's frame, the box standing at `c` turned `yaw` or `rot`
pub fn to_world(c: V3, yaw: f64, rot: Option<[f64; 4]>, l: V3, point: bool) -> V3 {
    let w = match rot {
        Some(q) => quat::rotate(q, l),
        None => { let (cs, sn) = (m::cos(yaw), m::sin(yaw)); [cs * l[0] + sn * l[2], l[1], -sn * l[0] + cs * l[2]] }
    };
    if point { v3::add(w, c) } else { w }
}
/// Pull `p` (on the face of `b` whose outward normal is `n`) in from that face's edges by `r` and onto the face plane. `on_cut`: on a box whose drawn surface is not its six
/// faces (one that says a cut, a round thing), a point inside the plain box, more than a millimetre under the face its normal leans to, is on the surface it was given, and
/// is left where it is.
fn clamp_to_face(b: &crate::scene_boxes::Box, p: V3, n: V3, r: f64, on_cut: bool) -> V3 {
    let (mut lp, ln) = (to_local(b, p, true), to_local(b, n, false));
    let (x, y, z) = (m::abs(ln[0]), m::abs(ln[1]), m::abs(ln[2]));
    let a = if x >= y && x >= z { 0 } else if y >= z { 1 } else { 2 };
    if on_cut && (b.cut.is_some() || b.round == Some(true)) && m::abs(lp[a]) < b.h[a] - 1e-3 { return p; }
    for i in 0..3 {
        if i == a { lp[i] = if ln[a] >= 0.0 { b.h[i] } else { -b.h[i] }; continue; }
        let lim = b.h[i] - r;
        lp[i] = if lim <= 0.0 { 0.0 } else { m::max(-lim, m::min(lim, lp[i])) };
    }
    to_world(b.c, b.yaw, b.rot, lp, true)
}
fn perpendicular(n: V3) -> V3 { v3::normalize(if m::abs(n[1]) < 0.9 { v3::cross(n, [0.0, 1.0, 0.0]) } else { v3::cross(n, [1.0, 0.0, 0.0]) }) }
