// DRAFT, NOT FOR THE BUILD: the depot as depot-gwot.ts lays it, in the second of the three palettes shown to the maintainer (depot-gwot.ts YARD_B).
// `bun run tools/levels/export.ts depot-roof-b` → levels/depot-roof-b.json · page: ?levelFile=depot-roof-b.json&roomclear&squad4
import { build as buildPlain, NOTES as PLAIN_NOTES } from './depot';
import { COAST_MATERIALS } from '../../src/scene/gwotPalette';
import { skinLevel } from './gwotSkin';
import { yardSkin, YARD_B } from './depot-gwot';
import type { Level } from '../../src/scene/level';
export const NAME = 'depot-roof-b';
export const DRESS_AS = 'depot';
export const MATERIALS = COAST_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''} — DRAFT: a palette beside the one laid.`;
export const build = buildPlain;
export { KEPT } from './depot-gwot';
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, yardSkin(YARD_B));
