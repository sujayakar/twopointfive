// A MAN'S COLUMNS (src/sim/men/: one schema, a generator, the store; rust/sim/src/men.rs): are the two languages' columns the one table, is a man's row his for his life, and do his
// columns live and move with the game's block?
//   bun run tools/qa/men-check.ts                      every line below; exit 0 = all pass, 1 = one failed (named)
//   TPS_PROFILE=shipped bun run tools/qa/men-check.ts  the same on the shipped stand
//   MEN_BREAK=1 bun run tools/qa/men-check.ts          the red path: the generated file read one word off, the module's fold read one row off, a stale handle and a man who left
//                                                      read as if nothing had happened, a snapshot's men not put back, a level read as too big, a lint line planted:
//                                                      every line must FAIL
// WHAT IS HELD:
//   table      the schema has no name twice, and rows a multiple of four
//   written    src/sim/men/cols.ts and rust/sim/src/men.rs are what the schema writes
//   both sides the module says the same hash, the same rows and every column where the store has it (a game holds these as it starts; a built.ts of another schema throws by name)
//   generator  a table with every kind of column on every kind of record (the animator's kept state among them) writes accessors that read back what was written, bit for bit, and a Rust struct with those columns
//   rows       the lowest free row, zeros; a freed row is free once; a handle taken before a row changed hands throws by name; a man who left reads his numbers as they stood
//   same bytes a game's men stand in its block once a tick has opened, and the module's fold of each row is the fold of what the accessors read
//   snapshot   a snapshot put back brings a man's columns back; put into another game they are that game's
//   levels     every level of levels/ fits the rows, and the fullest is named
//   lint       no file of src/game writes the columns but through their accessors
// Not game code; nothing here needs a GPU, and nothing here has been seen on one.
import { mkdtempSync, readdirSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
import { MEN_OUT, MEN_TABLE, layOut, rustText, schemaFaults, schemaHash, tsText, type Schema } from '../sim/men.ts';

const BREAK = process.env.MEN_BREAK === '1', DT = 1 / 60;
const S: any = await import(`${ROOT}/src/sim/sim.ts`), RB: any = await import(`${ROOT}/src/game/rollback.ts`), LF: any = await import(`${ROOT}/src/game/levelfit.ts`);
const M: any = await import(`${ROOT}/src/sim/men/store.ts`), C: any = await import(`${ROOT}/src/sim/men/cols.ts`), SC: any = await import(`${ROOT}/src/sim/men/schema.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const throwsName = (f: () => unknown, word: string): string => { try { f(); return ''; } catch (e) { const m = String((e as Error).message); return m.includes(word) ? m : ''; } };
console.log(`a man's columns · profile ${process.env.TPS_PROFILE ?? 'frozen'}${BREAK ? ' · MEN_BREAK=1 (the red path)' : ''}`);

{ const f = schemaFaults(MEN_TABLE), bad = schemaFaults({ rows: 6, fields: [...MEN_TABLE.fields, MEN_TABLE.fields[0]] });
  verdict(f.length === 0 && bad.length === 2 && !BREAK, 'table: the schema has no name twice, and rows a multiple of four', f.length ? f.join(' · ') : `${MEN_TABLE.fields.length} fields, ${MEN_TABLE.rows} rows · ${SC.MEN_KEPT.length} fields kept on the object by name · a table with both faults is told both`); }
{ const ts = readFileSync(MEN_OUT.ts, 'utf8'), rs = readFileSync(MEN_OUT.rust, 'utf8');
  const same = (BREAK ? ts + ' ' : ts) === tsText(MEN_TABLE) && rs === rustText(MEN_TABLE);
  verdict(same, 'written: the generated files are what the schema writes', same ? `${layOut(MEN_TABLE).cols.length} columns, hash ${schemaHash(MEN_TABLE).toString(16)}` : 'bun run tools/sim/men.ts'); }
{ const x = S.sim(), names = [...C.MEN_F64, ...C.MEN_U32], hash = BREAK ? C.MEN_SCHEMA + 1 : C.MEN_SCHEMA;
  let at = C.MEN_HEAD; const off: string[] = [];
  names.forEach((n: string, i: number) => { if (x.men_col(i) !== at) off.push(n); at += SC.MEN_ROWS * (i < C.MEN_F64.length ? 8 : 4); });
  const part = (await import(`${ROOT}/src/sim/state.ts`)).simLayout().parts.find((p: any) => p.name === 'men');
  verdict(x.men_schema() >>> 0 === hash && x.men_col(names.length) === SC.MEN_ROWS && off.length === 0 && part?.len === at, 'both sides: the module has the same hash, the same rows, and every column where the store has it',
    `module ${(x.men_schema() >>> 0).toString(16)} · here ${hash.toString(16)} · ${x.men_col(names.length)} rows · part ${part?.len} bytes, here ${at}${off.length ? ` · off: ${off.join(', ')}` : ''}`); }

// ---- the generator, on a table with every kind of column on every kind of record
{ const T: Schema = { rows: 8, fields: [
    { name: 'a', on: 'man', group: 'hot', type: 'f64', digest: true }, { name: 'b', on: 'man', group: 'cold', type: 'u32', digest: false }, { name: 'c', on: 'guard', group: 'hot', type: 'i32', digest: true },
    { name: 'd', on: 'guard', group: 'cold', type: 'bool', digest: false }, { name: 'e', on: 'player', group: 'hot', type: 'vec3', digest: true }, { name: 'f', on: 'guard', group: 'hot', type: 'vec3?', digest: true },
    { name: 'g', on: 'man', group: 'cold', type: ['one', 'two', 'three'], digest: false }, { name: 'h', on: 'anim', group: 'animator', type: 'f64', digest: false }, { name: 'i', on: 'guard', group: 'hot', type: 'bool', digest: true } ] };
  const dir = mkdtempSync(join(tmpdir(), 'men-gen-')), file = join(dir, 'cols.ts');
  writeFileSync(file, tsText(T).replace(`import type { MenStore } from './store';\n`, 'type MenStore = any;\n').replace(`import type { Vec3 } from '../../math/vec';\n`, 'type Vec3 = [number, number, number];\n'));
  const G: any = await import(file); rmSync(dir, { recursive: true });
  const { cols } = layOut(T), bytes = 16 + cols.reduce((n, c) => n + T.rows * (c.kind === 'f64' ? 8 : 4), 0), own = new Float64Array(bytes >>> 3), store = { d: own, u: new Uint32Array(own.buffer), b8: 0, b4: 0 };
  const mk = (K: any) => { const o = new K(); o.S = store; o.r = 3; return o; };
  const gd = mk(G.GuardCols), pl = mk(G.PlayerCols), an = mk(G.AnimCols), bad: string[] = [], eq = (n: string, a: unknown, b: unknown) => { if (JSON.stringify(a) !== JSON.stringify(b) || (typeof a === 'number' && !Object.is(a, b))) bad.push(`${n}: ${JSON.stringify(a)} ≠ ${JSON.stringify(b)}`); };
  gd.a = -0; eq('f64 -0', gd.a, -0); gd.a = 1 / 3; eq('f64', gd.a, 1 / 3); gd.b = 0xfffffffe; eq('u32', gd.b, 0xfffffffe); gd.c = -7; eq('i32', gd.c, -7);
  gd.d = true; gd.i = false; eq('bool on', gd.d, true); eq('bool neighbour', gd.i, false); gd.i = true; gd.d = false; eq('bool off', gd.d, false); eq('bool kept', gd.i, true);
  pl.e = [1, 2, 3]; eq('vec3', pl.e, [1, 2, 3]); pl.eY = 9; eq('vec3 by part', pl.e, [1, 9, 3]); { const p = pl.e; p[0] = 77; eq('vec3 is a copy', pl.eX, 1); }
  eq('vec3? none', gd.f, null); gd.f = [4, 5, 6]; eq('vec3?', gd.f, [4, 5, 6]); gd.f = null; eq('vec3? none again', gd.f, null); eq('vec3? zeros', gd.fX, 0); eq('truth kept', gd.i, true);
  gd.g = 'three'; eq('words', gd.g, 'three'); an.h = 2.5; eq('anim', an.h, 2.5);
  const R = rustText(T), hasAll = cols.every(c => R.includes(`pub ${c.rust}: [${c.kind}; ROWS]`)), hash = schemaHash(T) !== schemaHash({ ...T, rows: 12 });
  if (BREAK) bad.push('the red path');
  verdict(bad.length === 0 && hasAll && hash, 'generator: every kind of column on every kind of record reads back what was written, and the Rust struct has those columns', bad.length ? bad.join(' · ') : `${T.fields.length} fields, ${cols.length} columns; the animator's kept state among them; the hash moves with the rows`); }

// ---- rows
{ const st = new M.MenStore(), first = st.enter(), second = st.enter(), h = st.handleOf(second), ok1 = first === 0 && second === 1 && st.standing === 2 && h !== 0;
  const a = new C.ManCols(), b = new C.ManCols(); M.standMan(a, st); M.sameMan(b, a); a.owner = 5; a.since = 1.5; const same = b.owner === 5 && b.since === 1.5;
  st.leave(second); const stale = BREAK ? '' : throwsName(() => st.rowOf(h), 'stale handle'), twice = throwsName(() => st.leave(second), 'left twice');
  const third = st.enter(), again = third === second && st.handleOf(third) !== h && throwsName(() => st.rowOf(h), 'stale handle') !== '';
  const row = a.r, ah = st.handleOf(a.r); M.manLeaves(a, b);
  const kept = !BREAK && a.owner === 5 && b.since === 1.5 && a.S !== st && b.S === a.S, handle = throwsName(() => st.rowOf(ah), 'stale handle');
  const fresh = new C.ManCols(); M.standMan(fresh, st); const zero = fresh.r === row && fresh.owner === 0 && fresh.since === 0;
  fresh.owner = 9; const apart = a.owner === 5; M.manLeaves(a, b); const once = a.owner === 5;
  verdict(ok1 && same && !!stale && !!twice && again && kept && !!handle && zero && apart && once, 'rows: the lowest free row, zeros; a stale handle throws by name; a man who left keeps his numbers, and his row is the next man\'s, zeros',
    `two rows in order ${ok1} · two objects of one man read one row ${same} · stale: ${stale.slice(0, 60)} · a row left twice: ${twice.slice(0, 40)} · given again ${again} · he reads what he was: ${kept} · his row again, zeros ${zero} · apart ${apart}`); }

// ---- a stood game: its men in its block, the module reading the bytes the accessors write, a snapshot
{ const W: any = await standUp({ seed: 7 }), g = W.game; for (let f = 0; f < 5; f++) W.step(DT);
  const men = [g.player, ...g.guards, ...g.civilians], x = S.sim(), st = g.sim, slot = st.bind();
  men.forEach((m: any, i: number) => { m.owner = 100 + i; m.char.rank = 200 + i; m.reason = 300 + i; m.since = i + 0.25; });
  const fold = (r: number) => { let h = 0x811c9dc5; const w = (v: number) => { h = Math.imul(h ^ v, 0x01000193) >>> 0; }, F = new Float64Array(1), U = new Uint32Array(F.buffer), store = g.men;
    let at = C.MEN_HEAD; C.MEN_F64.forEach(() => { F[0] = store.d[(at >>> 3) + store.b8 + r]; w(U[0]); w(U[1]); at += SC.MEN_ROWS * 8; }); C.MEN_U32.forEach(() => { w(store.u[(at >>> 2) + store.b4 + r]); at += SC.MEN_ROWS * 4; }); return h; };
  const off = men.filter((m: any) => x.men_fold(slot, BREAK ? m.r + 1 : m.r) >>> 0 !== fold(m.r)).length, own = men.every((m: any) => m.S === g.men && m.char.S === g.men && m.char.r === m.r), rows = new Set(men.map((m: any) => m.r)).size === men.length;
  const inBlock = g.men.u === st.w && g.men.b4 > 0 && (st.w[1] & 256) !== 0;
  verdict(off === 0 && own && rows && inBlock && g.men.standing === men.length, 'same bytes: a game\'s men stand in its block, and the module reads each row as the accessors read it',
    `${men.length} men, a row each ${rows} · a man and his Character one row ${own} · in the block ${inBlock} · rows the module reads otherwise: ${off} · standing ${g.men.standing}`);
  const snap = RB.takeSnapshot(g), was = men.map((m: any) => m.owner); for (const m of men) m.owner = 1; W.step(DT);
  if (!BREAK) RB.restoreSnapshot(snap);
  const back = men.every((m: any, i: number) => m.owner === was[i] && m.since === i + 0.25);
  const W2: any = await standUp({ seed: 7 }), g2 = W2.game; W2.step(DT); RB.restoreSnapshot(snap, g2);
  const there = [g2.player, ...g2.guards].every((m: any, i: number) => m.owner === 100 + i && m.S === g2.men) && g2.men !== g.men && g2.men.standing === men.length;
  verdict(back && there, 'snapshot: a snapshot put back brings a man\'s columns back; put into another game they are that game\'s', `back ${back} · another game ${there}`); }

// ---- every level fits
{ const files = readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')); let most = 0, who = '', read = 0; const over: string[] = [];
  const log = console.log, err = console.error;
  for (const f of files) { console.log = console.error = () => {}; let level: any = null; try { level = f === 'office.json' ? null : loadLevel(JSON.parse(readFileSync(resolve(ROOT, 'levels', f), 'utf8'))); } catch { /* a document the loader refuses is the level check's */ } console.log = log; console.error = err;
    if (!level) continue; read++; const n = LF.levelCounts(level).men; if (n > most) { most = n; who = f; } if (n > (BREAK ? 5 : SC.MEN_ROWS)) over.push(`${f} ${n}`); }
  verdict(over.length === 0 && read > 10, `levels: every level's men fit the ${SC.MEN_ROWS} rows`, `${read} levels read · the fullest ${most} rows with its seats (${who})${over.length ? ` · over: ${over.slice(0, 4).join(', ')}` : ''}`); }

// ---- the lint: nothing in src/game reaches the columns but through their accessors
{ const dir = resolve(ROOT, 'src/game'), bad: string[] = [];
  const RE: readonly [RegExp, string][] = [[/\bMEN_(F64|U32|HEAD|GEN|BYTES)\b/, 'the layout'], [/\bP_MEN\b/, 'the part'], [/\.men\.(d|u|b8|b4)\b/, 'the store\'s bytes'], [/\.S\.(d|u|b8|b4)\b/, 'the store\'s bytes'], [/\bdefineProperty\([^)]*\b(char|gd|guard|man)\b/, 'a property put over a column']];
  const lintText = (name: string, text: string) => text.split('\n').forEach((l, i) => { const code = l.replace(/\/\/.*$/, ''); for (const [re, why] of RE) if (re.test(code)) bad.push(`${name}:${i + 1} ${why}`); });
  for (const f of readdirSync(dir)) if (f.endsWith('.ts')) lintText(f, readFileSync(join(dir, f), 'utf8'));
  const caught = (() => { const n = bad.length; lintText('(planted)', 'g.men.u[7] = 3;'); const hit = bad.length > n; if (!BREAK) bad.length = n; return hit; })();
  verdict(bad.length === 0 && caught, 'lint: no file of src/game writes the columns but through their accessors', bad.length ? bad.slice(0, 6).join(' · ') : `a planted line is caught: ${caught}`); }

console.log(fails ? `=== men-check: ${fails} line(s) FAILED ===` : '=== men-check: PASS ===');
process.exit(fails ? 1 : 0);
