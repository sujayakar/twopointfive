// Audio engine: synthesized SFX with HRTF spatialization, ray-cast occlusion (low-pass + attenuation through the
// same box scene the renderer traces), a reverb send sized by the traced free path around the listener, ambience
// emitters, and the generative score. Starts on the first user gesture (browser autoplay policy).
import { Vec3, v3 } from '../math/vec';
import { StaticCollision } from '../scene/collision';
import { GameAudioLike } from '../game/audio_iface';
import { buildSfx, SfxName } from './sfx';
import { Music } from './music';

interface Voice { src: AudioBufferSourceNode; panner?: PannerNode; filter?: BiquadFilterNode; gain: GainNode; pos?: Vec3; end: number; loop?: boolean; }

export class AudioEngine implements GameAudioLike {
  ctx: AudioContext | null = null;
  private sfx!: Record<SfxName, AudioBuffer>;
  private master!: GainNode; private sfxBus!: GainNode; private revSend!: GainNode; private reverb!: ConvolverNode; private revOut!: GainNode; private ambBus!: GainNode;
  music: Music | null = null;
  private voices: Voice[] = [];
  private listener: Vec3 = [0, 1.6, 0]; private camFwd: Vec3 = [0, 0, -1];
  private lastEnvT = 0;   // when the reverb environment was last re-probed
  volume = { master: 0.8, sfx: 1.0, music: 0.55, ambience: 0.6 };
  started = false; enabled = true;
  private ambience: { pos: Vec3; node: AudioBufferSourceNode; gain: GainNode; panner: PannerNode; filter: BiquadFilterNode; base: number; radius: number; f0: number }[] = [];   // f0 = the source's own low-pass (its timbre), the ceiling for the occlusion filter
  stats = { voices: 0, occluded: 0, routed: 0, wet: 0 };

  constructor(private col: StaticCollision) {}   // col.sound (propagation), col.raycast (reverb sizing) see walls, props and door leaves

  /** Must be called from a user-gesture handler. */
  start() {
    if (this.started) { if (this.ctx && this.ctx.state === 'suspended') void this.ctx.resume(); return; }   // a context created without a qualifying gesture starts suspended: retry on the next gesture
    const AC = (window as any).AudioContext ?? (window as any).webkitAudioContext; if (!AC) return;
    const ctx: AudioContext = new AC({ latencyHint: 'interactive' });
    this.ctx = ctx; this.started = true;
    this.master = ctx.createGain(); this.master.gain.value = this.volume.master;
    const comp = ctx.createDynamicsCompressor(); comp.threshold.value = -14; comp.knee.value = 12; comp.ratio.value = 4; comp.attack.value = 0.004; comp.release.value = 0.2;
    this.master.connect(comp).connect(ctx.destination);
    this.sfxBus = ctx.createGain(); this.sfxBus.gain.value = this.volume.sfx; this.sfxBus.connect(this.master);
    this.ambBus = ctx.createGain(); this.ambBus.gain.value = this.volume.ambience; this.ambBus.connect(this.master);
    // reverb: synthetic IR (decaying filtered noise, slightly different L/R)
    this.reverb = ctx.createConvolver(); this.reverb.buffer = makeIR(ctx, 1.9, 2.8);
    this.revSend = ctx.createGain(); this.revSend.gain.value = 0.3;
    this.revOut = ctx.createGain(); this.revOut.gain.value = 0.8;
    this.revSend.connect(this.reverb).connect(this.revOut).connect(this.master);
    this.sfx = buildSfx(ctx);
    this.music = new Music(ctx, this.master); this.music.setVolume(this.volume.music);
    this.setupAmbience();
  }

  setListener(pos: Vec3, camForward: Vec3) {
    this.listener = v3.copy(pos); this.camFwd = v3.normalize([camForward[0], 0, camForward[2]]);
    const ctx = this.ctx; if (!ctx) return;
    const L = ctx.listener; const t = ctx.currentTime;
    if (L.positionX) { L.positionX.setTargetAtTime(pos[0], t, 0.05); L.positionY.setTargetAtTime(pos[1], t, 0.05); L.positionZ.setTargetAtTime(pos[2], t, 0.05);
      L.forwardX.setTargetAtTime(this.camFwd[0], t, 0.05); L.forwardY.setTargetAtTime(0, t, 0.05); L.forwardZ.setTargetAtTime(this.camFwd[2], t, 0.05); L.upX.value = 0; L.upY.value = 1; L.upZ.value = 0; }
    else { L.setPosition(pos[0], pos[1], pos[2]); L.setOrientation(this.camFwd[0], 0, this.camFwd[2], 0, 1, 0); }
  }

  private static cutoff(muffle: number) { return 20000 * Math.pow(0.045, muffle); }   // 20 kHz clear → ~900 Hz through solid mass

  play(name: string, pos: Vec3 | null, gain = 1, opts: Record<string, unknown> = {}) {
    const ctx = this.ctx; if (!ctx || !this.enabled) return;
    if (name === 'smokeHiss') { this.hiss(pos ?? this.listener, (opts.duration as number) ?? 15); return; }   // a synthesized loop, not a table entry — must be checked before the lookup below
    const buf = this.sfx[name as SfxName]; if (!buf) return;
    const t = ctx.currentTime;
    const src = ctx.createBufferSource(); src.buffer = buf; src.playbackRate.value = (opts.rate as number) ?? (0.96 + Math.random() * 0.08);
    const g = ctx.createGain(); g.gain.value = gain;
    const v: Voice = { src, gain: g, end: t + buf.duration / src.playbackRate.value + 0.1 };
    if (pos) {
      // propagation: straight through air (partial occlusion from side rays softens doorway edges), or — when walled
      // off — around the corners along the walkable space: it then appears to come out of the doorway/bend, at the
      // path's length, muffled a little (a lot if a closed door sits across that doorway); sealed off → through the mass
      const prop = this.col.sound.propagate(pos, this.listener); const at = prop.apparent, occ = prop.muffle;
      if (prop.routed) this.stats.routed++; if (occ > 0.5) this.stats.occluded++;
      const p = ctx.createPanner(); p.panningModel = 'HRTF'; p.distanceModel = 'inverse'; p.refDistance = 1.6; p.rolloffFactor = 1.1; p.maxDistance = 80;
      p.positionX.value = at[0]; p.positionY.value = at[1]; p.positionZ.value = at[2];
      const f = ctx.createBiquadFilter(); f.type = 'lowpass'; f.frequency.value = AudioEngine.cutoff(occ);
      g.gain.value = gain * (1 - 0.55 * occ);
      src.connect(f).connect(p).connect(g);
      g.connect(this.sfxBus);
      // reverb send: more when muffled / routed (you hear the room, not the source) and with distance travelled
      const send = ctx.createGain(); send.gain.value = Math.min(1, 0.25 + 0.5 * occ + prop.pathLen * 0.02);   // revSend's own gain already carries how live the room is
      g.connect(send).connect(this.revSend);
      v.panner = p; v.filter = f; v.pos = v3.copy(pos);
    } else { src.connect(g).connect(this.sfxBus); }
    src.start(t);
    this.voices.push(v);
  }

  footstep(pos: Vec3, loudness: number, isPlayer: boolean) {
    const hard = this.surfaceHard(pos);
    this.play(hard ? 'footHard' : 'footSoft', pos, (isPlayer ? 0.55 : 0.7) * loudness * (hard ? 1 : 0.8), { rate: 0.9 + Math.random() * 0.25 });
  }
  /** lino / stone / asphalt vs carpet, by level regions (matches src/scene/level.ts finishes) */
  private surfaceHard(p: Vec3): boolean {
    const x = p[0], z = p[2];
    if (x < 4 || x > 36 || z < 4 || z > 24) return true;                 // outside: asphalt
    if (z > 10 && z < 12.2) return true;                                   // corridor lino
    if (x < 12 && z > 16) return true;                                     // lobby stone
    if (x > 28 && z > 12.2) return true;                                   // break room lino
    if (x > 14 && x < 22 && z < 10) return true;                           // server room raised floor
    return false;
  }

  private hiss(pos: Vec3, duration: number) {
    const ctx = this.ctx!; const t = ctx.currentTime;
    const src = ctx.createBufferSource(); src.buffer = noiseBuffer(ctx, 2); src.loop = true;
    const f = ctx.createBiquadFilter(); f.type = 'bandpass'; f.frequency.value = 2600; f.Q.value = 0.8;
    const occF = ctx.createBiquadFilter(); occF.type = 'lowpass'; occF.frequency.value = 20000;
    const p = ctx.createPanner(); p.panningModel = 'HRTF'; p.distanceModel = 'inverse'; p.refDistance = 1.2; p.rolloffFactor = 1.3; p.positionX.value = pos[0]; p.positionY.value = pos[1] + 0.1; p.positionZ.value = pos[2];
    const g = ctx.createGain(); g.gain.setValueAtTime(0, t); g.gain.linearRampToValueAtTime(0.5, t + 0.3); g.gain.setValueAtTime(0.5, t + Math.max(0.5, duration - 2)); g.gain.linearRampToValueAtTime(0, t + duration);
    src.connect(f).connect(occF).connect(p).connect(g).connect(this.sfxBus);
    src.start(t); src.stop(t + duration + 0.1);
    this.play('smokePop', pos, 0.9);
    this.voices.push({ src, panner: p, filter: occF, gain: g, pos: v3.copy(pos), end: t + duration + 0.2, loop: true });
  }

  private setupAmbience() {
    const ctx = this.ctx!;
    const mk = (pos: Vec3, kind: 'hum' | 'servers' | 'wind' | 'buzz', base: number, radius: number) => {
      const src = ctx.createBufferSource(); const g = ctx.createGain(); g.gain.value = 0; const f = ctx.createBiquadFilter();
      const p = ctx.createPanner(); p.panningModel = 'HRTF'; p.distanceModel = 'inverse'; p.refDistance = 2; p.rolloffFactor = 1; p.positionX.value = pos[0]; p.positionY.value = pos[1]; p.positionZ.value = pos[2];
      if (kind === 'servers') { src.buffer = toneNoise(ctx, 3, [118, 236, 355], 0.5); f.type = 'lowpass'; f.frequency.value = 1800; }
      else if (kind === 'hum') { src.buffer = toneNoise(ctx, 3, [50, 100, 150], 0.15); f.type = 'lowpass'; f.frequency.value = 600; }
      else if (kind === 'buzz') { src.buffer = toneNoise(ctx, 2, [100, 200, 300, 400], 0.05); f.type = 'bandpass'; f.frequency.value = 240; f.Q.value = 2; }
      else { src.buffer = noiseBuffer(ctx, 4); f.type = 'lowpass'; f.frequency.value = 500; f.Q.value = 0.5; const l = ctx.createOscillator(); l.frequency.value = 0.11; const lg = ctx.createGain(); lg.gain.value = 250; l.connect(lg).connect(f.frequency); l.start(); }
      src.loop = true; src.connect(f).connect(p).connect(g).connect(this.ambBus); src.start();
      this.ambience.push({ pos, node: src, gain: g, panner: p, filter: f, base, radius, f0: f.frequency.value });
    };
    mk([18, 1.2, 7], 'servers', 0.5, 12);        // server room
    mk([29.5, 1.2, 13.2], 'buzz', 0.35, 7);       // vending machines
    mk([16.5, 2.8, 20.6], 'buzz', 0.15, 6);       // flickering panel ballast
    mk([8, 2.6, 18], 'hum', 0.25, 14);            // HVAC over the lobby
    mk([16, 3, 27], 'wind', 0.5, 18);             // outside, south
    mk([1.5, 3, 20], 'wind', 0.4, 14);            // outside, west
  }

  /** Per-frame housekeeping: retire voices, refresh occlusion on loops/ambience, size the reverb by tracing around the listener. */
  update(dt: number, time: number) {
    const ctx = this.ctx; if (!ctx) return;
    const t = ctx.currentTime;
    this.voices = this.voices.filter(v => { if (t > v.end) { try { v.src.disconnect(); v.gain.disconnect(); } catch { /* already gone */ } return false; } return true; });
    this.stats.voices = this.voices.length;
    // environment probe (4×/s): mean free path around the listener → reverb amount; outdoor → less
    if (time - this.lastEnvT > 0.25 || this.lastEnvT === 0) {
      let sum = 0; const N = 10;
      for (let i = 0; i < N; i++) { const a = (i / N) * Math.PI * 2; const hit = this.col.raycast(this.listener, [Math.cos(a), 0, Math.sin(a)], 16); sum += hit ? hit.t : 16; }
      const mfp = sum / N; const outdoors = this.listener[0] < 4 || this.listener[0] > 36 || this.listener[2] < 4 || this.listener[2] > 24;
      const wet = outdoors ? 0.08 : Math.min(0.55, 0.12 + mfp * 0.045);
      this.revSend.gain.setTargetAtTime(wet, t, 0.3); this.stats.wet = wet;
      this.lastEnvT = time || 1e-3;
      // loops (canister hiss) + ambience go through the same propagation as one-shots: re-aim the panner at the apparent position, retune the low-pass
      const move = (p: PannerNode, at: Vec3, k: number) => { p.positionX.setTargetAtTime(at[0], t, k); p.positionY.setTargetAtTime(at[1], t, k); p.positionZ.setTargetAtTime(at[2], t, k); };
      for (const v of this.voices) if (v.loop && v.pos && v.filter && v.panner) { const pr = this.col.sound.propagate(v.pos, this.listener); move(v.panner, pr.apparent, 0.25); v.filter.frequency.setTargetAtTime(AudioEngine.cutoff(pr.muffle), t, 0.2); }
      for (const a of this.ambience) {
        const d = v3.dist(a.pos, this.listener); let g = a.base * Math.max(0, 1 - d / a.radius);
        if (g > 0.01) { const pr = this.col.sound.propagate(a.pos, this.listener); g *= 1 - 0.6 * pr.muffle; move(a.panner, pr.apparent, 0.4); a.filter.frequency.setTargetAtTime(Math.min(a.f0, 300 + AudioEngine.cutoff(pr.muffle)), t, 0.4); }   // (BiquadFilter.frequency.defaultValue is the spec's 350 Hz, not what we set — clamp against the authored f0)
        a.gain.gain.setTargetAtTime(g, t, 0.5);
      }
    }
    void dt;
  }

  setVolumes() {
    if (!this.ctx) return; const t = this.ctx.currentTime;
    this.master.gain.setTargetAtTime(this.enabled ? this.volume.master : 0, t, 0.1);
    this.sfxBus.gain.setTargetAtTime(this.volume.sfx, t, 0.1); this.ambBus.gain.setTargetAtTime(this.volume.ambience, t, 0.1);
    this.music?.setVolume(this.volume.music);
  }
}

// ---- buffer helpers ----
function noiseBuffer(ctx: BaseAudioContext, seconds: number): AudioBuffer {
  const b = ctx.createBuffer(1, Math.floor(ctx.sampleRate * seconds), ctx.sampleRate); const d = b.getChannelData(0);
  for (let i = 0; i < d.length; i++) d[i] = Math.random() * 2 - 1; return b;
}
function toneNoise(ctx: BaseAudioContext, seconds: number, freqs: number[], noiseAmt: number): AudioBuffer {
  const n = Math.floor(ctx.sampleRate * seconds); const b = ctx.createBuffer(1, n, ctx.sampleRate); const d = b.getChannelData(0);
  // make it loop seamlessly: integer cycles over the buffer
  const fs = freqs.map(f => Math.round(f * seconds) / seconds);
  let lpv = 0;
  for (let i = 0; i < n; i++) { const t = i / ctx.sampleRate; let v = 0; fs.forEach((f, k) => { v += Math.sin(2 * Math.PI * f * t + k) / (k + 1); }); lpv += 0.05 * ((Math.random() * 2 - 1) - lpv); d[i] = v * 0.3 + lpv * noiseAmt; }
  return b;
}
function makeIR(ctx: BaseAudioContext, seconds: number, decay: number): AudioBuffer {
  const n = Math.floor(ctx.sampleRate * seconds); const b = ctx.createBuffer(2, n, ctx.sampleRate);
  for (let c = 0; c < 2; c++) { const d = b.getChannelData(c); let lpv = 0; for (let i = 0; i < n; i++) { const t = i / n; lpv += 0.4 * ((Math.random() * 2 - 1) - lpv); d[i] = lpv * Math.pow(1 - t, decay) * (i < 200 ? i / 200 : 1); } }
  return b;
}
