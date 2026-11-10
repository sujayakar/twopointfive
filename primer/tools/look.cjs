// Opens the built page on the software adapter, drives it, and saves pictures. No GPU needed.
//   PLAYWRIGHT=<folder with playwright-core> node tools/look.cjs out.png '{"chapter":1,"frames":64}' [out2.png '{…}' …]
// A shot: { chapter, room, sliders, layout, views, knobs: {key: value}, frames, size: [w, h], pin: [u, v], play, whole }
const { chromium } = require((process.env.PLAYWRIGHT ?? '/tmp/pw') + '/node_modules/playwright-core');
const path = require('path');
const FLAGS = [
  '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan',
  '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox',
];
(async () => {
  const browser = await chromium.launch({ executablePath: '/usr/bin/chromium', args: FLAGS, headless: true });
  const page = await browser.newPage({ viewport: { width: 1100, height: 620 } });
  let bad = 0;
  page.on('console', (m) => { if (m.type() === 'error') { bad++; console.log('page:', m.text().slice(0, 400)); } });
  page.on('pageerror', (e) => { bad++; console.log('page error:', String(e.message + e.stack).slice(0, 1500)); });
  await page.goto('file://' + path.resolve(__dirname, '../dist/index.html'));
  await page.waitForFunction('window.lab?.world?.room', null, { timeout: 30000 });
  await page.evaluate('window.holdFrames = true');
  for (let i = 2; i + 1 < process.argv.length; i += 2) {
    const shot = JSON.parse(process.argv[i + 1]);
    if (shot.size) await page.setViewportSize({ width: shot.size[0], height: shot.size[1] });
    await page.waitForTimeout(100);
    const took = await page.evaluate(async (s) => {
      const { lab, page } = window;
      if (s.chapter !== undefined) page.go(s.chapter);
      if (s.room) page.openRoom(s.room, s.sliders);
      for (const [k, v] of Object.entries(s.knobs ?? {})) window.setKnob(k, v);
      if (s.layout) lab.layout = s.layout;
      (s.views ?? []).forEach((v, i) => (lab.shown[i] = v));
      if (s.camera) Object.assign(lab.camera, s.camera);
      if (s.pin) lab.pinned = s.pin;
      lab.playing = !!s.play;
      page.draw();
      lab.ready = false;
      const start = performance.now();
      for (let k = 0; k < 2000 && !lab.ready && !lab.graph.errors.size; k++) { lab.frame(); await new Promise((r) => setTimeout(r, 10)); }
      for (let k = 0; k < (s.frames ?? 1); k++) { lab.frame(); if (k % 8 === 7) await lab.gpu.device.queue.onSubmittedWorkDone(); }
      await lab.gpu.device.queue.onSubmittedWorkDone();
      if (s.run) window.said = await eval(s.run);
      await new Promise((r) => requestAnimationFrame(() => { lab.frame(); requestAnimationFrame(r); }));
      return { ms: performance.now() - start, errors: [...lab.graph.errors], paths: lab.truthCount, ran: lab.graph.ran, said: window.said };
    }, shot);
    await page.waitForTimeout(250);
    const target = shot.whole === false ? page.locator('#stage') : page;
    await target.screenshot({ path: process.argv[i] });
    for (const [stage, text] of took.errors) { bad++; console.log(`${stage}: ${text}`); }
    console.log(process.argv[i], Math.round(took.ms) + ' ms', took.paths + ' paths', took.ran.join(' > '), took.said === undefined ? '' : '\n  ' + JSON.stringify(took.said));
  }
  await browser.close();
  if (bad) process.exitCode = 1;
})();
