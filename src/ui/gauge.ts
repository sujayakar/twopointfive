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

#ammo {
  position: fixed; left: 18px; bottom: 92px;
  font: 10px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.14em; color: rgba(190,205,200,0.55);
  user-select: none; pointer-events: none;
}
#ammo .rounds {
  margin-top: 4px; font-size: 17px; letter-spacing: 0.06em;
  font-variant-numeric: tabular-nums; color: rgba(225,232,230,0.92);
}
/* Spare magazines, drawn rather than counted. A pooled number cannot say which
   magazine you are about to be holding, and that is the whole decision. */
#ammo .row { display: flex; align-items: flex-end; gap: 10px; }
#ammo .mags { display: flex; gap: 4px; padding-bottom: 4px; }
#ammo .magbox {
  position: relative; width: 9px; height: 20px; border-radius: 1px;
  box-shadow: inset 0 0 0 1px rgba(255,255,255,0.22);
  overflow: hidden;
}
#ammo .magbox i {
  position: absolute; left: 0; right: 0; bottom: 0;
  background: rgba(225,232,230,0.75);
  transition: height 0.12s linear;
}
#ammo .magbox.low i { background: rgba(255,170,90,0.8); }
/* Empty and reloading are the two states worth reading at a glance. */
#ammo.empty .rounds { color: rgba(255,120,90,0.95); }
#ammo .spare { opacity: 0.55; font-size: 11px; }
#ammo .busy { font-size: 9px; opacity: 0; transition: opacity 0.12s linear; }
#ammo.reloading .busy { opacity: 0.8; }

#equip {
  position: fixed; left: 18px; bottom: 152px;
  display: flex; gap: 6px;
  font: 10px/1 ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.1em; user-select: none; pointer-events: none;
}
#equip .slot {
  display: flex; align-items: baseline; gap: 5px;
  padding: 5px 8px; border-radius: 2px;
  background: rgba(255,255,255,0.04);
  box-shadow: inset 0 0 0 1px rgba(255,255,255,0.06);
  color: rgba(190,205,200,0.45);
  transition: background 0.12s linear, color 0.12s linear, box-shadow 0.12s linear;
}
#equip .slot .key { font-size: 9px; opacity: 0.6; }
#equip .slot .count { font-size: 10px; opacity: 0.85; }
#equip .slot .count.spent { opacity: 0.35; }
#equip .slot.on {
  background: rgba(255,255,255,0.11);
  box-shadow: inset 0 0 0 1px rgba(255,255,255,0.2);
  color: rgba(232,238,236,0.95);
}
/* Charging reads as unavailable without being alarming — it is a timer, not a
   failure, so it dims rather than turning red. */
#equip .slot.charging { opacity: 0.45; }
#equip .slot .meter {
  width: 26px; height: 2px; border-radius: 1px; align-self: center;
  background: rgba(255,255,255,0.12); overflow: hidden;
}
#equip .slot .meter i {
  display: block; height: 100%; width: 0%;
  background: rgba(120,200,255,0.9);
}
/* The torch is not a slot — it is a state that persists across every slot — so
   it reads as an indicator rather than another selectable box. */
#equip .torch {
  display: flex; align-items: center; gap: 5px;
  padding: 5px 8px; color: rgba(190,205,200,0.4);
}
#equip .torch .dot {
  width: 6px; height: 6px; border-radius: 50%;
  background: rgba(255,255,255,0.14);
  transition: background 0.12s linear, box-shadow 0.12s linear;
}
#equip .torch.on { color: rgba(232,238,236,0.9); }
#equip .torch.on .dot {
  background: rgba(255,236,190,0.95);
  box-shadow: 0 0 7px rgba(255,225,160,0.75);
}
`;

/** Bands, matching Visibility.band. Green reads as safe without being literal. */
const BANDS = {
  hidden: { lit: "rgba(90,220,150,0.85)", glow: "rgba(90,220,150,0.35)", text: "HIDDEN" },
  dim: { lit: "rgba(230,200,90,0.9)", glow: "rgba(230,200,90,0.35)", text: "PARTIAL" },
  exposed: { lit: "rgba(255,120,90,0.95)", glow: "rgba(255,120,90,0.45)", text: "EXPOSED" },
};

/**
 * Rounds in the magazine over rounds in reserve.
 *
 * Sits above the light gauge rather than in a corner of its own: both answer
 * "can I take this fight", and a player checking one is checking the other.
 */
export class AmmoReadout {
  private readonly el: HTMLDivElement;
  private readonly rounds: HTMLDivElement;
  private readonly boxes: HTMLDivElement[] = [];
  private readonly fills: HTMLElement[] = [];
  private last = "";

  /** @param slots how many spares are carried when nothing has been dropped */
  constructor(slots: number, private magSize: number) {
    this.el = document.createElement("div");
    this.el.id = "ammo";
    const title = document.createElement("div");
    title.textContent = "AMMO";
    this.rounds = document.createElement("div");
    this.rounds.className = "rounds";
    const busy = document.createElement("div");
    busy.className = "busy";
    busy.textContent = "RELOADING";

    const mags = document.createElement("div");
    mags.className = "mags";
    for (let i = 0; i < slots; i++) {
      const box = document.createElement("div");
      box.className = "magbox";
      const fill = document.createElement("i");
      box.appendChild(fill);
      this.boxes.push(box);
      this.fills.push(fill);
      mags.appendChild(box);
    }

    // The magazines sit beside the count, not under it: they are the same
    // reading — what is left — at two levels of detail.
    const row = document.createElement("div");
    row.className = "row";
    row.append(this.rounds, mags);

    this.el.append(title, row, busy);
    document.body.appendChild(this.el);
  }

  /** `spares` is one entry per magazine still carried, each its round count. */
  update(rounds: number, spares: number[], reloading: boolean): void {
    const key = `${rounds}/${spares.join(",")}/${reloading}`;
    if (key === this.last) return;
    this.last = key;
    this.rounds.textContent = String(rounds);
    this.el.classList.toggle("empty", rounds === 0);
    this.el.classList.toggle("reloading", reloading);

    // Fullest first, so the bar that is about to be loaded is the leftmost —
    // the reload always takes the best one, and the UI should say which.
    //
    // A magazine with nothing in it is not drawn at all. An empty outline reads
    // as something you still have; what you actually have is one fewer
    // magazine, and the row simply getting shorter says that plainly.
    const live = spares.filter((n) => n > 0).sort((a, b) => b - a);
    for (let i = 0; i < this.boxes.length; i++) {
      const n = live[i];
      const shown = n !== undefined;
      this.boxes[i].style.display = shown ? "block" : "none";
      if (!shown) continue;
      this.boxes[i].classList.toggle("low", n <= this.magSize * 0.34);
      this.fills[i].style.height = `${(n / this.magSize) * 100}%`;
    }
  }
}

/**
 * Equipment slots, selected with the number keys.
 *
 * Shows the OCP's recharge inline rather than as a separate readout — the only
 * question a player has about it is "can I use it yet", and that belongs on the
 * thing itself.
 */
export class EquipmentBar {
  private readonly el: HTMLDivElement;
  private readonly slots: HTMLDivElement[] = [];
  private readonly meters: HTMLElement[] = [];
  private readonly counts: HTMLElement[] = [];
  private readonly torch: HTMLDivElement;
  private last = "";

  constructor(labels: string[]) {
    this.el = document.createElement("div");
    this.el.id = "equip";
    labels.forEach((label, i) => {
      const s = document.createElement("div");
      s.className = "slot";
      const key = document.createElement("span");
      key.className = "key";
      key.textContent = String(i + 1);
      const name = document.createElement("span");
      name.textContent = label;
      s.append(key, name);
      // Countable ammunition (canisters). Hidden for slots without a count.
      const count = document.createElement("span");
      count.className = "count";
      count.style.display = "none";
      this.counts.push(count);
      s.appendChild(count);
      const meter = document.createElement("div");
      meter.className = "meter";
      const fill = document.createElement("i");
      meter.appendChild(fill);
      this.meters.push(fill);
      s.appendChild(meter);
      this.slots.push(s);
      this.el.appendChild(s);
    });
    this.torch = document.createElement("div");
    this.torch.className = "torch";
    const dot = document.createElement("span");
    dot.className = "dot";
    const tl = document.createElement("span");
    tl.textContent = "F LIGHT";
    this.torch.append(dot, tl);
    this.el.appendChild(this.torch);

    document.body.appendChild(this.el);
  }

  /**
   * `charge` is 0..1 per slot; 1 means ready and hides the meter. `counts`
   * carries remaining ammunition for countable slots (null = uncounted).
   */
  update(
    active: number, charge: number[], torchOn: boolean,
    counts: (number | null)[] = [],
  ): void {
    const key = `${active}/${charge.map((c) => c.toFixed(2)).join(",")}/` +
      `${counts.join(",")}/${torchOn}`;
    if (key === this.last) return;
    this.last = key;
    this.torch.classList.toggle("on", torchOn);
    this.slots.forEach((s, i) => {
      const c = charge[i] ?? 1;
      s.classList.toggle("on", i === active);
      s.classList.toggle("charging", c < 1);
      this.meters[i].style.width = `${Math.round(c * 100)}%`;
      (this.meters[i].parentElement as HTMLElement).style.display =
        c < 1 ? "block" : "none";
      const n = counts[i] ?? null;
      const badge = this.counts[i];
      if (n === null) {
        badge.style.display = "none";
      } else {
        badge.style.display = "";
        badge.textContent = `x${n}`;
        badge.classList.toggle("spent", n <= 0);
      }
    });
  }
}

const DETECT_CSS = `
#detect {
  position: fixed; left: 168px; bottom: 34px;
  font: 10px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.14em; color: rgba(190,205,200,0.55);
  user-select: none; pointer-events: none;
  transition: opacity 0.3s ease;
}
#detect .bar {
  position: relative; margin-top: 5px; width: 124px; height: 16px;
  border-radius: 1px; background: rgba(255,255,255,0.06);
  box-shadow: inset 0 0 0 1px rgba(255,255,255,0.05); overflow: hidden;
}
#detect .bar i {
  position: absolute; left: 0; top: 0; bottom: 0; width: 0%;
  background: var(--lit); box-shadow: 0 0 8px var(--glow);
  transition: width 0.1s linear, background 0.12s linear;
}
/* Threshold ticks: where suspicious and alert begin. The player is reading
   headroom, and the two lines are what headroom is measured against. */
#detect .bar b {
  position: absolute; top: 0; bottom: 0; width: 1px;
  background: rgba(255,255,255,0.22);
}
#detect .state { margin-top: 5px; font-size: 9px; opacity: 0.85; }
#detect.seen .state { color: rgba(255,120,90,0.95); }
`;

/** Detection states, ordered from safe to caught. */
const DETECT_STATES = {
  HIDDEN: { lit: "rgba(90,220,150,0.85)", glow: "rgba(90,220,150,0.35)" },
  SUSPICIOUS: { lit: "rgba(230,200,90,0.9)", glow: "rgba(230,200,90,0.35)" },
  SEEN: { lit: "rgba(255,120,90,0.95)", glow: "rgba(255,120,90,0.5)" },
  HUNTED: { lit: "rgba(255,90,60,0.95)", glow: "rgba(255,90,60,0.55)" },
} as const;

/**
 * The loudest guard's suspicion, and what it means.
 *
 * Companion to the light gauge and next to it on purpose: the light meter is
 * how visible you are, this is how much that visibility has cost you. The
 * ticks mark where SUSPICIOUS and ALERT begin, so the bar reads as headroom.
 */
export class DetectionMeter {
  private readonly el: HTMLDivElement;
  private readonly fill: HTMLElement;
  private readonly state: HTMLDivElement;
  private last = "";

  constructor(suspiciousAt: number, alertAt: number) {
    const style = document.createElement("style");
    style.textContent = DETECT_CSS;
    document.head.appendChild(style);

    this.el = document.createElement("div");
    this.el.id = "detect";
    const title = document.createElement("div");
    title.textContent = "DETECTION";
    const bar = document.createElement("div");
    bar.className = "bar";
    this.fill = document.createElement("i");
    bar.appendChild(this.fill);
    for (const t of [suspiciousAt, alertAt]) {
      const tick = document.createElement("b");
      tick.style.left = `${Math.round(t * 100)}%`;
      bar.appendChild(tick);
    }
    this.state = document.createElement("div");
    this.state.className = "state";
    this.el.append(title, bar, this.state);
    document.body.appendChild(this.el);
  }

  update(level: number, label: keyof typeof DETECT_STATES): void {
    const pct = Math.round(Math.min(Math.max(level, 0), 1) * 100);
    const key = `${pct}/${label}`;
    if (key === this.last) return;
    this.last = key;
    const s = DETECT_STATES[label];
    this.el.style.setProperty("--lit", s.lit);
    this.el.style.setProperty("--glow", s.glow);
    this.el.classList.toggle("seen", label === "SEEN");
    this.fill.style.width = `${pct}%`;
    this.state.textContent = label;
  }
}

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
