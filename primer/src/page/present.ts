// THE LAST STEP: panels onto the canvas. A panel's picture is a buffer of linear color; here it is
// magnified if asked, and coded for the screen.

import type { Gpu } from '../gpu/device';
import { makeBuffer } from '../gpu/device';

export interface Panel {
  picture: GPUBuffer;
  /** its place on the canvas, device pixels */
  x: number;
  y: number;
  width: number;
  height: number;
  /** what part of it is shown: the middle (0 to 1) and how much larger */
  middle: [number, number];
  zoom: number;
  /** only this much of its width, from here to here (for a wipe) */
  from: number;
  to: number;
}

const code = `
struct Panel { size: vec2f, zoom: f32, pad: f32, middle: vec2f, picture: vec2f }
@group(0) @binding(0) var<uniform> panel: Panel;
@group(0) @binding(1) var<storage, read> picture: array<vec4f>;
struct Corner { @builtin(position) at: vec4f, @location(0) uv: vec2f }
@vertex fn corner(@builtin(vertex_index) i: u32) -> Corner {
  let uv = vec2f(f32((i << 1u) & 2u), f32(i & 2u));
  return Corner(vec4f(uv * vec2f(2.0, -2.0) + vec2f(-1.0, 1.0), 0.0, 1.0), uv);
}
@fragment fn color(c: Corner) -> @location(0) vec4f {
  let uv = panel.middle + (c.uv - 0.5) / panel.zoom;
  if (any(uv < vec2f(0.0)) || any(uv >= vec2f(1.0))) { return vec4f(0.05, 0.05, 0.055, 1.0); }
  let q = vec2u(uv * panel.picture);
  let linear = clamp(picture[q.y * u32(panel.picture.x) + q.x].rgb, vec3f(0.0), vec3f(1.0));
  let coded = select(1.055 * pow(linear, vec3f(1.0 / 2.4)) - 0.055, linear * 12.92, linear <= vec3f(0.0031308));
  return vec4f(coded, 1.0);
}`;

export class Present {
  private pipeline: GPURenderPipeline;
  private uniforms: GPUBuffer[] = [];

  constructor(private gpu: Gpu) {
    const module = gpu.device.createShaderModule({ label: 'present', code });
    this.pipeline = gpu.device.createRenderPipeline({
      label: 'present',
      layout: 'auto',
      vertex: { module, entryPoint: 'corner' },
      fragment: { module, entryPoint: 'color', targets: [{ format: gpu.format }] },
      primitive: { topology: 'triangle-list' },
    });
  }

  draw(encoder: GPUCommandEncoder, panels: Panel[], pictureSize: [number, number]): void {
    const { device, context } = this.gpu;
    const target = context.getCurrentTexture();
    const pass = encoder.beginRenderPass({
      label: 'present',
      colorAttachments: [{
        view: target.createView(), loadOp: 'clear', storeOp: 'store',
        clearValue: { r: 0.05, g: 0.05, b: 0.055, a: 1 },
      }],
    });
    pass.setPipeline(this.pipeline);
    panels.forEach((p, i) => {
      const uniform = (this.uniforms[i] ??= makeBuffer(this.gpu, `panel ${i}`, 32, GPUBufferUsage.UNIFORM));
      device.queue.writeBuffer(uniform, 0, new Float32Array([
        p.width, p.height, p.zoom, 0, ...p.middle, ...pictureSize,
      ]));
      const x0 = Math.round(p.x + p.width * p.from);
      const x1 = Math.round(p.x + p.width * p.to);
      if (x1 <= x0 || p.x + p.width > target.width || p.y + p.height > target.height) return;
      pass.setViewport(p.x, p.y, p.width, p.height, 0, 1);
      pass.setScissorRect(x0, p.y, x1 - x0, p.height);
      pass.setBindGroup(0, device.createBindGroup({
        layout: this.pipeline.getBindGroupLayout(0),
        entries: [{ binding: 0, resource: { buffer: uniform } }, { binding: 1, resource: { buffer: p.picture } }],
      }));
      pass.draw(3);
    });
    pass.end();
  }
}
