//! src/game/director.ts lines 1 to 424
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the character budget a script's spawn is held to on a tutorial mission; anywhere else none.   (director.ts liftedBudget)
pub fn lifted_budget(g: &mut crate::game::Game) -> Option<f64> {
    if crate::rails::rails_live(g) { Some(16.0) } else { None }
}

/// Run a script's define pass against a recording api: everything it names and declares; beat bodies are kept, not run.   (director.ts manifestOf)
pub fn manifest_of(script: &crate::scene_level::LevelScriptFn) -> crate::director::Manifest {
    let mut manifest = Manifest::zeroed();
    seam_script_define(script, &mut manifest);
    manifest
}

// ---- the recording api of manifestOf: DirectorApi and ProgramApi are gone as objects, D.<verb> is d_<verb>(manifest, ...) and P.<verb> is p_<verb>(program, ...)

/// (levels/<name>.script.ts, `script(D)`: the define pass of the level this names)
fn seam_script_define(script: &crate::scene_level::LevelScriptFn, manifest: &mut Manifest) { todo!("needs: the by-hand script part's define pass for each LevelScriptFn") }
/// (director.ts:304, 313 throw: caught by the Director's constructor at :442, which then runs with no manifest)
fn seam_script_throw(what: String) -> ! { todo!("needs: manifest_of to return an error the constructor can read: {}", what) }

fn d_mark(manifest: &mut Manifest, id: &str, pos: Option<V3>, yaw: Option<f64>) -> MarkRef {
    if !manifest.marks.has(id.into()) || pos.is_some() {
        let pos = match pos {
            Some(p) => Some(v3::copy(p)),
            None => manifest.marks.get(id.into()).and_then(|row| row.pos),
        };
        manifest.marks.set(id.into(), ManifestMarksValue { pos, yaw });
    }
    MarkRef { kind: MarkRefKind::Mark, id: id.into() }
}
fn d_volume(manifest: &mut Manifest, id: &str) -> VolumeRef {
    manifest.volumes.add(id.into());
    VolumeRef { kind: VolumeRefKind::Volume, id: id.into() }
}
fn d_route(manifest: &mut Manifest, id: &str) -> RouteRef {
    manifest.routes.add(id.into());
    RouteRef { kind: RouteRefKind::Route, id: id.into() }
}
fn d_door(manifest: &mut Manifest, id: &str) -> DoorRef {
    manifest.doors.add(id.into());
    DoorRef { kind: DoorRefKind::Door, id: id.into() }
}
fn d_device(manifest: &mut Manifest, id: &str) -> DeviceRef {
    manifest.devices.add(id.into());
    DeviceRef { kind: DeviceRefKind::Device, id: id.into() }
}
fn d_plan(manifest: &mut Manifest, id: &str) -> PlanRef {
    let plan = if id == "breach" { PlanKind::Breach } else { PlanKind::RoomClear };
    manifest.plans.set(id.into(), plan);
    PlanRef { kind: PlanRefKind::Plan, id: id.into(), plan }
}
fn d_loadout(manifest: &mut Manifest, id: &str, spec: LoadoutSpec) -> LoadoutRef {
    manifest.loadouts.set(id.into(), spec);
    LoadoutRef { kind: LoadoutRefKind::Loadout, id: id.into() }
}
fn d_light_group(manifest: &mut Manifest, id: &str) -> GroupRef {
    manifest.groups.add(id.into());
    GroupRef { kind: GroupRefKind::Group, id: id.into() }
}
fn d_variant(manifest: &mut Manifest, id: VariantRefId) -> VariantRef {
    manifest.variants.add(id.as_str().into());
    VariantRef { kind: VariantRefKind::Variant, id }
}
fn d_program(manifest: &mut Manifest, name: &str, body: &mut dyn FnMut(&mut ProgramDef), opts: Option<ProgramOpts>) {
    if manifest.programs.has(name.into()) { seam_script_throw(format!("script: program '{}' declared twice", name)); }
    let mut program = ProgramDef { name: name.into(), opts: opts.unwrap_or_else(ProgramOpts::zeroed), beats: JsMap::new(), triggers: Vec::new() };
    body(&mut program);
    manifest.programs.set(name.into(), program);
}
fn d_expect_order(manifest: &mut Manifest, program: &str, beats: &[String]) {
    manifest.expects.push(ManifestExpectsItem { kind: ManifestExpectsItemKind::Order, program: Some(program.into()), beats: Some(beats.to_vec()), ..ManifestExpectsItem::zeroed() });
}
fn d_expect_within(manifest: &mut Manifest, program: &str, a: &str, b: &str, max: f64) {
    manifest.expects.push(ManifestExpectsItem { kind: ManifestExpectsItemKind::Within, program: Some(program.into()), a: Some(a.into()), b: Some(b.into()), max: Some(max), ..ManifestExpectsItem::zeroed() });
}
fn d_expect_never(manifest: &mut Manifest, what: &str) {
    manifest.expects.push(ManifestExpectsItem { kind: ManifestExpectsItemKind::Never, what: Some(what.into()), ..ManifestExpectsItem::zeroed() });
}
fn d_systems_qrf(manifest: &mut Manifest, on: bool) { manifest.systems.set("qrf", on); }
fn d_systems_needs(manifest: &mut Manifest, on: bool) { manifest.systems.set("needs", on); }
fn d_systems_civilians(manifest: &mut Manifest, on: bool) { manifest.systems.set("civilians", on); }
fn d_systems_autosave(manifest: &mut Manifest, on: bool) { manifest.systems.set("autosave", on); }
fn d_systems_their_frags(manifest: &mut Manifest, on: bool) { manifest.systems.set("theirFrags", on); }

fn p_on_start(program: &mut ProgramDef, beat: &str) {
    program.triggers.push(TriggerDef { kind: TriggerDefKind::Start, beat: beat.into(), ..TriggerDef::zeroed() });
}
fn p_after(program: &mut ProgramDef, secs: f64, beat: &str) {
    program.triggers.push(TriggerDef { kind: TriggerDefKind::After, beat: beat.into(), secs: Some(secs), ..TriggerDef::zeroed() });
}
fn p_on_event(program: &mut ProgramDef, event: EventKind, beat: &str) {
    program.triggers.push(TriggerDef { kind: TriggerDefKind::Event, beat: beat.into(), event: Some(event), ..TriggerDef::zeroed() });
}
fn p_on_signal(program: &mut ProgramDef, sig: &str, beat: &str) {
    program.triggers.push(TriggerDef { kind: TriggerDefKind::Signal, beat: beat.into(), name: Some(sig.into()), ..TriggerDef::zeroed() });
}
fn p_on_enter(program: &mut ProgramDef, v: &VolumeRef, beat: &str, who: Option<TriggerDefWho>, once: Option<bool>) {
    program.triggers.push(TriggerDef { kind: TriggerDefKind::Enter, beat: beat.into(), volume: Some(v.id.clone()), who: Some(who.unwrap_or(TriggerDefWho::Sam)), once: Some(once.unwrap_or(true)), ..TriggerDef::zeroed() });
}
fn p_when(program: &mut ProgramDef, beat: &str, label: Option<&str>) {
    program.triggers.push(TriggerDef { kind: TriggerDefKind::When, beat: beat.into(), label: Some(label.unwrap_or("when").into()), ..TriggerDef::zeroed() });
}
fn p_beat(program: &mut ProgramDef, bn: &str, fn_: crate::FnField, atomic: Option<bool>) {
    if program.beats.has(bn.into()) { seam_script_throw(format!("script: beat '{}' declared twice in program '{}'", bn, program.name)); }
    program.beats.set(bn.into(), ProgramDefBeatsValue { fn_, atomic: atomic.unwrap_or(true) });
}

/// small pure helpers so a script needs no runtime import for arithmetic on places.   (director.ts UTIL)
pub mod util {
    use super::*;
    /// (director.ts UTIL.yawTo)
    pub fn yaw_to(a: V3, b: V3) -> f64 { m::atan2(b[0] - a[0], b[2] - a[2]) }
    /// (director.ts UTIL.distXZ)
    pub fn dist_xz(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }
    /// (director.ts UTIL.lerp)
    pub fn lerp(a: V3, b: V3, t: f64) -> [f64; 3] {
        [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
    }
    /// (director.ts UTIL.add)
    pub fn add(a: V3, b: V3) -> [f64; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
    /// (director.ts UTIL.ahead)
    pub fn ahead(p: V3, yaw: f64, d: f64, y: Option<f64>) -> [f64; 3] {
        let y = y.unwrap_or(p[1]);
        [p[0] + m::sin(yaw) * d, y, p[2] + m::cos(yaw) * d]
    }
}

fn err(out: &mut Vec<LintProblem>, what: String) { out.push(LintProblem { level: LintProblemLevel::Error, what }); }
fn warn(out: &mut Vec<LintProblem>, what: String) { out.push(LintProblem { level: LintProblemLevel::Warn, what }); }
/// `${x}` of a number that may be undefined. (A number past 1e21 or under 1e-6 prints without JavaScript's exponent: this text is only ever read by a person.)
fn num_text(x: Option<f64>) -> String {
    match x {
        None => "undefined".into(),
        Some(x) if x.is_nan() => "NaN".into(),
        Some(x) if x == f64::INFINITY => "Infinity".into(),
        Some(x) if x == f64::NEG_INFINITY => "-Infinity".into(),
        Some(x) if x == 0.0 => "0".into(),
        Some(x) => format!("{}", x),
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

/// The define-time lint: every name the script uses exists in the level, loadouts are spawnable, triggers name declared beats, waits are finite. Errors disarm the script.   (director.ts lintManifest)
pub fn lint_manifest(g: &mut crate::game::Game, m: &crate::director::Manifest, level: &crate::director::LintManifestLevel) -> Vec<crate::director::LintProblem> {
    let route_names: Vec<String> = level.routes.iter().map(|r| g.tab.route[*r].name.clone()).collect();
    lint_with_routes(m, level, &route_names)
}
/// the lint itself, handed the names of the level's routes (a route is the one thing of a level that lives in `g`)
fn lint_with_routes(m: &Manifest, level: &LintManifestLevel, route_names: &[String]) -> Vec<LintProblem> {
    let mut out: Vec<LintProblem> = Vec::new();
    let no_devices = Vec::new();
    let devices: &Vec<crate::scene_level::DeviceDef> = level.devices.as_ref().unwrap_or(&no_devices);
    let no_volumes = Vec::new();
    let volumes: &Vec<crate::scene_level::VolumeDef> = level.volumes.as_ref().unwrap_or(&no_volumes);
    let no_plans = Vec::new();
    let clear_plans: &Vec<crate::scene_level::RoomClearPlan> = level.clear_plans.as_ref().unwrap_or(&no_plans);

    for id in m.doors.iter() {
        if !door_is_in(level, &id) {
            let names: Vec<&str> = level.doors.iter().map(|d| d.name.as_str()).collect();
            err(&mut out, format!("door '{}' is not in the level (doors: {})", id, names.join(", ")));
        }
    }
    for id in m.devices.iter() {
        if !devices.iter().any(|d| device_name(d) == id) {
            let names: Vec<&str> = devices.iter().map(device_name).collect();
            let names = if names.is_empty() { String::from("none") } else { names.join(", ") };
            err(&mut out, format!("device '{}' is not in the level (devices: {})", id, names));
        }
    }
    for id in m.routes.iter() {
        if !route_names.iter().any(|name| *name == id) { err(&mut out, format!("route '{}' is not in the level", id)); }
    }
    for id in m.volumes.iter() {
        if !volumes.iter().any(|v| v.name == id) {
            err(&mut out, format!("volume '{}' is not in the level{}", id, if volumes.is_empty() { " (it authors none)" } else { "" }));
        }
    }
    for (id, k) in m.plans.iter() {
        if *k == PlanKind::Breach {
            match &level.breach {
                None => err(&mut out, String::from("plan 'breach': the level has no breach plan (plans.breach)")),
                Some(b) => {
                    for dn in [&b.door, &b.next.door] {
                        if !door_is_in(level, dn) { err(&mut out, format!("plan 'breach' names door '{}', not in the level", dn)); }
                    }
                    if b.hostiles.is_empty() { err(&mut out, String::from("plan 'breach' has no hostiles")); }
                }
            }
        } else if !clear_plans.iter().any(|p| p.room.as_deref() == Some(id.as_str()) || p.door == id) {
            err(&mut out, format!("plan '{}': no room-clear plan of that room / door in the level", id));
        }
    }
    for (id, mark) in m.marks.iter() {
        match mark.pos {
            None => err(&mut out, format!("mark '{}' has no position (LevelData has no marks[] yet — declare it with D.mark(id, [x, 0, z]))", id)),
            Some(pos) => {
                if !pos.iter().all(|x| x.is_finite()) { err(&mut out, format!("mark '{}' position is not finite", id)); }
            }
        }
    }
    for (id, l) in m.loadouts.iter() {
        if !matches!(l.team, crate::teams::Team::Guard | crate::teams::Team::Operative | crate::teams::Team::Hostile) {
            err(&mut out, format!("loadout '{}': team '{}' cannot be spawned (guard / operative / hostile)", id, l.team.as_str()));
        }
        for t in [l.tint, l.tint2] {
            if let Some(t) = t {
                if !t.iter().all(|x| x.is_finite() && *x >= 0.0 && *x <= 1.0) { err(&mut out, format!("loadout '{}': tint out of range", id)); }
            }
        }
        if l.god == Some(true) {
            if let Some(play) = m.programs.values().find(|p| p.opts.showcase != Some(true)) {
                err(&mut out, format!("loadout '{}': god is a showcase cast's net — not in a script that has a play program ('{}')", id, play.name));
            }
        }
    }
    let mut groups: JsSet<String> = JsSet::new();
    for l in level.lights.iter() { groups.add(l.group.clone()); }
    for s in level.switchables.iter() { groups.add(s.group.clone()); }
    for d in devices.iter() { groups.add(device_group(d)); }
    for id in m.groups.iter() {
        if !groups.has(id.clone()) {
            let names: Vec<String> = groups.iter().filter(|name| !name.is_empty()).collect();
            err(&mut out, format!("light group '{}' is not in the level (groups: {})", id, names.join(", ")));
        }
    }
    for id in m.variants.iter() {
        if id != "daylight" { err(&mut out, format!("variant '{}' unknown", id)); }
        else if level.daylight.is_none() { err(&mut out, String::from("variant 'daylight': the level has no daylight block")); }
    }
    for (pn, p) in m.programs.iter() {
        if p.beats.is_empty() { warn(&mut out, format!("program '{}' declares no beats", pn)); }
        if !p.triggers.iter().any(|t| t.kind == TriggerDefKind::Start) && p.triggers.is_empty() {
            warn(&mut out, format!("program '{}' has no trigger — nothing will ever start a beat", pn));
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
                err(&mut out, format!("program '{}': trigger {}{} → beat '{}', which is not declared", pn, t.kind.as_str(), detail, t.beat));
            }
            if t.kind == TriggerDefKind::After && !t.secs.map_or(false, |s| s.is_finite() && s >= 0.0) {
                err(&mut out, format!("program '{}': after({}) is not a finite delay", pn, num_text(t.secs)));
            }
            if t.kind == TriggerDefKind::Enter && !t.volume.as_ref().map_or(false, |v| m.volumes.has(v.clone())) {
                err(&mut out, format!("program '{}': onEnter names volume '{}' the script never declared with D.volume", pn, t.volume.as_deref().unwrap_or("undefined")));
            }
        }
        if p.triggers.iter().filter(|t| t.kind == TriggerDefKind::When).count() as f64 > MAX_WHEN {
            err(&mut out, format!("program '{}': more than {} when() predicates", pn, MAX_WHEN));
        }
        if p.opts.end_veil.map_or(false, |v| !(v > 0.0)) || p.opts.end_hold.map_or(false, |v| !(v >= 0.0)) {
            err(&mut out, format!("program '{}': endVeil / endHold must be positive", pn));
        }
    }
    for e in m.expects.iter() {
        if let Some(program) = &e.program {
            if !m.programs.has(program.clone()) { err(&mut out, format!("expect: program '{}' is not declared", program)); }
        }
        if e.kind == ManifestExpectsItemKind::Within && !e.max.map_or(false, |max| max > 0.0) {
            err(&mut out, format!("expect.within: {} s", num_text(e.max)));
        }
    }
    out
}

/// (director.ts:384 String(B.fn): a beat's source text)
fn seam_beat_source(fn_: &crate::FnField) -> String { todo!("needs: a beat's source text, or the scan moved to where beats are written (FnField holds nothing)") }

/// `/\/\*[\s\S]*?\*\//g` then `/\/\/[^\n]*/g`: a mention in prose is not a use
fn strip_comments(src: &str) -> String {
    let mut no_blocks = String::new();
    let mut rest = src;
    while let Some(open) = rest.find("/*") {
        let Some(len) = rest[open + 2..].find("*/") else { break };
        no_blocks.push_str(&rest[..open]);
        rest = &rest[open + 2 + len + 2..];
    }
    no_blocks.push_str(rest);
    let mut out = String::new();
    let mut rest = no_blocks.as_str();
    while let Some(open) = rest.find("//") {
        out.push_str(&rest[..open]);
        rest = &rest[open..];
        rest = match rest.find('\n') { Some(end) => &rest[end..], None => "" };
    }
    out.push_str(rest);
    out
}
/// a regular expression's `\s`
fn is_js_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}
/// `/\.\s*view\b/` for 'view' (a property, read bare), else `\.\s*<verb>\s*[.(]`
fn uses_verb(src: &str, verb: &str) -> bool {
    src.match_indices('.').any(|(dot, _)| {
        let Some(after) = src[dot + 1..].trim_start_matches(is_js_space).strip_prefix(verb) else { return false };
        if verb == "view" { !after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') }
        else { after.trim_start_matches(is_js_space).starts_with(['.', '(']) }
    })
}

/// For every program not declared showcase, every beat's source searched for a showcase-only verb.   (director.ts scanShowcaseVerbs)
pub fn scan_showcase_verbs(m: &crate::director::Manifest) -> Vec<crate::director::ScanShowcaseVerbsItem> {
    let mut hits: Vec<ScanShowcaseVerbsItem> = Vec::new();
    let verbs = showcase_verbs();
    for (pn, p) in m.programs.iter() {
        if p.opts.showcase == Some(true) { continue; }
        for (bn, b) in p.beats.iter() {
            let src = strip_comments(&seam_beat_source(&b.fn_));
            for verb in verbs.iter() {
                if uses_verb(&src, verb) { hits.push(ScanShowcaseVerbsItem { program: pn.clone(), beat: bn.clone(), verb: verb.clone() }); }
            }
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    const A: V3 = [1.25, 0.7, -3.3];
    const B: V3 = [-4.1, 2.2, 9.75];
    fn bits(v: [f64; 3]) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }

    /// the TypeScript's own bits: UTIL through manifestOf's D.util, under bun with EXACT.on
    #[test]
    fn util_matches_the_typescript_bits() {
        assert_eq!(util::yaw_to(A, B).to_bits(), 0xbfd8e66e7eeccecb);
        assert_eq!(util::dist_xz(A, B).to_bits(), 0x402c3549a2de16a3);
        assert_eq!(bits(util::lerp(A, B, 0.37)), [0xbfe75810624dd2f0, 0x3ff4147ae147ae14, 0x3ff874bc6a7ef9dc]);
        assert_eq!(bits(util::add(A, B)), [0xc006cccccccccccc, 0x4007333333333334, 0x4019cccccccccccd]);
        assert_eq!(bits(util::ahead(A, 0.83, 2.6, None)), [0x400959564354aeaf, 0x3fe6666666666666, 0xbff8b9a4a290a3ca]);
        assert_eq!(bits(util::ahead(A, 0.83, 2.6, Some(5.5))), [0x400959564354aeaf, 0x4016000000000000, 0xbff8b9a4a290a3ca]);
    }

    /// each source and what the TypeScript's own regular expressions said of it, under bun
    #[test]
    fn the_scan_matches_the_typescript_regular_expressions() {
        let table: &[(&str, &str, &[&str])] = &[
            ("c.stage(x)", "c.stage(x)", &["stage"]),
            ("c . puppet (x)", "c . puppet (x)", &["puppet"]),
            ("/* c.stage( */ q", " q", &[]),
            ("// c.fade(\n", "\n", &[]),
            ("c.view.hover", "c.view.hover", &["view"]),
            ("const V = c.view;", "const V = c.view;", &["view"]),
            ("c.viewer", "c.viewer", &[]),
            ("c.camera.pin(s)", "c.camera.pin(s)", &["camera"]),
            ("c.cameraX(s)", "c.cameraX(s)", &[]),
            ("c.fade", "c.fade", &[]),
            ("x.select\u{a0}(", "x.select\u{a0}(", &["select"]),
            ("/* a */ c.press(k) // c.click(\n c.mission(on)", " c.press(k) \n c.mission(on)", &["press", "mission"]),
            ("c.view_", "c.view_", &[]),
            ("c.view", "c.view", &["view"]),
            ("/* open c.shift(", "/* open c.shift(", &["shift"]),
            ("c.chaos\n.x", "c.chaos\n.x", &["chaos"]),
            ("c.\n\tweather(o)", "c.\n\tweather(o)", &["weather"]),
        ];
        for (src, stripped, verbs) in table {
            let got = strip_comments(src);
            assert_eq!(got, *stripped);
            let table = showcase_verbs();
            let hits: Vec<&str> = table.iter().map(|verb| verb.as_str()).filter(|verb| uses_verb(&got, verb)).collect();
            assert_eq!(hits, *verbs, "{}", src);
        }
    }

    /// a mark named again with no position keeps the one it has, and the yaw it has
    #[test]
    fn a_mark_named_again_keeps_its_place() {
        let mut manifest = Manifest::zeroed();
        d_mark(&mut manifest, "m1", Some([1.0, 0.0, 2.0]), Some(0.5));
        d_mark(&mut manifest, "m1", None, None);
        d_mark(&mut manifest, "m2", None, None);
        let row = manifest.marks.get("m1".into()).unwrap();
        assert_eq!((row.pos, row.yaw), (Some([1.0, 0.0, 2.0]), Some(0.5)));
        assert_eq!(manifest.marks.get("m2".into()).unwrap().pos, None);
        d_mark(&mut manifest, "m1", Some([3.0, 0.0, 4.0]), None);
        let row = manifest.marks.get("m1".into()).unwrap();
        assert_eq!((row.pos, row.yaw), (Some([3.0, 0.0, 4.0]), None));
    }

    /// a program's body records into its row, the row lands under its name; D.systems sets by the TypeScript's own word, absent stays absent
    #[test]
    fn a_program_and_the_systems_are_recorded() {
        let mut manifest = Manifest::zeroed();
        d_program(&mut manifest, "tutorial", &mut |p| { p_on_start(p, "go"); p_beat(p, "go", crate::FnField, None); }, None);
        let row = manifest.programs.get("tutorial".into()).unwrap();
        assert_eq!(row.name, "tutorial");
        assert_eq!(row.triggers.len(), 1);
        assert!(row.beats.get("go".into()).unwrap().atomic);
        d_systems_their_frags(&mut manifest, false);
        assert_eq!(manifest.systems.get("theirFrags"), Some(&false));
        assert_eq!(manifest.systems.get("qrf"), None);
    }

    /// the same script and level given to the TypeScript's lintManifest under bun: the same rows in the same order
    #[test]
    fn the_lint_says_what_the_typescript_says() {
        let mut manifest = Manifest::zeroed();
        for id in ["a", "b"] { d_door(&mut manifest, id); }
        for id in ["kp", "zz"] { d_device(&mut manifest, id); }
        d_route(&mut manifest, "r");
        d_volume(&mut manifest, "v");
        for id in ["breach", "room1", "nope"] { d_plan(&mut manifest, id); }
        d_mark(&mut manifest, "m1", Some([1.0, 0.0, 2.0]), Some(0.5));
        d_mark(&mut manifest, "m1", None, None);
        d_mark(&mut manifest, "m2", None, None);
        d_mark(&mut manifest, "m3", Some([f64::NAN, 0.0, 0.0]), None);
        d_loadout(&mut manifest, "l1", LoadoutSpec { team: crate::teams::Team::Player, tint: Some([2.0, 0.0, 0.0]), tint2: Some([0.0, 0.0, 0.0]), god: Some(true), ..LoadoutSpec::zeroed() });
        d_loadout(&mut manifest, "l2", LoadoutSpec { team: crate::teams::Team::Guard, ..LoadoutSpec::zeroed() });
        for id in ["ceil", "security", "gone"] { d_light_group(&mut manifest, id); }
        d_variant(&mut manifest, VariantRefId::Daylight);
        manifest.programs.set("empty".into(), ProgramDef { name: "empty".into(), ..ProgramDef::zeroed() });
        let mut p = ProgramDef { name: "p".into(), opts: ProgramOpts { end_veil: Some(0.0), end_hold: Some(-1.0), ..ProgramOpts::zeroed() }, ..ProgramDef::zeroed() };
        p_on_start(&mut p, "x");
        p_after(&mut p, -1.0, "x");
        p_after(&mut p, 1.5, "nobeat");
        p_on_event(&mut p, EventKind::Shot, "nobeat2");
        p_on_signal(&mut p, "sg", "nobeat3");
        p_on_enter(&mut p, &VolumeRef { kind: VolumeRefKind::Volume, id: "vv".into() }, "nobeat4", None, None);
        p_when(&mut p, "nobeat5", None);
        p_beat(&mut p, "x", crate::FnField, None);
        for _ in 0..33 { p_when(&mut p, "x", None); }
        manifest.programs.set("p".into(), p);
        d_expect_order(&mut manifest, "ghost", &["x".into()]);
        d_expect_within(&mut manifest, "p", "x", "x", 0.0);
        d_expect_never(&mut manifest, "samHit");

        let door = |name: &str| crate::scene_level::LevelDoorDef { name: name.into(), ..crate::scene_level::LevelDoorDef::zeroed() };
        let mut level = LintManifestLevel::zeroed();
        level.doors = vec![door("a"), door("c")];
        level.lights = vec![crate::scene_level::LightDef { group: "ceil".into(), ..crate::scene_level::LightDef::zeroed() }];
        level.switchables = vec![crate::scene_level::LevelSwitchablesItem { group: "desk".into(), ..crate::scene_level::LevelSwitchablesItem::zeroed() }];
        level.clear_plans = Some(vec![crate::scene_level::RoomClearPlan { door: "room1".into(), ..crate::scene_level::RoomClearPlan::zeroed() }]);
        let mut breach = crate::scene_level::BreachPlan::zeroed();
        breach.door = "a".into();
        breach.next.door = "z".into();
        level.breach = Some(breach);
        level.volumes = Some(Vec::new());
        level.devices = Some(vec![
            crate::scene_level::DeviceDef::Keypad(crate::devices::KeypadDef2 { name: "kp".into(), ..crate::devices::KeypadDef2::zeroed() }),
            crate::scene_level::DeviceDef::Camera(crate::devices::CameraDef2 { name: "cam".into(), group: Some("yard".into()), ..crate::devices::CameraDef2::zeroed() }),
        ]);

        let want: &[(LintProblemLevel, &str)] = &[
            (LintProblemLevel::Error, "door 'b' is not in the level (doors: a, c)"),
            (LintProblemLevel::Error, "device 'zz' is not in the level (devices: kp, cam)"),
            (LintProblemLevel::Error, "route 'r' is not in the level"),
            (LintProblemLevel::Error, "volume 'v' is not in the level (it authors none)"),
            (LintProblemLevel::Error, "plan 'breach' names door 'z', not in the level"),
            (LintProblemLevel::Error, "plan 'breach' has no hostiles"),
            (LintProblemLevel::Error, "plan 'nope': no room-clear plan of that room / door in the level"),
            (LintProblemLevel::Error, "mark 'm2' has no position (LevelData has no marks[] yet — declare it with D.mark(id, [x, 0, z]))"),
            (LintProblemLevel::Error, "mark 'm3' position is not finite"),
            (LintProblemLevel::Error, "loadout 'l1': team 'player' cannot be spawned (guard / operative / hostile)"),
            (LintProblemLevel::Error, "loadout 'l1': tint out of range"),
            (LintProblemLevel::Error, "loadout 'l1': god is a showcase cast's net — not in a script that has a play program ('empty')"),
            (LintProblemLevel::Error, "light group 'gone' is not in the level (groups: ceil, desk, security, yard)"),
            (LintProblemLevel::Error, "variant 'daylight': the level has no daylight block"),
            (LintProblemLevel::Warn, "program 'empty' declares no beats"),
            (LintProblemLevel::Warn, "program 'empty' has no trigger — nothing will ever start a beat"),
            (LintProblemLevel::Error, "program 'p': after(-1) is not a finite delay"),
            (LintProblemLevel::Error, "program 'p': trigger after 1.5s → beat 'nobeat', which is not declared"),
            (LintProblemLevel::Error, "program 'p': trigger event 'shot' → beat 'nobeat2', which is not declared"),
            (LintProblemLevel::Error, "program 'p': trigger signal 'sg' → beat 'nobeat3', which is not declared"),
            (LintProblemLevel::Error, "program 'p': trigger enter 'vv' → beat 'nobeat4', which is not declared"),
            (LintProblemLevel::Error, "program 'p': onEnter names volume 'vv' the script never declared with D.volume"),
            (LintProblemLevel::Error, "program 'p': trigger when → beat 'nobeat5', which is not declared"),
            (LintProblemLevel::Error, "program 'p': more than 32 when() predicates"),
            (LintProblemLevel::Error, "program 'p': endVeil / endHold must be positive"),
            (LintProblemLevel::Error, "expect: program 'ghost' is not declared"),
            (LintProblemLevel::Error, "expect.within: 0 s"),
        ];
        let got = lint_with_routes(&manifest, &level, &[]);
        let got: Vec<(LintProblemLevel, &str)> = got.iter().map(|p| (p.level, p.what.as_str())).collect();
        assert_eq!(got, want);
    }

    /// the one thing lint_manifest reads of `g`: a level's route is known by the name its row holds
    #[test]
    fn a_route_is_found_by_its_row_name() {
        let mut g = crate::game::Game::blank();
        let r = g.tab.route.push(crate::scene_level::PatrolRoute { name: "yard".into(), ..crate::scene_level::PatrolRoute::zeroed() });
        let mut manifest = Manifest::zeroed();
        d_route(&mut manifest, "yard");
        d_route(&mut manifest, "gone");
        let mut level = LintManifestLevel::zeroed();
        level.routes = vec![r];
        let got = lint_manifest(&mut g, &manifest, &level);
        let got: Vec<&str> = got.iter().map(|p| p.what.as_str()).collect();
        assert_eq!(got, ["route 'gone' is not in the level"]);
    }
}
