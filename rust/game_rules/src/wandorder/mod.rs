pub use ::game_types::wandorder::types;
pub use types::*;
/// this module's own order
pub const WAND_ORDER: &str = crate::orders::rows::wand::ORDER.id;
pub mod p_01;
pub use p_01::*;
