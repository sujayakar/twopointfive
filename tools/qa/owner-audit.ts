// THE OWNER CHECK'S HANDS IN A RUN (tools/qa/owner-check.ts runs a mission with this as a Bun preload; no file of the repo changes): at every read of the record and at the end of every
// tick, every man's word against what each rule's own state says (`ownerWitness`, one line in the rule's file). At the process's exit what it found goes to OWNER_OUT.
// OWNER_DROP=<mask>: claims of those rules are dropped (the check's red path).
import { plugin } from 'bun';
import { writeFileSync } from 'node:fs';
import { OWNER_QA, ownerDisagrees, ownerWitnessed, OWN } from '../../src/game/owner.ts';

const out: { ticks: number; reads: Record<string, [number, number]>; held: Record<string, number>; bad: number; first: string[] } = { ticks: 0, reads: {}, held: {}, bad: 0, first: [] };
const G = globalThis as unknown as { __ownerGame: { time: number; guards: any[] } | null; __ownerEnd: (g: any) => void };
G.__ownerGame = null;
const note = (where: string, gd: any, D: { name: string; record: boolean; rule: boolean }[]) => {
  out.bad += D.length;
  if (out.first.length < 12) for (const d of D) out.first.push(`${d.name} on ${gd.callsign} at ${G.__ownerGame?.time.toFixed(2)} s: the record says ${d.record}, the rule ${d.rule} (${where})`);
};
const caller = () => (new Error().stack ?? '').split('\n').find(l => /\/src\/game\//.test(l) && !/owner\.ts/.test(l))?.trim().replace(/^at /, '').replace(/.*\/src\/game\//, '') ?? '?';
OWNER_QA.drop = Number(process.env.OWNER_DROP ?? 0);
OWNER_QA.read = (gd, rules) => {
  const g = G.__ownerGame; if (!g) return;
  for (const name of ownerWitnessed()) { const b = (OWN as Record<string, number>)[name]; if (b & rules) { const R = out.reads[name] ??= [0, 0]; R[0]++; if (gd.owner & b) R[1]++; } }
  const D = ownerDisagrees(gd, g, rules); if (D.length) note(`read in ${caller()}`, gd, D);
};
G.__ownerEnd = g => {
  out.ticks++;
  for (const gd of g.guards) {
    const D = ownerDisagrees(gd, g, ~0, false); if (D.length) note('end of the tick', gd, D);
    for (const name of ownerWitnessed()) if (gd.owner & (OWN as Record<string, number>)[name]) out.held[name] = (out.held[name] ?? 0) + 1;
  }
};
process.on('exit', () => { if (process.env.OWNER_OUT) writeFileSync(process.env.OWNER_OUT, JSON.stringify(out)); });
plugin({ name: 'owner-audit', setup(b) { b.onLoad({ filter: /src\/game\/game\.ts$/ }, async (a: { path: string }) => {
  let t = await Bun.file(a.path).text();
  if (!t.includes('this.ticking = true;') || !t.includes('\n    this.ticking = false;')) throw new Error('owner-audit: game.ts no longer has the lines it hangs on (this.ticking = true / false)');
  t = t.replace('this.ticking = true;', '(globalThis as any).__ownerGame = this; this.ticking = true;').replace('\n    this.ticking = false;', '\n    (globalThis as any).__ownerEnd(this); this.ticking = false;');
  return { contents: t, loader: 'ts' };
}); } });
