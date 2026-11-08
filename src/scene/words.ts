// THE WORDS A MISSION'S ROW SAYS OF ITSELF: the hour, the weather and the difficulty, as the mission list and the page that draws name them. No imports: the game's own
// tables of them (game/hours.ts, game/weatherKinds.ts, game/difficulty.ts) take the words from here.
export type HourId = 'night' | 'dawn' | 'midday' | 'overcast' | 'dusk';
export type WeatherKind = 'clear' | 'overcast' | 'rain' | 'downpour' | 'fog' | 'dust' | 'snow';
export type Difficulty = 'standard' | 'hard';
