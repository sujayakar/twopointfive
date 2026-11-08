// THE SLOT IN HAND, PRESSED AGAIN, OPENS ITS SETUP — the check behind SLOTSETUP.on (src/game/slotsetup.ts).
//   bun run tools/qa/probes.ts slotsetup                       (family `kit`)
//   SLOTSETUP_BREAK=1 bun run tools/qa/probes.ts slotsetup     the page's words for the presses as the rule off had them: the words' line must FAIL (the rule is the Rust game's: rust/game_start/tests/kit_view.rs)
// EVERY LINE IS ASKED THROUGH THE INPUT: a digit, the back key or a click in a tick, the rifle staged in his hands as play stages it (fn2000-probes.ts).
//   opens        2 with the rifle up: the setup is up, the launcher's load has not moved, the back key has something to back out of
//   the rows     2 1 the selector goes over and the setup shuts · 2 2 the next load · 2 3 the light
//   shuts        the back key (taken: nothing after it reads it) · the trigger (and the round goes) · a digit that is no row (4: the stun can is in his fist)
//                · when the gun leaves his hands · when Space goes down for its list
//   the loads    a load he is out of is stepped over; with nothing else to put up the row has no digit
//   Esc, words   Game.escape shuts it (the pause is for nothing to back out of) · the lines that name the launcher's presses follow the rule
//   the double tap   2 2 from the sling is the rifle and its setup at once, and a row's digit after them is the row
//   his own draw     2 2 off the pistol is the rifle and its setup · a round that goes with the setup up shuts it (a click the kit swap kept) · a gun the
//                game handed him and ONE press of its key is no setup
//   the pistol   1 again: its setup; 3 its light
// SINCE 2026-11-07 the rule and its three keys (pop, draw; stay off) are settled and nothing here writes them, nor the back key's. Out: 'with `stay` a
// pick leaves the setup up'; every `pop`-off and `draw`-off half (the clock that shut it, the box's second line, the double tap as the draw alone, the
// settling time at 0); 'a can's digit pressed again is as it was' (it was staged only on the draw-off game); the words and the column 'with the rule
// off'; 'off: the build as it was'; the recording from before the key, the frozen rows and the three address words.
//   the board    host/keymap.ts with `setFree`: Z and L give the game nothing, each row can be put on a key and then gives its code; without it, as built
import { ROOT } from './headless.ts';
type V = [number, number, number];
const DT = 1 / 60;
export async function slotSetupCheck(): Promise<boolean> {
  const BRK = process.env.SLOTSETUP_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== slotsetup: the slot in hand pressed again opens its setup${BRK ? ' (the rule dead: must fail)' : ''} ===`);
  const { SLOTSETUP, loadKeys, lightKeys } = await import(`${ROOT}/src/game/slotsetup.ts`);
    const { KITANIM } = await import(`${ROOT}/src/game/kitanim.ts`);
  const { stage, hudText } = await import('./fn2000-probes.ts');
  const was = { secs: SLOTSETUP.secs, settle: SLOTSETUP.settle }, log = console.log, err = console.error, warn = console.warn;
  const quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = console.error = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const AHEAD: V = [35.5, 1.2, 11.5];
  const key = (W: any, code: string, n = 1) => { W.input.pressed.add(code); for (let i = 0; i < n; i++) { W.cursorAt(AHEAD); W.step(DT); } };
  const wait = (W: any, secs: number) => { for (let i = 0; i < Math.round(secs * 60); i++) { W.cursorAt(AHEAD); W.step(DT); } };
  /** the rifle in his hands, off the sling and settled */
  const rifle = async (seed = 4101) => quiet(async () => { SLOTSETUP.secs = was.secs; SLOTSETUP.settle = was.settle; const W: any = await stage(seed);
    const g = W.game, pl = g.player; g.aiEnabled = false;
    for (let i = 0; i < 90 && (pl.equipT > 0 || !pl.rifleUp); i++) wait(W, DT); wait(W, 0.5); return { W, g, pl, rf: pl.rifle }; });
  const up = (g: any): boolean => !!g.view.kit.setup;
  /** the rows of the setup that is up, by their keys: 1 the selector, 2 the load, 3 the light */
  const rowsUp = (g: any): string => (g.view.kit.setup?.rows ?? []).map((r: any) => r.key.replace('Digit', '')).join(' ');
  try {
    // ---- opens
    { const { W, g, pl, rf } = await rifle(); const a0 = rf.attachment, held = pl.rifleUp, able0 = g.backable();
      await quiet(() => key(W, 'Digit2')); const open = up(g), a1 = rf.attachment, able = g.backable(), slot = pl.slot, rows = rowsUp(g);
      verdict(held && !able0 && open && a1 === a0 && able && slot === 'rifle' && /1/.test(rows) && /3/.test(rows), '2 with the rifle in his hands opens its setup: the load on the launcher has not moved, and the back key has something to back out of',
        `rifle up ${held} · setup up ${open} · load ${a0} → ${a1} · backable before ${able0}, after ${able} · rows: ${rows}`); }
    // ---- the rows: the same two presses each time
    { const { W, g, pl, rf } = await rifle(); const m0 = rf.mode, l0 = rf.lightOn;
      await quiet(() => { key(W, 'Digit2'); key(W, 'Digit1'); }); const m1 = rf.mode, shut1 = !up(g), slot1 = pl.slot;
      await quiet(() => { key(W, 'Digit2'); key(W, 'Digit3'); }); const l1 = rf.lightOn, shut3 = !up(g), slot3 = pl.slot, cans = pl.canisters;
      verdict(m1 !== m0 && shut1 && slot1 === 'rifle' && l1 !== l0 && shut3 && slot3 === 'rifle', '2 1 puts the selector over, 2 3 the light on; each pick shuts the setup and the rifle stays in his hands',
        `selector ${m0} → ${m1} (shut ${shut1}, slot ${slot1}) · light ${l0} → ${l1} (shut ${shut3}, slot ${slot3}, smoke still ${cans})`); }
    // ---- shuts
    { const A = await rifle(); await quiet(() => key(A.W, 'Digit2')); const openA = up(A.g); await quiet(() => key(A.W, 'Tab')); const shutA = !up(A.g), slotA = A.pl.slot, ableA = A.g.backable();
      const B = await rifle(); await quiet(() => key(B.W, 'Digit2')); const r0 = B.rf.roundsReady; await quiet(() => { B.W.cursorAt(AHEAD); B.W.input.clicked |= 1; B.W.input.buttons |= 1; B.W.step(DT); B.W.input.buttons &= ~1; wait(B.W, 0.1); }); const shutB = !up(B.g), fired = r0 - B.rf.roundsReady;
      const C = await rifle(); await quiet(() => { key(C.W, 'Digit2'); key(C.W, 'Digit4'); }); const shutC = !up(C.g), slotC = C.pl.slot;
      const F = await rifle(); await quiet(() => key(F.W, 'Digit2')); await quiet(() => wait(F.W, 6)); const stillF = up(F.g), n0 = F.rf.mode; await quiet(() => key(F.W, 'Digit1', 3)); const slotF = F.pl.slot, n1 = F.rf.mode, shutF = !up(F.g);   // (the mode: six seconds on it is up, and 1 is still its row)
      const E = await rifle(); await quiet(() => key(E.W, 'Digit2')); const openE = up(E.g); await quiet(() => key(E.W, 'KeyE', 2)); const shutE = openE && !up(E.g) && E.pl.setup === null, slung = E.pl.holstered;
      verdict(openA && shutA && !ableA && slotA === 'rifle' && shutB && fired === 1 && shutC && slotC === 'flash' && stillF && slotF === 'rifle' && n1 !== n0 && shutF && shutE && slung,
        'the setup shuts on the back key, on the trigger (the round goes), on a digit that is no row (4: the stun can comes to his fist) and when the gun leaves his hands; no clock runs on it (six seconds on, 1 is still the selector\'s row)',
        `Tab: up ${openA} → shut ${shutA}, anything left to back out of ${ableA}, slot ${slotA} · click: shut ${shutB}, rounds gone ${fired} · 4: shut ${shutC}, slot ${slotC} · six seconds on: up ${stillF}, then 1 → slot ${slotF}, selector ${n0} → ${n1}, shut ${shutF} · slung with it up: shut ${shutE}, slung ${slung}`); }
    // ---- another list's digits shut it
    {
      const B = await rifle(); await quiet(() => key(B.W, 'Digit2')); const openB = up(B.g); await quiet(() => { B.W.input.keys.add('Space'); key(B.W, 'Space', 3); }); const down = B.pl.sel.downT >= 0 || B.pl.sel.open, shutB = down && B.pl.setup === null; B.W.input.keys.delete('Space');   // (Space going down: the list's own keys from here)
      verdict(openB && shutB, 'Space going down for its list shuts the setup', `setup was ${openB}, the list\'s key held ${down}, gone ${shutB}`); }
    // ---- Esc where the back key's rule is off, and the words that name the presses
    { const A = await rifle(); await quiet(() => key(A.W, 'Digit2')); const open = up(A.g), esc = A.g.escape(), gone = !up(A.g) && A.pl.setup === null, again = A.g.escape(), on = `${loadKeys()} · ${lightKeys()}`;
      verdict(open && esc && gone && !again && on === 'press 2, then 2 · its gun\'s key again, then 3',
        'Game.escape shuts an open setup and says so (Esc pauses only with nothing to back out of); the words name the presses as they are; the setup\'s own digits (the gun\'s, and a row\'s while it is up) are the ones the page does not let the carried aim go for',
        `up ${open} · escape() ${esc}, gone ${gone}, a second escape() ${again} · the words: '${on}'`); }
    // ---- the double tap off another slot: the draw and the setup (`draw`)
    {
      const D: any = await quiet(() => stage(4102, false)); const dg = D.game, dp = dg.player; dg.aiEnabled = false; const b0 = dp.rifle.attachment, n0 = dp.rifle.mode;
      await quiet(() => { key(D, 'Digit2', 3); key(D, 'Digit2', 2); }); const midD = dp.equipT > 0, openD = up(dg), b1 = dp.rifle.attachment; await quiet(() => key(D, 'Digit1', 2)); const n1 = dp.rifle.mode, slotD = dp.slot, shutD = !up(dg);   // (2 2 1, a twentieth of a second apart: the rifle, on the other position of its selector)
      verdict(midD && openD && b1 === b0 && n1 !== n0 && slotD === 'rifle' && shutD,
        'a double-tapped 2 from the sling is the rifle and its setup at once, the load not moved, and a 1 straight after is the selector\'s row (the rifle stays in his hands)',
        `second 2 came mid-draw ${midD} · setup up ${openD} · load ${b0} → ${b1} · then 1: selector ${n0} → ${n1}, slot ${slotD}, shut ${shutD}`); }
    // ---- the pistol
    { const W: any = await quiet(() => stage(4103, false)); const g = W.game, pl = g.player; g.aiEnabled = false; const gun = pl.pistol;
      await quiet(() => { key(W, 'Digit3', 30); key(W, 'Digit1', 2); }); const inHand = !!pl.gunInHand();   // (1 off a can: the draw)
      await quiet(() => wait(W, 0.7)); const drawn = pl.pistolUp, shut0 = !up(g), l0 = gun.lightOn; await quiet(() => key(W, 'Digit1')); const open = up(g), rows = rowsUp(g);
      await quiet(() => key(W, 'Digit3')); const l1 = gun.lightOn, slot = pl.slot, shut = !up(g);
      const P: any = await quiet(() => stage(4103, false)); await quiet(() => { key(P, 'Digit3', 30); key(P, 'Digit1', 2); key(P, 'Digit1', 2); }); const tapP = up(P.game) && P.game.player.setup?.gun === 'pistol';   // (the same two taps with `draw` on: the pistol and its setup)
      verdict(drawn && inHand && tapP && shut0 && open && /3/.test(rows) && l1 !== l0 && slot === 'pistol' && shut, '1 draws the pistol; a double-tapped 1 is the pistol and its setup; 1 again once it has settled opens its setup; 3 there is its light, and the pistol stays in his hand', `1 off a can: in his hand ${inHand} · 1 1 off a can: the pistol and its setup ${tapP} · drawn ${drawn} (no setup yet ${shut0}) · 1 again: up ${open} · rows: ${rows} · light ${l0} → ${l1} · slot ${slot} · shut ${shut}`); }
    // ---- the double tap is his own draw and no other time his hands are busy; a round that goes shuts the setup
    { const W: any = await quiet(() => stage(4110, false)); const g = W.game, pl = g.player; g.aiEnabled = false;
      await quiet(() => key(W, 'Digit1', 90)); const from = pl.slot; await quiet(() => { key(W, 'Digit2', 2); key(W, 'Digit2', 2); }); const his = up(g) && pl.setup?.gun === 'rifle', swap = pl.equipT;   // (the pistol up and settled, then 2 2 a thirtieth of a second apart: the rifle and its setup)
      const A: any = await quiet(() => stage(4110, false)); const ag = A.game, ap = ag.player; ag.aiEnabled = false;
      await quiet(() => { key(A, 'Digit1', 90); key(A, 'Digit2'); let n = 0; while (ap.equipT > KITANIM.clickKeep - 0.02 && n++ < 200) wait(A, DT); }); const r0 = ap.rifle.roundsReady;
      await quiet(() => { A.cursorAt(AHEAD); A.input.clicked |= 1; A.input.buttons |= 1; A.step(DT); A.input.buttons &= ~1; key(A, 'Digit2'); }); const openA = up(ag); await quiet(() => wait(A, 0.7)); const gone = r0 - ap.rifle.roundsReady, leftUp = up(ag);   // (2, a click the swap keeps for its end, 2: the round goes with the setup up — and shuts it)
      const B: any = await quiet(() => stage(4111, false)); const bg = B.game, bp = bg.player; bg.aiEnabled = false; bp.canisters = 1;
      await quiet(() => { key(B, 'Digit3', 60); B.cursorAt(AHEAD); B.input.clicked |= 1; B.input.buttons |= 1; B.step(DT); B.input.buttons &= ~1; let n = 0; while (bp.slot !== 'pistol' && n++ < 400) wait(B, DT); wait(B, 3 * DT); });
      const handed = bp.slot; await quiet(() => key(B, 'Digit1')); const one = up(bg) || bp.setup !== null;   // (the last smoke thrown hands him the pistol; ONE press of 1 in the swing is not a double tap)
      const held = async (tap = false) => { const { W: H, g: hg, pl: hp } = await rifle(); hg.aiEnabled = false; if (hp.rifle.mode !== 'auto') await quiet(() => key(H, 'KeyZ'));   // (the trigger held on auto, then 2; `tap`: 2 again on the very frame a round shuts it)
        return quiet(() => { H.input.buttons |= 1; H.input.clicked |= 1; wait(H, 0.3); const h0 = hp.rifle.roundsReady, l0 = hp.rifle.attachment; key(H, 'Digit2'); const open = up(hg), t0 = hg.time; for (let n = 0; n < 30 && hp.lastFireT < t0; n++) wait(H, DT);
          if (tap) key(H, 'Digit2'); else wait(H, 2 * DT); const out = { open, gone: h0 - hp.rifle.roundsReady, left: up(hg), l0, l1: hp.rifle.attachment }; H.input.buttons &= ~1; return out; }); };
      const hOn = await held(), hTap = await held(true);
      const later: any = await quiet(() => stage(4103, false)); later.game.aiEnabled = false;
      const lock = await quiet(() => { const lp = later.game.player; key(later, 'Digit3', 30); key(later, 'Digit1', 2); wait(later, 0.1); lp.equipT = Math.max(lp.equipT, 0.6); wait(later, 0.45); key(later, 'Digit1'); return up(later.game) || lp.setup !== null; });   // (his own draw of the pistol, then a later lock on his hands — as a wand stowed or a tool's tail is — and ONE press of 1 under it)
      verdict(from === 'pistol' && his && openA && gone > 0 && !leftUp && handed === 'pistol' && !one && hOn.open && hOn.gone > 0 && !hOn.left && hTap.open && !hTap.left && hTap.l1 === hTap.l0 && !lock,
        'with the pistol up, 2 pressed twice quickly is the rifle and its setup; a round that goes while the setup is up shuts it (a click the kit swap kept for its end, a trigger held — and 2 on that frame is nothing, not the old key of the load); a gun the game handed him, or a later lock on his hands, and one press of its key is no setup',
        `2 2 off the pistol: setup ${his}, the swap still had ${swap.toFixed(2)} s · 2, a kept click, 2: setup ${openA}, then rounds gone ${gone}, setup left up ${leftUp} · last smoke thrown → ${handed}, one press of 1 → setup ${one} · his own draw, a later lock, one press of 1 → setup ${lock} · trigger held on auto, 2: setup ${hOn.open}, then rounds gone ${hOn.gone}, left up ${hOn.left}; 2 again on the frame it shuts: left up ${hTap.left}, load ${hTap.l0} → ${hTap.l1}`); }
    // ---- the box and the pop-up (ui/handbox.ts) off the real overlay code, on the probes' stand-in page
    { const { W, g, pl } = await rifle(); const shut = hudText(g), caps = (t: string) => (t.match(/kbd(\.\w+)?\{[^}]*\}/g) ?? []).join(' ');
      await quiet(() => key(W, 'Digit2')); const open = hudText(g); await quiet(() => key(W, 'Tab')); pl.setup = null;
      pl.holstered = true; const away = hudText(g); pl.holstered = false;
      const rT = pl.rifle.reloadT; pl.rifle.reloadT = 0.5; const re = hudText(g); pl.rifle.reloadT = rT;   // (a reload is written across the pips: what is set is still on the line)
      const lit = (t: string) => /div\.hb\.setup/.test(t), panel = (t: string) => /div\.hp/.test(t), words = (t: string) => /span\.hb-st\(span\.o\{(SEMI|AUTO)\}/.test(t);
      verdict(!panel(shut) && !/kbd/.test(shut) && words(shut) && !lit(shut) && panel(open) && lit(open) && /div\.hp\(.*kbd\{1\}.*kbd\{3\}.*kbd\.x\{(TAB|ESC)\}/.test(open) && !/kbd\{[6-9]\}/.test(open) && words(open)
        && !panel(away) && /div\.hb\.(holstered|empty)/.test(away) && /span\.hb-mag\.re\(.*span\.re\{RELOADING\}/.test(re) && words(re)
        && /div\.strip/.test(shut),
        'the kit column is one box that says what is set in plain words (through a reload too) and draws no key; the setup is a panel of its own, there only while it is up: a cap before each row, the back key\'s cap in its corner, no cap above 5',
        `shut: a panel ${panel(shut)}, caps '${caps(shut)}', what is set in words ${words(shut)} · up: a panel ${panel(open)}, caps ${caps(open)} (box marked ${lit(open)}) · put away: a panel ${panel(away)} · reloading: what is set in words ${words(re)} · a strip ${/div\.strip/.test(shut)}`); }
    // ---- a load: nothing is up after one
    { const { W, g, pl } = await rifle(); const S: any = g.saves; S.runId ||= 'slotsetup-check'; const { takeSnapshot } = await import(`${ROOT}/src/game/rollback.ts`);
      const file = await quiet(() => S.write(g, 'harness', 'before')); await quiet(() => wait(W, 1));   // (a file from a second ago, the setup shut)
      await quiet(() => key(W, 'Digit2')); const upAtSave = up(g), k = 'slotsetup-check-mem';
      S.shelf.set(k, { info: { key: k, slot: 'manual', n: 0, label: 'the setup up', kind: 'manual', savedAt: 0, missionT: 0, stage: g.mission.stage, runId: S.runId, bytes: 0, mem: true }, snap: takeSnapshot(g), pic: null });   // (a save kept in memory, as with a team on the floor, taken with the setup up)
      await quiet(() => wait(W, 0.3)); const whyMem = await quiet(() => S.load(g, k)), memUp = up(g) || pl.setup !== null; await quiet(() => key(W, 'Digit1', 3)); const m1 = pl.rifle.mode, slot = pl.slot;
      await quiet(() => { key(W, 'Digit2', 90); key(W, 'Digit2'); }); const upAgain = up(g), whyFile = file.ok ? await quiet(() => S.load(g, file.state)) : file.why, fileUp = up(g); await quiet(() => wait(W, DT)); const fileUp2 = up(g) || g.player.setup !== null;   // (a file's load builds the player again)
       }
    // ---- the board: Z and L give the game nothing until their rows are put on a key
    { const K = await import(`${ROOT}/src/host/keymap.ts`); const k0 = { ...K.KEYMAP }, kept = K.keysKept();
      try { K.KEYMAP.on = true; K.KEYMAP.setFree = !BRK; K.setKeys({});
        const z = K.gameCode('KeyZ'), l = K.gameCode('KeyL'), r = K.gameCode('KeyR'), capZ = K.capNow('KeyZ'), freed = K.keyFreed('KeyZ') && K.keyFreed('KeyL'), line = K.keyText('L is the weapon light');
        K.bindKey('KeyL', 'KeyO'); const o = K.gameCode('KeyO'), l2 = K.gameCode('KeyL'), capL = K.capNow('KeyL'); K.bindKey('KeyZ', 'KeyZ'); const z2 = K.gameCode('KeyZ');
        K.resetKeys(); const z3 = K.gameCode('KeyZ'); K.KEYMAP.setFree = false; const z4 = K.gameCode('KeyZ'), l4 = K.gameCode('KeyL');
        verdict(z === '' && l === '' && r === 'KeyR' && capZ === K.NONE && freed && line === 'L is the weapon light' && o === 'KeyL' && l2 === '' && capL === 'O' && z2 === 'KeyZ' && z3 === '' && z4 === 'KeyZ' && l4 === 'KeyL',
          'on the page\'s board Z and L give the game nothing and their rows read as having no key; a row put on a key gives its code (the light on O, the selector back on Z by hand); Reset frees them again; without the switch both are as built',
          `Z → '${z}', L → '${l}', R → '${r}' · Z's cap '${capZ}' · freed ${freed} · a line naming L unchanged ${line === 'L is the weapon light'} · light on O: O → '${o}', L → '${l2}', cap '${capL}' · Z put back by hand → '${z2}' · after Reset → '${z3}' · switch off: Z → '${z4}', L → '${l4}'`);
      } finally { Object.assign(K.KEYMAP, k0); K.setKeys(kept); } }
  } catch (x) { verdict(false, 'the check ran to its end', String((x as Error)?.stack ?? x).slice(0, 600)); }
  finally { SLOTSETUP.secs = was.secs; SLOTSETUP.settle = was.settle; console.log = log; console.error = err; console.warn = warn; }
  console.log(all ? '=== slotsetup: ALL PASS ===' : '=== slotsetup: FAIL ===');
  return all;
}
if (import.meta.main) process.exit((await slotSetupCheck()) ? 0 : 1);
