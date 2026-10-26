// The scripted player for the tutorial's third mission (levels/tut-kit.json; tools/qa/lesson-run.ts runs it as the page opens it: a tutorial mission is
// always stood up on the shipped stand): what each line asks, through the input only, in the order a new player would do it. He is mortal in his own half:
// the fights are the level's script's (levels/tut-kit.script.ts) and none of them is aimed at a man who does what the line says. In the squad's half he
// gives the orders the lines ask for (the command key tapped on a thing, held for its ring and a digit, the word) and shoots only on the last line.
import { OWNBLOW } from '../../../src/game/ownblow';
import type { Bot } from '../lesson-run';
import { AXIS, JAMB, INNER_X, INNER_Z, INNER_FROM, BAGS_END, GAP, BARRIER, BACK, NEST, LAMP, SHOP_DOOR, STORE_DOORWAY, STUN_AT, ARCH_MOUTH, LAUNCH_FROM, LAST_MEN, BREAKER, BUNK_DOOR, WALL_MARK, FRAG_AT, YARD_FROM, COURT, INSIDE } from '../../levels/tut-kit';

/** is a switch on IN THE GAME THIS RUN PLAYS: on a page the script is its own bundle, with its own copy of the table (the page's is `__knobs`) */
const pageOn = (name: string, here: boolean): boolean => (globalThis as { __knobs?: { registry?: () => Record<string, { on?: boolean }> } }).__knobs?.registry?.()[name]?.on ?? here;
const blowOn = (): boolean => pageOn('OWNBLOW', OWNBLOW.on);

export const query = '?roomclear=1&squad4';
export const ended = (g: any) => g.lesson.finished;
/** the level's fights are its script's, which the tutorial frame starts: on a stand without the frame this lesson cannot be played */
/** the run wears what the mission's page deals (lesson-run.ts): his card as the level names it, one of each can and the launcher on its grenade */
export const pageKit = true;
export const needs = 'rails';
/** THE MISTAKES A LINE WARNS OF (tools/qa/tut-kit-check.ts plays each): TUT_KIT_ARM=stand walks the sandbags standing and TUT_KIT_ARM=nosmoke crosses the
 *  gap with no smoke up (each must put him down); `early` gives the squad's key a tap and a hold in his own half, before the line that hands it to him
 *  (nothing may come of either); `tap` taps the command key on the bunk room's door where the line asks for the hold (the squad must not move); `shoot`
 *  has him clear the wash yard with his rifle where the line asks for the squad's frag (the mission must still end) */
const brk = (): string => (typeof process !== 'undefined' ? process.env.TUT_KIT_ARM : (globalThis as { __TUT_KIT_ARM?: string }).__TUT_KIT_ARM) ?? '';
/** A run off the page's stand has none of the things this mission teaches with: his own charge and his own ring, walls that break, the squad's ring that
 *  stays up for a digit, a teammate who works a thing. The runner raises them for this run alone; the page's stand has them */
export const raises = { TPS_OWNCHARGE: '1', TPS_HANDRING: '1', TPS_TABBACK: '1', PT_DESTR: '1', TPS_QRING: '1', TPS_QSTAY: '1', TPS_WORKTHINGS: '1',
  TPS_RAILFLAT: '1', TPS_QONLY: '1', TPS_RINGFLAT: '1', TPS_SQUADTHROW: '1',   // (the flat ring the level takes, `rails.flatRing`, with the throws on order among its rows: the lines' digits are that ring's)
  TPS_ISSUEWHOLE: '1', TPS_KITTREE: '1', TPS_GUNSLOTS: '1', TPS_BIGSLOT: '1' };   // (and the card the level names for him as his whole kit: without it the frozen stand lends him the built counts over the card, the tube on sponge rounds)
/** lines a fight or the squad may finish as they come up: the grenade took both of the launcher's two, the frag both of the yard's; a man who picked the
 *  charge with the ring's first showing has done the two lines after it */
export const idleOk = ['pistol', 'last', 'tab', 'charge', 'clear', 'lights'];   // ('pistol': his one frag away, the pistol is in his hands already)
/** …and a line inside the squad's own fight, which the room's fall may finish before he has pressed its key */
export const fightOk = ['breach', 'clear'];

/** the right button pressed, held on a world point for `secs` (the arc is up) and let go: Input's bits are 1 left · 2 middle · 4 right */
const rightHold = (B: Bot, p: number[], secs = 0.5) => { B.aim(p); B.step(); B.I.clicked |= 4; for (let i = 0; i < secs * 60; i++) { B.aim(p); B.I.buttons |= 4; B.step(); } B.I.buttons &= ~4; B.aim(p); B.step(3); };
const trigger = (B: Bot, at: () => number[], secs: number, stop?: () => boolean) => {
  B.aim(at()); B.step(); B.I.clicked |= 1;
  try { for (let i = 0; i < secs * 60; i++) { B.aim(at()); B.I.buttons |= 1; if (i % 14 === 13) B.I.clicked |= 1; B.step(); if (stop && stop()) break; } } finally { B.I.buttons &= ~1; B.step(); }   // (a fresh press every quarter second: the pistol fires a round a press)
};
const chest = (x: any): number[] => x.char.bones?.chest ?? [x.char.pos[0], 1.25, x.char.pos[2]];
const up = (x: any) => x.state !== 'dead';
const foes = (B: Bot): any[] => B.g.guards.filter((x: any) => x.team !== 'operative');
const named = (B: Bot, names: string[]): any[] => names.map(n => B.g.guards.find((x: any) => x.callsign === n)).filter(Boolean);
const squad = (B: Bot): any[] => B.g.guards.filter((x: any) => x.unit && x.team === 'operative' && x.state !== 'dead');
const line = (B: Bot, x: any) => { const P = B.g.player.char, a = P.bones?.head ?? [P.pos[0], 1.5, P.pos[2]]; return !B.g.col.shotBlocked(a, chest(x)); };
const gun = (B: Bot) => B.g.player.slot === 'rifle' ? B.g.player.rifle : B.g.player.pistol;
const downed = (B: Bot) => foes(B).filter(x => !up(x)).length;
/** shoot every man he has a line to until `n` are down (reloading as the gun runs dry), for `secs` at most */
const fight = (B: Bot, n: number, secs: number, move?: () => void, who?: any[]) => {
  const P = B.g.player, t0 = B.t();
  while (downed(B) < n && B.t() - t0 < secs && !P.down) {
    const w = gun(B); if (w.roundsReady === 0 && !w.reloading) { B.press('KeyR'); continue; }
    const x = (who ?? foes(B)).filter((y: any) => up(y) && line(B, y)).sort((a: any, b: any) => Math.hypot(a.char.pos[0] - B.pos()[0], a.char.pos[2] - B.pos()[2]) - Math.hypot(b.char.pos[0] - B.pos()[0], b.char.pos[2] - B.pos()[2]))[0];
    if (!x) { if (move) move(); else B.step(); continue; }
    if (P.slot === 'pistol') { for (let i = 0; i < 16 && up(x); i++) { B.aim(chest(x)); B.step(); } B.click(1, chest(x)); B.step(6); }   // (a round a press, once the gun has come round to the cursor: the page's aim has weight)
    else trigger(B, () => chest(x), 1.2, () => !up(x) || !line(B, x));
  }
  return downed(B) >= n;
};
/** a slot's key, and the half second the thing takes to come into the hand (a click before that is not a throw) */
const take = (B: Bot, code: string) => { B.press(code, 5); B.until(() => B.g.player.equipT <= 0, 3); B.step(3); };
/** the can in the air, until it has gone off (or `secs`) */
const landed = (B: Bot, kind: string, secs: number) => { let seen = false; for (let i = 0; i < secs * 60; i++) { const it = B.g.items.items.some((x: any) => x.kind === kind && !x.fired); if (seen && !it) break; seen = seen || it; B.step(); } B.step(20); };
const stepIs = (B: Bot, id: string) => B.g.lesson.step?.id === id;
const inRect = (p: number[], R: { x0: number; x1: number; z0: number; z1: number }) => p[0] > R.x0 && p[0] < R.x1 && p[2] > R.z0 && p[2] < R.z1;
const taken = (B: Bot, id: string) => { const r = B.g.roomClear.rooms.find((x: any) => x.def.id === id); if (!r) return false; if (r.cleared) return true; return squad(B).filter(m => inRect(m.char.pos, r.def.bounds)).length >= 3; };
/** his own ring is up */
const ownRing = (B: Bot) => { const S = B.g.player.sel; return !!S.open && !!S.ring; };
/** the command key tapped on a point */
const command = (B: Bot, p: number[]) => { B.aim(p); B.step(2); B.aim(p); B.press('KeyB', 2); };
/** the command key HELD on a point until the squad's ring is up, then the digits the line that is up names, in order */
const ring = (B: Bot, p: number[]) => {
  B.aim(p); B.step(2); B.aim(p); B.I.pressed.add('KeyB'); B.hold(['KeyB'], 0.45); B.step(2);
  if (!B.g.orders.menu) { B.say('  the squad\'s ring did not come up'); return false; }
  const T = B.g.lesson.step, said = (B.g.orders.menu.flat ? T?.flatLine : T?.menuLine) ?? '', digits = [...said.matchAll(/\[(\d)\]/g)].map(m => m[1]);
  B.say(`  the ring is up: '${said}' → ${digits.join(', ') || 'no digit named'}`);
  const rows = () => { const M = B.g.orders.menu; return M ? `${M.target?.kind ?? '?'}: ${M.rows.map((r: any, k: number) => `${k + 1} ${r.id}${M.why?.[k] ? ` (${M.why[k]})` : ''}`).join(' · ')}` : 'no ring'; };
  B.say(`    ${rows()}`);
  for (const d of digits) { B.press('Digit' + d, 4); B.say(`    [${d}] → ${rows()}`); }
  return true;
};
/** go after whoever is still up until `n` are down: toward the nearest of them until there is a line, then the fight's own aim and trigger */
const hunt = (B: Bot, n: number, secs: number) => fight(B, n, secs, () => { const x = foes(B).find(up); if (!x) { B.step(10); return; } const p = x.char.pos; B.go([p[0], 0, p[2]], 1.5, { stop: () => line(B, x) }); });
const orderOf = (B: Bot) => squad(B).map(m => `${m.callsign} ${B.g.orders.of(m)?.order ?? '-'}`).join(' · ');

export const steps: Record<string, (B: Bot) => void> = {
  // ---- his own kit, alone ----
  // the four stay at the truck: he walks in by the gate to the wooden door
  gate: B => {
    if (brk() === 'early') { const at = [B.pos()[0] + 2, 0.02, B.pos()[2] - 2]; command(B, at); B.aim(at); B.I.pressed.add('KeyB'); B.hold(['KeyB'], 0.5); B.step(4);
      if (B.g.orders.menu || B.g.orders.cmdRing) B.fail('early: the squad\'s ring came up before the line that hands him the command key'); B.wait(1); }
    B.go(INNER_FROM, 30);
    const far = squad(B).filter(m => m.char.pos[0] > 9.2); if (far.length) B.fail(`${far.map(m => m.callsign).join(', ')} followed him in through the gate: the four wait at the truck until he calls them (${orderOf(B)})`);
  },
  // a TAP of the interaction key, and the tap's row is the door's
  open: B => {
    const c = [INNER_X, 1.0, INNER_Z]; for (let i = 0; i < 20; i++) { B.aim(c); B.step(); }
    const top = B.g.nearby.find((r: any) => !!r.verb); if (top?.verb !== 'push') B.fail(`a tap at the wooden door gives '${top ? `${top.kind} ▸ ${top.verb}` : 'nothing'}', not the door opened`);
    B.press('Space', 3); if (!B.until(() => !B.g.doors.byName('inner')!.isClosed(), 5)) B.fail('a tap of the interaction key did not open the wooden door');
  },
  // through the doorway and up beside the blue gate; then down, as the gun starts along the sandbags
  arch: B => { B.go([INNER_X + 1.0, 0, INNER_Z], 20); B.go(JAMB, 20); },
  low: B => {
    if (brk() === 'stand') { B.go([16.9, 0, 13.3], 12, { stop: () => B.g.player.down }); B.go(BAGS_END, 15, { stop: () => B.g.player.down }); B.wait(2.5); if (!B.g.player.down) B.fail('BREAK stand: he walked the sandbags standing and was not hit'); return; }
    B.wait(1.2); B.crouch(true); B.go([16.9, 0, 13.3], 12); B.go(BAGS_END, 15);
  },
  smoke: B => {
    if (brk() === 'nosmoke') { B.go(BARRIER, 10, { near: 0.5, stop: () => B.g.player.down }); B.wait(2.5); if (!B.g.player.down) B.fail('BREAK nosmoke: he crossed the gap with no smoke up and was not hit'); return; }
    take(B, 'Digit3'); B.click(1, GAP); landed(B, 'smoke', 4); B.wait(2.8);
  },
  // when the plume is up: across the gap, behind the barrier
  cross: B => { B.go(BARRIER, 10, { near: 0.5 }); },
  // over the barrier and into the nest; a second one if the first fell short
  frag: B => {
    B.crouch(false);
    for (let i = 0; i < 2 && downed(B) < 1; i++) { take(B, 'Digit5'); B.click(1, NEST); landed(B, 'frag', 4.5); B.until(() => downed(B) >= 1, 4); }   // (the fuse runs on after it lands)
    if (downed(B) < 1) { const x = named(B, ['Novak'])[0]; take(B, 'Digit2'); B.go([24.9, 0, 11.6], 10, { stop: () => !!x && line(B, x) }); fight(B, 1, 20); }
  },
  pistol: B => { take(B, 'Digit1'); },
  // the lamp on its pole by the shop's front, from behind the barrier: a round a press until it is dark
  lamp: B => {
    const T = B.g.targets.find((t: any) => t.name === 'lamp_shop'); if (!T) B.fail('no lamp_shop among the light targets');
    const lit = () => !!T && T.on && !T.broken && T.factor > 0.05;
    for (let i = 0; i < 10 && lit(); i++) { for (let k = 0; k < 20; k++) { B.aim(LAMP); B.step(); } B.click(1, LAMP); B.step(12); if (gun(B).roundsReady === 0) { B.press('KeyR'); B.until(() => !gun(B).reloading, 5); } }
    if (lit()) B.fail('ten pistol rounds at the lamp and it is still lit');
  },
  // the interaction key HELD on the locked door: his ring, with the one row the level lets this door offer
  ring: B => {
    const g = B.g, c = [SHOP_DOOR[0], 1.0, SHOP_DOOR[2]];
    B.go([24.2, 0, AXIS + 0.2], 10);
    for (let i = 0; i < 30; i++) { B.aim(c); B.step(); }
    const rows = g.nearby.filter((r: any) => r.kind === 'door').map((r: any) => r.verb);
    B.say(`  the door's rows: ${rows.join(' ')}`);
    if (rows.join(' ') !== 'charge') B.fail(`the shop's door offers '${rows.join(' ')}', not the charge alone`);
    B.I.pressed.add('Space'); B.hold(['Space'], 0.5, () => ownRing(B)); B.step(3);
    if (!ownRing(B)) B.fail('the interaction key held on the locked door put no ring up');
  },
  tab: B => { if (!ownRing(B)) B.fail('his ring came down by itself before the line that asks for Tab'); B.press('Tab', 6); if (ownRing(B)) B.fail('Tab did not put his ring away'); if (B.g.doorTools.set.length) B.fail('Tab set the charge'); },
  charge: B => {
    const g = B.g, c = [SHOP_DOOR[0], 1.0, SHOP_DOOR[2]];
    if (!B.verb('door', 'charge', c)) B.fail(`no 'charge' row on the shop's door`);
    B.until(() => !g.player.charging && !g.player.kick, 12); B.step(20);
  },
  back: B => { B.go(BACK, 10, { near: 0.5 }); B.step(20); },
  // (the page's way: his own charge is a row of his own list — ownblow.ts — and stood clear it is the only one, so a tap gives it; the frozen stand's: the squad's key)
  blow: B => {
    const T = B.g.doorTools, c = T.set[0]; if (c && /too close:[^·]*\byou\b/.test(T.closeWords(c))) B.fail('the mark that says stand clear is inside his own charge\'s backblast');
    B.press(blowOn() ? 'Space' : 'KeyF', 30); },
  // out from beside the door, square to the doorway (from its edge the rounds go into the jamb), and the pistol
  shop: B => {
    B.go([23.5, 0, AXIS + 0.05], 6);
    if (!fight(B, 2, 25, () => { B.step(6); }, named(B, ['Tango 1']))) B.fail('the man in the shop did not drop');
  },
  // from the shop's floor, the stun grenade through the doorway to the store, and a step aside (its card says why)
  stun: B => { const pl = B.g.player, blind = () => pl.dazzledUntil > B.g.time;
    B.go([26.8, 0, AXIS], 10); take(B, 'Digit4'); B.click(1, STUN_AT);
    B.until(() => B.g.items.items.some((x: any) => x.kind === 'flash' && !x.fired), 2); B.go([26.8, 0, AXIS + 1.5], 4);   // (once it has left his hand: the cursor is the throw's aim until it does)
    landed(B, 'flash', 3.4);
    if (blind()) B.fail('he stepped aside as the line says and his own stun grenade still blinded him'); },
  // the pistol, and INTO the room (a gun in a doorway has no room to come up across the jamb)
  store: B => {
    take(B, 'Digit1'); B.go([STORE_DOORWAY[0], 0, AXIS], 6);
    if (!fight(B, 4, 30, () => { B.nudge(STORE_DOORWAY, 0.3); })) B.fail('the two in the store did not drop');
  },
  rifle: B => { take(B, 'Digit2'); },
  // back out through the shop and the gun court to the arch's mouth
  leave: B => { const d = B.g.doors.byName('store')!; if (!d.locked || Math.abs(d.angle) > 0.05) B.fail('the store\'s back door is open or unlocked before the two for the launcher come by it'); B.go([LAUNCH_FROM[0], 0, LAUNCH_FROM[2]], 40, { stop: () => !stepIs(B, 'leave') }); },   // (he stops as the line changes, as a person does: the ring's near edge, the closest the line lets him be to the doorway)
  // the line that only has him look: it goes by once the two stand in the shop's doorway, and not before
  behind: B => {
    B.until(() => !stepIs(B, 'behind'), 9); if (stepIs(B, 'behind')) return;
    const men = named(B, ['Brandt', 'Marsh']).filter(up), off = men.filter((x: any, k: number) => Math.hypot(x.char.pos[0] - LAST_MEN[k][0], x.char.pos[2] - LAST_MEN[k][2]) > 0.8);
    if (men.length < 2 || off.length) B.fail(`the shot is asked for before the two stand in the doorway (${men.length} up, ${off.length} still on the way)`);
  },
  // the two in the shop's doorway behind him: the grenade into it, once both stand there (8.5 m and more: armed)
  launch: B => {
    const men = () => named(B, ['Brandt', 'Marsh']);
    B.until(() => men().length >= 2 && men().every((x: any, k: number) => !up(x) || Math.hypot(x.char.pos[0] - LAST_MEN[k][0], x.char.pos[2] - LAST_MEN[k][2]) < 0.8), 12);
    const x = men().find(up); if (!x) B.fail('nobody came out through the shop');
    if (!(Math.abs(B.g.doors.byName('store')!.angle) > 1)) B.fail('the two are in the doorway and the store\'s back door is not open: they did not come by it');
    if (B.g.player.rifle.attachment !== 'he40') B.fail(`the launcher is on '${B.g.player.rifle.attachment}': the lesson has no line that swaps it`);
    B.wait(0.5); rightHold(B, chest(x)); B.wait(1.5);
    if (B.g.messages.some((m: any) => /too close to arm|was a dud/.test(m.text))) B.fail('the launcher\'s grenade was a dud from where the line before stood him: the doorway is inside the 8 m it needs to arm');
  },
  // whoever the grenade left: the rifle, from the arch, and after him if he will not come
  last: B => {
    fight(B, 6, 60, () => { const x = foes(B).find(y => up(y) && !y.script); if (!x) { B.step(10); return; } const p = x.char.pos; if (Math.hypot(p[0] - B.pos()[0], p[2] - B.pos()[2]) > 14) B.go([p[0], 0, p[2]], 1.5, { stop: () => line(B, x) }); else B.step(10); }, named(B, ['Brandt', 'Marsh']));
  },

  // ---- the squad's kit, on his word ----
  // a tap of the command key on himself: the four come up from the truck, at a run
  call: B => {
    const home = squad(B).filter(m => m.char.pos[0] < 9.2).length; B.say(`  ${home} of ${squad(B).length} still at the truck as he calls them`);
    if (home < squad(B).length) B.fail(`the four did not wait at the truck (${orderOf(B)})`);
    B.go([ARCH_MOUTH[0] + 1.6, 0, AXIS], 10);
    const me = B.pos(); command(B, [me[0], 1.0, me[2]]);
    if (!B.until(() => squad(B).filter(m => inRect(m.char.pos, INSIDE)).length >= Math.min(3, squad(B).length), 60)) B.fail(`the four did not come up to him (${orderOf(B)}) · they are at ${squad(B).map(m => `(${m.char.pos[0].toFixed(1)}, ${m.char.pos[2].toFixed(1)})${m.char.crouch ? ' low' : ''}`).join(' ')} · he is at (${B.pos()[0].toFixed(1)}, ${B.pos()[2].toFixed(1)})`);
    B.until(() => !stepIs(B, 'call'), 6);   // (the line holds a moment once they are in)
  },
  // (the line says a buddy team goes: one man works the box and the other of his pair covers him from behind)
  breaker: B => { for (let k = 0; k < 3 && !B.g.blackout.active; k++) { command(B, BREAKER);
      const O = B.g.orders, worker = () => squad(B).find(m => O.of(m)?.order === 'work'); B.until(() => !!worker() || B.g.blackout.active, 1);
      const w = worker(); if (w) B.say(`  the breaker box: ${w.callsign} works it`);
      B.until(() => B.g.blackout.active, 30); }
    if (!B.g.blackout.active) B.fail(`a tap of the command key on the breaker box did not put the lights out (${orderOf(B)})`); },
  nv: B => { B.press('KeyN', 10); },
  // the command key HELD on the bunk room's door, and the wedge the line names; then the word, if they wait on it
  door: B => {
    B.go([27.2, 0, AXIS], 30);
    if (brk() === 'tap') { command(B, BUNK_DOOR); B.wait(1.5); const on = squad(B).filter(m => B.g.orders.of(m)?.order === 'stack').length; if (on) B.fail(`tap: a tap of the command key on the door sent ${on} to stack on it, on the line that teaches the hold`); }
    for (let k = 0; k < 3 && !taken(B, 'bunk'); k++) {
      if (!ring(B, BUNK_DOOR)) continue;
      B.say(`  after the ring: ${orderOf(B)}`);
      B.until(() => taken(B, 'bunk') || squad(B).every(m => B.g.orders.of(m)?.arrived), 25); B.wait(1);
      if (!taken(B, 'bunk')) B.press('KeyF', 2);
      B.until(() => taken(B, 'bunk'), 30); B.wait(2);
    }
    if (!taken(B, 'bunk')) B.fail(`the bunk room is not theirs (${orderOf(B)})`);
  },
  // the ring on the wall between the two rooms: its second ring has the charge. A charge already on the wall is left to the word, never asked for again
  wall: B => { const set = () => (B.g.wallCharge?.set.length ?? 0) > 0, clear = () => set() && !B.g.wallCharge.tooClose(B.g.wallCharge.set[0]).length;
    B.go([28.0, 0, 8.6], 30);
    for (let k = 0; k < 3 && !set() && !taken(B, 'fuel'); k++) { ring(B, WALL_MARK); B.until(set, 20); }
    if (!set() && !taken(B, 'fuel')) B.fail(`no charge went on the wall (${orderOf(B)})`);
    B.go([27.4, 0, 12.4], 20);   // (he stands off too: out through the door, into the shop)
    B.until(clear, 20); B.wait(0.6); },
  breach: B => { B.wait(1); B.press('KeyF', 2); if (!B.until(() => taken(B, 'fuel'), 40)) B.fail(`the fuel room is not theirs (${orderOf(B)})`); B.wait(2); },
  // a tap on himself, and out through the shop to the court outside the wash yard's gateway
  lead: B => { const me = B.pos(); command(B, [me[0], 1.0, me[2]]); B.go([YARD_FROM[0], 0, YARD_FROM[2]], 40); B.until(() => !stepIs(B, 'lead'), 30); },
  lights: B => { B.until(() => !stepIs(B, 'lights'), 14); },
  // the ring on the floor behind the sandbags: its second ring has the frag
  throw: B => {
    const men = () => named(B, ['Voss', 'Kerr']);
    { const lit = men().filter(up), dark = lit.filter((x: any) => !x.flashlight.enabled); if (lit.length && dark.length === lit.length) B.fail(`the line says he can see their flashlights and ${dark.map((x: any) => x.callsign).join(' and ')} burn${dark.length === 1 ? 's' : ''} none`); }
    if (brk() === 'shoot') { B.go([26.4, 0, 22.0], 20); hunt(B, 11, 60); return; }   // (round the bags' south end, and after whoever is left)
    for (let k = 0; k < 2 && men().some(up); k++) {
      const n0 = B.g.doorTools.frags;
      if (!ring(B, FRAG_AT)) continue;
      B.say(`  after the ring: ${orderOf(B)} · the squad's frags ${n0} → ${B.g.doorTools.frags}`);
      landed(B, 'frag', 8); B.until(() => !men().some(up), 5);
    }
  },
  clear: B => { hunt(B, 11, 60); },
};
