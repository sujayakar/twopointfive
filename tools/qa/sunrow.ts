// EACH HOUR'S SUN, off the game's own table (rust/game_types/src/hours/types.rs): for the picture's checks, which ask what an hour's sun stands for.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const text = readFileSync(resolve(import.meta.dir, '../../rust/game_types/src/hours/types.rs'), 'utf8');
export type HourName = 'dawn' | 'midday' | 'overcast' | 'dusk';
export const HOURS = Object.fromEntries([...text.matchAll(/t\.set\(String::from\("(\w+)"\), crate::hours::HourDef \{.*?HourDefSun \{[^}]*?intensity: ([0-9.]+)/g)].map(m => [m[1], { sun: { intensity: Number(m[2]) } }])) as Record<HourName, { sun: { intensity: number } }>;
for (const h of ['dawn', 'midday', 'overcast', 'dusk'] as const) if (!(HOURS[h]?.sun.intensity > 0)) throw new Error(`sunrow: no sun read for ${h} in the game's hours`);
