//! The numbers of pressure.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PressureK {
    /// hard: the master switch of what its men do beside their squads (defenders/pressure.rs), and of their squads' drill (enemysquads)
    #[k(false)] pub on: bool,
    /// a charge each (0 = none; on under hard on The block only, HARD_AT): every defender carries this many stun charges, the team's own kind, for the drill at a doorway (tactics/)
    #[k(0.0)] pub stuns: f64,
    /// on a hostage rescue the captor's men act on his word (missions/rescue/crew.rs)
    #[k(false)] pub rescue_crew: bool,
    /// on a mission whose men are drilled, a defender is half as shaken by a stun canister or a boot, and fires part of his blind burst down the door he was watching
    #[k(false)] pub drilled: bool,
    /// the way round a flanker will take: metres at most, and against his shortest way to that room
    #[k(46.0)] pub flank_max_m: f64,
    #[k(3.0)] pub flank_k: f64,
    #[k(14.0)] pub flank_slack_m: f64,
    /// the last man's beat before he gives a room up: s, with two of the enemy in view the whole of it
    #[k([0.35, 0.9])] pub last_man_secs: [f64; 2],
    /// s a fight heard elsewhere on the floor keeps the last man of a room that ready to give it up
    #[k(90.0)] pub loud_secs: f64,
    /// … and his beat between the boots at his door and his move
    #[k([0.8, 1.6])] pub out_secs: [f64; 2],
    /// a drilled squad that has had word of the team and then nothing sends two to go and look (enemysquads' `sweep_*`)
    #[k(false)] pub sweep: bool,
}
