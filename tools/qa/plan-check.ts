// THE SQUAD'S PLAN DRAWN ON THE FLOOR (src/game/floorplan.ts → src/ui/planfloor.ts): what the overlay is handed, checked without a picture.
//   bun run tools/qa/plan-check.ts            (also `probes.ts plan`)
// PASS / FAIL each, all headless: the room clear on the office with its squad of four, the defenders left alive on the floor (the plan must list none of
// them). What is checked is `floorPlan(g)` on the Rust game's state: who is listed just after an order, the way from each man's feet to his mark, the menu's rings, the
// key's bar on a door, and the order mode (`menuOnly`, settled on).
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';
import { ringUp, ringDown } from './squadwords';

const FP: any = await import(`${ROOT}/src/game/floorplan.ts`);
const PF: any = await import(`${ROOT}/src/ui/planfloor.ts`);
const { hudSource } = await import(`${ROOT}/src/host/hudsource.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const { PLAN } = FP;
// the plan is read as the page reads it (ui/planfloor.ts plansToDraw over `view.plan`, its own store of kept legs); here a man is found by his id and a door by its name
const floorPlan = (g: any) => { if (g.view.me < 0) throw new Error('no view: a check with nothing to read holds nothing'); const P = PF.plansToDraw(hudSource(g))[0].P, by = (id: number) => g.guards.find((m: any) => m.char.id === id);
  return { ...P, men: P.men.map((m: any) => ({ ...m, gd: by(m.man) })), who: P.who.map((w: any) => ({ ...w, gd: by(w.man) })), throws: P.throws.map((t: any) => ({ ...t, gd: by(t.man) })), doors: P.doors.map((d: any) => ({ ...d, door: d.door ? g.doors.byName(d.door) : null })) }; };

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : '—');
const len = (p: number[][]) => { let s = 0; for (let i = 1; i < p.length; i++) s += dist(p[i - 1], p[i]); return s; };

async function rig(seed: number, ask: boolean, squad = 4) {
  const squad0 = NIGHTBREACH.squad; Object.assign(NIGHTBREACH, { squad });
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game, O = game.orders;
  game.godMode = true;
  const why = game.roomClear.start(seed);
  Object.assign(NIGHTBREACH, { squad: squad0 });
  const R = { W, game, O, why, asked: 0,
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; W.step(DT); if (ask) { floorPlan(game); R.asked += 1; } } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
  };
  R.step(30);
  for (const m of R.team) m.god = true;
  return R;
}
/** a spot of floor the menu takes, a walk of 7‥18 m from Sam */
function farFloor(R: any): number[] | null {
  const g = R.game, s = g.player.char.pos; let best: number[] | null = null, bd = 1e9;
  for (let dx = -16; dx <= 16; dx += 1) for (let dz = -16; dz <= 16; dz += 1) {
    const p = [s[0] + dx + 0.25, 0, s[2] + dz + 0.25], d = Math.hypot(dx, dz); if (d < 7 || d > 14) continue;
    const t = R.O.targetAt(p); if (t.kind !== 'floor' || dist(t.at, p) > 0.3) continue;
    const path = g.col.nav.findPath(s, p, 4000); if (!path) continue; const L = len([s, ...path]); if (L < 7 || L > 18) continue;
    if (Math.abs(L - 11) < bd) { bd = Math.abs(L - 11); best = p; }
  }
  return best;
}

let fails = 0;
const line = (ok: boolean, name: string, detail: string) => { if (!ok) fails++; console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${name} — ${detail}`); };

export async function planCheck(): Promise<boolean> {
  fails = 0;
  return await planLines();
}
async function planLines(): Promise<boolean> {
  console.log("plan-check: the squad's plan drawn on the floor");
  // 0. the tag

  const R = await rig(3, false), g = R.game, O = R.O, team = R.team;
  const hostiles = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').length;
  line(!R.why && team.length === 4 && hostiles > 0, 'staged', `room clear, ${team.length} teammates, ${hostiles} defenders alive${R.why ? ` · refused: ${R.why}` : ''}`);

  // 1. a move order: every man listed with a way from his feet to his mark
  const spot = farFloor(R);
  const why = spot ? O.give('move', O.targetAt(spot), 'all') : 'no floor found';
  R.step(2);
  {
    const P = floorPlan(g), men: any[] = P.men;
    const ok = men.length === 4 && men.every(m => m.path.length >= 2 && dist(m.path[0], m.gd.char.pos) < 0.15 && dist(m.path[m.path.length - 1], m.mark) < 0.06 && dist(m.mark, spot!) < 4 && !m.there && !m.held);
    line(!why && ok, 'a move order lists every man with his way', `${why ? `refused: ${why}` : men.map(m => `${m.tag} ${f1(len(m.path))} m in ${m.path.length} pts`).join(' · ')}`);
    const straight = men.every(m => len(m.path) >= dist(m.gd.char.pos, m.mark) - 0.01 && len(m.path) < 3 * Math.max(4, dist(m.gd.char.pos, m.mark)) + 12);
    const finite = men.every(m => m.path.every((q: number[]) => q.every(Number.isFinite)));
    const up = (q: number[]) => [q[0], 0.6, q[2]];
    const walls = men.reduce((n, m) => n + m.path.slice(1).filter((q: number[], i: number) => g.col.solidBlocked(up(m.path[i]), up(q))).length, 0);
    line(men.length > 0 && straight && finite && walls === 0, 'each way is a real walk, through no wall', `${men.map(m => `${m.tag}: ${f1(len(m.path))} m for ${f1(dist(m.gd.char.pos, m.mark))} m straight`).join(' · ')} · ${walls} legs through something solid`);
    // with his own path set aside: the floor's search (game.ask.way) for each leg, the answer kept by the picture and asking it writes nothing
    { const H = g.ask, way0 = H.way; let asked = 0; H.way = (x: any, y: any, n: number) => { asked++; return way0(x, y, n); };
      try { const W0: any[] = g.view.plan.own.men, hash0 = g.stateWords?.().parts?.join(','); const forced = W0.map((m: any) => PF.wayOf(hudSource(g), g.view.look.plan.reaskSecs, m.man, { ...m.way, own: null })), first = asked, again = W0.map((m: any) => PF.wayOf(hudSource(g), g.view.look.plan.reaskSecs, m.man, { ...m.way, own: null }));
        const real = forced.every((w: any[], i: number) => w.length >= 2 && dist(w[w.length - 1], W0[i].mark) < 0.06 && w.slice(1).every((q: number[], k: number) => !g.col.solidBlocked(up(w[k]), up(q))));
        const same = forced.every((w: any[], i: number) => JSON.stringify(w) === JSON.stringify(again[i]));
        line(W0.length === 4 && first >= 4 && asked === first && real && same && hash0 === g.stateWords?.().parts?.join(','), 'a man with no path of his own: the floor is asked, the answer kept, nothing written', `${first} legs asked of the floor, ${asked - first} more on a second ask · each a real walk to his mark: ${real}`);
      } finally { H.way = way0; } }
    const tags = new Set(men.map(m => m.tag)), tones = men.map(m => m.tone);
    line(tags.size === 4 && tones.filter(t => t === 'red').length === 2 && tones.filter(t => t === 'blue').length === 2, 'who: a tag a man, a colour a pair', men.map(m => `${m.tag}:${m.tone}`).join(' '));
    line(men.length > 0 && men.every(m => m.age < 0.5 && m.aim !== null), 'a fresh order is young, and says what he will watch', men.map(m => `${m.tag} age ${m.age.toFixed(2)} aim ${m.aim === null ? '—' : m.aim.toFixed(2)}`).join(' · '));
    // only our own men
    const own = new Set(team);
    line(men.length > 0 && men.every(m => own.has(m.gd) && m.gd.team === 'operative') && P.who.every((q: any) => own.has(q.gd)), 'our own men only', `${men.length} listed, ${hostiles} defenders on the floor, none of them in the plan`);
  }
  // 2. the way follows his own nav path once he walks it, and shortens
  {
    const before = (floorPlan(g).men as any[]).map(m => len(m.path));
    R.step(90);
    const P = floorPlan(g), men: any[] = P.men, after = men.map(m => len(m.path));
    const walking = men.filter(m => m.path.length >= 2);
    const shorter = walking.length > 0 && walking.every(m => { const i = men.indexOf(m); return after[i] < before[i] + 0.05; }) && after.reduce((a, b) => a + b, 0) < before.reduce((a, b) => a + b, 0) - 2;
    const onOwn = walking.filter(m => m.gd.pathGoal && m.gd.pathI < m.gd.path.length).length;
    line(shorter && onOwn > 0 && walking.every(m => dist(m.path[0], m.gd.char.pos) < 0.15), 'the way shortens from his feet as he walks, on his own nav path', `before ${before.map(f1).join(' ')} m · 1.5 s on ${after.map(f1).join(' ')} m · ${onOwn} of ${walking.length} on their own nav path`);
  }
  // 5. the menu's rings
  {
    ringDown(R.W.input); R.step(); const none = floorPlan(g).who.length, you0 = floorPlan(g).you;
    R.W.cursorAt(spot ?? g.player.char.pos); ringUp(R.W.input, R); R.step(2);
    const who: any[] = floorPlan(g).who, menuMen = O.menu?.men ?? [], you = floorPlan(g).you;
    const ok = none === 0 && who.length === menuMen.length && who.length > 0 && who.every(q => menuMen.includes(q.gd) && dist(q.at, q.gd.char.pos) < 0.01);
    ringDown(R.W.input); R.step(1);
    line(ok && floorPlan(g).who.length === 0, 'the menu up rings the men it goes to', `${who.length} ringed of ${menuMen.length} in the menu · none before, none after`);
    line(!you0 && !!you && dist(you, g.player.char.pos) < 0.01 && !floorPlan(g).you, 'the menu up rings the player, at his feet', `${you ? 'ringed' : 'NOT ringed'} with the menu up · ${you0 ? 'ringed' : 'none'} before, ${floorPlan(g).you ? 'ringed' : 'none'} after`);
  }
  // 6. a door: stacked men's marks are by the door
  {
    const s = g.player.char.pos; let best: any = null, bd = 1e9, bwhy: string | null = 'no door';
    const doors = [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, s) - dist(b.frameCentre, s));
    for (const d of doors) {
      const t = O.targetAt(d.frameCentre); if (t.kind !== 'door') continue;
      const w = O.give('stack', t, 'all'); if (w) { bwhy = w; continue; }
      best = d; bd = dist(d.frameCentre, s); bwhy = null; break;
    }
    R.step(2);
    const men: any[] = floorPlan(g).men;
    const ok = !!best && men.length === 4 && men.every(m => dist(m.mark, best.frameCentre) < 7 && (m.path.length >= 2 || m.there));
    line(ok, 'a stack order: a mark a man by the door', best ? `door ${f1(bd)} m off · ${men.map(m => `${m.tag} ${f1(dist(m.mark, best.frameCentre))} m from it${m.path.length ? `, ${f1(len(m.path))} m to walk` : ''}`).join(' · ')}` : `refused everywhere: ${bwhy}`);
  }
  // 6b. the word: the stacked door is the one door marked, with the key; the word given, the mark goes
  {
    const P0 = floorPlan(g), d0: any[] = P0.doors;
    R.until(() => (floorPlan(g).men as any[]).every(m => m.there), 40);
    const d1: any[] = floorPlan(g).doors, one = d1[0];
    const jamb = one ? dist(one.a, one.b) : 0, mid = one ? [(one.a[0] + one.b[0]) / 2, 0, (one.a[2] + one.b[2]) / 2] : [0, 0, 0];
    line(d0.length === 1 && d1.length === 1 && one.why === 'stack' && one.key === 'F' && one.now && one.tone === 'gold' && Math.abs(jamb - one.door.def.width) < 1e-6 && dist(mid, one.door.frameCentre) < 0.01,
      'a stacked door waits on the word: one bar, the key on it', d1.length ? `${d1.length} door · ${one.why} · key ${one.key} · ${one.tone} · bar ${f1(jamb)} m across its doorway` : 'no door listed');
    R.W.input.pressed.add('KeyF'); R.step(3);   // (the word: on to the next lines with nothing waiting)
  }
  // 6c. a tool given on the word to one pair: marked in that pair's colour; the key only while orders go to them
  {
    O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); R.until(() => g.doorTools.ops.length === 0, 20); R.step(60);
    const s = g.player.char.pos; let door: any = null, w: string | null = 'no door';
    for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, s) - dist(b.frameCentre, s))) { const t = O.targetAt(d.frameCentre); if (t.kind !== 'door') continue; w = O.give('openClear', t, 'red', { onWord: true }); if (!w) { door = d; break; } }
    R.step(3);
    const who0 = O.who; const d1: any[] = floorPlan(g).doors; O.who = 'blue'; const d2: any[] = floorPlan(g).doors; O.who = who0;
    // (the squad's only keys, cmdring.ts CMDRING.only — the page's stand: no key steps who the orders go to, so the word frees whoever waits, and the bar says so)
    const only = CMDRING.only;
    const ok = !!door && d1.length === 1 && d1[0].door === door && d1[0].why === 'tool' && d1[0].tone === 'red' && d1[0].now && d2.length === 1 && !!d2[0].now === only;
    line(ok, only ? 'a tool on the word: the pair\'s colour, and the key for them whoever the orders go to (the squad\'s only keys)' : 'a tool on the word: the pair\'s colour, the key only for the men it would send', door ? `${d1.map(d => `${d.why} · ${d.tone} · now ${d.now}`).join(' | ')} · orders to blue: now ${d2[0]?.now}` : `refused everywhere: ${w}`);
    // the key itself: pressed where the bar has no key on it nothing goes; pressed where it has, it goes
    const T = g.doorTools, waitingOp = () => T.ops.find((o: any) => o.door === door && !o.word);
    O.who = 'blue'; R.W.input.pressed.add('KeyF'); R.step(2); const stillWaits = !!waitingOp();
    O.who = who0; const nowAll = (floorPlan(g).doors as any[])[0]?.now; R.W.input.pressed.add('KeyF'); R.step(2); const wentNow = !waitingOp();
    line(!!door && (only ? !stillWaits : stillWaits && nowAll) && wentNow, 'the key does what the bar says', `orders to blue, F: ${stillWaits ? 'still waiting' : only ? 'went, as the bar said' : 'WENT'}${only ? '' : ` · orders to all (bar now ${nowAll}), F: ${wentNow ? 'went' : 'STILL WAITING'}`}`);
    // 6d. a charge: set, its door is the key's whoever the orders go to; Sam down, no key
    O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); R.until(() => T.ops.length === 0, 30); R.step(60);
    let cd: any = null, cw: string | null = 'no door';
    for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, g.player.char.pos) - dist(b.frameCentre, g.player.char.pos))) { const t = O.targetAt(d.frameCentre); if (t.kind !== 'door') continue; cw = O.give('charge', t, 'blue'); if (!cw) { cd = d; break; } }
    const setOk = R.until(() => T.set.length > 0, 40); R.step(5);
    const c1: any[] = floorPlan(g).doors; O.who = 'red'; const c2: any[] = floorPlan(g).doors; O.who = who0;
    g.player.down = true; const c3: any[] = floorPlan(g).doors; g.player.down = false;
    const cok = !!cd && setOk && c1.length === 1 && c1[0].door === cd && c1[0].why === 'charge' && c1[0].key === 'F' && c1[0].now && c1[0].tone === 'blue' && c2[0]?.now && c3.length === 1 && !c3[0].now;
    // (the key is pressed while the bar says it is live — the pair still walking to their stand-off marks — and the word is kept: the charge goes once
    //  both are stood off. That walk is the men's pace, not the key's, so it is waited out apart from the bang, which must follow the stand-off at once)
    const cop = () => T.ops.find((o: any) => o.kind === 'charge' && o.door === cd);
    R.W.input.pressed.add('KeyF'); R.step(2); const worded = !!cop()?.word || T.set.length === 0, t0 = g.time;
    const stood = R.until(() => T.set.length === 0 || !cop() || cop().phase === 'armed', 25), walk = g.time - t0, blown = R.until(() => T.set.length === 0, 4);
    line(cok && worded && stood && blown, 'a set charge: its door carries the key, and the key blows it', cd ? `${c1.map(d => `${d.why} · ${d.tone} · now ${d.now}`).join(' | ')} · orders to red: now ${c2[0]?.now} · Sam down: now ${c3[0]?.now} · F: the word ${worded ? 'kept' : 'NOT kept'}, stood off ${walk.toFixed(1)} s on, then ${(g.time - t0 - walk).toFixed(1)} s to the bang · ${T.set.length} set after` : `refused everywhere: ${cw}`);
    R.step(120);
    O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); R.step(30);
  }
  // 7. fall in: followers have no mark, nothing is listed
  {
    const w = O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); R.step(5);
    const P = floorPlan(g);
    line(!w && P.men.length === 0, 'fall in: a follower has no mark, nothing drawn', w ? `refused: ${w}` : `${P.men.length} listed`);
  }
  // 10. the pair's room clear drill: F stacks them, and the door it will go through then carries the key
  {
    const Q = await rig(3, false, 2), NB = Q.game.nightBreach, before = floorPlan(Q.game).doors.length;
    Q.W.input.pressed.add('KeyF'); Q.step(2);
    const set = Q.until(() => NB.state === 'set', 30);
    const ds: any[] = floorPlan(Q.game).doors, d = ds[0];
    Q.W.input.pressed.add('KeyF'); Q.step(2);
    line(before === 0 && set && ds.length === 1 && d.why === 'drill' && d.key === 'F' && d.now && d.door === NB.drill?.door && NB.state !== 'set' && floorPlan(Q.game).doors.every((x: any) => x.why !== 'drill'),
      'the drill stacked: its door carries the key, and the key sends it', `state ${set ? 'set' : NB.state} · ${ds.map(x => `${x.why} · key ${x.key} · now ${x.now}`).join(' | ') || 'no door'} · after F: ${NB.state}`);
  }
  // 17. only in the order mode (PLAN.menuOnly): normal play has no man's mark on the floor; an order just given shows and goes; the menu up shows what stands
  {
    const Q = await rig(5, false), game = Q.game, QO = Q.O, sp = farFloor(Q);
    const n = () => floorPlan(game).men.length;
    const w = sp ? QO.give('move', QO.targetAt(sp), 'all') : 'no spot'; Q.step(5);
    const flash = n(), flashMode = floorPlan(game).mode;
    Q.step(Math.round(PLAN.freshSecs * 60) + 30);
    const walking = Q.team.filter((m: any) => !QO.of(m)?.arrived).length, quiet = n();
    // a module's own re-assign (no order given) stamps him anew: still nothing
    const m0 = Q.team[0], A0 = QO.of(m0); QO.assign(m0, { order: A0.order, post: A0.post, face: A0.face, going: A0.going, there: A0.there }); Q.step(2);
    const reassigned = n();
    Q.W.cursorAt(sp ?? game.player.char.pos); ringUp(Q.W.input, Q); Q.step(2);
    const up = n(), upMode = floorPlan(game).mode;
    ringDown(Q.W.input); Q.step(2);
    const down = n();
    line(!w && flash === 4 && !flashMode && walking > 0 && quiet === 0 && reassigned === 0 && up === 4 && upMode && down === 0, 'only in the order mode: an order given shows and goes, normal play is clean, the menu up shows what stands',
      w ? `refused: ${w}` : `just given ${flash} · ${f1(PLAN.freshSecs + 0.5)} s on ${quiet} listed with ${walking} still walking · a module's re-assign ${reassigned} · menu up ${up} · closed ${down}`);
  }
  console.log(fails ? `plan-check: ${fails} FAIL` : 'plan-check: ALL PASS');
  return fails === 0;
}

if (import.meta.main) { const ok = await planCheck(); process.exit(ok ? 0 : 1); }
