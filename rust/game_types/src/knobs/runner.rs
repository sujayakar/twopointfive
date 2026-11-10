//! The numbers of runner.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// Metres: `closeM` the enemy in his eyes, `tellM` the mate he runs in on, `toldM` the mates who hear it, `shyM` how wide of an enemy his way is planned,
/// `mateM` the longest way to a mate worth running, `clearM` how far clear of the building's walls counts as out, `edgeM` the rim of a map with no building,
/// `cornerM` how far the corner may be. Seconds: `beatSecs` shout to drop, `tellBeat` the told men's own beat, `goneSecs` out and unseen, `mateSecs` the most
/// he spends getting to a mate, `stallSecs` going nowhere before he thinks again, `retrySecs` between a cornered man's looks for a way out that has come free
/// (`retries` = the runs at a way that would not let him through before he stays where he is for good), `replanSecs` between his looks at the `aheadM` of his
/// way in front of him for an enemy who has stepped into it. `mateM` is by the FLOOR (the path's own length). `askCells` / `askWide` = how many outside cells
/// (a metre's grid) are asked for one no enemy has a line to — on the first look, and on the wider looks of a man who is out with eyes still on him.
/// `tellAware` = the awareness a told man of no post is left at. UNTUNED by eye (nobody has seen it on a GPU, nobody has played it).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RunnerK {
    #[k(5.0)] pub close_m: f64,
    #[k(0.8)] pub out_secs: f64,
    #[k(0.6)] pub beat_secs: f64,
    #[k(4.0)] pub tell_m: f64,
    #[k(8.0)] pub told_m: f64,
    #[k(0.8)] pub tell_beat: f64,
    #[k(0.6)] pub tell_aware: f64,
    #[k(2.0)] pub shy_m: f64,
    #[k(20.0)] pub mate_m: f64,
    #[k(10.0)] pub mate_secs: f64,
    #[k(1.5)] pub clear_m: f64,
    #[k(2.0)] pub edge_m: f64,
    #[k(25.0)] pub corner_m: f64,
    #[k(2.0)] pub gone_secs: f64,
    #[k(3.0)] pub stall_secs: f64,
    #[k(2.0)] pub retry_secs: f64,
    #[k(0.5)] pub replan_secs: f64,
    #[k(3.0)] pub ahead_m: f64,
    #[k(60.0)] pub ask_cells: f64,
    #[k(240.0)] pub ask_wide: f64,
}
