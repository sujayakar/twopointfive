// EVERY GPU PASS HAS A TIMER, OR A ROW THAT SAYS WHY NOT. A pass the page opens with no `timestampWrites` is in no HUD row, no recording, no sweep's pass
// columns, no price table and not in the ladder's reading: whatever it costs is nobody's. One did cost: the look's `place` pass, 1.7 ms of a frame on a
// rented card and 2 ms on an M4 Max (2026-11-05), found only when a sweep's row gave back more than every pass it could name.
// THE RULE, read off the text of src/: every `beginComputePass(` and `beginRenderPass(` is handed `timestampWrites` in its own descriptor, or its file
// and label have a row in tools/qa/passtimer-exempt.ts with the reason (a pass outside the frame's encoder, a clear on the one frame a setting flips, a
// tool's readback). A row nothing uses is red too, so the table cannot keep a pass that is gone. No pass is named here.
// What it cannot see: whether the timer is really handed at run time (a caller that passes no `ts`, a descriptor whose `timestampWrites` is undefined: the
// light cache's boot bakes run so); the headless device has no timer to ask. The rule would hold by construction if the frame's encoder were wrapped
// to hand every pass its slot by label: not done here.
//   bun run tools/qa/passtimer-check.ts   (exit 0 = all pass; PASSTIMER_BREAK=1 = the table read as empty: must fail)
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { PASS_EXEMPT, type PassExempt } from './passtimer-exempt';
const BREAK = process.env.PASSTIMER_BREAK === '1';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };

interface PassSite { at: string; kind: 'compute' | 'render'; label: string; timed: boolean }
/** is index `i` of `text` outside every string of its own line (the quotes before it on the line pair up)? A line the test cannot be sure of is taken as
 *  inside one, so that what follows is kept as code: a note read as code can only make the check red, never green */
function plain(text: string, i: number): boolean {
  const from = text.lastIndexOf('\n', i - 1) + 1, n = { "'": 0, '"': 0, '`': 0 } as Record<string, number>;
  for (let k = from; k < i; k++) { const c = text[k]; if (c === '\\') k++; else if (c in n) n[c]++; }
  return n["'"] % 2 === 0 && n['"'] % 2 === 0 && n['`'] % 2 === 0;
}
/** the text with its notes blanked, in the order a compiler meets them: a `//` to the line's end, a block note to its end; neither is one inside a string,
 *  and a block note's opening inside a line note opens nothing (`// levels` + `/*.json` is one line's note) */
function code(text: string): string {
  let out = '', i = 0;
  while (i < text.length) {
    const a = text.indexOf('//', i), b = text.indexOf('/*', i), at = a < 0 ? b : b < 0 ? a : Math.min(a, b);
    if (at < 0) break;
    if (!plain(text, at)) { out += text.slice(i, at + 2); i = at + 2; continue; }
    const end = at === a ? (text.indexOf('\n', at) < 0 ? text.length : text.indexOf('\n', at)) : (text.indexOf('*/', at + 2) < 0 ? text.length : text.indexOf('*/', at + 2) + 2);
    out += text.slice(i, at) + text.slice(at, end).replace(/[^\n]/g, ' '); i = end;
  }
  return out + text.slice(i);
}
/** the words between a call's brackets, from the index of its opening one (strings stepped over whole) */
function inside(text: string, open: number): string {
  let depth = 0;
  for (let i = open; i < text.length; i++) { const c = text[i];
    if (c === "'" || c === '"' || c === '`') { for (i++; i < text.length && text[i] !== c; i++) if (text[i] === '\\') i++; continue; }
    if (c === '(') depth++; else if (c === ')' && --depth === 0) return text.slice(open + 1, i); }
  return text.slice(open + 1);
}
/** every pass a file's text opens */
function sitesOf(at: string, raw: string): PassSite[] {
  const text = code(raw), out: PassSite[] = [];
  for (const m of text.matchAll(/\bbegin(Compute|Render)Pass\s*\(/g)) { const d = inside(text, m.index! + m[0].length - 1);
    out.push({ at, kind: m[1] === 'Compute' ? 'compute' : 'render', label: /^\s*\{\s*label\s*:\s*['"`]([^'"`]*)['"`]/.exec(d)?.[1] ?? '', timed: /\btimestampWrites\s*:/.test(d) }); }
  return out;
}
/** the sites with no timer set against the table: which have no row, and which rows are not used up */
function judge(sites: readonly PassSite[], table: readonly PassExempt[]): { bare: string[]; stale: string[]; thin: string[] } {
  const left = table.map(r => r.n ?? 1), bare: string[] = [];
  for (const s of sites) { if (s.timed) continue; const i = table.findIndex((r, k) => r.at === s.at && r.label === s.label && left[k] > 0); if (i < 0) bare.push(`${s.at}: ${s.kind} pass '${s.label}'`); else left[i]--; }
  return { bare, stale: table.filter((_, k) => left[k] > 0).map(r => `${r.at}: '${r.label}'`), thin: table.filter(r => r.why.trim().length < 20).map(r => `${r.at}: '${r.label}'`) };
}

{ // the reader and the judge, on lines of their own
  const s = sitesOf('x.ts', [
    "const a = enc.beginComputePass({ label: 'one', timestampWrites: ts('one') });",
    "const b = enc.beginComputePass({ label: 'two' });   // (a note: enc.beginComputePass({ label: 'noted' }))",
    "const c = enc.beginRenderPass({\n  label: 'three',\n  colorAttachments: [{ view: f(x, (y)), loadOp: 'clear' }], timestampWrites: ts?.('three') });",
    "const d = enc.beginRenderPass({ colorAttachments: [{ view: t.createView({ label: 'a view' }) }] }); const e = enc.beginComputePass(); const g = enc.beginComputePass(desc);",
    "/* enc.beginComputePass({ label: 'gone' }) */ const u = 'http://x'; const h = enc.beginComputePass({ label: 'after an address' }); const t = 'timestampWrites';",
    "// the rooms come from levels/*.json",
    "const k = enc.beginComputePass({ label: 'after a star in a note' });   /** a note that ends the star's */",
    "const l = enc.beginComputePass({ label: 'a word is no timer', why: 'timestampWrites' });"].join('\n'));
  const got = s.map(x => `${x.kind[0]}:${x.label}:${x.timed ? 'timed' : 'bare'}`).join(' ');
  check(got === 'c:one:timed c:two:bare r:three:timed r::bare c::bare c::bare c:after an address:bare c:after a star in a note:bare c:a word is no timer:bare',
    `the reader finds each pass a text opens, its own label (not a view's inside it) and whether its own descriptor has a timer's key, and no pass a note speaks of; a star in a line's note opens no note (${got})`);
  const j = judge(s.slice(0, 7), [{ at: 'x.ts', label: 'two', why: 'a reason long enough to be one' }, { at: 'x.ts', label: '', n: 2, why: 'two passes with no label in one file' }, { at: 'x.ts', label: 'gone', why: 'short' }]);
  check(j.bare.join(' | ') === "x.ts: compute pass '' | x.ts: compute pass 'after an address'" && j.stale.join() === "x.ts: 'gone'" && j.thin.join() === "x.ts: 'gone'",
    `the judge: a pass with no timer and no row is bare (a row covers as many as it says), a row no pass uses is stale, and a row must say why (bare: ${j.bare.join(' | ')} · stale: ${j.stale.join()} · thin: ${j.thin.join()})`);
}
{ // the tree
  const root = join(import.meta.dir, '../../src'), sites: PassSite[] = [];
  const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.ts$/.test(e.name)) sites.push(...sitesOf(p.slice(root.length + 1), readFileSync(p, 'utf8'))); } };
  walk(root);
  const j = judge(sites, BREAK ? [] : PASS_EXEMPT), timed = sites.filter(s => s.timed).length;
  check(sites.length > 20 && timed > 10, `the page's passes are found: ${sites.length} opened under src/, ${timed} of them with a timer`);
  check(j.bare.length === 0, `every pass has a timer or a row that says why not${j.bare.length ? ` (NO TIMER AND NO ROW: ${j.bare.join(' · ')}: hand its descriptor timestampWrites, asked of gpu/timer.ts only where the pass is really opened, or give it a row in tools/qa/passtimer-exempt.ts)` : ''}`);
  check(j.stale.length === 0 && j.thin.length === 0, `every row of the table is used by a pass that is there, and says why${j.stale.length ? ` (no such pass now: ${j.stale.join(' · ')})` : ''}${j.thin.length ? ` (no reason: ${j.thin.join(' · ')})` : ''}`);
}
console.log(red ? `\n${red} FAILED` : '\nall pass');
process.exit(red ? 1 : 0);
