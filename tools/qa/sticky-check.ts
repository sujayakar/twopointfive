// STICKY CAMERAS (src/game/stickycam.ts — a small count on YOUR kit card; the rifle's launcher sends one to a wall, where it is one more eye on the fog of
// war), headless:
//   bun run tools/qa/sticky-check.ts [kit|fly|fog|save|default|take|mark|wand]      (probes.ts sticky; STICKY_BREAK=1 breaks each part's staging: must fail)
// The maintainer, 10-12: "i think we do really need to figure out if you get omniscience in stealth levels or not … maybe a limited number of sticky cameras or
// something like that would work."
//   kit     the row on your card, the last of the things carried; a pouch each: refused on the issued card until a pouch is given up; none without the
//           rifle; the store keeps the count; teammates have no row; staging puts the load and the count on the rifle; a card with none leaves the
//           rifle as built
//   fly     by the keys: 2, 2 again twice = the camera on the right button, a press shows the arc, letting go sends ONE, the count falls, the tube waits;
//           it sticks to a wall at height; off a door leaf and off a man it drops to the floor (the man is staggered and knows); a fourth sent takes the
//           oldest down; the tube and the landing are heard as small 'prop' noises
//   fog     a quiet mission with the fog on: a guard nobody sees is seen once a camera with a line to him has stuck, inside its wedge only, and is hidden
//           again when it is taken down; ONE feed at a time (three out: the newest alone is an eye, its lens red); every feed live = three eyes and the fog's
//           places never passed; with no fog on the floor a camera is no eye
//   save    a checkpoint carries the count and the cameras that are out, a plain save waits for one in the air, a forced one hands it back unsent
//   default THE DEFAULT AS IT SHIPS (the maintainer, 10-13: "i think it should be fog + teammate … + we find a way to balance sticky cams"):
//           every quiet mission opened with no click is under the fog all round with two cameras on the rifle; as two the partner's eye is one of the fog's;
//           quietfog=off = all shown, cams=0 = the fog alone; a lesson and a squad row are untouched; the links say only what differs
//   take    a stuck camera is a row of the use key at it — taken back, on the launcher again, its eye gone; not through a wall, not out of reach
//   mark    his own stuck cameras show in the fog of war — a dark one on floor no eye has stays in the picture's box list, the HUD's marks say where each
//           lens is and which one sends, none for one in the air or with no fog
//   wand    the mission that hands out its cameras hands Sam an optical wand too — not the fog alone, not with no fog, gone when the mission is quit
// EVERY SWITCH HERE IS SETTLED ON (STICKY.on / .take / .mark, QUIETFOG.on / .def 'team' / .wand, WAND.on, FOG.things): the check asks the page's side
// alone and writes none of them. Until 2026-11-07:
//   · the rig stood "the old world" for the parts written in it (QUIETFOG.def 'off': no rung on a link = all guards shown, no cams key = none). Those
//     parts now ask for the same floor by the link's own keys (cams=0: the fog and none of the mission's cameras; quietfog=off: no fog), and issue
//     their own cameras as before
//   · `ways` (a quiet row's link three ways with the default held at all shown, the Sticky cameras row under the fog lever from 'off', each way
//     started; it also held the fourth lever and the squad's fog-off rung down): the whole part asked the build before the default, out by ruling
//   · every part's switch-off half: the card and the store with the cameras' switch off, the take's row and the mark with theirs down ('as it was'),
//     the same run by digest down and up, the wand with its switch or the tool's down, the old default put back on the lever, a recording from
//     before each key and the two stands' rows
// STICKY_BREAK=1: kit = the rifle left at home; every camera sent is sent at the floor under its tube (fly, fog, save, take, mark); default and wand =
// the mission opened with every guard shown (quietfog=off).
// Nothing here looks at a picture.
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)

const SC = await import(`${ROOT}/src/game/stickycam.ts`);
const { STICKY, STICKY_STATS, stickyIssue, fireCam, stickyEyes, camLens, stickyMarks } = SC;
const K = await import(`${ROOT}/src/game/kit.ts`);
const QF = await import(`${ROOT}/src/game/quietfog.ts`);
const { FOG, fieldDeg, fogEyeCount } = await import(`${ROOT}/src/game/fogofwar.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SAVE = await import(`${ROOT}/src/game/savestate.ts`);
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);

type Vec3 = [number, number, number];
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const J = (o: unknown) => JSON.stringify(o), DT = 1 / 60, f2 = (x: number) => Math.round(x * 100) / 100;
const BREAK = process.env.STICKY_BREAK === '1';
const LIVE_WAS = STICKY.live;
const back = () => { STICKY.live = LIVE_WAS; };
const steps = (W: any, n: number, each?: () => void) => { for (let i = 0; i < n; i++) { each?.(); W.step(DT); } };
const memStore = (init: Record<string, string> = {}) => { const m = new Map(Object.entries(init)); return { m, getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, String(v)); }, removeItem: (k: string) => { m.delete(k); } }; };
/** the kit table, its rule and the solo kit flag (kit.ts KIT_SOLO.on: raised by every kitSoloIf on a quiet mission alone) as they stood, put back after `fn` */
async function keepKit<T>(fn: () => Promise<T> | T): Promise<T> {
  const cards = K.KIT.members.map((m: any) => ({ ...m })), on = K.KIT.on, slots = K.SLOTS.on, solo = K.KIT_SOLO.on;
  try { return await fn(); } finally { K.kitLoad(null, ''); for (let i = 0; i < cards.length; i++) K.KIT.members[i] = cards[i]; K.KIT.on = on; K.SLOTS.on = slots; K.KIT_SOLO.on = solo; }
}
async function stand(seed: number, file?: string) {
  Math.random = seeded(0x51c4 + seed * 7919);
  const level = file ? loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) : undefined;
  return await hushed(() => standUp({ uploadEvery: 1, ...(level ? { level } : {}) })) as any;
}
/** a quiet mission alone on `file` with the fog of war on (all round) and none of the mission's own cameras, the AI still; `fog` '' = no fog */
async function quietFloor(seed: number, file = 'police-post.json', fog = 'team') {
  const W = await stand(seed, file), g = W.game;
  await hushed(() => startFromQuery(g, `?levelFile=${file}&quiet=1${fog ? `&quietfog=${fog}&cams=0` : '&quietfog=off'}`));
  g.aiEnabled = false; g.godMode = true; steps(W, 30);
  return { W, g };
}
const chestOf = (gd: any): Vec3 => gd.char.bones.chest ?? [gd.char.pos[0], 1.1, gd.char.pos[2]];
/** a place on a wall a camera would have a line to `gd` from, and where to send it from: the first of 32 bearings off his head height that meets a static
 *  squarely enough, with clear air a metre off the wall */
function wallFor(g: any, gd: any): { P: Vec3; n: Vec3; from: Vec3; index: number } | null {
  const C: Vec3 = [gd.char.pos[0], STICKY.aimY, gd.char.pos[2]], chest = chestOf(gd);
  for (let k = 0; k < 32; k++) {
    const a = k * Math.PI / 16, d: Vec3 = [Math.sin(a), 0, Math.cos(a)], h = g.col.raycast(C, d, 12);
    if (!h || h.index >= 1e6 || h.t < 2.2 || Math.abs(h.n[1]) > 0.3 || -(h.n[0] * d[0] + h.n[2] * d[2]) < 0.5) continue;
    const P = v3.mad(C, d, h.t) as Vec3, from = v3.mad(P, h.n, 1.3) as Vec3, lens = v3.mad(P, h.n, STICKY.off) as Vec3;
    if (g.col.segmentBlocked(from, v3.mad(P, h.n, 0.05)) || g.col.segmentBlocked(lens, chest)) continue;
    return { P, n: [...h.n] as Vec3, from, index: h.index };
  }
  return null;
}
/** send one from `from` at `to` and step until it has come to rest (≤ 3 s) */
function send(W: any, from: Vec3, to: Vec3) { if (BREAK) to = [from[0], 0, from[2]]; const g = W.game, cam = fireCam(g, g.player, from, to); let n = 0; while (cam.vel && n++ < 180) W.step(DT); return cam; }

// ================================================================================================ kit
export async function stickyKit(): Promise<boolean> {
  const F = family('sticky-kit: sticky cameras are a row of your card that takes a pouch each, is kept per machine and rides onto your rifle');
  const rnd = Math.random;
  try {
    await keepKit(async () => {
      const fields = (i: number) => K.kitRows(i).map((r: any) => r.field as string);
      K.kitLoad(null, ''); if (BREAK) K.kitSet(0, 'weapon', 'pistol');
      const onRows = fields(0), mates = [1, 2, 3, 4].map(i => fields(i).includes('cams'));
      F.verdict(onRows.includes('cams') && onRows.indexOf('cams') === onRows.indexOf(onRows.includes('wand') ? 'wand' : 'suit') - 1 && onRows.indexOf('cams') > onRows.indexOf('shells') && !mates.some(Boolean),
        'the row is on YOUR card, after the 40 mm row and your charge rows, before the wand\'s where your card has one; no teammate has one', `${onRows.join(' ')} · teammates ${J(mates)}`);

      // the pouches rule: issued = full
      const c0 = K.kitCost(0), t0 = K.kitTry(0, 'cams', 1), stillNone = K.KIT.members[0].cams === 0;
      const gave = K.kitSet(0, 'smoke', 0), t1 = K.kitTry(0, 'cams', 1), c1 = K.kitCost(0);
      const row = K.kitRows(0).find((r: any) => r.field === 'cams'), rc = K.kitRowCost(0, row), words = K.kitWords(0), legal = K.kitLegal();
      F.verdict(K.SLOTS.on && c0.used === c0.total && !t0.ok && /pouch/.test(t0.why) && stillNone && gave && t1.ok && t1.value === 1 && c1.used === c1.total && rc?.pouches === 1 && /1.cam\b/.test(words) && legal === null,
        'issued, your four pouches are full: a camera is refused in words until a can is given up, then it takes that pouch and the card says so', `issued ${c0.used} of ${c0.total} · try → ${t0.ok} "${t0.why}" · smoke given up ${gave} · try → ${t1.ok} · ${c1.used} of ${c1.total} · row ${J(rc)} · "${words}" · legal ${legal}`);
      const top = row?.values?.length - 1, over = K.kitTry(0, 'smoke', 1);
      F.verdict(top === 3 && !over.ok, 'the row tops out at three, and with a camera in, the can it replaced no longer fits', `row values ${J(row?.values)} · smoke +1 → ${over.ok} "${over.why}"`);

      // no rifle, no launcher
      const pistol = K.kitSet(0, 'weapon', 'pistol'), home = K.KIT.members[0].cams, noTube = K.kitTry(0, 'cams', 1);
      F.verdict(pistol && home === 0 && !noTube.ok, 'the rifle left at home takes the cameras with it, and none can be added without it', `pistol ${pistol} · cameras then ${home} · try → ${noTube.ok} "${noTube.why}"`);
      K.kitSet(0, 'weapon', 'rifle');

      // the store
      const st = memStore(); K.kitLoad(st, ''); K.kitSet(0, 'smoke', 0); K.kitSet(0, 'frags', 0); K.kitSet(0, 'cams', 2);
      const kept = JSON.parse(st.m.get(K.KIT_LS) ?? '{}').members?.[0]?.cams;
      K.kitLoad(memStore(Object.fromEntries(st.m)), ''); const back = K.KIT.members[0].cams, backCost = K.kitCost(0).used;
      F.verdict(kept === 2 && back === 2 && backCost === 4, 'the store keeps the count; a fresh read has it', `stored ${kept} · read back ${back} (${backCost} of 4)`);

      // staging → the rifle
      K.kitLoad(null, ''); K.kitSet(0, 'smoke', 0); K.kitSet(0, 'frags', 0); K.kitSet(0, 'cams', 2);
      const W = await stand(1), g = W.game;
      await hushed(() => { startFromQuery(g, 'roomclear&squad4'); steps(W, 10); });
      const rf = g.player.rifle, staged = { loads: [...rf.attachments], cams: rf.cams, up: rf.attachment };
      K.kitLoad(null, '');
      const W2 = await stand(1), g2 = W2.game;
      await hushed(() => { startFromQuery(g2, 'roomclear&squad4'); steps(W2, 10); });
      const bare = { loads: [...g2.player.rifle.attachments], cams: g2.player.rifle.cams };
      F.verdict(staged.loads.join() === 'ring,he40,cam' && staged.cams === 2 && staged.up === 'ring' && bare.loads.join() === 'ring,he40' && bare.cams === 0,
        'a card with two puts the third load and the count on your rifle (the ring still up first); a card with none leaves the rifle as built', `two carried: ${J(staged)} · none carried: ${J(bare)}`);
    });
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { back(); Math.random = rnd; }
  return F.done();
}

// ================================================================================================ fly
export async function stickyFly(): Promise<boolean> {
  const F = family('sticky-fly: the launcher sends one by the keys; it sticks to a wall, drops off a leaf or a man, and is heard as a small thing');
  const rnd = Math.random;
  try {
    const P = await import('./fn2000-probes.ts');
    { // by the keys
      const W = await P.stage(7101), g = W.game, pl = g.player, rf = pl.rifle; stickyIssue(pl, 2);
      const tap = P.eventTap(g); tap.pump();
      const loads: string[] = []; for (let k = 0; k < 2; k++) { W.input.pressed.add('Digit2'); W.step(DT); loads.push(String(rf.attachment)); steps(W, 3); }
      const hud = P.hudText(g);
      const spot = (): Vec3 => [pl.char.pos[0] + 5, 0, pl.char.pos[2]];
      let held: unknown = null, arc = 0; P.rmbTap(W, spot, () => { held = pl.launchHeld; arc = pl.throwPreview?.points.length ?? 0; });
      tap.pump(); const sent = pl.cams.length, left = rf.cams, cd = rf.ringCd, launch = tap.evs.filter((e: any) => e.kind === 'prop' && e.who === pl.char.id);
      P.rmbTap(W, spot); const again = pl.cams.length;
      for (let f = 0; f < 150; f++) { W.step(DT); tap.pump(); }   // (an event lives half a second: read every frame)
      const cam = pl.cams[0], tick = tap.evs.filter((e: any) => e.kind === 'prop' && e.who === 0);
      W.input.pressed.add('Digit2'); W.step(DT); loads.push(String(rf.attachment));
      F.verdict(loads.join() === 'he40,cam,ring', '2 again with the rifle up steps ring → 40 mm → camera → ring', `after each press: ${loads.join(' → ')}`);
      F.verdict(/CAM ×2/.test(hud) && held === 'cam' && arc >= 2 && sent === 1 && left === 1 && cd > 0 && again === 1,
        'the kit column counts them; the press shows the arc and sends nothing, letting go sends ONE and the count falls; a second tap inside the tube\'s wait sends nothing', `column has CAM ×2 ${/CAM ×2/.test(hud)} · held ${held}, arc of ${arc} points · out ${sent}, left ${left}, tube ${f2(cd)} s · after a second tap: ${again} out`);
      F.verdict(!!cam && !cam.vel && (cam.si >= 0 ? cam.pos[1] > 0.8 && cam.pos[1] < 2.6 : cam.pos[1] < 0.3) && launch.length === 1 && Math.abs(launch[0].level - STICKY.launchNoise) < 1e-6 && tick.length === 1 && Math.abs(tick[0].level - STICKY.stickNoise) < 1e-6,
        'it comes to rest (on a wall at height, or on the floor), and the tube and the landing are each one small noise', `rest: on static ${cam?.si} at ${J(cam?.pos.map(f2))} · tube events ${J(launch.map((e: any) => e.level))} · landing events ${J(tick.map((e: any) => e.level))}`);
    }
    { // a wall, a leaf, a man, the fourth
      const W = await P.stage(7102, false), g = W.game, pl = g.player; stickyIssue(pl, 3); g.infiniteAmmo = true; SC.stickyStatsReset();
      const A = g.guards[0]; const w = wallFor(g, { char: { pos: pl.char.pos, bones: {} } });
      const cam = w ? send(W, w.from, v3.mad(w.P, w.n, -0.5) as Vec3) : null;
      const off = cam && w ? v3.len(v3.sub(cam.pos, w.P)) : NaN, dn = cam && w ? cam.n[0] * w.n[0] + cam.n[1] * w.n[1] + cam.n[2] * w.n[2] : NaN;
      F.verdict(!!cam && !cam.vel && cam.si === w!.index && off < 0.2 && dn > 0.99, 'sent at a wall it sticks where it struck, on that wall, facing off it', `wall static ${w?.index} · stuck on ${cam?.si}, ${f2(off)} m from the point aimed at, normal · wall's ${f2(dn)}`);

      // what it is on is taken out of the level (destruction hides a static so): inside a second or so it is on the floor
      if (cam && w) { const b = g.col.boxes[w.index], fl = b.flags; b.flags |= 0x1 | 0x2; steps(W, 90); const fell = cam.si === -1 && cam.pos[1] < 0.3; b.flags = fl;   // (NoPrimary | NoShadow)
        F.verdict(fell, 'its wall taken out of the level, it drops to the floor under it', `on ${cam.si} at ${J(cam.pos.map(f2))}`); }

      // a door leaf: nothing to hold to
      const door = g.doors.list.find((d: any) => d.angle === 0 && d.latched) ?? g.doors.list[0], c = door.box.c as Vec3;
      let leafCam: any = null, tried = '';
      for (const a of [0, Math.PI / 2, Math.PI, -Math.PI / 2, Math.PI / 4, -Math.PI / 4, 3 * Math.PI / 4, -3 * Math.PI / 4]) {
        const d: Vec3 = [Math.sin(a), 0, Math.cos(a)], from: Vec3 = [c[0] - d[0] * 1.2, 1.2, c[2] - d[2] * 1.2], h = g.col.raycast(from, d, 2);
        if (!h || h.index < 1e6) continue;
        leafCam = send(W, from, [c[0] + d[0], 1.2, c[2] + d[2]]); tried = `bearing ${f2(a)}`; break;
      }
      F.verdict(!!leafCam && !leafCam.vel && leafCam.si === -1 && leafCam.pos[1] < 0.3 && Math.abs(leafCam.n[1]) > 0.99, 'sent at a door leaf it drops to the floor in front of it and lies flat', `door ${door.def.name}, ${tried} · rest ${J(leafCam?.pos.map(f2))}, on ${leafCam?.si}`);

      // a man: staggered, and he knows
      A.hold = true; A.pinned = false; A.state = 'patrol'; A.awareness = 0; A.heardUpTo = 1e12;
      const at: Vec3 = [pl.char.pos[0] + 6, 0, pl.char.pos[2]]; P.place(A, at, Math.PI / 2); W.step(DT);
      const from: Vec3 = [pl.char.pos[0] + 0.6, 1.25, pl.char.pos[2]], before = g.time;
      const manCam = send(W, from, [at[0], 1.25, at[2]]);
      F.verdict(!manCam.vel && manCam.si === -1 && manCam.pos[1] < 0.3 && Math.hypot(manCam.pos[0] - at[0], manCam.pos[2] - at[2]) < 1 && A.staggerUntil > before && A.state !== 'dead',
        'sent at a man it staggers him like a ring to the body (never a knockout), and drops at his feet', `rest ${J(manCam.pos.map(f2))} (he stands at ${J(at)}) · staggered ${A.staggerUntil > before} · state ${A.state}, awareness ${f2(A.awareness)}`);

      // a fourth takes the oldest down
      const first = pl.cams[0]?.id, was = pl.cams.length, top = Math.max(...pl.cams.map((x: any) => x.id));
      const more = w ? [0.3].map(dy => send(W, [w.from[0], w.from[1] + dy, w.from[2]], [w.P[0] - w.n[0] * 0.5, w.P[1] + dy, w.P[2] - w.n[2] * 0.5])) : [];
      F.verdict(was === 3 && pl.cams.length === STICKY.max && !pl.cams.some((x: any) => x.id === first) && STICKY_STATS.retired === 1 && more.every(m => !m.vel && m.id > top),
        'no more than three stay out: the next one sent takes the oldest down, and no id is handed out twice', `out before ${was} · after one more ${pl.cams.length} (ids ${pl.cams.map((x: any) => x.id).join(' ')}; the first was ${first}) · retired ${STICKY_STATS.retired}`);
    }
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { back(); Math.random = rnd; }
  return F.done();
}

// ================================================================================================ fog
export async function stickyFog(): Promise<boolean> {
  const F = family('sticky-fog: on a quiet mission under the fog of war a stuck camera shows who it has a line to, inside its wedge, and nothing more');
  const rnd = Math.random;
  try {
    const { W, g } = await quietFloor(21), fog = g.fogOfWar, pl = g.player; stickyIssue(pl, 3);
    const unseen = g.guards.filter((gd: any) => gd.state !== 'dead' && !fog.seen(gd));
    let pick: any = null, w: ReturnType<typeof wallFor> = null;
    for (const gd of unseen) { w = wallFor(g, gd); if (w) { pick = gd; break; } }
    F.verdict(fog.active && !!pick, 'the fog is on and there is a guard nobody sees with a wall in his sight', `fog ${fog.active} · eyes ${fog.eyes.length} · unseen ${unseen.map((x: any) => x.callsign).join(' ')} · picked ${pick?.callsign}`);
    if (pick && w) {
      const eyes0 = fog.eyes.length, cam = send(W, w.from, v3.mad(w.P, w.n, -0.5) as Vec3); steps(W, 20);
      const seen = fog.seen(pick), eyes1 = fog.eyes.length, e = fog.eyes[fog.eyes.length - 1], lens = camLens(cam);
      const front = fog.inField(e, v3.mad(lens, w.n, 3)), behind = fog.inField(e, v3.mad(lens, w.n, -3));
      F.verdict(!cam.vel && cam.si >= 0 && seen && eyes1 === eyes0 + 1 && e.id >= 200 && e.fov === STICKY.fovDeg && front && !behind,
        'once it has stuck he is seen: one more eye in the fog\'s list, a wedge about the wall\'s normal — the floor in front of it, never what is behind its wall', `${pick.callsign} seen ${seen} · eyes ${eyes0} → ${eyes1} · eye id ${e?.id}, ${e?.fov}° · in front ${front}, behind ${behind}`);
      const others = unseen.filter((gd: any) => gd !== pick), still = others.filter((gd: any) => !fog.seen(gd)), free = others.filter((gd: any) => fog.seen(gd) && g.col.segmentBlocked(lens, chestOf(gd)));
      F.verdict(free.length === 0, 'nobody it has no line to is shown for it', `the other unseen men: ${others.length}, still unseen ${still.length}, shown with no line from the lens ${free.length}`);
      const coneWas = FOG.cone, lensWas = STICKY.fovDeg; FOG.cone = true; const cone = fieldDeg(), narrow = stickyEyes(g)[0]?.fov, want = Math.min(STICKY.fovDeg, cone);
      STICKY.fovDeg = 160; const wide = stickyEyes(g)[0]?.fov; STICKY.fovDeg = lensWas; FOG.cone = coneWas;
      F.verdict(narrow === want && wide === cone && cone < 160, 'under the fog\'s cone a wall camera\'s wedge is never wider than the cone (the picture has one soft edge for every wedge): the narrower of the lens and the cone', `all round ${STICKY.fovDeg}° · under the cone ${narrow}° (the cone ${cone}°) · a 160° lens under the cone ${wide}°`);
      pl.cams.length = 0; steps(W, 20); const gone = fog.seen(pick), eyes2 = fog.eyes.length;
      F.verdict(!gone && eyes2 === eyes0, 'taken down, he is in the fog again', `seen ${gone} · eyes ${eyes2}`);
      // the cap
      for (const dy of [0, 0.25, -0.25]) send(W, [w.from[0], w.from[1] + dy, w.from[2]], [w.P[0] - w.n[0] * 0.5, w.P[1] + dy, w.P[2] - w.n[2] * 0.5]);
      // one feed at a time: three out, the newest alone sends
      STICKY.live = 1; steps(W, 5); const one = fog.eyes.length, out = pl.cams.filter((c: any) => !c.vel), newest = out[out.length - 1], sends = SC.stickyLive(pl), eyeIs = fog.eyes[fog.eyes.length - 1];
      const dLens = newest ? v3.len(v3.sub(eyeIs.pos, camLens(newest))) : 9, shown = fog.seen(pick);
      // … so a newer camera that looks at nothing gives up what the older ones saw: the newest lies on the floor at the player's own feet (as one dropped)
      if (newest) { newest.pos = [pl.char.pos[0], 0.02, pl.char.pos[2]]; newest.n = [0, 1, 0]; newest.si = -1; } steps(W, 20); const lost = fog.seen(pick);
      const lensPaint = (() => { const bx: any[] = []; SC.stickyBoxes(g, bx); const reds = bx.filter(b => b.albedo[0] > 0.3 && b.albedo[1] < 0.1).length; return { boxes: bx.length, reds }; })();
      F.verdict(out.length === 3 && one === eyes0 + 1 && sends.length === 1 && sends[0] === newest && dLens < 1e-6 && shown && !lost && lensPaint.reds === 1,
        'three out and ONE feed: the newest stuck camera alone is an eye (its lens the one drawn red) — the older two stay on the wall and show nothing, so a newer camera that sees nothing of him gives up what they saw',
        `out ${out.length} · eyes ${eyes0} → ${one} · sending ${sends.length} (the newest ${sends[0] === newest}) · eye at its lens ${dLens.toFixed(4)} m · ${pick.callsign} shown ${shown}, with the newest lying at his own feet ${lost} · red lenses ${lensPaint.reds}`);
      STICKY.live = 3; steps(W, 5); const full = fog.eyes.length; STICKY.live = LIVE_WAS;
      const eyeCap = fogEyeCount(FOG.eyes);   // (how many eyes lift the fog: game/fogofwar.ts FOG.eyes — five on the frozen stand, seven in the page)
      F.verdict(full === Math.min(eyeCap, eyes0 + 3) && full <= eyeCap, 'with every feed live (the rule as it first shipped: live 3) every one is an eye, and the fog\'s places are never passed', `eyes ${full} of ${eyeCap}`);
    }
    // no fog on the floor: a camera is no eye and changes nothing anybody is shown
    const B = await quietFloor(21, 'police-post.json', ''), fb = B.g.fogOfWar; stickyIssue(B.g.player, 2);
    const gd = B.g.guards[0], wb = wallFor(B.g, gd); if (wb) send(B.W, wb.from, v3.mad(wb.P, wb.n, -0.5) as Vec3); steps(B.W, 10);
    const redB = (() => { const bx: any[] = []; SC.stickyBoxes(B.g, bx); return bx.filter(b => b.albedo[0] > 0.3 && b.albedo[1] < 0.1).length; })();
    F.verdict(!fb.active && fb.eyes.length === 0 && stickyEyes(B.g).length === (wb ? 1 : 0) && B.g.guards.every((x: any) => fb.seen(x)) && redB === 0, 'with no fog on the floor everybody is shown as ever, the fog keeps no eyes and no lens is drawn red (a camera that is nobody\'s eye sends nothing)', `fog ${fb.active} · fog eyes ${fb.eyes.length} · cameras out ${B.g.player.cams.length} · red lenses ${redB}`);
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { back(); Math.random = rnd; }
  return F.done();
}

// ================================================================================================ save
export async function stickySave(): Promise<boolean> {
  const F = family('sticky-save: a checkpoint carries the count and the cameras that are out');
  const rnd = Math.random;
  try {
    const { W, g } = await quietFloor(31), pl = g.player; stickyIssue(pl, 2);
    const w = wallFor(g, g.guards[0]) ?? wallFor(g, { char: { pos: pl.char.pos, bones: {} } });
    const flying = w ? fireCam(g, pl, w.from, v3.mad(w.P, w.n, -0.5) as Vec3) : null; pl.rifle.cams--;
    const why = SAVE.canSave(g, false); let n = 0; while (flying?.vel && n++ < 180) W.step(DT);
    F.verdict(!!flying && /sticky camera/.test(String(why)), 'a plain save waits for one in the air', `canSave → "${why}"`);
    const s = W.snapshot('sticky', true), slice = s.player, at = flying ? [...flying.pos] : [];
    pl.cams.length = 0; pl.rifle.cams = 0; W.restore(s); steps(W, 3);
    const p2 = g.player, back = p2.cams[0];
    F.verdict(slice.rifle.cams === 1 && slice.cams?.length === 1 && p2.rifle.cams === 1 && p2.rifle.attachments.includes('cam') && p2.cams.length === 1 && !back.vel && v3.len(v3.sub(back.pos, at as Vec3)) < 1e-6 && back.si === flying!.si,
      'the file holds one left and one out; the load puts both back, the camera where it stuck', `file: rifle.cams ${slice.rifle.cams}, out ${slice.cams?.length} · loaded: ${p2.rifle.cams} left, loads ${p2.rifle.attachments.join(' ')}, out ${p2.cams.length} at ${J(back?.pos.map(f2))}`);
    // forced with one in the air: it is handed back unsent, not lost
    if (w) { fireCam(g, p2, w.from, v3.mad(w.P, w.n, -0.5) as Vec3); p2.rifle.cams--; const s2 = W.snapshot('sticky-air', true);
      F.verdict(s2.player.rifle.cams === 1 && s2.player.cams?.length === 1, 'a forced save with one in the air writes it back onto the rifle', `file: rifle.cams ${s2.player.rifle.cams} (the rifle held ${p2.rifle.cams}, one in flight), out ${s2.player.cams?.length}`);
      const seqWas = p2.camSeq, top = Math.max(...(s2.player.cams ?? []).map((c: any) => c.id)); W.restore(s2); const p3 = g.player;
      F.verdict(s2.player.camSeq === seqWas && seqWas > top && p3.camSeq === seqWas, 'the file keeps the run\'s count of ids, so the one handed back unsent does not give its id out again', `count ${seqWas} · highest id out ${top} · file ${s2.player.camSeq} · loaded ${p3.camSeq}`); }
    // none issued: the file says so, and a load leaves the rifle as built
    const C = await quietFloor(32), s0 = C.W.snapshot('none', true); stickyIssue(C.g.player, 2); C.W.restore(s0);
    F.verdict(s0.player.rifle.cams === -1 && s0.player.cams === undefined && !C.g.player.rifle.attachments.includes('cam') && C.g.player.rifle.cams === 0, 'a file made with none issued says so, and loading it leaves the rifle with its two loads', `file: rifle.cams ${s0.player.rifle.cams} · loaded loads ${C.g.player.rifle.attachments.join(' ')}`);
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { back(); Math.random = rnd; }
  return F.done();
}

// ================================================================================================ off
/** a stuck camera of your own is a row of the use key, standing at it, and is on the launcher again */
export async function stickyTake(): Promise<boolean> {
  const F = family('sticky-take: a stuck camera of your own is taken back with the use key and goes on the launcher again');
  const rnd = Math.random;
  const mark = (g: any) => g.interactables.find((x: any) => x.kind === 'cam'), rowOf = (g: any) => g.nearby.find((r: any) => r.kind === 'cam' && r.verb === 'takeCam');
  const camEyes = (g: any) => g.fogOfWar.eyes.filter((e: any) => e.id >= 200).length;
  const tap = (W: any) => { const ok = W.game.selectVerb('cam', 'takeCam'); if (ok) { W.input.pressed.add('Space'); W.step(DT); } return ok; };
  try {
    const { W, g } = await quietFloor(23), fog = g.fogOfWar, pl = g.player; stickyIssue(pl, 2);
    let pick: any = null, w: ReturnType<typeof wallFor> = null;
    for (const gd of g.guards.filter((gd: any) => gd.state !== 'dead' && !fog.seen(gd))) { w = wallFor(g, gd); if (w) { pick = gd; break; } }
    F.verdict(!!pick && !!w, 'there is a guard nobody sees with a wall in his sight', `picked ${pick?.callsign}`);
    if (pick && w) {
      const at = (d: number): Vec3 => [w!.P[0] + w!.n[0] * d, 0, w!.P[2] + w!.n[2] * d], go = (p: Vec3) => { g.teleportPlayer(p); steps(W, 4); return v3.distXZ(pl.char.pos, w!.P) as number; };
      const shoot = () => { const c = send(W, w!.from, v3.mad(w!.P, w!.n, -0.5) as Vec3); pl.rifle.cams--; steps(W, 20); return c; };   // (player.ts spends one as it fires)
      const cam = shoot();
      const far = v3.distXZ(pl.char.pos, cam.pos) as number, m0 = mark(g);
      F.verdict(!cam.vel && pl.cams.length === 1 && pl.rifle.cams === 1 && camEyes(g) === 1 && far >= 3 && !m0, 'one sent and stuck: one left on the launcher, one eye on the fog, and from across the floor nothing is offered', `stuck ${!cam.vel} · on the launcher ${pl.rifle.cams} · camera eyes ${camEyes(g)} · Sam ${far.toFixed(1)} m off, marker ${!!m0}`);
      // through its own wall: within the reach on the plan, no line → nothing
      const dB = go(at(-0.7)), eye: Vec3 = [pl.char.pos[0], 1.3, pl.char.pos[2]], wallBetween = g.col.solidBlocked(eye, camLens(cam)), mB = mark(g);
      F.verdict(wallBetween && dB < STICKY.takeReach && !mB && !tap(W) && pl.cams.length === 1, 'from the far side of its wall it is not offered and Space takes nothing', `Sam ${dB.toFixed(2)} m from it on the plan · wall between ${wallBetween} · marker ${!!mB} · out ${pl.cams.length}`);
      // in sight, out of reach: a marker that says step closer, no row
      let d2 = NaN;   // (a spot in front of it, past the reach and inside the marker's range, with a clear line: furniture may stand on the normal itself)
      for (const [d, side] of [[2.2, 0], [1.9, 0], [2.5, 0], [2.0, 0.6], [2.0, -0.6], [1.7, 0.9], [1.7, -0.9]] as const) {
        const q = at(d); go([q[0] - w.n[2] * side, 0, q[2] + w.n[0] * side]); const dd = v3.distXZ(pl.char.pos, cam.pos) as number;
        if (dd > STICKY.takeReach + 0.15 && dd < 2.9 && !g.col.solidBlocked([pl.char.pos[0], 1.3, pl.char.pos[2]], camLens(cam))) { d2 = dd; break; }
      }
      const m2 = mark(g), r2 = rowOf(g), t2 = tap(W);
      F.verdict(!!m2 && !m2.inReach && m2.line2 === 'step closer' && !r2 && !t2 && pl.cams.length === 1, 'in sight but out of reach: a marker that says step closer, no row, Space takes nothing', `Sam ${d2.toFixed(2)} m · marker ${!!m2} "${m2?.line2}" in reach ${m2?.inReach} · row ${!!r2} · tap ${t2} · out ${pl.cams.length}`);
      // at it: the row
      const d1 = go(at(0.8)), r1 = rowOf(g);
      F.verdict(!!r1 && r1.line2 === 'Space  take it back' && r1.line1 === 'sticky camera' && pl.cams.length === 1, 'standing at it: one row, "take it back"', `Sam ${d1.toFixed(2)} m · row "${r1?.line1} / ${r1?.line2}" · out ${pl.cams.length}`);
      // the tap: off the wall, on the launcher, its eye gone
      const taken0 = SC.STICKY_STATS.taken, t1 = tap(W); steps(W, 3);
      F.verdict(t1 && pl.cams.length === 0 && pl.rifle.cams === 2 && pl.rifle.attachments.includes('cam') && camEyes(g) === 0 && SC.STICKY_STATS.taken === taken0 + 1 && !mark(g),
        'Space on the row: the camera is off its wall and on the launcher, its eye is off the fog, the row is gone', `tap ${t1} · out ${pl.cams.length} · on the launcher ${pl.rifle.cams} (${pl.rifle.attachments.join()}) · camera eyes ${camEyes(g)} · taken ${SC.STICKY_STATS.taken - taken0}`);
      // reuse: sent again it is a new camera and shows him again; both sent, the load is dry and taking one back loads it
      go(at(6)); const again = shoot(); const seenAgain = fog.seen(pick), newId = again.id > cam.id;
      const second = shoot(); const dry = pl.rifle.cams, listed = pl.rifle.attachments.includes('cam'), eyes2 = camEyes(g);
      go(at(0.8)); const want3 = SC.camToTake(g, pl), t3 = tap(W); steps(W, 3);
      F.verdict(!!want3 && !pl.cams.includes(want3) && !again.vel && seenAgain && newId && !second.vel && dry === 0 && listed && eyes2 === 1 && t3 && pl.cams.length === 1 && pl.rifle.cams === 1 && camEyes(g) === 1,
        'reuse: the one taken back is sent again and shows him again; with both out the load is dry, and taking one back loads it — the one still out sends (one live feed)', `sent again: stuck ${!again.vel}, ${pick.callsign} seen ${seenAgain}, a new id ${newId} · both out: on the launcher ${dry}, load listed ${listed}, camera eyes ${eyes2} · taken: tap ${t3}, out ${pl.cams.length}, on the launcher ${pl.rifle.cams}, camera eyes ${camEyes(g)}`);
      // hands full: no row
      go(at(0.8)); const walk = SC.camToTake(g, pl); pl.sprinting = true; const direct = SC.camToTake(g, pl); pl.sprinting = false;
      let full: any = null; pl.sprinting = true; SC.stickyRows(g, pl, false, (b: any, v: any, t: any) => ({ ...b, verb: v, line2: t }), (it: any, verbs: any[]) => { full = { it, verbs }; }); pl.sprinting = false;
      F.verdict(!!walk && direct === null && full?.it.line2 === 'hands full' && full.verbs.length === 0, 'sprinting past it takes nothing, and the marker says hands full', `camToTake at a walk: ${walk ? 'a camera' : 'none'} · sprinting: ${direct ? 'a camera' : 'none'}, marker "${full?.it.line2}", rows ${full?.verbs.length}`);
      // the launcher already full (unlimited ammo spends none): the one taken back does not go past the carry; the tick it makes
      pl.rifle.cams = STICKY.carry; const evs: any[] = [], evList = g.events, push0 = evList.push; evList.push = function (...e: any[]) { evs.push(...e); return push0.apply(evList, e); };
      const t4 = tap(W); evList.push = push0; const tick = evs.find((e: any) => e.kind === 'prop' && e.level === STICKY.takeNoise);
      F.verdict(t4 && pl.cams.length === 0 && pl.rifle.cams === STICKY.carry && !!tick && tick.who === pl.char.id && STICKY.takeNoise > 0.08 && STICKY.takeNoise < STICKY.stickNoise,
        'a full launcher stays at the carry, and the take is a small sound at his feet: over what a calm man hears from, under the landing\'s', `tap ${t4} · out ${pl.cams.length} · on the launcher ${pl.rifle.cams} of ${STICKY.carry} · tick ${tick ? `level ${tick.level}, his own ${tick.who === pl.char.id}` : 'none'} (hearing starts over 0.08; the landing ${STICKY.stickNoise})`);
      // a checkpoint's slices after a take: the cameras out and the count come back as they stood
      go(at(6)); shoot(); const snap = SC.stickySnapshot(pl), left = SC.stickyLeft(pl), seq = pl.camSeq, out0 = pl.cams.length, on0 = pl.rifle.cams; pl.cams.length = 0; pl.rifle.cams = 0;
      SC.stickyRestore(pl, snap, left, seq);
      F.verdict(out0 === 1 && pl.cams.length === 1 && pl.rifle.cams === on0 && pl.camSeq === seq, 'after a take a checkpoint\'s slices give back the camera out and the count', `out ${out0} → ${pl.cams.length} · on the launcher ${on0} → ${pl.rifle.cams} · seq ${seq} → ${pl.camSeq}`);
      // at a door: a camera lying on the floor by it is one row PAST the door's own — the door's rows as they were, the camera's last, the bare tap still the door's
      pl.cams.length = 0; pl.rifle.cams = 1; let doorCase = 'no shut door with rows found';
      door: for (const d of g.doors.list) {
        if (Math.abs(d.angle) > 0.01) continue;
        for (const [dx, dz] of [[0.8, 0], [-0.8, 0], [0, 0.8], [0, -0.8]]) {
          g.teleportPlayer([d.frameCentre[0] + dx, 0, d.frameCentre[2] + dz]); steps(W, 4);
          const key = (r: any) => `${r.kind}:${r.verb}:${r.line2}`, before = g.nearby.filter((r: any) => r.verb).map(key); if (before.length < 2 || g.nearby.some((r: any) => r.kind !== 'door')) continue;
          pl.cams.push({ pos: [pl.char.pos[0] + dx * 0.3, 0.02, pl.char.pos[2] + dz * 0.3], n: [0, 1, 0], vel: null, age: 0, si: -1, id: ++pl.camSeq }); steps(W, 2);
          const rows = g.nearby.filter((r: any) => r.verb), after = rows.filter((r: any) => r.kind !== 'cam').map(key), last = rows[rows.length - 1], hov = g.hover;
          const same = before.join('|') === after.join('|'), tookIt = tap(W); steps(W, 2);
          F.verdict(same && last?.kind === 'cam' && rows.filter((r: any) => r.kind === 'cam').length === 1 && hov?.kind === 'door' && tookIt && pl.cams.length === 0 && pl.rifle.cams === 2,
            'at a door a camera on the floor is one row past the door\'s own: the door\'s rows as they were, the bare tap still the door\'s, and it is taken off the floor', `door rows ${before.length} before, ${after.length} with it, the same ${same} · last row ${last?.kind} · the bare tap's row ${hov?.kind}:${hov?.verb} · taken ${tookIt}, on the launcher ${pl.rifle.cams}`);
          doorCase = ''; break door;
        }
      }
      if (doorCase) F.verdict(false, 'at a door a camera on the floor is one row past the door\'s own', doorCase);
    }
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { back(); Math.random = rnd; }
  return F.done();
}

// ================================================================================================ mark
/** his own stuck cameras show in the fog of war — never taken out of the picture, a lens on each on the HUD */
export async function stickyMark(): Promise<boolean> {
  const F = family('sticky-mark: your own stuck sticky cameras show in the fog of war');
  const coneWas = FOG.cone, ownWas = FOG.own, rnd = Math.random;
  const { quitScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`), games: any[] = [];   // (every mission this part starts is quit at its end: the rung's cone is the visit's, not the next probe's)
  const FILE = 'vault.json', RUNG = 'cones';   // (the floor and the rung the maintainer was on when he said it)
  /** two walls for two cameras: neither spot in any eye of the team's now, the second's lens with no line to the first */
  const twoWalls = (g: any) => {
    const fog = g.fogOfWar, all: NonNullable<ReturnType<typeof wallFor>>[] = [];
    for (const gd of g.guards) { const w = wallFor(g, gd); if (w && !fog.thingSeen(w.P[0] + w.n[0] * 0.06, w.P[2] + w.n[2] * 0.06, 0, 0)) all.push(w); }
    for (const a of all) for (const b of all) if (v3.dist(a.P, b.P) > 3 && g.col.segmentBlocked(v3.mad(b.P, b.n, STICKY.off), v3.mad(a.P, a.n, STICKY.off))) return [a, b];
    return null;
  };
  const boxesOf = (g: any, c: any) => (g.dynBoxes as any[]).filter(b => Math.hypot(b.c[0] - c.pos[0], b.c[1] - c.pos[1], b.c[2] - c.pos[2]) < 0.05);
  const red = (b: any) => !!b && b.albedo[0] > 0.2;
  try {
    const { W, g } = await quietFloor(31, FILE, RUNG), fog = g.fogOfWar, pl = g.player; games.push(g); stickyIssue(pl, 2);
    const ww = twoWalls(g);
    F.verdict(fog.active && !!FOG.cone && !!ww, 'the vault, quiet, under the fog\'s cones: two walls no eye of the team has', `fog ${fog.active} · cone ${FOG.cone} · eyes ${fog.eyes.length} · walls ${ww ? 2 : 0}`);
    if (ww) {
      const A = send(W, ww[0].from, v3.mad(ww[0].P, ww[0].n, -0.5) as Vec3), B = send(W, ww[1].from, v3.mad(ww[1].P, ww[1].n, -0.5) as Vec3); steps(W, 40);
      const unseenA = !fog.thingSeen(A.pos[0], A.pos[2], 0, 0), eyeB = fog.eyes.some((e: any) => e.id >= 200);
      F.verdict(!A.vel && !B.vel && pl.cams.length === 2 && unseenA && eyeB && fog.eyes.length === 2, 'two cameras stuck: the newer sends, the older is dark on floor nobody sees (one feed at a time)', `out ${pl.cams.length} · the older one's floor seen ${!unseenA} · eyes ${fog.eyes.length} (a camera among them ${eyeB})`);
      // in the picture: the list the frame hands the renderer (Game.dynBoxes after the fog's own rule for loose things)
      const upA = boxesOf(g, A), upB = boxesOf(g, B), dev = (b: any) => b?.mkey !== undefined && (b.mkey >>> 24) === 5;
      F.verdict(upA.length === 2 && upB.length === 2 && upA.every(dev) && !red(upA[1]) && red(upB[1]),
        'both are in the picture — the dark one too, its lens black, the sending one\'s red',
        `older ${upA.length} boxes (keyed ${upA.every(dev)}, lens red ${red(upA[1])}) · newer ${upB.length} (lens red ${red(upB[1])})`);
      // the HUD's marks
      const near = (m: any, c: any) => !!m && v3.dist(m.at, camLens(c)) < 1e-6;
      const M = stickyMarks(g) as any[];
      F.verdict(M.length === 2 && near(M[0], A) && near(M[1], B) && M[0].sends === false && M[1].sends === true,
        'the HUD is handed a mark at each lens, the sending one told apart', `marks ${M.length} · sends ${M.map(m => m.sends).join(' ')} · at the lenses ${near(M[0], A)} ${near(M[1], B)}`);
      // one in the air carries no mark and no key; landed, it is the one that sends
      stickyIssue(pl, 1); const C = fireCam(g, pl, ww[0].from, v3.mad(ww[0].P, ww[0].n, -0.5) as Vec3); W.step(DT);
      const air = !!C.vel, mAir = stickyMarks(g).length, keyAir = (g.dynBoxes as any[]).some(b => b.mkey !== undefined && (b.mkey >>> 24) === 5 && b.mkey >= (5 << 24) + 0x800000 && Math.hypot(b.c[0] - C.pos[0], b.c[2] - C.pos[2]) < 0.2);
      let n = 0; while (C.vel && n++ < 180) W.step(DT); steps(W, 20); const M3 = stickyMarks(g) as any[];
      F.verdict(air && mAir === 2 && keyAir && !C.vel && M3.length === 3 && M3.filter(m => m.sends).length === 1 && near(M3.find(m => m.sends), C),
        'a third one in the air is in the picture too (keyed) and carries no mark; landed it is marked and is the one that sends', `in the air ${air}: marks ${mAir}, keyed ${keyAir} · landed: marks ${M3.length}, sending ${M3.filter(m => m.sends).length} (the newest ${near(M3.find(m => m.sends), C)})`);
      // a camera that is no longer out (taken back, retired) has no mark; a viewer's picture with the fog lifted has none
      const gone = pl.cams.splice(pl.cams.indexOf(A), 1)[0]; steps(W, 2); const M4 = stickyMarks(g) as any[], k0 = (5 << 24) + 0x800000 + (gone.id % 4096) * 2, goneBoxes = (g.dynBoxes as any[]).filter(b => b.mkey === k0 || b.mkey === k0 + 1).length; pl.cams.unshift(gone); steps(W, 2);   // (by its own keys: the third camera lies beside where it was)
      fog.bared = true; const mBared = stickyMarks(g).length; fog.bared = false;
      F.verdict(M4.length === 2 && goneBoxes === 0 && stickyMarks(g).length === 3 && mBared === 0, 'a camera no longer out has no mark and no box; with the fog lifted for a viewer nothing is marked', `one gone: marks ${M4.length}, its boxes ${goneBoxes} · back: ${stickyMarks(g).length} · fog lifted for a viewer: ${mBared}`);
      // no fog, no marks (the floor is in plain view) — and the fog back, they are back
      fog.on = false; steps(W, 2); const mNoFog = stickyMarks(g).length, allIn = pl.cams.every((c: any) => boxesOf(g, c).length >= 2); fog.on = true; steps(W, 45); const mBack = stickyMarks(g).length;
      F.verdict(mNoFog === 0 && allIn && mBack === 3, 'with no fog up nothing is marked (every camera is drawn anyway); the fog back, the marks are back', `no fog: marks ${mNoFog}, all drawn ${allIn} · fog back: marks ${mBack}`);
    }
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { for (const g of games) try { await hushed(() => quitScenario(g)); } catch { /* a game that never started one */ } back(); FOG.cone = coneWas; FOG.own = ownWas; Math.random = rnd; }
  return F.done();
}

/** the mission's own wand — handed out with the mission's cameras, to Sam, whatever his card carries */
export async function stickyWand(): Promise<boolean> {
  const F = family('sticky-wand: a quiet mission under the fog hands you an optical wand with its cameras');
  const WD = await import(`${ROOT}/src/game/wand.ts`), { quitScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const rnd = Math.random;
  try {
    await keepKit(async () => {
      K.kitLoad(null, ''); K.KIT.on = true;
      const open = async (query: string) => {
        const W = await stand(61, 'police-post.json'), g = W.game;
        await hushed(() => startFromQuery(g, query)); K.kitSoloIf(g, false); steps(W, 10);
        const fit = g.doors.list.filter((d: any) => WD.wandFits(d)), offered = fit.filter((d: any) => WD.wandOffered(g, g.player, d)).length;
        return { W, g, wand: !!g.player.wand, card: !!K.KIT.members[0].wand, cams: g.player.rifle.cams, fit: fit.length, offered, fog: !!g.fogOfWar.active };
      };
      const say = (r: any) => `wand ${r.wand} · cameras ${r.cams} · fog ${r.fog} · shut doors a lens fits ${r.fit}, offered at ${r.offered}`;
      const Q = '?levelFile=police-post.json&quiet=1', a = await open(BREAK ? `${Q}&quietfog=off` : Q);
      F.verdict(a.wand && !a.card && a.cams === STICKY.issue && a.fog && a.fit > 0 && a.offered === a.fit, 'a quiet mission opened with no click: the wand is on him though his card carries none, with the mission\'s cameras, and every shut door a lens fits offers the look', say(a));
      // the row itself, standing at such a door
      let rowAt = 'no door gave rows';
      door: for (const d of a.g.doors.list.filter((d: any) => WD.wandFits(d))) for (const [dx, dz] of [[0.8, 0], [-0.8, 0], [0, 0.8], [0, -0.8]]) {
        a.g.teleportPlayer([d.frameCentre[0] + dx, 0, d.frameCentre[2] + dz]); steps(a.W, 4);
        if (a.g.nearby.some((r: any) => r.kind === 'door' && r.verb === 'wand')) { rowAt = `${d.def.name}: ${a.g.nearby.filter((r: any) => r.verb).map((r: any) => r.verb).join(', ')}`; break door; }
        if (a.g.nearby.some((r: any) => r.kind === 'door')) rowAt = `${d.def.name} gave ${a.g.nearby.filter((r: any) => r.verb).map((r: any) => r.verb).join(', ')}: no look`;
      }
      F.verdict(rowAt.includes('wand'), 'standing at a shut door the look under it is one of its rows', rowAt);
      // the mission over: the plain night's man carries none
      await hushed(() => quitScenario(a.g)); steps(a.W, 10);
      F.verdict(!a.g.player.wand, 'the mission quit, the man it leaves carries no wand', `wand ${!!a.g.player.wand}`);
      // inside the mission: a restart keeps it; a checkpoint does not speak for a lent wand
      const b = await open(Q), s1 = b.W.snapshot('wand-lent', true), said = 'wand' in s1.player;
      await hushed(() => b.g.restartEncounter()); steps(b.W, 5); const kept = !!b.g.player.wand;
      F.verdict(b.wand && !said && kept, 'a restart inside the mission keeps it; a checkpoint saved with it lent does not say "wand"', `lent ${b.wand} · the file says wand: ${said} · after a restart ${kept}`);
      // the fog alone, no fog: no wand
      const alone = await open(`${Q}&cams=0`), none = await open(`${Q}&quietfog=off`);
      F.verdict(!alone.wand && alone.fog && alone.cams === 0 && !none.wand && !none.fog,
        'the fog alone (no cameras) hands out none, nor does a mission with no fog',
        `fog alone: ${say(alone)} | no fog: ${say(none)}`);
    });
  } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  finally { back(); Math.random = rnd; }
  return F.done();
}

// ================================================================================================ the default as it ships: the fog, the partner's eye, two cameras
export async function stickyDefault(): Promise<boolean> {
  const F = family('sticky-default: a quiet mission opened with no click is under the fog all round — your eye, your partner\'s — with the cameras on the rifle');
  const rnd = Math.random, G = globalThis as any, hadLs = 'localStorage' in G, lsWas = G.localStorage;
  try {
    const { QUIET_MISSIONS, LESSON_MISSIONS, SCENARIOS, pickSearch } = await import(`${ROOT}/src/scene/scenarioList.ts`), { quitScenario, fogOfQuery, quietRungOfQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
    const { withDom, El } = await import('./scenario-picker-probes.ts'), LV = await import(`${ROOT}/src/ui/levers.ts`);
    const store = memStore(); G.localStorage = store; QF.quietFogForget();
    const soloWas = K.KIT_SOLO.on;
    const two = STICKY.issue, withCams = `ring,he40,cam ×${two}`, has = (h: string) => /[?&](quietfog|cams)\b/.test(h);
    await keepKit(async () => {
      K.kitLoad(null, ''); K.KIT.on = true;
      const start = async (file: string, query: string) => { const W = await stand(51, file), g = W.game;
        await hushed(() => startFromQuery(g, query)); K.kitSoloIf(g, false); steps(W, 10);
        const read = () => `${g.player.rifle.attachments.join(',')}${g.player.rifle.attachments.includes('cam') ? ` ×${g.player.rifle.cams}` : ''}`;
        const at0 = read(), fog = !!g.fogOfWar.on, eyes = g.fogOfWar.eyes.length, rows = `${FOG.cone ? 'cone' : 'round'}${FOG.own ? '/own' : ''}`, mates = g.guards.filter((x: any) => x.unit && x.team === 'operative' && x.state !== 'dead').length;
        await hushed(() => g.restartEncounter()); steps(W, 10); const at1 = read();
        await hushed(() => quitScenario(g)); const after = read(); return { at0, at1, fog, eyes, rows, mates, after }; };
      // every quiet row of the list, by the link its row carries on a machine that has kept nothing
      const lines: string[] = []; let all = true;
      for (const e of QUIET_MISSIONS as any[]) {
        const search = pickSearch(e, ''), href = QF.quietFogHref(`/${search}`, true), q = new URLSearchParams(search);
        const r = await start(e.level, BREAK ? `${search}&quietfog=off` : search), want = 1 + (e.squad ? 1 : 0);
        const ok = !has(href) && fogOfQuery(q) && quietRungOfQuery(q) === 'team' && r.fog && r.rows === 'round' && r.eyes === want && r.mates === (e.squad ? 1 : 0) && r.at0 === withCams && r.at1 === withCams && r.after === 'ring,he40';
        all = all && ok; lines.push(`${e.id}: ${has(href) ? href : 'no key'}, fog ${r.fog} (${r.rows}), eyes ${r.eyes} of ${want}, ${r.at0}${r.at1 === r.at0 ? '' : ` → restart ${r.at1}`}${r.after === 'ring,he40' ? '' : ` → quit ${r.after}`}${ok ? '' : ' ✗'}`);
      }
      F.verdict(all && (QUIET_MISSIONS as any[]).length > 0, 'every quiet mission of the list, by its row\'s own link with nothing kept: no key on the link, the fog all round, the player\'s eye and — as two — his partner\'s, the mission\'s cameras on the launcher, again after a restart — and none left on him once it is quit', lines.join(' · '));
      // the other ways are a key each; a lesson and a squad mission read none of it
      const P = '?levelFile=police-post.json&quiet=1', shown = await start('police-post.json', `${P}&quietfog=off`), bare = await start('police-post.json', `${P}&cams=0`), cones = await start('police-post.json', `${P}&quietfog=cones`), own = await start('police-post.json', `${P}&quietfog=own&cams=1`), nofow = await start('police-post.json', `${P}&nofow`);
      F.verdict(!shown.fog && shown.at0 === 'ring,he40' && bare.fog && bare.at0 === 'ring,he40' && cones.fog && cones.rows === 'cone' && cones.at0 === withCams && own.fog && own.rows === 'cone/own' && own.at0 === 'ring,he40,cam ×1' && !nofow.fog && nofow.at0 === 'ring,he40',
        'the other ways are a key each: quietfog=off = every guard shown and no camera, cams=0 = the fog alone, another rung keeps the cameras, cams=N hands N, the plain ?nofow = no fog and so no camera',
        `quietfog=off: fog ${shown.fog}, ${shown.at0} · cams=0: fog ${bare.fog}, ${bare.at0} · quietfog=cones: fog ${cones.fog} (${cones.rows}), ${cones.at0} · quietfog=own&cams=1: fog ${own.fog} (${own.rows}), ${own.at0} · nofow: fog ${nofow.fog}, ${nofow.at0}`);
      const les = (LESSON_MISSIONS as any[]).find(e => e.kind === 'quiet' && e.squad === 0), lq = pickSearch(les, ''), L = await start(les.level, lq);
      const sq = (SCENARIOS as any[]).find(e => e.kind === 'roomclear'), sqHref = QF.quietFogHref(`/${pickSearch(sq, '')}`, false);
      F.verdict(!L.fog && L.at0 === 'ring,he40' && quietRungOfQuery(new URLSearchParams(lq)) === 'off' && !has(sqHref) && !has(QF.quietFogHref(`/${lq}`, false)),
        'a tutorial mission played alone is as it was (every guard shown, no camera), and neither its link nor a squad row\'s carries a key', `${les.id}: fog ${L.fog}, ${L.at0}, rung ${quietRungOfQuery(new URLSearchParams(lq))} · squad row ${sq.id}: ${sqHref}`);
    });
    F.verdict(K.KIT_SOLO.on === soloWas, 'the starts above put your card on Sam (the page\'s own order) and leave the solo kit flag as they found it for whatever runs next', `KIT_SOLO.on ${soloWas} → ${K.KIT_SOLO.on}`);
    // the links and the lever say only what differs from the default
    const BASE = '/?quality=high&levelFile=police-post.json&quiet', href = () => QF.quietFogHref(BASE, true), keys = (h: string) => h.replace(/^[^?]*\?/, '').split('&').filter(k => /^(quietfog|cams)\b/.test(k)).join('&') || '(neither)';
    const kept = () => `${store.m.get('twopointsix.quietfog') ?? '-'}/${store.m.get('twopointsix.quietcams') ?? '-'}`;
    QF.quietFogForget(); store.m.clear(); const fresh = keys(href()), k0 = kept();
    QF.setQuietCams(false); const noCams = keys(href()), k1 = kept();
    QF.setQuietFog('off'); const allShown = keys(href()), k2 = kept();
    QF.setQuietFog('team'); const again = keys(href()), k3 = kept();
    QF.setQuietCams(true); const home = keys(href()), k4 = kept();
    QF.setQuietFog('own'); const ownL = keys(href()), k5 = kept(); QF.setQuietFog('team');
    F.verdict(fresh === '(neither)' && k0 === '-/-' && noCams === 'cams=0' && k1 === '-/0' && allShown === 'quietfog=off' && k2 === 'off/0' && again === 'cams=0' && k3 === '-/0' && home === '(neither)' && k4 === '-/-' && ownL === 'quietfog=own' && k5 === 'own/-',
      'a link and this machine\'s store say only what differs from the default: nothing kept = neither key; None = cams=0; all shown = quietfog=off (the None kept for when the fog is back); back on the default = nothing on the link, nothing kept; another rung = that rung alone',
      `fresh ${fresh} (kept ${k0}) · None ${noCams} (${k1}) · all shown ${allShown} (${k2}) · fog back ${again} (${k3}) · cameras back ${home} (${k4}) · own eyes ${ownL} (${k5})`);
    QF.quietFogForget(); store.m.clear();
    await withDom('?mute', async () => {
      const root = new El('DIV') as any, U = LV.leverRows(root, { row: 'mdiff mlever', label: 'dl', btn: 'dbtn', note: 'dn' }, () => { });
      const all = (f: (e: any) => boolean): any[] => (root as any).all(f);
      const fogRow = (): any => all(e => e.attrs['data-lever'] === 'fog')[0], camRow = (): any => all(e => e.attrs['data-lever'] === 'cams')[0];
      const rung = (id: string): any => fogRow().all((e: any) => e.attrs['data-rung'] === id)[0], cam = (on: boolean): any => camRow().all((e: any) => e.attrs['data-cams'] === (on ? '1' : '0'))[0];
      const lit = () => `${fogRow().all((e: any) => e.tagName === 'BUTTON' && e.cls.has('on')).map((b: any) => b.attrs['data-rung']).join()}/${camRow().all((e: any) => e.tagName === 'BUTTON' && e.cls.has('on')).map((b: any) => b.textContent).join()}${fogRow().cls.has('moved') ? ' fog-moved' : ''}${camRow().cls.has('moved') ? ' cams-moved' : ''}`;
      U.paint(true, '', true); const l0 = lit();
      cam(false).onclick(); const l1 = lit(); rung('off').onclick(); const l2 = lit(); rung('team').onclick(); const l3 = lit(); cam(true).onclick(); const l4 = lit();
      F.verdict(l0 === `team/${two}` && l1 === 'team/None cams-moved' && l2 === 'off/None fog-moved' && l3 === 'team/None cams-moved' && l4 === `team/${two}`,
        'the sheet\'s two rows on a quiet mission light the default with nothing marked moved; a press off it is marked, a press back is not', `fresh ${l0} → None: ${l1} → fog off: ${l2} → fog all round: ${l3} → cameras: ${l4}`);
    });
  } catch (e) { F.verdict(false, 'threw', String((e as Error)?.stack ?? e)); }
  finally { back(); QF.quietFogForget(); if (hadLs) G.localStorage = lsWas; else delete G.localStorage; Math.random = rnd; }
  return F.done();
}

/** the file's probes, by the names the runner calls them */
const PROBES = { 'sticky-kit': stickyKit, 'sticky-fly': stickyFly, 'sticky-fog': stickyFog, 'sticky-save': stickySave, 'sticky-default': stickyDefault, 'sticky-take': stickyTake, 'sticky-mark': stickyMark, 'sticky-wand': stickyWand };
if (import.meta.main) process.exit(exitCode(await asProbes(PROBES, wanted())));
