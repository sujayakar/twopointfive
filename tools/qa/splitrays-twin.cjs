// SEPARATE SHADOW RAYS: THE TWIN (tools/qa/splitrays-check.ts and the pictures of docs/agents/render.md "Separate shadow rays"). The split gather (render/gatherSplit.ts)
// cannot draw the megakernel's own noise: there the light loop draws from the texel's one stream, here each lit point has a stream of its own (gather_split.wgsl gxFork).
// So the megakernel is not what the split is compared with picture for picture. This is: today's fgather.wgsl, text for text, with ONE change — each of `main`'s three
// directAtPoint calls (the rays' loop, the floor's mirror ray, the pane's) draws its numbers from that point's own stream, seeded exactly as `light` seeds it, and hands the
// texel's stream back as it found it. One megakernel, the split's random numbers: what is left between its picture and the split's is what the buffer rounds (the paint and
// the unlit radiance as f16, a slanted normal as 16 + 16 bits) and the order two sums are taken in.
//   as a module:   { text(root) → the twin's WGSL, fn, arg }   fn / arg = the step tools/qa/draw-equal.cjs runs on a booted page under DRAW_EQUAL_AFTER_READY=<this file>
//                  (the row OFF on that page: the twin goes in through the gather's own hot hook, engine.passes.hotSwapGather)
//   node tools/qa/splitrays-twin.cjs   prints the twin's text
// ONLY FOR A VIEW WITH NO CUT PIECE IN IT: the hot hook pins the boot programs (render/cutSwap.ts hot()), so the twin's page draws and traces a cut piece whole where the
// row's page takes the cut-box set — a difference that is not the split's. It also compiles the megakernel four times over a view (main and mainGi, both direction patterns).
const fs = require('fs');
const path = require('path');

/** the body of `fn name(` in `src`: from its line to the line before the next module-scope `fn` / `@compute` / the phase-5 banner */
function span(src, name) {
  const a = src.search(new RegExp(`^fn ${name}\\(`, 'm')); if (a < 0) throw new Error(`splitrays-twin: fgather.wgsl has no fn ${name}`);
  const rest = src.slice(a + 3), b = rest.search(/^(fn |@compute|\/\/ =====|var<|const |struct |\/\*\*|\/\/ ----)/m);
  return [a, b < 0 ? src.length : a + 3 + b];
}
function swap(src, name, from, to) {
  const [a, b] = span(src, name), body = src.slice(a, b), n = body.split(from).length - 1;
  if (n !== 1) throw new Error(`splitrays-twin: fn ${name} has ${n} of '${from}' where one was expected`);
  return src.slice(0, a) + body.replace(from, to) + src.slice(b);
}
function text(root = path.resolve(__dirname, '../..')) {
  const dir = path.join(root, 'src/render/shaders');
  let src = fs.readFileSync(path.join(dir, 'fgather.wgsl'), 'utf8');
  const split = fs.readFileSync(path.join(dir, 'gather_split.wgsl'), 'utf8');
  const fork = /^fn gxFork\([\s\S]*?^}\n/m.exec(split); if (!fork) throw new Error('splitrays-twin: gather_split.wgsl has no fn gxFork');
  src = swap(src, 'main', 'Eh += directAtPoint(Ph, hit.n, ex);', 'Eh += directAtPointX(Ph, hit.n, ex, k);');
  src = swap(src, 'glossReflection', 'Eh += directAtPoint(Ph, hit.n, 0u);', 'Eh += directAtPointX(Ph, hit.n, 0u, RAYS);');
  src = swap(src, 'glassShade', 'Eh += directAtPoint(Ph, hit.n, 0u);', 'Eh += directAtPointX(Ph, hit.n, 0u, RAYS + 1u);');
  src = swap(src, 'main', 'rngInit(gid.xy, frame.frameIdx);', 'rngInit(gid.xy, frame.frameIdx); gxSeed = rngState;');
  return `var<private> gxSeed: u32;\n${fork[0]}fn directAtPointX(P: vec3f, N: vec3f, ex: u32, j: u32) -> vec3f { let keep = rngState; rngState = gxFork(gxSeed, j); let E = directAtPoint(P, N, ex); rngState = keep; return E; }\n` + src;
}
module.exports = {
  text,
  fn: async (src) => { const r = await window.__engine.passes.hotSwapGather(src); return { ok: r.ok, messages: r.messages.slice(0, 6) }; },
  get arg() { return text(); },
};
if (require.main === module) process.stdout.write(text());
