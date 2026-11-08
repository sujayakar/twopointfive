//! HOW A PUFF OF DUST LOOKS: the table the particles' kernels read for the kind (render/particles.ts). One row, and the page takes it when it changes.
use alloc::vec::Vec;
use game::game::Game;

section! {
    /// peak opacity at its centre · × radius over its life · seconds to full opacity · × the softening where it meets a surface · the share of a crossing beam scattered toward the eye · × the cache's ambient on it
    pub struct DustLook {
        pub opacity: f32,
        pub growth: f32,
        pub fade_in: f32,
        pub soft: f32,
        pub wrap: f32,
        pub ambient: f32,
    }
}

pub fn fill(g: &Game, out: &mut Vec<DustLook>) {
    let l = g.m.particle_fx.dust_look;
    out.push(DustLook { opacity: l.opacity as f32, growth: l.growth as f32, fade_in: l.fade_in as f32, soft: l.soft as f32, wrap: l.wrap as f32, ambient: l.ambient as f32 });
}
