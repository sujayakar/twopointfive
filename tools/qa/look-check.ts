#!/usr/bin/env bun
// THE DEFAULT LOOK'S CHECK (src/scene/look.ts) — `bun run tools/qa/look-check.ts`.
// The palette: no two rows of it are one colour, and no level outside the look carries one.
// Exit 0 = all pass, 1 = a check failed. Also `bun run tools/qa/probes.ts look` (in all-quick): lookCheck() below.
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
export async function lookCheck(): Promise<boolean> {
await import('./headless.ts');   // (the game modules are one import cycle: enter it the way every probe does)
const { GWOT_PALETTE, SNOW_PALETTE, lookPaintOf } = await import(`${ROOT}/src/scene/gwotPalette.ts`);
const { DOES_DEFAULT, doesOfWord } = await import(`${ROOT}/src/scene/materials.ts`);
/** what a box or a part of a file does: its own word, else its material row's, else the default row (as the loader writes it onto the box) */
const doesIn = (o: any, mats: any): string => o.does ?? (o.material !== undefined ? mats[o.material]?.does : undefined) ?? DOES_DEFAULT;
const { standUp } = await import('./headless.ts');
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { FLAT_STEP, flatClashes, isFlat } = await import(`${ROOT}/tools/levels/flatLayers.ts`);
/** the three destruction indexes of a file, as text */
async function indexesOf(file: string): Promise<string> {
  const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { };
  try { const was = DESTRUCTION.on; DESTRUCTION.on = true;
    try { const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) }); const d = W.game.destr, boxes = W.game.col.boxes;
      const items = [...d.items].map(([i, x]: [number, any]) => `${i}:${x.kind}`).sort().join(' ');
      const walls = d.walls.list.map((B: any) => `${boxes.indexOf(B.box)}:${B.kind}`).sort().join(' ');
      const furn = JSON.stringify(d.furn.inventory().list);
      return `items ${items}\nwalls ${walls}\nfurn ${furn}`; } finally { DESTRUCTION.on = was; } }
  finally { console.log = log; console.error = err; console.warn = warn; }
}

let ok = true; const check = (pass: boolean, what: string) => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}`); if (!pass) ok = false; };
const J = (f: string) => JSON.parse(readFileSync(resolve(ROOT, 'levels', f), 'utf8'));
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);
const albedoOf = (b: any, mats: any): number[] => b.albedo ?? mats[b.material]?.albedo;
/** the generator of a level file, by name: tools/levels/<stem>.ts, or the stem without its dashes */
const genOf = (file: string): string | undefined => { const stem = file.replace(/\.json$/, ''); return [stem, stem.replace(/-/g, '')].find(n => existsSync(resolve(ROOT, 'tools/levels', `${n}.ts`))); };
const PAINT = new Set(['albedo', 'material', 'rough', 'metal', 'stuff', 'does']);   // (what a box says it is made of is part of its paint: the skin drops it with the paint it repaints. `does` is NOT, and is held equal below: but a file writes it on the box or leaves it to the box's row, and the two files name different rows)

// the palette
{ const seen = new Map<string, string>(); let clash = 0;
  for (const [m, a] of [...Object.entries(GWOT_PALETTE), ...Object.entries(SNOW_PALETTE)] as [string, number[]][]) { const k = a.join(','), o = seen.get(k); if (o && o !== m) clash++; seen.set(k, m); }
  check(clash === 0, `the palette: ${seen.size} paints, no two of them one colour (${clash} clashes): a level file names a box's paint by its numbers`);
  // (a level BUILT in the kit carries the palette on purpose — its generator imports the kit; any other level carrying one of its colours does so by accident)
  let strays: string[] = [], built: string[] = [];
  // (built in the kit = its generator imports the kit, or imports a generator beside it that does: the training yards share tools/levels/train-ground.ts)
  const KIT = /gwotKit|gwotSkin|gwotPalette/;
  const inKit = (f: string): boolean => {
    const name = genOf(f), gen = name && resolve(ROOT, 'tools/levels', `${name}.ts`); if (!gen) return false;
    const src = readFileSync(gen, 'utf8'); if (KIT.test(src)) return true;
    return [...src.matchAll(/from '\.\/([\w-]+)'/g)].some(m => { const dep = resolve(ROOT, 'tools/levels', `${m[1]}.ts`); return existsSync(dep) && KIT.test(readFileSync(dep, 'utf8')); });
  };
  for (const f of readdirSync(resolve(ROOT, 'levels'))) { if (!f.endsWith('.json')) continue; const d = J(f); let n = 0;
    for (const m of Object.values(d.materials ?? {}) as any[]) if (lookPaintOf(m.albedo) !== undefined) n++; for (const b of d.boxes ?? []) if (b.albedo && lookPaintOf(b.albedo) !== undefined) n++;
    if (n) (inKit(f) ? built : strays).push(`${f} ×${n}`); }
  check(strays.length === 0, `no level outside the look carries one of its colours${strays.length ? ` — ${strays.join(', ')}` : ''} (the picture asks a level whether it wears the look by its paints: scene/lookTidy.ts; built in the kit, on purpose: ${built.map(x => x.split(' ')[0]).join(', ') || 'none'})`);
}

return ok;
}
if (import.meta.main) { const ok = await lookCheck(); console.log(ok ? '\n=== look-check: ALL PASS ===' : '\n=== look-check: FAILED ==='); process.exit(ok ? 0 : 1); }
