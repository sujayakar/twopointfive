// DRAFT, NOT FOR THE BUILD: the depot as depot-gwot.ts lays it, deeper and painted as a private yard (depot-gwot.ts YARD_D, LAY_YARD).
// `bun run tools/levels/export.ts depot-roof-d` → levels/depot-roof-d.json · page: ?levelFile=depot-roof-d.json&roomclear&squad4
import { build as buildPlain, NOTES as PLAIN_NOTES } from './depot';
import { COAST_MATERIALS } from '../../src/scene/gwotPalette';
import { skinLevel } from './gwotSkin';
import { yardSkin, YARD_D, LAY_YARD } from './depot-gwot';
import type { Level } from '../../src/scene/level';
export const NAME = 'depot-roof-d';
export const DRESS_AS = 'depot';
export const MATERIALS = COAST_MATERIALS;
export const NOTES = `${PLAIN_NOTES ?? ''} — DRAFT: a deeper palette and a private yard's paint, beside the one laid.`;
export const build = buildPlain;
export { KEPT } from './depot-gwot';
export const finish = (level: Level, groups: string[]) => skinLevel(level, groups, yardSkin(YARD_D, LAY_YARD));
