// THE GAME'S OWN SMOKE HANDED TO THE PICTURE'S SOLVER, as it lies (rust/game_draw/src/smoke.rs): ten times a second, and only where a step was taken.
import type { SmokeSystem } from '../smoke/smoke';
import type { Drawn } from './drawn';
import { Section, SmokeBrick, SmokeHead, Word } from './sections';

export function smokeFrom(smoke: SmokeSystem, D: Drawn): void {
  const u = D.u, h = D.at[Section.smoke];
  if (!D.n[Section.smoke]) return;   // (no grid, or one just put back: what is held goes on)
  smoke.bent(u[h + SmokeHead.made], u[h + SmokeHead.wide], u[h + SmokeHead.deep], D.n[Section.smokeBricks], (buffer, plan, bricks) => {
    D.send(smoke.queue, buffer, Section.smokeAt, Word.row, 0, D.n[Section.smokeAt], plan);
    D.send(smoke.queue, buffer, Section.smokeBricks, SmokeBrick.row, 0, D.n[Section.smokeBricks], bricks);
  });
}
