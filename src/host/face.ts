// THE RUST GAME'S MODULE, for a host that holds no game: fetched, compiled and called. Every game the page has is a number (a handle) the module answers;
// what it draws lies in the module's memory (src/draw/drawn.ts). A trap inside a call is the module's abort on a panic, whose words it kept.

export type Face = Record<string, (...a: number[]) => number>;
/** a failure that is told to the player in its own words */
export class FaceError extends Error { constructor(m: string) { super(m); this.name = 'RustGameError'; } }

export interface Fetched { exports: WebAssembly.Exports; bytes: number; fetchMs: number; compileMs: number; /** the file's own name, as its server has it: another module has another */ name?: string }
/** THE MODULE'S FILE, compiled off the page's thread. The dev server builds it where it is stale and answers 202 with a line to show while it does; a static
 *  host answers the file or 404. */
export async function fetchFace(say: (line: string) => void): Promise<Fetched> {
  const t0 = performance.now(); let res: Response;
  for (;;) {
    try { res = await fetch(new URL('game_face.wasm', location.href), { cache: 'no-store' }); } catch (e) { throw new FaceError(`The game's module could not be fetched (${String((e as Error)?.message ?? e)}). Is the dev server still running?`); }
    if (res.status !== 202) break;
    say((await res.text().catch(() => '')).slice(0, 400) || 'Building the game …');
    await new Promise(r => setTimeout(r, 2000));
  }
  if (!res.ok) { const why = (await res.text().catch(() => '')).slice(0, 900); throw new FaceError(`There is no game to play here. ${res.status === 404 ? 'This build was made without game_face.wasm (cargo and the wasm32-unknown-unknown target build it: tools/blitz/face-build.ts).' : why}`); }
  say('loading the game …');
  const t1 = performance.now(); let bytes: number, mod: WebAssembly.Module;
  if (typeof WebAssembly.compileStreaming === 'function' && (res.headers.get('content-type') ?? '').startsWith('application/wasm')) { bytes = Number(res.headers.get('content-length') ?? 0); mod = await WebAssembly.compileStreaming(res); }
  else { const b = await res.arrayBuffer(); bytes = b.byteLength; mod = await WebAssembly.compile(b); }
  // (a module of other sources may want what this page does not hand it)
  const inst = await WebAssembly.instantiate(mod).catch((e: unknown) => { throw new FaceError(`The game's module is of other sources than this page (${String((e as Error)?.message ?? e)}): tools/blitz/face-build.ts builds it.`); });
  return { exports: inst.exports, bytes, fetchMs: t1 - t0, compileMs: performance.now() - t1, name: (res.headers.get('etag') ?? res.headers.get('last-modified') ?? '').replace(/^W\//, '').replace(/"/g, '') };
}
