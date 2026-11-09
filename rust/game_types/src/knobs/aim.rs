//! The numbers of aim.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// Every number, both sides (panel-tunable; the QA probes read them rather than copy them). AIM_DEFAULTS keeps the shipped values for the revert dots / 'defaults'.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AimK {
    pub guard: AimKGuard,
    pub player: AimKPlayer,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AimKGuard {
    /// s from nothing to fully lined up (track 0 → 1), eyes on him and the gun on him throughout — at rangeRef metres and beyond; nearer, the man fills the
    ///  sights and it goes quicker in proportion (× dist / rangeRef, no quicker than × nearK: at arm's length there is nothing to line up, he points and fires)
    #[k(1.3)] pub settle_secs: f64,
    #[k(8.0)] pub range_ref: f64,
    #[k(0.25)] pub near_k: f64,
    /// the first-shot gate: no round under this much track — 0.55 × 1.3 s ≈ 0.7 s of lining up after the reaction beat before the first one goes
    #[k(0.55)] pub acquire_min: f64,
    /// × the row's cone at track 0 (→ ×1 at track 1): the rounds of a man who has only just found you go wide, then they settle
    #[k(3.0)] pub unsettled_k: f64,
    /// 'the gun on him': facing error under this many degrees for the track to climb (the fire gate itself has always been 14°)
    #[k(18.0)] pub on_deg: f64,
    /// /s the track drains while he has Sam in sight but the gun is off him (turning onto him, legs carrying the aim clamp away)
    #[k(3.0)] pub lose_rate: f64,
    /// eyes off Sam up to this long and the track holds (a mullion, a blink of smoke); longer and it is gone — he acquires again from nothing
    #[k(0.6)] pub mem_secs: f64,
    /// Sam's bearing jumping more than this in one frame (a teleport, the tour's cuts) is a new target: track 0, and the rate below is not fed the jump
    #[k(25.0)] pub jump_deg: f64,
    /// rad of cone per rad/s of Sam's own angular speed across him (his velocity square to the line, over the range), capped: a man crossing at a sprint 7 m off
    ///  is ~0.65 rad/s → +0.24, i.e. the gun wanders a sixth of a second either side of him (the cone is ± half: K = twice the lag) — a third of those rounds still
    ///  find him, against four in five today; running straight up the bore adds nothing (measured: tools/qa aim-motion)
    #[k(0.36)] pub target_move_k: f64,
    #[k(0.26)] pub target_move_cap: f64,
    /// s, smoothing of that rate
    #[k(0.15)] pub rate_tau: f64,
    /// rad of cone per m/s of his OWN feet (the chase's 2.7 m/s → +0.27: nothing aimed leaves a running man); what he was doing OVER a walking pace (steadyPace)
    ///  is remembered feetTau seconds — a man who has just skidded to a halt out of a run is still off balance for a round or two, one who stops out of a walk (the
    ///  standoff's side-step, a patrol) is steady at once (the term reads max(now, remembered − steadyPace))
    #[k(0.1)] pub shooter_move_k: f64,
    #[k(0.5)] pub feet_tau: f64,
    #[k(1.3)] pub steady_pace: f64,
    /// m/s: moving faster than this his track cannot pass movingTrackCap — under acquireMin, so he plants to shoot
    #[k(1.2)] pub stop_to_shoot: f64,
    #[k(0.5)] pub moving_track_cap: f64,
    /// closing on Sam with a line and him inside this many metres, the gun about ready (fireCd under brakeLead s, or a burst begun): brake to a standstill,
    ///  let the round go once the track is there, close again on the weapon's beat — the plain fight's hold-and-fire inside 9 m is untouched
    #[k(14.0)] pub shoot_band: f64,
    #[k(0.45)] pub brake_lead: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AimKPlayer {
    /// s to settle fully (at rangeRef metres off what he is pointing at, quicker nearer by the guards' rule); × the cone unsettled; /s it drains with the gun off the man
    #[k(0.8)] pub settle_secs: f64,
    #[k(8.0)] pub range_ref: f64,
    #[k(0.25)] pub near_k: f64,
    #[k(2.2)] pub unsettled_k: f64,
    /// rad: his own two hands, settled, standing / crouched (± a third of a degree: 8 cm at 12 m — a torso every time, a head shot asks him to hold still)
    #[k(0.012)] pub base_stand: f64,
    #[k(0.008)] pub base_crouch: f64,
    /// the man under his reticle crossing HIS view: rad per rad/s (a guard jogging across 7 m off ≈ 0.4 rad/s → +0.08), capped; smoothing; a jump this big is a new target
    #[k(0.2)] pub target_move_k: f64,
    #[k(0.16)] pub target_move_cap: f64,
    #[k(0.15)] pub rate_tau: f64,
    #[k(25.0)] pub jump_deg: f64,
    /// rad of cone per m/s of his own feet: the walk's 1.55 m/s → +0.12 (a torso at 8 m half the time), the creep a third of that; what he was doing over a
    ///  walking pace (steadyPace) is remembered feetTau seconds — a sprint stop blooms the ring (4.7 m/s → +0.38) and it drains over the better part of a
    ///  second: catch your breath, then shoot; stopping out of a walk costs nothing past the stop
    #[k(0.08)] pub shooter_move_k: f64,
    #[k(0.6)] pub feet_tau: f64,
    #[k(1.7)] pub steady_pace: f64,
    /// the track cannot pass this at a sprint (the gun is down anyway: it is where he starts from when he stops) / with his feet moving at all; crouched it climbs × crouchK
    #[k(0.3)] pub sprint_cap: f64,
    #[k(0.7)] pub walk_cap: f64,
    #[k(0.3)] pub moving_above: f64,
    #[k(1.15)] pub crouch_k: f64,
    /// CSS px: the reticle takes a LIGHT (a fixture, the breaker, a camera) only with the cursor this near its dot — about the light itself — or on the fixture's
    ///  own drawn box (player.ts updateAim). SNAP_PX = the reach it had, the same as onto a man's chest (which also caps it); 0 = only a drawn fixture under the cursor
    #[k(14.0)] pub light_snap_px: f64,
}

/// A TRAINED man's edge (Guard.trained — the response team, game/qrf.ts; nobody else carries it, so the staff's numbers are the numbers above to the letter):
///  he lines up in settleK × the settle time (so his first round comes that much sooner past his reaction beat), his hands are spreadK × his row's cone, and
///  qrf.ts caps his reaction beats at reactCap seconds (a dazzle or a ring in the body still roots him its full length).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TrainedK {
    #[k(0.6)] pub settle_k: f64,
    #[k(0.7)] pub spread_k: f64,
    #[k(0.45)] pub react_cap: f64,
}
