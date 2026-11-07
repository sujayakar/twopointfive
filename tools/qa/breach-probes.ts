// The 'breach' family, run through probes.ts — what is left of it since the daytime breach SHOWCASE was removed (its breach-run / breach-restore / breach-tour went
// with the show; the daylight variant it staged is the game's and stays checked):
//   bun run tools/qa/probes.ts breach | breach-daylight
// PASS / FAIL each, all headless:
//   breach-daylight   setDaylight(true) then (false) leaves the light list (name, kind, enabled, intensity, colour, dir) and the frame's sky / haze bit for bit as
//                     they were; while it is day: a 'sun' of kind 2 is in the set and lit, the moon and every street lamp are out, every listed group is on, and
//                     the level's blackout (thrown first) is over
import { standUp, ROOT } from './headless.ts';
const { setDaylight } = await import(`${ROOT}/src/game/lighting.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}

/** the light set as a comparable list (order matters: the GPU slots follow it) — everything but the momentary intensity, which fixtures on the blink and restriking
 *  tubes legitimately change from one frame to the next (the switches behind it are compared separately) */
function lightList(engine: any): string {
  return engine.lights.lights.map((l: any) => `${l.name}:${l.kind}:${/^flashlight|pistol_light/.test(l.name) ? '-' : l.enabled ? 1 : 0}:${l.peakIntensity.toFixed(3)}:${l.color.map((c: number) => c.toFixed(3)).join(',')}:${/^beacon|^flashlight|pistol_light|_cone$/.test(l.name) ? '-' : l.dir.map((c: number) => c.toFixed(3)).join(',')}:${l.radius.toFixed(3)}`).join('|');   // (the beacons turn, the torches swing and the cameras' haze cones pan whatever the hour: their direction is nobody's switch — and whether a carried torch burns is its man's: a calm patrol puts his out in a lit place, calmtorch.ts, so a torch is not what day and night hand back)
}
/** main.ts uploads the light set inside engine.render, which the harness has none of: do it by hand after a step so activeCount / activeIdx mean this frame */
function upload(W: any) { W.engine.lights.upload(W.game.time); }
const skyOf = (engine: any) => JSON.stringify({ z: engine.settings.skyZenith, h: engine.settings.skyHorizon, haze: engine.settings.hazeDensity });
const switches = (game: any) => game.targets.map((t: any) => `${t.name}:${t.on ? 1 : 0}`).join('|');

// ---------------------------------------------------------------- breach-daylight
export async function breachDaylight() {
  const F = family('breach-daylight: the daytime variant swaps in and back out bit for bit (lighting.ts setDaylight)');
  Math.random = seeded(3);
  const W = await standUp(); const { game, engine, level } = W;
  for (let i = 0; i < 10; i++) W.step(DT);
  game.setBlackout(30, 'debug'); for (let i = 0; i < 5; i++) W.step(DT);   // thrown first: day must bring the mains back
  game.endBlackout(); for (let i = 0; i < 6 / DT; i++) W.step(DT);         // …and the round trip is measured against a floor with the mains ON and every tube done restriking (a blackout is not something day → night should restore)
  upload(W); const L0 = lightList(engine), S0 = skyOf(engine), W0 = switches(game);
  game.setBlackout(30, 'debug'); W.step(DT);
  setDaylight(game, true); for (let i = 0; i < 3; i++) W.step(DT); upload(W);
  const D = level.daylight;
  { const f: string[] = [];
    const sun = engine.lights.lights.find((l: any) => l.name === 'sun'); if (!sun) f.push('no sun in the set'); else { if (sun.kind !== 2) f.push(`sun kind ${sun.kind}`); if (!sun.enabled) f.push('sun disabled'); if (!(engine.lights.activeIdx as number[]).some((i: number) => engine.lights.lights[i] === sun)) f.push('sun not uploaded'); }
    const moon = engine.lights.lights.find((l: any) => l.name === 'moon'); if (moon?.enabled) f.push('moon still lit');
    for (const t of game.targets) { if (D.groupsOff.includes(t.group) && (t.on || (t.light && t.light.enabled))) f.push(`${t.name} (${t.group}) still on`); if (D.groupsOn.includes(t.group) && !t.on) f.push(`${t.name} (${t.group}) off`); }
    if (game.blackout.active) f.push('blackout still active');
    if (Math.abs(engine.settings.hazeDensity - D.haze) > 1e-9) f.push(`haze ${engine.settings.hazeDensity}`);
    if (JSON.stringify(engine.settings.skyZenith) !== JSON.stringify(D.sky.zenith)) f.push(`sky zenith ${engine.settings.skyZenith}`);
    const Ld = lightList(engine); setDaylight(game, true); if (lightList(engine) !== Ld) f.push('a second setDaylight(true) changed the list');
    F.verdict(f.length === 0, 'day: one lit sun of kind 2 uploaded, moon and street lamps out, every listed group on, mains back, sky / haze the variant\'s, idempotent', f.join('; ') || `sun dir ${sun?.dir.map(f2)} I ${sun?.intensity}; ${engine.lights.activeCount} lights uploaded by day; sky ${JSON.stringify(engine.settings.skyZenith)} haze ${engine.settings.hazeDensity}`); }
  setDaylight(game, false); for (let i = 0; i < 6 / DT; i++) W.step(DT); upload(W);   // (six seconds again: the groups day switched on and night switches back off restrike nothing, but a fixture on the blink is compared like for like)
  { const f: string[] = [];
    const L1 = lightList(engine), S1 = skyOf(engine), W1 = switches(game);
    if (L1 !== L0) { const a = L0.split('|'), b = L1.split('|'); const i = a.findIndex((x, k) => x !== b[k]); f.push(`light list differs (first at #${i}: '${a[i]}' vs '${b[i]}'; ${a.length} vs ${b.length} lights)`); }
    if (S1 !== S0) f.push(`sky ${S0} → ${S1}`);
    if (W1 !== W0) { const a = W0.split('|'), b = W1.split('|'); f.push(`switches differ: ${a.filter((x: any, k: any) => x !== b[k]).slice(0, 6).join(', ')}`); }
    setDaylight(game, false); if (lightList(engine) !== L1) f.push('a second setDaylight(false) changed the list');
    F.verdict(f.length === 0, 'night again: light list, sky, haze and every switch exactly as before the round trip; idempotent', f.join('; ') || `${L0.split('|').length} lights, ${engine.lights.activeCount} uploaded by night, sky ${S0}`); }
  return F.done();
}

// ---------------------------------------------------------------- clear-doctrine
