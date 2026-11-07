// THE PLAYER AND HIS OWN SIDE — the check behind src/game/friends.ts (FRIENDS.noSnap / FRIENDS.lowReady: on in the page, ?friendsnap / ?nomuzzle; off on the
// frozen stand, TPS_FRIENDS=1 / TPS_MUZZLE=1).
//   bun run tools/qa/friends-check.ts        (also `probes.ts friends`; passes on both stands: it throws the switches itself)
// One room clear staged with the pair (doortools-probes.ts rig), nobody's senses on and every man put back on his mark each frame, Sam put three metres from a teammate on open floor, every click a real click:
//   noSnap   (1) the cursor on a teammate's chest: the reticle takes nobody (no ring, no snap point) — off, it took him;
//            (2) the cursor on a defender Sam has a line to: he is the reticle's either way;
//            (3) the rule passes over exactly the living men who are not hostile to him;
//   lowReady (4) the rifle up, a teammate in its line, nobody in a fight: low ready, and a click fires nothing and says whose line it is;
//            (4b) the cursor on his body (the free aim then lands beyond him and the line passes beside him): down all the same;
//            (5) the aim turned off him: up again a beat later, and a click fires;
//            (6) the pistol the same;
//            (7) a wall between them: the gun stays up;
//            (8) in a fight (Sam fired a moment ago / a defender alert): the gun stays up with the teammate in its line;
//            (9) a defender in the line, calm: the gun stays up;
//            (9b) a teammate standing BEYOND that calm defender on the same line: the gun is on the defender, stays up, and the click fires;
//            (10) the switch off: never lowered;
//   the stands (11) both rows are in FROZEN_HOLDS with their words, both words in STAND_ENV.
// FRIENDS_BREAK=1 = the switches left off for the 'on' lines: (1), (4) and (10b) must fail.   exit 0 = all hold · 1 = one does not
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');

export async function friendsCheck(): Promise<boolean> {
  const { rig } = await import('./doortools-probes.ts');
  const { FRIENDS } = await import(`${ROOT}/src/game/friends.ts`);
  const P = await import(`${ROOT}/src/host/profile.ts`); const E = await import(`${ROOT}/src/host/standEnv.ts`);
  const { hostileTo } = await import(`${ROOT}/src/game/teams.ts`);
  let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== friends: no reticle on his own side; his gun comes down off a teammate out of a fight ===');
  const R: any = await rig(5101, { stage: 'rc' }); const { W, game, input } = R; const pl = game.player, c = pl.char;
  const was = { ...FRIENDS }; const brk = process.env.FRIENDS_BREAK === '1';
  const msgs: string[] = R.said;
  game.aiEnabled = false;   // (nobody's senses: the floor stays calm and the men stay where they are put)
  const mate = R.ops[0], other = R.ops[1], foe = R.hostiles.find((h: any) => h.state !== 'dead');
  const chest = (gd: any): [number, number, number] => gd.char.bones.chest ?? [gd.char.pos[0], gd.char.pos[1] + 1.2, gd.char.pos[2]];
  /** put a man somewhere and keep him there, facing `yaw` */
  const put = (gd: any, p: number[], yaw = 0) => { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.char.vel[0] = gd.char.vel[2] = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; };
  // open floor: the middle aisle of the cubicles. Sam at A, the teammate 3 m east of him, the other teammate and the defender parked away
  const A = [15, 0, 17.6], B = [18, 0, 17.6], AWAY = [15, 0, 21.5], FOE = [12, 0, 17.6];
  const park: [any, number[]][] = [[mate, B], [other, AWAY]];
  const hold = () => { for (const [gd, p] of park) put(gd, p, -Math.PI / 2); for (const h of game.guards) if (hostileTo(h, 'player')) { h.state = h === foe && foeAlert ? 'alert' : 'patrol'; h.awareness = 0; } };
  let foeAlert = false, aimAt: number[] = chest(mate);
  const BEYOND = [22, 0.02, 17.6];   // a point of floor beyond the teammate on Sam's line to him: the cursor there lays the gun's own line through him
  const idle = (n: number, click = false) => { for (let i = 0; i < n; i++) { hold(); W.cursorAt(aimAt as any); if (click && i === 0) input.clicked |= 1; R.step(); } };
  /** the gun's own heading come round to the cursor's bearing (a degree), or `max` frames: with the aim's weight on (aimweight.ts, the page's stand) a swing
   *  from one side to the other takes longer than the few frames an eased aim did, and a line asked mid-swing is asked of a gun that is not on it yet */
  const settle = (max = 120) => { for (let i = 0; i < max; i++) { idle(1); const want = Math.atan2(aimAt[0] - c.pos[0], aimAt[2] - c.pos[2]); let d = c.aimYaw - want; d = Math.atan2(Math.sin(d), Math.cos(d)); if (Math.abs(d) < 0.0175) return i + 1; } return max; };
  game.teleportPlayer(A); idle(30);
  const rifleUp = () => { if (!pl.rifleUp) { input.pressed.add('Digit2'); idle(1); if (!pl.rifleUp) { input.pressed.add('KeyE'); idle(1); } idle(90); } };
  const pistolUp = () => { if (!pl.pistolUp) { input.pressed.add('Digit1'); idle(1); if (!pl.pistolUp) { input.pressed.add('KeyE'); idle(1); } idle(90); } };
  const calm = () => { pl.lastFireT = -100; foeAlert = false; };
  const carriage = () => `upper ${c.anim.upper} · long ${c.anim.longStance ?? '-'} · stance ${c.anim.stance}`;
  try {
    rifleUp(); calm();
    const lineOpen = !game.col.segmentBlocked([A[0], 1.3, A[2]], chest(mate));
    // (1) the reticle on a teammate
    { FRIENDS.noSnap = !brk; FRIENDS.lowReady = false; aimAt = chest(mate); idle(4); const on = game.aimGuard, snap = game.aimSnap;
      FRIENDS.noSnap = false; idle(4); 
       }
    // (2) a defender is still the reticle's
    { put(foe, FOE, 0); park.push([foe, FOE]); idle(4); aimAt = chest(foe); const open = !game.col.segmentBlocked([c.pos[0], 1.3, c.pos[2]], chest(foe));
      FRIENDS.noSnap = true; idle(4); const on = game.aimGuard;
      FRIENDS.noSnap = false; idle(4); const off = game.aimGuard;
      line(open && on === foe && off === foe, 'the cursor on a defender he has a line to: the reticle takes him, switch on or off', `line open ${open} · on: ${on?.callsign ?? 'none'} · off: ${off?.callsign ?? 'none'}`); }
    // (4) the rifle, a teammate in the line, calm
    { FRIENDS.noSnap = true; FRIENDS.lowReady = !brk; calm(); aimAt = BEYOND; const swing = settle(); idle(20); const low = pl.friendLow, car = carriage();
      const n0 = pl.rifle.shotsFired; msgs.length = 0; pl.fireCd = 0; idle(1, true); idle(5); const fired = pl.rifle.shotsFired - n0;
      line(low && c.anim.upper === 'none' && c.anim.longStance === 'lowLong' && fired === 0 && msgs.some(m => m.includes('is in your line') && m.includes(mate.callsign)),
        'the rifle up, a teammate three metres ahead in its line, nobody in a fight: the low ready, and a click fires nothing and names him', `lowered ${low} · ${car} · rounds on the click ${fired} · the gun came round in ${swing} frames · ${JSON.stringify(msgs.slice(-2))}`); }
    // (4b) the cursor ON him: the free aim lands on the floor beyond him as the lens sees it, the gun's own line passes beside him — it is down all the same
    { calm(); aimAt = [A[0] + 0.5, 1.2, A[2] - 6]; idle(40); const up0 = !pl.friendLow; aimAt = chest(mate); idle(20); const low = pl.friendLow;
      const dx = Math.sin(c.aimYaw), dz = Math.cos(c.aimYaw), h = mate.char.bones.hips ?? mate.char.pos, off = Math.abs((h[0] - c.pos[0]) * dz - (h[2] - c.pos[2]) * dx);
      line(up0 && low && game.aimFriend === mate && game.aimGuard === null, 'the cursor on the teammate\'s body: known to be on him, no snap, and the gun is down though its own line passes beside him', `up before ${up0} · lowered ${low} · under the cursor ${game.aimFriend?.callsign ?? 'nobody'} · the gun's line passes ${off.toFixed(2)} m from his hips`); }
    // (5) the aim turned off him
    { calm(); aimAt = [A[0] + 0.5, 1.2, A[2] - 6]; let up = -1; for (let i = 0; i < 90 && up < 0; i++) { idle(1); if (!pl.friendLow) up = i; } idle(20); const car = carriage();
      const n0 = pl.rifle.shotsFired; pl.fireCd = 0; idle(1, true); idle(3); const fired = pl.rifle.shotsFired - n0; calm();
      line(up >= 0 && c.anim.longStance === 'shoulder' && fired === 1, 'the aim turned off him: the gun is up again a beat after the line clears, and a click fires', `up after ${up} frames · ${car} · rounds on the click ${fired}`); }
    // (6) the pistol
    { pistolUp(); calm(); FRIENDS.lowReady = !brk; aimAt = BEYOND; idle(30); const low = pl.friendLow, car = carriage(), upper = c.anim.upper;
      const n0 = pl.pistol.shotsFired; msgs.length = 0; pl.fireCd = 0; idle(1, true); idle(5); const fired = pl.pistol.shotsFired - n0;
      aimAt = [A[0] + 0.5, 1.2, A[2] - 6]; idle(40); const up = !pl.friendLow, car2 = carriage();
      line(low && upper === 'none' && fired === 0 && up && c.anim.stance !== 'lowReady', 'the pistol in his hand the same: down for the teammate, no round on a click, its own carry back once the line is clear', `lowered ${low} · ${car} · rounds ${fired} · then up ${up} · ${car2}`);
      rifleUp(); calm(); }
    // (7) a wall between
    { let p: number[] | null = null;
      for (const q of [[15, 0, 12.6], [15, 0, 9.5], [9, 0, 17.6], [18, 0, 26], [22, 0, 13]]) if (game.col.segmentBlocked([A[0], 1.3, A[2]], [q[0], 1.2, q[2]])) { p = q; break; }
      if (!p) line(false, 'a wall between them', 'no walled place found from the aisle');
      else { park[0][1] = p; aimAt = [p[0], 1.2, p[2]]; idle(30); const low = pl.friendLow;
        line(!low && c.anim.longStance === 'shoulder', 'a wall between him and the teammate his gun points toward: nobody is being pointed at, the gun stays up', `teammate at ${p} · lowered ${low} · ${carriage()}`);
        park[0][1] = B; aimAt = BEYOND; idle(30); } }
    // (8) in a fight
    { aimAt = BEYOND; calm(); idle(20); const low0 = pl.friendLow;
      pl.lastFireT = game.time; idle(2); const a = pl.friendLow; calm(); idle(20); const back = pl.friendLow;
      foeAlert = true; idle(2); const b = pl.friendLow; foeAlert = false; idle(20);
      line(low0 && !a && back && !b, 'in a fight the gun is his own: up at once when he has just fired, and when a defender is alert, with the teammate still in its line', `calm ${low0} · just fired ${a} · calm again ${back} · a defender alert ${b}`); }
    // (9) a defender in the line
    { calm(); park[0][1] = AWAY; park[1][1] = [AWAY[0] + 1.5, 0, AWAY[2]]; const F2 = [18, 0, 17.6]; park[park.length - 1][1] = F2; aimAt = [F2[0], 1.2, F2[2]]; idle(30); const low = pl.friendLow;
      line(!low && c.anim.longStance === 'shoulder', 'a calm defender in the line, no teammate: the gun stays up (he is what it is up for)', `lowered ${low} · ${carriage()} · reticle on ${game.aimGuard?.callsign ?? 'nobody'}`);
      // (9b) …and a teammate BEYOND him on that line is not what the gun is on: it stays up, and the first shot is his to take
      park[0][1] = [F2[0] + 2.5, 0, F2[2]]; idle(30); const low2 = pl.friendLow, on2 = game.aimGuard, car2 = carriage();
      const n0 = pl.rifle.shotsFired; pl.fireCd = 0; idle(1, true); idle(3); const fired = pl.rifle.shotsFired - n0;
      line(!low2 && on2 === foe && fired === 1, 'a teammate 2.5 m beyond the calm defender on the same line: the reticle and the gun are on the defender, the gun stays up and the click fires', `lowered ${low2} · reticle on ${on2?.callsign ?? 'nobody'} · ${car2} · rounds on the click ${fired}`);
      park[park.length - 1][1] = FOE; park[0][1] = B; park[1][1] = AWAY; calm(); idle(10); }
    // (10) off
    { FRIENDS.lowReady = false; calm(); aimAt = BEYOND; idle(30); 
       }
    // (10b) the cursor ON a fixture's dot with a teammate, then a defender, on the cursor's ray: the defender is not ringed through the teammate, and the light the
    //       reticle had before the ray found him is still the right button's (it was dropped with him: that frame the pistol's OCP did nothing)
    
    // (11) the stands
    { const a = P.FROZEN_HOLDS['FRIENDS.noSnap'], b = P.FROZEN_HOLDS['FRIENDS.lowReady'];
      line(/TPS_FRIENDS=1/.test(a ?? '') && /TPS_MUZZLE=1/.test(b ?? '') && E.STAND_ENV.includes('TPS_FRIENDS') && E.STAND_ENV.includes('TPS_MUZZLE'), 'both switches are rows of FROZEN_HOLDS with their words, and both words are stand words', `${a} · ${b}`); }
    line(!R.bad, 'nobody left the world', R.bad);
  } finally { Object.assign(FRIENDS, was); }
  console.log(all ? '\nfriends: ALL PASS' : '\nfriends: FAIL');
  return all;
}
if (import.meta.main) process.exit((await friendsCheck()) ? 0 : 1);
