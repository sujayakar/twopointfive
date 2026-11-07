// THE SQUAD REACHES YOU (src/game/squadnet.ts: SQUADNET.on), headless:
//   bun run tools/qa/squadnet-check.ts [clear|end|net]  (probes.ts squadnet; SQUADNET_BREAK=1 breaks the staging: the quick key is never pressed,
//   the rooms asked of are staged with a point of floor unseen, the teammate who speaks is never put off the screen: must fail)
//   end    an entry into a room the mission has no name for (the office's conference room) ends with the lead's call too: 'room clear.' with all of its
//          floor seen and nobody in it. Open floor is called 'clear so far.'.
//   net    where a far line is put (the page's own layer, no game under it): at the view's edge on his side, along the border, never off the view.
// Where a far line is DRAWN (ui/bubbles.ts)
// is not looked at here, no picture is; where it is PUT is (squadnet.ts netPlace, the one place bubbles.ts asks), in `net`.
import { standUp, ROOT } from './headless.ts';

const SN: any = await import(`${ROOT}/src/game/squadnet.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS, sideOf } = await import(`${ROOT}/src/game/orders.ts`);
const { SQUADNET, roomCall, netPlace } = SN;

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const DT = 1 / 60, BREAK = process.env.SQUADNET_BREAK === '1';
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** the tables this file writes, put back after `fn` */
async function kept<T>(fn: () => Promise<T> | T): Promise<T> {
  const was = { sn: { ...SQUADNET }, squad: NIGHTBREACH.squad, rnd: Math.random };
  try { return await fn(); } finally { Object.assign(SQUADNET, was.sn); NIGHTBREACH.squad = was.squad; Math.random = was.rnd; }
}
/** a line that calls an entry's end */
const isCall = (x: string) => (/ clear\.$/.test(x) && !/ and clear\.$/.test(x)) || / quiet\. |^clear so far\.$/.test(x);   // (not an order's own answer: 'bang and clear.')

/** the room clear on the office with a squad of four under god mode, the autopilot on (autopilot-check's own staging); `play`: every call the mixer was asked for */
async function rig(seed: number) {
  const squad0 = NIGHTBREACH.squad; let W: any, why: any;
  try { NIGHTBREACH.squad = 4; Math.random = seeded(seed); W = await standUp(); W.game.godMode = true; why = W.game.roomClear.start(seed); } finally { NIGHTBREACH.squad = squad0; }
  const game = W.game, O = game.orders, play: { name: string; gain: number; rate: number | undefined }[] = [];
  { const a = game.audio, p0 = a.play.bind(a); a.play = (name: string, pos: unknown, gain = 1, opts: any = {}) => { play.push({ name, gain, rate: opts?.rate }); return p0(name, pos, gain, opts); }; }
  const said: string[] = []; { const msg = game.msg.bind(game); game.msg = (t: string) => { said.push(t); msg(t); }; }
  // (`said` is the log, which keeps a line only where its man is off the screen or in the fog; `bub` is every line a teammate's bubble has held, read a frame)
  const bub: string[] = [], bubSeen = new Set<string>();
  const R = { W, game, O, why, play, said, bub,
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; W.step(DT); for (const m of O.squad()) { const b = m.bubble, k = b ? `${m.callsign}|${b.t}|${b.text}` : ''; if (b && !bubSeen.has(k)) { bubSeen.add(k); bub.push(b.text); } } } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    stacked() { return R.team.filter((m: any) => m.state !== 'dead').every((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.arrived); },
  };
  R.step(30);
  for (const m of R.team) m.god = true;
  return R;
}
/** a point of that door's doorway the cursor on which IS that door (null: none) */
function aimOf(R: any, d: any): number[] | null {
  const c = d.frameCentre, th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
  for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { const p = [c[0] + nx * k, y, c[2] + nz * k]; R.W.cursorAt(p); R.step(2); const h = R.O.hover?.(); if (h?.kind === 'door' && h.door === d) return p; }
  return null;
}
const farRoom = (g: any, t: any) => g.cleared.roomBy(t.door, t.side > 0 ? -1 : 1);
/** the squad stacked on the nearest door whose far room nobody has stood in and that the cursor can be put on (`named`: and whose far room is one of the
 *  room clear's own, the rooms a lead calls by name — 'not': and is none of them; `first`: run once the game stands, before anything is said) */
async function stackedOn(seed: number, named: boolean | 'not' = false, first?: () => void): Promise<{ R: any; D: any; t: any; at: number[]; set: boolean } | { R: any; why: string }> {
  const R: any = await hushed(() => rig(seed)); if (R.why) return { R, why: String(R.why) }; first?.();
  const g = R.game, O = R.O, all = O.men('all'), stackDef = O.defs.find((d: any) => d.id === 'stack'), s = g.player.char.pos;
  for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, s) - dist(b.frameCentre, s))) {
    const t = O.targetAt(d.frameCentre); if (t.kind !== 'door' || t.door !== d) continue;
    const far = farRoom(g, t); if (!far || g.cleared.isKnown(far) || O.whyNot(stackDef, O.forMen(t, all), all)) continue;
    if (named) { const c = d.frameCentre, th = d.def.closedDir, k = -t.side * 1.5, p = [c[0] - Math.sin(th) * k, 0, c[2] + Math.cos(th) * k]; if (g.roomClear.rooms.some((r: any) => r.def.id === g.roomClear.roomAt(p)) !== (named === true)) continue; }
    const at = aimOf(R, d); if (!at) continue;
    if (!BREAK) R.W.input.pressed.add(ORDERS.quickKey); R.step();
    return { R, D: d, t, at, set: R.until(() => R.stacked(), 40) };
  }
  return { R, why: 'no door to stack on with an unknown room behind it' };
}

export async function squadnetClear(): Promise<boolean> {
  const F = family('squadnet-clear: \'clear\' means clear');
  await kept(async () => {
    const R: any = await hushed(() => rig(7)), g = R.game;
    const a = roomCall(g, 'mess', BREAK ? 1 : 0, false, false), b = roomCall(g, 'mess', 3, false, false), c = roomCall(g, 'mess', 3, true, false), d = roomCall(g, 'mess', 3, true, true), e = roomCall(g, 'mess', 0, true, true), f = roomCall(g, 'mess', 0, true, false);
    F.verdict(a === 'mess clear.' && b === "mess's quiet. we didn't see all of it." && c === b && d === null && e === null && f === null,
      'all of the floor seen and nobody in it: clear · floor unseen and nobody in sight: quiet · an enemy in sight, or one standing on floor they saw: nothing', `seen all, empty: ${a} · 3 unseen, empty: ${b} · 3 unseen, one inside unseen: ${c} · 3 unseen, one in sight: ${d} · seen all, one in sight: ${e} · seen all, one inside: ${f}`);
  });
  return F.done();
}

export async function squadnetEnd(): Promise<boolean> {
  const F = family('squadnet-end: an entry into a room the mission has no name for ends with a call too');
  // the entry itself: the squad goes into the nearest room that is none of the room clear's own, on the word
  const run = () => kept(async () => {
    const S: any = await stackedOn(5, 'not'); if (S.why) return { why: String(S.why) } as any;
    const { R, D } = S, g = R.game, O = R.O, T = g.doorTools, RC = g.roomClear;
    const tm = { ...S.t, side: O.of(R.team[0])?.post ? sideOf(D, O.of(R.team[0]).post) : S.t.side }, far = farRoom(g, tm); let emptied = 0;
    for (const x of g.guards) if (x.state !== 'dead' && x.team === 'hostile' && g.cleared.roomAt(x.char.pos) === far) { x.char.pos[0] += 400; emptied++; }   // (the room beyond is emptied: the call is the search's alone)
    const d0 = T.done.length; R.W.input.pressed.add('KeyF'); R.step(); R.step(2); const n0 = R.bub.length; R.until(() => T.ops.length === 0, 70); R.step(30);   // (the window opens once the press has been answered)
    const op = T.done.length > d0 ? T.done[T.done.length - 1] : null, lead = op?.lead;
    return { set: S.set, door: D.def.name, emptied, went: !!op?.beats.some((b: any) => b.what === 'in'), named: !!lead && RC.rooms.some((r: any) => r.def.id === RC.roomAt(lead.char.pos)), left: op?.search ? op.search.left : NaN, calls: R.bub.slice(n0).filter(isCall) as string[] };
  });
  const on = await run();
  if (on.why) { F.verdict(false, 'the squad stacks on a door into a room the mission has no name for', String(on.why)); return F.done(); }
  const tell = (r: any) => `${r.door}: stacked ${r.set} · went in ${r.went} · the lead ends in ${r.named ? 'a NAMED room' : 'no named room'} · floor unseen ${r.left} · called ${JSON.stringify(r.calls)}`;
  F.verdict(on.set && on.went && !on.named && on.calls.length === 1 && /^room clear\.$|^room's quiet\. /.test(on.calls[0]), 'the lead calls the room though the mission has no name for it', tell(on));
  return F.done();
}

export async function squadnetNet(): Promise<boolean> {
  const F = family('squadnet-net: where a far line is put');
  // where a far line is put (netPlace, the one place ui/bubbles.ts asks): at the view's edge on his side
  await kept(() => {
    const w = 960, h = 540, px = 470, py = 300, bw = 150, bh = 22, m = Math.min(w, h), pad = SQUADNET.edgeIn * m, x0 = bw / 2 + 6, x1 = w - bw / 2 - 6, y0 = bh + 12, y1 = h - 12, free = () => false;
    const dirs: [string, number, number][] = [['right', 1, 0], ['down right', 1, 1], ['down', 0, 1], ['down left', -1, 1], ['left', -1, 0], ['up left', -1, -1], ['up', 0, -1], ['up right', 1, -1]];
    /** how far inside the kept box's border the point lies, on the nearest border of his side */
    const gap = (x: number, y: number, dx: number, dy: number) => Math.min(dx > 0 ? x1 - x : dx < 0 ? x - x0 : Infinity, dy > 0 ? y1 - y : dy < 0 ? y - y0 : Infinity);
    const far = (dx: number, dy: number) => netPlace(w, h, px, py, px + dx * 4000, py + dy * 4000, bw, bh, free) as [number, number];
    const on = dirs.map(([n, dx, dy]) => { const [x, y] = far(dx, dy), L = Math.hypot(dx, dy), vx = x - px, vy = y - py, D = Math.hypot(vx, vy); return { n, x, y, gap: gap(x, y, dx, dy), D, straight: Math.abs(vx * dy - vy * dx) / L < 0.01 && vx * dx + vy * dy > 0 }; });
    const here = netPlace(w, h, px, py, px + 40, py, bw, bh, free), none = netPlace(w, h, px, py, px, py, bw, bh, free);
    const atEdge = on.every(o => Math.abs(o.gap - pad) < 0.01 && o.straight);
    F.verdict(atEdge && !!here && Math.abs(here[0] - (px + 40)) < 0.01 && Math.abs(here[1] - py) < 0.01 && none === null,
      'a far line is put at the edge of the view on his side, straight his way from you; a man near you is called where he is, a man on you not at all', `${w} x ${h}, you at ${px}, ${py} · kept ${pad.toFixed(1)} px in · ${on.map(o => `${o.n} ${o.x.toFixed(0)},${o.y.toFixed(0)} (${o.gap.toFixed(1)} in, ${o.D.toFixed(0)} out)`).join(' · ')} · a man 40 px away: ${here?.map((v: number) => v.toFixed(0))} · a man on you: ${none}`);
    // you out by the border yourself: a man past that border does not put the line on you, and it does not jump as his bearing crosses straight up
    { const W = 1280, H = 720, ux = 150, uy = 360, lw = 298, M = Math.min(W, H), P = SQUADNET.edgeIn * M, bx0 = lw / 2 + 6 + P, by0 = bh + 12 + P;
      const l = netPlace(W, H, ux, uy, ux - 5, -4000, lw, bh, free) as [number, number], r = netPlace(W, H, ux, uy, ux + 5, -4000, lw, bh, free) as [number, number];
      const you = (x: number, y: number) => x + lw / 2 > ux - 16 && x - lw / 2 < ux + 16 && y > uy - 30 && y - bh < uy + 34, past = netPlace(W, H, ux, uy, -4000, uy, lw, bh, you) as [number, number];
      F.verdict([l, r].every(q => Math.abs(q[1] - by0) < 0.01 && Math.hypot(q[0] - ux, q[1] - uy) > 100) && Math.abs(l[0] - r[0]) < 5 && Math.abs(past[0] - bx0) < 0.01 && !you(past[0], past[1]),
        'you out by the border yourself: a man off the top is called at the top whichever side of straight up he is, and a man past your border is called on that border clear of you', `${W} x ${H}, you at ${ux}, ${uy}, a ${lw} px line · off the top a hair left ${l.map(v => v.toFixed(0))} · a hair right ${r.map(v => v.toFixed(0))} · past the left border at your height ${past.map(v => v.toFixed(0))}`); }
    // a spot that hits something moves along the border it is at; never off the view
    { const A = 156 * Math.PI / 180, qx = 653, qy = 292, by0 = y0 + pad, by1 = y1 - pad, over = (r: number[], x: number, y: number) => x + bw / 2 > r[0] && x - bw / 2 < r[1] && y > r[2] && y - bh < r[3] + 4, card = [0, 250, 350, 540];
      const bare = netPlace(w, h, qx, qy, qx + Math.cos(A) * 4000, qy + Math.sin(A) * 4000, bw, bh, free) as [number, number], low = netPlace(w, h, qx, qy, qx + Math.cos(A) * 4000, qy + Math.sin(A) * 4000, bw, bh, (x: number, y: number) => over(card, x, y)) as [number, number];
      const top = netPlace(w, h, px, py, px + 300, py - 4000, bw, bh, free) as [number, number], second = netPlace(w, h, px, py, px + 320, py - 4000, bw, bh, (x: number, y: number) => Math.abs(y - top[1]) < 25.5 && Math.abs(x - top[0]) < bw + 4) as [number, number];
      const right = far(1, 0), under = netPlace(w, h, px, py, px + 4000, py, bw, bh, (_x: number, y: number) => y === right[1]) as [number, number], full = netPlace(w, h, px, py, px + 4000, py, bw, bh, () => true) as [number, number];
      let out = 0, n = 0;
      for (const lw of [100, 300, 900, 1200]) for (const [sx, sy] of [[470, 300], [10, 10], [950, 530], [480, 5], [5, 270]]) for (let k = 0; k < 16; k++) for (const hit of [free, () => true, (x: number, y: number) => over(card, x, y)]) {
        const q = netPlace(w, h, sx, sy, sx + Math.cos(k * Math.PI / 8) * 3000, sy + Math.sin(k * Math.PI / 8) * 3000, lw, bh, hit) as [number, number], a = lw / 2 + 6, b = Math.max(a, w - lw / 2 - 6); n++;
        if (!(q[0] >= a - 0.01 && q[0] <= b + 0.01 && q[1] >= y0 - 0.01 && q[1] <= y1 + 0.01)) out++; }
      F.verdict(Math.abs(bare[1] - by1) < 0.01 && bare[0] < 250 && Math.abs(low[1] - by1) < 0.01 && low[0] - bw / 2 >= 250 && low[0] - bare[0] < 26 * 7 && Math.abs(second[1] - (by0 + 26)) < 0.01 && Math.abs(under[0] - right[0]) < 0.01 && Math.abs(under[1] - (right[1] - 26)) < 0.01 && full[0] === right[0] && full[1] === right[1] && out === 0,
        'a spot that lies on something moves along the border it is at, the nearest free place first; never off the view', `at the bottom, under a card ${card.slice(0, 2).join(' to ')} wide: ${bare.map(v => v.toFixed(0))} moves to ${low.map(v => v.toFixed(0))} · a second man off the top: ${top.map(v => v.toFixed(0))} then ${second.map(v => v.toFixed(0))} · at the right border on a taken row: ${right.map(v => v.toFixed(0))} moves to ${under.map(v => v.toFixed(0))} · nothing free: ${full.map(v => v.toFixed(0))} · ${n} placings, ${out} off the view`); }
  });
  return F.done();
}

if (import.meta.main) {
  const which = process.argv[2] ?? 'all', runs: [string, () => Promise<boolean>][] = [['clear', squadnetClear], ['end', squadnetEnd], ['net', squadnetNet]];
  let ok = true; for (const [n, fn] of runs) if (which === 'all' || which === n) ok = (await fn()) && ok;
  process.exit(ok ? 0 : 1);
}
