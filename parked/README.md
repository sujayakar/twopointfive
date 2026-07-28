# Parked code

Not part of the build. `tsconfig.json` includes only `src`, and nothing imports
from here, so this directory is inert — it exists because the project has no
version control, and deleting tested work with no way to get it back is a
harder decision than it looks.

## physics.ts

Rigid-body simulation for grenades and shootable crates. Removed 2026-07-27
because moving geometry looks bad in this renderer, not because the module is
wrong: fast movement defeats temporal reprojection, so anything travelling
quickly is visibly noisy, and every moving object also has to leave the static
BVH and be tested linearly per ray.

It was verified before removal — 26/26 assertions, bodies settling with exactly
zero residual velocity, restitution exact to six decimals, no tunnelling at
34 m/s, raycasts cross-checked against hand-computed values, 0.066 ms per step
for 32 bodies. It was never run on the GPU or against the real level.

The one piece worth resurrecting on its own is `raycast()` — a CPU BVH traversal
ported from the WGSL in `common.wgsl`. Shooting out lights and guard
line-of-sight both need exactly that, and neither involves moving any geometry.
