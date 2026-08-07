// Showcase / attract mode: a hands-off tour that flies the camera between set pieces and fires scripted
// events (doors, blackout) while the guards keep patrolling, drives the real player through four beats (takedown + drag,
// firefight, canisters, the mission's drive pull) via Game.puppet, and hands two guards a room-clearing drill (squad.ts RoomClear)
// via Guard.script. Toggle with P or start with ?demo.
import { Vec3, v3, DEG, damp, wrapAngle, smoothstep } from '../math/vec';
import type { Game, Mission, LightTarget } from './game';
import type { FollowCamera } from './camera';
import type { SmokeSystemLike } from '../smoke/types';
import type { Input } from './input';
import { sendRackCheck } from './mission';
import { RoomClear, endGuardScript } from './squad';
import type { RoomClearPlan } from './squad';
import { cancelActions } from './player';

/** door 'who' id for scripted use: ≥ 10 reads as a guard to the hearing model, so the tour's own door noises don't alarm anyone */
const DEMO_HAND = 10;

/** The room-clearing stop's plan: after it went loud, the rest of the shift come looking — and clear the conference room the way they were taught.
 *  Two guards are handed the drill in squad.ts (RoomClear) through Guard.script: stack on the door's east jamb, the #2's squeeze, the #1's kick (the
 *  real leaf, unlatched and flung off the boot: it bangs off its stop, rebounds, and the #2 shoulders it flat going in), split to their points of
 *  domination with muzzles sweeping corner to corner, one pushes up the east wall while the other kneels and puts his light under the table, then
 *  they call it, radio it in, drop to low ready and rally on the door. Every mark below was walked headlessly against the level (statics, nav cells,
 *  settled chair footprints, the door model): the table-and-chairs block leaves a clean lane only along the south glass and up the east side — the
 *  kicked leaf seals the south-west strip — so the pair dominate from the south-east corner and the door side, and clear the west half by light.
 *  Exported so a headless soak can drive RoomClear with the very same numbers. */
export const CONFERENCE_CLEAR: RoomClearPlan = {
  door: 'conference',
  from: [[12.1, 0, 10.75], [12.85, 0, 10.98]],                                        // coming west along the corridor's north wall, already walking when the veil lifts
  stack: [[10.55, 0, 10.5], [11.4, 0, 10.62]], stackFace: [-1.92, -1.9], stackSide: 1,   // #1 on the east jamb bladed to the wall (wall and door on his right), #2 tucked in 0.85 m behind
  stackAim: [[9.65, 1.05, 10.02], [9.7, 1.95, 10.0]],                               // #1's muzzle on the leaf by the lever, #2's high over his shoulder at the lintel
  kickFrom: [10.0, 0, 10.76], kickFace: Math.PI, fling: -10,                        // square to the leaf by the lock, 0.76 m off it (Doors.kickSpot); −10 rad/s throws it north into the room: it bangs off the stop ¼ s later and settles ≈ 70° open
  entry: [
    [[9.8, 0, 9.55], [12.45, 0, 9.15]],                                             // #1: through the east half of the doorway, hook right along the glass to the south-east corner
    [[9.75, 0, 10.75], [9.7, 0, 9.6], [9.3, 0, 8.95]],                              // #2: square through the doorway a beat later, hard left onto the door side — walking the leaf flat — holding just clear of the funnel
  ],
  podFace: [-2.36, -2.6],                                                            // #1 quartering north-west across the table, #2 west-north-west onto his half
  entryAim: [
    [[13.5, 0.8, 9.5], [13.5, 1.0, 4.6], [9.0, 1.1, 4.4]],                          // #1: the near (south-east) corner → up the TV wall to the north-east corner → the whiteboard
    [[9.6, 1.1, 9.9], [4.5, 0.8, 9.5], [4.7, 0.9, 4.9]],                            // #2: the doorway as he takes it → the dead corner behind the door → down the west wall to the plant
  ],
  searchPath: [[12.75, 0, 7.25], [12.75, 0, 5.25]], searchFace: -1.25,              // #1 up the east side past the TV to the north-east corner (nav cell centres, so the path runs exactly there), then facing back down the table
  searchAimWalk: [[13.4, 0.5, 4.6], [13.4, 0.5, 4.6], [10.8, 0.35, 6.9]],          // the corner low ahead of him (held while he walks), then the table's east end as he turns in
  searchAimThere: [[7.2, 0.35, 7.1], [4.6, 1.0, 4.6], [8.0, 1.3, 4.3]],            // skimming the table to its west end, the north-west corner, back along the whiteboard
  underAim: [[9.2, 0.32, 7.4], [7.0, 0.32, 7.0], [10.6, 0.32, 7.2]], upAim: [4.4, 1.1, 7.5],   // #2 on a knee: low at the table's middle → west end → east end; then back up onto the west wall
  formUp: [[[12.75, 0, 8.75], [10.75, 0, 9.25]], [[9.8, 0, 9.4]]], formFace: [-0.99, -0.12],   // #1 back down the east side and along the glass to the door's shoulder, facing it; #2 steps into the doorway mouth facing out
  lines: {
    stack1: 'stack up — conference room.', stack2: 'on you.', go2: 'set… go!', in1: 'going in!',
    side1: 'right side clear.', side2: 'door side clear.',
    push1: 'pushing up the right — get your light under that table.', under2: 'checking under…', nothing2: 'nothing under there.', corner1: 'far corner clear.',
    empty2: "room's clear — he's not in here.", radio1: 'negative contact, conference room. moving on to the next one.',
  },
};

interface Shot {
  dur: number;
  pos: Vec3 | (() => Vec3);              // camera look-at (floor point) — or a live getter (follow a guard)
  yaw: number; dist: number | (() => number);   // degrees / metres (dist may be live too: a beat that pulls out on its own cue)
  drift?: number;                         // deg/s of slow yaw pan across the shot (default 0.7): a static shot never quite sits still
  yawTo?: number | (() => number);       // if set: a scripted rotation (what Q / E do by hand) eased from `yaw` to `yawTo` across the shot — shows the far walls exist
  swing?: [number, number];               // fraction of the shot where that rotation happens (default hold–swing–hold: 0.12 → 0.88)
  distFrom?: number;                      // if set: the shot opens at this distance (snapped under the fade) and pulls out to `dist` — a reveal from the subject to the situation
  caption: string; sub?: string;
  enter?: () => void; exit?: () => void;
  at?: { t: number; run: () => void; done?: boolean }[];   // timed events relative to shot start
  tick?: (t: number, dt: number) => void;                    // per-frame script (puppet goals / aim, edge-triggered actions)
}

export class Demo {
  active = false; t = 0; shotI = -1; shotT = 0;
  caption = ''; sub = '';
  /** 0..1: black overlay the page draws over the view — raised for a beat around every cut so teleports / resets never show */
  fade = 0;
  /** the room-clearing stop's drill while that stop runs (soaks read its phase and phase clocks through __demo.drill), else null */
  drill: RoomClear | null = null;
  private handedOver = false;   // the last stop already gave the viewer a fresh, live encounter: stopping there is a normal finish, not an abort
  private shots: Shot[] = [];
  private saved: { yaw: number; dist: number; ai: boolean; god: boolean; invisible: boolean; pos: Vec3; slot: 1 | 2 | 3; light: boolean } | null = null;

  constructor(private game: Game, private cam: FollowCamera, private smoke: SmokeSystemLike & { spawnCanister?: (p: Vec3) => void; clearAll?: () => void }, private input: Input) {}

  // ---- puppeteering helpers for the scripted player beats (the player runs through its normal update: we only move the goal, the aim point and press keys)
  private key(code: string) { this.input.pressed.add(code); }
  private click() { this.input.clicked |= 1; }
  private release() { const g = this.game; g.puppet = null; g.missionInTour = false; this.input.keys.delete('ShiftLeft'); if (g.player.dragging) g.toggleDrag(); g.player.crouch = false; g.player.sprinting = false; g.player.char.anim.stance = 'none'; for (const gd of g.guards) { gd.hold = false; gd.pinned = false; if (gd.script) endGuardScript(gd); } }   // (a scripted man is handed back to the AI too, and a ready stance a beat put on Sam comes off: no beat's choreography outlives its stop)
  /** the beats spend real kit: top the player up so a third run of the tour still has rounds and canisters */
  private rearm() { const p = this.game.player; p.pistol.cancelReload(); p.pistol.spare = [10, 10, 10]; if ((p.pistol.mag ?? 0) < 4) p.pistol.mag = 10; p.pistol.chamber = 1; p.canisters = Math.max(p.canisters, 2); p.flashbangs = Math.max(p.flashbangs, 1); }   // (chamber too: topping the magazine up by hand does not rack a round, and an empty chamber refuses to fire)

  private build(): Shot[] {
    const g = this.game;
    let hallYaw = 60;   // corridor stop: camera yaw that looks down the hallway the way guard 0 is walking (set at enter, follows his heading gently)
    const guardPos = (i: number) => () => { const gd = g.guards[i % g.guards.length]; return gd ? v3.copy(gd.char.pos) : [20, 0, 11] as Vec3; };
    const door = (n: string) => g.doors.byName(n);
    return [
      { dur: 9, pos: [7.6, 0, 17.6], yaw: -22, dist: 19, caption: 'Everything here is boxes with flat diffuse paint.', sub: 'All lighting is traced live: every light is a sized emitter sampled with per-pixel shadow rays, and bounce comes from rays traced off the exact surfaces you see (a coarse probe volume only feeds the second bounce).',
        enter: () => { g.setGroup('lobby', true); } },
      { dur: 11, pos: guardPos(0), yaw: -30, yawTo: () => hallYaw, swing: [0.04, 0.5], dist: 12, caption: 'Guards carry real spot lights.', enter: () => { const f = g.guards[0]?.char.forward() ?? [1, 0, 0]; hallYaw = Math.atan2(-f[0], -f[2]) / DEG; }, tick: () => { const gd0 = g.guards[0]; if (gd0 && gd0.char.vel && Math.hypot(gd0.char.vel[0], gd0.char.vel[2]) > 0.3) { const want = Math.atan2(-gd0.char.vel[0], -gd0.char.vel[2]) / DEG; hallYaw += wrapAngle((want - hallYaw) * DEG) / DEG * 0.02; } }, sub: 'Beams are ray-marched through the same box scene, so they shadow and scatter correctly; the torch and pistol in their hands are real geometry too — and the light meter the AI reads (bottom-left) is the same GPU query.' },
      { dur: 10, pos: [34.4, 0, 20.3], yaw: -32, yawTo: -12, dist: 15, caption: 'Doors are dynamic occluders.', sub: 'The fire exit opens onto the sodium-lit lot: watch the wedge sweep across the break-room floor — bounce included — then pinch down to slivers through the cracks as it closes.',
        enter: () => { g.setGroup('breakroom', false); const d = door('fire_exit'); if (d) { d.angle = 0; d.vel = 0; d.latched = true; } },
        at: [{ t: 1.2, run: () => { const d = door('fire_exit'); if (d) g.doors.use(d, [35.4, 0, 20.6], DEMO_HAND); } }, { t: 6.2, run: () => { const d = door('fire_exit'); if (d && Math.abs(d.angle) > 0.3) g.doors.use(d, [35.4, 0, 20.6], DEMO_HAND); } }] },
      // --- scripted player beats: the tour drives the real player through Game.puppet (goal / aim) and synthetic key presses ---
      (() => { let struck = false, grabbed = false, dropped = false; const gd = () => g.guards[1 % g.guards.length]; return {
        dur: 16, pos: () => v3.copy(g.player.char.pos), yaw: -28, dist: 10.5, distFrom: 5.5, caption: 'Close, quiet work: a takedown from behind, then haul the body out of the light.',
        sub: 'One F key does whatever the marker under the cursor offers — here the guard, then his body. He goes down as a ragdoll (particles and constraints colliding with the same box world), and dragging pins his ankles to your hands while the rest of him trails and bumps round corners.',
        enter: () => { struck = grabbed = dropped = false; g.playerInvisible = true; this.smoke.clearAll?.(); g.setGroup('cubicles', true); this.rearm(); g.clearAftermath(); if (gd()?.state === 'dead') g.resetGuards(); const t = gd(); if (t) { t.char.pos = [26.4, 0, 18.6]; t.char.bodyYaw = Math.PI / 2; t.char.aimYaw = Math.PI / 2; t.hold = true; t.state = 'patrol'; t.awareness = 0; }
          g.guards.forEach(o => { if (o !== t) o.hold = true; });
          g.teleportPlayer([22.6, 0, 18.6]); g.player.slot = 1; g.player.pistol.lightOn = false; g.puppet = { goal: [25.7, 0, 18.6], aim: [27.5, 1.2, 18.6], crouch: true }; },
        tick: (t) => { const tg = gd(); if (!tg || !g.puppet) return; const alive = tg.state !== 'dead';
          if (alive) { g.puppet.aim = v3.add(tg.char.pos, [0, 1.35, 0]); g.puppet.goal = [tg.char.pos[0] - 0.7, 0, tg.char.pos[2]]; if (!struck && t > 1.5 && g.hover?.kind === 'guard' && g.hover.inReach) { this.key('KeyF'); struck = true; } }
          else if (!grabbed) { g.puppet.goal = null; const hips = tg.char.bones.hips ?? tg.char.pos; g.puppet.aim = [hips[0], hips[1] + 0.12, hips[2]]; if (t > 5.2 && g.hover?.kind === 'body' && g.hover.inReach) { this.key('KeyF'); grabbed = true; } else if (t > 7 && !g.player.dragging) { g.toggleDrag(tg); grabbed = true; } }   // (aim = the body marker's own point; direct grab as the fallback)
          else if (!dropped) { g.puppet.aim = null; g.puppet.goal = [22.2, 0, 19.0]; if (t > 13.5 || !g.player.dragging) { if (g.player.dragging) g.toggleDrag(); dropped = true; g.puppet.goal = null; } } },
        exit: () => this.release() }; })(),
      (() => { let phase = 0, nextShot = 0, shots = 0, reloaded = false, burst = 0, wp = 0; let burstAt: typeof g.guards[0] | null = null;
        // hull-down firefight: the cubicle end caps are 1.4 m — chests covered, heads over the top — so both sides SEE each other and every round meets a
        // partition (sparks off the cover, nobody hit). Sam fires, ducks, shifts, lobs a stun charge over the top, and takes the dazzled pair from the flank.
        // His gun-handling reads through the animator's ready stances (character.ts Stance; the game never sets one on the player, so the beat owns it and
        // release() clears it): a compressed high ready for the half second before a pair breaks and over the last strides of the flank, a low ready — muzzle
        // to the floor, both hands on the gun — while he is down behind the partition or shifting along it, and NO stance from the instant he fires (the aim
        // layer's own pose is what the rounds leave from), while the can is in his fist (pistol on the thigh, the throw clip has the arms) or at a dead sprint.
        // (The pair keep the AI's own bearing: updateGuard sets a live guard's stance and upper mode itself every frame before it poses him, so a blinded
        // man's hunch would have to come from guards.ts, not from a write here.)
        const S1: Vec3 = [20.15, 0, 15.5], WEST: Vec3 = [19.4, 0, 15.5], BANG_AT: Vec3 = [24.9, 0.05, 17.9];   // (in the open aisle mouth west of the end-cap line: line of sight to both posts; any nearer and the can meets the caps)
        const ROUTE: Vec3[] = [[19.25, 0, 18.9], [19.5, 0, 23.1], [25.9, 0, 23.05]];   // the flank: south down the west cross-aisle, east along the south wall behind the cubicle block, to a spot due south of both posts — they line up, backs and flanks to him
        const READY_IN = 3.0;   // metres short of that spot at which the gun comes up out of the sprint into the high ready
        const posts: Vec3[] = [[25.9, 0, 15.5], [25.9, 0, 20.4], [27.3, 0, 17.9]];   // behind the NE end cap, behind the SE end cap, deep in the aisle mouth
        let posted: typeof g.guards = [];   // the pair manning the posts this run (a parked third man is out of the beat: neither a target nor part of the dazzle check)
        const living = () => posted.filter(x => x.state !== 'dead' && g.guards.includes(x));
        const target = () => { const me = g.player.char.pos; return living().sort((a, b) => v3.dist(a.char.pos, me) - v3.dist(b.char.pos, me))[0]; };
        const nearestBody = () => g.guards.filter(x => x.state === 'dead').sort((a, b) => v3.dist(a.char.pos, g.player.char.pos) - v3.dist(b.char.pos, g.player.char.pos))[0];
        return {
        dur: 19, pos: [23.0, 0, 19.4], yaw: -24, yawTo: -4, dist: 18, distFrom: 10, caption: 'When it goes loud: hull-down behind the partitions — every round meets somebody\'s cover.',
        sub: 'Their lasers and weapon lights find his cover the moment his head shows; muzzle flashes are real lights for a frame or two, sparks are traced streaks off whatever the round hit, and one hit drops anyone, Sam included — so he fires, ducks, shifts along the cover, lobs a stun charge over the top, sprints round the cubicle block while they are blind and takes the pair from the south, lined up with their flanks to him. The empty magazine that hits the floor afterwards is physical too.',
        enter: () => { phase = 0; nextShot = 0.9; shots = 0; reloaded = false; burst = 0; wp = 0; burstAt = null; g.playerInvisible = false; g.godMode = true; g.setGroup('cubicles', true); this.rearm(); g.player.pistol.mag = 9; g.player.pistol.chamber = 1;
          g.teleportPlayer(S1); g.player.slot = 1; g.player.pistol.lightOn = false; g.player.crouch = false; g.player.char.bodyYaw = Math.PI / 2; g.player.char.anim.stance = 'highReady';   // (opens on him already up behind the cap in the compressed high ready the first pair breaks from)
          let k = 0; posted = []; g.guards.forEach(gd2 => { if (gd2.state === 'dead') return; const i = k++; if (i >= 2) { gd2.hold = true; gd2.pinned = true; gd2.char.pos = [12.8, 0, 5.2]; gd2.state = 'patrol'; gd2.awareness = 0; return; }   // (takedown skipped: a third man waits this one out in the conference room, held AND pinned so the gunfire does not bring him running — the beat is timed for the pair)
            posted.push(gd2);
            gd2.hold = false; gd2.pinned = true; gd2.char.pos = v3.copy(posts[i % posts.length]); gd2.char.bodyYaw = -Math.PI / 2; gd2.char.aimYaw = -Math.PI / 2; gd2.state = 'alert'; gd2.awareness = 1; gd2.lastKnown = v3.copy(S1); gd2.lastSeenT = g.time - 2; gd2.reactT = 0.8 + i * 0.3; gd2.dazzledUntil = -1; });
          g.puppet = { goal: null, aim: v3.add(posts[0], [0, 1.25, 0]) }; },
        tick: (t) => { if (!g.puppet) return; const pl = g.player, gun = pl.pistol, an = pl.char.anim; const me = pl.char.pos;
          // they see him exactly when his HEAD is over the partition line (standing); crouched he is gone and they cover his last position
          const head: Vec3 = [me[0], pl.crouch ? 1.1 : 1.62, me[2]];
          for (const o of living()) { const eye: Vec3 = [o.char.pos[0], 1.62, o.char.pos[2]]; if (!pl.crouch && !g.col.segmentBlocked(eye, head)) { o.lastSeenT = g.time; o.lastKnown = v3.copy(me); } }
          const tg = target();
          g.puppet.target = tg ?? null;   // scripted rounds are FOR this man (the cursor still tracks him for the reticle)
          if (phase === 0) {   // stand and trade: two rounds into the far end cap, theirs come back into his
            if (tg) g.puppet.aim = v3.add(tg.char.pos, [0, 1.25, 0]);
            if (t >= nextShot && shots < 2) { this.click(); shots++; nextShot = t + 0.24; an.stance = 'none'; }   // a double tap into the far end cap — off the ready the instant he fires: the rounds leave from the aim layer's own pose
            if (t > 1.45) { g.puppet.crouch = true; phase = 1; an.stance = 'lowReady'; pl.lastFireT = -10; }   // down before their first rounds arrive (their reaction beat is ≥ 0.8 s after they see him at 0.9 s; a stray high round could otherwise find the head over the cap), and the gun comes down with him into the low ready — letting go of the 'just fired' hold (player.ts: the aim stays up 1.6 s after a round) that would keep the aim clip over the carry for the whole shift
          } else if (phase === 1) {   // down and out of it: shift a metre back along the cover
            g.puppet.goal = WEST; if (t > 2.9) { this.key('Digit3'); an.stance = 'none'; g.puppet.aim = BANG_AT; g.puppet.throwAt = BANG_AT; phase = 2; }   // (the can comes into the fist and the pistol goes to the thigh: nothing to carry — and the throw clip wants the arms)
          } else if (phase === 2) {   // up just long enough to lob the stun charge over the top (a metre back so the arc clears his own cap)
            g.puppet.goal = null; if (t > 3.3) g.puppet.crouch = false;
            if (t > 3.7) { this.click(); phase = 3; nextShot = t + 0.5; }
          } else if (phase === 3) {   // back down until it goes off — the can away, straight back onto the pistol, waiting at the low ready while the fuse burns
            if (t > nextShot) { g.puppet.crouch = true; if (pl.slot === 1) an.stance = 'lowReady'; else if (!pl.pendingThrow && !an.throwing) this.key('Digit1'); }   // (drawn only once the can has left the hand and the throw has played; the carry from the frame the gun is back in it)
            if (living().length && living().every(o => g.time < o.dazzledUntil)) { phase = 4; g.puppet.throwAt = null; if (pl.slot !== 1) this.key('Digit1'); an.stance = 'none'; for (const o of living()) { o.dazzledUntil = Math.max(o.dazzledUntil, g.time + 6.0); o.reactT = Math.max(o.reactT, 6.0); } }   // (scripted: blind and rooted for the length of the flank — the round-the-block route is longer than the falloff at their distance allows for)
            else if (t > 8.5) { phase = 4; g.puppet.throwAt = null; if (pl.slot !== 1) this.key('Digit1'); an.stance = 'none'; }   // (dud / nobody had line of sight: press on regardless — god mode is the net)
          } else if (phase === 4) {   // they are blind and rooted: up and sprinting — south round the cubicle block and along the south wall (they cannot hear it over the ringing either)
            g.puppet.crouch = false; g.puppet.walk = 1; this.input.keys.add('ShiftLeft'); pl.pistol.lightOn = wp >= ROUTE.length - 1;
            const last = ROUTE.length - 1; const dEnd = Math.hypot(me[0] - ROUTE[wp][0], me[2] - ROUTE[wp][2]); const there = dEnd < 0.45;
            if (there && wp < last) wp++;                       // next corner
            else if (there && wp === last) { g.puppet.goal = null; this.input.keys.delete('ShiftLeft'); phase = 5; nextShot = t + 0.25; }   // in position, due south of both (only from here are the lines to both posts clear of the end caps) — still in the high ready: the aim layer punches the gun out of it as he plants
            if (phase === 4) { g.puppet.goal = ROUTE[wp]; g.puppet.aim = wp === last && tg ? v3.add(tg.char.pos, [0, 1.2, 0]) : v3.add(ROUTE[wp], [0, 1.2, 0]);   // last leg: eyes (and the rail light) already on the nearer man
              an.stance = Math.hypot(me[0] - ROUTE[last][0], me[2] - ROUTE[last][2]) < READY_IN ? 'highReady' : 'none'; }   // gun down at the dead sprint; over the last strides it comes up compressed under the eye line, both hands on it
          } else if (phase === 5) {   // the drop: a double tap to the centre of mass of each, nearest first (still closing to the spot due south of both, then planted)
            g.puppet.crouch = false;
            if (burstAt && !g.guards.includes(burstAt)) { burstAt = null; burst = 0; }          // (guards were reset under us: never fire at a removed man's post)
            if (gun.roundsReady === 0 && !gun.reloading) { if (gun.spare.length) { this.key('KeyR'); nextShot = t + 2.1; } else phase = 6; }   // dry with a man still up: reload (or give it up), never stand there clicking
            else { if (burst === 0) { if (!tg) { phase = 6; } else if (t >= nextShot) { burstAt = tg; burst = 2; } }
              if (burst > 0 && burstAt) { g.puppet.target = burstAt; g.puppet.aim = v3.add(burstAt.char.pos, [0, 1.2, 0]);
                if (t >= nextShot && !gun.reloading) { this.click(); an.stance = 'none'; burst--; nextShot = t + (burst > 0 ? 0.24 : 0.45); } } }   // (off the ready as the first round breaks; the gun stays up on the aim layer between the two men)
          } else {   // aftermath: dump the magazine (the empty drops and skitters), walk the light over to the nearest body
            g.puppet.walk = 0.6; an.stance = 'none';   // (a drop that ended dry or with nobody to shoot must not leave the ready up either)
            if (!reloaded && !gun.reloading) { gun.mag = 0; this.key('KeyR'); reloaded = true; }
            const b = nearestBody(); if (b) { const hp = b.char.bones.hips ?? b.char.pos; g.puppet.aim = [hp[0], 0.25, hp[2]]; const dx = hp[0] - me[0], dz = hp[2] - me[2], dl = Math.hypot(dx, dz); g.puppet.goal = dl > 1.5 ? [hp[0] - dx / dl * 1.3, 0, hp[2] - dz / dl * 1.3] : null; }
          } },
        exit: () => { this.release(); g.playerInvisible = true; g.player.pistol.lightOn = false; } }; })(),
      (() => { let clear: RoomClear | null = null; let lit: { t: LightTarget; on: boolean }[] = []; let tIn = Infinity, tOpen = Infinity;   // shot times the point man went through the door (the camera follows him in from there) and the leaf went (the camera pulls out on it)
        // they sweep for him: two of the fresh shift clear the conference room as a pair (CONFERENCE_CLEAR above, run by squad.ts RoomClear) — the door pulled to
        // first so there is something to breach, the room's own lights off so their weapon lights do the work, the third man parked, Sam crouched in the lobby arch
        const SAM: Vec3 = [6.3, 0, 12.8];                                          // the lobby side of the arch, in the pier's shadow, 4 m from the door they are about to take
        const DOORPT: Vec3 = [9.7, 0, 9.75], ROOM: Vec3 = [9.9, 0, 7.7];           // camera: on the doorway for the stack and the kick, then the middle of the room
        return {
        dur: 18, yaw: -22, yawTo: -36, swing: [0.42, 0.9], distFrom: 7.2, dist: () => this.shotT > tOpen + 0.35 ? 13.5 : 8.2,   // tight on the stack; out to the whole room as the leaf goes
        pos: () => v3.lerp(DOORPT, ROOM, smoothstep(0, 1, (this.shotT - tIn + 0.2) / 3.2)),   // follows them in: leaves the doorway as the point man does
        caption: 'When they come looking they clear a room the way they were taught — stack, breach, split the corners, and talk the whole time.',
        sub: 'Two patrol AIs handed a drill instead of a route. The kick, the ready stances, the hand signals and the muzzle sweeps are procedural layers on the same mannequin Sam wears; the door is the physical leaf everyone uses — flung off the boot, banged off its stop, shouldered flat by the second man; with the room\'s own lights off, their weapon lights and lasers are most of what you see by; and every bark is a man saying out loud what he is about to do. Sam is behind them the whole time, crouched in the lobby arch.',
        enter: () => { g.clearAftermath(); g.resetGuards(); this.rearm(); g.playerInvisible = true; g.godMode = true; this.smoke.clearAll?.();   // (aftermath first: the fresh shift must not hear the firefight's last rounds)
          lit = g.targets.filter(t => t.group === 'conference').map(t => ({ t, on: t.on })); g.setGroup('conference', false);   // TV and panels off (put back as they were at exit): the room is theirs to light
          g.teleportPlayer(SAM); g.player.slot = 1; g.player.pistol.lightOn = false; g.player.crouch = true; g.player.char.bodyYaw = 2.28;   // facing the door across the corridor
          g.puppet = { goal: null, aim: [8.4, 0.5, 11.2], crouch: true };           // eyes low on the corridor floor short of the door (well off every marker)
          const [a, b] = g.guards; g.guards.forEach((o, i) => { if (i >= 2) o.hold = true; });   // the third man waits this one out at his route start in the break room, three walls away
          tIn = tOpen = Infinity; clear = this.drill = a && b ? new RoomClear(g, CONFERENCE_CLEAR, a, b) : null;
          const d = clear?.door; if (d) { d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; }   // pulled to and latched (it is authored standing open, no closer): something to breach
          clear?.start(); },
        tick: (t, dt) => { if (!clear) return; clear.tick(t, dt); if (isFinite(clear.tEnter)) tIn = clear.tEnter; if (isFinite(clear.tKick)) tOpen = clear.tKick; },
        exit: () => { clear?.end('patrol'); clear?.door?.reset(); clear = this.drill = null; for (const l of lit) l.t.on = l.on; lit = []; this.release(); g.clearAftermath(); g.resetGuards(); } };   // leaf back at its authored angle, the room's lights as they were, calm patrols on their routes for whatever follows
        })(),
      (() => { let phase = 0; const HIDE: Vec3 = [26.6, 0, 19.7], LAND: Vec3 = [27.6, 0.05, 18.4], ACROSS: Vec3 = [26.3, 0, 14.9];   // LAND is a metre short of the threshold: the can lands this side and rolls on through the opening, coming to rest about on the sill
        // start SOUTH of the doorway (the leaf hangs off the north jamb and swings this side, so from the south the opening is clear), roll the can onto the threshold so the plume fills the opening itself, then cross northward
        // the smoke canister on its own: two guards inside the break room watch the open doorway; Sam, tucked against the wall this side of it, puts a
        // canister into the doorway and walks across the opening behind the plume — the guards' sight test really is marched through the density field
        return {
        dur: 15, pos: () => { const u = Math.min(1, Math.max(0, (this.shotT - 0.9) / 2.4)); return v3.lerp(HIDE, [29.0, 0, 17.8], u * u * (3 - 2 * u)); }, yaw: -34, yawTo: 4, dist: 15, distFrom: 7, caption: 'The smoke canister: a GPU fluid sim, lit and shadowed by every light — and the guards genuinely cannot see through it.',
        sub: 'Two of them are watching this doorway from the break room. The plume vents for half a minute into a ceiling-high domain (about three of the old cans in one); their line-of-sight test is marched through the same density field the renderer draws, so once the doorway fills he simply walks across the opening, two metres from men looking straight at it.',
        enter: () => { phase = 0; g.clearAftermath(); g.resetGuards(); this.rearm(); g.playerInvisible = false; g.godMode = true; this.smoke.clearAll?.(); g.setGroup('breakroom', true); g.setGroup('cubicles', true);   // (aftermath first: fresh guards must not hear the firefight's last shots)
          const dw = door('breakroom_w'); if (dw) { dw.angle = 1.35; dw.vel = 0; dw.latched = false; dw.closing = false; }
          g.teleportPlayer(HIDE); g.player.slot = 2; g.player.crouch = true; g.player.char.bodyYaw = Math.PI * 0.8;
          const spots: [Vec3, number][] = [[[31.6, 0, 16.8], -Math.PI / 2], [[33.2, 0, 18.9], -Math.PI / 2], [[12.8, 0, 5.2], 0]];   // (clear of the break-room table / chairs and the conference table)   // two watching the doorway from inside, the third parked far away
          g.guards.forEach((o, i) => { o.hold = true; o.char.pos = v3.copy(spots[i % 3][0]); o.char.bodyYaw = spots[i % 3][1]; o.char.aimYaw = spots[i % 3][1]; });
          g.puppet = { goal: null, aim: LAND, crouch: true, throwAt: LAND }; },
        tick: (t) => { if (!g.puppet) return;
          if (phase === 0 && t > 0.7) { this.key('Digit2'); phase = 1; }
          else if (phase === 1 && t > 1.6) { this.click(); phase = 2; }                                             // canister into the doorway
          else if (phase === 2 && t > 7.2) { g.puppet.aim = null; g.puppet.throwAt = null; g.puppet.goal = ACROSS; g.puppet.walk = 1.0; phase = 3; }   // the doorway is full: across the opening, still crouched
          else if (phase === 3 && Math.hypot(g.player.char.pos[0] - ACROSS[0], g.player.char.pos[2] - ACROSS[2]) < 0.3) { g.puppet.goal = null; g.puppet.crouch = false; this.key('Digit1'); phase = 4; } },
        exit: () => { this.release(); g.player.slot = 1; g.playerInvisible = true; g.clearAftermath(); g.resetGuards(); this.smoke.clearAll?.(); } }; })(),   // hand the beats after it calm patrols with torches again (and a Sam they cannot see), and clear air
      (() => { let tF = -1, tOut = -1, sent = false; let saved: Mission | null = null;
        // the mission's own centrepiece, played rather than staged: Sam creeps along the rack fronts to the marked rack, F on its marker starts the very pull a
        // player makes (updateMission runs under the puppet while missionInTour is up), the rack drops off its feed when it completes — its LEDs and the green
        // wash they threw on the floor are real lights that simply go — and the corridor man is sent to look by the mission's own errand (sendRackCheck:
        // the game fires it 7 s after the pull, the tour calls the same function a second after it, purely for pacing), shouldering the real door open so
        // the corridor's light wedges in with him while Sam backs into the dark end of the room, behind the sweep of the torch
        const M = g.level.mission, n = M.rack.index + 1, RX = M.rack.front[0], RZ = M.rack.front[2];   // the LED face of the marked rack (east bank; every rack front sits on this z)
        const PULL: Vec3 = [RX, 0, RZ + 0.67], SPAWN: Vec3 = [17.3, 0, RZ + 0.67], HIDE: Vec3 = [16.4, 0, RZ + 0.77];   // planted 0.67 m off the face: inside DRIVE_REACH (1.1) with the 0.42 m body clear of the cabinet; spawn / hide on the same line in front of the west bank — the strip along the rack fronts is the part of the floor a south camera sees over the corridor wall
        const POST: Vec3 = [19.3, 0, 10.95], ROOM: Vec3 = [18.5, 0, 8.4];   // the corridor man's idle spot, a step east of the server door; camera centre between that door and the marked rack
        return {
        dur: 16, pos: () => { const u = Math.min(1, Math.max(0, (this.shotT - 0.8) / 2.7)); return v3.lerp(g.player.char.pos, ROOM, u * u * (3 - 2 * u)); }, yaw: -38, yawTo: -68, swing: [0.5, 0.84], dist: 11, distFrom: 6.5,   // opens tight on Sam from the SW (far enough round that the corridor wall's top edge sits below the rack-front strip he uses, LED faces still three-quarters on), pulls out to the room; the swing waits for the pull, then comes round to the WSW while the guard walks in: the doorway's wedge of light, his beam side-on across the racks, Sam's dark end of the room clear of the west wall's edge
        caption: `The job itself: hands on rack ${n} until its LEDs die — it really drops off its feed, and somebody is sent to look.`,
        sub: 'None of it is staged for the camera. F on the rack\'s marker starts the same timed pull a player makes; when it completes, the LEDs and the green wash they threw on the floor are just lights the renderer no longer has, the clunk goes into the hearing model, and the mission code radios the nearest calm man to check the rack — he shoulders a real door, so the corridor\'s light comes in with him and his torch does the rest. Sam is already backing into the dark end of the room, behind its sweep.',
        enter: () => { tF = tOut = -1; sent = false; g.clearAftermath(); g.resetGuards(); this.rearm(); g.playerInvisible = true; g.godMode = true;
          g.setGroup('server', false);   // the room's own cold panel off: lit by the racks' green wash and whatever the door lets in — so one rack dying and one torch arriving both read
          saved = g.mission; g.mission = { ...saved, stage: 'infiltrate', driveT: -1, pulled: false, sweepAt: -1, alarmOn: false, debrief: false, stats: { ...saved.stats } };   // a scratch copy for the beat: whatever it accrues (elapsed, the sandbox flag) is thrown away at exit with the player's own run put back; 'infiltrate' so the room announces its rack in the log the way it does for a player
          for (const t of g.rackTargets) t.on = true; g.missionInTour = true;   // (relit in case an earlier pull — the player's, or a previous lap of the tour — left it dark)
          const d = door('server'); if (d) { d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; }   // shut, as authored: his push swings it and the corridor light sweeps in across the floor
          g.teleportPlayer(SPAWN); g.player.slot = 1; g.player.pistol.lightOn = false; g.player.crouch = true; g.player.char.bodyYaw = Math.PI / 2;
          g.guards.forEach((o, i) => { o.hold = true; if (i === 0) { o.char.pos = v3.copy(POST); o.char.bodyYaw = -Math.PI / 2; o.char.aimYaw = -Math.PI / 2; } });   // the corridor man idles east of the door facing down the hallway; the other two hold their route starts, out of shot — held men are never candidates for the errand, so it is always him
          g.puppet = { goal: PULL, aim: v3.copy(M.rack.front), crouch: true }; },   // eyes (and so the cursor) on the LED face: the rack's marker sits exactly there, so it is the hovered one the moment the stage offers it
        tick: (t, dt) => { if (!g.puppet) return; const m = g.mission, me = g.player.char.pos; const cg = g.guards[0];
          if (!m.pulled) {
            const planted = Math.hypot(me[0] - PULL[0], me[2] - PULL[2]) < 0.3;
            if (m.driveT >= 0) { g.puppet.goal = null; const c = g.player.char; c.bodyYaw += wrapAngle(Math.PI - c.bodyYaw) * (1 - Math.exp(-4 * dt)); }   // pulling: stay planted and crouched, squared up to the face (he arrived crabbing east) — updateMission times it (3.5 s within reach) and drivePulled does the rest (LEDs off, clunk, stage → exfil)
            else if (planted && t > 1.2 && t >= tF + 0.5 && g.hover?.kind === 'objective' && g.hover.inReach) { this.key('KeyF'); tF = t; }   // F on the marker under the cursor — the player's own key path (pressed again if a press somehow did not take)
            else if (planted && t > 6 && m.stage === 'drive') m.driveT = 0;   // net: the marker never came under the cursor — start the pull directly so the beat still plays out
          } else {
            if (tOut < 0) tOut = t;   // (≈ 6.3 s in: 2.7 to creep over and plant, 3.5 on the rack)
            if (!sent && t > tOut + 1.0) { sent = true; m.sweepAt = -1; if (cg && cg.state !== 'dead') { cg.hold = false; cg.state = 'patrol'; cg.awareness = 0; cg.task = null; } sendRackCheck(g); }   // the game's own 7 s would land after the cut: cancel that timer and run the same errand now (hold off and calm first — sendRackCheck only picks a calm, free patrol). He places it for a beat, walks the corridor, pushes through the door ≈ 3 s later and is on the rack by ≈ 12 s, sweeping it till the cut
            if (t > tOut + 0.4) { const there = Math.hypot(me[0] - HIDE[0], me[2] - HIDE[2]) < 0.3; g.puppet.goal = there ? null : HIDE; g.puppet.walk = 1.1;   // back off west along the rack fronts (there by ≈ 9.7 s, before the door opens at his shoulder)…
              g.puppet.aim = cg && cg.state !== 'dead' ? v3.add(cg.char.pos, [0, 1.62, 0]) : [18.0, 1.62, 9.0]; }   // …backpedalling with his eyes on the man coming in (through the wall, then through the doorway) — at head height, which also keeps the cursor off the door's own marker as he passes it
          } },
        exit: () => { this.release(); g.setGroup('server', true); if (saved) { g.mission = saved; for (const t of g.rackTargets) t.on = !saved.pulled; saved = null; } g.clearAftermath(); g.resetGuards(); } }; })(),   // panel back on, the player's own mission back exactly as it was (rack lit or dark to match), and calm patrols on their routes for the blackout
      { dur: 13, pos: [19.5, 0, 12.4], yaw: -22, yawTo: -46, dist: 24, caption: 'OCP the breaker: the mains drop.', sub: 'Exit signs and the UPS-fed rack LEDs stay up (their green wash is a real light per rack), five ceiling beacons sweep red through the haze, and your light meter now pulses with the sweep.',
        at: [{ t: 1.0, run: () => g.setBlackout(9.5, 'debug') }] },
      { dur: 7, pos: [19.5, 0, 12.4], yaw: -22, dist: 24, caption: 'Fluorescents restrike one contactor group at a time.', sub: '' },
      { dur: 10, pos: [15.5, 0, 25.6], yaw: -35, yawTo: 20, dist: 18, caption: 'Sized lights give soft shadows.', sub: 'The sodium lamps are 18 cm emitters under their housings: stratified shadow rays per pixel — more only where they disagree, inside the penumbra — then a filter steered by the measured penumbra width; characters are traced as capsules.' },
      { dur: 8, pos: () => v3.copy(g.player.char.pos), yaw: -22, dist: 21, caption: 'Your turn — WASD and mouse; F works whatever marker you point at.', sub: 'Esc for settings and controls, Tab for every knob, P ends the tour.',
        enter: () => { g.restartEncounter(); this.smoke.clearAll?.(); g.playerInvisible = false; g.godMode = this.saved?.god ?? false; g.quietUtility = false; this.input.lockExcept = null; this.handedOver = true; } },   // live from here: the mission counts, no god mode unless the player had it on   // hand over a fresh encounter (guards, lights, props, noise memory, you back at the insertion point) with live controls — not the aftermath of the scripted beats
    ];
  }

  start() {
    if (this.active) return;
    const g = this.game;
    this.saved = { yaw: this.cam.yaw, dist: this.cam.desiredDistance, ai: g.aiEnabled, god: g.godMode, invisible: g.playerInvisible, pos: v3.copy(g.player.char.pos), slot: g.player.slot, light: g.player.pistol.lightOn };
    this.shots = this.build(); this.active = true; this.t = 0; this.shotI = -1; this.shotT = 0; this.handedOver = false;
    g.quietUtility = true;                                          // (before the reset below, so even that stays out of the log)
    g.clearAftermath();                                             // a shot fired half a second ago must not alert the guards the tour is about to reset
    for (const d of g.doors.list) d.reset();                        // a lock kicked in before the tour would pull stop 1's guard off his beat to look at it
    cancelActions(g);                                               // whatever the hands were doing (a kick wound up, a pick, a takedown, a drag, a throw) must not land under the tour
    this.input.lockExcept = new Set(['KeyP']); this.input.keys.clear(); this.input.buttons = 0;   // hands off: your mouse and keys do nothing during the tour except P to stop it (Esc still pauses)
    if (g.player.down) g.restartEncounter(); else g.resetGuards();
    g.aiEnabled = true; g.godMode = true; g.playerInvisible = true;   // walls between the camera and the scripted player drop while the tour runs
    this.fade = 1;                                                  // opens from black
    if (g.blackout.active) g.endBlackout();
    this.next();
    g.msg('showcase tour — P to stop');
  }

  stop() {
    if (!this.active) return;
    const cur = this.shots[this.shotI]; const finished = !cur || this.handedOver; cur?.exit?.(); this.release();
    this.active = false; this.caption = ''; this.sub = ''; this.fade = 0; this.game.quietUtility = false; this.input.lockExcept = null; this.input.keys.clear(); this.input.buttons = 0;
    if (this.saved) { const g = this.game, sv = this.saved; this.cam.yaw = sv.yaw; this.cam.desiredDistance = sv.dist; g.aiEnabled = sv.ai; g.godMode = sv.god; g.playerInvisible = sv.invisible;
      if (!finished) { g.clearAftermath(); this.smoke.clearAll?.(); g.resetGuards(); g.resetLevelState();   // (the beats flip light groups and doors for framing: an aborted tour must hand the authored level back, as the finished path's restart does)
        if (!g.player.down) { g.teleportPlayer(sv.pos); g.player.slot = sv.slot; g.player.pistol.lightOn = sv.light; g.player.crouch = false; } }   // stopped mid-beat: nothing in the air or in the sound queue survives (fresh guards would hear shots nobody alive fired), and never hand back a mortal player standing among men a beat just made alert
    }
    if (this.game.blackout.active && !this.game.blackout.permanent) this.game.endBlackout();
  }

  private next() {
    const prev = this.shots[this.shotI]; prev?.exit?.();
    this.shotI++; this.shotT = 0; this.fade = Math.max(this.fade, 0.999);   // the cut itself happens under black (update() fades it back in)
    const s = this.shots[this.shotI];
    if (!s) { this.stop(); return; }
    s.at?.forEach(e => e.done = false);
    s.enter?.();
    if (s.distFrom !== undefined) { this.cam.distance = this.cam.desiredDistance = s.distFrom; const p0 = typeof s.pos === 'function' ? s.pos() : s.pos; this.cam.target = v3.copy(p0); this.cam.desired = v3.copy(p0); }   // (after enter's teleports: the cut is under black, so snap distance AND look-at — the reveal must open ON the subject, not panning onto it)
    this.caption = s.caption; this.sub = s.sub ?? '';
  }

  /** Returns the camera follow point while active (else null). */
  update(dt: number): Vec3 | null {
    if (!this.active) return null;
    this.t += dt; this.shotT += dt;
    const s = this.shots[this.shotI]; if (!s) { this.stop(); return null; }
    for (const e of s.at ?? []) if (!e.done && this.shotT >= e.t) { e.done = true; e.run(); }
    s.tick?.(this.shotT, dt);
    // ease camera yaw / distance toward the shot; the target itself pans slowly across the shot (drift) so nothing sits dead still
    const [w0, w1] = s.swing ?? [0.12, 0.88]; const u = Math.min(1, Math.max(0, (this.shotT / s.dur - w0) / Math.max(0.01, w1 - w0))); const ease = u * u * (3 - 2 * u);   // hold, swing, hold
    const yawTo = typeof s.yawTo === 'function' ? s.yawTo() : s.yawTo;
    const yawT = (yawTo !== undefined ? s.yaw + wrapAngle((yawTo - s.yaw) * DEG) / DEG * ease : s.yaw + (s.drift ?? 0.7) * (this.shotT - s.dur * 0.5)) * DEG;
    this.cam.yaw += wrapAngle(yawT - this.cam.yaw) * (1 - Math.exp(-1.5 * dt));
    // fade: black over the last 0.35 s of a shot and the first 0.45 s of the next (the cut, teleports and resets happen in between)
    const toEnd = s.dur - this.shotT; const wantBlack = toEnd < 0.35 || this.shotT < 0.1;
    this.fade = wantBlack ? Math.min(1, this.fade + dt / 0.3) : Math.max(0, this.fade - dt / 0.45);
    const dist = typeof s.dist === 'function' ? s.dist() : s.dist;
    if (!(s.distFrom !== undefined && this.shotT < 0.55)) this.cam.desiredDistance = damp(this.cam.desiredDistance, dist, s.distFrom !== undefined && this.shotT < 4 ? 0.9 : 2, dt);   // (a distFrom shot holds its opening distance until the veil has cleared)
    if (this.shotT >= s.dur) this.next();
    const cur = this.active ? this.shots[this.shotI] ?? s : s;   // after a cut: the NEW shot's look-at (its enter has already run; the old getter would answer for a beat that has just torn itself down)
    return typeof cur.pos === 'function' ? cur.pos() : cur.pos;
  }
}
