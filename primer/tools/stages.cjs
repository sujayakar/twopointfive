// How long each stage's shader takes to be made and first run on the software adapter, and what is wrong with
// any of them.   node tools/stages.cjs <chapter> [stage name part]
const { chromium } = require((process.env.PLAYWRIGHT ?? '/tmp/pw') + '/node_modules/playwright-core');
const path = require('path');
const FLAGS = [
  '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan',
  '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox',
];
(async () => {
  const browser = await chromium.launch({ executablePath: '/usr/bin/chromium', args: FLAGS, headless: true });
  const page = await browser.newPage({ viewport: { width: 1000, height: 300 } });
  page.on('console', (m) => console.log(m.text().slice(0, 500)));
  await page.goto('file://' + path.resolve(__dirname, '../dist/index.html'));
  await page.waitForFunction('window.lab?.world?.room', null, { timeout: 30000 });
  await page.evaluate(async ([chapter, part]) => {
    window.holdFrames = true;
    const { lab, page } = window;
    page.go(chapter);
    window.setKnob('render_scale', 0.25);
    const run = lab.graph.run.bind(lab.graph);
    let only = () => false;
    let wanted = [];
    lab.graph.run = (e, w) => run(e, wanted, only);
    const all = lab.shown.map((id) => lab.views.bufferOf(id));
    lab.frame();
    const stages = lab.graph.needed(all).filter((s) => !part || s.name.includes(part));
    for (const s of stages) {
      wanted = Object.values(s.writes);
      only = (name) => name === s.name;
      const t = performance.now();
      lab.ready = false;
      for (let k = 0; k < 3000 && !lab.ready && !lab.graph.errors.has(s.name); k++) { lab.frame(); await new Promise((r) => setTimeout(r, 10)); }
      const made = performance.now();
      lab.frame();
      await lab.gpu.device.queue.onSubmittedWorkDone();
      const ran = performance.now();
      console.log(`${s.name.padEnd(24)} made ${Math.round(made - t)} ms, first run ${Math.round(ran - made)} ms ${lab.graph.errors.get(s.name) ?? ''}`);
    }
  }, [Number(process.argv[2] ?? 0), process.argv[3]]);
  await browser.close();
})();
