// A FRAME CAPTURE IN PIECES (src/render/framecap.ts; the maintainer, 11-03: "can you make the captures for the entire frame? i don't want to have to think
// about the cursor"). The whole picture over two frames is 130 to 200 MB gzipped, and a file that big cannot always be handed on (a store that takes a
// file up to some size, a message's attachment): so the dev server writes a capture over one piece's size a second time, cut into pieces of that size
// beside it (tools/serve.ts POST /capture), and the reader takes any one piece and reads them all (tools/qa/capture-read.ts). A piece is a plain cut of the
// gzipped file: the pieces joined in order are that file, byte for byte. A capture under one piece's size is one file, as it was.
//   bun run tools/qa/capture-pieces.ts captures/<name>.tpscap.gz      cuts a capture that is already on disk (one taken before the server cut them)
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';

/** the most a piece holds */
export const PIECE_BYTES = 24 << 20;
const PIECE = /^(.*)\.part(\d+)of(\d+)\.tpscap\.gz$/;
/** piece `i` of `n` (from 0) of the capture called `name` */
export const pieceName = (name: string, i: number, n: number): string => `${name}.part${i + 1}of${n}.tpscap.gz`;
/** `bytes` cut into pieces of `size` at most, in order (one piece = the bytes themselves: nothing to cut) */
export function cutPieces(bytes: Uint8Array, size = PIECE_BYTES): Uint8Array[] {
  const n = Math.max(1, Math.ceil(bytes.length / size)), out: Uint8Array[] = [];
  for (let i = 0; i < n; i++) out.push(bytes.subarray(i * size, Math.min(bytes.length, (i + 1) * size)));
  return out;
}
/** a capture's name without its ending, whether `file` is the capture or one of its pieces */
export const captureName = (file: string): string => { const b = basename(file), m = PIECE.exec(b); return m ? m[1] : b.replace(/\.tpscap(\.gz)?$/, ''); };
/** the capture `file` names, as it was written: the file itself, or, where `file` is a piece, every piece beside it joined in order (a missing one is said by name) */
export function readCapture(file: string): Uint8Array {
  const m = PIECE.exec(basename(file)); if (!m) return new Uint8Array(readFileSync(file));
  const n = +m[3], parts: Uint8Array[] = [];
  for (let i = 0; i < n; i++) { const f = join(dirname(file), pieceName(m[1], i, n)); if (!existsSync(f)) throw new Error(`piece ${i + 1} of ${n} is not beside the others: ${f}`); parts.push(new Uint8Array(readFileSync(f))); }
  // (every piece but the last is as long as the first: one cut short on its way is named here, not left to the unzip to choke on)
  for (let i = 0; i < n; i++) if (i < n - 1 ? parts[i].length !== parts[0].length : parts[i].length > parts[0].length) throw new Error(`piece ${i + 1} of ${n} is ${parts[i].length} bytes where the first is ${parts[0].length}: it did not arrive whole`);
  const out = new Uint8Array(parts.reduce((s, p) => s + p.length, 0)); let at = 0; for (const p of parts) { out.set(p, at); at += p.length; }
  return out;
}
/** `bytes` written as pieces beside where the capture called `name` lies in `dir`: the files' names (none where it fits one piece) */
export function writePieces(dir: string, name: string, bytes: Uint8Array): string[] {
  const cut = cutPieces(bytes); if (cut.length < 2) return [];
  return cut.map((c, i) => { const f = join(dir, pieceName(name, i, cut.length)); writeFileSync(f, c); return f; });
}
if (import.meta.main) {
  const file = process.argv[2]; if (!file || !existsSync(file) || PIECE.test(basename(file))) { console.error('usage: bun run tools/qa/capture-pieces.ts <a whole capture.tpscap.gz>'); process.exit(2); }
  const made = writePieces(dirname(file), captureName(file), new Uint8Array(readFileSync(file)));
  console.log(made.length ? `${made.length} pieces:\n${made.join('\n')}` : 'it fits one piece: hand it on as it is');
}
