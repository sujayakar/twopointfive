//! WHAT IS MADE AGAIN FROM THE REST OF A GAME AND THE LEVEL, and kept so that it is not made every time: in no bytes and in no copy. A game put back makes its own again, but for what
//! the game laid over another takes over of it (savestate/control.rs `stand`).
use crate::bytes::{ToBytes, W};
use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefCell;

/// one system's table, of a type of its own (`Game` is declared below them all): nothing until it is first asked for
#[derive(Default)]
pub struct Made(RefCell<Option<Box<dyn Any>>>);
impl Made {
    /// the table, made by `Default` the first time. A question is asked of a game that is only read, so it is lent for the length of `f`: no table is asked for inside its own.
    pub fn with<T: Any + Default, R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut t = self.0.borrow_mut();
        f(t.get_or_insert_with(|| Box::new(T::default())).downcast_mut().expect("a table of one system asked for as another's"))
    }
}
impl Clone for Made {
    fn clone(&self) -> Self { Made::default() }
}
impl core::fmt::Debug for Made {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result { f.write_str("Made") }
}
impl ToBytes for Made {
    fn w(&self, _: &mut W) {}
}

/// a number that is no other's in this process: which writing of a thing this is, for what is made from it (no rule of the game reads one)
pub fn next() -> u64 {
    static N: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(1);
    N.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
}

/// the tables of rust/sim
#[derive(Clone, Debug, Default, ToBytes)]
pub struct Tables {
    /// which ways smoke goes from cell to cell (bricks.rs)
    pub ways: Made,
    /// what the light under a roof is worked out from (glow.rs), and what of the sky reaches a place under it (dome.rs)
    pub light: Made,
    pub sky: Made,
}
