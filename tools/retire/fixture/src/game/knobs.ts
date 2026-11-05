import { K, N } from './k';
import { M } from './m';
/** the registry: a table's name is how a list of rows says which table it means */
export function knobRegistry(): Record<string, object> { return { K, N, M }; }
