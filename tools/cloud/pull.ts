// WHAT THE BACKEND HOLDS, READ BACK OUT (cloud/convex/ — a hosted build's kept tapes, records and browsers):
//   bun run tools/cloud/pull.ts [--url https://<name>.convex.cloud] [--key <deploy key>] [--since 2026-10-26T00:00:00Z] [--pid <id>] [--out DIR] [--limit N] [--quiet]
// The address and the key are read from the environment where the words are absent (TPS_CLOUD_URL or CONVEX_URL · CONVEX_DEPLOY_KEY): the key is the
// deployment's own deploy key, which the internal functions answer to — it is never written anywhere by this tool, and never belongs in the repo.
// Into --out (default ./cloud-pull):
//   twopointsix-cloud-<stamp>-<n>-visits.ndjson.gz   every visit's tape, in the browser's own hand-over format (src/replay/kept.ts: gzip members end to end) —
//                                                    `bun run tools/qa/replay-dump.ts <that file>` unpacks it, the /replay page opens it
//   telemetry.ndjson                                 the records, one a line, oldest first
//   players.json                                     the browsers seen
// A tape is DATA: nothing in one is run. EXIT 0 read · 1 the backend could not be read · 2 usage.
import { mkdirSync, writeFileSync } from 'node:fs';
import { gzipSync } from 'node:zlib';
import { join } from 'node:path';
import { wire, type Wire } from '../../src/cloud/wire.ts';
import { KEPT_VERSION } from '../../src/replay/kept.ts';

export interface VisitRow { sid: string; pid: string; startedAt: string; lastAt: string; scenario: string; url: string; commit?: string; seconds: number; pieces: number; bytes: number; have: number }
interface Chunk { seq: number; part: number; parts: number; at: string; body: Uint8Array }
const isGz = (u: Uint8Array): boolean => u[0] === 0x1f && u[1] === 0x8b;
const join1 = (parts: Uint8Array[]): Uint8Array => { const u = new Uint8Array(parts.reduce((a, p) => a + p.length, 0)); let o = 0; for (const p of parts) { u.set(p, o); o += p.length; } return u; };

/** one visit's tape as gzip members: its head, then its pieces in order. `missing` = pieces whose parts are not all there (left out: half a member does not unpack) */
export async function pullVisit(w: Wire, sid: string): Promise<{ members: Uint8Array[]; pieces: number; missing: number[] }> {
  const got = new Map<number, Chunk[]>();
  for (let from: { seq: number; part: number } | null = { seq: -1, part: 0 }; from !== null;) {
    const r: { chunks: Chunk[]; next: { seq: number; part: number } | null } = await w.query('tapes:chunks', { sid, ...from });
    for (const c of r.chunks) { let a = got.get(c.seq); if (!a) got.set(c.seq, a = []); a.push(c); }
    from = r.next;
  }
  const members: Uint8Array[] = [], missing: number[] = []; let pieces = 0;
  for (const seq of [...got.keys()].sort((a, b) => a - b)) {
    const parts = got.get(seq)!.sort((a, b) => a.part - b.part);
    if (parts.length !== parts[0].parts || parts.some((p, i) => p.part !== i || p.at !== parts[0].at)) { missing.push(seq); continue; }   // (…or its parts are not all of one sending)
    const u = join1(parts.map(p => p.body));
    members.push(isGz(u) ? u : new Uint8Array(gzipSync(u)));   // (a browser that could not pack sent the plain line: packed here, so the file is members throughout)
    if (seq >= 0) pieces++;
  }
  return { members, pieces, missing };
}

/** every visit the backend lists, as one hand-over file's bytes (null: none) */
export async function pullDump(w: Wire, opts: { since?: string; pid?: string; limit?: number; now?: string; say?: (s: string) => void } = {}): Promise<{ bytes: Uint8Array; name: string; visits: VisitRow[]; pieces: number } | null> {
  const rows: VisitRow[] = await w.query('tapes:list', { ...(opts.since ? { since: opts.since } : {}), ...(opts.pid ? { pid: opts.pid } : {}), ...(opts.limit ? { limit: opts.limit } : {}) });
  if (!rows.length) return null;
  const now = opts.now ?? new Date().toISOString(), body: Uint8Array[] = [], carried: { row: VisitRow; pieces: number }[] = []; let pieces = 0;
  for (const row of rows.sort((a, b) => a.startedAt < b.startedAt ? -1 : 1)) {
    // (a visit that does not read — somebody's oversized or broken tape — is said and left out: it does not cost the others)
    let v: Awaited<ReturnType<typeof pullVisit>>;
    try { v = await pullVisit(w, row.sid); } catch (e) { opts.say?.(`${row.sid}  NOT READ: ${String((e as Error)?.message ?? e).slice(0, 160)}`); continue; }
    opts.say?.(`${row.sid}  ${row.scenario || '(no scenario)'}  ${Math.round(row.seconds)} s  ${v.pieces} of ${row.pieces} pieces${v.missing.length ? `  (incomplete: ${v.missing.join(', ')})` : ''}`);
    if (!v.pieces) continue;
    for (const m of v.members) body.push(m); pieces += v.pieces; carried.push({ row, pieces: v.pieces });
  }
  if (!carried.length) return null;
  const top = gzipSync(JSON.stringify({ k: 'dump', v: KEPT_VERSION, at: now, shelf: 0, visits: carried.map(c => ({ sid: c.row.sid, pieces: c.pieces, bytes: c.row.bytes, startedAt: c.row.startedAt, lastAt: c.row.lastAt })) }) + '\n');
  return { bytes: join1([new Uint8Array(top), ...body]), name: `twopointsix-cloud-${now.replace(/[:.]/g, '-').replace('Z', '')}-${carried.length}-visit${carried.length === 1 ? '' : 's'}.ndjson.gz`, visits: carried.map(c => c.row), pieces };
}

if (import.meta.main) {
  const A = process.argv.slice(2), word = (k: string): string | undefined => { const i = A.indexOf(k); return i >= 0 ? A[i + 1] : undefined; };
  const url = word('--url') ?? process.env.TPS_CLOUD_URL ?? process.env.CONVEX_URL, key = word('--key') ?? process.env.CONVEX_DEPLOY_KEY;
  if (!url || !key || A.includes('--help')) { console.error('usage: bun run tools/cloud/pull.ts [--url <address>] [--key <deploy key>] [--since <ISO>] [--pid <id>] [--out DIR] [--limit N] [--quiet]\n(the address and the key may be in TPS_CLOUD_URL / CONVEX_URL and CONVEX_DEPLOY_KEY instead)'); process.exit(2); }
  const out = word('--out') ?? 'cloud-pull', quiet = A.includes('--quiet'), since = word('--since'), limit = word('--limit') ? Number(word('--limit')) : undefined;
  const w = wire(url, { key, timeoutMs: 60000 });
  try {
    mkdirSync(out, { recursive: true });
    const d = await pullDump(w, { since, pid: word('--pid'), limit, say: quiet ? undefined : s => console.log(`  ${s}`) });
    if (d) { writeFileSync(join(out, d.name), d.bytes); console.log(`${d.visits.length} visit${d.visits.length === 1 ? '' : 's'}, ${d.pieces} pieces → ${join(out, d.name)} (${(d.bytes.length / 1024).toFixed(0)} KB)`); } else console.log('no visits kept on the backend');
    const T: { at: number }[] = await w.query('telemetry:list', { ...(since ? { since: Date.parse(since) } : {}), limit: 2000 });
    writeFileSync(join(out, 'telemetry.ndjson'), T.sort((a, b) => a.at - b.at).map(r => JSON.stringify(r)).join('\n') + (T.length ? '\n' : ''));
    const P = await w.query('prefs:players', { limit: 2000 });
    writeFileSync(join(out, 'players.json'), JSON.stringify(P, null, 1));
    console.log(`${T.length} telemetry record${T.length === 1 ? '' : 's'} → ${join(out, 'telemetry.ndjson')} · ${(P as unknown[]).length} browser${(P as unknown[]).length === 1 ? '' : 's'} → ${join(out, 'players.json')}`);
  } catch (e) { console.error(`pull — the backend could not be read: ${String((e as Error)?.message ?? e)}`); process.exit(1); }
}
