//! src/game/smokebreak.ts lines 1 to 167
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The host's book of what was filed, for the panel and the tools; None before a man smokes.   (smokebreak.ts smokeShown)
pub fn smoke_shown(g: &mut crate::game::Game) -> Option<crate::smokebreak::SmokeShown2> {
    let s = g.engine.host_books.as_ref()?.smoke_break.as_ref()?;
    Some(SmokeShown2 { men: s.men.clone(), frame: s.frame, threads: s.threads, records: s.records })
}

/// A blank row of counts.   (smokebreak.ts fresh)
pub fn fresh() -> crate::smokebreak::SmokeBreakStats {
    SmokeBreakStats { men: 0.0, lit: 0.0, draws: 0.0, flames: 0.0, embers: 0.0, breaths: 0.0, frames: 0.0 }
}

/// Make the game's row if it has none yet.
fn ensure_state(g: &mut Game) {
    if g.m.smokebreak.states.is_none() {
        g.m.smokebreak.states = Some(State { men: JsMap::new(), stats: fresh(), seed: 0x51607e as f64 });
    }
}

/// The game's row, made on first asking. A COPY: a writer goes through `g.m.smokebreak.states`.   (smokebreak.ts stateOf)
pub fn state_of(g: &mut crate::game::Game) -> crate::smokebreak::State {
    ensure_state(g);
    g.m.smokebreak.states.clone().unwrap()
}

/// The counts a probe or the panel reads; a game nobody has smoked in gets a blank row and none is made.   (smokebreak.ts smokeBreakStats)
pub fn smoke_break_stats(g: &mut crate::game::Game) -> crate::smokebreak::SmokeBreakStats {
    match g.m.smokebreak.states.as_ref() {
        Some(s) => s.stats,
        None => fresh(),
    }
}

/// A word's bits, stirred. Everything is modulo 2^32 in the TypeScript too, so u32 arithmetic gives its bits.   (smokebreak.ts mix)
pub fn mix(x: f64) -> f64 {
    let x0 = js::to_u32(x);
    let mut x = (x0 ^ 61) ^ (x0 >> 16);
    x = x.wrapping_add(x << 3);
    x ^= x >> 4;
    x = x.wrapping_mul(0x27d4eb2d);
    x ^= x >> 15;
    x as f64
}

/// `gd.script?.smoke`, as a condition.
fn script_smokes(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].script.as_ref().map_or(false, |s| s.smoke.unwrap_or(false))
}

/// Is this man smoking now: a script that says so, on his feet where it sent him, nobody's arm round him.   (smokebreak.ts smoking)
pub fn smoking(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(s) = g.gd[gd].script.as_ref() else { return false };
    s.smoke.unwrap_or(false)
        && g.gd[gd].state != crate::game::GuardState::Dead
        && g.gd[gd].held.is_none()
        && g.ch[g.gd[gd].ch].alive
        && (s.goal.is_none() || s.arrived.unwrap_or(false))
}

/// `hand_at` over the knob table alone.
fn hand_of(k: &crate::knobs::SmokebreakK, t: f64) -> HandAt {
    if t < k.light_secs {
        let up = t >= k.light_up[0] && t < k.light_up[1];
        return HandAt { up: if up { 1.0 } else { 0.0 }, drawn: up && t >= k.flame_at + k.flame * 0.5, cycle: -1.0, into: t };
    }
    let u = t - k.light_secs;
    let cycle = m::floor(u / k.every);
    let into = u - cycle * k.every;
    let at = k.every - k.draw; // (the draw ends each round: he has just lit it)
    let up = into >= at;
    HandAt { up: if up { 1.0 } else { 0.0 }, drawn: up && into >= at + 0.6, cycle, into }
}

/// How far up his hand is asked this second of his smoke (0‥1), and whether the end is at his mouth.   (smokebreak.ts handAt)
pub fn hand_at(g: &mut crate::game::Game, t: f64) -> crate::smokebreak::HandAt {
    hand_of(&g.k.smokebreak, t)
}

/// His mouth, off the head joint, and the way his breath leaves it.   (smokebreak.ts mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, c_2: CharId) -> crate::smokebreak::MouthOf {
    let c = c_2;
    let hd = crate::character::character_head_dir(g, c);
    let head = *g.ch[c].bones.get(crate::character::Bone::Head).unwrap();
    let hl = m::hyp2(hd[0], hd[2]);
    let k = &g.k.smokebreak;
    let body_yaw = g.ch[c].body_yaw;
    let fx = if hl > 0.2 { hd[0] / hl } else { m::sin(body_yaw) };
    let fz = if hl > 0.2 { hd[2] / hl } else { m::cos(body_yaw) };
    let l = m::hyp2(1.0, k.breath.down);
    MouthOf {
        pos: [head[0] + fx * k.mouth_ahead, head[1] - k.mouth_down, head[2] + fz * k.mouth_ahead],
        dir: [fx / l, -k.breath.down / l, fz / l],
    }
}

/// The lit end: off the left hand's joint along the forearm's line, a little up.   (smokebreak.ts tipOf)
pub fn tip_of(g: &mut crate::game::Game, c_2: CharId) -> Option<V3> {
    let c = c_2;
    let h = *g.ch[c].bones.get(crate::character::Bone::HandL)?;
    let e = g.ch[c].bones.get(crate::character::Bone::ForearmL).copied();
    let k = &g.k.smokebreak;
    let body_yaw = g.ch[c].body_yaw;
    let mut d: V3 = [m::sin(body_yaw), 0.0, m::cos(body_yaw)];
    if let Some(e) = e {
        let (x, y, z) = (h[0] - e[0], h[1] - e[1], h[2] - e[2]);
        let l = m::hyp3(x, y, z);
        if l > 1e-4 {
            d = [x / l, y / l, z / l];
        }
    }
    Some([h[0] + d[0] * k.tip_out, h[1] + d[1] * k.tip_out + k.tip_up, h[2] + d[2] * k.tip_out])
}

/// The field the TypeScript hangs on a Man by a cast: when what he breathes out next leaves.
fn seam_out_at(m: &Man) -> Option<f64> {
    todo!("needs: Man.out_at: Option<f64> (smokebreak.ts:143)")
}
/// Its writer.
fn seam_set_out_at(m: &mut Man, at: Option<f64>) {
    todo!("needs: Man.out_at: Option<f64> (smokebreak.ts:143)")
}

/// One smoking man's frame: everything of the loop's body after he is counted (a `continue` there is a return here).
fn smoke_one(g: &mut Game, gd: GuardId, c: CharId, dt: f64, mn: &mut Man, t: &mut SmokeBreakStats) {
    let was = hand_at(g, mn.t);
    mn.t += dt;
    let now = hand_at(g, mn.t);
    g.ch[c].anim.smoke = Some(now.up);
    let hidden = crate::fogofwar::fog_of_war_hidden(g, gd);
    let tip = tip_of(g, c);
    let id = g.ch[c].id;
    let (flame_at, flame) = (g.k.smokebreak.flame_at, g.k.smokebreak.flame);
    // the lighter's flame, once
    if !mn.flamed && mn.t >= flame_at {
        mn.flamed = true;
        let a = mouth_of(g, c);
        let k = g.k.smokebreak;
        crate::render_lights::light_set_flash(&mut g.engine.lights, [a.pos[0] + a.dir[0] * 0.06, a.pos[1], a.pos[2] + a.dir[2] * 0.06], k.flame_color, k.flame_k, k.flame_range, k.flame, id);
        t.flames += 1.0;
    }
    // the end: a low glow all the while it is lit, bright at a draw
    if let (true, Some(tip)) = (mn.t >= flame_at + flame * 0.5, tip) {
        mn.ember_t -= dt;
        if mn.ember_t <= 0.0 {
            let k = g.k.smokebreak;
            mn.ember_t = k.ember_every;
            crate::render_lights::light_set_flash(&mut g.engine.lights, tip, k.ember_color, if now.drawn { k.ember_draw_k } else { k.ember_k }, k.ember_range, k.ember_every * 1.6, id);
            t.embers += 1.0;
        }
    }
    if js::truthy(now.up) && !js::truthy(was.up) && now.cycle >= 0.0 {
        mn.draws += 1.0;
        t.draws += 1.0;
    }
    if hidden {
        return; // (nothing live shows in the fog: the lights are the meter's and stay)
    }
    // what he breathes out: after the lighting and after every draw, as the hand comes down
    let due = if js::truthy(was.up) && !js::truthy(now.up) { mn.t } else { -1.0 };
    if due >= 0.0 {
        mn.outs += 1.0;
        seam_set_out_at(mn, Some(mn.t + g.k.smokebreak.out_after));
    }
    if let Some(out_at) = seam_out_at(mn) {
        if mn.t >= out_at {
            seam_set_out_at(mn, None);
            t.breaths += 1.0;
        }
    }
}

/// Once a frame, after the characters are posed: each told man's clock, hand and lights. His smoke is the dressing's.   (smokebreak.ts updateSmokeBreak)
pub fn update_smoke_break(g: &mut crate::game::Game, dt: f64) {
    if !(dt > 0.0) {
        return;
    }
    let mut any = false;
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if script_smokes(g, gd) || g.ch[g.gd[gd].ch].anim.smoke.is_some() {
            any = true;
            break;
        }
    }
    if !any {
        return; // (nothing of this file runs on a floor where nobody was told to smoke)
    }
    ensure_state(g);
    let mut t = g.m.smokebreak.states.as_ref().unwrap().stats;
    t.frames += 1.0;
    let mut men = 0.0;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let c = g.gd[gd].ch;
        let had = g.m.smokebreak.states.as_ref().unwrap().men.get(c).copied();
        if !smoking(g, gd) {
            if had.map_or(false, |x| x.live) || g.ch[c].anim.smoke.is_some() {
                g.ch[c].anim.smoke = None;
                if let Some(x) = g.m.smokebreak.states.as_mut().unwrap().men.get_mut(c) {
                    x.live = false;
                }
            }
            if had.is_some() && !script_smokes(g, gd) {
                g.m.smokebreak.states.as_mut().unwrap().men.delete(c); // (told again later, he lights another)
            }
            continue;
        }
        let mut mn = match had {
            Some(x) => x,
            None => {
                t.lit += 1.0;
                Man { t: 0.0, flamed: false, draws: 0.0, outs: 0.0, ember_t: 0.0, live: true }
            }
        };
        mn.live = true;
        men += 1.0;
        smoke_one(g, gd, c, dt, &mut mn, &mut t);
        g.m.smokebreak.states.as_mut().unwrap().men.set(c, mn);
    }
    t.men = men;
    g.m.smokebreak.states.as_mut().unwrap().stats = t;
}

/// One line for the panel and the probes.   (smokebreak.ts smokeBreakSummary)
pub fn smoke_break_summary(g: &mut crate::game::Game) -> String {
    let t = smoke_break_stats(g);
    let (threads, records) = match smoke_shown(g) {
        Some(p) => (p.threads, p.records),
        None => (0.0, 0.0),
    };
    alloc::format!("smoke break ▸ {} smoking · {} lit, {} draws, {} breathed out ({} threads on the solver, {} puff records)", t.men, t.lit, t.draws, t.breaths, threads, records)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (input, the TypeScript's bits): its own line of the file, run under bun.
    #[test]
    fn mix_gives_the_typescripts_bits() {
        let rows: [(f64, u64); 13] = [
            (0f64, 0x41e815292d400000),
            (1f64, 0x41c3c9164e800000),
            (5333118f64, 0x41ee72ec94a00000),
            (4294967295f64, 0x41dc3d2674c00000),
            (2147483648f64, 0x41e5a2d542800000),
            (1234567f64, 0x41ed94702b400000),
            (-1f64, 0x41dc3d2674c00000),
            (3.7f64, 0x41e0fa0df7c00000),
            (4294967301f64, 0x41e8c15dd1a00000),
            (16f64, 0x41e3ab5e65600000),
            (61f64, 0x0000000000000000),
            (2147483647f64, 0x41b8ce91a6000000),
            (2618109713f64, 0x41c4299574800000),
        ];
        for (x, bits) in rows {
            assert_eq!(mix(x).to_bits(), bits, "mix({x})");
        }
    }

    /// The hand's beats on the page's numbers, against the TypeScript's handAt: (t, up, drawn, cycle, into's bits).
    #[test]
    fn hand_beats_are_the_typescripts() {
        let k = crate::knobs::page_smokebreak();
        let rows: [(f64, f64, bool, f64, u64); 17] = [
            (0f64, 0f64, false, -1f64, 0x0000000000000000),
            (0.49f64, 0f64, false, -1f64, 0x3fdf5c28f5c28f5c),
            (0.5f64, 1f64, false, -1f64, 0x3fe0000000000000),
            (1.3f64, 1f64, false, -1f64, 0x3ff4cccccccccccd),
            (1.75f64, 1f64, true, -1f64, 0x3ffc000000000000),
            (2.59f64, 1f64, true, -1f64, 0x4004b851eb851eb8),
            (2.6f64, 0f64, false, -1f64, 0x4004cccccccccccd),
            (3.19f64, 0f64, false, -1f64, 0x4009851eb851eb85),
            (3.2f64, 0f64, false, 0f64, 0x0000000000000000),
            (5f64, 0f64, false, 0f64, 0x3ffccccccccccccc),
            (8.6f64, 0f64, false, 0f64, 0x4015999999999999),
            (9f64, 1f64, false, 0f64, 0x4017333333333333),
            (9.2f64, 1f64, false, 0f64, 0x4017ffffffffffff),
            (10.69f64, 1f64, true, 0f64, 0x401df5c28f5c28f5),
            (10.7f64, 1f64, true, 0f64, 0x401dffffffffffff),
            (20f64, 0f64, false, 2f64, 0x3ffcccccccccccd0),
            (123.456f64, 0f64, false, 16f64, 0x3fd0624dd2f1aa00),
        ];
        for (t, up, drawn, cycle, into) in rows {
            let h = hand_of(&k, t);
            assert_eq!((h.up, h.drawn, h.cycle, h.into.to_bits()), (up, drawn, cycle, into), "t={t}");
        }
    }
}
