//! The numbers of cues.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct CuesK {
    /// the strip: rows drawn at most · s a row stays · s a row is owed before one of its own class may take its place · rows one class may hold at once
    #[k(3.0)] pub rows: f64,
    #[k(8.0)] pub secs: f64,
    #[k(2.5)] pub min_show: f64,
    #[k(2.0)] pub per_class: f64,
    /// s inside which the same words are not drawn twice
    #[k(10.0)] pub dup_secs: f64,
    /// s between two rows of one class (index = class; 0 = a row that was always on the strip: control, the staff's net)
    #[k(alloc::vec![0.0, 1.2, 3.0, 3.0, 2.5, 3.0, 12.0])] pub gaps: Vec<f64>,
    /// every row of their net that reaches the strip squelches as it arrives (gain; 0 = silent)
    #[k(0.35)] pub squelch: f64,
    /// s between two cues from one man · s between two of his 'stay' / 'still' lines
    #[k(2.5)] pub man_secs: f64,
    #[k(28.0)] pub again_secs: f64,
    /// s alert with nobody in his sight, standing, before he says he is still holding · … and with no shot in his ears
    #[k(8.0)] pub still_secs: f64,
    #[k(5.0)] pub still_quiet: f64,
    #[k(4.0)] pub overheard_gap: f64,
}
