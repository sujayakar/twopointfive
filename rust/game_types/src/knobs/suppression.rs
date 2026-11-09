//! The numbers of suppression.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

/// Every number of the model and of the guards' reactions, in one place (panel-tunable; the QA probes read them rather than copy them).
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct SuppressionK {
    /// ---- the scalar (§4.1)
    /// a round passing within nearR metres of the chest adds wNear·(1 − d/nearR); one striking within hitR of it adds wHit·(1 − d/hitR) — both, for a round into the
    ///  partition beside your head. Two Five-seveN rounds into a jamb 0.6 m off a man's chest ≈ (0.27 + 0.23)·2 ≈ 1: pinned; one ≈ 0.5: a flinch.
    #[k(1.8)] pub near_r: f64,
    #[k(0.4)] pub w_near: f64,
    #[k(2.5)] pub hit_r: f64,
    #[k(0.3)] pub w_hit: f64,
    /// the first metres out of a muzzle suppress nobody: the closest approach must lie at least this far along the round's travel (colleagues abreast, the man in
    ///  Sam's arm under his own gun, and inside three metres nobody is being suppressed — he is being shot at point blank or grabbed). The doc's 1.5 doubled: a pair
    ///  advancing a stride apart must not flinch at each other's rounds.
    #[k(3.0)] pub min_travel: f64,
    /// seconds for the scalar to halve once the rounds stop (1 → under the flinch line in ~2 s, gone in ~3.5); the ceiling it is clamped to; the pulse envelope's decay (/s, R4's)
    #[k(1.1)] pub half_life: f64,
    #[k(1.0)] pub cap: f64,
    #[k(8.0)] pub kick_decay: f64,
    /// × what a round adds while the man already has his head down: you cannot pin-lock a man for ever with a metronome (with displaceAfter, the anti-lock)
    #[k(0.5)] pub down_accrual: f64,
    /// × what a COLLEAGUE's round adds to a guard (his own side's fire going past him, from behind and beside: he knows where it is going — it rattles, it does not
    ///  pin; abreast and just-ahead are already nothing by minTravel and by muzzle discipline lowering the rear gun). Sam's rounds on them and theirs on Sam: × 1.
    #[k(0.5)] pub friendly_k: f64,
    /// temperament: how readily each kind of man accrues it — the hard man shrugs off what rattles the nervy one
    #[k({ let mut t = JsMap::new(); t.set(String::from("steady"), 1.0); t.set(String::from("nervy"), 1.25); t.set(String::from("hard"), 0.8); t })] pub keen: JsMap<String, f64>,
    /// ---- guard reactions (§4.2), all behind tune.suppression
    /// s ≥ flinch: the start (anim), a bark, the next round a beat late, every round wider and the cadence slower while it lasts, no running INTO fire;
    ///  s ≥ pinned: head down — crouched behind whatever he stands at, gun lowered, feet still — for a drawn headDown seconds and then until s is under `release`;
    ///  head down displaceAfter seconds running (s never falling under flinch) and he shifts to cover instead of squatting there for good
    #[k(0.3)] pub flinch: f64,
    #[k(0.6)] pub pinned: f64,
    #[k(0.45)] pub release: f64,
    #[k([0.9, 1.5])] pub head_down: [f64; 2],
    /// fireCd floors: on the flinch's edge, and every frame the head is down (so the first round after coming up is a beat late, not on the frame)
    #[k(0.25)] pub flinch_delay: f64,
    #[k(0.6)] pub head_down_delay: f64,
    /// extra cone on a guard's round per unit s (radians, on top of his row's aiSpread: at rest 0.05 → ×2.6 at full), and × his cadence after each round (1 + gapK·s)
    #[k(0.08)] pub guard_spread: f64,
    #[k(1.0)] pub gap_k: f64,
    /// the closing run drops to the investigate jog while s ≥ flinch (he advances, but not at a sprint into fire); the standoff's orbit at half pace
    #[k(0.5)] pub standoff_pace_k: f64,
    /// displacement: after this long pinned; the cover looked for between dMin and dMax metres off, crouched-chest height there hidden from where the rounds came
    ///  from; how long he then holds that spot before the plain fight may close again; the least time between two shifts; the walk's cap and pace (× guardInvestigate);
    ///  how low he runs it (crouchTarget)
    #[k(4.0)] pub displace_after: f64,
    #[k(2.0)] pub displace_min: f64,
    #[k(4.5)] pub displace_max: f64,
    #[k(3.0)] pub cover_hold: f64,
    #[k(6.0)] pub displace_cooldown: f64,
    #[k(6.0)] pub displace_cap_secs: f64,
    #[k(1.1)] pub displace_pace: f64,
    #[k(0.55)] pub displace_crouch: f64,
    /// how long the origin of the last suppressing round stays 'where it is coming from' for the cover search (then his fix on Sam, then nothing)
    #[k(4.0)] pub src_memory_secs: f64,
    /// the talking: seconds between one man's suppression lines, between anybody's, and the window inside which two men pinned is 'he's got us pinned' (once per floorLineSecs)
    #[k(6.0)] pub bark_secs: f64,
    #[k(1.5)] pub floor_bark_secs: f64,
    #[k(2.0)] pub pair_secs: f64,
    #[k(20.0)] pub floor_line_secs: f64,
    /// ---- Sam (§4.3 — the one mechanical effect in this lane; the look and the sound are R4's)
    /// extra cone on Sam's round per unit of HIS s (radians: dead-on at 0, ± ~2° at full — a torso at 8 m mostly). Not under a puppet.
    #[k(0.07)] pub sam_spread: f64,
}
