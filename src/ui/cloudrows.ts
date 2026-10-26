// THE BACKEND'S TWO ROWS ON THE OPTIONS SCREEN (ui/options.ts, the Game tab), on a hosted build that has a backend — src/cloud/cloud.ts. With none (the dev
// server, a checkout, the one-file page) this adds nothing: the screen as it was.
//   Share replays and stats   this browser's own choice to send (cloud.ts `share`, kept in this browser)
//   Your settings link        copies the link that hands this browser's settings to another one
// The page's hook (`window.__cloud`, main.ts) is all this reads: no game code, and nothing at module level.
import type { RowKit } from './findme';

interface Hook { share: boolean; link(): string }
const hook = (): Hook | null => { try { return ((globalThis as { __cloud?: Hook | null }).__cloud) ?? null; } catch { return null; } };

export function cloudRows(K: Pick<RowKit, 'sec' | 'flip' | 'seg'>, body: HTMLElement): void {
  const C = hook(); if (!C) return;
  let copied = false;
  K.sec(body, 'Online');
  K.flip(body, 'Share replays and stats', () => C.share, v => { C.share = v; },
    'When this is on, the game sends your replays, how well it ran on your machine and your settings to the developer. Nothing else is sent, and you can turn it off here any time.', ['On', 'Off']);
  K.seg(body, 'Your settings link', [{ v: 'copy', label: 'Copy' }], () => copied ? 'copy' : '', () => {
    if (!C.share) return;
    try { void navigator.clipboard?.writeText(C.link()).then(() => { copied = true; }, () => { /* the browser would not: nothing copied */ }); } catch { /* */ }
  }, () => !C.share ? 'Turn on Share replays and stats first, and then you can copy a link to your settings.' : copied ? 'The link is copied. Open it in another browser and that browser will have your settings.' : 'Copy a link, open it in another browser, and that browser will have your settings. Anyone with the link can change them, so keep it to yourself.');
}
