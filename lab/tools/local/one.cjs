// node one.cjs out.png '{view json}'
const { chromium } = require('playwright-core'), fs = require('fs');
(async () => {
  const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox'];
  const b = await chromium.launch({ executablePath: '/usr/bin/chromium', args: FLAGS, headless: true });
  const p = await b.newPage(); p.on('console', m => m.type() === 'error' && console.log('page:', m.text().slice(0, 900)));
  await p.goto('http://localhost:5999/shot.html'); await p.waitForFunction('window.shots');
  if (process.env.SCENE) await p.evaluate(s => { window.SO = { scene: s }; }, process.env.SCENE);
  for (let i = 2; i + 1 < process.argv.length; i += 2) { const r = await p.evaluate(async v => { try { window.S ??= await window.shots(window.SO ?? {}); return await window.S.shot(v); } catch (e) { return { err: String(e.stack || e) }; } }, JSON.parse(process.argv[i + 1])); if (r.err) { console.log(r.err.slice(0, 2000)); process.exitCode = 1; } else { fs.writeFileSync(process.argv[i], Buffer.from(r.png.split(',')[1], 'base64')); console.log(process.argv[i], r.ms.toFixed(0)); if (r.smokes) console.log(r.smokes); } }
  await b.close();
})();
