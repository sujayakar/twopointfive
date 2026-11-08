// WHAT THE PAGE WRITES OF THE GAME, TRAPPED (the twin of readtrap-page.cjs; its walks and its words are writetrap-page.cjs). `install` runs in the page once its
// game stands; `window.__writetrap()` then says what was written so far. The Rust game alone writes its state, in a call of the module; so
//   THE BLOCK  is copied as every call of the module ends, and held to the copy as the next one begins: a word that moved in between was written by the page. The
//              arrays the page's accessors write through are trapped as well, so a word has the functions that wrote it ('nobody trapped': a view made elsewhere).
//   A ROW      every field the wire's table names is given a write trap on the running page: a write that changes it, outside the apply, is named with its writer.
// NOT SEEN: a module's own plain variable, and a write inside a row's own array or map (what the settings' and the tools' scans hold by value).
module.exports = { install: () => { const g = window.__game, BR = window.__rust.bridge(), { SCHEMA, parse, compiledExpr, helpersOf, bookOf, face: X } = BR, sim = g.sim;
  const OURS = /^(host_api|input|knobs)::/, APPLY = /^(applyGame|applyGameNow)$/; Error.stackTraceLimit = 60;
  const READERS = /^face_(alloc|free|out_ptr|out_len|panic_ptr|panic_len|panic_clear|mem|parts|part_name|part_addr|part_len|part_hashes|die_n|time|rows|out_n|slot|kept_n|kept_state|kept_block|export|reserve|puppet_warm|draw|draw_layout|draw_look|draw_name|frame|tick_view|debrief_view|cursor_view|under|line|ray|hear|layout|version|clock|recover_what|verdict|saves|knobs)$/;
  const BLOCK = new Map(), ROWS = new Map(); let quiet = 0, trapped = 0;
  const frames = from => (new Error().stack ?? '').split('\n').slice(from).map(l => (l.match(/at (?:Object\.|new |set |get )?([\w.$<>]+)/) ?? [])[1] ?? '?').filter(x => !/^(Proxy|Reflect)/.test(x));
  const add = (M, k, by) => { let e = M.get(k); if (!e) M.set(k, e = { n: 0, by: new Map() }); e.n++; e.by.set(by, (e.by.get(by) ?? 0) + 1); };
  // ---- the block
  const who = new Map(), raw = { w: sim.w, d: sim.d }, proxied = new WeakMap();
  const trapArray = (A, name) => { if (!A || !A.buffer || A.length === 0) return A; let P = proxied.get(A); if (P) return P; const step = A.BYTES_PER_ELEMENT;
    const note = (i, n) => { if (quiet) return; const by = frames(4).slice(0, 7).join(' < '), base = (A.byteOffset - sim.__at()) >>> 2; for (let k = 0; k < Math.max(1, n * step >>> 2) && k < 64; k++) who.set(base + (i * step >>> 2) + k, by); };
    P = new Proxy(A, { get(t, k) { const v = t[k]; if (typeof v !== 'function') return v; return function (...a) { if (k === 'set') note(a[1] ?? 0, a[0].length); else if (k === 'fill' || k === 'copyWithin') note(a[1] ?? 0, (a[2] ?? t.length) - (a[1] ?? 0)); return v.apply(t, a.map(x => raws.get(x) ?? x)); }; },
      set(t, k, v) { const was = t[k]; t[k] = v; if (!Object.is(was, t[k])) note(Number(k), 1); return true; } });
    proxied.set(A, P); raws.set(P, A); return P; };
  const raws = new WeakMap();
  sim.__at = () => raw.w.byteOffset;
  for (const k of ['w', 'd']) Object.defineProperty(sim, k, { configurable: true, enumerable: true, get: () => trapArray(raw[k], k), set: v => { raw[k] = raws.get(v) ?? v; } });
  { const was = sim.f32.bind(sim); sim.f32 = (at, n) => trapArray(was(at, n), 'f32'); }
  const LIVE = () => { quiet++; try { return sim.parts().map(p => ({ name: p.name, words: raws.get(p.words) ?? p.words })); } finally { quiet--; } };
  let copy = null; const snap = () => { copy = LIVE().map(p => ({ name: p.name, off: (p.words.byteOffset - raw.w.byteOffset) >>> 2, words: p.words.slice() })); who.clear(); };
  const hold = what => { if (!copy) return; const now = LIVE(); for (let i = 0; i < now.length; i++) { const a = copy[i].words, b = now[i].words, n = Math.min(a.length, b.length); let seen = 0;
      for (let j = 0; j < n && seen < 400; j++) if (a[j] !== b[j]) { seen++; add(BLOCK, now[i].name, who.get(copy[i].off + j) ?? `nobody trapped (word ${j}), before ${what}`); if (SAMPLE.length < 60) SAMPLE.push(`${now[i].name}[${j}] ${a[j].toString(16)} > ${b[j].toString(16)} before ${what}`); } } };
  const SAMPLE = [];
  for (const k of Object.keys(X)) { const f = X[k]; if (typeof f !== 'function' || !k.startsWith('face_') || READERS.test(k)) continue; X[k] = (...a) => { hold(k); copy = null; try { return f(...a); } finally { if (sim.where >= 0 && raw.w.length) snap(); } }; }
  // ---- the rows
  const plant = { o: null, k: '' };   // (the first number trapped: what the red path writes)
  const key = f => f.ts ?? f.expr?.match(/^o\.([A-Za-z_$][\w$]*)$/)?.[1] ?? null, AT = new WeakMap();
  const trap = (o, k, q) => { let T = AT.get(o); if (T?.has(k)) return; const d = Object.getOwnPropertyDescriptor(o, k); if (!d || !d.configurable || !('value' in d) || !d.writable) return; if (!T) AT.set(o, T = new Set()); T.add(k); let v = d.value; trapped++; if (typeof v === 'number' && !plant.o) { plant.o = o; plant.k = k; }
    Object.defineProperty(o, k, { configurable: true, enumerable: d.enumerable, get: () => v, set(x) { if (!Object.is(v, x) && !quiet) { const st = frames(3); if (!st.some(s => APPLY.test(s.split('.').pop()))) add(ROWS, q, st.slice(0, 7).join(' < ')); } v = x; } }); };
  const visit = (path, o, B, seen) => { if (!o || typeof o !== 'object') return; const was = seen.get(o); if (was?.has(path)) return; if (was) was.add(path); else seen.set(o, new Set([path])); const T = SCHEMA.types[path]; if (T?.k !== 'struct') return;
    for (const f of T.fields) { const q = `${T.mod}::${T.name}.${f.rust}`; if (f.echo || OURS.test(q) ) continue; const N = parse(f.ty), k = key(f);
      if (N.k === 'type' && SCHEMA.types[N.path]?.k === 'struct') { let v; try { v = k ? o[k] : f.expr === 'o' ? o : f.expr ? compiledExpr(f.expr)(o, g, B) : undefined; } catch { v = undefined; } visit(N.path, v, B, seen); if (k) trap(o, k, q); continue; }
      if (k) trap(o, k, q); } };
  const take = () => { quiet++; try { const B = helpersOf(g, BR.knobs), bk = bookOf(g), seen = new Map(); visit(SCHEMA.root, g, B, seen); if (bk) for (const t of SCHEMA.tables) { const N = parse(t.ty); if (N.k === 'type') for (const r of bk.listOf(t.id)) visit(N.path, r, B, seen); } } finally { quiet--; } };
  const list = M => [...M].map(([q, e]) => ({ q, n: e.n, by: [...e.by].sort((a, b) => b[1] - a[1]).slice(0, 6) }));
  take(); if (sim.where >= 0) snap();
  window.__writetrap = { take, plant: () => { plant.o[plant.k] += 1; plant.o[plant.k] -= 1; }, report: () => ({ block: list(BLOCK), rows: list(ROWS), trapped, sample: SAMPLE.slice(0, 40), on: sim.where >= 0 }) };
} };
