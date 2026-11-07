// THE SETTLED ROWS, RAISED. A switch row that is settled is a constant of the game (tools/qa/switches.json, kind "settled"): it has one value, the page's, and no
// address word. The tables still carry the row, at the value its module was written with, so whoever opens the page's rules (host/profile.ts pageSwitches: the
// page's boot, the harness on the page's stand, a game server) raises every one of them here first. The page then holds them (host/rustgame.ts rustSettledHold)
// and so do the tools (tools/qa/fold.ts): a later write of another value is refused and counted there.
import SWITCHES from '../../tools/qa/switches.json';
import { knobRegistry } from '../game/knobs';

export type SettledValue = boolean | number | string;
/** the rows, in the list's order */
export const SETTLED: readonly { row: string; page: SettledValue }[] = (SWITCHES as { rows: { row: string; page: SettledValue; kind: string }[] }).rows.filter(r => r.kind === 'settled');
/** the object that holds a row and its key there, or null where no table of the registry has it */
export function settledAt(row: string, tables: Record<string, any> = knobRegistry() as Record<string, any>): { table: any; key: string } | null {
  const path = row.split('.'), key = path.pop()!; let table: any = tables[path[0]]; for (let i = 1; i < path.length && table; i++) table = table[path[i]];
  return table && typeof table === 'object' && key in table ? { table, key } : null;
}
/** every settled row at its page value; the rows no table has, named */
export function settledUp(): string[] {
  const tables = knobRegistry() as Record<string, any>, absent: string[] = [];
  for (const r of SETTLED) { const at = settledAt(r.row, tables); if (!at) absent.push(r.row); else if (at.table[at.key] !== r.page) at.table[at.key] = r.page; }
  return absent;
}
