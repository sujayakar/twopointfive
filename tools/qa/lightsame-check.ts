// A LIGHT THAT IS AS IT WAS (src/render/lights.ts slotInv, SLOT_SAME; restir.wgsl sameLight): the byte that tells the direct light where a lamp was in last frame's list, and whether its record is
// last frame's word for word, so that what it gives a point need not be worked out twice. Lights of every list (the level's, the goggles', the picture's own) come, go, move, dim, flicker, change their
// fog class and change places over some hundred uploads; after each, a byte says the slot its light had, and is marked exactly where the sixteen words are the sixteen it had, whatever slot it had them in.
//   bun run tools/qa/lightsame-check.ts        exit 0 · 1 a check failed
//   LIGHTSAME_BREAK=1 …      a light that moved is marked as one that stood: the second line must fail
//   LIGHTSAME_BREAK=2 …      a light that is new is marked too: the first must fail
import { fake } from './here.ts';
import { LightSet, MAX_LIGHTS, SLOT_SAME, type RtLight } from '../../src/render/lights';
import { mulberry32 } from '../../src/math/dice';

const BREAK = process.env.LIGHTSAME_BREAK ?? '', roll = mulberry32(23), card = new Uint32Array(MAX_LIGHTS * 16); let mine: unknown = null;
const device = new Proxy(fake('device'), { get: (t, p) => p === 'queue' ? { writeBuffer: (b: unknown, _at: number, data: ArrayBuffer, from = 0, size?: number) => { if (b === mine) card.set(new Uint32Array(data.slice(from, from + (size ?? data.byteLength - from)))); } } : p === 'createBuffer' ? (d: { size: number }) => ({ ...d, destroy() { /* none */ } }) : (t as Record<PropertyKey, unknown>)[p] }) as unknown as GPUDevice;
const set = new LightSet(device); mine = set.buf;
const lamp = (): RtLight => ({ name: 'l', group: 'g', kind: ([0, 1, 3] as const)[Math.floor(roll() * 3)], pos: [roll() * 40, 2, roll() * 28], dir: [0, -1, 0], color: [1, 1, 1], intensity: 5, range: 8, innerDeg: 20, outerDeg: 40, radius: 0.1, volumetric: 1, owner: 0, enabled: true, fixtureBox: -1, flicker: roll() < 0.15 ? 0.5 : 0, ttl: -1, age: 0, peakIntensity: 5 } as RtLight);
const own: RtLight[] = [lamp(), lamp()]; for (let i = 0; i < 24; i++) set.add(lamp()); set.ir.push(lamp(), lamp());
let was = new Map<RtLight, { rec: Uint32Array; slot: number }>(), places = 0, wrong = 0, marked = 0, asked = 0, changed = 0, elsewhere = 0, fresh = 0, full = 0;
for (let step = 0; step < 500; step++) {
  for (const l of [...set.lights, ...set.ir, ...own]) { if (roll() < 0.06) l.pos = [l.pos[0] + 0.01, l.pos[1], l.pos[2]]; if (roll() < 0.04) l.intensity = 4 + roll(); if (roll() < 0.02) l.dir = [roll(), -1, 0]; if (roll() < (l.enabled ? 0.01 : 0.2)) l.enabled = !l.enabled; if (roll() < 0.02) l.fog = l.fog ? 0 : 1; if (roll() < 0.02) l.glassMask = Math.floor(roll() * 0xffffff); if (roll() < 0.02) l.transient = !l.transient; }
  if (roll() < 0.4) set.add(lamp()); if (roll() < 0.12) set.remove(set.lights[Math.floor(roll() * set.lights.length)]); if (roll() < 0.004) while (set.lights.length > 20) set.remove(set.lights[0]); set.tube = step % 50 > 30;
  const n = set.upload(step / 60, own), now: typeof was = new Map(), nir = set.tube ? set.ir.length : 0; if (n === MAX_LIGHTS) full++;
  if (BREAK) for (let s = 0; s < n; s++) if (BREAK === '1' ? set.slotInv[s] !== 255 : true) set.slotInv[s] = BREAK === '2' && set.slotInv[s] === 255 ? SLOT_SAME : set.slotInv[s] | SLOT_SAME;
  for (let s = 0; s < n; s++) {
    const i = set.activeIdx[s], l = i < set.lights.length ? set.lights[i] : i < set.lights.length + nir ? set.ir[i - set.lights.length] : own[i - set.lights.length - nir], rec = card.slice(s * 16, s * 16 + 16), old = was.get(l), same = !!old && old.rec.every((w, k) => w === rec[k]), b = set.slotInv[s];
    now.set(l, { rec, slot: s }); asked++; if (!old) fresh++; else if (!same) changed++; else { marked++; if (old.slot !== s) elsewhere++; }
    if (old ? b === 255 || (b & 63) !== old.slot : b !== 255) places++;
    if (((b & 192) === SLOT_SAME) !== same) wrong++;   // (restir.wgsl sameLight's own test)
  }
  was = now;
}
let red = 0; const line = (ok: boolean, what: string, got: string): void => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${got}`); };
line(places === 0 && fresh > 100 && full > 10, 'a byte is the slot its light had, or 255 where it had none', `${places} wrong of ${asked} (${fresh} new; the list full on ${full} uploads)`);
line(wrong === 0 && marked > 1000 && changed > 500 && elsewhere > 50, 'and is marked exactly where the record is last frame\'s', `${wrong} wrong (${marked} as they were, ${elsewhere} of them in another slot, ${changed} not)`);
console.log(red ? `FAIL — ${red}` : 'PASS'); process.exit(red ? 1 : 0);
