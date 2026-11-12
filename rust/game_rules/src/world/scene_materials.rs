pub use ::game_types::scene_materials::types;
pub use types::*;
use crate::game::Game;
use crate::ids::BoxId;

/// what a box of the world does
pub fn does_of_box(g: &Game, b: BoxId) -> &'static Does { does_of(g.tab.boxes[b].does) }

#[cfg(test)]
mod tests {
    use super::*;

    /// A FAILURE MEANS: a box nobody spoke for is another thing to a round than the default row.
    #[test]
    fn nothing_said_takes_the_default_row() {
        let d = does_of(None);
        assert_eq!((d.surf, d.breaks, d.casing), (Surf::Concrete, BreakClass::Metal, false));
        assert_eq!(d, DoesWord::Unnamed.row());
    }

    /// A FAILURE MEANS: a word takes another word's row, or a file's word is no word of the table.
    #[test]
    fn a_word_finds_its_own_row_and_is_found_by_what_a_file_calls_it() {
        let d = does_of(Some(DoesWord::Wood));
        assert_eq!((d.surf, d.breaks, d.cover), (Surf::Wood, BreakClass::Wood, Some(DoesCover::Wood)));
        assert_eq!((DoesWord::Screen.row().surf, DoesWord::Screen.row().screen, DoesWord::DarkGlass.row().screen), (Surf::Glass, true, false));
        for &w in DoesWord::ALL { assert_eq!(DoesWord::named(w.as_str()), Some(w)); }
        assert_eq!((DoesWord::named("unnamed dark"), DoesWord::named("mud wall")), (Some(DoesWord::UnnamedDark), None));
    }
}
