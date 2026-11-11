// The perf SWEEP take read back (src/ui/sweepread.ts) as text and as markdown, and the smoke count it stands beside.
import { mean, readSweeps, type Begin, type Dump, type Frame, type RowPrice, type Spread, type SweepResult } from '../../src/ui/sweepread';
export { readSweeps, type Dump, type Frame, type Ev } from '../../src/ui/sweepread';

const sgn = (v: number, d = 2) => (v >= 0 ? '+' : '−') + Math.abs(v).toFixed(d);
const sp = (s: Spread | null, d = 2) => s ? `${sgn(s.mean, d)} (${sgn(s.lo, d)} … ${sgn(s.hi, d)})` : 'n/a';
const rmsNote = (R: SweepResult) => { const byU = R.noise.union !== null && R.noise.rmsUnion !== null, u = byU ? R.noise.union! : R.noise.dt, r = byU ? R.noise.rmsUnion! : R.noise.rmsDt; return Number.isFinite(r) && u !== r ? `, read off the median difference: the plain rms is ±${r.toFixed(2)}, so stretches of this take ran slow` : ''; };
const rounds = (r: RowPrice) => r.per.map((v, i) => sgn(v) + (r.disturbed[i] ? '★' : '')).join(' ');
/** WHAT A TAKE'S SMOKE COUNT COUNTS. On the box solver it was the live clouds; on the solver without boxes it is the brick pool in 512s, rounded, 1 at the least while anything
 *  is alive (the dust row's label has the pool itself). Read off the file, the first of these that speaks: the recorder's stamp (bricks to a count) · ?sparsesmoke among the
 *  page's flags (the pool, while both solvers were in the build) · the settings the take started with (`smokeRenderSkip` was one only while the box solver was in) · how its
 *  sweep worded the dust row · else nothing says ('either': a file with no settings and no such row) */
export type SmokeUnit = { kind: 'bricks'; per: number } | { kind: 'clouds' } | { kind: 'either' };
export function smokeUnit(D: Dump): SmokeUnit {
  const pool = (per = 512): SmokeUnit => ({ kind: 'bricks', per });
  if (typeof D.smokeUnit === 'number' && D.smokeUnit > 0) return pool(D.smokeUnit);
  if (/[?&]sparsesmoke\b/.test(D.query ?? '')) return pool();
  if (D.settingsAtStart && Object.keys(D.settingsAtStart).length) return 'smokeRenderSkip' in D.settingsAtStart ? { kind: 'clouds' } : pool();
  const label = D.events.filter(e => e.kind === 'sweep' && /"op":"begin"/.test(e.text)).map(e => (JSON.parse(e.text) as Begin).rows.find(r => r.id === 'smoke')?.label ?? '').find(Boolean) ?? '';
  return /bricks/.test(label) ? pool() : /cloud/.test(label) ? { kind: 'clouds' } : { kind: 'either' };
}
const UNIT_NOTE = 'clouds on a take from the box solver, the brick pool in 512s on a take after it';
const poolNote = (u: { per: number }) => `the brick pool in ${u.per}s, rounded, 1 at the least`;
const smokeLive = (n: number, u: SmokeUnit) => !n ? 'no dust / smoke live' : u.kind === 'clouds' ? `${n} dust / smoke cloud${n === 1 ? '' : 's'} live` : `dust / smoke live: ${n} (${u.kind === 'bricks' ? poolNote(u) : UNIT_NOTE})`;
const head = (R: SweepResult, unit: SmokeUnit) => {
  const B = R.begin, b = R.base;
  return [
    `${B.mode === 'frozen' ? 'world frozen' : 'world running'} · ${B.preset} · ${B.res[0]}×${B.res[1]} (render scale ×${(+B.scale).toFixed(2)}) · ${smokeLive(B.smoke, unit)} · ${B.rows.length} rows × ${B.rounds} rounds, ${R.segments} segments${R.done ? '' : ` · NOT finished (${R.why})`}`,
    `base: ${b.union !== null ? `GPU time (union) ${b.union.toFixed(2)} ms · ${b.sum !== null ? `pass sum ${b.sum.toFixed(2)} ms · ` : ''}` : 'no GPU timers in this take, frame time only · '}frame ${b.dt.toFixed(2)} ms = ${b.fps.toFixed(1)} fps (${R.baseFrames} frames with nothing flipped)`,
    ...(B.mode === 'frozen' ? ['a frozen take: the smoke solver and the light meter do not run while the world stands still, so the base reads under play by their cost, and the dust row prices drawing the dust and smoke, not solving it'] : []),
    `noise: with nothing flipped one round reads ±${R.noise.union !== null ? `${R.noise.union.toFixed(2)} ms of GPU time, ±` : ''}${Number.isFinite(R.noise.dt) ? R.noise.dt.toFixed(2) : '?'} ms of frame time (1 sd over ${R.noise.n} base segments${rmsNote(R)}); a row whose median round is inside twice that over its rounds is marked ≈ 0`,
    ...(R.disturbed ? [`slow stretches: ${R.disturbed} round${R.disturbed === 1 ? '' : 's'} marked ★ had base segments either side that disagree by more than ${R.noise.gap.toFixed(2)} ms (the machine ran slower for a moment: something outside the feature); read those rows by their median and their other rounds`] : []),
  ];
};

/** the report's text section (appended to perf-report.ts's tables) */
export function sweepText(D: Dump): string[] {
  const L: string[] = [];
  for (const R of readSweeps(D)) {
    L.push('', 'SWEEP: what each feature gives back when flipped (base − flipped; + = cheaper flipped; mean over the rounds, lowest … highest round)'); for (const h of head(R, smokeUnit(D))) L.push('  ' + h);
    const u = R.rows.some(r => r.union);
    L.push(`  ${'row'.padEnd(46)} | GPU union ms            | median | ${`rounds${u ? '' : ' (frame ms)'}`.padEnd(22)} | frame ms               | fps                    | Σ passes | passes that moved`);
    for (const r of R.rows) L.push(`  ${r.label.slice(0, 46).padEnd(46)} | ${sp(r.union).padEnd(22)} | ${sgn(r.median).padStart(6)} | ${rounds(r).padEnd(22)} | ${sp(r.frame).padEnd(22)} | ${sp(r.fps, 1).padEnd(22)} | ${(r.sum ? sgn(r.sum.mean) : 'n/a').padStart(8)} | ${r.movers.map(m => `${m.pass} ${sgn(m.back)}`).join(', ') || '—'}${r.inNoise ? '   ≈ 0 (inside the noise)' : ''}   [${r.change}]`);
    if (R.begin.skipped.length) L.push(`  not flipped here: ${R.begin.skipped.map(s => `${s.id} (${s.why})`).join(' · ')}`);
    const bp = Object.entries(R.base.pass).sort((a, b) => b[1] - a[1]);
    if (bp.length) L.push(`  base passes (ms, nothing flipped): ${bp.map(([k, v]) => `${k} ${v.toFixed(2)}`).join(' · ')}`);
  }
  return L;
}

/** the feature table as markdown (the page's form) */
export function sweepMarkdown(D: Dump): string[] {
  const L: string[] = [];
  for (const R of readSweeps(D)) {
    for (const h of head(R, smokeUnit(D))) L.push(`- ${h}`); L.push('');
    L.push(`| Feature flipped | GPU time given back (ms) | Median round | Round by round${R.disturbed ? ' (★ = across a slow stretch)' : ''} | Share of base | fps given back | Frame time given back (ms) | Its passes fell by (ms) | Passes that moved |`, '|---|---|---|---|---|---|---|---|---|');
    for (const r of R.rows) L.push(`| ${r.label}${r.inNoise ? ' (≈ 0, inside the noise)' : ''} | ${sp(r.union)} | ${sgn(r.median)} | ${rounds(r)} | ${r.union && R.base.union ? `${Math.round(100 * r.union.mean / R.base.union) || 0} %` : 'n/a'} | ${sp(r.fps, 1)} | ${sp(r.frame)} | ${r.sum ? sgn(r.sum.mean) : 'n/a'} | ${r.movers.map(m => `${m.pass} ${sgn(m.back)}`).join(', ') || 'none'} |`);
    if (R.begin.skipped.length) L.push('', `Not flipped in this take: ${R.begin.skipped.map(s => `${s.id} (${s.why})`).join('; ')}.`);
    L.push('');
  }
  return L;
}

/** GPU time by how much dust / smoke was live (the frames' smoke count: see smokeUnit), at the take's most common preset and resolution (any take: an observation across the take's moments, not an A/B) */
export function smokeText(D: Dump): string[] {
  const sweepFrames = new Set<number>(); { const ev = D.events.filter(e => e.kind === 'sweep'); for (let k = 0; k + 1 < ev.length; k += 1) { if (/"op":"begin"/.test(ev[k].text)) { let e = k + 1; while (e < ev.length && !/"op":"end"/.test(ev[e].text)) e++; for (let i = ev[k].frame; i < (ev[e]?.frame ?? D.frames.length); i++) sweepFrames.add(i); } } }
  const F = D.frames.filter((_, i) => !sweepFrames.has(i)); if (!F.length || !F.some(f => Number(f.n.smoke) > 0)) return [];
  const key = (f: Frame) => `${f.preset} ${f.res[0]}×${f.res[1]}`; const count = new Map<string, number>(); for (const f of F) count.set(key(f), (count.get(key(f)) ?? 0) + 1);
  const top = [...count.entries()].sort((a, b) => b[1] - a[1])[0][0]; const G = F.filter(f => key(f) === top);
  const by = new Map<number, Frame[]>(); for (const f of G) { const n = Number(f.n.smoke) || 0; if (!by.has(n)) by.set(n, []); by.get(n)!.push(f); }
  const u = smokeUnit(D);
  const L = ['', `GPU TIME BY LIVE DUST / SMOKE${u.kind === 'clouds' ? ' CLOUDS' : ''} (${top}; different moments of the take, so the view changes too: an observation, not an A/B${u.kind === 'clouds' ? '' : `; count = ${u.kind === 'bricks' ? poolNote(u) : UNIT_NOTE}`}): ${u.kind === 'clouds' ? 'clouds' : u.kind === 'bricks' ? `pool ÷ ${u.per}` : 'count'} | frames | fps | GPU union | smoke solver | beams + march | direct | probe`];
  for (const n of [...by.keys()].sort((a, b) => a - b)) {
    const fr = by.get(n)!; if (fr.length < 20) continue; const u = fr.filter(f => typeof f.gpuUnion === 'number'); const p = (k: string) => mean(fr.map(f => f.gpu[k] ?? 0)).toFixed(2).padStart(6);
    L.push(`  ${String(n).padStart(6)} | ${String(fr.length).padStart(6)} | ${(1000 / mean(fr.map(f => f.dt))).toFixed(1).padStart(5)} | ${(u.length ? mean(u.map(f => f.gpuUnion!)).toFixed(2) : 'n/a').padStart(9)} | ${p('smoke')} | ${p('volumetrics')} | ${p('direct')} | ${p('probe')}`);
  }
  return L.length > 1 ? L : [];
}
