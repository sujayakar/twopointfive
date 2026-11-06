// TWO FACTS OF A START THE BRIDGE DOES NOT CARRY, written beside it (starts.txt) so that the runner states them and does not assume them:
//   which pose sets are AT HOME in the game's block (`PoseSet.home`, anim/skeleton.ts), and
//   what each animator holds for its body's place (`anim.posIn`: the body's own position ARRAY from a Character.update on, character.ts:4419, which a
//   stride replaces with a new array; an animator that holds an older array reads other numbers).
// A line a start: `<k> home <set ids…> | posin <char id>:<x>,<y>,<z> … | odd <n>`. The ids are the bridge's own (the book of the export just made, so
// call this straight after `exportGame`). `posin` lists only the animators whose array is NOT the body's own, with the numbers it holds; `odd` counts
// what the runner's old rule would have got wrong: a set at home that is not on the game's list of sets at home or the other way round, and those
// animators. The runner prints it.
import { bookOf } from './host.ts';
export function startFacts(g: any, k: number): string {
  const B = bookOf(g), sets: Map<any, number> = B.ids.get('PoseSetId') ?? new Map(), chars: Map<any, number> = B.ids.get('CharId') ?? new Map();
  const held = new Set<any>(g.poseSets ?? []), home: number[] = [], posin: string[] = []; let odd = 0;
  for (const [s, id] of sets) { const at = s.home !== null && s.home !== undefined; if (at) home.push(id); if (at !== held.has(s)) odd++; }
  for (const [c, id] of chars) { const p = c.anim?.posIn; if (p && p !== c.pos) { posin.push(`${id}:${p[0]},${p[1]},${p[2]}`); odd++; } }
  return `${k} home ${home.join(' ')} | posin ${posin.join(' ')} | odd ${odd}`;
}
