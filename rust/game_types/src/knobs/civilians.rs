//! The numbers of civilians.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// ================================================================ numbers (Tab ▸ Sandbox ▸ civilians rows read a few; the switch and killFails persist per machine)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CivK {
    /// her eye against the guards' (guards.ts GUARD_EYE 16 m / 58° / near field 3.5 m of 100°): walking about; head down over a task; once frightened (looking for it)
    pub eye: CivKEye,
    pub eye_busy: CivKEyeBusy,
    pub eye_alarmed: CivKEyeAlarmed,
    /// × the guards' sight gain on Sam (a civilian is not looking for anyone); × that again for a gun in his hand seen inside gunM (or him doing violence)
    #[k(0.6)] pub keen_k: f64,
    #[k(2.5)] pub gun_k: f64,
    #[k(10.0)] pub gun_m: f64,
    /// awareness lines: over glimpseAt she stops and looks ('hello…?'); over spotAt she has SEEN him — the guards' own alert line
    #[k(0.3)] pub glimpse_at: f64,
    #[k(0.85)] pub spot_at: f64,
    /// the startle: how long she stands looking before she talks herself out of it; a second inside this and it is no longer nothing
    #[k(4.0)] pub startle_secs: f64,
    #[k(20.0)] pub startle_again_secs: f64,
    /// bodies: seen inside this range in her cone with a clear line to the pelvis (no light asked, as the men's own body test asks none)
    #[k(10.0)] pub body_m: f64,
    /// the frozen beat before she runs (dealt per civilian and fright inside the window), her run, her walk, the crouched creep into a corner
    #[k([0.4, 0.8])] pub freeze: [f64; 2],
    #[k(3.2)] pub run_speed: f64,
    #[k(1.05)] pub walk_speed: f64,
    #[k(1.2)] pub creep_speed: f64,
    /// the report: she stops this close to the man (in sight of him) and takes this long telling him; the phone call's length
    #[k(2.2)] pub report_m: f64,
    #[k(2.6)] pub report_secs: f64,
    #[k(6.0)] pub phone_secs: f64,
    /// target choice: metres of path a GUARD is worth over a phone (a person beats a handset), what leaving the floor costs over either (the last resort), and what a
    ///  target the far side of the fright costs (she does not run past him to reach it)
    #[k(4.0)] pub guard_bonus_m: f64,
    #[k(6.0)] pub exit_malus_m: f64,
    #[k(15.0)] pub past_threat_m: f64,
    /// cowering: the corner is looked for within this; the man with the gun inside refleeM of her (and seen) puts her up and off again, once; not told to stay and
    ///  nobody come for her in loneSecs, she goes looking for a guard again; told and the floor calm again for settleSecs, back to work
    #[k(4.0)] pub corner_m: f64,
    #[k(6.0)] pub reflee_m: f64,
    #[k(45.0)] pub lone_secs: f64,
    #[k(75.0)] pub settle_secs: f64,
    /// 'stay down, stay quiet': minutes she stays put and tells nobody (unless a man walks onto her)
    #[k(3.0)] pub comply_min: f64,
    /// the men: a scream carries this far by the sound's own path (× its carry); a running / screaming woman is SEEN inside seeM in his cone if lit past seeLit on her own
    ///  meter (or inside 4 m); one crouched in a corner is found inside foundM with a clear line, and asked after foundSecs
    #[k(16.0)] pub scream_m: f64,
    #[k(12.0)] pub see_m: f64,
    #[k(0.25)] pub see_lit: f64,
    #[k(3.0)] pub found_m: f64,
    #[k(1.2)] pub found_secs: f64,
    /// control rings the floor this long after a civilian gets out of the building
    #[k(18.0)] pub outside_call_secs: f64,
    #[k(3.0)] pub greet_m: f64,
    #[k(240.0)] pub greet_pair_secs: f64,
    #[k(70.0)] pub greet_floor_secs: f64,
}

/// her eye against the guards' (guards.ts GUARD_EYE 16 m / 58° / near field 3.5 m of 100°): walking about; head down over a task; once frightened (looking for it)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CivKEye {
    #[k(12.0)] pub range: f64,
    #[k(50.0)] pub half_deg: f64,
    #[k(2.5)] pub near_m: f64,
    #[k(95.0)] pub near_deg: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CivKEyeBusy {
    #[k(8.0)] pub range: f64,
    #[k(34.0)] pub half_deg: f64,
    #[k(2.0)] pub near_m: f64,
    #[k(80.0)] pub near_deg: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CivKEyeAlarmed {
    #[k(14.0)] pub range: f64,
    #[k(62.0)] pub half_deg: f64,
    #[k(3.0)] pub near_m: f64,
    #[k(100.0)] pub near_deg: f64,
}
