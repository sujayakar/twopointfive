//! The numbers of packside.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// `on`: the switch · `ram`: half 3 of the rule · `gap`: m of air between the pack's face and the thing beside it · `handle`: bind z of the hammer's
///  handle's line (the operators' own back stands to −0.20 beside the spine, at the shoulder blades and the belt's pouches; the head, 4.5 cm through, lies
///  over the shoulder's top where the back is at −0.185) · `gun`: bind z of a long gun's top face · `over`: m the hammer's head clears the pack's top corner
///  by · `headMin`: the lowest its middle rides, bind y (the top of the shoulder blades: a head lower than that lies in them) · `butt`: bind y of the top of a
///  long gun stood at the side (its butt end, behind the shoulder) · `rest`: m of air kept between the thing and the back it lies along · `pitch0`‥`pitch1`:
///  rad, how far it may turn about its top end off the pack's upright (+ = its lower end toward him)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PacksideK {
    #[k(0.012)] pub gap: f64,
    #[k(-0.258)] pub handle: f64,
    #[k(-0.24)] pub gun: f64,
    #[k(0.012)] pub over: f64,
    #[k(1.5)] pub head_min: f64,
    #[k(1.56)] pub butt: f64,
    #[k(0.012)] pub rest: f64,
    #[k(-0.1)] pub pitch0: f64,
    #[k(1.3)] pub pitch1: f64,
}
