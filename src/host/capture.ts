// F6, A FRAME CAPTURE (render/framecap.ts), on whichever page draws: every target the renderer wrote, the whole picture over 2 frames (Shift+F6: a window at the cursor over 8 frames, a tenth the
// size), with the trace of what the renderer changed and each frame's length. One file, to the dev server's captures/ folder (a page served from anywhere else hands it to the save
// door, host/savefile.ts); read with tools/qa/capture-read.ts. `__framecap.arm()` waits for the renderer's next change and captures from that frame.
import type { Engine } from '../engine';
import type { Gpu } from '../gpu/device';
import type { Quality } from '../ui/quality';
import { FRAMECAP, FrameCap } from '../render/framecap';
import { pageSay } from '../view/pagesaid';
import { saveFile, savedWords } from './savefile';

/** `said`: what becomes of a capture, in words */
export function frameCapture(gpu: Gpu, engine: Engine, quality: Quality, said: (words: string) => void = pageSay): FrameCap {
  const cap = new FrameCap(gpu.device, {
    state: () => ({ internal: `${engine.width}×${engine.height}`, out: `${engine.outWidth}×${engine.outHeight}`, scale: +engine.settings.renderScale.toFixed(3), quality: quality.label, ladder: quality.lastChange,
      taau: !!engine.settings.taau, sharp: engine.sharpOn, view: engine.settings.debugView, targets: `${engine.passes.reallocs}: ${engine.passes.lastRealloc}`, hidden: document.hidden }),
    out: () => [engine.outWidth || engine.width, engine.outHeight || engine.height],
    gpu: () => { const o: Record<string, number> = {}; for (const [k, v] of engine.timer.results) o[k] = +v.toFixed(3); return o; },
    parked: t => engine.pool.isParked(t), adapter: gpu.adapterInfo,
    save: async (name, gz) => {   // (the dev server is a thing of a page served from this machine: a page opened from anywhere else is asked for nothing but a download)
      if (/^(localhost|127\.0\.0\.1|\[::1\])$/.test(location.hostname)) try { const r = await fetch(`/capture?name=${name}`, { method: 'POST', body: gz as BodyInit, headers: { 'X-Frame-Capture': '1', 'Content-Type': 'application/octet-stream' } }); const m = r.ok ? /^captured(?: in (\d+) pieces)?$/.exec(await r.text()) : null; if (m) return `→ captures/${name}.tpscap.gz${m[1] ? ` · to hand it on: its ${m[1]} pieces, ${name}.part1of${m[1]} …` : ''}`; } catch { /* no dev server behind this page */ }
      const w = saveFile('capture', `${name}.tpscap.gz`, new Blob([gz as BlobPart]));   // (anywhere else it leaves by the page's one door, host/savefile.ts: to whoever keeps the page's files, and to the browser only with ?downloads=1)
      return w.local ? '→ your downloads' : savedWords('The capture', w.went);
    },
    said,
  });
  (window as unknown as { __framecap: FrameCap }).__framecap = cap;
  return cap;
}
/** was Shift down with the capture's key; the browser's own F6 is kept off the page */
export function captureKey(pressed?: (shift: boolean) => void): () => boolean {
  let shift = false;
  window.addEventListener('keydown', e => { if (FRAMECAP.on && e.code === 'F6' && !e.metaKey && !e.ctrlKey && !e.altKey) { shift = e.shiftKey; e.preventDefault(); if (!e.repeat) pressed?.(shift); } }, true);
  return () => shift;
}
