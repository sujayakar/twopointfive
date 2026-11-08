// WHAT THE END CARD SHOWS, read the way the page reads it: the Rust game's record (src/view/debriefview.ts). A check asks here; nothing of the page's own works it out.
const mmss = (t: number) => { const s = Math.max(0, Math.floor(t)); return `${String(Math.floor(s / 60)).padStart(2, '0')}:${String(s % 60).padStart(2, '0')}`; };

export const debriefOf = (g: any) => g.debriefView().debrief;
export const runCounts = (g: any) => debriefOf(g).counts;
export const badgesOf = (g: any) => debriefOf(g).badges as { id: string; name: string; on: boolean; hint: string }[];
export const rowsOf = (g: any) => g.debriefView().rows as { t: number; kind: string; room: string; pos: number[]; text: string; down: boolean }[];
export const recapOf = (g: any) => g.debriefView().recap;
/** a row as the pin's hover reads it */
export const rowLine = (e: { t: number; room: string; text: string }) => `${mmss(e.t)} · ${e.room} · ${e.text}`;
