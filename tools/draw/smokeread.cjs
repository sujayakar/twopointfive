// WHAT IS DRAWN OF THE SMOKE AGAINST WHAT THE GAME HOLDS, read back off the solver's buffers: handed to the page by smokejump-page and smokelife-page.
/** in the page: a tick nobody draws, a frame of the solver alone, and what is drawn against what the game handed over */
const hands = broken => {
  const D = window.__draw, E = D.engine, S = D.smoke, dev = E.device;
  if (broken) S.whole = () => false;
  window.J = {
    tick(n = 1) { D.run(n, [], 'none'); },
    async step(dt) { E.lastDt = dt; S.update(dt); const enc = dev.createCommandEncoder(); S.encode(enc, () => undefined); dev.queue.submit([enc.finish()]); S.afterSubmit(); await dev.queue.onSubmittedWorkDone(); },
    async frame(dt = 1 / 60) { E.lastDt = dt; S.update(dt); const shown = S.binding.count, enc = dev.createCommandEncoder(); S.encode(enc, () => undefined); dev.queue.submit([enc.finish()]); S.afterSubmit(); await dev.queue.onSubmittedWorkDone(); return this.read(shown); },
    async read(shown) {
      const cap = S.grid.cap, words = S.bentAt + 16 + 512 * 162, mB = words * 4, sB = cap * 1664 * 4, rb = dev.createBuffer({ size: mB + sB, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ }), enc = dev.createCommandEncoder();
      enc.copyBufferToBuffer(S.M, 0, rb, 0, mB); enc.copyBufferToBuffer(S.S, 0, rb, mB, sB); dev.queue.submit([enc.finish()]); await rb.mapAsync(GPUMapMode.READ);
      const all = rb.getMappedRange(), M = new Uint32Array(all, 0, words), F = new Float32Array(all, 0, words), W = new Uint32Array(all, mB, sB / 4), H = new Uint16Array(all, mB, sB / 2);
      const half = h => { const e = (h >> 10) & 31, m = h & 1023; return e === 0 ? m * 2 ** -24 : (1 + m / 1024) * 2 ** (e - 15); };
      const at = S.bentAt, wide = S.bentRows ? M[at] : 0, deep = S.bentRows ? M[at + 1] : 0, drawn = new Map(), hues = new Map(), keys = new Set(), slots = new Map();
      for (let i = 0, n = S.needsReset ? 0 : M[0]; i < n; i++) {   // (a brick of the coarse lattice is a cell of the game's: what of it is the game's own, over its air; one whose middle is solid is nobody's)
        const s = M[16 + i], k = 16 + 2 * cap + s * 42, cw = M[k]; if ((cw >>> 27) & 1) continue;
        slots.set([cw & 1023, (cw >>> 10) & 127, (cw >>> 17) & 1023].join(), s);
        let sum = 0, air = 0; const at2 = [(cw & 1023) >> 2, ((cw >>> 17) & 1023) >> 2].join(), hue = hues.get(at2) ?? [0, 0, 0, 0]; hues.set(at2, hue);
        for (let c = 0; c < 512; c++) { if ((W[cap * 1024 + s * 128 + (c >> 2)] >>> ((c & 3) * 8)) & 1) continue; air++; const kw = W[cap * 1152 + s * 512 + c], d = half(H[(s * 512 + c) * 2] & 0x7fff); sum += d * (1 - (kw & 255) / 255); hue[0] += d; for (let a = 1; a < 4; a++) hue[a] += d * ((kw >>> (8 * a)) & 255); }
        if (air) { const key = [cw & 1023, (cw >>> 10) & 127, (cw >>> 17) & 1023].join(); drawn.set(key, (M[k + 1] & 128) ? -1 : sum / air); keys.add(key); }
      }
      const game = (x, y, z) => { const row = y > 7 || x >> 2 >= wide || z >> 2 >= deep ? 0 : M[at + 16 + (z >> 2) * wide + (x >> 2)]; return row ? F[at + 16 + 512 + (row - 1) * 161 + y * 16 + (z & 3) * 4 + (x & 3)] : 0; };
      for (let z = 0; z < deep * 4; z++) for (let x = 0; x < wide * 4; x++) for (let y = 0; y < 8; y++) if (game(x, y, z) > 0.001) keys.add([x, y, z].join());
      let off = 0, n = 0, has = 0, is = 0, bare = 0;
      for (const key of keys) { const g = game(...key.split(',').map(Number)), m = drawn.get(key) ?? 0; if (m < 0) continue; has += g; is += m; if (g > 0.03 && m < 0.2 * g) bare++; if (Math.max(g, m) > 0.01) { off += Math.abs(Math.exp(-18 * g) - Math.exp(-18 * m)); n++; } }
      // (the color of it, by the game's own bricks: where it has one, how far what is drawn is from it)
      let dyed = 0, hueOff = 0; for (let z = 0; z < deep; z++) for (let x = 0; x < wide; x++) { const row = M[at + 16 + z * wide + x]; if (!row) continue; const w = M[at + 16 + 512 + (row - 1) * 161 + 160], h = hues.get([x, z].join()); if (!w || !h || h[0] < 20) continue; dyed++; for (let a = 1; a < 4; a++) hueOff = Math.max(hueOff, Math.abs(((w >>> (8 * a)) & 255) - h[a] / h[0])); }
      // (lines of sight through it at four heights, eight metres each, round the middle of the game's: what each lets through, in the game's cells and in the cells drawn)
      const mid = [0, 0, 0]; for (const key of keys) { const p = key.split(',').map(Number), g = game(...p); mid[0] += g; mid[1] += g * (p[0] + 0.5) / 2; mid[2] += g * (p[2] + 0.5) / 2; }
      const cx = mid[1] / mid[0], cz = mid[2] / mid[0], fine = (px, py, pz) => { const X = Math.floor(px * 16), Y = Math.floor(py * 16), Z = Math.floor(pz * 16), s = slots.get([X >> 3, Y >> 3, Z >> 3].join()); if (s === undefined) return 0; const c = s * 512 + (X & 7) + ((Y & 7) << 3) + ((Z & 7) << 6); return half(H[c * 2] & 0x7fff) * (1 - (W[cap * 1152 + c] & 255) / 255); };
      let sight = 0, blind = 0, lines = 0; if (mid[0] > 0) for (let a = 0; a < 24; a++) for (const h of [0.5, 1, 1.6, 2.2]) for (const aside of [-2.5, -1.5, -0.7, 0, 0.7, 1.5, 2.5]) {
        const ux = Math.cos(a * Math.PI / 24), uz = Math.sin(a * Math.PI / 24); let tg = 0, td = 0;
        for (let t = -4; t < 4; t += 1 / 16) { const px = cx - uz * aside + ux * t, pz = cz + ux * aside + uz * t; if (px >= 0 && pz >= 0) { tg += game(Math.floor(px * 2), Math.floor(h * 2), Math.floor(pz * 2)); td += fine(px, h, pz); } }
        const G = Math.exp(-6 * tg / 16), T = Math.exp(-6 * td / 16); sight += Math.abs(G - T); lines++; if (G < 0.3 && T > 0.5) blind++; }
      rb.unmap(); rb.destroy(); const r = v => Math.round(v * 1000) / 1000;
      return { time: r(D.game.time), game: r(has), drawn: r(is), off: r(n ? off / n : 0), sight: r(lines ? sight / lines : 0), blind: r(lines ? blind / lines : 0), bare, shown, dyed, hueOff: r(hueOff), dusty: S.binding.dusty, heard: [...D.audio.going.values()].filter(g => !g.of.reach).map(g => [g.of.name, r(g.of.age)]), cans: S.emitters.filter(e => e.kind === 'canister').map(e => r(e.age)) };
    },
  };
};
module.exports = { hands };
