// THE LOOK'S MAPS ON WORKERS (src/render/lookWorkers.ts; the worker: lookWorkerMain.ts; its text: lookWorkerSource.ts → lookWorkerText.ts; the caller:
// src/render/gbuffer.ts makeLayers). A layer made on a worker must be the layer made on the page's own thread, to the byte, and a place with no worker (or
// one that fails, or never answers) must still get its maps. Run under bun, whose workers take the same text the page's do: every layer is made both ways and
// set against itself, and the page's own loop (makeLayers) is run with the workers down and up over a counting device.
//   bun run tools/qa/lookworkers-check.ts      exit 0 = all pass, 1 = a FAIL line
//   LOOKWORKERS_BREAK=1 …   = the workers are handed a text that answers with the NEXT layer's maps: the bytes line must fail
//   bun run tools/qa/probes.ts lookworkers     (also under `render`)
// No GPU and no browser: tools/qa/lookworkers-page.cjs does the bytes in Chromium; what the page's boot gains is in docs/agents/render.md.
import { LOOK_WORKERS, LookPool, lookWorkerText, lookWorkersGaveUp, lookWorkersAgain } from '../../src/render/lookWorkers';
import { LOOK_NAMES, LOOK_N, LOOK_LAYERS, lookLayer, lookMip } from '../../src/render/lookTex';
import { GBuffer, lookNeed, lookForget } from '../../src/render/gbuffer';
import { EXACT } from '../../src/math/exact';
import { sameMaps as same } from './lookworkers-same';

export async function lookWorkersCheck(): Promise<boolean> {
  let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
  const BREAK = process.env.LOOKWORKERS_BREAK === '1';
  const G = globalThis as { Worker?: unknown; sessionStorage?: unknown }, W0 = G.Worker, S0 = G.sessionStorage, tab = new Map<string, string>(), on0 = LOOK_WORKERS.on, wait0 = LOOK_WORKERS.waitMs, maths0 = EXACT.on, warn0 = console.warn;
  const said: string[] = []; console.warn = (...a: unknown[]) => { said.push(a.map(String).join(' ')); };   // (a place that gives its workers up says so once: counted, not printed)
  G.sessionStorage = { getItem: (k: string) => tab.get(k) ?? null, setItem: (k: string, v: string) => { tab.set(k, v); }, removeItem: (k: string) => { tab.delete(k); } };   // (a tab's storage, for the give-up it keeps)
  try {
    const text = await lookWorkerText();
    // ---- the text a worker runs: the worker's own module bundled whole as a plain script, nothing left to fetch
    check(text.length > 20000 && /\bfunction lookLayer\(/.test(text) && /\bonmessage = /.test(text) && /EXACT\.on = d\.exact/.test(text) && !/^\s*(import|export)[\s{*"']/m.test(text),
      `a worker's text is render/lookWorkerMain.ts bundled by itself when this was built: ${text.length} characters, the generator and the game's own maths in it, a plain script (no import, no export: a page opened from disk can start it)`);

    // ---- the stand: down in the module, so a tool makes its maps as it did; and no pool where there is no Worker
    { const down = (LOOK_WORKERS.on = false, await LookPool.open(4));
      LOOK_WORKERS.on = true; G.Worker = undefined; const none = await LookPool.open(4); G.Worker = W0;
      check(on0 === false && down === null && none === null && lookWorkersGaveUp() === '', `the rule is down in the module (${on0}): no workers for a tool; and with it up where there is no Worker, none either, and nothing given up (the caller makes the layer itself)`); }

    // ---- every layer, made on workers and made here, on the engine's maths and on the game's own (math/exact.ts: the page's)
    LOOK_WORKERS.on = true;
    { const N = LOOK_NAMES.length, wrong = text.replace('lookLayer(d.layer, d.n, d.even)', `lookLayer((d.layer + 1) % ${N}, d.n, d.even)`);
      const layers = LOOK_NAMES.map((_, l) => l), times: string[] = [], bad: string[] = []; let workers = 0, smaller = 0; const sizesOf = Math.log2(LOOK_N) + 1;
      const sameBytes = (x: Uint8Array, y: Uint8Array | undefined) => { if (!y || x.length !== y.length) return false; for (let i = 0; i < x.length; i++) if (x[i] !== y[i]) return false; return true; }; const heres: ReturnType<typeof lookLayer>[][] = [];
      for (const exact of [false, true]) {
        EXACT.on = exact;
        const pool = await LookPool.open(N, BREAK ? wrong : text);
        if (!pool) { bad.push('no pool opened (bun has workers: one should)'); continue; }
        const t0 = performance.now();
        const made = await Promise.all([...layers.map(l => pool.make(l, LOOK_N, true)), ...[0, 8, 15].map(l => pool.make(l, LOOK_N, false))].map(p => p.then(m => m, (e: unknown) => String(e))));
        const tW = performance.now() - t0; workers = pool.size; pool.close();
        const t1 = performance.now(), here = layers.map(l => lookLayer(l, LOOK_N, true)); heres.push(here);
        layers.forEach(l => { const m = made[l]; if (typeof m === 'string') bad.push(`${LOOK_NAMES[l]}: ${m}`); else if (!same(m, here[l])) bad.push(`${LOOK_NAMES[l]}${exact ? ' (the game\'s maths)' : ''}`);
          else { let a = here[l].a, b = here[l].b, ok = m.sizes.a.length === sizesOf && m.sizes.b.length === sizesOf && m.sizes.a[0] === m.a && m.sizes.b[0] === m.b;   // (its smaller sizes: the page's own lookMip of each, down to one texel)
            for (let n = LOOK_N, k = 1; ok && n > 1; n >>= 1, k++) { a = lookMip(a, n, false); b = lookMip(b, n, true); ok = sameBytes(a, m.sizes.a[k]) && sameBytes(b, m.sizes.b[k]); smaller += 2; }
            if (!ok) bad.push(`${LOOK_NAMES[l]}'s smaller sizes${exact ? ' (the game\'s maths)' : ''}`); } });
        [0, 8, 15].forEach((l, k) => { const m = made[N + k]; if (typeof m === 'string' || !same(m, lookLayer(l, LOOK_N, false))) bad.push(`${LOOK_NAMES[l]} (not evened)`); });
        times.push(`${exact ? 'the game\'s own maths' : 'the engine\'s maths'}: ${(tW / 1000).toFixed(1)} s on the workers, ${((performance.now() - t1) / 1000).toFixed(1)} s here`);
      }
      const differ = heres.length === 2 ? layers.filter(l => !same(heres[0][l], heres[1][l])).length : -1;   // (why a worker is told which maths)
      check(bad.length === 0, `bytes — all ${N} layers, and three as their materials wrote them, are the same maps made on ${workers} workers as made here (both arrays, the tone and the hollow, and the ${smaller} smaller maps a worker hands back with them against the page's own), on either maths${bad.length ? `: NOT ${bad.slice(0, 6).join(', ')}${bad.length > 6 ? ` and ${bad.length - 6} more` : ''}` : ''} · ${times.join(' · ')} · ${differ} of the layers come out differently on the two maths, so a worker is told which${BREAK && wrong === text ? ' · THE BREAK DID NOT APPLY (the text no longer holds the call it swaps)' : ''}`);
    }
    EXACT.on = maths0;

    // ---- one layer the generator throws on is refused alone; a worker that cannot start, or never answers, is given up on, said once and remembered
    { const half = await LookPool.open(1, text.replace('const m = lookLayer(', 'if (d.layer === 2) throw new Error("not this one"); const m = lookLayer('));
      const h = half ? await Promise.all([half.make(1, 64, true), half.make(2, 64, true), half.make(3, 64, true)].map(p => p.then(m => m.a.length === 64 * 64 * 4 ? 'made' : 'short', () => 'refused'))) : [];
      half?.close(); const sound = lookWorkersGaveUp() === '';
      const dead = await LookPool.open(2, 'throw new Error("no generator here");');
      const r = dead ? await Promise.all([dead.make(1, 64, true), dead.make(2, 64, true), dead.make(3, 64, true)].map(p => p.then(() => 'made', () => 'refused'))) : [];
      dead?.close(); const why1 = lookWorkersGaveUp(), after = await LookPool.open(2, text), warned1 = said.length, kept = [...tab.values()].join();
      lookWorkersAgain(); const cleared = tab.size === 0; tab.set('twopointsix.lookWorkersGaveUp', 'an earlier load of this tab said so');   // (the next page load of the tab: the module is new, the tab's note is there)
      const nextLoad = await LookPool.open(2, text), nextWhy = lookWorkersGaveUp(), quiet = said.length === warned1;
      lookWorkersAgain(); LOOK_WORKERS.waitMs = 400;
      const mute = await LookPool.open(1, 'self.onmessage = function () { };'), t0 = performance.now();
      const m = mute ? await Promise.all([mute.make(1, 64, true), mute.make(2, 64, true)].map(p => p.then(() => 'made', () => 'refused'))) : [];
      const waited = performance.now() - t0; mute?.close(); const why2 = lookWorkersGaveUp(); LOOK_WORKERS.waitMs = wait0; lookWorkersAgain();
      check(h.join() === 'made,refused,made' && sound && r.join() === 'refused,refused,refused' && why1 !== '' && after === null && warned1 === 1 && m.join() === 'refused,refused' && waited < 5000 && /did not answer/.test(why2) && said.length === 2 && kept === why1 && cleared && nextLoad === null && /earlier load/.test(nextWhy) && quiet,
        `one layer the generator throws on is refused alone and the workers go on (${h.join(' ') || 'no pool'}); workers that cannot start refuse every layer (${r.join(' ') || 'no pool'}), the place gives them up, says so once and starts no more (${after === null ? 'none' : 'a pool'}), and the tab keeps that for its next page load (${nextLoad === null && quiet ? 'none started, nothing said again' : 'NOT kept'}); a worker that never answers is given up on after its wait (${(waited / 1000).toFixed(1)} s at 0.4 s: ${m.join(' ') || 'no pool'}). Each refused layer is the caller's to make`); }

    // ---- the page's own loop: gbuffer.ts makeLayers over a counting device, the workers down and up — the same layers in the same order into the same slots, the same bytes
    { const hash = (d: Uint8Array) => { let h = 2166136261; for (let i = 0; i < d.length; i++) h = Math.imul(h ^ d[i], 16777619); return (h >>> 0).toString(16); };
      const drive = async (workers: boolean, noWorker = false) => {
        const writes: string[] = []; const dev = { queue: { submit: () => { }, writeTexture: (t: { texture: { label: string }; mipLevel: number; origin: number[] }, data: Uint8Array) => { writes.push(`${t.texture.label}:${t.mipLevel}:${t.origin[2]}:${hash(data)}`); } } };
        const g = Object.create(GBuffer.prototype) as Record<string, unknown> & { needLook(): Promise<void> };
        Object.assign(g, { device: dev, lookTex: [{ label: 'lookA' }, { label: 'lookB' }], lookRoom: LOOK_LAYERS, lookSlots: 0, lookSlot: new Int8Array(LOOK_LAYERS).fill(-1), lookMade: [], lookBy: { workers: 0, here: 0 }, lookEven: true, lookMips: 1, lookMen: [1, 0], lookFillMs: 0, lookJob: Promise.resolve(), setLook: () => { } });
        lookForget(); lookNeed(['mud wall', 'earth', 'paint', 'carpaint', 'tarmac', 'grass']);
        LOOK_WORKERS.on = workers; if (noWorker) G.Worker = undefined;
        try { await g.needLook(); } finally { G.Worker = W0; lookForget(); }
        return { made: (g.lookMade as string[]).join(', '), slots: [...(g.lookSlot as Int8Array)].join(), writes: writes.join('|'), by: g.lookBy as { workers: number; here: number }, men: (g.lookMen as number[]).join() };
      };
      const off = await drive(false), onW = await drive(true), bare = await drive(true, true), n = off.made.split(', ').length;
      check(n >= 7 && off.made === onW.made && off.slots === onW.slots && off.writes === onW.writes && off.men === onW.men && off.writes === bare.writes && off.by.here === n && off.by.workers === 0 && onW.by.workers === n && onW.by.here === 0 && bare.by.here === n && lookWorkersGaveUp() === '',
        `the page's own loop makes ${n} layers (${off.made}) in the same order, into the same slots, as the same ${off.writes.split('|').length} writes of the same bytes with the workers down (${off.by.here} here), up (${onW.by.workers} on workers, ${onW.by.here} here) and up where there is no Worker (${bare.by.here} here)`); }
  } finally { LOOK_WORKERS.on = on0; LOOK_WORKERS.waitMs = wait0; EXACT.on = maths0; G.Worker = W0; console.warn = warn0; lookWorkersAgain(); G.sessionStorage = S0; }

  console.log(`\nlookworkers-check: ${red ? `${red} FAILED` : 'all passed'}${BREAK ? ' (LOOKWORKERS_BREAK=1: the workers answer with the next layer — this run must fail)' : ''}`);
  return red === 0;
}
if (import.meta.main) process.exit(await lookWorkersCheck() ? 0 : 1);
