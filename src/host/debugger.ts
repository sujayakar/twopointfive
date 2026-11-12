// THE DEBUGGER, on a page that draws a game it does not hold (ui/panel.ts; backslash): every number of the rules a person may turn, found by its name and turned while the game plays
// (rust/game_start/src/host/knobs.rs), then the renderer's pages (ui/graphicspanel.ts) and the look's, with its takes (ui/lookpanel.ts); on a networked game's page, its connection (ui/netrows.ts).
import type { Engine } from '../engine';
import type { GltfCharacter } from '../anim/gltf';
import type { FollowCamera } from './camera';
import type { Input } from './input';
import type { Decals } from '../render/decals';
import type { SmokeSystem } from '../smoke/smoke';
import type { NetPageState } from '../net/door';
import type { Quality } from '../ui/quality';
import { buildGraphics, graphicsSystems, SYS } from '../ui/graphicspanel';
import { HOURS_OF } from '../draw/sky';
import { Section, Sky } from '../draw/sections';
import { buildLookPanel, type SkyTurns } from '../ui/lookpanel';
import { netRows } from '../ui/netrows';
import { Panel } from '../ui/panel';
import { rainRows } from '../ui/rainrows';
import type { DrawGame } from './drawgame';
import type { ActionRow } from '../view/read';
import { keepPick, keepRow } from './machine';
import type { DrawHud } from './drawhud';
import { DEBUG_KEY, gameCode, typedInto } from './keymap';

/** rows shown at once: a word narrows them */
const SHOWN = 40;
/** the panel's own row protocol for a list that is laid again as its words change: one row to the panel, its values read again as the panel reads its rows, and the keyboard given back */
function knobRows(panel: Panel, sec: HTMLElement, game: DrawGame, shut: boolean): void {
  const row = document.createElement('div'), find = document.createElement('input'), list = document.createElement('div');
  const inner = document.createElement('div'); inner.style.cssText = 'flex:1 1 100%;min-width:0;';
  row.className = 'row'; row.title = 'every knob of the rules';
  find.type = 'search'; find.placeholder = 'a word or two of a knob\'s name'; find.style.cssText = 'font:inherit;width:100%;box-sizing:border-box;padding:4px 6px;margin:2px 0 6px;background:#15130f;color:inherit;border:1px solid #4a4232;';
  list.style.cssText = 'display:grid;grid-template-columns:1fr auto;gap:4px 10px;align-content:start;';
  let reads: ((all: Map<string, number>) => void)[] = [];
  const now = (): Map<string, number> => new Map(game.knobs().map(k => [k.name, k.value]));
  const lay = (): void => {
    const words = find.value.toLowerCase().split(/\s+/).filter(Boolean), all = game.knobs(), hits = all.filter(k => words.every(w => k.name.toLowerCase().includes(w)));
    list.textContent = ''; reads = [];
    for (const k of hits.slice(0, SHOWN)) {
      const name = document.createElement('span'), v = document.createElement('input'); name.textContent = k.name; name.style.cssText = 'overflow-wrap:anywhere;';
      v.type = k.yes ? 'checkbox' : 'number'; if (!k.yes) v.step = 'any'; v.style.cssText = 'font:inherit;width:8em;background:#15130f;color:inherit;border:1px solid #4a4232;';
      const read = (m: Map<string, number>): void => { if (document.activeElement === v) return; const x = m.get(k.name) ?? 0; if (k.yes) v.checked = x !== 0; else if (v.value !== String(x)) v.value = String(x); };
      v.addEventListener('change', () => { const x = k.yes ? (v.checked ? 1 : 0) : v.valueAsNumber; if (Number.isFinite(x)) { game.turn(k.name, x); game.tell(); } v.blur(); });
      v.disabled = shut; if (shut) v.style.opacity = '0.45';
      read(new Map([[k.name, k.value]])); reads.push(read); list.append(name, v);
    }
    const more = document.createElement('div'); more.textContent = hits.length > SHOWN ? `and ${hits.length - SHOWN} more of ${all.length}` : `${hits.length} of ${all.length}`; more.style.cssText = 'grid-column:1/3;opacity:0.6;'; list.append(more);
  };
  find.addEventListener('input', lay);
  find.addEventListener('keydown', e => { if (e.code === 'Enter' || e.code === 'Escape') { find.blur(); e.preventDefault(); e.stopPropagation(); } });
  panel.root.addEventListener('mouseleave', () => { const a = document.activeElement; if (a instanceof HTMLElement && row.contains(a)) a.blur(); });
  inner.append(find, list); row.append(inner); sec.append(row); lay();
  panel.keep(() => { if (reads.length && !game.lost) { const m = now(); for (const r of reads) r(m); } }, row);
}

/** the sky as the module has it, turned by commands done at once */
function skyOf(game: DrawGame): SkyTurns {
  const D = game.drawn, C = game.commands, f = (w: Sky): number => D.f[D.at[Section.sky] + w], u = (w: Sky): number => D.u[D.at[Section.sky] + w];
  return {
    hour: () => HOURS_OF[u(Sky.hour)], setHour: id => { C.hour(id); game.tell(); },
    cover: () => { const c = f(Sky.cover); return c < 0 ? null : c; }, setCover: v => { C.cover(v); game.tell(); },
    sun: () => f(Sky.sunShare), setSun: v => { C.sun(v); game.tell(); },
    moon: () => u(Sky.moon) ? { k: f(Sky.moonK), elev: f(Sky.moonElev), bearing: f(Sky.moonBearing), up: u(Sky.moon) === 2 } : null,
    setMoon: o => { C.moon(o.k ?? NaN, o.elev ?? NaN, o.bearing ?? NaN); game.tell(); },
  };
}
/** THE ROWS OF THE GAME'S NUMBERS: laid out from the module's list (game_start host/shown.rs), a section each. What a row was turned to stays on this machine if the row says so (machine.ts keptRows reads it at a start). */
function shownRows(panel: Panel, game: DrawGame, secs: Map<string, HTMLElement>): void {
  const rows = game.shown(); let at = -1, now: Float64Array = new Float64Array(0);
  const read = (i: number): number => { if (at !== game.ticks) { at = game.ticks; now = game.shownNow(); } return now[i]; };
  rows.forEach((r, i) => {
    let sec = secs.get(r.section); if (!sec) secs.set(r.section, sec = panel.section(r.section, { open: false }));
    const def = Number.isNaN(r.def) ? read(i) : r.def * r.scale, was = panel.advanced; panel.advanced = r.adv;
    panel.slider(sec, r.label, r.lo, r.hi, r.step, () => read(i), v => { game.turn(r.name, v / r.scale); game.tell(); at = -1; if (r.kept) keepRow(r.name, v, def); }, r.title || undefined, def);
    panel.advanced = was;
  });
}
/** THE ACTIONS: drawn from the module's list (game_start host/actions.rs), and each a command. A row names no action here. */
function actionRows(panel: Panel, game: DrawGame, secs: Map<string, HTMLElement>): void {
  const act = (name: string, value: number, word: string): void => { game.commands.act(name, value, word); game.tell(); };
  let at = -1, now = game.actions(); const rows = (): ActionRow[] => { if (at !== game.ticks) { at = game.ticks; now = game.actions(); } return now; };
  now.forEach((a, i) => {
    let sec = secs.get(a.section); if (!sec) secs.set(a.section, sec = panel.section(a.section, { open: a.section === 'Sandbox' }));
    const title = a.title || undefined;
    switch (a.kind) {
      case 'press': panel.buttons(sec, a.label, [{ text: 'do it', run: () => act(a.name, 0, ''), title }]); break;
      case 'each': panel.buttons(sec, a.label, a.words.map(w => ({ text: w, run: () => act(a.name, 0, w), title }))); break;
      case 'choice': panel.select(sec, a.label, a.words, () => { const r = rows()[i]; return Math.max(0, r.words.indexOf(r.now)); }, v => { game.pick(a.name, a.words[v]); game.tell(); at = -1; keepPick(a.name, a.words[v]); }, title); break;
      case 'count': { let n = a.start; panel.slider(sec, a.label, a.lo, a.hi, a.step, () => n, v => { n = v; }, title); panel.buttons(sec, a.label, [{ text: 'do it', run: () => act(a.name, n, ''), title }]); break; }
      case 'reload': panel.buttons(sec, a.label, a.words.map(w => ({ text: w, run: () => { const u = new URL(location.href), was = u.searchParams.get(a.word); if (a.words[0] === w ? was === null : was === w) return; if (a.words[0] === w) u.searchParams.delete(a.word); else u.searchParams.set(a.word, w); location.href = u.href; }, title }))); break;
    } });
}

export interface DebuggerDeps { game: DrawGame; engine: Engine; smoke: SmokeSystem; cam: FollowCamera; quality: Quality; input: Input; chars: GltfCharacter; marks: Decals; over: DrawHud | null; /** a networked game's page: its connection, made worse on purpose (ui/netrows.ts) */ net: { state(): NetPageState } | null; /** a key that is a card's over the game, before the panel's or the game's */ card(e: KeyboardEvent): boolean; hud(): { fps: number; union: number } }
export function debuggerPanel(d: DebuggerDeps): Panel {
  const panel = new Panel();
  panel.system('Rules', { icon: '⚑', short: 'Rules', title: 'the sandbox\'s actions, the game\'s numbers, and every knob of the rules by its name' });
  // (a game that is walked takes no command but its own: a networked one is the server's, a replay the tape's. The rows are there to read)
  const why = d.net ? 'Numbers can\'t be turned in a co-op game.' : d.game.walked ? 'Numbers can\'t be turned while a replay plays.' : '';
  const knobs = panel.section('Knobs', { open: true }), secs = new Map<string, HTMLElement>();
  if (why) panel.text(knobs, () => why);
  knobRows(panel, knobs, d.game, why !== ''); actionRows(panel, d.game, secs); shownRows(panel, d.game, secs);
  if (why) for (const s of secs.values()) { s.inert = true; s.style.opacity = '0.45'; s.title = why; }
  { const o = d.over; if (o) { const s = panel.section('HUD', { open: true });
    panel.toggleBox(s, 'show AI debug labels', () => o.page.debug, v => { d.game.commands.labels(v ? 1 : 0); d.game.tell(); o.told(); }, 'state / awareness tags over the guards');
    panel.toggleBox(s, 'light fixture dots', () => o.overlay.showLightDots, v => { o.overlay.showLightDots = v; }); } }
  graphicsSystems(panel);
  buildGraphics(panel, d.engine, d.smoke, d.cam, d.quality, { chars: d.chars, marks: d.marks }, d.hud);
  panel.system(SYS.rfx); rainRows(panel, d.engine.settings);
  buildLookPanel(panel, d.engine, () => d.hud().fps, false, skyOf(d.game));
  if (d.net) netRows(panel, d.net);
  d.input.keyVeto = e => d.card(e) || panel.onKey(e);
  addEventListener('keydown', e => { if (e.repeat || e.metaKey || e.ctrlKey || e.altKey || gameCode(e.code) !== DEBUG_KEY || typedInto(e.target)) return; e.preventDefault(); panel.toggle(); }, true);
  (window as unknown as { __panel: Panel }).__panel = panel;
  return panel;
}
