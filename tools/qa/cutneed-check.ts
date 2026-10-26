// WALL-CUT SHADERS WHEN NEEDED (src/render/cutSwap.ts WHEN NEEDED; ui/shaderrows.ts row `cutWhenNeeded`, ?cutwhenneeded=1, off by default). With the row on the
// second (cut-box) set of the tracing pipelines is not built at the start: it is asked for by the first cut piece a ray can meet, by a loaded save that has pieces in
// it, by the sign that a cut is coming (a round fired by anyone, a bang or a frag in the world, a wall charge being set), or by a tool that awaits its readiness.
// What is checked here is the ASKING: the rule over a stand-in owner (nothing compiles headless), and the sign, the first piece and a loaded save off the real game.
// What the row does to a load is a measurement on a Windows machine; that the page compiles no cut twin before the first piece is tools/qa/cutneed-page.cjs.
//   bun run tools/qa/cutneed-check.ts                         exit 0 = every line passed, 1 = one failed (passes on both stands: TPS_PROFILE=shipped in front)
//   CUTNEED_BREAK=1 bun run tools/qa/cutneed-check.ts         the row is left OFF where the lines turn it on (the set is built at the start, as it was): must FAIL
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';

export async function cutneedCheck(): Promise<boolean> {
  const { cutSwap, cutComing } = await import(`${ROOT}/src/render/cutSwap.ts`);
  const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`);
  const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
  const { PLAYER_ID } = await import(`${ROOT}/src/game/game.ts`);
  const cutMod = await import(`${ROOT}/src/scene/cut.ts`);
  const { makeBox, BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
  const BRK = process.env.CUTNEED_BREAK === '1', DT = 1 / 60; let all = true;
  const line = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== cutneed: wall-cut shaders when needed${BRK ? ' (the row left OFF: must fail)' : ''} ===`);
  const log = console.log; const quiet = <T,>(f: () => T): T => { console.log = () => { /* the swap's own lines */ }; try { return f(); } finally { console.log = log; } };
  const was = { on: cutConfig.on, swap: cutConfig.swap, destr: DESTRUCTION.on, row: cutSwap.whenNeeded, members: new Set(cutSwap.members) };
  cutConfig.on = true; cutConfig.swap = true;
  /** the swap as a page has it before its boot's pipelines are in, with the row as asked (BREAK: never on) and one stand-in owner that counts */
  let builds = 0, used = false;
  const owner = () => ({ buildCut: async () => { builds++; }, useCut: (on: boolean) => { used = on; } });
  const fresh = (row: boolean) => { cutSwap.members.clear(); cutSwap.state = 'none'; cutSwap.ready = null; cutSwap.live = false; cutSwap.force = null; cutSwap.idle = 0; cutSwap.error = ''; cutSwap.hotPinned = false; cutSwap.noRaster = false;
    cutSwap.armed = false; cutSwap.asked = ''; cutSwap.whenNeeded = row && !BRK; builds = 0; used = false; cutSwap.members.add(owner()); };
  const frame = (traced: number) => quiet(() => cutSwap.frame(traced));
  try {
    // ---- the rule, over a stand-in owner
    { fresh(false); quiet(() => cutSwap.start()); const st = cutSwap.state; await cutSwap.ready;
      line(builds === 1 && st === 'building' && cutSwap.state === 'ready' && !cutSwap.waiting && cutSwap.asked === '', 'the row off: the start builds the second set at once, as it was', `builds ${builds} · state at the start ${st}, then ${cutSwap.state}`); }
    { fresh(true); frame(3); const early = builds;
      quiet(() => cutSwap.start()); const held = builds === 0 && cutSwap.state === 'none' && cutSwap.waiting;
      line(early === 0 && held, 'the row on: the start builds nothing — and a piece seen before the start (the boot\'s own pipelines still compiling) asks for nothing', `builds before the start ${early}, after it ${builds} · state ${cutSwap.state} · waiting ${cutSwap.waiting}`);
      for (let i = 0; i < 600; i++) frame(0);
      line(builds === 0 && cutSwap.state === 'none' && !cutSwap.drawsCut, 'ten seconds of calm frames: still nothing built (a cut piece would be drawn and traced whole)', `builds ${builds} · state ${cutSwap.state} · drawsCut ${cutSwap.drawsCut}`);
      const late = owner(); let lateBuilt = 0; late.buildCut = async () => { lateBuilt++; }; cutSwap.add(late);   // (a pass whose pipelines arrived after the boot: the fog's)
      frame(2); const st = cutSwap.state, why = cutSwap.asked, whole = !cutSwap.live && !cutSwap.drawsCut; frame(2); frame(2);
      await cutSwap.ready; frame(2);
      line(builds === 1 && lateBuilt === 1 && st === 'building' && why === 'piece' && whole && cutSwap.state === 'ready' && cutSwap.live && used && cutSwap.drawsCut,
        'the first frame with a cut piece a ray can meet asks for the set, once, every owner in it; the piece is whole until the set is in, then the frame takes it', `asked '${why}' · builds ${builds} (+ a late owner's ${lateBuilt}) · ${st} → ${cutSwap.state} · live ${cutSwap.live} · drawsCut ${cutSwap.drawsCut}`); }
    { fresh(true); quiet(() => cutSwap.start()); const p = quiet(() => cutSwap.need('ready')), q = quiet(() => cutSwap.need('ready')); const ok = await p;
      fresh(false); const none = await cutSwap.need('ready'), noBuild = builds;
      line(ok === true && p === q && cutSwap.state === 'none' && none === false && noBuild === 0, 'a tool that awaits the set\'s readiness gets the set (one build, the same promise twice); with the row off and no build asked it is told false at once, as it was', `row on: resolved ${ok}, same promise ${p === q} · row off, nothing started: resolved ${none}, builds ${noBuild}`); }

    { fresh(true); quiet(() => cutSwap.start()); cutSwap.noRaster = true; frame(2); await quiet(() => cutSwap.need('sign')); const none = builds; cutSwap.noRaster = false;
      const warn = console.warn; console.warn = () => { /* hot()'s one line */ }; try { cutSwap.hot(); } finally { console.warn = warn; } frame(2); const pinned = builds; const ok = await quiet(() => cutSwap.need('ready'));
      line(none === 0 && pinned === 0 && ok === true && builds === 1, 'a page whose picture can only draw pieces whole, or whose boot programs a dev hook pinned, is not made to compile a set its frame would never take — a tool that awaits it still gets it', `builds with no cut raster ${none} · pinned ${pinned} · after the tool asked ${builds}`); }

    // ---- the sign, the first piece and a loaded save, off the real game
    DESTRUCTION.on = true;
    const W: any = await standUp(); const g = W.game; g.godMode = true; g.aiEnabled = false; g.tune.chaos = 0.5;
    const tick = (X: any = W) => { X.step(DT); frame(X.engine.world.cutTraced); if (cutSwap.waiting && cutComing(X.game)) void quiet(() => cutSwap.need('sign')); };   // (main.ts: the frame, then the sign)
    { fresh(true); quiet(() => cutSwap.start()); let signs = 0; for (let i = 0; i < 180; i++) { tick(); if (cutComing(g)) signs++; }
      line(signs === 0 && builds === 0 && cutSwap.waiting && W.engine.world.cutTraced === 0, 'three seconds of a calm mission: no sign, no piece, nothing built', `frames with the sign ${signs} · cut boxes a ray can meet ${W.engine.world.cutTraced} · builds ${builds}`); }
    { const c = g.player.char, at = [c.pos[0] + 3, 1.2, c.pos[2]]; fireWeapon(g, c, at, true, { spread: 0 }); const now = cutComing(g); tick();
      line(now && builds === 1 && cutSwap.asked === 'sign', 'a round out of the player\'s gun is the sign: the set is asked for on that frame', `sign ${now} · asked '${cutSwap.asked}' · builds ${builds}`);
      await cutSwap.ready; for (let i = 0; i < 40; i++) W.step(DT);
      fresh(true); quiet(() => cutSwap.start()); const gd = g.guards.find((x: any) => !x.char.dead); let theirs = false;
      if (gd) { fireWeapon(g, gd.char, [gd.char.pos[0] + 3, 1.2, gd.char.pos[2]], false, { steady: true, spread: 0 }); theirs = cutComing(g); tick(); }
      line(!!gd && theirs && cutSwap.asked === 'sign', '… and so is a round out of anybody else\'s', gd ? `sign ${theirs} · asked '${cutSwap.asked}'` : 'no guard on the level');
      await cutSwap.ready; for (let i = 0; i < 40; i++) W.step(DT); }
    { const calm = { events: [] as { kind: string; weapon?: unknown }[], items: { items: [] as { kind: string }[] }, shells: [] as unknown[], wallCharge: { ops: [] as unknown[], set: [] as unknown[] }, doorTools: { set: [] as unknown[] } };
      const say = (o: Partial<typeof calm>) => cutComing({ ...calm, ...o });
      const yes = [say({ events: [{ kind: 'bang' }] }), say({ items: { items: [{ kind: 'frag' }] } }), say({ items: { items: [{ kind: 'flash' }] } }), say({ wallCharge: { ops: [{}], set: [] } }), say({ wallCharge: { ops: [], set: [{}] } }), say({ doorTools: { set: [{}] } }), say({ shells: [{}] })];
      const no = [say({}), say({ events: [{ kind: 'step' }, { kind: 'door' }, { kind: 'shot' }] }), say({ items: { items: [{ kind: 'smoke' }, { kind: 'mag' }, { kind: 'shell' }] } })];
      line(yes.every(Boolean) && !no.some(Boolean), 'the other signs: a bang, a frag or a stun grenade in the world, a wall charge being set or standing, a charge on a door, a launcher\'s round in the air — and not a footstep, a door, a smoke can\'s pop or a dropped magazine', `signs ${yes.map(Number).join('')} · not signs ${no.map(Number).join('')}`); }
    let save: any = null;
    { fresh(true); quiet(() => cutSwap.start()); for (let i = 0; i < 3; i++) tick(); const before = builds;
      const walls = g.destr.walls.all().filter((w: any) => w.kind === 'wall' && Math.max(w.box.h[0], w.box.h[2]) >= 1.6); let blown = 0, at0 = -1, frames = 0;
      for (const w of walls) { const b = w.box, alongX = b.h[0] >= b.h[2]; const at = alongX ? [b.c[0], 0.3, b.c[2] + 0.3] : [b.c[0] + 0.3, 0.3, b.c[2]];
        const res = g.destr.blast(at, { who: PLAYER_ID }); if (!res.walls) continue; blown++;
        for (frames = 1; frames <= 240; frames++) { W.step(DT); frame(W.engine.world.cutTraced); if (at0 < 0 && W.engine.world.cutTraced > 0) at0 = frames; if (builds > before && frames > 30) break; }   // (no sign asked here: the piece alone must do it)
        break; }
      line(before === 0 && blown === 1 && at0 > 0 && builds === 1 && cutSwap.asked === 'piece', 'a wall blown with no sign given (a scripted blast): the first frame whose upload lists a cut piece asks for the set', `walls blown ${blown} · first cut piece ${at0} frame(s) after the blast (${W.engine.world.cutTraced} a ray can meet at the end) · asked '${cutSwap.asked}' · builds ${builds}`);
      await cutSwap.ready; for (let i = 0; i < 120; i++) W.step(DT);
      try { save = W.snapshot('cutneed'); } catch (e) { save = null; console.log(`    (no save: ${(e as Error).message})`); } }
    { const W2: any = await standUp(); W2.game.godMode = true; W2.game.aiEnabled = false; W2.game.tune.chaos = 0.5;
      fresh(true); quiet(() => cutSwap.start()); for (let i = 0; i < 5; i++) tick(W2); const calm = builds === 0 && W2.engine.world.cutTraced === 0;
      let loaded = false; try { if (save) { W2.restore(save); loaded = true; } } catch (e) { console.log(`    (the load was refused: ${(e as Error).message})`); }
      W2.step(DT); const traced = W2.engine.world.cutTraced; frame(traced); const asked1 = builds;   // (the first frame after the load)
      line(calm && loaded && asked1 === (traced > 0 ? 1 : 0), 'a save with a blown wall in it, loaded into a fresh game: the first frame after the load asks for the set if, and only if, the load put back a cut piece a ray can meet',
        `fresh game calm ${calm} · loaded ${loaded} · cut boxes a ray can meet after the load ${traced} (a save keeps the hole, not the loose pieces, today) · builds ${asked1}`);
      // …and a load that DOES put pieces back (what a save that kept them would do): one stood in the world between the load and its first frame
      await cutSwap.ready; fresh(true); quiet(() => cutSwap.start()); if (save) W2.restore(save);
      const h = [0.5, 0.4, 0.35], c = W2.game.player.char.pos; const cut = cutMod.cutRecord(h, [cutMod.cutThrough3([-h[0], -h[1], -h[2]], [h[0], h[1], -h[2]], [h[0], h[1], h[2]])]).cut;
      W2.game.fx.push({ box: makeBox({ c: [c[0] + 1.5, h[1], c[2]], h, albedo: [0.6, 0.6, 0.6], flags: BoxFlag.Dynamic, cut }), ttl: 1e12 });
      W2.step(DT); const traced2 = W2.engine.world.cutTraced; frame(traced2);
      line(traced2 > 0 && builds === 1 && cutSwap.asked === 'piece', '… and with a piece in the world on the first frame after a load, that frame asks', `cut boxes a ray can meet ${traced2} · asked '${cutSwap.asked}' · builds ${builds}`);
      await cutSwap.ready; }
    { const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8');
      line(/\n  cutSwap\.start\(\);/.test(main) && !/cutSwap\.build\(\)/.test(main) && /cutSwap\.waiting && !kitViewOn\) engine\.afterSubmit\.push\(\(\) => \{ if \(cutSwap\.waiting && cutComing\(game\)\) void cutSwap\.need\('sign'\)/.test(main) && /get ready\(\) \{ return cutSwap\.need\('ready'\); \}/.test(main),
        'the page: main.ts starts the set through start(), asks the sign once a frame while it waits (not on the Team & kit picture), and window.__cut.programs.ready asks for the set', 'read in src/main.ts'); }
  } finally {
    cutConfig.on = was.on; cutConfig.swap = was.swap; DESTRUCTION.on = was.destr; fresh(false); cutSwap.whenNeeded = was.row; cutSwap.members.clear(); for (const m of was.members) cutSwap.members.add(m);
  }
  return all;
}

if (import.meta.main) { const ok = await cutneedCheck(); console.log(ok ? '\n=== cutneed-check: ALL PASS ===' : '\n=== cutneed-check: FAILED ==='); process.exit(ok ? 0 : 1); }
