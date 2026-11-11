//! The numbers of enemysquads.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SquadsK {
    /// s between a squad learning where the team is and its word on the net (drawn once per decision)
    #[k([2.0, 4.5])] pub think: [f64; 2],
    /// s between a squad's contact call and the other squads acting on it
    #[k(0.8)] pub net_lag: f64,
    /// men a squad sends to help, at most · the men of a flank
    #[k(3.0)] pub help_men: f64,
    #[k(2.0)] pub flank_men: f64,
    /// s before a squad that has made its choice weighs the same fight again · s after a choice before ANY news is weighed
    #[k(20.0)] pub again_secs: f64,
    #[k(8.0)] pub settle_secs: f64,
    /// s with nothing of the team seen or heard in its rooms before a squad in contact is holding again · s between two calls of one contact
    #[k(8.0)] pub quiet_secs: f64,
    #[k(20.0)] pub recall_secs: f64,
    /// a hit squad's men in its other rooms: their beat before they come
    #[k([1.0, 2.5])] pub reinforce: [f64; 2],
    #[k(0.4)] pub ready_beat: f64,
    #[k(10.0)] pub fire_m: f64,
    #[k(6.0)] pub rear_m: f64,
    #[k(3.0)] pub soft_m: f64,
    #[k(18.0)] pub mouth_m: f64,
    #[k(16.0)] pub hold_back_m: f64,
    #[k(40.0)] pub hold_secs: f64,
    /// HARD ON A SQUADS LEVEL: A DRILLED GARRISON (`squad_drill`). Read only with harder defenders on (`pressure.on`, the difficulty row's hard) and squads staged,
    ///  with a map's own rows (`drill_at`) over the table: standard, and hard on a level with no squads, never reach a row of it. The same eleven, the same three choices, the
    ///  same words on the net; what changes is how well drilled they are, each rule its own switch:
    ///  `drill_net`    THEY LET THE TEAM COMMIT: × `net_k` the thinking beat (`think`) and the net's lag (`net_lag`) of a squad that could send men out
    ///                (allowed to leave, nobody of its own still out, two to spare), and × `reinforce_k` the beat of a hit squad's men in its
    ///                other rooms before they come (`reinforce`). A drilled garrison does not run at the first noise: a squad that hears of a fight and could go says so
    ///                ('{sq}: hold — let them come in. wait for my call!') and takes twice the beat before it chooses and goes, stood to or not,
    ///                so the pair that goes round is set at the other door while the team is going in, instead of being met on its way. A squad that
    ///                stays in its rooms whatever is called answers as at standard. Under 1 the net is quicker, and that measured in the TEAM's favor: sent sooner,
    ///                the pair walks onto guns that are still up.
    ///  `drill_angles` THEY HOLD THEIR ANGLES: a man's eyes do not wander off the door he watches (the fight row's `hold` / `glance`, the untrained man's habit of a
    ///                look at another door now and then).
    ///  `follow_m`     how far the team's next loud thing (a boot, a charge, gunfire; not in that room) must be from the door the pair holds for them to hear it as gone on.
    ///  `drill_own`    NO MAN FED INTO A LOST ROOM: a hit squad's man in another of its rooms does not walk into the room under attack once fewer than `own_up` (1: nobody) of
    ///                his squad are up in it — he gets ready on his own door ('<Squad>: <room>'s gone — stay on your doors, make them come!').
    #[k(true)] pub drill_net: bool,
    #[k(2.0)] pub net_k: f64,
    #[k(2.0)] pub reinforce_k: f64,
    #[k(false)] pub drill_angles: bool,
    #[k(6.0)] pub follow_m: f64,
    #[k(false)] pub drill_own: bool,
    #[k(1.0)] pub own_up: f64,
    /// BACK ON THEIR OWN DOORS (off; a map's own row — DRILL_AT): two men holding the other door of a room the team took hear the team go on somewhere else (the
    ///  loud thing: `follow_m` or more from their door, not in that room) and give the door up — back to the posts they left, in their own
    ///  rooms, their eyes toward the noise; or the squad they went round for stops answering (every man of it down, `back_beat` s of silence), and they do the same. Standard's pair stays on a door nobody is behind, out in the open, until the team comes on it from the side. Said
    ///  once on their net: '<Squad>: they've left it — back on our doors!'.
    #[k(false)] pub drill_back: bool,
    #[k(3.0)] pub back_beat: f64,
    /// GO AND LOOK (`drill_look`; read only with `pressure.sweep` up as well, hard's one switch of the rule on every kind of level): the
    ///  garrison has had word of the team — a call on the net, one of them properly in a man's sight, a loud thing in a man's ears — and then nothing for
    ///  `sweep_secs`. Standard's squads sit on their doors from there for as long as the team cares to take. A drilled garrison sends a pair to look: of the
    ///  squads that have nobody out and can spare two (a man left in every room that had one; `sweep_men` is never fewer than two), the one nearest the place
    ///  that word put the team calls it on the net — '<Squad>: gone quiet — two of us going to look!', the cue, `sweep_beat` before either moves — and lets the
    ///  two go at it on the fight AI's own hunt (the stock rally's hand-over: their boots are theirs to be heard). Calm again out there with nothing found,
    ///  they walk back to the posts they left ('<Squad>: no sign of them — back on our doors!'). One look per word, never oftener than every `sweep_secs`;
    ///  nobody goes to a room a man of theirs holds, while any squad is in contact, deciding or has men out, on his way to cover or wedging a door, or with a
    ///  covered lane known that way; and A LOOK THAT LOSES A MAN IS THE LAST — no squad sends another that fight.
    ///  `sweep_stayers` (on): a squad the level tells to stay in its rooms (`mayLeave: false` — it answers no other squad's fight) still sends its look: the pair
    ///  is the squad finding out where the team went, not a walk to somebody else's fight. Off = such a squad never leaves, hard or not (Mousehole, the lessons).
    #[k(true)] pub drill_look: bool,
    #[k(20.0)] pub sweep_secs: f64,
    #[k(2.0)] pub sweep_men: f64,
    #[k(1.5)] pub sweep_beat: f64,
    #[k(true)] pub sweep_stayers: bool,
    #[k(40.0)] pub check_every: f64,
    #[k(0.2)] pub check_jitter: f64,
    #[k(0.5)] pub check_alert_k: f64,
    #[k(4.0)] pub check_ask: f64,
    #[k(8.0)] pub check_send: f64,
    #[k([1.0, 2.0])] pub check_heard: [f64; 2],
    #[k(4.0)] pub check_hear_m: f64,
    #[k(2.0)] pub look_men: f64,
    #[k([1.5, 2.5])] pub call_secs: [f64; 2],
    #[k([1.0, 2.0])] pub call_cut_ask: [f64; 2],
    #[k(60.0)] pub net_every: f64,
    #[k([6.0, 9.0])] pub net_beat: [f64; 2],
    #[k(2.0)] pub net_look_men: f64,
    #[k(3.0)] pub net_stops: f64,
}

/// WHAT HARD IS ON ONE SQUAD MAP, by its structure's name (`scenarios.roomClear.name`): rows of the drill read in place of the table's own
///  (`drill_over`). A map not named here gets the table's rows.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct DrillAtK {
    #[k({ let mut t = JsMap::new(); t.set(String::from("the block, three squads"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(true), net_k: Some(2.0), reinforce_k: Some(2.0), drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the warehouse"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the block"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(false), back_beat: None }); t.set(String::from("the compound"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(true), net_k: Some(1.0), reinforce_k: Some(0.75), drill_own: Some(true), own_up: Some(1.0), drill_back: Some(false), back_beat: None }); t.set(String::from("the barracks"), crate::enemysquads::DrillAtRow::Row { drill_net: Some(false), net_k: None, reinforce_k: None, drill_own: Some(false), own_up: None, drill_back: Some(true), back_beat: Some(3.0) }); t })] pub rows: JsMap<String, crate::enemysquads::DrillAtRow>,
}
