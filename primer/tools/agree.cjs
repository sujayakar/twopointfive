// Does an answer agree with the truth in the mean? Runs many frames small, averages a buffer's field over them,
// and compares with a field of the truth: the whole picture's mean, and the error over blocks of 8 × 8.
//   node tools/agree.cjs '{"room":"lamp-hall","knobs":{…},"answer":"still_raw.value","truth":"truth.still","frames":64}' …
const { chromium } = require((process.env.PLAYWRIGHT ?? '/tmp/pw') + '/node_modules/playwright-core');
const path = require('path');
const FLAGS = [
  '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan',
  '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox',
];
(async () => {
  const browser = await chromium.launch({ executablePath: '/usr/bin/chromium', args: FLAGS, headless: true });
  const page = await browser.newPage({ viewport: { width: 1000, height: 320 } });
  page.on('console', (m) => m.type() === 'error' && console.log('page:', m.text().slice(0, 400)));
  page.on('pageerror', (e) => console.log('page error:', String(e.message).slice(0, 400)));
  await page.goto('file://' + path.resolve(__dirname, '../dist/index.html'));
  await page.waitForFunction('window.lab?.world?.room', null, { timeout: 30000 });
  await page.evaluate('window.holdFrames = true');
  for (const text of process.argv.slice(2)) {
    const shot = JSON.parse(text);
    const said = await page.evaluate(async (s) => {
      const { lab, page } = window;
      if (s.chapter !== undefined) page.go(s.chapter);
      if (s.room || s.sliders) page.openRoom(s.room ?? lab.world.room.id, s.sliders);
      for (const [k, v] of Object.entries({ render_scale: 0.3, ...s.knobs })) window.setKnob(k, v);
      if (s.camera) Object.assign(lab.camera, s.camera);
      for (const [stage, from, to] of s.edits ?? []) {
        const st = lab.graph.stages.find((x) => x.name === stage);
        lab.graph.edit(stage, lab.graph.codeOf(st).replace(from, to));
      }
      lab.layout = 'one';
      lab.playing = !!s.play;
      lab.shown[0] = 'hit_x.normal';
      const [aName, aKey] = s.answer.split('.');
      const [tName, tKey] = s.truth.split('.');
      const mean = async (name, key, frames, also) => {
        lab.alsoWanted = [name, ...(also ?? [])];
        lab.ready = false;
        const spec = lab.graph.spec(name);
        const n = lab.graph.elements(spec.size) * spec.struct.stride;
        let sum = null;
        for (let k = 0; k < 3000 && !lab.ready && !lab.graph.errors.size; k++) { lab.frame(); await new Promise((r) => setTimeout(r, 10)); }
        if (lab.graph.errors.size) return String([...lab.graph.errors]);
        for (let k = 0; k < (s.settle ?? 0); k++) { lab.frame(); if (k % 8 === 7) await lab.gpu.device.queue.onSubmittedWorkDone(); }
        for (let k = 0; k < frames; k++) {
          lab.frame();
          const data = await window.readBack(lab.gpu, lab.graph.buffer(name), 0, lab.graph.elements(spec.size) * spec.struct.stride);
          const f = new Float32Array(data.buffer);
          const step = spec.struct.stride / 4, at = spec.struct.offsets[key] / 4;
          const count = lab.graph.elements(spec.size);
          sum ??= new Float64Array(count * 3);
          for (let i = 0; i < count; i++) for (let c = 0; c < 3; c++) sum[i * 3 + c] += f[i * step + at + c] / frames;
        }
        return { values: sum, size: lab.graph.pixels(spec.size) };
      };
      const t0 = performance.now();
      const answer = await mean(aName, aKey, s.frames ?? 64);
      if (typeof answer === 'string') return answer;
      // the truth keeps its own mean: only the last frame of many is read
      lab.alsoWanted = [tName];
      for (let k = 0; k < 3000 && (!lab.ready || lab.truthCount < (s.paths ?? 256)); k++) { lab.frame(); if (k % 8 === 7) await lab.gpu.device.queue.onSubmittedWorkDone(); if (!lab.ready) await new Promise((r) => setTimeout(r, 10)); }
      const truth = await mean(tName, tKey, 1);
      if (typeof truth === 'string') return truth;
      const b = (v, i) => 0.2126 * v[i * 3] + 0.7152 * v[i * 3 + 1] + 0.0722 * v[i * 3 + 2];
      const [w, h] = answer.size, [tw, th] = truth.size;
      const B = s.block ?? 8;
      let sa = 0, st = 0, err = 0, all = 0;
      for (let y = 0; y + B <= h; y += B) for (let x = 0; x + B <= w; x += B) {
        let a = 0, t = 0;
        for (let j = 0; j < B; j++) for (let i = 0; i < B; i++) {
          a += b(answer.values, (y + j) * w + x + i) / (B * B);
          const tx = Math.floor((x + i + 0.5) * tw / w), ty = Math.floor((y + j + 0.5) * th / h);
          t += b(truth.values, ty * tw + tx) / (B * B);
        }
        sa += a; st += t; err += (a - t) ** 2; all += t * t;
      }
      return `${s.say ?? s.answer}: mean ${(100 * (sa / st - 1)).toFixed(2)} % off, blocks ${(100 * Math.sqrt(err / all)).toFixed(2)} % (${w} × ${h}, ${s.frames ?? 64} frames, ${lab.truthCount} paths, ${Math.round((performance.now() - t0) / 1000)} s)`;
    }, shot);
    console.log(said);
  }
  await browser.close();
})();
