// THE VERDICT OF A MISSION PLAYED BY THE SCRIPTED PLAYER: the word the end card would carry, and the tick it was first said.
//   bun run tools/blitz/verdict-of.ts <mission>:<seed> [<mission>:<seed> …]
// The tally (tools/qa/playthrough.ts) records how the SCRIPTED PLAYER's own run ended (`end`: a room clear "taken" or "wiped", a rescue "rescued"), which
// is the room clear's or the rescue's word and not the verdict's: a defuse mission whose charges went off and whose rooms were then taken reads "taken"
// there, and its card reads BLOWN. This plays the same scripted player and prints the verdict itself, as the game's `verdict.result` first answers:
//   VERDICT defuse-four:1 · lost BLOWN · the try's clock 1:15 · the card's line: BLOWN 1:15 · 4 of 5 up · 6 of 8 defenders down
// and the scripted player's own end beside it. A line a mission; "no verdict" where the run ended and none was said. Nothing here is part of any digest.
// Exit 0 always but for a wrong word (64): it reads, it does not judge.
export {};
const A = process.argv.slice(2).filter(a => !a.startsWith('--'));
if (!A.length || A.some(a => !/^[a-z0-9-]+:\d+$/.test(a))) { console.error('verdict-of.ts <mission>:<seed> [<mission>:<seed> …]'); process.exit(64); }
const { verdictLine, clock } = await import('../../src/game/verdict.ts');
const { play } = await import('../qa/playthrough.ts');
const { RustHost } = await import('./rust-host.ts');
// (the verdict is the Rust game's own, latched as the run first ends and applied onto the puppet: read as the run is over, on the last game stood)
let last: any = null; const attach = RustHost.attach; RustHost.attach = function (g: any, o: any) { last = g; return attach.call(RustHost, g, o); };
const log = console.log;
let said = '';
for (const a of A) { const [scn, seed] = a.split(':'); said = '';
  console.log = () => { /* (the scripted player's own lines) */ }; let end = '';
  try { const r = await play(scn, +seed, 300, false, {}); end = `${r.end}${r.why ? ` (${r.why})` : ''} at ${r.t} s`; } catch (e) { end = `the play stopped: ${String((e as any)?.message ?? e).slice(0, 160)}`; } finally { console.log = log; }
  const R = last?.verdict?.result; said = R ? `${R.won ? 'won' : 'lost'} ${R.word}${R.why ? ` (${R.why})` : ''} · the try's clock ${clock(R.secs)} · the card's line: ${verdictLine(R)}` : '';
  log(`VERDICT ${a} · ${said || 'no verdict was said by the run\'s end'} · the scripted player's own end: ${end}`); }
process.exit(0);
