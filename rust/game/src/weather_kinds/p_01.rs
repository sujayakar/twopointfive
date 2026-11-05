//! src/game/weatherKinds.ts lines 1 to 192
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Every word of WeatherKind: the enum has no parser, and `weather_of_search` holds no game to walk WEATHER_PRESETS' keys with.
const ALL_KINDS: [WeatherKind; 7] = [
    WeatherKind::Clear,
    WeatherKind::Overcast,
    WeatherKind::Rain,
    WeatherKind::Downpour,
    WeatherKind::Fog,
    WeatherKind::Dust,
    WeatherKind::Snow,
];

/// `URLSearchParams.get(key)` of a query the host holds. (weatherKinds.ts:110, :175, :180)
fn seam_query_get(q: crate::Opaque, key: &str) -> Option<String> {
    todo!("needs: URLSearchParams as a type that can answer get(key)")
}

/// The kind a text names, of the kinds a level can be started in.
fn startable_kind(s: &str) -> Option<WeatherKind> {
    if !is_weather_kind(Some(s)) {
        return None;
    }
    ALL_KINDS.iter().copied().find(|k| k.as_str() == s)
}

/// One side of a `key=value` pair of a query, as URLSearchParams decodes it: + is a space, %XX a byte, the bytes UTF-8 with bad runs replaced.
fn form_decode(s: &str) -> String {
    let b = s.as_bytes();
    let hex = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'+' {
            out.push(b' ');
            i += 1;
            continue;
        }
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(hi), Some(lo)) = (hex(b[i + 1]), hex(b[i + 2])) {
                out.push(hi * 16 + lo);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// `new URLSearchParams(search).get(key)`: the first pair of that key, decoded.
fn text_query_get(search: &str, key: &str) -> Option<String> {
    let body = search.strip_prefix('?').unwrap_or(search);
    for pair in body.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        if form_decode(name) == key {
            return Some(form_decode(value));
        }
    }
    None
}

/// `/^[\w.-]+\.json$/`: a plain file name of a level. (weatherKinds.ts:110)
fn is_level_file(lf: &str) -> bool {
    match lf.strip_suffix(".json") {
        Some(stem) => !stem.is_empty() && stem.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'.' || c == b'-'),
        None => false,
    }
}

/// The first `=`-side of a query's pair, as weatherSearch compares it.
fn pair_key(p: &str) -> &str {
    p.split('=').next().unwrap_or("")
}

/// Is this one of the kinds a level can be started in? (weatherKinds.ts isWeatherKind)
pub fn is_weather_kind(s: Option<&str>) -> bool {
    match s {
        Some(t) => weather_kinds().iter().any(|k| k == t),
        None => false,
    }
}

/// Is the level called `name` one built for snow, with the switch on? (weatherKinds.ts snowLevel)
pub fn snow_level(g: &mut crate::game::Game, name: Option<&str>) -> bool {
    if !g.k.snow.on {
        return false;
    }
    match name {
        Some(n) => !n.is_empty() && snow_levels().iter().any(|l| l == n),
        None => false,
    }
}

/// snowLevel asked of a page's query: its `levelFile`. (weatherKinds.ts snowLevelOfQuery)
pub fn snow_level_of_query(g: &mut crate::game::Game, q: crate::Opaque) -> bool {
    let lf = match seam_query_get(q, "levelFile") {
        Some(lf) => lf,
        None => return false,
    };
    if !is_level_file(&lf) {
        return false;
    }
    snow_level(g, lf.strip_suffix(".json"))
}

/// A kind a checkpoint may carry: one of the row's, or a level's own. (weatherKinds.ts isAnyKind)
pub fn is_any_kind(g: &mut crate::game::Game, s: Option<crate::weather_kinds::WeatherKind>) -> bool {
    match s {
        Some(k) => g.m.weather_kinds.weather_presets.has(k),
        None => false,
    }
}

/// The kind's name for the mission list. (weatherKinds.ts weatherTitle)
pub fn weather_title(g: &mut crate::game::Game, k: Option<crate::weather_kinds::WeatherKind>) -> String {
    match k {
        Some(k) => g.m.weather_kinds.weather_presets.get(k).unwrap().title.clone(),
        None => String::from("As it is"),
    }
}

/// The kind's line for the mission list. (weatherKinds.ts weatherWords)
pub fn weather_words(g: &mut crate::game::Game, k: Option<crate::weather_kinds::WeatherKind>) -> String {
    match k {
        Some(k) => g.m.weather_kinds.weather_presets.get(k).unwrap().words.clone(),
        None => String::from("the level's own weather, as it has always played"),
    }
}

/// `x` in steps of `step`, held to 0‥max. (weatherKinds.ts q)
pub fn q(x: f64, step: f64, max: f64) -> f64 {
    m::max(0.0, m::min(max, m::round(x / step)))
}

/// The fog in 32 bits; 0 = none. (weatherKinds.ts packFog)
pub fn pack_fog(f: Option<crate::weather_kinds::FogSpec>) -> f64 {
    let f = match f {
        Some(f) if f.sigma > 0.0 => f,
        _ => return 0.0,
    };
    let s = m::max(1.0, q(f.sigma, 0.001, 255.0));
    let h = q(f.h, 0.5, 31.0);
    let l = q(f.leak, 1.0 / 7.0, 7.0);
    let gain = q(f.gain, 0.25, 15.0);
    let t = |i: usize| js::to_u32(q(f.tint[i], 1.0 / 15.0, 15.0));
    let w = js::to_u32(s)
        | js::to_u32(h) << 8
        | js::to_u32(l) << 13
        | t(0) << 16
        | t(1) << 20
        | t(2) << 24
        | js::to_u32(gain) << 28;
    w as f64
}

/// The fog's word with its glow (the gain) × k, on the gain's own steps and never to nothing. (weatherKinds.ts fogGlowWord)
pub fn fog_glow_word(w: f64, k: f64) -> f64 {
    let w = js::to_u32(w);
    if w == 0 || k == 1.0 {
        return w as f64;
    }
    let gain = m::max(1.0, m::min(15.0, m::round((w >> 28) as f64 * k)));
    ((w & 0x0FFF_FFFF) | js::to_u32(gain) << 28) as f64
}

/// The numbers the fog's word holds; the simulation reads these, not the preset's. (weatherKinds.ts unpackFog)
pub fn unpack_fog(w: f64) -> Option<crate::weather_kinds::FogSpec> {
    let w = js::to_u32(w);
    if w == 0 {
        return None;
    }
    Some(FogSpec {
        sigma: (w & 255) as f64 * 0.001,
        h: ((w >> 8) & 31) as f64 * 0.5,
        leak: ((w >> 13) & 7) as f64 / 7.0,
        tint: [((w >> 16) & 15) as f64 / 15.0, ((w >> 20) & 15) as f64 / 15.0, ((w >> 24) & 15) as f64 / 15.0],
        gain: (w >> 28) as f64 * 0.25,
    })
}

/// Optical depth of the fog along a → b, before any roof. (weatherKinds.ts fogTau)
pub fn fog_tau(f: crate::weather_kinds::FogSpec, a: V3, b: V3) -> f64 {
    let l = m::hyp3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if !(l > 0.0) {
        return 0.0;
    }
    if !(f.h > 0.0) {
        return f.sigma * l;
    }
    let ya = m::max(a[1], 0.0) / f.h;
    let k = (m::max(b[1], 0.0) - m::max(a[1], 0.0)) / f.h;
    f.sigma * m::exp(-ya) * l * (if m::abs(k) < 1e-3 { 1.0 - 0.5 * k } else { (1.0 - m::exp(-k)) / k })
}

/// A roof's footprint in 32 bits, rounded outward; 0 = none. (weatherKinds.ts packRect)
pub fn pack_rect(r: Option<[f64; 4]>) -> f64 {
    let r = match r {
        Some(r) if r[2] > r[0] && r[3] > r[1] => r,
        _ => return 0.0,
    };
    let lo = |x: f64| js::to_u32(m::max(0.0, m::min(255.0, m::floor(x / ROOF_Q + 1e-6))));
    let hi = |x: f64| js::to_u32(m::max(0.0, m::min(255.0, m::ceil(x / ROOF_Q - 1e-6))));
    (lo(r[0]) | lo(r[1]) << 8 | hi(r[2]) << 16 | hi(r[3]) << 24) as f64
}

/// The footprint a roof's word holds. (weatherKinds.ts unpackRect)
pub fn unpack_rect(w: f64) -> [f64; 4] {
    let w = js::to_u32(w);
    [(w & 255) as f64 * ROOF_Q, ((w >> 8) & 255) as f64 * ROOF_Q, ((w >> 16) & 255) as f64 * ROOF_Q, (w >> 24) as f64 * ROOF_Q]
}

/// Is (x, z) in the footprint, edges included? (weatherKinds.ts inRect)
pub fn in_rect(r: &[f64], x: f64, z: f64) -> bool {
    x >= r[0] && z >= r[1] && x <= r[2] && z <= r[3]
}

/// The kind a query string or the host's query names, if any. (weatherKinds.ts weatherOfSearch)
pub fn weather_of_search(search: &crate::weather_kinds::WeatherOfSearchSearch) -> Option<crate::weather_kinds::WeatherKind> {
    let v = match search {
        WeatherOfSearchSearch::Text(s) => text_query_get(s, WEATHER_KEY),
        WeatherOfSearchSearch::URLSearchParams(q) => seam_query_get(*q, WEATHER_KEY),
    };
    startable_kind(&v?)
}

/// Reads ?weather= at boot and locks the page to it; an unknown name boots the level as it is. (weatherKinds.ts initWeather)
pub fn init_weather(g: &mut crate::game::Game, qs: crate::Opaque) -> Option<crate::weather_kinds::WeatherKind> {
    let v = seam_query_get(qs, WEATHER_KEY);
    g.m.weather_kinds.weather_boot.locked = true;
    let kind = v.and_then(|v| startable_kind(&v));
    g.m.weather_kinds.weather_boot.kind = kind;
    kind
}

/// The query string with the weather put on it (None = taken off), every other flag kept in order. (weatherKinds.ts weatherSearch)
pub fn weather_search(search: &str, kind: Option<crate::weather_kinds::WeatherKind>) -> String {
    let body = search.strip_prefix('?').unwrap_or(search);
    let all: Vec<&str> = body.split('&').collect();
    let mut parts: Vec<String> = all.iter().filter(|p| !p.is_empty() && pair_key(p) != WEATHER_KEY).map(|p| String::from(*p)).collect();
    if kind.is_none() && !all.iter().any(|p| pair_key(p) == WEATHER_KEY) {
        return String::from(search);
    }
    if let Some(k) = kind {
        parts.push(format!("{}={}", WEATHER_KEY, k.as_str()));
    }
    format!("?{}", parts.join("&"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fog(k: WeatherKind) -> FogSpec {
        WeatherKindsMod::blank().weather_presets.get(k).unwrap().fog.unwrap()
    }
    fn bits(v: &[f64]) -> Vec<u64> {
        v.iter().map(|x| x.to_bits()).collect()
    }
    const SEGS: [(V3, V3); 3] = [([1.0, 0.5, 2.0], [30.25, 1.75, -4.0]), ([0.0, 3.0, 0.0], [0.0, 3.001, 0.5]), ([-2.0, -1.0, 4.0], [7.0, 12.0, 9.0])];

    /// The TypeScript's own words and bits, under bun with EXACT on.
    #[test]
    fn fog_words_and_depths() {
        let rows: [(WeatherKind, f64, f64, f64, [u64; 7], [u64; 3], u64); 3] = [
            (
                WeatherKind::Downpour, 1072507924.0, 1609378836.0, 535637012.0,
                [0x3f947ae147ae147b, 0x4018000000000000, 0x3fc2492492492492, 0x3febbbbbbbbbbbbc, 0x3fedddddddddddde, 0x3ff0000000000000, 0x3fe8000000000000],
                [0x3fdfc530d53c74cf, 0x3f78d76aeb699239, 0x3fc25a8d430445b1],
                0x3fe32064036c29b3,
            ),
            (
                WeatherKind::Fog, 1341007217.0, 2146313585.0, 535700849.0,
                [0x3fbced916872b021, 0x4004000000000000, 0x3fc2492492492492, 0x3fedddddddddddde, 0x3fedddddddddddde, 0x3ff0000000000000, 0x3ff0000000000000],
                [0x40016803806ab703, 0x3f916c2528868b1d, 0x3fd8c7950998461c],
                0x400b0426de6f2e19,
            ),
            (
                WeatherKind::Dust, 1221546014.0, 2026852382.0, 416239646.0,
                [0x3f9eb851eb851eb8, 0x4020000000000000, 0x3fd2492492492492, 0x3ff0000000000000, 0x3fe999999999999a, 0x3fe1111111111111, 0x3ff0000000000000],
                [0x3fe8f3947f8a029f, 0x3f851cbc5e4e517b, 0x3fd07d7a66688a0e],
                0x3fecb09605223e8c,
            ),
        ];
        for (k, word, up, down, un, taus, flat) in rows {
            let f = fog(k);
            let w = pack_fog(Some(f));
            assert_eq!(w, word);
            assert_eq!(fog_glow_word(w, 1.7), up);
            assert_eq!(fog_glow_word(w, 0.1), down);
            let u = unpack_fog(w).unwrap();
            assert_eq!(bits(&[u.sigma, u.h, u.leak, u.tint[0], u.tint[1], u.tint[2], u.gain]), un);
            for (i, (a, b)) in SEGS.iter().enumerate() {
                assert_eq!(fog_tau(f, *a, *b).to_bits(), taus[i]);
            }
            assert_eq!(fog_tau(FogSpec { h: 0.0, ..f }, SEGS[0].0, SEGS[0].1).to_bits(), flat);
        }
        assert_eq!(pack_fog(None), 0.0);
        assert!(unpack_fog(0.0).is_none());
    }

    #[test]
    fn glow_edges() {
        assert_eq!(fog_glow_word(-1.0, 2.0).to_bits(), 0x41efffffffe00000);
        assert_eq!(fog_glow_word(0.0, 2.0), 0.0);
        assert_eq!(fog_glow_word(268435457.0, 1.0), 268435457.0);
        assert_eq!(fog_glow_word(268435457.0, f64::NAN), 1.0);
    }

    #[test]
    fn roofs() {
        let rows: [([f64; 4], f64, [u64; 4]); 4] = [
            ([3.1, 4.2, 20.05, 30.6], 2573538575.0, [0x4008000000000000, 0x4010cccccccccccd, 0x4034333333333334, 0x403e99999999999a]),
            ([0.0, 0.0, 51.0, 51.0], 4294901760.0, [0, 0, 0x4049800000000000, 0x4049800000000000]),
            ([-1.0, 2.0, 300.0, 4.0000001], 352258560.0, [0, 0x4000000000000000, 0x4049800000000000, 0x4010000000000000]),
            ([1.2, 0.4, 1.4, 0.6], 50790918.0, [0x3ff3333333333334, 0x3fd999999999999a, 0x3ff6666666666667, 0x3fe3333333333334]),
        ];
        for (r, word, back) in rows {
            let w = pack_rect(Some(r));
            assert_eq!(w, word);
            assert_eq!(bits(&unpack_rect(w)), back);
        }
        assert_eq!(pack_rect(Some([2.0, 0.0, 1.0, 5.0])), 0.0);
        assert_eq!(pack_rect(None), 0.0);
    }

    #[test]
    fn search_strings() {
        let rows: [(&str, Option<WeatherKind>, &str); 8] = [
            ("?a=1&weather=fog&b", Some(WeatherKind::Rain), "?a=1&b&weather=rain"),
            ("?a=1&weather=fog&b", None, "?a=1&b"),
            ("?a=1", None, "?a=1"),
            ("", None, ""),
            ("", Some(WeatherKind::Rain), "?weather=rain"),
            ("a&&b=2&weatherx=1", Some(WeatherKind::Dust), "?a&b=2&weatherx=1&weather=dust"),
            ("?", None, "?"),
            ("?weather", None, "?"),
        ];
        for (s, k, want) in rows {
            assert_eq!(weather_search(s, k), want);
        }
    }

    #[test]
    fn kinds_of_search() {
        let rows: [(&str, Option<WeatherKind>); 8] = [
            ("?weather=fog", Some(WeatherKind::Fog)),
            ("weather=%66og&weather=rain", Some(WeatherKind::Fog)),
            ("a=1&weather=snow", None),
            ("wea%74her=rain", Some(WeatherKind::Rain)),
            ("?weather=", None),
            ("??weather=fog", None),
            ("x=1&weather=dust#z", None),
            ("weather=+fog", None),
        ];
        for (s, want) in rows {
            assert_eq!(weather_of_search(&WeatherOfSearchSearch::Text(String::from(s))), want);
        }
    }

    #[test]
    fn kinds_and_words() {
        for k in ["clear", "overcast", "rain", "downpour", "fog", "dust"] {
            assert!(is_weather_kind(Some(k)));
        }
        assert!(!is_weather_kind(Some("snow"))); // a level's own, never picked
        assert!(!is_weather_kind(Some("default")));
        assert!(!is_weather_kind(None));
        let mut g = crate::game::Game::blank();
        for k in ALL_KINDS {
            assert!(is_any_kind(&mut g, Some(k)));
        }
        assert!(!is_any_kind(&mut g, None));
        assert_eq!(weather_title(&mut g, Some(WeatherKind::Snow)), "Snow");
        assert_eq!(weather_title(&mut g, None), "As it is");
        assert_eq!(weather_words(&mut g, Some(WeatherKind::Fog)), "ground fog, so nobody can see past about twenty meters outdoors");
        assert_eq!(weather_words(&mut g, None), "the level's own weather, as it has always played");
        g.k.snow.on = true;
        assert!(snow_level(&mut g, Some("tut-road")));
        assert!(!snow_level(&mut g, Some("office")));
        assert!(!snow_level(&mut g, Some("")));
        assert!(!snow_level(&mut g, None));
        g.k.snow.on = false;
        assert!(!snow_level(&mut g, Some("pass")));
    }

    #[test]
    fn level_files() {
        assert!(is_level_file("pass.json"));
        assert!(is_level_file("a.b-c_1.json"));
        assert!(!is_level_file(".json"));
        assert!(!is_level_file("pass.json\n"));
        assert!(!is_level_file("a/b.json"));
        assert!(!is_level_file("pass"));
    }
}
