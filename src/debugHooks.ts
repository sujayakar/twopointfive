// The automation / debugging hooks the two pages hang on `window` (the game's in main.ts, the viewer's in viewer.ts — VIEWER.md lists
// those), declared once so both assign them typed instead of through `(window as any)`. Types only: nothing here runs, nothing imports
// it — tsc picks the file up through tsconfig's include glob and merges the block into the global Window. Every hook is optional because
// it only exists once its page has installed it (and the automation-set ones only when a script sets them).
import type { Engine } from './engine';
import type { Game } from './game/game';
import type { FollowCamera } from './game/camera';
import type { Demo } from './game/demo';
import type { Input } from './game/input';
import type { Character, Stance, SignalKind } from './game/character';
import type { Rig } from './game/rigProps';
import type { SmokeSystem } from './smoke/smoke';
import type { Menu } from './ui/menu';
import type { Quality } from './ui/quality';
import type { AudioEngine } from './audio/audio';
import type { RigGizmo } from './viewerGizmo';

/** One WGSL compiler message as __wgsl reports it (line numbers relative to the submitted source, prelude subtracted). */
export interface WgslLintMessage { type: GPUCompilationMessageType; line: number; col: number; msg: string; src: string; }

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
    __runFrames?: (n: number, dt?: number, keys?: string[]) => void;
    /** move the synthetic cursor (CSS px in the canvas); buttons: 1 LMB, 4 RMB held */
    __mouse?: (x: number, y: number, buttons?: number) => void;
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
    /** the Audio panel's 'force tension' slider is pinning the score intensity (main.ts then leaves it alone) */
    __forcedTension?: boolean;

    // ---- viewer
    /** the viewer's panel state object (drive / kit / stance selections …) */
    __viewer?: unknown;
    /** the mannequin (a getter: respawn replaces the Character) */
    __char?: Character;
    __rig?: Rig; __gizmo?: RigGizmo; __rigSource?: () => string;
    /** advance the viewer n frames of 1/60 s */
    __step?: (n?: number) => void;
    /** toss a dropped item from the hand onto the stage */
    __toss?: (kind: 'pistol' | 'torch' | 'smoke' | 'flash' | 'mag', suppressed?: boolean) => void;
    __cans?: { smokeCan(): void; stunCan(): void };
    /** the viewer's running clock (what its __shot renders with) */
    __vtime?: number;
    /** scripted stills of the procedural tactical layer: __tac.stance('stack', -1); __tac.kick(); __tac.at(0.38, 'kick') … */
    __tac?: {
      stance(name: Stance, side?: -1 | 1): void; kick(): void; signal(kind: SignalKind): void; flinch(): void;
      lockpick(on: boolean): void; loop(on: boolean, shot?: number, every?: number): void;
      at(t: number, shot?: 'kick' | SignalKind | 'flinch'): string; resume(): void;
    };
    /** the viewer's grab / hold section (holder or held man alone, arm or gun variant): __hold.set({ on: true, who: 0, variant: 1, phase: 3, auto: false, t: 0.6 }) */
    __hold?: { set(o: Record<string, unknown>): unknown; HOLD: unknown };
  }
}
