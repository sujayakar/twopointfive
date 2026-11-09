// A HAND-OVER FILE OF KEPT RUNS, UNPACKED AND GONE THROUGH (src/tape/kept.ts is its format; tools/cloud/pull.ts writes it from a hosted build's backend):
//   bun run tools/qa/replay-dump.ts <dump.ndjson[.gz] | replay.json[.gz] | telemetry.ndjson | folder> … [--out DIR] [--moments N] [--again] [--quiet]
// Takes any mix, in any number and order, as often as they are sent. A folder is pull.ts's --out: its hand-over files and its telemetry.ndjson.
//   · A VISIT IS ONE ENTRY however many files carry it: the longest copy of its tape stands, and its notes and page errors are added up.
//   · A visit's tape is written to --out as <visit>.tape: the bytes HandTape wrote, the file `?playtape=` plays (src/host/tapepage.ts) and tools/draw/walk.ts walks.
//   · <visit>.moments.md = what to look at first: page errors and crashes (the telemetry's, where it was given), notes, the mission begun again, a man down by the
//     tape's figures, a hands-off player taking the seat, why the tape ends. A moment on the tape names its tick: `&at=<tick>` opens the page there.
//   · A RECORDING OF THE OLD KIND (`piece` and `file` lines, a saved replay file, F7's): the index says no page plays it and keeps its notes' words. It is not decoded.
//   · --out/index.json remembers what earlier calls unpacked there, --out/index.md is the table to read; --again goes through what is there with this build's reader.
// A tape is DATA: nothing in one is run or followed; text from one is printed quoted. No game is stood up: a tape is read by its own count of hand numbers.
// EXIT 0 read · 1 nothing readable in what was given · 2 usage.
import { closeSync, createReadStream, existsSync, mkdirSync, openSync, readFileSync, readSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { createGunzip, gunzipSync } from 'node:zlib';
import { basename, join } from 'node:path';
import { clock, type ReplayNote } from '../../src/tape/format.ts';
import { visitsOf, type KeptLine, type Visit } from '../../src/tape/kept.ts';
import { handsOf } from '../../src/tape/keep.ts';
import { HandWalk } from '../../src/tape/handtape.ts';

const OLD_KIND = 'a recording of the old kind: no page plays it';
const INDEX_VERSION = 2;
const quoted = (s: string, n = 140) => JSON.stringify(s.length > n ? s.slice(0, n) + '…' : s);
/** a browser's words for a visit, as a file's name here */
const safeName = (sid: string) => sid.replace(/[^\w.-]/g, '_').replace(/^\.+/, '_');

// ---------------------------------------------------------------- what a tape says of itself
export interface TapeFacts { build: string; level: string; words: string; seed: number; ticks: number; seconds: number; shots: number; moments: Moment[]; runs: string[]; ends: string }
export interface Moment { r: number | null; tick: number | null; kind: string; text: string; weight: number }
export function tapeFacts(b: Uint8Array): TapeFacts {
  const walk = new HandWalk(b, b.length < 12 ? 0 : new DataView(b.buffer, b.byteOffset, b.byteLength).getUint32(8, true)), S = walk.start;
  const moments: Moment[] = [], down = new Set<number>(), starts = [0]; let seconds = 0, ticks = 0, shots = 0, ends = '';
  for (let row = walk.next(); row; row = walk.next()) {
    const at = (kind: string, text: string, weight: number, tick: number) => moments.push({ r: seconds, tick, kind, text, weight });
    if (row.kind === 'step') { seconds += row.dt; ticks = row.tick + 1; }
    else if (row.kind === 'try') { starts.push(seconds); down.clear(); at('try', 'the mission began again', 4, row.tick); }
    else if (row.kind === 'hands' && row.plan.length) at('seat', 'a hands-off player took the seat', 2, row.tick);
    else if (row.kind === 'end') { ends = row.why; at('end', `the tape ends: ${quoted(row.why)}`, 7, row.tick); }
    else if (row.kind === 'figures') {
      shots = row.figures.shots;
      for (const m of row.figures.men) { if (!m.down) down.delete(m.id); else if (!down.has(m.id)) { down.add(m.id); at('down', `man ${m.id} down, by this tick, at ${m.x.toFixed(1)}, ${m.z.toFixed(1)}`, 5, row.figures.tick); } }
    }
  }
  const runs = starts.map((r, i) => `${clock(r)}–${clock(starts[i + 1] ?? seconds)}`);
  return { build: S.build, level: S.levelName, words: S.words, seed: S.seed, ticks, seconds, shots, moments, runs, ends };
}

// ---------------------------------------------------------------- reading a file
const isGz = (b: Uint8Array) => b[0] === 0x1f && b[1] === 0x8b;
/** the first characters of a file's text (gzip or plain), without reading the rest: a dump can be hundreds of megabytes */
async function firstText(path: string, chars = 256): Promise<string> {
  const head = Buffer.alloc(2); { const fd = openSync(path, 'r'); readSync(fd, head, 0, 2, 0); closeSync(fd); }
  const file = createReadStream(path), src = isGz(head) ? file.pipe(createGunzip()) : file; let text = '';
  try { const dec = new TextDecoder(); for await (const chunk of src as AsyncIterable<Buffer>) { text += dec.decode(chunk, { stream: true }); if (text.length >= chars) break; } } catch { /* what was read stands */ }
  (file as unknown as { destroy(): void }).destroy(); return text.slice(0, chars);
}
/** a file's lines, streamed: gzip members one after another (a dump) or plain text; a line can be megabytes */
async function eachLine(path: string, on: (line: string) => void | Promise<void>): Promise<void> {
  const head = Buffer.alloc(2); { const fd = openSync(path, 'r'); readSync(fd, head, 0, 2, 0); closeSync(fd); }
  const src = isGz(head) ? createReadStream(path).pipe(createGunzip()) : createReadStream(path);
  const dec = new TextDecoder(); let parts: string[] = [];
  for await (const chunk of src as AsyncIterable<Buffer>) {
    let text = dec.decode(chunk, { stream: true }), nl: number;
    while ((nl = text.indexOf('\n')) >= 0) { parts.push(text.slice(0, nl)); const line = parts.join(''); parts = []; text = text.slice(nl + 1); if (line.trim()) await on(line); }
    if (text) parts.push(text);
  }
  const tail = parts.join('') + dec.decode(); if (tail.trim()) await on(tail);
}

// ---------------------------------------------------------------- the index
interface PageError { at: number; kind: 'fatal' | 'error' | 'warning'; where: string; message: string }
interface Entry {
  sid: string; name: string; from: string[];
  /** hands: its tape is <name>.tape · old: a recording of the old kind · none: nothing of its play came, only what the telemetry said */
  kind: 'hands' | 'old' | 'none';
  startedAt: string; scenario: string; url: string; commit: string; seed: number | null; machine: string; seconds: number; ticks: number; bytes: number; lost: boolean; tape: string;
  notes: ReplayNote[]; errors: PageError[]; moments: number; top: string[]; runs: string[];
}
interface Index { v: number; entries: Record<string, Entry> }
const newEntry = (sid: string, kind: Entry['kind'], from: string): Entry => ({ sid, name: safeName(sid), from: [from], kind, startedAt: '', scenario: '', url: '', commit: '', seed: null, machine: '', seconds: 0, ticks: 0, bytes: 0, lost: false, tape: kind === 'old' ? OLD_KIND : 'none', notes: [], errors: [], moments: 0, top: [], runs: [] });
/** a note's latest words: one note by its id, wherever it came from */
function addNotes(E: Entry, notes: readonly ReplayNote[]): void {
  for (const n of notes) { const i = E.notes.findIndex(x => x.id === n.id); if (i < 0) E.notes.push(n); else if ((n.at ?? '') >= (E.notes[i].at ?? '')) E.notes[i] = n; }
  E.notes.sort((a, b) => a.r - b.r);
}
function momentsOf(E: Entry, facts: TapeFacts | null): Moment[] {
  const notes = E.notes.map((n): Moment => ({ r: n.r, tick: E.kind === 'hands' ? n.n : null, kind: 'note', text: quoted(n.text, 400), weight: 8 }));
  const errors = E.errors.map((e): Moment => ({ r: null, tick: null, kind: e.kind, text: `${new Date(e.at).toISOString()} ${e.where ? `${e.where}: ` : ''}${quoted(e.message, 240)}`, weight: e.kind === 'fatal' ? 10 : e.kind === 'error' ? 9 : 3 }));
  return [...errors, ...[...notes, ...(facts?.moments ?? [])].sort((a, b) => (a.r ?? 0) - (b.r ?? 0))];
}
const momentLine = (m: Moment) => `${m.r === null ? '' : `${clock(m.r)} (${m.r.toFixed(1)}) `}**${m.kind}** ${m.text}${m.tick === null ? '' : ` · \`&at=${m.tick}\``}`;

if (import.meta.main) {
  const argv = process.argv.slice(2), val = (k: string) => { const i = argv.indexOf(`--${k}`); return i >= 0 ? argv[i + 1] : undefined; };
  const skip = new Set<number>(); argv.forEach((a, i) => { if (a === '--out' || a === '--moments') skip.add(i + 1); });
  const given = argv.filter((a, i) => !a.startsWith('--') && !skip.has(i)), out = val('out') ?? 'replay-dump-out', quiet = argv.includes('--quiet'), again = argv.includes('--again'), showN = Number(val('moments') ?? 14);
  if (!given.length && !again) { console.log('usage: bun run tools/qa/replay-dump.ts <dump.ndjson[.gz] | replay.json[.gz] | telemetry.ndjson | folder> … [--out DIR] [--moments N] [--again] [--quiet]'); process.exit(2); }
  const say = (s: string) => { if (!quiet) console.log(s); };
  const paths: string[] = [];
  for (const g of given) {
    if (!existsSync(g)) { console.log(`${g}: not there`); continue; }
    if (statSync(g).isDirectory()) { for (const f of readdirSync(g).sort()) if (/\.(json|ndjson)(\.gz)?$/.test(f) && !/^(index|players|files)\.json$/.test(f)) paths.push(join(g, f)); } else paths.push(g);
  }
  mkdirSync(out, { recursive: true });
  const ixPath = join(out, 'index.json');
  let IX: Index = { v: INDEX_VERSION, entries: {} };
  try { if (existsSync(ixPath)) { const old = JSON.parse(readFileSync(ixPath, 'utf8')) as Index; if (old.v === INDEX_VERSION) IX = old; } } catch { /* a fresh index */ }
  const report = { read: 0, bad: 0, tapes: 0, grown: 0, same: 0, old: 0, unreadable: [] as string[] };
  const touched = new Set<string>();
  const entry = (sid: string, kind: Entry['kind'], from: string): Entry => {
    const E = IX.entries[sid] ??= newEntry(sid, kind, from);
    if (E.kind === 'none' || (E.kind === 'old' && kind === 'hands')) { E.kind = kind; E.tape = kind === 'old' ? OLD_KIND : E.tape; }   // (a visit's tape outranks what was said of it before it came)
    if (!E.from.includes(from)) E.from.push(from);
    touched.add(sid); return E;
  };

  /** a whole file of the old kind: its notes, and what it says of itself. Not decoded */
  const oldFile = (F: { session?: string; savedAt?: string; seconds?: number; url?: string; commit?: string; seed?: number | null; level?: { scenario?: string; file?: string }; notes?: ReplayNote[] }, from: string): void => {
    const E = entry(F.session ?? `file-${String(F.savedAt ?? from)}`, 'old', from); report.old++;
    if (E.kind === 'old') { E.startedAt ||= F.savedAt ?? ''; E.seconds = Math.max(E.seconds, F.seconds ?? 0); E.url ||= F.url ?? ''; E.commit ||= F.commit ?? ''; E.seed ??= F.seed ?? null; E.scenario ||= F.level?.scenario ?? F.level?.file ?? ''; }
    addNotes(E, F.notes ?? []);
  };
  const telemetry = (row: { sid: string; at: number; kind: string; build?: string; scenario?: string; data?: { level?: string; where?: string; message?: unknown; machine?: { webgpu?: string; webgl?: string } } }, from: string): void => {
    if (row.kind === 'visit' && IX.entries[row.sid]) { const m = row.data?.machine; IX.entries[row.sid].machine = String(m?.webgpu || m?.webgl || ''); touched.add(row.sid); return; }
    if (row.kind !== 'fatal' && row.kind !== 'error') return;
    const E = entry(row.sid, 'none', from), kind = row.kind === 'error' && row.data?.level === 'warning' ? 'warning' : row.kind, message = String(row.data?.message ?? '');
    if (!E.errors.some(e => e.at === row.at && e.message === message)) E.errors.push({ at: row.at, kind, where: String(row.data?.where ?? ''), message });
    if (E.kind === 'none') { E.commit ||= row.build ?? ''; E.scenario ||= row.scenario ?? ''; E.startedAt ||= new Date(row.at).toISOString(); }
  };
  const telemetryAfter: { path: string; from: string }[] = [];

  for (const p of paths) {
    const from = basename(p); report.read++;
    try {
      const first = (await firstText(p)).split('\n')[0];
      if (/^\s*\{\s*"(sid|_id|_creationTime)"/.test(first) || /(^|\/)telemetry\.ndjson$/.test(p)) { telemetryAfter.push({ path: p, from }); continue; }   // (after the visits: its `visit` records say a visit's machine)
      if (!/^\s*\{\s*"k"\s*:/.test(first)) {   // a whole file, not lines: a saved replay file of the old kind (small enough to read whole)
        const buf = readFileSync(p), F = JSON.parse(isGz(buf) ? new TextDecoder().decode(gunzipSync(buf)) : buf.toString('utf8')) as { kind?: string };
        if (F?.kind !== 'replay') { console.log(`${from}: not a replay`); report.unreadable.push(from); continue; }
        say(`${from}: ${OLD_KIND}`); oldFile(F as Parameters<typeof oldFile>[0], from); continue;
      }
      say(`${from}: a hand-over file`);
      let nBad = 0;
      const visits = new Map<string, Visit>(), hands = new Map<string, KeptLine[]>(), pieces = new Map<string, number>();
      await eachLine(p, line => {
        let L: KeptLine; try { L = JSON.parse(line) as KeptLine; } catch { nBad++; return; }
        if (!L || typeof L !== 'object' || typeof L.k !== 'string') { nBad++; return; }
        if (L.k === 'dump') say(`  made ${L.at} · lists ${L.visits.length} visit(s)`);
        else if (L.k === 'head' || L.k === 'notes') visitsOf([L], visits);
        else if (L.k === 'hands') hands.set(L.sid, [...(hands.get(L.sid) ?? []), L]);
        else if (L.k === 'piece') pieces.set(L.sid, (pieces.get(L.sid) ?? 0) + 1);
        else if (L.k === 'file') oldFile(L.file, from);
        else if (L.k === 'f7') oldFile({ session: L.session, savedAt: L.savedAt, seconds: L.seconds, level: { scenario: L.scenario }, notes: L.notes ?? [] }, from);
      });
      for (const sid of new Set([...visits.keys(), ...hands.keys(), ...pieces.keys()])) {
        const V = visits.get(sid), H = V?.head as (Visit['head'] & { hands?: number }) | null | undefined, lines = hands.get(sid);
        const kind = lines ? 'hands' : pieces.has(sid) || (H && !H.hands) ? 'old' : IX.entries[sid]?.kind ?? 'none', E = entry(sid, kind, from);
        if (H) { E.scenario = H.level?.scenario ?? H.level?.file ?? E.scenario; E.url = H.url ?? E.url; E.commit = H.commit ?? E.commit; E.seed = H.seed ?? E.seed; E.startedAt ||= H.at; }
        addNotes(E, V?.notes ?? []);
        if (kind === 'old') { report.old++; say(`  visit ${sid}: ${OLD_KIND}, ${E.notes.length} note(s)`); continue; }
        if (!lines) { say(`  visit ${sid}: no tape in this file`); continue; }
        const tape = handsOf(lines as never); if (!tape) { nBad++; say(`  visit ${sid}: its tape's first bytes are missing`); continue; }
        const file = join(out, `${E.name}.tape`), reach = Math.max(...lines.map(l => { const h = l as Extract<KeptLine, { k: 'hands' }>; return h.from + Math.floor(h.b.length * 3 / 4) - (h.b.endsWith('==') ? 2 : h.b.endsWith('=') ? 1 : 0); }));
        if (existsSync(file) && statSync(file).size >= tape.length) { report.same++; say(`  visit ${sid}: already here (${statSync(file).size} bytes)`); continue; }
        if (existsSync(file)) report.grown++; else report.tapes++;
        writeFileSync(file, tape); E.bytes = tape.length; E.lost = reach > tape.length; E.tape = `${E.name}.tape`;
        say(`  visit ${sid}: ${(tape.length / 1024).toFixed(0)} KB of tape → ${E.name}.tape${E.lost ? ' (cut at a missing piece)' : ''}`);
      }
      if (nBad) { say(`  ${nBad} line(s) could not be read (a write cut short)`); report.bad += nBad; }
    } catch (e) { console.log(`${from}: could not be read — ${(e as Error).message}`); report.unreadable.push(from); }
  }
  for (const { path, from } of telemetryAfter) try {
    say(`${from}: telemetry`);
    await eachLine(path, line => { try { const r = JSON.parse(line); if (typeof r?.sid === 'string' && typeof r?.kind === 'string') telemetry(r, from); else report.bad++; } catch { report.bad++; } });
  } catch (e) { console.log(`${from}: could not be read — ${(e as Error).message}`); report.unreadable.push(from); }

  if (again) for (const sid of Object.keys(IX.entries)) touched.add(sid);

  // ---- every entry touched: its tape read, its moments listed
  for (const sid of touched) {
    const E = IX.entries[sid], file = join(out, `${E.name}.tape`); let facts: TapeFacts | null = null;
    if (E.kind === 'hands' && existsSync(file)) {
      try { facts = tapeFacts(new Uint8Array(readFileSync(file))); E.seconds = Math.round(facts.seconds * 10) / 10; E.ticks = facts.ticks; E.seed = facts.seed; E.commit ||= facts.build; E.scenario ||= facts.level; E.url ||= facts.words; E.runs = facts.runs; }
      catch (e) { E.tape = `${E.name}.tape, not read here: ${(e as Error).message}`; E.runs = []; report.bad++; }
    }
    const M = momentsOf(E, facts), top = M.slice().sort((a, b) => b.weight - a.weight || (a.r ?? 0) - (b.r ?? 0)).slice(0, showN).sort((a, b) => (a.r ?? -1) - (b.r ?? -1));
    E.moments = M.length; E.top = top.map(momentLine);
    const play = E.kind === 'hands' ? `Play it: \`?playtape=${E.name}.tape\`, or walk it: \`bun run tools/draw/walk.ts ${E.name}.tape\`.${E.lost ? ' Its tape is cut at a missing piece.' : ''}` : E.kind === 'old' ? `${OLD_KIND}.` : 'Nothing of its play came, only what the telemetry said.';
    writeFileSync(join(out, `${E.name}.moments.md`), [`# ${E.sid}`, '', `${E.scenario || 'no mission named'} · ${clock(E.seconds)} of play · page \`${E.url}\` · build ${E.commit || '?'} · seed ${E.seed ?? '?'}${E.machine ? ` · ${E.machine}` : ''}${facts ? ` · ${facts.shots} rounds fired` : ''}`, play, '',
      ...(E.runs.length ? ['## Runs', ...E.runs.map(r => `- ${r}`), ''] : []), `## Moments worth a look (${M.length})`, '', 'Words in quotes are the tape\'s own, or the player\'s.', '', ...M.map(m => `- ${momentLine(m)}`), ''].join('\n'));
  }
  writeFileSync(ixPath, JSON.stringify(IX, null, 1));
  const rows = Object.values(IX.entries).sort((a, b) => a.startedAt < b.startedAt ? -1 : 1);
  writeFileSync(join(out, 'index.md'), ['# Replays unpacked here', '', '| visit | when | mission | play | tape | notes | errors | moments | from |', '|---|---|---|---|---|---|---|---|---|',
    ...rows.map(e => `| ${e.sid} | ${e.startedAt.slice(0, 16).replace('T', ' ')} | ${e.scenario} | ${clock(e.seconds)} | ${e.tape} | ${e.notes.length} | ${e.errors.length} | ${e.moments} | ${e.from.join(', ')} |`), ''].join('\n'));

  console.log(`\n=== ${report.read} file(s) read: ${report.tapes} new tape(s), ${report.grown} grown, ${report.same} already here · ${report.old} of the old kind${report.bad ? ` · ${report.bad} unreadable part(s)` : ''} ===`);
  if (!quiet) for (const sid of touched) { const E = IX.entries[sid]; console.log(`\n${E.sid} — ${E.scenario} · ${clock(E.seconds)} · tape ${E.tape} · ${E.notes.length} notes · ${E.moments} moments (${E.name}.moments.md)`); for (const r of E.runs) console.log(`  run ${r}`); for (const t of E.top) console.log(`    ${t}`); }
  console.log(`\nindex: ${join(out, 'index.md')}`);
  process.exit(touched.size || report.same ? 0 : 1);
}
