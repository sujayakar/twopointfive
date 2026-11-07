// A KEPT RUN'S HOST HALVES GIVEN THE HOST'S OWN ROWS (tools/blitz/hold.sh). The host's half of a tick is each seat's record, then every camera row, then
// every Input row, then the seats' marks and a word (export.ts hostHalfParts). Since every host hands the game its camera and its Input at the stand
// (game.ts hostCam), a host that steps by records alone (seats.ts: a watcher, a server) has those two rows too, where a run kept before had none: its
// halves are a camera row and an Input row short, and not at their end. Nothing moves those two rows on such a host: no record is sampled from them and
// nothing steps the camera. So each kept half is given the two rows AS TICK 1's FRESH HALF HAS THEM, at the places the fresh start says they stand
// (hostrows.txt, written beside the fresh half by seats.ts / mission.ts --start-only), and the rest of every kept half stays byte for byte.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/half-grow.ts <the kept run's folder> <the fresh start's folder>
// Refused (exit 1, the reason said) unless the kept half of tick 1, with the two rows put in, IS the fresh half of tick 1 up to the fresh half's own
// longer tail: so a start that moved, or a half that changed in any other way, is never papered over. A run whose host DID have the rows (one seat,
// stepped by update) needs none of this: its halves have the fresh shape already and hold.sh never calls here.
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { parse } from './export.ts';
import { skipper } from './import.ts';
const [kept, fresh] = process.argv.slice(2);
const no = (why: string): never => { console.log(why); process.exit(1); };
if (!kept || !fresh) no('half-grow.ts <the kept run\'s folder> <the fresh start\'s folder>');
const CMD = parse('crate::cmd::PlayerCmd'), CAM = parse('crate::camera::FollowCamera'), INPUT = parse('crate::input::Input');
/** a half's pieces: the records (with their count), each camera row, each Input row, and what follows */
function cut(b: Uint8Array): { head: Uint8Array; cams: Uint8Array[]; ins: Uint8Array[]; rest: Uint8Array } {
  const I = { p: 0 }, { sk, dv } = skipper(b, I); const rows = (N: any): Uint8Array[] => { const n = dv.getUint32(I.p, true); I.p += 4; const out: Uint8Array[] = []; for (let i = 0; i < n; i++) { const a = I.p; sk(N); out.push(b.subarray(a, I.p)); } return out; };
  const n = dv.getUint32(0, true); I.p = 4; for (let i = 0; i < n; i++) sk(CMD); const head = b.subarray(0, I.p);
  const cams = rows(CAM), ins = rows(INPUT); return { head, cams, ins, rest: b.subarray(I.p) };
}
const u32 = (n: number) => { const a = new Uint8Array(4); new DataView(a.buffer).setUint32(0, n, true); return a; };
const join = (P: Uint8Array[]) => { const out = new Uint8Array(P.reduce((n, p) => n + p.length, 0)); let at = 0; for (const p of P) { out.set(p, at); at += p.length; } return out; };
const same = (a: Uint8Array, b: Uint8Array) => a.length === b.length && a.every((x, i) => x === b[i]);
const at = Object.fromEntries(readFileSync(`${fresh}/hostrows.txt`, 'utf8').trim().split('\n').map(l => l.split(' '))) as { cam: string; input: string };
const F = cut(new Uint8Array(readFileSync(`${fresh}/h1.bin`))), ci = +at.cam, ii = +at.input;
if (!(ci >= 0 && ci < F.cams.length && ii >= 0 && ii < F.ins.length)) no(`the fresh start names no host row (hostrows.txt: cam ${at.cam}, input ${at.input})`);
const grown = (b: Uint8Array): Uint8Array | string => { const K = cut(b);
  if (K.cams.length !== F.cams.length - 1 || K.ins.length !== F.ins.length - 1) return `it holds ${K.cams.length} camera and ${K.ins.length} Input rows where the fresh one holds ${F.cams.length} and ${F.ins.length}: not one of each short`;
  const cams = [...K.cams.slice(0, ci), F.cams[ci], ...K.cams.slice(ci)], ins = [...K.ins.slice(0, ii), F.ins[ii], ...K.ins.slice(ii)];
  return join([K.head, u32(cams.length), ...cams, u32(ins.length), ...ins, K.rest, F.rest.subarray(Math.min(F.rest.length, K.rest.length))]); };
const files = readdirSync(kept).filter(f => /^h\d+\.bin$/.test(f));
{ const g1 = grown(new Uint8Array(readFileSync(`${kept}/h1.bin`))); if (typeof g1 === 'string') no(`the kept half of tick 1: ${g1}`);
  if (!same(g1 as Uint8Array, new Uint8Array(readFileSync(`${fresh}/h1.bin`)))) no('the kept half of tick 1, given the host\'s two rows, is not the fresh half of tick 1: the start moved, or the half changed in another way'); }
for (const f of files) { const g = grown(new Uint8Array(readFileSync(`${kept}/${f}`))); if (typeof g === 'string') no(`${f}: ${g}`); writeFileSync(`${kept}/${f}`, g as Uint8Array); }
console.log(`${files.length} halves given the host's camera row (${ci} of ${F.cams.length}) and Input row (${ii} of ${F.ins.length})`);
process.exit(0);
