// FRAME COST, THE HOOKS — a bun PRELOAD (`bun --preload tools/qa/framecost-hooks.ts run tools/qa/<any harness tool>.ts …`) that times where a frame of the headless game goes,
// split by system, WITHOUT a line of src/ or of the harness changed on disk: a Bun.plugin `onLoad` rewrites two files AS THEY ARE LOADED —
//   src/game/game.ts     a `__FC.m(k)` statement in front of each statement of the game's step that starts another system (the MARKS table below: 'from here on the
//                        clock is on bucket k'), so the update's wall time is split exactly, nothing sampled. A mark names the CALL; the statement that holds it is
//                        looked for in the step and in every method of the class the step calls, wherever it stands (`stepRanges`, `statementWith`);
//   tools/qa/headless.ts  W.step bracketed (`__FC.begin()` … `__FC.end()`), the camera / the light set's ageing / the box list's upload each on its own bucket, and
//                        `__FC.stood(W)` on the freshly stood game — which (FC_DEEP, on by default) wraps the cross-cutting SERVICES every system calls into, on their
//                        prototypes: the animator + skeleton (Character.update), A* (NavGrid.findPath), sound propagation, the static line tests (sight / fire / reach), the
//                        circle collide. A service's time comes OUT of the system that called it and into its own bucket (outermost call only: a path's own line tests
//                        are the path's).
// Markers call performance.now() and add into a Float64Array: no dice drawn, nothing the simulation reads written — a run's PT_DIGEST is the same with the preload and
// without it (tools/qa/framecost.ts --verify makes that check). A mark whose call is gone (somebody re-wrote the step) is reported in the output's `missing`, and its
// time simply stays on the bucket before it: the tool keeps working, the split gets coarser there, and the table says so. `framecost.ts --marks` asks only that
// question, in two seconds and with nothing played, and is red when a mark is missing: the thing to run after a change to the step's own lines.
// At exit the per-frame matrix goes to $FC_OUT (JSON: bucket names, frames × buckets in ms as base64 Float32, the services' call counts) — tools/qa/framecost.ts is the
// one command that runs the missions under this and prints the tables. FC_DEEP=0: no service wraps (the cheapest clock: ~60 timestamps a frame).
import { plugin } from 'bun';
import { writeFileSync } from 'node:fs';

const MAXB = 64;
const NAMES: string[] = [];
const bucket = (n: string): number => { let i = NAMES.indexOf(n); if (i < 0) { i = NAMES.length; NAMES.push(n); if (i >= MAXB - 1) throw new Error('framecost: too many buckets'); } return i; };

// ── the buckets, in the order the tables print them. 'step:' rows are outside Game.update (the harness's W.step); 'svc:' rows are the services (FC_DEEP)
const B_PRE = bucket('update: head (journal, clocks)');
/** Game.update's statements, by the CALL that starts another system → the bucket that statement and everything after it is charged to, up to the next mark.
 *  A mark names the call and not who it is made on or what stands in front of it (`this.` or a seat's view, a loop over the seats, a guard on the count of
 *  them): the step was once re-written from one player to seats and sixteen marks that spelt the receiver were lost with their time (`statementWith`). */
const MARKS: [string, string][] = [
  ['useDice(this.dice);', 'update: head (the record loaded, clocks)'],
  ['this.senseStep(', 'senses on the CPU (the smoke and the light the game reads)'],
  ['S[0].load(', 'update: head (the record loaded, clocks)'],
  ['updateAim(', 'player (aim, interact, move, fire)'],
  ['.orders.input(', 'squad: order menu + orders'],
  ['.doorTools.menuKey(', 'squad: door tools'],
  ['updateInteract(', 'player (aim, interact, move, fire)'],
  ['updateSuppression(this', 'suppression'],
  ['updatePlayer(', 'player (aim, interact, move, fire)'],
  ['if (this.rings.length) updateRings(', 'gear: rings, 40 mm, frags'],
  ['this.searchProbeN = 0', 'guards: weapons dealt, anim flags'],
  ['this.director.tick(', 'scenario (director, breach, room clear, rescue)'],
  ['this.pair.input(', 'pair (two person stealth)'],
  ['.wallCharge.input(', 'destruction: wall charge, fire at cover'],
  ['.doorTools.input(', 'squad: door tools'],
  ['this.nightBreach.tick(', 'scenario (director, breach, room clear, rescue)'],
  ['wallReactTick(this', 'destruction: both sides react to a break'],
  ['this.entry.tick()', 'squad: entry points, cleared rooms, cover'],
  ['.doorTools.tick()', 'squad: door tools'],
  ['this.cover.tick()', 'squad: entry points, cleared rooms, cover'],
  ['silentTick(', 'squad: order menu + orders'],
  ['this.pair.tick(', 'pair (two person stealth)'],
  ['.orders.tick()', 'squad: order menu + orders'],
  ['this.fight.tick(', 'room fight (both sides: watch, fear, shoot)'],
  ['nvgTick(this)', 'squad: order menu + orders'],
  ['tacticsTick(this', 'tactics (flank, lone corner, doorways)'],
  ['needsTick(this', 'needs (patrol variation)'],
  ['fragThrowTick(this', 'gear: rings, 40 mm, frags'],
  ['updateGuard(', 'guards: updateGuard (senses, state, feet)'],
  ['coverfireTick(this', 'destruction: wall charge, fire at cover'],
  ['updateCivilians(this', 'civilians'],
  ['updateEscalation(this', 'escalation + QRF'],
  ['this.updateDoors(', 'doors'],
  ['updateMission(', 'mission'],
  ['this.updateProps(', 'props (furniture)'],
  ['this.destr.update(', 'destruction (heat, debris)'],
  ['this.events = this.events.filter(', 'fixtures + devices'],
  ['this.weather.update(', 'weather, wet, wind, moths, rain fx (game half)'],
  ['updateItems(this', 'items (throwables, drops)'],
  ['updateGlass(this)', 'glass (stars, breaking, shards)'],
  ['this.fogOfWar.update(', 'fog of war'],
  ['const all: Character[] =', 'dynamic boxes (leaves, props, debris, rig props, fx)'],
  ['this.drawCharacters(', 'characters → skin instances + shadow proxies'],
  ['this.lightMeter(', 'sensors (meter + smoke line asks)'],
  ['.player.hitFlash = Math.max(', 'update: tail (cues, the squad\'s net, tallies, the death recap)'],
  ['this.saves.tick(', 'checkpoints (the autosave\'s triggers and its take)'],
  ['this.journal.frameEnd()', 'journal + replay tape (the frame closed, the host\'s after)'],
  ['recapTick(this)', 'update: tail (cues, the squad\'s net, tallies, the death recap)'],
  ['this.replayTap?.(dt);', 'journal + replay tape (the frame closed, the host\'s after)'],
];
/** …and the marks that stand INSIDE a method the step calls (the part of the frame that method is): looked for in the methods only */
const INNER: [string, string][] = [
  ['if (!this.tune.tracers && this.tracers.live)', 'tracers + sparks'],
  ['// characters wading through smoke', 'smoke pushes'],
];
if (process.env.FC_MARKS_BREAK === '1') MARKS.push(['aCallTheStepDoesNotMake(', 'update: head (the record loaded, clocks)']);   // (the red path of `framecost.ts --marks`: a mark whose call is nowhere must be reported)
for (const [, n] of [...MARKS, ...INNER]) bucket(n);
const B_CAM = bucket('step: camera'), B_LIGHTS = bucket('step: lights.update'), B_UPLOAD = bucket('step: box list pack + trace grid (world.upload)');
// FC_GPU=1 (tools/qa/framecost-gpu.ts: the real Engine on a recording device): the engine's own CPU half of a frame, which the stub harness never runs
const GPU = process.env.FC_GPU === '1';
// destruction as the page raises it (src/main.ts: on unless ?nodestruction): the play-through reads PT_DESTR itself, the soak does not — so the switch is raised here for both
if (process.env.PT_DESTR === '1') { const m: any = await import('../../src/game/destructionSwitch.ts').catch(() => null); if (m?.DESTRUCTION) m.DESTRUCTION.on = true; }
const B_SMOKE = bucket('step: smoke solver update (CPU half)'), B_RENDER = bucket('render: frame uniforms, probe, submit (the rest of Engine.render)'),
  R_LIGHTS = bucket('render: light list + reach / glass masks upload'), R_DECALS = bucket('render: decal list'), R_PART = bucket('render: particles (births → GPU records, update encode; the splat encode is under the lighting passes)'),
  R_SMOKE = bucket('render: smoke solver encode'), R_GB = bucket('render: g-buffer + mesh shadows encode, motion snapshot'), R_RC = bucket('render: cascades encode'), R_ENC = bucket('render: lighting passes encode');
const S_ANIM = bucket('svc: animation + skeleton (Character.update)'), S_PATH = bucket('svc: nav paths (A*, walkable, nearest free)'), S_SOUND = bucket('svc: sound propagation'),
  S_LINE = bucket('svc: static line tests (sight, fire, reach)'), S_COLLIDE = bucket('svc: circle collide (feet vs walls)'), S_RAY = bucket('svc: box world raycast (cursor, lasers, rounds)'),
  S_REACH = bucket('destruction: light reach masks re-decided after a wall breaks (the keeper of the picture, render/reachKeep.ts tick: none on a stand with no picture)'),
  S_WALL = bucket('destruction: a wall or cover breaking (rounds and blasts on it: holes, pieces, re-baked paths)');   // (the wall engine's two ways in, wherever in the frame a round or a blast lands)
// HEADLESS ONLY — left OUT of the frame by tools/qa/framecost.ts: where the light probe has not answered, a meter falls back to a CPU estimate of the same direct light
// (guards.ts lightEstimateAt: every light × a line test). Headless the probe NEVER answers, so that estimate runs for every armed man every frame; on the page the GPU answers
// within a frame or two and it does not run at all. Its time (its line tests included) is taken out of the system that asked and kept apart under this name.
const X_STANDIN = bucket('headless only: CPU light estimate standing in for the GPU light probe');
const B_FIRST_STEP = B_CAM, B_FIRST_SVC = S_ANIM;

const now = () => performance.now();
const acc = new Float64Array(MAXB);
let cur = B_PRE, last = 0, inFrame = false, svc = 0, game: any = null;
const frames: Float32Array[] = [];
const calls = new Float64Array(MAXB), callsOuter = new Float64Array(MAXB), lineBy = new Float64Array(MAXB);   // (lineBy: the static line tests' ms by the bucket that asked)
const missing: string[] = [], matched: string[] = [];
let uploads = 0;

const FC = {
  B_SMOKE, B_RENDER, B_CAM, B_LIGHTS,
  /** what the two files' rewrite found, once they are loaded (framecost.ts --marks) */
  report: () => ({ marks: matched.slice(), placed: placed.slice(), harness: harnessPlaced.slice(), missing: [...missing, ...standInMissing()], standIns: standInSites }),
  begin() { acc.fill(0); cur = B_PRE; inFrame = true; svc = 0; last = now(); },
  m(i: number) { if (!inFrame) return; const t = now(); acc[cur] += t - last; last = t; cur = i; },
  up() { uploads++; },
  end() { if (!inFrame) return; const t = now(); acc[cur] += t - last; inFrame = false; const row = new Float32Array(NAMES.length + 1); for (let i = 0; i < NAMES.length; i++) row[i] = acc[i]; row[NAMES.length] = game ? game.time : 0; frames.push(row); },
  /** one of the stand-in sites (see X_STANDIN): `fn` runs as it would, its time on the headless-only bucket */
  standIn(fn: () => number): number {
    if (!inFrame) return fn();
    calls[X_STANDIN]++;
    if (svc) return fn();
    svc = 1; callsOuter[X_STANDIN]++; const t = now(); acc[cur] += t - last; last = t; const prev = cur; cur = X_STANDIN;
    try { return fn(); } finally { const t2 = now(); acc[X_STANDIN] += t2 - last; last = t2; cur = prev; svc = 0; }
  },
  /** the game is stood: wrap the services (FC_DEEP) on their prototypes — each a pass-through that moves the clock onto its bucket for the OUTERMOST call */
  stood(W: any) {
    game = W.game;
    // CHARACTER MATERIALS, as the page raises them (main.ts: on by itself for a mesh that carries slot-named materials = the cast): the per-man palette + proxy rows are part of 'characters → skin'
    if (process.env.FC_CHARMATS !== '0') { try { const cm = W.__fcCharMats; if (cm?.meshAuthored?.(W.game.ch?.mesh) && W.engine?.settings) { W.engine.settings.charMats = true; cm.charSlots?.(W.game.ch); } } catch { } }   // (+ the slot data solved now, as the page's boot does)
    if (process.env.FC_DEEP === '0') return;
    const wrap = (obj: any, name: string, b: number) => {
      let proto = obj; while (proto && !Object.hasOwn(proto, name)) proto = Object.getPrototypeOf(proto);   // (where the method lives: a prototype, or the object itself for a plain-object service)
      const fn = proto?.[name];
      if (typeof fn !== 'function' || fn.__fc) { if (typeof fn !== 'function') missing.push(`svc ${NAMES[b]} · ${name}`); return; }
      const w = function (this: any) {
        if (!inFrame) return fn.apply(this, arguments);   // (the driving script's own calls between frames are not the game's)
        calls[b]++;
        if (svc) return fn.apply(this, arguments);
        svc = 1; callsOuter[b]++; const t = now(); acc[cur] += t - last; last = t; const prev = cur; cur = b;
        try { return fn.apply(this, arguments); }
        finally { const t2 = now(); acc[b] += t2 - last; if (b === S_LINE) lineBy[prev] += t2 - last; last = t2; cur = prev; svc = 0; }
      };
      (w as any).__fc = true; proto[name] = w;
    };
    const g = W.game;
    wrap(g.player.char, 'update', S_ANIM);
    wrap(g.col.nav, 'findPath', S_PATH); wrap(g.col.nav, 'walkable', S_PATH); wrap(g.col.nav, 'nearestFreePoint', S_PATH);
    wrap(g.col.sound, 'propagate', S_SOUND);
    wrap(g.col, 'segmentBlocked', S_LINE); wrap(g.col, 'segmentBlockedDynamic', S_LINE); wrap(g.col, 'shotBlocked', S_LINE); wrap(g.col, 'raycast', S_LINE);
    wrap(g.col, 'collideCircle', S_COLLIDE);
    wrap(W.engine.world, 'raycast', S_RAY);
    if (g.destr?.walls) { wrap(g.destr.walls, 'round', S_WALL); wrap(g.destr.walls, 'blast', S_WALL); if (typeof W.engine.reach?.tick === 'function') wrap(W.engine.reach, 'tick', S_REACH); }   // (a build with the wall engine)
    if (GPU) {   // the engine's CPU half, by part (each comes out of 'render: … the rest')
      const e = W.engine;
      wrap(e.world, 'upload', B_UPLOAD); wrap(e.lights, 'upload', R_LIGHTS); wrap(e, 'glassLightMasks', R_LIGHTS);
      if (e.decals) wrap(e.decals, 'frame', R_DECALS);
      if (e.particles) { wrap(e.particles, 'prepare', R_PART); wrap(e.particles, 'encodeUpdate', R_PART); wrap(e.particles, 'encodeDraw', R_PART); }
      if (e.smoke?.encode) wrap(e.smoke, 'encode', R_SMOKE);
      wrap(e.gbuffer, 'encode', R_GB); wrap(e.gbuffer, 'encodeDown', R_GB); wrap(e.gbuffer, 'snapshotPrev', R_GB); if (e.meshShadows) wrap(e.meshShadows, 'encode', R_GB);
      wrap(e.rc, 'encode', R_RC); wrap(e.passes, 'encode', R_ENC);
    }
  },
};
(globalThis as any).__FC = FC;

/** where in `lines[from, to)` the statement that holds `token` begins, or null: the line, the column the statement starts at, and whether a mark can go IN the
 *  line there (`inline`: it stands behind a brace or a `;` on its own line, as the first call of a loop over the seats does, so the mark belongs inside the
 *  loop's body) or has to go on a line of its own in front (the line's own start, or the one statement of a loop or a guard with no brace). It has to be a
 *  place a statement may stand: only statement openers before the token (a loop or a guard and its brace, a `const` that ended in `;`, the thing a `.call(`
 *  is made on, the guard the call is the test of), the line no `else` arm, and the line before it the end of a statement or the opening of a block. A token
 *  inside a comment is not the call, and a line this file wrote itself is not the game's. */
function statementWith(lines: string[], from: number, to: number, token: string): { line: number; col: number; inline: boolean } | null {
  const code = (l: string) => { const c = l.indexOf('//'); return (c < 0 ? l : l.slice(0, c)).trim(); };
  for (let i = from; i < to; i++) {
    const line = lines[i], at = line.indexOf(token); if (at < 0 || line.includes('globalThis.__FC')) continue;
    const note = line.indexOf('//'); if (!token.startsWith('//') && note >= 0 && note < at) continue;
    if (/^(else\b|\} else\b)/.test(line.trim())) continue;
    // (a token that begins at the dot leaves out who the call is made on; a guard's own `if (` may open the statement the call is the test of)
    let lead = token.startsWith('.') ? line.slice(0, at).replace(/[\w$.\[\]]+$/, '') : line.slice(0, at);
    lead = lead.replace(/(^|[;{)]\s*|^\s*)(if|while) \($/, '$1');
    const col = lead.length, L = lead.trim();
    let depth = 0; for (const ch of L) depth += ch === '(' ? 1 : ch === ')' ? -1 : 0;
    const bare = /(^|[;{]\s*)((if|for|while) \(.*\)\s*)+$/.test(L);   // (the one statement of a loop or a guard with no brace)
    if (depth !== 0 || !(L === '' || /[;{]$/.test(L) || bare)) continue;
    let k = i - 1; while (k >= from && code(lines[k]) === '') k--;
    if (k >= from && !/[;{}]$/.test(code(lines[k]))) continue;
    return { line: i, col, inline: /[;{]$/.test(L) };
  }
  return null;
}
/** `mark` put where `at` says: in the line, or on a line of its own in front of it. true = a line was added */
function putMark(lines: string[], at: { line: number; col: number; inline: boolean }, mark: string): boolean {
  if (at.inline) { const l = lines[at.line]; lines[at.line] = `${l.slice(0, at.col)}${l.slice(at.col).startsWith(' ') ? '' : ' '}${mark}${l.slice(at.col)}`; return false; }
  lines.splice(at.line, 0, mark); return true;
}

/** THE STEP AND WHAT IT CALLS: the line ranges [from, to) of Game.step's body (the seats' step, which is where the tick lives) and of every method of the class
 *  it calls by `this.<name>(`, and they call, wherever in the file they stand, each with its method's name. (Until 11-03 only the private methods written
 *  straight after the step were searched, and a part of the frame moved below another method lost its mark.) */
function stepRanges(lines: string[]): { name: string; from: number; to: number }[] | null {
  const head = (name: string) => lines.findIndex(l => new RegExp(`^  (private |protected |public )?${name}\\(.*\\)(: [^{]+)? \\{$`).test(l));
  const bodyOf = (name: string, a: number) => { const e = lines.findIndex((l, i) => i > a && l === '  }'); return { name, from: a + 1, to: e < 0 ? lines.length : e }; };
  const first = head('stepSeats') >= 0 ? 'stepSeats' : 'step', a = head(first); if (a < 0) return null;
  const seen = new Set<number>([a]), out: { name: string; from: number; to: number }[] = [], todo: [string, number][] = [[first, a]];
  while (todo.length) {
    const r = bodyOf(...todo.shift()!); out.push(r);
    for (let i = r.from; i < r.to; i++) for (const m of lines[i].matchAll(/\bthis\.(\w+)\(/g)) { const h = head(m[1]); if (h >= 0 && !seen.has(h)) { seen.add(h); todo.push([m[1], h]); } }
  }
  return out;
}

/** where each mark went, for `framecost.ts --marks`: the token, the method, the game's own line number and the line's first words */
const placed: { token: string; bucket: string; method: string; line: number; text: string; inline: boolean }[] = [];
function instrumentGame(src: string): string {
  const lines = src.split('\n');
  const R = stepRanges(lines);
  if (!R) { missing.push('game.ts: Game.step not found — nothing split'); return src; }
  // every mark is placed on the file AS IT IS, then written from the bottom up, so that no mark's place is read off another's line.
  // A mark of the step's own table belongs to the step's body, in the order the tick runs: one that is found only out of that order is a mark on the wrong
  // call, and is reported rather than written. A mark of the methods' table is looked for in the methods the step calls.
  const put: { at: { line: number; col: number; inline: boolean }; mark: string; order: number }[] = []; let floor = R[0].from, order = 0;
  for (const [token, name] of MARKS) {
    const at = statementWith(lines, floor, R[0].to, token);
    if (!at) { missing.push(statementWith(lines, R[0].from, R[0].to, token) ? `game.ts mark · ${token} (found only ABOVE the mark before it in the table: the step's order changed, or it is another call)` : `game.ts mark · ${token}`); continue; }
    floor = at.line; matched.push(token); put.push({ at, mark: `globalThis.__FC.m(${bucket(name)});`, order: order++ });
    placed.push({ token, bucket: name, method: R[0].name, line: at.line + 1, text: lines[at.line].trim().slice(0, 70), inline: at.inline });
  }
  for (const [token, name] of INNER) {
    let hit: { at: { line: number; col: number; inline: boolean }; r: { name: string } } | null = null;
    for (const r of R.slice(1)) { const at = statementWith(lines, r.from, r.to, token); if (at) { hit = { at, r }; break; } }
    if (!hit) { missing.push(`game.ts mark · ${token}`); continue; }
    matched.push(token); put.push({ at: hit.at, mark: `globalThis.__FC.m(${bucket(name)});`, order: order++ });
    placed.push({ token, bucket: name, method: hit.r.name, line: hit.at.line + 1, text: lines[hit.at.line].trim().slice(0, 70), inline: hit.at.inline });
  }
  // (two marks at one place: the later of the table is written first, so it stands behind the earlier one — the earlier one's bucket then holds nothing, as it should)
  put.sort((a, b) => b.at.line - a.at.line || b.at.col - a.at.col || b.order - a.order);
  for (const p of put) putMark(lines, p.at, p.mark);
  return lines.join('\n');
}

/** the ways a meter's fall back to the CPU light estimate is spelt when the probe has no answer (guards.ts: one site each on 11-03; more of them, and one in
 *  civilians.ts, before the senses had rows of their own): the fallback expression wrapped, nothing else. How many sites there are is the game's to say; a
 *  spelling with NO site is reported: that meter's estimate is then inside the system that asked, and in the frame's total */
const STANDINS: [string, string][] = [
  [': Math.max(lightEstimateAt(g, pc), lightEstimateAt(g, ph));', ': (globalThis as any).__FC.standIn(() => Math.max(lightEstimateAt(g, pc), lightEstimateAt(g, ph)));'],
  [': lightEstimateAt(g, at);', ': (globalThis as any).__FC.standIn(() => lightEstimateAt(g, at));'],
];
const standInMissing = (): string[] => STANDINS.flatMap(([a], i) => standInHits[i] ? [] : [`the probe's CPU stand-in: no site spelt '${a.trim()}' in guards.ts / civilians.ts — its time is inside the system that asked`]);
let standInSites = 0; const standInHits = STANDINS.map(() => 0);
function instrumentStandIns(src: string): string {
  STANDINS.forEach(([a, b], i) => { const n = src.split(a).length - 1; if (n) { standInSites += n; standInHits[i] += n; src = src.replaceAll(a, b); } });
  return src;
}

const harnessPlaced: { token: string; line: number }[] = [];
function instrumentHarness(src: string): string {
  const lines = src.split('\n');
  const a = lines.findIndex(l => l.trimStart().startsWith('step(dt: number, follow: Vec3 | null = null) {'));
  if (a < 0) { missing.push('headless.ts: W.step not found — no frames'); return src; }
  let end = lines.findIndex((l, i) => i > a && l.trim() === '},'); if (end < 0) end = lines.length;
  // (the step's lines in the order they run: begin · the update · camera · lights · the upload · end. Each is placed on the file as it is, at or below the
  //  one before it, and they are written from the bottom up; a line this file wrote is never taken for the harness's own — statementWith)
  const want: [string, string][] = [['game.update(dt, input, cam, canvas);', 'globalThis.__FC.begin();'],
    ...(GPU ? [['cam.update(dt, follow ??', 'if (globalThis.__FCG) { globalThis.__FCG.frame(W, dt, follow); globalThis.__FC.end(); input.endFrame(); return; }']] as [string, string][] : []),   // the page's order from here: smoke → camera → lights → Engine.render
    ['cam.update(dt, follow ??', `globalThis.__FC.m(${B_CAM});`], ['engine.lights.update(dt, game.time);', `globalThis.__FC.m(${B_LIGHTS});`], ['input.endFrame();', 'globalThis.__FC.end();']];
  const put: { at: { line: number; col: number; inline: boolean }; mark: string; order: number }[] = []; let floor = a + 1;
  want.forEach(([token, mark], order) => { const at = statementWith(lines, floor, end, token); if (!at) { missing.push(`headless.ts mark · ${token}`); return; } floor = at.line; put.push({ at, mark, order }); harnessPlaced.push({ token, line: at.line + 1 }); });
  put.sort((x, y) => y.at.line - x.at.line || y.at.col - x.at.col || y.order - x.order);
  for (const p of put) if (putMark(lines, p.at, p.mark)) end++;
  { const i = lines.findIndex((l, k) => k > a && l.trimStart().startsWith('if (frame++ % uploadEvery === 0) engine.world.upload();'));   // (count the frames that did upload: the page uploads every frame, the play-through every 4th — the tables give the cost per upload)
    if (i >= 0) lines[i] = lines[i].replace('engine.world.upload();', `{ globalThis.__FC.up(); globalThis.__FC.m(${B_UPLOAD}); engine.world.upload(); }`); else missing.push('headless.ts: the upload line'); }
  const r = lines.findIndex((l, k) => k > end && l === '  return W;');
  // (which of the harness's two stands this game stood on, as the harness itself says — src/host/profile.ts; a build from before the stands has no such function: none filed)
  if (r >= 0) lines.splice(r, 0, "  (globalThis as any).__FC_STAND = typeof harnessProfile === 'function' ? harnessProfile() : undefined;", "  (W as any).__fcCharMats = await import('../../src/render/charMaterials').catch(() => null);   /* (a build from before the cast has no such module) */", GPU ? '  (globalThis as any).__FCG_W = W; await (globalThis as any).__FCG.ready(W); globalThis.__FC.stood(W);' : '  globalThis.__FC.stood(W);'); else missing.push('headless.ts: return W (no service wraps)');
  if (GPU) {   // standUp takes its engine and its smoke solver from tools/qa/framecost-gpu.ts instead of the stub
    const swap = (head: string, from: string, to: string) => { const i = lines.findIndex(l => l.trimStart().startsWith(head)); if (i >= 0 && lines[i].includes(from)) lines[i] = lines[i].replace(from, to); else missing.push(`headless.ts: ${head}`); };
    swap('const engine: any = stubMod.stubHost();', 'stubMod.stubHost()', '(await (globalThis as any).__FCG.engine())');
    swap('const smoke: any = standMod.nullSmoke();', 'standMod.nullSmoke()', '(await (globalThis as any).__FCG.smoke(engine))');
  }
  return lines.join('\n');
}

plugin({
  name: 'framecost',
  setup(build) {
    build.onLoad({ filter: /[\\/]src[\\/]game[\\/](guards|civilians)\.ts$/ }, async (args: any) => ({ contents: instrumentStandIns(await Bun.file(args.path).text()), loader: 'ts' }));
    build.onLoad({ filter: /[\\/]src[\\/]game[\\/]game\.ts$/ }, async (args: any) => ({ contents: instrumentGame(await Bun.file(args.path).text()), loader: 'ts' }));
    if (GPU) build.onLoad({ filter: /[\\/]src[\\/]engine\.ts$/ }, async (args: any) => ({ contents: (await Bun.file(args.path).text()).replaceAll('window.devicePixelRatio', '(globalThis as any).__FC_DPR'), loader: 'ts' }));   // (the one DOM read in the renderer)
    build.onLoad({ filter: /[\\/]tools[\\/]qa[\\/]headless\.ts$/ }, async (args: any) => ({ contents: instrumentHarness(await Bun.file(args.path).text()), loader: 'ts' }));
  },
});

if (GPU) await import('./framecost-gpu.ts');

process.on('exit', () => {
  const out = process.env.FC_OUT; if (!out) return;
  for (const m of standInMissing()) missing.push(m);
  const nb = NAMES.length, mat = new Float32Array(frames.length * (nb + 1));
  frames.forEach((r, i) => mat.set(r.length === nb + 1 ? r : Float32Array.from({ length: nb + 1 }, (_, k) => k < r.length - 1 ? r[k] : k === nb ? r[r.length - 1] : 0), i * (nb + 1)));
  try {
    writeFileSync(out, JSON.stringify({ v: 1, runtime: `bun ${Bun.version}`, stand: (globalThis as any).__FC_STAND, deep: process.env.FC_DEEP !== '0', names: NAMES, firstStep: B_FIRST_STEP, firstSvc: B_FIRST_SVC, upload: B_UPLOAD,
      frames: frames.length, uploads, missing, marks: matched.length, calls: Array.from(calls.subarray(0, nb)), callsOuter: Array.from(callsOuter.subarray(0, nb)), lineBy: Array.from(lineBy.subarray(0, nb)),
      gpu: GPU ? (globalThis as any).__FCG?.report() : undefined,
      ms: Buffer.from(mat.buffer).toString('base64') }));
  } catch (e) { console.error(`framecost: could not write ${out} — ${e}`); }
});
