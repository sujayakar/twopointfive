// THE TABLES THE PROGRAM COULD NOT WRITE FROM THE SOURCE (filled by a loop at load, or on first asking, or built by calls), read off a running game at the page's values.
//   TPS_PROFILE=shipped bun run --preload ./tools/blitz/expose.ts tools/blitz/dump-tables.ts <out.json>     then run the skeleton's program again
import { writeFileSync } from 'node:fs';
import { SCHEMA } from './export';
const { standUp } = await import('../qa/headless.ts');
const log = console.log; console.log = () => {}; const W = await standUp({}); const g: any = W.game;
const { startFromQuery } = await import('../../src/game/scenarioRun.ts'); await startFromQuery(g, new URLSearchParams('levelFile=compound.json&roomclear&squad4'));
for (let i = 0; i < 60; i++) g.step(1 / 60, undefined);
console.log = log;
const seen = new Set<object>();
const spell = (v: any, depth = 0): any => { if (typeof v === 'number') return Number.isFinite(v) ? v : `#${v}`; if (typeof v === 'function') return '#fn'; if (v === undefined) return null; if (v === null || typeof v !== 'object') return v; if (depth > 12 || seen.has(v)) return '#cycle'; seen.add(v); let r: any; if (Array.isArray(v) || ArrayBuffer.isView(v)) r = Array.from(v as any, x => spell(x, depth + 1)); else if (v instanceof Map) r = Object.fromEntries([...v].map(([k, x]) => [String(k), spell(x, depth + 1)])); else if (v instanceof Set) r = [...v].map(x => spell(x, depth + 1)); else { r = {}; for (const k in v) r[k] = spell(v[k], depth + 1); } seen.delete(v); return r; };
const out: Record<string, unknown> = {}; let n = 0, none = 0;
for (const { file, name } of SCHEMA.wanted) { const T = (globalThis as any).__tpsTop?.[file]; let v: unknown; try { v = T?.[name]; } catch { v = undefined; } if (v === undefined) { none++; continue; } out[`${file}::${name}`] = spell(v); n++; }
writeFileSync(process.argv[2], JSON.stringify(out)); console.log(`${n} tables read, ${none} out of reach`); process.exit(0);
