// A check, as tools/qa has them: the game's tables come through an untyped import, so a row is known here only by its name.
const ROOT = '../..';
const { K } = await import(`${ROOT}/src/game/k.ts`);
const KK = K;   // (another name for the table)
const line = (ok: boolean, label: string): boolean => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${label}`); return ok; };
const STAGE: Record<string, boolean> = {};
const worker = (set: Record<string, Record<string, unknown>>): string => JSON.stringify(set);   // (what a check hands the child it plays a run in: its --set)
const setFlag = (v: boolean, what: string): void => { STAGE[what] = v; };   // (it sets the stage for the next line: not a line itself)
export async function check(): Promise<void> {
  const keep = { on: K.on, extra: K.extra, stay: K.stay };
  K.on = true; Object.assign(K, { extra: true });
  line(K.on === true, 'the stand: on, as the page opens');
  KK.on = false;   // the off arm
  line(offArm() === 2, 'the rule off: the old answer');
  for (const key of ['on', 'extra'] as const) K[key] = keep[key];
  K.stay = keep.stay;
  line(K.stay === keep.stay, 'a row that stays is as the check found it');
  const more = { stay: keep.stay }; K.stay = !keep.stay; Object.assign(K, { on: true }, more);   // (a kept row given beside a retired one, in one call)
  line(K.stay === keep.stay, 'a kept row given in the same call as a retired one is still given');
  line(worker({ K: { on: false, stay: keep.stay } }).includes('stay'), 'a set handed to a worker still names its kept row');
  line(worker(JSON.parse('{"K":{"on":false}}')).length > 2, 'a set written as a string is handed on as it is');
  const kept3 = { K: { on: K.on, stay: K.stay } }; line(kept3.K.on === K.on && kept3.K.stay === K.stay, 'a kept copy under the table\'s name is read back, key by key');
  setFlag(!K.old, 'night'); line(STAGE.night === true, 'the stage is set for the line after it');
  const both = [K.on, K.stay]; [K.on, K.stay] = both;   // (a kept pair put back through a pattern)
  const pair = [K.stay, (K.on = true, K.stay)]; K.on = keep.on;
  line(pair[0] === pair[1] && both.length === 2, 'a pattern and a comma that wrote the rule are still a pattern and a value');
  K.on = false; const door = findDoor(K.on); K.on = keep.on;   // (a staging that finds its place only on the retired path)
  if (!door) line(false, 'a door to stage it on'); else line(door.length > 0, 'the staged case: asked at the door it found');
  await direct();
  table();
  await each('kept', async () => {
    line(K.stay === keep.stay, 'the first line of a group');
    const kept = line(!K.on || K.stay === true, 'a verdict kept in a name is not a statement of its own');
    if (!kept) console.log('  (it was red)');
  });
}
const each = async (_name: string, fn: () => Promise<void>): Promise<void> => { await fn(); };
/** a check that prints its verdicts itself, with no helper */
async function direct(): Promise<void> {
  console.log(`  ${K.stay === keep2.stay ? 'PASS' : 'FAIL'}  printed straight from a function: a row that stays`);
  console.log(`  ${(K.on ? 1 : 2) === 2 ? 'PASS' : 'FAIL'}  printed straight from a function: the rule off`);
}
const keep2 = { stay: false };
/** a check that prints a line a case, each under words worked out as it runs */
const CASES2 = [{ name: 'the first case', want: 2 }, { name: 'the second case', want: 2 }];
function table(): void { for (const c of CASES2) console.log(`  ${(CASES2.indexOf(c) === 1 && K.on ? 1 : 2) === c.want ? 'PASS' : 'FAIL'}  ${c.name}: ${c.want}`); }
function offArm(): number { return K.on ? 1 : 2; }
function findDoor(on: boolean): string | null { return on ? null : 'a door'; }
if ((import.meta as { main?: boolean }).main) await check();
