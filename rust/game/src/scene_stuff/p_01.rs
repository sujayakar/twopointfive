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
        Some(r) => format!(" at {}{}", r, if p.metal == Some(true) { ", metal" } else { "" }),
        None => String::new(),
    };
    let does = match p.does {
        Some(d) => format!(", doing {}", d.as_str()),
        None => String::new(),
    };
    format!("{}{}{}", stuff, finish, does)
}

/// A second saying of one array is refused: thrown where ONE_PAINT says so; otherwise passed over, and false = the caller leaves the paint as it was. (stuff.ts refuse)
pub fn refuse(g: &mut crate::game::Game, row: &[f64], was: &crate::scene_stuff::Paint, next: &crate::scene_stuff::Paint) -> bool {
    if one_paint().refuses {
        let mut nums = String::new();
        for (i, n) in row.iter().enumerate() {
            if i > 0 {
                nums.push_str(", ");
            }
            nums.push_str(&format!("{}", n));
        }
        let was_name = if !was.name.is_empty() { format!(" '{}'", was.name) } else { String::new() };
        let next_name = if !next.name.is_empty() && next.name != was.name { format!(" ('{}')", next.name) } else { String::new() };
        panic!(
            "stuff: the paint [{}]{} is {} and cannot also be {}{}: one array is one paint. A second thing in the same colour is its own array (saidAs, or the numbers written again)",
            nums, was_name, fin(was), fin(next), next_name
        );
    }
    false
}

/// The first saying of a paint stands; the same thing said twice is nothing; a different second saying is refused. (stuff.ts keep)
pub fn keep(g: &mut crate::game::Game, row: &[f64], next: &crate::scene_stuff::Paint) {
    let was = match seam_paints_get(g, row) {
        None => {
            seam_paints_set(g, row, next.clone());
            return;
        }
        Some(was) => was,
    };
    if was.stuff != next.stuff || was.rough != next.rough || was.metal.unwrap_or(false) != next.metal.unwrap_or(false) || was.does != next.does {
        refuse(g, row, &was, next);
    }
}

/// A kit's own paint, what it is made of and its finish said outright. (stuff.ts sayPaint)
pub fn say_paint(g: &mut crate::game::Game, row: &[f64], name: &str, said: crate::scene_stuff::SayPaintSaid) {
    let next = record(name, Some(Finish { rough: said.rough, metal: said.metal }), said.stuff, said.does);
    keep(g, row, &next);
}

/// A colour written as numbers says its word beside it. Returns the colour. (stuff.ts madeOf)
pub fn made_of(g: &mut crate::game::Game, stuff: crate::scene_stuff::BoxStuff, colour: [f64; 3], finish: Option<crate::scene_stuff::MadeOfFinish>) -> [f64; 3] {
    let said = SayPaintSaid {
        stuff: Some(stuff),
        does: None,
        rough: finish.map(|f| f.rough),
        metal: finish.and_then(|f| f.metal),
    };
    say_paint(g, &colour, "", said);
    colour
}

/// madeOf, where nobody chose the word. (stuff.ts drawnAs)
pub fn drawn_as(g: &mut crate::game::Game, stuff: crate::scene_stuff::BoxStuff, colour: [f64; 3], finish: Option<crate::scene_stuff::MadeOfFinish>) -> [f64; 3] {
    made_of(g, stuff, colour, finish)
}

/// The paint this array is, if anybody said so. (stuff.ts paintOf)
pub fn paint_of(g: &mut crate::game::Game, albedo: Option<&[f64]>) -> Option<crate::scene_stuff::Paint> {
    match albedo {
        Some(row) => seam_paints_get(g, row),
        None => None,
    }
}

/// The words a box made with `albedo` carries: the maker's ('plain' = nothing, said over a paint that says something), else the paint's. (stuff.ts wordFor)
pub fn word_for(g: &mut crate::game::Game, albedo: Option<&[f64]>, said: Option<crate::scene_stuff::BoxStuff>, does: Option<crate::scene_materials::DoesWord>) -> crate::scene_stuff::WordFor {
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

/// PAINTS.get(row) (stuff.ts:110, 141, 196): the table and the array's identity are not in the skeleton.
fn seam_paints_get(g: &mut Game, row: &[f64]) -> Option<Paint> {
    todo!("needs: the PAINTS table on g.m.scene_stuff and a key for a paint's array (TypeScript keys by the array's identity; a slice has none)")
}

/// PAINTS.set(row, next) (stuff.ts:142): as seam_paints_get.
fn seam_paints_set(g: &mut Game, row: &[f64], next: Paint) {
    todo!("needs: the PAINTS table on g.m.scene_stuff and a key for a paint's array (TypeScript keys by the array's identity; a slice has none)")
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
    }
}
