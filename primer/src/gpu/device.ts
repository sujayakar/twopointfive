// The device, and the few helpers everything else is made with.

export interface Gpu {
  device: GPUDevice;
  context: GPUCanvasContext;
  format: GPUTextureFormat;
  timestamps: boolean;
  about: string;
}

export async function openGpu(canvas: HTMLCanvasElement, onError: (text: string) => void): Promise<Gpu> {
  if (!navigator.gpu) throw new Error('This browser has no WebGPU. Chrome on a Mac or on Windows has it.');
  const adapter = await navigator.gpu.requestAdapter({ powerPreference: 'high-performance' });
  if (!adapter) throw new Error('WebGPU found no GPU here.');
  const timestamps = adapter.features.has('timestamp-query');
  const most = adapter.limits;
  const device = await adapter.requestDevice({
    requiredFeatures: timestamps ? ['timestamp-query'] : [],
    requiredLimits: {
      maxStorageBufferBindingSize: Math.min(most.maxStorageBufferBindingSize, 1 << 29),
      maxBufferSize: Math.min(most.maxBufferSize, 1 << 29),
      maxStorageBuffersPerShaderStage: most.maxStorageBuffersPerShaderStage,
    },
  });
  void device.lost.then((lost) => {
    if (lost.reason !== 'destroyed') onError('The GPU was lost: ' + lost.message);
  });
  device.onuncapturederror = (event) => onError(event.error.message);
  const context = canvas.getContext('webgpu');
  if (!context) throw new Error('This canvas gives no WebGPU context.');
  const format = navigator.gpu.getPreferredCanvasFormat();
  context.configure({ device, format, alphaMode: 'opaque' });
  const info = adapter.info;
  const about = [info?.vendor, info?.architecture, info?.description].filter(Boolean).join(' ');
  return { device, context, format, timestamps, about };
}

export function makeBuffer(gpu: Gpu, label: string, bytes: number, usage: GPUBufferUsageFlags): GPUBuffer {
  return gpu.device.createBuffer({
    label,
    size: Math.max(16, Math.ceil(bytes / 16) * 16),
    usage: usage | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC,
  });
}

export async function readBack(gpu: Gpu, from: GPUBuffer, offset: number, bytes: number): Promise<DataView> {
  const staging = gpu.device.createBuffer({
    size: bytes,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
  });
  const encoder = gpu.device.createCommandEncoder();
  encoder.copyBufferToBuffer(from, offset, staging, 0, bytes);
  gpu.device.queue.submit([encoder.finish()]);
  await staging.mapAsync(GPUMapMode.READ);
  const copy = staging.getMappedRange().slice(0);
  staging.destroy();
  return new DataView(copy);
}

/** GPU time a pass, a few frames late. Where the browser gives no timestamps, nothing. */
export class Timer {
  readonly ms = new Map<string, number>();
  private set: GPUQuerySet | null = null;
  private resolved: GPUBuffer | null = null;
  private free: GPUBuffer[] = [];
  private names: string[] = [];
  private static MOST = 128;

  constructor(private gpu: Gpu) {
    if (!gpu.timestamps) return;
    this.set = gpu.device.createQuerySet({ type: 'timestamp', count: Timer.MOST * 2 });
    this.resolved = gpu.device.createBuffer({
      size: Timer.MOST * 16,
      usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC,
    });
  }

  begin(): void {
    this.names = [];
  }

  pass(name: string): GPUComputePassTimestampWrites | undefined {
    if (!this.set || this.names.length >= Timer.MOST) return undefined;
    const i = this.names.push(name) - 1;
    return { querySet: this.set, beginningOfPassWriteIndex: i * 2, endOfPassWriteIndex: i * 2 + 1 };
  }

  /** Call before submit; call what it returns after. */
  end(encoder: GPUCommandEncoder): () => void {
    const { set, resolved, names } = this;
    if (!set || !resolved || names.length === 0) return () => {};
    const staging =
      this.free.pop() ??
      this.gpu.device.createBuffer({
        size: Timer.MOST * 16,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
      });
    const bytes = names.length * 16;
    encoder.resolveQuerySet(set, 0, names.length * 2, resolved, 0);
    encoder.copyBufferToBuffer(resolved, 0, staging, 0, bytes);
    return () => {
      staging.mapAsync(GPUMapMode.READ, 0, bytes).then(() => {
        const t = new BigUint64Array(staging.getMappedRange(0, bytes));
        const now = new Map<string, number>();
        names.forEach((name, i) => {
          const ms = Number(t[i * 2 + 1]! - t[i * 2]!) / 1e6;
          if (ms >= 0 && ms < 1000) now.set(name, (now.get(name) ?? 0) + ms);
        });
        for (const name of [...this.ms.keys()]) if (!now.has(name)) this.ms.delete(name);
        for (const [name, ms] of now) this.ms.set(name, (this.ms.get(name) ?? ms) * 0.9 + ms * 0.1);
        staging.unmap();
        this.free.push(staging);
      }, () => {});
    };
  }
}
