// THE BACKEND'S HALF OF A VISIT (src/cloud/cloud.ts), on a page that draws a game it does not hold: on a hosted build that was given one, and only where this browser sends, a few records of the visit, the
// settings and the files the page saves go up. Null on every other build: no id, no timer, no request.
import { buildName } from './buildname';
import type { Engine } from '../engine';
import type { Gpu } from '../gpu/device';
import { RENDER_PASSES } from '../gpu/timer';
import { CALLS, keptRecord } from '../gpu/keep';
import { cloudBoot, type Cloud } from '../cloud/cloud';
import { bootRecord, laterRecord } from '../cloud/loadtime';
import { doorTo } from '../cloud/door';
import { endStage } from '../cloud/telemetry';
import { machineFacts } from '../ui/bench';
import * as boot from '../ui/boot';
import { shaderRowsOn } from '../ui/shaderrows';
import type { Quality } from '../ui/quality';
import { View, Section } from '../draw/sections';
import type { DrawGame } from './drawgame';
import type { Measure } from './measure';
import type { KeptStore } from '../tape/keepstore';

export interface DrawCloudDeps {
  gpu: Gpu; engine: Engine; quality: Quality; game: DrawGame; measure: Measure;
  /** the visit's name, and what it keeps of its run (null: nothing) */
  sid: string; store: KeptStore | null;
  /** the mission, in a word */
  mission: string;
  /** a run's page is nobody's try */
  offBooks: boolean;
  /** a run is in the player's hands · the world holds still under a screen */
  live(): boolean; still(): boolean;
}
export interface DrawCloud { cloud: Cloud; begins(): void; drawn(rawDt: number, gpuMs: number): void }
export function drawCloud(q: URLSearchParams, d: DrawCloudDeps): DrawCloud | null {
  const { gpu, engine, quality, game, measure } = d, S = engine.settings, hz = (): number => quality.displayHzPin || quality.displayHz;
  let build: string | undefined, began = 0, callsAt = NaN, ticksAt = 0;
  // (the run's end as the rules latch it, asked once: the end card comes a beat later)
  let over = false, won = false;
  const end = (): { won: boolean } | null => { const now = !d.offBooks && (game.drawn.u[game.drawn.at[Section.view] + View.run] & 2) !== 0; if (now && !over) won = game.verdict()?.won ?? false; over = now; return now ? { won } : null; };
  /** what the module says, or nothing of a game that is lost */
  const asked = <T>(f: () => T, none: T): T => { try { return game.lost ? none : f(); } catch { return none; } };
  const cloud = cloudBoot(q, { sid: d.sid, store: d.store, build: () => build,
    machine: () => machineFacts(gpu.adapterInfo, gpu.adapterFallback, hz()).now,
    facts: () => { const t = asked(() => game.tally(), null), R = asked(() => d.offBooks ? null : game.verdict(), null);
      return { scenario: d.mission, stage: t?.stage ?? '', live: asked(d.live, false), stats: t?.stats ?? {}, preset: quality.current, scale: S.renderScale, auto: quality.auto, sharp: !!S.sharpDirect, shed: quality.shed, res: [engine.width, engine.height], out: [gpu.canvas.width, gpu.canvas.height], hz: hz(),
        over: R && { kind: t?.kind ?? null, won: R.won, word: R.word, why: R.why, secs: R.secs, approach: R.approach, void: R.void, clean: R.clean, lost: R.lost, of: R.of, foesDown: R.foesDown, foesOf: R.foesOf, ...(R.hostages ? { hostagesAlive: R.hostages.alive, hostagesOf: R.hostages.of } : {}) } }; },
    gpu: () => { const L = gpu.device.limits; return { timestamps: gpu.hasTimestamps, format: gpu.format, features: [...gpu.device.features].sort(), limits: { texture2d: L.maxTextureDimension2D, buffer: L.maxBufferSize, storageBinding: L.maxStorageBufferBindingSize, computeInvocations: L.maxComputeInvocationsPerWorkgroup } }; },
    passes: () => engine.timer.lastFrame, renderPasses: [...RENDER_PASSES],
    rung: () => [`${quality.current} ${Math.round(S.renderScale * 100) / 100} ${engine.width}x${engine.height}${S.sharpDirect ? ' sharp' : ''}${quality.shed ? ' shed' : ''}${quality.auto ? '' : ' pinned'}`, quality.lastChange ?? ''],
    cpu: out => { if (d.still()) { out[2] = NaN; return; } out[0] = measure.rec.cpuUpdate; out[1] = measure.rec.cpuRender; out[2] = performance.now() - began; out[3] = game.ticks - ticksAt; if (CALLS.play) out[4] = CALLS.play.ms - callsAt; },
    boot: last => keptRecord(bootRecord(boot.summary(), performance, last, shaderRowsOn(), location.origin, d.mission)),
    later: have => laterRecord(boot.summary(), have) });
  (window as unknown as { __cloud: unknown }).__cloud = cloud?.hook() ?? null;
  doorTo(cloud);
  if (!cloud) return null;
  void buildName().then(c => { build = c || build; });
  void gpu.device.lost.then(info => { if (info.reason !== 'destroyed') void cloud.telemetry.flush(true); });
  return { cloud, begins: () => { began = performance.now(); ticksAt = game.ticks; callsAt = CALLS.play ? CALLS.play.ms : NaN; },
    drawn: (rawDt, gpuMs) => cloud.frame(rawDt, asked(d.live, false), endStage(d.offBooks ? '' : 'play', asked(end, null)), gpuMs, gpu.device.queue) };
}
