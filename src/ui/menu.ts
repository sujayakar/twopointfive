// Start card + pause menu (Esc): the same panel in two modes. Start: title, the controls at a glance, a Start button (and the
// tour). Pause: Resume / Restart / Tour plus two tabs — a friendly Settings page (a curated handful of the tweak panel's
// hundred knobs) and the full Controls list. A third mode is the mission's debrief card (time, what the guards noticed, a Chaos-Theory
// style rating; Again / Keep playing). While it is open main.ts freezes the simulation and keeps rendering the frozen frame
// underneath. Plain DOM, styled in index.html (#menu…).
import { Engine } from '../engine';
import { Game, missionRating, fmtClock } from '../game/game';
import { Quality, QUALITY_NAMES, QualityName } from './quality';
import { AudioEngine } from '../audio/audio';
import { Overlay } from './overlay';

export const CONTROLS: [string, string][] = [
  ['W A S D', 'move (camera-relative) · Shift sprint · C / Ctrl-tap crouch · Alt walk'],
  ['Mouse', 'aim · LMB fire (1) / throw (2, 3 — arc shown while selected) · RMB raise weapon'],
  ['F', 'the marker under the cursor — OCP a light (also MMB), open / crack / shut a door (locked: hold F to pick it quietly · tap F standing, or sprint into it, to kick it in loud · tap F crouched just tries it), take a guard down from behind, drag or drop a body, pull the drive'],
  ['1 2 3', 'Five-seveN · smoke canister · stun canister · G quick-throw'],
  ['R  L  N', 'reload · rail light · night vision'],
  ['Q E · wheel', 'rotate the camera · zoom'],
  ['Esc', 'pause · settings · this list'],
  ['Enter', 'restart the encounter when you are down (Shift+Enter any time)'],
  ['P', 'hands-off showcase tour'],
  ['Tab  `  [ ]', 'tweak panel · debug views · exposure'],
];

export interface MenuDeps { engine: Engine; game: Game; quality: Quality; audio: AudioEngine; overlay: Overlay; startTour: () => void; stopTour?: () => void; onStart: () => void; focusGame?: () => void; }

export class Menu {
  private root: HTMLElement; private body!: HTMLElement; private tab: 'settings' | 'controls' = 'settings';
  mode: 'start' | 'pause' | 'debrief' | 'closed' = 'closed';
  constructor(private d: MenuDeps) {
    this.root = document.getElementById('menu')!;
    this.root.addEventListener('mousedown', e => e.stopPropagation());      // clicks in the menu never reach the game canvas
    this.root.addEventListener('contextmenu', e => e.preventDefault());
  }
  get isOpen() { return this.mode !== 'closed'; }

  open(mode: 'start' | 'pause' | 'debrief') { this.mode = mode; this.render(); this.root.classList.add('on'); }
  close() { if (this.mode === 'start') this.d.onStart(); this.mode = 'closed'; this.root.classList.remove('on'); this.d.focusGame?.(); }   // keys go to the game, not the (now hidden) button that had focus
  toggle() { if (this.isOpen) this.close(); else this.open('pause'); }

  private render() {
    const r = this.root; r.innerHTML = '';
    const card = el('div', 'card'); r.appendChild(card);
    if (this.mode === 'debrief') { this.debrief(card); return; }
    const head = el('div', 'head');
    head.innerHTML = `<div class="t1">two<b>point</b>six</div><div class="t2">${this.mode === 'start' ? 'a ray-traced stealth sandbox — every light, shadow and wisp of smoke is live' : 'paused'}</div>`;
    card.appendChild(head);
    const actions = el('div', 'actions');
    const primary = button(this.mode === 'start' ? 'Start' : 'Resume', () => this.close(), 'primary');
    actions.appendChild(primary);
    actions.appendChild(button('Showcase tour', () => { this.close(); this.d.startTour(); }));
    if (this.mode === 'pause') actions.appendChild(button('Restart encounter', () => { this.d.stopTour?.(); this.d.game.restartEncounter(); this.close(); }));   // (a running tour ends first: its beat must not keep puppeting the fresh encounter)
    card.appendChild(actions);
    if (this.mode === 'start') { const n = el('div', 'note'); n.textContent = 'First time here? The three-minute showcase tour walks through the renderer and the stealth systems — a silent takedown, a firefight, a smoke screen, a blackout — then hands you a fresh encounter.'; card.appendChild(n);
      const j = el('div', 'note'); j.textContent = 'The job: get into the server room, pull the drive from the marked rack (F), leave by the fire exit. The line at the top says what is next; the guards keep the score.'; card.appendChild(j); }
    if (this.mode === 'pause') {
      const tabs = el('div', 'tabs');
      for (const t of ['settings', 'controls'] as const) { const b = button(t === 'settings' ? 'Settings' : 'Controls', () => { this.tab = t; this.render(); }, this.tab === t ? 'tab on' : 'tab'); tabs.appendChild(b); }
      card.appendChild(tabs);
    }
    this.body = el('div', 'body'); card.appendChild(this.body);
    if (this.mode === 'start' || this.tab === 'controls') this.controls(this.mode === 'start');
    else this.settings();
    const foot = el('div', 'foot'); foot.textContent = this.mode === 'start' ? 'Esc pauses · Tab opens every knob' : 'Esc to resume'; card.appendChild(foot);
    setTimeout(() => primary.focus(), 0);
  }

  /** End card once the player is out with the drive (main.ts opens it on game.mission.debrief): the run in numbers and Chaos Theory's rating word.
   *  Again = a fresh encounter (and mission); Keep playing / Esc = back to the sandbox with the mission left done. */
  private debrief(card: HTMLElement) {
    const s = this.d.game.mission.stats; const rating = missionRating(s);
    const blurb = rating === 'ghost' ? 'never seen, no alarm raised, nobody shot' : rating === 'panther' ? 'they know someone was there — nobody ever saw who' : 'they had eyes on you';
    const head = el('div', 'head');
    head.innerHTML = `<div class="t1">exfiltrated</div><div class="t2">the drive is out of the building · <b>${rating}</b> — ${blurb}${s.sandbox ? ' · <i>sandbox flags were on</i>' : ''}</div>`;
    card.appendChild(head);
    const actions = el('div', 'actions'); const shown = performance.now();
    const again = button('Again', () => { if (performance.now() - shown < 500) return; this.d.stopTour?.(); this.d.game.restartEncounter(); this.close(); }, 'primary');   // (armed after half a second: the card appears mid-stride, often a Space-tap after the fire door — that tap must not restart the run unseen)
    actions.append(again, button('Keep playing', () => this.close()));
    card.appendChild(actions);
    this.body = el('div', 'body'); card.appendChild(this.body);
    const rows: [string, string][] = [
      ['time', fmtClock(s.elapsed)],
      ['alerts raised', String(s.alerts)],
      ['times spotted', String(s.spotted)],
      ['bodies found', String(s.bodies)],
      ['shots fired', String(s.shots)],
      ['kills · takedowns', `${s.kills} · ${s.takedowns}`],
      ['blackout', s.blackout ? 'used' : 'no'],
    ];
    const list = el('div', 'controls');
    for (const [k, v] of rows) { const row = el('div', 'row'); const kk = el('div', 'k'); kk.textContent = k; const vv = el('div', 'v'); vv.textContent = v; row.append(kk, vv); list.appendChild(row); }
    this.body.appendChild(list);
    const foot = el('div', 'foot'); foot.textContent = 'Esc keeps playing · Shift+Enter restarts any time'; card.appendChild(foot);
    setTimeout(() => again.focus(), 0);
  }

  private controls(brief: boolean) {
    const list = el('div', 'controls');
    for (const [k, v] of brief ? CONTROLS.slice(0, 8) : CONTROLS) { const row = el('div', 'row'); const kk = el('div', 'k'); kk.textContent = k; const vv = el('div', 'v'); vv.textContent = v; row.append(kk, vv); list.appendChild(row); }
    this.body.appendChild(list);
  }

  private settings() {
    const { engine, game, quality, audio, overlay } = this.d; const S = engine.settings; const P = engine.passes;
    const g1 = group(this.body, 'Picture');
    select(g1, 'Quality', ['auto', ...QUALITY_NAMES], () => quality.auto ? 'auto' : quality.current, v => { if (v === 'auto') quality.setAuto(); else quality.pin(v as QualityName); this.render(); }, 'auto picks the preset for this machine from the measured frame rate (and remembers it); or pin one: resolution cap, beam steps, cache density, smoke solver iterations');
    if (quality.auto) { const n = el('div', 'note'); n.textContent = `auto is currently on “${quality.current}”${quality.lastChange.startsWith('auto') ? ' — ' + quality.lastChange : ''}`; g1.appendChild(n); }
    toggle(g1, 'Adaptive resolution', () => quality.adaptive, v => { quality.adaptive = v; }, 'steps the internal resolution down if the frame rate sinks under ~27 fps');
    slider(g1, 'Exposure', 0.5, 4, 0.05, () => S.exposure, v => { S.exposure = v; });
    slider(g1, 'Bloom', 0, 0.3, 0.01, () => P.bloom, v => { P.bloom = v; });
    slider(g1, 'Film grain', 0, 0.08, 0.002, () => P.grain, v => { P.grain = v; });
    slider(g1, 'Night-vision phosphor', 0, 1, 0.05, () => P.nvPhosphor, v => { P.nvPhosphor = v; }, '0 white (modern) → 1 classic green');
    toggle(g1, 'Interaction markers', () => overlay.showLightDots, v => { overlay.showLightDots = v; }, 'the faint dots on lights, doors and bodies');
    // cheaper-but-lossy renderer paths, all off by default and independent (each is its own engine setting; the Tab panel has the same switches
    // next to the pass they change): what each buys and what it costs, honestly, so they can be judged one at a time
    const gt = group(this.body, 'Graphics — trade-offs');
    note(gt, 'the quality presets now set these (ultra: none · high: third-res bounce, checkerboard rays, dim-pixel rays · medium / low: all five) — flip them here to compare; picking a preset again resets them');
    toggle(gt, 'Checkerboard shadow rays', () => S.checkerDirect, v => { S.checkerDirect = v; }, 'direct light: every frame only half the pixels (a checkerboard that flips each frame) trace shadow rays; the others keep their exact unshadowed light and borrow the shadowing of their traced neighbours on the same surface — hard shadow edges and pixels with no usable neighbour still trace');
    note(gt, 'measured on the reference machine (cubicles, high, 1777×1000): direct 9.8 → 9.0 ms, invisible at rest · costs a frame of softness on narrow shadow edges in fast motion');
    toggle(gt, 'Bounce light at third resolution', () => S.gatherThird, v => { S.gatherThird = v; }, 'the final gather (4 traced rays per pixel, its history and its blur) runs at a third of the resolution instead of half; the picture is rebuilt with the same depth/normal-aware upsample');
    note(gt, 'measured: gather 6.6 → 3.2 ms, the frame 24.5 → 20.7 ms — the biggest lever here, and hard to see at normal exposure · costs softer, slightly blurrier contact darkening and colour bleeding');
    toggle(gt, 'Bounce cache at half rate', () => S.rcHalfRate, v => { S.rcHalfRate = v; }, 'the world-space radiance cache behind the bounce light (four cascades of probe rays + the irradiance bake) is refreshed over two frames instead of every frame');
    note(gt, 'measured: cascades + bake 3.0 → 2.8 ms averaged (they are already cheap on this machine; more on weaker GPUs) · costs bounce light, the light meter and smoke ambient trailing a moving light by a frame');
    toggle(gt, 'Beams and smoke at quarter resolution', () => S.volQuarter, v => { S.volQuarter = v; }, 'the volumetric pass (haze beams with their shadow rays, the smoke march) runs at a quarter of the resolution instead of half and is upsampled depth-aware as before');
    note(gt, 'measured: volumetrics 0.8 → 0.3 ms in a low-haze interior, more with smoke on screen · costs softer beam edges and blockier smoke silhouettes against geometry');
    toggle(gt, 'Smoke pressure at half resolution', () => !!game.smoke.params?.pressureHalf, v => { if (game.smoke.params) game.smoke.params.pressureHalf = v; }, 'the smoke solver\'s pressure step (what makes the gas push the air aside instead of piling up) is solved on a 32³ grid per smoke volume instead of 64³ and interpolated back; advection, swirl, buoyancy and everything the renderer samples stay at full resolution. Honest number: on the reference GPU the smoke pass only drops ~4–10 % (the sweeps were already cheap after empty-brick skipping; the extra dispatches eat most of it) — may matter more on slower GPUs; plumes keep their shape, lose a little fine curl');
    note(gt, 'not yet measured here: expected to roughly halve the pressure sweeps, the biggest part of the smoke step (which only costs anything while smoke is live) · costs fine swirl detail — softer billows at the canister mouth, slightly puffier plumes');
    toggle(gt, 'Fewer shadow rays in near-black', () => S.dimRays, v => { S.dimRays = v; }, 'direct light: where all the light that could reach a pixel adds up to less than about 2 % of white on screen, its shadows are sampled with one ray per light instead of the full budget (threshold in the Tab panel)');
    note(gt, 'measured: nothing in a lit view (by design); it only pays in dark rooms and deep shadow · costs slightly grainier soft shadows in the near-black, mostly under the tonemap');
    // the existing per-pixel shadow-ray budget (settings.directSamplesTop / Other / AdaptiveMin — the quality presets' `ds`), as one friendly select
    const BUDGETS: { label: string; ds: [number, number, number] }[] = [ { label: '8 / 4 (high)', ds: [8, 4, 3] }, { label: '6 / 3 (medium)', ds: [6, 3, 3] }, { label: '4 / 2 (low)', ds: [4, 2, 2] } ];
    const budgetNow = () => BUDGETS.find(b => b.ds[0] === S.directSamplesTop && b.ds[1] === S.directSamplesOther && b.ds[2] === S.directAdaptiveMin)?.label ?? 'custom (Tab panel)';
    select(gt, 'Shadow rays per pixel', [...BUDGETS.map(b => b.label), ...(budgetNow().startsWith('custom') ? ['custom (Tab panel)'] : [])], budgetNow, v => { const b = BUDGETS.find(x => x.label === v); if (b) { S.directSamplesTop = b.ds[0]; S.directSamplesOther = b.ds[1]; S.directAdaptiveMin = b.ds[2]; } this.render(); }, 'stratified emitter samples for the strongest light / the other leaders per pixel (and how many agreeing samples end the adaptive loop early: 3, 3, 2). The full budget is only ever spent inside penumbrae');
    note(gt, 'the quality presets already set this (high 8/4, medium 6/3, low 4/2) — picking a preset again overrides it · fewer rays = grainier soft-shadow edges, roughly halving the direct pass from 8/4 to 4/2');
    const g2 = group(this.body, 'Sound');
    slider(g2, 'Master volume', 0, 1, 0.05, () => audio.volume.master, v => { audio.volume.master = v; audio.setVolumes(); });
    slider(g2, 'Effects', 0, 1, 0.05, () => audio.volume.sfx, v => { audio.volume.sfx = v; audio.setVolumes(); });
    slider(g2, 'Music', 0, 1, 0.05, () => audio.volume.music, v => { audio.volume.music = v; audio.setVolumes(); });
    toggle(g2, 'Music on', () => audio.music?.enabled ?? true, v => { if (audio.music) audio.music.enabled = v; });
    const g3 = group(this.body, 'Game');
    toggle(g3, 'Guards think (AI)', () => game.aiEnabled, v => { game.aiEnabled = v; });
    toggle(g3, 'God mode', () => game.godMode, v => { game.godMode = v; }, 'they still see and shoot you; hits do nothing');
    toggle(g3, 'Unlimited ammo', () => game.infiniteAmmo, v => { game.infiniteAmmo = v; });
  }
}

// ---- tiny DOM helpers (menu-local styling lives in index.html)
function el(tag: string, cls?: string) { const e = document.createElement(tag); if (cls) e.className = cls; return e; }
function button(text: string, run: () => void, cls = '') { const b = el('button', 'btn ' + cls) as HTMLButtonElement; b.textContent = text; b.onclick = run; return b; }
function group(parent: HTMLElement, title: string) { const g = el('div', 'group'); const h = el('div', 'gtitle'); h.textContent = title; g.appendChild(h); parent.appendChild(g); return g; }
function note(parent: HTMLElement, text: string) { const n = el('div', 'note'); n.textContent = text; parent.appendChild(n); return n; }
function rowOf(parent: HTMLElement, label: string, title?: string) { const r = el('div', 'srow'); const l = el('label'); l.textContent = label; if (title) r.title = title; r.appendChild(l); parent.appendChild(r); return r; }
function slider(parent: HTMLElement, label: string, min: number, max: number, step: number, get: () => number, set: (v: number) => void, title?: string) {
  const r = rowOf(parent, label, title); const i = el('input') as HTMLInputElement; i.type = 'range'; i.min = String(min); i.max = String(max); i.step = String(step); i.value = String(get());
  const val = el('span', 'val'); val.textContent = fmt(get(), step);
  i.oninput = () => { set(parseFloat(i.value)); val.textContent = fmt(get(), step); }; r.append(i, val);
}
function toggle(parent: HTMLElement, label: string, get: () => boolean, set: (v: boolean) => void, title?: string) {
  const r = rowOf(parent, label, title); const i = el('input') as HTMLInputElement; i.type = 'checkbox'; i.checked = get(); i.onchange = () => set(i.checked); r.appendChild(i);
}
function select(parent: HTMLElement, label: string, opts: string[], get: () => string, set: (v: string) => void, title?: string) {
  const r = rowOf(parent, label, title); const s = el('select') as HTMLSelectElement;
  for (const o of opts) { const oe = el('option') as HTMLOptionElement; oe.value = o; oe.textContent = o; s.appendChild(oe); }
  s.value = get(); s.onchange = () => set(s.value); r.appendChild(s);
}
function fmt(v: number, step: number) { const d = step >= 1 ? 0 : step >= 0.1 ? 1 : step >= 0.01 ? 2 : 3; return v.toFixed(d); }
