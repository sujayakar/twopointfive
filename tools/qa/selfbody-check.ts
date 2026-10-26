// HIS OWN BODY UNDER THE CURSOR IS HIMSELF (src/game/orders.ts targetUnderOf, ORDERS.selfBody). Two tutorial missions found it the same day: the squad's key
// "on himself" with the cursor on his chest and a door a pace behind him gave that door (a stack) and not fall in, because the cursor's ray passes through
// his body and lands on the floor past him, by the door; at his feet it was fall in.
//   bun run tools/qa/selfbody-check.ts            (also `probes.ts selfbody`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/selfbody-check.ts      — the same lines on the page's stand
// PASS / FAIL each, headless: the room clear on the office with its squad of four under god mode, one deal (the rule has no die). He is stood in front of
// a door, on the camera's side of it, where the fault shows: the first door and distance at which, with the rule off, the cursor on his chest is that door.
// Every press is a one-frame press of the quick key with the harness cursor where the line says.
//   1. CONTROL — the rule off: the cursor on his chest is the door behind him, and the key there sends the squad to stack on it (the fault, as it was).
//   2. the rule on, the same place: the cursor on his chest is himself, and the key there is fall in for all four — no stack, no door worked.
//   3. back where he started, on open floor: the cursor at his feet is himself both ways (the rule changes nothing there). (In front of the door his feet
//      are behind the near wall's top from the camera: there his chest is all of him the cursor can be put on.)
//   4. the rule on: the cursor on the doorway itself, off his body, is still that door.
//   5. the rule on, back where he started: a teammate stood between the camera and him, the cursor on that teammate: the teammate's place, not his.
//   6. CONTROL (plumbing) — a recording from before the key replays with it off; the frozen stand holds it off, the page's has it on, FROZEN_HOLDS has its row.
// SELFBODY_BREAK=1 = the rule's line never taken (orders.ts): line 2 must fail.
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const { applyKnobs, knobsSnapshot } = await import(`${ROOT}/src/game/journal.ts`);
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const DT = 1 / 60, SEED = 5, BREAK = process.env.SELFBODY_BREAK === '1', VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

export async function selfBodyCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rnd0 = Math.random, squad0 = NIGHTBREACH.squad;
  console.log(`His own body under the cursor is himself (src/game/orders.ts targetUnderOf, ORDERS.selfBody)${BREAK ? ' — SELFBODY_BREAK=1: the rule\'s line is never taken, line 2 must fail' : ''}`);
  try {
    const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { };
    let W: any, why: any, stood = false;
    try { Object.assign(NIGHTBREACH, { squad: 4 }); Math.random = seeded(SEED); W = await standUp(); stood = !!ORDERS.selfBody; W.game.godMode = true; why = W.game.roomClear.start(SEED); }
    finally { console.log = log; console.warn = warn; Object.assign(NIGHTBREACH, { squad: squad0 }); }
    const g = W.game, O = g.orders, T = g.doorTools, I = W.input, pl = g.player;
    const quiet = <X,>(f: () => X): X => { if (VERBOSE) return f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
    const step = (n = 1) => quiet(() => { for (let i = 0; i < n; i++) { pl.visibility = 0.05; W.step(DT); } });
    step(30); for (const m of O.squad()) m.god = true;
    const all = () => O.men('all'), N = all().length;
    if (why || N !== 4) { line(false, 'the room clear on the office with its squad of four', String(why ?? `${N} men`)); return false; }
    /** what the cursor is on with its ray through `p`: put there until the view has stopped moving under it (the view leans to the cursor), then read with no tick between */
    const under = (p: number[]) => { for (let k = 0; k < 8; k++) { W.cursorAt(p); step(1); } W.cursorAt(p); return O.targetUnder(I, W.cam, W.canvas); };
    const chest = () => [pl.char.pos[0], 1.0, pl.char.pos[2]], feet = () => [pl.char.pos[0], 0.05, pl.char.pos[2]];
    const says = (t: any) => !t ? 'nothing' : t.kind === 'door' ? `door ${t.door.def.name}` : t.kind === 'none' ? `nobody's floor at (${t.at[0].toFixed(1)}, ${t.at[2].toFixed(1)})${t.man ? ` · ${t.man.callsign}` : ''}` : `${t.kind} at (${t.at[0].toFixed(1)}, ${t.at[2].toFixed(1)})`;
    const self = (t: any) => !!t && t.kind === 'none' && !t.man && dist(t.at, pl.char.pos) < ORDERS.selfM;
    // the place: in front of a door on the camera's side of it, where — the rule off — the cursor on his chest is that door
    const home = [...pl.char.pos]; let D: any = null, at: number[] | null = null;
    ORDERS.selfBody = false;
    find: for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, home) - dist(b.frameCentre, home))) for (const back of [0.7, 0.5, 0.6, 0.8, 0.9]) {
      const { fwd } = W.cam.planarBasis(), p = [d.frameCentre[0] - fwd[0] * back, 0, d.frameCentre[2] - fwd[2] * back];
      if (g.col.nav.isBlocked(p[0], p[2])) continue;
      g.teleportPlayer(p); step(45); if (dist(pl.char.pos, p) > 0.3) continue;
      const t = under(chest()); if (t?.kind === 'door' && t.door === d) { D = d; at = [...pl.char.pos]; break find; } }
    if (!D || !at) { line(false, 'a door and a place in front of it where the cursor on his chest is the door behind him', 'none found on this floor'); return false; }
    const place = () => { g.teleportPlayer(at); O.give('hold', O.targetAt(home), 'all'); step(30); };
    /** one press of the quick key with the cursor where `p()` says: what it was on, the order written, the door tools' entries on the door */
    const press = (p: () => number[]) => { place(); const t = under(p()), n0 = O.log.length, ops0 = T.ops.length; I.pressed.add(ORDERS.quickKey); step(1); step(2); const row = O.log.length > n0 ? O.log[O.log.length - 1] : null;
      return { t, order: row?.order ?? 'none', to: row?.who?.length ?? 0, door: row?.target?.kind === 'door' || all().some((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.door === D), ops: T.ops.length - ops0 }; };
    const tell = (R: ReturnType<typeof press>) => `under the cursor: ${says(R.t)} · the order: ${R.order} to ${R.to} · men sent to the door: ${R.door} · door tools started ${R.ops}`;

    // 1
    { ORDERS.selfBody = false; const R = press(chest);
      line(R.t?.kind === 'door' && R.t.door === D && R.order === 'stack' && R.to === N, `1. CONTROL — the rule off, stood ${dist(at, D.frameCentre).toFixed(1)} m in front of ${D.def.name}: the cursor on his chest is that door and the key stacks the squad on it`, tell(R)); }
    // 2
    { ORDERS.selfBody = true; const R = press(chest);
      line(self(R.t) && R.order === 'fallIn' && R.to === N && !R.door && R.ops === 0, '2. the rule on, the same place: the cursor on his chest is himself and the key is fall in for all four', tell(R)); }
    // 3
    { g.teleportPlayer(home); step(45); ORDERS.selfBody = false; const a = under(feet()); ORDERS.selfBody = true; const b = under(feet());
      line(self(a) && self(b), '3. back where he started: the cursor at his feet is himself with the rule off and on', `off: ${says(a)} · on: ${says(b)}`); }
    // 4
    { place(); ORDERS.selfBody = true; const c = D.frameCentre, th = D.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th); let got: any = null, tried = 0;
      for (const y of [c[1], 1.8, 0.05]) for (const k of [0.35, -0.35, 0]) { if (got) break; tried++; const p = [c[0] + nx * k, y, c[2] + nz * k];
        const t = under(p), ray = W.cam.ray(I.mouseX / Math.max(1, W.canvas.clientWidth), I.mouseY / Math.max(1, W.canvas.clientHeight)); if (pl.char.rayHit(ray.ro, ray.rd, 200) >= 0) continue;   // (off his body: a ray that crosses him is line 2's)
        if (t?.kind === 'door' && t.door === D) got = t; }
      line(!!got, '4. the rule on: the cursor on the doorway itself, off his body, is still that door', `${got ? says(got) : `not the door at any of ${tried} points of its doorway`}`); }
    // 5
    { g.teleportPlayer(home); O.give('hold', O.targetAt([home[0] + 3, 0, home[2]]), 'all'); step(45); ORDERS.selfBody = true; const m = all()[0], { fwd } = W.cam.planarBasis(), p = [home[0] - fwd[0] * 0.45, 0, home[2] - fwd[2] * 0.45];   // (half a pace nearer the camera: the ray through his chest goes on through Sam's legs)
      const put = () => { m.char.pos[0] = p[0]; m.char.pos[2] = p[2]; }; put(); step(1); put();
      W.cursorAt([p[0], 1.0, p[2]]); step(1); put(); const ray = W.cam.ray(I.mouseX / Math.max(1, W.canvas.clientWidth), I.mouseY / Math.max(1, W.canvas.clientHeight));
      const tm = m.char.rayHit(ray.ro, ray.rd, 200), ts = pl.char.rayHit(ray.ro, ray.rd, 200), t = O.targetUnder(I, W.cam, W.canvas);
      line(tm >= 0 && ts > tm && t?.kind === 'none' && dist(t.at, p) < 0.2 && dist(t.at, pl.char.pos) > 0.3, '5. the rule on, back where he started: a teammate between the camera and him, the cursor on that teammate: the teammate\'s place, not his own',
        `the ray meets ${m.callsign} at ${tm.toFixed(1)} m${ts >= 0 ? ` and him at ${ts.toFixed(1)} m` : ' and not him'} · under the cursor: ${says(t)} · ${m.callsign} stands at (${p[0].toFixed(1)}, ${p[2].toFixed(1)})`); }
    // 6
    { ORDERS.selfBody = true; const snap = JSON.parse(JSON.stringify(knobsSnapshot(g))), old = JSON.parse(JSON.stringify(snap)); delete old.tables.ORDERS.selfBody;
      applyKnobs(g, old); const before = ORDERS.selfBody; applyKnobs(g, snap); const on = ORDERS.selfBody;
      const shipped = String(process.env.TPS_PROFILE ?? '') === 'shipped', row = FROZEN_HOLDS['ORDERS.selfBody'];
      line(before === false && on === true && stood === shipped && /TPS_SELFBODY=1/.test(row ?? ''), '6. CONTROL (plumbing) — a recording from before the key replays with it off; the stands and the FROZEN_HOLDS row',
        `a file without the key: ${before} · one with it on: ${on} · this stand (${shipped ? 'the page\'s' : 'frozen'}) stood it ${stood} · the row: ${row ?? 'none'}`); }
  } finally { knobsBack(knobs0); Math.random = rnd0; }
  console.log(fails ? `=== selfbody-check: ${fails} FAIL ===` : '=== selfbody-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await selfBodyCheck()) ? 0 : 1);
