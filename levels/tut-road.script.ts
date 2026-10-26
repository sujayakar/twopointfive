// TUT-ROAD's fights, staged (director.ts): ONE play program, 'tutorial', started with the mission, on a restart and on a checkpoint load. Every beat starts on a
// cue the lesson sends as a step comes up ('lesson:<cue>', the step's `cue` key) and starts CLEAN from it: nothing here is carried from an earlier beat but the
// program's own flags, which a restart empties. The level's six men (the roster's order: Kowalski the sentry, Novak behind the hut's plank wall, Reyes on the gun, Okafor,
// Lindqvist and Brandt outside the upper gate) are held on their marks from the first frame and are let go, or made to shoot, beat by beat. Nobody is spawned.
//
// A scripted round is a real round (combat.ts): what the sentry, the gun and the last three fire at is a PLACE the player has no reason to stand in once he
// has done what the line asks, never the player. The gun is the one exception, and an honest one: a man who walks out onto the open snow in front of it is
// handed to the game's own gunner. The places the level's generator shares with this file are written HERE and read there (tools/levels/tut-road.ts): the
// page's bundle takes this file and no tool.
import type { LevelScriptFn, UnitRef, Query, BeatCtx } from '../src/game/director';
import type { Vec3 } from '../src/math/vec';

/** the truck's flank as the sentry sees it through the gateway: every line to it is clear of both jambs, and passes a man's width and more from where the
 *  player takes the wall beside the gateway */
export const TRUCK_FLANK: Vec3[] = [[5.6, 1.35, 13.05], [5.6, 0.85, 13.4], [5.6, 1.5, 13.75], [5.6, 1.1, 13.2]];
/** THE MAN IN THE HUT'S BACK ROOM: where he waits (a nook behind a stub of the hut's own wall: no round through the door, its leaf, the boards or the window
 *  reaches it, so a burst at the door from any stand leaves him for his own line) and where he steps to when the boards' line comes up (behind the boards
 *  from every stand in the doorway and out on the road, the boards and nothing else between) */
export const HUT_NOOK: Vec3 = [21.75, 0, 10.25], HUT_MARK: Vec3 = [20.7, 0, 11.45];
/** what the man behind the hut's plank wall shoots at, blind, through the boards: the head of the front room's far wall, the one the view sees the face of. Every
 *  line to it leaves the boards a man's height up and climbing, and passes two paces from the doorway the player stands in */
export const HUT_WALL: Vec3[] = [[18.3, 2.3, 9.83], [19.0, 2.2, 9.83], [19.6, 2.35, 9.83], [18.7, 2.4, 9.83]];
/** what the gun shoots at as he comes out of the hut: the hut's end wall and the head of its wind wall, on the gun's side of both. No line to any of them
 *  crosses floor he can reach from the side door without walking round the roadblock into the open */
export const HUT_END: Vec3[] = [[22.53, 1.5, 10.6], [22.53, 2.1, 11.6], [22.53, 1.2, 12.5], [22.53, 2.3, 14.3], [22.53, 1.8, 13.2], [22.53, 2.2, 15.0], [22.53, 1.4, 11.1], [22.53, 2.0, 14.7]];
/** along the sandbags, a point every half metre: a hand over their top, so over a crouched head in the lane behind them and through a standing chest. The
 *  gun walks its fire up and down this line (traversing fire: no man walks its length standing between two passes) */
export const BAGS_TOP: Vec3[] = [22.8, 23.3, 23.8, 24.3, 24.8, 25.3, 25.8, 26.3, 26.8, 27.2].map((x): Vec3 => [x, 1.42, 15.5]);
/** the roadblock's wall, the gun's face of it: where he is, once he is on it. Short of its bare end by a man's width (the end is his to lean round) */
export const WALL_FACE: Vec3[] = [[27.8, 1.3, 15.37], [28.5, 1.7, 15.37], [29.1, 1.2, 15.37], [28.1, 1.9, 15.37], [28.8, 1.5, 15.37]];
/** THE LAST THREE: where each waits outside the upper gate (the lane the road leaves the map by, out of every line through the gate), where each stands
 *  once he has come in by it (in the open, in the gate's lamp, far enough from the roadblock's wall to be seen whole from its end), and what each shoots
 *  at until a round finds one of them: the head of the roadblock's wall, over a man standing behind it */
export const LAST_OUT: Vec3[] = [[38.6, 0, 12.2], [38.6, 0, 16.4], [39.4, 0, 12.6]], LAST_MEN: Vec3[] = [[36.1, 0, 13.3], [35.4, 0, 15.3], [36.1, 0, 16.1]];
const WALL_HEAD: Vec3[][] = [[[28.0, 2.3, 15.5], [28.4, 2.25, 15.5]], [[28.8, 2.3, 15.5], [28.6, 2.2, 15.5]], [[29.3, 2.3, 15.5], [29.0, 2.25, 15.5]]];
/** the gateway and the upper gate, chest high: where a held man looks */
const GATEWAY: Vec3 = [11, 1.2, 15.2], HUT_DOOR: Vec3 = [17, 1.2, 12.75], HUT_SIDE: Vec3 = [22.5, 1.3, 14.4], GATE: Vec3 = [37, 1.2, 14.25];

const script: LevelScriptFn = D => {
  const hutDoor = D.door('hut'), gate = D.door('gate');
  const roadEast = D.volume('road_east'), roadNose = D.volume('road_nose'), hutNorth = D.volume('hut_north'), field = D.volume('field'), openEast = D.volume('open_east');
  const STAFF = ['Kowalski', 'Novak', 'Reyes', 'Okafor', 'Lindqvist', 'Brandt'], LAST = ['Okafor', 'Lindqvist', 'Brandt'];
  const AIMS: Record<string, Vec3> = { Kowalski: TRUCK_FLANK[1], Novak: HUT_DOOR, Reyes: HUT_SIDE, Okafor: GATE, Lindqvist: GATE, Brandt: GATE };

  // a man told to aim somewhere turns his whole body to it (an order's `aimAt` alone is his arms and his eyes: he would shoot over his shoulder)
  const aim = (c: BeatCtx, u: UnitRef, at: Vec3, hot = false) => c.order(u, { aimAt: at, upper: 'aim', face: D.util.yawTo(c.query.pos(u), at), ...(hot ? { hot } : {}) });

  D.program('tutorial', P => {
    P.onStart('hold');
    for (const cue of ['road', 'cease', 'way', 'hut', 'gun', 'bags', 'ease', 'gate', 'open']) P.onSignal(`lesson:${cue}`, cue);

    // every man on his mark, his gun on the way the player will come, until his own beat lets him go (a man a beat has ordered is the program's until it lets
    // him go, fails or stops: this beat's end hands nobody back)
    P.beat('hold', function* (c) {
      for (const n of STAFF) { const u = c.unit(n); if (u && c.query.alive(u)) c.order(u, { goal: null, aimAt: AIMS[n], upper: 'aim', face: D.util.yawTo(c.query.pos(u), AIMS[n]) }); }
      yield c.frames(1);
    });

    // THE ROAD: the sentry has stopped the convoy's last truck and is putting rounds into it, until the player is at the post's wall ('cease': the gateway
    // is then the player's to lean round, and one hit puts a man down). He never looks for anyone else: the first man is a free one.
    P.beat('cease', function* (c) { c.flag.cease = true; yield c.frames(1); });
    P.beat('road', function* (c) {
      const s = c.unit('Kowalski'); if (!s) return;
      // (his rounds cross the road between the truck and the wall and nowhere else. Nothing there tells a new player to keep off it, so while the player is
      //  on that side, or coming round the truck's nose to it, he holds his fire: three rounds are over before a running man is across the margin the two
      //  volumes leave)
      const lane = () => c.query.inside(roadEast, 'sam') || c.query.inside(roadNose, 'sam');
      yield c.wait(0.8);
      for (let i = 0; c.query.alive(s) && !c.flag.cease; i++) {
        if (lane()) { yield c.wait(0.2); continue; }
        const at = TRUCK_FLANK[i % TRUCK_FLANK.length]; aim(c, s, at, true);
        yield c.stimulus.burst(s, at, 3, { cadence: 0.12, spread: 0.02 });
        yield c.wait(1.3 + c.rand() * 1.1);
      }
      if (c.query.alive(s)) aim(c, s, GATEWAY);   // he has heard something at his gate, and looks at it: square on to the man who leans round its corner
      yield c.until(q => !q.alive(s), 'the sentry is down');
    });

    // THE HUT'S DOOR: the line that has him shoot it out has gone by. Its leaf is off wherever gunfire splinters a door; where that rule is off, or his
    // rounds found the jambs, the lesson let the line go after a long burst and the door is thrown open here, so the way on never hangs on it
    P.beat('way', function* (c) { if (!c.query.door(hutDoor).open) { c.doors.lock(hutDoor, false); c.doors.open(hutDoor, 1.5, { hold: true }); } yield c.frames(1); });

    // THE PLANK WALL: the hut's door is down, and the man in its back room starts shooting through the boards he is behind at the wall over the front
    // room: rounds coming through wood are what says a round goes through wood. He shoots at a place, a man's height up; while the player stands under
    // that place he holds his fire. He is nobody's but this beat's until he is down.
    P.beat('hut', function* (c) {
      const u = c.unit('Novak'); if (!u) return;
      // out of his nook to the boards: he has heard his door go
      c.order(u, { goal: HUT_MARK, speed: 2.6, aimAt: HUT_WALL[0], upper: 'aim', hot: true });
      yield c.any(c.until(q => !q.alive(u) || D.util.distXZ(q.pos(u), HUT_MARK) < 0.2, 'the man in the back room is at the boards'), c.wait(4));
      if (c.query.alive(u)) c.order(u, { goal: null, aimAt: HUT_WALL[0], upper: 'aim', face: D.util.yawTo(c.query.pos(u), HUT_WALL[0]), hot: true });
      yield c.wait(0.3);
      for (let i = 0; c.query.alive(u); i++) {
        if (c.query.inside(hutNorth, 'sam')) { yield c.wait(0.2); continue; }
        const at = HUT_WALL[i % HUT_WALL.length]; aim(c, u, at, true);
        yield c.stimulus.burst(u, at, 3, { cadence: 0.12, spread: 0.006 });
        yield c.wait(0.9 + c.rand() * 0.7);
      }
    });

    // THE GUN: one man on a belt-fed gun in his nest under the rock, across the open snow from the hut. What he shoots at is the lesson of the step that
    // is up. A player who walks out onto that snow, or round the roadblock into the gun's own ground, has done the one thing no line asks: the gunner is
    // the game's own from then on, and shoots at HIM.
    P.beat('gun', function* (c) {
      const u = c.unit('Reyes'); if (!u) return;
      const out = () => c.query.inside(field, 'sam') || c.query.inside(openEast, 'sam');
      yield c.wait(0.5);
      for (let k = BAGS_TOP.length - 1, dir = -1, i = 0; c.query.alive(u); i++) {
        if (out()) { c.release(u, 'alert'); return; }
        const mode = c.flag.gun as string | undefined;
        // (the lesson decides nothing here but WHERE the rounds go: every one of them is a real round)
        if (mode === 'bags') {          // the traverse: a short burst a point, there and back along the sandbags' top
          const at = BAGS_TOP[k]; k += dir; if (k <= 0 || k >= BAGS_TOP.length - 1) dir = -dir;
          aim(c, u, at, true);
          yield c.stimulus.burst(u, at, 4, { cadence: 0.08, spread: 0.01 }); yield c.wait(0.1);
        } else {                        // the hut's end wall as he comes out of it; the roadblock's wall once he is behind that
          const pts = mode === 'ease' ? WALL_FACE : HUT_END, at = pts[Math.floor(c.rand() * pts.length) % pts.length];
          aim(c, u, at, true);
          yield c.stimulus.burst(u, at, mode === 'ease' ? 5 : 8, { cadence: 0.1, spread: mode === 'ease' ? 0.008 : 0.016 });
          yield c.any(c.wait(mode === 'ease' ? 1.0 + c.rand() * 0.8 : 0.7 + c.rand() * 0.7), c.until(out, 'the player is out in the open'));
        }
      }
    });
    P.beat('bags', function* (c) { c.flag.gun = 'bags'; yield c.frames(1); });
    P.beat('ease', function* (c) { c.flag.gun = 'ease'; yield c.frames(1); });

    // THE UPPER GATE: the gun is out, and three more come in by the gate from up the road and stand in its lamp, shooting the head of the wall he is
    // behind. The line that has him look ends once they stand there ('done:gate'), so the trigger is asked for with men in front of it. They are the
    // game's own once one of them is down. While he stands out in the open they hold their fire: every round of theirs is for the wall.
    P.beat('gate', function* (c) {
      yield c.wait(0.6);
      // (found by name: a beat starts clean from its cue. Each keeps his own place and his own piece of wall)
      const men = LAST.map((n, k) => ({ k, u: c.unit(n) })).filter((m): m is { k: number; u: UnitRef } => !!m.u && c.query.alive(m.u));
      if (!men.length) { c.signal('done:gate'); return; }
      const all = men.length, up = () => men.filter(m => c.query.alive(m.u)).length;
      for (const m of men) c.order(m.u, { goal: LAST_MEN[m.k], speed: 3.4, aimAt: WALL_HEAD[m.k][0], upper: 'aim', hot: true });
      yield c.any(c.until((q: Query) => men.every(m => !q.alive(m.u) || q.arrived(m.u)), 'in the gate\'s lamp'), c.wait(7));
      c.signal('done:gate');
      c.doors.open(gate, -1.45, { hold: true });   // (three men have come through it: it stands as they found it)
      for (let i = 0; up() === all; i++) {
        if (c.query.inside(openEast, 'sam')) { yield c.any(c.wait(0.3), c.until(() => up() < all, 'one of them is down')); continue; }
        const shots = men.map(m => { const at = WALL_HEAD[m.k][i % WALL_HEAD[m.k].length]; c.order(m.u, { aimAt: at, face: D.util.yawTo(c.query.pos(m.u), at) }); return c.stimulus.burst(m.u, at, 4, { cadence: 0.14, spread: 0.008 }); });
        yield c.any(c.all(...shots), c.until(() => up() < all, 'one of them is down'));
        yield c.any(c.wait(0.9 + c.rand() * 0.6), c.until(() => up() < all, 'one of them is down'));
      }
      yield c.wait(0.9);
      for (const m of men) if (c.query.alive(m.u)) c.release(m.u, 'alert');
    });

    // THE WAY OUT: the gate stands open on its stay, whoever brushed past it in the fight (the frame offers no row on a door here: a gate pushed shut
    // would be a wall)
    P.beat('open', function* (c) { c.doors.open(gate, -1.45, { hold: true }); yield c.frames(1); });
  });
};
export default script;
