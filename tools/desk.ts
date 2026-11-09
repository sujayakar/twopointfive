// THE DESK, for a tool with no page: the game's module asked as the page asks it (src/view/desk.ts). It is the page's one desk from here on (src/view/desk.ts `desk`), so what of the page
// asks it answers too. Nothing is kept.
import { readFileSync } from 'node:fs';
import { Desk, setDesk, type DeskFace } from '../src/view/desk';
import type { MissionRow } from '../src/view/read';
import { faceHere } from './blitz/face-build.ts';

let made: WebAssembly.Module | null = null, standing: Desk | null = null;
/** the module of these sources, compiled once a process */
export function toolModule(): WebAssembly.Module {
  if (made) return made;
  const file = faceHere(); if (!file) throw new Error('the game\'s module is not built and the checked-in one is of other sources (bun run tools/blitz/face-build.ts)');
  return made = new WebAssembly.Module(new Uint8Array(readFileSync(file)));
}
export function toolDesk(): Desk {
  if (standing) return standing;
  const X = new WebAssembly.Instance(toolModule()).exports as unknown as DeskFace & { memory: WebAssembly.Memory };
  setDesk(standing = new Desk(X, X.memory, () => { /* a tool keeps nothing */ }));
  return standing;
}
/** every mission there is: the plain solo night, then the tables in their order (rust/game_types/src/scene_scenario_list/rows.rs) */
export const missions = (): MissionRow[] => toolDesk().missions().rows;
/** …without the solo night, which is no row of a table */
export const missionRows = (): MissionRow[] => missions().filter(m => m.kind !== 'solo');
