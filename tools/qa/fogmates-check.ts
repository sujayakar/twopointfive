// A TEAMMATE SEES WHAT THE PICTURE SHOWS (src/game/fogofwar.ts FOG.mates; src/game/guards.ts eyeThrough). The maintainer, 10-25, asked whether the men's own
// eyes should pass thin things as the fog's lines do: "i think teammates should see whatever we see".
//   bun run tools/qa/probes.ts fog-mates        (family fog; FOGMATES_BREAK=1 = the teammate is stood with his back to the man in every staging, and guards.ts
//                                                 is read with the mask dropped from the look it hands the module: every line must go red)
//   bar       The Pass, the squad staged: a teammate and a defender either side of a thing the level marked thin, facing each other (a pair whose plain
//             line, glass alone passed, is stopped): his own sight test has the man and his bare line to the man's chest is clear; what his line is handed
//             is glass and the marks, a defender's glass alone; a wall between them is a wall
//   agrees    every thin mark of The Pass, pairs put across it at sixteen bearings and nine spacings: a teammate's own test and the fog's own line from
//             his head say the same of every pair
//   stood     the game's own frames, the two kept on those marks: the room fight's read has the teammate notice the man and take him as his target
//             inside half a second; the defender never takes him
//   written   the code as written: a man's look goes to the module with his team's mask, and nothing else answers it (the game has no line of sight of
//             its own to a man: rust/sim/src/sight.rs)
// FOG.mates and FOG.thin are settled on: the check asks the page's side alone and writes neither. Until 2026-11-07 every line was also asked with each
// row off ('without the row his own sight test does not have the man', the pairs the row gains, 'the row never takes a man from him', 'with the thin row
// off it gives nothing', the defender's eye 'the same with and without'), `played` held twenty seconds of the squad with the rows off against on by
// digest, and `switch` asked the address, the journal and the two stands: comparisons with the off run and the old stand's words, with no keeper.
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

export async function fogMatesCheck(o: { verbose?: boolean } = {}): Promise<boolean> {
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
  const { canSeeChar, eyeThrough, eyeBlocked } = await import(`${ROOT}/src/game/guards.ts`);
  const { isHostile } = await import(`${ROOT}/src/game/teams.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const BREAK = process.env.FOGMATES_BREAK === '1';
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (o.verbose) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  let fails = 0, checks = 0;
  const verdict = (ok: boolean, label: string, detail: string) => { checks++; if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log('\n=== fog-mates: a teammate\'s own eye passes the thin things the fog\'s lines pass ===');
  const DT = 1 / 60;
  /** The Pass stood up with the squad of four staged on it */
  async function rig(seed: number) {
    const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'pass.json'), 'utf8')));
    const W: any = await quiet(() => standUp({ level: lvl, seed }));
    const g = W.game as any; await quiet(() => startFromQuery(g, '?levelFile=pass.json&roomclear=0&squad4'));
    return { W, g };
  }
  /** a man stood at (x, z) looking at (tx, tz), his bones posed there */
  /** (FOGMATES_BREAK: the teammate is turned to face away from what he is put to look at) */
  const putMate = (gd: any, x: number, z: number, tx: number, tz: number) => BREAK ? put(gd, x, z, 2 * x - tx, 2 * z - tz) : put(gd, x, z, tx, tz);
  const put = (gd: any, x: number, z: number, tx: number, tz: number) => { const c = gd.char; c.pos = [x, 0, z]; c.vel = [0, 0, 0]; c.bodyYaw = c.aimYaw = Math.atan2(tx - x, tz - z); c.aimPitch = 0; c.update(0); };
  const sees = (g: any, a: any, b: any) => canSeeChar(g, a, b.char, false) as { visible: boolean; inCone: boolean; dist: number };
  {
    { // ---- bar and agrees: stagings across every thin mark of the level
      const A = await rig(7), g = A.g, col = g.col, PF = g.fogOfWar; for (let i = 0; i < 3; i++) A.W.step(DT);
      g.aiEnabled = false; for (const x of g.guards) x.hold = true;
      const mate = (g.guards as any[]).find(x => x.team === 'operative' && x.char.alive), foe = (g.guards as any[]).find(x => x.char.alive && mate && isHostile(x.team, 'operative') && isHostile('operative', x.team));
      const statics: any[] = g.engine.world.statics, marks = statics.filter(b => b.flags & BoxFlag.Thin);
      if (!mate || !foe || !marks.length) verdict(false, 'bar — The Pass, the squad staged: a teammate, a man hostile to him, and the level\'s thin marks', `teammate ${mate?.callsign} · hostile ${foe?.callsign} · marks ${marks.length}`);
      else {
        const home = [mate, foe].map(x => ({ p: x.char.pos.slice(), yaw: x.char.bodyYaw, aim: x.char.aimYaw }));
        const chest = (c: any) => c.bones.chest ?? [c.pos[0], c.pos[1] + 1.1, c.pos[2]], head = (c: any) => c.bones.head ?? [c.pos[0], c.pos[1] + 1.65, c.pos[2]];
        /** the fog's own line from the teammate's head to the man, the thin row on, a field all round (the cone is not on trial) */
        const fogLine = () => { const e = { id: mate.char.id, pos: head(mate.char), look: [0, 0, 1], at: head(mate.char), fov: 360 }; return PF.lineToMan(e, foe, chest(foe.char)) as boolean; };
        let pairs = 0, differ = 0, inCone = 0, across = 0; let first: { m: number[]; f: number[]; at: number[] } | null = null;
        for (const t of marks) for (let k = 0; k < 16; k++) for (const r1 of [0.8, 1.5, 3]) for (const r2 of [0.8, 1.5, 3]) {
          const a = k * Math.PI / 8, mx = t.c[0] + Math.cos(a) * r1, mz = t.c[2] + Math.sin(a) * r1, fx = t.c[0] - Math.cos(a) * r2, fz = t.c[2] - Math.sin(a) * r2;
          if (col.nav.isBlocked(mx, mz) || col.nav.isBlocked(fx, fz)) continue;
          putMate(mate, mx, mz, fx, fz); put(foe, fx, fz, mx, mz); pairs++;
          const on = sees(g, mate, foe), fog = fogLine();
          if (!on.inCone) continue; inCone++;
          // (a pair the marks alone hide: he has the man, and the plain line from his head to the man's chest, glass alone passed, is stopped)
          if (on.visible && col.segmentBlocked(head(mate.char), chest(foe.char), 0.02, BoxFlag.Glass)) { across++; first ??= { m: [mx, mz], f: [fx, fz], at: [t.c[0], t.c[2]] }; }
          if (on.visible !== fog) differ++;
        }
        { // bar: the first such pair, every way round — and a wall
          const F = first as { m: number[]; f: number[]; at: number[] } | null; let on = false, through = [0, 0], wall = true, bare = true;
          if (F) { putMate(mate, F.m[0], F.m[1], F.f[0], F.f[1]); put(foe, F.f[0], F.f[1], F.m[0], F.m[1]);
            on = sees(g, mate, foe).visible; bare = eyeBlocked(g, mate, head(mate.char), chest(foe.char)); through = [eyeThrough(mate), eyeThrough(foe)]; }
          // a wall: the first pair of free cells two metres apart either side of something solid that is no thin mark (the plain line stopped with the marks passed too)
          let W: number[] | null = null; const nav = col.nav;
          for (let z = 2; z < nav.H - 2 && !W; z++) for (let x = 2; x < nav.W - 6 && !W; x++) { const ax = (x + 0.5) * nav.cell, az = (z + 0.5) * nav.cell, bx = ax + 2;
            if (nav.isBlocked(ax, az) || nav.isBlocked(bx, az)) continue;
            if (col.segmentBlocked([ax, 1.65, az], [bx, 1.1, az], 0.02, BoxFlag.Glass | BoxFlag.Thin) && col.segmentBlocked([ax, 1.65, az], [bx, 1.6, az], 0.02, BoxFlag.Glass | BoxFlag.Thin) && col.segmentBlocked([ax, 1.0, az], [bx, 1.0, az], 0.02, BoxFlag.Glass | BoxFlag.Thin)) W = [ax, az, bx]; }
          if (W) { put(mate, W[0], W[1], W[2], W[1]); put(foe, W[2], W[1], W[0], W[1]); wall = sees(g, mate, foe).visible; }
          const G = BoxFlag.Glass, GT = BoxFlag.Glass | BoxFlag.Thin;
          verdict(!!F && on && !bare && through[0] === GT && through[1] === G && !!W && !wall,
            'bar — The Pass, a teammate and a defender either side of a thing the level marked thin, facing each other: his own sight test has the man and his bare line to the man\'s chest (the room fight\'s turn-to lines) is clear; what his line is handed is glass and the marks, a defender\'s glass alone; across a wall he has nobody',
            F ? `${mate.callsign} at (${F.m[0].toFixed(2)}, ${F.m[1].toFixed(2)}), ${foe.callsign} at (${F.f[0].toFixed(2)}, ${F.f[1].toFixed(2)}), the mark at (${F.at[0].toFixed(2)}, ${F.at[1].toFixed(2)}): he has the man ${on} · his bare line stopped ${bare} · handed: teammate ${through[0]}, defender ${through[1]} (glass ${G}, glass and thin ${GT}) · across a wall${W ? ` at (${W[0].toFixed(1)}, ${W[1].toFixed(1)})` : ': none found'}: ${wall}` : `no pair found that the marks alone hide (pairs tried: ${pairs})`);
        }
        verdict(marks.length >= 6 && pairs >= 300 && inCone >= 300 && across >= 20 && differ === 0,
          'agrees — every thin mark of The Pass, a teammate and a defender put across it at sixteen bearings and nine spacings wherever both can stand: the teammate\'s own test and the fog\'s own line from his head say the same of every pair',
          `marks ${marks.length} · pairs ${pairs}, in his cone ${inCone} · pairs the marks alone would hide ${across} · his test against the fog's line: ${differ} differ`);
        { // stood: the game's own senses, the pair kept on the first such marks
          const F = first as { m: number[]; f: number[]; at: number[] } | null; const row: string[] = []; let ok = !!F;
          const op = () => (g.fight?.ops as any[] | undefined)?.find(o => o.gd === mate), dop = () => (g.fight?.defs as any[] | undefined)?.find(d => d.gd === foe);
          const stand = () => { g.aiEnabled = true; let tgt = 0, foeTook = 0;
            for (let i = 0; i < 30 && F; i++) { putMate(mate, F.m[0], F.m[1], F.f[0], F.f[1]); put(foe, F.f[0], F.f[1], F.m[0], F.m[1]); A.W.step(DT); if (op()?.tgt === foe) tgt++; if (foe.foe === mate || dop()?.tgt === mate) foeTook++; }
            g.aiEnabled = false; return { tgt, meter: (op()?.meters.get(foe)?.m ?? 0) as number, foeTook }; };
          const on = stand(); ok &&= !!op() && on.tgt > 0 && on.meter > 0.2 && on.foeTook === 0;
          row.push(`in the room fight: ${!!op()}`, `his target ${on.tgt} of 30 frames, his meter on the man ${on.meter.toFixed(2)}; the defender takes him ${on.foeTook}`);
          verdict(ok, 'stood — the game\'s own frames, the teammate and the defender kept either side of that mark for half a second: the room fight\'s own read has the teammate notice the man (his meter on him rises) and take him as his target; the defender never takes the teammate', F ? row.join(' · ') : 'no pair to stand');
        }
        [mate, foe].forEach((x, i) => { const c = x.char; c.pos = home[i].p; c.bodyYaw = home[i].yaw; c.aimYaw = home[i].aim; c.update(0); });
      }
    }
    { // ---- the code as written
      const fogText = readFileSync(resolve(ROOT, 'src/game/fogofwar.ts'), 'utf8'), moduleOn = /\n  mates: true,\n/.test(fogText);
      // (the red path reads the file with the mask taken off the one call that hands a look to the module: `passed` below must then miss it)
      const gtRead = readFileSync(resolve(ROOT, 'src/game/guards.ts'), 'utf8'), HANDS = 'g.col.eyeContact(E, tgt, tgt2, blind || (!isPlayer && !pc.alive), smoked, through);';
      const gt = BREAK ? gtRead.replace(HANDS, HANDS.replace(', through);', ');')) : gtRead;
      const helper = /export function teamThrough\(\): number \{ return FOG\.mates \? fogThrough\(\) : BoxFlag\.Glass; \}\nexport function eyeThrough\(gd: Guard\): number \{ return gd\.team === 'operative' \? teamThrough\(\) : BoxFlag\.Glass; \}/.test(gt) && /export function fogThrough\(\): number \{ return FOG\.thin \? BoxFlag\.Glass \| BoxFlag\.Thin : BoxFlag\.Glass; \}/.test(fogText);
      const pt = readFileSync(resolve(ROOT, 'src/game/picture.ts'), 'utf8'), sam = /thr = teamThrough\(\);/.test(pt) && (pt.match(/, 0\.02, thr\)/g) ?? []).length === 2;
      const rf = readFileSync(resolve(ROOT, 'src/game/roomfight.ts'), 'utf8'), bareHelper = /export function eyeBlocked\(g: Game, gd: Guard, a: Vec3, b: Vec3\): boolean \{ return g\.col\.segmentBlocked\(a, b, 0\.02, eyeThrough\(gd\)\); \}/.test(gt), fights = (rf.match(/eyeBlocked\(g, O\.gd, /g) ?? []).length, lead = (gt.match(/eyeBlocked\(g, gd, eye, /g) ?? []).length;
      const asks = (gt.match(/eyeThrough\(gd\)/g) ?? []).length;
      // a function of guards.ts as code (its line comments taken out), and every call of `what` in such a text: each call's own argument list, by its own brackets (a
      // `, through)` further along the line is some other call's). Text, so with text's limits: a bracket inside a string of the arguments would cut one short
      const body = (name: string): string => { const i = gt.indexOf(`function ${name}(`), j = i < 0 ? -1 : gt.indexOf('\n}\n', i); return j < 0 ? '' : gt.slice(i, j).split('\n').slice(1).map(l => l.replace(/\/\/.*$/, '')).join('\n'); };
      const calls = (text: string, what: RegExp): string[] => { const out: string[] = [];
        for (const m of text.matchAll(what)) { let d = 0, j = m.index + m[0].length - 1; const from = j + 1; for (; j < text.length; j++) { if (text[j] === '(') d++; else if (text[j] === ')' && --d === 0) break; } out.push(text.slice(from, j)); }
        return out; };
      const masked = (c: string[]) => c.filter(a => /, through$/.test(a)).length;
      // canSeeChar's looks: every eyeContact it calls, two, ends on the mask; and inside eyeContact the mask goes on to what answers the look, the module
      // (<anything>.eyeContact), at the one call there is. NOTHING ELSE ANSWERS A LOOK: eyeContact asks no line of the box world itself (the game's own
      // two lines of sight went with the senses' TypeScript). Whether the module then keeps to the mask is not asked here (sense-ident asks it thin masks)
      const looks = calls(body('canSeeChar'), /(?<![.\w])eyeContact\(/g), on = calls(body('eyeContact'), /\.eyeContact\(/g), own = calls(body('eyeContact'), /segmentBlocked\w*\(/g).length;
      const handed = masked(looks), passed = masked(on), alone = on.length === 1 && own === 0;
      verdict(moduleOn && helper && bareHelper && sam && alone && looks.length === 2 && handed === 2 && passed === on.length && asks === 2 && lead === 2 && fights === 6,
        'written — on in the module; a man\'s line is handed the marks by the one helper, as written: in canSeeChar\'s two looks (and guards.ts hands it on to the module, which alone answers a look: the game asks no line of its own to a man), and through the bare line in the lost-behind-something test and the room fight\'s six lines from a teammate\'s eye to a man; the player\'s own line for the squad\'s net takes the team\'s mask',
        `the module's on: ${moduleOn} · the helper as written ${helper}, canSeeChar's looks handed it ${handed} of ${looks.length}, passed on inside eyeContact at ${passed} of ${on.length} calls to the module, lines of its own there ${own}, asked ${asks} times, the bare line as written ${bareHelper}: the lead test's ${lead}, the room fight's ${fights}, the player's line for the net ${sam}`);
    }
  }
  console.log(`=== fog-mates: ${fails === 0 ? 'ALL PASS' : `${fails} of ${checks} FAILED`} ===`);
  return fails === 0;
}
