// Headless stand-up of the FULL game layer for QA scripts (bun): the real Game — its constructor, update(), doors / props / guards / squad / escalation /
// mission / combat / lighting modules, real Characters + animators + ragdolls off the glTF on disk, real collision / nav / sound propagation — on top of a
// fake WebGPU device. What is faked, and what that means for a script:
//   - the GPU device is a Proxy that accepts any call and returns more of itself: buffers "exist", writeBuffer is a no-op, mapAsync never resolves. So the
//     CPU halves of BoxWorld (packing, grid, CPU raycast for the cursor / lasers) and LightSet (add / flash / update / upload) run for real; nothing renders.
//   - the irradiance probe never answers: player.visibility / irradiance stay whatever the script sets (the light meter is the script's knob, not the
//     renderer's), and every guard's smokeTrans stays 1 (no smoke between anyone).
//   - the smoke solver is a null object (emit / push / clearAll accepted and dropped, inSmokeDomain false); throwables, canisters and stun charges still run
//     their CPU side (fuses, bounces, dazzle, hearing events).
//   - Input is the real class minus its DOM listeners: scripts write keys / pressed / mouse / clicked exactly as main.ts's automation hooks do.
//   - audio is a counting null object (what would have played, by name).
// Usage:  const W = await standUp();  W.step(1/60);  // = game.update + camera + lights + box upload + input.endFrame, as main.ts's step() does
import { resolve } from 'node:path';

export const ROOT = resolve(import.meta.dir, '../..');

// ---- WebGPU enum globals the CPU-side modules reference when they build their (fake) resources
const G = globalThis as any;
G.GPUBufferUsage ??= { MAP_READ: 1, MAP_WRITE: 2, COPY_SRC: 4, COPY_DST: 8, INDEX: 16, VERTEX: 32, UNIFORM: 64, STORAGE: 128, INDIRECT: 256, QUERY_RESOLVE: 512 };
G.GPUTextureUsage ??= { COPY_SRC: 1, COPY_DST: 2, TEXTURE_BINDING: 4, STORAGE_BINDING: 8, RENDER_ATTACHMENT: 16 };
G.GPUShaderStage ??= { VERTEX: 1, FRAGMENT: 2, COMPUTE: 4 };
G.GPUMapMode ??= { READ: 1, WRITE: 2 };

/** An object that is anything you ask of it: every property is another fake, every call returns one, promises never settle (except shader compile info). */
export function fake(name = 'gpu'): any {
  const fn = function () { /* callable target */ };
  return new Proxy(fn, {
    get(_t, p) {
      if (p === 'then') return undefined;                       // never look like a thenable
      if (p === Symbol.toPrimitive) return () => 0;
      if (p === 'toString' || p === Symbol.toStringTag) return () => `[fake ${name}]`;
      if (p === 'getCompilationInfo') return () => Promise.resolve({ messages: [] });
      if (p === 'mapAsync') return () => new Promise(() => { /* never resolves: no readbacks headless */ });
      if (p === 'label') return name;
      return fake(`${name}.${String(p)}`);
    },
    apply() { return fake(`${name}()`); },
    construct() { return fake(`${name}#new`); },
    set() { return true; },
  });
}

const { buildLevel } = await import(`${ROOT}/src/scene/level.ts`);
const { BoxWorld } = await import(`${ROOT}/src/scene/boxes.ts`);
const { LightSet, lightFromDef } = await import(`${ROOT}/src/render/lights.ts`);
const { loadGltfCharacter } = await import(`${ROOT}/src/anim/gltf.ts`);
const { Game } = await import(`${ROOT}/src/game/game.ts`);
const { FollowCamera } = await import(`${ROOT}/src/game/camera.ts`);
const { Input } = await import(`${ROOT}/src/game/input.ts`);
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
let defaultSettings: () => any;
try { defaultSettings = (await import(`${ROOT}/src/engine.ts`)).defaultSettings; }
catch { defaultSettings = () => ({ flashlightRadius: 0.07, emissiveScale: 1 }); }

let chPromise: Promise<any> | null = null;
/** the glTF mannequin, loaded once per process (file:// fetch of public/assets) */
export function loadCharacter(): Promise<any> { return chPromise ??= loadGltfCharacter(`file://${ROOT}/public/assets/ual/AnimationLibrary_Godot_Standard.gltf`); }

export type Vec3 = [number, number, number];

export interface Headless {
  game: any; engine: any; level: any; input: any; cam: any; canvas: { clientWidth: number; clientHeight: number; width: number; height: number };
  audioCounts: Map<string, number>;
  /** the null smoke object the Game was given (emit / push / clearAll dropped) — hand the same one to anything else main.ts constructs with `smoke` (the Demo) */
  smoke: any;
  /** one frame exactly as main.ts steps it (minus rendering): game.update → camera → lights → box upload → input.endFrame. `follow` is what main.ts calls
   *  demoPos: the showcase tour's camera look-at for this frame (Demo.update's return, called BEFORE this) — the camera then follows it and drops the aim lead */
  step(dt: number, follow?: Vec3 | null): void;
  /** put the cursor on a world point (what the automation hooks' __mouse does after projecting) */
  cursorAt(p: [number, number, number]): boolean;
}

/** A fresh level + engine stand-in + Game. Math.random should already be seeded by the caller if determinism matters (Guard / Character constructors draw from it). */
export async function standUp(opts: { uploadEvery?: number } = {}): Promise<Headless> {
  const ch = await loadCharacter();
  const device = fake('device');
  const level = buildLevel();
  const engine: any = { device, sceneLayout: fake('sceneLayout'), skin: null, lights: new LightSet(device), world: new BoxWorld(device), settings: defaultSettings(), effectiveScale: 1 };
  for (const b of level.boxes) engine.world.addStatic(b);
  for (const ld of level.lights) engine.lights.add(lightFromDef(ld));
  const smoke = { emit() { /* dropped */ }, push() { /* dropped */ }, inSmokeDomain() { return false; }, clearAll() { /* nothing */ }, budget() { return 64; } };
  const game = new Game(engine, level, ch, smoke);
  const audioCounts = new Map<string, number>();
  game.audio = { play(n: string) { audioCounts.set(n, (audioCounts.get(n) ?? 0) + 1); }, footstep() { audioCounts.set('footstep', (audioCounts.get('footstep') ?? 0) + 1); } };
  const input: any = Object.create(Input.prototype);
  Object.assign(input, { keys: new Set<string>(), pressed: new Set<string>(), mouseX: 800, mouseY: 500, buttons: 0, clicked: 0, wheel: 0, hasFocus: true, lockExcept: null, suppressClick: false, onLockedInput: null });
  const canvas = { clientWidth: 1600, clientHeight: 1000, width: 1600, height: 1000 };
  const cam = new FollowCamera(); cam.target = v3.copy(level.playerSpawn); cam.aspect = canvas.width / canvas.height; cam.rebuild();
  engine.world.upload();
  let frame = 0; const uploadEvery = Math.max(1, opts.uploadEvery ?? 1);
  const W: Headless = {
    game, engine, level, input, cam, canvas, audioCounts, smoke,
    step(dt: number, follow: Vec3 | null = null) {
      game.update(dt, input, cam, canvas);
      cam.update(dt, follow ?? game.followPos(), follow || game.spectate >= 0 ? null : game.aimPoint);   // (main.ts: demoPos ?? followPos, and no aim lead while the tour frames the shot)
      engine.lights.update(dt, game.time);
      engine.world.dynamics = game.dynBoxes;
      if (frame++ % uploadEvery === 0) engine.world.upload();   // CPU pack + grid (what the cursor's raycast and the lasers read); the GPU writes go nowhere
      input.endFrame();
    },
    cursorAt(p) { const [sx, sy, front] = cam.project(p, canvas.clientWidth, canvas.clientHeight); if (front) { input.mouseX = sx; input.mouseY = sy; } return front; },
  };
  return W;
}
