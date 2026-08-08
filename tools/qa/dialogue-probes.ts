// The 'dialogue' probe family (run through probes.ts: `bun run tools/qa/probes.ts dialogue | dialogue-none | dialogue-sawhurt | dialogue-heard | dialogue-intel |
// dialogue-lint | dialogue-seed`): the interrogation engine of src/game/dialogue.ts (docs/internal/grab-interrogate-design.md §4, slice 5 — the engine without the
// grab) driven through Game.interrogate on staged men. The two anchor cases from the design note — a man who never knew you were there (startled, knows
// nothing) and a man who watched you shoot his partner (agitated, hostile, names him) — then the heard-only man, the INTEL generator's knowledge gating (no keyed
// door for the cubicle man, the radio-check words against the net's real clock, an errand only men who were free to hear the order can mention), the table lint +
// a sweep of the whole knowledge × agitation × temperament matrix on synthetic contexts, and replay under a seed. Each prints what was said; PASS / FAIL.
import { standUp, ROOT } from './headless.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { killGuard, fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { TEMPERAMENT } = await import(`${ROOT}/src/game/game.ts`);
const guardsMod = await import(`${ROOT}/src/game/guards.ts`);
const { rollcallState, witnessSummary, pointInView } = guardsMod;
const D = await import(`${ROOT}/src/game/dialogue.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

/** A guard planted at `p` facing yaw, still, no path, posed twice (bones where the sight tests read them). */
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
/** parked out on the lot east of the building, held, facing east (away from everything) — still hearing, unless `deaf` */
function park(gd: any, i: number, deaf = false) { gd.hold = true; place(gd, [37.5, 0, 6 + i * 1.5], Math.PI / 2); if (deaf) gd.heardUpTo = 1e12; }
const chest = (gd: any): Vec3 => gd.char.bones.chest ?? v3.add(gd.char.pos, [0, 1.1, 0]);
/** tap game.say into an array (returns it) */
function tap(game: any) {
  const lines: { t: number; who: string; text: string; radio: boolean }[] = [];
  const origSay = game.say.bind(game); game.say = (gd: any, text: string, radio = false) => { lines.push({ t: game.time, who: gd.callsign, text, radio }); return origSay(gd, text, radio); };
  return { lines };
}
type Pick = { text: string; id: string; kind: string | null; stage: 1 | 2 | 3 | 'x'; stageN: number; cell: string; k: string; band: string; t: string; lineK: string; widened: string };
/** press him n times (a beat between presses so the bubbles are real bubbles), collecting the picks; Sam is put `behind` metres behind him first (his back to Sam, dark) */
function pressN(W: any, gd: any, n: number, rnd: () => number, behind = 1.0): (Pick | null)[] {
  const { game } = W; const f = gd.char.forward();
  game.teleportPlayer([gd.char.pos[0] - f[0] * behind, 0, gd.char.pos[2] - f[2] * behind]); game.player.visibility = 0.02; W.step(DT);
  const out: (Pick | null)[] = [];
  for (let i = 0; i < n; i++) { out.push(game.interrogate(gd, rnd)); for (let f2 = 0; f2 < 20; f2++) W.step(DT); }
  return out;
}
const fmtPick = (p: Pick | null) => p ? `s${String(p.stage)} [${p.id}${p.widened ? ' ~' + p.widened : ''}] "${p.text}"` : 'null';
/** the generic checks every transcript gets: no nulls, stages 1,2,3,x…, no unresolved token, no repeats (id or text), every line's own K his or '*', ≤ MAX_CHARS */
function transcriptFails(picks: (Pick | null)[], wantK: string): string[] {
  const fails: string[] = []; const ids = new Set<string>(), texts = new Set<string>();
  picks.forEach((p, i) => {
    if (!p) { fails.push(`press ${i + 1}: null pick`); return; }
    const wantStage = i < 3 ? i + 1 : 'x'; if (p.stage !== wantStage) fails.push(`press ${i + 1}: stage ${String(p.stage)} (want ${String(wantStage)})`);
    if (p.stageN !== i + 1) fails.push(`press ${i + 1}: stageN ${p.stageN}`);
    if (/[{}]/.test(p.text)) fails.push(`press ${i + 1}: unresolved token in "${p.text}"`);
    if (p.text.length > D.MAX_CHARS) fails.push(`press ${i + 1}: ${p.text.length} chars`);
    if (ids.has(p.id) && p.widened !== 'repeat') fails.push(`press ${i + 1}: repeated ${p.id} without the pool being dry`); ids.add(p.id);
    if (texts.has(p.text) && p.widened !== 'repeat') fails.push(`press ${i + 1}: repeated text`); texts.add(p.text);
    if (p.k !== wantK) fails.push(`press ${i + 1}: picked for K '${p.k}' (want ${wantK})`);
    if (p.lineK !== '*' && p.lineK !== wantK) fails.push(`press ${i + 1}: line ${p.id} is written for K '${p.lineK}', he is '${wantK}' — knowledge leaked`);
  });
  return fails;
}

// ================================================================ (1) never knew you were there: startled, knows nothing
/** Fresh floor, nothing has happened: the corridor man (Kowalski, steady) and the cubicle man (Novak, nervy) each get Sam a metre behind them in the dark and four
 *  presses. Both: knowledge 'none', agitation low (the nervy one's bias alone does not rattle him), press 1 from the startled / knows-nothing pool, stages 1 → 2 →
 *  3 → x, nothing repeated, every token resolved. The steady man's third press is the one useful thing he plausibly has (his route passes the keyed server-room
 *  door); the nervy one gives early (press 2) and, walking the cubicles past no keyed door, gives the radio-check rhythm instead. */
export async function dialogueNone() {
  console.log('\n=== dialogue-none: a man who never knew you were there — startled, knows nothing; stages advance to "nothing left" without repeats ===');
  let allPass = true;
  Math.random = seeded(501);
  const W = await standUp(); const { game } = W; tap(game);
  game.player.visibility = 0.02; game.godMode = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = game.guards;
  place(A, [14, 0, 11.2], Math.PI / 2); A.hold = true; place(B, [19.9, 0, 17.0], 0); B.hold = true; park(C, 0);
  rollcallState(game).nextAt = game.time + 100;   // (no radio check in flight during the presses — the next is 'in a couple of minutes': a live one outranks the door and the cadence in the generator's order, and this probe pins which useful thing each man gives)
  for (const [gd, label] of [[A, 'corridor man'], [B, 'cubicle man']] as [any, string][]) {
    const k0 = D.knowledgeOf(game, gd), a0 = D.agitationOf(game, gd), band0 = D.bandOf(a0), t = D.temperamentOf(gd);
    const ctx0 = D.gather(game, gd);
    const picks = pressN(W, gd, 4, seeded(11));
    const fails = transcriptFails(picks, 'none');
    if (k0 !== 'none') fails.push(`knowledge ${k0} (want none) — witness: ${witnessSummary(game, gd) || 'empty'}`);
    if (band0 !== 'low') fails.push(`agitation ${a0.toFixed(2)} ${band0} (want low)`);
    const p1 = picks[0]; if (p1 && (p1.lineK !== 'none' || p1.stage !== 1 || p1.kind)) fails.push(`press 1 not from the startled pool: ${fmtPick(p1)}`);
    const gives = picks.filter(p => p?.kind).map(p => `${p!.kind}@s${String(p!.stage)}`);
    if (t === 'steady' && !(picks[2]?.kind)) fails.push(`steady man gave nothing at press 3 (${fmtPick(picks[2])})`);
    if (t === 'nervy' && !(picks[1]?.kind)) fails.push(`nervy man gave nothing at press 2 (${fmtPick(picks[1])})`);
    if (gd === B && picks.some(p => p?.kind === 'door')) fails.push('the cubicle man gave a keyed door — none is on his route');
    if (gd === A && picks[2]?.kind !== 'door') fails.push(`the corridor man's useful thing was ${picks[2]?.kind} (want door: his route passes the server room's keyed door)`);
    if (D.knowledgeOf(game, gd) !== 'none') fails.push(`knowledge moved to ${D.knowledgeOf(game, gd)} while pressed (Sam was meant to be dark and behind him): ${witnessSummary(game, gd)}`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: ${gd.callsign} (${t}) · K ${k0} · A ${a0.toFixed(2)} ${band0} [${ctx0.why.join(', ') || '—'}] · gave ${gives.join(', ') || 'nothing'}\n    ` + picks.map(fmtPick).join('\n    ') + (fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''));
  }
  console.log(`  panel line: ${game.dialogueSummary()}`);
  console.log(`=== dialogue-none: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (2) watched you shoot his partner: agitated, hostile, names him  +  (3) the man who only heard it
/** The witness probe's staging: A (Kowalski) mid-corridor facing east with Sam dark behind him, B (Novak) six metres east facing west with A in his open view, C
 *  (Reyes) parked on the lot behind two walls, hearing. Sam stands and shoots A. B SAW it (witness.sawAct shot / Kowalski, through the real killGuard path) and
 *  goes alert on the shot; C HEARD it and saw nothing. Both are pinned where they stand (an alert man would otherwise run to the shot) and pressed four times. */
async function stageShooting(seed: number) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W; const T = tap(game);
  game.player.visibility = 0.02; game.godMode = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = game.guards;
  place(A, [14, 0, 11.2], Math.PI / 2); A.hold = true; place(B, [20, 0, 11.2], -Math.PI / 2); B.hold = true; park(C, 1);
  game.teleportPlayer([13.2, 0, 11.2]); game.player.crouch = true; W.step(DT); game.player.visibility = 0.02;
  const los = { bSeesA: pointInView(game, B, chest(A)), cSeesA: pointInView(game, C, chest(A)), bSeesSam: B.sawPlayerThisFrame };
  game.player.crouch = false; W.step(DT);
  fireWeapon(game, game.player.char, chest(A), true);
  B.pinned = true; C.pinned = true;   // (alert men hold where they are: this is about what they say, not where they run)
  for (let i = 0; i < 90; i++) W.step(DT);   // a second and a half: the shot has reached everyone, B has called the body in
  game.player.visibility = 0.02;
  return { W, game, A, B, C, los, lines: T.lines };
}
export async function dialogueSawHurt() {
  console.log('\n=== dialogue-sawhurt: the man who watched you shoot his partner — sawHurt, agitation high, hostile pool, names the victim ===');
  const S = await stageShooting(502); const { W, game, A, B, los } = S;
  const fails: string[] = [];
  if (!los.bSeesA) fails.push("staging: A was not in B's open view"); if (los.bSeesSam) fails.push('staging: B had eyes on Sam before the shot');
  if (A.state !== 'dead' || A.downKind !== 'shot') fails.push(`staging: A ${A.state}/${A.downKind}`);
  if (!B.witness.sawAct || B.witness.sawAct.victim !== A.callsign || B.witness.sawAct.kind !== 'shot') fails.push(`B.witness.sawAct ${JSON.stringify(B.witness.sawAct)} (want shot / ${A.callsign})`);
  const k0 = D.knowledgeOf(game, B), a0 = D.agitationOf(game, B), band0 = D.bandOf(a0), ctx0 = D.gather(game, B);
  if (k0 !== 'sawHurt') fails.push(`knowledge ${k0} (want sawHurt)`);
  if (band0 !== 'high') fails.push(`agitation ${a0.toFixed(2)} ${band0} (want high) [${ctx0.why.join(', ')}]`);
  if (ctx0.tokens.victim !== A.callsign) fails.push(`{victim} resolves to ${ctx0.tokens.victim} (want ${A.callsign})`);
  const picks = pressN(W, B, 4, seeded(12));   // (a metre behind him as he now stands — alert, pinned, facing where the shot came from; inside 1.8 m in front even a dark Sam is seen point-blank)
  fails.push(...transcriptFails(picks, 'sawHurt'));
  const p1 = picks[0];
  if (!p1 || !p1.text.includes(A.callsign)) fails.push(`press 1 does not name the victim ${A.callsign}: ${fmtPick(p1)}`);
  if (p1 && (p1.lineK !== 'sawHurt' || p1.kind)) fails.push(`press 1 not from the hostile (sawHurt) pool: ${fmtPick(p1)}`);
  const needs1: string[] = p1 ? (D.LINES.find((l: any) => l.id === p1.id)?.needs ?? []) : [];
  if (needs1.includes('sawDrop')) fails.push(`press 1 is a takedown-witness line (${p1!.id} needs sawDrop) for a man who watched a SHOT`);
  const ok = fails.length === 0;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${B.callsign} (${D.temperamentOf(B)}, ${B.state}) after watching ${A.callsign} shot: K ${k0} · A ${a0.toFixed(2)} ${band0} [${ctx0.why.join(', ')}]\n    witness: ${witnessSummary(game, B)}\n    ` + picks.map(fmtPick).join('\n    ') + (fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''));
  // the same man, hard: Reyes in B's shoes (temperament is by callsign — swap the callsigns of B and C for a second staging) — cold fury, 'No.' at the giving stage unless rattled (he is: high)
  {
    const S2 = await stageShooting(503); const { W: W2, game: g2, A: A2, B: B2, C: C2 } = S2;
    const f2: string[] = [];
    [B2.callsign, C2.callsign] = [C2.callsign, B2.callsign];   // Reyes now stands where Novak stood (TEMPERAMENT is keyed by callsign)
    if (D.temperamentOf(B2) !== 'hard') f2.push(`staging: swapped man is ${D.temperamentOf(B2)}`);
    const kk = D.knowledgeOf(g2, B2), aa = D.agitationOf(g2, B2), cx = D.gather(g2, B2);
    const picks2 = pressN(W2, B2, 4, seeded(13));
    f2.push(...transcriptFails(picks2, 'sawHurt'));
    if (kk !== 'sawHurt') f2.push(`knowledge ${kk}`);
    if (!picks2[0]?.text.includes(A2.callsign)) f2.push(`press 1 does not name ${A2.callsign}`);
    if (D.bandOf(aa) === 'low' && picks2.some(p => p?.kind)) f2.push('a hard man at low agitation gave something');
    const gave = picks2.find(p => p?.kind); if (gave && !gave.text.startsWith('…Fine.')) f2.push(`a hard man gave "${gave.text}" without the grudging lead-in`);
    const ok2 = f2.length === 0; fails.push(...f2);
    console.log(`- ${ok2 ? 'PASS' : 'FAIL'} — the same, hard (${B2.callsign} in his place): K ${kk} · A ${aa.toFixed(2)} ${D.bandOf(aa)} [${cx.why.join(', ')}]\n    ` + picks2.map(fmtPick).join('\n    ') + (f2.length ? '\n    ✗ ' + f2.join('\n    ✗ ') : ''));
  }
  const all = fails.length === 0;
  console.log(`=== dialogue-sawhurt: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
export async function dialogueHeard() {
  console.log('\n=== dialogue-heard: the man round the corner who only HEARD the shot — the heard pool, never a line written for men who saw ===');
  const S = await stageShooting(504); const { W, game, A, C, los } = S;
  const fails: string[] = [];
  if (los.cSeesA) fails.push('staging: C on the lot had a line to A');
  if (C.witness.sawAct) fails.push(`C.sawAct ${JSON.stringify(C.witness.sawAct)} behind two walls`);
  if (C.witness.heard.shots < 1) fails.push(`C.heard.shots ${C.witness.heard.shots} (gunfire carries to the lot)`);
  const k0 = D.knowledgeOf(game, C), a0 = D.agitationOf(game, C), ctx0 = D.gather(game, C);
  if (k0 !== 'heard') fails.push(`knowledge ${k0} (want heard)`);
  if (ctx0.tokens.victim) fails.push(`{victim} resolves (${ctx0.tokens.victim}) for a man who saw nothing`);
  const picks = pressN(W, C, 5, seeded(14));
  fails.push(...transcriptFails(picks.slice(0, 4), 'heard'));
  for (const p of picks) {
    if (!p) continue;
    if (['glimpsed', 'saw', 'sawHurt'].includes(p.lineK)) fails.push(`line ${p.id} is for men who SAW ('${p.lineK}'): "${p.text}"`);
    if (/\bI (saw|watched) you\b|\byou shot\b|\bI had you\b/i.test(p.text)) fails.push(`"${p.text}" claims sight`);
  }
  if (D.knowledgeOf(game, C) !== 'heard') fails.push(`knowledge moved to ${D.knowledgeOf(game, C)} while pressed`);
  const ok = fails.length === 0;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${C.callsign} (${D.temperamentOf(C)}, ${C.state}) on the lot: K ${k0} · A ${a0.toFixed(2)} ${D.bandOf(a0)} [${ctx0.why.join(', ')}] · body token ${ctx0.tokens.body ?? '—'} (he heard ${A.callsign} called in: ${!!C.witness.calledToBody})\n    witness: ${witnessSummary(game, C)}\n    ` + picks.map(fmtPick).join('\n    ') + (fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''));
  console.log(`=== dialogue-heard: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`);
  return ok;
}

// ================================================================ (4) INTEL gating: only what he plausibly knows
/** (a) the keyed door: resolves for the corridor and lobby men (their routes pass the server / manager / storage doors), never for the cubicle man — and six presses
 *      of him never produce one. (b) {checkEta} / {minutes} against the net's real radio-check clock (rollcallState): the words track nextAt as it is moved about,
 *      {minutes} is null before the first check and counts from its call after. (c) the errand: the lobby man dropped quietly out of everyone's sight, the corridor
 *      man 'held' (talk.heldSince stamped) BEFORE the net notices — when the check finds Reyes silent and sends Novak round his route, Novak (the checker) gives it in
 *      his own words, Kowalski held through the order cannot know it (his intel skips it), and the same Kowalski freed and grabbed afresh after the order can.
 *      (Reyes lies in the locked storage room so that no route simply finds him first: this is about the net's errand, not a body call.) */
export async function dialogueIntel() {
  console.log('\n=== dialogue-intel: the useful thing is gated by what he could know — doors on HIS route, the check clock as it is, an errand only if he heard the order ===');
  let allPass = true; const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  // ---- (a) doors on his route
  {
    Math.random = seeded(505);
    const W = await standUp(); const { game } = W; tap(game);
    game.player.visibility = 0.02; game.godMode = true;
    for (let i = 0; i < 5; i++) W.step(DT);
    const [A, B, C] = game.guards; const fails: string[] = [];
    const doors = game.guards.map((gd: any) => `${game.level.routes[gd.routeI].name}: ${D.keyedDoorOnRoute(game, gd) ?? 'none'}`);
    if (!D.keyedDoorOnRoute(game, A)) fails.push('corridor man has no keyed door on his route');
    if (D.keyedDoorOnRoute(game, B)) fails.push(`cubicle man got ${D.keyedDoorOnRoute(game, B)}`);
    if (!D.keyedDoorOnRoute(game, C)) fails.push('lobby man has none (his loop walks the corridor past three)');
    place(B, [19.9, 0, 17.0], 0); B.hold = true; A.hold = true; park(C, 0, true);
    const live = D.gather(game, B).intel.filter((i: any) => i.live && i.known).map((i: any) => i.kind);
    if (live.includes('door')) fails.push(`cubicle man's live intel includes door: ${live.join(', ')}`);
    const picks = pressN(W, B, 6, seeded(21));
    if (picks.some(p => p?.kind === 'door' || (p && /\bdoor\b.*\bkey|keyed\b/i.test(p.text)))) fails.push(`a press produced a keyed door: ${picks.map(fmtPick).join(' | ')}`);
    verdict(fails.length === 0, '(a) keyed doors resolve per route; the cubicle man never mentions one in six presses',
      `${doors.join(' · ')}\n    cubicle man's live intel: ${live.join(', ') || '—'} · gave: ${picks.filter(p => p?.kind).map(p => `${p!.kind}@s${String(p!.stage)} "${p!.text}"`).join(' | ') || 'nothing'}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (b) the radio-check words against the clock
  {
    Math.random = seeded(506);
    const W = await standUp(); const { game } = W; const { lines } = tap(game);
    game.player.visibility = 0.02; game.godMode = true; game.playerInvisible = true; game.teleportPlayer([37.5, 0, 26.5]);
    for (let i = 0; i < 5; i++) W.step(DT);
    const A = game.guards[0]; const R = rollcallState(game); const fails: string[] = []; const rows: string[] = [];
    if (!(R.nextAt > game.time)) fails.push(`staging: no check scheduled (nextAt ${R.nextAt})`);
    const eta0 = D.gather(game, A).tokens.checkEta; if (eta0 !== D.etaWords(R.nextAt - game.time)) fails.push(`checkEta "${eta0}" vs clock ${(R.nextAt - game.time).toFixed(0)} s → "${D.etaWords(R.nextAt - game.time)}"`);
    rows.push(`scheduled in ${(R.nextAt - game.time).toFixed(0)} s → "${eta0}"`);
    for (const [secs, want] of [[10, 'any second now'], [60, 'in a minute or so'], [120, 'in a couple of minutes'], [240, 'in four minutes']] as [number, string][]) {
      R.nextAt = game.time + secs; const got = D.gather(game, A).tokens.checkEta; rows.push(`${secs} s → "${got}"`); if (got !== want) fails.push(`nextAt +${secs} s gives "${got}" (want "${want}")`);
    }
    if (D.gather(game, A).tokens.minutes !== null) fails.push(`{minutes} before any check: ${D.gather(game, A).tokens.minutes} (want null)`);
    R.nextAt = game.time + 1;   // let a real check run now
    let called = -1; for (let f = 0; f < 60 * 20 && called < 0; f++) { W.step(DT); const l = lines.find(x => /sound off/.test(x.text)); if (l) called = l.t; }
    if (called < 0) fails.push('no radio check ran in 20 s');
    const m1 = D.gather(game, A).tokens.minutes; if (m1 !== 'one') fails.push(`{minutes} right after the check: ${m1} (want one)`);
    const live1 = D.gather(game, A).facts.rollcallLive; rows.push(`check called at ${called.toFixed(1)} s → minutes "${m1}", rollcallLive ${live1}, checkEta "${D.gather(game, A).tokens.checkEta}"`);
    for (let f = 0; f < 60 * 100; f++) W.step(DT);
    const m2 = D.gather(game, A).tokens.minutes; const wantM2 = ['one', 'two', 'three'][Math.max(1, Math.round((game.time - R.t0) / 60)) - 1];
    if (m2 !== wantM2) fails.push(`{minutes} 100 s on: ${m2} (want ${wantM2} — from R.t0 ${R.t0.toFixed(0)}, now ${game.time.toFixed(0)})`);
    rows.push(`+100 s → minutes "${m2}" (R.t0 ${R.t0.toFixed(0)} s, checks ${R.checks})`);
    verdict(fails.length === 0, '(b) {checkEta} follows the net\'s nextAt; {minutes} is null before the first check and counts from its call', rows.join(' · ') + (fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''));
  }
  // ---- (c) the errand: only men free to hear the order
  {
    Math.random = seeded(507);
    const W = await standUp(); const { game } = W; const { lines } = tap(game);
    game.player.visibility = 0.02; game.godMode = true; game.playerInvisible = true; game.teleportPlayer([37.5, 0, 26.5]);
    for (let i = 0; i < 60 * 3; i++) W.step(DT);
    const [A, B, C] = game.guards; const fails: string[] = [];
    place(C, [33.6, 0, 6.2], Math.PI); killGuard(game, C, [0, 0, -1], true, 'struck');   // the lobby man out cold in the locked storage room, where no route looks: silent on the net, never simply found
    const tKill = game.time;
    A.talk.heldSince = game.time; A.hold = true;   // Kowalski 'held' from now (what the grab will stamp) — and standing his ground, so the net sends Novak, not him
    let tSend = -1; for (let f = 0; f < 60 * 150 && tSend < 0; f++) { W.step(DT); if (B.task?.kind === 'checkOn') tSend = game.time; }
    if (tSend < 0) fails.push(`nobody was sent round ${C.callsign}'s route in 150 s (A ${A.state} task ${A.task?.kind ?? '—'}, B ${B.state} task ${B.task?.kind ?? '—'}, C found ${C.found}; lines: ${lines.filter(l => l.radio).map(l => l.text).slice(-4).join(' | ')})`);
    else {
      const heldCtx = D.gather(game, A); const item = heldCtx.intel.find((i: any) => i.kind === 'checkOn');
      const heldGive = D.intelFor(heldCtx, A.talk);
      if (!item?.live) fails.push('checkOn not live for A while B walks the route');
      if (item?.known) fails.push(`A, held since ${A.talk.heldSince.toFixed(0)} s (order at ${B.task.t0.toFixed(0)} s), KNOWS the errand`);
      if (heldGive?.kind === 'checkOn') fails.push(`held A would give the errand: "${heldGive.text}"`);
      const selfCtx = D.gather(game, B); const selfGive = D.intelFor(selfCtx, B.talk);
      if (selfGive?.kind !== 'checkOn' || !/I was sent round/.test(selfGive.text) || !selfGive.text.includes(C.callsign)) fails.push(`the checker's own useful thing: ${JSON.stringify(selfGive)} (want checkOn in his own words naming ${C.callsign})`);
      A.talk.heldSince = -1;   // let go and grabbed afresh after the order went out: now he heard it
      const freeCtx = D.gather(game, A); const freeGive = D.intelFor(freeCtx, A.talk);
      if (freeGive?.kind !== 'checkOn' || !freeGive.text.includes(B.callsign) || !freeGive.text.includes(C.callsign)) fails.push(`A grabbed after the order: ${JSON.stringify(freeGive)} (want checkOn naming ${B.callsign} and ${C.callsign})`);
      // and through the real presses: A (steady) gives it at press 3, said as a bubble
      A.hold = true; const picks = pressN(W, A, 3, seeded(31));
      if (picks[2]?.kind !== 'checkOn') fails.push(`A's press 3: ${fmtPick(picks[2])} (want the errand)`);
      verdict(fails.length === 0, '(c) the check-on errand: unknown to a man held through the order, the checker\'s in his own words, known to a man grabbed after it',
        `${C.callsign} dropped at ${tKill.toFixed(0)} s, ${B.callsign} sent at ${tSend.toFixed(0)} s\n    held ${A.callsign}: live ${item?.live} known ${item?.known} → gives ${heldGive?.kind ?? 'nothing'}${heldGive ? ` "${heldGive.text}"` : ''}\n    checker ${B.callsign}: "${selfGive?.text ?? '—'}"\n    ${A.callsign} grabbed after: "${freeGive?.text ?? '—'}"\n    presses: ${picks.map(fmtPick).join(' | ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    }
    if (tSend < 0) verdict(false, '(c) the check-on errand', fails.join(' · '));
  }
  console.log(`=== dialogue-intel: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (5) the table lint + the whole matrix on synthetic contexts
export async function dialogueLint() {
  console.log('\n=== dialogue-lint: the line table (tokens, needs, length, duplicates, every cell reachable) and a sweep of the K × A × T matrix ===');
  let allPass = true;
  const problems: string[] = D.lintDialogue();
  { const ok = problems.length === 0; allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — lintDialogue(): ${ok ? 'clean' : problems.length + ' problems'}${ok ? '' : '\n    ✗ ' + problems.join('\n    ✗ ')}`); }
  { const M = D.coverageMatrix(); console.log(`  coverage (${M.total} pool lines + ${M.intel} intel texts): stage →   1   2   3   x`); for (const r of M.rows) console.log(`    ${r.k.padEnd(20)} ${['1', '2', '3', 'x'].map(s => String(r.counts[s]).padStart(3)).join(' ')}   = ${r.total}`); }
  // the matrix: 5 K × 3 bands × 3 T, 24 sessions of 6 presses each with the facts / live intel dealt at random — no line above his K, no token left, no silent repeat
  {
    const rnd = seeded(508); const fails: string[] = []; let picksN = 0, sessions = 0, repeatsAfterDry = 0, gave = 0; const idsSeen = new Set<string>();
    const NEED_KEYS = ['floorUp', 'lockdown', 'episode', 'bodyKnown', 'bodyCalled', 'bodyFound', 'missingKnown', 'blackout', 'heldLong', 'rollcallLive', 'dazzled', 'heardShots', 'drewOnYou', 'alertNow'];
    for (const k of D.KNOWLEDGE) for (const band of D.BANDS) for (const t of D.TEMPERAMENTS) for (let sess = 0; sess < 24; sess++) {
      sessions++;
      const facts: Record<string, boolean> = {}; for (const n of NEED_KEYS) facts[n] = rnd() < 0.3;
      if (k === 'sawHurt') { facts.sawShot = rnd() < 0.6; facts.sawDrop = !facts.sawShot; }
      const tokens: Record<string, string | null> = { body: facts.bodyKnown || facts.bodyCalled || facts.bodyFound ? 'Reyes' : null, missing: facts.missingKnown ? 'Okafor' : null, partner: rnd() < 0.85 ? 'Novak' : null, door: rnd() < 0.6 ? 'the storage door' : null, checker: rnd() < 0.2 ? 'Brandt' : null, checked: 'Reyes', clearRoom: rnd() < 0.2 ? 'the break room' : null, minutes: rnd() < 0.5 ? 'two' : null };
      facts.partnerKnown = !!tokens.partner;
      const intelLive = rnd() < 0.3 ? [] : D.INTEL_KINDS.filter(() => rnd() < 0.35);   // (some sessions with nothing useful to give at all: the giving stages fall to the pools)
      const talk = { stage: 0, presses: 0, said: new Set<string>(), saidN: new Map<string, number>(), given: new Set<string>(), lastT: -1, heldSince: -1, last: null };
      const ids = new Set<string>();
      for (let press = 0; press < 6; press++) {
        const ctx = D.synthCtx(k, band, t, { facts, tokens, intelLive });
        const p: Pick = D.pickLine(ctx, talk, rnd); picksN++; idsSeen.add(p.id); if (p.kind) gave++;
        const where = `${k}·${band}·${t} #${sess} press ${press + 1}`;
        if (p.lineK !== '*' && p.lineK !== k) fails.push(`${where}: ${p.id} written for '${p.lineK}' — LEAK`);
        if (/[{}]/.test(p.text) || !p.text.trim() || p.text === '…') fails.push(`${where}: bad text "${p.text}" (${p.id})`);
        if (p.text.length > D.MAX_CHARS) fails.push(`${where}: ${p.text.length} chars`);
        if (ids.has(p.id)) { if (p.widened !== 'repeat') fails.push(`${where}: ${p.id} repeated while its pool still had unsaid lines`); else repeatsAfterDry++; }
        ids.add(p.id);
        const wantStage = press < 3 ? press + 1 : 'x'; if (p.stage !== wantStage) fails.push(`${where}: stage ${String(p.stage)}`);
        if (fails.length > 12) break;
      }
    }
    const unused = D.LINES.map((l: any) => l.id).filter((id: string) => !idsSeen.has(id));
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — matrix sweep: ${sessions} sessions, ${picksN} picks, ${gave} useful things given, ${repeatsAfterDry} repeats (all after the pool ran dry), ${idsSeen.size} distinct ids used; never used: ${unused.join(', ') || 'none'}${fails.length ? '\n    ✗ ' + fails.slice(0, 12).join('\n    ✗ ') : ''}`);
  }
  console.log(`=== dialogue-lint: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (6) replay under a seed
/** The fresh-floor staging twice with the same seeds: the eight lines (four each from the corridor and the cubicle man) come out identical; with another picker
 *  seed at least one differs (reported, not required — small pools can legitimately agree). */
export async function dialogueSeed() {
  console.log('\n=== dialogue-seed: same seed, same words ===');
  const run = async (pickSeed: number) => {
    Math.random = seeded(509);
    const W = await standUp(); const { game } = W; tap(game);
    game.player.visibility = 0.02; game.godMode = true;
    for (let i = 0; i < 5; i++) W.step(DT);
    const [A, B, C] = game.guards; place(A, [14, 0, 11.2], Math.PI / 2); A.hold = true; place(B, [19.9, 0, 17.0], 0); B.hold = true; park(C, 0);
    const rnd = seeded(pickSeed);
    return [...pressN(W, A, 4, rnd), ...pressN(W, B, 4, rnd)].map(p => p ? `${p.id}:${p.text}` : 'null');
  };
  const a = await run(77), b = await run(77), c = await run(78);
  const same = a.length === b.length && a.every((x, i) => x === b[i]); const differs = a.some((x, i) => x !== c[i]);
  console.log(`- ${same ? 'PASS' : 'FAIL'} — two runs with picker seed 77: ${same ? 'identical' : 'DIFFER'} (8 lines); seed 78 ${differs ? 'differs' : 'happens to agree'}\n    ` + a.map((x, i) => `${x === b[i] ? '=' : '≠'} ${x}${x !== c[i] ? `   | 78: ${c[i]}` : ''}`).join('\n    '));
  console.log(`=== dialogue-seed: ${same ? 'ALL PASS' : 'FAILURES above'} ===`);
  return same;
}
