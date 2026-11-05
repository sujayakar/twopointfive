// THE TYPESCRIPT SIDE OF THE TICK LOOP: the real game at N consecutive ticks of a mission, as blobs for the Rust runner and the diff.
//   TPS_PROFILE=shipped bun run --preload ./tools/blitz/expose.ts tools/blitz/ts-ticks.ts <mission>:<seed> --out <dir> [--from K] [--ticks N] [--die n] [--dress 1]
// (run.sh calls it; tick k counts from 1 at the mission's tick K.) Writes into <dir>:
//   c<k>.bin   the state as tick k's step OPENS (c1 is the start both sides step from; every c<k> holds that tick's input record in cmd_buf)
//   a<k>.bin   the state as tick k returns            s<k>.bin   rust/sim's block as tick k returns        s0.bin   the block as tick 1 opens
//   engine.txt what the host holds that a tick reads (host.ts)        gives.txt   the test driver's orders, by the tick they were given before
//   host.txt   a line a tick: dt · the probe's asks pending · the game's time · the die's count · uploads of the box world before this tick
//   knobs.bin  the module's table of knobs at the end (only to check the Rust game's own write of them)        layout.json   the block's parts
// Exit 0 with every tick written · 5 it stopped early (the reason is printed) · 4 over the memory ceiling.
import { writeFileSync, mkdirSync } from 'node:fs';
import { stand, standArgs } from './stand';
const { exportGame, engineText } = await import('./host');
const say = (s = '') => process.stdout.write(s + '\n');
const argv = process.argv.slice(2); const oi = argv.indexOf('--out'); const out = oi < 0 ? '' : argv.splice(oi, 2)[1];
let o; try { o = standArgs(argv); if (!out) throw new Error('give --out <dir>'); } catch (e) { say(`ts-ticks: ${(e as Error).message}`); process.exit(64); }
mkdirSync(out, { recursive: true });
const { knobRegistry } = await import('../../src/game/knobs.ts');
const { simLayout } = await import('../../src/sim/state.ts');
const reg = knobRegistry();
const CEIL = +(process.env.BLITZ_CEIL_GB ?? 4) * 1e9;
const block = (g: any): Uint8Array => { const w: Uint32Array = g.sim.w; return new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice(); };
const host: string[] = []; let guards = 0, die = -1, over = false;
const r = await stand(o, {
  open(g, k, dt, uploads) {
    writeFileSync(`${out}/c${k}.bin`, exportGame(g, reg));
    if (k === 1) { writeFileSync(`${out}/s0.bin`, block(g)); writeFileSync(`${out}/engine.txt`, engineText(g)); }
    host.push(`${dt} ${g.probe?.pending?.length ?? 0} ${g.time} ${g.dice?.n ?? -1} ${uploads}`);
  },
  shut(g, k) {
    writeFileSync(`${out}/a${k}.bin`, exportGame(g, reg)); writeFileSync(`${out}/s${k}.bin`, block(g)); guards = g.guards.length; die = g.dice?.n ?? -1;
    if (process.memoryUsage().rss > CEIL) { over = true; throw new Error(`over the memory ceiling at tick ${k}: ${(process.memoryUsage().rss / 1e9).toFixed(1)} GB resident`); }
  },
});
{ const { sim } = await import('../../src/sim/sim.ts'); const x: any = sim(), at = x.knob_addr(0), len = x.knob_len(0) * 8; writeFileSync(`${out}/knobs.bin`, new Uint8Array(x.memory.buffer).slice(at, at + len)); }
writeFileSync(`${out}/layout.json`, JSON.stringify(simLayout()));
writeFileSync(`${out}/gives.txt`, r.gives.join('\n') + (r.gives.length ? '\n' : ''));
writeFileSync(`${out}/host.txt`, host.join('\n') + '\n');
if (r.ticks) say(`typescript: ${o.scn} seed ${o.seed}, mission ticks ${o.from}..${o.from + r.ticks - 1}, stand ${process.env.TPS_PROFILE ?? 'frozen'}; game time ${r.time.toFixed(4)}; die n=${die}; ${guards} guards; ${r.gives.length} orders of the driver's; ${(process.memoryUsage().rss / 1e9).toFixed(2)} GB resident`);
if (r.why) say(`typescript: ${r.why}`);
process.exit(over ? 4 : r.ticks >= o.ticks ? 0 : 5);
