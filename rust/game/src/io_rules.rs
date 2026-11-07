//! What of the bridge asks rules by row: above every rule, so it is here and not in the wire.
use game_wire::io::*;
use game_wire::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};
/// WHAT THE HOST ASKS OF AN ORDER ROW (bridge/ask.ts): the closures of a row in the TypeScript, each answered by the match over its arms. `kind` is ask.ts's; `args` is written by the schema. 0 to 7 only read; 8, 9 and 10 write, and the host applies again after them.
pub fn ask(g: &mut crate::game::Game, kind: u32, row: u32, seat: u32, args: &[u8]) -> Vec<u8> {
    use crate::orders::OrderTarget as T;
    let s = <SeatId as Id>::from_ix(seat as usize); let d = <OrdersOrderDefId as Id>::from_ix(row as usize);
    crate::seats::with_seat(g, s, &mut |g: &mut crate::game::Game| { let known = crate::id_tables::id_tables(g); let mut i = R::measured(args, &known); let mut o = W::new(); match kind {
        0 => { let t: T = Io::r(&mut i); crate::orders::order_label(g, d, &t).w(&mut o); }
        1 => { let gd: GuardId = Io::r(&mut i); let t: T = Io::r(&mut i); crate::orders::order_can(g, d, gd, &t).w(&mut o); }
        2 => { let men: Vec<GuardId> = Io::r(&mut i); let t: T = Io::r(&mut i); crate::orders::order_only(g, d, &men, &t).w(&mut o); }
        3 => { let t: T = Io::r(&mut i); let men: Vec<GuardId> = Io::r(&mut i); match crate::orders::order_valid(g, d, &t, &men) { Ok(()) => o.b.push(0), Err(None) => o.b.push(1), Err(Some(w)) => { o.b.push(2); w.w(&mut o); } } }
        4 => { let t: T = Io::r(&mut i); let men: Vec<GuardId> = Io::r(&mut i); crate::orders::order_when(g, d, &t, &men).w(&mut o); }
        5 => { let t: T = Io::r(&mut i); let men: Vec<GuardId> = Io::r(&mut i); crate::orders::order_gone(g, d, &t, &men).w(&mut o); }
        6 => { crate::silent::posture_word(g).w(&mut o); }
        7 => { String::from(crate::silent::posture_mark(g, &crate::seats::here(g), d)).w(&mut o); }
        8 => { crate::silent::posture_given(g, d); }
        9 => { let men: Vec<GuardId> = Io::r(&mut i); let t: T = Io::r(&mut i); let opts: Option<crate::orders::GiveOpts> = Io::r(&mut i); crate::orders::order_issue(g, d, &men, &t, opts.as_ref()).w(&mut o); }
        10 => { let id: String = Io::r(&mut i); let t: Option<T> = Io::r(&mut i); let who: Option<crate::orders::Who> = Io::r(&mut i); let opts: Option<crate::orders::GiveOpts> = Io::r(&mut i); crate::orders::squad_orders_give(g, &id, t.as_ref(), who, opts.as_ref()).w(&mut o); }
        k => panic!("bridge: no question of kind {}", k),
    } assert!(i.p == args.len(), "bridge: {} bytes left over in a question of kind {}", args.len() - i.p, kind); o.b })
}
/// WHAT THE WALK CANNOT READ, DEALT BY KEY: every order row's closures by its id (the arms as the parts register them), the 'go' row by which of its two the game has dealt.
/// A row with an id this does not know stands at its blank: export.ts names it in its report.
pub fn deal_closures(g: &mut crate::game::Game) {
    use crate::orders::*;
    use crate::squadthrow::SquadThrowKind::{Frag as F, Stun as S};
    let small = g.door_tools.go_key == Some(Some(String::from("small")));
    for id in g.tab.orders_order_def.ids().collect::<Vec<_>>() {
        let d = &mut g.tab.orders_order_def[id];
        match d.id.as_str() {
            "move" => { d.label = OrderLabel::Text { text: String::from("move there") }; d.can = Some(OrderCan::Move); d.only = None; d.valid = Some(OrderValid::Move); d.issue = OrderIssue::Move; d.tick = Some(OrderTick::Move); d.gone = None; }
            "stack" => { d.label = OrderLabel::Text { text: String::from("stack up") }; d.can = None; d.only = None; d.valid = Some(OrderValid::Stack); d.issue = OrderIssue::Stack; d.tick = None; d.gone = None; }
            "cover" => { d.label = OrderLabel::Cover; d.can = Some(OrderCan::Cover); d.only = None; d.valid = None; d.issue = OrderIssue::Cover; d.tick = None; d.gone = None; }
            "fallIn" => { d.label = OrderLabel::Text { text: String::from("fall in") }; d.can = None; d.only = None; d.valid = None; d.issue = OrderIssue::FallIn; d.tick = None; d.gone = None; }
            "hold" => { d.label = OrderLabel::Text { text: String::from("hold position") }; d.can = None; d.only = None; d.valid = None; d.issue = OrderIssue::Hold; d.tick = None; d.gone = None; }
            "tools" => { d.label = OrderLabel::Text { text: String::from("door tools") }; d.can = None; d.only = None; d.valid = None; d.issue = OrderIssue::Nothing; d.tick = None; d.gone = None; }
            "held" => { d.label = OrderLabel::Text { text: String::from("threshold held") }; d.can = None; d.only = None; d.valid = None; d.issue = OrderIssue::Nothing; d.tick = None; d.gone = None; if let Some(s) = d.stands_in.as_mut() { s.when = OrderWhen::DoorHeld; } }
            "walltools" => { d.label = OrderLabel::Text { text: String::from("wall tools") }; d.can = None; d.only = None; d.valid = None; d.issue = OrderIssue::Nothing; d.tick = None; d.gone = None; }
            "throwThere" => { d.label = OrderLabel::ThrowOpen; d.can = None; d.only = None; d.valid = None; d.issue = OrderIssue::Nothing; d.tick = None; d.gone = None; }
            "openClear" => { d.label = OrderLabel::DoorOpenClear; d.can = None; d.only = Some(OrderOnly::DoorHeldOrAll); d.valid = Some(OrderValid::DoorOpenClear); d.issue = OrderIssue::DoorOpenClear; d.tick = None; d.gone = None; }
            "bangClear" => { d.label = OrderLabel::DoorBangClear; d.can = None; d.only = Some(OrderOnly::DoorHeldOrAll); d.valid = Some(OrderValid::DoorBangClear); d.issue = OrderIssue::DoorBangClear; d.tick = None; d.gone = None; }
            "charge" => { d.label = OrderLabel::DoorCharge; d.can = None; d.only = None; d.valid = Some(OrderValid::DoorCharge); d.issue = OrderIssue::DoorCharge; d.tick = None; d.gone = Some(OrderGone::DoorCharge); }
            "fragClear" => { d.label = OrderLabel::DoorFragClear; d.can = None; d.only = None; d.valid = Some(OrderValid::DoorFragClear); d.issue = OrderIssue::DoorFragClear; d.tick = None; d.gone = None; }
            "thresholdHold" => { d.label = OrderLabel::DoorThresholdHold; d.can = None; d.only = None; d.valid = Some(OrderValid::DoorThresholdHold); d.issue = OrderIssue::DoorThresholdHold; d.tick = None; d.gone = None; }
            "bigClear" => { d.label = OrderLabel::DoorBigClear; d.can = None; d.only = None; d.valid = Some(OrderValid::DoorBigClear); d.issue = OrderIssue::DoorBigClear; d.tick = None; d.gone = Some(OrderGone::DoorBigClear); }
            "thrWord" => { d.label = OrderLabel::DoorThrWord; d.can = None; d.only = Some(OrderOnly::DoorThrWord); d.valid = Some(OrderValid::DoorThrWord); d.issue = OrderIssue::DoorThrWord; d.tick = None; d.gone = None; }
            "shutDoor" => { d.label = OrderLabel::Text { text: String::from("shut that door") }; d.can = None; d.only = Some(OrderOnly::ShutDoor); d.valid = Some(OrderValid::ShutDoor); d.issue = OrderIssue::ShutDoor; d.tick = Some(OrderTick::ShutDoor); d.gone = None; if let Some(s) = d.stands_in.as_mut() { s.when = OrderWhen::ShutDoor; } }
            "work" => { d.label = OrderLabel::Work; d.can = Some(OrderCan::Work); d.only = Some(OrderOnly::Work); d.valid = Some(OrderValid::Work); d.issue = OrderIssue::Work; d.tick = Some(OrderTick::Work); d.gone = None; }
            "wallCharge" => { d.label = OrderLabel::WallRow; d.can = Some(OrderCan::WallRow); d.only = None; d.valid = Some(OrderValid::WallRow); d.issue = OrderIssue::WallRow; d.tick = None; d.gone = Some(OrderGone::WallRow); }
            "silent" => { d.label = OrderLabel::Text { text: String::from("go silent") }; d.can = None; d.only = Some(OrderOnly::SilentAll); d.valid = None; d.issue = OrderIssue::Silent; d.tick = None; d.gone = None; }
            "loud" => { d.label = OrderLabel::Text { text: String::from("go loud") }; d.can = None; d.only = Some(OrderOnly::SilentAll); d.valid = None; d.issue = OrderIssue::Loud; d.tick = None; d.gone = None; if let Some(s) = d.stands_in.as_mut() { s.when = OrderWhen::Loud; } }
            "wandLook" => { d.label = OrderLabel::Text { text: String::from("look under it · wand") }; d.can = None; d.only = Some(OrderOnly::Wand); d.valid = Some(OrderValid::Wand); d.issue = OrderIssue::Wand; d.tick = Some(OrderTick::Wand); d.gone = None; }
            "smokeThere" => { d.label = OrderLabel::Smoke; d.can = None; d.only = Some(OrderOnly::Smoke); d.valid = Some(OrderValid::Smoke); d.issue = OrderIssue::Smoke; d.tick = None; d.gone = None; }
            "fragThere" => { d.label = OrderLabel::ThrowRow { kind: F }; d.can = None; d.only = Some(OrderOnly::ThrowRow { kind: F }); d.valid = Some(OrderValid::ThrowRow { kind: F }); d.issue = OrderIssue::ThrowRow { kind: F }; d.tick = Some(OrderTick::ThrowRow); d.gone = None; }
            "stunThere" => { d.label = OrderLabel::ThrowRow { kind: S }; d.can = None; d.only = Some(OrderOnly::ThrowRow { kind: S }); d.valid = Some(OrderValid::ThrowRow { kind: S }); d.issue = OrderIssue::ThrowRow { kind: S }; d.tick = Some(OrderTick::ThrowRow); d.gone = None; }
            "pairMark" => { d.label = OrderLabel::Text { text: String::from("take him · on your mark") }; d.can = Some(OrderCan::PairTake); d.only = Some(OrderOnly::PairTake); d.valid = Some(OrderValid::PairTake); d.issue = OrderIssue::PairTake { mode: crate::pairorders::JobMode::Mark }; d.tick = Some(OrderTick::PairTake); d.gone = None; }
            "pairTake" => { d.label = OrderLabel::Text { text: String::from("take him now") }; d.can = Some(OrderCan::PairTake); d.only = Some(OrderOnly::PairTake); d.valid = Some(OrderValid::PairTake); d.issue = OrderIssue::PairTake { mode: crate::pairorders::JobMode::Now }; d.tick = Some(OrderTick::PairTake); d.gone = None; }
            "pairWatch" => { d.label = OrderLabel::Text { text: String::from("watch him") }; d.can = Some(OrderCan::PairWatch); d.only = None; d.valid = Some(OrderValid::PairWatch); d.issue = OrderIssue::PairWatch; d.tick = Some(OrderTick::PairWatch); d.gone = None; }
            "pairDark" => { d.label = OrderLabel::Text { text: String::from("move there · hold in the dark") }; d.can = Some(OrderCan::PairDark); d.only = None; d.valid = None; d.issue = OrderIssue::PairDark; d.tick = Some(OrderTick::PairDark); d.gone = None; if let Some(s) = d.stands_in.as_mut() { s.when = OrderWhen::PairDark; } }
            "pairGo" => { d.label = OrderLabel::PairGo; d.can = None; d.only = Some(OrderOnly::PairGo); d.valid = Some(OrderValid::PairGo); d.issue = OrderIssue::PairGo; d.tick = None; d.gone = None; }
            "go" => { if small { d.label = OrderLabel::DoorGoSmall; d.only = Some(OrderOnly::DoorGoSmall); d.valid = Some(OrderValid::DoorGoSmall); d.issue = OrderIssue::DoorGoSmall; } else { d.label = OrderLabel::DoorGoBig; d.only = Some(OrderOnly::DoorGoBig); d.valid = Some(OrderValid::DoorGoBig); d.issue = OrderIssue::DoorGoBig; } }
            other => { let Some(r) = rows::by_id(other) else { panic!("the order {} has no closures: it is neither an arm here nor a line of orders/rows", other) }; let data = d.clone(); *d = rows::def(r, data); d.gone = None; }
        }
    }
}
/// WHAT A LEVEL SCRIPT IS IN AN IMPORTED GAME. A script is code, hung on the Level by the level's name, and its define pass keeps its locals in a closure the bridge cannot read. So the level's name says which script it is,
/// and the define pass runs again as the Director's constructor runs it: the manifest with its locals, each program a run holds told its script, each run the locals. A beat in the middle of a yield is still refused.
pub fn deal_scripts(g: &mut crate::game::Game) {
    let Some(s) = g.m.scene_level_load.level_names.as_deref().and_then(crate::director::level_script_of) else { return };
    g.level.script = Some(s);
    if g.director.manifest.is_none() { return; }
    let m = crate::director::manifest_of(s).expect("bridge: the script's define pass");
    for r in g.tab.director_run.ids().collect::<Vec<_>>() { g.tab.director_run[r].vars = m.vars.clone(); for k in g.tab.director_run[r].def.beats.keys() { if let Some(b) = g.tab.director_run[r].def.beats.get_mut(k.as_str().into()) { b.fn_ = s; } } }
    g.director.manifest = Some(m);
}
/// the bytes of tools/blitz/export.ts or of export above, as a game. What the walk does not carry stands at its blank
pub fn import(b: &[u8]) -> crate::game::Game { let mut i = R::tallied(b); let mut g = crate::game::Game::r(&mut i);
    g.tab.boxes = Io::r(&mut i);
    g.tab.clip = Io::r(&mut i);
    g.tab.anim_gltf_mesh_data = Io::r(&mut i);
    g.tab.anim_gltf_cast_variant = Io::r(&mut i);
    g.tab.anim_gltf_cast_info = Io::r(&mut i);
    g.tab.rig = Io::r(&mut i);
    g.tab.pose_set = Io::r(&mut i);
    g.tab.ragdoll = Io::r(&mut i);
    g.ch = Io::r(&mut i);
    g.dr = Io::r(&mut i);
    g.tab.props_prop = Io::r(&mut i);
    g.tab.input = Io::r(&mut i);
    g.tab.camera = Io::r(&mut i);
    g.tab.item = Io::r(&mut i);
    g.gd = Io::r(&mut i);
    g.pl = Io::r(&mut i);
    g.tab.debris_chunk = Io::r(&mut i);
    g.tab.cmd_seat = Io::r(&mut i);
    g.tab.light = Io::r(&mut i);
    g.tab.sim_lightshome_fixture = Io::r(&mut i);
    g.tab.devices_camera_rt = Io::r(&mut i);
    g.tab.devices_laser_rt = Io::r(&mut i);
    g.tab.wallbreak_breakable = Io::r(&mut i);
    g.tab.furnblast_member = Io::r(&mut i);
    g.tab.wind_wind_source = Io::r(&mut i);
    g.tab.doortools_door_op = Io::r(&mut i);
    g.tab.orders_assignment = Io::r(&mut i);
    g.tab.doortools_room_search = Io::r(&mut i);
    g.tab.doortools_word = Io::r(&mut i);
    g.tab.doortools_door_gate = Io::r(&mut i);
    g.tab.doortools_known_threat = Io::r(&mut i);
    g.tab.doortools_set_charge = Io::r(&mut i);
    g.tab.orders_order_def = Io::r(&mut i);
    g.tab.roomfight_def = Io::r(&mut i);
    g.tab.roomfight_room = Io::r(&mut i);
    g.tab.cover_opening = Io::r(&mut i);
    g.tab.wallreact_breach = Io::r(&mut i);
    g.tab.tactics_role = Io::r(&mut i);
    g.tab.tactics_plan = Io::r(&mut i);
    g.tab.tactics_danger_rec = Io::r(&mut i);
    g.tab.lanes_lane = Io::r(&mut i);
    g.tab.pressure_man = Io::r(&mut i);
    g.tab.stickycam_sticky_cam = Io::r(&mut i);
    g.tab.rescueflow_region = Io::r(&mut i);
    g.tab.squad_room_clear = Io::r(&mut i);
    g.tab.director_run = Io::r(&mut i);
    g.tab.roomfight_op = Io::r(&mut i);
    g.tab.wallcharge_wall_op = Io::r(&mut i);
    g.tab.wallcharge_set_wall_charge = Io::r(&mut i);
    g.tab.route = Io::r(&mut i);
    g.tab.enemysquads_squad = Io::r(&mut i);
    g.tab.director_live_beat = Io::r(&mut i);
    g.tab.guards_standoff = Io::r(&mut i);
    g.cv = Io::r(&mut i);
    g.tab.game_talk = Io::r(&mut i);
    g.fa = Io::r(&mut i);
    g.serial_next = Io::r(&mut i); g.loose_poses = Io::r(&mut i); assert!(i.p == b.len(), "bridge: bytes left over"); i.refuse_past(&crate::id_tables::id_tables(&g)); refuse_mid_script(&g); deal_closures(&mut g); deal_scripts(&mut g); g.col.mirror.slot = g.sim as f64; g }
/// the same bytes over a game that already stands: what the walk does not carry (closures as enums, what a body sets up at new) stays as this game has it
pub fn import_into(g: &mut crate::game::Game, b: &[u8]) { let mut i = R::tallied(b); g.ri(&mut i);
    g.tab.boxes.ri(&mut i);
    g.tab.clip.ri(&mut i);
    g.tab.anim_gltf_mesh_data.ri(&mut i);
    g.tab.anim_gltf_cast_variant.ri(&mut i);
    g.tab.anim_gltf_cast_info.ri(&mut i);
    g.tab.rig.ri(&mut i);
    g.tab.pose_set.ri(&mut i);
    g.tab.ragdoll.ri(&mut i);
    g.ch.ri(&mut i);
    g.dr.ri(&mut i);
    g.tab.props_prop.ri(&mut i);
    g.tab.input.ri(&mut i);
    g.tab.camera.ri(&mut i);
    g.tab.item.ri(&mut i);
    g.gd.ri(&mut i);
    g.pl.ri(&mut i);
    g.tab.debris_chunk.ri(&mut i);
    g.tab.cmd_seat.ri(&mut i);
    g.tab.light.ri(&mut i);
    g.tab.sim_lightshome_fixture.ri(&mut i);
    g.tab.devices_camera_rt.ri(&mut i);
    g.tab.devices_laser_rt.ri(&mut i);
    g.tab.wallbreak_breakable.ri(&mut i);
    g.tab.furnblast_member.ri(&mut i);
    g.tab.wind_wind_source.ri(&mut i);
    g.tab.doortools_door_op.ri(&mut i);
    g.tab.orders_assignment.ri(&mut i);
    g.tab.doortools_room_search.ri(&mut i);
    g.tab.doortools_word.ri(&mut i);
    g.tab.doortools_door_gate.ri(&mut i);
    g.tab.doortools_known_threat.ri(&mut i);
    g.tab.doortools_set_charge.ri(&mut i);
    g.tab.orders_order_def.ri(&mut i);
    g.tab.roomfight_def.ri(&mut i);
    g.tab.roomfight_room.ri(&mut i);
    g.tab.cover_opening.ri(&mut i);
    g.tab.wallreact_breach.ri(&mut i);
    g.tab.tactics_role.ri(&mut i);
    g.tab.tactics_plan.ri(&mut i);
    g.tab.tactics_danger_rec.ri(&mut i);
    g.tab.lanes_lane.ri(&mut i);
    g.tab.pressure_man.ri(&mut i);
    g.tab.stickycam_sticky_cam.ri(&mut i);
    g.tab.rescueflow_region.ri(&mut i);
    g.tab.squad_room_clear.ri(&mut i);
    g.tab.director_run.ri(&mut i);
    g.tab.roomfight_op.ri(&mut i);
    g.tab.wallcharge_wall_op.ri(&mut i);
    g.tab.wallcharge_set_wall_charge.ri(&mut i);
    g.tab.route.ri(&mut i);
    g.tab.enemysquads_squad.ri(&mut i);
    g.tab.director_live_beat.ri(&mut i);
    g.tab.guards_standoff.ri(&mut i);
    g.cv.ri(&mut i);
    g.tab.game_talk.ri(&mut i);
    g.fa.ri(&mut i);
    g.serial_next = Io::r(&mut i); g.loose_poses = Io::r(&mut i); assert!(i.p == b.len(), "bridge: bytes left over"); i.refuse_past(&crate::id_tables::id_tables(g)); refuse_mid_script(g); deal_closures(g); deal_scripts(g); }
#[cfg(test)]
mod bridge_test {
    extern crate std;
    /// BRIDGE_BLOB=<a file of tools/blitz/export.ts> cargo test --lib bridge_round_trip
    #[test] fn bridge_round_trip() { let Ok(p) = std::env::var("BRIDGE_BLOB") else { return; }; let t = std::thread::Builder::new().stack_size(1 << 28).spawn(move || { let b = std::fs::read(&p).unwrap(); let g = crate::io::import(&b); let c = crate::io::export(&g); { let mut h = crate::io::import(&b); crate::io::import_into(&mut h, &b); assert!(crate::io::export(&h) == b, "import_into differs"); } if let Ok(out) = std::env::var("BRIDGE_OUT") { std::fs::write(out, &c).unwrap(); } assert!(c == b, "the bytes differ: {} against {}, first at {:?}", c.len(), b.len(), c.iter().zip(b.iter()).position(|(x, y)| x != y)); }).unwrap(); t.join().unwrap(); }
}
#[cfg(test)]
mod puppet_test { extern crate std; use crate::puppet::*; use crate::io::*;
    /// PUPPET_LIST=<blobs, comma> PUPPET_OUT=<dir> cargo test --lib --release puppet_round: the first at start, the rest as steps, to p0.bin p1.bin ...
    #[test] fn puppet_round() { let Ok(list) = std::env::var("PUPPET_LIST") else { return; }; let t = std::thread::Builder::new().stack_size(1 << 28).spawn(move || { let out = std::env::var("PUPPET_OUT").unwrap(); let mut m = PuppetMemo::default(); for (i, f) in list.split(',').enumerate() { let g = crate::io::import(&std::fs::read(f).unwrap()); let t0 = std::time::Instant::now(); let p = export_puppet(&g, &mut m, i == 0); let d = t0.elapsed(); std::println!("p{}: {} bytes in {:?}", i, p.len(), d); std::fs::write(std::format!("{}/p{}.bin", out, i), &p).unwrap(); } }).unwrap(); t.join().unwrap(); }
}
#[cfg(test)]
mod out_test { extern crate std; fn z<T: crate::io::Io>() -> T { static Z: [u8; 4096] = [0; 4096]; T::r(&mut crate::io::R::new(&Z)) } #[test] fn host_round() { let Ok(a) = std::env::var("HOST_BLOB") else { return; }; let h = std::env::var("HOST_HALF").unwrap(); let o = std::env::var("HOST_OUT").unwrap(); let t = std::thread::Builder::new().stack_size(1 << 28).spawn(move || { let mut g = crate::io::import(&std::fs::read(&a).unwrap()); let c = crate::io::take_host_half(&mut g, &std::fs::read(&h).unwrap()); std::println!("{} record(s)", c.len()); std::fs::write(&o, crate::io::export(&g)).unwrap(); crate::io::set_slot(&mut g, 1); assert!(g.sim == 1 && g.col.mirror.slot == 1.0); }).unwrap(); t.join().unwrap(); } #[test] fn script_round() { let Ok(a) = std::env::var("SCRIPT_BLOB") else { return; }; let t = std::thread::Builder::new().stack_size(1 << 28).spawn(move || { let b = std::fs::read(&a).unwrap(); let g = crate::io::import(&b); let m = g.director.manifest.as_ref(); std::println!("script {:?}; manifest {}; programs {}; vars {:?}; runs {}; bytes the same: {}", g.level.script, m.is_some(), m.map(|m| m.programs.len()).unwrap_or(0), m.map(|m| core::mem::discriminant(&m.vars)), g.tab.director_run.len(), crate::io::export(&g) == b); for r in g.tab.director_run.ids().collect::<std::vec::Vec<_>>() { let R = &g.tab.director_run[r]; std::println!("  run {}: {} beats, vars {:?}, fn {:?}", R.name, R.def.beats.len(), core::mem::discriminant(&R.vars), R.def.beats.keys().first().and_then(|k| R.def.beats.get(k.as_str().into()).map(|b| b.fn_))); } }).unwrap(); t.join().unwrap(); } #[test] fn out_round() { let Ok(a) = std::env::var("OUT_BLOB") else { return; }; let t = std::thread::Builder::new().stack_size(1 << 28).spawn(move || { let mut g = crate::io::import(&std::fs::read(&a).unwrap()); crate::savestate::checkpoints_keep(&mut g, "x"); crate::savestate::checkpoints_request(&mut g, "x", "x"); crate::render_decals::decals_add(&mut g, z(), z(), z(), z(), z()); crate::render_decals::decals_clear(&mut g); crate::render_decals::decals_touch(&mut g); crate::audio_iface::game_audio_like_footstep(&mut g, z(), z(), z(), z()); crate::audio_iface::game_audio_like_play(&mut g, "x", z(), z(), None); crate::audio_iface::game_audio_like_rain(&mut g, z()); crate::host_api::host_api_restart_picture(&mut g, "x"); crate::moths::moths_clear(&mut g); crate::out_events::push_particles_emit(&mut g, z(), z(), z(), z(), z(), z()); crate::render_particles::particles_impulse(&mut g, z(), z(), z(), None); crate::render_particles::particles_set_heat(&mut g, &[]); crate::render_reach_keep::reach_keep_changed(&mut g, z(), z()); crate::journal::session_recorder_cut(&mut g, "x", z()); crate::out_events::push_smoke_system_like_clear_all(&mut g); crate::out_events::push_smoke_system_like_emit(&mut g, &z()); crate::smoke_types::smoke_system_like_keep_hot(&mut g, z()); crate::smoke_types::smoke_system_like_obstacles_changed(&mut g, z(), z()); crate::smoke_types::smoke_system_like_push(&mut g, z()); crate::wallwear::wall_wear_reset(&mut g); crate::wallwear::wall_wear_round(&mut g, z(), z(), z(), z(), z(), z(), z()); crate::deathrecap::recap_round(&mut g, z(), z(), z(), z(), z(), &crate::weapons::WeaponDef::default(), z()); crate::floormess::mess_burst(&mut g, z(), z()); crate::floormess::mess_opened(&mut g, z()); crate::floormess::mess_wall(&mut g, z(), z(), z(), z()); crate::marks::bang_scorch(&mut g, z()); crate::marks::bullet_mark(&mut g, z(), z(), z(), &crate::weapons::WeaponDef::default()); crate::marks::chip_mark(&mut g, z(), z(), z(), z()); crate::marks::kick_scuff(&mut g, z(), z()); crate::snow_blast::clear_snow_blast(&mut g); crate::snow_marks::clear_snow_marks(&mut g); crate::snow_tracks::clear_snow_tracks(&mut g, &mut Default::default()); crate::out_events::push_tap(&mut g, z(), &[], &[]); crate::out_events::push_wall_feel(&mut g, z()); { use crate::audio_iface::{game_audio_like_play as pl, PlayOpts as P}; pl(&mut g, "thunder", None, Some(0.5), Some(&P::Thunder { lowpass: 900.0, rate: 0.8, pan: -0.25, wet: 0.5 })); pl(&mut g, "radio", Some([1.0, 2.0, 3.0]), None, Some(&P::Rate { rate: 1.05 })); pl(&mut g, "smokeHiss", Some([1.0, 2.0, 3.0]), Some(1.0), Some(&P::Duration { duration: 15.0 })); pl(&mut g, "chairRoll", Some([4.0, 0.05, 5.0]), Some(0.3), Some(&P::ChairRoll { tok: 7.0 })); } std::println!("{} events", g.out.len()); let mut m = crate::puppet::PuppetMemo::default(); let p = crate::puppet::export_puppet(&g, &mut m, true); std::fs::write(std::env::var("OUT_FILE").unwrap(), &p).unwrap(); }).unwrap(); t.join().unwrap(); } }
