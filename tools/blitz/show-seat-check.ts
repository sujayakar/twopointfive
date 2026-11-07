// WHICH SEAT A MACHINE SHOWS CHANGES NOTHING OF THE GAME. A game of two seats is played three times by the same scripted hands (seats-run.ts): showing the first seat, showing the second from before its
// first tick, and changing seat twice in play. Every part of the whole state words must be the same on all three, tick for tick, with what a host hands in left out as ever.
//   TPS_GAME=rust bun run tools/blitz/show-seat-check.ts [address] [ticks]     exit 0 the same · 1 a part differs. A fourth play, with the dressing rows up, is only said, not held.
// SHOW_SEAT_BREAK=1 gives the second play one more tick before it reads, and the check must fail.
if (process.env.SHOW_SEAT_CHILD) {
  const { standTwo } = await import('../qa/seats-run.ts');
  const q = process.argv[2], N = +process.argv[3], how = process.env.SHOW_SEAT_CHILD, P: any = await standTwo(q, 9, null, false, how === 'dressed'), g = P.g;
  if (how !== 'first' && how !== 'dressed') g.localSeat = g.seats[1];
  const out: string[] = []; if (process.env.SHOW_SEAT_EXTRA) P.tick(P.sample(0));
  for (let t = 1; t <= N; t++) { if (how === 'turns' && t === 40) g.localSeat = g.seats[0]; if (how === 'turns' && t === 80) g.localSeat = g.seats[1]; P.tick(P.sample(t - 1)); if (t % 30 === 0) out.push(`${t} ${[...g.rustHost.wholeWords()].join(',')}`); }
  console.log(`NAMES ${g.rustHost.wholeNames().join(',')}`); for (const l of out) console.log(`WORDS ${l}`); process.exit(0);
}
const q = process.argv[2] ?? 'roomclear&squad4', N = process.argv[3] ?? '180';
const play = async (how: string, extra = false): Promise<{ names: string[]; words: Map<string, string[]> }> => { const p = Bun.spawn(['bun', 'run', import.meta.path, q, N], { env: { ...process.env, SHOW_SEAT_CHILD: how, ...(extra ? { SHOW_SEAT_EXTRA: '1' } : {}) }, stdout: 'pipe', stderr: 'inherit' }); const text = await new Response(p.stdout).text(); await p.exited; const names = (text.match(/^NAMES (.*)$/m)?.[1] ?? '').split(','); const words = new Map<string, string[]>(); for (const m of text.matchAll(/^WORDS (\d+) (.*)$/gm)) words.set(m[1], m[2].split(',')); if (!words.size) { console.log(`show-seat-check: the play "${how}" said nothing:\n${text.slice(-600)}`); process.exit(3); } return { names, words }; };
const base = await play('first'), others = [['the second seat', await play('second', !!process.env.SHOW_SEAT_BREAK)], ['two changes of seat', await play('turns')]] as const, dressed = await play('dressed');
let bad = 0;
for (const [what, o] of others) { const parts = new Map<string, number>(); for (const [t, w] of base.words) { const x = o.words.get(t)!; w.forEach((v, i) => { if (v !== x?.[i]) parts.set(base.names[i], (parts.get(base.names[i]) ?? 0) + 1); }); } console.log(`${parts.size ? 'FAIL' : 'PASS'}: showing ${what}: ${base.words.size} word ticks of ${base.names.length} parts${parts.size ? `, ${parts.size} parts differ: ${[...parts].slice(0, 40).map(([k, n]) => `${k} ${n}`).join(' · ')}` : ', all the same'}`); if (parts.size) bad++; }
{ const parts = new Map<string, number>(); for (const [t, w] of base.words) { const x = dressed.words.get(t)!; w.forEach((v, i) => { if (v !== x?.[i]) parts.set(base.names[i], (parts.get(base.names[i]) ?? 0) + 1); }); } console.log(`SAID: with the dressing rows up, as a player's page has them, against down, as a server has them: ${parts.size} parts differ${parts.size ? `: ${[...parts].map(([k, n]) => `${k} ${n}`).join(' · ')}` : ''}`); }
console.log(`=== show-seat-check: ${bad ? 'FAILED' : 'the same'} ===`); process.exit(bad ? 1 : 0);
