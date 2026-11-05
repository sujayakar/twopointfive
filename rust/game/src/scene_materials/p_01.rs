//! src/scene/materials.ts lines 1 to 147
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The columns of a word; the default row's for a word the table does not have.   (materials.ts doesOfWord)
pub fn does_of_word(g: &mut crate::game::Game, w: Option<&str>) -> crate::scene_materials::Does {
    match w.and_then(|w| g.m.scene_materials.rows.get(String::from(w))) {
        Some(d) => d.clone(),
        None => crate::scene_materials::default(),
    }
}

/// What a box does: its own word's row, else the default row.   (materials.ts doesOf)
pub fn does_of(g: &mut crate::game::Game, b: Option<&crate::scene_materials::DoesOfB>) -> crate::scene_materials::Does {
    does_of_word(g, b.and_then(|b| b.does.as_deref()))
}

/// What a box does while soaked: the row its `wet` word names, else its own.   (materials.ts doesWet)
pub fn does_wet(g: &mut crate::game::Game, b: Option<&crate::scene_materials::DoesWetB>) -> crate::scene_materials::Does {
    let d = does_of_word(g, b.and_then(|b| b.does.as_deref()));
    match d.wet.as_deref() {
        Some(w) => does_of_word(g, Some(w)),
        None => d,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(w: &str) -> DoesOfB { DoesOfB { does: Some(String::from(w)) } }

    #[test]
    fn nothing_said_or_unknown_takes_the_default_row() {
        let mut g = Game::blank();
        for d in [does_of_word(&mut g, None), does_of_word(&mut g, Some("")), does_of_word(&mut g, Some("mud wall")), does_of(&mut g, None), does_of(&mut g, Some(&DoesOfB { does: None }))] {
            assert_eq!((d.surf, d.breaks, d.casing), (Surf::Concrete, BreakClass::Metal, None));
        }
    }

    #[test]
    fn a_word_finds_its_own_row() {
        let mut g = Game::blank();
        let d = does_of(&mut g, Some(&word("wood")));
        assert_eq!((d.surf, d.breaks, d.cover), (Surf::Wood, BreakClass::Wood, Some(DoesCover::Wood)));
    }

    #[test]
    fn soaked_takes_the_wet_word_and_is_otherwise_itself() {
        let mut g = Game::blank();
        let d = does_of(&mut g, Some(&word("tarmac")));
        assert_eq!(d.surf, Surf::Concrete);
        let wet = does_wet(&mut g, Some(&DoesWetB { does: Some(String::from("tarmac")) }));
        assert_eq!((wet.surf, wet.breaks, wet.struct_), (Surf::Metal, BreakClass::Metal, None));
        let dry = does_wet(&mut g, Some(&DoesWetB { does: Some(String::from("wood")) }));
        assert_eq!(dry.surf, Surf::Wood);
        let nothing = does_wet(&mut g, None);
        assert_eq!(nothing.surf, Surf::Concrete);
    }
}
