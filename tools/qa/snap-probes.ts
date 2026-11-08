// GUNS, TRUNKS AND HEADS THAT SNAP — a watcher over whole scripted play-throughs (playthrough.ts's own runs, each in a fresh process as the tallies run them) and
// the probes that hold what it finds. The maintainer, 10-08: "the AI look crisp when clearing rooms, but i wonder if they're rotating too quickly. needs to
// feel realistic and reflect weapon weight". A SWING here = a net turn of MIN degrees or more inside WIN seconds (60° inside a tenth by default: a gun, a chest
// or a face that is somewhere else the next time you look) — its size the LARGEST net turn any span of it up to WIN long holds — read every frame after each armed man's own
// update:
//   gun     Character.gunYaw — where the MODEL has his gun: what the fire gates read (the turn weight's own yaw; a round leaves the DRAWN muzzle)
//   rifle   the yaw of Character.gunDir — the bore AS DRAWN (a long gun's is the hands' own line; the weapon light rides it), read only while it is within 60° of
//           level (muzzle up at port arms it has no heading). A carriage change throws this one where the model's yaw does not move
//   trunk   the SHOULDER LINE as posed (upper arm joint to upper arm joint, in the world) — what the camera over him sees of his chest: the aim's twist, the
//           long gun's carriage and every layer included; a hunch or a lean does not move it (the chest's own facing, 30‥45° nose down over a rifle, does)
//   head    the head's own left-right line as posed (Character.headRot) — the neck's lead and the gaze layer included; a nod does not move it
//   body    Character.bodyYaw — his feet (reported beside each swing, never a verdict by itself)
// A row is dropped, and the window with it, across a staging (moved more than character.ts TURN.jump in a frame), a ragdoll, a gap in his clock.
//   TPS_CAST=1 TPS_MOVES=1 TPS_TURN=1 bun run tools/qa/snap-probes.ts --one room-clear-four:1 [--min 60] [--win 0.1] [--cap 300] [--out f.json]
//                                                               [--trace 'Red Two' --from 16 --to 17.2]   that man's yaws every frame of the stretch
//   bun run tools/qa/snap-probes.ts --runs room-clear-four:1-8,compound-four:1,compound-four:3 [--turn 1|0|light|medium|heavy] [--jobs 4] [--json f.json]   the table, as the probes make it
//   bun run tools/qa/probes.ts snap          THE GATE. snap-page (the page's defaults: no swing of 60° in the model's gun yaw; nothing drawn — rifle, trunk,
//                                            head — comes DRAWN_MIN° round inside a tenth; the bang entries are in the sample) · snap-bites (the watcher's
//                                            arithmetic on rows made by hand: what is a swing and what is not; SNAPBITES_BREAK=1 must fail)
//   bun run tools/qa/probes.ts snap-open     THE OPEN FINDING, red today and so not one of the family's: the gun's own bar of 60° held to what is DRAWN. The model's
//                                            muzzle keeps it; the drawn rifle, the shoulders and the face pass it, mostly in the tenth a rifle changes carriage
//                                            (shouldered ↔ low ready) while the gun is swinging. The animator's to fix, and the fight reads those bones: a fix
//                                            moves where rounds go, so it wants its own switch and paired tally
// (the environment IS the arm: TPS_MOVES=1 with TPS_TURN=0 = the page under ?noturn; with TPS_TURN=1 = the page as it opens; TPS_CAST=1 = the cast's own meshes, which
// the trunk and the head are read off.) THE HARNESS HAS NO GPU: this reads the pose the page would draw, not a picture of it.
// (tools/qa/turn-rates.ts is the older, narrower watcher the turn weight's numbers were set against: aim yaw only, sided by training.)
import { resolve } from 'node:path';
import { tmpdir } from 'node:os';
import { readFileSync, writeFileSync, rmSync } from 'node:fs';
import { ROOT, castStood } from './headless.ts';

const DEG = 180 / Math.PI;
const { wrapAngle: wrap, quat } = await import(`${ROOT}/src/math/vec.ts`);
export const PARTS = ['gun', 'rifle', 'trunk', 'head'] as const;
export type Part = typeof PARTS[number];
export interface Swing { part: Part; who: string; side: string; at: number; deg: number; /** how far the gun stood off his feet when it ended, ° */ gap: number;
  /** over the same frames, signed the swing's own way: what his gun and his feet turned, ° — and what the animator was doing (his upper layer / long-gun carriage at each end, hit / blinded / reloading) */
  gun: number; body: number; note: string; /** seconds since the last stun charge went off (null: none yet) */ bang: number | null; state: string }
export interface SnapRun { scn: string; seed: number; moves: string; turn: string; /** the turn weight as the run's game had it while it played (character.ts TURN.on, read in the game's first update), not the word it was asked by */ turnOn: boolean; cast: boolean; end: string; t: number; team: string; min: number; win: number; bangs: number[]; men: number; frames: number; /** frame-to-frame turns read, per part (the rifle's only while its bore is level) */ read: Record<Part, number>; peak: Record<Part | 'body', number>; /** the furthest any gun stood off its man's feet, ° (the spine's twist ends at 109°) */ gapMax: number; /** man-frames it stood further off them than the AI's own twist limit lets it WISH (squad.ts / guards.ts: 1.5 rad = 86°) */ gapOver: number; swings: Swing[]; why?: string }
interface Row { t: number; gun: number; rifle: number; /** the drawn bore is near enough level to have a heading */ level: boolean; trunk: number; head: number; body: number; x: number; z: number; brk: boolean; state: string; pose: string }

/** the swings in one man's rows: every stretch that holds a net turn of `min`° or more inside ANY span of `win` seconds or less (a pop of three frames
 *  between a wind-up and a settle is a swing, though no whole tenth round it nets `min`). Spans that end within a tenth of each other are ONE swing: it is
 *  timed from the first and sized by the largest. A span never crosses a break (a staging, a ragdoll, a gap in his clock) nor, for the rifle, a frame its
 *  bore had no heading. `seen`, if given, is handed the number of frame-to-frame turns that were read — the floor under 'none found' */
export function swingsOf(L: Row[], part: Part, min: number, win: number, seen?: (n: number) => void): { at: number; deg: number; gap: number; state: string; gun: number; body: number; note: string }[] {
  const out: { at: number; deg: number; gap: number; state: string; gun: number; body: number; note: string }[] = []; let last = -Infinity, read = 0;
  const ok = (q: number) => !L[q].brk && (part !== 'rifle' || L[q].level && L[q - 1].level);
  for (let i = 1; i < L.length; i++) {
    if (!ok(i)) continue; read++;
    let net = 0, best = 0, bj = i;
    for (let q = i; q >= 1 && ok(q) && L[i].t - L[q - 1].t <= win + 0.005; q--) { net += wrap(L[q][part] - L[q - 1][part]); if (Math.abs(net) > Math.abs(best)) { best = net; bj = q - 1; } }
    const deg = Math.round(Math.abs(best) * DEG * 10) / 10; if (deg < min) continue;
    const along = (k: 'gun' | 'body') => { let a = 0; for (let q = bj + 1; q <= i; q++) a += wrap(L[q][k] - L[q - 1][k]); return Math.round(a * DEG * Math.sign(best)); };
    const S = { at: +L[bj].t.toFixed(2), deg, gap: Math.round(Math.abs(wrap(L[i].gun - L[i].body)) * DEG), state: L[i].state, gun: along('gun'), body: along('body'), note: L[bj].pose === L[i].pose ? L[i].pose : `${L[bj].pose} → ${L[i].pose}` };
    if (out.length && L[i].t - last <= win + 0.005) { const o = out[out.length - 1]; if (deg > o.deg) out[out.length - 1] = { ...S, at: o.at }; } else out.push(S);
    last = L[i].t;
  }
  seen?.(read);
  return out;
}

/** one play-through under the watcher, in THIS process (the environment is the arm) */
export async function snapRun(scn: string, seed: number, min = 60, win = 0.1, cap = 300, trace?: { who: string; from: number; to: number }): Promise<SnapRun> {
  const { play } = await import('./playthrough.ts');
  const { Character, TURN } = await import(`${ROOT}/src/game/character.ts`);
  const { Game } = await import(`${ROOT}/src/game/game.ts`);
  const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
  let now = 0, game: any = null; const bangs: number[] = [], seenBang = new WeakSet<any>();
  const gu = Game.prototype.update, cu = Character.prototype.update;
  const rows = new Map<any, Row[]>(), cut = new WeakSet<any>(), who = new Map<any, { name: string; side: string; gd: any } | null>();
  let turnOn: boolean | null = null;   // (the turn weight as the game has it WHILE it plays: read in its own update — play() puts every knob table back as it ends)
  Game.prototype.update = function (this: any, ...a: any[]) { now = this.time; game = this; turnOn ??= !!TURN.on; const r = gu.apply(this, a); for (const e of this.events) if (e.kind === 'bang' && !seenBang.has(e)) { seenBang.add(e); bangs.push(+e.time.toFixed(2)); } return r; };
  Character.prototype.update = function (this: any, dt: number) {
    cu.call(this, dt);
    if (this.id === PLAYER_ID || !(dt > 0) || !game) return;
    let W = who.get(this); if (W === undefined) { const gd = game.guards.find((x: any) => x.char === this); if (!gd) return; who.set(this, W = gd.team !== 'civilian' && !gd.civ ? { name: gd.callsign ?? gd.name ?? String(this.id), side: gd.team === 'operative' ? 'team' : 'theirs', gd } : null); }   // (not on the roster yet: asked again next frame)
    if (!W) return;
    let L = rows.get(this); if (!L) rows.set(this, L = []);
    if (this.ragdoll) { cut.add(this); return; }
    const b = this.bones, hl = quat.rotate(this.headRot(), [1, 0, 0]), gd = this.gunDir, p = L[L.length - 1], A = this.anim;   // (+X = the figure's left: a left-pointing line l faces atan2(−l.z, l.x))
    L.push({ t: now, gun: this.gunYaw, rifle: Math.atan2(gd[0], gd[2]), level: Math.hypot(gd[0], gd[2]) >= 0.5, trunk: Math.atan2(-(b.upperArmL[2] - b.upperArmR[2]), b.upperArmL[0] - b.upperArmR[0]), head: Math.atan2(-hl[2], hl[0]), body: this.bodyYaw, x: this.pos[0], z: this.pos[2], state: W.gd.state ?? '',
      pose: `${A.upper}/${A.longShown ?? '-'}${A.dazzle ? ' blinded' : ''}${A.reloading ? ' reloading' : ''}${A.inThrow ? ' throwing' : ''}${A.kicking ? ' kicking' : ''}`,
      brk: !p || cut.delete(this) || now - p.t > 0.05 || Math.hypot(this.pos[0] - p.x, this.pos[2] - p.z) > TURN.jump });
  };
  const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { };
  let r: any; try { r = await play(scn, seed, cap); } finally { console.log = l; console.error = e; console.warn = w; Game.prototype.update = gu; Character.prototype.update = cu; }
  const out: SnapRun = { scn, seed, moves: process.env.TPS_MOVES ?? '', turn: process.env.TPS_TURN ?? '', turnOn: turnOn ?? !!TURN.on, cast: castStood(), end: r.end, t: r.t, team: `${r.team}/${r.of}`, min, win, bangs, men: 0, frames: 0, read: { gun: 0, rifle: 0, trunk: 0, head: 0 }, peak: { gun: 0, rifle: 0, trunk: 0, head: 0, body: 0 }, gapMax: 0, gapOver: 0, swings: [] };
  if (r.end === 'error') out.why = r.why;
  for (const [ch, L] of rows) { const W = who.get(ch)!; out.men++; out.frames += L.length;
    for (let i = 1; i < L.length; i++) { const gap = Math.abs(wrap(L[i].gun - L[i].body)); out.gapMax = Math.max(out.gapMax, Math.round(gap * DEG)); if (gap > 1.5 + 1e-3) out.gapOver++; if (L[i].brk) continue; for (const k of ['gun', 'rifle', 'trunk', 'head', 'body'] as const) if (k !== 'rifle' || L[i].level && L[i - 1].level) out.peak[k] = Math.max(out.peak[k], Math.round(Math.abs(wrap(L[i][k] - L[i - 1][k])) / Math.max(1e-6, L[i].t - L[i - 1].t) * DEG)); }
    for (const part of PARTS) for (const s of swingsOf(L, part, min, win, n => { out.read[part] += n; })) { const b = bangs.filter(x => x <= s.at + win).pop(); out.swings.push({ part, who: W.name, side: W.side, at: s.at, deg: s.deg, gap: s.gap, gun: s.gun, body: s.body, note: s.note, bang: b === undefined ? null : +(s.at - b).toFixed(2), state: s.state }); }
    if (trace && W.name === trace.who) for (const q of L) if (q.t >= trace.from && q.t <= trace.to) l(`TR ${W.name} f${Math.round(q.t * 60)} ${q.t.toFixed(3)} gun ${(q.gun * DEG).toFixed(1)} rifle ${q.level ? (q.rifle * DEG).toFixed(1) : '(not level)'} trunk ${(q.trunk * DEG).toFixed(1)} head ${(q.head * DEG).toFixed(1)} body ${(q.body * DEG).toFixed(1)} at ${q.x.toFixed(2)},${q.z.toFixed(2)} ${q.state}${q.brk ? ' (break)' : ''}`); }
  out.swings.sort((a, b) => a.at - b.at);
  return out;
}

const line = (r: SnapRun) => `${`${r.scn}:${r.seed}`.padEnd(20)} ${r.end} ${r.t} s · team ${r.team} · ${r.men} men · peak °/s gun ${r.peak.gun} rifle ${r.peak.rifle} trunk ${r.peak.trunk} head ${r.peak.head} body ${r.peak.body} · gun at most ${r.gapMax}° off his feet (${r.gapOver} frames past 86°) · swings ≥${r.min}° in ${r.win} s: ${PARTS.map(p => `${p} ${r.swings.filter(s => s.part === p).length}`).join(' ')} · ${r.bangs.length} charges`;
const swingLine = (r: SnapRun, s: Swing) => `    ${r.scn}:${r.seed} @ ${s.at} s · ${s.who} (${s.side}) · ${s.part} ${s.deg}°${s.part === 'gun' ? '' : ` (his gun ${s.gun}°`}${s.part === 'gun' ? ` (his feet ${s.body}°)` : `, his feet ${s.body}°)`} · ${s.note} · gun ${s.gap}° off his feet · ${s.state}${s.bang !== null && s.bang <= BANG_SECS ? ` · ${s.bang} s after a charge` : ''}`;
/** how long after a stun charge goes off a swing still counts as 'after a bang entry' */
export const BANG_SECS = 8;

/** `room-clear-four:1-8,compound-four:1,compound-four:3` → the runs */
export function parseRuns(spec: string): { scn: string; seed: number }[] {
  return spec.split(',').flatMap(p => { const [scn, rg] = p.split(':'), [a, b = a] = (rg ?? '1').split('-').map(Number);
    if (!scn || !Number.isInteger(a) || !Number.isInteger(b) || a < 1 || b < a) throw new Error(`runs are <scenario>:<seed> or <scenario>:<lo>-<hi>, got '${p}'`);
    return Array.from({ length: b - a + 1 }, (_, i) => ({ scn, seed: a + i })); });
}
/** the runs, each in its own process with `env` over this one's, `jobs` at a time */
export async function snapRuns(runs: { scn: string; seed: number }[], env: Record<string, string | undefined>, jobs = 4, min = 60, win = 0.1): Promise<SnapRun[]> {
  const out: SnapRun[] = []; let next = 0;
  const E: Record<string, string> = {}; for (const [k, v] of Object.entries({ ...process.env, ...env })) if (v !== undefined) E[k] = v;
  await Promise.all(Array.from({ length: Math.max(1, jobs) }, async () => { for (;;) { const j = runs[next++]; if (!j) return;
    const file = resolve(tmpdir(), `snap-${process.pid}-${j.scn}-${j.seed}-${Math.random().toString(36).slice(2)}.json`);
    const pr = Bun.spawn(['bun', 'run', import.meta.path, '--one', `${j.scn}:${j.seed}`, '--out', file, '--min', String(min), '--win', String(win)], { stdout: 'ignore', stderr: 'pipe', env: E });
    const errText = await new Response(pr.stderr).text(); await pr.exited;
    let r: SnapRun; try { r = JSON.parse(readFileSync(file, 'utf8')); rmSync(file); } catch { r = { scn: j.scn, seed: j.seed, moves: E.TPS_MOVES ?? '', turn: E.TPS_TURN ?? '', turnOn: false, cast: !!E.TPS_CAST, end: 'error', t: 0, team: '0/0', min, win, bangs: [], men: 0, frames: 0, read: { gun: 0, rifle: 0, trunk: 0, head: 0 }, peak: { gun: 0, rifle: 0, trunk: 0, head: 0, body: 0 }, gapMax: 0, gapOver: 0, swings: [], why: `the run left no result — ${errText.trim().split('\n').slice(-2).join(' / ').slice(0, 300)}` }; }
    out.push(r); } }));
  return out.sort((a, b) => a.scn === b.scn ? a.seed - b.seed : runs.findIndex(x => x.scn === a.scn) - runs.findIndex(x => x.scn === b.scn));
}

// ───────────────────────────────────────────────────────────── the probes ─────────────────────────────────────────────────────────────
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
/** the page as it opens: the cast, the movement pass, the turn weight as shipped */
const PAGE = { TPS_CAST: '1', TPS_MOVES: '1', TPS_TURN: '1' };
const ROOMS = 'room-clear-four:1-8', SQUADS = 'compound-four:1,compound-four:3';

/** THE GATE'S BAR FOR WHAT IS DRAWN: 100° inside a tenth. The model's gun yaw is held to 60° (it may come round 52° in a tenth at the shipped weight —
 *  character.ts TURN_FEEL.light × a carbine's handling; a handling-1 pistol would reach 63°, and no fighting man in these runs carries one). The rifle as drawn,
 *  the shoulders and the face go with that yaw AND with the long gun's carriage, and pass 60° today (snap-open counts those: the largest 63° / 83° / 80°). This bar
 *  is the regression guard over them: 17° over where the page stands, and the same runs with the weight off hold 9 or 10 of each past it. */
export const DRAWN_MIN = 100;
const most = (rs: SnapRun[], part: Part) => Math.max(0, ...rs.flatMap(r => r.swings.filter(s => s.part === part).map(s => s.deg)));
const ALL = parseRuns(`${ROOMS},${SQUADS}`);

export async function snapBites(): Promise<boolean> {
  const F = family('snap-bites: the watcher finds the snaps where they are — a turn in one frame');
  const BRK = process.env.SNAPBITES_BREAK === '1';   // (the red path: the staged quarter turn is never made, so the second line has nothing to find)
  { const L: Row[] = []; let y = 0; for (let i = 0; i < 40; i++) { if (i === 20 && !BRK) y += 90 / DEG; L.push({ pose: '', t: i / 60, gun: y, rifle: i >= 10 && i < 22 ? (i - 9) * 20 / DEG : i >= 22 ? 240 / DEG : 0, level: i < 30, trunk: i >= 20 && i < 26 ? (i - 19) * 11 / DEG : i >= 26 ? 66 / DEG : 0, head: i * 9 / DEG, body: 0, x: 0, z: 0, brk: i === 0, state: '' }); }
    const g = swingsOf(L, 'gun', 60, 0.1), r = swingsOf(L, 'rifle', 60, 0.1), t = swingsOf(L, 'trunk', 60, 0.1), h = swingsOf(L, 'head', 60, 0.1);
    // (what the watcher must NOT read, and the pop a whole-tenth net would have lost)
    const row = (i: number, o: Partial<Row>): Row => ({ pose: '', t: i / 60, gun: 0, rifle: 0, level: true, trunk: 0, head: 0, body: 0, x: 0, z: 0, brk: i === 0, state: '', ...o });
    const cut = Array.from({ length: 30 }, (_, i) => row(i, { gun: i >= 15 ? 90 / DEG : 0, brk: i === 0 || i === 15 })), lev = Array.from({ length: 30 }, (_, i) => row(i, { rifle: i >= 15 ? 90 / DEG : 0, level: i !== 15 }));
    const popY = [0, -7, -14, -21, 4, 29, 54, 47, 40, 33], pop = Array.from({ length: 30 }, (_, i) => row(i, { trunk: (i < 10 ? 0 : i < 20 ? popY[i - 10] : 33) / DEG }));
    const cg = swingsOf(cut, 'gun', 60, 0.1), cr = swingsOf(lev, 'rifle', 60, 0.1), lg = swingsOf(lev, 'gun', 60, 0.1), pt = swingsOf(pop, 'trunk', 60, 0.1);
    F.verdict(cg.length === 0 && cr.length === 0 && lg.length === 0 && pt.length === 1 && pt[0].deg === 75, 'a 90° turn across a staging is none, nor is one the bore was not level for; 75° there in three frames between 21° the other way each side is one swing of 75° (the whole tenth nets 33°)', `gun across a staging ${cg.map(s => s.deg + '°').join(' ') || 'none'} · rifle off level ${cr.map(s => s.deg + '°').join(' ') || 'none'} · the pop ${pt.map(s => s.deg + '°').join(' ') || 'none'}`);
    F.verdict(g.length === 1 && g[0].deg === 90 && r.length === 1 && r[0].deg === 120 && t.length === 1 && t[0].deg === 66 && h.length === 0, 'a 90° turn in one frame is one swing of 90°, 20° a frame for twelve frames is one of 120° (its largest tenth, not the first to reach 60°), 66° over six frames is one, a steady 54° a tenth is none', `gun ${g.map(s => s.deg + '°').join(' ') || 'none'} · rifle ${r.map(s => s.deg + '°').join(' ') || 'none'} · trunk ${t.map(s => s.deg + '°').join(' ') || 'none'} · head ${h.length}`); }
  // (until 2026-11-07 the probe went on to play both missions with the turn weight off, the page under ?noturn, and asked that the watcher DOES find gun
  //  and drawn swings there, and after a stun charge: five lines on the off side of the turn weight, which is settled on. They are out; what is left is the
  //  watcher's own arithmetic on rows made here)
  return F.done();
}

if (import.meta.main) {
  const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; }, num = (n: string, d: number) => flag(n) === undefined ? d : Number(flag(n));
  const min = num('--min', 60), win = num('--win', 0.1), cap = num('--cap', 300);
  if (flag('--one')) { const [scn, seed] = flag('--one')!.split(':');
    const r = await snapRun(scn, Number(seed), min, win, cap, flag('--trace') ? { who: flag('--trace')!, from: num('--from', 0), to: num('--to', 1e9) } : undefined);
    if (flag('--out')) writeFileSync(flag('--out')!, JSON.stringify(r)); else { console.log(line(r)); for (const s of r.swings) console.log(swingLine(r, s)); if (r.why) console.log(r.why); }
    process.exit(0); }
  if (flag('--runs')) { const turn = flag('--turn') ?? '1';
    const rs = await snapRuns(parseRuns(flag('--runs')!), { ...PAGE, TPS_TURN: turn }, num('--jobs', 4), min, win);
    for (const r of rs) { console.log(line(r)); for (const s of r.swings) console.log(swingLine(r, s)); if (r.why) console.log(r.why); }
    if (flag('--json')) writeFileSync(flag('--json')!, JSON.stringify(rs, null, 1));
    process.exit(rs.some(r => r.end === 'error') ? 1 : 0); }
  console.error('snap-probes.ts --one <scenario>:<seed> | --runs <scenario>:<lo-hi>,… [--turn 1|0|light|medium|heavy]   (or: bun run tools/qa/probes.ts snap)'); process.exit(2);
}
