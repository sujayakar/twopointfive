// THE HOST'S SIDE OF THE TICK LOOP (README.md "Host steps"): the bridge's walk with its books readable, and what the host holds that a tick reads and the bridge
// does not carry. HARNESS STAND-INS FOR BRIDGE GAPS: each goes when the bridge carries the thing itself.
import { plugin } from 'bun';
// (export.ts keeps its object → ID books private, and a light of the engine's list has to be named by the ID the walk gave it. One line is added to the module as it
// is loaded, in memory: no file is written and export.ts is not touched.)
plugin({ name: 'blitz-books', setup(build) {
  build.onLoad({ filter: /[\\/]tools[\\/]blitz[\\/]export\.ts$/ }, async ({ path }) => {
    const text = await Bun.file(path).text(); if (!/\bconst BOOKS\b/.test(text)) throw new Error('tools/blitz/host.ts: export.ts no longer has a table called BOOKS');
    return { contents: `${text}\nexport const bookOf = (g: object) => BOOKS.get(g);\n`, loader: 'ts' };
  });
} });
const X: any = await import('./export.ts');
export const exportGame: (g: any, knobs: Record<string, unknown>) => Uint8Array = X.exportGame;
export const bookOf: (g: object) => any = X.bookOf;

/** HARNESS STAND-IN: `Game.engine` as text, a line an item, for rust/game's `host_api::b_01::host_engine_fill`. The bridge leaves the engine out as the picture's,
 *  but a tick reads the lights' list in its own order, the box world's lists and counts, and every plain setting. Lights are named by the bridge's own LightIds;
 *  one the walk never reached has no row and is handed over whole (`light_new`, named `n<k>` in the lists). Take it after an export of the same game. */
export function engineText(g: any): string {
  const E = g.engine, bk: any = bookOf(g), fresh: any[] = [];
  const id = (l: any) => { const i = bk.ids.get('LightId')?.get(l); if (i !== undefined) return String(i); let k = fresh.indexOf(l); if (k < 0) { k = fresh.length; fresh.push(l); } return `n${k}`; };
  const L: string[] = [];
  L.push(`lights ${E.lights.lights.map(id).join(' ')}`, `ir ${E.lights.ir.map(id).join(' ')}`, `fog_team ${+E.lights.fogTeam.on}`, `fog_owners ${[...E.lights.fogTeam.owners].join(' ')}`, `fog_lights ${[...E.lights.fogTeam.lights].map(id).join(' ')}`);
  for (const l of fresh) L.unshift(`light_new ${(l.name || '-').replace(/\s/g, '_')} ${(l.group || '-').replace(/\s/g, '_')} ${[l.kind, ...l.pos, ...l.dir, ...l.color, l.intensity, l.range, l.innerDeg, l.outerDeg, l.radius, l.volumetric, l.owner, +l.enabled, l.fixtureBox, l.flicker, l.ttl, l.age, l.peakIntensity, +l.broad].join(' ')}`);
  const W = E.world; for (let i = 0; i < W.statics.length; i++) if (W.statics[i] !== g.col.boxes[i]) throw new Error(`the engine's static ${i} is not the level's`);
  L.push(`max_boxes ${W.maxBoxes}`, `world_statics ${W.statics.length}`, `geom_serial ${W.geomSerial}`, `glass_statics ${[...(W.glassStatics ?? [])].join(' ')}`, `glass_statics_n ${W.glassStaticsN}`, `glass_list ${[...(W.glassList ?? [])].join(' ')}`, `effective_scale ${E.effectiveScale}`);
  for (const [k, v] of Object.entries(E.settings)) { const row = typeof v === 'number' ? [v] : typeof v === 'boolean' ? [+v] : Array.isArray(v) && v.every(x => typeof x === 'number') ? v : null; if (row && row.length) L.push(`set ${k.toLowerCase()} ${row.join(' ')}`); }
  return L.join('\n') + '\n';
}
