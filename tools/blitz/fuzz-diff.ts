// WHERE TWO PLAYS OF ONE SOAK SEED PART (tools/blitz/fuzz.sh; PLAY.md, "The fuzz"). The folder holds a line a tick from each side (`ts-run.txt`,
// `rust-run.txt`, written by the host under RH_TICKS: the tick, the die's count, a hash of the host's half as handed in and the orders given so far, a hash
// of each part of rust/sim's block but `poses`) and each side's own output (`ts.log`, `rust.log`: the soak's timeline, a line a stimulus).
//   bun run tools/blitz/fuzz-diff.ts <folder> [--json] [--but-panes]
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
// (a tick written twice is a tick a kept game stood again in: its last line is the game as that update left it)
const last = (L: string[]): string[] => { const out: string[] = []; for (const l of L) { if (!l) continue; const k = l.slice(0, l.indexOf(' ')); if (out.length && out[out.length - 1].startsWith(k + ' ')) out[out.length - 1] = l; else out.push(l); } return out; };
const T = last(read('ts-run.txt')), R = last(read('rust-run.txt')), TL = read('ts.log'), RL = read('rust.log');
if (!T.length || !R.length) { const why = `fuzz-diff: ${!T.length ? 'the TypeScript side' : 'the Rust side'} wrote no tick (${dir}); its log ends: ${(T.length ? RL : TL).filter(Boolean).slice(-3).join(' / ').slice(0, 500)}`; if (asJson) console.log(JSON.stringify({ dir, ran: false, why, kind: /panicked/.test(RL.join('\n')) ? 'a panic' : 'a throw in the host' })); else console.log(why); process.exit(3); }
// a line: `<k> <die> <half>|<gives> <part hashes…>`
// (--but-panes: the block's last part, the glass panes, is left out of the compare: the TypeScript's snapshot does not put a broken pane back whole, PLAY.md)
const butPanes = process.argv.includes('--but-panes');
const cut = (l: string) => { const w = l.split(' '); return { k: +w[0], die: w[1], half: w[2], game: w.slice(3, butPanes ? -1 : undefined).join(' ') }; };
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
// (the soak's own invariants are a third reading: a violation the Rust side reports and the reference does not, by its kind, is something the puppet
// shows the tool that the TypeScript game did not, whether or not the two games part)
const viol = (L: string[]) => { const M = new Map<string, string>(); for (const l of L) { const m = l.match(/^\s*seed \d+ f(\d+) [\d.]+s ([\w-]+) \(([^)]*)\): (.*)$/) ?? l.match(/!! VIOLATION ([\w-]+) \(([^)]*)\): (.*)$/); if (m) { const k = m.length === 5 ? m[2] : m[1]; if (!M.has(k)) M.set(k, l.trim().slice(0, 240)); } } return M; };
const vT = viol(TL), vR = viol(RL), onlyRust = [...vR].filter(([k]) => !vT.has(k)).map(([k, l]) => ({ kind: k, line: l }));
// WHERE THE TWO TIMELINES FIRST SAY ANOTHER THING (the soak's own lines: stimuli, what they found, alarm levels; its violations left out). The schedule
// reads the game it is given: a line that differs BEFORE the games part is the puppet showing the driver something the TypeScript game did not.
const lines = (L: string[]) => L.filter(l => /^\s*\[seed \d+\] /.test(l) && !/^!!/.test(l.trim())).map(l => l.trim());
const tlT = lines(TL), tlR = lines(RL); let tlAt = 0; while (tlAt < tlT.length && tlAt < tlR.length && tlT[tlAt] === tlR[tlAt]) tlAt++;
const said = tlAt < Math.max(tlT.length, tlR.length) ? { ts: (tlT[tlAt] ?? '(no more lines)').slice(0, 260), rust: (tlR[tlAt] ?? '(no more lines)').slice(0, 260), f: row(tlT[tlAt] ?? tlR[tlAt] ?? '')?.f ?? -1 } : null;
// … and where the DRIVER'S OWN ACTS first differ (a stimulus, a teleport, a key, a respite): from there the two sides are no longer driven alike
const acts = (L: string[]) => L.filter(l => /(STIM |teleport player|press |RESPITE|godMode|ai back on|interrupt scheduled)/.test(l));
const acT = acts(tlT), acR = acts(tlR); let acAt = 0; while (acAt < acT.length && acAt < acR.length && acT[acAt] === acR[acAt]) acAt++;
const acted = acAt < Math.max(acT.length, acR.length) ? { ts: (acT[acAt] ?? '(no more)').slice(0, 260), rust: (acR[acAt] ?? '(no more)').slice(0, 260), f: row(acT[acAt] ?? acR[acAt] ?? '')?.f ?? -1, n: acAt } : null;
const surface = RL.find(l => l.startsWith('the call surface:'))?.slice(0, 1600) ?? '';
const stimAt = [...tsNear].reverse().find(r => r.s.startsWith('STIM ') && r.f >= at - 1), lastStim = [...tsNear].reverse().find(r => r.s.startsWith('STIM '));
// (what the soak does that is no scheduled stimulus: Enter once down, Shift+Enter, a tour's end: `press …` in its timeline)
const pressAt = [...tsNear].reverse().find(r => /^press /.test(r.s) && r.f >= at - 2);
const name = (r?: { s: string }) => r ? r.s.replace(/^STIM /, '').split(':')[0].trim() : '';
const kind = !first ? 'none' : panic ? 'a panic'
  : excR && !excT ? 'a throw in the host'
  : pressAt ? `the driver's ${pressAt.s.replace(/\s*\(.*$/, '')}`
  : acted && acted.f >= 0 && acted.f <= at && /STIM /.test(acted.ts) ? `the stimulus ${acted.ts.replace(/^.*STIM /, '').split(':')[0].trim()} (the driver's acts differ from frame ${acted.f})`
  : stimAt ? `the stimulus ${name(stimAt)}` : lastStim && at - lastStim.f <= 60 ? `within a second of the stimulus ${name(lastStim)}` : 'a difference in play';
const out = { dir, ran: true, ticks: { ts: T.length, rust: R.length, compared: n }, parted: first, halfParted: half, parts: parts.map(j => j < 0 ? 'the die' : `part ${j}`), kind, stimAt: pressAt ? `f${pressAt.f} ${pressAt.s}`.slice(0, 200) : stimAt ? `f${stimAt.f} ${stimAt.s}`.slice(0, 200) : '', lastStim: lastStim ? `f${lastStim.f} ${lastStim.s}`.slice(0, 200) : '', excRust: excR, excTs: excT, panic, surface, said, acted, onlyRust, soak: { ts: TL.find(l => l.startsWith('=== soak:'))?.slice(0, 160) ?? '', rust: RL.find(l => l.startsWith('=== soak:'))?.slice(0, 160) ?? '' } };
if (asJson) { console.log(JSON.stringify(out)); process.exit(first ? 1 : 0); }
if (onlyRust.length) console.log(`fuzz-diff: the soak's own invariants, on the Rust side only: ${onlyRust.map(v => v.kind).join(', ')}\n  first: ${onlyRust[0].line}`);
if (!first) console.log(`fuzz-diff: EQUAL on all ${n} ticks (the die, every part of the block)${half ? `; the host's half differs from tick ${half}` : ', the host\'s half too'}`);
else {
  console.log(`fuzz-diff: ${game ? `THE GAME PARTED AT TICK ${game} (${out.parts.join(', ')})` : `one side stopped: the TypeScript played ${T.length} ticks, the Rust game ${R.length}`}; the host's half ${half ? `first differs at tick ${half}` : 'is equal up to there'}; kind: ${kind}`);
  console.log(`  the reference's last three seconds:`); for (const r of tsNear.slice(-14)) console.log(`    ${r.t.toFixed(2)}s f${r.f} ${r.s.slice(0, 220)}`);
  console.log(`  the Rust side's:`); for (const r of ruNear.slice(-14)) console.log(`    ${r.t.toFixed(2)}s f${r.f} ${r.s.slice(0, 220)}`);
  if (said) console.log(`  the two timelines first say another thing at frame ${said.f}:\n    the reference: ${said.ts}\n    the Rust side: ${said.rust}`);
  if (acted) console.log(`  the driver's own acts first differ at frame ${acted.f} (after ${acted.n} the same):\n    the reference: ${acted.ts}\n    the Rust side: ${acted.rust}`);
  if (excR) console.log(`  the Rust side's first exception up to there: ${excR}`); if (excT) console.log(`  the reference's: ${excT}`);
  if (surface) console.log(`  ${surface}`); }
process.exit(first ? 1 : 0);
