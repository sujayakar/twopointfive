// A PARKED VEHICLE'S LAMPS ARE FOUND BY THE RIGHT SIGNS (tools/levels/lib/carLamps.ts, which writes a level's `vehicleLamps`): boxes made up here, a pair of each sign and what a pair is not. That the files there are
// name today's lamps is exports-check's.
//   bun run tools/qa/carlamps-check.ts      exit 0 = every line holds · 1 = one does not
//   CARLAMPS_BREAK=twin   a twin may stand anywhere:                the twin out of place, and the one higher, must FAIL
//   CARLAMPS_BREAK=fit    a lens may be any size, …=tone any paint or glow: the near misses of it must FAIL
import type { Box } from '../../src/scene/boxes';
import { carLampsIn, LAMP_FIT, lampsOf, sigs, type LampSig } from '../levels/lib/carLamps';

const BREAK = process.env.CARLAMPS_BREAK ?? '', K = { ...LAMP_FIT, ...(BREAK === 'twin' ? { twin: 99 } : BREAK === 'fit' ? { fit: 99 } : BREAK === 'tone' ? { tone: 99 } : {}) };
if (BREAK && !['twin', 'fit', 'tone'].includes(BREAK)) { console.error(`carlamps-check: no fault is called ${BREAK} (twin, fit, tone)`); process.exit(2); }
let fails = 0; const line = (ok: boolean, what: string, why = ''): void => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${why ? `\n    ${why}` : ''}`); };
/** a lens of a sign, standing at (x, z) */
const lens = (s: LampSig, x: number, z: number, over: Partial<Box> = {}): Box => ({ c: [x, 0.7, z], h: [...s.h], yaw: 0.3, albedo: s.albedo ? [...s.albedo] : [0.2, 0.02, 0.02], emissive: s.emissive ? [...s.emissive] : [0, 0, 0], ...over }) as unknown as Box;
const wall = { c: [0, 1.5, 9], h: [4, 1.5, 0.12], yaw: 0, albedo: [0.5, 0.5, 0.5], emissive: [0, 0, 0] } as unknown as Box;
const SIGNS = sigs(), said = (s: LampSig): string => `${s.what} ${s.h.join(' × ')}`;
/** how many of a pair of `s` are found with its second lens changed */
const found = (s: LampSig, second: (b: Box) => Box | null): number => { const b = second(lens(s, 5, 2 + s.gap)); return carLampsIn([wall, lens(s, 5, 2), ...(b ? [b] : [])], K).length; };
/** the signs a change does not turn down */
const taken = (second: (b: Box, s: LampSig) => Box | null, of = SIGNS): string[] => of.filter(s => found(s, b => second(b, s)) !== 0).map(said);
const none = (what: string, second: (b: Box, s: LampSig) => Box | null, of = SIGNS): void => { const t = taken(second, of); line(of.length > 0 && t.length === 0, what, t.join(' · ')); };

{ const bad: string[] = [];
  for (const s of SIGNS) { const pair = [wall, lens(s, 5, 2), wall, lens(s, 5, 2 + s.gap)], L = carLampsIn(pair, LAMP_FIT), said2 = lampsOf(pair).vehicleLamps;
    if (L.map(l => `${l.box} ${l.what} ${l.lit}`).join() !== [1, 3].map(i => `${i} ${s.what} ${!!s.emissive}`).join() || JSON.stringify(said2) !== JSON.stringify([1, 3].map(box => ({ box, what: s.what })))) bad.push(`${said(s)}: ${JSON.stringify(L)}`); }
  line(SIGNS.length >= 6 && !bad.length, 'a pair of each sign is two lamps, by their boxes in their order and by which lamp they are', bad.join(' · ')); }
line(SIGNS.some(s => s.emissive && !s.albedo) && SIGNS.some(s => s.albedo && !s.emissive) && SIGNS.every(s => !!s.albedo !== !!s.emissive), 'some signs burn and some are dark, each told by its glow or its paint and never by both');
line(!('vehicleLamps' in lampsOf([wall])), 'a level with no lamp says none');
none('a lone lens is no lamp', () => null);
none('a twin half a meter out of place is no twin', b => ({ ...b, c: [b.c[0], b.c[1], b.c[2] + 0.5] }));
none('… nor one a hand higher', b => ({ ...b, c: [b.c[0], b.c[1] + 0.1, b.c[2]] }));
none('… nor one that stands another way', b => ({ ...b, yaw: b.yaw + 0.01 }));
none('a lens a centimeter longer is no lens', b => ({ ...b, h: [b.h[0], b.h[1], b.h[2] + 0.01] }));
none('a tilted one is none', b => ({ ...b, rot: [0, 0, 0, 1] }) as unknown as Box);
none('a dark sign in a near paint is none', b => ({ ...b, albedo: [b.albedo[0] - 0.01, b.albedo[1], b.albedo[2]] }), SIGNS.filter(s => s.albedo));
none('… nor one that glows', b => ({ ...b, emissive: [0, 0.1, 0] }), SIGNS.filter(s => s.albedo));
none('a burning sign with another glow is none', b => ({ ...b, emissive: [b.emissive[0], b.emissive[1] + 0.01, b.emissive[2]] }), SIGNS.filter(s => s.emissive));
none('… nor one gone dark', b => ({ ...b, emissive: [0, 0, 0] }), SIGNS.filter(s => s.emissive));
console.log(`=== carlamps: ${fails ? `${fails} FAILED` : 'ALL PASS'} ===`);
process.exit(fails ? 1 : 0);
