"use node";
// THE HOSTED RUNNER: game servers as Modal sandboxes, one a game — the runner's calls (runners.ts says them) answered here, by Modal's own client, with
// the deployment's Modal token (`MODAL_TOKEN_ID` · `MODAL_TOKEN_SECRET` in its environment). A runner registered at `modal:<app>` is this one
// (`npx convex run runners:hosted '{"build":"<commit>"}'`, which the hosted build's script runs).
//   start   a sandbox of the build's image runs `bun run tools/mp/server.ts`; its port is behind a TLS tunnel, and that is the address (wss://),
//           handed back once the server answers /health there (a sandbox whose server does not is terminated, and the start has failed)
//   stop    the sandbox is terminated (one that is gone is not an error)
//   list    every sandbox of the app that runs and is this owner's, by its tags (whose it is, which game)
//   ready   the build's image is built now, so that the first game of a build does not wait for it
// THE SAME BUILD: a build's image is the game's repository checked out at that commit, and nothing else of ours. Modal keeps an image it has built, so
// a build is fetched once.
// BOUNDS that hold with this deployment away: Modal kills a sandbox `maxS` + `graceS` seconds after its start whatever runs in it, and the server ends
// itself on its own idle and lifetime clocks.
// NEVER RUN where it was written (no token there): the control plane is held to these calls by tools/qa/mp-control-check.ts against the local runner;
// what is unproven is everything on this page that is Modal's.
import { v } from "convex/values";
import { ModalClient, NotFoundError } from "modal";
import { internalAction } from "./_generated/server";

const MODAL = {
  /** the image a build's is made from, and where the game's repository is (`MP_REPO` in the deployment's environment names another) */
  base: "oven/bun:1",
  repo: "https://github.com/sujayakar/twopointfive",
  /** the server's port inside the sandbox, and what a sandbox is given */
  port: 8080,
  cpu: 2,
  memoryMiB: 2048,
  /** a sandbox outlives its server's own lifetime by this much at most, seconds */
  graceS: 120,
  tunnelMs: 60_000,
  /** how long a start waits for the server to answer /health through its tunnel, ms */
  healthMs: 45_000,
};
type Start = { game: string; build: string; setup: string; control: string; owner: string; secret: string; idleS: number; maxS: number };

const client = (): ModalClient => {
  const tokenId = process.env.MODAL_TOKEN_ID, tokenSecret = process.env.MODAL_TOKEN_SECRET;
  if (!tokenId || !tokenSecret) throw new Error("this deployment has no Modal token");
  return new ModalClient({ tokenId, tokenSecret });
};
const image = (m: ModalClient, build: string) => {
  // (the build goes into a command line: a commit's hex and nothing else)
  if (!/^[0-9a-f]{7,40}$/.test(build)) throw new Error("build: not a commit");
  const repo = process.env.MP_REPO || MODAL.repo;
  return m.images.fromRegistry(MODAL.base).dockerfileCommands([
    "RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates && rm -rf /var/lib/apt/lists/*",
    `RUN git clone --filter=blob:none ${repo} /game && git -C /game checkout --detach ${build}`,
  ]);
};

export const call = internalAction({
  args: { app: v.string(), what: v.union(v.literal("start"), v.literal("stop"), v.literal("list"), v.literal("ready")), body: v.any() },
  handler: async (_ctx, { app: name, what, body }): Promise<unknown> => {
    const m = client();
    try {
      if (what === "stop") {
        try {
          await (await m.sandboxes.fromId(String((body as { id: string }).id))).terminate();
        } catch (e) {
          if (!(e instanceof NotFoundError)) throw e;
        }
        return { ok: true };
      }
      const app = await m.apps.fromName(name, { createIfMissing: true });
      if (what === "list") {
        const servers = [];
        for await (const sb of m.sandboxes.list({ appId: app.appId, tags: { owner: String((body as { owner: string }).owner) } })) {
          const t = await sb.getTags();
          servers.push({ id: sb.sandboxId, game: t.game ?? "", owner: t.owner ?? "" });
        }
        return { servers };
      }
      if (what === "ready") {
        await image(m, String((body as { build: string }).build)).build(app);
        return { ok: true };
      }
      const b = body as Start;
      const sb = await m.sandboxes.create(app, image(m, b.build), {
        command: ["bun", "run", "tools/mp/server.ts"],
        workdir: "/game",
        env: {
          // (every interface: the server listens on loopback where nothing says, and the tunnel does not come in by loopback)
          MP_HOST: "0.0.0.0", MP_PORT: String(MODAL.port), MP_GAME: b.game, MP_SETUP: b.setup, MP_CONTROL: b.control, MP_SECRET: b.secret,
          MP_IDLE_S: String(b.idleS), MP_MAX_S: String(b.maxS),
        },
        timeoutMs: Math.ceil(b.maxS + MODAL.graceS) * 1000,
        encryptedPorts: [MODAL.port],
        cpu: MODAL.cpu,
        memoryMiB: MODAL.memoryMiB,
        tags: { owner: b.owner, game: b.game },
      });
      // (a sandbox that cannot be reached, or whose server never answers, is not left running)
      try {
        const tunnel = (await sb.tunnels(MODAL.tunnelMs))[MODAL.port];
        if (!tunnel) throw new Error("the sandbox has no tunnel to its port");
        for (const until = Date.now() + MODAL.healthMs; ; await new Promise(r => setTimeout(r, 250))) {
          if (await fetch(`${tunnel.url}/health`).then(r => r.ok, () => false)) break;
          const exit = await sb.poll();
          if (exit !== null) throw new Error(`the server stopped before it answered (exit ${exit})`);
          if (Date.now() > until) throw new Error(`the server did not answer /health in ${MODAL.healthMs} ms`);
        }
        return { id: sb.sandboxId, address: tunnel.url.replace(/^https:/, "wss:") };
      } catch (e) {
        await sb.terminate().catch(() => {});
        throw e;
      }
    } finally {
      m.close();
    }
  },
});
