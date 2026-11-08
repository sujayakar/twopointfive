// WHAT THE CAMERA THAT RUNS ITSELF IS TOLD IS WHAT THE COPY OF THE GAME TOLD IT. One Rust game under the headless host; after every tick the old read (host/autocam.ts autoRead, off the
// copy) and the row (rust/game_draw shot.rs, by draw/shot.ts) make one scene each, from the mode's begin on, and every field is compared. Played by keys: a walk round the floor, a squad
// order at a door, a burst, a man closed on. It goes when the copy does.
//   TPS_PROFILE=shipped tools/blitz/rust.sh rust bun run tools/draw/shot-check.ts [--ticks N] [level[:query]…]     default: the levels below, each a room clear with four
//   SHOT_BREAK=1 …     the row is read with a man counting only within a metre: it must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';
import { RustHost } from '../blitz/rust-host.ts';
import { Section, View, Want } from '../../src/draw/sections';
import { Drawn } from '../../src/draw/drawn';
import { shotFrom, shotScene } from '../../src/draw/shot';
import { AUTOCAM, SHOT_NUMBERS, autoTuneApply, type AutoScene } from '../../src/host/autoframe';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { autoCam, autoRead, autoRest, autoBegin, autoFree } = await import(`${ROOT}/src/host/autocam.ts`);

const argv = process.argv.slice(2), at = argv.indexOf('--ticks'), TICKS = at >= 0 ? Number(argv.splice(at, 2)[1]) : 1500;
const LEVELS: string[] = argv.length ? argv : ['office', 'compound', 'mousehole', 'glasshouse', 'depot', 'pass', 'bazaar:quiet', 'crossing:quiet&squad=1', 'tut-kit:roomclear=1&squad4', 'consulate:rescue&squad=4', 'registry:rescue'];
const BREAK = process.env.SHOT_BREAK === '1', DT = 1 / 60;
let fails = 0;
const line = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };

// the page's numbers and the module's are the one list, by name
{ const text = readFileSync(resolve(ROOT, 'rust/game_draw/src/shot.rs'), 'utf8'), rust = (text.match(/^numbers! \{ ([^}]*)\}/m)?.[1] ?? '').trim().split(/\s+/).map(s => s.replace(/_([a-z0-9])/g, (_, c) => c.toUpperCase()));
  line(rust.join() === SHOT_NUMBERS.join(), 'the numbers a read is made by are named alike, in one order', `${rust.join()} · ${SHOT_NUMBERS.join()}`); }

AUTOCAM.tune = true; autoTuneApply();
const near = (a: number, b: number): boolean => Math.abs(a - b) <= 2e-5 * Math.max(1, Math.abs(a), Math.abs(b));
const flat = (a: number[], b: number[]): number => Math.hypot(a[0] - b[0], a[2] - b[2]);
type Seen = Map<string, [number, string]>;
const differ = (seen: Seen, name: string, t: number, a: unknown, b: unknown): void => { const e = seen.get(name); if (e) e[0]++; else seen.set(name, [1, `tick ${t}: copy ${JSON.stringify(a)} · row ${JSON.stringify(b)}`]); };
const list = (seen: Seen, name: string, t: number, a: number[][], b: number[][]): void => { if (a.length !== b.length || a.some((p, i) => !near(p[0], b[i][0]) || !near(p[2], b[i][2]))) differ(seen, name, t, a, b); };
const thing = (seen: Seen, name: string, t: number, a: AutoScene['beat'], b: AutoScene['beat']): void => { if (!a !== !b || (a && b && (!near(a.w, b.w) || !near(a.at[0], b.at[0]) || !near(a.at[2], b.at[2])))) differ(seen, name, t, a, b); };
function compare(seen: Seen, t: number, a: AutoScene, b: AutoScene): void {
  if (!near(a.sam[0], b.sam[0]) || !near(a.sam[2], b.sam[2])) differ(seen, 'sam', t, a.sam, b.sam);
  list(seen, 'threats', t, a.threats, b.threats); list(seen, 'fight', t, a.fight, b.fight);
  thing(seen, 'beat', t, a.beat, b.beat); thing(seen, 'door', t, a.door, b.door); thing(seen, 'room', t, a.room, b.room);
  if (!near(a.zoomK, b.zoomK)) differ(seen, 'zoomK', t, a.zoomK, b.zoomK);
  for (const k of ['hold', 'still', 'free', 'shown', 'zoomFree'] as const) if (a[k] !== b[k]) differ(seen, k, t, a[k], b[k]);
  if (!!a.onto !== !!b.onto) differ(seen, 'onto', t, a.onto, b.onto);
  if (!near(a.vel[0], b.vel[0]) || !near(a.vel[1], b.vel[1])) differ(seen, 'vel', t, a.vel, b.vel);
  if (!near(a.firedAgo, b.firedAgo)) differ(seen, 'firedAgo', t, a.firedAgo, b.firedAgo);
  const p = a.lock ?? null, q = b.lock ?? null;
  if (!p !== !q || (p && q && (!p.at !== !q.at || !near(p.s, q.s) || p.stalk !== q.stalk || (p.at && q.at && (!near(p.at[0], q.at[0]) || !near(p.at[2], q.at[2])))))) differ(seen, 'lock', t, p, q);
}

const met: Record<string, number> = { threats: 0, fight: 0, beat: 0, door: 0, room: 0, lock: 0, onto: 0, hold: 0, still: 0, away: 0, shown: 0, 'in close or his arm': 0, 'a stalk': 0, 'the door under his hand': 0 };
for (const L of LEVELS) { const [name, query] = L.split(':');
  const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8'))), seed: 3 }));
  const why = (await quiet(() => startFromQuery(W.game, `?levelFile=${name}.json&${query ?? 'roomclear&squad4'}`)) as any)?.why; if (why) { line(false, `${name}: the start was refused`, why); continue; }
  const g = W.game, I = W.input, A = autoCam(), S = shotScene(), seen: Seen = new Map();
  let D: Drawn = null as never, n = 0;
  const WALK = ['KeyW', 'KeyD', 'KeyS', 'KeyA'], OFF = Math.floor(TICKS * 0.8);
  for (let t = 0; t < TICKS; t++) {
    g.godMode = true;
    // a walk round the floor, a turn every two seconds or so, pausing to stand
    for (const k of WALK) I.keys.delete(k);
    if (t % 400 < 300) I.keys.add(WALK[Math.floor(t / 130) % 4]);
    // the squad's orders: a door cleared, men sent to a mark
    if (t % 500 === 90 && g.orders?.active) { const p = g.player.char.pos; let best: any = null; for (const d of g.doors.list) if (!best || flat(d.frameCentre, p) < flat(best.frameCentre, p)) best = d; if (best) { g.orders.give('openClear', g.orders.targetAt([best.frameCentre[0], 1, best.frameCentre[2]]), 'all'); } }
    if (t % 500 === 400 && g.orders?.active) { const p = g.player.char.pos; g.orders.give('hold', g.orders.targetAt([p[0] + 2, 0, p[2]]), 'all'); }
    // a burst
    const half = Math.floor(TICKS / 2);
    I.buttons = t % 700 > 640 || (t >= half + 10 && t < half + 120) ? 1 : 0; if (t % 700 === 641 || t === half + 10) I.clicked = 1;
    // somebody else's view for a moment
    if (t === 200) g.spectate = 0; if (t === 230) g.spectate = -1;
    // a man walked up on: the player is put behind a man who is not onto him, as the sandbox puts him, and the click held
    if (t === half) { const man = g.guards.find((m: any) => m.team !== 'operative' && m.state !== 'dead' && !m.civ && m.state !== 'alert'); if (man) { const q = man.char.pos, a = man.char.bodyYaw; g.teleportPlayer([q[0] - Math.sin(a) * 1.5, 0, q[2] - Math.cos(a) * 1.5]); } }
    if (t >= half && t < half + 120) for (const k of WALK) I.keys.delete(k);
    // the mode put off for a moment, and on again: it begins afresh
    if (t === OFF && D) D.want = Want.view;
    if (t === OFF + 30 && D) { D.want = Want.shot | Want.view; autoBegin(A, W.cam); }
    await quiet(() => W.step(DT));
    const H: any = RustHost.last; if (!H) { line(false, `${name}: no Rust game stands`); break; }
    if (t === 0) {
      const X = H.F.X; D = new Drawn(X, H.F.mem, Want.shot | Want.view);
      const v = SHOT_NUMBERS.map(k => AUTOCAM[k]); if (BREAK) v[SHOT_NUMBERS.indexOf('fightM')] = 1;
      const p = X.face_alloc(v.length * 8) >>> 0; new Float64Array(H.F.mem.buffer, p, v.length).set(v); X.face_draw_shot(H.h, p, v.length); X.face_free(p, v.length * 8);
    }
    D.take(H.h); D.fresh();
    if (t >= OFF && t < OFF + 30) { if (D.n[Section.shot]) differ(seen, 'a row nobody asked for', t, 0, D.n[Section.shot]); autoRest(A); continue; }
    const own = g.spectate < 0 && !g.director.showing && !W.cam.pin && autoFree(g);   // (the old loop's `mine`, but for the mode's own word)
    if (own !== !!D.u[D.at[Section.view] + View.own] || own !== !!D.n[Section.shot]) differ(seen, 'the view is his own', t, own, [D.u[D.at[Section.view] + View.own], D.n[Section.shot]]);
    if (!shotFrom(S, D)) { met.away++; autoRest(A); continue; }   // (the view is somebody else's: the old loop does not read, and rests)
    const copy: AutoScene = autoRead(A, g, I, DT);
    n++; compare(seen, t, copy, S);
    met.threats += +!!S.threats.length; met.fight += +!!S.fight.length; met.beat += +!!S.beat; met.door += +!!S.door; met.room += +!!S.room; met.lock += +!!S.lock?.at; met.onto += +!!S.onto; met.hold += +S.hold; met.still += +S.still; met.shown += +!!S.shown; met['in close or his arm'] += +(S.zoomK < 0.85); met['a stalk'] += +!!S.lock?.stalk; met['the door under his hand'] += +(S.door?.w === 1);
  }
  const rows = [...seen].map(([k, [c, first]]) => `${k}: ${c} ticks, first ${first}`);
  line(n > 0 && !seen.size, `${name}: the row is the old read on ${n} ticks`, rows.join('\n    '));
}
const none = Object.entries(met).filter(([k, v]) => !v).map(([k]) => k);
line(!none.length, `the runs framed each kind of thing: ${Object.entries(met).map(([k, v]) => `${k} ${v}`).join(' · ')}`, none.length ? `never: ${none.join(', ')}` : '');
console.log(fails ? `shot: ${fails} failed` : 'shot: all hold');
process.exit(fails ? 1 : 0);
