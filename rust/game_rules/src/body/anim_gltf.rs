pub use ::game_types::anim_gltf::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use alloc::vec::Vec;

/// `(1 << i) >>> 0`: the shift counts modulo 32, as the language's does.
fn part_bit(i: usize) -> f64 {
    1i32.wrapping_shl(i as u32) as u32 as f64
}

/// A part's bit by name, 0 where the cast has no such part.
fn named_bit(g: &Game, cast: AnimGltfCastInfoId, n: &str) -> f64 {
    match g.art.cast[cast].parts.iter().position(|p| p == n) {
        Some(i) => part_bit(i),
        None => 0.0,
    }
}

/// THE GOGGLES RULE: `boxes` = the rig's own box goggles are drawn, so neither mesh pair; else the mesh pair flipped up on the mount.
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

/// THE SNEAKING SUIT's mask: a variant's parts less the helmet, every `kit_*` and the mesh goggles.
pub fn cast_suit(g: &mut crate::game::Game, cast: AnimGltfCastInfoId, mask: f64) -> f64 {
    let b = match g.m.anim_gltf.suit_bits.get(cast).copied() {
        Some(b) => b,
        None => {
            let mut b = 0.0;
            for (i, n) in g.art.cast[cast].parts.iter().enumerate() {
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

/// The variants of the cast worn by `side`, in the cast's order.
fn variants_of(g: &Game, cast: AnimGltfCastInfoId, side: CastSide) -> Vec<AnimGltfCastVariantId> {
    let mut v = Vec::new();
    for &x in g.art.cast[cast].variants.iter() {
        if g.art.variant[x].side == side {
            v.push(x);
        }
    }
    v
}

/// The variants of one side; a side with none falls back to all of them, the responders to the defenders first.
pub fn cast_side(g: &crate::game::Game, cast: AnimGltfCastInfoId, side: crate::anim_gltf::CastSide) -> Vec<AnimGltfCastVariantId> {
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
    g.art.cast[cast].variants.clone()
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::string::String;
    use alloc::vec;
    use super::*;

    fn make_cast(g: &mut Game, parts: &[&str]) -> AnimGltfCastInfoId {
        let mut c = CastInfo::default();
        c.parts = parts.iter().map(|p| String::from(*p)).collect();
        alloc::sync::Arc::make_mut(&mut g.art).cast.push(c)
    }

    fn make_variant(g: &mut Game, cast: AnimGltfCastInfoId, name: &str, side: CastSide) -> AnimGltfCastVariantId {
        let mut v = CastVariant::default();
        v.name = String::from(name);
        v.side = side;
        let id = alloc::sync::Arc::make_mut(&mut g.art).variant.push(v);
        alloc::sync::Arc::make_mut(&mut g.art).cast[cast].variants.push(id);
        id
    }

    /// A cast with both pairs of mesh goggles, a helmet and two bits of kit. Each part's bit is its place's power of two.
    const PARTS: &[&str] = &["shirt", "nvg_up", "helmet", "kit_radio", "nvg_down", "vest", "kit_ram"];
    const SHIRT: f64 = 1.0;
    const UP: f64 = 2.0;
    const DOWN: f64 = 16.0;

    #[test]
    fn under_the_box_goggles_he_wears_no_mesh_pair_and_without_them_the_pair_goes_up_on_the_mount() {
        let mut g = Game::blank();
        let cast = make_cast(&mut g, PARTS);
        for boxes in [true, false] { assert_eq!(cast_goggles(&mut g, cast, SHIRT, boxes), SHIRT, "a man with no goggles stays as he is"); }
        for pair in [UP, DOWN, UP + DOWN] {
            assert_eq!(cast_goggles(&mut g, cast, SHIRT + pair, true), SHIRT);
            assert_eq!(cast_goggles(&mut g, cast, SHIRT + pair, false), SHIRT + UP);
        }
        // a cast with no raised pair keeps the one it has, and a cast with no goggles has nothing to change
        let only_down = make_cast(&mut g, &["a", "nvg_down"]);
        assert_eq!(cast_goggles(&mut g, only_down, 3.0, false), 3.0);
        assert_eq!(cast_goggles(&mut g, only_down, 3.0, true), 1.0);
        let none = make_cast(&mut g, &["a", "b"]);
        assert_eq!(cast_goggles(&mut g, none, 3.0, true), 3.0);
    }

    #[test]
    fn the_sneaking_suit_is_his_clothes_with_no_helmet_no_kit_and_no_goggles() {
        let mut g = Game::blank();
        let cast = make_cast(&mut g, PARTS);
        assert_eq!(cast_suit(&mut g, cast, 127.0), SHIRT + 32.0);
    }

    #[test]
    fn the_last_part_a_mask_can_hold_is_a_part_like_any_other() {
        let mut g = Game::blank();
        let parts: Vec<String> = (0..32).map(|i| if i == 31 { String::from("nvg_up") } else { format!("p{}", i) }).collect();
        let parts: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
        let cast = make_cast(&mut g, &parts);
        let top = 2147483648.0;
        assert_eq!(cast_goggles(&mut g, cast, 1.0 + top, false), 1.0 + top, "a high bit reads as a whole number, never as a negative one");
        assert_eq!(cast_goggles(&mut g, cast, 1.0 + top, true), 1.0);
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
