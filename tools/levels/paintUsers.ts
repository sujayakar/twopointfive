// EVERYTHING IN A LEVEL FILE THAT WEARS A PAINT, and the material row it wears: what a tool that reads "what is each box made of" walks (the material check,
// tools/qa/stuff-check.ts; and, while files from before 2026-11-04 exist, tools/levels/stuff-bake.ts and tools/qa/stuff-audit.ts).
export type Paints = { material?: string; albedo?: number[]; stuff?: string };
export type Data = { materials: Record<string, { albedo: number[]; stuff?: string }>; boxes: Paints[]; props: { parts: Paints[] }[]; scenarios?: { kitView?: { ways?: { add?: { boxes?: Paints[] } }[]; sets?: Record<string, { boxes: Paints[] }> } } };
/** everything in a level file that wears a paint, with where it is: the boxes, the props' parts, and a gear screen's own boxes (its sets, a way's pieces) */
export function* paintUsers(d: Data): Generator<{ o: Paints; where: string; kind: 'box' | 'part' | 'set' }> {
  for (let i = 0; i < d.boxes.length; i++) yield { o: d.boxes[i], where: `boxes[${i}]`, kind: 'box' };
  for (let i = 0; i < d.props.length; i++) for (let j = 0; j < d.props[i].parts.length; j++) yield { o: d.props[i].parts[j], where: `props[${i}].parts[${j}]`, kind: 'part' };
  const K = d.scenarios?.kitView;
  for (const [name, T] of Object.entries(K?.sets ?? {})) for (let i = 0; i < T.boxes.length; i++) yield { o: T.boxes[i], where: `kitView.sets.${name}.boxes[${i}]`, kind: 'set' };
  for (let w = 0; w < (K?.ways ?? []).length; w++) { const add = K!.ways![w].add?.boxes ?? []; for (let i = 0; i < add.length; i++) yield { o: add[i], where: `kitView.ways[${w}].add.boxes[${i}]`, kind: 'set' }; }
}
/** the row a paint user wears: its `material`, or for a gear screen's box that names alternatives ('look|plain') the first the file has (host/kitViewSet.ts) */
export const rowOf = (d: Data, o: Paints): string | undefined => o.material === undefined ? undefined : !o.material.includes('|') ? o.material : o.material.split('|').find(m => Object.hasOwn(d.materials, m)) ?? o.material.split('|')[0];
/** a file from before says nothing in its rows */
export const fromBefore = (d: Data): boolean => !Object.values(d.materials).some(m => m.stuff !== undefined);
