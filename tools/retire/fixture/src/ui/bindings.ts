// The debugger's rows, as src/ui/bindings.ts has them: a toggle that reads and writes a row, and lists of a table's rows by name.
import { K } from '../game/k';
const panel = { toggleBox(_label: string, _get: () => boolean, _set: (v: boolean) => void): void { /* a row of the panel */ } };
function rows(_table: string, _list: { k: string; label: string }[]): void { /* one row each */ }
/** what a machine keeps of a table, by name */
export const KEPT: Record<string, Record<string, null>> = { K: { panelOnly: null, stay: null }, N: { far: null } };
Object.assign(KEPT.K, { extra: null });   // (added later, its own statement)
Object.assign(KEPT.K, { listRead: null });
export function bind(): void {
  panel.toggleBox('the rule', () => K.on, v => { K.on = v; });
  panel.toggleBox('the old way', () => K.old, v => { K.old = v; if (v) K.extra = false; });
  panel.toggleBox('state', () => K.stay, v => { K.stay = v; });
  rows('K', [
    { k: 'panelOnly', label: 'a row that retires' },
    { k: 'stay', label: 'a row that stays' },
  ]);
  rows('K', [{ k: 'viaList', label: 'the only row of its list' }]);
}
