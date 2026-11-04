// The look prototype's "Snow puffs" row (src/game/snowFx.ts SNOWPUFF, bootKicks; render/frame.ts lookConfig.puffs): a foot that lands on lying snow kicks powder
// into the page's powder field (render/powderField.ts `kick`) by how hard it lands. No sprite is filed. No GPU: what the field does with a kick is
// tools/qa/powder-field-page.cjs's (a sprinter's kicks leave more than a walker's).
//   bun run tools/qa/snowpuffs-check.ts   (exit 0 = all pass; SNOWPUFFS_BREAK=1 gives every kick a walker's force and must fail)
import { SNOWPUFF, bootKicks, bootPuffsFiled } from '../../src/game/snowFx';
import type { SnowKick } from '../../src/game/snowTracks';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.SNOWPUFFS_BREAK === '1';
const kick = (n: number, force: number, dx = 1, dz = 0): SnowKick => ({ x: 10 + n, z: 4, dx, dz, force: BREAK ? 0.45 : force, wide: 0.12, serial: 7, t: 0, n });
const field = () => { const got: number[][] = []; return { got, kick: (x: number, z: number, f: number, dx = 0, dz = 0) => { got.push([x, z, f, dx, dz]); } }; };
{ const F = field(), list = [kick(21, 0.45), kick(22, 1.2), kick(23, 0.05), kick(24, 2, 0, -3)], was = JSON.stringify(list), before = bootPuffsFiled();
  const filed = bootKicks(F, list, 21, 1), again = bootKicks(F, list, 24, 1);
  check(filed === 2 && again === 0 && F.got.length === 2 && bootPuffsFiled() - before === 2 && JSON.stringify(list) === was, 'only the kicks past the book\'s count go in, one too soft to raise anything does not, the game\'s list is only read, and the tally counts them');
  check(F.got[0]?.[0] === 32 && F.got[0][1] === 4 && F.got[0][2] === 1.2 && F.got[1]?.[2] === 2 && F.got[1][3] === 0 && F.got[1][4] === -3, 'each lands where the boot did, with the boot\'s own force and the way it was going: a sprinter\'s 1.2, a blast\'s 2'); }
{ const F = field(), G = field(), H = field(); bootKicks(F, [kick(1, 0.45), kick(2, 1.2)], 0, 1); bootKicks(G, [kick(1, 0.45), kick(2, 1.2)], 0, 0.5); bootKicks(H, [kick(2, 1.2)], 0, 9);
  check(F.got.length === 2 && F.got[1][2] > 2 * F.got[0][2] && G.got[1][2] === F.got[1][2] * 0.5 && H.got[0][2] === 1.2 * SNOWPUFF.most, `a sprinter's step puts in ${(F.got[1]?.[2] / F.got[0]?.[2]).toFixed(1)} times a walker's; the row at a half halves it, and the row cannot ask for more than ${SNOWPUFF.most} times a kick`); }
{ const F = field(); check(bootKicks(F, [kick(1, 1.2)], 0, 0) === 0 && bootKicks(null, [kick(1, 1.2)], 0, 1) === 0 && bootKicks(undefined, [kick(1, 1.2)], 0, 1) === 0 && F.got.length === 0, 'the row at 0, or a page with no powder field, files nothing'); }
{ const s = readFileSync(resolve(import.meta.dir, '../../src/game/snowFx.ts'), 'utf8'), a = s.indexOf('export function bootKicks('), body = s.slice(a, s.indexOf('\n}', a));
  check(a > 0 && !/\.emit\(/.test(body) && !/emit\('dust'/.test(s.slice(s.indexOf('THE LOOK PROTOTYPE\'s SNOW PUFFS'), a)), 'a puff is no sprite: the kicks go to the field and nothing is filed with the particles for them'); }
process.exit(red ? 1 : 0);
