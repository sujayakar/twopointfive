//! src/anim/gltf.ts lines 1 to 237
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `(1 << i) >>> 0`: the shift counts modulo 32, as the language's does.
fn part_bit(i: usize) -> f64 {
    1i32.wrapping_shl(i as u32) as u32 as f64
}

/// A part's bit by name, 0 where the cast has no such part.   (gltf.ts castGoggles's `bit`)
fn named_bit(g: &Game, cast: AnimGltfCastInfoId, n: &str) -> f64 {
    match g.tab.anim_gltf_cast_info[cast].parts.iter().position(|p| p == n) {
        Some(i) => part_bit(i),
        None => 0.0,
    }
}

/// THE GOGGLES RULE: `boxes` = the rig's own box goggles are drawn, so neither mesh pair; else the mesh pair flipped up on the mount.   (gltf.ts castGoggles)
pub fn cast_goggles(g: &mut crate::game::Game, cast: AnimGltfCastInfoId, mask: f64, boxes: bool) -> f64 {
    let nb = match g.m.anim_gltf.nvg_bits.get(cast).copied() {
        Some(nb) => nb,
        None => {
            let nb = AnimGltfModNvgBitsValue { up: named_bit(g, cast, "nvg_up"), down: named_bit(g, cast, "nvg_down") };
            g.m.anim_gltf.nvg_bits.set(cast, nb);
            nb
        }
    };
    let (up, down) = (nb.up, nb.down);
    let pair = js::to_i32(up) | js::to_i32(down);
    if js::to_i32(mask) & pair == 0 {
        return js::to_u32(mask) as f64;
    }
    let bare = (js::to_i32(mask) & !pair) as u32 as f64;
    if boxes { bare } else { (js::to_i32(bare) | js::to_i32(js::or(up, down))) as u32 as f64 }
}

/// THE SNEAKING SUIT's mask: a variant's parts less the helmet, every `kit_*` and the mesh goggles.   (gltf.ts castSuit)
pub fn cast_suit(g: &mut crate::game::Game, cast: AnimGltfCastInfoId, mask: f64) -> f64 {
    let b = match g.m.anim_gltf.suit_bits.get(cast).copied() {
        Some(b) => b,
        None => {
            let mut b = 0.0;
            for (i, n) in g.tab.anim_gltf_cast_info[cast].parts.iter().enumerate() {
                if n == "helmet" || n.starts_with("kit_") {
                    b = (js::to_i32(b) | js::to_i32(part_bit(i))) as u32 as f64;
                }
            }
            g.m.anim_gltf.suit_bits.set(cast, b);
            b
        }
    };
    let worn = (js::to_i32(mask) & !js::to_i32(b)) as u32 as f64;
    cast_goggles(g, cast, worn, true)
}

/// The variants of the cast worn by `side`, in the cast's order.   (gltf.ts castSide's filter)
fn variants_of(g: &Game, cast: AnimGltfCastInfoId, side: CastSide) -> Vec<AnimGltfCastVariantId> {
    let mut v = Vec::new();
    for &x in g.tab.anim_gltf_cast_info[cast].variants.iter() {
        if g.tab.anim_gltf_cast_variant[x].side == side {
            v.push(x);
        }
    }
    v
}

/// The variants of one side; a side with none falls back to all of them, the responders to the defenders first.   (gltf.ts castSide)
pub fn cast_side(g: &mut crate::game::Game, cast: AnimGltfCastInfoId, side: crate::anim_gltf::CastSide) -> Vec<AnimGltfCastVariantId> {
    let v = variants_of(g, cast, side);
    if !v.is_empty() {
        return v;
    }
    if side == CastSide::Responder {
        let d = variants_of(g, cast, CastSide::Defender);
        if !d.is_empty() {
            return d;
        }
    }
    g.tab.anim_gltf_cast_info[cast].variants.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cast(g: &mut Game, parts: &[&str]) -> AnimGltfCastInfoId {
        let mut c = CastInfo::zeroed();
        c.parts = parts.iter().map(|p| String::from(*p)).collect();
        g.tab.anim_gltf_cast_info.push(c)
    }

    fn make_variant(g: &mut Game, cast: AnimGltfCastInfoId, name: &str, side: CastSide) -> AnimGltfCastVariantId {
        let mut v = CastVariant::zeroed();
        v.name = String::from(name);
        v.side = side;
        let id = g.tab.anim_gltf_cast_variant.push(v);
        g.tab.anim_gltf_cast_info[cast].variants.push(id);
        id
    }

    /// The TypeScript's own bits under bun: (cast, mask, castGoggles boxes, castGoggles no boxes, castSuit).
    const BITS: &[(&str, f64, u64, u64, u64)] = &[
        ("parts", 0.0, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000),
        ("parts", 1.0, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
        ("parts", 2.0, 0x0000000000000000, 0x4000000000000000, 0x0000000000000000),
        ("parts", 3.0, 0x3ff0000000000000, 0x4008000000000000, 0x3ff0000000000000),
        ("parts", 18.0, 0x0000000000000000, 0x4000000000000000, 0x0000000000000000),
        ("parts", 127.0, 0x405b400000000000, 0x405bc00000000000, 0x4040800000000000),
        ("parts", 4294967295.0, 0x41effffffda00000, 0x41effffffde00000, 0x41effffff4200000),
        ("parts", 2147483648.0, 0x41e0000000000000, 0x41e0000000000000, 0x41e0000000000000),
        ("parts", 2147483649.0, 0x41e0000000200000, 0x41e0000000200000, 0x41e0000000200000),
        ("parts", 3221225473.0, 0x41e8000000200000, 0x41e8000000200000, 0x41e8000000200000),
        ("parts", -1.0, 0x41effffffda00000, 0x41effffffde00000, 0x41effffff4200000),
        ("parts", 4294967298.0, 0x0000000000000000, 0x4000000000000000, 0x0000000000000000),
        ("up_only", 0.0, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000),
        ("up_only", 1.0, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000),
        ("up_only", 2.0, 0x4000000000000000, 0x4000000000000000, 0x4000000000000000),
        ("up_only", 3.0, 0x4000000000000000, 0x4008000000000000, 0x4000000000000000),
        ("up_only", 18.0, 0x4032000000000000, 0x4032000000000000, 0x4032000000000000),
        ("up_only", 127.0, 0x405f800000000000, 0x405fc00000000000, 0x405f800000000000),
        ("up_only", 4294967295.0, 0x41efffffffc00000, 0x41efffffffe00000, 0x41efffffffc00000),
        ("up_only", 2147483648.0, 0x41e0000000000000, 0x41e0000000000000, 0x41e0000000000000),
        ("up_only", 2147483649.0, 0x41e0000000000000, 0x41e0000000200000, 0x41e0000000000000),
        ("up_only", 3221225473.0, 0x41e8000000000000, 0x41e8000000200000, 0x41e8000000000000),
        ("up_only", -1.0, 0x41efffffffc00000, 0x41efffffffe00000, 0x41efffffffc00000),
        ("up_only", 4294967298.0, 0x4000000000000000, 0x4000000000000000, 0x4000000000000000),
        ("down_only", 0.0, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000),
        ("down_only", 1.0, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
        ("down_only", 2.0, 0x0000000000000000, 0x4000000000000000, 0x0000000000000000),
        ("down_only", 3.0, 0x3ff0000000000000, 0x4008000000000000, 0x3ff0000000000000),
        ("down_only", 18.0, 0x4030000000000000, 0x4032000000000000, 0x4030000000000000),
        ("down_only", 127.0, 0x405f400000000000, 0x405fc00000000000, 0x405f400000000000),
        ("down_only", 4294967295.0, 0x41efffffffa00000, 0x41efffffffe00000, 0x41efffffffa00000),
        ("down_only", 2147483648.0, 0x41e0000000000000, 0x41e0000000000000, 0x41e0000000000000),
        ("down_only", 2147483649.0, 0x41e0000000200000, 0x41e0000000200000, 0x41e0000000200000),
        ("down_only", 3221225473.0, 0x41e8000000200000, 0x41e8000000200000, 0x41e8000000200000),
        ("down_only", -1.0, 0x41efffffffa00000, 0x41efffffffe00000, 0x41efffffffa00000),
        ("down_only", 4294967298.0, 0x0000000000000000, 0x4000000000000000, 0x0000000000000000),
        ("none", 0.0, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000),
        ("none", 1.0, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
        ("none", 2.0, 0x4000000000000000, 0x4000000000000000, 0x4000000000000000),
        ("none", 3.0, 0x4008000000000000, 0x4008000000000000, 0x4008000000000000),
        ("none", 18.0, 0x4032000000000000, 0x4032000000000000, 0x4032000000000000),
        ("none", 127.0, 0x405fc00000000000, 0x405fc00000000000, 0x405fc00000000000),
        ("none", 4294967295.0, 0x41efffffffe00000, 0x41efffffffe00000, 0x41efffffffe00000),
        ("none", 2147483648.0, 0x41e0000000000000, 0x41e0000000000000, 0x41e0000000000000),
        ("none", 2147483649.0, 0x41e0000000200000, 0x41e0000000200000, 0x41e0000000200000),
        ("none", 3221225473.0, 0x41e8000000200000, 0x41e8000000200000, 0x41e8000000200000),
        ("none", -1.0, 0x41efffffffe00000, 0x41efffffffe00000, 0x41efffffffe00000),
        ("none", 4294967298.0, 0x4000000000000000, 0x4000000000000000, 0x4000000000000000),
        ("p32", 0.0, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000),
        ("p32", 1.0, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
        ("p32", 2.0, 0x4000000000000000, 0x4000000000000000, 0x4000000000000000),
        ("p32", 3.0, 0x4008000000000000, 0x4008000000000000, 0x4008000000000000),
        ("p32", 18.0, 0x4032000000000000, 0x4032000000000000, 0x4032000000000000),
        ("p32", 127.0, 0x405fc00000000000, 0x405fc00000000000, 0x405fc00000000000),
        ("p32", 4294967295.0, 0x41dfffffffc00000, 0x41efffffffe00000, 0x41bfffffff000000),
        ("p32", 2147483648.0, 0x0000000000000000, 0x41e0000000000000, 0x0000000000000000),
        ("p32", 2147483649.0, 0x3ff0000000000000, 0x41e0000000200000, 0x3ff0000000000000),
        ("p32", 3221225473.0, 0x41d0000000400000, 0x41e8000000200000, 0x3ff0000000000000),
        ("p32", -1.0, 0x41dfffffffc00000, 0x41efffffffe00000, 0x41bfffffff000000),
        ("p32", 4294967298.0, 0x4000000000000000, 0x4000000000000000, 0x4000000000000000),
    ];

    #[test]
    fn masks_match_the_typescripts_bits() {
        let mut g = Game::blank();
        let p32: Vec<String> = (0..32)
            .map(|i| match i {
                31 => String::from("nvg_up"),
                30 => String::from("helmet"),
                29 => String::from("kit_x"),
                _ => format!("p{}", i),
            })
            .collect();
        let p32: Vec<&str> = p32.iter().map(|s| s.as_str()).collect();
        let casts = [
            ("parts", make_cast(&mut g, &["shirt", "nvg_up", "helmet", "kit_radio", "nvg_down", "vest", "kit_ram"])),
            ("up_only", make_cast(&mut g, &["nvg_up", "a"])),
            ("down_only", make_cast(&mut g, &["a", "nvg_down"])),
            ("none", make_cast(&mut g, &["a", "b"])),
            ("p32", make_cast(&mut g, &p32)),
        ];
        assert_eq!(BITS.len(), 60);
        for &(name, mask, boxes, no_boxes, suit) in BITS {
            let cast = casts.iter().find(|c| c.0 == name).unwrap().1;
            assert_eq!(cast_goggles(&mut g, cast, mask, true).to_bits(), boxes, "{} {} boxes", name, mask);
            assert_eq!(cast_goggles(&mut g, cast, mask, false).to_bits(), no_boxes, "{} {} no boxes", name, mask);
            assert_eq!(cast_suit(&mut g, cast, mask).to_bits(), suit, "{} {} suit", name, mask);
        }
    }

    #[test]
    fn a_side_with_no_variants_falls_back() {
        let mut g = Game::blank();
        let cast = make_cast(&mut g, &["a"]);
        let op = make_variant(&mut g, cast, "operator_1", CastSide::Operator);
        let civ = make_variant(&mut g, cast, "civilian_1", CastSide::Civilian);
        assert_eq!(cast_side(&mut g, cast, CastSide::Operator), vec![op]);
        // no defenders in the file: everybody, in order
        assert_eq!(cast_side(&mut g, cast, CastSide::Defender), vec![op, civ]);
        assert_eq!(cast_side(&mut g, cast, CastSide::Responder), vec![op, civ]);
        // the responders dress as the staff once there are any
        let def = make_variant(&mut g, cast, "defender_1", CastSide::Defender);
        assert_eq!(cast_side(&mut g, cast, CastSide::Responder), vec![def]);
        let resp = make_variant(&mut g, cast, "responder_1", CastSide::Responder);
        assert_eq!(cast_side(&mut g, cast, CastSide::Responder), vec![resp]);
    }
}
