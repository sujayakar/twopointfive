//! The numbers of wallcharge.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use game_derive::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WallchargeKMenu {
    #[default] Page,
    Direct,
}
impl WallchargeKMenu {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallchargeKMenu::Page => "page",
            WallchargeKMenu::Direct => "direct",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct WallchargeK {
    /// how the menu reaches the row: 'page' = X, 5 'wall tools ▸', 3 — the door charge's own three presses · 'direct' = X, 5
    #[k(WallchargeKMenu::Page)] pub menu: WallchargeKMenu,
    /// m: how near a wall's footprint the cursor's point must be for the target to be that wall
    #[k(0.06)] pub pick_m: f64,
    /// m: the charge's height on the wall · how near a wall's end it may sit · how far along the wall a spot is slid to find floor on both faces
    #[k(1.05)] pub height: f64,
    #[k(0.75)] pub end_m: f64,
    #[k(alloc::vec![0.0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5])] pub slide: Vec<f64>,
    /// m off the face: where the man who sets it stands · how near that he must be to be setting it
    #[k(0.5)] pub set_off: f64,
    #[k(0.9)] pub set_near: f64,
    /// Sam's own row: how near the foot of the wall he must be · how far he may shift while setting it
    #[k(1.35)] pub sam_reach: f64,
    #[k(0.35)] pub sam_shift: f64,
    #[k(3.0)] pub sam_marker_m: f64,
    /// the stand-off: metres from the charge along the wall (tried in turn), off its face, between men; else back across the floor at these distances
    #[k(alloc::vec![2.6, 2.0])] pub standoff: Vec<f64>,
    #[k(0.55)] pub post_off: f64,
    #[k(0.9)] pub file_gap: f64,
    #[k(alloc::vec![3.0, 3.9, 4.8])] pub back_m: Vec<f64>,
    /// m: a post ALONG the wall is legal only this far past where the blast can open it — WALLS.blast.radius × portK plus half a cell (2.01 m of the charge at
    ///  the shipped numbers), so the 2.0 m fallback of `standoff` starts at 2.46 (on the 0.45 m step that is the 2.6 pass's own first post, 2.70: a stretch that only fitted a man at 2.25 goes to the fan
    ///  back across the floor, or is refused) and the 2.6 is as it was. 0 = the posts as they were
    #[k(0.45)] pub clear_m: f64,
    /// m: a teammate outside the order nearer the charge than this on its own side is in the blast (refused)
    #[k(2.0)] pub mate_m: f64,
    /// s after the blast before the first man goes · between men: DOORTOOLS.second · the whole entry's cap · on their points before the order ends
    #[k(0.7)] pub after_blow: f64,
    #[k(16.0)] pub enter_cap: f64,
    #[k(1.5)] pub sweep_secs: f64,
    /// m past the far face: the point in the hole's far mouth each man is sent through · their lanes' half-spread across the hole
    #[k(0.6)] pub mouth_in: f64,
    #[k(0.35)] pub lane: f64,
    /// m: on his way to the hole a man's gun is on a point this deep inside it, through the opening (he comes to the hole's edge already looking into the room,
    ///  not along the wall with the opening on his shoulder). 0 = as it was: no point, the gun leads his feet
    #[k(3.0)] pub lane_watch: f64,
    /// the points inside, [across the hole's centre line, past the far face] in metres — first man's first; mirrored / pulled in where the floor says no
    #[k(alloc::vec![[-1.3, 1.9], [1.3, 1.9], [-2.1, 1.0], [2.1, 1.0]])] pub pods: Vec<[f64; 2]>,
    /// THE TRADE: what one wall charge takes out of the squad's breaching bag: 2 = the whole bag, so a run is one wall OR
    ///  two doors, chosen at the wall — the row is offered only while the bag holds it, and all of it comes back if the charge comes off the wall unblown.
    ///  1 = a wall costs what a door costs (the build before this knob, by digest)
    #[k(2.0)] pub cost: f64,
    /// WALL CHARGES CARRIED apart from the bag, for a kit screen that sells the slot before the run: null (the default) = none are, the `cost` rule above exactly;
    ///  a number = every run starts with that many (WallCharge.carried, set per run by setCarried): the row is offered while one is left, a wall spends one and
    ///  leaves the door charges alone
    #[k(None)] pub carried: Option<f64>,
    /// … and each man takes his up by himself, once HE has been on his point `holdSecs` with his gun on the room (false: all of them together, when the last
    ///  man is on his — a mate held up in the hole left every door of the room unwatched until he was through)
    #[k(1.5)] pub hold_secs: f64,
    /// m: no door is given to a man whose line to it passes this near a mate's point inside (he would hold it with a friend down his muzzle)
    #[k(0.6)] pub mate_line: f64,
    /// m: none of this order's marks is in a DOORWAY — a stand-off post or the setter's spot within `offFrame` of a door frame's centre is passed over for
    ///  the next place (both knobs 0 = the marks as they were: on the compound a man stood off in the 'ab' door frame, 4 s and more at a time); a point INSIDE only
    ///  within `offFramePod` (the frame's own footprint: wider, and in a small room the point moves into the hole's mouth, where Sam walks in)
    #[k(0.8)] pub off_frame: f64,
    #[k(0.5)] pub off_frame_pod: f64,
    #[k(0.26)] pub outer_thick: f64,
    #[k(1.5)] pub outer_min_len: f64,
    #[k(0.5)] pub far_slack: f64,
    #[k(0.375)] pub far_reach: f64,
    #[k(0.35)] pub hung_m: f64,
    /// m: how far from the charge a man with a line to it stands off (it was `standoff`'s 2.6, then 2.0)
    #[k([4.0, 6.0])] pub safe_standoff: [f64; 2],
    /// m: on the charge's own side, with a line to it, the blast reaches this far (it was DOORTOOLS.backblastM, 1.6)
    #[k([3.0, 5.0])] pub safe_reach: [f64; 2],
    /// s a man caught inside the reach is flash-blind: at the wall · at the reach's edge (it was DOORTOOLS.backblastSecs, 1.5, flat)
    #[k([3.0, 1.0])] pub safe_secs: [f64; 2],
    /// m: a place OUT OF THE BLAST'S LINE (something solid between it and the charge that the blast cannot open) is looked for this far from the charge · and
    ///  taken only within this much walk of the wall's foot (a room away, not a floor away)
    #[k(9.0)] pub safe_hide_m: f64,
    #[k(16.0)] pub safe_walk_m: f64,
    #[k(9.0)] pub safe_back_m: f64,
    #[k(30.0)] pub fire_known_s: f64,
    #[k(30.0)] pub fire_m: f64,
    #[k(0.5)] pub fire_step: f64,
    #[k(0.03)] pub fire_grain: f64,
    #[k(0.1)] pub men_clear_in: f64,
    #[k(0.75)] pub men_clear_s: f64,
    /// SAM inside the reach when it goes: s with no round and no throw, at the wall · at the reach's edge — and his aim opened by suppression's own scalar,
    ///  whole at the wall, `samSup` of it at the edge
    #[k([2.0, 0.6])] pub sam_secs: [f64; 2],
    #[k(0.5)] pub sam_sup: f64,
}

/// A WALL SHOT, FELT (the maintainer, 10-19, of a room taken through a blown wall: "the smoke after a wall shot makes it too hard -- we should probably (1) turn
///  down its density a little and (2) make it so everyone in the room gets more of a shock"). With `on`:
///  (1) the charge's pall is asked at `pall` of its density, the picture's dust thinned with it: the hole reads sooner. Only the
///      wall charge's: a grenade's pall and the charge's borrowed report are as they were.
///  (2) EVERY defender in the room the hole opens into is shocked. One with a line to the hole, as before, but longer: folded (rooted, no round) `staggerSecs`
///      (it was DOORTOOLS.staggerSecs, 2.0) and blind `stunSecs` at the wall · at stunM (it was DOORTOOLS.stunSecs, 6.0 · 2.5). One with NO line to the hole,
///      or past stunM, who was untouched: folded the same `staggerSecs` (the shock wave fills the room; the flash needs the line, so his eyes are his own) —
///      out to `roomM` from the charge, so that a hole into a long hall or the open air does not fold a man a street away.
///      One of ours and the player beyond the wall, with a line: blind `oursSecs` (it was DOORTOOLS.backblastSecs, 1.5) — a blast does to the player what it
///      does to anyone (one of a team of TWO beyond the wall takes what a defender there takes, as he always did: the longer numbers are his too). The reach
///      inside which a man behind the wall is killed still needs the line and is as it was; no health number is touched.
///  OFF here and on the frozen stand (TPS_WALLSHOT=1); ON in the page (?wallshot=0). Check: tools/qa/wallshot-check.ts (WALLSHOT_BREAK=1).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallshotK {
    #[k(0.75)] pub pall: f64,
    #[k(3.5)] pub stagger_secs: f64,
    #[k([8.0, 4.0])] pub stun_secs: [f64; 2],
    #[k(12.0)] pub room_m: f64,
    #[k(2.0)] pub ours_secs: f64,
}

/// A WALL CHARGE KILLS WHOEVER STANDS AT IT (the maintainer, 10-19, god mode off, 0.9 m from his own charge when he blew it and told only 'you were too
///  close': "blowing a wall charge with me right next to it should kill me"). A blast does to the player what it does to anyone, so with `on` the rule is one
///  rule for every man on the floor — the player, ours, the defenders, a civilian who is in the building:
///    on the charge's OWN side, nearer than `nearM` with a line to it: killed, dead as a frag leaves a man (he was blind for the safety radius's seconds — a
///      teammate, a defender — or rocked and blind — the player; a civilian was not asked about at all);
///    BEHIND a wall it opened, nearer than DOORTOOLS.knockM with a line to the hole: down as a defender there always was (the charge's knock: 'struck') — now
///      the player too (he was blind a second or two), a civilian, and one of ours whatever the team's size (on a team of more than two he was blind a second
///      or two; on a team of two he already went down);
///    past that reach one of ours behind the wall is blind as one of ours, whatever the team's size (on a team of two he took the defenders' longer stun and fold).
///  The player goes down as a round puts him down: never in god mode, never the tour's scripted player. A man with god mode
///  of his own (Guard.god) is passed over. WHO BLOWS IT: the player's trigger and his order's release. With the safety radius on (the page) the trigger's row names who
///  is too close and who is behind the wall before he presses it. No hit points anywhere: nobody takes more or fewer rounds.
///  OFF here and on the frozen stand (TPS_WALLKILL=1); ON in the page (?wallkill=0). Check: tools/qa/wallkill-check.ts (WALLKILL_BREAK=1).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallkillK {
    #[k(1.5)] pub near_m: f64,
}
