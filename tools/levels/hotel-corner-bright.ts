// HOTEL-CORNER-BRIGHT: tools/levels/hotel-corner.ts again with every paint a third brighter (its terrace stone 0.63 where the corner's is 0.47), so the two
// can be drawn side by side: how pale a sunlit plane may be painted before it sits at white is what is being found out. A look test, in no mission list.
//   bun run tools/levels/export.ts hotel-corner-bright [--check]     → levels/hotel-corner-bright.json
import { build as corner, MATERIALS } from './hotel-corner';
export const NAME = 'hotel-corner-bright';
export const NOTES = 'A look test: tools/levels/hotel-corner.ts with every paint a third brighter, to set beside it. Not in the missions list. Regenerate: bun run tools/levels/export.ts hotel-corner-bright';
export { MATERIALS };
export function build() { return corner(1.34); }
