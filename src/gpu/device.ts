// WebGPU device bootstrap + small helpers.
export interface Gpu {
  device: GPUDevice;
  adapterInfo: string;
  canvas: HTMLCanvasElement;
  ctx: GPUCanvasContext;
  format: GPUTextureFormat;
  hasTimestamps: boolean;
  linearSampler: GPUSampler;
  nearestSampler: GPUSampler;
}

export async function initGpu(canvas: HTMLCanvasElement): Promise<Gpu> {
  if (!('gpu' in navigator)) throw new Error('WebGPU not available (navigator.gpu missing). Use a current Chrome/Edge/Safari.');
  const adapter = await navigator.gpu.requestAdapter({ powerPreference: 'high-performance' });
  if (!adapter) throw new Error('No WebGPU adapter available.');
  const want: GPUFeatureName[] = ['timestamp-query', 'float32-filterable'];
  const feats = want.filter(f => adapter.features.has(f));
  const lim = adapter.limits;
  const device = await adapter.requestDevice({
    requiredFeatures: feats,
    requiredLimits: {
      maxStorageBufferBindingSize: Math.min(lim.maxStorageBufferBindingSize, 1 << 30),
      maxBufferSize: Math.min(lim.maxBufferSize, 1 << 30),
      maxComputeInvocationsPerWorkgroup: lim.maxComputeInvocationsPerWorkgroup,
      maxComputeWorkgroupSizeX: lim.maxComputeWorkgroupSizeX,
      maxStorageBuffersPerShaderStage: lim.maxStorageBuffersPerShaderStage,
      maxStorageTexturesPerShaderStage: lim.maxStorageTexturesPerShaderStage,
      maxSampledTexturesPerShaderStage: lim.maxSampledTexturesPerShaderStage,
      maxColorAttachmentBytesPerSample: lim.maxColorAttachmentBytesPerSample,
    },
  });
  device.lost.then(info => { console.error('WebGPU device lost:', info.message); showFatal('GPU device lost: ' + info.message); });
  device.addEventListener('uncapturederror', (ev) => { console.error('WebGPU error:', (ev as GPUUncapturedErrorEvent).error.message); });
  const ctx = canvas.getContext('webgpu') as GPUCanvasContext;
  const format = navigator.gpu.getPreferredCanvasFormat();
  ctx.configure({ device, format, alphaMode: 'opaque' });
  const info = adapter.info;
  return {
    device, canvas, ctx, format,
    adapterInfo: `${info.vendor} ${info.architecture} ${info.description}`.trim(),
    hasTimestamps: feats.includes('timestamp-query'),
    linearSampler: device.createSampler({ magFilter: 'linear', minFilter: 'linear', mipmapFilter: 'nearest', addressModeU: 'clamp-to-edge', addressModeV: 'clamp-to-edge', addressModeW: 'clamp-to-edge' }),
    nearestSampler: device.createSampler({ magFilter: 'nearest', minFilter: 'nearest' }),
  };
}

export function showFatal(msg: string) {
  const el = document.getElementById('fatal'); if (el) { el.style.display = 'flex'; el.textContent = msg; }
}

export function makeBuffer(device: GPUDevice, size: number, usage: GPUBufferUsageFlags, label?: string): GPUBuffer {
  return device.createBuffer({ size: Math.max(16, (size + 3) & ~3), usage, label });
}
export function makeBufferWithData(device: GPUDevice, data: ArrayBufferView, usage: GPUBufferUsageFlags, label?: string): GPUBuffer {
  const buf = device.createBuffer({ size: Math.max(16, (data.byteLength + 3) & ~3), usage: usage | GPUBufferUsage.COPY_DST, label });
  device.queue.writeBuffer(buf, 0, data.buffer, data.byteOffset, data.byteLength);
  return buf;
}
/** Compile a WGSL module; logs compile messages with line context. */
export function makeShader(device: GPUDevice, code: string, label: string): GPUShaderModule {
  const mod = device.createShaderModule({ code, label });
  mod.getCompilationInfo().then(info => {
    for (const m of info.messages) {
      const lines = code.split('\n'); const ctxLine = lines[m.lineNum - 1] ?? '';
      const s = `[WGSL ${m.type}] ${label}:${m.lineNum}:${m.linePos} ${m.message}\n  > ${ctxLine.trim()}`;
      if (m.type === 'error') console.error(s); else console.warn(s);
    }
  });
  return mod;
}
