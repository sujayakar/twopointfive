// Sliders, switches and choices, made from a knob's declaration.

import { defaultOf, knob, onKnob, setKnob, type Knob } from './knobs';
import { el } from './dom';

const refreshers = new Map<string, Set<() => void>>();
onKnob((k) => refreshers.get(k.key)?.forEach((refresh) => refresh()));
/** forget controls that are no longer on the page */
export function forgetControls(): void {
  refreshers.clear();
}
const refreshOn = (key: string, refresh: () => void) => {
  if (!refreshers.has(key)) refreshers.set(key, new Set());
  refreshers.get(key)!.add(refresh);
  refresh();
};

const digits = (step: number) => Math.min(4, Math.max(0, Math.ceil(-Math.log10(step) - 1e-9)));

export function control(key: string): HTMLElement {
  const k = knob(key);
  if (k.kind === 'slider') return sliderRow(k);
  if (k.kind === 'toggle') return toggleRow(k);
  return choiceRow(k);
}

function sliderRow(k: Extract<Knob, { kind: 'slider' }>): HTMLElement {
  // a slider in ratios moves over the logarithm
  const toSlider = (v: number) => (k.ratio ? Math.log(v) : v);
  const fromSlider = (v: number) => (k.ratio ? Math.exp(v) : v);
  const input = el('input', {
    type: 'range', id: `knob-${k.key}`,
    min: String(toSlider(k.min)), max: String(toSlider(k.max)), step: k.ratio ? '0.01' : String(k.step),
    input: () => setKnob(k.key, Math.round(fromSlider(Number(input.value)) / k.step) * k.step),
  });
  const shown = el('output', { for: `knob-${k.key}` });
  refreshOn(k.key, () => {
    input.value = String(toSlider(k.value));
    shown.textContent = k.value.toFixed(digits(k.step)) + (k.unit ? ` ${k.unit}` : '');
  });
  return el('div', { class: 'knob', title: k.about, dblclick: () => setKnob(k.key, defaultOf(k.key)) },
    el('label', { for: `knob-${k.key}` }, k.label), shown, input);
}

function toggleRow(k: Extract<Knob, { kind: 'toggle' }>): HTMLElement {
  const input = el('input', { type: 'checkbox', id: `knob-${k.key}`, change: () => setKnob(k.key, input.checked ? 1 : 0) });
  refreshOn(k.key, () => (input.checked = k.value !== 0));
  return el('div', { class: 'knob toggle', title: k.about }, input, el('label', { for: `knob-${k.key}` }, k.label));
}

function choiceRow(k: Extract<Knob, { kind: 'choice' }>): HTMLElement {
  const buttons = k.options.map(([, label], i) =>
    el('button', { type: 'button', click: () => setKnob(k.key, i) }, label));
  refreshOn(k.key, () => buttons.forEach((b, i) => b.setAttribute('aria-pressed', String(i === k.value))));
  return el('div', { class: 'knob choice', title: k.about },
    el('span', {}, k.label), el('div', { class: 'segments', role: 'group' }, ...buttons));
}
