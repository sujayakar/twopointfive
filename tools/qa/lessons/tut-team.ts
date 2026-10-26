// The scripted player for the squad's tutorial mission on the ground (levels/tut-team.json; tools/qa/lesson-run.ts runs it as the page opens it: a tutorial
// mission is always stood up on the shipped stand). He is a man on the floor: his own feet, the view's zoom, the squad's key on a point under the cursor
// (the ground, a door, himself) and the go key. He never fires: what the squad does with one order and one word is what the run is about.
import type { Bot } from '../lesson-run';

export const query = '?roomclear=1&squad4';
export const ended = (g: any) => g.lesson.finished;
/** the roadblock can be down before he is through the gate: then its step is done as it comes up */
export const idleOk = ['gate'];
/** TUTTEAM_GOD=1: rounds do not drop him (a run that is about the steps alone). Without it he is as mortal as a player, which is what the run is for */
export const mode = { god: process.env.TUTTEAM_GOD === '1' };
/** on the frozen stand (lesson-gate's own run of it): a door the squad opened stays open. He has no key for a door in this mission (that is the next
 *  one's), so the way into a room they took is the door as they left it, which is how the page has it */
export const raises = { QA_DOORSTAY: '1' };

const YARD = [11.4, 0.02, 16.4], TEA_DOOR = [9.0, 1.0, 11.7], KITCHEN_DOOR = [13.8, 1.0, 6.25], BUNK_DOOR = [28.8, 1.0, 16.25], TRUCKS = [6.1, 0, 23.6];
/** inside the yard's wall beside the gate, out of the gateway's line · inside the tea room, in sight of the kitchen's door · beside the bunkhouse, short of its door */
const BY_GATE = [10.4, 0, 18.6], IN_TEA = [10.4, 0, 8.6], BY_BUNK = [26.2, 0, 17.5];
const BLOCK = /^Tango (6|7|8)$/;
const squad = (B: Bot) => B.g.guards.filter((x: any) => x.unit && x.team === 'operative' && x.state !== 'dead');
const taken = (B: Bot, id: string) => { const r = B.g.roomClear.rooms.find((x: any) => x.def.id === id); if (!r) throw new Error(`the mission has no room '${id}'`); if (r.cleared) return true; const b = r.def.bounds; return squad(B).filter((m: any) => m.char.pos[0] > b.x0 && m.char.pos[0] < b.x1 && m.char.pos[2] > b.z0 && m.char.pos[2] < b.z1).length >= 3; };
const near = (B: Bot, p: number[], r: number) => squad(B).every((m: any) => Math.hypot(m.char.pos[0] - p[0], m.char.pos[2] - p[2]) < r);
const stacked = (B: Bot) => squad(B).every((m: any) => { const o = B.g.orders.of(m); return o?.order === 'stack' && o.arrived; });
const following = (B: Bot) => squad(B).every((m: any) => B.g.orders.of(m)?.order === 'fallIn');
const last = (B: Bot) => { const l = B.g.orders.log; return l.length ? l[l.length - 1] : null; };
const where = (B: Bot) => squad(B).map((m: any) => `${m.callsign} (${m.char.pos[0].toFixed(1)},${m.char.pos[2].toFixed(1)}) ${B.g.orders.of(m)?.order ?? '-'}`).join(' · ');
/** the squad's key on a point */
function command(B: Bot, p: number[]) { B.aim(p); B.step(2); B.aim(p); B.press('KeyB', 2); }
/** the squad's key on himself: they fall in. Asked of the order book: a tap that gave anything else is a line that lies */
function callIn(B: Bot) {
  for (let k = 0; k < 3 && !following(B); k++) {
    const p = B.pos(); command(B, [p[0], 1.0, p[2]]); B.step(4);
    // (beside a doorway the cursor on himself can be on the door as drawn, and the tap is then that door's: what a hand does is step off it and tap again)
    if (!following(B) && last(B)?.order === 'stack') { B.say(`  the squad's key on himself at (${p[0].toFixed(1)}, ${p[2].toFixed(1)}) gave the door beside him (${last(B).target}): he steps off it and taps again`); B.go([p[0] - 2.2, 0, p[2] + 0.6], 12); }
  }
  if (!following(B)) B.fail(`the squad's key on himself gave ${last(B) ? `${last(B).order} on ${last(B).target}` : 'nothing'}: ${where(B)}`);
}
/** stack on a door, then the go key once they stand at it. The squad's key is tapped on the door only while they are NOT stacked on it (a tap on a door
 *  they stand on is no way in to lean on), and the room has to fall to the go key: a go key that does nothing fails the run */
function breach(B: Bot, door: number[], room: string) {
  for (let k = 0; k < 3 && !stacked(B); k++) { command(B, door); B.until(() => stacked(B), 25); }
  if (!stacked(B)) B.fail(`the squad's key on the door of '${room}' did not stack them: ${where(B)}`);
  B.wait(1.5);
  for (let k = 0; k < 2 && !taken(B, room); k++) { B.press('KeyF', 2); B.until(() => taken(B, room), 30); }
  if (!taken(B, room)) B.fail(`the go key did not take '${room}': ${where(B)}`);
  B.wait(2);
}

export const steps: Record<string, (B: Bot) => void> = {
  zoom: B => { for (let i = 0; i < 20; i++) { if (!B.g.rails.locks('zoom')) B.cam.zoom(60); B.step(); } B.wait(1); },
  // nothing to press: the two cans are called, thrown and burning before the line goes by
  smoke: B => { B.wait(4.2); },
  move: B => { command(B, YARD); if (!B.until(() => squad(B).every((m: any) => m.char.pos[2] < 19.7 && m.char.pos[0] > 4.4), 45)) { command(B, YARD); B.until(() => squad(B).every((m: any) => m.char.pos[2] < 19.7), 30); } B.wait(1); },
  with: B => { if (!B.go(BY_GATE, 40)) B.fail(`he did not reach the yard from the road: he is at ${B.pos().map(v => v.toFixed(1)).join(', ')}`); },
  gate: B => { B.until(() => !B.g.guards.some((x: any) => BLOCK.test(x.callsign) && x.state !== 'dead'), 70); B.wait(0.9); },
  stack: B => { for (let k = 0; k < 3; k++) { command(B, TEA_DOOR); if (B.until(() => stacked(B) || near(B, TEA_DOOR, 3.2), 30)) break; B.say('  not at the door: ' + where(B)); }
    B.until(() => stacked(B), 30); B.wait(0.5); },
  go: B => { for (let k = 0; k < 2 && !taken(B, 'tea'); k++) { B.press('KeyF', 2); B.until(() => taken(B, 'tea'), 30); } if (!taken(B, 'tea')) B.fail(`the go key did not take the tea room: ${where(B)}`); B.wait(2); },
  // (the kitchen's door is inside the tea room: he walks in to point at it, as a player does)
  kitchen: B => { if (!B.go(IN_TEA, 40)) B.fail(`he did not reach the tea room: he is at ${B.pos().map(v => v.toFixed(1)).join(', ')}`); breach(B, KITCHEN_DOOR, 'kitchen'); },
  fallin: B => { callIn(B); B.wait(1); },
  lead: B => { if (!B.go(BY_BUNK, 60)) B.fail(`he did not reach the bunkhouse: he is at ${B.pos().map(v => v.toFixed(1)).join(', ')}`);
    if (!B.until(() => squad(B).filter((m: any) => m.char.pos[0] > 19 && m.char.pos[0] < 28.9 && m.char.pos[2] > 11.7 && m.char.pos[2] < 19.8).length >= Math.min(3, squad(B).length), 40)) B.say('  not with him: ' + where(B)); },
  bunk: B => { breach(B, BUNK_DOOR, 'bunk'); },
  out: B => { callIn(B); if (!B.go(TRUCKS, 80)) B.fail(`he did not reach the trucks: he is at ${B.pos().map(v => v.toFixed(1)).join(', ')}`);
    if (!B.until(() => squad(B).filter((m: any) => m.char.pos[0] < 12.5 && m.char.pos[2] > 20.1).length >= Math.min(3, squad(B).length), 60)) B.say('  not back with him: ' + where(B)); B.wait(1.2); },
};
