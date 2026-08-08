// Procedurally synthesized sound effects rendered into AudioBuffers at startup (no samples shipped).
export type SfxName =
  | 'pistolSuppressed' | 'pistolLoud' | 'dryFire' | 'magOut' | 'magIn' | 'rack' | 'magDrop' | 'footSoft' | 'footHard' | 'ocp' | 'nvOn' | 'nvOff'
  | 'click' | 'throw' | 'canisterBounce' | 'bulletImpact' | 'bodyHit' | 'bodyFall' | 'lightBreak' | 'lightOff' | 'deny' | 'equip' | 'smokePop' | 'alertSting' | 'radio'
  | 'doorCreak' | 'doorBang' | 'doorLatch' | 'doorBash' | 'doorRattle' | 'lockPick' | 'lockOpen' | 'lockBreak' | 'powerDown' | 'powerUp' | 'stunBang' | 'earRing' | 'propScrape' | 'propThud';

type Gen = (t: number, i: number, sr: number, rnd: () => number, state: Float32Array) => number;

function render(ctx: BaseAudioContext, dur: number, gen: Gen, seed = 1): AudioBuffer {
  const sr = ctx.sampleRate; const n = Math.max(1, Math.floor(dur * sr));
  const buf = ctx.createBuffer(1, n, sr); const d = buf.getChannelData(0);
  let s = seed >>> 0 || 1; const rnd = () => { s ^= s << 13; s ^= s >>> 17; s ^= s << 5; return ((s >>> 0) / 4294967296) * 2 - 1; };
  const state = new Float32Array(16);
  for (let i = 0; i < n; i++) d[i] = gen(i / sr, i, sr, rnd, state);
  // gentle fade out to avoid clicks
  const f = Math.min(n, Math.floor(sr * 0.01)); for (let i = 0; i < f; i++) d[n - 1 - i] *= i / f;
  return buf;
}

// one-pole lowpass helper using state slot k
const lp = (x: number, st: Float32Array, k: number, a: number) => (st[k] += a * (x - st[k]));
const env = (t: number, a: number, d: number) => (t < a ? t / a : Math.exp(-(t - a) / d));

export function buildSfx(ctx: BaseAudioContext): Record<SfxName, AudioBuffer> {
  const TAU = Math.PI * 2;
  return {
    // suppressed 5.7×28: sharp 'thwip' — bandpassed noise snap + low thump + slide clack 35 ms later
    pistolSuppressed: render(ctx, 0.32, (t, _i, _sr, rnd, st) => {
      const n = rnd();
      const snap = (lp(n, st, 0, 0.35) - lp(n, st, 1, 0.06)) * env(t, 0.0008, 0.022) * 1.1;
      const thump = Math.sin(TAU * (70 * t - 18 * t * t)) * env(t, 0.002, 0.05) * 0.9;
      const tc = t - 0.035; const clack = tc > 0 ? (rnd() - lp(rnd(), st, 2, 0.2)) * env(tc, 0.0005, 0.012) * 0.5 + Math.sin(TAU * 1900 * tc) * env(tc, 0.0005, 0.006) * 0.15 : 0;
      const tail = lp(n, st, 3, 0.02) * env(t, 0.01, 0.12) * 0.5;
      return (snap + thump + clack + tail) * 0.72;
    }, 11),
    // unsuppressed 9 mm indoors: broadband crack, big low end, long-ish slap tail
    pistolLoud: render(ctx, 0.9, (t, _i, _sr, rnd, st) => {
      const n = rnd();
      const crack = n * env(t, 0.0005, 0.03) * 1.0;
      const body = lp(n, st, 0, 0.12) * env(t, 0.001, 0.09) * 1.3;
      const thump = Math.sin(TAU * (95 * t - 30 * t * t)) * env(t, 0.001, 0.08) * 1.2;
      const tail = lp(rnd(), st, 1, 0.015) * env(t, 0.02, 0.35) * 0.9;
      return Math.tanh((crack + body + thump + tail) * 1.4) * 0.95;
    }, 23),
    dryFire: render(ctx, 0.08, (t, _i, _sr, rnd, st) => (rnd() - lp(rnd(), st, 0, 0.3)) * env(t, 0.0004, 0.006) * 0.6 + Math.sin(TAU * 2600 * t) * env(t, 0.0004, 0.004) * 0.2, 3),
    magOut: render(ctx, 0.25, (t, _i, _sr, rnd, st) => {
      const c1 = (rnd() - lp(rnd(), st, 0, 0.25)) * env(t, 0.0005, 0.01) * 0.5;
      const ts = t - 0.05; const slide = ts > 0 ? lp(rnd(), st, 1, 0.08 + 0.1 * ts) * env(ts, 0.01, 0.06) * 0.35 : 0;
      const t2 = t - 0.16; const c2 = t2 > 0 ? (rnd() - lp(rnd(), st, 2, 0.3)) * env(t2, 0.0005, 0.008) * 0.35 : 0;
      return c1 + slide + c2;
    }, 5),
    magIn: render(ctx, 0.22, (t, _i, _sr, rnd, st) => {
      const c1 = lp(rnd(), st, 0, 0.5) * env(t, 0.0005, 0.012) * 0.5 + Math.sin(TAU * 900 * t) * env(t, 0.001, 0.01) * 0.2;
      const t2 = t - 0.09; const c2 = t2 > 0 ? (lp(rnd(), st, 1, 0.6) * env(t2, 0.0005, 0.02) * 0.8 + Math.sin(TAU * 520 * t2) * env(t2, 0.001, 0.025) * 0.3) : 0;
      return c1 + c2;
    }, 7),
    rack: render(ctx, 0.3, (t, _i, _sr, rnd, st) => {
      const a = lp(rnd(), st, 0, 0.4) * env(t, 0.001, 0.03) * 0.7 + Math.sin(TAU * 700 * t) * env(t, 0.001, 0.02) * 0.2;
      const t2 = t - 0.13; const b = t2 > 0 ? lp(rnd(), st, 1, 0.5) * env(t2, 0.001, 0.035) * 0.9 + Math.sin(TAU * 480 * t2) * env(t2, 0.001, 0.03) * 0.3 : 0;
      return a + b;
    }, 13),
    // polymer magazine clattering on carpet/lino
    magDrop: render(ctx, 0.5, (t, _i, _sr, rnd, st) => {
      let out = 0;
      const hits = [0, 0.11, 0.19, 0.25, 0.29];
      for (let k = 0; k < hits.length; k++) { const th = t - hits[k]; if (th < 0) continue; const a = Math.pow(0.55, k); out += (Math.sin(TAU * (1450 + k * 210) * th) * 0.4 + Math.sin(TAU * (2900 + k * 330) * th) * 0.25 + lp(rnd(), st, k, 0.5) * 0.5) * env(th, 0.0005, 0.018) * a; }
      return out * 0.7;
    }, 17),
    footSoft: render(ctx, 0.18, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.04) * env(t, 0.004, 0.05) * 1.6 + lp(rnd(), st, 1, 0.3) * env(t, 0.001, 0.015) * 0.15, 19),
    footHard: render(ctx, 0.2, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.07) * env(t, 0.002, 0.04) * 1.5 + (rnd() - lp(rnd(), st, 1, 0.4)) * env(t, 0.0008, 0.01) * 0.35 + Math.sin(TAU * 140 * t) * env(t, 0.002, 0.04) * 0.4, 29),
    // OCP: capacitor whine-thump
    ocp: render(ctx, 0.45, (t) => {
      const f = 2400 * Math.exp(-t * 9) + 180; const ph = TAU * (2400 / -9 * Math.exp(-t * 9) + 180 * t);
      return (Math.sin(ph) * (0.5 + 0.5 * Math.sin(TAU * 90 * t))) * env(t, 0.002, 0.12) * 0.7 + Math.sin(TAU * 60 * t) * env(t, 0.005, 0.1) * 0.5 + (f > 0 ? 0 : 0);
    }, 31),
    // goggles: the classic rising whine
    nvOn: render(ctx, 0.9, (t) => { const f0 = 500, f1 = 5200; const k = Math.log(f1 / f0) / 0.8; const ph = TAU * f0 * (Math.exp(k * Math.min(t, 0.8)) - 1) / k; return Math.sin(ph) * env(t, 0.02, 0.5) * 0.22 + Math.sin(ph * 0.5) * env(t, 0.02, 0.4) * 0.08; }, 37),
    nvOff: render(ctx, 0.5, (t) => { const f0 = 3000, f1 = 200; const k = Math.log(f1 / f0) / 0.45; const ph = TAU * f0 * (Math.exp(k * Math.min(t, 0.45)) - 1) / k; return Math.sin(ph) * env(t, 0.005, 0.2) * 0.2; }, 41),
    click: render(ctx, 0.05, (t, _i, _sr, rnd, st) => (rnd() - lp(rnd(), st, 0, 0.3)) * env(t, 0.0003, 0.004) * 0.5 + Math.sin(TAU * 3200 * t) * env(t, 0.0003, 0.003) * 0.2, 43),
    throw: render(ctx, 0.35, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.03 + 0.25 * Math.sin(Math.PI * Math.min(1, t / 0.3))) * Math.sin(Math.PI * Math.min(1, t / 0.32)) * 0.5, 47),
    canisterBounce: render(ctx, 0.4, (t) => (Math.sin(TAU * 1310 * t) * 0.5 + Math.sin(TAU * 2140 * t) * 0.35 + Math.sin(TAU * 3370 * t) * 0.25 + Math.sin(TAU * 5210 * t) * 0.12) * env(t, 0.0005, 0.09) * 0.6, 53),
    bulletImpact: render(ctx, 0.25, (t, _i, _sr, rnd, st) => (rnd() * env(t, 0.0004, 0.012) * 0.7 + lp(rnd(), st, 0, 0.1) * env(t, 0.001, 0.05) * 0.8 + Math.sin(TAU * (400 + 900 * Math.exp(-t * 40)) * t) * env(t, 0.001, 0.03) * 0.3), 59),
    bodyHit: render(ctx, 0.25, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.05) * env(t, 0.002, 0.06) * 1.6 + Math.sin(TAU * 80 * t) * env(t, 0.002, 0.07) * 0.6, 61),
    bodyFall: render(ctx, 0.9, (t, _i, _sr, rnd, st) => { const t2 = t - 0.32; return lp(rnd(), st, 0, 0.03) * env(t, 0.01, 0.09) * 1.2 + (t2 > 0 ? lp(rnd(), st, 1, 0.02) * env(t2, 0.005, 0.14) * 1.8 + Math.sin(TAU * 55 * t2) * env(t2, 0.004, 0.12) * 0.9 + lp(rnd(), st, 2, 0.2) * env(t2, 0.02, 0.2) * 0.15 : 0); }, 67),
    lightBreak: render(ctx, 0.7, (t, _i, _sr, rnd, st) => {
      let out = rnd() * env(t, 0.0005, 0.02) * 0.6 + Math.sin(TAU * 180 * t) * env(t, 0.001, 0.04) * 0.3;
      for (let k = 0; k < 9; k++) { const th = t - (0.01 + ((k * 0.6180339) % 1) * 0.35); if (th < 0) continue; out += Math.sin(TAU * (3200 + ((k * 977) % 2600)) * th) * env(th, 0.0004, 0.03 + (k % 3) * 0.02) * 0.18; }
      return out + lp(rnd(), st, 0, 0.5) * env(t, 0.02, 0.25) * 0.08;
    }, 71),
    lightOff: render(ctx, 0.2, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.2) * env(t, 0.001, 0.02) * 0.6 + Math.sin(TAU * 120 * t) * env(t, 0.002, 0.05) * 0.5, 73),
    deny: render(ctx, 0.25, (t) => (Math.sin(TAU * 220 * t) + Math.sin(TAU * 330 * t) * 0.5) * (env(t, 0.005, 0.05) + (t > 0.11 ? env(t - 0.11, 0.005, 0.05) : 0)) * 0.25, 79),
    equip: render(ctx, 0.2, (t, _i, _sr, rnd, st) => { const t2 = t - 0.08; return (rnd() - lp(rnd(), st, 0, 0.3)) * env(t, 0.0005, 0.008) * 0.35 + (t2 > 0 ? lp(rnd(), st, 1, 0.5) * env(t2, 0.0005, 0.015) * 0.5 : 0); }, 83),
    smokePop: render(ctx, 0.6, (t, _i, _sr, rnd, st) => rnd() * env(t, 0.0005, 0.015) * 0.7 + lp(rnd(), st, 0, 0.08) * env(t, 0.002, 0.06) * 0.7 + lp(rnd(), st, 1, 0.3) * env(t, 0.05, 0.4) * 0.25, 89),
    // reverse-swell + hit for the alert transition
    alertSting: render(ctx, 1.6, (t, _i, _sr, rnd, st) => {
      const rise = t < 0.7 ? lp(rnd(), st, 0, 0.02 + 0.5 * (t / 0.7) ** 3) * Math.pow(t / 0.7, 3) * 0.7 : 0;
      const th = t - 0.7; const hit = th >= 0 ? (Math.sin(TAU * (48 * th + 4 * Math.exp(-th * 6))) * env(th, 0.002, 0.35) * 1.1 + lp(rnd(), st, 1, 0.2) * env(th, 0.001, 0.15) * 0.7 + Math.sin(TAU * 587 * th) * env(th, 0.005, 0.5) * 0.12 + Math.sin(TAU * 622 * th) * env(th, 0.005, 0.6) * 0.1) : 0;
      return Math.tanh((rise + hit) * 1.2) * 0.9;
    }, 97),
    // guard radio squelch blip
    // doors: hinge creak (stick-slip: a wobbly saw-ish tone through a resonant body), slam, latch click, kicked open
    doorCreak: render(ctx, 0.7, (t, _i, sr, rnd, st) => {
      const f = 310 + 120 * Math.sin(t * 9) + 60 * Math.sin(t * 23 + 1) + 40 * lp(rnd(), st, 2, 0.002) * 30;
      st[3] += f / sr; const ph = st[3] % 1; const saw = (ph * 2 - 1);
      const body = lp(saw, st, 0, 0.08) - lp(saw, st, 1, 0.02);
      return body * (0.5 + 0.5 * Math.sin(t * 31)) * env(t, 0.03, 0.4) * Math.min(1, t / 0.05) * 0.35;
    }, 107),
    doorBang: render(ctx, 0.8, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.02) * env(t, 0.002, 0.12) * 2.2 + Math.sin(TAU * 62 * t) * env(t, 0.002, 0.15) * 0.9 + Math.sin(TAU * 124 * t) * env(t, 0.002, 0.08) * 0.4 + (rnd() - lp(rnd(), st, 1, 0.5)) * env(t, 0.001, 0.02) * 0.3 + lp(rnd(), st, 2, 0.1) * (t > 0.09 ? env(t - 0.09, 0.003, 0.04) : 0) * 0.5, 109),
    doorLatch: render(ctx, 0.16, (t, _i, _sr, rnd, st) => { const t2 = t - 0.045; return (rnd() - lp(rnd(), st, 0, 0.25)) * env(t, 0.0005, 0.006) * 0.45 + Math.sin(TAU * 1900 * t) * env(t, 0.0005, 0.01) * 0.15 + (t2 > 0 ? lp(rnd(), st, 1, 0.3) * env(t2, 0.0008, 0.012) * 0.6 + Math.sin(TAU * 900 * t2) * env(t2, 0.001, 0.02) * 0.2 : 0); }, 113),
    doorBash: render(ctx, 0.9, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.03) * env(t, 0.001, 0.09) * 2.4 + Math.sin(TAU * 48 * t) * env(t, 0.001, 0.2) * 1.0 + rnd() * env(t, 0.0005, 0.01) * 0.5 + lp(rnd(), st, 1, 0.15) * (t > 0.05 ? env(t - 0.05, 0.004, 0.06) : 0) * 0.7, 127),
    // locks: the lever tried against a thrown bolt (a few metallic clacks over the leaf knocking in its frame), one click of a pick working the pins (a tick and a
    // whisper of scrape), the cylinder going over and the bolt drawing back (snap, then a duller clunk), and the keep splintering out of the jamb under a boot
    // (a burst of wood crackle over a low thump, the strike plate clinking away) — that one plays under doorBash
    doorRattle: render(ctx, 0.32, (t, _i, _sr, rnd, st) => { let out = 0; const taps = [0, 0.05, 0.11, 0.2], amp = [1, 0.7, 0.85, 0.4]; for (let k = 0; k < taps.length; k++) { const th = t - taps[k]; if (th < 0) continue; out += ((rnd() - lp(rnd(), st, k, 0.3)) * env(th, 0.0004, 0.005) * 0.3 + Math.sin(TAU * (1650 + k * 140) * th) * env(th, 0.0005, 0.012) * 0.22 + Math.sin(TAU * 210 * th) * env(th, 0.001, 0.03) * 0.35) * amp[k]; } return out; }, 151),
    lockPick: render(ctx, 0.14, (t, _i, _sr, rnd, st) => { const t2 = t - 0.03; return (rnd() - lp(rnd(), st, 0, 0.3)) * env(t, 0.0003, 0.003) * 0.35 + Math.sin(TAU * 4300 * t) * env(t, 0.0003, 0.005) * 0.12 + (t2 > 0 ? (lp(rnd(), st, 1, 0.12) - lp(rnd(), st, 2, 0.03)) * env(t2, 0.004, 0.03) * 0.25 : 0); }, 157),
    lockOpen: render(ctx, 0.3, (t, _i, _sr, rnd, st) => { const t2 = t - 0.07; return (rnd() - lp(rnd(), st, 0, 0.25)) * env(t, 0.0004, 0.006) * 0.5 + Math.sin(TAU * 2500 * t) * env(t, 0.0004, 0.008) * 0.2 + (t2 > 0 ? lp(rnd(), st, 1, 0.35) * env(t2, 0.001, 0.02) * 0.8 + Math.sin(TAU * 320 * t2) * env(t2, 0.001, 0.04) * 0.4 : 0); }, 163),
    lockBreak: render(ctx, 0.6, (t, _i, _sr, rnd, st) => {
      let out = rnd() * env(t, 0.0004, 0.015) * 0.8 + lp(rnd(), st, 0, 0.06) * env(t, 0.001, 0.07) * 1.4 + Math.sin(TAU * 150 * t) * env(t, 0.001, 0.06) * 0.6;
      for (let k = 0; k < 7; k++) { const th = t - (0.012 + ((k * 0.6180339) % 1) * 0.16); if (th < 0) continue; out += (rnd() - lp(rnd(), st, 1 + (k % 3), 0.4)) * env(th, 0.0003, 0.006 + (k % 2) * 0.005) * 0.35; }   // splinters letting go one after another
      const c1 = t - 0.09, c2 = t - 0.21;   // the strike plate: off the jamb, then off the floor
      if (c1 > 0) out += (Math.sin(TAU * 2700 * c1) * 0.3 + Math.sin(TAU * 4100 * c1) * 0.15) * env(c1, 0.0005, 0.05);
      if (c2 > 0) out += (Math.sin(TAU * 3100 * c2) * 0.18 + Math.sin(TAU * 4600 * c2) * 0.1) * env(c2, 0.0005, 0.035);
      return Math.tanh(out * 1.2) * 0.9;
    }, 167),
    // mains: contactor clunk + transformer hum winding down / spinning back up with ballast plinks
    powerDown: render(ctx, 1.6, (t, _i, sr, rnd, st) => {
      const f = 100 * Math.exp(-t * 1.6) + 18; st[3] += f / sr; const hum = (Math.sin(st[3] * TAU) * 0.6 + Math.sin(st[3] * TAU * 2) * 0.3 + Math.sin(st[3] * TAU * 3) * 0.15) * env(t, 0.005, 0.7) * 0.35;
      const clunk = lp(rnd(), st, 0, 0.05) * env(t, 0.001, 0.05) * 1.8 + Math.sin(TAU * 70 * t) * env(t, 0.001, 0.08) * 0.6;
      return hum + clunk;
    }, 131),
    powerUp: render(ctx, 1.8, (t, _i, sr, rnd, st) => {
      const f = 30 + 70 * (1 - Math.exp(-t * 2.2)); st[3] += f / sr; const hum = (Math.sin(st[3] * TAU) * 0.5 + Math.sin(st[3] * TAU * 2) * 0.35) * Math.min(1, t / 0.3) * env(t, 0.3, 0.9) * 0.3;
      const clunk = lp(rnd(), st, 0, 0.05) * env(t, 0.001, 0.05) * 1.6 + Math.sin(TAU * 60 * t) * env(t, 0.001, 0.08) * 0.5;
      const t2 = t - 0.9, t3 = t - 1.25; const plink = (t2 > 0 ? Math.sin(TAU * 2100 * t2) * env(t2, 0.0005, 0.02) * 0.2 + rnd() * env(t2, 0.0003, 0.004) * 0.2 : 0) + (t3 > 0 ? Math.sin(TAU * 1700 * t3) * env(t3, 0.0005, 0.02) * 0.15 : 0);
      return hum + clunk + plink;
    }, 137),
    radio: render(ctx, 0.35, (t, _i, _sr, rnd, st) => { const sq = (t < 0.06 || (t > 0.25 && t < 0.31)) ? lp(rnd(), st, 0, 0.6) * 0.4 : 0; const tone = (t > 0.07 && t < 0.22) ? (Math.sign(Math.sin(TAU * 1250 * t)) * 0.12 + Math.sin(TAU * 1870 * t) * 0.08) : 0; return sq + tone; }, 101),
    // stun canister indoors: supersonic crack, a chest thump that dives (phase-accumulated so the sweep is exact at any sample rate), the steel body
    // ringing for a moment, and a long slap off every wall — driven hard into tanh, it is meant to be the loudest thing in the building
    stunBang: render(ctx, 1.5, (t, _i, sr, rnd, st) => {
      const n = rnd();
      const crack = n * env(t, 0.0003, 0.012) * 1.3 + (n - lp(n, st, 0, 0.5)) * env(t, 0.0002, 0.004) * 0.8;
      const body = lp(rnd(), st, 1, 0.09) * env(t, 0.001, 0.11) * 1.8;
      const f = 36 + 95 * Math.exp(-t * 9); st[3] += f / sr; const thump = Math.sin(TAU * st[3]) * env(t, 0.001, 0.2) * 1.5;
      const ring = (Math.sin(TAU * 1180 * t) * 0.5 + Math.sin(TAU * 2210 * t) * 0.3 + Math.sin(TAU * 3470 * t) * 0.2) * env(t, 0.001, 0.22) * 0.14;
      const tail = lp(rnd(), st, 2, 0.02) * env(t, 0.03, 0.55) * 0.9 + lp(rnd(), st, 4, 0.006) * env(t, 0.08, 0.7) * 0.6;
      return Math.tanh((crack + body + thump + ring + tail) * 1.7) * 0.97;
    }, 139),
    // what the player's ears do about it when it goes off in sight: a thin whistle that swells in as the bang decays and takes a couple of seconds to clear
    earRing: render(ctx, 2.6, (t) => (Math.sin(TAU * 3120 * t) + Math.sin(TAU * 4310 * t) * 0.3 + Math.sin(TAU * 2650 * t) * 0.15) * Math.min(1, t / 0.18) * Math.exp(-t / 1.1) * 0.11, 149),
    // loose furniture: a short scrape / caster rattle (band-limited hiss chopped by stick-slip chatter over a low judder whose pitch wanders) and a hollow knock
    // (played at different rates per kind: cardboard low and dull, a steel bin bright)
    propScrape: render(ctx, 0.42, (t, _i, sr, rnd, st) => {
      const f = 70 + 25 * Math.sin(t * 21) + 15 * Math.sin(t * 57 + 1); st[3] += f / sr; const judder = Math.sin(st[3] * TAU) * 0.14;
      const grain = lp(rnd(), st, 0, 0.14) - lp(rnd(), st, 1, 0.02);
      const chatter = 0.55 + 0.45 * (Math.sin(t * 83) > 0 ? 1 : -1) * (0.5 + 0.5 * Math.sin(t * 7));
      return (grain * chatter * 0.9 + judder) * env(t, 0.012, 0.15) * Math.min(1, t / 0.02) * 0.8;
    }, 139),
    propThud: render(ctx, 0.4, (t, _i, _sr, rnd, st) => lp(rnd(), st, 0, 0.04) * env(t, 0.002, 0.07) * 1.7 + Math.sin(TAU * 96 * t) * env(t, 0.002, 0.08) * 0.7 + Math.sin(TAU * 232 * t) * env(t, 0.001, 0.03) * 0.3 + (rnd() - lp(rnd(), st, 1, 0.4)) * env(t, 0.0006, 0.008) * 0.3, 149),
  };
}
