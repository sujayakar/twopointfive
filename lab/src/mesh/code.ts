// How a scene with meshes is asked the questions every scene is asked: the pieces' own answers keep their text, under other names, and wgsl/mesh.wgsl answers in their place.
import meshCode from '../wgsl/mesh.wgsl' with { type: 'text' };
const rename = (code: string, from: string, to: string) => { if (code.split(from).length !== 2) throw new Error('mesh/code.ts: ' + from + ' is not there once'); return code.replace(from, to); };
export const meshed = {
  scene: (code: string, row: number) => rename(rename(code, 'fn meet(k: u32,', 'fn meetPiece(k: u32,'), 'fn visible(ro: vec3f,', 'fn visiblePieces(ro: vec3f,') + `\nconst MESH_ROW = ${row}u;\n` + meshCode + '\n',
  surface: (code: string) => rename(code, 'fn surfaceAt(ro: vec3f,', 'fn surfacePiece(ro: vec3f,'),
};
