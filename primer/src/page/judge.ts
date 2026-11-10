// THE JUDGE. Two numbers for an answer beside its truth: how far its mean over time is off, and how noisy
// single frames still are. Read at some thousands of pixels spread over the picture.

import { readBack, type Gpu } from '../gpu/device';
import type { Graph } from '../gpu/graph';
import { struct, vec3f, f32 } from '../gpu/layout';
import type { View } from './views';

const ACROSS = 64;
const DOWN = 40;
const Judged = struct('Judged', { answer: vec3f(''), spare: f32(''), truth: vec3f(''), spare2: f32('') });

export interface Verdict {
  /** root mean square of (mean − truth), over the truth's own */
  off: number;
  /** the spread of single frames about their mean, over the truth's */
  noise: number;
  frames: number;
}

export class Judge {
  verdicts = new Map<string, Verdict>();
  private sums = new Map<string, { n: number; sum: Float64Array; square: Float64Array; truth: Float32Array }>();

  constructor(private gpu: Gpu, private graph: Graph) {}

  /** the buffer a view's judge writes; its stage is added when first asked for */
  bufferOf(view: View): string {
    const name = `judge ${view.id}`;
    if (this.graph.buffers.has(name) || !view.judged) return name;
    this.graph.addBuffer({ name, struct: Judged, size: { count: ACROSS * DOWN }, about: '' });
    this.graph.addStage({
      name, question: 'How far is this from the truth?',
      reads: { answer: view.judged.answer, truth: view.judged.truth, hit_x: 'hit_x' },
      writes: { judged: name }, uses: [], over: { count: ACROSS * DOWN },
      code: `fn run(i: u32) {
  let cell = vec2f(f32(i % ${ACROSS}u), f32(i / ${ACROSS}u));
  let p = vec2i((cell + 0.5) / vec2f(${ACROSS}.0, ${DOWN}.0) * vec2f(frame.render_size));
  let hit = hit_x_at(p);
  if (hit.distance < 0.0 || hit.lamp != NOTHING) { judged[i] = Judged(); return; }
  judged[i] = Judged(answer_at(p).value, 1.0, truth_at(p).value, 0.0);
}`,
    });
    return name;
  }

  forget(): void {
    this.sums.clear();
    this.verdicts.clear();
  }

  private reading = false;
  async read(views: View[]): Promise<void> {
    if (this.reading) return;
    this.reading = true;
    try {
      for (const view of views) {
        const data = await readBack(this.gpu, this.graph.buffer(`judge ${view.id}`), 0, ACROSS * DOWN * Judged.stride);
        const values = new Float32Array(data.buffer);
        const size = ACROSS * DOWN * 3;
        let s = this.sums.get(view.id);
        if (!s) {
          s = { n: 0, sum: new Float64Array(size), square: new Float64Array(size), truth: new Float32Array(size) };
          this.sums.set(view.id, s);
        }
        s.n++;
        let off = 0;
        let noise = 0;
        let scale = 0;
        for (let i = 0; i < ACROSS * DOWN; i++) {
          if (values[i * 8 + 3] === 0) continue;
          for (let c = 0; c < 3; c++) {
            const v = values[i * 8 + c]!;
            const k = i * 3 + c;
            s.sum[k]! += v;
            s.square[k]! += v * v;
            s.truth[k] = values[i * 8 + 4 + c]!;
            const mean = s.sum[k]! / s.n;
            off += (mean - s.truth[k]!) ** 2;
            noise += Math.max(0, s.square[k]! / s.n - mean * mean);
            scale += s.truth[k]! ** 2;
          }
        }
        this.verdicts.set(view.id, { off: Math.sqrt(off / scale), noise: Math.sqrt(noise / scale), frames: s.n });
      }
    } finally {
      this.reading = false;
    }
  }
}
