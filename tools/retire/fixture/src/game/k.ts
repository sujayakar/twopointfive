// The small tree tools/retire/retire-check.ts runs the retire tool on: two knob tables laid out as the game's are.
/** K: switches. `on`, `extra`, `viaList` and `panelOnly` are on in the page, `old` is off there; the rest are rows the tool must refuse. */
export const K = {
  /** a rule that is on in the page */
  on: false,
  /** another, read from the address by its value */
  extra: false,
  /** the way it was: off in the page */
  old: false,
  /** written by the journal through a list of keys */
  viaList: false,
  /** has a panel row and a kept row, both by name */
  panelOnly: false,
  /** a number that is a row like any other */
  num: 3,
  /** a number below zero */
  neg: -1,
  /** the game writes it as it runs: state, not a switch */
  stay: false,
  /** the game reads it through a list of keys */
  listRead: true,
  /** another table holds another value for it */
  preset: false,
};
/** N: a number the frozen stand holds at an old value */
export const N = { reach: 3, far: 9 };
