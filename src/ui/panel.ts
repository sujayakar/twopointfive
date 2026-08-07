// Tweak panel (Tab): collapsible sections of sliders / toggles / buttons bound to live settings.
type Getter<T> = () => T; type Setter<T> = (v: T) => void;

export class Panel {
  root = document.getElementById('panel')!;
  private refreshers: (() => void)[] = [];
  private sections = new Map<string, HTMLElement>();
  visible = false;

  constructor() {
    setInterval(() => { if (this.visible) this.refresh(); }, 400);
  }
  toggle() { this.visible = !this.visible; this.root.classList.toggle('hidden', !this.visible); if (this.visible) this.refresh(); }
  refresh() { for (const r of this.refreshers) r(); }
  /** Drop refreshers whose controls were removed from the DOM (sections rebuilt by their owner, e.g. the viewer's selection box). */
  prune() { this.refreshers = this.refreshers.filter(r => !(r as { el?: HTMLElement }).el || document.contains((r as { el?: HTMLElement }).el!)); }

  section(name: string, open = true): HTMLElement {
    let s = this.sections.get(name); if (s) return s;
    const h = document.createElement('h3'); h.textContent = (open ? '▾ ' : '▸ ') + name;
    const body = document.createElement('div'); body.style.display = open ? 'block' : 'none';
    h.onclick = () => { const o = body.style.display === 'none'; body.style.display = o ? 'block' : 'none'; h.textContent = (o ? '▾ ' : '▸ ') + name; };
    this.root.appendChild(h); this.root.appendChild(body); this.sections.set(name, body); return body;
  }
  private row(sec: HTMLElement, label: string, title?: string): HTMLElement {
    const r = document.createElement('div'); r.className = 'row'; const l = document.createElement('label'); l.textContent = label; if (title) { l.title = title; r.title = title; } r.appendChild(l); sec.appendChild(r); return r;
  }
  slider(sec: HTMLElement, label: string, min: number, max: number, step: number, get: Getter<number>, set: Setter<number>, title?: string) {
    const r = this.row(sec, label, title); const inp = document.createElement('input'); inp.type = 'range'; inp.min = String(min); inp.max = String(max); inp.step = String(step);
    const val = document.createElement('span'); val.className = 'val';
    const fmt = (v: number) => (Math.abs(step) >= 1 ? v.toFixed(0) : Math.abs(step) >= 0.1 ? v.toFixed(1) : Math.abs(step) >= 0.01 ? v.toFixed(2) : v.toFixed(3));
    inp.oninput = () => { const v = parseFloat(inp.value); set(v); val.textContent = fmt(v); };
    const rf = () => { if (document.activeElement !== inp) { const v = get(); inp.value = String(v); val.textContent = fmt(v); } }; (rf as { el?: HTMLElement }).el = inp;
    rf(); this.refreshers.push(rf); r.appendChild(inp); r.appendChild(val); return inp;
  }
  toggleBox(sec: HTMLElement, label: string, get: Getter<boolean>, set: Setter<boolean>, title?: string) {
    const r = this.row(sec, label, title); const inp = document.createElement('input'); inp.type = 'checkbox';
    inp.onchange = () => set(inp.checked); const rf = () => { inp.checked = get(); }; rf(); this.refreshers.push(rf); r.appendChild(inp); return inp;
  }
  select(sec: HTMLElement, label: string, options: string[], get: Getter<number>, set: Setter<number>, title?: string) {
    const r = this.row(sec, label); const s = document.createElement('select'); if (title) r.title = title;
    options.forEach((o, i) => { const op = document.createElement('option'); op.value = String(i); op.textContent = o; s.appendChild(op); });
    s.onchange = () => set(parseInt(s.value, 10)); const rf = () => { if (document.activeElement !== s) s.value = String(get()); }; rf(); this.refreshers.push(rf); r.appendChild(s); return s;
  }
  buttons(sec: HTMLElement, label: string, btns: { text: string; run: () => void; title?: string }[]) {
    const r = this.row(sec, label);
    for (const b of btns) { const el = document.createElement('button'); el.textContent = b.text; if (b.title) el.title = b.title; el.onclick = () => { b.run(); this.refresh(); }; r.appendChild(el); }
  }
  text(sec: HTMLElement, get: Getter<string>) {
    const r = document.createElement('div'); r.className = 'row'; r.style.color = '#8aa'; r.style.whiteSpace = 'pre-wrap'; sec.appendChild(r);
    const rf = () => { r.textContent = get(); }; rf(); this.refreshers.push(rf);
  }
}
