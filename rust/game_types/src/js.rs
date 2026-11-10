//! WHAT THE TYPESCRIPT LEANS ON OF ITS LANGUAGE, kept as it was: a Map and a Set that remember the order their keys were first set (the game
//! walks them, and the order is play), the integer conversions, the stable sort. A translated file uses these wherever the TypeScript's result
//! depends on the language's own behaviour.
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};

/// A `Map`: `get` / `set` / `delete` by key, walked in the order keys were FIRST set (setting a key again keeps its place; a deleted key set
/// again goes to the end), as JavaScript's.
///
/// A deleted key's row stays as a hole only until the holes outnumber the live rows by `PACK_OVER`; then the map closes them up, in `delete`
/// itself. That is safe under an open walk because A WALK'S CURSOR IS NOT A PLACE IN THE ROWS: it is a row's NUMBER, the count of keys first set
/// before it since the map was made, which no packing changes (`next_key`). So a table set and deleted every tick stays as small as what it holds.
///
/// A key is FOUND by a scan of the rows while they are few, and past `SCAN` rows by a table of their places, open addressed on the key's hash. Nothing but finding a
/// key reads the table: the order of the game's walks is the rows' and no hash is ever in it.
#[derive(Clone, Debug)]
pub struct JsMap<K: Eq + Clone + Hash, V> {
    /// a key's place in `rows`, plus one, at a slot its hash leads to: `EMPTY`, or `GONE` where one was deleted. Empty while the rows are few. A power of two long.
    table: Vec<u32>,
    /// the table's slots that are not `EMPTY`
    used: usize,
    /// (the row's number, the row or the hole a delete left), in the order first set: the numbers rise along it
    rows: Vec<(usize, Option<(K, V)>)>,
    live: usize,
    /// the number the next key first set takes. Never taken back, not by `clear` either: a walk open over a clear then meets the keys set after
    /// it, as the language's iterator does
    next: usize,
}
/// how many more holes than live rows a map holds before a delete closes them up: the packing is paid for by at least that many deletes
const PACK_OVER: usize = 16;
/// how many rows are found by walking them before a map keeps a table of their places
const SCAN: usize = 8;
const EMPTY: u32 = 0;
const GONE: u32 = u32::MAX;
/// the multiply and rotate of a word at a time: a key's hash is read only to find its slot, on one machine, within one run
struct KeyHash(u64);
impl Hasher for KeyHash {
    fn write(&mut self, bytes: &[u8]) {
        for c in bytes.chunks(8) { let mut w = [0u8; 8]; w[..c.len()].copy_from_slice(c); self.write_u64(u64::from_le_bytes(w)); }
    }
    fn write_u8(&mut self, x: u8) { self.write_u64(x as u64); }
    fn write_u16(&mut self, x: u16) { self.write_u64(x as u64); }
    fn write_u32(&mut self, x: u32) { self.write_u64(x as u64); }
    fn write_usize(&mut self, x: usize) { self.write_u64(x as u64); }
    fn write_u64(&mut self, x: u64) { self.0 = (self.0.rotate_left(5) ^ x).wrapping_mul(0x517c_c1b7_2722_0a95); }
    // (a whole number held as a double has only high bits, and a multiply keeps its low ones zero: its high bits are brought down, twice over, before a table's mask reads the low)
    fn finish(&self) -> u64 {
        let h = (self.0 ^ (self.0 >> 33)).wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^ (h >> 33)
    }
}
fn hash_of<K: Hash>(k: &K) -> u64 {
    let mut h = KeyHash(0);
    k.hash(&mut h);
    h.finish()
}
impl<K: Eq + Clone + Hash, V> Default for JsMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Eq + Clone + Hash, V> JsMap<K, V> {
    pub const fn new() -> Self {
        JsMap { table: Vec::new(), used: 0, rows: Vec::new(), live: 0, next: 0 }
    }
    pub fn len(&self) -> usize {
        self.live
    }
    pub fn is_empty(&self) -> bool {
        self.live == 0
    }
    /// the key's place in the rows
    fn place(&self, k: &K) -> Option<usize> {
        if self.table.is_empty() {
            return self.rows.iter().position(|r| matches!(&r.1, Some((x, _)) if x == k));
        }
        let mask = self.table.len() - 1;
        let mut s = hash_of(k) as usize & mask;
        loop {
            match self.table[s] {
                EMPTY => return None,
                GONE => {}
                p => {
                    let i = p as usize - 1;
                    if matches!(&self.rows[i].1, Some((x, _)) if x == k) { return Some(i); }
                }
            }
            s = (s + 1) & mask;
        }
    }
    /// every row's place in a table of `slots` slots (none: found by walking the rows)
    fn file_all(&mut self) {
        self.used = 0;
        if self.rows.len() <= SCAN { self.table = Vec::new(); return; }
        let slots = (self.rows.len() * 4).next_power_of_two();
        self.table.clear();
        self.table.resize(slots, EMPTY);
        for i in 0..self.rows.len() {
            if let Some((k, _)) = &self.rows[i].1 {
                let mut s = hash_of(k) as usize & (slots - 1);
                while self.table[s] != EMPTY { s = (s + 1) & (slots - 1); }
                self.table[s] = i as u32 + 1;
                self.used += 1;
            }
        }
    }
    pub fn has(&self, k: K) -> bool {
        self.place(&k).is_some()
    }
    pub fn get(&self, k: K) -> Option<&V> {
        self.place(&k).and_then(|i| self.rows[i].1.as_ref()).map(|r| &r.1)
    }
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        match self.place(&k) {
            Some(i) => self.rows[i].1.as_mut().map(|r| &mut r.1),
            None => None,
        }
    }
    /// `map.set(k, v)`; what was there before, if anything
    pub fn set(&mut self, k: K, v: V) -> Option<V> {
        if let Some(i) = self.place(&k) {
            return self.rows[i].1.as_mut().map(|r| core::mem::replace(&mut r.1, v));   // (the key first set stays, as the language's)
        }
        let i = self.rows.len();
        let slot = if self.table.is_empty() { None } else { Some(hash_of(&k) as usize) };
        self.rows.push((self.next, Some((k, v))));
        self.next += 1;
        self.live += 1;
        match slot {
            // (the table is kept under half full, what was deleted counted; at that it is made again)
            Some(h) if (self.used + 1) * 2 <= self.table.len() => {
                let mask = self.table.len() - 1;
                let mut s = h & mask;
                while self.table[s] != EMPTY && self.table[s] != GONE { s = (s + 1) & mask; }
                if self.table[s] == EMPTY { self.used += 1; }
                self.table[s] = i as u32 + 1;
            }
            _ => if self.rows.len() > SCAN { self.file_all(); },
        }
        None
    }
    /// `let s = map.get(k); if (!s) map.set(k, s = make());` in one step
    pub fn get_or_insert_with(&mut self, k: K, make: impl FnOnce() -> V) -> &mut V {
        let i = match self.place(&k) {
            Some(i) => i,
            None => { self.set(k, make()); self.rows.len() - 1 }
        };
        &mut self.rows[i].1.as_mut().unwrap().1
    }
    /// `map.delete(k)`; what was there
    pub fn delete(&mut self, k: K) -> Option<V> {
        let i = self.place(&k)?;
        if !self.table.is_empty() {
            let mask = self.table.len() - 1;
            let mut s = hash_of(&k) as usize & mask;
            while self.table[s] != i as u32 + 1 { s = (s + 1) & mask; }
            self.table[s] = GONE;
        }
        self.live -= 1;
        let was = self.rows[i].1.take().map(|r| r.1);
        // (the hole is closed up once there are more of them than the map is worth: a walk in progress keeps its place by the row's number)
        if self.rows.len() - self.live > self.live + PACK_OVER {
            self.pack();
        }
        was
    }
    pub fn clear(&mut self) {
        self.table = Vec::new();
        self.used = 0;
        self.rows.clear();
        self.live = 0;
    }
    /// close up the places of deleted keys (the order and every row's number stay). Safe at any time, under an open walk too.
    pub fn pack(&mut self) {
        self.rows.retain(|r| r.1.is_some());
        self.file_all();
    }
    /// how many holes the rows hold now (for a count of them at a run's end: never more than the live rows and `PACK_OVER`)
    pub fn holes(&self) -> usize {
        self.rows.len() - self.live
    }
    /// A WALK THAT SEES THE MAP AS IT CHANGES, as `for (const [k, v] of map)` does: a key deleted before the walk reaches it is skipped, a key
    /// set during the walk is visited. `let mut at = 0; while let Some(k) = m.next_key(&mut at) { … m may be written here … }`
    /// The cursor is the NUMBER of the next row to look at, not its place: the map may pack between two calls.
    pub fn next_key(&self, at: &mut usize) -> Option<K> {
        let mut i = self.rows.partition_point(|r| r.0 < *at);
        while i < self.rows.len() {
            let (n, r) = &self.rows[i];
            i += 1;
            if let Some((k, _)) = r {
                *at = n + 1;
                return Some(k.clone());
            }
        }
        *at = self.next;
        None
    }
    /// the keys, in the order first set (a copy: the map may be written while the list is walked, as a loop over `[...map.keys()]`)
    pub fn keys(&self) -> Vec<K> {
        self.rows.iter().filter_map(|r| r.1.as_ref()).map(|r| r.0.clone()).collect()
    }
    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.rows.iter().filter_map(|r| r.1.as_ref()).map(|r| (r.0.clone(), &r.1))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.rows.iter_mut().filter_map(|r| r.1.as_mut()).map(|r| (r.0.clone(), &mut r.1))
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.rows.iter().filter_map(|r| r.1.as_ref()).map(|r| &r.1)
    }
}

/// A `Set`, with the same order rule.
#[derive(Clone, Debug)]
pub struct JsSet<K: Eq + Clone + Hash> {
    m: JsMap<K, ()>,
}
impl<K: Eq + Clone + Hash> Default for JsSet<K> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Eq + Clone + Hash> JsSet<K> {
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
/// `x | bits` on flags kept as numbers
#[inline]
pub fn or_bits(x: f64, bits: f64) -> f64 {
    (to_i32(x) | to_i32(bits)) as f64
}
/// `(x & bits) !== 0` on flags kept as numbers
#[inline]
pub fn has_bits(x: f64, bits: i32) -> bool {
    (to_i32(x) & bits) != 0
}
/// a pattern's `\w`
#[inline]
pub fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
/// `s.length`
#[inline]
pub fn utf16_len(s: &str) -> f64 {
    s.encode_utf16().count() as f64
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

/// `x.toFixed(digits)`: the decimal of the double's EXACT value, rounded half up at `digits` places (a tie goes to the larger number, as
/// the language says; `format!("{:.1}")` rounds a tie to even and so differs at 0.25). `digits` 0 to 20, as JavaScript's.
pub fn to_fixed(x: f64, digits: u32) -> alloc::string::String {
    use alloc::string::{String, ToString};
    if x.is_nan() {
        return "NaN".to_string();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() };
    }
    let digits = if digits > 20 { 20 } else { digits };
    let neg = x < 0.0;
    let a = if neg { -x } else { x };
    let bits = a.to_bits();
    let ex = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & 0x000f_ffff_ffff_ffff;
    // a = mant × 2^e, exactly
    let (mant, e) = if ex == 0 { (frac as u128, -1074) } else { ((frac | (1 << 52)) as u128, ex - 1075) };
    let p10 = 10u128.pow(digits);
    // n = round half up of a × 10^digits, exactly, in 128 bits where it fits (a below about 2^60 at 20 places: every number a game prints)
    if a >= 1e21 {
        // JavaScript itself gives up the fixed form here: its plain number-to-string, exponent form ("1e+21")
        let s = alloc::format!("{:e}", a).replace('e', "e+");
        return if neg { alloc::format!("-{}", s) } else { s };
    }
    // n = round half up of a × 10^digits, exactly. A double with e >= 0 is a whole number (below 2^70 here): its places are zeros.
    let (n, whole): (u128, bool) = if e >= 0 {
        (mant << e, true)
    } else {
        let s = (-e) as u32;
        if s > 127 {
            (0, false)
        } else {
            let v = mant * p10; // 53 bits × at most 67: fits
            ((v + (1u128 << (s - 1))) >> s, false)
        }
    };
    if whole {
        let mut out = if neg { alloc::format!("-{}", n) } else { n.to_string() };
        if digits > 0 {
            out.push('.');
            for _ in 0..digits {
                out.push('0');
            }
        }
        return out;
    }
    let mut d = n.to_string();
    let w = digits as usize;
    while d.len() < w + 1 {
        d.insert(0, '0');
    }
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    if w == 0 {
        out.push_str(&d);
    } else {
        out.push_str(&d[..d.len() - w]);
        out.push('.');
        out.push_str(&d[d.len() - w..]);
    }
    out
}

/// `String(x)` / `${x}` of a number: JavaScript's own form (the shortest digits that read back the same double; plain decimals from
/// 1e-6 up to 1e21, exponent form outside: "1e+21", "1.5e-7"; "0" for minus zero; "NaN", "Infinity").
pub fn num_str(x: f64) -> alloc::string::String {
    use alloc::string::{String, ToString};
    if x.is_nan() {
        return "NaN".to_string();
    }
    if x == 0.0 {
        return "0".to_string();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() };
    }
    let neg = x < 0.0;
    // Rust's `{:e}` gives the same shortest digits: "d.ddde<exp>"
    let e = alloc::format!("{:e}", if neg { -x } else { x });
    let (mant, exp) = e.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32; // how many digits
    let n = exp + 1; // the decimal point sits after n digits
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    if k <= n && n <= 21 {
        out.push_str(&digits);
        for _ in 0..(n - k) {
            out.push('0');
        }
    } else if 0 < n && n <= 21 {
        out.push_str(&digits[..n as usize]);
        out.push('.');
        out.push_str(&digits[n as usize..]);
    } else if -6 < n && n <= 0 {
        out.push_str("0.");
        for _ in 0..(-n) {
            out.push('0');
        }
        out.push_str(&digits);
    } else {
        out.push_str(&digits[..1]);
        if k > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        out.push(if n - 1 < 0 { '-' } else { '+' });
        out.push_str(&(n - 1).abs().to_string());
    }
    out
}

// ── A STRING READ AS A NUMBER, AND WHAT THE LANGUAGE CALLS A BLANK ─────────────────────────────────────────────────────────────────────────────
// `Number(text)`, `parseFloat(text)` and the language's blank: its own grammar, which Rust's `parse` and `char::is_whitespace` are near and not
// equal to. One place: an address's words (query.rs), the mission list and the loadout read through here.

/// how much of `s` is a decimal number as JavaScript writes one (`+.5e1`, `-3`, `5.`, `Infinity`): its length in bytes, 0 = none
fn decimal_prefix(s: &str) -> usize {
    let b = s.as_bytes();
    let mut i = if matches!(b.first(), Some(b'+') | Some(b'-')) { 1 } else { 0 };
    if s[i..].starts_with("Infinity") { return i + 8; }
    let d0 = i;
    while i < b.len() && b[i].is_ascii_digit() { i += 1; }
    let whole = i - d0;
    let mut frac = 0;
    if i < b.len() && b[i] == b'.' { let f0 = i + 1; let mut j = f0; while j < b.len() && b[j].is_ascii_digit() { j += 1; } frac = j - f0; if whole + frac > 0 { i = j; } }
    if whole + frac == 0 { return 0; }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') { j += 1; }
        let e0 = j;
        while j < b.len() && b[j].is_ascii_digit() { j += 1; }
        if j > e0 { i = j; }
    }
    i
}
fn decimal(s: &str) -> f64 {
    match s.trim_start_matches('+') { "Infinity" => f64::INFINITY, "-Infinity" => f64::NEG_INFINITY, t => t.parse::<f64>().unwrap_or(f64::NAN) }
}
/// `Number(string)`: the language's own grammar, not Rust's (which takes 'inf' and 'nan', and refuses '' and hex).
pub fn number_of(s: &str) -> f64 {
    let t = s.trim_matches(is_space);
    if t.is_empty() { return 0.0; }
    let b = t.as_bytes();
    if b.len() >= 2 && b[0] == b'0' {
        let bits = match b[1] { b'x' | b'X' => 4, b'o' | b'O' => 3, b'b' | b'B' => 1, _ => 0 };
        if bits != 0 { return radix_value(&b[2..], bits); }
    }
    let unsigned = t.strip_prefix(['+', '-']).unwrap_or(t);
    if unsigned == "Infinity" { return if b[0] == b'-' { f64::NEG_INFINITY } else { f64::INFINITY }; }
    if !is_decimal_literal(unsigned) { return f64::NAN; }
    t.parse().unwrap_or(f64::NAN)
}

/// digits [. digits] [e [+-] digits], with a digit somewhere before the exponent.
fn is_decimal_literal(s: &str) -> bool {
    let digits = |s: &str| s.bytes().take_while(u8::is_ascii_digit).count();
    let whole = digits(s);
    let mut rest = &s[whole..];
    let mut fraction = 0;
    if let Some(after) = rest.strip_prefix('.') {
        fraction = digits(after);
        rest = &after[fraction..];
    }
    if whole + fraction == 0 { return false; }
    if rest.is_empty() { return true; }
    let Some(exp) = rest.strip_prefix(['e', 'E']) else { return false };
    let exp = exp.strip_prefix(['+', '-']).unwrap_or(exp);
    !exp.is_empty() && digits(exp) == exp.len()
}

/// A `0x` / `0o` / `0b` literal's digits, `bits` a digit, rounded once to the nearest double as the language does; NaN if none or a bad one.
fn radix_value(digits: &[u8], bits: u32) -> f64 {
    if digits.is_empty() { return f64::NAN; }
    let (mut kept, mut scale) = (0u64, 0u32);
    for &b in digits {
        let Some(d) = (b as char).to_digit(1 << bits) else { return f64::NAN };
        if kept >> (64 - bits) == 0 {
            kept = kept << bits | d as u64;
        } else {
            scale += bits;
            if d != 0 { kept |= 1; } // past 54 bits only "was anything left" matters to the rounding
        }
    }
    let mut v = kept as f64;
    for _ in 0..scale {
        v *= 2.0;
        if v.is_infinite() { break; }
    }
    v
}

/// The language's StrWhiteSpaceChar (`char::is_whitespace` differs at U+0085 and U+FEFF).
pub fn is_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{B}' | '\u{C}' | '\r' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}')
}
/// `parseFloat(text)`: leading blanks passed, then the decimal the text STARTS with (`2px` is 2); NaN when it starts with none
pub fn float_of(text: &str) -> f64 {
    let s = text.trim_start_matches(is_space);
    match decimal_prefix(s) { 0 => f64::NAN, n => decimal(&s[..n]) }
}

#[cfg(test)]
mod tests {
    /// past the scan the table finds what the rows hold: whole numbers as doubles, a delete, a key set again at the end, a pack, and the order of the walk throughout
    #[test]
    fn a_big_map_finds_and_walks_as_a_small_one() {
        use crate::Num;
        let mut m: JsMap<Num, u32> = JsMap::new();
        for i in 0..2000u32 { m.set(Num(i as f64 * 3.0), i); }
        for i in 0..2000u32 { assert_eq!(m.get(Num(i as f64 * 3.0)), Some(&i)); assert!(!m.has(Num(i as f64 * 3.0 + 1.0))); }
        for i in (0..2000u32).step_by(2) { assert_eq!(m.delete(Num(i as f64 * 3.0)), Some(i)); }
        assert_eq!(m.len(), 1000);
        m.set(Num(0.0), 7);
        let keys = m.keys();
        assert_eq!((keys[0].0, keys[1].0, keys[1000].0), (3.0, 9.0, 0.0));
        for i in (1..2000u32).step_by(2) { m.delete(Num(i as f64 * 3.0)); }
        assert_eq!(m.keys().len(), 1);
        assert_eq!(m.get(Num(0.0)), Some(&7));
        m.clear();
        assert!(!m.has(Num(0.0)));
        m.set(Num(1.0), 1);
        assert_eq!(m.get(Num(1.0)), Some(&1));
    }
    /// a number as a key, as the language's Map has it
    #[test]
    fn negative_zero_is_zero_and_nan_is_nan() {
        use crate::Num;
        for n in [4usize, 40] {
            let mut m: JsMap<Num, u32> = JsMap::new();
            for i in 0..n { m.set(Num(i as f64 + 1.0), 0); }
            m.set(Num(0.0), 1);
            assert_eq!(m.get(Num(-0.0)), Some(&1));
            m.set(Num(f64::NAN), 2);
            assert_eq!(m.get(Num(-f64::NAN)), Some(&2));
            assert_eq!(m.len(), n + 2);
        }
    }

    use super::*;
    use alloc::string::String;
    /// each row: the text, JavaScript's `Number(text)` and `parseFloat(text)` of it as the language prints them (read off bun)
    #[test]
    fn reads_a_word_as_a_number_as_the_language_does() {
        let rows: [(&str, &str, &str); 32] = [("2", "2", "2"), (" 2 ", "2", "2"), ("", "0", "NaN"), ("  ", "0", "NaN"), ("2.5", "2.5", "2.5"), (".5", "0.5", "0.5"), ("5.", "5", "5"), ("-3", "-3", "-3"),
            ("+4", "4", "4"), ("1e3", "1000", "1000"), ("1E-2", "0.01", "0.01"), ("0x10", "16", "0"), ("0b11", "3", "0"), ("0o17", "15", "0"), ("Infinity", "Infinity", "Infinity"),
            ("-Infinity", "-Infinity", "-Infinity"), ("inf", "NaN", "NaN"), ("nan", "NaN", "NaN"), ("NaN", "NaN", "NaN"), ("2px", "NaN", "2"), ("px2", "NaN", "NaN"), ("1.2.3", "NaN", "1.2"),
            ("1e", "NaN", "1"), ("--1", "NaN", "NaN"), ("0.5x", "NaN", "0.5"), ("1_0", "NaN", "1"), ("+.5e1", "5", "5"), (".", "NaN", "NaN"), ("-", "NaN", "NaN"), ("e5", "NaN", "NaN"),
            ("12abc", "NaN", "12"), (" 3.25cams", "NaN", "3.25")];
        let said = |x: f64| if x.is_nan() { String::from("NaN") } else if x.is_infinite() { String::from(if x > 0.0 { "Infinity" } else { "-Infinity" }) } else { alloc::format!("{}", x) };
        for (text, number, float) in rows {
            assert_eq!(said(number_of(text)), number, "Number({:?})", text);
            assert_eq!(said(float_of(text)), float, "parseFloat({:?})", text);
        }
    }
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
    fn a_map_set_and_deleted_every_tick_stays_small_and_a_walk_keeps_its_place() {
        let mut m: JsMap<u32, u32> = JsMap::new();
        m.set(1, 1);
        m.set(2, 2);
        for i in 10..100_010 {
            m.set(i, i);
            m.set(i + 1_000_000, i);
            m.delete(i);
            m.delete(i + 1_000_000);
            assert!(m.holes() <= m.len() + PACK_OVER + 1);
        }
        assert_eq!(m.keys(), [1, 2]);
        // a walk open while the map packs under it: every key set before or during it is met once, in the order first set, a deleted one never
        for i in 0..40 {
            m.set(100 + i, i);
        }
        let (mut at, mut seen) = (0, Vec::new());
        while let Some(k) = m.next_key(&mut at) {
            seen.push(k);
            if k == 2 {
                for i in 0..38 {
                    m.delete(100 + i);   // (enough holes to pack, twice over)
                }
                m.set(7, 7);
            }
            if k == 138 {
                m.delete(139);
                m.set(8, 8);
            }
        }
        assert_eq!(seen, [1, 2, 138, 7, 8]);
        // …and over a clear: the keys set after it are met
        let (mut at, mut seen) = (0, Vec::new());
        while let Some(k) = m.next_key(&mut at) {
            seen.push(k);
            if k == 2 {
                m.clear();
                m.set(5, 5);
                m.set(6, 6);
            }
        }
        assert_eq!(seen, [1, 2, 5, 6]);
    }
    #[test]
    fn a_walk_that_deletes_the_key_it_stands_on_meets_every_key_once_across_packs() {
        // (the shape of lanes' walks over `held` and `who` and pressure's over `marks`: the key just handed out is deleted before the next is asked)
        let mut m: JsMap<u32, u32> = JsMap::new();
        for i in 0..200 {
            m.set(i, i);
        }
        let (mut at, mut seen, mut most_holes) = (0, Vec::new(), 0);
        while let Some(k) = m.next_key(&mut at) {
            seen.push(k);
            m.delete(k);
            most_holes = most_holes.max(m.holes());
            if k % 50 == 0 && k < 200 {
                m.set(1000 + k, k);
            }
        }
        let mut want: Vec<u32> = (0..200).collect();
        want.extend([1000, 1050, 1100, 1150]);
        assert_eq!(seen, want);
        assert!(m.is_empty());
        assert!(most_holes < 150, "the map packed under the walk ({most_holes} holes at most, of 204 deletes)");
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
    #[test]
    fn to_fixed_is_javascripts() {
        // (the double's bits, digits, what bun printed for x.toFixed(digits))
        let cases: &[(u64, u32, &str)] = &[
            (0x0, 0, "0"),
            (0x0, 1, "0.0"),
            (0x0, 2, "0.00"),
            (0x0, 3, "0.000"),
            (0x8000000000000000, 0, "0"),
            (0x8000000000000000, 1, "0.0"),
            (0x8000000000000000, 2, "0.00"),
            (0x8000000000000000, 3, "0.000"),
            (0x3fe0000000000000, 0, "1"),
            (0x3fe0000000000000, 1, "0.5"),
            (0x3fe0000000000000, 2, "0.50"),
            (0x3fe0000000000000, 3, "0.500"),
            (0x3ff8000000000000, 0, "2"),
            (0x3ff8000000000000, 1, "1.5"),
            (0x3ff8000000000000, 2, "1.50"),
            (0x3ff8000000000000, 3, "1.500"),
            (0x4004000000000000, 0, "3"),
            (0x4004000000000000, 1, "2.5"),
            (0x4004000000000000, 2, "2.50"),
            (0x4004000000000000, 3, "2.500"),
            (0x3fd0000000000000, 0, "0"),
            (0x3fd0000000000000, 1, "0.3"),
            (0x3fd0000000000000, 2, "0.25"),
            (0x3fd0000000000000, 3, "0.250"),
            (0x3fd6666666666666, 0, "0"),
            (0x3fd6666666666666, 1, "0.3"),
            (0x3fd6666666666666, 2, "0.35"),
            (0x3fd6666666666666, 3, "0.350"),
            (0x3ff0147ae147ae14, 0, "1"),
            (0x3ff0147ae147ae14, 1, "1.0"),
            (0x3ff0147ae147ae14, 2, "1.00"),
            (0x3ff0147ae147ae14, 3, "1.005"),
            (0xbfe0000000000000, 0, "-1"),
            (0xbfe0000000000000, 1, "-0.5"),
            (0xbfe0000000000000, 2, "-0.50"),
            (0xbfe0000000000000, 3, "-0.500"),
            (0xbf1a36e2eb1c432d, 0, "-0"),
            (0xbf1a36e2eb1c432d, 1, "-0.0"),
            (0xbf1a36e2eb1c432d, 2, "-0.00"),
            (0xbf1a36e2eb1c432d, 3, "-0.000"),
            (0x405edd2f1a9fbe77, 0, "123"),
            (0x405edd2f1a9fbe77, 1, "123.5"),
            (0x405edd2f1a9fbe77, 2, "123.46"),
            (0x405edd2f1a9fbe77, 3, "123.456"),
            (0x444b1ae4d6e2ef50, 0, "1e+21"),
            (0x444b1ae4d6e2ef50, 1, "1e+21"),
            (0x444b1ae4d6e2ef50, 2, "1e+21"),
            (0x444b1ae4d6e2ef50, 3, "1e+21"),
            (0x4415af1d78b58c40, 0, "100000000000000000000"),
            (0x4415af1d78b58c40, 1, "100000000000000000000.0"),
            (0x4415af1d78b58c40, 2, "100000000000000000000.00"),
            (0x4415af1d78b58c40, 3, "100000000000000000000.000"),
            (0x3eb0c6f7a0b5ed8d, 0, "0"),
            (0x3eb0c6f7a0b5ed8d, 1, "0.0"),
            (0x3eb0c6f7a0b5ed8d, 2, "0.00"),
            (0x3eb0c6f7a0b5ed8d, 3, "0.000"),
            (0x3ea0c6f7a0b5ed8d, 0, "0"),
            (0x3ea0c6f7a0b5ed8d, 1, "0.0"),
            (0x3ea0c6f7a0b5ed8d, 2, "0.00"),
            (0x3ea0c6f7a0b5ed8d, 3, "0.000"),
            (0xbff8000000000000, 0, "-2"),
            (0xbff8000000000000, 1, "-1.5"),
            (0xbff8000000000000, 2, "-1.50"),
            (0xbff8000000000000, 3, "-1.500"),
            (0x4002c28f5c28f5c3, 0, "2"),
            (0x4002c28f5c28f5c3, 1, "2.3"),
            (0x4002c28f5c28f5c3, 2, "2.35"),
            (0x4002c28f5c28f5c3, 3, "2.345"),
            (0x4058ffae147ae148, 0, "100"),
            (0x4058ffae147ae148, 1, "100.0"),
            (0x4058ffae147ae148, 2, "100.00"),
            (0x4058ffae147ae148, 3, "99.995"),
            (0x3fd5555555555555, 0, "0"),
            (0x3fd5555555555555, 1, "0.3"),
            (0x3fd5555555555555, 2, "0.33"),
            (0x3fd5555555555555, 3, "0.333"),
            (0xc005666666666666, 0, "-3"),
            (0xc005666666666666, 1, "-2.7"),
            (0xc005666666666666, 2, "-2.67"),
            (0xc005666666666666, 3, "-2.675"),
            (0x4132d687e4189375, 0, "1234568"),
            (0x4132d687e4189375, 1, "1234567.9"),
            (0x4132d687e4189375, 2, "1234567.89"),
            (0x4132d687e4189375, 3, "1234567.891"),
            (0x3fd3333333333334, 0, "0"),
            (0x3fd3333333333334, 1, "0.3"),
            (0x3fd3333333333334, 2, "0.30"),
            (0x3fd3333333333334, 3, "0.300"),
            (0x7ff8000000000000, 0, "NaN"),
            (0x7ff8000000000000, 1, "NaN"),
            (0x7ff8000000000000, 2, "NaN"),
            (0x7ff8000000000000, 3, "NaN"),
            (0x7ff0000000000000, 0, "Infinity"),
            (0x7ff0000000000000, 1, "Infinity"),
            (0x7ff0000000000000, 2, "Infinity"),
            (0x7ff0000000000000, 3, "Infinity"),
            (0xfff0000000000000, 0, "-Infinity"),
            (0xfff0000000000000, 1, "-Infinity"),
            (0xfff0000000000000, 2, "-Infinity"),
            (0xfff0000000000000, 3, "-Infinity"),
            (0x4011666666666666, 0, "4"),
            (0x4011666666666666, 1, "4.3"),
            (0x4011666666666666, 2, "4.35"),
            (0x4011666666666666, 3, "4.350"),
            (0x4020b0a3d70a3d71, 0, "8"),
            (0x4020b0a3d70a3d71, 1, "8.3"),
            (0x4020b0a3d70a3d71, 2, "8.35"),
            (0x4020b0a3d70a3d71, 3, "8.345"),
            (0x3ff7333333333333, 0, "1"),
            (0x3ff7333333333333, 1, "1.4"),
            (0x3ff7333333333333, 2, "1.45"),
            (0x3ff7333333333333, 3, "1.450"),
            (0x3fa70a3d70a3d70a, 0, "0"),
            (0x3fa70a3d70a3d70a, 1, "0.0"),
            (0x3fa70a3d70a3d70a, 2, "0.04"),
            (0x3fa70a3d70a3d70a, 3, "0.045"),
        ];
        for &(b, f, want) in cases {
            assert_eq!(to_fixed(f64::from_bits(b), f), want, "{:#x} {}", b, f);
        }
    }
    #[test]
    fn num_str_is_javascripts() {
        let cases: &[(u64, &str)] = &[
            (0x0, "0"),
            (0x8000000000000000, "0"),
            (0x3ff0000000000000, "1"),
            (0xbff0000000000000, "-1"),
            (0x3ff8000000000000, "1.5"),
            (0x3fb999999999999a, "0.1"),
            (0x3fd3333333333334, "0.30000000000000004"),
            (0x419d6f3454000000, "123456789"),
            (0x444b1ae4d6e2ef50, "1e+21"),
            (0x4415af1d78b58c40, "100000000000000000000"),
            (0x4454542ba12a337c, "1.5e+21"),
            (0x3e7ad7f29abcaf48, "1e-7"),
            (0x3eb0c6f7a0b5ed8d, "0.000001"),
            (0x3e8421f5f40d8376, "1.5e-7"),
            (0x405edd2f1a9fbe77, "123.456"),
            (0xbeb4b3fd5942cd96, "-0.000001234"),
            (0x1, "5e-324"),
            (0x7fefffffffffffff, "1.7976931348623157e+308"),
            (0x7ff8000000000000, "NaN"),
            (0x7ff0000000000000, "Infinity"),
            (0xfff0000000000000, "-Infinity"),
            (0x4059000000000000, "100"),
            (0x430c6bf526340000, "1000000000000000"),
            (0x441ac53a7e04bcda, "123456789012345680000"),
            (0x3eb0c6f7a0b5ed8d, "0.000001"),
            (0x4340000000000000, "9007199254740992"),
            (0x3fd5555555555555, "0.3333333333333333"),
            (0xbe25798ee2308c3a, "-2.5e-9"),
            (0x4045000000000000, "42"),
            (0x400921f9f01b866e, "3.14159"),
        ];
        for &(b, want) in cases {
            assert_eq!(num_str(f64::from_bits(b)), want, "{:#x}", b);
        }
    }
}
