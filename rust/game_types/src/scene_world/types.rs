//! The types of `scene_world`.

#[derive(Clone, Copy, Debug, Default)]
pub struct WORLD {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub grid_cell: f64,
    pub grid_w: f64,
    pub grid_h: f64,
    pub grid_y0: f64,
    pub grid_y1: f64,
    pub grid_y_levels: f64,
    pub grid_y_pad: f64,
    pub grid_jitter: f64,
    pub ceiling_y: f64,
    pub max_boxes: f64,
}

pub fn world() -> crate::scene_world::WORLD { crate::scene_world::WORLD { min: [0.0, -0.25, 0.0], max: [40.0, 8.0, 28.0], grid_cell: 1.0, grid_w: 40.0, grid_h: 28.0, grid_y0: -0.25, grid_y1: 3.35, grid_y_levels: 240.0, grid_y_pad: 0.04, grid_jitter: 0.00005, ceiling_y: 3.0, max_boxes: 4096.0 } }
