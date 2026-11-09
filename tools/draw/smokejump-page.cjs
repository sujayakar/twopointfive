// WHERE THE GAME JUMPS, THE DRAWN SMOKE IS THE GAME'S IN THE FIRST FRAME: the page on the software adapter, its solver stepped by hand. A smoke can is thrown, then a grenade whose dust is tan, and nobody draws it; a save, a load, a load under a
// card (no time goes by), a restart, and the run's tape sought on and back. After each, what is drawn is read back and averaged over the game's own cells. Never a GPU's judgement of how it looks.
//   PORT=<a dev server> node tools/draw/smokejump-page.cjs      exit 0 = every check passed, 1 = a FAIL, 2 = the script died · JUMP_BREAK=1 = the page never takes the smoke whole: it must fail
const PAGE = require('../qa/page.cjs').page({ name: 'smokejump-page', detail: 'json' }), { check } = PAGE, BREAK = process.env.JUMP_BREAK === '1';
const { hands } = require('./smokeread.cjs');
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
    // a grenade, and a save in its dust
    I.pressed.add('Digit5'); I.keys.add('Digit5'); J.tick(5); I.keys.delete('Digit5'); J.tick(40); I.buttons |= 1; I.clicked |= 1; J.tick(20); I.buttons = 0; J.tick(330); C.save(); J.tick(3); const dust = d.game.saves().rows.at(-1).key;
    J.tick(60); C.load(dust); J.tick(); out.dust = await J.frame();
    J.tick(200); out.ticks = d.game.tape.ticks; C.restart(''); J.tick(); out.restart = await J.frame();
    window.dispatchEvent(new KeyboardEvent('keydown', { code: 'F7', key: 'F7', cancelable: true, bubbles: true })); await new Promise(r => setTimeout(r, 1500));
    const db = await new Promise((res, rej) => { const q = indexedDB.open('twopointsix-kept'); q.onsuccess = () => res(q.result); q.onerror = rej; });
    out.sid = (await new Promise(res => { const q = db.transaction('visits').objectStore('visits').getAll(); q.onsuccess = () => res(q.result); }))[0]?.sid; return out;
  });
  check(whole(run.load), 'the first frame after a load has the whole of the game\'s smoke', run.load);
  check(run.load.cans.length === 1 && Math.abs(run.load.cans[0] - 4) < 0.3, 'the can still pours, as old as it is', run.load.cans);
  check(run.on.off < 0.08 && run.on.cans.length === 1, 'a tenth of a second on it is the game\'s still, and the can is one', run.on);
  check(whole(run.held), 'a load drawn with no time gone by has it too', run.held);
  check(run.load.heard.length === 1 && run.load.heard[0][0] === 'smokeHiss' && Math.abs(run.load.heard[0][1] - 4) < 0.3, 'and is heard, as old as it is', run.load.heard);
  check(run.dust.dyed > 0 && run.dust.hueOff < 12 && run.dust.dusty, 'dust has its color in the first frame after a load', { dyed: run.dust.dyed, off: run.dust.hueOff, of: 255, dusty: run.dust.dusty });
  check(run.restart.heard.length === 0, 'a restart is heard without it', run.restart.heard);
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
