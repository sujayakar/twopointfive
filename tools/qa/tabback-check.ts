// ESC IS ONLY THE PAUSE; TAB BACKS OUT — the check behind BACKKEY.tab (src/game/backkey.ts; settled on, and not written here since 2026-11-07, nor is
// HANDRING.on, which this file held off: the lines read the Space list as the page has it).
//   bun run tools/qa/probes.ts tabback        (family `orders`)        TABBACK_BREAK=1 = another key is pressed where a line presses Tab: must FAIL
// OUT with the rule's off side: 'with the rule off Tab is nothing to it' of the Space list, the eased leaf, the held arc and the order menu, 'with it off
// the row is as it was' of the Options row, and the recording from before the key with the frozen stand's row.
// the maintainer, 10-17: "esc is overloaded -- exiting out of menus as well as going to pause. maybe we make tab exit out of menus and esc is always pause?"
// THE FOUR THINGS ESC BACKED OUT OF, each staged as play stages it and given a Tab in the tick:
//   the squad's order menu (X on the floor, a room clear with four) · the open Space list (Space held at a shut door) · a running verb (the leaf being eased)
//   (the launcher's held arc is not asked: its rounds are staged on the copy, which is never sent).
// THEN THE PAUSE with the rule on, under stepFrame (host/loop.ts, the loop main.ts runs) with the card up for some frames and Space let go under it: the first
// tick after the card, and only it, holds the one-tick code 'Paused'; the Space list is shut and its row is not done; the order menu is still up. The same
// keys given to a fresh game tick by tick (what a recording holds and a replay does) end the same way.
// A headless run cannot press the page's keys: main.ts's lines (Esc's press taken out of the frame, no back-out on it, Tab's trial waiting its turn) are held
// by their source text here, and looked at on the real page apart.
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
type V = [number, number, number];
const DT = 1 / 60, IK = 'Space';
export async function tabBackCheck(): Promise<boolean> {
  const BRK = process.env.TABBACK_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  const TAB = BRK ? 'KeyZ' : 'Tab';   // (the red path: a key that backs out of nothing)
  console.log(`\n=== tabback: Esc is only the pause, Tab backs out${BRK ? ' (TABBACK_BREAK: another key where Tab is pressed — must fail)' : ''} ===`);
  const { backName } = await import(`${ROOT}/src/game/backkey.ts`); const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { stage, rmbHold, rmbUp } = await import('./fn2000-probes.ts');
  const log = console.log, err = console.error, warn = console.warn;
  const quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = console.error = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const key = (W: any, code: string, n = 1) => { W.input.pressed.add(code); for (let i = 0; i < n; i++) W.step(DT); };
  try {
    // ---- the Space list and the leaf being eased: the office, a shut door, everybody parked
    const door = async () => quiet(async () => { const W: any = await standUp({ seed: 31 }); const g = W.game; g.godMode = true; g.aiEnabled = false; g.playerInvisible = true;
      const d = g.doors.byName('breakroom_n'); d.reset(); g.teleportPlayer([30.6, 0, 11.45]); g.player.crouch = false; g.player.char.bodyYaw = 0; W.step(DT); W.step(DT); return { W, g, d, pl: g.player }; });
    const list = async (code: string) => { const { W, g, d, pl } = await door();
      return quiet(() => { W.input.keys.add(IK); W.input.pressed.add(IK); for (let i = 0; i < 30 && !pl.sel.open; i++) W.step(DT); const open = !!pl.sel.open, a0 = d.angle;
        key(W, code); const shut = !pl.sel.open; W.input.keys.delete(IK); for (let i = 0; i < 30; i++) W.step(DT);
        return { open, shut, did: Math.abs(d.angle - a0) > 0.02 || !!pl.easing || !!pl.picking, g, W }; }); };
    const ease = async (code: string) => { const { W, g, d, pl } = await door();
      return quiet(() => { key(W, 'ArrowDown'); const hv = g.hover?.verb; key(W, IK, 12); const easing = pl.easing === d; key(W, code, 2); return { hv, easing, stopped: !pl.easing }; }); };
    { const on = await list(TAB), esc = await list('Escape');
      verdict(on.open && on.shut && !on.did && esc.open && esc.shut && !esc.did, 'the open Space list: Tab backs out of it and the press is spent (letting go does nothing); an Escape that reaches the tick still backs out',
        `Tab: open ${on.open} → shut ${on.shut}, anything done on letting go ${on.did} · Escape in the tick: shut ${esc.shut}, done ${esc.did}`); }
    { const on = await ease(TAB);
      verdict(on.easing && on.stopped, 'a running verb (the leaf being eased): Tab stops it where it is', `${on.hv} running ${on.easing} → stopped ${on.stopped}`); }


    // ---- the squad's order menu: a room clear with four, X on the floor ahead
    const squad = async () => quiet(async () => { const W: any = await standUp({ seed: 33 }); const g = W.game; g.godMode = true;
      const why = startFromQuery(g, new URLSearchParams('roomclear&squad4')).why; for (let i = 0; i < 20; i++) { g.player.visibility = 0.05; W.step(DT); }
      const p = g.player.char.pos, at: V = [p[0] + 1.5, 0, p[2] + 0.5]; W.cursorAt(at); key(W, ORDERS.key); W.cursorAt(at); W.step(DT);
      return { W, g, O: g.orders, why, at }; });
    { const A = await squad(), hintOn = String(A.O.hud().menu?.hint ?? ''), upA = !!A.O.menu, ableA = A.g.backable(); await quiet(() => { A.W.cursorAt(A.at); key(A.W, TAB, 2); }); const goneA = !A.O.menu, ableAfter = A.g.backable();
      verdict(A.why === null && upA && goneA && ableA && !ableAfter && /\btab\b/.test(hintOn) && !/\besc\b/.test(hintOn),
        'the order menu: Tab closes it and its line names Tab, not Esc', `up ${upA} → gone on Tab ${goneA} · something to back out of before ${ableA}, after ${ableAfter} · "${hintOn}"`); }

    // ---- the pause: the first tick after the card holds 'Paused'; a held list and a held arc are let go unsent in it; the order menu stays
    { const { stepFrame } = await import(`${ROOT}/src/host/loop.ts`);
      const under = (W: any, st: { frozen: boolean; seen: number; esc: number }) => { const g = W.game, up = g.update;
        g.update = function (this: any, dt: number, input: any, ...a: unknown[]) { if (input.pressed.has('Paused')) st.seen++; if (input.pressed.has('Escape')) st.esc++; return up.call(this, dt, input, ...a); };
        return { game: g, input: W.input, cam: W.cam, view: W.canvas, host: W.engine, render: () => { W.engine.world.upload(); }, betweenTicks: () => { W.engine.world.relist(); },
          smoke: null, audio: null, attract: null, hud: null, bubbles: null, time: 0, updateMs: 0, frozen: () => st.frozen, present: () => false, fixedStep: () => false }; };
      const A = await squad(), up0 = !!A.O.menu, stA = { frozen: false, seen: 0, esc: 0 };
      await quiet(() => { const ctx = under(A.W, stA); A.W.cursorAt(A.at); stepFrame(ctx, DT); stA.frozen = true; for (let i = 0; i < 3; i++) stepFrame(ctx, DT); stA.frozen = false; for (let i = 0; i < 3; i++) { A.W.cursorAt(A.at); stepFrame(ctx, DT); } }); const up1 = !!A.O.menu;
      const stL = { frozen: false, seen: 0, esc: 0 };
      const L = await quiet(async () => { const { W, d, pl } = await door(); W.input.keys.add(IK); W.input.pressed.add(IK); for (let i = 0; i < 30 && !pl.sel.open; i++) W.step(DT); const open = !!pl.sel.open, a0 = d.angle;
        const ctx = under(W, stL); stepFrame(ctx, DT); stL.frozen = true; stepFrame(ctx, DT); W.input.keys.delete(IK); stepFrame(ctx, DT); stepFrame(ctx, DT); const under0 = !!pl.sel.open; stL.frozen = false; stepFrame(ctx, DT); const shut = !pl.sel.open;
        for (let i = 0; i < 30; i++) stepFrame(ctx, DT); return { open, under0, shut, did: Math.abs(d.angle - a0) > 0.02 || !!pl.easing || !!pl.picking }; });
      const T = await quiet(async () => { const { W, d, pl } = await door(); W.input.keys.add(IK); W.input.pressed.add(IK); for (let i = 0; i < 30 && !pl.sel.open; i++) W.step(DT); const open = !!pl.sel.open, a0 = d.angle;
        W.input.keys.delete(IK); W.input.pressed.add('Paused'); W.step(DT); const shut = !pl.sel.open; for (let i = 0; i < 30; i++) W.step(DT); return { open, shut, did: Math.abs(d.angle - a0) > 0.02 || !!pl.easing || !!pl.picking }; });
      const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), never = !/game\.escape\(\)/.test(main), takes = /input\.pressed\.delete\('Escape'\); menu\.toggle\(\);/.test(main), trial = /input\.hit\('Tab'\) && !\(!frozen && game\.backable\(\)\)/.test(main);
      verdict(up0 && up1 && stA.seen === 1 && L.open && L.under0 && L.shut && !L.did && stL.seen === 1 && stL.esc === 0 && T.open && T.shut && !T.did && never && takes && trial && backName() === 'Tab',
        'the pause is the pause and nothing else: the first tick after the card holds \'Paused\' once, a Space list held as the card came up is shut with its row not done (and the same keys given tick by tick, as a replay gives them, end the same), the order menu is still up, and main.ts takes Esc\'s press out of the frame and never backs out on it',
        `order menu up ${up0} → after the card ${up1}, ticks holding 'Paused' ${stA.seen} · Space list open ${L.open}, still open under the card ${L.under0} → shut on the first tick back ${L.shut}, anything done ${L.did}, ticks holding 'Paused' ${stL.seen} · the same keys tick by tick: shut ${T.shut}, anything done ${T.did} · main.ts: no back-out on Esc ${never}, the press leaves the frame ${takes}, Tab's trial waits its turn ${trial} · the hint rows name ${backName()}`); }

  } finally { console.log = log; console.error = err; console.warn = warn; }
  console.log(`=== tabback: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
