// The 'pairs' family (the four-man squad's two pairs USED APART — orders.ts men's side / hold arcs / the shy route / the lane rule, doortools.ts the word,
// making room and the pods kept apart; all of it behind 'more than two men', orders.ts `big`), run through probes.ts:
//   bun run tools/qa/probes.ts pairs | pairs-off | pairs-side | pairs-hold | pairs-go | pairs-tool | pairs-haltback | pairs-lanes
// PASS / FAIL each, all headless (the Game of headless.ts under the room clear's staging with NIGHTBREACH.squad = 4). The harness has no light meter: Sam's
// visibility is planted dark and he is in god mode, and what anybody SEES of anybody is guards.ts's direct-light estimate, not the page's. Orders are given
// through the order API (orders.give / doorTools.go) — the menu, the keys and the HUD are not this family's. Nothing here looks at a picture.
//   pairs-off     the pair build is as it was: no 'go' order, hold = the old hold (no arc, low ready, no light), a door order keeps Sam's side; with four the
//                 'go' order is there and refuses when nobody waits
//   pairs-side    a pair is sent to the side of a door IT can reach without crossing the room: Sam in the cubicles (his side of the break room's north door
//                 is the INSIDE) orders red onto it — red stacks on the corridor face, never sets foot in the room, nobody lost; where both sides cost the
//                 same the pair takes Sam's
//   pairs-hold    one pair holds while the other clears: (a) red stacked on the north door keeps its stack while blue opens and clears through the west
//                 door; (b) red told to HOLD in the corridor splits the two ways out between its two men, guns and lights up, and engages what comes down
//                 them unordered while blue bangs and clears — A/B against the plain hold (each man told alone) over seeds
//   pairs-go      two pairs, two doors, one word: both stacked, doorTools.go() / the 'go' order — the two doors are shoved on ONE frame, the pods are dealt
//                 apart, losses counted over seeds; 'go' to one pair sends that pair alone; `onWord` tools (bang at one door, open at the other) wait in
//                 'ready' until the word, then the bang goes first and the quiet door opens on the bang door's go
//   pairs-tool    a door tool given to one pair on a door the other pair is stacked on, all four tools on two doors: the stacked pair steps back to rear
//                 security (its order still 'stack' on that door), the tool's pair takes the jambs, a charge's backblast catches nobody, the tool runs to
//                 its end; a charge on a door with a teammate behind it is refused
//   pairs-haltback  a pair HALTED on a door the other pair is charging (DOORTOOLS.haltBack): blue is given the charge on the locked manager's door, red then 'open and clear' on it —
//                 red's lead tries the handle, the order ends 'found shut'. Red steps back to rear security, to the marks a pair stacked there
//                 BEFORE the charge order is sent to, and the charge catches nobody; no charge on the door: nobody steps back; Sam at the door (put
//                 off it, and as played); a charge left set with its order gone. HALTBACK_BREAK=1 = blue's charge is never ordered: must fail
//                 (2026-11-07: the key is settled on and not written. Out: the key-off runs — 'KEY ON against KEY OFF', the two 'KEY OFF, the fault'
//                 CONTROLs, the key-off half of the no-charge CONTROL — and the journal. Out of the rest of the file with the same day: the
//                 defenders' grenades held off in pairs-hold (a) and (b), and the plain-stack run of the tool under a charge, COVER.stacked off)
//   pairs-lanes   lines of fire: a posted man lowers his muzzle while a man of the OTHER pair crosses his line (the fights' own lanes are asked in pairs-hold and pairs-go)
import { doing, notesOf } from './squadwords';
import { standUp, ROOT } from './headless.ts';
import { sample, swept } from './sweep.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS, sideOf } = await import(`${ROOT}/src/game/orders.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const names = (l: any[]) => l.map((g: any) => g.callsign).join(', ');
const wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const DT_DEFAULTS = JSON.parse(JSON.stringify(DOORTOOLS)), RC_DEFAULTS = { ...ROOMCLEAR }, NB_DEFAULTS = { ...NIGHTBREACH }, ORD_DEFAULTS = { ...ORDERS, followBack: [...ORDERS.followBack] };
/** every probe leaves the knob tables as it found them, pass, fail or throw */
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } finally { Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); Object.assign(ROOMCLEAR, RC_DEFAULTS); Object.assign(NIGHTBREACH, NB_DEFAULTS); Object.assign(ORDERS, ORD_DEFAULTS, { followBack: [...ORD_DEFAULTS.followBack] }); } };

// where Sam stands: the corridor outside both of the break room's doors · by the manager's door · in the cubicles (west of the break room)
const CORRIDOR: number[] = [25.0, 0, 11.0], MANAGER: number[] = [24.0, 0, 11.2], CUBICLES: number[] = [25.0, 0, 18.9];
/** the corridor spot the holding pair is put on: east of the manager's glass, between the break room's north door and the storage door */
const FLANK: number[] = [28.5, 0, 11.0];
const BREAKROOM = { x0: 28.12, x1: 35.88, z0: 12.26, z1: 23.88 };
const inBounds = (b: { x0: number; x1: number; z0: number; z1: number }, p: number[], m = 0) => p[0] > b.x0 - m && p[0] < b.x1 + m && p[2] > b.z0 - m && p[2] < b.z1 + m;

/** the family's tally over every fighting run made in this process: runs · teammates' rounds · the ones through a teammate · teammates down to a teammate · lost */
const TALLY = { runs: 0, rounds: 0, through: [] as string[], fratricide: [] as string[], lost: 0, heldForFriend: 0 };

/** One game under a seed: the room clear staged with `squad` teammates (default four), Sam in god mode, dark, standing at `sam`.
 *  `quiet`: the defenders kept dazzled every frame (they neither see nor fire) — for the probes that are about feet, not the fight. */
async function rig(seed: number, sam: number[], o: { squad?: 2 | 4; quiet?: boolean; tag?: string } = {}) {
  Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); Object.assign(ROOMCLEAR, RC_DEFAULTS); Object.assign(NIGHTBREACH, NB_DEFAULTS, { squad: o.squad ?? 4 });
  Math.random = seeded(seed);
  const W: any = await standUp(); const game = W.game; const RC = game.roomClear, NB = game.nightBreach, O = game.orders, D = game.doorTools;
  game.godMode = true; RC.start(seed);
  const seen = new Set<any>(); let tallied = false;
  const R = { W, game, RC, NB, O, D, quiet: !!o.quiet, downs: [] as { who: any; t: number; by: any }[], rounds: 0, through: [] as string[], post: null as null | (() => void),
    get ops(): any[] { return NB.ops; }, get hostiles(): any[] { return NB.hostiles; },
    step(n = 1) {
      for (let i = 0; i < n; i++) {
        game.player.visibility = 0.05;
        if (R.quiet) for (const h of NB.hostiles) if (h.state !== 'dead') h.dazzledUntil = game.time + 5;
        const was = game.guards.map((x: any) => [x, x.state] as const);
        W.step(DT);
        for (const e of game.events) { if (seen.has(e)) continue; seen.add(e);
          // LANES: a teammate's round whose line passes through a living teammate's body (combat.ts lets a friend's round through him by rule, so nobody can fall
          // to it — what can be measured is how often a man fires with a friend in his lane)
          if (e.kind !== 'guardShot' || !e.from || !e.to) continue; const sh = R.ops.find((x: any) => x.char.id === e.who); if (!sh) continue; R.rounds++;
          const ax = e.from[0], az = e.from[2], bx = e.to[0] - ax, bz = e.to[2] - az, L2 = bx * bx + bz * bz || 1;
          for (const m of R.ops) { if (m === sh || m.state === 'dead') continue; const k = ((m.char.pos[0] - ax) * bx + (m.char.pos[2] - az) * bz) / L2; if (k <= 0.02 || k >= 1) continue;
            const y = e.from[1] + (e.to[1] - e.from[1]) * k - m.char.pos[1]; if (Math.hypot(ax + bx * k - m.char.pos[0], az + bz * k - m.char.pos[2]) < 0.28 && y > 0 && y < 1.8) { R.through.push(`${sh.callsign} through ${m.callsign} at ${f2(game.time)} s`); break; } } }
        for (const [x, st] of was) if (st !== 'dead' && x.state === 'dead') R.downs.push({ who: x, t: game.time, by: game.guards.find((k: any) => k.char.id === x.killerId) ?? null });
        R.post?.();
      }
    },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(name: string): any { return game.doors.list.find((d: any) => d.def.name === name); },
    target(name: string) { return O.targetAt(R.door(name).frameCentre); },
    give(id: string, name: string, who: any = 'all', opts?: { onWord?: boolean }): string | null { return O.give(id, R.target(name), who, opts); },
    here() { return O.targetAt(game.player.char.pos); },
    men(who: any, down = false): any[] { return O.men(who, { down }); },
    set(who: any = 'all', secs = 30) { return R.until(() => R.men(who).every((m: any) => O.of(m)?.arrived), secs); },
    lost(who: any = 'all'): any[] { return R.men(who, true).filter((m: any) => m.state === 'dead'); },
    killsBy(men: any[]): any[] { return NB.hostiles.filter((h: any) => h.state === 'dead' && men.some((m: any) => m.char.id === h.killerId)); },
    where(who: any = 'all') { return R.men(who, true).map((m: any) => `${m.callsign} ${m.state === 'dead' ? 'DOWN' : at(m.char.pos)} [${doing(O.g, m)}]`).join(' · '); },
    beats(op: any): string { return (op?.beats ?? []).map((b: any) => `${f2(b.t)} ${b.what}`).join(' | '); },
    beatT(op: any, what: string): number { return op?.beats.find((b: any) => b.what === what || b.what.startsWith(what))?.t ?? NaN; },
    /** friendly fire: a teammate down by a teammate's (or Sam's) hand */
    get fratricide(): string[] { return R.downs.filter(x => x.who.team === 'operative' && (x.by?.team === 'operative' || x.who.killerId === game.player.char.id)).map(x => `${x.who.callsign} by ${x.by?.callsign ?? 'Sam'}`); },
    /** file this run under the family's tally (once; the quiet runs are not fights and are left out) */
    tally() { if (tallied || R.quiet) return; tallied = true; TALLY.runs++; TALLY.rounds += R.rounds; TALLY.lost += R.lost().length; TALLY.heldForFriend += game.fight.stats.heldForFriend;
      for (const x of R.through) TALLY.through.push(`${o.tag ?? ''}/${seed}: ${x}`); for (const x of R.fratricide) TALLY.fratricide.push(`${o.tag ?? ''}/${seed}: ${x}`); },
  };
  R.step(30); game.teleportPlayer(sam); R.step(5);
  return R;
}


// ============================================================================================================================ pairs-side
export const pairsSide = guarded(async () => {
  const { verdict, done } = family('pairs-side: a pair takes the side of a door it can reach');
  const R = await rig(5, CUBICLES); const { O } = R; const dn = R.door('breakroom_n'), red = R.men('red');
  const t = R.target('breakroom_n'), mine = O.forMen(t, red).side, samSide = sideOf(dn, R.game.player.char.pos);
  verdict(t.side === samSide && mine === -samSide, 'Sam in the cubicles: his side of the break room\'s north door is the inside; red\'s is the other face (the corridor)', `Sam's side ${samSide} · red's ${mine}`);
  let inside = 0; R.post = () => { for (const m of R.men('red')) if (inBounds(BREAKROOM, m.char.pos, -0.3)) inside++; };
  const why = R.give('stack', 'breakroom_n', 'red'); const ok = R.set('red', 35); R.post = null;
  const sides = red.map((m: any) => sideOf(dn, m.char.pos)), near = red.map((m: any) => dist(m.char.pos, dn.frameCentre));
  verdict(why === null && ok && sides.every((s: number) => s === mine) && near.every((x: number) => x < 2.6), 'ordered onto it, red stacks on the corridor face of the door', `give → ${why} · ${R.where('red')} · ${near.map(f2).join(' / ')} m from the frame`);
  verdict(inside === 0 && R.lost().length === 0, 'red never set foot in the break room on the way, and nobody is lost', `man-frames inside the room ${inside} · lost ${names(R.lost()) || 'none'}`);
  const blue = R.men('blue'), tw = R.target('breakroom_w'), bs = O.forMen(tw, blue).side;
  verdict(bs === tw.side, 'the west door from the cubicles: blue takes Sam\'s side (it is the one it can reach)', `Sam's side ${tw.side} · blue's ${bs}`);
  R.game.teleportPlayer(CORRIDOR); R.step(5); const ts = R.target('storage'), ss = O.forMen(ts, blue).side;
  verdict(ss === ts.side, 'a corridor door from the corridor: the pair takes Sam\'s side', `Sam's side ${ts.side} · blue's ${ss}`);
  R.tally(); return done();
});

// ============================================================================================================================ pairs-hold
export const pairsHold = guarded(async () => {
  const { verdict, done } = family('pairs-hold: one pair holds while the other clears');
  // ---- (a) red stacked on the north door, blue opens and clears through the west door
  { const rows: string[] = []; let kept = 0, ended = 0, n = 0, lost = 0, drift = 0; const ff: string[] = [], thr: string[] = [];
    for (const seed of sample([5, 2, 9], 1)) { n++; const R = await rig(seed, CORRIDOR, { tag: 'hold-a' }); const { O, D } = R; const dn = R.door('breakroom_n'), red = R.men('red');
      R.give('stack', 'breakroom_n', 'red'); R.set('red', 30); const posts = new Map(red.map((m: any) => [m, [...O.of(m).post]] as const));
      const why = R.give('openClear', 'breakroom_w', 'blue'); const op = D.ops[0], sent = op ? [...op.men] : []; let stayed = true, far = 0;   // (who the order TOOK: a man lost in the room is off op.men by the end)
      R.post = () => { for (const m of R.men('red')) { const a = O.of(m); if (a?.order !== 'stack' || a.door !== dn) stayed = false; far = Math.max(far, dist(m.char.pos, posts.get(m)! as any)); } };
      R.until(() => !D.ops.includes(op), 45); R.step(240); R.post = null; R.tally();
      const good = why === null && !!op && sent.length === 2 && sent.every((m: any) => O.pairOf(m) === 'blue'); if (stayed && good) kept++; if (op?.ended === 'in') ended++;
      lost += R.lost().length; drift = Math.max(drift, far); ff.push(...R.fratricide); thr.push(...R.through);
      rows.push(`seed ${seed}: blue's order → ${why} (${sent.length} men), ended '${op?.ended}' · red kept its stack ${stayed}, most it moved off its marks ${f2(far)} m · lost ${names(R.lost()) || 'none'} · defenders down ${R.hostiles.filter((h: any) => h.state === 'dead').length}`); }
    verdict(kept === n && ended === n, 'red keeps its stack on the north door from the order to the end while blue — two men, blue alone — opens and clears through the west door', rows.join('\n    '));
    verdict(drift < 0.8, 'red stays on its marks (within 0.8 m) the whole time', `most ${f2(drift)} m`);
    verdict(ff.length === 0 && thr.length === 0, 'no teammate down to a teammate, no round fired through one', `${ff.join(' · ') || 'none'} · through ${thr.length} · ${lost} lost to defenders in ${n} runs`); }
  // ---- (b) red HOLDS a flank in the corridor (arcs dealt) while blue bangs and clears. (2026-11-07: one seed, to its order's end; out went the kills over seeds and the A/B against the plain hold, which took a dozen fights)
  { const SEEDS = sample([2, 5, 9, 11, 14, 17], 1); let arcsOk = 0, stayed = 0, thr = 0, ff = 0; const rows: string[] = [];
    for (const seed of SEEDS) { const R = await rig(seed, CORRIDOR, { tag: 'hold-arcs' }); const { O, D } = R; const red = R.men('red');
      O.give('move', O.targetAt(FLANK), 'red'); R.set('red', 25);
      O.give('hold', R.here(), 'red');
      R.step(60); const as = red.map((m: any) => O.of(m)), spots = red.map((m: any) => [...m.char.pos]);
      const yaws = as.map((a: any, i: number) => a.watch ? Math.atan2(a.watch[0] - spots[i][0], a.watch[2] - spots[i][2]) : NaN);
      if (as.every((a: any) => a.order === 'hold' && a.watch && a.upper === 'aim' && a.light) && Math.abs(wrap(yaws[0] - yaws[1])) > Math.PI / 2) arcsOk++;
      const words = red.map((m: any, i: number) => `${m.callsign} ${at(spots[i])} [${as[i].there}]`).join(' / ');
      R.give('bangClear', 'breakroom_w', 'blue'); const op = D.ops[0]; let held = true;
      R.post = () => { for (let i = 0; i < red.length; i++) if (red[i].state !== 'dead' && (O.of(red[i])?.order !== 'hold' || dist(red[i].char.pos, spots[i]) > 0.8)) held = false; };
      R.until(() => !D.ops.includes(op), 60); R.post = null; R.tally();
      thr += R.through.length; ff += R.fratricide.length; if (held) stayed++;
      rows.push(`arcs seed ${seed}: ${words} · red lost ${R.lost('red').length}, red killed ${R.killsBy(red).length} (${names(R.killsBy(red)) || '-'}) · blue's order ended '${op?.ended}'`); }
    verdict(arcsOk === SEEDS.length, 'told to hold as a pair, red splits the ways out: each man a watch point, gun and light up, the two arcs more than 90° apart', rows.join('\n    '));
    verdict(stayed === SEEDS.length, 'the holding pair stays where it was told (order \'hold\', within 0.8 m) while it lives', `${stayed} of ${SEEDS.length} runs`);
    verdict(thr === 0 && ff === 0, 'no teammate down to a teammate, no round fired through one', `through ${thr} · fratricide ${ff}`); }
  return done();
});

// ============================================================================================================================ pairs-go
export const pairsGo = guarded(async () => {
  const { verdict, done } = family('pairs-go: two pairs, two doors, one word');
  // ---- (a) both stacked on a door each of the break room, the word
  { const SEEDS = sample([2, 5, 9, 11, 14], 1); const rows: string[] = []; let same = 0, inAll = 0, apart = 0, lost = 0, thr = 0, ff = 0;
    for (const seed of SEEDS) { const R = await rig(seed, CORRIDOR, { tag: 'go' }); const { O, D } = R;
      R.give('stack', 'breakroom_n', 'red'); R.give('stack', 'breakroom_w', 'blue'); R.set('all', 35);
      const said = seed === 5 ? O.give('go', R.here(), 'all') : (R.W.input.pressed.add('KeyF'), R.step(), null);   // (one of the runs through the order itself, as a menu row would give it)
      const ops = [...D.ops]; R.until(() => !D.ops.length, 45); R.step(240); R.tally();
      const gos = ops.map((op: any) => R.beatT(op, 'go')), pods = ops.flatMap((op: any) => [...op.pod.values()].map((p: any) => p.pos)); let near = Infinity;
      for (let a = 0; a < pods.length; a++) for (let b = a + 1; b < pods.length; b++) near = Math.min(near, dist(pods[a], pods[b]));
      const two = ops.length === 2 && new Set(ops.map((op: any) => op.door.def.name)).size === 2;
      if (two && Math.abs(gos[0] - gos[1]) < 1.5 * DT) same++;
      if (two && ops.every((op: any) => op.ended === 'in' && [...op.pod.keys()].every((m: any) => op.crossed.has(m) || m.state === 'dead'))) inAll++;
      if (pods.length >= 3 && near >= DOORTOOLS.podApart - 0.01) apart++;
      const left = R.hostiles.slice(0, 2).filter((h: any) => h.state !== 'dead').length;
      lost += R.lost().length; thr += R.through.length; ff += R.fratricide.length;
      rows.push(`seed ${seed}: go → ${said} · ${ops.map((op: any, i: number) => `${op.door.def.name} (${names(op.men)}) go at ${f2(gos[i])}, ended '${op.ended}'`).join(' · ')} · pods ${pods.map(at).join(' ')} (nearest two ${f2(near)} m) · lost ${names(R.lost()) || 'none'} · the room's defenders left ${left}`); }
    verdict(same === SEEDS.length, 'on the word both pairs go through their own door on the SAME frame', rows.join('\n    '));
    verdict(inAll === SEEDS.length, 'every man sent in crosses his frame line (or falls trying) and both orders end \'in\'', `${inAll} of ${SEEDS.length} runs`);
    verdict(apart === SEEDS.length, `the four are dealt places inside that are apart (no two within ${DOORTOOLS.podApart} m)`, `${apart} of ${SEEDS.length} runs`);
    verdict(thr === 0 && ff === 0, 'two pairs converging in one room: no teammate down to a teammate, no round fired through one', `through ${thr} · fratricide ${ff}`); }
  // ---- (c) tools given `onWord`: a bang at the north door, a quiet open at the west door
  { const SEEDS = sample([2, 5, 9], 1); const rows: string[] = []; let waited = 0, ordered = 0, noted = 0, lost = 0, thr = 0, ff = 0, inAll = 0;
    for (const seed of SEEDS) { const R = await rig(seed, CORRIDOR, { tag: 'go-bang' }); const { O, D } = R;
      const w1 = R.give('bangClear', 'breakroom_n', 'red', { onWord: true }), w2 = R.give('openClear', 'breakroom_w', 'blue', { onWord: true });
      const ready = R.until(() => D.ops.length === 2 && D.ops.every((o: any) => o.phase === 'ready'), 45); R.step(180);
      const still = D.ops.length === 2 && D.ops.every((o: any) => o.phase === 'ready'), notes = notesOf(D.g).filter((s: string) => s.includes('on your word'));
      if (ready && still && w1 === null && w2 === null) waited++; if (notes.length === 2) noted++;
      R.W.input.pressed.add('KeyF'); R.step(); const said = null; const ops = [...D.ops], red = ops.find((o: any) => o.kind === 'bangClear'), blue = ops.find((o: any) => o.kind === 'openClear');
      R.until(() => !D.ops.length, 45); R.step(240); R.tally();
      const bang = R.beatT(red, 'bang'), rgo = R.beatT(red, 'go'), bshove = R.beatT(blue, 'shove'), bgo = R.beatT(blue, 'go');
      if (bang < bgo && Math.abs(bshove - rgo) < 2.5 * DT && bgo >= rgo) ordered++;
      if (ops.every((op: any) => op.ended === 'in')) inAll++;
      lost += R.lost().length; thr += R.through.length; ff += R.fratricide.length;
      rows.push(`seed ${seed}: both ready ${ready}, still ready 3 s on ${still} · notes ${notes.length} · go → ${said} · red (bang, north) bang ${f2(bang)} go ${f2(rgo)} · blue (open, west) shove ${f2(bshove)} go ${f2(bgo)} · ended ${ops.map((op: any) => `'${op.ended}'`).join(' ')} · lost ${names(R.lost()) || 'none'}`); }
    verdict(waited === SEEDS.length, 'given `onWord`, both pairs stack, call ready and WAIT — three seconds on they are still waiting', rows.join('\n    '));
    verdict(noted === SEEDS.length, 'each waiting pair is on the door tools\' status notes (\'… on your word\')', `${noted} of ${SEEDS.length} runs`);
    verdict(ordered === SEEDS.length && inAll === SEEDS.length, 'on the word the bang goes first; the quiet door is shoved as the bang door\'s men go, so both pairs enter after the bang', `${ordered} of ${SEEDS.length} in order · ${inAll} of ${SEEDS.length} both ended 'in'`);
    verdict(thr === 0 && ff === 0 && lost <= 1, 'no teammate down to a teammate, no round through one, at most one teammate lost over the seeds', `through ${thr} · fratricide ${ff} · lost ${lost}`); }
  return done();
});

// ============================================================================================================================ pairs-tool
export const pairsTool = guarded(async () => {
  const { verdict, done } = family('pairs-tool: a door tool on a door the other pair is stacked on');
  const rows: string[] = []; let n = 0, kept = 0, back = 0, clear = 0, ran = 0, lost = 0, blast = 0, charges = 0, thr = 0, ff = 0, threw = '';
  // (a charge first and last: a sample of the sweep asks the two doors under a charge; QA_SWEEP=all asks every tool on both)
  const CASES: [string, string][] = [['manager', 'charge'], ['manager', 'openClear'], ['manager', 'bangClear'], ['manager', 'outside'], ['breakroom_w', 'openClear'], ['breakroom_w', 'bangClear'], ['breakroom_w', 'outside'], ['breakroom_w', 'charge']];
  for (const [name, tool] of sample(CASES, 1)) { const seed = 5; n++;
    try { const R = await rig(seed, name === 'manager' ? MANAGER : CUBICLES, { tag: `tool-${tool}-${name}` }); const { O, D } = R; const d = R.door(name), red = R.men('red');
      R.give('stack', name, 'red'); R.set('red', 30); const before = red.map((m: any) => [...m.char.pos]), was = D.stats.steppedBack;
      const why = R.give(tool === 'outside' ? 'thresholdHold' : tool, name, 'blue') /* ('clear from outside' is the one plain order that holds the door until told) */; const op = D.ops[0]; const good = why === null && !!op && op.men.length === 2 && op.men.every((m: any) => O.pairOf(m) === 'blue'); R.step(2);
      const as = red.map((m: any) => O.of(m)), stepped = D.stats.steppedBack - was;
      const bluePosts = op ? [...op.post.values()].map((p: any) => p.pos) : [], redPosts = as.map((a: any) => a.post);
      let near = Infinity; for (const a of redPosts) for (const b of bluePosts) near = Math.min(near, dist(a, b));
      let stayedStack = true, res: any = null;
      R.post = () => { for (const m of R.men('red')) { const a = O.of(m); if (a?.order !== 'stack' || a.door !== d) stayedStack = false; } };
      for (let i = 0; i < 60 * 70 && op && D.ops.includes(op) && op.phase !== 'held'; i++) { R.step(); if (op.phase === 'armed' && !res) res = D.blow(); }
      const endedAs = op ? (D.ops.includes(op) ? `held outside` : op.ended) : 'no order'; R.step(180); R.post = null; R.tally();
      if (good && stayedStack) kept++;
      if (stepped === 2 && as.every((a: any, i: number) => dist(a.post, before[i]) > 0.5 && String(a.there).startsWith('rear security'))) back++;
      if (near >= 0.8) clear++;
      if (tool === 'outside' ? endedAs === 'held outside' || endedAs === 'in' : endedAs === 'in') ran++;
      if (tool === 'charge') { charges++; blast += res ? res.backblast : 99; }
      lost += R.lost().length; thr += R.through.length; ff += R.fratricide.length;
      rows.push(`${tool} on ${name}: give → ${why} · red ${red.map((m: any, i: number) => `${m.callsign} ${at(before[i])} → ${at(as[i].post)} [${as[i].there}]`).join(', ')} · blue on ${bluePosts.map(at).join(' ')} (nearest red mark ${f2(near)} m)${res ? ` · blown: knocked ${res.knocked}, stunned ${res.stunned}, backblast ${res.backblast}` : ''} · ended '${endedAs}' · red still on 'stack' ${stayedStack} · lost ${names(R.lost()) || 'none'}`);
    } catch (e) { threw = `${tool}/${name}: ${e instanceof Error ? e.stack : e}`; } }
  verdict(!threw, 'nothing threw', threw || `${n} runs`);
  verdict(kept === n, 'the tool is taken by blue alone (two men) and red\'s order stays \'stack\' on that door from start to end', rows.join('\n    '));
  verdict(back === n, 'both of red step back off the jambs to rear security', `${back} of ${n} runs`);
  verdict(clear === n, 'red\'s new marks are clear of the marks blue works from (0.8 m or more)', `${clear} of ${n} runs`);
  verdict(ran === n, 'the tool runs to its end with the other pair there (\'in\'; clear from outside: held outside or in)', `${ran} of ${n} runs`);
  verdict(charges === swept(1, 2) && blast === 0, 'the charge: nobody of ours inside its backblast when Sam blows it', `${charges} charges · backblast ${blast}`);
  verdict(thr === 0 && ff === 0, 'no teammate down to a teammate, no round fired through one', `through ${thr} · fratricide ${ff} · ${lost} teammates lost to defenders in ${n} runs`);
  // ---- a teammate BEHIND the door: the charge is refused
  { const R = await rig(5, CUBICLES, { quiet: true }); const { O, D } = R;
    O.give('move', O.targetAt([29.6, 0, 17.6]), 'red'); R.set('red', 30); R.step(30); { const d = R.door('breakroom_w'); d.angle = 0; d.vel = 0; d.latched = true; d.place(); } R.step(2);   // (red went in through it: the leaf shut behind them, as the staging had it)
    const inside = R.men('red').filter((m: any) => inBounds(BREAKROOM, m.char.pos, 0)), why = R.give('charge', 'breakroom_w', 'blue'), ops = D.ops.length;
    verdict(inside.length > 0 && typeof why === 'string' && /behind that door/.test(why) && ops === 0, 'a charge on a door with a teammate just behind it is refused, by name', `red inside the room: ${names(inside) || 'none'} · give('charge') → ${why}`); }
  return done();
});

// ============================================================================================================================ pairs-haltback
export const pairsHaltback = guarded(async () => {
  const { verdict, done } = family('pairs-haltback: a pair halted \'found shut\' on a door the other pair is charging steps back out of the blast (DOORTOOLS.haltBack)');
  const BREAK = process.env.HALTBACK_BREAK === '1'; if (BREAK) console.log('  (HALTBACK_BREAK: blue\'s charge is never ordered — the rear security and backblast lines must fail)');
  type Man = { c: string; order: string; door: string; there: string; post: number[]; pos: number[] };
  type Run = { w1: string | null; w2: string | null; ended: string; said: string; halt: Man[]; atBlow: Man[]; blue: number[][]; frame: number[]; res: any; chEnded: string; stale: string[]; stepped: number; notes: string[]; end: Man[]; orphan: string };
  const AWAY: number[] = [19.5, 0, 11.2], EAST: number[] = [34.5, 0, 11.0];
  /** 'halt' = blue's charge first, then red's entry halts on the door · 'ref' = red plain-stacked on it BEFORE blue's charge order (the step back as it has
   *  always been asked) · 'bare' = red's entry halts on the door and nobody has a charge on it. Sam stands down the corridor, off the door: at its lock
   *  rows he would KNOW it locked (doortools.ts testStacks) and no order halts on a door the team knows. `near` = both orders given with Sam AT the door, as
   *  a player gives them — his body is then in the stack's way and the entry's last man is dealt a mark further down the wall. 'leave': he is then put
   *  down the corridor and what he learnt at the lock is taken back, by hand, so that the order halts; 'stay': he stays, as played —
   *  the order halts or not by the game's own rules. 'set' = blue's charge set and armed, blue then sent off down the corridor (its charge order ends,
   *  the charge stays on the door), and only then red's entry, which halts on a door with a teammate's charge and no order on it */
  const play = async (asked: 'halt' | 'ref' | 'bare' | 'set', near: false | 'leave' | 'stay' = false): Promise<Run> => {
    const mode = BREAK && asked !== 'ref' ? 'bare' : asked;   // (the red path: no charge is ordered on the door)
    const R = await rig(5, near ? MANAGER : AWAY, { quiet: true, tag: `haltback-${mode}` }); const { O, D } = R; const d = R.door('manager'), red = R.men('red');
    const snap = (): Man[] => red.map((m: any) => { const a = O.of(m); return { c: m.callsign, order: String(a?.order), door: String(a?.door?.def.name), there: String(a?.there), post: a?.post ? [...a.post] : [NaN, 0, NaN], pos: [...m.char.pos] }; });
    const was = D.stats.steppedBack, log0 = D.log.length; let w1: string | null = null, w2: string | null = null, ended = '', cop: any = null, gone: any = null, orphan = '';
    if (mode === 'ref') { w2 = R.give('stack', 'manager', 'red'); R.set('red', 30); w1 = R.give('charge', 'manager', 'blue'); cop = D.ops[0]; ended = 'stacked beforehand'; R.step(2); }
    else { if (mode === 'halt') { w1 = R.give('charge', 'manager', 'blue'); cop = D.ops[0]; R.step(5); }
      if (mode === 'set') { w1 = R.give('charge', 'manager', 'blue'); gone = D.ops[0]; R.until(() => gone.phase === 'armed' || !D.ops.includes(gone), 45); const ph = gone.phase;
        const wm = O.give('move', O.targetAt(EAST), 'blue'); R.set('blue', 30); const c = D.chargeOn(d);
        orphan = `armed '${ph}' · blue moved off → ${wm} · its charge order ${D.ops.includes(gone) ? 'STILL LIVE' : `ended '${gone.ended}'`} · on the door: ${c ? `a charge set by ${c.by}` : 'NO CHARGE'} · door orders live ${D.ops.length}`; }
      w2 = R.give('openClear', 'manager', 'red'); const rop = D.ops.find((x: any) => x !== cop);
      if (mode === 'halt' && near === 'leave') { R.game.teleportPlayer(AWAY); R.step(2); D.tried.clear(); }
      if (rop) { R.until(() => !D.ops.includes(rop), 30); ended = String(rop.ended); } R.step(2); }
    const said = red.map((m: any) => m.bubble?.text).filter(Boolean).join(' / '), halt = snap();
    let res: any = null, atBlow = halt;
    if (cop) { R.until(() => cop.phase === 'armed' || !D.ops.includes(cop), 45); R.set('red', 20); R.step(30); atBlow = snap(); if (cop.phase === 'armed') res = D.blow(); R.until(() => !D.ops.length, 50); }
    else if (mode === 'set') { R.set('red', 20); R.step(30); atBlow = snap(); const c = D.chargeOn(d); if (c) res = D.blow(c); R.step(180); }
    else R.step(600);
    const blue = cop ? [...cop.post.values()].map((P: any) => [...P.pos]) : []; R.step(180);
    return { w1, w2, ended, said, halt, atBlow, blue, frame: [...d.frameCentre], res, chEnded: cop ? String(cop.ended) : gone ? String(gone.ended) : 'no charge', stale: R.men('all').filter((m: any) => doing(O.g, m).startsWith('setting the charge')).map((m: any) => m.callsign),
      stepped: D.stats.steppedBack - was, notes: D.log.slice(log0).map((l: any) => l.what).filter((w: string) => /made room|no room to stand off/.test(w)), end: snap(), orphan };
  };
  const row = (r: Run, l: Man[] = r.halt) => l.map(m => `${m.c} ${at(m.post)} ${f2(dist(m.post, r.frame))} m off the frame [${m.there}]`).join(', ');
  const blast = (r: Run) => r.res ? `knocked ${r.res.knocked}, stunned ${r.res.stunned}, backblast ${r.res.backblast}` : 'NOT BLOWN';
  const on = await play('halt');
  const K = DOORTOOLS.backblastM, rear = (m: Man) => m.order === 'stack' && m.door === 'manager' && m.there.startsWith('rear security · ');
  const staged = (r: Run) => r.w1 === null && r.w2 === null && r.ended === 'found shut' && !!r.res;
  verdict(staged(on), 'staged: blue takes the charge on the locked manager\'s door, red\'s \'open and clear\' on it ends \'found shut\', and the charge is set, armed and blown',
    `charge → ${on.w1}, open and clear → ${on.w2}, ended '${on.ended}', said "${on.said}" · ${blast(on)}`);
  verdict(on.halt.length === 2 && on.halt.every(rear) && on.stepped === 2 && on.notes.length === 1 && on.end.every(rear), `${BREAK ? '(HALTBACK_BREAK: no charge ordered) ' : ''}both of red read 'rear security · <door>' as the order ends, their order still 'stack' on that door, and still do after the blast`,
    `${row(on)} · stepped back ${on.stepped} · log: ${on.notes.join(' | ') || 'nothing'}`);
  verdict(on.halt.every(m => dist(m.post, on.frame) >= K) && on.atBlow.every(m => dist(m.pos, on.frame) >= K), `their marks, and where they stand as Sam blows it, are ${K} m (the backblast) or more from the frame`,
    `marks ${on.halt.map(m => f2(dist(m.post, on.frame))).join(' / ')} m · standing ${on.atBlow.map(m => f2(dist(m.pos, on.frame))).join(' / ')} m`);
  verdict(!!on.res && on.res.backblast === 0 && on.stale.length === 0 && on.chEnded === 'in', 'blown: nobody of ours inside the backblast, the charge order runs to its end, nobody is left reading \'setting the charge\'',
    `${blast(on)} · charge order ended '${on.chEnded}' · reading 'setting the charge': ${on.stale.join(', ') || 'nobody'}`);
  return done();
});

// ============================================================================================================================ pairs-lanes
export const pairsLanes = guarded(async () => {
  const { verdict, done } = family('pairs-lanes: nobody in the other pair\'s line of fire');
  // ---- a posted gun comes down while a man of the other pair crosses its line
  { const R = await rig(5, CUBICLES, { quiet: true }); const { O } = R; const red = R.men('red');   // (Sam out of the corridor: a muzzle that comes down here comes down for blue)
    O.give('move', O.targetAt(FLANK), 'red'); R.set('red', 25); O.give('hold', R.here(), 'red'); R.step(60); const arcs = red.map((m: any) => O.of(m)?.there).join(' / ');
    let crossed = 0, up = 0, lowSeen = 0;
    R.post = () => { for (const w of red) { const a = O.of(w); if (!a?.watch || !a.arrived) continue; const p = w.char.pos, yaw = Math.atan2(a.watch[0] - p[0], a.watch[2] - p[2]), fx = Math.sin(yaw), fz = Math.cos(yaw), len = dist(p, a.watch);
      const hit = R.men('blue').some((m: any) => { const sx = m.char.pos[0] - p[0], sz = m.char.pos[2] - p[2], al = sx * fx + sz * fz; return al > 0.3 && al < len - 0.1 && Math.abs(sx * fz - sz * fx) < 0.3; });
      if (hit) { crossed++; if (!a.low) up++; } if (a.low) lowSeen++; } };
    O.give('move', O.targetAt([34.5, 0, 11.0]), 'blue'); R.set('blue', 30); R.step(30); const east = R.men('blue').map((m: any) => at(m.char.pos)).join(' '), eastLow = red.map((m: any) => !!O.of(m)?.low);
    O.give('move', O.targetAt([17.5, 0, 11.0]), 'blue'); R.set('blue', 40); R.step(90); const after = red.map((m: any) => !!O.of(m)?.low);
    const end = R.men('blue').map((m: any) => at(m.char.pos)).join(' ');
    verdict(crossed > 10 && up <= 2, 'blue sent down the corridor past red\'s held arc: on every frame a blue man is in a red gun\'s line, that muzzle is down (two frames\' grace as he steps in)', `red ${arcs} · blue east to ${east} (red's muzzles down there: ${eastLow.join(' / ')}), then west to ${end} · man-frames in a red line ${crossed} · with the muzzle still up ${up} · frames a muzzle was down ${lowSeen}`);
    verdict(eastLow.some(Boolean) && after.every((l: boolean) => !l), 'a muzzle stays down while blue stands in its arc, and both are back up once blue is out of the lines', `down with blue stopped east ${eastLow.join(' / ')} · down at the end ${after.join(' / ')}`); }
  return done();
});
