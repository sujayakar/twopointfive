// One more path a pixel, added to the mean of all before it. Nothing is reused, clamped or blurred.
// Light from lamps is found two ways at every surface, by aiming at them and by running into them along a
// lobe, and each is weighed so that the two weights add to 1.
//
// Three ways leave x each frame: 0 aims at a lamp, 1 follows the matte lobe, 2 follows the coat's. They are
// walked one after another by a single loop, so that the tree of the world is in this shader once.

const AIM = 0u;
const WIDE = 1u;
const NARROW = 2u;

// its own dice, always white, so that the truth does not change when yours do
var<private> dice_state: u32;
fn start_dice(pixel: vec2u) { dice_state = hash2(pixel.x, hash2(pixel.y, hash(frame.truth_number))); }
fn die() -> f32 {
  dice_state = dice_state * 747796405u + 2891336453u;
  return unit(hash(dice_state));
}
fn die2() -> vec2f { return vec2f(die(), die()); }

/// One way's walk: where its ray is now, and what it has found so far.
struct Walk {
  way: u32,
  bounces: u32,     // surfaces visited after x
  shadow: bool,     // this ray only asks whether a lamp is seen
  done: bool,
  place: Surface,   // the surface the ray leaves
  toward: vec3f,
  chance: f32,      // of that direction
  most: f32,        // how far a shadow ray goes
  carried: vec3f,   // the share of what is found next that reaches x
  aimed: vec3f,     // what a shadow ray brings if nothing is in its way
  total: f32,       // the sum of all lamps' scores at that surface
}

struct Parts { still: vec3f, changing: vec3f, shine: vec3f, sky: vec3f, one: vec3f, two: vec3f, more: vec3f, narrow: vec3f }

/// Light that reached x along a way, after so many bounces, from a lamp (or from the sky).
fn deliver(parts: ptr<function, Parts>, w: Walk, bounces: u32, light: vec3f, lamp: u32) {
  if (w.way == NARROW) {
    if (bounces == 0u && lamp != NOTHING) { (*parts).shine += light; }
    else { (*parts).narrow += light; }
  }
  else if (bounces == 1u) { (*parts).one += light; }
  else if (bounces == 2u) { (*parts).two += light; }
  else if (bounces > 2u) { (*parts).more += light; }
  else if (lamp == NOTHING) { (*parts).sky += light; }
  else if (lamps[lamp].changing != 0u) { (*parts).changing += light; }
  else { (*parts).still += light; }
}

fn sun_met(toward: vec3f) -> u32 {
  for (var i = 0u; i < world.lamp_count; i++) {
    let l = lamps[i];
    if (l.kind == LAMP_SUN && dot(toward, l.normal) > 1.0 - 0.5 * l.radius * l.radius) { return i; }
  }
  return NOTHING;
}

/// A lamp that a ray ran into counts for less, the likelier it was to be aimed at from there.
fn weight_of_met(w: Walk, lamp: u32, distance: f32) -> f32 {
  if (w.total <= 0.0) { return 1.0; }
  let at = lamps[lamp];
  let picked = score(at, w.place.position, w.place.normal) / w.total;
  return balance(w.chance, picked * aim_chance(at, w.place.position, w.toward, distance));
}

/// How likely each lobe is to be followed past x.
fn coat_odds(s: Surface) -> f32 {
  let coat = brightness(coat_share(s));
  return clamp(coat / max(coat + brightness(matte_share(s)), 1e-6), 0.1, 0.9);
}

/// Sets the ray that asks one lamp, from the surface the walk stands at.
fn aim_a_ray(w: ptr<function, Walk>) -> u32 {
  let s = (*w).place;
  let pick = pick_by_score(s.position, s.normal, die());
  (*w).total = pick.total;
  (*w).shadow = false;
  if (pick.lamp == NOTHING) { return NOTHING; }
  let aim = aim_at(lamps[pick.lamp], s.position, die2());
  if (dot(s.normal, aim.toward) <= 0.0) { return NOTHING; }
  (*w).shadow = true;
  (*w).toward = aim.toward;
  (*w).most = aim.distance - 2e-3;
  (*w).chance = pick.chance * aim.chance;
  (*w).aimed = aim.light;
  return pick.lamp;
}

/// The pinned pixel's rays, for the page.
fn show_ray(pixel: vec2u, w: Walk, ray: Ray, met: Meeting) {
  let end = ray.origin + ray.toward * min(met.distance, select(6.0, 1e4, w.shadow || met.piece != NOTHING));
  var kind = RAY_ONWARD;
  if (w.shadow) { kind = select(RAY_BLOCKED, RAY_SEEN, met.piece == NOTHING); }
  else if (w.bounces == 0u) { kind = select(RAY_NARROW, RAY_WIDE, w.way == WIDE); }
  draw_ray(pixel, ray.origin, end, kind);
}

fn run(pixel: vec2u) {
  let hit = hit_x_at(vec2i(pixel));
  var sum = kept_truth(pixel);
  if (hit.distance < 0.0 || hit.lamp != NOTHING) {
    put_truth(pixel, sum);
    return;
  }
  start_dice(pixel);
  let x = Surface(hit.position, hit.normal, -pixel_toward(pixel), hit.matte, hit.through, hit.roughness, hit.metal);
  var parts: Parts;
  var w: Walk;
  var asked = NOTHING;   // the lamp a shadow ray goes to
  var total_at_x = 0.0;
  w.way = AIM;
  w.place = x;
  w.carried = vec3f(1.0);
  asked = aim_a_ray(&w);
  total_at_x = w.total;
  w.done = !w.shadow;

  let most_rays = 2u + 4u * (knobs.truth_bounces + 1u);
  for (var k = 0u; k < most_rays; k++) {
    // a walk that is over: the next way starts at x
    if (w.done) {
      if (w.way == NARROW) { break; }
      w.way++;
      w.bounces = 0u;
      w.shadow = false;
      w.done = false;
      w.place = x;
      w.total = total_at_x;
      w.carried = vec3f(1.0);
      if (w.way == WIDE) {
        w.toward = wide_draw(x.normal, die2());
        w.chance = wide_chance(x.normal, w.toward);
      } else {
        w.toward = coat_draw(x, die2());
        w.chance = coat_chance(x, w.toward);
        w.carried = coat_weight(x, w.toward);
        if (w.chance <= 0.0) { break; }
      }
    }

    // the one ray of this turn
    let reach = select(1e4, w.most, w.shadow);
    let ray = Ray(leave_surface(w.place.position, w.place.normal, w.toward), w.toward, 1e-4, reach);
    let met = nearest(ray);
    show_ray(pixel, w, ray, met);

    if (w.shadow) {
      let clear = met.piece == NOTHING;
      let s = w.place;
      let cosine = dot(s.normal, w.toward);
      if (w.way == AIM) {
        if (clear) {
          let matte = cosine / PI * balance(w.chance, wide_chance(s.normal, w.toward)) / w.chance;
          let coat = coat_response(s, w.toward) * balance(w.chance, coat_chance(s, w.toward)) / w.chance;
          if (lamps[asked].changing != 0u) { parts.changing += w.aimed * matte; }
          else { parts.still += w.aimed * matte; }
          parts.shine += w.aimed * coat;
        }
        w.done = true;
        continue;
      }
      let odds = coat_odds(s);
      if (clear) {
        let lobe = mix(wide_chance(s.normal, w.toward), coat_chance(s, w.toward), odds);
        let response = matte_share(s) * cosine / PI + coat_response(s, w.toward);
        deliver(&parts, w, w.bounces, w.carried * response * w.aimed * (balance(w.chance, lobe) / w.chance), asked);
      }
      // the lamp has been asked; the walk goes on, along one lobe or the other
      go_on(&w, odds);
      continue;
    }

    // an open ray: what did it find?
    var lamp = NOTHING;
    var found: Found;
    if (met.piece == NOTHING) { lamp = sun_met(w.toward); }
    else { found = found_at(ray, met); lamp = found.lamp; }
    if (lamp != NOTHING) {
      deliver(&parts, w, w.bounces, w.carried * lamps[lamp].radiance * weight_of_met(w, lamp, met.distance), lamp);
    }
    if (met.piece == NOTHING) { deliver(&parts, w, w.bounces, w.carried * sky(w.toward), NOTHING); }
    if (met.piece == NOTHING || lamp != NOTHING || w.bounces == knobs.truth_bounces) {
      w.done = true;
      continue;
    }
    w.bounces++;
    w.place = found.surface;
    asked = aim_a_ray(&w);
    if (!w.shadow) { go_on(&w, coat_odds(w.place)); }
  }

  let count = sum.count + 1.0;
  let k = 1.0 / count;
  sum.count = count;
  sum.still = mean_with(sum.still, parts.still, k);
  sum.changing = mean_with(sum.changing, parts.changing, k);
  sum.shine = mean_with(sum.shine, parts.shine, k);
  sum.sky = mean_with(sum.sky, parts.sky, k);
  sum.one = mean_with(sum.one, parts.one, k);
  sum.two = mean_with(sum.two, parts.two, k);
  sum.more = mean_with(sum.more, parts.more, k);
  sum.narrow = mean_with(sum.narrow, parts.narrow, k);
  put_truth(pixel, sum);
}

/// From the surface the walk stands at, on along a lobe.
fn go_on(w: ptr<function, Walk>, odds: f32) {
  let s = (*w).place;
  var toward = wide_draw(s.normal, die2());
  if (die() < odds) { toward = coat_draw(s, die2()); }
  let cosine = dot(s.normal, toward);
  let chance = mix(wide_chance(s.normal, toward), coat_chance(s, toward), odds);
  (*w).shadow = false;
  (*w).toward = toward;
  (*w).chance = chance;
  if (cosine <= 0.0 || chance <= 0.0) { (*w).done = true; return; }
  let slant = select(cosine, 0.5, knobs.break_no_cosine != 0u);
  (*w).carried *= (matte_share(s) * slant / PI + coat_response(s, toward)) / chance;
  // what carries little is followed less often, and counts for more when it is
  if ((*w).bounces > 2u) {
    let c = (*w).carried;
    let keep = clamp(max(c.x, max(c.y, c.z)), 0.05, 1.0);
    if (die() >= keep) { (*w).done = true; return; }
    (*w).carried /= keep;
  }
}

fn mean_with(old: vec3f, new_value: vec3f, weight: f32) -> vec3f {
  if (is_bad(new_value.x + new_value.y + new_value.z)) { return old; }
  return mix(old, new_value, weight);
}
