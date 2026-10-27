// THE NOTICE ON THE TITLE PAGE of a hosted build that has a backend and whose browser sends (src/cloud/cloud.ts; the maintainer's pick, 2026-10-26: on, with a
// notice): one line that says what is sent and where to turn it off, and a press of it opens Options on the row. With no backend, or in a browser that does
// not send, nothing is added: the title as it was.
// IT ALSO TELLS THE BACKEND'S SIDE THE LINE HAS BEEN SEEN (`__cloud.told()`): a browser that sends only because that is the default sends nothing until then.
// "Seen" is spelt out below (in the window, on top, for two seconds). A CARD THAT IS NOT THE TITLE can carry the same line (`off`): a benchmark's result
// card, so a page opened straight onto a benchmark is told and its report goes up; the error card of a page that never started, so its crash record goes.
// The boot card never has the line: the hook does not exist yet. The page's hook is all this reads: no game code, and nothing at module level.
interface Hook { share: boolean; told(): void }
const hook = (): Hook | null => { try { return ((globalThis as { __cloud?: Hook | null }).__cloud) ?? null; } catch { return null; } };

/** how long the line must have been on screen before the browser counts as told, and how often that is looked at (ms) */
export const CLOUD_NOTE_MS = 2000, CLOUD_NOTE_STEP = 500;
export const CLOUD_NOTE = 'This game sends your replays, stats and settings to the developer. You can turn that off in Options.';

/** `openRow`: what a press of the line does (the title: Options on the row; absent = the line is words only). `off`: a card that is NOT the title — a
 *  benchmark's result, the error card of a page that never started, where there is no hook yet and `told` is the caller's own (cloudFatal's answer) */
export function cloudNote(card: HTMLElement, openRow?: () => void, off?: { told?: () => void }): void {
  const H = hook(), C = off?.told ? { share: true, told: off.told } : H; if (!C || !C.share) return;
  const b = document.createElement('button'); b.type = 'button'; b.className = 'cloudnote'; b.textContent = CLOUD_NOTE; if (openRow) b.onclick = openRow;
  b.style.cssText = `all:unset;display:block;cursor:${openRow ? 'pointer' : 'default'};margin:0 0 10px;max-width:56ch;font:11px/1.45 var(--m-mono, ui-monospace, monospace);letter-spacing:0.04em;color:var(--m-soft, #9aa3ad)`;
  if (openRow) { b.onfocus = b.onmouseenter = () => { b.style.textDecoration = 'underline'; }; b.onblur = b.onmouseleave = () => { b.style.textDecoration = ''; }; }
  card.appendChild(b);
  // SEEN = the line is on the live title, inside the window, with nothing over it (the starting page, Options, an error card: the topmost thing at its
  // middle is the line itself), for `CLOUD_NOTE_MS` in a row. A title scrolled so the line is under the fold has not shown it.
  const seen = (): boolean => {
    if (document.hidden) return false;
    if (!off) { const m = document.getElementById('menu'); if (!m || !m.classList.contains('on') || !m.classList.contains('title') || m.hasAttribute('inert')) return false; }
    const r = b.getBoundingClientRect(); if (!(r.height > 0 && r.width > 0) || r.top < 0 || r.bottom > window.innerHeight) return false;
    const top = document.elementFromPoint(r.left + Math.min(r.width, 40) / 2, r.top + r.height / 2); return top === b;
  };
  let run = 0;
  const look = (): void => { if (!b.isConnected) return; run = seen() ? run + CLOUD_NOTE_STEP : 0; if (run >= CLOUD_NOTE_MS) C.told(); else setTimeout(look, CLOUD_NOTE_STEP); };
  setTimeout(look, CLOUD_NOTE_STEP);
}
