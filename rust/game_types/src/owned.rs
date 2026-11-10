//! STATE A SYSTEM KEEPS FOR ITSELF. `Game` is declared below the rules, so any type it names is declared below them too, where every rule can
//! reach into it and a change to it rebuilds everything above. A system whose state only its own code reads keeps it in one of these slots:
//! `Game` holds the slot, the system's module declares the type, and nothing else can name it. The state clones with the game like any other.
use crate::bytes::{ToBytes, W};
use alloc::boxed::Box;
use core::any::{type_name, Any};
use core::fmt::Debug;

trait Kept: Any + Debug + ToBytes {
    fn copy(&self) -> Box<dyn Kept>;
    fn read(&self) -> &dyn Any;
    fn write(&mut self) -> &mut dyn Any;
}
impl<T: Any + Clone + Debug + ToBytes> Kept for T {
    fn copy(&self) -> Box<dyn Kept> { Box::new(self.clone()) }
    fn read(&self) -> &dyn Any { self }
    fn write(&mut self) -> &mut dyn Any { self }
}

/// one system's state: nothing until it first asks to write
#[derive(Debug, Default)]
pub struct Owned(Option<Box<dyn Kept>>);
impl Clone for Owned {
    fn clone(&self) -> Self { Owned(self.0.as_ref().map(|k| (**k).copy())) }
}
impl ToBytes for Owned { fn w(&self, o: &mut W) { self.0.w(o) } }
impl Owned {
    pub const fn new() -> Self { Owned(None) }
    /// the state, if it has been made
    pub fn get<T: Any>(&self) -> Option<&T> { self.0.as_ref().and_then(|k| (**k).read().downcast_ref()) }
    /// the state, made by `Default` the first time
    pub fn at<T: Any + Clone + Debug + Default + ToBytes>(&mut self) -> &mut T {
        let k = self.0.get_or_insert_with(|| Box::new(T::default()));
        (**k).write().downcast_mut().unwrap_or_else(|| panic!("a slot of the game holds another system's state, asked for {}", type_name::<T>()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone, Debug, Default, PartialEq, ToBytes)]
    struct Mine { n: u32 }
    #[test]
    fn a_slot_is_empty_until_written_and_clones_with_its_holder() {
        let mut a = Owned::new();
        assert!(a.get::<Mine>().is_none());
        a.at::<Mine>().n = 3;
        let mut b = a.clone();
        b.at::<Mine>().n = 4;
        assert_eq!((a.get::<Mine>().unwrap().n, b.get::<Mine>().unwrap().n), (3, 4));
    }
    #[test]
    #[should_panic(expected = "another system's state")]
    fn a_slot_refuses_another_type() {
        let mut a = Owned::new();
        a.at::<Mine>();
        a.at::<u32>();
    }
}
