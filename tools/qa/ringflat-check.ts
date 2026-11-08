// THE SQUAD'S RING IS TWO RINGS DEEP AND NOTHING RIDES THE CURSOR (src/game/cmdring.ts CMDRING.flat / plainHint / bareHint; orders.ts flatRows, seats). The maintainer, 10-23, at a wall with three rings between him and one greyed row: "radial menu tree needs
// some work, way too nested"; "loud and quiet should be an option on squad wheel"; "let's remove the icons on mouse position from squad orders
// entirely". (The squad's corner cut down to its key lines is the page's own layer's, not the order book's: no line here.)
//   bun run tools/qa/ringflat-check.ts            (also `probes.ts ringflat`, in family squadorders)
// PASS / FAIL each, headless: the room clear on the office with its squad of four under god mode, ONE deal (seed 3: the rules have no die in them). The ring goes
// up as a hand puts it up: the key kept down on what the cursor is on.
//   1. the floor's ring: what a tap gives, cover, fall in, hold, 'more' — and 'more' is one ring of the throws with the posture row last; given there,
//      the squad's posture turns.
//   2. a door's ring: the two entries are on the first ring where 'fall in' and 'hold position' were; the rest of its tools are the one ring behind
//      'more', the posture row last; a wedge of the first ring gives its entry.
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';

const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const { AUTOPILOT } = await import(`${ROOT}/src/game/autopilot.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const SIL: any = await import(`${ROOT}/src/game/silent.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

export async function ringflatCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rnd0 = Math.random, squad0 = NIGHTBREACH.squad, K = CMDRING;
  try {
    Object.assign(NIGHTBREACH, { squad: 4 }); Math.random = seeded(3);
    const W: any = await standUp();
    AUTOPILOT.on = true;
    const g = W.game, O = g.orders, input = W.input, KEY = K.key; g.godMode = true; g.roomClear.start(3);
    const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } };
    step(30); for (const m of O.squad()) m.god = true;
    const sam = g.player.char.pos, HOLD = Math.ceil(K.stayHoldSecs * 60) + 2;
    const away = () => { input.keys.delete(KEY); O.cmdRing = null; O.close(); input.pressed.add('Escape'); step(2); if (O.cmdRing) { input.pressed.add('Escape'); step(2); } };
    /** the cursor where the squad's key reads that target */
    const aim = (t: any): boolean => { if (t.kind === 'door') { const c = t.door.frameCentre, th = t.door.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
        for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { W.cursorAt([c[0] + nx * k, y, c[2] + nz * k]); step(2); const h = O.hover(); if (h?.kind === 'door' && h.door === t.door) return true; } return false; }
      if (t.man) W.cursorAt([t.man.char.pos[0], 1.0, t.man.char.pos[2]]); else W.cursorAt(t.kind === 'none' ? [sam[0], 0.05, sam[2]] : [t.at[0], 0.04, t.at[2]]); step(2); return true; };
    /** the ring put up on a target as a hand does it: the key kept down past its hold */
    const up = (t: any) => { away(); if (!aim(t)) return false; input.keys.add(KEY); input.pressed.add(KEY); step(HOLD); return !!O.cmdRing; };
    const press = (d: number) => { input.pressed.add(`Digit${d}`); step(); };
    const give = () => { input.keys.delete(KEY); step(); };
    const view = () => O.g.view.ring, texts = () => (view()?.wedges ?? []).map((w: any) => `${w.key}:${w.text}${w.off ? ' (off)' : ''}`);
    const ids = () => { const M = O.menu; return M ? M.rows.map((d: any) => d.id) : []; };
    const tail = () => O.log[O.log.length - 1];
    // targets
    let floor: any = null;
    for (let gy = 100; gy <= 900 && !floor; gy += 50) for (let gx = 100; gx <= 1500 && !floor; gx += 50) { input.mouseX = gx; input.mouseY = gy; step(1); const h = O.hover();
      if (h?.kind === 'floor' && !h.thing && !h.wall && dist(h.at, sam) > 2 && dist(h.at, sam) < 9) floor = h; }
    const doors = [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, sam) - dist(b.frameCentre, sam)), tgt = (d: any) => O.targetAt([d.frameCentre[0], 0, d.frameCentre[2]]);
    const shut = doors.find((d: any) => !d.locked && !d.def.steel && Math.abs(d.angle) < 0.05 && tgt(d).kind === 'door');
    const none = { kind: 'none', at: [sam[0], 0, sam[2]] };
    const posture = () => SIL.squadSilent(g);

    // ---- 1
    { const ok0 = !!floor && up(floor), first = texts(); press(5); const second = texts(), page = O.menu?.page, more = !!O.cmdRing?.more;
      const last = second[second.length - 1] ?? '', was = posture(); const k = (view()?.wedges ?? []).find((w: any) => /^go (silent|loud)$/.test(w.text))?.key ?? 0; if (k) press(k); give(); step(5); const now = posture(); if (now !== was) { O.give(now ? 'loud' : 'silent', none, 'all'); step(3); }
      line(ok0 && first.length === 5 && /^1:move there/.test(first[0]) && /^2:cover that way/.test(first[1]) && first[2] === '3:fall in' && first[3] === '4:hold position' && first[4] === '5:more' && more && page === null && second.length >= 2 && second.some((x: string) => /frag there/.test(x)) && second.some((x: string) => /bang there/.test(x)) && /go silent$/.test(last) && now !== was,
        'the floor\'s ring: a tap\'s order, cover, fall in, hold, more — and more is ONE ring of the throws with the posture row last, which turns the squad\'s posture', `first ${first.join(' | ') || 'no ring'} · wedge 5: more ${more}, page ${page} · second ${second.join(' | ')} · silent ${was} → ${now}`); }
    // ---- 2
    { const T = shut ? tgt(shut) : null, ok0 = !!T && up(T), first = texts(), has = ids(); press(5); const second = texts(), page = O.menu?.page; away();
      const n0 = O.log.length; let gave = '';
      if (T && up(T)) { press(3); give(); step(3); gave = O.log.length > n0 ? tail().order : ''; }
      O.give('fallIn', none, 'all'); for (let i = 0; i < 60 * 30 && g.doorTools.ops.length; i++) step(); step(60);
      line(ok0 && /^1:stack up/.test(first[0] ?? '') && /^2:cover this door/.test(first[1] ?? '') && /^3:open and clear/.test(first[2] ?? '') && /^4:bang and clear/.test(first[3] ?? '') && first[4] === '5:more' && !first.some((x: string) => /fall in|hold position/.test(x)) && page === null && second.length >= 2 && second.length <= K.wedges && /go silent$/.test(second[second.length - 1] ?? '') && !second.some((x: string) => /fall in|hold position|more/.test(x)) && gave === 'openClear',
        'a door\'s ring: its two entries are on the first ring, the rest of its tools one ring behind \'more\' with the posture row last, and wedge 3 gives its entry', `door ${shut?.def.name} · first ${first.join(' | ') || 'no ring'} · second ${second.join(' | ')} · page ${page} · rows ${has.length} · wedge 3 gave '${gave}'`); }
  } finally { knobsBack(knobs0); Math.random = rnd0; Object.assign(NIGHTBREACH, { squad: squad0 }); }
  console.log(fails ? `ringflat-check: ${fails} FAIL` : 'ringflat-check: ALL PASS');
  return fails === 0;
}

if (import.meta.main) process.exit((await ringflatCheck()) ? 0 : 1);
