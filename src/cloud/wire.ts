// THE WIRE TO THE BACKEND (cloud/convex/, a Convex deployment): its functions called over plain HTTP — POST <address>/api/query or /api/mutation with
// `{ path: 'module:function', format: 'convex_encoded_json', args: [ {…} ] }`, the answer `{ status: 'success', value }` or `{ status: 'error', errorMessage }`
// (the shape the service's own client speaks; bytes ride as `{ $bytes: base64 }`). No package is loaded for it: the page has no dependencies, and this is all
// of the service it uses. Every call resolves — to the value, or to a `WireError` it REJECTS with, never anything else — inside `timeoutMs`.
// No game code and no DOM here: the page (src/cloud/cloud.ts) and the reading tool under bun (tools/cloud/pull.ts) use the same file.

export class WireError extends Error {
  /** `refused` = the backend ran the function and it threw (bad arguments, a bound: sending the same again will fail the same); else the call did not get there */
  constructor(message: string, readonly refused: boolean) { super(message); }
}
export interface Wire {
  readonly url: string;
  query<T = unknown>(path: string, args: Record<string, unknown>): Promise<T>;
  /** `keepalive`: the page is going away — the browser is asked to finish the request without it (a small body only: browsers cap those at 64 KB) */
  mutation<T = unknown>(path: string, args: Record<string, unknown>, opts?: { keepalive?: boolean }): Promise<T>;
  readonly stats: { calls: number; failed: number; refused: number; bytesOut: number; lastError: string };
}
export interface WireOpts {
  /** a deploy key: internal functions answer to it (the reading tool's; a page never has one) */
  key?: string;
  fetch?: typeof fetch;
  timeoutMs?: number;
}

const B64 = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
/** bytes as base64 (by hand: `btoa` wants a string of the bytes, and one call's worth of them is past what `String.fromCharCode(...)` takes) */
export function toBase64(u: Uint8Array): string {
  let out = ''; const n = u.length; let i = 0;
  for (; i + 2 < n; i += 3) { const x = (u[i] << 16) | (u[i + 1] << 8) | u[i + 2]; out += B64[x >> 18] + B64[(x >> 12) & 63] + B64[(x >> 6) & 63] + B64[x & 63]; }
  if (i + 1 === n) { const x = u[i] << 16; out += B64[x >> 18] + B64[(x >> 12) & 63] + '=='; }
  else if (i + 2 === n) { const x = (u[i] << 16) | (u[i + 1] << 8); out += B64[x >> 18] + B64[(x >> 12) & 63] + B64[(x >> 6) & 63] + '='; }
  return out;
}
export function fromBase64(s: string): Uint8Array {
  const bin = atob(s), u = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) u[i] = bin.charCodeAt(i);
  return u;
}
/** a value as the service's JSON: bytes become `{ $bytes }`, a number that is not finite becomes null (it has no plain JSON), `undefined` fields are left out */
export function encode(v: unknown): unknown {
  if (v instanceof Uint8Array) return { $bytes: toBase64(v) };
  if (typeof v === 'number') return Number.isFinite(v) ? v : null;
  if (Array.isArray(v)) return v.map(encode);
  if (v && typeof v === 'object') { const o: Record<string, unknown> = {}; for (const [k, x] of Object.entries(v)) if (x !== undefined) o[k] = encode(x); return o; }
  return v;
}
export function decode(v: unknown): unknown {
  if (Array.isArray(v)) return v.map(decode);
  if (v && typeof v === 'object') {
    const o = v as Record<string, unknown>;
    if (typeof o.$bytes === 'string' && Object.keys(o).length === 1) return fromBase64(o.$bytes);
    const out: Record<string, unknown> = {}; for (const [k, x] of Object.entries(o)) out[k] = decode(x); return out;
  }
  return v;
}

export function wire(url: string, opts: WireOpts = {}): Wire {
  const base = url.replace(/\/+$/, ''), timeoutMs = opts.timeoutMs ?? 15000;
  const stats = { calls: 0, failed: 0, refused: 0, bytesOut: 0, lastError: '' };
  const fail = (why: string, refused: boolean): never => { stats.failed++; if (refused) stats.refused++; stats.lastError = why; throw new WireError(why, refused); };
  const call = async <T>(kind: 'query' | 'mutation', path: string, args: Record<string, unknown>, keepalive = false): Promise<T> => {
    const f = opts.fetch ?? fetch, body = JSON.stringify({ path, format: 'convex_encoded_json', args: [encode(args)] });
    stats.calls++; stats.bytesOut += body.length;
    // (the wait grows with the body: a call's worth of tape on a slow line — 25 KB a second is allowed for — must not be cut off and sent again for ever)
    const stop = typeof AbortController !== 'undefined' ? new AbortController() : null, t = stop ? setTimeout(() => stop.abort(), timeoutMs + Math.round(body.length / 25)) : null;
    let res: Response, json: { status?: string; value?: unknown; errorMessage?: string };
    try {
      res = await f(`${base}/api/${kind}`, { method: 'POST', headers: { 'Content-Type': 'application/json', ...(opts.key ? { Authorization: `Convex ${opts.key}` } : {}) }, body, ...(keepalive ? { keepalive: true } : {}), ...(stop ? { signal: stop.signal } : {}) });
      json = await res.json();
    } catch (e) { return fail(`${path}: ${String((e as Error | undefined)?.message ?? e).slice(0, 160)}`, false); }
    finally { if (t) clearTimeout(t); }
    if (json?.status === 'success') return decode(json.value) as T;
    // (a function the backend does not HAVE is not a refusal of these arguments: the backend is older or newer than the page, and the call is worth asking again)
    const why = String(json?.errorMessage ?? `HTTP ${res.status}`).slice(0, 200);
    return fail(`${path}: ${why}`, json?.status === 'error' && !/Could not find (public )?function/i.test(why));
  };
  return { url: base, stats, query: (path, args) => call('query', path, args), mutation: (path, args, o) => call('mutation', path, args, !!o?.keepalive) };
}
