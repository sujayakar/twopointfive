// The automation / debugging hooks the game's page hangs on `window` (main.ts; docs/agents/qa.md lists them), declared once so it assigns them
// typed instead of through `(window as any)`. Types
// only: nothing here runs, nothing imports it — tsc picks the file up through tsconfig's include glob and merges the block into the global
// Window. Every hook is optional because it only exists once its page has installed it (and the automation-set ones only when a script sets
// them). The many knob-panel hooks (`__wet`, `__glass`, `__froxel` …) share the KnobHook shape: `summary()`, usually `knobs()` / `set({})`, plus
// their own verbs — described structurally here so a script gets completion without this file importing half the game.
import type { Engine } from './engine';
import type { Game } from './game/game';
import type { FollowCamera } from './game/camera';
import type { Demo } from './game/demo';
import type { Input } from './game/input';
import type { SmokeSystem } from './smoke/smoke';
import type { Menu } from './ui/menu';
import type { Quality } from './ui/quality';
import type { AudioEngine } from './audio/audio';
import type { Attract } from './game/attract';
import type { Director } from './game/director';
import type { PerfRecorder } from './ui/perfrec';
import type { BootSummary } from './ui/boot';

/** One WGSL compiler message as __wgsl reports it (line numbers relative to the submitted source, prelude subtracted). */
export interface WgslLintMessage { type: GPUCompilationMessageType; line: number; col: number; msg: string; src: string; }
/** The shape the knob-panel hooks share (`__wet`, `__glass`, `__froxel`, `__rt`, `__gi`, `__dn`, `__taa`, `__sharp`, `__sdfrig`, `__mark`, `__destr` …): a one-line summary,
 *  usually the knob table and a setter taking a partial of it, plus the module's own verbs and read-outs (indexed loosely — each is documented in CLAUDE.md's bullet). */
export interface KnobHook { summary(): string; knobs?(): Record<string, unknown>; set?(patch: Record<string, unknown>): unknown; [verb: string]: unknown; }
/** a shader hot-swap hook: recompile the pass from `src` (or the file the dev server serves under /qa/shader/) into the live page — resolves to the compiler's verdict */
export type HotSwapHook = (src?: string) => Promise<unknown>;

declare global {
  interface Window {
    // ---- shared by the game page (main.ts) and the viewer (viewer.ts)
    __engine?: Engine;
    __cam?: FollowCamera;
    /** render one offscreen frame, read it back and POST a JPEG to the capture sink (tools/shot-server.ts); the viewer's takes no scale */
    __shot?: (name: string, quality?: number, scale?: number) => Promise<string>;

    // ---- game page
    __game?: Game; __demo?: Demo; __input?: Input; __smoke?: SmokeSystem; __menu?: Menu; __audio?: AudioEngine; __quality?: Quality;
    /** n fixed steps of dt with the given key codes held (synthetic input; works with the rAF loop paused) */
    __runFrames?: (n: number, dt?: number, keys?: string[], draw?: boolean) => void;
    /** move the synthetic cursor (CSS px in the canvas); buttons: 1 LMB, 4 RMB held */
    __mouse?: (x: number, y: number, buttons?: number) => void;
    /** fixed camera framing + staging for captures (main.ts): pin = { look, yaw, pitch, dist } | null; opts.park / guards / player */
    __stage?: (pin: { look: [number, number, number]; yaw: number; pitch: number; dist: number } | null, opts?: { park?: boolean; guards?: { i: number; pos: [number, number, number]; yaw?: number; torch?: boolean; aimPitch?: number }[]; player?: { pos?: [number, number, number]; yaw?: number; crouch?: boolean; holstered?: boolean; invisible?: boolean } }) => { pinned: boolean; parked: boolean; guards: number; player: boolean };
    /** edge-trigger a mouse button this frame (0 LMB, 1 MMB, 2 RMB) */
    __click?: (button?: number) => void;
    /** lossless PNG readback to the capture sink (pixel A/B; pair with __pauseLoop) */
    __png?: (name: string) => Promise<string>;
    /** zero the per-frame counters behind jitter / grain / parities and snap the camera, for repeatable scripted captures */
    __resetClock?: () => void;
    /** compile-check WGSL against the live device with the scene passes' prelude ('scene' = world consts + common.wgsl, true | 'lighting' = + lighting_common.wgsl) */
    __wgsl?: (code: string, opts?: { prelude?: boolean | 'scene' | 'lighting'; label?: string }) => Promise<WgslLintMessage[]>;
    /** true: the rAF loop renders nothing and advances nothing — only __runFrames does (?paused sets it at boot) */
    __pauseLoop?: boolean;
    /** multiplier on the game clock (1 = real time) */
    __timeScale?: number;
    /** set by automation: treat the (headless, nominally hidden) pane as visible so quality calibration / sampling run */
    __testVisible?: boolean;
    /** a page check's word for whether somebody is at this window (main.ts `attended`): unset = the browser's own answer */
    __testFocus?: boolean;
    /** the Audio panel's 'force tension' slider is pinning the score intensity (main.ts then leaves it alone) */
    __forcedTension?: boolean;
    /** decals (experiment E1): the panel's buttons by hand — the test pattern in front of Sam, n random interior marks, clear; each returns the count placed */
    __marks?: { test: () => number; random: (n?: number, seed?: number) => number; clear: () => void; bang: (p: [number, number, number]) => void; scuff: (door: string, from?: [number, number, number]) => boolean };
    /** replay tools/qa/mesh-shadow-fixture.json through the production mesh-shadow walk on the GPU and compare with the CPU reference (main.ts; ?paused page) */
    __meshTrace?: (fixture: { digest: string; joints: number[]; rays: number[][]; expect: number[]; nodes: number }) => Promise<unknown>;
    /** resolves once every hook is assigned, the start card is live and every kernel is in, the ones the card does not wait for too (rejects if the boot dies) —
     *  `await window.__ready` is the one line a script needs */
    __ready?: Promise<void>;
    /** the start card is live and every hook is assigned: what a player has, with the later set of shaders perhaps still compiling behind it (gpu/device.ts
     *  PIPE_QUEUE.later). Only a tool that looks at those first frames waits on this and not on `__ready` */
    __started?: boolean;
    /** the boot timeline (ui/boot.ts): marks, pipeline counts, long tasks, frame gaps, first-frame GPU + the one console line */
    __boot?: BootSummary & { line: string };
    /** the numbers behind the HUD's read-outs (main.ts `readout`) */
    __readout?: Record<string, unknown>;
    /** the start card's attract mode (game/attract.ts): .start() / .stop() / .poke(), .active / .live */
    __attract?: Attract;
    /** the level script's DIRECTOR (game/director.ts): .programs, .armed, .problems, .start(name) / .stop(), .run, .log / rowsOf(), .frame, .summary() */
    __director?: Director;
    /** the F9 perf recorder (ui/perfrec.ts) */
    __perfrec?: PerfRecorder;
    __perfsweep?: import('./ui/perfsweep').PerfSweep;
    /** THE list of knob tables (game/knobs.ts): `registry()` every switch table by name (the live objects), `copy()` a deep copy, `back(copy)` puts one back */
    __knobs?: { registry(): Record<string, object>; copy(names?: readonly string[]): Record<string, Record<string, unknown>>; back(copy: Record<string, Record<string, unknown>>): void };
    /** the session journal (game/journal.ts, F10): start / stop / hand the last file to the door again (host/savefile.ts), its state and counters */
    __journal?: { start(opts?: { seed?: number; note?: string }): string; stop(immediate?: boolean): unknown; toggle(): string; save(): string; readonly state: string; readonly file: unknown; readonly stats: Record<string, unknown>; check(f?: unknown): string; summary(): string; [k: string]: unknown };
    /** staging: drop guard i as if shot / struck (killGuard through the real path); false if he is already down */
    __killGuard?: (i: number, how?: 'shot' | 'struck' | 'wounded', zone?: 'side' | 'leg', dir?: [number, number, number]) => boolean;
    /** the animation's life-layer / foot-lock constant tables (character.ts LIFE, FEET) — noise measurements zero their `on` */
    __anim?: { LIFE: Record<string, unknown>; FEET: Record<string, unknown> };
    /** warm the smoke + particle pipelines behind the start card once (false when it already ran) */
    __prewarm?: () => boolean;
    /** the response team by hand: request / perimeter / standDown, .state = Game.qrf */
    __qrf?: { request(why?: string): unknown; perimeter(why?: string): unknown; standDown(): void; readonly state: unknown };
    /** the civilians by hand: list, panic(i, 'panic' | 'phone'), set(on), knobs, stats, log, spots, check, summary */
    __civ?: KnobHook & { panic(i?: number, what?: 'panic' | 'phone'): unknown; set(on: boolean): unknown };
    /** wounds (game/wounds.ts, ?wounds): on, knobs, stats, men() */
    __wounds?: { on: boolean; [k: string]: unknown };
    __wallsnap?: { on: boolean; [k: string]: unknown };
    /** the 2010s issue kit (game/gear.ts, ?oldgear = off): the switch, live */
    __gear?: { on: boolean };
    /** the needs by hand: send(i, kind) / trip(i) / log */
    __needs?: { send(i?: number, kind?: string): unknown; trip(i?: number): unknown; [k: string]: unknown };
    /** checkpoints by hand (game/savestate.ts via main.ts): save / load / slots / newest / export / import / clear / auto / summary / canSave / snapshot */
    __save?: { save(label?: string, force?: boolean): unknown; load(key: string): string | null; slots(): unknown[]; summary(): string; [k: string]: unknown };
    /** destruction by hand (game/destruction.ts): chaos, stats, log, knobs, debris, inventory(), summary(), heat(p), escalation(p), bang(p), burst(p, n, every), reset(), defaults() */
    __destr?: KnobHook;
    /** the SDF character rig's ray queries on the live scene group vs a disc integral (render/rigDebug.ts) */
    __rigTrace?: (rays: number[][]) => Promise<unknown>;
    /** knob panels of the renderer's systems and the game's presentation modules — see KnobHook and each system's CLAUDE.md bullet */
    __wet?: KnobHook; __weather?: KnobHook; __glass?: KnobHook; __froxel?: KnobHook; __particles?: KnobHook; __taa?: KnobHook; __sharp?: KnobHook; __sdfrig?: KnobHook; __mark?: KnobHook; __dn?: KnobHook; __gi?: KnobHook;
    /** the RT-reflection + material knobs (renderer v3 phase 4) on the GAME page */
    __rt?: KnobHook;
    /** shader hot-swap: volumetrics / restir / direct / reflect / gather / froxel / denoise from disk into the live page */
    __hotVol?: HotSwapHook; __hotRestir?: HotSwapHook; __hotDirect?: HotSwapHook; __hotReflect?: HotSwapHook; __hotGather?: HotSwapHook; __hotFroxel?: HotSwapHook; __hotDenoise?: HotSwapHook;
    /** the capture sink's port for this page's __shot / __png (default 5174; a lane's own sink on another) */
    __shotPort?: number;
  }
}
