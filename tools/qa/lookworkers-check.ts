// THE LOOK'S MAPS ON WORKERS (src/render/lookWorkers.ts; the generator's text: src/render/lookWorkerSource.ts; the caller: src/render/gbuffer.ts makeLayers).
// A layer made on a worker must be the layer made on the page's own thread, to the byte, and a place with no worker (or one that fails) must still get its
// maps. Run under bun, whose workers take the same text the page's do: every layer is made both ways and set against itself.
//   bun run tools/qa/lookworkers-check.ts      exit 0 = all pass, 1 = a FAIL line
//   LOOKWORKERS_BREAK=1 …   = the workers are handed a text that answers with the NEXT layer's maps: the bytes line must fail
//   bun run tools/qa/probes.ts lookworkers     (also under `render`)
// No GPU and no browser: that the page's own boot takes this road, and what it saves there, is a page's to show (docs/agents/render.md says what was seen).
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { LOOK_WORKERS, LookPool, lookWorkerText } from '../../src/render/lookWorkers';
import { LOOK_NAMES, LOOK_N, lookLayer, type LookMaps } from '../../src/render/lookTex';

export async function lookWorkersCheck(): Promise<boolean> {
  let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
  const BREAK = process.env.LOOKWORKERS_BREAK === '1';
  const SRC = resolve(import.meta.dir, '../../src/render');
  const text = lookWorkerText(), own = readFileSync(resolve(SRC, 'lookTex.ts'), 'utf8');
  const same = (a: LookMaps, b: LookMaps) => { if (a.tone !== b.tone || a.hollow !== b.hollow || a.a.length !== b.a.length || a.b.length !== b.b.length) return false; for (let i = 0; i < a.a.length; i++) if (a.a[i] !== b.a[i] || a.b[i] !== b.b[i]) return false; return true; };

  // ---- the text a worker runs is the generator itself, whole
  { const imports = /^\s*import[\s{*"']/m.test(text), tsImports = /^\s*import\s+(?!type\b)/m.test(own);
    check(text.length > 20000 && /\bfunction lookLayer\(/.test(text) && /self\.onmessage = /.test(text) && !imports && !tsImports,
      `a worker's text is render/lookTex.ts as this build compiled it, with its answer after it: ${text.length} characters, lookLayer in it, no module asked for (the file imports none, so its text alone runs)`); }

  // ---- the stand: down in the module, so a tool makes its maps as it did; and no pool where there is no Worker
  { const was = LOOK_WORKERS.on, down = (LOOK_WORKERS.on = false, LookPool.open(4));
    LOOK_WORKERS.on = true; const G = globalThis as { Worker?: unknown }, W = G.Worker; G.Worker = undefined; const none = LookPool.open(4); G.Worker = W; LOOK_WORKERS.on = was;
    check(was === false && down === null && none === null, `the rule is down in the module (${was}): no workers for a tool; and with it up where there is no Worker, none either (the caller makes the layer itself)`); }

  // ---- every layer, made on workers and made here
  LOOK_WORKERS.on = true;
  { const wrong = text.replace('lookLayer(d.layer, d.n, d.even)', 'lookLayer(d.layer + 1 < MATS.length ? d.layer + 1 : 0, d.n, d.even)');
    const pool = LookPool.open(LOOK_NAMES.length, BREAK ? wrong : text); const layers = LOOK_NAMES.map((_, l) => l);
    if (!pool) check(false, 'bun has workers, so a pool opens here (none did: the bytes cannot be compared)');
    else {
      const t0 = performance.now();
      const made = await Promise.all([...layers.map(l => pool.make(l, LOOK_N, true)), ...[0, 8, 15].map(l => pool.make(l, LOOK_N, false))].map(p => p.then(m => m, (e: unknown) => String(e))));
      const tW = performance.now() - t0; const workers = pool.size; pool.close();
      const bad: string[] = []; const t1 = performance.now();
      layers.forEach(l => { const m = made[l]; if (typeof m === 'string') bad.push(`${LOOK_NAMES[l]}: ${m}`); else if (!same(m, lookLayer(l, LOOK_N, true))) bad.push(LOOK_NAMES[l]); });
      [0, 8, 15].forEach((l, k) => { const m = made[layers.length + k]; if (typeof m === 'string' || !same(m, lookLayer(l, LOOK_N, false))) bad.push(`${LOOK_NAMES[l]} (not evened)`); });
      check(bad.length === 0 && (BREAK ? wrong !== text : true), `bytes — all ${layers.length} layers, and three as their materials wrote them, are the same maps made on ${workers} workers as made here (both arrays, the tone and the hollow)${bad.length ? `: NOT ${bad.slice(0, 6).join(', ')}${bad.length > 6 ? ` and ${bad.length - 6} more` : ''}` : ''} · ${(tW / 1000).toFixed(1)} s on the workers, ${((performance.now() - t1) / 1000).toFixed(1)} s here`);
    } }

  // ---- a worker that fails: its layer is refused (the caller then makes it), and a pool with no worker left refuses what waits
  { const dead = LookPool.open(2, 'throw new Error("no generator here");');
    const r = dead ? await Promise.all([dead.make(1, 64, true), dead.make(2, 64, true), dead.make(3, 64, true)].map(p => p.then(() => 'made', () => 'refused'))) : [];
    dead?.close();
    const half = LookPool.open(1, text.replace('const m = lookLayer(', 'if (d.layer === 2) throw new Error("not this one"); const m = lookLayer('));
    const h = half ? await Promise.all([half.make(1, 64, true), half.make(2, 64, true), half.make(3, 64, true)].map(p => p.then(m => m.a.length === 64 * 64 * 4 ? 'made' : 'short', () => 'refused'))) : [];
    half?.close();
    check(r.join() === 'refused,refused,refused' && h.join() === 'made,refused,made', `a pool whose workers cannot start refuses every layer (${r.join(' ') || 'no pool'}), and one layer a worker throws on is refused alone (${h.join(' ') || 'no pool'}): each refused layer is the caller's to make`); }
  LOOK_WORKERS.on = false;

  // ---- the caller: the batch's own order, and the old road for a layer no worker made
  { const g = readFileSync(resolve(SRC, 'gbuffer.ts'), 'utf8'), a = g.indexOf('private async makeLayers('), body = g.slice(a, g.indexOf('lookMade: string[] = [];', a));
    check(a > 0 && /const pool = LookPool\.open\(make\.length\), asked = pool \? make\.map\(l => pool\.make\(l, LOOK_N, this\.lookEven\)\) : null;/.test(body)
      && /if \(asked\) \{ try \{ got = await asked\[k\]; this\.lookBy\.workers\+\+; \} catch \{ got = null; \} \}\s*\n\s*if \(!got\) \{ got = await lookLayerSliced\(l, LOOK_N, this\.lookEven, sliceMs, nextTask\); this\.lookBy\.here\+\+; \}/.test(body)
      && /\} finally \{ pool\?\.close\(\); \}/.test(body),
      'the page asks for a batch\'s layers at once and takes them in the batch\'s own order; a layer no worker made is made on its own thread in slices, as before; the workers are stopped when the batch is in'); }

  console.log(`\nlookworkers-check: ${red ? `${red} FAILED` : 'all passed'}${BREAK ? ' (LOOKWORKERS_BREAK=1: the workers answer with the next layer — this run must fail)' : ''}`);
  return red === 0;
}
if (import.meta.main) process.exit(await lookWorkersCheck() ? 0 : 1);
