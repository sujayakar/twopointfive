// THE ANIMATOR'S LAYERS IN RUST, ASKED BOTH WAYS (rust/sim/src/layers.rs, the row RUST.walk; docs/agents/rust.md "A layer of the animator") — a scripted
// mission is played on the server's stand with the poses at home and the row up, and EVERY call of a layer that has moved is run twice on the same pose from
// the same start, once by the module and once by the TypeScript method (anim/skeleton.ts layerBoth), and the two poses compared single for single, and the run-forward bones and the state each leaves double for double. So the
// questions are the ones a game asks, with the numbers a game has. With it: the two lists Rust code reads by name (rust/sim/src/knobs.rs: the knobs, the
// rig's named nodes) are held to the game's own tables.
//   bun run tools/qa/layer-ident.ts [--run warehouse-four:1]   exit 0 = every line passes · 1 = one failed (named)
//   RUST_BREAK=1 bun run tools/qa/layer-ident.ts               the red path: every knob is handed to the module a hundredth large — `both` and `changed` must FAIL
// WHAT IS HELD:
//   list      every path of the knob list is a number of a registered knob table or of the animator's tables (a path that is not is an error as the list is first read)
//   read      every name of the three lists (knobs, named nodes, lists of nodes) is read by some line of the crate outside knobs.rs (a line nobody reads is rot)
//   both      every layer in LAYERS_MOVED was asked at least FLOOR times in the run, and no answer of any layer differs
//   changed   a knob changed between two ticks is the module's by the next tick's opening, and back again
// Not game code; nothing here needs a GPU, and nothing here has been seen on one.
import { resolve } from 'node:path';
import { readFileSync, readdirSync } from 'node:fs';
const ROOT = resolve(import.meta.dir, '../..');
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.TPS_RUSTPOSES = '1'; process.env.TPS_RUSTWALK = '1';
const BREAK = process.env.RUST_BREAK === '1', args = process.argv.slice(2), flag = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
/** the layers that have moved (rust/sim/src/layers.rs, a `pub fn` each), by the name game/character.ts asks them both ways under */
const LAYERS_MOVED = ['mvTrunk', 'longBody', 'lifeLayer'], FLOOR = 100;
const { standUp } = await import('./headless.ts');
const S: any = await import(`${ROOT}/src/sim/sim.ts`), SK: any = await import(`${ROOT}/src/anim/skeleton.ts`), RK: any = await import(`${ROOT}/src/game/rustknobs.ts`);
const KN: any = await import(`${ROOT}/src/game/knobs.ts`), PT: any = await import(`${ROOT}/tools/qa/playthrough.ts`);
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
console.log(`the animator's layers in Rust · profile ${process.env.TPS_PROFILE}${BREAK ? ' · RUST_BREAK=1 (the red path)' : ''}`);
const no = S.simWhyNot();
if (no) { console.log(`- FAIL — the module does not start here: ${no}`); process.exit(1); }
if (BREAK) RK.RUST_KNOBS.hand = (_i: number, v: number) => v * 1.01;

// ---- list
let lines: { path: string; value: number | null }[] = [], listErr = '';
try { lines = RK.rustKnobLines(); } catch (e) { listErr = String((e as Error).message); }
verdict(!listErr && lines.length > 0 && lines.every(l => typeof l.value === 'number'), 'list: every knob Rust code reads is a number of a registered knob table',
  listErr || `${lines.length} knobs: ${lines.map(l => `${l.path} ${l.value}`).join(' · ')} · ${RK.rustList(3).length} named nodes: ${RK.rustList(3).join(' ')}`);

// ---- read
{ const dir = `${ROOT}/rust/sim/src`, src = readFileSync(`${dir}/knobs.rs`, 'utf8'), names = [...src.matchAll(/^\s+([A-Z][A-Z0-9_]*) = "([^"]+)",$/gm)].map(m => m[1]);
  const others = readdirSync(dir).filter(f => f.endsWith('.rs') && f !== 'knobs.rs').map(f => readFileSync(`${dir}/${f}`, 'utf8')).join('\n');
  const unread = names.filter(n => !new RegExp(`\\b${n}\\b`).test(others));
  verdict(names.length === lines.length + RK.rustList(3).length + RK.rustList(5).length && unread.length === 0, 'read: every knob, every named node and every list of nodes of the lists is read by a line of the crate',
    unread.length ? `NOBODY READS ${unread.join(', ')}` : `${names.length} names in rust/sim/src/knobs.rs, the module lists ${lines.length} + ${RK.rustList(3).length} + ${RK.rustList(5).length}, each read outside it`); }

// ---- both
{ const [scn, seed] = (flag('--run') ?? 'warehouse-four:1').split(':'); SK.LAYERS.both = true;
  const t0 = performance.now(), r = await PT.play(scn, Number(seed), 300, false, {}); SK.LAYERS.both = false;
  const by: Record<string, { asked: number; differ: number }> = SK.LAYERS.by, thin = LAYERS_MOVED.filter(n => (by[n]?.asked ?? 0) < FLOOR), off = Object.keys(by).filter(n => by[n].differ > 0), stray = Object.keys(by).filter(n => !LAYERS_MOVED.includes(n));
  console.log('LAYER-IDENT ' + JSON.stringify({ by }));   // (what tools/qa/rust-check.ts reads: under RUST_BREAK=1 every layer's own answers must have differed)
  verdict(thin.length === 0 && off.length === 0 && stray.length === 0, 'both: every call of a moved layer in a mission, run by the module and by the TypeScript on the same pose, leaves the same pose single for single',
    `${scn}:${seed} ends ${String(r.end)} · ${LAYERS_MOVED.map(n => `${n} ${by[n]?.asked ?? 0} asked, ${by[n]?.differ ?? 0} differ`).join(' · ')}${thin.length ? ` · ASKED TOO LITTLE: ${thin.join(', ')}` : ''}${stray.length ? ` · asked and not in LAYERS_MOVED: ${stray.join(', ')}` : ''}${SK.LAYERS.first ? ` · first: ${SK.LAYERS.first}` : ''} · ${((performance.now() - t0) / 1000).toFixed(1)} s`); }

// ---- changed
{ const W: any = await standUp({ seed: 7 }), DT = 1 / 60, at = S.sim().knob_addr(0) / 8, i = lines.findIndex(l => l.path === 'MV.gait.lean'), MV = KN.knobRegistry().MV;
  const was = MV.gait.lean, mod = () => S.f64()[at + i];
  W.step(DT); const a = mod() === was; MV.gait.lean = was + 1.25; const stale = mod() === was; W.step(DT); const b = mod() === was + 1.25; MV.gait.lean = was; W.step(DT); const c = mod() === was;
  verdict(i >= 0 && a && stale && b && c, 'changed: a knob changed between two ticks is the module\'s as the next tick opens',
    `MV.gait.lean ${was}: the module's after a tick ${a} · the table changed, the module's not yet ${stale} · after the next tick ${b} · put back, and a tick ${c}`); }

console.log(fails ? `=== layer-ident: ${fails} FAILED ===` : '=== layer-ident: PASS — every layer the same pose both ways, every knob the table\'s ===');
process.exit(fails ? 1 : 0);
