// tools/qa/packside-check.ts — a long thing a man carries on his back stands at the side of the pack he wears, and moves with it
// (src/game/packside.ts PACKSIDE.on; rigProps.ts packSideBind / bigBoxes / sc20kFrame; cast.ts castParts; character.ts bigLayer, `bigSide`).
//
// Seen on the measuring pass over every worn and carried thing: Sam's slung rifle through his radio set, the hammer and the shotgun through whatever pack a
// teammate wears, the breacher's modelled ram crossed by both.
//
// THE BENCH (no game: one man of each operator outfit, driven the way the controllers drive the animator). What is
// measured is the DRAWN thing against the DRAWN man: every box of the tool sampled every 1.5 cm through its volume, each sample asked whether it is INSIDE the
// skinned mesh of the parts he wears (a ray out of his back: an odd count of crossings), the pack and the body counted apart, and how deep (to the nearest face).
//   (3) CLEAR OF THE PACK: on all five operators, hammer and shotgun, standing, walking, running, crouched, creeping, carrying and aiming, no sample
//       of the tool is inside the pack he wears — and it is still ON the pack: its nearest face no further than a strap from the pack's side;
//   (4) CLEAR OF THE MAN: no sample is deeper inside his body, his arms, his helmet or his legs than a strap's give;
//   (5) Sam in his kit with the rifle on his back: the rifle is clear of the radio and of him, and of the tool on the pack's other side;
//   (5b) … and so is each long gun drawn as a model of its own (gunlook.ts), which stands there by the stand-in's sizes;
//   (6) the BREACHER: carrying a real tool he is drawn without his modelled ram, in the small pack; carrying none he keeps it;
//   (7) it MOVES WITH THE PACK: through a walk, a crouch and an aim swung across him the tool's top end keeps its place against the pack's own vertices and
//       no box slides across the pack's face, to a millimetre — while its lower end swings in to lie along his back;
//   (8) THE HAND FINDS IT: through the hammer's draw the right fist is drawn closing on the handle where the drawer has it, within 3 cm, and the drawn fist
//       is over half way to the pack's side when a man is shot in the middle of the reach;
// THE GAME:
//   (10) in a room clear the team's tools are drawn on their backs on the way, and the hand that draws one is told the pack's side.
// SINCE 2026-11-07 the rule is settled on and nothing here writes it (nor the gun models' switch, the motion pass, or the big slot's moves). Out: (1) the
// stand; (2) 'rule off, the fault is there'; the rule-off halves of (4), (5), (6) and (8) ('as near as across a bare back', 'the joints the simulation
// reads are the ones with the rule off': both were this bench against itself with the switch off); (9) 'its boxes are the rule-off boxes'; (10)'s 'the
// same run by digest with the rule off and on'; (11) the recording from before the key and ?packside=0.
//
//   bun run tools/qa/packside-check.ts [--verbose]
//   PACKSIDE_BREAK=1 …                                    (the bench places each tool as on a man with no pack, and measures it against the pack he wears: (3) must fail)
import './stand.ts';   // (the stand's word, written down before this file reads it)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { sample } from './sweep.ts';
const ROOT = resolve(import.meta.dir, '../..');

type V3 = [number, number, number];
const sub = (a: ArrayLike<number>, b: ArrayLike<number>): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]], dot = (a: ArrayLike<number>, b: ArrayLike<number>) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]], len = (a: V3) => Math.hypot(a[0], a[1], a[2]);
const qOf = (b: any): number[] => b.rot ?? [0, Math.sin(b.yaw / 2), 0, Math.cos(b.yaw / 2)];
const rotV = (q: number[], v: V3): V3 => { const [qx, qy, qz, qw] = q; const tx = 2 * (qy * v[2] - qz * v[1]), ty = 2 * (qz * v[0] - qx * v[2]), tz = 2 * (qx * v[1] - qy * v[0]); return [v[0] + qw * tx + qy * tz - qz * ty, v[1] + qw * ty + qz * tx - qx * tz, v[2] + qw * tz + qx * ty - qy * tx]; };
/** > 0: metres outside the box; < 0: minus the depth inside it */
function sdBox(b: any, p: V3): number { const q = qOf(b), l = rotV([-q[0], -q[1], -q[2], q[3]], sub(p, b.c)), dx = Math.abs(l[0]) - b.h[0], dy = Math.abs(l[1]) - b.h[1], dz = Math.abs(l[2]) - b.h[2];
  return dx <= 0 && dy <= 0 && dz <= 0 ? Math.max(dx, dy, dz) : Math.hypot(Math.max(dx, 0), Math.max(dy, 0), Math.max(dz, 0)); }
/** every 1.5 cm through a box's volume, its faces included */
function samples(b: any, out: V3[], step = 0.015) { const q = qOf(b), n = [0, 1, 2].map(i => Math.max(1, Math.ceil(2 * b.h[i] / step)));
  for (let i = 0; i <= n[0]; i++) for (let j = 0; j <= n[1]; j++) for (let k = 0; k <= n[2]; k++) { const w = rotV(q, [(2 * i / n[0] - 1) * b.h[0], (2 * j / n[1] - 1) * b.h[1], (2 * k / n[2] - 1) * b.h[2]]); out.push([b.c[0] + w[0], b.c[1] + w[1], b.c[2] + w[2]]); } }
/** a point against a triangle: the distance to it */
function triDist(p: V3, a: V3, b: V3, c: V3): number {
  const ab = sub(b, a), ac = sub(c, a), ap = sub(p, a), d1 = dot(ab, ap), d2 = dot(ac, ap); if (d1 <= 0 && d2 <= 0) return len(ap);
  const bp = sub(p, b), d3 = dot(ab, bp), d4 = dot(ac, bp); if (d3 >= 0 && d4 <= d3) return len(bp);
  const vc = d1 * d4 - d3 * d2; if (vc <= 0 && d1 >= 0 && d3 <= 0) { const v = d1 / (d1 - d3); return len(sub(ap, [ab[0] * v, ab[1] * v, ab[2] * v])); }
  const cp = sub(p, c), d5 = dot(ab, cp), d6 = dot(ac, cp); if (d6 >= 0 && d5 <= d6) return len(cp);
  const vb = d5 * d2 - d1 * d6; if (vb <= 0 && d2 >= 0 && d6 <= 0) { const w = d2 / (d2 - d6); return len(sub(ap, [ac[0] * w, ac[1] * w, ac[2] * w])); }
  const va = d3 * d6 - d5 * d4; if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) { const w = (d4 - d3) / (d4 - d3 + d5 - d6), e = sub(c, b); return len(sub(bp, [e[0] * w, e[1] * w, e[2] * w])); }
  const n = cross(ab, ac), l = len(n) || 1; return Math.abs(dot(ap, n)) / l;
}
/** does the ray p + t·d (t > 0) cross the triangle */
function rayHits(p: V3, d: V3, a: V3, b: V3, c: V3): boolean {
  const e1 = sub(b, a), e2 = sub(c, a), h = cross(d, e2), det = dot(e1, h); if (Math.abs(det) < 1e-12) return false;
  const f = 1 / det, s = sub(p, a), u = f * dot(s, h); if (u < 0 || u > 1) return false;
  const q = cross(s, e1), v = f * dot(d, q); if (v < 0 || u + v > 1) return false;
  return f * dot(e2, q) > 1e-7;
}

export async function packsideCheck(): Promise<boolean> {
  const H: any = await import('./headless.ts');
  const PS: any = await import(`${ROOT}/src/game/packside.ts`);
  const CH: any = await import(`${ROOT}/src/game/character.ts`);
  const RP: any = await import(`${ROOT}/src/game/rigProps.ts`);
  const GL: any = await import(`${ROOT}/src/anim/gltf.ts`);
  const CB: any = await import(`${ROOT}/src/game/combat.ts`);
  const CA: any = await import(`${ROOT}/src/game/cast.ts`);
  const KT: any = await import(`${ROOT}/src/game/kit.ts`);
  const KN: any = await import(`${ROOT}/src/game/knobs.ts`);
  const { skinVertices } = await import(`${ROOT}/src/render/meshbvh.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const VERBOSE = process.argv.includes('--verbose'), BREAK = process.env.PACKSIDE_BREAK === '1';
  let all = true; const f1 = (x: number) => x.toFixed(1);
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  console.log('\n=== packside: a long thing on a back that wears a pack stands at the pack\'s side, and moves with it ===');
  const knobs0 = KN.knobsCopy(), kit0 = JSON.parse(JSON.stringify(KT.KIT.members));
  try {
    // ------------------------------------------------------------ the bench
    const ch: any = await H.loadCastCharacter(); const cast = ch.cast, mesh = ch.mesh, DT = 1 / 60;
    const I = mesh.indices as Uint32Array | Uint16Array, partOf = mesh.part as Uint8Array;
    const JN: string[] = ch.skin.joints.map((n: number) => ch.nodes[n].name as string);
    const dom = new Uint8Array(mesh.vertexCount);
    for (let v = 0; v < mesh.vertexCount; v++) { let b = 0; for (let k = 1; k < 4; k++) if (mesh.weights[v * 4 + k] > mesh.weights[v * 4 + b]) b = k; dom[v] = mesh.joints[v * 4 + b]; }
    const kitBits = cast.parts.reduce((m: number, n: string, i: number) => (n.startsWith('kit_') ? (m | (1 << i)) >>> 0 : m), 0);
    const j3 = JN.indexOf('DEF-spine.003');
    const ops: any[] = cast.variants.filter((v: any) => v.side === 'operator');
    type Sit = { name: string; go(c: any, run: (s: number) => void): void };
    const walk = (c: any, v: number) => { c.anim.speed = v; c.vel = [0, 0, v]; }, carry = (c: any) => { c.anim.upper = 'none'; c.anim.stance = 'lowReady'; };
    const SITS: Sit[] = [
      { name: 'idle', go: (c, run) => run(2) }, { name: 'walk', go: (c, run) => { walk(c, 1.4); run(2.3); } }, { name: 'run', go: (c, run) => { walk(c, 3.4); run(2.15); } },
      { name: 'crouch', go: (c, run) => { c.anim.crouchTarget = 1; run(2); } }, { name: 'crouch-walk', go: (c, run) => { c.anim.crouchTarget = 1; walk(c, 0.9); run(2.3); } },
      { name: 'carry', go: (c, run) => { carry(c); run(2); } }, { name: 'carry-walk', go: (c, run) => { carry(c); walk(c, 1.3); run(2.3); } }, { name: 'carry-crouch', go: (c, run) => { carry(c); c.anim.crouchTarget = 1; run(2); } },
      { name: 'aim', go: (c, run) => { c.anim.upper = 'aim'; run(2); } }, { name: 'aim-walk', go: (c, run) => { c.anim.upper = 'aim'; walk(c, 1.3); run(2.3); } }, { name: 'aim-crouch', go: (c, run) => { c.anim.upper = 'aim'; c.anim.crouchTarget = 1; run(2); } },
      { name: 'aim-aside', go: (c, run) => { c.anim.upper = 'aim'; c.aimYaw = c.bodyYaw + 70 * Math.PI / 180; run(2); } },
    ];
    /** one man of a variant, stood up bare and armed with his rifle (attach-mesh.ts's stand) */
    const asked = sample(ops, 2);   // (two of the five operators on an ordinary run)
    const man = (variant: any) => { const c: any = new CH.Character(ch, { id: 20 + ops.indexOf(variant), tint: [0.07, 0.09, 0.1], tint2: [0.05, 0.07, 0.06], pos: [0, 0, 0], yaw: 0 });
      c.anim.phase = 0; c.anim.trained = 1; c.anim.upper = 'none'; c.anim.stance = 'none';
      c.anim.longGun = true; c.anim.longSteady = 1; c.anim.longGrip = RP.sc20kGrip(undefined, variant === ops[0]); c.muzzleAhead = RP.sc20kMuzzleAhead();
      const run = (secs: number) => { for (let i = 0; i < Math.round(secs * 60); i++) { if (c.vel) c.pos = [c.pos[0] + c.vel[0] * DT, c.pos[1], c.pos[2] + c.vel[2] * DT]; c.update(DT); } };
      run(0.5); return { c, run }; };
    /** the part mask the game would draw him with: the box goggles' mask, and (rule on, a tool carried) the breacher's pack for his ram */
    const maskOf = (variant: any, carries: boolean, suit = false): number => PS.packMask(ch, suit ? GL.castSuit(cast, variant.mask) : GL.castGoggles(cast, variant.mask, true), carries);
    interface In { n: number; deep: number; at: string }
    interface Read { pack: In; body: In; near: number; samples: number }
    /** the boxes of one drawn thing against the man as skinned now: how many samples are inside the pack / the rest of him, how deep, and how near the pack's faces it stands */
    function read(c: any, mask: number, boxes: any[]): Read {
      const P = skinVertices(mesh.positions, mesh.joints, mesh.weights, c.skel.jointData) as Float32Array;
      const jd = c.skel.jointData, a = j3 * 16, out: V3 = [-jd[a + 8], -jd[a + 9], -jd[a + 10]], ol = len(out) || 1; out[0] /= ol; out[1] /= ol; out[2] /= ol;   // out of his back, as the pack's own matrix has it
      const S: V3[] = []; for (const b of boxes) samples(b, S);
      const lo: V3 = [Infinity, Infinity, Infinity], hi: V3 = [-Infinity, -Infinity, -Infinity]; for (const p of S) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k], p[k]); hi[k] = Math.max(hi[k], p[k]); }
      const vtx = (v: number): V3 => [P[v * 3], P[v * 3 + 1], P[v * 3 + 2]];
      // (only the faces a sample's ray can cross or a sample can lie near: those inside the samples' own box, grown 15 cm and swept out along the ray)
      for (let k = 0; k < 3; k++) { lo[k] -= 0.15; hi[k] += 0.15; if (out[k] > 0) hi[k] += 1.2 * out[k]; else lo[k] += 1.2 * out[k]; }
      const tris: { a: V3; b: V3; c: V3; pack: boolean; j: number }[] = [];
      for (let t = 0; t < I.length; t += 3) { const p = partOf[I[t]]; if (!((mask >>> p) & 1)) continue; const A = vtx(I[t]), B = vtx(I[t + 1]), C = vtx(I[t + 2]); let far = false;
        for (let k = 0; k < 3 && !far; k++) far = Math.max(A[k], B[k], C[k]) < lo[k] || Math.min(A[k], B[k], C[k]) > hi[k];
        if (!far) tris.push({ a: A, b: B, c: C, pack: ((kitBits >>> p) & 1) === 1, j: dom[I[t]] }); }
      const R: Read = { pack: { n: 0, deep: 0, at: '' }, body: { n: 0, deep: 0, at: '' }, near: Infinity, samples: S.length };
      for (const p of S) { let np = 0, nb = 0;
        for (const T of tris) if (rayHits(p, out, T.a, T.b, T.c)) { if (T.pack) np++; else nb++; }
        for (const [odd, grp, isPack] of [[np & 1, R.pack, true], [nb & 1, R.body, false]] as [number, In, boolean][]) { if (!odd) continue; let d = Infinity, at = '';
          for (const T of tris) if (T.pack === isPack) { const x = triDist(p, T.a, T.b, T.c); if (x < d) { d = x; at = JN[T.j].replace('DEF-', ''); } }
          if (d > 0.002) grp.n++; if (d > grp.deep) { grp.deep = d; grp.at = at; } }
      }
      for (let k = 0; k < S.length; k += 7) for (const T of tris) if (T.pack) { const x = triDist(S[k], T.a, T.b, T.c); if (x < R.near) R.near = x; }
      return R;
    }
    const tool = (c: any, kind: 'hammer' | 'shotgun', clear = false) => { const o: any[] = []; RP.bigBoxes(c, kind, 0, o, RP.RIG, clear); return o; };
    const rifle = (c: any) => { const o: any[] = [], meta: any[] = []; RP.rigBoxes(c, { goggles: false, nv: false, slot: 0, lightOn: false, isGuard: false, rifle: 'slung' }, o, RP.RIG, meta); const m = meta.find(k => k.path === 'sc20k.slung'); return m ? o.slice(m.boxStart, m.boxStart + m.boxCount) : []; };
    const worst = (rs: { r: Read; tag: string }[], f: (r: Read) => number) => rs.reduce((w, x) => (f(x.r) > f(w.r) ? x : w), rs[0]);
    const cm = (m: number) => f1(m * 100);
    type Row = { r: Read; tag: string };
    const sweep = (kinds: ('hammer' | 'shotgun')[]): Row[] => { const rows: Row[] = [];
      for (const v of asked) for (const kind of kinds) for (const s of SITS) { const { c, run } = man(v); c.parts = maskOf(v, true, BREAK); s.go(c, run); c.pos = [0, 0, 0]; c.update(0);   // (PACKSIDE_BREAK: placed as on a man with no pack…)
        const r = read(c, maskOf(v, true), tool(c, kind)); rows.push({ r, tag: `${v.name} ${kind} ${s.name}` });   // (…and measured against the pack he wears)
        if (VERBOSE) console.log(`    ${v.name} ${kind.padEnd(7)} ${s.name.padEnd(13)} pack ${String(r.pack.n).padStart(4)} / ${cm(r.pack.deep).padStart(4)} cm · body ${String(r.body.n).padStart(4)} / ${cm(r.body.deep).padStart(4)} cm ${r.body.at.padEnd(12)} · near ${cm(r.near)} cm · ${r.samples} samples`); }
      return rows; };
    const on = sweep(['hammer', 'shotgun']);
    // (3)
    { const wp = worst(on, r => r.pack.deep), wn = worst(on, r => r.near), inPack = on.filter(x => x.r.pack.n > 0).length;
      line(on.length === asked.length * 2 * SITS.length && inPack === 0 && wp.r.pack.deep <= 0.003 && wn.r.near <= 0.03, '(3) on all five operators, hammer and shotgun, in every pose, no sample of the tool is inside the pack he wears — and it stands on the pack\'s side, not off it',
        `${on.length} cases · inside the pack: ${inPack} cases, the worst ${cm(wp.r.pack.deep)} cm (${wp.tag}) · the furthest it stands off the pack's faces ${cm(wn.r.near)} cm (${wn.tag})`); }
    // (4)
    { const wb = worst(on, r => r.body.deep), deep = on.filter(x => x.r.body.deep > 0.015);
      line(wb.r.body.deep <= 0.015, '(4) no sample is deeper inside his body, his arms, his helmet or his legs than a strap\'s give (1.5 cm)',
        `the worst ${cm(wb.r.body.deep)} cm into his ${wb.r.body.at}, ${wb.r.body.n} samples (${wb.tag}) · ${deep.length} cases over`); }
    // (5) Sam in his kit, the rifle on his back
    { const rows: { rule: boolean; r: Read; cross: number; tag: string }[] = [];
      for (const kind of [null, 'hammer', 'shotgun'] as const) for (const s of SITS) { const { c, run } = man(ops[0]); c.parts = maskOf(ops[0], kind !== null); s.go(c, run); c.pos = [0, 0, 0]; c.update(0);
        const rf = rifle(c), r = read(c, c.parts, rf); let cross = 0;
        if (kind) { const tb = tool(c, kind, true), S: V3[] = []; for (const b of rf) samples(b, S); for (const p of S) for (const b of tb) cross = Math.max(cross, -sdBox(b, p)); }
        rows.push({ rule: true, r, cross, tag: `${kind ?? 'alone'} ${s.name}` });
        if (VERBOSE) console.log(`    sam rifle + ${(kind ?? 'nothing').padEnd(7)} ${s.name.padEnd(13)} pack ${String(r.pack.n).padStart(4)} / ${cm(r.pack.deep).padStart(4)} cm · body ${String(r.body.n).padStart(4)} / ${cm(r.body.deep).padStart(4)} cm ${r.body.at.padEnd(12)} · into the tool ${cm(cross)} cm`); }
      const b = rows, mx = (xs: typeof rows, f: (x: typeof rows[0]) => number) => xs.reduce((w, x) => (f(x) > f(w) ? x : w), xs[0]);
      const bp = mx(b, x => x.r.pack.deep), bb = mx(b, x => x.r.body.deep), bc = mx(b, x => x.cross); const drawn = rows.every(x => x.r.samples > 500), far = mx(b, x => x.r.near);
      line(drawn && far.r.near <= 0.05 && bp.r.pack.deep <= 0.003 && bb.r.body.deep <= 0.015 && bc.cross <= 0.002, '(5) Sam in his kit with the rifle on his back: clear of the radio, of him and of the tool on the pack\'s other side',
        `${rows.length} cases, the rifle drawn in ${drawn ? 'all' : 'NOT ALL'} (${rows[0].r.samples} samples) · off the radio's faces at most ${cm(far.r.near)} cm (${far.tag}), into the radio ${cm(bp.r.pack.deep)} cm (${bp.tag}), into him ${cm(bb.r.body.deep)} cm (${bb.r.body.at}, ${bb.tag}), into the tool ${cm(bc.cross)} cm (${bc.tag})`); }
    // (5b) each long gun drawn as its own model (gunlook.ts), slung on that back. The model stands where the stand-in does, by the stand-in's sizes
    //      (rigProps.ts packSideDims): this line is what says that is enough. Sam alone: only the player's rifle goes across his back — a teammate's hangs in front.
    
    // (6) the breacher
    { const br = ops.find((v: any) => v.parts.includes('kit_breacher')), bit = (n: string) => (1 << cast.parts.indexOf(n)) >>> 0, ram = bit('kit_breacher'), pack = bit('kit_rifleman');
      const withTool = maskOf(br, true), without = maskOf(br, false);
      line(!!br && (withTool & ram) === 0 && (withTool & pack) !== 0 && (without & ram) !== 0, '(6) the breacher: carrying a real tool he is drawn in the small pack, without his modelled ram; carrying none he keeps it',
        `a tool: ram ${(withTool & ram) !== 0} pack ${(withTool & pack) !== 0} · none: ram ${(without & ram) !== 0}`); }
    // (7) it moves with the pack
    { let drift = 0, slide = 0, swing = 0, n = 0, at = '';
      for (const v of [ops[0], ops[4]]) for (const kind of ['hammer', 'shotgun'] as const) { const { c, run } = man(v); c.parts = maskOf(v, true);
        const pv: number[] = []; for (let k = 0; k < mesh.vertexCount && pv.length < 3; k++) if ((kitBits >>> partOf[k]) & (c.parts >>> partOf[k]) & 1) pv.push(k);   // three of the pack's own vertices: its frame
        // its TOP END (the hammer's head; a gun's butt plate) against three of the pack's vertices, and every box's distance across from the pack's upright plane (the chest joint's own x)
        const top = (bs: any[]) => (kind === 'hammer' ? bs.find(b => Math.max(...b.h) < 0.1) : bs.reduce((w, b) => (b.c[1] > w.c[1] ? b : w), bs[0]));
        const rel = (): { d: number[]; x: number[]; lean: number } => { const jd = c.skel.jointData, a = j3 * 16, P = skinVertices(mesh.positions, mesh.joints, mesh.weights, jd) as Float32Array, bs = tool(c, kind), t = top(bs);
          const long = bs.reduce((w, b) => (Math.max(...b.h) > Math.max(...w.h) ? b : w), bs[0]), ax = rotV(qOf(long), long.h[0] > long.h[1] ? (long.h[0] > long.h[2] ? [1, 0, 0] : [0, 0, 1]) : long.h[1] > long.h[2] ? [0, 1, 0] : [0, 0, 1]);
          return { d: pv.map(k => (t ? len(sub(t.c, [P[k * 3], P[k * 3 + 1], P[k * 3 + 2]])) : 9)), x: bs.map(b => jd[a] * (b.c[0] - jd[a + 12]) + jd[a + 1] * (b.c[1] - jd[a + 13]) + jd[a + 2] * (b.c[2] - jd[a + 14])),
            lean: Math.acos(Math.min(1, Math.abs(ax[0] * jd[a + 4] + ax[1] * jd[a + 5] + ax[2] * jd[a + 6]))) }; };
        const r0 = rel();
        const step = (secs: number, tag: string) => { for (let i = 0; i < secs * 60; i++) { run(DT); const r = rel(); n++; swing = Math.max(swing, Math.abs(r.lean - r0.lean));
          for (let k = 0; k < r.x.length; k++) slide = Math.max(slide, Math.abs(r.x[k] - (r0.x[k] ?? 9)));
          if (kind === 'hammer') for (let k = 0; k < 3; k++) { const d = Math.abs(r.d[k] - r0.d[k]); if (d > drift) { drift = d; at = `${v.name} ${kind} ${tag}`; } } } };
        walk(c, 1.4); step(1.5, 'walk'); c.anim.crouchTarget = 1; step(1.5, 'crouch-walk'); c.anim.speed = 0; c.vel = null; c.anim.upper = 'aim'; c.aimYaw = c.bodyYaw + 1.2; step(1, 'aim aside'); c.aimYaw = c.bodyYaw - 1.2; step(1, 'aim across'); }
      line(n > 500 && drift <= 0.001 && slide <= 0.001 && swing > 0.02, '(7) it moves with the pack: through a walk, a crouch and an aim swung across him the tool\'s top end keeps its place against the pack\'s own vertices and no part of it slides across the pack\'s face — while its lower end swings in to lie along his back',
        `the most the hammer's head moved against three of the pack's vertices: ${(drift * 1000).toFixed(2)} mm (${at || '-'}) · the most any box moved across the pack's upright plane: ${(slide * 1000).toFixed(2)} mm · over ${n} frames · the most it turned about its top end against the pack: ${(swing * 180 / Math.PI).toFixed(0)}°`); }
    // (8) the hand finds it
    { const rows: { gap: number; tag: string }[] = [], simOn: number[] = [], fallOn: number[] = []; let bad = '', reach = 1;
      for (const v of asked) { const { c, run } = man(v); c.parts = maskOf(v, true); carry(c); run(1);
        const B = { kind: 'hammer', t: 0, equip: 0.7, windup: 0.5, stow: 0.6, blows: [] as number[], end: -1, at: [0, 1.0, 0.8] as V3, snug: true };
        let best = Infinity;
        for (let t = 0; t < 0.5; t += DT) { B.t = t; c.anim.bigTool = B; c.anim.bigSide = CB.hammerSide(c); run(DT); { const hs = c.handFrameR(true); simOn.push(...(c.bones.handR as number[]), ...hs.fingers, ...hs.back); }   // (his wrist, and the hand's frame a kit swap starts from: kitanim.ts)
          if ((c.anim.bigK ?? 0) > 0.02) break;   // (it has left his back: the fist had it)
          const hb = tool(c, 'hammer')[0]; if (!hb) { bad = 'no hammer drawn'; break; }
          const w = c.world, hp = c.skel.nodePos(c.anim.nodes.handR), wr: V3 = [w[0] * hp[0] + w[4] * hp[1] + w[8] * hp[2] + w[12], w[1] * hp[0] + w[5] * hp[1] + w[9] * hp[2] + w[13], w[2] * hp[0] + w[6] * hp[1] + w[10] * hp[2] + w[14]];
          const hf = c.handFrameR(), hollow: V3 = [0, 1, 2].map(i => wr[i] + hf.fingers[i] * 0.08 - hf.back[i] * 0.03) as V3;
          best = Math.min(best, Math.max(0, sdBox(hb, hollow))); }
        // (shot in the middle of the reach: the arm he falls from is the simulation's, not the drawn one)
        { const { c: d, run: go } = man(v); d.parts = maskOf(v, true); carry(d); go(1);
          for (let t = 0; t < 0.2; t += DT) { d.anim.bigTool = { ...B, t }; d.anim.bigSide = CB.hammerSide(d); go(DT); }
          reach = Math.min(reach, d.anim.bigAtBack); d.ragdollize(null, [0, 0, 1], true); const nn = d.anim.nodes;
          for (const j of [nn.upperArmR, nn.forearmR, nn.handR]) { const r = d.skel.globalRot[j]; fallOn.push(r[0], r[1], r[2], r[3]); } }
        rows.push({ gap: best, tag: v.name }); if (VERBOSE) console.log(`    ${v.name} the fist's hollow to the handle at its nearest before it leaves his back: ${cm(best)} cm`); }
      const b = rows, mx = (xs: typeof rows) => xs.reduce((w, x) => (x.gap > w.gap ? x : w), xs[0]);
      line(!bad && b.length === asked.length && mx(b).gap <= 0.03 && reach > 0.5 && simOn.length > 100 && fallOn.length === asked.length * 12, '(8) the hand finds it: drawing the hammer, the right fist is DRAWN closing on the handle where the drawer has it (within 3 cm), and a man shot in the middle of the reach has his drawn fist over half way to the pack\'s side',
        `the fist's hollow to the handle at its nearest, the worst man ${cm(mx(b).gap)} cm (${mx(b).tag}) · ${simOn.length / 9} frames of the draw read · shot in the middle of the reach: the drawn fist ${(reach * 100).toFixed(0)}% of the way to the pack's side${bad ? ' · ' + bad : ''}`); }

    // ------------------------------------------------------------ the game
    const BK: any = await import(`${ROOT}/src/game/bigkit.ts`);
    const game = async () => {
      for (let i = 0; i < KT.KIT_MEMBERS; i++) KT.KIT.members[i] = { ...KT.kitDefault(i), big: i % 2 ? 'hammer' : 'shotgun' };
      const W: any = await hushed(() => H.standUp({ uploadEvery: 4, seed: 1, cast: true })); const g = W.game, I = W.input;
      const BS = KT.BIGSLOT, was = { on: BS.on }; BS.on = true; g.godMode = true;
      await hushed(() => g.roomClear.start(1)); W.step(1 / 60);   // (what a man wears is what the game last told the page: a step on)
      const mates = g.guards.filter((m: any) => BK.bigCarried(g, m.char) !== null);
      let drawn = 0, sided = 0;
      const run = (secs: number, keys: string[] = []) => { for (const k of keys) I.keys.add(k); for (let i = 0; i < secs * 60; i++) { W.step(1 / 60); const out: any[] = []; CB.handProps(g, out);
        for (const m of mates) { if (out.some((b: any) => b.name === RP.BIG_TAG && (b.owner & 0xff) === (m.char.id & 0xff))) drawn++; if (m.char.anim.bigSide) sided++; } }
        for (const k of keys) I.keys.delete(k); };
      run(2); run(3, ['KeyW']); run(2, ['KeyD']); run(2);
      BS.on = was.on;
      return { g, drawn, sided, mates: mates.length };
    };
    const g1 = await game();
    // (10)
    line(g1.mates >= 2 && g1.drawn > 100 && g1.sided > 100, '(10) in a room clear the team\'s tools are drawn on their backs on the way, and a hand is told the pack\'s side',
      `${g1.mates} teammates · tool frames drawn ${g1.drawn} · frames a hand was told the pack's side ${g1.sided}`);
  } finally { KN.knobsBack(knobs0); KT.KIT.members.splice(0, KT.KIT.members.length, ...kit0); }
  return all;
}

if (import.meta.main) process.exit((await packsideCheck()) ? 0 : 1);
