use crate::v3::V3;

/// The scattering tint of a surface's dust (reached by the stun grenade's tile drop, tick 1621). A pure table; the solver never reads the field.
pub(super) fn dust_tint(surf: crate::scene_materials::Surf) -> V3 {
    use crate::scene_materials::Surf as S;
    match surf {
        S::Drywall => [0.9, 0.87, 0.81],
        S::Concrete => [0.72, 0.71, 0.69],
        S::Wood | S::Cardboard => [0.78, 0.68, 0.54],
        S::Fabric | S::Carpet => [0.76, 0.75, 0.74],
        _ => [0.82, 0.8, 0.77],
    }
}
