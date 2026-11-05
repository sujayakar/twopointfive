// WHAT A LEVEL FILE SAYS, wherever it says it: one walk over a level's data for every object a reader takes. A leaf: it loads nothing of the game or the
// harness, so a tool that only reads level files (tools/qa/walked-check.ts) can ask it. tools/qa/marksight-check.ts reads its men by it.
/** EVERYTHING A LEVEL FILE SAYS OF ONE KIND, outside its boxes and its props: each thing `take` knows, with a name. A thing is named by where the file has
 *  it, under `scenarios` without that word; one of a list by its place in it (`rescue.guards#0`), and a thing of a list that says an `id` by its id in
 *  place of the list's name (`roomClear.tea.marks#0`: the tea room's first mark, wherever the room stands in the list). Nothing here names a kind of
 *  mission, so the next kind is read the day it is written. (tools/qa/walked-check.ts reads a level's starts by it.) */
export function saidIn<T>(data: any, take: (o: any) => T | null): { id: string; v: T }[] {
  const out: { id: string; v: T }[] = [];
  const walk = (o: any, path: string): void => {
    if (!o || typeof o !== 'object') return;
    if (Array.isArray(o)) { o.forEach((x, k) => walk(x, typeof x?.id === 'string' ? `${path.slice(0, path.lastIndexOf('.') + 1)}${x.id.replace(/\s/g, '_')}` : `${path}#${k}`)); return; }
    const v = take(o); if (v !== null) out.push({ id: path, v });
    for (const [k, w] of Object.entries(o)) walk(w, path ? `${path}.${k}` : k);
  };
  for (const [k, v] of Object.entries(data)) if (k === 'scenarios') walk(v, ''); else if (k !== 'boxes' && k !== 'props') walk(v, k);
  return out;
}
