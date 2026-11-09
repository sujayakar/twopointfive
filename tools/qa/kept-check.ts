// HOW KEPT A PLACE IS, ONE WORD A LEVEL: the check behind a level file's `meta.kept`, scene/look.ts lookKeptOf, look.wgsl lookKept and markings.wgsl's old
// paint (docs/agents/render.md has the rule at length).
//   bun run tools/qa/checks.ts kept
//   KEPT_BREAK=1     the weather's rules and a mark's age do not ask the word (the second writing below leaves it out), and the level a look's second
//                    file dresses cannot be read when its word is asked: must FAIL
// Until 11-05 whether a surface looked looked-after was said stuff by stuff (a stuff with no `fades` is "a place somebody keeps"), and every painted mark
// was drawn as old paint by one number for the whole game. A level that should read as cleaned last night had to say every wall again as another stuff, and
// its signs still drew rubbed.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 is the level file's own code, run. Line 2 is the page's rule, run. Line 3 holds the shaders BY THEIR
// TEXT to line 4, a second writing of the places the word acts: whether it is the first is as good as those. Line 5 runs on every
// generator against its level file. What a kept place LOOKS like is a page's to show.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './here.ts';
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
    const theirs = { notes: 'x', kept: 0.1, markTexel: 0.05 } as { notes: string };   // (a caller's meta with the two words the Level owns: both are dropped)
    const again = levelToData(lv, { name: d.name, meta: theirs }) as any, none = levelToData(plain, { name: d.name, meta: theirs }) as any;   // (the Level is the one source: a caller's own word is dropped)
    const errs = (v: unknown) => (validateLevelData(said(v)) as string[]).filter(e => e.includes('meta.kept')).length;
    const ok = !('kept' in plain) && !('kept' in (canonicalLevelData(base) as any).meta) && lv.kept === 0.7 && back.meta.kept === 0.7 && again.meta.kept === 0.7 && again.meta.notes === 'x' && !('kept' in none.meta) && none.meta.notes === 'x' && again.meta.markTexel === base.meta.markTexel && none.meta.markTexel === base.meta.markTexel
      && errs(0.7) === 0 && errs(0) === 0 && errs(1) === 0 && errs(1.5) === 1 && errs(-0.1) === 1 && errs('1') === 1;
    verdict(ok, 'a level file says how kept it is in meta.kept, a number from 0 to 1: it loads to the level, is written back from the level and from nothing else, and is kept by the canonical form; a file without it has no such key, and a number outside 0 to 1 or a string is an error',
      `no word: level ${'kept' in plain ? 'has a key' : 'no key'} · said 0.7: level ${lv.kept}, canonical ${back.meta.kept}, written back ${again.meta.kept} over a caller's 0.1 · a level with no word and a caller's 0.1: ${'kept' in none.meta ? none.meta.kept : 'no key'} · errors at 0.7 / 0 / 1 / 1.5 / -0.1 / "1": ${[0.7, 0, 1, 1.5, -0.1, '1'].map(errs).join(' ')}`); }
  // 2. the page's rule
  { const { lookKeptOf } = await import(`${ROOT}/src/scene/look.ts`);
    const rows: [number | undefined, string | null, number][] = [[undefined, null, 0], [0.7, null, 0.7], [0.7, '0', 0], [undefined, '1', 1], [0.4, '2', 1], [0.4, '-3', 0], [0.4, 'x', 0.4], [0.4, '', 0.4], [0.4, ' ', 0.4], [0.4, ' 1 ', 1], [NaN, null, 0]];
    const got = rows.map(r => lookKeptOf(r[0], r[1]));
    verdict(rows.every((r, i) => got[i] === r[2]), 'the frame takes the address\'s &kept for the visit, else the level\'s own word, else 0; held to 0 to 1; a word that is no number, empty or blank leaves the level\'s',
      rows.map((r, i) => `(${r[0]}, ${r[1] === null ? 'none' : `'${r[1]}'`}) → ${got[i]}`).join(' · ')); }
  // 3. the text: one writer, one reader, and every place the word acts
  const look = text('src/render/shaders/look.wgsl'), marks = text('src/render/shaders/markings.wgsl');
  const between = (a: string, b: string) => { const i = look.indexOf(a), j = look.indexOf(b); return i < 0 || j < i ? '' : look.slice(i, j); };
  const surface = between('fn lookSurface(', '  // ---- USE on a wall somebody keeps'), standing = between('  // ---- WEATHER on anything that stands on the ground or lies in the open', '  // ---- USE on a wall somebody keeps');
  { const pins: [string, boolean][] = [
      ['how far a surface is left to the weather', surface.includes('let left = 1.0 - lookKept();')],
      ['how wild a surface is is its stuff\'s word × the place\'s, and nothing goes by the stuff alone but as a gate', surface.includes('let wildS = step(0.001, keep.y);') && surface.includes('let wild = select(0.0, wildS * left, cls != 0u);') && count(surface, 'wildS ==') === 0 && count(surface, '- wildS') === 0],
      ['the weather\'s amount is the panel\'s row × how far the place is left to it', count(look, 'let kw = k.w * left;') === 1],
      ['the weather on what stands never asks the row alone', standing.length > 0 && count(standing, 'k.w') === 0],
      ['the trodden lane does not ask the word, but for the share the stuff\'s own `worn` answer has taken of it', look.includes('clamp(wm.x * (0.5 + rag), 0.0, 1.0) * k.w * select(1.0, step(0.6, roughIn), wears) * (1.0 - (1.0 - wild) * step(0.001, keep.w));')],
      ['what a stuff\'s `worn` says of use is shown by how far the surface is kept, never by the weather\'s amount', look.includes('keep.w * min(k.w, 1.0) * (1.0 - wild), fp,') && look.includes('let knock = stretch * (1.0 - smoothstep(high - max(0.008, fp), high, wp.y)) * keep.w * min(k.w, 1.0) * (1.0 - wild);') && look.includes('alb *= 1.0 - 0.2 * keep.w * (1.0 - wild) * min(k.w, 1.0) * clamp(lookWay.x * uneven, 0.0, 1.0);')],
      ['a mark is handed how far its paint is old', look.includes('fn lookMarkSoft() -> vec2f { return vec2f(lookCfg.g.x, 1.0 - lookKept()); }')],
      ['old paint is rubbed thin by that much and by feet whatever it is', marks.includes('let keep = 1.0 - 0.85 * min(age, 1.0) * worn * max(soft2.y, min(trod, 1.0));') && marks.includes('fn markTone(soft: vec2f, near: f32) -> f32 { return 1.0 - 0.18 * min(soft.x, 2.0) * soft.y * near; }')],
    ];
    const bad = pins.filter(p => !p[1]).map(p => p[0]);
    verdict(bad.length === 0, 'the lines the second writing below is held to: how far a surface is left to the weather, how wild it is, the weather on what stands, what a stuff\'s `worn` says of use and the age of a mark\'s paint', bad.length ? `NOT HELD: ${bad.join('; ')}` : `${pins.length} pins hold`); }
  // 4. a second writing of the places it acts (held to the shader by line 3)
  { const left = (kept: number) => BRK ? 1 : 1 - kept;
    const blotch = (kept: number, fades: number, lying: boolean) => left(kept) * (lying ? 1.7 : 1) * fades * 0.25;   // (the most a slow blotch moves a paint either way: 0.3 × ½ + 0.2 × ½)
    const wild = (kept: number, stuffWild: 0 | 1) => stuffWild * left(kept), drift = (kept: number, stuffWild: 0 | 1) => 0.03 * (1 - wild(kept, stuffWild));   // (the No repeat row at 1: 0.06 × ½ either way)
    const band = (kept: number, stuffWild: 0 | 1, near: number) => near * near * (1 - wild(kept, stuffWild)), knock = (kept: number, stuffWild: 0 | 1, worn: number) => worn * (1 - wild(kept, stuffWild));   // (the detail layer's dust at a wall's foot, of 1 hard against it; a wall's knocked foot)
    const lane = (kept: number, stuffWild: 0 | 1, worn: number) => { const w = wild(kept, stuffWild); return (1 - 0.2 * worn * (1 - w)) * (1 + 0.24 * (1 - (1 - w) * (worn > 0 ? 1 : 0))); };   // (a matte floor's lane, walked at 1: the dirt feet bring in × the weather's beaten pale)
    const paint = (kept: number, soft: number, worn: number, trod: number) => 1 - 0.85 * Math.min(soft, 1) * worn * Math.max(left(kept), Math.min(trod, 1));
    const tone = (kept: number, soft: number) => 1 - 0.18 * Math.min(soft, 2) * left(kept);
    const r = (v: number) => +v.toFixed(3), K = [0, 0.5, 1];
    const b = K.map(k => r(blotch(k, 1, false))), g = K.map(k => r(blotch(k, 1, true))), f = K.map(k => r(blotch(k, 0.3, false)));
    const w = K.map(k => r(wild(k, 1))), w0 = K.map(k => r(wild(k, 0))), d = K.map(k => r(drift(k, 1))), d0 = K.map(k => r(drift(k, 0)));
    const p = K.map(k => r(paint(k, 1, 1, 0))), pt = K.map(k => r(paint(k, 1, 1, 0.6))), t = K.map(k => r(tone(k, 1)));
    const bd = K.map(k => r(band(k, 1, 1))), kn = K.map(k => r(knock(k, 1, 1))), kn0 = K.map(k => r(knock(k, 0, 0.8)));
    const ls = K.map(k => r(lane(k, 1, 1))), le = K.map(k => r(lane(k, 1, 0)));
    const ok = b[0] === 0.25 && b[1] === 0.125 && b[2] === 0 && g[0] === 0.425 && g[2] === 0 && f[0] === 0.075 && f[2] === 0
      && w[0] === 1 && w[1] === 0.5 && w[2] === 0 && w0.every(v => v === 0) && d[0] === 0 && d[1] === 0.015 && d[2] === 0.03 && d0.every(v => v === 0.03)
      && ls[0] === 1.24 && ls[2] === 0.8 && ls[1] === r(0.9 * 1.12) && le.every(v => v === 1.24)
      && bd[0] === 0 && bd[1] === 0.5 && bd[2] === 1 && kn[0] === 0 && kn[1] === 0.5 && kn[2] === 1 && kn0.every(v => v === 0.8)
      && p[0] === 0.15 && p[1] === 0.575 && p[2] === 1 && pt[0] === 0.15 && pt[1] === 0.49 && pt[2] === 0.49 && t[0] === 0.82 && t[1] === 0.91 && t[2] === 1;
    verdict(ok, 'left to the weather a wall\'s paint drifts a quarter either way, and the paint of a mark is rubbed to 0.15 of itself where it is worn and stands at 0.82 of its colour; kept, the wall is one shade but for the three in a hundred no forty metres are without, has the dust at its foot and the knocked coat its stuff\'s `worn` says, and the mark is whole and its own colour on a wall and still rubbed to half where its floor is walked; half way is half',
      `kept 0 / 0.5 / 1: a wall's blotch ${b.join(' / ')} · the ground's ${g.join(' / ')} · a factory finish's ${f.join(' / ')} · how far the weather's rules answer for a weathered stuff ${w.join(' / ')} (a kept stuff ${w0.join(' / ')}) · its drift ${d.join(' / ')} (a kept stuff's ${d0.join(' / ')}) · a slab floor's lane ${ls.join(' / ')} (beaten pale to a shade darker; bare earth's ${le.join(' / ')}) · the dust band at its foot ${bd.join(' / ')} · its knocked foot where its stuff says worn 1 ${kn.join(' / ')} (a kept stuff's of 0.8: ${kn0.join(' / ')}) · a worn mark's paint ${p.join(' / ')} · the same where feet go ${pt.join(' / ')} · its tone ${t.join(' / ')}`); }
  // 5. THE FILE the page loads has its generator's word: run on every generator against the level file as it lies (a stale or hand-changed file fails). KEPT_BREAK: a generator's word is not the file's.
  { const { readdirSync, existsSync } = await import('node:fs'); const dir = resolve(ROOT, 'tools/levels');
    const bad: string[] = []; let gens = 0, say = 0;
    for (const f of readdirSync(dir).filter(f => f.endsWith('.ts')).sort()) { if (!/^export (const|function) build\b/m.test(readFileSync(`${dir}/${f}`, 'utf8'))) continue;
      const g = await import(`${dir}/${f}`), name = g.NAME ?? f.replace(/\.ts$/, ''), file = resolve(ROOT, `levels/${name}.json`); if (!existsSync(file)) continue;   // (no file of that name: the exporter's own --check says so)
      const want = BRK ? 0.5 : g.KEPT, got = JSON.parse(readFileSync(file, 'utf8')).meta?.kept; gens++; if (want !== undefined) say++;
      if (got !== want) bad.push(`${name}.json says ${got}, its generator ${want}`); }
    verdict(gens > 30 && bad.length === 0, 'every level file has its generator\'s word (none where it says none)',
      `${gens} generators with a file, ${say} with a word today${bad.length ? ` · WRONG: ${bad.slice(0, 4).join('; ')}` : ' · every file agrees with its generator'}`); }
  console.log(`=== kept: ${all ? 'ALL PASS' : 'FAILED'} ===`);
  return all;
}
if (import.meta.main) process.exit((await keptCheck()) ? 0 : 1);
