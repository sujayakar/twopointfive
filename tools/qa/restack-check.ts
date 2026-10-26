// THE KEY ON A STACK IS 'STACK UP' AGAIN (src/game/autopilot.ts autoPick, AUTOPILOT.restack; orders.ts again). The maintainer, 10-26: "let's actually
// remove that second tap of Q thing. tapping Q just always sends the order, so tapping Q on a door they're stacked up on would just resend the stack up
// command, which would then have them say ("stacked up" or "we're ready to go" or "we're already stacked up" or something like that)".
//   bun run tools/qa/restack-check.ts            (also `probes.ts restack`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/restack-check.ts      — the same lines on the page's stand
// PASS / FAIL each, headless: the room clear on the office with its squad of four under god mode, ONE deal (seed 5, plainentry-check's: the rule has no
// die in it). The check raises the quick key's autopilot, the plain entry and the rule itself, so both stands ask the same lines. Every press is a
// one-frame press of the quick key, or of the word's key, with the harness cursor on the door.
//   1. stacked and set on a door: the squad's pick for the key is 'stack up' for the four, and the HUD's line for the key says so.
//   2. the key on that door: the panel shows the order, ONE man answers with a line of RESTACK_LINES, no row is written, no door is worked, and a
//      second later every man is on the mark he had, still set.
//   3. the key again: another man answers with the other line, and again nobody moves.
//   4. the WORD's key: they go in by open and clear, all four — the word's key is the way in.
//   5. stacked there again, the same asked for a goal's men (orders.ts quickFor): answered, nothing given.
//   6. CONTROL — the rule off, the same stack: the key sends them in, as it did.
//   7. CONTROL (plumbing) — a recording from before the key replays with it off; the frozen stand holds it off, the page's has it on, FROZEN_HOLDS has its row.
// RESTACK_BREAK=1 = the switch left off and the same lines asked: 1, 2, 3 and 5 must fail (4 and 6 fall with them: the men went in at line 2).
// (line 5 gives the men a second and a half on their marks first: a man just arrived is still shuffling his last centimetres.)
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';
const AP: any = await import(`${ROOT}/src/game/autopilot.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const { applyKnobs, knobsSnapshot } = await import(`${ROOT}/src/game/journal.ts`);
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const { AUTOPILOT, AUTOPILOT_STATS, RESTACK_LINES, autoPick, autoHint } = AP;
const DT = 1 / 60, SEED = 5, BREAK = process.env.RESTACK_BREAK === '1', VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

export async function restackCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rnd0 = Math.random, squad0 = NIGHTBREACH.squad;
  console.log(`The key on a stack is 'stack up' again (src/game/autopilot.ts autoPick, AUTOPILOT.restack)${BREAK ? ' — RESTACK_BREAK=1: the switch left off, lines 1, 2, 3 and 5 must fail' : ''}`);
  try {
    const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { };
    let W: any, why: any, stood = false;
    try { Object.assign(NIGHTBREACH, { squad: 4 }); Math.random = seeded(SEED); W = await standUp(); stood = !!AUTOPILOT.restack; AUTOPILOT.on = true; AUTOPILOT.plain = true; AUTOPILOT.restack = !BREAK; W.game.godMode = true; why = W.game.roomClear.start(SEED); }
    finally { console.log = log; console.warn = warn; Object.assign(NIGHTBREACH, { squad: squad0 }); }
    const g = W.game, O = g.orders, T = g.doorTools, I = W.input;
    const quiet = <X,>(f: () => X): X => { if (VERBOSE) return f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
    const step = (n = 1) => quiet(() => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } });
    const until = (pred: () => boolean, secs: number) => { for (let i = 0; i < secs * 60 && !pred(); i++) step(); return pred(); };
    step(30); for (const m of O.squad()) m.god = true;
    const all = () => O.men('all'), N = all().length, stackDef = O.defs.find((d: any) => d.id === 'stack');
    if (why || N !== 4) { line(false, 'the room clear on the office with its squad of four', String(why ?? `${N} men`)); return false; }
    /** a point of that door's doorway the cursor on which IS that door (null: none) */
    const aimOf = (d: any): number[] | null => { const c = d.frameCentre, th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
      for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { const p = [c[0] + nx * k, y, c[2] + nz * k]; W.cursorAt(p); step(2); const h = O.hover?.(); if (h?.kind === 'door' && h.door === d) return p; }
      return null; };
    // the door: the nearest whose far room nobody has stood in, that the four can stack on and the cursor can be put on
    const s = g.player.char.pos; let D: any = null, t: any = null;
    for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, s) - dist(b.frameCentre, s))) { const x = O.targetAt(d.frameCentre); if (x.kind !== 'door' || x.door !== d) continue;
      const far = g.cleared.roomBy(d, x.side > 0 ? -1 : 1); if (!far || g.cleared.isKnown(far)) continue; if (O.whyNot(stackDef, O.forMen(x, all()), all()) || !aimOf(d)) continue; D = d; t = x; break; }
    if (!D) { line(false, 'a door to stack on with a room behind it nobody has stood in', 'none'); return false; }
    const stacked = () => all().every((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.door === D && O.of(m)?.arrived);
    /** all four stacked and set on the door, whatever went before (true: they are) */
    const stack = (): boolean => { until(() => T.ops.length === 0, 60); O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); step(60); O.give('stack', O.forMen(O.targetAt(D.frameCentre), all()), 'all'); return until(stacked, 40) && (step(20), stacked()); };
    const opOn = () => T.ops.find((o: any) => o.door === D), far = () => g.cleared.roomBy(D, t.side > 0 ? -1 : 1), unknown = () => !!far() && !g.cleared.isKnown(far());

    const said: [any, string][] = [], shown: string[] = [], say0 = g.say.bind(g), msg0 = g.msg.bind(g);
    g.say = (who: any, text: string, ...r: any[]) => { said.push([who, text]); return say0(who, text, ...r); }; g.msg = (text: string, ...r: any[]) => { shown.push(text); return msg0(text, ...r); };
    const marks = () => all().map((m: any) => [m.char.pos[0], 0, m.char.pos[2]]), moved = (a: number[][]) => Math.max(...marks().map((p: number[], i: number) => dist(p, a[i])));
    /** one press of the quick key on the door, then a second of the game: what was said and shown, what was written, what the door tools hold, how far anybody went */
    const press = () => { const on = !!aimOf(D), n0 = O.log.length, ops0 = T.ops.length, at = marks(), posts = all().map((m: any) => O.of(m)?.post), s0 = said.length, m0 = shown.length, k0 = AUTOPILOT_STATS.again;
      I.pressed.add(ORDERS.quickKey); step(3); const lines = said.slice(s0), msgs = shown.slice(m0); step(60);
      return { on, rows: O.log.length - n0, ops: T.ops.length - ops0, lines, msgs, far: moved(at), same: all().every((m: any, i: number) => O.of(m)?.post === posts[i]), set: stacked(), counted: AUTOPILOT_STATS.again - k0 }; };
    const ok = (R: ReturnType<typeof press>) => R.on && R.rows === 0 && R.ops === 0 && R.lines.length === 1 && all().includes(R.lines[0][0]) && RESTACK_LINES.includes(R.lines[0][1]) && R.msgs.some(x => /^you: .*stack up/.test(x)) && R.far < 0.05 && R.same && R.set && R.counted === 1;
    const tell = (R: ReturnType<typeof press>) => `the cursor on the door: ${R.on} · rows written ${R.rows} · door tools started ${R.ops} · said: ${R.lines.map(([w, x]) => `${w?.callsign ?? '?'} '${x}'`).join(', ') || 'nothing'} · shown: ${R.msgs.map(x => `'${x}'`).join(', ') || 'nothing'} · the furthest any man went ${R.far.toFixed(2)} m · the marks they had: ${R.same} · still set: ${R.set}`;

    // 1
    T.bangs = 3; const set1 = stack(), pick = autoPick(g, O.targetAt(D.frameCentre)), hint = autoHint(g, O.targetAt(D.frameCentre)) ?? '';
    line(set1 && pick.id === 'stack' && pick.why === 'set' && pick.men?.length === N && /^stack up/.test(hint), '1. stacked and set on a door: the key\'s pick is \'stack up\' for the four, and the HUD\'s line for the key says so',
      `stacked and set: ${set1} · pick ${pick.id} (${pick.why}) for ${pick.men?.length ?? 0} · the HUD's line: '${hint}'`);
    // 2
    const R2 = press(); line(ok(R2), '2. the key on that door: the order is shown, one man answers that they are stacked, no row is written, no door is worked and nobody leaves his mark', tell(R2));
    // 3
    const R3 = press(); line(ok(R3) && R3.lines[0][1] !== R2.lines[0]?.[1] && R3.lines[0][0] !== R2.lines[0]?.[0], '3. the key again: another man answers with the other line, and again nobody moves', tell(R3));
    // 4
    { const set = stacked(), had = new Set<any>(T.ops); I.pressed.add(DOORTOOLS.blowKey); step(3); const op = T.ops.find((o: any) => !had.has(o) && o.door === D);
      line(set && op?.kind === 'openClear' && op.men.length === N, '4. the WORD\'s key: they go in by open and clear, all four', `stacked and set: ${set} · the door tools: ${op?.kind ?? 'nothing'} for ${op?.men?.length ?? 0}`); }
    // 5
    { const set = stack() && (step(90), stacked()), n0 = O.log.length, ops0 = T.ops.length, s0 = said.length, at = marks(), got = O.quickFor(all(), O.targetAt(D.frameCentre)); step(60); const lines = said.slice(s0);
      line(set && got === null && O.log.length === n0 && T.ops.length === ops0 && lines.length === 1 && RESTACK_LINES.includes(lines[0][1]) && moved(at) < 0.05 && stacked(), '5. stacked there again, the same asked for a goal\'s men: answered, nothing given',
        `stacked and set: ${set} · quickFor: ${got} · rows written ${O.log.length - n0} · door tools started ${T.ops.length - ops0} · said: ${lines.map(([w, x]) => `${w?.callsign ?? '?'} '${x}'`).join(', ') || 'nothing'} · the furthest any man went ${moved(at).toFixed(2)} m`); }
    // 6
    { AUTOPILOT.restack = false; const set = stacked(), n0 = O.log.length, on = !!aimOf(D); I.pressed.add(ORDERS.quickKey); step(3); const row = O.log[O.log.length - 1], op = opOn(); AUTOPILOT.restack = !BREAK;
      line(set && on && O.log.length === n0 + 1 && row?.order === 'openClear' && op?.kind === 'openClear', '6. CONTROL — the rule off, the same stack: the key sends them in, as it did', `stacked and set: ${set} · the order: ${row?.order} to ${row?.who?.length ?? 0} · the door tools: ${op?.kind ?? 'nothing'}`); }
    // 7
    { AUTOPILOT.restack = true; const snap = JSON.parse(JSON.stringify(knobsSnapshot(g))), old = JSON.parse(JSON.stringify(snap)); delete old.tables.AUTOPILOT.restack;
      applyKnobs(g, old); const before = AUTOPILOT.restack; applyKnobs(g, snap); const on = AUTOPILOT.restack;
      const shipped = String(process.env.TPS_PROFILE ?? '') === 'shipped', row = FROZEN_HOLDS['AUTOPILOT.restack'];
      line(before === false && on === true && stood === shipped && /TPS_RESTACK=1/.test(row ?? ''), '7. CONTROL (plumbing) — a recording from before the key replays with it off; the stands and the FROZEN_HOLDS row',
        `a file without the key: ${before} · one with it on: ${on} · this stand (${shipped ? 'the page\'s' : 'frozen'}) stood it ${stood} · the row: ${row ?? 'none'}`); }
  } finally { knobsBack(knobs0); Math.random = rnd0; }
  console.log(fails ? `=== restack-check: ${fails} FAIL ===` : '=== restack-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await restackCheck()) ? 0 : 1);
