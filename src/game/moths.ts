// MOTHS round the LIT sodium lamps at NIGHT (approved with the wind lane as "dust motes, sodium light moths"; the MOTES half — a particle kind topped up inside the
// torch cones — was cut 08-11, the maintainer: 'they look weird … it's obvious that they spawn when your flashlight shows up somewhere, not ambiently always present').
// A dozen at most, CPU boxes (a moth must orbit a point, which a particle cannot) — one small NoShadow box each, drawn through the g-buffer like anything (so the lamp
// lights it, the wet lot mirrors it, its motion vector rides its key), on a noisy analytic orbit under the cobra head with a bob, a wobble, a dart at the lens now and
// then, wings a-flutter as a fast roll of the box; they gather over a couple of seconds when a lamp comes on and scatter when it goes out (shot, the daylight), in a
// gale (the wind field at the head) or a downpour. Twelve boxes, a few dozen trig calls: nothing. Nobody's AI reads them. Behind tune.moths (kept with the wind's rows).
import { Vec3, v3 } from '../math/vec';
import { Box, BoxFlag, makeBox, MKEY } from '../scene/boxes';
import type { Game, LightTarget } from './game';
import type { RtLight } from '../render/lights';
import './lighting';
import { crossToRust } from './crossing';

/** the moths' numbers */
export const MOTHS = {
  max: 12, perLamp: 7,
  rMin: 0.14, rMax: 0.5,      // orbit radius about the lamp's axis (m)
  below: 0.25, bob: 0.16,     // mean depth under the lens, ± bob
  omegaMin: 2.2, omegaMax: 4.6,   // rad/s round the head
  flapHz: 9, wing: 0.016,     // the roll flutter and the wing half-span (m): a big moth — under two pixels from the play camera even so, it is the flicker across the lens that reads
  gatherSecs: 2.5, scatterSecs: 1.2,
  galeMs: [3, 6] as [number, number],   // wind speed at the head (m/s) over which they thin to none
  rainMax: 0.92,              // rain rate past which even the lamp's lee is too wet
  albedo: [0.62, 0.56, 0.42] as Vec3,
};
export const MOTH_STATS = { shown: 0 };

/** integer hash → [0, 1) (the module's dice: every choice counts off its own serial — never Math.random) */
function h01(x: number): number { x = Math.imul((x ^ 0x2545F491) >>> 0, 0x9E3779B1) >>> 0; x ^= x >>> 15; x = Math.imul(x, 0x85EBCA77) >>> 0; x ^= x >>> 13; return (x >>> 8) / 16777216; }

interface Moth { lamp: number; r: number; omega: number; ph: number; bobF: number; bobPh: number; wob: number; dartT: number; dartPh: number; flapPh: number; away: Vec3; shown: number; box: Box; }

export class Moths {
  /** the sodium lamps (exterior spot lights hung high, found once off the light targets) and the moth pool dealt round them */
  private lamps: { target: LightTarget; light: RtLight; head: Vec3; act: number }[] = [];
  private moths: Moth[] = [];
  constructor(private g: Game) {
    for (const t of g.targets) { const l = t.light; if (t.kind === 'analytic' && l && l.kind === 1 && t.group === 'exterior' && l.pos[1] > 3.5) this.lamps.push({ target: t, light: l, head: [l.pos[0], l.pos[1] + 0.08, l.pos[2]], act: 0 }); }   // TODO(LevelData): a lamp record; today: any exterior spot over 3.5 m is a street lamp (the office's three)
    for (let i = 0; i < MOTHS.max && this.lamps.length; i++) {
      const hs = (k: number) => h01(i * 7919 + k * 131);
      const box = makeBox({ c: [0, -50, 0], h: [0.006, 0.0025, MOTHS.wing], albedo: v3.copy(MOTHS.albedo), flags: BoxFlag.Dynamic | BoxFlag.NoShadow, rough: 0.8 });   // body along local +X (the heading), the span across local Z: rolling about the body flickers the span seen from above
      box.mkey = MKEY.test + 0x4000 + i;   // motion identity (scene/boxes.ts MKEY: the test range's upper numbers — nothing else keys there)
      const az = hs(9) * Math.PI * 2;
      this.moths.push({ lamp: i % this.lamps.length, r: MOTHS.rMin + (MOTHS.rMax - MOTHS.rMin) * hs(1), omega: (MOTHS.omegaMin + (MOTHS.omegaMax - MOTHS.omegaMin) * hs(2)) * (hs(3) < 0.5 ? -1 : 1), ph: hs(4) * 6.283, bobF: 0.7 + 1.6 * hs(5), bobPh: hs(6) * 6.283, wob: 0.3 + 0.7 * hs(7), dartT: 4 + 6 * hs(8), dartPh: hs(10) * 10, flapPh: hs(11) * 6.283, away: [Math.cos(az), -0.25 - 0.5 * hs(12), Math.sin(az)], shown: 0, box });
    }
  }
  /** the deal back as constructed (encounter restart: a restarted encounter repeats) */
  clear() { this.moths.forEach((m, i) => { m.shown = 0; m.lamp = this.lamps.length ? i % this.lamps.length : 0; }); for (const L of this.lamps) L.act = 0; }

  /** Once a frame (game.ts, after the wind): move the moths. */
  update(dt: number): void { return crossToRust('Moths.update', this, [dt]) as any; }
  /** the moths' boxes into the frame's dynamic list (only the ones about) */
  boxes(out: Box[]) { for (const m of this.moths) if (m.shown > 0.01) out.push(m.box); }
  mothsShown(): number { let n = 0; for (const m of this.moths) if (m.shown > 0.01) n++; return n; }
  summary(): string {
    return (this.g.tune as { moths?: boolean }).moths === false ? 'moths off' : `moths ${this.mothsShown()}/${this.moths.length} about ${this.lamps.length} lamp(s) [${this.lamps.map(L => `${L.target.name} ${L.act.toFixed(2)}`).join(', ')}]`;
  }
}
