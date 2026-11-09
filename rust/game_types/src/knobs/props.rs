//! The numbers of props.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// decel: Coulomb ground friction as a deceleration (m/s²) — casters roll on, a carton on carpet stops almost at once; visc: extra viscous damping (1/s);
///  rest: restitution against walls / other props; scrapeRate / thudRate: playback rate of the shared scrape / knock one-shots (a steel bin rings
///  bright, a carton knocks dull). Angular friction is derived from decel and the footprint size.
/// AN OFFICE CHAIR ROLLS ON ITS CASTORS (the maintainer: "i'd also expect the chair to have wheels and roll"). on: off in the module and on the frozen stand
///  (TPS_CHAIRROLL=1), on in the page (?chairroll=0 = the chair as it was). With it on a prop of kind 'chair' coasts on rolling resistance `decel` (m/s²) and
///  `visc` (1/s) in place of KIND.chair's 1.6 / 0.5 — pushed at a walk it rolls on about a metre and a half over two seconds where it stopped in half a metre —
///  its spin dies at `spin` / radius (rad/s²), and while it coasts the system raises PropEvent sound 'roll' (start above `startV`, stop under `stopV` m/s).
///  Every other kind, and every contact, wall and door rule, is untouched. First-pass numbers, never played.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CastorK {
    #[k(0.4)] pub decel: f64,
    #[k(0.45)] pub visc: f64,
    #[k(1.2)] pub spin: f64,
    #[k(0.3)] pub start_v: f64,
    #[k(0.1)] pub stop_v: f64,
}
