//! src/scene/world.ts lines 1 to 49
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A number as WGSL wants a float: a whole one keeps a `.0`.   (world.ts wgslWorldConsts, its `f`)
fn f(n: f64) -> String {
    if n.is_finite() && m::floor(n) == n { format!("{:.1}", n) } else { format!("{}", n) }
}

/// Three numbers through `f`, joined by commas.   (world.ts wgslWorldConsts, `.map(f).join(', ')`)
fn f3(v: [f64; 3]) -> String {
    format!("{}, {}, {}", f(v[0]), f(v[1]), f(v[2]))
}

/// The world's constants as WGSL source, for the shaders.   (world.ts wgslWorldConsts)
pub fn wgsl_world_consts() -> String {
    let w = world();
    format!(
        "
const WORLD_MIN = vec3f({});
const WORLD_MAX = vec3f({});
const GRID_CELL: f32 = {};
const GRID_W: i32 = {};
const GRID_H: i32 = {};
const GRID_CB: u32 = {}u;
const GRID_YQ0: f32 = {};
const GRID_YQS: f32 = {};
const GRID_YQMAX: f32 = {};
const CEIL_Y: f32 = {};
const MAX_BOXES: u32 = {}u;   // boxes.ts BoxWorld.maxBoxes: the marking table follows the mat rows of this many boxes (common.wgsl MARK_BASE)
const REACH_ROW_WORDS: u32 = {}u;   // per-light reach mask row (render/reach.ts: 2 bits a cell)
const RC_MIN = vec3f({});
const RC_SIZE = vec3f({});
",
        f3(w.min),
        f3(w.max),
        f(w.grid_cell),
        w.grid_w,
        w.grid_h,
        w.grid_w * w.grid_h + 1.0,
        f(w.grid_y0),
        f(w.grid_y_levels / (w.grid_y1 - w.grid_y0)),
        f(w.grid_y_levels - 1.0),
        f(w.ceiling_y),
        w.max_boxes,
        m::ceil((w.grid_w * w.grid_h) / 16.0),
        f3(w.rc_min),
        f3(w.rc_size),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The string the TypeScript itself prints (bun, src/scene/world.ts wgslWorldConsts()).
    const FROM_TYPESCRIPT: &str = "
const WORLD_MIN = vec3f(0.0, -0.25, 0.0);
const WORLD_MAX = vec3f(40.0, 8.0, 28.0);
const GRID_CELL: f32 = 1.0;
const GRID_W: i32 = 40;
const GRID_H: i32 = 28;
const GRID_CB: u32 = 1121u;
const GRID_YQ0: f32 = -0.25;
const GRID_YQS: f32 = 66.66666666666667;
const GRID_YQMAX: f32 = 239.0;
const CEIL_Y: f32 = 3.0;
const MAX_BOXES: u32 = 4096u;   // boxes.ts BoxWorld.maxBoxes: the marking table follows the mat rows of this many boxes (common.wgsl MARK_BASE)
const REACH_ROW_WORDS: u32 = 70u;   // per-light reach mask row (render/reach.ts: 2 bits a cell)
const RC_MIN = vec3f(0.0, 0.0, 0.0);
const RC_SIZE = vec3f(40.0, 3.0, 28.0);
";

    #[test]
    fn the_same_text_as_the_typescript() {
        assert_eq!(wgsl_world_consts(), FROM_TYPESCRIPT);
    }
}
