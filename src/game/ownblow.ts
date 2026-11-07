// HIS OWN CHARGE GOES OFF FROM HIS OWN ACTIONS, NOT FROM THE SQUAD'S KEY (the maintainer, 10-22, on a tutorial's 'F: blow it': "F should be reserved for
// squad. instead it should be one of our own actions").
//
// ONE SWITCH, `OWNBLOW.on` — ON in the page (host/profile.ts pageSwitches; `?ownblow=0` = as it was), OFF here and on the frozen harness stand
// (`TPS_OWNBLOW=1`; a FROZEN_HOLDS row, in the knob registry and a journal's tables: a recording from before the table replays with it off). With it on:
//
//   · A ROW OF HIS OWN LIST, 'blow the charge', wherever he stands, while a charge his own hands set (a door's — doortools.ts, a wall's — wallcharge.ts)
//     stands free and his hands are: under the rows the things in his reach had (player.ts buildInteractables), and never the highlighted row while
//     another thing has one — so a tap of the interaction key gives it only when it is the only thing to do, and with anything else in reach it is a
//     wedge of his ring (handring.ts), never past the fourth. The nearest of his own goes; a door's linked charges go with it as on any trigger.
//   · A BARE TAP NEVER SETS IT OFF WITH HIM IN ITS BLAST (interact.ts asks `ownBlowHeld`): the row is greyed and its line says 'too close: you' first; the
//     ring's wedge, or the held list's row, still does it — two presses, meant. (The tap is the key that set the charge and the key that breaks a
//     setting off: one press too many at a wall's foot would have been the blast.)
//   · THE SQUAD'S KEY (doortools.ts blowKey) NO LONGER BLOWS IT, and no longer stops at it: a press with only his own charge set goes on to whoever
//     else reads the key (the drill, a rescue's entry), as with no charge set. The key is still the word: a charge the squad set and left to his call
//     is the key's, and so is HIS charge while the squad works its door or its wall (stood off behind it, stacked on it): one press, it goes with
//     them. `keyIs` on each tool says which; the floor plan's bars (floorplan.ts waitsOnKey) read the same.
//   · The words that named the key for his own charge (the door's line, the row's hint, the message when it is set, the squad block's note) name the
//     interaction key.
//
// A mission on rails that lists the rows it offers (rails.ts `rows`) offers this one where it offers 'charge' — what he may set he may set off — and from
// the step that opens the action `blow`, or the squad's word (the key that had it): until then a tap beside his own charge does nothing. A step whose
// line names the squad's key for it carries a `blowLine` for this switch (scene/scenarios.ts LessonStep, lesson.ts lineOf).
// No dice, no clock of its own, no state: the row is derived from the charges that stand set.

export const OWNBLOW = {
  /** the switch (above). false = the squad's key blows any set charge, his own among them, and his list has no row for it */
  on: false,
};
/** who set a charge, when it was the player's own hands (doortools.ts SetCharge.by, wallcharge.ts SetWallCharge.by) */
export const SAM_BY = 'Sam';

/** the key a set charge's ROW names — a row's key column and its `line2`, which the page words as the key that action is on: the squad's (`key`: its cap,
 *  as the caller words it) while that key blows it, else the interaction's */
export const blowKeyWord = (keyIs: boolean, key: string): string => keyIs ? key : 'Space';
/** …and what a line drawn AS IT IS WRITTEN says after 'charge set' (a door's header, a note, the log): the squad's key by name while that key blows it;
 *  nothing for his own — its row, with the key it is on, is on his list */
export const blowTail = (keyIs: boolean, key: string, sep = ' · '): string => keyIs ? `${sep}press ${key} to blow it` : '';

