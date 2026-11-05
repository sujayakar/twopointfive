//! src/game/cmd.ts lines 1 to 194
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// cmd.ts:40 cam: the host's live camera. The skeleton dropped it as "once", but `g.director.cam` is an Option lent by the host, and the camera's methods have no `this`.
fn seam_host_lens(g: &mut Game) -> crate::camera::FollowCamera {
    todo!("needs: sample_cmd's `cam` parameter back (FollowCamera as a value class, `this: &mut FollowCamera`)")
}

/// The record for this tick, sampled from the host's live input, camera and view; `into` only saved an allocation, and a fresh record holds the same.   (cmd.ts sampleCmd)
pub fn sample_cmd(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View, into: Option<&crate::cmd::PlayerCmd>) -> crate::cmd::PlayerCmd {
    let aim = g.puppet.as_ref().and_then(|p| p.aim);
    if let Some(aim) = aim {
        let (sx, sy, front) = crate::camera::follow_camera_project(g, aim, canvas.client_width, canvas.client_height, Some(f64::INFINITY), Some(1e-3));
        if front {
            g.tab.input[input].mouse_x = sx;
            g.tab.input[input].mouse_y = sy;
        }
    }
    let cam = seam_host_lens(g);
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

/// `String(x)` of a finite number other than -0: the shortest digits that come back to the bit, laid out as the language lays them out.
fn js_number(x: f64) -> String {
    let e = alloc::format!("{:e}", x);
    let (mant, exp) = e.split_once('e').unwrap();
    let (sign, mant) = match mant.strip_prefix('-') { Some(rest) => ("-", rest), None => ("", mant) };
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let n = exp.parse::<i32>().unwrap() + 1;
    let mut s = String::from(sign);
    if k <= n && n <= 21 {
        s.push_str(&digits);
        for _ in 0..n - k { s.push('0'); }
    } else if 0 < n && n <= 21 {
        s.push_str(&digits[..n as usize]);
        s.push('.');
        s.push_str(&digits[n as usize..]);
    } else if -6 < n && n <= 0 {
        s.push_str("0.");
        for _ in 0..-n { s.push('0'); }
        s.push_str(&digits);
    } else {
        s.push_str(&digits[..1]);
        if k > 1 {
            s.push('.');
            s.push_str(&digits[1..]);
        }
        s.push('e');
        s.push(if n > 0 { '+' } else { '-' });
        s.push_str(&alloc::format!("{}", (n - 1).abs()));
    }
    s
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
        None => out.push_str(&js_number(x)),
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

/// A record as text, to the bit, its fields in the order a sampled record has them.   (cmd.ts cmdText)
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

/// A record from its text. A field the text lacks reads as `absent_record` holds it, which the check answers as it answers `undefined`.   (cmd.ts cmdFromText)
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

/// The word for a number JSON cannot carry; None = the number goes as it is.   (cmd.ts numWord)
pub fn num_word(_k: &str, x: f64) -> Option<&'static str> {
    if x.is_finite() && x.to_bits() != (-0.0f64).to_bits() { return None; }
    Some(if x != x { "nan" } else if x == 0.0 { "-0" } else if x > 0.0 { "inf" } else { "-inf" })
}

/// The number a word stands for; None = the string is no such word.   (cmd.ts wordNum)
pub fn word_num(_k: &str, x: &str) -> Option<f64> {
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

/// The check's `num`: a number held in bounds, `dflt` where it is none; a change is written back and counted.   (cmd.ts checkCmdIn)
fn num(x: &mut f64, lo: f64, hi: f64, dflt: f64, fixed: &mut f64) -> f64 {
    let v = *x;
    let y = if v == v { if v < lo { lo } else if v > hi { hi } else { v } } else { dflt };
    if !same(v, y) {
        *x = y;
        *fixed += 1.0;
    }
    y
}

/// The check's `codes`: keeps what looks like a key code, no more than a tick can hold.   (cmd.ts checkCmdIn, KEY_CODE)
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

/// The check's `bits`: a word of mouse buttons, the five there are.   (cmd.ts checkCmdIn)
fn bits(x: &mut f64, fixed: &mut f64) {
    let v = *x;
    let y = if v == v { (js::to_i32(v) & 31) as f64 } else { 0.0 };
    if !same(v, y) {
        *x = y;
        *fixed += 1.0;
    }
}

/// The check's `mats`: a matrix of sixteen, all NaN where it is not one (its numbers are not bounded).   (cmd.ts checkCmdIn)
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

/// The check up to the view's working: returns the record, what was put right, and whether both matrices are numbers.   (cmd.ts checkCmdIn)
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

/// The pose a view is worked out from, put on the check's lens; the pitch is held off straight down.   (cmd.ts checkCmdIn)
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

/// The check's `put`: a vector written over with what the lens worked out, counted once if anything moved.   (cmd.ts checkCmdIn)
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

/// What the record says of the eye and the matrices, replaced by the lens's working.   (cmd.ts checkCmdIn)
fn view_from_lens(v: &mut PlayerCmdView, s: &crate::camera::FollowCamera, fixed: &mut f64) {
    put(&mut v.pos, s.pos.iter().copied(), fixed);
    put(&mut v.forward, s.forward.iter().copied(), fixed);
    put(&mut v.vp, s.view_proj.iter().map(|x| *x as f64), fixed);
    put(&mut v.ivp, s.inv_view_proj.iter().map(|x| *x as f64), fixed);
}

/// The whole check on a record already typed; `None` = no object.   (cmd.ts checkCmdIn)
fn check_record(g: &mut Game, raw: Option<PlayerCmd>, near: Option<V3>) -> (PlayerCmd, f64) {
    let l = g.k.cmdcheck;
    let (mut c, mut fixed, whole) = check_fields(&l, raw, near);
    if near.is_some() || !whole {
        // (`new FollowCamera()`: the class has no constructor, its field initialisers are blank())
        let mut s = g.m.cmd.check_lens.take().unwrap_or_else(crate::camera::FollowCamera::blank);
        pose_onto_lens(l.steep, &mut c.view, &mut s, &mut fixed);
        seam_rebuild(g, &mut s);
        view_from_lens(&mut c.view, &s, &mut fixed);
        g.m.cmd.check_lens = Some(s);
    }
    (c, fixed)
}

/// cmd.ts:118 S.rebuild(): the stub of FollowCamera.rebuild takes `g` alone and cannot name this lens.
fn seam_rebuild(g: &mut Game, lens: &mut crate::camera::FollowCamera) {
    todo!("needs: follow_camera_rebuild on a given lens (the check's own, not the game's)")
}

/// A record made one an honest seat could have sent; `fixed` = how many things were put right. `raw` None = what was sent was no object.   (cmd.ts checkCmd)
pub fn check_cmd(g: &mut crate::game::Game, raw: Option<&crate::cmd::PlayerCmd>, near: Option<V3>) -> crate::cmd::CheckCmd {
    let r = check_cmd_in(g, raw, near);
    CheckCmd { cmd: r.cmd, fixed: r.fixed }
}

/// The check itself, on a copy: `raw` is only lent here, so a caller always takes `cmd` from the answer.   (cmd.ts checkCmdIn)
pub fn check_cmd_in(g: &mut crate::game::Game, raw: Option<&crate::cmd::PlayerCmd>, near: Option<V3>) -> crate::cmd::CheckCmdIn {
    let (cmd, fixed) = check_record(g, raw.cloned(), near);
    CheckCmdIn { cmd, fixed }
}

/// `c` with nothing pressed: the same cursor and view, no key, no button, no wheel.   (cmd.ts stillCmd)
pub fn still_cmd(c_2: &crate::cmd::PlayerCmd) -> crate::cmd::PlayerCmd {
    let c = c_2;
    PlayerCmd {
        keys: Vec::new(), pressed: Vec::new(), buttons: 0.0, clicked: 0.0, wheel: 0.0,
        mx: c.mx, my: c.my, w: c.w, h: c.h, pw: c.pw, ph: c.ph, view: c.view.clone(),
    }
}

/// A record as a tick would take it, for code that must leave it as it came: the check on a copy.   (cmd.ts honestCopy)
pub fn honest_copy(g: &mut crate::game::Game, c_2: &crate::cmd::PlayerCmd, near: Option<V3>) -> crate::cmd::PlayerCmd {
    check_cmd(g, Some(c_2), near).cmd
}

/// The body of loadLens: a record's view put on a camera, its matrices rounded to the camera's singles as `Float32Array.set` does.   (cmd.ts loadLens)
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

/// cmd.ts:139 cam: the parameter is still dropped as a once-a-game object, but it is a seat's lens or the game's aside lens.
fn seam_load_lens(g: &mut Game, v: &PlayerCmdView) {
    todo!("needs: load_lens's `cam` parameter back; its body is then lens_from(cam, v)")
}

/// A record's view put on a camera of the game's own.   (cmd.ts loadLens)
pub fn load_lens(g: &mut crate::game::Game, v: &crate::cmd::PlayerCmdView) {
    seam_load_lens(g, v)
}

/// A player's seat: its Input and its camera, of the game's own; the view is seats.ts's to set.   (cmd.ts Seat.constructor)
pub fn seat_new(g: &mut crate::game::Game, index: Option<f64>) -> SeatId {
    let input = crate::input::input_detached(g);
    let mut s = Seat::blank();
    s.index = index.unwrap_or(0.0);
    s.input = input;
    s.mark = s.cam.desired_serial;
    g.tab.cmd_seat.push(s)
}

/// His man: the seat's set is on Game while the pass is about him (always, in a game of one), else on his own row.   (cmd.ts Seat.man)
pub fn seat_man(g: &mut crate::game::Game, id: SeatId) -> PlayerId {
    if g.seat == id { return g.player; }
    match &g.tab.cmd_seat[id].own {
        Some(own) => own.player,
        None => g.player,
    }
}

/// The record of the tick loaded into the seat's Input, camera and view size.   (cmd.ts Seat.load)
fn load_row(s: &mut Seat, i: &mut crate::input::Input, c: &PlayerCmd) {
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
    lens_from(&mut s.cam, &c.view);
}

/// (cmd.ts Seat.load)
pub fn seat_load(g: &mut crate::game::Game, id: SeatId, c_2: &crate::cmd::PlayerCmd) {
    let input = g.tab.cmd_seat[id].input;
    load_row(&mut g.tab.cmd_seat[id], &mut g.tab.input[input], c_2);
}

/// A record waiting for this seat's next tick.   (cmd.ts Seat.hand)
pub fn seat_hand(g: &mut crate::game::Game, id: SeatId, c_2: &crate::cmd::PlayerCmd) {
    g.tab.cmd_seat[id].waiting = Some(c_2.clone());
}

/// What was handed, once; else a still record of the view it last had (`like`'s before it has had one).   (cmd.ts Seat.handed)
pub fn seat_handed(g: &mut crate::game::Game, id: SeatId, like: &crate::cmd::PlayerCmd) -> crate::cmd::PlayerCmd {
    let s = &mut g.tab.cmd_seat[id];
    match s.waiting.take() {
        Some(c) => c,
        None => still_cmd(s.cmd.as_ref().unwrap_or(like)),
    }
}

/// The swallow on a seat's row.   (cmd.ts Seat.swallow)
fn swallow_row(s: &mut Seat, i: &mut crate::input::Input, bits: f64) {
    let b = js::to_i32(bits);
    s.mask = (js::to_i32(s.mask) | b) as f64;
    i.buttons = (js::to_i32(i.buttons) & !b) as f64;
}

/// Held buttons the game took for something of its own: they read as up until the player lets go.   (cmd.ts Seat.swallow)
pub fn seat_swallow(g: &mut crate::game::Game, id: SeatId, bits: f64) {
    let input = g.tab.cmd_seat[id].input;
    swallow_row(&mut g.tab.cmd_seat[id], &mut g.tab.input[input], bits);
}

/// OUTPUT: the look-at a tick cut the seat's camera to, once; None = no cut since the last call (a cut with the same numbers is still a cut).   (cmd.ts Seat.takeCut)
pub fn seat_take_cut(g: &mut crate::game::Game, id: SeatId) -> Option<V3> {
    cut_row(&mut g.tab.cmd_seat[id])
}

/// The take on a seat's row: `mark` is the serial of the last write of `desired` taken.   (cmd.ts Seat.takeCut)
fn cut_row(s: &mut Seat) -> Option<V3> {
    if s.cam.desired_serial == s.mark { return None; }
    s.mark = s.cam.desired_serial;
    Some(s.cam.desired)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(b: u64) -> f64 { f64::from_bits(b) }
    fn r(keys: &[&str], pressed: &[&str], n: &[f64], vp: &[f64], ivp: &[f64], pos: &[f64], fw: &[f64], t: &[f64], o: &[f64]) -> PlayerCmd {
        let st = |a: &[&str]| a.iter().map(|s| String::from(*s)).collect::<Vec<_>>();
        PlayerCmd {
            keys: st(keys), pressed: st(pressed), buttons: n[0], clicked: n[1], wheel: n[2], mx: n[3], my: n[4], w: n[5], h: n[6], pw: n[7], ph: n[8],
            view: PlayerCmdView {
                vp: vp.to_vec(), ivp: ivp.to_vec(), pos: [pos[0], pos[1], pos[2]], forward: [fw[0], fw[1], fw[2]], target: [t[0], t[1], t[2]],
                yaw: o[0], pitch: o[1], distance: o[2], fov: o[3], aspect: o[4], aim_lead: o[5],
            },
        }
    }
    fn nums(c: &PlayerCmd) -> Vec<f64> {
        let v = &c.view;
        let mut a = vec![c.buttons, c.clicked, c.wheel, c.mx, c.my, c.w, c.h, c.pw, c.ph, v.yaw, v.pitch, v.distance, v.fov, v.aspect, v.aim_lead];
        a.extend(v.pos); a.extend(v.forward); a.extend(v.target); a.extend(v.vp.iter()); a.extend(v.ivp.iter());
        a
    }

    /// The TypeScript's own results for checkCmd (generated under bun with EXACT on), the lens's working taken from its answer: rebuild is a seam.
    #[test]
    fn check_matches_the_typescript() {
        let l = crate::knobs::page_cmdcheck();
        for (name, raw, near, want, want_fixed) in cases() {
            let (mut c, mut fixed, whole) = check_fields(&l, raw, near);
            if near.is_some() || !whole {
                let mut s = crate::camera::FollowCamera::blank();
                pose_onto_lens(l.steep, &mut c.view, &mut s, &mut fixed);
                assert_eq!(s.target.map(f64::to_bits), want.view.target.map(f64::to_bits), "{name} target");
                s.pos = want.view.pos;
                s.forward = want.view.forward;
                for i in 0..16 {
                    s.view_proj[i] = want.view.vp[i] as f32;
                    s.inv_view_proj[i] = want.view.ivp[i] as f32;
                }
                view_from_lens(&mut c.view, &s, &mut fixed);
            }
            assert_eq!(c.keys, want.keys, "{name} keys");
            assert_eq!(c.pressed, want.pressed, "{name} pressed");
            let (a, b) = (nums(&c), nums(&want));
            assert_eq!(a.len(), b.len(), "{name} length");
            for i in 0..a.len() {
                assert!(same(a[i], b[i]), "{name} number {i}: {} {}", a[i], b[i]);
            }
            assert_eq!(fixed.to_bits(), want_fixed, "{name} fixed");
        }
    }

    /// The swallow's beats against the TypeScript's Seat, signed words included.
    #[test]
    fn swallow_beats_match_the_typescript() {
        let mut s = Seat::blank();
        let mut input = crate::input::Input::blank();
        let base = cases().remove(0).1.unwrap();
        for (i, (load, a, b, mask, buttons)) in beats().into_iter().enumerate() {
            if load {
                let mut c = base.clone();
                c.buttons = a;
                c.clicked = b;
                load_row(&mut s, &mut input, &c);
            } else {
                swallow_row(&mut s, &mut input, a);
            }
            assert_eq!(s.mask.to_bits(), mask.to_bits(), "beat {i} mask");
            assert_eq!(input.buttons.to_bits(), buttons.to_bits(), "beat {i} buttons");
        }
    }

    /// The take of a cut goes by the serial: unchanged is no cut, the same numbers written again are one, and each is taken once.
    #[test]
    fn a_cut_is_taken_once_by_its_serial() {
        let mut s = Seat::blank();
        assert_eq!(cut_row(&mut s), None);
        s.cam.desired_serial += 1;
        assert_eq!(cut_row(&mut s), Some(s.cam.desired));
        assert_eq!(cut_row(&mut s), None);
    }

    /// A number as the language writes it, against the TypeScript's `String(x)`.
    #[test]
    fn numbers_read_as_the_language_writes_them() {
        for (b, want) in number_texts() { assert_eq!(js_number(f(b)), want); }
    }

    /// The record's text both ways, against the TypeScript's cmdText and cmdFromText: words for what JSON cannot carry, escapes, the order of the fields.
    #[test]
    fn a_record_as_text_matches_the_typescript() {
        let c = cmd_from_text(RECORD_TEXT);
        let v = &c.view;
        assert_eq!(c.keys.len(), 2);
        let mut a = vec![c.buttons, c.clicked, c.wheel, c.mx, c.my, c.w, c.h, c.pw, c.ph];
        a.extend(v.vp.iter()); a.extend(v.ivp.iter()); a.extend(v.pos); a.extend(v.forward); a.extend(v.target);
        a.extend([v.yaw, v.pitch, v.distance, v.fov, v.aspect, v.aim_lead]);
        let want = record_numbers();
        assert_eq!(a.len(), want.len());
        for i in 0..a.len() { assert!(same(a[i], f(want[i])), "number {i}: {} {}", a[i], f(want[i])); }
        assert_eq!(cmd_text(&c), RECORD_TEXT);
    }

    fn number_texts() -> Vec<(u64, &'static str)> { vec![(0x0000000000000000,"0"),(0x3ff0000000000000,"1"),(0xbff0000000000000,"-1"),(0x3fb999999999999a,"0.1"),(0x3ff8000000000000,"1.5"),(0x4059000000000000,"100"),(0x4415af1d78b58c40,"100000000000000000000"),(0x444b1ae4d6e2ef50,"1e+21"),(0x4454542ba12a337c,"1.5e+21"),(0x441ac53a7e04bcda,"123456789012345680000"),(0x3eb0c6f7a0b5ed8d,"0.000001"),(0x3e7ad7f29abcaf48,"1e-7"),(0x3e8421f5f40d8376,"1.5e-7"),(0x3eb4b3fd5942cd96,"0.000001234"),(0x0000000000000001,"5e-324"),(0x7fefffffffffffff,"1.7976931348623157e+308"),(0xbe25798ee2308c3a,"-2.5e-9"),(0x3fd3333333333334,"0.30000000000000004"),(0x40d0000000000000,"16384"),(0x41e0000000000000,"2147483648"),(0x41efffffffe00000,"4294967295"),(0x3fd5555555555555,"0.3333333333333333"),(0x400921fb54442d18,"3.141592653589793"),(0xc44b1ae4d6e2ef50,"-1e+21"),(0x40934a456d5cfaad,"1234.5678")] }
    const RECORD_TEXT: &str = "{\"keys\":[\"KeyW\",\"a\\\"b\\\\c\\n\\t\\u0001é\"],\"pressed\":[\"Digit1\"],\"buttons\":5,\"clicked\":\"-0\",\"wheel\":\"nan\",\"mx\":\"inf\",\"my\":\"-inf\",\"w\":1280,\"h\":720,\"pw\":2560.5,\"ph\":1e+21,\"view\":{\"vp\":[1.5,\"-0\",0,\"nan\",1e-7,3],\"ivp\":[0.1],\"pos\":[1,\"-0\",3],\"forward\":[0.1,0.2,0.3],\"target\":[\"inf\",0,-1e-9],\"yaw\":0.25,\"pitch\":0.3333333333333333,\"distance\":21,\"fov\":0.6,\"aspect\":1.7777777777777777,\"aimLead\":0}}";
    fn record_numbers() -> Vec<u64> { vec![0x4014000000000000,0x8000000000000000,0x7ff8000000000000,0x7ff0000000000000,0xfff0000000000000,0x4094000000000000,0x4086800000000000,0x40a4010000000000,0x444b1ae4d6e2ef50,0x3ff8000000000000,0x8000000000000000,0x0000000000000000,0x7ff8000000000000,0x3e7ad7f29abcaf48,0x4008000000000000,0x3fb999999999999a,0x3ff0000000000000,0x8000000000000000,0x4008000000000000,0x3fb999999999999a,0x3fc999999999999a,0x3fd3333333333333,0x7ff0000000000000,0x0000000000000000,0xbe112e0be826d695,0x3fd0000000000000,0x3fd5555555555555,0x4035000000000000,0x3fe3333333333333,0x3ffc71c71c71c71c,0x0000000000000000] }

    fn cases() -> Vec<(&'static str, Option<PlayerCmd>, Option<V3>, PlayerCmd, u64)> { vec![
    ("honest",Some(r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x3ff0000000000000),f(0x4000000000000000),f(0x4008000000000000)],&[f(0x8000000000000000),f(0xbfe999999999999a),f(0x3fe3333333333333)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0x3feeb851eb851eb8),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)])),None,r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x3ff0000000000000),f(0x4000000000000000),f(0x4008000000000000)],&[f(0x8000000000000000),f(0xbfe999999999999a),f(0x3fe3333333333333)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0x3feeb851eb851eb8),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)]),0x0000000000000000),
    ("dirty",Some(r(&["KeyW","bad key","","Ab_c","xxxxxxxxxxxxxxxxxxxxxxxxx","yyyyyyyyyyyyyyyyyyyyyyyy","KeyE"],&["K0","K1","K2","K3","K4","K5","K6","K7","K8","K9","K10","K11","K12","K13","K14","K15","K16","K17","K18","K19","K20","K21","K22","K23","K24","K25","K26","K27","K28","K29"],&[f(0x406ff66666666666),f(0x7ff8000000000000),f(0x41cdcd6500000000),f(0x7ff0000000000000),f(0x7ff8000000000000),f(0x0000000000000000),f(0x41cdcd6500000000),f(0x7ff8000000000000),f(0x8000000000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x40f86a0000000000),f(0xc0f86a0000000000),f(0x7ff8000000000000)],&[f(0x4000000000000000),f(0xc000000000000000),f(0x8000000000000000)],&[f(0x8000000000000000),f(0x7ff0000000000000),f(0x4008000000000000)],&[f(0x40d3880000000000),f(0x4010000000000000),f(0x0000000000000000),f(0x4022000000000000),f(0x7ff8000000000000),f(0x4008000000000000)])),None,r(&["KeyW","yyyyyyyyyyyyyyyyyyyyyyyy","KeyE"],&["K0","K1","K2","K3","K4","K5","K6","K7","K8","K9","K10","K11","K12","K13","K14","K15","K16","K17","K18","K19","K20","K21","K22","K23"],&[f(0x403f000000000000),f(0x0000000000000000),f(0x40b0000000000000),f(0x40e0000000000000),f(0x40c0000000000000),f(0x3ff0000000000000),f(0x40d0000000000000),f(0x3ff0000000000000),f(0x3ff0000000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x40c3880000000000),f(0xc0c3880000000000),f(0x0000000000000000)],&[f(0x3ff0000000000000),f(0xbff0000000000000),f(0x8000000000000000)],&[f(0x8000000000000000),f(0x40c3880000000000),f(0x4008000000000000)],&[f(0x40c3880000000000),f(0x400921fb54442d18),f(0x3fa999999999999a),f(0x4008000000000000),f(0x3f10000000000000),f(0x3ff0000000000000)]),0x4037000000000000),
    ("many",Some(r(&["bad!","K1","bad!","K3","bad!","K5","bad!","K7","bad!","K9","bad!","K11","bad!","K13","bad!","K15","bad!","K17","bad!","K19","bad!","K21","bad!","K23","bad!","K25","bad!","K27","bad!","K29","bad!","K31","bad!","K33","bad!","K35","bad!","K37","bad!","K39","bad!","K41","bad!","K43","bad!","K45","bad!","K47","bad!","K49","bad!","K51","bad!","K53","bad!","K55","bad!","K57","bad!","K59","bad!","K61","bad!","K63","bad!","K65","bad!","K67","bad!","K69","bad!","K71","bad!","K73","bad!","K75","bad!","K77","bad!","K79","bad!","K81","bad!","K83","bad!","K85","bad!","K87","bad!","K89","bad!","K91","bad!","K93","bad!","K95","bad!","K97","bad!","K99","bad!","K101","bad!","K103","bad!","K105","bad!","K107","bad!","K109","bad!","K111","bad!","K113","bad!","K115","bad!","K117","bad!","K119","bad!","K121","bad!","K123","bad!","K125","bad!","K127","bad!","K129","bad!","K131","bad!","K133","bad!","K135","bad!","K137","bad!","K139","bad!","K141","bad!","K143","bad!","K145","bad!","K147","bad!","K149","bad!","K151","bad!","K153","bad!","K155","bad!","K157","bad!","K159","bad!","K161","bad!","K163","bad!","K165","bad!","K167","bad!","K169","bad!","K171","bad!","K173","bad!","K175","bad!","K177","bad!","K179","bad!","K181","bad!","K183","bad!","K185","bad!","K187","bad!","K189","bad!","K191","bad!","K193","bad!","K195","bad!","K197","bad!","K199"],&[],&[f(0x41f0000000500000),f(0x8000000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x3ff0000000000000),f(0x4000000000000000),f(0x4008000000000000)],&[f(0x8000000000000000),f(0xbfe999999999999a),f(0x3fe3333333333333)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0x3feeb851eb851eb8),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)])),None,r(&["K1","K3","K5","K7","K9","K11","K13","K15","K17","K19","K21","K23","K25","K27","K29","K31","K33","K35","K37","K39","K41","K43","K45","K47"],&[],&[f(0x4014000000000000),f(0x0000000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x3ff0000000000000),f(0x4000000000000000),f(0x4008000000000000)],&[f(0x8000000000000000),f(0xbfe999999999999a),f(0x3fe3333333333333)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0x3feeb851eb851eb8),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)]),0x4008000000000000),
    ("wire",Some(r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x7ff8000000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x7ff8000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x3ff0000000000000),f(0x4000000000000000),f(0x4008000000000000)],&[f(0x8000000000000000),f(0xbfe999999999999a),f(0x3fe3333333333333)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0x3feeb851eb851eb8),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x7ff8000000000000),f(0x3fc70a3d70a3d70a)])),None,r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x4084000000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x4000c927e0000000),f(0x3ff38725a0000000),f(0x3fcb57e9c0000000),f(0x3fcb3abf20000000),f(0x0000000000000000),f(0x40026de500000000),f(0xbfea52e580000000),f(0xbfea36d140000000),f(0x3fead15ae0000000),f(0xc008721f20000000),f(0xbfe11d60e0000000),f(0xbfe10b1f60000000),f(0xc00aea9740000000),f(0xc0373760c0000000),f(0x403085b3e0000000),f(0x4030f41420000000)],&[f(0x3fda4e35a0000000),f(0x3e24cfbce0000000),f(0x3fc5037540000000),f(0x3df851eaa0000000),f(0x3fb35da860000000),f(0x3fc246bc40000000),f(0xbfc83e2f60000000),f(0xbe3bd82800000000),f(0x3fedc8f1a0000000),f(0xc04121a020000000),f(0xc024a713e0000000),f(0xbfffdddea0000000),f(0xbfe71a2940000000),f(0x4040cb1e60000000),f(0x4023ac8120000000),f(0x4000000060000000)],&[f(0xbfdde8d6ba775df0),f(0x403133f94f9f3f1a),f(0x4014bd325478e71a)],&[f(0x3fcb3abf1df2fcc2),f(0xbfea36d13c606028),f(0xbfe10b1f5d21de22)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0x3feeb851eb851eb8),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc71c71c71c71c),f(0x3fc70a3d70a3d70a)]),0x4018000000000000),
    ("remote",Some(r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x3ff0000000000000),f(0x4000000000000000),f(0x4008000000000000)],&[f(0x8000000000000000),f(0xbfe999999999999a),f(0x3fe3333333333333)],&[f(0x407f400000000000),f(0x4008000000000000),f(0x4079000000000000)],&[f(0xbfd851eb851eb852),f(0x3ff8cccccccccccd),f(0x4079000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)])),Some([f(0x4059000000000000),f(0x0000000000000000),f(0xc049000000000000)]),r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x40010d5d40000000),f(0x3ff7c74000000000),f(0x3f9afae160000000),f(0x3f9ade1a00000000),f(0x0000000000000000),f(0x3fd22f30e0000000),f(0xbff006d5c0000000),f(0xbfefeb7aa0000000),f(0x3feb3e5420000000),f(0xc00dc44040000000),f(0xbfb0e325c0000000),f(0xbfb0d12260000000),f(0xc074ec6c00000000),f(0xc067e8f380000000),f(0x4063a5ade0000000),f(0x4063a0b8e0000000)],&[f(0x3fd9e4fce0000000),f(0x3e5076fd00000000),f(0x3fc4af6760000000),f(0x3dd9575d00000000),f(0x3fb794b5c0000000),f(0x3f920877a0000000),f(0xbfcd850260000000),f(0xbe221cf4c0000000),f(0xc0728a7280000000),f(0xc073ea7bc0000000),f(0xc04437b140000000),f(0xbfffddaf80000000),f(0x40729eba20000000),f(0x4073efdb40000000),f(0x404444f0a0000000),f(0x3fffffd1a0000000)],&[f(0x40629e69925a33c9),f(0x4063ffee11c7370e),f(0x40344d763a7397e2)],&[f(0x3f9ade1a0e29af70),f(0xbfefeb7a9b2c6d8b),f(0xbfb0d12267aa4ffd)],&[f(0x406324c014a10436),f(0x3fd982e196e52ee3),f(0x4023958173524bc8)],&[f(0xbfd851eb851eb852),f(0x3ff8000000000000),f(0x4064000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)]),0x401c000000000000),
    ("remoteNear",Some(r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x3ff8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4002000000000000),f(0xbfe0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fe0000000000000),f(0x39b4484bfeebc2a0),f(0xbff0000000000000),f(0x4008000000000000),f(0xc010000000000000),f(0x4014000000000000),f(0x4018000000000000)],&[f(0x3fe8000000000000),f(0x8000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ff2000000000000),f(0xbfd0000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3fd0000000000000),f(0x39a4484bfeebc2a0),f(0xbfe0000000000000),f(0x3ff8000000000000),f(0xc000000000000000),f(0x4004000000000000),f(0x4008000000000000)],&[f(0x3ff0000000000000),f(0x4000000000000000),f(0x4008000000000000)],&[f(0x8000000000000000),f(0xbfe999999999999a),f(0x3fe3333333333333)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0xbff8cccccccccccd),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)])),Some([f(0x4010000000000000),f(0x3ff0000000000000),f(0xc018000000000000)]),r(&["KeyW","ShiftLeft"],&["Digit1"],&[f(0x4014000000000000),f(0x3ff0000000000000),f(0xc05e000000000000),f(0x4084040000000000),f(0x4076840000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x40a4000000000000),f(0x4096800000000000)],&[f(0x40010d5d40000000),f(0xbff7c74000000000),f(0x3f9afae160000000),f(0x3f9ade1a00000000),f(0x0000000000000000),f(0x3fd22f30e0000000),f(0x3ff006d5c0000000),f(0x3fefeb7aa0000000),f(0x3feb3e5420000000),f(0x400dc44040000000),f(0xbfb0e325c0000000),f(0xbfb0d12260000000),f(0xc00b57f720000000),f(0x403c450040000000),f(0x403415a7a0000000),f(0x4034803b20000000)],&[f(0x3fd9e4fce0000000),f(0xbe259a4b40000000),f(0x3fc4af6760000000),f(0x3dd9578000000000),f(0xbfb794bae0000000),f(0x3f92088b60000000),f(0x3fcd8501e0000000),f(0x3e221d0dc0000000),f(0xc01b7a1f60000000),f(0x4044dc2ee0000000),f(0x402267a8a0000000),f(0xbfffdddb80000000),f(0x401bb26c20000000),f(0xc04472d920000000),f(0xc0229d01c0000000),f(0x3ffffffde0000000)],&[f(0x400b978fb9ad2938),f(0xc034f28875d527e3),f(0xc0127b60b5fc1dc1)],&[f(0x3f9ade1a0e29af70),f(0x3fefeb7a9b2c6d8b),f(0xbfb0d12267aa4ffd)],&[f(0x4010000000000000),f(0x0000000000000000),f(0xc018000000000000)],&[f(0xbfd851eb851eb852),f(0xbff8000000000000),f(0x4035000000000000),f(0x3fdf3b645a1cac08),f(0x3ffc000000000000),f(0x3fc70a3d70a3d70a)]),0x4014000000000000),
    ("null",None,None,r(&[],&[],&[f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4084000000000000),f(0x4076800000000000),f(0x4094000000000000),f(0x4086800000000000),f(0x4094000000000000),f(0x4086800000000000)],&[f(0x3ffd1834a0000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x3ffdaa1fe0000000),f(0xbfea52e580000000),f(0xbfea36d140000000),f(0x0000000000000000),f(0xc0052f8fe0000000),f(0xbfe26def20000000),f(0xbfe25a46a0000000),f(0x0000000000000000),f(0x0000000000000000),f(0x403495f560000000),f(0x4035000000000000)],&[f(0x3fe1990920000000),f(0x8000000000000000),f(0x8000000000000000),f(0x8000000000000000),f(0x8000000000000000),f(0x3fc6b56260000000),f(0xbfd037cca0000000),f(0xbe3dbe3f40000000),f(0x0000000000000000),f(0xc04121a080000000),f(0xc037fccc20000000),f(0xbfffdddf40000000),f(0x8000000000000000),f(0x4040cb1ec0000000),f(0x403783aba0000000),f(0x40000000c0000000)],&[f(0x0000000000000000),f(0x403133f94f9f3f1a),f(0x4028167ca4d6e905)],&[f(0x8000000000000000),f(0xbfea36d13c606028),f(0xbfe25a4695f91310)],&[f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000)],&[f(0x0000000000000000),f(0x3feeb851eb851eb8),f(0x4035000000000000),f(0x3fe3333333333333),f(0x3ffc71c71c71c71c),f(0x0000000000000000)]),0x403c000000000000),
    ] }
    fn beats() -> Vec<(bool,f64,f64,f64,f64)> { vec![(true,f(0x4014000000000000),f(0x3ff0000000000000),f(0x0000000000000000),f(0x4014000000000000)),(false,f(0x3ff0000000000000),f(0x0000000000000000),f(0x3ff0000000000000),f(0x4010000000000000)),(true,f(0x4014000000000000),f(0x0000000000000000),f(0x3ff0000000000000),f(0x4010000000000000)),(true,f(0x4010000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x4010000000000000)),(true,f(0x4014000000000000),f(0x3ff0000000000000),f(0x0000000000000000),f(0x4014000000000000)),(false,f(0x4014000000000000),f(0x0000000000000000),f(0x4014000000000000),f(0x0000000000000000)),(true,f(0x4014000000000000),f(0x0000000000000000),f(0x4014000000000000),f(0x0000000000000000)),(true,f(0x4014000000000000),f(0x4010000000000000),f(0x3ff0000000000000),f(0x4010000000000000)),(true,f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000),f(0x0000000000000000)),(false,f(0x41e0000000000000),f(0x0000000000000000),f(0xc1e0000000000000),f(0x0000000000000000)),(true,f(0x41efffffffe00000),f(0x0000000000000000),f(0xc1e0000000000000),f(0x41dfffffffc00000))] }
}
