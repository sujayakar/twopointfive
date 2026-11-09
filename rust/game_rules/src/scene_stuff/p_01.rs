//! src/scene/stuff.ts lines 1 to 273
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A paint's record with only the words it means: a finish needs its roughness, a metal flag goes with one, 'plain' is no word. (stuff.ts record)
pub fn record(name: &str, finish: Option<crate::scene_stuff::Finish>, stuff: Option<crate::scene_stuff::BoxStuff>, does: Option<crate::scene_materials::DoesWord>) -> crate::scene_stuff::Paint {
    let (rough, metal) = match finish {
        Some(f) if f.rough.is_some() => (f.rough, if f.metal == Some(true) { Some(true) } else { None }),
        _ => (None, None),
    };
    Paint {
        name: String::from(name),
        rough,
        metal,
        stuff: if stuff != Some(BoxStuff::Plain) { stuff } else { None },
        does,
    }
}

/// What a paint is, in words, for the refusal's message. (stuff.ts fin)
pub fn fin(p: &crate::scene_stuff::Paint) -> String {
    let stuff = match p.stuff {
        Some(s) => s.as_str(),
        None => "plain",
    };
    let finish = match p.rough {
        Some(r) => format!(" at {}{}", js::num_str(r), if p.metal == Some(true) { ", metal" } else { "" }),
        None => String::new(),
    };
    let does = match p.does {
        Some(d) => format!(", doing {}", d.as_str()),
        None => String::new(),
    };
    format!("{}{}{}", stuff, finish, does)
}

/// The paint this array is, if anybody said so. (stuff.ts paintOf)
pub fn paint_of(g: &crate::game::Game, albedo: Option<&[f64]>) -> Option<crate::scene_stuff::Paint> {
    match albedo {
        Some(row) => seam_paints_get(g, row),
        None => None,
    }
}

/// The words a box made with `albedo` carries: the maker's ('plain' = nothing, said over a paint that says something), else the paint's. (stuff.ts wordFor)
pub fn word_for(g: &crate::game::Game, albedo: Option<&[f64]>, said: Option<crate::scene_stuff::BoxStuff>, does: Option<crate::scene_materials::DoesWord>) -> crate::scene_stuff::WordFor {
    let p = paint_of(g, albedo);
    let p_stuff = p.as_ref().and_then(|p| p.stuff);
    let p_does = p.as_ref().and_then(|p| p.does);
    let w = said.or(p_stuff);
    let d = does.or(p_does);
    WordFor {
        stuff: if w != Some(BoxStuff::Plain) { w } else { None },
        does: d,
    }
}

/// PAINTS.get(row) (stuff.ts:110, 141, 196): skeleton 5.1's table, keyed by the colour's bits (review 5's ruling: the TypeScript keys by the ARRAY's identity, so
/// two paints of one colour are one here, and a colour written out as numbers that happens to equal a paint's is that paint).
fn seam_paints_get(g: &Game, row: &[f64]) -> Option<Paint> {
    if row.len() < 3 { return None; }
    g.m.scene_stuff.paints.get([row[0].to_bits(), row[1].to_bits(), row[2].to_bits()]).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_holds_only_the_words_it_means() {
        // a metal flag without a roughness is no finish, false is no flag, 'plain' is no word
        let p = record("a", Some(Finish { rough: None, metal: Some(true) }), Some(BoxStuff::Plain), None);
        assert!(p.rough.is_none() && p.metal.is_none() && p.stuff.is_none() && p.does.is_none());
        assert_eq!(p.name, "a");
        let p = record("", Some(Finish { rough: Some(0.3), metal: Some(false) }), Some(BoxStuff::Steel), Some(crate::scene_materials::DoesWord::Steel));
        assert_eq!(p.rough, Some(0.3));
        assert_eq!(p.metal, None);
        assert_eq!(p.stuff, Some(BoxStuff::Steel));
        assert_eq!(p.does, Some(crate::scene_materials::DoesWord::Steel));
        let p = record("", Some(Finish { rough: Some(0.0), metal: Some(true) }), None, None);
        assert_eq!((p.rough, p.metal), (Some(0.0), Some(true)));
        assert!(record("", None, Some(BoxStuff::Wood), None).rough.is_none());
    }

    #[test]
    fn a_paint_in_words() {
        let mut p = record("", None, None, None);
        assert_eq!(fin(&p), "plain");
        p = record("", Some(Finish { rough: Some(0.3), metal: Some(true) }), Some(BoxStuff::MudWall), Some(crate::scene_materials::DoesWord::DarkGlass));
        assert_eq!(fin(&p), "mud wall at 0.3, metal, doing dark glass");
        p = record("", Some(Finish { rough: Some(1.0), metal: None }), None, None);
        assert_eq!(fin(&p), "plain at 1");
        // the language's own number words, not Rust's (`at ${p.rough}`)
        p = record("", Some(Finish { rough: Some(1e-7), metal: None }), None, None);
        assert_eq!(fin(&p), "plain at 1e-7");
    }
}
