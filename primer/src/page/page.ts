// THE PAGE round the canvas: the steps, the controls, the panels' menus, the pinned pixel, the mixer, the
// meter, and the shaders' text.

import type { Lab, Layout } from '../lab';
import { panelsIn } from '../lab';
import type { Value } from '../gpu/layout';
import { ROOMS, roomNamed } from '../rooms';
import { RAY_KINDS, projectLine } from './rays';
import { MIXER } from '../stages/picture';
import { dolly, orbit, pan } from '../world/camera';
import { add, scale, type Vec3 } from '../world/vec';
import { CHAPTERS, type Chapter } from './chapters';
import { control, forgetControls } from './controls';
import { el } from './dom';
import { defaultOf, knob, setKnob } from './knobs';

const LAYOUTS: [Layout, string][] = [['one', '1'], ['two', '2'], ['four', '4'], ['wipe', 'Wipe']];

export class Page {
  private chapter = CHAPTERS[0]!;
  private side = el('aside', { id: 'side' });
  private right = el('aside', { id: 'right' });
  private stage = el('main', { id: 'stage' });
  private top = el('header', { id: 'top' });
  private headers = el('div', { id: 'panel-heads' });
  private marker = el('div', { id: 'marker', hidden: true });
  private overlay = el('canvas', { id: 'overlay' });
  private wipeLine = el('div', { id: 'wipe-line', hidden: true, title: 'Drag: one picture on each side' });
  private status = el('div', { id: 'status' });
  private pinnedRows = el('div', { id: 'pinned' });
  private meterRows = el('div', { id: 'meter' });
  private code: HTMLDialogElement = el('dialog', { id: 'code' });
  private tab: 'pixel' | 'mixer' | 'meter' = 'pixel';

  constructor(private lab: Lab, root: HTMLElement) {
    this.stage.append(lab.canvas, this.overlay, this.headers, this.marker, this.wipeLine, this.status);
    root.append(this.top, this.side, this.stage, this.right, this.code);
    this.listen();
    this.go(this.chapter.number);
    setInterval(() => void this.refresh(), 150);
  }

  go(number: number): void {
    const c = CHAPTERS.find((x) => x.number === number) ?? CHAPTERS[0]!;
    this.chapter = c;
    const { lab } = this;
    if (lab.world.room?.id !== c.room) this.openRoom(c.room);
    lab.layout = c.layout;
    lab.zoom = 1;
    lab.middle = [0.5, 0.5];
    c.views.forEach((id, i) => (lab.shown[i] = id));
    this.draw();
  }

  private openRoom(id: string, sliders?: Record<string, number>): void {
    const room = roomNamed(id);
    const fresh = this.lab.world.room?.id !== room.id;
    this.lab.openRoom(room, sliders);
    if (fresh) setKnob('exposure', room.exposure);
  }

  // ---- drawn when something is chosen

  private draw(): void {
    forgetControls();
    this.drawTop();
    this.drawSide(this.chapter);
    this.drawRight();
    this.drawHeads();
  }

  private drawTop(): void {
    const { lab } = this;
    const button = (text: string, title: string, press: () => void, pressed?: boolean) =>
      el('button', {
        type: 'button', title, click: () => { press(); this.draw(); },
        ...(pressed === undefined ? {} : { 'aria-pressed': String(pressed) }),
      }, text);
    const rooms = el('select', { id: 'room', title: 'The room', change: () => { this.openRoom(rooms.value); this.draw(); } },
      ...ROOMS.map((r) => el('option', { value: r.id, selected: r.id === lab.world.room.id }, r.name)));
    this.top.replaceChildren(
      el('strong', {}, 'Light Primer'),
      rooms,
      el('span', { class: 'question' }, lab.world.room.question),
      el('div', { class: 'group' },
        button(lab.playing ? 'Pause' : 'Play', 'The world’s clock (Space)', () => (lab.playing = !lab.playing), lab.playing),
        button('Freeze', 'Nothing is made anew; the frame stands (F)', () => (lab.frozen = !lab.frozen), lab.frozen),
        button('One frame', 'Make one frame (.)', () => { lab.frozen = true; lab.step(); }),
        button('Ghost', 'Fly a second camera through the frozen frame (G)', () => this.toggleGhost(), lab.ghost !== null),
        button('Camera', 'Back to where the room begins', () => lab.resetCamera())),
      el('div', { class: 'group' },
        ...LAYOUTS.map(([layout, text]) =>
          button(text, 'Panels', () => (lab.layout = layout), lab.layout === layout))),
    );
  }

  private drawSide(c: Chapter): void {
    const { lab } = this;
    const numbers = el('nav', { id: 'steps', 'aria-label': 'Steps' },
      ...CHAPTERS.map((x) =>
        el('button', {
          type: 'button', title: x.title, click: () => this.go(x.number), 'aria-pressed': String(x === c),
        }, String(x.number))));
    const sliders = lab.world.room.sliders ?? [];
    const roomRows = sliders.map((s) => {
      const shown = el('output', {}, String(lab.world.sliders[s.key]));
      const input = el('input', {
        type: 'range', id: `room-${s.key}`, min: String(s.min), max: String(s.max), step: String(s.step),
        input: () => {
          shown.textContent = input.value;
          this.openRoom(lab.world.room.id, { ...lab.world.sliders, [s.key]: Number(input.value) });
        },
      });
      input.value = String(lab.world.sliders[s.key]);
      return el('div', { class: 'knob', title: s.about }, el('label', { for: `room-${s.key}` }, s.label), shown, input);
    });
    const section = (title: string, rows: HTMLElement[], className = '') =>
      rows.length ? el('section', { class: className }, el('h3', {}, title), ...rows) : null;
    this.side.replaceChildren(
      numbers,
      el('h1', {}, `${c.number}. ${c.title}`),
      el('p', { class: 'lead' }, c.question),
      ...c.text.map((t) => el('p', { html: t })),
      ...(c.formula ? [el('pre', { class: 'formula' }, c.formula)] : []),
      ...[
        section('Turn', c.knobs.map(control)),
        section('The room', roomRows),
        section('Break it', c.breaks.map(control), 'breaks'),
        section('Read and edit', c.stages.map((name) =>
          el('button', { type: 'button', class: 'stage-link', click: () => this.showCode(name) },
            el('b', {}, name), ' ', el('span', {}, lab.graph.stages.find((s) => s.name === name)?.question ?? '')))),
      ].filter((x): x is HTMLElement => x !== null),
    );
  }

  private drawRight(): void {
    const tabs: ['pixel' | 'mixer' | 'meter', string][] = [['pixel', 'Pixel'], ['mixer', 'Mixer'], ['meter', 'Meter']];
    const head = el('div', { class: 'segments', role: 'group' },
      ...tabs.map(([id, text]) =>
        el('button', { type: 'button', 'aria-pressed': String(this.tab === id), click: () => { this.tab = id; this.draw(); } }, text)));
    const body =
      this.tab === 'pixel' ? this.pinnedRows
      : this.tab === 'meter' ? this.meterRows
      : el('div', { id: 'mixer' }, ...MIXER.map((k) => this.mixerRow(k.key)));
    this.right.replaceChildren(head, body);
  }

  private mixerRow(key: string): HTMLElement {
    const row = control(key);
    const all = MIXER.map((k) => k.key);
    const alone = () => all.every((k) => k === key || knob(k).value === 0);
    row.append(
      el('span', { class: 'mix-buttons' },
        el('button', { type: 'button', title: 'Mute', click: () => setKnob(key, knob(key).value ? 0 : 1) }, 'M'),
        el('button', {
          type: 'button', title: 'Only this',
          click: () => { const was = alone(); all.forEach((k) => setKnob(k, was || k === key ? defaultOf(k) : 0)); },
        }, 'S')));
    return row;
  }

  private drawHeads(): void {
    const { lab } = this;
    const groups = new Map<string, HTMLOptGroupElement>();
    const makeMenu = (i: number) => {
      groups.clear();
      const menu = el('select', { 'aria-label': `Panel ${i + 1} shows`, change: () => { lab.shown[i] = menu.value; this.drawHeads(); } });
      for (const v of lab.views.all()) {
        if (!groups.has(v.group)) {
          const g = el('optgroup', { label: v.group });
          groups.set(v.group, g);
          menu.append(g);
        }
        groups.get(v.group)!.append(el('option', { value: v.id, selected: v.id === lab.shown[i] }, v.name));
      }
      return menu;
    };
    const dpr = lab.canvas.width / Math.max(1, lab.canvas.clientWidth);
    const rects = lab.panelRects();
    this.headers.replaceChildren(
      ...Array.from({ length: panelsIn[lab.layout] }, (_, i) => {
        const r = rects[i]!;
        const right = lab.layout === 'wipe' && i === 1;
        const view = lab.views.find(lab.shown[i]!);
        return el('div', {
          class: 'panel-head',
          style: `top:${r.y / dpr}px;` + (right ? 'right:0' : `left:${r.x / dpr}px`),
          title: view?.about ?? '',
        }, makeMenu(i));
      }));
    this.wipeLine.hidden = lab.layout !== 'wipe';
    this.wipeLine.style.left = `${lab.wipe * 100}%`;
  }

  // ---- drawn as time goes by

  private async refresh(): Promise<void> {
    const { lab } = this;
    const building = !lab.ready && lab.graph.errors.size === 0;
    const paths = lab.made.includes('truth') ? `${lab.truthCount} paths a pixel` : '';
    const errors = [...lab.graph.errors].map(([stage, text]) => `${stage}\n${text}`).join('\n\n');
    this.status.textContent = errors || [building ? 'making shaders…' : '', paths].filter(Boolean).join(' · ');
    this.status.classList.toggle('error', errors !== '');

    const dpr = lab.canvas.width / Math.max(1, lab.canvas.clientWidth);
    this.marker.hidden = !lab.pinned;
    if (lab.pinned) {
      const r = lab.panelRects()[0]!;
      const x = (lab.pinned[0] - lab.middle[0]) * lab.zoom + 0.5;
      const y = (lab.pinned[1] - lab.middle[1]) * lab.zoom + 0.5;
      this.marker.style.left = `${(r.x + x * r.width) / dpr}px`;
      this.marker.style.top = `${(r.y + y * r.height) / dpr}px`;
    }
    if (lab.pinned) await lab.rays.read();
    this.drawOverlay();
    if (this.tab === 'pixel') await this.refreshPinned();
    if (this.tab === 'meter') this.refreshMeter();
  }

  /** the pinned pixel's rays, and the frame's camera as the ghost sees it */
  private drawOverlay(): void {
    const { lab, overlay } = this;
    overlay.width = lab.canvas.width;
    overlay.height = lab.canvas.height;
    const pen = overlay.getContext('2d')!;
    lab.panelRects().slice(0, panelsIn[lab.layout]).forEach((r, i) => {
      const eye = lab.eyeOfPanel(i);
      pen.save();
      pen.beginPath();
      pen.rect(r.x, r.y, r.width, r.height);
      pen.clip();
      const at = ([u, v]: [number, number]): [number, number] => [
        r.x + ((u - lab.middle[0]) * lab.zoom + 0.5) * r.width,
        r.y + ((v - lab.middle[1]) * lab.zoom + 0.5) * r.height,
      ];
      const line = (a: Vec3, b: Vec3, color: string, width: number) => {
        const l = projectLine(eye, a, b);
        if (!l) return;
        pen.strokeStyle = color;
        pen.lineWidth = width;
        pen.beginPath();
        pen.moveTo(...at(l[0]));
        pen.lineTo(...at(l[1]));
        pen.stroke();
      };
      if (lab.ghost && lab.shown[i] === 'ghost') {
        const e = lab.eyeOfPanel(-1);
        const corners = [[-1, -1], [1, -1], [1, 1], [-1, 1]].map(([x, y]) =>
          add(e.eye, add(e.forward, add(scale(e.right, x!), scale(e.up, y!)))));
        corners.forEach((c, k) => {
          line(e.eye, c, '#e8b04b', 1);
          line(c, corners[(k + 1) % 4]!, '#e8b04b', 1);
        });
      }
      if (lab.pinned) lab.rays.lines.forEach((l) => line(l.start, l.end, RAY_KINDS[l.kind]?.[1] ?? '#fff', 1.25));
      pen.restore();
    });
  }

  private reading = false;
  private async refreshPinned(): Promise<void> {
    if (!this.lab.pinned) {
      this.pinnedRows.replaceChildren(el('p', { class: 'hint' }, 'Click a pixel to see its value in every buffer that is being made.'));
      return;
    }
    if (this.reading) return;
    this.reading = true;
    try {
      const all = await this.lab.readPinned();
      const kinds = [...new Set(this.lab.rays.lines.map((l) => l.kind))].sort();
      this.pinnedRows.replaceChildren(
        el('p', { class: 'hint rays' }, ...kinds.map((k) =>
          el('span', {}, el('span', { class: 'swatch', style: `background:${RAY_KINDS[k]![1]}` }), RAY_KINDS[k]![2]))),
        ...all.map(({ name, about, struct, values }) =>
          el('section', {},
            el('h4', { title: about }, name.replaceAll('_', ' ')),
            el('dl', {}, ...Object.entries(struct.fields).flatMap(([key, f]) => [
              el('dt', { title: f.about }, key.replaceAll('_', ' ')),
              el('dd', {}, shown(values[key]!, f.look.kind === 'light' || f.look.kind === 'color'), f.unit ? el('i', {}, ' ' + f.unit) : null),
            ])))));
    } finally {
      this.reading = false;
    }
  }

  private refreshMeter(): void {
    const { graph, gpu } = this.lab;
    const [w, h] = graph.renderSize;
    const rows = this.lab.made.filter((n) => !n.startsWith('view ')).map((name) => {
      const stage = graph.stages.find((s) => s.name === name)!;
      const writes = Array.isArray(stage.writes) ? stage.writes : Object.values(stage.writes);
      const bytes = writes.reduce((sum, x) => sum + graph.bytesOf(x), 0);
      const ms = graph.timer.ms.get(name);
      return el('tr', {}, el('td', {}, name), el('td', {}, ms === undefined ? '–' : ms.toFixed(2)), el('td', {}, (bytes / 1e6).toFixed(1)));
    });
    this.meterRows.replaceChildren(
      el('p', { class: 'hint' }, `${gpu.about || 'GPU'} · render ${w} × ${h}, output ${graph.outputSize.join(' × ')}`),
      el('table', {}, el('thead', {}, el('tr', {}, el('th', {}, 'Stage'), el('th', {}, 'ms'), el('th', {}, 'MB'))), el('tbody', {}, ...rows)),
      el('p', { class: 'hint' }, 'Only what the panels depend on runs. On Apple GPUs passes overlap, so these do not add up to the frame.'));
  }

  // ---- shaders

  private showCode(name: string): void {
    const { graph } = this.lab;
    const stage = graph.stages.find((s) => s.name === name);
    if (!stage) return;
    const list = (names: string[] | Record<string, string>) => (Array.isArray(names) ? names : Object.values(names)).join(', ') || '–';
    const text = el('textarea', { spellcheck: 'false', 'aria-label': `The shader of ${name}` });
    text.value = graph.codeOf(stage);
    const problem = el('pre', { class: 'problem' });
    let waiting = 0;
    text.addEventListener('input', () => {
      clearTimeout(waiting);
      waiting = window.setTimeout(() => {
        graph.edit(name, text.value);
        setTimeout(() => (problem.textContent = graph.errors.get(name) ?? ''), 400);
      }, 400);
    });
    this.code.replaceChildren(
      el('header', {},
        el('h2', {}, name), el('p', {}, stage.question),
        el('button', { type: 'button', click: () => { graph.edit(name, null); text.value = stage.code; problem.textContent = ''; } }, 'As it was'),
        el('button', { type: 'button', click: () => this.code.close() }, 'Close')),
      el('p', { class: 'hint' }, `reads ${list(stage.reads)} · writes ${list(stage.writes)} · uses ${stage.uses.join(', ')}`),
      text, problem);
    this.code.show();
  }

  // ---- mouse and keys

  private toggleGhost(): void {
    const { lab } = this;
    if (lab.ghost) {
      lab.ghost = null;
      lab.frozen = false;
      return;
    }
    lab.ghost = structuredClone(lab.camera);
    lab.frozen = true;
    lab.shown[0] = 'ghost';
  }

  private listen(): void {
    const { lab, stage } = this;
    const { canvas } = lab;
    const fit = () => {
      const dpr = Math.min(window.devicePixelRatio, 1);
      canvas.width = Math.max(16, Math.round(stage.clientWidth * dpr));
      canvas.height = Math.max(16, Math.round(stage.clientHeight * dpr));
      if (lab.world.room) this.drawHeads();
    };
    new ResizeObserver(fit).observe(stage);
    fit();
    const at = (e: PointerEvent): [number, number] => {
      const r = canvas.getBoundingClientRect();
      return [((e.clientX - r.left) / r.width) * canvas.width, ((e.clientY - r.top) / r.height) * canvas.height];
    };
    let from: { x: number; y: number; moved: boolean } | null = null;
    canvas.addEventListener('pointerdown', (e) => {
      canvas.setPointerCapture(e.pointerId);
      from = { x: e.clientX, y: e.clientY, moved: false };
    });
    canvas.addEventListener('pointermove', (e) => {
      if (!from) return;
      const dx = e.clientX - from.x;
      const dy = e.clientY - from.y;
      if (!from.moved && Math.hypot(dx, dy) < 4) return;
      from = { x: e.clientX, y: e.clientY, moved: true };
      const c = lab.ghost ?? lab.camera;
      if (e.shiftKey || e.buttons === 4 || e.buttons === 2) pan(c, dx, dy);
      else orbit(c, dx, dy);
    });
    canvas.addEventListener('pointerup', (e) => {
      if (from && !from.moved) lab.pinned = e.altKey ? null : lab.pictureAt(...at(e));
      from = null;
    });
    canvas.addEventListener('contextmenu', (e) => e.preventDefault());
    canvas.addEventListener('wheel', (e) => {
      e.preventDefault();
      if (e.ctrlKey || e.metaKey) {
        // magnify the pictures about the pointer
        const p = lab.pictureAt(...at(e as unknown as PointerEvent));
        const next = Math.max(1, Math.min(32, lab.zoom * Math.exp(-e.deltaY * 0.005)));
        if (p) lab.middle = lab.middle.map((m, i) => p[i]! + (m - p[i]!) * (lab.zoom / next)) as [number, number];
        if (next === 1) lab.middle = [0.5, 0.5];
        lab.zoom = next;
      } else dolly(lab.ghost ?? lab.camera, e.deltaY);
    }, { passive: false });
    this.wipeLine.addEventListener('pointerdown', (e) => {
      this.wipeLine.setPointerCapture(e.pointerId);
      const move = (m: PointerEvent) => {
        const r = stage.getBoundingClientRect();
        lab.wipe = Math.max(0, Math.min(1, (m.clientX - r.left) / r.width));
        this.wipeLine.style.left = `${lab.wipe * 100}%`;
      };
      this.wipeLine.addEventListener('pointermove', move);
      this.wipeLine.addEventListener('pointerup', () => this.wipeLine.removeEventListener('pointermove', move), { once: true });
    });
    window.addEventListener('keydown', (e) => {
      if ((e.target as HTMLElement).closest('textarea, input, select')) return;
      if (e.key === ' ') lab.playing = !lab.playing;
      else if (e.key === 'f') lab.frozen = !lab.frozen;
      else if (e.key === 'g') this.toggleGhost();
      else if (e.key === '.') { lab.frozen = true; lab.step(); }
      else if (e.key === 'Escape') lab.pinned = null;
      else if (e.key === 'ArrowRight' || e.key === 'ArrowLeft') {
        const i = CHAPTERS.indexOf(this.chapter) + (e.key === 'ArrowRight' ? 1 : -1);
        if (CHAPTERS[i]) return this.go(CHAPTERS[i].number);
      } else return;
      e.preventDefault();
      this.draw();
    });
  }
}

function shown(value: Value, asColor: boolean): string | HTMLElement {
  const one = (v: number) => (v === 0xffffffff ? 'none' : Number.isInteger(v) && Math.abs(v) < 1e6 ? String(v) : v.toPrecision(4));
  if (typeof value === 'number') return one(value);
  const text = value.map(one).join('  ');
  if (!asColor) return text;
  const most = Math.max(...value.slice(0, 3), 1);
  const rgb = value.slice(0, 3).map((v) => Math.round(255 * Math.pow(Math.max(0, v) / most, 1 / 2.2)));
  return el('span', {}, el('span', { class: 'swatch', style: `background:rgb(${rgb.join(',')})` }), text);
}
