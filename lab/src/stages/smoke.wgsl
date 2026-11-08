// SMOKE: air that is moved, kept incompressible, and carries density and heat. A cell knows which of its three far faces is shut (a wall's thickness does not
// matter: a face is shut when something stands between the two cells' middles), so smoke turns at walls, and goes through a door when the door opens.
//   state a, b   middle velocity xyz, density (per metre)      heat a, b      faces   bits 0‥2 shut by what stands, 3‥5 by that or a door      press
const N = vec3i(128, 40, 96); const H = 0.2; const AT = vec3f(-12.8, 0.0, -9.6);
fn ix(c: vec3i) -> u32 { return u32(c.x + N.x * (c.y + N.y * c.z)); }
fn inside(c: vec3i) -> bool { return all(c >= vec3i(0)) && all(c < N); }
fn mid(c: vec3i) -> vec3f { return AT + (vec3f(c) + 0.5) * H; }
