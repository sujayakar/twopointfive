// A MISSION'S TIMELINE on the stand the loop compares on (the page's, every DRESS row down, the game's own die), with no exports: what happens at which tick.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/timeline.ts <mission>:<seed> [--ticks 20000]
import '../qa/stand.ts';
const argv = process.argv.slice(2), arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const say = (s = '') => process.stdout.write(s + '\n');
const TICKS = +arg('--ticks', '100000'), DIE = +arg('--die', '20942');
if (!argv[0] || argv[0].startsWith('--')) { say('timeline.ts <mission>:<seed> [--ticks N]'); process.exit(64); }
const [scn, seedS, frameS] = argv[0].split(':'); const seed = +(seedS ?? 1), frame = +(frameS ?? 1);
const { Game } = await import('../../src/game/game.ts');
const { play } = await import('../qa/playthrough.ts');
const { newDice } = await import('../../src/math/dice.ts');
const { dressAll } = await import('../../src/game/dress.ts');
class Stop extends Error {}
let k = 0, n = 0; let last: any = {}; let gives = 0;
const was = Game.prototype.update;
Game.prototype.update = function (this: any, ...a: any[]) {
  if (++n < frame) return (was as any).apply(this, a);
  const g = this; k++;
  if (k === 1) { dressAll(false); if (!g.dice) g.dice = newDice(DIE);
    const O = g.orders, w = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) { if (!g.ticking) { gives++; say(`  t${k} GIVE ${id} who=${typeof who === 'object' ? 'man' : who} kind=${(t ?? this.targetAt(g.aimPoint)).kind}`); } return w.call(this, id, t, who, opts); }; }
  (was as any).apply(this, a);
  const cur: any = { die: g.dice?.n, msgs: g.messages?.length, dead: g.guards.filter((x: any) => x.state === 'dead').length, guards: g.guards.length,
    doorsOpen: g.doors?.doors?.filter((d: any) => Math.abs(d.angle) > 0.05).length, fight: g.fight?.log?.length, thrown: g.throwables?.live?.length ?? g.thrown?.length, shots: g.stats?.shots ?? g.shotsFired, over: g.over ?? g.ended ?? g.result };
  const ch: string[] = []; for (const key of Object.keys(cur)) if (JSON.stringify(cur[key]) !== JSON.stringify(last[key])) ch.push(`${key} ${JSON.stringify(last[key])}→${JSON.stringify(cur[key])}`);
  if (ch.length) { let extra = ''; if (cur.msgs !== last.msgs && g.messages?.length) { const m = g.messages[g.messages.length - 1]; extra = ' | ' + JSON.stringify(m.text ?? m).slice(0, 90); } say(`t${k} (${g.time.toFixed(2)}s) ${ch.join(' · ')}${extra}`); }
  last = cur;
  if (k >= TICKS) throw new Stop();
} as any;
const log = console.log, er = console.error, wn = console.warn; let res: any = null;
try { res = await play(scn, seed, 300, false, {}); } catch (e) { if (!(e instanceof Stop)) { console.log = log; throw e; } } finally { console.log = log; console.error = er; console.warn = wn; }
say(`END at tick ${k}: ${res ? JSON.stringify({ end: res.end, why: res.why, t: res.t, team: res.team, of: res.of, hostiles: res.hostiles, hostilesOf: res.hostilesOf, orders: res.orders }) : 'stopped'}; gives ${gives}`);
process.exit(0);
