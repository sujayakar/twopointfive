// THE STATE BRIDGE AS THE PAGE NAMES IT (`tps-bridge/<name>`): the generated files of tools/blitz, reached through the bundler (tools/blitz/page-plugin.ts) so
// that the type check of src/ never walks into tools/. Each is what the file of that name exports, as far as src/host/rustgame.ts uses it.
declare module 'tps-bridge/export' {
  export const SCHEMA: any;
  export function exportGame(g: any, knobs: Record<string, unknown>): Uint8Array;
  export function hostHalf(g: any, knobs: Record<string, unknown>, cmds: readonly any[]): Uint8Array;
  export function bookOf(g: object): any;
  export function report(): Map<string, { n: number; eg: string }>;
}
declare module 'tps-bridge/import' {
  export function applyGame(g: any, blob: Uint8Array, knobs: Record<string, unknown>, opts?: any): any;
}
declare module 'tps-bridge/classes' {
  export function classTable(): Record<string, { prototype: object }>;
  export function valueTable(): Record<string, any>;
}
declare module 'tps-bridge/puppet-apply' {
  export function puppetRows(ctx: any): any;
  export const SLOW: { type: string; field: string }[];
  export const VALS: string[];
}
declare module 'tps-bridge/out-dispatch' {
  export const OUT: Record<string, (g: any, e: any) => void>;
  export function dispatchOut(g: any, events: { arm: string; e: any }[]): void;
}
declare module 'tps-bridge/puppet-lists.json' {
  const lists: { step: string[]; start: string[] };
  export default lists;
}
