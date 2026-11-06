// WHERE TWO PLAYS OF ONE SOAK SEED PART (tools/blitz/fuzz.sh; PLAY.md, "The fuzz"). The folder holds a line a tick from each side (`ts-run.txt`,
// `rust-run.txt`, written by the host under RH_TICKS: the tick, the die's count, a hash of the host's half as handed in and the orders given so far, a hash
// of each part of rust/sim's block but `poses`) and each side's own output (`ts.log`, `rust.log`: the soak's timeline, a line a stimulus).
//   bun run tools/blitz/fuzz-diff.ts <folder> [--json]
// It names the first tick whose die or block differs (THE GAME PARTED) and the first tick whose host's half differs (the puppet showed the player
// something else: usually earlier), the stimuli of the last three seconds before it on each side, the first exception either side's log holds up to
// there, and the call surface's exit line of the Rust side. Then a kind, READ OFF THOSE AND NOT PROVED:
//   a panic · a throw in the host (an exception on the Rust side at or before the parting that the reference has not) · the stimulus that fired on the
//   parting tick or the one before it (named: what it calls or writes is what did not cross) · a difference in play (no stimulus within a second).
// Exit 0: equal on every tick and both sides played the same number of ticks · 1: they part (or one side stopped early) · 3: a side did not run.
import { readFileSync, existsSync } from 'node:fs';
const dir = process.argv[2], asJson = process.argv.includes('--json');
if (!dir) { console.error('fuzz-diff.ts <folder> [--json]'); process.exit(64); }
const read = (f: string): string[] => existsSync(`${dir}/${f}`) ? readFileSync(`${dir}/${f}`, 'utf8').split('\n') : [];
const T = read('ts-run.txt').filter(Boolean), R = read('rust-run.txt').filter(Boolean), TL = read('ts.log'), RL = read('rust.log');
if (!T.length || !R.length) { const why = `fuzz-diff: ${!T.length ? 'the TypeScript side' : 'the Rust side'} wrote no tick (${dir}); its log ends: ${(T.length ? RL : TL).filter(Boolean).slice(-3).join(' / ').slice(0, 500)}`; if (asJson) console.log(JSON.stringify({ dir, ran: false, why, kind: /panicked/.test(RL.join('\n')) ? 'a panic' : 'a throw in the host' })); else console.log(why); process.exit(3); }
// a line: `<k> <die> <half>|<gives> <part hashes…>`
const cut = (l: string) => { const w = l.split(' '); return { k: +w[0], die: w[1], half: w[2], game: w.slice(3).join(' ') }; };
const n = Math.min(T.length, R.length); let game = 0, half = 0, parts: number[] = [];
for (let i = 0; i < n && !game; i++) { const a = cut(T[i]), b = cut(R[i]); if (!half && a.half !== b.half) half = a.k; if (a.die !== b.die || a.game !== b.game) { game = a.k; const x = a.game.split(' '), y = b.game.split(' '); for (let j = 0; j < x.length; j++) if (x[j] !== y[j]) parts.push(j); if (a.die !== b.die) parts.unshift(-1); } }
if (!half) for (let i = 0; i < n; i++) if (cut(T[i]).half !== cut(R[i]).half) { half = cut(T[i]).k; break; }
const first = game || (T.length !== R.length ? n + 1 : 0);
// the soak's timeline: `  [seed N] 12.34s f1234 STIM name: detail`; a tick k is the frame k - 1 while every frame steps once
const row = (l: string) => { const m = l.match(/^\s*\[seed \d+\] ([\d.]+)s f(\d+) (.*)$/); return m ? { t: +m[1], f: +m[2], s: m[3] } : null; };
const upTo = (L: string[], f: number, back: number) => L.map(row).filter((r): r is { t: number; f: number; s: string } => !!r && r.f <= f && r.f >= f - back);
const at = (first || n) - 1, tsNear = upTo(TL, at, 180), ruNear = upTo(RL, at, 180);
const exc = (L: string[], f: number) => { for (const l of L) { const r = row(l); if (r && r.f > f + 1) break; if (/VIOLATION exception|panicked|error: |Error: /.test(l)) return l.trim().slice(0, 400); } return ''; };
const excR = exc(RL, at), excT = exc(TL, at), panic = RL.find(l => /panicked/.test(l))?.trim().slice(0, 300) ?? '';
const surface = RL.find(l => l.startsWith('the call surface:'))?.slice(0, 1600) ?? '';
const stimAt = [...tsNear].reverse().find(r => r.s.startsWith('STIM ') && r.f >= at - 1), lastStim = [...tsNear].reverse().find(r => r.s.startsWith('STIM '));
const name = (r?: { s: string }) => r ? r.s.replace(/^STIM /, '').split(':')[0].trim() : '';
const kind = !first ? 'none' : panic ? 'a panic'
  : excR && !excT ? 'a throw in the host'
  : stimAt ? `the stimulus ${name(stimAt)}` : lastStim && at - lastStim.f <= 60 ? `within a second of the stimulus ${name(lastStim)}` : 'a difference in play';
const out = { dir, ran: true, ticks: { ts: T.length, rust: R.length, compared: n }, parted: first, halfParted: half, parts: parts.map(j => j < 0 ? 'the die' : `part ${j}`), kind, stimAt: stimAt ? `${stimAt.f} ${stimAt.s}`.slice(0, 200) : '', lastStim: lastStim ? `f${lastStim.f} ${lastStim.s}`.slice(0, 200) : '', excRust: excR, excTs: excT, panic, surface };
if (asJson) { console.log(JSON.stringify(out)); process.exit(first ? 1 : 0); }
if (!first) console.log(`fuzz-diff: EQUAL on all ${n} ticks (the die, every part of the block)${half ? `; the host's half differs from tick ${half}` : ', the host\'s half too'}`);
else {
  console.log(`fuzz-diff: ${game ? `THE GAME PARTED AT TICK ${game} (${out.parts.join(', ')})` : `one side stopped: the TypeScript played ${T.length} ticks, the Rust game ${R.length}`}; the host's half ${half ? `first differs at tick ${half}` : 'is equal up to there'}; kind: ${kind}`);
  console.log(`  the reference's last three seconds:`); for (const r of tsNear.slice(-14)) console.log(`    ${r.t.toFixed(2)}s f${r.f} ${r.s.slice(0, 220)}`);
  console.log(`  the Rust side's:`); for (const r of ruNear.slice(-14)) console.log(`    ${r.t.toFixed(2)}s f${r.f} ${r.s.slice(0, 220)}`);
  if (excR) console.log(`  the Rust side's first exception up to there: ${excR}`); if (excT) console.log(`  the reference's: ${excT}`);
  if (surface) console.log(`  ${surface}`); }
process.exit(first ? 1 : 0);
