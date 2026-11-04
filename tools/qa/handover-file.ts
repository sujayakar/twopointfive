// A HAND-OVER FILE, MADE FOR THE CHECKS (src/replay/kept.ts has the format): everything a kept store holds as the one file of gzip members a reader takes —
// `{k:'dump'}` first, then each visit's head, its pieces as the store holds them, and the notes on its row. The page made this file itself ('Hand over
// replays', and /replay's two buttons) until that was taken out on 2026-10-27; tools/cloud/pull.ts still writes the same file from a hosted build's backend,
// and every tape sent before is one. So the READER stays (tools/qa/replay-dump.ts over kept.ts visitsOf / assemble; no page opens this file, /replay reads a visit out of
// the store with keepstore.ts readVisit), and replaykeep-probes.ts holds it over files made here. Nothing of a page calls this.
import { ROOT } from './headless.ts';
const KP = await import(`${ROOT}/src/replay/kept.ts`), KS = await import(`${ROOT}/src/replay/keepstore.ts`);

export interface HandOver { blob: Blob; name: string; visits: number; pieces: number }
/** every visit of `store` with a piece, oldest first, as one file's bytes (null: nothing kept) */
export async function handOver(store: any, opts: { now?: string } = {}): Promise<HandOver | null> {
  const gz = KS.gzip as ((text: string) => Promise<Blob>) | null, now = opts.now ?? new Date().toISOString();
  const line = (o: unknown) => JSON.stringify(o) + '\n', z = async (text: string): Promise<Blob | string> => gz ? gz(text) : text;
  const rows = ((await store.rows()) as any[]).filter(r => r.pieces > 0).sort((a, b) => a.startedAt < b.startedAt ? -1 : 1);
  const body: (Blob | string)[] = [], carried: { sid: string; pieces: number; bytes: number; startedAt: string; lastAt: string }[] = []; let pieces = 0;
  for (const r of rows) {
    const head = await store.head(r.sid), bodies = await store.bodies(r.sid) as (Blob | string)[]; if (!head || !bodies.length) continue;
    body.push(await z(head));
    for (const b of bodies) body.push(typeof b === 'string' ? await z(b) : b);   // (a piece written as its page went away is plain text: packed here)
    if (r.notes?.length) body.push(await z(line({ k: 'notes', sid: r.sid, notes: r.notes })));
    pieces += bodies.length; carried.push({ sid: r.sid, pieces: bodies.length, bytes: r.bytes, startedAt: r.startedAt, lastAt: r.lastAt });
  }
  if (!carried.length) return null;
  const top = await z(line({ k: 'dump', v: KP.KEPT_VERSION, at: now, shelf: 0, visits: carried }));
  return { blob: new Blob([top, ...body], { type: gz ? 'application/gzip' : 'application/x-ndjson' }),
    name: `twopointsix-replays-${now.replace(/[:.]/g, '-').replace('Z', '')}-${carried.length}-visit${carried.length === 1 ? '' : 's'}.ndjson${gz ? '.gz' : ''}`, visits: carried.length, pieces };
}
