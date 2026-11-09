// WHERE THE GAME JUMPS, THE DRAWN SMOKE IS THE GAME'S IN THE FIRST FRAME: the page on the software adapter, its solver stepped by hand. A smoke can is thrown and nobody draws it; a save, a load, a load under a
// card (no time goes by), a restart, and the run's tape sought on and back. After each, what is drawn is read back and averaged over the game's own cells. Never a GPU's judgement of how it looks.
//   PORT=<a dev server> node tools/draw/smokejump-page.cjs      exit 0 = every check passed, 1 = a FAIL, 2 = the script died · JUMP_BREAK=1 = the page never takes the smoke whole: it must fail
const PAGE = require('../qa/page.cjs').page({ name: 'smokejump-page', detail: 'json' }), { check } = PAGE, BREAK = process.env.JUMP_BREAK === '1';
/** in the page: a tick nobody draws, a frame of the solver alone, and what is drawn against what the game handed over */
const hands = broken => {
  const D = window.__draw, E = D.engine, S = D.smoke, dev = E.device;
  if (broken) S.whole = () => false;
  window.J = {
    tick(n = 1) { D.run(n, [], 'none'); },
    async frame(dt = 1 / 60) { E.lastDt = dt; S.update(dt); const shown = S.binding.count, enc = dev.createCommandEncoder(); S.encode(enc, () => undefined); dev.queue.submit([enc.finish()]); S.afterSubmit(); await dev.queue.onSubmittedWorkDone(); return this.read(shown); },
    async read(shown) {
      const cap = S.grid.cap, words = S.bentAt + 16 + 512 * 161, mB = words * 4, sB = cap * 1664 * 4, rb = dev.createBuffer({ size: mB + sB, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ }), enc = dev.createCommandEncoder();
      enc.copyBufferToBuffer(S.M, 0, rb, 0, mB); enc.copyBufferToBuffer(S.S, 0, rb, mB, sB); dev.queue.submit([enc.finish()]); await rb.mapAsync(GPUMapMode.READ);
      const all = rb.getMappedRange(), M = new Uint32Array(all, 0, words), F = new Float32Array(all, 0, words), W = new Uint32Array(all, mB, sB / 4), H = new Uint16Array(all, mB, sB / 2);
      const half = h => { const e = (h >> 10) & 31, m = h & 1023; return e === 0 ? m * 2 ** -24 : (1 + m / 1024) * 2 ** (e - 15); };
      const at = S.bentAt, wide = S.bentRows ? M[at] : 0, deep = S.bentRows ? M[at + 1] : 0, drawn = new Map(), keys = new Set();
      for (let i = 0, n = S.needsReset ? 0 : M[0]; i < n; i++) {   // (a brick of the coarse lattice is a cell of the game's: what of it is the game's own, over its air; one whose middle is solid is nobody's)
        const s = M[16 + i], k = 16 + 2 * cap + s * 42, cw = M[k]; if ((cw >>> 27) & 1) continue;
        let sum = 0, air = 0; for (let c = 0; c < 512; c++) { if ((W[cap * 1024 + s * 128 + (c >> 2)] >>> ((c & 3) * 8)) & 1) continue; air++; sum += half(H[(s * 512 + c) * 2] & 0x7fff) * (1 - (W[cap * 1152 + s * 512 + c] & 255) / 255); }
        if (air) { const key = [cw & 1023, (cw >>> 10) & 127, (cw >>> 17) & 1023].join(); drawn.set(key, (M[k + 1] & 128) ? -1 : sum / air); keys.add(key); }
      }
      const game = (x, y, z) => { const row = y > 7 || x >> 2 >= wide || z >> 2 >= deep ? 0 : M[at + 16 + (z >> 2) * wide + (x >> 2)]; return row ? F[at + 16 + 512 + (row - 1) * 160 + y * 16 + (z & 3) * 4 + (x & 3)] : 0; };
      for (let z = 0; z < deep * 4; z++) for (let x = 0; x < wide * 4; x++) for (let y = 0; y < 8; y++) if (game(x, y, z) > 0.001) keys.add([x, y, z].join());
      let off = 0, n = 0, has = 0, is = 0, bare = 0;
      for (const key of keys) { const g = game(...key.split(',').map(Number)), m = drawn.get(key) ?? 0; if (m < 0) continue; has += g; is += m; if (g > 0.03 && m < 0.2 * g) bare++; if (Math.max(g, m) > 0.01) { off += Math.abs(Math.exp(-18 * g) - Math.exp(-18 * m)); n++; } }
      rb.unmap(); rb.destroy(); const r = v => Math.round(v * 1000) / 1000;
      return { time: r(D.game.time), game: r(has), drawn: r(is), off: r(n ? off / n : 0), bare, shown, cans: S.emitters.filter(e => e.kind === 'canister').map(e => r(e.age)) };
    },
  };
};
const whole = r => r.game > 20 && r.off < 0.04 && r.bare === 0 && Math.abs(r.drawn / r.game - 1) < 0.05 && r.shown > 0;
(async () => {
  const context = await PAGE.context({ size: [160, 90] });
  const open = async words => {
    const { page, errors, go } = await PAGE.open({ context, timeout: 900000, errors: { also: /Invalid|error:/ } }); await go(words, { timeout: 600000 });
    await page.waitForFunction(() => !!window.__ready && !!window.__draw, null, { timeout: 900000 }); await page.evaluate(() => window.__ready); await page.evaluate(hands, BREAK);
    return { page, errors };
  };
  const R = await open('paused&quality=verylow&adapt=0&mute&cloud=0&seed=7&levelFile=office.json&roomclear&squad4&loadbrief=0');
  const run = await R.page.evaluate(async () => {
    const d = window.__draw, C = d.game.commands, I = d.input, out = {}; d.run(1, [], 'last'); C.god(1); C.ai(0);
    I.mouseX = 80; I.mouseY = 32; J.tick(2); I.pressed.add('Digit3'); I.keys.add('Digit3'); J.tick(5); I.keys.delete('Digit3'); J.tick(30); I.buttons |= 1; I.clicked |= 1; J.tick(20); I.buttons = 0;
    for (let i = 0; i < 300 && !d.smoke.emitters.some(e => e.kind === 'canister'); i++) J.tick();
    J.tick(240); C.save(); J.tick(3); const key = d.game.saves().rows[0].key;
    J.tick(120); C.load(key); J.tick(); out.load = await J.frame(); J.tick(6); out.on = await J.frame();
    C.load(key); J.tick(); out.held = await J.frame(0);
    J.tick(200); out.ticks = d.game.tape.ticks; C.restart(''); J.tick(); out.restart = await J.frame();
    window.dispatchEvent(new KeyboardEvent('keydown', { code: 'F7', key: 'F7', cancelable: true, bubbles: true })); await new Promise(r => setTimeout(r, 1500));
    const db = await new Promise((res, rej) => { const q = indexedDB.open('twopointsix-kept'); q.onsuccess = () => res(q.result); q.onerror = rej; });
    out.sid = (await new Promise(res => { const q = db.transaction('visits').objectStore('visits').getAll(); q.onsuccess = () => res(q.result); }))[0]?.sid; return out;
  });
  check(whole(run.load), 'the first frame after a load has the whole of the game\'s smoke', run.load);
  check(run.load.cans.length === 1 && Math.abs(run.load.cans[0] - 4) < 0.3, 'the can still pours, as old as it is', run.load.cans);
  check(run.on.off < 0.08 && run.on.cans.length === 1, 'a tenth of a second on it is the game\'s still, and the can is one', run.on);
  check(whole(run.held), 'a load drawn with no time gone by has it too', run.held);
  check(run.restart.drawn === 0 && run.restart.cans.length === 0, 'a restart has none of it', run.restart);
  check(R.errors.length === 0, 'the run: nothing threw', R.errors.slice(0, 4));
  await R.page.close();
  const P = await open(`playtape=${run.sid}&quality=verylow&adapt=0&mute&paused`);
  const seek = to => P.page.evaluate(async to => { const d = window.__draw; d.played.seek(to); for (let i = 0; i < 4000 && d.played.seeking; i++) J.tick(); const r = await J.frame(); return { ...r, at: d.played.at }; }, to);
  await P.page.evaluate(() => { const d = window.__draw; d.run(1, [], 'last'); });
  for (const [to, words] of [[run.ticks - 30, 'on'], [run.ticks - 90, 'back']]) { const r = await seek(to); check(Math.abs(r.at - to) <= 2 && whole(r) && r.cans.length === 1, `a place sought ${words} on the run's tape has the whole of the smoke there, and its can`, r); }
  check(P.errors.length === 0, 'the tape: nothing threw', P.errors.slice(0, 4));
  await PAGE.end();
})().catch(e => { console.error(e); process.exit(2); });
