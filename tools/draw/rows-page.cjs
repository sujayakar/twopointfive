// THE PANEL'S ROWS AND THE ADDRESS'S WORDS ON THE PAGE: the game's page on the software adapter, stepped by hand. The module lists the rows, a turned row reads back, and an address word
// turned its number before the game was made. Never a GPU: what it holds is the list, its read and its write, and that nothing threw.
//   PORT=<a dev server> node tools/draw/rows-page.cjs      exit 0 = every check passed, 1 = a FAIL, 2 = the script died
const PAGE = require('../qa/page.cjs').page({ name: 'rows-page', detail: 'json' }), { check } = PAGE;
(async () => {
  const { page, errors, go, ready } = await PAGE.open({ errors: { also: /Invalid|error:/ } });
  await go(`paused&quality=verylow&mute&seed=5&levelFile=compound.json&roomclear&qtwice=0&knob.sneak.seen_rate=0.125`, { timeout: 300000 });
  await ready();
  const r = await page.evaluate(() => { const d = window.__draw, G = d.game, rows = G.shown(), now0 = [...G.shownNow()], at = rows.findIndex(x => x.name === 'tune.flash_intensity'), chip = rows.findIndex(x => x.name === 'pfx.chip_size');
    const K = () => new Map(G.knobs().map(k => [k.name, k.value])), k0 = K(); G.turn(rows[at].name, 123); G.turn(rows[chip].name, 0.02); G.tell(); d.run(2, [], 'last'); const now1 = [...G.shownNow()]; const said = G.turnedNow;
    const P = window.__panel, sliders = [...P.root.querySelectorAll('input[type=range]')].length, names = [...P.root.querySelectorAll('.row')].map(x => x.textContent).filter(t => /torch \/ unit light|p-rain:/.test(t)).length;
    return { n: rows.length, sections: [...new Set(rows.map(x => x.section))], before: now0[at], torch: now1[at], chipShown: now1[chip], said, twice: k0.get('cmdring.twice'), seen: k0.get('sneak.seen_rate'), halfOn: k0.get('cmdring.goal_half'), sliders, names }; });
  check(r.n >= 38 && r.sections.length >= 5, 'the module lists the rows', { n: r.n, sections: r.sections });
  check(r.torch === 123 && r.before !== 123, 'a row turned reads back', { before: r.before, after: r.torch });
  check(r.said['tune.flash_intensity'] === 123, 'a play record is told what was turned', r.said);
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
  // WHAT THIS MACHINE KEEPS, ON A RESTART AND A LOAD (host/machine.ts): a row and a pick in the stores, a new page, and they stand on its game, go over a restart and a load, and are what a play record says.
  // CONTROL: the same row with the address's own word for its number: the word stands over a restart too, and the record does not say the kept number.
  const ROW = 'wet.prints', KEPT = 17, stand = async words => {
    await page.evaluate(([row, v]) => { localStorage.setItem('twopointsix.rows', JSON.stringify({ [row]: v })); sessionStorage.setItem('twopointsix.picks', JSON.stringify({ nerve: 'brittle' })); }, [ROW, KEPT]);
    await go(`paused&quality=verylow&mute&seed=5&levelFile=compound.json&roomclear${words}`, { timeout: 300000 }); await ready(); };
  const seen = () => page.evaluate(row => { const G = window.__draw.game; return { row: G.shownNow()[G.shown().findIndex(x => x.name === row)], nerve: G.actions().find(a => a.name === 'nerve').now, said: G.turnedNow, time: G.time }; }, ROW);
  const go2 = (how, n = 30) => page.evaluate(([how, n]) => { const d = window.__draw, C = d.game.commands; if (how === 'restart') C.restart(''); else if (how === 'save') C.save(); else if (how === 'load') C.load('manual.0'); d.game.tell(); d.run(n, [], 'last'); }, [how, n]);
  await stand('');
  const kept0 = await seen(); await go2('restart'); const kept1 = await seen(); await go2('save', 60); await go2('load'); const kept2 = await seen(), saved = await page.evaluate(() => window.__draw.game.saves().rows.length);
  check(kept0.row === KEPT && kept0.nerve === 'brittle', 'a kept row and a kept pick stand on the game a new page stands', kept0);
  check(kept0.said[ROW] === KEPT / 1 && kept0.said.nerve === 'brittle', 'a play record says what is kept', kept0.said);
  check(kept1.row === KEPT && kept1.nerve === 'brittle', 'they go over a restart', kept1);
  check(saved > 0 && kept2.row === KEPT && kept2.nerve === 'brittle', 'they go over a load', { saved, ...kept2 });
  await stand(`&knob.${ROW}=3`);
  const word0 = await seen(); await go2('restart'); const word1 = await seen();
  check(word0.row === 3 && word1.row === 3 && !(ROW in word0.said) && !(ROW in word1.said), 'the address\'s word is over the kept row, over a restart too, and no record says the kept number', { word0, word1 });
  await page.evaluate(() => { localStorage.removeItem('twopointsix.rows'); sessionStorage.removeItem('twopointsix.picks'); });
  check(errors.length === 0, 'nothing threw and no GPU error was said', errors.slice(0, 4));
  await PAGE.end(PAGE.fails ? `=== rows-page: ${PAGE.fails} FAILED ===` : '=== rows-page: all pass ===');
})().catch(e => { console.error(e); process.exit(2); });
