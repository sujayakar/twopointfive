//! WHAT A HOST IS TOLD OF THE GAME beside what it draws, at face_out: views (their bytes, read on the page by src/view/read.ts) and a few lines of JSON. Reads: no word of the state is written.
use game::bytes::{ToBytes, W};

fn told(v: &dyn ToBytes) -> Vec<u8> { let mut o = W::new(); v.w(&mut o); o.b }
/// WHAT A HUD IS TOLD of the game as its last tick left it (`view::tick_view`), about the seat this machine shows: a `TickView`'s bytes at face_out, read
/// by the host with the table's own reader. A read: no word of the game's state is written and no pass about a seat is opened.
#[no_mangle]
pub extern "C" fn face_tick_view(h: u32) {
    super::out(super::with_ref(h, |x| told(&game::view::tick_view(&x.g, &game::seats::seat(&x.g, x.g.local_seat.unwrap_or(x.g.seat))))));
}
/// what the end card shows of the run (a DebriefView at face_out): asked once, as the card opens; a read, nothing kept
#[no_mangle]
pub extern "C" fn face_debrief_view(h: u32) {
    super::out(super::with_ref(h, |x| { let g = &x.g; told(&game::view::debrief_view(g, &game::seats::seat(g, g.local_seat.unwrap_or(g.seat)))) }))
}
/// what a machine keeps of the run (a RunView at face_out): asked as the try begins and as the run ends; a read
#[no_mangle]
pub extern "C" fn face_run_view(h: u32) {
    super::out(super::with_ref(h, |x| told(&game::verdict::run_view(&x.g))))
}
/// WHAT A HUD IS TOLD of what the cursor is on (`view::cursor_view`), about a seat by its place in the game's list (−1: the one this machine shows): a `CursorView`'s
/// bytes at face_out. `p` = 36 doubles the host has as it draws: the cursor (x y), the picture it is in (w h), the camera's view and projection, and their
/// inverse. Nothing of the game is written.
/// # Safety
/// `p` names 288 bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_cursor_view(h: u32, seat: i32, p: *const u8) {
    let b = super::bytes(p, 36 * 8);
    let n = |i: usize| f64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
    let m16 = |at: usize| -> [f32; 16] { std::array::from_fn(|i| n(at + i) as f32) };
    let now = game::view::HostNow { mouse: [n(0), n(1)], client: [n(2), n(3)], view_proj: m16(4), inv_view_proj: m16(20) };
    super::out(super::with_ref(h, |x| {
        let g: &super::Game = &x.g;
        let s = if seat >= 0 && (seat as usize) < g.seats.len() { g.seats[seat as usize] } else { g.local_seat.unwrap_or(g.seat) };
        told(&game::view::cursor_view(g, &game::seats::seat(g, s), &now))
    }));
}
/// THE MISSION'S VERDICT, for a host with no picture (a server): 0 while the mission is being played, 1 lost, 2 won; when not 0 the latched record is
/// at face_out as lines (game::verdict::verdict_lines: the end's word, the why, the seconds, the way won, the counts, the men). The game's own tick
/// latches it: the first end of a run stays. While it is 0 nothing is allocated and face_out is left as it was.
#[no_mangle]
pub extern "C" fn face_verdict(h: u32) -> i32 {
    match super::with_ref(h, |x| game::verdict::verdict_lines(&x.g)) {
        None => 0,
        Some((won, s)) => { super::out(s.into_bytes()); if won { 2 } else { 1 } }
    }
}
/// what a measuring take notes of the game, as JSON: each man by what he is at and whether he carries a light, the reaction force, the alarm, the run's saves
#[no_mangle]
pub extern "C" fn face_tally(h: u32) {
    let s = super::with_ref(h, |x| {
        let g = &x.g;
        let men: Vec<String> = g.guards.iter().map(|&gd| { let m = &g.gd[gd]; format!("[\"{}\",{}]", m.state.as_str(), g.tab.light[m.flashlight].enabled as u32) }).collect();
        let (s, m) = (&g.saves.stats, &g.mission.stats);
        let word = |w: Option<&str>| w.map_or(String::from("null"), |w| format!("\"{}\"", w.replace('\\', "").replace('"', "")));
        format!("{{\"men\":[{}],\"qrf\":\"{}\",\"escalation\":{},\"autosaves\":{},\"manual\":{},\"loads\":{},\"stage\":\"{}\",\"alarm\":{},\"alerts\":{},\"clear\":{},\"kind\":{},\"stats\":{{\"elapsed\":{},\"alerts\":{},\"spotted\":{},\"bodies\":{},\"shots\":{},\"kills\":{},\"knockouts\":{},\"keys\":{},\"blackout\":{},\"sandbox\":{},\"qrf\":{},\"qrfAt\":{},\"qrfWho\":{}}}}}", men.join(","), g.qrf.state.as_str(), g.escalation, s.autosaves, s.manual, s.loads, g.mission.stage.as_str(), g.mission.alarm_on, g.mission.stats.alerts, game::roomclear::room_clear_scenario_active(g),
            word(g.verdict.res.as_ref().and_then(|r| r.kind).map(|k| k.as_str())), m.elapsed, m.alerts, m.spotted, m.bodies, m.shots, m.kills, m.knockouts, m.keys, m.blackout, m.sandbox, m.qrf, m.qrf_at, word(m.qrf_who.as_deref()))
    });
    super::out(s.into_bytes());
}
/// every knob a person may turn, at face_out: its name's length (its top bit: a yes or no), the name, what it is turned to (f64)
#[no_mangle]
pub extern "C" fn face_knobs(h: u32) {
    let b = super::with(h, |x| {
        let mut b = Vec::new();
        for (name, yes, v) in game_start::host::knobs::all(&x.g) {
            b.extend_from_slice(&(name.len() as u32 | (yes as u32) << 31).to_le_bytes());
            b.extend_from_slice(name.as_bytes());
            b.extend_from_slice(&v.to_le_bytes());
        }
        b
    });
    super::out(b);
}
/// the panel's rows of the game's numbers (game_start host/shown.rs), at face_out: each its name, label, section and title (a u32 length, the text), then what it runs over and shows (lo, hi, step, scale: f64), and is it kept on this machine (bit 0) and advanced (bit 1, a u32)
#[no_mangle]
pub extern "C" fn face_shown() {
    let mut b = Vec::new();
    for r in game_start::host::shown::ROWS {
        for t in [r.name, r.label, r.section, r.title] { b.extend_from_slice(&(t.len() as u32).to_le_bytes()); b.extend_from_slice(t.as_bytes()); }
        for v in [r.lo, r.hi, r.step, r.scale, game_start::host::knobs::shipped(r.name).unwrap_or(f64::NAN)] { b.extend_from_slice(&v.to_le_bytes()); }
        b.extend_from_slice(&(r.kept as u32 | (r.adv as u32) << 1).to_le_bytes());
    }
    super::out(b);
}
/// the panel's actions (game_start host/actions.rs), at face_out: each its name, label, section and title (a u32 length, the text), then how it is drawn (u32: 0 a button, 1 a button a word, 2 a pick of words, 3 a count, 4 a pick that is a new page), and what that takes: the words (a u32 count, each as text), and for a pick which stands now (text); lo, hi, step, start (f64); the address's word (text), then the words
#[no_mangle]
pub extern "C" fn face_actions(h: u32) {
    use game_start::host::actions::{Takes, ACTIONS};
    let b = super::with(h, |x| {
        let mut b = Vec::new();
        fn text(b: &mut Vec<u8>, t: &str) { b.extend_from_slice(&(t.len() as u32).to_le_bytes()); b.extend_from_slice(t.as_bytes()); }
        fn words<S: AsRef<str>>(b: &mut Vec<u8>, w: &[S]) { b.extend_from_slice(&(w.len() as u32).to_le_bytes()); for t in w { text(b, t.as_ref()); } }
        for a in ACTIONS {
            for t in [a.name, a.label, a.section, a.title] { text(&mut b, t); }
            match &a.takes {
                Takes::Press => b.extend_from_slice(&0u32.to_le_bytes()),
                Takes::Each(all) => { b.extend_from_slice(&1u32.to_le_bytes()); words(&mut b, &all(&x.g)); }
                Takes::Choice(all, now) => { b.extend_from_slice(&2u32.to_le_bytes()); words(&mut b, &all(&x.g)); text(&mut b, &now(&x.g)); }
                Takes::Count { lo, hi, step, start } => { b.extend_from_slice(&3u32.to_le_bytes()); for v in [*lo, *hi, *step, *start] { b.extend_from_slice(&v.to_le_bytes()); } }
                Takes::Reload { word, picks } => { b.extend_from_slice(&4u32.to_le_bytes()); text(&mut b, word); words(&mut b, picks); }
            }
        }
        b
    });
    super::out(b);
}
/// what each of those rows stands at now, as a row shows it (f64 each, in their order), at face_out
#[no_mangle]
pub extern "C" fn face_shown_now(h: u32) {
    let b = super::with(h, |x| { let mut b = Vec::new(); for r in game_start::host::shown::ROWS { b.extend_from_slice(&(game_start::host::knobs::get(&x.g, r.name).unwrap_or(0.0) * r.scale).to_le_bytes()); } b });
    super::out(b);
}
