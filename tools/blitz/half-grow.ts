// A KEPT RUN'S HOST HALVES GIVEN THE HOST'S OWN ROWS (tools/blitz/hold.sh). The host's half of a tick is each seat's record, then every camera row, then
// every Input row, then the seats' marks and a word (export.ts hostHalfParts). Since every host hands the game its camera and its Input at the stand
// (game.ts hostCam), a host that steps by records alone (seats.ts: a watcher, a server) has those two rows in its half too, where a run kept before had
// none: its halves are a camera row and an Input row short, and not at their end.
// WHAT THIS DOES: each kept half is given the two rows AS THE FRESH START'S TICK 1 HAS THEM, at the places the fresh start says they stand
// (hostrows.txt, written beside the fresh half by seats.ts / mission.ts --start-only: export.ts hostRowsText). The rest of every kept half stays byte
// for byte.
// WHAT THAT IS, PLAINLY: A STAND-IN. A fresh run of the same game would move those two rows every tick (seats.ts drives its first seat through the
// host's own Input and camera), and the kept run never recorded them, so nothing can give a kept tick 500 the rows a fresh tick 500 has. The grown
// run holds because NO RULE OF THE TICK READS THE HOST'S OWN ROWS on a host that steps by records (the records are the seats' whole say, and the two
// fields are on the wire's hosts-own list, in no state word). The day a rule does read them there, a grown run parts from a fresh one and has to be
// kept anew: this tool cannot show that day has not come, only hold.sh's replay of the whole run can, tick by tick.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/half-grow.ts <the kept run's folder> <the fresh start's folder>
//   HALFGROW_BREAK=1 tools/blitz/hold.sh tools/blitz/kept      must read NOT HELD for every run that needs its halves grown (exit 3)
// Refused (exit 1, the reason said, NOTHING WRITTEN) unless: every kept half, the first and the last and each between, is short of the fresh one by
// exactly one camera row and one Input row and reads to its last byte in the half's own shape; and the kept half of tick 1, with the two rows put in,
// IS the fresh half of tick 1 up to the fresh half's own longer tail. So a start that moved, or a half that changed in any other way at any tick, is
// never papered over. A run whose host DID have the rows (one seat, stepped by update) needs none of this: its halves have the fresh shape already
// and hold.sh never calls here.
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { parse } from './export.ts';
import { skipper } from './import.ts';
const [kept, fresh] = process.argv.slice(2);
const no = (why: string): never => { console.log(why); process.exit(1); };
if (!kept || !fresh) no('half-grow.ts <the kept run\'s folder> <the fresh start\'s folder>');
const CMD = parse('crate::cmd::PlayerCmd'), CAM = parse('crate::camera::FollowCamera'), INPUT = parse('crate::input::Input');
/** a half's pieces: the records (with their count), each camera row, each Input row, and what follows (the seats' marks and a word); a reason where the
 *  bytes are not a half's to their last */
function cut(b: Uint8Array): { head: Uint8Array; cams: Uint8Array[]; ins: Uint8Array[]; rest: Uint8Array } | string {
  try {
    const I = { p: 0 }, { sk, dv } = skipper(b, I); const rows = (N: any): Uint8Array[] => { const n = dv.getUint32(I.p, true); I.p += 4; const out: Uint8Array[] = []; if (n > b.length) throw new Error(`a count of ${n} rows in ${b.length} bytes`); for (let i = 0; i < n; i++) { const a = I.p; sk(N); if (I.p > b.length) throw new Error('a row runs past the bytes'); out.push(b.subarray(a, I.p)); } return out; };
    const n = dv.getUint32(0, true); I.p = 4; if (n > b.length) throw new Error(`a count of ${n} records in ${b.length} bytes`); for (let i = 0; i < n; i++) { sk(CMD); if (I.p > b.length) throw new Error('a record runs past the bytes'); } const head = b.subarray(0, I.p);
    const cams = rows(CAM), ins = rows(INPUT), rest = b.subarray(I.p);
    const marks = rest.length >= 4 ? new DataView(rest.buffer, rest.byteOffset, rest.byteLength).getUint32(0, true) : -1;
    if (marks < 0 || (rest.length !== 4 + 4 * marks && rest.length !== 8 + 4 * marks)) return `its end is not the seats' marks and at most one word (${rest.length} bytes after the Input rows, ${marks} marks said)`;   // (a half from before the last word was added ends at the marks: hold.sh's end-grown rule)
    return { head, cams, ins, rest };
  } catch (e) { return `it does not read as a half (${String((e as Error)?.message ?? e).slice(0, 120)})`; }
}
const u32 = (n: number) => { const a = new Uint8Array(4); new DataView(a.buffer).setUint32(0, n, true); return a; };
const join = (P: Uint8Array[]) => { const out = new Uint8Array(P.reduce((n, p) => n + p.length, 0)); let at = 0; for (const p of P) { out.set(p, at); at += p.length; } return out; };
const same = (a: Uint8Array, b: Uint8Array) => a.length === b.length && a.every((x, i) => x === b[i]);
let at: { cam?: string; input?: string } = {};
try { at = Object.fromEntries(readFileSync(`${fresh}/hostrows.txt`, 'utf8').trim().split('\n').map(l => l.split(' '))); } catch { no('the fresh start wrote no hostrows.txt: it is of a tree from before a host handed its rows over, or it did not stand'); }
const F0 = cut(new Uint8Array(readFileSync(`${fresh}/h1.bin`))); if (typeof F0 === 'string') no(`the fresh half of tick 1: ${F0}`);
const F = F0 as Exclude<typeof F0, string>, ci = Number(at.cam), ii = Number(at.input);
if (!(Number.isInteger(ci) && ci >= 0 && ci < F.cams.length && Number.isInteger(ii) && ii >= 0 && ii < F.ins.length)) no(`the fresh start names no host row (hostrows.txt: cam ${at.cam}, input ${at.input}; the fresh half holds ${F.cams.length} camera and ${F.ins.length} Input rows)`);
const grown = (b: Uint8Array): Uint8Array | string => { const K = cut(b); if (typeof K === 'string') return K;
  if (K.cams.length !== F.cams.length - 1 || K.ins.length !== F.ins.length - 1) return `it holds ${K.cams.length} camera and ${K.ins.length} Input rows where the fresh one holds ${F.cams.length} and ${F.ins.length}: not one of each short`;
  const cams = [...K.cams.slice(0, ci), F.cams[ci], ...K.cams.slice(ci)], ins = [...K.ins.slice(0, ii), F.ins[ii], ...K.ins.slice(ii)];
  return join([K.head, u32(cams.length), ...cams, u32(ins.length), ...ins, K.rest, F.rest.subarray(Math.min(F.rest.length, K.rest.length))]); };
const files = readdirSync(kept).filter(f => /^h\d+\.bin$/.test(f)).sort((a, b) => +a.slice(1, -4) - +b.slice(1, -4));
if (!files.includes('h1.bin')) no('the kept run has no half of tick 1');
// (every half is made before one is written: a refusal at any tick leaves the kept run as it was)
const made: [string, Uint8Array][] = [];
for (const f of files) { const g = grown(new Uint8Array(readFileSync(`${kept}/${f}`))); if (typeof g === 'string') no(`${f}: ${g}`); made.push([f, g as Uint8Array]); }
if (process.env.HALFGROW_BREAK === '1') made[0][1][made[0][1].length >> 1] ^= 1;   // (HALFGROW_BREAK=1 in front of hold.sh: one bit of the grown tick 1 turned, and every run that needs growing must read NOT HELD)
if (!same(made[0][1], new Uint8Array(readFileSync(`${fresh}/h1.bin`)))) no('the kept half of tick 1, given the host\'s two rows, is not the fresh half of tick 1: the start moved, or the half changed in another way');
for (const [f, g] of made) writeFileSync(`${kept}/${f}`, g);
console.log(`${files.length} halves (ticks 1 to ${files[files.length - 1].slice(1, -4)}, each read to its last byte) given the host's camera row (${ci} of ${F.cams.length}) and Input row (${ii} of ${F.ins.length}) as tick 1's stand-in`);
process.exit(0);
