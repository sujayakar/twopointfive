// TUT-KIT's fights, staged (director.ts): ONE play program, 'tutorial', started with the mission, on a restart and on a checkpoint load. Every beat starts on a
// cue the lesson sends as a step comes up ('lesson:<cue>', the step's `cue` key) and starts CLEAN from it: nothing here is carried from an earlier beat but the
// program's own flags, which a restart empties. The halt's six men (the room clear's own, named down its rooms: Tango 1 in the shop, 2 and 3 in the store, 4
// and 5 in the bunk room, 6 in the fuel room) are held on their marks from the first frame and are let go beat by beat. The man on the gun, the two who come
// through the store for the launcher and the two who come into the wash yard are this file's own, found by name before they are spawned.
//
// A scripted round is a real round (combat.ts): what the gun fires at is a PLACE the player has no reason to stand in once he has done what the line asks,
// never the player. The places the level's generator shares with this file (where the spawned men come in and stand) are written HERE and read there
// (tools/levels/tut-kit.ts): the page's bundle takes this file and no tool. The gun court's numbers are the fuel halt's own (levels/tut-solo.script.ts).
import type { LevelScriptFn, UnitRef, Query, BeatCtx, BeatFn } from '../src/game/director';
import type { Vec3 } from '../src/math/vec';

/** the man on the gun, behind his two low walls in the gun court's far corner */
const GUNNER = 'Novak', GUN_AT: Vec3 = [22.7, 0, 6.9];
/** through the arch, what the gun can reach of the truck court: the parked truck and the drums on its bed */
const TRUCK: Vec3[] = [[12.4, 1.45, 18.6], [11.6, 1.5, 19.2], [13.2, 1.4, 18.3], [12.0, 1.55, 18.9]];
/** along the sandbags, a point every 0.6 m: a hand over their top, so over a crouched head and through a standing chest. The gun walks its fire up and down
 *  this line (traversing fire: no man walks its length standing between two passes) */
const BAGS_TOP: Vec3[] = [16.6, 17.2, 17.8, 18.4, 19.0, 19.6, 20.2, 20.8].map((x): Vec3 => [x, 1.45, 13.6]);
/** the gap between the sandbags and the barrier, where a man crossing it is: hosed without a pause until the smoke is up. Eight lanes a hand apart, shin to
 *  belt (a crouched man is no smaller to them), two rounds a lane: no man is across between two passes */
const GAP_FLOOR: Vec3[] = [[21.55, 0.5, 13.75], [21.95, 0.75, 13.8], [21.75, 0.45, 13.75], [22.1, 0.7, 13.85], [21.65, 0.8, 13.7], [22.05, 0.5, 13.8], [21.85, 0.65, 13.75], [21.6, 0.6, 13.75]];
/** where he was: the sandbags' near end (the gun fires blind once the smoke is up) */
const WAS: Vec3[] = [[17.2, 1.3, 13.2], [18.4, 1.3, 13.4], [16.8, 1.42, 13.6]];
/** THE TWO FOR THE LAUNCHER come through the building behind him once he is back at the arch: where they come onto the floor (the lane behind the store,
 *  outside its back door, which is thrown open for them), where they stand (the shop's doorway, one a step behind the other) and what each shoots at until
 *  the grenade or a round finds one of them: the top of the arch's wall either side of its mouth, a man's height and more over anyone standing in it */
export const LAST_IN: Vec3[] = [[37.4, 0, 14.0], [38.4, 0, 14.5]], LAST_MEN: Vec3[] = [[25.7, 0, 13.98], [26.5, 0, 14.5]];
const LAST = ['Brandt', 'Marsh'];
const ARCH_HEAD: Vec3[][] = [[[16.0, 2.2, 12.5], [16.0, 2.1, 12.0], [16.0, 2.25, 12.3]], [[16.0, 2.2, 16.0], [16.0, 2.1, 16.5], [16.0, 2.25, 16.2]]];
/** THE TWO OF THE WASH YARD come in by its back door from the plain behind the halt while the squad is still inside the building (the cue of the line
 *  that sends him back out): where they come onto the floor, and where they get down, behind the wall of sandbags across the yard. They are down
 *  before anyone in the court can look: a man on his feet at that door shows over the bags */
export const YARD_IN: Vec3[] = [[37.6, 0, 19.5], [38.6, 0, 20.1]], YARD_MEN: Vec3[] = [[29.5, 0, 19.2], [29.5, 0, 20.4]];
const YARD = ['Voss', 'Kerr'], YARD_GATEWAY: Vec3 = [25.0, 1.2, 19.75];
/** the shop's doorway, the doorway in the wall between the shop and the store, the bunk room's door and the fuel room's from inside, chest high */
const SHOP_DOORWAY: Vec3 = [25.4, 1.2, 14.25], PART_DOORWAY: Vec3 = [30.5, 1.2, 14.25], BUNK_DOORWAY: Vec3 = [27.75, 1.1, 10.2], FUEL_DOORWAY: Vec3 = [31.3, 1.1, 9.25];
/** who is aimed where while he is held */
const AIMS: Record<string, Vec3> = { 'Tango 1': SHOP_DOORWAY, 'Tango 2': PART_DOORWAY, 'Tango 3': PART_DOORWAY, 'Tango 4': BUNK_DOORWAY, 'Tango 5': BUNK_DOORWAY, 'Tango 6': FUEL_DOORWAY };
const SQUAD = ['Red One', 'Red Two', 'Blue One', 'Blue Two'];

const script: LevelScriptFn = D => {
  const backDoor = D.door('store'), washDoor = D.door('wash');
  const shopLine = D.volume('shop_line'), shopIn = D.volume('shop_in'), storeIn = D.volume('store_in'), bunkIn = D.volume('bunk_in'), fuelIn = D.volume('fuel_in'), washIn = D.volume('wash_in');
  const gun = D.loadout('gunner', { team: 'hostile', weapon: 'pkm', light: false }), rifleman = D.loadout('rifleman', { team: 'hostile', weapon: 'akms', light: false }), torch = D.loadout('torch', { team: 'hostile', weapon: 'akms', light: true });
  D.systems.qrf(false);          // nobody is called: every man on this floor is one the mission put there
  D.systems.theirFrags(false);   // …and the only grenades thrown here are his and his squad's

  // a man told to aim somewhere turns his whole body to it (an order's `aimAt` alone is his arms and his eyes: he would shoot over his shoulder)
  const aim = (c: BeatCtx, u: UnitRef, at: Vec3, hot = false) => c.order(u, { aimAt: at, upper: 'aim', face: D.util.yawTo(c.query.pos(u), at), ...(hot ? { hot } : {}) });
  const men = (c: { unit(n: string): UnitRef | null; query: Query }, names: string[]): UnitRef[] => names.map(n => c.unit(n)).filter((u): u is UnitRef => !!u && c.query.alive(u));
  /** a line on the squad's net from the first of `names` on his feet */
  const radio = (c: BeatCtx, names: string[], line: string) => { const u = men(c, names)[0] ?? men(c, SQUAD)[0]; if (u) c.say(u, line, { radio: true }); };
  /** a held room is the AI's own once a stun grenade or a blast has gone off in it, one of its men is hit, or the player or a teammate is inside it */
  function* room(c: BeatCtx, names: string[], inside: ReturnType<typeof D.volume>, box: { x0: number; x1: number; z0: number; z1: number }): ReturnType<BeatFn> {
    const held = men(c, names); if (!held.length) return;
    const hit = () => c.until((q: Query) => held.some(u => !q.alive(u)), 'one of them is down');
    const inRoom = () => c.until((q: Query) => q.inside(inside, 'team'), 'the team is in the room');
    const r = yield c.any(c.on('bang', e => e.pos[0] > box.x0 && e.pos[0] < box.x1 && e.pos[2] > box.z0 && e.pos[2] < box.z1), hit(), inRoom());
    if (r.index === 0) yield c.any(c.wait(2.0), hit(), inRoom());   // (folded by the bang: the men coming in behind it have them)
    yield c.wait(0.3);
    for (const u of held) if (c.query.alive(u)) c.release(u, 'alert');
  }

  D.program('tutorial', P => {
    P.onStart('hold');
    for (const cue of ['gun', 'bags', 'gap', 'blind', 'shop', 'room', 'trucks', 'loose', 'call', 'box', 'dark', 'bunk', 'wall', 'yard', 'seen', 'free']) P.onSignal(`lesson:${cue}`, cue);

    // every man on his mark, his gun on the way the team will come, until his own beat lets him go (a man a beat has ordered is the program's until it lets
    // him go, fails or stops: this beat's end hands nobody back). The gun's man is put on his gun once: a checkpoint put back has him already, up or down
    P.beat('hold', function* (c) {
      // (not on the first frame: the mission stages with the level's own six, as the floor's check counts them, and he is twenty metres off the picture)
      if (!c.unit(GUNNER)) { yield c.wait(0.5); if (!c.unit(GUNNER)) c.spawn(gun, GUN_AT, { callsign: GUNNER, yaw: Math.PI / 2, light: false }); }
      for (const n of Object.keys(AIMS)) { const u = c.unit(n); if (u && c.query.alive(u)) c.order(u, { goal: null, aimAt: AIMS[n], upper: 'aim', face: D.util.yawTo(c.query.pos(u), AIMS[n]) }); }
      { const u = c.unit(GUNNER); if (u && c.query.alive(u)) c.order(u, { goal: null, aimAt: TRUCK[0], upper: 'aim', face: D.util.yawTo(c.query.pos(u), TRUCK[0]) }); }
      c.start('bunkroom'); c.start('fuelroom');
      yield c.frames(1);
    });

    // THE GUN: one man on a belt-fed gun in the far corner of the next court. What he shoots at is the lesson of the step that is up.
    P.beat('gun', function* (c) {
      const u = c.unit(GUNNER); if (!u) return;
      yield c.wait(0.5);
      for (let k = BAGS_TOP.length - 1, dir = -1, i = 0; c.query.alive(u); i++) {
        const mode = c.flag.gun as string | undefined;
        // (the lesson decides nothing here but WHERE the rounds go: every one of them is a real round)
        if (mode === 'bags') {          // the traverse: a short burst a point, there and back along the sandbags' top
          const at = BAGS_TOP[k]; k += dir; if (k <= 0 || k >= BAGS_TOP.length - 1) dir = -dir;
          aim(c, u, at, true);
          yield c.stimulus.burst(u, at, 4, { cadence: 0.08, spread: 0.01 }); yield c.wait(0.1);
        } else if (mode === 'gap') {    // the gap, hosed: nobody is meant to try it before the smoke
          const at = GAP_FLOOR[i % GAP_FLOOR.length]; aim(c, u, at, true);
          yield c.stimulus.burst(u, at, 2, { cadence: 0.08, spread: 0.006 }); yield c.wait(0.08);   // (a burst's first round leaves at once: the pause is the gun's own beat between the last of one and the first of the next)
        } else {                        // the parked truck through the arch, and (blind, slower) where he was before the smoke went up
          const pts = mode === 'blind' ? WAS : TRUCK, at = pts[Math.floor(c.rand() * pts.length) % pts.length];
          aim(c, u, at, true);
          yield c.stimulus.burst(u, at, mode === 'blind' ? 5 : 8, { cadence: 0.1, spread: 0.022 });
          yield c.wait(mode === 'blind' ? 1.6 + c.rand() : 0.7 + c.rand() * 0.7);
        }
      }
    });
    P.beat('bags', function* (c) { c.flag.gun = 'bags'; yield c.frames(1); });
    P.beat('gap', function* (c) { c.flag.gun = 'gap'; yield c.frames(1); });
    P.beat('blind', function* (c) { c.flag.gun = 'blind'; yield c.frames(1); });

    // THE SHOP: the man behind the door, in the doorway's line. The charge folds him; he is held until the player steps into that line (or the shop) and a second
    // more, then he is the AI's own. Nobody is shot at while he reads the line beside the door.
    P.beat('shop', function* (c) {
      const u = c.unit('Tango 1'); if (!u) return;
      const down = () => c.until((q: Query) => !q.alive(u), 'he is down');
      const r = yield c.any(c.until((q: Query) => q.inside(shopLine, 'sam') || q.inside(shopIn, 'sam'), 'the player is in his line'), down());
      if (r.index === 0) yield c.any(c.wait(1.0), down());
      if (c.query.alive(u)) c.release(u, 'alert');
    });

    // THE STORE: two men hold the room on its doorway, where the shop cannot see them. A stun grenade that goes off in their room folds them: they are his until
    // he has been in the room two seconds, or a breath after one of them is hit. A man who walks in with no grenade gone off is shot from both sides.
    P.beat('room', function* (c) {
      const pair = ['Tango 2', 'Tango 3'].map(n => c.unit(n)).filter((u): u is UnitRef => !!u); if (!pair.length) return;
      const hit = () => c.until((q: Query) => pair.some(u => !q.alive(u)), 'one of the pair is down');
      const inside = () => c.until((q: Query) => q.inside(storeIn, 'sam'), 'the player is in the store');
      const r = yield c.any(c.on('bang', e => e.pos[0] > PART_DOORWAY[0]), hit(), inside());
      if (r.index === 0) { const r2 = yield c.any(inside(), hit()); yield c.any(c.wait(r2.index === 0 ? 2.0 : 0.6), hit()); }
      else if (r.index === 1) yield c.wait(0.6);
      for (const u of pair) if (c.query.alive(u)) c.release(u, 'alert');
    });

    // THE LAUNCHER: he is back at the arch with his rifle when two more come out through the building behind him and stand in the shop's doorway, shooting
    // the head of the arch's wall over him: a doorway with two men in it, 8.5 m and more off, which is what the grenade is for. They are the AI's once one
    // of them is down.
    P.beat('loose', function* (c) { c.flag.loose = true; yield c.frames(1); });
    P.beat('trucks', function* (c) {
      c.doors.lock(backDoor, false); c.doors.open(backDoor, 1.5, { hold: true });
      yield c.wait(0.4);
      // (found by name first: a beat starts clean from its cue, and nobody is spawned twice. Each keeps his own place and his own piece of wall)
      const two = LAST_IN.map((p, k) => ({ k, u: c.unit(LAST[k]) ?? c.spawn(rifleman, p, { yaw: -Math.PI / 2, callsign: LAST[k], light: false }) })).filter(m => c.query.alive(m.u));
      if (!two.length) { c.signal('done:trucks'); return; }
      const all = two.length, up = () => two.filter(m => c.query.alive(m.u)).length;
      for (const m of two) c.order(m.u, { goal: LAST_MEN[m.k], speed: 3.4, aimAt: ARCH_HEAD[m.k][0], upper: 'aim', hot: true });
      yield c.any(c.until((q: Query) => two.every(m => !q.alive(m.u) || q.arrived(m.u)), 'in the doorway'), c.wait(6));
      c.signal('done:trucks');   // the line that had him look goes by: the shot is asked for with the two standing in the doorway
      // the show: until one of them is hit or the line has gone by ('loose'), however long he takes over the launcher
      for (let i = 0; up() === all && !c.flag.loose; i++) {
        const shots = two.map(m => { const at = ARCH_HEAD[m.k][i % ARCH_HEAD[m.k].length]; c.order(m.u, { aimAt: at, face: D.util.yawTo(c.query.pos(m.u), at) }); return c.stimulus.burst(m.u, at, 4, { cadence: 0.14, spread: 0.008 }); });
        yield c.any(c.all(...shots), c.until(() => up() < all, 'one of them is down'));
        yield c.any(c.wait(0.9 + c.rand() * 0.6), c.until((q: Query) => up() < all || !!q.flag('loose'), 'the show is over'));
      }
      yield c.wait(0.6);
      for (const m of two) if (c.query.alive(m.u)) c.release(m.u, 'alert');
    });

    // THE SQUAD'S HALF. What the four say as each thing comes up, a beat before its line asks for it
    P.beat('call', function* (c) { yield c.wait(0.8); radio(c, ['Red One'], 'We heard all that from the road. Say the word.'); });
    P.beat('box', function* (c) { yield c.wait(0.6); radio(c, ['Red One'], 'There\'s a breaker box on the court wall. We can kill the lights.'); });
    // THE DARK: the men still inside turn their flashlights on by the game's own rule for men in the dark (darkfoe.ts, powerout.ts), not by an order here
    P.beat('dark', function* (c) { yield c.wait(0.5); radio(c, ['Red Two'], 'Lights are out. Goggles on.'); });
    P.beat('bunk', function* (c) { yield c.wait(0.6); radio(c, ['Blue One'], 'There\'s a door off the shop we haven\'t opened.'); });
    P.beat('wall', function* (c) { yield c.wait(0.6); radio(c, ['Blue One'], 'That door\'s locked. We can put a charge on the wall.'); });
    // (started with the program, so a checkpoint put back anywhere has them: each room is held until the squad's way into it, whichever it is)
    P.beat('bunkroom', function* (c) { yield* room(c, ['Tango 4', 'Tango 5'], bunkIn, { x0: 25, x1: 31, z0: 5, z1: 10.5 }); });
    P.beat('fuelroom', function* (c) { yield* room(c, ['Tango 6'], fuelIn, { x0: 30.4, x1: 36, z0: 5, z1: 10.5 }); });   // (from the bunk room's face of the wall: the squad's charge goes off on that side of it)

    // THE WASH YARD: two more come in by its back door, low, and get down behind the sandbags with their flashlights on, where no rifle in the court
    // reaches them. They are the AI's own once a grenade has gone off in the yard, one of them is hit, or the team is in there with them — or when the
    // last line comes up. `seen` is the line that has him look: a teammate calls them, and the line goes by a breath later
    P.beat('free', function* (c) { c.flag.free = true; yield c.frames(1); });
    P.beat('seen', function* (c) {
      if (men(c, YARD).length) { radio(c, ['Red Two', 'Blue Two'], 'Two more in the wash yard, down behind the sandbags. No shot from here.'); yield c.wait(2.2); }
      c.signal('done:seen');
    });
    P.beat('yard', function* (c) {
      c.doors.lock(washDoor, false); c.doors.open(washDoor, 1.5, { hold: true });
      yield c.wait(0.4);
      const two = YARD_IN.map((p, k) => ({ k, u: c.unit(YARD[k]) ?? c.spawn(torch, p, { yaw: -Math.PI / 2, callsign: YARD[k], light: true }) })).filter(m => c.query.alive(m.u));
      if (!two.length) { c.signal('done:yard'); return; }
      const hit = () => c.until((q: Query) => two.some(m => !q.alive(m.u)), 'one of them is down');
      // (each carries a light of his own and the order keeps it dark while he comes in: an order's `light: false` puts a light out, and `light: true` only stops doing that)
      for (const m of two) c.order(m.u, { goal: YARD_MEN[m.k], speed: 2.6, crouch: true, aimAt: YARD_GATEWAY, upper: 'aim', light: false, hot: false });
      yield c.any(c.until((q: Query) => two.every(m => !q.alive(m.u) || q.arrived(m.u)), 'behind the sandbags'), c.wait(9), hit());
      for (const m of two) if (c.query.alive(m.u)) c.order(m.u, { goal: null, crouch: true, aimAt: YARD_GATEWAY, upper: 'aim', face: D.util.yawTo(c.query.pos(m.u), YARD_GATEWAY), light: true });
      c.signal('done:yard');
      yield c.any(c.on('bang', e => e.pos[0] > 25 && e.pos[2] > 17.5), hit(), c.until((q: Query) => q.inside(washIn, 'team') || !!q.flag('free'), 'the team is in the yard'));
      yield c.wait(0.3);
      for (const m of two) if (c.query.alive(m.u)) c.release(m.u, 'alert');
    });
  });
};
export default script;
