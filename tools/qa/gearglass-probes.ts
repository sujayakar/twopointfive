// The 'gearglass' family (`bun run tools/qa/probes.ts gearglass | gearglass-joined | gearglass-pinned`): what the three gear-and-glass parts could not check
// alone — BOTH records of a pane live at once (the fracture model's, which a burst writes: game/frag.ts → glassbreak.ts blast; and a shell's own, which a load
// of shot writes: shotgun.ts Shell.glass → glasspen.ts, the shell rule) — and a pinned defender's fire through glass.
// PASS / FAIL each, all headless on the real game with the glass parts ON (raised by name, put back after); staged on the conference room's glazing (three
// 1 × 2 m panes over a half-metre sill along z = 10, the room north of them, the corridor south). No picture and no sound.
//   gearglass-joined   (a) a shell at a frame a burst has EMPTIED (toughened glass, diced): nothing is filed — no row in the shell's record, no blow, no hole
//                      remembered, no 'glass' event — and the nine fly on to the man beyond. (b) a shell at a float pane a burst has PARTLY taken: every row
//                      of the shell's record lies on glass that stood before the pull, the record obeys the shell rule, the fracture model was given exactly
//                      its marks and the holes exactly the marks that went through, and all nine still end beyond the glass. (c) a burst beside a pane a
//                      shell has HOLED: one blow, the pane ending exactly where the fracture model's own blast puts that holed pane; the burst has a line
//                      through the shell's hole before it goes off; and over a grid of points beyond the glazing a line the blast takes as open is open
//                      once the glass has fallen, and no other. Also: the same shell four frames behind the burst (pieces still queued), a shell on a laminated sheet a
//                      burst has starred (marks and joins, the energy on top of the blast's), and the grenade's own report of a man behind a shell's hole
//   gearglass-pinned   a room defender held on his post (pinned: the alert branch never asks him clearShot) with a lit man in plain view behind standing
//                      glass and nothing else in the line: he fires through it. ("Defenders shoot through glass" is a settled rule: until 2026-11-07 three
//                      lines here asked the page under ?glassfoes=0, where he held his fire)
import { standUp, ROOT } from './headless.ts';
import { CONF_PANES } from './office-pane.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { FRAG, FRAG_LOG, fragStatsReset, burstLine, exposure } = await import(`${ROOT}/src/game/frag.ts`);
const { GB_STATS } = await import(`${ROOT}/src/game/glassbreak.ts`);
const { GLASSPEN, GLASSPEN_STATS, holeCount, shotEnergy } = await import(`${ROOT}/src/game/glasspen.ts`);
const { GLASSFX, GLASS } = await import(`${ROOT}/src/game/glass.ts`);
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { SHOTGUN_LAST } = await import(`${ROOT}/src/game/shotgun.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}

/** the conference room's glazing along z = 10 (its three sheets: x 6‥7, 7‥8, 8‥9; y 0.5‥2.5) */
const [PW, PM, PE] = CONF_PANES, PANES = [PW, PM, PE], GZ = 10;   // (by where they stand, office-pane.ts: the door-size export moved each index up one)
const FRAG_DEFAULTS = JSON.parse(JSON.stringify(FRAG));
const SWITCHES = { random: Math.random };
/** every probe leaves the module switches and knobs as it found them (the runner's next probe stands up on them) */
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } finally { Object.assign(FRAG, JSON.parse(JSON.stringify(FRAG_DEFAULTS))); Math.random = SWITCHES.random; } };

/** One office under a seed with the glass parts on (fracture, rounds through glass, shards as the stand-up leaves them), AI off, Sam out of it. */
async function rig(seed: number) {
  Object.assign(FRAG, JSON.parse(JSON.stringify(FRAG_DEFAULTS))); fragStatsReset(); for (const k of Object.keys(GB_STATS)) (GB_STATS as any)[k] = 0;
  Math.random = seeded(seed);
  const W: any = await standUp(); const g = W.game;
  g.tune.glassBreak = true; g.tune.chaos = 0.5;
  g.aiEnabled = false; g.godMode = true; g.teleportPlayer([24, 0, 17.6]);
  const gb = g.glassBreak; const seen = new WeakSet<any>(), events: any[] = [];
  const R = { W, g, gb, events,
    step(n = 1) { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); for (const e of g.events) { if (!seen.has(e)) { seen.add(e); events.push(e); } } } },
    settle(secs: number) { R.step(Math.round(secs * 60)); },
    types(t: string) { for (const bi of PANES) gb.setType(bi, t); },
    put(gd: any, p: number[], face?: number[]) { gd.char.pos = [p[0], 0, p[2]]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.lastPos = [p[0], 0, p[2]]; if (face) { const y = Math.atan2(face[0] - p[0], face[2] - p[2]); gd.char.bodyYaw = y; gd.char.aimYaw = y; } },
    park(...keep: any[]) { let k = 0; for (const x of g.guards) if (!keep.includes(x)) R.put(x, [60 + 2 * k++, 0, 60]); },
    /** a grenade on a short fuse at p; the blast's report once it has gone */
    frag(p: number[]): any { const n0 = FRAG_LOG.length; g.items.spawn('frag', [p[0], 0.05, p[2]], [0, 0, 0], { fuse: 0.2, who: 1 }); for (let i = 0; i < 120 && FRAG_LOG.length === n0; i++) R.step(); R.step(1); return FRAG_LOG[n0] ?? null; },
    /** ONE PULL of the M870 from a muzzle put at `from`, aimed at `to`, the shoulder a forearm behind it on the line: the player's own round */
    shell(from: Vec3, to: Vec3) { const c = g.player.char, m0 = c.muzzle, a0 = c.bones.upperArmR, d = v3.normalize(v3.sub(to, from));
      c.muzzle = [...from] as Vec3; c.bones.upperArmR = v3.mad(from, d, -0.3);
      try { fireWeapon(g, c, to, true, { weapon: (WEAPONS as any).m870 }); } finally { c.muzzle = m0; c.bones.upperArmR = a0; }
      R.step(1); },
    open(bi: number): number { const i = gb.paneInfo(bi); return i ? (i.gone ? 1 : i.openFraction) : 0; },
    pane(bi: number) { const i = gb.paneInfo(bi); return i ? `${bi} ${i.type} ${Math.round(100 * i.openFraction)}% open${i.gone ? ' GONE' : ''}${i.run ? ' run' : ''}` : `${bi} —`; },
    /** the fracture model's state of a pane, as one comparable line */
    state(bi: number): string { const i = gb.paneInfo(bi), st = gb.panes.get(bi); return i ? `${i.type} open ${i.openFraction.toFixed(4)} gone ${i.gone} run ${i.run} sites ${i.sites} energy ${i.energy.toFixed(4)} damage ${(st?.damage ?? 0).toFixed(4)}` : 'none'; },
    hits(): number { let n = 0; for (const bi of PANES) n += gb.panes.get(bi)?.hits ?? 0; return n; },
    holes(): number { let n = 0; for (const bi of PANES) n += holeCount(g, bi); return n; },
    glassEvents(): number { return events.filter((e: any) => e.kind === 'glass' && e.level === GLASS.hearLevel).length; },
  };
  R.step(2);
  return R;
}
type Rig = Awaited<ReturnType<typeof rig>>;
/** pellets of the last shell that ended north of the glazing (beyond it, fired from the corridor) */
const beyond = () => (SHOTGUN_LAST.ends as any[]).filter(e => e.at[2] < GZ - 0.05).length;

/** one man stood 1.5 m north of the glazing at x, facing it */
function behind(R: Rig, x: number): any { const B = R.g.guards[1]; R.put(B, [x, 0, 8.5], [x, 0, 12]); B.char.update(0); B.char.update(DT); return B; }

// ================================================================ gearglass-pinned
/** a room defender put north of the conference glazing and held there, a lit Sam in the corridor south of it in plain view, nobody else near; six seconds */
async function pinnedCase(seed: number, type = 'annealed') {
  const R = await rig(seed); const g = R.g; g.aiEnabled = true; g.godMode = true;
  g.roomClear.start(seed); R.step(32); R.types(type);
  const C = g.nightBreach.hostiles[0]; const CAR = [7.4, 0, 5.3], T1 = [7.3, 0, 11.2];
  g.guards.filter((x: any) => x !== C).forEach((x: any, i: number) => R.put(x, [60 + 2 * i, 0, 60]));
  C.god = true; R.put(C, CAR, T1); g.teleportPlayer(T1);
  let shots = 0, alertT = 0, searchT = 0, pinnedT = 0, energy: number | undefined = NaN, firstT = NaN; const t0 = g.time, n0 = R.events.length, imp0 = GB_STATS.impacts, x0 = GLASSPEN_STATS.crossed;
  for (let i = 0; i < 6 * 60; i++) {
    C.char.pos = [CAR[0], 0, CAR[2]]; C.char.vel = [0, 0, 0]; g.player.visibility = 1; R.W.step(DT);
    for (const e of g.events) { if (R.events.includes(e)) continue; R.events.push(e); if (e.kind === 'guardShot' && e.who === C.char.id) { shots++; if (!(firstT === firstT)) firstT = g.time - t0; } }
    if (C.state === 'alert') { alertT += DT; energy = shotEnergy(g, C.char, C.weapon.def); if (C.pinned) pinnedT += DT; } else if (C.state === 'search') searchT += DT;
  }
  void n0;
  return { R, g, C, shots, alertT, searchT, pinnedT, energy, firstT, impacts: GB_STATS.impacts - imp0, crossed: GLASSPEN_STATS.crossed - x0, whole: PANES.every(bi => R.open(bi) === 0),
    line: `${C.callsign} ${C.state} (alert ${f2(alertT)} s, pinned ${f2(pinnedT)} s of it, searching ${f2(searchT)} s) · his gun's energy for a pane ${energy === undefined ? 'the default' : f2(energy as number)} · rounds fired ${shots}${shots ? ` (first at ${f2(firstT)} s)` : ''} · panes crossed ${GLASSPEN_STATS.crossed - x0} · blows on glass ${GB_STATS.impacts - imp0} · ${PANES.map(bi => R.pane(bi)).join(' | ')}` };
}
/** one seed's answer, printed for the process that asked (below) */
export async function pinnedSeed(seed: number) { const a = await pinnedCase(seed); process.stdout.write(`PINNED ${JSON.stringify({ through: a.shots > 0 && a.crossed > 0 && (a.energy ?? 0) > 0, line: a.line })}\n`); }
// A process of its own a seed: the man the room clear stages is made by the Rust game, a module of the copy keeps him, and the next game stood in that process
// cannot export him ("SkeletonInstance.rig: no object for RigId"). The question is one man's, and each seed asks it on a game of its own.
export const gearglassPinned = guarded(async () => {
  const F = family('gearglass-pinned: a PINNED defender fires through standing glass at a man he sees');
  const SEEDS = [3, 5, 8];
  { let through = 0; const lines: string[] = [];
    for (const s of SEEDS) {
      const r = Bun.spawnSync(['bun', '-e', `await (await import(${JSON.stringify(import.meta.path)})).pinnedSeed(${s}); process.exit(0)`], { cwd: ROOT, env: process.env as Record<string, string>, stdout: 'pipe', stderr: 'pipe' });
      const row = r.stdout.toString().split('\n').find(l => l.startsWith('PINNED '));
      if (!row) { lines.push(`seed ${s}: no answer (exit ${r.exitCode}): ${r.stderr.toString().trim().split('\n')[0]?.slice(0, 200)}`); continue; }
      const a = JSON.parse(row.slice(7)); if (a.through) through++; lines.push(`seed ${s}: ${a.line}`); }
    F.verdict(through === SEEDS.length, 'pinned on his post, alert on a lit man he sees through the glazing: he fires through the pane', `${through}/${SEEDS.length}\n    ${lines.join('\n    ')}`); }
  return F.done();
});
