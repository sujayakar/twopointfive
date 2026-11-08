// The title screen, the pause card and the debrief: one panel (#menu) in three modes.
// TITLE (mode 'start'; the one starting page is the way in — ui/startpage.ts): the wordmark and the main menu down the left of the screen — PLAY (the
// starting page: every mission, its team and kit, deploy), OPTIONS, CONTROLS (ui/options.ts) — with the floor, and the attract reel, showing through on the
// right. Up from the first tick as the BOOT card (the same screen with a progress line while the shaders compile).
// PAUSE (Esc): the run's name and Resume / checkpoint / Restart / Options / Run so far / Quit down a narrow card; 'Run so far' opens the debrief of the run as
// it stands beside the menu (ui/debrief.ts). DEBRIEF: the mission's card proper — up on mission complete (Again / Keep playing) and, a beat after Sam drops, as
// the down card (Restart): the rating word and score, the numbers, the badges, the floor map with his path and the pins, the timeline. While any of them is
// open main.ts freezes the simulation and keeps rendering the frozen frame underneath. Plain DOM, styled in index.html (#menu…) with ui/theme.ts's tokens.
import { kitSolo } from '../game/kit';   // (the mission list's solo night off the start card: your card, on this game)
export { loadSandbox } from '../host/sandbox';
export { bootCard } from './title';
import { Engine } from '../engine';
import { Game } from '../game/game';
import { renderDebrief } from './debrief';
import { runEndOf } from '../host/gameend';
import { Quality } from './quality';
import { AudioEngine } from '../audio/audio';
import { Overlay } from './overlay';
import { MISSIONS, closeMissions, missionsBusy } from './missions';
import { nextPickOf } from './picks';
import { nextLessonOf } from './verdictNext';
import { openList } from './startpage';
import { menuTheme } from './theme';
import { keyText } from '../host/keymap';
import { openOptions, closeOptions, optionsOpen, setOptionsDeps, refreshOptions, type OptionsGame } from './options';
import { CMDRING } from '../game/cmdring';
import { WALLSNAP } from '../game/wallsnap';
import { cloudNote } from './cloudnote';
import { COOP } from '../net/lobby';
import { coopRows } from './coop';
import { TITLE, cheatWords, titleScreen } from './title';
import type { SlotInfo } from '../game/savestate';
import { button, checkpointButton, el, layStripAgain, saveStrip, type Press, type RunSaves } from './runcard';
import { locked } from '../view/seatview';

TITLE.coop = () => (COOP.on ? coopRows : null);

export interface MenuDeps { engine: Engine; game: Game; quality: Quality; audio: AudioEngine; overlay: Overlay; stopTour?: () => void; onStart: () => void; focusGame?: () => void;
  /** the benchmark (ui/bench.ts): Extras ▸ Benchmark on the live title begins it (a page load of its first scene: ui/benchrun.ts) */
  startBench?: () => void;
  /** checkpoints (game/savestate.ts): load the stored slot `key` into the game — main.ts stops any show first and snaps the camera after; returns why not, or null */
  loadCheckpoint?: (key: string) => string | null;
  /** F5 came on the pause card and the save will be written as play goes on (main.ts: the press is kept, the world is held) — the strip shows its tile to come */
  saveDue?: () => boolean;
  /** the scenario that is up right now (game/scenarioRun.ts — null: the plain night) in the picker's words, and the way out of it back to the start card's
   *  picker (main.ts: the scenario stopped, its flags off the URL). Absent: no scenario rows on the pause / down cards. */
  scenario?: { current: () => { title: string; team: string; saves?: boolean; entry?: { id: string } | null } | null; quit: () => void; refused?: () => string | null };
  /** the run just played, opened in the replay viewer in this window (ui/replaykey.ts openViewer, Shift+F7's work) — asked each time a card is drawn, since the
   *  replay key is made after the menu; null or absent: no viewer page or no buffer, and the end card has no Replay button */
  replay?: () => (() => void) | null; }

/** what Options sets of this page's game (ui/options.ts) */
function settingsOf(game: Game, overlay: Overlay): OptionsGame {
  return {
    get autosave() { return game.saves.enabled; }, set autosave(v) { game.saves.setEnabled(v); },
    get god() { return game.godMode; }, set god(v) { game.godMode = v; },
    get thinks() { return game.aiEnabled; }, set thinks(v) { game.aiEnabled = v; },
    get ammo() { return game.infiniteAmmo; }, set ammo(v) { game.infiniteAmmo = v; },
    get wallFeel() { return WALLSNAP.on; }, get wallPress() { return WALLSNAP.press; }, set wallPress(v) { WALLSNAP.press = v; },
    get fallIn() { return CMDRING.twice && !locked(game.view, 'twice'); },
    get hints() { return overlay.showLightDots; }, set hints(v) { overlay.showLightDots = v; },
  };
}
export class Menu {
  private root: HTMLElement; private body!: HTMLElement; private run = false;   // (run: the pause card's 'Run so far' is open beside its menu)
  mode: 'start' | 'pause' | 'debrief' | 'closed' = 'closed';
  constructor(private d: MenuDeps) {
    menuTheme();
    this.root = document.getElementById('menu')!;
    this.root.addEventListener('mousedown', e => e.stopPropagation());      // clicks in the menu never reach the game canvas
    this.root.addEventListener('contextmenu', e => e.preventDefault());
    setOptionsDeps({ engine: d.engine, quality: d.quality, audio: d.audio, game: settingsOf(d.game, d.overlay) });   // (ui/options.ts: the boot is over — its Graphics, Audio and Game tabs come alive, and what this machine kept is applied)
  }
  /** main.ts hands the scenario hooks over once the scenario is started (after this card is constructed) */
  setScenario(s: MenuDeps['scenario']) { this.d.scenario = s; }
  get isOpen() { return this.mode !== 'closed'; }
  /** the start mode is the TITLE SCREEN (the one starting page is the way in) */
  private get titled() { return this.mode === 'start'; }
  /** Auto's calibration has just settled (main.ts wires Quality.onCalibrated): the options screen's live line follows, if it is the one up */
  refreshQualityNote() { refreshOptions(); }

  open(mode: 'start' | 'pause' | 'debrief') { if (mode === 'debrief') closeOptions(); this.mode = mode; MISSIONS.startSolo = mode === 'start' ? (card) => { this.close(); if (card) kitSolo(this.d.game); } : null; this.render(); this.root.classList.add('on'); }   // (the mission list's solo night off the start card IS Start)
  close() { closeOptions(); MISSIONS.startSolo = null; closeMissions(); if (this.mode === 'start') this.d.onStart(); this.mode = 'closed'; this.run = false; this.root.classList.remove('on'); this.d.focusGame?.(); }   // keys go to the game, not the (now hidden) button that had focus
  private stripAt: HTMLElement | null = null;   // (the pause card's node the strip of saves sits before)
  /** F5 came on the pause card (main.ts `saveDue`): the strip alone is laid again, with the tile to come — the card and where the keys had walked on it stay */
  saveAsked() {
    if (this.mode === 'pause') layStripAgain(this.root, on => this.saveStrip(undefined, on), this.stripAt);
  }
  toggle() { if (this.titled) return; if (this.isOpen) this.close(); else this.open('pause'); }   // (Esc: on the title screen there is nothing to back out of — the solo night is a row of the starting page)

  private render() {
    const r = this.root; r.innerHTML = '';
    const titled = this.titled, pause = this.mode === 'pause';
    if (titled) r.classList.add('title'); else r.classList.remove('title');
    if (pause) r.classList.add('paused'); else r.classList.remove('paused');
    const card = el('div', 'card'); r.appendChild(card);
    if (this.mode === 'debrief') { card.classList.add('wide'); this.debrief(card); return; }
    if (pause) { this.pause(card); return; }
    this.title(card);
  }

  /** THE TITLE SCREEN (ui/title.ts) */
  private title(card: HTMLElement) {
    // A MISSION REFUSED FOR 'NO AI' IS TRIED AGAIN when Options closes with that cheat off (off a tape of play, 10-13: the kept switch refused a rescue at boot, he
    // switched it off and was on a floor with no mission while the card still said 'did not start'): its own link is loaded again, as Deploy would — a start is a
    // page load, and the switch has just been kept. Any other refusal stays said.
    const g = this.d.game, set = settingsOf(g, this.d.overlay), stuck = !g.aiEnabled && !!this.d.scenario?.refused?.();
    titleScreen(card, { cheats: () => cheatWords(set), closed: moved => { if (stuck && g.aiEnabled && typeof location !== 'undefined') { location.reload(); return; } if (this.mode === 'start' && moved) this.render(); },   // (the chip follows what Options ▸ Game did)
      shows: { bench: () => { const sb = this.d.startBench; if (!sb) return false; sb(); return true; } },
      said: c => this.refused(c), note: cloudNote });   // (ui/cloudnote.ts: a hosted build's one line on what it sends; nothing on a build with no backend)
  }

  /** THE PAUSE CARD: what is paused, then the menu — Resume first; 'Run so far' opens the debrief of the run as it stands beside it */
  private pause(card: HTMLElement) {
    const scn = this.d.scenario?.current() ?? null;   // a scenario is up under the pause card: it is named, with its team, and can be quit to the missions
    const side = el('div', 'side'); card.appendChild(side);
    const head = el('div', 'head');
    head.innerHTML = `<div class="t1">two<b>point</b>six</div><div class="t2">${scn ? `paused · ${esc(scn.title)} · ${esc(scn.team)}` : 'paused'}</div>`;
    side.appendChild(head);
    const actions = el('div', 'actions'), gap = () => actions.appendChild(el('div', 'gap'));
    const resume = button('Resume', () => this.close(), 'primary');
    actions.appendChild(resume);
    { const ck = this.checkpointButton(); if (ck) actions.appendChild(ck); const strip = this.saveStrip(); if (strip) actions.appendChild(strip); }   // 'Restart from checkpoint (40 s ago)' when one is stored (game/savestate.ts) — under Resume here; on the down card it is THE default
    actions.appendChild(this.stripAt = button(scn ? 'Restart scenario' : 'Restart encounter', () => { this.d.stopTour?.(); this.d.game.restart(); this.close(); }));   // (a running tour or level-script program ends first: its beat must not keep puppeting the fresh encounter; under a scenario the restart stages it again)
    gap();
    actions.appendChild(button('Options', () => openOptions()));
    actions.appendChild(button('Run so far', () => { this.run = !this.run; this.render(); }, this.run ? 'on' : ''));
    gap();
    if (scn) actions.appendChild(this.quitButton()); else actions.appendChild(button('Missions', () => { openList({ backLabel: '‹ Back' }); }));   // the level select (ui/missions.ts): every scenario and the solo night; a pick is a page load
    { const nx = this.nextPickButton(); if (nx) actions.appendChild(nx); }   // (a run a pick started: the next one on the short list, one press away)
    side.appendChild(actions);
    const foot = el('div', 'foot'); foot.textContent = 'Esc resumes · W S choose'; side.appendChild(foot);
    this.body = el('div', 'body');
    if (this.run) { card.classList.add('wide'); card.appendChild(this.body); renderDebrief(this.body, runEndOf(this.d.game)); }   // the debrief of the run as it stands (nothing to press but 'copy summary': Resume / Restart are the card's own)
    setTimeout(() => { if (!missionsBusy() && !optionsOpen()) resume.focus(); }, 0);
  }
  /** the URL named a scenario and its start() said no: the reason, where the player is looking */
  private refused(card: HTMLElement) { const why = this.d.scenario?.refused?.() ?? null; if (why) { const r = el('div', 'refused'); r.textContent = why; card.appendChild(r); } }

  /** The debrief card (main.ts opens it when the game says the end card is due): on MISSION COMPLETE — Again = a fresh encounter (and mission), Keep playing / Esc =
   *  back to the sandbox with the mission left done — and, a beat after Sam goes DOWN, as the down card — Restart (or Esc back to the floor, where Enter restarts
   *  as it always has). The contents are ui/debrief.ts's: rating + score, the numbers, the badges, the map, the timeline. */
  private debrief(card: HTMLElement) {
    const g = this.d.game; const V = g.verdict?.result ?? null; const down = V ? !V.won || g.player.down : g.mission.stage !== 'done';   // (won or lost is the run's verdict where there is one — a squad's mission never reaches the stealth game's 'done' — game/verdict.ts)
    const shown = performance.now();
    const restart = () => { if (performance.now() - shown < 500) return; this.d.stopTour?.(); g.restart(); this.close(); };   // (armed after half a second: the card appears mid-stride, often a Space-tap after the fire door — that tap must not restart the run unseen)
    const info = down ? this.checkpointInfo() : null; const mine = !!info && g.saves.isThisRun(info);   // DOWN with a checkpoint of THIS run stored: 'Restart from checkpoint (N s ago)' is the default (focused, Enter — the bare key does the same before the card is up); an earlier run's is offered but never the default
    const cur = this.d.scenario?.current() ?? null, scn = !!cur;   // down under a scenario: the restart stages it again, and the picker is one press away (no checkpoint button: none is written with a team on the floor, and a solo run's is refused there — but a scenario that SAVES, a quiet mission alone, offers its own)
    const ck = info ? this.checkpointButton(mine ? 'primary' : '', () => performance.now() - shown >= 500) : null;
    const strip = down ? this.saveStrip(() => performance.now() - shown >= 500) : null;   // (this run's saves laid out in time: any of them is one press away, the default stays the newest sound one)
    const nl = V && V.won && !down && g.verdict.kept?.lesson ? nextLessonOf(cur?.entry?.id) : null;   // a Training lesson done (the row that is UP, as the books read it — main.ts vBooks): the next lesson leads the card (ui/verdictNext.ts)
    const lead = nl ? button(`Next: ${nl.title} ›`, () => { if (performance.now() - shown < 500) return; this.d.stopTour?.(); nl.start(); }, 'primary next') : null;
    const primary = button(down ? (scn ? 'Restart scenario' : 'Restart encounter') : 'Again', restart, (ck && mine) || lead ? '' : 'primary');
    const actions: HTMLElement[] = down ? (ck ? (mine ? [ck, primary] : [primary, ck]).concat([button('Back to the floor', () => this.close())]) : [primary, button('Back to the floor', () => this.close())]) : [primary, button('Keep playing', () => this.close())];
    if (strip) actions.push(strip);
    if (lead && nl) { lead.dataset.href = nl.href; actions.unshift(lead); }
    if (scn) actions.push(this.quitButton(() => performance.now() - shown >= 500));
    { const nx = this.nextPickButton(() => performance.now() - shown >= 500); if (nx) actions.push(nx); }
    { const view = this.d.replay?.() ?? null; if (view) { const b = button('Replay', () => { if (performance.now() - shown >= 500) view(); }, 'replay'); b.title = 'watch this run again in the game'; actions.push(b); } }   // (the maintainer: "would be nice to be able to open the replay immediately here")
    this.body = el('div', 'body'); card.appendChild(this.body);
    renderDebrief(this.body, runEndOf(g), actions);
    const foot = el('div', 'foot'); foot.textContent = keyText(down ? (ck && mine ? 'Press Enter to go back to the checkpoint · press Shift+Enter to restart the mission · press Esc to close this' : 'Press Esc to close this · press Enter or Shift+Enter to restart') : 'Press Esc to keep playing · press Shift+Enter to restart any time · Esc ▸ Run so far shows this for a run in progress'); card.appendChild(foot);
    setTimeout(() => (lead ?? (ck && mine ? ck : primary)).focus(), 0);
  }
  /** 'Restart from checkpoint (40 s ago)' for the newest stored checkpoint (this run's for choice), or null when there is none / no loader is wired: loads it
   *  through main.ts (any show stopped, the camera snapped) and closes the card; a refusal goes on the message line and re-renders the card. `armed`: the down
   *  card's half-second guard against the tap that raised it. */
  private checkpointButton(cls = '', armed: () => boolean = () => true): HTMLElement | null { const run = this.saves(); return run && checkpointButton(run, this.press(armed), cls); }
  private press(armed: () => boolean): Press { return { armed, gone: () => this.close(), again: () => this.render() }; }
  /** this run's saves as a card asks of them (ui/runcard.ts) */
  private saves(): RunSaves | null {
    const g = this.d.game, load = this.d.loadCheckpoint; if (!load) return null;
    return { elapsed: () => g.mission.stats.elapsed, timeline: () => g.saves.timeline(), offer: () => this.checkpointInfo(), age: row => g.saves.ageWords(g, row as SlotInfo), picture: row => g.saves.pictureOf(row as SlotInfo), load, due: () => this.mode === 'pause' && !!this.d.saveDue?.() };
  }
  /** 'Quit to scenarios' (the pause card and the down card while a scenario is up): the scenario stopped and the start card's picker back (main.ts) */
  private quitButton(armed: () => boolean = () => true): HTMLElement { return button('Quit to scenarios', () => { if (!armed()) return; this.d.stopTour?.(); this.d.scenario?.quit(); }, 'quit'); }
  /** 'Next: <title>' while the page is a run the short list started (scene/picks.ts pickOfQuery) and a row follows it: that row's own page load. Null otherwise. */
  private nextPickButton(armed: () => boolean = () => true): HTMLElement | null {
    const nx = nextPickOf(); if (!nx) return null;
    if (nx.scenario !== !!this.d.scenario?.current()) return null;   // (the URL is a pick's, but what is up is not it: its scenario was refused at boot and the plain night is being played — or a scenario is up on the solo pick's page)
    return button(`Next: ${nx.title} ›`, () => { if (!armed()) return; this.d.stopTour?.(); nx.start(); }, 'next');
  }
  /** the slot the cards offer: this run's newest at or before now on the mission clock, else an earlier run's newest (null: none / no loader) */
  private checkpointInfo() {
    const g = this.d.game; if (!this.d.loadCheckpoint) return null;
    const scn = this.d.scenario?.current() ?? null, n = g.saves.offer(g, g.mission.stats.elapsed);
    if (scn && !scn.saves) return n && g.saves.isThisRun(n) ? n : null;   // (a scenario with a team: no file is written there and a solo run's is refused — but this run's own saves, kept in memory, are offered)
    return n;
  }   // (savestate.ts offer: an earlier run's slot only if this game would load it)
  /** this run's saves laid out in time (ui/runcard.ts) */
  private saveStrip(armed: () => boolean = () => true, on?: string): HTMLElement | null { const run = this.saves(); return run && saveStrip(run, this.press(armed), on); }

  /** a save's picture came after its tile was laid (main.ts takes it a frame or a few after the save, host/savepic.ts): the tile on the card, if one is up, takes it — nothing is laid again */
  pictureCame(info: { key: string; savedAt: number }, url: string) {
    for (const t of this.root.querySelectorAll<HTMLElement>('.saves .tile')) if (t.dataset.save === `${info.key}@${info.savedAt}`) t.style.backgroundImage = `url("${url}")`;
  }
}

// ---- tiny DOM helpers (menu-local styling lives in index.html)
function esc(t: string) { return t.replace(/[&<>"]/g, c => c === '&' ? '&amp;' : c === '<' ? '&lt;' : c === '>' ? '&gt;' : '&quot;'); }
