// Five-seveN sidearm model: chamber + detachable magazines, staged reload (mag out → fullest spare in → rack if empty).
export interface ReloadEvents { magOut?(rounds: number, dropped: boolean): void; magIn?(rounds: number): void; rack?(): void; done?(): void; }

export class Pistol {
  readonly name = 'FN Five-seveN (suppressed)';
  magCapacity = 10;
  chamber = 1;                       // 0 | 1
  infinite = false;                  // sandbox: the magazine never runs dry (rounds are still cycled so cadence / animation / smoke stay the same)
  mag: number | null = 10;           // rounds in the seated magazine (null = no magazine)
  spare: number[] = [10, 10, 10];    // rounds per spare magazine ("storage")
  lightOn = false;
  reloadT = -1;                      // seconds into the reload, -1 = idle
  readonly reloadDur = 1.6;
  private stage = 0;                 // 0 nothing done, 1 mag out, 2 mag in, 3 racked
  private ev: ReloadEvents = {};
  shotsFired = 0;

  get reloading() { return this.reloadT >= 0; }
  get roundsReady() { return this.chamber + (this.mag ?? 0); }
  canFire() { return (this.chamber === 1 || (this.infinite && this.mag !== null)) && !this.reloading; }   // (infinite: a dry gun comes back to life when the toggle goes on)
  /** Fullest-first view of the spare mags (for the HUD). */
  spareSorted() { return [...this.spare].sort((a, b) => b - a); }

  fire(): boolean {
    if (!this.canFire()) return false;
    this.chamber = 0; this.shotsFired++;
    if (this.infinite && this.mag !== null) this.chamber = 1;                          // bottomless magazine: always feeds, never empties
    else if (this.mag !== null && this.mag > 0) { this.mag--; this.chamber = 1; }   // slide cycles a fresh round
    return true;
  }

  /** Begin a reload if it makes sense (have a spare that improves things). Returns false if refused. */
  startReload(ev: ReloadEvents = {}): boolean {
    if (this.reloading) return false;
    if (this.spare.length === 0) return false;
    const best = Math.max(...this.spare);
    if (this.mag !== null && this.mag >= best && this.chamber === 1) return false;   // nothing to gain
    this.reloadT = 0; this.stage = 0; this.ev = ev;
    return true;
  }
  cancelReload() { this.reloadT = -1; this.stage = 0; }

  update(dt: number) {
    if (!this.reloading) return;
    this.reloadT += dt;
    const t = this.reloadT;
    if (this.stage < 1 && t >= 0.38) {
      // magazine out: empties are dropped on the floor, partials go back to storage (nothing to do if the well is already empty,
      // e.g. a reload that was cancelled by sprinting after the old magazine came out)
      if (this.mag !== null) {
        const out = this.mag; const dropped = out === 0;
        if (!dropped) this.spare.push(out);
        this.mag = null; this.ev.magOut?.(out, dropped);
      }
      this.stage = 1;
    }
    if (this.stage < 2 && t >= 1.02) {
      // fullest spare goes in
      let bi = 0; for (let i = 1; i < this.spare.length; i++) if (this.spare[i] > this.spare[bi]) bi = i;
      this.mag = this.spare.splice(bi, 1)[0] ?? 0; this.stage = 2; this.ev.magIn?.(this.mag);
    }
    if (this.stage < 3 && t >= 1.38) {
      if (this.chamber === 0 && this.mag !== null && this.mag > 0) { this.mag--; this.chamber = 1; this.ev.rack?.(); }
      this.stage = 3;
    }
    if (t >= this.reloadDur) { this.reloadT = -1; this.stage = 0; this.ev.done?.(); }
  }
}

/** OCP: pistol-mounted emitter that knocks out electronics briefly; recharges. */
export class Ocp {
  charge = 1; readonly rechargePerSec = 0.28; readonly cost = 1;
  ready() { return this.charge >= this.cost - 1e-3; }
  use(): boolean { if (!this.ready()) return false; this.charge = 0; return true; }
  update(dt: number) { this.charge = Math.min(1, this.charge + dt * this.rechargePerSec); }
}
