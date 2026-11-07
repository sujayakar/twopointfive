// WHAT A PLAYER DID, as a tick is handed it (rust/game_face `face_hand`): numbers in this order, then the keys' codes as text.
export const HAND = { buttons: 0, clicked: 1, wheel: 2, mx: 3, my: 4, w: 5, h: 6, pw: 7, ph: 8, vp: 9, ivp: 25, pos: 41, forward: 44, target: 47, yaw: 50, pitch: 51, distance: 52, fov: 53, aspect: 54, aimLead: 55, count: 56 } as const;

/** the host's input, camera and canvas, as a hand is sampled of them */
export interface HandInput { readonly keys: ReadonlySet<string>; readonly pressed: ReadonlySet<string>; buttons: number; clicked: number; wheel: number; mouseX: number; mouseY: number }
export interface HandCamera { viewProj: ArrayLike<number>; invViewProj: ArrayLike<number>; pos: ArrayLike<number>; forward: ArrayLike<number>; target: ArrayLike<number>; yaw: number; pitch: number; distance: number; fov: number; aspect: number; aimLead: number }
export interface HandCanvas { clientWidth: number; clientHeight: number; width: number; height: number }

/** the hand for the next tick, written to `N`; answers the keys' text */
export function sampleHand(N: Float64Array, input: HandInput, cam: HandCamera, canvas: HandCanvas): string {
  N[HAND.buttons] = input.buttons; N[HAND.clicked] = input.clicked; N[HAND.wheel] = input.wheel; N[HAND.mx] = input.mouseX; N[HAND.my] = input.mouseY;
  N[HAND.w] = canvas.clientWidth; N[HAND.h] = canvas.clientHeight; N[HAND.pw] = canvas.width; N[HAND.ph] = canvas.height;
  for (let i = 0; i < 16; i++) { N[HAND.vp + i] = cam.viewProj[i]; N[HAND.ivp + i] = cam.invViewProj[i]; }
  for (let i = 0; i < 3; i++) { N[HAND.pos + i] = cam.pos[i]; N[HAND.forward + i] = cam.forward[i]; N[HAND.target + i] = cam.target[i]; }
  N[HAND.yaw] = cam.yaw; N[HAND.pitch] = cam.pitch; N[HAND.distance] = cam.distance; N[HAND.fov] = cam.fov; N[HAND.aspect] = cam.aspect; N[HAND.aimLead] = cam.aimLead;
  let held = '', down = ''; for (const k of input.keys) held += held ? ` ${k}` : k; for (const k of input.pressed) down += down ? ` ${k}` : k;
  return `${held}\n${down}`;
}
