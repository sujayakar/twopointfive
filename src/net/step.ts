// ONE TICK OF A NETWORKED GAME, the same on the server and on every client: the seats' records, then what a host does between two ticks that the next tick
// reads — the lights aged (a flash lives by the tick), the world's boxes listed again (the cursor's ray walks the list; the pack for the picture waits for a draw) and an expired
// speech bubble let go (the HUD's one write to the game: journal.ts hudExpireBubbles). The order is the page's own between two ticks of one frame
// (host/loop.ts betweenTicks) and a re-run's (tools/qa/rerun.ts). dt is exactly 1 / NET.hz.
import type { Game } from '../game/game';
import type { PlayerCmd } from '../game/cmd';
import { hudExpireBubbles } from '../game/journal';   // (called inside netStep only: src/game is one import cycle)
import { NET } from './wire';

/** the two things of a host's engine a tick's end touches (the page's engine, the headless stand-in) */
export interface StepHost { lights: { update(dt: number, t: number): void }; world: { relist(): void }; }

export function netStep(g: Game, host: StepHost, cmds: readonly PlayerCmd[]): void {
  const dt = 1 / NET.hz;
  g.stepSeats(dt, cmds);
  host.lights.update(dt, g.time); host.world.relist(); hudExpireBubbles(g);   // (the list alone: a tick is not a draw, and a server never draws — scene/boxes.ts relist; a page's next draw packs what it is handed)
}
/** every seat of a networked game is refereed on every machine (wire.ts: the view is worked out again from the pose). Once, when the game is stood */
export function netSeats(g: Game): void { for (const s of g.seats) s.remote = true; }
