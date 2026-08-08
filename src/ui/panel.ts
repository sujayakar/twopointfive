// Tweak panel (Tab): collapsible sections of sliders / toggles / buttons bound to live settings.
type Getter<T> = () => T; type Setter<T> = (v: T) => void;

export class Panel {
  root = document.getElementById('panel')!;
  private refreshers: (() => void)[] = [];
  private sections = new Map<string, HTMLElement>();
  private heads = new Map<string, HTMLElement>();
  private advanced = new Set<string>();          // sections that only show with "all knobs" on (or while searching)
  private body: HTMLElement; private search: HTMLInputElement; private allBox: HTMLInputElement;
  private open: Set<string>; private showAll: boolean;
  visible = false;

  constructor() {
    // header: a search box that filters every row by its label / tooltip across all sections, and an "all knobs" switch; the rarely
    // used sections are registered as advanced and stay out of the way until asked for. Open/closed state and the switch persist.
    this.open = new Set(JSON.parse(localStorage.getItem('twopointsix.panelOpen') || '["Frame","Look","Sandbox"]'));
    this.showAll = localStorage.getItem('twopointsix.panelAll') === '1';
    const head = document.createElement('div'); head.className = 'phead';
    this.search = document.createElement('input'); this.search.type = 'search'; this.search.placeholder = 'search every knob…'; this.search.spellcheck = false;
    this.search.oninput = () => this.applyFilter();
    this.search.onkeydown = e => { if (e.key === 'Escape') { this.search.value = ''; this.applyFilter(); this.search.blur(); } e.stopPropagation(); };   // typing here must not drive the game
    const lab = document.createElement('label'); lab.className = 'all'; this.allBox = document.createElement('input'); this.allBox.type = 'checkbox'; this.allBox.checked = this.showAll;
    this.allBox.onchange = () => { this.showAll = this.allBox.checked; localStorage.setItem('twopointsix.panelAll', this.showAll ? '1' : '0'); this.applyFilter(); };
    lab.append(this.allBox, document.createTextNode(' all knobs'));
    head.append(this.search, lab); this.root.appendChild(head);
    this.body = document.createElement('div'); this.body.className = 'pbody'; this.root.appendChild(this.body);
    setInterval(() => { if (this.visible) this.refresh(); }, 400);
  }
  toggle() { this.visible = !this.visible; this.root.classList.toggle('hidden', !this.visible); document.body.classList.toggle('panel-open', this.visible); if (this.visible) { this.refresh(); this.applyFilter(); } }
  refresh() { for (const r of this.refreshers) r(); }
  /** Drop refreshers whose controls were removed from the DOM (sections rebuilt by their owner, e.g. the viewer's selection box). */
  prune() { this.refreshers = this.refreshers.filter(r => !(r as { el?: HTMLElement }).el || document.contains((r as { el?: HTMLElement }).el!)); }
  /** True while the search box has focus (main.ts: let Esc / keys go to it, not the game). */
  get typing() { return document.activeElement === this.search; }

  /** `open` is only the first-run default now (the user's own open/closed choice persists); `advanced` sections hide unless "all knobs" is on or a search matches inside them. */
  section(name: string, open = true, advanced = false): HTMLElement {
    let s = this.sections.get(name); if (s) return s;
    if (advanced) this.advanced.add(name);
    if (!localStorage.getItem('twopointsix.panelOpen') && open) this.open.add(name);
    const h = document.createElement('h3'); const body = document.createElement('div');
    const paint = () => { const o = this.open.has(name); body.style.display = o ? 'block' : 'none'; h.textContent = (o ? '▾ ' : '▸ ') + name; };
    h.onclick = () => { if (this.open.has(name)) this.open.delete(name); else this.open.add(name); localStorage.setItem('twopointsix.panelOpen', JSON.stringify([...this.open])); paint(); };
    paint(); this.body.appendChild(h); this.body.appendChild(body); this.sections.set(name, body); this.heads.set(name, h); return body;
  }
  /** Search: show only rows whose label / tooltip contain every typed word (sections with hits are forced open, empty ones vanish); no query: sections per their open state, advanced ones hidden unless "all knobs". */
  private applyFilter() {
    const q = this.search.value.trim().toLowerCase(); const words = q ? q.split(/\s+/) : [];
    for (const [name, body] of this.sections) {
      const h = this.heads.get(name)!; let hits = 0;
      for (const r of Array.from(body.children) as HTMLElement[]) {
        const hay = ((r.querySelector('label')?.textContent ?? r.textContent ?? '') + ' ' + (r.title ?? '') + ' ' + name).toLowerCase();
        const show = !words.length || words.every(w => hay.includes(w)); r.style.display = show ? '' : 'none'; if (show) hits++;
      }
      const secVisible = words.length ? hits > 0 : (this.showAll || !this.advanced.has(name));
      h.style.display = secVisible ? '' : 'none';
      body.style.display = !secVisible ? 'none' : words.length ? 'block' : (this.open.has(name) ? 'block' : 'none');
    }
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
