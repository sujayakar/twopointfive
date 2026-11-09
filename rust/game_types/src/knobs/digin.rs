//! The numbers of digin.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DiginK {
    /// THE switch: off = nothing here runs (the room fight as it was)
    #[k(false)] pub on: bool,
    /// EVERY man who hears them coming holds his room and digs in — nobody goes to the fight next door. Off (shipped): the room fight's rally decides first, as
    ///  ever (ROOMFIGHT.rallyK is the dial for how many go), and only a man who holds his room digs in. MEASURED: with nobody on the hunt the scripted team loses
    ///  FEWER men than today, flashbangs in hand — the men who come to the fight are what kills it between doors
    #[k(false)] pub stay: bool,
    /// REACH (m, on foot): a man whose way to the fight he hears is longer than this cannot join it in time — he holds his room and digs in, whatever the room
    ///  fight's rally would have rolled; a man nearer than this is the rally's (or a press rule's) to send, as ever. 0 (shipped) = no reach rule: the rally
    ///  decides for everybody and only the men it leaves at home dig in. MEASURED at 18 m: it claims six men in ten on every level there is (they all sit in one
    ///  32 × 20 m shell), and men kept home are men not on the hunt — the scripted team loses FEWER. It is the knob a press / push rule wants beside it (near men
    ///  go at once, far men get ready, then come): by itself it only takes the hunters away. AS BUILT the way is measured ONCE, when he first hears them, to the
    ///  noise (or the net's fix) that told him: a fight that later moves next door does not release a man judged far, nor claim one judged near
    #[k(0.0)] pub reach_m: f64,
    /// WITH THE PRESS ON TOO: the man a press leaves to KEEP his room digs in. Off: he does not,
    ///  from the moment he is left until the fight is reset — he is the press's keeper and nothing more, and its last-man rules send him out by another door when
    ///  the team's boots reach his. MEASURED (The block, the depot, the records building): the keeper is a man who at standard came hunting; dug in he is behind
    ///  the one door a stun canister comes through, and costs the team less than the hunter he was. A man the press has left nowhere (no fight he could go to) still digs in, as ever; a man
    ///  already at his furniture when he is made the keeper carries on with it
    #[k(true)] pub keeper: bool,
    /// move to a covered spot that holds the door they will come by (off: he turns to that door from where he stands)
    #[k(true)] pub cover: bool,
    /// WHERE HE MAY GO FOR IT: the spot he pushes the furniture from is in HIS OWN ROOM and so is the way to it. 'Behind the prop' is a point 0.7 m past it from
    ///  the door: for a bin or a carton against a wall that point is IN the wall, and the nearest free floor to it can be the far side — the warehouse's dock men
    ///  walked out into the lot for a carton by the east wall and held the building from outside it. On: such a prop is pushed from the best spot behind it inside
    ///  the room (within reach of it, nothing solid between, and the haul from there to the door inside the room too), else the next nearest prop is tried, else no furniture (he shuts the door and takes cover, as a man with nothing to haul does). Off = as it was:
    ///  only a last-stand man is refused a goal outside his room
    #[k(true)] pub brace_in_room: bool,
    /// … once HE has noticed them (on; off = on his first glimpse, as it was — the 10-13 tapes: a calm man at 1–20 % awareness radioing 'they're on us. watch
    ///  your doors!' half a second before his own 'huh?', and a man dropped 0.18 s after his first glimpse having told every room). Noticed = roused (alert or
    ///  searching: the line the squads' net draws for a man's eyes, enemysquads.ts) AND `noticedSecs` of them in his eyes without a break: this tick runs
    ///  before the room fight hands a man his own start, so 'past his own start' alone let a roused man's word out on the very frame of the glimpse
    #[k(0.35)] pub noticed_secs: f64,
    /// WHERE DOORS STAY AS THEY ARE LEFT (doors.ts DOORSTAY — nothing below runs where every leaf has its closer, or with this off: the 2.5 s wait and his own
    ///  door only, as it was): a man who has pulled a door of his room to keeps his hand on it until the latch has it — a leaf with no closer takes four or five
    ///  seconds from its stop, not the moment a sprung one had left, and he used to walk off from his furniture before it was home, the wedge never placed. He
    ///  pushes it home the last of the way (a one-way leaf: it comes to on its frame) and pulls it again if something stopped it. And before he goes to cover he
    ///  shuts the room's OTHER doors that stand open, nearest first: never one a mate has in hand, never by a way that leaves his room, and not at all once
    ///  they are heard at any door of it (shut or open: then he takes cover, as he did).
    ///  SHIPS OFF: it overshoots the build with closers. On the barracks (144 scripted seeds) defenders wedged 669 doors with it on against 248 on the old build
    ///  and 249 with it off, the squad booted 74 against 29, and the men it lost in open-and-clear entries went from 62 to 105. On the block with three squads it
    ///  only puts back what closers gave (wedges 133 on, 74 off, 72 seeds). Tab ▸ Sandbox ▸ Defenders dig in turns it on
    #[k(false)] pub shut_open: bool,
    /// …seconds he gives a pulled leaf to latch before he lets it be · how near home (rad) his hand pushes it the rest · the other doors one man goes to at most
    #[k(7.0)] pub shut_secs: f64,
    #[k(0.3)] pub shut_home_rad: f64,
    #[k(2.0)] pub shut_doors: f64,
    /// DOUBLE UP (off as shipped: measured, it puts two men under one flashbang and the team loses fewer): a man alone in his room, who hears them coming, runs to
    ///  a room of his side's that still has men in it and digs in THERE (said on the net) — the team finds his room empty and the next one full
    #[k(false)] pub gather: bool,
    /// … no further than this by the floor (m), and never by a way that passes this near the noise (m)
    #[k(24.0)] pub gather_m: f64,
    #[k(6.0)] pub gather_shy_m: f64,
    /// seconds between hearing it and moving [min, max] — his own, drawn per man
    #[k([2.0, 5.0])] pub think: [f64; 2],
    /// seconds of work at the door to wedge the furniture under it [min, max] — bumps heard the whole time; a door opened meanwhile is not wedged
    #[k([4.0, 7.0])] pub wedge_secs: [f64; 2],
    /// doors one room wedges at most (the one they will come by first, then the next likeliest, while there is furniture and time)
    #[k(2.0)] pub wedges: f64,
    /// caught with his hands on the furniture or the door: seconds MORE before his first round [min, max] (his gun was not in his shoulder)
    #[k([0.5, 0.9])] pub caught: [f64; 2],
    /// a teammate (or Sam) who stands this long (s) at the outside of a wedged door has tried the handle: he says it is blocked — BEFORE anybody boots it
    #[k(0.8)] pub try_secs: f64,
    /// every step's length × this (1 = as shipped; 2 = digging in takes twice as long — measurement: a team that arrives twice as soon)
    #[k(1.0)] pub pace: f64,
    /// the noise this near the door he would brace (m): no time for furniture, cover only
    #[k(6.0)] pub close_m: f64,
    /// how far a man goes for a prop / hauls it (m, straight)
    #[k(9.0)] pub prop_m: f64,
    /// hauling pace (m/s)
    #[k(0.7)] pub haul_speed: f64,
    /// the furniture's scrape: hearing level (a shoved chair is ~0.3–0.5) and seconds between scrapes
    #[k(0.5)] pub scrape_level: f64,
    #[k(0.8)] pub scrape_every: f64,
    /// a shout carries this many metres of open air (× the propagation's carry)
    #[k(18.0)] pub shout_m: f64,
    /// a second door counts as a way in they might take when its way round is at most this much longer (m)
    #[k(10.0)] pub alt_m: f64,
    /// a new spot must beat where he stands by this much (score)
    #[k(0.6)] pub better: f64,
    /// the fear he keeps on his door while dug in (the room fight's evidence score, 0‥3), and for how long after the last loud noise (s)
    #[k(2.4)] pub fear: f64,
    #[k(90.0)] pub hold_secs: f64,
    /// dug in on one door, the noise now says another: seconds before he shifts
    #[k(6.0)] pub redig_secs: f64,
    /// The spot he takes to cover his door is never one in such a lane's line, and a man already set whose spot comes to lie in one — nobody in his own sights —
    /// picks again, at most every `laneSecs`: another spot of HIS room on the same door, out of the line. No such spot = he stays where he is, as before; he
    /// still does not give the room up (holdSet). false = spots as they were dealt before the rule
    #[k(2.0)] pub lane_secs: f64,
}
