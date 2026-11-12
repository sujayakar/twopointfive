//! The numbers of fire.
use crate::bytes::ToBytes;
use alloc::string::String;
use alloc::vec::Vec;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct FireK {
    /// a level light whose name starts with one of these is an open fire
    #[k(alloc::vec![String::from("brazier")])] pub names: Vec<String>,
    /// THE FLAME: its height over the coals (m, a calm flame's tallest tongue is about twice this) and its radius as a share of the glowing box's half-width (the box is a
    ///  plate in the middle of a drum's lid: the fire fills the drum's mouth)
    #[k(1.15)] pub height: f64,
    #[k(1.15)] pub mouth: f64,
    /// THE COALS: the glowing box the level hung on the lamp keeps this share of its authored glow while a flame stands on it (the lid alone was authored bright
    ///  enough to be the whole fire: under a flame it clips to white and the flame is lost on it), and breathes with the light by `coalSwing` of the swing
    #[k(0.26)] pub coals: f64,
    #[k(0.6)] pub coal_swing: f64,
    /// … and THE BED OF COALS the picture lays over that box (render/shaders/fire.wgsl fireCoals): its radius as a share of the box's half-width (0.27 m on a
    ///  0.29 m drum: all of its lid but the rim — the lid is lit white by its own lamp, and the bed is what hides that)
    #[k(1.42)] pub bed_r: f64,
    /// THE LIGHT: how far its strength swings either side of the authored one (the noise's weight: at 0.4 about a fifth of it most of the time, two fifths now and then, never past `least` and `most`; with the cracks' kick on top the mean runs a twentieth over the authored), the rates it swings at (Hz; firelight on a yard is slower than
    ///  the flames that make it: their flutter is the picture's own, render/shaders/fire.wgsl), how far its place wanders (m, across and up) about a point `lift` m over where the level hung it
    ///  (a level hangs the lamp a hand over the lid; the light of a fire comes from the body of its flame, and from there the drum's own shadow on the ground is
    ///  half as wide), and what a crack adds for `kickSecs`
    #[k(0.4)] pub depth: f64,
    #[k(alloc::vec![0.43, 1.3, 2.7, 4.9, 8.3])] pub rates: Vec<f64>,
    #[k(alloc::vec![0.8, 0.9, 0.7, 0.45, 0.2])] pub weights: Vec<f64>,
    #[k(0.6)] pub least: f64,
    #[k(1.6)] pub most: f64,
    #[k(0.14)] pub wander: f64,
    #[k(0.1)] pub wander_up: f64,
    #[k(0.5)] pub lift: f64,
    #[k(0.6)] pub kick: f64,
    #[k(0.12)] pub kick_secs: f64,
    /// A FIRE THROWS A FIRE'S LIGHT (the maintainer's third look, on his GPU beside the gate's flood: "still seems way too dark, especially relative to the other lights
    ///  in the scene"; "i'd also expect the color from the fire to be more warm"): the level hung a 9 on the pass's brazier where a room's bulb is 26 and the gate's flood 70,
    ///  in a lantern's colour, a hand over the lid. While it burns the lamp stands `lift` 0.5 m over the lid, in the body of the flame (at 0.25 the drum's own rim
    ///  shadowed the ground a metre and a half round it: the dark ring in his shot, which ate most of its pool), reaches × `reach` as far, its colour is × `warm` (a
    ///  deeper orange) and its strength × `bright`: 1.25 and no more, because THE TONE CURVE PALES A BRIGHT ORANGE ON SNOW TO CREAM. More strength reads as whiter, not
    ///  warmer, and a flame cannot look bright over ground brighter than itself (tried at 3 in the real page: the yard washed out and the flame went pale against it).
    ///  The warm of a fire is in the wide dimmer pool, and the brightness of it is the flame's own (`look.gain`, `look.core`). THE GAME READS IT: the light meter, and
    ///  the steady readers are told the lamp as it is (a place by a fire IS that lit). `?firebright=0` = 1, 1, no tint and 0.25: the lamp as the level hung it.
    ///  ITS PLACE SHIFTS ("shadows cast by the fire should be more dynamic i think. the fire as an emitter should be shifting continuously over time"): `wander` /
    ///  `wanderUp` above are 14 and 10 cm, where they were 6 and 4 (`?firesway=0`), which is what moves a shadow's edge. NEVER SEEN ON A GPU.
    #[k(1.25)] pub bright: f64,
    #[k(alloc::vec![1.0, 0.72, 0.51])] pub warm: Vec<f64>,
    #[k(1.4)] pub reach: f64,
    #[k(0.3)] pub stir_gap: f64,
    /// THE LAMP IS AS BIG AS WHAT BURNS (the emitter's radius while the fire burns, as a share of the mouth's, which is the flame's own reach: 0.24 m on the pass's drum; 0 = the size the level
    ///  gave it, a pinpoint, which is `?firesize=0` in the page): a flame is a hand and a half
    ///  wide and lights the ground from all of itself, so the drum's own shadow in the snow is a soft ramp. With the level's 6 cm it was a black disc with a
    ///  drawn edge a metre round the drum — the first thing seen on a GPU (the maintainer's first look: "fire looks a bit jank tbh"). `lift` was 0.2 then: 0.25
    ///  puts the lamp nearer the flame's middle and the shadow's foot nearer the drum (0.3 took more than a tenth off the light meter a stride from the drum)
    #[k(1.1)] pub size: f64,
    /// THE WOOD CRACKS: on average this often (a second), each of the clock's `popSlot`-second slots asked once
    #[k(1.1)] pub pop_rate: f64,
    #[k(0.2)] pub pop_slot: f64,
    /// THE EMBERS are GPU particles (render/particles.ts: sparks filed as embers, EmitOpts.ember — they ride the hot air up, go with the wind, land on things and
    ///  feed the bloom; the maintainer: "embers can be GPU particles though"): about `embers` in the air at once off a fire within `emberR` m of what the view looks
    ///  at, each alight `emberLife` s and leaving the flame at `emberSpeed` m/s; `emberSize` its streak's thickness (m), `emberGain` its light as a share of a
    ///  spark's; a crack throws `emberCrack` more, harder. `embers` 0 = none (`?fireembers=0`)
    #[k(60.0)] pub embers: f64,
    #[k(1.7)] pub ember_life: f64,
    #[k(1.1)] pub ember_speed: f64,
    #[k(0.004)] pub ember_size: f64,
    #[k(0.45)] pub ember_gain: f64,
    #[k(10.0)] pub ember_crack: f64,
    #[k(30.0)] pub ember_r: f64,
    /// THE LOOK OF THE FLAME (render/shaders/fire.wgsl FireLook; the picture only): × its light, × its white heart (the
    ///  bloom spreads it), how soon it hides what is behind it (1/m), × how fast its shapes climb, × its height, how torn it is (0‥2), the smoke drawn off its
    ///  tips (0‥2; `?fireplume=0` = none), the shimmer of the hot air over it (m the picture behind is pushed aside; `?fireheat=0` = none), how dark its cool
    ///  edge is (0‥1) and how hard it breathes (0‥2). Set on the software adapter's stills against the maintainer's reference (a fluid-simulated camp fire: a white
    ///  heart, orange body, dark red tips, smoke off them), after his second look on a GPU: "still very ugly. needs more dynamic range, smoke, embers, movement,
    ///  could we do heat distortion?" NEVER SEEN ON A GPU.
    /// THESE NUMBERS ARE NOW THE MAINTAINER'S OWN (and the JSDoc above is how they came to be first), set by eye on his GPU with live sliders
    /// (2026-10-24) and taken as he sent them: the look's first ten here, and in this table `height`, `coals`, `depth`, `kick`, the embers'
    /// and the smoke's density, temp and speed. He set them on the build BEFORE the newer rules (the flame marked for the temporal resolve, the lifted and tinted
    /// lamp, the lean, the lamp's wider shift, twice the cracks), so with those on the picture is not the one he tuned: the nearest to that is
    /// `?firereact=0&firelean=0&firebright=0&firesway=0`. `wander` is not his 0.07: his note asking for shadows that move came with it (FIRE_WANDER).
    pub look: FireKLook,
    pub plume: FireKPlume,
    /// THE SMOKE (the solver's units: 6 /m of extinction a unit of density, +1.6 m/s² a unit of heat): its source's least width in cells, what it lets out a second, how hot, how fast it
    ///  leaves, how hard the air is dragged along and how much its way wobbles; filed afresh every `every` s for that long. Where it stands and how wide is `plume`'s.
    ///  `on` and `view_r` (m) are the picture's: a fire's painted soot hands over to its smoke only with the one, and within the other of what the view looks at
    pub smoke: FireKSmoke,
    /// THE SOUND: the bed's level at the fire (heard for as long as it burns) and each crack's (× how hard the crack is)
    #[k(0.85)] pub bed: f64,
    #[k(0.9)] pub crack: f64,
    /// the renderer takes this many fires (four at most: its uniform's size), nearest the view first, none past `drawR` m of what it looks at; the wind a flame and
    ///  its embers are shown is capped at `windMax` m/s (a storm lays the smoke over, not the flame flat)
    #[k(4.0)] pub max: f64,
    #[k(70.0)] pub draw_r: f64,
    #[k(3.0)] pub wind_max: f64,
}

/// THE LOOK OF THE FLAME (render/shaders/fire.wgsl FireLook; the picture only): × its light, × its white heart (the
///  bloom spreads it), how soon it hides what is behind it (1/m), × how fast its shapes climb, × its height, how torn it is (0‥2), the smoke drawn off its
///  tips (0‥2; `?fireplume=0` = none), the shimmer of the hot air over it (m the picture behind is pushed aside; `?fireheat=0` = none), how dark its cool
///  edge is (0‥1) and how hard it breathes (0‥2). Set on the software adapter's stills against the maintainer's reference (a fluid-simulated camp fire: a white
///  heart, orange body, dark red tips, smoke off them), after his second look on a GPU: "still very ugly. needs more dynamic range, smoke, embers, movement,
///  could we do heat distortion?" NEVER SEEN ON A GPU.
/// THESE NUMBERS ARE NOW THE MAINTAINER'S OWN (and the JSDoc above is how they came to be first), set by eye on his GPU with live sliders
/// (2026-10-24) and taken as he sent them: the look's first ten here, and in this table `height`, `coals`, `depth`, `kick`, the embers'
/// and the smoke's density, temp and speed. He set them on the build BEFORE the newer rules (the flame marked for the temporal resolve, the lifted and tinted
/// lamp, the lean, the lamp's wider shift, twice the cracks), so with those on the picture is not the one he tuned: the nearest to that is
/// `?firereact=0&firelean=0&firebright=0&firesway=0`. `wander` is not his 0.07: his note asking for shadows that move came with it (FIRE_WANDER).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FireKLook {
    #[k(3.0)] pub gain: f64,
    #[k(4.0)] pub core: f64,
    #[k(2.5)] pub sigma: f64,
    #[k(0.7)] pub speed: f64,
    #[k(0.95)] pub tall: f64,
    #[k(1.1)] pub torn: f64,
    #[k(1.0)] pub plume: f64,
    #[k(0.019)] pub shimmer: f64,
    #[k(0.5)] pub soot: f64,
    #[k(1.0)] pub breathe: f64,
    /// THE PICTURE'S MEMORY LETS A FLAME GO (0‥1; `?firereact=0`): the temporal resolve (taau.wgsl) keeps about ten frames of every pixel, forty where the picture
    ///  is all contrast, and a flame is a new shape every frame: on a GPU it was the average of forty flames, soft, dim, see-through and nearly still ("pale blob",
    ///  "feels very static", "way too dark" — three looks, one cause, and the software adapter's stills, a frame at a time with no history, never showed it). The
    ///  composite marks a fire's pixels and the resolve takes them from this frame by this much. `lean` = how far the wind leans the flame: metres at one flame
    ///  height for each m/s (it was 0.07: "would also expect the fire to be affected by the wind, looks like only the smoke is"; `?firelean=0` = 0.07)
    #[k(1.0)] pub react: f64,
    #[k(0.2)] pub lean: f64,
    /// HOW MANY SAMPLES OF THE FLAME A PIXEL'S RAY TAKES (4‥64; `?firesteps=N`): the flame's price is this × the pixels it covers, and a flame
    ///  this thin (sigma 2.5) never goes opaque, so no ray stops early. 40 is the count the maintainer's own numbers were tuned at; fewer is cheaper and bands
    ///  sooner. A lever for a timing on a real GPU: nobody has timed the flame, and the count's price is a model.
    #[k(40.0)] pub steps: f64,
}

/// THE SMOKE (the solver's units: 6 /m of extinction a unit of density, +1.6 m/s² a unit of heat): its source's least width in cells, what it lets out a second, how hot, how fast it
///  leaves, how hard the air is dragged along and how much its way wobbles; filed afresh every `every` s for that long. Where it stands and how wide is `plume`'s.
///  `on` and `view_r` (m) are the picture's: a fire's painted soot hands over to its smoke only with the one, and within the other of what the view looks at
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FireKSmoke {
    #[k(true)] pub on: bool,
    #[k(4.0)] pub cells: f64,
    #[k(1.0)] pub density: f64,
    #[k(21.0)] pub temp: f64,
    #[k(0.9)] pub speed: f64,
    #[k(6.0)] pub push: f64,
    #[k(0.5)] pub jitter: f64,
    #[k(1.0)] pub every: f64,
    #[k(18.0)] pub view_r: f64,
}

/// THE COLUMN OVER A FIRE: the smoke it lets out on the solver (a steady source and, a few a second, puffs of thicker, hotter gas), the way its embers leave, and its hot air
/// on the falling snow. The picture alone. Density and heat are the solver's units (see `FireKSmoke`), and "how hard it burns" is the fire's heat, 1 at full strength.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FireKPlume {
    /// the steady source: how high its middle is over the coals, as a share of `height` · its radius (m) · × `smoke.density` and how hard it burns
    #[k(0.8)] pub up: f64,
    #[k(0.2)] pub radius: f64,
    #[k(0.8)] pub density: f64,
    /// seconds from one puff to the next
    #[k(0.42)] pub puff_every: f64,
    /// how long a puff blows (s): up to full in 0.06 s, fading over its last quarter
    #[k(0.26)] pub puff_secs: f64,
    /// how high a puff's middle is over the coals, as a share of `height` · its radius (m)
    #[k(0.95)] pub puff_up: f64,
    #[k(0.24)] pub puff_radius: f64,
    /// m: a puff starts up to this far off the column's middle, dealt on each level axis, and the same two numbers tip the way it leaves
    #[k(0.09)] pub puff_off: f64,
    /// what it lets out a second, × how hard the fire burns: density · heat. Then how fast it leaves (m/s). All three also × a share dealt each puff, 0.7‥1.3
    #[k(3.2)] pub puff_density: f64,
    #[k(30.0)] pub puff_temp: f64,
    #[k(1.25)] pub puff_speed: f64,
    /// how hard it drags the air along · how fast it swells as it leaves (1/s)
    #[k(9.0)] pub puff_push: f64,
    #[k(2.2)] pub puff_expand: f64,
    /// the color of the smoke, from both sources
    #[k([0.46, 0.48, 0.53])] pub soot: V3,
    /// per second: how fast the soot painted in the flame hands over to this column as the fire comes near (`smoke.view_r`), and back (0.5 = two seconds)
    #[k(0.5)] pub soot_ease: f64,
    /// the embers: where they start over the coals, as a share of the flame's height · their half-angle (radians; a hard crack adds up to 0.6) · how far off the middle they start, as a
    /// share of the mouth's radius · × `ember_speed` (a hard crack makes that up to 2.6 times faster)
    #[k(0.72)] pub ember_up: f64,
    #[k(0.2)] pub ember_spread: f64,
    #[k(0.4)] pub ember_jit: f64,
    #[k(1.5)] pub ember_speed: f64,
    /// the hot air, as the falling snow meets it
    pub snow: FireKPlumeSnow,
}

/// A FIRE'S HOT AIR ON THE FALLING SNOW: a jet up the flame's leaning middle, where the look has `fire_snow` on. It carries the flakes and melts them. The picture alone.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FireKPlumeSnow {
    /// m/s at the mouth, × how hard the fire burns. The melt reads the air's speed as a share of a fixed 2.6, so this moves the melt too
    #[k(2.6)] pub speed: f64,
    /// m of climb at which it has slowed to half
    #[k(2.2)] pub reach: f64,
    /// m it widens per meter of climb
    #[k(0.2)] pub spread: f64,
    /// its radius at the mouth, as a share of the fire's mouth
    #[k(1.3)] pub mouth: f64,
    /// m of climb where it stops, fading out from 70% of that. Nothing past it feels the jet
    #[k(4.5)] pub top: f64,
}

