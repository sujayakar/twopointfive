//! The numbers of stalk.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StalkK {
    /// m: the press is ON a man when the cursor's floor point is this near him (or the reticle has him)
    #[k(1.1)] pub cursor_m: f64,
    /// cos of his bearing to Sam under which a press can begin one (−0.25 = the grab's own rear arc, 104.5° off his facing) · and past which Sam is no longer
    ///  behind him and the stalk is over (93°: a few degrees of grace so a man swaying on his feet does not throw him out)
    #[k(-0.25)] pub begin_cos: f64,
    #[k(-0.05)] pub out_cos: f64,
    /// m: the point behind his back Sam is walked toward · inside this the hold becomes the grab
    #[k(0.55)] pub station_m: f64,
    #[k(0.95)] pub grab_m: f64,
    /// m/s: never faster than this · the closing speed on the man at `grabM` · and what each metre beyond it adds
    #[k(1.6)] pub pace: f64,
    #[k(0.3)] pub close_min: f64,
    /// 1/s: how quickly his speed takes up what the stalk asks, speeding up · and slowing (his own walk's are 6 and 12: gentler in, so the creep starts like a breath)
    #[k(4.0)] pub ease_up: f64,
    #[k(9.0)] pub ease_down: f64,
    /// THE STEP IN: the grab a stalk ends in closes the last of the gap in one step over `catchSecs` (a step longer than `catchM` takes up to `catchLong` times that: the haul back has what is left of HOLD.secs.grab; `turnIn`: he comes round to the man's back through the step, not in its first frames) —
    ///  a lunge: off the mark over its first three frames, most of the way inside a third of it, landing gently (stepEase). The grab's own way in covers a
    ///  fifth of the gap in its first FRAME. His legs are given the step, at no more than `catchStride` m/s. 0 = the grab's own.
    ///  THE YANK: once the step has landed the man is hauled `yank` m straight back the way he was facing from — Sam going back with him (or he would stay where he landed and the man come 11 cm into him) — over what is left of
    ///  the grab: movement across the floor is what the game's camera shows of a grab. Less where something solid
    ///  stands behind the pair (`yankRoom` m kept clear behind Sam); 0 = he is held where he stood.
    #[k(0.3)] pub catch_secs: f64,
    #[k(0.75)] pub catch_m: f64,
    #[k(1.1)] pub catch_long: f64,
    #[k(1.5)] pub catch_stride: f64,
    #[k(0.28)] pub yank: f64,
    #[k(0.4)] pub yank_room: f64,
    #[k(1.35)] pub corner_m: f64,
    #[k(110.0)] pub corner_deg: f64,
    /// BROKEN OFF BY THE MAN (`stalk_step`): `jolt` = how much of the animator's duck Sam gives when the man turns off his back with
    ///  the stalk on AND comes on round to face him (Sam inside `jolt_face`° of his nose within `jolt_secs` s: a patrol's quarter turn is not that) — the whole of it,
    ///  at once, when the man is onto him. 0 = none: the stalk ends and he sinks back into the crouch. `jolt_face` 180 = at any turn that ends the stalk.
    ///  THE WAIT SHOWS (`close_tick`): on the wall with the button held he coils — sunk `corner_coil` of the way into the crouch, the lead hand tucked toward the man at `corner_reach[0]`
    ///  of the grab's reach, growing to `corner_reach[1]` as the nearest man he could take comes from `corner_see` m to `corner_m`; he comes up out of the coil over
    ///  `corner_rise` s of the step in.
    #[k(0.6)] pub jolt: f64,
    #[k(50.0)] pub jolt_face: f64,
    #[k(0.6)] pub jolt_secs: f64,
    #[k(3.2)] pub corner_see: f64,
    #[k([0.25, 0.55])] pub corner_reach: [f64; 2],
    #[k(0.3)] pub corner_coil: f64,
    #[k(0.24)] pub corner_rise: f64,
    #[k(0.8)] pub key_m: f64,
    #[k(2.6)] pub low_from: f64,
    #[k(1.1)] pub low_by: f64,
    #[k(0.35)] pub low_k: f64,
    /// …its shape, all the way down (fed to the animator, whose CREEP.low has the same numbers until it is): the pelvis this much lower (m) and further back
    ///  (m) and the spine bowed this many degrees more; the share of the carriage's raised hands given up; the spring's rate, and the seconds of the grab it winds up to it over
    #[k(0.06)] pub low_drop: f64,
    #[k(0.03)] pub low_back: f64,
    #[k(6.0)] pub low_lean: f64,
    #[k(1.0)] pub low_hands: f64,
    #[k(18.0)] pub low_spring: f64,
    #[k(0.12)] pub low_wind: f64,
    #[k(0.35)] pub wide_by: f64,
    #[k(20.0)] pub heft_twist: f64,
    #[k(0.11)] pub heft_swing: f64,
    #[k(0.04)] pub heft_side: f64,
    #[k(4.0)] pub heft_lean: f64,
    #[k(0.02)] pub heft_dip: f64,
    #[k(1.0)] pub heft_surge: f64,
    #[k(0.18)] pub heft_out: f64,
    #[k(0.4)] pub heft_down: f64,
    #[k(-0.2)] pub heft_fwd: f64,
    #[k(0.0)] pub heft_lead: f64,
    #[k([0.6, 0.0, -0.8])] pub heft_pole: V3,
    #[k(3.0)] pub orbit_k: f64,
    #[k(0.05)] pub orbit_edge: f64,
    #[k(0.3)] pub orbit_snap: f64,
    /// …and with it a press can begin one from further off ("i think we can make the start radius a bit larger too"): inside `low_begin_m`, kept out to `low_out_m`,
    ///  past the close-in mode's own range (`close.mode_m`, `close.mode_out_m`: 3.0 and 3.6 m). From out there he comes on at his plain crouch walk; the mode's carriage and the sink come in inside it
    #[k(4.2)] pub low_begin_m: f64,
    #[k(5.0)] pub low_out_m: f64,
}
