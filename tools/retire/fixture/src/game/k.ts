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
  /** read on a copy of the table taken as the module loads */
  snap: false,
  /** read as a module loads, before the page has written anything */
  early: false,
  /** a word, compared */
  mode: 'slow',
  /** a word with a quote in it */
  quote: "it's",
  /** zero */
  zero: 0,
  /** the page works it out from a row that stays */
  rides: false,
  /** a loosely typed map lays another value over it */
  forced: false,
  /** written through a list of keys that something else reads too */
  shared: false,
  /** asked for at a value the page does not give it */
  wrong: false,
  /** read by a key whose type names it */
  half1: false,
  /** a row a level down */
  deep: { on: false, gain: 2 },
  /** another, which a map a level down holds at another value */
  deep2: { on: false },
  /** read inside a function that is called on the spot as its module loads */
  iife: false,
  /** written by the game through a list of keys */
  rt1: false,
  /** written by the page through a list of keys */
  kl1: false,
  /** the page's write of it reads the row itself */
  selfref: true,
  /** read through a list of keys handed to a helper with the table */
  hl1: false,
  /** a number below zero that a map holds at another */
  negp: -1,
  /** state of the game's own, kept in a list of things with a function that writes it and one that reads it */
  ph: false,
  /** a word that is the subject of a switch */
  sw: 'a',
  /** read in a function that a module's top-level statement calls */
  e2: false,
  /** read in a field of a class an object of which is made as a module loads */
  e3: false,
  /** a number only the old way read, and which a panel's list still names as a word */
  oldGain: 4,
  /** asked of an untyped thing in the game under the table's name */
  rec: false,
  /** the journal's line for it also writes a row that stays */
  j1: false,
  /** the journal reads a recording's word for it into a name */
  j2: false,
  /** a part whose other key a panel names by its path */
  deepn: { on: false, old: 7 },
  /** read through a getter as a module loads */
  e4: false,
  /** read in a function reached through a shorthand key as a module loads */
  e5: false,
  /** read in a function written where it is handed to a function of the game's as a module loads */
  e6: false,
  /** a word that is the subject of a switch under a cast */
  sw2: 'a',
};
/** the table as it loaded: what a row's default is read from */
export const K_DEFAULTS: typeof K = JSON.parse(JSON.stringify(K));
/** a table typed by an index: a read of one of its rows cannot be found by its name */
export const LOOSE: Record<string, boolean> = { a: false };
/** N: a number the frozen stand holds at an old value */
export const N = { reach: 3, far: 9, wide: 2, w2: 2, w3: 2, h1: 2 };
