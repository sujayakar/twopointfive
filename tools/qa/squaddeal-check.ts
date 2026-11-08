// SQUADS FROM THE ROOMS — the check behind src/game/squaddeal.ts (SQUADDEAL.on: on in the page, ?nosquaddeal; off on the frozen stand, TPS_SQUADDEAL=1).
//   bun run tools/qa/squaddeal-check.ts        (also `probes.ts squaddeal`; run it on BOTH stands: plain, and with TPS_PROFILE=shipped in front)
// The switch is thrown by the check, and the radio check with it (enemysquads.ts SQUADS.radioCheck: the quiet answer is its men — a frozen stand without it
// has nobody miss a silent post, which is another check's business). The floors are staged as tools/qa/answers.ts stages them (the mission's own row, no
// teammates, Sam out of everybody's sight), and (c) is that tool's own reading of ONE level of LEVELS:
//   (a) THE CUT, on a sample of the levels as its mission deals it: squads are staged; every room of the structure is in exactly one of them; every squad has
//       a man; as many squads as the men call for (`one` or fewer: one · `two` or fewer: two · more: three); their names are the table's; the room clear's
//       own squads check (roomclear.ts checkSquads) has nothing to say against the block;
//   (b) THE FILE WINS: a level that names its squads is handed back as it is (the same object), and its squads are the file's;
//   (c) A LOUD WAY IN AT THE FRONT draws men to the noise, their word on the net `LEAD_S` or more before the first of them is there;
//   (g) THE MISSION SHEET SAYS SO: Hard on a level the rule cuts into squads is worded as the squads' (difficulty.ts squadsHeld); the row's own fact (squadsOf: what its level FILE says) is untouched either way.
// SQUADDEAL_BREAK=1 = the rule left OFF for (a) (c) (g): each must fail.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: a line on the net is a string, a boot is an event. Nobody has seen or heard any of this in play.
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
import { sample } from './sweep.ts';
/** the listed missions whose level file names no squads (the office's own structure too: room-clear-pair has no file) */
const ROWS = ['depot-four', 'records-pair', 'killhouse-four', 'hall-four', 'floodlit-four', 'defuse-four', 'room-clear-pair'];
const FILE_WINS = 'warehouse-four', LEAD_S = 1.5;


export async function squaddealCheck(): Promise<boolean> {
  const { SQUADDEAL, squadDeal } = await import(`${ROOT}/src/game/squaddeal.ts`);
  const { SQUADS, squadsReport, squadsLive } = await import(`${ROOT}/src/game/enemysquads.ts`);
  const { checkSquads } = await import(`${ROOT}/src/game/roomclear.ts`);
  const { wallAt, WALLCHARGE } = await import(`${ROOT}/src/game/wallcharge.ts`), { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const { squadsOf, squadsHeld, difficultyWords } = await import(`${ROOT}/src/game/difficulty.ts`);
  const { answer, answerRow, answerSetup, answerStand, seeded, quiet } = await import('./answers.ts');
  const DT = 1 / 60, rnd0 = Math.random, was = { ...SQUADDEAL, names: [...SQUADDEAL.names] }, rc0 = SQUADS.radioCheck, walls0 = { d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer }; let all = true;
  const BREAK = process.env.SQUADDEAL_BREAK === '1';
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== squaddeal: a level whose file names no squads is handed some cut from its rooms ===');
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  /** the switches as this check runs them (a stand-up writes the held ones again: thrown after every one, and by `rule` inside a run of the answers tool) */
  const throwOn = (on: boolean) => { Object.assign(SQUADDEAL, was, { on, names: [...was.names] }); SQUADS.radioCheck = true; };
  const row = (id: string) => (SCENARIOS as any[]).find(e => e.id === id);
  const stand = async (id: string, on: boolean, seed = 7): Promise<any> => {
    Math.random = seeded(seed);
    const { W, why } = await answerStand(row(id), () => throwOn(on)); if (why) throw new Error(`${id} did not start: ${why}`);
    await quiet(async () => { for (let i = 0; i < 90; i++) W.step(DT); });
    return W;
  };
  try {
    // (a) the cut, level by level
    const on = !BREAK, bad: string[] = [], said: string[] = [];
    for (const id of sample(ROWS, 2)) {
      const W = await stand(id, on), g = W.game, S = g.roomClear.structure, R: any[] = squadsReport(g);
      if (!S) { bad.push(`${id}: no structure`); continue; }
      if (!squadsLive(g) || !R.length) { bad.push(`${id}: no squads staged`); continue; }
      const dealt = squadDeal(S, (g.roomClear.placement as any[]).map(p => p.room)), Q: any[] = dealt?.squads ?? [];
      const men = R.reduce((a, q) => a + q.men, 0), want = Math.min(men <= was.one ? 1 : men <= was.two ? 2 : 3, new Set((g.roomClear.placement as any[]).map(p => p.room)).size);
      const owner = new Map<string, number>(); for (const q of Q) for (const r of q.rooms ?? []) owner.set(r, (owner.get(r) ?? 0) + 1);
      if (R.length !== want) bad.push(`${id}: ${R.length} squads for ${men} men (the rule: ${want})`);
      if (R.some(q => q.men < 1)) bad.push(`${id}: a squad with no man (${R.map(q => `${q.name} ${q.men}`).join(', ')})`);
      if (S.rooms.some((r: any) => owner.get(r.id) !== 1)) bad.push(`${id}: a room in no squad or in two (${S.rooms.filter((r: any) => owner.get(r.id) !== 1).map((r: any) => r.id).join(', ')})`);
      if (R.some((q, i) => q.name !== was.names[i])) bad.push(`${id}: names ${R.map(q => q.name).join(', ')}`);
      const sound = checkSquads(g, dealt); if (sound.length) bad.push(`${id}: the room clear's own check says ${sound.join(' · ')}`);
      said.push(`${id} ${R.map(q => `${q.name} ${q.men}`).join(' + ')}`);
    }
    line(!bad.length, '(a) every listed level with no squads block is cut into squads: each room in one, a man in each, the count the men call for, the room clear\'s own check silent', bad.length ? bad.join('\n    ') : said.join(' · '));

    // (b) the file wins
    { const W = await stand(FILE_WINS, true), g = W.game, S = g.roomClear.structure, R: any[] = squadsReport(g);
      const same = squadDeal(S, (g.roomClear.placement as any[]).map(p => p.room)) === S, names = R.map(q => q.name), file = (S.squads as any[]).filter(q => !q.arrive).map(q => q.name ?? q.id);
      line(same && names.join() === file.join(), '(b) a level that names its squads keeps them: the structure is handed back as it is', `${FILE_WINS}: ${names.join(', ')} (the file: ${file.join(', ')})${same ? '' : ' — a copy was made'}`); }

    // (c) the loud way in, as the answers tool reads it, on the first level that draws men
    answerSetup({ seed: 7 });
    const rule = (v: boolean) => (g: any) => { void g; throwOn(v); };
    let loud: any = null, where = '';
    for (const id of [...new Set(['killhouse-four', 'hall-four', ...ROWS])]) {   // (the first two draw NOBODY to a loud way in at standard without the rule: the break path's (c) is read there)
      throwOn(on); const E = answerRow(id); if (!E) continue;
      // (the stand-up inside writes the held switches again: the hook throws ours the frame before the plan — too late for the staging, so the staging is
      // done under a stand whose env says so; see `pin`)
      const L = await pin(on, () => answer(E, 'loud', rule(on)));
      if (!loud || L.kind === 'counter') { loud = L; where = id; }
      if (L.kind === 'counter') break;
    }
    line(loud?.kind === 'counter' && loud.lead >= LEAD_S, `(c) a loud way in at the front draws men to the noise, their word ${LEAD_S} s or more before the first is there`,
      `${where}: ${loud?.kind} · ${loud?.men.join(', ') || 'nobody'} · word ${f1(loud?.word)} s · there ${f1(loud?.lands)} s (lead ${f1(loud?.lead)}) · "${loud?.line ?? ''}"`);

    // (g) the mission sheet words Hard by what holds the level
    { throwOn(on); const E = row(ROWS[0]), F = row(FILE_WINS), words = difficultyWords('hard', squadsHeld(E)) as string;
      throwOn(true); const fact = squadsOf(E) === false && !!squadsOf(F);
      line(/squads are a drilled garrison/.test(words) && fact, '(g) the mission sheet words Hard on a level cut into squads as the squads\'; the row\'s own fact is the level file\'s',
        `${ROWS[0]}: "${words.slice(0, 64)}…" · squadsOf ${squadsOf(E)} (${FILE_WINS}: ${squadsOf(F)})`); }
  } finally { Object.assign(SQUADDEAL, was); SQUADS.radioCheck = rc0; DESTRUCTION.on = walls0.d; WALLCHARGE.on = walls0.w; WALLCHARGE.outer = walls0.o; Math.random = rnd0; delete process.env.TPS_SQUADDEAL; if (env0 !== undefined) process.env.TPS_SQUADDEAL = env0; }
  console.log(`=== squaddeal: ${all ? 'ALL PASS' : 'FAIL'} ===`);
  return all;
}
const f1 = (x: number | undefined) => typeof x === 'number' && Number.isFinite(x) ? x.toFixed(1) : '—';
/** a run of the answers tool with the stand's own word for the switch set, so its stand-up stages the floor with the rule as asked (standup.ts reads
 *  TPS_SQUADDEAL when a held switch is written) */
const env0 = process.env.TPS_SQUADDEAL;
async function pin<T>(on: boolean, fn: () => Promise<T>): Promise<T> { process.env.TPS_SQUADDEAL = on ? '1' : '0'; try { return await fn(); } finally { if (env0 === undefined) delete process.env.TPS_SQUADDEAL; else process.env.TPS_SQUADDEAL = env0; } }

if (import.meta.main) process.exit((await squaddealCheck()) ? 0 : 1);
