// THE CHARACTER'S FILES OF THIS CHECKOUT (public/assets), for a tool that stands a game in the module. A page fetches the same files.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

let CHARACTER: Uint8Array[] | null = null;
/** read once: the mannequin's gltf, its buffer, the cast's gltf, the look's cast's gltf */
export function characterFiles(root: string): Uint8Array[] {
  const f = (name: string) => new Uint8Array(readFileSync(resolve(root, 'public/assets', name)));
  return CHARACTER ??= [f('ual/AnimationLibrary_Godot_Standard.gltf'), f('ual/AnimationLibrary_Godot_Standard.bin'), f('chars/cast.gltf'), f('chars/cast-gwot.gltf')];
}
