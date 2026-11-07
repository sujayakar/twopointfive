// The host's end of the tick's out events: each makes today's call on today's objects. Written by tools/wire/puppetgen.cjs from out-events.json.
import { trackOf } from './track';
import '../../src/game/game';
import { WEAPONS } from '../../src/game/weapons';
import { recapRound } from '../../src/game/deathrecap';
import { messBurst, messOpened, messWall } from '../../src/game/floormess';
import { bangScorch, bulletMark, chipMark, kickScuff } from '../../src/game/marks';
import { clearSnowBlast } from '../../src/game/snowBlast';
import { clearSnowMarks } from '../../src/game/snowMarks';
import { clearSnowTracks } from '../../src/game/snowTracks';

/** the sound's options as audio.play reads them. `tok` is the rolling chair's voice: game.ts rollTick's `follow` closure, here because a closure cannot cross from Rust */
function playOpts(g: any, e: any): any { const o = e.opts; if (!o) return undefined; if (o.tok === undefined) { const x: any = {}; for (const k of Object.keys(o)) if (o[k] !== undefined) x[k] = o[k]; return x; } const tok = o.tok, r = g.rolls.find((x: any) => x.voice === tok), at = e.pos; return { follow: () => { if (!r || r.voice !== tok) return null; at[0] = r.p.x; at[2] = r.p.z; return { pos: at, k: r.k, t: g.time }; } }; }

/** a checkpoint is the Rust game's, not the puppet's (OUT-EVENTS.md): set this and the two Checkpoints arms call it */
export const OUT_HOST: { checkpoint?: (g: any, what: string, e: any) => void; tap?: (g: any, row: number, args: number[], answer: number[]) => void; wallFeel?: (g: any, at: any) => void } = {};

export const OUT: Record<string, (g: any, e: any) => void> = {
  CheckpointsKeep: (g, e) => { OUT_HOST.checkpoint ? OUT_HOST.checkpoint(g, "keep", e) : g.saves?.keep?.(e.label); },
  CheckpointsRequest: (g, e) => { OUT_HOST.checkpoint ? OUT_HOST.checkpoint(g, "request", e) : g.saves?.request?.(g, e.why, e.label); },
  DecalsAdd: (g, e) => { g.decals?.add?.(e.kind, e.pos, e.normal, e.size, e.opts); },
  DecalsClear: (g, e) => { g.decals?.clear?.(); },
  DecalsTouch: (g, e) => { g.decals?.touch?.(); },
  GameAudioLikeFootstep: (g, e) => { g.audio?.footstep?.(e.pos, e.loudness, e.isPlayer, e.mat); },
  GameAudioLikePlay: (g, e) => { g.audio?.play?.(e.name, e.pos ?? null, e.gain, playOpts(g, e)); },
  GameAudioLikeRain: (g, e) => { g.audio?.rain?.(e.level); },
  HostApiRestartPicture: (g, e) => { g.engine?.restartPicture?.(e.why); },
  MothsClear: (g, e) => { g.moths?.clear?.(); },
  ParticlesEmit: (g, e) => { const P = g.engine.particles; if (!P) return; const was = P.ours; P.ours = e.ours; try { P.emit(e.kind, e.pos, e.dir, e.count, e.o); } finally { P.ours = was; } },
  ParticlesImpulse: (g, e) => { g.engine.particles?.impulse?.(e.pos, e.radius, e.strength, e.o); },
  ParticlesSetHeat: (g, e) => { g.engine.particles?.setHeat?.(e.srcs); },
  ReachKeepChanged: (g, e) => { g.engine.reach?.changed?.(e.rect ?? null, e.hold); },
  SessionRecorderCut: (g, e) => { g.journal?.cut?.(e.why, e.lost); },
  SmokeSystemLikeClearAll: (g, e) => { g.smoke?.clearAll?.(); },
  SmokeSystemLikeEmit: (g, e) => { if (e.e.track) e.e.track = trackOf(g, e.e.track); g.smoke?.emit?.(e.e); },
  SmokeSystemLikeKeepHot: (g, e) => { g.smoke?.keepHot?.(e.heat); },
  SmokeSystemLikeObstaclesChanged: (g, e) => { g.smoke?.obstaclesChanged?.(e.c2, e.r); },
  SmokeSystemLikePush: (g, e) => { g.smoke?.push?.(e.p); },
  WallWearReset: (g, e) => { g.destr.walls.wear?.reset?.(); },
  WallWearRound: (g, e) => { g.destr.walls.wear?.round?.(g, e.box_, e.surf, e.key, e.counted, e.frac, e.hitP, e.dir); },
  RecapRound: (g, e) => { recapRound(g, e.victim, e.wasDown, e.shooter, e.muzzle, e.hit, WEAPONS[e.def], e.blind); },
  MessBurst: (g, e) => { messBurst(g, e.at, e.size); },
  MessOpened: (g, e) => { messOpened(g, e.rect); },
  MessWall: (g, e) => { messWall(g, e.p, e.n, e.thick, e.through); },
  BangScorch: (g, e) => { bangScorch(g, e.at); },
  BulletMark: (g, e) => { bulletMark(g, e.hitP, e.n, e.s ?? null, WEAPONS[e.def]); },
  ChipMark: (g, e) => { chipMark(g, e.p, e.n, e.s ?? null, e.size); },
  KickScuff: (g, e) => { kickScuff(g, e.d, e.from); },
  ClearSnowBlast: (g, e) => { clearSnowBlast(g); },
  ClearSnowMarks: (g, e) => { clearSnowMarks(); },
  ClearSnowTracks: (g, e) => { if (g.snowTracks) clearSnowTracks(g.snowTracks); },
  Tap: (g, e) => { OUT_HOST.tap?.(g, e.row, e.args, e.answer); },
  WallFeel: (g, e) => { OUT_HOST.wallFeel?.(g, e.at); },
};
/** after an apply: every event of the step, in the order the rules made them */
export function dispatchOut(g: any, events: { arm: string; e: any }[]): void { for (const { arm, e } of events) OUT[arm](g, e); }
