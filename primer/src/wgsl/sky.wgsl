/// What comes from beyond the world, the sun's disc left out (the sun is a lamp).
fn sky(toward: vec3f) -> vec3f {
  let up = toward.y;
  if (up < 0.0) { return mix(world.sky_horizon, world.sky_ground, smoothstep(0.0, 0.25, -up)); }
  return mix(world.sky_horizon, world.sky_zenith, pow(up, 0.5));
}
