//! WHAT THE TYPESCRIPT LEANS ON OF ITS LANGUAGE, kept as it was: a Map and a Set that remember the order their keys were first set (the game
//! walks them, and the order is play), the integer conversions, the stable sort. A translated file uses these wherever the TypeScript's result
//! depends on the language's own behaviour.
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// A `Map`: `get` / `set` / `delete` by key, walked in the order keys were FIRST set (setting a key again keeps its place; a deleted key set
/// again goes to the end), as JavaScript's.
#[derive(Clone, Debug)]
pub struct JsMap<K: Ord + Clone, V> {
    at: BTreeMap<K, usize>,
    rows: Vec<Option<(K, V)>>,
    live: usize,
}
impl<K: Ord + Clone, V> Default for JsMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Ord + Clone, V> JsMap<K, V> {
    pub const fn new() -> Self {
        JsMap { at: BTreeMap::new(), rows: Vec::new(), live: 0 }
    }
    pub fn len(&self) -> usize {
        self.live
    }
    pub fn is_empty(&self) -> bool {
        self.live == 0
    }
    pub fn has(&self, k: K) -> bool {
        self.at.contains_key(&k)
    }
    pub fn get(&self, k: K) -> Option<&V> {
        self.at.get(&k).and_then(|&i| self.rows[i].as_ref()).map(|r| &r.1)
    }
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        match self.at.get(&k) {
            Some(&i) => self.rows[i].as_mut().map(|r| &mut r.1),
            None => None,
        }
    }
    /// `map.set(k, v)`; what was there before, if anything
    pub fn set(&mut self, k: K, v: V) -> Option<V> {
        if let Some(&i) = self.at.get(&k) {
            return self.rows[i].replace((k, v)).map(|r| r.1);
        }
        self.at.insert(k.clone(), self.rows.len());
        self.rows.push(Some((k, v)));
        self.live += 1;
        None
    }
    /// `let s = map.get(k); if (!s) map.set(k, s = make());` in one step
    pub fn get_or_insert_with(&mut self, k: K, make: impl FnOnce() -> V) -> &mut V {
        if !self.has(k.clone()) {
            self.set(k.clone(), make());
        }
        self.get_mut(k).unwrap()
    }
    /// `map.delete(k)`; what was there
    pub fn delete(&mut self, k: K) -> Option<V> {
        let i = self.at.remove(&k)?;
        self.live -= 1;
        // (the row's place is left empty and NEVER reused or closed up while anything is in the map: a walk in progress keeps its place, as a
        //  JavaScript iterator does. `pack` is for whoever knows no walk is open)
        self.rows[i].take().map(|r| r.1)
    }
    pub fn clear(&mut self) {
        self.at.clear();
        self.rows.clear();
        self.live = 0;
    }
    /// close up the places of deleted keys (the order stays). Only where no walk of this map is open: a cursor's place would move.
    pub fn pack(&mut self) {
        self.rows.retain(|r| r.is_some());
        for (i, r) in self.rows.iter().enumerate() {
            if let Some((k, _)) = r {
                self.at.insert(k.clone(), i);
            }
        }
    }
    /// A WALK THAT SEES THE MAP AS IT CHANGES, as `for (const [k, v] of map)` does: a key deleted before the walk reaches it is skipped, a key
    /// set during the walk is visited. `let mut at = 0; while let Some(k) = m.next_key(&mut at) { … m may be written here … }`
    pub fn next_key(&self, at: &mut usize) -> Option<K> {
        while *at < self.rows.len() {
            let i = *at;
            *at += 1;
            if let Some((k, _)) = &self.rows[i] {
                return Some(k.clone());
            }
        }
        None
    }
    /// the keys, in the order first set (a copy: the map may be written while the list is walked, as a loop over `[...map.keys()]`)
    pub fn keys(&self) -> Vec<K> {
        self.rows.iter().flatten().map(|r| r.0.clone()).collect()
    }
    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.rows.iter().flatten().map(|r| (r.0.clone(), &r.1))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.rows.iter_mut().flatten().map(|r| (r.0.clone(), &mut r.1))
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.rows.iter().flatten().map(|r| &r.1)
    }
}

/// A `Set`, with the same order rule.
#[derive(Clone, Debug)]
pub struct JsSet<K: Ord + Clone> {
    m: JsMap<K, ()>,
}
impl<K: Ord + Clone> Default for JsSet<K> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Ord + Clone> JsSet<K> {
    pub const fn new() -> Self {
        JsSet { m: JsMap::new() }
    }
    pub fn len(&self) -> usize {
        self.m.len()
    }
    pub fn is_empty(&self) -> bool {
        self.m.is_empty()
    }
    pub fn has(&self, k: K) -> bool {
        self.m.has(k)
    }
    /// `set.add(k)`; true where it was not there before
    pub fn add(&mut self, k: K) -> bool {
        self.m.set(k, ()).is_none()
    }
    pub fn delete(&mut self, k: K) -> bool {
        self.m.delete(k).is_some()
    }
    pub fn clear(&mut self) {
        self.m.clear()
    }
    pub fn keys(&self) -> Vec<K> {
        self.m.keys()
    }
    /// a walk that sees the set as it changes (JsMap::next_key)
    pub fn next_key(&self, at: &mut usize) -> Option<K> {
        self.m.next_key(at)
    }
    pub fn iter(&self) -> impl Iterator<Item = K> + '_ {
        self.m.iter().map(|r| r.0)
    }
}

/// `x | 0` (ToInt32: toward zero, then wrapped into 32 bits; NaN and the infinities are 0)
pub fn to_i32(x: f64) -> i32 {
    to_u32(x) as i32
}
/// `x >>> 0` (ToUint32)
pub fn to_u32(x: f64) -> u32 {
    if x != x || x == f64::INFINITY || x == f64::NEG_INFINITY {
        return 0;
    }
    let t = prim::trunc(x);
    let m = t - prim::floor(t / 4294967296.0) * 4294967296.0;
    m as u32
}
/// a number as a condition: `if (x)`, `!x`: 0, −0 and NaN are false
#[inline]
pub fn truthy(x: f64) -> bool {
    x != 0.0 && x == x
}
/// `x || d` on numbers: `d` where x is 0, −0 or NaN
#[inline]
pub fn or(x: f64, d: f64) -> f64 {
    if truthy(x) { x } else { d }
}
/// `a % b` as the language has it for numbers (the sign of the dividend): Rust's `%` on doubles is the same operation, named here so a reader
/// of a translated line sees it was meant
#[inline]
pub fn rem(a: f64, b: f64) -> f64 {
    a % b
}
/// `array.sort((a, b) => number)`: stable, the comparer's sign decides, a NaN from it is "equal"
pub fn sort_by<T>(v: &mut [T], mut cmp: impl FnMut(&T, &T) -> f64) {
    v.sort_by(|a, b| {
        let c = cmp(a, b);
        if c < 0.0 {
            core::cmp::Ordering::Less
        } else if c > 0.0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_map_keeps_the_order_first_set() {
        let mut m: JsMap<u32, &str> = JsMap::new();
        m.set(3, "c");
        m.set(1, "a");
        m.set(2, "b");
        m.set(3, "C");
        assert_eq!(m.keys(), [3, 1, 2]);
        assert_eq!(m.get(3), Some(&"C"));
        m.delete(3);
        m.set(3, "c");
        assert_eq!(m.keys(), [1, 2, 3]);
        assert_eq!(m.len(), 3);
        for i in 10..200 {
            m.set(i, "x");
        }
        for i in 10..200 {
            m.delete(i);
        }
        assert_eq!(m.keys(), [1, 2, 3]);
        m.pack();
        assert_eq!(m.keys(), [1, 2, 3]);
        assert_eq!(m.get(2), Some(&"b"));
        // a walk sees a key set during it and skips one deleted ahead of it, as the language's does
        let (mut at, mut seen) = (0, Vec::new());
        while let Some(k) = m.next_key(&mut at) {
            seen.push(k);
            if k == 1 {
                m.delete(2);
                m.set(7, "g");
            }
        }
        assert_eq!(seen, [1, 3, 7]);
        m.delete(7);
        *m.get_or_insert_with(9, || "n") = "N";
        assert_eq!(m.get(9), Some(&"N"));
    }
    #[test]
    fn a_set_too() {
        let mut s: JsSet<i32> = JsSet::new();
        assert!(s.add(5) && s.add(2) && !s.add(5));
        assert_eq!(s.keys(), [5, 2]);
        assert!(s.delete(5) && !s.delete(5));
    }
    #[test]
    fn the_integer_conversions() {
        assert_eq!(to_i32(3.9), 3);
        assert_eq!(to_i32(-3.9), -3);
        assert_eq!(to_i32(4294967296.0 + 5.0), 5);
        assert_eq!(to_i32(2147483648.0), -2147483648);
        assert_eq!(to_u32(-1.0), 4294967295);
        assert_eq!(to_i32(f64::NAN), 0);
        assert_eq!(rem(-7.0, 3.0), -1.0);
        assert_eq!(or(0.0, 1.0), 1.0);
        assert_eq!(or(-0.0, 1.0), 1.0);
        assert_eq!(or(f64::NAN, 1.0), 1.0);
        assert_eq!(or(-2.0, 1.0), -2.0);
        assert!(!truthy(f64::NAN) && truthy(f64::INFINITY));
    }
    #[test]
    fn the_sort_is_stable() {
        let mut v = [(2, 'a'), (1, 'b'), (2, 'c'), (1, 'd')];
        sort_by(&mut v, |a, b| (a.0 - b.0) as f64);
        assert_eq!(v, [(1, 'b'), (1, 'd'), (2, 'a'), (2, 'c')]);
    }
}
