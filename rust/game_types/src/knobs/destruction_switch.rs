//! The numbers of destruction switch.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// KICKED RUBBLE: men on their feet shove the broken pieces lying in their way aside, and a piece that
///  came to rest in a man is pushed off him. OFF here, so everything headless stands as it was (the pass is not called: no hash drawn, no piece touched); the PAGE turns
///  it on at boot unless the URL carries ?nokickrubble, tools/qa/playthrough.ts under PT_KICK=1. A boot's shove is silent to the AI (no
///  hearing event), pushes no furniture and breaks nothing. The numbers: `radius` a man's (m), `reach` a boot's past it, `inside` how far a piece must lie INTO him to
///  be pushed off a man standing still, `floor` how far over his own floor a piece's underside may lie and still be at his feet (one on a desk top is not), `heavy`
///  the mass (kg) past which a boot shifts nothing (a cabinet on its side: he walks through it as before), `walk` the pace (m/s) under which he steps over things;
///  `lead` how far ahead of his middle (s of his own pace) his leading foot meets things; a kick's speed = (base + gain × his pace) ÷ (1 + mass/4), ≤ vMax, `aside`
///  of it across his line; `off` the push off a man (m/s); `slide` the floor's friction under a shoved piece (it stops in v² ÷ 2·slide·g);
///  `spin` the turn a boot on one end gives; `perFrame` shoves a frame, `cooldown` s per piece, a piece shoved `giveUp` times running let be for `giveUpSecs`;
///  `scuff` the gain of the sound it makes for the ear alone (0 = none).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RubbleK {
    #[k(0.3)] pub radius: f64,
    #[k(0.12)] pub reach: f64,
    #[k(0.12)] pub lead: f64,
    #[k(0.06)] pub inside: f64,
    #[k(0.12)] pub floor: f64,
    #[k(12.0)] pub heavy: f64,
    #[k(0.35)] pub walk: f64,
    #[k(0.6)] pub base: f64,
    #[k(0.8)] pub gain: f64,
    #[k(1.2)] pub aside: f64,
    #[k(2.2)] pub v_max: f64,
    #[k(1.0)] pub off: f64,
    #[k(0.3)] pub slide: f64,
    #[k(2.2)] pub spin: f64,
    #[k(4.0)] pub per_frame: f64,
    #[k(0.3)] pub cooldown: f64,
    #[k(8.0)] pub give_up: f64,
    #[k(3.0)] pub give_up_secs: f64,
    #[k(0.12)] pub scuff: f64,
}

/// THE DEBRIS POOL IS DRESSING: the pieces a thing that breaks throws are the picture's alone, and what the game takes from a piece — that the
///  floor hears it come down — is decided by the SIMULATION when the piece is thrown: one landing in the game's falls, at the
///  time and the place a body let go there at that speed reaches the floor (stopped short of the first wall in its way), told to the floor at the piece's level
///  and on whoever threw it. From there on nothing a machine draws writes the game: a pool that is full, small (a lower quality tier's) or not there at all hears
///  the same landings; a chunk's own knock is for the ear; a chunk landing on a pushable thing leaves it be; a piece a later blast kicks is not heard again;
///  and the three places where a refused piece was fed back into what breaks next (a ceiling tile counted as still up, a blast's piece budget, the break table's
///  share and its die) ask the simulation instead. OFF here, so everything headless stands as it was; the PAGE raises it at boot unless the URL carries
///  ?debrisfall=0, a headless run with TPS_DEBRISFALL=1. `minSpeed`: a piece that would meet the floor slower (m/s) makes no
///  landing, as a chunk's own touch under it made no knock.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DebrisK {
    #[k(1.0)] pub min_speed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DestructionK {
    #[k(true)] pub on: bool,
}
