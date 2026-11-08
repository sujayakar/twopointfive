// THE GUARD of the Team & kit picture's window (`?kitview`: the kit screen's frame, host/kitpage.ts), at this module's load: the page imports it FIRST, before any module that reads or
// writes a store. This window's Storage writes do nothing (a frame has its own Storage.prototype: the page that holds it is untouched), it has no IndexedDB and asks for no persistence,
// and no key, button, wheel or touch reaches a listener registered after this one.
export const kitPicture: boolean = (() => { try { return /[?&]kitview(=|&|$)/.test(location.search); } catch { return false; } })();
if (kitPicture) {
  try { const P = Storage.prototype; P.setItem = () => { /* read-only */ }; P.removeItem = () => { /* read-only */ }; P.clear = () => { /* read-only */ }; } catch { /* no Storage */ }
  try { Object.defineProperty(window, 'indexedDB', { configurable: true, get: () => undefined }); } catch { /* */ }
  try { const st = navigator.storage as unknown as { persist?: () => Promise<boolean> } | undefined; if (st) st.persist = () => Promise.resolve(false); } catch { /* */ }
  for (const ev of ['keydown', 'keyup', 'keypress', 'mousedown', 'mouseup', 'mousemove', 'click', 'dblclick', 'contextmenu', 'wheel', 'pointerdown', 'pointerup', 'pointermove', 'touchstart', 'touchmove', 'touchend']) window.addEventListener(ev, e => { e.stopImmediatePropagation(); }, { capture: true });
  try { document.body.classList.add('kitview'); } catch { /* */ }
}
