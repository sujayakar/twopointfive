// THE TABLE OF LEVELS: one page, a row a level of the mission list, built from files (the levels goal, 11-03: "we'll want to dress up all the levels and check
// their budget"). Nothing is measured here: this reads what the other tools and the tasks that dress levels wrote into ONE folder, and writes <dir>/read-first.md.
//   <dir>/levels/<level>/row.json     a dressing task's row for its level: { level, part, dressed, address, hour, what_changed[], left[], needs_his_eye[],
//                                     counts: { before, after }, library: { used[], added[], wanted[] }, branch, commits[], checks[] }
//                                     (counts: `level-audit.ts --counts <level>` prints the block)
//   <dir>/levels/<level>/before.png, after.png     its stills (linked, not read)
//   <dir>/audit/<level>.json          tools/qa/level-audit.ts --out <dir>/audit: the level as it stands on the tree the audit ran on. It is the "after" of the
//                                     level's counts wherever the folder has it; a row's own `counts.after` is read only for a level with no audit file
//   tools/qa/fixtures/level-audit-base.json        every level before it was dressed: the "before" of a level whose row has none
//   <dir>/cost/<level>/cost.json      tools/qa/level-cost-page.cjs --out <dir>/cost: one boot on the software adapter (a relative read), look.png, and with
//                                     --plain-page the same mission without the look, plain-page.png
//   <dir>/mac/*.json                  the file the page's own run hands back on a real GPU (`?look&pricelevels`, ui/pricerun.ts): the newest one is read,
//                                     by the time the run itself wrote in it. Each row carries the run's own verdict (`over`): this page prints it and
//                                     judges no GPU number itself
//   <dir>/cloud/*.json                the same run's file from a rented GPU of another make (the cloud GPU goal's box): the newest one is read, and
//                                     <dir>/cloud/note.txt is the sentence that says what card and what tree it ran on, <dir>/cloud/reading.txt a paragraph printed under the ranking, and <dir>/cloud/trials-off/*.json the
//                                     same levels' look pages with the look's geometry trials off (a look-only run), where there is one. Shown as its own table, RELATIVE
//                                     ONLY: no verdict of its rows is printed and nothing of it is set against the budget, which is the Mac's
//   <dir>/cloud/before/*.json         optional: the run the newest one takes the place of (the same card, an earlier tree), with <dir>/cloud/before/note.txt the
//                                     sentence on what it stood on. Its look page and what the look added are printed beside the new ones, a level a line, and a
//                                     level it priced that the new run did not keeps its line with the new columns empty
//   <dir>/budget.txt                  optional: the budget as it stands, in the folder's own words: its first line is the sentence on who has ruled on what,
//                                     each further line one row of it. Without the file the page says the rows as they were first proposed
//   <dir>/parts.json                  optional: { "<task's name>": ["level", …] }, for the levels that have no row yet
//   <dir>/notice.txt                  optional: what a reader must know before any number here, in the folder's own words. Its first line is printed first on the
//                                     page; each further line `<level>: <mark>` puts that mark beside the level's name in the rented card's table (a price that
//                                     no longer stands for the level as it is: say why, in two or three words)
//
//   bun run tools/qa/level-table.ts --dir <dir>            writes <dir>/read-first.md, prints what it found and what is missing
//   bun run tools/qa/level-table.ts --dir <dir> --check    … and exits 1 when a level of the list has nothing in the folder: no row, no audit, no page read
//   --open "<one line>"  the one line that serves the tree the levels' own addresses open on (the dressed levels' preview branch), said above the table
//   --tree "<a sentence>"  what tree that line serves, said under it (a mend picked onto the branch for the run)
//   --run "<one line>"  the one line the page gives him to get the pricing run going (or --branch <name>: the plain git line for that branch)
// It also writes <dir>/read-first.html: the same page as one HTML file to publish (its stills are linked by the same relative paths).
// THE BUDGET (its standing is the folder's to say, budget.txt; first proposed 11-02): on High the GPU's time with the look within 10% of look off, sixty frames a second held, shaders
// compiled once. Only a GPU answers it. Until a file from one is in <dir>/mac/, a level is called out from what can be counted (level-audit.ts's bars) and from the
// software read, and the page says so beside every such line.
import { readFileSync, writeFileSync, existsSync, readdirSync } from 'node:fs';
import { resolve, dirname } from 'node:path';

const HERE = dirname(new URL(import.meta.url).pathname), ROOT_DIR = resolve(HERE, '../..');
const argv = process.argv.slice(2), valueOf = (w: string) => { const i = argv.indexOf(w); return i >= 0 ? argv[i + 1] : undefined; };
const DIR = valueOf('--dir'); if (!DIR) { console.error('level-table: --dir <folder> (the folder with levels/, audit/, cost/, mac/)'); process.exit(2); }
/** the ONE LINE that serves the tree the levels' own addresses open on, where that is another branch than the run's (the dressed levels' preview): said once above the table */
const OPEN = valueOf('--open') ?? '';
/** what tree that line serves, where it is not plainly a branch's own (a mend picked on for the run): one sentence, said under the line */
const TREE = valueOf('--tree') ?? '';
/** the ONE LINE that gets the pricing run onto his machine (he asked for a one-liner whenever he is to run something): handed in, never written here */
const RUN = valueOf('--run') ?? `git fetch origin && git checkout ${valueOf('--branch') ?? '<the branch that has the run>'} && git pull && bun run dev`;
const D = resolve(DIR), json = (p: string): any => { try { return JSON.parse(readFileSync(p, 'utf8')); } catch { return null; } };
const { listedLevels } = await import(`${ROOT_DIR}/src/scene/scenarioList.ts`), { scenarioQuery } = await import(`${ROOT_DIR}/src/scene/scenarios.ts`), { PRESENT_PASSES } = await import(`${ROOT_DIR}/src/gpu/timer.ts`), { PRICE } = await import(`${ROOT_DIR}/src/ui/pricerun.ts`);   // (… and the pricing run's own second for its running read: the words above its address say the run as it is)
const { AUDIT_BARS } = await import('./level-count.ts');   // (the budget check's bars: level-audit.ts --check holds a level to them, this page says them)

type Slim = { boxes: number; moving: number; drawn: number; planes: number; round: number; bites: number; lampsOn: number; lamps: number; panels: number; glowing: number; openings: number; props: number; none: number };
// (scene/scenarioList.ts listedLevels: the one answer to which levels the list draws; the pricing run's rows are joined to these by the same names)
const levels: { level: string; title: string; hour: string; query: string; missions: string[] }[] = (listedLevels() as { level: string; rows: any[] }[]).map(({ level, rows }) => ({ level, title: rows[0].title, hour: rows[0].hour ?? 'night', query: scenarioQuery(rows[0]), missions: rows.map(e => e.title) }));
const base: Record<string, Slim> = json(`${HERE}/fixtures/level-audit-base.json`)?.levels ?? {};
const parts: Record<string, string[]> = json(`${D}/parts.json`) ?? {};
const slimOf = (r: any): Slim | null => r && r.boxes && r.picture ? { boxes: r.boxes.all, moving: r.boxes.moving, drawn: r.boxes.drawn, planes: r.boxes.planes, round: r.shapes.rounds + r.shapes.pass, bites: r.shapes.walls, lampsOn: r.lamps.on, lamps: r.lamps.all,
  panels: r.lamps.panels, glowing: r.glowing, openings: r.openings.found, props: r.props, none: r.picture.none } : null;
// THE FILES A GPU HANDED BACK, newest first by the time the run wrote in them (a copy or a checkout gives every file the same age on disk). A LEVEL'S ROW IS
// ITS NEWEST among the files of one kind with the newest: the same adapter, quality, look and kind of run (with the page without the look, or without).
// So a later run of three levels mends those three rows and leaves the rest of the full run standing, and a run of another kind takes none of its place
const gpuFiles = (dir: string): { f: string; j: any }[] => existsSync(dir) ? readdirSync(dir).filter(f => f.endsWith('.json')).map(f => ({ f, j: json(`${dir}/${f}`) })).filter(x => x.j && Array.isArray(x.j.rows)).sort((a, b) => String(b.j.at ?? b.f).localeCompare(String(a.j.at ?? a.f))) : [];
const kindOf = (j: any): string => [j.version, j.commit ?? '', j.gpu, j.quality, asWords(j.look), (j.rows as any[]).some(r => r.page) ? 'two pages' : 'look page only'].join(' | ');
const asWords = (a: unknown): string => Array.isArray(a) ? a.join('&') : String(a ?? '');
/** the files of the newest one's kind, and each level's newest row among them */
function newestRows(files: { f: string; j: any }[]): { files: { f: string; j: any }[]; rows: any[] } {
  const same = files.length ? files.filter(x => kindOf(x.j) === kindOf(files[0].j)) : [], rows: any[] = [];
  // (A ROW KEEPS THE SECOND ITS OWN FILE RAN THE WORLD TO, `ranTo`: a later run of held pictures only still mends a level's held read, and a running read
  //  taken after another second than the newest running file's is not shown beside it — liveS)
  // … AND A LEVEL'S RUNNING READ IS ITS NEWEST TOO: where the level's newest row has none (a later run of held pictures only) the newest older row's
  // running read, with its own verdict and reasons, stays on the level's line beside the newer held one, so it does not vanish unsaid
  const at = new Map<string, number>();
  for (const x of same) for (const r of x.j.rows as any[]) { const ranTo = x.j.bar?.runS ?? 0, k = at.get(r.level);
    if (k === undefined) { at.set(r.level, rows.length); rows.push({ ...r, ranTo }); }
    else if (!rows[k].run && r.run) rows[k] = { ...rows[k], run: r.run, ...(rows[k].page ? { page: { ...rows[k].page, ...(r.page?.run ? { run: r.page.run } : {}) } } : {}), runApart: r.runApart, overRun: r.overRun, ranTo, runFrom: x.f }; }
  return { files: same, rows };
}
/** the second the newest file with a running read ran its world to (0: none of the set has one), and a row's running read where it was taken after that second */
const liveS = (set: { files: { f: string; j: any }[] }): number => set.files.find(x => x.j.bar?.runS > 0)?.j.bar.runS ?? 0;
const liveOf = (r: any, s: number): boolean => !!r?.run && s > 0 && r.ranTo === s;
const macDir = `${D}/mac`, macFiles = gpuFiles(macDir), macSet = newestRows(macFiles);
const mac: any = macFiles[0]?.j ?? null, macRow = (level: string) => macSet.rows.find((r: any) => r.level === level) ?? null;
/** a file from before the run wrote its own verdicts: its numbers are shown, and no level is called over or under on them */
/** the Mac's files read their pages with the world running too (ui/pricerun.ts `run`, from 11-05): the second it ran to, and whether any row has such a read */
const macLiveS = liveS(macSet), macRun = macSet.rows.some((r: any) => liveOf(r, macLiveS));
/** the Mac's newest file is from before the run's verdicts can be read as they are now */
const macOld = !!mac && (!(mac.rows as any[]).every(r => Array.isArray(r.over)) || !(mac.version >= 6));   // (… or from before the run it is read as: version 6 plays every page by steps after the page's warm-up, times the frames the page drew and judges no lag. A version 4 or 5 file's frame time and verdict mean other things, and a version 3 file's second span is every stamped pass, which holds the display's wait under Metal)
/** the budget as the folder says it (budget.txt: a sentence on who has ruled on what, then a row a line), else the rows as first proposed */
const BUDGET: string[] = (existsSync(`${D}/budget.txt`) ? readFileSync(`${D}/budget.txt`, 'utf8').split('\n').map(l => l.trim()).filter(Boolean) : []);
if (BUDGET.length < 2) BUDGET.splice(0, BUDGET.length, 'The budget is a proposal: nobody has ruled on its numbers.', 'An M4 Max holds 60 on High, with GPU time within 10 % of look off.', 'An RTX 4080 at 4K stays locked at 60. An M1 Max stays under 16.6 ms. The AMD box shows no new failure.', 'Shaders compile once. A layer over 0.5 ms gets a tier row.');
/** the folder's notice (notice.txt): the line a reader meets first, and a mark a level */
const NOTICE_LINES: string[] = existsSync(`${D}/notice.txt`) ? readFileSync(`${D}/notice.txt`, 'utf8').split('\n').map(l => l.trim()).filter(Boolean) : [];
const NOTICE = NOTICE_LINES[0] ?? '', MARKS = new Map(NOTICE_LINES.slice(1).flatMap(l => { const i = l.indexOf(':'); return i > 0 ? [[l.slice(0, i).trim(), l.slice(i + 1).trim()] as [string, string]] : []; }));
/** a field a row may write as a list or as one line */
const asList = (a: unknown): string[] => Array.isArray(a) ? a.map(String) : typeof a === 'string' && a.trim() ? [a] : [];
/** a level is dressed when its row says so outright */
const isDressed = (row: any): boolean => row?.dressed === true;

const pct = (v: number | null | undefined, d = 1) => v === null || v === undefined || !isFinite(v) ? '—' : `${(100 * v).toFixed(v >= 0.0995 ? 0 : d)}%`;
const more = (v: number | null | undefined) => v === null || v === undefined || !isFinite(v) ? '—' : `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`;
const arrow = (a: number | undefined, b: number | undefined) => a === undefined ? (b === undefined ? '—' : String(b)) : b === undefined || a === b ? String(a) : `${a} → ${b}`;
// … and the newest file from a rented card, with the sentence that says what it ran on
const cloudDir = `${D}/cloud`, cloudFiles = gpuFiles(cloudDir), cloudSet = newestRows(cloudFiles), cloudLiveS = liveS(cloudSet);
const cloudOffSet = newestRows(gpuFiles(`${cloudDir}/trials-off`)), cloudOff: any = cloudOffSet.files[0]?.j ?? null;
/** the run the newest one takes the place of, where the folder keeps it (cloud/before/): read the same way, shown beside, never merged into the new rows */
const cloudWasSet = newestRows(gpuFiles(`${cloudDir}/before`)), cloudWas: any = cloudWasSet.files[0]?.j ?? null, cloudWasNote = existsSync(`${cloudDir}/before/note.txt`) ? readFileSync(`${cloudDir}/before/note.txt`, 'utf8').trim().replace(/\s*\n\s*/g, ' ') : '';
const cloud: any = cloudFiles[0]?.j ?? null, cloudNote = existsSync(`${cloudDir}/note.txt`) ? readFileSync(`${cloudDir}/note.txt`, 'utf8').trim() : '',
  /** what the folder reads from the ranking, in its own words (reading.txt: one paragraph, printed under the table; the tool reads nothing into the numbers itself) */ cloudReading = existsSync(`${cloudDir}/reading.txt`) ? readFileSync(`${cloudDir}/reading.txt`, 'utf8').trim().replace(/\s*\n\s*/g, ' ') : '';
/** a cloud row as this page shows it: the page without the look and the look page as it opens, over the file's second span (version 4: the compute passes and the
 *  g-buffer; version 3: every stamped pass, the same thing to a tenth of a millisecond on a card with no display), the two passes the look moves most, and whether
 *  the GPU kept up. Ranked by what the look adds in ms; the three dearest are marked. No verdict. */
const ms2 = (v: unknown) => typeof v === 'number' ? v.toFixed(2) : '—';
/** the passes a state's time rose most in over another's, in words: "direct +0.36, volumetrics +0.31" (the two largest rises of 0.1 ms or more; the cascades as one) */
const roseIn = (a: any, b: any): string => { const A = a?.passes ?? {}, B = b?.passes ?? {}, d = new Map<string, number>();
  for (const k of new Set([...Object.keys(A), ...Object.keys(B)])) { if ((PRESENT_PASSES as string[]).includes(k)) continue; /* (the passes that draw to the display: their stamps hold its wait, gpu/timer.ts) */ const name = k.startsWith('rc ') ? 'the cascades' : k === 'gbuffer' ? 'the g-buffer' : k === 'compS' ? 'the composite' : k === 'sharpU' ? 'the sharp upscale' : k === 'gatherLight' ? 'the gather\'s light' : k === 'fogofwar' ? 'the fog of war' : k; d.set(name, (d.get(name) ?? 0) + ((B[k] ?? 0) - (A[k] ?? 0))); }
  return [...d].filter(([, v]) => v >= 0.1).sort((x, y) => y[1] - x[1]).slice(0, 2).map(([k, v]) => `${k} +${v.toFixed(2)}`).join(', ') || '—'; };
/** a row the run itself set apart (its clock stood, its two pages on different steps or sizes, or the GPU did not keep step with it): shown apart, never ranked */
const notOnePicture = (r: any): boolean => Array.isArray(r?.apart) ? r.apart.length > 0   // (the run's own word, from its version 5; an older file says it only in its notes)
  : asList(r?.notes).some(n => /clock stood at|not the same step|not the same size|moved during the turns/.test(n));
const cloudBad = (cloudSet.rows as any[]).filter(notOnePicture).map(r => r.level as string);
type Was = { look: number; adds: number; share: number | null };
/** a level's line in the run before: its look page and what the look added then (a row that was not one picture then has none) */
const wasOf = (level: string): Was | null => { const r = (cloudWasSet.rows as any[]).find(x => x.level === level && x.page?.win && x.on && !notOnePicture(x)), a = r?.page.win.all, b = r?.on.all;
  return typeof a === 'number' && typeof b === 'number' ? { look: b, adds: b - a, share: a ? (b - a) / a : null } : null; };
const cloudRows = (cloudSet.rows as any[]).filter(r => r.page?.win && r.on && !notOnePicture(r)).map(r => { const a = r.page.win, b = r.on, adds = typeof a.all === 'number' && typeof b.all === 'number' ? b.all - a.all : null;
  const t = (cloudOffSet.rows as any[]).find(x => x.level === r.level && !notOnePicture(x))?.on ?? null, bare = t && typeof t.all === 'number' ? t.all as number : null;
  // THE WORLD RUNNING (ui/pricerun.ts `run`, from 11-05): the same two pages read again with the world stepping. A running read the run set apart is not shown
  const shown = liveOf(r, cloudLiveS) && !asList(r.runApart).length, live = shown ? r.run.win : null, livePlain = shown ? r.page?.run?.win ?? null : null;
  const runLook = typeof live?.all === 'number' ? live.all as number : null, runPlain = typeof livePlain?.all === 'number' ? livePlain.all as number : null, runAdds = runLook !== null && runPlain !== null ? runLook - runPlain : null;
  return { level: r.level as string, plain: a.all as number | null, look: b.all as number | null, bare, adds, share: adds !== null && a.all ? adds / a.all : null, bareShare: bare !== null && a.all ? (bare - a.all) / a.all : null,
    layers: typeof r.off?.all === 'number' && typeof b.all === 'number' ? b.all - r.off.all : null, where: roseIn(a, b), trials: t ? roseIn(t, b) : '', bySwitch: (r.programs?.bySwitch ?? 0) as number | null, notes: asList(r.notes), was: wasOf(r.level as string),
    runLook, runAdds, runShare: runAdds !== null && runPlain ? runAdds / runPlain : null, world: runLook !== null && typeof b.all === 'number' ? runLook - b.all : null, worldWhere: live ? roseIn(b, live) : '—' }; })
  .sort((x, y) => (y.adds ?? -1e9) - (x.adds ?? -1e9));
/** the newest run read its pages with the world running too (a row of the set has such a read, judged or set apart) · the levels whose running read the run
 *  itself set apart, each with the run's own reasons (printed under the table as the run wrote them: this page reads nothing into them) */
const cloudRun = (cloudSet.rows as any[]).some(r => liveOf(r, cloudLiveS)), cloudRunBad = (cloudSet.rows as any[]).filter(r => liveOf(r, cloudLiveS) && asList(r.runApart).length && !notOnePicture(r)).map(r => ({ level: r.level as string, why: asList(r.runApart) }));
/** ONE ORDER, ONE QUANTITY: by what the look adds with the world running where EVERY ranked level has that read (nearer what is played), else by what it adds held */
const cloudByRun = cloudRun && cloudRows.length > 0 && cloudRows.every(r => r.runAdds !== null);
if (cloudByRun) cloudRows.sort((x, y) => (y.runAdds ?? -1e9) - (x.runAdds ?? -1e9));
/** … and the lines the table prints: the newest run's, then each level only the run before priced, by what the look added then */
const cloudShown: typeof cloudRows = [...cloudRows, ...(cloudWasSet.rows as any[]).map(r => r.level as string).filter(l => !cloudRows.some(r => r.level === l) && wasOf(l)).map(l => ({ level: l, plain: null, look: null, bare: null, adds: null, share: null, bareShare: null,
  layers: null, where: '—', trials: '', bySwitch: null, notes: [] as string[], was: wasOf(l), runLook: null, runAdds: null, runShare: null, world: null, worldWhere: '—' })).sort((x, y) => (y.was?.adds ?? 0) - (x.was?.adds ?? 0))];
/** a row of a GPU's file whose level the mission list no longer draws: shown with that said beside its name, never counted as a level of the list nor marked dearest */
const offList = (level: string): boolean => !levels.some(L => L.level === level), cloudOn = cloudRows.filter(r => !offList(r.level));
const markOf = (level: string): string => [MARKS.get(level) ?? '', offList(level) ? 'off the mission list' : ''].filter(Boolean).join('; ');
const cloudTop = new Set(cloudOn.slice(0, 3).map(r => r.level));
const cloudLine = cloud ? `${cloud.gpu}, ${String(cloud.at).slice(0, 16).replace('T', ' ')} UTC, at quality ${cloud.quality}, ${cloudRows.length} level${cloudRows.length === 1 ? '' : 's'} with both pages timed${cloudSet.files.length > 1 ? `; ${cloudSet.files.length} files of this kind are read, and a level's row is its newest` : ''}` : '';
const CLOUD_LEGEND = `The levels are in order of what the look adds${cloudByRun ? ' with the world running' : cloudRun ? ' on the held picture (not every level has a running read to rank by)' : ''}, and the three dearest are marked. The times are the GPU's busy time over ${cloud?.version >= 4 ? 'the compute passes and the g-buffer' : 'every pass the browser stamps'}, at High with the quality ladder off.${cloudOff ? ' "Trials off" is the look page with its geometry trials off (ground that rays can see, round and cut pieces, the pieces placed by hand).' : ''} "Of which its layers" is what the look's own switchable layers add on the look page. "Where it rises" names the two passes whose time rises most from the page without the look${cloudOff ? ', and "where the trials are" the two that rise most from trials off to on' : ''}. "Programs made by the switch" should be 0.${cloudWas ? ` "Before" is the run this one takes the place of${cloudWasNote ? `: ${cloudWasNote.replace(/\.$/, '')}` : ''}. A level with only its "before" was not priced again.` : ''}${cloudShown.some(r => offList(r.level)) ? ' A level marked "off the mission list" was priced while the list drew it, and is no longer one of its levels.' : ''}${cloudRun ? ` "World running" is the same two pages read again after the world has run to ${cloudLiveS} seconds of the game's clock and is still stepping: falling snow or rain, a fire's smoke and the solver that moves it, the wind. A held picture steps none of these. "What running adds" is the look page running over the same page held, with the two passes that rise most. Lights in fog are in the held read already. Nobody fires in either read, so a fight's smoke, grenades and charges are above both columns.${cloudRunBad.length ? ` The run set the running read apart on ${cloudRunBad.map(x => x.level).join(', ')}, so only the held one is shown there: its reasons are listed under this.` : ''}` : ''}`;
const CLOUD_WHAT = `These numbers are from a rented NVIDIA card, not from your Mac. They are relative only: they say which levels the look costs most on and in which pass, and nothing here is set against the budget. Each level is one held picture${cloudRun ? ', and the same page again with the world running' : ''}, with the quality ladder off and both pages at one size.`;
/** a row's library block in words: how many pieces it lays, what it added to the library, what it still wants from it */
const lib = (row: any): string => { const L = row?.library; if (!L) return '—';
  const used = asList(L.used), added = asList(L.added), wanted = asList(L.wanted); return [used.length ? `uses ${used.length} piece${used.length === 1 ? '' : 's'}` : '', added.length ? `added: ${added.join(', ')}` : '', wanted.length ? `wants: ${wanted.join(', ')}` : ''].filter(Boolean).join('; ') || '—'; };
/** the address that opens a level, printed once whole: a row may write its address with the host, with a scheme too, or as the query alone */
const addr = (r: { row: any; L: { query: string } }): string => `localhost:5173/${String(r.row?.address ?? `?look&${r.L.query.slice(1)}`).trim().replace(/^(https?:\/\/)?localhost(:\d+)?\/?/, '').replace(/^\//, '')}`;
/** a GPU run's three states for a level, ms: the page without the look, the look page with its layers off, with them on */
const gpuCell = (m: any): string => !m || m.on.gpu === null ? '—' : `${m.page?.win.gpu ?? '—'} → ${m.off.gpu ?? '—'} → ${m.on.gpu}`;
/** … the look against the page without it (only where the run timed that page), the layers on against off, and the surface pass's own stamp, off → on */
const vsPage = (m: any): string => m?.page ? more(m.gpuOverPage) : '—', vsOff = (m: any): string => m ? more(m.gpuMore) : '—';
const three = (m: any, k: 'gpu' | 'all'): string => !m || m.on?.[k] === null || m.on?.[k] === undefined ? '—' : `${m.page?.win?.[k] ?? '—'} → ${m.off?.[k] ?? '—'} → ${m.on[k]}`;
const allVs = (m: any): string => !m || (m.page ? m.allOverPage : m.allMore) === undefined ? '—' : more(m.page ? m.allOverPage : m.allMore);
/** THE WORLD RUNNING on the Mac's row (ui/pricerun.ts `run`): the compute passes with the g-buffer, the page without the look → the look page, and what the look adds between them */
const liveCell = (m: any): string => { const a = m?.page?.run?.win?.all, b = m?.run?.win?.all; return !liveOf(m, macLiveS) || typeof b !== 'number' ? '—' : `${typeof a === 'number' ? a : '—'} → ${b}`; };   // (two numbers: the run judges, this page works out no share of its own)
const surface = (m: any): string => { const a = m?.off?.passes?.gbuffer, b = m?.on?.passes?.gbuffer; return typeof a === 'number' && typeof b === 'number' ? `${a.toFixed(2)} → ${b.toFixed(2)}` : '—'; };
const cell = (s: string) => s.replace(/\|/g, '/').replace(/\n/g, ' ');
const passCost = json(`${D}/cost/pass/cost.json`)?.look?.frameMs?.onOverOff as number | undefined;
const heaviest = (k: keyof Slim) => Object.entries(base).filter(([lv]) => levels.some(L => L.level === lv)).reduce((m, [lv, b]) => (b[k] as number) > m[1] ? [lv, b[k] as number] as [string, number] : m, ['', 0] as [string, number]);

const rows = levels.map(L => {
  const row = json(`${D}/levels/${L.level}/row.json`), audit = slimOf(json(`${D}/audit/${L.level}.json`)), cost = json(`${D}/cost/${L.level}/cost.json`), m = macRow(L.level);
  const before: Slim | undefined = row?.counts?.before ?? base[L.level], after: Slim | undefined = audit ?? row?.counts?.after ?? undefined;   // (the folder's audit is the level AS IT STANDS on the tree it ran on: a row's own "after" is what its dresser counted on the day, before the vehicles, the materials and the library's pieces moved it)
  const part = row?.part ?? Object.entries(parts).find(([, ls]) => ls.includes(L.level))?.[0] ?? '—';
  const still = (f: string) => existsSync(`${D}/levels/${L.level}/${f}`) ? `levels/${L.level}/${f}` : null;
  // ---- over the proposed budget, and why
  const over: string[] = [], forGpu: string[] = [];
  if (m && !macOld) over.push(...asList(m.over));   // (the run's own verdict on its row, in its words: ui/pricerun.ts priceOver is the one place a GPU number is judged)
  // (… and its verdict on THE RUNNING READ, kept apart: the bar was ruled on, a running read never was, so a level over on that read alone is not counted
  //  "over a bar"; and a running read the run did not judge, where the held one was)
  const live = !!m && !macOld && liveOf(m, macLiveS) && !notOnePicture(m), overRun: string[] = live ? asList(m.overRun) : [], runApart: string[] = live ? asList(m.runApart) : [];
  const apart: string[] = m && !macOld && notOnePicture(m) ? (asList(m.apart).length ? asList(m.apart) : asList(m.notes)) : [];   // (… and a row the run did not judge, because its pages were not one picture: said, never read as "under")
  if (cost?.look?.compiled?.bySwitch && (cost.look.compiled.bySwitch.pipelines > 0 || cost.look.compiled.bySwitch.modules > 0)) over.push(`switching the layers compiled ${cost.look.compiled.bySwitch.pipelines} more programs on the software page (the bar is none)`);
  const now = after ?? before;
  if (now) { for (const [what, k] of [['boxes drawn', 'drawn'], ['boxes that move', 'moving'], ['cut planes', 'planes'], ['lamps burning', 'lampsOn'], ['boxes that glow', 'glowing']] as [string, keyof Slim][]) {
      const [top, n] = heaviest(k); if ((now[k] as number) > AUDIT_BARS.overHeaviest * Math.max(1, n)) over.push(`${what}: ${now[k]}, over ${AUDIT_BARS.overHeaviest} × the heaviest level before dressing (${top}, ${n})`);
      else if (base.pass && L.level !== 'pass' && (now[k] as number) > AUDIT_BARS.overPass * Math.max(1, base.pass[k] as number)) forGpu.push(`${what} ${now[k]} (the pass ${base.pass[k]})`); }
    if (now.none > AUDIT_BARS.bare) over.push(`${pct(now.none)} of the picture has no material (the bar is ${pct(AUDIT_BARS.bare)})`); }
  const ratio = cost?.look?.frameMs?.onOverOff as number | undefined;
  if (!m && ratio !== undefined && passCost !== undefined && L.level !== 'pass' && ratio - 1 > 1.25 * (passCost - 1) && ratio - 1 > 0.05) forGpu.push(`on the software page the layers add ${more(ratio - 1)}, the pass ${more(passCost - 1)}`);
  return { L, row, audit, part, before, after, cost, m, over, apart, overRun, runApart, forGpu, ratio, stills: { before: still('before.png'), after: still('after.png'), look: existsSync(`${D}/cost/${L.level}/look.png`) ? `cost/${L.level}/look.png` : null, plain: existsSync(`${D}/cost/${L.level}/plain-page.png`) ? `cost/${L.level}/plain-page.png` : null } };
});

// ---- the page
const dressed = rows.filter(r => isDressed(r.row)).length, priced = rows.filter(r => r.m).length, read = rows.filter(r => r.cost?.look?.frameMs?.onOverOff).length, overs = rows.filter(r => r.over.length);
const out: string[] = [];
out.push('# Every level in the new look: dressed or not, and what it costs', '');
out.push(`Built ${new Date().toISOString().slice(0, 16).replace('T', ' ')} UTC by \`tools/qa/level-table.ts\` from the files in this folder. Do not edit it by hand: the next build writes over it.`, '');
if (NOTICE) out.push(`**${NOTICE}**`, '');
out.push(`${dressed} of ${rows.length} levels are dressed. ${priced} of ${rows.length} are priced on your Mac.${cloud ? ` ${cloudOn.length} of ${rows.length} are priced on a rented card, as a relative reading.` : ''} ${read} of ${rows.length} have a software read.`, '');
out.push('## The budget, and who answers it', '');
out.push(BUDGET[0], '', ...BUDGET.slice(1).map(l => `- ${l}`), '');
out.push('Nothing on this page was seen on a GPU unless its line says so. Counts are exact. A software read is a relative read: the same page on a CPU renderer, where a frame takes seconds.', '');
out.push('### Price every level on your Mac', '');
out.push('This is the one run that answers the budget. It is hands off: ' + rows.length * 2 + ' page loads, about a minute each, with the laptop plugged in.', '');
out.push('```', RUN, '```', '');
if (TREE) out.push(TREE, '');
out.push('Then open this and leave the tab in front:', '', '```', 'localhost:5173/?look&pricelevels&downloads=1', '```', '');
out.push('Each level loads twice. The look page plays four seconds, holds its picture still and times it with the look\'s layers off and on in turn. Then it lets the world run on to second ' + PRICE.runS + ' of its clock and times it once more as it runs: falling snow or rain, a fire\'s smoke, the wind, none of which a held picture steps. Then the same mission loads without the look and is timed the same two ways. Nobody fires in either read, so a fight costs more than both.', 'The last page shows the table and saves one file, `price-levels-<date>.json`, to your downloads. Drop that file in the task and this page fills in.', '');
out.push('"Look off" has two meanings and the file has both. The page without the look is the game as it is drawn today, and it is what the budget means. The layers off is the look page with every layer that has a cost put off at once: same shaders, same shapes, same camera.', '');
out.push('- `?look&pricelevels=pass,block,compound&downloads=1` prices only those.', '- Add `&quality=medium` for another preset. The default is High, the one the budget is said for.', '- Add `&pricepage=0` to skip the loads without the look: half the time, and only the layers are priced.', '- Add `&pricelive=0` to skip the read with the world running: about half the time, held pictures only.', '- Esc stops the run.', '');
const macLine = mac ? `from ${mac.gpu}${mac.machine?.hz ? `, a ${mac.machine.hz} Hz display` : ''}, ${String(mac.at).slice(0, 16).replace('T', ' ')} UTC, at quality ${mac.quality}${macSet.files.length > 1 ? `; ${macSet.files.length} files of this kind are read, and a level's row is its newest` : ''}` : '', OLD = 'It is from an older build of the run, whose verdicts were made by rules that have since been mended: its numbers are shown where the columns still exist, and no level is called over or under on them. Run the line again for a file this page can judge by.';
out.push(mac ? `The newest file here: \`mac/${macFiles[0].f}\`, ${macLine}.${macOld ? ' ' + OLD : ''}` : 'No file from a GPU is here yet, so every cost below is a count or a software read.', '');

out.push('## Over a bar', '');
if (!overs.length) out.push('No level is over a bar that can be read here.', '');
else { out.push(mac ? 'Each line says which number puts the level there.' : 'Each line says which count puts the level there. These are the bars a count can answer; the GPU bars wait for your run.', '');
  for (const r of overs) out.push(`- **${r.L.level}**${isDressed(r.row) ? '' : ' (not dressed yet)'}: ${r.over.join('; ')}.`); out.push(''); }
const aparts = rows.filter(r => r.apart.length), APART = 'The run set these levels apart (their pages were not one picture, or the GPU did not keep step with them), so it read no verdict off their numbers. They are neither over nor under: run them again.';
if (aparts.length) { out.push('### Not judged on your Mac', '', APART, ''); for (const r of aparts) out.push(`- **${r.L.level}**: ${r.apart.join('; ')}.`); out.push(''); }
const oversRun = rows.filter(r => r.overRun.length), OVER_RUN = 'The same lines of the bar, read off the pages with the world running. Nobody has ruled on this read, and a level here is not counted over a bar above: it says where a played frame costs more than a held one shows.';
if (oversRun.length) { out.push('### Past the same lines with the world running', '', OVER_RUN, ''); for (const r of oversRun) out.push(`- **${r.L.level}**: ${r.overRun.join('; ')}.`); out.push(''); }
const apartsRun = rows.filter(r => r.runApart.length), APART_RUN = 'On these levels the run judged the held picture and set the read with the world running apart, for the reason beside each. That read is neither over nor under.';
if (apartsRun.length) { out.push('### The running read not judged on your Mac', '', APART_RUN, ''); for (const r of apartsRun) out.push(`- **${r.L.level}**: ${r.runApart.join('; ')}.`); out.push(''); }
const gpus = cloud ? [] : rows.filter(r => r.forGpu.length && !r.m && !r.over.length);   // (a rented card's file has timed every level: the list of levels to time first has done its work)
if (gpus.length) { out.push('### Not over a bar, but wants a GPU read first', '', 'The look was timed on one level, the pass. These ask more of a frame than the pass does.', '');
  for (const r of gpus) out.push(`- **${r.L.level}**: ${r.forGpu.join('; ')}.`); out.push(''); }

const HEAVY: [string, keyof Slim][] = [['boxes drawn', 'drawn'], ['boxes that move', 'moving'], ['cut planes', 'planes'], ['lamps burning', 'lampsOn'], ['boxes that glow', 'glowing']];
const nowOf = (r: typeof rows[number]) => r.after ?? r.before, heavy = HEAVY.map(([what, k]) => { const top = rows.filter(r => nowOf(r)).sort((a, b) => (nowOf(b)![k] as number) - (nowOf(a)![k] as number))[0]; return { what, level: top?.L.level ?? '—', n: top ? nowOf(top)![k] as number : 0, pass: (nowOf(rows.find(r => r.L.level === 'pass')!)?.[k] ?? 0) as number }; });
out.push('### The heaviest level in each count', '', 'The pass is the level the look was timed on, so its own count is beside each.', '');
for (const h of heavy) out.push(`- ${h.what}: **${h.level}** with ${h.n} (the pass has ${h.pass}).`); out.push('');
out.push('## Where each level stands', '');
if (OPEN) out.push('The addresses in this table open on the tree this one line serves:', '', '```', OPEN, '```', '');
out.push('| Level | Task | Dressed | Stills | Open it | From the library | What is left | Needs your eye on a GPU |', '|---|---|---|---|---|---|---|---|');
for (const r of rows) { const s = r.stills, pics = [s.before ? `[before](${s.before})` : '', s.after ? `[after](${s.after})` : '', !s.after && s.look ? `[look, software](${s.look})` : '', !s.before && s.plain ? `[without the look, software](${s.plain})` : ''].filter(Boolean).join(' · ') || '—';
  out.push(`| ${r.L.level} | ${cell(r.part)} | ${r.row ? (isDressed(r.row) ? 'yes' : 'not yet') : 'no row yet'} | ${pics} | \`${cell(addr(r))}\` | ${cell(lib(r.row))} | ${cell(asList(r.row?.left).join('; ') || (r.row ? '—' : 'not started'))} | ${cell(asList(r.row?.needs_his_eye).join('; ') || '—')} |`); }
out.push('', 'The hour is the one the mission opens at: ' + [...new Set(rows.map(r => r.L.hour))].map(h => `${h} (${rows.filter(r => r.L.hour === h).length})`).join(', ') + '.', '');

out.push('## What each level costs', '');
out.push('Counts are before → after dressing, as the look page draws the level at the hour its mission opens at. One number means it has not moved.', '');
out.push(`| Level | Boxes drawn | Boxes that move | Cut planes | Round things | Lamps burning | No material | Software read, layers on | GPU ms: no look → layers off → on | Look against no look | Layers on against off | With the g-buffer ms: no look → layers off → on | With the g-buffer, look against no look |${macRun ? ' World running, with the g-buffer ms: no look → on |' : ''} Surface pass ms: off → on | A frame every | Programs made by the switch |`, `|---|---|---|---|---|---|---|---|---|---|---|---|---|${macRun ? '---|' : ''}---|---|---|`);
for (const r of rows) { const b = r.before, a = r.after, sw = r.m ? r.m.programs?.bySwitch : r.cost?.look?.compiled?.bySwitch?.pipelines;
  out.push(`| ${r.L.level} | ${arrow(b?.drawn, a?.drawn)} | ${arrow(b?.moving, a?.moving)} | ${arrow(b?.planes, a?.planes)} | ${arrow(b?.round, a?.round)} | ${arrow(b?.lampsOn, a?.lampsOn)} | ${b ? pct(b.none) : '—'}${a && b && Math.abs(a.none - b.none) > 0.0005 ? ` → ${pct(a.none)}` : ''} | ${r.ratio !== undefined ? more(r.ratio - 1) : '—'} | ${gpuCell(r.m)} | ${vsPage(r.m)} | ${vsOff(r.m)} | ${three(r.m, 'all')} | ${allVs(r.m)} |${macRun ? ` ${liveCell(r.m)} |` : ''} ${surface(r.m)} | ${r.m ? `${r.m.on.ms.toFixed(1)} ms` : '—'} | ${sw === undefined ? '—' : sw} |`); }
out.push('');
out.push('- **Software read, layers on** is how much longer a frame takes with the look\'s layers on than off, on the same page in the same boot, on a CPU renderer. It is a direction, not a price.', '- **GPU** columns are from your run. **GPU ms** is the time the GPU is busy with the compute passes, which is how the game times itself everywhere. The look\'s surface layers (paint, relief, finish, weather, marks) are drawn in a render pass that this number leaves out, so **With the g-buffer** adds the g-buffer\'s own pass to it, which is where those layers are shaded, and that is judged against the same 10%. **Surface pass ms** shows that one pass by itself. **A frame every** is the time between two frames: on a display the game holds, it is the display\'s own rate.', '- **Programs made by the switch** should be 0: the look must not compile anything when its layers change.', '- **No material** is the share of the picture from above that the look draws flat.', '');
if (cloud) { out.push('## On a rented NVIDIA card (relative)', '', CLOUD_WHAT, '', `The file: \`cloud/${cloudFiles[0].f}\`, ${cloudLine}.${cloudNote ? ' ' + cloudNote : ''}`, '');
  const sgn = (v: number | null, pc: number | null) => v === null ? '—' : `${v >= 0 ? '+' : ''}${v.toFixed(2)} ms (${more(pc)})`;
  const liveMd = (r: typeof cloudRows[number]) => cloudRun ? ` ${ms2(r.runLook)} | ${sgn(r.runAdds, r.runShare)} | ${r.world === null ? '—' : `${r.world >= 0 ? '+' : ''}${r.world.toFixed(2)} ms${r.worldWhere !== '—' ? ` (${r.worldWhere})` : ''}`} |` : '';
  out.push(`| Level | Without the look, ms |${cloudOff ? ' Look, trials off, ms |' : ''} With the look, ms | The look adds |${cloudRun ? ' World running: with the look, ms | World running: the look adds | What running adds |' : ''}${cloudWas ? ' Before: with the look, ms | Before: the look added |' : ''}${cloudOff ? ' With the trials off |' : ''} Of which its layers | Where it rises | ${cloudOff ? 'Where the trials are | ' : ''}Programs made by the switch |`, `|---|---|${cloudOff ? '---|' : ''}---|---|${cloudRun ? '---|---|---|' : ''}${cloudWas ? '---|---|' : ''}${cloudOff ? '---|' : ''}---|---|${cloudOff ? '---|' : ''}---|`);
  for (const r of cloudShown) out.push(`| ${cloudTop.has(r.level) ? `**${r.level}**` : r.level}${markOf(r.level) ? ` (${markOf(r.level)})` : ''} | ${ms2(r.plain)} |${cloudOff ? ` ${ms2(r.bare)} |` : ''} ${ms2(r.look)} | ${sgn(r.adds, r.share)} |${liveMd(r)}${cloudWas ? ` ${ms2(r.was?.look)} | ${sgn(r.was?.adds ?? null, r.was?.share ?? null)} |` : ''}${cloudOff ? ` ${sgn(r.bare !== null && r.plain !== null ? r.bare - r.plain : null, r.bareShare)} |` : ''} ${r.layers === null ? '—' : `${r.layers.toFixed(2)} ms`} | ${r.where} | ${cloudOff ? `${r.trials || '—'} | ` : ''}${r.bySwitch ?? '—'} |`);
  if (cloudReading) out.push('', cloudReading);
  out.push('', CLOUD_LEGEND + (cloudBad.length ? ` Not ranked, because the run set them apart (their two pages were not one picture, or the GPU did not keep step): ${cloudBad.join(', ')}.` : ''), '');
  if (cloudRunBad.length) { out.push('The running read the run set apart, in its own words:', ''); for (const x of cloudRunBad) out.push(`- **${x.level}**: ${x.why.join('; ')}.`); out.push(''); } }
const plainPage = json(`${D}/cost/pass/cost.json`)?.againstPlainPage;
if (plainPage) { out.push('## What each page compiles', '');
  out.push(`On the pass, on the software page: the page without the look compiled ${plainPage.textKB[0]} KB of shader text in ${plainPage.pipelines[0]} pipelines and came ready in ${plainPage.readySecs[0]} s. The look page compiled ${plainPage.textKB[1]} KB in ${plainPage.pipelines[1]} and came ready in ${plainPage.readySecs[1]} s.`, 'The page without the look also compiles a second set of ray programs for cut boxes that swap in, which the look page does not need. The text size and the pipeline count carry over to a GPU; the seconds do not.', ''); }
out.push('## How this page is made', '');
out.push('- `bun run tools/qa/level-audit.ts --out <this folder>/audit` counts every level, headless, in under a minute. `--check <level>` is the budget check a dressed level must pass.', '- `node tools/qa/level-cost-page.cjs <level> --out <this folder>/cost` boots the page once on the software renderer: programs compiled, layers off against on, two stills.', '- `bun run tools/qa/level-table.ts --dir <this folder>` builds this page.', '- A task that dresses a level writes `levels/<level>/row.json`, `before.png` and `after.png`.', '');
writeFileSync(`${D}/read-first.md`, out.join('\n'));

// ---- the same page as one HTML file, to publish (a page he can open): the stills by the same relative paths, published beside it
{ const esc = (s: unknown) => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  const code = (s: string) => `<code>${esc(s)}</code>`, list = (a: string[]) => a.length ? `<ul>${a.map(x => `<li>${x}</li>`).join('')}</ul>` : '';
  const state = (r: typeof rows[number]) => isDressed(r.row) ? '<span class="pill ok">dressed</span>' : r.row ? '<span class="pill warn">in work</span>' : '<span class="pill">not started</span>';
  const pic = (src: string | null, label: string) => src ? `<a class="still" href="${esc(src)}"><img src="${esc(src)}" alt="${esc(label)}" loading="lazy"><span>${esc(label)}</span></a>` : '';
  const stand = rows.map(r => { const s = r.stills, pics = [pic(s.before, 'before'), pic(s.after, 'after'), !s.after ? pic(s.look, 'look on, software') : '', !s.before ? pic(s.plain, 'without the look, software') : ''].join('');
    return `<tr><th scope="row">${esc(r.L.level)}<small>${esc(r.L.missions.join(' · '))} · ${esc(r.L.hour)}</small></th><td>${state(r)}<small>${esc(r.part)}</small></td><td class="stills">${pics || '<span class="none">none yet</span>'}</td><td>${code(addr(r))}</td><td>${esc(lib(r.row))}</td><td>${esc(asList(r.row?.left).join('; ') || (r.row ? '—' : 'not started'))}</td><td>${esc(asList(r.row?.needs_his_eye).join('; ') || '—')}</td></tr>`; }).join('\n');
  const num = (b: number | undefined, a: number | undefined) => `<td class="n">${esc(arrow(b, a))}</td>`;
  const costs = rows.map(r => { const b = r.before, a = r.after, m = r.m, sw = m ? m.programs?.bySwitch : r.cost?.look?.compiled?.bySwitch?.pipelines, flag = r.over.length ? ' class="over"' : '';
    return `<tr${flag}><th scope="row">${esc(r.L.level)}</th>${num(b?.drawn, a?.drawn)}${num(b?.moving, a?.moving)}${num(b?.planes, a?.planes)}${num(b?.round, a?.round)}${num(b?.lampsOn, a?.lampsOn)}<td class="n">${b ? pct(b.none) : '—'}${a && b && Math.abs(a.none - b.none) > 0.0005 ? ` → ${pct(a.none)}` : ''}</td><td class="n soft">${r.ratio !== undefined ? more(r.ratio - 1) : '—'}</td><td class="n gpu">${esc(gpuCell(m))}</td><td class="n gpu">${vsPage(m)}</td><td class="n gpu">${vsOff(m)}</td><td class="n gpu">${esc(three(m, 'all'))}</td><td class="n gpu">${allVs(m)}</td>${macRun ? `<td class="n gpu">${esc(liveCell(m))}</td>` : ''}<td class="n gpu">${esc(surface(m))}</td><td class="n gpu">${m ? `${m.on.ms.toFixed(1)}` : '—'}</td><td class="n">${sw === undefined ? '—' : sw}</td></tr>`; }).join('\n');
  const page = `<title>Levels in the new look</title>
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@500;600&family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;500;600&display=swap">
<style>
/* layout: a field sheet. One column of sections; the two tables scroll sideways in their own frames; numbers in a mono face so columns line up */
:root { --bg: #eef0ea; --panel: #f8f9f5; --fg: #1c2420; --muted: #5d6a61; --line: #cdd3c8; --accent: #b8480f; --ok: #2f6f45; --warn: #9a6a00; --bad: #a3271c; --gpu: #e4ebdf;
  --display: "Barlow Condensed", "Arial Narrow", sans-serif; --body: "IBM Plex Sans", system-ui, sans-serif; --mono: "IBM Plex Mono", ui-monospace, Menlo, monospace; }
@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) { --bg: #131714; --panel: #1b211d; --fg: #dde4dc; --muted: #93a096; --line: #323b34; --accent: #f08a4b; --ok: #6cc08a; --warn: #e0b040; --bad: #f0796c; --gpu: #202a23; color-scheme: dark } }
:root[data-theme="dark"] { --bg: #131714; --panel: #1b211d; --fg: #dde4dc; --muted: #93a096; --line: #323b34; --accent: #f08a4b; --ok: #6cc08a; --warn: #e0b040; --bad: #f0796c; --gpu: #202a23; color-scheme: dark }
body { background: var(--bg); color: var(--fg); font: 15px/1.5 var(--body); }
.sheet { max-width: 1180px; margin: 0 auto; padding-block: 28px 56px; padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); display: flex; flex-direction: column; gap: 30px; }
.sheet h1 { font: 600 40px/1.05 var(--display); letter-spacing: .01em; margin: 0; text-wrap: balance; }
.sheet h2 { font: 600 24px/1.15 var(--display); letter-spacing: .02em; margin: 0 0 10px; text-wrap: balance; }
.sheet h3 { font: 600 15px/1.3 var(--body); margin: 14px 0 6px; }
.sheet p, .sheet ul { margin: 0 0 10px; max-width: 70ch; } .sheet ul { padding-left: 20px; }
.sheet small { display: block; color: var(--muted); font-size: 12px; line-height: 1.35; }
.sheet code { font: 13px var(--mono); background: var(--panel); border: 1px solid var(--line); border-radius: 3px; padding: 1px 5px; overflow-wrap: anywhere; }
.lede { color: var(--muted); max-width: 70ch; }
.sheet .notice { border: 1px solid var(--warn); border-left: 4px solid var(--warn); background: var(--panel); padding: 12px 16px; max-width: 90ch; margin: 0; font-weight: 500; }
.frame tbody th .mark { display: block; font: 500 11px var(--mono); color: var(--warn); white-space: normal; }
.counts { display: flex; flex-wrap: wrap; gap: 8px 28px; font: 500 14px var(--mono); }
.counts b { font: 600 30px/1 var(--display); color: var(--accent); margin-right: 6px; font-variant-numeric: tabular-nums; }
.run { border: 1px solid var(--line); border-left: 4px solid var(--accent); background: var(--panel); padding: 16px 18px; }
.run pre { font: 13px/1.5 var(--mono); margin: 0 0 10px; white-space: pre-wrap; overflow-wrap: anywhere; }
.run .addr { font: 500 16px var(--mono); overflow-wrap: anywhere; }
.frame { overflow-x: auto; border: 1px solid var(--line); background: var(--panel); min-width: 0; }
.frame table { border-collapse: collapse; width: 100%; font-size: 13px; }
.frame th, .frame td { text-align: left; vertical-align: top; padding: 8px 10px; border-bottom: 1px solid var(--line); }
.frame thead th { font: 500 11px var(--mono); text-transform: uppercase; letter-spacing: .06em; color: var(--muted); white-space: nowrap; position: sticky; top: 0; background: var(--panel); }
.frame tbody th { font: 600 14px var(--body); white-space: nowrap; }
.frame td.n { font: 13px var(--mono); font-variant-numeric: tabular-nums; white-space: nowrap; text-align: right; }
.frame thead th.n { text-align: right; } .frame td.gpu, .frame th.gpu { background: var(--gpu); } .frame td.soft { color: var(--muted); } .frame td.was, .frame th.was { color: var(--muted); background: none; }
.frame tr.over th { color: var(--bad); } .frame tr.dear th { color: var(--accent); } .frame td.words, .frame th.words { min-width: 210px; } .frame tr:last-child th, .frame tr:last-child td { border-bottom: 0; }
.stills { min-width: 190px; } .still { display: inline-block; margin: 0 6px 4px 0; color: var(--muted); font-size: 11px; text-decoration: none; }
.still img { display: block; width: 168px; aspect-ratio: 16 / 9; object-fit: cover; border: 1px solid var(--line); }
.none { color: var(--muted); }
.pill { display: inline-block; font: 500 11px var(--mono); text-transform: uppercase; letter-spacing: .05em; padding: 2px 7px; border: 1px solid var(--line); border-radius: 2px; color: var(--muted); }
.pill.ok { color: var(--ok); border-color: var(--ok); } .pill.warn { color: var(--warn); border-color: var(--warn); }
.overs li b { color: var(--bad); } .legend { color: var(--muted); font-size: 13px; }
</style>
<div class="sheet">
<header><h1>Levels in the new look</h1><p class="lede">Every level on the mission list: dressed or not, and what the look costs on it. Built ${esc(new Date().toISOString().slice(0, 16).replace('T', ' '))} UTC from the dressing tasks' rows, the level audit and the page reads.</p>
${NOTICE ? `<p class="notice">${esc(NOTICE)}</p>` : ''}
<div class="counts"><span><b>${dressed}</b>of ${rows.length} dressed</span><span><b>${priced}</b>of ${rows.length} priced on your Mac</span>${cloud ? `<span><b>${cloudOn.length}</b>of ${rows.length} on a rented card</span>` : ''}<span><b>${read}</b>of ${rows.length} with a software read</span><span><b>${overs.length}</b>over a bar</span></div></header>

<section><h2>Price every level on your Mac</h2>
<div class="run"><p>This is the one run that answers the budget. It is hands off: ${rows.length * 2} page loads, about a minute each, with the laptop plugged in.</p>
<pre>${esc(RUN)}</pre>
${TREE ? `<p>${esc(TREE)}</p>` : ''}
<p>Then open this and leave the tab in front:</p>
<pre class="addr">localhost:5173/?look&amp;pricelevels&amp;downloads=1</pre>
<p>Each level loads twice. The look page plays four seconds, holds its picture still and times it with the look's layers off and on in turn. Then it lets the world run on to second ${PRICE.runS} of its clock and times it once more as it runs: falling snow or rain, a fire's smoke, the wind, none of which a held picture steps. Then the same mission loads without the look and is timed the same two ways. Nobody fires in either read, so a fight costs more than both. The last page shows the table and saves one file, ${code('price-levels-<date>.json')}, to your downloads. Drop that file in the task and this page fills in.</p>
<ul><li>${code('?look&pricelevels=pass,block,compound&downloads=1')} prices only those.</li><li>Add ${code('&quality=medium')} for another preset. The default is High, the one the budget is said for.</li><li>Add ${code('&pricelive=0')} to skip the read with the world running: about half the time, held pictures only.</li><li>Add ${code('&pricepage=0')} to skip the loads without the look: half the time, and only the layers are priced.</li><li>Esc stops the run.</li></ul>
<p>"Look off" has two meanings and the file has both. The page without the look is the game as it is drawn today, and it is what the budget means. The layers off is the look page with every layer that has a cost put off at once: same shaders, same shapes, same camera.</p>
<p>${mac ? `The newest file here is ${esc(macLine)}.${macOld ? ' ' + OLD : ''}` : 'No file from a GPU is here yet, so every cost below is a count or a software read.'}</p></div></section>

<section><h2>The budget</h2><p>${esc(BUDGET[0])}</p>
<ul>${BUDGET.slice(1).map(l => `<li>${esc(l)}</li>`).join('')}</ul>
<p>Nothing on this page was seen on a GPU unless its column is shaded. Counts are exact. A software read is the same page on a CPU renderer, where a frame takes seconds: a direction, not a price.</p></section>

<section><h2>Over a bar</h2>
${overs.length ? `<p>${mac ? 'Each line says which number puts the level there.' : 'Each line says which count puts the level there. These are the bars a count can answer. The GPU bars wait for your run.'}</p><ul class="overs">${overs.map(r => `<li><b>${esc(r.L.level)}</b>${isDressed(r.row) ? '' : ' (not dressed yet)'}: ${esc(r.over.join('; '))}.</li>`).join('')}</ul>` : '<p>No level is over a bar that can be read here.</p>'}
${aparts.length ? `<h3>Not judged on your Mac</h3><p>${esc(APART)}</p><ul class="overs">${aparts.map(r => `<li><b>${esc(r.L.level)}</b>: ${esc(r.apart.join('; '))}.</li>`).join('')}</ul>` : ''}
${oversRun.length ? `<h3>Past the same lines with the world running</h3><p>${esc(OVER_RUN)}</p><ul class="overs">${oversRun.map(r => `<li><b>${esc(r.L.level)}</b>: ${esc(r.overRun.join('; '))}.</li>`).join('')}</ul>` : ''}
${apartsRun.length ? `<h3>The running read not judged on your Mac</h3><p>${esc(APART_RUN)}</p><ul class="overs">${apartsRun.map(r => `<li><b>${esc(r.L.level)}</b>: ${esc(r.runApart.join('; '))}.</li>`).join('')}</ul>` : ''}
<h3>The heaviest level in each count</h3><p>The pass is the level the look was timed on, so its own count is beside each.</p>${list(heavy.map(h => `${esc(h.what)}: <b>${esc(h.level)}</b> with ${h.n} (the pass has ${h.pass}).`))}
${gpus.length ? `<h3>Not over a bar, but wants a GPU read first</h3><p>The look was timed on one level, the pass. These ask more of a frame than the pass does.</p>${list(gpus.map(r => `<b>${esc(r.L.level)}</b>: ${esc(r.forGpu.join('; '))}.`))}` : ''}</section>

<section><h2>Where each level stands</h2>
${OPEN ? `<p>The addresses in this table open on the tree this one line serves:</p><div class="run"><pre>${esc(OPEN)}</pre></div>` : ''}
<div class="frame"><table><thead><tr><th>Level</th><th>State</th><th>Stills</th><th>Open it</th><th>From the library</th><th>What is left</th><th>Needs your eye on a GPU</th></tr></thead><tbody>
${stand}
</tbody></table></div></section>

<section><h2>What each level costs</h2>
<p>Counts are before → after dressing, as the look page draws the level at the hour its mission opens at. One number means it has not moved. Shaded columns are from a GPU.</p>
<div class="frame"><table><thead><tr><th>Level</th><th class="n">Boxes drawn</th><th class="n">That move</th><th class="n">Cut planes</th><th class="n">Round things</th><th class="n">Lamps burning</th><th class="n">No material</th><th class="n">Software: layers on</th><th class="n gpu">GPU ms: no look → layers off → on</th><th class="n gpu">Look vs no look</th><th class="n gpu">Layers on vs off</th><th class="n gpu">With the g-buffer ms: no look → layers off → on</th><th class="n gpu">With the g-buffer vs no look</th>${macRun ? '<th class="n gpu">World running, with the g-buffer ms: no look → on</th>' : ''}<th class="n gpu">Surface pass ms: off → on</th><th class="n gpu">A frame every, ms</th><th class="n">Programs by the switch</th></tr></thead><tbody>
${costs}
</tbody></table></div>
<p class="legend">No material is the share of the picture from above that the look draws flat. Software: layers on is how much longer a frame takes with the layers on than off, on a CPU renderer. Programs by the switch should be 0: the look must not compile anything when its layers change. GPU ms is the time the GPU is busy with the compute passes, which is how the game times itself everywhere. The look's surface layers (paint, relief, finish, weather, marks) are drawn in a render pass that this number leaves out, so With the g-buffer adds the g-buffer's own pass to it, which is where those layers are shaded, and that is judged against the same 10%. Surface pass ms shows that one pass by itself. A frame every is the time between two frames: on a display the game holds, it is the display's own rate.</p>
${plainPage ? `<h3>What each page compiles</h3><p>On the pass, on the software page: the page without the look compiled ${plainPage.textKB[0]} KB of shader text in ${plainPage.pipelines[0]} pipelines and came ready in ${plainPage.readySecs[0]} s. The look page compiled ${plainPage.textKB[1]} KB in ${plainPage.pipelines[1]} and came ready in ${plainPage.readySecs[1]} s. The page without the look also compiles a second set of ray programs for cut boxes that swap in, which the look page does not need. The text size and the pipeline count carry over to a GPU; the seconds do not.</p>` : ''}</section>

${cloud ? `<section><h2>On a rented NVIDIA card (relative)</h2><p>${esc(CLOUD_WHAT)}</p><p>The file is ${esc(cloudLine)}.${cloudNote ? ' ' + esc(cloudNote) : ''}</p>
<div class="frame"><table><thead><tr><th>Level</th><th class="n gpu">Without the look, ms</th>${cloudOff ? '<th class="n gpu">Look, trials off, ms</th>' : ''}<th class="n gpu">With the look, ms</th><th class="n gpu">The look adds</th>${cloudRun ? '<th class="n gpu">World running: with the look, ms</th><th class="n gpu">World running: the look adds</th><th class="words">What running adds</th>' : ''}${cloudWas ? '<th class="n gpu was">Before: with the look, ms</th><th class="n gpu was">Before: the look added</th>' : ''}${cloudOff ? '<th class="n gpu">With the trials off</th>' : ''}<th class="n gpu">Of which its layers</th><th class="words">Where it rises</th>${cloudOff ? '<th class="words">Where the trials are</th>' : ''}<th class="n">Programs made by the switch</th></tr></thead><tbody>
${cloudShown.map(r => { const sgn = (v: number | null, pc: number | null) => v === null ? '—' : `${v >= 0 ? '+' : ''}${v.toFixed(2)} ms (${esc(more(pc))})`;
  return `<tr${cloudTop.has(r.level) ? ' class="dear"' : ''}><th scope="row">${esc(r.level)}${markOf(r.level) ? `<span class="mark">${esc(markOf(r.level))}</span>` : ''}</th><td class="n gpu">${ms2(r.plain)}</td>${cloudOff ? `<td class="n gpu">${ms2(r.bare)}</td>` : ''}<td class="n gpu">${ms2(r.look)}</td><td class="n gpu">${sgn(r.adds, r.share)}</td>${cloudRun ? `<td class="n gpu">${ms2(r.runLook)}</td><td class="n gpu">${sgn(r.runAdds, r.runShare)}</td><td class="words">${r.world === null ? '—' : esc(`${r.world >= 0 ? '+' : ''}${r.world.toFixed(2)} ms${r.worldWhere !== '—' ? ` (${r.worldWhere})` : ''}`)}</td>` : ''}${cloudWas ? `<td class="n gpu was">${ms2(r.was?.look)}</td><td class="n gpu was">${sgn(r.was?.adds ?? null, r.was?.share ?? null)}</td>` : ''}${cloudOff ? `<td class="n gpu">${sgn(r.bare !== null && r.plain !== null ? r.bare - r.plain : null, r.bareShare)}</td>` : ''}<td class="n gpu">${r.layers === null ? '—' : `${r.layers.toFixed(2)} ms`}</td><td class="words">${esc(r.where)}</td>${cloudOff ? `<td class="words">${esc(r.trials || '—')}</td>` : ''}<td class="n">${r.bySwitch ?? '—'}</td></tr>`; }).join('\n')}
</tbody></table></div>
${cloudReading ? `<p>${esc(cloudReading)}</p>\n` : ''}<p class="legend">${esc(CLOUD_LEGEND + (cloudBad.length ? ` Not ranked, because the run set them apart (their two pages were not one picture, or the GPU did not keep step): ${cloudBad.join(', ')}.` : ''))}</p>${cloudRunBad.length ? `<p class="legend">The running read the run set apart, in its own words:</p><ul class="overs">${cloudRunBad.map(x => `<li><b>${esc(x.level)}</b>: ${esc(x.why.join('; '))}.</li>`).join('')}</ul>` : ''}</section>

` : ''}<section><h2>How this page is made</h2>
<ul><li>${code('bun run tools/qa/level-audit.ts')} counts every level, headless, in under a minute. ${code('--check <level>')} is the budget check a dressed level must pass.</li><li>${code('node tools/qa/level-cost-page.cjs <level>')} boots the page once on the software renderer: programs compiled, layers off against on, two stills.</li><li>${code('bun run tools/qa/level-table.ts --dir <folder>')} builds this page from those files and from each dressing task's row.</li></ul></section>
</div>
`;
  writeFileSync(`${D}/read-first.html`, page); }

const missing = rows.filter(r => !r.row && !r.audit && !r.cost).map(r => r.L.level);   // (nothing of the level in the folder: the committed baseline does not count)
console.log(`level-table: ${rows.length} levels → ${D}/read-first.md`);
console.log(`  rows from the dressing tasks: ${rows.filter(r => r.row).length} (${dressed} dressed) · audit files: ${rows.filter(r => r.after).length} · software reads: ${read} · priced on a GPU: ${priced}${mac ? ` (mac/${macFiles[0].f})` : ''}`);
console.log(`  over a bar: ${overs.map(r => r.L.level).join(', ') || 'none'} · for a GPU first: ${gpus.map(r => r.L.level).join(', ') || 'none'}`);
if (missing.length) console.log(`  nothing in the folder for: ${missing.join(', ')}`);
process.exit(argv.includes('--check') && missing.length ? 1 : 0);
