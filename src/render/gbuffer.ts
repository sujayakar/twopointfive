// G-buffer targets + raster pipelines (boxes, skinned characters).
import { makeShader } from '../gpu/device';
import { scenePrelude } from './frame';
import gbufSrc from './shaders/gbuffer.wgsl' with { type: 'text' };

export interface SkinnedDrawable {
  vertexBuf: GPUBuffer;      // interleaved: pos f32x3, normal f32x3, weights f32x4, matSel f32 (44 B)
  jointBuf: GPUBuffer;       // uint8x4 (4 B) or uint16x4 (8 B), see jointsFormat
  jointsFormat: GPUVertexFormat;
  indexBuf: GPUBuffer; indexCount: number; indexFormat: GPUIndexFormat;
  jointMats: GPUBuffer;      // array<mat4x4f>
  charInsts: GPUBuffer;      // array<CharInst>
  instanceCount: number;
}

export class GBuffer {
  width = 0; height = 0;
  albedo!: GPUTexture; normal!: GPUTexture; id!: GPUTexture; depth!: GPUTexture;
  albedoView!: GPUTextureView; normalView!: GPUTextureView; idView!: GPUTextureView; depthView!: GPUTextureView;
  private boxPipe: GPURenderPipeline; private skinPipes = new Map<GPUVertexFormat, GPURenderPipeline>();
  private objLayout: GPUBindGroupLayout; private mod: GPUShaderModule; private pipeLayout: GPUPipelineLayout;
  private targets: GPUColorTargetState[]; private depthStencil: GPUDepthStencilState;
  private objBG: GPUBindGroup | null = null; private objBGKey = '';

  constructor(private device: GPUDevice, private sceneLayout: GPUBindGroupLayout) {
    const mod = makeShader(device, scenePrelude() + gbufSrc, 'gbuffer');
    this.objLayout = device.createBindGroupLayout({ label: 'gbufObj', entries: [
      { binding: 0, visibility: GPUShaderStage.VERTEX, buffer: { type: 'read-only-storage' } },
      { binding: 1, visibility: GPUShaderStage.VERTEX, buffer: { type: 'read-only-storage' } },
      { binding: 2, visibility: GPUShaderStage.VERTEX, buffer: { type: 'read-only-storage' } },
    ] });
    const layout = this.pipeLayout = device.createPipelineLayout({ bindGroupLayouts: [sceneLayout, this.objLayout] });
    const targets: GPUColorTargetState[] = this.targets = [{ format: 'rgba8unorm' }, { format: 'rgb10a2unorm' }, { format: 'r32uint' }];
    const depthStencil: GPUDepthStencilState = this.depthStencil = { format: 'depth32float', depthWriteEnabled: true, depthCompare: 'less' };
    this.mod = mod;
    this.boxPipe = device.createRenderPipeline({
      label: 'gbufBoxes', layout,
      vertex: { module: mod, entryPoint: 'vsBox' },
      fragment: { module: mod, entryPoint: 'fsBox', targets },
      primitive: { topology: 'triangle-list', cullMode: 'back' },
      depthStencil,
    });
  }

  private skinPipe(fmt: GPUVertexFormat): GPURenderPipeline {
    let p = this.skinPipes.get(fmt);
    if (p) return p;
    p = this.device.createRenderPipeline({
      label: 'gbufSkin', layout: this.pipeLayout,
      vertex: { module: this.mod, entryPoint: 'vsSkin', buffers: [
        { arrayStride: 44, attributes: [
          { shaderLocation: 0, offset: 0, format: 'float32x3' },
          { shaderLocation: 1, offset: 12, format: 'float32x3' },
          { shaderLocation: 3, offset: 24, format: 'float32x4' },
          { shaderLocation: 4, offset: 40, format: 'float32' },
        ] },
        { arrayStride: fmt === 'uint8x4' ? 4 : 8, attributes: [{ shaderLocation: 2, offset: 0, format: fmt }] },
      ] },
      fragment: { module: this.mod, entryPoint: 'fsSkin', targets: this.targets },
      primitive: { topology: 'triangle-list', cullMode: 'back' },
      depthStencil: this.depthStencil,
    });
    this.skinPipes.set(fmt, p);
    return p;
  }

  resize(w: number, h: number) {
    if (w === this.width && h === this.height) return;
    this.width = w; this.height = h;
    for (const t of [this.albedo, this.normal, this.id, this.depth]) t?.destroy();
    const usage = GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING;
    const mk = (format: GPUTextureFormat, label: string) => this.device.createTexture({ size: [w, h], format, usage, label });
    this.albedo = mk('rgba8unorm', 'gAlbedo'); this.normal = mk('rgb10a2unorm', 'gNormal'); this.id = mk('r32uint', 'gId'); this.depth = mk('depth32float', 'gDepth');
    this.albedoView = this.albedo.createView(); this.normalView = this.normal.createView(); this.idView = this.id.createView(); this.depthView = this.depth.createView();
  }

  private objectBindGroup(visibleBuf: GPUBuffer, skin: SkinnedDrawable | null, dummy: GPUBuffer): GPUBindGroup {
    const jm = skin?.jointMats ?? dummy, ci = skin?.charInsts ?? dummy;
    const key = `${idOf(visibleBuf)}:${idOf(jm)}:${idOf(ci)}`;
    if (this.objBG && this.objBGKey === key) return this.objBG;
    this.objBG = this.device.createBindGroup({ layout: this.objLayout, entries: [
      { binding: 0, resource: { buffer: visibleBuf } }, { binding: 1, resource: { buffer: jm } }, { binding: 2, resource: { buffer: ci } },
    ] });
    this.objBGKey = key; return this.objBG;
  }

  encode(enc: GPUCommandEncoder, sceneBG: GPUBindGroup, visibleBuf: GPUBuffer, visibleCount: number, skin: SkinnedDrawable | null, dummy: GPUBuffer, ts?: GPURenderPassTimestampWrites) {
    const pass = enc.beginRenderPass({
      label: 'gbuffer',
      colorAttachments: [
        { view: this.albedoView, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 0 } },
        { view: this.normalView, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0.5, g: 1, b: 0.5, a: 0 } },
        { view: this.idView, loadOp: 'clear', storeOp: 'store', clearValue: { r: 0xFFFFFFFF, g: 0, b: 0, a: 0 } },
      ],
      depthStencilAttachment: { view: this.depthView, depthLoadOp: 'clear', depthStoreOp: 'store', depthClearValue: 1 },
      timestampWrites: ts,
    });
    const obg = this.objectBindGroup(visibleBuf, skin, dummy);
    pass.setBindGroup(0, sceneBG); pass.setBindGroup(1, obg);
    pass.setPipeline(this.boxPipe);
    pass.draw(36, visibleCount);
    if (skin && skin.instanceCount > 0) {
      pass.setPipeline(this.skinPipe(skin.jointsFormat));
      pass.setVertexBuffer(0, skin.vertexBuf); pass.setVertexBuffer(1, skin.jointBuf);
      pass.setIndexBuffer(skin.indexBuf, skin.indexFormat);
      pass.drawIndexed(skin.indexCount, skin.instanceCount);
    }
    pass.end();
  }
}

const ids = new WeakMap<object, number>(); let nextId = 1;
function idOf(o: object): number { let v = ids.get(o); if (!v) { v = nextId++; ids.set(o, v); } return v; }
