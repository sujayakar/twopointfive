// THE FIGHT COUNTER'S HANDS IN A RUN (tools/qa/fights-check.ts runs a mission with this as a Bun preload; no file of the repo changes): every write to a deciding field of a man,
// with the module that made it (the first src/game frame of the stack that is not a helper), and at the process's exit what it counted to FIGHTS_OUT. A tick counts when two
// modules set one field of one man to different values in it. Writes outside a tick and writes by save, rollback, replay or journal are not counted.
// Not game code. It reads the game's own fields through their own accessors (a field that is a column of the men's table stays in the column), so a run's digest is the same with it.
import { plugin } from 'bun';
import { writeFileSync } from 'node:fs';

type Write = { who: string; value: string };
const G = globalThis as unknown as { __fightTick: number; __fightOn: boolean; __fightMan: (m: object) => void };
G.__fightTick = 0; G.__fightOn = false;

/** a man's own deciding fields, and those of his order (`script`) */
const FIELDS = ['state', 'lastKnown', 'awareness', 'pathGoal', 'reactT', 'hold', 'pinned', 'foe', 'task', 'bodyDuty'];
const SCRIPT_FIELDS = ['goal', 'aimAt', 'stance', 'upper', 'crouch', 'over', 'speed', 'face'];
/** helpers a write passes through on its way: the write is their caller's */
const HELPERS = /^(goTo|order|legTo|set|proxSet|who|note|endGuardScript|resetChase|noteW)$/;
const NOT_PLAY = /^(savestate|rollback|replaybuf|journal)$/;

const out: { ticks: number; pairs: Record<string, number>; runs?: number } = { ticks: 0, pairs: {} };
let cur = -1, thisTick = new Map<string, Write[]>();

/** a value as the count compares it: a point to a tenth of a metre, a number to a hundredth, a man by name */
function word(v: unknown): string {
  if (v === null || v === undefined) return 'null';
  if (Array.isArray(v)) return v.length ? v.map(x => typeof x === 'number' ? x.toFixed(1) : 'o').join() : '[]';
  if (typeof v === 'object') return (v as { callsign?: string }).callsign ?? 'obj';
  return typeof v === 'number' ? v.toFixed(2) : String(v);
}
function writer(): string | null {
  for (const l of (new Error().stack ?? '').split('\n')) {
    const m = l.match(/at (?:new )?([\w.<> ]+)? ?\(?.*\/src\/game\/(\w+)\.ts/);
    if (!m) continue;
    if (HELPERS.test((m[1] ?? '').trim().split('.').pop()!)) continue;
    return NOT_PLAY.test(m[2]) ? null : m[2];
  }
  return 'outside';
}
function flush(): void {
  out.ticks++;
  for (const [k, a] of thisTick) {
    const mods = new Set(a.map(x => x.who));
    if (mods.size < 2 || new Set(a.map(x => x.value)).size < 2) continue;
    const p = `${k.split('|')[1]}: ${[...mods].sort().join(' + ')}`;
    out.pairs[p] = (out.pairs[p] ?? 0) + 1;
  }
  thisTick = new Map();
}
function noted(id: string, field: string, v: unknown, was: unknown): void {
  if (!G.__fightOn) return;
  const value = word(v); if (value === word(was)) return;
  const who = writer(); if (!who) return;
  if (G.__fightTick !== cur) { if (cur >= 0) flush(); cur = G.__fightTick; }
  const k = `${id}|${field}`; let a = thisTick.get(k); if (!a) thisTick.set(k, a = []);
  a.push({ who, value });
}
/** where a field stands: an accessor on his class (a column) or a value on him */
function accessorOf(o: object, f: string): PropertyDescriptor | null {
  for (let p = Object.getPrototypeOf(o); p; p = Object.getPrototypeOf(p)) { const d = Object.getOwnPropertyDescriptor(p, f); if (d?.get) return d; }
  return null;
}
let serial = 0;
const scripts = new WeakMap<object, object>();
G.__fightMan = (o: object) => {
  const m = o as Record<string, unknown>, id = `${m.callsign ?? 'man'}#${serial++}`;
  for (const f of FIELDS) {
    const d = accessorOf(o, f);
    let raw = m[f];
    Object.defineProperty(o, f, { enumerable: !d, configurable: true,
      get() { return d ? d.get!.call(o) : raw; },
      set(v) { const was = d ? d.get!.call(o) : raw; if (d) d.set!.call(o, v); else raw = v; noted(id, f, v, was); } });
  }
  let s = m.script as object | null | undefined;
  Object.defineProperty(o, 'script', { enumerable: true, configurable: true, get() { return s; }, set(v) {
    if (v && typeof v === 'object' && !(v as { __fightSeen?: boolean }).__fightSeen) {
      let p: object | undefined = scripts.get(v);
      if (!p) { p = new Proxy(v as object, { get(t, k) { return k === '__fightSeen' ? true : (t as Record<string | symbol, unknown>)[k]; }, set(t, k, x) { const r = t as Record<string | symbol, unknown>, was = r[k]; r[k] = x; if (SCRIPT_FIELDS.includes(k as string)) noted(id, `script.${String(k)}`, x, was); return true; } }); scripts.set(v as object, p); }
      v = p;
    }
    s = v; } });
};
process.on('exit', () => { flush(); if (process.env.FIGHTS_OUT) writeFileSync(process.env.FIGHTS_OUT, JSON.stringify(out)); });
plugin({ name: 'fights', setup(b) { b.onLoad({ filter: /src\/game\/game\.ts$/ }, async (a: { path: string }) => {
  let t = await Bun.file(a.path).text();
  const head = t.match(/export class Guard\b[^{]*\{/);
  if (!head || !t.includes('this.ticking = true;')) throw new Error('fights-preload: game.ts no longer has the lines it hangs on (export class Guard, this.ticking = true)');
  t = t.replace(head[0], head[0].replace('export class Guard', 'class FightGuard')).replace('this.ticking = true;', 'this.ticking = true; (globalThis as any).__fightTick++; (globalThis as any).__fightOn = true;');
  const end = t.indexOf('\n}\n', t.indexOf('class FightGuard')) + 3;
  t = `${t.slice(0, end)}export class Guard extends (FightGuard as any) { constructor(...a: any[]) { super(...a); (globalThis as any).__fightMan(this); } }\n${t.slice(end)}`;
  return { contents: t, loader: 'ts' };
}); } });
