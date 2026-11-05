// STIRRED SMOKE CURLS, as the game tells the solver (src/game/game.ts smokeWake + SMOKE_STIR; the look page's row `stir`; the solver's half is smoke/smoke.wgsl
// emBody and the stirred confinement, which only a page runs and no tool here draws: a step of the solver takes seconds on the software adapter). The maintainer, 10-31: "i do wonder if there could be more turbulence
// or vorticity near the edges, especially when it's driven by player movement".
// No GPU: what is read here is what a moving man HANDS the solver: one body a frame, moving as he moves, and a kick behind him that changes side with his
// stride; with the row at nothing, or off the look page, the two soft shoves the build has. What the smoke then does is never seen here.
//   bun run tools/qa/smokestir-check.ts   (exit 0 = all pass; SMOKESTIR_BREAK=1 leaves the row at nothing where the lines want it up, and must fail)
import { standUp } from './headless.ts';
import { lookConfig } from '../../src/render/frame';

const BRK = process.env.SMOKESTIR_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== smokestir: a moving man as the smoke solver is told of him${BRK ? ' (the row never raised: must fail)' : ''} ===`);
const l = console.log; console.log = () => { }; const W: any = await standUp({ seed: 3 }); for (let i = 0; i < 30; i++) W.step(1 / 60); console.log = l;
const g = W.game, sam = g.player.char, was = { on: lookConfig.on, stir: lookConfig.stir, smoke: g.smoke };
interface Told { body: any[]; kick: any[]; soft: any[]; stir: number[] }
/** the wake of one man over one stride (twenty phases), at this speed along +x, with the row where `amount` says (null = off the look page) */
function stride(speed: number, amount: number | null): Told {
  const T: Told = { body: [], kick: [], soft: [], stir: [] };
  g.smoke = { ...was.smoke, push(p: any) { (p.body > 0 ? T.body : p.radius < 0.2 ? T.kick : T.soft).push({ ...p, pos: [...p.pos], dir: [...p.dir], phase: sam.anim.phase }); }, setStir(a: number) { T.stir.push(a); } };
  lookConfig.on = amount !== null; lookConfig.stir = amount === null ? 1 : BRK ? 0 : amount;
  const v = [sam.vel[0], sam.vel[1], sam.vel[2]], ph = sam.anim.phase;
  for (let i = 0; i < 20; i++) { sam.vel[0] = speed; sam.vel[1] = 0; sam.vel[2] = 0; sam.anim.phase = i / 20; g.smokeWake([sam]); }
  sam.vel[0] = v[0]; sam.vel[1] = v[1]; sam.vel[2] = v[2]; sam.anim.phase = ph; g.smoke = was.smoke; return T;
}
try {
  const up = stride(1.2, 1), f = (v: number) => v.toFixed(2);
  const b = up.body[0], left = up.kick.filter(k => k.dir[2] > 0), right = up.kick.filter(k => k.dir[2] < 0);
  check(up.body.length === 20 && up.soft.length === 0 && !!b && Math.abs(b.speed - 1.2) < 1e-6 && b.dir[0] > 0.99 && b.dir[1] === 0 && b.radius > 0.15 && b.radius < 0.4 && b.body > 0.3 && b.body < 1.1,
    'with the row up a walking man is one body a frame, upright, moving as he moves, and the two soft shoves are not sent',
    `bodies ${up.body.length}, soft ${up.soft.length}; radius ${b ? f(b.radius) : '-'} m, half height ${b ? f(b.body) : '-'} m, speed ${b ? f(b.speed) : '-'} along ${b ? b.dir.map(f).join(' ') : '-'}`);
  check(left.length >= 4 && right.length >= 4 && left.length + right.length < 20 && up.kick.every(k => Math.abs(k.dir[0]) < 1e-6 && k.dir[1] === 0 && Math.sign(k.dir[2]) === Math.sign(Math.sin(2 * Math.PI * k.phase)) && k.pos[1] < b.pos[1]),
    'behind him a kick across his way changes side with his stride, low at his legs, and rests between the two',
    `${left.length} to one side, ${right.length} to the other, ${20 - up.kick.length} frames of the stride with none; speed ${up.kick[0] ? f(up.kick[0].speed) : '-'} m/s`);
  const fast = stride(4, 1), twice = stride(1.2, 2);
  check(up.kick.length > 0 && fast.kick.every(k => k.speed <= 2.2 + 1e-6) && fast.kick[0].speed > up.kick[0].speed && twice.kick[0].speed > up.kick[0].speed * 1.9 && up.stir[0] === 1 && twice.stir[0] === 2,
    'the kick grows with his speed up to a ceiling and with the row, and the solver is told the row',
    `walking ${f(up.kick[0]?.speed ?? 0)}, running ${f(fast.kick[0]?.speed ?? 0)}, the row at two ${f(twice.kick[0]?.speed ?? 0)} m/s; told ${up.stir[0]} and ${twice.stir[0]}`);
  const still = stride(0.1, 1);
  check(still.body.length + still.kick.length + still.soft.length === 0, 'a man who stands is not told of', `${still.body.length + still.kick.length + still.soft.length} pushes`);
  // (the row at nothing and a page that is not the look page are the build's wake: asked without the break, which is about the row being UP)
  const zero = (() => { const T: Told = { body: [], kick: [], soft: [], stir: [] }; g.smoke = { ...was.smoke, push(p: any) { (p.body > 0 ? T.body : T.soft).push(p); }, setStir(a: number) { T.stir.push(a); } };
    lookConfig.on = true; lookConfig.stir = 0; const v0 = sam.vel[0]; for (let i = 0; i < 20; i++) { sam.vel[0] = 1.2; g.smokeWake([sam]); } const a = { ...T, soft: [...T.soft] };
    lookConfig.on = false; lookConfig.stir = 1; T.soft.length = 0; T.stir.length = 0; for (let i = 0; i < 20; i++) { sam.vel[0] = 1.2; g.smokeWake([sam]); } sam.vel[0] = v0; g.smoke = was.smoke; return { a, b: T }; })();
  check(zero.a.body.length === 0 && zero.a.soft.length === 40 && zero.b.body.length === 0 && zero.b.soft.length === 40 && zero.a.stir.every(s => s === 0) && zero.b.stir.every(s => s === 0),
    'with the row at nothing, and off the look page, he is the two soft shoves a frame the build has, and the solver is told nothing is stirred',
    `row at nothing: ${zero.a.soft.length} shoves, ${zero.a.body.length} bodies; off the look page: ${zero.b.soft.length} shoves, ${zero.b.body.length} bodies`);
} finally { lookConfig.on = was.on; lookConfig.stir = was.stir; g.smoke = was.smoke; }
process.exit(red ? 1 : 0);
