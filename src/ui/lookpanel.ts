// THE LOOK PROTOTYPE's page of the debugger (the page /look; render/frame.ts lookConfig, shaders/look.wgsl): one slider a layer of the look, written straight into the
// g-buffer's uniform block, a button that shows the boxes plain while it is held, the hour and the quality preset, and what a frame costs. Made only on that page.
import type { Engine } from '../engine';
import { lookAmounts, lookSync, LOOK_DEFAULTS } from '../render/gbuffer';
import type { PerfRecorder, PerfDump } from './perfrec';
import type { PerfSweep } from './perfsweep';
import { takeRead, takeWords, sendTake } from './perfsend';
import { sharpForce } from './quality';
import type { Panel } from './panel';
import { POWDER, POWDER_DRAW } from '../render/powderField';

/** rows that are not the materials' own: "plain boxes" leaves them as they are */
const LIVE: ReadonlySet<string> = new Set(['even', 'size', 'sky', 'lamps', 'veil', 'puffs', 'eyes', 'firesmoke', 'windows', 'glare', 'stir', 'rooms', 'steady', 'roomb', 'order', 'dark', 'gexp', 'contrast', 'highs', 'lows', 'colour']);
const ROWS: readonly [keyof typeof lookAmounts, string, string, number][] = [
  ['clarity', 'Clarity', 'one number over Relief, Cavities, Edges and Snow relief: how hard neighbouring detail is set against each other. Lower = calmer surfaces', 1.5],
  ['paint', 'Paint', 'the colour the maps add to the box paint', 1.5],
  ['relief', 'Relief', 'how deep the normal maps cut', 1.6],
  ['shine', 'Finish', 'roughness from the maps, and how far a lacquer mirrors more than plain paint does (car paint)', 1.5],
  ['cavity', 'Cavities', 'joints and pits hold shadow', 1.5],
  ['weather', 'Weather', 'damp wall feet, bleached tops, runs, trodden ground; on what somebody keeps, use: lanes, chipped and changed pieces, knocked wall feet and worn paint where feet go', 1.5],
  ['untile', 'Break the repeat', 'a stuff made of pieces deals them from its map and is laid in bays; a plain coat is read twice and cross-faded; a kept place drifts a shade over ten metres. Past 1: stronger bays and drift', 2],
  ['edges', 'Edges', 'small boxes: rounded, rubbed corners', 1.5],
  ['snow', 'Snow that clings', 'a snow level: blown against wall feet, lying on what was left out', 1.5],
  ['ground', 'Snow relief', '× the depth of lying snow\'s drifts, wind ridges and prints (1 = as the game draws it)', 3],
  ['crust', 'Snow crust', 'wind-glazed streaks of snow that hold a lamp\'s sheen', 1.5],
  ['stone', 'Stonework', 'a plinth of field stone under the mud walls and mud brick showing where the render has fallen', 1.5],
  ['even', 'Maps keep their paint', 'a material map varies a thing\'s paint and never moves it: over a tile, a map is as bright as the paint under it, and its hollows darken as much as its high places lighten. Before, most maps left a thing a little darker or bluer than its paint. Read when the page opens: put ?look=even:0 on the address and reload for the maps as they were', 1],
  ['marks', 'Painted marks', 'signs, stripes and rugs drawn as old paint in place of square cells: strokes of an uneven width, rubbed off in patches, a shade into the surface under them. 0 = the square cells, 1 = a plain, moderate read, 2 = heavily worn', 2],
  ['men', 'Men', 'the men drawn as cloth, webbing, steel and skin: a print on the uniforms, folds, scuffs', 1.5],
  ['tailor', 'Kit detail', 'what was cut and sewn on the men: seams, pockets and patches, a combat shirt, bar tacks on the webbing, sand boots, knee pads, buttons', 1],
  ['print', 'Printed cloth', "the uniform's print as it is on the bolt: a ground that drifts from tan to sage, green and brown shapes, cream flecks. 0 = the first print", 1],
  ['air', 'Air', '× the haze that shows a lamp in the air', 4],
  ['sky', 'Sky light', '× the light the sky gives: what reaches under a roof and into shade by day (1 = the hour as it is). A measuring row. It moves the real light on this page, so the guards and your light meter see it too', 4],
  ['shade', 'Eyes adjust to shade', 'by day, a place that gets less light than the open air is shown brighter, the way your eyes open up when you look into a shed. 0 = one exposure for the whole picture, 1 = the shade is brought all the way up. The picture only: the guards and your light meter read the light as it is', 1],
  ['eyes', 'Eyes follow Sam', 'by day, the picture is exposed for the light where Sam stands. When he is in a dim room the whole picture opens up, the room keeps its fall-off from the window, and the yard outside is a little overexposed. It eases over about a second as he walks in and out. 0 = every shaded place is brought up by its own darkness, wherever he stands', 1],
  ['windows', 'Daylight at openings', 'each window, open door and hole blown in a wall is a light of its own. The light falls off from the opening and anyone standing by it throws a shadow into the room. Shut the door and its light goes, and it comes back with the first hand\'s width the door opens. The light is what you could see through the opening from where it lands, which is the sky itself and the ground outside at every hour, and the bounce light does not bring the same sky in a second time. This row is a switch, so any amount is all of it. 0 = daylight comes in only through the bounce light, which has no direction', 1],
  ['lamps', 'Real light values', 'by day, everything that is not the sun or the sky is drawn at its real strength for that hour: a bulb at 1 in 5,000 at midday and 1 in 70 at dusk, a flashlight at 1 in 100 at midday and about two thirds at dusk, lamp glass, flashes and fire light by their own real values. 1 = real, 0 = as the game has them today (a lantern as strong as the midday sun). Night is not touched. It moves the real light on this page, so your light meter and what the guards see of you move with it', 1],
  ['veil', 'Blowing snow', 'on a snow level, the wind picks loose powder up off lying snow where it blows hard enough (the two wind rows of the Snow wisps section say how hard) and carries it low over the ground. It rides the same air the falling snow does, so it bends around a wall, gathers where the wind squeezes through a gap and drops out in a wall\'s shelter. 0 = the wind picks none up (the game\'s own wisps and grains are not touched by this row)', 1],
  ['puffs', 'Snow puffs', 'on a snow level, each boot that lands kicks powder into that same air, more the harder it lands, and the wind takes what is left of it. It is drawn on the ground under it, with that spot\'s own light and shadow. 0 = none', 1],
  ['stir', 'Stirred smoke curls', 'a man moving through smoke is a body the smoke has to go round, he kicks the air behind him to one side and then the other as he strides, and the eddies he sheds are kept turning while he is near. Smoke nobody walks through is not touched. Guards see through the same smoke, so this changes sight lines too. 0 = the two soft shoves along his heading, as the game has them', 2],
  ['firesmoke', 'Fire smoke', 'a fire\'s smoke is born in the flame and climbs out of its tips, and the fire puffs a few times a second so the smoke rolls as it rises. 0 = the smoke as the game has it, a steady column that starts well above the flame. A change shows over the next few seconds', 1],
  ['size', 'Map size', '× the 2 m a map covers', 2],
  ['glare', 'Glare outside', 'with Sam in a dim room, how much brighter than itself the open ground outside is shown. 0 = the yard exactly as it is when he stands in it, 1 = up to four times brighter, which burns the snow out flat', 1],
  ['rooms', 'Light stays in its room', 'the bounced light no longer leaks through walls: a shut room with no window holds its own lamps\' light and not a share of the yard\'s daylight, a room with a window holds what comes in at the window, and by day a lamp\'s bounce is kept. Day and night. It changes the light itself, so the light meter and what guards see change with it on this page. 0 = the bounce as it was (with the Walls have two sides row down as well: that row stands in place of the older wall guards)', 1],
  ['steady', 'Steady bounce', 'by day, the bounced light holds still. By day a room is lit by what comes in at a door or a window, which is small and very bright, and the bounce caught it on some frames and missed it on others, so walls, shiny things and people pulsed. With this up the bounce is the average of a few fixed looks taken in turn, so nothing changes from frame to frame while the scene is still, and a real change of light comes in over about an eighth of a second. A reflection is also held to a brightness that fits the room it is in, where it was held to a number made for nights and one stray ray out of a door was a white streak. 0 = both as at night. Needs the row above', 2],
  ['walls', 'Walls have two sides', 'one rule for every read of the bounced light: the level\'s own floor plan says which side of each wall and shut door a point is on, and a read keeps to its side. The bounce is kept at points a metre apart and every surface blends the points around it, so within a metre of a wall it took in the point on the far side: by day the far side of an outer wall is the open air, and the floor behind a row of drums in a shut room glowed. It stands in place of four older guards that each met one case of this. Read when the page opens: put ?look=walls:0 on the address and reload for the page with the older guards', 1],
  ['roomb', 'Room brightness', 'by day, how far a room is brought up toward the open air\'s brightness. 1 = as the one exposure model shows it. Lower and every room is shown darker, a dark shut room by more than a room with windows: 0.9 takes about half a stop off a room with windows and a stop off a shut one. 0 = no room is brought up at all. Over 1 = brighter. It does nothing at night or out of doors', 1.5],
  ['order', 'Light in order', 'by day, seen from a brighter place: what stands where two places meet (an open door\'s leaf, the floor just inside a door, a man in a hole blown in a wall) is shown as the brighter place would show that much light, and no longer by the dim room\'s whole gain. Deep in a room nothing changes. 0 = as it was', 1],
  ['dark', 'Eyes open in the dark', 'how far past the night\'s own exposure a place is shown when it holds under about two fifths of a night yard\'s light. Such a place comes up by the Eyes adjust to shade row\'s share of how dark it is, wherever you stand, so a darker room stays the darker one. 1 = by two stops at the most (four times), 0 = not at all, 2 = four stops. Under the goggles nothing is brought up: the tube is the eye', 2],
  ['gexp', 'Exposure', 'the whole picture brighter or darker, as a camera\'s exposure: 1 = the hour\'s own, 0 = two stops under, 2 = two stops over', 2],
  ['contrast', 'Contrast', 'by day, how far everything darker than sunlit ground is set under it. The snow keeps its brightness, shadows and dark kit go down. 1 = the tone curve as the game has it, which is made for nights and leaves a day pale', 2],
  ['highs', 'Highlights', 'the bright end only: sunlit snow, sky light on a wall. Under 1 pulls them down so the snow keeps its shape, over 1 pushes them up', 2],
  ['lows', 'Shadows', 'the dark end only. Over 1 opens the shadows, under 1 sinks them', 2],
  ['colour', 'Colour', 'how strong the colours are (1 = as it is)', 2],
];

/** the cloud cover the look page opens a snow level under (main.ts; the game's own snow is 0.9) */
export const LOOK_SNOW_COVER = 0.5;

/** the look page's own system on the debugger's rail (ui/panel.ts): the page opens on it */
export const LOOK_SYSTEM = 'Materials prototype';

// A PAGE OF THE DEBUGGER, not a box of its own (the maintainer, 10-31: "could you make the materials prototype menu just another tab in the debug panel?"). The box
// stood over the debugger's own corner, so on this page the debugger's rows (the debug views, the denoiser, the gather) could not be reached while the look's rows
// were up. Now the rows are the panel's own: its sliders (a dot puts one row back, the section's ↺ puts them all back), its search finds them, its key shuts them.
// The page still OPENS with them up (panel.show), as the box did.
export function buildLookPanel(panel: Panel, engine: Engine, fps: () => number, shapes?: { on: boolean; flip: () => void }, hills?: { on: boolean; flip: () => void }, lorry?: { on: boolean; flip: () => void }): void {
  const w = window as any;
  const here = () => panel.system(LOOK_SYSTEM, { icon: '◍', short: 'Look', title: 'a slider a layer of the look · plain boxes to compare · the hour, sun and moon · the trials · performance takes · the character lab' });
  here();
  const send = () => { engine.gbuffer.setLook(); lookSync(); };
  let kept: typeof lookAmounts | null = null;   // the amounts as the sliders have them, while the boxes are shown plain
  let stayPlain = false;
  /** a row of the panel's that takes buttons with manners of their own (a hold, a text that changes) */
  const row = (sec: HTMLElement, label: string) => { const r = panel.buttons(sec, label, []); return (r.querySelector('.btns') as HTMLElement | null) ?? r; };
  const btn = (host: HTMLElement, text: string, tip = '') => { const b = document.createElement('button'); b.type = 'button'; b.textContent = text; if (tip) b.title = tip; b.addEventListener('mousedown', e => e.preventDefault()); host.append(b); return b; };   // (no focus: a focused control takes the game's keys, game/input.ts typingTarget)
  const words = (sec: HTMLElement, text = '') => { const d = document.createElement('div'); d.className = 'row textrow'; d.style.cssText = 'color:#8aa;white-space:pre-wrap'; d.textContent = text; sec.append(d); return d; };
  /** a slider of the panel's whose value can be absent (the moon by day): it greys out and shows a dot */
  const slide = (sec: HTMLElement, name: string, tip: string, min: number, max: number, step: number, get: () => number | null, set: (v: number) => void) => {
    const inp = panel.slider(sec, name, min, max, step, () => get() ?? min, set, tip), val = inp.nextElementSibling as HTMLElement | null;
    panel.keep(() => { const off = get() === null; inp.disabled = off; if (off && val) val.textContent = '·'; }, inp);
  };
  const layers = panel.section('Layers', { open: true, reset: () => { stayPlain = false; kept = null; for (const [k] of ROWS) lookAmounts[k] = LOOK_DEFAULTS[k]; send(); }, resetTitle: 'every row back to the number the page opens with' });
  panel.text(layers, () => { let f = 0; try { f = fps(); } catch { return ''; }   // (the frame loop's number is declared further down main.ts than this panel is made, and a row is read once as it is made)
    return `${f.toFixed(0)} fps · ${(1000 / Math.max(f, 1)).toFixed(1)} ms a frame · ${engine.width}×${engine.height} → ${engine.outWidth}×${engine.outHeight}`; });
  for (const [key, name, tip, max] of ROWS) panel.slider(layers, name, key === 'size' ? 0.5 : 0, max, 0.05, () => (kept ?? lookAmounts)[key], v => { (kept ?? lookAmounts)[key] = v; if (LIVE.has(key)) lookAmounts[key] = v; send(); }, tip, LOOK_DEFAULTS[key]);
  // SNOW WISPS (the maintainer, 11-03: "we also need to tweak the snow wisps before this release. make sure there's a section in the look panel for them"). The
  // Blowing snow and Snow puffs rows above say HOW MUCH; these are the powder's own numbers (render/powderField.ts POWDER: what the wind lifts and how long it
  // stays up, read every step) and how it is drawn (POWDER_DRAW: the streaks, live through the frame). A row is a number of those tables and nothing else, the
  // section's ↺ puts back what the page opened with, and "copy them" hands the lot back as one line. Picture only: nothing in the game reads the powder.
  {
    // (a row's name fits the panel's name column, twenty letters: the full sentence is the head of its tip, shown on hover)
    type Row = readonly [string, string, number, number, number, string];
    const table = (sec: HTMLElement, T: Record<string, number>, rows: readonly Row[]) => { for (const [key, name, min, max, step, tip] of rows) panel.slider(sec, name, min, max, step, () => T[key], v => { T[key] = v; }, tip, T[key]); };
    const was = { ...POWDER }, drawWas = { ...POWDER_DRAW };
    const WIND: readonly Row[] = [
      ['pick', 'how much wind lifts', 0, 4, 0.05, 'How much the wind picks up. How much loose snow a full gust lifts off open snow each second. 0 = the wind lifts none'],
      ['liftPick', 'more in rising air', 0, 5, 0.1, 'More where the wind climbs a wall. Extra snow lifted where the air is pushed upward, at the foot of a wall the wind blows against'],
      ['gateLo', 'wind to start', 0, 2.5, 0.05, 'Slowest wind that lifts any. As a share of the day\'s wind. Air slower than this lifts nothing, so the lee of a wall stays clear'],
      ['gateHi', 'wind for full lift', 0.5, 3.5, 0.05, 'Wind that lifts it in full. As a share of the day\'s wind. Lower and the open ground streams everywhere, higher and only the gaps where the wind squeezes through do.'],
      ['life', 'life in moving air', 1, 30, 0.5, 'Seconds it stays up in moving air. How long a streak lasts while the wind carries it. Longer and the streaks run further from where they were lifted'],
      ['lifeStill', 'life in still air', 0.2, 6, 0.1, 'Seconds it stays up in still air. How fast it drops out when it reaches shelter'],
      ['velK', 'how fast it travels', 0.3, 2, 0.02, 'How fast it travels. As a share of the air\'s own speed close to the ground. The falling snow rides the same air'],
      ['patch', 'gust size', 1, 12, 0.5, 'Size of a gust, metres. The snow is lifted in patches this big that come and go'],
      ['crest', 'streak source size', 0.1, 1, 0.02, 'Size of a streak\'s source, metres. Inside a gust the snow comes off small crests this big, and each one gives a streak about this wide'],
      ['crestCut', 'bare ground share', 0.5, 0.98, 0.01, 'Share of the ground that gives none. Higher = fewer, more separate streaks. Lower = an even sheet'],
      ['change', 'gust change speed', 0.02, 0.5, 0.01, 'How fast gusts come and go. Higher and the patches flicker, lower and the same places stream for a long time'],
      ['clump', 'thick falls faster', 0, 4, 0.1, 'Thick snow falls out faster. A thick cloud drops out sooner than a thin one, so a puff\'s middle goes in a second and the wind keeps the rest'],
      ['low', 'how high it rides', 0.03, 0.5, 0.01, 'How high the wind lifts it, metres. It is shaded and hides the ground as if it stood this high'],
    ];
    const DRAW: readonly Row[] = [
      ['gain', 'how strong it looks', 0.05, 12, 0.05, 'How strongly it shows. How much of the ground a given amount of blown snow hides. This is the row for wisps that are too faint or too heavy'],
      ['thin', 'haze between streaks', 0.01, 1, 0.01, 'How much shows between the streaks. Low = separate streaks on clear ground, high = a haze with streaks in it'],
      ['thread', 'streak brightness', 0.1, 20, 0.1, 'How bright a streak\'s middle is. The contrast of a streak against the haze between'],
      ['grain', 'streak width', 0.02, 0.25, 0.005, 'How wide a streak is, metres. A streak\'s length goes with its width. Wider reads softer and less like lines'],
    ];
    const PUFF: readonly Row[] = [
      ['kickColumn', 'boot kick amount', 0, 6, 0.1, 'How much a boot kicks up. Per step, more the harder the boot lands'],
      ['kickSigma', 'puff starting width', 0.03, 0.4, 0.01, 'How wide a puff starts, metres'],
      ['kickGrow', 'how much it spreads', 0, 1, 0.02, 'How much it spreads. Metres it grows by over its life, for a hard step'],
      ['kickSecs', 'seconds of swelling', 0.1, 1.5, 0.05, 'Seconds it goes on swelling'],
      ['kickHigh', 'throw height', 0.05, 0.8, 0.01, 'How high it is thrown, metres'],
      ['kickGo', 'follows the boot', 0, 2, 0.05, 'How far it follows the boot. The puff\'s middle travels the way the boot was going'],
    ];
    const ws = panel.section('Snow wisps', { open: true, reset: () => { Object.assign(POWDER, was); Object.assign(POWDER_DRAW, drawWas); }, resetTitle: 'the wisps\' and the puffs\' numbers back to what the page opened with' });
    panel.text(ws, () => 'On a snow level, with the Blowing snow row up. The first four rows are how the streaks are drawn, the rest is what the wind does.');
    table(ws, POWDER_DRAW as Record<string, number>, DRAW); table(ws, POWDER as Record<string, number>, WIND);
    const pf = panel.section('Snow puffs', { open: false, reset: () => { for (const [k] of PUFF) (POWDER as Record<string, number>)[k] = (was as Record<string, number>)[k]; }, resetTitle: 'the puffs\' numbers back' });
    table(pf, POWDER as Record<string, number>, PUFF);
    let copied = '';
    const r3 = (T: Record<string, number>) => Object.fromEntries(Object.entries(T).map(([k, v]) => [k, +v.toFixed(3)]));
    panel.buttons(ws, 'numbers', [{ text: 'copy them', run: () => { copied = JSON.stringify({ blowing: +lookAmounts.veil.toFixed(2), puffs: +lookAmounts.puffs.toFixed(2), draw: r3(POWDER_DRAW), powder: r3(POWDER) }); try { navigator.clipboard?.writeText(copied).catch(() => { /* refused: the line below shows them */ }); } catch { /* no clipboard here */ } }, title: 'the wisps\' numbers as they stand, as one line of text on the clipboard (and shown below)' }]);
    panel.text(ws, () => copied || 'When it looks right, copy the numbers and paste them where you send notes.');
  }
  // plain while held; the switch leaves it plain
  const plain = (on: boolean) => {
    if (on && !kept) { kept = { ...lookAmounts }; for (const [k] of ROWS) if (!LIVE.has(k)) lookAmounts[k] = k === 'air' || k === 'ground' || k === 'clarity' ? 1 : 0; }
    else if (!on && kept) { Object.assign(lookAmounts, kept); kept = null; }
    send();
  };
  const cmp = panel.section('Compare', { open: true });
  const hold = btn(row(cmp, 'plain boxes'), 'hold', 'the page as the game draws it today, while this is held'); let held = false; hold.addEventListener('pointerdown', () => { held = true; plain(true); }); for (const ev of ['pointerup', 'pointerleave', 'pointercancel']) hold.addEventListener(ev, () => { if (!held) return; held = false; if (!stayPlain) plain(false); });   // (only a hold that began here ends here: the sweep's plain row is not ended by a pointer passing over)
  panel.toggleBox(cmp, 'stay plain', () => stayPlain, v => { stayPlain = v; plain(v); }, 'the boxes stay plain until this is put off');
  // THE SUN AND THE MOON (the maintainer, 10-23: "do we model a sun at all for dawn/midday? i think that's a big reason why it looks flat. and perhaps we should also
  // model a moon on night levels (and can configure it)"). Both are there: the hour's sun (game/hours.ts), which a snow level's cloud leaves 9 % of, and the level's
  // moon, which the pass has at 0.06. These rows set them for the visit: Sun = the share of the hour's own sun that comes through (window.__hour.sun: it then stands
  // against the cloud), Moon / height / bearing = the level's moon (window.__moon). The page shows what it has: a row follows the hour and the level.
  const sky = panel.section('Hour, sun and moon', { open: true, summary: () => String(w.__hour?.id ?? '') });
  panel.buttons(sky, 'hour', ['night', 'dawn', 'midday', 'overcast', 'dusk'].map(h => ({ text: h, run: () => { w.__hour?.set(h); } })));
  slide(sky, 'Cloud', `by day: how much of the sky the weather’s cloud covers. 0 = a clear sky and a hard sun, 1 = a solid deck and no shadows. The game’s snow falls from 0.90; this page opens at ${LOOK_SNOW_COVER.toFixed(2)}. The sun, its shadows’ edge and the sky’s colour follow, and so do the guards and your light meter`, 0, 1, 0.01, () => w.__hour && w.__hour.id !== 'night' ? w.__hour.cover() : null, v => w.__hour?.cover(v));
  slide(sky, 'Sun', 'by day: how much of the hour’s own sun comes through that cloud (the game’s snow leaves 0.09). Set by hand it stands until Cloud is moved', 0, 1, 0.01, () => w.__hour && w.__hour.id !== 'night' ? w.__hour.sun() : null, v => w.__hour?.sun(v));
  slide(sky, 'Moon', 'at night: the moon’s strength (the pass has 0.06, the level kit’s own is 0.9)', 0, 1.5, 0.01, () => { const m = w.__moon?.get(); return m && m.up ? m.k : null; }, v => w.__moon?.set({ k: v }));
  slide(sky, 'Moon height', 'degrees over the horizon: low = long shadows', 5, 85, 1, () => { const m = w.__moon?.get(); return m && m.up ? m.elev : null; }, v => w.__moon?.set({ elev: v }));
  slide(sky, 'Moon bearing', 'where it stands, in degrees round the level', 0, 359, 1, () => { const m = w.__moon?.get(); return m && m.up ? m.bearing : null; }, v => w.__moon?.set({ bearing: v }));
  const qual = panel.section('Quality', { open: true, summary: () => String(w.__quality?.current ?? '') });
  panel.buttons(qual, 'preset', ['low', 'medium', 'high', 'ultra'].map(p => ({ text: p, run: () => { if (w.__quality) { w.__quality.auto = false; w.__quality.apply(p); } } })));   // (auto off for the visit: the ladder would move the preset again)
  // A FRAME CAPTURE (render/framecap.ts): every target the renderer wrote, whole, with the trace of what it changed (F6)
  panel.buttons(qual, 'capture', [
    { text: 'whole picture', run: () => w.__framecap?.start({}), title: 'every render target, whole, over 2 frames, as one file in the dev server\'s captures folder (F6). A big file' },
    { text: 'small window', run: () => w.__framecap?.start({ whole: false }), title: 'a window in the middle of the picture over 8 frames, a tenth the size. Shift+F6 takes it at the cursor' },
    { text: 'at next change', run: () => w.__framecap?.arm({}), title: 'waits, then takes the whole picture from the frame the renderer next changes something: a quality step, its size, its targets made again. For a flash that comes and goes' },
  ]);
  // THE TRIALS: each is made when the level loads (or is a const of the ray programs), so its button loads the page again
  if (lorry || shapes || hills) {
    const tr = panel.section('Trials', { open: true });
    if (lorry) panel.buttons(tr, 'lorry', [{ text: lorry.on ? 'curved: on' : 'curved: off', run: lorry.flip, title: 'the lorry on the pass with a bowed tilt, an arched cab roof, a round bonnet between fenders and round wheels. The page loads again to change it' }]);   // A LORRY WITH CURVES (scene/lorryTry.ts)
    if (shapes) panel.buttons(tr, 'shapes', [{ text: shapes.on ? 'round and broken: on' : 'round and broken: off', run: shapes.flip, title: 'round drums and tyres, a bite out of the yard wall’s top. The page loads again to change it' }]);   // A GEOMETRY TRIAL (scene/geoTry.ts)
    if (hills) panel.buttons(tr, 'ground', [{ text: hills.on ? 'hills cast shadows: on' : 'hills cast shadows: off', run: hills.flip, title: 'the snow’s hills stand in the light’s way as their own round shape. The page loads again to change it' }]);   // GROUND THAT RAYS CAN SEE, A TRIAL (scene/groundField.ts)
  }
  // PERFORMANCE: the F9 recorder and the F8 sweep (ui/perfrec.ts, ui/perfsweep.ts) from two buttons, with the look's layers as rows of the sweep, and the take
  // sent where a hosted page can send it (ui/perfsend.ts): a page in a sandbox starts no download, so a take made there never left it. The maintainer, 10-25:
  // "it'd be good to have a way to collect a lot of perf data so you can see how it runs on my laptop", and "i do worry that we have a few stuttering frames even
  // if overall fps is pretty high". Record play = every frame's time while he plays (the slow frames are listed one by one); Price the layers = the sweep, hands off.
  const perfRows = (): boolean => {   // (the page makes its recorder after this panel: asked for until it is there)
    const rec = w.__perfrec as PerfRecorder | undefined, sweep = w.__perfsweep as PerfSweep | undefined;
    if (!rec || !sweep) return false;
    {
      here(); const pf = panel.section('Performance', { open: true });
      const takes = row(pf, 'a take');
      const play = btn(takes, 'Record play', 'records every frame while you play; press again to stop and send. A minute or two with a fight in it says most');
      const price = btn(takes, 'Price the layers', 'hands off for a few minutes: the picture holds still and each layer is switched off in turn. The picture flickers. The prices are read from the take afterwards');
      // SHARP DIRECT OFF, for a take to set beside one with it on (the costliest single row on the maintainer's laptop, 10-25: 4.9 ms of 15.5 at High). What ?nosharp
      // does (main.ts), from a button, since a hosted page's address takes no flags: forced off for every preset the ladder puts on, and the preset put on again
      // so that its other resolve rows follow. The debugger's own row only holds until the next preset.
      const Q = w.__quality as { apply(name: string): void; current: string } | undefined;
      const sharp = btn(row(pf, 'sharp direct'), 'as the preset', 'direct light drawn at the screen\'s full size: the costliest single feature on a laptop. Off = a softer picture and a cheaper frame, whatever the quality goes to. For a recording to set beside one with it on');
      sharp.addEventListener('click', () => { if (!Q || sweep.active || rec.recording) return; sharpForce.v = sharpForce.v === false ? null : false; Q.apply(Q.current); sharp.textContent = sharpForce.v === false ? 'off' : 'as the preset'; });
      const said = words(pf);
      const box = document.createElement('textarea'); box.readOnly = true; box.hidden = true; box.style.cssText = 'width:100%;height:70px;margin-top:4px;box-sizing:border-box;font:10px/1.3 ui-monospace,Menlo,monospace;background:#161b1a;color:#cfd6d2;border:1px solid #3a4441;border-radius:3px'; pf.append(box);
      let copyMade = false;   // (the Copy row is made with the first read: a row of the panel's cannot be kept hidden, its search shows and hides every row)
      const copyRow = () => { if (copyMade) return; copyMade = true; const cp = btn(row(pf, 'the read'), 'Copy');
        cp.addEventListener('click', () => { const done = () => { cp.textContent = 'Copied'; setTimeout(() => { cp.textContent = 'Copy'; }, 1500); }; const old = () => { box.focus(); box.select(); }; if (navigator.clipboard?.writeText) navigator.clipboard.writeText(box.value).then(done, old); else old(); }); };
      const OFF: (keyof typeof lookAmounts)[] = ['size', 'sky', 'air', 'clarity', 'untile', 'lamps', 'glare', 'dark', 'gexp', 'contrast', 'highs', 'lows', 'colour', 'walls', 'even'];   // (not layers with a cost of their own to switch off; `even` is read once, when the maps are made; `walls` is a word of the page's address, compiled into the shaders at the start: moving its row draws nothing, so a take of it would read as free. Its cost is two loads, with and without ?look=walls:0)
      sweep.extra = [
        { id: 'look.all', label: 'look: every layer off (plain boxes)', flip: () => kept ? 'the boxes are shown plain already' : { from: { look: 1 }, to: { look: 0 }, apply: () => { plain(true); return () => plain(false); } } },
        ...ROWS.filter(([k]) => !OFF.includes(k)).map(([key, name]) => ({ id: `look.${key}`, label: `look: ${name} off`, flip: () => { const from = lookAmounts[key], to = key === 'ground' ? 1 : 0; if (kept) return 'the boxes are shown plain'; if (from === to) return 'already off';   // (the snow's relief is × the game's own: 1 is off, as plain boxes have it)
          return { from: { [key]: from }, to: { [key]: to }, apply: () => { lookAmounts[key] = to; send(); return () => { lookAmounts[key] = from; send(); }; } }; } })),
      ];
      const deliver = rec.deliver.bind(rec);
      rec.deliver = (dump: PerfDump) => {
        // the recorder's own way out (a download, the shot server on this machine) where no store is handed in. With a store the page is a hosted one that
        // starts no download: the try only raises the host's "downloads aren't available" notice over a take that did get out
        if ((window as unknown as { __takeStore?: unknown }).__takeStore) (window as unknown as { __lastPerfDump?: PerfDump }).__lastPerfDump = dump; else deliver(dump);
        const id = `perf-${dump.startedAt.replace(/[:.]/g, '-')}`, read = takeRead(dump, id), told = takeWords(read).join('\n');
        box.value = JSON.stringify(read); box.hidden = false; copyRow(); said.textContent = told;
        void sendTake(dump, read, t => { said.textContent = `${told}\n${t}`; }).then(t => { said.textContent = `${told}\n${t}`; said.scrollIntoView({ block: 'nearest' }); });
      };
      play.addEventListener('click', () => { if (sweep.active) return; if (rec.recording) rec.stop(); else { rec.start('look prototype: a play take (the panel)'); said.textContent = 'recording: play, then press Stop'; } });
      price.addEventListener('click', () => { if (sweep.active) { sweep.stop('the panel'); return; } if (rec.recording) return;
        const why = sweep.start({ only: [...sweep.extra.map(x => x.id), 'fog', 'sharp', 'reflect', 'meshNear', 'scaleDown'] }); if (!sweep.active) said.textContent = `not started (${why})`; });
      setInterval(() => { play.textContent = rec.recording && !sweep.active ? 'Stop and send' : 'Record play'; price.textContent = sweep.active ? `Stop (${sweep.progress})` : 'Price the layers'; }, 500);   // (its own clock, not the panel's refresh: the words must be right the moment the panel is opened again)
    }
    return true;
  };
  // THE CHARACTER LAB (host/charlab.ts): the camera down on you at a man's height, and you in any outfit of the cast. The maintainer, 10-25: "we should just
  // iterate on the characters in a character lab". Its rows show while it is open.
  const labRows = (): boolean => {   // (made after this panel, like the recorder)
    const lab = w.__charlab as { state: { on: boolean; yaw: number; pitch: number; dist: number; height: number; turn: boolean }; open(): void; leave(): void; wear(pick: string | number): string; worn(): string; outfits(): string[] } | undefined;
    if (!lab) return false;
    here(); const cl = panel.section('Character lab', { open: true });
    const open = btn(row(cl, 'the lab'), 'Open', 'the camera comes down to you and stays on you; nobody notices or hurts you. Walk into the light you want to judge a man in');
    const rows = document.createElement('div'); rows.hidden = true; cl.append(rows);   // (in the page before its rows are made: the panel files a row's refresh by the page it stands on)
    const wear = row(rows, 'outfit'), prev = btn(wear, '◀', 'the outfit before'), next = btn(wear, '▶', 'the next outfit'), worn = document.createElement('span'); worn.style.cssText = 'color:#e8efe9;margin-left:6px'; wear.append(worn);
    const S = lab.state;
    slide(rows, 'Angle', 'where the camera stands round you, in degrees', 0, 359, 1, () => S.yaw, v => { S.yaw = v; });
    slide(rows, 'Height', 'how steeply it looks down, in degrees', 2, 80, 1, () => S.pitch, v => { S.pitch = v; });
    slide(rows, 'Distance', 'metres from you', 1.2, 9, 0.1, () => S.dist, v => { S.dist = v; });
    slide(rows, 'Looks at', 'the height on you it looks at, in metres: 1.6 is the face, 0.2 the boots', 0.1, 1.8, 0.05, () => S.height, v => { S.height = v; });
    panel.toggleBox(rows, 'turn slowly round him', () => S.turn, v => { S.turn = v; });
    words(rows, 'You are the mannequin. Walk him into the sun, the shade or by a lamp.');
    const show = () => { rows.hidden = !S.on; open.textContent = S.on ? 'Leave' : 'Open'; worn.textContent = S.on ? lab.worn() : ''; };
    open.addEventListener('click', () => { if (S.on) lab.leave(); else lab.open(); show(); });
    prev.addEventListener('click', () => { lab.wear(-1); show(); }); next.addEventListener('click', () => { lab.wear(1); show(); });
    show();
    return true;
  };
  for (const made of [labRows, perfRows]) if (!made()) { const h = setInterval(() => { if (made()) clearInterval(h); }, 300); }
  panel.show(LOOK_SYSTEM);
}
