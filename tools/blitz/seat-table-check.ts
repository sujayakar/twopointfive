// A SEAT'S TABLE CROSSES THE BRIDGE BOTH WAYS. The TypeScript keeps it by the seat's view (the Game itself in a game of one seat), the Rust game by the seat's man. A row written before the
// first tick must reach the Rust game, and what the Rust game then holds must be found on the puppet under the key the TypeScript asks by.
//   TPS_GAME=rust bun run tools/blitz/seat-table-check.ts [address]     exit 0 every line passes · 1 a line fails
// SEAT_TABLE_BREAK=1 asks the table by the player, which no reader does, and the check must fail.
const { standUp } = await import('../qa/headless.ts');
const { setSilent, squadSilent } = await import('../../src/game/silent.ts');
const { startFromQuery } = await import('../../src/game/scenarioRun.ts');
const q = process.argv[2] ?? 'roomclear&squad4';
const stand = async (): Promise<any> => { const W: any = await standUp({ seed: 9 }), said = startFromQuery(W.game, q); if (said.why) { console.log(`seat-table-check: ${q} refused: ${said.why}`); process.exit(3); } return W; };
const ticks = (W: any) => { for (let t = 0; t < 5; t++) W.step(1 / 60); };
let bad = 0; const line = (ok: boolean, say: string) => { console.log(`${ok ? 'PASS' : 'FAIL'}: ${say}`); if (!ok) bad++; };
{ const W = await stand(), g = W.game, who = process.env.SEAT_TABLE_BREAK ? g.player : g;   // (the table has no row until the Rust game writes it)
  ticks(W); setSilent(g, true); ticks(W); line(squadSilent(who), 'set silent in play with no row before it, the puppet finds the row under the Game');
  setSilent(g, false); ticks(W); line(!squadSilent(g), 'set loud in play, the puppet reads loud'); }
{ const W = await stand(), g = W.game; setSilent(g, true); ticks(W); line(squadSilent(g), 'set silent before the first tick, the start carries the row and the squad is silent after it'); }
console.log(`=== seat-table-check: ${bad ? `${bad} failed` : 'all pass'} ===`); process.exit(bad ? 1 : 0);
