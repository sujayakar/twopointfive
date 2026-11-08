// THE PANEL'S ROWS AND THE ADDRESS'S WORDS ON THE PAGE: the game's page on the software adapter, stepped by hand. The module lists the rows, a turned row reads back, and an address word
// turned its number before the game was made. Never a GPU: what it holds is the list, its read and its write, and that nothing threw.
//   PORT=<a dev server> node tools/draw/rows-page.cjs      exit 0 = every check passed, 1 = a FAIL, 2 = the script died
const { chromium } = (() => { try { return require('playwright'); } catch { return require('/opt/node-tools/node_modules/playwright'); } })();
const PORT = process.env.PORT || 5199;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--no-sandbox'];
let fails = 0;
const check = (ok, what, got) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${got === undefined ? '' : `   (${JSON.stringify(got)})`}`); if (!ok) fails++; };
(async () => {
  const browser = await chromium.launch({ headless: true, executablePath: process.env.CHROMIUM || '/usr/bin/chromium', args: FLAGS });
  const page = await browser.newPage({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 }), errors = [];
  page.on('pageerror', x => errors.push(String(x && x.stack || x).slice(0, 500)));
  page.on('console', m => { const t = m.text(); if (m.type() === 'error' || /Invalid|error:/.test(t)) errors.push(t.slice(0, 300)); });
  await page.goto(`http://localhost:${PORT}/?paused&quality=verylow&mute&seed=5&levelFile=compound.json&roomclear&qtwice=0&knob.sneak.seen_rate=0.125`, { timeout: 300000 });
  await page.evaluate(() => window.__ready);
  const r = await page.evaluate(() => { const d = window.__draw, G = d.game, rows = G.shown(), now0 = [...G.shownNow()], at = rows.findIndex(x => x.name === 'tune.flash_intensity'), chip = rows.findIndex(x => x.name === 'pfx.chip_size');
    const K = () => new Map(G.knobs().map(k => [k.name, k.value])), k0 = K(); G.turn(rows[at].name, 123); G.turn(rows[chip].name, 0.02); G.tell(); d.run(2, [], 'last'); const now1 = [...G.shownNow()];
    const P = window.__panel, sliders = [...P.root.querySelectorAll('input[type=range]')].length, names = [...P.root.querySelectorAll('.row')].map(x => x.textContent).filter(t => /torch \/ unit light|p-rain:/.test(t)).length;
    return { n: rows.length, sections: [...new Set(rows.map(x => x.section))], before: now0[at], torch: now1[at], chipShown: now1[chip], twice: k0.get('cmdring.twice'), seen: k0.get('sneak.seen_rate'), halfOn: k0.get('cmdring.goal_half'), sliders, names }; });
  check(r.n >= 38 && r.sections.length >= 5, 'the module lists the rows', { n: r.n, sections: r.sections });
  check(r.torch === 123 && r.before !== 123, 'a row turned reads back', { before: r.before, after: r.torch });
  check(r.chipShown === 20, 'a row shows its number at its scale: a chip of 0.02 m is 20 mm', r.chipShown);
  check(r.twice === 0, '?qtwice=0 put the double tap off before the game was made', r.twice);
  check(r.seen === 0.125, '?knob.<name>=<number> turned that knob', r.seen);
  check(r.halfOn === 1, 'a word nobody said is left as it was', r.halfOn);
  check(r.names >= 2, 'the panel has the torch\'s row and the rain\'s', { rows: r.names, sliders: r.sliders });
  const A = await page.evaluate(() => { const d = window.__draw, G = d.game, rows = G.actions(), read = n => G.actions().find(a => a.name === n), P = window.__panel, button = (label, text) => [...P.root.querySelectorAll('.row')].filter(r => r.textContent.includes(label)).flatMap(r => [...r.querySelectorAll('button')]).find(b => text === undefined || b.textContent === text), press = (label, text) => { const b = button(label, text); if (b) b.click(); return !!b; };
    const nerve0 = read('nerve').now; G.pick('nerve', 'brittle'); G.tell(); d.run(2, [], 'last'); const nerve1 = read('nerve').now;
    const me = () => { const v = G.view(); return G.at(v.me); }; const was = me(); const smoke = press('smoke at the cursor'), teleport = press('teleport to the cursor'); d.run(2, [], 'last'); const moved = me();
    const lightsOff = press('all lights off'), strike = press('lightning now'), soak = press('soak the player'); d.run(30, [], 'last'); const lightsOn = press('all lights on'); d.run(2, [], 'last');
    return { n: rows.length, kinds: [...new Set(rows.map(a => a.kind))], nerve0, nerve1, smoke, teleport, lightsOff, strike, soak, lightsOn, was, moved, sections: [...new Set(rows.map(a => a.section))] }; });
  check(A.n >= 20 && A.kinds.length === 5, 'the module lists the actions, of every kind', { n: A.n, kinds: A.kinds, sections: A.sections });
  check(A.nerve1 === 'brittle' && A.nerve0 !== 'brittle', 'a pick goes as a command and its row reads it back', { was: A.nerve0, now: A.nerve1 });
  check(A.smoke && A.teleport && A.lightsOff && A.strike && A.soak && A.lightsOn, 'the panel has the buttons, and they press', A);
  check(errors.length === 0, 'nothing threw and no GPU error was said', errors.slice(0, 4));
  await browser.close();
  console.log(fails ? `=== rows-page: ${fails} FAILED ===` : '=== rows-page: all pass ===');
  process.exit(fails ? 1 : 0);
})().catch(e => { console.error(e); process.exit(2); });
