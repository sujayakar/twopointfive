// THE PICTURE'S SIZE IN THE WINDOW, read off the page as a frame begins and at no other time. A browser asked for it after something has written the page lays
// the page out there and then, and the HUD writes the page between the readers: the hand, the cursor, the renderer, the overlay and the bubbles all have it here.
import type { View } from './hud';

export class ViewSize implements View {
  clientWidth = 0; clientHeight = 0;
  constructor(private canvas: View) { this.read(); }
  /** a frame begins */
  read(): void { this.clientWidth = this.canvas.clientWidth; this.clientHeight = this.canvas.clientHeight; }
  get width(): number { return this.canvas.width; }
  get height(): number { return this.canvas.height; }
}
