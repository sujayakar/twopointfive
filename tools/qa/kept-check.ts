// HOW KEPT A PLACE IS, ONE WORD A LEVEL: the check behind a level file's `meta.kept`, scene/look.ts lookKeptOf, look.wgsl lookKept and markings.wgsl's old
// paint (docs/agents/render.md has the rule at length).
//   bun run tools/qa/probes.ts kept        (family `render`)
//   KEPT_BREAK=1     the weather's rules and a mark's wear do not ask the word (the second writing below leaves it out): must FAIL
// Until 11-05 whether a surface looked looked-after was said stuff by stuff (a stuff with no `fades` is "a place somebody keeps"), and every painted mark
// was drawn as old paint by one number for the whole game. A level that should read as cleaned last night had to say every wall again as another stuff, and
// its signs still drew rubbed.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 is the level file's own code, run. Line 2 is the page's rule, run. Line 3 holds the rule BY ITS
// TEXT in the shaders and the writer of their uniform. Line 4 is a second writing of the two places the word acts, held to the shader by line 3's pins:
// whether it is the first is as good as those. What a kept place LOOKS like is a page's to show.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function keptCheck(): Promise<boolean> {
  const BRK = process.env.KEPT_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== kept: how kept a place is, one word a level${BRK ? ' (nothing asks the word: must fail)' : ''} ===`);
  const text = (f: string) => readFileSync(resolve(ROOT, f), 'utf8'), count = (t: string, s: string) => t.split(s).length - 1;
  // 1. the word in the file: there and back
  { const { parseLevelData, validateLevelData, canonicalLevelData, serializeLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
    const { loadLevel, levelToData } = await import(`${ROOT}/src/scene/levelLoad.ts`);
    const base = parseLevelData(text('levels/white-roof.json')) as any, said = (v: unknown) => ({ ...base, meta: { ...base.meta, kept: v } });
    const plain = loadLevel(base) as any, d = said(0.7), lv = loadLevel(d) as any;
    const back = parseLevelData(serializeLevelData(canonicalLevelData(d))) as any;
    const again = levelToData(lv, { name: d.name, meta: { notes: 'x' } }) as any;
    const errs = (v: unknown) => (validateLevelData(said(v)) as string[]).filter(e => e.includes('meta.kept')).length;
    const ok = !('kept' in plain) && !('kept' in (canonicalLevelData(base) as any).meta) && lv.kept === 0.7 && back.meta.kept === 0.7 && again.meta.kept === 0.7 && again.meta.notes === 'x'
      && errs(0.7) === 0 && errs(0) === 0 && errs(1) === 0 && errs(1.5) === 1 && errs(-0.1) === 1 && errs('1') === 1;
    verdict(ok, 'a level file says how kept it is in meta.kept, a number from 0 to 1: it loads to the level, is written back, and is kept by the canonical form; a file without it has no such key, and a number outside 0 to 1 or a string is an error',
      `no word: level ${'kept' in plain ? 'has a key' : 'no key'} · said 0.7: level ${lv.kept}, canonical ${back.meta.kept}, written back ${again.meta.kept} · errors at 0.7 / 0 / 1 / 1.5 / -0.1 / "1": ${[0.7, 0, 1, 1.5, -0.1, '1'].map(errs).join(' ')}`); }
  // 2. the page's rule
  { const { lookKeptOf } = await import(`${ROOT}/src/scene/look.ts`);
    const rows: [number | undefined, string | null, number][] = [[undefined, null, 0], [0.7, null, 0.7], [0.7, '0', 0], [undefined, '1', 1], [0.4, '2', 1], [0.4, '-3', 0], [0.4, 'x', 0.4], [0.4, '', 0.4], [NaN, null, 0]];
    const got = rows.map(r => lookKeptOf(r[0], r[1]));
    verdict(rows.every((r, i) => got[i] === r[2]), 'the frame takes the address\'s &kept for the visit, else the level\'s own word, else 0; held to 0 to 1; a word that is no number leaves the level\'s',
      rows.map((r, i) => `(${r[0]}, ${r[1] === null ? 'none' : `'${r[1]}'`}) → ${got[i]}`).join(' · ')); }
  // 3. the text: one writer, one reader, and every place the word acts
  const look = text('src/render/shaders/look.wgsl'), marks = text('src/render/shaders/markings.wgsl'), gb = text('src/render/gbuffer.ts'), main = text('src/main.ts');
  const standing = (() => { const i = look.indexOf('  // ---- WEATHER on anything that stands on the ground or lies in the open'), j = look.indexOf('  // ---- USE on a wall somebody keeps'); return i < 0 || j < i ? '' : look.slice(i, j); })();
  { const pins: [string, boolean][] = [
      ['the uniform\'s writer puts the word at c.x', gb.includes('k.cavity * c, k.size, lookConfig.kept, 0, 0, 0,')],
      ['the page hands the level\'s word over and writes the uniform', main.includes("lookConfig.kept = lookKeptOf(level.kept, q.get('kept'));") && /lookConfig\.kept = lookKeptOf[^\n]*\n  engine\.gbuffer\.setLook\(\);/.test(main)],
      ['look.wgsl reads it in one place', count(look, 'lookCfg.c.x') === 1 && look.includes('fn lookKept() -> f32 { return clamp(lookCfg.c.x, 0.0, 1.0); }')],
      ['the weather\'s amount is the panel\'s row × how far the place is left to it', count(look, 'let kw = k.w * (1.0 - lookKept());') === 1],
      ['the weather on what stands asks that amount and never the row alone', standing.length > 0 && count(standing, 'k.w') === 0 && count(standing, 'kw') >= 7 && standing.includes('if (kw > 0.0 && cls != 0u && wild > 0.0) {')],
      ['the rubbish at a wall\'s foot asks it, the trodden lane beside it does not', look.includes('wm.y * (0.55 + 0.9 * rag)) * kw;') && look.includes('clamp(wm.x * (0.5 + rag), 0.0, 1.0) * k.w * select(')],
      ['the rules of use still ask the row', look.includes('keep.w * min(k.w, 1.0) * (1.0 - wild)') && look.includes('let knock = stretch * (1.0 - smoothstep(high - max(0.008, fp), high, wp.y)) * keep.w * min(k.w, 1.0);')],
      ['a mark is handed how far its paint is old', look.includes('fn lookMarkSoft() -> vec2f { return vec2f(lookCfg.g.x, 1.0 - lookKept()); }') && gb.includes('fn lookMarkSoft() -> vec2f { return vec2f(0.0, 1.0); }')],
      ['old paint is rubbed thin and gone into its wall by that much', marks.includes('let keep = 1.0 - 0.85 * min(age, 1.0) * worn * soft2.y;') && marks.includes('fn markTone(soft: vec2f, near: f32) -> f32 { return 1.0 - 0.18 * min(soft.x, 2.0) * soft.y * near; }') && marks.includes('if (soft.x > 0.0) {')],
      ['the address\'s word is a visit\'s and the look page\'s', /VISIT_FLAGS = \[[^\]]*'kept'/.test(text('src/ui/replaykey.ts')) && /LOOK_PAGE_WORDS[^\n]*'kept'/.test(text('src/scene/look.ts'))],
    ];
    const bad = pins.filter(p => !p[1]).map(p => p[0]);
    verdict(bad.length === 0, 'the word has one writer and one reader, and the weather on what stands, the rubbish at a wall\'s foot and a mark\'s old paint ask it; the rules of use and a trodden lane do not', bad.length ? `NOT HELD: ${bad.join('; ')}` : `${pins.length} pins hold`); }
  // 4. a second writing of the two places it acts (held to the shader by line 3)
  { const kw = (row: number, kept: number) => row * (BRK ? 1 : 1 - kept);
    const blotch = (kept: number, fades: number, lying: boolean) => kw(1, kept) * (lying ? 1.7 : 1) * fades * 0.25;   // (the most a slow blotch moves a paint either way: 0.3 × ½ + 0.2 × ½)
    const paint = (kept: number, soft: number, worn: number) => 1 - 0.85 * Math.min(soft, 1) * worn * (BRK ? 1 : 1 - kept);
    const tone = (kept: number, soft: number) => 1 - 0.18 * Math.min(soft, 2) * (BRK ? 1 : 1 - kept);
    const r = (v: number) => +v.toFixed(3);
    const b = [0, 0.5, 1].map(k => r(blotch(k, 1, false))), g = [0, 0.5, 1].map(k => r(blotch(k, 1, true))), f = [0, 0.5, 1].map(k => r(blotch(k, 0.3, false)));
    const p = [0, 0.5, 1].map(k => r(paint(k, 1, 1))), t = [0, 0.5, 1].map(k => r(tone(k, 1)));
    const ok = b[0] === 0.25 && b[1] === 0.125 && b[2] === 0 && g[0] === 0.425 && g[2] === 0 && f[0] === 0.075 && f[2] === 0 && p[0] === 0.15 && p[1] === 0.575 && p[2] === 1 && t[0] === 0.82 && t[1] === 0.91 && t[2] === 1;
    verdict(ok, 'left to the weather a wall\'s paint drifts a quarter either way and the paint of a mark is rubbed to 0.15 of itself where it is worn and stands at 0.82 of its colour; kept, the wall is one shade and the mark whole and its own colour; half way is half',
      `kept 0 / 0.5 / 1: a wall's blotch ${b.join(' / ')} · the ground's ${g.join(' / ')} · a factory finish's ${f.join(' / ')} · a worn mark's paint ${p.join(' / ')} · its tone ${t.join(' / ')}`); }
  console.log(`=== kept: ${all ? 'ALL PASS' : 'FAILED'} ===`);
  return all;
}
if (import.meta.main) process.exit((await keptCheck()) ? 0 : 1);
