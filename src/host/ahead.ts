// ONE MAN DRAWN AHEAD OF THE TICK (the picture's half of a networked page's prediction: src/net/page.ts works out WHERE he is, this file draws him there).
// A networked page's game stands at the last tick the server confirmed. The player's own man has records on their way that the game has not had yet, and where
// they take him is known (game/walkahead.ts walkAhead: his legs walked on a copy, nothing of the game written). So he is DRAWN there and everything else where the game has it: for the length of the host's render
// every drawn thing that is his is moved as ONE RIGID BODY from where the tick has him to where his records take him — a move, and a turn about the vertical
// through his feet:
//
//   his instance of the men's palette      every joint matrix (the palette the tween blended, where it blended one: host/tween.ts)
//   his shadow proxies and his cull sphere  the capsule ends in BoxWorld.capsuleData, matched by his id
//   the boxes he wears and carries          the rig's own boxes, by their motion key (MKEY.rig with his id): centre, yaw, quaternion, drawn centre
//   the lights he carries                   a weapon light, a torch, an infrared lamp (his, with no life to run out): position and direction
//   where the outline pass looks for him    render/outline.ts, laid from the game before every draw
//
// WHAT IS NOT AHEAD: his pose. The walk cycle, the lift of the gun and the twist of his chest are the tick's, carried over whole; the turn is by his AIM, so the
// gun and its light point where the newest record aims. What he has DONE at a place stays where the tick drew it (a tracer in flight and its head light, a muzzle flash, a casing, a laser's line, a charge laid on a wall),
// and so does the HUD's mark of him, which is laid from the game after the move is taken down.
// THE RULE is the tween's: the game's own state is never the moved one when a tick runs. `pose()` hangs moved arrays on the drawn things and `unpose()` hangs
// the tick's own back by reference (the capsule ends, a flat array, are written back value for value); nothing of the game runs between the two. It is laid
// OVER the tween's blend — pose after the tween's, unpose before it — and reads the blend's factor, so between two ticks he is drawn between the two places
// the last two ticks' look ahead gave him, as everybody else is drawn between the two places those ticks gave them.
// THE WORLD'S PACK: the render packs the boxes as it finds them, so the moved ones. A host that runs a tick after such a render packs the world again first
// (net/page.ts does, before its first confirmed tick of a page tick), as it must after a tweened render too.
import { BoxWorld, MKEY, type Box } from '../scene/boxes';
import type { RtLight } from '../render/lights';
import type { Vec3, Quat } from '../math/vec';
import { TWEEN, type TweenHost, type TweenSkin } from './tween';

/** the men's palette as this file needs it: the tween's, and one man's matrices written alone (anim/characters.ts drawJointsOf) */
interface AheadSkin extends TweenSkin { drawJointsOf(i: number, data: Float32Array): void }
/** a man the outline pass rims (render/outline.ts OutlineMan): whose pixels, and where it looks for them */
interface Rimmed { on: boolean; owner: number; at: ArrayLike<number> & { [i: number]: number } }
/** the host: the tween's, and the outline's frame once it has one (ui/findme.ts lays it from the game before every draw) */
export type AheadHost = TweenHost & { outline?: (Rimmed & { mates: Rimmed[] }) | null };
/** a box a man wears or carries: the rig's own motion key with his id in it (game/combat.ts: MKEY.rig + his id << 8 + the box's ordinal). NOT his owner byte
 *  alone — that says whose rays skip a thing, and a tracer in flight and a charge he has laid carry it too: those are where the tick left them */
const RIG = MKEY.rig >>> 24, wornBy = (b: Box, id8: number): boolean => b.mkey !== undefined && b.mkey >>> 24 === RIG && ((b.mkey >>> 8) & 0xff) === id8;
/** a light a man carries (a weapon light, a torch, an infrared lamp): his, and with no life of its own to run out — a muzzle flash and a tracer's head light
 *  are his doing at a place, and stay there */
const carriedBy = (l: RtLight, id8: number): boolean => l.owner !== undefined && (l.owner & 0xff) === id8 && !(l.ttl >= 0);

/** where a man stands, as this file moves him: his feet, and the way he aims (rad, the character's own yaw) */
export interface Stand { pos: ArrayLike<number>; yaw: number }

const CAPS = BoxWorld.CAPS_PER_CHAR, CW = CAPS * 8;   // records a man, and his words: 8 floats a record (ends at 0‥2 and 4‥6)
const ENDS = [0, 1, 2, 4, 5, 6];
const turnOf = (to: number, from: number): number => { let d = (to - from) % (2 * Math.PI); if (d > Math.PI) d -= 2 * Math.PI; else if (d < -Math.PI) d += 2 * Math.PI; return d; };

export class AheadDraw {
  /** what the last `pose()` did, for the checks: whether anybody was moved, how far (m) and how far round (rad), and how many of his things it moved */
  last = { on: false, move: 0, turn: 0, boxes: 0, lights: 0, men: 0, caps: 0 };
  /** draws so far (every `pose()`, moved or not): a host that runs a tick after one packs the world again first — the render packed the boxes as it DREW them */
  draws = 0;
  private id = -1; private have = 0;
  // a tick's pair: from(3) yaw · to(3) yaw
  private prev = new Float64Array(8); private cur = new Float64Array(8);
  // what pose() hung in place, for unpose(): the thing and the arrays it had
  private rb: Box[] = []; private rc: Vec3[] = []; private ryaw: number[] = []; private rrot: (Quat | undefined)[] = []; private rdc: (Vec3 | undefined)[] = [];
  private rl: RtLight[] = []; private rlp: Vec3[] = []; private rld: Vec3[] = [];
  private capAt: number[] = []; private capSave = new Float64Array(0);
  private out = new Float32Array(0); private posed: number[] = [];   // (the instances whose matrices on the GPU are moved ones)
  // scratch arrays handed to the picture for one draw (never to the game): reused frame after frame
  private v3s: Vec3[] = []; private v3n = 0; private q4s: Quat[] = []; private q4n = 0;
  private vec(): Vec3 { return this.v3s[this.v3n++] ?? (this.v3s[this.v3n - 1] = [0, 0, 0]); }
  private quat(): Quat { return this.q4s[this.q4n++] ?? (this.q4s[this.q4n - 1] = [0, 0, 0, 1]); }
  private at = new Float64Array(8);

  constructor(private host: AheadHost) {}
  private skin(): AheadSkin | null { const s = this.host.skin as Partial<AheadSkin> | null; return s && s.staged && typeof s.drawJointsOf === 'function' ? s as AheadSkin : null; }

  /** THE END OF A TICK: where that tick has man `id` (`from`) and where his records take him (`to`). Copies what it keeps. */
  capture(id: number, from: Stand, to: Stand): void {
    if (id !== this.id) { this.id = id; this.have = 0; }
    const S = this.prev; this.prev = this.cur; this.cur = S;
    S[0] = from.pos[0]; S[1] = from.pos[1]; S[2] = from.pos[2]; S[3] = from.yaw; S[4] = to.pos[0]; S[5] = to.pos[1]; S[6] = to.pos[2]; S[7] = to.yaw;
    this.have = Math.min(2, this.have + 1);
  }
  /** nobody is ahead (the next capture starts afresh) */
  rest(): void { this.have = 0; this.id = -1; }

  /** ONE DRAW WITH HIM AHEAD: the pose, the host's draw, and the pose taken down WHATEVER HAPPENED IN EITHER (a pose that threw half way had already hung
   *  some of his things: whoever reads the game after it, a scripted page's next step or the record of the crash, must find the tick's own). The pair lives
   *  HERE and not at the host's call, so that a check can hold it (tools/qa/aheaddraw-check.ts); the host calls this and never the two below by hand.
   *  NOT PUT BACK after a throw: his cull sphere where a rig fit is set (unpose leaves it to the next upload, which rebuilds it; only the GPU reads it) */
  around(a: number, blended: Float32Array | null, draw: () => void): void { try { this.pose(a, blended); draw(); } finally { this.unpose(); } }

  /** RIGHT BEFORE THE HOST'S RENDER, after the tween's pose: `a` = the tween's factor this frame (1 = the last tick as it is), `blended` = the palette the tween
   *  handed the GPU this frame (null: it handed none, the tick's own is there). Always paired with `unpose()`: `around()` is the pair. */
  private pose(a: number, blended: Float32Array | null): void {
    this.draws++; const R = this.last; R.on = false; R.move = 0; R.turn = 0; R.boxes = 0; R.lights = 0; R.men = 0; R.caps = 0;
    this.v3n = 0; this.q4n = 0;
    const sk = this.skin(), W = this.host.world, L = this.host.lights, id8 = this.id & 0xff;
    const back = () => { if (sk) { const from = blended ?? sk.staged, jf = sk.jointFloats; for (const i of this.posed) if (i < sk.instanceCount) sk.drawJointsOf(i, from.subarray(i * jf, (i + 1) * jf)); } this.posed.length = 0; };   // (a man whose moved matrices were the last thing written to the GPU has the tween's or the tick's again)
    if (!this.have || this.id < 0 || !id8) { back(); return; }
    // ---- where from and where to, at this frame's factor: a place that jumped past the tween's `snap` in the one tick was cut there, not flown
    const P = this.prev, C = this.cur, S = this.at, between = this.have > 1 && a < 1, snap = TWEEN.snap;
    for (let o = 0; o < 8; o += 4) {
      const cut = !between || Math.hypot(C[o] - P[o], C[o + 1] - P[o + 1], C[o + 2] - P[o + 2]) > snap;
      for (let k = 0; k < 3; k++) S[o + k] = cut ? C[o + k] : P[o + k] + (C[o + k] - P[o + k]) * a;
      S[o + 3] = cut ? C[o + 3] : P[o + 3] + turnOf(C[o + 3], P[o + 3]) * a;
    }
    const fx = S[0], fz = S[2], tx = S[4], tz = S[6], dy = S[5] - S[1], turn = turnOf(S[7], S[3]), move = Math.hypot(tx - fx, dy, tz - fz);
    if (move < 1e-6 && Math.abs(turn) < 1e-6) { back(); return; }
    R.on = true; R.move = move; R.turn = turn;
    const c = Math.cos(turn), s = Math.sin(turn);
    /** a point of his, moved (the yaw's own sense — scene/boxes.ts: local +X → (cos, 0, −sin)) */
    const pt = (p: ArrayLike<number>, o: Vec3): Vec3 => { const x = p[0] - fx, z = p[2] - fz; o[0] = tx + x * c + z * s; o[1] = p[1] + dy; o[2] = tz - x * s + z * c; return o; };
    // ---- the boxes he wears and carries
    const dyn = W.dynamics, hs = Math.sin(turn / 2), hc = Math.cos(turn / 2);
    for (let i = 0; i < dyn.length; i++) {
      const b = dyn[i]; if (!wornBy(b, id8)) continue;
      const r = b.rot, dc = b.dc;
      this.rb.push(b); this.rc.push(b.c); this.ryaw.push(b.yaw); this.rrot.push(r); this.rdc.push(dc);
      b.c = pt(b.c, this.vec()); b.yaw += turn;
      if (r) { const q = this.quat(); q[0] = hc * r[0] + hs * r[2]; q[1] = hc * r[1] + hs * r[3]; q[2] = hc * r[2] - hs * r[0]; q[3] = hc * r[3] - hs * r[1]; b.rot = q; }   // (the turn about +Y, then his own)
      if (dc) b.dc = pt(dc, this.vec());
      R.boxes++;
    }
    // ---- the lights he carries (the infrared list too)
    for (let pass = 0; pass < 2; pass++) {
      const list = pass ? L.ir : L.lights;
      for (let i = 0; i < list.length; i++) {
        const l = list[i]; if (!carriedBy(l, id8)) continue;
        const d = l.dir;
        this.rl.push(l); this.rlp.push(l.pos); this.rld.push(d);
        l.pos = pt(l.pos, this.vec());
        { const v = this.vec(); v[0] = d[0] * c + d[2] * s; v[1] = d[1]; v[2] = -d[0] * s + d[2] * c; l.dir = v; }
        R.lights++;
      }
    }
    // ---- his instance of the men's palette: every joint matrix of his moved, in an array of our own, and his alone handed to the GPU
    back();
    if (sk && sk.instanceCount > 0) {
      const cnt = sk.instanceCount, jf = sk.jointFloats, from = blended ?? sk.staged;
      for (let i = 0; i < cnt; i++) {
        if ((sk.ownerOf(i) & 0xff) !== id8) continue;
        const need = (this.posed.length + 1) * jf; if (this.out.length < need) { const f = new Float32Array(need); f.set(this.out); this.out = f; }
        const out = this.out.subarray(need - jf, need); out.set(from.subarray(i * jf, (i + 1) * jf));
        for (let m = 0; m < jf; m += 16) {
          for (let k = 0; k < 12; k += 4) { const x = out[m + k], z = out[m + k + 2]; out[m + k] = x * c + z * s; out[m + k + 2] = -x * s + z * c; }
          const x = out[m + 12] - fx, z = out[m + 14] - fz; out[m + 12] = tx + x * c + z * s; out[m + 13] += dy; out[m + 14] = tz - x * s + z * c;
        }
        sk.drawJointsOf(i, out); this.posed.push(i); R.men++;
      }
    }
    // ---- where the outline pass looks for him (laid from the game before this, and laid again before the next draw: nothing to hang back)
    { const F = this.host.outline; if (F) { if (F.on && (F.owner & 0xff) === id8) pt(F.at, F.at as unknown as Vec3); for (const m of F.mates) if (m.on && (m.owner & 0xff) === id8) pt(m.at, m.at as unknown as Vec3); } }
    // ---- his shadow proxies and his cull sphere: written in place, put back by unpose()
    { const cn = W.charCount, D = W.capsuleData, v = this.vec();
      for (let ch = 0; ch < cn; ch++) {
        if ((W.capsuleU32[ch * CW + 7] & 0xff) !== id8) continue;
        const k = this.capAt.length, need = (k + 1) * (CAPS * 6 + 3);
        if (this.capSave.length < need) { const f = new Float64Array(Math.max(need, 2 * (CAPS * 6 + 3))); f.set(this.capSave); this.capSave = f; }
        let w = k * (CAPS * 6 + 3); this.capAt.push(ch);
        for (let i = 0; i < CAPS; i++) { const o = ch * CW + i * 8; for (let j = 0; j < 6; j++) this.capSave[w++] = D[o + ENDS[j]];
          if (!(D[o + 3] > 0)) continue;   // (a padding record: left as it is)
          for (let e = 0; e < 8; e += 4) { v[0] = D[o + e]; v[1] = D[o + e + 1]; v[2] = D[o + e + 2]; pt(v, v); D[o + e] = v[0]; D[o + e + 1] = v[1]; D[o + e + 2] = v[2]; } }
        const bo = 24 + ch * 4, I = W.infoF32; this.capSave[w++] = I[bo]; this.capSave[w++] = I[bo + 1]; this.capSave[w++] = I[bo + 2];
        v[0] = I[bo]; v[1] = I[bo + 1]; v[2] = I[bo + 2]; pt(v, v); I[bo] = v[0]; I[bo + 1] = v[1]; I[bo + 2] = v[2];   // (the game's own cull sphere rides with him; a rig fit rebuilds it from the ends at the upload)
        R.caps++;
      }
    }
  }

  /** RIGHT AFTER THE HOST'S RENDER, before the tween's unpose: every array `pose()` hung is taken down and what stood there hung back */
  private unpose(): void {
    const rb = this.rb;
    for (let i = 0; i < rb.length; i++) { const b = rb[i]; b.c = this.rc[i]; b.yaw = this.ryaw[i]; if (this.rrot[i] !== undefined) b.rot = this.rrot[i]; if (this.rdc[i] !== undefined) b.dc = this.rdc[i]; }
    rb.length = 0; this.rc.length = 0; this.ryaw.length = 0; this.rrot.length = 0; this.rdc.length = 0;
    const rl = this.rl;
    for (let i = 0; i < rl.length; i++) { rl[i].pos = this.rlp[i]; rl[i].dir = this.rld[i]; }
    rl.length = 0; this.rlp.length = 0; this.rld.length = 0;
    const W = this.host.world, D = W.capsuleData;
    for (let k = 0; k < this.capAt.length; k++) {
      const ch = this.capAt[k]; let w = k * (CAPS * 6 + 3);
      for (let i = 0; i < CAPS; i++) { const o = ch * CW + i * 8; for (let j = 0; j < 6; j++) D[o + ENDS[j]] = this.capSave[w++]; }
      if (!W.rig) { const bo = 24 + ch * 4; W.infoF32[bo] = this.capSave[w]; W.infoF32[bo + 1] = this.capSave[w + 1]; W.infoF32[bo + 2] = this.capSave[w + 2]; }
    }
    this.capAt.length = 0;
  }
}
