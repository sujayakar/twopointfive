// THE SMOKE LAB — a place to throw smoke and see what the solver is doing with it. The page opened with ?smokelab (the dev server's /smokelab: the look page on the
// lab's own floor, levels/smoke-lab.json — a yard, a tunnel, two rooms with a door between them and a shed, tools/levels/smoke-lab.ts; a levelFile on the address
// opens that level instead) gets its own system in the panel, open from the first frame:
//   what the pool holds right now · a can of smoke at your feet or at the cursor, all of it cleared, the field held still
//   TWO PICTURES OF THE SOLVER: its bricks drawn as wire (the coarse lattice's blue, the fine one's amber), and the smoke coloured by how hard the air in it turns
//   THE CURLS, each kind on a box of its own (what the solver keeps turning, the gusts it adds, what a body sheds), then every number of the motion, of a can,
//   of how it is drawn and of the pool, as live sliders · the build's numbers back by a button · the numbers as they stand copied out as one line of text
// Nothing here is the game's: it writes the tables the solver already reads (smoke.params, smoke.sparse), the solver's `show` word (two pictures the composite
// draws, smoke.ts) and two of the sandbox's flags. Without the word none of it is made.
import type { Game } from '../game/game';
import type { Panel } from '../ui/panel';
import type { Engine } from '../engine';
import type { SmokeSystem } from '../smoke/smoke';
import { lookAmounts, lookSync } from '../render/gbuffer';

export const SMOKELAB = { on: false, unseen: true, safe: true };
export const SMOKELAB_SYSTEM = 'Smoke lab';
type Table = Record<string, unknown>;
interface Num { t: 'p' | 'x'; key: string; label: string; min: number; max: number; step: number; tip: string; pub?: boolean }
const MOVE: Num[] = [
  { t: 'p', key: 'buoyancy', label: 'how hard heat lifts it', min: 0, max: 6, step: 0.1, tip: 'hot smoke climbs by this much: 0 = it only goes where it is thrown' },
  { t: 'p', key: 'weight', label: 'how heavy cold smoke is', min: 0, max: 0.3, step: 0.005, tip: 'smoke that has cooled sinks by this much: more and a plume settles to the floor sooner' },
  { t: 'p', key: 'velDamp', label: 'how fast the air comes to rest', min: 0, max: 2, step: 0.05, tip: 'the share of its speed the air loses a second: low = it keeps drifting, high = it stops where it was pushed' },
  { t: 'p', key: 'densDecay', label: 'how fast smoke thins out, /s', min: 0, max: 1, step: 0.01, tip: 'the share of the smoke that goes each second: this is how long a cloud lasts' },
  { t: 'p', key: 'tempDecay', label: 'how fast it cools, /s', min: 0, max: 4, step: 0.05, tip: 'fast = it stops climbing a little above the can, slow = it reaches the ceiling and spreads there' },
  { t: 'p', key: 'rbIters', label: 'pressure passes a step (its cost)', min: 1, max: 10, step: 1, tip: 'how hard the solver works to keep the air from piling up: more = smoke goes round a wall and through a door more truly, and each step costs more' },
  { t: 'p', key: 'omega', label: 'how hard each pressure pass pushes', min: 1, max: 1.95, step: 0.05, tip: 'over-relaxation: higher gets further with the same passes, too high rings' },
];
const CURLS: Num[] = [
  { t: 'p', key: 'vortConf', label: 'how hard a curl is kept turning', min: 0, max: 15, step: 0.25, tip: 'the solver smears small curls away as it moves the air; this puts their spin back. 0 = smooth, lazy smoke, high = busy, rolling smoke' },
  { t: 'p', key: 'turbulence', label: 'how gusty a plume is', min: 0, max: 10, step: 0.25, tip: 'uneven pushes added where the smoke leaves its source: what breaks a column into puffs' },
  { t: 'p', key: 'stirConf', label: 'how many times more a moving body keeps', min: 0, max: 8, step: 0.25, tip: 'with the box above it up: right round a man walking through smoke the curls are kept this many times harder' },
];
const CAN: Num[] = [
  { t: 'p', key: 'canisterDensity', label: 'how much smoke a second', min: 5, max: 200, step: 1, tip: 'for cans thrown after the change' },
  { t: 'p', key: 'canisterDuration', label: 'how long it burns, s', min: 3, max: 60, step: 1, tip: 'for cans thrown after the change' },
  { t: 'p', key: 'canisterRadius', label: 'how wide its mouth is, m', min: 0.05, max: 0.4, step: 0.01, tip: 'for cans thrown after the change' },
  { t: 'p', key: 'canisterTemp', label: 'how hot it leaves', min: 0, max: 8, step: 0.1, tip: 'low = it hugs the floor and spreads, high = it goes for the ceiling' },
  { t: 'p', key: 'canisterSpeed', label: 'how fast it leaves, m/s', min: 0, max: 5, step: 0.1, tip: 'the jet up out of the can' },
];
const DRAWN: Num[] = [
  { t: 'p', key: 'densityScale', label: 'how thick a can\'s smoke is', min: 0.1, max: 4, step: 0.05, pub: true, tip: 'times the solver\'s smoke, for everything that reads it: the picture, the shadows and what people see through' },
  { t: 'p', key: 'fxDensityScale', label: 'how thick gun smoke and dust are', min: 0.2, max: 3, step: 0.05, pub: true, tip: 'the same for muzzle gas, impact puffs and dust' },
];
const POOL: Num[] = [
  { t: 'x', key: 'ring', label: 'cells of air kept round the smoke', min: 1, max: 8, step: 1, tip: 'smoke can only move into air the solver holds. More = it spreads more freely and more bricks are in use (watch the wire)' },
  { t: 'x', key: 'keep', label: 'steps an empty brick is held', min: 1, max: 60, step: 1, tip: 'how long a brick nothing needs any more waits before it is given back' },
  { t: 'x', key: 'flushEps', label: 'smoke thinner than this is dropped', min: 0, max: 0.02, step: 0.0005, tip: 'the faintest smoke the solver keeps: lower and a cloud\'s invisible skirt holds bricks for longer' },
  { t: 'x', key: 'thinDecay', label: 'thin smoke goes this much faster, /s', min: 0, max: 3, step: 0.05, tip: 'how far a cloud\'s faint skirt spreads before it is gone. 0 = thin smoke lasts as long as thick' },
  { t: 'x', key: 'thinBelow', label: 'what counts as thin', min: 0.01, max: 1, step: 0.01, tip: 'the density under which the row above bites' },
  { t: 'x', key: 'calmMax', label: 'frames a calm brick may wait', min: 1, max: 4, step: 1, tip: '1 = every brick steps every frame; 2 or 4 = bricks whose air is slow step less often, which is most of the saving' },
  { t: 'p', key: 'calmCells', label: 'how calm is calm, cells', min: 0, max: 1.5, step: 0.25, tip: 'a brick may wait when its fastest air crosses no more than this many cells meanwhile. 0 = nothing in sight ever waits' },
  { t: 'x', key: 'farCells', label: 'the same, out of sight', min: 0, max: 3, step: 0.25, tip: 'how much laxer the rule is away from where the camera looks' },
  { t: 'x', key: 'fresh', label: 'share of the pool kept for new smoke', min: 0, max: 0.4, step: 0.01, tip: 'so a can just thrown stands up in a pool full of old haze' },
  { t: 'x', key: 'fineDecay', label: 'gun gas goes this much faster, /s', min: 0, max: 6, step: 0.1, tip: 'the fine lattice (amber wire) gives its bricks back sooner' },
  { t: 'x', key: 'fineShare', label: 'share of the pool gun gas may hold', min: 0.05, max: 1, step: 0.05, tip: 'the fine lattice\'s ceiling' },
];
const ALL = [...MOVE, ...CURLS, ...CAN, ...DRAWN, ...POOL];

/** is this page the lab (the word on its address) */
export const smokeLabAsked = (q: URLSearchParams): boolean => q.has('smokelab');

/** ONCE, after the panel is built: the lab's system, its rows, the panel open on it */
export function smokeLabBoot(game: Game, panel: Panel, smoke: SmokeSystem, engine: Engine) {
  SMOKELAB.on = true;
  const tab = (n: Num) => (n.t === 'p' ? smoke.params : smoke.sparse) as unknown as Table;
  const get = (n: Num) => +(tab(n)[n.key] as number), set = (n: Num, v: number) => { tab(n)[n.key] = v; if (n.pub) smoke.refreshDomains(); };
  const was = ALL.map(get), wasStir = lookAmounts.stir;
  const back = (rows: Num[]) => () => { for (const n of rows) set(n, was[ALL.indexOf(n)]); };
  const rows = (sec: HTMLElement, list: Num[]) => { for (const n of list) panel.slider(sec, n.label, n.min, n.max, n.step, () => get(n), v => set(n, v), n.tip); };
  /** a box that puts one number to 0 and back to what it was (the slider under it still moves it) */
  const onOff = (sec: HTMLElement, label: string, n: Num, tip: string) => { let kept = get(n) || was[ALL.indexOf(n)] || 1; panel.toggleBox(sec, label, () => get(n) > 0, v => { if (!v) { kept = get(n) || kept; set(n, 0); } else set(n, kept); }, tip); };
  const can = (at: readonly number[]) => smoke.spawnCanister([at[0], 0, at[2]]);
  let copied = '';
  const asText = () => JSON.stringify({ params: Object.fromEntries(ALL.filter(n => n.t === 'p').map(n => [n.key, get(n)])), pool: Object.fromEntries(ALL.filter(n => n.t === 'x').map(n => [n.key, get(n)])), stir: lookAmounts.stir,
    // (the picture's rows too: a plume tuned with the blur behind it at 0 is not the plume its solver numbers alone describe)
    draw: { creases: D.creases, selfK: D.selfK || 6, selfReach: D.selfReach || 0.72, ahead: D.ahead, keep: P.smokeDiffuseKeep, glowThrough: S.smokeMultiScatter, blurBehind: P.smokeDiffuse, blurStart: S.smokeDiffuseStart ?? 0.35, leanOnNew: S.smokeTaaReact, marchLights: S.smokeMarchLights, stepCells: S.volStepVoxels || 1.5, quarter: S.volQuarter, scaledFreely: S.smokeUpsampleRelax },
    streak: { ...K }, wake: { speed: W.speed, radius: W.radius } });
  panel.system(SMOKELAB_SYSTEM, { icon: '☁', short: 'smoke', title: 'throw smoke, see the solver\'s bricks and curls, move its numbers' });

  const now = panel.section('Right now', { open: true });
  panel.text(now, () => { const st = smoke.stats; return st.pool ? `${st.pool} bricks of ${smoke.grid.cap} in use (${st.fineAll} of them the fine lattice's). ${st.stepped} stepped this frame, ${st.skipped} waited. ${st.emitters} sources.${st.failed ? ` The pool refused ${st.failed} bricks this step.` : ''}` : 'No smoke yet. Throw a can ([3] then click), or use the buttons below.'; });
  panel.buttons(now, 'smoke', [
    { text: 'a can at my feet', run: () => can(game.player.char.pos), title: 'a smoke grenade starts burning where you stand' },
    { text: 'a can at the cursor', run: () => can(game.aimPoint), title: 'a smoke grenade starts burning on the floor under the cursor' },
    { text: 'clear it all', run: () => smoke.clearAll(), title: 'every source and all the smoke gone, the pool empty' }]);
  panel.toggleBox(now, 'hold the smoke still', () => smoke.params.frozen, v => { smoke.params.frozen = v; }, 'the field stops exactly as it is, still drawn, so you can walk round it and look. Nothing new is let out while it is held');
  panel.toggleBox(now, 'they can\'t notice you', () => SMOKELAB.unseen, v => { SMOKELAB.unseen = v; }, 'on: nobody sees or hears you, so you can look at the smoke. Off: the real guards');
  panel.toggleBox(now, 'nothing hurts you', () => SMOKELAB.safe, v => { SMOKELAB.safe = v; }, 'god mode while the lab is open');

  const see = panel.section('See the solver', { open: true, reset: () => { smoke.show.bricks = false; smoke.show.spin = false; smoke.show.spinFull = 8; engine.settings.debugView = 0; }, resetTitle: 'both pictures off, the final picture back' });
  panel.toggleBox(see, 'draw the bricks', () => smoke.show.bricks, v => { smoke.show.bricks = v; }, 'the solver only holds air in bricks of 8 × 8 × 8 cells, where smoke is and a little round it. Blue wire = a brick of the coarse lattice (cans, dust: 0.44 m across), amber = the fine one (gun gas, a fire\'s flame: 0.18 m). Wire behind a wall is hidden');
  panel.toggleBox(see, 'colour the smoke by its spin', () => smoke.show.spin, v => { smoke.show.spin = v; }, 'the smoke of the coarse lattice drawn again by how hard the air in it is turning: blue = still, yellow = turning, red = the full spin of the row below. This is where the curls are. Gun gas is not coloured, and dust is drawn as plain smoke meanwhile');
  panel.slider(see, 'spin drawn as full red, /s', 1, 40, 1, () => smoke.show.spinFull, v => { smoke.show.spinFull = v; }, 'lower it to see the gentle curls of drifting smoke, raise it to tell the fast ones at a source apart');
  panel.toggleBox(see, 'the smoke alone, nothing else drawn', () => engine.settings.debugView === 5, v => { engine.settings.debugView = v ? 5 : 0; }, 'the debugger\'s volumetrics view: the smoke and the lit air with the level left out (the two pictures above are drawn in it too)');

  const cu = panel.section('The curls', { open: true, reset: () => { back(CURLS)(); lookAmounts.stir = wasStir; lookSync(); }, resetTitle: 'the curls\' numbers back to the build\'s' });
  onOff(cu, 'the solver keeps curls turning', CURLS[0], 'off = no spin is put back: smooth smoke that only drifts and rises. The first slider below is how hard');
  onOff(cu, 'a plume is gusty', CURLS[1], 'off = smoke leaves its source in one even column. The second slider below is how gusty');
  panel.toggleBox(cu, 'a body moving through smoke sheds curls', () => lookAmounts.stir > 0, v => { lookAmounts.stir = v ? 1 : 0; lookSync(); }, 'the look page\'s Stirred smoke curls row: a man walking through smoke kicks the air behind him to one side and then the other, and the eddies are kept turning while he is near. Walk through a cloud with the spin picture up to see them');
  rows(cu, CURLS);

  const mo = panel.section('How it moves', { open: true, reset: back(MOVE), resetTitle: 'the motion\'s numbers back to the build\'s' });
  rows(mo, MOVE);
  const cn = panel.section('A can of smoke', { open: false, reset: back(CAN), resetTitle: 'the can\'s numbers back to the build\'s' });
  rows(cn, CAN);
  const dr = panel.section('How thick it is drawn', { open: false, reset: back(DRAWN), resetTitle: 'back to the build\'s' });
  rows(dr, DRAWN);

  // HOW IT'S DRAWN: the solver's field is one thing and the picture of it another. The first rows are new and all start where the picture has always been (0, or the
  // build's own number); the rest are rows the debugger already had under Volume, gathered here so the whole drawing can be played with in one place.
  const S = engine.settings, D = engine.smokeDraw, P = engine.passes;
  const drawBack = () => { D.creases = 0; D.selfK = 0; D.selfReach = 0; D.ahead = 0; P.smokeDiffuseKeep = 0; };   // (the rows under them belong to the quality preset and the debugger: their own pages put them back)
  const hd = panel.section('How it\'s drawn', { open: true, reset: drawBack, resetTitle: 'the five new rows back to the picture as it was' });
  panel.text(hd, () => 'None of these move the smoke itself. The first five are new and start at the picture as it was.');
  panel.slider(hd, 'creases go dark', 0, 2.5, 0.05, () => D.creases, v => { D.creases = v; }, 'the soft light on smoke is dimmed where more smoke crowds round it, so the crease between two billows goes dark and a crest keeps its light. 0 = one even soft light over the whole plume, as it was. Try 1');
  panel.slider(hd, 'how dark the far side of a fold is', 1, 20, 0.5, () => D.selfK || 6, v => { D.selfK = v === 6 ? 0 : v; }, 'how much the smoke between a sample and a lamp takes from that lamp\'s light. 6 = as it was. Try 10: the lit side of a billow keeps its light and the fold behind it goes darker');
  panel.slider(hd, 'how far a fold\'s shadow looks, m', 0.1, 0.74, 0.02, () => D.selfReach || 0.72, v => { D.selfReach = Math.abs(v - 0.72) < 0.005 ? 0 : v; }, 'the shadow smoke throws on itself is read at four points over this distance toward the lamp. 0.72 = as it was, and as far as it can look. Shorter (try 0.4) picks out small folds');
  panel.slider(hd, 'a lamp behind smoke glows through it', 0, 1, 0.05, () => D.ahead, v => { D.ahead = v; }, 'smoke sends most of the light it scatters nearly straight on. Look at a lit sign or a lamp through smoke and the smoke between glows in its colour, softer and wider the more smoke there is. Smoke lit from the side does not change. 0 = smoke only dims a lamp behind it, as it was. Try 0.5');
  panel.slider(hd, 'the smoke\'s own light stays sharp', 0, 1, 0.05, () => P.smokeDiffuseKeep, v => { P.smokeDiffuseKeep = v; }, 'the blur two rows down softens the whole pixel, the plume\'s own lit folds with it. At 1 it only blurs what is behind the plume: where nearly all of a pixel is the smoke\'s own light, the pixel is left as the march drew it. 0 = as it was');
  panel.slider(hd, 'light glows through the body', 0, 1.5, 0.05, () => S.smokeMultiScatter, v => { S.smokeMultiScatter = v; }, 'a second, softer pass of each lamp\'s light through half the smoke: a thick plume shows the colour of its lit side through its body');
  panel.slider(hd, 'smoke blurs what is behind it', 0, 1, 0.05, () => P.smokeDiffuse, v => { P.smokeDiffuse = v; }, 'what you see through smoke is blurred, more the more smoke you look through. 0 = smoke only dims what is behind it');
  panel.slider(hd, 'the blur starts at this much smoke', 0, 1.5, 0.05, () => S.smokeDiffuseStart ?? 0.35, v => { S.smokeDiffuseStart = v; }, 'thinner smoke than this blurs nothing (an optical depth: 0.35 lets about seven tenths of the light through)');
  panel.slider(hd, 'a moving plume leans on the new frame', 0, 1, 0.05, () => S.smokeTaaReact, v => { S.smokeTaaReact = v; }, 'the picture is built up over several frames. Low = smoother smoke that smears as it moves, high = each frame\'s own march shows, crisper in motion and grainier');
  panel.slider(hd, 'lamps that light each step themselves', 0, 6, 1, () => S.smokeMarchLights, v => { S.smokeMarchLights = v; }, 'the strongest lamps on a stretch of smoke are lit sample by sample with their own shadow and the smoke\'s shadow on itself. 0 = every lamp reaches the smoke as one soft wash, the flat look');
  panel.slider(hd, 'how far apart the samples are, cells', 1, 3, 0.1, () => S.volStepVoxels || 1.5, v => { S.volStepVoxels = v; }, 'the march through the smoke takes a sample every so many cells of the solver. 1 = every cell is seen, and it costs more');
  panel.toggleBox(hd, 'drawn at a quarter of the size', () => S.volQuarter, v => { S.volQuarter = v; }, 'smoke is marched at half the picture\'s size and scaled up. On = a quarter: cheaper and softer. Off is the most detail the picture can show');
  panel.toggleBox(hd, 'scaled up freely inside smoke', () => S.smokeUpsampleRelax, v => { S.smokeUpsampleRelax = v; }, 'on = inside a plume the scaling up stops following the edges of what is behind it');

  // STREAKS THE AIR CAN FOLD: a can that burns unevenly (smoke.ts pack). The bands are a tint the smoke carries, drawn by the dust program with its shaping put down.
  const K = smoke.streak;
  const sk = panel.section('Streaks the air can fold', { open: true, reset: () => { K.bands = 0; K.puffs = 0; K.every = 1.4; }, resetTitle: 'an even can again' });
  panel.slider(sk, 'dark bands in a can\'s smoke', 0, 0.6, 0.02, () => K.bands, v => { K.bands = v; }, 'the can lets out a darker band of smoke, then a pale one. The air carries the bands, so a curl, a draught through a door or a round going through shows as the bands it bends. 0 = one even white, as it was. Try 0.3: a can that is already burning starts at once. While it is up, dust is drawn as a plain tint too');
  panel.slider(sk, 'seconds from one band to the next', 0.4, 4, 0.1, () => K.every, v => { K.every = v; }, 'shorter = many thin bands, longer = a few wide ones');
  panel.slider(sk, 'the can lets smoke out in puffs', 0, 0.9, 0.05, () => K.puffs, v => { K.puffs = v; }, 'how unevenly the can burns: its smoke comes thicker and thinner by this share. This one changes the smoke itself, so what people can see through it changes too');
  // A ROUND THROUGH SMOKE (smoke.ts `wake`): the air along a round's path goes with it for a moment.
  const W = smoke.wake, wakeWas = { ...W };
  const wk = panel.section('A round through smoke', { open: true, reset: () => { W.speed = wakeWas.speed; W.radius = wakeWas.radius; }, resetTitle: 'the wake\'s numbers back' });
  panel.slider(wk, 'how fast a round drags the air, m/s', 0, 11, 0.5, () => W.speed, v => { W.speed = v; }, 'a round takes the air along its path with it for a moment, so smoke follows it out of the far side of a plume and clear air is pulled in behind. 0 = the faint nudge it was, which nothing showed');
  panel.slider(wk, 'how wide its wake is, metres', 0.06, 0.3, 0.01, () => W.radius, v => { W.radius = v; }, 'the half width of the air a round takes with it. The solver\'s cells are 5.5 cm, so under 0.1 there is little to see');
  const po = panel.section('The pool of bricks', { open: false, reset: back(POOL), resetTitle: 'the pool\'s numbers back to the build\'s' });
  rows(po, POOL);

  const out = panel.section('Hand the numbers back', { open: true });
  panel.buttons(out, 'numbers', [{ text: 'copy them', run: () => { copied = asText(); try { navigator.clipboard?.writeText(copied).catch(() => { /* refused: the line below shows them */ }); } catch { /* no clipboard here: the line below shows them */ } }, title: 'the numbers as they stand, as one line of text on the clipboard (and shown below)' }]);
  panel.text(out, () => copied || 'Move the sliders until it looks right, then copy the numbers and paste them where you send notes.');
  game.playerInvisible = SMOKELAB.unseen; game.godMode = SMOKELAB.safe;
  panel.show(SMOKELAB_SYSTEM);
}
/** ONCE A PAGE FRAME, before the game's update: the lab's two sandbox flags */
export function smokeLabFrame(game: Game) { if (!SMOKELAB.on) return; game.playerInvisible = SMOKELAB.unseen; game.godMode = SMOKELAB.safe; }
