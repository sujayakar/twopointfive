// HOW KEPT A PLACE IS, the generator's side (docs/level-format.md meta.kept; docs/agents/render.md, A LEVEL SAYS HOW KEPT IT IS). A generator says the word
// as `export const KEPT = <0 to 1>`. A look's second file is the same PLACE in other clothes (`DRESS_AS` names the level it dresses), so it has that
// level's word unless it says its own, however many files stand between. The exporter asks this function and so does the check (tools/qa/kept-check.ts),
// so the rule has one writing. A generator that builds another's floor with no `DRESS_AS` (the block's squads level on the block) is another LEVEL of the
// same place: it says the word itself, `export { KEPT } from './block'`.
export interface SaysKept { KEPT?: number; DRESS_AS?: string }
export async function generatorKept(gen: SaysKept, load: (name: string) => Promise<SaysKept>, depth = 0): Promise<number | undefined> {
  if (gen.KEPT !== undefined || !gen.DRESS_AS || depth > 8) return gen.KEPT;
  return generatorKept(await load(gen.DRESS_AS), load, depth + 1);
}
