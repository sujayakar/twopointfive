//! WHERE HIS MAN WENT: his eyes have just lost him behind something, and his fix is led on the way he was going.
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::character::Bone;
use crate::v3;

pub fn lead(g: &mut Game, gd: GuardId, c: CharId, fc: Option<CharId>) {
    let now = g.time;
    if g.gd[gd].sight_t < now && g.gd[gd].sight_t > g.gd[gd].led_t {
        let fresh = now - g.gd[gd].sight_t < 0.25;
        let on_fix = fresh && crate::belief::fix_at_sight(g, gd);
        if let (true, Some(pc)) = (on_fix, fc) {
            let eye = eye_of_char(g, c);
            let chest = match g.ch[pc].bones.get(Bone::Chest).copied() { Some(b) => b, None => v3::add(g.ch[pc].pos, [0.0, 1.1, 0.0]) };
            if eye_blocked(g, gd, eye, chest) {
                let head = match g.ch[pc].bones.get(Bone::Head).copied() { Some(b) => b, None => v3::add(g.ch[pc].pos, [0.0, 1.6, 0.0]) };
                if eye_blocked(g, gd, eye, head) {
                    let (sp, sv) = (g.gd[gd].sight_pos, g.gd[gd].sight_vel);
                    let led = lead_fix(g, sp, sv);
                    crate::mind::fix(g, gd, Some(led), "guards");
                    g.gd[gd].led_at = Some(led);
                }
            }
        }
        g.gd[gd].led_t = g.gd[gd].sight_t;
    }
}
