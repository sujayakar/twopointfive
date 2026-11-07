// THE SETTINGS, BOTH WAYS. The knob tables and the difficulty's own state are read by menus and the debugger as the TypeScript's tables, and by the Rust game as its own.
// What the Rust game writes reaches the tables (import.ts, the `knob` way), and what is written to the tables reaches the Rust game as the next tick opens (scan.ts).
//   tools/blitz/rust.sh rust bun run tools/blitz/settings-check.ts      SETTINGS_BREAK=1 in front: neither way carries, and it must go red
// Exit 0 every line passes · 1 a line failed.
import { standUp } from '../qa/headless.ts';
const { knobRegistry } = await import('../../src/game/knobs.ts');
const { difficultyNow, setDifficulty } = await import('../../src/game/difficulty.ts');
const reg: any = knobRegistry();
if (process.env.SETTINGS_BREAK === '1') (globalThis as any).__settingsBreak = true;
let failed = 0; const say = (ok: boolean, line: string) => { if (!ok) failed++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${line}`); };
const words = (o: unknown) => JSON.stringify(o, (_k, v) => v instanceof Map ? [...v] : v instanceof Float32Array || v instanceof Float64Array ? [...v] : v);
const tables = () => Object.fromEntries(Object.entries(reg).map(([k, v]) => [k, words(v)]));

const before = tables();
const W: any = await standUp();
const step = (n: number) => { for (let i = 0; i < n; i++) W.step(1 / 60); };
step(30);
{ const after = tables(), moved = Object.keys(after).filter(k => after[k] !== before[k]);
  const rows = moved.slice(0, 6).map(k => { const a = JSON.parse(before[k] ?? '{}'), b = JSON.parse(after[k]); return `${k}: ${Object.keys({ ...a, ...b }).filter(r => JSON.stringify(a[r]) !== JSON.stringify(b[r])).slice(0, 4).map(r => `${r} ${JSON.stringify(a[r])?.slice(0, 40)} > ${JSON.stringify(b[r])?.slice(0, 40)}`).join(', ')}`; });
  say(moved.length === 0, `a start and thirty ticks leave every one of the ${Object.keys(after).length} tables as they were, in their own shape${moved.length ? `: ${moved.length} moved: ${rows.join(' · ')}` : ''}`); }

const P = reg.PRESSURE;
setDifficulty('hard');
say(P.on === true, `the Rust game's own write reaches the table: hard puts PRESSURE.on ${P.on}, and the menus read that`);
say(difficultyNow() === 'hard', `the Rust game answers hard: ${difficultyNow()}`);
step(2);
say(P.on === true && difficultyNow() === 'hard', `two ticks on it still stands: PRESSURE.on ${P.on}, the Rust game answers ${difficultyNow()}`);

P.on = false; reg.DIGIN.on = false;   // (as the debugger's row writes it: the table, by name, with no call)
step(1);
say(difficultyNow() === 'standard' || difficultyNow() === 'custom', `a write to the table reaches the Rust game as the next tick opens: it now answers ${difficultyNow()}, not hard`);
setDifficulty('standard');
say(P.on === false && difficultyNow() === 'standard', `and back to standard: PRESSURE.on ${P.on}, the Rust game answers ${difficultyNow()}`);

console.log(failed ? `${failed} failed` : 'all pass');
process.exit(failed ? 1 : 0);
