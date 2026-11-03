// THE TABLE OF LEVELS: one page, a row a level of the mission list, built from files (the levels goal, 11-03: "we'll want to dress up all the levels and check
// their budget"). Nothing is measured here: this reads what the other tools and the tasks that dress levels wrote into ONE folder, and writes <dir>/read-first.md.
//   <dir>/levels/<level>/row.json     a dressing task's row for its level: { level, part, dressed, address, hour, what_changed[], left[], needs_his_eye[],
//                                     counts: { before, after }, library: { used[], added[], wanted[] }, branch, commits[], checks[] }
//                                     (counts: `level-audit.ts --counts <level>` prints the block)
//   <dir>/levels/<level>/before.png, after.png     its stills (linked, not read)
//   <dir>/audit/<level>.json          tools/qa/level-audit.ts --out <dir>/audit: the level as it stands on the tree the audit ran on
//   tools/qa/fixtures/level-audit-base.json        every level before it was dressed: the "before" of a level whose row has none
//   <dir>/cost/<level>/cost.json      tools/qa/level-cost-page.cjs --out <dir>/cost: one boot on the software adapter (a relative read), look.png and plain.png
//   <dir>/mac/*.json                  the file the page's own run hands back on a real GPU (`?look&pricelevels`, ui/pricerun.ts): the newest one is read
//   <dir>/parts.json                  optional: { "<task's name>": ["level", …] }, for the levels that have no row yet
//
//   bun run tools/qa/level-table.ts --dir <dir>            writes <dir>/read-first.md, prints what it found and what is missing
//   bun run tools/qa/level-table.ts --dir <dir> --check    … and exits 1 when a level of the list has no row in the table's inputs at all
// THE BUDGET is the one PROPOSED on 11-02 and not ruled on: on High the GPU's time with the look within 10% of look off, a frame under 16.6 ms, shaders compiled
// once. Only a GPU answers it. Until a file from one is in <dir>/mac/, a level is called out from what can be counted (level-audit.ts's bars) and from the
// software read, and the page says so beside every such line.
import { readFileSync, writeFileSync, existsSync, readdirSync, statSync } from 'node:fs';
import { resolve, dirname } from 'node:path';

const HERE = dirname(new URL(import.meta.url).pathname), ROOT_DIR = resolve(HERE, '../..');
const argv = process.argv.slice(2), valueOf = (w: string) => { const i = argv.indexOf(w); return i >= 0 ? argv[i + 1] : undefined; };
const DIR = valueOf('--dir'); if (!DIR) { console.error('level-table: --dir <folder> (the folder with levels/, audit/, cost/, mac/)'); process.exit(2); }
const D = resolve(DIR), json = (p: string): any => { try { return JSON.parse(readFileSync(p, 'utf8')); } catch { return null; } };
const { ALL_MISSIONS } = await import(`${ROOT_DIR}/src/scene/scenarioList.ts`), { scenarioQuery } = await import(`${ROOT_DIR}/src/scene/scenarios.ts`);
const { AUDIT_BARS } = await import('./level-count.ts');   // (the budget check's bars: level-audit.ts --check holds a level to them, this page says them)

type Slim = { boxes: number; moving: number; drawn: number; planes: number; round: number; bites: number; lampsOn: number; lamps: number; panels: number; glowing: number; openings: number; props: number; none: number; byChance: number };
const levels: { level: string; title: string; hour: string; query: string; missions: string[] }[] = [];
for (const e of ALL_MISSIONS as any[]) { if (e.offList) continue; const level = e.level ? e.level.replace(/\.json$/, '') : 'office', at = levels.find(l => l.level === level);
  if (at) at.missions.push(e.title); else levels.push({ level, title: e.title, hour: e.hour ?? 'night', query: scenarioQuery(e), missions: [e.title] }); }
const base: Record<string, Slim> = json(`${HERE}/fixtures/level-audit-base.json`)?.levels ?? {};
const parts: Record<string, string[]> = json(`${D}/parts.json`) ?? {};
const slimOf = (r: any): Slim | null => r && r.boxes && r.picture ? { boxes: r.boxes.all, moving: r.boxes.moving, drawn: r.boxes.drawn, planes: r.boxes.planes, round: r.shapes.rounds + r.shapes.pass, bites: r.shapes.walls, lampsOn: r.lamps.on, lamps: r.lamps.all,
  panels: r.lamps.panels, glowing: r.glowing, openings: r.openings.found, props: r.props, none: r.picture.none, byChance: r.picture.byChance } : null;
// the newest file a GPU handed back
const macDir = `${D}/mac`, macFiles = existsSync(macDir) ? readdirSync(macDir).filter(f => f.endsWith('.json')).map(f => ({ f, t: statSync(`${macDir}/${f}`).mtimeMs })).sort((a, b) => b.t - a.t) : [];
const mac: any = macFiles.length ? json(`${macDir}/${macFiles[0].f}`) : null, macRow = (level: string) => (mac?.rows ?? []).find((r: any) => r.level === level) ?? null;

const pct = (v: number | null | undefined, d = 1) => v === null || v === undefined || !isFinite(v) ? '—' : `${(100 * v).toFixed(v >= 0.0995 ? 0 : d)}%`;
const more = (v: number | null | undefined) => v === null || v === undefined || !isFinite(v) ? '—' : `${v >= 0 ? '+' : ''}${(100 * v).toFixed(1)}%`;
const arrow = (a: number | undefined, b: number | undefined) => a === undefined ? (b === undefined ? '—' : String(b)) : b === undefined || a === b ? String(a) : `${a} → ${b}`;
/** a row's library block in words: how many pieces it lays, what it added to the library, what it still wants from it */
const lib = (row: any): string => { const L = row?.library; if (!L) return '—'; const list = (a: unknown) => Array.isArray(a) ? a.map(String) : [];
  const used = list(L.used), added = list(L.added), wanted = list(L.wanted); return [used.length ? `uses ${used.length} piece${used.length === 1 ? '' : 's'}` : '', added.length ? `added: ${added.join(', ')}` : '', wanted.length ? `wants: ${wanted.join(', ')}` : ''].filter(Boolean).join('; ') || '—'; };
const cell = (s: string) => s.replace(/\|/g, '/').replace(/\n/g, ' ');
const passCost = json(`${D}/cost/pass/cost.json`)?.look?.frameMs?.onOverOff as number | undefined;
const heaviest = (k: keyof Slim) => Object.entries(base).reduce((m, [lv, b]) => (b[k] as number) > m[1] ? [lv, b[k] as number] as [string, number] : m, ['', 0] as [string, number]);

const rows = levels.map(L => {
  const row = json(`${D}/levels/${L.level}/row.json`), audit = slimOf(json(`${D}/audit/${L.level}.json`)), cost = json(`${D}/cost/${L.level}/cost.json`), m = macRow(L.level);
  const before: Slim | undefined = row?.counts?.before ?? base[L.level], after: Slim | undefined = row?.counts?.after ?? audit ?? undefined;
  const part = row?.part ?? Object.entries(parts).find(([, ls]) => ls.includes(L.level))?.[0] ?? '—';
  const still = (f: string) => existsSync(`${D}/levels/${L.level}/${f}`) ? `levels/${L.level}/${f}` : null;
  // ---- over the proposed budget, and why
  const over: string[] = [], forGpu: string[] = [];
  if (m) { if (m.gpuMore !== null && m.gpuMore > (mac.bar?.gpuMore ?? 0.1)) over.push(`on the GPU the look's layers add ${more(m.gpuMore)} (${m.off.gpu} → ${m.on.gpu} ms; the bar is +10%)`);
    if (m.on.ms > (mac.bar?.frameMs ?? 16.6)) over.push(`a frame takes ${m.on.ms.toFixed(1)} ms with the layers on (the bar is 16.6 ms)`);
    if (m.programs?.bySwitch > 0) over.push(`switching the layers made ${m.programs.bySwitch} shader programs (the bar is none)`); }
  if (cost?.look?.compiled?.bySwitch && (cost.look.compiled.bySwitch.pipelines > 0 || cost.look.compiled.bySwitch.modules > 0)) over.push(`switching the layers compiled ${cost.look.compiled.bySwitch.pipelines} more programs on the software page (the bar is none)`);
  const now = after ?? before;
  if (now) { for (const [what, k] of [['boxes drawn', 'drawn'], ['boxes that move', 'moving'], ['cut planes', 'planes'], ['lamps burning', 'lampsOn'], ['boxes that glow', 'glowing']] as [string, keyof Slim][]) {
      const [top, n] = heaviest(k); if ((now[k] as number) > AUDIT_BARS.overHeaviest * Math.max(1, n)) over.push(`${what}: ${now[k]}, over ${AUDIT_BARS.overHeaviest} × the heaviest level before dressing (${top}, ${n})`);
      else if (base.pass && L.level !== 'pass' && (now[k] as number) > AUDIT_BARS.overPass * Math.max(1, base.pass[k] as number)) forGpu.push(`${what} ${now[k]} (the pass ${base.pass[k]})`); }
    if (now.none > AUDIT_BARS.bare) over.push(`${pct(now.none)} of the picture has no material (the bar is ${pct(AUDIT_BARS.bare)})`);
    if (now.byChance > AUDIT_BARS.chance) over.push(`${pct(now.byChance)} of the picture is a material by chance (the bar is ${pct(AUDIT_BARS.chance)})`); }
  const ratio = cost?.look?.frameMs?.onOverOff as number | undefined;
  if (!m && ratio !== undefined && passCost !== undefined && L.level !== 'pass' && ratio - 1 > 1.25 * (passCost - 1) && ratio - 1 > 0.05) forGpu.push(`on the software page the layers add ${more(ratio - 1)}, the pass ${more(passCost - 1)}`);
  return { L, row, part, before, after, cost, m, over, forGpu, ratio, stills: { before: still('before.png'), after: still('after.png'), look: existsSync(`${D}/cost/${L.level}/look.png`) ? `cost/${L.level}/look.png` : null, plain: existsSync(`${D}/cost/${L.level}/plain.png`) ? `cost/${L.level}/plain.png` : null } };
});

// ---- the page
const dressed = rows.filter(r => r.row?.dressed === true).length, priced = rows.filter(r => r.m).length, read = rows.filter(r => r.cost?.look?.frameMs?.onOverOff).length, overs = rows.filter(r => r.over.length);
const out: string[] = [];
out.push('# Every level in the new look: dressed or not, and what it costs', '');
out.push(`Built ${new Date().toISOString().slice(0, 16).replace('T', ' ')} UTC by \`tools/qa/level-table.ts\` from the files in this folder. Do not edit it by hand: the next build writes over it.`, '');
out.push(`${dressed} of ${rows.length} levels are dressed. ${priced} of ${rows.length} are priced on a real GPU. ${read} of ${rows.length} have a software read.`, '');
out.push('## The budget, and who answers it', '');
out.push('The budget is the one proposed on 2 November. You have not ruled on its numbers.', '');
out.push('- An M4 Max holds 60 on High, with GPU time within 10 % of look off.', '- An RTX 4080 at 4K stays locked at 60. An M1 Max stays under 16.6 ms. The AMD box shows no new failure.', '- Shaders compile once. A layer over 0.5 ms gets a tier row.', '');
out.push('Nothing on this page was seen on a GPU unless its line says so. Counts are exact. A software read is a relative read: the same page on a CPU renderer, where a frame takes seconds.', '');
out.push('### Price every level on your Mac', '');
out.push('This is the one run that answers the budget. It is hands off and takes about ' + Math.max(1, Math.round(rows.length * 25 / 60)) + ' minutes with the laptop plugged in.', '');
out.push('```', 'git fetch origin && git checkout <the branch named below> && git pull', 'bun run dev', '```', '');
out.push('Then open `localhost:5173/?look&pricelevels` and leave the tab in front.', '');
out.push('Each level loads, plays four seconds, holds its picture still and times it with the look\'s layers off and on in turn.', 'The last page shows the table and saves one file, `price-levels-<date>.json`, to your downloads. Drop that file in the task and this page fills in.', '');
out.push('- `?look&pricelevels=pass,block,compound` prices only those.', '- Add `&quality=medium` for another preset. The default is High, the one the budget is said for.', '- Esc stops the run.', '');
out.push(mac ? `The newest file here: \`mac/${macFiles[0].f}\`, from ${mac.gpu}, ${String(mac.at).slice(0, 16).replace('T', ' ')} UTC, quality ${mac.quality}.` : 'No file from a GPU is here yet, so every cost below is a count or a software read.', '');

out.push('## Over the proposed budget', '');
if (!overs.length) out.push('No level is over a bar that can be read here.', '');
else { out.push(mac ? 'Each line says which number puts the level there.' : 'Each line says which count puts the level there. These are the bars a count can answer; the GPU bars wait for your run.', '');
  for (const r of overs) out.push(`- **${r.L.level}**${r.row?.dressed ? '' : ' (not dressed yet)'}: ${r.over.join('; ')}.`); out.push(''); }
const gpus = rows.filter(r => r.forGpu.length && !r.m);
if (gpus.length) { out.push('### Not over a bar, but wants a GPU read first', '', 'The look was timed on one level, the pass. These ask more of a frame than the pass does.', '');
  for (const r of gpus) out.push(`- **${r.L.level}**: ${r.forGpu.join('; ')}.`); out.push(''); }

out.push('## Where each level stands', '');
out.push('| Level | Task | Dressed | Stills | Open it | From the library | What is left | Needs your eye on a GPU |', '|---|---|---|---|---|---|---|---|');
for (const r of rows) { const s = r.stills, pics = [s.before ? `[before](${s.before})` : '', s.after ? `[after](${s.after})` : '', !s.after && s.look ? `[look, software](${s.look})` : '', !s.before && s.plain ? `[layers off, software](${s.plain})` : ''].filter(Boolean).join(' · ') || '—';
  out.push(`| ${r.L.level} | ${cell(r.part)} | ${r.row ? (r.row.dressed ? 'yes' : 'not yet') : 'no row yet'} | ${pics} | \`localhost:5173/${cell(r.row?.address ?? `?look&${r.L.query.slice(1)}`)}\` | ${cell(lib(r.row))} | ${cell((r.row?.left ?? []).join('; ') || (r.row ? '—' : 'not started'))} | ${cell((r.row?.needs_his_eye ?? []).join('; ') || '—')} |`); }
out.push('', 'The hour is the one the mission opens at: ' + [...new Set(rows.map(r => r.L.hour))].map(h => `${h} (${rows.filter(r => r.L.hour === h).length})`).join(', ') + '.', '');

out.push('## What each level costs', '');
out.push('Counts are before → after dressing, as the look page draws the level at the hour its mission opens at. One number means it has not moved.', '');
out.push('| Level | Boxes drawn | Boxes that move | Cut planes | Round things | Lamps burning | No material | Software read, layers on | GPU, layers off → on | Frame on GPU | Programs made by the switch |', '|---|---|---|---|---|---|---|---|---|---|---|');
for (const r of rows) { const b = r.before, a = r.after, sw = r.m ? r.m.programs?.bySwitch : r.cost?.look?.compiled?.bySwitch?.pipelines;
  out.push(`| ${r.L.level} | ${arrow(b?.drawn, a?.drawn)} | ${arrow(b?.moving, a?.moving)} | ${arrow(b?.planes, a?.planes)} | ${arrow(b?.round, a?.round)} | ${arrow(b?.lampsOn, a?.lampsOn)} | ${b ? pct(b.none) : '—'}${a && b && Math.abs(a.none - b.none) > 0.0005 ? ` → ${pct(a.none)}` : ''} | ${r.ratio !== undefined ? more(r.ratio - 1) : '—'} | ${r.m && r.m.off.gpu !== null ? `${r.m.off.gpu} → ${r.m.on.gpu} ms (${more(r.m.gpuMore)})` : '—'} | ${r.m ? `${r.m.on.ms.toFixed(1)} ms` : '—'} | ${sw === undefined ? '—' : sw} |`); }
out.push('');
out.push('- **Software read, layers on** is how much longer a frame takes with the look\'s layers on than off, on the same page in the same boot, on a CPU renderer. It is a direction, not a price.', '- **GPU** columns are from your run. **Programs made by the switch** should be 0: the look must not compile anything when its layers change.', '- **No material** is the share of the picture from above that the look draws flat.', '');
const plainPage = json(`${D}/cost/pass/cost.json`)?.againstPlainPage;
if (plainPage) { out.push('## What the look costs every player, layers on or off', '');
  out.push(`The look is switched by numbers at run time, so its shader text is compiled whichever way the layers stand. On the pass, software page: shader text ${plainPage.textKB[0]} → ${plainPage.textKB[1]} KB, pipelines ${plainPage.pipelines[0]} → ${plainPage.pipelines[1]}, ready in ${plainPage.readySecs[0]} → ${plainPage.readySecs[1]} s, a frame ${plainPage.frameMs[0]} → ${plainPage.frameMs[1]} ms.`, 'The first number is the page without the look, the second the look page. Compile time on a CPU renderer is not a GPU driver\'s: the text size and the pipeline count carry over, the seconds do not.', ''); }
out.push('## How this page is made', '');
out.push('- `bun run tools/qa/level-audit.ts --out <this folder>/audit` counts every level, headless, in under a minute. `--check <level>` is the budget check a dressed level must pass.', '- `node tools/qa/level-cost-page.cjs <level> --out <this folder>/cost` boots the page once on the software renderer: programs compiled, layers off against on, two stills.', '- `bun run tools/qa/level-table.ts --dir <this folder>` builds this page.', '- A task that dresses a level writes `levels/<level>/row.json`, `before.png` and `after.png`.', '');
writeFileSync(`${D}/read-first.md`, out.join('\n'));

const missing = rows.filter(r => !r.before && !r.after).map(r => r.L.level);
console.log(`level-table: ${rows.length} levels → ${D}/read-first.md`);
console.log(`  rows from the dressing tasks: ${rows.filter(r => r.row).length} (${dressed} dressed) · audit files: ${rows.filter(r => r.after).length} · software reads: ${read} · priced on a GPU: ${priced}${mac ? ` (mac/${macFiles[0].f})` : ''}`);
console.log(`  over a bar: ${overs.map(r => r.L.level).join(', ') || 'none'} · for a GPU first: ${gpus.map(r => r.L.level).join(', ') || 'none'}`);
if (missing.length) console.log(`  no counts at all for: ${missing.join(', ')}`);
process.exit(argv.includes('--check') && missing.length ? 1 : 0);
