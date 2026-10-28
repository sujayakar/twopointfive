// A LOAD'S RECORD (the `boot` kind of src/cloud/telemetry.ts): the whole load and its stages, made from what the page ALREADY TAKES — ui/boot.ts's marks and
// spans, and the browser's own timing of the page's downloads. Nothing is measured for it and nothing is asked in a frame: cloud.ts asks once the boot is over.
//   whole         ms from the navigation's start to the later of the start card being live and the first frame's GPU work being back: THE LOAD.
//                 Absent where the first picture never came back before the record was written (the page was left, or `waitMs` ran out): such a
//                 load is known only up to its start card, and is never read as a fast one
//   interactive   …to the start card being live · `shown` …to the first frame's GPU work being back (the first picture)
//   script        …to the page's script starting (the HTML and the bundle fetched and parsed) · `device` the adapter and the device · `level` the level
//                 built · `gltf` the characters fetched and read · `game` the game made · `pipelinesMs` the wait on shader pipelines, with `pipelines`
//                 (total · done · failed), the three `slowest` by label and `shaderText` · `firstFrameCpu` / `firstFrameGpu` the first frame
//   marks, spans  ui/boot.ts's two tables whole (ms since the navigation's start by mark; ms by span, the shader groups' among them): a mark added there
//                 is in the record with no change here. The stages OVERLAP (shaders compile while the characters load and the game is made), so the
//                 durations do not add up to `whole`; the marks in time order are the true timeline
//   net           what came down for it, by the browser's count (same origin only, up to `whole`): `html` ms to the document's last byte, `script` ms the
//                 bundle took and `scriptWire` / `scriptSize` its bytes over the wire and unpacked (wire 0 = from the browser's cache; a few hundred = asked
//                 again and not sent again), `files` how many
//                 files, `wire` / `size` their bytes, `last` ms to the last one's last byte. Numbers only: no file's name
//   rows          the shader rows this load was started with, by their URL words (ui/shaderrows.ts): a load's time means little without them
//   nth, mission  WHICH LOAD OF THE TAB this is (1 = its first; sessionStorage keeps the count) and the mission the page opened on ('' = the start
//                 page). A MISSION START IS A PAGE LOAD, and so is the way back to the list: `nav` says `navigate` for those as for a first visit, so
//                 without these two a tab's first load and its tenth mission read alike
//   nav           the browser's word for the load: navigate · reload · back_forward (a browser keeps compiled shaders between loads, or does not)
//   maxRafGap, maxLongTask   the longest frozen stretch and the longest main-thread task of the boot
// The flat keys of the first records (interactive, script, device, pipelinesMs …) are kept as they were: an old reader reads a new record.
// No game code is imported: the page hands in the summary.

/** ui/boot.ts's summary, the part read here */
export interface BootLike {
  marks: Record<string, number>; spans: Record<string, number>;
  pipelines: { total: number; done: number; failed: number };
  shaderText: { after: number };
  slowest: { label: string; ms: number }[];
  maxRafGap: number; maxLongTask: number;
}
/** the browser's `performance`, the part read here */
export interface PerfLike { now(): number; getEntriesByType(type: string): unknown[] }
/** `waitMs`: a boot whose first frame never came back is written this long after the start card went live; `keys`: marks or spans kept, at most;
 *  `late`: THE BOOT'S MARKS THAT MAY COME AFTER THE LOAD'S RECORD IS WRITTEN, by the record's key for each (ui/boot.ts makes them, or does not):
 *  `later` = the moment the last program built BEHIND the start card was in (a load that starts on the first frame's shaders makes it), `settled` = the
 *  moment a mission's START went live (the performance work's ui/settle.ts: every program in and the frames behind the brief drawn — only a page with a brief;
 *  no page of this build makes `settled` yet); `laterMs`: how long after the page opened it still looks for them */
export const LOADTIME = { waitMs: 60000, keys: 48, late: { later: 'laterReady', settled: 'settled' } as Record<string, string>, laterMs: 600000 };
/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — the record as it was before the whole load was in it; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();

interface Timing { name?: string; startTime?: number; responseEnd?: number; duration?: number; transferSize?: number; decodedBodySize?: number; initiatorType?: string; type?: string }
const num = (x: unknown): number => typeof x === 'number' && Number.isFinite(x) ? x : 0;
const r1 = (x: number): number => Math.round(x * 10) / 10;
const table = (t: Record<string, number>): Record<string, number> => { const o: Record<string, number> = {}; let n = 0; for (const k of Object.keys(t)) { if (n++ >= LOADTIME.keys) break; if (Number.isFinite(t[k])) o[k] = r1(t[k]); } return o; };

/** what came down for the load, by the browser's count: same-origin files that had ended by `until` ms. Never throws; {} where the browser says nothing */
export function loadNet(P: PerfLike, until: number, origin: string): Record<string, number> {
  try {
    const nav = P.getEntriesByType('navigation')[0] as Timing | undefined, o: Record<string, number> = {};
    if (nav && num(nav.responseEnd) > 0) o.html = r1(num(nav.responseEnd));
    let files = 0, wire = num(nav?.transferSize), size = num(nav?.decodedBodySize), last = num(nav?.responseEnd), script: Timing | null = null;
    for (const e of P.getEntriesByType('resource') as Timing[]) {
      if (!origin || !String(e.name ?? '').startsWith(origin + '/') || !(num(e.responseEnd) <= until)) continue;
      files++; wire += num(e.transferSize); size += num(e.decodedBodySize); last = Math.max(last, num(e.responseEnd));
      // (the bundle: the biggest script the page asked for)
      if (e.initiatorType === 'script' && (!script || num(e.decodedBodySize) > num(script.decodedBodySize))) script = e;
    }
    if (script) { o.script = r1(num(script.duration)); o.scriptWire = num(script.transferSize); o.scriptSize = num(script.decodedBodySize); }
    o.files = files; o.wire = wire; o.size = size; if (last > 0) o.last = r1(last);
    return o;
  } catch { return {}; }
}

const SS_LOADS = 'twopointsix.cloud.loads';
let nthLoad = 0;
/** which load of this tab this is, 1 = its first (counted once a page, the first time it is asked; 1 where the browser keeps no session storage) */
export function tabLoad(store?: { getItem(k: string): string | null; setItem(k: string, v: string): void }): number {
  if (nthLoad) return nthLoad;
  nthLoad = 1;
  try { const S = store ?? sessionStorage, n = Math.min(9999, (Number(S.getItem(SS_LOADS)) || 0) + 1); S.setItem(SS_LOADS, String(n)); nthLoad = n; } catch { /* no session storage: every load reads as a first */ }
  return nthLoad;
}
/** (a check's: a page loads once, a check many times) */
export function tabLoadForget(): void { nthLoad = 0; }

/** THE LOAD'S RECORD, or null while the boot is not over: the start card is not live yet, or the first frame's GPU work is not back and the card has been
 *  live for less than `waitMs` (`last` = the page is going: say what there is). `rows` = the shader rows the load was started with; `origin` = the page's;
 *  `mission` = the mission the page opened on */
export function bootRecord(s: BootLike, P: PerfLike, last: boolean, rows: readonly string[], origin: string, mission = '', store?: Parameters<typeof tabLoad>[0]): Record<string, unknown> | null {
  const m = s.marks, sp = s.spans;
  if (m.interactive === undefined || (!last && m.gpuReady === undefined && P.now() - m.interactive < LOADTIME.waitMs)) return null;
  let nav = ''; try { nav = String((P.getEntriesByType('navigation')[0] as Timing | undefined)?.type ?? ''); } catch { /* */ }
  const was = { interactive: m.interactive, script: sp.beforeScript, device: sp.gpu, pipelinesMs: sp.pipelines, pipelines: s.pipelines, slowest: s.slowest.slice(0, 3), shaderText: s.shaderText.after, firstFrameCpu: sp.firstFrameCpu, firstFrameGpu: sp.firstFrameGpu, maxRafGap: s.maxRafGap, maxLongTask: s.maxLongTask, nav };
  if (BREAK) return was;
  const whole = Math.max(m.interactive, m.gpuReady ?? 0);
  return { ...was, ...(m.gpuReady !== undefined ? { whole: r1(whole), shown: m.gpuReady } : {}), level: sp.level, gltf: sp.gltf, game: sp.gameCtor, marks: table(m), spans: table(sp), net: loadNet(P, whole, origin), ...lateMarks(m, []), rows: rows.slice(0, 16), nth: tabLoad(store), mission: mission.slice(0, 120) };
}
/** the late marks that are there and that `have` does not hold yet, by the record's key */
function lateMarks(m: Record<string, number>, have: readonly string[]): Record<string, number> {
  const o: Record<string, number> = {};
  for (const k of Object.keys(LOADTIME.late)) { const at = m[LOADTIME.late[k]]; if (at !== undefined && Number.isFinite(at) && !have.includes(k)) o[k] = r1(at); }
  return o;
}
/** A LOAD THAT STARTED ON THE FIRST FRAME'S SHADERS is not over at its first picture: the heaviest programs come in behind the start card, and until
 *  they are in the picture runs on stand-ins — `whole`, `shown` and `pipelinesMs` are then the short wait and say nothing of how long that was. The
 *  boot marks the moment the last of them is in, and a mission's page the moment its START went live (`LOADTIME.late`). A mark that is there when the
 *  load's record is written rides on it under its key (`later`, `settled`); one that comes after goes in a small record of its own (kind `later`, the
 *  same visit's), with `after` / `settledAfter` = ms since the start card went live. `have` = the keys the visit's records already carry; `more` = other
 *  late facts of the load by key, sent the same way once each (the page's: `cut`, the build of the programs with the cut rule). Null = nothing new */
export function laterRecord(s: BootLike, have: readonly string[] = [], more: Record<string, unknown> = {}): Record<string, unknown> | null {
  if (BREAK) return null;
  const o: Record<string, unknown> = lateMarks(s.marks, have), live = s.marks.interactive;
  if (live !== undefined) { if (typeof o.later === 'number') o.after = r1(o.later - live); if (typeof o.settled === 'number') o.settledAfter = r1(o.settled - live); }
  for (const k of Object.keys(more)) if (more[k] !== undefined && more[k] !== null && !have.includes(k) && !(k in o)) o[k] = more[k];
  return Object.keys(o).length ? o : null;
}
