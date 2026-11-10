// THE PAGE round the canvas: the steps, the controls, the panels' menus, the pinned pixel, the mixer, the
// meter, and the shaders' text.

import type { Lab, Layout } from '../lab';
import { panelsIn } from '../lab';
import { readBack } from '../gpu/device';
import { unpack, type Value } from '../gpu/layout';
import { CACHE_SLOTS } from '../stages/cache';
import { ROOMS, roomNamed } from '../rooms';
import { RAY_KINDS, projectLine } from './rays';
import { DIAL_FORMS, DIAL_RAYS } from '../stages/dial';
import { STRIP_MOST, STRIP_WIDTHS } from '../stages/strip';
import { MIXER } from '../stages/picture';
import { dolly, orbit, pan } from '../world/camera';
import { add, axesAround, dot, scale, sub, type Vec3 } from '../world/vec';
import { CHAPTERS, type Chapter } from './chapters';
import { control, forgetControls } from './controls';
import { el } from './dom';
import { defaultOf, knob, setKnob, valueOf } from './knobs';

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
  private lampRows = el('div', { id: 'lamps' });
  private dialRows = el('div', { id: 'dial-rows' });
  private levelRows = el('div', { id: 'level-rows' });
  private cacheRows = el('div', { id: 'cache-rows' });
  private tab: 'pixel' | 'lamps' | 'cache' | 'levels' | 'dial' | 'mixer' | 'meter' = 'pixel';

  constructor(private lab: Lab, root: HTMLElement) {
    this.stage.append(lab.canvas, this.overlay, this.headers, this.marker, this.wipeLine, this.status);
    root.append(this.top, this.side, this.stage, this.right, this.code);
    this.listen();
    this.go(this.chapter.number);
    setInterval(() => void this.refresh(), 150);
  }

  go(number: number): void {
    const c = CHAPTERS.find((x) => x.number === number) ?? CHAPTERS[0]!;
    const { lab } = this;
    for (const key of Object.keys(this.chapter.sets ?? {})) setKnob(key, defaultOf(key));
    this.chapter = c;
    for (const [key, value] of Object.entries(c.sets ?? {})) setKnob(key, value);
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
        button('Freeze', 'The frame stands, and the truth goes on settling on it (F)', () => (lab.frozen = !lab.frozen), lab.frozen),
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
          if (s.live) lab.turnLive(s.key, Number(input.value));
          else this.openRoom(lab.world.room.id, { ...lab.world.sliders, [s.key]: Number(input.value) });
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
        c.actions ? el('div', { class: 'actions' }, ...c.actions.map(([text, action]) =>
          el('button', { type: 'button', click: () => this.act(action) }, text))) : null,
        section('Turn', c.knobs.map(control)),
        section('The room', roomRows),
        section('Break it', c.breaks.map(control), 'breaks'),
        section('Read and edit', c.stages.map((name) =>
          el('button', { type: 'button', class: 'stage-link', click: () => this.showCode(name) },
            el('b', {}, name), ' ', el('span', {}, lab.graph.stages.find((s) => s.name === name)?.question ?? '')))),
      ].filter((x): x is HTMLElement => x !== null),
    );
  }

  private act(action: string): void {
    const { cache } = this.lab;
    if (action === 'cache zero') cache.zero = true;
    if (action === 'cache round') cache.rounds = 1;
    if (action === 'cache rounds') cache.rounds = 32;
    this.history = [];
  }

  private drawRight(): void {
    const tabs: [typeof this.tab, string][] = [['pixel', 'Pixel'], ['lamps', 'Lamps'], ['cache', 'Cache'], ['levels', 'Levels'], ['dial', 'Dial'], ['mixer', 'Mixer'], ['meter', 'Meter']];
    const head = el('div', { class: 'segments', role: 'group' },
      ...tabs.map(([id, text]) =>
        el('button', { type: 'button', 'aria-pressed': String(this.tab === id), click: () => { this.tab = id; this.draw(); } }, text)));
    const body =
      this.tab === 'pixel' ? this.pinnedRows
      : this.tab === 'lamps' ? this.lampRows
      : this.tab === 'cache' ? this.cacheRows
      : this.tab === 'levels' ? this.levelRows
      : this.tab === 'dial' ? this.dialRows
      : this.tab === 'meter' ? this.meterRows
      : el('div', { id: 'mixer' }, ...MIXER.map((k) => this.mixerRow(k.key)));
    this.lab.alsoWanted = this.tab === 'lamps' ? ['bars', 'reservoir_still', 'reservoir_changing'] : this.tab === 'levels' ? ['strip']
      : this.tab === 'dial' && this.lab.meshes.jobCount ? DIAL_FORMS.map((f) => `dial ${f.id}`) : [];
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
        }, makeMenu(i), el('span', { class: 'verdict', 'data-view': lab.shown[i]! }));
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
    if (lab.pinned) {
      await lab.rays.read();
      await this.findPinnedHit();
      lab.pinnedAt = this.pinnedHit;
    }
    this.drawOverlay();
    if (this.tab === 'pixel') await this.refreshPinned();
    if (this.tab === 'lamps') await this.refreshLamps();
    if (this.tab === 'meter') this.refreshMeter();
    if (this.tab === 'cache' || valueOf('cache_entries')) await this.readEntries();
    if (this.tab === 'cache') this.refreshCache();
    if (this.tab === 'levels') await this.refreshLevels();
    if (this.tab === 'dial') await this.refreshDial();
    const judged = lab.judged();
    if (judged.length && lab.ready) {
      await lab.judge.read(judged);
      this.drawVerdicts();
    }
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
      if (valueOf('cache_entries') && lab.shown[i] !== 'disc') {
        const bright = 2 ** valueOf('exposure');
        for (const e of this.entries) {
          if (dot(e.normal, sub(eye.eye, e.position)) < 0) continue;
          const rgb = e.direct.map((d, c) => Math.round(255 * Math.min(1, Math.max(0, (d + e.known[c]!) * 0.8 * bright)) ** (1 / 2.2)));
          line(e.position, add(e.position, scale(e.normal, 0.06)), `rgb(${rgb.join(',')})`, 3);
        }
      }
      if (lab.shown[i] === 'disc') {
        this.drawDots(pen, r);
        pen.restore();
        return;
      }
      if (lab.pinned) lab.rays.lines.forEach((l) => line(l.start, l.end, RAY_KINDS[l.kind]?.[1] ?? '#fff', 1.25));
      pen.restore();
    });
  }

  /** where the pinned point's rays went, on its disc */
  private drawDots(pen: CanvasRenderingContext2D, r: { x: number; y: number; width: number; height: number }): void {
    const { lines } = this.lab.rays;
    const hit = this.pinnedHit;
    if (!hit) return;
    const [t, b] = axesAround(hit.normal);
    const radius = 0.48 * Math.min(r.width, r.height);
    for (const l of lines) {
      const name = RAY_KINDS[l.kind]?.[0];
      if (name === 'onward' || name === 'reused' || name === 'candidate') continue;
      const toward = sub(l.end, l.start);
      const size = Math.hypot(...toward);
      if (Math.hypot(...sub(l.start, hit.position)) > 0.03 || size < 1e-6) continue;
      pen.fillStyle = RAY_KINDS[l.kind]![1];
      pen.beginPath();
      pen.arc(
        r.x + r.width / 2 + (dot(toward, t) / size) * radius,
        r.y + r.height / 2 - (dot(toward, b) / size) * radius,
        3.5, 0, 2 * Math.PI);
      pen.fill();
      pen.strokeStyle = '#000';
      pen.stroke();
    }
  }

  private drawVerdicts(): void {
    const percent = (x: number) => (Number.isFinite(x) ? `${(100 * x).toFixed(1)} %` : '–');
    for (const node of this.headers.querySelectorAll<HTMLElement>('.verdict')) {
      const v = this.lab.judge.verdicts.get(node.dataset.view!);
      node.textContent = v && this.lab.truthCount < 64 ? 'the truth is still settling' : v ? `off ${percent(v.off)}   noise ${percent(v.noise)}` : '';
    }
  }

  // ---- the cache

  private entries: { slot: number; position: Vec3; normal: Vec3; direct: Vec3; known: Vec3; bounced: Vec3; samples: number; frame: number }[] = [];
  /** the pinned entry's gathered light, over time */
  private history: { round: number; light: number }[] = [];

  private async readEntries(): Promise<void> {
    const { lab } = this;
    if (!lab.made.includes('cache gather')) return;
    const points = lab.graph.spec('cache_points').struct;
    const entry = lab.graph.spec('cache').struct;
    const [p, e] = await Promise.all([
      readBack(lab.gpu, lab.graph.buffer('cache_points'), 0, CACHE_SLOTS * points.stride),
      readBack(lab.gpu, lab.graph.buffer('cache@before'), 0, CACHE_SLOTS * entry.stride),
    ]);
    this.entries = [];
    for (let slot = 0; slot < CACHE_SLOTS; slot++) {
      const at = slot * points.stride + points.offsets.normal!;
      if (p.getFloat32(at, true) === 0 && p.getFloat32(at + 4, true) === 0 && p.getFloat32(at + 8, true) === 0) continue;
      const a = unpack(points, p, slot * points.stride);
      const b = unpack(entry, e, slot * entry.stride);
      this.entries.push({
        slot, position: a.position as Vec3, normal: a.normal as Vec3, frame: a.frame as number,
        direct: b.direct as Vec3, known: b.known as Vec3, bounced: b.bounced as Vec3, samples: b.samples as number,
      });
    }
  }

  private refreshCache(): void {
    const { lab } = this;
    const full = this.entries.length / CACHE_SLOTS;
    const rows: HTMLElement[] = [
      el('p', { class: 'hint' }, `round ${lab.cache.round} · ${this.entries.length} entries, ${(100 * full).toFixed(1)} % of the table`),
    ];
    const hit = this.pinnedHit;
    if (!lab.pinned || !hit) {
      rows.push(el('p', { class: 'hint' }, 'Click a place to follow the entry that speaks for it.'));
    } else {
      // the nearest entry that faces the same way
      const near = this.entries
        .filter((e) => dot(e.normal, hit.normal) > 0.7)
        .map((e) => ({ e, d: Math.hypot(...sub(e.position, hit.position)) }))
        .sort((a, b) => a.d - b.d)[0];
      if (near) {
        const { e } = near;
        const light = 0.2126 * e.bounced[0] + 0.7152 * e.bounced[1] + 0.0722 * e.bounced[2];
        const last = this.history[this.history.length - 1];
        if (!last || last.round !== lab.cache.round) this.history.push({ round: lab.cache.round, light });
        if (this.history.length > 400) this.history.shift();
        const row = (name: string, v: Value, color = true) => [el('dt', {}, name), el('dd', {}, shown(v, color))];
        rows.push(
          el('h4', {}, `entry ${e.slot}`),
          el('dl', {}, ...row('from here', near.d, false), ...row('direct', e.direct), ...row('gathering', e.bounced), ...row('known', e.known), ...row('samples', e.samples, false)),
          el('h4', {}, 'what it gathers, over time'),
          this.chart(this.history.map((h) => h.light)));
      }
    }
    this.cacheRows.replaceChildren(...rows);
  }

  // ---- levels

  /** what one object blocks and sends back, by the width of the ray that asks */
  private async refreshLevels(): Promise<void> {
    const { lab } = this;
    if (!lab.world.room.subject) {
      this.levelRows.replaceChildren(el('p', { class: 'hint' }, 'This room has no object whose levels are measured. Try Levels or Stone yard.'));
      return;
    }
    if (!lab.made.includes('strip')) return;
    const spec = lab.graph.spec('strip');
    const count = lab.graph.elements(spec.size);
    const data = await readBack(lab.gpu, lab.graph.buffer('strip'), 0, count * spec.struct.stride);
    const per = count / STRIP_WIDTHS;
    const bins = Array.from({ length: STRIP_WIDTHS }, (_, bin) => {
      let blocks = 0;
      let sends = 0;
      let steps = 0;
      for (let k = 0; k < per; k++) {
        const v = unpack(spec.struct, data, (bin * per + k) * spec.struct.stride);
        const c = v.sends as number[];
        blocks += (v.blocks as number) / per;
        sends += (0.2126 * c[0]! + 0.7152 * c[1]! + 0.0722 * c[2]!) / per;
        steps += (v.steps as number) / per;
      }
      return { width: STRIP_MOST * 0.004 ** (1 - (bin + 0.5) / STRIP_WIDTHS), blocks, sends, steps };
    });
    const W = 240;
    const H = 110;
    const x = (i: number) => (i / (STRIP_WIDTHS - 1)) * W;
    const line = (values: number[], color: string) => {
      const base = values.slice(0, 6).reduce((a, b) => a + b, 0) / 6 || 1;
      const points = values.map((v, i) => `${x(i).toFixed(1)},${(H / 2 - ((v / base - 1) * H) / 1).toFixed(1)}`).join(' ');
      return `<polyline points="${points}" fill="none" stroke="${color}" stroke-width="1.6"/>`;
    };
    const ticks = [0.01, 0.1, 1].map((w) => {
      const i = (1 - Math.log(w / STRIP_MOST) / Math.log(0.004)) * STRIP_WIDTHS - 0.5;
      return `<line x1="${x(i)}" x2="${x(i)}" y1="0" y2="${H}" stroke="#2c2f34"/><text x="${x(i) + 3}" y="${H - 3}" fill="#9a9c98" font-size="9">${w} m</text>`;
    });
    const grid = [-0.1, 0, 0.1].map((v) => `<line x1="0" x2="${W}" y1="${H / 2 - v * H}" y2="${H / 2 - v * H}" stroke="${v ? '#24272b' : '#4a4e55'}"/>`);
    const chart = el('div', { class: 'chart' });
    chart.innerHTML = `<svg viewBox="0 0 ${W} ${H}" width="100%" role="img" aria-label="what it blocks and sends back, by ray width">${grid.join('')}${ticks.join('')}${line(bins.map((b) => b.blocks), '#6bb8ff')}${line(bins.map((b) => b.sends), '#ffe08a')}</svg>`;
    const row = (name: string, f: (b: (typeof bins)[0]) => number) =>
      el('tr', {}, el('td', {}, name), ...[4, 24, 34, 44].map((i) => el('td', {}, f(bins[i]!).toFixed(name === 'nodes' ? 1 : 3))));
    this.levelRows.replaceChildren(
      el('p', { class: 'hint' }, 'One object, asked from all sides by rays of each width, beside the finest (the middle line; faint lines 10 % off). Blue: what it blocks. Yellow: what it sends back. A good level keeps both flat.'),
      chart,
      el('table', {}, el('thead', {}, el('tr', {}, el('th', {}, ''), ...[4, 24, 34, 44].map((i) => el('th', {}, `${bins[i]!.width.toFixed(2)} m`)))),
        el('tbody', {}, row('blocks', (b) => b.blocks), row('sends', (b) => b.sends), row('nodes', (b) => b.steps))));
  }

  // ---- the dial

  private async refreshDial(): Promise<void> {
    const { lab } = this;
    if (!lab.meshes.jobCount) {
      this.dialRows.replaceChildren(el('p', { class: 'hint' }, 'Open the Ladder or the Stone yard: the dial asks the first mesh of a room.'));
      return;
    }
    const rays = DIAL_RAYS * valueOf('dial_repeat');
    const rows = await Promise.all(DIAL_FORMS.map(async (f) => {
      const ms = lab.graph.timer.ms.get(`dial ${f.id}`);
      if (!lab.made.includes(`dial ${f.id}`)) return null;
      const data = new Float32Array((await readBack(lab.gpu, lab.graph.buffer(`dial ${f.id}`), 0, 4096 * 8)).buffer);
      let steps = 0;
      for (let i = 0; i < 4096; i++) steps += data[i * 2 + 1]! / 4096 / valueOf('dial_repeat');
      return el('tr', { title: f.about }, el('td', {}, f.name), el('td', {}, ms === undefined ? '–' : ((ms * 1e6) / rays).toFixed(2)), el('td', {}, steps.toFixed(1)));
    }));
    this.dialRows.replaceChildren(
      el('p', { class: 'hint' }, `${(DIAL_RAYS / 1000).toFixed(0)} thousand rays through a ball round one stone, for each form, every frame.`),
      el('table', {}, el('thead', {}, el('tr', {}, el('th', {}, 'The stone is'), el('th', {}, 'ns a ray'), el('th', {}, 'steps'))), el('tbody', {}, ...rows.filter((r) => r !== null))),
      control('dial_coherent'), control('dial_repeat'),
      el('p', { class: 'hint' }, 'For a feel of where forms cross over. The times are of passes on this GPU, smoothed; with nothing else in the panels they are steadier.'));
  }

  /** a small line, from zero up */
  private chart(values: number[]): HTMLElement {
    const W = 220;
    const H = 70;
    const top = Math.max(...values, 1e-6) * 1.15;
    const line = values.map((v, i) => `${((i / Math.max(1, values.length - 1)) * W).toFixed(1)},${(H - (v / top) * H).toFixed(1)}`).join(' ');
    const box = el('div', { class: 'chart' });
    box.innerHTML = `<svg viewBox="0 0 ${W} ${H}" width="100%" role="img" aria-label="light over time"><polyline points="${line}" fill="none" stroke="#5fd4c4" stroke-width="1.5"/></svg><span>${(top / 1.15).toPrecision(3)}</span>`;
    return box;
  }

  private picks = new Map<number, number>();
  private picksKey = '';
  /** what each lamp would send to the pinned point, how much is seen, and how often it was picked */
  private async refreshLamps(): Promise<void> {
    const { lab } = this;
    if (!lab.pinned || !lab.made.includes('bars')) {
      this.lampRows.replaceChildren(el('p', { class: 'hint' }, 'Click a pixel to see what each lamp would send there.'));
      return;
    }
    const key = `${lab.pinned}${lab.world.version}${JSON.stringify(lab.camera)}`;
    if (key !== this.picksKey) this.picks.clear();
    this.picksKey = key;
    const spec = lab.graph.spec('bars');
    const data = await readBack(lab.gpu, lab.graph.buffer('bars'), 0, lab.world.lampCount * spec.struct.stride);
    for (const p of await lab.readPinned()) {
      if (!p.name.startsWith('reservoir_')) continue;
      const lamp = p.values.lamp as number;
      if (lamp < lab.world.lampCount) this.picks.set(lamp, (this.picks.get(lamp) ?? 0) + 1);
    }
    const picked = Math.max(1, [...this.picks.values()].reduce((a, b) => a + b, 0));
    const rows = Array.from({ length: lab.world.lampCount }, (_, i) => {
      const v = unpack(spec.struct, data, i * spec.struct.stride);
      const u = v.unblocked as number[];
      return { i, sends: 0.2126 * u[0]! + 0.7152 * u[1]! + 0.0722 * u[2]!, seen: v.seen as number, color: v.color as number[], share: (this.picks.get(i) ?? 0) / picked };
    }).filter((r) => r.sends > 0).sort((a, b) => b.sends - a.sends);
    const total = rows.reduce((sum, r) => sum + r.sends, 0);
    const bar = (share: number, color: string) => el('span', { class: 'bar', style: `width:${Math.min(100, 100 * share)}%;background:${color}` });
    const most = rows[0]?.sends ?? 1;
    this.lampRows.replaceChildren(
      el('p', { class: 'hint' }, `${rows.length} lamps can reach this point. Pale: what it would send. Solid: what is seen of it. Below: how often the ray asked it.`),
      ...rows.slice(0, 24).map((r) => {
        const top = Math.max(...r.color, 1e-6);
        const rgb = `rgb(${r.color.map((c) => Math.round(255 * Math.pow(c / top, 1 / 2.2))).join(',')})`;
        return el('div', { class: 'lamp', title: `sends ${((100 * r.sends) / total).toFixed(1)} % of all, ${(100 * r.seen).toFixed(0)} % seen, picked ${(100 * r.share).toFixed(1)} %` },
          el('span', { class: 'lamp-name' }, String(r.i)),
          el('span', { class: 'bars' },
            el('span', { class: 'track' }, bar(r.sends / most, `color-mix(in srgb, ${rgb} 30%, transparent)`), bar((r.sends / most) * r.seen, rgb)),
            el('span', { class: 'track thin' }, bar(r.share, '#9a9c98'))));
      }));
  }

  private pinnedHit: { position: Vec3; normal: Vec3 } | null = null;
  private async findPinnedHit(): Promise<void> {
    const { lab } = this;
    if (!lab.pinned || !lab.graph.grids.render[0]) return;
    const spec = lab.graph.spec('hit_x');
    const [w, h] = lab.graph.grids.render;
    const index = Math.floor(lab.pinned[1] * h) * w + Math.floor(lab.pinned[0] * w);
    const v = unpack(spec.struct, await readBack(lab.gpu, lab.graph.buffer('hit_x'), index * spec.struct.stride, spec.struct.stride));
    this.pinnedHit = { position: v.position as Vec3, normal: v.normal as Vec3 };
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
    const [w, h] = graph.grids.render;
    const rows = this.lab.made.filter((n) => !n.startsWith('view ')).map((name) => {
      const stage = graph.stages.find((s) => s.name === name)!;
      const writes = Array.isArray(stage.writes) ? stage.writes : Object.values(stage.writes);
      const bytes = writes.reduce((sum, x) => sum + graph.bytesOf(x), 0);
      const ms = graph.timer.ms.get(name);
      return el('tr', {}, el('td', {}, name), el('td', {}, ms === undefined ? '–' : ms.toFixed(2)), el('td', {}, (bytes / 1e6).toFixed(1)));
    });
    this.meterRows.replaceChildren(
      el('p', { class: 'hint' }, `${gpu.about || 'GPU'} · render ${w} × ${h}, output ${graph.grids.output.join(' × ')}`),
      el('table', {}, el('thead', {}, el('tr', {}, el('th', {}, 'Stage'), el('th', {}, 'ms'), el('th', {}, 'MB'))), el('tbody', {}, ...rows)),
      el('p', { class: 'hint' }, `Making the tree of what moves: ${this.lab.world.treeMs.toFixed(2)} ms on the CPU.`),
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
