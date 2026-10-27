// WHAT WENT WRONG ON PLAYERS' MACHINES, READ OUT OF THE BACKEND (cloud/convex/telemetry.ts — the `fatal` and `error` records of src/cloud/, joined to each
// visit's `visit` record for the machine it happened on):
//   bun run tools/cloud/errors.ts [--url https://<name>.convex.cloud] [--key <deploy key>] [--since 2026-10-26T00:00:00Z] [--metrics] [--json] [--limit N]
// The address and the key are read from the environment where the words are absent (TPS_CLOUD_URL or CONVEX_URL · CONVEX_DEPLOY_KEY), as tools/cloud/pull.ts
// reads them; the key is never written anywhere by this tool. Queries only.
//   (default)   the crashes and errors, one block a distinct message: how often, on how many visits and browsers, which builds, which GPUs, where the page
//               was (a fatal's stage), the first and last time, one stack. Pages that died come first, then by how many browsers met it.
//   --metrics   a line a browser besides: its GPU, visits, the shader pipelines' wait at boot, the frame's and the GPU's time in play, the preset it sat on
//   --json      the same as JSON on stdout
// EXIT 0 read · 1 the backend could not be read · 2 usage.
import { wire } from '../../src/cloud/wire.ts';

export interface Rec { sid: string; pid: string; at: number; kind: string; build?: string; scenario?: string; data: any }
export interface ErrGroup { kind: 'fatal' | 'error'; message: string; count: number; visits: number; browsers: number; builds: string[]; gpus: string[]; stages: string[]; scenarios: string[]; first: number; last: number; stack: string[]; sid: string; data?: unknown }
export interface MachineLine { pid: string; gpu: string; browser: string; visits: number; fatals: number; errors: number; pipelinesMs: number | null; frameP50: number | null; frameP99: number | null; gpuMs: number | null; preset: string; playedMin: number }

/** the machine a visit ran on, in a few words: the adapter where the page got one, else the WebGL card, else that it had neither */
export const gpuOf = (visit: Rec | undefined): string => { const m = visit?.data?.machine ?? {}; return String(m.webgpu || (m.webgl ? `no WebGPU adapter · ${m.webgl}` : '') || (visit ? 'no adapter' : 'unknown machine')).slice(0, 120); };
const browserOf = (visit: Rec | undefined): string => { const m = visit?.data?.machine ?? {}; return [m.browser, m.os].filter(Boolean).join(' · ') || String(m.userAgent ?? '').slice(0, 80) || 'unknown browser'; };
/** two messages that differ only in a number, an address or a place in a file are one */
export const messageKey = (m: string): string => m.replace(/https?:\/\/\S+/g, '<url>').replace(/0x[0-9a-f]+/gi, '<hex>').replace(/:\d+:\d+/g, '').replace(/\d{3,}/g, '<n>').trim().slice(0, 200);
const uniq = (a: (string | undefined)[]): string[] => [...new Set(a.filter((x): x is string => !!x))];
const median = (a: number[]): number | null => { const s = a.filter(Number.isFinite).sort((x, y) => x - y); return s.length ? s[(s.length - 1) >> 1] : null; };

/** the crashes and errors as groups, pages that died first */
export function errorGroups(rows: Rec[]): ErrGroup[] {
  const visits = new Map<string, Rec>(); for (const r of rows) if (r.kind === 'visit') visits.set(r.sid, r);
  const by = new Map<string, { g: ErrGroup; sids: Set<string>; pids: Set<string> }>();
  for (const r of rows) {
    if (r.kind !== 'fatal' && r.kind !== 'error') continue;
    const message = String(r.data?.message ?? ''), key = `${r.kind}|${messageKey(message)}`, stack = Array.isArray(r.data?.stack) ? r.data.stack.map(String) : String(r.data?.stack ?? '').split('\n').filter(Boolean);
    let e = by.get(key);
    if (!e) by.set(key, e = { g: { kind: r.kind, message, count: 0, visits: 0, browsers: 0, builds: [], gpus: [], stages: [], scenarios: [], first: r.at, last: r.at, stack, sid: r.sid, ...(r.data?.data !== undefined ? { data: r.data.data } : {}) }, sids: new Set(), pids: new Set() });
    const g = e.g; g.count++; e.sids.add(r.sid); e.pids.add(r.pid); g.first = Math.min(g.first, r.at); g.last = Math.max(g.last, r.at);
    g.builds = uniq([...g.builds, r.build]); g.gpus = uniq([...g.gpus, gpuOf(visits.get(r.sid))]); g.stages = uniq([...g.stages, r.data?.stage]); g.scenarios = uniq([...g.scenarios, r.scenario]);
  }
  return [...by.values()].map(e => ({ ...e.g, visits: e.sids.size, browsers: e.pids.size })).sort((a, b) => (a.kind === b.kind ? 0 : a.kind === 'fatal' ? -1 : 1) || b.browsers - a.browsers || b.count - a.count || b.last - a.last);
}

/** a line a browser: its machine and how the game ran on it */
export function machineLines(rows: Rec[]): MachineLine[] {
  const by = new Map<string, Rec[]>(); for (const r of rows) { let a = by.get(r.pid); if (!a) by.set(r.pid, a = []); a.push(r); }
  return [...by.entries()].map(([pid, rs]) => {
    const of = (k: string) => rs.filter(r => r.kind === k), visit = of('visit').sort((a, b) => b.at - a.at)[0], perf = of('perf');
    const presets = new Map<string, number>(); for (const p of perf) presets.set(String(p.data?.preset ?? ''), (presets.get(String(p.data?.preset ?? '')) ?? 0) + (Number(p.data?.secs) || 0));
    return { pid, gpu: gpuOf(visit), browser: browserOf(visit), visits: of('visit').length, fatals: of('fatal').length, errors: of('error').length,
      pipelinesMs: median(of('boot').map(b => Number(b.data?.pipelinesMs))), frameP50: median(perf.map(p => Number(p.data?.p50))), frameP99: median(perf.map(p => Number(p.data?.p99))), gpuMs: median(perf.map(p => Number(p.data?.gpu?.mean))),
      preset: [...presets.entries()].sort((a, b) => b[1] - a[1])[0]?.[0] ?? '', playedMin: perf.reduce((a, p) => a + (Number(p.data?.secs) || 0), 0) / 60 };
  }).sort((a, b) => b.fatals - a.fatals || b.visits - a.visits);
}

const when = (ms: number): string => new Date(ms).toISOString().slice(0, 16).replace('T', ' ');
export function errorText(groups: ErrGroup[]): string {
  if (!groups.length) return 'no crashes and no errors in the records read';
  return groups.map(g => [
    `${g.kind === 'fatal' ? 'THE PAGE DIED' : 'error'}${g.stages.length ? ` (${g.stages.join(', ')})` : ''}: ${g.message}`,
    `  ${g.count} time${g.count === 1 ? '' : 's'} · ${g.visits} visit${g.visits === 1 ? '' : 's'} · ${g.browsers} browser${g.browsers === 1 ? '' : 's'} · ${when(g.first)}${g.last !== g.first ? ` to ${when(g.last)}` : ''} UTC`,
    `  builds: ${g.builds.join(', ') || 'not said'}`, `  GPUs: ${g.gpus.join(' | ')}`,
    ...(g.scenarios.length ? [`  in: ${g.scenarios.join(', ')}`] : []), ...g.stack.slice(0, 8).map(l => `    ${l}`),
    ...(g.data !== undefined ? [`  reporter: ${JSON.stringify(g.data).slice(0, 1200)}`] : []), `  a visit with it: ${g.sid}`,
  ].join('\n')).join('\n\n');
}
const n1 = (x: number | null, unit: string): string => x === null ? '-' : `${x >= 100 ? Math.round(x) : x.toFixed(1)} ${unit}`;
export function machineText(lines: MachineLine[]): string {
  return lines.map(m => `${m.pid.slice(0, 8)}  ${m.gpu}  (${m.browser})\n  ${m.visits} visit${m.visits === 1 ? '' : 's'} · ${m.fatals} died · ${m.errors} errors · shaders ${n1(m.pipelinesMs, 'ms')} · frame ${n1(m.frameP50, 'ms')} median, ${n1(m.frameP99, 'ms')} at the 99th · GPU ${n1(m.gpuMs, 'ms')} · ${m.preset || 'no play'} · ${m.playedMin.toFixed(1)} min played`).join('\n');
}

if (import.meta.main) {
  const A = process.argv.slice(2), word = (k: string): string | undefined => { const i = A.indexOf(k); return i >= 0 ? A[i + 1] : undefined; };
  const url = word('--url') ?? process.env.TPS_CLOUD_URL ?? process.env.CONVEX_URL, key = word('--key') ?? process.env.CONVEX_DEPLOY_KEY;
  if (!url || !key || A.includes('--help')) { console.error('usage: bun run tools/cloud/errors.ts [--url <address>] [--key <deploy key>] [--since <ISO>] [--metrics] [--json] [--limit N]\n(the address and the key may be in TPS_CLOUD_URL / CONVEX_URL and CONVEX_DEPLOY_KEY instead)'); process.exit(2); }
  const since = word('--since'), limit = Math.min(Number(word('--limit')) || 2000, 2000), metrics = A.includes('--metrics'), w = wire(url, { key, timeoutMs: 60000 });
  try {
    const rows: Rec[] = [];
    for (const kind of ['fatal', 'error', 'visit', ...(metrics ? ['boot', 'perf'] : [])]) {
      const got: Rec[] = await w.query('telemetry:list', { kind, ...(since ? { since: Date.parse(since) } : {}), limit });
      if (got.length >= limit && !A.includes('--json')) console.error(`(${kind}: the newest ${limit} records were read; there may be older ones — narrow with --since)`);
      rows.push(...got);
    }
    const groups = errorGroups(rows), lines = metrics ? machineLines(rows) : [];
    if (A.includes('--json')) console.log(JSON.stringify({ groups, ...(metrics ? { machines: lines } : {}) }, null, 1));
    else { console.log(errorText(groups)); if (metrics) console.log(`\n=== the machines ===\n${machineText(lines)}`); }
  } catch (e) { console.error(`errors — the backend could not be read: ${String((e as Error)?.message ?? e)}`); process.exit(1); }
}
