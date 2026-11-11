//! WHERE HIS MAN WENT: his eyes have just lost him behind something, and his fix is led on the way he was going.
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::character::Bone;
use crate::v3;

pub(in crate::people) fn lead(g: &mut Game, gd: GuardId, c: CharId, fc: Option<CharId>) {
    let now = g.time;
    if g.gd[gd].sight_t < now && g.gd[gd].sight_t > g.gd[gd].led_t {
        let fresh = now - g.gd[gd].sight_t < 0.25;
        let on_fix = fresh && crate::belief::fix_at_sight(g, gd);
        if let (true, Some(pc)) = (on_fix, fc) {
            let eye = eye_of_char(g, c);
            let chest = g.ch[pc].bone_or(Bone::Chest, 1.1);
            if eye_blocked(g, gd, eye, chest) {
                let head = g.ch[pc].bone_or(Bone::Head, 1.6);
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
