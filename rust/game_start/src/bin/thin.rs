//! THIN: the corners no hand of `hold` reaches, staged and hashed. Not part of the gate: run it on two builds and compare what they print.
//!
//!   thin [SCENE]… > FILE      every scene of every level below (or those named, `office.bars`), a line a part of the state: the scene, the part, a hash of the part after every frame
//!   diff FILE OTHER           two builds that print the same played the same, part for part, after every frame of every scene
//!
//! For a build from before this file: copy it into that tree's `src/bin` and build it there (it asks only what older trees answer too).
//!
//! THE SCENES (each a game of its own on each level, the dressing rows up as a page stands it, Sam unhurt, the men's minds off but for `shells`, which stands the level's room clear if it has one):
//!   shells   a shotgun in Sam's hands by the trigger, dry and its shells loaded again; a shell and a breaching shell into men near and far, locked doors at the lock and past it, panes,
//!            lamps, loose things, walls; the same from a man of theirs
//!   bars     rounds and shells through window bars from both sides and aslant (several bars a round), and with the gun's arm through a bar; the arm through a pane, a wall, a shut leaf
//!   sheets   every wall of the level blown and chewed, with what is painted on it as authored, marked, and cut to a shape that leaves a marked one no room
//!   capped   the same with room for one piece a sheet, then for ten in all, turn about, so that ragged ends are given up, whole columns go, and sheets are left as they stood
//! Every scene ends in a restart and a second more. The `reached` lines say what each scene came to, counted from the game: a scene that reached nothing proves nothing.
use game::combat::FireOpts;
use game::ids::*;
use game::weapons::WeaponId;
use game_start::parts;
use game_start::play::{Button, Opts, Play};

type V3 = [f64; 3];
const LEVELS: [&str; 16] = ["office", "consulate", "registry", "warehouse", "killhouse", "block", "bazaar", "pass", "records", "compound", "depot", "police-post", "library-hall", "vault", "glasshouse", "snatch"];
const SCENES: [&str; 4] = ["shells", "bars", "sheets", "capped"];

struct T { p: Play, sums: Vec<u64>, w: game::bytes::W, words: Vec<u32>, frames: usize, n: std::collections::BTreeMap<&'static str, usize> }
fn add(a: V3, d: V3, s: f64) -> V3 { [a[0] + d[0] * s, a[1] + d[1] * s, a[2] + d[2] * s] }

impl T {
    fn new(level: &str, men: bool) -> T {
        let file = format!("{level}.json");
        let clear = men && game_start::play::room_clear_files().contains(&file);
        let mut p = Play::level(&file, clear, &Opts { seed: 1, dress: true, ..Default::default() });
        p.g.god_mode = true;
        p.g.tune.chaos = 0.5;
        T { p, sums: vec![0xcbf2_9ce4_8422_2325; parts::names().len()], w: game::bytes::W::new(), words: Vec::new(), frames: 0, n: Default::default() }
    }
    fn reached(&mut self, what: &'static str, n: usize) { *self.n.entry(what).or_insert(0) += n; }
    fn frame(&mut self) {
        self.p.frame();
        parts::words(&mut self.p.g, &mut self.w, &mut self.words);
        for (s, x) in self.sums.iter_mut().zip(&self.words) { *s = (*s ^ *x as u64).wrapping_mul(0x0000_0100_0000_01b3); }
        self.frames += 1;
        let k = &self.p.g.destr.walls.skins;
        let (cut, marked, most) = (k.cut.len(), k.marked.len(), k.cut.values().map(|l| l.len()).max().unwrap_or(0));
        for (what, n) in [("sheets cut at once, at most", cut), ("marked sheets left whole for want of room", marked), ("pieces of one sheet, at most", most)] { let e = self.n.entry(what).or_insert(0); *e = (*e).max(n); }
    }
    fn frames(&mut self, n: usize) { for _ in 0..n { self.frame(); } }
    fn sam(&self) -> CharId { self.p.g.pl[self.p.g.player].ch }
    /// what stands on the straight way from `a` to `b`: a static or a leaf by its index
    fn between(&self, a: V3, b: V3) -> Option<f64> {
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        game::scene_collision::raycast(&self.p.g, a, [d[0] / l, d[1] / l, d[2] / l], l).map(|h| h.index)
    }
    fn stat(&self, i: f64) -> Option<BoxId> { if i < 1e6 { self.p.g.col.boxes.get(i as usize).copied() } else { None } }
    /// a round (or a shell) of `gun` by combat's own call, the shooter's shoulder and muzzle put where they are said to be
    fn fires(&mut self, c: CharId, shoulder: V3, muzzle: V3, target: V3, gun: Option<WeaponId>) {
        self.p.g.ch[c].bones.set(game::character::Bone::UpperArmR, shoulder);
        self.p.g.ch[c].muzzle = muzzle;
        match self.between(shoulder, muzzle) {
            Some(i) if i >= 1e6 => self.reached("arm through a leaf", 1),
            Some(i) if game::winbars::bar_at(&self.p.g, i) => self.reached("arm through a bar", 1),
            Some(i) if game::glass::is_pane(&self.p.g, self.stat(i)) => self.reached("arm through a pane", 1),
            Some(_) => self.reached("arm through a wall or a thing", 1),
            None => self.reached("arm through nothing", 1),
        }
        let weapon = gun.map(|w| self.p.g.m.weapons.weapons.get(w).unwrap().clone());
        if weapon.as_ref().is_some_and(|w| w.pellets.is_some()) { self.reached("shells fired", 1); } else { self.reached("rounds fired", 1); }
        let o = FireOpts { steady: Some(true), spread: Some(0.0), weapon, blind: None };
        game::combat::fire_weapon(&mut self.p.g, c, target, Some(&o));
        self.frames(2);
    }
    /// through `at` the way `n` points: from `far` back, or the arm across it
    fn through(&mut self, c: CharId, at: V3, n: V3, far: f64, gun: Option<WeaponId>) {
        let end = add(at, n, 6.0);
        if far > 0.0 { self.fires(c, add(at, n, -far - 0.4), add(at, n, -far), end, gun); } else { self.fires(c, add(at, n, -0.4), add(at, n, 0.2), end, gun); }
    }
    /// men whose hands may fire: Sam, and the first man of theirs who is up
    fn shooters(&self) -> Vec<CharId> {
        let mut v = vec![self.sam()];
        if let Some(&m) = self.p.g.guards.iter().find(|&&m| !self.p.g.team(m).ours() && self.p.up(m)) { v.push(self.p.g.gd[m].ch); }
        v
    }
    fn ends(&mut self) {
        game::savestate::control::restart(&mut self.p.g);
        self.frames(60);
    }
    fn open_walls(&self) -> usize { game::wallbreak::wall_break_all(&self.p.g).iter().filter(|w| w.open > 0.0).count() }
}

/// statics a rule picks, the first `most` of them
fn statics(t: &T, most: usize, pick: impl Fn(&T, f64) -> bool) -> Vec<(f64, V3, V3)> {
    let g = &t.p.g;
    (0..g.col.boxes.len()).map(|i| i as f64).filter(|&i| pick(t, i)).map(|i| { let b = &g.tab.boxes[g.col.boxes[i as usize]]; (i, b.c, b.h) }).take(most).collect()
}
const WAYS: [V3; 4] = [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];

fn shells(t: &mut T) {
    // by the trigger: a pump gun in his hands, fired dry, its shells loaded one at a time, and fired again
    let rifle = t.p.g.pl[t.p.g.player].rifle;
    (t.p.g.fa[rifle].def, t.p.g.fa[rifle].chamber, t.p.g.fa[rifle].mag, t.p.g.fa[rifle].spare) = (WeaponId::M870, 1.0, Some(3.0), vec![6.0, 6.0]);
    t.p.key_down("Digit2"); t.frames(6); t.p.key_up("Digit2"); t.frames(42);
    let s = t.p.sam_at();
    for k in 0..14 {
        t.p.point_at([s[0] + 4.0 * (k as f64).cos(), s[1] + 4.0 * (k as f64).sin()]);
        t.frames(30);
        t.p.press(Button::Left); t.frames(2); t.p.release(Button::Left); t.frames(40);
    }
    let shots = t.p.g.fa[rifle].shots_fired as usize;
    t.reached("shells by the trigger", shots);
    t.frames(240);
    t.p.g.ai_enabled = false;
    let guns = [Some(WeaponId::M870), Some(WeaponId::M1014)];
    // locked doors, at the lock and by the hinge; a leaf with an arm through it
    for (k, d) in t.p.g.doors.list.clone().into_iter().take(10).enumerate() {
        if !t.p.g.dr[d].leaf.latched { continue; }
        t.p.g.dr[d].leaf.locked = true;
        let side = if k % 2 == 0 { 1.0 } else { -1.0 };
        let at = game::shotgun::lock_point(&t.p.g, d, side);
        let mid = t.p.g.dr[d].frame_centre;
        let off = [at[0] - mid[0], 0.0, at[2] - mid[2]];
        for n in WAYS {
            let out = add(at, n, -0.7);
            if t.between(out, add(at, n, 0.3)).is_none_or(|i| i < 1e6) { continue; }
            let c = t.shooters()[k % 2 % t.shooters().len()];
            t.fires(c, add(out, n, -0.3), out, add(at, n, 2.0), guns[k % 2]);
            if t.p.g.dr[d].leaf.latched { t.fires(c, add(out, n, -0.3), out, add([at[0] - 2.0 * off[0], at[1], at[2] - 2.0 * off[2]], n, 1.0), Some(WeaponId::M870)); }
            break;
        }
        t.frames(10);
        if !t.p.g.dr[d].leaf.latched { t.reached("locks taken by a shell", 1); }
    }
    // panes, lamps, loose things, walls
    for (k, (i, at, _)) in statics(t, 24, |t, i| game::glass::is_pane(&t.p.g, t.stat(i))).into_iter().enumerate() {
        for n in WAYS { if t.between(add(at, n, -1.2), add(at, n, 0.1)) == Some(i) { let c = t.shooters()[k % t.shooters().len()]; t.through(c, at, n, if k % 3 == 0 { 0.0 } else { 1.2 }, guns[k % 2]); t.reached("panes shot with a shell", 1); break; } }
    }
    for (k, l) in t.p.g.targets.clone().into_iter().take(16).enumerate() {
        let at = t.p.g.tab.fixture[l].pos;
        for n in WAYS {
            let from = add([at[0], at[1] - 0.4, at[2]], n, -2.0);
            if t.between(from, add(at, n, -0.15)).is_some() { continue; }
            let c = t.sam();
            t.fires(c, add(from, n, -0.3), from, at, guns[k % 2]);
            break;
        }
        if t.p.g.tab.fixture[l].broken { t.reached("lamps shot out", 1); }
    }
    for (k, pr) in t.p.g.props.props.clone().into_iter().take(12).enumerate() {
        let at = { let q = &t.p.g.tab.props_prop[pr]; let b = &t.p.g.tab.boxes[q.boxes[0]]; b.c };
        let n = WAYS[k % 4];
        let c = t.shooters()[k % t.shooters().len()];
        t.through(c, at, n, 1.0, guns[k % 2]);
    }
    // men: near (a drop), past the drop's reach (pellets counted), and peppered
    let men: Vec<GuardId> = t.p.g.guards.iter().copied().filter(|&m| t.p.up(m)).collect();
    for (k, m) in men.into_iter().enumerate() {
        if k >= 18 { break; }
        let c = t.p.g.gd[m].ch;
        let at = t.p.g.ch[c].pos;
        let chest = [at[0], at[1] + 1.2, at[2]];
        let far = [1.2, 5.0, 11.0, 16.0][k % 4];
        for n in WAYS {
            let from = add(chest, n, -far);
            if t.between(from, add(chest, n, -0.4)).is_some() { continue; }
            let s = t.sam();
            t.fires(s, add(from, n, -0.4), from, [chest[0] + if k % 3 == 2 { 0.35 } else { 0.0 }, chest[1], chest[2]], guns[k % 2]);
            break;
        }
    }
    let st = t.p.g.stats.shotgun.clone();
    for (what, n) in [("pellets in men", st.in_men), ("men dropped by a shell", st.dropped), ("men peppered", st.peppered)] { t.reached(what, n as usize); }
    t.p.g.ai_enabled = true;
    t.frames(120);
}

fn bars(t: &mut T) {
    t.p.g.ai_enabled = false;
    t.frames(6);
    let guns = [None, Some(WeaponId::M870), Some(WeaponId::Akms), Some(WeaponId::M1014), Some(WeaponId::Fiveseven)];
    let list = statics(t, 400, |t, i| game::winbars::bar_at(&t.p.g, i));
    let step = (list.len() / 40).max(1);
    for (k, &(i, at, h)) in list.iter().step_by(step).enumerate() {
        if !game::winbars::bar_at(&t.p.g, i) { continue; }
        let at = [at[0], at[1] + h[1] * ((k % 3) as f64 - 1.0) * 0.4, at[2]];
        for n in WAYS {
            if t.between(add(at, n, -0.6), add(at, n, 0.1)) != Some(i) || t.between(add(at, n, 0.6), add(at, n, -0.1)) != Some(i) { continue; }
            let n = if k % 2 == 0 { n } else { [-n[0], 0.0, -n[2]] };
            let who = t.shooters();
            let c = who[k % who.len()];
            let gun = guns[k % 5];
            match k % 4 {
                0 | 1 => t.through(c, at, n, 0.0, gun),
                2 => t.through(c, at, n, 1.5, gun),
                _ => {   // (aslant: along the row of them)
                    let side = [n[2], 0.0, -n[0]];
                    let d = { let d = [n[0] * 0.2 + side[0], 0.0, n[2] * 0.2 + side[2]]; let l = (d[0] * d[0] + d[2] * d[2]).sqrt(); [d[0] / l, 0.0, d[2] / l] };
                    let from = add(at, d, -1.0);
                    t.fires(c, add(from, d, -0.3), from, add(at, d, 4.0), gun);
                }
            }
            t.frames(6);
            break;
        }
    }
    let gone = sim::panes::bars_gone(t.p.g.sim) as usize;
    t.reached("bars broken", gone);
    t.reached("bars on the level", list.len());
    // the arm through a pane, a wall, a shut leaf
    for (k, (i, at, _)) in statics(t, 10, |t, i| game::glass::is_pane(&t.p.g, t.stat(i))).into_iter().enumerate() {
        for n in WAYS { if t.between(add(at, n, -0.4), add(at, n, 0.2)) == Some(i) { let who = t.shooters(); t.through(who[k % who.len()], at, n, 0.0, guns[k % 5]); break; } }
    }
    let walls: Vec<BoxId> = t.p.g.destr.walls.list.iter().map(|&b| t.p.g.tab.wallbreak_breakable[b].box_).collect();
    for (k, b) in walls.into_iter().take(14).enumerate() {
        let (c, h) = { let x = &t.p.g.tab.boxes[b]; (x.c, x.h) };
        let (n, th) = if h[0] < h[2] { ([1.0, 0.0, 0.0], h[0]) } else { ([0.0, 0.0, 1.0], h[2]) };
        let who = t.shooters();
        let at = [c[0], 1.3, c[2]];
        t.fires(who[k % who.len()], add(at, n, -th - 0.3), add(at, n, if k % 2 == 0 { 0.0 } else { th + 0.2 }), add(at, n, 5.0), guns[k % 5]);
    }
    for (k, d) in t.p.g.doors.list.clone().into_iter().take(10).enumerate() {
        let at = t.p.g.dr[d].frame_centre;
        let at = [at[0], 1.2, at[2]];
        for n in WAYS { if t.between(add(at, n, -0.4), add(at, n, 0.2)).is_some_and(|i| i >= 1e6) { let who = t.shooters(); t.through(who[k % who.len()], at, n, 0.0, guns[k % 5]); break; } }
    }
    t.frames(60);
}

fn sheets(t: &mut T, capped: bool) {
    t.p.g.ai_enabled = false;
    t.frames(6);
    // the sheets' list is made when a wall first opens: one is, off in a corner of the list
    let walls: Vec<WallbreakBreakableId> = t.p.g.destr.walls.list.clone();
    let blow = |t: &mut T, b: WallbreakBreakableId, u: f64, y: f64, power: Option<f64>, side: f64| {
        let bx = t.p.g.tab.wallbreak_breakable[b].box_;
        let (c, h) = { let x = &t.p.g.tab.boxes[bx]; (x.c, x.h) };
        let mut at = c;
        let (n, l) = if h[0] < h[2] { (0, 2) } else { (2, 0) };
        at[l] += h[l] * u; at[n] += (h[n] + 0.3) * side; at[1] = y;
        let o = game::wallbreak::BlastOpts { who: Some(game::consts::PLAYER_ID), chaos: Some(false), power, ..Default::default() };
        game::destruction::destruction_blast(&mut t.p.g, at, Some(o));
    };
    if let Some(&b) = walls.last() { blow(t, b, 0.0, 0.6, None, 1.0); t.frames(3); }
    let cand = t.p.g.destr.walls.skins.cand.clone();
    t.reached("sheets on the level", cand.len());
    let drawn = t.p.g.col.boxes.iter().find_map(|&b| t.p.g.tab.boxes[b].marks.clone().filter(|m| !m.is_empty()));
    for (k, &j) in cand.iter().enumerate() {
        let b = t.p.g.col.boxes[j as usize];
        let has = t.p.g.tab.boxes[b].marks.as_ref().is_some_and(|m| !m.is_empty());
        if has { t.reached("sheets marked as authored", 1); }
        if k % 3 != 0 {
            let mark = drawn.clone().unwrap_or_else(|| vec![Default::default()].into());
            t.p.g.tab.boxes.edit(b).marks = Some(mark);
            t.reached("sheets marked here", 1);
        }
        if k % 3 == 2 {
            let h = t.p.g.tab.boxes[b].h;
            let r = std::f64::consts::FRAC_1_SQRT_2;
            let far = r * (h[1] + h[0].max(h[2])) * 0.96;
            let long = if h[0] < h[2] { 2 } else { 0 };
            let planes = [(r, r), (r, -r), (-r, r), (-r, -r), (0.0, 1.0)].map(|(u, v)| { let mut n = [0.0, v, 0.0]; n[long] = u; game::scene_cut::CutPlane { n, d: if u == 0.0 { h[1] } else { far } } });
            t.p.g.tab.boxes.edit(b).cut = Some(planes.to_vec());
            t.reached("sheets given a shape", 1);
        }
    }
    for (k, &b) in walls.iter().enumerate().take(60) {
        if capped { (t.p.g.k.wallskin.per_skin, t.p.g.k.wallskin.max_pieces) = if k % 20 < 10 { (1.0, 256.0) } else { (12.0, 10.0) }; }
        let (u, y) = ([0.0, 0.4, -0.5][k % 3], [1.0, 0.5, 1.8, 1.3][k % 4]);
        blow(t, b, u, y, if k % 5 == 4 { Some(2.0) } else { None }, if k % 2 == 0 { 1.0 } else { -1.0 });
        t.frames(3);
        // (a long wall: two holes more, so that a sheet along it stands in many pieces)
        let long = { let h = t.p.g.tab.boxes[t.p.g.tab.wallbreak_breakable[b].box_].h; h[0].max(h[2]) > 1.2 };
        if long { blow(t, b, -0.75, 0.7, None, 1.0); t.frames(2); blow(t, b, 0.75, 1.1, None, -1.0); t.frames(2); }
        // …chewed by rounds and a shell beside the hole, and a second charge by the first on every fourth
        let bx = t.p.g.tab.wallbreak_breakable[b].box_;
        let (c, h) = { let x = &t.p.g.tab.boxes[bx]; (x.c, x.h) };
        let (n, l) = if h[0] < h[2] { ([1.0, 0.0, 0.0], 2) } else { ([0.0, 0.0, 1.0], 0) };
        let mut at = [c[0], 1.1, c[2]];
        at[l] += h[l] * (u + 0.25);
        let s = t.sam();
        t.through(s, at, n, 1.4, if k % 2 == 0 { Some(WeaponId::Akms) } else { Some(WeaponId::M870) });
        if k % 4 == 3 { blow(t, b, u + 0.35, y, None, 1.0); t.frames(2); }
        // …and the wall engine asked of a heavy round by itself
        if k % 7 == 6 {
            let (id, pkm) = (t.p.g.tab.wallbreak_breakable[b].id, t.p.g.m.weapons.weapons.get(WeaponId::Pkm).unwrap().clone());
            game::wallbreak::wall_break_round(&mut t.p.g, game::consts::PLAYER_ID, at, n, id, &pkm, 1.0);
            t.frames(1);
        }
    }
    let open = t.open_walls();
    t.reached("walls opened", open);
    t.frames(30);
}

fn scene(name: &str) {
    let (level, what) = name.rsplit_once('.').expect("LEVEL.SCENE");
    let mut t = T::new(level, what == "shells");
    match what { "shells" => shells(&mut t), "bars" => bars(&mut t), "sheets" => sheets(&mut t, false), "capped" => sheets(&mut t, true), _ => panic!("no scene '{what}'") }
    t.ends();
    for (what, n) in &t.n { println!("{name} reached {what}: {n}"); }
    println!("{name} frames {}", t.frames);
    for (n, s) in parts::names().iter().zip(&t.sums) { println!("{name} {n} {s:016x}"); }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() == 2 && a[0] == "one" {
        let name = a[1].clone();
        // (the start's frames are deep: a thread with room)
        if std::thread::Builder::new().stack_size(1 << 30).spawn(move || scene(&name)).unwrap().join().is_err() { std::process::exit(2); }
        return;
    }
    let all: Vec<String> = if a.is_empty() { LEVELS.iter().flat_map(|l| SCENES.iter().map(move |s| format!("{l}.{s}"))).collect() } else { a };
    let me = std::env::current_exe().unwrap();
    let mut bad = 0;
    for some in all.chunks(3) {
        let kids: Vec<_> = some.iter().map(|n| std::process::Command::new(&me).args(["one", n]).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap()).collect();
        for (n, k) in some.iter().zip(kids) {
            let o = k.wait_with_output().unwrap();
            print!("{}", String::from_utf8_lossy(&o.stdout));
            if !o.status.success() { bad += 1; println!("{n} STOPPED: {}", String::from_utf8_lossy(&o.stderr).lines().filter(|l| !l.trim().is_empty()).take(2).collect::<Vec<_>>().join(" | ")); }
        }
    }
    eprintln!("thin: {} of {} scenes played", all.len() - bad, all.len());
    std::process::exit(if bad > 0 { 1 } else { 0 });
}
