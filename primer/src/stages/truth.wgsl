// One more path a pixel, added to the mean of all before it. Nothing is reused, clamped or blurred.
// Light from lamps is found two ways at every surface, by aiming at them and by running into them along a
// lobe, and each is weighed so that the two weights add to 1.
//
// Three ways leave x each frame: 0 aims at a lamp, 1 follows the matte lobe, 2 follows the coat's. They are
// walked one after another by a single loop, so that the tree of the world is in this shader once.

const AIM = 0u;
const WIDE = 1u;
const NARROW = 2u;
const AIR = 3u;

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
  in_air: bool,     // the place is in smoke, and turns light by its phase
  g: f32,
  place: Surface,   // the surface the ray leaves
  forward: vec3f,   // the way light came into the smoke, seen from x
  toward: vec3f,
  chance: f32,      // of that direction
  most: f32,        // how far a shadow ray goes
  carried: vec3f,   // the share of what is found next that reaches x
  aimed: vec3f,     // what a shadow ray brings if nothing is in its way
  total: f32,       // the sum of all lamps' scores at that surface
}

struct Parts { air: vec3f, still: vec3f, changing: vec3f, shine: vec3f, sky: vec3f, one: vec3f, two: vec3f, more: vec3f, narrow: vec3f }

/// Light that reached x along a way, after so many bounces, from a lamp (or from the sky).
fn deliver(parts: ptr<function, Parts>, w: Walk, bounces: u32, light: vec3f, lamp: u32) {
  if (w.way == AIR) { (*parts).air += light; }
  else if (w.way == NARROW) {
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

/// A lamp that a ray ran into counts for less, the likelier it was to be aimed at from there.
fn weight_of_met(w: Walk, lamp: u32, distance: f32) -> f32 {
  if (w.in_air || is_thin(w.place)) { return 0.0; } // from smoke and thin things, lamps are only aimed at
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

fn score_in_air(lamp: Lamp, origin: vec3f) -> f32 {
  let b = brightness(lamp.radiance);
  if (lamp.kind == LAMP_SUN) { return b * lamp.radius * lamp.radius; }
  let to = lamp.center - origin;
  let d2 = dot(to, to);
  if (lamp.kind == LAMP_BALL) { return b * min(1.0, lamp.radius * lamp.radius / d2); }
  return b * max(-dot(to, lamp.normal), 0.0) * lamp.area / (d2 * sqrt(d2) * PI + 1e-4);
}
fn aim_from_air(w: ptr<function, Walk>) -> u32 {
  var total = 0.0;
  var kept = NOTHING;
  var kept_score = 0.0;
  for (var i = 0u; i < world.lamp_count; i++) {
    let s = score_in_air(lamps[i], (*w).place.position);
    total += s;
    if (s > 0.0 && die() * total < s) { kept = i; kept_score = s; }
  }
  (*w).total = 0.0;
  (*w).shadow = false;
  if (kept == NOTHING) { return NOTHING; }
  let aim = aim_at(lamps[kept], (*w).place.position, die2());
  if (aim.chance <= 0.0) { return NOTHING; }
  (*w).shadow = true;
  (*w).toward = aim.toward;
  (*w).most = aim.distance - 2e-3;
  (*w).chance = kept_score / total * aim.chance;
  (*w).aimed = aim.light;
  return kept;
}

/// Sets the ray that asks one lamp, from the surface the walk stands at.
fn aim_a_ray(w: ptr<function, Walk>) -> u32 {
  let s = (*w).place;
  if ((*w).in_air) { return aim_from_air(w); }
  // something thin is lit from both sides; one is asked, half the time each
  var normal = s.normal;
  if (is_thin(s) && die() < 0.5) { normal = -normal; }
  let pick = pick_by_score(s.position, normal, die());
  (*w).total = pick.total;
  (*w).shadow = false;
  if (pick.lamp == NOTHING) { return NOTHING; }
  let aim = aim_at(lamps[pick.lamp], s.position, die2());
  if (dot(normal, aim.toward) <= 0.0) { return NOTHING; }
  (*w).shadow = true;
  (*w).toward = aim.toward;
  (*w).most = aim.distance - 2e-3;
  (*w).chance = pick.chance * aim.chance * select(1.0, 0.5, is_thin(s));
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
  let is_surface = hit.distance >= 0.0 && hit.lamp == NOTHING;
  if (!is_surface && world.volume_count == 0u) {
    put_truth(pixel, sum);
    return;
  }
  let first_reach = select(hit.distance, 60.0, hit.distance < 0.0);
  start_dice(pixel);
  let x = Surface(hit.position, hit.normal, -pixel_toward(pixel), hit.matte, hit.through, hit.roughness, hit.metal);
  var parts: Parts;
  var w: Walk;
  var asked = NOTHING;   // the lamp a shadow ray goes to
  var total_at_x = 0.0;
  w.way = AIM;
  w.place = x;
  w.carried = vec3f(1.0);
  if (is_surface) { asked = aim_a_ray(&w); } else { w.way = NARROW; }
  total_at_x = w.total;
  w.done = !w.shadow;

  let most_rays = 2u + 6u * (knobs.truth_bounces + 1u);
  for (var k = 0u; k < most_rays; k++) {
    // a walk that is over: the next way starts at x
    if (w.done) {
      if (w.way == AIR || (w.way == NARROW && world.volume_count == 0u)) { break; }
      w.way++;
      w.in_air = false;
      w.bounces = 0u;
      w.shadow = false;
      w.done = false;
      w.place = x;
      w.total = total_at_x;
      w.carried = vec3f(1.0);
      if (w.way == AIR) {
        // from the eye toward x, as far as that is: only what happens on the way
        w.place.position = frame.eye;
        w.place.normal = vec3f(0.0);
        w.toward = pixel_toward(pixel);
        w.in_air = true;
        w.total = 0.0;
      } else if (w.way == WIDE) {
        let front = brightness(front_of(x));
        let side = select(-x.normal, x.normal, front >= 1.0 || die() < front);
        w.toward = wide_draw(side, die2());
        w.chance = wide_chance(side, w.toward);
      } else {
        w.toward = coat_draw(x, die2());
        w.chance = coat_chance(x, w.toward);
        w.carried = coat_weight(x, w.toward);
        if (w.chance <= 0.0) { w.done = true; continue; }
      }
    }

    // the one ray of this turn
    var reach = select(1e4, w.most, w.shadow);
    if (w.way == AIR && w.bounces == 0u && !w.shadow) { reach = first_reach; }
    var start = w.place.position;
    if (!w.in_air) { start = leave_surface(w.place.position, w.place.normal, w.toward); }
    var ray = fine_ray(start, w.toward, 1e-4, reach);
    if (knobs.truth_by_width != 0u) {
      // as wide as the real-time frame's rays are: what the levels alone do to the light
      if (w.shadow) { ray = shadow_cone(ray.origin, w.toward, reach, lamp_size(lamps[asked])); }
      else if (w.way == NARROW && w.bounces == 0u) { ray = narrow_cone(ray.origin, w.toward, w.place.roughness); }
      else { ray = wide_cone(ray.origin, w.toward); }
    }
    let met = nearest(ray);
    show_ray(pixel, w, ray, met);
    // the smoke on the way: some of the light is turned there, and the rest gets through
    let air = smoke_scatter(ray.origin, ray.toward, min(met.distance, reach), 0.0, 24u, die(), die());

    if (w.shadow) {
      let clear = met.piece == NOTHING;
      w.aimed *= air.passes;
      let s = w.place;
      if (w.in_air) {
        if (clear) { deliver(&parts, w, w.bounces, w.carried * w.aimed * (phase(dot(w.forward, w.toward), w.g) / w.chance), asked); }
        w.shadow = false;
        w.toward = phase_draw(w.forward, w.g, die2());
        if (w.bounces >= knobs.truth_bounces) { w.done = true; }
        continue;
      }
      let cosine = dot(s.normal, w.toward);
      if (w.way == AIM) {
        if (clear) {
          let front = front_of(s);
          let weight = select(balance(w.chance, wide_chance(s.normal, w.toward)), 1.0, is_thin(s));
          let matte = select(1.0 - front, front, cosine > 0.0) * (abs(cosine) / PI * weight / w.chance);
          let coat = coat_response(s, w.toward) * select(balance(w.chance, coat_chance(s, w.toward)), 1.0, is_thin(s)) / w.chance;
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
        let response = matte_response(s, w.toward) + coat_response(s, w.toward);
        let weight = select(balance(w.chance, lobe), 1.0, is_thin(s));
        deliver(&parts, w, w.bounces, w.carried * response * w.aimed * (weight / w.chance), asked);
      }
      // the lamp has been asked; the walk goes on, along one lobe or the other
      go_on(&w, odds);
      continue;
    }

    // an open ray. The smoke's own light first; then either a place in the smoke that turns light, or what is behind
    if (any(air.added > vec3f(0.0))) { deliver(&parts, w, max(w.bounces, select(0u, 1u, w.way != AIR)), w.carried * air.added, NOTHING); }
    if (air.happens) {
      let turned = brightness(air.weight);
      let through = brightness(air.passes);
      let odds = select(turned / max(turned + through, 1e-6), 1.0, w.way == AIR && w.bounces == 0u);
      if (die() < odds) {
        w.carried *= air.weight / odds;
        w.bounces++;
        w.in_air = true;
        w.g = air.g;
        w.forward = w.toward;
        w.place.position = ray.origin + ray.toward * air.distance;
        w.place.normal = vec3f(0.0);
        asked = aim_a_ray(&w);
        if (!w.shadow) {
          w.toward = phase_draw(w.forward, w.g, die2());
          if (w.bounces >= knobs.truth_bounces) { w.done = true; }
        }
        continue;
      }
      w.carried *= air.passes / max(1.0 - odds, 1e-6);
    } else { w.carried *= air.passes; }
    if (w.way == AIR && w.bounces == 0u) { w.done = true; continue; } // x itself is the other ways' business
    // what did it find?
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
    w.in_air = false;
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
  sum.air = mean_with(sum.air, parts.air, k);
  put_truth(pixel, sum);
}

/// From the surface the walk stands at, on along a lobe.
fn go_on(w: ptr<function, Walk>, odds: f32) {
  let s = (*w).place;
  let front = brightness(front_of(s));
  var side = s.normal;
  if (front < 1.0 && die() >= front) { side = -side; }
  var toward = wide_draw(side, die2());
  if (die() < odds) { toward = coat_draw(s, die2()); }
  let on_front = dot(s.normal, toward) > 0.0;
  let cosine = abs(dot(s.normal, toward));
  let by_matte = cosine / PI * select(1.0 - front, front, on_front);
  let chance = mix(by_matte, coat_chance(s, toward), odds);
  (*w).shadow = false;
  (*w).toward = toward;
  (*w).chance = chance;
  if (cosine <= 0.0 || chance <= 0.0) { (*w).done = true; return; }
  let slant = select(cosine, 0.5, knobs.break_no_cosine != 0u);
  let share = front_of(s);
  (*w).carried *= (matte_share(s) * select(1.0 - share, share, on_front) * slant / PI + coat_response(s, toward)) / chance;
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
