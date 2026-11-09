//! The numbers of doze.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use game_derive::Knobs;

/// the switch and its numbers. `minWait` s: the shortest wait he sits one out for · `maxWait` s: the longest (a man stood somewhere longer than this is POSTED
///  there: a picket, not a man pausing on his round) · `chairM`: how far from his waypoint the chair may stand (straight; the walk to
///  it may be `chairM` × 1.6) · `nodSecs`: seated this long his eyes close · `riseSecs`: he is up this long before the wait ends · `hearK`: his ears asleep, as a
///  share of a waking man's (the small noises: a step, a door, furniture, glass, a silenced round) · `lightAt` / `lightSecs`: light on his face, on the meter's 0‥1 scale, for this
///  long · `askSecs`: how often the light is read · `wakeAw`: the awareness he wakes with (0.3 = suspicious, 0.85 = alert) · `bumpAw`: the same for a shoved chair
///  · `stirSecs`: rooted while he comes round · `breathSecs`: one breath (the 'z' grows over it). UNTUNED by eye and by hand: first numbers.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct DozeK {
    #[k(20.0)] pub min_wait: f64,
    #[k(90.0)] pub max_wait: f64,
    #[k(4.5)] pub chair_m: f64,
    #[k(2.5)] pub nod_secs: f64,
    #[k(2.0)] pub rise_secs: f64,
    #[k(0.6)] pub hear_k: f64,
    #[k(0.3)] pub light_at: f64,
    #[k(0.8)] pub light_secs: f64,
    #[k(0.25)] pub ask_secs: f64,
    #[k(0.55)] pub wake_aw: f64,
    #[k(0.7)] pub bump_aw: f64,
    #[k(1.3)] pub stir_secs: f64,
    #[k(3.6)] pub breath_secs: f64,
    #[k(1.5)] pub screen_m: f64,
    #[k(0.5)] pub pull_m: f64,
    #[k(40.0)] pub nap_wait: f64,
    #[k(1.2)] pub settle_secs: f64,
    #[k(0.5)] pub ears: f64,
    #[k(0.6)] pub noises: f64,
    #[k(alloc::vec![5.0, 11.0])] pub say_first: Vec<f64>,
    #[k(alloc::vec![16.0, 30.0])] pub say_every: Vec<f64>,
    #[k(60.0)] pub idle_secs: f64,
}
