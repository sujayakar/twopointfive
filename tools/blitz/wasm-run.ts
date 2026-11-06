// THE MISSION LOOP THROUGH WASM (tools/blitz/FACE.md): the Rust game's face (rust/game_face, one wasm module) stepped through a whole mission in bun, beside
// what the TypeScript side of the loop wrote (mission.ts: the start, the host's half every tick, a hash of each part of rust/sim's block and the die's count
// after every tick). The same checks the native runner makes (rust/game_run bin `mission`), by the same lines round a step (rust/game_run/src/lib.rs).
//   bun run tools/blitz/wasm-run.ts <dir> [ticks] [--wasm FILE] [--two] [--one-call] [--reserve MB] [--max-mb N] [--js-every N] [--quiet]
//   bun run tools/blitz/wasm-run.ts --ts-time <mission>:<seed>   the TypeScript tick's own time for the same play (nothing written, no module loaded)
//   bun run tools/blitz/wasm-run.ts --exports [--wasm FILE]      the module's exports beside the shipped simulation's (src/sim/built.ts): missing, extra, same
// <dir> is a folder run.sh / mission.ts filled (c1.bin, s0.bin, h<k>.bin, host.txt, post.txt, gives.txt, hostwrites.txt, knobs.bin; a tutorial mission's lesson.json and meter.txt). The mission is not played again.
// AFTER EVERY TICK the die's count and each part's hash are held to post.txt's line; the first tick a part other than `poses` parts is the stop (`poses` parts
// by design from the first death: README.md). The hashes are taken inside the module (face_part_hashes); every --js-every'th tick (100) and the last they are
// also taken HERE over the module's memory in place, through face_part_addr / face_part_len, and the two must agree.
//   --two        a second game from the same start in the module's next slot, stepped after the first every tick and held to the same lines
//                (--second-first: the second game is stepped before the first every tick, to tell a slot's fault from a fault of whoever steps second)
//   --reserve MB a run of memory asked of the module and freed before the start: does the memory then still grow inside a tick (FACE.md, memory)
//   --max-mb N   stop, still in step, once the module's linear memory is over N MB (every made box is a row for good: a long mission nears wasm32's 4 GB)
//   --one-call   face_step with UPLOAD and LIGHTS as its flags: the one call a tick of a host that plays. The lights' update then runs before the hashes are
//                taken (the TypeScript's are taken before its host's), so a part that update writes may differ: such a part is named once and left out
// Exit: 0 in step · 1 the block or the die parted · 2 the Rust tick panicked (its words are printed) · 3 a file is missing or the module does not stand.
import { readFileSync, existsSync } from 'node:fs';
const argv = process.argv.slice(2), has = (n: string) => argv.includes(n), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
if (has('--ts-time')) {
  // THE TYPESCRIPT TICK'S OWN TIME FOR THE SAME PLAY: the mission stood up and played as mission.ts does (the page's stand, every DRESS row down, the game's
  // own die, the poses settled before the first tick) with nothing written, and the clock read round `stepSeats` alone: the call `game_step` is the translation of.
  await import('../qa/stand.ts');
  const { Game } = await import('../../src/game/game.ts'), { play } = await import('../qa/playthrough.ts'), { newDice } = await import('../../src/math/dice.ts'), { dressAll } = await import('../../src/game/dress.ts');
  const [scn, seed] = arg('--ts-time', '').split(':'); let k = 0, spent = 0, worst = 0, worstAt = 0, die = -1;
  const was = Game.prototype.update;
  Game.prototype.update = function (this: any, ...a: any[]) {
    const g = this, step = g.stepSeats; k++;
    if (k === 1) dressAll(false);
    if (k === 1 && !g.dice) g.dice = newDice(20942);
    g.stepSeats = function (this: any, dt: number, cmds: unknown) { if (k === 1) g.sim.settle(g); const t = performance.now(); try { return step.call(this, dt, cmds); } finally { const el = performance.now() - t; spent += el; if (el > worst) { worst = el; worstAt = k; } die = g.dice?.n ?? -1; } };
    try { return (was as any).apply(this, a); } finally { delete g.stepSeats; }
  } as any;
  const log = console.log, er = console.error, wn = console.warn; let res: any = null;
  try { res = await play(scn, +(seed ?? 1), 300, false, {}); } finally { console.log = log; console.error = er; console.warn = wn; }
  say(`TypeScript: ${scn}:${seed ?? 1} played out at tick ${k} (${res?.end}); the die's count at the end ${die}; the tick (stepSeats alone): mean ${(spent / Math.max(1, k)).toFixed(3)} ms, worst ${worst.toFixed(1)} ms at tick ${worstAt}`);
  process.exit(0);
}
const WASM = arg('--wasm', new URL('../../rust/game_face/target/wasm32-unknown-unknown/debug/game_face.wasm', import.meta.url).pathname);
if (!existsSync(WASM)) { say(`wasm-run: no module at ${WASM} (cd rust/game_face && cargo build --target wasm32-unknown-unknown)`); process.exit(3); }
const bytes = readFileSync(WASM);
const t0 = performance.now(), mod = new WebAssembly.Module(bytes), tC = performance.now();
const imports = WebAssembly.Module.imports(mod);
if (imports.length) { say(`wasm-run: the module imports ${imports.length} things (${imports.slice(0, 4).map(i => i.module + '.' + i.name).join(', ')}): a module of the game imports nothing`); process.exit(3); }
const X: any = new WebAssembly.Instance(mod, {}).exports, mem: WebAssembly.Memory = X.memory;
say(`wasm-run: ${WASM.split('/').slice(-3).join('/')}: ${bytes.length} bytes, compiled in ${(tC - t0).toFixed(0)} ms, imports 0, exports ${WebAssembly.Module.exports(mod).length}; memory at the start ${(mem.buffer.byteLength / 1048576).toFixed(1)} MB`);

if (has('--exports')) {
  // THE EXPORT LIST BESIDE THE SHIPPED SIMULATION'S: every export of src/sim/built.ts must be an export of this module, of the same kind and, where it is a function, the same number of parameters
  const { SIM_WASM } = await import('../../src/sim/built.ts'), { SIM_ABI, SIM_EXPORTS } = await import('../../src/sim/sim.ts');
  const smod = new WebAssembly.Module(Uint8Array.from(atob((SIM_WASM as string[]).join('')), c => c.charCodeAt(0))), S: any = new WebAssembly.Instance(smod, {}).exports;
  const mine = new Map(WebAssembly.Module.exports(mod).map(e => [e.name, e.kind])), theirs = new Map(WebAssembly.Module.exports(smod).map(e => [e.name, e.kind]));
  const missing = [...theirs.keys()].filter(n => !mine.has(n)), extra = [...mine.keys()].filter(n => !theirs.has(n)), same = [...theirs.keys()].filter(n => mine.has(n));
  const odd = same.filter(n => mine.get(n) !== theirs.get(n) || (theirs.get(n) === 'function' && X[n].length !== S[n].length));
  const unnamed = (SIM_EXPORTS as string[]).filter(n => !mine.has(n));
  say(`the shipped simulation (src/sim/built.ts) exports ${theirs.size}; this module ${mine.size}`);
  say(`  same      ${same.length}${odd.length ? `, of which ${odd.length} differ in kind or parameter count: ${odd.join(' ')}` : ' (each the same kind, each function the same number of parameters)'}`);
  say(`  missing   ${missing.length}${missing.length ? ': ' + missing.join(' ') : ''}`);
  say(`  extra     ${extra.length}: ${extra.join(' ')}`);
  say(`  src/sim/sim.ts SIM_EXPORTS names ${(SIM_EXPORTS as string[]).length}; not in this module: ${unnamed.length ? unnamed.join(' ') : 'none'}`);
  const words = [['sim_abi', X.sim_abi(), S.sim_abi(), SIM_ABI], ['state_layout', X.state_layout() >>> 0, S.state_layout() >>> 0], ['state_len', X.state_len(), S.state_len()], ['state_games', X.state_games(), S.state_games()], ['state_parts', X.state_parts(), S.state_parts()], ['knob_len(0)', X.knob_len(0), S.knob_len(0)]];
  for (const w of words) say(`  ${w[0]}: this module ${w[1]}, the shipped ${w[2]}${w.length > 3 ? `, sim.ts ${w[3]}` : ''}${w.slice(1).every(v => v === w[1]) ? '' : '   DIFFERS'}`);
  say(`  the block of slot 0 at ${X.state_addr(0)} here, ${S.state_addr(0)} in the shipped module (an address is the module's own: src/sim/state.ts asks state_addr)`);
  process.exit(missing.length || odd.length || words.some(w => w.slice(1).some(v => v !== w[1])) ? 1 : 0);
}

const dir = argv[0];
if (!dir || dir.startsWith('--') || !existsSync(`${dir}/c1.bin`)) { say('wasm-run.ts <dir of a mission.ts run> [ticks] [--wasm FILE] [--two] [--one-call] [--js-every N] [--quiet]   |   wasm-run.ts --exports'); process.exit(3); }
const MAX_MB = +arg('--max-mb', '0');
const QUIET = has('--quiet'), TWO = has('--two'), ONE = has('--one-call'), JS_EVERY = +arg('--js-every', '100');
const text = (n: string) => existsSync(`${dir}/${n}`) ? readFileSync(`${dir}/${n}`, 'utf8') : '', lines = (n: string) => text(n).split('\n').filter(Boolean);
const post = lines('post.txt').map(l => l.split(' ')), head = post.shift() ?? [];
const meter = lines('meter.txt').map(Number);
const host = lines('host.txt').map(l => l.split(' ').map(Number)), gives = lines('gives.txt'), hostwrites = lines('hostwrites.txt');
let ticks = argv[1] && !argv[1].startsWith('--') ? +argv[1] : +text('ticks.txt') || host.length;

// ---- the module's memory. A view dies when the memory grows (and the game allocates inside a tick): every read and write asks for a fresh one.
const u8 = () => new Uint8Array(mem.buffer), u32 = () => new Uint32Array(mem.buffer);
const dec = new TextDecoder(), enc = new TextEncoder();
const outText = () => dec.decode(u8().subarray(X.face_out_ptr(), X.face_out_ptr() + X.face_out_len()));
const panicText = () => dec.decode(u8().subarray(X.face_panic_ptr(), X.face_panic_ptr() + X.face_panic_len()));
/** bytes handed to a call: written into the module's memory, the call made, the room freed */
function hand<T>(b: Uint8Array, f: (p: number, n: number) => T): T { const p = X.face_alloc(b.length); u8().set(b, p); try { return f(p, b.length); } finally { X.face_free(p, b.length); } }
const NAMES: string[] = []; for (let i = 0; i < X.face_parts(); i++) { X.face_part_name(i); NAMES.push(outText()); }
if (head[0] !== 'parts' || head.slice(1).join(' ') !== NAMES.join(' ')) { say(`post.txt names the block's parts [${head.slice(1)}] and this module [${NAMES}]: one side is of another tree`); process.exit(3); }
// (rust/sim's own exports and the face's say the same of the block: one block, reached by either name)
for (let slot = 0; slot < X.state_games(); slot++) for (let i = 0; i < NAMES.length; i++) if (X.state_addr(slot) + X.state_part(i, 0) !== X.face_part_addr(slot, i) || X.state_part(i, 1) !== X.face_part_len(i)) { say(`wasm-run: part ${NAMES[i]} of slot ${slot}: rust/sim's state_addr / state_part and the face's face_part_addr / face_part_len disagree`); process.exit(3); }
const POSES = hand(enc.encode('poses'), (p, n) => X.face_part(p, n));
/** the hashes of the block's parts as the module takes them */
const inside = (slot: number): number[] => { const at = X.face_part_hashes(slot) >>> 2; return Array.from(u32().subarray(at, at + NAMES.length)); };
/** … and taken here, over the module's memory in place (mission.ts `hashes`: a part's words up to the last that is not zero) */
function outside(slot: number): number[] {
  const w = u32();
  return NAMES.map((_, i) => { const off = X.face_part_addr(slot, i) >>> 2; let n = off + (X.face_part_len(i) >>> 2); while (n > off && w[n - 1] === 0) n--; let h = 0x811c9dc5; for (let j = off; j < n; j++) h = Math.imul(h ^ w[j], 0x01000193); return h >>> 0; });
}

// --reserve MB: room asked for and given back before the game is made, so the allocator holds it and a tick need not grow the memory to find it
{ const mb = +arg('--reserve', '0'); if (mb > 0) { const n = mb * 1048576, p = X.face_alloc(n); X.face_free(p, n); say(`wasm-run: ${mb} MB asked for and given back; memory ${(mem.buffer.byteLength / 1048576).toFixed(1)} MB`); } }
const start = readFileSync(`${dir}/c1.bin`), block = readFileSync(`${dir}/s0.bin`);
say(`wasm-run: rust/sim's block is ${X.state_len()} bytes here, ${block.length} in s0.bin; layout word ${X.state_layout() >>> 0}; the start ${start.length} bytes`);
const UPLOAD = 1, LIGHTS = 2;
let k = 0;
try {
  const games: number[] = [];
  for (let i = 0; i < (TWO ? 2 : 1); i++) { const h = hand(start, (p, n) => hand(block, (bp, bn) => X.face_new(p, n, bp, bn))); if (!h) throw new Error('face_new: no free slot'); games.push(h); if (i === 0 && outText()) say(outText());
    if (existsSync(`${dir}/lesson.json`)) { hand(readFileSync(`${dir}/lesson.json`), (p, n) => X.face_lesson(h, p, n)); if (i === 0 && outText()) say(outText()); } }
  const slots = games.map(h => X.face_slot(h));
  say(`wasm-run: ${games.length} game${TWO ? 's' : ''} in slot${TWO ? 's' : ''} ${slots.join(', ')}; memory after the import ${(mem.buffer.byteLength / 1048576).toFixed(1)} MB`);
  const st = games.map(() => ({ parted: 0, poolParted: 0, dieParted: 0, nParted: 0, wasSame: true, left: new Set<number>() }));
  const ORDER = has('--second-first') ? [1, 0] : games.map((_, i) => i);
  let spent = 0, worst = 0, worstAt = 0, lightsSpent = 0, grewTicks = 0, firstGrow = 0, jsChecks = 0, done = 0, stop = false;
  for (k = 1; k <= ticks && !stop; k++) {
    if (!existsSync(`${dir}/h${k}.bin`)) { say(`no h${k}.bin: the TypeScript's run ended at tick ${k - 1}`); break; }
    const hb = readFileSync(`${dir}/h${k}.bin`), dt = host[k - 1][0], up = (host[k - 1][4] ?? 0) > 0, p = post[k - 1];
    for (const gi of ORDER) {
      const h = games[gi], slot = slots[gi], S = st[gi], tag = TWO ? ` [game ${gi + 1}]` : '';
      hand(hb, (ptr, n) => X.face_host(h, ptr, n, k === 1 ? 1 : 0));
      for (const l of gives) if (+l.split(' ')[0] === k) { hand(enc.encode(l), (ptr, n) => X.face_give(h, k, ptr, n)); if (!QUIET) say(outText() + tag); }
      for (const l of hostwrites) if (+l.split(' ')[0] === k) { const r = hand(enc.encode(l), (ptr, n) => X.face_hostwrite(h, k, ptr, n)); if (r === 1 && !QUIET) say(outText() + tag); if (r === 0) say(`  NOTE tick ${k}: a line of hostwrites.txt the face does not know: ${l}`); }
      if (meter.length >= k) X.face_meter(h, meter[k - 1]);
      // (the box world's list is asked for by name outside the clock, as the native runner's is; --one-call hands it to the step as a flag)
      if (up && !ONE) X.face_upload(h);
      const b0 = mem.buffer.byteLength, a = performance.now();
      X.face_step(h, dt, ONE ? (up ? UPLOAD : 0) | LIGHTS : 0);
      const el = performance.now() - a;
      if (gi === 0) { spent += el; if (el > worst) { worst = el; worstAt = k; } if (mem.buffer.byteLength !== b0) { grewTicks++; firstGrow ||= k; } }
      const said = outText(); if (said && !QUIET) say(said.trimEnd() + tag);
      if (k === 1 && gi === 0 && existsSync(`${dir}/knobs.bin`)) {
        // (rust/sim's own exports, on this module: the knob table the Rust game wrote, beside the TypeScript module's)
        const want = new Float64Array(new Uint8Array(readFileSync(`${dir}/knobs.bin`)).buffer), n = X.knob_len(0), got = new Float64Array(mem.buffer, X.knob_addr(0), n);
        let bad = 0; for (let i = 0; i < Math.min(n, want.length); i++) if (!Object.is(got[i], want[i])) bad++;
        say(`KNOBS: ${bad} of ${n} written by the Rust game differ from the TypeScript module's table (${want.length} doubles)`);
      }
      const got = inside(slot), die = X.face_die_n(h);
      if (gi === 0 && (k === 1 || k === ticks || (JS_EVERY > 0 && k % JS_EVERY === 0))) { const mine = outside(slot); jsChecks++; if (mine.join() !== got.join()) { say(`TICK ${k}: the hashes taken here over the module's memory are not the module's own: ${mine} / ${got}`); process.exit(3); } }
      if (!ONE) { const a2 = performance.now(); X.face_lights(h, dt); if (gi === 0) lightsSpent += performance.now() - a2; }
      let off = p ? NAMES.filter((_, i) => p[4 + i] !== String(got[i])) : ['no line of post.txt'];
      const poolOff = off.includes(NAMES[POSES]); off = off.filter(n => n !== NAMES[POSES]);
      if (ONE && off.length && off.every(n => n === 'lights')) { if (!S.left.size) say(`TICK ${k}: the part \`lights\` differs after the lights' update ran inside the step (the TypeScript's hash is taken before its host's): left out from here${tag}`); S.left.add(NAMES.indexOf('lights')); off = []; }
      const same = off.length === 0;
      if (same && poolOff && !S.poolParted) { S.poolParted = k; say(`TICK ${k}: the part \`poses\` parts and every other part is the same: from here the alarm is the block without \`poses\`${tag}`); }
      const dieSame = !!p && p[1] === String(die);
      if (!dieSame && !S.dieParted) S.dieParted = k;
      const first = !same && !S.parted; if (first) S.parted = k;
      if (!same) { S.nParted++; } else if (S.nParted > 0 && !S.wasSame) say(`TICK ${k}: the block is the same again${tag}`);
      S.wasSame = same;
      if (first || k % 100 === 0 || k === ticks || !dieSame) {
        if (!QUIET || first || !dieSame || k === ticks) say(`TICK ${k}; time ${X.face_time(h).toFixed(4)}; die n=${die} (TypeScript ${p?.[1] ?? '?'}); block ${same ? 'the same' : `PARTS in [${off}]`}; boxes ${X.face_rows(h)}; mean tick ${(spent / k).toFixed(2)} ms, worst ${worst.toFixed(1)} ms; memory ${(mem.buffer.byteLength / 1048576).toFixed(0)} MB${tag}`);
      }
      if (S.parted) { say(`THE BLOCK FIRST PARTED AT TICK ${S.parted}${tag}: [${off}]`); stop = true;
        // (two games from one start, one in step and one not: where their blocks differ says what the second game did not have of its own)
        if (TWO && !st[1 - gi].parted) for (const n of off) { const i = NAMES.indexOf(n), a = X.face_part_addr(slots[1 - gi], i) >>> 2, b = X.face_part_addr(slot, i) >>> 2, w = u32(), d: string[] = []; let nd = 0;
          for (let j = 0; j < X.face_part_len(i) >>> 2; j++) if (w[a + j] !== w[b + j]) { nd++; if (d.length < 12) d.push(`+${j * 4}: ${w[a + j]} / ${w[b + j]}`); }
          say(`  \`${n}\` of the other game and of this game differ in ${nd} words (byte in the part: the other / this): ${d.join(', ')}`); } }
    }
    done = k;
    if (MAX_MB > 0 && mem.buffer.byteLength > MAX_MB * 1048576) { say(`TICK ${k}: the module's memory is ${(mem.buffer.byteLength / 1048576).toFixed(0)} MB, over --max-mb ${MAX_MB}: stopped here, in step so far (${X.face_rows(games[0])} rows of the box table)`); break; }
  }
  const S = st[0], bad = st.some(s => s.parted || s.dieParted);
  say(`${bad ? 'STOPPED' : 'DONE'}: ${done} ticks through wasm${TWO ? ', two games' : ''}${ONE ? ', one call a tick' : ''}; the die's count ${st.every(s => !s.dieParted) ? 'the same after every tick' : `FIRST PARTED AT TICK ${st.map(s => s.dieParted)}`}; the block without \`poses\`${S.left.size ? ' and `lights`' : ''} ${st.every(s => !s.parted) ? 'the same after every tick' : `PARTED AT TICK ${st.map(s => s.parted)}`}; \`poses\` ${S.poolParted ? `parted at tick ${st.map(s => s.poolParted)}` : 'the same after every tick'}`);
  say(`  checked: ${done} ticks x ${NAMES.length - 1 - S.left.size} parts by the module's own hashes, ${jsChecks} of those ticks also hashed here over the module's memory in place`);
  say(`  the step (face_step alone${ONE ? ', the upload and the lights\' update inside' : ''}): mean ${(spent / Math.max(1, done)).toFixed(3)} ms, worst ${worst.toFixed(1)} ms at tick ${worstAt}${ONE ? '' : `; the lights' update after it: mean ${(lightsSpent / Math.max(1, done)).toFixed(4)} ms`}`);
  say(`  memory: ${(mem.buffer.byteLength / 1048576).toFixed(1)} MB of linear memory at the end (${mem.buffer.byteLength} bytes), ${X.face_rows(games[0])} rows of the box table; it grew inside ${grewTicks} of ${done} steps, first in tick ${firstGrow}`);
  process.exit(bad ? 1 : 0);
} catch (e) {
  const words = (() => { try { return panicText(); } catch { return ''; } })();
  if (words) { say(`PANIC in tick ${k}: ${words}`); process.exit(2); }
  say(`wasm-run: tick ${k}: ${(e as Error)?.stack ?? e}`); process.exit((e as Error)?.name === 'RuntimeError' || (e as Error)?.name === 'RangeError' ? 2 : 3);
}
