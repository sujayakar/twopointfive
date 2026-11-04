// tools/qa/pose-ident.ts — THE POSING FUNCTIONS, the Rust module's against the TypeScript's (src/anim/skeleton.ts against rust/sim/src/skeleton.rs).
//
//   bun run tools/qa/pose-ident.ts [--n 20000] [--seed 1]
//   RUST_BREAK=1 bun run tools/qa/pose-ident.ts     the red path, must FAIL, and for EVERY KIND of question by a break of its own: the module is handed a
//                                                   rig with one node hung from the wrong parent (the run, the solve), a turn's first number, a
//                                                   solve's target, a blend's weight and a sample's time a hair off, and the plain sampler kept here
//                                                   is asked a hair off (the lines)
//
// Real poses (the game's own rig, its own clips sampled at random times) are blended (every weight, with and without a mask, into a third pose and in place),
// run forward (whole, and from a node down after a turn), turned (in a node's own frame, about the model's axes, set outright) and solved (the two-bone solve
// over every chain of three in the rig: targets in reach, out of reach and on the joint, with and without a pole and an end rotation, fresh and not). Each
// question is asked twice, RUST.poses off and on, and EVERY single of the pose and every double of the run after it is compared by its bits.
// A clip is sampled three ways (the TypeScript, the module, the definition kept here: every track searched), and the module's table of clips is
// written twice, a single off, half and to the full. The module's posing exports are counted at its door: a pose at home is asked where it lives.
// The game's own maths are put in Math's place first (EXACT.on): the module answers only there. Exit 0 = every bit the same · 1 = not · 3 = under the
// break word a kind of question stayed green (tools/qa/rust-check.ts reads that as the poses' row NOT red).
// Not game code: it reads. Nothing here needs a GPU, and nothing here has been seen on one.
import { ROOT, loadCharacter } from './headless.ts';
const args = process.argv.slice(2), flag = (k: string) => { const i = args.indexOf(k); return i >= 0 ? args[i + 1] : undefined; };
const N = Number(flag('--n') ?? 20000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.RUST_BREAK === '1';
const SK: any = await import(`${ROOT}/src/anim/skeleton.ts`), S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`);
const { Pose, PoseFK, PoseSet, poseSetIn, sampleClip, clipHeld, blendPose, restPose, rotateLocal, rotateModel, setModelRot, twoBoneIK } = SK;
const ST: any = await import(`${ROOT}/src/sim/state.ts`);
const V: any = await import(`${ROOT}/src/math/vec.ts`);
const why = S.simWhyNot(); if (why) { console.log(`pose-ident: the module does not run here (${why})`); process.exit(1); }
X.EXACT.on = true;
const ch = await loadCharacter(), n = ch.nodes.length, clips = [...ch.clips.values()];
// (a die of the tool's own: xorshift32)
let s = (SEED * 2654435761) >>> 0 || 1; const rnd = () => { s ^= s << 13; s >>>= 0; s ^= s >>> 17; s ^= s << 5; s >>>= 0; return s / 4294967296; };
const pick = <T>(a: T[]): T => a[Math.floor(rnd() * a.length)];
const unit = (): number[] => { let q: number[], l: number; do { q = [rnd() * 2 - 1, rnd() * 2 - 1, rnd() * 2 - 1, rnd() * 2 - 1]; l = Math.hypot(q[0], q[1], q[2], q[3]); } while (l < 0.1); return q.map(v => v / l); };
const chains: [number, number, number][] = []; for (let c = 0; c < n; c++) { const b = ch.nodes[c].parent; if (b < 0) continue; const a = ch.nodes[b].parent; if (a >= 0) chains.push([a, b, c]); }
const masks: (Float32Array | null)[] = [null, new Float32Array(n).map(() => rnd()), new Float32Array(n).map(() => rnd() < 0.5 ? 0 : 1), new Float32Array(n).map(() => rnd() * 1.4 - 0.2)];
const poseOf = () => { const p = restPose(ch, new Pose(n)); sampleClip(pick(clips), rnd() * 4, rnd() < 0.5, p); return p; };
const bitsOf = (p: any) => new Uint32Array(p.data.buffer, p.data.byteOffset, p.data.length);
const F = new Float64Array(1), U = new Uint32Array(F.buffer);
const same = (a: number, b: number) => { F[0] = a; const l = U[0], h = U[1]; F[0] = b; return U[0] === l && U[1] === h; };
let asked = 0, differ = 0, first = '';
// (the answers that differed, by the KIND of question: the red path must turn every kind red — one it leaves green is a kernel no check holds)
const KINDS = ['sample', 'lines', 'blend', 'run', 'turn', 'after', 'ik'] as const; type Kind = typeof KINDS[number];
// (a question's kind is read off its label, and a label of no kind is an error: nothing falls into a bucket by default. `run` is the whole run alone and
//  `turn` the turned pose alone, so that under the break word each is red by its own break; the run made from a turn in the same question is `after`,
//  red by either)
const KIND_OF: [string, Kind][] = [['rotateLocal, then', 'after'], ['parentRot + rotateModel, then', 'after'], ['setModelRot, then', 'after'], ['lines', 'lines'], ['sampleClip', 'sample'], ['blendPose', 'blend'], ['twoBoneIK', 'ik'], ['run', 'run'], ['rotateLocal', 'turn'], ['parentRot + rotateModel', 'turn'], ['setModelRot', 'turn']];
const kindOf = (what: string): Kind => { for (const [k, v] of KIND_OF) if (what.startsWith(k)) return v; throw new Error(`pose-ident: a question of no kind: ${what}`); };
const by: Record<Kind, number> = { sample: 0, lines: 0, blend: 0, run: 0, turn: 0, after: 0, ik: 0 };
const say = (what: string) => { differ++; by[kindOf(what)]++; if (!first) first = what; };
// (…and the questions the module answered of a pose WHERE IT LIVES against those sent by the bench, counted at the module's own door, a call an export:
//  every at-home round asks each export of a kept pose a known number of times, and none of those may go by the bench)
const CALLS = ['pose_sample', 'pose_blend', 'pose_run', 'pose_parent_rot', 'pose_turn', 'pose_ik'] as const; type Call = typeof CALLS[number];
const inX = Object.fromEntries(CALLS.map(k => [k, 0])) as Record<Call, number>;
/** an at-home round's calls of a pose where it lives: a sample · a blend into a third pose and one in place · the run, a run from a node after each of two
 *  turns, the run after the third · the parent's rotation for the turn about the model's axes · three turns · three solves */
const ROUND: Record<Call, number> = { pose_sample: 1, pose_blend: 2, pose_run: 4, pose_parent_rot: 1, pose_turn: 3, pose_ik: 3 };
const poses = (what: string, a: any, b: any) => { const u = bitsOf(a), v = bitsOf(b); for (let i = 0; i < u.length; i++) if (u[i] !== v[i]) { say(`${what}: node ${Math.floor(i / 10)} value ${i % 10}: TypeScript ${a.data[i]} (${u[i].toString(16)}), the module ${b.data[i]} (${v[i].toString(16)})`); return; } };
const runs = (what: string, a: any, b: any) => { for (let i = 0; i < n; i++) { for (let k = 0; k < 4; k++) if (!same(a.rot[i][k], b.rot[i][k])) { say(`${what}: node ${i} rotation ${k}: TypeScript ${a.rot[i][k]}, the module ${b.rot[i][k]}`); return; }
  for (let k = 0; k < 3; k++) if (!same(a.pos[i][k], b.pos[i][k])) { say(`${what}: node ${i} origin ${k}: TypeScript ${a.pos[i][k]}, the module ${b.pos[i][k]}`); return; } } };
// SAMPLING, AS IT IS DEFINED: every track searched for its own place among its own keys. Both languages find a place once for each set of key times a clip's
// tracks share (anim/skeleton.ts sideOf, rust/sim/src/clips.rs `end`); this is what they must both still answer, kept here in the plainest words. (Under the
// break word it is asked a hair off, so that the comparison itself is seen to fail.)
const plain = (clip: any, t: number, loop: boolean, out: any) => {
  if (BREAK) t += 1e-4;
  const dur = clip.duration; if (loop) { t = t % dur; if (t < 0) t += dur; } else t = Math.min(Math.max(t, 0), dur);
  const d = out.data;
  for (const tr of clip.tracks) {
    const times = tr.times, vals = tr.values, k = times.length, comps = tr.path === 1 ? 4 : 3, o = tr.node * 10 + (tr.path === 0 ? 0 : tr.path === 1 ? 3 : 7);
    if (k === 1 || t <= times[0]) { for (let c = 0; c < comps; c++) d[o + c] = vals[c]; continue; }
    if (t >= times[k - 1]) { for (let c = 0; c < comps; c++) d[o + c] = vals[(k - 1) * comps + c]; continue; }
    let lo = 0, hi = k - 1; while (hi - lo > 1) { const mid = (lo + hi) >> 1; if (times[mid] <= t) lo = mid; else hi = mid; }
    const a = lo * comps, b = hi * comps;
    if (tr.step) { for (let c = 0; c < comps; c++) d[o + c] = vals[a + c]; continue; }
    const f = (t - times[lo]) / Math.max(1e-6, times[hi] - times[lo]);
    if (comps === 3) { for (let c = 0; c < 3; c++) d[o + c] = vals[a + c] + (vals[b + c] - vals[a + c]) * f; continue; }
    let bx = vals[b], by = vals[b + 1], bz = vals[b + 2], bw = vals[b + 3]; const ax = vals[a], ay = vals[a + 1], az = vals[a + 2], aw = vals[a + 3];
    if (ax * bx + ay * by + az * bz + aw * bw < 0) { bx = -bx; by = -by; bz = -bz; bw = -bw; }
    const x = ax + (bx - ax) * f, y = ay + (by - ay) * f, z = az + (bz - az) * f, w = aw + (bw - aw) * f, l = V.hyp4(x, y, z, w) || 1;
    d[o] = x / l; d[o + 1] = y / l; d[o + 2] = z / l; d[o + 3] = w / l;
  }
};
/** the TypeScript's sample `got` (of the pose `from`) against the definition's */
const lines = (what: string, clip: any, t: number, loop: boolean, from: any, got: any) => { const want = new Pose(n).copy(from); plain(clip, t, loop, want); const u = bitsOf(got), v = bitsOf(want);
  for (let i = 0; i < u.length; i++) if (u[i] !== v[i]) { say(`lines, ${what}: node ${Math.floor(i / 10)} value ${i % 10}: sampleClip ${got.data[i]} (${u[i].toString(16)}), every track searched ${want.data[i]} (${v[i].toString(16)})`); return; } };
/** one question both ways: `ask` is run with the row off on the first pair and on on the second */
const both = (ask: (p: any, fk: any, a: any, b: any) => void, pT: any, fT: any, pR: any, fR: any) => { S.RUST.poses = false; ask(pT, fT, A, Bp); S.RUST.poses = true; ask(pR, fR, AR, BR); S.RUST.poses = false; asked++; };
// EVERY OTHER ROUND the module's side of a question is asked of poses that LIVE IN A GAME'S BLOCK (src/sim/state.ts; anim/skeleton.ts PoseSet): no copy onto
// the bench, the pose read and written where it is. Four poses of a set at home, used again each such round.
const home = new PoseSet(), block = new ST.SimState(); block.bind(); poseSetIn(home, block); const hA = home.make(n), hB = home.make(n), hO = home.make(n), hI = home.make(n);
let A: any, Bp: any, AR: any, BR: any;
S.RUST.poses = true; if (!S.RUST.poses) { console.log('pose-ident: RUST.poses does not come on'); process.exit(1); } S.RUST.poses = false;
const t0 = performance.now(); let inModule = 0;
for (let t = 0; t < N; t++) {
  const there = t % 2 === 1; A = poseOf(); Bp = poseOf(); AR = there ? hA.copy(A) : A; BR = there ? hB.copy(Bp) : Bp; const w = t % 7 === 0 ? pick([0, 1, -0.5, 1.5]) : rnd(), mask = pick(masks);
  // sample a clip into a pose that already holds another (only the clip's nodes are written): in the clip, past either end, on a key's own time, looped or held
  { const clip = pick(clips), tr = pick(clip.tracks) as any, r = rnd(), tt = r < 0.5 ? rnd() * clip.duration : r < 0.7 ? (rnd() * 3 - 1) * clip.duration : r < 0.9 ? tr.times[Math.floor(rnd() * tr.times.length)] : pick([0, -0, clip.duration, -clip.duration, 1e-9, 1e9]), lp = rnd() < 0.5;
    const sT = new Pose(n).copy(A), sR = (there ? hI : new Pose(n)).copy(A); both(p => sampleClip(clip, tt, lp, p), sT, null, sR, null); poses('sampleClip', sT, sR); lines(clip.name, clip, tt, lp, A, sT); }
  // blend into a third pose, and in place
  const oT = new Pose(n), oR = there ? hO : new Pose(n); if (there) oR.data.fill(0); both((p, _f, a, b) => blendPose(a, b, w, mask, p), oT, null, oR, null); poses('blendPose', oT, oR);
  const iT = new Pose(n).copy(A), iR = (there ? hI : new Pose(n)).copy(A); both((p, _f, _a, b) => blendPose(p, b, w, mask, p), iT, null, iR, null); poses('blendPose in place', iT, iR);
  // run forward
  const fT = new PoseFK(ch), fR = new PoseFK(ch);
  // (under the break word every question starts from the TypeScript's pose and run: a kind is red by its OWN answer, never by what an earlier one left)
  const even = () => { if (!BREAK) return; oR.copy(oT); for (let k = 0; k < n; k++) { for (let c = 0; c < 4; c++) fR.rot[k][c] = fT.rot[k][c]; for (let c = 0; c < 3; c++) fR.pos[k][c] = fT.pos[k][c]; } };
  even(); both((p, fk) => fk.run(p), oT, fT, oR, fR); runs('run', fT, fR); even();
  // turn a node three ways, each followed by a run from it down
  const i = Math.floor(rnd() * n), q = unit();
  both((p, fk) => { rotateLocal(p, i, q); fk.runFrom(p, i); }, oT, fT, oR, fR); poses('rotateLocal', oT, oR); runs('rotateLocal, then runFrom', fT, fR); even();
  both((p, fk) => { const pr = fk.parentRot(p, i); rotateModel(p, i, q, pr); fk.runFrom(p, i); }, oT, fT, oR, fR); poses('parentRot + rotateModel', oT, oR); runs('parentRot + rotateModel, then runFrom', fT, fR); even();
  const j = Math.floor(rnd() * n), q2 = unit();
  both((p, fk) => { const par = ch.nodes[j].parent; setModelRot(p, j, q2, par >= 0 ? fk.rot[par] : [0, 0, 0, 1]); fk.run(p); }, oT, fT, oR, fR); poses('setModelRot', oT, oR); runs('setModelRot, then run', fT, fR); even();
  // the two-bone solve
  for (let k = 0; k < 3; k++) {
    const [a, b, c] = pick(chains), far = rnd(), at = fT.pos[c], R = far < 0.15 ? 5 : far < 0.3 ? 0 : 0.6;
    const target = [at[0] + (rnd() * 2 - 1) * R, at[1] + (rnd() * 2 - 1) * R, at[2] + (rnd() * 2 - 1) * R];
    const pole = rnd() < 0.7 ? unit().slice(0, 3) : null, poleW = pick([0, 0.5, 1, rnd()]), endRot = rnd() < 0.5 ? unit() : null, fresh = rnd() < 0.5;
    both((p, fk) => twoBoneIK(fk, p, a, b, c, target, pole, poleW, endRot, fresh), oT, fT, oR, fR); poses('twoBoneIK', oT, oR); runs('twoBoneIK', fT, fR); even();
  }
  if (t === 0) { inModule = SK.POSE_BENCH.asked;
    // THE MODULE'S DOOR, from the second round on: every posing export counted where the pose was at home (g >= 0). And under the break word EACH KIND HAS A
    // BREAK OF ITS OWN there: the rig on the bench has its last node hung from its first (a run; the solve reads the rig too), a turn's first number and a
    // solve's target are a hair off on the bench, a blend's weight and a sample's time are a hair off in the call — the module's side alone, every one.
    if (BREAK) S.i32()[S.sim().pose_addr(5) / 4 + ch.order[ch.order.length - 1]] = ch.order[0];
    const x = SK.POSE_BENCH.x, door: any = { ...x }, hair = () => { S.f64()[SK.POSE_BENCH.args] += 1e-6; };
    for (const k of CALLS) door[k] = (g: number, ...r: number[]) => { if (g >= 0) inX[k]++;
      if (BREAK) { if (k === 'pose_blend') r[2] += 1e-9; else if (k === 'pose_sample') r[3] += 1e-4; else if (k === 'pose_turn' || k === 'pose_ik') hair(); }
      return x[k](g, ...r); };
    SK.POSE_BENCH.x = door; }
}
const half = Math.floor(N / 2), notIn = CALLS.filter(k => inX[k] !== ROUND[k] * half);
// THE CLIPS' COPY (rust/sim/src/clips.rs), read back by its answers: every clip sampled both ways at EVERY KEY'S OWN TIME of every track — there a track's
// answer is that key's values to the bit, so a single of the copy that is not the file's shows — and each must be one the module holds.
// …and four clips made here for what the game's own never do: more sets of key times than a sample keeps its place on, sets of one length that differ in one
// single (and by the sign of a zero), two tracks that write the same values of a node (the later one stands), a track of one key among longer ones
const f32s = (...v: number[]) => new Float32Array(v), ramp = (k: number, c: number, s: number) => Float32Array.from({ length: k * c }, (_, i) => Math.sin(i * 0.7 + s));
const units = (k: number, s: number) => { const v = ramp(k, 4, s); for (let i = 0; i < k; i++) { const l = Math.hypot(v[i * 4], v[i * 4 + 1], v[i * 4 + 2], v[i * 4 + 3]) || 1; for (let c = 0; c < 4; c++) v[i * 4 + c] /= l; } return v; };
const made = [
  { name: 'made: twelve sets of times', duration: 2, tracks: Array.from({ length: 12 }, (_, i) => ({ node: 1 + i, path: i % 3, step: i % 5 === 4, times: f32s(0, 0.5 + i * 0.01, 1, 2), values: i % 3 === 1 ? units(4, i) : ramp(4, 3, i) })) },
  { name: 'made: sets that differ in one single', duration: 1, tracks: [0, 1, 2, 3, 4].map(i => ({ node: 2 + (i >> 1), path: i % 3, step: false, times: i === 4 ? f32s(-0, 0.25, 1) : f32s(0, i === 2 ? 0.25 + 3e-8 : 0.25, i === 3 ? 0.999 : 1), values: i % 3 === 1 ? units(3, i) : ramp(3, 3, i) })) },
  { name: 'made: the same values written twice', duration: 1, tracks: [{ node: 3, path: 0, step: false, times: f32s(0, 1), values: ramp(2, 3, 1) }, { node: 3, path: 0, step: false, times: f32s(0, 0.5, 1), values: ramp(3, 3, 2) }, { node: 3, path: 1, step: true, times: f32s(0, 1), values: units(2, 3) }, { node: 3, path: 1, step: false, times: f32s(0, 0.5, 1), values: units(3, 4) }] },
  { name: 'made: one key among many', duration: 1, tracks: [{ node: 4, path: 2, step: false, times: f32s(0.5), values: f32s(1, 2, 3) }, { node: 4, path: 0, step: false, times: f32s(0, 0.5, 1), values: ramp(3, 3, 5) }, { node: 5, path: 2, step: false, times: f32s(0.5), values: f32s(3, 2, 1) }] }];
for (const clip of made) for (let i = 0; i < 400; i++) { const lp = i % 2 === 1, tt = i < 8 ? [0, -0, 0.25, 0.5, 1, 2, 0.25 + 3e-8, 0.999][i] : (rnd() * 1.4 - 0.2) * clip.duration, pT = new Pose(n).copy(A), pR = new Pose(n).copy(A);
  both(p => sampleClip(clip, tt, lp, p), pT, null, pR, null); poses(`sampleClip, ${clip.name} at ${tt}`, pT, pR); lines(clip.name, clip, tt, lp, A, pT); }
let keys = 0, held = 0;
for (const clip of made) if (clipHeld(clip) < 0) say(`sampleClip: the module does not hold ${clip.name}`);
for (const clip of clips) {
  const pT = restPose(ch, new Pose(n)), pR = restPose(ch, new Pose(n)), seen = new Set<number>();
  for (const tr of clip.tracks as any[]) for (const tt of tr.times) { if (seen.has(tt)) continue; seen.add(tt); A = pT; Bp = pT; AR = pR; BR = pR; const was = new Pose(n).copy(pT); both(p => sampleClip(clip, tt, false, p), pT, null, pR, null); poses(`sampleClip, every key (${clip.name} at ${tt})`, pT, pR); lines(`every key (${clip.name} at ${tt})`, clip, tt, false, was, pT); keys++; }
  if (clipHeld(clip) >= 0) held++;
}
// …and clips the module must refuse are sampled here, the same pose either way: no track, a track with no key, values that are not its keys', a node no pose has
const refused = [{ name: 'none', duration: 1, tracks: [] }, { name: 'no key', duration: 1, tracks: [{ node: 1, path: 0, times: new Float32Array(0), values: new Float32Array(0), step: false }] },
  { name: 'short', duration: 1, tracks: [{ node: 1, path: 1, times: new Float32Array([0, 1]), values: new Float32Array([0, 0, 0, 1, 0, 0, 1]), step: false }] },
  { name: 'far node', duration: 1, tracks: [{ node: 4000, path: 0, times: new Float32Array([0, 1]), values: new Float32Array([1, 2, 3, 4, 5, 6]), step: false }] }];
let refusedOk = true;
for (const clip of refused) { const pT = restPose(ch, new Pose(n)), pR = restPose(ch, new Pose(n)); A = pT; Bp = pT; AR = pR; BR = pR; both(p => sampleClip(clip, 0.5, false, p), pT, null, pR, null); poses(`sampleClip, a clip the module refuses (${clip.name})`, pT, pR); if (clipHeld(clip) >= 0) refusedOk = false; }
// …and a clip the module TAKES but that keys a node this pose has not (the table holds poses of up to 128 nodes; the rig has fewer): the host samples it
// itself, the same pose (what it writes past the pose's end is nowhere, in both)
const past = { name: 'made: a node past the pose', duration: 1, tracks: [{ node: 2, path: 0, step: false, times: f32s(0, 1), values: f32s(1, 2, 3, 4, 5, 6) }, { node: n + 3, path: 0, step: false, times: f32s(0, 1), values: f32s(6, 5, 4, 3, 2, 1) }] };
let pastOk = n + 3 < 128;
{ const pT = restPose(ch, new Pose(n)), pR = restPose(ch, new Pose(n)), before = SK.POSE_BENCH.asked; A = pT; Bp = pT; AR = pR; BR = pR; both(p => sampleClip(past, 0.5, false, p), pT, null, pR, null); poses(`sampleClip, ${past.name}`, pT, pR); if (clipHeld(past) < 0 || SK.POSE_BENCH.asked !== before) pastOk = false; }
// A CLIP WRITTEN TWICE IS ONE CLIP IN THE MODULE (clips.rs `end`): the character file's clips made again as new objects, as a file loaded a second time makes
// them, get the numbers the first have; one that differs from a held clip in ONE single (the last value of its last track) is another clip
const copyOf = (c: any) => ({ ...c, tracks: c.tracks.map((t: any) => ({ ...t, times: t.times.slice(), values: t.values.slice() })) });
const REST = restPose(ch, new Pose(n));
const sampled = (clip: any, tt: number, what: string) => { const pT = new Pose(n).copy(REST), pR = new Pose(n).copy(REST); A = pT; Bp = pT; AR = pR; BR = pR; both(p => sampleClip(clip, tt, false, p), pT, null, pR, null); poses(`sampleClip, ${what} (${clip.name})`, pT, pR); lines(`${what} (${clip.name})`, clip, tt, false, REST, pT); };
let twice = 0;
for (const clip of clips) { const c2 = copyOf(clip); sampled(c2, clip.duration * 0.37, 'a clip written twice'); if (clipHeld(c2) >= 0 && clipHeld(c2) === clipHeld(clip)) twice++; }
const near = copyOf(clips[0]); { const v = near.tracks[near.tracks.length - 1].values; v[v.length - 1] += 0.25; } sampled(near, clips[0].duration * 0.37, 'a clip one single off a held one');
const nearOk = clipHeld(near) >= 0 && clipHeld(near) !== clipHeld(clips[0]);
// THE TABLE FULL, each of the three ways it fills: clips of many tracks made here until the tracks run out, long ones until the singles do, short ones
// until the clips do. Each is sampled the same either way (the ones it refuses, by the TypeScript). And a full table still knows what it holds (it keeps
// room for its largest clip to be written again): the character file's clips written AGAIN after each filling are the same numbers, and a held clip
// still answers from the module. Before any of it, a clip left HALF WRITTEN a hundred times over (begun, a long track asked for, never ended) must cost
// the table nothing: each is dropped by the next one begun. (The wide ones have fewer tracks than the rig's widest clip, so that it is the room KEPT
// that lets that clip be written again, and one of five thousand tracks is turned away at the door: `clip_begin`.)
const wide = (i: number, tracks = 150) => ({ name: `made: wide filler ${i}`, duration: 1, tracks: Array.from({ length: tracks }, (_, k) => ({ node: 1 + k % 20, path: (k % 3) as 0 | 1 | 2, step: false, times: f32s(0.5), values: k % 3 === 1 ? units(1, i + k) : f32s(i, k, i + k) })) });
const long = (i: number) => { const k = 6000; return { name: `made: long filler ${i}`, duration: 1, tracks: [{ node: 1 + i % 20, path: 1, step: false, times: Float32Array.from({ length: k }, (_, j) => j / (k - 1)), values: units(k, i) }] }; };
let fullWide = 0, fullLong = 0, fullShort = 0, again = 0;
const writeAgain = () => { for (const clip of clips) { const c3 = copyOf(clip); sampled(c3, clip.duration * 0.61, 'a clip written again, the table full'); if (clipHeld(c3) >= 0 && clipHeld(c3) === clipHeld(clip)) again++; } };
{ const x = S.sim(); for (let i = 0; i < 100; i++) { if (!x.clip_begin(1, 1)) say('sampleClip: the table would not begin a clip after one left half written'); x.clip_track(1, 1, 0, 6000); } }
for (let i = 0; i < 250; i++) { const clip = wide(i); sampled(clip, 0.3, 'the tracks filling'); if (clipHeld(clip) < 0) fullWide++; }
{ const clip = wide(999, 5000); sampled(clip, 0.3, 'more tracks than there is room to write'); if (clipHeld(clip) >= 0) say('sampleClip: a clip of five thousand tracks was taken by a table with no room to write it'); fullWide++; }
writeAgain();
for (let i = 0; i < 40; i++) { const clip = long(i); sampled(clip, 0.3, 'the singles filling'); if (clipHeld(clip) < 0) fullLong++; }
writeAgain();
for (let i = 0; i < 400; i++) { const clip = { name: `made: filler ${i}`, duration: 1, tracks: [{ node: 1 + i % 20, path: 0, step: false, times: f32s(0, 1), values: f32s(i, i + 1, i + 2, i * 2, 3, 4) }] }; sampled(clip, 0.3, 'the clips filling'); if (clipHeld(clip) < 0) fullShort++; }
writeAgain();
{ const before = SK.POSE_BENCH.asked; sampled(clips[0], clips[0].duration * 0.5, 'a held clip, the table full'); if (SK.POSE_BENCH.asked !== before + 1) say('sampleClip: a held clip was not asked of the module once the table was full'); }
const filled = fullWide > 1 && fullWide < 250 && fullLong > 0 && fullLong < 40 && fullShort > 0 && fullShort < 400, full = fullWide + fullLong + fullShort;
const clipsOk = twice === clips.length && nearOk && pastOk && filled && again === 3 * clips.length;
const total = SK.POSE_BENCH.asked, inPlace = SK.POSE_BENCH.inPlace, ok = differ === 0 && clipsOk && total >= asked - refused.length - 1 - full && inModule > 0 && notIn.length === 0 && hO.home !== null && held === clips.length && refusedOk;
const blind = BREAK ? KINDS.filter(k => !by[k]) : [];
console.log(`pose-ident: ${N} rounds on the game's rig (${n} nodes, ${clips.length} clips, ${chains.length} chains of three) · ${asked} questions asked both ways, ${total} calls answered by the module, ${inPlace} of them of a pose where it lives in a game's block · ${differ} differ${first ? ` · first: ${first}` : ''} · the clips: ${held} of ${clips.length} held by the module, ${keys} keys' own times sampled both ways, ${refusedOk ? `${refused.length} clips it must refuse sampled here` : 'A CLIP IT MUST REFUSE WAS TAKEN'}, ${twice === clips.length ? `${twice} written a second time are the same ${twice} there` : `ONLY ${twice} OF ${clips.length} WRITTEN TWICE ARE ONE CLIP THERE`}, ${nearOk ? 'one a single off is another' : 'A CLIP ONE SINGLE OFF A HELD ONE IS NOT ANOTHER'}, ${pastOk ? 'one keying a node past the pose taken and sampled here' : 'A CLIP KEYING A NODE PAST THE POSE WAS NOT TAKEN, OR WAS ASKED OF THE MODULE'}, ${filled ? `the table filled by its tracks (${fullWide} more sampled here), its singles (${fullLong}) and its clips (${fullShort})` : `THE TABLE DID NOT FILL AS MEANT (${fullWide} of 251 wide, ${fullLong} of 40 long and ${fullShort} of 400 short refused)`}, ${again === 3 * clips.length ? 'and still knew every clip it held' : `AND KNEW ONLY ${again} OF ${3 * clips.length} CLIPS WRITTEN AGAIN ONCE FULL`}${notIn.length ? ` · AT HOME, NOT ASKED WHERE THE POSE LIVES AS OFTEN AS MEANT: ${notIn.map(k => `${k} ${inX[k]} of ${ROUND[k] * half}`).join(', ')}` : ''} · ${Math.round(performance.now() - t0) / 1000} s`);
console.log('POSE-IDENT ' + JSON.stringify({ asked, differ, by, clips: { held, of: clips.length, keys, twice, near: nearOk, past: pastOk, full, again }, inPlace: inX }));   // (what tools/qa/rust-check.ts reads: under RUST_BREAK=1 the poses' own answers must have differed)
console.log(ok ? '=== pose-ident: PASS — every single of every pose and every double of every run the same ===' : `=== pose-ident: FAIL${BREAK ? blind.length ? ` — AND THE RED PATH IS NOT RED FOR ${blind.join(', ')}` : ' — every kind of question differed somewhere, as the red path must' : ''} ===`);
process.exit(blind.length ? 3 : ok ? 0 : 1);
