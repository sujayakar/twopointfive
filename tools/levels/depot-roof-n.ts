// DRAFT, NOT FOR THE BUILD: the depot as depot-gwot.ts lays it, in no accent: the planes, edges and marks alone (depot-gwot.ts YARD_N).
// `bun run tools/levels/export.ts depot-roof-n` → levels/depot-roof-n.json · page: ?levelFile=depot-roof-n.json&roomclear&squad4
import { build as buildPlain, NOTES as PLAIN_NOTES } from './depot';
import { COAST_MATERIALS } from '../../src/scene/gwotPalette';
import { skinLevel } from './gwotSkin';
import { yardSkin, YARD_N } from './depot-gwot';
import type { Level } from '../../src/scene/level';
export const NAME = 'depot-roof-n';
export const DRESS_AS = 'depot';
export const MATERIALS = COAST_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''} — DRAFT: a palette beside the one laid.`;
export const build = buildPlain;
export { KEPT } from './depot-gwot';
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, yardSkin(YARD_N));
