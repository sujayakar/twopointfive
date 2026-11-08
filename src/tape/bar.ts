// THE BAR OVER A RUN BEING PLAYED AGAIN: back to the game, play and pause, the pace, the clock, the line it is sought on (the notes and the tries stand on it), the run's own view, notes. The run is whoever
// plays it (host/drawplay.ts, through `Played`); nothing here is the game's.
import { clock, type ReplayNote } from './format';
import { rectZoom, textZoom, TEXT_EVENT } from '../ui/textsize';
import type { FreeCam } from './freecam';
import type { TapeMap } from './map';
import { speedName, speedStep, TAPE_SPEEDS } from './pace';

/** a run being played again, as a bar turns it */
export interface Played {
  paused: boolean; pace: number;
  /** the tick it stands after · how many there are · a place asked for is being played to */
  readonly at: number; readonly total: number; readonly seeking: boolean;
  /** where it is first not the run that was played · the walk was lost to a panic, its first line · the ticks the mission began again after */
  readonly parted: number | null; readonly lost: string | null; readonly tries: readonly number[];
  seek(tick: number): void;
}
/** how whoever watches sees it: the fog on the picture, and through a tube or not (null: as the run had it) */
export interface Sees { readonly fog: boolean; readonly tube: boolean | null; /** there is a fog to take off · the picture's tube is on · the run's is */ readonly fogged: boolean; readonly lit: boolean; readonly runs: boolean; set(fog: boolean, tube: boolean | null): void }
export interface BarDeps {
  played: Played; sees: Sees; map: TapeMap; cam: FreeCam; canvas: HTMLCanvasElement;
  /** what is being played, in a word */
  title: string;
  notes: ReplayNote[];
  /** keep the notes; null: a replay nothing is kept of. False = this browser would not */
  keep: ((notes: ReplayNote[]) => Promise<boolean>) | null;
  back(): void;
  /** the first press: sound may start */
  begun(): void;
}
const HZ = 60, STEP = 10 * HZ;
const CSS = `
#tapebar{position:fixed;left:0;right:0;bottom:0;z-index:70;display:flex;align-items:center;gap:10px;padding:8px 14px;background:var(--h-field,rgb(21,19,16));border-top:1px solid rgba(233,226,214,.14);color:#e9e2d6;font:12px/1.2 var(--m-head,system-ui),system-ui,sans-serif;letter-spacing:.04em;user-select:none}
#tapebar button{all:unset;cursor:pointer;padding:5px 9px;border:1px solid rgba(233,226,214,.28);border-radius:3px;color:inherit;text-transform:uppercase}
#tapebar button:hover{border-color:var(--m-accent,#e8833a);color:var(--m-accent,#e8833a)}
#tapebar button.on{color:var(--m-accent,#e8833a);background:rgba(232,131,58,.16);box-shadow:inset 0 -2px 0 var(--m-accent,#e8833a)}
#tapebar [hidden]{display:none}
#tapebar .seg{display:flex;border:1px solid rgba(233,226,214,.28);border-radius:3px;overflow:hidden}
#tapebar .seg button{border:0;border-left:1px solid rgba(233,226,214,.28);border-radius:0;min-width:.8em;text-align:center}
#tapebar .seg button:first-child{border-left:0}
#tapebar .seg .x{min-width:1.7em;font-variant-numeric:tabular-nums}
#tapebar .seg button.end{color:rgba(233,226,214,.25);cursor:default}
#tapebar .gap{width:1px;align-self:stretch;margin:2px 2px;background:rgba(233,226,214,.14)}
#tapebar .pp{min-width:1.2em;text-align:center}
#tapebar .time{font-variant-numeric:tabular-nums;min-width:8.5em;text-align:center}
#tapebar .line{position:relative;flex:1;height:28px;cursor:pointer;touch-action:none;background:rgba(233,226,214,.07)}
#tapebar .fill{position:absolute;left:0;top:0;height:100%;min-width:2px;border-right:2px solid var(--h-hi,#f0e9d9);border-bottom:2px solid var(--m-accent,#e8833a);box-sizing:border-box;pointer-events:none}
#tapebar .fill::after{content:'';position:absolute;right:-5px;top:-4px;width:8px;height:8px;border-radius:50%;background:var(--h-hi,#f0e9d9)}
#tapebar .try{position:absolute;top:0;width:1px;height:100%;background:rgba(233,226,214,.45);pointer-events:none}
#tapebar .tk{position:absolute;top:-6px;transform:translateX(-50%);min-width:13px;height:13px;padding:0 2px;box-sizing:border-box;border-radius:7px;background:#f2cf6b;color:var(--h-on,#140d05);font:700 9px/13px system-ui,sans-serif;text-align:center;letter-spacing:0}
#tapebar .tk::after{content:'';position:absolute;left:50%;top:13px;width:1px;height:21px;background:#f2cf6b;opacity:.55}
#tapebar .parted{position:absolute;top:0;width:0;height:100%;margin-left:-1px;border-left:2px dashed #ffb35c;pointer-events:none}
#tapebar .ghost{position:absolute;top:0;width:1px;height:100%;background:rgba(233,226,214,.7);pointer-events:none}
#tapebar .ghost::after{content:attr(data-t);position:absolute;bottom:calc(100% + 6px);left:50%;transform:translateX(-50%);padding:2px 6px;border-radius:2px;background:var(--h-field,rgb(21,19,16));font-variant-numeric:tabular-nums;white-space:nowrap}
#tapebar .say{min-width:0;white-space:nowrap;color:#ffb35c}
#tapebar .say:empty{display:none}
body.taping #equip{bottom:calc(86px + (var(--tape-h,44px) + var(--sndl-h,0px)) / var(--tz, 1))}
body.taping #meters{bottom:calc(16px + (var(--tape-h,44px) + var(--sndl-h,0px)) / var(--tz, 1))}
body.taping #log{bottom:calc(44px + (var(--tape-h,44px) + var(--sndl-h,0px)) / var(--tz, 1))}
#tapebig{position:fixed;left:0;right:0;top:0;bottom:calc((var(--tape-h,44px) + var(--sndl-h,0px)) / var(--tz, 1));z-index:69;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:14px;color:#e9e2d6;font:14px var(--m-head,system-ui),system-ui,sans-serif;letter-spacing:.06em;text-transform:uppercase;cursor:pointer}
#tapebig b{display:grid;place-items:center;width:84px;height:84px;border:2px solid var(--m-accent,#e8833a);border-radius:50%;font-size:34px;color:var(--m-accent,#e8833a);padding-left:6px;box-sizing:border-box;background:var(--h-field,rgb(21,19,16))}
#tapebig span{padding:5px 12px;border-radius:3px;background:var(--h-field,rgb(21,19,16))}
#tapenote{position:fixed;left:50%;bottom:calc((var(--tape-h,44px) + var(--sndl-h,0px)) / var(--tz, 1) + 10px);transform:translateX(-50%);z-index:71;width:min(560px,calc(100vw / var(--tz, 1) - 32px))}
#tapenote[hidden]{display:none}
#tapenote input{width:100%;box-sizing:border-box;padding:10px 12px;border:1px solid var(--m-accent,#e8833a);border-radius:3px;background:var(--h-field,rgb(21,19,16));color:#e9e2d6;font:14px system-ui,sans-serif;outline:none}`;

const el = <K extends keyof HTMLElementTagNameMap>(tag: K, cls = '', text = ''): HTMLElementTagNameMap[K] => { const e = document.createElement(tag); if (cls) e.className = cls; if (text) e.textContent = text; return e; };

export class TapeBar {
  private bar = el('div'); private play = el('button', 'pp', '▶'); private time = el('span', 'time'); private line = el('div', 'line'); private fill = el('div', 'fill'); private parted = el('div', 'parted'); private marks = el('div');
  private speed = el('button', 'x', '1×'); private slow = el('button', '', '−'); private fast = el('button', '', '+'); private view = el('button', '', 'Player view'); private fog = el('button', '', 'Fog'); private nv = el('button', '', 'NV'); private say = el('span', 'say');
  private big = el('div'); private box = el('div'); private field = el('input');
  /** a note is being typed: at which tick, on which spot of the floor, and was the run playing */
  private noting: { pos: [number, number] | null; tick: number; was: boolean } | null = null;
  private unkept = false; private painted = '';

  constructor(private d: BarDeps) {
    const css = el('style'); css.textContent = CSS; document.head.append(css);
    const { played: P, line } = { played: d.played, line: this.line }, ghost = el('div', 'ghost'), back = el('button', '', '‹ Game'), note = el('button', '', 'Note'), pace = el('div', 'seg'), sees = el('div', 'seg');
    this.bar.id = 'tapebar'; this.big.id = 'tapebig'; this.box.id = 'tapenote'; this.box.hidden = true; ghost.hidden = true; this.parted.hidden = true;
    this.play.title = 'Space'; this.slow.title = '-'; this.fast.title = '+'; this.view.title = 'C'; this.fog.title = 'F'; this.nv.title = 'V'; note.title = 'N, or click the picture'; back.title = 'Back to the game';
    const pressed = (b: HTMLButtonElement, f: () => void): void => { b.onclick = () => { b.blur(); f(); this.paint(); }; };   // (it lets go of the keyboard: Enter on a button that kept it would press it again)
    pressed(this.play, () => this.toggle()); pressed(this.slow, () => { P.pace = speedStep(P.pace, -1); }); pressed(this.fast, () => { P.pace = speedStep(P.pace, 1); }); pressed(this.speed, () => { P.pace = 1; });
    pressed(this.view, () => d.cam.back()); pressed(this.fog, () => this.fogTurn()); pressed(this.nv, () => this.nvTurn()); pressed(note, () => this.noteOpen(null)); pressed(back, () => { this.noteClose(true); d.back(); }); note.hidden = !d.keep;
    line.append(this.marks, this.parted, this.fill, ghost); pace.append(this.slow, this.speed, this.fast); sees.append(this.view, this.fog, this.nv);
    this.bar.append(back, el('i', 'gap'), this.play, pace, this.time, line, sees, el('i', 'gap'), note, this.say);
    this.big.append(el('b', '', '▶'), el('span', '', `${d.title} · ${clock(P.total / HZ)}`)); this.big.onclick = () => this.toggle(); 
    this.field.type = 'text'; this.field.maxLength = 400; this.box.append(this.field);
    // the line is dragged to scrub: a ghost with its time follows the pointer, and the place is gone to when the button comes up
    { const at = (e: MouseEvent): number => { const r = line.getBoundingClientRect(); return Math.max(0, Math.min(1, (e.clientX - r.left) / Math.max(1, r.width))); };
      const show = (e: MouseEvent): void => { const k = at(e); ghost.hidden = false; ghost.style.left = `${100 * k}%`; ghost.dataset.t = clock(P.total * k / HZ); };
      let scrub = -1, done = false; const go = (e: MouseEvent): void => { this.big.remove(); P.seek(P.total * at(e)); };
      line.addEventListener('pointerdown', e => { if (e.button !== 0) return; scrub = e.pointerId; done = false; try { line.setPointerCapture(e.pointerId); } catch { /* it was let go of */ } show(e); });
      line.addEventListener('pointermove', e => { if (scrub >= 0 && e.pointerId === scrub && !e.buttons) { scrub = -1; ghost.hidden = true; return; } show(e); });
      line.addEventListener('pointerleave', () => { if (scrub < 0) ghost.hidden = true; });
      const end = (e: PointerEvent, seek: boolean): void => { if (e.pointerId !== scrub) return; scrub = -1; try { line.releasePointerCapture(e.pointerId); } catch { /* it was let go of */ } ghost.hidden = true; if (seek) { done = true; go(e); setTimeout(() => { done = false; }, 0); } };
      line.addEventListener('pointerup', e => end(e, true)); line.addEventListener('pointercancel', e => end(e, false));
      line.onclick = e => { if (done) { done = false; return; } go(e); }; }   // (a click with no press before it, a script's, goes there too)
    document.body.append(this.bar, this.box); if (P.paused) document.body.append(this.big); document.body.classList.add('taping');
    for (const e of [this.bar, this.big, this.box]) for (const ev of ['mousedown', 'pointerdown', 'contextmenu', 'wheel']) e.addEventListener(ev, x => x.stopPropagation());
    d.canvas.addEventListener('click', e => { if (d.cam.wasDrag() || this.noting || !d.keep) return; const [u, v] = d.cam.uv(e), p = d.cam.floorAt(d.cam.view(), u, v); this.noteOpen(p ? [p[0], p[2]] : null); });
    window.addEventListener('keydown', e => this.key(e), { capture: true }); window.addEventListener('keyup', e => { d.cam.key(e.code, false); }, { capture: true }); window.addEventListener('blur', () => d.cam.letGo());
    const lay = (): void => { const h = Math.round(this.bar.getBoundingClientRect().height * textZoom() / rectZoom()) || 44; document.documentElement.style.setProperty('--tape-h', `${h}px`); };
    window.addEventListener('resize', lay); window.addEventListener(TEXT_EVENT, lay); lay(); this.marked(); this.paint();
  }
  /** a note is being typed: the keys and the mouse are its */
  get busy(): boolean { return !!this.noting; }

  /** there is nothing more of it to play */
  private get over(): boolean { const P = this.d.played; return !P.seeking && (P.lost !== null || P.paused && P.at >= P.total); }
  toggle(): void {
    const P = this.d.played; this.big.remove(); this.d.begun();
    if (this.over) { P.seek(0); P.paused = false; } else P.paused = !P.paused;
    this.paint();
  }
  private fogTurn(): void { const S = this.d.sees; if (S.fogged || !S.fog) S.set(!S.fog, S.tube); }
  /** every press flips the picture; one that lands on what the run has follows the run again */
  private nvTurn(): void { const S = this.d.sees, on = !S.lit; S.set(S.fog, on === S.runs ? null : on); }
  private key(e: KeyboardEvent): void {
    const { played: P, cam, map } = this.d;
    if (e.target === this.field) { if (e.code === 'Enter' || e.code === 'NumpadEnter') { e.preventDefault(); this.noteClose(true); } else if (e.code === 'Escape') { e.preventDefault(); this.noteClose(false); } e.stopImmediatePropagation(); return; }
    if (e.metaKey || e.ctrlKey || e.altKey) cam.letGo();   // (a key let go under Cmd sends no keyup on a Mac)
    if (e.metaKey || e.ctrlKey || e.altKey || /^F\d+$/.test(e.code) || e.code === 'Backslash') return;
    const t = e.target as HTMLElement | null; if (e.code === 'Escape' && (t instanceof HTMLInputElement || t instanceof HTMLSelectElement)) { t.dataset.esc = '1'; t.blur(); delete t.dataset.esc; e.preventDefault(); e.stopImmediatePropagation(); return; }   // (a field lets go of the keyboard, and of what was typed)
    if (t && (t.tagName === 'INPUT' || t.tagName === 'SELECT' || (t.tagName === 'TEXTAREA' && t.getAttribute('aria-label') !== 'game input'))) return;   // (the game's own key sink is no field somebody types in)
    e.stopImmediatePropagation();
    if (this.noting) { if (e.code === 'Escape') this.noteClose(false); else this.field.focus(); return; }   // (a press of the picture took the keyboard from the note)
    if (cam.key(e.code, true) || e.repeat) return;
    if (e.code === 'Space') { e.preventDefault(); this.toggle(); }
    else if (e.code === 'KeyN') { e.preventDefault(); this.noteOpen(null); }
    else if (e.code === 'KeyC') cam.back(); else if (e.code === 'KeyF') this.fogTurn(); else if (e.code === 'KeyV') this.nvTurn(); else if (e.code === 'KeyM') map.open();
    else if (e.code === 'Escape' && map.up) map.open(false);
    // (the pace by the character the key types: - and + are other keys on other keyboards)
    else if (e.key === '-' || e.key === '_' || e.code === 'NumpadSubtract') P.pace = speedStep(P.pace, -1);
    else if (e.key === '+' || e.key === '=' || e.code === 'NumpadAdd') P.pace = speedStep(P.pace, 1);
    else if (e.code === 'ArrowRight') P.seek(P.at + STEP); else if (e.code === 'ArrowLeft') P.seek(P.at - STEP);
    this.paint();
  }
  private noteOpen(pos: [number, number] | null): void {
    const P = this.d.played; if (!this.d.keep || this.noting) return;
    this.big.remove(); this.d.map.open(false); this.noting = { pos, tick: P.at, was: !P.paused }; P.paused = true; this.d.cam.letGo(); this.box.hidden = false; this.field.value = ''; this.field.placeholder = pos ? `Note on this spot at ${clock(P.at / HZ)}` : `Note at ${clock(P.at / HZ)}`; this.field.focus(); this.paint();
  }
  private noteClose(kept: boolean): void {
    const N = this.noting; if (!N) return;
    this.noting = null; this.box.hidden = true; this.field.blur(); if (kept) void this.note(this.field.value, N.pos, N.tick); if (N.was) this.d.played.paused = false; this.paint();
  }
  /** a note at a tick, kept with the run */
  async note(text: string, pos: [number, number] | null = null, tick = this.d.played.at): Promise<boolean> {
    const { notes, keep } = this.d; if (!keep || !text.trim()) return false;
    const r = Math.round(tick / HZ * 1000) / 1000, at = (x: number): number => Math.round(x * 100) / 100;
    notes.push({ id: `n${Date.now().toString(36)}${Math.floor(Math.random() * 1296).toString(36)}`, r, t: r, n: tick, pos: pos ? [at(pos[0]), at(pos[1])] : null, text: text.trim(), at: new Date().toISOString() });
    notes.sort((a, b) => a.n - b.n); this.marked(); this.unkept = !(await keep(notes)); this.paint(); return !this.unkept;
  }
  private marked(): void {
    const P = this.d.played, at = (tick: number): string => `${100 * Math.max(0, Math.min(1, tick / Math.max(1, P.total)))}%`; this.marks.textContent = '';
    for (const t of P.tries) { const d = el('div', 'try'); d.style.left = at(t); this.marks.append(d); }
    this.d.notes.forEach((n, i) => { const d = el('div', 'tk', String(i + 1)); d.style.left = at(n.n); d.title = n.text; this.marks.append(d); });
  }
  /** every frame: what it says of the run as it stands */
  paint(): void {
    if (!this.d.played.seeking && this.d.played.lost === null) this.d.map.frame();
    const { played: P, cam, sees: S } = this.d, says = P.seeking ? '…' : this.unkept ? 'This browser wouldn’t keep that note.' : P.lost !== null ? 'The game stopped here.' : P.parted !== null ? `After ${clock(P.parted / HZ)}, this isn’t what really happened.` : '';
    const ended = this.over, sig = `${P.paused}|${ended}|${Math.round(P.at / 6)}|${P.pace}|${says}|${!!cam.set}|${S.fog}|${S.fogged}|${S.lit}`; if (sig === this.painted) return; this.painted = sig;
    this.play.textContent = ended ? '↺' : P.paused ? '▶' : '❚❚'; this.time.textContent = `${clock(P.at / HZ)} / ${clock(P.total / HZ)}`; this.fill.style.width = `${100 * P.at / Math.max(1, P.total)}%`;
    this.speed.textContent = speedName(P.pace); this.speed.classList.toggle('on', P.pace !== 1); this.slow.classList.toggle('end', P.pace === TAPE_SPEEDS[0]); this.fast.classList.toggle('end', P.pace === TAPE_SPEEDS[TAPE_SPEEDS.length - 1]);
    this.view.classList.toggle('on', !cam.set); this.fog.hidden = !S.fogged && S.fog; this.fog.classList.toggle('on', S.fog); this.nv.classList.toggle('on', S.lit); this.parted.hidden = P.parted === null; if (P.parted !== null) this.parted.style.left = `${100 * P.parted / Math.max(1, P.total)}%`;
    this.say.textContent = says; this.say.title = P.lost ?? '';
  }
}
