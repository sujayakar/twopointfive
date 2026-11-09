pub use ::game_types::wallcharge::types;
pub use types::*;
/// this module's own order
pub const WALL_ORDER: &str = crate::orders::rows::wallrow::ORDER.id;
/// the page its row opens, by the row's own id
pub const WALL_PAGE: &str = crate::orders::rows::walltools::ORDER.id;
pub mod walls;
pub use walls::*;
pub mod posts;
pub use posts::*;
pub mod blow;
pub use blow::*;
pub mod sam;
pub use sam::*;
