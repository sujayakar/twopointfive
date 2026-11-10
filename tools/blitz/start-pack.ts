// WHAT A HOST HANDS THE GAME'S OWN START, as one run of bytes (rust/game_start/src/handed.rs is the reader, and says the layout: a count, then each
// part as its length and its bytes, every number 32 bits with the little end first). The parts, in order: the level file's text, the address's
// words (empty: none), the mannequin's gltf and its buffer, the cast's gltf, the look's cast's gltf (each cast empty: none), what
// the host does to the game before it is made (its commands, src/draw/commands.ts; empty: nothing).
// For a tool: the character's files are this checkout's (public/assets). A page fetches the same files and packs them the same way.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

/** the parts as one run of bytes */
export function pack(parts: readonly Uint8Array[]): Uint8Array {
  const out = new Uint8Array(4 + parts.reduce((n, p) => n + 4 + p.length, 0)), v = new DataView(out.buffer); let at = 4;
  v.setUint32(0, parts.length, true);
  for (const p of parts) { v.setUint32(at, p.length, true); out.set(p, at + 4); at += 4 + p.length; }
  return out;
}
let CHARACTER: Uint8Array[] | null = null;
/** the character's files of this checkout, read once: the mannequin's gltf, its buffer, the cast's gltf, the look's cast's gltf */
export function characterFiles(root: string): Uint8Array[] {
  const f = (name: string) => new Uint8Array(readFileSync(resolve(root, 'public/assets', name)));
  return CHARACTER ??= [f('ual/AnimationLibrary_Godot_Standard.gltf'), f('ual/AnimationLibrary_Godot_Standard.bin'), f('chars/cast.gltf'), f('chars/cast-gwot.gltf')];
}
/** a start as a tool hands it: the level file's text, the link a mission is started from ('' = the level as its file has it), this checkout's
 *  character, and no commands */
export function packStart(root: string, levelText: Uint8Array, link: string): Uint8Array {
  return pack([levelText, new TextEncoder().encode(link), ...characterFiles(root), new Uint8Array(0)]);
}
