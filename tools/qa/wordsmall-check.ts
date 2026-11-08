// THE WORD SENDS A SMALL SQUAD IN FROM ITS STACK (src/game/doortools.ts DOORTOOLS.wordSmall, settled). One or two teammates stacked on a door by a plain
// 'stack up' wait on the word's key as the squad of four does.
//   bun run tools/qa/wordsmall-check.ts            (also `probes.ts wordsmall`, in families doortools and squadorders)
//   (1) two teammates, the room clear: the quick key on a door stacks them; the panel then offers the word on its key, the door carries the key's mark
//       and it would go now; the key pressed files 'go' for both, an open-and-clear of that door is made from the stack, and a man is through it;
//   (2) one teammate: the same;
//   (5) the key again a second after the word: the men stay on the entry it made (nothing that reads the key after the door tools takes them off it).
// NOTHING HERE WRITES A SETTLED ROW (2026-11-07). Out: the stand's row; 'the rule off, two teammates stacked: no line, no mark'; 'the squad of four, the same
// hand with the rule on and off'; the recording from before the key and the page's boot flags; and 'the orders going to one man of the two', which stepped
// the orders to one man with the who key under a hold on CMDRING.wholeSquad (settled: an order is the whole squad's, and that hold was refused).
// WORDSMALL_BREAK=1, the red path: the stack is never ordered, so there is no word to offer and every line must fail (it was: the switch left off).
// Headless: the lines are the game's own state. Never seen on a GPU; nobody has played it.
import { blockOf } from './squadwords';
import { standUp, ROOT } from './headless.ts';
const { ORDERS, sideOf } = await import(`${ROOT}/src/game/orders.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { floorPlan } = await import(`${ROOT}/src/game/floorplan.ts`);

const DT = 1 / 60, F = 'KeyF';
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

const BREAK = process.env.WORDSMALL_BREAK === '1';
/** the room clear with `squad` teammates, the team stacked on the nearest door it can stack on by the quick key (WORDSMALL_BREAK: the key is never pressed) */
async function rig(squad: number) {
  const squad0 = NIGHTBREACH.squad; let W: any, why: any;
  try { Object.assign(NIGHTBREACH, { squad }); Math.random = seeded(5); W = await standUp(); W.game.godMode = true; why = W.game.roomClear.start(5); }
  finally { Object.assign(NIGHTBREACH, { squad: squad0 }); }
  const g = W.game, O = g.orders, D = g.doorTools, I = W.input;
  const R = { W, g, O, D, why: why ? String(why) : null as string | null, door: null as any, n0: 0,
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    aim(p: number[]) { W.cursorAt([p[0], p[1], p[2]]); R.step(2); return O.hover?.() ?? null; },
    aimOf(d: any): number[] | null { const c = d.frameCentre, th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
      for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { const p = [c[0] + nx * k, y, c[2] + nz * k]; const h = R.aim(p); if (h?.kind === 'door' && h.door === d) return p; } return null; },
    press(k: string) { I.keys.add(k); I.pressed.add(k); R.step(); I.keys.delete(k); },
    /** the panel's lines that carry a key, and the notes among them */
    lines(): string[] { return blockOf(O).flatMap((s: any) => s.opts.map((o: any) => `${o.key ? `[${o.key}] ` : ''}${o.text}`)); },
    wordLine(): string | null { return R.lines().find((l: string) => l.startsWith('[F] go · ')) ?? null; },
    mark(): any { return floorPlan(g).doors.find((x: any) => x.door === R.door && x.why === 'stack') ?? null; },
    filed(): string[] { return O.log.slice(R.n0).map((r: any) => `${r.order} ${r.who.join('+')}`); },
    op(): any { return D.ops.find((o: any) => o.door === R.door) ?? null; },
    stacked() { return R.team.every((m: any) => m.state === 'dead' || (O.of(m)?.order === 'stack' && O.of(m)?.arrived)); },
  };
  R.step(30); for (const m of R.team) m.god = true;
  if (R.why) return R;
  const stackDef = O.defs.find((d: any) => d.id === 'stack'), all = () => O.men('all');
  for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, g.player.char.pos) - dist(b.frameCentre, g.player.char.pos))) { const t = O.targetAt(d.frameCentre); if (t.kind !== 'door' || t.door !== d) continue; const far = g.cleared.roomBy(t.door, t.side > 0 ? -1 : 1); if (!far || g.cleared.isKnown(far)) continue; if (!O.whyNot(stackDef, O.forMen(t, all()), all()) && R.aimOf(d)) { R.door = d; break; } }
  if (!R.door) { R.why = 'no door to stack on'; return R; }
  if (!BREAK) R.press(ORDERS.quickKey);
  if (!R.until(() => R.stacked(), 40)) { R.why = `not stacked after 40 s: ${R.team.map((m: any) => `${m.callsign} ${O.of(m)?.order}/${O.of(m)?.there}`).join(', ')}`; return R; }
  R.step(30); const p = g.player.char.pos; R.aim([p[0] + 0.4, 0.04, p[2] + 0.4]); R.n0 = O.log.length;
  return R;
}

export async function wordsmallCheck(): Promise<boolean> {
  console.log(`\n=== wordsmall: the word sends one or two teammates in from a plain stack${BREAK ? ' — WORDSMALL_BREAK: the stack never ordered' : ''} ===`);
  let all = true; const line = (ok: boolean, label: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
  const log = console.log, quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await fn(); } finally { console.log = log; } };
  const rnd0 = Math.random;
  try {
    // (1) (2) the word, for two teammates and for one
    for (const n of [2, 1]) {
      const R = await quiet(() => rig(n));
      if (R.why) { line(false, `${n === 2 ? 'two teammates' : 'one teammate'}: the word from a plain stack`, `not staged: ${R.why}`); continue; }
      const names = R.team.map((m: any) => m.callsign), word = R.wordLine(), mark = R.mark(), side0 = R.team.map((m: any) => sideOf(R.door, m.char.pos));
      await quiet(() => R.press(F)); const filed = R.filed(), op = R.op(), opMen = op ? op.men.map((m: any) => m.callsign) : [];
      const through = await quiet(() => R.until(() => R.team.some((m: any, i: number) => sideOf(R.door, m.char.pos) !== side0[i]), 25));
      const went = filed.length === 1 && filed[0] === `go ${names.join('+')}`, made = !!op && op.kind === 'openClear' && names.every((c: string) => opMen.includes(c)) && op.beats.some((b: any) => String(b.what).startsWith('from the stack'));
      line(!!word && !!mark && mark.key === 'F' && mark.now === true && went && made && through,
        `${n === 2 ? 'two teammates' : 'one teammate'}, stacked on a door by the quick key: the panel offers the word on its key and the door carries its mark; the key files 'go' for ${n === 2 ? 'both' : 'him'}, an open-and-clear is made from the stack, and a man goes through the door`,
        `${R.door.def.name} · the panel: ${word ?? 'NO LINE'} · the door's mark: ${mark ? `${mark.key ?? 'no key'}${mark.now ? ', now' : ', not now'}` : 'NONE'} · filed: ${filed.join(', ') || 'NOTHING'} · the door's order: ${op ? `${op.kind} (${opMen.join(', ')}) ${op.beats.map((b: any) => b.what).slice(0, 3).join(' → ')}` : 'NONE'} · ${through ? 'a man is through' : 'NOBODY THROUGH in 25 s'}`);
    }

    // (5) the word AGAIN a second later: the men it sent in are not pulled back out (the room clear's drill reads the key after the door tools)
    { const R = await quiet(() => rig(2));
      if (R.why) line(false, 'a second press a second after the word', `not staged: ${R.why}`);
      else { await quiet(() => { R.press(F); R.step(60); }); const op = R.op(), drill0 = R.g.nightBreach.state; await quiet(() => { R.press(F); R.step(30); });
        const op2 = R.op(), on = !!op2 && op2 === op && R.team.every((m: any) => op2.men.includes(m));
        line(!!op && on && R.g.nightBreach.state === drill0 && R.filed().length === 1, 'the key again a second after the word: the two men stay on the entry the word made — nothing else takes them off it, and nothing more is filed',
          `after the word: ${op ? `${op.kind} · ${op.phase}` : 'NO DOOR ORDER'} · after the second press: ${op2 ? `${op2.kind} · ${op2.phase} (${op2.men.map((m: any) => m.callsign).join(', ')})` : 'THE DOOR ORDER IS GONE'} · the drill ${drill0} → ${R.g.nightBreach.state} · filed: ${R.filed().join(', ') || 'NOTHING'}`); } }

  } finally { console.log = log; Math.random = rnd0; }
  console.log(`=== wordsmall: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await wordsmallCheck()) ? 0 : 1);
