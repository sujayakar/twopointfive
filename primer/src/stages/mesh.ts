// MESHES TO THE EYE: pick clusters, draw their numbers, make places of them.

import type { Graph } from '../gpu/graph';
import { f32, struct, u32, vec3f } from '../gpu/layout';
import { MOST_JOBS } from '../mesh/meshes';
import { slider, toggle } from '../page/knobs';
import pick from './mesh_pick.wgsl';
import hit from './mesh_hit.wgsl';

export const MeshHit = struct('MeshHit', {
  normal: vec3f('', { look: { kind: 'direction' } }),
  distance: f32('negative: no mesh here', { unit: 'm', look: { kind: 'heat', from: 0, to: 12 } }),
  instance: u32('which one of them'),
  triangle: u32('which triangle'),
  cluster: u32('which cluster'),
  material: u32(''),
  level: f32('0 the finest', { look: { kind: 'heat', from: 0, to: 8 } }),
  error: f32('how far off the finest it may be', { unit: 'pixels', look: { kind: 'heat', from: 0, to: 2 } }),
});
const Picked = struct('Job', { instance: u32(''), cluster: u32('') });

export function addMesh(graph: Graph): void {
  slider('mesh_pixels', {
    label: 'Error that may be seen', min: 0.1, max: 64, step: 0.1, value: 0.75, ratio: true, unit: 'pixels', resetsTruth: true,
    about: 'A cluster is fine enough if it stands off the finest mesh by less than this, where it is nearest the eye.',
  });
  toggle('mesh_cull', { label: 'Leave out what is out of the picture', value: true, about: 'By each cluster’s ball. Fly the ghost to see what is missing.' });
  toggle('mesh_frozen', { label: 'Freeze the cut', about: 'The clusters stay picked for where the eye is now. Move, and look at what it picked.' });

  graph.addModule('mesh jobs', { code: '', needs: [], shared: ['world', 'mesh_jobs', 'mesh_clusters', 'mesh_instances', 'mesh_args'] });
  graph.addModule('mesh shapes', { code: '', needs: [], shared: ['mesh_vertices', 'mesh_corners', 'mesh_clusters', 'mesh_instances', 'mesh_ids'] });
  graph.addBuffer({ name: 'mesh_picked', struct: Picked, size: { count: MOST_JOBS }, about: 'the clusters that are drawn' });
  graph.addBuffer({ name: 'mesh_hit', struct: MeshHit, size: 'render', about: 'what of meshes each pixel sees' });
  graph.addBuffer({ name: 'no_mesh_hit', struct: MeshHit, size: 'render', about: 'nothing: there is no mesh in this room' });
  graph.addStage({
    name: 'mesh pick', question: 'Which clusters are fine enough, and not too fine?',
    reads: [], writes: ['mesh_picked'], uses: ['mesh jobs'], over: { count: MOST_JOBS }, code: pick,
  });
  graph.addStage({
    name: 'mesh draw', question: 'Which triangle is at each pixel?',
    reads: ['mesh_picked'], writes: ['mesh_drawn'], uses: [], over: { count: 1 }, code: '', custom: true,
  });
  graph.addBuffer({ name: 'mesh_drawn', struct: Picked, size: { count: 1 }, about: 'stands for the picture of numbers' });
  graph.addStage({
    name: 'mesh hit', question: 'Where is that, and which way does it face?',
    reads: ['mesh_drawn'], writes: ['mesh_hit'], uses: ['mesh shapes'], over: 'render', code: hit,
  });
}
