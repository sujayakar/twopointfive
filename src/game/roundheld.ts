// IS THIS ROUND HELD? Which rule keeps a teammate's round from a man is a question about a shot at a target, not about who has the shooter: one read, here, with the rules' own
// answers registered by their files (their state stays theirs). The room fight and the cover's fire ask the same three, in the same order; a rule's own stat counts as it answers.
// A leaf of src/game's import cycle (type-only imports), so a top-level `roundHoldRule` line in a rule's file is safe wherever the cycle starts.
import type { Game, Guard } from './game';
import type { Character } from './character';
import type { Vec3 } from '../math/vec';

export type RoundRule = 'silent' | 'pair' | 'spotted';
type Hold = (g: Game, gd: Guard, h: Guard) => boolean;
const HOLDS: Partial<Record<RoundRule, Hold>> = {};
export function roundHoldRule(name: RoundRule, hold: Hold): void { HOLDS[name] = hold; }
/** that rule alone: does it hold this man's round at `h`? */
export function roundHeldBy(name: RoundRule, g: Game, gd: Guard, h: Guard): boolean {
  const hold = HOLDS[name]; if (!hold) throw new Error(`roundheld.ts: the rule '${name}' has not said how it holds (its file is not in the build?)`);
  return hold(g, gd, h);
}
/** THE ROOM FIGHT'S QUESTION: the silent squad's hold, the pair's 'watch him', the hands going onto him */
export function roundHeld(g: Game, gd: Guard, h: Guard): boolean { return roundHeldBy('silent', g, gd, h) || roundHeldBy('pair', g, gd, h) || roundHeldBy('spotted', g, gd, h); }

type GlassHold = (g: Game, gd: Guard, h: Guard, c: Character, to: Vec3) => boolean;
let glass: GlassHold | null = null;
export function roundGlassRule(hold: GlassHold): void { glass = hold; }
/** …and his round at `h` that would have to go through standing glass to reach `to` */
export function roundHeldGlass(g: Game, gd: Guard, h: Guard, c: Character, to: Vec3): boolean {
  if (!glass) throw new Error('roundheld.ts: the rule of the glass has not said how it holds (its file is not in the build?)');
  return glass(g, gd, h, c, to);
}

export type TargetRule = 'silent' | 'holdfire';
const TARGET: Partial<Record<TargetRule, (g: Game, gd: Guard) => boolean>> = {};
export function targetHoldRule(name: TargetRule, hold: (g: Game, gd: Guard) => boolean): void { TARGET[name] = hold; }
/** THE MAN THE ROOM FIGHT HAS HIM ON IS ONE THIS RULE WILL NOT LET HIM SHOOT, asked afresh and counting nothing */
export function targetHeld(name: TargetRule, g: Game, gd: Guard): boolean {
  const hold = TARGET[name]; if (!hold) throw new Error(`roundheld.ts: the rule '${name}' has not said how it holds (its file is not in the build?)`);
  return hold(g, gd);
}
