// THE BREAK LAB — a place to set bursts off beside every kind of thing and watch how each breaks. The page opened with ?breaklab (the dev server's /breaklab:
// the kit's showroom yard, every piece of the kit stood in it once, indoors and out; /breaklab?levelFile=pass.json for the pass) gets its own system in the
// panel, open from the first frame:
//   the one index of every thing and the one set of rings for every burst (game/things.ts) as two switches, to see the build as it was beside them ·
//   the rings' radii and a grenade's size as live sliders, the build's numbers back by a button ·
//   a hand grenade, a door charge and a wall charge as three buttons that ARM a burst (one at a time, the grenade to begin with): while one is armed a
//   click on the view sets it off where the cursor is, and the click is the lab's, not the gun's. Pressed again the button lets go and the click is the
//   game's once more. EACH IS THE GAME'S OWN BLAST, whole (labBurst below): the grenade is frag.ts's blast (flash, report, sparks, scorch, glass, men, and
//   its one call into the destruction engine); a wall charge clicked on a wall is SET on that wall, on the player's side of it, and blown by the wall
//   charge's own trigger (an outer wall is taken on by the engine and holed, the pall, the mess, the report); a door charge clicked at a door is set on
//   that door and blown by the door charge's own trigger (the leaf off its hinges). Clicked in the open, a charge is its blast with the report and flash
//   it borrows, and nothing to be set on. The level put back as it began ·
//   what the level under you is made of: how many things each engine breaks, and what stays and why ·
//   'they can't notice you' and 'nothing hurts you' (both on to begin with: the lab is for the things)
// Nothing here is the game's: it writes the table the game reads, calls the one blast entry point every charge and grenade calls, and sets two of the
// sandbox's flags. Without the word none of it is made.
import type { Game } from '../game/game';
import type { Panel } from '../ui/panel';
import type { Vec3 } from '../math/vec';
import type { Input } from '../game/input';
import { THINGS, ringsOn, ringsOf, burstSize } from '../game/things';
import { FURN_STATS } from '../game/furnblast';
import { FRAG, blast as fragBlast } from '../game/frag';
import { detonateStun } from '../game/combat';
import { WALLCHARGE, wallAt, wallChargeOn } from '../game/wallcharge';
import type { WallSpot } from '../game/wallcharge';
import { offOf } from '../game/doortools';
import { DOORBLAST } from '../game/doorblast';
import { WALL_STATS } from '../game/wallbreak';
import { DESTRUCTION } from '../game/destructionSwitch';
import { PLAYER_ID } from '../game/consts';

export type Armed = '' | 'grenade' | 'door charge' | 'wall charge';
export type Burst = Exclude<Armed, ''>;
/** wallM / doorM: how near the cursor a wall's footprint / a doorway's middle must be for a charge to be set on it (the game's own pick is the cursor ON the wall) */
export const BREAKLAB = { on: false, unseen: true, safe: true, armed: 'grenade' as Armed, wallM: 0.8, doorM: 1.3 };
const BURSTS: Record<Burst, { title: string; opts: () => { radius?: number; power?: number; chaos?: boolean } }> = {
  'grenade': { title: 'a real hand grenade where the cursor is: its flash, report, sparks and scorch, what it breaks, and what it does to anyone near (you too, with \'nothing hurts you\' off)', opts: () => ({ radius: FRAG.destrM, power: 1 }) },
  'door charge': { title: 'a real door charge: clicked at a door it is set on that door and blown, and the leaf comes off. Clicked in the open it is the charge\'s blast alone', opts: () => ({ chaos: false, radius: 1.0 }) },
  'wall charge': { title: 'a real wall charge: clicked on a wall it is set on that wall, on your side of it, and blown, and the wall is holed (an outer wall too). Clicked in the open it is the charge\'s blast alone', opts: () => ({ chaos: false }) },
};
/** the report, flash, scorch and what it knocks about that a door or a wall charge borrows from the stun charge, with nobody on its list (doortools.ts and
 *  wallcharge.ts blowRaw do exactly this; here for a charge with nothing to be set on) */
function borrowedBang(g: Game, at: Vec3) {
  const it = g.items.spawn('flash', at, [0, 0, 0], { fuse: 99, who: PLAYER_ID }); it.fired = true;
  const all = g.guards, last = g.messages[g.messages.length - 1]; g.guards = []; try { detonateStun(g, it, true); } finally { g.guards = all; }
  const m = g.messages[g.messages.length - 1]; if (m && m !== last && /flashbang/.test(m.text)) g.messages.pop();
  const ii = g.items.items.indexOf(it); if (ii >= 0) g.items.items.splice(ii, 1);
}
/** ONE BURST OF THE LAB at `a` (the cursor's point), as the game itself sets it off. Returns the line the panel shows: how it was set off, what it broke.
 *  No DOM: the panel's click and a headless check call the same thing. */
export function labBurst(g: Game, kind: Burst, a: Vec3): string {
  const f = FURN_STATS, n0 = f.statics + f.razed + f.items + f.propsBroken, s0 = f.shaken, h0 = f.held, t0 = f.propsThrown;
  const w0 = WALL_STATS.holes + WALL_STATS.ports, c0 = WALL_STATS.coverCells, off = () => g.doors.list.filter(d => d.off).length, d0 = off();
  const y = Math.max(a[1], 0), sam = g.player.char.pos; let how = '';
  if (kind === 'grenade') { fragBlast(g, [a[0], y + 0.05, a[2]], { who: PLAYER_ID, name: 'frag grenade', destr: { radius: FRAG.destrM, power: 1 } }); how = 'a hand grenade lying where you clicked'; }
  else if (kind === 'wall charge') {
    const pick = WALLCHARGE.pickM; let W: WallSpot | null = null; WALLCHARGE.pickM = BREAKLAB.wallM; try { W = wallAt(g, [a[0], 0, a[2]], sam); } finally { WALLCHARGE.pickM = pick; }
    const C = g.wallCharge;
    if (W && W.ok && C && wallChargeOn(g)) { const c = { wall: W, t: g.time, by: 'the lab', paid: 0, free: true }; C.set.push(c); const r = C.blow(c); how = `set on the ${W.outer ? 'outer wall' : 'wall'} you clicked, on your side of it: ${r?.hole ? 'it is holed' : 'the wall held'}`; }
    else { const p: Vec3 = [a[0], Math.max(y, 0.3), a[2]]; g.destr.blast(p, { who: PLAYER_ID, chaos: false }); borrowedBang(g, p);
      how = W ? `not set: ${W.why || 'wall charges are off in this build'} Its blast alone` : 'no wall under the cursor, so its blast alone. Click on a wall and it is set on that wall and holes it'; }
  } else {
    const D = g.doorTools, d = g.doors.list.filter(x => !x.off && Math.hypot(x.frameCentre[0] - a[0], x.frameCentre[2] - a[2]) <= BREAKLAB.doorM).sort((p, q) => Math.hypot(p.frameCentre[0] - a[0], p.frameCentre[2] - a[2]) - Math.hypot(q.frameCentre[0] - a[0], q.frameCentre[2] - a[2]))[0];
    if (d && D && !D.chargeOn(d)) { const c = { door: d, side: (offOf(d, sam, 1) >= 0 ? 1 : -1) as 1 | -1, t: g.time, by: 'the lab' }; D.set.push(c); D.blow(c); how = `set on the door you clicked, on your side of it: ${d.off ? 'the leaf is off' : 'the leaf stayed on its hinges'}`; }
    else { const p: Vec3 = [a[0], Math.max(y, 0.3), a[2]]; if (DOORBLAST.on && DOORBLAST.chargeR > 0) g.destr.blast(p, { radius: DOORBLAST.chargeR, who: PLAYER_ID, chaos: false }); borrowedBang(g, p);
      how = 'no door under the cursor, so its blast alone. Click at a door and it is set on that door and takes the leaf off'; }
  }
  const walls = WALL_STATS.holes + WALL_STATS.ports - w0, cover = WALL_STATS.coverCells - c0, doors = off() - d0, Q = ringsOn() ? ringsOf(burstSize(BURSTS[kind].opts())) : null;
  return `${kind}, ${how}.\n${f.statics + f.razed + f.items + f.propsBroken - n0} parts broke, ${f.propsThrown - t0} loose things thrown, ${f.shaken - s0} small things fell, ${f.held - h0} stone things held, ${walls} wall opening${walls === 1 ? '' : 's'}, ${cover} cover cell${cover === 1 ? '' : 's'}, ${doors} door${doors === 1 ? '' : 's'} off` +
    (Q ? `\nits rings: contact ${Q.contact.toFixed(1)}, shatter ${Q.shatter.toFixed(1)}, throw ${Q.throw.toFixed(1)}, shake ${Q.shake.toFixed(1)} m` : '\neach engine by its own reach');
}
let setOff: (() => void) | null = null;   // (the armed burst at the cursor: made by the boot, called by the frame on a click)
const HOW = (): string => BREAKLAB.armed ? `Click anywhere on the view to set off a ${BREAKLAB.armed} there${BREAKLAB.armed === 'wall charge' ? ' (on a wall, to hole it)' : BREAKLAB.armed === 'door charge' ? ' (at a door, to take it off)' : ''}. Press its button again to get your own click back.` : 'No burst is armed, so a click is your own. Press a burst to arm it.';
export const BREAKLAB_SYSTEM = 'Break lab';
const RINGS = ['contact', 'shatter', 'throw', 'shake'] as const;
const TIP: Record<typeof RINGS[number], string> = {
  contact: 'what the burst lies against: a wall is holed from the floor up, stone and concrete things break',
  shatter: 'a thing breaks into its material\'s pieces: a wall loses its upper half, cover its cells, a door its hinges',
  throw: 'a thing comes apart at its joints or goes over whole, loose things fly',
  shake: 'small things fall off what they rest on, loose things are nudged',
};

/** is this page the lab (the word on its address) */
export const breakLabAsked = (q: URLSearchParams): boolean => q.has('breaklab');

/** ONCE, after the panel is built: the lab's system, its rows, the panel open on it */
export function breakLabBoot(game: Game, panel: Panel) {
  BREAKLAB.on = true;
  const was = RINGS.map(k => THINGS.rings[k]), frag0 = THINGS.rings.frag; let made = '', last = '';
  const row = (o: Record<string, number>) => Object.entries(o).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${v}`).join(', ');
  panel.system(BREAKLAB_SYSTEM, { icon: '✸', short: 'break', title: 'set bursts off beside every kind of thing and watch how each breaks' });
  setOff = () => { const k = BREAKLAB.armed; if (k) { last = labBurst(game, k, game.aimPoint); panel.refresh(); } };
  const now = panel.section('Bursts at the cursor', { open: true });
  const kinds = Object.keys(BURSTS) as Exclude<Armed, ''>[];
  const armRow = panel.buttons(now, 'a click sets off', kinds.map(k => ({ text: k, run: () => { BREAKLAB.armed = BREAKLAB.armed === k ? '' : k; }, title: `${BURSTS[k].title}. Press to arm it: every click on the view then sets one off where the cursor is. Press again to let go.` })));
  const armEls = [...armRow.querySelectorAll('button')];
  const lit = () => { armEls.forEach((el, i) => { const on = BREAKLAB.armed === kinds[i]; el.setAttribute('aria-pressed', String(on)); el.style.background = on ? '#c8a03c' : ''; el.style.color = on ? '#111' : ''; el.style.fontWeight = on ? '600' : ''; }); };
  lit(); panel.keep(lit, armRow);
  panel.text(now, HOW);
  panel.buttons(now, 'the level', [
    { text: 'put it all back', run: () => { const p = game.player.char.pos, at: Vec3 = [p[0], 0, p[2]]; game.restartEncounter(); game.teleportPlayer(at); made = ''; last = 'The level is back as it began: walls, doors, glass, furniture, loose things and lights.'; }, title: 'the level as it began (a restart of the encounter), with you left where you stand' }]);
  panel.text(now, () => last);
  panel.toggleBox(now, 'they can\'t notice you', () => BREAKLAB.unseen, v => { BREAKLAB.unseen = v; }, 'on: nobody sees or hears you. Off: the real guards');
  panel.toggleBox(now, 'nothing hurts you', () => BREAKLAB.safe, v => { BREAKLAB.safe = v; }, 'god mode while the lab is open');
  const one = panel.section('One treatment', { open: true });
  panel.toggleBox(one, 'one index of every thing', () => THINGS.on, v => { THINGS.on = v; game.destr.furn.forget(); made = ''; },
    'also ?things=0 on the address. On: every solid thing in the level has one row — what it is made of, how big it is, whether it stands, rests on something or hangs — and whatever is a thing breaks, under a roof or not. Off: only what stands under a roof between 0.3 and 2 m tall is furniture, as it was. What is already broken stays broken until you put it back.');
  panel.toggleBox(one, 'one set of rings for every burst', () => THINGS.rings.on, v => { THINGS.rings.on = v; },
    'also ?rings=0 on the address; acts only with the row above on. On: every burst is the same rings scaled by its size (a hand grenade is 0.6 of a wall charge, a door charge 0.7) and walls, cover, doors, furniture and loose things all read their reach from them. Off: each keeps its own reach, as it was.');
  const rg = panel.section('The rings of a wall charge, m', { open: true, reset: () => { RINGS.forEach((k, i) => { THINGS.rings[k] = was[i]; }); THINGS.rings.frag = frag0; }, resetTitle: 'the rings back to the build\'s' });
  for (const k of RINGS) panel.slider(rg, k, 0.2, 10, 0.1, () => THINGS.rings[k], v => { THINGS.rings[k] = v; }, TIP[k]);
  panel.slider(rg, 'a grenade\'s size', 0.3, 1.5, 0.05, () => THINGS.rings.frag, v => { THINGS.rings.frag = v; }, 'every burst that is not a breaching charge, against its own radius: 1 = a hand grenade is 2.5 ∕ 2.2 of a wall charge, as the walls have always taken it; 0.5 = about what the furniture took of one before');
  const lv = panel.section('What this level is made of', { open: true });
  panel.text(lv, () => { if (!DESTRUCTION.on) return 'destruction is off (?nodestruction)';
    if (!made) { const c = game.destr.things.census(); made = `${c.n} boxes · who breaks them: ${row(c.via)}\nwhat stays: ${row(c.why)}\nthe things: ${row(c.mat)} · ${row(c.hold)}`; }   // (the rows are the level as authored: counted once)
    return `${made}\n${game.destr.furn.summary()}`; });
  game.playerInvisible = BREAKLAB.unseen; game.godMode = BREAKLAB.safe;
  panel.show(BREAKLAB_SYSTEM);
}
/** ONCE A PAGE FRAME, before the game's update: the lab's two flags, and a click on the view while a burst is armed sets it off at the cursor. The click is
 *  taken from the input before the game reads it (the edge and the held bit both), so the gun does not fire and a takedown does not start. */
export function breakLabFrame(game: Game, input: Input) {
  if (!BREAKLAB.on) return; game.playerInvisible = BREAKLAB.unseen; game.godMode = BREAKLAB.safe;
  if (!BREAKLAB.armed || !(input.clicked & 1)) return;
  input.clicked &= ~1; input.buttons &= ~1; setOff?.();
}
