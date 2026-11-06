// THE HOST'S HAND-OVER'S CHECK. Role a stands a game at frame F and writes the whole state; at F+1 it writes the whole state and the host's half. `cargo test --lib host_round` (HOST_BLOB=f0.bin HOST_HALF=h1.bin HOST_OUT=x.bin)
// imports F, takes the half and exports. Role b compares: the record and every camera and Input row that is no seat's must read as F+1 has them, and everything else as F has it.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/host-check.ts <scn>:<seed>:<F> --role a|b --dir d
import { tmpdir } from 'node:os';
import { writeFileSync, readFileSync, mkdirSync } from 'node:fs';
import { exportGame, hostHalf, bookOf } from './export';
import { decode } from './paths';
const argv = process.argv.slice(2); const [scn, seed, F] = argv[0].split(':'); const arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const dir = arg('--dir', `${tmpdir()}/hostc`), role = arg('--role', 'a'); mkdirSync(dir, { recursive: true }); const say = (x: string) => { process.stdout.write(x + '\n'); };
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts');
if (role === 'b') {
  const seatRows = JSON.parse(readFileSync(`${dir}/seats.json`, 'utf8')); const M = (b: string) => new Map(decode(new Uint8Array(readFileSync(`${dir}/${b}`))).map((r: any) => [r.path, r]));
  const f0 = M('f0.bin'), f1 = M('f1.bin'), x = M('x.bin'); const sameV = (a: any, b: any) => a?.bits === b?.bits;
  const isHost = (p: string) => { if (/^g\.cmd_buf\b/.test(p)) return true; const m = p.match(/^g\.tab\.(camera|input)\[(\d+)\]/); return !!m && !seatRows[m[1]].includes(+m[2]); };
  let moved = 0, kept = 0, bad = 0; const show: string[] = [];
  for (const [p, r] of x) { const w = isHost(p) ? f1.get(p) : f0.get(p); if (!w || !sameV(r, w)) { bad++; if (show.length < 12) show.push(`${p}: ${String((r as any).bits).slice(0, 50)} wanted ${String((w as any)?.bits).slice(0, 50)}`); } else if (isHost(p)) moved++; else kept++; }
  for (const p of f0.keys()) if (!x.has(p) && !isHost(p)) { bad++; if (show.length < 12) show.push(`${p}: gone`); }
  say(`${moved} values of the host's taken as the next frame has them, ${kept} others as they stood, ${bad} wrong`); for (const l of show) say('  ' + l); process.exit(bad ? 1 : 0);
}
class Stop extends Error {} const was = Game.prototype.update; let n = 0;
Game.prototype.update = function (this: any, ...a: unknown[]) { n++;
  if (n === +F) writeFileSync(`${dir}/f0.bin`, exportGame(this, reg));
  if (n === +F + 1) { writeFileSync(`${dir}/f1.bin`, exportGame(this, reg)); const bk: any = bookOf(this); const T: number[] = []; let h: Uint8Array = new Uint8Array(); for (let i = 0; i < 200; i++) { const t0 = performance.now(); h = hostHalf(this, reg, [this.cmdBuf]); T.push(performance.now() - t0); } T.sort((p, q) => p - q);
    writeFileSync(`${dir}/h1.bin`, h); writeFileSync(`${dir}/seats.json`, JSON.stringify({ camera: this.seats.map((s: any) => bk.idOf('CameraId', s.cam)), input: this.seats.map((s: any) => bk.idOf('InputId', s.input)) }));
    say(`host's half: ${h.length} bytes, ${T[100].toFixed(3)} ms median; ${bk.listOf('CameraId').length} camera rows, ${bk.listOf('InputId').length} Input rows, ${this.seats.length} seat(s)`); throw new Stop(); }
  return (was as any).apply(this, a); } as any;
console.log = () => {}; const run = scn.startsWith('tut-') ? (await import('../qa/lesson-run.ts')).runLesson(scn, { seed: +seed, sky: false, glow: false, floor: 0 }) : play(scn, +seed, 300, false, {}); await (run as Promise<unknown>).catch((e: unknown) => { if (!(e instanceof Stop)) throw e; }); process.exit(0);
