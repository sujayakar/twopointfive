// The 'checkpoints' family (run through probes.ts: `bun run tools/qa/probes.ts checkpoints` — also in 'all'): the encounter serialised and put back
// (src/game/savestate.ts saveState / loadState, the autosave controller Game.saves). What is held:
//   checkpoint-identity  (a) a rich mid-mission state — a man choked out and hauled, a door kicked in, a key in Sam's pocket, the mains thrown, a beam tripped,
//                            marks on the walls, the drive out, a chair shoved, a torch on the floor — is saved; then THREE ten-second futures are digested once a
//                            second over guards / Sam / doors / mission / chronicle / lights / devices / items with the seeded Math.random re-seeded to one
//                            value at their start: A = the game simply stepped on after the save, B = the SAME game after loadState(snapshot), C = a FRESH game
//                            after loadState(snapshot). A ≡ B ≡ C (the file holds everything the next frames hang on), and save(load(s)) ≡ s slice by slice;
//   checkpoint-turn      (a2) a man caught in the MIDDLE OF A TURN (character.ts TURN, the turn weight: on in the page, raised here on either stand by
//                            TPS_TURN=1) is filed with it under SAVETIMELINE.turn (GuardSnap.turn, character.ts turnSnap; TPS_SAVETURN=1, raised here
//                            too) and goes on turning after a load: every man's place and yaws agree frame for frame over the first half second, A ≡ B ≡ C
//                            over three seconds, and the file made from a fresh game after a load holds the same turn. Twice: as his own update left him,
//                            and with a wish written AFTER his update on the frame of the save (two men turning to talk do that), which the loader's bake
//                            must hand back. With the switch off no file says a turn and one that has it loads as the keyless file. CHECKPOINT_TURN_BREAK=1 = the key taken out of the file before it is
//                            loaded, as a file from before the key: he starts from rest, and the futures must part (lines 1 and 2 fail);
//   checkpoint-autosave  (b) every autosave trigger fires exactly once in a scripted run — a room entered, the stage advancing, a body hauled away, 90 s of
//                            quiet — and NOTHING is written while compromised: a man alert / searching / over the awareness line, Sam mid-pick, a can on its
//                            fuse (the request waits, then lands or lapses); the slots rotate through three; the HUD tag and the chronicle row appear;
//   checkpoint-death     (c) Sam shot dead after a checkpoint: the newest slot loads, he stands where it was written with the run's chronicle intact (+ the
//                            'loaded' row, the score's load count), no double guards / leaked items / stale cursor, and the run walks on to the fire exit;
//   checkpoint-slots     (d) three autosaves + a manual fill auto.0‥2 + manual and the fourth autosave overwrites auto.0; export → import → load round-trips
//                            (the imported file lands in the manual slot and loads to the same digest); a file of another 'level' / version is refused;
//   checkpoint-refuse    (e) refused with the reason: the tour flag up, daylight in, the QRF requested, a unit on the floor, Sam down, a man scripted / the
//                            pair clearing (plain), Sam holding a man (plain) — and `force` files the last two (the hold let go of in the FILE, not the game).
import { standUp, ROOT } from './headless.ts';
const { v3, wrapAngle } = await import(`${ROOT}/src/math/vec.ts`);
const SS = await import(`${ROOT}/src/game/savestate.ts`);
const { saveState, loadState, canSave, canLoad, CKPT, levelFingerprint } = SS;
const ringSize = (): number => SS.SAVETIMELINE.on ? SS.SAVETIMELINE.auto : CKPT.autoSlots;   // how many autosaves the stand keeps
const { killGuard, fireWeapon, hitPlayer } = await import(`${ROOT}/src/game/combat.ts`);
const { startGrab, startKick, startPicking, toggleDrag } = await import(`${ROOT}/src/game/player.ts`);
const { spawnUnit, despawnUnit, rollcallState } = await import(`${ROOT}/src/game/guards.ts`);
const { ambientState, hashStr } = await import(`${ROOT}/src/game/dialogue.ts`);
const { HOLD, TURN } = await import(`${ROOT}/src/game/character.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
export function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const steps = (W: any, secs: number, until?: () => boolean) => { const n = Math.round(secs / DT); for (let i = 0; i < n; i++) { W.step(DT); if (until?.()) return true; } return !until; };
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
const verdictLine = (ok: boolean, label: string, detail: string, fails: string[]) => console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
let familyFails = 0;
export function checkpointFailures(): number { return familyFails; }

/** ONE sample of everything the identity claim is over: guards / Sam / doors / mission / chronicle / lights / devices / items / the floor's memory. Positions to
 *  the millimetre, clocks to the hundredth. The soak's --checkpoint-at reads it too. Lives in src/game/digest.ts now (the session journal digests a recording with
 *  the same function); this name stays for the probes that import it from here. */
export const digestSample: (game: any) => any = (await import(`${ROOT}/src/game/digest.ts`)).digestGame;
/** Step `secs`, sampling once a second: the samples and one hash over them all. Math.random is re-seeded to `seed` FIRST (the identity claim's terms). */
export function digestRun(W: any, secs: number, seed: number): { hash: string; samples: any[] } {
  Math.random = seeded(seed);
  const samples: any[] = [digestSample(W.game)];
  for (let s = 0; s < secs; s++) { steps(W, 1); samples.push(digestSample(W.game)); }
  const hash = hashStr(JSON.stringify(samples)).toString(16);
  return { hash, samples };
}
/** the first difference between two sample lists, in words ('' = none) */
function firstDiff(a: any[], b: any[]): string {
  for (let i = 0; i < Math.max(a.length, b.length); i++) {
    const x = JSON.stringify(a[i]), y = JSON.stringify(b[i]); if (x === y) continue;
    // narrow to the key
    const A = a[i] ?? {}, B = b[i] ?? {};
    for (const k of new Set([...Object.keys(A), ...Object.keys(B)])) {
      const xa = JSON.stringify(A[k]), xb = JSON.stringify(B[k]); if (xa === xb) continue;
      if (k === 'guards' && Array.isArray(A[k]) && Array.isArray(B[k])) { for (let j = 0; j < A[k].length; j++) { const ga = JSON.stringify(A[k][j]), gb = JSON.stringify(B[k][j]); if (ga !== gb) return `sample ${i} (t ${A.t}) guard ${j}: ${ga.slice(0, 300)} ≠ ${gb.slice(0, 300)}`; } }
      return `sample ${i} (t ${A.t}) '${k}': ${String(xa).slice(0, 300)} ≠ ${String(xb).slice(0, 300)}`;
    }
    return `sample ${i} differs (unnarrowed)`;
  }
  return '';
}
/** slice-by-slice comparison of two SaveStates, ignoring what a save legitimately stamps afresh (wall time, the appended checkpoint rows / counters, the label) */
function stateDiff(a: any, b: any): string[] {
  
  const strip = (s: any) => { const c = JSON.parse(JSON.stringify(s)); delete c.savedAt; delete c.label; delete c.kind; delete c.messages; c.chronicle.events = c.chronicle.events.filter((e: any) => e.kind !== 'checkpoint'); c.chronicle.counts.saves = 0; c.chronicle.counts.loads = 0; c.autosave.lastSaveT = 0;
    for (const gd of c.guards) if (gd.state === 'dead') { delete gd.pos; delete gd.lastPos; delete gd.beamDir; delete gd.rag; }   // (a body is laid down again by Ragdoll.settleAt, its trunk snapped rigid: its geometry is compared apart, to a tolerance — bodyDrift below)
    return c; };
  const A = strip(a), B = strip(b); const out: string[] = [];
  for (const k of new Set([...Object.keys(A), ...Object.keys(B)])) { const x = JSON.stringify(A[k]), y = JSON.stringify(B[k]); if (x !== y) out.push(`${k}: ${firstJsonDiff(A[k], B[k], k)}`); }
  return out;
}
/** the largest distance any particle of any body moved between two files (m) — the settle's trunk snap, a centimetre or two by design */
function bodyDrift(a: any, b: any): number { let m = 0; a.guards.forEach((g0: any, i: number) => { const g1 = b.guards[i]; if (g0.state !== 'dead' || !g0.rag || !g1?.rag) return; for (let k = 0; k < g0.rag.x.length; k += 3) m = Math.max(m, Math.hypot(g0.rag.x[k] - g1.rag.x[k], g0.rag.x[k + 1] - g1.rag.x[k + 1], g0.rag.x[k + 2] - g1.rag.x[k + 2])); m = Math.max(m, Math.hypot(g0.pos[0] - g1.pos[0], g0.pos[2] - g1.pos[2])); }); return m; }
function firstJsonDiff(a: any, b: any, path: string): string {
  if (typeof a !== 'object' || typeof b !== 'object' || a === null || b === null) return `${path} ${JSON.stringify(a)?.slice(0, 120)} ≠ ${JSON.stringify(b)?.slice(0, 120)}`;
  const keys = Array.isArray(a) ? [...Array(Math.max(a.length, b.length)).keys()] : [...new Set([...Object.keys(a), ...Object.keys(b)])];
  for (const k of keys) { if (JSON.stringify(a[k as any]) !== JSON.stringify(b[k as any])) return firstJsonDiff(a[k as any], b[k as any], `${path}.${String(k)}`); }
  return `${path} (order?)`;
}

/** The rich mid-mission state every case starts from: seeded, staged step by step through the game's own verbs, then left CALM (Sam dark and away, the men on
 *  patrol) so a checkpoint is allowed. Returns the standing game and notes on what was staged. */
export async function stageRich(seed = 4471): Promise<{ W: any; notes: string[] }> {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W; const notes: string[] = [];
  game.player.visibility = 0.02; game.godMode = true; game.saves.enabled = false;   // (no autosaves while staging: the cases write their own)
  steps(W, 0.2);
  const [A, B, C] = game.guards;   // Kowalski (corridor), Novak (cubicles), Reyes (lobby / break room)
  rollcallState(game).nextAt = game.time + 900;   // (the radio check kept out of the ten-second windows: it asks after the choked man on its own clock otherwise — legitimate, but a coin-flip on WHICH window it lands in is not what (a) is about)
  // 1. Kowalski grabbed from behind in the corridor, questioned to his key, choked out; the body hauled two metres and let go
  place(A, [16, 0, 11.2], Math.PI / 2); A.heardUpTo = game.eventSeq;
  game.teleportPlayer([15.2, 0, 11.2]); game.player.crouch = true; steps(W, DT); game.player.visibility = 0.02;
  startGrab(game, A);
  steps(W, 3, () => game.player.holding?.phase === 'held'); game.player.fHeld = false; steps(W, DT);
  let gave: string | null = null; for (let i = 0; i < 8 && gave !== 'door'; i++) { const p = game.interrogate(A, seeded(60 + i)); gave = p?.kind ?? null; steps(W, 0.5); }
  notes.push(`questioned ${A.callsign}: ${gave === 'door' ? 'gave the door + key' : 'no key (' + gave + ')'}; keys [${[...game.player.keys].join(',')}]`);
  if (game.player.holding) { const { endFor } = await import('./verbs.ts'); endFor(W, HOLD.secs.chokeHold + HOLD.secs.choke + 0.3); steps(W, 0.8); }
  notes.push(`${A.callsign} ${A.state}/${A.downKind}`);
  steps(W, 2.5);   // (let him settle on the floor)
  toggleDrag(game, A); const from = v3.copy(game.player.char.pos);
  for (let i = 0; i < 150 && game.player.dragging; i++) { W.input.keys.add('KeyA'); W.step(DT); } W.input.keys.delete('KeyA'); steps(W, DT);
  if (game.player.dragging) toggleDrag(game); steps(W, 1.5);
  notes.push(`hauled ${v3.distXZ(from, game.player.char.pos).toFixed(1)} m, dropped; torch item ${A.dropped ? A.dropped.kind : 'none'}`);
  // 2. the storage door kicked in (the parked pair deaf to it), marks on the leaf; a round into the corridor wall (a bullet hole)
  B.heardUpTo = 1e12; C.heardUpTo = 1e12;
  const storage = game.doors.byName('storage');
  game.teleportPlayer([32.6, 0, 10.9]); game.player.crouch = false; game.player.char.bodyYaw = Math.PI; steps(W, 2 * DT);
  startKick(game, storage); steps(W, 2.0, () => storage.lockBroken); steps(W, 1.0);
  game.player.slot = 'pistol'; game.player.holstered = false; steps(W, 0.3);
  fireWeapon(game, game.player.char, [30, 1.2, 10.05], true); steps(W, 0.5);
  game.player.holstered = true; steps(W, 0.2);
  notes.push(`storage lockBroken ${storage.lockBroken}, decals ${game.decals.list.length}`);
  // 3. the mains thrown by hand (permanent till reset), a chair shoved in the break room, the corridor beam walked through (a trip on the log)
  game.setBlackout(86400, 'hand'); steps(W, 0.5);
  for (const gd of game.guards) if (gd.task?.kind === 'breaker') { gd.task = null; gd.state = 'patrol'; gd.awareness = 0; gd.lastKnown = null; }   // (nobody walks to the breaker inside the windows: the errand is its own probe's)
  { const chair = game.props.props.find((p: any) => p.def.kind === 'chair'); if (chair) { game.props.applyImpulse(chair, [chair.x, 0.4, chair.z], [40, 0, 15], 1); } steps(W, 1.5); notes.push(`chair ${chair ? `${chair.id} moved ${Math.hypot(chair.x - chair.home[0], chair.z - chair.home[1]).toFixed(2)} m` : 'none'}`); }
  { const L = game.devices?.lasers?.[0]; if (L) { const mid: Vec3 = [(L.def.from[0] + L.def.to[0]) / 2, 0, (L.def.from[2] + L.def.to[2]) / 2]; game.teleportPlayer([mid[0] - 1.2, 0, mid[2]]); for (let i = 0; i < 90; i++) { W.input.keys.add('KeyD'); W.step(DT); } W.input.keys.delete('KeyD'); steps(W, 0.5); notes.push(`beam ${L.def.name} trips ${L.trips}`); } }
  // 4. the mission on: into the server room (stage → drive), the drive pulled by hand (stage → exfil, the rack dark, the clunk in the queue)
  { const M = game.level.mission; const R = M.serverRoom; game.teleportPlayer([(R.x0 + R.x1) / 2, 0, (R.z0 + R.z1) / 2]); steps(W, 0.3);
    game.teleportPlayer([M.rack.front[0], 0, M.rack.front[2] + 0.7]); steps(W, 0.2); const { drivePulled } = await import(`${ROOT}/src/game/mission.ts`); if (game.mission.stage === 'drive') drivePulled(game); steps(W, 0.3);
    game.mission.sweepAt = -1;   // (the rack check would send a man in mid-window — again its own probe's business)
    notes.push(`mission ${game.mission.stage}, rack pulled ${game.mission.pulled}`); }
  // 5. calm again: everybody deaf no more, on patrol, Sam dark in the cubicles' SW corner; a couple of seconds to settle
  for (const gd of game.guards) { if (gd.state === 'dead') continue; gd.heardUpTo = game.eventSeq; gd.awareness = 0; gd.state = 'patrol'; gd.lastKnown = null; gd.task = null; gd.hold = false; }
  game.events.length = 0;
  if (game.clearing) { game.clearing.cancel(); game.clearing = null; for (const gd of game.guards) if (gd.state !== 'dead') { gd.state = 'patrol'; gd.awareness = 0; gd.lastKnown = null; } }   // (a pair the staging's alarm had clearing rooms is let go of like every other errand above: which second the body was found in is the level's walls' business, not this state's)
  game.teleportPlayer([16.5, 0, 22.5]); game.player.crouch = true; steps(W, DT); game.player.visibility = 0.02;
  if (game.clearing) { game.clearing.cancel(); game.clearing = null; for (const gd of game.guards) if (gd.state !== 'dead') { gd.state = 'patrol'; gd.awareness = 0; gd.lastKnown = null; } }   // (and once more a frame on: the men put to patrol above END the alarm's episode on the next frame, and an episode that came to nothing deals the pair its rooms afresh — guards.ts updateEscalation — unless the floor was raised inside the last second and a half, which is which second a civilian rang it in)
  steps(W, 2.0);
  notes.push(`t ${game.time.toFixed(1)} s: ${game.guards.map((x: any) => `${x.callsign} ${x.state} ${x.awareness.toFixed(2)}`).join(', ')}; chronicle ${game.chronicle.events.length} rows; canSave: ${canSave(game) ?? 'yes'}`);
  return { W, notes };
}

/** the three futures of one file, each run the same way: A the game that simply goes on, B the same game with the file loaded into it, C a fresh game with it loaded */
async function futures<T>(W: any, file: () => any, run: (W: any) => T): Promise<{ A: T; B: T; C: T; W2: any }> {
  const A = run(W);
  loadState(W.game, file()); W.cam.target = v3.copy(W.game.player.char.pos);
  const B = run(W);
  Math.random = seeded(777);   // (whatever the fresh boot draws)
  const W2 = await standUp(); loadState(W2.game, file()); W2.cam.target = v3.copy(W2.game.player.char.pos);
  const C = run(W2);
  return { A, B, C, W2 };
}

// ================================================================================================================================ (a) identity
export async function checkpointIdentity() {
  console.log('\n=== checkpoint-identity: a rich mid-mission state saved; A (stepped on) ≡ B (same game, loaded) ≡ C (fresh game, loaded) over ten seconds digested once a second under one seed; save(load(s)) ≡ s ===');
  const fails: string[] = []; const notes: string[] = [];
  const { W, notes: staged } = await stageRich(); const { game } = W; notes.push(...staged);
  // no conversation under way at the save (one is cut short by a load, by design — documented; the identity is claimed outside it)
  steps(W, 12, () => !ambientState(game).run); if (ambientState(game).run) fails.push('an ambient conversation would not end inside 12 s');
  const why = canSave(game); if (why) { fails.push(`cannot save the staged state: ${why}`); verdictLine(false, '(a) identity', notes.join('\n    '), fails); familyFails++; return; }
  const s = saveState(game, { label: 'identity', kind: 'harness' }); const json = JSON.stringify(s);
  notes.push(`saved ${(json.length / 1024).toFixed(1)} KB · ${s.guards.filter((x: any) => x.state === 'dead').length} body · ${s.items.length} items · ${s.decals.length} decals · ${s.chronicle.events.length} rows · blackout ${s.blackout.active} · stage ${s.stage}`);
  const SEED = 90210, SECS = 10;
  const { A, B, C: Cd, W2 } = await futures(W, () => JSON.parse(json), (w) => digestRun(w, SECS, SEED));
  notes.push(`digests: A ${A.hash} · B ${B.hash} · C ${Cd.hash}`);
  if (A.hash !== B.hash) fails.push(`A ≠ B — ${firstDiff(A.samples, B.samples)}`);
  if (B.hash !== Cd.hash) fails.push(`B ≠ C — ${firstDiff(B.samples, Cd.samples)}`);
  // save(load(s)) ≡ s: the fresh game's own snapshot straight after another load of the same file
  { Math.random = seeded(1); const W3 = await standUp(); loadState(W3.game, JSON.parse(json)); const s2 = saveState(W3.game, { label: 'identity', kind: 'harness' }); const d = stateDiff(s, s2); const drift = bodyDrift(s, s2); if (d.length) fails.push(`save(load(s)) ≠ s in ${d.length} slice(s): ${d.slice(0, 4).join(' · ')}`); else if (drift > 0.03) fails.push(`the body's particles drifted ${drift.toFixed(3)} m through load→save`); else notes.push(`save(load(s)) ≡ s in every slice (the body re-laid within ${(drift * 100).toFixed(1)} cm)`); }
  // the body came back settled where it lay, the torch item with it, the marks, the chair, the blackout, the key
  { const g2 = W2.game; const dead = g2.guards.find((x: any) => x.state === 'dead'); const d0 = s.guards.find((x: any) => x.state === 'dead');
    if (!dead || !d0) fails.push('no body in the loaded game'); else { const dx = Math.hypot(dead.char.ragdoll.x[0] - d0.rag.x[0], dead.char.ragdoll.x[2] - d0.rag.x[2]); if (dx > 0.03 || !dead.char.ragdoll.sleeping) fails.push(`body pelvis ${dx.toFixed(3)} m off the file / sleeping ${dead.char.ragdoll.sleeping}`); if (!dead.dropped) fails.push('the torch item link is gone'); }
    if (g2.decals.list.length !== s.decals.length) fails.push(`decals ${g2.decals.list.length} ≠ ${s.decals.length}`);
    if (!g2.blackout.active) fails.push('blackout not restored'); if (!g2.player.keys.size && s.player.keys.length) fails.push('key not restored'); if (!g2.doors.byName('storage').lockBroken) fails.push('kicked lock not restored'); if (g2.mission.stage !== 'exfil' || !g2.mission.pulled) fails.push(`mission ${g2.mission.stage} pulled ${g2.mission.pulled}`); }
  const ok = !fails.length; if (!ok) familyFails++;
  verdictLine(ok, '(a) the staged state (body hauled, door kicked, key taken, mains thrown, beam tripped, marks, chair, drive out) saves; A ≡ B ≡ C over 10 s at one seed; save∘load is the identity on the file; the body / marks / blackout / key / lock / stage are back in a fresh game', notes.join('\n    '), fails);
}

// ================================================================================================================================ (a2) a man caught mid-turn
export async function checkpointTurn() {
  console.log('\n=== checkpoint-turn: a man in the middle of a turn at the save is filed with it and goes on turning after a load (the turn weight and the save\'s own switch raised: TPS_TURN=1 TPS_SAVETURN=1) ===');
  const { SAVETIMELINE } = SS; const BREAK = process.env.CHECKPOINT_TURN_BREAK === '1'; const env = { turn: process.env.TPS_TURN, save: process.env.TPS_SAVETURN }, was = { turn: TURN.on, save: SAVETIMELINE.turn };
  process.env.TPS_TURN = '1'; process.env.TPS_SAVETURN = '1';
  const lines: { ok: boolean; what: string; notes: string[]; fails: string[] }[] = [];
  try {
    Math.random = seeded(4242); let W = await standUp();
    // the patrols walk until a living man's weight is carrying him round (speed in his feet, his AI's wish well ahead of where he has got to) on a frame that can be saved
    const mid = (g: any) => g.guards.filter((gd: any) => { const t = gd.state !== 'dead' && gd.char.turnSnap(); return !!t && Math.abs(t[2]) > 1 && Math.abs(wrapAngle(t[4] - t[0])) > 0.3; });
    const turns = (f: any) => JSON.stringify(f.guards.map((x: any) => [x.turn ?? null, x.bodyYaw, x.aimYaw]));
    // every man's place and yaws on each of the first frames, to the bit (a turn that starts from rest parts from the run that went on in its first frame, by a
    // fraction of a degree — the once-a-second digest after it would see it only as the millimetres his feet end up off), then the family's digest
    const FRAMES = 30, SEED = 31337, SECS = 3;
    const run = (w: any) => { Math.random = seeded(SEED); const f: string[] = []; for (let i = 0; i < FRAMES; i++) { w.step(DT); f.push(JSON.stringify(w.game.guards.map((gd: any) => [gd.callsign, gd.char.pos[0], gd.char.pos[2], gd.char.bodyYaw, gd.char.aimYaw]))); } return { f, d: digestRun(w, SECS, SEED) }; };
    const part = (a: string[], b: string[]) => { const i = a.findIndex((x, k) => x !== b[k]); if (i < 0) return ''; const x = JSON.parse(a[i]), y = JSON.parse(b[i]); const m = x.find((r: any, k: number) => JSON.stringify(r) !== JSON.stringify(y[k])); return `frame ${i + 1}${m ? `, ${m[0]}` : ''}`; };
    const one = async (what: string, wish: boolean) => {
      const fails: string[] = [], notes: string[] = []; const { game } = W;
      const hit = steps(W, 90, () => mid(game).length > 0 && !canSave(game) && !ambientState(game).run);
      if (TURN.on <= 0.5 || !SAVETIMELINE.turn) fails.push(`the switches are not up under TPS_TURN=1 TPS_SAVETURN=1 (TURN.on ${TURN.on}, SAVETIMELINE.turn ${SAVETIMELINE.turn})`);
      if (!hit) { fails.push('no man was caught mid-turn on a frame that can be saved inside 90 s'); lines.push({ ok: false, what, notes, fails }); return; }
      const men = mid(game), who = men.map((gd: any) => gd.callsign);
      if (wish) { const c = men[0].char; c.bodyYaw = wrapAngle(c.bodyYaw + 0.4); notes.push(`${who[0]}'s body yaw written 0.4 rad on after his own update, as a man turning to talk has it written`); }
      const s = saveState(game, { label: 'turn', kind: 'harness' }); const json = JSON.stringify(s);
      const filed = s.guards.filter((x: any) => Array.isArray(x.turn)); const t0 = filed.find((x: any) => x.callsign === who[0])?.turn as number[] | undefined;
      notes.push(`t ${game.time.toFixed(2)}: ${who.join(', ')} mid-turn (${t0 ? `${Math.abs(wrapAngle(t0[4] - t0[0])).toFixed(2)} rad still to come round, ${Math.abs(t0[2]).toFixed(1)} rad/s in his feet` : 'NOT in the file'}); ${filed.length} of ${s.guards.length} men filed with a turn`);
      if (!t0 || t0.length !== 9) fails.push('the man mid-turn has no turn in the file');
      if (wish && t0 && s.guards.find((x: any) => x.callsign === who[0]).bodyYaw === t0[0]) fails.push('the staged wish is not in the file');
      const file = () => { const f = JSON.parse(json); if (BREAK) for (const x of f.guards) delete x.turn; return f; };   // (the break: a file from before the key)
      const { A, B, C, W2 } = await futures(W, file, run);
      const ab = part(A.f, B.f), bc = part(B.f, C.f);
      notes.push(`the first ${FRAMES} frames: A against B ${ab ? `part at ${ab}` : 'the same'}, B against C ${bc ? `part at ${bc}` : 'the same'} · digests over the ${SECS} s after: A ${A.d.hash} · B ${B.d.hash} · C ${C.d.hash}${BREAK ? ' (CHECKPOINT_TURN_BREAK=1: the key taken out of the file — the futures must part)' : ''}`);
      if (ab) fails.push(`A ≠ B from ${ab}`); else if (A.d.hash !== B.d.hash) fails.push(`A ≠ B — ${firstDiff(A.d.samples, B.d.samples)}`);
      if (bc) fails.push(`B ≠ C from ${bc}`); else if (B.d.hash !== C.d.hash) fails.push(`B ≠ C — ${firstDiff(B.d.samples, C.d.samples)}`);
      if (!BREAK) { Math.random = seeded(1); const W3 = await standUp(); loadState(W3.game, file()); const s2 = saveState(W3.game, { label: 'turn', kind: 'harness' }); if (turns(s) !== turns(s2)) fails.push('save(load(s)) does not hold the same turns and yaws'); }   // (under the break the file has no turn to hold: the futures above are what must part)
      lines.push({ ok: !fails.length, what, notes, fails }); W = W2;   // (the next case walks on in the fresh game)
    };
    await one('(a2.1) a man caught mid-turn is filed with his turn; every man\'s place and yaws agree frame for frame over the first half second and A ≡ B ≡ C over the 3 s after; the file made after a load holds the same turns', false);
    await one('(a2.2) … and with a wish written after his own update on the frame of the save: the loader poses him where his weight had got him and hands the wish back', true);
    // the switch off: no file says a turn (the file as it was), and a file that has one is loaded as the same file without the key
    { const fails: string[] = [], notes: string[] = []; const { game } = W; const hit = steps(W, 90, () => mid(game).length > 0 && !canSave(game) && !ambientState(game).run);
      if (!hit) fails.push('no man was caught mid-turn on a frame that can be saved inside 90 s'); else {
        const on = saveState(game, { label: 'turn', kind: 'harness' }), json = JSON.stringify(on); SAVETIMELINE.turn = false; const off = saveState(game, { label: 'turn', kind: 'harness' });
        const nOn = on.guards.filter((x: any) => 'turn' in x).length, nOff = off.guards.filter((x: any) => 'turn' in x).length; const bare = JSON.parse(json); for (const x of bare.guards) delete x.turn;
        const same = JSON.stringify(bare.guards) === JSON.stringify(off.guards);
        notes.push(`${nOn} men filed with a turn with the switch on, ${nOff} with it off; the guards' slice bar the key ${same ? 'the same' : 'DIFFERENT'}`);
        if (!nOn || nOff || !same) fails.push('SAVETIMELINE.turn off does not write the file as it was');
        const load = (f: any) => { loadState(game, f); W.cam.target = v3.copy(game.player.char.pos); return run(W); };
        const keyed = load(JSON.parse(json)), plain = load(JSON.parse(JSON.stringify(bare))); SAVETIMELINE.turn = true; const put = load(JSON.parse(json));
        const kp = part(keyed.f, plain.f) || (keyed.d.hash !== plain.d.hash ? 'the digest after' : ''), pp = part(put.f, plain.f);
        notes.push(`the switch off, the file with its turns against the same file with the key taken out: ${kp ? `part at ${kp}` : `the same over ${FRAMES} frames and the ${SECS} s after`}; the switch on again, the same two ${pp ? `part at ${pp}` : 'do not part'}`);
        if (kp) fails.push(`with the switch off a file's turn is still put back (parts from the keyless file at ${kp})`);
        if (!pp) fails.push('with the switch on the file\'s turn made no difference: this line proves nothing'); }
       }
  } finally { for (const [k, v] of [['TPS_TURN', env.turn], ['TPS_SAVETURN', env.save]] as const) { if (v === undefined) delete process.env[k]; else process.env[k] = v; } TURN.on = was.turn; SAVETIMELINE.turn = was.save; }
  for (const L of lines) { if (!L.ok) familyFails++; verdictLine(L.ok, L.what, L.notes.join('\n    '), L.fails); }
}

// ================================================================================================================================ (b) autosave triggers + gates
export async function checkpointAutosave() {
  console.log('\n=== checkpoint-autosave: each trigger fires once (room / stage / body / quiet), nothing is written while compromised (alert man, awareness over the line, mid-pick, a live can), the request waits then lands or lapses; slots rotate; the tag + the row appear ===');
  const fails: string[] = []; const notes: string[] = [];
  Math.random = seeded(515);
  const W = await standUp(); const { game } = W; const S = game.saves; const store = S.store;
  game.player.visibility = 0.02; game.godMode = true; game.playerInvisible = true;   // (unseen, unheard: this is about the controller, not the senses)
  const written: any[] = []; S.onSaved = (s: any, info: any) => written.push({ label: s?.label ?? info?.label, key: info?.key, t: game.time, why: S.log[S.log.length - 1]?.what });
  game.guards.forEach((gd: any, i: number) => { gd.heardUpTo = 1e12; place(gd, [8 + i * 3, 0, 1.6], Math.PI); gd.hold = true; gd.flashlight.enabled = false; });   // parked on the north verge, held
  // (THE CIVILIANS ARE OUT OF THIS SCRIPT, as the staff's senses are: a civilian on the phone to security rightly holds the gate (review G3.4), which is
  // the senses' business and not the controller's. They were only made deaf, for the stun can. On the page's stand the cleaner SEES the body hauled below,
  // runs for the reception phone and is on it until 17 s later: the body's request waits on her, the stage's and the room's JOIN it with no line of their
  // own in the log ('body+stage+room', which this probe read as no stage request at all), and the three land as one save 8 s short of lapsing.
  // CKPTCIVIN_BREAK=1 = the red path: the civilians are left in, deaf, as they were. It bites where the cleaner's round passes the break room, the page's stand.)
  if (process.env.CKPTCIVIN_BREAK === '1') { for (const cv of game.civilians ?? []) cv.heardUpTo = 1e12; } else if (game.civilians) { game.tune.civilians = false; game.civilians.length = 0; }
  rollcallState(game).nextAt = 1e9;
  steps(W, 0.5);
  const req0 = S.stats.requests;
  // ROOM: teleport into the conference room (a region he has not been in — he starts in the lobby) → a request 'room', written after the debounce
  game.teleportPlayer([8, 0, 8]); steps(W, DT);
  if (S.stats.requests !== req0 + 1 || !/room/.test(S.pending?.why ?? '')) fails.push(`entering the conference room raised ${S.stats.requests - req0} request(s) (pending ${S.pending?.why}; log ${S.log.slice(-2).map((l: any) => l.what).join(' | ')})`);
  steps(W, CKPT.debounceSecs + 0.5);
  if (written.length !== 1) fails.push(`the room request wrote ${written.length} save(s) after the debounce (gate ${S.gate(game)}; log ${S.log.slice(-2).map((l: any) => l.what).join(' | ')})`); else notes.push(`room → ${written[0].key} at ${written[0].t.toFixed(1)} s`);
  if (!written.length) { verdictLine(false, '(b) autosave', notes.join('\n    '), fails); familyFails++; return; }
  // the same room again: nothing; another new room inside minGap: requested, deferred past the gap, then written
  game.teleportPlayer([9.5, 0, 8.5]); steps(W, 0.5); if (S.pending) fails.push('re-entering the conference room raised a request');
  game.teleportPlayer([20, 0, 11]); steps(W, DT); const tReq = game.time;   // the corridor
  if (!S.pending) fails.push('the corridor raised no request');
  steps(W, CKPT.debounceSecs + 0.5); if (written.length !== 1) fails.push(`a save landed ${(game.time - written[0].t).toFixed(1)} s after the last (min gap ${CKPT.minGapSecs})`);
  steps(W, CKPT.minGapSecs); if (written.length !== 2) fails.push(`the deferred corridor request never landed (${written.length} saves, pending ${S.pending?.why ?? 'none'})`); else notes.push(`corridor deferred ${(written[1].t - tReq).toFixed(1)} s by the gap → ${written[1].key}`);
  // COMPROMISED: a man alert → the cubicles entered raises a request that must NOT land while he is alert; calm him under pendingSecs → it lands
  const [A, B] = game.guards;
  steps(W, CKPT.minGapSecs + 1);
  A.hold = false; A.state = 'alert'; A.awareness = 1; A.lastKnown = [30, 0, 6]; A.lastSeenT = game.time - 30; A.pinned = true;   // (pinned: alert in place, going nowhere)
  game.teleportPlayer([20, 0, 16]); steps(W, DT); if (!S.pending) fails.push('the cubicles raised no request');
  const nBefore = written.length; steps(W, 8); if (written.length !== nBefore) fails.push(`a save was WRITTEN with ${A.callsign} alert`); if (!/alert|search/.test(S.gate(game) ?? '')) fails.push(`gate says '${S.gate(game)}' with a man alert`);
  A.state = 'patrol'; A.awareness = 0; A.lastKnown = null; A.pinned = false; A.hold = true; A.path = []; game.alarm.episode = false; game.escalation = 0; game.escalationT = -1; steps(W, 1.0);
  if (written.length !== nBefore + 1) fails.push(`the cubicles request did not land once he calmed (${written.length - nBefore}, pending ${S.pending?.why ?? 'none'}, gate ${S.gate(game)})`); else notes.push('alert man: held off, landed on calm');
  // awareness over the line (suspicious 0.5) blocks; 0.35 does not
  steps(W, CKPT.minGapSecs + 1);
  const pinB = (aw: number, secs: number) => { const n = Math.round(secs / DT); for (let i = 0; i < n; i++) { B.awareness = aw; B.state = 'suspicious'; B.lastKnown = [22, 0, 18]; B.reactT = 1; W.step(DT); } };   // (his suspicion held at that level frame by frame: left alone it decays under the line inside the debounce, which is the gate working, not failing)
  S.request(game, 'probe', 'over the line'); pinB(0.5, CKPT.debounceSecs + 1); const nb2 = written.length; if (written.length !== nBefore + 1) fails.push('a save landed with a man at awareness 0.5');
  pinB(0.35, 1); if (written.length !== nb2 + 1) fails.push(`awareness 0.35 (under ${CKPT.awareMax}) still blocked the save (gate ${S.gate(game)})`); else notes.push('awareness 0.5 blocks, 0.35 passes');
  B.awareness = 0; B.state = 'patrol'; B.lastKnown = null; B.reactT = 0;
  // MID-ACTION: picking a lock → blocked; the pick ends → lands. A live stun can → blocked till it has gone off.
  steps(W, CKPT.minGapSecs + 1);
  { const d = game.doors.byName('server'); const spot: Vec3 = [d.frameCentre[0], 0, d.def.hinge[1] + d.keySide * 0.9]; game.teleportPlayer(spot); game.player.crouch = true; steps(W, 2 * DT);
    if (game.doors.lockedOut(d, game.player.char.pos, game.player.keys)) { startPicking(game, d); steps(W, 0.3); S.request(game, 'probe', 'mid-pick'); steps(W, CKPT.debounceSecs + 0.5); const nb = written.length; if (game.player.picking && written.length !== nb) fails.push('a save landed mid-pick'); if (!/pick/.test(S.gate(game) ?? '') && game.player.picking) fails.push(`gate '${S.gate(game)}' mid-pick`);
      const nb3 = written.length; steps(W, 6, () => !game.player.picking); steps(W, 1); if (written.length !== nb3 + 1) fails.push(`the mid-pick request did not land when the pick ended (${written.length - nb3}; pending ${S.pending?.why ?? 'none'}; gate ${S.gate(game)})`); else notes.push('mid-pick: held, landed after'); }
    else fails.push('STAGING: the server door is not locked against him at its keyed face — the mid-pick case could not run'); }   // (a documented case that cannot be staged is a red, never a quiet skip)
  steps(W, CKPT.minGapSecs + 1);
  { game.dropThrowable('flash', [game.player.char.pos[0] + 3, 0, game.player.char.pos[2]]); for (const gd of game.guards) gd.dazzledUntil = -1;
    S.request(game, 'probe', 'live can'); const nb = written.length; steps(W, 0.2); if (!/fuse|air/.test(S.gate(game) ?? '')) fails.push(`gate '${S.gate(game)}' with a stun can on its fuse`);
    steps(W, CKPT.debounceSecs + 2.5); for (const gd of game.guards) { gd.awareness = 0; gd.state = 'patrol'; gd.lastKnown = null; gd.dazzledUntil = -1; gd.reactT = 0; } steps(W, 1.5);
    if (written.length !== nb + 1) fails.push(`the live-can request: ${written.length - nb} saves (pending ${S.pending?.why ?? 'none'}, gate ${S.gate(game)})`); else notes.push('stun can on its fuse: held, landed after the bang'); }
  // BODY: choke a parked man out, haul him 2 m, let go → 'body' request
  steps(W, 2);
  { const Cg = game.guards[2]; Cg.hold = false; place(Cg, [30, 0, 20], 0); killGuard(game, Cg, [0, 0, 1], true, 'choked'); steps(W, 2.5);
    for (const gd of game.guards) if (gd.state !== 'dead') { gd.awareness = 0; gd.state = 'patrol'; }
    const hips = Cg.char.bones.hips ?? Cg.char.pos; game.teleportPlayer([hips[0], 0, hips[2] - 0.75]); game.player.crouch = true; steps(W, CKPT.debounceSecs + 0.5);   // (the break room is a new region: its own request lands first)
    toggleDrag(game, Cg); if (!game.player.dragging) fails.push(`could not take hold of the body (${v3.distXZ(game.player.char.pos, hips).toFixed(2)} m off his hips)`);
    const rag = Cg.char.ragdoll; const b0: Vec3 = [rag.x[0], 0, rag.x[2]];
    for (let i = 0; i < 540 && game.player.dragging && Math.hypot(rag.x[0] - b0[0], rag.x[2] - b0[2]) < CKPT.bodyHideM + 0.3; i++) { W.input.keys.add('KeyS'); W.step(DT); } W.input.keys.delete('KeyS'); steps(W, DT);
    const hauled = Math.hypot(rag.x[0] - b0[0], rag.x[2] - b0[2]);
    const r0 = S.stats.requests; if (game.player.dragging) toggleDrag(game); steps(W, 2 * DT);
    if (S.stats.requests !== r0 + 1 || !/body/.test(S.pending?.why ?? '')) fails.push(`letting go of a body hauled ${hauled.toFixed(2)} m raised ${S.stats.requests - r0} request(s) (${S.pending?.why ?? S.log[S.log.length - 1]?.what})`); else notes.push(`body hauled ${hauled.toFixed(2)} m and dropped → request`);
    steps(W, CKPT.debounceSecs + 1); }
  // STAGE: into the server room → 'stage' request (+ maybe 'room')
  steps(W, CKPT.minGapSecs + 1);
  { const R = game.level.mission.serverRoom; const r0 = S.stats.requests; game.teleportPlayer([(R.x0 + R.x1) / 2, 0, (R.z0 + R.z1) / 2]); steps(W, 2 * DT);
    if (game.mission.stage !== 'drive') fails.push(`server room did not advance the stage (${game.mission.stage})`);
    if (!S.log.some((l: any) => /request: (room\+)?stage|request: stage/.test(l.what) || /\+stage/.test(l.what))) fails.push(`no 'stage' request in the log: ${S.log.slice(-3).map((l: any) => l.what).join(' | ')}`); else notes.push(`stage → requested (${S.stats.requests - r0} with the room)`);
    steps(W, CKPT.debounceSecs + 1); }
  // every write: never with a man alert / searching / over the line at that frame (the onSaved hook stamped the gate's world)
  // slots: only the ring's autosaves (+ index) in the store — auto.0‥2, or five of them under SAVETIMELINE — the index names no more, the newest is the last written
  { const N = ringSize(); const keys = [...store.map.keys()].filter((k: string) => k !== 'twopointsix.save.index' && !k.includes('.save.pic.')).sort(); if (keys.some((k: string) => { const m = /auto\.(\d)$/.exec(k); return !m || +m[1] >= N; })) fails.push(`store keys ${keys.join(',')}`);
    const slots = S.slots(); if (slots.length > N || (written.length >= N && slots.length !== N)) fails.push(`${slots.length} slots indexed after ${written.length} autosaves`);
    if (slots[0]?.label !== written[written.length - 1]?.label) fails.push(`newest slot '${slots[0]?.label}' ≠ last written '${written[written.length - 1]?.label}'`);
    notes.push(`${written.length} autosaves → slots [${slots.map((x: any) => x.key.replace('twopointsix.save.', '') + ':' + x.label).join(', ')}]`); }
  // the tag and the row
  if (!S.flash || S.flash.text !== 'CHECKPOINT') fails.push(`HUD flash ${JSON.stringify(S.flash)}`);
  { const rows = game.chronicle.events.filter((e: any) => e.kind === 'checkpoint' && e.data.what === 'save').length; if (rows !== written.length) fails.push(`${rows} 'checkpoint saved' rows for ${written.length} autosaves`); }
  if (game.chronicle.counts.saves !== written.length) fails.push(`counts.saves ${game.chronicle.counts.saves} ≠ ${written.length}`);
  const ok = !fails.length; if (!ok) familyFails++;
  verdictLine(ok, '(b) triggers: room (once per region), stage, body hauled + dropped — each a request; gates: alert / search / awareness > 0.4 / mid-pick / a can on its fuse hold the write, calm lands it at 25 s; debounce 2 s, gap 10 s; three rotating slots; the HUD tag + a chronicle row per save', notes.join('\n    '), fails);
}



