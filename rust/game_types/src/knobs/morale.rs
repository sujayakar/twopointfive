//! The numbers of morale.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MoraleKDarkRead {
    #[default] Place,
    Watch,
}

/// Dread is 0‥1: `jumpy` / `broken` are the rungs' lines, `hyst` how far under its line a rung holds. `blast` is a blast's shock inside `nearM`, × `farK` by `blastM`, `shockCap` the most shock a man carries; `alone` / `dark` the two standing terms. The rest are seconds, metres,
/// metres a second and rounds, named as the head of the file names them; `barkSecs` = the least time between two of one man's lines from this file. `fanDeg` / `fanStep` / `straightM`: the scrabble's fan (backGoal). TRAINING: `training` = who is green — 'difficulty' (the squad's
/// own key, else MORALE_AT), 'green' / 'mixed' / 'trained' for every squad, 'none' = the first slice's men; `greenBroken` = a green man's broken line; `leaderDown` his standing term with his leader down, `leaderShock` the shock of seeing it, `flinchSecs` the flinch; `shoutGap`
/// how long the duck's shout waits on a line already in his mouth; `orderM` / `orderBeat` / `orderSecs` / `orderGap` / `standSecs` / `sideM` the leader's order (the head), `steadyDuck` a steady man's duck; `holdShock` / `holdMax` end a break (the head); `nerve` picks the row of
/// `nerveK` every shock is scaled by. UNTUNED by eye (nobody has seen it on a GPU, nobody has played it).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MoraleK {
    #[k(25.0)] pub half_life: f64,
    #[k(0.25)] pub ask_secs: f64,
    #[k(0.25)] pub alone: f64,
    #[k(7.0)] pub alone_m: f64,
    #[k(0.25)] pub dark: f64,
    #[k(3.0)] pub lit_asks: f64,
    #[k(0.35)] pub blast: f64,
    #[k(0.5)] pub shock_cap: f64,
    #[k(10.0)] pub blast_m: f64,
    #[k(4.0)] pub near_m: f64,
    #[k(0.6)] pub far_k: f64,
    #[k(0.3)] pub jumpy: f64,
    #[k(0.7)] pub broken: f64,
    #[k(0.15)] pub hyst: f64,
    #[k(2.5)] pub duck_secs: f64,
    #[k(0.6)] pub down_secs: f64,
    #[k(2.2)] pub back_until: f64,
    #[k(3.5)] pub back_m: f64,
    #[k(2.0)] pub back_speed: f64,
    #[k(5.0)] pub burst_rounds: f64,
    #[k(0.2)] pub burst_every: f64,
    #[k(0.3)] pub spread: f64,
    #[k(6.0)] pub call2_secs: f64,
    #[k(2.0)] pub bait_m: f64,
    #[k(5.0)] pub bait_short: f64,
    #[k(0.1)] pub bait_every: f64,
    #[k(3.2)] pub reload_secs: f64,
    #[k(2.0)] pub bark_secs: f64,
    #[k(70.0)] pub fan_deg: f64,
    #[k(17.5)] pub fan_step: f64,
    #[k(1.5)] pub straight_m: f64,
    #[k(crate::morale::Training::Difficulty)] pub training: crate::morale::Training,
    #[k(0.5)] pub green_broken: f64,
    #[k(0.25)] pub leader_down: f64,
    #[k(0.3)] pub leader_shock: f64,
    #[k(0.8)] pub flinch_secs: f64,
    #[k(0.6)] pub shout_gap: f64,
    #[k(4.0)] pub order_m: f64,
    #[k(0.5)] pub order_beat: f64,
    #[k(1.0)] pub order_secs: f64,
    #[k(0.15)] pub order_gap: f64,
    #[k(2.5)] pub stand_secs: f64,
    #[k(0.75)] pub side_m: f64,
    #[k(1.0)] pub steady_duck: f64,
    #[k(0.05)] pub hold_shock: f64,
    #[k(70.0)] pub hold_max: f64,
    #[k(crate::morale::Nerve::Normal)] pub nerve: crate::morale::Nerve,
    pub nerve_k: MoraleKNerveK,
    /// how the dark is read (`in_the_dark`): 'place' = where he STANDS; 'watch' = `dark_foe_blind`, the floor he is WATCHING
    #[k(MoraleKDarkRead::Place)] pub dark_read: MoraleKDarkRead,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MoraleKNerveK {
    #[k(1.4)] pub brittle: f64,
    #[k(1.0)] pub normal: f64,
    #[k(0.7)] pub steady: f64,
}
