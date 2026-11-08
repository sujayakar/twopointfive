// SHAKEN — the check behind src/game/morale.ts (MORALE.on: on in the page, ?nomorale; off on the frozen stand, TPS_MORALE=1). Standalone and through probes.ts:
//   bun run tools/qa/morale-check.ts          bun run tools/qa/probes.ts morale
// The compound's room clear, the guard hut's man 2.4 m behind his door (doorstun-check's staging: where the maintainer's replay had its man), the 40 mm's burst
// on the door's outside face as he40.ts asks it, the player off in the yard, wide of the door's line and too dim to be made out. Every lamp on the level is
// put out for 'dark' and left burning for 'lit'; the rest of the garrison stands across the yard. Headless, PASS / FAIL each, on sim state:
//   (1) alone in the dark: BROKEN. While the blast's own stagger has him he is down, and neither steps nor fires; then the call goes out on the net BEFORE he
//       has moved and before any round (he says it, then he does it); inside 3.5 s of the stagger's end he is at least a metre further from the hole and still
//       facing it, and at least three real rounds have left his gun — the shots themselves, measured off the sim's own ray, lie toward the hole
//   (2) a living mate two metres off in the same dark hut: both JUMPY, neither broken; both down inside a second; nobody scrabbles, no round into the dust;
//       the one nearer the blast shouts it (not on the net), the other says nothing of this file's — and a second blast a second later breaks neither
//   (4) THE BAIT, after (1) has settled into the hold: the first draws the rest of his magazine — the gun is empty by its own count — then the shout that he is
//       out, spoken, on the frame the magazine change starts, and not a round leaves his gun from that shout to the end of the change (shots counted); a thing
//       thrown through the hole, the second bait, draws the short burst; a third a word and no round
//   (5) the first bait is a man: the player seen a stride inside the door, off the hole's line — the burst's shots, measured the same way, lie toward the HOLE, not
//       toward him; and once the baits are spent the next sighting ends the hold for good
//   (6) the office's night staff are men too: one of them alone in a blacked-out office with a grenade six metres off goes 'alert' by the floor's own rules
//       (his CONTACT, the alarm), draws, is broken the same way, and a checkpoint taken in his hold loads him the same man on the same rung in the same act
//   (7) a restart between a blast and the next tick: the fresh floor is fed nothing
//   (8) the hostage rescue's cast is nobody of this file's: a blast beside the captor meets no man of it
//   (9) every line of the file fits a bubble, names nothing off the blocklist, no compass word, none twice
// Asked of the Rust game on its copy: where a round goes is read off the way his body faces on the frame each round leaves his gun (the copy has no ray to wrap).
// These are the FIRST SLICE's men: every case runs with MORALE.training 'none' (nobody green, nobody a leader — training-check.ts is the check of those).
// It raises what it needs itself (blasts take doors off, the destruction dial).
// MORALE_BREAK=1 = the alone and dark terms count for nothing (the rule broken, the switch left alone): (1), (4), (5) and (6) must fail.   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
type V3 = [number, number, number];
const DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

export async function moraleCheck(): Promise<boolean> {
  const { standUp } = (await import('./headless.ts')) as any;
  const M = await import(`${ROOT}/src/game/morale.ts`);
  const { MORALE, MORALE_STATS, MORALE_LOG, moraleOf, rungOf, dreadOf, moraleLines, inTheDark } = M;
  const { hasHim, OWN } = await import(`${ROOT}/src/game/owner.ts`);
  const { DOORBLAST } = await import(`${ROOT}/src/game/doorblast.ts`);
  const { FRAG, blast } = await import(`${ROOT}/src/game/frag.ts`);
  const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { HE40 } = await import(`${ROOT}/src/game/he40.ts`);
  const { doorPt } = await import(`${ROOT}/src/game/doortools.ts`);
  const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const D = await import(`${ROOT}/src/game/dialogue.ts`);
  const BREAK = process.env.MORALE_BREAK === '1'; let all = true;
  const was = { morale: JSON.parse(JSON.stringify(MORALE)), frag: JSON.parse(JSON.stringify(FRAG)), door: DOORBLAST.on, sw: DESTRUCTION.on, rnd: Math.random };
  const { toggleTarget } = await import(`${ROOT}/src/game/lighting.ts`);
  const line = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? "PASS" : "FAIL"} — ${label}\n    ${detail}`); };
  console.log(`\n=== morale: a blast shakes the men it does not hurt${BREAK ? ' (MORALE_BREAK: alone and dark count for nothing)' : ''} ===`);
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]), wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));

  /** the hut, its man behind his door, the rest across the yard: `dark` = the hut's own lamps put out, `mate` = a second man two metres from him */
  const stage = async (seed: number, dark: boolean, mate = false) => {
    Math.random = seeded(seed);
    const W: any = await quiet(() => standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'compound.json'), 'utf8'))) }));
    const g = W.game; DOORBLAST.on = true; DESTRUCTION.on = true; FRAG.doorStun = true; g.tune.chaos = 0.5; MORALE.training = 'none'; MORALE.nerve = 'normal';
    if (BREAK) { MORALE.alone = 0; MORALE.dark = 0; }
    startFromQuery(g, 'levelFile=compound.json&roomclear&squad4'); g.godMode = true;
    const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } };
    step(10);
    const d = g.doors.list.find((x: any) => x.def.name === 'hut_w'); d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.place();
    const mid = doorPt(d, d.def.width / 2, 0, 1, 0);
    const men = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').sort((a: any, b: any) => flat(a.char.pos, mid) - flat(b.char.pos, mid));
    const T = men[0], T2 = men[1];
    const inside: 1 | -1 = g.doors.side(d, T.char.pos), side = -inside as 1 | -1;
    { const s = doorPt(d, d.def.width / 2 + 9, 6, side, 0); g.teleportPlayer([s[0], 0, s[2]]); }   // (the player, and the team where it falls in behind him, well off the door's line: nobody of theirs looks in through the hole)
    const put = (gd: any, p: number[]) => { gd.char.pos = [p[0], 0, p[2]]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.lastPos = [p[0], 0, p[2]]; };
    const p = doorPt(d, d.def.width / 2, 2.4, inside, 0);
    men.forEach((o: any, i: number) => { if (o !== T) put(o, [4.8 + (i % 4) * 0.9, 0, 5.8 + Math.floor(i / 4) * 0.9]); });
    put(T, p); if (mate) { const q = doorPt(d, d.def.width / 2 + 1.6, 3.4, inside, 0); put(T2, q); }
    if (dark) for (const t of g.targets) if (t.on) toggleTarget(g, t);   // (every lamp: what the yard's and the next room's lamps send round lights a dark hut past the floor)
    step(40); if (dark) for (let i = 0; i < 300 && !(inTheDark(g, T) && moraleOf(T)?.dark); i++) step();   // (the place's light eases down over a couple of seconds, and he reads where he stands on his own beat)
    const hole = doorPt(d, d.def.width / 2, HE40.standoff + 0.03, side, 1.0) as V3;
    const shell = () => blast(g, [hole[0], hole[1] - FRAG.lift, hole[2]], { who: PLAYER_ID, name: '40 mm grenade' });
    /** a spent magazine lobbed in through the hole from the yard, by the player's hand */
    const toss = () => { const a = doorPt(d, d.def.width / 2, 1.2, side, 1.3), b = doorPt(d, d.def.width / 2, 0, side, 1.3), k = 4 / Math.max(1e-3, flat(a, b)); return g.items.spawn('mag', a, [(b[0] - a[0]) * k, 1.2, (b[2] - a[2]) * k], { who: PLAYER_ID, fill: 1 }); };
    return { W, g, d, T, T2, p, hole, inside, step, shell, toss };
  };
  type Rig = Awaited<ReturnType<typeof stage>>;
  /** step `secs`, noting every new bubble of `who`: [name, text, radio, sim seconds since the call began] */
  const listen = (R: Rig, who: any[], secs: number, each?: (t: number) => void) => {
    const said: { who: any; text: string; radio: boolean; t: number }[] = [], last = new Map<any, any>(who.map(h => [h, h.bubble?.t]));
    for (let i = 1; i <= Math.round(secs * 60); i++) { R.step(); for (const h of who) { const b = h.bubble; if (b && last.get(h) !== b.t) { last.set(h, b.t); said.push({ who: h, text: b.text, radio: !!b.radio, t: i * DT }); } } each?.(i * DT); }
    return said;
  };
  const fired = (gd: any) => gd.weapon.shotsFired;
  const facing = (gd: any, at: number[]) => Math.abs(wrap(Math.atan2(at[0] - gd.char.pos[0], at[2] - gd.char.pos[2]) - gd.char.bodyYaw));
  /** how far his body lies off `at`, once for every round that has left his gun since the last call: call it every frame */
  const rounds = (who: any, at: () => number[]) => { const out: number[] = []; let n = fired(who); return { out, frame() { for (; n < fired(who); n++) out.push(facing(who, at())); } }; };
  const mean = (xs: number[]) => xs.length ? xs.reduce((a, b) => a + b, 0) / xs.length : NaN;
  /** (1)'s run, kept for (4) and (5): the lone man in the dark, through the stagger and 3.5 s past it */
  const lone = async (seed: number) => {
    const R = await stage(seed, true), T = R.T, p0 = [...T.char.pos], d0 = flat(p0, R.hole), n0 = fired(T), dark0 = inTheDark(R.g, T), hits = rounds(T, () => R.hole); MORALE_LOG.length = 0;
    R.shell(); const stagger = T.staggerUntil - R.g.time;
    let downAt = NaN, stagMoved = 0, stagShots = 0, low = 0, frames = 0, movedAtCall = NaN, shotsAtCall = NaN, firstShot = NaN; const seenCall = { done: false };
    const said = listen(R, [T], stagger + 3.5, t => { hits.frame();
      if (Number.isNaN(downAt) && T.char.anim.crouch > 0.7) downAt = t;
      if (R.g.time < T.staggerUntil) { frames++; if (T.char.anim.crouch > 0.7) low++; stagMoved = Math.max(stagMoved, flat(T.char.pos, p0)); stagShots = fired(T) - n0; }
      if (Number.isNaN(firstShot) && fired(T) > n0) firstShot = t;
      if (!seenCall.done && T.bubble?.radio && moraleLines('call').includes(T.bubble.text)) { seenCall.done = true; movedAtCall = flat(T.char.pos, p0); shotsAtCall = fired(T) - n0; } });
    const call = said.find(s => s.radio && moraleLines('call').includes(s.text));
    return { R, T, p0, d0, n0, dark0, hits, stagger, said, call, downAt, stagMoved, stagShots, low, frames, movedAtCall, shotsAtCall, firstShot };
  };
  try {
    // (1) alone in the dark: broken
    const L = await lone(21);
    { const { R, T } = L, S = moraleOf(T), back = flat(T.char.pos, R.hole) - L.d0, real = fired(T) - L.n0, face = facing(T, R.hole), offs = L.hits.out;
      line(L.stagger > 2 && L.downAt <= 1 && L.low >= L.frames - 60 && L.stagMoved < 0.05 && L.stagShots === 0,
        '(1) the blast\'s own stagger is honoured: he is down through it, and neither steps nor fires until it is over',
        `stagger ${f2(L.stagger)} s · down at ${f2(L.downAt)} s, low for ${L.low} of its ${L.frames} frames · moved ${f2(L.stagMoved)} m · rounds ${L.stagShots}`);
      line(!!L.call && L.call.who === T && L.call.t >= L.stagger + MORALE.downSecs - 0.1 && L.call.t <= L.stagger + MORALE.downSecs + 0.3 && L.movedAtCall < 0.05 && L.shotsAtCall === 0 && L.firstShot > L.call.t + 0.5,
        '(1) he says it, then he does it: the call goes out on the net before he has moved a hand\'s breadth and before any round',
        L.call ? `"${L.call.text}" (radio) at ${f2(L.call.t)} s · moved ${f2(L.movedAtCall)} m and ${L.shotsAtCall} rounds by then · first round at ${f2(L.firstShot)} s` : `no call · said: ${L.said.map(s => `"${s.text}"`).join(' ') || 'nothing'}`);
      line(T.state !== 'dead' && L.dark0 && rungOf(T) === 2 && T.char.anim.crouch > 0.7 && back >= 1 && face < 0.5 && real >= 3 && offs.length === real && mean(offs) < 0.2 && Math.max(...offs) < 0.45,
        '(1) alone in the dark, the 40 mm on his door: broken — down, a metre and more back off the hole and facing it, real rounds into the dust at the hole, inside 3.5 s of the stagger\'s end',
        `door off ${R.d.off} · dark before ${L.dark0} · ${S ? `dread ${f2(dreadOf(T))} rung ${S.rung} act ${S.act} alone ${S.alone} dark ${S.dark}` : 'NEVER MET'} · state ${T.state} · crouch ${f2(T.char.anim.crouch)} · ${f2(back)} m further off · ${f2(face)} rad off the hole · ${real} rounds left his gun · ${offs.length} shots measured, his body on average ${f2(mean(offs))} rad off the hole's bearing (worst ${f2(Math.max(0, ...offs))})`); }
    // (2) a mate at his shoulder: both jumpy
    { const R = await stage(22, true, true), T = R.T, T2 = R.T2, at0 = [T, T2].map(x => [...x.char.pos]), n0 = [fired(T), fired(T2)], sh0 = MORALE_STATS.shouts; MORALE_LOG.length = 0;
      const near = flat(T.char.pos, R.hole) <= flat(T2.char.pos, R.hole) ? T : T2, apart = flat(T.char.pos, T2.char.pos);
      R.shell(); const said = listen(R, [T, T2], 1); const down1 = [T, T2].map(x => x.char.anim.crouch);
      const rungs = [rungOf(T), rungOf(T2)], acts = [moraleOf(T)?.act, moraleOf(T2)?.act];
      const duck = said.filter(s => moraleLines('duck').includes(s.text)), shouts = MORALE_STATS.shouts - sh0;
      // …and a second burst a second on, too small to hurt anybody (frag.ts blast with its radii drawn in): shock is capped, and nobody with a mate breaks
      blast(R.g, [R.hole[0], R.hole[1] - FRAG.lift, R.hole[2]], { who: PLAYER_ID, name: 'frag grenade', lethalM: 0.05, woundM: 0.1, fx: false });
      const said2 = listen(R, [T, T2], 5); const moved = [T, T2].map((x, i) => flat(x.char.pos, at0[i])), shock = [moraleOf(T)?.shock ?? NaN, moraleOf(T2)?.shock ?? NaN], rungs2 = [rungOf(T), rungOf(T2)];
      const calls = [...said, ...said2].filter(s => [...moraleLines('call'), ...moraleLines('again')].includes(s.text));
      line(T.state !== 'dead' && T2.state !== 'dead' && rungs[0] === 1 && rungs[1] === 1 && down1.every(c => c > 0.7) && acts.every(a => a === 'duck') && moved.every(m => m < 0.3) && fired(T) === n0[0] && fired(T2) === n0[1] && MORALE_LOG.length === 0,
        '(2) the same blast, a living mate two metres off in the dark: both jumpy and down inside a second — nobody broken, nobody scrabbling, no round into the dust',
        `${apart.toFixed(1)} m apart · rungs ${rungs.join(' / ')} · crouch at 1 s ${down1.map(f2).join(' / ')} · acts ${acts.join(' / ')} · moved ${moved.map(f2).join(' / ')} m · rounds ${fired(T) - n0[0]} / ${fired(T2) - n0[1]}`);
      line(duck.length === 1 && duck[0].who === near && !duck[0].radio && duck[0].t <= 0.1 + MORALE.shoutGap && shouts === 1,   // (at once — or `shoutGap` after the floor's own CONTACT, where that was in his mouth)
        '(2) the nearer of the two shouts it as they go down — spoken, not on the net — and the other says nothing of this file\'s',
        duck.length ? duck.map(s => `${s.who.callsign}: "${s.text}"${s.radio ? ' (RADIO)' : ''} at ${f2(s.t)} s`).join(' · ') + ` · nearer: ${near.callsign}` : `no duck line · said: ${said.map(s => `"${s.text}"`).join(' ') || 'nothing'}`);
      line(rungs2[0] === 1 && rungs2[1] === 1 && shock.every(x => x <= MORALE.shockCap + 1e-9 && x > MORALE.blast) && !calls.length && fired(T) === n0[0] && fired(T2) === n0[1],
        '(2) a second blast a second later: shock is capped and neither man of a pair breaks — nobody calls the net with a mate at his shoulder',
        `rungs ${rungs2.join(' / ')} · shock ${shock.map(f2).join(' / ')} (cap ${MORALE.shockCap}) · calls ${calls.length} · rounds ${fired(T) - n0[0]} / ${fired(T2) - n0[1]}`); }
    // (4) (5) the bait, on (1)'s man: first with a man in it, seen a stride inside the door off the hole's line; then two things thrown through the hole; then the sighting that ends it
    { const { R, T } = L, g = R.g; for (let i = 0; i < 150 && !(moraleOf(T)?.act === 'hold' && T.char.anim.crouch > 0.7 && facing(T, R.hole) < 0.5); i++) R.step(); const S = () => moraleOf(T), act0 = S()?.act, onHole = T.char.anim.crouch > 0.7 && facing(T, R.hole) < 0.5, mag0 = T.weapon.roundsReady, n0 = fired(T);
      const him = doorPt(R.d, R.d.def.width / 2 + 1.5, 0.8, R.inside, 0), away = [...g.player.char.pos]; g.teleportPlayer([him[0], 0, him[2]]);   // (he leaves as the change starts: a team that followed him in would end the hold)
      const dHole = flat(him, R.hole), hits = rounds(T, () => R.hole), hitsHim = rounds(T, () => him), said: { text: string; radio: boolean; t: number }[] = []; let heard = T.bubble?.t, seenAt = NaN, reloadAt = NaN, magAtOut = NaN, shotsAtOut = NaN, endAt = NaN, n = n0;
      for (let i = 1; i <= 720 && (Number.isNaN(endAt) || S()?.act !== 'hold'); i++) { const t = i * DT; g.player.visibility = Number.isNaN(reloadAt) ? 1 : 0.05; R.W.step(DT); hits.frame(); hitsHim.frame();
        const b = T.bubble; if (b && b.t !== heard) { heard = b.t; said.push({ text: b.text, radio: !!b.radio, t }); }
        if (Number.isNaN(seenAt) && S()?.act === 'bait') seenAt = t;
        if (Number.isNaN(reloadAt) && T.reloadT >= 0) { reloadAt = t; magAtOut = T.weapon.roundsReady; shotsAtOut = fired(T); g.teleportPlayer(away); }
        if (!Number.isNaN(reloadAt) && Number.isNaN(endAt) && T.reloadT < 0) { endAt = t; n = fired(T); } }
      const out = said.find(s => moraleLines('out').includes(s.text)), burst = shotsAtOut - n0, inWindow = n - shotsAtOut, toHole = hits.out.slice(0, burst), toHim = hitsHim.out.slice(0, burst), apart = Math.abs(wrap(Math.atan2(him[0] - T.char.muzzle[0], him[2] - T.char.muzzle[2]) - Math.atan2(R.hole[0] - T.char.muzzle[0], R.hole[2] - T.char.muzzle[2])));
      line(act0 === 'hold' && onHole && mag0 >= 12 && burst === mag0 && magAtOut === 0 && !Number.isNaN(endAt) && inWindow === 0 && fired(T) === n,
        '(4) the bait: the rest of his magazine at the hole — the gun empty by its own count — then not a round leaves it until the change is over',
        `before: ${act0}, ${onHole ? 'low on the hole' : 'NOT low on the hole'}, ${mag0} in the gun · burst ${burst} · in the gun at the shout ${magAtOut} · change ${f2(reloadAt)} → ${f2(endAt)} s (${f2(endAt - reloadAt)} s) · rounds fired in it ${inWindow}`);
      line(!!out && !out.radio && Math.abs(out.t - reloadAt) <= DT + 1e-6,
        '(4) he shouts that he is out on the frame the change starts — spoken, so the window is something the player hears', out ? `"${out.text}" at ${f2(out.t)} s (change at ${f2(reloadAt)} s)` : `no such line · said: ${said.map(s => `"${s.text}"`).join(' ') || 'nothing'}`);
      line(dHole <= MORALE.baitM && !Number.isNaN(seenAt) && burst >= 12 && toHole.length === burst && apart > 0.25 && mean(toHole) < 0.2 && mean(toHim) > apart / 2,
        '(5) a man standing a stride inside the door, off the hole\'s line, seen: the long burst goes at the HOLE, not at him',
        `he stands ${f2(dHole)} m from the hole, ${f2(apart)} rad off its bearing · bait at ${f2(seenAt)} s · ${burst} rounds · ${toHole.length} shots measured, his body on average ${f2(mean(toHole))} rad off the hole, ${f2(mean(toHim))} rad off the man`);
      R.step(10); const a2 = S()?.act, n2 = fired(T); R.toss(); R.step(180); const short = fired(T) - n2;
      const a3 = S()?.act, n3 = fired(T); R.toss(); const said3 = listen(R, [T], 3); const none = fired(T) - n3, spent = said3.find(s => moraleLines('spent').includes(s.text));
      line(a2 === 'hold' && short === MORALE.baitShort && a3 === 'hold' && none === 0 && !!spent && !spent.radio && T.state !== 'dead',
        '(4) the second bait, a thing thrown through the hole, draws the short burst, the third a word and no round', `second: ${short} of ${MORALE.baitShort} (from ${a2}) · third: ${none} rounds (from ${a3})${spent ? `, "${spent.text}"` : ', NO WORD'} · baits ${S()?.baits}`);
      const afterChange = S()?.act; g.teleportPlayer([him[0], 0, him[2]]); for (let i = 0; i < 90; i++) { g.player.visibility = 1; R.W.step(DT); } const afterSight = S()?.act ?? 'none', owned = hasHim(T, OWN.morale);
      for (let i = 0; i < 120; i++) { g.player.visibility = 0.05; R.W.step(DT); } const later = S()?.act ?? 'none';   // (nobody's man any more — or, with a team at the player's back, a dead one: either way not this file's)
      line(afterChange === 'hold' && afterSight === 'none' && !owned && later === 'none',
        '(5) once the change is over, the next sighting ends the hold for good: he is the fight\'s man, and does not turn back to the hole',
        `after the change: ${afterChange} · with the man in his eyes: ${afterSight}, owned ${owned} · two seconds after he is lost again: ${later} (${T.state})`); }
    // (6) the night staff, and a checkpoint in the hold
    { Math.random = seeded(41); const W: any = await quiet(() => standUp()), g = W.game; MORALE.training = 'none'; MORALE.nerve = 'normal'; if (BREAK) { MORALE.alone = 0; MORALE.dark = 0; }
      g.playerInvisible = true; g.godMode = true; const step = (n = 1) => { for (let i = 0; i < n; i++) W.step(DT); }; step(10);
      const staff = g.guards.filter((x: any) => !x.unit && x.team === 'guard'), T = staff[0], name = T.callsign;
      for (const o of staff) if (o !== T) { o.hold = true; o.pinned = true; }   // (the rest of the shift held on their marks: nobody comes to the bang with a torch)
      for (const t of g.targets) if (t.on && t.kind !== 'breaker') toggleTarget(g, t);
      step(60); for (let i = 0; i < 120 && !inTheDark(g, T); i++) step(); const p = [...T.char.pos], f = T.char.forward(), at: V3 = [p[0] + f[0] * 6.5, 0, p[2] + f[2] * 6.5];
      const clear = !g.col.segmentBlocked([at[0], 0.6, at[2]], [p[0], 1.2, p[2]]), dark0 = inTheDark(g, T), alone0 = !!moraleOf(T)?.alone, n0 = fired(T), state0 = T.state;
      blast(g, at, { who: PLAYER_ID, name: 'frag grenade', fx: false });
      let alertAt = NaN, firstShot = NaN, calm = 0, undrawn = 0, contact = false;
      for (let i = 1; i <= 480; i++) { step(); if (Number.isNaN(alertAt) && T.state === 'alert') alertAt = i * DT; if (T.bubble?.text === 'CONTACT!') contact = true;
        if (fired(T) > n0) { if (Number.isNaN(firstShot)) firstShot = i * DT; if (T.state !== 'alert') calm++; if (!T.drawn) undrawn++; } }
      line(state0 === 'patrol' && alertAt < 0.5 && contact && g.alarm.episode && fired(T) - n0 >= 3 && calm === 0 && undrawn === 0,
        '(6) a man of the night staff: he goes \'alert\' by the floor\'s own rules — his CONTACT, the alarm episode — and has his gun out before a round leaves it',
        `${name} · ${state0} → alert at ${f2(alertAt)} s · CONTACT said ${contact} · alarm episode ${g.alarm.episode} · first round at ${f2(firstShot)} s · frames firing while calm ${calm}, undrawn ${undrawn}`);
      const S0 = moraleOf(T), was5 = S0 ? { rung: S0.rung, act: S0.act, shock: S0.shock, baits: S0.baits, calls: S0.calls } : null, dust = fired(T) - n0;
      const snap = W.snapshot('morale', true); step(240);
      Math.random = seeded(5); W.restore(snap); const m2 = g.guards.find((x: any) => x.callsign === name), S1 = moraleOf(m2);
      const same = !!was5 && !!S1 && S1.rung === was5.rung && S1.act === was5.act && Math.abs(S1.shock - was5.shock) < 1e-9 && S1.baits === was5.baits && S1.calls === was5.calls;
      step(30); const low = m2.char.anim.crouch > 0.7 && hasHim(m2, OWN.morale);
      line(T.state !== 'dead' && clear && same && was5!.rung === 2 && was5!.act === 'hold' && dust >= 3 && low,
        '(6) alone in a blacked-out office, a grenade six metres off: broken too — and a checkpoint taken in his hold loads him the same man, low on the hole',
        `${name} · line to the blast ${clear} · dark ${dark0} alone ${alone0} · saved rung ${was5?.rung} ${was5?.act} shock ${f2(was5?.shock ?? NaN)} calls ${was5?.calls} · rounds into the dust ${dust} · loaded rung ${S1?.rung} ${S1?.act} shock ${f2(S1?.shock ?? NaN)} · low and held after the load ${low}`); }
    // (7) a restart between a blast and the next tick
    { const R = await stage(24, true), T = R.T, fed0 = MORALE_STATS.fed, filed0 = MORALE_STATS.blasts; R.shell(); const filed = MORALE_STATS.blasts - filed0; R.g.clearAftermath(); R.step(90);
      line(filed === 1 && MORALE_STATS.fed === fed0 && (moraleOf(T)?.shock ?? 0) === 0 && (moraleOf(T)?.act ?? 'none') === 'none' && !hasHim(T, OWN.morale),
        '(7) a restart between a blast and the next tick: the blast filed is dropped, and the floor after it is fed nothing', `filed ${filed} · fed ${MORALE_STATS.fed - fed0} · his shock ${f2(moraleOf(T)?.shock ?? 0)} · act ${moraleOf(T)?.act ?? 'none'}`); }
    // (8) the hostage rescue's cast
    { Math.random = seeded(42); const W: any = await quiet(() => standUp()), g = W.game, Rs = g.rescue; g.godMode = true; const why = Rs.start(); MORALE.training = 'none'; MORALE.nerve = 'normal';
      for (let i = 0; i < 30; i++) W.step(DT); const cap = Rs.captor(), fed0 = MORALE_STATS.fed, filed0 = MORALE_STATS.blasts, cast: any[] = Rs.hostiles;
      const at: V3 = cap ? [cap.char.pos[0] + 1.2, 0, cap.char.pos[2]] : [0, 0, 0];
      if (cap) blast(g, at, { who: PLAYER_ID, name: 'frag grenade', lethalM: 0.05, woundM: 0.1, fx: false });
      let owned = 0; for (let i = 0; i < 120; i++) { W.step(DT); if (cast.some(x => hasHim(x, OWN.morale))) owned++; }
      line(!!cap && Rs.active && MORALE_STATS.blasts - filed0 === 1 && MORALE_STATS.fed === fed0 && cast.every(x => !moraleOf(x)) && owned === 0,
        '(8) the hostage rescue: a blast a stride from the captor meets no man of its cast — his clock and his gun are the rescue\'s alone',
        cap ? `captor ${cap.callsign} · cast ${cast.length} · filed ${MORALE_STATS.blasts - filed0} · fed ${MORALE_STATS.fed - fed0} · men of the cast met ${cast.filter(x => moraleOf(x)).length} · frames owned ${owned}` : `no captor (${why ?? 'started'})`); }
    // (9) the words
    { const lines: string[] = moraleLines(), long = lines.filter(t => t.length > D.AIM_CHARS), blocked = lines.filter(t => D.blockedPhrase(t)), compass = lines.filter(t => /\b(north|south|east|west)/i.test(t)), twice = lines.filter((t, i) => lines.indexOf(t) !== i), tight = lines.filter(t => /\S—\s|\s—\S/.test(t));
      line(lines.length >= 12 && !long.length && !blocked.length && !compass.length && !twice.length && !tight.length && (['duck', 'call', 'again', 'out', 'spent'] as const).every(k => moraleLines(k).length >= 3),
        `(9) every line fits a bubble (≤ ${D.AIM_CHARS}), none on the originality blocklist, no compass word, none twice`, `${lines.length} lines · too long: ${long.join(' | ') || 'none'} · blocked: ${blocked.join(' | ') || 'none'} · twice: ${twice.join(' | ') || 'none'}`); }
  } catch (e) { all = false; console.log(`- FAIL — the check threw\n    ${e instanceof Error ? e.stack : String(e)}`); }
  finally { Object.assign(MORALE, was.morale); Object.assign(FRAG, was.frag); DOORBLAST.on = was.door; DESTRUCTION.on = was.sw; Math.random = was.rnd; }
  console.log(`=== morale: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await moraleCheck()) ? 0 : 1);
