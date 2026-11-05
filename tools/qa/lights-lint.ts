// ONE WAY IN TO THE LIGHTS' STATE (rust/sim/src/lights.rs, sim/lightshome.ts LightHome.in): the state of a fixture and of the mains is in the module and is read-only to the
// compiler (game.ts LightTarget is sim/lightshome.ts Fixture, whose on, broken, disabledUntil, factor, handOff, baseIntensity and stagger are getters). This reads src/ for what the compiler
// cannot see: a write through `any` or a cast, a lamp's burning written from outside (`enabled` on a fixture's own lamp, which is a row's: it is followed by `feed` or it is
// the module's copy), or the state kept a second time on an object.
//   bun run tools/qa/lights-lint.ts        exit 0 clean · 1 a line breaks it, named
//   LIGHTS_LINT_BREAK=1 …                  the red path: a made-up line is read too, and the run must FAIL
// (tools/qa is not read: a check's line through `any` that writes a getter throws by itself, TypeError, in the run; a page's line, sloppy, is why tools/qa's .cjs were grepped by hand at the port.)
// Not game code; nothing here runs the game.
import { readdirSync, readFileSync, statSync } from 'fs';
import { join, relative, resolve } from 'path';
const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.LIGHTS_LINT_BREAK === '1';
/** a write of the fixture's state, by any name: never right outside the home */
const STATE = /\.(disabledUntil|handOff|baseIntensity)\s*(=[^=>]|\+=|-=)|(\b(t|tg|target|lamp|panel|fixture|pool|had|mon)|\.target|\.mon|\.t)\.(on|broken|factor|stagger)\s*(=[^=>]|\+=|-=)/;
/** a fixture's own lamp written from outside: `enabled` on `t.light`, `al.light`, `b.light`, a camera's cone */
const LAMP = /\b(t\.light|al\.light|b\.light|c\.cone|cone)\.enabled\s*=[^=]/;
/** a second copy of the state on a Game */
const COPY = /\b(g|game|this|G)\.(blackout|breakerFixT)\s*=[^=>]/;
/** what may: the home itself, the kit picture's lamp (the host's still picture, followed by `feed`), and the words after a `//` are no code */
const HOME = new Set(['src/sim/lightshome.ts']);
const LAMP_OK = new Set(['src/host/kitView.ts', 'src/game/lighting.ts', 'src/game/doze.ts']);   // (lighting.ts: a lamp hung after the level was stood, which has no row. doze.ts: faceLight's screens put out around one question, until the question takes a mask of what it leaves out)
const out: string[] = [];
function read(file: string, text: string): void {
  text.split('\n').forEach((raw, i) => {
    const line = raw.replace(/\/\/.*$/, '').replace(/\/\*.*?\*\//g, ''); if (!line.trim()) return;
    if (STATE.test(line)) out.push(`${file}:${i + 1} — writes a fixture's state: LightHome.in, a kind and a fixture`);
    if (LAMP.test(line) && !LAMP_OK.has(file)) out.push(`${file}:${i + 1} — writes a fixture's lamp's burning: the module's, and RtLight its copy`);
    if (COPY.test(line)) out.push(`${file}:${i + 1} — writes the mains on a Game: LightHome.in`);
  });
}
function walk(dir: string): void {
  for (const e of readdirSync(dir)) { const p = join(dir, e); if (statSync(p).isDirectory()) { walk(p); continue; } if (!e.endsWith('.ts')) continue;
    const file = relative(ROOT, p); if (HOME.has(file) || e === 'built.ts') continue; read(file, readFileSync(p, 'utf8')); }
}
walk(join(ROOT, 'src'));
if (BREAK) read('(the red path)', 't.broken = true;\nc.target.disabledUntil = 3;\nt.light.enabled = false;\n');
if (!/get on\(\): boolean/.test(readFileSync(join(ROOT, 'src/sim/lightshome.ts'), 'utf8')) || !/export type LightTarget = Fixture/.test(readFileSync(join(ROOT, 'src/game/game.ts'), 'utf8'))) out.push('game.ts LightTarget is no longer sim/lightshome.ts Fixture, whose state is getters: the compiler stops holding it');
for (const o of out) console.log(`- FAIL — ${o}`);
console.log(out.length ? `=== lights-lint: FAIL, ${out.length} line(s)${BREAK ? ' (the red path: as it should)' : ''} ===` : '=== lights-lint: PASS — nothing writes the lights but the one way in ===');
process.exit(out.length ? 1 : 0);
