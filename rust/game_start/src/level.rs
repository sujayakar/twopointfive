//! A LEVEL FILE INTO THE GAME, record by record: what `file.rs` read, laid. The level's boxes become rows of `g.tab.boxes` (listed in the answer, in the file's order: index i of the file IS static
//! box i), its routes rows of `g.tab.route`, and `g.level` is the `Level` the game's constructor reads.
//!
//! A start refuses a file with any fault of the reading (`file.rs`): a level stood without part of itself is another level. The rules a file is held to beside that (an id that points nowhere, a
//! number out of its range) are `faults.rs`, which a level is asked before it is committed, and not at every start.
use crate::file::{self, BoxData, FlagName, LeafPaintData, LevelFile, LightData, LightDataKind, Material, Part, ShapeData, ShapePieceData};
use crate::json::Json;
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use game::game::Game;
use game::ids::*;
use game::js::JsMap;
use game::scene_boxes::MakeBoxP;
use game::scene_level::*;
use game::scene_markings::{MarkPal, MarkRes, MarkingDef};
use game::scene_materials::DoesWord;
use game::scene_stuff::BoxStuff;
use game::V3;

/// a word of the file as one of a type's words (the types say their words by `as_str`; this is the other way, over a list written once here)
macro_rules! words {
    ($f:ident, $t:ty, [$($v:ident),* $(,)?]) => {
        pub fn $f(w: &str) -> Option<$t> { $(if <$t>::$v.as_str() == w { return Some(<$t>::$v); })* None }
    };
}
words!(stuff_word, BoxStuff, [Carpet, Grass, Rubber, Metal, Glass, Wood, Plastic, Fabric, Steel, Enamel, Tarmac, MudWall, Plaster, Block, Earth, Gravel, Slab, Sheet, Cloth, Paint, Veneer, Carpaint, Stucco, Skim, GlossBlock, BoardConcrete, CastPanel, Paver, Brushed, Louvre, Plain, Waits]);
words!(does_word, DoesWord, [Snow, Carpet, Lino, Stone, Grass, Wood, Plastic, Concrete, Fabric, Steel, Enamel, Tarmac, Casing, Screen, UnnamedDark, Plasterboard, Partition, Ceiling, Laminate, Pane, DarkCarpet, Card, Frame, Plant, PackedSnow, OldSnow, Unnamed, UnnamedGrey, UnnamedLight, UnnamedLightGrey, UnnamedPale, UnnamedPaleGrey, DarkPlastic, GreyPlastic, DarkGlass]);

/// the box flags a file may say, as the bits of `Box.flags`
fn flags_to_bits(names: &[FlagName]) -> f64 {
    names.iter().fold(0u32, |bits, n| bits | match n { FlagName::NoPrimary => game::scene_boxes::box_flag::NO_PRIMARY as f64, FlagName::NoShadow => game::scene_boxes::box_flag::NO_SHADOW as f64, FlagName::Cutaway => game::scene_boxes::box_flag::CUTAWAY as f64, FlagName::Gloss => game::scene_boxes::box_flag::GLOSS as f64, FlagName::Glass => game::scene_boxes::box_flag::GLASS as f64 } as u32) as f64
}

/// A LIST OF MARKINGS AS IT IS LAID (the structure a generator builds): a list of none is no list, the base size goes unsaid, a colour of its own leaves no multiplier and a multiplier of 1 is none, and a colour
/// that emits nothing emits none.
fn laid_markings(ms: Option<Vec<MarkingDef>>) -> Option<Arc<[MarkingDef]>> {
    let ms = ms.filter(|ms| !ms.is_empty())?;
    Some(ms.into_iter().map(|m| MarkingDef {
        res: m.res.filter(|&r| r != MarkRes::Base),
        pal: m.pal.into_iter().map(|p| MarkPal {
            mul: if p.albedo.is_some() { None } else { p.mul.filter(|&x| x != 1.0) },
            emit: p.emit.filter(|e| e[0] > 0.0 || e[1] > 0.0 || e[2] > 0.0),
            albedo: p.albedo, rough: p.rough,
        }).collect(),
        face: m.face, at: m.at, size: m.size, cells: m.cells,
    }).collect())
}

/// WHAT A RECORD SAYS OF ITS PAINT, ITS FINISH AND ITS WORDS: a box, a shape's piece and a loose thing's part say it in the same six members (a part has no finish of its own)
pub struct Said<'a> { pub material: Option<&'a str>, pub albedo: Option<V3>, pub rough: Option<f64>, pub metal: Option<bool>, pub stuff: Option<&'a str>, pub does: Option<&'a str> }
impl<'a> From<&'a BoxData> for Said<'a> {
    fn from(b: &'a BoxData) -> Self { Said { material: b.material.as_deref(), albedo: b.albedo, rough: b.rough, metal: b.metal, stuff: b.stuff.as_deref(), does: b.does.as_deref() } }
}
impl<'a> From<&'a ShapePieceData> for Said<'a> {
    fn from(w: &'a ShapePieceData) -> Self { Said { material: w.material.as_deref(), albedo: w.albedo, rough: w.rough, metal: w.metal, stuff: w.stuff.as_deref(), does: w.does.as_deref() } }
}
impl<'a> From<&'a Part> for Said<'a> {
    fn from(q: &'a Part) -> Self { Said { material: q.material.as_deref(), albedo: q.albedo, rough: None, metal: None, stuff: q.stuff.as_deref(), does: q.does.as_deref() } }
}
impl Said<'_> {
    pub fn painted(&self) -> bool { self.material.is_some() || self.albedo.is_some() }
}

/// the file's paint palette, and the readers of a record's paint, finish and words: one reader for a box, a loose thing's part and a shape's piece
struct Paint<'a> { name: &'a str, mats: &'a JsMap<String, Material> }
impl<'a> Paint<'a> {
    fn row(&self, o: &Said) -> Option<&'a Material> {
        let m = o.material?;
        Some(self.mats.of(m).unwrap_or_else(|| panic!("level '{}': unknown material '{}'", self.name, m)))
    }
    fn albedo(&self, o: &Said) -> V3 { self.row(o).map(|m| m.albedo).or(o.albedo).unwrap_or_else(|| panic!("level '{}': a box says no paint", self.name)) }
    fn rough(&self, o: &Said) -> Option<f64> { o.rough.or_else(|| self.row(o)?.rough) }
    /// (`metal` is only ever carried as true: false is the default and stays unwritten)
    fn metal(&self, o: &Said) -> Option<bool> { o.metal.or_else(|| self.row(o)?.metal).filter(|&m| m) }
    /// what it is made of: its own word, else its material row's; nothing for plain
    fn stuff(&self, o: &Said) -> Option<BoxStuff> {
        let w = o.stuff.or_else(|| self.row(o)?.stuff.as_deref()).filter(|w| *w != "plain")?;
        Some(stuff_word(w).unwrap_or_else(|| panic!("level '{}': '{}' is nothing a box is made of", self.name, w)))
    }
    /// what it does: its own word, else its material row's; nothing for the default row
    fn does(&self, o: &Said) -> Option<DoesWord> {
        let w = o.does.or_else(|| self.row(o)?.does.as_deref()).filter(|w| *w != "unnamed")?;
        Some(does_word(w).unwrap_or_else(|| panic!("level '{}': '{}' is no row of the table of materials", self.name, w)))
    }
}

/// A BOX'S SHAPE, carried on the box as authored, its planes copied: a round of 0 is none and `old` is said only when true; a
/// piece's paint, finish and word are read as a box's are, and a piece that says no paint is left with none (it wears its box's: the shape pass)
fn shape_of(s: ShapeData, paint: &Paint) -> game::scene_shapes::BoxShape {
    let (round_of, old_of) = (|r: Option<f64>| r.filter(|&r| r != 0.0), |o: Option<bool>| o.filter(|&o| o));
    let with = s.with.map(|ws| ws.into_iter().map(|w| {
        let said = Said::from(&w);
        let painted = said.painted();
        game::scene_shapes::ShapePiece {
            off: w.off, h: w.h, yaw: w.yaw.filter(|&y| y != 0.0),
            albedo: if painted { Some(paint.albedo(&said)) } else { None }, rough: if painted { paint.rough(&said) } else { None }, metal: if painted { paint.metal(&said) } else { None },
            stuff: if painted { paint.stuff(&said) } else { None }, does: if painted { paint.does(&said) } else { None },
            round: round_of(w.round), arc: w.arc, cut: w.cut, old: old_of(w.old),
        }
    }).collect());
    game::scene_shapes::BoxShape { with, round: round_of(s.round), arc: s.arc, cut: s.cut, old: old_of(s.old) }
}

/// a loose thing's half size in plan and its height, fitted from its parts
fn fit_prop_footprint(parts: &[PropPart]) -> ([f64; 2], f64) {
    use game::m;
    let (mut x0, mut x1, mut z0, mut z1, mut height) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY, 0.1);
    for q in parts {
        let (cs, sn) = (m::abs(m::cos(q.yaw)), m::abs(m::sin(q.yaw)));
        let (ex, ez) = (cs * q.h[0] + sn * q.h[2], sn * q.h[0] + cs * q.h[2]);
        x0 = m::min(x0, q.off[0] - ex); x1 = m::max(x1, q.off[0] + ex); z0 = m::min(z0, q.off[2] - ez); z1 = m::max(z1, q.off[2] + ez); height = m::max(height, q.off[1] + q.h[1]);
    }
    ([m::max(0.05, (x1 - x0) / 2.0), m::max(0.05, (z1 - z0) / 2.0)], height)
}

/// What a file's reading made, beside `g.level` itself.
pub struct Loaded {
    /// the file's name
    pub name: String,
    /// the level's boxes as rows, in the file's order
    pub boxes: Vec<BoxId>,
}

/// THE LEVEL FILE `data` INTO `g`: its boxes and routes as rows, `g.level`, the level's name and its script. Refuses a file it cannot read whole.
pub fn load(g: &mut Game, data: &Json) -> Loaded {
    let (read, faults) = file::read(data);
    match read {
        Some(f) => lay(g, f),
        None => panic!("level '{}': {} faults in the file:\n  {}", data.get("name").str().unwrap_or("?"), faults.len(), faults.join("\n  ")),
    }
}

fn lay(g: &mut Game, f: LevelFile) -> Loaded {
    let name = f.name;
    let paint = Paint { name: &name, mats: &f.materials };
    // boxes, in file order (index i of the file IS static box i)
    let mut ids: Vec<String> = Vec::new();
    let mut boxes: Vec<BoxId> = Vec::new();
    for b in f.boxes {
        let said = Said::from(&b);
        let mut p = MakeBoxP { c: b.c, h: b.h, ..blank_box() };
        p.yaw = Some(b.yaw.unwrap_or(0.0));
        p.albedo = Some(paint.albedo(&said));
        p.emissive = Some(b.emissive.unwrap_or([0.0, 0.0, 0.0]));
        p.flags = Some(flags_to_bits(b.flags.as_deref().unwrap_or(&[])));
        p.rough = paint.rough(&said); p.metal = paint.metal(&said); p.stuff = paint.stuff(&said); p.does = paint.does(&said);
        p.rot = b.rot; p.round = b.round.filter(|&r| r); p.cut_height = b.cut_height;
        p.marks = laid_markings(b.markings);
        p.shape = b.shape.map(|s| shape_of(s, &paint));
        ids.push(b.id);
        boxes.push(game::scene_boxes::make_box(g, &p));
    }
    let bx = |id: &str| -> f64 { ids.iter().position(|x| x == id).unwrap_or_else(|| panic!("level '{}': unknown box '{}'", name, id)) as f64 };
    // loose props
    let props: Vec<Arc<PropDef>> = f.props.into_iter().map(|p| {
        let parts: Vec<PropPart> = p.parts.into_iter().map(|q| {
            let said = Said::from(&q);
            PropPart { off: q.off, h: q.h, yaw: q.yaw.unwrap_or(0.0), albedo: paint.albedo(&said), stuff: paint.stuff(&said), does: paint.does(&said), marks: laid_markings(q.markings) }
        }).collect();
        let (half, height) = fit_prop_footprint(&parts);
        Arc::new(PropDef {
            kind: p.kind, name: p.name.unwrap_or(p.id), x: p.x, z: p.z, yaw: p.yaw.unwrap_or(0.0), half: p.half.unwrap_or(half), height: p.height.unwrap_or(height), mass: p.mass, parts,
        })
    }).collect();
    // lights
    let light_of = |l: LightData| LightDef {
        kind: match l.kind { LightDataKind::Spot => LightDefKind::Spot, LightDataKind::Dir => LightDefKind::Dir, LightDataKind::Point => LightDefKind::Point },
        pos: l.pos, dir: l.dir, color: l.color, intensity: l.intensity, range: l.range, inner_deg: l.inner_deg, outer_deg: l.outer_deg, name: l.id, group: l.group,
        enabled: l.enabled, volumetric: l.volumetric, fixture_box: l.fixture.as_deref().map(bx), radius: l.radius, mains: l.mains, broad: l.broad,
    };
    let lights: Vec<LightDef> = f.lights.into_iter().map(light_of).collect();
    let switchables: Vec<LevelSwitchablesItem> = f.switchables.into_iter().map(|s| LevelSwitchablesItem {
        name: s.id, boxes: s.boxes.iter().map(|b| bx(b)).collect(), group: s.group, on: s.on, base_emissive: s.base_emissive, flicker: s.flicker, mains: s.mains,
    }).collect();
    let doors: Vec<LevelDoorDef> = f.doors.into_iter().map(|d| LevelDoorDef {
        name: d.id, hinge: d.hinge, closed_dir: d.closed_dir, width: d.width, angle: d.angle, exterior: d.exterior, closer: d.closer, min_angle: d.min_angle, max_angle: d.max_angle,
        locked: d.locked, lock: d.lock, key_side: d.key_side, steel: d.steel, swing: d.swing,
    }).collect();
    let routes: Vec<RouteId> = f.routes.into_iter().map(|r| g.tab.route.push(PatrolRoute { name: r.name.unwrap_or(r.id), points: r.points, wait: r.wait, weapon: r.weapon, keys: r.keys })).collect();
    let chokepoints: Vec<Chokepoint> = f.chokepoints.into_iter().map(|c| Chokepoint { name: c.name.unwrap_or(c.id), pos: c.pos, yaw: c.yaw }).collect();
    let mut level = Level::default();
    level.props = props; level.lights = lights; level.switchables = switchables; level.doors = doors;
    level.devices = Some(f.devices); level.vents = f.vents; level.hums = f.hums; level.civilians = f.civilians; level.breach = f.plans.breach.map(|p| p.plan);
    level.breaker = LevelBreaker { pos: f.breaker.pos, boxes: vec![bx(&f.breaker.panel), bx(&f.breaker.led)], fumble: f.breaker.fumble };
    level.beacons = f.beacons.iter().map(|b| LevelBeaconsItem { pos: b.pos, box_: bx(&b.box_) }).collect();
    level.routes = routes; level.chokepoints = Some(chokepoints); level.clear_plans = Some(f.plans.room_clear.into_iter().map(|p| p.plan).collect());
    level.volumes = Some(f.volumes).filter(|v| !v.is_empty());
    level.marks = f.marks.filter(|m| !m.is_empty());
    level.sky = LevelSky { zenith: f.sky.zenith, horizon: f.sky.horizon };
    let m = f.mission;
    level.player_spawn = m.player_spawn;
    let (said, x) = (|name: Option<String>, else_: &str| name.unwrap_or_else(|| String::from(else_)), m.exfil);
    level.errand = game::mission::Errand {
        place: said(m.reach.name, "the marked room"), area: m.reach.area, door: m.reach.door,
        thing: said(m.drive.name, "what's there"), holder: said(m.drive.holder, "the marked place"), index: m.drive.rack_index, front: m.drive.front, half_w: m.drive.half_w, told: true,
        exits: vec![game::scene_scenarios::QuietExit { name: said(x.name, "the way out"), door: x.door, x: x.x, z: x.z, out: x.out.unwrap_or([1.0, 0.0]), half_w: None }],
    };
    level.mark_texel = f.meta.mark_texel;
    level.kept = f.meta.kept;
    level.snow = f.snow == Some(true);
    level.door_paint = f.door_paint.map(|p| {
        let side = |x: LeafPaintData| LeafPaint { albedo: x.albedo, stuff: paint.stuff(&Said { material: None, albedo: None, rough: None, metal: None, stuff: x.stuff.as_deref(), does: None }) };
        DoorPaint { outside: side(p.outside), inside: side(p.inside) }
    });
    // the scenarios' block and the footsteps' map: the runtime's own types, as they are written
    level.scenarios = f.scenarios.map(|s| s.kept); level.surfaces = f.surfaces;
    level.script = f.script;
    g.level = Arc::new(level);
    g.m.scene_level_load.level_names = Some(name.clone());
    Loaded { name, boxes }
}

/// `makeBox`'s argument with nothing said but a place and a size
pub fn blank_box() -> MakeBoxP {
    MakeBoxP { c: [0.0; 3], h: [0.0; 3], yaw: None, albedo: None, emissive: None, flags: None, owner: None, cut_height: None, wet_k: None, rot: None, dc: None, dh: None, rough: None, metal: None, stuff: None, does: None,
        aniso: None, marks: None, mark_gain: None, fog_emit: None, fog_mark: None, mkey: None, cut: None, cut_core: None, cut_glass: None, cut_round: None, cut_arc: None, cut_old: None, cut_core_stuff: None, round: None,
        shape: None, shaped: None, piece: None, of: None, dress: None }
}
