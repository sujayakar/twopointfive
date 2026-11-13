//! THE PANES OF GLASS THAT HAVE BEEN TOUCHED, as the box world holds them (`World.panes`): a line that meets a pane reads its row where it stands. The rules are sim's
//! panes.rs and glass.rs, and the picture reads the rows.
use crate::bytes::ToBytes;
use crate::V3;
use alloc::vec::Vec;

/// doubles a pane's row lays out (sites and the dice), and words (its static, the flags, the open pieces)
pub const GD: usize = 80;
pub const GW: usize = 64;
/// the sites a pane takes
pub const MAX_SITES: usize = 8;
/// the charges a pane remembers the blows of
pub const BK: usize = 8;

/// A PANE THAT HAS BEEN TOUCHED
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Row {
    /// the sites (sim glass.rs: nine doubles each), the dice's blow, half sides and last clump out, when the dice began
    pub d: [f64; GD],
    /// the static, flags and site count, the dice's lattice and seed, each site's radials and seed, a bit a piece out, a bit a piece queued, a bit a crater queued
    pub w: [u32; GW],
    /// energy taken at each site, and blows
    pub e: [f64; MAX_SITES],
    pub hits: [f64; MAX_SITES],
    /// where it stands (read from its box as it is first touched: statics do not move), its half sides in its own frame
    pub c: V3,
    pub cs: f64,
    pub sn: f64,
    pub hu: f64,
    pub hv: f64,
    pub ht: f64,
    pub energy: f64,
    pub damage: f64,
    pub next_think: f64,
    pub last_dir: V3,
    pub last_uv: [f64; 2],
    pub sound_at: f64,
    pub who: f64,
    /// what the charges in hand have already done to it: which charge (its number) and how hard, a few at a time
    pub bk: [f64; 2 * BK],
    /// the pane's own word (`box_seed`): every hash of its break starts here, none from its place in the level's list
    pub key: u32,
    pub ty: u32,
    pub u_axis: u32,
    pub exterior: u32,
    pub hit_n: u32,
    pub n_open: u32,
    pub d_next: u32,
    pub walkable: u32,
    pub bk_at: u32,
}
impl Row {
    /// static `bi`'s, with nothing done to it
    pub fn of(bi: u32) -> Row {
        let mut w = [0; GW];
        w[0] = bi;
        Row {
            d: [0.0; GD], w, e: [0.0; MAX_SITES], hits: [0.0; MAX_SITES], c: [0.0; 3], cs: 0.0, sn: 0.0, hu: 0.0, hv: 0.0, ht: 0.0, energy: 0.0, damage: 0.0, next_think: 0.0,
            last_dir: [0.0; 3], last_uv: [0.0; 2], sound_at: 0.0, who: 0.0, bk: [0.0; 2 * BK],
            key: 0, ty: 0, u_axis: 0, exterior: 0, hit_n: 0, n_open: 0, d_next: 0, walkable: 0, bk_at: 0,
        }
    }
}
/// A PIECE QUEUED TO LEAVE: when, where the blow came from (a blast's, a walker's), how hard, which piece, why
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Going {
    pub at: f64,
    pub from: V3,
    pub k: f64,
    pub fid: u32,
    pub cause: u32,
}
/// THE THROUGH-HOLES OF ONE PANE, the oldest first
#[derive(Clone, Debug, ToBytes)]
pub struct Hole {
    pub bi: u32,
    pub at: Vec<V3>,
}
/// A CHARGE LATELY SEEN: when, where, its number
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Charge {
    pub t: f64,
    pub at: V3,
    pub seq: f64,
}

#[derive(Clone, Debug, Default, ToBytes)]
pub struct Panes {
    /// in the order they were first touched: the rule's own, which serves them a frame's budget in that order
    pub rows: Vec<Row>,
    /// what each row has queued to leave, the soonest first (then by which piece)
    pub going: Vec<Vec<Going>>,
    pub holes: Vec<Hole>,
    /// the window's bars a round or a blast has taken, ascending: a static and its flags as it stood
    pub bars: Vec<[u32; 2]>,
    /// a word of the level's on a pane's glass: which pane and which glass
    pub kinds: Vec<[u32; 2]>,
    /// the floor's last 'glass' event a pane a shooter: (pane × 1024 + who, when)
    pub told: Vec<[f64; 2]>,
    /// the charges in hand, and the number of the last one seen
    pub charges: Vec<Charge>,
    pub seq: u32,
    /// pieces let go this frame (a frame's budget, across the panes)
    pub released: u32,
    /// a pane on an outside wall has changed (the wind's gusts ask again)
    pub gust: bool,
}
impl Panes {
    /// which row is static `bi`'s (None: it is whole, and has never been touched)
    #[inline]
    pub fn row_of(&self, bi: u32) -> Option<usize> { self.rows.iter().position(|r| r.w[0] == bi) }
}
