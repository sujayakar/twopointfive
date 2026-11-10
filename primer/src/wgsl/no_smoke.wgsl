// There is no smoke in this room: the same names, and nothing in them.
struct Passage { passes: vec3f, added: vec3f }
struct Scattering { happens: bool, distance: f32, weight: vec3f, passes: vec3f, added: vec3f, g: f32 }
fn smoke_between(origin: vec3f, toward: vec3f, most: f32, width: f32, spread: f32, steps: u32, die: f32) -> Passage { return Passage(vec3f(1.0), vec3f(0.0)); }
fn smoke_scatter(origin: vec3f, toward: vec3f, most: f32, spread: f32, steps: u32, die: f32, pick: f32) -> Scattering { return Scattering(false, 0.0, vec3f(0.0), vec3f(1.0), vec3f(0.0), 0.0); }
fn phase(cosine: f32, g: f32) -> f32 { return 1.0 / (4.0 * PI); }
fn phase_draw(forward: vec3f, g: f32, dice: vec2f) -> vec3f { return forward; }
