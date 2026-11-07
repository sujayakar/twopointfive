// THE WALL HOLD, HEARD AND UNHURRIED — the check behind WALLSNAP.voice and WALLSNAP.slow (src/game/wallsnap.ts; player.ts wallVoice / wallSnapPose / wallStepOff,
// character.ts stanceIn, audio/sfx.ts 'wallOn' / 'wallOff'; both settled on: the check asks the page's side alone and writes neither; both act only with
// the hold's feel pass, WALLSNAP.on, which these lines turn on).
//   bun run tools/qa/probes.ts wallfelt        (families `wallsnap`, `wall-snap`)        WALLFELT_BREAK=1 = every take is tried 2.5 m off the wall, where Q takes nothing: must FAIL
// Until 2026-11-07 it also had `unhurried` (the same take twice as long and its fastest frame six tenths of what it was with WALLSNAP.slow off: a
// comparison with the off run and nothing else), the rates 'with the rule off', `off` (the scuff as it was) and `recordings`.
// the maintainer, 10-21, from his play: "we should have some audio feedback for when you snap/unsnap from a wall. the transition animation is also too fast,
// should probably be 2x slower".
// WHAT IS ASKED, on the office corridor's north wall (the feel pass's own rig, wallsnap-probes.ts world):
//   sidling     a key held along the wall through the take: he sidles on from the landing, never stalled, that way never shut
//   the pose    while he holds a wall the animator is given the slower rate on, and his own ways off give it the slower rate out; a hold ended any other way
//               (a teleport) leaves both at rest
//   heard       one 'wallOn' on the frame his back lands and none before it or after it while he stays and sidles; one 'wallOff' for each of his ways off (Q,
//               the stick held away, breaking into a run); the old scuff is not played as well; a teleport off the wall makes no sound; the rifle's stack
//               has both too; nothing is filed for anyone to hear
//   let go      Q and Q again before his back has landed: neither sound (there was no plaster to peel off)
//   the stack   one click made during the slower take is kept: one round leaves once the rifle is up
//   the table   both sounds are in the game's table and on the listening page's list, neither silent, clipped nor longer than half a second, the landing's
//               weight under 500 Hz and the step off's above it
// Headless: clocks, positions, the animator's channels and the calls to the mixer are read. NOBODY HAS HEARD EITHER SOUND OR SEEN THE SLOWER TAKE.
import { ROOT } from './headless.ts';
import { world } from './wallsnap-probes.ts';
export async function wallFeltCheck(): Promise<boolean> {
  const BRK = process.env.WALLFELT_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== wallfelt: taking and leaving a wall is heard${BRK ? ' (every take 2.5 m off the wall: must fail)' : ''} ===`);
  const { WALLSNAP } = await import(`${ROOT}/src/game/wallsnap.ts`);
  const was = { on: WALLSNAP.on, press: WALLSNAP.press }, log = console.log, err = console.error, warn = console.warn;
  const quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = console.error = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const f2 = (x: number) => x.toFixed(2);
  try {
    const S = await quiet(() => world(true)); const { pl, c, g } = S as any;
    WALLSNAP.on = true; const FAR = BRK ? 2.5 : 0;
    // every call to the mixer, by frame
    const heard: { f: number; name: string; gain: number; rate: number }[] = []; let frame = 0; const play0 = g.audio.play.bind(g.audio);
    g.audio.play = (name: string, pos: any, gain = 1, opts: any = {}) => { heard.push({ f: frame, name, gain, rate: opts?.rate ?? 1 }); return play0(name, pos, gain, opts); };
    const step = (n = 1, each?: () => void) => { for (let i = 0; i < n; i++) { frame++; S.step(1); each?.(); } };
    const names = (from: number, re = /^wall(On|Off)$/) => heard.filter(h => h.f > from && re.test(h.name)).map(h => h.name);
    const scuffs = (from: number) => heard.filter(h => h.f > from && h.name === 'footSoft' && Math.abs(h.rate - 0.7) < 1e-9).length;
    /** one take: frames from the first of the hold to his back on the wall, the fastest frame of it (m), and where it ended */
    const take = (z: number, yaw: number, how: 'q' | 'key', kit: 'empty' | 'rifle' = 'empty') => {
      WALLSNAP.press = how === 'key'; S.place(21, z + FAR, yaw, kit); frame += 40; if (how === 'q') S.tap('KeyQ'); else S.keys('KeyW');
      let began = -1, landed = -1, fast = 0, prev = [c.pos[0], c.pos[2]], secs = NaN;
      for (let f = 0; f < 150 && landed < 0; f++) { step(1); const d = Math.hypot(c.pos[0] - prev[0], c.pos[2] - prev[1]); prev = [c.pos[0], c.pos[2]];
        if (pl.wall) { if (began < 0) { began = f; secs = pl.wall.takeT; } fast = Math.max(fast, d); if (pl.wall.settle >= 1) landed = f; } }
      S.keys(); return { frames: landed - began, fast, secs, at: [c.pos[0], c.pos[2]], yaw: c.bodyYaw, stance: c.anim.stance, on: !!pl.wall };
    };
    const off = () => { if (pl.wall) { S.tap('KeyQ'); step(30); } WALLSNAP.press = false; };

    // ---- a key held along the wall through the take: he sidles on from the landing (the glide's own lag behind a station that ran on is not a man in his way)
    { S.place(21, 10.9 + FAR, 0); frame += 40; S.tap('KeyQ'); S.keys('KeyD'); let landed = -1, stalls = 0, shut = 0; const u0 = c.pos[0];
      step(70, () => { const w = pl.wall; if (!w) return; if (landed < 0 && w.settle >= 1) landed = 0; else if (landed >= 0) { landed++; if (Math.abs(w.v) < 0.3) stalls++; if (w.blockDir) shut++; } });
      const went = Math.abs(c.pos[0] - u0); S.keys(); off();
      verdict(landed > 20 && stalls === 0 && shut === 0 && went > 0.8, 'a key held along the wall through the slower take: he sidles on from the landing with no stall', `${landed} frames on the wall after the landing · frames slower than 0.3 m/s ${stalls} · frames with that way shut ${shut} · ${f2(went)} m along in 70 frames`); }

    // ---- the pose's rates
    { take(10.9, 0, 'q'); step(10); const inOn = c.anim.stanceIn; S.tap('KeyQ'); step(1); const outOn = c.anim.stanceOut, inAfter = c.anim.stanceIn; step(30);
      take(10.9, 0, 'q'); step(10); g.teleportPlayer([21, 0, 13]); step(1); const telIn = c.anim.stanceIn, telOut = c.anim.stanceOut; step(20);
      verdict(inOn === WALLSNAP.slowIn && outOn === WALLSNAP.slowOut && inAfter === 0 && telIn === 0 && telOut === 0 && WALLSNAP.slowIn < 22 && WALLSNAP.slowOut < WALLSNAP.outRate,
        'on a wall the pose is given the slower rate on, his own way off the slower rate out; a teleport leaves both at rest',
        `on the wall ${inOn} (its own is 22) · Q off: out ${outOn}, on ${inAfter} · a teleport off: ${telIn} / ${telOut}`); }

    // ---- heard
    { const propsBefore = g.events.filter((e: any) => e.kind === 'prop').length;
      // Q on: the landing's frame, and nothing more while he stays and sidles
      S.place(21, 10.9 + FAR, 0); frame += 40; const f0 = frame; S.tap('KeyQ'); let landedAt = -1; step(80, () => { if (landedAt < 0 && pl.wall && pl.wall.settle >= 1 && pl.wall.seated) landedAt = frame; });
      const onAt = heard.filter(h => h.f > f0 && h.name === 'wallOn').map(h => h.f); S.keys('KeyD'); step(60); S.keys(); step(20); const stay = names(f0);
      // his three ways off, one sound each
      const f1 = frame; S.tap('KeyQ'); step(30); const offQ = names(f1);
      take(10.9, 0, 'q'); step(20); const f2_ = frame; S.keys('KeyS'); step(40); S.keys(); step(10); const offAway = names(f2_), awayFree = !pl.wall;
      take(10.9, 0, 'q'); step(20); const f3 = frame; S.keys('KeyD', 'ShiftLeft'); step(20); S.keys(); step(20); const offRun = names(f3), runFree = !pl.wall;
      // a teleport: no sound
      take(10.9, 0, 'q'); step(20); const f4 = frame; g.teleportPlayer([21, 0, 13]); step(20); const tel = names(f4);
      // the stack
      const f5 = frame; take(10.9, 0, 'q', 'rifle'); step(20); const stack = pl.wall?.kind === 'stack'; S.tap('KeyQ'); step(30); const stackHeard = names(f5);
      const sc = scuffs(f0), props = g.events.filter((e: any) => e.kind === 'prop').length - propsBefore, gains = heard.filter(h => h.f > f0 && /^wall/.test(h.name)).every(h => h.gain === (h.name === 'wallOn' ? WALLSNAP.voiceOn : WALLSNAP.voiceOff));
      verdict(onAt.length === 1 && onAt[0] === landedAt && stay.join() === 'wallOn' && offQ.join() === 'wallOff' && offAway.join() === 'wallOff' && awayFree && offRun.join() === 'wallOff' && runFree && tel.length === 0 && stack && stackHeard.join() === 'wallOn,wallOff' && sc === 0 && props === 0 && gains,
        'one sound as his back lands and none while he stays and sidles; one for each of his ways off; none for a teleport; the stack has both; the old scuff is not played as well and nothing is filed for anyone to hear',
        `landing: heard on frame ${onAt.map(f => f - f0).join(',') || 'never'}, his back on it on frame ${landedAt - f0} · staying and sidling: ${stay.join(',') || 'nothing'} · Q off: ${offQ.join(',') || 'nothing'} · the stick away: ${offAway.join(',') || 'nothing'} · a run: ${offRun.join(',') || 'nothing'} · a teleport off: ${tel.join(',') || 'nothing'} · the stack ${stack}: ${stackHeard.join(',') || 'nothing'} · old scuffs ${sc} · 'prop' events filed ${props} · at the table's gains ${gains}`); }

    // ---- a take let go of before his back landed: nothing to peel off, so nothing is heard
    { S.place(21, 11.0 + FAR, 180); frame += 40; const f0 = frame; S.tap('KeyQ'); step(5); const mid = !!pl.wall && pl.wall.settle < 1; S.tap('KeyQ'); step(30); const n = names(f0), free = !pl.wall;
      verdict(mid && free && n.length === 0, 'Q and Q again before his back lands: off the wall, and neither sound', `in the take at the second Q ${mid} · off it after ${free} · heard ${n.join(',') || 'nothing'}`); }

    // ---- the stack: a click that falls inside the longer take is kept, and a round leaves once the rifle is up
    { const shot = () => { S.place(21.5, 10.9 + FAR, 270, 'rifle'); frame += 40; S.setCursor([15, 1.2, 10.9]); step(10); const n0 = pl.rifle.roundsReady; S.tap('KeyQ'); step(8); const inTake = !!pl.wall && pl.wall.settle < 1, stack = pl.wall?.kind === 'stack'; S.click(); let at = -1; step(70, () => { if (at < 0 && pl.rifle.roundsReady < n0) at = frame; });
        const left = n0 - pl.rifle.roundsReady; S.setCursor(null); off(); return { inTake, stack, left }; };
      const a = shot();
      verdict(a.stack && a.inTake && a.left === 1, 'on the stack, one click made during the slower take sends one round once the rifle is up', `stacked ${a.stack} · the click fell inside the take ${a.inTake} · rounds that left ${a.left}`); }

    // ---- the table
    { const { buildSfxLater } = await import(`${ROOT}/src/audio/sfx.ts`); const { sfxContext } = await import(`${ROOT}/src/audio/backend.ts`); const { SOUNDS } = await import(`${ROOT}/src/sounds/catalog.ts`); const SR = 48000;
      const later: Map<string, () => any> = buildSfxLater(sfxContext(SR)); const table: any = { wallOn: later.get('wallOn')?.(), wallOff: later.get('wallOff')?.() }; const rows: string[] = []; let ok = true; const low: Record<string, number> = {};
      for (const name of ['wallOn', 'wallOff']) {
        const b = table[name], d: Float32Array | undefined = b?.getChannelData(0); if (!d) { ok = false; rows.push(`${name}: NOT IN THE TABLE`); continue; }
        let peak = 0, e = 0, lo = 0, y = 0, bad = false; const a = 1 - Math.exp(-2 * Math.PI * 500 / SR);
        for (let i = 0; i < d.length; i++) { const x = d[i]; if (!Number.isFinite(x)) bad = true; peak = Math.max(peak, Math.abs(x)); e += x * x; y += a * (x - y); lo += y * y; }
        low[name] = lo / Math.max(1e-12, e); const rms = Math.sqrt(e / d.length), good = !bad && peak > 0.2 && peak < 1 && rms > 0.02 && b.duration <= 0.5 && !!SOUNDS[name]; ok &&= good;
        rows.push(`${name}: ${f2(b.duration)} s, peak ${f2(peak)}, rms ${rms.toFixed(3)}, ${(low[name] * 100).toFixed(0)} % of it under 500 Hz, on the listening page ${!!SOUNDS[name]}${good ? '' : ' ✗'}`);
      }
      verdict(ok && low.wallOn > 0.5 && low.wallOff < 0.5, 'both sounds are in the table and on the listening page, neither silent, clipped nor over half a second; the landing sits low and the step off above it', rows.join(' · ')); }

  } finally { WALLSNAP.on = was.on; WALLSNAP.press = was.press; console.log = log; console.error = err; console.warn = warn; }
  console.log(`=== wallfelt: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await wallFeltCheck()) ? 0 : 1);
