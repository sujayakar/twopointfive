// Lights as game objects: switchable fixtures and their groups, the OCP, the mains breaker (blackout / restrike / beacons / a guard sent to reset it), light events into the hearing model.
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { Vec3, v3 } from '../math/vec';
import { missionLive } from './mission';
import type { Game, Guard, LightTarget } from './game';

export function updateTargets(g: Game, dt: number) {
  const time = g.time;
  const bo = g.blackout;
  if (bo.active && !bo.permanent && time >= bo.until) g.endBlackout();
  // a guard who reaches the (intact) panel resets it after fumbling for a moment
  if (bo.active && !bo.permanent && time - bo.since > 3) {
    const bp = g.level.breaker.pos; let near: Guard | null = null;
    for (const gd of g.guards) if (gd.state !== 'dead' && Math.hypot(gd.char.pos[0] - bp[0], gd.char.pos[2] - (bp[2] - 0.6)) < 1.3) near = gd;
    g.breakerFixT = near ? g.breakerFixT + dt : 0;
    if (g.breakerFixT > 2.2 && near) { g.say(near, "breaker's back in", true); g.endBlackout(); }
  }
  // emergency beacons: spin up ~0.8 s after the trip, keep turning for a second after power returns; each is slaved to its dome fixture
  const beaconsOn = bo.active ? time - bo.since > 0.8 : time - bo.restoredAt < 1.2;
  for (const b of g.beacons) {
    b.target.on = beaconsOn;
    b.light.enabled = beaconsOn && b.target.factor > 0;              // shot-out / OCP'd dome → no beam either (factor is last frame's)
    const a = time * 3.3 + b.phase;                                   // ceiling-hung: full rotation about the vertical, tilted down
    b.light.dir = v3.normalize([Math.cos(a), -0.5, Math.sin(a)]);
    b.light.intensity = b.light.peakIntensity;
  }
  for (const t of g.targets) {
    let f = t.on && !t.broken ? 1 : 0;
    if (f > 0 && t.disabledUntil > 0) { f *= supply(time, t.disabledUntil); if (time > t.disabledUntil + 1.2) t.disabledUntil = -1; }   // OCP knock-out + restrike
    if (f > 0 && t.mains) f *= bo.active ? 0 : supply(time, bo.restoredAt + t.stagger);                                             // building supply
    if (f > 0 && t.fluorescentFlicker) {
      const s = Math.sin(time * 3.1) + Math.sin(time * 7.3 + 2) * 0.6; // occasional dropout bursts
      if (s > 1.15) f *= (Math.sin(time * 90) > 0 ? 0.15 : 1);
    }
    t.factor = f;
    if (t.kind === 'breaker') { const e: Vec3 = t.broken ? [0, 0, 0] : bo.active ? [5, 0.3, 0.1] : [0.3, 4, 0.6]; for (const bi of t.boxes) g.engine.world.statics[bi].emissive = e; continue; }
    if (t.kind === 'analytic' && t.light) { t.light.enabled = f > 0.001; t.light.intensity = t.baseIntensity * f; }
    if (t.areaLights) { const es = g.engine.settings.emissiveScale; for (const al of t.areaLights) { al.light.enabled = f > 0.001; al.light.intensity = al.light.peakIntensity = al.base * f * es; } }
    for (const bi of t.boxes) { const b = g.engine.world.statics[bi]; b.emissive = [t.baseEmissive[0] * f, t.baseEmissive[1] * f, t.baseEmissive[2] * f]; }
  }
}

/** Trip the mains: everything fed from the breaker goes dark, emergency beacons spin up. Restrike is staggered per contactor group. */
export function setBlackout(g: Game, seconds: number, cause: 'ocp' | 'shot' | 'debug') {
  const bo = g.blackout; const permanent = !isFinite(seconds);
  if (bo.active && bo.permanent) return;
  if (!bo.active) bo.since = g.time;
  bo.active = true; bo.permanent = permanent; bo.until = permanent ? Infinity : Math.max(bo.until, g.time + seconds);
  g.audio.play('powerDown', g.level.breaker.pos, 1.0); g.audio.play('powerDown', null, 0.35);
  g.msg(cause === 'shot' ? 'breaker destroyed — mains gone for good' : 'mains tripped — emergency lighting');
  if (cause !== 'debug' && missionLive(g)) g.mission.stats.blackout = true;   // the player's own doing (OCP / round), not the panel button or the tour
  // guards: everyone gets edgy; the nearest one goes to check the breaker
  let best: Guard | null = null, bd = 1e9;
  for (const gd of g.guards) {
    if (gd.state === 'dead') continue;
    if (gd.state === 'patrol') { gd.awareness = Math.max(gd.awareness, 0.34); gd.reactT = 1.0 + Math.random(); }
    const d = v3.dist(gd.char.pos, g.level.breaker.pos); if (d < bd && gd.state !== 'alert') { bd = d; best = gd; }
  }
  if (best && !best.task) {
    const goal: Vec3 = [g.level.breaker.pos[0] - 0.4, 0, g.level.breaker.pos[2] - 0.9];
    best.task = { kind: 'breaker', pos: goal }; best.lastKnown = v3.copy(goal); best.awareness = Math.max(best.awareness, 0.42);
    if (best.state === 'patrol') { best.state = 'suspicious'; best.reactT = 1.2; best.searchT = 0; }
    g.say(best, "power's out — checking the breaker", true); g.audio.play('radio', best.char.pos, 0.6);
  }
}

export function endBlackout(g: Game) {
  const bo = g.blackout; if (!bo.active) return;
  bo.active = false; bo.permanent = false; bo.until = -1; bo.restoredAt = g.time; g.breakerFixT = 0;
  g.audio.play('powerUp', g.level.breaker.pos, 1.0); g.audio.play('powerUp', null, 0.3); g.msg('mains restored');
  for (const gd of g.guards) if (gd.task?.kind === 'breaker') gd.task = null;
}

export function lightEvent(g: Game, t: LightTarget) {
  g.events.push({ kind: 'lightOut', pos: [t.pos[0], 0, t.pos[2]], time: g.time, loud: false });
}

export function ocp(g: Game, t: LightTarget | null) {
  const o = g.player.ocp;
  if (!o.ready()) { g.msg('OCP recharging'); g.audio.play('deny', null, 0.5); return; }
  if (!t) {
    const gd = g.aimGuard;
    if (gd && gd.state !== 'dead') {
      o.use(); g.audio.play('ocp', g.player.char.muzzle, 1);
      gd.lightDeadUntil = g.time + g.tune.ocpDuration * 0.7; g.msg("OCP → guard's flashlight fried");
      if (gd.state === 'patrol') { gd.awareness = Math.max(gd.awareness, 0.32); gd.lastKnown = v3.add(gd.char.pos, v3.scale(gd.char.forward(), 2)); }
      g.audio.play('lightOff', gd.char.pos, 0.6);
      return;
    }
    g.audio.play('deny', null, 0.5); return;   // nothing under the cursor: just the click
  }
  if (t.kind === 'breaker') {
    const eye = g.player.char.bones.head ?? v3.add(g.player.char.pos, [0, 1.6, 0]);
    if (g.col.segmentBlocked(eye, v3.add(t.pos, [0, 0, -0.18]))) { g.msg('OCP: no line of sight to the breaker'); g.audio.play('deny', null, 0.5); return; }
    if (g.blackout.active) { g.msg('mains already down'); g.audio.play('deny', null, 0.5); return; }
    o.use(); g.audio.play('ocp', g.player.char.muzzle, 1); g.setBlackout(g.tune.ocpDuration * 2.5, 'ocp'); return;
  }
  if (t.factor <= 0) { g.msg(`OCP: ${t.name} already off`); return; }
  o.use();
  g.audio.play('ocp', g.player.char.muzzle, 1);
  t.disabledUntil = g.time + g.tune.ocpDuration; g.msg(`OCP → ${t.name} disabled`); g.lightEvent(t);
  g.audio.play('lightOff', t.pos, 0.7);
}

// ------------------------------------------------------------ lights
export function setGroup(g: Game, group: string, on: boolean) { for (const t of g.targets) if (t.group === group) t.on = on; }

export function toggleTarget(g: Game, t: LightTarget) { t.on = !t.on; t.broken = false; }

/** All fixtures repaired, OCP timers cleared, mains back (panel 'repair' + encounter restart). */
export function repairLights(g: Game) { for (const t of g.targets) { t.broken = false; t.disabledUntil = -1; } if (g.blackout.active) g.endBlackout(); }

/** 0 while a fixture has no supply, then the fluorescent restrike flicker for 1.2 s after `since`, then 1. */
export function supply(time: number, darkUntil: number): number {
  if (time < darkUntil) return 0;
  if (time < darkUntil + 1.2) { const x = (time - darkUntil) / 1.2; const s = Math.sin(time * 45) * Math.sin(time * 13.7 + 1.3); return s > 0.65 - x * 0.9 ? 1 : 0.05; }
  return 1;
}
