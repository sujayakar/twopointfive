// THE FIRES OF A PAGE THAT HOLDS NO GAME: the picture's own, reading the rows a tick left. Once a tick what it files, once a drawn frame what the frame is told; on a cut (a load, a
// restart, a rollback) what it holds is another moment's, and it begins again from the rows.
import type { FollowCamera } from '../game/camera';
import type { Particles } from '../render/particles';
import type { SmokeSystem } from '../smoke/smoke';
import type { Engine } from '../engine';
import type { Drawn } from '../draw/drawn';
import type { DrawWorld } from '../draw/world';
import { FirePicture } from './fire';

export class WeatherPicture {
  readonly fire: FirePicture;
  constructor(E: Engine, D: Drawn, _world: DrawWorld, P: Particles | null, smoke: SmokeSystem) { this.fire = new FirePicture(E, D, smoke, P); }
  /** the game is another moment than the one last drawn: the books begin again */
  cut(): void { this.fire.cut(); }
  /** a tick has run, the camera that follows it stepped */
  tick(dt: number, cam: FollowCamera): void { this.fire.tick(dt, cam.focus, []); }
  /** before a frame is drawn */
  frame(cam: FollowCamera): void { this.fire.frame(cam.focus); }
}
