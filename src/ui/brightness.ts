// ---------------------------------------------------------------------------
// Brightness, and a calibration pass for it.
//
// This game is almost entirely dark by design, which makes it the one setting
// a player genuinely cannot be left to guess at: a value dialled in at night
// is unplayable in a lit room, and the failure mode is an apparently black
// screen rather than an obviously wrong setting.
//
// Calibration is the familiar ladder of near-black patches with one marked
// "only just visible". The patches are not arbitrary CSS greys: each one is a
// scene luminance pushed through the same AgX curve the renderer uses, at the
// currently selected exposure. So a patch shows exactly the pixel the pipeline
// would put on screen for a surface that dark, and the ladder is calibrating
// against the real transfer function rather than an unrelated one.
// ---------------------------------------------------------------------------

/**
 * Temporary: where the bias slider parks its value between reloads.
 *
 * Deliberately outside the settings blob. This is a tool for choosing
 * LADDER_BIAS_EV, and it goes away with the slider once the number is baked.
 */
const BIAS_KEY = "twopointfive.ladderBias";

// --- AgX, mirroring agx() in post.wgsl -------------------------------------
// Kept in step with the shader by hand. If the tonemap there changes, the
// patches stop predicting the render and the calibration silently lies.

const AGX_IN = [
  [0.842479062253094, 0.0423282422610123, 0.0423756549057051],
  [0.0784335999999992, 0.878468636469772, 0.0784336],
  [0.0792237451477643, 0.0791661274605434, 0.879142973793104],
];
const AGX_OUT = [
  [1.19687900512017, -0.0528968517574562, -0.0529716355144438],
  [-0.0980208811401368, 1.15190312990417, -0.0980434501171241],
  [-0.0990297440797205, -0.0989611768448433, 1.15107367264116],
];

/** WGSL's mat3x3f takes columns, so index [col][row]. */
function mul3(m: number[][], v: number[]): number[] {
  return [0, 1, 2].map((r) => m[0][r] * v[0] + m[1][r] * v[1] + m[2][r] * v[2]);
}

function agxContrast(x: number): number {
  const x2 = x * x, x4 = x2 * x2;
  return 15.5 * x4 * x2 - 40.14 * x4 * x + 31.96 * x4 - 6.868 * x2 * x
    + 0.4298 * x2 + 0.1191 * x - 0.00232;
}

/** Scene luminance and exposure in, 0..255 display byte out. */
function agxByte(lum: number, exposure: number): number {
  const minEv = -12.47393, maxEv = 4.026069;
  const c = [lum * exposure, lum * exposure, lum * exposure];
  let v = mul3(AGX_IN, c.map((x) => Math.max(x, 0)));
  v = v.map((x) => Math.min(Math.max(Math.log2(Math.max(x, 1e-10)), minEv), maxEv))
    .map((x) => agxContrast((x - minEv) / (maxEv - minEv)));
  const o = mul3(AGX_OUT, v).map((x) => Math.max(x, 0));
  // The swap chain is a plain 8-bit unorm, not an _srgb format, so what the
  // tonemapper writes is what the display gets — and a CSS byte of the same
  // value lands on exactly the same pixel.
  return Math.round(Math.min(1, o[0]) * 255);
}

/**
 * Scene luminances for the ladder, just under a factor of two apart.
 *
 * These set the *spacing*; LADDER_BIAS_EV sets where the whole ladder sits.
 * The two darkest are meant to stay black across the slider's whole range,
 * which is what makes them the "if you can see this, something is wrong" end.
 */
const PATCHES = [
  0.000394, 0.000769, 0.0015, 0.002925, 0.005704, 0.011122, 0.021689, 0.042293,
];

/** The one that should sit right at the threshold. */
const TARGET_PATCH = 2;

/**
 * Stops to shift the whole ladder by.
 *
 * The first cut anchored the target patch at the exposure where its output
 * first becomes non-zero — one part in 255. That is the wrong threshold. A
 * single code value above black is not "only just visible" on a real display
 * in a lit room; it is invisible, which is what made the ladder read as
 * miscalibrated even at an exposure that looked right in play.
 *
 * So the anchor is measured rather than derived, using the bias slider on the
 * calibration screen. Set by hand; see BIAS_KEY.
 */
const LADDER_BIAS_EV = 0;

/** Shows the bias slider on the calibration screen. Remove once baked. */
const BIAS_TUNING = true;

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
.exslider {
  -webkit-appearance: none; appearance: none;
  width: 100%; height: 14px; margin-top: 4px; background: transparent;
  cursor: pointer; pointer-events: auto;
}
.exslider::-webkit-slider-runnable-track {
  height: 2px; border-radius: 1px;
  background: linear-gradient(to right, rgba(255,255,255,0.1), rgba(255,244,214,0.75));
}
.exslider::-webkit-slider-thumb {
  -webkit-appearance: none; appearance: none;
  width: 10px; height: 10px; margin-top: -4px; border-radius: 50%;
  background: rgba(240,244,242,0.95);
}
.exslider:focus-visible { outline: 1px solid rgba(255,255,255,0.35); }
#bright .link {
  margin-top: 5px; font-size: 9px; opacity: 0.5;
  cursor: pointer; pointer-events: auto; text-decoration: underline;
}
#bright .link:hover { opacity: 0.85; }

/* Opaque, and actually black. A patch at 2/255 cannot be judged against a
   translucent backdrop with the render glowing through it, and any bright
   chrome nearby raises the eye's adaptation and hides the very thing being
   looked for. Everything here is deliberately dim for that reason. */
#calib {
  position: fixed; inset: 0; display: none;
  align-items: center; justify-content: center;
  background: #000;
  font: 12px/1.6 ui-monospace, SFMono-Regular, Menlo, monospace;
  color: rgba(226,234,231,0.62); letter-spacing: 0.08em;
  z-index: 50;
}
#calib.show { display: flex; }
#calib .panel {
  width: min(560px, 88vw); padding: 0 26px;
  text-align: center; pointer-events: auto;
}
#calib h2 {
  margin: 0 0 10px; font-size: 13px; font-weight: 600; letter-spacing: 0.18em;
  color: rgba(226,234,231,0.7);
}
#calib p { margin: 0 0 30px; opacity: 0.6; font-size: 11px; }
#calib .big { width: 100%; }
#calib .exslider::-webkit-slider-runnable-track {
  background: linear-gradient(to right, rgba(255,255,255,0.06), rgba(255,244,214,0.34));
}
#calib .exslider::-webkit-slider-thumb { background: rgba(200,206,204,0.6); }
#calib .num {
  margin-top: 8px; font-size: 15px; font-variant-numeric: tabular-nums;
  color: rgba(226,234,231,0.6);
}

/* --- the patch ladder --- */
#calib .ladder { display: flex; gap: 6px; justify-content: center; }
#calib .patch { width: 52px; }
#calib .patch .sw {
  height: 62px; border-radius: 2px;
  /* No border: an outline at any visible brightness would be a reference the
     eye latches onto, and the whole point is judging the fill against black. */
}
#calib .patch .n {
  margin-top: 7px; font-size: 9px; opacity: 0.34;
  font-variant-numeric: tabular-nums;
}
#calib .patch.target .n { opacity: 0.75; }
/* Marks the target without putting anything bright alongside the patch. */
#calib .patch .tick { height: 9px; font-size: 9px; opacity: 0; }
#calib .patch.target .tick { opacity: 0.6; }
#calib .ladderNote { margin: 14px 0 26px; font-size: 10px; opacity: 0.45; }

/* Temporary tuning row; goes with BIAS_TUNING. */
#calib .biasRow { margin-top: 42px; opacity: 0.5; }
#calib .biasLabel { font-size: 9px; letter-spacing: 0.14em; opacity: 0.7; }
#calib .biasVal {
  margin-top: 5px; font-size: 10px; font-variant-numeric: tabular-nums;
  letter-spacing: 0.1em;
}
#calib button {
  margin-top: 18px; padding: 8px 22px; font: inherit; letter-spacing: 0.16em;
  cursor: pointer; border-radius: 2px;
  color: rgba(226,234,231,0.55);
  background: rgba(255,255,255,0.045);
  border: 1px solid rgba(255,255,255,0.09);
}
#calib button:hover { background: rgba(255,255,255,0.1); }
#calib button:focus-visible { outline: 1px solid rgba(255,255,255,0.35); }
`;

export class Brightness {
  private readonly slider: HTMLInputElement;
  private readonly bigSlider: HTMLInputElement;
  private readonly readout: HTMLSpanElement;
  private readonly bigReadout: HTMLDivElement;
  private readonly overlay: HTMLDivElement;
  private readonly swatches: HTMLDivElement[] = [];
  private biasEv = LADDER_BIAS_EV;
  private biasReadout: HTMLDivElement | null = null;

  /**
   * @param start    exposure to open with, already loaded from settings
   * @param onChange pushed the new exposure on every move
   * @param onDone   fired when calibration is dismissed, however it was opened
   */
  constructor(
    start: number,
    private onChange: (v: number) => void,
    private onDone: () => void = () => {},
  ) {
    const style = document.createElement("style");
    style.textContent = CSS;
    document.head.appendChild(style);

    if (BIAS_TUNING) {
      const b = Number(localStorage.getItem(BIAS_KEY));
      if (Number.isFinite(b)) this.biasEv = b;
    }

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
    p.textContent =
      "Raise the slider until patch 3 is only just visible. Patches 1 and 2 " +
      "should stay pure black.";

    // Build the ladder. Colours are filled in by apply(), which runs on every
    // slider move, so the patches track exposure live.
    const ladder = document.createElement("div");
    ladder.className = "ladder";
    PATCHES.forEach((_, i) => {
      const cell = document.createElement("div");
      cell.className = i === TARGET_PATCH ? "patch target" : "patch";
      const tick = document.createElement("div");
      tick.className = "tick";
      tick.textContent = "▾";
      const sw = document.createElement("div");
      sw.className = "sw";
      const n = document.createElement("div");
      n.className = "n";
      n.textContent = String(i + 1);
      cell.append(tick, sw, n);
      this.swatches.push(sw);
      ladder.appendChild(cell);
    });
    const note = document.createElement("div");
    note.className = "ladderNote";
    note.textContent =
      "If you can see 1 or 2, it is too bright. If 3 is black, too dark.";

    this.bigSlider = this.makeSlider(start);
    this.bigSlider.classList.add("big");
    this.bigReadout = document.createElement("div");
    this.bigReadout.className = "num";
    const done = document.createElement("button");
    done.textContent = "DONE";
    done.addEventListener("click", () => this.close());
    panel.append(h, p, ladder, note, this.bigSlider, this.bigReadout, done);
    if (BIAS_TUNING) panel.appendChild(this.makeBiasRow());
    this.overlay.appendChild(panel);
    document.body.appendChild(this.overlay);

    this.apply(start);
  }

  /**
   * Temporary tuning control for LADDER_BIAS_EV.
   *
   * Slide until the marked patch is genuinely at the edge of visible at an
   * exposure that looks right in play, then bake the printed number into
   * LADDER_BIAS_EV and delete this. The byte row is printed alongside because
   * it is the actual evidence — "+1.8 EV" means nothing on its own, whereas
   * the resulting code values say precisely how far off the old anchor was.
   */
  private makeBiasRow(): HTMLDivElement {
    const wrap = document.createElement("div");
    wrap.className = "biasRow";
    const label = document.createElement("div");
    label.className = "biasLabel";
    label.textContent = "LADDER BIAS (dev) — bake into LADDER_BIAS_EV";
    const s = document.createElement("input");
    s.type = "range";
    s.className = "exslider";
    s.min = "-2";
    s.max = "6";
    s.step = "0.1";
    s.value = String(this.biasEv);
    s.addEventListener("input", () => {
      this.biasEv = parseFloat(s.value);
      try { localStorage.setItem(BIAS_KEY, String(this.biasEv)); } catch { /* ignore */ }
      this.apply(parseFloat(this.bigSlider.value));
    });
    this.biasReadout = document.createElement("div");
    this.biasReadout.className = "biasVal";
    wrap.append(label, s, this.biasReadout);
    return wrap;
  }

  private makeSlider(value: number): HTMLInputElement {
    const s = document.createElement("input");
    s.type = "range";
    s.className = "exslider";
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
    this.paintLadder(v);
    // Persistence is the settings store's job now, reached through onChange.
    // Two places writing the same value was how the first-run check ended up
    // reading a key its own constructor had just created.
    this.onChange(v);
  }

  private paintLadder(exposure: number): void {
    const gain = Math.pow(2, this.biasEv);
    const bytes: number[] = [];
    for (let i = 0; i < this.swatches.length; i++) {
      const b = agxByte(PATCHES[i] * gain, exposure);
      bytes.push(b);
      this.swatches[i].style.background = `rgb(${b},${b},${b})`;
    }
    if (this.biasReadout) {
      const row = bytes
        .map((b, i) => (i === TARGET_PATCH ? `[${b}]` : String(b)))
        .join(" ");
      this.biasReadout.textContent =
        `${this.biasEv >= 0 ? "+" : ""}${this.biasEv.toFixed(1)} EV   ${row}`;
    }
  }

  open(): void {
    this.overlay.classList.add("show");
  }

  close(): void {
    this.overlay.classList.remove("show");
    this.onDone();
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
    this.paintLadder(v);
  }
}
