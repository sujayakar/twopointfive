pub use ::game_types::squadthrow::types;
pub use types::*;
/// this module's own order
pub const STUN_THERE: &str = crate::orders::rows::stunthere::ORDER.id;
/// this module's own order
pub const FRAG_THERE: &str = crate::orders::rows::fragthere::ORDER.id;
/// this module's own order
pub const THROW_OPEN: &str = crate::orders::rows::throwopen::ORDER.id;
pub mod spots;
pub use spots::*;
pub mod walk;
pub use walk::*;
pub mod plan;
pub use plan::*;
