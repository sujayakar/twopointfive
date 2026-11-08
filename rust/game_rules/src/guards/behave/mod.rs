//! WHAT A MAN DOES WHEN NOTHING HAS TAKEN HIM: his state's behaviour, one file a state. `guards::update_guard` senses for him first (`Sensed`, which no
//! behaviour writes), asks his state's behaviour what he does this tick (`Acts`, which no behaviour reads), and poses him by the answer.
//!
//! TO CHANGE HOW HE FIGHTS, SEARCHES, LOOKS OR PATROLS: his state's file. What takes him from his behaviour (a grenade, a blast, rounds past his head)
//! is a rule of control/, asked before he senses or at the top of `alert`.
use crate::character::UpperMode;
use crate::game::{Game, GuardState};
use crate::ids::*;
use crate::V3;

pub mod alert;
pub mod patrol;
pub mod search;
pub mod suspicious;

/// what he senses this tick and how he is: settled before his behaviour is asked
#[derive(Clone, Copy)]
pub struct Sensed {
    pub c: CharId,
    pub route: RouteId,
    /// he is of the building's own staff (a guard), not a unit of a side
    pub staff: bool,
    /// the building's escalation as his frame began
    pub lvl: f64,
    /// how far his man is
    pub dist: f64,
    /// the hostage and holder he faces, in a standoff
    pub pair: Option<crate::guards::Pair>,
    /// he cannot see · he is reeling
    pub dazzled: bool,
    pub staggered: bool,
    /// where he stood as his behaviour began
    pub p0x: f64,
    pub p0z: f64,
}

/// what his behaviour decided, for his pose (his legs it has moved already)
#[derive(Clone, Copy)]
pub struct Acts {
    /// how fast his legs go
    pub move_speed: f64,
    /// what his hands have up
    pub upper: UpperMode,
    /// where he aims or looks; None: ahead
    pub aim_at: Option<V3>,
    /// how low, 0 standing to 1 down
    pub crouch: f64,
    /// his gun is lowered
    pub duck: bool,
    /// he is slicing a corner
    pub pied: bool,
}
impl Acts {
    /// he stands where he is, nothing up, looking ahead
    pub const fn still() -> Self { Acts { move_speed: 0.0, upper: UpperMode::None, aim_at: None, crouch: 0.0, duck: false, pied: false } }
}

/// what he does this tick, by his state
pub fn act(g: &mut Game, gd: GuardId, dt: f64, s: &Sensed) -> Acts {
    match g.gd[gd].state {
        GuardState::Patrol => patrol::patrol(g, gd, dt, s),
        GuardState::Suspicious => suspicious::suspicious(g, gd, dt, s),
        GuardState::Alert => alert::alert(g, gd, dt, s),
        GuardState::Search => search::search(g, gd, dt, s),
        GuardState::Dead => Acts::still(),
    }
}
