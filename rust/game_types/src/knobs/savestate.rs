//! The numbers of savestate.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// ================================================================================================================================ knobs
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CkptK {
    /// the undetected-play autosave: every this many seconds of a quiet live run
    #[k(90.0)] pub quiet_secs: f64,
    /// the compromise line: no autosave while any living man's awareness is over it (or anybody is alert / searching)
    #[k(0.4)] pub aware_max: f64,
    /// a request waits this long (more triggers inside it are the same save), never lands within minGap of the last save, and lapses unmet after pendingSecs
    #[k(2.0)] pub debounce_secs: f64,
    #[k(10.0)] pub min_gap_secs: f64,
    #[k(25.0)] pub pending_secs: f64,
    /// a dragged body let go of at least this far from where it was taken hold of counts as hidden
    #[k(1.5)] pub body_hide_m: f64,
    /// how long the HUD's CHECKPOINT tag shows
    #[k(2.6)] pub flash_secs: f64,
    /// an autosave followed inside this many seconds by a man going alert / an alarm episode is REGRETTED: kept, but the cards and the Enter path offer the one before it (a checkpoint written a stride before he was seen is a death loop, not a checkpoint)
    #[k(8.0)] pub regret_secs: f64,
}

/// THE SAVE TIMELINE (after a play test: one checkpoint to go back to is awkward, most of all in a lesson that has got away from its player). OFF here and on
///  the frozen stand (`TPS_SAVETIMELINE=1` raises it), ON in the page (`?savetimeline=0` puts it off). Off = three rotating autosaves, one slot by hand, nothing
///  kept with a team on the floor: the build as it was. On:
///    - the saves ROLL in two rings, `auto` autosaves and `manual` saves by hand: neither kind pushes
///      out the other, and the slots an older build wrote are read, listed and loaded as they are (same keys, same index; the one old manual slot is the ring's first);
///    - every save keeps a small PICTURE of its moment, taken by the host (`Checkpoints.picture`: the simulation never sees a canvas) and stored beside the slot
///      (`…pic.<slot>`); a picture that does not fit is dropped, and a save that does not fit has every picture thrown out for it before it is given up;
///    - WITH A TEAM ON THE FLOOR, where no file can be written (a unit has no slice), the save is the whole game kept in memory and
///      put back in place: this run's only, gone with the page or the run — the cards list both kinds on one strip (`timeline`), in the mission clock's order.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SavetimelineK {
    /// how many saves by hand and how many autosaves are kept — two rings
    #[k(5.0)] pub manual: f64,
    #[k(5.0)] pub auto: f64,
}
