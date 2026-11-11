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

// a mark's head; its cells follow, twelve to a float: 8 rows from `row` of the table
const MARK_HEAD: u32 = 8u;
fn markPlace(row: u32) -> vec4f { return boxMat[MARK_BASE + row + 0u]; }   // where it lies on the face and how many cells it is across and up
fn markSays(row: u32) -> f32 { return boxMat[MARK_BASE + row + 1u].x; }   // its face in the low 3 bits, its cells' size in 2, then: the box's last mark, some of the list emits, its light is an area's, which entries emit (3 bits), which have a colour and which a roughness (3 and 3)
fn markShares(row: u32) -> vec3f { return boxMat[MARK_BASE + row + 1u].yzw; }   // how much of it each of its three entries covers
const MARK_PALS: u32 = 6u;
fn markPal(row: u32, k: u32) -> vec4f { return boxMat[MARK_BASE + row + 2u + k]; }   // an entry's colour and roughness (under 0: the box's), then what it emits
