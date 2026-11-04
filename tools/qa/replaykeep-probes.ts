// EVERY RUN KEPT, AND READ BACK OUT — the probes (src/replay/keeper.ts · keepstore.ts · kept.ts; game/replaybuf.ts since()):   bun run tools/qa/replaykeep-probes.ts
//   whole      a run kept in pieces, written out as one hand-over file (handover-file.ts) and put back together is the file F7 would have made at that moment: samples, events, lines, cast, head,
//              run tape — and a fresh game walks its tape to the same digest
//   long       a run longer than the ring is kept WHOLE (F7's file holds the ring's last stretch; the kept tape's tail is that stretch, sample for sample)
//   identity   a game stepped with its tape being kept digests, second by second, as the same game with no buffer at all (the keeper only reads)
//   leave      a page that goes away with pieces still being packed loses none: every piece is in the store, once, in order
//   holes      a piece the browser lost: the trace picks up at the next keyframe with a mark, the run tape ends there, the file still passes its check
//   end        a run's end is filed about a second after it, not at the next beat; one beat an end; nothing is lost by the extra beats; and on a real room
//              clear, whose stage word never moves (REPLAYEND_BREAK=1 = the red path: an end files nothing early)
//   ended      a run tape that ended (a scenario started from a menu): the pieces before it still carry a tape that walks, the trace says where it ends
//   unplayed   nothing is filed until the game has been played; then the visit is, from its first step
//   budget     over the budget the oldest visits go first, never the one being written; a full disk is retried once
//   twice      a visit written out twice as it grew: a reader holding both files keeps each piece once; a note on its row rides in the next file
//   reader     tools/qa/replay-dump.ts: hand-over files and saved files of one visit unpack to one tape that replay-at reads, notes and the key's presses on it; sent again = nothing new
// EXIT 0 all pass · 1 a failure.
import { gunzipSync } from 'node:zlib';
import { standUp, ROOT } from './headless.ts';
import { handOver } from './handover-file.ts';
const { ReplayBuffer, REPLAY } = await import(`${ROOT}/src/game/replaybuf.ts`);
const F = await import(`${ROOT}/src/replay/format.ts`);
const KP = await import(`${ROOT}/src/replay/kept.ts`);
const KS = await import(`${ROOT}/src/replay/keepstore.ts`);
const { ReplayKeeper } = await import(`${ROOT}/src/replay/keeper.ts`);
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
  const K = buf ? new ReplayKeeper(buf, { sid: o.sid ?? 'visit-a', store, played: o.played ?? (() => true), head: () => { const { session: _s, ...m } = meta(); return buf.head(W.game, m); },
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
{ const R = await run(42, { query: 'roomclear&squad4' }), file = R.buf.file(R.W.game, 'kept', R.meta());
  const d = await handOver(R.store), got = d ? await lines(d.blob) : { lines: [], bad: 1 };
  check(!!d && d.visits === 1 && got.bad === 0 && got.lines[0]?.k === 'dump' && /\.ndjson\.gz$/.test(d.name), `whole: written out it is one file of ${got.lines.length} records, the first saying what it holds (${d?.name}, ${((d?.blob.size ?? 0) / 1024).toFixed(0)} KB for 42 s)`);
  const V = KP.visitsOf(got.lines).get('visit-a'), A = V ? KP.assemble(V) : null;
  check(!!A && A.pieces === 9 && !A.missing.length, `whole: nine pieces for 42 s at one every 5 s and a last one on leaving (${A?.pieces}, missing ${A?.missing.length})`);
  if (A) {
    const bad = F.checkReplay(A.file); check(bad === '', `whole: the file put together passes the replay's own check${bad ? ` — ${bad}` : ''}`);
    for (const k of ['samples', 'events', 'cast', 'map', 'flags', 'level', 'seed', 'hz', 'keep', 'commit', 'url', 'userAgent', 'session', 'seconds', 'rerun'] as const)
      check(same(A.file[k], file[k]), `whole: its ${k} ${k === 'samples' ? `(${A.file.samples.length}) ` : k === 'events' ? `(${A.file.events.length}) ` : k === 'cast' ? `(${A.file.cast.length}) ` : ''}are the file's F7 would have made`);
    check(same(A.file.lines, file.lines), `whole: its lines (${A.file.lines.length}) are that file's`);
    check(A.tapeTicks === 42 * 60 && A.tapeEnd === '', `whole: the run tape is whole (${A.tapeTicks} steps)`);
    { const T = A.file.rerun as any; check(T?.sens?.f32 === (file.rerun as any)?.sens?.f32, `whole: the kept tape says how its sensor values are written as F7's does (f32 ${T?.sens?.f32 ?? 'absent'}, ${T?.sens?.rows?.length ?? 0} sensor rows)`); }
    const dAt = R.hashes[39], Rr = await rerun(A.file, { toN: 2400 });
    check(!Rr.refuse && !!Rr.report && !Rr.report.drift && !!Rr.W && hashDigest(digestGame(Rr.W.game, true)) === dAt, `whole: a fresh game walks the kept tape to step 2400 and digests as the first run did there${Rr.refuse ? ` — REFUSED: ${Rr.refuse}` : ''}`);
    console.log(`  cost: ${R.K.stats.beats} beats, ${(R.K.stats.ms / R.K.stats.beats).toFixed(2)} ms each to take and word a piece (worst ${R.K.stats.worstMs.toFixed(2)} ms); ${(R.K.filed.bytes / 42 / 1024).toFixed(1)} KB a second of play kept`);
  } }
// ---- long
{ const keep = REPLAY.seconds; REPLAY.seconds = 10;
  const R = await run(32), file = R.buf.file(R.W.game, 'key', R.meta()); REPLAY.seconds = keep;
  const V = await visit(R.store), A = V ? KP.assemble(V) : null;
  check(!!A && file.seconds < 16 && A.file.seconds > 31.5, `long: the ring holds ${file.seconds} s, the kept tape ${A?.file.seconds} s of a 32 s run`);
  if (A) {
    const n = file.samples.length, tail = A.file.samples.slice(-n), rb = (S: any[]) => S.map(s => ({ ...s, r: Math.round((s.r - S[0].r) * 1000) / 1000 }));
    check(same(rb(tail), rb(file.samples)), `long: its last ${n} samples are the ring's, one for one`);
    check(same(A.file.rerun, file.rerun) && A.tapeTicks === 32 * 60, 'long: its run tape is the whole run\'s');
    check(F.checkReplay(A.file) === '', 'long: it passes the replay\'s own check'); } }
// ---- identity
{ const a = await run(20, { keep: false }), b = await run(20, { every: 60 });
  const first = a.hashes.findIndex((h, i) => h !== b.hashes[i]);
  check(first < 0 && a.hashes.length === 20, `identity: 20 s with the tape kept (a piece a second) digest as 20 s with no buffer${first >= 0 ? ` — FIRST DIFFERENCE at second ${first + 1}` : ''}`); }
// ---- leave
{ let release: () => void = () => {}; const gate = new Promise<void>(r => { release = r; });
  const slow = async (t: string) => { await gate; return KS.gzip!(t); };   // (the packing never finishes while the page lives)
  const store = KS.memoryStore(), R = await run(22, { store, gz: slow, noFlush: true, mid: (R2, f) => { if (f === 22 * 60 - 1) { void R2.K.flush(); } } });
  await new Promise(r => setTimeout(r, 20)); const before = (await store.bodies('visit-a')); release(); await R.K.idle();
  const after = await store.bodies('visit-a'), V = await visit(store), A = V ? KP.assemble(V) : null;
  check(before.length === 5 && before.every((b: any) => typeof b === 'string'), `leave: the page going away put all five pieces in the store at once, as they stood (${before.length})`);
  check(after.length === 5 && !!A && !A.missing.length && same(A.file.samples, R.buf.file(R.W.game, 'k', R.meta()).samples), 'leave: none was written twice or lost once the packing came back'); }
// ---- leave, on a store like the browser's: its plain put reads the row back first, and a dying page never gets that read — the going page must use the put that asks nothing back
{ let release: () => void = () => {}; const gate = new Promise<void>(r => { release = r; });
  const slow = async (t: string) => { await gate; return KS.gzip!(t); };
  const M = KS.memoryStore(); let plain = 0, now = 0;
  const store: any = { ...M, put: () => { plain++; return new Promise<boolean>(() => {}); }, putNow: (r: any, q: number, b: any, h?: string) => { now++; return M.put(r, q, b, h); } };
  const R = await run(22, { store, gz: slow, noFlush: true, mid: (R2, f) => { if (f === 22 * 60 - 1) { void R2.K.flush(); } } });
  await new Promise(r => setTimeout(r, 20)); const got = await M.bodies('visit-a'), row = (await M.rows())[0]; release();
  check(got.length === 5 && now === 5 && plain === 0 && row?.pieces === 5 && row.seconds >= 21, `leave: on a store whose plain put waits on a read, the going page's five pieces and its row are in at once by the put that reads nothing (${got.length} pieces, ${now} now, ${plain} plain, row ${row?.pieces} pieces ${row?.seconds} s)`);
  void R; }
// ---- left: a page that says it is going (pagehide → keeper.leave) is not 'being played' a moment later; one whose timer beats again is
//      (REPLAYLEFT_BREAK=1 = the red path: the mark is not read, and a game just left reads as being played for a minute)
{ const M = KS.memoryStore(), R = await run(11, { store: M, noFlush: true }), K = R.K, soon = (r: any) => Date.parse(r.lastAt) + 3000;
  const more = (n: number) => { for (let f = 0; f < n; f++) { hands(R.W, 660 + f); R.W.step(1 / 60); } };
  await K.idle(); const live = (await M.rows())[0];
  check(!!live && !live.left && KS.beingWritten(live, soon(live)), `left: a visit its page filed three seconds ago reads as being played (${live?.pieces} pieces)`);
  more(60); await K.leave(); const gone = (await M.rows())[0], V = await visit(M), A = V ? KP.assemble(V) : null;
  check(!!gone && gone.left === true && gone.pieces === live.pieces + 1 && !KS.beingWritten(gone, soon(gone)), `left: the page going filed what was left and said so — three seconds on the visit does not read as being played (${gone?.pieces} pieces, left ${gone?.left})`);
  check(!!A && !A.missing.length && same(A.file.samples, R.buf.file(R.W.game, 'k', R.meta()).samples), 'left: and the tape it left is whole to the last step');
  await K.flush(); const hid = (await M.rows())[0];
  check(hid.left === true && hid.pieces === gone.pieces, 'left: the tab going hidden right after (nothing new to file) does not take the mark off');
  K.beat(); await K.idle(); const idle = (await M.rows())[0];
  check(!idle.left && idle.pieces === gone.pieces, 'left: the same page brought back — its timer beats, nothing new to file — is not marked as gone any more');
  await K.leave(); more(60); K.beat(); await K.idle(); const back = (await M.rows())[0];
  check(!back.left && back.pieces === gone.pieces + 1 && KS.beingWritten(back, soon(back)), `left: gone and back again, it files by its timer and reads as being played (${back.pieces} pieces)`);
  // another tab put a note on the visit while this page sat: the going page lays nothing over it
  await M.patch('visit-a', { notes: [{ r: 1, text: 'n' }] }); await K.leave(); const kept = (await M.rows())[0];
  check(kept.left === true && kept.notes?.length === 1 && !KS.beingWritten(kept, soon(kept)), `left: going with nothing new to file marks the visit and leaves a note another tab wrote as it was (notes ${kept.notes?.length})`);
  const out = (await KS.pruneKept(M, 0, null, soon(kept))).map((r: any) => r.sid);
  check(same(out, ['visit-a']), `left: a visit its page has left is let go like any other when the room runs out (${out.join(', ') || 'none'})`);
  // …and that visit, removed while its page sat, is not brought back by the page going
  K.back(); await K.leave(); await K.idle();
  check((await M.rows()).length === 0 && (await M.bodies('visit-a')).length === 0, 'left: a visit removed while its page sat is not made again by the page going');
  // …and the page brought back after that goes on as a tape that reads: its drawing filed again, its front marked missing — not pieces with no start
  //    (REPLAYLEFT_BREAK=1 never lets the visit go above, so this row is red on it too)
  K.back(); more(420); K.beat(); await K.idle(); const V2 = await visit(M), A2 = V2 ? KP.assemble(V2) : null, again = (await M.rows())[0];
  check(K.stats.again > 0 && (await M.head('visit-a')) !== null && !!A2 && A2.missing.length === back.pieces && F.checkReplay(A2.file) === '' && A2.file.samples.length > 20 && !!again && again.bytes > 0 && again.seconds > 5 && again.seconds <= 8 && !again.left,
    `left: brought back after its visit was removed, the page files its drawing again and the tape reads from its next keyframe (${A2?.missing.length} pieces marked missing, ${A2?.file.samples.length} samples, ${again?.seconds} s; ${A2 ? F.checkReplay(A2.file) || 'check ok' : 'no file'})`);
  // played, and gone before the timer's first beat: filed and marked all the same
  const S1 = KS.memoryStore(), R1 = await run(3, { store: S1, noFlush: true }); await R1.K.leave(); const early = (await S1.rows())[0];
  check(!!early && early.pieces === 1 && early.left === true, `left: a page left before its first beat files what was played and is marked (${early?.pieces} piece, left ${early?.left})`);
  const S2 = KS.memoryStore(), R2 = await run(2, { store: S2, played: () => false, noFlush: true }); await R2.K.leave();
  check((await S2.rows()).length === 0, 'left: a page nobody played, going, files nothing'); }
// ---- holes
{ const R = await run(32), whole = KP.assemble(await visit(R.store))!, V = await visit(R.store); V.pieces.delete(2);
  const A = KP.assemble(V);
  check(!!A && same(A.missing, [2]) && F.checkReplay(A.file) === '', `holes: a visit with piece 2 lost still makes a file that passes its check (missing ${JSON.stringify(A?.missing)})`);
  if (A) {
    const mark = A.file.lines.find((l: any) => l.k === 'mark' && /missing/.test(l.text)), gapAt = A.file.samples.findIndex((s: any, i: number) => i > 0 && s.n - A.file.samples[i - 1].n > 60);
    check(!!mark && gapAt > 0 && !!A.file.samples[gapAt].k, `holes: the trace picks up at a keyframe after the hole, and a line marks it (${A.file.samples.length} of ${whole.file.samples.length} samples)`);
    check(A.tapeTicks === 600 && /missing/.test(A.tapeEnd) && (A.file.rerun as any)?.ticks === 600, `holes: the run tape walks to the hole and says why it stops (${A.tapeTicks} steps: ${A.tapeEnd})`); } }
// ---- end: a run that BECOMES over has its piece asked KEEP.endMs later, whatever the timer (here a beat every ten seconds): over at 13 s (and still over at
//      16 s, where a page's word would move from one end to the other), a new run at 17 s and its end at 18 s
{ const store = KS.memoryStore(), snaps: Record<number, { beats: number; ends: number; seconds: number }> = {};
  const over = (f: number) => (f >= 13 * 60 && f < 17 * 60) || f >= 18 * 60;
  const R = await run(22, { store, every: 600, mid: async (R2, f) => { if (R2.K.watch(over(f), (f + 1) * 1000 / 60)) void R2.K.endBeat();
    if (f === 14 * 60 + 30 || f === 17 * 60 + 30 || f === 19 * 60 + 30) { await R2.K.idle(); const row = (await store.rows())[0]; snaps[Math.floor(f / 60)] = { beats: R2.K.stats.beats, ends: R2.K.stats.ends, seconds: row?.seconds ?? 0 }; } } });
  const a = snaps[14], b = snaps[17], c = snaps[19];
  check(!!a && a.ends === 1 && a.beats === 2 && a.seconds > 13.5 && a.seconds < 14.5, `end: a mission that ended at 13 s is in the store to ${a?.seconds} s a second and a half later, with the timer's next beat still seconds off (${a?.beats} beats, ${a?.ends} for the end)`);
  check(!!b && b.ends === 1 && b.beats === 2, `end: a run that stays over is one end (${b?.beats} beats, ${b?.ends} for an end)`);
  check(!!c && c.ends === 2 && c.beats === 3 && c.seconds > 18.5, `end: the next run's end is filed too (${c?.seconds} s, ${c?.ends} for an end)`);
  const V = await visit(store), A = V ? KP.assemble(V) : null;
  check(!!A && !A.missing.length && same(A.file.samples, R.buf.file(R.W.game, 'k', R.meta()).samples), 'end: the early beats lose nothing: the visit puts back together as the file F7 would make');
  const R3 = await run(6, { every: 600, mid: (R2, f) => { if (R2.K.watch(true, (f + 1) * 1000 / 60)) void R2.K.endBeat(); } });
  check(R3.K.stats.ends === 0, `end: a page whose first answer is 'over' files nothing for it (${R3.K.stats.ends})`);
  // the timer's beat has just taken everything: the end's own beat finds nothing new, and the end is filed all the same (counted, nothing written twice)
  let R4n = -1;
  const R4 = await run(12, { every: 600, mid: (R2, f) => { if (f !== 10 * 60 + 5) return; R2.K.watch(false, 0); R2.K.watch(true, 0);
    if (R2.K.watch(true, 5000)) { R2.K.beat(); const n = R2.K.stats.beats; void R2.K.endBeat(); R4n = R2.K.stats.beats - n; } } });
  check(R4.K.stats.ends === 1 && R4n === 0, `end: an end the timer's beat has already taken is an end filed, with no second piece (${R4.K.stats.ends} for the end, ${R4n} new beats)`); }
// ---- end, on a real mission: a room clear with every defender down. The mission's stage word never moves there — the page's own answer (ui/replaykey.ts runOver)
//      reads the verdict — and the keeper files the end a second after it (the verdict's rule is held off on the frozen stand: it is put on for this, as the page has it)
{ const { VERDICT } = await import(`${ROOT}/src/game/verdict.ts`), { killGuard } = await import(`${ROOT}/src/game/combat.ts`), { hostileTo } = await import(`${ROOT}/src/game/teams.ts`), { runOver } = await import(`${ROOT}/src/ui/replaykey.ts`);
  const von = VERDICT.on; VERDICT.on = true; let word0 = '', wordEnd = '', overAt = -1, endAt = -1, secsAt = 0; const store = KS.memoryStore();
  try {
    const R = await run(12, { query: 'roomclear&squad4', store, every: 600, mid: async (R2, f) => { const G = R2.W.game;
      if (f === 5) { const p = G.player.char.pos; G.teleportPlayer([p[0] + 3, 0, p[2]]); }
      if (f === 10) { word0 = G.mission.stage; for (const h of G.guards.filter((x: any) => x.team !== 'operative' && hostileTo(x, 'player') && x.state !== 'dead')) killGuard(G, h, [1, 0, 0], false, 'shot', 'player'); }
      if (overAt < 0 && runOver(G)) overAt = f;
      if (R2.K.watch(runOver(G), (f + 1) * 1000 / 60)) { await R2.K.endBeat(); endAt = f; secsAt = (await store.rows())[0]?.seconds ?? 0; } } });
    wordEnd = R.W.game.mission.stage;
    check(overAt > 10, `end: a squad's room clear is over by its verdict at ${(overAt / 60).toFixed(1)} s, and the mission's stage word reads '${word0}' before and '${wordEnd}' after`);
    check(endAt > overAt && endAt - overAt <= 62 && R.K.stats.ends === 1 && secsAt > overAt / 60, `end: the keeper filed it ${((endAt - overAt) / 60).toFixed(2)} s later, the store holding ${secsAt} s (${R.K.stats.ends} for the end)`);
  } finally { VERDICT.on = von; } }
// ---- ended
{ const R = await run(30, { mid: (R2, f) => { if (f === 16 * 60) { const r = startFromQuery(R2.W.game, 'roomclear&squad4'); if (r.why) throw new Error(r.why); } } });
  const file = R.buf.file(R.W.game, 'k', R.meta()), V = await visit(R.store), A = V ? KP.assemble(V) : null;
  check('dead' in (file.rerun ?? {}), `ended: F7's file has no tape left (${(file.rerun as any)?.dead})`);
  check(!!A && A.tapeTicks === 900 && /scenario/.test(A.tapeEnd) && (A.file.rerun as any)?.ticks === 900, `ended: the kept tape still walks the ${A?.tapeTicks} steps handed over before it ended (${A?.tapeEnd})`);
  if (A) { check(A.file.lines.some((l: any) => l.k === 'mark' && /run tape ends here/.test(l.text)) && A.file.seconds > 29.5, 'ended: the trace runs on to the end and marks where the tape stops');
    const Rr = await rerun(A.file, { toN: 900 }); check(!Rr.refuse && !!Rr.report && !Rr.report.drift && !!Rr.W && hashDigest(digestGame(Rr.W.game, true)) === R.hashes[14], `ended: a fresh game walks it to step 900 and digests as the first run did${Rr.refuse ? ` — REFUSED: ${Rr.refuse}` : ''}`); } }
// ---- unplayed
{ let played = false; const store = KS.memoryStore();
  const R = await run(20, { store, played: () => played, mid: async (R2, f) => { if (f === 11 * 60) { await R2.K.idle(); check((await store.rows()).length === 0 && R2.K.stats.beats === 0, 'unplayed: ten seconds of a game nobody played filed nothing'); played = true; } } });
  const V = await visit(store), A = V ? KP.assemble(V) : null;
  check(!!A && A.file.seconds > 19.5 && A.tapeTicks === 1200 && !A.file.lines.some((l: any) => l.k === 'mark'), `unplayed: once played the visit is kept from its first step (${A?.file.seconds} s, ${A?.tapeTicks} steps)`); }
// ---- budget
{ const store = KS.memoryStore(), row = (sid: string, day: number, bytes: number) => ({ sid, startedAt: `2026-10-0${day}T00:00:00Z`, lastAt: `2026-10-0${day}T01:00:00Z`, scenario: '', url: '', seconds: 60, pieces: 1, bytes });
  // (laid in no order of age; 'second' carries the marks a row of an older page may still hold — sent, sentAt, noted — which nobody reads any more)
  for (const r of [row('second', 2, 400), row('oldest', 1, 400), row('newest', 5, 400), row('live', 6, 400), row('third', 4, 400)]) { await store.put(r, 0, 'x'.repeat(r.bytes), '{}\n'); if (r.sid === 'second') await store.patch(r.sid, { sent: 1, sentAt: 'x', noted: true } as any); }
  const out = (await KS.pruneKept(store, 1300, 'live')).map((r: any) => r.sid);
  check(same(out, ['oldest', 'second']), `budget: 2000 bytes into 1300: the two oldest go, oldest first (${out.join(', ')})`);
  const out2 = (await KS.pruneKept(store, 500, 'live')).map((r: any) => r.sid);
  check(same(out2, ['third', 'newest']) && (await store.rows()).map((r: any) => r.sid).join() === 'live', `budget: into 500: then the next oldest, and never the visit being written (${out2.join(', ')})`);
  const out3 = await KS.pruneKept(store, 0, null, Date.parse('2026-10-06T01:00:30Z'));
  check(out3.length === 0 && (await store.rows()).length === 1, 'budget: a visit some page wrote to half a minute ago (another tab, still played) is not let go');
  // a disk that fills: the piece is tried again once the oldest tape has made room
  const tight = KS.memoryStore(60000); await tight.put(row('stale', 1, 50000), 0, 'x'.repeat(50000), '{}\n');
  const R = await run(12, { store: tight, sid: 'visit-b', gz: null, budget: async () => 60000 });
  const rows = (await tight.rows()).map((r: any) => r.sid);
  check(R.K.stats.failed >= 1 && R.K.stats.let >= 1 && !rows.includes('stale') && rows.includes('visit-b'), `budget: a put the disk refused let the stale tape go and went in on the second try (${R.K.stats.failed} refused, ${R.K.stats.let} let go; kept: ${rows.join(', ')})`);
  const none = KS.memoryStore(10), R3 = await run(22, { store: none, sid: 'visit-c', gz: null });
  check(!!R3.K.broke && R3.K.stats.pieces === 0, `budget: a browser that keeps nothing stops the keeper after three tries, with the reason (${R3.K.broke})`);
  const R4 = await run(22, { sid: 'visit-d', gz: null, budget: async () => 2000 });
  check(/past everything/.test(R4.K.broke), `budget: one visit bigger than the whole budget stops being kept, in words (${R4.K.broke})`); }
// ---- twice: the same visit written out as it grew
{ const store = KS.memoryStore(); let d1: any = null;
  const R = await run(27, { store, mid: async (R2, f) => { if (f === 16 * 60) { await R2.K.idle(); d1 = await handOver(store); } } });
  const d2 = await handOver(store);
  check(!!d1 && d1.pieces === 3 && !!d2 && d2.pieces === 6, `twice: a file made at 16 s carried 3 pieces; the visit grew, and a second file carries it whole (${d2?.pieces} pieces)`);
  if (d1 && d2) {
    const both = KP.visitsOf([...(await lines(d1.blob)).lines, ...(await lines(d2.blob)).lines]), A = KP.assemble(both.get('visit-a')!);
    check(both.size === 1 && !!A && A.pieces === 6 && same(A.file.samples, R.buf.file(R.W.game, 'k', R.meta()).samples), 'twice: a reader holding both files has one visit, each piece once, and the whole run');
    const S = await KS.keptSummary(store); check(S.visits === 1 && S.bytes > 0 && S.seconds > 0 && !('fresh' in S) && !('noted' in S), `twice: the store says what is kept, and nothing of what a file carried (${S.visits} visit, ${S.seconds} s)`);
    // a note put on the visit's row (the viewer's) rides in the next file, and writing one out leaves the row as it was
    await store.patch('visit-a', { notes: [{ id: 'n1', r: 3, t: 3, n: 180, pos: null, text: 'late note', at: 'x' }] });
    const before = JSON.stringify((await store.rows())[0]), dn = await handOver(store), ln = dn ? (await lines(dn.blob)).lines : [], V = KP.visitsOf(ln).get('visit-a');
    check(!!dn && ln.some((l: any) => l.k === 'notes' && l.sid === 'visit-a' && l.notes[0].text === 'late note') && V?.notes?.[0]?.text === 'late note' && JSON.stringify((await store.rows())[0]) === before, 'twice: a note put on the visit rides in the next file, and the row is as it was'); } }

// ---- reader: tools/qa/replay-dump.ts over two hand-over files of one visit, a file saved by the key in it (with a note), and the same things sent again
{ const { mkdtempSync, writeFileSync, readFileSync, existsSync, rmSync } = await import('node:fs'); const { tmpdir } = await import('node:os'); const { join } = await import('node:path');
  const dir = mkdtempSync(join(tmpdir(), 'replaykeep-')), inn = join(dir, 'in'), out = join(dir, 'out'); (await import('node:fs')).mkdirSync(inn);
  const store = KS.memoryStore(); let d1: any = null, f7: any = null;
  const R = await run(27, { query: 'roomclear&squad4', store, sid: 'visit-r', mid: async (R2, f) => { if (f === 16 * 60) { await R2.K.idle(); d1 = await handOver(store); f7 = R2.buf.file(R2.W.game, 'key', { ...R2.meta(), session: 'visit-r' }); } } });
  const d2 = await handOver(store);
  const sm = f7.samples[Math.floor(f7.samples.length / 2)]; f7.notes = [{ id: 'n1', r: sm.r, t: sm.t, n: sm.n, pos: [10, 12], text: 'look here', at: '2026-10-11T12:00:00.000Z' }];
  writeFileSync(join(inn, 'a-dump.ndjson.gz'), Buffer.from(await d1.blob.arrayBuffer())); writeFileSync(join(inn, 'b-dump.ndjson.gz'), Buffer.from(await d2!.blob.arrayBuffer()));
  writeFileSync(join(inn, 'c-f7.json'), JSON.stringify(f7)); writeFileSync(join(inn, 'd-f7-again.json'), JSON.stringify(f7));
  const tool = (...a: string[]) => { const r = Bun.spawnSync(['bun', 'run', `${ROOT}/tools/qa/replay-dump.ts`, ...a, '--out', out]); return { code: r.exitCode, text: r.stdout.toString() + r.stderr.toString() }; };
  const t1 = tool(inn), IX = existsSync(join(out, 'index.json')) ? JSON.parse(readFileSync(join(out, 'index.json'), 'utf8')) : { entries: {} }, E = IX.entries['visit-r'];
  check(t1.code === 0 && Object.keys(IX.entries).length === 1 && !!E && E.pieces === 6, `reader: two dumps and two saved files of one visit unpack to ONE tape (${Object.keys(IX.entries).length} entr${Object.keys(IX.entries).length === 1 ? 'y' : 'ies'}, ${E?.pieces} pieces)`);
  if (E) {
    const T = JSON.parse(gunzipSync(readFileSync(join(out, `${E.name}.json.gz`))).toString('utf8')), whole = R.buf.file(R.W.game, 'k', R.meta());
    check(F.checkReplay(T) === '' && same(T.samples, whole.samples) && !!T.rerun?.ticks, 'reader: the tape it writes is a replay file like any other, the whole run with its run tape');
    const note = T.notes[0], at = T.samples.find((x: any) => x.n === sm.n);
    check(T.notes.length === 1 && note.text === 'look here' && !!at && Math.abs(note.r - at.r) < 1e-6, `reader: a note put on the saved file sits on the visit's tape at the same step (${T.notes.length} note)`);
    check(E.f7.length === 1 && /F7 pressed/.test(readFileSync(join(out, `${E.name}.moments.md`), 'utf8')), `reader: where the key was pressed is a moment on the tape, once for the file sent twice (${E.f7.length})`);
    const at2 = Bun.spawnSync(['bun', 'run', `${ROOT}/tools/qa/replay-at.ts`, join(out, `${E.name}.json.gz`), '--note', '1']);
    check(at2.exitCode === 0 && /look here/.test(at2.stdout.toString()), 'reader: replay-at reads the unpacked tape at its note');
  }
  const t2 = tool(join(inn, 'b-dump.ndjson.gz'), join(inn, 'c-f7.json')), IX2 = JSON.parse(readFileSync(join(out, 'index.json'), 'utf8'));
  check(t2.code === 0 && /already here/.test(t2.text) && Object.keys(IX2.entries).length === 1 && IX2.entries['visit-r'].f7.length === 1 && IX2.entries['visit-r'].notes === 1, 'reader: the same dump and file sent again add nothing');
  // a whole kept visit as ONE replay file (why 'kept': what the viewer's 'Download with notes' made until that button went on 2026-10-27, so such files
  // are out there; written in parts): its note moves onto the tape and it is NOT a press of the key
  const dl = KP.assemble(await visit(store, 'visit-r'))!.file, sm2 = dl.samples[Math.floor(dl.samples.length / 3)]; dl.notes = [{ id: 'n2', r: sm2.r, t: sm2.t, n: sm2.n, pos: [8, 9], text: 'from the viewer', at: '2026-10-27T05:00:00.000Z' }];
  const parts = F.replayParts(dl) as string[];
  check(parts.length > 8 && parts.join('') === JSON.stringify(dl) && Math.max(...parts.map(p => p.length)) < JSON.stringify(dl).length, `reader: the download's text is written in parts that join to the file's own JSON (${parts.length} parts, the longest ${Math.max(...parts.map(p => p.length))} of ${JSON.stringify(dl).length} chars)`);
  check(same(F.replayParts({ a: [], b: {}, c: undefined, d: null, e: { f: undefined, g: [1, [2]] } }).join(''), JSON.stringify({ a: [], b: {}, c: undefined, d: null, e: { f: undefined, g: [1, [2]] } })) && F.replayParts([]).join('') === '[]' && F.replayParts({}).join('') === '{}', 'reader: and the parts of empty and nested things are the same text too');
  writeFileSync(join(dir, 'viewer.json'), parts.join(''));
  const t4 = tool(join(dir, 'viewer.json')), IX4 = JSON.parse(readFileSync(join(out, 'index.json'), 'utf8')), E4 = IX4.entries['visit-r'];
  check(t4.code === 0 && Object.keys(IX4.entries).length === 1 && E4.f7.length === 1 && E4.notes === 2 && (readFileSync(join(out, `${E4.name}.moments.md`), 'utf8').match(/F7 pressed/g) ?? []).length === 1, `reader: the viewer's download of a visit whose tape is here adds its note and no press of the key (${E4?.notes} notes, ${E4?.f7.length} press)`);
  const out4 = join(dir, 'out4'), r4 = Bun.spawnSync(['bun', 'run', `${ROOT}/tools/qa/replay-dump.ts`, join(dir, 'viewer.json'), '--out', out4]), IX5 = existsSync(join(out4, 'index.json')) ? JSON.parse(readFileSync(join(out4, 'index.json'), 'utf8')) : { entries: {} }, e5 = Object.values(IX5.entries)[0] as any;
  check(r4.exitCode === 0 && !!e5 && e5.f7.length === 0 && /-visit$/.test(e5.name) && /^\d+ steps$/.test(e5.tape) && /A whole visit/.test(readFileSync(join(out4, `${e5.name}.moments.md`), 'utf8')) && !/F7 pressed/.test(readFileSync(join(out4, `${e5.name}.moments.md`), 'utf8')), `reader: fed alone, that download is a whole visit with its run tape and no press of the key (${e5?.name}, ${e5?.tape})`);
  // single files with no kept tape: the shorter window of the same play is dropped, its note moves over
  const out2 = join(dir, 'out2'), long = R.buf.file(R.W.game, 'key', R.meta()); delete long.session; const short = { ...f7 }; delete short.session;
  writeFileSync(join(dir, 'long.json'), JSON.stringify(long)); writeFileSync(join(dir, 'short.json'), JSON.stringify(short));
  const r3 = Bun.spawnSync(['bun', 'run', `${ROOT}/tools/qa/replay-dump.ts`, join(dir, 'short.json'), join(dir, 'long.json'), '--out', out2]), IX3 = existsSync(join(out2, 'index.json')) ? JSON.parse(readFileSync(join(out2, 'index.json'), 'utf8')) : { entries: {} }, es = Object.values(IX3.entries) as any[];
  check(r3.exitCode === 0 && es.length === 1 && es[0].notes === 1 && es[0].n1 === long.samples[long.samples.length - 1].n, `reader: of two saved files of one play with no kept tape the longer stands and has the other's note (${es.length} kept, ${es[0]?.notes} note)`);
  const bad = Bun.spawnSync(['bun', 'run', `${ROOT}/tools/qa/replay-dump.ts`, join(out, 'index.md'), '--out', join(dir, 'out3')]);
  check(bad.exitCode === 1, `reader: a file that is no replay is said so and nothing is unpacked (exit ${bad.exitCode})`);
  rmSync(dir, { recursive: true, force: true }); }

console.log(fails ? `\n=== replaykeep: ${fails} FAILED ===` : '\n=== replaykeep: ALL PASS ===');
process.exit(fails ? 1 : 0);
