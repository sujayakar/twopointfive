import commonSrc from "../shaders/common.wgsl?raw";
import pathtraceSrc from "../shaders/pathtrace.wgsl?raw";
import flashmapSrc from "../shaders/flashmap.wgsl?raw";
import radiositySrc from "../shaders/radiosity.wgsl?raw";
import reprojectSrc from "../shaders/reproject.wgsl?raw";
import atrousSrc from "../shaders/atrous.wgsl?raw";
import compositeSrc from "../shaders/composite.wgsl?raw";
import bloomSrc from "../shaders/bloom.wgsl?raw";
import postSrc from "../shaders/post.wgsl?raw";
import probeSrc from "../shaders/probe.wgsl?raw";
import lightVolumeSrc from "../shaders/lightvolume.wgsl?raw";
import { COUNTER_SLOTS } from "./counters";

/**
 * Counter slot constants (CT_<name>) generated from the one authoritative
 * list in counters.ts, so the shader indices and the readback labels cannot
 * drift apart the way two hand-maintained copies eventually would.
 */
const counterConstants =
  COUNTER_SLOTS.map((name, i) => `const CT_${name} : u32 = ${i}u;`).join("\n") +
  `\nconst CT_COUNT : u32 = ${COUNTER_SLOTS.length}u;\n`;

const shared = `${counterConstants}\n${commonSrc}`;

/**
 * Passes that touch scene data share `common.wgsl` (and therefore the group(0)
 * scene bind group layout). The post-processing passes are self-contained and
 * declare their own group(0).
 */
export const SHADERS = {
  pathtrace: `${shared}\n${pathtraceSrc}`,
  flashmap: `${shared}\n${flashmapSrc}`,
  radiosity: `${shared}\n${radiositySrc}`,
  reproject: `${shared}\n${reprojectSrc}`,
  atrous: `${shared}\n${atrousSrc}`,
  probe: `${shared}\n${probeSrc}`,
  lightVolume: `${shared}\n${lightVolumeSrc}`,
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
