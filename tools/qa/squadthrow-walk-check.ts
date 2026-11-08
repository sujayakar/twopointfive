// A TEAMMATE WITH NO THROW FROM WHERE HE STANDS WALKS TO ONE (src/game/squadthrow.ts SQUADTHROW.walk): X on a spot of floor, G, then 1 / 2.
//   bun run tools/qa/squadthrow-walk-check.ts          (also `probes.ts squadthrow-walk`, and under `squadthrow` / `squadorders`)
// PASS / FAIL each, all headless, on squadthrow-check.ts's rig: the room clear on the office with its four, every hostile off the floor, the squad holding.
// Asked: a spot nobody has a throw to gets a man who WALKS: the row is live, the order by keys, nothing thrown or spent until he is on his spot, the thing
// then comes to rest by the spot asked for, he goes back to where he stood and to the order he had (a hold, a fall in); another order mid-walk ends it with
// nothing spent.
//   exit 0 = all hold · 1 = one does not
import { ROOT } from './headless.ts';
import { rig } from './squadthrow-check.ts';

const ST: any = await import(`${ROOT}/src/game/squadthrow.ts`);
const { FRAG } = await import(`${ROOT}/src/game/frag.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { SQUADTHROW, SQUADTHROW_STATS, SQUADTHROW_LOG, FRAG_THERE, STUN_THERE } = ST;
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : '—');
const p2 = (p: number[]) => `(${f1(p[0])}, ${f1(p[2])})`;

/** floor spots nobody of the squad has a throw to from where he stands and one of them has after a walk (a frag: nobody of ours in it), nearest walk first */
function spots(R: any, kind: 'frag' | 'stun', max = 6): { at: number[]; th: any }[] {
  const { game, O } = R, team = R.team, out: { at: number[]; th: any; d: number }[] = [];
  for (let x = 2.25; x <= 38; x += 3) for (let z = 2.25; z <= 26; z += 3) {
    const t = O.targetAt([x, 0, z]); if (t.kind !== 'floor' || dist(t.at, [x, 0, z]) > 0.3) continue;
    if (ST.squadThrower(game, team, t.at, kind)) continue;
    const th = ST.squadThrowPlan(game, team, t.at, kind, true); if (!th?.post) continue;
    if (kind === 'frag' && ST.squadFragCatches(game, th.rest, t.at)) continue;
    out.push({ at: t.at, th, d: dist(th.gd.char.pos, th.post) });
  }
  return out.sort((a, b) => b.d - a.d).slice(0, max);   // (the longest walks first: a walk worth the name)
}

export async function squadthrowWalkCheck(): Promise<boolean> {
  let all = true;
  const kept = { ...SQUADTHROW, frag: FRAG.on, squad: NIGHTBREACH.squad, random: Math.random };
  const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log(`\n=== squadthrow-walk: a teammate walks to where he has a throw ===`);
  const guard = async (name: string, fn: () => Promise<void>) => { try { await fn(); } catch (e) { verdict(false, `${name}: the line threw`, e instanceof Error ? (e.stack ?? e.message).split('\n').slice(0, 3).join(' · ') : String(e)); } };
  /** one walked throw by the keys on `at`; what happened */
  const walked = (R: any, at: number[], kind: 'frag' | 'stun', key: string) => {
    const { game, O } = R, D = game.doorTools, item = kind === 'frag' ? 'frag' : 'flash', id = kind === 'frag' ? FRAG_THERE : STUN_THERE, count = () => kind === 'frag' ? D.frags : D.bangs;
    const n0 = count(), log0 = SQUADTHROW_LOG.length, lines0 = R.lines.length;
    const rows = R.pageAt(at), row = rows[kind === 'frag' ? 0 : 1], pre = game.view.throws;
    if (!game.view.plan) throw new Error('no view: this check has nothing to read');
    R.press(key); R.step(2);
    const man = R.team.find((m: any) => O.of(m)?.order === id), home = man ? [...man.char.pos] : [0, 0, 0], post = man ? [...O.of(man).post] : [0, 0, 0];
    const live = game.view.throws, early = R.items(item).length, spentEarly = n0 - count(), saidGo = R.lines.slice(lines0).some((l: any) => /moving to throw/.test(l.text));
    let maxV = 0; const thrown = R.until(() => { if (man) maxV = Math.max(maxV, Math.hypot(man.char.vel[0], man.char.vel[2])); return SQUADTHROW_LOG.length > log0; }, 25);
    const L = SQUADTHROW_LOG[SQUADTHROW_LOG.length - 1], from = thrown ? dist(L.from, post) : NaN, after = game.view.throws.length, spent = n0 - count();
    const said = R.lines.slice(lines0).map((l: any) => `${l.who}: ${l.text}`).join(' / '), end = man ? [...man.char.pos] : [0, 0, 0];
    return { rows, row, pre, man, home, post, live, early, spentEarly, saidGo, thrown, L, from, after, spent, maxV, id, item, said, end };
  };
  try {
    // ───────────── 1 · the walk, the frag, back to his hold ─────────────
    await guard('walk and frag', async () => {
      const R = await rig(11, true), { O } = R, c = spots(R, 'frag')[0];
      if (!c) { verdict(false, 'a spot nobody has a throw to and one man can walk to a throw for', `none found on the office with the four holding at ${R.team.map((m: any) => p2(m.char.pos)).join(' ')}`); return; }
      const w = walked(R, c.at, 'frag', 'Digit1'), m = w.man;
      verdict(!!w.row && !w.row.off && /^frag there · 2 left/.test(w.row.text) && w.pre.length === 1 && w.pre[0].preview === true && (w.pre[0].walk?.length ?? 0) > 1 && w.pre[0].points.length > 2,
        'a spot nobody has a throw to from where he stands: the frag row is live, and the page shows one plan — who, his walk, the arc — before anything is ordered',
        `spot ${p2(c.at)} · row '${w.row?.text}'${w.row?.off ? ' (off)' : ''} · preview ${w.pre.length} (${w.pre[0]?.man}, walk ${w.pre[0]?.walk?.length ?? 0} pts, arc ${w.pre[0]?.points?.length ?? 0} pts)`);
      verdict(!!m && w.saidGo && w.early === 0 && w.spentEarly === 0 && w.live.length === 1 && !w.live[0].preview && w.live[0].man === m.char.id && dist(w.live[0].from, w.post) < 0.05 && dist(w.home, w.post) > 1,
        '1 gives it: one man is walked to the floor he has a throw from under the row\'s own id, he says so, nothing has left a hand and nothing is spent, and the picture carries his walk',
        `${m?.callsign} from ${p2(w.home)} to ${p2(w.post)} (${f1(dist(w.home, w.post))} m) · said: ${w.saidGo} · items ${w.early} · spent ${w.spentEarly} · live plans ${w.live.length} (${w.live[0]?.gd?.callsign}${w.live[0]?.preview ? ', preview' : ''}, from ${w.live[0] ? p2(w.live[0].from) : '—'}, walk ${w.live[0]?.walk?.length ?? 0} pts)`);
      verdict(w.thrown && w.from < 0.7 && w.spent === 1 && dist(w.L.rest, c.at) < 1.3 && w.after === 0 && w.maxV <= 2.3,
        'on his spot he throws: the frag leaves from that floor, one is spent, it is planned to rest by the spot asked for, the picture is gone with the throw, and he never ran',
        `thrown ${w.thrown} · ${f1(w.from)} m from his spot · spent ${w.spent} · rest ${f1(w.thrown ? dist(w.L.rest, c.at) : NaN)} m from the spot · plans after ${w.after} · fastest ${w.maxV.toFixed(2)} m/s · said: ${w.said} · he stood ${p2(w.end)} for a spot ${p2(w.post)}, the throw to ${p2(c.at)}`);
      const item = R.items('frag')[0]; R.until(() => !R.items('frag').length, 8);
      const back = R.until(() => !!m && dist(m.char.pos, w.home) < 0.6 && O.of(m)?.order === 'hold', 20);
      verdict(back && R.team.every((x: any) => x.state !== 'dead') && !!item,
        'thrown, he walks back to where he stood and holds again, as he was ordered before',
        `${m?.callsign} ${f1(m ? dist(m.char.pos, w.home) : NaN)} m from where he stood · order '${m ? O.of(m)?.order : '—'}' · walks ${SQUADTHROW_STATS.walks} walked ${SQUADTHROW_STATS.walked}`);
    });
    // ───────────── 2 · the stun; a man who was following falls in again ─────────────
    await guard('stun and fall in', async () => {
      const R = await rig(12, true), { game, O } = R;
      O.give('fallIn', O.targetAt(game.player.char.pos)); R.step(60 * 3);
      const c = spots(R, 'stun')[0];
      if (!c) { verdict(false, 'a spot for a walked stun with the squad fallen in', 'none found'); return; }
      const w = walked(R, c.at, 'stun', 'Digit2'), m = w.man;
      verdict(!!m && w.thrown && w.spent === 1 && w.spentEarly === 0 && w.from < 0.7 && dist(w.L.rest, c.at) < 1.3,
        '2 on such a spot: a man walks and throws the stun canister from his spot — one can, spent at the throw',
        `${m?.callsign} ${p2(w.home)} → ${p2(w.post)} · thrown ${w.thrown} · ${f1(w.from)} m from his spot · spent ${w.spent} (before the throw ${w.spentEarly}) · rest ${f1(w.thrown ? dist(w.L.rest, c.at) : NaN)} m off`);
      const back = R.until(() => !!m && O.of(m)?.order === 'fallIn', 6);
      verdict(back, 'a man who was following falls in again once it is thrown', `${m?.callsign} order '${m ? O.of(m)?.order : '—'}'`);
    });
    // ───────────── 3 · another order ends it; the limits; off ─────────────
    await guard('re-ordered, limits, off', async () => {
      const R = await rig(13, true), { game, O } = R, D = game.doorTools, c = spots(R, 'frag')[0];
      if (!c) { verdict(false, 'a spot for the re-order line', 'none found'); return; }
      R.pageAt(c.at); R.press('Digit1'); R.step(20);
      const m = R.team.find((x: any) => O.of(x)?.order === FRAG_THERE), n0 = R.items('frag').length;
      O.give('hold', O.targetAt(game.player.char.pos)); R.step(60 * 6);
      verdict(!!m && O.of(m)?.order === 'hold' && R.items('frag').length === n0 && D.frags === 2 && game.view.throws.length === 0,
        'told to hold while he walks: the throw is off — nothing thrown, nothing spent, nothing drawn',
        `${m?.callsign} order '${m ? O.of(m)?.order : '—'}' · frags ${D.frags} · plans ${game.view.throws.length}`);
    });
  } finally {
    const { frag, squad, random, ...knobs } = kept; Object.assign(SQUADTHROW, knobs); FRAG.on = frag; NIGHTBREACH.squad = squad; Math.random = random;
  }
  console.log(`=== squadthrow-walk: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await squadthrowWalkCheck()) ? 0 : 1);
