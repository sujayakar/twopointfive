// WHICH ARMS THE WALK FOUND. Stands a game at a frame, exports it, and prints everything the walk could not place (`miss`), with the arm each Cond, Json and lesson test took.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/arm-check.ts <scn>:<seed>:<F>
import { exportGame, report, bookOf } from './export';
import { decode } from './paths';
const [scn, seed, F] = process.argv[2].split(':'); const say = (x: string) => { process.stdout.write(x + '\n'); };
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts'); class Stop extends Error {}
const was = Game.prototype.update; let n = 0;
Game.prototype.update = function (this: any, ...a: unknown[]) { if (++n === +F) { const b = exportGame(this, reg); const R: any = report(); const M = [...R].map(([k, v]: any) => `${k}: ${v?.n ?? v?.count ?? ''} ${String(v?.first ?? v?.why ?? '').slice(0, 60)}`); say(`${M.length} things the walk could not place`); for (const l of M.slice(0, 40)) say('  ' + l.slice(0, 180));
    const C = new Map<string, number>(); for (const r of decode(b) as any[]) if (r.path.endsWith('#') || /\.done#|waiting|\.him/.test(r.path)) { const k = `${r.path.replace(/\[\d+\]/g, '[]').slice(0, 70)} ${String(r.bits).split('(')[0]}`; C.set(k, (C.get(k) ?? 0) + 1); }
    let lim = 0, bad = 0; for (const r of decode(b) as any[]) if (/\.limb_ov\[/.test(r.path)) { lim++; if (String(r.bits) === '4294967295') bad++; } say(`limb overrides: ${lim}, ${bad} of them -1`);
    let lifted = 0; for (const c of (bookOf(this) as any).listOf('CharId') as any[]) if (c.rd) for (const x of c.rd.limbOv) if (c.rd.ov.indexOf(x) < 0) lifted++; say(`limbs lifted out of ov right now: ${lifted}`);
    const L = [...C].filter(([k]) => /done#|waiting|\.data|\.him/.test(k)).sort(); for (const [k, v] of L.slice(0, 50)) say(`  ${v} ${k}`); throw new Stop(); } return (was as any).apply(this, a); } as any;
console.log = () => {}; const run = scn.startsWith('tut-') ? (await import('../qa/lesson-run.ts')).runLesson(scn, { seed: +seed, sky: false, glow: false, floor: 0 }) : play(scn, +seed, 300, false, {}); await (run as Promise<unknown>).catch((e: unknown) => { if (!(e instanceof Stop)) throw e; }); process.exit(0);
