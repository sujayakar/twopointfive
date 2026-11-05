// HOW KEPT A PLACE IS, ONE WORD A LEVEL: the check behind a level file's `meta.kept`, scene/look.ts lookKeptOf, look.wgsl lookKept and markings.wgsl's old
// paint (docs/agents/render.md has the rule at length).
//   bun run tools/qa/probes.ts kept        (family `render`)
//   KEPT_BREAK=1     the weather's rules and a mark's age do not ask the word (the second writing below leaves it out), and a look's second file is not
//                    given the word of the level it dresses: must FAIL
// Until 11-05 whether a surface looked looked-after was said stuff by stuff (a stuff with no `fades` is "a place somebody keeps"), and every painted mark
// was drawn as old paint by one number for the whole game. A level that should read as cleaned last night had to say every wall again as another stuff, and
// its signs still drew rubbed.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 is the level file's own code, run. Line 2 is the page's rule, run. Line 3 holds the rule BY ITS
// TEXT in the shaders, the writer of their uniform, the page and the exporter. Line 4 is a second writing of the places the word acts, held to the shader
// by line 3's pins: whether it is the first is as good as those. Line 5 is the exporter's rule for a look's second file, written again and run on the
// generators as they stand. What a kept place LOOKS like is a page's to show.
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
    const base = parseLevelData(text('levels/white-roof.json')) as any; { const m = { ...base.meta }; delete m.kept; base.meta = m; }   // (a level to stand on, whatever it says of itself today)
    const said = (v: unknown) => ({ ...base, meta: { ...base.meta, kept: v } });
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
    const rows: [number | undefined, string | null, number][] = [[undefined, null, 0], [0.7, null, 0.7], [0.7, '0', 0], [undefined, '1', 1], [0.4, '2', 1], [0.4, '-3', 0], [0.4, 'x', 0.4], [0.4, '', 0.4], [0.4, ' ', 0.4], [0.4, ' 1 ', 1], [NaN, null, 0]];
    const got = rows.map(r => lookKeptOf(r[0], r[1]));
    verdict(rows.every((r, i) => got[i] === r[2]), 'the frame takes the address\'s &kept for the visit, else the level\'s own word, else 0; held to 0 to 1; a word that is no number, empty or blank leaves the level\'s',
      rows.map((r, i) => `(${r[0]}, ${r[1] === null ? 'none' : `'${r[1]}'`}) → ${got[i]}`).join(' · ')); }
  // 3. the text: one writer, one reader, and every place the word acts
  const look = text('src/render/shaders/look.wgsl'), marks = text('src/render/shaders/markings.wgsl'), gb = text('src/render/gbuffer.ts'), main = text('src/main.ts'), exp = text('tools/levels/export.ts');
  const between = (a: string, b: string) => { const i = look.indexOf(a), j = look.indexOf(b); return i < 0 || j < i ? '' : look.slice(i, j); };
  const surface = between('fn lookSurface(', '  // ---- USE on a wall somebody keeps'), standing = between('  // ---- WEATHER on anything that stands on the ground or lies in the open', '  // ---- USE on a wall somebody keeps');
  { const pins: [string, boolean][] = [
      ['the uniform\'s writer puts the word at c.x', gb.includes('k.cavity * c, k.size, lookConfig.kept, 0, 0, 0,')],
      ['every way a level enters the page hands its word over and writes the uniform', count(main, "lookConfig.kept = lookKeptOf(level.kept, q.get('kept'));") === 1 && /function hostLevel\([^\n]*\n  lookConfig\.kept = lookKeptOf[^\n]*\n  engine\.gbuffer\.setLook\(\);/.test(main) && count(main, 'hostLevel(engine, level, q);') === 2 && count(main, 'populateHost(engine, level)') === 1],
      ['look.wgsl reads it in one place', count(look, 'lookCfg.c.x') === 1 && look.includes('fn lookKept() -> f32 { return clamp(lookCfg.c.x, 0.0, 1.0); }')],
      ['a surface asks it once: how far the place is left to the weather', count(surface, 'lookKept()') === 1 && surface.includes('let left = 1.0 - lookKept();')],
      ['how wild a surface is is its stuff\'s word × the place\'s', surface.includes('let wildS = step(0.001, keep.y);') && surface.includes('let wild = wildS * left;') && surface.includes('let wildH = select(0.0, wild, cls != 0u);')],
      ['the weather\'s amount is the panel\'s row × how far the place is left to it', count(look, 'let kw = k.w * left;') === 1],
      ['the weather on what stands asks that amount and never the row alone', standing.length > 0 && count(standing, 'k.w') === 0 && count(standing, 'kw') >= 7 && standing.includes('if (kw > 0.0 && cls != 0u && wildS > 0.0) {')],
      ['the rubbish at a wall\'s foot asks it, the trodden lane beside it does not', look.includes('wm.y * (0.55 + 0.9 * rag)) * kw;') && look.includes('clamp(wm.x * (0.5 + rag), 0.0, 1.0) * k.w * select(')],
      ['what the weather gives up the kept rules take: the detail layer\'s dust band and shoe marks, the No repeat row\'s drift', look.includes('vec2f(lookWay.x, lookWay.y * (1.0 - wildH))') && look.includes('&& stands && wildH < 1.0) {') && look.includes('let t = 1.0 - wildH;') && look.includes('if (k2.x > 0.0 && wild < 1.0) { mult *= 1.0 + 0.06 * min(k2.x, 2.0) * (1.0 - wild) * (lookSlow(')],
      ['a mud wall\'s fallen render shrinks as the place is kept, and its plinth does not', look.includes('let thr = thr0 + (1.2 - thr0) * (1.0 - left);') && look.includes('let line = (0.34 + 0.4 * detailNoise(vec2f(uv.x * 0.31, 7.7)) + 0.1 * detailNoise(vec2f(uv.x * 1.9, 2.2))) * min(km, 1.0);')],
      ['the rules of use go by the stuff and the row, not the place', look.includes('keep.w * min(k.w, 1.0) * (1.0 - wildS)') && look.includes('let knock = stretch * (1.0 - smoothstep(high - max(0.008, fp), high, wp.y)) * keep.w * min(k.w, 1.0);') && look.includes('if (k.w > 0.0 && cls != 0u && wildS == 0.0 && keep.w > 0.0 && !lying && stands && wp.y < 0.3) {')],
      ['a mark is handed how far its paint is old', look.includes('fn lookMarkSoft() -> vec2f { return vec2f(lookCfg.g.x, 1.0 - lookKept()); }') && gb.includes('fn lookMarkSoft() -> vec2f { return vec2f(0.0, 1.0); }')],
      ['old paint is rubbed thin by the years by that much and by feet whatever it is, and gone into its wall by that much', marks.includes('- (0.72 - 0.12 * age * soft2.y - 0.16 * trod));') && marks.includes('let keep = 1.0 - 0.85 * min(age, 1.0) * worn * max(soft2.y, min(trod, 1.0));') && marks.includes('fn markTone(soft: vec2f, near: f32) -> f32 { return 1.0 - 0.18 * min(soft.x, 2.0) * soft.y * near; }') && marks.includes('if (soft.x > 0.0) {')],
      ['the address\'s word is a visit\'s and the look page\'s', /VISIT_FLAGS = \[[^\]]*'kept'/.test(text('src/ui/replaykey.ts')) && /LOOK_PAGE_WORDS[^\n]*'kept'/.test(text('src/scene/look.ts'))],
      ['the exporter gives a generator\'s word to the level, and a look\'s second file the word of the level it dresses', exp.includes('{ const kept = gen.KEPT ?? (gen.DRESS_AS ? (await import(resolve(import.meta.dir, `${gen.DRESS_AS}.ts`))).KEPT : undefined);\n  if (kept !== undefined) level.kept = kept; }')],
      ['the file\'s writer takes the word from the level', text('src/scene/levelLoad.ts').includes("...(level.kept !== undefined ? { kept: level.kept } : {}) },")],
    ];
    const bad = pins.filter(p => !p[1]).map(p => p[0]);
    verdict(bad.length === 0, 'the word has one writer and one reader; the weather on what stands, the rubbish at a wall\'s foot, a mud wall\'s fallen render and the age of a mark\'s paint ask it, and what the weather gives up the kept rules take; the rules of use, a trodden lane and paint trodden off a floor do not ask it; every way a level enters the page hands it over', bad.length ? `NOT HELD: ${bad.join('; ')}` : `${pins.length} pins hold`); }
  // 4. a second writing of the places it acts (held to the shader by line 3)
  { const left = (kept: number) => BRK ? 1 : 1 - kept;
    const blotch = (kept: number, fades: number, lying: boolean) => left(kept) * (lying ? 1.7 : 1) * fades * 0.25;   // (the most a slow blotch moves a paint either way: 0.3 × ½ + 0.2 × ½)
    const wild = (kept: number, stuffWild: 0 | 1) => stuffWild * left(kept), drift = (kept: number, stuffWild: 0 | 1) => 0.03 * (1 - wild(kept, stuffWild));   // (the No repeat row at 1: 0.06 × ½ either way)
    const paint = (kept: number, soft: number, worn: number, trod: number) => 1 - 0.85 * Math.min(soft, 1) * worn * Math.max(left(kept), Math.min(trod, 1));
    const tone = (kept: number, soft: number) => 1 - 0.18 * Math.min(soft, 2) * left(kept);
    const r = (v: number) => +v.toFixed(3), K = [0, 0.5, 1];
    const b = K.map(k => r(blotch(k, 1, false))), g = K.map(k => r(blotch(k, 1, true))), f = K.map(k => r(blotch(k, 0.3, false)));
    const w = K.map(k => r(wild(k, 1))), w0 = K.map(k => r(wild(k, 0))), d = K.map(k => r(drift(k, 1))), d0 = K.map(k => r(drift(k, 0)));
    const p = K.map(k => r(paint(k, 1, 1, 0))), pt = K.map(k => r(paint(k, 1, 1, 0.6))), t = K.map(k => r(tone(k, 1)));
    const ok = b[0] === 0.25 && b[1] === 0.125 && b[2] === 0 && g[0] === 0.425 && g[2] === 0 && f[0] === 0.075 && f[2] === 0
      && w[0] === 1 && w[1] === 0.5 && w[2] === 0 && w0.every(v => v === 0) && d[0] === 0 && d[1] === 0.015 && d[2] === 0.03 && d0.every(v => v === 0.03)
      && p[0] === 0.15 && p[1] === 0.575 && p[2] === 1 && pt[0] === 0.15 && pt[1] === 0.49 && pt[2] === 0.49 && t[0] === 0.82 && t[1] === 0.91 && t[2] === 1;
    verdict(ok, 'left to the weather a wall\'s paint drifts a quarter either way, and the paint of a mark is rubbed to 0.15 of itself where it is worn and stands at 0.82 of its colour; kept, the wall is one shade but for the three in a hundred no forty metres are without, and the mark is whole and its own colour on a wall and still rubbed to half where its floor is walked; half way is half',
      `kept 0 / 0.5 / 1: a wall's blotch ${b.join(' / ')} · the ground's ${g.join(' / ')} · a factory finish's ${f.join(' / ')} · how far the weather's rules answer for a weathered stuff ${w.join(' / ')} (a kept stuff ${w0.join(' / ')}) · its drift ${d.join(' / ')} (a kept stuff's ${d0.join(' / ')}) · a worn mark's paint ${p.join(' / ')} · the same where feet go ${pt.join(' / ')} · its tone ${t.join(' / ')}`); }
  // 5. a look's second file is the same place: it has the word of the level it dresses unless it says its own (the exporter's rule, written again and run
  //    on the generators as they stand; line 3 pins the exporter's own line)
  { const { readdirSync } = await import('node:fs'); const dir = resolve(ROOT, 'tools/levels');
    const keptOf = async (g: any): Promise<number | undefined> => g.KEPT ?? (g.DRESS_AS && !BRK ? (await import(`${dir}/${g.DRESS_AS}.ts`)).KEPT : undefined);
    const twins: string[] = [], bad: string[] = []; let say = 0;
    for (const f of readdirSync(dir).filter(f => /-gwot\.ts$/.test(f)).sort()) { const g = await import(`${dir}/${f}`); if (!g.DRESS_AS) continue; twins.push(f.replace(/\.ts$/, ''));
      const base = await import(`${dir}/${g.DRESS_AS}.ts`), own = await keptOf(g), want = g.KEPT ?? base.KEPT; if (want !== undefined) say++;
      if (own !== want) bad.push(`${f}: ${own} for ${want}`); }
    // … and the rule on a pair made here, so the line fails when the rule is taken out and no generator says the word yet
    const rule = async (twin: { KEPT?: number; DRESS_AS?: string }, base: { KEPT?: number }) => twin.KEPT ?? (twin.DRESS_AS && !BRK ? base.KEPT : undefined);
    const made = [await rule({ DRESS_AS: 'x' }, { KEPT: 0.8 }), await rule({ DRESS_AS: 'x', KEPT: 0.2 }, { KEPT: 0.8 }), await rule({ DRESS_AS: 'x' }, {}), await rule({}, { KEPT: 0.8 })];
    const ok = twins.length > 0 && bad.length === 0 && made[0] === 0.8 && made[1] === 0.2 && made[2] === undefined && made[3] === undefined;
    verdict(ok, 'a look\'s second file has the word of the level it dresses unless it says its own; a level that dresses nothing has only its own',
      `${twins.length} second files, ${say} with a word today${bad.length ? ` · WRONG: ${bad.join('; ')}` : ''} · a pair made here: the plain level's 0.8 → ${made[0]} · its own 0.2 over it → ${made[1]} · neither → ${made[2]} · no level dressed → ${made[3]}`); }
  console.log(`=== kept: ${all ? 'ALL PASS' : 'FAILED'} ===`);
  return all;
}
if (import.meta.main) process.exit((await keptCheck()) ? 0 : 1);
