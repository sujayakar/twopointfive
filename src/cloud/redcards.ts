// THE BACKEND IS TOLD OF EVERY RED CARD, on whichever page puts one up (gpu/device.ts FATAL.tell; cloud/cloud.ts cloudFatal), set before a page boots so a start that never got a GPU is one
// of them: no request on a build with no backend, under ?cloud=0 or ?cloudstats=0, or on a page that is mute (the kit's small view). A browser that has not been shown what the
// game sends is shown it on the card itself, on top, and the record goes once the line has been read (ui/cloudnote.ts: two seconds in view).
import { cloudFatal, cloudUrl } from './cloud';
import { FATAL } from '../gpu/device';
import { cloudNote } from '../ui/cloudnote';

export function tellRedCards(mute?: () => boolean): void {
  FATAL.tell = (r, card) => {
    if (mute?.()) return;
    const q = new URLSearchParams(location.search); if (!cloudUrl(q)) return;
    const data = { ...(r.where ? { where: r.where } : {}), ...(r.adapter ? { adapter: r.adapter } : {}), ...(r.data !== undefined ? { facts: r.data } : {}) };
    const say = (build?: string): void => {
      const s = cloudFatal(q, { message: r.message, stack: r.stack, stage: r.stage ?? (r.where ? 'boot' : 'run'), ...(Object.keys(data).length ? { data } : {}), ...(build ? { build } : {}) });
      if (!s?.waits) return;
      card.style.flexDirection = 'column'; card.style.gap ||= '16px'; cloudNote(card, undefined, { told: s.told });
      const line = card.lastElementChild; if (line?.classList.contains('cloudnote')) card.prepend(line);
    };
    // (a visit that is up knows its build. A page that died before that asks for the one small file a build carries, and waits a second and a half for it)
    if ((window as unknown as { __cloud?: unknown }).__cloud) { say(); return; }
    let said = false; const once = (c?: string): void => { if (said) return; said = true; try { say(c && /^[0-9a-f]{7,40}$/.test(c) ? c : undefined); } catch { /* the card is up */ } };
    try { setTimeout(once, 1500); void fetch('/qa/commit', { cache: 'no-store' }).then(x => x.ok ? x.text() : '').then(t => once(t.trim()), () => once()); } catch { once(); }
  };
}
