pub use ::game_types::wallcharge::types;
pub use types::*;
/// this module's own order
pub const WALL_ORDER: &str = crate::orders::rows::wallrow::ORDER.id;
/// the page its row opens, by the row's own id
pub const WALL_PAGE: &str = crate::orders::rows::walltools::ORDER.id;
pub mod p_01;
pub use p_01::*;
pub mod p_02;
pub use p_02::*;
pub mod p_03;
pub use p_03::*;
pub mod p_04;
pub use p_04::*;
