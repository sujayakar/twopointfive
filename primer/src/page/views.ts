// VIEWS. What a panel can show: any field of any buffer, by itself, and the ones written by hand.

import type { Graph, Stage } from '../gpu/graph';
import { Struct, struct, vec4f, type Field } from '../gpu/layout';

export interface View {
  id: string;
  group: string;
  name: string;
  about: string;
  reads: string[];
  uses: string[];
  /** the body of `fn view(p: vec2u) -> vec3f`: p is an output pixel. Linear color out */
  code: string;
}

const Shown: Struct = struct('Shown', { color: vec4f('linear, 0 to 1') });

const drawn = (field: Field, value: string): string | null => {
  const look = field.look;
  switch (look.kind) {
    case 'color': return value;
    case 'light': return `show_light(${value})`;
    case 'direction': return `show_direction(${value})`;
    case 'place': return `show_place(${value})`;
    case 'id': return `show_id(u32(${value}))`;
    case 'share': return `vec3f(${value})`;
    case 'heat': return `show_heat((${value} - ${look.from.toFixed(4)}) / ${(look.to - look.from).toFixed(4)})`;
    case 'none': return null;
  }
};
const readable = (name: string) => name.replaceAll('_', ' ');

export class Views {
  private byHand: View[] = [];

  constructor(private graph: Graph) {}

  add(view: Omit<View, 'uses'> & { uses?: string[] }): void {
    this.byHand.push({ ...view, uses: view.uses ?? [] });
  }

  /** every field of every buffer that has a pixel grid */
  private automatic(): View[] {
    const out: View[] = [];
    for (const spec of this.graph.buffers.values()) {
      if (typeof spec.size === 'object' || spec.name.startsWith('view ')) continue;
      for (const [key, field] of Object.entries(spec.struct.fields)) {
        const value = drawn(field, 'v');
        if (!value) continue;
        out.push({
          id: `${spec.name}.${key}`,
          group: `${readable(spec.name)} (${spec.size} size)`,
          name: readable(key),
          about: field.about + (field.unit ? ` (${field.unit})` : ''),
          reads: [spec.name],
          uses: [],
          code: `let v = ${spec.name}_at(at_${spec.size}(p)).${key}; return ${value};`,
        });
      }
    }
    return out;
  }

  all(): View[] {
    return [...this.byHand, ...this.automatic()];
  }
  find(id: string): View | undefined {
    return this.all().find((v) => v.id === id);
  }

  /** the buffer a view's picture is in; its stage is added to the graph when first asked for */
  bufferOf(id: string): string {
    const name = `view ${id}`;
    if (this.graph.buffers.has(name)) return name;
    const view = this.find(id);
    if (!view) throw new Error(`no view named ${id}`);
    this.graph.addBuffer({ name, struct: Shown, size: 'output', about: view.about });
    const stage: Stage = {
      name,
      question: view.about,
      reads: view.reads,
      writes: { shown: name },
      uses: ['show', ...view.uses],
      over: 'output',
      code: `fn view(p: vec2u) -> vec3f { ${view.code} }
fn run(p: vec2u) { put_shown(p, Shown(vec4f(view(p), 1.0))); }`,
    };
    this.graph.addStage(stage);
    return name;
  }
}

/** output pixels to pixels of other sizes */
export const AT_SIZE = `
fn at_render(p: vec2u) -> vec2i { return vec2i((vec2f(p) + 0.5) * vec2f(frame.render_size) / vec2f(frame.output_size)); }
fn at_half(p: vec2u) -> vec2i { return vec2i((vec2f(p) + 0.5) * vec2f(frame.half_size) / vec2f(frame.output_size)); }
fn at_output(p: vec2u) -> vec2i { return vec2i(p); }
`;
