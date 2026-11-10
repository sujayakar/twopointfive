//! THE TICK BY ITS STEPS (game::game::STEPS): what each costs, and which parts of the game it writes. `steps.txt` says what each writes, made by `hold steps DIR --write` off the kept runs;
//! the gate's `steps` rows hold the tick to it.
use crate::parts;
use crate::play::Play;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use std::time::Instant;

pub struct Step { pub name: &'static str, pub us: f64, pub worst: f64, pub parts: Vec<bool> }

pub struct Steps {
    pub names: Vec<String>,
    pub steps: Vec<Step>,
    w: game::bytes::W,
    was: Vec<u32>,
    now: Vec<u32>,
    timed: usize,
}

impl Steps {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Steps { Steps { names: parts::names(), steps: Vec::new(), w: game::bytes::W::new(), was: Vec::new(), now: Vec::new(), timed: 0 } }

    /// A frame of `p`, timed, or with every step asked what it wrote (which is slow, and is not timed).
    pub fn frame(&mut self, p: &mut Play, ask: bool) {
        let Steps { names, steps, w, was, now, timed } = self;
        was.clear();
        if ask { parts::words(&mut p.g, w, was); } else { *timed += 1; }
        let mut t = Instant::now();
        let mut at = 0;
        p.frame_watched(&mut |g, name| {
            let us = t.elapsed().as_secs_f64() * 1e6;
            // (the rows come in one order every tick, a seat's rows once a seat)
            let i = match steps.iter().skip(at).position(|s| s.name == name) {
                Some(i) => at + i,
                None => steps.iter().position(|s| s.name == name).unwrap_or_else(|| { steps.push(Step { name, us: 0.0, worst: 0.0, parts: vec![false; names.len()] }); steps.len() - 1 }),
            };
            at = i;
            let s = &mut steps[i];
            if ask {
                now.clear();
                parts::words(g, w, now);
                for (seen, (a, b)) in s.parts.iter_mut().zip(was.iter().zip(now.iter())) { *seen |= a != b; }
                core::mem::swap(was, now);
            } else {
                s.us += us;
                s.worst = s.worst.max(us);
            }
            t = Instant::now();
        }, |_| ());
    }

    /// microseconds a frame
    pub fn mean(&self, s: &Step) -> f64 { s.us / self.timed.max(1) as f64 }
    /// the parts a step was seen to write
    pub fn wrote<'a>(&'a self, s: &'a Step) -> impl Iterator<Item = &'a str> { self.names.iter().zip(&s.parts).filter(|(_, x)| **x).map(|(n, _)| n.as_str()) }
    /// what was written that the file does not say
    pub fn unsaid(&self, said: &BTreeMap<String, Vec<String>>) -> Vec<String> {
        let mut out = Vec::new();
        for s in &self.steps {
            let row = said.get(s.name);
            for part in self.wrote(s).filter(|part| !row.is_some_and(|r| r.iter().any(|x| x == part))) { out.push(format!("'{}' wrote {}", s.name, part)); }
        }
        out
    }
}

const HEAD: &str = "# WHAT EACH STEP OF THE TICK WRITES, in the tick's order (STEPS in game_rules/src/tick/game/steps.rs): the parts of the game it was seen to change. Made by `hold steps DIR --write`.\n# A step that writes another part is a red row of the gate: a new coupling, to be meant and added here, or not made.\n";
pub fn path() -> std::path::PathBuf { std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("steps.txt") }
pub fn read_file() -> BTreeMap<String, Vec<String>> {
    std::fs::read_to_string(path()).unwrap_or_default().lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.split_once(':')).map(|(n, p)| (String::from(n), p.split_whitespace().map(String::from).collect())).collect()
}
pub fn write_file(rows: &[(String, Vec<String>)]) {
    let text: String = rows.iter().map(|(n, p)| if p.is_empty() { format!("{n}:\n") } else { format!("{n}: {}\n", p.join(" ")) }).collect();
    std::fs::write(path(), String::from(HEAD) + &text).unwrap();
}
