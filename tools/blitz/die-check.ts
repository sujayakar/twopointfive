// THE GAME'S STREAM SEEDED AFTER THE GAME IS STOOD, under each word (tools/qa/headless.ts `seedGame`; tools/blitz/PLAY.md "The die"). One game: two seconds of
// play, then the stream is named, then Sam draws and fires for two seconds by the host's input (his rounds' spread comes off the game's die), ten seconds in all.
// It prints one line: the rolls the game's die took after the seed, the word it stands at, and a hash of the digest of every half second after it.
//   bun run tools/blitz/die-check.ts            the helper: `seedGame(game, mulberry32(7), 7)`
//   bun run tools/blitz/die-check.ts --site     the site as tools wrote it: `Math.random = mulberry32(7)`
// What must hold (run by hand; tools/blitz/PLAY.md has the table):
//   with no word      the helper's line IS the site's line (the helper is the assignment and nothing else)
//   TPS_GAME=ts|rust  the helper's two lines are equal (both ticks roll the game's own die from the seed's start);
//                     the site's two lines are NOT (the TypeScript tick follows the tool's function, the Rust tick cannot)
import { standUp, seedGame, ROOT } from '../qa/headless.ts';
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const SITE = process.argv.includes('--site'), DT = 1 / 60;
Math.random = mulberry32(11);
const W: any = await standUp(), g = W.game; let drawn = 0; const rows: unknown[] = [];
for (let f = 0; f < 600; f++) {
  if (f === 120) { const die = mulberry32(7), counted = () => { drawn++; return die(); }; if (SITE) Math.random = counted; else seedGame(g, counted, 7); }
  const at = 180, on = f >= at + 60 && f < at + 180; if (f === at) W.input.pressed.add('KeyE'); if (on && (f - at) % 10 === 0) W.input.clicked |= 1; W.input.buttons = on ? W.input.buttons | 1 : W.input.buttons & ~1;
  W.step(DT); if (f >= 120 && (f + 1) % 30 === 0) rows.push(digestGame(g, true)); }
const own = g.dice ? g.dice.n : null;
console.log(`die-check ${SITE ? 'site' : 'helper'} · word ${process.env.TPS_GAME ?? 'none'} · the game's own die rolled ${own ?? 'no die'}${g.dice ? ` and stands at ${(g.dice.s >>> 0).toString(16)}` : ''} · the tool's function was drawn ${drawn} · digest ${Bun.hash(JSON.stringify(rows)).toString(36)}`);
