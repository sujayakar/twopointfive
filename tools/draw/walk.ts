// A TAPE READ: what it is of, and walked to its end (or a tick) on this checkout's module, with no page: is it still the run that was played, and how did it stand there.
//   bun run tools/draw/walk.ts <tape | a kept visit's lines> [--to tick]      (either may be gzipped)      exit 0 it is the run · 1 it parts, or cannot be walked here · 2 no such file
import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { gunzipSync } from 'node:zlib';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { DrawGame } from '../../src/host/drawgame';
import { DrawPlay } from '../../src/host/drawplay';
import { parseLines } from '../../src/replay/kept';
import { handsOf } from '../../src/tape/keep';

const [file, ...rest] = process.argv.slice(2), to = rest[0] === '--to' ? Number(rest[1]) : Infinity;
if (!file || !existsSync(file)) { console.error('walk: which file?'); process.exit(2); }
let b = new Uint8Array(readFileSync(file)); if (b[0] === 0x1f && b[1] === 0x8b) b = new Uint8Array(gunzipSync(b));
if (b[0] === 0x7b) { const t = handsOf(parseLines(new TextDecoder().decode(b)).lines); if (!t) { console.error('walk: no tape of hands in these lines'); process.exit(1); } b = t; }
const wasm = faceHere(); if (!wasm) { console.error('walk: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const game = new DrawGame({ exports: new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm))), { env: { face_roll: () => { throw new Error('the game asked a die'); } } }).exports, bytes: 0, fetchMs: 0, compileMs: 0 });
const [mannequin, mannequinBin, cast, castLook] = characterFiles(resolve(import.meta.dir, '../..')), secs = (t: number): string => (t / 60).toFixed(1);
try {
  const P = new DrawPlay(game, b), S = P.walk.start; console.log(`${S.levelName} · ${S.words} · seed ${S.seed} · build ${S.build || '?'} · ${P.total} ticks (${secs(P.total)} s)${P.tries.length ? ` · begun again at ${P.tries.map(secs).join(' ')} s` : ''}${P.ends ? ` · ends: ${P.ends.split('\n')[0]}` : ''}`);
  P.start({ mannequin, mannequinBin, cast, castLook }); while (P.at < to && P.tick(false)) { /* on */ }
  const v = game.lost ? null : game.verdict(), t = game.lost ? null : game.tally();
  console.log(`${P.lost ? `lost at tick ${P.at}: ${P.lost.message.split('\n')[0]}` : `tick ${P.at}`}${t ? ` · ${t.stage} · ${t.men.length} men · ${t.manual + t.autosaves} saves, ${t.loads} loads` : ''}${v ? ` · ${v.word}: ${v.why}` : ''}`);
  console.log(P.parted ? `PARTED by tick ${P.parted.tick} (${secs(P.parted.tick)} s): ${JSON.stringify(P.parted.mine)}, on the tape ${JSON.stringify(P.parted.said)}` : 'as it was played');
  process.exit(P.parted || (P.lost && P.lost.message !== P.ends) ? 1 : 0);
} catch (e) { console.error(`walk: ${(e as Error).message}`); process.exit(1); }
