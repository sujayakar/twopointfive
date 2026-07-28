// ---------------------------------------------------------------------------
// Brightness, and a calibration pass for it.
//
// This game is almost entirely dark by design, which makes it the one setting
// a player genuinely cannot be left to guess at: a value dialled in at night
// is unplayable in a lit room, and the failure mode is an apparently black
// screen rather than an obviously wrong setting.
//
// The calibration deliberately runs against the *live* render rather than
// against DOM swatches. Exposure feeds an AgX tonemap curve, so a CSS grey is
// not the same grey the pipeline would produce, and calibrating against one
// would be calibrating against the wrong thing.
// ---------------------------------------------------------------------------

const STORE_KEY = "twopointfive.exposure";

/** Matches the slider in the debug panel; see RenderSettings.exposure. */
export const EXPOSURE_MIN = 0.02;
export const EXPOSURE_MAX = 0.5;

const CSS = `
#bright {
  position: fixed; right: 18px; bottom: 34px; width: 190px;
  font: 10px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.14em; color: rgba(190,205,200,0.55);
  user-select: none;
}
#bright .row { display: flex; justify-content: space-between; align-items: baseline; }
#bright .val { font-variant-numeric: tabular-nums; color: rgba(225,232,230,0.85); }
#bright input[type="range"] {
  -webkit-appearance: none; appearance: none;
  width: 100%; height: 14px; margin-top: 4px; background: transparent;
  cursor: pointer; pointer-events: auto;
}
#bright input[type="range"]::-webkit-slider-runnable-track {
  height: 2px; border-radius: 1px;
  background: linear-gradient(to right, rgba(255,255,255,0.1), rgba(255,244,214,0.75));
}
#bright input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none; appearance: none;
  width: 10px; height: 10px; margin-top: -4px; border-radius: 50%;
  background: rgba(240,244,242,0.95);
}
#bright input[type="range"]:focus-visible { outline: 1px solid rgba(255,255,255,0.45); }
#bright .link {
  margin-top: 5px; font-size: 9px; opacity: 0.5;
  cursor: pointer; pointer-events: auto; text-decoration: underline;
}
#bright .link:hover { opacity: 0.85; }

/* The overlay dims the surrounding UI but leaves the render visible, because
   the render is the thing being judged. */
#calib {
  position: fixed; inset: 0; display: none;
  align-items: flex-end; justify-content: center;
  background: rgba(4,5,8,0.55);
  font: 12px/1.6 ui-monospace, SFMono-Regular, Menlo, monospace;
  color: rgba(226,234,231,0.9); letter-spacing: 0.08em;
  z-index: 50;
}
#calib.show { display: flex; }
#calib .panel {
  width: min(560px, 88vw); margin-bottom: 12vh; padding: 22px 26px;
  background: rgba(8,10,14,0.92);
  box-shadow: inset 0 0 0 1px rgba(255,255,255,0.09);
  border-radius: 3px; text-align: center; pointer-events: auto;
}
#calib h2 {
  margin: 0 0 10px; font-size: 13px; font-weight: 600; letter-spacing: 0.18em;
}
#calib p { margin: 0 0 16px; opacity: 0.75; font-size: 11px; }
#calib .big { width: 100%; }
#calib .num {
  margin-top: 8px; font-size: 15px; font-variant-numeric: tabular-nums;
}
#calib button {
  margin-top: 18px; padding: 8px 22px; font: inherit; letter-spacing: 0.16em;
  color: inherit; cursor: pointer; border-radius: 2px;
  background: rgba(255,255,255,0.09);
  border: 1px solid rgba(255,255,255,0.16);
}
#calib button:hover { background: rgba(255,255,255,0.15); }
#calib button:focus-visible { outline: 2px solid rgba(255,255,255,0.5); }
`;

export class Brightness {
  private readonly slider: HTMLInputElement;
  private readonly bigSlider: HTMLInputElement;
  private readonly readout: HTMLSpanElement;
  private readonly bigReadout: HTMLDivElement;
  private readonly overlay: HTMLDivElement;

  /**
   * @param initial  used only when nothing has been stored yet
   * @param onChange pushed the new exposure on every move
   */
  constructor(initial: number, private onChange: (v: number) => void) {
    const style = document.createElement("style");
    style.textContent = CSS;
    document.head.appendChild(style);

    const stored = Number(localStorage.getItem(STORE_KEY));
    const start = Number.isFinite(stored) && stored > 0 ? stored : initial;

    const el = document.createElement("div");
    el.id = "bright";
    const row = document.createElement("div");
    row.className = "row";
    const label = document.createElement("span");
    label.textContent = "BRIGHTNESS";
    this.readout = document.createElement("span");
    this.readout.className = "val";
    row.append(label, this.readout);
    this.slider = this.makeSlider(start);
    const link = document.createElement("div");
    link.className = "link";
    link.textContent = "calibrate";
    link.addEventListener("click", () => this.open());
    el.append(row, this.slider, link);
    document.body.appendChild(el);

    // ---- calibration overlay ---------------------------------------------
    this.overlay = document.createElement("div");
    this.overlay.id = "calib";
    const panel = document.createElement("div");
    panel.className = "panel";
    const h = document.createElement("h2");
    h.textContent = "CALIBRATE BRIGHTNESS";
    const p = document.createElement("p");
    // Naming a concrete target beats "adjust to taste": the player needs to
    // know what correct looks like, and the walls are the darkest large surface
    // that still has to stay readable.
    p.textContent =
      "Raise until you can just make out the corridor walls behind this panel, " +
      "then stop. Lower is better — most of this game happens in the dark.";
    this.bigSlider = this.makeSlider(start);
    this.bigSlider.className = "big";
    this.bigReadout = document.createElement("div");
    this.bigReadout.className = "num";
    const done = document.createElement("button");
    done.textContent = "DONE";
    done.addEventListener("click", () => this.close());
    panel.append(h, p, this.bigSlider, this.bigReadout, done);
    this.overlay.appendChild(panel);
    document.body.appendChild(this.overlay);

    this.apply(start);
    // First run only. Returning players keep what they chose, and nobody has to
    // dismiss a settings screen every time they load the page.
    if (!localStorage.getItem(STORE_KEY)) this.open();
  }

  private makeSlider(value: number): HTMLInputElement {
    const s = document.createElement("input");
    s.type = "range";
    s.min = String(EXPOSURE_MIN);
    s.max = String(EXPOSURE_MAX);
    s.step = "0.005";
    s.value = String(value);
    s.addEventListener("input", () => this.apply(parseFloat(s.value)));
    return s;
  }

  private apply(v: number): void {
    this.slider.value = String(v);
    this.bigSlider.value = String(v);
    const text = v.toFixed(3);
    this.readout.textContent = text;
    this.bigReadout.textContent = text;
    localStorage.setItem(STORE_KEY, String(v));
    this.onChange(v);
  }

  open(): void {
    this.overlay.classList.add("show");
  }

  close(): void {
    this.overlay.classList.remove("show");
  }

  get isOpen(): boolean {
    return this.overlay.classList.contains("show");
  }

  /** Keeps the control in step when something else changes exposure. */
  sync(v: number): void {
    if (Math.abs(parseFloat(this.slider.value) - v) < 1e-4) return;
    this.slider.value = String(v);
    this.bigSlider.value = String(v);
    const text = v.toFixed(3);
    this.readout.textContent = text;
    this.bigReadout.textContent = text;
  }
}
