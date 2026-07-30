// ---------------------------------------------------------------------------
// End-of-run card: COMPROMISED on death, GHOST / CLEAN on success.
//
// Two lines of text and a fade, deliberately not a menu system. The loop this
// closes is short — get seen, get shot, press R — and the card only has to say
// which end you reached and what to press.
// ---------------------------------------------------------------------------

const CSS = `
#endcard {
  position: fixed; inset: 0; display: flex; flex-direction: column;
  align-items: center; justify-content: center;
  background: rgba(0,0,0,0); pointer-events: none;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.32em; opacity: 0;
  transition: opacity 0.9s ease, background 0.9s ease;
}
#endcard.show { opacity: 1; background: rgba(4,3,5,0.72); }
#endcard.show.win { background: rgba(4,3,5,0.48); }
#endcard .word { font-size: 30px; color: rgba(255,120,90,0.95); }
#endcard.win .word { color: rgba(150,230,190,0.95); }
#endcard .sub {
  margin-top: 14px; font-size: 10px; letter-spacing: 0.22em;
  color: rgba(200,210,205,0.6);
}
`;

const OBJECTIVE_CSS = `
#objective {
  position: fixed; left: 18px; top: 16px;
  font: 10px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace;
  letter-spacing: 0.16em; color: rgba(190,205,200,0.42);
  user-select: none; pointer-events: none;
}
`;

/** One quiet line saying what "done" means. Not a briefing screen. */
export class Objective {
  constructor(text: string) {
    const style = document.createElement("style");
    style.textContent = OBJECTIVE_CSS;
    document.head.appendChild(style);
    const el = document.createElement("div");
    el.id = "objective";
    el.textContent = text;
    document.body.appendChild(el);
  }
}

export class EndCard {
  private readonly el: HTMLDivElement;
  private readonly word: HTMLDivElement;
  private readonly sub: HTMLDivElement;
  visible = false;

  constructor() {
    const style = document.createElement("style");
    style.textContent = CSS;
    document.head.appendChild(style);
    this.el = document.createElement("div");
    this.el.id = "endcard";
    this.word = document.createElement("div");
    this.word.className = "word";
    this.sub = document.createElement("div");
    this.sub.className = "sub";
    this.el.append(this.word, this.sub);
    document.body.appendChild(this.el);
  }

  show(word: string, sub: string, win: boolean): void {
    this.word.textContent = word;
    this.sub.textContent = sub;
    this.el.classList.toggle("win", win);
    this.el.classList.add("show");
    this.visible = true;
  }

  hide(): void {
    this.el.classList.remove("show", "win");
    this.visible = false;
  }
}
