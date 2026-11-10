#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
use crate::game::Game;

/// A blast over the snow is told to the picture: the crater, the dig, the throw and the shove are its own (src/picture/snow). Nothing of the game reads it.
pub fn snow_blast_at(g: &mut crate::game::Game, at: &[f64], radius: f64) -> bool {
    let name = crate::scene_level_load::level_name_of(g);
    if !crate::weather_kinds::snow_level(g, name.as_deref()) { return false; }
    g.out.push(crate::out_events::OutEvent::SnowBlast(crate::out_events::OutSnowBlast { at: [at[0], at[1], at[2]], radius }));
    true
}
