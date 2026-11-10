// THE REPLAYS THAT COVER THE GAME'S SOUNDS (public/assets/replays/, the sound page's 'In play' links): tapes of hands, each walked on its own page for the sounds it asks the mixer for, by tick.
//
//   PORT=<dev server> node tools/draw/sound-tapes.cjs [id …]                 walk the kept ones and write index.json
//   PORT=… node tools/draw/sound-tapes.cjs --again [id …]                    AFTER A CHANGE TO PLAY OR TO A LEVEL: the same hands played anew first (tools/draw/tape-again.ts), and what they no longer reach is said
//   PORT=… node tools/draw/sound-tapes.cjs --check [id …]                    walk them, write nothing: red if one parts from its tape or asks for other sounds
//   PORT=… node tools/draw/sound-tapes.cjs --add <tape | kept lines> <id> <title>    any kept run becomes one
//   node tools/draw/sound-tapes.cjs --cover                                  which of the mixer's sounds each plays, and which none reaches (no page)
//
// Headless Chromium on the software adapter; nothing is drawn while a tape is walked. Exit 0 / 1, 2 = the tool died.
const fs = require('fs'), path = require('path'), cp = require('child_process');
const ROOT = path.resolve(__dirname, '../..'), DIR = path.join(ROOT, 'public/assets/replays'), INDEX = path.join(DIR, 'index.json');
const PAGE = require('../qa/page.cjs').page({ name: 'sound tapes', port: process.env.PORT || '5173' }), args = process.argv.slice(2), has = w => args.includes(w);
/** a sound the mixer plays inside another's own call, which a page with no sound running never gets as far as asking for: filed with the sound that plays it */
const read = () => fs.existsSync(INDEX) ? JSON.parse(fs.readFileSync(INDEX, 'utf8')) : { tapes: [] };
const write = X => { fs.writeFileSync(INDEX + '.tmp', JSON.stringify(X, null, 1) + '\n'); fs.renameSync(INDEX + '.tmp', INDEX); };
/** every name of the mixer's table (src/audio/sfx.ts SfxName) */
const sfxNames = () => { const s = fs.readFileSync(path.join(ROOT, 'src/audio/sfx.ts'), 'utf8'), a = s.indexOf('export type SfxName'), b = s.indexOf('\n\n', a); return [...s.slice(a, b).replace(/\/\/.*$/gm, '').matchAll(/'([A-Za-z0-9]+)'/g)].map(m => m[1]); };
function cover() {
  const X = read(), all = sfxNames(), by = new Map(all.map(n => [n, []]));
  for (const T of X.tapes) for (const n of Object.keys(T.sounds)) (by.get(n) ?? by.set(n, []).get(n)).push(T.id);
  for (const T of X.tapes) console.log(`${T.id.padEnd(14)} ${String(Math.round(T.seconds)).padStart(4)} s  ${Object.keys(T.sounds).length} sounds: ${Object.keys(T.sounds).join(' ')}`);
  const none = all.filter(n => !by.get(n).length), odd = [...by.keys()].filter(n => !all.includes(n));
  console.log(`\n${all.length - none.length} of ${all.length} sounds are played by a replay.`);
  if (none.length) console.log(`in no replay: ${none.map(n => `${n}${X.why?.[n] ? ` (${X.why[n]})` : ''}`).join(' · ')}`);
  if (odd.length) console.log(`also asked for, made on the spot and not a row of the table: ${odd.join(' ')}`);
}
if (has('--cover')) { cover(); process.exit(0); }
const again = (from, to) => cp.execFileSync('bun', ['run', path.join(__dirname, 'tape-again.ts'), from, to], { cwd: ROOT, stdio: ['ignore', 'inherit', 'inherit'] });
(async () => {
  let X = read(), names = args.filter(a => !a.startsWith('--'));
  if (has('--add')) { const [from, id, ...title] = names, file = `${id}.tape.gz`; again(from, path.join(DIR, file)); X.tapes = [...X.tapes.filter(t => t.id !== id), { id, title: title.join(' ') || id, file, sounds: {} }]; write(X); names = [id]; }
  const todo = X.tapes.filter(t => !names.length || names.includes(t.id)); for (const n of names) if (!X.tapes.some(t => t.id === n)) { console.error(`no replay '${n}' (there are: ${X.tapes.map(t => t.id).join(' ')})`); process.exit(2); }
  for (const T of todo) {
    const file = path.join(DIR, T.file), was = { ...T };
    if (has('--again')) { again(file, file + '.new'); fs.renameSync(file + '.new', file); }
    const { context, page, errors, go } = await PAGE.open({ size: [320, 180], timeout: 900000, errors: { console: false } });
    await go(`paused&quality=verylow&mute&playtape=assets/replays/${T.file}`, { timeout: 300000 });
    await page.waitForFunction(() => !!window.__ready && (!!window.__draw || document.getElementById('fatal')?.style.display === 'flex')); await page.evaluate(() => window.__ready.catch(() => 0));
    const s = await page.evaluate(async () => {
      const d = window.__draw; if (!d) return { why: document.getElementById('fatal').textContent.slice(0, 200) };
      const P = d.played, heard = [], play = d.audio.play.bind(d.audio); d.audio.play = (n, ...r) => { heard.push([P.at, n]); return play(n, ...r); };
      let going = new Set(); const last = d.audio.lasting.bind(d.audio); d.audio.lasting = (rows, t) => { const now = new Set(rows.map(r => `${r.name} ${r.id}`)); for (const r of rows) if (!going.has(`${r.name} ${r.id}`)) heard.push([P.at, r.name]); going = now; return last(rows, t); };   // (one that goes on is asked for as it begins)
      while (P.at < P.total && P.lost === null && P.parted === null) { d.run(Math.min(240, P.total - P.at), [], 'none'); await new Promise(r => setTimeout(r, 0)); }
      return { at: P.at, total: P.total, parted: P.parted, lost: P.lost, heard };
    });
    await context.close();
    const sounds = {}; let asked = 0; for (const [f, n] of s.heard ?? []) { asked++; if (!(n in sounds)) sounds[n] = f; }
    const whole = !s.why && s.at === s.total && s.parted === null && s.lost === null && !errors.length, gone = Object.keys(was.sounds).filter(n => !(n in sounds)), fresh = Object.keys(sounds).filter(n => !(n in was.sounds));
    const ok = whole && (!has('--check') || (asked === was.asked && !gone.length && !fresh.length));
    PAGE.check(ok, `${T.id}: ${s.why ?? `${s.at} of ${s.total} ticks${s.parted !== null ? `, parted by tick ${s.parted}` : ', as it was played'}${s.lost ? `, lost: ${s.lost}` : ''}, ${asked} sounds asked${has('--check') ? ` (kept ${was.asked})` : ''}`}${gone.length ? ` · no longer asks for ${gone.join(' ')}` : ''}${fresh.length && was.asked ? ` · new: ${fresh.join(' ')}` : ''}${errors.length ? ` · ${errors[0]}` : ''}`);
    if (whole && !has('--check')) { X = read(); const i = X.tapes.findIndex(t => t.id === T.id); X.tapes[i] = { id: T.id, title: T.title, file: T.file, seconds: Math.round(s.total / 6) / 10, steps: s.total, bytes: fs.statSync(file).size, asked, sounds }; write(X); }
  }
  await PAGE.close(); if (!has('--check')) cover();
  await PAGE.end(PAGE.fails ? `\n=== sound tapes: ${PAGE.fails} FAILED ===` : '\n=== sound tapes: ALL PASS ===');
})().catch(e => { console.error(e); process.exit(2); });
