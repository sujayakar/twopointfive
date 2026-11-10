// EVERY NUMBER THAT CAN BE TURNED, by name. A knob is declared once, next to the stage that reads it.
// From the declaration come the slider, the tooltip, and `knobs.<key>` in any shader.

import { struct, f32, u32, pack, type Field, type Struct } from '../gpu/layout';

interface Common {
  key: string;
  label: string;
  /** what it does and what it trades */
  about: string;
  /** the truth starts over when this is turned */
  resetsTruth?: boolean;
}
export interface Slider extends Common {
  kind: 'slider';
  min: number;
  max: number;
  step: number;
  value: number;
  unit?: string;
  /** equal steps on the slider are equal ratios */
  ratio?: boolean;
}
export interface Toggle extends Common {
  kind: 'toggle';
  value: number;
}
export interface Choice extends Common {
  kind: 'choice';
  /** WGSL name, then what the page calls it. The first is number 0 */
  options: [string, string][];
  value: number;
}
export type Knob = Slider | Toggle | Choice;

const all = new Map<string, Knob>();
const defaults = new Map<string, number>();
const listeners = new Set<(knob: Knob) => void>();

function add<K extends Knob>(knob: K): K {
  if (all.has(knob.key)) throw new Error(`two knobs named ${knob.key}`);
  all.set(knob.key, knob);
  defaults.set(knob.key, knob.value);
  return knob;
}
type Given<K extends Knob> = Omit<K, 'kind' | 'key'>;
export const slider = (key: string, given: Given<Slider>): Slider => add({ kind: 'slider', key, ...given });
export const toggle = (key: string, given: Omit<Given<Toggle>, 'value'> & { value?: boolean }): Toggle =>
  add({ kind: 'toggle', key, ...given, value: given.value ? 1 : 0 });
export const choice = (key: string, given: Given<Choice>): Choice => add({ kind: 'choice', key, ...given });

export const allKnobs = (): Knob[] => [...all.values()];
export function knob(key: string): Knob {
  const found = all.get(key);
  if (!found) throw new Error(`no knob named ${key}`);
  return found;
}
export const valueOf = (key: string): number => knob(key).value;
export const defaultOf = (key: string): number => defaults.get(key)!;
/** the WGSL name of what a choice stands at */
export function chosen(key: string): string {
  const k = knob(key);
  if (k.kind !== 'choice') throw new Error(`${key} is not a choice`);
  return k.options[k.value]![0];
}
export function setKnob(key: string, value: number): void {
  const k = knob(key);
  if (k.value === value) return;
  k.value = value;
  for (const tell of listeners) tell(k);
}
export function onKnob(tell: (knob: Knob) => void): void {
  listeners.add(tell);
}

const isWhole = (k: Knob) => k.kind !== 'slider' || Number.isInteger(k.step);
/** knobs as a struct; made once, when every knob has been declared */
export function knobStruct(): Struct {
  const fields: Record<string, Field> = {};
  for (const k of all.values()) fields[k.key] = (isWhole(k) ? u32 : f32)('');
  return struct('Knobs', fields);
}
export function packKnobs(s: Struct): ArrayBuffer {
  const bytes = new ArrayBuffer(Math.ceil(s.stride / 16) * 16);
  const values: Record<string, number> = {};
  for (const k of all.values()) values[k.key] = k.value;
  pack(s, new DataView(bytes), 0, values);
  return bytes;
}
/** `const LAMP_PICK_SCORE = 1u;` for every option of every choice */
export function choiceConstants(): string {
  const lines: string[] = [];
  for (const k of all.values()) {
    if (k.kind !== 'choice') continue;
    k.options.forEach(([name], i) => lines.push(`const ${k.key.toUpperCase()}_${name.toUpperCase()} = ${i}u;`));
  }
  return lines.join('\n');
}
