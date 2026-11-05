// WHO WRITES THIS, ON THE TYPESCRIPT SIDE (the truth): every write of one property during one tick, with the stack that made it.
//   TPS_PROFILE=shipped bun run --preload ./tools/blitz/expose.ts tools/blitz/watch.ts <mission>:<seed> [--from K] --tick T '<expression of g>' <property> [--all]
// e.g.  … watch.ts room-clear-four:1 --tick 39 'g.col.boxes[412]' emissive        … 'g.guards[2]' irradiance        … 'g.guards[2].ch.pos' 0
// The mission is stood up exactly as run.sh stands it (stand.ts), so tick T is run.sh's tick T. The expression is JavaScript over the game `g` and names the OBJECT;
// the property is the field (or the index of a plain array) written. A write of the value it already holds is left out unless --all. An element of a typed array
// cannot be trapped: watch the field that holds the array, or the function that fills it. The stack names the TypeScript functions; every Rust function's doc line
// in sig.rs names its source the same way, so `grep -rn <function> rust/game/src/*/sig.rs` finds the translated one.
import { stand, standArgs } from './stand';
const argv = process.argv.slice(2); const ti = argv.indexOf('--tick'); const T = ti < 0 ? 1 : +argv.splice(ti, 2)[1]; const ai = argv.indexOf('--all'); const ALL = ai >= 0; if (ALL) argv.splice(ai, 1);
const say = (s = '') => process.stdout.write(s + '\n');
let o; try { o = standArgs([...argv, '--ticks', String(T)]); if (o.rest.length !== 2) throw new Error("give '<expression of g>' <property>"); } catch (e) { say(`watch: ${(e as Error).message}`); process.exit(64); }
const [expr, prop] = o.rest;
const show = (v: unknown) => { try { const s = typeof v === 'object' && v !== null ? JSON.stringify(v) : String(v); return s.length > 80 ? s.slice(0, 80) + '…' : s; } catch { return String(v); } };
let writes = 0, obj: any, had: PropertyDescriptor | undefined, cur: unknown, armed = false;
const r = await stand(o, {
  open(g, k) { if (k !== T) return;
    obj = new Function('g', `return (${expr});`)(g);
    if (obj === null || typeof obj !== 'object') throw new Error(`watch: ${expr} is ${show(obj)}, not an object`);
    if (ArrayBuffer.isView(obj)) throw new Error(`watch: ${expr} is a typed array: its elements cannot be trapped`);
    had = Object.getOwnPropertyDescriptor(obj, prop); if (had && (had.get || had.set || !had.configurable)) throw new Error(`watch: ${prop} of ${expr} is an accessor or is frozen`);
    cur = obj[prop]; say(`tick ${T} opens: ${expr}.${prop} = ${show(cur)}`);
    Object.defineProperty(obj, prop, { configurable: true, enumerable: had?.enumerable ?? true, get: () => cur, set: (v: unknown) => {
      if (armed && (ALL || !Object.is(v, cur))) { writes++; const st = (new Error().stack ?? '').split('\n').slice(2).filter(l => l.includes('/src/')).slice(0, 9).map(l => '      ' + l.trim().replace(/\(?[^\s(]*\/(src\/[^)\s]+)\)?/, '$1'));
        say(`  write ${writes}: ${show(cur)} -> ${show(v)}`); for (const l of st) say(l); }
      cur = v; } });
    armed = true; },
  shut(g, k) { if (k !== T || !obj) return; armed = false; Object.defineProperty(obj, prop, { configurable: true, enumerable: had?.enumerable ?? true, writable: true, value: cur });
    say(`tick ${T} returns: ${expr}.${prop} = ${show(cur)}; ${writes} write${writes === 1 ? '' : 's'}${ALL ? '' : ' that changed it'}`); },
});
if (r.why) say(`watch: ${r.why}`);
process.exit(r.why ? 5 : 0);
