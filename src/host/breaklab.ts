// THE BREAK LAB — a place to set bursts off beside every kind of thing and watch how each breaks. The page opened with ?breaklab (the dev server's /breaklab:
// the kit's showroom yard, every piece of the kit stood in it once, indoors and out; /breaklab?levelFile=pass.json for the pass) gets its own system in the
// panel, open from the first frame:
//   the one index of every thing and the one set of rings for every burst (game/things.ts) as two switches, to see the build as it was beside them ·
//   the rings' radii and a grenade's size as live sliders, the build's numbers back by a button ·
//   a hand grenade, a door charge and a wall charge set off where the cursor is, and the level put back as it began ·
//   what the level under you is made of: how many things each engine breaks, and what stays and why ·
//   'they can't notice you' and 'nothing hurts you' (both on to begin with: the lab is for the things)
// Nothing here is the game's: it writes the table the game reads, calls the one blast entry point every charge and grenade calls, and sets two of the
// sandbox's flags. Without the word none of it is made.
import type { Game } from '../game/game';
import type { Panel } from '../ui/panel';
import type { Vec3 } from '../math/vec';
import { THINGS, ringsOn, ringsOf, burstSize } from '../game/things';
import { FURN_STATS } from '../game/furnblast';
import { FRAG } from '../game/frag';
import { DESTRUCTION } from '../game/destructionSwitch';
import { PLAYER_ID } from '../game/consts';

export const BREAKLAB = { on: false, unseen: true, safe: true };
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
  const was = RINGS.map(k => THINGS.rings[k]), frag0 = THINGS.rings.frag; let made = '', last = 'Aim the cursor at the ground beside a thing and press one of the bursts.';
  const at = (): Vec3 => { const a = game.aimPoint; return [a[0], Math.max(a[1], 0.3), a[2]]; };
  const burst = (what: string, opts: { radius?: number; power?: number; chaos?: boolean }) => {
    const f = FURN_STATS, n0 = f.statics + f.razed + f.items + f.propsBroken, s0 = f.shaken, h0 = f.held, t0 = f.propsThrown, p = at();
    let r = { walls: 0, cover: 0, doors: 0 };
    if (what === 'door charge') { game.destr.bang(p, PLAYER_ID, 1, false); game.destr.furnBlast(p, 'door', PLAYER_ID); }   // (as the game's own door charge does it, combat.ts: its borrowed bang and the furniture round it — the leaf itself is the door tools' work, not the lab's)
    else { r = game.destr.blast(p, { ...opts, who: PLAYER_ID }); if (opts.chaos === false) game.destr.bang(p, PLAYER_ID, 2); }   // (a wall charge brings its own bang in play: the lab stands in for it, for the dust)
    const Q = ringsOn() ? ringsOf(burstSize(opts)) : null;
    last = `${what}: ${f.statics + f.razed + f.items + f.propsBroken - n0} parts broke, ${f.propsThrown - t0} loose things thrown, ${f.shaken - s0} small things fell, ${f.held - h0} stone things held, ${r.walls} wall${r.walls === 1 ? '' : 's'} opened, ${r.cover} cover, ${r.doors} door${r.doors === 1 ? '' : 's'} off` +
      (Q ? `\nits rings: contact ${Q.contact.toFixed(1)}, shatter ${Q.shatter.toFixed(1)}, throw ${Q.throw.toFixed(1)}, shake ${Q.shake.toFixed(1)} m` : '\neach engine by its own reach');
  };
  const row = (o: Record<string, number>) => Object.entries(o).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${v}`).join(', ');
  panel.system(BREAKLAB_SYSTEM, { icon: '✸', short: 'break', title: 'set bursts off beside every kind of thing and watch how each breaks' });
  const now = panel.section('Bursts at the cursor', { open: true });
  panel.buttons(now, 'bursts', [
    { text: 'grenade', run: () => burst('grenade', { radius: FRAG.destrM, power: 1 }), title: 'a hand grenade\'s burst where the cursor is: what it breaks only, it hurts nobody' },
    { text: 'door charge', run: () => burst('door charge', { chaos: false, radius: 1.0 }), title: 'a door charge\'s burst where the cursor is' },
    { text: 'wall charge', run: () => burst('wall charge', { chaos: false }), title: 'a wall charge\'s burst where the cursor is (a building\'s outer wall still wants the charge set on it by hand)' },
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
/** ONCE A PAGE FRAME, before the game's update: the lab's two flags */
export function breakLabFrame(game: Game) { if (!BREAKLAB.on) return; game.playerInvisible = BREAKLAB.unseen; game.godMode = BREAKLAB.safe; }
