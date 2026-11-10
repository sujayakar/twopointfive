// THE AIR. What gets through it to the eye, what it sends by itself, and what it scatters: one place along each
// ray is picked by how much it turns light our way, and is then lit like any other place, by a lamp and by a ray.

import { f32, struct, vec3f } from '../gpu/layout';
import type { Graph } from '../gpu/graph';
import type { Views } from '../page/views';
import { choice, slider, toggle } from '../page/knobs';
import { Signal } from './signals';
import { HitY } from './bounce';

const AirPoint = struct('AirPoint', {
  position: vec3f('the place that was picked', { unit: 'm', look: { kind: 'place' } }),
  distance: f32('how far off; negative: none', { unit: 'm', look: { kind: 'heat', from: 0, to: 10 } }),
  weight: vec3f('how much of the light there is turned to the eye'),
  g: f32('how forward', { look: { kind: 'share' } }),
  passes: vec3f('what gets through to the eye', { look: { kind: 'color' } }),
  steps: f32(''),
  added: vec3f('what the smoke itself sends'),
  spare: f32('', { look: { kind: 'none' } }),
});

choice('source_air', {
  label: 'Smoke', value: 2, options: [['off', 'Vacuum'], ['raw', 'Raw'], ['settled', 'Settled'], ['truth', 'Truth']],
  about: 'What the air adds and takes. Vacuum: neither.',
});
slider('air_steps', { label: 'Steps along a ray', min: 2, max: 64, step: 1, value: 20, about: 'Fewer, and thin smoke is missed or banded; the dice spread what is missed as noise.' });
slider('air_shadow_steps', { label: 'Steps of a shadow ray', min: 0, max: 24, step: 1, value: 8, about: '0: shadow rays go through smoke as if it were not there.' });
choice('air_bounces', {
  label: 'From bounces, smoke takes', value: 0, options: [['absorbs', 'What it absorbs'], ['all', 'What it scatters too']],
  about: 'Light that smoke turns out of a bounce ray is lost, and light that it turns into it is not looked for. Taking only what is absorbed lets the two cancel; taking all makes rooms with haze too dark.',
});
toggle('break_air_skips', { label: 'Shadow rays skip smoke', about: 'Smoke that nothing shades: it shines evenly, and no beam shows.' });
toggle('break_mean_density', { label: 'Average the density', about: 'A coarse level of smoke made by averaging how thick it is. Thick next to thin lets more through than all of it alike, so it blocks too much.' });

export function addAir(graph: Graph, views: Views): void {
  graph.addBuffer({ name: 'air_point', struct: AirPoint, size: 'wide', about: 'the air between the eye and x' });
  graph.addBuffer({ name: 'air_lamp', struct: Signal, size: 'wide', about: 'what lamps scatter to the eye' });
  graph.addBuffer({ name: 'hit_a', struct: HitY, size: 'wide', about: 'where rays from the air landed' });
  graph.addStage({
    name: 'air march', question: 'What gets through, and where may light be turned our way?',
    reads: ['hit_x'], writes: ['air_point'], uses: ['smoke', 'dice', 'draw'], over: 'wide',
    code: `
fn run(pixel: vec2u) {
  let hit = hit_x_at(render_pixel(vec2i(pixel), frame.wide_step));
  let toward = normalize(hit.position - frame.eye);
  var most = hit.distance;
  if (most < 0.0) { most = 60.0; }
  let d = dice2(pixel, DICE_AIR);
  let s = smoke_scatter(frame.eye, toward, most, max(width_of_pixel(), knobs.forced_width / max(most, 1.0)), knobs.air_steps, d.x, d.y);
  var out: AirPoint;
  out.position = frame.eye + toward * s.distance;
  out.distance = select(-1.0, s.distance, s.happens);
  out.weight = s.weight;
  out.g = s.g;
  out.passes = s.passes;
  out.added = s.added;
  if (s.happens) { draw_ray_of(pixel, frame.wide_step, frame.eye, out.position, RAY_ONWARD); }
  put_air_point(pixel, out);
}`,
  });
  graph.addStage({
    name: 'air lamps', question: 'What does one lamp send to that place?',
    reads: ['air_point'], writes: ['air_lamp'], uses: ['ray', 'lamps', 'smoke', 'dice', 'draw'], over: 'wide',
    code: `
/// what a lamp sends to a place that faces every way
fn score_in_air(lamp: Lamp, origin: vec3f) -> f32 {
  let b = brightness(lamp.radiance);
  if (lamp.kind == LAMP_SUN) { return b * lamp.radius * lamp.radius; }
  let to = lamp.center - origin;
  let d2 = dot(to, to);
  if (lamp.kind == LAMP_BALL) { return b * min(1.0, lamp.radius * lamp.radius / d2); }
  return b * max(-dot(to, lamp.normal), 0.0) * lamp.area / (d2 * sqrt(d2) * PI + 1e-4);
}
fn run(pixel: vec2u) {
  let a = air_point_at(vec2i(pixel));
  var out: Signal;
  if (a.distance < 0.0) { put_air_lamp(pixel, out); return; }
  var total = 0.0;
  var kept = NOTHING;
  var kept_score = 0.0;
  var u = dice1(pixel, DICE_AIR_LAMP);
  for (var i = 0u; i < world.lamp_count; i++) {
    let s = score_in_air(lamps[i], a.position);
    if (s <= 0.0) { continue; }
    total += s;
    let take = s / total;
    if (u < take) { kept = i; kept_score = s; u = u / take; } else { u = (u - take) / (1.0 - take); }
  }
  if (kept == NOTHING) { put_air_lamp(pixel, out); return; }
  let aim = aim_at(lamps[kept], a.position, dice2(pixel, DICE_AIR_POINT));
  let most = aim.distance - 2e-3;
  let met = nearest(shadow_cone(a.position, aim.toward, most, lamp_size(lamps[kept])));
  draw_ray_of(pixel, frame.wide_step, a.position, a.position + aim.toward * min(met.distance, 20.0), select(RAY_BLOCKED, RAY_SEEN, met.piece == NOTHING));
  if (met.piece == NOTHING && aim.chance > 0.0) {
    var passes = vec3f(1.0);
    if (knobs.break_air_skips == 0u && knobs.air_shadow_steps > 0u) {
      passes = smoke_between(a.position, aim.toward, most, 0.0, 0.02, knobs.air_shadow_steps, dice1(pixel, DICE_AIR_STEP), false).passes;
    }
    let forward = normalize(a.position - frame.eye);
    out.value = aim.light * passes * (phase(dot(forward, aim.toward), a.g) * total / (kept_score * aim.chance));
  }
  put_air_lamp(pixel, out);
}`,
  });
  graph.addStage({
    name: 'air rays', question: 'What else is round that place?',
    reads: ['air_point'], writes: ['hit_a'], uses: ['y', 'smoke', 'dice', 'draw'], over: 'wide',
    code: `
fn run(pixel: vec2u) {
  let a = air_point_at(vec2i(pixel));
  var out: HitY;
  out.distance = NO_RAY;
  if (a.distance >= 0.0) {
    // drawn as the smoke turns light, so that the mean is plain
    let toward = phase_draw(normalize(a.position - frame.eye), a.g, dice2(pixel, DICE_AIR_WAY));
    let ray = wide_cone(a.position, toward);
    out = y_of(ray, nearest(ray), 1.0);
  }
  put_hit_a(pixel, out);
}`,
  });
  views.add({
    id: 'air passes', group: 'Air', name: 'What gets through', about: 'The share of x’s light that reaches the eye, of each color.',
    reads: ['air_point'], code: 'return air_point_at(at_wide(p)).passes;',
  });
}

/** after its rays have been answered: what the air adds, as a signal, and its denoiser's answer */
export function addAirAnswer(graph: Graph, views: Views): void {
  graph.addBuffer({ name: 'air_raw', struct: Signal, size: 'wide', about: 'what the air adds, this frame' });
  graph.addStage({
    name: 'air answer', question: 'What does the air add?',
    reads: { air_point: 'air_point', air_lamp: 'air_lamp', bounced: 'air_raw_y' }, writes: ['air_raw'], uses: [], over: 'wide',
    code: `
fn run(pixel: vec2u) {
  let a = air_point_at(vec2i(pixel));
  var v = a.added;
  if (a.distance >= 0.0) { v += a.weight * (air_lamp_at(vec2i(pixel)).value + bounced_at(vec2i(pixel)).value); }
  put_air_raw(pixel, Signal(v, max(a.distance, 0.0)));
}`,
  });
  views.add({
    id: 'air added', group: 'Air', name: 'What it adds, raw', about: 'Scattered and sent out, this frame.',
    reads: ['air_raw'], code: 'return show_light(air_raw_at(at_wide(p)).value);',
  });
  views.add({
    id: 'air settled', group: 'Air', name: 'What it adds, settled', about: 'After its denoiser.',
    reads: ['air_settled'], code: 'return show_light(air_settled_at(at_render(p)).value);',
  });
  views.add({
    id: 'air true', group: 'Air', name: 'What it adds, the truth', about: 'By the path tracer.',
    reads: ['truth'], code: 'return show_light(truth_at(at_render(p)).air);',
  });
}
