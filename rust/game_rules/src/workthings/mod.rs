pub use ::game_types::workthings::types;
pub use types::*;
/// this module's own order
pub const WORK_ORDER: &str = crate::orders::rows::work::ORDER.id;
pub mod p_01;
pub use p_01::*;
