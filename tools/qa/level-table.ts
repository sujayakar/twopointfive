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
//   --run "<one line>"  the one line the page gives him to get the pricing run going (or --branch <name>: the plain git line for that branch)
// It also writes <dir>/read-first.html: the same page as one HTML file to publish (its stills are linked by the same relative paths).
// THE BUDGET is the one PROPOSED on 11-02 and not ruled on: on High the GPU's time with the look within 10% of look off, a frame under 16.6 ms, shaders compiled
// once. Only a GPU answers it. Until a file from one is in <dir>/mac/, a level is called out from what can be counted (level-audit.ts's bars) and from the
// software read, and the page says so beside every such line.
import { readFileSync, writeFileSync, existsSync, readdirSync, statSync } from 'node:fs';
import { resolve, dirname } from 'node:path';

const HERE = dirname(new URL(import.meta.url).pathname), ROOT_DIR = resolve(HERE, '../..');
const argv = process.argv.slice(2), valueOf = (w: string) => { const i = argv.indexOf(w); return i >= 0 ? argv[i + 1] : undefined; };
const DIR = valueOf('--dir'); if (!DIR) { console.error('level-table: --dir <folder> (the folder with levels/, audit/, cost/, mac/)'); process.exit(2); }
/** the ONE LINE that gets the pricing run onto his machine (he asked for a one-liner whenever he is to run something): handed in, never written here */
const RUN = valueOf('--run') ?? `git fetch origin && git checkout ${valueOf('--branch') ?? '<the branch that has the run>'} && git pull && bun run dev`;
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
/** a GPU run's three states for a level, ms: the page without the look, the look page with its layers off, with them on */
const gpuCell = (m: any): string => !m || m.on.gpu === null ? '—' : `${m.page?.win.gpu ?? '—'} → ${m.off.gpu ?? '—'} → ${m.on.gpu}`;
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
  if (m) { const vs = m.page ? m.gpuOverPage : m.gpuMore, from = m.page ? m.page.win.gpu : m.off.gpu;   // (against the page without the look where the run timed it: the budget's own "look off"; else against the layers off)
    if (vs !== null && vs !== undefined && vs > (mac.bar?.gpuMore ?? 0.1)) over.push(`on the GPU the look adds ${more(vs)} against ${m.page ? 'the page without the look' : 'its layers off'} (${from} → ${m.on.gpu} ms; the bar is +10%)`);
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
out.push('This is the one run that answers the budget. It is hands off: ' + rows.length * 2 + ' page loads, about half a minute each, with the laptop plugged in.', '');
out.push('```', RUN, '```', '');
out.push('Then open this and leave the tab in front:', '', '```', 'localhost:5173/?look&pricelevels', '```', '');
out.push('Each level loads twice. The look page plays four seconds, holds its picture still and times it with the look\'s layers off and on in turn. Then the same mission loads without the look and is timed at the same moment.', 'The last page shows the table and saves one file, `price-levels-<date>.json`, to your downloads. Drop that file in the task and this page fills in.', '');
out.push('"Look off" has two meanings and the file has both. The page without the look is the game as it is drawn today, and it is what the budget means. The layers off is the look page with every layer that has a cost put off at once: same shaders, same shapes, same camera.', '');
out.push('- `?look&pricelevels=pass,block,compound` prices only those.', '- Add `&quality=medium` for another preset. The default is High, the one the budget is said for.', '- Add `&pricepage=0` to skip the loads without the look: half the time, and only the layers are priced.', '- Esc stops the run.', '');
out.push(mac ? `The newest file here: \`mac/${macFiles[0].f}\`, from ${mac.gpu}, ${String(mac.at).slice(0, 16).replace('T', ' ')} UTC, quality ${mac.quality}.` : 'No file from a GPU is here yet, so every cost below is a count or a software read.', '');

out.push('## Over the proposed budget', '');
if (!overs.length) out.push('No level is over a bar that can be read here.', '');
else { out.push(mac ? 'Each line says which number puts the level there.' : 'Each line says which count puts the level there. These are the bars a count can answer; the GPU bars wait for your run.', '');
  for (const r of overs) out.push(`- **${r.L.level}**${r.row?.dressed ? '' : ' (not dressed yet)'}: ${r.over.join('; ')}.`); out.push(''); }
const gpus = rows.filter(r => r.forGpu.length && !r.m);
if (gpus.length) { out.push('### Not over a bar, but wants a GPU read first', '', 'The look was timed on one level, the pass. These ask more of a frame than the pass does.', '');
  for (const r of gpus) out.push(`- **${r.L.level}**: ${r.forGpu.join('; ')}.`); out.push(''); }

const HEAVY: [string, keyof Slim][] = [['boxes drawn', 'drawn'], ['boxes that move', 'moving'], ['cut planes', 'planes'], ['lamps burning', 'lampsOn'], ['boxes that glow', 'glowing']];
const nowOf = (r: typeof rows[number]) => r.after ?? r.before, heavy = HEAVY.map(([what, k]) => { const top = rows.filter(r => nowOf(r)).sort((a, b) => (nowOf(b)![k] as number) - (nowOf(a)![k] as number))[0]; return { what, level: top?.L.level ?? '—', n: top ? nowOf(top)![k] as number : 0, pass: (nowOf(rows.find(r => r.L.level === 'pass')!)?.[k] ?? 0) as number }; });
out.push('### The heaviest level in each count', '', 'The pass is the level the look was timed on, so its own count is beside each.', '');
for (const h of heavy) out.push(`- ${h.what}: **${h.level}** with ${h.n} (the pass has ${h.pass}).`); out.push('');
out.push('## Where each level stands', '');
out.push('| Level | Task | Dressed | Stills | Open it | From the library | What is left | Needs your eye on a GPU |', '|---|---|---|---|---|---|---|---|');
for (const r of rows) { const s = r.stills, pics = [s.before ? `[before](${s.before})` : '', s.after ? `[after](${s.after})` : '', !s.after && s.look ? `[look, software](${s.look})` : '', !s.before && s.plain ? `[layers off, software](${s.plain})` : ''].filter(Boolean).join(' · ') || '—';
  out.push(`| ${r.L.level} | ${cell(r.part)} | ${r.row ? (r.row.dressed ? 'yes' : 'not yet') : 'no row yet'} | ${pics} | \`localhost:5173/${cell(r.row?.address ?? `?look&${r.L.query.slice(1)}`)}\` | ${cell(lib(r.row))} | ${cell((r.row?.left ?? []).join('; ') || (r.row ? '—' : 'not started'))} | ${cell((r.row?.needs_his_eye ?? []).join('; ') || '—')} |`); }
out.push('', 'The hour is the one the mission opens at: ' + [...new Set(rows.map(r => r.L.hour))].map(h => `${h} (${rows.filter(r => r.L.hour === h).length})`).join(', ') + '.', '');

out.push('## What each level costs', '');
out.push('Counts are before → after dressing, as the look page draws the level at the hour its mission opens at. One number means it has not moved.', '');
out.push('| Level | Boxes drawn | Boxes that move | Cut planes | Round things | Lamps burning | No material | Software read, layers on | GPU ms: no look → layers off → on | Look against no look | Frame on GPU | Programs made by the switch |', '|---|---|---|---|---|---|---|---|---|---|---|---|');
for (const r of rows) { const b = r.before, a = r.after, sw = r.m ? r.m.programs?.bySwitch : r.cost?.look?.compiled?.bySwitch?.pipelines;
  out.push(`| ${r.L.level} | ${arrow(b?.drawn, a?.drawn)} | ${arrow(b?.moving, a?.moving)} | ${arrow(b?.planes, a?.planes)} | ${arrow(b?.round, a?.round)} | ${arrow(b?.lampsOn, a?.lampsOn)} | ${b ? pct(b.none) : '—'}${a && b && Math.abs(a.none - b.none) > 0.0005 ? ` → ${pct(a.none)}` : ''} | ${r.ratio !== undefined ? more(r.ratio - 1) : '—'} | ${gpuCell(r.m)} | ${r.m ? more(r.m.page ? r.m.gpuOverPage : r.m.gpuMore) : '—'} | ${r.m ? `${r.m.on.ms.toFixed(1)} ms` : '—'} | ${sw === undefined ? '—' : sw} |`); }
out.push('');
out.push('- **Software read, layers on** is how much longer a frame takes with the look\'s layers on than off, on the same page in the same boot, on a CPU renderer. It is a direction, not a price.', '- **GPU** columns are from your run. **Programs made by the switch** should be 0: the look must not compile anything when its layers change.', '- **No material** is the share of the picture from above that the look draws flat.', '');
const plainPage = json(`${D}/cost/pass/cost.json`)?.againstPlainPage;
if (plainPage) { out.push('## What the look costs every player, layers on or off', '');
  out.push(`The look is switched by numbers at run time, so its shader text is compiled whichever way the layers stand. On the pass, software page: shader text ${plainPage.textKB[0]} → ${plainPage.textKB[1]} KB, pipelines ${plainPage.pipelines[0]} → ${plainPage.pipelines[1]}, ready in ${plainPage.readySecs[0]} → ${plainPage.readySecs[1]} s, a frame ${plainPage.frameMs[0]} → ${plainPage.frameMs[1]} ms.`, 'The first number is the page without the look, the second the look page. Compile time on a CPU renderer is not a GPU driver\'s: the text size and the pipeline count carry over, the seconds do not.', ''); }
out.push('## How this page is made', '');
out.push('- `bun run tools/qa/level-audit.ts --out <this folder>/audit` counts every level, headless, in under a minute. `--check <level>` is the budget check a dressed level must pass.', '- `node tools/qa/level-cost-page.cjs <level> --out <this folder>/cost` boots the page once on the software renderer: programs compiled, layers off against on, two stills.', '- `bun run tools/qa/level-table.ts --dir <this folder>` builds this page.', '- A task that dresses a level writes `levels/<level>/row.json`, `before.png` and `after.png`.', '');
writeFileSync(`${D}/read-first.md`, out.join('\n'));

// ---- the same page as one HTML file, to publish (a page he can open): the stills by the same relative paths, published beside it
{ const esc = (s: unknown) => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  const code = (s: string) => `<code>${esc(s)}</code>`, list = (a: string[]) => a.length ? `<ul>${a.map(x => `<li>${x}</li>`).join('')}</ul>` : '';
  const state = (r: typeof rows[number]) => r.row?.dressed ? '<span class="pill ok">dressed</span>' : r.row ? '<span class="pill warn">in work</span>' : '<span class="pill">not started</span>';
  const pic = (src: string | null, label: string) => src ? `<a class="still" href="${esc(src)}"><img src="${esc(src)}" alt="${esc(label)}" loading="lazy"><span>${esc(label)}</span></a>` : '';
  const stand = rows.map(r => { const s = r.stills, pics = [pic(s.before, 'before'), pic(s.after, 'after'), !s.after ? pic(s.look, 'look on, software') : '', !s.before ? pic(s.plain, 'layers off, software') : ''].join('');
    return `<tr><th scope="row">${esc(r.L.level)}<small>${esc(r.L.missions.join(' · '))} · ${esc(r.L.hour)}</small></th><td>${state(r)}<small>${esc(r.part)}</small></td><td class="stills">${pics || '<span class="none">none yet</span>'}</td><td>${code(`localhost:5173/${r.row?.address ?? `?look&${r.L.query.slice(1)}`}`)}</td><td>${esc(lib(r.row))}</td><td>${esc((r.row?.left ?? []).join('; ') || (r.row ? '—' : 'not started'))}</td><td>${esc((r.row?.needs_his_eye ?? []).join('; ') || '—')}</td></tr>`; }).join('\n');
  const num = (b: number | undefined, a: number | undefined) => `<td class="n">${esc(arrow(b, a))}</td>`;
  const costs = rows.map(r => { const b = r.before, a = r.after, m = r.m, sw = m ? m.programs?.bySwitch : r.cost?.look?.compiled?.bySwitch?.pipelines, vs = m ? (m.page ? m.gpuOverPage : m.gpuMore) : null, flag = r.over.length ? ' class="over"' : '';
    return `<tr${flag}><th scope="row">${esc(r.L.level)}</th>${num(b?.drawn, a?.drawn)}${num(b?.moving, a?.moving)}${num(b?.planes, a?.planes)}${num(b?.round, a?.round)}${num(b?.lampsOn, a?.lampsOn)}<td class="n">${b ? pct(b.none) : '—'}${a && b && Math.abs(a.none - b.none) > 0.0005 ? ` → ${pct(a.none)}` : ''}</td><td class="n soft">${r.ratio !== undefined ? more(r.ratio - 1) : '—'}</td><td class="n gpu">${esc(gpuCell(m))}</td><td class="n gpu">${m ? more(vs) : '—'}</td><td class="n gpu">${m ? `${m.on.ms.toFixed(1)}` : '—'}</td><td class="n">${sw === undefined ? '—' : sw}</td></tr>`; }).join('\n');
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
.frame thead th.n { text-align: right; } .frame td.gpu, .frame th.gpu { background: var(--gpu); } .frame td.soft { color: var(--muted); }
.frame tr.over th { color: var(--bad); } .frame tr:last-child th, .frame tr:last-child td { border-bottom: 0; }
.stills { min-width: 190px; } .still { display: inline-block; margin: 0 6px 4px 0; color: var(--muted); font-size: 11px; text-decoration: none; }
.still img { display: block; width: 168px; aspect-ratio: 16 / 9; object-fit: cover; border: 1px solid var(--line); }
.none { color: var(--muted); }
.pill { display: inline-block; font: 500 11px var(--mono); text-transform: uppercase; letter-spacing: .05em; padding: 2px 7px; border: 1px solid var(--line); border-radius: 2px; color: var(--muted); }
.pill.ok { color: var(--ok); border-color: var(--ok); } .pill.warn { color: var(--warn); border-color: var(--warn); }
.overs li b { color: var(--bad); } .legend { color: var(--muted); font-size: 13px; }
</style>
<div class="sheet">
<header><h1>Levels in the new look</h1><p class="lede">Every level on the mission list: dressed or not, and what the look costs on it. Built ${esc(new Date().toISOString().slice(0, 16).replace('T', ' '))} UTC from the dressing tasks' rows, the level audit and the page reads.</p>
<div class="counts"><span><b>${dressed}</b>of ${rows.length} dressed</span><span><b>${priced}</b>of ${rows.length} priced on a GPU</span><span><b>${read}</b>of ${rows.length} with a software read</span><span><b>${overs.length}</b>over a proposed bar</span></div></header>

<section><h2>Price every level on your Mac</h2>
<div class="run"><p>This is the one run that answers the budget. It is hands off: ${rows.length * 2} page loads, about half a minute each, with the laptop plugged in.</p>
<pre>${esc(RUN)}</pre>
<p>Then open this and leave the tab in front:</p>
<pre class="addr">localhost:5173/?look&amp;pricelevels</pre>
<p>Each level loads twice. The look page plays four seconds, holds its picture still and times it with the look's layers off and on in turn. Then the same mission loads without the look and is timed at the same moment. The last page shows the table and saves one file, ${code('price-levels-<date>.json')}, to your downloads. Drop that file in the task and this page fills in.</p>
<ul><li>${code('?look&pricelevels=pass,block,compound')} prices only those.</li><li>Add ${code('&quality=medium')} for another preset. The default is High, the one the budget is said for.</li><li>Add ${code('&pricepage=0')} to skip the loads without the look: half the time, and only the layers are priced.</li><li>Esc stops the run.</li></ul>
<p>"Look off" has two meanings and the file has both. The page without the look is the game as it is drawn today, and it is what the budget means. The layers off is the look page with every layer that has a cost put off at once: same shaders, same shapes, same camera.</p>
<p>${mac ? `The newest file here is from ${esc(mac.gpu)}, ${esc(String(mac.at).slice(0, 16).replace('T', ' '))} UTC, at quality ${esc(mac.quality)}.` : 'No file from a GPU is here yet, so every cost below is a count or a software read.'}</p></div></section>

<section><h2>The budget</h2><p>This is the budget proposed on 2 November. You have not ruled on its numbers.</p>
<ul><li>An M4 Max holds 60 on High, with GPU time within 10 % of look off.</li><li>An RTX 4080 at 4K stays locked at 60. An M1 Max stays under 16.6 ms. The AMD box shows no new failure.</li><li>Shaders compile once. A layer over 0.5 ms gets a tier row.</li></ul>
<p>Nothing on this page was seen on a GPU unless its column is shaded. Counts are exact. A software read is the same page on a CPU renderer, where a frame takes seconds: a direction, not a price.</p></section>

<section><h2>Over the proposed budget</h2>
${overs.length ? `<p>${mac ? 'Each line says which number puts the level there.' : 'Each line says which count puts the level there. These are the bars a count can answer. The GPU bars wait for your run.'}</p><ul class="overs">${overs.map(r => `<li><b>${esc(r.L.level)}</b>${r.row?.dressed ? '' : ' (not dressed yet)'}: ${esc(r.over.join('; '))}.</li>`).join('')}</ul>` : '<p>No level is over a bar that can be read here.</p>'}
<h3>The heaviest level in each count</h3><p>The pass is the level the look was timed on, so its own count is beside each.</p>${list(heavy.map(h => `${esc(h.what)}: <b>${esc(h.level)}</b> with ${h.n} (the pass has ${h.pass}).`))}
${gpus.length ? `<h3>Not over a bar, but wants a GPU read first</h3><p>The look was timed on one level, the pass. These ask more of a frame than the pass does.</p>${list(gpus.map(r => `<b>${esc(r.L.level)}</b>: ${esc(r.forGpu.join('; '))}.`))}` : ''}</section>

<section><h2>Where each level stands</h2>
<div class="frame"><table><thead><tr><th>Level</th><th>State</th><th>Stills</th><th>Open it</th><th>From the library</th><th>What is left</th><th>Needs your eye on a GPU</th></tr></thead><tbody>
${stand}
</tbody></table></div></section>

<section><h2>What each level costs</h2>
<p>Counts are before → after dressing, as the look page draws the level at the hour its mission opens at. One number means it has not moved. Shaded columns are from a GPU.</p>
<div class="frame"><table><thead><tr><th>Level</th><th class="n">Boxes drawn</th><th class="n">That move</th><th class="n">Cut planes</th><th class="n">Round things</th><th class="n">Lamps burning</th><th class="n">No material</th><th class="n">Software: layers on</th><th class="n gpu">GPU ms: no look → layers off → on</th><th class="n gpu">Look vs no look</th><th class="n gpu">Frame ms</th><th class="n">Programs by the switch</th></tr></thead><tbody>
${costs}
</tbody></table></div>
<p class="legend">No material is the share of the picture from above that the look draws flat. Software: layers on is how much longer a frame takes with the layers on than off, on a CPU renderer. Programs by the switch should be 0: the look must not compile anything when its layers change.</p>
${plainPage ? `<h3>What the look costs every player, layers on or off</h3><p>The look is switched by numbers at run time, so its shader text is compiled whichever way the layers stand. On the pass, on the software page: shader text ${plainPage.textKB[0]} → ${plainPage.textKB[1]} KB, pipelines ${plainPage.pipelines[0]} → ${plainPage.pipelines[1]}, ready in ${plainPage.readySecs[0]} → ${plainPage.readySecs[1]} s. The first number is the page without the look. The text size and the pipeline count carry over to a GPU; the seconds do not.</p>` : ''}</section>

<section><h2>How this page is made</h2>
<ul><li>${code('bun run tools/qa/level-audit.ts')} counts every level, headless, in under a minute. ${code('--check <level>')} is the budget check a dressed level must pass.</li><li>${code('node tools/qa/level-cost-page.cjs <level>')} boots the page once on the software renderer: programs compiled, layers off against on, two stills.</li><li>${code('bun run tools/qa/level-table.ts --dir <folder>')} builds this page from those files and from each dressing task's row.</li></ul></section>
</div>
`;
  writeFileSync(`${D}/read-first.html`, page); }

const missing = rows.filter(r => !r.before && !r.after).map(r => r.L.level);
console.log(`level-table: ${rows.length} levels → ${D}/read-first.md`);
console.log(`  rows from the dressing tasks: ${rows.filter(r => r.row).length} (${dressed} dressed) · audit files: ${rows.filter(r => r.after).length} · software reads: ${read} · priced on a GPU: ${priced}${mac ? ` (mac/${macFiles[0].f})` : ''}`);
console.log(`  over a bar: ${overs.map(r => r.L.level).join(', ') || 'none'} · for a GPU first: ${gpus.map(r => r.L.level).join(', ') || 'none'}`);
if (missing.length) console.log(`  no counts at all for: ${missing.join(', ')}`);
process.exit(argv.includes('--check') && missing.length ? 1 : 0);
