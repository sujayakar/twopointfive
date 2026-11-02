// THE LIST WITHOUT THE PACK (scene/boxes.ts BoxWorld.relist): between two ticks nothing is drawn, so the box world is only made into the list the game reads — the
// dynamics' rows, the visible and cut lists, the glass list, the trace grid and every write to the GPU wait for the next upload(). Two things must then hold, and this
// plays the same scripted runs twice to hold them: (A) every undrawn tick followed by upload(true), the whole pack, as the page did; (B) by relist().
//   the game       the raw bits of what moves, chained after every update (game/digest.ts rawBits): the same in both — the game reads nothing of the pack
//   the picture    after every DRAWN tick's upload() (the play-through draws every 4th) every buffer the box world has written on the device, byte for byte as the
//                  writes left it, and the visible counts: the same in both — what relist leaves undone the next upload does whole
//   bun run tools/qa/relist-check.ts [--runs mousehole-four:1,hostage-rescue:1]
//   RELIST_BREAK=1   the red path: after every relist the statics' rows are called sent when they are not — a static packed on an undrawn tick never reaches the
//                    device, and 'the picture' must FAIL
// EXIT 0 = both hold on every run · 1 = one does not · 2 = unknown flag.   Headless: the device is the harness's stand-in, the bytes are what a GPU would have been sent.
import { resolve } from 'node:path';
export {};
const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const MARK = 'RELIST-ARM ';

async function arm(which: string, scn: string, seed: number) {
  process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1';
  const { Game } = await import('../../src/game/game.ts');
  const PT = await import('./playthrough.ts');
  const { rawBits, RAW_BITS_SEED } = await import('../../src/game/digest.ts');
  const { MARK_TABLE_ROWS } = await import('../../src/scene/markings.ts'); const { GB_ROWS } = await import('../../src/scene/boxes.ts'); const { AIR_ROWS } = await import('../../src/scene/airGrid.ts'); const { CUT_STRIDE } = await import('../../src/scene/cut.ts');
  const BREAK = process.env.RELIST_BREAK === '1' && which === 'b';
  const sent = new Map<object, Uint8Array>(), live = new Map<object, number>(); const tags = new Map<string, object>(); let inWorld = false, writes = 0;
  const put = (buf: object, off: number, data: ArrayBuffer | ArrayBufferView, dOff = 0, size?: number) => {
    if (!inWorld) return; writes++;
    const per = ArrayBuffer.isView(data) ? (data as unknown as { BYTES_PER_ELEMENT?: number }).BYTES_PER_ELEMENT ?? 1 : 1;   // (offset and size count ELEMENTS of a typed array, bytes of a buffer)
    const src = ArrayBuffer.isView(data) ? new Uint8Array(data.buffer, data.byteOffset + dOff * per, size === undefined ? data.byteLength - dOff * per : size * per)
      : new Uint8Array(data, dOff, size === undefined ? data.byteLength - dOff : size);
    if (off === 0) live.set(buf, src.length);   // (a buffer written from its start holds that much: what lies past it is an older, longer write's — nothing reads it)
    let b = sent.get(buf); if (!b || b.length < off + src.length) { const n = new Uint8Array(Math.max(off + src.length, b ? b.length * 2 : 0)); if (b) n.set(b); sent.set(buf, b = n); }
    b.set(src, off);
  };
  const cutRows = (w: any, b: Uint8Array, base: number): Uint8Array[] => { const out: Uint8Array[] = [];
    for (let i = 0; i < w.all.length; i++) { const x = w.all[i]; if ((w.cutOn && x.cut !== undefined && x.cut.length > 0) || (w.roundOn && x.round === true)) out.push(b.subarray(base + i * CUT_STRIDE * 16, base + (i + 1) * CUT_STRIDE * 16)); } return out; };
  let h: number = RAW_BITS_SEED, ticks = 0, packed = true, wrapped: any = null, frames = 0, undrawn = 0; let pic = 0n; const marks: string[] = [];
  const hash = (Bun as unknown as { hash: (d: Uint8Array, seed?: bigint) => bigint }).hash;
  const update = Game.prototype.update;
  Game.prototype.update = function (this: any, ...a: unknown[]) {
    const world = this.engine?.world;
    if (world && wrapped !== world) { wrapped = world;
      const dev = world.device, q = dev.queue; const queue = new Proxy(q, { get: (t, k) => k === 'writeBuffer' ? (buf: object, off: number, data: ArrayBuffer | ArrayBufferView, dOff?: number, size?: number) => put(buf, off, data, dOff, size) : (t as any)[k] });
      world.device = new Proxy(dev, { get: (t, k) => k === 'queue' ? queue : (t as any)[k] });
      const tag = () => { for (const k of Object.keys(world)) if (k.endsWith('Buf')) { let t = tags.get(k); if (!t) tags.set(k, t = { label: k, destroy() { } }); if (world[k] !== t) world[k] = t; } };   // (the stand-in hands out one object for every buffer: each gets a name of its own)
      const up = world.upload;
      world.upload = function (this: any, ...x: unknown[]) { tag(); inWorld = true; try { return up.apply(this, x); } finally { inWorld = false;
        if (!x[0]) { packed = true; frames++;   // a DRAWN tick's pack: what is on the device now?
          // what the shaders can read of each buffer: the rows of the boxes there are (geo, mat), the tables behind the mat rows whole, the visible words there are, and of
          // a buffer written from its start (the grid's cells and items, the capsules, the info block) as much as was last written; of the cut table the rows of the boxes that are cut or round now
          const n = this.all.length, gs = this.geoData.byteLength / this.maxBoxes, ms = this.matData.byteLength / this.maxBoxes;
          const cutBase = this.maxBoxes * ms + (MARK_TABLE_ROWS + GB_ROWS + AIR_ROWS) * 16;   // (boxMat's tail: the marking table, the glass table, the air grid, then the cut table — a box's rows there are read only while the box is cut or round)
          for (const [k, t] of [...tags].sort((p, q) => p[0] < q[0] ? -1 : 1)) { const b = sent.get(t); if (!b) continue;
            const parts: Uint8Array[] = k === 'geoBuf' ? [b.subarray(0, n * gs)] : k === 'matBuf' ? [b.subarray(0, n * ms), b.subarray(this.maxBoxes * ms, cutBase), ...cutRows(this, b, cutBase)] : k === 'visibleBuf' ? [b.subarray(0, (this.visibleCount + (this.visibleCutCount ?? 0)) * 4)] : [b.subarray(0, live.get(t) ?? 0)];
            let hk = BigInt(k.length); for (const part of parts) hk = hash(part, hk); pic = hash(new Uint8Array(new BigUint64Array([hk]).buffer), pic); }
          pic = hash(new Uint8Array(new Uint32Array([this.visibleCount, this.visibleCutCount ?? 0, this.all.length]).buffer), pic);
          if (frames % 300 === 0) marks.push(pic.toString(36)); } } };
    }
    if (world && ticks > 0 && !packed) { undrawn++;   // the tick before this one was not drawn: what the page does between two ticks
      if (which === 'a') world.upload(true); else { world.relist(); if (BREAK) world.rowsStatN = world.statics.length; } }
    packed = false;
    const r = (update as (...x: unknown[]) => unknown).apply(this, a);
    h = rawBits(this as never, h); ticks++; return r; } as typeof update;
  const r = await (PT as any).play(scn, seed, 300, false, {}) as any;
  console.log(MARK + JSON.stringify({ which, scn, seed, ticks, frames, undrawn, writes, end: String(r.end), bits: h.toString(36), picture: pic.toString(36), marks }));
}

if (argv[0] === '--arm') { const [scn, seed] = argv[2].split(':'); await arm(argv[1], scn, Number(seed)); process.exit(0); }
for (const a of argv) if (a.startsWith('--') && a !== '--runs') { console.error(`unknown flag ${a} — --runs`); process.exit(2); }
const runs = (flag('--runs') ?? 'mousehole-four:1,hostage-rescue:1').split(',');
const self = resolve(import.meta.dir, 'relist-check.ts'); let bad = false; const lines: string[] = [];
for (const run of runs) {
  const got = await Promise.all(['a', 'b'].map(async which => {
    const pr = Bun.spawn(['bun', 'run', self, '--arm', which, run], { cwd: resolve(import.meta.dir, '../..'), stdout: 'pipe', stderr: 'pipe', env: { ...process.env } });
    const [out, err] = await Promise.all([new Response(pr.stdout).text(), new Response(pr.stderr).text()]); const code = await pr.exited; const l = out.split('\n').find(x => x.startsWith(MARK));
    if (!l || code !== 0) console.error(`  ${run} arm ${which} exit ${code}: ${err.trim().split('\n').slice(-3).join(' / ').slice(0, 400)}`);
    return l ? JSON.parse(l.slice(MARK.length)) as { ticks: number; frames: number; undrawn: number; writes: number; end: string; bits: string; picture: string; marks: string[] } : null; }));
  const [A, B] = got; if (!A || !B) { bad = true; lines.push(`- FAIL — ${run}: an arm left no line`); continue; }
  const game = A.bits === B.bits && A.ticks === B.ticks && A.end === B.end, picture = A.picture === B.picture && A.frames === B.frames;
  let first = -1; for (let i = 0; i < Math.max(A.marks.length, B.marks.length); i++) if (A.marks[i] !== B.marks[i]) { first = i; break; }
  if (!game || !picture) bad = true;
  lines.push(`- ${game ? 'PASS' : 'FAIL'} — ${run}: the game is the same run to the bit with the list alone between two ticks`, `      ${A.ticks} ticks (${A.end}), ${A.undrawn} of them undrawn · bits ${A.bits} / ${B.bits}`);
  lines.push(`- ${picture ? 'PASS' : 'FAIL'} — ${run}: every drawn tick leaves the device the bytes the whole pack between ticks left it`, `      ${A.frames} drawn ticks · ${A.writes} writes with the pack between, ${B.writes} with the list · the device's bytes ${A.picture} / ${B.picture}${first >= 0 ? ` · apart from drawn tick ${(first + 1) * 300} or before` : ''}`);
}
console.log(`the list without the pack · ${runs.join(', ')}\n${lines.join('\n')}\n=== relist-check: ${bad ? 'FAILED' : 'ALL PASS'} ===`);
process.exit(bad ? 1 : 0);
