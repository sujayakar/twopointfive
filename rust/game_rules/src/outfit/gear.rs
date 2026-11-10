pub use ::game_types::gear::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use alloc::string::String;

/// A weapon row's short name as it is written down: the two renamed rows by their old names, every other row its own.
pub fn plain_short(def: &crate::weapons::WeaponDef) -> String {
    if def.id == crate::weapons::WeaponId::Fn2000 || def.id == crate::weapons::WeaponId::Mk18bare {
        return crate::gear::old_names().rifle;
    }
    if def.id == crate::weapons::WeaponId::Fiveseven || def.id == crate::weapons::WeaponId::G19bare {
        return crate::gear::old_names().pistol;
    }
    def.short.clone()
}

/// The launcher's sponge round, in the kit's words.
pub(super) fn ring_tag() -> &'static str { "SPONGE" }

/// The launcher's high-explosive round, in the kit's words.
pub(super) fn shell_tag() -> &'static str { "HE" }

/// A line written in the old kit's words, in the kit's words as issued. `GEAR.on` is settled on.
pub fn gear_text(s: &str) -> String {
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
    start == 0 || !crate::js::is_word(s.as_bytes()[start - 1])
}

/// `\b…\b` around a run of word characters.
fn whole_word(s: &str, start: usize, end: usize) -> bool {
    word_starts_here(s, start, end) && s.as_bytes().get(end).map_or(true, |b| !crate::js::is_word(*b))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A FAILURE MEANS: a player reads a gun the kit no longer has, "ring airfoil", or a word cut in two: lines are written in the old kit's words and said in the new.
    #[test]
    fn a_line_in_the_old_words_reads_in_the_words_of_the_kit_as_issued() {
        for (old, issued) in [
            ("FN2000 and FN2000, Five-seveN", "Mk 18 and Mk 18, Glock 19"),
            ("2 again for the 40 mm", "2 again for the 40 mm HE"),
            ("2 again for the 40 mm HE", "2 again for the 40 mm HE"),
            ("a ring airfoil and 2 ring airfoils off a ring launcher", "a 40 mm sponge round and 2 40 mm sponge rounds off a 40 mm launcher"),
            ("a ring to the head", "a sponge round to the head"),
            ("spring to the head", "spring to the head"),
            ("none for the launcher: rings only", "none for the launcher: sponge rounds only"),
            ("RING (RING) BRING", "SPONGE (SPONGE) BRING"),
            ("plain ×3 — words", "plain ×3 — words"),
        ] {
            assert_eq!(issued_words(old), issued);
        }
    }

    /// A FAILURE MEANS: the words of a renamed gun are not the words every line of the old kit is matched by, so its row keeps its old name on the card.
    #[test]
    fn plain_short_writes_the_old_names_down() {
        let row = |id: crate::weapons::WeaponId| plain_short(&crate::weapons::WeaponDef { id, short: String::from("own"), ..crate::weapons::WeaponDef::default() });
        assert_eq!(row(crate::weapons::WeaponId::Fn2000), "FN2000");
        assert_eq!(row(crate::weapons::WeaponId::Mk18bare), "FN2000");
        assert_eq!(row(crate::weapons::WeaponId::Fiveseven), "Five-seveN");
        assert_eq!(row(crate::weapons::WeaponId::G19bare), "Five-seveN");
        assert_eq!(row(crate::weapons::WeaponId::M870), "own");
    }
}
