//! THE GAME'S DIE as a value a state can hold: the same stream as rust/sim's (it rolls rust/sim's own function), with the derives a cloned
//! state needs. `simRandom()` in the TypeScript is `g.dice.roll()` here; a die kept on a record (`Civ.dice`) is this type too.
use sim::dice::Dice;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Die {
    /// draws taken off it
    pub n: f64,
    /// the mulberry32 word
    pub s: u32,
}
impl Die {
    pub const fn new(seed: u32) -> Die {
        Die { n: 0.0, s: seed }
    }
    /// one draw, uniform in [0, 1)   (dice.ts simRandom / rollOn)
    pub fn roll(&mut self) -> f64 {
        let mut d = Dice { n: self.n, s: self.s, spare: 0 };
        let r = sim::dice::roll(&mut d);
        self.n = d.n;
        self.s = d.s;
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_stream_as_sim() {
        let mut a = Die::new(12345);
        let mut b = Dice { n: 0.0, s: 12345, spare: 0 };
        for _ in 0..100 {
            assert_eq!(a.roll().to_bits(), sim::dice::roll(&mut b).to_bits());
        }
        assert_eq!(a.n, 100.0);
        let mut c = a;
        assert_eq!(a.roll().to_bits(), c.roll().to_bits());
    }
}
