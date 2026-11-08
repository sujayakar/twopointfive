// THE START PAGE, WALKED: every mission, its setting rows, its team sizes, every man's card and every step of every row, the named kits, copy and paste, and the link
// Deploy loads, each written down as the words and states the page shows. Two trees' walks are compared: a swap of what is behind the cards changes no line.
//   bun run tools/qa/cards-page.ts OUT [--port N] [--missions id,id]     a real browser (CHROME=…), no GPU asked for, about a minute
//   bun run tools/qa/cards-page.ts --diff OUT OTHER                        exit 1 and the first lines that differ
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { Cdp } from './cdp';

const args = process.argv.slice(2), ROOT = resolve(import.meta.dir, '../..');
if (args[0] === '--diff') {
  const [a, b] = [args[1], args[2]].map(f => readFileSync(f, 'utf8').split('\n')); let n = 0, head = '';
  for (let i = 0; i < Math.max(a.length, b.length); i++) { if (a[i] === b[i]) { if (a[i]?.startsWith('#')) head = a[i]; continue; } if (n++ < 40) console.log(`${head}\n  ${i + 1} - ${a[i]}\n  ${i + 1} + ${b[i]}`); }
  console.log(n ? `cards: ${n} of ${a.length} lines differ` : `cards: ${a.length} lines, the same`); process.exit(n ? 1 : 0);
}
const out = args[0]; if (!out || out.startsWith('--')) { console.error('usage: bun run tools/qa/cards-page.ts OUT'); process.exit(2); }
const word = (k: string) => { const i = args.indexOf(k); return i < 0 ? undefined : args[i + 1]; };
const port = Number(word('--port') ?? 5991), deep = word('--missions')?.split(',');

const pageBridge = (await import(resolve(ROOT, 'tools/blitz/page-plugin.ts'))).default;   // (a bundler's plugin: no part of this folder's type check)
const built = await Bun.build({ entrypoints: [resolve(import.meta.dir, 'cards-entry.ts')], target: 'browser', plugins: [pageBridge,
  // (the man drawn beside the card is the renderer's: no part of what the cards say)
  { name: 'no-picture', setup(b: any) { b.onLoad({ filter: /src[\\/]ui[\\/]kitView\.ts$/ }, () => ({ loader: 'ts', contents: 'export const KIT_VIEW = { acts: true, kitMs: 150, quiet: true }; export function mountKitView() { return null; }' })); } }] });
if (!built.success) { for (const m of built.logs) console.error(m); process.exit(2); }
const js = await built.outputs[0].text();
const html = readFileSync(resolve(ROOT, 'index.html'), 'utf8').replace(/<script[^>]*type="module"[^>]*>[\s\S]*?<\/script>/g, '').replace('</body>', '<script type="module" src="/cards.js"></script></body>');
const server = Bun.serve({ port, fetch(req: Request) {
  const path = decodeURIComponent(new URL(req.url).pathname);
  if (path === '/') return new Response(html, { headers: { 'content-type': 'text/html' } });
  if (path === '/cards.js') return new Response(js, { headers: { 'content-type': 'text/javascript' } });
  for (const dir of ['', 'public', 'dist']) { const f = resolve(ROOT, dir, '.' + path); if (f.startsWith(ROOT) && existsSync(f)) return new Response(Bun.file(f) as unknown as Blob); }
  return new Response('no', { status: 404 });
} });

/** the walk, in the page */
function walk(deep: string[] | undefined): Promise<string> {
  const W = window as any, lines: string[] = [], said = (s: string) => lines.push(s.replace(/\s+/g, ' ').trim());
  const root = document.getElementById('missions')!, wait = (ms: number) => new Promise(r => setTimeout(r, ms));
  const all = (sel: string, at: ParentNode = root) => Array.from(at.querySelectorAll(sel)) as HTMLElement[];
  const gone = (e: HTMLElement): boolean => !!e.hidden || e.style.display === 'none';
  const flags = (e: HTMLElement) => Array.from(e.classList).filter(c => /^(on|off|sel|pick|changed|refused|idle|lent|zero|none|fixed|na|moved|touched|clip|loss|more|full|big|own|lit|open|issued|custom|armed|solo|long|quiet|nokit|unplayed|first|shut|in|out)$/.test(c)).sort().join('.');
  const hash = (s: string) => { let h = 2166136261; for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619); } return (h >>> 0).toString(16); };
  const words = (e: Element | null | undefined) => e?.textContent ?? '';
  const P = () => W.__cards.page(), kept = () => { const o: string[] = []; for (const k of Object.keys(localStorage).sort()) if (/^twopointsix\.(kit|levers|difficulty|loadout|quiet\w*)$/.test(k)) o.push(`${k}=${sorted(localStorage.getItem(k)!)}`); return o.join(' '); };
  /** a kept text by its members' names: the order it was written in, and the split of six an older build reads, are no part of it */
  const sorted = (text: string): string => { const at = (v: any): any => Array.isArray(v) ? v.map(at) : v && typeof v === 'object' ? Object.fromEntries(Object.keys(v).filter(k => k !== 'cans').sort().map(k => [k, at(v[k])])) : v; try { return JSON.stringify(at(JSON.parse(text))); } catch { return text; } };
  const pct = (e: HTMLElement | null) => e ? `${e.style.left || ''}+${e.style.width || ''}` : '';
  const rail = () => { said(`rail ${flags(root)} | ${words(root.querySelector('.mst'))} | ${words(root.querySelector('.mplace'))} | ${words(root.querySelector('.mblurb'))} | ${words(root.querySelector('.mtip'))} | ${words(root.querySelector('.mcsum'))} | ${all('.mfact').map(f => gone(f) ? '' : words(f)).join(' / ')}`);
    for (const v of all('.mval')) if (!gone(v)) said(`  val ${flags(v)} ${words(v)} | ${v.title}`);
    for (const r of all('.mstore .mdiff')) said(`  row ${flags(r)}${gone(r) ? ' gone' : ''} ${words(r.querySelector('.dl'))}: ${all('button', r).map(b => `${words(b)}${gone(b) ? '~' : ''}${flags(b) ? `(${flags(b)})` : ''}`).join(' ')} | ${words(r.querySelector('.dn'))}`);
    said(`  read ${words(root.querySelector('.mread'))}`); said(`  brief ${all('[class^=kb-], [class*=" kb-"]').filter(e => !e.children.length && !gone(e)).map(words).join(' / ')}`);
    const p = P(); said(`  link ${p.baseOf(p.missions.selected)} > ${p.loadout?.search() ?? 'none'}`); };
  const team = () => { const h = root.querySelector('#loadout'); if (!h) { said('team none'); return; }
    said(`team ${words(h.querySelector('.lo-words'))} | ${all('.lo-size', h).map(b => `${words(b)}${flags(b)}:${b.title}`).join(' / ')} | ${words(h.querySelector('.lo-note'))} | ${words(h.querySelector('.lo-squadp'))} | ${words(h.querySelector('.lo-pool'))} | ${words(h.querySelector('[data-act="deploy"]'))}`);
    said(`  kits ${all('[data-kit]', h).map(b => `${words(b)}${gone(b) ? '~' : ''}${flags(b)}:${b.title}`).join(' / ')} ${words(h.querySelector('.lo-kits-own'))}`);
    for (const s of all('.lo-slot', h)) said(`  man ${flags(s)} ${words(s.querySelector('.lo-name'))} | ${words(s.querySelector('.lo-role'))} | ${words(s.querySelector('.lo-sum'))} | ${words(s.querySelector('.lo-pouch'))} | ${flags(s.querySelector('.lo-name')!)} | ${all('.lo-ic', s).map(c => `${c.title}${words(c)}#${hash(c.innerHTML)}`).join(',')} gun#${hash(s.querySelector('.lo-gun')?.innerHTML ?? '')} ${s.title}`); };
  const card = () => { const c = root.querySelector('.kc') as HTMLElement | null; if (!c) { said('card none'); return; }
    said(`card ${flags(c)} ${words(c.querySelector('.kc-title'))}${flags(c.querySelector('.kc-title')!)} | ${words(c.querySelector('.kc-sub'))} | ${words(c.querySelector('.kc-pouch'))} ${all('.kc-pips i', c).map(i => flags(i) || '-').join(' ')} | ${all('.kc-fig i', c).map(i => i.style.background || 'issued').join(' ')}`);
    for (const b of all('.kc-bar[data-load]', c)) said(`  bar ${b.dataset.load} ${b.dataset.score} ${b.dataset.share} ${b.dataset.more} ${Array.from(b.querySelectorAll('i')).map(i => `${flags(i)}${pct(i)}`).join(' ')}`);
    for (const r of all('.kc-row', c)) if (!gone(r)) { said(`  ${r.dataset.field} ${flags(r)} under ${(r.closest('.kc-kids')?.parentElement as HTMLElement | null)?.dataset.gun ?? ''} | ${words(r.querySelector('.kc-label'))} | ${words(r.querySelector('.kc-val'))} | ${words(r.querySelector('.kc-note'))} | ${all('.kc-step', r).map(b => `${flags(b)}:${b.title}`).join(' ')} | ${r.title} | ico#${hash(r.querySelector('.kc-ico')?.innerHTML ?? '')}${flags(r.querySelector('.kc-ico')!)} noise ${r.dataset.noise ?? ''}`);
      for (const s of all('.kc-stat', r)) said(`    ${words(s)} ${Array.from(s.querySelectorAll('i')).map(i => `${flags(i)}${pct(i)}`).join(' ')}`);
      const chips = all('.kc-chip', r); if (chips.length) said(`    chips ${chips.map(x => `${x.dataset.chip}${flags(x)} ${words(x)} ${x.title} #${hash(x.innerHTML)}`).join(' / ')}`); }
    for (const p of all('.kc-pop.open', c)) { said(`  pop ${p.dataset.pop} ${words(p.querySelector('.kc-pop-note'))}`); for (const b of all('.kc-pick', p)) said(`    pick ${b.dataset.pick} ${flags(b)} ${words(b.querySelector('.kc-pick-name'))} | ${b.title} | ${all('[data-stat]', b).map(t => `${t.dataset.stat} ${t.dataset.score}`).join(' ')} #${hash(b.querySelector('.kc-pick-ico')?.innerHTML ?? '')}`); }
    said(`  sets ${all('.kc-set, .kc-clip', c).map(b => `${words(b)}${flags(b)}:${b.title}`).join(' / ')} | ${words(c.querySelector('.kc-words'))}`); };
  const press = (e: Element | null | undefined) => { (e as HTMLElement | null)?.click(); };
  const key = (code: string, k = code) => { window.dispatchEvent(new KeyboardEvent('keydown', { code, key: k, bubbles: true, cancelable: true })); window.dispatchEvent(new KeyboardEvent('keyup', { code, key: k, bubbles: true })); };
  return (async () => {
    const ids = P().missions.list.map((m: any) => m.id);
    said(`# rows`); for (const h of all('.msec')) said(`head ${words(h)} ${h.title} ${h.dataset.count ?? ''}`); for (const r of all('.mrow')) said(`${r.dataset.mission} ${r.dataset.group ?? ''} ${flags(r)} ${words(r)} ${r.getAttribute('href') ?? ''}`);
    for (const id of ids) { P().missions.select(id); await wait(30); said(`# mission ${id}`); rail(); team(); }
    // the setting rows, pressed: on one mission of each kind
    const kinds = new Map<string, string>(); for (const m of P().missions.list) { const k = `${m.scenario?.kind ?? 'solo'}:${m.squad}:${m.scenario?.ways ? 'ways' : ''}:${m.group}`; if (!kinds.has(k)) kinds.set(k, m.id); }
    for (const id of kinds.values()) { P().missions.select(id); await wait(20);
      for (const r of all('.mstore .mdiff')) { if (gone(r) || r.classList.contains('na')) continue;
        for (const b of all('button', r)) { if (gone(b)) continue; press(b); await wait(2); said(`# ${id} ${words(r.querySelector('.dl'))} = ${words(b)}`); rail(); team(); said(kept()); }
        press(all('button', r).find(b => !gone(b))); } }
    // the kit, mission by mission: every size, every man, every step
    for (const id of deep ?? kinds.values()) { P().missions.select(id); await wait(20); P().setPane('loadout'); await wait(30); if (P().pane !== 'loadout') { said(`# ${id} has no kit step`); continue; }
      for (const size of all('.lo-size').map(b => b.dataset.squad)) { press(root.querySelector(`.lo-size[data-squad="${size}"]`)); await wait(2); said(`# ${id} size ${size}`); team(); rail();
        for (const k of all('[data-kit]').map(b => b.dataset.kit)) { press(root.querySelector(`[data-kit="${k}"]`)); said(`# ${id} size ${size} kit ${k}`); team(); card(); said(kept()); } }
      const men = all('.lo-slot').length;
      for (let i = 0; i < men; i++) { const at = () => all('.lo-slot')[i]; if (!at().classList.contains('in')) { press(at()); await wait(2); } press(at()); await wait(2); press(all('.kc-set')[0]);
        said(`# ${id} man ${i}`); team(); card();
        for (const f of all('.kc-row').map(r => r.dataset.field)) for (const dir of [1, 0]) for (let n = 0; n < (dir ? 9 : 3); n++) { const r = root.querySelector(`.kc-row[data-field="${f}"]`); if (!r) break; const was = words(root.querySelector('.kc'));
          press(r.querySelectorAll('.kc-step')[dir]); said(`# ${id} man ${i} ${f} ${dir ? '+' : '-'}${n + 1}`); card(); if (n === 0) { team(); said(kept()); } if (words(root.querySelector('.kc')) === was && n > 1) break; }
        for (const slot of ['primary', 'secondary']) { press(root.querySelector(`.kc-row[data-field="${slot}"]`)); said(`# ${id} man ${i} ${slot} open`); card();
          for (const v of all(`.kc-pop[data-pop="${slot}"] .kc-pick`).map(b => b.dataset.pick)) { press(root.querySelector(`.kc-pop[data-pop="${slot}"] .kc-pick[data-pick="${v}"]`)); said(`# ${id} man ${i} ${slot} = ${v}`); card(); said(kept()); }
          press(root.querySelector(`.kc-row[data-field="${slot}"]`)); }
        // the keys: copy here, paste on the next man, back
        key('KeyC', 'c'); said(`# ${id} man ${i} copied`); team(); card(); press(all('.lo-slot')[(i + 1) % men]); key('KeyV', 'v'); said(`# ${id} man ${i} pasted on the next`); team(); card(); said(kept()); press(at()); }
      for (const k of ['KeyW', 'KeyW', 'KeyW', 'KeyD', 'KeyS', 'KeyD', 'KeyA', 'KeyS', 'Enter', 'Escape', 'KeyS', 'KeyD', 'KeyS', 'KeyS', 'KeyA', 'KeyD', 'Enter']) { if (P().pane !== 'loadout') break; if (k === 'Enter' && root.querySelector('.lo-row.sel')) continue;   /* (Deploy loads another page) */ key(k); said(`# ${id} key ${k}: pane ${P().pane} member ${P().loadout?.member} row ${P().loadout?.card?.at()}`); card(); }
      P().setPane('list'); await wait(20); }
    said('# kept'); said(kept());
    return lines.join('\n') + '\n';
  })();
}

const chrome = await Cdp.launch({ port: port + 1, dpr: 1, extraArgs: ['--no-sandbox', '--disable-gpu'] });
let code = 0;
try {
  await chrome.open(`http://localhost:${port}/`);
  for (let i = 0; i < 100 && !(await chrome.eval('!!window.__cards?.page()').catch(() => false)); i++) await new Promise(r => setTimeout(r, 100));
  await chrome.eval(`localStorage.clear(); window.__deployed = []; 1`);
  const text = await chrome.eval<string>(`(${walk.toString()})(${JSON.stringify(deep) ?? 'undefined'})`, { timeoutMs: 600000 });
  writeFileSync(out, text);
  const errors = chrome.errors.filter(e => !/404|favicon/.test(e)); for (const e of errors) console.error(e.slice(0, 300));
  console.log(`cards: ${text.split('\n').length} lines, ${errors.length} errors on the page → ${out}`); code = errors.length ? 1 : 0;
} finally { await chrome.close(); server.stop(true); }
process.exit(code);
