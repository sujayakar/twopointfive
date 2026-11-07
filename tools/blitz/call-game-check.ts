// A CALL'S GAME IS NAMED BY THE CALL. A tool stands two games in one process. A call on a row of the first goes to the first though the second stood last; a call that names no game runs its TypeScript body whatever stands, and crosses to neither; a function with a body on a game that has not started runs on the puppet, which the start takes across; one with no body starts that game and crosses.
//   TPS_GAME=rust bun run tools/blitz/call-game-check.ts     exit 0 every line passes · 1 a line fails
// CALL_GAME_BREAK=1 asks the row's game of the second game's man, and the check must fail.
const { standUp } = await import('../qa/headless.ts');
const { gameOfRow } = await import('./export.ts');
const { startFromQuery } = await import('../../src/game/scenarioRun.ts');
const { difficultyNow, setDifficulty } = await import('../../src/game/difficulty.ts');
const { kitCost } = await import('../../src/game/kit.ts');
const { samAim } = await import('../../src/game/aim.ts');
let red = 0; const line = (ok: boolean, what: string) => { console.log(`${ok ? 'PASS' : 'FAIL'}: ${what}`); if (!ok) red++; };
const crossed = () => (globalThis as any).__tpsTally?.crossed ?? 0;
const stand = async () => { const W: any = await standUp({} as any); const r = startFromQuery(W.game, '?roomclear&squad=2'); if (r.why) throw new Error(r.why); return W; };
const A = await stand(); A.step(1 / 60); const a = A.game, ma = a.guards[0];
const B = await stand(); B.step(1 / 60); const b = B.game, mb = b.guards[0];
line(gameOfRow(ma) === a && gameOfRow(mb) === b, 'each man is a row of his own game');
const { resetChase, spawnUnit } = await import('../../src/game/guards.ts');
// (one rule that writes a man's own field, called on the first game's man with the second stood last, and read back on each game)
const hostA = a.rustSurface, hostB = b.rustSurface; let toA = 0, toB = 0; for (const [H, up] of [[hostA, () => toA++], [hostB, () => toB++]] as const) { const was = H.surfaceCall; H.surfaceCall = function (...x: any[]) { up(); return was.apply(this, x); }; }
resetChase(process.env.CALL_GAME_BREAK ? mb : ma);
line(toA === 1 && toB === 0, `a call on the first game's man crosses to the first, with the second stood last (${toA} to the first, ${toB} to the second)`);
toA = toB = 0; const c0 = crossed(); difficultyNow(); kitCost(0); line(toA === 0 && toB === 0 && crossed() === c0, 'a call that names no game crosses to neither: its body runs, as on the page');
const was = difficultyNow(); setDifficulty(was === 'hard' ? 'normal' : 'hard'); const now = difficultyNow(); line(now !== was, `a setting a call with no game writes is read back the same whatever stands (${was} to ${now})`);
B.step(1 / 60); A.step(1 / 60); line(difficultyNow() === now, 'and the next ticks of both games leave it as it was written'); setDifficulty(was);
const C: any = await standUp({} as any); const c = C.game, H = c.rustSurface, starts0 = H.starts; { const r = startFromQuery(c, '?roomclear&squad=2'); if (r.why) throw new Error(r.why); } line(H.starts === starts0, "a mission's staging starts nothing: what it calls has a body, and runs on the puppet");
let toC = 0; { const w = H.surfaceCall; H.surfaceCall = function (...x: any[]) { toC++; return w.apply(this, x); }; } spawnUnit(c, { team: 'enemy', pos: [...c.player.char.pos], budget: 16 }); line(H.starts === starts0 && toC === 0, 'a call with a body on a game that has not started runs on the puppet, and starts nothing');
samAim(c); line(H.starts === starts0 + 1 && toC === 1, `a call with no body on a game that stands and has not started starts it, and crosses (${H.starts - starts0} starts, ${toC} crossings)`);
console.log(`=== call-game-check: ${red ? `${red} lines fail` : 'all pass'} ===`); process.exit(red ? 1 : 0);
