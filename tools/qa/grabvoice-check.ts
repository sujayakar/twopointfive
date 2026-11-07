// THE GRAB'S THREE VOICES — the check behind the sounds of HOLD.disarm (src/game/disarm.ts, player.ts startGrab; the voices are audio/sfx.ts grabCatch, gunStrip
// and gunDown). The move is settled on, and so are the rule under it (HOLD.dropGun) and the disarm leading (HOLD.stripFirst): the check asks the page's side
// alone and writes no switch. NOTE: all but one of its takes used to hold HOLD.stripFirst off (the catch first, then the strip); with that write out every take
// is the page's (the gun first), so the lines that ask for the catch on the grab's own frame and the strip at HOLD.strip.take ask a build from before that
// rule: 'a drawn pistol: the catch on the grab's frame, the strip on the frame the gun leaves his hand', 'the listening page plays the whole grab on the
// game's own beats' (its catch-first row), the carbine's catch-first row, and 'torn loose before the take' (timed off HOLD.strip.take: tools/qa/stripfirst-check.ts
// `torn` asks it of the page's order). Those four went by ruling, with the staging line with the move off, the take with HOLD.dropGun off, and the comparisons
// with the move off or the disarm not leading.
//   bun run tools/qa/probes.ts grabvoice        (family `grab`)        GRABVOICE_BREAK=1 = the pistol man is left calm (his torch in his hand): nothing to strip, must FAIL
// the maintainer, 10-23: "we should also make the grab super satisfying. cool animation where sam disarms him and satisfying sounds".
// It asks WHAT IS PLAYED, WHEN, WHERE AND HOW LOUD (the headless audio is a list of calls: nobody here can listen — the voices themselves are measured by
// audio-render.ts and sound-takes-check.ts), and WHAT THE ROOM HEARS: the move adds nothing to the grab's one noise, which a second man hears to just under 4 m.
// THE STAGING is the hold probes' own (holddrop-check.ts): the office, one man placed with a gun in his hand, Sam behind him, startGrab as the game calls it.
import { standUp, ROOT } from './headless.ts';
import { place, park } from './standoff-probes.ts';
type Vec3 = [number, number, number];
const DT = 1 / 60, SEED = 4471, OTHER = 97;
export async function grabVoiceCheck(): Promise<boolean> {
  const BRK = process.env.GRABVOICE_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== grabvoice: the grab's three voices${BRK ? ' (nothing to strip: must fail)' : ''} ===`);
  const { startGrab } = await import(`${ROOT}/src/game/player.ts`);
  const { DISARM_GAIN } = await import(`${ROOT}/src/game/disarm.ts`); const { inHand } = await import(`${ROOT}/src/game/guards.ts`);
  const { SOUNDS } = await import(`${ROOT}/src/sounds/catalog.ts`);
  const log = console.log, AT: Vec3 = [14, 0, 11.2], YAW = Math.PI / 2, LOBBY: Vec3 = [10.2, 0, 19.5];
  const near = (a: number, b: number, e = 1e-6) => Math.abs(a - b) <= e;
  const guns = (g: any) => g.items.items.filter((it: any) => it.kind === 'rifle' || it.kind === 'pistol');
  /** one man with a gun in his hand taken from behind on a fresh stand, two seconds of it recorded: every audio call and every noise event from the grab on.
   *  `ear`: a second man stood that many metres ahead of him on the lobby's open floor, back turned, to hear it or not · `cancelAfter`: let go that many frames in ·
   *  `seed`: the game's own die (every take has one, so a red here can be run again) · `wall`: taken in
   *  the office, a wall 0.35 m off Sam's right hand (in the way of the thrown gun); every other take is on the lobby's open floor ·
   *  `key`: taken by the row's key inside a step,
   *  as the page takes him (the catch is heard on that frame, and the move's clock already reads one step at its end: called from out here the grab is a
   *  step ahead of its own voices, so every beat the page's rows are held to is counted on a take by the key) */
  const take = async (who: 'rifle' | 'pistol', o: { gunHold?: boolean; ear?: number; seed?: number; cancelAfter?: number; wall?: boolean; key?: boolean } = {}) => {
    console.log = () => { };
    try {
      const W: any = await standUp({ seed: o.seed ?? SEED }); const g = W.game; g.godMode = true; g.player.visibility = 0.02; g.tune.carbineGuards = who === 'rifle' ? 'reyes' : 'none';
      for (let i = 0; i < 5; i++) W.step(DT);
      const A = g.guards.find((x: any) => x.callsign === 'Reyes'), B = o.ear ? g.guards.find((x: any) => x !== A) : null, at = o.wall ? AT : LOBBY;
      g.guards.forEach((x: any, i: number) => { if (x !== A && x !== B) park(x, i); });
      place(A, at, YAW); A.wp = 1; if (B) { place(B, [at[0] + o.ear!, 0, at[2]], YAW); B.hold = true; B.pinned = true; }
      g.teleportPlayer([at[0] - Math.sin(YAW) * 0.8, 0, at[2] - Math.cos(YAW) * 0.8]); g.player.crouch = true; g.player.slot = 'pistol'; g.player.holstered = !o.gunHold; W.step(DT); g.player.visibility = 0.02;
      if (who === 'pistol' && !BRK) { A.state = 'suspicious'; A.awareness = 0.5; }   // (a man with a reason to have it out: the sidearm, not the torch, is in his hand)
      if (B) { B.state = 'patrol'; B.awareness = 0; }
      const had = inHand(A); W.audioLog = []; let t0 = g.time, peak = 0; const noise: any[] = [];
      if (o.key) { g.selectVerb('guard', 'grab'); W.input.pressed.add('Space'); W.step(DT); t0 = g.time; } else startGrab(g, A);
      for (let i = 0; i < 120; i++) { for (const e of g.events) if (e.time >= t0 - 1e-9 && !noise.includes(e)) noise.push(e); g.player.visibility = 0.02; if (i === o.cancelAfter) g.freeHeld('cancel'); W.step(DT); if (B) peak = Math.max(peak, B.awareness); }
      const calls = (W.audioLog as any[]).map(c => ({ name: c.name, at: c.t - t0, gain: c.gain, rate: (c.opts?.rate as number) ?? 1, pos: c.pos as Vec3 | null }));
      return { W, g, A, B, had, calls, noise, peak, gun: guns(g)[0] ?? null, throat: [at[0], 1.4, at[2]] as Vec3 };
    } finally { console.log = log; }
  };
  const one = (R: { calls: any[] }, name: string) => R.calls.filter(c => c.name === name);
  const said = (R: { calls: any[] }) => R.calls.filter(c => c.name !== 'footstep').map(c => `${c.name} ${c.at.toFixed(2)} s at ${c.gain.toFixed(2)}${c.rate !== 1 ? ` x${c.rate.toFixed(2)}` : ''}`).join(' · ') || 'nothing';
  try {
    const G = DISARM_GAIN;
    // ---- the table: the three rows are voices of the game and have their rows on the listening page
    { const rows = ['grabCatch', 'gunStrip', 'gunDown'], onPage = rows.filter(n => !!(SOUNDS as any)[n]), pageGain = rows.map(n => (SOUNDS as any)[n]?.gain), want = [G.catch, G.strip, G.down];
      const long = ['gunStrip', 'gunDown'].map(n => ((SOUNDS as any)[n]?.also ?? []).some((a: any) => near(a.rate ?? 1, G.long)));
      verdict(onPage.length === 3 && pageGain.every((x, i) => near(x, want[i])) && long.every(Boolean),
        'the three voices have their rows on the listening page, which plays each at the gain the game does and the long gun\'s two at the game\'s lower rate',
        `on the page ${onPage.join(', ') || 'none'} at ${pageGain.join(' / ')} (the game: ${want.join(' / ')}) · a carbine's strip and knock at x${G.long}: ${long.join(' / ')}`); }
    // ---- a drawn pistol taken from behind (the take the wall, the dice and the room's hearing are held to)
    const on = await take('pistol');
    // ---- a carbine is the same two buffers played lower; the pistol hold raps it out a little brighter
    { const R = await take('rifle', { key: true }), P = await take('pistol', { gunHold: true }), s = one(R, 'gunStrip'), d = one(R, 'gunDown'), ps = one(P, 'gunStrip'), pd = one(P, 'gunDown');
      verdict(s.length === 1 && near(s[0].rate, G.long) && d.length === 1 && near(d[0].rate, G.long) && ps.length === 1 && ps[0].rate > 1 && pd.length === 1 && pd[0].at < (one(on, 'gunDown')[0]?.at ?? 0),
        'a carbine: the strip and the knock at the long gun\'s lower rate; Sam\'s own pistol drawn: the gun rapped out (a brighter strip) falls from the man\'s hand and lands sooner',
        `carbine ${said(R)} · pistol hold ${said(P)}`); }
    // ---- the disarm leading (HOLD.stripFirst, settled on): the strip, then the catch as the arm lands; the page's two gun-first rows (under gunStrip, timed
    //      from the strip) are the game's
    { const lines: string[] = []; let ok = true;
      for (const [who, n] of [['pistol', 1], ['rifle', 2]] as const) {
        const F = await take(who, { key: true }), s = one(F, 'gunStrip'), c = one(F, 'grabCatch'), d = one(F, 'gunDown'), hp = one(F, 'heldProtest')[0]?.at;
        const row = (SOUNDS as any).gunStrip?.also?.[n], plus = row?.plus ?? [], pg = (m: string) => plus.find((l: any) => l.name === m)?.after as number | undefined, s0 = s[0]?.at ?? -9;
        const page = [[pg('grabCatch'), c[0]?.at], [pg('heldProtest'), hp], [pg('gunDown'), d[0]?.at]].every(([a, b]) => a !== undefined && b !== undefined && Math.abs(a - (b - s0)) <= 0.01) && near(row?.rate ?? 0, who === 'rifle' ? G.long : 1);
        ok &&= s.length === 1 && c.length === 1 && d.length === 1 && s0 < c[0].at && page;
        lines.push(`${who}: ${said(F)} · the page's row from the strip: catch ${pg('grabCatch')} s, protest ${pg('heldProtest')} s, gun down ${pg('gunDown')} s (the game: ${c[0] ? (c[0].at - s0).toFixed(3) : '—'} / ${hp !== undefined ? (hp - s0).toFixed(3) : '—'} / ${d[0] ? (d[0].at - s0).toFixed(3) : '—'})`);
      }
      verdict(ok, 'with the disarm leading (as the page has it): the strip, then the catch, and the gun on the floor, pistol and carbine; the listening page\'s gun-first rows are those beats, to the frame',
        lines.join(' · ')); }
    // ---- by a wall: he throws it short of the wall, so the knock is on the open floor's frame, once, where the gun lies
    { const Wl = await take('pistol', { wall: true }), d = one(Wl, 'gunDown'), it = Wl.gun, open = one(on, 'gunDown')[0]?.at ?? -9;
      const onFloor = !!d[0]?.pos && !!it && d[0].pos[1] < 0.2 && Math.hypot(d[0].pos[0] - it.pos[0], d[0].pos[2] - it.pos[2]) < 0.35;
      verdict(d.length === 1 && onFloor && Math.abs(d[0].at - open) <= 0.01, 'taken beside a wall that stands in the way of the throw: one knock, at the floor where the gun lies (not at the wall), on the open floor\'s frame',
        `gun down ${d.map(c => `${c.at.toFixed(3)} s at height ${c.pos ? c.pos[1].toFixed(2) : '?'}`).join(', ') || 'never'} · on open floor ${open.toFixed(3)} s · the knock ${onFloor ? 'on the floor by the gun' : 'NOT at the gun on the floor'}`); }
    // ---- the same beats on other dice
    { const B = await take('pistol', { seed: OTHER }), a = one(on, 'gunDown')[0], b = one(B, 'gunDown')[0];   // (the staging's own proof that the dice were other dice was the knocked-out gun of the move-off runs lying somewhere else: it went with those runs)
      const same = !!a && !!b && near(a.at, b.at) && near(a.gain, b.gain, 1e-4) && !!on.gun && !!B.gun && Math.hypot(on.gun.pos[0] - B.gun.pos[0], on.gun.pos[2] - B.gun.pos[2]) < 1e-3;
      verdict(same, 'the knock is on the same frame at the same gain, and the gun in the same place, whatever the dice (a beat a player can learn)',
        `gun down ${a ? a.at.toFixed(3) : '—'} / ${b ? b.at.toFixed(3) : '—'} s at ${a ? a.gain.toFixed(3) : '—'} / ${b ? b.gain.toFixed(3) : '—'} · the two guns lie ${on.gun && B.gun ? Math.hypot(on.gun.pos[0] - B.gun.pos[0], on.gun.pos[2] - B.gun.pos[2]).toFixed(4) : '—'} m apart (seeds ${SEED} and ${OTHER})`); }
    // ---- what the room hears: the grab's one noise and nothing more
    { const lv = (R: { noise: any[] }) => R.noise.map(e => `${e.kind} ${e.level}`).join(', ') || 'none';
      const same = on.noise.length === 1 && on.noise[0].kind === 'prop';
      const ears = [3.5, 4.5], heard: number[] = []; for (const d of ears) heard.push((await take('pistol', { ear: d })).peak);
      verdict(same && heard[0] > 0.05 && heard[0] < 0.45 && heard[1] === 0,
        'the room hears the grab\'s one scuffle and nothing of the strip or the gun set down: a second man 3.5 m off turns an ear (well short of alarm), one 4.5 m off hears nothing',
        `noise events from the grab on: ${lv(on)} · a man ${ears[0]} m off: awareness 0 → ${heard[0].toFixed(3)} · ${ears[1]} m off: 0 → ${heard[1].toFixed(3)}`); }
  } finally { console.log = log; }
  console.log(`=== grabvoice: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) { const ok = await grabVoiceCheck(); (globalThis as any).process.exit(ok ? 0 : 1); }
