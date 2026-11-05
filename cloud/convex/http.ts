// THE DOORS THAT ARE PLAIN HTTP (a deployment answers these at its own site address; everything a page calls is a function, not a door here).
//   POST /mp/heartbeat   a game server says how it is: `{ game, secret, state: 'waiting'|'playing'|'ended', tick, seats: [connected…], outcome? }`
//                        200 `{ ok: true }` · 410 = no live game has this id (the server stops) · 403 = not this game's secret · 400 = not a heartbeat
import { httpRouter } from "convex/server";
import { ConvexError } from "convex/values";
import { internal } from "./_generated/api";
import { httpAction } from "./_generated/server";

const answer = (status: number, body: Record<string, unknown>): Response =>
  new Response(JSON.stringify(body), { status, headers: { "Content-Type": "application/json" } });

const http = httpRouter();
http.route({
  path: "/mp/heartbeat",
  method: "POST",
  handler: httpAction(async (ctx, req) => {
    let b: Record<string, unknown>;
    try {
      b = (await req.json()) as Record<string, unknown>;
      if (!b || typeof b !== "object") throw 0;
    } catch {
      return answer(400, { ok: false, why: "not JSON" });
    }
    const { game, secret, state, tick, seats, outcome } = b;
    if (typeof game !== "string" || typeof secret !== "string" || typeof state !== "string" || typeof tick !== "number" || !Array.isArray(seats) || seats.some(s => typeof s !== "boolean"))
      return answer(400, { ok: false, why: "not a heartbeat" });
    try {
      const r = await ctx.runMutation(internal.games.heard, { game, secret, state, tick, seats: seats as boolean[], ...(outcome === undefined ? {} : { outcome }) });
      return r === "ok" ? answer(200, { ok: true }) : r === "gone" ? answer(410, { ok: false, why: "no live game has this id" }) : answer(403, { ok: false, why: "not this game's secret" });
    } catch (e) {
      return answer(400, { ok: false, why: e instanceof ConvexError ? String(e.data) : "not a heartbeat" });
    }
  }),
});
export default http;
