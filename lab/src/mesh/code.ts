// How a scene with meshes is asked the questions every scene is asked: the pieces' own answers keep their text, under other names, and wgsl/mesh.wgsl answers in their place.
import meshCode from '../wgsl/mesh.wgsl' with { type: 'text' };
const rename = (code: string, from: string, to: string) => { if (code.split(from).length !== 2) throw new Error('mesh/code.ts: ' + from + ' is not there once'); return code.replace(from, to); };
export const meshed = {
  scene: (code: string, row: number, lost: number, stack: number) => rename(rename(code, 'fn meet(k: u32,', 'fn meetPiece(k: u32,'), 'fn visible(ro: vec3f,', 'fn visiblePieces(ro: vec3f,') + `\nconst MESH_ROW = ${row}u; const MESH_LOST = ${lost}u; const MESH_STACK = ${stack + 3}u;\n` + meshCode + '\n',
  surface: (code: string) => rename(code, 'fn surfaceAt(ro: vec3f,', 'fn surfacePiece(ro: vec3f,'),
  /** the same, where no ray meets triangles: the boxes that stand in for them answer. (Text that is there and never run still weighs on a kernel) */
  boxes: (shared: string) => put(put(shared, /fn meet\(k: u32,[^\n]*\n/, 'fn meet(k: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32, cut: f32) -> bool { if (isMesh(pieces[k].flags)) { return false; } return meetPiece(k, ro, rd, tmin, tmax, cut); }\n'), /fn surfaceAt\(ro: vec3f,[^\n]*\n/, 'fn surfaceAt(ro: vec3f, rd: vec3f, h: Hit, fp: f32) -> Surf { return surfacePiece(ro, rd, h, fp); }\n'),
};
const put = (code: string, from: RegExp, to: string) => { if ((code.match(new RegExp(from, 'g')) ?? []).length !== 1) throw new Error('mesh/code.ts: ' + from); return code.replace(from, to); };
