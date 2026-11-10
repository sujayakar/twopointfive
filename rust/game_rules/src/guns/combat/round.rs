//! One round's flight: the muzzle it left, the panes and thin things it went through, and whom or what it came to.
use super::*;
use crate::game::Game;
use crate::glasspen::{path_at, seg_at, RoundPath, RoundPathSt, RoundSeg};
use crate::ids::*;
use crate::m;
use crate::props::PropSystemRaycast;
use crate::v3::{self, V3};
use alloc::{vec, vec::Vec};

/// how far a round that meets nothing flies
const RANGE: f64 = 60.0;

/// One round as it left its gun.
pub(super) struct Round<'a> {
    pub shooter: CharId,
    pub who: f64,
    pub by_player: bool,
    /// Its gun is on the squad's own side: it can find his mates.
    pub ours: bool,
    pub blind: Option<bool>,
    pub def: crate::weapons::WeaponDef,
    /// The shell it is a pellet of.
    pub shell: Option<&'a mut crate::shotgun::Shell>,
    /// The only round, or a shell's first pellet: the shot is seen and heard once.
    pub lead: bool,
    /// The only round, or one pellet in three: what weighs on the world is counted by these.
    pub third: bool,
    pub muzzle: V3,
    pub dir: V3,
}

/// The way it flew.
pub(super) struct Flight {
    /// Its legs, at least one, the panes it crossed, and the static it could get no further than.
    pub path: RoundPath,
    /// The thin things it went through besides; None when it met nothing but air on the way.
    pub walls: Option<Vec<crate::wallpen::WallCross>>,
    /// The bars it met, out of its way until it is known how far it got.
    pub bars: Vec<crate::winbars::BarMet>,
    /// The gun's arm is through something: the round ends in that, at the muzzle.
    pub through: bool,
    /// What it has of its energy as it leaves the muzzle.
    pub e: f64,
    /// What its gun gives it.
    pub e0: f64,
    /// The first static on its line.
    first: Option<RoundPathSt>,
    /// The panes its gun's arm was through.
    arm_panes: f64,
}

/// Where it ended, and what it found there.
pub(super) struct End {
    /// How many metres of its path.
    pub t: f64,
    pub at: V3,
    /// Its heading as it came.
    pub dir: V3,
    /// The man it found.
    pub hit: Option<CharId>,
    /// It ended in the level: in nobody, and not in what the arm was through.
    pub bare: bool,
    /// The loose thing it came to, which it knocks, and which stops it with nobody behind it.
    pub thing: Option<PropSystemRaycast>,
    /// The static it stopped in.
    pub stop: Option<RoundPathSt>,
}

impl Flight {
    /// Where it is `t` metres on, its heading there, and what it has of its energy.
    pub fn at(&self, t: f64) -> V3 { path_at(&self.path, t) }
    pub fn way(&self, t: f64) -> V3 { seg_at(&self.path, t).d }
    pub fn energy(&self, t: f64) -> f64 { seg_at(&self.path, t).e }
    /// Was it turned aside on the way, or might it have been: it went through something thin.
    pub fn bent(&self) -> bool { self.walls.is_some() }

    /// Out from the muzzle, as far as its first static or through what is thin.
    fn goes(&mut self, g: &Game, muzzle: V3, dir: V3) {
        match crate::wallpen::thin_path(g, muzzle, dir, RANGE, self.e, Some(self.first), Some(self.arm_panes)) {
            Some(p) => {
                self.path = RoundPath { segs: p.segs, cross: p.cross, st: p.st, len: p.len };
                self.walls = Some(p.walls);
            }
            None => {
                self.path = straight(muzzle, dir, self.first, self.first.map_or(RANGE, |s| s.t), self.e);
                self.walls = None;
            }
        }
    }
}

/// One leg, through nothing.
fn straight(muzzle: V3, dir: V3, st: Option<RoundPathSt>, len: f64, e: f64) -> RoundPath {
    RoundPath { segs: vec![RoundSeg { o: muzzle, d: dir, t0: 0.0, len, e }], cross: Vec::new(), st, len }
}

/// The first static on its line.
fn first_stop(g: &Game, r: &Round) -> Option<RoundPathSt> {
    crate::scene_collision::raycast(g, r.muzzle, r.dir, RANGE).map(|h| RoundPathSt { t: h.t, index: h.index, n: h.n })
}

/// Its flight, every bar it meets let out of its way.
pub(super) fn flies(g: &mut Game, r: &mut Round) -> Flight {
    let (shooter, muzzle, dir) = (r.shooter, r.muzzle, r.dir);
    let mut first = first_stop(g, r);
    let e0 = crate::glasspen::round_energy(g, r.def.id);
    let mut bars = Vec::new();
    let mut through = arm_through(g, shooter);
    if through {
        if let Some(a) = across_arm(g, shooter).filter(|a| crate::winbars::bar_at(g, a.index)) {
            bars.push(crate::winbars::bar_meet(g, a.index, 0.0, a.at, dir));
            through = arm_through(g, shooter);
            first = first_stop(g, r);
        }
    }
    let (mut e, mut arm_panes) = (e0, 0.0);
    if through {
        if let Some(arm) = r.with_glass(|sh| crate::glasspen::arm_through_glass(g, shooter, dir, e0, sh)) {
            through = false;
            (e, arm_panes) = (arm.e, arm.panes);
        }
    }
    let mut f = Flight { path: straight(muzzle, dir, first, 0.0, e), walls: None, bars, through, e, e0, first, arm_panes };
    if through { return f; }
    f.goes(g, muzzle, dir);
    while (f.bars.len() as f64) < g.k.winbars.per_round {
        let Some(stop) = f.path.st else { break };
        if !crate::winbars::bar_at(g, stop.index) { break; }
        f.bars.push(crate::winbars::bar_meet(g, stop.index, stop.t, f.at(stop.t), f.way(stop.t)));
        f.first = first_stop(g, r);
        f.goes(g, muzzle, dir);
    }
    f
}

/// Point-blank, what its arm is through has it: a pane as a round or a pellet spent in it, anything else as the wall engine counts a round.
pub(super) fn at_the_arm(g: &mut Game, r: &mut Round, f: &Flight) -> Option<AcrossArm> {
    if !f.through { return None; }
    let (who, dir, e0) = (r.who, r.dir, f.e0);
    let arm = across_arm(g, r.shooter);
    if let Some(a) = arm.filter(|a| a.index < 1e6) {
        if r.shell.is_some() {
            r.with_glass(|sh| crate::glasspen::spent_in_pane(g, a.index, a.at, dir, e0, who, sh));
        } else {
            crate::glass::glass_hit(g, a.index, a.at, Some(dir), who, None, None);
        }
    }
    let chaos = crate::destruction::chaos_of(g);
    if r.third && chaos > 0.0 {
        if let Some(a) = across_arm(g, r.shooter) { crate::wallbreak::wall_break_round(g, who, a.at, a.d, a.index, &r.def, chaos); }
    }
    arm
}

/// Where it ends: in the nearest man it finds, or in the loose thing before him, or where its path does.
pub(super) fn ends(g: &Game, r: &Round, f: &Flight) -> End {
    let thing = if f.through { None } else { first_thing(g, f) };
    let mut t = f.path.len;
    let mut hit = None;
    if !f.through {   // (through: into the panel at the muzzle, nobody beyond it)
        for (c, own) in whom_it_can_find(g, r.shooter, r.ours) {
            let at = finds(g, r, f, t, c, own);
            if at >= 0.0 && at < t { (t, hit) = (at, Some(c)); }
        }
    }
    if let (Some(h), None) = (thing, hit) { t = h.t; }
    End {
        t,
        at: f.at(t),
        dir: f.way(t),
        hit,
        bare: hit.is_none() && !f.through,
        thing: thing.filter(|h| h.t <= t + 1e-3),
        stop: f.path.st.filter(|s| m::abs(t - s.t) < 1e-6),
    }
}

/// The first loose thing on its way.
fn first_thing(g: &Game, f: &Flight) -> Option<PropSystemRaycast> {
    f.path.segs.iter().find_map(|s| crate::props::prop_system_raycast(g, s.o, s.d, s.len).map(|h| PropSystemRaycast { t: s.t0 + h.t, ..h }))
}

/// Where it enters this man, in metres of its path short of `t`, or -1. `own`: a man of the shooter's own side.
fn finds(g: &Game, r: &Round, f: &Flight, t: f64, c: CharId, own: bool) -> f64 {
    for (k, s) in f.path.segs.iter().enumerate() {
        if s.t0 >= t { break; }
        let contact = if k > 0 { BodyHitContact::LegCrown } else if own { BodyHitContact::Entering } else { at_the_muzzle(r, f) };
        let at = body_hit(g, c, s.o, s.d, m::min(s.len, t - s.t0), Some(contact));
        if at >= 0.0 { return s.t0 + at; }
    }
    -1.0
}

/// A man the muzzle is inside of already: body_hit asks whether that counts.
fn at_the_muzzle(r: &Round, f: &Flight) -> BodyHitContact {
    BodyHitContact::Muzzle { st0_t: f.first.map(|s| s.t), muzzle: r.muzzle, glass_arm: f.arm_panes > 0.0, shooter: r.shooter }
}

/// The man in the arm of the player whose seat this is.
pub(super) fn shield_of(g: &Game) -> Option<CharId> { g.pl[g.player].holding().map(|h| g.gd[h.g].ch) }

/// Whom a round of this man's can find, his enemies first, each with whether he is of the shooter's own side. A man in a player's arm is in front of
/// him, for anybody's rounds but his; a civilian is found by a player's alone; a teammate and Sam by each other's (friendlyfire).
fn whom_it_can_find(g: &Game, shooter: CharId, ours: bool) -> Vec<(CharId, bool)> {
    let by_player = crate::character::is_player(g, shooter);
    let side = g.ch[shooter].team;
    let shield = shield_of(g);
    let (mut foes, mut own) = (Vec::new(), Vec::new());
    for pl in crate::seats::men(g) {
        let c = g.pl[pl].ch;
        if by_player || crate::condition::down(g, c) { continue; }
        if side.theirs() { foes.push((c, false)); } else if ours { own.push((c, true)); }
    }
    if let Some(c) = shield.filter(|&c| !by_player && c != shooter && !crate::condition::down(g, c)) { foes.push((c, false)); }
    for &gd in &g.guards {
        let c = g.gd[gd].ch;
        if Some(c) == shield || c == shooter || !g.ch[c].alive || crate::condition::down(g, c) { continue; }
        if side.foe_of(g.team(gd)) {
            if by_player || !g.gd[gd].god { foes.push((c, false)); }
        } else if ours && crate::friendlyfire::ff_body(g, gd) {
            own.push((c, true));
        }
    }
    if by_player { foes.extend(crate::civilians::civilian_bodies(g).into_iter().map(|cv| (g.gd[cv].ch, false))); }
    foes.extend(own);
    foes
}

impl Round<'_> {
    /// A pellet's shell handed on as the glass its pellets share: lent for the call, and back with what the call wrote.
    pub fn with_glass<R>(&mut self, run: impl FnOnce(Option<&mut crate::glasspen::ShellGlass>) -> R) -> R {
        let Some(shell) = &mut self.shell else { return run(None) };
        let mut lent = crate::glasspen::ShellGlass { glass: core::mem::take(&mut shell.glass) };
        let r = run(Some(&mut lent));
        shell.glass = lent.glass;
        r
    }
}

/// What the gun's arm is through and where: a leaf's index (>= 1e6) is what the lock rule wants of a muzzle pressed to the keep.
fn across_arm(g: &Game, c: CharId) -> Option<AcrossArm> {
    let from = shoulder(g, c);
    let d = v3::sub(g.ch[c].muzzle, from);
    let l = v3::len(d);
    if l < 1e-3 { return None; }
    let rd = v3::scale(d, 1.0 / l);
    crate::scene_collision::raycast(g, from, rd, l).map(|h| AcrossArm { index: h.index, at: v3::mad(from, rd, h.t), d: rd })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// what a call writes on the lent glass is on the shell afterwards (the pellets of one shell share it)
    #[test]
    fn a_shell_gets_its_glass_back() {
        let mut shell = crate::shotgun::Shell::default();
        let mut r = round_of(Some(&mut shell));
        r.with_glass(|sh| { sh.unwrap().glass.set(crate::Num(7.0), vec![1.0]); });
        assert_eq!(shell.glass.get(crate::Num(7.0)).map(|v| v.len()), Some(1));
        assert!(round_of(None).with_glass(|sh| sh.is_none()));
    }
    fn round_of(shell: Option<&mut crate::shotgun::Shell>) -> Round<'_> {
        Round { shooter: CharId::default(), who: 0.0, by_player: false, ours: false, blind: None, def: Default::default(), shell, lead: true, third: true, muzzle: [0.0; 3], dir: [0.0; 3] }
    }
}
