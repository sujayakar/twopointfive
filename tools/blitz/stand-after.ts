// A GAME STOOD AFTER A GAME WHOSE MEN THE RUST GAME MADE (tools/blitz/PLAY.md, "A tool that stands many games"): a squad mission is the process's first game (so the cast's
// rig is in its book), it is begun again (the new men are the Rust game's, and have no rig on the TypeScript side), and a plain game is stood and played. It must be the game
// a process of its own plays (asked of a child, `--plain`), at every second. Between the plain game's stand and its first step a call names the shared cast, and so the
// older game. And a refused stand-up leaves the older game as it was: its calls cross again.
//   tools/blitz/rust.sh rust bun run tools/blitz/stand-after.ts [--secs 2]
//   exit 0: the same digest at every second · 1: it threw, or it parted. RH_OFF=standing in front must be red.
const { standUp, ROOT } = await import('../qa/headless.ts');
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { castGoggles } = await import(`${ROOT}/src/anim/gltf.ts`);
const A = process.argv.slice(2), at = A.indexOf('--secs'), SECS = at >= 0 ? +A[at + 1] : 2, DT = 1 / 60;
const plain = async (): Promise<string[]> => { Math.random = mulberry32(0xc0de); const W: any = await standUp({ seed: 3 }), cast = W.game.ch.cast; castGoggles(cast, cast.variants[0].mask, true); const words = [hashDigest(digestGame(W.game))];
  for (let s = 0; s < SECS; s++) { W.steps(60, DT); words.push(hashDigest(digestGame(W.game))); } return words; };
if (A.includes('--plain')) { console.log(`WORDS ${(await plain()).join(' ')}`); process.exit(0); }
const child = Bun.spawnSync(['bun', 'run', import.meta.path, '--plain', '--secs', String(SECS)], { env: process.env }), first = (new TextDecoder().decode(child.stdout).match(/^WORDS (.*)$/m)?.[1] ?? '').split(' ');
if (first.length !== SECS + 1) throw new Error(`stand-after: the plain game's process said no words: ${new TextDecoder().decode(child.stderr).slice(-300)}`);
Math.random = mulberry32(0xc0de); const S: any = await standUp({ seed: 9, address: 'roomclear&squad4' }), why = S.start().why; if (why) throw new Error(`stand-after: the squad mission refused: ${why}`);
S.steps(60, DT); S.game.roomClear.restage(); S.steps(60, DT);
{ const T = (globalThis as any).__tpsTally, n0 = T.crossed; let refused = false; try { await standUp({ address: 'roomclear', stood: {} } as any); } catch { refused = true; } if (!refused) throw new Error('stand-after: the stand-up was to be refused');
  S.game.roomClear.restage(); if (T.crossed === n0) { console.log('=== stand-after: RED: after a refused stand-up the older game\'s call ran its TypeScript ==='); process.exit(1); } S.steps(60, DT); }
let again: string[]; try { again = await plain(); } catch (e) { console.log(`=== stand-after: RED: the game stood after it threw: ${String((e as any)?.message ?? e).slice(0, 200)} ===`); process.exit(1); }
const part = again.findIndex((w, i) => w !== first[i]);
console.log(part < 0 ? `=== stand-after: the game a process of its own plays, stood after a mission begun again, ${SECS} seconds: ${first[SECS]} ===` : `=== stand-after: RED: it parts at second ${part}: ${first[part]} then ${again[part]} ===`);
process.exit(part < 0 ? 0 : 1);
