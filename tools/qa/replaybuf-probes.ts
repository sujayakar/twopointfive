// The REPLAY BUFFER's probes (src/game/replaybuf.ts → src/replay/format.ts):   bun run tools/qa/replaybuf-probes.ts [--write <file.json>] [--scenario '<query>'] [--level <file.json>] [--secs N]
//   ring       the ring keeps `seconds` of play and no more, its head is always a keyframe, the file passes its own check and decodes
//   content    the trace holds what a bug report needs: every man each sample, a route for a walking man, a round with where it went, a door that moved
//   notes      a note put on the file survives the trip through JSON
//   fight      a room fight's teammate's row names the man his gun is on (the room fight's own record of him, read only)
//   tape       a room clear taped over a checkpoint loaded and a restart to the start, both from outside a step, is walked again to the digest; a tape that ended, and one of
//              another version, are refused in words. ANEW_BREAK=1 (the restart is not counted) and LOADROW_BREAK=1 (the load's slot is not told) must fail
//   crash      the page's own save on an error (ui/replaykey.ts) makes one file a minute and three a page load, in a browser that keeps no shelf (where the file is sent, never a download);
//              the error itself is a line in the ring at its moment of PLAY, in every file made while the ring holds it
// --write: the content probe's file is written there (what the viewer page is developed against); --scenario / --level: that file is of a scenario on a level file
// instead of the quiet office (its walk / route / shot lines are then told, not judged: the scripted hands were written for the office). EXIT 0 all pass · 1 a failure.
import { writeFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { ReplayBuffer, REPLAY } = await import(`${ROOT}/src/game/replaybuf.ts`);
const F = await import(`${ROOT}/src/replay/format.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { levelFingerprint } = await import(`${ROOT}/src/game/savestate.ts`);

const argv = process.argv.slice(2); const val = (k: string) => { const i = argv.indexOf(`--${k}`); return i >= 0 ? argv[i + 1] : undefined; };
let fails = 0;

// (the `stand` section's second child: the night alone played with a stealth card changed by hand — kit.ts KITDEAL.stealth: the sponge rounds taken off it — taped
// on whatever stand this process was started on, written to the file named; then nothing else runs)
if (val('stealth-tape-to')) {
  const Kt = await import(`${ROOT}/src/game/kit.ts`);
  Kt.kitMission({ rings: 5, cams: 2, wand: true }); Kt.kitSet(0, 'rings', 0); Kt.kitMission(null);
  Math.random = mulberry32(0xbeef);
  const W: any = await standUp({ seed: 21 }); W.game.godMode = true;
  const buf = new ReplayBuffer(); buf.tape.born(W.game);
  W.game.replayTap = (dt: number, cmd?: unknown) => { if (cmd) buf.top(W.game, dt, cmd); else buf.tick(W.game, dt, W.cam); };
  for (let f = 0; f < 300; f++) { const I = W.input, s = f / 60; I.keys.clear(); if (s > 1) I.keys.add('KeyW'); I.mouseX = 800 + 300 * Math.sin(s * 0.7); I.mouseY = 420; W.step(1 / 60); }
  writeFileSync(val('stealth-tape-to')!, JSON.stringify({ ...buf.file(W.game, 'probe', { url: '', fp: levelFingerprint(W.game) }), probeDigest: hashDigest(digestGame(W.game, true)), probeRule: Kt.kitStealthOn(), probeRings: W.game.player.rifle.rings }));
  process.exit(0);
}
// (the `stand` section's child: a room clear with the four taped on whatever stand this process was started on, written to the file named — then nothing else runs)
if (val('tape-to')) {
  const K = await import(`${ROOT}/src/game/knobs.ts`), Kt = await import(`${ROOT}/src/game/kit.ts`), { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  // what a browser keeps per machine: a Team feel row (a table no recording carries — and one that moves this run: the teammates' legs), a kit card
  (K.knobsNamed().PACE as { accel: number }).accel = 0.6; Kt.KIT.members[4].suit = 'navy';   // (how fast a teammate's legs pick a walk up, 2.6 as built: it moves this run on either stand. The row here was PACE.max = 1.4 until the team's walk went under it — teampace.ts, 0.8 of the player's — and the cap stopped moving anything)
  Math.random = mulberry32(0xbeef);
  const W: any = await standUp({ seed: 21 }); const r = startFromQuery(W.game, 'roomclear&squad4'); if (r.why) throw new Error(r.why);
  const buf = new ReplayBuffer(); buf.tape.born(W.game);
  W.game.replayTap = (dt: number, cmd?: unknown) => { if (cmd) buf.top(W.game, dt, cmd); else buf.tick(W.game, dt, W.cam); };
  let digest = '';
  for (let f = 0; f < 20 * 60; f++) { const I = W.input, s = f / 60; I.keys.clear(); if (s > 1 && s < 9) I.keys.add('KeyW'); I.mouseX = 800 + 300 * Math.sin(s * 0.7); I.mouseY = 420 + 150 * Math.cos(s * 0.5);
    if (f === 120 || f === 700) I.pressed.add('KeyB'); W.step(1 / 60); if (f + 1 === 1140) digest = hashDigest(digestGame(W.game, true)); }
  writeFileSync(val('tape-to')!, JSON.stringify({ ...buf.file(W.game, 'probe', { url: '?roomclear&squad4', fp: levelFingerprint(W.game) }), probeDigest: digest }));
  process.exit(0);
}
const check = (ok: boolean, text: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) fails++; };

/** a scripted minute of hands: walk, turn the aim, draw, fire, crouch — the same every call */
function hands(W: any, f: number): void {
  const I = W.input; const s = f / 60;
  I.keys.clear();
  if (s > 0.5 && s < 6) I.keys.add('KeyW'); if (s > 3 && s < 5) I.keys.add('KeyD'); if (s > 9 && s < 12) I.keys.add('KeyS');
  I.mouseX = 800 + 300 * Math.sin(s * 0.7); I.mouseY = 500 + 200 * Math.cos(s * 0.5);
  if (f === 400) I.pressed.add('KeyE');
  if (f === 470 || f === 500 || f === 530) { I.clicked |= 1; I.buttons |= 1; } else I.buttons = 0;
  if (f === 700) I.pressed.add('KeyC');
}
async function run(secs: number, tap: boolean, query: string | null, levelFile: string | null = null): Promise<{ W: any; buf: any; hashes: string[] }> {
  Math.random = mulberry32(0xbeef);
  const W = await standUp({ seed: 7, ...(levelFile ? { level: await (await import('./rerun.ts')).levelOf(levelFile) } : {}) });
  if (query) { const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`); const r = startFromQuery(W.game, query); if (r.why) console.log(`  (scenario '${query}' refused: ${r.why})`); }
  const buf = new ReplayBuffer();
  if (tap) { buf.tape.born(W.game); W.game.replayTap = (dt: number, cmd?: unknown) => { if (cmd) buf.top(W.game, dt, cmd); else buf.tick(W.game, dt, W.cam); }; }
  const hashes: string[] = [];
  for (let f = 0; f < secs * 60; f++) { hands(W, f); W.step(1 / 60); if ((f + 1) % 60 === 0) hashes.push(hashDigest(digestGame(W.game, true))); }
  return { W, buf, hashes };
}

console.log('\n=== replay buffer ===');
// ---- ring
{ const keep = REPLAY.seconds; REPLAY.seconds = 10;
  const { W, buf } = await run(15, true, null); const file = buf.file(W.game, 'probe');
  REPLAY.seconds = keep;
  const bad = F.checkReplay(file);
  check(bad === '', `ring: the file passes its own check${bad ? ` — ${bad}` : ''}`);
  check(file.seconds >= 10 && file.seconds <= 10 + REPLAY.keySecs + 0.1, `ring: asked to keep 10 s, holds ${file.seconds} s (10 ‥ ${10 + REPLAY.keySecs})`);
  check(file.samples[0].k === 1 && file.samples[0].r === 0, `ring: the head is a keyframe at 0 (r ${file.samples[0].r})`);
  const D = F.decode(file); const n0 = D.men[0].filter(Boolean).length, nMen = 1 + W.game.guards.length + W.game.civilians.length;
  check(n0 === nMen, `ring: the first sample places every man there is (${n0} of ${nMen})`);
  check(file.events.every((e: any) => e.r >= 0) && file.lines.every((l: any) => l.r >= 0), 'ring: nothing in the file is from before its head'); }
// ---- content
{ const secs = Number(val('secs') ?? 25), query = val('scenario') ?? null, levelFile = val('level') ?? null, judged = !query && !levelFile;
  const said = (ok: boolean, text: string) => judged ? check(ok, text) : console.log(`  (${ok ? 'yes' : 'no'}: ${text})`);
  const { W, buf } = await run(secs, true, query, levelFile);
  const fp = levelFingerprint(W.game);
  const file = buf.file(W.game, 'probe', { commit: 'probe', url: query ? `?${query}` : '', fp, ...(levelFile ? { levelFile } : {}) });
  const D = F.decode(file); const json = JSON.stringify(file);
  check(F.checkReplay(file) === '', 'content: the file passes its own check');
  const sam = file.cast.findIndex((c: any) => c.kind === 'player'); const s0 = D.men[0][sam], s1 = D.men[D.men.length - 1][sam];
  said(!!s0 && !!s1 && Math.hypot(s1.x - s0.x, s1.z - s0.z) > 1, `content: Sam walked and the trace says so (${s0 ? `(${s0.x}, ${s0.z})` : '?'} → ${s1 ? `(${s1.x}, ${s1.z})` : '?'})`);
  const real = W.game.player.char.pos; check(!!s1 && Math.abs(s1.x - real[0]) < 0.011 && Math.abs(s1.z - real[2]) < 0.011, 'content: the last sample is where Sam stands now, to the centimetre');
  const routed = file.samples.some((q: any) => (q.x ?? []).some(([, d]: [number, any]) => d.path && d.path.length >= 2));
  said(routed, 'content: a walking man carries the route ahead of him');
  const shot = file.events.find((e: any) => e.k === 'shot' && e.to);
  if (shot) check(shot.who === sam, `content: Sam's round is filed with where it went and whose it was (who ${shot.who})`); else console.log(`  (no round fired in this run — events: ${[...new Set(file.events.map((e: any) => e.k))].join(', ') || 'none'})`);
  check(file.map.boxes.length >= 6 * 20 && file.map.doors.length === W.game.doors.list.length, `content: the map holds ${file.map.boxes.length / 6} solids and ${file.map.doors.length} doors`);
  console.log(`  ${file.samples.length} samples · ${file.cast.length} cast · ${file.events.length} events · ${file.lines.length} lines · ${(json.length / 1024).toFixed(0)} KB of JSON for ${file.seconds} s (${(json.length / Math.max(1, file.seconds) / 1024).toFixed(1)} KB/s) · ${(Bun.gzipSync(new TextEncoder().encode(json)).length / 1024).toFixed(0)} KB gzipped`);
  // ---- notes
  file.notes.push({ id: 'n1', r: file.seconds / 2, t: 0, n: 0, pos: [12.5, 14.25], text: 'he walks into the wall here', at: new Date().toISOString() });
  const back = JSON.parse(JSON.stringify(file));
  check(F.checkReplay(back) === '' && back.notes.length === 1 && back.notes[0].text === 'he walks into the wall here' && back.notes[0].pos[1] === 14.25, 'notes: a note survives the trip through JSON');
  const out = val('write'); if (out) { writeFileSync(out, JSON.stringify(back)); console.log(`  wrote ${out}`); } }

// ---- fight: the man a room fight's teammate has his gun on is on his row (the room fight keeps him on its own record, not on the guard AI's)
{ const { W, buf } = await run(4, true, 'roomclear&squad4');
  const g = W.game, mate = g.guards.find((x: any) => x.unit && x.team === 'operative' && x.state !== 'dead'), foe = g.guards.find((x: any) => x.team === 'hostile' && x.state !== 'dead'), O = mate ? g.fight.opOf(mate) : null;
  check(!!O && !!foe && g.fight.live, `fight: a room clear is up with a teammate on the room fight's books (${mate?.callsign ?? '?'})`);
  if (O && foe) { const was = O.tgt; O.tgt = foe; for (let i = 0; i < 6; i++) buf.tick(g, 1 / 60, W.cam); O.tgt = was;   // (set between two steps and put back: the room fight never sees it)
    const D = F.decode(buf.file(g, 'probe', { url: '?roomclear&squad4', fp: levelFingerprint(g) })), ci = D.file.cast.findIndex((c: any) => c.name === mate.callsign), cf = D.file.cast.findIndex((c: any) => c.name === foe.callsign), last = D.men[D.men.length - 1][ci];
    check(ci >= 0 && cf >= 0 && last?.do?.foe === cf, `fight: his row says whom the gun is on (${last?.do ? F.doWords(last.do, D.file.cast) : 'no row'})`); } }

// ---- tape: what reached the game from OUTSIDE a step (a checkpoint of this run loaded from a card, a restart that puts the game back to its start, clock, die and
// all, autosave off) is a row each, and the walk of them is the run again
{ const SS = await import(`${ROOT}/src/game/savestate.ts`), { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`), { rerun } = await import('./rerun.ts'), Q = '?roomclear&squad4&freshstart=0&seed=7', was = SS.SAVETIMELINE.on;
  try {
    Math.random = mulberry32(0xa1a); const W: any = await standUp({ seed: 7, search: Q }), g = W.game; startFromQuery(g, Q); SS.SAVETIMELINE.on = true; g.godMode = true; g.rustHost.keepStart();
    const buf = new ReplayBuffer(); buf.tape.born(g); g.replayTap = (dt: number, cmd?: unknown) => { if (cmd) buf.top(g, dt, cmd); else buf.tick(g, dt, W.cam); };
    let f = 0; const go = (n: number, pre?: () => void) => { for (let i = 0; i < n; i++, f++) { const I = W.input; I.keys.clear(); I.buttons = 0; I.mouseX = 800; I.mouseY = 520; if (f % 240 < 150) I.keys.add('KeyW'); pre?.(); W.step(1 / 60); } };
    go(200); go(1, () => W.input.pressed.add('F5')); go(99); const key = (g.saves.timeline() as any[]).find(x => x.mem)?.key, t0 = g.time, why = key ? g.saves.load(g, key) : 'no save in memory', loaded = g.time;
    if (process.env.LOADROW_BREAK === '1') g.saves.loadedKey = null;
    go(120); g.rustHost.toStart(); if (process.env.ANEW_BREAK === '1') g.saves.stats.restarts--; const back = g.time; go(100); g.saves.enabled = false; go(140); g.replayTap = null;
    const N = f, dEnd = hashDigest(digestGame(g, true)), file = JSON.parse(JSON.stringify(buf.file(g, 'probe', { url: Q, fp: levelFingerprint(g) }))), rows = (file.rerun?.x ?? []) as any[], x = rows.filter(q => q[1] === 'restart' || q[1] === 'load').map(q => q.join(':')).join(' ');
    check(why === null && loaded < t0 && back < 0.1 && file.rerun?.ticks === N && x === `300:load:${key} 420:restart:start` && F.lastRun(file) > 6, `tape: a checkpoint loaded and a restart to the start are a row each, the tape alive (clock ${t0.toFixed(2)} to ${loaded.toFixed(2)}, then ${back.toFixed(2)}; tape: ${file.rerun?.dead || 'alive'}; rows ${x || 'none'}${why ? `; load: ${why}` : ''})`);
    const R = await rerun(file, { toN: N }), same = !!R.W && hashDigest(digestGame(R.W.game, true)) === dEnd, off = (R.W?.game as any)?.saves?.enabled === false;
    check(!R.refuse && !!R.report && R.report.compared > 100 && !R.report.drift && same && off, `tape: its walk is the run again, to the digest at step ${N}, its autosave as taped${R.refuse ? ` — REFUSED: ${R.refuse}` : ` (${R.report?.compared} trace samples, worst ${R.report?.worst?.metres?.toFixed(3)} m, drift ${R.report?.drift ? 'YES' : 'none'}, digest ${same ? 'equal' : 'DIFFERS'})`}`);
    const other = (rr: unknown) => rerun({ ...file, rerun: rr } as any), ended = await other({ dead: 'a scenario was started or quit from a menu', tick: 12 }), v1 = await other({ ...file.rerun, v: 1 });
    const onFile = (o: object) => other({ ...file.rerun, recipe: { ...file.rerun?.recipe, ...o } }), second = await onFile({ levelFile: '/levels/hall-gwot.json' }), plainFile = await onFile({ search: '?look=plain' });
    check(/level file that has since been replaced/.test(second.refuse) && /level file that has since been replaced/.test(plainFile.refuse), `tape: one on a level file that has gone is refused in words (${second.refuse.slice(0, 110)}…)`);
    check(file.rerun?.v === F.TAPE_VERSION && F.tapeWhy(file) === '' && /from a menu \(step 12\)/.test(ended.refuse) && /a different version of the game/.test(v1.refuse), `tape: one that ended and one of another version are refused in words (${ended.refuse.slice(0, 110)}… · ${v1.refuse.slice(0, 90)}…)`);
  } finally { SS.SAVETIMELINE.on = was; } }

// ---- crash: an error that repeats makes ONE file a minute and three a page load, in a browser that keeps no shelf (bun has no IndexedDB: the shelf's own branch
// stamps through the same line of ReplayKey.put). With no shelf the file is SENT to the build's backend, never downloaded: the probe stands in for the backend ----
{ const G: any = globalThis, kept: Record<string, any> = {}, random = Math.random;
  let sends = 0, clicks = 0, W: any = null, one = false; const made: any[] = [];   // (every element the key's file makes: the tag it says things on. A link that is clicked would be a download, and there is none)
  let answer: 'sent' | 'failed' | 'off' | 'no' = 'sent', slow = 0; const sent: { name: string; size: number; meta: Record<string, string> }[] = [];
  const stub: Record<string, any> = {
    window: { addEventListener() { /* the key and the error listeners: the probe calls crashed() itself */ } },
    location: { search: '' }, localStorage: { getItem: () => null, setItem() { /* */ }, removeItem() { /* */ } },
    document: { baseURI: 'http://localhost/', body: { appendChild() { /* */ } }, createElement: () => { const el: any = { style: {}, textContent: '', click() { clicks++; }, remove() { /* */ } }; made.push(el); return el; }, querySelector: (q: string) => one && /tps-one-page/.test(q) ? {} : null },
  };
  try {
    W = await standUp(); Math.random = mulberry32(5);   // (the game stands as in every other section; the page's globals are only up round the key's own calls)
    for (const k of Object.keys(stub)) { kept[k] = Object.getOwnPropertyDescriptor(G, k); Object.defineProperty(G, k, { value: stub[k], configurable: true, writable: true }); }
    const { ReplayKey } = await import(`${ROOT}/src/ui/replaykey.ts`);
    const notes: string[] = [];
    const key = new ReplayKey(W.game, W.cam, (s: string) => notes.push(s)), K = key as any;
    const out0 = key.out, sends0 = key.sends;   // (the page's own two: asked once below, with no backend)
    key.out = (name: string, blob: Blob, meta: Record<string, string>) => { sends++; sent.push({ name, size: blob.size, meta }); const a = answer; return new Promise(r => slow ? setTimeout(() => r(a), slow) : r(a)); };
    key.sends = async () => answer !== 'off';
    const settle = async () => { for (let i = 0; i < 400 && K.crashing; i++) await new Promise(r => setTimeout(r, 5)); await new Promise(r => setTimeout(r, 5)); };
    const minuteOn = () => { K.crashAt = K.crashTry = performance.now() - 61000; };
    for (let i = 0; i < 4; i++) { key.crashed('before any play'); await settle(); }
    check(sends === 0 && key.last === null && notes.filter(n => /nothing played/.test(n)).length === 1, `crash: an error before any play makes no file and says so once, not every time (${notes.length} line)`);
    for (let f = 0; f < 120; f++) { hands(W, f); W.step(1 / 60); }
    K.crashTry = -Infinity; key.crashed(new Error('boom')); key.crashed(new Error('boom')); await settle();
    check(sends === 1 && key.last?.id === null && /^error: boom/.test(key.last?.file.why ?? ''), `crash: with no shelf the error's replay is sent once (${sends}) for two errors in one tick, named for the error`);
    check(/hit an error/.test(notes[notes.length - 1] ?? ''), `crash: the message line says an error caused the replay ("${(notes[notes.length - 1] ?? '').slice(0, 48)}…")`);
    for (let i = 0; i < 5; i++) { W.step(1 / 60); key.crashed(new Error('boom')); await settle(); }
    check(sends === 1, `crash: the same error five more times inside the minute sends nothing more (${sends} in all)`);
    minuteOn(); key.crashed('later'); await settle();
    check(sends === 2, `crash: a minute on, the next error saves again (${sends} in all)`);
    for (let i = 0; i < 4; i++) { minuteOn(); key.crashed('for ever'); await settle(); }
    check(sends === 3, `crash: an error that never stops makes three files in a page load and no more (${sends} in all)`);
    const id = await key.save('key'); check(id === null && sends === 4, 'crash: F7 itself still saves after that');
    one = true; const id2 = await key.save('key'); one = false;
    check(id2 === null && sends === 5 && /^The replay of the last .* was sent\./.test(notes[notes.length - 1] ?? ''), `crash: on the one-file build, which ships no viewer page, F7 sends the file and says so ("${(notes[notes.length - 1] ?? '').slice(0, 60)}")`);
    K.crashes = 0; minuteOn(); one = true; answer = 'off'; const n0 = notes.length;
    for (let i = 0; i < 3; i++) { key.crashed(new Error('one page')); await settle(); }
    one = false; answer = 'sent';
    check(sends === 5 && notes.length === n0 + 1 && !/holds the page's error/.test(notes[n0] ?? '') && /^The page hit an error, and the replay of the last .* wasn't kept, because this build can't save or send one\./.test(notes[n0] ?? '') && /^error: one page/.test(key.last?.file.why ?? ''), `crash: on a one-file build with no backend an error's replay is not even packed, and the line says so once, not three times ("${(notes[n0] ?? '').slice(0, 96)}")`);
    // the error is a LINE in the ring at the play moment it came: every file made while the ring holds that moment carries it, whatever saved the file
    const texts = (f: any) => F.errorMarks(f).map((e: any) => e.text);
    { const f = key.file('probe'), E = F.errorMarks(f);
      check(texts(f).join('|') === 'before any play|boom|later|for ever|one page' && E[0].r === 0 && Math.abs(E[1].r - 2) < 0.1, `crash: each error is one line in the ring at the moment it came, however often it repeated (${texts(f).join(' · ')}; boom at ${E[1]?.r.toFixed(2)} s)`); }
    one = true; const id3 = await key.save('key'), l3 = notes[notes.length - 1] ?? ''; one = false;
    check(id3 === null && sends === 6 && key.last?.file.why === 'key' && texts(key.last?.file).includes('one page') && /holds the page's error, at 0:0/.test(l3), `crash: on the one-file build the F7 that follows an error sends a file that holds the error's line, and says so ("${l3.slice(-44)}")`);
    const d0 = sends, m0 = notes.length; key.crashed(new Error('swallowed')); await settle();
    check(sends === d0 && notes.length === m0 && texts(key.file('probe')).pop() === 'swallowed', 'crash: an error the budget lets save nothing is still in the next file');
    // the lines' own rule is on the PLAY clock (the buffer's): nothing below waits on, or moves, the wall's
    for (let i = 0; i < 10; i++) { key.crashed(new Error('alt A')); key.crashed(new Error('alt B')); }
    { const t = texts(key.file('probe')); check(t.length === 8 && t.slice(-2).join('|') === 'alt A|alt B', `crash: two errors that take turns are two lines, not twenty (${t.length} lines in all)`); }
    for (let i = 0; i < 20; i++) key.crashed(new Error(`flood ${i}`));
    { const f = key.file('probe'), t = texts(f), R = F.decode(f).r, end = R.length ? R[R.length - 1] : 0;
      check(t.length === 8 && t[0] === 'before any play' && t[7] === 'flood 19', `crash: errors that never repeat a word hold eight lines of a minute's play, the first of them and the NEWEST (${t.length}: … ${t.slice(-2).join(' · ')})`);
      check(f.lines.every((l: any) => l.r >= 0 && l.r <= end), `crash: no line of a file lies before its first sample or past its last (${f.lines.length} lines, the last sample at ${end.toFixed(2)} s)`); }
    await settle();
    const keep = REPLAY.seconds; REPLAY.seconds = 6;
    try { for (let f = 0; f < 600; f++) { hands(W, f); W.step(1 / 60); } } finally { REPLAY.seconds = keep; }
    const id4 = await key.save('key'), l4 = notes[notes.length - 1] ?? '';
    check(id4 === null && texts(key.last?.file).length === 0 && !/holds the page/.test(l4), `crash: once the ring has rolled past the errors a file holds none of them and F7 does not say it does (${texts(key.last?.file).length} lines in ${key.last?.file.seconds.toFixed(0)} s)`);
    { const buf = key.buf!, rs = () => F.errorMarks(key.file('probe')).filter((e: any) => e.text === 'still').map((e: any) => e.r);
      const a = [buf.pageError('still', true), buf.pageError('still', true), buf.pageError('still')];
      check(a.join() === 'true,false,false' && rs().length === 1, 'crash: with the game standing still the same error is one line, asked plainly or by an error about to save a file');
      buf.on = false; const off = buf.pageError('while off'); buf.on = true; const on = buf.pageError('while off');
      (buf as any).broke = 'probe'; const dead = buf.pageError('while stopped'); (buf as any).broke = '';
      check(!off && on && !dead && texts(key.file('probe')).join('|') === 'still|while off', 'crash: a buffer that is off or has stopped itself files no line and says so, and files the same error once it runs again'); }
    // WHAT F7 DID IS SAID ON THE PAGE ITSELF: the note handed in (the message line, the debug read-out) is drawn nowhere on a page that never asked for the read-out,
    // so the key's file keeps a tag of its own — one element, the last line said, shown — and a save that throws or a shelf that never answers still ends in words
    { const { SHELF } = await import(`${ROOT}/src/replay/shelf.ts`);
      const tags = () => made.filter(e => e.id === 'replaytag'), tagNow = () => String(tags()[0]?.textContent ?? ''), last = () => notes[notes.length - 1] ?? '';
      const hadIdb = Object.getOwnPropertyDescriptor(G, 'indexedDB'), idb = (v: any) => Object.defineProperty(G, 'indexedDB', { value: v, configurable: true, writable: true });
      // a shelf that answers, as far as shelf.ts asks: two stores, requests that succeed a microtask on, a transaction that completes a task on
      const shelf = () => { const rows = new Map<number, any>(), bodies = new Map<number, any>(); let next = 1, opens = 0, closes = 0;
        const rq = (run: () => any) => { const r: any = {}; queueMicrotask(() => { r.result = run(); r.onsuccess?.(); }); return r; };
        const store = (m: Map<number, any>) => ({ add: (v: any) => rq(() => { const id = next++; m.set(id, { ...v, id }); return id; }), put: (v: any, k?: number) => rq(() => { m.set(k ?? v.id, v); return k; }), getAll: () => rq(() => [...m.values()]), get: (k: number) => rq(() => m.get(k)), delete: (k: number) => rq(() => { m.delete(k); }) });
        const db = { objectStoreNames: [], transaction: () => { const tx: any = { objectStore: (n: string) => store(n === 'rows' ? rows : bodies) }; setTimeout(() => tx.oncomplete?.(), 0); return tx; }, close() { closes++; } };
        return { rows, bodies, get opens() { return opens; }, get closes() { return closes; }, open: (late = 0) => { opens++; const r: any = { result: db }; setTimeout(() => r.onsuccess?.(), late); return r; } }; };
      const meta = K.meta, nt = K.note, ms = SHELF.openMs, buf = key.buf as any;
      try {
        const id5 = await key.save('key'), l5 = last(), T = tags();
        check(id5 === null && T.length === 1 && T[0].textContent === l5 && /^The replay of the last/.test(l5) && T[0].style.opacity === '0.9' && /position:fixed/.test(T[0].style.cssText ?? '') && /background:/.test(T[0].style.cssText ?? ''), `say: what F7 did is on a tag of the page's own, one element however many saves ("${tagNow().slice(0, 40)}…")`);
        K.meta = () => { throw new Error('meta broke'); };
        let threw = false, id6: number | null = 0; const d6 = sends; try { id6 = await key.save('key'); } catch { threw = true; }
        check(!threw && id6 === null && sends === d6 && /could not be saved: meta broke/.test(tagNow()) && /could not be saved/.test(last()), `say: a save that throws makes no file, rejects nothing and says why ("${tagNow().slice(0, 48)}")`);
        { const before = tagNow(), n6 = notes.length; K.crashes = 0; K.crashing = false; for (let i = 0; i < 3; i++) { K.crashAt = K.crashTry = -Infinity; key.crashed('while the save throws'); await settle(); }
          check(notes.length === n6 + 3 && /could not be saved/.test(last()) && tagNow() === before && K.crashes === 0, 'say: an ERROR whose save makes no file goes to the note alone each time it is asked again: the tag is what a key press did'); }
        K.meta = meta;
        K.note = () => { throw new Error('no message line'); }; const d7 = sends; await key.save('key'); K.note = nt;
        check(sends === d7 + 1 && /^The replay of the last/.test(tagNow()), 'say: the tag has the words though the page\'s own note threw');
        buf.broke = 'probe stopped it'; await key.save('key'); const l8 = tagNow(); buf.broke = '';
        check(/^The replay of the last/.test(l8) && /stopped itself earlier \(probe stopped it\)/.test(l8), 'say: a buffer that stopped itself hands over what it held and the ONE line shown says both');
        buf.on = false; await key.save('key'); const l8b = tagNow(); buf.on = true;
        check(/^The replay of the last/.test(l8b) && /switched off/.test(l8b), 'say: a buffer switched off from the console hands over what it held and says it is not running');
        // WHAT BECAME OF A FILE WITH NO SHELF: the build's backend has four answers and each has its own words; nothing is a download
        { const say1 = async (a: typeof answer, onePage = false, search = '') => { answer = a; one = onePage; G.location.search = search; const s0 = sends; await key.save('key'); one = false; answer = 'sent'; G.location.search = ''; return { l: tagNow(), n: sends - s0 }; };
          const BACK = '?cloud=http://127.0.0.1:3210', tail = /holds the page's error/;
          const A = await say1('sent'), B = await say1('failed'), C = await say1('no'), D = await say1('off'), E = await say1('off', true), D2 = await say1('off', false, BACK), E2 = await say1('off', true, BACK);
          check(A.n === 1 && /^The replay of the last [\d:.]+ was sent\. This browser won't keep replays, so you can't open it here\./.test(A.l) && tail.test(A.l), `send: a file the backend took says it was sent, and why nothing opens here ("${A.l.slice(0, 96)}")`);
          check(B.n === 1 && /^The replay of the last [\d:.]+ couldn't be sent, so it wasn't kept\.$/.test(B.l), `send: one the backend did not take says it was not kept, and nothing more of a replay that is nowhere ("${B.l}")`);
          check(C.n === 1 && /^The replay of the last [\d:.]+ wasn't kept, because it's too big to send\.$/.test(C.l), `send: one past the backend's bound says so ("${C.l}")`);
          check(D.n === 0 && /^The replay of the last [\d:.]+ wasn't kept, because this browser won't store it and this build has nowhere to send it\.$/.test(D.l), `send: with no backend and no shelf nothing is packed, and the line names both ("${D.l}")`);
          check(E.n === 0 && /^The replay of the last [\d:.]+ wasn't kept, because this build can't save or send one\.$/.test(E.l), `send: the one-file build with no backend says so of itself ("${E.l}")`);
          check(D2.n === 0 && /^The replay of the last [\d:.]+ wasn't kept, because this browser won't store it and isn't sending replays\.$/.test(D2.l), `send: with a backend and sending off the line blames the sending, not the build ("${D2.l}")`);
          check(E2.n === 0 && /^The replay of the last [\d:.]+ wasn't kept, because this build can't save one and this browser isn't sending replays\.$/.test(E2.l), `send: the one-file build with a backend and sending off says each half's own reason ("${E2.l}")`);
          const s0 = sent.length; await key.save('key'); const f = sent[sent.length - 1];
          check(sent.length === s0 + 1 && /^twopointsix-replay-.*\.json$/.test(f?.name ?? '') && f.size > 1000 && key.last?.id === null && f.name === F.replayName(key.last!.file) && f.meta.savedAt === key.last!.file.savedAt && f.meta.why === 'key', `send: what goes is the replay's one file under the replay's own name, plain JSON, with when and why it was saved beside it (${f?.name}, ${((f?.size ?? 0) / 1024).toFixed(0)} KB)`);
          // the page's own two answers with no backend: nothing sends, and neither throws. With a backend on the address the replays' own word still stops it
          { const can0 = await sends0(), end0 = await out0('probe.json', new Blob(['{}']), {});
            G.location.search = BACK; const can1 = await sends0(); G.location.search = `${BACK}&cloudtapes=0`; const can2 = await sends0(); G.location.search = '';
            check(can0 === false && end0 === 'off' && can1 === true && can2 === false, `send: the page's own road says 'off' with no backend, would send to one the address names, and not where the replays' word is off (${can0} · ${end0} · ${can1} · ${can2})`); }
          // an answer that takes its time: the key is let go with "is waiting to be sent", and the answer's own line follows by itself
          slow = 3000; const t0 = performance.now(), idS = await key.save('key'), waited = performance.now() - t0, l1 = tagNow(); await K.sending; const l2 = tagNow(); slow = 0;
          check(idS === null && waited < 2500 && /^The replay of the last [\d:.]+ is waiting to be sent\./.test(l1) && /^The replay of the last [\d:.]+ was sent\./.test(l2), `send: a slow answer does not hold the key: "${l1.slice(0, 36)}" after ${waited.toFixed(0)} ms, then the answer's own line ("${l2.slice(0, 34)}")`);
          key.out = () => Promise.reject(new Error('the door threw')); await key.save('key'); const l3 = tagNow();
          key.out = () => { throw new Error('the door threw at once'); }; let threw = false; try { await key.save('key'); } catch { threw = true; } const l4 = tagNow();
          key.sends = () => Promise.reject(new Error('the question threw')); try { await key.save('key'); } catch { threw = true; } const l4b = tagNow(); key.sends = async () => answer !== 'off';
          key.out = (name: string, blob: Blob, meta: Record<string, string>) => { sends++; sent.push({ name, size: blob.size, meta }); const a = answer; return new Promise(r => slow ? setTimeout(() => r(a), slow) : r(a)); };
          check(!threw && /couldn't be sent, so it wasn't kept\./.test(l3) && /couldn't be sent, so it wasn't kept\./.test(l4) && /couldn't be sent, so it wasn't kept\./.test(l4b), 'send: a door that rejects or throws, or a question that does, is a file that was not sent, said so, and the save rejects nothing');
          const h0 = sends, name = key.hook().send(); await K.sending;
          check(typeof name === 'string' && sends === h0 + 1 && /^The replay of the last [\d:.]+ was sent\./.test(tagNow()) && !('download' in key.hook()), `send: the console's own send (__replay.send()) goes the same way, and the hook has no download (${name})`); }
        // a shelf that never answers (no event of the open request ever fires): waited on for the shelf's own time, then the file is sent — and one that answers LATE is closed unused
        SHELF.openMs = 60;
        { let opened = 0; idb({ open: () => { opened++; return {}; } }); const t0 = performance.now(), d9 = sends, id9 = await key.save('key'), waited = performance.now() - t0;
          check(id9 === null && opened === 1 && sends === d9 + 1 && waited >= 50 && /^The replay of the last .* was sent\./.test(tagNow()), `say: a shelf that never answers is waited on for the shelf's own time, then the file is sent and the line says so (${waited.toFixed(0)} ms of a 60 ms wait)`); }
        { const S = shelf(); idb({ open: () => S.open(150) }); const d10 = sends, id10 = await key.save('key'); await new Promise(r => setTimeout(r, 220));
          check(id10 === null && sends === d10 + 1 && S.rows.size === 0 && S.closes === 1, `say: a shelf that answers after the wait is closed unused: the file that was sent is the only copy (${S.rows.size} rows, ${S.closes} close)`); }
        SHELF.openMs = ms;
        // a shelf that answers: the line a dev page shows — saved, nothing sent by the key — and it carries what the other line carries
        { const S = shelf(); idb({ open: () => S.open() }); const d11 = sends, id11 = await key.save('key'), l11 = tagNow();
          check(id11 === 1 && sends === d11 && S.rows.size === 1 && typeof S.bodies.get(1) === 'string' && /^Replay saved: the last .* of play\. Shift\+F7 leaves the game for the viewer\./.test(l11) && l11 === last() && key.last?.id === 1, `say: with a shelf F7 shelves the file, the key sends nothing and says where it went ("${l11.slice(0, 44)}…")`);
          buf.broke = 'probe stopped it'; const id12 = await key.save('key'), l12 = tagNow();
          const r13 = await K.put('error: probe'), l13 = tagNow(); buf.broke = '';
          check(id12 === 2 && /^Replay saved/.test(l12) && /stopped itself earlier/.test(l12) && r13.made && r13.id === 3 && /^The page hit an error: replay of the last .* saved\./.test(l13) && /stopped itself earlier/.test(l13) && sends === d11, 'say: on the shelf too, the key\'s line and an error\'s line both say the buffer had stopped itself'); }
      } finally { K.meta = meta; K.note = nt; SHELF.openMs = ms; buf.broke = ''; buf.on = true; if (hadIdb) Object.defineProperty(G, 'indexedDB', hadIdb); else delete G.indexedDB; }
      // no buffer at all (?noreplay): F7 is not silent either
      { const heard: Array<(e: any) => void> = [], w = G.window, loc = G.location;
        G.window = { addEventListener: (k: string, f: (e: any) => void) => { if (k === 'keydown') heard.push(f); } }; G.location = { search: '?noreplay' };
        try { const said: string[] = [], off = new ReplayKey(W.game, W.cam, (s: string) => said.push(s)), n0 = made.length;
          for (const f of heard) { f({ code: 'F7', target: null }); f({ code: 'F8', target: null }); f({ code: 'F7', repeat: true, target: null }); }
          const t = made.slice(n0).filter(e => e.id === 'replaytag');
          check(off.buf === null && heard.length === 1 && said.length === 1 && /buffer is off/.test(said[0]) && t.length === 1 && t[0].textContent === said[0], `say: with the buffer off F7 says so, once a press, and nothing else is listened for ("${said[0] ?? ''}")`);
        } finally { G.window = w; G.location = loc; } } }
    check(clicks === 0 && sends > 10, `send: in all of that no link was clicked: nothing was a download (${clicks} clicks, ${sends} files sent)`);
  } finally { Math.random = random; if (W) W.game.replayTap = null; for (const k of Object.keys(stub)) { if (kept[k]) Object.defineProperty(G, k, kept[k]); else delete G[k]; } } }

console.log(`\n=== replay buffer: ${fails ? `${fails} FAIL` : 'ALL PASS'} ===`);
process.exit(fails ? 1 : 0);
