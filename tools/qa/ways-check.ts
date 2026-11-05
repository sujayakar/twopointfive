// HOW A LEVEL IS WALKED: the check behind scene/ways.ts levelWays and its reader, render/wear.ts buildWearMap's `walked` (the look's page; ?walked=0 at boot =
// the traffic map as every other page has it; a picture's only: no game table, nothing for a recording to carry).
//   bun run tools/qa/probes.ts ways        (family `render`)        WAYS_BREAK=1 = the reader takes no ways and the same lines are asked: must FAIL
// The maintainer, 11-04: "wear, paths in the ground, different materials can all tell a story for how people walk through the level, its logic, leading
// the player for game play".
// WHAT THIS CAN AND CANNOT SAY. The map of ways is plain arithmetic on the walk grid and is asked outright, on a level of the game and on its own names. What
// a floor then SHOWS of it is the shaders' (look.wgsl's USE AND DIRT, lookPiece, markings.wgsl markSoft), held here by their text only. Never seen on a GPU.
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function waysCheck(): Promise<boolean> {
  const BRK = process.env.WAYS_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== ways: how a level is walked${BRK ? ' (the reader takes none: must fail)' : ''} ===`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`), { StaticCollision, NavGrid } = await import(`${ROOT}/src/scene/collision.ts`);
  const { levelWays } = await import(`${ROOT}/src/scene/ways.ts`), { buildWearMap, WEAR_RES } = await import(`${ROOT}/src/render/wear.ts`), { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
  const open = (f: string) => { const level = loadLevel(JSON.parse(readFileSync(resolve(ROOT, 'levels', f), 'utf8'))), nav = new NavGrid(new StaticCollision(level.boxes)); return { level, nav }; };
  const sumOf = (a: Float32Array) => { let n = 0, most = 0; for (const v of a) { if (v > 0) n++; most = Math.max(most, v); } return { n, most }; };
  // 1. a level of the game with no routes: a team's start has a way to every room, through doors, on free floor only; the same twice; the grid's own search
  //    tables are not this function's
  { const { level, nav } = open('block.json'), before = nav.blocked.slice();
    const t0 = performance.now(), A = levelWays(level, level.scenarios, nav), ms = performance.now() - t0, B = levelWays(level, level.scenarios, nav);
    const same = ['patrol', 'team', 'doors', 'hint'].every(k => (A as any)[k].every((v: number, i: number) => v === (B as any)[k][i])), free = A.team.every((v: number, i: number) => v === 0 || !nav.blocked[i]);
    const rooms = level.scenarios!.roomClear!.rooms, starts = 1 + (level.scenarios!.roomClear!.starts?.length ?? 0), team = sumOf(A.team);
    const inRoom = rooms.every((r: any) => { for (let j = 0; j < A.h; j++) for (let i = 0; i < A.w; i++) { const x = (i + 0.5) * A.cell, z = (j + 0.5) * A.cell; if (A.team[j * A.w + i] > 0 && x > r.bounds.x0 && x < r.bounds.x1 && z > r.bounds.z0 && z < r.bounds.z1) return true; } return false; });
    const doorsHit = level.doors.filter((d: any) => { const cx = d.hinge[0] + Math.cos(d.closedDir) * d.width * 0.5, cz = d.hinge[1] + Math.sin(d.closedDir) * d.width * 0.5; for (let dj = -1; dj <= 1; dj++) for (let di = -1; di <= 1; di++) { const i = Math.floor(cx / A.cell) + di, j = Math.floor(cz / A.cell) + dj; if (A.team[j * A.w + i] > 0) return true; } return false; }).length;
    verdict(same && free && inRoom && team.n > 100 && team.most <= starts * rooms.length && doorsHit >= rooms.length && sumOf(A.patrol).n === 0 && sumOf(A.doors).n >= level.doors.length * 2 && A.paths.filter((q: any) => q.kind === 'doors').length === (n => n * (n - 1) / 2)(level.doors.filter((d: any) => d.exterior).length) && nav.blocked.every((v: number, i: number) => v === before[i]) && A.w === nav.W && A.cell === nav.cell,
      'on a level with no routes a team\'s start has a way into every room, on free floor and through doors; a cell counts each way once (never more than starts × rooms); every door counts at both its faces and every pair of outer doors has a way between them; the same map twice',
      `block: team ${team.n} cells, most ${team.most} of ${starts * rooms.length} ways · ${doorsHit} of ${level.doors.length} doors on a team's way · doors ${sumOf(A.doors).n} cells · ${ms.toFixed(1)} ms`); }
  // 2. the author's own, by name: weights add, a name the level does not have walks nothing and is listed; the office's routes are its patrol
  { const { level, nav } = open('white-roof.json'), H = level.scenarios?.walked ?? [];
    const W = levelWays(level, level.scenarios, nav), twice = levelWays(level, level.scenarios, nav, { hints: [...H, ...H] }), none = levelWays(level, level.scenarios, nav, { hints: [] }), bad = levelWays(level, level.scenarios, nav, { hints: [{ from: 'no such thing', to: 'service' }] });
    const most = sumOf(W.hint).most, wsum = H.reduce((a: number, h: any) => a + (h.weight ?? 1), 0), doubles = W.hint.every((v: number, i: number) => twice.hint[i] === 2 * v);
    const off = open('office.json'), O = levelWays(off.level, off.level.scenarios, off.nav);
    // … and over every level of the game: no way without a path, no name a level does not have, no name that is two things (a door's id is looked for first)
    const sick: string[] = []; let ways = 0;
    for (const f of readdirSync(resolve(ROOT, 'levels')).filter(x => x.endsWith('.json') && !x.endsWith('.lint.json'))) { let L; try { L = open(f); } catch { continue; }
      const A = levelWays(L.level, L.level.scenarios, L.nav), sc: any = L.level.scenarios ?? {}; ways += A.paths.length;
      if (A.lost) sick.push(`${f}: ${A.lost} with no path`); if (A.unknown.length) sick.push(`${f}: no such ${A.unknown.join(', ')}`);
      const kinds = (n: string) => [n === 'spawn', L.level.doors.some((d: any) => d.name === n), (L.level.marks ?? []).some((m: any) => m.name === n), L.level.routes.some((r: any) => r.name === n), (sc.roomClear?.rooms ?? []).some((r: any) => r.id === n || r.name === n), (sc.places?.areas ?? []).some((a: any) => a.name === n)].filter(Boolean).length;
      for (const h of sc.walked ?? []) for (const n of [h.from, h.to]) if (kinds(n) > 1) sick.push(`${f}: '${n}' is two things`); }
    verdict(H.length >= 4 && W.unknown.length === 0 && sumOf(W.hint).n > 60 && most > 1 && most <= wsum && doubles && sumOf(none.hint).n === 0 && bad.unknown.join() === 'no such thing' && sumOf(bad.hint).n === 0 && sumOf(O.patrol).n > 100 && off.level.routes.length > 0 && sick.length === 0 && W.paths.filter((q: any) => q.kind === 'hint').length === H.length && W.lost === 0,
      'a level\'s own ways by name: every name of the white roof\'s is found, ways that share floor add their weights, the same list twice counts twice, no list walks nothing, and a name the level does not have is listed and walks nothing; a level with routes has them as its patrol; and over every level file no way is without a path, no name is missing and none is two things',
      `white roof: ${H.length} ways, ${sumOf(W.hint).n} cells, most ${most} of ${wsum} · unknown asked for: ${bad.unknown.join(', ')} · office patrol ${sumOf(O.patrol).n} cells over ${off.level.routes.length} routes · ${ways} ways over every level${sick.length ? ' · ' + sick.slice(0, 4).join(' · ') : ''}`); }
  // 3. the reader: with `walked` the traffic map has a lane where a team walks on a level with no routes, and without it the map is what it was
  { const { level, nav } = open('block.json'), plain = buildWearMap(level, nav, false, false), was = buildWearMap(level, nav, false), on = buildWearMap(level, nav, false, !BRK), W = levelWays(level, level.scenarios, nav);
    let k = 0; for (let i = 0; i < W.team.length; i++) if (W.team[i] > W.team[k]) k = i;
    const x = (k % W.w + 0.5) * W.cell, z = (Math.floor(k / W.w) + 0.5) * W.cell, at = (m: { data: Uint8Array; w: number }) => m.data[(Math.floor((z - WORLD.min[2]) * WEAR_RES) * m.w + Math.floor((x - WORLD.min[0]) * WEAR_RES)) * 2];
    let lanes = 0, band = true; for (let i = 0; i < on.data.length; i += 2) { if (on.data[i] > plain.data[i] + 8) lanes++; if (on.data[i + 1] !== plain.data[i + 1]) band = false; }
    const off = open('office.json'), om = buildWearMap(off.level, off.nav, false); let osum = 0; for (let i = 0; i < om.data.length; i += 2) osum = (Math.imul(osum, 31) + om.data[i]) >>> 0;
    const OFFICE_TRAFFIC_SUM = 2773109031;   // (the office's traffic bytes as every page but the look's has them, held from before this reader took ways)
    verdict(plain.data.every((v: number, i: number) => v === was.data[i]) && osum === OFFICE_TRAFFIC_SUM && at(on) > at(plain) + 60 && lanes > 2000 && band,
      'the traffic map takes the ways on the look\'s page: where a team walks most on a level with no routes there is a lane that was not there, the wall band is not touched, and without the word the map is to the byte what it was',
      `block, the busiest cell of a team's way (${x.toFixed(2)}, ${z.toFixed(2)}): ${at(plain)} of 255 before, ${at(on)} with the ways · ${lanes} texels gained a lane · the office's traffic without the word sums to ${osum} (held: ${OFFICE_TRAFFIC_SUM})`); }
  // 4. the wiring, by its text: one layer wears a pixel, the map is read once, use follows feet, and nothing of the game's imports the ways
  { const sh = (f: string) => readFileSync(resolve(ROOT, 'src/render/shaders', f), 'utf8'), look = sh('look.wgsl'), det = sh('detail.wgsl'), gb = sh('gbuffer.wgsl'), gc = sh('gbuffer_cut.wgsl'), mk = sh('markings.wgsl');
    const body = look.slice(look.indexOf('fn lookSurface('));
    const one = /let floorPix = !byLook && /.test(det) && /let skirtPix = !byLook && /.test(det) && gb.includes('let lookW = LOOK && !cap && lookWears(n, bi, v.hf);') && gc.includes('let lookW = LOOK && !cap && own < 0 && !side && lookWears(n, bi, hfS);')
      && [gb, gc].every(t => /detailSurface\([^;]*frame\.matCfg\.w, lookW\);/.test(t) && /lookSurface\([^;]*frame\.invScreen\.y, lookW\);/.test(t) && (t.match(/lookMarkSoft\(\), lookTrod\(\)\)/g) ?? []).length >= 1)
      && look.includes('fn lookWears(n: vec3f, bi: u32, hf: vec4f) -> bool { return lookCfg.a.w > 0.0 && lookDresses(n, bi, hf); }') && body.includes('if (!lookDresses(n, bi, hf) || (all(lookCfg.a == vec4f(0.0))') && body.includes('  if (wears) {');
    const once = (body.match(/lookWayAt\(/g) ?? []).length === 2 && (body.match(/textureSampleLevel\(wearTex/g) ?? []).length === 0 && body.includes('let wm = lookWay;');
    const use = look.includes('worn *= 1.0 + 0.8 * lookWayAt(') && body.includes('smoothstep(0.44 - 0.22 * beside, 0.6 - 0.22 * beside,') && mk.includes('- (0.72 - 0.12 * age - 0.16 * trod));');
    const game = readdirSync(resolve(ROOT, 'src/game')).filter(f => f.endsWith('.ts') && /scene\/ways/.test(readFileSync(resolve(ROOT, 'src/game', f), 'utf8')));
    const main = readFileSync(resolve(ROOT, 'src/main.ts'), 'utf8'), gm = readFileSync(resolve(ROOT, 'src/game/game.ts'), 'utf8');
    verdict(one && once && use && game.length === 0 && main.includes("if (q.get('walked') === '0') lookConfig.walked = 0;") && gm.includes('lookConfig.on && lookConfig.walked > 0);'),
      'on the look\'s page one layer wears a pixel\'s floor and skirting (the g-buffer asks once and tells both layers; a cut face and a round\'s side have neither), the traffic map is read once a pixel in it (a floor where it is, a kept wall a stride out from its foot), a piece\'s history, a wall\'s knocked foot and a painted mark\'s wear follow where feet go, the page\'s word puts the ways off, and nothing in src/game imports them',
      `one layer ${one} · read once ${once} · use follows feet ${use} · src/game imports of scene/ways: ${game.length}`); }
  console.log(all ? '=== ways: ALL PASS ===' : '=== ways: FAILURES above ===');
  return all;
}
if (import.meta.main) process.exit(await waysCheck() ? 0 : 1);
