pub use ::game_types::cmd::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// The record for this tick, sampled from the host's live input, camera and view; `into` only saved an allocation, and a fresh record holds the same.
pub fn sample_cmd(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> crate::cmd::PlayerCmd {
    if let Some(aim) = g.hand_at {
        let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, aim, canvas.client_width, canvas.client_height, Some(f64::INFINITY), Some(1e-3));
        if front {
            g.tab.input[input].mouse_x = sx;
            g.tab.input[input].mouse_y = sy;
        }
    }
    let cam = &g.tab.camera[cam];
    let i = &g.tab.input[input];
    PlayerCmd {
        keys: i.keys.iter().collect(),
        pressed: i.pressed.iter().collect(),
        buttons: i.buttons, clicked: i.clicked, wheel: i.wheel, mx: i.mouse_x, my: i.mouse_y,
        w: canvas.client_width, h: canvas.client_height, pw: canvas.width, ph: canvas.height,
        view: PlayerCmdView {
            vp: cam.view_proj.iter().map(|x| *x as f64).collect(),
            ivp: cam.inv_view_proj.iter().map(|x| *x as f64).collect(),
            pos: cam.pos, forward: cam.forward, target: cam.target,
            yaw: cam.yaw, pitch: cam.pitch, distance: cam.distance, fov: cam.fov, aspect: cam.aspect, aim_lead: cam.aim_lead,
        },
    }
}

/// A string as JSON.stringify writes it.
fn text_string(out: &mut String, x: &str) {
    out.push('"');
    for c in x.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&alloc::format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// A number of a record as text: a word where JSON has no such number.
fn text_num(out: &mut String, x: f64) {
    match num_word("", x) {
        Some(w) => text_string(out, w),
        None => out.push_str(&js::num_str(x)),
    }
}

fn text_nums(out: &mut String, a: &[f64]) {
    out.push('[');
    for (i, x) in a.iter().enumerate() {
        if i > 0 { out.push(','); }
        text_num(out, *x);
    }
    out.push(']');
}

fn text_strs(out: &mut String, a: &[String]) {
    out.push('[');
    for (i, x) in a.iter().enumerate() {
        if i > 0 { out.push(','); }
        text_string(out, x);
    }
    out.push(']');
}

/// A record as text, to the bit, its fields in the order a sampled record has them.
pub fn cmd_text(c_2: &crate::cmd::PlayerCmd) -> String {
    let c = c_2;
    let mut o = String::new();
    o.push_str("{\"keys\":");
    text_strs(&mut o, &c.keys);
    o.push_str(",\"pressed\":");
    text_strs(&mut o, &c.pressed);
    let tops = [("buttons", c.buttons), ("clicked", c.clicked), ("wheel", c.wheel), ("mx", c.mx), ("my", c.my), ("w", c.w), ("h", c.h), ("pw", c.pw), ("ph", c.ph)];
    for (k, x) in tops {
        o.push_str(",\"");
        o.push_str(k);
        o.push_str("\":");
        text_num(&mut o, x);
    }
    let v = &c.view;
    o.push_str(",\"view\":{\"vp\":");
    text_nums(&mut o, &v.vp);
    o.push_str(",\"ivp\":");
    text_nums(&mut o, &v.ivp);
    o.push_str(",\"pos\":");
    text_nums(&mut o, &v.pos);
    o.push_str(",\"forward\":");
    text_nums(&mut o, &v.forward);
    o.push_str(",\"target\":");
    text_nums(&mut o, &v.target);
    let rest = [("yaw", v.yaw), ("pitch", v.pitch), ("distance", v.distance), ("fov", v.fov), ("aspect", v.aspect), ("aimLead", v.aim_lead)];
    for (k, x) in rest {
        o.push_str(",\"");
        o.push_str(k);
        o.push_str("\":");
        text_num(&mut o, x);
    }
    o.push_str("}}");
    o
}

/// What JSON.parse makes, as far as a record's reader needs to tell.
enum Json {
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
    Other,
}

/// JSON.parse over the text's bytes. A text that is no JSON panics where the TypeScript throws.
struct JsonReader<'a> {
    t: &'a [u8],
    at: usize,
}
impl<'a> JsonReader<'a> {
    fn space(&mut self) {
        while self.at < self.t.len() && matches!(self.t[self.at], b' ' | b'\t' | b'\n' | b'\r') { self.at += 1; }
    }
    fn eat(&mut self, b: u8) -> bool {
        self.space();
        if self.at < self.t.len() && self.t[self.at] == b {
            self.at += 1;
            return true;
        }
        false
    }
    fn need(&mut self, b: u8) {
        if !self.eat(b) { panic!("cmd_from_text: no JSON at byte {}", self.at); }
    }
    fn word(&mut self, w: &str, v: Json) -> Json {
        if !self.t[self.at..].starts_with(w.as_bytes()) { panic!("cmd_from_text: no JSON at byte {}", self.at); }
        self.at += w.len();
        v
    }
    fn hex4(&mut self) -> u32 {
        let h = core::str::from_utf8(&self.t[self.at..self.at + 4]).unwrap();
        self.at += 4;
        u32::from_str_radix(h, 16).unwrap()
    }
    fn string(&mut self) -> String {
        self.need(b'"');
        let mut out = String::new();
        loop {
            let start = self.at;
            while self.t[self.at] != b'"' && self.t[self.at] != b'\\' { self.at += 1; }
            out.push_str(core::str::from_utf8(&self.t[start..self.at]).unwrap());
            let b = self.t[self.at];
            self.at += 1;
            if b == b'"' { return out; }
            let e = self.t[self.at];
            self.at += 1;
            match e {
                b'b' => out.push('\u{8}'),
                b'f' => out.push('\u{c}'),
                b'n' => out.push('\n'),
                b'r' => out.push('\r'),
                b't' => out.push('\t'),
                b'u' => {
                    let mut u = self.hex4();
                    if (0xd800..0xdc00).contains(&u) && self.t[self.at..].starts_with(b"\\u") {
                        self.at += 2;
                        u = 0x10000 + ((u - 0xd800) << 10) + (self.hex4() - 0xdc00);
                    }
                    out.push(char::from_u32(u).unwrap_or('\u{fffd}'));
                }
                e => out.push(e as char),
            }
        }
    }
    fn value(&mut self) -> Json {
        self.space();
        match self.t[self.at] {
            b'{' => {
                self.at += 1;
                let mut o = Vec::new();
                if self.eat(b'}') { return Json::Obj(o); }
                loop {
                    self.space();
                    let k = self.string();
                    self.need(b':');
                    o.push((k, self.value()));
                    if self.eat(b',') { continue; }
                    self.need(b'}');
                    return Json::Obj(o);
                }
            }
            b'[' => {
                self.at += 1;
                let mut a = Vec::new();
                if self.eat(b']') { return Json::Arr(a); }
                loop {
                    a.push(self.value());
                    if self.eat(b',') { continue; }
                    self.need(b']');
                    return Json::Arr(a);
                }
            }
            b'"' => Json::Str(self.string()),
            b't' => self.word("true", Json::Other),
            b'f' => self.word("false", Json::Other),
            b'n' => self.word("null", Json::Other),
            _ => {
                let start = self.at;
                while self.at < self.t.len() && matches!(self.t[self.at], b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') { self.at += 1; }
                let s = core::str::from_utf8(&self.t[start..self.at]).unwrap();
                match s.parse::<f64>() {
                    Ok(x) => Json::Num(x),
                    Err(_) => panic!("cmd_from_text: no JSON at byte {}", start),
                }
            }
        }
    }
}

/// A number of a record read back: the wire's word for one that JSON cannot carry; what is no number is absent, as `absent_record` holds it.
fn read_num(j: &Json) -> f64 {
    match j {
        Json::Num(x) => *x,
        Json::Str(s) => word_num("", s).unwrap_or(f64::NAN),
        _ => f64::NAN,
    }
}

fn read_nums(j: &Json) -> Vec<f64> {
    match j {
        Json::Arr(a) => a.iter().map(read_num).collect(),
        _ => Vec::new(),
    }
}

fn read_strs(j: &Json) -> Vec<String> {
    match j {
        Json::Arr(a) => a.iter().filter_map(|x| match x { Json::Str(s) => Some(s.clone()), _ => None }).collect(),
        _ => Vec::new(),
    }
}

fn read_v3(j: &Json, at: &mut V3) {
    for (x, y) in at.iter_mut().zip(read_nums(j)) { *x = y; }
}

/// A record from its text. A field the text lacks reads as `absent_record` holds it, which the check answers as it answers `undefined`.
pub fn cmd_from_text(t: &str) -> crate::cmd::PlayerCmd {
    let mut r = JsonReader { t: t.as_bytes(), at: 0 };
    let top = r.value();
    r.space();
    if r.at != r.t.len() { panic!("cmd_from_text: no JSON at byte {}", r.at); }
    let mut c = absent_record();
    let Json::Obj(top) = top else { return c; };
    for (k, j) in &top {
        match k.as_str() {
            "keys" => c.keys = read_strs(j),
            "pressed" => c.pressed = read_strs(j),
            "buttons" => c.buttons = read_num(j),
            "clicked" => c.clicked = read_num(j),
            "wheel" => c.wheel = read_num(j),
            "mx" => c.mx = read_num(j),
            "my" => c.my = read_num(j),
            "w" => c.w = read_num(j),
            "h" => c.h = read_num(j),
            "pw" => c.pw = read_num(j),
            "ph" => c.ph = read_num(j),
            "view" => {
                let Json::Obj(view) = j else { continue; };
                let v = &mut c.view;
                for (k, j) in view {
                    match k.as_str() {
                        "vp" => v.vp = read_nums(j),
                        "ivp" => v.ivp = read_nums(j),
                        "pos" => read_v3(j, &mut v.pos),
                        "forward" => read_v3(j, &mut v.forward),
                        "target" => read_v3(j, &mut v.target),
                        "yaw" => v.yaw = read_num(j),
                        "pitch" => v.pitch = read_num(j),
                        "distance" => v.distance = read_num(j),
                        "fov" => v.fov = read_num(j),
                        "aspect" => v.aspect = read_num(j),
                        "aimLead" => v.aim_lead = read_num(j),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    c
}

/// The word for a number JSON cannot carry; None = the number goes as it is.
fn num_word(_k: &str, x: f64) -> Option<&'static str> {
    if x.is_finite() && x.to_bits() != (-0.0f64).to_bits() { return None; }
    Some(if x != x { "nan" } else if x == 0.0 { "-0" } else if x > 0.0 { "inf" } else { "-inf" })
}

/// The number a word stands for; None = the string is no such word.
fn word_num(_k: &str, x: &str) -> Option<f64> {
    match x {
        "-0" => Some(-0.0),
        "nan" => Some(f64::NAN),
        "inf" => Some(f64::INFINITY),
        "-inf" => Some(f64::NEG_INFINITY),
        _ => None,
    }
}

/// `Object.is`, the test of "came through to the bit" (a NaN is any NaN)
fn same(a: f64, b: f64) -> bool {
    (a != a && b != b) || a.to_bits() == b.to_bits()
}

/// The check's `num`: a number held in bounds, `dflt` where it is none; a change is written back and counted.
fn num(x: &mut f64, lo: f64, hi: f64, dflt: f64, fixed: &mut f64) -> f64 {
    let v = *x;
    let y = if v == v { if v < lo { lo } else if v > hi { hi } else { v } } else { dflt };
    if !same(v, y) {
        *x = y;
        *fixed += 1.0;
    }
    y
}

/// The check's `codes`: keeps what looks like a key code, no more than a tick can hold.
fn codes(a: &mut Vec<String>, keys: f64, key_len: f64, fixed: &mut f64) {
    let mut w = 0usize;
    let n = m::min(a.len() as f64, keys * 4.0);
    let mut i = 0usize;
    while (i as f64) < n && (w as f64) < keys {
        let s = &a[i];
        // (the pattern is ASCII, so a string it passes has as many bytes as the language's units; one it fails is out whatever its length)
        if s.len() as f64 <= key_len && !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric()) {
            a.swap(w, i);
            w += 1;
        }
        i += 1;
    }
    if a.len() != w {
        a.truncate(w);
        *fixed += 1.0;
    }
}

/// The check's `bits`: a word of mouse buttons, the five there are.
fn bits(x: &mut f64, fixed: &mut f64) {
    let v = *x;
    let y = if v == v { (js::to_i32(v) & 31) as f64 } else { 0.0 };
    if !same(v, y) {
        *x = y;
        *fixed += 1.0;
    }
}

/// The check's `mats`: a matrix of sixteen, all NaN where it is not one (its numbers are not bounded).
fn mats(a: &mut Vec<f64>, fixed: &mut f64) {
    if a.len() != 16 {
        *a = vec![f64::NAN; 16];
        *fixed += 1.0;
    }
}

/// A raw record that was no object, as the walk finds `{}`: every number absent (NaN answers `num` and `bits` as `undefined` does).
fn absent_record() -> PlayerCmd {
    let n = f64::NAN;
    PlayerCmd {
        keys: Vec::new(), pressed: Vec::new(), buttons: n, clicked: n, wheel: n, mx: n, my: n, w: n, h: n, pw: n, ph: n,
        view: PlayerCmdView {
            vp: Vec::new(), ivp: Vec::new(), pos: [0.0; 3], forward: [0.0; 3], target: [0.0; 3],
            yaw: n, pitch: n, distance: n, fov: n, aspect: n, aim_lead: n,
        },
    }
}

/// The check up to the view's working: returns the record, what was put right, and whether both matrices are numbers.
fn check_fields(l: &crate::knobs::CmdcheckK, raw: Option<PlayerCmd>, near: Option<V3>) -> (PlayerCmd, f64, bool) {
    // None = no object: counted as the walk counts each shape it finds missing there (the object; keys, pressed; the view; its three vectors)
    let (mut c, mut fixed) = match raw {
        Some(c) => (c, 0.0),
        None => (absent_record(), 1.0 + 2.0 + 1.0 + 3.0),
    };
    let f = &mut fixed;
    codes(&mut c.keys, l.keys, l.key_len, f);
    codes(&mut c.pressed, l.keys, l.key_len, f);
    bits(&mut c.buttons, f);
    bits(&mut c.clicked, f);
    num(&mut c.wheel, -l.wheel, l.wheel, 0.0, f);
    let w = num(&mut c.w, 1.0, l.span, 1280.0, f);
    let h = num(&mut c.h, 1.0, l.span, 720.0, f);
    num(&mut c.pw, 1.0, l.span, w, f);
    num(&mut c.ph, 1.0, l.span, h, f);
    num(&mut c.mx, -l.span, 2.0 * l.span, w / 2.0, f);
    num(&mut c.my, -l.span, 2.0 * l.span, h / 2.0, f);
    let v = &mut c.view;
    for x in v.pos.iter_mut() { num(x, -l.far, l.far, 0.0, f); }
    for x in v.target.iter_mut() { num(x, -l.far, l.far, 0.0, f); }
    for x in v.forward.iter_mut() { num(x, -1.0, 1.0, 0.0, f); }
    num(&mut v.yaw, -1e4, 1e4, 0.0, f);
    num(&mut v.pitch, -m::PI, m::PI, 0.96, f);
    let dist = num(&mut v.distance, l.dist_min, l.dist_max, 21.0, f);
    num(&mut v.fov, l.fov_min, l.fov_max, 0.6, f);
    num(&mut v.aspect, l.aspect_min, l.aspect_max, w / h, f);
    num(&mut v.aim_lead, 0.0, 1.0, 0.0, f);
    mats(&mut v.vp, f);
    mats(&mut v.ivp, f);
    let mut whole = true;
    for i in 0..16 {
        if !v.vp[i].is_finite() || !v.ivp[i].is_finite() { whole = false; }
    }
    if let Some(near) = near {
        if l.reach > 0.0 {   // a remote seat looks at its own man's surroundings
            let t = &mut v.target;
            let (dx, dy, dz) = (t[0] - near[0], t[1] - near[1], t[2] - near[2]);
            let d = m::sqrt(dx * dx + dy * dy + dz * dz);
            if d > l.reach {
                let k = l.reach / d;
                t[0] = near[0] + dx * k;
                t[1] = near[1] + dy * k;
                t[2] = near[2] + dz * k;
                *f += 1.0;
            }
            if dist > 2.0 * l.reach {
                v.distance = 2.0 * l.reach;
                *f += 1.0;
            }
        }
    }
    (c, fixed, whole)
}

/// The pose a view is worked out from, put on the check's lens; the pitch is held off straight down.
fn pose_onto_lens(steep: f64, v: &mut PlayerCmdView, s: &mut crate::camera::FollowCamera, fixed: &mut f64) {
    let pitch = v.pitch;
    let p = if pitch > steep { steep } else if pitch < -steep { -steep } else { pitch };
    if p != pitch {
        v.pitch = p;
        *fixed += 1.0;
    }
    s.target = v.target;
    s.yaw = v.yaw;
    s.pitch = p;
    s.distance = v.distance;
    s.fov = v.fov;
    s.aspect = v.aspect;
}

/// The check's `put`: a vector written over with what the lens worked out, counted once if anything moved.
fn put(a: &mut [f64], b: impl Iterator<Item = f64>, fixed: &mut f64) {
    let mut off = false;
    for (x, y) in a.iter_mut().zip(b) {
        if !same(*x, y) {
            *x = y;
            off = true;
        }
    }
    if off { *fixed += 1.0; }
}

/// What the record says of the eye and the matrices, replaced by the lens's working.
fn view_from_lens(v: &mut PlayerCmdView, s: &crate::camera::FollowCamera, fixed: &mut f64) {
    put(&mut v.pos, s.pos.iter().copied(), fixed);
    put(&mut v.forward, s.forward.iter().copied(), fixed);
    put(&mut v.vp, s.view_proj.iter().map(|x| *x as f64), fixed);
    put(&mut v.ivp, s.inv_view_proj.iter().map(|x| *x as f64), fixed);
}

/// The whole check on a record already typed; `None` = no object.
fn check_record(g: &Game, raw: Option<PlayerCmd>, near: Option<V3>) -> (PlayerCmd, f64) {
    let l = g.k.cmdcheck;
    let (mut c, mut fixed, whole) = check_fields(&l, raw, near);
    if near.is_some() || !whole {
        let mut s = crate::camera::FollowCamera::blank();
        pose_onto_lens(l.steep, &mut c.view, &mut s, &mut fixed);
        crate::camera::rebuild(&mut s);
        view_from_lens(&mut c.view, &s, &mut fixed);
    }
    (c, fixed)
}

/// A record made one an honest seat could have sent. `raw` None = what was sent was no object.
pub(super) fn check_cmd(g: &crate::game::Game, raw: Option<&crate::cmd::PlayerCmd>, near: Option<V3>) -> crate::cmd::CheckCmd {
    CheckCmd { cmd: check_record(g, raw.cloned(), near).0 }
}

/// `c` with nothing pressed: the same cursor and view, no key, no button, no wheel.
fn still_cmd(c_2: &crate::cmd::PlayerCmd) -> crate::cmd::PlayerCmd {
    let c = c_2;
    PlayerCmd {
        keys: Vec::new(), pressed: Vec::new(), buttons: 0.0, clicked: 0.0, wheel: 0.0,
        mx: c.mx, my: c.my, w: c.w, h: c.h, pw: c.pw, ph: c.ph, view: c.view.clone(),
    }
}

/// A record as a tick would take it, for code that must leave it as it came: the check on a copy.
pub(super) fn honest_copy(g: &crate::game::Game, c_2: &crate::cmd::PlayerCmd, near: Option<V3>) -> crate::cmd::PlayerCmd {
    check_cmd(g, Some(c_2), near).cmd
}

/// The body of loadLens: a record's view put on a camera, its matrices rounded to the camera's singles as `Float32Array.set` does.
fn lens_from(cam: &mut crate::camera::FollowCamera, v: &PlayerCmdView) {
    for (x, y) in cam.view_proj.iter_mut().zip(v.vp.iter()) { *x = *y as f32; }
    for (x, y) in cam.inv_view_proj.iter_mut().zip(v.ivp.iter()) { *x = *y as f32; }
    cam.pos = v.pos;
    cam.forward = v.forward;
    cam.target = v.target;
    cam.yaw = v.yaw;
    cam.pitch = v.pitch;
    cam.distance = v.distance;
    cam.fov = v.fov;
    cam.aspect = v.aspect;
    cam.aim_lead = v.aim_lead;
}

/// A record's view put on a camera of the game's own.
pub(super) fn load_lens(g: &mut crate::game::Game, cam: CameraId, v: &crate::cmd::PlayerCmdView) {
    lens_from(&mut g.tab.camera[cam], v);
}

/// A player's seat: its Input and its camera, of the game's own; the view is seats.ts's to set.
pub(super) fn seat_new(g: &mut crate::game::Game, index: Option<f64>) -> SeatId {
    let input = crate::input::input_detached(g);
    let cam = g.tab.camera.push(crate::camera::FollowCamera::blank());
    let mut s = Seat::blank();
    s.index = index.unwrap_or(0.0);
    s.input = input;
    s.cam = cam;
    s.mark = g.tab.camera[cam].desired_serial;
    g.tab.cmd_seat.push(s)
}

/// His man.
pub fn seat_man(g: &crate::game::Game, id: SeatId) -> PlayerId {
    crate::seats::seat(g, id).player
}

/// The record of the tick loaded into the seat's Input, camera and view size.
fn load_row(s: &mut Seat, i: &mut crate::input::Input, cam: &mut crate::camera::FollowCamera, c: &PlayerCmd) {
    s.cmd = Some(c.clone());
    i.keys.clear();
    for k in &c.keys { i.keys.add(k.clone()); }
    i.pressed.clear();
    for k in &c.pressed { i.pressed.add(k.clone()); }
    let (buttons, clicked) = (js::to_i32(c.buttons), js::to_i32(c.clicked));
    let mask = js::to_i32(s.mask) & (buttons & !clicked);   // (let go, or pressed afresh since the last record: the swallow is over)
    s.mask = mask as f64;
    i.buttons = (buttons & !mask) as f64;
    i.clicked = c.clicked;
    i.wheel = c.wheel;
    i.mouse_x = c.mx;
    i.mouse_y = c.my;
    s.canvas.client_width = c.w;
    s.canvas.client_height = c.h;
    s.canvas.width = c.pw;
    s.canvas.height = c.ph;
    lens_from(cam, &c.view);
}

pub(super) fn seat_load(g: &mut crate::game::Game, id: SeatId, c_2: &crate::cmd::PlayerCmd) {
    let (input, cam) = (g.tab.cmd_seat[id].input, g.tab.cmd_seat[id].cam);
    load_row(&mut g.tab.cmd_seat[id], &mut g.tab.input[input], &mut g.tab.camera[cam], c_2);
}

/// What was handed, once; else a still record of the view it last had (`like`'s before it has had one).
pub(super) fn seat_handed(g: &mut crate::game::Game, id: SeatId, like: &crate::cmd::PlayerCmd) -> crate::cmd::PlayerCmd {
    let s = &mut g.tab.cmd_seat[id];
    match s.waiting.take() {
        Some(c) => c,
        None => still_cmd(s.cmd.as_ref().unwrap_or(like)),
    }
}

/// The swallow on a seat's row.
fn swallow_row(s: &mut Seat, i: &mut crate::input::Input, bits: f64) {
    let b = js::to_i32(bits);
    s.mask = (js::to_i32(s.mask) | b) as f64;
    i.buttons = (js::to_i32(i.buttons) & !b) as f64;
}

/// Held buttons the game took for something of its own: they read as up until the player lets go.
pub fn seat_swallow(g: &mut crate::game::Game, id: SeatId, bits: f64) {
    let input = g.tab.cmd_seat[id].input;
    swallow_row(&mut g.tab.cmd_seat[id], &mut g.tab.input[input], bits);
}

/// OUTPUT: the look-at a tick cut the seat's camera to, once; None = no cut since the last call (a cut with the same numbers is still a cut).
pub(super) fn seat_take_cut(g: &mut crate::game::Game, id: SeatId) -> Option<V3> {
    let cam = g.tab.cmd_seat[id].cam;
    cut_row(&mut g.tab.cmd_seat[id], &g.tab.camera[cam])
}

/// did this key go down in the step the first seat's record was of
pub(super) fn seat_pressed(g: &crate::game::Game, key: &str) -> bool { g.tab.cmd_seat[g.seats[0]].cmd.as_ref().is_some_and(|c| c.pressed.iter().any(|k| k == key)) }

/// The take on a seat's row: `mark` is the serial of the last write of `desired` taken.
fn cut_row(s: &mut Seat, cam: &crate::camera::FollowCamera) -> Option<V3> {
    if cam.desired_serial == s.mark { return None; }
    s.mark = cam.desired_serial;
    Some(cam.desired)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A record as an honest seat sends it: its view worked out by a real camera.
    fn honest() -> PlayerCmd {
        let mut g = Game::blank();
        let mut c = crate::camera::FollowCamera::blank();
        c.target = [3.0, 1.0, -7.0]; c.yaw = 0.7; c.pitch = 0.96; c.distance = 21.0; c.aspect = 1280.0 / 720.0;
        let id = g.tab.camera.push(c);
        crate::camera::follow_camera_rebuild(&mut g, id);
        let c = &g.tab.camera[id];
        PlayerCmd {
            keys: vec![String::from("KeyW"), String::from("ShiftLeft")], pressed: vec![String::from("Digit1")],
            buttons: 5.0, clicked: 1.0, wheel: -120.0, mx: 640.0, my: 360.0, w: 1280.0, h: 720.0, pw: 2560.0, ph: 1440.0,
            view: PlayerCmdView {
                vp: c.view_proj.iter().map(|x| *x as f64).collect(), ivp: c.inv_view_proj.iter().map(|x| *x as f64).collect(),
                pos: c.pos, forward: c.forward, target: c.target, yaw: c.yaw, pitch: c.pitch, distance: c.distance, fov: c.fov, aspect: c.aspect, aim_lead: c.aim_lead,
            },
        }
    }
    fn checked(raw: Option<PlayerCmd>, near: Option<V3>) -> (PlayerCmd, f64) {
        check_record(&Game::blank(), raw, near)
    }
    fn k() -> crate::knobs::CmdcheckK { crate::knobs::CmdcheckK::page() }

    /// The tick takes a seat's record through this: a machine that plays by the rules has nothing put right.
    /// A FAILURE MEANS: an honest player's clicks are moved, or he is counted as a cheat.
    #[test]
    fn an_honest_record_comes_through_untouched() {
        let c = honest();
        let (out, fixed) = checked(Some(c.clone()), None);
        assert_eq!(fixed, 0.0);
        assert_eq!(cmd_text(&out), cmd_text(&c));
    }

    #[test]
    fn a_record_with_no_object_is_a_still_player_at_the_middle_of_a_picture() {
        let (c, fixed) = checked(None, None);
        assert!(fixed > 0.0);
        assert!(c.keys.is_empty() && c.pressed.is_empty());
        assert_eq!((c.buttons, c.clicked, c.wheel), (0.0, 0.0, 0.0));
        assert_eq!((c.w, c.h, c.pw, c.ph, c.mx, c.my), (1280.0, 720.0, 1280.0, 720.0, 640.0, 360.0));
        assert!(c.view.vp.iter().chain(c.view.ivp.iter()).all(|x| x.is_finite()), "its view is worked out");
    }

    /// No key code a keyboard could not send and no more than a tick holds.
    #[test]
    fn a_key_is_kept_only_if_it_looks_like_one_and_a_tick_holds_only_so_many() {
        let l = k();
        let mut c = honest();
        c.keys = vec![String::from("KeyW"), String::from("bad key"), String::new(), String::from("Ab_c"), "x".repeat(l.key_len as usize + 1), String::from("KeyE")];
        let (out, fixed) = checked(Some(c), None);
        assert_eq!(out.keys, vec![String::from("KeyW"), String::from("KeyE")]);
        assert_eq!(fixed, 1.0, "counted once for the list");
        let mut c = honest();
        c.pressed = (0..(l.keys as usize * 6)).map(|i| alloc::format!("K{i}")).collect();
        let (out, _) = checked(Some(c), None);
        assert_eq!(out.pressed.len(), l.keys as usize);
        // nor does a long list of rubbish buy a look at what lies past it
        let mut c = honest();
        c.keys = (0..(l.keys as usize * 4)).map(|_| String::from("bad!")).chain([String::from("KeyW")]).collect();
        assert!(checked(Some(c), None).0.keys.is_empty());
    }

    /// Each number stays in what a real screen and a real camera could send, or takes its default if it is none.
    #[test]
    fn a_number_out_of_bounds_is_held_to_them_and_one_that_is_none_takes_its_default() {
        let l = k();
        let nan = f64::NAN;
        let mut c = honest();
        c.buttons = 96.0 + 5.0;
        c.clicked = nan;
        c.wheel = l.wheel * 10.0;
        c.w = nan;
        c.h = 0.0;
        c.mx = 1e12;
        c.view.yaw = nan;
        c.view.pitch = nan;
        c.view.distance = 1e9;
        c.view.fov = 0.0;
        c.view.aim_lead = 7.0;
        c.view.target = [nan, 1e12, 0.0];
        let (out, _) = checked(Some(c), None);
        assert_eq!(out.buttons, 5.0, "only the buttons a mouse has");
        assert_eq!(out.clicked, 0.0);
        assert_eq!(out.wheel, l.wheel);
        assert_eq!((out.w, out.h), (1280.0, 1.0));
        assert_eq!(out.mx, 2.0 * l.span);
        assert_eq!((out.view.yaw, out.view.pitch, out.view.distance, out.view.fov, out.view.aim_lead), (0.0, 0.96, l.dist_max, l.fov_min, 1.0));
        assert_eq!(out.view.target, [0.0, l.far, 0.0]);
        assert!(out.view.vp.iter().all(|x| x.is_finite()));
    }

    /// A seat that is not the host's says what it looks at, but no further from its own man than it could really look, and the matrices are worked out here.
    /// A FAILURE MEANS: a remote player can click on what his man cannot be looking at.
    #[test]
    fn a_remote_seat_looks_near_its_own_man_and_its_view_is_worked_out_here() {
        let l = k();
        let man: V3 = [0.0, 0.0, 0.0];
        let mut c = honest();
        c.view.target = [l.reach * 3.0, 0.0, 0.0];
        c.view.distance = l.dist_max;
        c.view.vp = vec![1.0; 16];
        let (out, fixed) = checked(Some(c), Some(man));
        assert!(fixed >= 3.0);
        assert!(m::abs(v3::dist(out.view.target, man) - l.reach) < 1e-9, "held to the reach: {:?}", out.view.target);
        assert!(out.view.distance <= 2.0 * l.reach);
        assert_ne!(out.view.vp, vec![1.0; 16], "a matrix of its own making does not stand");
        // near enough, it stands, and an honest view is the one worked out
        let mut c = honest();
        c.view.target = [0.5, 0.0, 0.0];
        let (out, _) = checked(Some(c), Some(man));
        assert_eq!(out.view.target, [0.5, 0.0, 0.0]);
    }

    /// A camera straight down has no way up: a view that says so is held off it.
    #[test]
    fn a_view_is_held_off_straight_down() {
        let l = k();
        let mut c = honest();
        c.view.pitch = m::PI / 2.0;
        c.view.vp[0] = f64::NAN;
        let (out, fixed) = checked(Some(c), None);
        assert_eq!(out.view.pitch, l.steep);
        assert!(fixed >= 1.0);
        assert!(out.view.vp.iter().chain(out.view.ivp.iter()).all(|x| x.is_finite()));
    }

    /// A button the game took for itself reads as up until he lets go or presses it afresh; the others go on working.
    /// A FAILURE MEANS: the click that closed a menu also fires his gun.
    #[test]
    fn a_swallowed_button_reads_up_until_he_lets_go_or_presses_it_afresh() {
        let mut s = Seat::blank();
        let mut input = crate::input::Input::blank();
        let mut cam = crate::camera::FollowCamera::blank();
        let mut load = |s: &mut Seat, input: &mut crate::input::Input, buttons: f64, clicked: f64| {
            let mut c = honest();
            c.buttons = buttons;
            c.clicked = clicked;
            load_row(s, input, &mut cam, &c);
            input.buttons
        };
        assert_eq!(load(&mut s, &mut input, 5.0, 1.0), 5.0, "two buttons down");
        swallow_row(&mut s, &mut input, 1.0);
        assert_eq!(input.buttons, 4.0, "it reads up at once");
        assert_eq!(load(&mut s, &mut input, 5.0, 0.0), 4.0, "and on the next record, held");
        assert_eq!(load(&mut s, &mut input, 5.0, 1.0), 5.0, "pressed afresh it is his again");
        swallow_row(&mut s, &mut input, 1.0);
        assert_eq!(load(&mut s, &mut input, 4.0, 0.0), 4.0, "let go");
        assert_eq!(s.mask, 0.0, "and that is the end of it");
        assert_eq!(load(&mut s, &mut input, 5.0, 0.0), 5.0, "the next press is his");
    }

    /// The take of a cut goes by the serial: unchanged is no cut, the same numbers written again are one, and each is taken once.
    #[test]
    fn a_cut_is_taken_once_by_its_serial() {
        let mut s = Seat::blank();
        let mut cam = crate::camera::FollowCamera::blank();
        assert_eq!(cut_row(&mut s, &cam), None);
        cam.desired_serial += 1;
        assert_eq!(cut_row(&mut s, &cam), Some(cam.desired));
        assert_eq!(cut_row(&mut s, &cam), None);
    }

    /// A record goes over the wire and into a kept run as text: what comes back is what went, to the bit, with words for what JSON cannot carry.
    /// A FAILURE MEANS: a kept run or a guest plays differently from what was played.
    #[test]
    fn a_record_goes_to_text_and_back_to_the_bit() {
        let mut c = honest();
        c.keys.push(String::from("a\"b\\c\n\t\u{1}é"));
        c.clicked = -0.0;
        c.wheel = f64::NAN;
        c.mx = f64::INFINITY;
        c.my = f64::NEG_INFINITY;
        c.pw = 2560.5;
        c.ph = 1e21;
        c.view.vp[0] = -0.0;
        let text = cmd_text(&c);
        let back = cmd_from_text(&text);
        assert_eq!(back.keys, c.keys);
        assert_eq!(back.pressed, c.pressed);
        assert_eq!(back.clicked.to_bits(), (-0.0f64).to_bits());
        assert!(back.wheel.is_nan());
        assert_eq!((back.mx, back.my, back.pw, back.ph), (f64::INFINITY, f64::NEG_INFINITY, 2560.5, 1e21));
        assert_eq!(back.view.vp.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), c.view.vp.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
        assert_eq!(back.view.pos.map(f64::to_bits), c.view.pos.map(f64::to_bits));
        assert_eq!(back.view.aim_lead, c.view.aim_lead);
        assert_eq!(cmd_text(&back), text, "and it says the same thing again");
    }

    /// What a record's text lacks reads as absent, and the check takes it as it takes no object.
    #[test]
    fn a_field_the_text_lacks_is_absent_and_the_check_makes_it_good() {
        let c = cmd_from_text("{\"keys\":[\"KeyW\"]}");
        assert_eq!(c.keys, vec![String::from("KeyW")]);
        assert!(c.buttons.is_nan() && c.view.yaw.is_nan());
        let (out, fixed) = checked(Some(c), None);
        assert!(fixed > 0.0);
        assert_eq!((out.buttons, out.w, out.h), (0.0, 1280.0, 720.0));
        assert_eq!(out.keys, vec![String::from("KeyW")]);
    }
}
