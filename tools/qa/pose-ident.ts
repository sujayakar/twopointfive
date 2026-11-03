// tools/qa/pose-ident.ts — THE POSING FUNCTIONS, the Rust module's against the TypeScript's (src/anim/skeleton.ts against rust/sim/src/skeleton.rs).
//
//   bun run tools/qa/pose-ident.ts [--n 20000] [--seed 1]
//   RUST_BREAK=1 bun run tools/qa/pose-ident.ts     the red path: the module is handed a rig with one node hung from the wrong parent — must FAIL
//
// Real poses (the game's own rig, its own clips sampled at random times) are blended (every weight, with and without a mask, into a third pose and in place),
// run forward (whole, and from a node down after a turn), turned (in a node's own frame, about the model's axes, set outright) and solved (the two-bone solve
// over every chain of three in the rig: targets in reach, out of reach and on the joint, with and without a pole and an end rotation, fresh and not). Each
// question is asked twice, RUST.poses off and on, and EVERY single of the pose and every double of the run after it is compared by its bits.
// The game's own maths are put in Math's place first (EXACT.on): the module answers only there. Exit 0 = every bit the same, 1 = not.
// Not game code: it reads. Nothing here needs a GPU, and nothing here has been seen on one.
import { ROOT, loadCharacter } from './headless.ts';
const args = process.argv.slice(2), flag = (k: string) => { const i = args.indexOf(k); return i >= 0 ? args[i + 1] : undefined; };
const N = Number(flag('--n') ?? 20000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.RUST_BREAK === '1';
const SK: any = await import(`${ROOT}/src/anim/skeleton.ts`), S: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`);
const { Pose, PoseFK, sampleClip, blendPose, restPose, rotateLocal, rotateModel, setModelRot, twoBoneIK } = SK;
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
const say = (what: string) => { differ++; if (!first) first = what; };
const poses = (what: string, a: any, b: any) => { const u = bitsOf(a), v = bitsOf(b); for (let i = 0; i < u.length; i++) if (u[i] !== v[i]) { say(`${what}: node ${Math.floor(i / 10)} value ${i % 10}: TypeScript ${a.data[i]} (${u[i].toString(16)}), the module ${b.data[i]} (${v[i].toString(16)})`); return; } };
const runs = (what: string, a: any, b: any) => { for (let i = 0; i < n; i++) { for (let k = 0; k < 4; k++) if (!same(a.rot[i][k], b.rot[i][k])) { say(`${what}: node ${i} rotation ${k}: TypeScript ${a.rot[i][k]}, the module ${b.rot[i][k]}`); return; }
  for (let k = 0; k < 3; k++) if (!same(a.pos[i][k], b.pos[i][k])) { say(`${what}: node ${i} origin ${k}: TypeScript ${a.pos[i][k]}, the module ${b.pos[i][k]}`); return; } } };
/** one question both ways: `ask` is run with the row off on the first pair and on on the second */
const both = (ask: (p: any, fk: any) => void, pT: any, fT: any, pR: any, fR: any) => { S.RUST.poses = false; ask(pT, fT); S.RUST.poses = true; ask(pR, fR); S.RUST.poses = false; asked++; };
S.RUST.poses = true; if (!S.RUST.poses) { console.log('pose-ident: RUST.poses does not come on'); process.exit(1); } S.RUST.poses = false;
const t0 = performance.now(); let inModule = 0;
for (let t = 0; t < N; t++) {
  const A = poseOf(), Bp = poseOf(), w = t % 7 === 0 ? pick([0, 1, -0.5, 1.5]) : rnd(), mask = pick(masks);
  // blend into a third pose, and in place
  const oT = new Pose(n), oR = new Pose(n); both(p => blendPose(A, Bp, w, mask, p), oT, null, oR, null); poses('blendPose', oT, oR);
  const iT = new Pose(n).copy(A), iR = new Pose(n).copy(A); both(p => blendPose(p, Bp, w, mask, p), iT, null, iR, null); poses('blendPose in place', iT, iR);
  // run forward
  const fT = new PoseFK(ch), fR = new PoseFK(ch); both((p, fk) => fk.run(p), oT, fT, oR, fR); runs('run', fT, fR);
  // turn a node three ways, each followed by a run from it down
  const i = Math.floor(rnd() * n), q = unit();
  both((p, fk) => { rotateLocal(p, i, q); fk.runFrom(p, i); }, oT, fT, oR, fR); poses('rotateLocal', oT, oR); runs('runFrom', fT, fR);
  both((p, fk) => { const pr = fk.parentRot(p, i); rotateModel(p, i, q, pr); fk.runFrom(p, i); }, oT, fT, oR, fR); poses('parentRot + rotateModel', oT, oR); runs('runFrom after rotateModel', fT, fR);
  const j = Math.floor(rnd() * n), q2 = unit();
  both((p, fk) => { const par = ch.nodes[j].parent; setModelRot(p, j, q2, par >= 0 ? fk.rot[par] : [0, 0, 0, 1]); fk.run(p); }, oT, fT, oR, fR); poses('setModelRot', oT, oR); runs('run after setModelRot', fT, fR);
  // the two-bone solve
  for (let k = 0; k < 3; k++) {
    const [a, b, c] = pick(chains), far = rnd(), at = fT.pos[c], R = far < 0.15 ? 5 : far < 0.3 ? 0 : 0.6;
    const target = [at[0] + (rnd() * 2 - 1) * R, at[1] + (rnd() * 2 - 1) * R, at[2] + (rnd() * 2 - 1) * R];
    const pole = rnd() < 0.7 ? unit().slice(0, 3) : null, poleW = pick([0, 0.5, 1, rnd()]), endRot = rnd() < 0.5 ? unit() : null, fresh = rnd() < 0.5;
    both((p, fk) => twoBoneIK(fk, p, a, b, c, target, pole, poleW, endRot, fresh), oT, fT, oR, fR); poses('twoBoneIK', oT, oR); runs('twoBoneIK', fT, fR);
  }
  if (t === 0) { inModule = SK.POSE_BENCH.asked;
    // (the red path: the rig is on the module's bench by now — its last node is hung from its first there, and nowhere else)
    if (BREAK) S.i32()[S.sim().pose_addr(5) / 4 + ch.order[ch.order.length - 1]] = ch.order[0]; }
}
const total = SK.POSE_BENCH.asked, ok = differ === 0 && total >= asked && inModule > 0;
console.log(`pose-ident: ${N} rounds on the game's rig (${n} nodes, ${clips.length} clips, ${chains.length} chains of three) · ${asked} questions asked both ways, ${total} calls answered by the module · ${differ} differ${first ? ` · first: ${first}` : ''} · ${Math.round(performance.now() - t0) / 1000} s`);
console.log(ok ? '=== pose-ident: PASS — every single of every pose and every double of every run the same ===' : '=== pose-ident: FAIL ===');
process.exit(ok ? 0 : 1);
