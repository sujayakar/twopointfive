// WHAT THE RUST GAME WRITES AND A TOOL READS COMES BACK. A module's own table or log that a module exports (`DESTR_LOG`, `FRAG_LOG`, `HOSTILE`) is a copy on the puppet, kept by the apply. One on no list stays as the start left it: a tool reads an empty log after a blast, and nothing says why.
// So: every field of a module's top-level record whose TypeScript name the module exports and a tool reads, and which a line of that module's Rust writes, is on `everyTick` (tools/wire/fields.json), or on the list below with a reason.
//   bun run tools/wire/back-check.ts     exit 0 all come back · 1 one does not: `wire.ts list <row>`, or say why here
// BACK_BREAK=1 reads the list without destruction's log, and the check must fail.
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
const ROOT = join(import.meta.dir, '../..'), S = JSON.parse(readFileSync(join(ROOT, 'tools/blitz/schema.json'), 'utf8')), L = JSON.parse(readFileSync(join(ROOT, 'tools/wire/fields.json'), 'utf8'));
/** a table the host keeps for itself, and no copy of the game's */
const NOT_BACK: Record<string, string> = {};
const every = new Set<string>(L.everyTick); if (process.env.BACK_BREAK) every.delete('destruction::DestructionMod.destr_log');
const toolText = readdirSync(join(ROOT, 'tools/qa')).filter(f => f.endsWith('.ts')).map(f => readFileSync(join(ROOT, 'tools/qa', f), 'utf8')).join('\n');
const WRITES = (r: string) => new RegExp(`\\.${r}\\b\\s*(?:=[^=]|\\+=|-=|\\.push|\\.insert|\\.clear|\\.remove|\\.pop|\\.extend|\\.retain|\\.truncate|\\.drain|\\.entry|\\.splice|\\.get_mut|\\.iter_mut)|&mut [\\w.]*\\.${r}\\b`);
const lost: string[] = []; let held = 0;
for (const T of Object.values<any>(S.types)) { if (T.k !== 'struct' || !T.name.endsWith('Mod')) continue; const dir = join(ROOT, 'rust/game_rules/src', T.mod); if (!existsSync(dir)) continue; const body = readdirSync(dir).filter(f => f.endsWith('.rs')).map(f => readFileSync(join(dir, f), 'utf8')).join('\n');
  for (const f of T.fields) { const m = (f.expr ?? '').match(/B\.top\("([^"]+)", "([^"]+)"/); if (!m) continue; const q = `${T.mod}::${T.name}.${f.rust}`; if (every.has(q) || NOT_BACK[q] || !WRITES(f.rust).test(body)) continue;
    const file = join(ROOT, m[1]); if (!existsSync(file) || !new RegExp(`export (const|let) ${m[2]}\\b`).test(readFileSync(file, 'utf8')) || !new RegExp(`\\b${m[2]}\\b`).test(toolText)) continue; lost.push(`${q} (${m[2]})`); }
  held++; }
for (const q of lost) console.log(`FAIL: ${q}: the Rust game writes it and a tool reads it, and it comes back on no list`);
console.log(`=== back-check: ${held} modules read: ${lost.length ? `${lost.length} do not come back` : 'all come back'} ===`); process.exit(lost.length ? 1 : 0);
