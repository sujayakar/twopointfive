// A STRUCT, DECLARED ONCE. From one declaration come the WGSL struct, the byte layout, the menu of
// views and the rows of the inspector. Nothing is packed: every field is a named value with a unit.

export type Scalar = 'f32' | 'u32' | 'i32';
export type FieldType = Scalar | 'vec2f' | 'vec3f' | 'vec4f' | 'vec2u' | 'vec3u' | 'vec4u';

/** How a field is drawn when someone asks to see it. */
export type Look =
  | { kind: 'color' } //                         paint, 0 to 1, no light in it
  | { kind: 'light' } //                         light, through the exposure and the curve
  | { kind: 'direction' } //                     a unit vector: x, y, z as red, green, blue
  | { kind: 'place' } //                         a point: one square a meter
  | { kind: 'id' } //                            a number that means nothing: a color of its own
  | { kind: 'heat'; from: number; to: number } //a scalar between two ends
  | { kind: 'share' } //                         0 to 1, black to white
  | { kind: 'none' };

export interface Field {
  type: FieldType;
  unit: string;
  about: string;
  look: Look;
}

type Options = { unit?: string; look?: Look };
const field =
  (type: FieldType, fallback: Look) =>
  (about: string, options: Options = {}): Field => ({
    type,
    about,
    unit: options.unit ?? '',
    look: options.look ?? fallback,
  });

export const f32 = field('f32', { kind: 'heat', from: 0, to: 1 });
export const u32 = field('u32', { kind: 'id' });
export const i32 = field('i32', { kind: 'id' });
export const vec2f = field('vec2f', { kind: 'none' });
export const vec3f = field('vec3f', { kind: 'light' });
export const vec4f = field('vec4f', { kind: 'none' });
export const vec2u = field('vec2u', { kind: 'none' });
export const vec4u = field('vec4u', { kind: 'none' });

const SIZE: Record<FieldType, number> = {
  f32: 4, u32: 4, i32: 4, vec2f: 8, vec2u: 8, vec3f: 12, vec3u: 12, vec4f: 16, vec4u: 16,
};
const ALIGN: Record<FieldType, number> = {
  f32: 4, u32: 4, i32: 4, vec2f: 8, vec2u: 8, vec3f: 16, vec3u: 16, vec4f: 16, vec4u: 16,
};
export const componentsOf = (type: FieldType): number => SIZE[type] / 4;
export const scalarOf = (type: FieldType): Scalar =>
  type === 'i32' ? 'i32' : type === 'u32' || type.endsWith('u') ? 'u32' : 'f32';

export interface Struct {
  name: string;
  fields: Record<string, Field>;
  offsets: Record<string, number>;
  /** bytes from one element of an array to the next */
  stride: number;
  wgsl: string;
}

const alignUp = (value: number, to: number) => Math.ceil(value / to) * to;

export function struct(name: string, fields: Record<string, Field>): Struct {
  const offsets: Record<string, number> = {};
  let end = 0;
  let align = 4;
  for (const [key, { type }] of Object.entries(fields)) {
    offsets[key] = alignUp(end, ALIGN[type]);
    end = offsets[key]! + SIZE[type];
    align = Math.max(align, ALIGN[type]);
  }
  const lines = Object.entries(fields).map(
    ([key, f]) => `  ${key}: ${f.type},${f.about ? ` // ${f.unit ? f.unit + ': ' : ''}${f.about}` : ''}`,
  );
  return {
    name, fields, offsets,
    stride: alignUp(end, align),
    wgsl: `struct ${name} {\n${lines.join('\n')}\n}`,
  };
}

export type Value = number | number[];

/** One element, read out of raw bytes. */
export function unpack(s: Struct, bytes: DataView, base = 0): Record<string, Value> {
  const out: Record<string, Value> = {};
  for (const [key, { type }] of Object.entries(s.fields)) {
    const scalar = scalarOf(type);
    const read = (at: number) =>
      scalar === 'f32' ? bytes.getFloat32(at, true)
      : scalar === 'u32' ? bytes.getUint32(at, true)
      : bytes.getInt32(at, true);
    const at = base + s.offsets[key]!;
    const n = componentsOf(type);
    out[key] = n === 1 ? read(at) : Array.from({ length: n }, (_, i) => read(at + 4 * i));
  }
  return out;
}

/** One element, written into raw bytes. Fields left out stay zero. */
export function pack(s: Struct, into: DataView, base: number, values: Record<string, Value>): void {
  for (const [key, value] of Object.entries(values)) {
    const f = s.fields[key];
    if (!f) throw new Error(`${s.name} has no field ${key}`);
    const scalar = scalarOf(f.type);
    const list = typeof value === 'number' ? [value] : value;
    list.forEach((v, i) => {
      const at = base + s.offsets[key]! + 4 * i;
      if (scalar === 'f32') into.setFloat32(at, v, true);
      else if (scalar === 'u32') into.setUint32(at, v >>> 0, true);
      else into.setInt32(at, v, true);
    });
  }
}

/** Many elements as the bytes of an array. */
export function packAll(s: Struct, rows: Record<string, Value>[]): ArrayBuffer {
  const out = new ArrayBuffer(Math.max(1, rows.length) * s.stride);
  const view = new DataView(out);
  rows.forEach((row, i) => pack(s, view, i * s.stride, row));
  return out;
}
