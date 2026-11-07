// A MODULE TABLE KEYED BY THE GAME IS THE SAME TABLE ON BOTH SIDES OF THE BRIDGE.
//
// The TypeScript keeps a squad's posture, the hold-fire word, the rear watch's book and a dozen more in module tables keyed by the GAME (a seat's view of
// it: `new SideMap<Game, …>('silent.states')`). The Rust game keeps the same tables keyed by the seat's man (schema type `JsMap<PlayerId, …>`). What the
// page and the tools read is the TypeScript object, so a row must come back under the key the TypeScript asks with, and a row the TypeScript made before
// the first tick must go over at the start. Two faults this holds (both seen 2026-11-07, pass.json, the room clear with four):
//   - `setSilent(g, true)` before the first tick put the row under the game; after the first tick the row was gone and the Rust game was loud;
//   - set again later (the call crosses), the Rust game was silent and the row arrived under the PLAYER object: `squadSilent(g)` read false for good,
//     and so did every reader that asks by the game (a lesson's condition, the autopilot, the panel's words).
//
//   bun run tools/blitz/gamekey-check.ts          exit 0 all pass · 1 a line failed
//   GAMEKEY_BREAK=1 …                             the posture is never set: lines 1 and 2 must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { SILENT, setSilent, squadSilent } = await import(`${ROOT}/src/game/silent.ts`);
const { SNEAK, sneaking } = await import(`${ROOT}/src/game/sneak.ts`);
const { sideRegistry } = await import(`${ROOT}/src/math/side.ts`);

const BREAK = process.env.GAMEKEY_BREAK === '1', DT = 1 / 60;
let fails = 0; const line = (ok: boolean, what: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };
const GAMES: any[] = [];   // (every game this process stood: a row under another of them is that game's own row, not a wrong key)
const stage = async () => { const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'pass.json'), 'utf8'))), seed: 1 })); const g = W.game; GAMES.push(g); g.godMode = true;
  const why = (await quiet(() => startFromQuery(g, '?levelFile=pass.json&roomclear&squad4')) as any)?.why; if (why) throw new Error(`start refused: ${why}`);
  SILENT.on = true; SNEAK.on = true; return { W, g, men: g.orders.men('all') as any[], step: (n: number) => { for (let i = 0; i < n; i++) W.step(DT); } }; };
/** the men the game itself says are sneaking (the rule's own answer, asked of the game that steps) */
const sneakers = (g: any, men: any[]) => men.filter(m => sneaking(g, m)).length;

console.log(`\n=== gamekey: a module table keyed by the game, on both sides of the bridge${BREAK ? ' — GAMEKEY_BREAK: the posture never set' : ''} ===`);
// (1) a row set before the first tick starts in the game that steps
{ const { g, men, step } = await stage(); if (!BREAK) setSilent(g, true); const before = squadSilent(g); step(30);
  const n = sneakers(g, men);
  line(before && n === men.length && squadSilent(g), 'the squad put silent before the first tick: half a second on the game that steps has it silent, and the table says so by the game', `silent before the first tick ${before} · sneaking ${n} of ${men.length} · squadSilent(game) ${squadSilent(g)}`); }
// (2) a row the stepping game made reads by the game
const S = await stage(); S.step(30); if (!BREAK) setSilent(S.g, true); S.step(2);
{ const n = sneakers(S.g, S.men);
  line(n === S.men.length && squadSilent(S.g), 'the squad put silent half a second in: the game that steps has it silent, and the table says so by the game', `sneaking ${n} of ${S.men.length} · squadSilent(game) ${squadSilent(S.g)}`); }
// (3) every such table, by its type: the schema's tables keyed by the seat's man that the TypeScript declares keyed by the game hold no row under another key
{ const SCHEMA = JSON.parse(readFileSync(resolve(ROOT, 'tools/blitz/schema.json'), 'utf8')), g = S.g, tables: any[] = sideRegistry().tables as any[];
  const bad: string[] = [], seen: string[] = []; let asked = 0;
  for (const t of Object.values(SCHEMA.types) as any[]) for (const f of t.fields ?? []) { const m = /^JsMap<PlayerId, /.test(f.ty ?? '') && /^B\.top\("([^"]+)", "([^"]+)", "([^"]+)"\)$/.exec(f.expr ?? ''); if (!m) continue;
    const src = readFileSync(resolve(ROOT, m[1]), 'utf8'); if (!new RegExp(`\\b${m[2]} = new (Game)?SideMap<Game\\b`).test(src)) continue;   // (keyed by the man in the TypeScript too: not this rule's)
    asked++; const T = tables.find(x => x.sideName === m[3]); if (!T) { bad.push(`${m[3]}: no such table in the registry`); continue; }
    const keys: any[] = T.liveKeys(); if (!keys.length) continue; seen.push(m[3]);
    const other = keys.filter(k => !GAMES.some(x => x === k || x.seats?.some((s: any) => s.view === k)));
    if (other.length || T.get(g) === undefined) bad.push(`${m[3]}: ${other.length ? `a row under ${other.map(k => k === g.player ? 'the player' : k?.constructor?.name).join(', ')}` : 'no row'}${T.get(g) === undefined ? ', nothing by the game' : ''}`); }
  line(asked > 0 && seen.length > 0 && bad.length === 0, 'every table the TypeScript keys by the game and the bridge carries by the seat\'s man: its rows are under the game', `${asked} such tables, ${seen.length} with a row now${bad.length ? ` · ${bad.join(' · ')}` : ''}`); }
console.log(fails ? `=== gamekey: ${fails} FAILED ===` : '=== gamekey: ALL PASS ===');
process.exit(fails ? 1 : 0);
