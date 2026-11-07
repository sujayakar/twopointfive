// THE KNOBS, on the new page: every number of the rules a person may turn, found by its name and turned while the game plays (rust/game_run knobs.rs). Backslash.
import type { DrawGame } from './drawgame';

/** rows shown at once: a word narrows them */
const SHOWN = 40;
export function knobPanel(app: HTMLElement, game: DrawGame): void {
  const el = document.createElement('div'), find = document.createElement('input'), list = document.createElement('div');
  el.style.cssText = 'position:absolute;top:0;right:0;bottom:0;width:min(420px,100%);z-index:15;display:none;flex-direction:column;gap:8px;padding:12px;box-sizing:border-box;background:rgba(12,11,9,0.88);color:#d2c9b5;font:12px/1.4 ui-monospace,Menlo,monospace;';
  find.placeholder = 'knobs: a word or two of a name'; find.style.cssText = 'font:inherit;padding:6px 8px;background:#15130f;color:inherit;border:1px solid #4a4232;';
  list.style.cssText = 'overflow:auto;display:grid;grid-template-columns:1fr auto;gap:4px 10px;align-content:start;';
  const show = (): void => {
    const words = find.value.toLowerCase().split(/\s+/).filter(Boolean), all = game.knobs(), hits = all.filter(k => words.every(w => k.name.toLowerCase().includes(w)));
    list.textContent = '';
    for (const k of hits.slice(0, SHOWN)) {
      const name = document.createElement('label'), v = document.createElement('input'); name.textContent = k.name; name.style.cssText = 'overflow-wrap:anywhere;';
      v.type = k.yes ? 'checkbox' : 'number'; v.style.cssText = 'font:inherit;width:8em;background:#15130f;color:inherit;border:1px solid #4a4232;'; if (k.yes) v.checked = k.value !== 0; else { v.step = 'any'; v.value = String(k.value); }
      v.addEventListener('change', () => { const x = k.yes ? (v.checked ? 1 : 0) : v.valueAsNumber; if (Number.isFinite(x)) game.commands.knob(k.name, x); });
      list.append(name, v);
    }
    const more = document.createElement('div'); more.textContent = hits.length > SHOWN ? `and ${hits.length - SHOWN} more of ${all.length}` : `${hits.length} of ${all.length}`; more.style.cssText = 'grid-column:1/3;opacity:0.6;'; list.append(more);
  };
  find.addEventListener('input', show);
  // (what is typed here is no key of the game's)
  el.addEventListener('keydown', e => { if (e.code !== 'Backslash' && e.code !== 'Escape') e.stopPropagation(); });
  addEventListener('keydown', e => {
    if (e.code !== 'Backslash' || e.repeat || e.ctrlKey || e.metaKey) return;
    e.preventDefault(); const up = el.style.display === 'none'; el.style.display = up ? 'flex' : 'none';
    if (up) { show(); find.focus(); } else find.blur();
  }, true);
  el.append(find, list); app.append(el);
}
