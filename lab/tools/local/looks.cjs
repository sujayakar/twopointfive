const { chromium } = require('playwright-core');
(async () => {
  const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox'];
  const b = await chromium.launch({ executablePath: '/usr/bin/chromium', args: FLAGS, headless: true }); let bad = 0;
  const p = await b.newPage({ viewport: { width: 1280, height: 760 } }); p.on('console', m => { if (m.type() === 'error' && !/404/.test(m.text())) { bad++; console.log('page:', m.text().slice(0, 900)); } }); p.on('pageerror', e => { bad++; console.log('error:', String(e.stack).slice(0, 1500)); });
  await p.goto('http://localhost:5999/index.html?px=14000&scale=0.6'); await p.waitForFunction('window.lab && window.lab.frameNo > 2', null, { timeout: 200000 });
  await p.evaluate(() => { teach.pace = 2; }); await p.keyboard.press('t'); await p.waitForTimeout(1500);
  const r = await p.evaluate(async () => { const out = []; teach.pace = 2; for (const s of teach.steps) for (const l of s.looks) { if (!l.code) continue; teach.stageOf(l); for (let i = 0; i < 900 && !(teach.built.get(l) && teach.built.get(l) !== 'making'); i++) await new Promise(r => setTimeout(r, 50)); out.push([s.title, l.name, teach.built.get(l) === 'making' ? 'NOT BUILT' : 'ok'].join(' / ')); } teach.partsOf(); for (let i = 0; i < 900 && !teach.partsDone; i++) await new Promise(r => setTimeout(r, 50)); out.push('parts ' + teach.partsDone); out.push('first at the end: ' + (teach.built.get(teach.steps[0].looks[0]) === 'making' ? 'NOT BUILT' : 'built')); return out.slice(-3).concat(out.filter(x => /NOT/.test(x))); });
  console.log(r.join('\n')); if (r.slice(0, 3).some(x => /NOT|false/.test(x))) bad++; console.log('error:', await p.evaluate(() => document.getElementById('error').textContent)); await b.close(); process.exit(bad ? 1 : 0);
})().catch(e => { console.error(e); process.exit(2); });
