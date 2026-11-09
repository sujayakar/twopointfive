// WHERE A PAGE ON THIS MACHINE LEAVES A RECORDING: POST /perf?name=<name> (the F9 recorder's dump, src/ui/perfrec.ts) is written to captures/perf/<name>.json.
//   bun run tools/shot-server.ts      PORT= (5174, the page's `window.__shotPort`) · HOST= (loopback)
// It answers loopback pages alone: another page open in the same browser gets a 403 and no grant.
import { mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
const DIR = resolve(import.meta.dir, '../captures/perf'), port = Number(process.env.PORT ?? 5174), hostname = process.env.HOST ?? '127.0.0.1';
const LOOPBACK = /^https?:\/\/(localhost|127\.0\.0\.1|\[::1\])(:\d+)?$/i;
Bun.serve({
  port, hostname,
  async fetch(req) {
    const origin = req.headers.get('origin');
    if (origin && !LOOPBACK.test(origin)) return new Response(`origin ${origin} refused: loopback pages only`, { status: 403 });
    const headers = { 'Access-Control-Allow-Origin': origin ?? '*', 'Access-Control-Allow-Methods': 'POST, OPTIONS', 'Access-Control-Allow-Headers': '*', 'Vary': 'Origin' };
    const url = new URL(req.url);
    if (req.method !== 'POST' || url.pathname !== '/perf') return new Response(req.method === 'OPTIONS' ? null : 'POST /perf?name=', { headers });
    const name = (url.searchParams.get('name') ?? `perf-${Date.now()}`).replace(/[^A-Za-z0-9_.-]/g, '_').slice(0, 96), body = await req.text();
    mkdirSync(DIR, { recursive: true }); await Bun.write(`${DIR}/${name}.json`, body);
    console.log(`captures/perf/${name}.json (${(body.length / 1024).toFixed(0)} KB)`);
    return new Response(`captures/perf/${name}.json ${body.length}`, { headers });
  },
});
console.log(`recordings on http://${hostname}:${port}`);
