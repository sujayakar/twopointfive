use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A weapon row's short name as it is written down: the two renamed rows by their old names, every other row its own.   (gear.ts plainShort)
pub fn plain_short(def: &crate::gear::PlainShortDef) -> String {
    if def.id == "fn2000" || def.id == "mk18bare" {
        return crate::gear::old_names().rifle;
    }
    if def.id == "fiveseven" || def.id == "g19bare" {
        return crate::gear::old_names().pistol;
    }
    def.short.clone()
}

/// The page's rule for the switch. `GEAR.on` is settled on, so `?oldgear` is not read and `q` is unused.   (gear.ts gearFromQuery)
pub fn gear_from_query(g: &mut crate::game::Game, q: crate::Opaque) {
    g.k.gear.on = true;
}

/// The HUD tag of the rifle's first attachment. `GEAR.on` is settled on.   (gear.ts ringTag)
pub fn ring_tag(g: &mut crate::game::Game) -> String {
    String::from("SPONGE")
}

/// The HUD tag of the rifle's second attachment, the 40 mm HE round. `GEAR.on` is settled on.   (gear.ts shellTag)
pub fn shell_tag(g: &mut crate::game::Game) -> String {
    String::from("HE")
}

/// A line written in the old kit's words, in the kit's words as issued. `GEAR.on` is settled on.   (gear.ts gearText)
pub fn gear_text(g: &mut crate::game::Game, s: &str) -> String {
    issued_words(s)
}

/// The eight replacements of gearText, in its order; each is one pass over the result of the one before.
fn issued_words(s: &str) -> String {
    let names = crate::gear::gear_names();
    let s = s.replace("FN2000", &names.rifle);
    let s = s.replace("Five-seveN", &names.pistol);
    let s = replace_where(&s, "2 again for the 40 mm", "2 again for the 40 mm HE", not_followed_by_he);
    let s = s.replace("ring airfoils", "40 mm sponge rounds");
    let s = s.replace("ring airfoil", "40 mm sponge round");
    let s = s.replace("ring launcher", "40 mm launcher");
    let s = replace_where(&s, "ring to the head", "sponge round to the head", word_starts_here);
    let s = s.replace("rings only", "sponge rounds only");
    replace_where(&s, "RING", "SPONGE", whole_word)
}

/// A regex's `/g` over one literal with a condition around it: leftmost matches, no overlap, a refused place moves on by one.
/// `pat` starts with an ASCII byte, so one past a refused place is always a char boundary.
fn replace_where(s: &str, pat: &str, by: &str, ok: fn(&str, usize, usize) -> bool) -> String {
    let mut out = String::with_capacity(s.len());
    let (mut copied, mut from) = (0, 0);
    while let Some(found) = s[from..].find(pat) {
        let (start, end) = (from + found, from + found + pat.len());
        if ok(s, start, end) {
            out.push_str(&s[copied..start]);
            out.push_str(by);
            copied = end;
            from = end;
        } else {
            from = start + 1;
        }
    }
    out.push_str(&s[copied..]);
    out
}

/// `(?! HE)`
fn not_followed_by_he(s: &str, _start: usize, end: usize) -> bool {
    !s[end..].starts_with(" HE")
}

/// `\b` before a word character. JavaScript's `\w` without the `u` flag is ASCII, and no byte of a multi-byte character is ASCII.
fn word_starts_here(s: &str, start: usize, _end: usize) -> bool {
    start == 0 || !is_word_byte(s.as_bytes()[start - 1])
}

/// `\b…\b` around a run of word characters.
fn whole_word(s: &str, start: usize, end: usize) -> bool {
    word_starts_here(s, start, end) && s.as_bytes().get(end).map_or(true, |b| !is_word_byte(*b))
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each pair: what the TypeScript's gearText returned under bun with GEAR.on true.
    #[test]
    fn issued_words_match_the_typescript() {
        let pairs = [
            ("FN2000 and FN2000, Five-seveN", "Mk 18 and Mk 18, Glock 19"),
            ("2 again for the 40 mm", "2 again for the 40 mm HE"),
            ("2 again for the 40 mm HE", "2 again for the 40 mm HE"),
            ("2 again for the 40 mm, 2 again for the 40 mm HE, 2 again for the 40 mmX", "2 again for the 40 mm HE, 2 again for the 40 mm HE, 2 again for the 40 mm HEX"),
            ("a ring airfoil and 2 ring airfoils off a ring launcher", "a 40 mm sponge round and 2 40 mm sponge rounds off a 40 mm launcher"),
            ("spring airfoil", "sp40 mm sponge round"),
            ("ring to the head; a ring to the head; spring to the head; _ring to the head; é ring to the head; éring to the head", "sponge round to the head; a sponge round to the head; spring to the head; _ring to the head; é sponge round to the head; ésponge round to the head"),
            ("rings only · brings only", "sponge rounds only · bsponge rounds only"),
            ("RING RINGS BRING RING. (RING) RING_ é RINGé 1RING RING", "SPONGE RINGS BRING SPONGE. (SPONGE) RING_ é SPONGEé 1RING SPONGE"),
            ("plain ×3 — words", "plain ×3 — words"),
            ("", ""),
        ];
        for (input, want) in pairs {
            assert_eq!(issued_words(input), want);
        }
    }

    #[test]
    fn plain_short_writes_the_old_names_down() {
        let row = |id: &str| plain_short(&crate::gear::PlainShortDef { id: String::from(id), short: String::from("own") });
        assert_eq!(row("fn2000"), "FN2000");
        assert_eq!(row("mk18bare"), "FN2000");
        assert_eq!(row("fiveseven"), "Five-seveN");
        assert_eq!(row("g19bare"), "Five-seveN");
        assert_eq!(row("shotgun"), "own");
    }
}
