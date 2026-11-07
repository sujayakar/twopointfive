// DOZING IN THE DARK — the check behind src/game/doze.ts (DOZE.on, settled on, with its halves `screen` and `lend` and the attention rule: the check asks the
// page's side alone and writes no switch. Until 2026-11-07 it also asked the same man awake with the switch off as the control of (b) and (b2), (i) the
// timetable against the switch-off run, (n2)'s sight and hearing rates against the attention rule off and `lend` off, (r) the half-switch DOZE.screen off
// and (l) the switch off = the base: those went with the off side).
//   bun run tools/qa/doze-check.ts        (also `probes.ts doze`)
// The police post's quiet mission (its hand-set deal), the shift on its own beats, Sam out of everybody's sight unless a line says so:
//   (a) a man with a long wait in the dark bunk room sits in the chair there and sleeps: seated on the chair, his place dark, his torch out, the 'z' over him;
//   (b) asleep his eyes are shut: Sam stood lit in front of him for three seconds is not seen;
//   (c) his ears are dull, not gone: crouched steps beside the chair and a walking step 2.8 m off do not wake him; a walking step beside the chair and a
//       running step 3.4 m off do;
//   (d) woken by a noise he is suspicious on the frame, placed on the noise, rooted while he comes round, out of the chair — and awake for the forty seconds after;
//       a whistle beside him wakes him;
//   (e) light on his face wakes him; (f) his chair shoved wakes him; (g) the floor going up has him on his feet;
//   (h) from behind, the close-in mode offers the take over the chair;
//   AT A SCREEN (the police post's terminal, built off, on the bunk room's footlocker):
//   (m) a man with a short wait in the dark pulls the chair up, sits to it and switches it on: facing it, its light on his face, torch out, awake, '♪' over him;
//   (n) headphones on: a walking step near him counts for less (or nothing) and he stays; running steps beside him have him up, the screen left on;
//   (n2) his attention is the screen's (a lent hold at the screen that the floor shows) and the headphones count once;
//   (o) the screen going dark under his eyes has him up and looking at it;
// DOZE_BREAK=1 = the level is stood without its lived-in furniture (no chair in the bunk room): nobody sits, and (a) must fail.   exit 0 = all hold · 1 = one does not
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
const ROOT = resolve(import.meta.dir, '../..');

function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

export async function dozeCheck(): Promise<boolean> {
  const { standUp } = await import('./headless.ts');
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { DOZE, DOZE_STATS, DOZE_LOG, dozeOf, dozeMark, dozeAsleep, dozeWoke, dozeAt, dozeLines } = await import(`${ROOT}/src/game/doze.ts`);
  const { CLOSE, closeAt, closeKindOf, closeSeated } = await import(`${ROOT}/src/game/close.ts`);
  const { workLit } = await import(`${ROOT}/src/game/calmtorch.ts`);
  const { raiseEscalation, lightEstimateAt } = await import(`${ROOT}/src/game/guards.ts`);
  const { ROW_BIT } = await import(`${ROOT}/src/sim/lightshome.ts`);
  const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
  const { attentionOf, attentionSideK, attentionMarks } = await import(`${ROOT}/src/game/attention.ts`);
  const DT = 1 / 60, rnd0 = Math.random, was = { ears: DOZE.ears }, BREAK = process.env.DOZE_BREAK === '1'; let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== doze: a calm man with a long wait in the dark sits in the chair there and sleeps; what wakes him ===');
  const text = readFileSync(resolve(ROOT, 'levels/police-post.json'), 'utf8');
  /** the police post's quiet mission, its hand-set deal (start 0 / spot 0 / patrol set 0), Sam unseen and unhurt — with the level's lived-in furniture on either stand
   *  (the bunk room's chair is part of it: the frozen stand leaves that furniture out) */
  const stand = async (seed: number) => { Math.random = seeded(seed);
    const W: any = await quiet(() => standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(text), { lived: !BREAK }) })); const g = W.game;
    await quiet(() => { let pin = 0; for (; pin < 400; pin++) { const r = startFromQuery(g, `?quiet=${pin}`); if (r.why) throw new Error(`doze-check: the quiet mission would not start — ${r.why}`); const d = g.quiet.deal; if (d.start === 0 && d.spot === 0 && Math.max(0, d.patrol) === 0) break; } });
    g.playerInvisible = true; g.godMode = true; Math.random = seeded(seed + 1000); return W; };
  const staff = (g: any) => g.guards.filter((x: any) => !x.unit && x.team === 'guard');
  const flat = (a: any, b: any) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const chairOf = (g: any, gd: any) => g.props.props.filter((p: any) => p.def.kind === 'chair').sort((a: any, b: any) => Math.hypot(a.x - gd.char.pos[0], a.z - gd.char.pos[2]) - Math.hypot(b.x - gd.char.pos[0], b.z - gd.char.pos[2]))[0];
  /** step until a man with at least `left` s of his wait to go is asleep (the long wait: Reyes's forty-eight seconds in the bunk room) */
  const untilAsleep = (W: any, left = 30, cap = 120): any => { const g = W.game; for (let i = 0; i < cap * 60; i++) { W.step(DT); const m = staff(g).find((x: any) => dozeAsleep(x) && x.wait > left); if (m) return m; } return null; };
  /** a point `d` m from him with clear air and free floor between */
  const clearPoint = (g: any, gd: any, d: number): any => { const p = gd.char.pos; for (let k = 0; k < 72; k++) { const a = k * Math.PI / 36, q = [p[0] + Math.sin(a) * d, 0, p[2] + Math.cos(a) * d];
    if (!g.col.nav.isBlocked(q[0], q[2]) && !g.col.segmentBlocked([p[0], 1.2, p[2]], [q[0], 1.2, q[2]]) && !g.col.segmentBlocked([p[0], 0.4, p[2]], [q[0], 0.4, q[2]])) return q; } return null; };
  /** one of Sam's footfalls at a point, at a gait's level (player.ts: crouched 0.125, walking 0.26, running 0.6 — × the floor's) — heard for `frames` frames with Sam
   *  himself a man whose noise counts (the sandbox's invisible Sam makes none), then unseen again */
  const step = (W: any, at: any, level: number, frames: number) => { const g = W.game; g.playerInvisible = false; g.events.push({ kind: 'step', pos: [at[0], 0, at[2]], time: g.time, loud: false, level }); for (let i = 0; i < frames; i++) W.step(DT); g.playerInvisible = true; };
  /** a point in front of him, inside his cone, with clear air from his eyes to a standing man's chest and floor to stand on — where the game then really puts Sam */
  const frontPoint = (g: any, gd: any): any => { const p = gd.char.pos; for (const d of [2.2, 1.8, 2.6, 1.5]) for (const a of [0, 0.26, -0.26, 0.52, -0.52, 0.78, -0.78]) { const y = gd.char.bodyYaw + a, q = [p[0] + Math.sin(y) * d, 0, p[2] + Math.cos(y) * d];
    if (g.col.nav.isBlocked(q[0], q[2]) || g.col.segmentBlocked([p[0], 1.15, p[2]], [q[0], 1.1, q[2]]) || g.col.segmentBlocked([p[0], 1.15, p[2]], [q[0], 0.5, q[2]])) continue;
    g.teleportPlayer(q); if (flat(g.player.char.pos, q) < 0.35) return q; } return null; };
  const same = (g: any, name: string) => staff(g).find((x: any) => x.callsign === name);

  try {
    // (a) he sits and sleeps
    const W = await stand(21), g = W.game; const man = untilAsleep(W);
    if (!man) { line(false, 'a man with a long wait in the dark sits in the chair there and sleeps', 'nobody slept inside two minutes'); return false; }
    const name = man.callsign, ch = chairOf(g, man), sleptAt = g.time;
    { const c = man.char, hips = [c.pos[0] - Math.sin(c.bodyYaw) * 0.33, 0, c.pos[2] - Math.cos(c.bodyYaw) * 0.33];
      for (let i = 0; i < 90; i++) W.step(DT);   // (the slump eases in)
      const onChair = flat(hips, [ch.x, 0, ch.z]), dark = !workLit(g, c.pos[0], c.pos[2]), torch = !!man.flashlight.enabled, clip = c.anim.act?.clip ?? '-';
      line(onChair < 0.2 && dark && !torch && closeSeated(g, man) && clip === 'Sitting_Idle_Loop' && c.anim.slump === 1 && dozeMark(g, man) !== '' && man.state === 'patrol',
        'a man with a long wait in the dark sits in the chair there and sleeps: on the chair, his place dark, his torch out, the mark over him',
        `${name} asleep ${sleptAt.toFixed(1)} s in, ${man.wait.toFixed(0)} s of his ${man.route.wait[man.wp]} s wait to go · hips ${onChair.toFixed(2)} m off the chair · place ${dark ? 'dark' : 'LIT'} · torch ${torch ? 'BURNING' : 'out'} · clip ${clip} · mark '${dozeMark(g, man)}'`); }
    const recs = () => JSON.stringify(staff(g).map((x: any) => { const r = dozeOf(x); return r ? [x.callsign, r.phase, r.wp, r.why, +r.secs.toFixed(3)] : [x.callsign]; }));
    const snap = W.snapshot('doze', true), recs0 = recs();
    const back = () => { Math.random = seeded(77); W.restore(snap); const m = same(g, name); for (let i = 0; i < 60; i++) W.step(DT); return m; };   // (a second for the clip to have him down again)

    // (b) eyes shut
    { const m = back(), c = m.char; for (const o of staff(g)) if (o !== m) o.dazzledUntil = g.time + 10;   // (the others see nothing for the look: the line is about HIS eyes — a mate awake in the same room would see Sam and raise the floor)
      g.playerInvisible = false; const front = frontPoint(g, m); g.player.crouch = true; let peak = 0;
      for (let i = 0; i < 180; i++) { g.player.visibility = 1; W.step(DT); peak = Math.max(peak, m.awareness); }
      const still = dozeAsleep(m); g.playerInvisible = true;
      line(!!front && still && peak < 0.05, 'asleep his eyes are shut: Sam stood lit two metres in front of him for three seconds is not seen', `${front ? '' : 'NO FLOOR IN FRONT OF HIM · '}awareness peaked ${peak.toFixed(2)}, still asleep ${still}`); }

    // (c) dull ears
    { let m = back(); const near = clearPoint(g, m, 1.0), far = clearPoint(g, m, 3.4), mid = clearPoint(g, m, 2.8);
      if (!near || !far || !mid) line(false, 'his ears are dull, not gone', 'no clear air round the chair to make the noises in');
      else {
        for (let i = 0; i < 12; i++) step(W, near, 0.125, 20); const crouch = dozeAsleep(m);
        step(W, mid, 0.26, 5); const walkFar = dozeAsleep(m);
        step(W, near, 0.26, 3); const walkNear = !dozeAsleep(m);
        m = back(); step(W, far, 0.6, 3); const run = !dozeAsleep(m);
        line(crouch && walkFar && walkNear && run, 'his ears are dull, not gone: crouched steps beside the chair and a walking step across the room leave him asleep; a walking step beside it and a run across the room wake him',
          `twelve crouched steps at 1 m: ${crouch ? 'asleep' : 'WOKE'} · a walking step at 2.8 m: ${walkFar ? 'asleep' : 'WOKE'} · a walking step at 1 m: ${walkNear ? 'woke' : 'SLEPT ON'} · a running step at 3.4 m: ${run ? 'woke' : 'SLEPT ON'}`); } }

    // (d) woken by a noise
    { const m = back(), at = clearPoint(g, m, 1.0), p0 = [m.char.pos[0], 0, m.char.pos[2]]; step(W, at, 0.26, 2);
      const sus = m.state === 'suspicious', placed = !!m.lastKnown && flat(m.lastKnown, at) < 0.6, seated = closeSeated(g, m), why = dozeWoke(g, m);
      let moved = 0; for (let i = 0; i < Math.floor((DOZE.stirSecs - 0.15) / DT); i++) { W.step(DT); moved = Math.max(moved, flat(m.char.pos, p0)); }
      let again = false; for (let i = 0; i < 40 * 60 && !again; i++) { W.step(DT); if (dozeAsleep(m)) again = true; }
      line(sus && placed && !seated && why === 'noise' && moved < 0.45 && !again, 'woken by a noise: suspicious on the frame, placed on the noise, out of the chair, rooted while he comes round — and awake for the forty seconds after',
        `two frames on: ${m.state === 'patrol' && !sus ? 'still calm' : sus ? 'suspicious' : m.state} · fix ${m.lastKnown ? flat(m.lastKnown, at).toFixed(2) + ' m off the noise' : 'none'} · seated ${seated} · woken by ${why} · moved ${moved.toFixed(2)} m in the first ${(DOZE.stirSecs - 0.15).toFixed(2)} s · slept again ${again}`); }

    // (d2) a whistle beside him is his to answer
    { const m = back(), at = clearPoint(g, m, 1.0); const { WHISTLE } = await import(`${ROOT}/src/game/whistle.ts`);
      g.playerInvisible = false; g.events.push({ kind: 'whistle', pos: [at[0], 0, at[2]], time: g.time, loud: false, level: WHISTLE.level, who: 1 }); for (let i = 0; i < 3; i++) W.step(DT); g.playerInvisible = true;
      line(!dozeAsleep(m) && dozeWoke(g, m) === 'noise' && m.state === 'suspicious', 'a whistle beside him wakes him, suspicious', `asleep ${dozeAsleep(m)} · woken by ${dozeWoke(g, m)} · ${m.state}`); }

    // (e) light on his face
    { const m = back(), head = m.char.bones.head ?? [m.char.pos[0], 1.15, m.char.pos[2]], f = [Math.sin(m.char.bodyYaw), 0, Math.cos(m.char.bodyYaw)];
      const l = g.engine.lights.lights.find((x: any) => x.enabled && !x.owner && x.kind !== 2 && !(x.ttl >= 0)), keep = l ? { pos: [...l.pos], range: l.range, intensity: l.intensity, kind: l.kind, radius: l.radius } : null; let at = -1;
      if (l) { for (let i = 0; i < 180 && at < 0; i++) { l.pos[0] = head[0] + f[0] * 0.8; l.pos[1] = head[1] + 0.6; l.pos[2] = head[2] + f[2] * 0.8; l.kind = 0; l.range = Math.max(l.range, 6); l.intensity = Math.max(l.intensity, 6); g.lightHome.feed(l); W.step(DT); if (!dozeAsleep(m)) at = i + 1; }   // (a lamp moved by hand is fed to its row: a light question reads the rows)
        Object.assign(l, { range: keep!.range, intensity: keep!.intensity, kind: keep!.kind }); l.pos[0] = keep!.pos[0]; l.pos[1] = keep!.pos[1]; l.pos[2] = keep!.pos[2]; g.lightHome.feed(l); }
      const lim = Math.ceil((DOZE.lightSecs + 2 * DOZE.askSecs) / DT) + 2;
      line(at > 0 && at <= lim && dozeWoke(g, m) === 'light' && m.awareness >= 0.3, 'light on his face wakes him inside the dwell, suspicious', l ? `a lamp put a metre off his face: ${at < 0 ? 'SLEPT ON for three seconds' : `woke ${(at * DT).toFixed(2)} s on`} (the bar ${(lim * DT).toFixed(2)} s) · woken by ${dozeWoke(g, m)}` : 'the level has no lamp to move'); }

    // (f) the chair shoved · (g) the floor going up
    { let m = back(); const c0 = chairOf(g, m); c0.pushT = g.time; c0.lastWho = 1; W.step(DT); W.step(DT); const bump = dozeWoke(g, m) === 'bump' && m.state === 'suspicious' && !closeSeated(g, m);
      m = back(); await quiet(() => raiseEscalation(g, 1)); W.step(DT); W.step(DT); const up = !dozeAsleep(m) && !closeSeated(g, m), why = dozeWoke(g, m);
      line(bump && up, 'his chair shoved wakes him suspicious; the floor going up has him out of the chair', `shoved: ${bump ? 'woke (bump), suspicious' : `${dozeWoke(g, m)} / ${m.state}`} · alarm raised: ${up ? `up (${why})` : 'STILL IN THE CHAIR'}`); }

    // (h) the take over the chair
    { const m = back(), c = m.char, behind = [c.pos[0] - Math.sin(c.bodyYaw) * (0.33 + CLOSE.seatDist - 0.1), 0, c.pos[2] - Math.cos(c.bodyYaw) * (0.33 + CLOSE.seatDist - 0.1)];
      const o = closeAt(g, m, behind), kind = closeKindOf(o);
      line(kind === 'seat' && dozeAsleep(m), 'from behind his chair the close-in mode offers the take over the chair, and he sleeps on', `the window behind him: ${kind}${o.win ? '' : ' (none)'} · asleep ${dozeAsleep(m)}`); }

    // ---- AT A SCREEN (the police post's terminal on the footlocker, built off; the chair is the lived-in bunk room's) ----
    { const W5 = await stand(24), g5 = W5.game; DOZE_LOG.length = 0;
      const mon = g5.targets.find((t: any) => t.group === 'monitors'), chair = mon && g5.props.props.find((p: any) => p.def.kind === 'chair' && Math.hypot(p.x - mon.pos[0], p.z - mon.pos[2]) < DOZE.screenM);
      if (!mon || !chair) { line(false, 'the police post has a screen with a chair at it', `screen ${!!mon} · chair ${!!chair}`); return false; }
      const c0 = [chair.x, 0, chair.z], builtOn = mon.on, pool: string[] = dozeLines();
      const atWork = (x: any) => dozeOf(x)?.phase === 'work';
      let m: any = null; for (let i = 0; i < 240 * 60 && !m; i++) { W5.step(DT); m = staff(g5).find(atWork) ?? null; }
      if (!m) { line(false, 'a man with a short wait in the dark sits to the screen there', 'nobody sat to it inside four minutes'); return false; }
      for (let i = 0; i < 60; i++) W5.step(DT);
      // (m) he sits to it: on the chair, facing it, the screen on, its light on his face, his torch out, the mark over him
      { const c = m.char, hips = [c.pos[0] - Math.sin(c.bodyYaw) * 0.33, 0, c.pos[2] - Math.cos(c.bodyYaw) * 0.33], onChair = flat(hips, [chair.x, 0, chair.z]);
        const to = Math.atan2(mon.pos[0] - c.pos[0], mon.pos[2] - c.pos[2]), off = Math.abs(Math.atan2(Math.sin(to - c.bodyYaw), Math.cos(to - c.bodyYaw)));
        const head: any = [c.pos[0], 1.1, c.pos[2]], lit = g5.visibilityOf(lightEstimateAt(g5, head));
        const dark = g5.visibilityOf(lightEstimateAt(g5, head, undefined, false, undefined, undefined, ROW_BIT.screen));   // (the light there with the screens' rows left out: doze.ts faceLight's own question)
        line(!builtOn && mon.on && mon.factor > 0.9 && onChair < 0.2 && off < 0.3 && flat([chair.x, 0, chair.z], c0) <= DOZE.pullM + 0.02 && lit > dark + 0.02 && !m.flashlight.enabled && closeSeated(g5, m) && dozeMark(g5, m) === '♪' && dozeAt(m) === 'screen' && !dozeAsleep(m) && m.state === 'patrol' && m.route.wait[m.wp] < DOZE.napWait,
          'a man with a short wait in the dark sits to the screen there and switches it on: on the chair, facing it, its light on his face, his torch out, awake, the mark over him',
          `${m.callsign} (${m.route.name}, a ${m.route.wait[m.wp]} s wait) · screen built ${builtOn ? 'ON' : 'off'}, now ${mon.on ? 'on' : 'OFF'} (supply ${mon.factor.toFixed(2)}) · the chair pulled ${flat([chair.x, 0, chair.z], c0).toFixed(2)} m up to it · hips ${onChair.toFixed(2)} m off the chair · ${(off * 180 / Math.PI).toFixed(0)}° off the screen · the light at his face ${lit.toFixed(3)} (${dark.toFixed(3)} with the screen dark) · torch ${m.flashlight.enabled ? 'BURNING' : 'out'} · mark '${dozeMark(g5, m)}'`); }
      const name5 = m.callsign, c1 = [chair.x, 0, chair.z], snap5 = W5.snapshot('doze-screen', true);
      const back5 = () => { Math.random = seeded(78); W5.restore(snap5); return same(g5, name5); };
      // (n) headphones on: the same footfall counts for a share at his ear — and a noise that does reach him has him up, the screen left on
      { const walkAt = (ears: number) => { const x = back5(); DOZE.ears = ears; const q = clearPoint(g5, x, 2.5); const a0 = x.awareness; if (q) step(W5, q, 0.26, 12); DOZE.ears = was.ears; return { q, d: x.awareness - a0, still: atWork(x) }; };
        const full = walkAt(1), half = walkAt(was.ears);
        const x = back5(), q = clearPoint(g5, x, 1.5); if (q) for (let k = 0; k < 4 && atWork(x); k++) step(W5, q, 0.6, 12);
        const r = dozeOf(x);
        line(!!full.q && full.d > 0.02 && half.d < full.d * 0.75 && half.still && !atWork(x) && !closeSeated(g5, x) && mon.on && r?.phase !== 'work' && x.awareness >= 0.3,
          'headphones on: a walking step near him counts for less than it would (or for nothing), and he stays at his screen — running steps beside him have him up and looking, the screen left on',
          `a walking step at 2.5 m: +${half.d.toFixed(3)} at his ear (+${full.d.toFixed(3)} with his ears whole) · after running steps at 1.5 m: ${r ? `${r.phase} (${r.why})` : 'no record'}, awareness ${x.awareness.toFixed(2)}, seated ${closeSeated(g5, x)}, screen ${mon.on ? 'on' : 'OFF'}`); }
      // (n2) the attention rule (attention.ts): his attention is the screen's — a lent hold at the screen that the floor shows, and the headphones counted ONCE
      //      (the lend passes 1 and 1). Until 2026-11-07 the line also held his sight rate and his hearing to the same with that rule off and with `lend` off.
      { const x = back5(); for (const o of staff(g5)) if (o !== x) o.dazzledUntil = g5.time + 10; for (let i = 0; i < 6; i++) W5.step(DT);
        const H = attentionOf(g5, x), front = attentionSideK(g5, x, mon.pos), marked = attentionMarks(g5).some((k: any) => k.gd === x && k.mode === 'lent') || !!H && g5.fogOfWar.hidden(x);
        const onA = { mode: H?.mode ?? 'none', ears: H?.ears ?? -1, noises: H?.noises ?? -1, off: H ? flat(H.at, mon.pos) : -1, front, marked };
        line(onA.mode === 'lent' && onA.off < 0.05 && onA.ears === 1 && onA.noises === 1 && onA.front === 1 && onA.marked,
          'his attention is the screen\'s: a lent hold at the screen that the floor shows, his eye whole at the screen, and the headphones counted once',
          `${onA.mode} hold ${onA.off.toFixed(2)} m from the screen, ears × ${onA.ears}, noises × ${onA.noises}, eye × ${onA.front} at the screen, on the floor ${onA.marked}`); }
      // (o) the screen dies under his eyes
      { const x = back5(); W5.game.lightHome.stage(mon, false, null); for (let i = 0; i < 20; i++) W5.step(DT); const r = dozeOf(x);
        line(!atWork(x) && !closeSeated(g5, x) && r?.why === 'light' && x.awareness >= DOZE.wakeAw - 0.05 && !!x.lastKnown && flat(x.lastKnown, mon.pos) < 0.5,
          'the screen goes dark under his eyes: that he notices — up, and looking at it', `${r ? `${r.phase} (${r.why})` : 'no record'} · awareness ${x.awareness.toFixed(2)} · looking ${x.lastKnown ? `${flat(x.lastKnown, mon.pos).toFixed(2)} m from the screen` : 'NOWHERE'}`); }
    }
  } finally { DOZE.ears = was.ears; Math.random = rnd0; }
  console.log(`=== doze: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await dozeCheck()) ? 0 : 1);
