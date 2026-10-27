// THE KEPT REPLAYS, on a page: this browser's store of them (one connection a page) and what is kept, in a line. Shared by the game page (ui/replaykey.ts keeps
// into it; main.ts hands it to the backend's sync, src/cloud/tapes.ts) and the replay viewer, which lists it. Nothing here sends or downloads anything.
// (This file was `dumpui.ts`, the hand-over: every kept replay as one downloaded file. That was taken out on 2026-10-27 — a hosted build's backend reads the
// store by itself, and one replay still leaves the viewer as a file with 'Download with notes'.)
import type { KeptStore, KeptSummary } from './keepstore';
import { browserStore, keptSummary } from './keepstore';

let store: KeptStore | null = null;
/** this browser's store of kept tapes (one connection a page) */
export const keptStore = (): KeptStore => store ??= browserStore();

export const mb = (bytes: number): string => bytes >= 1048576 * 10 ? `${Math.round(bytes / 1048576)} MB` : bytes >= 104858 ? `${(bytes / 1048576).toFixed(1)} MB` : `${Math.max(1, Math.round(bytes / 1024))} KB`;
export const mins = (s: number): string => s >= 90 ? `${Math.round(s / 60)} min` : `${Math.round(s)} s`;
/** what is kept, in a line */
export function keptWords(S: KeptSummary): string { return S.visits ? `${S.visits} kept (${mins(S.seconds)}, ${mb(S.bytes)})` : 'nothing kept yet'; }
export function keptNow(): Promise<KeptSummary> { return keptSummary(keptStore()); }
