// tools/qa/gunhand-reload-check.ts — a long gun reloads with a move of its own: the trigger hand keeps the grip, the gun comes to a carry a rifle is worked in
// and goes back to the carriage it came from, the head goes down to the well and comes back (src/game/gunhand.ts GUNHAND.longReload; character.ts LONG_RELOAD /
// reloadCarry).
//
// Before: through a long gun's magazine change the trigger hand and the whole upper body played the PISTOL's reload clip — the rifle swung up to the chest in a
// fist turned for a sidearm, the trunk and head with it — while the support arm worked a magazine on a gun that was no longer where a rifle is held.
//
// THE BENCH (no game: a figure of each side with its long gun, driven the way the game drives the animator — the player's own clock, or the beat the game
// deals a guard and a room-fight teammate; two figures stepped by the same calls: one that reloads, and one that never does):
//   (3) ON THE GRIP: in every frame of every reload the drawn grip is in the trigger hand's palm, and the long hold never lets go (longShown);
//   (4) AT THE WELL: the support hand is on the magazine's mark, on the gun where it now is, when the empty goes and when the fresh one seats;
//   (5) A RIFLE'S MOVE: the gun comes off its carriage (the muzzle up) and never turns more than a few degrees in a frame; the head goes down to the well; the
//       chest stays the carriage's own;
//   (6) BACK WHERE IT WAS: by the reload's last frame the gun, the trigger hand and the head are where the figure that never reloaded has them;
//   (7) THE GAME'S RULES: `reloading` is true through the change where the clip's clock runs, and never for a room-fight teammate, who plays no clip;
//   (10) against a wall (a low carriage tucked) the muzzle stays in its tuck; (10c) the fog cone's gaze does not jump; (10b) a beat that stands still.
// THE GAME (the player's rifle on the range the rifle probes use):
//   (11) the reload ends with the rounds in, no click through it fires, and the gun is on its carriage when they are in and when the next round leaves.
// THE RULE IS SETTLED (GUNHAND.on, the in-hand pass, the movement pass and the 2010s gear are constants of the game), so since 2026-11-07 nothing here
// writes them and no figure is stepped with the rule off. Gone, with their numbers: (1) the two stands' row and word; (2) the fault with the rule off;
// the half of (6), (7) and (11) that held the reload against the rule-off one; (8) a pistol's reload the same numbers off and on; (9) the movement pass
// off; (12) a recording from before the key and the page's off flag. THE BULLPUP IS GONE TOO: the player's first six reloads, and (10), were staged on
// the rifle of before the gear rule by writing that rule off for the drawer, which was refused, so its magazine was asked on a gun drawn as the issue
// carbine ((4) was red by that alone). (10) is now asked of the issue carbine.
//
//   bun run tools/qa/gunhand-reload-check.ts [--verbose]
//   GUNHAND_BREAK=1 …                                        (the staging broken: no figure of (3) to (10c) is ever told to reload — (4), (5), (7)
//                                                             and (10) must fail)
import './stand.ts';   // (the stand's word, written down before this file reads it)
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');

type V3 = [number, number, number];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]], dot = (a: V3, b: V3) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2], len = (a: V3) => Math.hypot(a[0], a[1], a[2]);
const deg = (c: number) => Math.acos(Math.max(-1, Math.min(1, c))) * 180 / Math.PI;
const qrot = (q: number[], v: V3): V3 => { const [x, y, z, w] = q, ux = y * v[2] - z * v[1], uy = z * v[0] - x * v[2], uz = x * v[1] - y * v[0]; return [v[0] + 2 * (w * ux + y * uz - z * uy), v[1] + 2 * (w * uy + z * ux - x * uz), v[2] + 2 * (w * uz + x * uy - y * ux)]; };
/** a joint of the DRAWN skeleton, world (the look-only pass may have moved a hand off the bones the game reads) */
const drawn = (c: any, node: number): V3 => { const w = c.world, p = c.skel.nodePos(node); return [w[0] * p[0] + w[4] * p[1] + w[8] * p[2] + w[12], w[1] * p[0] + w[5] * p[1] + w[9] * p[2] + w[13], w[2] * p[0] + w[6] * p[1] + w[10] * p[2] + w[14]]; };
const hollowR = (c: any): V3 => { const h = drawn(c, c.anim.nodes.handR), hf = c.handFrameR(); return [0, 1, 2].map(i => h[i] + hf.fingers[i] * 0.09 - hf.back[i] * 0.03) as V3; };
/** how far a point stands off a drawn box (0 inside it) */
const offBox = (b: any, p: V3): number => { const r = sub(p, b.c), q = b.rot ? [-b.rot[0], -b.rot[1], -b.rot[2], b.rot[3]] : [0, 0, 0, 1], l = qrot(q, r); return Math.hypot(...[0, 1, 2].map(i => Math.max(0, Math.abs(l[i]) - b.h[i]))); };
/** the gun's frame as rigProps draws it off what the game reads (x his left, y up out of the receiver, z the bore), and a point of it in the world */
const gunAt = (c: any, p: readonly number[]): V3 => { const d = c.gunDir as V3, g = c.gunUnder() as V3, u: V3 = [-g[0], -g[1], -g[2]], x: V3 = [u[1] * d[2] - u[2] * d[1], u[2] * d[0] - u[0] * d[2], u[0] * d[1] - u[1] * d[0]], h = c.bones.handR as V3;
  return [0, 1, 2].map(i => h[i] + x[i] * p[0] + u[i] * p[1] + d[i] * p[2]) as V3; };

export async function gunhandReloadCheck(): Promise<boolean> {
  const H: any = await import('./headless.ts');
  const CH: any = await import(`${ROOT}/src/game/character.ts`);
  const FOG: any = await import(`${ROOT}/src/game/fogofwar.ts`);
  const RP: any = await import(`${ROOT}/src/game/rigProps.ts`);
  const KN: any = await import(`${ROOT}/src/game/knobs.ts`);
  const VERBOSE = process.argv.includes('--verbose'), BREAK = process.env.GUNHAND_BREAK === '1';
  let all = true; const f1 = (x: number) => x.toFixed(1), cm = (x: number) => (x * 100).toFixed(1);
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  console.log('\n=== gunhand-reload: a long gun reloads with a move of its own — the trigger hand on the grip, the gun to a carry and back, the head to the well ===');
  const knobs0 = KN.knobsCopy();
  try {
    // ------------------------------------------------------------ the bench
    const ch: any = await H.loadCastCharacter(); const DT = 1 / 60;
    type Gun = 'm4' | 'carbine';
    interface Man { c: any; gun: Gun | 'pistol' }
    /** who: an operator (the player, a teammate: drilled) · a defender (untrained) · a responder (a trained guard) */
    const man = (who: 'operator' | 'defender' | 'responder', gun: Gun | 'pistol'): Man => { const op = who === 'operator', c = new CH.Character(ch, { id: op ? 21 : 24, tint: [0.1, 0.1, 0.1], tint2: [0.05, 0.05, 0.05], pos: [0, 0, 0], yaw: 0 });
      const a = c.anim; a.phase = 0; a.trained = op ? 1 : who === 'responder' ? 0.75 : 0.12; a.upper = 'none'; a.stance = 'none'; a.longGun = gun !== 'pistol';
      if (gun !== 'pistol') { a.longSteady = op ? 1 : 0; a.longGrip = op ? RP.sc20kGrip() : RP.carbineGrip(); c.muzzleAhead = op ? RP.sc20kMuzzleAhead() : RP.carbineMuzzleAhead(); a.reloadMag = true; }
      return { c, gun }; };
    /** his boxes this frame as combat.ts deals them (the operator's rifle is the issue carbine) */
    const draw = (m: Man, meta: any[]): any[] => { const out: any[] = [];
      { if (m.gun === 'carbine') RP.rigBoxes(m.c, { goggles: false, nv: false, slot: 1, lightOn: false, isGuard: true, holster: false, torch: false, weapon: 'carbine' }, out, RP.RIG, meta);
        else if (m.gun !== 'pistol') RP.rigBoxes(m.c, { goggles: false, nv: false, slot: 0, lightOn: false, isGuard: false, holster: false, rifle: 'hand' }, out, RP.RIG, meta); }
      return out; };
    const step = (m: Man) => { CH.MOVES.reload = 1; const c = m.c;
      c.inHand = m.gun === 'pistol' ? null : RP.longHold(m.gun === 'carbine' ? 'carbine' : 'sc20k'); if (c.vel) c.pos = [c.pos[0] + c.vel[0] * DT, c.pos[1], c.pos[2] + c.vel[2] * DT]; c.update(DT); };
    interface Row { t: number; grip: number; well: number; mag: number; fore: number; turn: number; pitch: number; headOff: number; headEnd: number; headDown: number; chest: number; gunOff: number; handOff: number; shown: boolean; carry: number; gaze: number; reloading: boolean }
    interface Beat { dur: number; out: number; in_: number; rack: number }
    const FN: Beat = { dur: 2.4, out: 0.5, in_: 1.5, rack: 2.0 }, AK: Beat = { dur: 2.6, out: 0.7, in_: 1.7, rack: 2.2 };
    type Pose = (c: any) => void;
    const POSES: Record<string, Pose> = {
      'low ready': c => { c.anim.upper = 'none'; c.anim.stance = 'lowReady'; },
      'on the aim': c => { c.anim.upper = 'aim'; c.anim.stance = 'none'; },
      'crouched on the aim': c => { c.anim.upper = 'aim'; c.anim.stance = 'none'; c.anim.crouchTarget = 1; },
      'on the aim, up a stair': c => { c.anim.upper = 'aim'; c.anim.stance = 'none'; c.aimPitch = 0.3; },
      'walking at the low ready': c => { c.anim.upper = 'none'; c.anim.stance = 'lowReady'; c.anim.speed = 1.4; c.vel = [0, 0, 1.4]; },
      'calm patrol': c => { c.anim.upper = 'none'; c.anim.stance = 'none'; },
      'high port': c => { c.anim.upper = 'none'; c.anim.stance = 'highReady'; },
    };
    /** one reload, two figures by the same calls: B reloads, R never does. `clock`: 'own' = reload(dur, beat), the player's; 'game' = the beat dealt frame by
     *  frame with the clip started beside it (a guard: guards.ts startReloadBeat + game.ts) or without (a room-fight teammate). GUNHAND_BREAK=1: B is never told */
    const play = (who: 'operator' | 'defender' | 'responder', gun: Gun, pose: Pose, beat: Beat, clock: 'own' | 'guard' | 'mate', wall = 0): Row[] => {
      const B = man(who, gun), R = man(who, gun), rows: Row[] = [], men: Man[] = [B, R];
      for (const m of men) { pose(m.c); m.c.anim.gunWall = wall; }
      for (let i = 0; i < 90; i++) for (const m of men) step(m);
      const M = CH.reloadMarks(false), N = Math.round(beat.dur * 60);
      if (BREAK) { /* nobody reloads */ } else if (clock === 'own') B.c.anim.reload(beat.dur, beat);
      else if (clock === 'guard') B.c.anim.reload();
      let last: V3 | null = null;
      for (let f = 0; f < N + 24; f++) { const t = f * DT;
        if (clock !== 'own' && !BREAK) { B.c.anim.reloadRow = beat; B.c.anim.reloadAt = t < beat.dur ? t : -1; }
        for (const m of men) step(m);
        const c = B.c, meta: any[] = [], bx = draw(B, meta), gm = meta.find(x => /\.grip$/.test(x.path)), mg = meta.find(x => /\.mag$/.test(x.path)), gl = meta.find(x => /gripL$/.test(x.path));
        const gd = c.gunDir as V3, rel = (m: Man, k: string): V3 => sub(m.c.bones[k], m.c.pos);
        rows.push({ t, grip: gm ? len(sub(bx[gm.boxStart].c, hollowR(c))) : 0, well: M ? len(sub(c.bones.handL, gunAt(c, M.mag))) : 0, mag: mg ? offBox(bx[mg.boxStart], drawn(c, c.anim.nodes.handL)) : 0, fore: gl ? len(sub(drawn(c, c.anim.nodes.handL), gl.frame.o)) : 0,
          turn: last ? deg(dot(last, gd)) : 0, pitch: Math.asin(Math.max(-1, Math.min(1, gd[1]))) * 180 / Math.PI,
          headOff: deg(dot(c.headDir(), R.c.headDir())), headEnd: deg(dot(c.headDir(), R.c.headDir())), headDown: (R.c.headDir()[1] - c.headDir()[1]), chest: len(sub(rel(B, 'chest'), rel(R, 'chest'))), gunOff: deg(dot(gd, R.c.gunDir)), handOff: len(sub(rel(B, 'handR'), rel(R, 'handR'))),
          shown: c.anim.longShown !== null, carry: c.anim.rlShown.carry, gaze: (g => Math.atan2(g[0], g[2]) * 180 / Math.PI)(FOG.gazeOf(c)), reloading: c.anim.reloading });
        last = [...gd] as V3; }
      return rows; };
    const mx = (rs: Row[], f: (r: Row) => number) => rs.reduce((a, r) => Math.max(a, f(r)), 0), at = (rs: Row[], t: number) => rs[Math.round(t * 60) - 1];
    const CASES: [string, 'operator' | 'defender' | 'responder', Gun, Beat, 'own' | 'guard' | 'mate', string[]][] = [
      ['the player, the issue carbine (the box ahead of it)', 'operator', 'm4', FN, 'own', ['low ready', 'on the aim', 'crouched on the aim', 'walking at the low ready']],
      ['a teammate in the room fight (the game\'s beat, no clip)', 'operator', 'm4', FN, 'mate', ['on the aim', 'crouched on the aim', 'low ready']],
      ['a defender, the short carbine (the game\'s beat over the clip)', 'defender', 'carbine', AK, 'guard', ['on the aim', 'crouched on the aim', 'calm patrol', 'low ready']],
      ['a responder, the short carbine', 'responder', 'carbine', AK, 'guard', ['on the aim', 'walking at the low ready']],
    ];
    const runs: { name: string; pose: string; beat: Beat; clock: string; on: Row[] }[] = [];
    for (const [name, who, gun, beat, clock, poses] of CASES) for (const p of poses) runs.push({ name, pose: p, beat, clock, on: play(who, gun, POSES[p], beat, clock) });
    const each = (f: (r: typeof runs[number]) => { ok: boolean; say: string }, verbose = VERBOSE): { ok: boolean; say: string } => { let ok = true; const bad: string[] = [], allSay: string[] = [];
      for (const r of runs) { const v = f(r); ok &&= v.ok; const s = `${r.name}, ${r.pose}: ${v.say}`; allSay.push(s); if (!v.ok) bad.push(s); } return { ok, say: (verbose || !ok ? (ok ? allSay : bad) : []).join('\n    ') }; };
    
    const body = (rs: Row[], b: Beat) => rs.filter(r => r.t < b.dur);

    // (3) on the grip
    { const v = each(r => { const g = mx(r.on, x => x.grip), sh = r.on.every(x => x.shown); return { ok: g <= 0.015 && sh, say: `palm off the grip ${cm(g)} cm at worst${sh ? '' : ' · THE LONG HOLD LET GO'}` }; });
      line(v.ok, '(3) on the grip: in every frame of every reload the drawn grip is in the trigger hand\'s palm (to 1.5 cm), and the long hold never lets go', `worst ${cm(Math.max(...runs.map(r => mx(r.on, x => x.grip))))} cm over ${runs.length} reloads, ${runs.reduce((a, r) => a + r.on.length, 0)} frames${v.say ? '\n    ' + v.say : ''}`); }
    // (4) at the well
    { const v = each(r => { const o = at(r.on, r.beat.out), i = at(r.on, r.beat.in_); return { ok: o.well <= 0.03 && i.well <= 0.03 && o.mag <= 0.12 && i.mag <= 0.12, say: `wrist off its mark ${cm(o.well)} / ${cm(i.well)} cm · wrist ${cm(o.mag)} / ${cm(i.mag)} cm from the drawn magazine (out / seat)` }; });
      const w = Math.max(...runs.flatMap(r => [at(r.on, r.beat.out).well, at(r.on, r.beat.in_).well])), m = Math.max(...runs.flatMap(r => [at(r.on, r.beat.out).mag, at(r.on, r.beat.in_).mag]));
      line(v.ok, '(4) at the well: when the empty goes and when the fresh one seats, the support hand\'s wrist is on the magazine\'s mark on the gun where it now is (to 3 cm) — a hand\'s width beside the magazine as drawn', `worst: wrist ${cm(w)} cm off the mark · ${cm(m)} cm from the drawn box${v.say ? '\n    ' + v.say : ''}`); }
    // (5) a rifle's move
    { const LR = CH.LONG_RELOAD;
      const v = each(r => { const b = body(r.on, r.beat), t = mx(r.on, x => x.turn), mid = at(r.on, (r.beat.out + r.beat.in_) / 2), want = LR.box.pitch, hd = mx(b, x => x.headOff), cs = mx(b, x => x.chest), cy = mx(b, x => x.carry);
        return { ok: t <= 5 && cy > 0.99 && Math.abs(mid.pitch - want) <= 3 && hd >= 10 && mx(b, x => x.headDown) > 0.1 && cs <= 0.012, say: `turn ${f1(t)}°/frame · muzzle ${f1(mid.pitch)}° up mid-change (the carry's ${want}), ${f1(mx(b, x => x.gunOff))}° off the carriage · head ${f1(hd)}° off, down · chest ${cm(cs)} cm` }; });
      line(v.ok, '(5) a rifle\'s move: the gun comes off its carriage to the reload carry (the muzzle a little up, to 3°) and never turns more than 5° in a frame; the head goes down to the well (10° or more); the chest stays the carriage\'s own (to 1.2 cm)',
        `fastest frame ${f1(Math.max(...runs.map(r => mx(r.on, x => x.turn))))}° · head down ${f1(Math.min(...runs.map(r => mx(body(r.on, r.beat), x => x.headOff))))}° at the least · chest ${cm(Math.max(...runs.map(r => mx(body(r.on, r.beat), x => x.chest))))} cm at worst${v.say ? '\n    ' + v.say : ''}`); }
    // (6) back where it was
    { const v = each(r => { const e = at(r.on, r.beat.dur), l = r.on[r.on.length - 1]; return { ok: e.gunOff <= 0.5 && e.handOff <= 0.005 && e.headEnd <= 1 && e.carry === 0 && l.gunOff <= 0.2 && l.handOff <= 0.002, say: `at the last frame: bore ${e.gunOff.toFixed(2)}°, trigger hand ${cm(e.handOff)} cm, head ${e.headEnd.toFixed(2)}° off the figure that never reloaded · 0.4 s on: ${l.gunOff.toFixed(2)}°, ${cm(l.handOff)} cm` }; });
      line(v.ok, '(6) back where it was: on the reload\'s last frame the gun, the trigger hand and the head are where the figure that never reloaded has them, and the carry is out', `worst at the last frame: bore ${Math.max(...runs.map(r => at(r.on, r.beat.dur).gunOff)).toFixed(2)}° · hand ${cm(Math.max(...runs.map(r => at(r.on, r.beat.dur).handOff)))} cm${v.say ? '\n    ' + v.say : ''}`); }
    // (7) the game's rules
    { const v = each(r => { const b = r.on.filter(x => x.reloading).length; return { ok: r.clock === 'mate' ? b === 0 : b > 60, say: `reloading for ${b} frames` }; });
      line(v.ok, '(7) the game\'s rules: `reloading` is true through the change where the clip\'s clock runs (over 60 frames), and never for a room-fight teammate, who plays no clip', v.say); }
    // (10) against a wall
    { const p = play('operator', 'm4', POSES['low ready'], FN, 'own', 1), b = body(p, FN);
      line(mx(b, r => r.gunOff) <= 1 && mx(b, r => r.carry) === 0 && mx(p, r => r.grip) <= 0.015 && at(p, FN.out).well <= 0.03 && mx(b, r => r.headOff) >= 10, '(10) against a wall (the low ready tucked): the muzzle keeps to its tuck, the hands do the change there, the head still goes down',
        `head ${f1(mx(b, r => r.headOff))}° · bore ${mx(b, r => r.gunOff).toFixed(2)}° off the tuck · palm off the grip ${cm(mx(p, r => r.grip))} cm · wrist off the well's mark ${cm(at(p, FN.out).well)} cm`); }

    // (10c) the fog cone's gaze (fogofwar.ts gazeOf: his head, and on his sights his aim) through the change
    { const sw = (rs: Row[]) => rs.reduce((a, r, i) => i ? Math.max(a, Math.abs(((r.gaze - rs[i - 1].gaze + 540) % 360) - 180)) : 0, 0);
      const on = Math.max(...runs.map(r => sw(r.on))), worst = runs.reduce((a, r) => sw(r.on) > sw(a.on) ? r : a);
      line(on <= 3, '(10c) the fog cone does not jump: through every reload the gaze the cone is laid on (his aim on his sights, his head at the well) turns no more than 3° in a frame', `fastest frame ${f1(on)}° (${worst.name}, ${worst.pose})`); }

    // (10b) a beat that stands still
    { const m = man('defender', 'carbine'); POSES['on the aim'](m.c); for (let i = 0; i < 90; i++) step(m);
      const an = m.c.anim, deal = (t: number) => { an.reloadRow = AK; an.reloadAt = t; step(m); };
      an.reload(); let t = 0, up = 0; for (; t < 1.0; t += DT) { deal(t); up = Math.max(up, an.rlShown.carry); }
      const well = an.atWell; for (let f = 0; f < 45; f++) deal(t);   // (off his man: guards.ts counts the beat down only at his fire sites — game.ts deals the same second every frame)
      const still = an.rlShown.carry, stillHead = an.rlShown.head, eyes = an.atWell; let again = 0; for (; t < AK.dur; t += DT) { deal(t); again = Math.max(again, an.rlShown.carry); }
      for (let f = 0; f < 6; f++) { an.reloadAt = -1; step(m); }
      line(up > 0.99 && well === true && still === 0 && stillHead === 0 && eyes === false && again > 0.99 && an.rlShown.carry === 0, '(10b) a beat that stands still (a guard off his man): the gun is back on its carriage and his head is up within three quarters of a second, his eyes are his sights\' again, and the carry comes back up when the change goes on',
        `carry ${up.toFixed(2)} into the change (at the well: ${well}) · ${still.toFixed(2)} and head ${stillHead.toFixed(2)} after 0.75 s of a stopped beat (at the well: ${eyes}) · ${again.toFixed(2)} once it runs again · ${an.rlShown.carry.toFixed(2)} after it`); }

    // ------------------------------------------------------------ the game
    { const FNP: any = await import('./fn2000-probes.ts');
      const game = async () => { const W: any = await hushed(() => FNP.stage(2002)); const g = W.game, pl = g.player, rf = pl.rifle, I = W.input, an = pl.char.anim;
        for (let f = 0; f < 60; f++) { W.cursorAt([27.5, 1.2, 10.0]); if (f % 12 === 0) I.clicked |= 1; W.step(1 / 60); } for (let f = 0; f < 20; f++) W.step(1 / 60);
        const mag0 = rf.mag ?? -1, shots0 = rf.shotsFired; I.pressed.add('KeyR'); let done = -1, carryAtDone = -1, top = 0, began = false, during = 0, shown = true;
        for (let f = 0; f < 200 && done < 0; f++) { W.cursorAt([27.5, 1.2, 10.0]); if (f > 0 && f % 20 === 0) I.clicked |= 1;   // (a click every third of a second through the change: nothing may leave the gun)
          W.step(1 / 60); if (f === 0) began = rf.reloading; top = Math.max(top, an.rlShown.carry); shown &&= an.longShown !== null;
          if (f > 0 && !rf.reloading) { done = f; carryAtDone = an.rlShown.carry; } else during = rf.shotsFired - shots0; }
        const mag1 = rf.mag ?? -1; let firedAt = -1, carryAtShot = -1;
        for (let f = 0; f < 30 && firedAt < 0; f++) { W.cursorAt([27.5, 1.2, 10.0]); const n = rf.shotsFired; I.clicked |= 1; const k = an.rlShown.carry; W.step(1 / 60); if (rf.shotsFired > n) { firedAt = f; carryAtShot = Math.max(k, an.rlShown.carry); } }
        return { began, done, carryAtDone, top, mag0, mag1, shown, during, firedAt, carryAtShot }; };
      const b = await game();
      line(b.began && b.done > 60 && b.mag1 > b.mag0 && b.carryAtDone === 0 && b.top > 0.99 && b.shown && b.during === 0 && b.firedAt >= 0 && b.carryAtShot === 0,
        '(11) in a game, the player\'s rifle: the reload takes its time and ends with the rounds in; no click through it fires; the gun comes to the carry, and is on its carriage on the frame the rounds are in and on the frame the next round leaves',
        `done at frame ${b.done} · magazine ${b.mag0} → ${b.mag1} · rounds fired through the change ${b.during} · the next round ${b.firedAt} frames after · the carry at its most ${b.top.toFixed(2)}, ${b.carryAtDone} on the last frame, ${b.carryAtShot} at the shot`); }
  } finally { KN.knobsBack(knobs0); }
  return all;
}

if (import.meta.main) process.exit((await gunhandReloadCheck()) ? 0 : 1);
