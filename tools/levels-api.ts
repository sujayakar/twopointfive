// The dev server's level files (tools/serve.ts mounts it): the game's page fetches a level by its file name (`?levelFile=<name>.json`, the missions list's
// plans) and the dev server answers from the repo's levels/ folder. Read only: the level editor that wrote through here is gone, and a level is written by
// its generator (tools/levels/).
//   GET  /levels/                → { levels: [{ name, bytes, mtime }] }   the repo's levels/*.json (not the *.lint.json sidecars)
//   GET  /levels/<name>.json     → the file            GET /levels/<name>.lint.json → its lint sidecar (404 if none)
// Dev-only by construction (the static build copies the files beside the page). <name> must match NAME_RE (no paths, no dots). It answers whoever the dev
// server answers (loopback unless HOST says otherwise): the files are the repo's own and nothing is written.
import { resolve, join } from 'node:path';
import { existsSync, readdirSync, statSync, readFileSync } from 'node:fs';

const ROOT = resolve(import.meta.dir, '..');

export const NAME_RE = /^[A-Za-z0-9_-]{1,64}$/;

const json = (status: number, body: unknown) => new Response(JSON.stringify(body), { status, headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-store' } });

/** Handle /levels/* or return null for anything else. `rootDir` = the repo root whose levels/ directory is served (a scratch dir in the probes). */
export async function handleLevels(req: Request, rootDir: string = ROOT): Promise<Response | null> {
  const url = new URL(req.url);
  let path: string; try { path = decodeURIComponent(url.pathname); } catch { return json(400, { error: 'bad path encoding' }); }
  if (path !== '/levels' && !path.startsWith('/levels/')) return null;
  const dir = join(rootDir, 'levels');
  if (req.method !== 'GET' && req.method !== 'HEAD') return json(405, { error: 'GET only: a level is written by its generator' });
  const rest = path.replace(/^\/levels\/?/, '');
  if (rest === '') {   // index
    if (!existsSync(dir)) return json(200, { levels: [] });
    const levels = readdirSync(dir).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json') && NAME_RE.test(f.slice(0, -5))).map(f => { const st = statSync(join(dir, f)); return { name: f.slice(0, -5), bytes: st.size, mtime: st.mtimeMs }; }).sort((a, b) => a.name.localeCompare(b.name));
    return json(200, { levels });
  }
  // a single file: <name>.json or <name>.lint.json — nothing else, no sub-paths
  const m = /^([^/]+?)(\.lint)?\.json$/.exec(rest);
  if (!m || !NAME_RE.test(m[1])) return json(400, { error: `bad level name '${rest}': use letters, digits, _ or - (1–64) + .json` });
  const name = m[1]; const sidecar = !!m[2]; const file = join(dir, `${name}${sidecar ? '.lint' : ''}.json`);
  if (!existsSync(file)) return json(404, { error: `no such level file: levels/${name}${sidecar ? '.lint' : ''}.json` });
  return new Response(req.method === 'HEAD' ? null : readFileSync(file), { status: 200, headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-store' } });
}
