// THE RUN'S BASE ON THE REAL PAGE (src/game/savestate.ts SAVETIMELINE.base, Checkpoints.prime): headless Chromium on the software WebGPU adapter
// (tools/qa/page-look.cjs's recipe), a squad mission's own address loaded as a visitor loads it — no ?paused and no ?mute, either of which makes the page a
// script's and drops the brief — so its brief is up and the world is held behind it.
//   PORT=5199 node tools/qa/savebase-page.cjs      (a dev server on that port: PORT=5199 bun run tools/serve.ts; JOBS=on,off = only those jobs;
//                                                   LEVEL=<file>.json, default compound.json; AT=10,40,70 = the mission seconds the saves are asked at)
//   on    the page as it opens: with the brief up and the run's first tick not yet run, the save's code has been run once (the host's warm-up) and nothing
//         of it is kept; started with Enter, the run's first tick leaves its base on the shelf, and the first save of play is a take SINCE that base (and
//         so are the ones after it, but for one whole walk a longer run may force once the game has outgrown its chain); no page error
//   off   the same page under ?savebase=0: nothing warmed behind the brief, no base after the first tick, the first save of play the whole take, the
//         next one a take since it
// exit 0 = both jobs, with no page error · 1 = otherwise. SAVEBASE_BREAK=1 loads the `on` job with the switch off: it must FAIL.
// WHAT IT ALSO PRINTS, AND DOES NOT JUDGE: what each save cost in this Chromium (V8, the page's own bundle), and the run's first ticks. That is the
// measure of the switch on the page's engine: the first save of play with the base against without. A server box's one core, not a player's machine.
// THE FRAMES ARE NOT DRAWN: the software adapter takes minutes over one frame of a level and nothing here reads one. The saves are asked for at fixed
// mission seconds with the game's own autosave off and nobody hurt, so both jobs take theirs at the same moments of the same run.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', BREAK = process.env.SAVEBASE_BREAK === '1';
const LEVEL = process.env.LEVEL || 'compound.json', AT = (process.env.AT || '10,40,70').split(',').map(Number);
const ONLY = (process.env.JOBS || '').split(',').map(s => s.trim()).filter(Boolean);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
const JOBS = [{ name: 'on', query: '' }, { name: 'off', query: 'savebase=0&' }].filter(j => !ONLY.length || ONLY.includes(j.name));
if (!JOBS.length) { console.log(`- FAIL — no job is named ${ONLY.join(', ')}`); process.exit(2); }

// in the page, as the boot ends: nothing drawn, nobody hurt, and a clock round every update and every save in memory
const RIG = async () => {
  await window.__ready; const e = window.__engine; e.render = () => { e.world.upload(); };
  const g = window.__game, S = g.saves, shelf = S.shelf, R = window.__sb = { updates: [], takes: [] };
  g.godMode = true;   // (autosave stays ON until the run's first tick has run: a run with it off takes no base. It goes off once the run is started, below)
  const up = g.update.bind(g); g.update = (...a) => { const t0 = performance.now(); const r = up(...a); if (R.updates.length < 6) R.updates.push(+(performance.now() - t0).toFixed(1)); return r; };
  const wm = S.writeMem.bind(S); S.writeMem = (...a) => { const b = shelf.last, t0 = performance.now(), res = wm(...a), ms = performance.now() - t0; R.takes.push({ t: +g.time.toFixed(1), ms: +ms.toFixed(1), ok: !!res.ok, why: res.why || '', since: !!b && !!res.ok && shelf.get(res.info.key).snap.chain === b.chain }); return res; };
};
// in the page: what stands behind the brief
const BEHIND = () => { const g = window.__game, sh = g.saves.shelf, b = document.getElementById('brief');
  return { brief: !!b && b.classList.contains('on'), live: !!b && b.classList.contains('live'), t: g.time, updates: window.__sb.updates.length, begun: g.saves.missionRef === g.mission, warmed: sh.warmed, based: !!sh.last, units: g.guards.filter(x => x.unit).length }; };
// in the page: undrawn frames up to a mission second, then a save asked for as an autosave is written
const PLAY = (to) => { const g = window.__game; let n = 0; while (g.mission.stats.elapsed < to && n < 900) { window.__runFrames(1, 1 / 60, [], 'none'); n++; } if (g.mission.stats.elapsed < to) return { at: g.mission.stats.elapsed, done: false };
  const r = g.saves.write(g, 'auto', `asked at ${to}`); return { at: g.mission.stats.elapsed, done: true, ok: !!r.ok, why: r.why || '' }; };

(async () => {
  let red = 0; const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS }), told = [];
  for (const job of JOBS) {
    const page = await (await browser.newContext({ viewport: { width: 1024, height: 576 }, deviceScaleFactor: 1 })).newPage();
    const errs = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300))); page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errs.push(m.text().slice(0, 300)); });
    const on = job.name === 'on', query = (BREAK && on ? 'savebase=0&' : job.query) + `levelFile=${LEVEL}&roomclear&squad4`;
    // (settle=0: the brief is live the moment the level stands — its wait for the picture is a wait for drawn frames, and none is drawn here)
    const url = `http://localhost:${PORT}/?${query}&quality=${QUALITY}&settle=0`; log(`${job.name}: ${url}`);
    const r = {}; let ok = false, why = '';
    try {
      await page.goto(url, { waitUntil: 'commit' });
      await page.waitForSelector('#brief.on', { timeout: 180000 });
      await page.evaluate(RIG); log(` ${job.name}: ready`);
      await page.waitForSelector('#brief.live', { timeout: 240000 });
      await page.waitForTimeout(1500); r.behind = await page.evaluate(BEHIND);
      await page.keyboard.press('Enter');
      for (const end = Date.now() + 300000; Date.now() < end;) { if ((await page.evaluate(() => window.__game.time)) > r.behind.t) break; await page.waitForTimeout(300); }
      r.started = await page.evaluate(() => { const b = document.getElementById('brief'), g = window.__game, sh = g.saves.shelf; g.saves.enabled = false; return { brief: !!b && b.classList.contains('on'), t: g.time, based: !!sh.last && sh.last.game === g, warmed: sh.warmed }; });
      r.asked = [];
      for (const to of AT) { let p = { done: false }; for (let i = 0; i < 40 && !p.done; i++) p = await page.evaluate(PLAY, to); r.asked.push(p); }
      r.sb = await page.evaluate(() => ({ updates: window.__sb.updates, takes: window.__sb.takes }));
    } catch (e) { r.err = String(e).slice(0, 300); }
    await page.close();
    const takes = (r.sb && r.sb.takes) || [], B = r.behind || {};
    if (r.err) why = `${r.err}${errs.length ? ` · page errors: ${errs.slice(0, 3).join(' | ')}` : ''}`;
    else if (errs.length) why = `page errors: ${errs.slice(0, 3).join(' | ')}`;
    else {
      const held = B.brief && B.live && B.updates === 0 && !B.begun && !B.based && B.units > 0, went = !r.started.brief && r.started.t > B.t, all = takes.length === AT.length && takes.every(x => x.ok);   // (held: no update since the boot's end, and the run's first tick has not run — the controller has not met this run's mission)
      ok = held && went && all && r.started.warmed === B.warmed && (on ? B.warmed === 1 && r.started.based && takes[0].since && takes.filter(x => !x.since).length <= 1 : B.warmed === 0 && !r.started.based && !takes[0].since && takes.slice(1).filter(x => !x.since).length <= 1);   // (one whole walk after the first save is let by on either side: a chain the game has outgrown is walked again, by design — rollback.ts WALK_AGAIN)
      why = JSON.stringify({ behindTheBrief: { held, units: B.units, warmed: B.warmed, kept: B.based }, started: went, baseAfterTheFirstTick: r.started.based, saves: takes.map(x => `${x.t} s ${x.since ? 'since' : 'WHOLE'}${x.ok ? '' : ' REFUSED ' + x.why}`) });
    }
    if (!ok) red++;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${on ? `the page as it opens${BREAK ? ' (SAVEBASE_BREAK: the switch put off)' : ''}: behind the brief, before the run's first tick, the save's code has run once and kept nothing; that tick leaves the run's base; the first save of play and the ones after it are takes since it` : 'under ?savebase=0: nothing warmed, no base, the first save of play the whole take, the next a take since it'} — ${why}`);
    if (r.sb) told.push(`  ${job.name}: saves ${takes.map(x => `${x.ms} ms at ${x.t} s`).join(' · ') || 'none'} · the run's first updates ${r.sb.updates.join(', ')} ms`);
  }
  await browser.close();
  console.log(`what it cost here (this Chromium's V8, one run each, not judged):\n${told.join('\n')}`);
  console.log(`=== savebase-page: ${red ? `${red} FAILED` : 'ALL PASS'} ===`);
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
