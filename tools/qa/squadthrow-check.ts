// TEAMMATES THROW A FRAG OR A STUN CANISTER ON ORDER (src/game/squadthrow.ts): X on a spot of floor, G, then 1 = frag there / 2 = bang there.
//   bun run tools/qa/squadthrow-check.ts            (also `probes.ts squadthrow`, and under `squadorders`)
// PASS / FAIL each, all headless: the room clear on the office with its squad of four (the rescue for the hostage line), every hostile off the floor
// unless a line wants one. What is checked is the menu's rows by KEYS, who throws, the item (whose it is, where its fuse finds it), the count, the words and every
// refusal; nothing here looks at a picture. (The squad block's line is not asked: the panel on the tools' copy does not carry the Rust game's notes.)
// SQUADTHROW_BREAK=1 = the switch left off and the same lines asked: they must fail.   exit 0 = all hold · 1 = one does not
import { blockOf, listedOf, ringUp, ringDown } from './squadwords';
import { standUp, ROOT } from './headless.ts';

const ST: any = await import(`${ROOT}/src/game/squadthrow.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const SLM: any = await import(`${ROOT}/src/game/silent.ts`);
const { FRAG } = await import(`${ROOT}/src/game/frag.ts`);
const { WORDS } = await import(`${ROOT}/src/game/words.ts`);
const { SQUADTHROW, THROW_OPEN, FRAG_THERE, STUN_THERE } = ST;

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : '—');
const p2 = (p: number[]) => `(${f1(p[0])}, ${f1(p[2])})`;
interface Row { key: string; text: string; off: boolean }
const show = (rows: Row[]) => rows.map(r => `${r.key || '·'} ${r.text}${r.off ? ' (off)' : ''}`).join(' | ');
const digitAbove5 = (rows: Row[]) => rows.some(r => /^[6-9]$|^0$/.test(r.key));
export function place(gd: any, p: number[], yaw = 0) { gd.char.pos = [p[0], p[1], p[2]]; gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }

/** a squad mission on the office, stood up with the switch as `on`; every hostile off the floor (`keep`: but the first, held where he is and not to be hurt) */
export async function rig(seed: number, on: boolean, o: { rescue?: boolean; squad?: 2 | 4; keep?: boolean; live?: boolean; silent?: boolean; walk?: boolean; lob?: boolean } = {}) {
  const squad0 = NIGHTBREACH.squad; Object.assign(NIGHTBREACH, { squad: o.squad ?? 4 });
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game, input = W.input, O = game.orders;
  SQUADTHROW.on = on; FRAG.on = true; if (o.silent) SLM.SILENT.on = true;   // (walk: a man with no throw walks to one — squadthrow-walk-check.ts asks about that; every line here is about the throw from where he stands)
  game.godMode = true;
  const said: string[] = []; { const msg = game.msg.bind(game); game.msg = (t: string) => { said.push(t); msg(t); }; }
  const lines: { who: string; text: string }[] = []; { const say = game.say.bind(game); game.say = (gd: any, t: string, ...a: any[]) => { lines.push({ who: gd?.callsign ?? '?', text: t }); return say(gd, t, ...a); }; }
  const why = o.rescue ? game.rescue.start() : game.roomClear.start(seed);
  Object.assign(NIGHTBREACH, { squad: squad0 });
  const R = { W, game, input, O, why, said, lines, held: 0, foe: null as any,
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; R.held = Math.max(R.held, input.keys.size + (input.buttons ? 1 : 0)); W.step(DT); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    cursor(p: number[]) { W.cursorAt([p[0], p[1], p[2]]); },
    press(code: string) { input.pressed.add(code); R.step(); },
    /** the menu's order rows as the panel draws them now (the who row and the cancel row left out) */
    rows(): Row[] { const hud = listedOf(O); return (hud?.rows ?? []).map((r: any) => ({ key: r.opts[0]?.key ?? '', text: r.opts[0]?.text ?? '', off: !!r.off })); },
    /** X with the cursor on p: the first page — the menu left UP */
    menu(p: number[]): Row[] { R.esc(); R.cursor(p); ringUp(input, R); R.step(2); return R.rows(); },
    /** X on p, then G: the throw page — the menu left UP */
    page(p: number[]): Row[] { R.menu(p); R.press(FRAG.orderKey); R.step(2); return R.rows(); },
    /** the same page with the menu opened from code on p (the camera, settled over Sam, may have a wall's top between it and the spot) — G is still a key */
    pageAt(p: number[]): Row[] { R.esc(); R.cursor(p); ringUp(input, R); R.press(FRAG.orderKey); R.step(2); return R.rows(); },
    esc() { for (let i = 0; i < 4 && O.menu; i++) R.press('Escape'); },
    notes(): string[] { return (blockOf(O) ?? []).flatMap((r: any) => r.opts.map((x: any) => x.text as string)); },
    items(kind: string): any[] { return game.items.items.filter((x: any) => x.kind === kind); },
    /** the squad's own bag, written (the frozen stand issues no frags) */
    bag(frags: number, bangs: number) { game.doorTools.frags = frags; game.doorTools.bangs = bangs; },
    /** Sam put on the floor at p (the camera with him), the squad left holding where it stands */
    sam(p: number[]) { game.teleportPlayer([p[0], 0, p[2]]); R.step(20); },
  };
  R.step(30);
  for (const m of R.team) m.god = true;
  if (!o.live) { for (const h of game.guards) if (h.team === 'hostile' && h.state !== 'dead') { if (o.keep && !R.foe) { R.foe = h; h.god = true; continue; } killGuard(game, h, [0, 0, 1], true); } }
  if (R.foe) place(R.foe, [2, 0, 2]);
  if (!o.live) { O.give('hold', O.targetAt(game.player.char.pos)); R.step(60 * 3); }
  R.bag(2, 3);
  return R;
}
/** p is floor the menu takes for itself, and the cursor put on it finds it (no wall's top between the camera and the spot) */
export function floorAt(R: any, p: number[]): number[] | null {
  const t = R.O.targetAt(p); if (t.kind !== 'floor' || dist(t.at, p) > 0.3) return null;
  R.cursor(t.at); const u = R.O.targetUnder(R.input, R.W.cam, R.W.canvas); return u.kind === 'floor' && dist(u.at, t.at) < 0.3 ? u.at : null;
}
/** the first spot on rings round the squad's men that `ok` takes */
function find(R: any, ds: number[], ok: (at: number[]) => boolean): number[] | null {
  for (const d of ds) for (const gd of R.team) { const p = gd.char.pos;
    for (let k = 0; k < 24; k++) { const a = k * Math.PI / 12, at = floorAt(R, [p[0] + Math.sin(a) * d, 0, p[2] + Math.cos(a) * d]); if (at && ok(at)) return at; } }
  return null;
}
/** a floor point for Sam: `far` = more than 10 m from `spot` or with something solid between, else within `near` m of it with a line; never on a man */
function samSpot(R: any, spot: number[], near: number | null): number[] | null {
  const g = R.game, clear = (a: number[], b: number[]) => !g.col.solidBlocked([a[0], 1.1, a[2]], [b[0], 1.1, b[2]]) && !g.col.solidBlocked([a[0], 0.3, a[2]], [b[0], 0.3, b[2]]);
  if (near !== null) { for (const d of [near, near * 0.75, near * 1.25]) for (let k = 0; k < 16; k++) { const a = k * Math.PI / 8, p = [spot[0] + Math.sin(a) * d, 0, spot[2] + Math.cos(a) * d], t = R.O.targetAt(p);
      if (t.kind === 'floor' && dist(t.at, p) < 0.3 && clear(p, spot) && R.team.every((m: any) => dist(m.char.pos, p) > 1)) return p; } return null; }
  for (let x = 3; x <= 37; x += 1.5) for (let z = 3; z <= 25; z += 1.5) { const p = [x, 0, z], t = R.O.targetAt(p); if (t.kind === 'floor' && dist(t.at, p) < 0.3 && dist(p, spot) > 11 && R.team.every((m: any) => dist(m.char.pos, p) > 1.5)) return p; }
  return null;
}


