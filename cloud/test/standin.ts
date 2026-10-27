// A STAND-IN FOR A CONVEX DEPLOYMENT, for checks on a machine that cannot reach one: the functions in ../convex run in convex-test's in-memory backend,
// behind the two routes of a deployment's HTTP API a page and the pull tool use (POST /api/query · /api/mutation, `convex_encoded_json`, the answer's
// shape and its 560 for a function that threw — what the official client speaks), and the two of its file storage (POST /api/storage/upload?token=… with
// the bytes as the body answers `{ storageId }`; GET /api/storage/<id> answers the bytes). The test library hands out addresses on a host of its own
// making: they are rewritten to this server's, and the token is not checked (the service checks it; here any POST stores). An internal function wants `Authorization: Convex <key>`, as a deploy
// key is wanted there. It is the functions and the schema that are real here; the service is not.
//   bun run test/standin.ts [port]        (STANDIN_KEY=… the key internal calls must show; default 'standin-key')
import { convexTest } from "convex-test";
import { makeFunctionReference } from "convex/server";
import { convexToJson, jsonToConvex, ConvexError } from "convex/values";
import schema from "../convex/schema";

export const modules: Record<string, () => Promise<unknown>> = {
  "../convex/_generated/api.js": () => import("../convex/_generated/api.js"),
  "../convex/_generated/server.js": () => import("../convex/_generated/server.js"),
  "../convex/files.ts": () => import("../convex/files"),
  "../convex/limits.ts": () => import("../convex/limits"),
  "../convex/notes.ts": () => import("../convex/notes"),
  "../convex/prefs.ts": () => import("../convex/prefs"),
  "../convex/tapes.ts": () => import("../convex/tapes"),
  "../convex/telemetry.ts": () => import("../convex/telemetry"),
};
const CORS = { "Access-Control-Allow-Origin": "*", "Access-Control-Allow-Headers": "*", "Access-Control-Allow-Methods": "GET, POST, OPTIONS" };
const THEIRS = "https://some-deployment.convex.cloud";   // (the host the test library writes into a storage address)

export function standIn(port = 0, key = process.env.STANDIN_KEY ?? "standin-key") {
  const t = convexTest(schema, modules);
  const stats = { calls: 0, failed: 0, uploads: 0, uploadBytes: 0, byPath: {} as Record<string, number> };
  let self = "";
  const server = Bun.serve({
    port, hostname: "127.0.0.1",
    async fetch(req) {
      const url = new URL(req.url);
      if (req.method === "OPTIONS") return new Response(null, { status: 204, headers: CORS });
      if (url.pathname === "/__stats") return Response.json(stats, { headers: CORS });   // (the stand-in's own: how many calls it has answered, by function)
      if (url.pathname === "/api/storage/upload" && req.method === "POST") {
        const body = await req.blob(); stats.uploads++; stats.uploadBytes += body.size;
        const storageId = await t.run(async ctx => await ctx.storage.store(new Blob([await body.arrayBuffer()], { type: req.headers.get("content-type") ?? "application/octet-stream" })));
        return Response.json({ storageId }, { headers: CORS });
      }
      if (url.pathname.startsWith("/api/storage/") && req.method === "GET") {
        // (the test library's address for a stored file ends in its hash, not its id)
        const tail = decodeURIComponent(url.pathname.slice("/api/storage/".length));
        const blob = await t.run(async ctx => {
          const doc = (await ctx.db.system.query("_storage").collect()).find(d => d.sha256 === tail || d._id === tail);
          const blob = doc ? await ctx.storage.get(doc._id) : null;   // (a blob does not cross the library's boundary: its bytes do)
          return blob ? { bytes: await blob.arrayBuffer(), type: doc!.contentType ?? "application/octet-stream" } : null;
        }).catch(() => null);
        return blob ? new Response(blob.bytes, { headers: { ...CORS, "Content-Type": blob.type } }) : new Response("not found", { status: 404, headers: CORS });
      }
      const kind = url.pathname === "/api/query" ? "query" : url.pathname === "/api/mutation" ? "mutation" : null;
      if (!kind || req.method !== "POST") return new Response("not found", { status: 404, headers: CORS });
      let body: { path?: string; format?: string; args?: unknown[] };
      try { body = await req.json(); } catch { return new Response("bad json", { status: 400, headers: CORS }); }
      if (typeof body.path !== "string" || body.format !== "convex_encoded_json" || !Array.isArray(body.args)) return new Response("bad request", { status: 400, headers: CORS });
      stats.calls++; stats.byPath[body.path] = (stats.byPath[body.path] ?? 0) + 1;
      const admin = req.headers.get("authorization") === `Convex ${key}`;
      try {
        const [mod, name] = body.path.split(":");
        const fn = ((await modules[`../convex/${mod}.ts`]?.()) as Record<string, { isInternal?: boolean }> | undefined)?.[name];
        if (!fn || (fn.isInternal && !admin)) throw new Error(`Could not find public function for '${body.path}'`);
        const ref = makeFunctionReference<any>(body.path), args = jsonToConvex(body.args[0] as never) as Record<string, never>;
        const value = kind === "query" ? await t.query(ref, args) : await t.mutation(ref, args);
        const json = JSON.stringify({ status: "success", value: convexToJson(value === undefined ? null : value), logLines: [] }).split(THEIRS).join(self);
        return new Response(json, { headers: { ...CORS, "Content-Type": "application/json" } });
      } catch (e) {
        stats.failed++;
        const data = e instanceof ConvexError ? { errorData: convexToJson(e.data) } : {};
        return Response.json({ status: "error", errorMessage: String((e as Error)?.message ?? e), ...data, logLines: [] }, { status: 560, headers: CORS });
      }
    },
  });
  self = `http://127.0.0.1:${server.port}`;
  return { t, server, stats, url: self, key, stop: () => server.stop(true) };
}

if (import.meta.main) { const s = standIn(Number(process.argv[2] ?? 3210)); console.log(`stand-in Convex deployment: ${s.url}  (internal calls: Authorization: Convex ${s.key})`); }
