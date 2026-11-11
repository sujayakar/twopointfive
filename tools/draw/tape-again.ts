// A RUN PLAYED AGAIN AS IT WOULD GO NOW: a tape's hands and commands, from its start, on this checkout's module and level file, taped anew. A step that moves a level or how it plays makes a replay part from
// its tape; this gives the run those same hands make today. What they no longer reach (a door that stands elsewhere now) is whoever reads its sounds' to say.
//   bun run tools/draw/tape-again.ts <tape[.gz] | a kept visit's lines> <out.tape.gz>      exit 0 · 1 it cannot be played
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { gunzipSync, gzipSync } from 'node:zlib';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/character.ts';
import { DrawGame } from '../../src/host/drawgame';
import { parseLines } from '../../src/tape/kept';
import { HandTape, HandWalk } from '../../src/tape/handtape';
import { handsOf } from '../../src/tape/keep';

const ROOT = resolve(import.meta.dir, '../..'), [file, to] = process.argv.slice(2); if (!file || !to) { console.error('tape-again: <tape> <out>'); process.exit(1); }
export function tapeIn(path: string): Uint8Array { let b = new Uint8Array(readFileSync(path)); if (b[0] === 0x1f && b[1] === 0x8b) b = new Uint8Array(gunzipSync(b)); return b[0] === 0x7b ? handsOf(parseLines(new TextDecoder().decode(b)).lines) ?? b : b; }
const wasm = faceHere(); if (!wasm) { console.error('tape-again: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const g = new DrawGame({ exports: new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array(readFileSync(wasm)))).exports, bytes: 0, fetchMs: 0, compileMs: 0 });
const W = new HandWalk(tapeIn(file)), S = W.start, [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT), T = g.tape = new HandTape();
g.start({ level: new Uint8Array(readFileSync(resolve(ROOT, 'levels', S.levelName))), words: S.words, kept: S.kept, mannequin, mannequinBin, cast, castLook, seed: S.seed, seats: S.seats, named: { level: S.levelName, build: '' } });
let why = '';
for (let r = W.next(); r; r = W.next()) {
  if (r.kind === 'step') { g.numbersOf().all.set(r.hand); g.step(r.dt, r.keys); }
  else if (r.kind === 'order') { g.commands.put(r.bytes); g.tell(); }
  else if (r.kind === 'hands') g.handsOff(r.plan);
  else if (r.kind === 'end') why = r.why;
}
T.end(why); writeFileSync(to, gzipSync(T.whole(), { level: 9 })); console.log(`${S.levelName} ${S.words} · ${T.ticks} ticks · ${(gzipSync(T.whole()).length / 1024).toFixed(0)} KB`);
