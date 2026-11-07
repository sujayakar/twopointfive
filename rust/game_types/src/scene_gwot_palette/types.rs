//! The types of `scene_gwot_palette`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/scene/gwotPalette.ts:14 GWOT_PALETTE, src/scene/gwotPalette.ts:14)
#[derive(Clone, Copy, Debug)]
pub struct GWOTPALETTE {
    pub mud: V3,
    pub mud_dark: V3,
    pub mud_light: V3,
    pub render: V3,
    pub whitewash: V3,
    pub block: V3,
    pub t_wall: V3,
    pub paint_blue: V3,
    pub paint_green: V3,
    pub paint_rust: V3,
    pub paint_cream: V3,
    pub dust: V3,
    pub dirt: V3,
    pub gravel: V3,
    pub mud_floor: V3,
    pub slab: V3,
    pub damp: V3,
    pub oil: V3,
    pub rug_red: V3,
    pub rug_navy: V3,
    pub rug_cream: V3,
    pub rug_ochre: V3,
    pub rug_green: V3,
    pub canvas: V3,
    pub blanket_grey: V3,
    pub blanket_brown: V3,
    pub tarp_blue: V3,
    pub sack: V3,
    pub sandbag: V3,
    pub hesco: V3,
    pub olive: V3,
    pub coyote: V3,
    pub stencil: V3,
    pub rust: V3,
    pub iron: V3,
    pub galv: V3,
    pub tin: V3,
    pub drum_blue: V3,
    pub drum_red: V3,
    pub gen_yellow: V3,
    pub plastic_white: V3,
    pub plastic_green: V3,
    pub plastic_blue: V3,
    pub plastic_black: V3,
    pub jerry_yellow: V3,
    pub rubber: V3,
    pub poplar: V3,
    pub wood_grey: V3,
    pub ply: V3,
    pub truck_white: V3,
    pub truck_tan: V3,
    pub truck_red: V3,
    pub glass_dark: V3,
    pub prod_orange: V3,
    pub prod_green: V3,
    pub prod_red: V3,
    pub prod_yellow: V3,
    pub straw: V3,
    pub scrub: V3,
    pub rug_madder: V3,
}
impl GWOTPALETTE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GWOTPALETTE {
            mud: [0.0; 3],
            mud_dark: [0.0; 3],
            mud_light: [0.0; 3],
            render: [0.0; 3],
            whitewash: [0.0; 3],
            block: [0.0; 3],
            t_wall: [0.0; 3],
            paint_blue: [0.0; 3],
            paint_green: [0.0; 3],
            paint_rust: [0.0; 3],
            paint_cream: [0.0; 3],
            dust: [0.0; 3],
            dirt: [0.0; 3],
            gravel: [0.0; 3],
            mud_floor: [0.0; 3],
            slab: [0.0; 3],
            damp: [0.0; 3],
            oil: [0.0; 3],
            rug_red: [0.0; 3],
            rug_navy: [0.0; 3],
            rug_cream: [0.0; 3],
            rug_ochre: [0.0; 3],
            rug_green: [0.0; 3],
            canvas: [0.0; 3],
            blanket_grey: [0.0; 3],
            blanket_brown: [0.0; 3],
            tarp_blue: [0.0; 3],
            sack: [0.0; 3],
            sandbag: [0.0; 3],
            hesco: [0.0; 3],
            olive: [0.0; 3],
            coyote: [0.0; 3],
            stencil: [0.0; 3],
            rust: [0.0; 3],
            iron: [0.0; 3],
            galv: [0.0; 3],
            tin: [0.0; 3],
            drum_blue: [0.0; 3],
            drum_red: [0.0; 3],
            gen_yellow: [0.0; 3],
            plastic_white: [0.0; 3],
            plastic_green: [0.0; 3],
            plastic_blue: [0.0; 3],
            plastic_black: [0.0; 3],
            jerry_yellow: [0.0; 3],
            rubber: [0.0; 3],
            poplar: [0.0; 3],
            wood_grey: [0.0; 3],
            ply: [0.0; 3],
            truck_white: [0.0; 3],
            truck_tan: [0.0; 3],
            truck_red: [0.0; 3],
            glass_dark: [0.0; 3],
            prod_orange: [0.0; 3],
            prod_green: [0.0; 3],
            prod_red: [0.0; 3],
            prod_yellow: [0.0; 3],
            straw: [0.0; 3],
            scrub: [0.0; 3],
            rug_madder: [0.0; 3],
        }
    }
}
impl Default for GWOTPALETTE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/gwotPalette.ts:69 SNOW_PALETTE, src/scene/gwotPalette.ts:69)
#[derive(Clone, Copy, Debug)]
pub struct SNOWPALETTE {
    pub snow: V3,
    pub snow_packed: V3,
    pub snow_old: V3,
}
impl SNOWPALETTE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWPALETTE {
            snow: [0.0; 3],
            snow_packed: [0.0; 3],
            snow_old: [0.0; 3],
        }
    }
}
impl Default for SNOWPALETTE { fn default() -> Self { Self::zeroed() } }

/// (src/scene/gwotPalette.ts:14 GWOT_PALETTE: a table nothing writes)
pub fn gwot_palette() -> crate::scene_gwot_palette::GWOTPALETTE { crate::scene_gwot_palette::GWOTPALETTE { mud: [0.42, 0.32, 0.21], mud_dark: [0.29, 0.22, 0.15], mud_light: [0.56, 0.46, 0.33], render: [0.57, 0.53, 0.45], whitewash: [0.72, 0.69, 0.61], block: [0.4, 0.39, 0.37], t_wall: [0.46, 0.45, 0.43], paint_blue: [0.06, 0.19, 0.32], paint_green: [0.08, 0.24, 0.16], paint_rust: [0.3, 0.075, 0.04], paint_cream: [0.66, 0.6, 0.44], dust: [0.205, 0.145, 0.085], dirt: [0.135, 0.09, 0.05], gravel: [0.15, 0.135, 0.115], mud_floor: [0.17, 0.115, 0.07], slab: [0.3, 0.285, 0.26], damp: [0.075, 0.052, 0.034], oil: [0.11, 0.095, 0.08], rug_red: [0.3, 0.052, 0.04], rug_navy: [0.04, 0.06, 0.15], rug_cream: [0.6, 0.52, 0.38], rug_ochre: [0.5, 0.33, 0.1], rug_green: [0.09, 0.22, 0.13], canvas: [0.33, 0.31, 0.2], blanket_grey: [0.3, 0.3, 0.31], blanket_brown: [0.27, 0.18, 0.12], tarp_blue: [0.05, 0.17, 0.48], sack: [0.62, 0.58, 0.48], sandbag: [0.45, 0.39, 0.27], hesco: [0.47, 0.44, 0.36], olive: [0.15, 0.17, 0.09], coyote: [0.4, 0.31, 0.2], stencil: [0.62, 0.5, 0.1], rust: [0.28, 0.12, 0.06], iron: [0.098, 0.1, 0.112], galv: [0.5, 0.52, 0.53], tin: [0.56, 0.55, 0.52], drum_blue: [0.06, 0.15, 0.33], drum_red: [0.4, 0.07, 0.05], gen_yellow: [0.6, 0.42, 0.04], plastic_white: [0.7, 0.7, 0.66], plastic_green: [0.08, 0.28, 0.14], plastic_blue: [0.03, 0.12, 0.36], plastic_black: [0.045, 0.045, 0.05], jerry_yellow: [0.62, 0.46, 0.05], rubber: [0.03, 0.03, 0.033], poplar: [0.47, 0.36, 0.22], wood_grey: [0.33, 0.29, 0.23], ply: [0.55, 0.43, 0.27], truck_white: [0.68, 0.68, 0.65], truck_tan: [0.5, 0.42, 0.29], truck_red: [0.42, 0.06, 0.05], glass_dark: [0.05, 0.06, 0.08], prod_orange: [0.7, 0.3, 0.04], prod_green: [0.22, 0.4, 0.1], prod_red: [0.5, 0.07, 0.05], prod_yellow: [0.7, 0.55, 0.1], straw: [0.51, 0.43, 0.22], scrub: [0.24, 0.26, 0.13], rug_madder: [0.1, 0.03, 0.035] } }

/// (src/scene/gwotPalette.ts:69 SNOW_PALETTE: a table nothing writes)
pub fn snow_palette() -> crate::scene_gwot_palette::SNOWPALETTE { crate::scene_gwot_palette::SNOWPALETTE { snow: [0.66, 0.71, 0.78], snow_packed: [0.43, 0.47, 0.53], snow_old: [0.27, 0.3, 0.34] } }
