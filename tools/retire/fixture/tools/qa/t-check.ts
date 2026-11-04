// A check, as tools/qa has them: the game's tables come through an untyped import, so a row is known here only by its name.
const ROOT = '../..';
const { K } = await import(`${ROOT}/src/game/k.ts`);
const KK = K;   // (another name for the table)
const line = (ok: boolean, label: string): void => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${label}`); };
export async function check(): Promise<void> {
  const keep = { on: K.on, extra: K.extra, stay: K.stay };
  K.on = true; Object.assign(K, { extra: true });
  line(K.on === true, 'the stand: on, as the page opens');
  KK.on = false;   // the off arm
  line(offArm() === 2, 'the rule off: the old answer');
  for (const key of ['on', 'extra'] as const) K[key] = keep[key];
  K.stay = keep.stay;
  line(K.stay === keep.stay, 'a row that stays is as the check found it');
}
function offArm(): number { return K.on ? 1 : 2; }
if ((import.meta as { main?: boolean }).main) await check();
