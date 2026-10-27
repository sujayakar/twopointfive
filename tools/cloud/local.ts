// THE LOCAL BUILD'S BACKEND: a Convex deployment on this machine, started beside the dev server (the maintainer, 10-27: "we can make the local build use a
// local convex deployment"). `bun run dev` sets TPS_LOCAL_CLOUD=1; tools/serve.ts then calls `localCloud()` and answers the page's `GET /qa/cloud` with
// what it returns (src/cloud/cloud.ts cloudFind). A plain `bun run tools/serve.ts` — every check's dev server — starts nothing and answers with nothing.
//
//   TPS_DEV_CLOUD=http://127.0.0.1:3210   use the backend that is already there (a stand-in, a `convex dev` in another terminal): nothing is started.
//                                         A hosted address works too (TPS_DEV_CLOUD=https://<name>.convex.cloud: local play goes to that backend); the
//                                         page then treats it as it treats a built-in one, notice first. NOT `TPS_CLOUD_URL`: that word is the hosted
//                                         build's address for tools/build.ts and the reading tools, and a shell that has it exported must not turn a
//                                         dev server's play into the real backend's data.
//   TPS_LOCAL_CLOUD=1                     start one: `convex dev` in cloud/, which runs Convex's own local backend, pushes cloud/convex/ to it and keeps
//                                         pushing as the functions change. With no deployment chosen in cloud/ yet (no cloud/.env.local) it is started
//                                         with no account (CONVEX_AGENT_MODE=anonymous); a deployment you chose there yourself is used as it is.
//                                         Needs `cd cloud && npm ci` once; without it this says so and the page has no backend.
// ONLY A BACKEND ON THIS MACHINE is one the page sends to with no notice (cloud.ts cloudLocal): nothing leaves the machine.
// The server does not wait for it: the address is handed out once the backend answers (a first run downloads Convex's backend, which takes a while), and a
// page loaded before that has no backend for that load. The data stays under cloud/.convex/ (ignored by git). If `convex dev` stops by itself — another
// one already runs for this folder, say — this says so and the page has no backend: stop the other, or name its address with TPS_DEV_CLOUD.
//
// READING IT BACK: `bun run tools/cloud/pull.ts --local --out DIR` and `bun run tools/cloud/errors.ts --local` take the address and the key from
// cloud/.convex/local/default/config.json, where `convex dev` keeps them (`localKey`). The key is never printed.
import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { LOOPBACK, HTTPS_ADDR, bare } from '../../src/cloud/addr.ts';

const CLOUD_DIR = resolve(import.meta.dir, '../../cloud');
const say = (s: string): void => console.log(`local backend: ${s}`);

/** what `convex dev` wrote for a deployment on this machine: its address and the key internal reads must show. null = none was ever started here */
export function localKey(dir = CLOUD_DIR): { url: string; key: string } | null {
  try {
    const c = JSON.parse(readFileSync(join(dir, '.convex', 'local', 'default', 'config.json'), 'utf8')) as { ports?: { cloud?: number }; adminKey?: string };
    return c.ports?.cloud && c.adminKey ? { url: `http://127.0.0.1:${c.ports.cloud}`, key: c.adminKey } : null;
  } catch { return null; }
}
/** the address `convex dev` wrote into cloud/.env.local ('' = none yet) */
/** THE ADDRESS A DEPLOY KEY IS FOR, where a read was given a key and no address (tools/cloud/pull.ts, errors.ts): a hosted deployment's key reads
 *  `<prod|dev>:<name>|<secret>` and that deployment answers at https://<name>.convex.cloud. '' for any other key (a preview key names a project, a local
 *  deployment's admin key names nothing): those need `--url` or `--local`. Only the name is read; the secret is not looked at */
export const keyUrl = (key: string | undefined): string => { const m = /^(?:prod|dev):([a-z0-9]+(?:-[a-z0-9]+)*)\|/.exec(key ?? ''); return m ? `https://${m[1]}.convex.cloud` : ''; };
const envUrl = (dir: string): string => { try { return /^CONVEX_URL=(\S+)/m.exec(readFileSync(join(dir, '.env.local'), 'utf8'))?.[1]?.replace(/^["']|["']$/g, '') ?? ''; } catch { return ''; } };
const answers = async (url: string): Promise<boolean> => { try { return (await fetch(`${url}/version`, { signal: AbortSignal.timeout(1500) })).ok; } catch { return false; } };

export interface LocalCloud { /** '' until the backend answers */ url(): string; stop(): void }

/** `env`: the process's (a check hands its own). `dir`: the backend's folder. `spawn`: the check's stand-in for the tool. null = no local backend was asked for */
export function localCloud(env: Record<string, string | undefined> = process.env, dir = CLOUD_DIR,
  spawn: (cmd: string[], o: { cwd: string; env: Record<string, string | undefined> }) => { exited: Promise<number>; kill(signal?: 'SIGINT'): void } = (cmd, o) => Bun.spawn(cmd, { cwd: o.cwd, env: o.env as Record<string, string>, stdin: 'ignore', stdout: 'ignore', stderr: 'ignore' })): LocalCloud | null {
  const given = bare(env.TPS_DEV_CLOUD ?? '');
  if (given) {
    if (!LOOPBACK.test(given) && !HTTPS_ADDR.test(given)) { say(`TPS_DEV_CLOUD is not an address a page can use (${given}). The page has no backend.`); return null; }
    say(LOOPBACK.test(given) ? `using ${given} (TPS_DEV_CLOUD). Nothing was started.` : `using ${given} (TPS_DEV_CLOUD), a hosted backend: play on this dev server is sent THERE, behind the page's notice. Nothing was started.`);
    return { url: () => given, stop: () => { /* not ours */ } };
  }
  if (env.TPS_LOCAL_CLOUD !== '1') return null;
  const tool = join(dir, 'node_modules', '.bin', 'convex');
  if (!existsSync(tool)) { say('not set up. Run `cd cloud && npm ci` once and start the dev server again. Until then the page has no backend.'); return null; }
  let url = '', gone = false;
  const chosen = existsSync(join(dir, '.env.local'));
  // (never with a deploy key from this shell: a dev server must not be able to push to the deployment a key names — only cloud/.env.local chooses one)
  const childEnv: Record<string, string | undefined> = { ...env, ...(chosen ? {} : { CONVEX_AGENT_MODE: 'anonymous' }) };
  for (const k of ['CONVEX_DEPLOY_KEY', 'CONVEX_DEPLOYMENT', 'CONVEX_URL', 'CONVEX_SELF_HOSTED_URL', 'CONVEX_SELF_HOSTED_ADMIN_KEY']) delete childEnv[k];
  const child = spawn([tool, 'dev', '--tail-logs', 'disable'], { cwd: dir, env: childEnv });
  say(chosen ? 'starting the deployment chosen in cloud/.env.local…' : 'starting a deployment on this machine (no account needed)…');
  void child.exited.then(code => { gone = true; if (!url) say(`convex dev stopped (exit ${code}) before a backend answered. Run \`cd cloud && npx convex dev\` to see why. The page has no backend.`); else say(`convex dev stopped (exit ${code}). Pages loaded from now on have no backend.`); url = ''; });
  void (async () => {
    // (only while OUR `convex dev` runs: a backend that answers at the address the tool last wrote but that this start did not bring up may be an old
    // one with old functions, or another project's, and is not taken)
    for (let i = 0; i < 600 && !gone; i++) {   // (ten minutes: a first run downloads the backend)
      const u = bare(envUrl(dir) || localKey(dir)?.url || '');
      if (u && (LOOPBACK.test(u) || HTTPS_ADDR.test(u)) && await answers(u) && !gone) {
        url = u;
        say(LOOPBACK.test(u) ? `up at ${u}. Pages loaded from now on send to it. Read it back with \`bun run tools/cloud/pull.ts --local --out cloud-pull\`.`
          : `up at ${u}, the hosted deployment chosen in cloud/.env.local: play on this dev server is sent THERE, behind the page's notice. Read it back with pull.ts --url and that deployment's key.`);
        return;
      }
      await Bun.sleep(1000);
    }
    if (!gone) say('no backend answered in ten minutes. The page has no backend.');
  })();
  // (SIGINT, not the default: `convex dev` stops the backend it started on an interrupt, and leaves it running on a plain kill)
  return { url: () => url, stop: () => { if (!gone) try { child.kill('SIGINT'); } catch { /* */ } } };
}
