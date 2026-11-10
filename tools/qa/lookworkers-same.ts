// what "the same maps" means, for the bun check and for the page tool's entry (both import it; no node module here, so a browser bundle can take it)
import type { LookMaps } from '../../src/render/lookTex';
export function sameMaps(a: LookMaps, b: LookMaps): boolean {
  if (a.tone !== b.tone || a.hollow !== b.hollow || a.a.length !== b.a.length || a.b.length !== b.b.length) return false;
  for (let i = 0; i < a.a.length; i++) if (a.a[i] !== b.a[i] || a.b[i] !== b.b[i]) return false;
  return true;
}
