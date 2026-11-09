// THE WORDS A MISSION'S ROW SAYS OF ITSELF: the hour, the weather and the difficulty, as the mission list and the page that draws name them. No imports: the game's own
// tables of them (rust/game_rules/src/hours, rust/game_rules/src/weather_kinds, rust/game_rules/src/difficulty) take the words from here.
export type HourId = 'night' | 'dawn' | 'midday' | 'overcast' | 'dusk';
export type WeatherKind = 'clear' | 'overcast' | 'rain' | 'downpour' | 'fog' | 'dust' | 'snow';
export type Difficulty = 'standard' | 'hard';
