// WHAT A PLAYER DID, as a tick is handed it: his input and the view he has (src/draw/input.ts), and his keys' codes as text.
import { SeatInput, SeatLens, type Doubles } from './input';

/** a tick's hand: its two rows, and the one run of numbers they lie in, the input first */
export interface Hand { readonly all: Float64Array; readonly input: Doubles<'SeatInput'>; readonly lens: Doubles<'SeatLens'> }
/** how many numbers a hand is */
export const HAND_NUMBERS = SeatInput.row + SeatLens.row;
export function newHand(): Hand {
  const all = new Float64Array(HAND_NUMBERS);
  return { all, input: all.subarray(0, SeatInput.row) as unknown as Doubles<'SeatInput'>, lens: all.subarray(SeatInput.row) as unknown as Doubles<'SeatLens'> };
}

/** the host's input, camera and canvas, as a hand is sampled of them */
export interface HandInput { readonly keys: ReadonlySet<string>; readonly pressed: ReadonlySet<string>; buttons: number; clicked: number; wheel: number; mouseX: number; mouseY: number }
export interface HandCamera { viewProj: ArrayLike<number>; invViewProj: ArrayLike<number>; pos: ArrayLike<number>; forward: ArrayLike<number>; target: ArrayLike<number>; yaw: number; pitch: number; distance: number; fov: number; aspect: number; aimLead: number; desiredDistance: number }
export interface HandCanvas { clientWidth: number; clientHeight: number; width: number; height: number }

/** the hand for the next tick, written to `hand`; answers the keys' text */
export function sampleHand(hand: Hand, input: HandInput, cam: HandCamera, canvas: HandCanvas): string {
  const I = hand.input, L = hand.lens;
  I[SeatInput.buttons] = input.buttons; I[SeatInput.clicked] = input.clicked; I[SeatInput.wheel] = input.wheel; I[SeatInput.mx] = input.mouseX; I[SeatInput.my] = input.mouseY;
  I[SeatInput.w] = canvas.clientWidth; I[SeatInput.h] = canvas.clientHeight; I[SeatInput.pw] = canvas.width; I[SeatInput.ph] = canvas.height;
  I[SeatInput.yaw] = cam.yaw; I[SeatInput.pitch] = cam.pitch; I[SeatInput.distance] = cam.distance; I[SeatInput.fov] = cam.fov; I[SeatInput.aspect] = cam.aspect; I[SeatInput.aimLead] = cam.aimLead; L[SeatLens.desired] = cam.desiredDistance;
  for (let i = 0; i < 16; i++) { L[SeatLens.vp + i] = cam.viewProj[i]; L[SeatLens.ivp + i] = cam.invViewProj[i]; }
  for (let i = 0; i < 3; i++) { I[SeatInput.target + i] = cam.target[i]; L[SeatLens.pos + i] = cam.pos[i]; L[SeatLens.forward + i] = cam.forward[i]; }
  let held = '', down = ''; for (const k of input.keys) held += held ? ` ${k}` : k; for (const k of input.pressed) down += down ? ` ${k}` : k;
  return `${held}\n${down}`;
}
