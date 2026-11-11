//! The numbers of whistle.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// Every number of it (panel-tunable; the QA probes read them rather than copy them).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WhistleK {
    /// the hearing-model level stamped on the event and shown on the SOUND meter (a crouched step on lino is ~0.2, a walk 0.33, the suppressed pistol 0.6)
    #[k(0.35)] pub level: f64,
    /// metres of OPEN AIR it carries (the propagated path must be shorter than this × the route's carry: 1 direct, 0.9 over a partition, 0.72 round a corner,
    ///  0.32 through a shut door, 0.25 through the mass) — × KEEN on an alarmed floor, as every small noise
    #[k(8.0)] pub reach_k: f64,
    /// straight-line pre-filter (m): nobody further off than this is even asked (the propagation is the dear part)
    #[k(12.0)] pub horizon: f64,
    /// seconds between two whistles (V inside it does nothing)
    #[k(2.0)] pub cooldown: f64,
    /// the same man answering a second one inside this many seconds searches the place properly instead of taking another look
    #[k(60.0)] pub twice_window: f64,
    /// what answering it leaves him with: the suspicion of a creak (under the 0.85 that is CONTACT, over the 0.3 that is 'huh?'), and the beat he takes to place it
    #[k(0.45)] pub awareness: f64,
    #[k(0.5)] pub react_t: f64,
    /// while he is still WALKING over to one, his suspicion is held at least this high (the guards' suspicious branch: it wears off otherwise, and a spot
    ///  eight metres off would have him give up two strides short of it); arrived, it drains from here through the look round — ~3 s of torch into the corners — and he
    ///  talks himself out of it ('must have been nothing') and goes back. 0 = the creak's behaviour exactly.
    #[k(0.32)] pub walk_floor: f64,
    /// the others who heard it: how long their heads stay turned toward it (s), and how far round a neck goes for it (rad)
    #[k(1.5)] pub glance_secs: f64,
    #[k(1.2)] pub glance_max: f64,
    /// the voice's gain at Sam's head (sfx.ts 'whistle' peaks ~0.3: quiet — it is for the man six metres off, not the floor)
    #[k(0.55)] pub gain: f64,
}
