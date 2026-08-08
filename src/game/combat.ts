// Combat and kit: shots and hits, kills and what leaves dying hands, hand props / holsters / lasers, live throwables (canisters, stun) and their effects.
// (Split out of game.ts mechanically: plain functions taking the Game as their first argument; Game keeps one-line delegators for
//  anything other files call.)
import { Vec3, v3, clamp, lerp, quat } from '../math/vec';
import { Character } from './character';
import { CanisterParams } from '../smoke/types';
import { Box, BoxFlag, makeBox, rayBox } from '../scene/boxes';
import { Item } from './throwables';
import * as fx from './effects';
import { rigBoxes, holsterBoxes, RIG } from './rigProps';
import { tally } from './mission';
import { witnessKill, hostageHitBark } from './guards';
import type { Game, Guard, ThrowKind, DownKind, DownBy } from './game';

// ------------------------------------------------------------ shooting
/** One round from `shooter`'s muzzle at `target`. The player's own shot goes exactly where it is sent unless `spread` says otherwise (fired one-handed past a man in
 *  his arm: player.ts updateHold); a guard's spread is his own business below. The man Sam is holding is never in the way of Sam's round (the muzzle sits beside
 *  his jaw, inside his own body cylinder's reach: bodyHit's contact rule would put it in him). */
export function fireWeapon(g: Game, shooter: Character, target: Vec3, isPlayer: boolean, spread = 0) {
  shooter.anim.fire();
  if (isPlayer) tally(g, 'shots');
  const muzzle = v3.copy(shooter.muzzle);
  let dir = v3.normalize(v3.sub(target, muzzle));
  if (!isPlayer) { // guards are imperfect shots: wider when the target is moving fast or far away (their first rounds tend to go wide, then they settle)
    const pl = g.player; const tv = Math.hypot(pl.char.vel[0], pl.char.vel[2]); const range = v3.dist(shooter.pos, pl.char.pos);
    spread = 0.05 + Math.min(0.06, tv * 0.014) + Math.max(0, range - 8) * 0.006;
  }
  if (spread > 0) dir = v3.normalize([dir[0] + (Math.random() - 0.5) * spread, dir[1] + (Math.random() - 0.5) * spread, dir[2] + (Math.random() - 0.5) * spread]);
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
  const through = armThrough(g, shooter); if (through) tHit = 0;      // the pistol itself out through a panel (below): the round goes into that, at the muzzle
  // loose furniture in the line of fire gets knocked (below); it stops the round only if nobody is standing behind it — a chair back or a carton is
  // concealment, not cover, which keeps hits consistent with guard sight (props never block line of sight)
  const ph = through ? null : g.props.raycast(muzzle, dir, tHit);
  const contact = (c: Character) => { const h = c.bones.hips ?? c.pos; return (!st || st.t > Math.hypot(h[0] - muzzle[0], h[2] - muzzle[2])) && contactClear(g, shooter, c); };   // a contact shot (the muzzle already inside him, bodyHit) counts only with nothing solid nearer along the bore than his middle and no panel between the two chests
  // The man in Sam's arm (player.ts Hold — the human shield): a colleague's round meets whichever of the two bodies it reaches first (from in front of the pair that
  // is him: he dies by his own side's hand and Sam is untouched; from behind, Sam); Sam's own rounds never test him (the muzzle rides beside his ear, inside his
  // cylinder — bodyHit's contact rule would put every one of them in him; he cannot shoot his own shield). Every other colleague stays transparent to their fire:
  // muzzle discipline is what keeps those rounds in, not luck.
  const held = g.player.holding?.g ?? null; let hostageHit = false;
  if (through) { /* into the panel at the muzzle: nobody beyond it */ }
  else if (isPlayer) { for (const gd of g.guards) { if (gd === held) continue; const t = bodyHit(gd.char, muzzle, dir, tHit, contact); if (t >= 0 && t < tHit) { tHit = t; hitGuard = gd; } } }
  else {
    const tH = held && held.state !== 'dead' && held.char !== shooter ? bodyHit(held.char, muzzle, dir, tHit, contact) : -1;
    const t = bodyHit(g.player.char, muzzle, dir, tHit, contact);
    if (tH >= 0 && (t < 0 || tH < t)) { tHit = tH; hitGuard = held; hostageHit = true; }
    else if (t >= 0) { tHit = t; playerHit = true; }
  }
  if (ph && !hitGuard && !playerHit) tHit = ph.t;
  const hitP = v3.mad(muzzle, dir, tHit);
  if (ph && ph.t <= tHit + 1e-3) { const J = isPlayer ? 4 : 6; g.props.applyImpulse(ph.prop, v3.mad(muzzle, dir, ph.t), [dir[0] * J, 0, dir[2] * J], shooter.id, true); }   // (not a prop standing behind the body the round stopped in) a few N·s, exaggerated over a real 5.7 / 9 mm round so it reads: cartons hop, chairs twitch and roll — and it is heard at the prop
  // bullet wake through smoke volumes
  for (let d = 0.3; d < Math.min(tHit, 25); d += 0.28) { const p = v3.mad(muzzle, dir, d); if (g.smoke.inSmokeDomain(p)) g.smoke.push({ pos: p, dir, speed: 5.5, radius: 0.07 }); }
  // impact flash + sound
  g.engine.lights.flash(v3.mad(hitP, dir, -0.05), [1, 0.8, 0.5], 1.5, 2.5, 0.05, 0);
  if (!hitGuard && !playerHit) g.audio.play('bulletImpact', hitP, 0.8);
  if (hitGuard) {   // (by 'guard': the held man, above — the only colleague a guard's round ever tests)
    killGuard(g, hitGuard, dir, false, 'shot', isPlayer ? 'player' : 'guard'); g.audio.play('bodyHit', hitP, 0.8);
    if (hostageHit) { const sh = g.guards.find(x => x.char === shooter); if (sh && sh.state !== 'dead') hostageHitBark(g, sh, hitGuard); }   // '…christ — I hit him': whoever fired it says so (the shot itself has everybody's ears; the hold drops from under Sam next frame — updateHold — and the men watching find the body there and then)
  }
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

/** The pistol itself poked out through a panel — the gun shoulder and the muzzle on opposite sides of a static or of a leaf standing (as good as) shut: a man
 *  pressed to a door has the reach to put his muzzle out of its far face, and a bore that starts beyond the wood sees nothing of it. fireWeapon puts that round
 *  into the panel at the muzzle; the guard AI (guards.ts) will not fire it, and steps clear if it lasts. An open leaf's free edge does not count (solidBetween). */
export function armThrough(g: Game, c: Character): boolean { return solidBetween(g, c.bones.upperArmR ?? v3.add(c.pos, [0, 1.4, 0]), c.muzzle); }

/** A panel between `from` and `to` — the things that keep two pressed men apart: a static (partition, jamb), or a leaf standing shut or as good as (within 0.3 rad
 *  of its frame: the SoundSeal bit Door.place keeps — latched or not, a leaf pulled to and resting on somebody is a full panel). A leaf standing open does not
 *  count: at its free edge the two are as good as touching, and it yields to them. (Short segments only: an arm, or two chests within a metre or so.) */
function solidBetween(g: Game, from: Vec3, to: Vec3): boolean {
  if (g.col.segmentBlockedDynamic(from, to, BoxFlag.SoundSeal)) return true;
  const d = v3.sub(to, from); const L = v3.len(d); if (L < 1e-3) return false; const rd = v3.scale(d, 1 / L);
  const n = g.col.gather(Math.min(from[0], to[0]) - 0.1, Math.min(from[2], to[2]) - 0.1, Math.max(from[0], to[0]) + 0.1, Math.max(from[2], to[2]) + 0.1, Math.min(from[1], to[1]) - 0.1, Math.max(from[1], to[1]) + 0.1, solidScratch);
  for (let i = 0; i < n; i++) if (rayBox(from, rd, g.col.boxes[solidScratch[i]], L)) return true;
  return false;
}
const solidScratch = new Int32Array(64);
/** May a contact shot from `a` count on `b` (bodyHit): no panel between their chests — each shoved to his own face of a shut door, the pistol's reach still puts
 *  the muzzle inside the other's cylinder, and that must not fire into a chest through the wood. Only asked when they are close enough to be in contact. */
function contactClear(g: Game, a: Character, b: Character): boolean {
  if (v3.distXZ(a.pos, b.pos) > 1.2) return true;   // (not in contact anyway: bodyHit's inside test cannot pass, whatever this says)
  const from = v3.add(a.pos, [0, 1.1, 0]); const h = b.bones.hips ?? b.pos; const to: Vec3 = [h[0], 1.1, h[2]];
  return !solidBetween(g, from, to);
}

/** Character.rayHit — where a round from `ro` along `rd` enters the man — plus the contact shot: a muzzle already INSIDE his body cylinder (pressed into his chest
 *  at arm's length, the way a guard who has pushed through a leaf onto Sam ends up) is a hit where it is. rayHit only reports rounds entering the cylinder from
 *  outside, so point-blank was a blind spot: a man planted on top of Sam firing round after round clean through him. `contact` — false, or a test paid for only
 *  once the muzzle is found inside him (fireWeapon: no panel between the two, nothing solid nearer along the bore) — gates that case; the lasers ask for the
 *  entering hit only (a stacked pair's beams pass through each other's backs as they always did rather than blank). */
export function bodyHit(c: Character, ro: Vec3, rd: Vec3, tmax: number, contact: boolean | ((c: Character) => boolean) = true): number {
  const t = c.rayHit(ro, rd, tmax); if (t >= 0 || !c.alive || !contact) return t;
  const cx = c.bones.hips?.[0] ?? c.pos[0], cz = c.bones.hips?.[2] ?? c.pos[2]; const top = (c.bones.headTop?.[1] ?? c.pos[1] + 1.75) + 0.25;   // (+0.25: a standing man's muzzle pressed down onto a crouched one sits about at his crown)
  if (!(Math.hypot(ro[0] - cx, ro[2] - cz) < 0.28 && ro[1] > c.pos[1] + 0.05 && ro[1] < top)) return -1;   // (the cylinder rayHit tests: 0.28 about the hips, soles to crown)
  return contact === true || contact(c) ? 0 : -1;   // (the panel test is only paid for once the muzzle IS inside him)
}

/** A man goes down: `how` he went (shot = dead; struck — the takedown's cross — or choked = out cold, breathing: Guard.downKind, isBreathing) and `by` whose hand
 *  (only the player's own doing goes on the debrief card: a kill for a shot, a knockout otherwise). His state is 'dead' whichever it was — the floor treats a man
 *  out cold exactly as a corpse (he fails the radio check, his body raises the alarm, his role is re-dealt once they know) — only what the men SAY over him, the
 *  card, and the witnesses' memory of it differ. `quiet` is the physical side: no clatter, the item laid down, the body folding where it stood. The defaults keep
 *  the two old call shapes meaning what they meant: (dir) = shot by the player, (dir, true) = the takedown.
 *  `line`: the log line if the default for `how` does not say it (the pistol-whip is 'struck' like the takedown, but not a takedown).
 *  TODO (docs/internal/grab-interrogate-design.md §6, item 9b): a breathing man could come round after a few minutes — there is no way back from a ragdoll today
 *  (Character.ragdollize is one-way, no get-up clip), so nobody wakes; and a body on the floor cannot be shot (Character.rayHit skips the fallen), so a knockout
 *  cannot be finished off either. */
export function killGuard(g: Game, gd: Guard, dir: Vec3, quiet = false, how: DownKind = quiet ? 'struck' : 'shot', by: DownBy = 'player', line?: string) {
  // (below: state checks come first everywhere, but a corpse should not carry choreography around)
  if (gd.bubble && g.time - gd.bubble.t < 0.8) g.msg('guard: ' + gd.bubble.text);   // he did not get to finish saying it on screen
  if (gd.state === 'dead') return;
  gd.script = null;   // (his lockdown role — leader / post — deliberately stays on the corpse: the others only re-deal once they KNOW, see superviseLockdown)
  const torch = gd.state === 'patrol' && !gd.drawn;   // what was in the hand: the torch on a calm patrol, the drawn pistol otherwise (alarmed floor, or any other state)
  if (by === 'player') tally(g, how === 'shot' ? 'kills' : 'knockouts');   // (a man dropped by his own side's round or by the world is on nobody's card)
  witnessKill(g, gd, how, by);   // whoever has it in view remembers seeing it done, and to whom (before his state flips: the observers are the OTHER living men)
  gd.state = 'dead'; gd.speed = 0; gd.diedAt = g.time; gd.downKind = how; gd.downBy = by;
  gd.dropped = dropFromHand(g, gd.char, torch ? 'torch' : 'pistol', torch ? gd.beamDir : gd.char.gunDir, quiet ? [0, 0, 0] : dir, false, quiet);   // torch on patrol, drawn pistol otherwise (the other stays holstered); a takedown lays it down quietly
  // hand him to the ragdoll: thrown along the round (falls away from the shooter) or, for a takedown, folding forward where he stood; it collides with the
  // walls / furniture / door leaves itself, and the thud plays when the trunk actually lands (updateGuard → bodyThud)
  gd.char.ragdollize(g.col, dir, quiet);
  g.msg(line ?? (how === 'choked' ? 'choked out — he\'ll keep' : how === 'struck' ? 'takedown — out cold' : by === 'guard' ? 'guard down — hit by his own side' : 'guard down'));
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
    if (pl.holding) g.freeHeld('shot');                  // the man in his arm staggers free — and turns on him knowing everything (player.ts handBack)
    pl.down = true; pl.sprinting = false; pl.crouch = false; pl.char.vel = [0, 0, 0];
    if (pl.wall) { pl.wall = null; pl.char.anim.stance = 'none'; pl.char.anim.lookYawExtra = 0; }   // off the wall the moment he drops (updatePlayer's down branch would only let go next frame)
    pl.pistol.cancelReload(); pl.char.anim.cancelReload(); pl.throwHeld = false; pl.pendingThrow = null;
    if (pl.slot === 1 && !pl.holstered) dropFromHand(g, pl.char, 'pistol', pl.char.gunDir, fromDir, true);   // the Five-seveN leaves the hand too (a holstered one stays on the thigh)
    pl.char.ragdollize(g.col, fromDir, false);            // thrown along the round, like the guards; the thud plays when the trunk lands (updatePlayer)
    g.msg("you're down — Enter to restart the encounter");
  } else g.msg(pl.hitsLeft === 1 ? 'hit — one more and you are down' : 'you were hit');
}

/** Held / worn props as small oriented boxes (traced; lit lenses raster-only): the Five-seveN (+ can + rail light) in the player's hand and the trifocal goggles on
 *  their head, pistol or torch with a glowing lens for guards. All placement numbers live in rigProps.RIG (tune them in viewer.html). */
export function handProps(g: Game, out: Box[]) {
  const pl = g.player;
  const busy = pl.char.anim.hideHeldItem;   // hands on the lock tools (anim.lockpick) or both round a held man (the arm variant): empty hand, the Five-seveN on the thigh, no can, no lit lens — whatever the kit says. Holding a man with the pistol drawn (the gun variant) it stays in the hand — up beside his head, laid along the hold layer's gun hand like any carry — and its rail light with it if L had it on (updateHold poses the light off the same hand)
  // the can in the hand: during a throw it is the one being thrown until the release frame and nothing for the rest of the swing (whatever the
  // count says — the count drops at release, which used to pop the next can into the follow-through and empty the hand while winding up the last one)
  const kindSel: 'smoke' | 'flash' = pl.slot === 3 ? 'flash' : 'smoke'; const an = pl.char.anim;
  const held: 'smoke' | 'flash' | null = pl.down || busy || pl.slot === 1 || pl.holstered || pl.dragging || pl.sprinting ? null
    : an.inThrow ? (an.throwing ? (pl.pendingThrow?.kind === 'flash' ? 'flash' : pl.pendingThrow ? 'smoke' : kindSel) : null)
    : g.throwCount(kindSel) > 0 ? kindSel : null;
  rigBoxes(pl.char, { goggles: true, nv: pl.nv, slot: pl.down ? 2 : busy || pl.holstered ? 0 : pl.slot, lightOn: pl.pistol.lightOn && !pl.down && !pl.sprinting && !busy && !pl.holstered, isGuard: false, holster: !pl.down || pl.slot !== 1 || pl.holstered, held }, out);   // holstered (E): empty hands, the pistol drawn on the thigh   // canister up: gun on the thigh, can in the fist; down: whatever was in the hand has left it (dropFromHand) — a holstered gun stays holstered, the goggles stay on   // (the light itself is forced off while down / sprinting: no blazing lens then)   // (a downed player's light is forced off in updatePlayer: no lit lens on the body either)
  for (const gd of g.guards) {
    if (gd.state === 'dead') { if (gd.dropped?.kind === 'torch') holsterBoxes(gd.char, out, { slide: RIG.guardPistol.slide, grip: RIG.guardPistol.grip }); continue; }   // what he held is on the floor; a man who died on patrol still wears his sidearm
    const fl = gd.flashlight;   // torch carried in the hand on a calm patrol (the spot light sits at its lens), drawn pistol with a weapon light otherwise (an alarmed floor patrols drawn: Guard.drawn)
    rigBoxes(gd.char, { goggles: false, nv: false, slot: 1, lightOn: fl.enabled, isGuard: true, torch: gd.state === 'patrol' && !gd.drawn, beamDir: gd.beamDir, lensColor: fl.color }, out);
    if ((gd.state === 'alert' || gd.state === 'search') && !gd.muzzleDown && !gd.held) laserBoxes(g, gd, out);   // drawn and hunting: the sidearm's laser is on — thumb off the switch while the gun is down for a colleague in the line (guards.ts muzzleCheck): nobody paints the back of the man ahead of him; pinned in Sam's arm the hand is off it altogether
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
  { const tp = g.player.char.rayHit(from, dir, L); if (tp >= 0) L = Math.min(L, tp); }   // (a live Sam stops it; bodies on the floor do not — rayHit ignores the dead; a muzzle inside a body — the man stacked ahead — draws through, not blank)
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
    if (g.aiEnabled) gd.witness.dazzled++;   // (his memory of it; the bang itself reaches his ears through hearingCheck like everyone's — nothing is remembered with the AI off)
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
