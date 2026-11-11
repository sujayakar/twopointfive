// Written by the program `half` (rust/game_start/src/bin/half) from rust/game_draw/src/gpu.rs: THE BOXES' MATERIAL BUFFER (`boxMat`, rows of four floats), where each of its tables begins, and a reader a field of those
// Rust lays out. To change one, change it there.
const MAT_BASE: u32 = 0u;
const MARK_BASE: u32 = 16384u;
const PANE_BASE: u32 = 22528u;
const AIR_BASE: u32 = 23680u;
const CUT_BASE: u32 = 24800u;

// its material: 4 rows a box
const MAT_STRIDE: u32 = 4u;
fn boxAlbedo(i: u32) -> vec3f { return boxMat[MAT_BASE + i * 4u + 0u].xyz; }
fn boxCutHeight(i: u32) -> f32 { return boxMat[MAT_BASE + i * 4u + 0u].w; }   // above this height it is hidden from the eye, where it is cut away
fn boxEmits(i: u32) -> vec3f { return boxMat[MAT_BASE + i * 4u + 1u].xyz; }
fn boxFinishWord(i: u32) -> f32 { return boxMat[MAT_BASE + i * 4u + 1u].w; }   // its roughness to 1, 2 for a metal, 4 for an anisotropic one, and 8 of its look's word (what it is made of, whether it moves, a number of its own)
fn boxRot(i: u32) -> vec4f { return boxMat[MAT_BASE + i * 4u + 2u]; }
fn boxAlbedoSec(i: u32) -> vec3f { return boxMat[MAT_BASE + i * 4u + 3u].xyz; }   // the colour a ray that does not see its marks finds
fn boxMarkGain(i: u32) -> f32 { return boxMat[MAT_BASE + i * 4u + 3u].w; }   // what its marks' light is multiplied by

// its planes, where it is cut: each n and d, box-local, the side kept is n p <= d (a normal of nought ends them): 9 rows a box
const CUT_STRIDE: u32 = 9u;
const CUT_PLANES: u32 = 8u;
fn cutPlane(i: u32, k: u32) -> vec4f { return boxMat[CUT_BASE + i * 9u + 0u + k]; }
fn cutCore(i: u32) -> vec4f { return boxMat[CUT_BASE + i * 9u + 8u]; }   // the colour its cut faces show (w over 0: it has one), and what else is said of them in w

// what is broken of a listed pane (all nought: it is whole): 36 rows a box
const PANE_STRIDE: u32 = 36u;
fn paneSites(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 0u].x; }   // how many sites it has been cut from
fn paneSays(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 0u].y; }   // 1 something is open, 2 it has run, 4 it lets go in clumps, 8 it is gone, 16 its breaks are a web
fn paneAxis(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 0u].z; }   // 0: its own x runs across it; 1: its z does
fn paneDamage(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 0u].w; }   // how far it has gone, to 1
fn paneHalf(i: u32) -> vec2f { return boxMat[PANE_BASE + i * 36u + 1u].xy; }
fn paneKind(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 1u].z; }
fn panePad(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 1u].w; }
fn paneBlow(i: u32) -> vec2f { return boxMat[PANE_BASE + i * 36u + 2u].xy; }   // where the blow that diced it fell
fn paneOpenAt(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 2u].z; }   // the clumps whose key is under this have let go
fn paneGrain(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 2u].w; }
fn paneCols(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 3u].x; }
fn paneRows(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 3u].y; }
fn paneSeed(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 3u].z; }
fn panePad2(i: u32) -> f32 { return boxMat[PANE_BASE + i * 36u + 3u].w; }
const PANE_SITES: u32 = 32u;
fn paneSite(i: u32, k: u32) -> vec4f { return boxMat[PANE_BASE + i * 36u + 4u + k]; }   // a site: its place, its turn and its ring's radius · its radials, seed, energy and how much is merged · the four rings' radii · the pieces that have fallen out, twelve bits a band, two bands a float

// a mark's head; its cells follow, twelve to a float: 8 rows from `row` of the table
const MARK_HEAD: u32 = 8u;
fn markPlace(row: u32) -> vec4f { return boxMat[MARK_BASE + row + 0u]; }   // where it lies on the face and how many cells it is across and up
fn markSays(row: u32) -> f32 { return boxMat[MARK_BASE + row + 1u].x; }   // its face in the low 3 bits, its cells' size in 2, then: the box's last mark, some of the list emits, its light is an area's, which entries emit (3 bits), which have a colour and which a roughness (3 and 3)
fn markShares(row: u32) -> vec3f { return boxMat[MARK_BASE + row + 1u].yzw; }   // how much of it each of its three entries covers
const MARK_PALS: u32 = 6u;
fn markPal(row: u32, k: u32) -> vec4f { return boxMat[MARK_BASE + row + 2u + k]; }   // an entry's colour and roughness (under 0: the box's), then what it emits
