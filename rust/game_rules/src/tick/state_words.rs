//! THE STATE'S WORDS: a hash of the whole game, a word a part, for two machines of one game to be held to each other. A part is a field of the game (of its modules, its
//! tables, a seat's set); what is in no word says so beside itself (`#[bytes(skip)]`): what a host hands in, a step's output, the picture's.
use crate::bytes::{Parts, W};
use crate::game::{Game, Mods, Tabs};
use crate::host_api::HostApi;
use crate::seats::SeatSet;
use alloc::{format, string::String, vec, vec::Vec};

/// the game's own parts: those that are walked by their own fields, and a seat's set's (which lies in its seat's row), stand elsewhere in the list
fn own(name: &str) -> bool { !matches!(name, "m" | "engine" | "tab") && !SeatSet::NAMES.contains(&name) }

/// each word's part, in the words' order
pub fn state_names() -> Vec<String> {
    let of = |at: &str, names: &[&str]| names.iter().map(|n| format!("{at}{n}")).collect::<Vec<_>>();
    let mut v: Vec<String> = Game::NAMES.iter().filter(|n| own(n)).map(|n| String::from(*n)).collect();
    v.extend(of("m.", Mods::NAMES));
    v.extend(of("engine.", HostApi::NAMES));
    v.extend(of("tab.", Tabs::NAMES));
    v.extend(of("seats.", SeatSet::NAMES));
    v
}

/// `o` and `out` are the caller's to keep, and reach their size once. With the first seat standing, as inside a tick: which seat a machine shows moves no word
pub fn state_words(g: &mut Game, o: &mut W, out: &mut Vec<u32>) {
    out.clear();
    crate::seats::on_first_bare(g, &mut |g| crate::seats::parked(g, &mut |g| {
        let mut say = |x: &dyn crate::bytes::ToBytes| { o.b.clear(); x.w(o); word(&o.b) };
        g.parts(&mut |name, x| if own(name) { out.push(say(x)); });
        g.m.parts(&mut |_, x| out.push(say(x)));
        g.engine.parts(&mut |_, x| out.push(say(x)));
        g.tab.parts(&mut |_, x| out.push(say(x)));
        // (a part of the sets is one word, whoever many seats there are)
        let at = out.len();
        out.resize(at + SeatSet::NAMES.len(), 0);
        for s in g.tab.cmd_seat.ids() {
            let mut k = at;
            if let Some(set) = &g.tab.cmd_seat[s].own { set.parts(&mut |_, x| { out[k] = out[k].rotate_left(5) ^ say(x); k += 1; }); }
        }
        o.b.clear();
    }));
}

/// eight bytes at a time, how many there are first
fn word(b: &[u8]) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325 ^ b.len() as u64;
    let mut eights = b.chunks_exact(8);
    for x in &mut eights {
        h = (h ^ u64::from_le_bytes(x.try_into().unwrap())).wrapping_mul(0x100000001b3);
        h ^= h >> 29;
    }
    for x in eights.remainder() { h = (h ^ *x as u64).wrapping_mul(0x100000001b3); }
    (h ^ (h >> 32)) as u32
}
