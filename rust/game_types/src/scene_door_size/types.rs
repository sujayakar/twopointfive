//! The types of `scene_door_size`.

#[derive(Clone, Copy, Debug, Default)]
pub struct DOORSIZE {
    pub gap: f64,
    pub head: f64,
    pub cell: f64,
    pub reach: f64,
}

pub fn door_size() -> crate::scene_door_size::DOORSIZE { crate::scene_door_size::DOORSIZE { gap: 0.9, head: 2.05, cell: 0.5, reach: 0.1 } }

pub const LEAF_H: f64 = 2.01;
