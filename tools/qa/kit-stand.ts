// A KIT STOOD UP EMPTY, for a check that lays one piece and looks at what it laid (tools/qa/ground-pieces-check.ts, tools/qa/kit-stores-check.ts), as
// tools/levels/library.ts lays a piece for the catalogue. One place, so that a change to how a kit is stood up is made once.
import type { Box } from '../../src/scene/boxes';
import { officeKit } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice } from '../../src/scene/gwotKit';
import { GWOT_PALETTE, type GwotMat } from '../../src/scene/gwotPalette';

export type Kit = ReturnType<typeof gwotKit>;
/** the look's kit over an office kit with nothing in it, and the group of every box it is handed */
export const standKit = () => { const groups: string[] = [], K = officeKit({ groups }); return { K, G: gwotKit(gwotOfOffice(K)), groups }; };
/** what one call of `lay` adds to a kit stood up empty (after `before`, which is not counted), what the call handed back, and the kit it was laid in */
export const laid = <T>(lay: (G: Kit) => T, before?: (G: Kit) => void): { boxes: Box[]; back: T; K: ReturnType<typeof standKit>['K']; G: Kit } => { const { K, G } = standKit(); before?.(G); const n = K.boxes.length, back = lay(G); return { boxes: K.boxes.slice(n), back, K, G }; };
/** every row of the palette: a check that asks them all asks the row added tomorrow */
export const ROWS = Object.keys(GWOT_PALETTE) as GwotMat[];
/** a check's lines: PASS or FAIL with what was held and what was found, and how many failed */
export const lines = () => { let failed = 0; return { check: (ok: boolean, what: string, detail = '') => { if (!ok) failed++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); }, failed: () => failed }; };
