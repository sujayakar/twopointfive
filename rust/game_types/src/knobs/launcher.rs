//! The numbers of launcher.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// Every number of the ring's flight and of what it does (panel-tunable; the QA probes read them rather than copy them).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RingK {
    /// muzzle speed (m/s), gravity (m/s²), air drag (/s: v ×= e^(−drag·t)), the ring's contact radius (m), seconds of flight before it is spent regardless, substeps a frame
    #[k(30.0)] pub speed: f64,
    #[k(9.81)] pub gravity: f64,
    #[k(0.4)] pub drag: f64,
    #[k(0.04)] pub radius: f64,
    #[k(1.5)] pub life: f64,
    #[k(4.0)] pub substeps: f64,
    /// where on a man is 'out cold': the hit point no lower than this far under his neck joint (the throat, the jaw, the skull, the top of the sternum — CT-generous);
    ///  anything lower staggers. The body is Character.rayHit's cylinder (0.28 about the hips, soles to crown), the same body a round finds.
    #[k(0.07)] pub ko_below_neck: f64,
    /// the stagger: seconds doubled over (Guard.staggerUntil — rooted, gun down, no round; guards.ts), his trigger dead at least this long, the awareness it leaves him
    ///  with (0.92: alert — somebody just shot him with something), and how far back along the ring's line his fix on 'where from' is put when he has no better one
    #[k(1.5)] pub stagger: f64,
    #[k(1.5)] pub stagger_fire_cd: f64,
    #[k(0.92)] pub stagger_aware: f64,
    #[k(4.0)] pub stagger_back: f64,
    /// hearing ('prop' events, the room's business — guards.ts hearingCheck's door / furniture branch: ~9 m of path at 0.45, ~6 at 0.3): the launcher going off AT
    ///  THE SHOOTER, the ring clacking off a wall / a desk at the impact (who 0: a noise the room cannot explain — men go and look at IT, which is half of what the thing
    ///  is for), a man folding under a head hit (the takedown's 0.3, at him), the thump and yelp of a body hit
    #[k(0.45)] pub launch_noise: f64,
    #[k(0.3)] pub clack_noise: f64,
    #[k(0.3)] pub ko_noise: f64,
    #[k(0.35)] pub hit_noise: f64,
    /// N·s into a chair or a carton it strikes (it hops); how much of its normal speed a ring keeps coming back off a wall and how much of its slide along it (rubber
    ///  at thirty metres a second: most of it goes in the slap), and the most it may carry into throwables' hands after — so it drops and skitters by the mark it
    ///  made its noise at, where the men who heard it will look, rather than skating off down the corridor
    #[k(3.0)] pub prop_j: f64,
    #[k(0.2)] pub restitution: f64,
    #[k(0.25)] pub keep_tangent: f64,
    #[k(5.0)] pub spent_speed_max: f64,
    /// aim: lead the man under the reticle by his ground velocity (one iteration), and solve the launch elevation so the arc passes through the aim point (two)
    #[k(2.0)] pub solve_iters: f64,
}
