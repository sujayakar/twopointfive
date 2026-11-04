// HOW NEAR A KERNEL IS TO THE SOFTWARE ADAPTER'S LIMIT, IN FORTY SECONDS (where a page boot is six minutes and says only that the GPU process died).
// The software adapter (SwiftShader) pastes every function a kernel calls into one routine and its compiler refuses a routine whose stack frame passes a fixed
// size ("Stack size limit exceeded"). Nothing reports it: the pipeline is made, the page boots with no error, and the GPU process dies of a null routine the first
// time the pass is dispatched ("WebGPU device lost: A valid external Instance reference no longer exists."). The final gather has stood within a hundredth of
// that limit for a long time, so a few lines added to anything it calls is enough. This is a limit of the adapter the cloud tests on, not of a GPU, but what
// grows the frame is the same pasted size a Windows shader compiler pays for (tools/qa/shaderinline-check.ts).
//
// What it does: loads the page (the look page by default) only as far as its shader modules and compute pipeline descriptors, which takes seconds, and keeps
// their texts AS THE PAGE MAKES THEM (tools/qa/wgsl-dump.ts's stand-in leaves page switches down and its texts are smaller). Then, a kernel at a time, in a
// fresh browser: makes that one pipeline with a layout read off the text's own declarations, binds zero-filled stand-ins, dispatches one workgroup. With gdb
// on the machine and symbols in the adapter's library it also reads the compiler's own numbers at its check: the frame, and the limit.
//   node tools/qa/kernel-stack.cjs                                   the final gather on the look page
//   node tools/qa/kernel-stack.cjs --kernel fgatherS/main,rcC0/rcC0  the kernels named (label/entry point, as --list prints them)
//   node tools/qa/kernel-stack.cjs --list                            every compute pipeline the page makes, with its text's size
//   node tools/qa/kernel-stack.cjs --query '&look=walls:0'           more of the page's URL · --plain = the game page, not the look page
//   node tools/qa/kernel-stack.cjs --text file.wgsl                  that text in place of the (one) kernel's own: for trying a cut before making it
// exit 0 = every kernel asked for was built and ran · 1 = one was not · 2 = no browser, no adapter or no such kernel
// Run it alone: it finds the GPU process as the newest one on the machine. Compute kernels only (a render pipeline needs its targets; the page's boot says those).
const { spawn, spawnSync } = require('node:child_process'); const fs = require('node:fs'); const net = require('node:net'); const path = require('node:path');
const { chromium } = require('playwright');
const ROOT = path.resolve(__dirname, '../..'); const argv = process.argv.slice(2), flag = (n, d) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : d; };
const QUERY = flag('--query', ''), LIST = argv.includes('--list'), ALT = flag('--text', ''), WANT = flag('--kernel', 'fgatherS/main').split(',');
const BASE = argv.includes('--plain') ? '?roomclear&squad4&levelFile=pass.json' : '?look&roomclear&squad4&levelFile=pass.json';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const freePort = () => new Promise((res, rej) => { const s = net.createServer(); s.unref(); s.on('error', rej); s.listen(0, '127.0.0.1', () => { const p = s.address().port; s.close(() => res(p)); }); });
const sleep = ms => new Promise(r => setTimeout(r, ms));

/** the page's module texts and compute pipeline descriptors, taken as it makes them (the page is left as soon as the count stops growing) */
async function capture() {
  const port = await freePort(); const srv = spawn('bun', ['run', 'tools/serve.ts'], { cwd: ROOT, env: { ...process.env, PORT: String(port) }, stdio: 'ignore' });
  try {
    let up = false; for (let i = 0; i < 240 && !up; i++) { try { up = (await fetch(`http://127.0.0.1:${port}/`)).ok; } catch { } if (!up) await sleep(250); }
    const browser = await chromium.launch({ args: FLAGS });
    try {
      const page = await browser.newPage({ viewport: { width: 640, height: 400 } });
      await page.addInitScript(() => { window.__mods = []; window.__pipes = []; const ids = new WeakMap();
        const o = GPUDevice.prototype.createShaderModule; GPUDevice.prototype.createShaderModule = function (d) { const m = o.call(this, d); ids.set(m, window.__mods.length); window.__mods.push(d.code); return m; };
        for (const k of ['createComputePipelineAsync', 'createComputePipeline']) { const p = GPUDevice.prototype[k]; GPUDevice.prototype[k] = function (d) { window.__pipes.push({ label: (d.label || '?') + '/' + d.compute.entryPoint, mod: ids.get(d.compute.module), entry: d.compute.entryPoint }); return p.call(this, d); }; } });
      await page.goto(`http://localhost:${port}/${BASE}&nofow&seed=1&paused&mute&quality=high${QUERY}`, { waitUntil: 'commit' });
      let n = -1; for (let i = 0; i < 60; i++) { await sleep(2000); const k = await page.evaluate(() => window.__pipes ? window.__pipes.length : 0).catch(() => 0); if (k > 0 && k === n) break; n = k; }
      const got = await page.evaluate(() => ({ mods: window.__mods, pipes: window.__pipes }));
      return got.pipes.map(p => ({ label: p.label, entry: p.entry, code: got.mods[p.mod] }));
    } finally { await browser.close().catch(() => { }); }
  } finally { srv.kill(); }
}

/** where the adapter's compiler compares a routine's frame with its limit, read off the library's own code: { at: offset in addProlog, a, b: the two operands as gdb names them } or null */
function findCheck() {
  if (spawnSync('gdb', ['--version']).status !== 0) return null;
  const lib = path.join(path.dirname(chromium.executablePath()), 'libvk_swiftshader.so'); if (!fs.existsSync(lib)) return null;
  const d = spawnSync('gdb', ['-batch', '-ex', "disassemble 'Ice::X8664::TargetX8664::addProlog(Ice::CfgNode*)'", lib], { encoding: 'utf8', maxBuffer: 1 << 26 });
  const lines = (d.stdout || '').split('\n'); const call = lines.findIndex(l => /call.*Ice3Cfg8setError|call.*Ice::Cfg::setError/.test(l)); if (call < 0) return null;
  for (let i = call; i >= 0 && i > call - 40; i--) { const m = lines[i].match(/<\+(\d+)>:\s+cmp\s+(0x[0-9a-f]+)\((%\w+)\),(%e\w+)/); if (m && /\bj(be|a|ae|b)\b/.test(lines[i + 1] || '')) return { at: +m[1], limit: `*(int*)($${m[3].slice(1)}+${m[2]})`, frame: `$${m[4].slice(1)}` }; }
  return null;
}

/** one kernel alone: { made, ran, err, frame, limit, why } */
async function runOne(k, check) {
  const tag = `--kernel-stack-run=${process.pid}-${Date.now()}`;   // (a word of this run's own on the browser's command line: its GPU process is found by its parent, never "the newest on the machine", which may be another page check's)
  const browser = await chromium.launch({ args: [...FLAGS, '--no-sandbox', '--disable-gpu-sandbox', tag] });   // (the sandbox off: a debugger may then attach to the GPU process)
  let dbg = null, dbgOut = '', script = null;
  try {
    const page = await browser.newPage();
    await page.route('https://kernel.test/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><title>kernel</title>' })); await page.goto('https://kernel.test/');
    if (!(await page.evaluate(async () => !!(navigator.gpu && await navigator.gpu.requestAdapter())))) return { none: true };
    if (check !== undefined && spawnSync('gdb', ['--version']).status === 0) {
      const pg = args => (spawnSync('pgrep', args, { encoding: 'utf8' }).stdout || '').trim().split('\n').filter(Boolean);
      const mine = pg(['-f', '--', tag]), pid = mine.flatMap(p => pg(['-P', p, '-f', 'type=gpu-process']))[0] ?? mine.find(p => /type=gpu-process/.test(spawnSync('ps', ['-o', 'args=', '-p', p], { encoding: 'utf8' }).stdout || ''));
      if (pid) { script = path.join(require('node:os').tmpdir(), `kernel-stack-${process.pid}.gdb`);
        fs.writeFileSync(script, ['handle SIGSEGV nostop noprint pass', 'handle SIGUSR1 nostop noprint pass',
          'break Ice::Cfg::setError', 'commands', 'silent', 'printf "WHY %s\\n", *(char**)$rsi', 'continue', 'end',
          ...(check ? [`break *('Ice::X8664::TargetX8664::addProlog(Ice::CfgNode*)' + ${check.at})`, 'commands', 'silent', `printf "FRAME %d of %d\\n", ${check.frame}, ${check.limit}`, 'continue', 'end'] : []), 'continue'].join('\n') + '\n');
        dbg = spawn('gdb', ['-p', pid, '-batch', '-x', script], { stdio: ['ignore', 'pipe', 'ignore'] }); dbg.stdout.on('data', b => { dbgOut += b; }); await sleep(2500); }
    }
    const res = await page.evaluate(async ([code, entry]) => {
      const adapter = await navigator.gpu.requestAdapter(); const want = {};
      for (const k of ['maxStorageBuffersPerShaderStage', 'maxComputeWorkgroupStorageSize', 'maxStorageTexturesPerShaderStage', 'maxSampledTexturesPerShaderStage', 'maxBindingsPerBindGroup', 'maxUniformBufferBindingSize', 'maxStorageBufferBindingSize', 'maxSamplersPerShaderStage', 'maxUniformBuffersPerShaderStage']) want[k] = adapter.limits[k];
      const device = await adapter.requestDevice({ requiredLimits: want, requiredFeatures: [...adapter.features].filter(f => ['float32-filterable', 'bgra8unorm-storage', 'rg11b10ufloat-renderable'].includes(f)) });
      let lost = null; device.lost.then(i => { lost = i.message; }); const errs = []; device.addEventListener('uncapturederror', e => errs.push(e.error.message.slice(0, 300)));
      // the layout, read off the text's own declarations: every binding a zero-filled stand-in of its kind
      const plain = code.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\/\/[^\n]*/g, ''); const groups = {};
      for (const d of plain.matchAll(/@group\((\d+)\)\s*@binding\((\d+)\)\s*var\s*(<[^>]*>)?\s*(\w+)\s*:\s*([^;]+);/g)) {
        const g = +d[1], b = +d[2], q = d[3] || '', ty = d[5].trim(); let lay, res;
        if (q.includes('uniform')) { lay = { buffer: { type: 'uniform' } }; res = { buffer: device.createBuffer({ size: Math.min(65536, device.limits.maxUniformBufferBindingSize), usage: GPUBufferUsage.UNIFORM }) }; }
        else if (q.includes('storage')) { lay = { buffer: { type: q.includes('read_write') ? 'storage' : 'read-only-storage' } }; res = { buffer: device.createBuffer({ size: 1 << 20, usage: GPUBufferUsage.STORAGE }) }; }
        else if (ty.startsWith('sampler_comparison')) { lay = { sampler: { type: 'comparison' } }; res = device.createSampler({ compare: 'less' }); }
        else if (ty.startsWith('sampler')) { lay = { sampler: { type: 'filtering' } }; res = device.createSampler({ magFilter: 'linear', minFilter: 'linear' }); }
        else if (ty.startsWith('texture_storage')) { const m = ty.match(/texture_storage_(\w+)<\s*(\w+)\s*,\s*(\w+)\s*>/); const dim = m[1] === '3d' ? '3d' : '2d', arr = m[1] === '2d_array';
          lay = { storageTexture: { access: { write: 'write-only', read: 'read-only', read_write: 'read-write' }[m[3]], format: m[2], viewDimension: arr ? '2d-array' : dim } };
          res = device.createTexture({ size: dim === '3d' ? [8, 8, 8] : [8, 8, arr ? 2 : 1], dimension: dim, format: m[2], usage: GPUTextureUsage.STORAGE_BINDING }).createView({ dimension: arr ? '2d-array' : dim }); }
        else if (ty.startsWith('texture_')) { const m = ty.match(/texture_(depth_)?(\w+?)(?:<\s*(\w+)\s*>)?$/); const st = m[1] ? 'depth' : m[3] === 'u32' ? 'uint' : m[3] === 'i32' ? 'sint' : 'float';
          const vd = { '2d': '2d', '3d': '3d', '2d_array': '2d-array', cube: 'cube' }[m[2]]; const fmt = m[1] ? 'depth32float' : st === 'uint' ? 'rgba32uint' : st === 'sint' ? 'rgba32sint' : 'rgba16float';
          lay = { texture: { sampleType: st, viewDimension: vd } }; res = device.createTexture({ size: vd === '3d' ? [8, 8, 8] : [8, 8, vd === '2d-array' ? 2 : vd === 'cube' ? 6 : 1], dimension: vd === '3d' ? '3d' : '2d', format: fmt, usage: GPUTextureUsage.TEXTURE_BINDING }).createView({ dimension: vd }); }
        else continue;
        (groups[g] = groups[g] || []).push({ b, lay, res });
      }
      const ng = Math.max(-1, ...Object.keys(groups).map(Number)) + 1, layouts = [], binds = [];
      for (let g = 0; g < ng; g++) { const es = (groups[g] || []).sort((x, y) => x.b - y.b);
        layouts.push(device.createBindGroupLayout({ entries: es.map(e => ({ binding: e.b, visibility: GPUShaderStage.COMPUTE, ...e.lay })) }));
        binds.push(device.createBindGroup({ layout: layouts[g], entries: es.map(e => ({ binding: e.b, resource: e.res })) })); }
      let pipe; try { pipe = await device.createComputePipelineAsync({ layout: device.createPipelineLayout({ bindGroupLayouts: layouts }), compute: { module: device.createShaderModule({ code }), entryPoint: entry } }); } catch (e) { return { made: false, err: String(e).slice(0, 400), errs }; }
      const enc = device.createCommandEncoder(); const p = enc.beginComputePass(); p.setPipeline(pipe); binds.forEach((b, i) => p.setBindGroup(i, b)); p.dispatchWorkgroups(1); p.end(); device.queue.submit([enc.finish()]);
      await Promise.race([device.queue.onSubmittedWorkDone().catch(() => { }), device.lost]); await new Promise(r => setTimeout(r, 400));
      return { made: true, ran: !lost && errs.length === 0, err: lost || errs[0] || '', errs };
    }, [k.code, k.entry]).catch(e => ({ made: true, ran: false, err: String(e).slice(0, 200) }));
    await sleep(300);
    const frames = [...dbgOut.matchAll(/FRAME (\d+) of (\d+)/g)].map(m => [+m[1], +m[2]]).sort((a, b) => b[0] - a[0]); const why = (dbgOut.match(/WHY (.*)/) || [])[1];
    return { ...res, frame: frames.length ? frames[0][0] : null, limit: frames.length ? frames[0][1] : null, why };
  } finally { if (dbg) dbg.kill('SIGKILL'); if (script) { try { fs.unlinkSync(script); } catch { /* gone already */ } } await browser.close().catch(() => { }); }
}

(async () => {
  const all = await capture(); if (!all.length) { console.error('kernel-stack: the page made no compute pipeline (no adapter?)'); process.exit(2); }
  if (LIST) { for (const k of all) console.log(String(Math.round(k.code.length / 1024)).padStart(4) + ' KB  ' + k.label); return; }
  const picks = []; for (const w of WANT) { const k = all.find(x => x.label === w); if (!k) { console.error(`kernel-stack: no compute pipeline named ${w} on this page (--list prints them)`); process.exit(2); } picks.push(k); }
  if (ALT) picks[0] = { ...picks[0], code: fs.readFileSync(ALT, 'utf8') };
  const check = findCheck(); let bad = 0;
  console.log(`\n=== kernel-stack: ${picks.length} kernel${picks.length > 1 ? 's' : ''} alone on the software adapter${check ? '' : ' (no debugger or no symbols: built or not, without the numbers)'} ===`);
  for (const k of picks) { const r = await runOne(k, check); if (r.none) { console.error('kernel-stack: no adapter'); process.exit(2); }
    const size = r.frame !== null && r.frame !== undefined ? `frame ${r.frame.toLocaleString('en-US')} of ${r.limit.toLocaleString('en-US')} bytes (${(100 * r.frame / r.limit).toFixed(1)} %)` : 'frame not read';
    const ok = r.made && r.ran; if (!ok) bad++;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${k.label}: ${size}; ${ok ? 'built and ran' : !r.made ? `no pipeline: ${r.err}` : `NOT BUILT${r.why ? ` (the compiler: "${r.why}")` : ''}: ${r.err}`}`); }
  console.log(`\n=== kernel-stack: ${bad ? 'FAILED' : 'ALL PASS'} ===`); process.exit(bad ? 1 : 0);
})().catch(e => { console.error(e); process.exit(2); });
