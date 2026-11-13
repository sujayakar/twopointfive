// THE MODULE A PAGE RUNS IS THE GAME THE GATE PLAYS. The native gate plays a few fights and says every hand of them and, every second, the word of each part of the game
// (rust/game_start/src/bin/gate/records.rs); the module of this checkout is handed the same hands, as a machine that is sent them, and its words must be the same.
//   bun run tools/draw/native-check.ts [the gate's binary]     (none: cargo makes it)     exit 0 · 1 a fight parted, with the second and the parts
//   NATIVE_BREAK=1 …    one hand of each fight is handed wrong: every line must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { toolModule, missions } from '../desk.ts';
import { characterFiles } from '../blitz/character.ts';
import { Face } from '../../src/draw/face';
import { seatInput } from '../../src/draw/input';
import { StartFlag, StepFlag } from '../../src/draw/bits';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.NATIVE_BREAK === '1', own = process.argv[2];
const said = Bun.spawnSync(own ? [own, 'records'] : ['cargo', 'run', '-q', '--profile', 'gate', '--bin', 'gate', '--', 'records'], { cwd: resolve(ROOT, 'rust/game_start'), stdout: 'pipe', stderr: 'pipe', maxBuffer: 1 << 28 });
if (said.exitCode !== 0) { console.error(`native-check: the gate said no records: ${said.stderr.toString().slice(0, 400)}`); process.exit(1); }
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT), face = new Face(new WebAssembly.Instance(toolModule()).exports), names = face.stateNames(), rows = missions();
let fails = 0, of = 0;
for (const fight of said.stdout.toString().split(/^fight\t/m).slice(1)) {
  const lines = fight.split('\n'), [id, seats, seed] = lines[0].split('\t'), m = rows.find(r => r.id === id)!, file = new URLSearchParams(m.link).get('levelFile') ?? 'office.json';
  const h = face.start(new Uint8Array(readFileSync(resolve(ROOT, 'levels', file))), m.link, mannequin, mannequinBin, cast, castLook, '', +seed, +seats, StartFlag.Dress); face.networked(h);
  let ticks = 0, asked = 0, parted = '', hands = 0;
  for (const line of lines.slice(1)) { const w = line.split('\t');
    if (w[0] === 'hand') { const v = seatInput(); w[2].split(' ').forEach((x, i) => { v[i] = +x; }); if (w[3] || +v[0]) hands++; if (BREAK && ticks === 90 && w[1] === '0') v[9] += 1; face.record(h, +w[1], v, `${w[3]}\n${w[4]}`); }
    else if (w[0] === 'tick') { face.step(h, 1 / 60, 0 as StepFlag); face.outClear(h); ticks++; }
    else if (w[0] === 'words') { asked++; const is = face.stateWords(h), was = w[1].split(' ').map(x => parseInt(x, 16)), apart = names.filter((_, i) => is[i] !== was[i]);
      if (is.length !== was.length || apart.length) { parted = `${(ticks / 60).toFixed(1)} s in, in ${is.length !== was.length ? `${is.length} parts for ${was.length}` : apart.slice(0, 8).join(', ') + (apart.length > 8 ? ` and ${apart.length - 8} more` : '')}`; break; } } }
  face.drop(h); of++; if (parted) fails++;
  console.log(`- ${parted ? 'FAIL' : 'PASS'} — ${id}, ${seats === '1' ? 'one player' : 'two'}: ${ticks} ticks, ${hands} hands that did something, the module's words are the native game's ${asked} times${parted ? `\n    parts ${parted}` : ''}`); }
console.log(`=== native: ${fails ? `${fails} of ${of} FAILED` : 'ALL PASS'} ===`);
process.exit(fails || !of ? 1 : 0);
