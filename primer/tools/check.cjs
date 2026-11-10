// Is every shader right, in every room's variant (with triangles, with smoke)? The text is checked; nothing is run.
//   node tools/check.cjs
const { chromium } = require((process.env.PLAYWRIGHT ?? '/tmp/pw') + '/node_modules/playwright-core');
const path = require('path');
const FLAGS = [
  '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan',
  '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox',
];
(async () => {
  const browser = await chromium.launch({ executablePath: '/usr/bin/chromium', args: FLAGS, headless: true });
  const page = await browser.newPage({ viewport: { width: 1000, height: 300 } });
  page.on('pageerror', (e) => console.log('page error:', String(e.message).slice(0, 300)));
  await page.goto('file://' + path.resolve(__dirname, '../dist/index.html'));
  await page.waitForFunction('window.lab?.world?.room', null, { timeout: 30000 });
  const said = await page.evaluate(async () => {
    window.holdFrames = true;
    const { lab, page } = window;
    const bad = [];
    let most = 0;
    let count = 0;
    for (const [room, trace] of [['shapes', 0], ['smoke-window', 0], ['stone-yard', 2]]) {
      page.openRoom(room);
      window.setKnob('mesh_trace', trace);
      for (const v of lab.views.all()) if (!v.id.includes('.')) lab.views.bufferOf(v.id);
      for (const s of lab.graph.stages) {
        if (s.custom) continue;
        const { text, before } = lab.graph.source(s);
        const storage = (text.match(/var<storage/g) ?? []).length;
        if (storage > most) most = storage;
        if (storage > 8) bad.push(`${room}: ${s.name} binds ${storage} storage buffers`);
        const uniforms = (text.match(/var<uniform/g) ?? []).length;
        if (uniforms > 12) bad.push(`${room}: ${s.name} binds ${uniforms} uniform buffers`);
        const module = lab.gpu.device.createShaderModule({ code: text });
        const info = await module.getCompilationInfo();
        count++;
        for (const m of info.messages.filter((m) => m.type === 'error').slice(0, 2)) {
          bad.push(`${room}: ${s.name} line ${m.lineNum - before}: ${m.message.split('\n')[0]}\n    ${text.split('\n')[m.lineNum - 1].trim()}`);
        }
      }
      window.setKnob('mesh_trace', 0);
    }
    return `${count} shaders, at most ${most} storage buffers\n${[...new Set(bad)].join('\n')}`;
  });
  console.log(said);
  await browser.close();
})();
