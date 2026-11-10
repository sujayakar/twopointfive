// KNOBS FOR WHAT A RAY SEES: how wide rays are, and what that does to levels.

import { choice, slider, toggle } from '../page/knobs';

export function addLevelKnobs(): void {
  slider('forced_width', {
    label: 'Force detail: no finer than', min: 0, max: 1, step: 0.005, value: 0, unit: 'm', resetsTruth: true,
    about: 'The eye’s rays count as at least this wide, so every object, pattern and mesh shows the level that a ray so wide would meet. It is how a bounce sees the scene.',
  });
  toggle('forced_all', { label: '…for every ray', about: 'Force the same on shadow rays, wide rays and mirrors.', resetsTruth: true });
  slider('spread_wide', {
    label: 'Wide rays widen by', min: 0, max: 2, step: 0.01, value: 0.3, unit: 'm a m', resetsTruth: true,
    about: 'How much width a wide ray gains for each meter. What it meets is then the level of that width. 0: the finest everywhere.',
  });
  slider('spread_shadow', {
    label: 'Shadow rays widen by', min: 0, max: 2, step: 0.01, value: 0.5, unit: '× the lamp', resetsTruth: true,
    about: 'In units of how large the lamp looks. Wider than the lamp, and light comes past things that should hide it.',
  });
  slider('level_blend', {
    label: 'Levels blend over', min: 0, max: 3, step: 0.1, value: 1, unit: 'octaves', resetsTruth: true,
    about: 'A die a ray makes its width a little more or less, so that one level gives way to the next as noise and not as a seam.',
  });
  choice('seam', {
    label: 'A ray that starts inside', value: 0, options: [['skip', 'Gets out'], ['solid', 'Is stopped'], ['margin', 'Starts further off']], resetsTruth: true,
    about: 'x is found on the fine surface; its rays meet a coarser one, which may lie over it. Let them out through the coarse piece’s back; or stop them, and the surface blackens; or start them further off, and shadows come loose.',
  });
  slider('seam_margin', { label: 'How much further off', min: 0, max: 0.2, step: 0.005, value: 0.03, unit: 'm', about: 'For “starts further off”.', resetsTruth: true });
  toggle('truth_by_width', {
    label: 'The truth obeys levels too', resetsTruth: true,
    about: 'Paths are traced with rays as wide as the real-time frame’s. The difference from the plain truth is what levels alone do to the light.',
  });
  choice('mesh_trace', {
    label: 'Of meshes, rays meet', value: 0, options: [['pieces', 'Pieces'], ['coarse', 'Coarse triangles'], ['fine', 'Fine triangles']], resetsTruth: true,
    about: 'Pieces that stand in for the mesh (one kind of test, smooth, few); a coarse cut of it; or the mesh as it is. With triangles the ray shader holds more code, and pays for it everywhere.',
  });
  choice('mesh_eye', {
    label: 'The eye meets meshes by', value: 0, options: [['raster', 'Raster'], ['rays', 'Rays']], resetsTruth: true,
    about: 'Drawn as triangles with clusters picked by their error on the screen, or met by the same rays as everything else. The two must agree.',
  });
  toggle('use_change', { label: 'Heed the change signal', value: true, about: 'Where something moved or a lamp changed, histories and the cache trust the past less.' });
  slider('change_reach', { label: 'A change reaches', min: 0, max: 6, step: 0.1, value: 1.5, unit: 'm', about: 'How far from what changed the past is doubted.' });
  toggle('break_no_before', { label: 'Forget where things were', about: 'Every history then looks for a moving thing where it is now: it smears.' });
  toggle('lists_merged', { label: 'One list for all', about: 'What stands and what moves in one tree, made anew every frame. See the meter for what the making costs.' });
}

export const CHANGE = `
// THE CHANGE SIGNAL: how far the past can be trusted at a place. 1 unless something moved or a lamp changed
// near by.
fn trust_at(position: vec3f) -> f32 {
  if (knobs.use_change == 0u) { return 1.0; }
  var trust = 1.0;
  for (var i = 0u; i < events.count; i++) {
    let e = events.items[i];
    let d = distance(position, e.center) - e.radius;
    trust *= 1.0 - e.strength * (1.0 - smoothstep(0.0, max(knobs.change_reach, 1e-3), d));
  }
  return trust;
}`;
