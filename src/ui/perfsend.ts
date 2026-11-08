// THE SHORT READ OF A TAKE (ui/lookpanel.ts). The F9 recorder's take (ui/perfrec.ts) leaves a page as a file; here is a short read of it (takeRead: the frame
// times, the slow frames one by one with what each cost, the passes) that fits a text box with a Copy button. A sweep's rows are NOT priced here:
// tools/qa/perf-report.ts prices them off the whole take, by its own rules.
import type { PerfDump, PerfFrame } from './perfrec';

const r2 = (v: number) => Math.round(v * 100) / 100;
const mean = (a: number[]) => a.length ? a.reduce((s, v) => s + v, 0) / a.length : 0;
const at = (sorted: number[], q: number) => sorted.length ? sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))] : 0;
const cost = (f: PerfFrame) => f.gpuUnion ?? f.gpuSum;

export interface TakeRead { [k: string]: unknown; id: string; frames: number; seconds: number; dt: Record<string, number>; over: Record<string, number>; slow: unknown[]; passes: Record<string, [number, number]>; sweep: { rows: number; rounds: number; segments: number; skipped: unknown; end: unknown } | null }

/** the short read of a take. Frames inside a sweep's settle (the flip itself hitches: a program compiles, a history starts again) are left out of the slow list.
 *  The GPU figure of a frame is the compute passes' (the recorder's union, or their sum in a take without it): a render pass's timer is not to be trusted on every GPU */
export function takeRead(dump: PerfDump, id: string): TakeRead {
  const F = dump.frames, n = F.length, dts = F.map(f => f.dt).sort((a, b) => a - b), med = at(dts, 0.5);
  const segs = dump.events.filter(e => e.kind === 'sweep').map(e => { try { return { e, j: JSON.parse(e.text) as Record<string, unknown> }; } catch { return null; } }).filter((x): x is { e: PerfDump['events'][number]; j: Record<string, unknown> } => !!x);
  const begin = segs.find(s => s.j.op === 'begin')?.j, settle0 = Number(begin?.settleMs ?? 500);
  const seg = segs.filter(s => s.j.op === 'seg'), end = segs.find(s => s.j.op === 'end');
  const b0 = segs.find(s => s.j.op === 'begin')?.e.t, s0 = seg[0]?.e.t ?? Infinity;
  const quiet = (t: number) => !(b0 !== undefined && t >= b0 - 50 && t < s0) && !seg.some(s => t >= s.e.t - 50 && t < s.e.t + Number(s.j.settleMs ?? settle0));   // (not the sweep's warm-up, not a flip's settle)
  // the slow frames: well over the take's own middle frame, the forty slowest, each with the passes that ran over THEIR middle and what was said near it
  const passMed: Record<string, number> = {}, passAll: Record<string, number[]> = {};
  for (const f of F) for (const k of Object.keys(f.gpu)) (passAll[k] ??= []).push(f.gpu[k]);
  const passes: Record<string, [number, number]> = {};
  for (const k of Object.keys(passAll)) { const s = passAll[k].sort((a, b) => a - b); passMed[k] = at(s, 0.5); passes[k] = [r2(mean(s)), r2(at(s, 0.95))]; }
  const line = Math.max(med * 1.6, med + 8);
  const slow = F.map((f, i) => ({ f, i })).filter(x => x.f.dt > line && quiet(x.f.t)).sort((a, b) => b.f.dt - a.f.dt).slice(0, 40).sort((a, b) => a.i - b.i).map(({ f, i }) => {
    const hot = Object.keys(f.gpu).map(k => [k, f.gpu[k] - passMed[k]] as const).filter(x => x[1] > 0.5).sort((a, b) => b[1] - a[1]).slice(0, 4).map(([k, d]) => `${k} +${r2(d)}`);
    const near = dump.events.filter(e => e.kind !== 'sweep' && Math.abs(e.t - f.t) < 400).slice(0, 3).map(e => `${e.kind}: ${e.text.slice(0, 80)}`);
    return { frame: i, t: Math.round(f.t), dt: r2(f.dt), update: r2(f.cpu.update), render: r2(f.cpu.render), other: r2(f.cpu.other), gpu: r2(cost(f)), hot, near };
  });
  const rowsAsked = (begin?.rows as unknown[] | undefined)?.length ?? 0;
  const f0 = F[0], fN = F[n - 1];
  return { id, startedAt: dump.startedAt, commit: dump.commit ?? '', query: dump.query ?? '', machine: dump.machine, dpr: dump.dpr, userAgent: dump.userAgent, note: dump.note,
    preset: f0 ? `${f0.preset}${f0.adaptive ? ' +adapt' : ''}` : '', res: f0 ? [f0.res, fN.res] : [], frames: n, seconds: n ? r2((fN.t - f0.t) / 1000) : 0,
    dt: { mean: r2(mean(dts)), p50: r2(med), p95: r2(at(dts, 0.95)), p99: r2(at(dts, 0.99)), worst: r2(dts[n - 1] ?? 0) },
    over: { '20ms': dts.filter(v => v > 20).length, '25ms': dts.filter(v => v > 25).length, '33ms': dts.filter(v => v > 33.4).length, '50ms': dts.filter(v => v > 50).length, '100ms': dts.filter(v => v > 100).length },
    gpu: { mean: r2(mean(F.map(cost))), p95: r2(at(F.map(cost).sort((a, b) => a - b), 0.95)) }, cpu: { update: r2(mean(F.map(f => f.cpu.update))), render: r2(mean(F.map(f => f.cpu.render))), other: r2(mean(F.map(f => f.cpu.other))) },
    slowLine: r2(line), slow, passes, sweep: begin ? { rows: rowsAsked, rounds: Number(begin.rounds ?? 0), segments: seg.length, skipped: begin.skipped ?? [], end: end?.j ?? null } : null, events: dump.events.filter(e => e.kind !== 'sweep').length };
}

/** the read as a person reads it: a few lines for the panel */
export function takeWords(r: TakeRead): string[] {
  const out = [`${r.frames} frames in ${r.seconds} s`, `a frame: ${r.dt.p50} ms in the middle, ${r.dt.p95} at the 95th, ${r.dt.worst} at worst`, `slow frames: ${r.over['25ms']} over 25 ms, ${r.over['33ms']} over 33, ${r.over['50ms']} over 50`];
  if (r.sweep) out.push(`the sweep: ${r.sweep.rows} rows, ${r.sweep.segments} segments; its rows are priced when the take is read`);
  return out;
}
