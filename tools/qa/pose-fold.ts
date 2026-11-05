// EVERY MAN'S POSE, FOLDED — a scripted mission played (tools/qa/playthrough.ts), and after every tick every pose every character keeps (node by node, its
// ten numbers in order, however the pose is stored), every skinning matrix, every bone and every ragdoll particle folded into one word.
// WHY: the run's bits (game/digest.ts rawBits) fold where a man is, not how he is posed, and tools/qa/pose-ident.ts asks only the functions the Rust module
// answers. A change to how poses are STORED or INDEXED that bent a finger or a skinning matrix would pass both. Two trees that print the same word here
// posed every man alike, picture and all — so this is the check for a change that must not move a pose: run it on the tree before and the tree after.
//   bun run tools/qa/pose-fold.ts <run> [root]        run = a play-through scenario and seed (hostage-rescue:1); root = another checkout to play (default: this one)
//   TPS_PROFILE=frozen …                              the frozen stand (default: the shipped stand with the senses on the CPU, as tools/qa/tickcost.ts plays)
//   QA_HOLD=poses=0 …                                 a check's own arm (tools/qa/headless.ts hold): a pose's questions answered by the TypeScript
//   POSEMATH_LATE=0|1 …                               the pose rule (anim/skeleton.ts POSEMATH) written on the standing game before its first tick
// It prints the word and exits 0; it judges nothing by itself. Not game code: it reads.
import { resolve } from 'node:path';
const [run, root] = process.argv.slice(2); const [scn, seed] = (run ?? '').split(':');
if (!scn || !seed) { console.error('pose-fold: bun run tools/qa/pose-fold.ts <scenario:seed> [root of another checkout]'); process.exit(2); }
const ROOT = resolve(root ?? resolve(import.meta.dir, '../..'));
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.PT_DIGEST ??= '1';
const { Game } = await import(`${ROOT}/src/game/game.ts`);
const PT = await import(`${ROOT}/tools/qa/playthrough.ts`);
const { men } = await import(`${ROOT}/src/game/seats.ts`);
const u = new Uint32Array(1), f = new Float32Array(u.buffer);
let h = 2166136261, ticks = 0, poses = 0;
const add = (v: number) => { f[0] = v; h = Math.imul(h ^ u[0], 16777619) >>> 0; };
// (a pose as ten columns, or — a checkout from before that — ten singles a node: the same numbers in the same order either way)
const pose = (p: any) => { const n = p.numNodes; poses++; if (p.cols) { const d = p.cols, N = p.rows; for (let i = 0; i < n; i++) for (let c = 0; c < 10; c++) add(d[c * N + i]); } else { const d = p.data; for (let i = 0; i < n * 10; i++) add(d[i]); } };
const man = (c: any) => { if (!c) return; for (const p of c.anim.poses.list) pose(p); const j = c.skel.jointData; for (let i = 0; i < j.length; i++) add(j[i]); for (const k of Object.keys(c.bones).sort()) { const b = c.bones[k]; if (b) for (const x of b) add(x); } if (c.ragdoll) for (const x of c.ragdoll.x) add(x); };
const update = Game.prototype.update;
const LATE = process.env.POSEMATH_LATE, SKEL: any = await import(`${ROOT}/src/anim/skeleton.ts`);   // (POSEMATH_LATE=0|1: the pose rule written on the standing game before its first tick — tools/qa/posemath-check.ts `late`)
Game.prototype.update = function (this: any, ...a: unknown[]) { if (ticks === 0 && LATE !== undefined && SKEL.POSEMATH) SKEL.POSEMATH.singles = LATE === '1'; const r = update.apply(this, a as any); for (const pl of men(this)) man(pl.char); for (const g of this.guards) man(g.char); for (const c of this.civilians) man(c.char); ticks++; return r; };
const r = await (PT as any).play(scn, Number(seed), 300, false, {});
console.log(`${run} ${process.env.TPS_PROFILE}${process.env.QA_HOLD ? ` held ${process.env.QA_HOLD}` : ''}: ${ticks} ticks, end ${r.end}, ${poses} poses folded, word ${h.toString(16)}`);
process.exit(0);
