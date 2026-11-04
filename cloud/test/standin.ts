// A STAND-IN FOR A CONVEX DEPLOYMENT, for checks on a machine that cannot reach one: the functions in ../convex run in convex-test's in-memory backend,
// behind the two routes of a deployment's HTTP API a page and the pull tool use (POST /api/query · /api/mutation, `convex_encoded_json`, the answer's
// shape and its 560 for a function that threw — what the official client speaks). An internal function wants `Authorization: Convex <key>`, as a deploy
// key is wanted there. It is the functions and the schema that are real here; the service is not.
//   bun run test/standin.ts [port]        (STANDIN_KEY=… the key internal calls must show; default 'standin-key')
import { convexTest } from "convex-test";
import { makeFunctionReference } from "convex/server";
import { convexToJson, jsonToConvex, ConvexError } from "convex/values";
import schema from "../convex/schema";

export const modules: Record<string, () => Promise<unknown>> = {
  "../convex/_generated/api.js": () => import("../convex/_generated/api.js"),
  "../convex/_generated/server.js": () => import("../convex/_generated/server.js"),
  "../convex/limits.ts": () => import("../convex/limits"),
  "../convex/prefs.ts": () => import("../convex/prefs"),
  "../convex/tapes.ts": () => import("../convex/tapes"),
  "../convex/telemetry.ts": () => import("../convex/telemetry"),
};
const CORS = { "Access-Control-Allow-Origin": "*", "Access-Control-Allow-Headers": "*", "Access-Control-Allow-Methods": "POST, OPTIONS" };

export function standIn(port = 0, key = process.env.STANDIN_KEY ?? "standin-key") {
  const t = convexTest(schema, modules);
  const stats = { calls: 0, failed: 0, byPath: {} as Record<string, number> };
  const server = Bun.serve({
    port, hostname: "127.0.0.1",
    async fetch(req) {
      const url = new URL(req.url);
      if (req.method === "OPTIONS") return new Response(null, { status: 204, headers: CORS });
      if (url.pathname === "/__stats") return Response.json(stats, { headers: CORS });   // (the stand-in's own: how many calls it has answered, by function)
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
        return Response.json({ status: "success", value: convexToJson(value === undefined ? null : value), logLines: [] }, { headers: CORS });
      } catch (e) {
        stats.failed++;
        const data = e instanceof ConvexError ? { errorData: convexToJson(e.data) } : {};
        return Response.json({ status: "error", errorMessage: String((e as Error)?.message ?? e), ...data, logLines: [] }, { status: 560, headers: CORS });
      }
    },
  });
  return { t, server, stats, url: `http://127.0.0.1:${server.port}`, key, stop: () => server.stop(true) };
}

if (import.meta.main) { const s = standIn(Number(process.argv[2] ?? 3210)); console.log(`stand-in Convex deployment: ${s.url}  (internal calls: Authorization: Convex ${s.key})`); }
