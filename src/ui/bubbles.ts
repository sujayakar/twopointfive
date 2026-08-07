// Speech bubbles: what a guard is saying, drawn over his head while he is on screen (Game.say() decides bubble vs log line).
// One absolutely-positioned element per guard inside the overlay root; projection matches the state markers so the bubble sits
// just above the ! / ? glyph. Kept deliberately small and monochrome — a caption, not a comic balloon.
import type { Game } from '../game/game';
import type { FollowCamera } from '../game/camera';

export class Bubbles {
  private els: HTMLElement[] = [];
  constructor(private root: HTMLElement) {}

  update(game: Game, cam: FollowCamera, canvas: HTMLCanvasElement) {
    const w = canvas.clientWidth, h = canvas.clientHeight;
    while (this.els.length < game.guards.length) { const el = document.createElement('div'); el.className = 'bubble'; el.style.display = 'none'; this.root.appendChild(el); this.els.push(el); }
    for (let i = 0; i < this.els.length; i++) {
      const el = this.els[i]; const g = game.guards[i]; const b = g?.bubble;
      if (!g || !b || g.state === 'dead') { el.style.display = 'none'; continue; }
      const age = game.time - b.t;
      if (age > b.dur) { g.bubble = null; el.style.display = 'none'; continue; }
      const head = g.char.bones.head ?? g.char.pos;
      const [x, y, vis] = cam.project([head[0], head[1] + 0.62, head[2]], w, h, 1.05);   // a hand above the state glyph (which rides at +0.45); gone once 5 % of the view past its edge
      if (!vis) { el.style.display = 'none'; continue; }
      // ease in over 0.12 s (rise 6 px + fade), hold, fade over the last 0.35 s
      const aIn = Math.min(1, age / 0.12), aOut = Math.min(1, (b.dur - age) / 0.35); const a = Math.min(aIn, aOut);
      const text = (b.radio ? '·)) ' : '') + b.text;
      if (el.textContent !== text) el.textContent = text;
      const cls = 'bubble ' + (g.state === 'alert' ? 'alert' : g.state === 'suspicious' || g.state === 'search' ? 'sus' : 'calm') + (b.radio ? ' radio' : '');
      if (el.className !== cls) el.className = cls;
      el.style.display = 'block'; el.style.left = `${Math.round(x)}px`; el.style.top = `${Math.round(y - (1 - aIn) * 6)}px`; el.style.opacity = a.toFixed(2);
    }
  }
}
