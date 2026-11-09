pub use ::game_types::shutdoor::types;
pub use types::*;
/// this module's own order
pub const SHUT_ID: &str = crate::orders::rows::shutdoor::ORDER.id;
pub mod p_01;
pub use p_01::*;
