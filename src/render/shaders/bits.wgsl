// Written by the program `half` (rust/game_start/src/bin/half) from a declaration (`bits!`). To change a bit, change it there.

// a box's word as a shader reads it (`Geo.flags`): what of its own bits a shader asks, whose it is in the second byte, from bit 18 the first row (from 1) of its marks
const GEO_NO_PRIMARY: u32 = 1u;   // not rasterized
const GEO_NO_SHADOW: u32 = 2u;   // no ray meets it
const GEO_CUTAWAY: u32 = 4u;   // rasterized only below its cut height
const GEO_ROUND: u32 = 8u;   // the cylinder about its own y, radius h.x (to a program with the cut rule, the prism in its rows of the cut table). The box's own word has DYNAMIC here, which no shader asks
const GEO_EMISSIVE: u32 = 16u;   // has non-zero emission
const GEO_AREA_LIT: u32 = 32u;   // its emission is an analytic area light's
const GEO_ROT: u32 = 64u;   // fully rotated: its turn is the quaternion, not the packed yaw
const GEO_GLOSS: u32 = 128u;   // polished floor finish
const GEO_CUT: u32 = 65536u;   // clipped by the planes in its rows of the cut table. The box's own word has SOUND_SEAL here, which is the rules' alone
const GEO_GLASS: u32 = 131072u;   // a pane: never in the trace grid nor the raster

// what is so of a broken pane (`Pane.says`)
const GB_OPEN: u32 = 1u;   // something of it has fallen out
const GB_RUN: u32 = 2u;   // its cracks run to the frame
const GB_DICING: u32 = 4u;   // it lets go in clumps
const GB_GONE: u32 = 8u;   // all of it has fallen out
const GB_WEB: u32 = 16u;   // its breaks are a web (a laminated sheet)
