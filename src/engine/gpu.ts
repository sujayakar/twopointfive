export interface GPUContext {
  adapter: GPUAdapter;
  device: GPUDevice;
  context: GPUCanvasContext;
  canvas: HTMLCanvasElement;
  format: GPUTextureFormat;
  features: Set<string>;
  adapterInfo: GPUAdapterInfo | null;
  /**
   * Resolves when the device is lost, including an intentional `destroy()`.
   * Never rejects. `options.onDeviceLost` is the convenient way to consume it;
   * this is here for callers that want to await the raw promise.
   */
  lost: Promise<GPUDeviceLostInfo>;
}

export class GPUInitError extends Error {
  constructor(message: string, readonly detail: string) {
    super(message);
  }
}

export interface GPUInitOptions {
  /**
   * Called once if the device is lost for any reason other than an explicit
   * `device.destroy()`.
   *
   * Worth wiring: after a GPU reset, driver update or eGPU switch, WebGPU does
   * not throw — it turns nearly every call into a no-op. So the render loop's
   * try/catch never fires and the failure presents as the last frame frozen on
   * screen forever with no message at all.
   */
  onDeviceLost?(info: GPUDeviceLostInfo): void;
  /**
   * Uncaptured-error handler, installed on the device the moment it exists —
   * before any pipeline, shader module or bind group. Registering this after
   * the renderer is constructed means every validation error raised while
   * building those bypasses it. Defaults to a console.error.
   */
  onUncapturedError?(error: GPUError): void;
}

/** Optional features we take if the adapter offers them. */
const WANTED_FEATURES = ["timestamp-query", "shader-f16"] as const;

export async function initGPU(
  canvas: HTMLCanvasElement,
  options: GPUInitOptions = {},
): Promise<GPUContext> {
  if (!("gpu" in navigator) || !navigator.gpu) {
    throw new GPUInitError(
      "WebGPU unavailable",
      "This browser did not expose navigator.gpu. Chrome 113+, Edge 113+ or Safari 18+ " +
        "on macOS should work. In Safari, check Develop › Feature Flags › WebGPU.",
    );
  }

  const adapter = await navigator.gpu.requestAdapter({
    powerPreference: "high-performance",
  });
  if (!adapter) {
    throw new GPUInitError(
      "No WebGPU adapter",
      "requestAdapter() returned null. The GPU may be blocklisted, or the browser may be " +
        "running without hardware acceleration.",
    );
  }

  const requiredFeatures: GPUFeatureName[] = [];
  for (const f of WANTED_FEATURES) {
    if (adapter.features.has(f)) requiredFeatures.push(f as GPUFeatureName);
  }

  // The path tracer's storage-buffer count and workgroup storage are modest,
  // but ask for headroom on buffer size so big levels fit in one allocation.
  const device = await adapter.requestDevice({
    requiredFeatures,
    requiredLimits: {
      maxStorageBufferBindingSize: Math.min(
        adapter.limits.maxStorageBufferBindingSize,
        512 * 1024 * 1024,
      ),
      maxBufferSize: Math.min(adapter.limits.maxBufferSize, 512 * 1024 * 1024),
      // The trace pass fits the default of 4 exactly (its three radiance
      // signals share one array binding); ask for headroom up to 8 anyway,
      // which Apple and desktop GPUs report.
      maxStorageTexturesPerShaderStage: Math.min(
        adapter.limits.maxStorageTexturesPerShaderStage,
        8,
      ),
      // Scene buffers (boxes, materials, lights, BVH, dynamic, prev-dynamic) plus
      // the two merged reservoir buffers (DI and GI, both parity halves in one
      // binding each) plus the work-counters buffer = 9 in the trace pass. The
      // default is 8; Apple and desktop GPUs report at least 10.
      maxStorageBuffersPerShaderStage: Math.min(
        adapter.limits.maxStorageBuffersPerShaderStage,
        9,
      ),
    },
  });

  // Both handlers go on before the limit checks below, so that even a failure
  // during startup reports through the same path as one at frame 10000.
  device.addEventListener("uncapturederror", (e) => {
    const error = (e as GPUUncapturedErrorEvent).error;
    if (options.onUncapturedError) options.onUncapturedError(error);
    else console.error("[webgpu]", error.message);
  });

  const lost = device.lost;
  void lost.then((info) => {
    // "destroyed" is us tearing the device down on purpose; anything else is a
    // real loss the user needs to be told about.
    if (info.reason === "destroyed") return;
    console.error(`[webgpu] device lost (${info.reason}): ${info.message}`);
    options.onDeviceLost?.(info);
  });

  const need: Array<[keyof GPUSupportedLimits, number]> = [
    ["maxStorageTexturesPerShaderStage", 4],
    ["maxStorageBuffersPerShaderStage", 9],
  ];
  for (const [name, min] of need) {
    const got = device.limits[name] as number;
    if (got < min) {
      throw new GPUInitError(
        "GPU limits too low for this renderer",
        `${name} is ${got}, but at least ${min} is required. Without this ` +
          `check the failure surfaces later as a pipeline error that looks ` +
          `like a shader bug.`,
      );
    }
  }

  const context = canvas.getContext("webgpu");
  if (!context) {
    throw new GPUInitError(
      "Could not create WebGPU canvas context",
      "canvas.getContext('webgpu') returned null.",
    );
  }

  const format = navigator.gpu.getPreferredCanvasFormat();
  context.configure({
    device,
    format,
    alphaMode: "opaque",
    // The path tracer works in HDR internally; the swap chain stays SDR and the
    // tonemapper bridges the two.
    usage: GPUTextureUsage.RENDER_ATTACHMENT,
  });

  let adapterInfo: GPUAdapterInfo | null = null;
  try {
    adapterInfo = adapter.info ?? (await (adapter as never as { requestAdapterInfo(): Promise<GPUAdapterInfo> }).requestAdapterInfo());
  } catch {
    adapterInfo = null;
  }

  return {
    adapter,
    device,
    context,
    canvas,
    format,
    features: new Set(Array.from(device.features) as string[]),
    adapterInfo,
    lost,
  };
}
