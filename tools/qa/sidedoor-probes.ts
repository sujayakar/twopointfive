// The 'sidedoor' family: a door face with NO FLOOR AT EITHER JAMB and no marks authored for it — the office's keypad door between the server room and the
// manager's office, from the server room (racks down one side of the aisle, a cart on the hinge jamb, the corner on the latch's). Until the men worked their
// own marks out (orders.ts workedMarks) every door order on that face was refused: 'no room on that door.' / 'no room to stand off.'
//   bun run tools/qa/probes.ts sidedoor | sidedoor-marks | sidedoor-feet | sidedoor-fight
// All headless (headless.ts's Game under the room clear's staging): nothing here looks at a picture — how a man stands on a mark that is not a wall's is unseen.
//   sidedoor-marks   geometry: neither jamb of that face has floor; the marks the men work out for one to four of them are on free floor, a clear walk from
//                    the doorway's front, off the line of the opening, a body's room apart, the stand-off's clear of a charge's backblast; marks authored
//                    for the face (the hostage rescue's) still win
//   sidedoor-feet    the rows on that face, by four and by the pair, the room beyond kept blind (feet, not the fight): stack up, open and clear, bang and
//                    clear, breach with a charge, hold the door / step in, clear from outside — each is taken, and each entry ends 'in' with every man it
//                    sent across the frame line and nobody left in the opening; and from the manager's side (jamb floor, the keypad's lock): the boot
//   sidedoor-fight   the fight, over seeds: the server room taken by its corridor door, then the manager's office through the side door (a can while there is
//                    one) — the order is taken and ends 'in' every time; what it costs is printed beside the same seeds taking the office by its corridor door
import { standUp, ROOT } from './headless.ts';
import { sample } from './sweep.ts';
import { toolsRow4 } from './order-keys.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR, propGap } = await import(`${ROOT}/src/game/roomclear.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { SIDE_MARKS } = await import(`${ROOT}/src/game/rescue.ts`);

const DT = 1 / 60, DOOR = 'server_manager';
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const at = (p: number[]) => `(${p[0].toFixed(2)}, ${p[2].toFixed(2)})`;
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const DT_DEFAULTS = JSON.parse(JSON.stringify(DOORTOOLS)), RC_DEFAULTS = { ...ROOMCLEAR }, NB_DEFAULTS = { ...NIGHTBREACH }, ORD_DEFAULTS = { ...ORDERS, followBack: [...ORDERS.followBack] };
const restore = () => { Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); Object.assign(ROOMCLEAR, RC_DEFAULTS); Object.assign(NIGHTBREACH, NB_DEFAULTS); Object.assign(ORDERS, ORD_DEFAULTS, { followBack: [...ORD_DEFAULTS.followBack] }); };
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } finally { restore(); } };
const alongOf = (d: any, p: number[]) => { const th = d.def.closedDir; return (p[0] - d.def.hinge[0]) * Math.cos(th) + (p[2] - d.def.hinge[1]) * Math.sin(th); };
const offOf = (d: any, p: number[]) => { const th = d.def.closedDir; return Math.cos(th) * (p[2] - d.def.hinge[1]) - Math.sin(th) * (p[0] - d.def.hinge[0]); };
const inDoorway = (d: any, p: number[]) => { const a = alongOf(d, p); return a > 0 && a < d.def.width && Math.abs(offOf(d, p)) < 0.5; };
const inB = (B: any, p: number[], pad = 0) => p[0] > B.x0 - pad && p[0] < B.x1 + pad && p[2] > B.z0 - pad && p[2] < B.z1 + pad;
// the server room's aisle (Sam, then the men) and the manager's office floor by the side door
const AISLE: number[][] = [[18.6, 0, 8.6], [17.8, 0, 8.2], [17.8, 0, 9.0], [17.0, 0, 8.2], [17.0, 0, 9.0]];
const OFFICE: number[][] = [[25.4, 0, 7.4], [26.2, 0, 7.0], [26.2, 0, 7.8], [27.0, 0, 7.0], [27.0, 0, 7.8]];

/** the room clear staged under a seed with `squad` teammates, Sam in god mode and dark; `blind`: room ids whose defenders are kept dazzled every frame */
async function rig(seed: number, o: { squad?: 2 | 4; blind?: string[] } = {}) {
  restore(); Object.assign(NIGHTBREACH, { squad: o.squad ?? 4 });
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game; const RC = game.roomClear, NB = game.nightBreach, O = game.orders, D = game.doorTools;
  game.godMode = true;
  const why = RC.start(seed);
  const R = { W, game, RC, NB, O, D, why, blind: new Set(o.blind ?? []), downs: [] as { who: any; t: number; by: any }[],
    get ops(): any[] { return NB.ops; }, get up(): any[] { return NB.ops.filter((m: any) => m.state !== 'dead'); },
    room(id: string) { return RC.rooms.find((r: any) => r.def.id === id); },
    door(name: string): any { return game.doors.byName(name); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05;
      for (const id of R.blind) for (const h of R.room(id).defenders) if (h.state !== 'dead') h.dazzledUntil = game.time + 5;
      const was = game.guards.map((x: any) => [x, x.state] as const); W.step(DT);
      for (const [x, st] of was) if (st !== 'dead' && x.state === 'dead') R.downs.push({ who: x, t: game.time, by: game.guards.find((k: any) => k.char.id === x.killerId) ?? null }); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    /** a room's garrison out cold, quietly — the room as it stands once it has been taken */
    empty(id: string) { for (const h of R.room(id).defenders) if (h.state !== 'dead') killGuard(game, h, [0, 0, 1], true, 'struck'); R.step(5); },
    /** Sam and the men stood on these spots (Sam's first) */
    place(P: number[][]) { game.teleportPlayer(P[0]); R.up.forEach((m: any, i: number) => { const p = P[1 + i]; m.char.pos[0] = p[0]; m.char.pos[2] = p[2]; }); R.step(10); },
    give(id: string, name: string, who: any = 'all'): string | null { return O.give(id, O.targetAt(R.door(name).frameCentre), who); },
    lostTo(): string { return R.downs.filter(x => x.who.team === 'operative').map(x => `${x.who.callsign} by ${x.by?.callsign ?? '?'}`).join(', ') || 'nobody'; },
  };
  R.step(30);
  return R;
}

// ---------------------------------------------------------------- marks
export const sidedoorMarks = guarded(async () => {
  const { verdict, done } = family('sidedoor-marks: a door face with no floor at either jamb — the men work their own marks out');
  const R = await rig(1); const { game: g, O, D } = R; const d = R.door(DOOR), nav = g.col.nav, men = R.ops, K = ORDERS, W = d.def.width;
  R.empty('server'); R.place(AISLE);
  const side: 1 | -1 = offOf(d, AISLE[0]) >= 0 ? 1 : -1;   // the server room's face
  const jamb = (along: number): number[] => { const th = d.def.closedDir; return [d.def.hinge[0] + Math.cos(th) * along - Math.sin(th) * side * K.jambOut, 0, d.def.hinge[1] + Math.sin(th) * along + Math.cos(th) * side * K.jambOut]; };
  const latch = jamb(W + K.jambPast), hinge = jamb(-K.jambPast);
  verdict(nav.isBlocked(latch[0], latch[2]) && nav.isBlocked(hinge[0], hinge[2]) && !O.doorMarks.some((m: any) => m.door === d), 'the cause: from the server room neither jamb of the side door has floor (the corner on the latch side, the cart on the hinge side), and the room clear authors no marks for the face',
    `latch jamb ${at(latch)} blocked ${nav.isBlocked(latch[0], latch[2])} · hinge jamb ${at(hinge)} blocked ${nav.isBlocked(hinge[0], hinge[2])} · authored marks on it: ${O.doorMarks.filter((m: any) => m.door === d).length}`);
  const rows: string[] = []; let ok = true;
  const front = [d.frameCentre[0] - Math.sin(d.def.closedDir) * side * K.jambOut, 0.45, d.frameCentre[2] + Math.cos(d.def.closedDir) * side * K.jambOut];
  const good = (P: number[][]) => P.every(p => !nav.isBlocked(p[0], p[2]) && offOf(d, p) * side > 0 && (alongOf(d, p) < -0.2 || alongOf(d, p) > W + 0.2) && !g.col.solidBlocked(front, [p[0], 0.45, p[2]]))
    && P.every((p, i) => P.every((q, j) => i === j || dist(p, q) >= 0.75));
  for (const n of [1, 2, 3, 4]) { const P = O.stackPosts(d, side, men.slice(0, n)), S = D.standoffPosts(d, side, n);
    const okP = !!P && P.length === n && good(P.map((x: any) => x.pos)) && P.filter((x: any) => x.lead).length === 1 && P[0].lead && P.every((x: any) => dist(x.pos, d.frameCentre) >= dist(P[0].pos, d.frameCentre) - 1e-6);
    const okS = !!S && S.length === n && good(S.map((x: any) => x.pos)) && S.every((x: any) => dist(x.pos, d.frameCentre) >= DOORTOOLS.backblastM);
    ok &&= okP && okS;
    rows.push(`${n}: stack ${P ? P.map((x: any) => `${at(x.pos)}${x.lead ? '*' : ''}`).join(' ') : 'none'} · stand-off ${S ? S.map((x: any) => `${at(x.pos)} ${f2(dist(x.pos, d.frameCentre))} m`).join(' ') : 'none'}${okP && okS ? '' : '  ← short'}`); }
  verdict(ok, 'worked out for one to four men: every mark on free floor on that face, a clear walk from the doorway\'s front, off the line of the opening, a body\'s room apart; one lead, the nearest the door; the stand-off outside a charge\'s backblast', rows.join('\n    '));
  const score = (m: any) => m.walk + ORDERS.workSeenM * m.seen, WM = O.workedMarks(d, side), wm = [...WM.marks].sort((a: any, b: any) => score(a) - score(b)), best = wm[0], north = wm.filter((m: any) => alongOf(d, m.pos) > W), south = wm.filter((m: any) => alongOf(d, m.pos) < 0);
  const mean = (l: any[]) => l.reduce((s, m) => s + m.seen, 0) / Math.max(1, l.length);
  verdict(WM.jambless && wm.length >= 8 && wm.every((m: any) => m.seen >= 0 && m.seen <= 1 && !g.props.props.some((q: any) => propGap(q, m.pos) < 0.3)) && dist(best.pos, O.workedPick(d, side, 1, [])[0]) < 1e-6 && alongOf(d, best.pos) < 0 && best.seen < 0.1 && dist(best.pos, SIDE_MARKS.stack[0]) < 0.9,
    'the marks are ranked by the walk from the door plus how much of the room beyond sees each through the open doorway — and the best of them is on the hinge side by the cart, out of the room\'s sight, a stride from the hostage rescue\'s hand-placed stack (a real 0.9 m door leaves a man\'s width of wall between the cart and the jamb, and that is the nearer place)',
    `${wm.length} places · best ${at(best.pos)} seen by ${f2(best.seen)} of the office's floor, ${f2(dist(best.pos, SIDE_MARKS.stack[0]))} m from the rescue's ${at(SIDE_MARKS.stack[0])} · mean seen: hinge side ${f2(mean(south))} (${south.length}), latch side ${f2(mean(north))} (${north.length})`);
  // marks authored for the face still win (the hostage rescue's)
  O.doorMarks.push({ door: d, side, stack: SIDE_MARKS.stack, standoff: SIDE_MARKS.standoff });
  const au = O.stackPosts(d, side, men.slice(0, 2)), auS = D.standoffPosts(d, side, 2); O.doorMarks.pop();
  verdict(!!au && au.every((x: any, i: number) => dist(x.pos, SIDE_MARKS.stack[i]) < 1e-6) && !!auS && auS.every((x: any, i: number) => dist(x.pos, SIDE_MARKS.standoff[i]) < 1e-6), 'marks authored for the face are still the ones used (the hostage rescue\'s pair)', `stack ${au?.map((x: any) => at(x.pos)).join(' ')} · stand-off ${auS?.map((x: any) => at(x.pos)).join(' ')}`);
  // every face that has jamb floor: the same marks, switch on or off
  return done();
});

// ---------------------------------------------------------------- feet
export const sidedoorFeet = guarded(async () => {
  const { verdict, done } = family('sidedoor-feet: every door order on the side door from the server room, by four and by the pair');
  const rows: string[] = []; let ok = true, threw = '';
  // THE ROWS ARE THE BOARD'S, AND WHAT A LOCKED DOOR DOES TO AN ENTRY IS THE RULE'S. Where 'clear from outside' is the one plain order (DOORTOOLS.outsidePlain,
  // on in the page) the board has no 'outside' row: that order is thresholdHold's, asked above it in the list. (The case asked whyNot of a row that is not
  // there and threw; the throw failed the second line too.) Where an entry order halts at a door found locked (DOORTOOLS.testDoor, on in the page) open and
  // clear from the keypad's side tries it, says the lock and holds: the boot is the player's next word, and doortest-check.ts's to hold.
  // SIDEFEET_BREAK=1 = the red path of both: the check asks for the board and the entry as they were before the two rules.
  const BREAK = process.env.SIDEFEET_BREAK === '1'; let ran = 0;
  const CASES = sample([['openClear', 2], ['stack', 4], ['openClear', 4], ['bangClear', 4], ['charge', 4], ['thresholdHold', 4], ['charge', 2], ['outside', 4], ['stack', 2], ['bangClear', 2], ['outside', 2], ['thresholdHold', 2]] as [string, 2 | 4][], 3);
  for (const [id, squad] of CASES) {
    try {
      const R = await rig(3, { squad, blind: ['manager'] }); const { O, D } = R; const d = R.door(DOOR), room = R.room('manager'), n = squad;
      R.empty('server'); R.place(AISLE);
      const def = O.defs.find((x: any) => x.id === id);
      if (id === 'outside' && !BREAK) { const there = !!def; ok &&= !there; rows.push(`outside by ${n}: ${there ? 'THE BOARD HAS THE ROW and it was passed over  ← FAIL' : `no such row on this board: 'clear from outside' is ${toolsRow4(DOORTOOLS)}'s, row 4 of the tools`}`); continue; }
      if (!def) { ok = false; rows.push(`${id} by ${n}: THE BOARD HAS NO SUCH ROW  ← FAIL`); continue; }
      ran++;
      const t = O.targetAt(d.frameCentre), grey = O.whyNot(def, O.forMen(t, R.up), R.up);
      const why = R.give(id, DOOR); R.step(2); const op = D.ops.find((o: any) => o.door === d) ?? null, took = op ? op.men.length : 0, marks0 = op ? op.men.map((m: any) => at(op.post.get(m)?.pos ?? m.char.pos)).join(' ') : '';
      let good = why === null && grey === null, tail = '';
      if (id === 'stack') { const set = R.until(() => R.up.every((m: any) => O.of(m)?.arrived), 25); R.step(60);
        const on = R.up.map((m: any) => dist(m.char.pos, O.of(m).post)); good &&= set && on.every((x: number) => x < 0.3) && !R.up.some((m: any) => inDoorway(d, m.char.pos));
        tail = `all on their marks ${set} (${on.map(f2).join(' / ')} m off) · ${R.up.map((m: any) => `${m.callsign} ${at(m.char.pos)}`).join(' ')}`; }
      else if (id === 'outside') { const held = R.until(() => op?.phase === 'held' || !D.ops.includes(op), 60); R.step(120);
        good &&= !!op && held && op.phase === 'held' && op.crossed.size === 0; tail = `phase ${op?.phase ?? '—'} · over the frame line ${op?.crossed.size ?? '—'}`; }
      else {
        if (id === 'charge') { let res: any = null; for (let i = 0; i < 60 * 60 && op && D.ops.includes(op) && !res; i++) { R.step(); if (op.phase === 'armed') res = D.blow(); }
          good &&= !!res && res.backblast === 0; tail = `blown: ${res ? `knocked ${res.knocked}, stunned ${res.stunned}, backblast ${res.backblast}` : 'never armed'} · `; }
        if (id === 'thresholdHold') { const on = R.until(() => op?.phase === 'thr' || !D.ops.includes(op), 50); R.step(60 * 4); const none = op?.crossed.size === 0;
          const told = R.give('openClear', DOOR); good &&= on && none && told === null; tail = `held on the jambs ${on}, nobody in ${none} · told to step in → ${told} · `; }
        R.until(() => !D.ops.includes(op), 70); R.step(60);
        const sent = op ? [...op.pod.keys()] : [], crossed = sent.filter((m: any) => op.crossed.has(m) || m.state === 'dead').length, stuck = R.up.filter((m: any) => inDoorway(d, m.char.pos)).map((m: any) => m.callsign);
        good &&= !!op && took === n && op.ended === 'in' && sent.length >= Math.min(n, 3) && crossed === sent.length && stuck.length === 0;
        tail += `ended '${op?.ended}' · sent in ${sent.length}, crossed ${crossed}, inside now ${R.up.filter((m: any) => inB(room.def.bounds, m.char.pos, 0.2)).length} · in the opening: ${stuck.join(', ') || 'nobody'} · down ${R.lostTo()}`; }
      ok &&= good; rows.push(`${id} by ${n}: greyed ${grey === null ? 'no' : `"${grey}"`} · give → ${why}${op ? ` · ${took} on it, lead ${op.lead.callsign} · marks ${marks0}` : ''} · ${tail}${good ? '' : '  ← FAIL'}`);
    } catch (e) { threw = `${id} by ${squad}: ${e instanceof Error ? e.stack : e}`; ok = false; }
  }
  verdict(!threw, 'nothing threw', threw || `${ran} runs of ${CASES.length} cases`);
  verdict(ok, 'from the server room every row is offered and taken; the entries end \'in\' with every man they sent across the frame line and nobody left in the opening; the charge is blown with nobody in its backblast; the threshold is held with nobody in until told; clear from outside stays outside', rows.join('\n    '));
  // from the manager's office: jamb floor on that face, and the keypad's lock is on it — the boot
  { const R = await rig(3, { blind: ['server'] }); const { O, D } = R; const d = R.door(DOOR); R.empty('manager'); R.place(OFFICE);
    const lines: string[] = []; { const say = R.game.say.bind(R.game); R.game.say = (gd: any, text: string, ...r: any[]) => { lines.push(text); return say(gd, text, ...r); }; }
    const why = R.give('openClear', DOOR); R.step(2); const op = D.ops.find((o: any) => o.door === d); R.until(() => !D.ops.includes(op), 70); R.step(60);
    const sent = op ? [...op.pod.keys()] : [], kicked = !!op?.beats.some((b: any) => /kick/.test(b.what)) || d.lockBroken;
    const tries = !BREAK, said = lines.some(l => /lock/.test(l));
    if (tries) verdict(why === null && !!op && op.ended === 'found shut' && said && !d.lockBroken && d.locked && (op.crossed.size ?? 0) === 0, 'from the manager\'s office (the keypad\'s side: locked against the team) open and clear is taken, the door is tried, the lock is said and the men hold on it: nobody in, the lock whole (the boot is the next word\'s)', `give → ${why} · locked ${d.locked}`
      + ` · lock broken ${d.lockBroken} · "${lines.filter(l => /lock|kick/.test(l)).join('" "')}" · ended '${op?.ended}' · sent in ${sent.length}, crossed ${op?.crossed.size}`);
    else verdict(why === null && !!op && op.ended === 'in' && kicked && sent.every((m: any) => op.crossed.has(m) || m.state === 'dead'), 'from the manager\'s office (the keypad\'s side: locked against the team) open and clear boots it and goes in', `give → ${why} · lock broken ${d.lockBroken} · "${lines.filter(l => /lock|kick/.test(l)).join('" "')}" · ended '${op?.ended}' · sent in ${sent.length}, crossed ${op?.crossed.size}`); }
  return done();
});

// ---------------------------------------------------------------- the fight
