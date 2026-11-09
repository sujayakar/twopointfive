// THE WEATHER, THE AIR AND THE FIRES OF A PAGE THAT HOLDS NO GAME: each the picture's own, each reading the rows a tick left. Once a tick what it files, once a drawn frame what the frame
// is told; on a cut (a load, a restart, a rollback) what it holds is another moment's, and it begins again from the rows.
import type { FollowCamera } from '../host/camera';
import type { Particles } from '../render/particles';
import type { SmokeSystem } from '../smoke/smoke';
import type { Engine } from '../engine';
import type { Drawn } from '../draw/drawn';
import { DrawWinds } from '../draw/winds';
import type { DrawWorld } from '../draw/world';
import { snowConfig } from '../render/frame';
import { Reset, Section, View } from '../draw/sections';
import { AirPicture } from './air';
import { FirePicture } from './fire';
import { RainPicture, Roofs } from './rain';
import { SnowPicture } from './snow/snow';
import { skyFrom } from '../draw/sky';

const LEVEL = 1, SMOKE = 2, CANS = 32;

export class WeatherPicture {
  readonly winds: DrawWinds; readonly air: AirPicture; readonly fire: FirePicture; readonly rain: RainPicture; readonly roofs: Roofs; readonly snow: SnowPicture | null;
  private said = -1;
  constructor(private E: Engine, private D: Drawn, world: DrawWorld, private P: Particles | null, private smoke: SmokeSystem) {
    this.winds = new DrawWinds(D, P, smoke); this.roofs = new Roofs(D);
    this.air = new AirPicture(E, D, world.statics, this.winds);
    this.fire = new FirePicture(E, D, smoke, P);
    this.rain = new RainPicture(E, D, P, world.statics, this.roofs);
    this.from();
    this.snow = snowConfig.on ? new SnowPicture(E, D, world.statics, P, smoke, this.air, this.roofs, this.winds) : null;
  }
  private from(): void { this.winds.from(); this.roofs.from(); }
  /** the game is another moment than the one last drawn: the field is told its map again, the books begin again */
  cut(): void { const D = this.D; this.said = D.n[Section.resets] ? D.u[D.at[Section.resets] + Reset.tick] : -1; this.air.cut(); this.fire.cut(); this.rain.cut(); this.snow?.cut(); }
  /** what the game said goes this tick, before anything of the tick is played: the tick's own order */
  told(): void {
    const D = this.D;
    if (D.n[Section.resets]) { const o = D.at[Section.resets], id = D.u[o + Reset.tick]; if (id !== this.said) { const w = D.u[o + Reset.what];
      if (w & LEVEL) { this.E.restartPicture('level restarted'); this.P?.clear(); this.cut(); }
      if (w & SMOKE) this.smoke.anew();
      if (w & CANS) this.smoke.shutCans();
      this.snow?.reset(w); this.said = id; } }
  }
  /** a tick has run, the camera that follows it stepped */
  tick(dt: number, cam: FollowCamera): void {
    const D = this.D, time = D.u[D.at[Section.view] + View.tick] / 60;
    this.from(); skyFrom(this.E, D);
    if (this.snow) this.snow.tick(dt, cam); else this.rain.tick(dt, cam);
    this.fire.tick(dt, cam.focus, this.winds.heat.read());
    this.air.tick(dt, time, cam.focus);
  }
  /** before a frame is drawn, the sky's section just written over the frame's weather */
  frame(cam: FollowCamera): void { if (this.snow) this.snow.dress(); else this.rain.dress(); this.fire.frame(cam.focus); }
}
