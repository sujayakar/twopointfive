// A SEAT'S TABLE CROSSES THE BRIDGE BOTH WAYS. The TypeScript keeps it by the seat's view (the Game itself in a game of one seat), the Rust game by the seat's man. A row written before the
// first tick must reach the Rust game, and what the Rust game then holds must be found on the puppet under the key the TypeScript asks by.
//   TPS_GAME=rust bun run tools/blitz/seat-table-check.ts [address]     exit 0 every line passes · 1 a line fails
// SEAT_TABLE_BREAK=1 asks the table by the player, which no reader does, and the check must fail.
const { standUp } = await import('../qa/headless.ts');
const { setSilent, squadSilent } = await import('../../src/game/silent.ts');
const q = process.argv[2] ?? 'roomclear&squad4', W: any = await standUp({ search: `?${q}` } as any), g = W.game, who = process.env.SEAT_TABLE_BREAK ? g.player : g;
let bad = 0; const line = (ok: boolean, say: string) => { console.log(`${ok ? 'PASS' : 'FAIL'}: ${say}`); if (!ok) bad++; };
setSilent(g, true); for (let t = 0; t < 5; t++) W.step(1 / 60);
line(squadSilent(who), 'a squad set silent before the first tick is silent after it: the start carried the row and the apply brought it back');
setSilent(g, false); for (let t = 0; t < 5; t++) W.step(1 / 60); line(!squadSilent(g), 'set loud in play, the puppet reads loud');
setSilent(g, true); for (let t = 0; t < 5; t++) W.step(1 / 60); line(squadSilent(who), 'set silent in play, the puppet reads silent');
console.log(`=== seat-table-check: ${bad ? `${bad} failed` : 'all pass'} ===`); process.exit(bad ? 1 : 0);
