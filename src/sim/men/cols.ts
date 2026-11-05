// WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as the game reads and writes them.
import type { Vec3 } from '../../math/vec';
import type { MenStore } from './store';

/** the hash of the layout; the module holds the same one (rust/sim/src/men.rs SCHEMA), and store.ts holds the two to each other as a game starts */
export const MEN_SCHEMA = 0x2566cc8;
export const MEN_HEAD = 16;
export const MEN_F64 = ["posX","posY","posZ","velX","velY","velZ","bodyYaw","aimYaw","aimPitch","radius","stepPhase","speed","lastPosX","lastPosY","lastPosZ","speedSm","stepTravel","stepOwed","stepAt","since"] as const;
export const MEN_U32 = ["gen","owner","rank","reason"] as const;
/** the generation column, as an index into the words from the part's start (the store's own) */
export const MEN_GEN = 10244;

/** what every man is: which store his columns are in and his row. Character and Guard extend these, so a column reads as a field */
export class ManCols {
  S!: MenStore; r = -1;
  get pos(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 2], S.d[i + 258], S.d[i + 514]]; }
  set pos(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 2] = v[0]; S.d[i + 258] = v[1]; S.d[i + 514] = v[2]; }
  get posX(): number { const S = this.S; return S.d[S.b8 + 2 + this.r]; } set posX(v: number) { const S = this.S; S.d[S.b8 + 2 + this.r] = v; }
  get posY(): number { const S = this.S; return S.d[S.b8 + 258 + this.r]; } set posY(v: number) { const S = this.S; S.d[S.b8 + 258 + this.r] = v; }
  get posZ(): number { const S = this.S; return S.d[S.b8 + 514 + this.r]; } set posZ(v: number) { const S = this.S; S.d[S.b8 + 514 + this.r] = v; }
  get vel(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 770], S.d[i + 1026], S.d[i + 1282]]; }
  set vel(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 770] = v[0]; S.d[i + 1026] = v[1]; S.d[i + 1282] = v[2]; }
  get velX(): number { const S = this.S; return S.d[S.b8 + 770 + this.r]; } set velX(v: number) { const S = this.S; S.d[S.b8 + 770 + this.r] = v; }
  get velY(): number { const S = this.S; return S.d[S.b8 + 1026 + this.r]; } set velY(v: number) { const S = this.S; S.d[S.b8 + 1026 + this.r] = v; }
  get velZ(): number { const S = this.S; return S.d[S.b8 + 1282 + this.r]; } set velZ(v: number) { const S = this.S; S.d[S.b8 + 1282 + this.r] = v; }
  get bodyYaw(): number { const S = this.S; return S.d[S.b8 + 1538 + this.r]; } set bodyYaw(v: number) { const S = this.S; S.d[S.b8 + 1538 + this.r] = v; }
  get aimYaw(): number { const S = this.S; return S.d[S.b8 + 1794 + this.r]; } set aimYaw(v: number) { const S = this.S; S.d[S.b8 + 1794 + this.r] = v; }
  get aimPitch(): number { const S = this.S; return S.d[S.b8 + 2050 + this.r]; } set aimPitch(v: number) { const S = this.S; S.d[S.b8 + 2050 + this.r] = v; }
  get radius(): number { const S = this.S; return S.d[S.b8 + 2306 + this.r]; } set radius(v: number) { const S = this.S; S.d[S.b8 + 2306 + this.r] = v; }
  /** where his walk stands in its cycle */
  get stepPhase(): number { const S = this.S; return S.d[S.b8 + 2562 + this.r]; } set stepPhase(v: number) { const S = this.S; S.d[S.b8 + 2562 + this.r] = v; }
  /** which rule has him (0: nobody) */
  get owner(): number { const S = this.S; return S.u[S.b4 + 10500 + this.r]; } set owner(v: number) { const S = this.S; S.u[S.b4 + 10500 + this.r] = v; }
  /** how hard that rule holds him against another */
  get rank(): number { const S = this.S; return S.u[S.b4 + 10756 + this.r]; } set rank(v: number) { const S = this.S; S.u[S.b4 + 10756 + this.r] = v; }
  /** why it has him */
  get reason(): number { const S = this.S; return S.u[S.b4 + 11012 + this.r]; } set reason(v: number) { const S = this.S; S.u[S.b4 + 11012 + this.r] = v; }
  /** the game time it took him */
  get since(): number { const S = this.S; return S.d[S.b8 + 4866 + this.r]; } set since(v: number) { const S = this.S; S.d[S.b8 + 4866 + this.r] = v; }
}
/** a Guard record's own columns (a guard, a teammate, a civilian) */
export class GuardCols extends ManCols {
  /** how fast his feet are going (m/s) */
  get speed(): number { const S = this.S; return S.d[S.b8 + 2818 + this.r]; } set speed(v: number) { const S = this.S; S.d[S.b8 + 2818 + this.r] = v; }
  /** where his watchdog last saw him */
  get lastPos(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 3074], S.d[i + 3330], S.d[i + 3586]]; }
  set lastPos(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 3074] = v[0]; S.d[i + 3330] = v[1]; S.d[i + 3586] = v[2]; }
  get lastPosX(): number { const S = this.S; return S.d[S.b8 + 3074 + this.r]; } set lastPosX(v: number) { const S = this.S; S.d[S.b8 + 3074 + this.r] = v; }
  get lastPosY(): number { const S = this.S; return S.d[S.b8 + 3330 + this.r]; } set lastPosY(v: number) { const S = this.S; S.d[S.b8 + 3330 + this.r] = v; }
  get lastPosZ(): number { const S = this.S; return S.d[S.b8 + 3586 + this.r]; } set lastPosZ(v: number) { const S = this.S; S.d[S.b8 + 3586 + this.r] = v; }
}
/** the player's own columns */
export class PlayerCols extends ManCols {
  /** his speed, eased */
  get speedSm(): number { const S = this.S; return S.d[S.b8 + 3842 + this.r]; } set speedSm(v: number) { const S = this.S; S.d[S.b8 + 3842 + this.r] = v; }
  /** how far the walk cycle has come since the last footfall filed, in turns, either way added up, 1 at the most (player.ts FOOTFALL.once; 1 = far enough, which is where it starts, and where it stays while the key has never been on) */
  get stepTravel(): number { const S = this.S; return S.d[S.b8 + 4098 + this.r]; } set stepTravel(v: number) { const S = this.S; S.d[S.b8 + 4098 + this.r] = v; }
  /** how far the walk cycle has come at a footfall's speed since the last edge of it passed or footfall filed, in turns, either way added up (player.ts FOOTFALL.owed: at half a turn a footfall is owed; 0 where it starts, when he slows, after frames the footfall test was not asked, and while the key is down) */
  get stepOwed(): number { const S = this.S; return S.d[S.b8 + 4354 + this.r]; } set stepOwed(v: number) { const S = this.S; S.d[S.b8 + 4354 + this.r] = v; }
  /** the game clock the footfall test was last asked at with the key up (-1: never) */
  get stepAt(): number { const S = this.S; return S.d[S.b8 + 4610 + this.r]; } set stepAt(v: number) { const S = this.S; S.d[S.b8 + 4610 + this.r] = v; }
}
/** the animator's kept state */
export class AnimCols {
  S!: MenStore; r = -1;
}
