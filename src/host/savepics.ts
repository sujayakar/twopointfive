// A SAVE'S PICTURE is taken from the first presented frame after it and handed back small, for whichever page keeps the save: boxed down on the GPU and read back with no wait where the
// canvas may be read (render/savepic.ts), else drawn into a 2D canvas in that frame, which makes the page wait for the GPU. One that cannot be taken is simply not there.
import type { Gpu } from '../gpu/device';
import { SavePicture } from '../render/savepic';

export class SavePics<T> {
  private due: T[] = [];
  private pass: SavePicture | null;
  private lost = false;
  private now: HTMLCanvasElement | null = null; private late: HTMLCanvasElement | null = null;
  /** `got`: a save's picture, as a data URL · `failed`: why one was not taken */
  constructor(private gpu: Gpu, private canvas: HTMLCanvasElement, private px: () => number, private q: () => number, private got: (row: T, url: string) => void, private failed: (e: unknown) => void) { this.pass = gpu.canvasRead ? new SavePicture(gpu.device, gpu.linearSampler) : null; }
  /** a save was made: the next presented frame is its picture */
  want(row: T): void { this.due.push(row); if (this.due.length > 4) this.due.shift(); }
  get wanted(): boolean { return this.due.length > 0; }
  private get road(): boolean { return !!this.pass?.ready && !this.lost; }
  /** (a read-out) */
  said(): { late: boolean; canvasRead: boolean; pending: number; taken: number; lost: number } { const p = this.pass; return { late: this.road, canvasRead: !!this.gpu.canvasRead, pending: p?.pending ?? 0, taken: p?.taken ?? 0, lost: p?.lost ?? 0 }; }
  /** (a check's: the late road comes to nothing from here) */
  lose(): void { if (this.pass) this.pass.take = () => Promise.resolve(null); }
  /** a frame has just been presented */
  take(): void {
    const due = this.due.splice(0);
    try {
      const W = Math.max(16, Math.round(this.px())), H = Math.max(9, Math.round(W * this.canvas.height / Math.max(1, this.canvas.width)));
      const jpeg = (c: HTMLCanvasElement, cx: CanvasRenderingContext2D | null, draw: (cx: CanvasRenderingContext2D) => void): void => { if (!cx) return; draw(cx); const url = c.toDataURL('image/jpeg', this.q()); for (const row of due) this.got(row, url); };
      if (this.pass && this.road) {
        // (its own small canvas, kept on the CPU: the pixels are put in and read straight out, and a canvas the browser backs with the GPU would wait for it again)
        this.pass.take(this.gpu.ctx.getCurrentTexture(), W, H).then(px => {
          if (!px) { if (!this.lost) console.warn('save picture: the late road came to nothing; pictures are taken the old way from here'); this.lost = true; this.due.push(...due); return; }   // (…and these saves' pictures are the next presented frame's)
          try { const c = this.late ??= document.createElement('canvas'); c.width = W; c.height = H; jpeg(c, c.getContext('2d', { willReadFrequently: true }), cx => { const im = cx.createImageData(W, H); im.data.set(px); cx.putImageData(im, 0, 0); }); } catch (e) { this.failed(e); }
        });
        return;
      }
      const c = this.now ??= document.createElement('canvas'); c.width = W; c.height = H;
      jpeg(c, c.getContext('2d'), cx => cx.drawImage(this.canvas, 0, 0, W, H));
    } catch (e) { this.failed(e); }
  }
}
