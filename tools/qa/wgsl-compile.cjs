// DOES EVERY SHADER TEXT COMPILE (a minute, where a page boot on the software adapter is six). tools/qa/wgsl-dump.ts writes every module text the page's boot
// makes (the cut-box set too, with the switches main.ts raises: tools/qa/pageboot.ts) under one setting of the switches; this asks for every setting the tree
// has (the title, a snow level in its weather, the look prototype's page), each with the shader rows as written and as the page opens, hands each distinct
// text to a headless Chromium's WGSL compiler (createShaderModule + getCompilationInfo: no pipeline is made, so it is quick) and prints every error AND EVERY
// WARNING with its line of the text (the page files each warning as a report, and a text the fold of constant switches cut badly says so first as 'code is
// unreachable'). It says a text PARSES AND TYPE-CHECKS. It does not say a pipeline can be made of it (bindings, limits), which is still a page boot's to say
// (tools/qa/shader-count-page.cjs).
//   node tools/qa/wgsl-compile.cjs            exit 0 = every text compiles with no warning, 1 = an error or a warning (printed), 2 = no browser or no adapter
//   WGSL_BREAK=1 node tools/qa/wgsl-compile.cjs      one text is given a line that cannot compile: must FAIL
const { spawnSync } = require('node:child_process'); const fs = require('node:fs'); const os = require('node:os'); const path = require('node:path');
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const ROOT = path.resolve(__dirname, '../..'), JSON_AT = path.join(os.tmpdir(), `wgsl-dump-${process.pid}.json`);
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
(async () => {
  // two dumps a setting: the shader rows are read once at import (ui/shaderrows.ts), so the texts of the rows as the page opens (the compact writings, the split
  // gather, the first-frame programs) come from a run of their own with each row's word in the environment. One dump with the rows flipped after the import
  // compiled none of the compact writings and said nothing.
  const ROWS = { TPS_COMPACTSHADERS: '1', TPS_SPLITRAYS: '1', TPS_CUTWHENNEEDED: '1', TPS_FIRSTFRAME: '1', TPS_SHADERSWHENNEEDED: '1' };
  const mods = [], seen = new Set();
  const SETTINGS = ['title', 'snow', ...(/\blookConfig\b/.test(fs.readFileSync(path.join(ROOT, 'src/render/frame.ts'), 'utf8')) ? ['look'] : [])];
  for (const setting of SETTINGS) for (const [label, env] of [['', {}], ['rows on', ROWS]]) {
    const d = spawnSync('bun', ['run', path.join(ROOT, 'tools/qa/wgsl-dump.ts'), JSON_AT, '--setting', setting, ...(label ? [label] : [])], { cwd: ROOT, encoding: 'utf8', env: { ...process.env, ...env } });
    if (d.status !== 0) { console.error(d.stdout, d.stderr); process.exit(2); }
    for (const m of JSON.parse(fs.readFileSync(JSON_AT, 'utf8'))) if (!seen.has(m.code)) { seen.add(m.code); mods.push(m); }
    fs.unlinkSync(JSON_AT);
  }
  if (!mods.some(m => /, rows on\)/.test(m.label))) { console.error('wgsl-compile: the run with the shader rows on made no text of its own: the compact writings were not compiled'); process.exit(2); }
  if (process.env.WGSL_BREAK === '1') mods[0].code += '\nfn wgslBreak() -> f32 { return aNameNothingDeclares; }';   // (the break path: one text is given a line that cannot compile, and the run must FAIL)
  const browser = await chromium.launch({ args: FLAGS });
  try {
    const page = await browser.newPage();
    await page.route('https://wgsl.test/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><title>wgsl</title>' }));   // (a secure page: navigator.gpu is not on about:blank)
    await page.goto('https://wgsl.test/');
    const res = await page.evaluate(async mods => {
      if (!navigator.gpu) return { none: 'no navigator.gpu' };
      const adapter = await navigator.gpu.requestAdapter(); if (!adapter) return { none: 'no adapter' };
      const want = {}; for (const k of ['maxStorageBuffersPerShaderStage', 'maxComputeWorkgroupStorageSize', 'maxStorageTexturesPerShaderStage', 'maxSampledTexturesPerShaderStage', 'maxBindingsPerBindGroup', 'maxUniformBufferBindingSize']) want[k] = adapter.limits[k];
      const device = await adapter.requestDevice({ requiredLimits: want, requiredFeatures: [...adapter.features].filter(f => ['float32-filterable', 'bgra8unorm-storage', 'rg11b10ufloat-renderable', 'timestamp-query'].includes(f)) });
      const out = [];
      for (const m of mods) { const info = await device.createShaderModule({ code: m.code }).getCompilationInfo(); const lines = m.code.split('\n');
        for (const e of info.messages) if (e.type === 'error' || e.type === 'warning') out.push({ label: m.label, type: e.type, at: e.lineNum, msg: e.message, line: (lines[e.lineNum - 1] || '').slice(Math.max(0, e.linePos - 80), e.linePos + 120) }); }
      return { out };
    }, mods);
    if (res.none) { console.error(`wgsl-compile: ${res.none}`); process.exit(2); }
    for (const e of res.out.slice(0, 20)) console.log(`- FAIL — ${e.label} line ${e.at} (${e.type}): ${e.msg}\n         … ${e.line.trim()} …`);
    console.log(res.out.length ? `wgsl-compile: ${res.out.filter(e => e.type === 'error').length} errors and ${res.out.filter(e => e.type === 'warning').length} warnings in ${new Set(res.out.map(e => e.label)).size} of ${mods.length} texts` : `- PASS — all ${mods.length} distinct shader texts of the page's boot compile, with no warning (${SETTINGS.join(', ')}; the rows as written and as the page opens)\nwgsl-compile: all passed`);
    process.exitCode = res.out.length ? 1 : 0;
  } finally { await browser.close(); }
})().catch(err => { console.error(err); process.exit(2); });
