/** WHICH MISSIONS SEAT TWO: the kinds whose own rules have been taken through for more than one player. The others are written round one commander (the hostage rescue's word and its route flow, the quiet
 *  mission's one partner) and refuse a game of two until they are. */
export const SEAT_KINDS: readonly string[] = ['roomclear', 'nightbreach'];
/** can a mission of this kind be played by two (the lobby asks before a game is made: ui/coop.ts) */
export const seatsKind = (kind: string): boolean => SEAT_KINDS.includes(kind);
