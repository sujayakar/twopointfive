pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{tut_squad_define, tut_squad_picket_step, tut_squad_picket_test, tut_squad_store_step, tut_squad_store_test, tut_squad_shop_step, tut_squad_shop_test, tut_squad_hall_step, tut_squad_hall_test, tut_squad_gun_step, tut_squad_gun_test};
