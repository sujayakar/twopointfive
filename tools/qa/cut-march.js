// WHAT THE CUT RULE COSTS THE RAY PROGRAMS ON THIS MACHINE, WITH NOTHING CUT IN THE PICTURE — measured in the page, in about a minute.
// The page keeps two sets of its ray programs (render/cutSwap.ts): the boot's, without the cut rule, and a second set with it, taken only while a cut box a
// ray can meet is alive. On one NVIDIA card under Linux the set with the rule was found to cost the fog pass 1.9 times as much, the particles 1.6 times and
// the one-program gather 2.5 times with no cut box anywhere: the cost of the test being in the program at all. Whether another machine's compiler does the
// same nobody knows until it is asked. This asks it:
//   boot  http://localhost:5173/?levelFile=tut-team.json&roomclear&squad4&seed=1&noverdict&nokeep&mute&quality=high      (any mission's page will do; fog helps)
//   then in the console:   await import('/qa/cut-march.js')
// It holds the picture (the quality ladder off, the render scale as it stands, the game's clock stopped), waits for the second set, pins each set in turn
// three times round for four seconds a take, and prints ONE table: the median GPU ms of each pass that traces, base against cut, and the whole frame's.
// The same text is left in window.__cutMarch and, where the browser allows, on the clipboard. A page whose programs are not swapped (a look page with a
// geometry trial on, ?cutprograms=boot) has one set and nothing to compare: it says so.
// tools/qa/cut-cost.js is the long instrument (what cut PIECES cost); this is the one question, for a machine nobody here can sit at.
const w = window, sl = ms => new Promise(r => setTimeout(r, ms));
const Q = w.__quality, E = w.__engine, C = w.__cut, R = w.__perfrec;
const say = s => { console.log(`cut-march: ${s}`); };
const med = a => { a = a.filter(v => typeof v === 'number' && Number.isFinite(v)).sort((x, y) => x - y); return a.length ? a[a.length >> 1] : NaN; };
async function run() {
  if (!E || !C || !C.programs || !R) return 'cut-march: this page has no engine, no cut programs or no frame recorder (window.__engine, __cut.programs, __perfrec)';
  if (!C.programs.swap) return 'cut-march: this page runs ONE set of ray programs (a geometry trial of the look page, or ?cutprograms=boot): there is nothing to compare. Open a mission on the plain page.';
  const adaptive = Q ? Q.adaptive : null, time0 = w.__timeScale, visible0 = w.__testVisible;
  const back = () => { C.programs.force = null; w.__timeScale = time0 === undefined ? 1 : time0; w.__testVisible = visible0; if (Q) Q.adaptive = adaptive; };   // (the page as it was, whatever happens below)
  if (Q) Q.adaptive = false;
  say('waiting for the second set of programs …'); const ready = await C.programs.ready;
  if (!ready) { back(); return 'cut-march: the second set of programs did not build on this machine; nothing measured.'; }
  w.__testVisible = true; await sl(3000); w.__timeScale = 0; await sl(3000);
  const take = async set => { C.programs.force = set; await sl(1500); R.start('cut-march', 900); await sl(4000); const d = R.stop(false), fr = d && d.frames ? d.frames : [], P = {}; const ks = new Set(); for (const f of fr) for (const k in f.gpu) ks.add(k);
    for (const k of ks) P[k] = med(fr.map(f => f.gpu[k])); return { set, frames: fr.length, union: med(fr.map(f => f.gpuUnion === undefined ? f.gpuSum : f.gpuUnion)), P }; };
  const rows = [];
  try { for (let r = 0; r < 3; r++) for (const s of (r % 2 ? ['cut', 'base'] : ['base', 'cut'])) { say(`round ${r + 1} of 3, ${s}`); rows.push(await take(s)); } } finally { back(); }
  if (rows.some(x => x.frames < 8 || !(x.union > 0))) return `cut-march: too few timed frames to say anything (${rows.map(x => x.frames).join(', ')} in the six takes). Keep this tab in front and its window open for the minute it runs; if the frames are there and the times are not, this browser gives the page no GPU timer.`;
  const of = (set, f) => med(rows.filter(x => x.set === set).map(f)), names = [...new Set(rows.flatMap(x => Object.keys(x.P)))].filter(k => of('base', x => x.P[k]) > 0.02 || of('cut', x => x.P[k]) > 0.02);
  const line = (name, a, b) => `${name.padEnd(14)} ${a.toFixed(3).padStart(8)} ${b.toFixed(3).padStart(8)}   ${(b / Math.max(a, 1e-6)).toFixed(2)} x`;
  const info = E.gpu && typeof E.gpu.adapterInfo === 'string' ? E.gpu.adapterInfo : '?';
  const out = [`cut-march · ${info} · ${E.width}×${E.height} · ${location.search} · about ${rows[0].frames} frames a take, 3 takes a set`,
    `${'pass (GPU ms)'.padEnd(14)} ${'base'.padStart(8)} ${'cut'.padStart(8)}   cut / base`,
    ...names.sort((a, b) => of('cut', x => x.P[b]) - of('cut', x => x.P[a])).map(k => line(k, of('base', x => x.P[k]), of('cut', x => x.P[k]))),
    line('WHOLE FRAME', of('base', x => x.union), of('cut', x => x.union))].join('\n');
  w.__cutMarch = out; try { navigator.clipboard.writeText(out).catch(() => { }); } catch { /* no clipboard here */ }   // (not waited for: a page that is not in front never answers; the text is in window.__cutMarch and printed below either way)
  return out;
}
console.log(await run());
