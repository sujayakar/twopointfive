// A SEEDED STREAM OF PERTURBATIONS LAID OVER ANY HEADLESS TOOL, between two ticks (tools/blitz/PLAY.md, "The fuzz"). The scripted missions drive the game
// through one road; a person at the keys does a hundred other things. `QA_PERTURB=<seed>` in front of a tool that stands its game through
// tools/qa/headless.ts (the scripted player of playthrough.ts, a lesson, a probe) makes that harness call `lay(W)`: from then on, before every step, a
// schedule of its own (one seeded stream; never the game's die, never `Math.random`) may do one of the things below THROUGH THE GAME'S OWN DOORS, as the
// menu, the keys or a tool would:
//   order      a row the orders menu offers on a random target (a door, a floor spot, one of the team, a man of theirs), its pages' rows too, to all or
//              to whoever the standing choice is: `orders.rowsFor(target, page)` then `orders.give(id, target, who)`. A refusal is logged with its words
//   fallin     `orders.give('fallIn')`: an order under way called off
//   throw      a frag, a flash or a smoke can dropped at a random spot near one of the team (`game.dropThrowable`)
//   fire       the player's round at a random living man of theirs (`fireWeapon`)
//   kill       one of theirs killed; now and then one of the team (`killGuard`)
//   word       the hostages told to follow, where the mission has any (`rescue.ending.word`)
//   mains      the mains cut for some seconds (`game.setBlackout`)
//   save/load  the save controller's own save by hand and the load of the last one made (`saves.write`, `saves.load`)
//   restart    the pause card's restart (`game.restart`)
// QA_PERTURB_ALARM=<tick>: THE RESPONSE TEAM CALLED at that tick, as tools/qa/qrf-squad-check.ts calls it (the response switched on, the squad's own door and
// room code for it (`tune.qrfSquad`), four men fourteen seconds out, the alarm's place put on one of the team, `requestQrf(g, 'panel')`), and the alarm's
// place put on one of the team again every twenty seconds while they are out: two casts of the door tools on one floor.
// A level with no response team of its own refuses the call (the compound and the warehouse have none; the depot, the records office, the bazaar, the
// crossing and the police post have): the line then says `called (false)` and `no-team`.
// QA_PERTURB_GOD=player|squad|all: the sandbox row's god mode held on (`game.godMode`: the player is not hit), and / or the net a showcase cast wears put on
// every living man of the team (`guard.god`: rounds and fragments pass through him), from the first perturbed tick and again on whoever a restart or a
// load brings: a run that lasts, with the team's deaths, the player's, or neither.
// QA_PERTURB_RATE=<k> scales every rate (1); QA_PERTURB_ONLY / QA_PERTURB_SKIP=<kinds, comma>; QA_PERTURB_FROM=<tick> (120).
// THE LOG: a line a perturbation in `<RH_TICKS>/<word>.log`, in the soak's own shape (`  [seed N] 12.34s f741 STIM order: …`), so tools/blitz/fuzz-diff.ts
// reads a squad run as it reads a soak. A perturbation that throws is logged with what it threw (its stack in `<word>.throws.txt`) and the play goes on.
// It is a tool's layer: nothing here runs unless the word is given, and nothing of src reads it.
import { appendFileSync, writeFileSync } from 'node:fs';
const ROOT = new URL('../..', import.meta.url).pathname.replace(/\/$/, '');
type V3 = [number, number, number];
const RATES: Record<string, number> = { order: 7, fallin: 1.2, throw: 1.6, fire: 1.6, kill: 0.7, word: 0.5, mains: 0.2, save: 0.35, load: 0.3, restart: 0.04 };   // (a minute)
const mulberry = (a: number) => () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };

export async function lay(W: any): Promise<void> {
  const seed = Number(process.env.QA_PERTURB); if (!Number.isFinite(seed)) throw new Error(`QA_PERTURB=${process.env.QA_PERTURB}: a seed (a number)`);
  const { killGuard, fireWeapon } = await import(`${ROOT}/src/game/combat.ts`); const { requestQrf } = await import(`${ROOT}/src/game/qrf.ts`);
  const g = W.game, rng = mulberry(seed * 104729 + 977), K = Number(process.env.QA_PERTURB_RATE ?? 1), FROM = Number(process.env.QA_PERTURB_FROM ?? 120);
  const only = new Set((process.env.QA_PERTURB_ONLY ?? '').split(',').filter(Boolean)), skip = new Set((process.env.QA_PERTURB_SKIP ?? '').split(',').filter(Boolean));
  const kinds = Object.keys(RATES).filter(k => !skip.has(k) && (!only.size || only.has(k)));
  const word = process.env.TPS_GAME_BY ?? process.env.TPS_GAME ?? 'none', dir = process.env.RH_TICKS ?? '', file = dir ? `${dir}/${word}.log` : '';
  if (file) writeFileSync(file, '');
  let n = 0; const saved: string[] = [];
  const say = (s: string) => { const line = `  [seed ${seed}] ${g.time.toFixed(2)}s f${n} ${s}`; if (file) appendFileSync(file, line + '\n'); else console.log(line); };
  const pick = <T>(a: T[]): T => a[Math.floor(rng() * a.length)], rr = (a: number, b: number) => a + rng() * (b - a), at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;
  const team = (): any[] => (g.orders?.squad?.() ?? []).filter((x: any) => x.state !== 'dead'), theirs = (): any[] => g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead');
  const near = (p: number[], r: number): V3 => { const a = rr(0, Math.PI * 2), d = rr(0.5, r); return [p[0] + Math.sin(a) * d, 0, p[2] + Math.cos(a) * d]; };
  const name = (x: any) => x?.callsign ?? x?.name ?? `g${g.guards.indexOf(x)}`;
  const DO: Record<string, () => string | null> = {
    order() { const O = g.orders, T = team(); if (!O?.active || !T.length) return null;
      const r = rng(); let p: number[]; let what: string;
      if (r < 0.45 && g.doors.list.length) { const m = pick(T).char.pos, D = [...g.doors.list].sort((a: any, b: any) => Math.hypot(a.pos[0] - m[0], a.pos[2] - m[2]) - Math.hypot(b.pos[0] - m[0], b.pos[2] - m[2])).slice(0, 5), d = pick(D) as any; p = d.pos; what = `door ${g.doors.list.indexOf(d)}`; }
      else if (r < 0.75) { p = near(pick(T).char.pos, 7); what = `the floor ${at(p)}`; }
      else if (r < 0.9) { const m = pick(T); p = m.char.pos; what = name(m); }
      else { const F = theirs(); if (!F.length) return null; const m = pick(F); p = m.char.pos; what = `${name(m)} (theirs)`; }
      const t = O.targetAt(p as V3); let rows: any[] = O.rowsFor(t); const pages = rows.filter((d: any) => d.opens); rows = rows.filter((d: any) => !d.opens);
      if (pages.length && rng() < 0.3) { const pg = pick(pages); rows = O.rowsFor(t, pg.opens).filter((d: any) => !d.opens); what += `, page ${pg.opens}`; }
      if (!rows.length) return `no row on ${what} (${t.kind})`;
      const def = pick(rows), all = rng() < 0.7, res = all ? O.give(def.id, t, 'all') : O.give(def.id, t);
      return `${def.id} on ${what} (${t.kind}) to ${all ? 'all' : 'the standing choice'} -> ${res === null ? 'given' : `refused: ${res}`}`; },
    fallin() { const O = g.orders; if (!O?.active) return null; const res = O.give('fallIn', undefined, 'all'); return res === null ? 'given' : `refused: ${res}`; },
    throw() { const T = team(); if (!T.length) return null; const kind = pick(['frag', 'flash', 'smoke']), p = near(pick(T).char.pos, 6); p[1] = 1; g.dropThrowable(kind, p); return `${kind} at ${at(p)}`; },
    fire() { const F = theirs(); if (!F.length || g.player.down) return null; const m = pick(F); fireWeapon(g, g.player.char, [m.char.pos[0], m.char.pos[1] + 1.25, m.char.pos[2]], true); return `at ${name(m)}`; },
    kill() { const mate = rng() < 0.15, L = mate ? team() : theirs(); if (!L.length) return null; const m = pick(L), a = rr(-Math.PI, Math.PI); killGuard(g, m, [Math.sin(a), 0, Math.cos(a)], false); return `${name(m)}${mate ? ' (one of the team)' : ''}`; },
    word() { const E = g.rescue?.ending; if (!E?.word) return null; const r = E.word(g.player.char); return `the hostages told to follow -> ${r}`; },
    mains() { if (g.blackout?.active) { g.endBlackout(); return 'back on'; } const s = rr(5, 30); g.setBlackout(s, 'ocp'); return `cut for ${s.toFixed(0)} s`; },
    save() { const S = g.saves; if (!S) return null; const r = S.write(g, 'manual', `fuzz ${n}`); if (r.ok && r.info?.key) saved.push(r.info.key); return r.ok ? `kept as ${String(r.info?.key ?? '').replace(/^.*save\./, '')}` : `refused: ${r.why}`; },
    load() { const S = g.saves; if (!S || !saved.length) return null; const key = saved[saved.length - 1], why = S.load(g, key); return why ? `refused: ${why}` : `${key.replace(/^.*save\./, '')} put back`; },
    restart() { g.restart('fuzz: the pause card\'s restart'); saved.length = 0; return 'the pause card'; },
  };
  const ALARM = process.env.QA_PERTURB_ALARM ? Number(process.env.QA_PERTURB_ALARM) : -1;
  // (THE NAMES THIS FILE WRITES ON THE GAME. The walk for writes by name watches what the files of tools/qa write, read as each loads (tools/blitz/expose.ts);
  // this file is not one of them, so it says its own: without this the Rust game dealt the team at the page's 75 s while the puppet read 14.)
  if (ALARM >= 0) { const TW = (globalThis as any).__tpsToolWrites; if (TW?.names) { for (const k of ['qrf', 'qrfSquad', 'qrfEta', 'qrfSize', 'placed', 'pos', 'raisedAt']) TW.names.add(k); TW.version++; } }
  const alarm = (first: boolean): string | null => { const T = team(), m = T.length ? pick(T).char.pos : g.player.char.pos, Q = g.qrf; if (!Q) return null;
    if (first) { g.tune.qrf = true; g.tune.qrfSquad = true; g.tune.qrfEta = 14; g.tune.qrfSize = 4; }
    g.alarm.placed = true; g.alarm.pos = [m[0], m[1], m[2]]; g.alarm.raisedAt = g.time;
    const asked = first ? requestQrf(g, 'panel') : null;
    return `${first ? `the response team called (${asked})` : 'the alarm\'s place moved'} to ${at(m)} · state ${Q.state}${Q.cast ? ' · the cast is built' : ''}${Q.op ? ' · on an entry' : ''} · its last note: ${Q.log?.[Q.log.length - 1]?.what ?? 'none'}`; };
  const GOD = process.env.QA_PERTURB_GOD ?? '', godPl = GOD === 'player' || GOD === 'all', godSq = GOD === 'squad' || GOD === 'all';
  if (GOD && !godPl && !godSq) throw new Error(`QA_PERTURB_GOD=${GOD}: player, squad or all`);
  if (GOD) { const TW = (globalThis as any).__tpsToolWrites; if (TW?.names) { for (const k of ['godMode', 'god']) TW.names.add(k); TW.version++; } }
  const god = (): void => { if (godPl && !g.godMode) { g.godMode = true; say('god mode put on (the player)'); } if (godSq) for (const m of team()) if (!m.god) { m.god = true; say(`the net put on ${name(m)}`); } };
  const step = W.step;
  W.step = function (this: any, ...a: any[]) {
    if (ALARM >= 0 && n >= ALARM && (n - ALARM) % 1200 === 0 && !g.ticking) { let d: string | null; try { d = alarm(n === ALARM); } catch (e) { d = `THREW ${String((e as Error)?.message ?? e).slice(0, 200)}`; if (file) appendFileSync(file.replace(/\.log$/, '.throws.txt'), `f${n} alarm: ${String((e as Error)?.stack ?? e)}\n`); } if (d !== null) say(`STIM alarm: ${d}`); }
    if (GOD && n >= FROM && !g.ticking) god();
    if (n >= FROM && !g.ticking) for (const k of kinds) { if (rng() >= RATES[k] * K / 3600) continue; let d: string | null; try { d = DO[k](); } catch (e) { d = `THREW ${String((e as Error)?.message ?? e).slice(0, 200)}`; if (file) appendFileSync(file.replace(/\.log$/, '.throws.txt'), `f${n} ${k}: ${String((e as Error)?.stack ?? e)}\n`); } if (d !== null) say(`STIM ${k}: ${d}`); }
    n++; return step.apply(this, a); };
  say(`--- perturbations laid, seed ${seed}: ${kinds.join(', ')}; rates a minute x${K}${GOD ? `; god: ${GOD}` : ''}${ALARM >= 0 ? `; the response team called at tick ${ALARM}` : ''}`);
}
