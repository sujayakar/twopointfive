// WHAT THE PAGE ITSELF SAYS: a save's word, a capture's, a panel's answer. None of it is the game's, and no game hears it: the HUD draws it on a strip of its own
// (ui/overlay.ts, under the radio's rows), on the page's clock. Imports nothing of the game (tools/qa/view-check.ts).
const ROWS = 3, SECS = 6;
const said: { text: string; at: number }[] = [];
/** one line for the page's strip */
export function pageSay(text: string): void { said.push({ text, at: performance.now() }); if (said.length > ROWS) said.shift(); }
/** the lines up now, oldest first */
export function pageRows(): string[] { const now = performance.now(); return said.filter(r => now - r.at < SECS * 1000).map(r => r.text); }
