//! WHAT AN END CARD IS DRAWN FROM beside its debrief (face_debrief_view): how the run ended with every man of it, the course's par, and the floor its map is of. Asked as a card opens; a read.
fn r2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }
use game_run::kept::text;
fn or(s: Option<String>) -> String { s.unwrap_or_else(|| String::from("null")) }
fn list(v: &[f64]) -> String { let s: Vec<String> = v.iter().map(|&x| format!("{}", r2(x))).collect(); format!("[{}]", s.join(",")) }

fn verdict(g: &game::game::Game) -> Option<String> {
    let v = &g.verdict;
    let r = if v.run == Some(g.mission.serial) { v.res.as_ref()? } else { return None };
    let men: Vec<String> = r.men.iter().map(|m| format!("{{\"id\":{},\"name\":{},\"tone\":{},\"human\":{},\"state\":{},\"downAt\":{},\"downRoom\":{},\"downBy\":{},\"downPos\":{},\"drops\":{},\"path\":{}}}",
        m.id, text(&m.name), or(m.tone.map(|t| text(t.as_str()))), m.human, text(m.state.as_str()), if m.down_at.is_finite() { m.down_at } else { 0.0 }, text(&m.down_room), text(&m.down_by), or(m.down_pos.map(|p| list(&p))), list(&m.drops), list(&m.path))).collect();
    Some(format!("{{\"kind\":{},\"won\":{},\"word\":{},\"why\":{},\"secs\":{},\"approach\":{},\"void\":{},\"clean\":{},\"men\":[{}],\"lost\":{},\"of\":{},\"foes\":{{\"down\":{},\"of\":{},\"fell\":{}}},\"hostages\":{},\"start\":{}}}",
        or(r.kind.map(|k| text(k.as_str()))), r.won, text(&r.word), text(&r.why), r.secs, or(r.approach.map(|a| text(a.as_str()))), r.void, r.clean, men.join(","), r.lost, r.of, r.foes.down, r.foes.of, list(&r.foes.fell),
        or(r.hostages.map(|h| format!("{{\"alive\":{},\"of\":{}}}", h.alive, h.of))), list(&r.start)))
}

/// {verdict, par: {medal, line}, holder, marks: [{glyph, cls, x, z, title}], floor: {w, h, cell, rows}}: `rows` a figure a cell, 0 open ground, 1 a floor indoors, 2 and 3 the same where nothing walks
#[no_mangle]
pub extern "C" fn face_end(h: u32) {
    let s = super::with_ref(h, |x| {
        let g = &x.g;
        let par = game::par::par_result_now(g).map(|p| format!("{{\"medal\":{},\"line\":{}}}", text(p.medal.as_str()), text(&p.line)));
        let mut marks: Vec<String> = Vec::new();
        let mut mark = |glyph: &str, cls: &str, x: f64, z: f64, title: &str| marks.push(format!("{{\"glyph\":{},\"cls\":{},\"x\":{},\"z\":{},\"title\":{}}}", text(glyph), text(cls), r2(x), r2(z), text(title)));
        let at = game::roomclear::room_clear_scenario_start_at(g).unwrap_or(g.level.player_spawn);
        mark("S", "spawn", at[0], at[2], "insertion point");
        let (m, errand) = (&g.level.errand, !g.staged.facts.fight);   // (a fight has its floor and no errand)
        if errand { mark("\u{25ce}", "rack", m.front[0], m.front[2], &format!("{} \u{2014} {}", m.holder, m.thing)); }
        let centre = |name: &str| game::doors::doors_by_name(g, name).map(|d| g.dr[d].frame_centre);
        for e in m.exits.iter().filter(|_| errand) { let c = centre(&e.door).unwrap_or([e.x, 0.0, e.z]); mark("\u{21e5}", "exit", c[0], c[2], &e.name); }
        let nav = &g.col.nav;
        let (w, h) = (nav.w as usize, nav.h as usize);
        let mut rows = String::with_capacity(w * h);
        for z in 0..h { for x in 0..w {
            let blocked = nav.blocked.get(z * w + x) == Some(&1);
            let inside = game::chronicle::indoors([(x as f64 + 0.5) * nav.cell, 0.0, (z as f64 + 0.5) * nav.cell]);
            rows.push(char::from(b'0' + inside as u8 + 2 * blocked as u8));
        } }
        format!("{{\"verdict\":{},\"par\":{},\"holder\":{},\"marks\":[{}],\"floor\":{{\"w\":{},\"h\":{},\"cell\":{},\"rows\":\"{}\"}}}}", or(verdict(g)), or(par), text(if errand { &m.holder } else { "" }), marks.join(","), w, h, nav.cell, rows)
    });
    super::out(s.into_bytes());
}
