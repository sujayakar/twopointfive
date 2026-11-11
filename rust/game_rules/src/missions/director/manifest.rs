pub use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the character budget a script's spawn is held to on a tutorial mission; anywhere else none.
pub(super) fn lifted_budget(g: &crate::game::Game) -> Option<f64> {
    if crate::rails::rails_live(g) { Some(16.0) } else { None }
}

/// Run a script's define pass against a recording manifest: everything it names and declares; beat bodies are kept, not run. Err = the pass threw.
pub(super) fn manifest_of(script: crate::scene_level::LevelScriptFn) -> Result<crate::director::Manifest, String> {
    let mut manifest = Manifest::default();
    seam_script_define(script, &mut manifest)?;
    Ok(manifest)
}

/// (levels/<name>.script.ts, `script(D)`: the define pass of the level this names; its verbs are the manifest_* and program_* below)
fn seam_script_define(script: crate::scene_level::LevelScriptFn, manifest: &mut Manifest) -> Result<(), String> { crate::director::script_define(script, manifest) }

/// D.mark: a named place; a mark named again keeps its pos unless given one.
pub(in crate::missions) fn manifest_mark(m: &mut crate::director::Manifest, id: &str, pos: Option<V3>, yaw: Option<f64>) -> crate::director::MarkRef {
    if !m.marks.has(id.into()) || pos.is_some() {
        let pos = match pos {
            Some(p) => Some(v3::copy(p)),
            None => m.marks.get(id.into()).and_then(|row| row.pos),
        };
        m.marks.set(id.into(), ManifestMarksValue { pos, yaw });
    }
    MarkRef { id: id.into() }
}
/// D.volume.
pub(in crate::missions) fn manifest_volume(m: &mut crate::director::Manifest, id: &str) -> crate::director::VolumeRef {
    m.volumes.add(id.into());
    VolumeRef { id: id.into() }
}
/// D.door.
pub(in crate::missions) fn manifest_door(m: &mut crate::director::Manifest, id: &str) -> crate::director::DoorRef {
    m.doors.add(id.into());
    DoorRef { id: id.into() }
}
/// D.loadout.
pub(in crate::missions) fn manifest_loadout(m: &mut crate::director::Manifest, id: &str, spec: crate::director::LoadoutSpec) -> crate::director::LoadoutRef {
    m.loadouts.set(id.into(), spec);
    LoadoutRef { id: id.into() }
}
/// D.program: Err = declared twice, or the body's Err; the row goes in the manifest after the body.
pub(in crate::missions) fn manifest_program(m: &mut crate::director::Manifest, name: &str, body: &mut dyn FnMut(&mut crate::director::ProgramDef) -> Result<(), String>) -> Result<(), String> {
    if m.programs.has(name.into()) { return Err(format!("script: program '{}' declared twice", name)); }
    let mut program = ProgramDef { name: name.into(), beats: JsMap::new(), triggers: Vec::new() };
    body(&mut program)?;
    m.programs.set(name.into(), program);
    Ok(())
}
/// P.onStart.
pub(in crate::missions) fn program_on_start(p: &mut crate::director::ProgramDef, beat: &str) {
    p.triggers.push(TriggerDef { kind: TriggerDefKind::Start, beat: beat.into(), ..TriggerDef::default() });
}
/// P.onSignal.
pub(in crate::missions) fn program_on_signal(p: &mut crate::director::ProgramDef, sig: &str, beat: &str) {
    p.triggers.push(TriggerDef { kind: TriggerDefKind::Signal, beat: beat.into(), name: Some(sig.into()), ..TriggerDef::default() });
}
/// P.beat: `script` is whose match holds this beat's code. Err = declared twice.
pub(in crate::missions) fn program_beat(p: &mut crate::director::ProgramDef, name: &str, script: &crate::scene_level::LevelScriptFn, atomic: Option<bool>) -> Result<(), String> {
    if p.beats.has(name.into()) { return Err(format!("script: beat '{}' declared twice in program '{}'", name, p.name)); }
    p.beats.set(name.into(), ProgramDefBeatsValue { fn_: *script, atomic: atomic.unwrap_or(true) });
    Ok(())
}

/// small pure helpers so a script needs no runtime import for arithmetic on places: the path callers write, sig.rs's `pub mod util`.
pub mod util {
    pub use crate::v3::{dist_xz, yaw_to};
}

fn err(out: &mut Vec<LintProblem>, what: String) { out.push(LintProblem { level: LintProblemLevel::Error, what }); }
fn warn(out: &mut Vec<LintProblem>, what: String) { out.push(LintProblem { level: LintProblemLevel::Warn, what }); }
/// `${x}` of a number that may be undefined
fn num_text(x: Option<f64>) -> String {
    match x {
        None => "undefined".into(),
        Some(x) => js::num_str(x),
    }
}
fn device_name(d: &crate::scene_level::DeviceDef) -> &str {
    match d {
        crate::scene_level::DeviceDef::Camera(c) => &c.name,
        crate::scene_level::DeviceDef::Keypad(c) => &c.name,
        crate::scene_level::DeviceDef::Laser(c) => &c.name,
    }
}
/// `('group' in d && d.group) || 'security'`
fn device_group(d: &crate::scene_level::DeviceDef) -> String {
    let group = match d {
        crate::scene_level::DeviceDef::Camera(c) => c.group.as_deref(),
        crate::scene_level::DeviceDef::Keypad(_) => None,
        crate::scene_level::DeviceDef::Laser(c) => c.group.as_deref(),
    };
    match group {
        Some(name) if !name.is_empty() => name.into(),
        _ => "security".into(),
    }
}
fn door_is_in(level: &LintManifestLevel, name: &str) -> bool { level.doors.iter().any(|d| d.name == name) }

/// The define-time lint: every name the script uses exists in the level, loadouts are spawnable, triggers name declared beats, waits are finite. Errors disarm the script.
pub(super) fn lint_manifest(g: &crate::game::Game, m: &crate::director::Manifest, level: &crate::director::LintManifestLevel) -> Vec<crate::director::LintProblem> {
    let route_names: Vec<String> = level.routes.iter().map(|r| g.tab.route[*r].name.clone()).collect();
    lint_with_routes(m, level, &route_names)
}
/// the lint itself, handed the names of the level's routes (a route is the one thing of a level that lives in `g`)
fn lint_with_routes(m: &Manifest, level: &LintManifestLevel, route_names: &[String]) -> Vec<LintProblem> {
    let mut out: Vec<LintProblem> = Vec::new();
    let devices: &[crate::scene_level::DeviceDef] = level.devices.as_deref().unwrap_or(&[]);
    let volumes: &[crate::scene_level::VolumeDef] = level.volumes.as_deref().unwrap_or(&[]);
    let clear_plans: &[crate::scene_level::RoomClearPlan] = level.clear_plans.as_deref().unwrap_or(&[]);
    lint_doors_and_devices(m, level, devices, &mut out);
    lint_routes_and_volumes(m, volumes, route_names, &mut out);
    lint_plans(m, level, clear_plans, &mut out);
    lint_marks_and_loadouts(m, &mut out);
    lint_light_groups(m, level, devices, &mut out);
    for (pn, p) in m.programs.iter() {
        lint_program(m, &pn, p, &mut out);
    }
    lint_expects(m, &mut out);
    out
}

/// Every door and device named is in the level; the error lists what is.
fn lint_doors_and_devices(m: &Manifest, level: &LintManifestLevel, devices: &[crate::scene_level::DeviceDef], out: &mut Vec<LintProblem>) {
    for id in m.doors.iter() {
        if !door_is_in(level, &id) {
            let names: Vec<&str> = level.doors.iter().map(|d| d.name.as_str()).collect();
            err(out, format!("door '{}' is not in the level (doors: {})", id, names.join(", ")));
        }
    }
    for id in m.devices.iter() {
        if !devices.iter().any(|d| device_name(d) == id) {
            let names: Vec<&str> = devices.iter().map(device_name).collect();
            let names = if names.is_empty() { String::from("none") } else { names.join(", ") };
            err(out, format!("device '{}' is not in the level (devices: {})", id, names));
        }
    }
}

/// Every route and volume named is in the level.
fn lint_routes_and_volumes(m: &Manifest, volumes: &[crate::scene_level::VolumeDef], route_names: &[String], out: &mut Vec<LintProblem>) {
    for id in m.routes.iter() {
        if !route_names.iter().any(|name| *name == id) { err(out, format!("route '{}' is not in the level", id)); }
    }
    for id in m.volumes.iter() {
        if !volumes.iter().any(|v| v.name == id) {
            err(out, format!("volume '{}' is not in the level{}", id, if volumes.is_empty() { " (it authors none)" } else { "" }));
        }
    }
}

/// The breach plan named is there and whole; a room-clear plan named is one of the level's.
fn lint_plans(m: &Manifest, level: &LintManifestLevel, clear_plans: &[crate::scene_level::RoomClearPlan], out: &mut Vec<LintProblem>) {
    for (id, k) in m.plans.iter() {
        if *k == PlanKind::Breach {
            match &level.breach {
                None => err(out, String::from("plan 'breach': the level has no breach plan (plans.breach)")),
                Some(b) => {
                    for dn in [&b.door, &b.next.door] {
                        if !door_is_in(level, dn) { err(out, format!("plan 'breach' names door '{}', not in the level", dn)); }
                    }
                    if b.hostiles.is_empty() { err(out, String::from("plan 'breach' has no hostiles")); }
                }
            }
        } else if !clear_plans.iter().any(|p| p.room.as_deref() == Some(id.as_str()) || p.door == id) {
            err(out, format!("plan '{}': no room-clear plan of that room / door in the level", id));
        }
    }
}

/// Every mark has a finite place, and every loadout a team that can be spawned and tints in range.
fn lint_marks_and_loadouts(m: &Manifest, out: &mut Vec<LintProblem>) {
    for (id, mark) in m.marks.iter() {
        match mark.pos {
            None => err(out, format!("mark '{}' has no position (LevelData has no marks[] yet — declare it with D.mark(id, [x, 0, z]))", id)),
            Some(pos) => {
                if !pos.iter().all(|x| x.is_finite()) { err(out, format!("mark '{}' position is not finite", id)); }
            }
        }
    }
    for (id, l) in m.loadouts.iter() {
        if !teams().iter().any(|team| team == l.team.as_str()) {
            err(out, format!("loadout '{}': team '{}' cannot be spawned (guard / operative / hostile)", id, l.team.as_str()));
        }
        for t in [l.tint, l.tint2] {
            if let Some(t) = t {
                if !t.iter().all(|x| x.is_finite() && *x >= 0.0 && *x <= 1.0) { err(out, format!("loadout '{}': tint out of range", id)); }
            }
        }
    }
}

/// Every light group named is carried by a light, a switch or a device of the level.
fn lint_light_groups(m: &Manifest, level: &LintManifestLevel, devices: &[crate::scene_level::DeviceDef], out: &mut Vec<LintProblem>) {
    let mut groups: JsSet<String> = JsSet::new();
    for l in level.lights.iter() { groups.add(l.group.clone()); }
    for s in level.switchables.iter() { groups.add(s.group.clone()); }
    for d in devices.iter() { groups.add(device_group(d)); }
    for id in m.groups.iter() {
        if !groups.has(id.clone()) {
            let names: Vec<String> = groups.iter().filter(|name| !name.is_empty()).collect();
            err(out, format!("light group '{}' is not in the level (groups: {})", id, names.join(", ")));
        }
    }
}

/// A program has beats and a way to start one; its triggers are sound, and its when() predicates few enough.
fn lint_program(m: &Manifest, pn: &str, p: &crate::director::ProgramDef, out: &mut Vec<LintProblem>) {
    if p.beats.is_empty() { warn(out, format!("program '{}' declares no beats", pn)); }
    if !p.triggers.iter().any(|t| t.kind == TriggerDefKind::Start) && p.triggers.is_empty() {
        warn(out, format!("program '{}' has no trigger — nothing will ever start a beat", pn));
    }
    for t in p.triggers.iter() {
        if !p.beats.has(t.beat.clone()) {
            let detail = match t.kind {
                TriggerDefKind::After => format!(" {}s", num_text(t.secs)),
                TriggerDefKind::Enter => format!(" '{}'", t.volume.as_deref().unwrap_or("undefined")),
                TriggerDefKind::Event => format!(" '{}'", t.event.map_or("undefined", |e| e.as_str())),
                TriggerDefKind::Signal => format!(" '{}'", t.name.as_deref().unwrap_or("undefined")),
                _ => String::new(),
            };
            err(out, format!("program '{}': trigger {}{} → beat '{}', which is not declared", pn, t.kind.as_str(), detail, t.beat));
        }
        if t.kind == TriggerDefKind::After && !t.secs.map_or(false, |s| s.is_finite() && s >= 0.0) {
            err(out, format!("program '{}': after({}) is not a finite delay", pn, num_text(t.secs)));
        }
        if t.kind == TriggerDefKind::Enter && !t.volume.as_ref().map_or(false, |v| m.volumes.has(v.clone())) {
            err(out, format!("program '{}': onEnter names volume '{}' the script never declared with D.volume", pn, t.volume.as_deref().unwrap_or("undefined")));
        }
    }
    if p.triggers.iter().filter(|t| t.kind == TriggerDefKind::When).count() as f64 > MAX_WHEN {
        err(out, format!("program '{}': more than {} when() predicates", pn, MAX_WHEN));
    }
}

/// What the script expects names a program it declares, and a time above zero.
fn lint_expects(m: &Manifest, out: &mut Vec<LintProblem>) {
    for e in m.expects.iter() {
        if let Some(program) = &e.program {
            if !m.programs.has(program.clone()) { err(out, format!("expect: program '{}' is not declared", program)); }
        }
        if e.kind == ManifestExpectsItemKind::Within && !e.max.map_or(false, |max| max > 0.0) {
            err(out, format!("expect.within: {} s", num_text(e.max)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // what no script of the six calls and the lint still answers for: how a manifest comes to name these
    fn d_route(manifest: &mut Manifest, id: &str) { manifest.routes.add(id.into()); }
    fn d_device(manifest: &mut Manifest, id: &str) { manifest.devices.add(id.into()); }
    fn d_plan(manifest: &mut Manifest, id: &str) { manifest.plans.set(id.into(), if id == "breach" { PlanKind::Breach } else { PlanKind::RoomClear }); }
    fn d_light_group(manifest: &mut Manifest, id: &str) { manifest.groups.add(id.into()); }
    fn p_trigger(program: &mut ProgramDef, kind: TriggerDefKind, beat: &str, secs: Option<f64>) {
        program.triggers.push(TriggerDef { kind, beat: beat.into(), secs, ..TriggerDef::default() });
    }
    fn errors(manifest: &Manifest, level: &LintManifestLevel, routes: &[String]) -> Vec<String> {
        lint_with_routes(manifest, level, routes).iter().filter(|p| p.level == LintProblemLevel::Error).map(|p| p.what.clone()).collect()
    }

    /// what a tutorial's script stands a man by: the way from one place to another, on the floor
    #[test]
    fn a_scripts_arithmetic_on_places_is_on_the_floor() {
        assert_eq!(util::yaw_to([0.0; 3], [0.0, 5.0, 2.0]), 0.0);
        assert!((util::yaw_to([0.0; 3], [2.0, 5.0, 0.0]) - m::PI / 2.0).abs() < 1e-12);
        assert_eq!(util::dist_xz([1.0, 0.0, 2.0], [4.0, 9.0, 6.0]), 5.0);
    }

    /// a mark named again with no position keeps the one it has, and the yaw it has
    #[test]
    fn a_mark_named_again_keeps_its_place() {
        let mut manifest = Manifest::default();
        manifest_mark(&mut manifest, "m1", Some([1.0, 0.0, 2.0]), Some(0.5));
        manifest_mark(&mut manifest, "m1", None, None);
        manifest_mark(&mut manifest, "m2", None, None);
        let row = manifest.marks.get("m1".into()).unwrap();
        assert_eq!((row.pos, row.yaw), (Some([1.0, 0.0, 2.0]), Some(0.5)));
        assert_eq!(manifest.marks.get("m2".into()).unwrap().pos, None);
        manifest_mark(&mut manifest, "m1", Some([3.0, 0.0, 4.0]), None);
        let row = manifest.marks.get("m1".into()).unwrap();
        assert_eq!((row.pos, row.yaw), (Some([3.0, 0.0, 4.0]), None));
    }

    /// a program's body records into its row, and the row lands under its name after the body
    #[test]
    fn a_program_is_recorded() {
        let mut manifest = Manifest::default();
        let script = crate::scene_level::LevelScriptFn::TutTeam;
        manifest_program(&mut manifest, "tutorial", &mut |p| { program_on_start(p, "go"); program_on_signal(p, "lesson:a", "go"); program_beat(p, "go", &script, None) }).unwrap();
        let row = manifest.programs.get("tutorial".into()).unwrap();
        assert_eq!(row.name, "tutorial");
        assert_eq!(row.triggers.len(), 2);
        assert_eq!(row.triggers[1].name.as_deref(), Some("lesson:a"));
        assert!(row.beats.get("go".into()).unwrap().atomic);
    }

    /// a name declared twice stops the define pass, in words that name it; a body that fails leaves no row
    #[test]
    fn a_name_declared_twice_is_the_err() {
        let mut manifest = Manifest::default();
        let script = crate::scene_level::LevelScriptFn::TutTeam;
        let got = manifest_program(&mut manifest, "p", &mut |p| { program_beat(p, "b", &script, None)?; program_beat(p, "b", &script, None) });
        assert_eq!(got, Err(String::from("script: beat 'b' declared twice in program 'p'")));
        assert!(!manifest.programs.has("p".into()));
        manifest_program(&mut manifest, "p", &mut |_| Ok(())).unwrap();
        let got = manifest_program(&mut manifest, "p", &mut |_| Ok(()));
        assert_eq!(got, Err(String::from("script: program 'p' declared twice")));
    }

    /// A FAILURE MEANS: a script that drives a door, a device, a route or a light its level has not runs and does nothing, where an error would disarm it and say which.
    #[test]
    fn a_name_the_level_lacks_is_an_error_that_says_which() {
        let mut manifest = Manifest::default();
        for id in ["a", "b"] { manifest_door(&mut manifest, id); }
        for id in ["kp", "zz"] { d_device(&mut manifest, id); }
        for id in ["yard", "r"] { d_route(&mut manifest, id); }
        manifest_volume(&mut manifest, "v");
        for id in ["ceil", "gone"] { d_light_group(&mut manifest, id); }
        let door = |name: &str| crate::scene_level::LevelDoorDef { name: name.into(), ..crate::scene_level::LevelDoorDef::default() };
        let mut level = LintManifestLevel::default();
        level.doors = vec![door("a"), door("c")];
        level.lights = vec![crate::scene_level::LightDef { group: "ceil".into(), ..crate::scene_level::LightDef::default() }];
        level.volumes = Some(Vec::new());
        level.devices = Some(vec![crate::scene_level::DeviceDef::Keypad(crate::devices::KeypadDef2 { name: "kp".into(), ..crate::devices::KeypadDef2::default() })]);
        assert_eq!(errors(&manifest, &level, &[String::from("yard")]), [
            "door 'b' is not in the level (doors: a, c)",
            "device 'zz' is not in the level (devices: kp)",
            "route 'r' is not in the level",
            "volume 'v' is not in the level (it authors none)",
            "light group 'gone' is not in the level (groups: ceil, security)",   // (a keypad stands in a group of its own)
        ]);
    }

    /// A FAILURE MEANS: a beat no trigger can reach, or a trigger that fires into nothing, is found only by playing the mission to that moment.
    #[test]
    fn every_trigger_must_reach_a_beat_and_an_empty_program_is_only_a_warning() {
        let mut manifest = Manifest::default();
        manifest.programs.set("empty".into(), ProgramDef { name: "empty".into(), ..ProgramDef::default() });
        let level = LintManifestLevel::default();
        let got = lint_with_routes(&manifest, &level, &[]);
        assert!(got.len() == 2 && got.iter().all(|p| p.level == LintProblemLevel::Warn), "{} rows", got.len());

        let mut manifest = Manifest::default();
        let mut p = ProgramDef { name: "p".into(), ..ProgramDef::default() };
        program_beat(&mut p, "x", &crate::scene_level::LevelScriptFn::TutTeam, None).unwrap();
        program_on_start(&mut p, "x");
        p_trigger(&mut p, TriggerDefKind::After, "x", Some(-1.0));
        p_trigger(&mut p, TriggerDefKind::After, "nobeat", Some(1.5));
        manifest.programs.set("p".into(), p);
        manifest.expects.push(ManifestExpectsItem { kind: ManifestExpectsItemKind::Within, program: Some("ghost".into()), max: Some(0.0), ..ManifestExpectsItem::default() });
        assert_eq!(errors(&manifest, &level, &[]), [
            "program 'p': after(-1) is not a finite delay",
            "program 'p': trigger after 1.5s → beat 'nobeat', which is not declared",
            "expect: program 'ghost' is not declared",
            "expect.within: 0 s",
        ]);

        let mut manifest = Manifest::default();
        let mut p = ProgramDef { name: "p".into(), ..ProgramDef::default() };
        program_beat(&mut p, "x", &crate::scene_level::LevelScriptFn::TutTeam, None).unwrap();
        for _ in 0..32 { p_trigger(&mut p, TriggerDefKind::When, "x", None); }
        manifest.programs.set("p".into(), p.clone());
        assert!(errors(&manifest, &level, &[]).is_empty(), "32 predicates are allowed");
        p_trigger(&mut p, TriggerDefKind::When, "x", None);
        manifest.programs.set("p".into(), p);
        assert_eq!(errors(&manifest, &level, &[]), ["program 'p': more than 32 when() predicates"]);
    }

    /// A FAILURE MEANS: a script stages a plan the level has not, or stands a man nowhere, and the mission opens broken.
    #[test]
    fn a_plan_a_mark_and_a_loadout_must_be_able_to_stand() {
        let mut manifest = Manifest::default();
        for id in ["breach", "room1", "nope"] { d_plan(&mut manifest, id); }
        manifest_mark(&mut manifest, "m1", Some([1.0, 0.0, 2.0]), None);
        manifest_mark(&mut manifest, "m2", None, None);
        manifest_mark(&mut manifest, "m3", Some([f64::NAN, 0.0, 0.0]), None);
        manifest_loadout(&mut manifest, "l1", LoadoutSpec { team: crate::teams::Team::Civilian, tint: Some([2.0, 0.0, 0.0]), ..LoadoutSpec::default() });
        manifest_loadout(&mut manifest, "l2", LoadoutSpec { team: crate::teams::Team::Guard, tint: Some([1.0, 0.0, 0.0]), ..LoadoutSpec::default() });
        let mut level = LintManifestLevel::default();
        level.doors = vec![crate::scene_level::LevelDoorDef { name: "a".into(), ..crate::scene_level::LevelDoorDef::default() }];
        level.clear_plans = Some(vec![crate::scene_level::RoomClearPlan { door: "room1".into(), ..crate::scene_level::RoomClearPlan::default() }]);
        let mut breach = crate::scene_level::BreachPlan::default();
        breach.door = "a".into();
        breach.next.door = "z".into();
        level.breach = Some(breach);
        assert_eq!(errors(&manifest, &level, &[]), [
            "plan 'breach' names door 'z', not in the level",
            "plan 'breach' has no hostiles",
            "plan 'nope': no room-clear plan of that room / door in the level",
            "mark 'm2' has no position (LevelData has no marks[] yet — declare it with D.mark(id, [x, 0, z]))",
            "mark 'm3' position is not finite",
            "loadout 'l1': team 'civilian' cannot be spawned (guard / operative / hostile)",
            "loadout 'l1': tint out of range",
        ]);
    }

    /// a level's route is known by the name its row holds
    #[test]
    fn a_route_is_found_by_its_row_name() {
        let mut g = crate::game::Game::blank();
        let r = g.tab.route.push(crate::scene_level::PatrolRoute { name: "yard".into(), ..crate::scene_level::PatrolRoute::default() });
        let mut manifest = Manifest::default();
        d_route(&mut manifest, "yard");
        d_route(&mut manifest, "gone");
        let mut level = LintManifestLevel::default();
        level.routes = vec![r];
        let got = lint_manifest(&mut g, &manifest, &level);
        let got: Vec<&str> = got.iter().map(|p| p.what.as_str()).collect();
        assert_eq!(got, ["route 'gone' is not in the level"]);
    }
}
