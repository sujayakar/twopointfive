//! src/math/side.ts lines 1 to 145
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Lists a table at the top of a module under its name; a name is given once. (side.ts named)
pub fn named(g: &mut crate::game::Game, t: &crate::math_side::Table, name: &str) {
    if g.m.math_side.names.has(String::from(name)) { panic!("side table '{}' is named twice", name); }
    g.m.math_side.names.add(String::from(name));
    g.m.math_side.named.push(t.clone());
}

/// The game a key stands for. A view object does not exist in Rust (seats swap on `g`), so every key answers itself. (side.ts rawKey)
pub fn raw_key<K: Clone + core::fmt::Debug>(k: &K) -> K {
    k.clone()
}

/// Lists a module `let` the simulation writes while it runs. (side.ts rollbackVar)
pub fn rollback_var<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, name: &str, get: &mut dyn FnMut(&mut crate::game::Game) -> T, set: &mut dyn FnMut(&mut crate::game::Game, T)) {
    if g.m.math_side.names.has(String::from(name)) { panic!("rollback variable '{}' is named twice", name); }
    g.m.math_side.names.add(String::from(name));
    g.m.math_side.vars.push(crate::math_side::SideVar { name: String::from(name), get: crate::FnField, set: crate::FnField });
}

/// Lists a module-level container the simulation writes while it runs, and hands it back. (side.ts rollbackRoot)
pub fn rollback_root<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, name: &str, obj: &T) -> T {
    if g.m.math_side.names.has(String::from(name)) { panic!("rollback root '{}' is named twice", name); }
    g.m.math_side.names.add(String::from(name));
    g.m.math_side.roots.push(crate::math_side::MathSideModROOTSItem { name: String::from(name) });
    obj.clone()
}

/// Marks the fields of `o` that hold a reading of the wall clock, and hands `o` back. (side.ts wallClock)
pub fn wall_clock<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, o: &T, keys: &[crate::math_side::WallClockKeys]) -> T {
    let k = seam_clocked_key(g, o);
    let names = seam_clocked_names(keys);
    g.m.math_side.clocked.set(k, names);
    o.clone()
}

/// The fields of an object marked as clock readings. Only snapshot and compare code asked. (side.ts wallClockKeys)
pub fn wall_clock_keys(g: &mut crate::game::Game) -> Option<Vec<String>> {
    todo!("needs: the parameter `o`, which the skeleton dropped on purpose: only rollback.ts snapshot and compare code calls this, and nothing translated may")
}

/// Everything registered, each list in the order of its name; the memos left out of the tables. (side.ts sideRegistry)
pub fn side_registry(g: &mut crate::game::Game) -> crate::math_side::SideRegistry {
    let s = &mut g.m.math_side;
    if s.sorted != s.names.len() as f64 || s.sorted_memos != s.memos {
        js::sort_by(&mut s.named, |a, b| if js_lt(&seam_side_name(a), &seam_side_name(b)) { -1.0 } else { 1.0 });
        js::sort_by(&mut s.vars, |a, b| if js_lt(&a.name, &b.name) { -1.0 } else { 1.0 });
        js::sort_by(&mut s.roots, |a, b| if js_lt(&a.name, &b.name) { -1.0 } else { 1.0 });
        s.sorted = s.names.len() as f64;
        s.state = s.named.iter().filter(|t| !seam_side_memo(t)).cloned().collect();
        s.sorted_memos = s.memos;
    }
    crate::math_side::SideRegistry {
        tables: s.state.clone(),
        vars: s.vars.clone(),
        roots: s.roots.iter().map(|r| crate::math_side::SideRegistryRootsItem { name: r.name.clone() }).collect(),
    }
}

/// JavaScript's `a < b` on strings: by UTF-16 code unit, which is not the bytes' order past U+FFFF.
fn js_lt(a: &str, b: &str) -> bool {
    a.encode_utf16().lt(b.encode_utf16())
}

/// side.ts:124 `CLOCKED.set(o, keys)`: the key of `o`.
fn seam_clocked_key<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, o: &T) -> crate::ObjKey {
    todo!("needs: an identity for `o`: T has no ObjKey to give")
}

/// side.ts:124 the names in `keys`.
fn seam_clocked_names(keys: &[crate::math_side::WallClockKeys]) -> Vec<String> {
    todo!("needs: `keys` as field names: WallClockKeys is a string's `length`, the name itself is lost")
}

/// side.ts:139 `t.sideName`.
fn seam_side_name(t: &crate::math_side::Table) -> String {
    todo!("needs: sideName on Table")
}

/// side.ts:140 `t.sideMemo`.
fn seam_side_memo(t: &crate::math_side::Table) -> bool {
    todo!("needs: sideMemo on Table")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(g: &mut Game, name: &str) {
        rollback_var(g, name, &mut |_: &mut Game| 0.0f64, &mut |_: &mut Game, _: f64| {});
    }
    fn var_names(g: &mut Game) -> Vec<String> {
        side_registry(g).vars.iter().map(|v| v.name.clone()).collect()
    }

    #[test]
    fn the_registry_is_in_name_order_and_sorts_again_when_a_name_comes() {
        let mut g = Game::blank();
        var(&mut g, "b.x");
        var(&mut g, "a.y");
        assert_eq!(var_names(&mut g), ["a.y", "b.x"]);
        assert_eq!(g.m.math_side.sorted, 2.0);
        var(&mut g, "B.z");   // a capital is before every lower case letter
        assert_eq!(var_names(&mut g), ["B.z", "a.y", "b.x"]);
    }

    #[test]
    fn the_roots_sort_by_utf16_code_unit_not_by_byte() {
        let mut g = Game::blank();
        assert_eq!(rollback_root(&mut g, "\u{FF61}", &7.0f64), 7.0);
        assert_eq!(rollback_root(&mut g, "\u{10000}", &8.0f64), 8.0);   // its first unit is U+D800, under U+FF61; its first byte is above
        let roots: Vec<String> = side_registry(&mut g).roots.iter().map(|r| r.name.clone()).collect();
        assert_eq!(roots, ["\u{10000}", "\u{FF61}"]);
    }

    #[test]
    #[should_panic(expected = "rollback variable 'a.b' is named twice")]
    fn a_name_is_one_across_the_kinds() {
        let mut g = Game::blank();
        rollback_root(&mut g, "a.b", &0.0f64);
        var(&mut g, "a.b");
    }
}
