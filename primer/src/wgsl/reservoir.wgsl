// A RESERVOIR keeps one sample out of many it was offered, each with a chance in proportion to its weight,
// and remembers enough to weigh what it kept. A sample is a lamp and a place on it (two numbers from 0 to
// 1, which each surface turns into a point of the lamp as it sees it).

/// What a sample would send here with nothing in the way, over its chance on the lamp: landing light.
fn landing(lamp: u32, dice: vec2f, position: vec3f, normal: vec3f) -> vec3f {
  let aim = aim_at(lamps[lamp], position, dice);
  let cosine = dot(normal, aim.toward);
  if (cosine <= 0.0 || aim.chance <= 0.0) { return vec3f(0.0); }
  return aim.light * (cosine / (PI * aim.chance));
}

struct Gathering {
  lamp: u32,
  dice: vec2f,
  score: f32,  // brightness of what the kept sample would send here
  total: f32,  // the sum of all weights offered
  memory: f32, // how many samples stand behind it
  seen: f32,
  age: f32,
}

fn empty_gathering() -> Gathering { return Gathering(NOTHING, vec2f(0.0), 0.0, 0.0, 0.0, 1.0, 0.0); }

/// Offers one sample. `weight` = its score here / its chance.
fn offer(g: ptr<function, Gathering>, lamp: u32, dice: vec2f, score_here: f32, weight: f32, memory: f32, die: f32) -> bool {
  (*g).total += weight;
  (*g).memory += memory;
  if (weight <= 0.0 || die * (*g).total >= weight) { return false; }
  (*g).lamp = lamp;
  (*g).dice = dice;
  (*g).score = score_here;
  return true;
}

/// W: what to multiply the kept sample's light by, so that the mean is right.
fn weigh(g: Gathering, samples: f32) -> Reservoir {
  var r: Reservoir;
  r.lamp = g.lamp;
  r.dice = g.dice;
  r.score = g.score;
  r.memory = g.memory;
  r.seen = g.seen;
  r.age = g.age;
  if (g.score > 0.0) {
    r.weight = g.total / (g.score * select(samples, 1.0, knobs.break_no_mean != 0u));
  }
  return r;
}

/// Do two pixels look at the same surface? `spot` is where this one's is (or was).
fn same_surface(hit: HitX, spot: vec3f, other: HitX) -> bool {
  if (other.distance < 0.0 || other.lamp != NOTHING) { return false; }
  if (knobs.break_no_tests != 0u) { return true; }
  let off = abs(dot(other.position - spot, hit.normal));
  return dot(other.normal, hit.normal) > 0.9 && off < 0.02 * hit.distance && distance(other.position, spot) < 0.2 * hit.distance;
}
