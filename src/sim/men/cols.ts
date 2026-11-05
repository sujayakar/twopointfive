// WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as the game reads and writes them.

import type { MenStore } from './store';

/** the hash of the layout; the module holds the same one (rust/sim/src/men.rs SCHEMA), and store.ts holds the two to each other as a game starts */
export const MEN_SCHEMA = 0xa1f8eb71;
export const MEN_HEAD = 16;
export const MEN_F64 = ["since"] as const;
export const MEN_U32 = ["gen","owner","rank","reason"] as const;
/** the generation column, as an index into the words from the part's start (the store's own) */
export const MEN_GEN = 132;

/** what every man is: which store his columns are in and his row. Character and Guard extend these, so a column reads as a field */
export class ManCols {
  S!: MenStore; r = -1;
  /** a bit for each rule with a hand on him, the highest set has him (0: nobody): src/game/owner.ts */
  get owner(): number { const S = this.S; return S.u[S.b4 + 196 + this.r]; } set owner(v: number) { const S = this.S; S.u[S.b4 + 196 + this.r] = v; }
  /** the place of that top bit in the ranks (1 the weakest, 0 nobody): who is shown as having him, never who may take him */
  get rank(): number { const S = this.S; return S.u[S.b4 + 260 + this.r]; } set rank(v: number) { const S = this.S; S.u[S.b4 + 260 + this.r] = v; }
  /** which of the top rule's reasons it gave */
  get reason(): number { const S = this.S; return S.u[S.b4 + 324 + this.r]; } set reason(v: number) { const S = this.S; S.u[S.b4 + 324 + this.r] = v; }
  /** the game time the claim now on top came to the top (a lower claim surfacing counts from then) */
  get since(): number { const S = this.S; return S.d[S.b8 + 2 + this.r]; } set since(v: number) { const S = this.S; S.d[S.b8 + 2 + this.r] = v; }
}
/** a Guard record's own columns (a guard, a teammate, a civilian) */
export class GuardCols extends ManCols {
}
/** the player's own columns */
export class PlayerCols extends ManCols {
}
/** the animator's kept state */
export class AnimCols {
  S!: MenStore; r = -1;
}
