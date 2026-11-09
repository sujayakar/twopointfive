pub use ::game_types::doortools::types;
pub use types::*;
/// the page its row opens, by the row's own id
pub const HELD_PAGE: &str = crate::orders::rows::held::ORDER.id;
/// this module's own order
pub const BIG_ORDER: &str = crate::orders::rows::bigclear::ORDER.id;
/// the page its row opens, by the row's own id
pub const TOOLS_PAGE: &str = crate::orders::rows::tools::ORDER.id;
pub mod p_02;
pub use p_02::*;
pub mod p_03;
pub use p_03::*;
pub mod p_04;
pub use p_04::*;
pub mod p_05;
pub use p_05::*;
pub mod p_06;
pub use p_06::*;
pub mod p_07;
pub use p_07::*;
pub mod p_08;
pub use p_08::*;
pub mod p_09;
pub use p_09::*;
pub mod p_10;
pub use p_10::*;
