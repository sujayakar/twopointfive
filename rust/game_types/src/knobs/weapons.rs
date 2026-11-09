//! The numbers of weapons.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The ring-airfoil launcher's numbers: how many Sam brings (spent, never retrieved — no scavenging of
///  anything), and the seconds the launcher takes to hand-cycle between two rings. The rest of the ring — its speed, its arc, what a head or a hip does with it —
///  lives with its flight in launcher.ts RING.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct LauncherK {
    #[k(5.0)] pub rings: f64,
    #[k(0.9)] pub cd: f64,
}

/// The 40 mm high-explosive round out of the same tube: how many he brings (spent for good), and the
///  seconds it takes to break the tube, seat the next round and close it — the ring's hand-cycle clock (Rifle.ringCd: there is one tube) set to this after a round.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct Launcher40K {
    #[k(3.0)] pub rounds: f64,
    #[k(2.6)] pub cd: f64,
}
