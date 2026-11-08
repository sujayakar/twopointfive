// EVERY RUN KEPT, AND READ BACK OUT — the probes (src/tape/keeper.ts · keepstore.ts · kept.ts; game/replaybuf.ts since()):   bun run tools/qa/replaykeep-probes.ts
//   whole      a run kept in pieces, written out as one hand-over file (handover-file.ts) and put back together is the file F7 would have made at that moment: samples, events, lines, cast, head,
//              run tape — and a fresh game walks its tape to the same digest
// EXIT 0 all pass · 1 a failure.
import { gunzipSync } from 'node:zlib';
import { standUp, ROOT } from './headless.ts';
import { handOver } from './handover-file.ts';
const { ReplayBuffer, REPLAY } = await import(`${ROOT}/src/game/replaybuf.ts`);
const F = await import(`${ROOT}/src/tape/format.ts`);
const KP = await import(`${ROOT}/src/tape/kept.ts`);
const KS = await import(`${ROOT}/src/tape/keepstore.ts`);
const { ReplayKeeper, ringSource } = await import(`${ROOT}/src/tape/keeper.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { levelFingerprint } = await import(`${ROOT}/src/game/savestate.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { rerun } = await import('./rerun.ts');

let fails = 0;
const check = (ok: boolean, text: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) fails++; };
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);

function hands(W: any, f: number): void {
  const I = W.input, s = f / 60;
  I.keys.clear();
  if (s > 0.5 && s < 6) I.keys.add('KeyW'); if (s > 3 && s < 5) I.keys.add('KeyD'); if (s > 9 && s < 12) I.keys.add('KeyS');
  I.mouseX = 800 + 300 * Math.sin(s * 0.7); I.mouseY = 500 + 200 * Math.cos(s * 0.5);
  if (f === 400) I.pressed.add('KeyE');
  if (f === 470 || f === 500 || f === 530) { I.clicked |= 1; I.buttons |= 1; } else I.buttons = 0;
  if (f === 700) I.pressed.add('KeyC'); if (f === 120 || f === 900) I.pressed.add('KeyB');
}
interface Rig { W: any; buf: any; K: any; store: any; hashes: string[]; meta: () => any; }
/** a game with its tape being kept: a beat every `every` frames (packed, as the page's are), the last one a flush */
async function run(secs: number, o: { query?: string | null; keep?: boolean; every?: number; store?: any; sid?: string; played?: () => boolean; mid?: (R: Rig, f: number) => void | Promise<void>; gz?: any; budget?: () => Promise<number>; noFlush?: boolean } = {}): Promise<Rig> {
  Math.random = mulberry32(0xbeef);
  const W: any = await standUp({ seed: 7 });
  if (o.query) { const r = startFromQuery(W.game, o.query); if (r.why) throw new Error(r.why); }
  const keep = o.keep !== false, buf = keep ? new ReplayBuffer() : null, store = o.store ?? KS.memoryStore();
  const meta = () => ({ url: o.query ? `?${o.query}` : '', commit: 'abc1234', userAgent: 'probe', fp: levelFingerprint(W.game), session: o.sid ?? 'visit-a' });
  let now = 0;
  const K = buf ? new ReplayKeeper(ringSource(buf), { sid: o.sid ?? 'visit-a', store, played: o.played ?? (() => true), head: () => { const { session: _s, ...m } = meta(); return buf.head(W.game, m); },
    now: () => new Date(Date.UTC(2026, 9, 11, 12, 0, now++)).toISOString(), ...(o.gz !== undefined ? { gz: o.gz } : {}), ...(o.budget ? { budget: o.budget } : {}) }) : null;
  if (buf) { buf.tape.born(W.game); W.game.replayTap = (dt: number, cmd?: unknown) => { if (cmd) buf.top(W.game, dt, cmd); else buf.tick(W.game, dt, W.cam); }; }
  const R: Rig = { W, buf, K, store, hashes: [], meta };
  for (let f = 0; f < secs * 60; f++) {
    hands(W, f); W.step(1 / 60);
    if ((f + 1) % 60 === 0) R.hashes.push(hashDigest(digestGame(W.game, true)));
    if (K && (f + 1) % (o.every ?? 300) === 0) K.beat();
    if (o.mid) await o.mid(R, f);
  }
  if (K && !o.noFlush) await K.flush();
  return R;
}
/** a hand-over file's bytes as records (gzip members one after another, or plain lines) */
async function lines(blob: Blob): Promise<{ lines: any[]; bad: number }> {
  const u = Buffer.from(await blob.arrayBuffer()); return KP.parseLines(u[0] === 0x1f && u[1] === 0x8b ? gunzipSync(u).toString('utf8') : u.toString('utf8'));
}
/** a kept visit as the page reads it out of the store (the viewer's own road: keepstore.ts readVisit) */
async function visit(store: any, sid = 'visit-a'): Promise<any> { return KS.readVisit(store, sid); }

console.log('\n=== every run kept, and read back out ===');
// ---- whole
{ const R = await run(32, { query: 'roomclear&squad4' }), file = R.buf.file(R.W.game, 'kept', R.meta());
  const d = await handOver(R.store), got = d ? await lines(d.blob) : { lines: [], bad: 1 };
  check(!!d && d.visits === 1 && got.bad === 0 && got.lines[0]?.k === 'dump' && /\.ndjson\.gz$/.test(d.name), `whole: written out it is one file of ${got.lines.length} records, the first saying what it holds (${d?.name}, ${((d?.blob.size ?? 0) / 1024).toFixed(0)} KB for 42 s)`);
  const V = KP.visitsOf(got.lines).get('visit-a'), A = V ? KP.assemble(V) : null;
  check(!!A && A.pieces === 7 && !A.missing.length, `whole: seven pieces for 32 s at one every 5 s and a last one on leaving (${A?.pieces}, missing ${A?.missing.length})`);
  if (A) {
    const bad = F.checkReplay(A.file); check(bad === '', `whole: the file put together passes the replay's own check${bad ? ` — ${bad}` : ''}`);
    for (const k of ['samples', 'events', 'cast', 'map', 'flags', 'level', 'seed', 'hz', 'keep', 'commit', 'url', 'userAgent', 'session', 'seconds', 'rerun'] as const)
      check(same(A.file[k], file[k]), `whole: its ${k} ${k === 'samples' ? `(${A.file.samples.length}) ` : k === 'events' ? `(${A.file.events.length}) ` : k === 'cast' ? `(${A.file.cast.length}) ` : ''}are the file's F7 would have made`);
    check(same(A.file.lines, file.lines), `whole: its lines (${A.file.lines.length}) are that file's`);
    check(A.tapeTicks === 32 * 60 && A.tapeEnd === '', `whole: the run tape is whole (${A.tapeTicks} steps)`);
    { const old = { ...V!, pieces: new Map([...V!.pieces].map(([i, p]: [number, any]) => [i, p.tape ? { ...p, tape: { ...p.tape, v: undefined } } : p])) }, O = KP.assemble(old as any);   // (a visit kept before the tape said its version: the map viewer's alone)
      check((A.file.rerun as any)?.v === F.TAPE_VERSION && F.tapeWhy(A.file) === '' && !!O && /a different version of the game/.test(F.tapeWhy(O.file)), `whole: the kept tape says its version; a visit kept before tapes said one is put together and not walked (${O ? F.tapeWhy(O.file) : 'not put together'})`); }
    const dAt = R.hashes[29], Rr = await rerun(A.file, { toN: 1800 });
    check(!Rr.refuse && !!Rr.report && !Rr.report.drift && !!Rr.W && hashDigest(digestGame(Rr.W.game, true)) === dAt, `whole: a fresh game walks the kept tape to step 1800 and digests as the first run did there${Rr.refuse ? ` — REFUSED: ${Rr.refuse}` : ''}`);
    console.log(`  cost: ${R.K.stats.beats} beats, ${(R.K.stats.ms / R.K.stats.beats).toFixed(2)} ms each to take and word a piece (worst ${R.K.stats.worstMs.toFixed(2)} ms); ${(R.K.filed.bytes / 42 / 1024).toFixed(1)} KB a second of play kept`);
  } }
console.log(fails ? `\n=== replaykeep: ${fails} FAILED ===` : '\n=== replaykeep: ALL PASS ===');
process.exit(fails ? 1 : 0);
