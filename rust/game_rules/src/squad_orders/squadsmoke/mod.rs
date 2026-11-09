pub use ::game_types::squadsmoke::types;
pub use types::*;
/// this module's own order
pub const SMOKE_ORDER: &str = crate::orders::rows::smokethere::ORDER.id;
pub mod p_01;
pub use p_01::*;
