// Read off a link, and nothing else: the page that shows the cards asks it of a link before any game stands.
/** WHICH QUIET KIND a mission's link is, by the reading the dress above has always used (it is that reading, moved here so that two callers cannot
 *  drift): 'quiet' — a quiet mission, alone or as two; 'solo' — a link that names NO mission (the solo night); null — a loud one (room clear, night
 *  breach, rescue, the four). Read off the LINK and never off the scenario that is up: a set way's picture is staged as a room clear
 *  (kitViewSetQuery strips `quiet` / `solo`), so `game.quiet.active` is false there. The quiet picture's switch (host/kitView.ts KIT_VIEW_QUIET)
 *  asks this. */
export function kitViewQuietKind(q: URLSearchParams): 'quiet' | 'solo' | null {
  const loud = ['roomclear', 'nightbreach', 'rescue'].some(k => q.has(k)), quiet = !loud && q.has('quiet'), none = !loud && !quiet && !q.has('squad4');
  return quiet ? 'quiet' : none ? 'solo' : null;
}
