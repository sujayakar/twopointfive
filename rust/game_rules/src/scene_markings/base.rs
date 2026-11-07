//! What the parts of `scene_markings` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// INVENTED: AREA_LIT_MARKS asks by the object (markings.ts:369), and in Rust a mark is a value that boxes and lists copy, so its content says which. Picture only
pub fn mark_key(m: &crate::scene_markings::MarkingDef) -> u32 { let mut k = 0x811c9dc5u32; let mut eat = |b: &[u8]| { for x in b { k = (k ^ *x as u32).wrapping_mul(0x01000193); } }; eat(m.face.as_str().as_bytes()); for v in m.at.iter().chain(m.size.iter()) { eat(&v.to_bits().to_le_bytes()); } for c in &m.cells { eat(c.as_bytes()); } k }
