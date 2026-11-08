// WHAT A ROW THE APPLY MAKES HOLDS BESIDE WHAT IT IS SENT, class by class: tools/blitz/class-defaults.json. A row or a part the Rust game makes at run time is its class's prototype and the fields the wire carries. This tool looks at the things a start builds with the class's own constructor, every mission of the list, and writes down for each type of the wire's table:
//   typed  a leaf the class keeps in a typed array (kind, length): it arrives as one, not as a plain list
//   plain  a field no list carries whose value is a plain value: its value where every instance agrees, else that kind's zero (a block place, a count)
//   lists  a field no list carries that is an empty list everywhere
//   nan, inf  a field no list carries that is NaN or Infinity everywhere (no JSON for them)
// The host's own (a mirror's addresses in the module, the engine's handles) are left out: a table's address is no class's value, and no row of those is made at run time.
// and names, as FINDINGS, a class the class table lacks and a field no list carries that holds an object (the picture's half: `picture()` and the page's made hook).
//   TPS_GAME=rust bun run tools/wire/class-defaults.ts [--check] [--say]     --check: exit 1 where the file would change or a class is missing from the table
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { join } from 'node:path';
const ROOT = join(import.meta.dir, '../..'), OUT = join(ROOT, 'tools/blitz/class-defaults.json'), argv = process.argv.slice(2), CHECK = argv.includes('--check'), SAY = argv.includes('--say');
const { standUp } = await import('../qa/headless.ts');
const { SCHEMA, parse, bookOf, compiledExpr, helpersOf } = await import('../blitz/export.ts');
const { classTable } = await import('../blitz/classes.ts');
const { knobRegistry } = await import('../../src/game/knobs.ts').catch(() => ({ knobRegistry: null as any }));
const { ALL_MISSIONS } = await import('../../src/scene/scenarioList.ts');
const { startFromQuery } = await import('../../src/game/scenarioRun.ts');
const { loadLevel } = await import('../../src/scene/levelLoad.ts'), { parseLevelData } = await import('../../src/scene/levelData.ts');
const { scenarioQuery } = await import('../../src/scene/scenarios.ts');
const CLASSES = classTable();
type Seen = { fields: Map<string, Set<string>>; values: Map<string, Set<unknown>>; n: number; proto: Set<object> };
const SEEN = new Map<string, Seen>();
const TYPED = new Set(['Float32Array', 'Float64Array', 'Int32Array', 'Uint32Array', 'Int16Array', 'Uint16Array', 'Int8Array', 'Uint8Array']);
const keyOf = (f: any): string | null => f.ts ?? f.expr?.match(/^o\.([\w$]+)/)?.[1] ?? f.expr?.match(/^o\["(.+?)"\]/)?.[1] ?? null;
const kindOf = (v: unknown): string => { if (v === null) return 'null'; if (Array.isArray(v)) return v.length ? 'list' : 'list0'; if (ArrayBuffer.isView(v)) return `${v.constructor.name}:${(v as any).length}`; if (typeof v === 'object') return 'object'; return typeof v; };
function look(path: string, o: any): void { let S = SEEN.get(path); if (!S) SEEN.set(path, S = { fields: new Map(), values: new Map(), n: 0, proto: new Set() }); S.n++; S.proto.add(Object.getPrototypeOf(o));
  for (const k of Object.keys(o)) { const v = o[k], K = kindOf(v); (S.fields.get(k) ?? S.fields.set(k, new Set()).get(k)!).add(K); if (typeof v !== 'object' || v === null) (S.values.get(k) ?? S.values.set(k, new Set()).get(k)!).add(v); } }
function walk(n: any, v: any, g: any, B: any, seen: Set<object>): void { if (v === null || v === undefined) return; switch (n.k) {
  case 'opt': return walk(n.of, v, g, B, seen);
  case 'vec': case 'arr': if (typeof v === 'object' && !ArrayBuffer.isView(v) && Symbol.iterator in v) for (const x of v) walk(n.of, x, g, B, seen); return;
  case 'tup': if (Array.isArray(v)) n.of.forEach((x: any, i: number) => walk(x, v[i], g, B, seen)); return;
  case 'map': if (v instanceof Map) for (const x of v.values()) walk(n.V, x, g, B, seen); return;
  case 'type': { const T = SCHEMA.types[n.path]; if (T?.k !== 'struct' || typeof v !== 'object' || seen.has(v)) return; seen.add(v); look(n.path, v);
    for (const f of T.fields) { let x: any; try { x = f.expr ? compiledExpr(f.expr)(v, g, B) : v[f.ts]; } catch { continue; } walk(parse(f.ty), x, g, B, seen); } } } }
const missions = (ALL_MISSIONS as any[]); let stood = 0; const refused: string[] = [];
for (const m of missions) { let W: any; try { const level = m.level || 'office.json'; W = await standUp({ dress: true, level: loadLevel(parseLevelData(readFileSync(existsSync(level) ? level : join(ROOT, 'levels', level), 'utf8'))), seed: 20942 } as any); const r = startFromQuery(W.game, scenarioQuery(m)); if (r.why) { refused.push(`${m.id}: ${r.why}`); continue; } W.step(1 / 60); } catch (e) { refused.push(`${m.id}: ${String((e as Error)?.message ?? e).slice(0, 120)}`); continue; }
  const g = W.game, bk = bookOf(g); if (!bk) { refused.push(`${m.id}: no book`); continue; } const reg = (globalThis as any).__tpsKnobs ?? (knobRegistry ? knobRegistry() : {}), B = helpersOf(g, reg), seen = new Set<object>(); stood++;
  for (const t of SCHEMA.tables) for (const o of bk.listOf(t.id)) walk(parse(t.ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')), o, g, B, seen);
  walk({ k: 'type', path: SCHEMA.root }, g, g, B, seen); }
if (!stood) { console.error('class-defaults: no mission stood'); process.exit(2); }
const OUTP: Record<string, any> = {}, lacking: string[] = [], picture: string[] = [];
for (const path of [...SEEN.keys()].sort()) { const S = SEEN.get(path)!, T = SCHEMA.types[path], name = `${T.mod}::${T.name}`; if (/^(scene_box_mirror|host_api|scene_collision|scene_boxes)::/.test(name)) continue; const carried = new Set(T.fields.map(keyOf).filter(Boolean)); const d: any = {};
  for (const P of S.proto) if (P !== Object.prototype && P !== null && !(CLASSES as any)[name]) { const c = (P as any).constructor?.name; lacking.push(`${name}: class ${c}`); }
  for (const [k, kinds] of [...S.fields].sort((a, b) => a[0] < b[0] ? -1 : 1)) { const Ks = [...kinds], ty = Ks.find(x => TYPED.has(x.split(':')[0]));
    if (ty) { const [c, n] = ty.split(':'); (d.typed ??= {})[k] = [c, +n]; continue; }
    if (carried.has(k) || k === 'g') continue;
    if (Ks.every(x => x === 'list0')) { (d.lists ??= []).push(k); continue; }
    const scalar = Ks.every(x => ['null', 'number', 'string', 'boolean'].includes(x)); if (!scalar) { if (Ks.some(x => x === 'object' || x === 'list')) picture.push(`${name}.${k}`); continue; }
    const vals = [...(S.values.get(k) ?? [])]; const one = vals.length === 1 ? vals[0] : Ks.includes('number') ? 0 : Ks.includes('string') ? '' : Ks.includes('boolean') ? false : null; if (one === Infinity) (d.inf ??= []).push(k); else if (typeof one === 'number' && !Number.isFinite(one)) (d.nan ??= []).push(k); else (d.plain ??= {})[k] = one; }
  if (Object.keys(d).length) OUTP[name] = d; }
const text = JSON.stringify(OUTP, null, 0).replace(/\},"/g, '},\n"') + '\n', was = existsSync(OUT) ? readFileSync(OUT, 'utf8') : '';
console.log(`class-defaults: ${stood} of ${missions.length} missions stood, ${SEEN.size} types seen, ${Object.keys(OUTP).length} with a default, ${text.length} bytes`);
if (refused.length) console.log(`refused: ${refused.slice(0, 8).join(' · ')}`);
const lack = [...new Set(lacking)].sort(); if (lack.length) console.log(`LACKING from the class table (classes.ts): ${lack.join(', ')}`);
if (SAY) console.log(`a field no list carries that holds an object, ${picture.length}: ${picture.join(', ')}`);
if (CHECK) { if (was !== text) { console.log('class-defaults: tools/blitz/class-defaults.json would change: run this tool'); process.exit(1); } if (lack.length) process.exit(1); console.log('class-defaults: the file is what the start builds'); process.exit(0); }
writeFileSync(OUT, text); process.exit(0);
