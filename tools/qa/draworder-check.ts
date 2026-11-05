// THE STATICS ARE DRAWN FROM THE TOP DOWN: the check behind scene/boxes.ts BoxWorld's drawRank (docs/agents/render.md has the rule and what was timed).
//   bun run tools/qa/probes.ts draworder        (family `render`)
//   DRAWORDER_BREAK=1   two statics' places in the order are swapped behind the world's back: must FAIL
// Why: the g-buffer drew the static boxes in the list's order, so a ground slab laid first under a whole level was shaded over the whole screen and then
// drawn over by the floor. Under the look that slab is a built surface, and the pass cost a third more wherever the camera stood. The camera looks down,
// so what stands higher is nearly always nearer: drawn tops falling, what lies under a thing fails the depth test before it is shaded.
// WHAT THIS CAN AND CANNOT SAY. No GPU runs here. It reads the list the g-buffer is handed (BoxWorld.visible) on a game stood up headless and holds its
// order to the rule, written here a second time. What the order saves is a GPU's to time; that a hidden fragment is not shaded is the depth test's.
import { standUp } from './headless.ts';
export async function drawOrderCheck(): Promise<boolean> {
  const BRK = process.env.DRAWORDER_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== draworder: the statics are drawn from the top down${BRK ? ' (two places swapped: must fail)' : ''} ===`);
  const W = await standUp(); const { game } = W; const w = (game.engine as any).world; const DT = 1 / 60;
  for (let i = 0; i < 3; i++) W.step(DT);
  if (BRK) { const r = w.drawRank as Uint32Array, vis = w.visible as Uint32Array, a = vis[0], b = vis[w.visibleCount - 1] < w.statics.length ? vis[w.visibleCount - 1] : vis[1]; const t = r[a]; r[a] = r[b]; r[b] = t; W.step(DT); }
  // the rule again: a static's top as the GPU has it (single floats), tops falling, the list's order between equal tops
  const top = (b: any) => Math.fround(Math.fround(b.c[1]) + Math.fround(b.rot ? (w.constructor as any).heightExtent(b) : b.h[1]));
  const read = () => { const n = w.statics.length as number, vis = Array.from((w.visible as Uint32Array).subarray(0, w.visibleCount)); const st = vis.filter(v => v < n); return { n, vis, st, lead: vis.slice(0, st.length) }; };
  { const { n, vis, st, lead } = read(); const boxes = w.all as any[]; const f: string[] = [];
    if (new Set(vis).size !== vis.length) f.push('a word is drawn twice');
    if (lead.some((v, k) => v !== st[k])) f.push('the drawn statics are not the head of the list');
    const dyn = vis.slice(st.length).map(v => v & 0xffff); if (dyn.some((v, k) => k > 0 && v <= dyn[k - 1])) f.push('the dynamics are not in the list\'s own order behind them');
    let out = 0, firstOut = ''; for (let k = 1; k < st.length; k++) { const a = boxes[st[k - 1]], b = boxes[st[k]], ta = top(a), tb = top(b); if (tb > ta || (tb === ta && st[k] < st[k - 1])) { if (!out++) firstOut = `place ${k}: box ${st[k - 1]} (top ${ta}) before box ${st[k]} (top ${tb})`; } }
    if (out) f.push(`${out} neighbours out of order, the first at ${firstOut}`);
    // what it is for: a thing that lies on another is drawn before it. The widest static (the ground under the level) comes after everything above its top
    let wide = st[0]; for (const i of st) if (boxes[i].h[0] * boxes[i].h[2] > boxes[wide].h[0] * boxes[wide].h[2]) wide = i;
    const above = st.filter(i => top(boxes[i]) > top(boxes[wide])), late = above.filter(i => st.indexOf(i) > st.indexOf(wide));
    if (late.length) f.push(`${late.length} statics above the widest one (box ${wide}) are drawn after it`);
    verdict(f.length === 0, 'the drawn statics lead the list with their tops falling and the list\'s own order between equal tops; the dynamics follow as ever; the widest static comes after all that stands above it',
      f.join('; ') || `${st.length} of ${n} statics drawn, ${dyn.length} dynamics behind them; the widest (box ${wide}, top ${top(boxes[wide])}) at place ${st.indexOf(wide) + 1} with ${above.length} above it drawn first`); }
  { // a top that moves is ranked again, and put back the order is the one it was; a static taken out of the picture leaves the rest as they were
    const before = read().st, boxes = w.all as any[], f: string[] = []; const low = before[before.length - 2], b = boxes[low], y0 = b.c[1];
    b.c = [b.c[0], y0 + 50, b.c[2]]; W.step(DT); const up = read().st; if (up[0] !== low) f.push(`box ${low} raised 50 m is at place ${up.indexOf(low) + 1}, not first`);
    if (up.filter(i => i !== low).some((v, k) => v !== before.filter(i => i !== low)[k])) f.push('raising one box moved others among themselves');
    b.c = [b.c[0], y0, b.c[2]]; W.step(DT); const back = read().st; if (back.length !== before.length || back.some((v, k) => v !== before[k])) f.push('put back, the order is not the one it was');
    const mid = before[before.length >> 1], fl = boxes[mid].flags; boxes[mid].flags = fl | 1; W.step(DT); const gone = read().st; boxes[mid].flags = fl; W.step(DT);
    const hidden = !gone.includes(mid); if (hidden && gone.some((v, k) => v !== before.filter(i => i !== mid)[k])) f.push('a static taken out of the picture moved the others');
    const again = read().st; if (again.some((v, k) => v !== before[k])) f.push('shown again, the order is not the one it was');
    verdict(f.length === 0, 'a static whose top moves is ranked again the same frame and the others keep their order; put back, the order is the one it was; so is it when a static leaves the picture and returns',
      f.join('; ') || `box ${low} raised: first of ${up.length}; back: place ${back.indexOf(low) + 1} as before; box ${mid} ${hidden ? 'hidden and shown' : 'kept its flag (the flag bit asked is not the picture\'s on this build)'}: the same ${again.length}`); }
  console.log(`=== draworder: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await drawOrderCheck()) ? 0 : 1);
