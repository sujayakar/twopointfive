// THE SQUAD'S TUTORIAL MISSION ON THE GROUND, ITS SCRIPT (levels/tut-team.json; game/director.ts): what the road does while the player, behind his own truck,
// learns the squad's key. The road's beats are the tunnel station's own (levels/tut-sky.script.ts: the roadblock's rounds WALKED over everything around the
// two files, the squad's called smoke, the blind fire into it, the roadblock coming on for the gate) with three differences, each because he is on the floor:
//   · THE SQUAD IS HELD ON ITS KNEES BY THIS SCRIPT until his first order (the mission from above has the frame's seat hold it), and NOTHING IS AIMED WHERE A
//     STANDING MAN'S HEAD IS behind the tails: a pickup hides a man to 1.6 m and one round puts the player down. So the two trucks' boxes stop at the bonnet
//     (1.15 m: a round at a screen went on over the cab at head height), the boxes on the road are well in front of the trucks (a round aimed at the
//     road near a nose skims on under the truck to a kneeling man's shin), the box over the tails is a man's height higher, and the blind fire into the
//     smoke is aimed at a bonnet's height in front of a nose and held tight. tools/qa/tut-team-check.ts holds it, he on his feet, pinned and under the
//     blind fire.
//   · THE ROADBLOCK COMES FOR THE GATE ON THE STEP'S OWN CUE, which comes up when he is through the gate himself: a line that asks him to walk is never the
//     start of a fight he is not standing in.
//   · NOTHING IS STAGED IN THE DARK: the lamps stay on, and the bunkhouse keeps to itself behind its door until the squad comes to it.
// One play program, 'tutorial'. Every beat starts on a lesson step's cue and can start clean from it. Scripted rounds are real rounds. Imports nothing from src at run time.
import type { LevelScriptFn, BeatCtx, UnitRef, Query } from '../src/game/director';
import type { Vec3 } from '../src/math/vec';

/** the roadblock's three posts (behind the sandbags tools/levels/tut-sky.ts lays across the road at the tunnel's mouth) and the names its men answer to
 *  (the station's own five are Tango 1 to 5, down its rooms: game/roomclear.ts) */
const POSTS: { name: string; at: Vec3 }[] = [{ name: 'Tango 6', at: [34.0, 0, 21.6] }, { name: 'Tango 7', at: [34.9, 0, 23.6] }, { name: 'Tango 8', at: [32.4, 0, 23.2] }];
/** WHERE THE ROADBLOCK'S ROUNDS GO while the squad is behind its trucks: boxes to aim into (`by`: the posts with a clear line into it — the trucks, the
 *  sandbags and the gun truck stand in the others' — · `w`: its share of the bursts). The maintainer, playing: "would be nice if the fire wasn't at a
 *  single point, was spread around". A round aimed into a box in the air goes on to what is behind it: the wall, the snow, the road out west. Measured on
 *  this floor from each post's eye (fifteen points a box, all but the last stretch of each line clear), and none is nearer a kneeling man than a truck's width */
interface Zone { lo: Vec3; hi: Vec3; w: number; by?: string[] }
const ZONES: Zone[] = [
  { lo: [10.9, 0.65, 21.7], hi: [11.9, 1.15, 22.9], w: 3 },                                  // the north truck: nose and bonnet (never its screen: see the head of this file)
  { lo: [9.3, 0.65, 24.0], hi: [10.5, 1.15, 25.2], w: 3, by: ['Tango 6', 'Tango 8'] },       // the south truck's
  { lo: [16.0, 0.05, 21.6], hi: [19.0, 0.05, 23.0], w: 3, by: ['Tango 7', 'Tango 8'] },      // the road well in front of them (a round aimed at the road nearer the trucks skims on under one)
  { lo: [15.0, 0.05, 23.6], hi: [18.5, 0.05, 24.4], w: 2, by: ['Tango 6'] },
  { lo: [3.0, 0.7, 20.5], hi: [9.0, 1.6, 20.5], w: 3 },                                      // past the north truck's flank: the road wall beside that file
  { lo: [7.5, 0.7, 25.7], hi: [11.0, 1.1, 26.1], w: 3, by: ['Tango 6'] },                    // past the south truck's: the rail's bank beside the other
  { lo: [1.5, 2.9, 22.0], hi: [1.5, 3.2, 25.0], w: 2, by: ['Tango 6', 'Tango 8'] },          // over the tails, well over a standing man (he is one of the file here)
];
/** what the men behind the trucks call while they are pinned, in turn */
const PINNED: [string, string][] = [['Blue Two', 'Taking fire! Stay down!'], ['Red Two', 'Rounds coming past the trucks!'], ['Blue One', 'We can\'t stay here!']];
/** where the squad's two smokes land (between the trucks and the roadblock) · seconds before more goes out while no order has come (a can burns about 32) */
const SMOKES: Vec3[] = [[15.6, 0, 22.4], [17.8, 0, 24.2]], SMOKE_AGAIN = 27;
/** what each thrower calls, the first cans and the cans after them · seconds from a man's call to his can leaving his hand · seconds from one man's call to the next man's */
const SMOKE_CALLS = [['Throwing smoke!', 'Smoke out!'], ['More smoke!', 'Smoke out!']], SMOKE_CALLED = 0.7, SMOKE_APART = 0.45;
/** seconds from the second can leaving a hand to the smoke's line going by (the cans are on the road and burning) */
const SMOKE_UP = 1.6;
/** …and once the smoke is out: into it, and the south lane behind it (the squad's lane is the wall's) */
const AT_SMOKE: Vec3[] = [[15.4, 0.9, 24.5], [14.2, 0.9, 24.8], [16.6, 0.95, 24.6], [13.6, 0.85, 24.9]];   // (at the height of a bonnet, in front of a truck's nose: whatever goes through the smoke stops in a truck)
/** seconds the road is quiet after the squad is through, before the roadblock fires into the smoke again at a man who has not followed */
const LINGER = 9;
const SQUAD = ['Red One', 'Red Two', 'Blue One', 'Blue Two'];
const YARD = { x0: 4.4, x1: 20.5, z0: 12.2, z1: 19.7 };
const WEST: Vec3 = [4.5, 1.1, 23.6];
/** the bunkhouse's two, and the door they keep */
const BUNK_MEN = ['Tango 4', 'Tango 5'], BUNK_DOOR: Vec3 = [28.6, 1.1, 16.25];
/** the kitchen's one man, and the door from the tea room he is held looking at (the way this mission brings the squad to him) */
const COOK = 'Tango 3', PASS_IN: Vec3 = [14.2, 1.1, 6.25];

const script: LevelScriptFn = D => {
  const rifle = D.loadout('roadblock', { team: 'hostile', weapon: 'akms', light: false });
  const yaw = -Math.PI / 2;
  const passDoor = D.door('pass'), kitchenDoor = D.door('kitchen'), bunkDoor = D.door('bunk');
  D.systems.qrf(false);   // nobody is called: every man on this floor is one the mission put there

  /** the men of the roadblock on their feet, by name (spawned once: a checkpoint put back has them already) */
  const block = (c: BeatCtx, spawn: boolean): UnitRef[] => {
    const out: UnitRef[] = [];
    for (const P of POSTS) { let u = c.unit(P.name); if (!u && spawn) u = c.spawn(rifle, P.at, { callsign: P.name, yaw, light: false }); if (u && c.query.alive(u)) out.push(u); }
    return out;
  };
  const squad = (c: BeatCtx): UnitRef[] => SQUAD.map(n => c.unit(n)).filter((u): u is UnitRef => !!u && c.query.alive(u));
  /** inside the bunkhouse's own walls */
  const inBunk = (q: Query, u: UnitRef) => { const p = q.pos(u); return p[0] > 29.1 && p[0] < 35.9 && p[2] > 13.6 && p[2] < 19.9; };
  const inKitchen = (q: Query, u: UnitRef) => { const p = q.pos(u); return p[0] > 14.0 && p[0] < 22.0 && p[2] > 4.0 && p[2] < 11.5; };
  const inYard = (q: Query, u: UnitRef) => { const p = q.pos(u); return p[0] > YARD.x0 && p[0] < YARD.x1 && p[2] > YARD.z0 && p[2] < YARD.z1; };
  const pick = <T,>(c: BeatCtx, a: T[]): T => a[Math.min(a.length - 1, Math.floor(c.rand() * a.length))];
  /** the roadblock fires in turns, each burst WALKED over one of `zones` — two or three rounds to a place in it, then another place — until `stop` */
  function* rake(c: BeatCtx, zones: Zone[], stop: () => boolean, o: { rounds: [number, number]; gap: [number, number]; spread: number }) {
    const total = zones.reduce((a, z) => a + z.w, 0), mix = (a: number, b: number) => a + c.rand() * (b - a);
    while (!stop()) {
      const men = block(c, false); if (!men.length) return;
      let r = c.rand() * total, Z = zones[0]; for (const z of zones) { r -= z.w; if (r <= 0) { Z = z; break; } }
      const can = men.filter(m => !Z.by || Z.by.includes(m.callsign ?? '')); if (!can.length) { yield c.wait(0.05); continue; }   // (its posts are down: another box)
      const u = pick(c, can), heavy = u.callsign === 'Tango 7';
      let left = Math.round(mix(o.rounds[0], o.rounds[1])) * (heavy ? 2 : 1);
      while (left > 0 && !stop() && c.query.alive(u)) {
        const at: Vec3 = [mix(Z.lo[0], Z.hi[0]), mix(Z.lo[1], Z.hi[1]), mix(Z.lo[2], Z.hi[2])], n = Math.min(left, c.rand() < 0.5 ? 2 : 3); left -= n;
        c.order(u, { goal: null, aimAt: at, upper: 'aim', hot: true });
        yield c.stimulus.burst(u, at, n, { cadence: heavy ? 0.085 : 0.11, spread: o.spread });
        yield c.wait(mix(0.05, 0.16));
      }
      yield c.wait(mix(o.gap[0], o.gap[1]));
    }
  }
  /** the roadblock fires in turns at `marks` until `stop` */
  function* harass(c: BeatCtx, marks: Vec3[], stop: () => boolean, o: { rounds: [number, number]; gap: [number, number]; spread: number }) {
    while (!stop()) {
      const men = block(c, false); if (!men.length) return;
      // (a mark in the south lane is the north post's: from the other two the gun truck stands in the line)
      const at = pick(c, marks), north = men.find(m => m.callsign === 'Tango 6'), u = at[2] > 24 && north ? north : pick(c, men), heavy = u.callsign === 'Tango 7';
      c.order(u, { goal: null, aimAt: at, upper: 'aim', hot: true });
      const n = Math.round(o.rounds[0] + c.rand() * (o.rounds[1] - o.rounds[0])) * (heavy ? 2 : 1);
      yield c.stimulus.burst(u, at, n, { cadence: heavy ? 0.085 : 0.11, spread: o.spread });
      yield c.wait(o.gap[0] + c.rand() * (o.gap[1] - o.gap[0]));
    }
  }

  D.program('tutorial', P => {
    P.onStart('hold'); P.onStart('cook');
    P.onSignal('lesson:ambush', 'ambush'); P.onSignal('lesson:smoke', 'smoke'); P.onSignal('lesson:push', 'push'); P.onSignal('lesson:indoors', 'indoors'); P.onSignal('lesson:house', 'house'); P.onSignal('lesson:out', 'out');

    // THE BUNKHOUSE KEEPS TO ITSELF (from the start, and again after every checkpoint, kept or put back): its two hold their marks beside their own door
    // through the whole fight for the range across the yard, and while he stands outside it with the squad reading the line: one round puts him down, and
    // a man let go with five rifles two steps from his door comes out. They are the game's own men from the moment their door opens or one of the squad
    // is in their room: the room is then a fight like any other. Only a man still INSIDE his room is held
    P.beat('hold', function* (c) {
      const met = (q: Query) => q.door(bunkDoor).open || squad(c).some(m => inBunk(q, m));
      if (met(c.query)) return;
      const held = BUNK_MEN.map(n => c.unit(n)).filter((u): u is UnitRef => !!u && c.query.alive(u) && inBunk(c.query, u));
      for (const u of held) c.order(u, { goal: null, aimAt: BUNK_DOOR, upper: 'aim', hot: false });
      yield c.frames(1);
      yield c.until(met);
      for (const u of held) if (c.query.alive(u) && inBunk(c.query, u)) c.release(u, 'alert');
    });

    // THE KITCHEN'S MAN STAYS IN HIS KITCHEN (from the start, and again after every checkpoint). Left to himself he answers the fight for the tea room by
    // going round through the yard to it, and the player is standing in that yard: one round puts him down, on the step that asks him for a key. So he is
    // held on his mark, looking at the door from the tea room, until a door of his room opens or one of the squad is in it. From then he is the game's own
    P.beat('cook', function* (c) {
      const u = c.unit(COOK); if (!u || !c.query.alive(u) || !inKitchen(c.query, u)) return;
      const met = (q: Query) => q.door(passDoor).open || q.door(kitchenDoor).open || squad(c).some(m => inKitchen(q, m));
      if (met(c.query)) return;
      c.order(u, { goal: null, aimAt: PASS_IN, upper: 'aim', hot: false });
      yield c.frames(1);
      yield c.until(q => !q.alive(u) || met(q));
      if (c.query.alive(u)) c.release(u, 'alert');
    });

    // THE ROAD: the roadblock opens up on the two trucks and keeps it up, its rounds walked over the noses and screens, the road, the wall and the bank
    // beside the two files, the air over their heads (rake, ZONES). The squad is on its knees behind its tails, held there by this beat, and says so
    P.beat('ambush', function* (c) {
      for (const u of block(c, true)) c.order(u, { goal: null, aimAt: WEST, upper: 'aim', crouch: false, hot: true, flinch: false });
      for (const u of squad(c)) c.order(u, { goal: null, crouch: true, hot: false, flinch: true });
      yield c.wait(0.6);
      const red = c.unit('Red One'); if (red) c.say(red, 'Contact front! Roadblock!', { radio: true });
      c.start('pinned');
      yield* rake(c, ZONES, () => c.flag.smoke === true || c.flag.broke === true, { rounds: [5, 9], gap: [0.2, 0.7], spread: 0.014 });
    });
    P.beat('pinned', function* (c) {
      for (let k = 0; !c.flag.smoke && !c.flag.broke; k++) {
        yield c.any(c.wait(5.5 + c.rand() * 2), c.until(() => c.flag.smoke === true || c.flag.broke === true)); if (c.flag.smoke || c.flag.broke) return;
        const [who, line] = PINNED[k % PINNED.length], u = c.unit(who); if (u && c.query.alive(u)) c.say(u, line, { radio: true });
      }
    });

    // THE SMOKE, THEN THE BREAK: the squad's own smoke goes out on its own step, a line that asks for nothing and is gone when both cans are out, so the
    // line that asks for the move comes up with the smoke standing (more goes out while no order comes). Every can is called by the man who throws it, a
    // beat before it leaves his hand. The roadblock fires blind into the smoke and the south lane, never down the lane along the wall the squad runs
    P.beat('smoke', function* (c) {
      if (c.flag.smoke) return; c.flag.smoke = true;
      const smoke = function* (again: boolean) {
        const men = squad(c), by = [c.unit('Red Two') ?? men[0], c.unit('Blue One') ?? men[1] ?? men[0]];
        const call = (i: number) => { const u = by[i]; if (u && c.query.alive(u)) c.radio(u.callsign, SMOKE_CALLS[again ? 1 : 0][i]); };
        const toss = (i: number) => { const u = by[i]; if (u && c.query.alive(u)) c.stimulus.charge(SMOKES[i], { by: u, kind: 'smoke', fuse: 0.8, toss: true }); };
        call(0); yield c.wait(SMOKE_APART); call(1); yield c.wait(SMOKE_CALLED - SMOKE_APART); toss(0); yield c.wait(SMOKE_APART); toss(1);
      };
      c.start('ordered'); c.start('break'); c.start('linger');
      yield c.wait(0.4);
      c.start('blind'); yield* smoke(false);
      yield c.wait(SMOKE_UP);
      c.signal('done:smoke');
      while (!c.flag.broke) { yield c.any(c.wait(SMOKE_AGAIN - SMOKE_CALLED - SMOKE_APART - SMOKE_UP), c.until(() => c.flag.broke === true)); if (!c.flag.broke) yield* smoke(true); }
    });
    P.beat('break', function* (c) {
      yield c.until(() => c.flag.broke === true);
      yield c.wait(0.25);
      for (const u of squad(c)) c.release(u, 'patrol');
      yield c.any(c.wait(2.2), c.until(q => squad(c).every(u => inYard(q, u))));
      c.flag.through = true;
    });
    P.beat('ordered', function* (c) { yield c.on('signal', e => !!e.name && e.name.startsWith('order:')); c.flag.broke = true; });
    // (the fire stops while the squad runs the wall's lane for the gate, and stays stopped: he runs that lane after them)
    P.beat('blind', function* (c) { yield* harass(c, AT_SMOKE, () => c.flag.through === true, { rounds: [3, 5], gap: [0.5, 1.2], spread: 0.02 }); });
    // (…for as long as it takes a man to run it. If he stays on the road the roadblock starts again, into the smoke and the south lane as before, and keeps
    //  it up until the gate's step: three men standing with their guns down are men he would walk up to)
    P.beat('linger', function* (c) {
      yield c.until(() => c.flag.through === true || c.flag.pushed === true);
      yield c.any(c.wait(LINGER), c.until(() => c.flag.pushed === true));
      yield* harass(c, AT_SMOKE, () => c.flag.pushed === true, { rounds: [3, 5], gap: [0.7, 1.6], spread: 0.02 });
    });

    // THE GATE (and its checkpoint put back): he is in the yard with them. The roadblock's men come on for the gate one after another through the gap in
    // their own sandbags and down the road into its mouth, and each is his own man there. The fight at the gate is the game's own; whoever is still on his
    // feet after it is walked in again, a little further the second time
    P.beat('push', function* (c) {
      c.flag.broke = true; c.flag.smoke = true; c.flag.through = true; c.flag.pushed = true;
      yield c.wait(0.8);
      { const u = c.unit('Red One') ?? squad(c)[0]; if (u && block(c, false).length) c.say(u, 'They\'re coming for the gate. We\'ve got it.', { radio: true }); }
      for (let k = 0; block(c, false).length; k++) {
        const left = block(c, false);
        for (const [i, u] of left.entries()) {
          c.order(u, { goal: [14.6 + i * 0.7 - Math.min(1, k) * 0.9, 0, 19.2 - i * 0.5 - Math.min(1, k) * 0.6], speed: 3.4, aimAt: [11.4, 1.1, 16.4], upper: 'aim', hot: true, direct: true });
          if (i < left.length - 1) yield c.any(c.wait(0.7), c.until(() => block(c, false).length === 0));
        }
        yield c.any(c.wait(k ? 5 : 7), c.until(q => left.every(u => !q.alive(u) || q.arrived(u))));
        for (const u of block(c, false)) c.release(u, 'alert');
        yield c.any(c.wait(4), c.until(() => block(c, false).length === 0));
      }
      c.signal('done:roadblock');
    });

    // INDOORS (and the checkpoint in the yard put back): the road is quiet. Whoever of the roadblock is somehow left is held where he stands
    P.beat('indoors', function* (c) {
      c.flag.broke = true; c.flag.smoke = true; c.flag.through = true; c.flag.pushed = true;
      for (const u of block(c, false)) c.order(u, { goal: null, aimAt: [16.5, 1.1, 21.4], upper: 'aim', hot: false });
      yield c.wait(0.1);
      c.signal('done:roadblock');
    });

    // THE RANGE IS THEIRS, and then the station: one man says so, a breath after the line comes up (each starts clean: a checkpoint put back says it again)
    P.beat('house', function* (c) {
      c.flag.broke = true; c.flag.smoke = true; c.flag.through = true; c.flag.pushed = true;
      yield c.wait(0.6);
      const u = c.unit('Red One') ?? squad(c)[0]; if (u) c.say(u, 'Kitchen\'s clear. There\'s one more building across the yard.', { radio: true });
    });
    P.beat('out', function* (c) {
      c.flag.broke = true; c.flag.smoke = true; c.flag.through = true; c.flag.pushed = true;
      yield c.wait(0.4);
      const u = c.unit('Red One') ?? squad(c)[0]; if (u) c.say(u, 'Station\'s clear.', { radio: true });
    });
  });
};
export default script;
