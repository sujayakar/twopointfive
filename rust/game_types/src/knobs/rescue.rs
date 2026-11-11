//! The numbers of rescue.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RescueKStart {
    #[default] Quiet,
    Hot,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RescueKDoorMan {
    #[default] Inside,
    Corridor,
}

/// The scenario's knobs. Who stands where, the stun, the lights and the clock's dealt length are read at a staging
///  ('restage' applies them); the clocks' other numbers, blind fire and the clock's own switch are read as it runs.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RescueK {
    /// 'hot' = the captor's clock starts on Sam's first move, shot or order; 'quiet' = on the first thing a defender hears or sees of the team
    #[k(RescueKStart::Hot)] pub start: RescueKStart,
    /// seconds from the clock's start to the first hostage shot, ± the spread drawn at each staging; the second hostage this long after the first; his aim on her
    ///  (at 40 a room-by-room crawl with a bang on every door still brought a hostage out in half its runs; at 35
    ///  it fails in most, which is what the design asks of it, and the fast entry keeps a dozen seconds in hand)
    #[k(35.0)] pub resolve_secs: f64,
    #[k(8.0)] pub second_hostage_secs: f64,
    #[k(1.2)] pub aim_secs: f64,
    /// clock two: seconds from a defender first learning of the assault to his being set, and the reaction time a man caught before that is held to on his first alert
    #[k(9.0)] pub set_secs: f64,
    #[k(1.1)] pub unset_react: f64,
    /// the door man: in the corridor's east half, on the straight run — or just inside the office, a second gun on the door
    #[k(RescueKDoorMan::Corridor)] pub door_man: RescueKDoorMan,
    /// men elsewhere on the floor who come to the noise (0‥2: the break room, the conference room — the second costs a hostage his shadow in the page's renderer);
    ///  `seedRoamers`: which of the two rooms the first is in is drawn at each staging (the conference room is the first door the team passes, the break room the
    ///  last — so no room is known empty); off = the break room first
    #[k(1.0)] pub roamers: f64,
    #[k(1.5)] pub crew_call_secs: f64,
    /// how many teammates are staged (the loadout's size; the mission's staging writes it): 2 = the pair · 0 / 1 = nobody /
    ///  Blue One alone · 3 / 4 = the four-man squad's first three / all of it (`squad4()`: a red pair and a blue pair, in pairs for the order menu), the
    ///  men past the second stood a stride behind the pair's marks. F takes whoever is up.
    #[k(2.0)] pub squad: f64,
    /// where the team is staged: the LEVEL's (tools/levels/lib/scenarios.ts RescueData.team — staging writes the team start of the level it stages on here; these are the
    ///  office's until it has: the lobby arch at the corridor's west end)
    #[k([7.6, 0.0, 13.4])] pub sam_at: V3,
    #[k(0.9)] pub sam_yaw: f64,
    #[k([[6.6, 0.0, 14.25], [8.5, 0.0, 14.3]])] pub ops_at: [V3; 2],
}
