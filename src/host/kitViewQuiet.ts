// Read off a link, and nothing else: the page that shows the cards asks it of a link before any game stands.
/** WHICH QUIET KIND a mission's link is, by the reading the men's dress uses: 'quiet' — a quiet mission, alone or as two; 'solo' — a link that names NO mission (the solo night); null — a loud one (room clear, night
 *  breach, rescue, the four). Read off the LINK and never off the scenario that is up: a set way's picture is staged as a room clear
 *  (rust/game_start/src/kit_set.rs set_query strips `quiet` / `solo`). The kit screen asks this for the quiet picture's fade (ui/kitView.ts); the module's own reading is kit_set.rs quiet_kind. */
export function kitViewQuietKind(q: URLSearchParams): 'quiet' | 'solo' | null {
  const loud = ['roomclear', 'nightbreach', 'rescue'].some(k => q.has(k)), quiet = !loud && q.has('quiet'), none = !loud && !quiet && !q.has('squad4');
  return quiet ? 'quiet' : none ? 'solo' : null;
}
