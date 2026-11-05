// A MISSION STOOD UP HEADLESS AND HELD AT ITS TICKS: the one way every tool of the tick loop runs the TypeScript game, so that a watch or a peek sees the very run
// that run.sh compared. The mission and its scripted player are tools/qa/playthrough.ts's; nothing of the game is copied here.
//
// What is done to the game so that the two sides can be stepped from one start (each a harness matter, README.md "Harness artefacts"):
//   · every DRESS row is put down before the start (the Rust tick is the tick with them down, and src/game/dress.ts says that is the same game to the bit);
//   · the game is given ITS OWN DIE on the first compared tick, so its word and count are in the block both sides start from and nothing of Math.random is handed over;
//   · `sim.settle` is called before the start is taken, so every pose is in the pool (the bridge carries no floats of a pose that is not at home);
//   · the host's doings between ticks that the Rust side cannot run are recorded: the test driver's orders (`gives`) and the box world's uploads.
export interface StandOpts { scn: string; seed: number; /** the mission's tick the run starts at (1 = its first) */ from: number; ticks: number; die?: number; dressUp?: boolean }
export interface TickHooks {
  /** as tick k's step opens (k counts from 1 at `from`); `uploads` = how often the host listed the box world again since the last tick */
  open?(g: any, k: number, dt: number, uploads: number): void;
  /** as tick k's step returns */
  shut?(g: any, k: number): void;
}
export interface Stood { /** ticks stepped */ ticks: number; /** the driver's orders, a line each: `<tick> <id> <who> <kind> <x> <y> <z> [door <place> <side> | man <place>]` */ gives: string[]; /** why it stopped early, or '' */ why: string; time: number }

const ROOT = new URL('../..', import.meta.url).pathname.replace(/\/$/, '');

export async function stand(o: StandOpts, hooks: TickHooks): Promise<Stood> {
  const { Game } = await import('../../src/game/game.ts');
  const { play } = await import('../qa/playthrough.ts');
  const { newDice } = await import('../../src/math/dice.ts');
  const { dressAll } = await import('../../src/game/dress.ts');
  class Stop extends Error {}
  const gives: string[] = []; let k = 0, n = 0, uploads = 0, why = '', time = 0;
  // the test driver gives its orders through the menu's own call, `orders.give(id, target, who)`, BETWEEN ticks: each such call (never one the game makes inside a
  // tick) is a line for the tick that opens next
  const tapGives = (g: any) => { const O = g.orders, was = O.give; O.give = function (this: any, id: string, t: any, who: any, opts: any) {
    if (!g.ticking) { const T = t ?? this.targetAt(g.aimPoint), w = who === undefined ? '-' : typeof who === 'object' ? `g${g.guards.indexOf(who)}` : String(who);
      const tail = T.kind === 'door' ? ` door ${g.doors.doors.indexOf(T.door)} ${T.side}` : T.kind === 'none' && T.man ? ` man ${g.guards.indexOf(T.man)}` : T.kind === 'foe' ? ` man ${g.guards.indexOf(T.foe)}` : (T.wall || T.thing) ? ' UNSUPPORTED-wall-or-thing' : '';
      gives.push(`${k + 1} ${id} ${w} ${T.kind} ${T.at.join(' ')}${tail}${opts ? ' UNSUPPORTED-opts' : ''}`); }
    return was.call(this, id, t, who, opts); }; };
  const was = Game.prototype.update;
  Game.prototype.update = function (this: any, ...a: any[]) {
    if (++n < o.from) return (was as any).apply(this, a);
    const g = this, step = g.stepSeats; k++;
    if (k === 1) {
      const W = g.engine.world, up = W.upload; W.upload = function (this: any, ...x: any[]) { uploads++; return up.apply(this, x); };
      tapGives(g);
      if (!o.dressUp) dressAll(false);
      if (!g.dice) g.dice = newDice(o.die ?? 20942);   // (update's first line takes the die and it moves into the block)
    }
    let err: unknown = null;
    g.stepSeats = function (this: any, dt: number, cmds: unknown) {
      if (k === 1) g.sim.settle(g);
      hooks.open?.(g, k, dt, k === 1 ? 0 : uploads); uploads = 0;
      try { return step.call(this, dt, cmds); } finally { hooks.shut?.(g, k); }
    };
    try { (was as any).apply(this, a); } catch (e) { err = e; } finally { delete g.stepSeats; }
    time = g.time;
    if (err) { why = `the TypeScript tick ${k} threw: ${String((err as any)?.stack ?? err).split('\n').slice(0, 7).join('\n').replaceAll(ROOT, '.')}`; throw new Stop(); }
    if (k >= o.ticks) throw new Stop();
  } as any;
  const log = console.log, er = console.error, wn = console.warn;
  try { await play(o.scn, o.seed, 300, false, {}); if (k < o.ticks && !why) why = `the mission ended after ${k} of ${o.ticks} ticks`; }
  catch (e) { if (!(e instanceof Stop)) why = String((e as any)?.message ?? e); }
  finally { console.log = log; console.error = er; console.warn = wn; Game.prototype.update = was; }
  return { ticks: k, gives: gives.filter(l => +l.split(' ')[0] <= o.ticks), why, time };
}

/** `<mission>:<seed>` and the flags every tool of the loop shares */
export function standArgs(argv: string[]): StandOpts & { rest: string[] } {
  const flag = (n: string) => { const i = argv.indexOf(n); if (i < 0) return undefined; const v = argv[i + 1]; argv.splice(i, 2); return v; };
  const from = +(flag('--from') ?? 1), ticks = +(flag('--ticks') ?? 1), die = flag('--die'), dress = flag('--dress');
  const [scn, seedS] = (argv.shift() ?? '').split(':');
  if (!scn || !Number.isInteger(+seedS) || !(from >= 1) || !(ticks >= 1)) throw new Error('give <mission>:<seed>, and whole numbers from 1 for --from and --ticks');
  return { scn, seed: +seedS, from, ticks, die: die === undefined ? undefined : +die, dressUp: dress === '1', rest: argv };
}
