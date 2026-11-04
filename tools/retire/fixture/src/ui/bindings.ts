// The debugger's rows, as src/ui/bindings.ts has them: a toggle that reads and writes a row, and lists of a table's rows by name.
import { K } from '../game/k';
const panel = { toggleBox(_label: string, _get: () => boolean, _set: (v: boolean) => void): void { /* a row of the panel */ } };
function rows(_table: string, _list: { k: string; label: string }[]): void { /* one row each */ }
type KnobTable = Record<string, unknown>;
function knobRows(_sec: string, _table: KnobTable, _defs: null, _save: () => void, _list: { k: string; kind: string }[]): void { /* the table itself, then its rows by name */ }
/** a loosely typed map of tables' rows laid over the tables as the game runs (a preset): a value in it is a WRITE of the row */
export const PRESETS: Record<string, Record<string, Record<string, unknown>>> = { hard: { K: { forced: false } }, soft: { K: { negp: -5, deep2: { on: false } } } };
export function preset(name: string): void { for (const [, rowsOf] of Object.entries(PRESETS[name] ?? {})) Object.assign(K, rowsOf); }
/** what a machine keeps of a table, by name */
export const KEPT: Record<string, Record<string, null>> = { K: { panelOnly: null, stay: null }, N: { far: null } };
Object.assign(KEPT.K, { extra: null });   // (added later, its own statement)
Object.assign(KEPT.K, { listRead: null });
/** a lab's switches kept as data: a name, how to read one and how to set it */
export const TOGS = (): { label: string; get: () => boolean; set: (v: boolean) => void }[] => [
  { label: 'the rule', get: () => K.on, set: v => { K.on = v; } },
  { label: 'state', get: () => K.stay, set: v => { K.stay = v; } },
];
export function bind(): void {
  panel.toggleBox('the rule', () => K.on, v => { K.on = v; });
  panel.toggleBox('the old way', () => K.old, v => { K.old = v; if (v) K.extra = false; });
  panel.toggleBox('state', () => K.stay, v => { K.stay = v; });
  rows('K', [
    { k: 'panelOnly', label: 'a row that retires' },
    { k: 'stay', label: 'a row that stays' },
  ]);
  rows('K', [{ k: 'viaList', label: 'the only row of its list' }]);
  rows('K', [{ k: 'oldGain', label: 'a number of the old way' }]);
  rows('K', [{ k: 'stay', label: 'on' }]);   // (a label that reads like a retired key is not the key)
  knobRows('sec', K as unknown as KnobTable, null, () => {}, [{ k: 'on', kind: 'toggle' }, { k: 'stay', kind: 'toggle' }]);
}
