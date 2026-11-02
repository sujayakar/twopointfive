// TWO NOTES FROM A PLAY TEST ON THE SQUAD'S KEY (the maintainer, 11-02, passing them on: "double tap Q -> cancel everything and fall in" and "first
// shift Q should just send half the squad, whoever's closest"):
//   CMDRING.goalHalf (src/game/planmode.ts goalMen) — with nothing going, Shift + the key sends HALF the squad, and the rest stay with him.
//   CMDRING.twice (src/game/orders.ts tapGive) — the key tapped twice on the same thing is 'fall in' for everyone; the first tap is still given at once.
//   bun run tools/qa/halftwice-check.ts            (also `probes.ts halftwice`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/halftwice-check.ts      — the same lines on the page's stand
// PASS / FAIL each, headless, on goals-check.ts's own rig (the relay post's floor, the room clear, the squad of four under god mode, seed 1; the keys
// pressed as a hand presses them). The check raises the ring, the goals and the two switches itself, so both stands ask the same lines.
//  H1. nobody on anything, Shift + the key on a door: ONE whole pair is sent to stack there, the pair that stands nearer it, and the other two are
//      still with him.
//  H2. Shift + the key on the second door: the two who stayed stack there, and the first pair is still on the first.
//  H3. nobody on anything, Shift + the key on a spot: two cover it and two are still with him. A second spot: two cover each. A third: two, one and one.
//  H4. nobody on anything, Shift + the key KEPT DOWN on a door: the ring is one whole pair's and nobody is told anything; its first wedge: that pair
//      stacks there and the other two are still with him. On himself the rule still names everyone.
//  H5. ONE teammate: Shift + the key on a door is his (half of one man is nobody: it stays everyone's).
//  H7. TWO teammates: Shift + the key on a door sends the nearer man alone, and the other is still with him; on the second door, the other.
//  H8. THREE teammates: Shift + the key on a door sends the nearer of the whole pair and the odd man; on the second door, the rest.
//  H6. CONTROL — the switch off: nobody on anything, Shift + the key on a door: all four stack on it, as the goals were built.
//  T1. the key tapped on a door: all four are sent to stack on it at once. Tapped again inside the window: all four fall in.
//  T2. two pairs on two doors by two goals, the key tapped twice: all four fall in, nobody is left on a door, and no door tool is left running.
//  T3. tapped again AFTER the window, on the same door: nobody falls in.
//  T4. tapped on one door and at once on the other: everyone is sent to the second door; nobody falls in.
//  T5. a tap with Shift and at once a plain tap on the same door: the plain key's own order, everyone's; nobody falls in.
//  T6. CONTROL — the switch off: tapped twice on a door, nobody falls in.
//  T7. on a tutorial mission the second tap is 'fall in' only from a step that lists `twice` (the rule asked directly, the frame stood in;
//      the fourth tutorial played with the key tapped twice is tutqueue-check's line 10).
//  T8. a spot, then a spot a pace off inside the window (the hand moved): the second spot's own order, and nobody falls in.
//  T9. a hand that has not moved: a spot tapped twice is 'fall in'.
// T10. three taps on a door: stack, fall in, stack — the third starts again.
// T11. on himself twice: the first tap is 'fall in', and the second tells them nothing more.
// T12. a pause between the two taps (the loop's 'Thawed', the first tick after a frozen frame): the tap after it is a first tap, and all four stack.
//   P. CONTROL (plumbing) — a recording from before the keys replays with both off; the frozen stand holds both off, the page's has both on,
//      FROZEN_HOLDS has their rows.
// GOALHALF_BREAK=1 = that switch left off and the same lines asked: H1 to H4, H7 and H8 must fail.
// QTWICE_BREAK=1 = that switch left off: T1, T2 and T9 to T11 must fail.
//   exit 0 = all hold · 1 = one does not
import { ROOT } from './headless.ts';
import { rig, stoodCmd } from './goals-check.ts';
const { RAILS, RAILS_ACTIONS } = await import(`${ROOT}/src/game/rails.ts`);
const { CMDRING, CMDRING_STATS } = await import(`${ROOT}/src/game/cmdring.ts`);
const PM: any = await import(`${ROOT}/src/game/planmode.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const { applyKnobs, knobsSnapshot } = await import(`${ROOT}/src/game/journal.ts`);
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const HBREAK = process.env.GOALHALF_BREAK === '1', TBREAK = process.env.QTWICE_BREAK === '1';
type V = number[];
const flat = (a: V, b: V) => Math.hypot(a[0] - b[0], a[2] - b[2]);

export async function halftwiceCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rails0 = RAILS.on, K = CMDRING;
  console.log(`The first goal sends half the squad (CMDRING.goalHalf) and the key tapped twice is 'fall in' (CMDRING.twice)${HBREAK ? ' — GOALHALF_BREAK=1: that switch left off, H1 to H4, H7 and H8 must fail' : TBREAK ? ' — QTWICE_BREAK=1: that switch left off, T1, T2 and T9 to T11 must fail' : ''}`);
  try {
    const R = await rig(true, { goalHalf: !HBREAK, twice: !TBREAK }), g = R.g, O = R.O, D = R.D, W = R.W, N = O.men('all').length;
    const A = R.door('guard'), B = R.door('guard_side'), names = (l: any[]) => l.map((m: any) => m.callsign).sort().join(', ') || 'nobody';
    if (R.why || !A || !B || N !== 4) { line(false, 'the room clear on the relay post\'s floor, four men and the guardroom\'s two doors', R.why ?? `${N} men · doors ${A ? 'guard' : '-'} ${B ? 'guard_side' : '-'}`); return false; }
    /** the men still with him: no order of their own, or in the file behind him */
    const withHim = (): any[] => O.men('all').filter((m: any) => { const a = O.of(m); return !a || a.post === null; });
    const fallen = (): any[] => O.men('all').filter((m: any) => O.of(m)?.order === 'fallIn');
    const far = (men: any[], d: any) => men.reduce((s: number, m: any) => s + flat(m.char.pos, d.frameCentre), 0) / Math.max(1, men.length);
    const sizes = (): string => [...R.watches().values()].map((l: string[]) => l.length).sort((x: number, y: number) => y - x).join(' + ') || 'nobody covering';

    // H1, H2
    R.home();
    { const s0 = PM.PLAN_STATS.split, onA = R.onDoor(A), pairs: any[][] = O.pairs().map((id: any) => O.men('all').filter((m: any) => O.pairOf(m) === id)), near0 = [...pairs].sort((x, y) => far(x, A) - far(y, A))[0];
      R.key(true); const a = R.onStack(A), rest = withHim(), pair = a.length === 2 && O.pairOf(a[0]) === O.pairOf(a[1]);
      line(onA && pair && rest.length === 2 && names(a) === names(near0) && PM.PLAN_STATS.split - s0 === 1, 'H1. nobody on anything, Shift + the key on a door: one whole pair is sent to stack there, the nearer pair, and the other two are still with him',
        `the cursor on the door: ${onA} · sent to stack on it: ${names(a)}${pair ? ` (${O.pairOf(a[0])})` : ''} · the nearer pair was ${names(near0)} · still with him: ${names(rest)}`);
      R.step(20); const onB = R.onDoor(B); R.key(true); const a2 = R.onStack(A), b2 = R.onStack(B);
      line(onB && b2.length === 2 && names(b2) === names(rest) && names(a2) === names(a) && a2.length === 2, 'H2. Shift + the key on the second door: the two who stayed stack there, and the first pair is still on the first',
        `the cursor on the second door: ${onB} · on the first: ${names(a2)} · on the second: ${names(b2)}`); }
    // H3
    R.home();
    { const spots: V[] = [], add = (): string => { const p = R.onSpot(spots); if (!p) return 'no spot'; spots.push(p); R.key(true); R.step(10); return sizes(); };
      const s1 = add(), rest1 = withHim().length, s2 = add(), s3 = add();
      line(s1 === '2' && rest1 === 2 && s2 === '2 + 2' && s3 === '2 + 1 + 1', 'H3. nobody on anything, Shift + the key on a spot: two cover it and two are still with him. A second spot: two cover each. A third: two, one and one',
        `covering after the first: ${s1}, still with him ${rest1} · the second: ${s2} · the third: ${s3}`); }
    // H4
    R.home();
    { const onA = R.onDoor(A), n0 = O.log.length, ring = R.hold(), part: any[] = O.menu?.part ?? [], told = O.log.length - n0, pair = part.length === 2 && O.pairOf(part[0]) === O.pairOf(part[1]);
      const text = ring ? R.wedge(1) : null; if (!ring) R.away(); R.step(20); const a = R.onStack(A), rest = withHim();
      const p = g.player.char.pos, self = PM.goalMen(O, { kind: 'none', at: [p[0], 0, p[2]] });
      line(onA && ring && pair && told === 0 && names(a) === names(part) && rest.length === 2 && self?.how === 'all' && self.men.length === N,
        'H4. nobody on anything, Shift + the key KEPT DOWN on a door: the ring is one whole pair\'s; its first wedge: that pair stacks there and the other two are still with him. On himself the rule names everyone',
        `the ring ${ring ? 'up' : 'NOT up'}, for ${names(part)}, orders while it was up ${told} · the wedge '${text}' · on the door: ${names(a)} · still with him: ${names(rest)} · on himself: ${self?.how} (${self?.men.length})`); }
    // H6 (H5 has a rig of its own, below)
    R.home(); K.goalHalf = false;
    { const onA = R.onDoor(A); R.key(true); const a = R.onStack(A);
      line(onA && a.length === N, 'H6. CONTROL — the switch off: nobody on anything, Shift + the key on a door: all four stack on it', `sent to stack on it: ${names(a)}`); }
    K.goalHalf = !HBREAK;

    // T1
    R.home();
    { const t0 = CMDRING_STATS.twice, onA = R.onDoor(A); R.key(false); const first = R.onStack(A).length; R.key(false); const f = fallen();
      line(onA && first === N && f.length === N && R.onStack(A).length === 0 && CMDRING_STATS.twice - t0 === 1, 'T1. the key tapped on a door: all four are sent to stack on it at once. Tapped again inside the window: all four fall in',
        `after the first tap, sent to stack: ${first} of ${N} · after the second: falling in ${f.length} of ${N}, on the door ${R.onStack(A).length}`); }
    // T2
    R.home();
    { R.onDoor(A); R.key(true); R.step(20); R.onDoor(B); R.key(true); R.until(() => R.set(R.onStack(A)) && R.set(R.onStack(B)), 60);
      const split = R.onStack(A).length === 2 && R.onStack(B).length === 2, t0 = CMDRING_STATS.twice, onA = R.onDoor(A), left0 = R.onStack(B).length; R.key(false); R.key(false); const f = fallen(), counted = CMDRING_STATS.twice - t0;
      const clear = R.until(() => D.ops.length === 0, 30), onDoors = R.onStack(A).length + R.onStack(B).length;
      line(split && onA && left0 === 2 && counted === 1 && f.length === N && onDoors === 0 && clear, 'T2. two pairs on two doors by two goals, the key tapped twice: all four fall in, nobody is left on a door, and no door tool is left running',
        `two and two first: ${split} · the cursor on the first door: ${onA} · second taps counted ${counted} · falling in ${f.length} of ${N} · on a door ${onDoors} · door tools running ${D.ops.length}`); }
    // T3
    R.home(); K.twice = true;
    { const t0 = CMDRING_STATS.twice, onA = R.onDoor(A); R.key(false); R.step(Math.ceil((K.twiceSecs + 0.2) * 60)); R.key(false); const f = fallen();
      line(onA && f.length === 0 && R.onStack(A).length === N && CMDRING_STATS.twice === t0, 'T3. tapped again AFTER the window, on the same door: nobody falls in',
        `the second tap ${(K.twiceSecs + 0.2).toFixed(2)} s on (the window is ${K.twiceSecs} s) · falling in ${f.length} · on the door ${R.onStack(A).length} of ${N}`); }
    // T4 (the cursor's place on the second door found first, so the two taps are a quick hand's)
    R.home();
    { let pB: V | null = null; { const c = B.frameCentre, th = B.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
        for (const y of [c[1], 0.05, 1.0, 1.8]) { for (const k of [0, 0.35, -0.35]) { const p = [c[0] + nx * k, y, c[2] + nz * k]; W.cursorAt(p); R.step(2); const t = O.targetUnder(R.I, W.cam, W.canvas); if (t.kind === 'door' && t.door === B) { pB = p; break; } } if (pB) break; } }
      const t0 = CMDRING_STATS.twice, onA = R.onDoor(A), g0 = g.time; R.key(false); if (pB) W.cursorAt(pB); R.step(2); R.key(false); const dt = g.time - g0, f = fallen(), b = R.onStack(B);
      line(onA && !!pB && dt < K.twiceSecs && f.length === 0 && b.length === N && CMDRING_STATS.twice === t0, 'T4. tapped on one door and at once on the other: everyone is sent to the second door; nobody falls in',
        `the two taps ${dt.toFixed(2)} s apart · falling in ${f.length} · sent to the second door ${b.length} of ${N}`); }
    // T5
    R.home();
    { const t0 = CMDRING_STATS.twice, onA = R.onDoor(A); R.key(true); const n1 = R.onStack(A).length; R.key(false); R.step(4); const f = fallen(), a = R.onStack(A);
      line(onA && n1 > 0 && f.length === 0 && a.length === N && CMDRING_STATS.twice === t0, 'T5. a tap with Shift and at once a plain tap on the same door: the plain key\'s own order, everyone\'s; nobody falls in',
        `after the Shift tap on the door: ${n1} · after the plain tap: on the door ${a.length} of ${N}, falling in ${f.length}`); }
    // T6
    R.home(); K.twice = false;
    { const onA = R.onDoor(A); R.key(false); R.key(false); const f = fallen();
      line(onA && f.length === 0 && R.onStack(A).length === N, 'T6. CONTROL — the switch off: tapped twice on a door, nobody falls in', `falling in ${f.length} · on the door ${R.onStack(A).length} of ${N}`); }
    // T7
    { K.twice = true; const rails = g.rails, plain = (O as any).twiceOn();
      g.rails = { live: true, locks: (a: string) => a !== 'order' }; const untaught = (O as any).twiceOn();
      g.rails = { live: true, locks: (a: string) => a !== 'order' && a !== 'twice' }; const taught = (O as any).twiceOn(); g.rails = rails;
      const listed = (RAILS_ACTIONS as string[]).includes('twice');   // (… and a step may list it: the lesson lint knows the action)
      line(plain === true && untaught === false && taught === true && listed, 'T7. on a tutorial mission the second tap is \'fall in\' only from a step that lists `twice`', `here: ${plain} · a mission no step of which lists it: ${untaught} · one that does: ${taught} · the lesson lint knows \`twice\`: ${listed}`); }
    // T8, T9 (each half of 'the same place' asked with the other left out)
    { K.twice = !TBREAK; const tap2 = (off: number): { n: number; f: number; d: number; px: number; mv: number } | null => { R.home(); const p = R.onSpot(); if (!p) return null; const t0 = CMDRING_STATS.twice, x0 = R.I.mouseX, y0 = R.I.mouseY; R.key(false);
        const q = [p[0] + off, 0.04, p[2]]; W.cursorAt(q); R.step(); const t = O.targetUnder(R.I, W.cam, W.canvas); const px = Math.hypot(R.I.mouseX - x0, R.I.mouseY - y0); R.key(false); return { n: CMDRING_STATS.twice - t0, f: fallen().length, d: flat(t.at, p), px, mv: O.men('all').filter((m: any) => O.of(m)?.order === 'move').length }; };
      const near = tap2(1.0);
      line(!!near && near.d > 0.5 && near.px > K.twicePx && near.n === 0 && near.f === 0 && near.mv === N, 'T8. a spot, then a spot a pace off inside the window (the hand moved): the second spot\'s own order, and nobody falls in',
        near ? `a pace off (${near.d.toFixed(2)} m, the cursor ${near.px.toFixed(0)} px on, the slack is ${K.twicePx}): counted ${near.n}, falling in ${near.f}, sent to the spot ${near.mv} of ${N}` : 'no spot');
      const still = tap2(0);
      line(!!still && still.px <= K.twicePx && still.n === 1 && still.f === N, 'T9. a hand that has not moved: a spot tapped twice is \'fall in\'', still ? `the cursor ${still.px.toFixed(0)} px on · counted ${still.n}, falling in ${still.f} of ${N}` : 'no spot'); }
    // T10
    R.home();
    { const t0 = CMDRING_STATS.twice, onA = R.onDoor(A); R.key(false); const a1 = R.onStack(A).length; R.key(false); const f2 = fallen().length; R.key(false); const a3 = R.onStack(A).length, f3 = fallen().length;
      line(onA && a1 === N && f2 === N && a3 === N && f3 === 0 && CMDRING_STATS.twice - t0 === 1, 'T10. three taps on a door: stack, fall in, stack — the third starts again', `on the door after the first ${a1} · falling in after the second ${f2} · on the door after the third ${a3} · second taps counted ${CMDRING_STATS.twice - t0}`); }
    // T11
    R.home();
    { W.cursorAt(g.player.char.pos); R.step(2); const t = O.targetUnder(R.I, W.cam, W.canvas), t0 = CMDRING_STATS.twice; R.key(false); const n1 = O.log.length, f1 = fallen().length; R.key(false); const n2 = O.log.length;
      line(t.kind === 'none' && f1 === N && n2 === n1 && fallen().length === N && CMDRING_STATS.twice - t0 === 1, 'T11. on himself twice: the first tap is \'fall in\', and the second tells them nothing more', `the cursor on himself: ${t.kind === 'none'} · falling in after the first ${f1} of ${N} · orders the second added ${n2 - n1} · second taps counted ${CMDRING_STATS.twice - t0}`); }
    // T12
    R.home(); K.twice = true;
    { const t0 = CMDRING_STATS.twice, onA = R.onDoor(A), g0 = g.time; R.key(false); R.I.pressed.add('Thawed'); R.step(); R.onDoor(A); const dt = g.time - g0; R.key(false); const f = fallen();
      line(onA && dt < K.twiceSecs && f.length === 0 && R.onStack(A).length === N && CMDRING_STATS.twice === t0, 'T12. a pause between the two taps: the tap after it is a first tap, and all four stack', `the two taps ${dt.toFixed(2)} s of game time apart · falling in ${f.length} · on the door ${R.onStack(A).length} of ${N}`); }
    // P
    { K.goalHalf = true; K.twice = true; const snap = JSON.parse(JSON.stringify(knobsSnapshot(g))), old = JSON.parse(JSON.stringify(snap)); delete old.tables.CMDRING.goalHalf; delete old.tables.CMDRING.twice;
      applyKnobs(g, old); const before = K.goalHalf || K.twice; applyKnobs(g, snap); const on = K.goalHalf && K.twice;
      const shipped = String(process.env.TPS_PROFILE ?? '') === 'shipped', rowH = FROZEN_HOLDS['CMDRING.goalHalf'], rowT = FROZEN_HOLDS['CMDRING.twice'], sH = !!stoodCmd?.goalHalf, sT = !!stoodCmd?.twice;
      line(before === false && on === true && sH === shipped && sT === shipped && /TPS_GOALHALF=1/.test(rowH ?? '') && /TPS_QTWICE=1/.test(rowT ?? ''), 'P. CONTROL (plumbing) — a recording from before the keys replays with both off; the stands and the FROZEN_HOLDS rows',
        `a file without the keys: ${before} · one with them on: ${on} · this stand (${shipped ? 'the page\'s' : 'frozen'}) stood them ${sH} and ${sT} · the rows: ${rowH ?? 'none'} · ${rowT ?? 'none'}`); }

    // H5 (a rig of its own: one teammate)
    { const R1 = await rig(true, { goalHalf: !HBREAK, twice: !TBREAK }, 'squad=1'), O1 = R1.O, A1 = R1.door('guard'), n = O1.men('all').length;
      R1.home(); const onA = !!A1 && R1.onDoor(A1); R1.key(true); const a = A1 ? R1.onStack(A1) : [];
      line(n === 1 && onA && a.length === 1, 'H5. one teammate: Shift + the key on a door is his', `teammates ${n} · the cursor on the door: ${onA} · sent to stack on it: ${names(a)}`); }
    // H7, H8 (rigs of their own: two teammates, then three)
    for (const n of [2, 3]) { const R2 = await rig(true, { goalHalf: !HBREAK, twice: !TBREAK }, `squad=${n}`), O2 = R2.O, A2 = R2.door('guard'), B2 = R2.door('guard_side'), all: any[] = O2.men('all');
      R2.home(); const onA = !!A2 && R2.onDoor(A2), parts: any[][] = n === 2 ? all.map((m: any) => [m]) : O2.pairs().map((id: any) => all.filter((m: any) => O2.pairOf(m) === id)).filter((l: any[]) => l.length > 0);
      // (… and whichever part a point is nearer is the part the rule names for it: asked at each part's own feet, before anything is given)
      const feet = (l: any[]): V => [l.reduce((x: number, m: any) => x + m.char.pos[0], 0) / l.length, 0, l.reduce((x: number, m: any) => x + m.char.pos[2], 0) / l.length], each = parts.map(l => names(PM.goalMen(O2, { kind: 'floor', at: feet(l) })?.men ?? []));
      const near0 = A2 ? [...parts].sort((x, y) => far(x, A2) - far(y, A2))[0] : []; R2.key(true); const a = A2 ? R2.onStack(A2) : [], rest = all.filter((m: any) => !a.includes(m)), kept = rest.every((m: any) => { const q = O2.of(m); return !q || q.post === null; });
      R2.step(20); const onB = !!B2 && R2.onDoor(B2); R2.key(true); const b = B2 ? R2.onStack(B2) : [], a2 = A2 ? R2.onStack(A2) : [];
      line(all.length === n && onA && onB && a.length > 0 && a.length < n && names(a) === names(near0) && each.every((x, i) => x === names(parts[i])) && kept && names(b) === names(rest) && names(a2) === names(a),
        n === 2 ? 'H7. two teammates: Shift + the key on a door sends the nearer man alone, and the other is still with him; on the second door, the other' : 'H8. three teammates: Shift + the key on a door sends the nearer of the whole pair and the odd man; on the second door, the rest',
        `teammates ${all.length} · sent to the first door: ${names(a)} · the nearer was ${names(near0)} (of ${parts.map(names).join(' | ')}) · asked at each part's feet: ${each.join(' | ')} · the rest still with him: ${kept} · sent to the second: ${names(b)} · still on the first: ${names(a2)}`); }
  } finally { knobsBack(knobs0); RAILS.on = rails0; }

  console.log(fails ? `=== halftwice-check: ${fails} FAIL ===` : '=== halftwice-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await halftwiceCheck()) ? 0 : 1);
