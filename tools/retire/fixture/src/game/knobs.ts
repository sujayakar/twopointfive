import { K, N } from './k';
/** the registry: a table's name is how a list of rows says which table it means */
export function knobRegistry(): Record<string, object> { return { K, N }; }
