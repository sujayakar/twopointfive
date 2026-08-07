// Tiny capture sink for automated screenshots: POST /shot?name=foo (image body) → docs/shots/foo.jpg
import { mkdirSync } from 'node:fs';
const dir = new URL('../docs/shots/', import.meta.url).pathname; mkdirSync(dir, { recursive: true });
Bun.serve({
  port: 5174,
  async fetch(req) {
    const cors = { 'Access-Control-Allow-Origin': '*', 'Access-Control-Allow-Methods': 'POST, OPTIONS', 'Access-Control-Allow-Headers': '*' };
    if (req.method === 'OPTIONS') return new Response(null, { headers: cors });
    const url = new URL(req.url);
    if (req.method === 'POST' && url.pathname === '/shot') {
      const name = (url.searchParams.get('name') ?? 'shot').replace(/[^a-zA-Z0-9_.-]/g, '_');
      const bytes = new Uint8Array(await req.arrayBuffer());
      await Bun.write(`${dir}${name}.jpg`, bytes);
      console.log(`saved ${name}.jpg (${(bytes.length / 1024).toFixed(0)} KB)`);
      return new Response('ok', { headers: cors });
    }
    return new Response('capture sink: POST /shot?name=', { headers: cors });
  },
});
console.log(`shot server on http://localhost:5174 → ${dir}`);
