// DRAWING THE PICKED CLUSTERS: one indirect draw of triangles into a picture of numbers.

import type { Gpu } from '../gpu/device';
import type { Graph } from '../gpu/graph';
import type { Meshes } from './meshes';
import code from '../stages/mesh_draw.wgsl';
import { CLUSTER_SIZE } from './clusters';

export class MeshDraw {
  private pipeline: GPURenderPipeline | null = null;
  private building = false;
  private ids: GPUTexture | null = null;
  private depth: GPUTexture | null = null;

  constructor(private gpu: Gpu, private graph: Graph, private meshes: Meshes) {
    graph.addShared('mesh_ids', {
      type: 'texture_2d<u32>', structs: [], access: 'texture',
      buffer: () => { throw new Error('a texture'); }, view: () => this.target(),
    });
    graph.setCustom('mesh draw', (encoder) => this.run(encoder));
  }

  private viewOf: GPUTextureView | null = null;
  private target(): GPUTextureView {
    const [width, height] = this.graph.grids.render;
    if (!this.ids || this.ids.width !== width || this.ids.height !== height) {
      this.ids?.destroy();
      this.depth?.destroy();
      const { RENDER_ATTACHMENT, TEXTURE_BINDING } = GPUTextureUsage;
      this.ids = this.gpu.device.createTexture({ label: 'mesh ids', size: [width, height], format: 'rgba32uint', usage: RENDER_ATTACHMENT | TEXTURE_BINDING });
      this.depth = this.gpu.device.createTexture({ label: 'mesh depth', size: [width, height], format: 'depth32float', usage: RENDER_ATTACHMENT });
      this.viewOf = this.ids.createView();
    }
    return this.viewOf!;
  }

  /** before the clusters are picked: none is */
  start(encoder: GPUCommandEncoder): void {
    this.gpu.device.queue.writeBuffer(this.meshes.args, 0, new Uint32Array([CLUSTER_SIZE * 3, 0, 0, 0]));
    void encoder;
  }

  private names = ['frame', 'knobs', 'mesh_picked_list', 'mesh_clusters', 'mesh_instances', 'mesh_vertices', 'mesh_corners'];
  private run(encoder: GPUCommandEncoder): boolean {
    if (!this.pipeline) {
      if (!this.building) void this.build();
      return false;
    }
    const { device } = this.gpu;
    const view = this.target();
    const pass = encoder.beginRenderPass({
      label: 'mesh draw',
      colorAttachments: [{ view, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 0 } }],
      depthStencilAttachment: { view: this.depth!.createView(), depthLoadOp: 'clear', depthStoreOp: 'discard', depthClearValue: 0 },
      timestampWrites: this.graph.timer.pass('mesh draw') as GPURenderPassTimestampWrites | undefined,
    });
    pass.setPipeline(this.pipeline);
    pass.setBindGroup(0, device.createBindGroup({
      layout: this.pipeline.getBindGroupLayout(0),
      entries: this.buffers().map((buffer, binding) => ({ binding, resource: { buffer } })),
    }));
    pass.drawIndirect(this.meshes.args, 0);
    pass.end();
    return true;
  }

  private buffers: () => GPUBuffer[] = () => [];
  private async build(): Promise<void> {
    this.building = true;
    const { device } = this.gpu;
    this.graph.addShared('mesh_picked_list', {
      type: 'array<Job>', structs: [this.graph.spec('mesh_picked').struct], access: 'read',
      buffer: () => this.graph.buffer('mesh_picked'),
    });
    const shared = this.graph.sharedFor(this.names);
    const module = device.createShaderModule({ label: 'mesh draw', code: shared.wgsl + '\n' + code.replaceAll('mesh_picked[', 'mesh_picked_list[') });
    const layout = device.createBindGroupLayout({
      entries: this.names.map((name, binding) => ({
        binding,
        visibility: GPUShaderStage.VERTEX,
        buffer: { type: name === 'frame' || name === 'knobs' ? 'uniform' as const : 'read-only-storage' as const },
      })),
    });
    try {
      this.pipeline = await device.createRenderPipelineAsync({
        label: 'mesh draw',
        layout: device.createPipelineLayout({ bindGroupLayouts: [layout] }),
        vertex: { module, entryPoint: 'corner' },
        fragment: { module, entryPoint: 'ids', targets: [{ format: 'rgba32uint' }] },
        primitive: { topology: 'triangle-list', cullMode: 'none' },
        depthStencil: { format: 'depth32float', depthWriteEnabled: true, depthCompare: 'greater' },
      });
      this.buffers = shared.buffers;
    } catch (error) {
      this.graph.errors.set('mesh draw', String(error));
    }
  }
}
