// THE LOCAL BUILD'S BACKEND: a Convex deployment on this machine, started beside the dev server (the maintainer, 10-27: "we can make the local build use a
// local convex deployment"). `bun run dev` sets TPS_LOCAL_CLOUD=1; tools/serve.ts then calls `localCloud()` and answers the page's `GET /qa/cloud` with
// what it returns (src/cloud/cloud.ts cloudFind). A plain `bun run tools/serve.ts` — every check's dev server — starts nothing and answers with nothing.
//
//   TPS_CLOUD_URL=http://127.0.0.1:3210   use the backend that is already there (a stand-in, a deployment started by hand): nothing is started
//   TPS_LOCAL_CLOUD=1                     start one: `convex dev` in cloud/, which runs Convex's own local backend, pushes cloud/convex/ to it and keeps
//                                         pushing as the functions change. With no deployment chosen in cloud/ yet (no cloud/.env.local) it is started
//                                         with no account (CONVEX_AGENT_MODE=anonymous); a deployment you chose there yourself is used as it is.
//                                         Needs `cd cloud && npm ci` once; without it this says so and the page has no backend.
// A backend that already answers at the address the tool last wrote (one left running, or started by hand) is used and nothing is started.
// The server does not wait for it: the address is handed out once the backend answers (a first run downloads Convex's backend, which takes a while), and a
// page loaded before that has no backend for that load. The data stays under cloud/.convex/ (ignored by git).
//
// READING IT BACK: `bun run tools/cloud/pull.ts --local --out DIR` and `bun run tools/cloud/errors.ts --local` take the address and the key from
// cloud/.convex/local/default/config.json, where `convex dev` keeps them (`localKey`). The key is never printed.
import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const CLOUD_DIR = resolve(import.meta.dir, '../../cloud');
const LOOPBACK = /^http:\/\/(127\.0\.0\.1|localhost|\[::1\])(:\d+)?\/?$/;
const say = (s: string): void => console.log(`local backend: ${s}`);

/** what `convex dev` wrote for a deployment on this machine: its address and the key internal reads must show. null = none was ever started here */
export function localKey(dir = CLOUD_DIR): { url: string; key: string } | null {
  try {
    const c = JSON.parse(readFileSync(join(dir, '.convex', 'local', 'default', 'config.json'), 'utf8')) as { ports?: { cloud?: number }; adminKey?: string };
    return c.ports?.cloud && c.adminKey ? { url: `http://127.0.0.1:${c.ports.cloud}`, key: c.adminKey } : null;
  } catch { return null; }
}
/** the address `convex dev` wrote into cloud/.env.local ('' = none yet) */
const envUrl = (dir: string): string => { try { return /^CONVEX_URL=(\S+)/m.exec(readFileSync(join(dir, '.env.local'), 'utf8'))?.[1]?.replace(/^["']|["']$/g, '') ?? ''; } catch { return ''; } };
const answers = async (url: string): Promise<boolean> => { try { return (await fetch(`${url}/version`, { signal: AbortSignal.timeout(1500) })).ok; } catch { return false; } };

export interface LocalCloud { /** '' until the backend answers */ url(): string; stop(): void }

/** `env`: the process's (a check hands its own). `dir`: the backend's folder. `spawn`: the check's stand-in for the tool. null = no local backend was asked for */
export function localCloud(env: Record<string, string | undefined> = process.env, dir = CLOUD_DIR,
  spawn: (cmd: string[], o: { cwd: string; env: Record<string, string | undefined> }) => { exited: Promise<number>; kill(): void } = (cmd, o) => Bun.spawn(cmd, { cwd: o.cwd, env: o.env as Record<string, string>, stdin: 'ignore', stdout: 'ignore', stderr: 'ignore' })): LocalCloud | null {
  const given = (env.TPS_CLOUD_URL ?? '').trim().replace(/\/+$/, '');
  if (given) {
    if (!LOOPBACK.test(given) && !/^https:\/\/[\w.-]+(:\d+)?$/.test(given)) { say(`TPS_CLOUD_URL is not an address a page can use (${given}). The page has no backend.`); return null; }
    say(`using ${given} (TPS_CLOUD_URL). Nothing was started.`); return { url: () => given, stop: () => { /* not ours */ } };
  }
  if (env.TPS_LOCAL_CLOUD !== '1') return null;
  const tool = join(dir, 'node_modules', '.bin', 'convex');
  if (!existsSync(tool)) { say('not set up. Run `cd cloud && npm ci` once and start the dev server again. Until then the page has no backend.'); return null; }
  let url = '', gone = false, child: { exited: Promise<number>; kill(): void } | null = null;
  const where = (): string => (envUrl(dir) || localKey(dir)?.url || '').replace(/\/+$/, '');
  const start = (): void => {
    const chosen = existsSync(join(dir, '.env.local'));
    // (never with a deploy key from this shell: a dev server must not be able to push to the deployment a key names — only cloud/.env.local chooses one)
    const childEnv: Record<string, string | undefined> = { ...env, ...(chosen ? {} : { CONVEX_AGENT_MODE: 'anonymous' }) };
    for (const k of ['CONVEX_DEPLOY_KEY', 'CONVEX_DEPLOYMENT', 'CONVEX_URL', 'CONVEX_SELF_HOSTED_URL', 'CONVEX_SELF_HOSTED_ADMIN_KEY']) delete childEnv[k];
    const c = child = spawn([tool, 'dev', '--tail-logs', 'disable'], { cwd: dir, env: childEnv });
    say(chosen ? 'starting the deployment chosen in cloud/.env.local…' : 'starting a deployment on this machine (no account needed)…');
    void c.exited.then(code => { gone = true; if (!url) say(`convex dev stopped (exit ${code}) before a backend answered. Run \`cd cloud && npx convex dev\` to see why. The page has no backend.`); else say(`convex dev stopped (exit ${code}). Pages loaded from now on have no backend.`); url = ''; });
  };
  void (async () => {
    // (one left running — a dev server whose terminal was closed, a `convex dev` in another terminal — is used as it is: a second one would fight it for the port)
    const there = where();
    if (there && LOOPBACK.test(there) && await answers(there)) { url = there; say(`already up at ${there}. Nothing was started.`); return; }
    start();
    for (let i = 0; i < 600 && !gone; i++) {   // (ten minutes: a first run downloads the backend)
      const u = where();
      if (u && (LOOPBACK.test(u) ? await answers(u) : /^https:\/\//.test(u))) { url = u; say(`up at ${u}. Pages loaded from now on send to it. Read it back with \`bun run tools/cloud/pull.ts --local --out cloud-pull\`.`); return; }
      await Bun.sleep(1000);
    }
    if (!gone) say('no backend answered in ten minutes. The page has no backend.');
  })();
  return { url: () => url, stop: () => { if (child && !gone) try { child.kill(); } catch { /* */ } } };
}
