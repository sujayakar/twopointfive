// A REPLAY FILE WITH NO PAGE: the game stood up headless, walked a few seconds, and the buffer's own file (what F7 saves) written out. For a page check that
// needs a replay to open and has no minutes to play one on the software adapter (tools/qa/replaysend-page.cjs).
//   bun run tools/qa/replay-make.ts <out.json> [frames=180] [session]      (session = the page visit the file says it is a cut of; none by default)
// exit 0 = written · 2 = no path given
import { writeFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const out = process.argv[2], frames = Math.max(30, Number(process.argv[3]) || 180), session = process.argv[4] || '';
if (!out) { console.error('usage: bun run tools/qa/replay-make.ts <out.json> [frames] [session]'); process.exit(2); }
const { ReplayBuffer } = await import(`${ROOT}/src/game/replaybuf.ts`);
const { levelFingerprint } = await import(`${ROOT}/src/game/savestate.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
Math.random = mulberry32(0xbeef);
const W: any = await standUp({ seed: 21 }); W.game.godMode = true;
const buf = new ReplayBuffer(); buf.tape.born(W.game);
W.game.replayTap = (dt: number, cmd?: unknown) => { if (cmd) buf.top(W.game, dt, cmd); else buf.tick(W.game, dt, W.cam); };
for (let f = 0; f < frames; f++) { const I = W.input, s = f / 60; I.keys.clear(); if (s > 0.5) I.keys.add('KeyW'); I.mouseX = 800 + 300 * Math.sin(s * 0.7); I.mouseY = 420; W.step(1 / 60); }
const F = buf.file(W.game, 'check', { url: '', fp: levelFingerprint(W.game), ...(session ? { session } : {}) });
writeFileSync(out, JSON.stringify(F));
console.log(`${out}: ${F.samples.length} samples, ${F.seconds} s`);
process.exit(0);
