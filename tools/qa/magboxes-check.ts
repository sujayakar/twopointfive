// 'magboxes' — THE BOX COUNTS THE ROUNDS IN THE GUN OVER WHAT ITS MAGAZINE HOLDS, WITH A BOX FOR EACH SPARE MAGAZINE BESIDE IT (SLOTSETUP.mags,
// src/game/slotsetup.ts; drawn by src/ui/handbox.ts). The maintainer, of the box that read 31 / 60 at a mission's start: "we should say 31/30 and then two
// spare magazines on the side".
//   bun run tools/qa/magboxes-check.ts        (also `probes.ts magboxes`)
//   box       the rifle up at a mission's start: the count is the rounds in the gun over the magazine's size and a box stands for each spare, full. Rounds
//             fired come off the first number alone. After a reload the fullest spare is in the gun, the magazine that came out is a box as full as it was,
//             and the boxes are still one a magazine. The pistol's box reads the same way.
// SINCE 2026-11-07 the key and the slot setup's own switches are settled on and nothing here writes them: the two-line box (the setup's panel put off),
// "the key off: the box as it was", the recording from before the key and the whole of `harness` (the stands, the page's ?magboxes=0) are out.
// The rule itself is held natively, red where it breaks: rust/game_start/tests/kit_view.rs, the_box_counts_the_rounds_over_the_magazine_and_a_box_a_spare. This reads what
// the page draws of it.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO PAGE: the box is drawn by the real overlay code on the probes' stand-in elements (fn2000-probes.ts hudText) and read as text, each
// box's drawn height with it. How it looks is not seen here.
import { ROOT } from './headless.ts';

const { stage } = await import('./fn2000-probes.ts');
const { Overlay } = await import(`${ROOT}/src/ui/overlay.ts`);
/** the kit column drawn by the real overlay code on stand-in elements and read as text (fn2000-probes.ts hudText, with a box's height kept: `b[80%]`) */
function hudText(game: any): string {
  const mkEl = (tag: string): any => { const el: any = { tag, className: '', kids: [] as any[], text: '', style: {}, title: '', appendChild(c: any) { el.kids.push(c); return c; } }; Object.defineProperty(el, 'textContent', { get() { return el.text + el.kids.map((k: any) => k.textContent).join(''); }, set(v: string) { el.text = v ?? ''; el.kids = []; } }); return el; };
  const ser = (el: any): string => `${el.tag}${el.className.trim() ? '.' + el.className.trim().split(/\s+/).join('.') : ''}${el.style.height ? `[${el.style.height}]` : ''}${el.text ? `{${el.text}}` : ''}${el.kids.length ? `(${el.kids.map(ser).join(' ')})` : ''}`;
  const G = globalThis as any, had = 'document' in G, prev = G.document; G.document = { createElement: (t: string) => mkEl(t) };
  try { const ctx = { equip: mkEl('div'), equipKey: '' }; (Overlay.prototype as any).updateEquip.call(ctx, game.view.kit); return ctx.equip.kids.map(ser).join(' | '); }
  finally { if (had) G.document = prev; else delete G.document; }
}
const DT = 1 / 60, AHEAD: [number, number, number] = [15, 1.2, 11.5];
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => T | Promise<T>): Promise<T> { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } }
const wait = (W: any, secs: number) => { for (let i = 0; i < Math.round(secs * 60); i++) { W.cursorAt(AHEAD); W.step(DT); } };
const key = (W: any, code: string, n = 1) => { W.input.pressed.add(code); for (let i = 0; i < n; i++) { W.cursorAt(AHEAD); W.step(DT); } };
/** what the box says of the gun in his hands: the two numbers and how full each spare's box is (a hundred = a full magazine) */
/** what the box says of the gun in his hands: the two numbers and each spare's box as DRAWN, in a hundred (null: no box of spares at all) */
const read = (g: any) => { const t = hudText(g), m = /span\.hb-n(?:\.dry)?\(b\{(\d+)\} i(\.of)?\{\/ ?(\d+)\}\)/.exec(t);
  return m ? { inGun: +m[1], of: m[2] === '.of', over: +m[3], drawn: /span\.mags/.test(t) ? [...t.matchAll(/span\.mag\(b\[(\d+)%\]\)/g)].map(x => +x[1]) : null as number[] | null } : null; };
const want = (f: any) => f.spareSorted().map((r: number) => Math.round((r / f.def.magCapacity) * 100)), same = (a: number[] | null, b: number[]) => !!a && a.length === b.length && a.every((v, i) => v === b[i]);
const up = async () => quiet(async () => { const W: any = await stage(4120); const g = W.game, pl = g.player; g.aiEnabled = false; g.infiniteAmmo = false;
  for (let i = 0; i < 90 && (pl.equipT > 0 || !pl.rifleUp); i++) wait(W, DT); wait(W, 0.5); return { W, g, pl, rf: pl.rifle }; });

async function box() {
  console.log('\n=== box: the gun\'s rounds over its magazine\'s size, a box a spare ===');
  const { W, g, pl, rf } = await up(); const cap = rf.def.magCapacity, n0 = rf.spare.length;
  { const R = read(g);
    verdict(!!R && R.of && R.inGun === rf.roundsReady && R.over === cap && n0 > 0 && same(R.drawn, Array(n0).fill(100)), 'the rifle at the start: rounds in the gun over what the magazine holds, and a full box for each spare magazine', R ? `${R.inGun}/${R.over} · boxes ${R.drawn?.join(' ') ?? 'none'} (he carries ${n0} spare, each ${cap})` : `no count read: ${hudText(g).slice(0, 200)}`); }
  { await quiet(() => { for (let i = 0; i < 6; i++) { W.cursorAt(AHEAD); W.input.clicked |= 1; W.input.buttons |= 1; W.step(DT); W.input.buttons &= ~1; wait(W, 0.3); } }); const R = read(g), gone = cap + 1 - rf.roundsReady;
    verdict(!!R && gone > 0 && R.inGun === rf.roundsReady && R.over === cap && same(R.drawn, Array(n0).fill(100)), 'rounds fired come off the first number alone: the magazine\'s size and the boxes stand', R ? `${gone} fired: ${R.inGun}/${R.over} · boxes ${R.drawn?.join(' ') ?? 'none'}` : 'no count read'); }
  { const was = rf.mag; await quiet(() => { key(W, 'KeyR'); let n = 0; while ((rf.reloading || n < 5) && n++ < 400) wait(W, DT); wait(W, 0.2); }); const R = read(g), part = Math.round((was / cap) * 100);
    verdict(!!R && R.inGun === cap + 1 && R.over === cap && same(R.drawn, want(rf)) && R.drawn!.length === n0 && R.drawn![0] === 100 && R.drawn![n0 - 1] === part && part < 100, 'after a reload the full magazine is in the gun and the one that came out is still a box, drawn as full as it was, the fullest first', R ? `${R.inGun}/${R.over} · boxes ${R.drawn?.join(' ') ?? 'none'} (the one that came out held ${was} of ${cap})` : `no count read (reloading ${rf.reloading})`); }
  { await quiet(() => { key(W, 'Digit1', 90); wait(W, 0.4); }); const R = read(g), p = pl.pistol;
    verdict(pl.slot === 'pistol' && !!R && R.of && R.inGun === p.roundsReady && R.over === p.def.magCapacity && same(R.drawn, want(p)) && p.spare.length > 0, 'the pistol\'s box reads the same way', R ? `${R.inGun}/${R.over} · boxes ${R.drawn?.join(' ') ?? 'none'} (he carries ${p.spare.length})` : `no count read (slot ${pl.slot})`); }
}

/** the whole card (probes.ts 'magboxes'): true = every check passed. The knobs are put back as they were found. */
export async function magBoxesCheck(): Promise<boolean> {
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`); const was = knobsCopy(), rnd = Math.random; allPass = true;
  try { await box(); }
  finally { knobsBack(was); Math.random = rnd; }
  console.log(`\n=== magboxes: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit(await magBoxesCheck() ? 0 : 1);
