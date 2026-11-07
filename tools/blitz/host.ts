// THE HOST'S SIDE OF THE MISSION LOOP (README.md): the bridge's walk with its book readable, and ONE PART of a game as bytes, so that the host's half of a tick
// is a couple of kilobytes and not a whole state (a whole state grows by the frame's boxes every tick).
// (export.ts exports the book and the two part writers itself.)
const X: any = await import('./export.ts');
export const exportGame: (g: any, knobs: Record<string, unknown>) => Uint8Array = X.exportGame;
export const report: () => Map<string, { n: number; eg: string }> = X.report;
/** the bridge's book of a game that has been exported once: `ids.get('DoorId')` (object → ID), `listOf('GuardId')` (a table's rows in ID order) */
export const bookOf: (g: object) => any = X.bookOf;
export const exportPart: (g: any, knobs: Record<string, unknown>, parts: [string, any][]) => Uint8Array = X.exportPart;
export const fieldAt: (g: any, knobs: Record<string, unknown>, path: string[]) => { ty: string; v: any } = X.fieldAt;
