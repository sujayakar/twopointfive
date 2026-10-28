// The look prototype's "Window light" row (src/render/lookwindow.ts; engine.ts lookWindowLights keeps the lights): by day each window of a level is a light of
// its own, on the inside of its wall, as bright as what the glass looks out on. Here: which glass is a window and which way its light faces (on the pass's own
// boxes), how bright it is by the hour, and that the hour's rescale of lamps leaves it alone. No GPU: the picture itself is never SEEN here.
//   bun run tools/qa/lookwindow-check.ts   (exit 0 = all pass; LOOKWINDOW_BREAK=1 takes the open side for the room and must fail)
import { readFileSync } from 'fs';
import { resolve } from 'path';
import { LOOK_WINDOW, lookWindows, windowRadiance } from '../../src/render/lookwindow';
import { lightClass, lightDayK } from '../../src/render/photometry';
import { HOURS } from '../../src/game/hours';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKWINDOW_BREAK === '1';
const L = JSON.parse(readFileSync(resolve(import.meta.dir, '../../levels/pass.json'), 'utf8')) as { boxes: { c: number[]; h: number[]; yaw?: number; group?: string; flags?: string[] }[] };
const panes = L.boxes.filter(b => b.flags?.includes('Glass')).map(b => ({ c: b.c, h: b.h, yaw: b.yaw ?? 0 })), roofs = L.boxes.filter(b => b.group === 'ceiling');
const under = (p: readonly number[]) => roofs.some(r => Math.abs(p[0] - r.c[0]) < r.h[0] && Math.abs(p[2] - r.c[2]) < r.h[2] && r.c[1] > p[1]);
const roofed = (p: readonly number[]) => BREAK ? !under(p) : under(p);
const W = lookWindows(panes, roofed);
{ const inward = W.every(w => under([w.pos[0] + w.dir[0] * 0.5, w.pos[1], w.pos[2] + w.dir[2] * 0.5]) && !under([w.pos[0] - w.dir[0] * 0.8, w.pos[1], w.pos[2] - w.dir[2] * 0.8]));
  check(W.length >= 4 && W.length < panes.length && inward, `the pass's ${panes.length} panes are ${W.length} windows (panes side by side in one wall are one), and every one's light hangs under its room's roof and faces into it`);
  const tea = W.filter(w => Math.abs(w.pos[2] - (11 - LOOK_WINDOW.proud)) < 0.01 && w.pos[0] > 12 && w.pos[0] < 22), kit = W.filter(w => Math.abs(w.pos[2] - (11 - LOOK_WINDOW.proud)) < 0.01 && w.pos[0] > 22 && w.pos[0] < 28);
  check(tea.length === 2 && kit.length === 1 && Math.abs(tea[0].area - 1.8 * 0.95) < 0.02 && Math.abs(kit[0].area - 1.2 * 0.95) < 0.02 && kit[0].dir[2] === -1 && W.every(w => w.radius <= 0.5 && w.radius >= 0.3),
    `the tea room has two (${tea.map(w => w.area.toFixed(2)).join(' and ')} m² of glass), the kitchen one (${kit[0]?.area.toFixed(2)} m²), each a disk no wider than its glass is tall`); }
{ const mid = [{ c: [5, 1.4, 5], h: [0.5, 0.5, 0.006], yaw: 0 }], all = () => true, none = () => false;
  check(lookWindows(mid, all).length === 0 && lookWindows(mid, none).length === 0 && lookWindows([{ c: [5, 3, 5], h: [0.5, 0.006, 0.5], yaw: 0 }], p => p[2] > 5).length === 0 && lookWindows([{ c: [5, 1.4, 5], h: [0.5, 0.5, 0.2], yaw: 0 }], p => p[2] > 5).length === 0,
    'glass between two rooms, glass out of doors, a pane lying flat and a block of glass are no windows');
  const turned = lookWindows([{ c: [5, 1.4, 5], h: [0.5, 0.5, 0.006], yaw: Math.PI / 2 }], p => p[0] > 5);
  check(turned.length === 1 && Math.abs(turned[0].dir[0] - 1) < 1e-9 && Math.abs(turned[0].pos[0] - (5 + LOOK_WINDOW.proud)) < 1e-9, 'a pane turned a quarter faces along the other axis, into its room'); }
{ const sunOf = (h: typeof HOURS[keyof typeof HOURS]) => ({ intensity: h.sun.intensity, dir: [0, -Math.sin(h.sun.elevDeg * Math.PI / 180), 0], color: h.sun.color }), lum = (r: readonly number[]) => 0.2126 * r[0] + 0.7152 * r[1] + 0.0722 * r[2];
  const of = (id: keyof typeof HOURS, amount = 1) => lum(windowRadiance(HOURS[id].sky.zenith, HOURS[id].sky.horizon, 1, sunOf(HOURS[id]), amount));
  const sky = (L as unknown as { sky: { zenith: number[]; horizon: number[] } }).sky, night = lum(windowRadiance(sky.zenith, sky.horizon, 1, { intensity: 0.3, dir: [0, -0.6, 0], color: [0.6, 0.7, 1] }, 1));   // (the level's own night, under its moon)
  const grey = of('overcast'), noon = of('midday'), dusk = of('dusk'), kitA = 1.2 * 0.95;
  check(grey > 0.3 && grey < 1.2 && noon > grey && dusk > 0.15 && night === 0 && grey * kitA > 50 * LOOK_WINDOW.least, `a window is as bright as what it looks out on: ${grey.toFixed(2)} under the grey sky, ${noon.toFixed(2)} at midday, ${dusk.toFixed(2)} at dusk, and by night it is no light at all (${night})`);
  check(of('overcast', 0.5) === grey * 0.5 && of('overcast', 0) === 0, 'the row is × its light, and 0 is none'); }
{ const l = { name: 'window0', group: 'sky', kind: 3, ttl: -1, intensity: 0.6, peakIntensity: 0.6 };
  check(lightClass(l) === 'none' && lightDayK(l, 0.05, 1) === 1 && lightClass({ ...l, group: 'interior' }) === 'fixture', 'a window\'s light is the hour\'s own: the day\'s rescale of lamps leaves it as it is (a lamp of the same strength is taken down)'); }
process.exit(red ? 1 : 0);
