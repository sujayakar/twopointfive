//! The numbers of digin.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DiginK {
    /// THE switch: off = nothing here runs (the room fight as it was)
    #[k(false)] pub on: bool,
    /// REACH (m, on foot): a man whose way to the fight he hears is longer than this cannot join it in time — he holds his room and digs in, whatever the room
    ///  fight's rally would have rolled; a man nearer than this is his squad's to send. 0 (shipped) = no reach rule. MEASURED at 18 m: it claims six men in ten on every level there is (they all sit in one
    ///  32 × 20 m shell), and men kept home are men not on the hunt — the scripted team loses FEWER. It is the knob a press / push rule wants beside it (near men
    ///  go at once, far men get ready, then come): by itself it only takes the hunters away. AS BUILT the way is measured ONCE, when he first hears them, to the
    ///  noise (or the net's fix) that told him: a fight that later moves next door does not release a man judged far, nor claim one judged near
    #[k(0.0)] pub reach_m: f64,
    /// he hears them coming once HE has noticed them, so a calm man at 1–20 % awareness does not radio 'they're on us. watch your doors!' half a second before his own 'huh?', and a man
    ///  dropped on his first glimpse has told no room. Noticed = roused (alert or searching: the line the squads' net draws for a man's eyes) AND `noticed_secs` of them in
    ///  his eyes without a break: this tick runs before the room fight hands a man his own start, so 'past his own start' alone would let a roused man's word out on the very frame of the glimpse
    #[k(0.35)] pub noticed_secs: f64,
    /// a man who falls back to a room dug in (`dig_in_refuge`) goes no further than this by the floor (m), and never by a way that passes this near the noise (m)
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
