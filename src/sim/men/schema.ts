// A MAN'S STATE, FIELD BY FIELD — THE ONE SCHEMA. Every man of a game (a guard, a teammate, the player, a civilian) has one row of the `men` part of the game's block
// (rust/sim/src/state.rs, src/sim/state.ts), and this table says what the columns of that row are. `bun run tools/sim/men.ts` writes the two sides from it:
//   src/sim/men/cols.ts     the TypeScript game's accessors (a man reads and writes himself through them, with no copy)
//   rust/sim/src/men.rs     the Rust struct of columns (a rule there reads the same bytes, with no copy)
// Both are written, never edited. tools/qa/men-check.ts is red when they are not this table, and every game holds the two sides' hashes to each other as it starts.
//
// A ROW OF THE TABLE:
//   name    what the game calls it. A `vec3` makes three columns, `<name>X`, `<name>Y`, `<name>Z`, and an accessor for each; `vec3` and `vec3?` also read as a whole
//           `<name>`, a fresh array each read (never a view: nothing keeps a view of the block).
//   on      whose accessor it is: 'man' = every man (the player and every Guard record, through his Character too); 'guard' or 'player' = that record's; 'anim' = the
//           animator's kept state (CharacterAnimator's clocks, weights and phases, a group moved by the part that moves the animator).
//   group   'hot' read every tick by the fights (position, facing, velocity, stance, health, alert state, goal) · 'cold' the rest · 'animator' the animator's. The
//           columns are laid out hot before cold, so a walk over the men reads the fewest lines.
//   type    'f64' a double · 'u32' / 'i32' a word · 'bool' a bit of a word of truths · 'vec3' three doubles · 'vec3?' three doubles and a bit that says there are any ·
//           an array of strings, a word holding the place in that list.
//   digest  game/digest.ts reads it, so a run's digest holds it.
//   why     what it is, when its name does not say.
// A NUMBER A DIGEST READS IS A DOUBLE: the game's rules are worked out in doubles, a single would round them (rust/sim/src/lib.rs says the same of the crate).
//
// STAYS WITH THE OBJECT: `KEPT` names a field that belongs to a man and is NOT a column, and why: 'dressing' only the picture, the sound or the HUD reads it · 'object' it
// holds what a column cannot (a list, a closure, a record that has its own family to move with).
export type ColType = 'f64' | 'u32' | 'i32' | 'bool' | 'vec3' | 'vec3?' | readonly string[];
export type ColOn = 'man' | 'guard' | 'player' | 'anim';
export type ColGroup = 'hot' | 'cold' | 'animator';
export interface MenField { readonly name: string; readonly on: ColOn; readonly group: ColGroup; readonly type: ColType; readonly digest: boolean; readonly why?: string }
export interface MenKept { readonly name: string; readonly on: ColOn; readonly stays: 'dressing' | 'object'; readonly why?: string }

/** rows of the part. A man's id is a byte (the lights' owner byte: render/lights.ts), so a game can never stand more men at once than this */
export const MEN_ROWS = 256;

export const MEN_FIELDS: readonly MenField[] = [
  // WHO HAS THIS MAN: the one record the "does X have this man?" questions become. Written and read by the game's owner rules (they are not moved here: they arrive empty).
  { name: 'owner', on: 'man', group: 'cold', type: 'u32', digest: false, why: 'which rule has him (0: nobody)' },
  { name: 'rank', on: 'man', group: 'cold', type: 'u32', digest: false, why: 'how hard that rule holds him against another' },
  { name: 'reason', on: 'man', group: 'cold', type: 'u32', digest: false, why: 'why it has him' },
  { name: 'since', on: 'man', group: 'cold', type: 'f64', digest: false, why: 'the game time it took him' },
];

export const MEN_KEPT: readonly MenKept[] = [];
