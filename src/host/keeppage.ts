// THE KEPT PAGE, the page's side (the rule and the shell's keeper: gpu/keep.ts). main() asks this first of all, before a key is read or a record begun:
//   shell   this window is the tab's own page: it keeps the GPU device and shows the game in a frame of this address. main() goes no further here
//   leave   this window is a frame under a shell it cannot stay under (the page was asked to load whole, or the shell is another build's): the tab is sent
//           to this address. main() goes no further here either
//   kept    a frame under its shell: boot as ever, the device taken from `keeper`
//   plain   a page as it always was
import { KEEP_PAGE, KEPT, keepRole, keeperAbove, makeKeeper, showKeptFrame, type KeepRole, type Keeper } from '../gpu/keep';
import { openDevice } from '../gpu/device';

/** when (wall clock, ms) this tab last left a shell of another build: kept for the tab */
const LEFT = 'twopointsix.keeppage.left';
interface NavEvent extends Event { hashChange: boolean; downloadRequest: string | null; destination: { url: string; sameDocument: boolean } }

/** `own` = a page that keeps its own page whatever the tab does (the benchmark and its scenes, the kit picture's window) */
export function keepPageBoot(q: URLSearchParams, own: boolean): { role: KeepRole; keeper: Keeper | null } {
  const word = q.get(KEEP_PAGE.word);
  // (the word is the tab's from the load that carried it: a pick's or a restart's address does not carry it on)
  let tab: string | null = null;
  try { if (word === '0') sessionStorage.setItem(KEEP_PAGE.store, '0'); else if (word !== null) sessionStorage.removeItem(KEEP_PAGE.store); tab = sessionStorage.getItem(KEEP_PAGE.store); } catch { /* no session store: the address's word alone */ }
  // (a build is known by the scripts its page names: a deploy in the middle of a visit hands the frame another page than the shell's)
  // (…its OWN scripts: one an extension or a host's toolbar wrote into the tab's page is not the build's)
  const build = Array.from(document.querySelectorAll<HTMLScriptElement>('script[type="module"][src]'), s => s.src).filter(u => u.startsWith(location.origin + '/')).join(' ');
  const above = keeperAbove(window, build);
  let top = false; try { top = window.top === window.self; } catch { /* a window of another origin above */ }
  KEPT.shelled = !!above;
  // (a shell of another build is left ONCE: a tab whose two pages can never agree — something rewrites one of them — must not load for ever, so a second
  // leave inside a minute is not made and the frame plays on a device of its own)
  let stale = !!above && !above.same && !above.keeper.refused;
  if (stale) try { const last = Number(sessionStorage.getItem(LEFT)) || 0; if (Date.now() - last < 60_000) stale = false; else sessionStorage.setItem(LEFT, String(Date.now())); } catch { stale = false; }
  // (a frame is KNOWN to draw with its parent's device in the Chromium family — Chrome, Edge and their kin say `Chrome/`, a headless one `HeadlessChrome/` —
  // and nowhere else has it been seen)
  const proven = /Chrome\/\d/.test(navigator.userAgent);
  const role = keepRole({ word, tab, top, keeper: !!above?.same, stale, own, driven: navigator.webdriver === true || q.has('paused'), proven, onefile: !!document.querySelector('meta[name="tps-one-page"]') });
  if (role === 'shell') {
    const keeper = makeKeeper(() => openDevice(navigator.gpu), build, performance.now(), ((): string => { try { return String((performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming | undefined)?.type ?? ''); } catch { return ''; } })());
    if (q.get(KEEP_PAGE.drainWord) === '0') keeper.drain = false;   // (the tab's first address: the keeper is this window's for the visit)
    (window as unknown as Record<string, unknown>)[KEEP_PAGE.hook] = keeper;
    showKeptFrame(document, location.href);
  } else if (role === 'leave') {
    try { window.top!.location.replace(location.href); } catch { return { role: 'plain', keeper: null }; }
  }
  if (role === 'kept') {
    // the address bar is the shell's: it takes the frame's address at each load of the frame (gpu/keep.ts showKeptFrame), and here whenever this page
    // rewrites its own with no load (a quit to the list on the office, a word taken off the address)
    const H = history, mirror = (): void => { try { const up = window.parent; if (up.location.href !== location.href) up.history.replaceState(up.history.state, '', location.href); } catch { /* */ } };
    for (const k of ['replaceState', 'pushState'] as const) { const real = H[k].bind(H); H[k] = (...a: Parameters<History['replaceState']>): void => { real(...a); mirror(); }; }
    addEventListener('popstate', mirror); addEventListener('hashchange', mirror);
    // ANOTHER PAGE of the site (the replay viewer, the workbench, the listening pages) is the tab's to load, not the frame's: it writes its own address and
    // title as it goes, and nothing of the kept page is its to use. The load is stopped on its way and made by the tab (the Navigation API; a browser
    // without it loads the page in the frame and the shell then sends the tab there: gpu/keep.ts showKeptFrame)
    try {
      (window as unknown as { navigation?: EventTarget }).navigation?.addEventListener('navigate', ev => {
        const e = ev as NavEvent; if (!e.cancelable || e.hashChange || e.downloadRequest !== null || e.destination.sameDocument) return;
        let to: URL; try { to = new URL(e.destination.url); } catch { return; }
        if (to.origin !== location.origin || to.pathname === location.pathname) return;
        e.preventDefault(); window.top!.location.assign(to.href);
      });
    } catch { /* the shell's load listener */ }
  }
  // (for a console and the page check: what this window is, and the lease its device came on — null before the device is asked for and on a plain page)
  (window as unknown as Record<string, unknown>).__keptPage = { role, get lease() { return KEPT.lease; } };
  return { role, keeper: role === 'kept' ? above!.keeper : null };
}
