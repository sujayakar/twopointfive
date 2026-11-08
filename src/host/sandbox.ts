// THE SANDBOX ROW AS THIS MACHINE KEEPS IT: god mode · nobody thinks · unlimited ammunition. One store, written one key at a time by whichever screen flipped it (Options ▸ Game, the debugger),
// put on a game once it stands (not on a benchmark's scene address, whose fight is the set's) and over a loaded file's own flags: a load from an earlier run must not switch the
// guards back on under a machine that keeps them off. With no store (a harness) nothing here does anything.
export type SandboxKey = 'god' | 'noAi' | 'ammo';
export const SANDBOX_LS = 'twopointsix.sandbox';
/** what this machine last chose, a key it never chose not there */
export function sandboxKept(): Partial<Record<SandboxKey, boolean>> {
  const out: Partial<Record<SandboxKey, boolean>> = {};
  try { const o: unknown = JSON.parse(localStorage.getItem(SANDBOX_LS) ?? 'null'); if (o && typeof o === 'object') for (const k of ['god', 'noAi', 'ammo'] as const) { const v = (o as Record<string, unknown>)[k]; if (typeof v === 'boolean') out[k] = v; } } catch { /* no store, or junk in it */ }
  return out;
}
/** keep one of the three (the others as they are kept: a show that forces one for its length is no choice of the machine's) */
export function keepSandbox(key: SandboxKey, on: boolean): void { try { localStorage.setItem(SANDBOX_LS, JSON.stringify({ ...sandboxKept(), [key]: on })); } catch { /* no store */ } }
/** AUTOSAVE, kept beside it: on until it is switched off */
const AUTOSAVE_LS = 'twopointsix.autosave';
export function autosaveKept(): boolean { try { return localStorage.getItem(AUTOSAVE_LS) !== '0'; } catch { return true; } }
export function keepAutosave(on: boolean): void { try { localStorage.setItem(AUTOSAVE_LS, on ? '1' : '0'); } catch { /* no store */ } }
/** put what is kept on a game; one console line when anything is, so no page runs with nobody thinking silently */
export function loadSandbox(game: { godMode: boolean; aiEnabled: boolean; infiniteAmmo: boolean }, announce = true): void {
  const j = sandboxKept();
  if (j.god !== undefined) game.godMode = j.god; if (j.noAi !== undefined) game.aiEnabled = !j.noAi; if (j.ammo !== undefined) game.infiniteAmmo = j.ammo;
  if (announce && (j.god || j.noAi || j.ammo)) console.log(`sandbox (start card, this machine): ${[j.god && 'god mode', j.noAi && 'NO AI', j.ammo && 'unlimited ammo'].filter(Boolean).join(' · ')}`);
}
