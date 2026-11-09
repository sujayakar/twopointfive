//! The numbers of fogofwar.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The knobs (Tab ▸ Sandbox ▸ Fog of war, __fow.knobs; read every frame).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FogK {
    /// how far down the unseen floor and walls are pushed: radiance × this …
    #[k(0.4)] pub dim: f64,
    /// … and how far toward grey (0 = colour kept, 1 = grey). 0 since 10-15 — the maintainer, on a replay of the mouse hole: 'given how pretty the game is, i think
    ///  having fog of war mute the colors is unfortunate' (was 0.75: ?fogdesat=0.75 puts it back, ?fogdesat=share another). The picture only. Never seen on a GPU
    #[k(0.0)] pub desat: f64,
    /// each eye's field in plan, degrees (360 = all round — the default; under it a wedge about where his chest and head point)
    #[k(360.0)] pub fov_deg: f64,
    /// with a wedge: inside this many metres he sees all round regardless
    #[k(2.0)] pub near_m: f64,
    /// THE CONE (hard's fog: difficulty.ts setDifficulty writes this row from HARD.cone with the level; ?fogcone / ?fogcone=0 for a visit; the Tab row flips it
    ///  live): every eye sees a wedge of `coneDeg` about where he looks with `coneNearM` all round him, in place of fovDeg / nearM. false = as it was
    #[k(false)] pub cone: bool,
    /// … the wedge, degrees in plan (a man's two eyes together take in about 120°; the defenders' own cone is 116° — guards.ts GUARD_EYE)
    #[k(120.0)] pub cone_deg: f64,
    /// … and the all-round near field, metres (a man knows what is at his shoulder)
    #[k(1.5)] pub cone_near_m: f64,
    /// the PICTURE's edges under the cone: the dimming comes in over this many degrees across the wedge's edge (0 = a hard line, as fovDeg's wedge has) …
    #[k(12.0)] pub cone_soft_deg: f64,
    /// … and over this many metres outward from the near field's rim
    #[k(0.6)] pub cone_near_soft_m: f64,
    /// ONLY THE PLAYER'S OWN EYES lift the fog: the teammates are no eyes (false = the team's sight is shared, as it was). An option on any level of difficulty:
    ///  ?fogown / ?fogown=0 for a visit
    #[k(false)] pub own: bool,
    /// … s before the same enemy is called again · s between any two calls on the floor · s an enemy the player's own eye has just had goes uncalled
    #[k(10.0)] pub call_secs: f64,
    #[k(3.0)] pub call_gap: f64,
    #[k(3.0)] pub call_known: f64,
    /// the air along the line (smoke, and a picked weather's fog and dust): transmittance under this and the man is lost in it (the guards' own bar — guards.ts eyeContact; 0 = the air hides nobody)
    #[k(0.3)] pub smoke: f64,
    /// seconds a man stays drawn after the last frame somebody saw him
    #[k(0.25)] pub linger: f64,
    /// the picture only: a top face above the eye (a wall's top, a tall cabinet's) is taken as seen when the line reaches within this of it — 0 = wall tops always dim
    #[k(0.3)] pub top_pad: f64,
    #[k(2.0)] pub squad_doors_m: f64,
    #[k(1.0)] pub squad_fire_secs: f64,
    /// THE FOG'S OWN LIGHT (rule 4e; the picture only): > 0 = a pixel nobody on the team sees is the building's paint under one fixed light at this displayed
    ///  level — nothing of the LIVE frame is in it: no lamp or weapon light coming on in a room nobody watches, no muzzle flash, tracer, smoke, beam
    ///  or particle there. 0 = the live frame dimmed and greyed, as it was: a guard's torch in a room nobody watches lit that room's floor in the fog and gave him
    ///  away (the maintainer, 10-13: 'you can see enemies flashlights in fog of war'). ON in the page since then at FOG_FLAT_ON — on software-adapter stills
    ///  of the mouse hole the level at which a lamp-lit room nobody sees is as bright as the old fog had it (0.2 was 40% brighter). ?fogflat=0 = the fog as
    ///  it was, ?fogflat=level another level; the frozen harness stand pins 0 (src/host/standup.ts, `TPS_FOGFLAT=1`). THE COST: a DARK room nobody sees is
    ///  the same grey as a lit one (it was black), so on a night map the fog can be brighter than the ground the team stands on. Never seen on a GPU: the
    ///  level, and whether the fog should rather keep dark rooms dark, are the maintainer's to judge
    #[k(0.12)] pub flat: f64,
    /// THE FOG LIT AS THE TEAM LAST KNEW IT (rule 4f; the picture only; read only while `flat` > 0 — with that at 0 the fog is the live frame dimmed, whatever
    ///  this says): > 0 = an unseen pixel is its paint under the level's own lamps as the team last saw each (their light × this × `dim`, with shadows from the
    ///  picture's own boxes) + the sun or moon + `flat`'s fixed light as a fill. 0 = `flat`'s one fixed light alone, as it was (?foglit=0; the frozen stand
    ///  pins 0 — src/host/standup.ts, `TPS_FOGLIT=1`, with `TPS_FOGFLAT=1`). ON in the page since 10-14 at FOG_LIT_ON. Never seen on a GPU: the levels, the fill
    ///  and what a never-seen room should look like are the maintainer's to judge
    #[k(1.0)] pub lit: f64,
    /// THE AIR IS THE PLACE (the picture only; under `full` alone — it rides the sight table): true = the solver's smoke is drawn over floor the team does not see
    ///  too, dimmed with the ground behind it on the screen — a fire's plume, a grenade's cloud (the maintainer, 10-23, on a fire with no flame in the fog:
    ///  'I think we should at least show everything in the environment visible at the start of the level all the time?'). What a hidden MAN breathes, smokes or fires
    ///  is not filed with the solver at all (snowBreath.ts, smokebreak.ts and combat.ts ask the fog at the source — airHides below), and a hidden light still gives
    ///  a plume nothing (lightFogK). The puffs the PARTICLES draw (a blast's dust ring, a round's chip of plaster) are `ours`'s, below: particles_draw.wgsl. false = the
    ///  march takes no density over unseen floor, as it was (common.wgsl fogSmoke). ON in the page since 10-23 (?fogair=0 puts it off; the frozen stand pins it
    ///  off — src/host/standup.ts, `TPS_FOGAIR=1`). Never seen on a GPU
    #[k(true)] pub air: bool,
    /// HOW MANY EYES LIFT THE FOG (the fourth of the fog's principles: to see more, get more eyes on it): Sam, the living teammates in list order, then a
    ///  sticky camera's feed, then a tutorial's lookout or a step's own eye — as many as this. 5 until 10-24: a squad of four filled it, so a camera that sent
    ///  was 'drawn, blind' (stickycam.ts STICKY_STATS.blind) and a lookout had no place. 7 = the room the renderer's block and its sight table have
    ///  (render/fogofwar.ts FOG_EYES; fogEyeCount below: under five is five, over seven is seven). Not the picture alone: an eye more is a man more the squad may have seen. 5 = as it was. 7 in the
    ///  page since 10-24 (?fogeyes=5 puts it back; the frozen stand pins 5 — src/host/standup.ts, `TPS_FOGEYES=1`). Never seen on a GPU
    #[k(7.0)] pub eyes: f64,
    /// WHAT OTHERS LEFT BEHIND STAYS AS THE TEAM LAST SAW IT (the third of the fog's principles; the picture alone, under `full`): (1) a lamp the team has in
    ///  another state than it is in keeps its FACE that way too — the fog already lit the room by a lamp put out unseen while its ceiling panel went dark at
    ///  once (Box.fogEmit, which the upload alone reads: the game's Box.emissive is the real lamp's); (2) a case, a chip or a leaf of paper at rest that the
    ///  team has seen lying there stays in the picture when they look away (render/fogofwar.ts FogFrame.kept → shaders/particles.wgsl), where it used to be
    ///  cut and come back. false = both as they were. ON in the page since 10-24 (?fogkept=0 puts it off; the frozen stand pins it off —
    ///  src/host/standup.ts, `TPS_FOGKEPT=1`). Never seen on a GPU
    #[k(true)] pub kept: bool,
    /// … a fill added on top of the dimmed frame: this share of `flat`'s level, corner-shaped by `litAo` / `litAoM` (0 = none — a dark room is dark in the fog)
    #[k(0.0)] pub full_fill: f64,
    /// THE FOG'S SOFT EDGE (the maintainer, 10-15, on the pass level: 'fog of war edges shouldn't be so sharp'; the picture only; under `full` alone — it reads the
    ///  sight table): the band, in metres, the fog comes in over ACROSS a sight edge — the line a wall's corner casts across open floor — half on each side: the
    ///  last quarter metre of what is seen is a little dimmer, the first quarter metre of what is not a little less (it is the fog's own picture there, dimmed
    ///  less: nothing hidden is drawn, and a hidden light is still cut on the line). A wall's own face keeps its line; the band is nothing at the corner that casts
    ///  the line and widens behind it. 0 = the hard line as it was (?fogedge=0, ?fogedge=metres another band; the frozen stand pins 0 — src/host/standup.ts,
    ///  `TPS_FOGEDGE=1`). ON in the page since 10-15 at FOG_EDGE_ON. Never seen on a GPU: the width is the maintainer's to judge
    #[k(0.5)] pub edge: f64,
    /// … its fill: the share of `flat`'s level that every unseen surface keeps whatever its lamps do (under 1, so a dark room reads darker than a lit one)
    #[k(0.5)] pub lit_fill: f64,
    /// … how much of that fill a corner, the foot of a wall or the floor under a table loses (0 = none, and no rays asked) and how far a box shuts it out from (m)
    #[k(0.75)] pub lit_ao: f64,
    #[k(1.5)] pub lit_ao_m: f64,
    /// … a lamp that would add less than this, as displayed, at a pixel is not traced (one under twice it is faded in: no line where a lamp's reach ends)
    #[k(0.006)] pub lit_cut: f64,
    /// WHERE HE WAS LAST SEEN: a living enemy who has gone out of the team's sight keeps a fading '?' on the spot he was last seen at for this many seconds
    ///  (the HUD's alone: the Rust game's fog_of_war_last_seen, game.view.marks.seen — nothing in the game reads it). 0 (as it ships) = the 'you're told' lever decides (told.ts: a few seconds on
    ///  Standard, longer on Guided, none on Bare or with that lever off, which is no mark, as it was); turned up, this is the seconds on every rung
    #[k(0.0)] pub last_seen: f64,
}
