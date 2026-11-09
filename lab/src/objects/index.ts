import type { ObjectKind } from './object';
import { chair } from './chair';
import { ficus } from './ficus';
import { person } from './person';
import { shelf } from './shelf';
import { car } from './car';
import { gravel } from './gravel';
import { bollards } from './bollards';
import { fruitcrate } from './fruitcrate';
import { officechair } from './officechair';
/** every object there is, by name */
export const OBJECTS: Record<string, ObjectKind> = Object.fromEntries([chair, ficus, person, shelf, car, gravel, bollards, fruitcrate, officechair].map(o => [o.name, o]));
