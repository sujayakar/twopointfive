// THE MEN LEFT ALIVE ON A RESCUE COME FOR THE HOSTAGES — the check behind src/game/rescueflush.ts (RESCUEFLUSH.on: on in the page, ?norescueflush; off on
// the frozen stand, TPS_RESCUEFLUSH=1).
//   bun run tools/qa/rescueflush-check.ts      (also `probes.ts rescueflush`; run it on BOTH stands: plain, and with TPS_PROFILE=shipped in front)
//   RESCUEFLUSH_SEEDS=7,8,9                    the dice each floor is staged under (default 7)
// A sample of the mission list's rescues (RESCUES; QA_SWEEP=all for each) is staged as tools/qa/answers.ts stages a floor (the mission's own row, no teammates, Sam out of everybody's
// sight and unhurt, so NOBODY stops a man of theirs) and the check's own hand does what a team would have done: one loud round at the hostage door, the
// men posted with the hostages put down. What the men left alive then do is theirs. The switch is thrown through the stand's own word for it.
//   (a) THEY COME: the first man's word is RESCUEFLUSH.afterSecs after the room falls, the next gapSecs after it; every man of theirs alive outside the
//       room is sent inside the time that takes plus 12 s; where the room has a side door the second word names it;
//   (b) THE GUN ON HER: with nobody to stop him a man who came turns his gun on a hostage and she is shot, inside 40 s of the fall — and she is logged
//       'executed', never the team's own fire;
//   (c) NOT WHEN QUIET: the same men put down without a sound and no loud thing done — past the first word's time nobody has been sent and nobody has been shot;
// NOT ASKED HERE: what a man sent does at a corridor the team's guns hold — his walk is the fight AI's, priced and gated by the lanes like any other
// (`probes.ts corridor` holds the lanes' own rules; this floor has no team to hold one).
// exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: a word on the net is a string, a line to a hostage is the collision's segment test. Nobody has seen or heard any of this in play.
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const RESCUES = ['hostage-rescue', 'rescue-registry', 'rescue-consulate'], DT = 1 / 60;
import { sample } from './sweep.ts';
const SEEDS = (process.env.RESCUEFLUSH_SEEDS ?? '7').split(',').map(Number).filter(Number.isFinite);
const f1 = (x: number | undefined) => typeof x === 'number' && Number.isFinite(x) ? x.toFixed(1) : '—';
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

async function pin<T>(words: Record<string, boolean>, fn: () => Promise<T>): Promise<T> {
  const was: Record<string, string | undefined> = {};
  for (const [k, on] of Object.entries(words)) { was[k] = process.env[k]; process.env[k] = on ? '1' : '0'; }
  try { return await fn(); } finally { for (const [k, v] of Object.entries(was)) { if (v === undefined) delete process.env[k]; else process.env[k] = v; } }
}

export async function rescueflushCheck(): Promise<boolean> {
  const { RESCUEEND } = await import(`${ROOT}/src/game/rescueend.ts`), { RESCUEFLUSH } = await import(`${ROOT}/src/game/rescueflush.ts`);
  const { ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const { answerStand, seeded } = await import('./answers.ts');
  const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
  const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`), { lanesRestore, laneKnownAt, laneCovers, LANES } = await import(`${ROOT}/src/game/lanes.ts`);
  const was = { end: { ...RESCUEEND }, flush: { ...RESCUEFLUSH }, lanes: LANES.on }, rnd = Math.random; let all = true;
  const BREAK = process.env.RESCUEFLUSH_BREAK === '1', K = RESCUEFLUSH;
  /** has this mission's hostage room a side door? (asked of its level's own scenario data once a floor stands: (h) is for those) */
  const sides = new Map<string, boolean>(), sideOf = (id: string) => sides.get(id) ?? false;
  const line = (ok: boolean, what: string, detail: string) => { if (!ok) all = false; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  /** one floor: the mission staged under `seed`; `loud`: a round of Sam's at the hostage door before the room's own men are put down (else they go down
   *  without a sound); `out`: the hostages stood at the way out first */
  const floor = async (id: string, seed: number, on: boolean, o: { loud: boolean; out?: boolean; end?: boolean; lanes?: boolean }) => await pin({ TPS_RESCUEFLUSH: on, TPS_RESCUEEND: o.end ?? true, ...(o.lanes ? { TPS_LANES: true } : {}) }, async () => {   // (word left out on purpose: `lanes` not asked is the covered lanes as the stand has them)
    Math.random = seeded(seed);
    const E = (ALL_MISSIONS as any[]).find(e => e.id === id), { W, g, why } = await answerStand(E); if (why) throw new Error(`${id}: ${why}`);
    const R = g.rescue, B = R.bounds, log = console.log, inB = (p: number[]) => p[0] >= B.x0 && p[0] <= B.x1 && p[2] >= B.z0 && p[2] <= B.z1;
    const shotAt: { who: string; at: number[]; by: string | null; byAt: number[] | null }[] = []; let acts = 0;
    const step = (s: number, until?: () => boolean) => { console.log = () => {}; try { for (let i = 0; i < Math.round(s / DT); i++) {
      const up = R.hostages.filter((h: any) => h.state !== 'dead'), cap = R.captor();
      W.step(DT);
      for (const h of up) if (h.state === 'dead') shotAt.push({ who: h.callsign, at: [...h.char.pos], by: cap?.callsign ?? null, byAt: cap ? [...cap.char.pos] : null });
      acts = R.log.filter((l: any) => l.what === 'act').length;
      if (until?.()) return true; } return false; } finally { console.log = log; } };
    step(1);
    if (o.out) { const O = R.ending.out; R.hostages.forEach((h: any, i: number) => { const p = g.col.nav.nearestFreePoint(O[0] + 0.8 * i, O[2] + 0.6 * (i % 2)) ?? O; h.char.pos[0] = p[0]; h.char.pos[2] = p[2]; h.civ.cowerAt = [p[0], 0, p[2]]; }); }
    if (o.loud) { const P = R.cast().find((c: any) => c.inRoom).gd.char.pos; g.events.push({ kind: 'shot', pos: [P[0], 0, P[2]], time: g.time, loud: true, level: 1, who: PLAYER_ID }); step(0.5); }
    for (const c of R.cast()) if (c.inRoom) killGuard(g, c.gd, [1, 0, 0], !o.loud, o.loud ? 'shot' : 'struck', 'player');
    const t0 = g.time; step(2 * DT);
    const lines = (re: RegExp) => R.log.filter((l: any) => re.test(l.what)).map((l: any) => ({ t: l.t - t0, what: l.what as string }));
    return { W, g, R, B, inB, step, t0, lines, shotAt, acts: () => acts, up: (l: any[]) => l.filter(x => x.state !== 'dead') };
  });
  try {
    for (const id of sample(RESCUES, 1)) for (const seed of SEEDS) {
      const tag = `${id}${SEEDS.length > 1 ? ` · seed ${seed}` : ''}`, on = !BREAK;
      { // (a) (b): they come; the gun on her
        const F = await floor(id, seed, on, { loud: true }), { R } = F;
        const outside = F.up(R.hostiles).filter((h: any) => !F.inB(h.char.pos)), need = K.afterSecs + outside.length * K.gapSecs + 12;
        F.step(need);
        const words = F.lines(/^flush:(room|side|noise|out):/), held = F.lines(/^flush:lane:/), first = words[0], second = words[1];
        const named = new Set(words.map((l: any) => l.what.split(':')[2])), missed = outside.filter((h: any) => h.state !== 'dead' && !named.has(h.callsign)).map((h: any) => h.callsign);
        const side = !!R.data.side; sides.set(id, side);
        line(F.lines(/^flush:fell$/).length === 1 && !!first && Math.abs(first.t - K.afterSecs) < 0.2 && /^flush:room:/.test(first.what) && (outside.length < 2 || (!!second && second.t - first.t >= K.gapSecs - 0.05 && (!side || /^flush:side:/.test(second.what)))) && !missed.length,
          `(a) ${tag}: once it is loud and the room has fallen, every man left alive outside it is sent for the hostages, one after another`,
          `${outside.length} alive outside the room · first word ${first ? `${first.what} at ${f1(first.t)} s` : 'none'} (afterSecs ${K.afterSecs}) · second ${second ? `${second.what} at ${f1(second.t)} s` : 'none'} · turned to the other door by a lane ${held.length} · never sent: ${missed.join(', ') || 'nobody'}`);
        F.step(Math.max(0, 40 - need), () => F.shotAt.length > 0);
        const ex = F.lines(/^executed:/), tf = F.lines(/^teamfire:/), s0 = F.shotAt[0];
        line(F.acts() > 0 && ex.length > 0 && !tf.length && ex[0].t <= 40 && !!s0?.by, `(b) ${tag}: with nobody to stop him, a man who came shoots a hostage`,
          `'act' ${F.acts()} · ${ex[0] ? `${ex[0].what} at ${f1(ex[0].t)} s` : 'nobody shot'}${s0 ? ` by ${s0.by ?? '—'}` : ''} · the team's own fire logged ${tf.length} · result ${R.result ? `${R.result.state} (${R.result.why})` : 'none'}`);
      }
      { // (c) not when quiet
        const F = await floor(id, seed, on, { loud: false }); F.step(K.afterSecs + K.gapSecs + 2);
        const words = F.lines(/^flush:/);
        line(!words.length && !F.acts() && !F.shotAt.length, `(c) ${tag}: the room taken without a sound — nobody is sent and nobody is shot`,
          `flush lines ${words.length} · 'act' ${F.acts()} · shot ${F.shotAt.length}`);
      }
    }
  } finally { Object.assign(RESCUEEND, was.end); Object.assign(RESCUEFLUSH, was.flush); LANES.on = was.lanes; Math.random = rnd; }
  console.log(`\n=== rescueflush: ${all ? 'ALL PASS' : 'FAILED'}${BREAK ? ' (RESCUEFLUSH_BREAK=1: a failure is the expected result)' : ''} ===`);
  return all;
}
if (import.meta.main) process.exit((await rescueflushCheck()) ? 0 : 1);
