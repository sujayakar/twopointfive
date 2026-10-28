// The look prototype's "Real light values" row (render/photometry.ts; render/lights.ts LightSet.dayExp / dayAmount, engine.ts lookExposure) against the
// renderer's own light set: by day every source that is not the hour's own goes up at min(as authored, its real strength × the hour's exposure), by what it
// is; the sun, a lightning stroke, an infrared illuminator, a device's sight cone and the Team & kit figure's staging go up as they are; the list the game reads
// is never written; and at a night's exposure the upload is word for word what it was. No level, no GPU: the device is a stand-in that takes every write.
//   bun run tools/qa/lookunits-check.ts   (exit 0 = all pass; LOOKUNITS_BREAK=1 uploads at the night's exposure whatever the hour and must fail)
import { LightSet, type RtLight } from '../../src/render/lights';
import { LUX_PER_UNIT, HOUR_SUN_LUX, TRUE_CD, FLASH_TRUE, EMISSIVE_TRUE, hourExposure, lightClass, lightDayK, emissiveDayK } from '../../src/render/photometry';
import { HOURS } from '../../src/game/hours';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const BREAK = process.env.LOOKUNITS_BREAK === '1';
(globalThis as any).GPUBufferUsage ??= new Proxy({}, { get: () => 0 });
const device: any = { createBuffer: () => ({}), queue: { writeBuffer() {} } };
const light = (name: string, group: string, kind: 0 | 1 | 2, intensity: number, more: Partial<RtLight> = {}): RtLight => ({ name, group, kind, pos: [0, 2, 0], dir: [0, -1, 0], color: [1, 0.5, 0.25], intensity, range: 22, innerDeg: 40, outerDeg: 65, radius: 0.04, volumetric: 0, owner: 0, enabled: true, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: intensity, ...more });
const near = (a: number, b: number, rel = 1e-5) => Math.abs(a - b) <= rel * Math.max(Math.abs(a), Math.abs(b), 1e-9);
const one = (n: number) => n >= 1 ? '1' : `1/${Math.round(1 / n).toLocaleString('en-US')}`;

// (1) the hours' exposures, from the game's own hour table
const ids = Object.keys(HOURS) as (keyof typeof HOURS)[], exp = Object.fromEntries(ids.map(id => [id, hourExposure(id, HOURS[id].sun.intensity)])) as Record<string, number>;
check(ids.every(id => HOUR_SUN_LUX[id] > 0) && hourExposure('night', 0) === 1 && hourExposure('no such hour', 6) === 1, `every hour of the game has the lux its sun stands for (${ids.map(id => `${id} ${HOUR_SUN_LUX[id].toLocaleString('en-US')}`).join(', ')}); a night and an hour nobody knows are 1`);
check(near(exp.midday, HOURS.midday.sun.intensity * LUX_PER_UNIT / 100000) && exp.midday < exp.overcast && exp.overcast < exp.dusk && exp.dusk < 1 && exp.dawn < 1, `the exposures against the night's: ${ids.map(id => `${id} ${one(exp[id])}`).join(', ')} (brighter hour, smaller number)`);

// (2) the light set at midday
const ls = new LightSet(device);
const bulb = light('bulb_store', 'store', 0, 26), lantern = light('lantern_hut', 'hut', 0, 4.5), torch = light('flashlight0', 'flashlights', 1, 70), rail = light('pistol_light', 'player', 1, 60, { card: true }), van = light('van_lamp', 'exterior', 1, 150),
  ghost = light('seen out', 'tea', 0, 0, { enabled: false, fogGhost: 8 }), flash = light('flash', 'fx', 0, 160, { ttl: 0.05 }), fire = light('fire0', 'fire', 0, 30, { steady: { intensity: 0, pos: [0, 0, 0], radius: 0.1 } as RtLight['steady'] });
const scaled = [bulb, lantern, torch, rail, van, ghost, flash, fire];
const kept = [light('sun', 'hour', 2, 6), light('stroke', 'weather', 2, 20, { transient: true }), light('cam_cone', 'devices', 1, 1.1), light('kitview_fill', 'kitview', 0, 3)];
ls.lights.push(...scaled, ...kept); ls.ir.push(light('ir_sam', 'infrared', 1, 4)); ls.tube = true;
const all = [...scaled, ...kept, ls.ir[0]], f32 = (ls as any).f32 as Float32Array;
const up = (l: RtLight) => f32[(l.slot ?? -1) * 16 + 8], words = () => Array.from(((ls as any).u32 as Uint32Array).subarray(0, ls.activeCount * 16)).join(',');
const was = all.map(l => l.intensity);
ls.upload(0); const first = words(), night = new Map(all.map(l => [l, up(l)]));
check(ls.dayExp === 1 && ls.dayAmount === 1 && ls.activeCount === all.length, `the exposure is the night's as the light set is made, and all ${all.length} lights are taken`);
check(['fixture', 'fixture', 'hand', 'hand', 'vehicle', 'fixture', 'flash', 'flash'].every((c, i) => lightClass(scaled[i]) === c) && kept.every(l => lightClass(l) === 'none'), 'a bulb, a lantern and a lamp remembered in the fog are fixtures; a torch and the rail light hand lights; a van\'s lamp a vehicle\'s; a flash and a fire flashes; the sun, a stroke, a cone and the kit figure\'s lamp are nobody\'s to scale');
const K = (l: RtLight) => up(l) / night.get(l)!;
ls.dayExp = BREAK ? 1 : exp.midday; ls.upload(0);
check(near(K(bulb), exp.midday) && near(K(lantern), exp.midday) && near(K(ghost), exp.midday), `midday: a fixture goes up at the hour's exposure, ${one(K(bulb))}: its authored strength is its real one`);
check(near(K(torch) * 70, TRUE_CD.hand / LUX_PER_UNIT * exp.midday) && near(K(rail) * 60, TRUE_CD.hand / LUX_PER_UNIT * exp.midday) && near(K(van) * 150, TRUE_CD.vehicle / LUX_PER_UNIT * exp.midday), `a hand light goes up as ${TRUE_CD.hand.toLocaleString('en-US')} candela at that exposure whatever the game gave it (the torch ${one(K(torch))}, the rail light ${one(K(rail))}), a van's lamp as ${TRUE_CD.vehicle.toLocaleString('en-US')} (${one(K(van))})`);
check(near(K(flash), FLASH_TRUE * exp.midday) && near(K(fire), FLASH_TRUE * exp.midday), `a flash and a fire's light at ${FLASH_TRUE} times theirs (${one(K(flash))})`);
check(kept.every(l => up(l) === night.get(l)) && up(ls.ir[0]) === night.get(ls.ir[0]), 'the sun, a lightning stroke, a device\'s cone, the kit figure\'s lamp and an infrared illuminator go up as they were');
{ // the real proportions against the sun, at midday: a face 3 m down the rail light's beam, the floor 2.3 m under the bulb
  const sun = HOURS.midday.sun.intensity, beam = up(rail) / 9 / sun, pool = up(bulb) / (2.3 * 2.3) / sun;
  check(beam > 0.005 && beam < 0.03 && pool < 0.001 && pool > 0.00005, `against the midday sun the rail light puts ${(beam * 100).toFixed(1)} % on a wall 3 m off (a real weapon light: about 1 %) and the bulb ${(pool * 100).toFixed(3)} % on the floor under it (a real one: 0.015 %)`);
}
check(all.every((l, i) => l.intensity === was[i]), 'and no light\'s own intensity was written: the game\'s readers of the list see the authored one');

// (3) the night's hold, at dusk: nothing is drawn brighter than the night draws it
ls.dayExp = BREAK ? 1 : exp.dusk; ls.upload(0);
check(near(K(bulb), exp.dusk) && K(torch) <= 1 && K(torch) > 0.3 && K(van) <= 1 && near(K(torch), Math.min(1, TRUE_CD.hand / LUX_PER_UNIT / 70 * exp.dusk)) && near(K(flash), Math.min(1, FLASH_TRUE * exp.dusk)), `dusk: a fixture at ${one(K(bulb))}, the torch at ${K(torch).toFixed(2)} of its night strength, a van's lamp at ${K(van).toFixed(2)}, a flash at ${K(flash).toFixed(2)}: each its real strength at that exposure and never over the night's`);
{ const l = { ...torch }; check(lightDayK(l, 0.5) === 1 && lightDayK(bulb, 0.5) === 0.5 && lightDayK(bulb, 1) === 1 && lightDayK(torch, exp.midday, 0) === 1 && near(lightDayK(bulb, exp.midday, 0.5), Math.sqrt(exp.midday)), 'by the rule alone: at half the night\'s exposure a torch is held at its night strength while a bulb halves; at the night\'s exposure, or with the row at 0, everything is 1; the row at a half is half the stops'); }
check(near(emissiveDayK(exp.midday), EMISSIVE_TRUE * exp.midday) && emissiveDayK(exp.dusk) === 1 && emissiveDayK(1) === 1 && emissiveDayK(exp.midday, 0) === 1, `a glowing face is drawn at ${emissiveDayK(exp.midday).toFixed(2)} of its night glow at midday and in full at dusk and at night`);

// (4) back at the night's exposure
ls.dayExp = 1; ls.upload(0);
check(words() === first, 'back at the night\'s exposure the upload is word for word the first one');
process.exit(red ? 1 : 0);
