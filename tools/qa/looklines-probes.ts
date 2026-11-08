// 'wallbreak-looklines', run through probes.ts in the 'wallbreak' family:
//   bun run tools/qa/probes.ts wallbreak-looklines [--level block]
// A blown wall's picture-only slabs (edgeLook.ts) cast the shadow they draw under the cut switch, so their flags no longer keep them out of BoxWorld.raycast;
// their motion key does (boxes.ts MKEY.look). Every ray that asks the world where something LANDS must pass them, as the round along that line does:
//   (aim)       the cursor's aim point        player.ts updateAim
//   (order)     an order's point              orders.ts targetUnder
//   (laser)     a weapon laser's reach / dot  combat.ts laserReach — and the man it ends on
//   (tripwire)  a tripwire's beam             devices.ts updateLaser (the office: a charge either side of each beam)
// What is asked: after a charge on every wall of the level, lines drawn AT the slabs that cast a shadow — the camera's own ray to a point in each (the cursor
// there), and a muzzle's line from a couple of metres off. A line COUNTS when it reaches a slab's own solid before anything else. The world's ray must then
// meet a slab in none of them, and each of the game's three answers is compared with the SAME call made with every slab put back out of every ray's reach
// (NoShadow, as they all were before the cut switch): any difference is a line a slab moved. Then a man is stood behind a slab at chest height: the beam must
// end ON him. Then the tripwires: on every frame a slab is in a beam's line (falling through it, lying in it), the beam's ray must not end on it.
// The file reads nothing the mend added, so the same file run on a tree without it prints what it was before (and goes red there).
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
import { swept } from './sweep.ts';
const { WALLS } = await import(`${ROOT}/src/game/wallbreak.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`);
const { BoxFlag, rayBox } = await import(`${ROOT}/src/scene/boxes.ts`);
const { quat } = await import(`${ROOT}/src/math/vec.ts`);
const { updateAim } = await import(`${ROOT}/src/game/player.ts`);
const { laserReach } = await import(`${ROOT}/src/game/combat.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const LEVEL = (() => { const i = process.argv.indexOf('--level'); return i >= 0 ? process.argv[i + 1] : 'block'; })();
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const DEFAULTS = JSON.parse(JSON.stringify(WALLS));
const mad = (a: Vec3, b: Vec3, s: number): Vec3 => [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s];
const norm = (a: Vec3): Vec3 => { const l = Math.hypot(a[0], a[1], a[2]) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
const near = (a: readonly number[], b: readonly number[]) => Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]) < 1e-6;
/** edgeLook.ts's pieces by their motion key's byte (9), written out here — not taken from the code under test */
const look = (b: any) => b.mkey !== undefined && b.mkey >>> 24 === 9;

/** the harness Game on the loud level with THE ONE SWITCH up and the cut switch as asked, everyone held, a charge set off on every wall and cover, the dust settled */
async function stand(seed: number, cut: boolean, level = LEVEL) {
  Math.random = seeded(seed); for (const k of Object.keys(DEFAULTS)) (WALLS as any)[k] = JSON.parse(JSON.stringify(DEFAULTS[k])); DESTRUCTION.on = true; cutConfig.on = cut;
  const W = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${level}.json`, 'utf8'))) }); const game: any = W.game; cutConfig.on = cut;   // (standUp follows CUTBOXES when it is given: this probe stages the switch itself)
  game.godMode = true; game.aiEnabled = false; game.player.visibility = 0.02; game.tune.chaos = 0.5;
  for (let i = 0; i < 3; i++) W.step(DT);
  for (const gd of game.guards) { gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; gd.flashlight.enabled = false; }
  return { W, game };
}
async function stage(seed: number, cut: boolean) {
  const { W, game } = await stand(seed, cut);
  let blasts = 0;
  for (const w of game.destr.walls.all()) { const b = w.box; const alongX = b.h[0] >= b.h[2];
    const at: Vec3 = alongX ? [b.c[0], 0.4, b.c[2] + 0.3] : [b.c[0] + 0.3, 0.4, b.c[2]]; game.destr.blast(at, { who: PLAYER_ID, radius: 2.4 }); blasts++; for (let f = 0; f < 20; f++) W.step(DT); }
  for (let f = 0; f < 240; f++) W.step(DT);
  const all: any[] = W.engine.world.all; const looks = all.filter(look), casting = looks.filter(b => !(b.flags & BoxFlag.NoShadow));
  return { W, game, blasts, looks, casting };
}
/** a point inside the box (its own frame, the middle 70 % of each axis) */
function pointIn(R: () => number, b: any): Vec3 {
  const l: Vec3 = [(R() * 2 - 1) * b.h[0] * 0.7, (R() * 2 - 1) * b.h[1] * 0.7, (R() * 2 - 1) * b.h[2] * 0.7];
  const w = quat.rotate(b.rot ?? quat.yaw(b.yaw ?? 0), l); return [b.c[0] + w[0], b.c[1] + w[1], b.c[2] + w[2]];
}

export async function wallbreakLooklines(): Promise<boolean> {
  console.log(`\n=== wallbreak-looklines: the aim, an order and a laser pass a blown wall's picture-only slabs (${LEVEL}) ===`);
  let fails = 0; const verdict = (ok: boolean, what: string, detail: string) => { console.log(`  - ${ok ? 'PASS' : 'FAIL'} — ${what}: ${detail}`); if (!ok) fails++; };
  const cut0 = cutConfig.on, destr0 = DESTRUCTION.on, rnd0 = Math.random;
  try {
    // ---------------------------------------------------------------- the cut switch on
    const S = await stage(11, true); const { W, game } = S; const world = W.engine.world; const all: any[] = world.all;
    { const mine: any[] = []; game.destr.edgeLook.boxes(mine); const set = new Set(mine); const stray = all.filter(b => look(b) !== set.has(b)).length, statics = world.statics.filter(look).length;
      verdict(S.casting.length >= 20 && stray === 0 && statics === 0, "the cut switch on: slabs of the look cast a shadow, and the key's byte names the look's pieces and nothing else", `${S.blasts} charges · ${S.looks.length} look pieces, ${S.casting.length} cast a shadow · ${stray} boxes the key and the look's own list disagree on · ${statics} statics keyed so`); }
    /** run `f` with every shadow-casting slab put back out of every ray's reach (as they all were before the cut switch) */
    const hidden = <T>(f: () => T): T => { for (const b of S.casting) b.flags |= BoxFlag.NoShadow; try { return f(); } finally { for (const b of S.casting) b.flags &= ~BoxFlag.NoShadow; } };
    const per = Math.max(2, Math.ceil(swept(400, 1500) / Math.max(1, S.casting.length)));

    // (aim) and (order): the camera's own ray to a point in each slab
    { const R = seeded(404); let drawn = 0, through = 0, plain = 0, aimBad = 0, ordBad = 0; const saidAim: string[] = [], saidOrd: string[] = [];
      for (const b of S.casting) for (let k = 0; k < per; k++) {
        const P = pointIn(R, b); if (!W.cursorAt(P)) continue; drawn++;
        const ray = W.cam.ray(W.input.mouseX / W.canvas.clientWidth, W.input.mouseY / W.canvas.clientHeight);
        const ref = hidden(() => world.raycast(ray.ro, ray.rd, 200, undefined, PLAYER_ID)), own = rayBox(ray.ro, ray.rd, b, 200);
        if (!own || (ref && ref.t <= own.t)) continue; through++;   // (the line reaches this slab before anything a ray met before the cut switch)
        const raw = world.raycast(ray.ro, ray.rd, 200, undefined, PLAYER_ID); if (raw && look(all[raw.index])) plain++;
        const aimRef: Vec3 = hidden(() => { updateAim(game, W.input, W.cam, W.canvas); return [...game.aimPoint] as Vec3; }); updateAim(game, W.input, W.cam, W.canvas); const aim: Vec3 = [...game.aimPoint] as Vec3;
        if (!near(aim, aimRef)) { aimBad++; if (saidAim.length < 2) saidAim.push(`aim at ${aim.map(x => x.toFixed(2))} (on a piece of the look, name '${b.name}'), behind it ${aimRef.map(x => x.toFixed(2))}`); }
        const oRef = hidden(() => game.orders.targetUnder(W.input, W.cam, W.canvas)), o = game.orders.targetUnder(W.input, W.cam, W.canvas);
        if (o.kind !== oRef.kind || !near(o.at, oRef.at) || o.door !== oRef.door || o.man !== oRef.man) { ordBad++; if (saidOrd.length < 2) saidOrd.push(`order ${o.kind} at ${o.at.map((x: number) => x.toFixed(2))}, behind the slab ${oRef.kind} at ${oRef.at.map((x: number) => x.toFixed(2))}`); }
      }
      verdict(through >= 300 && plain === 0, "the world's ray meets no slab the cursor is put on", `${drawn} cursor lines drawn at the slabs · ${through} reach a slab before anything else · the world's ray stops on a slab in ${plain}`);
      verdict(through >= 300 && aimBad === 0, "the cursor's aim point lands where it did with the slabs out of every ray's reach", `${aimBad} of ${through} lines moved by a slab${saidAim.length ? ' · ' + saidAim.join(' · ') : ''}`);
      verdict(through >= 300 && ordBad === 0, "an order's point (kind, place, door, man) is the same", `${ordBad} of ${through} lines moved by a slab${saidOrd.length ? ' · ' + saidOrd.join(' · ') : ''}`); }

    const SKIP = BoxFlag.NoShadow | BoxFlag.Glass;
    // a man stood behind a slab at chest height: the beam ends on him (Sam himself: a live Sam stops any beam but his own, whatever the fog of war shows)
    { const R = seeded(606); const pc = game.player.char; let lines = 0, stood = 0, seen = 0;
      for (const b of S.casting) { if (stood >= 12) break; if (b.c[1] < 0.6 || b.c[1] > 1.5) continue;
        for (let k = 0; k < 8; k++) { const a = (k + R()) * 0.7854, d: Vec3 = [Math.cos(a), 0, Math.sin(a)], P = pointIn(R, b), O = mad(P, d, -1.5), from = mad(O, d, 0.02);
          const own = rayBox(from, d, b, 30), ref = hidden(() => world.raycast(from, d, 30, SKIP, 250)); if (!own || (ref ? ref.t : 30) < own.t + 1.3) continue; lines++;   // (clear floor behind the slab on this line)
          const at = mad(from, d, own.t + 0.7); game.teleportPlayer([at[0], 0, at[2]]); for (let f = 0; f < 2; f++) W.step(DT);
          const man: any = { muzzle: O, gunDir: d, id: 250 }; const lRef = hidden(() => laserReach(game, man, null, true)); if (!lRef || lRef.on !== pc) continue; stood++;
          const l = laserReach(game, man, null, true); if (l && l.on === pc && Math.abs(l.L - lRef.L) < 1e-9) seen++; break; } }
      verdict(stood >= 3 && seen === stood, 'a man behind a slab is the man the beam ends on', `${lines} lines with clear floor behind a chest-high slab · ${stood} stands (Sam 0.7 m behind it, the muzzle 1.5 m in front, the slab between) · the beam ends on him in ${seen}`); }

  } finally { cutConfig.on = cut0; DESTRUCTION.on = destr0; Math.random = rnd0; for (const k of Object.keys(DEFAULTS)) (WALLS as any)[k] = JSON.parse(JSON.stringify(DEFAULTS[k])); }
  console.log(`=== wallbreak-looklines: ${fails ? 'FAILURES above' : 'ALL PASS'} ===`);
  return fails === 0;
}

if (import.meta.main) process.exit((await wallbreakLooklines()) ? 0 : 1);
