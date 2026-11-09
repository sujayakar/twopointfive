//! The numbers of stickycam.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// Every number of it (Tab ▸ Player & weapons ▸ Sticky cameras; in the knob registry and a journal's tables).
///  `carry` = the most a rifle is handed (kit.ts CAMS_MAX: the card's own cap, the same three) · `max` = the most that stay out on their walls (one more sent
///  retires the oldest) · `live` = how many of those SEND A PICTURE, the newest first (1: his headset shows one feed — sending the next camera gives the last
///  one's view up; `max` or more = every camera out is an eye, as it first shipped) · `issue` = what a mission
///  started the 'cameras' way hands him when his card carries fewer · the flight (a Ballistics: speed m/s, gravity, drag /s, contact radius m, seconds, substeps
///  a frame) · `cd` s before the tube's next round · `aimY` m: the height it flies over the spot the cursor names (above a head, under a door's head) ·
///  `fovDeg`: a wall camera's wedge about the wall's normal (a camera lying flat sees all round) · `off` m: how far off its surface the lens stands ·
///  `flatY`: |normal.y| past which a surface is 'flat' · hearing: the tube at the shooter, the tick where it sticks ('prop' events, who 0 for the tick).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StickyK {
    #[k(3.0)] pub carry: f64,
    #[k(3.0)] pub max: f64,
    #[k(1.0)] pub live: f64,
    #[k(2.0)] pub issue: f64,
    #[k(26.0)] pub speed: f64,
    #[k(9.81)] pub gravity: f64,
    #[k(0.5)] pub drag: f64,
    #[k(0.03)] pub radius: f64,
    #[k(1.6)] pub life: f64,
    #[k(4.0)] pub substeps: f64,
    #[k(2.0)] pub solve_iters: f64,
    #[k(1.2)] pub cd: f64,
    #[k(1.9)] pub aim_y: f64,
    #[k(110.0)] pub fov_deg: f64,
    #[k(0.06)] pub off: f64,
    #[k(0.7)] pub flat_y: f64,
    #[k(0.3)] pub launch_noise: f64,
    #[k(0.12)] pub stick_noise: f64,
    #[k(1.3)] pub take_reach: f64,
    #[k(0.1)] pub take_noise: f64,
}
