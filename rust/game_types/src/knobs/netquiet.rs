//! The numbers of netquiet.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The shocks are morale.ts dread, 0‥1, before its nerve and cap: `unanswered` / `heardIt` a mate who does not answer (the asker / a man who heard the check), `body` / `bodyTold` a body found (the finder / the men told), `fallSeen` / `fallUnseen` a mate seen to
/// fall with / without an enemy in view, all × `steadyK` for a steady man — and ONE dead man is one shock to one man: the largest of the three, not their sum. Seconds: `askBeat` / `bodyBeat` from the event to the line, `barkSecs` the least between two of one man's
/// lines from this file (`sayHold` how long a line owed waits for that, or for a man to be out of his eyes, before it is dropped), `shooterSecs` how fresh a sighting makes the shooter 'in view', `whipShift` / `whipSecs` the whip's start and length, `whipHold` the
/// least a whip stays on one noise, `lastBeat` from the last man's line to his first step, `calmSecs` with nobody in his eyes before he starts (and roused before he asks the net himself), `alertSecs` with nothing seen or heard before a last man low on his spot
/// stops being 'alert' (he keeps the spot and the crouch), `giveUp` how long he waits for the room fight to hand him back before the act is dropped. `whipTurn` / `whipTurnBroken` rad/s; metres: `whipM` the furthest noise he whips to, `whipNearM` how near the place
/// his EARS put a noise its true source must be (with a clear line to it) for him to look at the source itself, `bodyM` how far he knows a man lying in his view, `mateM` a living mate this near and he is not alone; `arcDeg` the arc every entry should sit in;
/// `eyeDeg` the half-cone of a man's eyes (the floor's own number — guards.ts GUARD_EYE.halfDeg; the check holds the two equal) and `eyeSlack` the margin inside it he keeps his doors; `crouch` how far down he goes, 0‥1 — HALF: measured on the floor's eye, a man
/// fully down on his gun (1) looks 48° round from his aim through a cone 26° either way in plan, so past the near field he loses the very man he aims at; at 0.5 it is 42° and 47°, at 0.4 41° and 49°: what he aims at stays in his eyes, with a margin; `wary` the
/// awareness he does not fall under, low on his spot, once he is no longer alert (the floor's 'suspicious' line is 0.3; 0 = he calms right down, as a man his squad has calmed does); `backSpeed` m/s; `maxM` the furthest he walks for the spot; `doorM` the nearest he
/// sits to a door; `eyeM` his eye that far down; `stayM` how much better than where he stands a spot must score to be walked to. UNTUNED by eye (nobody has seen it on a GPU, nobody has played it).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct NetquietK {
    #[k(0.12)] pub unanswered: f64,
    #[k(0.06)] pub heard_it: f64,
    #[k(0.2)] pub body: f64,
    #[k(0.08)] pub body_told: f64,
    #[k(0.08)] pub fall_seen: f64,
    #[k(0.22)] pub fall_unseen: f64,
    #[k(0.6)] pub steady_k: f64,
    #[k(1.4)] pub ask_beat: f64,
    #[k(2.0)] pub body_beat: f64,
    #[k(3.0)] pub bark_secs: f64,
    #[k(4.0)] pub say_hold: f64,
    #[k(1.0)] pub shooter_secs: f64,
    #[k(10.0)] pub body_m: f64,
    #[k(0.12)] pub whip_shift: f64,
    #[k(1.5)] pub whip_secs: f64,
    #[k(0.8)] pub whip_hold: f64,
    #[k(10.0)] pub whip_turn: f64,
    #[k(14.0)] pub whip_turn_broken: f64,
    #[k(15.0)] pub whip_m: f64,
    #[k(4.0)] pub whip_near_m: f64,
    #[k(1.5)] pub last_beat: f64,
    #[k(2.0)] pub calm_secs: f64,
    #[k(15.0)] pub alert_secs: f64,
    #[k(8.0)] pub give_up: f64,
    #[k(5.0)] pub mate_m: f64,
    #[k(40.0)] pub arc_deg: f64,
    #[k(58.0)] pub eye_deg: f64,
    #[k(4.0)] pub eye_slack: f64,
    #[k(0.4)] pub crouch: f64,
    #[k(0.35)] pub wary: f64,
    #[k(2.0)] pub back_speed: f64,
    #[k(9.0)] pub max_m: f64,
    #[k(2.0)] pub door_m: f64,
    #[k(1.3)] pub eye_m: f64,
    #[k(1.0)] pub stay_m: f64,
}
