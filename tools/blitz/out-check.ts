// THE OUT EVENTS' CHECK. `cargo test --lib out_round` (OUT_BLOB=<a whole state> OUT_FILE=<out>) imports a state, calls every generated function once and writes a filtered start with the 33 events on its tail. This stands a game at
// the same frame, applies that file, and runs each event through the dispatcher with stand-ins on the receivers: each must make the right call with the right number of arguments, none of them a raw ID where an object is meant.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/out-check.ts <scn>:<seed>:<F> --file /tmp/out0.bin
import { tmpdir } from 'node:os';
import { readFileSync } from 'node:fs';
import { exportGame } from './export';
import { applyGame } from './import';
import { classTable, valueTable } from './classes';
import { puppetRows, SLOW, VALS } from './puppet-apply';
import { OUT, dispatchOut } from './out-dispatch';
const argv = process.argv.slice(2); const [scn, seed, F] = argv[0].split(':'); const arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const file = arg('--file', `${tmpdir()}/out0.bin`); const say = (x: string) => { process.stdout.write(x + '\n'); };
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts'); class Stop extends Error {}
const LJ = JSON.parse(readFileSync(`${import.meta.dir}/puppet-lists.json`, 'utf8')); const lists = { step: new Set<string>(LJ.step), start: new Set<string>(LJ.start) }; const hooks = { classes: classTable(), values: valueTable() };
const was = Game.prototype.update; let n = 0, code = 0;
Game.prototype.update = function (this: any, ...a: unknown[]) { n++; if (n === +F) { Game.prototype.update = was; try { check(this); } finally { process.exit(code); } } return (was as any).apply(this, a); };
const show = (v: any): string => v === undefined ? 'undefined' : v === null ? 'null' : typeof v === 'object' ? (v.constructor?.name ?? 'object') + (Array.isArray(v) ? `[${v.length}]` : '') : JSON.stringify(v);
function check(g: any): void {
  exportGame(g, reg); const ctx: any = { p: 0 }; const rows = puppetRows(ctx);
  const R: any = applyGame(g, new Uint8Array(readFileSync(file)), reg, { hooks, puppet: { lists, start: true, rows, ctx, slowTable: SLOW, vals: VALS } } as any);
  say(`${R.out.length} events`);
  const calls: string[] = []; const stand = (name: string) => new Proxy({}, { get: (_, m) => (...a: any[]) => { calls.push(`${name}.${String(m)}(${a.map(show).join(', ')})`); } });
  // a method goes to a stand-in that records it; a function of the game runs for real on the receivers the game has
  const real = { saves: g.saves, decals: g.decals, audio: g.audio, engine: g.engine, moths: g.moths, smoke: g.smoke, wear: g.destr.walls.wear };
  const stands = (on: boolean) => { g.saves = on ? stand('saves') : real.saves; g.decals = on ? stand('decals') : real.decals; g.audio = on ? stand('audio') : real.audio; g.engine = on ? { ...real.engine, particles: stand('particles'), reach: stand('reach'), restartPicture: (...a: any[]) => { calls.push(`engine.restartPicture(${a.map(show).join(', ')})`); } } : real.engine; g.moths = on ? stand('moths') : real.moths; g.smoke = on ? stand('smoke') : real.smoke; g.destr.walls.wear = on ? stand('wear') : real.wear; };
  const FREE = /^(Mess|BangScorch|BulletMark|ChipMark|KickScuff|ClearSnow|SnowBlast)/; if (!g.rolls.some((r: any) => r.voice === 7)) { const p = g.props?.props?.[0]; if (p) g.rolls.push({ p, x: p.x, z: p.z, k: 0.5, voice: 7, moveT: g.time }); }
  const seen = new Set<string>();
  for (const E of R.out as any[]) { const fn = OUT[E.arm]; seen.add(E.arm); calls.length = 0; stands(!FREE.test(E.arm)); if (!fn) { say(`  ${E.arm}: NO DISPATCHER`); code = 1; continue; }
    try { dispatchOut(g, [E]); say(`  ${E.arm}: ${calls.join('; ') || '(a function of the game ran)'}`); if (E.arm === 'GameAudioLikePlay') say(`      opts ${JSON.stringify(E.e.opts)}`); } catch (x: any) { say(`  ${E.arm}: THREW ${x.message} ${String(x.stack).split("\n").slice(1,4).join(" | ")}  payload ${JSON.stringify(E.e, (_k, v) => typeof v === 'object' && v && v.constructor !== Object && !Array.isArray(v) ? v.constructor.name : v)}`); code = 1; } }
  for (const k of Object.keys(OUT)) if (!seen.has(k)) { say(`  ${k}: never arrived`); code = 1; }
  say(code ? 'RED' : `green: ${seen.size} of ${Object.keys(OUT).length} arms`);
}
console.log = () => {};
await play(scn, +seed, 300, false, {}).catch((x: any) => { say(`play: ${x?.message ?? x}`); });
