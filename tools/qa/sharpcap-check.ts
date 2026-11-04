// SHARP DIRECT'S CAP AND ITS OFF WORD (src/render/shaders/sharp_direct.wgsl and its compact writing; render/frame.ts sharpCapConfig; ?sharpcap=0 in the page).
// A class's light is bounded by what its live lights send unshadowed, which took the white beads off round edges near a walled-off strong light. It is a rule
// of every page, so it has a word that puts it off: the scene prelude's `const SHARP_CAP`, read once at boot, and the bound stands behind it in both writings
// of the kernel. No GPU: what is read is the texts and the prelude under each setting. What the bound draws is the page's to say; never seen on a GPU with
// the word.
//   bun run tools/qa/sharpcap-check.ts   (exit 0 = all pass; SHARPCAP_BREAK=1 reads the prelude as if the word were never lowered and must fail)
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { scenePrelude, sharpCapConfig } from '../../src/render/frame';
const ROOT = resolve(import.meta.dir, '../..'), BRK = process.env.SHARPCAP_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== sharpcap: sharp direct's bound and the word that puts it off${BRK ? ' (the word unread: must fail)' : ''} ===`);
const BOUND = /E0 = min\(E0, max\(U0 - Uw0, vec3f\(0\.0\)\) \* hiR\); E1 = min\(E1, max\(U1 - Uw1, vec3f\(0\.0\)\) \* hiR\);/g;
for (const f of ['src/render/shaders/sharp_direct.wgsl', 'src/render/shaders/compact/sharp_direct.wgsl']) {
  const t = readFileSync(resolve(ROOT, f), 'utf8'), all = t.match(BOUND) ?? [], guarded = t.match(/if \(SHARP_CAP\) \{ E0 = min\(E0, max\(U0 - Uw0, vec3f\(0\.0\)\) \* hiR\); E1 = min\(E1, max\(U1 - Uw1, vec3f\(0\.0\)\) \* hiR\); \}/g) ?? [];
  check(all.length === 1 && guarded.length === 1, `${f}: the bound is written once and stands behind SHARP_CAP`, `written ${all.length}, behind the const ${guarded.length}`);
}
const constOf = (): string => /const SHARP_CAP: bool = (true|false);/.exec(scenePrelude())?.[1] ?? 'missing';
const was = sharpCapConfig.on, up = constOf(); sharpCapConfig.on = BRK ? was : false; const down = constOf(); sharpCapConfig.on = was;
check(was === true && up === 'true' && down === 'false' && constOf() === 'true', 'the prelude carries the setting: up as the module stands, down when it is lowered, up again when put back', `module ${was} · const ${up} · lowered ${down}`);
const main = readFileSync(resolve(ROOT, 'src/main.ts'), 'utf8');
check(/if \(q\.get\('sharpcap'\) === '0'\) sharpCapConfig\.on = false;/.test(main) && main.indexOf("q.get('sharpcap') === '0'") < main.indexOf('new Engine('), 'the page lowers it for ?sharpcap=0, before the engine and its programs are made');
process.exit(red ? 1 : 0);
