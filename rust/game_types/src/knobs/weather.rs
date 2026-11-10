//! The numbers of weather.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// ---- WEATHER KINDS (weatherKinds.ts; `?weather=<kind>`, the mission list's Weather row). `Weather.kind` null = everything in this file exactly as it was: the night
/// storm, the two office footprints below, and NOTHING of the simulation touched (hearMask and airTrans answer exactly 1). With a kind set:
///   ROOFS      the no-rain footprints are the LEVEL's own ceiling slabs (roofsOf: up to four — the compound's yard and the barracks' courtyard are open sky), for the
///              rain drawn, the wet men, the rain bed in the mixer, the wind and the fog alike; the renderer gets the first two as before (weatherC / D) and the other
///              two packed in spare words of the Frame, so the Frame is the size it was.
///   FOG        one field for picture and simulation: composite.wgsl draws it along every pixel's ray, vol_common.wgsl thickens the lit haze by it,
///              and airTrans() below is its transmittance along a sight line — guards.ts / roomfight.ts / civilians.ts / devices.ts multiply it into the smoke
///              transmittance they already carry, so fog blinds an eye by the smoke's own thresholds (< 0.3 nothing, 0.3‥0.8 slower).
///   HEARING    rain is NOISE at the listener (it drowns SMALL sounds; a loud one is placed as in dry air: loudLen): hearMask() stretches every sound's path by 1 + WX_SIM.rainK × the very bed level the mixer plays there (full outdoors,
///              a third under a roof), and for WX_SIM.thunderSecs after a near thunder plays by WX_SIM.thunderK more. hearPath() is propagate() with that applied —
///              the one call the hearing model's consumers make.
///   LIGHTNING  was real light already: the renderer's meter and the eyes read it. A downpour simply has more of it, closer. The meter's CPU stand-in (headless
///              always) gets the stroke too, under a picked weather: strokeLight().
///   CLOUD      `Weather.cloud` 0‥1 — for whoever owns the sun and the moon to read; nothing here dims a light. By day hours.ts does (cloudLight: the light on open ground at
///              1 − 0.75 c^3.4 of the hour's own, moved from the sun to the sky as the deck closes, the disc spread and the colours greyed past half cover); the
///              night's moon is left as the level has it.
/// The kind rides in the checkpoint (WeatherSnap.kind) and so in a session journal's frame 0: a load / a replay runs under the weather the file was made in.
/// the simulation's weather constants (not knobs: a recording does not carry them)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WxSimK {
    #[k(1.2)] pub rain_k: f64,
    #[k(1.5)] pub thunder_k: f64,
    #[k(1.4)] pub thunder_secs: f64,
    #[k(0.45)] pub thunder_loud: f64,
    #[k(0.32)] pub roof_bed: f64,
}
