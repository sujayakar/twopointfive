// ---------------------------------------------------------------------------
// Light gauge.
//
// The one piece of diegetic UI in the game: how visible the player currently
// is, read from the actual illumination reaching them rather than from a
// trigger volume. Splinter Cell put this on the goggles for a reason — a
// stealth game where you cannot tell whether you are hidden is a guessing game,
// and the whole point here is that the renderer already knows the answer.
// ---------------------------------------------------------------------------

const SEGMENTS = 14;

const CSS = `
#gauge {
  position: fixed; left: 18px; bottom: 34px;
  font: 10px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.14em; color: rgba(190,205,200,0.55);
  user-select: none; pointer-events: none;
  transition: opacity 0.3s ease;
}
#gauge .cells { display: flex; gap: 2px; margin-top: 5px; }
#gauge .cell {
  width: 7px; height: 16px; border-radius: 1px;
  background: rgba(255,255,255,0.06);
  box-shadow: inset 0 0 0 1px rgba(255,255,255,0.05);
  transition: background 0.12s linear, box-shadow 0.12s linear;
}
#gauge .cell.on {
  background: var(--lit);
  box-shadow: 0 0 6px var(--glow), inset 0 0 0 1px rgba(255,255,255,0.18);
}
#gauge .state { margin-top: 5px; font-size: 9px; opacity: 0.75; }
`;

/** Bands, matching Visibility.band. Green reads as safe without being literal. */
const BANDS = {
  hidden: { lit: "rgba(90,220,150,0.85)", glow: "rgba(90,220,150,0.35)", text: "HIDDEN" },
  dim: { lit: "rgba(230,200,90,0.9)", glow: "rgba(230,200,90,0.35)", text: "PARTIAL" },
  exposed: { lit: "rgba(255,120,90,0.95)", glow: "rgba(255,120,90,0.45)", text: "EXPOSED" },
};

export class LightGauge {
  private readonly el: HTMLDivElement;
  private readonly cells: HTMLDivElement[] = [];
  private readonly state: HTMLDivElement;
  private lastLit = -1;
  private lastBand = "";

  constructor() {
    const style = document.createElement("style");
    style.textContent = CSS;
    document.head.appendChild(style);

    this.el = document.createElement("div");
    this.el.id = "gauge";

    const title = document.createElement("div");
    title.textContent = "LIGHT";

    const cells = document.createElement("div");
    cells.className = "cells";
    for (let i = 0; i < SEGMENTS; i++) {
      const c = document.createElement("div");
      c.className = "cell";
      this.cells.push(c);
      cells.appendChild(c);
    }

    this.state = document.createElement("div");
    this.state.className = "state";

    this.el.append(title, cells, this.state);
    document.body.appendChild(this.el);
  }

  /** `level` is 0..1; `band` comes straight from Visibility. */
  update(level: number, band: keyof typeof BANDS): void {
    const lit = Math.round(Math.min(Math.max(level, 0), 1) * SEGMENTS);
    // Touching the DOM every frame for 14 unchanged nodes is pure waste, and
    // this sits in the frame loop.
    if (lit === this.lastLit && band === this.lastBand) return;

    if (band !== this.lastBand) {
      const b = BANDS[band];
      this.el.style.setProperty("--lit", b.lit);
      this.el.style.setProperty("--glow", b.glow);
      this.state.textContent = b.text;
      this.lastBand = band;
    }
    for (let i = 0; i < SEGMENTS; i++) {
      this.cells[i].classList.toggle("on", i < lit);
    }
    this.lastLit = lit;
  }

  setVisible(v: boolean): void {
    this.el.style.opacity = v ? "1" : "0";
  }
}
