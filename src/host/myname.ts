// THE NAME THIS BROWSER'S PLAYER PICKED (host/names.ts has what a page calls the others): typed on the title's Co-op page, kept for the next visit. '' = none, he is his seat's number.
import { cleanName } from '../net/wire';

const LS = 'twopointsix.name';
let mem: string | null = null;   // (a browser with no store: the name lasts the page)
export function myName(): string { if (mem !== null) return mem; try { return cleanName(localStorage.getItem(LS)); } catch { return ''; } }
/** …as he types it: cleaned, kept, and handed back as kept */
export function setMyName(raw: string): string { const n = cleanName(raw); try { if (n) localStorage.setItem(LS, n); else localStorage.removeItem(LS); mem = null; } catch { mem = n; } return n; }
