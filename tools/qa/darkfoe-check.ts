// AN ENEMY ON ALERT DOES NOT SIT BLIND IN THE DARK — the check behind src/game/darkfoe.ts (DARKFOE.on: on in the page, ?nodarkfoe; off on the frozen stand,
// TPS_DARKFOE=1).
//   bun run tools/qa/darkfoe-check.ts        (also `probes.ts darkfoe`; run it on BOTH stands: plain, and with TPS_PROFILE=shipped in front)
// Mousehole's room clear at night (where the maintainer's tape was played: nine defenders, weapon lights dealt off, some rooms lit and some not), Sam out of
// everybody's sight, the switch and the numbers thrown by hand. "Gunfire has reached him" is written into a man's witness record in (a)-(c), (e), (g) — no
// event, so dig-in and the squads sleep and the lamp rule is read alone — and is REAL gunfire (eight 'shot' events where Sam stands) in (d), where they wake:
//   (a) nobody has been told: with the rule on, six seconds in, no man knows, no lamp burns and nobody has left his mark — though some of them are blind;
//   (c) the lamps are the word: ONE man told, the rest not — a man who has only the word has a burning lamp of his side before his eyes (inside his cone, a
//       clear line), at least one such man lights his own, and the word never becomes knowing for good;
// exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: every light here is guards.ts's direct-light estimate (no bounce — it errs dark). Nobody has seen any of this on a GPU.
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
const ROOT = resolve(import.meta.dir, '../..');

function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

export async function darkfoeCheck(): Promise<boolean> {
  const { standUp } = await import('./headless.ts');
  const { DARKFOE, DARKFOE_STATS, darkFoeOf, darkFoeSnapshot, darkFoeRestore, darkFoeRuled, darkFoeKept, darkFoeBlind, darkFoeSpot } = await import(`${ROOT}/src/game/darkfoe.ts`);
  const { lightEstimateAt, GUARD_EYE } = await import(`${ROOT}/src/game/guards.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
  const DT = 1 / 60, rnd0 = Math.random, was = { ...DARKFOE }; let all = true;
  const BREAK = process.env.DARKFOE_BREAK === '1';
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== darkfoe: an enemy on alert does not sit blind in the dark ===');
  const levelOf = (file: string) => loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
  // (`dealt`: squaddeal.ts held off or on for this floor through the stand's own word, set before the room clear starts; left out = as the stand has it)
  const stand = async (seed: number, file = 'mousehole.json', deal = 1, dealt?: boolean) => { Math.random = seeded(seed); const sd0 = process.env.TPS_SQUADDEAL; if (dealt !== undefined) process.env.TPS_SQUADDEAL = dealt ? '1' : '0';
    const W: any = await quiet(() => standUp({ uploadEvery: 4, level: levelOf(file) })).finally(() => { if (sd0 === undefined) delete process.env.TPS_SQUADDEAL; else process.env.TPS_SQUADDEAL = sd0; }); const g = W.game;
    g.playerInvisible = true; g.godMode = true;
    const why = await quiet(async () => g.roomClear.start(deal)); if (why) throw new Error(`the room clear did not start on ${file}: ${why}`);
    await quiet(async () => { for (let i = 0; i < 90; i++) W.step(DT); }); return W; };
  const step = (W: any, n = 1) => quiet(async () => { for (let i = 0; i < n; i++) W.step(DT); });
  const foes = (g: any): any[] => g.guards.filter((x: any) => x.unit && x.team === 'hostile' && x.state !== 'dead');
  const tell = (gd: any) => { gd.witness.heard.shots = Math.max(1, gd.witness.heard.shots); };
  const shots = async (W: any) => { const g = W.game; for (let k = 0; k < 8; k++) { g.events.push({ kind: 'shot', pos: [...g.player.char.pos], time: g.time, loud: true, level: 2 }); await step(W, 6); } };   // (real gunfire where Sam stands: a 'shot' counts as one from level 1)
  const burns = (gd: any) => !!gd.flashlight.enabled;
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  // the check's own readings, off the estimate (not the module's functions)
  const floor = (g: any, x: number, z: number, skip: (o: number) => boolean) => g.visibilityOf(lightEstimateAt(g, [x, 1.2, z], skip, true));
  const ownBlind = (g: any, gd: any) => { const c = gd.char, eye = [c.pos[0], 1.5, c.pos[2]], skip = (o: number) => o === c.id; let n = 0, lit = 0;
    for (const off of [0, -DARKFOE.fanRad, DARKFOE.fanRad]) { const fx = Math.sin(c.aimYaw + off), fz = Math.cos(c.aimYaw + off);
      for (let r = DARKFOE.stepM; r <= DARKFOE.lookM + 1e-6; r += DARKFOE.stepM) { const x = eye[0] + fx * r, z = eye[2] + fz * r; if (g.col.segmentBlocked(eye, [x, 1.5, z])) break; n++; if (floor(g, x, z, skip) >= DARKFOE.litAt) lit++; } }
    return n ? lit / n < DARKFOE.seeShare : floor(g, c.pos[0], c.pos[2], skip) < DARKFOE.litAt; };
  const ownSees = (g: any, p: number[]) => { const place = () => true; if (floor(g, p[0], p[2], place) >= DARKFOE.litAt) return true; const eye = [p[0], 1.5, p[2]];
    for (let ring = 1; ring <= 2; ring++) { const r = DARKFOE.edgeM * ring / 2; for (let k = 0; k < 8; k++) { const a = (k + (ring & 1) * 0.5) * Math.PI / 4, qx = p[0] + Math.sin(a) * r, qz = p[2] + Math.cos(a) * r;
      if (!g.col.segmentBlocked(eye, [qx, 1.5, qz]) && floor(g, qx, qz, place) >= DARKFOE.litAt) return true; } }
    return false; };

  try {
    // (a) nobody has been told
    { const W = await stand(31), g = W.game; Object.assign(DARKFOE, was, { on: true, carry: 1 });
      await step(W, 360); const men = foes(g), blind = men.filter(gd => ownBlind(g, gd)).length;
      const knows = men.filter(gd => darkFoeOf(gd)?.knows).length, lit = men.filter(burns).length, walked = men.filter(gd => (darkFoeOf(gd)?.moves ?? 0) > 0).length;
      line(men.length >= 5 && blind >= 2 && knows === 0 && lit === 0 && walked === 0, 'nobody has been told: six seconds on, no man knows, no lamp burns and nobody has left his mark — though some stand blind',
        `${men.length} defenders · ${blind} with a dark view by the check's own reading · know ${knows} · lamps ${lit} · sent ${walked}`); }

    // (c) the lamps are the word
    { const W = await stand(33), g = W.game; Object.assign(DARKFOE, was, { on: true, carry: 1 }); const men = foes(g);
      const los = (a: any, b: any) => flat(a.char.pos, b.char.pos) <= DARKFOE.lampM + 0.5 && !g.col.segmentBlocked([a.char.pos[0], 1.5, a.char.pos[2]], [b.char.pos[0], 1.5, b.char.pos[2]]);
      const inCone = (a: any, b: any) => { let d = Math.atan2(b.char.pos[0] - a.char.pos[0], b.char.pos[2] - a.char.pos[2]) - a.char.aimYaw; d = Math.atan2(Math.sin(d), Math.cos(d)); return Math.abs(d) <= (GUARD_EYE.halfDeg + 8) * Math.PI / 180; };
      const first = men.filter(gd => ownBlind(g, gd)).sort((a, b) => men.filter(o => o !== b && los(o, b) && inCone(o, b)).length - men.filter(o => o !== a && los(o, a) && inCone(o, a)).length)[0];
      if (!first) line(false, 'the lamps are the word: no blind man to tell on this staging'); else { tell(first); const last = new Map<any, number>(), had = new Set<any>(), litToo = new Set<any>(); let unseen = 0, latched = 0;
      for (let i = 0; i < 60 * 15; i++) { await step(W);
        for (const gd of men) { if (gd === first || gd.state === 'dead') continue; if (men.some(o => o !== gd && burns(o) && darkFoeOf(o)?.knows && los(gd, o) && inCone(gd, o))) last.set(gd, i);   // (the lamp of a man who KNOWS: a lamp lit on the word passes none)
          const S = darkFoeOf(gd), own = gd.state !== 'patrol' || gd.witness.heard.shots + gd.witness.heard.kick + gd.witness.heard.bang > 0 || gd.witness.alertedBy !== null;   // (he has it for himself: not this line's)
          if (own) continue; if (burns(gd)) litToo.add(gd);
          if (S?.knows) latched++;
          if (S?.word) { had.add(gd); if (i - (last.get(gd) ?? -1e9) > 30) unseen++; } } }
      const names = (set: Set<any>) => [...set].map(gd => gd.callsign).join(', ') || 'nobody';
      line(burns(first) && had.size >= 1 && litToo.size >= 1 && unseen === 0 && latched === 0, 'one man told: his lamp comes on; a man with only the word has a burning lamp before his eyes, lights his own, and never knows for good by it',
        `${first.callsign} told, lamp ${burns(first) ? 'on' : 'OFF'} · had the word: ${names(had)} · lit their own: ${names(litToo)} · frames with the word and no lamp before him: ${unseen} · frames knowing for good: ${latched}`);
      } }

  } finally { Object.assign(DARKFOE, was); Math.random = rnd0; }
  console.log(`=== darkfoe: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await darkfoeCheck()) ? 0 : 1);
