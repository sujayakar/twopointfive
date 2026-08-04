// ===========================================================================
// Incandescent sparks, drawn as additive trails.
//
// Separate from the fluid on purpose. Reference footage of a live flashbang is
// mostly smoke by volume, but what reads as *violence* in the first fifth of a
// second is several hundred burning fragments arcing outward, each drawing a
// fine bright filament. Those are emissive points under ballistic motion —
// they are not a density field, they do not advect, and trying to express them
// as smoke sources produced fat blobs rather than streaks.
//
// One line segment per spark, from where it was last frame to where it is now,
// so the trail length is its speed and no history buffer is needed. Additive,
// because an ember is emissive and nothing it passes in front of should dim
// it — which is also why it reads through smoke in the footage.
// ===========================================================================

struct SparkView {
  viewProj : mat4x4f,
}

@group(0) @binding(0) var<uniform> S : SparkView;

struct VSOut {
  @builtin(position) pos : vec4f,
  @location(0) color : vec3f,
}

@vertex
fn vs(
  @location(0) worldPos : vec3f,
  @location(1) color : vec3f,
) -> VSOut {
  var o: VSOut;
  o.pos = S.viewProj * vec4f(worldPos, 1.0);
  o.color = color;
  return o;
}

@fragment
fn fs(in: VSOut) -> @location(0) vec4f {
  // Alpha 1 with a One/One blend: the colour already carries the intensity,
  // and per-vertex fade is what makes the tail of each streak vanish.
  return vec4f(in.color, 1.0);
}
