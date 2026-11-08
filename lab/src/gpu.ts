// The device, and the few helpers everything else is made with.
export type Gpu = { device: GPUDevice; context: GPUCanvasContext | null; format: GPUTextureFormat; timestamps: boolean; info: string };

export async function openGpu(canvas: HTMLCanvasElement | null): Promise<Gpu> {
  if (!navigator.gpu) throw new Error('This browser has no WebGPU. Chrome on a Mac or on Windows has it.');
  const adapter = await navigator.gpu.requestAdapter({ powerPreference: 'high-performance' });
  if (!adapter) throw new Error('WebGPU found no GPU here.');
  const timestamps = adapter.features.has('timestamp-query'), L = adapter.limits;
  const device = await adapter.requestDevice({
    requiredFeatures: timestamps ? ['timestamp-query'] : [],
    requiredLimits: {
      maxStorageBufferBindingSize: Math.min(L.maxStorageBufferBindingSize, 1 << 28), maxBufferSize: Math.min(L.maxBufferSize, 1 << 28),
      maxStorageBuffersPerShaderStage: L.maxStorageBuffersPerShaderStage, maxStorageTexturesPerShaderStage: L.maxStorageTexturesPerShaderStage,
    },
  });
  device.lost.then(i => { if (i.reason !== 'destroyed') showError('The GPU was lost: ' + i.message); });
  device.onuncapturederror = e => { console.error(e.error.message); showError(e.error.message); };
  const format = navigator.gpu.getPreferredCanvasFormat();
  const context = canvas ? canvas.getContext('webgpu') : null;
  context?.configure({ device, format, alphaMode: 'opaque' });
  const i = adapter.info;
  return { device, context, format, timestamps, info: [i?.vendor, i?.architecture, i?.description].filter(Boolean).join(' ') };
}

export function showError(text: string) {
  const el = document.getElementById('error'); if (!el) return;
  el.hidden = false; if ((el.textContent?.length ?? 0) < 2000) el.textContent += text + '\n';
}

export function buffer(g: Gpu, label: string, data: ArrayBufferView | number, usage: number): GPUBuffer {
  const bytes = typeof data === 'number' ? data : data.byteLength;
  const b = g.device.createBuffer({ label, size: Math.max(16, (bytes + 15) & ~15), usage: usage | GPUBufferUsage.COPY_DST });
  if (typeof data !== 'number') g.device.queue.writeBuffer(b, 0, data.buffer as ArrayBuffer, data.byteOffset, data.byteLength);
  return b;
}

export async function readBuffer(g: Gpu, b: GPUBuffer, bytes = b.size): Promise<ArrayBuffer> {
  const r = g.device.createBuffer({ size: bytes, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
  const e = g.device.createCommandEncoder(); e.copyBufferToBuffer(b, 0, r, 0, bytes); g.device.queue.submit([e.finish()]);
  await r.mapAsync(GPUMapMode.READ); const out = r.getMappedRange().slice(0); r.destroy(); return out;
}

const BYTES: Record<string, number> = { rgba8unorm: 4, r32float: 4, rg16float: 4, rgba16float: 8, rg32uint: 8, rgba32float: 16, rgba32uint: 16, r32uint: 4 };
/** a texture read back as tightly packed rows */
export async function readTexture(g: Gpu, t: GPUTexture): Promise<{ data: ArrayBuffer; w: number; h: number; format: string }> {
  const px = BYTES[t.format]; if (!px) throw new Error('readTexture: ' + t.format);
  const row = t.width * px, pitch = (row + 255) & ~255;
  const r = g.device.createBuffer({ size: pitch * t.height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
  const e = g.device.createCommandEncoder();
  e.copyTextureToBuffer({ texture: t }, { buffer: r, bytesPerRow: pitch }, [t.width, t.height]); g.device.queue.submit([e.finish()]);
  await r.mapAsync(GPUMapMode.READ);
  const src = new Uint8Array(r.getMappedRange()), out = new Uint8Array(row * t.height);
  for (let y = 0; y < t.height; y++) out.set(src.subarray(y * pitch, y * pitch + row), y * row);
  r.destroy(); return { data: out.buffer, w: t.width, h: t.height, format: t.format };
}

/** GPU time of each pass, a few frames late; nothing where the browser gives no timestamps */
export class Timer {
  readonly ms = new Map<string, number>();
  private set: GPUQuerySet | null = null; private resolve!: GPUBuffer; private reads: GPUBuffer[] = []; private names: string[] = [];
  constructor(private g: Gpu, private max = 48) {
    if (!g.timestamps) return;
    this.set = g.device.createQuerySet({ type: 'timestamp', count: max * 2 });
    this.resolve = g.device.createBuffer({ size: max * 16, usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC });
  }
  begin() { this.names = []; }
  pass(name: string): GPUComputePassTimestampWrites | undefined {
    if (!this.set || this.names.length >= this.max) return undefined;
    const i = this.names.push(name) - 1;
    return { querySet: this.set, beginningOfPassWriteIndex: i * 2, endOfPassWriteIndex: i * 2 + 1 };
  }
  end(e: GPUCommandEncoder): (() => void) | null {
    if (!this.set || !this.names.length) return null;
    const n = this.names.length, names = this.names, r = this.reads.pop() ?? this.g.device.createBuffer({ size: this.max * 16, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    e.resolveQuerySet(this.set, 0, n * 2, this.resolve, 0); e.copyBufferToBuffer(this.resolve, 0, r, 0, n * 16);
    return () => r.mapAsync(GPUMapMode.READ, 0, n * 16).then(() => {
      const t = new BigUint64Array(r.getMappedRange(0, n * 16)), sum = new Map<string, number>();
      for (let i = 0; i < n; i++) sum.set(names[i]!, (sum.get(names[i]!) ?? 0) + Number(t[i * 2 + 1]! - t[i * 2]!) / 1e6);
      for (const k of [...this.ms.keys()]) if (!sum.has(k)) this.ms.delete(k);
      for (const [k, v] of sum) this.ms.set(k, v < 0 || v > 1000 ? 0 : (this.ms.get(k) ?? v) * 0.9 + v * 0.1);
      r.unmap(); this.reads.push(r);
    }).catch(() => {});
  }
}
