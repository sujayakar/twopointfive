// WET GROUND, AS THE PICTURE DRAWS IT (src/game/wetGround.ts, a dressing step: src/game/dress.ts `wetGround`): under a rain, is what lies flat in the open wet?
//   bun run tools/qa/wetground-check.ts                      exit 0 = all pass, 1 = a line failed (named); TPS_PROFILE=shipped in front for the page's stand
//   WETGROUND_BREAK=1 bun run tools/qa/wetground-check.ts    the red path: the step's row is put down, so nothing but the lot is wet — lines 1, 2, 3 and 6 must FAIL
//   bun run tools/qa/probes.ts wetground                     the same, as a probe (family `weather`)
// The game is stood up on a level file with a rain picked and stepped as the page steps it (game.update, so the picture's block runs). WHAT IS HELD, on the
// levels of LEVELS (the newer kit's yards, a look twin, the office, a pad under a canopy, a hut with no floor):
//   1 the yard     nothing flat lies dry in the open: every ground slab, every solid within WET_OVER over one with the sky clearly over it, and every sheet on
//                  those with the sky clearly over it, is wet. The kit's table of what lies on what (scene/gwotKit.ts `lie`) tops out under WET_OVER
//   2 the same rule as before, moved: on a level in the look's paints every wet sheet is one the page's old step wetted (scene/lookTidy.ts wetTidy until 11-04,
//                  kept below as it was) or one whose ground is a finish this step newly wets — none else; and each of those that is NOT wet has a roof over
//                  it (the one clause the rule gained in the move: a sheet asks the sky for itself)
//   3 how wet shows what soaks water (a lawn, a rug, a mat: its paint is cloth, foliage or card to a round) goes darker and takes no film; what sheds it takes
//                  the film (gloss, the water's roughness, the stretched lobe); bare metal takes the film and is not darkened (its albedo is its mirror)
//   4 picture only no albedo of any static but the lot's moves; nothing clearly under a roof is wet but a ground slab (which is the yard's ground too); with no
//                  rain nothing is wet
//   5 put back     the rain taken away, the yard is dry IN THAT CALL (no update between) and every static has the finish the same level stands with unrained,
//                  bit for bit; picked again, the same set is wet; the row put down under the rain dries what the step wetted, and raised wets the same set
//   6 as it stands a wet thing hidden IN PLACE (a flag written, no box added, the world's serial not moved, nobody told) is dry an update later with the finish
//                  it stands with unrained, and put back it is wet again with the same set: the step follows the boxes as they stand, and keeps no count
//                  of what has happened to them (so a level restarted after a fight holds what the same level left alone holds: cleanrestart-check.ts)
// THE SKY IS ASKED HERE APART FROM THE STEP: 81 spots a box where the step reads 25, and a line only judges a box that is CLEARLY out (six tenths and more
// of it) or CLEARLY under (four tenths and less); between the two the step's own count decides and no line speaks.
// No picture is judged: what the renderer makes of wetK and the film is for a GPU. Never seen on one.
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

type B = { c: number[]; h: number[]; yaw: number; albedo: number[]; flags: number; rough?: number; metal?: boolean; aniso?: boolean; wetK?: number };
const LEVELS = ['crossing', 'police-post', 'vault', 'quiet-relay', 'bazaar', 'compound-gwot', 'floodlit-gwot', 'tut-kit', 'office'];

export async function wetGroundCheck(): Promise<boolean> {
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { BoxFlag, SHEET_MASK } = await import(`${ROOT}/src/scene/boxes.ts`);
  const { lookPaintOf } = await import(`${ROOT}/src/scene/gwotPalette.ts`);
  const { gwotKit, gwotOfOffice } = await import(`${ROOT}/src/scene/gwotKit.ts`);
  const { officeKit } = await import(`${ROOT}/src/scene/officeGen.ts`);
  const { DRESS } = await import(`${ROOT}/src/game/dress.ts`);
  const { soaks, groundFlat, WET_OVER } = await import(`${ROOT}/src/game/wetGround.ts`);
  const { WEATHER_BOOT } = await import(`${ROOT}/src/game/weatherKinds.ts`);

  const BREAK = process.env.WETGROUND_BREAK === '1', rowWas = DRESS.wetGround, bootWas = WEATHER_BOOT.kind;
  let failed = 0;
  try {   // (the row and the picked kind are put back however this ends: a probe run goes on in this process)
  if (BREAK) DRESS.wetGround = false;
  const check = (ok: boolean, what: string, detail = '') => { if (!ok) failed++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
  const top = (b: B) => b.c[1] + b.h[1];
  const quiet = async <T>(f: () => Promise<T>): Promise<T> => { const log = console.log; console.log = () => {}; try { return await f(); } finally { console.log = log; } };
  async function stand(name: string, kind: string | null) {
    WEATHER_BOOT.kind = kind;
    try { const H: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8'))) })); for (let i = 0; i < 90; i++) H.step(1 / 60); return H; } finally { WEATHER_BOOT.kind = bootWas; }
  }
  const finishOf = (st: B[]) => st.map(b => `${b.albedo.join(',')}|${b.flags}|${b.rough}|${b.aniso}|${b.wetK}`);
  /** THE SKY OVER A BOX, ASKED APART FROM THE STEP: the share of 81 spots over its rectangle (turned with it, out to its edge) with no roof of the rain's over them, each spot the middle of an eighty-first of it */
  const sky = (b: B, roofs: number[][]) => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); let n = 0; for (let i = -4; i <= 4; i++) for (let j = -4; j <= 4; j++) { const lx = b.h[0] * i * (2 / 9), lz = b.h[2] * j * (2 / 9), x = b.c[0] + cs * lx + sn * lz, z = b.c[2] - sn * lx + cs * lz; if (!roofs.some(r => x >= r[0] && x <= r[2] && z >= r[1] && z <= r[3])) n++; } return n / 81; };
  const OUT = 0.6, UNDER = 0.4;
  const sheet = (b: B) => (b.flags & SHEET_MASK) === BoxFlag.NoShadow;
  /** THE PAGE'S OLD STEP, AS IT WAS (scene/lookTidy.ts find's ground half and wetTidy, until 11-04): each ground sheet and the topmost flat under its middle; a
   *  sheet was wet when that flat was. Only on a level that wears one of the look's paints. Returns the pairs. */
  function oldSheets(st: B[]): { d: B; g: B }[] {
    if (!st.some(b => lookPaintOf(b.albedo, 0.004) !== undefined)) return [];
    const grounds = st.filter(b => !(b.flags & SHEET_MASK) && !b.yaw && top(b) <= 0.06 && b.h[0] > 0.4 && b.h[2] > 0.4), out: { d: B; g: B }[] = [];
    for (const d of st) { if ((d.flags & SHEET_MASK) !== BoxFlag.NoShadow || d.h[1] > 0.02 || top(d) > 0.03 || d.c[1] - d.h[1] < 0) continue;
      let g: B | undefined, at = -1e9; for (const b of grounds) if (Math.abs(b.c[0] - d.c[0]) <= b.h[0] && Math.abs(b.c[2] - d.c[2]) <= b.h[2] && top(b) > at) { at = top(b); g = b; }
      if (g) out.push({ d, g }); }
    return out;
  }
  /** the slabs the weather's old loop wetted (weather.ts setWet's wetOnly until 11-04): tops within 3 mm of the lot's, 4 m2 and more, some sky */
  const some = (b: B, roofs: number[][]) => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); for (let i = -2; i <= 2; i++) for (let j = -2; j <= 2; j++) { const lx = b.h[0] * i * 0.45, lz = b.h[2] * j * 0.45, x = b.c[0] + cs * lx + sn * lz, z = b.c[2] - sn * lx + cs * lz; if (!roofs.some(r => x >= r[0] && x <= r[2] && z >= r[1] && z <= r[3])) return true; } return false; };
  const oldSlab = (b: B, roofs: number[][]) => top(b) > -0.011 && top(b) < 0.003 && b.h[0] * b.h[2] >= 4 && !(b.flags & (BoxFlag.NoPrimary | BoxFlag.Glass | BoxFlag.Dynamic)) && some(b, roofs);

  const rows: Record<string, string[]> = { yard: [], same: [], shows: [], only: [], back: [], stands: [] }; const bad: Record<string, number> = { yard: 0, same: 0, shows: 0, only: 0, back: 0, stands: 0 };
  let newly = 0, soaked = 0, filmed = 0, metals = 0, roofedDry = 0, dried = 0, canopy = 0, hidden = 0;
  for (const name of LEVELS) {
    const D = await stand(name, null), dst = D.game.engine.world.statics as B[], unrained = finishOf(dst), albedos = dst.map(b => b.albedo.join(','));
    const dryWet = dst.filter(b => b.wetK !== undefined).length, ownStorm = !!D.game.weather.groundWet;   // (a level's own storm may be raining with no kind picked)
    const H = await stand(name, 'downpour'), g = H.game, st = g.engine.world.statics as B[], W = g.weather, roofs = W.roofs as number[][], lot = W.lot as B | undefined;
    const isWet = (b: B) => b.wetK !== undefined || b === lot;
    // 1. nothing flat lies dry in the open (the rule said again, apart from the step's own code and with this file's own sky)
    const drawn = (b: B) => !(b.flags & (BoxFlag.NoPrimary | BoxFlag.Glass | BoxFlag.Dynamic));
    const slabs = st.filter(b => b !== lot && oldSlab(b, roofs)), bases = lot ? [lot, ...slabs] : slabs, under = (b: B, s: B) => Math.abs(b.c[0] - s.c[0]) <= s.h[0] && Math.abs(b.c[2] - s.c[2]) <= s.h[2];
    const laid = (b: B) => !sheet(b) && drawn(b) && b !== lot && !slabs.includes(b) && bases.some(s => under(b, s) && top(b) > top(s) && top(b) - top(s) <= WET_OVER);
    const finishes = st.filter(b => laid(b) && sky(b, roofs) >= OUT), ground = new Set<B>([...bases, ...finishes]);
    const floors = st.filter(groundFlat as (b: B) => boolean);
    const groundOf = (d: B) => { let f: B | undefined, at = -1e9; for (const b of floors) if (under(d, b) && top(b) > at) { at = top(b); f = b; } return f; };
    const lying = (b: B) => sheet(b) && b.h[1] <= 0.02 && top(b) <= 0.03 && b.c[1] - b.h[1] >= 0;
    const sheets = st.filter(b => lying(b) && sky(b, roofs) >= OUT && ground.has(groundOf(b) as B));
    const owed = [...slabs, ...finishes, ...sheets], dry = owed.filter(b => !isWet(b));
    const stands = st.filter(b => !sheet(b) && drawn(b) && b.h[1] <= 0.02 && top(b) > WET_OVER && top(b) <= 0.03 && sky(b, roofs) >= OUT).length;
    if (!W.groundWet || dry.length || !owed.length) bad.yard++;
    rows.yard.push(`${name}: ${owed.length - dry.length} of ${owed.length} wet${stands ? ` (${stands} more stand on the ground, 20 to 30 mm up, and stay dry)` : ''}${dry.length ? ` — DRY: ${dry.slice(0, 4).map(b => `${(top(b) * 1000).toFixed(0)} mm ${(b.h[0] * 2).toFixed(1)}×${(b.h[2] * 2).toFixed(1)}`).join(', ')}` : ''}`);
    // 2. the sheets are the old step's, plus those on a newly wet finish, less those under a roof (a level in the look's paints; elsewhere the old step did nothing)
    const pairs = oldSheets(st), look = pairs.length > 0;
    if (look) {
      const wasWet = (b: B) => b === lot || oldSlab(b, roofs), old = new Set(pairs.filter(p => wasWet(p.g)).map(p => p.d)), fresh = new Set(pairs.filter(p => !wasWet(p.g) && isWet(p.g)).map(p => p.d));
      const now = new Set(st.filter(b => sheet(b) && b.wetK !== undefined)), want = new Set([...old, ...fresh]);
      const more = [...now].filter(b => !want.has(b)).length, left = [...want].filter(b => !now.has(b)), fewer = left.filter(b => sky(b, roofs) >= OUT).length; if (more || fewer || !now.size) bad.same++;
      newly += [...fresh].filter(b => now.has(b)).length; roofedDry += left.length - fewer;
      rows.same.push(`${name}: ${now.size} sheets wet, of ${old.size} the old step wetted + ${fresh.size} on a newly wet finish, ${left.length - fewer} of those left dry under a roof${more || fewer ? ` — ${more} MORE, ${fewer} FEWER IN THE OPEN` : ''}`);
    }
    // 3. what soaks takes no film, what sheds takes it; metal is not darkened
    const mine = st.filter(b => b.wetK !== undefined), film = (b: B) => (b.flags & BoxFlag.Gloss) !== 0 && b.aniso === true;
    const wrong = mine.filter(b => soaks(b) === film(b) || b.wetK !== (b.metal ? 1 : 0.5)); soaked += mine.filter(b => soaks(b)).length; filmed += mine.filter(b => !soaks(b)).length; metals += mine.filter(b => b.metal).length;
    if (wrong.length || !mine.length) bad.shows++; if (wrong.length) rows.shows.push(`${name}: ${wrong.length} wrong`);
    // 4. picture only: no albedo but the lot's; nothing clearly under a roof is wet but a slab; dry, nothing is wet
    const moved = st.filter((b, i) => b !== lot && b.albedo.join(',') !== albedos[i]).length, roofed = mine.filter(b => !slabs.includes(b) && sky(b, roofs) <= UNDER).length;
    const kept = st.filter(b => b !== lot && !slabs.includes(b) && !isWet(b) && sky(b, roofs) <= UNDER && (laid(b) || (lying(b) && isWet(groundOf(b) as B)))).length; canopy += kept;
    if (moved || roofed || (dryWet && !ownStorm)) bad.only++;
    rows.only.push(`${name}: ${moved} albedos moved, ${roofed} wet under a roof (${kept} laid or lying on wet ground under one, kept dry), ${dryWet} wet with no rain picked${ownStorm ? ' (its own storm)' : ''}`);
    // 5. the rain taken away: dry in that call, then as the level stands unrained; picked again: the same set; the row down and up under the rain
    const set1 = st.map(b => b.wetK !== undefined);
    DRESS.wetGround = false; H.step(1 / 60); const rowDown = st.filter(b => b.wetK !== undefined).length; DRESS.wetGround = !BREAK; H.step(1 / 60); const rowUp = st.every((b, i) => (b.wetK !== undefined) === set1[i]);
    W.setKind(null); const atOnce = st.filter(b => b.wetK !== undefined).length + (W.lot ? 1 : 0); for (let i = 0; i < 60 * 40; i++) H.step(1 / 60);
    const after = finishOf(st), differ = after.filter((x, i) => x !== unrained[i]).length; if (!W.groundWet) dried++;
    W.setKind('downpour'); for (let i = 0; i < 90; i++) H.step(1 / 60); const same = st.every((b, i) => (b.wetK !== undefined) === set1[i]);
    if (differ || !same || atOnce || rowDown || !rowUp) bad.back++;
    rows.back.push(`${name}: ${atOnce} still wet in the call that took the rain away, ${differ} statics not as unrained${W.groundWet && ownStorm ? ' (its own storm rains on)' : ''}, picked again ${same ? 'the same set' : 'ANOTHER SET'}, the row down ${rowDown} wet, up ${rowUp ? 'the same set' : 'ANOTHER SET'}`);
    // 6. a wet thing hidden in place and put back (a finish where the level has one wet, else a slab, else a sheet): one flag written, nothing else moved
    { const world = g.engine.world as { geomSerial: number }, n0 = st.length, serial0 = world.geomSerial, was = finishOf(st);
      const f = st.find(b => b.wetK !== undefined && laid(b)) ?? st.find(b => b.wetK !== undefined && !sheet(b)) ?? st.find(b => b.wetK !== undefined), i = f ? st.indexOf(f) : -1;
      if (!f) { bad.stands++; rows.stands.push(`${name}: NOTHING WET TO HIDE`); }
      else {
        f.flags |= BoxFlag.NoPrimary; H.step(1 / 60);
        const now = `${f.albedo.join(',')}|${f.flags & ~BoxFlag.NoPrimary}|${f.rough}|${f.aniso}|${f.wetK}`, dry = f.wetK === undefined && (ownStorm || now === unrained[i]), told = st.length !== n0 || world.geomSerial !== serial0;
        f.flags &= ~BoxFlag.NoPrimary; H.step(1 / 60);
        const back = finishOf(st).filter((x, k) => x !== was[k]).length;
        if (!dry || told || back) bad.stands++; else hidden++;
        rows.stands.push(`${name}: a ${sheet(f) ? 'sheet' : laid(f) ? 'finish' : 'slab'} ${(f.h[0] * 2).toFixed(1)}×${(f.h[2] * 2).toFixed(1)} m hidden ${dry ? `is dry${ownStorm ? ' (its own storm: there is no unrained finish to hold it to)' : ' and as unrained'}` : `STAYS WET OR IS NOT AS UNRAINED (${now.split('|').slice(1).join('|')} against ${unrained[i].split('|').slice(1).join('|')})`}${told ? ', AND THE WORLD WAS TOLD' : ''}, put back ${back ? `${back} STATICS NOT AS BEFORE` : 'the same set'}`);
      } }
  }
  // (the kit's table of what lies on what stays under the window the step calls the ground)
  const lie = gwotKit(gwotOfOffice(officeKit({ groups: [] }))).lie as Record<string, number>, highest = Math.max(...Object.values(lie));
  check(bad.yard === 0 && highest + 0.002 < WET_OVER, '1 under a picked rain nothing flat lies dry in the open: every ground slab, every finish laid on one and every sheet on those is wet; the kit lays nothing that lies higher than the step calls the ground', rows.yard.join(' · ') + ` · the kit's highest lying piece ${(highest * 1000).toFixed(0)} mm (+2 for the exporter's steps) under ${(WET_OVER * 1000).toFixed(0)}`);
  check(bad.same === 0 && rows.same.length >= 5 && newly > 0, '2 the same rule, moved: on a level in the look\'s paints every wet sheet is one the page\'s old step wetted or lies on a finish that is newly wet, and each of those left dry has a roof over it', rows.same.join(' · ') + ` · ${roofedDry} left dry under a roof in all`);
  check(bad.shows === 0 && filmed > 0 && soaked > 0, '3 how wet shows is what the thing is made of: what sheds water takes the film, what soaks it up (a lawn, a rug) goes darker and takes none, bare metal takes the film and keeps its colour', `${filmed} with the film (${metals} of them metal), ${soaked} soaked and without${rows.shows.length ? ' · ' + rows.shows.join(' · ') : ''}`);
  check(bad.only === 0 && canopy > 0, '4 picture only: no static\'s albedo moves but the lot\'s; nothing clearly under a roof is wet but a ground slab; with no rain nothing is wet', rows.only.join(' · '));
  check(bad.back === 0 && dried >= 3, '5 the rain taken away, the yard is dry in that call and every static has the finish the level stands with unrained; picked again, the same set is wet; the row put down dries what the step wetted', rows.back.join(' · ') + ` · the ground went dry on ${dried} of ${LEVELS.length} levels (the rest keep their own storm)`);
  check(bad.stands === 0 && hidden === LEVELS.length, '6 the step follows the boxes as they stand: a wet thing hidden in place, with no box added and the world\'s serial not moved, is dry an update later with the finish it stands with unrained, and put back it is wet again with the same set', rows.stands.join(' · '));
  console.log(failed ? `=== wetground: ${failed} FAILED ===` : '=== wetground: ALL PASS ===');
  } finally { DRESS.wetGround = rowWas; WEATHER_BOOT.kind = bootWas; }
  return failed === 0;
}
if (import.meta.main) process.exit((await wetGroundCheck()) ? 0 : 1);
