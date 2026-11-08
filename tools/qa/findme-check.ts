// FINDING YOUR OWN MAN, AND THE CAMERA UNDER THE HAND — the check behind src/host/findme.ts, src/ui/findmeviz.ts and src/render/outline.ts (settings in Options > Game, every one
// starting where the game stood before it had them but the outline and the camera's two, which the PAGE starts at the maintainer's own).
//   bun run tools/qa/findme-check.ts        (also `probes.ts findme`; the same on both stands)
// Headless: the whole Game under stepFrame, the loop the page runs, with nothing drawn; where the HUD would draw the ring is asked of findMeRing itself, and
// the outline's pass is run on a stand-in device that writes down what it was asked for (what the rim LOOKS like is the page's to show: findme-page.cjs).
//   table    (1) a harness has no store: the table is the starts; a kept line that is missing, broken or out of range reads as the starts, a good one as itself, and a line
//                that names a row that is no longer one (the outline's width, strength, fade, light rows, teammates' width, colour; the double tap's) reads as if it did not;
//                a row moved is kept alone (the rows still at their starts are not written, so a start tuned later reaches them), and moved back, nothing is;
//   as was   (3) the ring and the outline ON, and asked every frame, against both off: one seed, one scripted hand — the same game digest for digest and
//                the same view to the bit (picture only: nothing a man senses, nothing the light meter reads);
//   picture  (8) the ring is at his feet, gone when he is down and nothing with its row off; the shelved setting (his shape through a wall) is not in the
//                table and a kept line that names it reads without it;
//            (10) the outline: its frame is off with the row off, every man down or the page saying his marks are hidden; on, it names his owner byte and
//                a box on the picture he stands inside and that is a small part of it (empty when he is behind the lens); the pass asks the device
//                for NOTHING while the frame is off, builds one program the first frame it is on and draws nothing until that program is there, then one
//                scissored draw inside the box, the settled opacity, width, two ends and the exposure in the slot and the copy of the composite bound; nothing with no
//                copy, under the goggles or with him off the picture; a program that fails is never drawn or asked again. What the shader makes of the copy is the
//                page's to show (findme-squad-page.cjs).
//            (11) his teammates: every living man of his squad has a rim of his own, in his pair's colour, covering OUTLINE_LOOK.mates of what the player's covers
//                — a man dead gives his place up; one render pass, a scissored draw a man from his own slot and under his own box, the
//                player's last, his place kept in a crowd; a box across the lens off to the side is empty; the game's digest is not moved by any of
//                it, and a squad stepped with the outline on and filled every frame ends digest for digest where the same squad ends with it off.
// FINDME_BREAK=1 = the rows are moved and the table is not (the settings do nothing): (8), (10) and (11) must fail.   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';

export async function findmeCheck(): Promise<boolean> {
  const { stepFrame, FIXEDSTEP } = await import(`${ROOT}/src/host/loop.ts`);
  const { digestGame, diffDigest } = await import(`${ROOT}/src/game/digest.ts`);
  const { mulberry32: seeded } = await import(`${ROOT}/src/math/dice.ts`);
  const F = await import(`${ROOT}/src/host/findme.ts`) as typeof import('../../src/host/findme.ts');
  const { OutlinePass, makeOutlineFrame, makeOutlineMan, outlineBox, outlineBoxes, outlineRadius, OUTLINE, OUTLINE_MEN } = await import(`${ROOT}/src/render/outline.ts`) as typeof import('../../src/render/outline.ts');
  const { findMeRing, findMeOutline } = await import(`${ROOT}/src/ui/findmeviz.ts`) as typeof import('../../src/ui/findmeviz.ts');
  const { FINDME, FINDME_START, findMeRead } = F;
  const BREAK = process.env.FINDME_BREAK === '1';
  const set: typeof F.setFindMe = BREAK ? () => {} : F.setFindMe;
  let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== findme: the rows to find your own man and turn the view; at their starts the game as it was ===');
  const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);
  F.findMeStarts();   // (the starts follow the switch as this stand left it: a probe run before this one may have booted the page's switches and put the knob back)
  const tunedWas = F.FINDTUNED.on, startsWas = { ...FINDME_START }, foundWas = { ...FINDME };   // (the shipped stand boots the page's switches: the tuned starts. The frozen one holds the switch off)
  F.FINDTUNED.on = false; F.findMeStarts();   // (1) to (11) are the rows' own, on the starts as built
  const fixedWas = FIXEDSTEP.on, startWas = { ...FINDME };
  const ctxOf = (W: any, extra: Record<string, unknown> = {}): any => ({ game: W.game, input: W.input, cam: W.cam, view: W.canvas, host: W.engine, render: () => { W.engine.world.upload(); }, betweenTicks: () => { W.engine.world.relist(); },
    smoke: null, audio: null, attract: null, hud: null, bubbles: null, time: 0, updateMs: 0, frozen: () => false, present: () => false, fixedStep: () => true, ...extra });
  const stand = async (seed: number) => { Math.random = seeded(1); const W = await standUp({ seed }) as any; F.FINDTUNED.on = false; F.findMeStarts(); return W; };
  // (…and the starts as built: a stand-up under TPS_FINDTUNED=1 raises the tuned ones each time — src/host/standup.ts — and these lines are the rows' own)
  const DT = 1 / 60;
  try {
    // (1) the table
    { const start = same(startWas, FINDME_START);
      const bad = [null, '', 'no', '[]', '{"pan":"x","ring":1,"turn":2}', '{"pan":NaN}'].every(t => same(findMeRead(t), FINDME_START));
      const good = findMeRead('{"pan":0.4,"ring":true,"outline":true,"shape":true,"turn":-1,"behind":true}'), goodOk = same(good, { pan: 0.4, ring: true, outline: true, turn: -1 }), clamp = findMeRead('{"pan":7}').pan === 1 && findMeRead('{"pan":-3}').pan === 0;   // (a line kept by the build that had six)
      const named = findMeRead('{"pan":0,"outline":true,"width":0.25,"opacity":1,"picture":false,"lit":0.9,"dark":0.1,"picLit":0.9,"picDark":0.1,"mates":0.8,"colour":0.5,"turn":-1}'), gone = same(Object.keys(named).sort(), ['outline', 'pan', 'ring', 'turn']);
      // what is kept: only the rows that are not at their starts (a store stood in for the page's; F.setFindMe itself, whatever FINDME_BREAK says)
      const kept: string[] = [], had = Object.getOwnPropertyDescriptor(globalThis, 'localStorage');
      Object.defineProperty(globalThis, 'localStorage', { configurable: true, value: { getItem: () => kept.at(-1) ?? null, setItem: (_k: string, v: string) => { kept.push(v); } } });
      try { F.setFindMe('ring', true); F.setFindMe('pan', 0.2); F.setFindMe('pan', FINDME_START.pan); F.setFindMe('ring', true); F.resetFindMe(); F.resetFindMe(); }
      finally { if (had) Object.defineProperty(globalThis, 'localStorage', had); else delete (globalThis as any).localStorage; }
      const keptOk = same(kept, ['{"ring":true}', '{"pan":0.2,"ring":true}', '{"ring":true}', '{}']) && same(findMeRead(kept[1]), { ...FINDME_START, ring: true, pan: 0.2 }) && same(FINDME, FINDME_START);
      line(start && bad && goodOk && clamp && gone && keptOk, 'the table: a line that is broken reads as the starts, a good one as itself, the outline\'s old rows read as if unnamed, and only a row moved is kept',
        `starts ${start} · broken lines ${bad} · good line ${goodOk} · clamp ${clamp} · old rows dropped ${gone} (${Object.keys(named).join(' ')}) · what was kept: ${kept.join(' ')}`); }

    // (3) picture only: the ring and the outline on and asked every frame = both off
    { const run = async (on: boolean) => {
        const W = await stand(7), ctx = ctxOf(W); FIXEDSTEP.on = true; FINDME.ring = FINDME.outline = on; let asked = 0; const OF = makeOutlineFrame();
        for (let i = 0; i < 480; i++) {
          if (i === 20) W.input.keys.add('KeyW'); if (i === 200) { W.input.keys.delete('KeyW'); W.input.keys.add('KeyD'); } if (i === 320) W.input.keys.delete('KeyD');
          W.input.mouseX = 800 + 300 * Math.sin(i / 40); W.input.mouseY = 500 + 200 * Math.cos(i / 55); if (i % 50 === 10) W.input.orbitDX = 40; if (i === 260) W.input.clicked |= 1;
          stepFrame(ctx, DT); findMeOutline(OF, W.game); outlineBox(OF, p => W.cam.project(p, 1, 1)); if (on && findMeRing(W.game) && OF.on) asked++;
        }
        FINDME.ring = FINDME.outline = false;
        return { d: digestGame(W.game), view: [...W.cam.viewProj], asked, moved: Math.hypot(W.game.player.char.pos[0] - 5.1, W.game.player.char.pos[2] - 15.2) };
      };
      const A = await run(false), B = await run(true), diff = diffDigest(A.d, B.d);
      line(!diff.length && same(A.view, B.view) && B.asked === 480 && A.moved > 1, 'the ring and the outline on and asked every frame: the same game digest for digest, the same view to the bit',
        `eight seconds, a walk, an orbit, a shot · he moved ${A.moved.toFixed(1)} m · asked ${B.asked} times · ${diff.length ? `digests differ: ${diff.slice(0, 3).join('; ')}` : 'digests equal'} · view ${same(A.view, B.view) ? 'equal' : 'DIFFERS'}`); }

    // (8) the picture
    { const W = await stand(5), ctx = ctxOf(W); FIXEDSTEP.on = true; for (let i = 0; i < 30; i++) stepFrame(ctx, DT);
      const g = W.game, pl = g.player, p = pl.char.pos;
      const off = findMeRing(g); set('ring', true); const r = findMeRing(g); const wasDown = pl.down; pl.down = true; const rDown = findMeRing(g); pl.down = wasDown; set('ring', false);
      const ringOk = !off && !!r && r[0] === p[0] && r[2] === p[2] && !rDown, shelved = !('shape' in FINDME) && !('shape' in FINDME_START);
      line(ringOk && shelved, 'the ring at his feet, gone when he is down, nothing with its row off; his shape through a wall is not in the table',
        `ring ${r ? `at ${r[0].toFixed(1)}, ${r[2].toFixed(1)}` : 'none'}${ringOk ? '' : ' ✗'} (down: ${rDown ? 'drawn ✗' : 'none'} · row off: ${off ? 'drawn ✗' : 'none'}) · the table's keys: ${Object.keys(FINDME).join(', ')}`); }

    // (10) the outline in the dark: the frame the host fills, and what the pass asks the device for
    { const W = await stand(5), ctx = ctxOf(W); FIXEDSTEP.on = true; for (let i = 0; i < 30; i++) stepFrame(ctx, DT);
      const g = W.game, pl = g.player, cam = W.cam, Fm = makeOutlineFrame(), wasNv = pl.nvAmount, proj = (p: [number, number, number]) => cam.project(p, 1, 1);
      const fill = (shown = true) => { findMeOutline(Fm, g, shown); outlineBox(Fm, proj); };
      fill(); const offRow = !Fm.on;
      set('outline', true); pl.nvAmount = 0; fill(); const onRow = Fm.on && Fm.owner === pl.char.id && Fm.whole === 1;
      const [hx, hy] = proj([pl.char.pos[0], pl.char.pos[1] + 1, pl.char.pos[2]]), r = [...Fm.rect];
      const inBox = hx > r[0] && hx < r[2] && hy > r[1] && hy < r[3], share = Math.max(0, Math.min(1, r[2]) - Math.max(0, r[0])) * Math.max(0, Math.min(1, r[3]) - Math.max(0, r[1]));
      g.recap.allDown = true; fill(); const offDown = !Fm.on; g.recap.allDown = false; fill(false); const offHidden = !Fm.on; fill();
      const back = { ...Fm, at: [...Fm.at] as [number, number, number], rect: [9, 9, 9, 9] as [number, number, number, number] }; { const f = cam.planarBasis().fwd; back.at[0] -= f[0] * 400; back.at[2] -= f[2] * 400; outlineBox(back, proj); }
      const behindLens = back.rect.every(v => v === 0);
      // the pass, on a device that writes down what it is asked
      const rig = () => { const calls: string[] = []; const st: any = { calls, desc: null, u: null, us: [], sc: null, scs: [], offs: [], drawn: 0, draws: 0, pass: null, res: null, rej: null };
        const device: any = { createBindGroupLayout: (d: unknown) => { calls.push('layout'); st.layout = d; return {}; }, createBuffer: (d: unknown) => { calls.push('buffer'); st.buf = d; return {}; }, createPipelineLayout: () => ({}), createBindGroup: (d: unknown) => { calls.push('group'); st.group = d; return {}; }, createTexture: () => { calls.push('texture'); return { createView: () => ({ blank: true }) }; },
          createShaderModule: (d: { code: string }) => { calls.push('module'); st.code = d.code; return { getCompilationInfo: async () => ({ messages: [] }) }; },
          createRenderPipelineAsync: (d: unknown) => { calls.push('pipeline'); st.desc = d; return new Promise((res, rej) => { st.res = res; st.rej = rej; }); },
          queue: { writeBuffer: (_b: unknown, o: number, data: ArrayBuffer) => { st.u = data.slice(0); st.us.push({ o, f: new Float32Array(data.slice(0)), w: new Uint32Array(data.slice(0)) }); } } };
        const enc: any = { beginRenderPass: (d: unknown) => { calls.push('pass'); st.pass = d; return { setPipeline() {}, setBindGroup: (_i: number, _g: unknown, offs?: Uint32Array, at = 0) => { st.offs.push(offs ? offs[at] : -1); }, setScissorRect: (x: number, y: number, w: number, h: number) => { st.sc = [x, y, w, h]; st.scs.push([x, y, w, h].join(' ')); }, draw: (n: number) => { st.drawn = n; st.draws++; }, end() {} }; } };
        return { st, device, enc, P: new OutlinePass(device) }; };
      const n = (st: any, k: string) => st.calls.filter((c: string) => c === k).length, tick = () => new Promise(r => setTimeout(r, 0));
      const A = rig(), CW = 1280, CH = 720, hdr = {} as any, id = {} as any, dep = {} as any, off = makeOutlineFrame(), picV = { picture: true } as any;
      A.P.encode(A.enc, off, hdr, id, dep, CW, CH, picV); const offAsks = A.st.calls.length === 0;
      A.P.encode(A.enc, Fm, hdr, id, dep, CW, CH, picV); A.P.encode(A.enc, Fm, hdr, id, dep, CW, CH, picV); const built = n(A.st, 'module') === 1 && n(A.st, 'pipeline') === 1 && n(A.st, 'pass') === 0;
      A.st.res?.({}); await tick(); Fm.exposure = 2; A.P.encode(A.enc, Fm, hdr, id, dep, CW, CH, picV); A.P.encode(A.enc, Fm, hdr, id, dep, CW, CH, picV); Fm.exposure = 1;
      const sc = A.st.sc as number[] | null, u = A.st.u ? { f: new Float32Array(A.st.u), w: new Uint32Array(A.st.u) } : null, near = (a: number, b: number) => Math.abs(a - b) < 1e-4;
      const inside = !!sc && sc[0] >= Math.floor(r[0] * CW) - 1 && sc[1] >= Math.floor(r[1] * CH) - 1 && sc[0] + sc[2] <= Math.min(CW, Math.ceil(r[2] * CW)) && sc[1] + sc[3] <= Math.min(CH, Math.ceil(r[3] * CH)) && sc[2] > 0 && sc[3] > 0 && sc[0] >= 0 && sc[1] >= 0;
      const tgt = (A.st.desc as any)?.fragment?.targets?.[0], att = (A.st.pass as any)?.colorAttachments?.[0];
      const drew = n(A.st, 'pass') === 2 && n(A.st, 'pipeline') === 1 && n(A.st, 'group') === 1 && A.st.drawn === 3 && A.P.drawn === 2 && inside && !!tgt?.blend && tgt.format === 'rgba16float' && att?.loadOp === 'load' && att.view === hdr
        && !!u && u.w[4] === pl.char.id && near(u.f[3], OUTLINE.opacity) && u.f[5] === Math.fround(outlineRadius(CH)) && near(u.f[8], OUTLINE.lit) && near(u.f[9], OUTLINE.dark) && u.f[10] === 2 && A.st.group?.entries?.[3]?.resource === picV && !/\bstorage\b|var<storage/.test(A.st.code ?? 'storage');
      // the settled rim: a quarter of its width by what it covers, the teammates' four fifths of that, never over the cap
      const numbers = OUTLINE.wide === 0.25 && OUTLINE.opacity === 0.6 && OUTLINE.lit === 0.3 && OUTLINE.dark === 0.12 && F.OUTLINE_LOOK.mates === 0.8 && F.OUTLINE_LOOK.colour === 0.5 && same(F.FINDME_TUNED, { pan: 0, outline: true, turn: -1 });
      const own = (h: number) => Math.max(OUTLINE.minPx, Math.min(OUTLINE.maxPx, h * OUTLINE.width)), covers = [300, 720, 2160].every(h => near(outlineRadius(h) - 0.5, (own(h) - 0.5) / 16) && near(outlineRadius(h, 0.8) - 0.5, (own(h) - 0.5) / 16 * 0.8) && outlineRadius(h) <= OUTLINE.capPx);
      // nothing with no copy of the composite to read, under the goggles, or with him off the picture or behind the lens
      const count = () => n(A.st, 'pass'), p0 = count(); A.P.encode(A.enc, Fm, hdr, id, dep, CW, CH, null); const noCopy = count() === p0;
      pl.nvAmount = 1; fill(); A.P.encode(A.enc, Fm, hdr, id, dep, CW, CH, picV); const goggles = count() === p0; pl.nvAmount = 0; fill();
      const away = { ...Fm, rect: [1.2, 0.2, 1.4, 0.5] as [number, number, number, number] }; A.P.encode(A.enc, away, hdr, id, dep, CW, CH, picV); A.P.encode(A.enc, back, hdr, id, dep, CW, CH, picV); A.P.encode(A.enc, off, hdr, id, dep, CW, CH, picV); const skipped = count() === p0;
      const B = rig(), err = console.error; let said = 0; console.error = () => { said++; };
      try { B.P.encode(B.enc, Fm, hdr, id, dep, CW, CH, picV); B.st.rej?.(new Error("no")); await tick(); await tick(); for (let i = 0; i < 3; i++) B.P.encode(B.enc, Fm, hdr, id, dep, CW, CH, picV); } finally { console.error = err; }
      const failed = n(B.st, 'pipeline') === 1 && n(B.st, 'pass') === 0 && said === 1;
      set('outline', false); pl.nvAmount = wasNv; fill();   // (and the frame is off again)
      const src = await Bun.file(`${ROOT}/src/render/passes.ts`).text(), eng = await Bun.file(`${ROOT}/src/engine.ts`).text(), site = (src.match(/new OutlinePass\(/g) ?? []).length === 1 && src.includes('if (this.outline?.on) {') && src.includes('(this.outlinePass ??= new OutlinePass(this.device)).encode(enc, this.outline,')
        && /private get copyOn\(\): boolean \{ return this\.gbaaOn \|\| this\.sceneCopyOn \|\| this\.outlineCopyOn; \}/.test(src) && eng.includes('this.passes.setOutlineCopy(!!this.outline?.on &&');   // (and hands it a copy of this frame's composite: GBAA's, or one kept for it)
      line(offRow && onRow && inBox && share < 0.5 && offDown && offHidden && behindLens && offAsks && built && drew && numbers && covers && noCopy && goggles && skipped && failed && site,
        'the outline: off with the row off, all down or its marks hidden; one program, then a scissored draw at the settled numbers with the copy bound; nothing with no copy, under the goggles or off the picture',
        `row off ${offRow}, on ${onRow}, all down ${offDown}, hidden ${offHidden} · his box holds him ${inBox}, ${(share * 100).toFixed(0)} % of the picture, behind the lens empty ${behindLens} · frame off asks nothing ${offAsks} · one program ${built} · draws ${drew} · numbers ${numbers} · covers ${covers} · no copy ${noCopy}, goggles ${goggles}, off the picture ${skipped} · failed program ${failed} · the site ${site}`);
      // (11) his teammates: a squad of four stood up, every man in the dark
      { const W2 = await stand(5), { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`), hush = console.log; console.log = () => {}; try { await startFromQuery(W2.game, '?squad4'); } finally { console.log = hush; }
        const ctx2 = ctxOf(W2), LIVE = 120; for (let i = 0; i < LIVE; i++) stepFrame(ctx2, DT);
        const g2 = W2.game, p2 = g2.player, men = g2.orders.men() as any[], M = makeOutlineFrame(), proj2 = (p: [number, number, number]) => W2.cam.project(p, 1, 1), INK = F.OUTLINE_INK;
        // the same squad, the same seed, the same frames, with the outline on and its frame filled and boxed every one of them: the game cannot tell
        const W3 = await stand(5); console.log = () => {}; try { await startFromQuery(W3.game, '?squad4'); } finally { console.log = hush; }
        set('outline', true); const ctx3 = ctxOf(W3), M3 = makeOutlineFrame(); let filled = 0;
        for (let i = 0; i < LIVE; i++) { stepFrame(ctx3, DT); findMeOutline(M3, W3.game); outlineBoxes(M3, p => W3.cam.project(p, 1, 1)); if (M3.on && M3.mates.filter(m => m.on).length === 4) filled++; }
        set('outline', false); const liveDiff = diffDigest(digestGame(g2), digestGame(W3.game)), stepped = filled === LIVE && !liveDiff.length;
        const fillM = () => { p2.nvAmount = 0; findMeOutline(M, g2); outlineBoxes(M, proj2); return M.mates.filter(m => m.on); }, S = F.OUTLINE_LOOK.colour;
        const mix = (hue: 'you' | 'red' | 'blue') => INK.pale.map((v: number, i: number) => v + (INK[hue][i] - v) * S), inkIs = (m: { ink: number[] }, hue: 'you' | 'red' | 'blue') => m.ink.every((v, i) => Math.abs(v - mix(hue)[i]) < 1e-9);
        const d0 = JSON.stringify(digestGame(g2)); set('outline', true); const on = fillM(), pairOf = (id: number) => g2.orders.pairOf(men.find(gd => gd.char.id === id));
        const each = men.length === 4 && on.length === 4 && on.every(m => m.owner !== p2.char.id && m.times === F.OUTLINE_LOOK.mates && men.some(gd => gd.char.id === m.owner && gd.char.pos[0] === m.at[0] && gd.char.pos[2] === m.at[2])) && new Set(on.map(m => m.owner)).size === 4;
        const hues = g2.orders.paired && inkIs(M, 'you') && on.filter(m => pairOf(m.owner) === 'red').length === 2 && on.every(m => inkIs(m, pairOf(m.owner) === 'red' ? 'red' : 'blue'));
        const st1 = men[1].state; men[1].state = 'dead'; const three = fillM(), gone = three.length === 3 && !three.some(m => m.owner === men[1].char.id); men[1].state = st1; const backIn = fillM().length === 4;
        // the pass: a scissored triangle a man, each with his own slot
        const D = rig(); D.P.encode(D.enc, M, hdr, id, dep, CW, CH, picV); D.st.res?.({}); await tick(); D.st.us.length = 0; D.P.encode(D.enc, M, hdr, id, dep, CW, CH, picV);
        const boxOf = (m: { rect: number[] }) => { const x0 = Math.max(0, Math.floor(m.rect[0] * CW)), y0 = Math.max(0, Math.floor(m.rect[1] * CH)), x1 = Math.min(CW, Math.ceil(m.rect[2] * CW)), y1 = Math.min(CH, Math.ceil(m.rect[3] * CH)); return [x0, y0, x1 - x0, y1 - y0].join(' '); };
        const vis = [...M.mates, M].filter(m => { const x0 = Math.max(0, Math.floor(m.rect[0] * CW)), y0 = Math.max(0, Math.floor(m.rect[1] * CH)), x1 = Math.min(CW, Math.ceil(m.rect[2] * CW)), y1 = Math.min(CH, Math.ceil(m.rect[3] * CH)); return m.on && x1 > x0 && y1 > y0; });
        const slots = [...D.st.us] as { o: number; f: Float32Array; w: Uint32Array }[], drawsOk = vis.length >= 2 && n(D.st, 'pass') === 1 && D.st.draws === vis.length && slots.length === vis.length && slots.every((u, i) => u.o === i * 256 && u.w[4] === vis[i].owner && Math.abs(u.f[0] - vis[i].ink[0]) < 1e-6)
          && same(D.st.offs, vis.map((_m, i) => i * 256)) && (D.st.desc as any) && n(D.st, 'buffer') === 1 && n(D.st, 'group') === 1
          && vis[vis.length - 1] === M && same(D.st.scs.slice(-vis.length), vis.map(boxOf)) && new Set(D.st.scs.slice(-vis.length)).size > 1   // (his own last, over theirs; each man's draw under his own box)
          && D.st.layout?.entries?.[2]?.buffer?.hasDynamicOffset === true && D.st.layout.entries[2].buffer.minBindingSize === 48 && D.st.buf?.size === OUTLINE_MEN * 256 && D.st.group?.entries?.[2]?.resource?.size === 48 && !D.st.group.entries[2].resource.offset;
        // the teammates' share on the device, from the radius each slot was written with
        D.st.us.length = 0; D.P.encode(D.enc, M, hdr, id, dep, CW, CH, picV); const rs = (D.st.us as { f: Float32Array; w: Uint32Array }[]), mine = rs.find(u => u.w[4] === p2.char.id), theirs = rs.filter(u => u.w[4] !== p2.char.id);
        const shareDrawn = !!mine && theirs.length === vis.length - 1 && theirs.every(u => Math.abs((u.f[5] - 0.5) - (mine.f[5] - 0.5) * F.OUTLINE_LOOK.mates) < 1e-6);
        // more men than the pass has places: his own is still drawn, the last
        const crowd = { ...M, mates: Array.from({ length: OUTLINE_MEN + 3 }, (_v, i) => ({ ...makeOutlineMan(), on: true, owner: 40 + i, rect: [0.1, 0.1, 0.2, 0.2] as [number, number, number, number] })) }; D.st.us.length = 0; const dr0 = D.st.draws; D.P.encode(D.enc, crowd, hdr, id, dep, CW, CH, picV);
        const crowded = D.st.draws - dr0 === OUTLINE_MEN && D.st.us.length === OUTLINE_MEN && D.st.us[OUTLINE_MEN - 1].w[4] === p2.char.id && D.st.us[OUTLINE_MEN - 1].o === (OUTLINE_MEN - 1) * 256;
        // a man off to the side with his box across the lens's plane is not looked for over the whole picture; one in the middle of it is
        const lens = (cx: number) => { const m = makeOutlineMan(); let k = 0; outlineBox(m, () => (k++ === 0 ? [cx, 0.5, true] : [0.5, 0.5, false])); return m.rect.join(' '); }, aside = lens(3) === '0 0 0 0' && lens(0.5) === '0 0 1 1';
        for (let i = 0; i < 200; i++) fillM(); const pure = JSON.stringify(digestGame(g2)) === d0;
        set('outline', false); const offAll = fillM().length === 0 && !M.on && M.mates.every(m => !m.owner);
        line(each && hues && gone && backIn && drawsOk && shareDrawn && crowded && aside && pure && stepped && offAll,
          'his teammates: each living man of his squad rimmed in his pair\'s colour, his rim four fifths of what the player\'s covers; one pass, a draw a man under his own box, the player\'s last; nothing of the game\'s written',
          `men ${men.length}, rimmed ${on.length}${each ? '' : ' ✗'} · inks: yellow for him, two red, two blue ${hues ? 'yes' : 'NO ✗'} · on the device ${shareDrawn ? 'the share drawn' : '✗'} · a man dead: ${gone ? 'his place given up' : '✗'}${backIn ? '' : ' (not taken back ✗)'} · the pass: ${vis.length} men on the picture, slots ${slots.map(u => u.o).join(' ')}, his own last, a box each${drawsOk ? '' : ' ✗'}; ${OUTLINE_MEN + 3} teammates: ${crowded ? `${OUTLINE_MEN} draws, his the last` : '✗'}; a box across the lens off to the side: ${aside ? 'empty' : '✗'} · digests ${pure && stepped ? 'unmoved' : 'MOVED ✗'} · off ${offAll}`); }
    }
  } finally { F.FINDTUNED.on = tunedWas; F.findMeStarts(); Object.assign(FINDME, foundWas); FIXEDSTEP.on = fixedWas; }
  console.log(all ? '\nfindme: ALL PASS' : '\nfindme: FAIL');
  return all;
}
if (import.meta.main) process.exit((await findmeCheck()) ? 0 : 1);
