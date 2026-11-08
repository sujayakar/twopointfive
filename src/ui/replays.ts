// THE REPLAYS LIST, down the title's column: every run this browser keeps (src/tape/run.ts), newest first. A row plays it, its cross lets it go.
import { clock } from '../tape/format';
import { keptNow, keptStore, keptWords, mb } from '../tape/keptui';
import { beingWritten } from '../tape/keepstore';
import { PLAYTAPE } from '../tape/address';
import { reportWarning } from '../cloud/report';
import { button, el } from './runcard';

const CSS = `
#menu.title .actions .xlist{max-height:min(58vh,560px);overflow-y:auto;overflow-x:hidden;display:flex;flex-direction:column;width:min(540px,100%)}
#menu.title .actions .xlist .one{position:relative}
#menu.title .actions .xlist .btn.xrow{padding-right:52px}
#menu.title .actions .xlist .btn.xrow i{display:block;font:500 13px/1.4 var(--h-font,sans-serif);letter-spacing:.02em;text-transform:none;color:var(--m-soft)}
#menu.title .actions .xlist .drop{all:unset;position:absolute;right:14px;top:50%;transform:translateY(-50%);cursor:pointer;padding:6px 9px;color:var(--m-soft)}
#menu.title .actions .xlist .drop:hover,#menu.title .actions .xlist .drop:focus{color:var(--m-accent)}
#menu.title .actions .xnote{max-width:520px;margin:8px 0 0 4px;font:500 14px/1.5 var(--h-font,sans-serif);color:var(--m-soft)}`;
let styled = false;
/** fills `host` with the list; `back` = the way out of it */
export function replayRows(host: HTMLElement, back: () => void, focus: boolean): void {
  if (!styled) { styled = true; const s = document.createElement('style'); s.textContent = CSS; document.head.append(s); }
  host.textContent = ''; const head = el('div', 'xhead'), list = el('div', 'xlist'), note = el('div', 'xnote'), out = button('‹ Main menu', back, 'xback'); head.textContent = 'Replays'; host.append(head, list, note, out);
  const fill = async (): Promise<void> => {
    const S = await keptNow(), runs = S.rows.filter(r => r.hands); list.textContent = '';
    for (const r of runs) {
      const one = el('div', 'one'), a = el('a', 'btn xrow') as HTMLAnchorElement, name = document.createElement('b'), line = document.createElement('i'), drop = el('button', 'drop') as HTMLButtonElement, live = beingWritten(r), n = r.notes?.length ?? 0;
      a.href = `?${PLAYTAPE}=${r.sid}`; a.dataset.replay = r.sid; name.textContent = r.scenario || 'A run'; line.textContent = [new Date(r.lastAt).toLocaleString([], { month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit' }), clock(r.seconds), mb(r.bytes), ...(n ? [`${n} note${n > 1 ? 's' : ''}`] : []), ...(live ? ['being played'] : [])].join(' · ');
      a.append(name, line); drop.textContent = '✕'; drop.title = 'Remove it from this browser'; drop.hidden = live;
      drop.onclick = () => { if (confirm(n ? 'This replay has notes on it. Remove it from this browser?' : 'Remove this replay from this browser?')) void keptStore().drop(r.sid).then(fill); };
      one.append(a, drop); list.append(one);
    }
    note.textContent = runs.length ? keptWords({ ...S, rows: runs, visits: runs.length, seconds: runs.reduce((s, r) => s + r.seconds, 0), bytes: runs.reduce((s, r) => s + r.bytes, 0) }) : 'Every run you play is kept here, so play one and come back.';
    if (focus) ((list.querySelector('.xrow') as HTMLElement | null) ?? out).focus();
  };
  void fill().catch(e => { reportWarning('replays.list', e); note.textContent = 'This browser won’t keep replays.'; });
}
