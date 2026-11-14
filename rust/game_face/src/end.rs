//! WHAT AN END CARD IS DRAWN FROM beside its debrief: how the run ended with every man of it, the course's par, and the floor its map is of. Asked as a card opens.
use super::Held;
use game::view::{EndView, OutcomeView, ParLine, RunMark};

fn latched(x: &Held) -> Option<&game::verdict::RunVerdict> { let v = &x.g.verdict; if v.run == Some(x.g.mission.serial) { v.res.as_ref() } else { None } }

pub fn verdict(x: &Held) -> Option<OutcomeView> {
    latched(x).map(|r| OutcomeView { won: r.won, word: r.word.clone(), why: r.why.clone(), secs: r.secs, approach: r.approach, lost: r.lost, of: r.of, foes_down: r.foes.down, foes_of: r.foes.of, hostages: r.hostages, void: r.void, clean: r.clean })
}
pub fn end_view(x: &Held) -> EndView {
    let g = &x.g;
    let mut verdict = latched(x).cloned();
    for m in verdict.iter_mut().flat_map(|v| &mut v.men) { if !m.down_at.is_finite() { m.down_at = 0.0; } }
    let mut marks: Vec<RunMark> = Vec::new();
    let mut mark = |glyph: &str, cls: &str, x: f64, z: f64, title: &str| marks.push(RunMark { glyph: glyph.into(), cls: cls.into(), x, z, title: title.into() });
    let at = game::roomclear::room_clear_scenario_start_at(g).unwrap_or(g.level.player_spawn);
    mark("S", "spawn", at[0], at[2], "insertion point");
    let (m, errand) = (&g.level.errand, !g.staged.facts.fight);   // (a fight has its floor and no errand)
    if errand { mark("\u{25ce}", "rack", m.front[0], m.front[2], &format!("{} \u{2014} {}", m.holder, m.thing)); }
    let centre = |name: &str| game::doors::doors_by_name(g, name).map(|d| g.dr[d].frame_centre);
    for e in m.exits.iter().filter(|_| errand) { let c = centre(&e.door).unwrap_or([e.x, 0.0, e.z]); mark("\u{21e5}", "exit", c[0], c[2], &e.name); }
    EndView {
        verdict, par: game::par::par_result_now(g).map(|p| ParLine { medal: p.medal, line: p.line.clone() }), holder: if errand { m.holder.clone() } else { String::new() }, marks,
        floor: game::view::walked_floor(g),
    }
}
