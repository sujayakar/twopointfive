// The plans of the race, written from the lists below: bun run tools/kept/plans.ts → tools/kept/*.json
import { writeFileSync } from 'node:fs';
const bits = (c: number) => c < 0.28 ? 20 : c < 0.4 ? 19 : c < 0.9 ? 18 : 17, P = (c: number, more = '') => `patch cell=${c} coarse=0 spread=1 bits=${bits(c)}${more ? ' ' + more : ''}`;
const sizes = [0.16, 0.24, 0.32, 0.48, 0.64, 0.96, 1.28, 1.92, 2.56];
const ways = ['none', ...[1, 2, 3, 4, 6].map(n => `paths n=${n}`), 'today', 'patch',
  ...sizes.map(c => P(c)), ...[0.32, 0.64, 1.28, 2.56].map(c => P(c, 'direct=0')), ...[0.32, 0.64, 1.28, 2.56].map(c => P(c, 'roam=1')), ...[0.64, 1.28, 2.56].map(c => P(c, 'roam=1 flat=1')),
  ...[0.5, 1, 2].map(w => `patch levels=4 cell=0.32 flat=1 roam=1 spread=1 bits=19 widen=${w}`), 'patch levels=4 cell=0.32 spread=1 bits=19 widen=1', 'patch levels=4 cell=0.32 flat=1 roam=1 spread=1 bits=19 widen=1 direct=0',
  'patch lamp=0', 'patch joins=0', 'patch tries=1', 'patch keep=24', 'patch keep=400', 'patch rate=1', 'patch rate=2', 'patch rate=32', P(0.32, 'warm=0'),
  ...[0.16, 0.32, 0.64, 1.28, 2.56].map(c => `faces cell=${c}`), ...[0.32, 0.64, 1.28].map(c => `faces cell=${c} direct=0`), 'faces rate=1', 'faces rate=2', 'faces rate=32', 'faces joins=0',
  'probes step=0.5 rays=16', 'probes step=0.75', 'probes step=1', 'probes step=1.5', 'probes step=2', 'probes step=1 vis=0', 'probes step=1 vis=1', 'probes step=1 rays=64', 'probes step=1 rate=2', 'probes step=1 rate=32',
  'probes oct=0 step=0.5 rays=16', 'probes oct=0 step=0.75', 'probes oct=0 step=1', 'probes oct=0 step=2', 'probes oct=0 step=1 vis=2',
  'near cell=0.25 bits=18', 'near cell=0.5', 'near cell=1', 'near cell=2', 'near cell=0.5 rays=16', 'near cell=0.5 spread=1 bits=18',
  'surfels cell=0.3 bits=18', 'surfels cell=0.45', 'surfels cell=0.7', 'surfels cell=1',
  'net bits=12', 'net bits=14', 'net bits=16', 'net batch=2048', 'net batch=32768'];
const room = { w: 1280, h: 720, tick: 600, dist: 34, cut: 2.55, look: [0, 0.6, 0], doors: 0.8, lamp: 'atrium' }, dark = { atrium: 0.1, orange: 0, green: 0.06, blue: 0.25, gallery: 0, studio: 0, lounge: 0.12, landing: 0.08, terrace: 0.5, pool: 0.35 };
const V = { day: { ...room, name: 'day', hour: 15 }, night: { ...room, name: 'night', hour: 21.5 }, dark: { ...room, name: 'dark', hour: 23, groups: dark, lamp: 'blue' }, leak: { ...room, name: 'leak', hour: 23, moon: 0, doors: 0, groups: { atrium: 0, orange: 0, green: 0, blue: 1, gallery: 0, studio: 0, lounge: 0, landing: 0, terrace: 0, pool: 0 } }, street: { name: 'street', w: 1280, h: 720, dist: 27, look: [0.5, 1, -1.0], yaw: 0.5, pitch: 0.72, hour: 22.2, cut: 2.5, tick: 300, lamp: 'store' } };
const frames = { truth: 320, rest: 96, wait: 2500, settle: 24, bands: 64 }, rest = { frames, rest: ways, cost: ways, rounds: 3 };
const moved = ['today', 'patch', 'patch rate=1', 'patch rate=2', 'patch rate=32', 'patch keep=24', 'patch keep=24 rate=1', P(0.64), P(1.28), P(0.32, 'direct=0'), 'faces', 'faces rate=1', 'faces rate=2', 'faces cell=0.64', 'probes step=1', 'probes step=1 rate=2', 'probes step=1 rate=32', 'probes step=2', 'probes oct=0 step=1', 'near cell=0.5', 'surfels cell=0.45', 'net', 'net batch=32768', 'net ema=0.9 lr=0.02'];
const some = ['paths n=24 exact=1', 'today', 'patch', P(0.64), 'faces', 'probes step=1', 'near cell=0.5', 'surfels cell=0.45', 'net'];
// the second race: coarse cells made as good as they can be (an answer where nothing is joined; texels that look from many places), and the direct light kept by those that were charged for asking it
const S = (c: number, more = '') => P(c, 'soft=1' + (more ? ' ' + more : '')), coarse = [0.64, 1.28, 2.56], F = (c: number, more = '') => `faces cell=${c}${c > 0.4 ? ' many=8' : ''}${more ? ' ' + more : ''}`;
const ways2 = ['none', 'today', 'patch', 'faces', 'patch soft=1', 'patch soft=1 tries=4', 'patch soft=1 tries=4 rate=2', ...sizes.map(c => S(c)), ...[0.32, ...coarse].map(c => S(c, 'tries=4')), ...[0.32, ...coarse].map(c => S(c, 'tries=4 direct=0')), ...[0.32, ...coarse].map(c => S(c, 'tries=4 roam=1 flat=1')), ...coarse.map(c => S(c, 'tries=4 roam=1 flat=1 direct=0')),
  ...[0.5, 1].flatMap(w => [`patch levels=3 cell=0.32 flat=1 roam=1 spread=1 soft=1 tries=4 bits=19 widen=${w}`, `patch levels=3 cell=0.32 flat=1 roam=1 spread=1 soft=1 tries=4 bits=19 widen=${w} direct=0`]),
  ...[0.48, 0.64, 0.96, 1.28, 2.56].map(c => F(c)), ...[0.64, 1.28, 2.56].map(c => F(c, 'direct=0')), 'faces keep=24', 'faces keep=48 rate=2', 'faces cell=0.48 many=8 joins=0',
  'probes step=0.5 rays=16 direct=1', 'probes step=0.75 direct=1', 'probes step=1 direct=1', 'probes step=2 direct=1', 'probes oct=0 step=0.5 rays=16 vis=2 direct=1', 'probes oct=0 step=1 vis=2 direct=1', 'probes step=1',
  'near cell=0.25 bits=18 direct=1', 'near cell=0.5 direct=1', 'near cell=1 direct=1', 'net direct=1', 'net direct=1 batch=32768', 'net direct=1 bits=16', 'net'];
const band2 = [...sizes.map(c => S(c, 'tries=4')), ...coarse.map(c => S(c, 'tries=4 roam=1 flat=1')), ...coarse.map(c => S(c, 'tries=4 roam=1 flat=1 direct=0')), ...[0.32, 0.64, 1.28, 2.56].map(c => F(c)), ...[0.64, 1.28, 2.56].map(c => F(c, 'direct=0'))];
const moved2 = ['today', 'patch', 'patch keep=24', 'faces', 'faces keep=24', 'faces rate=1', 'faces keep=48 rate=2', 'probes step=1 direct=1', 'near cell=0.5 direct=1', 'net direct=1', 'net direct=1 batch=32768'], settle2 = { lamp: moved2, cold: moved2 };
const write = (n: string, p: unknown) => writeFileSync(`tools/kept/${n}.json`, JSON.stringify(p));
write('rest1', { scene: 'rooftop', views: [V.night, V.day], ...rest, pictures: some }); write('rest2', { scene: 'rooftop', views: [V.dark, V.leak], ...rest, pictures: some }); write('rest3', { scene: 'street', views: [V.street], ...rest, pictures: some });
const band = [...sizes.map(c => P(c)), P(0.32, 'direct=0'), P(1.28, 'direct=0'), P(1.28, 'roam=1'), P(1.28, 'roam=1 flat=1'), 'faces', 'faces cell=1.28', 'probes step=1'], settle = { doors: moved, lamp: moved, cold: moved };
write('moved1', { scene: 'rooftop', frames, rest: [], cost: ['none', ...moved], rounds: 3, bands: band, views: [{ ...V.night, settle }, { ...V.day, settle: { ...settle, sun: moved } }, { ...V.dark, settle: { lamp: moved } }], full: some });
write('moved2', { scene: 'street', frames, rest: [], cost: ['none', ...moved], rounds: 3, bands: band, views: [{ ...V.street, settle: { lamp: moved, cold: moved } }], full: some });
const rest2 = { frames, rest: ways2, cost: ways2, rounds: 3 }; write('second1', { scene: 'rooftop', views: [{ ...V.night, settle: settle2 }, V.day], ...rest2, bands: band2 }); write('second2', { scene: 'rooftop', views: [V.leak, { ...V.dark, settle: { lamp: moved2 } }], ...rest2 }); write('second3', { scene: 'street', views: [V.street], ...rest2 });
console.log(ways2.length, 'ways in the second race');
console.log(ways.length, 'ways');
