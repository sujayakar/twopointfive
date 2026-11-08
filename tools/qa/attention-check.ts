// A GUARD HAS ONE ATTENTION, AND WHERE IT IS CAN BE SEEN — the check behind src/game/attention.ts (ATTENTION.on: on in the page, ?noattention; off on the frozen
// stand, TPS_ATTENTION=1).
//   bun run tools/qa/attention-check.ts        (also `probes.ts attention`; run it on both stands: TPS_PROFILE=shipped in front)
// The office at night, the shift on its own beats, the switch thrown by hand. A 'clack' below is the ring airfoil's own event (launcher.ts clack: a 'prop' of
// RING.clackNoise with nobody behind it) filed five metres ahead of the corridor man:
//   (a) TAKEN, FOR A TIME: one clack and the calm man goes to look at it — his attention is on the place he put it, whole while he walks, then exactly
//       `noiseSecs` at the thing; he says he is done `tellSecs` before the end, says it once, and is back on his route.;
//   (c) A GLIMPSE AT THE EDGE WRITES NO FIX, THEN BREAKS IT: that lit man standing there feeds his awareness while his fix stays the thing; `breakGain` later
//       the hold is gone, he says so, and his fix is the man.;
//   (d) A FOOTFALL: a walking step three metres behind him does not reach a held ear; a running one does, breaks the hold and
//       turns him to it;
//   (e) ANYTHING LOUD rouses him as it did: a slammed door ends the hold with no line of this file's;
//   (g) SHOWN: the mark is there from the taking to the end, its clock only ever runs down, it flags the last beat — and a man the fog of war hides has none;
//   (h) THE WHISTLE he answers holds him the same way for `whistleSecs`;
//   (i) LENT: another file's standing claim (a screen) holds a calm man for as long as it is renewed, with the lender's ears, and a real clack takes it over;
//   (j) a checkpoint carries a hold: continue ≡ load;
//   (k) the lines carry no compass word and no digit above 5;
// exit 0 = all hold · 1 = one does not
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');

function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

/** what the HUD is told of each held attention, a man the fog hides left out as it draws them; the man found by the id it names him by */
const attentionMarks = (g: any): any[] => g.view.attention.filter((l: any) => !l.hidden).map((l: any) => { const gd = g.guards.find((m: any) => m.char.id === l.man); return { ...l, gd, from: [gd.char.pos[0], 0.05, gd.char.pos[2]] }; });

export async function attentionCheck(): Promise<boolean> {
  const { standUp } = await import('./headless.ts');
  const { ATTENTION, ATTENTION_STATS, ATTENTION_LOG, attentionOf, attentionOffDeg, attentionLend, attentionLines } = await import(`${ROOT}/src/game/attention.ts`);
  const { sightGain, canSee } = await import(`${ROOT}/src/game/guards.ts`);
  const { RING } = await import(`${ROOT}/src/game/launcher.ts`);
  const { tryWhistle } = await import(`${ROOT}/src/game/whistle.ts`);
  const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
  const DT = 1 / 60, rnd0 = Math.random, was = { ...ATTENTION }; let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== attention: a guard has one attention, and where it is can be seen ===');
  const FAR: [number, number, number] = [5.1, 0, 15.2];
  /** the office, Sam parked dark where he spawns, two seconds of the shift's own walk; the switch as asked */
  const stand = async (seed: number, on = true) => { Math.random = seeded(seed); const W: any = await quiet(() => standUp()); const g = W.game; ATTENTION.on = on; g.godMode = true; g.playerInvisible = true;   // (Sam is nobody's sighting but where a line below stands him up for one)
    const step = (n = 1, vis = 0) => { for (let i = 0; i < n; i++) { g.player.visibility = vis; W.step(DT); } };
    step(120); const A = g.guards[0]; return { W, g, A, step }; };
  type S = Awaited<ReturnType<typeof stand>>;
  const ahead = (A: any, m: number, side = 0): [number, number, number] => { const p = A.char.pos, f = A.char.forward(); return [p[0] + f[0] * m + f[2] * side, 0, p[2] + f[2] * m - f[0] * side]; };
  const clack = (g: any, at: number[]) => g.events.push({ kind: 'prop', pos: [at[0], 0, at[2]], time: g.time, loud: false, level: RING.clackNoise, who: 0, lure: true });
  const said = (g: any, A: any) => (A.bubble && g.time - A.bubble.t < DT * 1.5 ? A.bubble.text as string : '');
  /** step until `test` or `secs` are up; the seconds it took, or -1 */
  const until = (s: S, secs: number, test: () => boolean, vis = 0, each?: () => void): number => { for (let f = 0; f < secs * 60; f++) { s.step(1, vis); each?.(); if (test()) return (f + 1) * DT; } return -1; };
  /** a clack ahead of him, and on to the frame he is AT it (the hold's clock running) */
  const toThing = (s: S, at: number[]) => { clack(s.g, at); return until(s, 15, () => { const H = attentionOf(s.g, s.A); return !!H && H.until >= 0; }); };
  const zero = () => { for (const k of Object.keys(ATTENTION_STATS)) (ATTENTION_STATS as any)[k] = 0; ATTENTION_LOG.length = 0; };
  const WARY = ['again?', '…that\'s twice.'], SPENT = ['no. somebody\'s doing that.', 'that\'s no accident. come out.'], BROKE = ['—wait.', 'what—'];

  try {
    // ---- (a) taken, for a time
    { const s = await stand(3), { g, A } = s; zero(); const at = ahead(A, 5);
      clack(g, at); s.step(1);
      const H0 = attentionOf(g, A), took = !!H0 && H0.mode === 'go' && H0.kind === 'noise' && Math.hypot(H0.at[0] - at[0], H0.at[2] - at[2]) < 0.6 && A.state === 'suspicious';
      const lines: string[] = []; let leftMax = -1, leftUp = false, lastLeft = 2, marked = 0, frames = 0, ending = 0, dwell = 0, walk = 0, toldAt = -1, endAt = -1, t = 0;
      until(s, 30, () => A.state === 'patrol', 0, () => { t += DT; const H = attentionOf(g, A), M = attentionMarks(g).find((m: any) => m.gd === A), w = said(g, A); if (w && lines[lines.length - 1] !== w) { lines.push(w); if (w === 'must have been nothing') toldAt = t; }
        if (H) { frames++; if (M) { marked++; if (M.left > lastLeft + 1e-9) leftUp = true; lastLeft = M.left; leftMax = Math.max(leftMax, M.left); if (M.ending) ending++; } if (H.until >= 0) dwell += DT; else walk += DT; endAt = t; } });
      const after = said(g, A), nothings = lines.filter(x => x === 'must have been nothing').length + (after === 'must have been nothing' ? 1 : 0);
      line(took, 'one clack and the calm man\'s attention is on the place he put it, and he goes', `${A.callsign}: ${A.state === 'patrol' ? 'back on patrol' : A.state}; taken ${ATTENTION_STATS.taken}, the place ${H0 ? `(${H0.at[0].toFixed(1)}, ${H0.at[2].toFixed(1)})` : '—'} for a clack at (${at[0].toFixed(1)}, ${at[2].toFixed(1)})`);
      line(Math.abs(dwell - ATTENTION.noiseSecs) < 0.06 && walk > 0.5, `whole while he walks, then exactly noiseSecs at the thing`, `walked ${walk.toFixed(2)} s, at it ${dwell.toFixed(2)} s (noiseSecs ${ATTENTION.noiseSecs})`);
      line(toldAt > 0 && Math.abs((endAt - toldAt) - ATTENTION.tellSecs) < 0.06 && nothings === 1 && ATTENTION_STATS.released === 1, 'he says he is done tellSecs before his eye widens, and says it once', `told ${(endAt - toldAt).toFixed(2)} s before the end (tellSecs ${ATTENTION.tellSecs}); said: ${lines.join(' / ')}; 'must have been nothing' × ${nothings}`);
      line(marked === frames && frames > 0 && !leftUp && leftMax > 0.999 && lastLeft < 0.02 && ending > 0 && Math.abs(ending * DT - ATTENTION.tellSecs) < 0.06, '(g) the mark stands from the taking to the end, its clock only runs down, and it flags the last beat', `${marked} of ${frames} frames marked, the clock ${leftMax.toFixed(2)} → ${lastLeft.toFixed(2)}, ${ending} frames flagged`);
      { const { attentionShown } = await import(`${ROOT}/src/ui/attentionviz.ts`), s2 = await stand(3), at2 = ahead(s2.A, 5); clack(s2.g, at2); s2.step(30); const L = s2.g.view.attention, shown = attentionShown(L, false).length, forced = L.map((l: any) => ({ ...l, hidden: true })), hidden = attentionShown(forced, false).length, bared = attentionShown(forced, true).length; line(shown === 1 && hidden === 0 && bared === 1, '(g) a man the fog of war hides has no mark (the view saying so: rust/game_start/tests/hud.rs), but on a viewer\'s own picture with the fog lifted', `${shown} shown, ${hidden} with every man hidden, ${bared} with the fog lifted`); }
      const u = await stand(3); zero(); const at4 = ahead(u.A, 5); u.g.events.push({ kind: 'prop', pos: at4, time: u.g.time, loud: false, level: RING.clackNoise, who: 0 }); let peakU = 0, susU = false; for (let i = 0; i < 300; i++) { u.step(1); peakU = Math.max(peakU, u.A.awareness); susU ||= u.A.state !== 'patrol'; }
      line(!susU && peakU < 0.3 && ATTENTION_STATS.taken === 0, 'a knock as faint that nobody made to be looked at (a chair a door swept) lures nobody: only the lure does', `awareness peaked at ${peakU.toFixed(2)}, ${ATTENTION_STATS.taken} taken`); }

    // ---- (c) a glimpse at the edge writes no fix, then breaks it
    { const run = async (on: boolean) => { const s = await stand(5, true), { g, A } = s; const at = ahead(A, 5); toThing(s, at); const H = attentionOf(g, A)!; const eye = A.char.pos, y = H.yaw ?? A.char.bodyYaw; g.playerInvisible = false; s.step(1, 0.9); let P: [number, number, number] = FAR;   // (both reach the game with a step, and canSee below asks the game: he is lit at the spot the search picks, as he is in the run)
        spot: for (const m of [4, 5, 3.5, 3, 6, 8]) for (const deg of [35, -35, 40, -40, 30, -30, 45, -45, 50, -50, 60, 70, 80, 90, 100, 110]) {   // (his eye is along his sights, which may not be along the thing: the far side of a bearing is asked too)
        const a = y + deg * Math.PI / 180; P = [eye[0] + Math.sin(a) * m, 0, eye[2] + Math.cos(a) * m]; g.teleportPlayer(P); s.step(1, 0.9); P = [g.player.char.pos[0], 0, g.player.char.pos[2]]; const v = canSee(g, A); if (v.visible && v.dist > 2.6 && (attentionOffDeg(g, A, P) ?? 0) > ATTENTION.focusDeg + 3) break spot; }
        const clear = canSee(g, A).visible, dist = canSee(g, A).dist, deg = attentionOffDeg(g, A, P) ?? 0; ATTENTION.on = on; zero();
        let noFix = 0, grew = false, aw0 = A.awareness, w = '', fixAt = -1, t = 0, broke = -1;
        const contact = until(s, 12, () => A.state === 'alert', 0.9, () => { t += DT; const h = attentionOf(g, A); if (h) { if (A.lastKnown && Math.hypot(A.lastKnown[0] - h.at[0], A.lastKnown[2] - h.at[2]) < 0.6) noFix++; grew ||= A.awareness > aw0 + 0.02; } else if (broke < 0) { broke = t; w = said(g, A); }
          if (fixAt < 0 && A.lastKnown && Math.hypot(A.lastKnown[0] - P[0], A.lastKnown[2] - P[2]) < 0.5) fixAt = t; });
        return { noFix, grew, w, fixAt, broke, contact, clear, dist, deg, P, log: ATTENTION_LOG.map((x: any) => x.what).join(',') }; };
      const on = await run(true);
      line(on.clear && on.dist > 2.6 && on.noFix > 3 && on.grew && on.log.includes('glimpse') && BROKE.includes(on.w) && on.fixAt >= on.broke - DT * 1.5, 'a lit man at the edge of a held eye feeds his awareness and writes no fix; breakGain later the hold breaks, he says so and turns to the man', `${on.noFix} frames with his fix still the thing, broke at ${on.broke.toFixed(2)} s saying '${on.w}', his fix the man from ${on.fixAt.toFixed(2)} s (${on.log}; a clear line to him: ${on.clear}, at ${on.P[0].toFixed(1)}, ${on.P[2].toFixed(1)})`); }

    // ---- (d) a footfall
    { const heard = async (on: boolean, level: number) => { const s = await stand(6, true), { g, A } = s; const at = ahead(A, 5); toThing(s, at); ATTENTION.on = on; zero(); const b = ahead(A, -3), n0 = A.witness.heard.small;
        g.playerInvisible = false; g.events.push({ kind: 'step', pos: b, time: g.time, loud: false, level }); s.step(2); g.playerInvisible = true;
        return { heard: A.witness.heard.small > n0, held: !!attentionOf(g, A), toIt: !!A.lastKnown && Math.hypot(A.lastKnown[0] - b[0], A.lastKnown[2] - b[2]) < 1.5, log: ATTENTION_LOG.map((x: any) => x.what).join(',') }; };
      const walkOn = await heard(true, 0.3), runOn = await heard(true, 0.6);
      line(!walkOn.heard && walkOn.held, 'a walking step three metres behind a held man does not reach him', `held: heard ${walkOn.heard}, the hold ${walkOn.held ? 'stands' : 'gone'}`);
      line(runOn.heard && !runOn.held && runOn.toIt && runOn.log.includes('step'), 'a running one does, breaks the hold and turns him to it', `heard ${runOn.heard}, hold ${runOn.held ? 'stands' : 'gone'} (${runOn.log}), his fix the step: ${runOn.toIt}`); }

    // ---- (e) anything loud
    { const s = await stand(7), { g, A } = s; const at = ahead(A, 5); toThing(s, at); zero(); const b = ahead(A, -3);
      g.events.push({ kind: 'door', pos: b, time: g.time, loud: true, level: 1, who: 1 }); s.step(2); const w = said(g, A);
      line(!attentionOf(g, A) && A.awareness >= ATTENTION.calmAw && ATTENTION_LOG.some((x: any) => x.what === 'roused') && !attentionLines().includes(w), 'a slammed door rouses him as it did: the hold is over, with no line of this file\'s', `awareness ${A.awareness.toFixed(2)}, ${ATTENTION_LOG.map((x: any) => x.what).join(',')}`); }

    // ---- (f2) one happening is one taking
    { const s = await stand(8), { g, A } = s; zero(); const says: string[] = [];
      for (let k = 0; k < 3; k++) { clack(g, ahead(A, 4 + k * 1.3)); for (let i = 0; i < 21; i++) { s.step(1); const w = said(g, A); if (w && says[says.length - 1] !== w) says.push(w); } }
      const H = attentionOf(g, A);
      line(ATTENTION_STATS.taken === 1 && ATTENTION_STATS.spent === 0 && A.state === 'suspicious' && !!H && H.nth === 1 && !says.some(w => WARY.includes(w) || SPENT.includes(w)), 'three knocks a third of a second apart, each further on (a canister bouncing), are one taking that follows it', `${ATTENTION_STATS.taken} taken, ${ATTENTION_STATS.spent} spent, ${A.state}; said: ${says.join(' / ')}`); }

    // ---- (h) the whistle
    { const s = await stand(9), { g, A } = s; zero(); g.teleportPlayer(ahead(A, 5)); g.playerInvisible = false; s.step(1); const blew = tryWhistle(g); s.step(2); const H = attentionOf(g, A);
      g.teleportPlayer(FAR); g.playerInvisible = true; let dwell = 0; until(s, 30, () => A.state === 'patrol', 0, () => { const h = attentionOf(g, A); if (h && h.until >= 0) dwell += DT; });
      line(blew && !!H && H.kind === 'whistle' && H.mode === 'go' && Math.abs(dwell - ATTENTION.whistleSecs) < 0.06, 'a whistle he answers holds him the same way, for whistleSecs', `${H ? `${H.kind}, ${H.mode}` : 'no hold'}, at it ${dwell.toFixed(2)} s; ${ATTENTION_STATS.looks} other head(s) turned`); }

    // ---- (i) lent
    { const s = await stand(10), { g, A } = s; zero(); const scr = ahead(A, 2, 1.5);
      for (let i = 0; i < 60; i++) { attentionLend(g, A, scr, { ears: 0.2, noises: 0.5 }); s.step(1); } const H = attentionOf(g, A), M = attentionMarks(g).find((m: any) => m.gd === A);
      const ok1 = !!H && H.mode === 'lent' && H.ears === 0.2 && H.noises === 0.5 && !!M && M.left === 1 && A.state === 'patrol';
      attentionLend(g, A, scr, { ears: 0.2, noises: 1 }); clack(g, ahead(A, 5)); s.step(1); const H2 = attentionOf(g, A);
      const t = await stand(10); for (let i = 0; i < 30; i++) { attentionLend(t.g, t.A, ahead(t.A, 2, 1.5)); t.step(1); } const had = !!attentionOf(t.g, t.A); t.step(Math.ceil(0.4 * 60)); const gone = !attentionOf(t.g, t.A);
      line(ok1 && !!H2 && H2.mode === 'go' && H2.kind === 'noise' && had && gone, 'another file\'s standing claim holds a calm man with the lender\'s ears, a real clack takes it over, and it ends when it is no longer renewed', `lent: ${H ? `${H.mode}, ears ${H.ears}, noises ${H.noises}` : '—'}; after a clack: ${H2 ? `${H2.mode} ${H2.kind}` : '—'}; unrenewed: ${gone ? 'gone' : 'still held'}`); }

    // ---- (j) a checkpoint carries a hold
    { const s = await stand(11), { g, A, W } = s; toThing(s, ahead(A, 5)); s.step(60); const snap = await quiet(() => W.snapshot('attention'));
      const read = () => { const A2 = g.guards[0], H = attentionOf(g, A2); return H ? `${H.mode} ${H.kind} ${(H.until - g.time).toFixed(3)} ${H.nth} ${A2.state}` : `none ${A2.state}`; };
      const at0 = read(); s.step(90); const cont = read(); await quiet(() => W.restore(snap)); const back = read(); s.step(90); const load = read();
      line(at0 === back && cont === load && at0.startsWith('go'), 'a checkpoint carries a hold: continue ≡ load', `at the save '${at0}', loaded '${back}'; 1.5 s on '${cont}' against '${load}'`); }

    // ---- (i2) a lend that is not renewed is over at once (not a quarter second later: he has got up from the screen)
    { const t = await stand(10); for (let i = 0; i < 30; i++) { attentionLend(t.g, t.A, ahead(t.A, 2, 1.5)); t.step(1); } const had = attentionOf(t.g, t.A)?.mode === 'lent';
      let frames = 0; while (frames < 30 && attentionOf(t.g, t.A)) { t.step(1); frames++; }
      line(had && frames <= 2 && attentionMarks(t.g).length === 0, 'a lend that is not renewed is over inside two frames, its mark with it', `${had ? 'lent' : 'NOT LENT'}; gone ${frames} frame(s) after the last call`); }

    // ---- (k) the lines
    { const L: string[] = attentionLines(); const compass = L.filter(t => /\b(north|south|east|west)/i.test(t)), digit = L.filter(t => /[6-9]/.test(t));
      line(L.length > 0 && !compass.length && !digit.length, 'no compass word and no digit above 5 in any line of it', `${L.length} lines`); }

  } finally { Object.assign(ATTENTION, was); Math.random = rnd0; }
  console.log(`=== attention: ${all ? 'ALL PASS' : 'FAIL'} ===`);
  return all;
}
if (import.meta.main) process.exit((await attentionCheck()) ? 0 : 1);
