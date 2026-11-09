//! A HAND: a small file of what is done to the game from outside, a line each (`hands/*.hand`; the head of `src/bin/hold.rs` is its grammar). What plays one is `hold` and the gate.
use crate::play::{dist, Button, Play};
use game::game::GuardState;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Place { Nearest, Ahead, Door, OpenDoor, Sam, At(f64, f64) }
#[derive(Clone, Debug)]
pub enum What { Order(String, Place, game::orders::Who), Fire(Place), Keys(&'static [&'static str]), Can(game::game::ThrowKind, Place), KeyDown(&'static str), KeyUp(&'static str), Point(Place), Press, Release }
/// `at`: (the frame, what is done, which line of the file it is of); `lines`: the file's lines that do something, as written
pub struct Hand { pub unhurt: bool, pub at: Vec<(usize, What, usize)>, pub lines: Vec<String> }

pub fn hands_dir() -> std::path::PathBuf { std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hands") }

/// a hand's file read for a run of `frames` frames: every line as the frames it happens on, in the file's order within a frame
pub fn read_hand(text: &str, frames: usize) -> Hand {
    let mut h = Hand { unhurt: false, at: Vec::new(), lines: Vec::new() };
    let place = |w: &str| -> Place {
        match w { "nearest" => Place::Nearest, "ahead" => Place::Ahead, "door" => Place::Door, "opendoor" => Place::OpenDoor, "sam" => Place::Sam, xz => { let mut p = xz.split(',').map(|v| v.parse::<f64>().unwrap_or_else(|_| panic!("a place is nearest, door, sam or X,Z: '{}'", xz))); Place::At(p.next().unwrap(), p.next().expect("X,Z")) } }
    };
    for (n, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() { continue; }
        let w: Vec<&str> = line.split_whitespace().collect();
        let bad = || -> ! { panic!("line {} of the hand is not one this tool knows: '{}'", n + 1, line) };
        if w == ["unhurt"] { h.unhurt = true; continue; }
        let secs = |v: &str| -> usize { (v.parse::<f64>().unwrap_or_else(|_| bad()) * 60.0).round() as usize };
        let (first, step, rest) = match w[0] {
            "at" if w.len() > 2 => (secs(w[1]), 0, &w[2..]),
            "every" if w.len() > 4 && w[2] == "from" => (secs(w[3]), secs(w[1]).max(1), &w[4..]),
            _ => bad(),
        };
        let what = match rest {
            ["order", id, at, who] => What::Order(id.to_string(), place(at), match *who { "all" => game::orders::Who::All, "red" => game::orders::Who::Red, "blue" => game::orders::Who::Blue, _ => bad() }),
            ["fire", at] => What::Fire(place(at)),
            ["frag", at] => What::Can(game::game::ThrowKind::Frag, place(at)),
            ["flash", at] => What::Can(game::game::ThrowKind::Flash, place(at)),
            // (the mission again by the keys: Enter, which does something once he is down or the mission has failed; Shift+Enter, at any time)
            ["enter"] => What::Keys(&["Enter"]),
            ["again"] => What::Keys(&["ShiftLeft", "Enter"]),
            _ => bad(),
        };
        h.lines.push(rest.join(" "));
        let of = h.lines.len() - 1;
        let mut f = first;
        while f < frames {
            match &what {
                // (the rifle comes up over 0.8 s, the cursor rests 0.3 s on the place, the trigger is down two frames)
                What::Fire(p) => for (d, x) in [(0, What::KeyDown("Digit2")), (6, What::KeyUp("Digit2")), (48, What::Point(*p)), (66, What::Press), (68, What::Release)] { h.at.push((f + d, x, of)); },
                What::Keys(ks) => { for k in ks.iter() { h.at.push((f, What::KeyDown(k), of)); } for k in ks.iter() { h.at.push((f + 2, What::KeyUp(k), of)); } }
                x => h.at.push((f, x.clone(), of)),
            }
            if step == 0 { break; }
            f += step;
        }
    }
    h.at.sort_by_key(|x| x.0);   // (a stable sort: lines of one frame stay in the file's order)
    h
}

/// the men standing who are not of Sam's squad, nearest to Sam first
pub fn others(p: &Play) -> Vec<game::ids::GuardId> {
    let sam = p.sam_at();
    let mut v: Vec<game::ids::GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative && p.g.gd[gd].state != GuardState::Dead && p.g.gd[gd].down_kind.is_none()).collect();
    v.sort_by(|x, y| dist(p.at(*x), sam).partial_cmp(&dist(p.at(*y), sam)).unwrap());
    v
}
pub fn place_of(p: &Play, at: Place) -> Option<[f64; 2]> {
    match at {
        Place::Sam => Some(p.sam_at()),
        Place::At(x, z) => Some([x, z]),
        Place::Nearest => others(p).first().map(|&m| p.at(m)),
        // (5 metres on from the squad's first man toward the nearest man: a place a man can throw at, and clear of the squad, whichever way the fight has gone)
        Place::Ahead => {
            let (him, from) = (p.at(*others(p).first()?), p.at(*p.mates().first()?));
            let s = (5.0 / dist(from, him).max(1e-6)).min(1.0);
            Some([from[0] + (him[0] - from[0]) * s, from[1] + (him[1] - from[1]) * s])
        }
        Place::Door | Place::OpenDoor => {
            // (the way to the nearest man; with nobody left, the door nearest Sam: a door is still worked)
            let him = others(p).first().map(|&m| p.at(m)).unwrap_or_else(|| p.sam_at());
            let mut doors: Vec<[f64; 2]> = p.g.doors.list.iter().filter(|&&d| at != Place::OpenDoor || game::doortools::is_open(&p.g, d)).map(|&d| { let c = p.g.dr[d].frame_centre; [c[0], c[2]] }).collect();
            doors.sort_by(|a, b| dist(*a, him).partial_cmp(&dist(*b, him)).unwrap());
            doors.first().copied()
        }
    }
}

/// one line of a hand done to the game; Err says why it could not be (the game's own refusal of an order, or nobody to do it to)
pub fn act(p: &mut Play, what: &What) -> Result<(), String> {
    let nobody = || String::from("nobody is left");
    match what {
        What::Order(id, at, who) => {
            let at = place_of(p, *at).ok_or_else(nobody)?;
            let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
            match game::orders::squad_orders_give(&mut p.g, id, Some(&t), Some(who.clone()), None) { Some(why) => Err(why), None => Ok(()) }
        }
        What::Can(kind, at) => { let at = place_of(p, *at).ok_or_else(nobody)?; game::game::game_drop_throwable(&mut p.g, *kind, [at[0], 0.0, at[1]]); Ok(()) }
        What::KeyDown(k) => { p.key_down(k); Ok(()) }
        What::KeyUp(k) => { p.key_up(k); Ok(()) }
        What::Point(at) => { let at = place_of(p, *at).ok_or_else(nobody)?; p.point_at(at); Ok(()) }
        What::Press => { p.press(Button::Left); Ok(()) }
        What::Release => { p.release(Button::Left); Ok(()) }
        What::Fire(_) | What::Keys(_) => unreachable!("read_hand writes these out as their keys"),
    }
}
