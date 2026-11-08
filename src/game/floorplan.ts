// THE PLAN ON THE FLOOR'S NUMBERS. What it lists is the Rust game's (rust/game_rules/src/view/plan.rs, `view.plan`) and ui/planfloor.ts draws it: its men's ways, marks and fans, what waits on the
// key, the throws. The maintainer (10-13): "it would be good to have the squad move to do a throw (and maybe visualize who's doing it and the path on the ground?)"; the Door Kickers 2
// focus list's item 6: "four placed men are four lines of text. Done looks like: cover the top-left corner: you still know each man's mark, sector and which door waits on F. Our men only."
export const PLAN = {
  on: true,
  /** the parts, each off = that part not drawn */
  paths: true, marks: true, sectors: true, who: true, word: true, throws: true,
  /** the men's ways, marks, tags and fans only while the order menu is up — his word 10-13: 'they should only show when you're in the mode after pressing x. in
   *  normal gameplay they shouldn't show up'. Outside it stay: an order just GIVEN (the order book's own log, not a module's re-assign), fading out inside
   *  `freshSecs`; whatever waits on the key; a teammate's ordered throw until it leaves his hand. false = all of it for as long as the orders stand, as before */
  menuOnly: true,
  /** THE KEY ON A DOOR SAYS WHETHER IT IS LIVE (the maintainer, 10-22: "need some indicator on the F on the door when it's queued vs live. maybe black
   *  background to lighting up to yellow background"): the key's cap is drawn on every door that waits on it — DARK while its men are still on their
   *  way to it (queued: the press is kept, or sends men who are not set) or while the press would not send it, LIT in their colour once every man of
   *  it stands on his place and the press sends them now (PlanDoor.cap). false: the cap only on the door the key would send now, on its plate,
   *  the same whether or not its men are there. Picture only */
  keyLit: true,
  /** … AND ON A DOOR HELD FROM OUTSIDE IT IS LIVE ONLY ONCE THE MEN HAVE SAID SO (squad sense's finding, 10-23: on a held door the key pressed
   *  before the men's 'holding · on your word' — six or seven seconds into the hold — cuts their slice of the room short, and the cap was lit the
   *  moment their feet were on the jambs, asking for exactly that press): the cap of a door whose men hold its threshold on the word stays DARK
   *  until they have made that call (doortools.ts `thr.told`), then lights. The key itself does what it did. false: lit with their feet on their
   *  places, as it was. Picture only, on on both stands (`?thrtold=0` off) */
  thrTold: true,
  /** seconds an order just given is drawn at full strength — its men ringed, its ways bright */
  freshSecs: 2.5,
  /** how strong a way is drawn after that, while he still walks it · a mark and a fan under a man who stands */
  walkDim: 0.6, restDim: 0.45,
  /** a mark's ring, metres · a fan's reach, metres, and half angle, degrees · a sight line to what he watches is drawn out to this far, metres */
  ringM: 0.3, sectorM: 1.6, sectorDeg: 24, sightM: 14,
  /** the nav grid is asked for a leg his own path does not cover at most this often, seconds */
  reaskSecs: 0.5,
  /** the quiet pair's man waiting on the mark: the key (and, with `keyLit` off, its bar) is this far ahead of his feet, metres */
  pairBarM: 0.55,
  /** A GAME OF SEVERAL PLAYERS: another player's squad's plan is drawn on this page this much as strong as the page's own (ui/planfloor.ts
   *  plansToDraw); 0 = not asked for, the page as it was. A guess: nobody has seen it on a GPU. Picture only */
  others: 0.45,
};
