//! THE ROOM FIGHT: defenders who hold rooms (defender.rs) against a squad (operative.rs), and what both ask: the rooms (rooms.rs), who is in the line of a
//! round (line.rs), the fight as a whole (fight.rs).
pub use ::game_types::roomfight::types;
pub use types::*;
pub mod b_01;
pub use b_01::*;
pub mod defender;
pub use defender::*;
pub mod fight;
pub use fight::*;
pub mod line;
pub use line::*;
pub mod operative;
pub use operative::*;
pub mod rooms;
pub use rooms::*;
