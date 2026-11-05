//! LYING SNOW UNDERFOOT, AS WHOEVER IS NOT A WALK OF THE THROWN THINGS ASKS IT — src/game/snowBog.ts snowBogAt. THE ONE RULE is items.rs `bog_of`, read off the ONE table of
//! the snow (world.rs `snow`, a byte a static, each box's place and turn being its own row's): this hands it the roofs in force from the weather's table and the
//! rule's numbers as they stand, where a walk is handed its own. A bounce's own numbers (`dead`, `grip`, `stick`) are no part of how boggy the ground is.
use crate::items::{bog_of, Snow};
use crate::knobs::{k, SNOWBOG_BANK, SNOWBOG_OLD, SNOWBOG_PACKED, SNOWBOG_TOP};
use crate::weather::of;

/// snowBog.ts snowBogAt: how boggy the ground of game `g`, world `w`, is where a thing touched down at (x, y, z), 0‥1
pub fn bog_at(g: usize, w: usize, x: f64, y: f64, z: f64) -> f64 {
    let wx = of(g);
    let s = Snow { top: k(SNOWBOG_TOP), bank: k(SNOWBOG_BANK), packed: k(SNOWBOG_PACKED), old: k(SNOWBOG_OLD), dead: 0.0, grip: 0.0, stick: 0.0, roofs: wx.roofs_n as usize, roof: wx.roofs };
    bog_of(w, &s, x, y, z)
}
