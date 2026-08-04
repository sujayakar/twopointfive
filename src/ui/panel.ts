// ---------------------------------------------------------------------------
// A dependency-free debug panel.
//
// Every renderer parameter in this project interacts with every other one —
// exposure against ambient against sky against bloom threshold — so tuning them
// one edit-reload cycle at a time is hopeless. This binds them to live controls.
// ---------------------------------------------------------------------------

export interface SliderSpec {
  kind: "slider";
  label: string;
  min: number;
  max: number;
  step: number;
  get(): number;
  set(v: number): void;
  /** Called after set(), e.g. to reallocate render targets. */
  onChange?(): void;
}

export interface ToggleSpec {
  kind: "toggle";
  label: string;
  get(): boolean;
  set(v: boolean): void;
  onChange?(): void;
}

export interface SelectSpec {
  kind: "select";
  label: string;
  /** Option labels; the bound value is the index. */
  options: string[];
  get(): number;
  set(v: number): void;
  onChange?(): void;
}

export type ControlSpec = SliderSpec | ToggleSpec | SelectSpec;

export interface GroupSpec {
  title: string;
  collapsed?: boolean;
  /**
   * Hides the whole group when it returns false. Re-evaluated by refresh().
   *
   * For panels whose controls only apply to one mode: showing every mode's
   * sliders at once means most of what is on screen does nothing, and there is
   * no way to tell which. Absent, the group is always shown.
   */
  show?: () => boolean;
  items: ControlSpec[];
}

const CSS = `
#tweak {
  position: fixed; top: 0; right: 0; bottom: 0; width: 290px;
  background: rgba(8,10,14,0.9); backdrop-filter: blur(8px);
  border-left: 1px solid rgba(255,255,255,0.09);
  font: 11px/1.4 ui-monospace, "SF Mono", Menlo, monospace;
  color: #c9d2e0; overflow-y: auto; padding: 10px 12px 40px;
  z-index: 10; display: none;
}
#tweak.open { display: block; }
#tweak h2 {
  font-size: 10px; font-weight: 600; letter-spacing: 0.09em;
  text-transform: uppercase; color: #7fd1c1;
  margin: 14px 0 7px; padding-bottom: 4px;
  border-bottom: 1px solid rgba(127,209,193,0.18); cursor: pointer;
  user-select: none;
}
#tweak h2:first-child { margin-top: 2px; }
#tweak h2::before { content: "▾ "; opacity: 0.6; }
#tweak h2.closed::before { content: "▸ "; }
#tweak .body.closed { display: none; }
#tweak .row { margin: 5px 0; }
#tweak .lab {
  display: flex; justify-content: space-between; gap: 8px;
  opacity: 0.82; margin-bottom: 2px;
}
#tweak .val { color: #e8c98a; font-variant-numeric: tabular-nums; }
#tweak input[type=range] {
  width: 100%; height: 14px; margin: 0; appearance: none; background: transparent;
}
#tweak input[type=range]::-webkit-slider-runnable-track {
  height: 3px; background: rgba(255,255,255,0.16); border-radius: 2px;
}
#tweak input[type=range]::-webkit-slider-thumb {
  appearance: none; width: 11px; height: 11px; margin-top: -4px;
  border-radius: 50%; background: #7fd1c1; cursor: pointer;
}
#tweak label.tog {
  display: flex; align-items: center; gap: 7px; cursor: pointer;
  padding: 3px 0; opacity: 0.9;
}
#tweak .hint {
  margin-top: 16px; padding-top: 8px; opacity: 0.4; font-size: 10px;
  border-top: 1px solid rgba(255,255,255,0.08); line-height: 1.6;
}
#tweak button {
  font: inherit; color: #c9d2e0; background: rgba(255,255,255,0.07);
  border: 1px solid rgba(255,255,255,0.14); border-radius: 4px;
  padding: 4px 9px; cursor: pointer; margin-top: 10px;
}
#tweak button:hover { background: rgba(255,255,255,0.13); }
#tweak select {
  width: 100%; margin-top: 3px; padding: 3px 4px;
  background: rgba(255,255,255,0.07); color: inherit; font: inherit;
  border: 1px solid rgba(255,255,255,0.14); border-radius: 3px;
}
#tweak select:focus { outline: 1px solid rgba(255,255,255,0.3); }
`;

export class TweakPanel {
  private el: HTMLDivElement;
  private refreshers: Array<() => void> = [];

  constructor(groups: GroupSpec[], onReset?: () => void) {
    const style = document.createElement("style");
    style.textContent = CSS;
    document.head.appendChild(style);

    this.el = document.createElement("div");
    this.el.id = "tweak";

    for (const g of groups) {
      const h = document.createElement("h2");
      h.textContent = g.title;
      const body = document.createElement("div");
      body.className = "body";
      if (g.collapsed) {
        h.classList.add("closed");
        body.classList.add("closed");
      }
      h.addEventListener("click", () => {
        h.classList.toggle("closed");
        body.classList.toggle("closed");
      });
      this.el.append(h, body);
      if (g.show) this.gated.push({ show: g.show, h, body });
      for (const item of g.items) {
        body.appendChild(
          item.kind === "slider" ? this.slider(item)
            : item.kind === "select" ? this.select(item)
            : this.toggle(item),
        );
      }
    }

    if (onReset) {
      const btn = document.createElement("button");
      btn.textContent = "reset to defaults";
      btn.addEventListener("click", () => {
        onReset();
        this.refresh();
      });
      this.el.appendChild(btn);
    }

    const hint = document.createElement("div");
    hint.className = "hint";
    hint.textContent = "` toggles this panel\nclick a heading to fold it";
    hint.style.whiteSpace = "pre";
    this.el.appendChild(hint);

    document.body.appendChild(this.el);
  }

  private slider(s: SliderSpec): HTMLElement {
    const row = document.createElement("div");
    row.className = "row";
    const lab = document.createElement("div");
    lab.className = "lab";
    const name = document.createElement("span");
    name.textContent = s.label;
    const val = document.createElement("span");
    val.className = "val";
    const input = document.createElement("input");
    input.type = "range";
    input.min = String(s.min);
    input.max = String(s.max);
    input.step = String(s.step);

    // Show only as many decimals as the step actually resolves.
    const decimals = Math.max(0, -Math.floor(Math.log10(s.step)));
    const sync = () => {
      const v = s.get();
      input.value = String(v);
      val.textContent = v.toFixed(decimals);
    };
    input.addEventListener("input", () => {
      s.set(parseFloat(input.value));
      val.textContent = parseFloat(input.value).toFixed(decimals);
      s.onChange?.();
    });
    sync();
    this.refreshers.push(sync);

    lab.append(name, val);
    row.append(lab, input);
    return row;
  }

  private toggle(t: ToggleSpec): HTMLElement {
    const label = document.createElement("label");
    label.className = "tog";
    const box = document.createElement("input");
    box.type = "checkbox";
    const span = document.createElement("span");
    span.textContent = t.label;
    const sync = () => { box.checked = t.get(); };
    box.addEventListener("change", () => {
      t.set(box.checked);
      t.onChange?.();
    });
    sync();
    this.refreshers.push(sync);
    label.append(box, span);
    return label;
  }

  private select(sp: SelectSpec): HTMLElement {
    const row = document.createElement("div");
    row.className = "row";
    const lab = document.createElement("div");
    lab.className = "lab";
    const name = document.createElement("span");
    name.textContent = sp.label;
    lab.appendChild(name);

    const sel = document.createElement("select");
    sp.options.forEach((o, i) => {
      const opt = document.createElement("option");
      opt.value = String(i);
      opt.textContent = o;
      sel.appendChild(opt);
    });
    const sync = () => { sel.value = String(sp.get()); };
    sel.addEventListener("change", () => {
      sp.set(parseInt(sel.value, 10));
      sp.onChange?.();
    });
    sync();
    this.refreshers.push(sync);

    row.append(lab, sel);
    return row;
  }

  /** Groups with a `show` predicate, re-evaluated on every refresh. */
  private readonly gated: {
    show: () => boolean; h: HTMLElement; body: HTMLElement;
  }[] = [];

  /** Pull every control back in sync with its source of truth. */
  refresh(): void {
    for (const r of this.refreshers) r();
    for (const g of this.gated) {
      const on = g.show();
      g.h.style.display = on ? "" : "none";
      g.body.style.display = on ? "" : "none";
    }
  }

  toggleVisible(): void {
    this.el.classList.toggle("open");
  }

  get visible(): boolean {
    return this.el.classList.contains("open");
  }
}
