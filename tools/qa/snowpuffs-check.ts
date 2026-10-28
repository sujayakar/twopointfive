// The look prototype's "Snow puffs" row (src/game/snowFx.ts SNOWPUFF, bootPuffOf, bootPuffs; render/frame.ts lookConfig.puffs): a foot that lands on lying snow
// throws a puff of the particles' own soft sprites, sized by how hard it came down, leaning the way the boot went, and each kick the game lists is filed once.
// No level, no GPU: the pool is a stand-in that takes every `emit`, so a puff here is a record. Nobody has SEEN one drift down the wind from this check.
//   bun run tools/qa/snowpuffs-check.ts   (exit 0 = all pass; SNOWPUFFS_BREAK=1 gives every kick a walker's force and must fail)
import { SNOWPUFF, bootPuffOf, bootPuffs, bootPuffsFiled } from '../../src/game/snowFx';
import type { SnowKick } from '../../src/game/snowTracks';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.SNOWPUFFS_BREAK === '1';
const kick = (n: number, force: number, dx = 1, dz = 0): SnowKick => ({ x: 10 + n, z: 4, dx, dz, force: BREAK ? 0.45 : force, wide: 0.12, serial: 7, t: 0, n });
const walk = bootPuffOf(kick(1, 0.45), 1)!, run = bootPuffOf(kick(2, 0.8), 1)!, sprint = bootPuffOf(kick(3, 1.2), 1)!, blast = bootPuffOf(kick(4, 2), 1)!, huge = bootPuffOf(kick(5, 40), 1)!;
const row = (b: typeof walk) => `${b.n} lumps of ${b.opts.size.toFixed(2)} m at ${b.opts.speed.toFixed(2)} m/s for ${b.opts.life.toFixed(1)} s`;
check(!!walk && !!sprint && walk.n < run.n && run.n < sprint.n && walk.opts.size < sprint.opts.size && walk.opts.speed < sprint.opts.speed && walk.opts.life < sprint.opts.life, `a harder step throws a bigger puff: a walk ${row(walk)}, a sprint ${row(sprint)}`);
check(!!blast && blast.n >= sprint.n && blast.opts.size > sprint.opts.size && blast.n <= SNOWPUFF.most && huge.n <= SNOWPUFF.most && huge.opts.size === bootPuffOf(kick(6, 2.5), 1)!.opts.size, `a blast's kick more again (${row(blast)}), and never over ${SNOWPUFF.most} lumps or the size of a force of 2.5 however hard`);
check(bootPuffOf(kick(7, SNOWPUFF.least * 0.5), 1) === null === !BREAK && bootPuffOf(kick(8, 1.2), 0) === null, `a step softer than ${SNOWPUFF.least} throws nothing, and the row at 0 throws nothing`);
{ const half = bootPuffOf(kick(9, 1.2), 0.5)!; check(!!half && half.n < sprint.n && half.opts.size === sprint.opts.size, `the row at a half files fewer lumps of the same size (${half?.n} for a sprint's ${sprint.n})`); }
{ const a = bootPuffOf(kick(10, 1.2, 0, -3), 1)!, still = bootPuffOf(kick(11, 1.2, 0, 0), 1)!, l = Math.hypot(...a.dir);
  check(Math.abs(l - 1) < 1e-6 && a.dir[2] < -0.3 && Math.abs(a.dir[0]) < 1e-9 && a.dir[1] > 0.5 && still.dir[1] === 1 && still.dir[0] === 0, 'the puff leans the way the boot was going, mostly up, and a foot that came straight down throws straight up'); }
check(walk.opts.seed !== run.opts.seed && bootPuffOf(kick(1, 0.45), 1)!.opts.seed === walk.opts.seed && walk.pos[1] > 0 && walk.pos[0] === 11, 'each kick has its own seed, the same every time it is asked: no dice are thrown');
{ // the list: only kicks past `seen` are filed, once each, into the dust kind
  const got: { kind: string; n: number }[] = []; const pool = { emit: (kind: string, _p: unknown, _d: unknown, n: number) => { got.push({ kind, n }); return n; } } as any;
  const list = [kick(21, 0.45), kick(22, 1.2), kick(23, 0.05), kick(24, 0.8)], was = JSON.stringify(list), before = bootPuffsFiled();
  const filed = bootPuffs(pool, list, 21, 1), again = bootPuffs(pool, list, 24, 1);
  const each = SNOWPUFF.shade > 0 ? 2 : 1;
  check(got.length === (BREAK ? 3 : 2) * each && got.every(g => g.kind === 'dust') && filed === got.reduce((a, g) => a + g.n, 0) && again === 0 && bootPuffsFiled() - before === filed && JSON.stringify(list) === was,
    `of four listed kicks with the first already seen and one too soft, two puffs are filed (${filed} lumps of the dust kind, each puff its shade and its white), none a second time, and the game's list is as it was`);
}
{ const got: { n: number; o: any }[] = []; const pool = { emit: (_k: string, _p: unknown, _d: unknown, n: number, o: unknown) => { got.push({ n, o }); return n; } } as any;
  bootPuffs(pool, [kick(31, 1.2), { ...kick(32, 1.2), x: 80 }], 0, 1, 40, 4, 12);
  const sh = got[0], wh = got[1];
  check(got.length === 2 && sh.o.color[2] > SNOWPUFF.shadeColor[2] && sh.o.color[2] < SNOWPUFF.color[2] && wh.o.color === SNOWPUFF.color && sh.o.size > wh.o.size && sh.o.speed < wh.o.speed && sh.n <= wh.n && sh.o.seed !== wh.o.seed && sh.o.color[0] < 0.7 * wh.o.color[0],
    `under a puff's white lumps go its shade's: ${sh?.n} wider, slower ones in the blue of snow in shade for the ${wh?.n} white (white on white snow cannot be seen)`);
  const deep: any[] = [], none: any[] = []; bootPuffs({ emit: (_k: string, _p: unknown, _d: unknown, n: number, o: unknown) => { deep.push(o); return n; } } as any, [kick(41, 1.2)], 0, 1, 0, 0, Infinity, 1);
  bootPuffs({ emit: (_k: string, _p: unknown, _d: unknown, n: number, o: unknown) => { none.push(o); return n; } } as any, [kick(42, 1.2)], 0, 1, 0, 0, Infinity, 0);
  check(deep.length === 2 && deep[0].color.every((c: number, i: number) => Math.abs(c - SNOWPUFF.shadeColor[i]) < 1e-9) && none.length === 1 && none[0].color === SNOWPUFF.color, 'the "Puff shade" row sets how deep that blue is: at 1 the shade lumps are the full blue, at 0 a puff is its white lumps alone');
  check(got.length === 2, 'a kick out of the view (farther than the flakes\' own reach from its centre) files no puff'); }
process.exit(red ? 1 : 0);
