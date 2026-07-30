import commonSrc from "../shaders/common.wgsl?raw";
import pathtraceSrc from "../shaders/pathtrace.wgsl?raw";
import flashmapSrc from "../shaders/flashmap.wgsl?raw";
import reprojectSrc from "../shaders/reproject.wgsl?raw";
import atrousSrc from "../shaders/atrous.wgsl?raw";
import compositeSrc from "../shaders/composite.wgsl?raw";
import bloomSrc from "../shaders/bloom.wgsl?raw";
import postSrc from "../shaders/post.wgsl?raw";
import probeSrc from "../shaders/probe.wgsl?raw";

/**
 * Passes that touch scene data share `common.wgsl` (and therefore the group(0)
 * scene bind group layout). The post-processing passes are self-contained and
 * declare their own group(0).
 */
export const SHADERS = {
  pathtrace: `${commonSrc}\n${pathtraceSrc}`,
  flashmap: `${commonSrc}\n${flashmapSrc}`,
  reproject: `${commonSrc}\n${reprojectSrc}`,
  atrous: `${commonSrc}\n${atrousSrc}`,
  probe: `${commonSrc}\n${probeSrc}`,
  composite: compositeSrc,
  bloom: bloomSrc,
  post: postSrc,
};

/**
 * Creates a shader module and surfaces compilation diagnostics. WGSL errors are
 * otherwise easy to miss: the pipeline creation failure comes later and points
 * at the wrong place.
 */
export async function createShaderModule(
  device: GPUDevice,
  label: string,
  code: string,
): Promise<GPUShaderModule> {
  const module = device.createShaderModule({ label, code });
  const info = await module.getCompilationInfo();
  const errors = info.messages.filter((m) => m.type === "error");
  if (info.messages.length > 0) {
    const lines = code.split("\n");
    for (const m of info.messages) {
      const src = m.lineNum > 0 ? lines[m.lineNum - 1] : "";
      const text = `[${label}] ${m.type} ${m.lineNum}:${m.linePos} ${m.message}\n    ${src}`;
      if (m.type === "error") console.error(text);
      else console.warn(text);
    }
  }
  if (errors.length > 0) {
    throw new Error(`${label}: ${errors.length} WGSL error(s) — see console`);
  }
  return module;
}
