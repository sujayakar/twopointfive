//! The numbers of workstow.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WorkstowK {
    /// a teammate: s from his gun let go of to both hands on the job · s from the job's end to the gun back up (no round inside either) · the share of `mateUp`
    ///  after which the gun is drawn in his hands again
    #[k(0.3)] pub mate_down: f64,
    #[k(0.45)] pub mate_up: f64,
    #[k(0.6)] pub show: f64,
    /// you: s at most the job's clock waits for your hand to put the thing away (kit handling's errand; an errand something else took the arm from waits no longer) ·
    ///  s at least before a round leaves what comes back into your hand (kit handling's own draw is longer and is the lock where it is on; the rifle's own beat too)
    #[k(1.6)] pub down_max: f64,
    #[k(0.3)] pub sam_up: f64,
    /// a teammate the fight has: s at most, at a stretch, his charge waits on it — then he sets it anyway (door tools' own cap for a lock under fire: kickCap + 3)
    #[k(8.0)] pub fight_cap: f64,
}
