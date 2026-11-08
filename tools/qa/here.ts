// WHERE THE CHECKOUT IS, AND A PLATFORM WITH NO GPU: the enums a render module names as it loads, and a stand-in for the device. Nothing of the game is loaded:
// a check of the picture's own code imports this; a check that stands a game imports headless.ts.
import { resolve } from 'node:path';

export const ROOT = resolve(import.meta.dir, '../..');

const G = globalThis as any;
G.GPUBufferUsage ??= { MAP_READ: 1, MAP_WRITE: 2, COPY_SRC: 4, COPY_DST: 8, INDEX: 16, VERTEX: 32, UNIFORM: 64, STORAGE: 128, INDIRECT: 256, QUERY_RESOLVE: 512 };
G.GPUTextureUsage ??= { COPY_SRC: 1, COPY_DST: 2, TEXTURE_BINDING: 4, STORAGE_BINDING: 8, RENDER_ATTACHMENT: 16 };
G.GPUShaderStage ??= { VERTEX: 1, FRAGMENT: 2, COMPUTE: 4 };
G.GPUMapMode ??= { READ: 1, WRITE: 2 };
G.GPUColorWrite ??= { RED: 1, GREEN: 2, BLUE: 4, ALPHA: 8, ALL: 15 };

/** an object that is anything you ask of it: every property is another, every call returns one, promises never settle (shader compile info and a closed error scope apart) */
export function fake(name = 'gpu'): any {
  return new Proxy(function () { /* callable */ }, {
    get(_t, p) {
      if (p === 'then') return undefined;
      if (p === Symbol.toPrimitive) return () => 0;
      if (p === 'toString' || p === Symbol.toStringTag) return () => `[fake ${name}]`;
      if (p === 'getCompilationInfo') return () => Promise.resolve({ messages: [] });
      if (p === 'mapAsync') return () => new Promise(() => { /* no readbacks */ });
      if (p === 'popErrorScope') return () => Promise.resolve(null);
      if (p === 'label') return name;
      return fake(`${name}.${String(p)}`);
    },
    apply: () => fake(`${name}()`), construct: () => fake(`${name}#new`), set: () => true,
  });
}
