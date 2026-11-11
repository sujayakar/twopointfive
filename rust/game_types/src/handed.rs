//! WHAT A HOST HANDS THE GAME AS NUMBERS, a row each: what a seat's hands did in a tick and the view they did it in, the cursor a question is asked of, a man drawn ahead of the tick, the page's look. Every one is all
//! doubles, and the page writes them by name (src/draw/input.ts, which the program `half` writes from here).
use crate::rows::{table, Row, RowTable};
use alloc::{string::String, vec, vec::Vec};

row! {
    /// WHAT A SEAT'S HANDS DID IN A TICK, as a player's own machine hands it and as it travels to another's: his keys go beside it, as their codes
    pub struct SeatInput {
        /// the mouse's buttons held, a bit each
        pub buttons: f64,
        /// those that went down in the tick
        pub clicked: f64,
        /// what the wheel turned
        pub wheel: f64,
        /// the cursor, in the page's pixels
        pub mx: f64,
        pub my: f64,
        /// the view it is in: its width and height in the page's pixels
        pub w: f64,
        pub h: f64,
        /// … and in the picture's own
        pub pw: f64,
        pub ph: f64,
        /// what his camera looks at
        pub target: [f64; 3],
        /// from where about it: the turn, the tilt, how far
        pub yaw: f64,
        pub pitch: f64,
        pub distance: f64,
        /// its lens
        pub fov: f64,
        pub aspect: f64,
        /// how far it leads his aim
        pub aim_lead: f64,
    }
}
row! {
    /// THE VIEW A PLAYER'S OWN MACHINE HAS beside that: what the game works out again for a seat that is another machine's
    pub struct SeatLens {
        pub vp: [f64; 16],
        pub ivp: [f64; 16],
        pub pos: [f64; 3],
        pub forward: [f64; 3],
        /// the distance the camera is on its way to
        pub desired: f64,
    }
}
row! {
    /// THE CURSOR A QUESTION IS ASKED OF, as the frame is drawn: where it is and in what picture (the page's pixels), through which camera
    pub struct CursorNow {
        pub mouse: [f64; 2],
        pub client: [f64; 2],
        pub view_proj: [f64; 16],
        pub inv_view_proj: [f64; 16],
    }
}
row! {
    /// A NETWORKED HOST'S OWN MAN, AHEAD OF THE TICK: where the tick has him (his feet, the way he aims), and where he is drawn
    pub struct LeadNow {
        pub man: f64,
        pub at: [f64; 3],
        pub yaw: f64,
        pub to: [f64; 3],
        pub to_yaw: f64,
    }
}
/// the stuffs a box may say
pub const STUFFS: usize = 32;
row! {
    /// WHAT THE PAGE SAYS OF ITS LOOK: a yes is 1
    pub struct PageLook {
        pub on: f64,
        pub marks: f64,
        pub cut: f64,
        pub round: f64,
        pub motion: f64,
        pub panes: f64,
        /// lines of light: mirrors show them, and the air shows a weapon's
        pub mirrors: f64,
        pub air: f64,
        /// solid glass: the darkest it is sent and its roughness (under 0: as any box)
        pub glass_lum: f64,
        pub glass_rough: f64,
        pub texel: f64,
        /// by stuff: its number in the look's maps, and the roughness its coat lends (under 0: none)
        pub stuff: [f64; 32],
        pub coat: [f64; 32],
        /// by side of men: the colours a cast man's own are put for
        pub swaps: [f64; 4],
        /// the men's cloth and skin show wet
        pub wet: f64,
    }
}
const _: () = assert!(core::mem::size_of::<PageLook>() == 8 * (11 + 2 * STUFFS + 5));

macro_rules! handed { ($($t:ident)*) => {
    /// the rows that are handed, for the program that writes the page's half
    pub fn tables() -> Vec<RowTable> { vec![$(table::<$t>()),*] }
    /// as text, a line a row
    pub fn text() -> String { let mut s = String::new(); $(line::<$t>(&mut s);)* s }
} }
handed!(SeatInput SeatLens CursorNow LeadNow PageLook);
fn line<T: Row>(s: &mut String) {
    use core::fmt::Write;
    s.push_str(T::NAME);
    for (f, t) in T::FIELDS { let _ = write!(s, " {f}:{t}"); }
    s.push('\n');
}
/// what a tape keeps of a tick, the input and then the lens, as text
pub fn hand_text() -> String { let mut s = String::new(); line::<SeatInput>(&mut s); line::<SeatLens>(&mut s); s }
