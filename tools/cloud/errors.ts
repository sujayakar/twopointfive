// WHAT WENT WRONG ON PLAYERS' MACHINES, READ OUT OF THE BACKEND (cloud/convex/telemetry.ts — the `fatal` and `error` records of src/cloud/, joined to each
// visit's `visit` record for the machine it happened on):
//   bun run tools/cloud/errors.ts [--url https://<name>.convex.cloud] [--key <deploy key>] [--since 2026-10-26T00:00:00Z] [--metrics] [--json] [--limit N]
// The address and the key are read from the environment where the words are absent (TPS_CLOUD_URL or CONVEX_URL · CONVEX_DEPLOY_KEY), as tools/cloud/pull.ts
// reads them (the key alone is enough for a hosted deployment: it names it); the key is never written anywhere by this tool. Queries only.
//   (default)   the crashes and errors, one block a distinct message: how often, on how many visits and browsers, which builds, which GPUs, where the page
//               was (a fatal's stage), the first and last time, one stack. Pages that died come first, then by how many browsers met it.
//   --metrics   a line a browser besides: its GPU, visits, the shader pipelines' wait at boot, the frame's and the GPU's time in play, the preset it sat on
//               and THE LOADS: a line for every browser × shader rows × kind of load (first load, reload) — the whole load and its stages as medians
//               (src/cloud/loadtime.ts), so loads under one set of shader rows can be read against another's
//   --json      the same as JSON on stdout
// EXIT 0 read · 1 the backend could not be read · 2 usage.
import { keyUrl, localKey } from './local.ts';
import { wire } from '../../src/cloud/wire.ts';
import { LOADTIME } from '../../src/cloud/loadtime.ts';
import { SHADER_ROW_WORDS } from '../../src/ui/shaderrows.ts';

export interface Rec { sid: string; pid: string; at: number; kind: string; build?: string; scenario?: string; data: any }
export interface ErrGroup { kind: 'fatal' | 'error' | 'warning'; wheres: string[]; message: string; count: number; visits: number; browsers: number; builds: string[]; gpus: string[]; stages: string[]; scenarios: string[]; first: number; last: number; stack: string[]; sid: string; data?: unknown; pages: string[]; pageLoads: number }
export interface MachineLine { pid: string; gpu: string; browser: string; visits: number; fatals: number; errors: number; warnings: number; pipelinesMs: number | null; frameP50: number | null; frameP99: number | null; gpuMs: number | null; preset: string; playedMin: number; lagWorst: number | null; lagOver500: number; pages: number }

/** A PAGE THAT IS NOT A VISIT (src/cloud/cloud.ts cloudReports: the replay viewer, the game's page playing a tape) marks its `visit` record with the
 *  page's word: '' for a visit somebody played */
export const pageOf = (visit: Rec | undefined): string => typeof visit?.data?.page === 'string' ? visit.data.page : '';
/** the machine a visit ran on, in a few words: the adapter where the page got one, else the WebGL card, else that it had neither (a page that asks for
 *  no adapter, the replay viewer, says only that) */
export const gpuOf = (visit: Rec | undefined): string => { const m = visit?.data?.machine ?? {}; return String(m.webgpu || (m.webgl ? `no WebGPU adapter · ${m.webgl}` : '') || (pageOf(visit) ? 'no adapter asked' : visit ? 'no adapter' : 'unknown machine')).slice(0, 120); };
const browserOf = (visit: Rec | undefined): string => { const m = visit?.data?.machine ?? {}; return [m.browser, m.os].filter(Boolean).join(' · ') || String(m.userAgent ?? '').slice(0, 80) || 'unknown browser'; };
/** two messages that differ only in a number, an address or a place in a file are one */
export const messageKey = (m: string): string => m.replace(/https?:\/\/\S+/g, '<url>').replace(/0x[0-9a-f]+/gi, '<hex>').replace(/:\d+:\d+/g, '').replace(/\d{3,}/g, '<n>').trim().slice(0, 200);
const uniq = (a: (string | undefined)[]): string[] => [...new Set(a.filter((x): x is string => !!x))];
const median = (a: number[]): number | null => { const s = a.filter(Number.isFinite).sort((x, y) => x - y); return s.length ? s[(s.length - 1) >> 1] : null; };

const rank = (k: string): number => k === 'fatal' ? 0 : k === 'error' ? 1 : 2;
/** the crashes, errors and warnings as groups: pages that died first, warnings last */
export function errorGroups(rows: Rec[]): ErrGroup[] {
  const visits = new Map<string, Rec>(); for (const r of rows) if (r.kind === 'visit') visits.set(r.sid, r);
  const by = new Map<string, { g: ErrGroup; sids: Set<string>; pids: Set<string> }>();
  for (const r of rows) {
    if (r.kind !== 'fatal' && r.kind !== 'error') continue;
    // (a record the game went on from is a WARNING: src/cloud/report.ts `reportWarning`, and the page's console.warn)
    const kind = r.kind === 'error' && r.data?.level === 'warning' ? 'warning' : r.kind, message = String(r.data?.message ?? ''), key = `${kind}|${messageKey(message)}`, stack = Array.isArray(r.data?.stack) ? r.data.stack.map(String) : String(r.data?.stack ?? '').split('\n').filter(Boolean);
    let e = by.get(key);
    if (!e) by.set(key, e = { g: { kind, wheres: [], message, count: 0, visits: 0, browsers: 0, builds: [], gpus: [], stages: [], scenarios: [], first: r.at, last: r.at, stack, sid: r.sid, ...(r.data?.data !== undefined ? { data: r.data.data } : {}), pages: [], pageLoads: 0 }, sids: new Set(), pids: new Set() });
    const g = e.g; g.count++; e.sids.add(r.sid); e.pids.add(r.pid); g.first = Math.min(g.first, r.at); g.last = Math.max(g.last, r.at);
    g.builds = uniq([...g.builds, r.build]); g.gpus = uniq([...g.gpus, gpuOf(visits.get(r.sid))]); g.stages = uniq([...g.stages, r.data?.stage]); g.wheres = uniq([...g.wheres, r.data?.where]); g.scenarios = uniq([...g.scenarios, r.scenario]); g.pages = uniq([...g.pages, pageOf(visits.get(r.sid))]);
  }
  // (a page that is not a visit — the replay viewer, a tape's page — is counted apart: its name is no visit)
  return [...by.values()].map(e => { const pages = [...e.sids].filter(s => pageOf(visits.get(s))).length; return { ...e.g, visits: e.sids.size - pages, pageLoads: pages, browsers: e.pids.size }; }).sort((a, b) => rank(a.kind) - rank(b.kind) || b.browsers - a.browsers || b.count - a.count || b.last - a.last);
}

/** a line a browser: its machine and how the game ran on it */
export function machineLines(rows: Rec[]): MachineLine[] {
  const by = new Map<string, Rec[]>(); for (const r of rows) { let a = by.get(r.pid); if (!a) by.set(r.pid, a = []); a.push(r); }
  return [...by.entries()].map(([pid, rs]) => {
    // (the machine is read off a visit somebody played where there is one: a page's own record — the replay viewer's — names no adapter)
    const of = (k: string) => rs.filter(r => r.kind === k), played = of('visit').filter(v => !pageOf(v)), visit = (played.length ? played : of('visit')).sort((a, b) => b.at - a.at)[0], perf = of('perf');
    const presets = new Map<string, number>(); for (const p of perf) presets.set(String(p.data?.preset ?? ''), (presets.get(String(p.data?.preset ?? '')) ?? 0) + (Number(p.data?.secs) || 0));
    return { pid, gpu: gpuOf(visit), browser: browserOf(visit), visits: played.length, pages: of('visit').length - played.length, fatals: of('fatal').length, errors: of('error').filter(r => r.data?.level !== 'warning').length, warnings: of('error').filter(r => r.data?.level === 'warning').length,
      pipelinesMs: median(of('boot').map(b => Number(b.data?.pipelinesMs))), frameP50: median(perf.map(p => Number(p.data?.p50))), frameP99: median(perf.map(p => Number(p.data?.p99))), gpuMs: median(perf.map(p => Number(p.data?.gpu?.mean))),
      preset: [...presets.entries()].sort((a, b) => b[1] - a[1])[0]?.[0] ?? '', playedMin: perf.reduce((a, p) => a + (Number(p.data?.secs) || 0), 0) / 60,
      // (the GPU's wait on a frame, the stand-in for a frozen picture — src/cloud/telemetry.ts `lag`: the worst of all the minutes, and how many waits ran past half a second)
      lagWorst: perf.some(p => p.data?.lag) ? Math.max(...perf.map(p => Number(p.data?.lag?.worst) || 0)) : null, lagOver500: perf.reduce((a, p) => a + (Number(p.data?.lag?.over500) || 0), 0) };
  }).sort((a, b) => b.fatals - a.fatals || b.visits - a.visits);
}

/** WHERE THE QUALITY STOOD AND WHEN IT MOVED (the perf records' `rung`: src/cloud/telemetry.ts), a visit's stretches of play in order, each with the
 *  mission it was played in, the word it began on with the ladder's line by then, and every change with its second of play into the stretch and the
 *  ladder's line for it (`held` = made while play was not live: its second is where play stood). Only visits whose quality moved, the newest first */
export interface MoveLine { pid: string; sid: string; stretches: { scenario: string; secs: number; from: string; why: string; moves: { t: number; to: string; why: string; held: boolean }[]; more: number }[] }
export function moveLines(rows: Rec[]): MoveLine[] {
  const by = new Map<string, Rec[]>();
  for (const r of rows) if (r.kind === 'perf' && typeof r.data?.rung?.from === 'string') { let a = by.get(r.sid); if (!a) by.set(r.sid, a = []); a.push(r); }
  const out: MoveLine[] = [];
  for (const rs of [...by.values()].sort((a, b) => b[0].at - a[0].at)) {
    rs.sort((a, b) => a.at - b.at);
    // (the backend takes any words under a record: an entry that is not a move is left out, never read)
    const stretches = rs.map(r => { const g = r.data.rung, raw: unknown[] = Array.isArray(g.moves) ? g.moves : [];
      return { scenario: String(r.scenario ?? ''), secs: Number(r.data.secs) || 0, from: String(g.from), why: typeof g.why === 'string' ? g.why : '', more: Number(g.more) || 0,
        moves: raw.filter((m): m is unknown[] => Array.isArray(m) && Number.isFinite(Number(m[0])) && typeof m[1] === 'string').map(m => ({ t: Number(m[0]), to: String(m[1]), why: typeof m[2] === 'string' ? m[2] : '', held: m[3] === 1 })) }; });
    const last = (x: MoveLine['stretches'][number]): string => x.moves.length ? x.moves[x.moves.length - 1].to : x.from;
    if (stretches.some((x, i) => x.moves.length > 0 || x.more > 0 || (i > 0 && x.from !== last(stretches[i - 1])))) out.push({ pid: rs[0].pid, sid: rs[0].sid, stretches });
  }
  return out;
}
export function moveText(lines: MoveLine[]): string {
  if (!lines.length) return "no visit's quality moved during play (or none says: records from before the page noted it)";
  const out: string[] = [];
  for (const l of lines) {
    out.push(`${l.pid.slice(0, 8)}  ${l.sid}`);
    l.stretches.forEach((x, i) => {
      out.push(`  stretch ${i + 1}${x.scenario ? `, ${x.scenario}` : ''} (${Math.round(x.secs)} s of play): began on ${x.from}${x.why ? `  [${x.why}]` : ''}`);
      for (const m of x.moves) out.push(`    ${m.t.toFixed(1)} s${m.held ? ' (while play was held)' : ''} → ${m.to}${m.why ? `  [${m.why}]` : ''}`);
      if (x.more) out.push(`    and ${x.more} more`);
    });
  }
  return out.join('\n');
}
/** the records with a record written twice read once (the same visit, kind, moment and words: a leaving page's call that did not come back was sent again) */
export function once(rows: Rec[]): Rec[] {
  const seen = new Set<string>();
  return rows.filter(r => { const k = `${r.sid}|${r.kind}|${r.at}|${JSON.stringify(r.data ?? null)}`; if (seen.has(k)) return false; seen.add(k); return true; });
}
/** the shader rows' URL words as of 2026-10-27 (src/ui/shaderrows.ts), for records that carry no list of their own */
/** the loads of one browser under one set of shader rows and one kind of load (`nav`: first · later · navigate · reload · …) (the `boot` records: src/cloud/loadtime.ts), each number a median in ms */
/** a bundle that came over the wire in fewer bytes than this did not come over it: 0 = the browser's own copy, a few hundred = asked again and not sent */
const CACHED_WIRE = 2048;
export interface LoadLine { pid: string; gpu: string; browser: string; nav: string; rows: string; loads: number; missions: number; whole: number | null; toCard: boolean; script: number | null; device: number | null; level: number | null; gltf: number | null; game: number | null; pipelinesMs: number | null; shown: number | null; later: number | null; wire: number | null; cached: number;
  /** `settled`: ms to a mission's START going live (the boot's mark, on the load's record or in the visit's `later` record); `cutMs`: ms the programs with
   *  the cut rule took to build behind the boot; `madeMost`: the most programs one load of the line made (`made` is the median) */
  settled: number | null; cutMs: number | null; madeMost: number | null;
  /** THE KEPT PAGE (a mission start without a page load): '' = a whole page as before · 'first' = the load that made the page and compiled (the record's `kept` 1) ·
   *  'start' = a mission start on the page already made (`kept` 2+). `shell`: ms the shell page took before the game's frame began, part of a 'first' load's
   *  whole · `made` of `asked`: programs compiled in the load, of those it asked for · `labels`: names of what a 'start' still compiled */
  kept: '' | 'first' | 'start'; shell: number | null; made: number | null; asked: number | null; labels: string[] }
/** a line for every browser × shader rows × kind of load, a browser's lines together, first loads before reloads. A record from before the whole load was
 *  in it has no `whole` and no `rows`: its load is read to the start card (`toCard`) and its rows are read off the visit's address, or are '?' = not recorded */
export function loadLines(rows: Rec[]): LoadLine[] {
  const visits = new Map<string, Rec>(), bySid = new Map<string, Rec>();
  // (a browser's machine is read off its newest visit somebody played: a page's own record — the replay viewer's — names no adapter)
  for (const r of rows) if (r.kind === 'visit') { bySid.set(r.sid, r); const had = visits.get(r.pid); if (!had || (!pageOf(had) !== !pageOf(r) ? !pageOf(r) : had.at < r.at)) visits.set(r.pid, r); }
  // the moment the programs built behind the start were in: on the load's own record, or in the visit's `later` record where it came after that was written
  // (…and so for a mission's START going live and the cut programs' build: a visit may write several `later` records, each with what was new)
  const lateBySid = new Map<string, Record<string, unknown>>();
  for (const r of rows) if (r.kind === 'later' && r.data && typeof r.data === 'object') { const o = lateBySid.get(r.sid) ?? {}; for (const k of Object.keys(LOADTIME.late)) if (Number(r.data[k]) > 0) o[k] = Number(r.data[k]); if (Number(r.data.cut?.ms) > 0 && !r.data.cut.failed) o.cutMs = Number(r.data.cut.ms); lateBySid.set(r.sid, o); }
  // the rows a load was started with: the record's own list; a record from before it carried one says only what the visit's ADDRESS asked for
  // (`?<row>`, any value but 0, as the page reads it: a row put on in Options is not there), marked as such; neither = not recorded
  const rowsOf = (r: Rec): string => {
    if (Array.isArray(r.data?.rows)) return [...r.data.rows].map(String).sort().join(' + ');
    const q = bySid.get(r.sid)?.data?.query; if (typeof q !== 'string') return '?';
    const on: string[] = []; try { for (const [k, v] of new URLSearchParams(q)) if (v !== '0' && SHADER_ROW_WORDS.includes(k)) on.push(k); } catch { /* */ }
    return `by the address alone: ${on.sort().join(' + ') || 'none'}`;
  };
  // the kind of load: a reload; else, where the record counts the tab's loads (`nth`), the tab's FIRST load or a LATER one (a mission start is a page
  // load, and so is the way back to the list); a record from before the count says only that it was not a reload
  // (on the kept page the game runs in a frame whose own navigation is always "navigate": the TAB's kind rides on the first load's record as `shellNav`)
  const kindOf = (r: Rec): string => { const nav = String(r.data?.shellNav || r.data?.nav || 'navigate'), n = Number(r.data?.nth); return nav !== 'navigate' ? nav : n === 1 ? 'first' : n > 1 ? 'later' : 'navigate'; };
  // the kept page: the load that made it, or a mission start on it (a start is one kind whatever the tab's navigation was)
  const keptOf = (r: Rec): LoadLine['kept'] => { const k = Number(r.data?.kept); return k === 1 ? 'first' : k > 1 ? 'start' : ''; };
  const by = new Map<string, Rec[]>();
  for (const r of rows) {
    if (r.kind !== 'boot') continue;
    const set = rowsOf(r);
    const key = `${r.pid}|${keptOf(r) === 'start' ? 'start' : kindOf(r)}|${keptOf(r)}|${set}`; let a = by.get(key); if (!a) by.set(key, a = []); a.push(r);
  }
  const order = (nav: string): number => Math.max(0, ['first', 'later', 'navigate', 'reload'].indexOf(nav) + 1 || 5);
  const keptOrder = (k: string): number => k === 'start' ? 1 : 0;
  return [...by.values()].map(rs => {
    const d = rs.map(r => ({ ...(lateBySid.get(r.sid) ?? {}), ...(r.data ?? {}) })), med = (f: (x: any) => unknown) => median(d.map(x => Number(f(x) ?? NaN))), r0 = rs[0], visit = visits.get(r0.pid);
    const toCard = d.every(x => !(Number(x.whole) > 0));
    const kept = keptOf(r0), labels = uniq(d.flatMap(x => Array.isArray(x.made?.labels) ? x.made.labels.map(String) : [])).slice(0, 8);
    // (a kept page's first load began in its shell: the record's `whole` is on the frame's clock, so the shell's ms are added for the load as the player had it)
    const wholeOf = (x: any): unknown => Number(x.whole) > 0 ? Number(x.whole) + (kept === 'first' && Number(x.shell) > 0 ? Number(x.shell) : 0) : x.interactive;
    return { pid: r0.pid, gpu: gpuOf(visit), browser: browserOf(visit), nav: kept === 'start' ? 'start' : kindOf(r0), rows: rowsOf(r0), kept, shell: kept === 'first' ? med(x => x.shell) : null, made: med(x => x.made?.pipelines), asked: med(x => x.made?.asked), labels, loads: rs.length, missions: d.filter(x => typeof x.mission === 'string' && x.mission).length,
      whole: med(wholeOf), toCard, script: med(x => x.script), device: med(x => x.device), level: med(x => x.level), gltf: med(x => x.gltf), game: med(x => x.game), pipelinesMs: med(x => x.pipelinesMs), shown: med(x => x.shown), later: med(x => x.later), settled: med(x => x.settled), cutMs: med(x => x.cutMs), madeMost: d.some(x => Number.isFinite(Number(x.made?.pipelines))) ? Math.max(...d.map(x => Number(x.made?.pipelines)).filter(Number.isFinite)) : null,
      wire: med(x => x.net?.wire), cached: d.filter(x => x.net && Number(x.net.scriptSize) > 0 && Number(x.net.scriptWire) < CACHED_WIRE).length };
  }).sort((a, b) => a.pid.localeCompare(b.pid) || keptOrder(a.kept) - keptOrder(b.kept) || order(a.nav) - order(b.nav) || String(a.rows).localeCompare(String(b.rows)));
}
const secs = (ms: number | null): string => ms === null ? '-' : `${(ms / 1000).toFixed(ms >= 10000 ? 0 : 1)} s`;
const mb = (bytes: number | null): string => bytes === null ? '-' : bytes >= 1e6 ? `${(bytes / 1e6).toFixed(1)} MB` : `${Math.round(bytes / 1e3)} kB`;
export function loadText(lines: LoadLine[]): string {
  if (!lines.length) return 'no load has been recorded';
  const out: string[] = []; let pid = '';
  for (const l of lines) {
    if (l.pid !== pid) { pid = l.pid; out.push(`${l.pid.slice(0, 8)}  ${l.gpu}  (${l.browser})`); }
    const pl = l.loads === 1 ? '' : 's', kind = (l.kept === 'start' ? `mission start${pl} on the kept page (no page load)` : l.nav === 'first' ? `first load${pl} of a tab` : l.nav === 'later' ? `later load${pl} in a tab` : l.nav === 'navigate' ? `load${pl} that ${l.loads === 1 ? 'was' : 'were'} not a reload (first of a tab or a mission start: not recorded)` : l.nav === 'reload' ? `reload${pl}` : `${l.nav} load${pl}`) + (l.kept === 'first' ? ' that made the kept page' : '') + (l.missions && l.kept !== 'start' ? `, ${l.missions} into a mission` : '');
    const stages = [['page script at', l.script], ['device', l.device], ['level', l.level], ['characters', l.gltf], ['game', l.game], ['shaders', l.pipelinesMs], ['first picture at', l.shown], ['the last program built behind the start in at', l.later], ['START live at', l.settled], ['the cut programs built in', l.cutMs]].filter(x => x[1] !== null).map(x => `${x[0]} ${secs(x[1] as number)}`);
    out.push(`  ${l.loads} ${kind} · ${l.rows === '?' ? 'shader rows not recorded' : l.rows ? `rows ${l.rows}` : 'no shader row on'} · ${l.toCard ? 'to the start card' : l.kept === 'start' ? 'to its first picture' : 'whole load'} ${secs(l.whole)}${l.shell !== null ? ` (the shell page ${secs(l.shell)} of it)` : ''}${l.made !== null ? ` · compiled ${Math.round(l.made)}${l.madeMost !== null && l.madeMost > l.made ? ` (${Math.round(l.madeMost)} at the most)` : ''}${l.asked !== null ? ` of ${Math.round(l.asked)}` : ''} programs${l.kept === 'start' && l.labels.length ? ` (${l.labels.join(', ')})` : ''}` : ''}${stages.length ? ` · ${stages.join(' · ')}` : ''}${l.wire !== null ? ` · ${mb(l.wire)} over the wire${l.cached ? `, the bundle from the browser's cache in ${l.cached}` : ''}` : ''}${l.later === null && /(^| )firstframe( |$)/.test(l.rows) ? ' · the programs built behind the start never said they were in' : ''}`);
  }
  return out.join('\n');
}

const when = (ms: number): string => new Date(ms).toISOString().slice(0, 16).replace('T', ' ');
export function errorText(groups: ErrGroup[]): string {
  if (!groups.length) return 'no crashes and no errors in the records read';
  return groups.map(g => [
    `${g.kind === 'fatal' ? 'THE PAGE DIED' : g.kind}${g.stages.length ? ` (${g.stages.join(', ')})` : ''}${g.wheres.length ? ` at ${g.wheres.join(', ')}` : ''}: ${g.message}`,
    `  ${g.count} time${g.count === 1 ? '' : 's'} · ${g.visits} visit${g.visits === 1 ? '' : 's'}${g.pageLoads ? ` · ${g.pageLoads} other page${g.pageLoads === 1 ? '' : 's'}` : ''} · ${g.browsers} browser${g.browsers === 1 ? '' : 's'} · ${when(g.first)}${g.last !== g.first ? ` to ${when(g.last)}` : ''} UTC`,
    `  builds: ${g.builds.join(', ') || 'not said'}`, `  GPUs: ${g.gpus.join(' | ')}`,
    ...(g.scenarios.length ? [`  in: ${g.scenarios.join(', ')}`] : []), ...(g.pages.length ? [`  ${g.visits ? 'also ' : ''}on a page that is not a visit: ${g.pages.join(', ')}`] : []), ...g.stack.slice(0, 8).map(l => `    ${l}`),
    ...(g.data !== undefined ? [`  reporter: ${JSON.stringify(g.data).slice(0, 1200)}`] : []), `  a visit with it: ${g.sid}`,
  ].join('\n')).join('\n\n');
}
const n1 = (x: number | null, unit: string): string => x === null ? '-' : `${x >= 100 ? Math.round(x) : x.toFixed(1)} ${unit}`;
export function machineText(lines: MachineLine[]): string {
  return lines.map(m => `${m.pid.slice(0, 8)}  ${m.gpu}  (${m.browser})\n  ${m.visits} visit${m.visits === 1 ? '' : 's'}${m.pages ? ` · ${m.pages} other page${m.pages === 1 ? '' : 's'} that reported` : ''} · ${m.fatals} died · ${m.errors} errors${m.warnings ? ` · ${m.warnings} warnings` : ''} · shaders ${n1(m.pipelinesMs, 'ms')} · frame ${n1(m.frameP50, 'ms')} median, ${n1(m.frameP99, 'ms')} at the 99th · GPU ${n1(m.gpuMs, 'ms')} · ${m.preset || 'no play'} · ${m.playedMin.toFixed(1)} min played${m.lagWorst === null ? '' : ` · the GPU's longest wait on a frame ${n1(m.lagWorst, 'ms')}, ${m.lagOver500} past half a second`}`).join('\n');
}

if (import.meta.main) {
  const A = process.argv.slice(2), word = (k: string): string | undefined => { const i = A.indexOf(k); return i >= 0 ? A[i + 1] : undefined; };
  // (--local: the deployment `bun run dev` runs on this machine — tools/cloud/local.ts; its address and key are where `convex dev` keeps them)
  const here = A.includes('--local') ? localKey() : null; if (A.includes('--local') && !here) { console.error('no local deployment was ever started here (cloud/.convex/local/default/config.json is missing). Run `bun run dev` once.'); process.exit(2); }
  const key = here?.key ?? word('--key') ?? process.env.CONVEX_DEPLOY_KEY, url = here?.url ?? word('--url') ?? process.env.TPS_CLOUD_URL ?? process.env.CONVEX_URL ?? (keyUrl(key) || undefined);
  if (!url || !key || A.includes('--help')) { console.error('usage: bun run tools/cloud/errors.ts [--local | --url <address> --key <deploy key>] [--since <ISO>] [--metrics] [--json] [--limit N]\n(the address and the key may be in TPS_CLOUD_URL / CONVEX_URL and CONVEX_DEPLOY_KEY instead; the key of a hosted deployment alone names its address)'); process.exit(2); }
  const since = word('--since'), limit = Math.min(Number(word('--limit')) || 2000, 2000), metrics = A.includes('--metrics'), w = wire(url, { key, timeoutMs: 60000 });
  try {
    const rows: Rec[] = [];
    for (const kind of ['fatal', 'error', 'visit', ...(metrics ? ['boot', 'later', 'perf'] : [])]) {
      const got: Rec[] = await w.query('telemetry:list', { kind, ...(since ? { since: Date.parse(since) } : {}), limit });
      if (got.length >= limit && !A.includes('--json')) console.error(`(${kind}: the newest ${limit} records were read; there may be older ones — narrow with --since)`);
      rows.push(...got);
    }
    { const n = rows.length, kept = once(rows); if (kept.length < n) { if (!A.includes('--json')) console.error(`(${n - kept.length} record${n - kept.length === 1 ? ' was' : 's were'} in the backend twice and read once)`); rows.length = 0; rows.push(...kept); } }
    const groups = errorGroups(rows), lines = metrics ? machineLines(rows) : [], loads = metrics ? loadLines(rows) : [], moves = metrics ? moveLines(rows) : [];
    if (A.includes('--json')) console.log(JSON.stringify({ groups, ...(metrics ? { machines: lines, loads, moves } : {}) }, null, 1));
    else { console.log(errorText(groups)); if (metrics) console.log(`\n=== the machines ===\n${machineText(lines)}\n\n=== the loads (medians; the stages overlap and do not add up) ===\n${loadText(loads)}\n\n=== where the quality stood and when it moved (seconds of play into each stretch) ===\n${moveText(moves)}`); }
  } catch (e) { console.error(`errors — the backend could not be read: ${String((e as Error)?.message ?? e)}`); process.exit(1); }
}
