// THE NOTICE ON THE TITLE PAGE of a hosted build that has a backend and whose browser sends (src/cloud/cloud.ts; the maintainer's pick, 2026-10-26: on, with a
// notice): one line that says what is sent and where to turn it off, and a press of it opens Options on the row. With no backend, or in a browser that does
// not send, nothing is added: the title as it was.
// IT ALSO TELLS THE BACKEND'S SIDE THE LINE HAS BEEN SEEN (`__cloud.told()`): a browser that sends only because that is the default sends nothing until then.
// "Seen" = the line is on the live title and nothing is over it (the title is `inert` under the starting page and Options). The boot card never has it: the
// hook does not exist yet. The page's hook is all this reads: no game code, and nothing at module level.
interface Hook { share: boolean; told(): void }
const hook = (): Hook | null => { try { return ((globalThis as { __cloud?: Hook | null }).__cloud) ?? null; } catch { return null; } };

export const CLOUD_NOTE = 'This game sends your replays, stats and settings to the developer. You can turn that off in Options.';

export function cloudNote(card: HTMLElement, openRow: () => void): void {
  const C = hook(); if (!C || !C.share) return;
  const b = document.createElement('button'); b.type = 'button'; b.className = 'cloudnote'; b.textContent = CLOUD_NOTE; b.onclick = openRow;
  b.style.cssText = 'all:unset;display:block;cursor:pointer;margin:0 0 10px;max-width:56ch;font:11px/1.45 var(--m-mono);letter-spacing:0.04em;color:var(--m-soft)';
  b.onfocus = b.onmouseenter = () => { b.style.textDecoration = 'underline'; }; b.onblur = b.onmouseleave = () => { b.style.textDecoration = ''; };
  card.appendChild(b);
  const seen = (): boolean => { const m = document.getElementById('menu'); return !!m && b.isConnected && m.classList.contains('on') && m.classList.contains('title') && !m.hasAttribute('inert') && !document.hidden; };
  const look = (): void => { if (seen()) C.told(); else if (b.isConnected) setTimeout(look, 500); };
  setTimeout(look, 0);   // (after the card is in the page)
}
