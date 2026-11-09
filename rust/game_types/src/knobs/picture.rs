//! The numbers of picture.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PictureKShow {
    #[default] All,
    Off,
    Cursor,
}
impl PictureKShow {
    pub const fn as_str(self) -> &'static str {
        match self {
            PictureKShow::All => "all",
            PictureKShow::Off => "off",
            PictureKShow::Cursor => "cursor",
        }
    }
}

/// the switch and every number of it (UNTUNED by play)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PictureK {
    /// how often each man looks about him for it (s; the men take turns) and the sight rate that counts (the game's own glimpse line)
    #[k(0.2)] pub look_secs: f64,
    #[k(0.05)] pub glimpse: f64,
    /// a man tells his net of an enemy once he has known of him this long; a defender with no squad of his own tells the men inside `shoutM`
    #[k(1.0)] pub tell_secs: f64,
    #[k(16.0)] pub shout_m: f64,
    /// a report under the gunshot line is placed by ear out to this far along the way the sound takes (× how well that way carries: guards.ts's own rule);
    ///  gunfire is heard over the whole floor, as the game's hearing has it
    #[k(7.0)] pub pop_m: f64,
    /// a place he looks at tells him something (it is empty; a man lies there) only if it is lit this well on the light meter's scale, or is this near (m)
    #[k(0.2)] pub made_out: f64,
    #[k(3.0)] pub made_out_m: f64,
    /// an idea counts whole for `freshSecs`, falls to `staleK` of itself by `staleSecs` and is forgotten at `forgetSecs`; one whose place he has seen empty × `lostK`
    #[k(2.0)] pub fresh_secs: f64,
    #[k(20.0)] pub stale_secs: f64,
    #[k(0.4)] pub stale_k: f64,
    #[k(90.0)] pub forget_secs: f64,
    #[k(0.5)] pub lost_k: f64,
    /// an unseen man may have walked: the places he may be by now stand `walkMps` × the idea's age off it, up to `walkMax` (a heard one starts `heardM` wide), and count × `maybeK`
    #[k(1.2)] pub walk_mps: f64,
    #[k(6.0)] pub walk_max: f64,
    #[k(2.0)] pub heard_m: f64,
    #[k(0.6)] pub maybe_k: f64,
    /// a believed enemy's line reaches this far; his eye, a standing / kneeling man's chest, and his head above it (m): in a line = either shows
    #[k(30.0)] pub reach_m: f64,
    #[k(1.6)] pub eye_h: f64,
    #[k(1.15)] pub chest_h: f64,
    #[k(0.7)] pub low_h: f64,
    #[k(0.45)] pub head_up: f64,
    #[k(8.0)] pub flee_cost: f64,
    #[k(0.5)] pub stack_k: f64,
    #[k(0.5)] pub step_at: f64,
    #[k(0.25)] pub step_free: f64,
    #[k(2.5)] pub step_m: f64,
    #[k(0.6)] pub step_after: f64,
    #[k(1.5)] pub step_rest: f64,
    #[k(5.0)] pub step_secs: f64,
    /// the floor view of one man's picture (ui/pictureviz.ts): 'off' · 'cursor' = the fighter nearest the aim point · 'all' = every teammate's ideas
    #[k(PictureKShow::Off)] pub show: PictureKShow,
    /// the view's reach round the man (m)
    #[k(9.0)] pub show_m: f64,
}
