/**
 * Every identifier a key event should answer to.
 *
 * `code` is the right primary key for WASD — it is the physical position, so
 * the movement cluster stays under the same fingers on AZERTY. But `code` is
 * empty under some remote-desktop stacks, virtual keyboards and automation
 * harnesses, which silently makes the whole game unresponsive. Deriving the
 * same identifier from `key` as a fallback costs nothing and removes that
 * whole failure class.
 */
function keyIds(e: KeyboardEvent): string[] {
  const ids: string[] = [];
  if (e.code) ids.push(e.code);
  const k = e.key;
  if (k && k.length === 1) {
    if (k >= "a" && k <= "z") ids.push(`Key${k.toUpperCase()}`);
    else if (k >= "A" && k <= "Z") ids.push(`Key${k}`);
    else if (k >= "0" && k <= "9") ids.push(`Digit${k}`);
    else if (k === "`" || k === "~") ids.push("Backquote");
    else if (k === " ") ids.push("Space");
  } else if (k === "Shift") {
    ids.push("ShiftLeft");
  }
  return ids;
}

export class Input {
  private down = new Set<string>();
  private pressedThisFrame = new Set<string>();
  /** Mouse position in device pixels, matching the canvas backing store. */
  mouseX = 0;
  mouseY = 0;

  constructor(private canvas: HTMLCanvasElement) {
    window.addEventListener("keydown", (e) => {
      // Let the panel's own controls take the keyboard when focused.
      if (e.target instanceof HTMLInputElement) return;
      for (const k of keyIds(e)) {
        if (!this.down.has(k)) this.pressedThisFrame.add(k);
        this.down.add(k);
        // Stop the page scrolling out from under the game.
        if (k.startsWith("Arrow") || k === "Space") e.preventDefault();
      }
    });
    window.addEventListener("keyup", (e) => {
      for (const k of keyIds(e)) this.down.delete(k);
    });
    window.addEventListener("blur", () => this.down.clear());

    // Mouse buttons go into the same sets as keys, under "Mouse0".."Mouse4",
    // so firing reads exactly like every other action and needs no second
    // edge-detection path. Down is bound to the canvas (the panel's own
    // controls keep their clicks); up is bound to the window, or dragging off
    // the canvas and releasing would leave the button stuck down forever.
    this.canvas.addEventListener("mousedown", (e) => {
      const id = `Mouse${e.button}`;
      if (!this.down.has(id)) this.pressedThisFrame.add(id);
      this.down.add(id);
    });
    window.addEventListener("mouseup", (e) => this.down.delete(`Mouse${e.button}`));
    // Right-drag inside the game should not open the browser menu.
    this.canvas.addEventListener("contextmenu", (e) => e.preventDefault());

    const onMove = (e: MouseEvent) => {
      const r = this.canvas.getBoundingClientRect();
      const dpr = this.canvas.width / Math.max(r.width, 1);
      this.mouseX = (e.clientX - r.left) * dpr;
      this.mouseY = (e.clientY - r.top) * dpr;
    };
    this.canvas.addEventListener("mousemove", onMove);
    this.canvas.addEventListener("mouseenter", onMove);
  }

  /** Key codes, plus "Mouse0".."Mouse4" for the mouse buttons. */
  held(code: string): boolean {
    return this.down.has(code);
  }

  /** True only on the frame the key or button went down. Call endFrame() each tick. */
  pressed(code: string): boolean {
    return this.pressedThisFrame.has(code);
  }

  endFrame(): void {
    this.pressedThisFrame.clear();
  }
}
