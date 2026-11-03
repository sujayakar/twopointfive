// THE GAME'S OWN DIE, AT HOME IN THE RUST MODULE — the first system whose state lives in a game's block (src/sim/state.ts), and the proof of that mechanism.
// While the row RUST.dice is on, a game's die (math/dice.ts SimDice: the mulberry32 word and the count of draws) is the `dice` part of the game's block and
// simRandom() rolls it there (rust/sim/src/dice.rs, statement for statement). Off, it is the two fields it always was. A row is never a rule of play: the
// same draws either way.
import { sim } from './sim';
import { simTenant, simPart, P_DICE, type SimState } from './state';
import type { SimDice, DiceHome } from '../math/dice';

/** the die's part of one game's block (rust/sim/src/dice.rs Dice: the count as a double, then the word). Holds the game's SimState, never a view of it. */
class DiceInBlock implements DiceHome {
  constructor(readonly st: SimState) { }
  get s(): number { return this.st.w[(simPart(P_DICE) + 8) >>> 2] | 0; }
  set s(v: number) { this.st.w[(simPart(P_DICE) + 8) >>> 2] = v; }
  get n(): number { return this.st.d[simPart(P_DICE) >>> 3]; }
  set n(v: number) { this.st.d[simPart(P_DICE) >>> 3] = v; }
  roll(): number { return sim().dice_roll(this.st.bind()); }
}
interface HasDice { dice: SimDice | null }
// the die's two numbers move into a game's block as its tick opens with RUST.dice on, and out again with it off (state.ts settle)
simTenant({ row: 'dice', bit: 1,
  has(g: HasDice) { return g.dice !== null; },
  into(g: HasDice, st: SimState) { const d = g.dice!, s = d.s, n = d.n; d.home = new DiceInBlock(st); d.s = s; d.n = n; },
  outOf(g: HasDice) { const d = g.dice; if (!d) return; const s = d.s, n = d.n; d.home = null; d.s = s; d.n = n; } });
