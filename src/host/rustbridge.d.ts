// THE STATE BRIDGE AS THE PAGE NAMES IT (`tps-bridge/<name>`): the generated files of tools/blitz, reached through the bundler (tools/blitz/page-plugin.ts) so
// that the type check of src/ never walks into tools/. Each is what the file of that name exports, as far as src/host/rustgame.ts uses it.
declare module 'tps-bridge/export' {
  export const SCHEMA: any;
  export function exportGame(g: any, knobs: Record<string, unknown>): Uint8Array;
  export function hostHalf(g: any, knobs: Record<string, unknown>, cmds: readonly any[]): Uint8Array;
  export function shownSeat(g: any): number;
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
  export const OUT_HOST: { checkpoint?: (g: any, what: string, e: any) => void; wallFeel?: (g: any, at: [number, number, number]) => void; endCard?: (g: any) => void };
  export function dispatchOut(g: any, events: { arm: string; e: any }[]): void;
}
declare module 'tps-bridge/puppet-lists.json' {
  const lists: { step: string[]; start: string[] };
  export default lists;
}
declare module 'tps-bridge/scan' {
  export function makeScan(g: unknown, knobs: Record<string, unknown>, send: (b: Uint8Array) => void, count: (q: string) => void, settingsOnly?: boolean, opts?: { traps?: boolean; audit?: number; every?: boolean }): { flush(audit?: boolean): void; rebase(audit?: boolean): void; stats(): { settledLeft: number } & Record<string, unknown> };
}
declare module 'tps-bridge/poke' {
  export function makePoke(g: any, knobs: Record<string, unknown>, send: (b: Uint8Array) => void): (obj: unknown, path: string | (string | number)[], value: unknown) => void;
}
declare module 'tps-bridge/ask' {
  export function askNothing(kind: number, name: string): Uint8Array;
  export function installAsks(g: any, knobs: Record<string, unknown>, ask: (kind: number, row: number, seat: number, bytes: Uint8Array) => Uint8Array | number, reapply: () => void): { after(): void; give(S: any, id: string, t?: any, who?: any, o?: any): string | null; refused: { n: number; rows: Record<string, string> } };
}
declare module 'tps-bridge/switches.json' {
  const list: { rows: { row: string; page: unknown; kind: string }[] };
  export default list;
}
declare module 'tps-bridge/surface' {
  export function surfaceHost(H: { g: any; knobs: Record<string, unknown>; surfaceOpen(): boolean; surfaceDispatching(): boolean; surfaceCall(row: number, bytes: Uint8Array, view?: number): Uint8Array } | null, gone?: unknown): void;
  export function crossing(key: string, self: unknown, args: unknown[]): unknown;
}
