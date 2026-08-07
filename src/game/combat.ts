// Combat and kit: shots and hits, kills and what leaves dying hands, hand props / holsters / lasers, live throwables (canisters, stun) and their effects.
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { Vec3, v3, clamp, lerp, quat } from '../math/vec';
import { Character } from './character';
import { CanisterParams } from '../smoke/types';
import { Box, BoxFlag, makeBox } from '../scene/boxes';
import { Item } from './throwables';
import * as fx from './effects';
import { rigBoxes, holsterBoxes, RIG } from './rigProps';
import { tally } from './mission';
import type { Game, Guard, ThrowKind } from './game';

// ------------------------------------------------------------ shooting
export function fireWeapon(g: Game, shooter: Character, target: Vec3, isPlayer: boolean) {
  shooter.anim.fire();
  if (isPlayer) tally(g, 'shots');
  const muzzle = v3.copy(shooter.muzzle);
  let dir = v3.normalize(v3.sub(target, muzzle));
  if (!isPlayer) { // guards are imperfect shots: wider when the target is moving fast or far away (their first rounds tend to go wide, then they settle)
    const pl = g.player; const tv = Math.hypot(pl.char.vel[0], pl.char.vel[2]); const range = v3.dist(shooter.pos, pl.char.pos);
    const spread = 0.05 + Math.min(0.06, tv * 0.014) + Math.max(0, range - 8) * 0.006;
    dir = v3.normalize([dir[0] + (Math.random() - 0.5) * spread, dir[1] + (Math.random() - 0.5) * spread, dir[2] + (Math.random() - 0.5) * spread]);
  }
  // 1-frame emissive flash card (visible flame), boxes only
  g.fx.push({ box: makeBox({ c: v3.mad(muzzle, dir, isPlayer ? 0.05 : 0.09), h: isPlayer ? [0.018, 0.018, 0.045] : [0.03, 0.03, 0.08], yaw: v3.yawOf(dir), albedo: [1, 0.8, 0.5], emissive: isPlayer ? [40, 22, 8] : [90, 50, 16], flags: BoxFlag.Dynamic | BoxFlag.NoShadow }), ttl: 0.035 });
  // --- gas + light + powder: the graded shot from effects.ts — bore jet thrown down the barrel, two side-port puffs, the warm wisp that curls off
  // the barrel for seconds (tracks the muzzle, plus the small ejection-port curl), a narrow cone of spark streaks and the brief ~2000 K flash.
  // The suppressed Five-seveN only dims the light; the gas is the same gas.
  const track = () => ({ pos: shooter.muzzle, dir: shooter.gunDir });
  const port = () => ({ pos: v3.add(v3.mad(shooter.bones.handR ?? shooter.muzzle, shooter.gunDir, 0.06), [0, 0.06, 0]), dir: [0, 1, 0] as Vec3 });
  for (const L of fx.spawnShot(g.smoke, g.sparks, muzzle, dir, { seed: g.fxSeed++, lightGain: isPlayer ? fx.shot.suppressedGain : 1, track, eject: port }))
    g.engine.lights.flash(L.pos, L.color, L.intensity, L.range, L.ttl, shooter.id);
  g.audio.play(isPlayer ? 'pistolSuppressed' : 'pistolLoud', muzzle, 1);
  // --- hit scan
  let tHit = 60; let hitGuard: Guard | null = null; let playerHit = false;
  const st = g.col.raycast(muzzle, dir, tHit); if (st) tHit = st.t;   // statics + door leaves
  // loose furniture in the line of fire gets knocked (below); it stops the round only if nobody is standing behind it — a chair back or a carton is
  // concealment, not cover, which keeps hits consistent with guard sight (props never block line of sight)
  const ph = g.props.raycast(muzzle, dir, tHit);
  if (isPlayer) { for (const gd of g.guards) { const t = gd.char.rayHit(muzzle, dir, tHit); if (t >= 0 && t < tHit) { tHit = t; hitGuard = gd; } } }
  else { const t = g.player.char.rayHit(muzzle, dir, tHit); if (t >= 0) { tHit = t; playerHit = true; } }
  if (ph && !hitGuard && !playerHit) tHit = ph.t;
  const hitP = v3.mad(muzzle, dir, tHit);
  if (ph && ph.t <= tHit + 1e-3) { const J = isPlayer ? 4 : 6; g.props.applyImpulse(ph.prop, v3.mad(muzzle, dir, ph.t), [dir[0] * J, 0, dir[2] * J], shooter.id, true); }   // (not a prop standing behind the body the round stopped in) a few N·s, exaggerated over a real 5.7 / 9 mm round so it reads: cartons hop, chairs twitch and roll — and it is heard at the prop
  // bullet wake through smoke volumes
  for (let d = 0.3; d < Math.min(tHit, 25); d += 0.28) { const p = v3.mad(muzzle, dir, d); if (g.smoke.inSmokeDomain(p)) g.smoke.push({ pos: p, dir, speed: 5.5, radius: 0.07 }); }
  // impact flash + sound
  g.engine.lights.flash(v3.mad(hitP, dir, -0.05), [1, 0.8, 0.5], 1.5, 2.5, 0.05, 0);
  if (!hitGuard && !playerHit) g.audio.play('bulletImpact', hitP, 0.8);
  if (hitGuard) { killGuard(g, hitGuard, dir); g.audio.play('bodyHit', hitP, 0.8); }
  if (playerHit) { g.player.char.anim.hit(); g.player.hitFlash = 1; if (g.player.picking) { g.player.picking = null; g.player.fHeld = true; } g.audio.play('bodyHit', hitP, 0.9); if (!g.godMode) hitPlayer(g, dir); }   // (a round in the vest ends any lock work, god mode or not — and that F press is spent)
  // lights: a shot passing close to a fixture breaks it
  if (isPlayer) {
    for (const t of g.targets) {
      if (t.broken || !t.interactive || t.factor <= 0 && !t.on) continue;   // scenery (rack LEDs / glow strips) cannot be shot out
      const toT = v3.sub(t.pos, muzzle); const depth = v3.dot(toT, dir);
      if (depth > tHit + (t.kind === 'breaker' ? 0.08 : 0.3)) continue;                     // fixture is beyond what the round hit (wall-mounted breaker: no through-the-wall kills)
      const along = clamp(depth, 0, tHit); const dist = v3.len(v3.sub(toT, v3.scale(dir, along)));
      const rad = t.kind === 'emissive' ? 0.45 : t.kind === 'breaker' ? 0.4 : 0.3;
      if (dist < rad) {
        if (t.kind === 'breaker') { g.engine.lights.flash(t.pos, [0.6, 0.7, 1], 10, 5, 0.2, 0); g.audio.play('lightBreak', t.pos, 1); g.setBlackout(Infinity, 'shot'); t.broken = true; break; }
        t.broken = true; g.msg(`shot out ${t.name}`); g.lightEvent(t); g.engine.lights.flash(t.pos, [0.7, 0.8, 1], 6, 4, 0.12, 0); g.audio.play('lightBreak', t.pos, 1); break;
      }
    }
  }
  g.events.push({ kind: isPlayer ? 'shot' : 'guardShot', pos: v3.copy(shooter.pos), time: g.time, loud: true, level: 1 });   // gunfire — suppressed or not — carries through the whole floor on a dead-quiet night (hearingCheck: everyone goes to high alert)
  if (isPlayer) g.player.noise = 1;   // suppressed, not silent: the whole floor hears it (hearingCheck)
}

export function killGuard(g: Game, gd: Guard, dir: Vec3, quiet = false) {
  // (below: state checks come first everywhere, but a corpse should not carry choreography around)
  if (gd.bubble && g.time - gd.bubble.t < 0.8) g.msg('guard: ' + gd.bubble.text);   // he did not get to finish saying it on screen
  if (gd.state === 'dead') return;
  gd.script = null;   // (his lockdown role — leader / post — deliberately stays on the corpse: the others only re-deal once they KNOW, see superviseLockdown)
  const torch = gd.state === 'patrol' && !gd.drawn;   // what was in the hand: the torch on a calm patrol, the drawn pistol otherwise (alarmed floor, or any other state)
  tally(g, quiet ? 'takedowns' : 'kills');   // (only the player kills anyone: his rounds via fireWeapon, or the takedown)
  gd.state = 'dead'; gd.speed = 0; gd.diedAt = g.time;
  gd.dropped = dropFromHand(g, gd.char, torch ? 'torch' : 'pistol', torch ? gd.beamDir : gd.char.gunDir, quiet ? [0, 0, 0] : dir, false, quiet);   // torch on patrol, drawn pistol otherwise (the other stays holstered); a takedown lays it down quietly
  // hand him to the ragdoll: thrown along the round (falls away from the shooter) or, for a takedown, folding forward where he stood; it collides with the
  // walls / furniture / door leaves itself, and the thud plays when the trunk actually lands (updateGuard → bodyThud)
  gd.char.ragdollize(g.col, dir, quiet);
  g.msg(quiet ? 'takedown' : 'guard down');
}

/** What a dying hand lets go of: an item at the hand, carrying the body's motion plus a nudge along the round, spinning as it goes. It clatters (audible to the room)
 *  on its first hard bounces. kind 'torch' | 'pistol'; fill 1 = the player's suppressed Five-seveN. */
export function dropFromHand(g: Game, c: Character, kind: 'torch' | 'pistol', along: Vec3, dir: Vec3, suppressed = false, quiet = false): Item {
  const h = c.bones.handR ?? v3.add(c.pos, [0, 1.0, 0]);
  const v: Vec3 = [c.vel[0] * 0.6 + dir[0] * 1.1 + (Math.random() - 0.5) * 0.6, 0.9 + Math.random() * 0.6, c.vel[2] * 0.6 + dir[2] * 1.1 + (Math.random() - 0.5) * 0.6];
  const it = g.items.spawn(kind, [h[0], Math.max(0.25, h[1]), h[2]], v, { who: c.id, fill: suppressed ? 1 : 0, heading: v3.yawOf(along), yaw: v3.yawOf(along), life: kind === 'pistol' ? 240 : 90 });
  if (kind === 'pistol') it.rounds = suppressed ? g.player.pistol.roundsReady : 3 + Math.floor(Math.random() * 8);   // his sidearm has whatever he had left; yours keeps its real count
  let clatters = 0;
  it.onBounce = (sp, p) => { if (sp < 0.7) return; g.audio.play('magDrop', p, Math.min(1, sp / 3) * (quiet ? 0.5 : 1), { rate: kind === 'pistol' ? 0.72 : 0.85 }); if (!quiet && clatters++ < 2) g.events.push({ kind: 'prop', pos: v3.copy(p), time: g.time, loud: false, level: Math.min(0.5, 0.2 + sp * 0.08), who: 0 }); };   // who 0: nobody living made it — a clatter the room cannot explain (a guard-attributed noise would be shrugged off)
  return it;
}

export function hitPlayer(g: Game, fromDir: Vec3) {
  const pl = g.player; if (pl.down) return;
  pl.hitsLeft = Math.max(0, pl.hitsLeft - 1);
  if (pl.hitsLeft === 0) {
    if (pl.dragging) g.toggleDrag();                     // lets go of the ankles (unpins them)
    pl.down = true; pl.sprinting = false; pl.crouch = false; pl.char.vel = [0, 0, 0];
    pl.pistol.cancelReload(); pl.char.anim.cancelReload(); pl.throwHeld = false; pl.pendingThrow = null;
    if (pl.slot === 1) dropFromHand(g, pl.char, 'pistol', pl.char.gunDir, fromDir, true);   // the Five-seveN leaves the hand too
    pl.char.ragdollize(g.col, fromDir, false);            // thrown along the round, like the guards; the thud plays when the trunk lands (updatePlayer)
    g.msg("you're down — Enter to restart the encounter");
  } else g.msg(pl.hitsLeft === 1 ? 'hit — one more and you are down' : 'you were hit');
}

/** Held / worn props as small oriented boxes (traced; lit lenses raster-only): the Five-seveN (+ can + rail light) in the player's hand and the trifocal goggles on
 *  their head, pistol or torch with a glowing lens for guards. All placement numbers live in rigProps.RIG (tune them in viewer.html). */
export function handProps(g: Game, out: Box[]) {
  const pl = g.player;
  const busy = pl.char.anim.hideHeldItem;   // hands on the lock tools (anim.lockpick): empty hand, the Five-seveN on the thigh, no can, no lit lens — whatever the kit says
  const held: 'smoke' | 'flash' | null = !pl.down && !busy && pl.slot !== 1 && !pl.dragging && !pl.sprinting && g.throwCount(pl.slot === 3 ? 'flash' : 'smoke') > 0 ? (pl.slot === 3 ? 'flash' : 'smoke') : null;
  rigBoxes(pl.char, { goggles: true, nv: pl.nv, slot: pl.down ? 2 : busy ? 0 : pl.slot, lightOn: pl.pistol.lightOn && !pl.down && !pl.sprinting && !busy, isGuard: false, holster: !pl.down || pl.slot !== 1, held }, out);   // canister up: gun on the thigh, can in the fist; down: whatever was in the hand has left it (dropFromHand) — a holstered gun stays holstered, the goggles stay on   // (the light itself is forced off while down / sprinting: no blazing lens then)   // (a downed player's light is forced off in updatePlayer: no lit lens on the body either)
  for (const gd of g.guards) {
    if (gd.state === 'dead') { if (gd.dropped?.kind === 'torch') holsterBoxes(gd.char, out, { slide: RIG.guardPistol.slide, grip: RIG.guardPistol.grip }); continue; }   // what he held is on the floor; a man who died on patrol still wears his sidearm
    const fl = gd.flashlight;   // torch carried in the hand on a calm patrol (the spot light sits at its lens), drawn pistol with a weapon light otherwise (an alarmed floor patrols drawn: Guard.drawn)
    rigBoxes(gd.char, { goggles: false, nv: false, slot: 1, lightOn: fl.enabled, isGuard: true, torch: gd.state === 'patrol' && !gd.drawn, beamDir: gd.beamDir, lensColor: fl.color }, out);
    if (gd.state === 'alert' || gd.state === 'search') laserBoxes(g, gd, out);   // drawn and hunting: the sidearm's laser is on
  }
}

/** A hunting guard's weapon laser: a hair-thin emissive line from the muzzle to whatever it meets (wall, prop, Sam) and a brighter dot there. Purely
 *  emissive geometry flagged NoShadow, so no ray (light, sight, bullet, cursor) ever sees it; in the haze it reads exactly like a laser in smoke, and it
 *  tells you precisely where he is looking. */
export function laserBoxes(g: Game, gd: Guard, out: Box[]) {
  const c = gd.char; const from = v3.mad(c.muzzle, c.gunDir, 0.02); const dir = c.gunDir;
  if (!isFinite(dir[0]) || v3.len(dir) < 0.5) return;
  let L = 30; const wh = g.engine.world.raycast(from, dir, L, BoxFlag.NoShadow, c.id & 0xff); if (wh) L = wh.t;   // ceilings (NoPrimary) DO stop it — a flailing dazzled man must not paint a 30 m line through the roof
  const ph = g.props.raycast(from, dir, L); if (ph) L = Math.min(L, ph.t);
  { const tp = g.player.char.rayHit(from, dir, L); if (tp >= 0) L = Math.min(L, tp); }   // (a live Sam stops it; bodies on the floor do not — rayHit ignores the dead)
  for (const o of g.guards) { if (o === gd || o.state === 'dead') continue; const t = o.char.rayHit(from, dir, L); if (t >= 0) L = Math.min(L, t); }
  if (L < 0.05) return;
  const red: Vec3 = [1, 0.06, 0.03];
  const hw = Math.max(0.0022, 0.45 * g.mPerPx); const gain = 0.0022 / hw;   // never thinner than ~a pixel (a 4 mm beam at 15 m is a crawling dashed line otherwise); widened → dimmed by the same factor so the line carries the same light
  const up: Vec3 = Math.abs(dir[1]) < 0.95 ? [0, 1, 0] : [1, 0, 0]; const x = v3.normalize(v3.cross(up, dir)); const y = v3.cross(dir, x);
  out.push(makeBox({ c: v3.mad(from, dir, L / 2), h: [hw, hw, L / 2], yaw: v3.yawOf(dir), rot: quat.fromBasis(x, y, dir), albedo: [0.02, 0, 0], emissive: v3.scale(red, 9 * gain), flags: BoxFlag.NoShadow | BoxFlag.Dynamic }));
  if (wh || ph || L < 30) { const end = v3.mad(from, dir, L - 0.006); out.push(makeBox({ c: end, h: [0.011, 0.011, 0.011], yaw: 0, albedo: [0.05, 0, 0], emissive: v3.scale(red, 40), flags: BoxFlag.NoShadow | BoxFlag.Dynamic })); }   // the dot: a small bright bead where the beam lands
}

export function updateItems(g: Game, dt: number) {
  g.items.update(dt);
  for (const it of g.items.items) {
    if (it.kind === 'smoke') {
      if (!it.fired) { it.fuse -= dt; if (it.fuse <= 0) { it.fired = true; startCanister(g, it); } }
      else if (it.emitT > 0) { it.emitT -= dt; }
    } else if (it.kind === 'flash' && !it.fired) {
      // stun canister: the fuse runs from first contact (it goes off where it lands, ON a surface — the whole shape depends on that). maxAir is a
      // cap on FLIGHT only — once it has landed the fuse alone decides, or a long throw would eat into it and `bang.fuse` would stop meaning anything
      const landed = it.bounces > 0 || it.settled;
      if (landed) it.fuse -= dt;
      if (it.fuse <= 0 || (!landed && it.age > fx.bang.maxAir)) { it.fired = true; detonateStun(g, it); }
    }
  }
}

/** the attached solver's canister tuning (SmokeSystem.params), or null behind a stand-in without any */
export function smokeParams(g: Game): CanisterParams | null { return g.smoke.params ?? null; }

export function startCanister(g: Game, it: Item) {
  let seconds: number;
  if (fx.flags.ventCanister) {
    // the graded can: a cold dense jet along the throw direction that hugs the floor and piles up (effects.ts `vent`); the mouth follows the can while it still rolls
    const yaw = it.heading;
    seconds = fx.spawnVent(g.smoke, fx.ventMouth(it.pos, yaw), yaw, { track: () => ({ pos: fx.ventMouth(it.pos, yaw), dir: [Math.sin(yaw), 0, Math.cos(yaw)] }) });
  } else {
    const P = smokeParams(g) ?? { canisterDuration: 32, canisterDensity: 95, canisterRadius: 0.2, canisterTemp: 1.6, canisterSpeed: 1.9 }; seconds = P.canisterDuration;
    g.smoke.emit({ pos: v3.add(it.pos, [0, 0.1, 0]), dir: [0, 1, 0], speed: P.canisterSpeed, radius: P.canisterRadius, density: P.canisterDensity, temperature: P.canisterTemp, ttl: P.canisterDuration, age: 0, kind: 'canister',
      track: () => ({ pos: v3.add(it.pos, [0, 0.12, 0]), dir: [0, 1, 0] }) });
  }
  it.emitT = seconds;
  g.audio.play('smokeHiss', it.pos, 1, { duration: seconds });
  g.msg('smoke deployed');
  g.events.push({ kind: 'shot', pos: v3.copy(it.pos), time: g.time, loud: false });   // the pop draws attention nearby
}

/** The stun canister goes off: graded burst (jets, wisp, sparks + trails) on the coarse smoke lattice, the white flash, the bang every guard on the
 *  floor hears, and the gameplay effect — guards with line of sight within bang.dazzleRadius are dazzled (Guard.dazzledUntil: canSee() fails, they
 *  cannot fire, and reactT roots them to the spot), then go and look at where it went off like any other loud noise. */
export function detonateStun(g: Game, it: Item) {
  const B = fx.bang; const at: Vec3 = [it.pos[0], it.pos[1] + B.height, it.pos[2]];
  const L = fx.spawnBurst(g.smoke, g.sparks, at, g.fxSeed++);
  const light = g.engine.lights.flash(L.pos, L.color, L.intensity, L.range, L.ttl, 0); if (L.radius) light.radius = L.radius;
  g.audio.play('stunBang', at, 1.0);
  const eye = g.player.char.bones.head ?? v3.add(g.player.char.pos, [0, 1.6, 0]);
  if (!g.player.down && v3.dist(eye, at) < B.dazzleRadius && !g.col.segmentBlocked(v3.add(at, [0, 0.3, 0]), eye)) g.audio.play('earRing', null, clamp(1.2 - v3.dist(eye, at) / B.dazzleRadius, 0.25, 0.8));
  g.events.push({ kind: 'bang', pos: v3.copy(at), time: g.time, loud: true });
  let n = 0;
  if (fx.flags.dazzle) for (const gd of g.guards) {
    if (gd.state === 'dead') continue;
    const head = gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]); const d = v3.dist(head, at);
    if (d > B.dazzleRadius || g.col.segmentBlocked(v3.add(at, [0, 0.3, 0]), head)) continue;   // behind a wall or a closed door: only hears it
    const secs = lerp(B.dazzleSeconds, B.dazzleMin, clamp(d / B.dazzleRadius, 0, 1));
    gd.dazzledUntil = Math.max(gd.dazzledUntil, g.time + secs); gd.reactT = Math.max(gd.reactT, secs);   // reactT > 0 is the AI's existing "stand and stare" — reused as the stagger
    gd.awareness = Math.max(gd.awareness, 0.6); if (gd.state !== 'alert') gd.lastKnown = v3.copy(at);
    gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.char.anim.hit(); n++;
  }
  g.msg(n ? `stun canister — ${n} guard${n > 1 ? 's' : ''} dazzled` : 'stun canister went off');
}

/** Sandbox / panel: drop an armed canister at `p` as if it had just landed there, thrown from where the player stands (short fuse, no inventory cost). */
export function dropThrowable(g: Game, kind: ThrowKind, p: Vec3) {
  const from = g.player.char.pos; const heading = v3.yawTo(from, p);
  const it = g.items.spawn(kind === 'flash' ? 'flash' : 'smoke', [p[0], Math.max(p[1], 0) + 0.25, p[2]], [Math.sin(heading) * 0.5, 0, Math.cos(heading) * 0.5], { fuse: 0.35, heading });
  it.onBounce = (sp, q) => g.audio.play('canisterBounce', q, Math.min(1, sp / 4));
}
