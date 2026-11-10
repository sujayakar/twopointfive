// Audio engine: synthesized SFX with HRTF spatialization, ray-cast occlusion (low-pass + attenuation through the
// same box scene the renderer traces), a reverb send sized by the traced free path around the listener, ambience
// emitters, and the generative score. Starts on the first user gesture (browser autoplay policy).
// Since the native-engine seams the GRAPH lives behind an AudioBackend (src/audio/backend.ts): WebAudioBackend (src/audio/webaudio.ts) is
// today's node graph, node for node; a native host hands another. What stays HERE is the game's half — the voice budget, the propagation through the walls, the ambience emitters'
// placement and levels, the reverb sized by the traced free path, the footstep material — exactly as it was.
import type { WorldAsk } from '../view/worldask';
import { Vec3, v3 } from '../math/vec';
import type { GameAudioLike } from './iface';
import { FLOOR, type FloorMat } from './floors';
import { buildSfx, buildSfxLater, ROLL_LOOP, FIRE_LOOP, SfxName } from './sfx';
import { buildSfxWas, wasInGame, wasOnly, WAS_IN_GAME } from './sfxWas';
import { Music, WebAudioMusicBackend } from './music';
import { sysRandom } from '../math/dice';   // the mixer / the solver draw the SYSTEM die (math/dice.ts)
import type { AudioBackend, BackendVoice, SfxBuffer, SfxContext } from './backend';
import { WebAudioBackend, noiseBuffer } from './webaudio';
import { sfxContext } from './backend';
import { AMBIENCE, AMBIENCE_DISTANCE, AmbienceKind, HISS } from './loops';
import { MAX_DIST, REF_DIST, ROLLOFF, VOLUME_DEFAULTS, cutoffOf, distanceGain, hrtfFor, occludedGain, sendOf } from './mixlaw';

interface Voice { v: BackendVoice; positional: boolean; /** made by `again()`: the lanes' re-listen, faded apart from the tape's own voices */ again?: boolean; pos?: Vec3; end: number; /** estimated level at the listener when it started (positional one-shots): what the voice cap steals by */ level?: number; /** stopped, and fading out */ dying?: boolean;
  /** a voice HELD for a thing that sounds for as long as it goes on */ held?: Held; }
/** A THING THAT SOUNDS FOR AS LONG AS IT GOES ON, as the game has it now (rust/game_draw Lasting): which one, where, how loud, how hard it goes (0‥1), how long it has gone on and how much longer it
 *  will (s; Infinity = nobody knows). Told whole every tick and whenever the game jumps: no event begins or ends one, so a load, a seek or a join is heard as it stands. */
export interface LastingSound { name: string; id: number; pos: Vec3; gain: number; hard: number; age: number; left: number; /** how far off there is nothing of it (0 = as far as it carries) */ reach: number }
interface Going { of: LastingSound; voice: Voice | null; /** the most the tap was told of it (−1 = nothing) */ told: -1 | 0 | 1 | 2; culled?: boolean }
interface Held { of: LastingSound; rate: number; occ: number }
/** what each is held by: the stretch of its buffer that repeats (a take shorter than it loops whole), or a loop made here under a filter of its own; its level and pitch by how it goes. `room` = what hums in
 *  a room: on the ambience bus, held wherever the listener is, going on under a card */
interface LastingKind { loopAt?: { start: number; end: number }; made?: (sc: SfxContext) => SfxBuffer; filter?: { type: 'lowpass' | 'bandpass'; freq: number; Q?: number }; wobble?: { rateHz: number; depthHz: number }; distance?: { ref: number; rolloff: number }; room?: boolean; gain: (r: LastingSound) => number; rate: (r: LastingSound) => number }
const unit = (k: number) => Math.min(1, Math.max(0, k));
/** quieter and lower as it slows, so a chair coasting to rest winds down instead of cutting off; a thing that goes at nothing makes no sound */
const byHard = { gain: (r: LastingSound) => Math.min(1, unit(r.hard) / 0.08) * (0.3 + 0.7 * unit(r.hard)), rate: (r: LastingSound) => 0.8 + 0.3 * unit(r.hard) };
const hum = (kind: AmbienceKind): LastingKind => ({ ...AMBIENCE[kind], distance: AMBIENCE_DISTANCE, room: true, gain: r => r.hard, rate: () => 1 });
const LASTING: Record<string, LastingKind> = {
  chairRoll: { loopAt: ROLL_LOOP, ...byHard },
  fireBed: { loopAt: FIRE_LOOP, ...byHard },
  smokeHiss: { made: sc => noiseBuffer(sc, HISS.seconds), filter: HISS.filter, distance: HISS.distance, gain: r => HISS.peak * unit(r.age / HISS.rise) * unit(r.left / HISS.tail), rate: () => 1 },
  hum: hum('hum'), servers: hum('servers'), buzz: hum('buzz'), wind: hum('wind'),
};
const MAX_LASTING = 8, LASTING_FADE_MS = 140;   // held at once, the nearest (a room's hums are over and above) · the fade of one whose thing has stopped
/** the game's clock moves by its steps: anything else is another moment of it */
const CLOCK_STEP = 0.5;
/** frames stop coming (a lost game, a red card, a long seek) and what is held is silent this many ms on */
const UNHEARD_MS = 500;

/** THE VOICE BUDGET (W8.6). The loud minute — four QRF carbines, the shift's pistols, Sam's rifle, impacts, brass, doors — asked for 40–80 positional voices alive at
 *  once, every one an HRTF panner (a pair of convolutions) behind a propagate() through the walls, most of them 30 m and three rooms away at −50 dB under the compressor.
 *  So: a positional one-shot whose level at the listener (its gain × the occlusion cut × the panner's own inverse-distance law) is under CULL_LEVEL is not played at
 *  all; at most MAX_POSITIONAL positional one-shots live at once (a newcomer louder than the quietest steals its place, a quieter one is dropped); and only the near or
 *  loud ones get the HRTF model — the far and faint are panned 'equalpower', which costs next to nothing and, that far down, sounds the same. Held voices (a can's hiss)
 *  and non-positional sounds (stings, thunder, UI) are outside all three. */
const MAX_POSITIONAL = 24;
const CULL_LEVEL = 0.0056;                    // ≈ −45 dB re a gain-1 sound at the reference distance

export { VOLUME_DEFAULTS };   // (audio/mixlaw.ts: the levels a fresh page has, and the law for one positional voice — the listening page reads the same)

/** which table of effects the mixer loads, read once when the audio starts. As shipped: the redone table (audio/sfx.ts) with the voices the maintainer picked as
 *  BETTER BEFORE laid over it from the old one (audio/sfxWas.ts `WAS_IN_GAME`: his votes of 10-12 — the redone ones stay in sfx.ts, on the listening page, as 'after').
 *  `was` = all of the old table (the page's `?oldsounds`), `redone` = all of the redone one (`?newsounds`) — for hearing either whole in play. The mixer's ears
 *  only: nothing the game hears with reads it. */
export const SFX_TABLE = { was: false, redone: false };
/** the table the game plays (a voice added since the old table was frozen keeps its own) */
export function gameSfx(ctx: SfxContext): Record<SfxName, SfxBuffer> {
  const t = buildSfx(ctx);
  return SFX_TABLE.was ? { ...t, ...buildSfxWas(ctx) } : SFX_TABLE.redone ? t : { ...t, ...wasInGame(ctx) };
}

/** THE SOUNDS ARE MADE BEFORE THE FIRST CLICK, ONE AT A TIME. The mixer may not start until the visitor clicks or presses a key, and it used to synthesise its
 *  whole table inside that first click: eighty voices, most of a second with the page stopped — heard as a hitch just as the sound comes in (a play test,
 *  10-19: "a hitch in the audio right when it gets started"). Now the page asks for the table as soon as it is up (`warmSfx`), a voice at a time in the browser's
 *  idle moments while the shaders compile; the first click then only hands the mixer what is made. A voice asked for before its turn is made on the spot, the
 *  rest carry on in idle moments. The samples are the same ones (each voice has its own seed: sfx.ts `lazy`), and the voices the old table gives the game
 *  (sfxWas.ts WAS_IN_GAME) are no longer made twice. `?sfxwarm=0` = the whole table inside the first click, as it was.
 *  THE RATE: a page cannot ask what its output runs at before the click, so the warm-up makes the table at the rate the mixer last started at in this browser
 *  (`rateKey`, written at every start), `rate` on a first visit. Where the guess is wrong, what was made is dropped at the click and the table is made at the
 *  device's rate the same way, a voice at a time — that page's first sounds each pay for their own making, and the next page guesses right.
 *  ONE MIXER: the warm-up feeds the first mixer that starts; another on the same page makes its own table whole, as it was. */
export const SFX_WARM = { on: true, rate: 48000, rateKey: 'twopointsix.sfxRate' };
try { if (new URLSearchParams(location.search).get('sfxwarm') === '0') SFX_WARM.on = false; } catch { /* no page */ }
/** gameSfx's table, no voice rendered: name → the call that renders it (the same samples; a WAS_IN_GAME voice is the old table's and the redone one is never made) */
export function gameSfxLater(ctx: SfxContext): Map<string, () => SfxBuffer> {
  const t: Map<string, () => SfxBuffer> = buildSfxLater(ctx);
  if (!SFX_TABLE.redone) for (const n of WAS_IN_GAME) { const redone = t.get(n); let b: SfxBuffer | null = null; t.set(n, () => b ??= (wasOnly(ctx, [n])[n] as SfxBuffer | undefined) ?? redone!()); }
  return t;
}
/** when the page has a moment: the browser's idle callback where there is one (a slice between frames, or all of a quiet stretch — `left` = the ms it has, and
 *  half a second gone by with no such moment is one anyway), a short timer where not */
type Idle = { didTimeout: boolean; timeRemaining(): number };
const whenIdle = (fn: (left: number) => void) => { const ric = (globalThis as { requestIdleCallback?: (f: (d: Idle) => void, o?: { timeout: number }) => number }).requestIdleCallback; if (ric) ric(d => fn(d.didTimeout ? Infinity : d.timeRemaining()), { timeout: 500 }); else setTimeout(() => fn(Infinity), 16); };
/** the warm-up's state: the calls still to make (null = not begun), the voices made and not yet handed over, the mixer's loader once one has started */
const warm = { rate: 0, later: null as Map<string, () => SfxBuffer> | null, made: new Map<string, SfxBuffer>(), sink: null as ((name: string, buf: SfxBuffer) => void) | null, pumping: false, slices: 0, longest: 0, total: 0 };
function warmOne(name: string): void {
  const job = warm.later?.get(name); if (!job) return;
  warm.later!.delete(name);
  const t = performance.now(); const buf = job(); const ms = performance.now() - t; warm.longest = Math.max(warm.longest, ms); warm.total += ms; warm.slices++;
  if (warm.sink) warm.sink(name, buf); else warm.made.set(name, buf);
}
function warmPump(): void {
  if (warm.pumping || !warm.later?.size) return;
  warm.pumping = true;
  // one voice a moment, and only in a moment long enough for the usual one (a frame's few spare ms pass it by: the picture is not made to wait for a sound)
  whenIdle(left => { warm.pumping = false; const next = warm.later?.keys().next(); if (next && !next.done && left >= Math.min(12, warm.slices ? warm.total / warm.slices : 4)) warmOne(next.value); warmPump(); });
}
/** the page, once, as soon as it is up: start making the table (nothing under ?sfxwarm=0, or once a mixer has started) */
export function warmSfx(rate = lastRate()): void {
  if (!SFX_WARM.on || SFX_TABLE.was || warm.later || warm.sink) return;
  warm.rate = rate; warm.later = gameSfxLater(sfxContext(rate)); warmPump();
}
/** the rate the mixer last started at in this browser, SFX_WARM.rate where none is kept */
function lastRate(): number { try { const v = Number(localStorage.getItem(SFX_WARM.rateKey)); if (v >= 8000 && v <= 384000) return v; } catch { /* no store */ } return SFX_WARM.rate; }
/** a check's: the warm-up as on a page just opened (a mixer it fed keeps what it has and gets no more) */
export function warmReset(): void { warm.rate = 0; warm.later = null; warm.made.clear(); warm.sink = null; warm.slices = 0; warm.longest = 0; warm.total = 0; }
/** how the warm-up stands (the page check reads it): voices still to make, made and waiting, slices run, the longest one in ms */
export const warmState = () => ({ left: warm.later?.size ?? 0, waiting: warm.made.size, slices: warm.slices, longest: warm.longest, rate: warm.rate, live: !!warm.sink });

/** one sound as the game asked the mixer for it: `dur` = seconds as played (the buffer's length over its rate), `heard` 2 = played, 1 = asked and culled (inaudible, or under a full
 *  budget), 0 = the mixer was off (a muted page, a replay's undrawn seek) */
export interface SoundAsk { name: string; gain: number; dur: number; heard: 0 | 1 | 2; mix?: SoundMix }
/** how one voice was mixed, BEFORE the page's trim (`trims`) — kept by the lanes so `again()` can play that moment once more under whatever is picked now. `rel` = where
 *  it seemed to come from, in the listener's own frame (right, up, ahead; absent = a sound with no place), `gain` after occlusion, `cut` the low-pass in Hz. */
export interface SoundMix { rel?: [number, number, number]; gain: number; rate: number; cut?: number; send?: number; hrtf?: boolean; pan?: number }
/** what the page lays over ONE sound for the mixer's ears: another loaded buffer (`loadTake`), pitch, level, a low-pass in Hz. The game hears with none of it. */
export interface SoundTrim { take?: string; rate?: number; gain?: number; tone?: number }

export class AudioEngine implements GameAudioLike {
  /** the graph (null until start()): the Web Audio one on the page, another host's otherwise */
  backend: AudioBackend | null = null;
  /** the Web Audio context while the backend is the browser's (what the page's hooks / tests looked at) */
  get ctx(): AudioContext | null { return this.backend?.webAudio?.ctx ?? null; }
  music: Music | null = null;
  /** the main menu is up : the score plays the theme alone there (music.ts `menu`) */
  get scoreMenu() { return this.menuUp; }
  set scoreMenu(v: boolean) { this.menuUp = v; if (this.music) this.music.menu = v; }
  private menuUp = false;
  /** the level's own music, as PIECE_NAMES indices (audio/setlist.ts; host/drawpage.ts says it once at boot, before the first gesture makes the score): music.ts `setlist` */
  get scoreSet() { return this.levelSet; }
  set scoreSet(v: readonly number[] | null) { this.levelSet = v; if (this.music) this.music.setlist = v; }
  private levelSet: readonly number[] | null = null;
  private voices: Voice[] = [];
  private listener: Vec3 = [0, 1.6, 0]; private camFwd: Vec3 = [0, 0, -1];
  private lastEnvT = 0;   // when the reverb environment was last re-probed
  volume = { ...VOLUME_DEFAULTS };
  started = false; enabled = true;
  /** is this place under the open sky: how much room a sound is given there (host/drawpage.ts: the roofs the game hands over) */
  outdoors: (x: number, z: number) => boolean = () => false;
  /** told of every sound asked for (the replay's sound lanes: src/tape/soundlanes.ts); null = nobody listens, nothing is built */
  tap: ((a: SoundAsk) => void) | null = null;
  /** per sound name, what plays in its place / how (the replay page's picks); empty = the table as loaded */
  trims = new Map<string, SoundTrim>();
  /** voices = alive now · maxVoices = the most alive at once since start · culled = positional one-shots not played (inaudible, or quieter than a full budget) · stolen = quietest
   *  voices stopped to make room · hrtf / equalpower = positional one-shots by the panner they got · occluded / routed = propagation outcomes · wet = the reverb send now */
  stats = { voices: 0, maxVoices: 0, culled: 0, stolen: 0, hrtf: 0, equalpower: 0, occluded: 0, routed: 0, wet: 0 };

  /** the box world is asked (src/view/worldask.ts) how a sound reaches and how large the room is: walls, props and door leaves; `makeBackend` = the host's graph (default: Web Audio off the page's AudioContext) */
  constructor(private ask: WorldAsk, private makeBackend: (volume: { master: number; sfx: number; ambience: number }) => AudioBackend | null = v => WebAudioBackend.create(v)) {}

  /** Must be called from a user-gesture handler. */
  start() {
    if (this.started) { (this.backend as { resume?: () => void } | null)?.resume?.(); return; }   // a context created without a qualifying gesture starts suspended: retry on the next gesture
    const b = this.makeBackend(this.volume); if (!b) return;
    this.backend = b; this.started = true;
    // a hidden tab draws no frame, so nobody would turn a held voice down as its thing comes to rest: it is silent until the page is back
    if (typeof document !== 'undefined') document.addEventListener('visibilitychange', () => { if (document.hidden) for (const v of this.voices) if (v.held && !v.dying) b.setVoice(v.v, { gain: 0 }, 0.05); });
    try { localStorage.setItem(SFX_WARM.rateKey, String(b.sfxContext.sampleRate)); } catch { /* no store: the next page guesses SFX_WARM.rate */ }
    if (SFX_WARM.on && !SFX_TABLE.was && !warm.sink) {   // the table a voice at a time (SFX_WARM): what is made already goes in now, the rest as it is made or asked for
      if (warm.later && warm.rate !== b.sfxContext.sampleRate) { warm.later = null; warm.made.clear(); }   // (made for another rate than this device's: not what it would play)
      if (!warm.later && !warm.made.size) { warm.rate = b.sfxContext.sampleRate; warm.later = gameSfxLater(b.sfxContext); }
      for (const [n, buf] of warm.made) b.loadBuffer(n, buf);
      warm.made.clear(); warm.sink = (n, buf) => b.loadBuffer(n, buf); warmPump();
    } else {
      const sfx = gameSfx(b.sfxContext);
      for (const n of Object.keys(sfx) as SfxName[]) b.loadBuffer(n, sfx[n]);
    }
    { const mb = b.webAudio ? new WebAudioMusicBackend(b.webAudio.ctx, b.webAudio.master) : b.music?.() ?? null; if (mb) { this.music = new Music(mb); this.music.menu = this.menuUp; this.music.setlist = this.levelSet; this.music.setVolume(this.volume.music); } }   // the score: its sequencer over the page's Web Audio voices, or the backend's own MusicBackend (the native synth) — silent on a host with neither
    // (whatever holds the frames up, nothing held sounds on under it)
    setInterval(() => { if (performance.now() - this.beat > UNHEARD_MS) this.still(0.1); }, UNHEARD_MS / 2);
  }

  /** another take of `name` under an id of its own (`name@take`), for a trim to point at; '' before the mixer has started */
  loadTake(name: string, take: string, buf: SfxBuffer): string {
    const b = this.backend; if (!b) return '';
    const id = `${name}@${take}`; b.loadBuffer(id, buf); return id;
  }

  setListener(pos: Vec3, camForward: Vec3) {
    this.listener = v3.copy(pos); this.camFwd = v3.normalize([camForward[0], 0, camForward[2]]);
    this.backend?.setListener(pos, this.camFwd);
  }

  play(name: string, pos: Vec3 | null, gain = 1, opts: Record<string, unknown> = {}) {
    const b = this.backend, tap = this.tap;
    if (b && (this.enabled || tap)) warmOne(name);   // (SFX_WARM: a voice asked for before its turn is made now — where it will be played, or its length told)
    if (!b || !this.enabled) {   // (the mixer is off: the lanes are still told what was asked for — of a name the table holds, where there is a table to ask, as below)
      if (tap) { const dur = b ? b.duration(name) : 0; if (!b || dur) tap({ name, gain, dur, heard: 0 }); }
      return;
    }
    const trim = this.trims.size ? this.trims.get(name) : undefined;
    const buffer = trim?.take && b.duration(trim.take) ? trim.take : name;
    const dur = b.duration(buffer); if (!dur) return;
    const t = b.now();
    let rate = (opts.rate as number) ?? (0.96 + sysRandom() * 0.08);   // (drawn before any cull below, so a sound culled or not leaves Math.random where it always did)
    const filter = trim?.tone ? { type: 'lowpass' as const, freq: trim.tone } : undefined;
    const gain0 = gain, rate0 = rate;   // (as asked, before the page's trim: what `SoundAsk.mix` carries)
    if (trim) { rate *= trim.rate ?? 1; gain *= trim.gain ?? 1; }
    const ask: SoundAsk | null = tap ? { name, gain, dur: dur / rate, heard: 1 } : null;
    const told = (heard: 0 | 1 | 2) => { if (ask) { ask.heard = heard; tap!(ask); } };
    let v: Voice;
    if (pos) {
      // propagation FIRST — straight through air (partial occlusion from side rays softens doorway edges), or, when walled off, around the corners along the walkable
      // space: it then appears to come out of the doorway/bend, at the path's length, muffled a little (a lot if a closed door sits across that doorway); sealed off →
      // through the mass — because what it says decides whether the sound is worth a voice at all (the budget above)
      const prop = this.ask.hear(pos, this.listener); const at = prop.apparent, occ = prop.muffle;
      const d = v3.dist(at, this.listener);
      const level = occludedGain(gain, occ) * distanceGain(d);   // what reaches the ear: the gain the graph applies × the panner's inverse-distance law at the apparent position
      if ((trim ? occludedGain(gain0, occ) * distanceGain(d) : level) < CULL_LEVEL) { this.stats.culled++; told(1); return; }   // (judged as ASKED: a level dial turned down on a replay page still leaves the lanes a voice to hear again)                                                             // inaudible under the mix: no voice
      let alive = 0, quiet: Voice | null = null;
      for (const w of this.voices) if (w.positional && !w.held && !w.dying && t <= w.end) { alive++; if (!quiet || (w.level ?? 1) < (quiet.level ?? 1)) quiet = w; }
      if (alive >= MAX_POSITIONAL) {
        if (!quiet || (quiet.level ?? 1) >= level) { this.stats.culled++; told(1); return; }                                          // the budget is full of louder sounds: this one would not be missed
        b.stop(quiet.v, 12); quiet.end = t + 0.08; quiet.dying = true; this.stats.stolen++;   // else the quietest goes — a 12 ms fade, not a click; out of the budget this instant, retired by update() once stopped — and this one plays
      }
      if (prop.routed) this.stats.routed++; if (occ > 0.5) this.stats.occluded++;
      const near = hrtfFor(prop.pathLen, level);   // HRTF where it can be heard AS a place — near, or loud; the far and faint take the cheap panner
      if (near) this.stats.hrtf++; else this.stats.equalpower++;
      // the graph: [occlusion low-pass] → panner at the APPARENT position → gain → the sfx bus, + a reverb send: more when muffled / routed (you hear the room, not the source) and with distance travelled
      const bv = b.play({ buffer, pos: at, gain: occludedGain(gain, occ), rate, bus: 'sfx', filter, hrtf: near, distance: { ref: REF_DIST, rolloff: ROLLOFF, max: MAX_DIST }, cutoffHz: cutoffOf(occ), send: sendOf(occ, prop.pathLen) });   // (revSend's own gain already carries how live the room is)
      if (!bv) { told(1); return; }
      v = { v: bv, positional: true, pos: v3.copy(pos), end: t + dur / rate + 0.1, level };
      if (ask) { const L = this.listener, f = this.camFwd, dx = at[0] - L[0], dz = at[2] - L[2];
        ask.mix = { rel: [dz * f[0] - dx * f[2], at[1] - L[1], dx * f[0] + dz * f[2]], gain: occludedGain(gain0, occ), rate: rate0, cut: cutoffOf(occ), send: sendOf(occ, prop.pathLen), hrtf: near }; }
    } else {
      // non-positional (stings, UI, thunder): optionally low-passed (opts.lowpass, Hz — thunder dulls with distance), leaned to a side (opts.pan −1‥1 — toward
      // the storm cell's bearing) and given some room (opts.wet 0‥1); with none of them it is the plain src → gain → bus it always was
      const bv = b.play({ buffer, pos: null, gain, rate, bus: 'sfx', filter, cutoffHz: typeof opts.lowpass === 'number' ? opts.lowpass : undefined, cutoffQ: typeof opts.lowpass === 'number' ? 0.5 : undefined,
        pan: typeof opts.pan === 'number' ? opts.pan : undefined, send: typeof opts.wet === 'number' && opts.wet > 0 ? opts.wet : undefined });
      if (!bv) { told(1); return; }
      v = { v: bv, positional: false, end: t + dur / rate + 0.1 };
      if (ask) ask.mix = { gain: gain0, rate: rate0, cut: typeof opts.lowpass === 'number' ? opts.lowpass : undefined, pan: typeof opts.pan === 'number' ? opts.pan : undefined, send: typeof opts.wet === 'number' && opts.wet > 0 ? opts.wet : undefined };
    }
    this.voices.push(v);
    if (this.voices.length > this.stats.maxVoices) this.stats.maxVoices = this.voices.length;
    told(2);
  }

  /** One voice the lanes kept (`SoundAsk.mix`), played once more as it was mixed then — the same place from the listener, as muffled, as far — under what the page
   *  lays over that sound NOW (`trims`: the take, pitch, tone, level). The replay page's "hear that again": it asks nothing of the level, tells no tap, draws no die,
   *  takes no place in the positional budget, and plays whether the mixer is switched on or not (`enabled`). False = nothing was played. */
  again(name: string, m: SoundMix): boolean {
    const b = this.backend; if (!b) return false;
    warmOne(name);
    const trim = this.trims.get(name);
    const buffer = trim?.take && b.duration(trim.take) ? trim.take : name;
    const dur = b.duration(buffer); if (!dur) return false;
    const rate = m.rate * (trim?.rate ?? 1), gain = m.gain * (trim?.gain ?? 1);
    const filter = trim?.tone ? { type: 'lowpass' as const, freq: trim.tone } : undefined;
    let bv: BackendVoice | null;
    if (m.rel) { const L = this.listener, f = this.camFwd, [r, u, a] = m.rel;
      bv = b.play({ buffer, pos: [L[0] - f[2] * r + f[0] * a, L[1] + u, L[2] + f[0] * r + f[2] * a], gain, rate, bus: 'sfx', filter, hrtf: m.hrtf, distance: { ref: REF_DIST, rolloff: ROLLOFF, max: MAX_DIST }, cutoffHz: m.cut, send: m.send });
    } else bv = b.play({ buffer, pos: null, gain, rate, bus: 'sfx', filter, cutoffHz: m.cut, cutoffQ: m.cut !== undefined ? 0.5 : undefined, pan: m.pan, send: m.send });
    if (!bv) return false;
    this.voices.push({ v: bv, positional: false, again: true, end: b.now() + dur / rate + 0.1 });   // (not `positional`: never counted in play()'s budget nor stolen by it)
    return true;
  }
  /** every one-shot faded out now (the lanes' re-listen starting over or being called off; another moment of the game); `again` = only the voices `again()` made (the tape
   *  plays on under a stretch still ringing: its own new voices stay). A voice is let go only once the fade has run out: the backend's stop is a time constant. A held voice is silent until the next
   *  frame: what it is held for says whether it goes on. */
  hush(ms = 30, again = false) {
    const b = this.backend; if (!b) return; const t = b.now();
    if (!again) this.still(ms / 1000);
    for (const v of this.voices) if (!v.held && !v.dying && t <= v.end && (!again || v.again)) { b.stop(v.v, ms); v.end = t + 5 * ms / 1000 + 0.02; v.dying = true; }
  }

  /** The rain bed (weather.ts calls this every frame with the level it wants, 0 = off): a loop of noise shaped into the hiss of rain on asphalt over the drum of it on
   *  the roof and the cars, built by the backend on first use, its gain eased toward `level` × its own ceiling on the ambience bus (src/audio/webaudio.ts rainLevel). */
  rain(level: number) { this.backend?.rainLevel(level); }

  /** One footfall. The floor it lands on comes from the shared material map (rust/game_rules/src/world/floor.rs — the same map the guards' ears and the SOUND meter read; `mat` arrives
   *  resolved by the caller, rust/game_rules/src/world/floor.rs tread, so the mixer cannot disagree with them): hard finishes get the heel click, carpet and grass the soft scuff, and the
   *  sample runs a touch brighter on stone / duller on carpet. `loudness` already carries the material's share (carpet 0.6 ×, stone 1.1 × …), so it goes straight to the gain. */
  footstep(pos: Vec3, loudness: number, isPlayer: boolean, mat: FloorMat) {
    const f = FLOOR[mat];
    this.play(f.hard ? 'footHard' : 'footSoft', pos, (isPlayer ? 0.55 : 0.7) * loudness, { rate: (0.9 + sysRandom() * 0.25) * f.rate });
  }

  private going = new Map<string, Going>();   // what sounds for as long as it goes on, by which it is
  private clock = NaN; private moved = -1; private jumped = true; private beat = 0;   // the game's clock as last told, and the mixer's when it last moved by a step; a jump since the last frame; when a frame was last heard
  /** THE THINGS THAT SOUND FOR AS LONG AS THEY GO ON, all of them, as the game stands at `clock`: one that is here no more fades out, one that is new is held from the next frame on. A clock that stands
   *  still (a pause, a frozen card) is a thing that makes no sound. The tap is told as each begins, and again where it is heard that was not. */
  lasting(rows: readonly LastingSound[], clock: number) {
    if (rows.length || this.going.size) {
      const was = this.going; this.going = new Map();
      for (const of of rows) { const key = `${of.name} ${of.id}`, g = was.get(key); was.delete(key); if (g) { g.of = of; if (g.voice) g.voice.held!.of = of; } this.going.set(key, g ?? { of, voice: null, told: -1 }); }
      for (const g of was.values()) if (g.voice) this.letGo(g.voice);
    }
    const b = this.backend, step = clock - this.clock; this.clock = clock;
    if (!(step >= 0 && step < CLOCK_STEP)) { this.jumped = true; this.moved = -1; } else if (step > 0 && b) this.moved = b.now();
    if (!b || !this.enabled) for (const g of this.going.values()) if (!g.voice) this.tell(g, 0);
  }
  private tell(g: Going, heard: 0 | 1 | 2, mix?: SoundMix) {
    if (!this.tap || heard <= g.told) return;
    const b = this.backend, kind = LASTING[g.of.name]; if (kind?.room) return;   // (a room's hum is nobody's doing: no lane of it)
    if (b && !kind?.made) warmOne(g.of.name);
    const dur = Number.isFinite(g.of.left) ? g.of.left : b && !kind?.made ? b.duration(g.of.name) : 0; if (b && !dur) return;
    g.told = heard; this.tap({ name: g.of.name, gain: g.of.gain, dur, heard, mix });
  }
  /** every held voice silent, until a frame says otherwise */
  private still(secs: number) { for (const v of this.voices) if (v.held && !v.dying) this.backend!.setVoice(v.v, { gain: 0 }, secs); }
  /** a held voice let go: faded out, kept in the list until the fade is done */
  private letGo(v: Voice) { if (v.dying) return; this.backend!.stop(v.v, LASTING_FADE_MS); v.dying = true; v.end = this.backend!.now() + 6 * LASTING_FADE_MS / 1000; for (const g of this.going.values()) if (g.voice === v) g.voice = null; }
  /** what goes on and is not held yet is held, the nearest of it: ONE looping voice a thing, on its buffer's loop stretch, outside the one-shots' budget (a full house gives its FARTHEST voice to a thing
   *  well nearer). `update()` reads the thing's row every frame: level and pitch ride it, the panner, the low-pass and the room's share four times a second. */
  private hold() {
    if (!this.enabled) return;
    for (const g of this.going.values()) if (!g.voice && LASTING[g.of.name]) {
      if (LASTING[g.of.name].room) { this.begin(g); continue; }
      const d = v3.dist(g.of.pos, this.listener); let alive = 0, all = 0, far: Voice | null = null, farD = -1;
      for (const w of this.voices) if (w.held && !LASTING[w.held.of.name].room) { all++; if (!w.dying) { alive++; const e = v3.dist(w.held.of.pos, this.listener); if (e > farD) { farD = e; far = w; } } }
      if (d > MAX_DIST || all >= MAX_LASTING + 4 || alive >= MAX_LASTING && d > 0.8 * farD) { if (!g.culled) { g.culled = true; this.stats.culled++; } this.tell(g, 1); continue; }
      this.begin(g); if (g.voice && alive >= MAX_LASTING) this.letGo(far!);
    }
  }
  private begin(g: Going) {
    const b = this.backend!, of = g.of, kind = LASTING[of.name], made = `lasting:${of.name}`;
    if (!kind.made) warmOne(of.name); else if (!b.duration(made)) b.loadBuffer(made, kind.made(b.sfxContext));
    const trim = this.trims.size && !kind.made ? this.trims.get(of.name) : undefined;
    const buffer = kind.made ? made : trim?.take && b.duration(trim.take) ? trim.take : of.name;
    const dur = b.duration(buffer); if (!dur) return;
    const rate = kind.made ? 1 : 0.96 + sysRandom() * 0.08, prop = this.ask.hear(of.pos, this.listener), at = prop.apparent, occ = prop.muffle, lp = kind.loopAt;
    const bv = b.play({ buffer, pos: at, gain: 0, rate: rate * (trim?.rate ?? 1) * kind.rate(of), loop: true, loopAt: lp && dur >= lp.end ? lp : undefined, bus: kind.room ? 'ambience' : 'sfx',
      filter: kind.filter ?? (trim?.tone ? { type: 'lowpass', freq: trim.tone } : undefined), wobble: kind.wobble, distance: kind.distance ?? { ref: REF_DIST, rolloff: ROLLOFF, max: MAX_DIST }, cutoffHz: cutoffOf(occ), send: kind.room ? undefined : sendOf(occ, prop.pathLen) });
    if (!bv) { this.tell(g, 1); return; }
    this.voices.push(g.voice = { v: bv, positional: true, pos: v3.copy(of.pos), end: Infinity, held: { of, rate, occ } });
    if (this.voices.length > this.stats.maxVoices) this.stats.maxVoices = this.voices.length;
    const L = this.listener, f = this.camFwd, dx = at[0] - L[0], dz = at[2] - L[2];
    this.tell(g, 2, kind.made ? undefined : { rel: [dz * f[0] - dx * f[2], at[1] - L[1], dx * f[0] + dz * f[2]], gain: occludedGain(of.gain, occ), rate, cut: cutoffOf(occ), send: sendOf(occ, prop.pathLen), hrtf: true });
  }

  /** Per-frame housekeeping: retire voices, hold what goes on and refresh its occlusion, size the reverb by tracing around the listener. */
  update(dt: number, time: number) {
    const b = this.backend; if (!b) return;
    const t = b.now();
    this.voices = this.voices.filter(v => { if (t > v.end) { b.release(v.v); return false; } return true; });
    this.hold(); this.beat = performance.now();
    this.stats.voices = this.voices.length;
    // held voices, every frame: where the thing is now and how it goes (its panner and low-pass are re-aimed below)
    for (const v of this.voices) { const h = v.held; if (!h || v.dying) continue;
      const of = h.of, kind = LASTING[of.name], vp = v.pos!; vp[0] = of.pos[0]; vp[1] = of.pos[1]; vp[2] = of.pos[2];
      const trim = this.trims.size ? this.trims.get(of.name) : undefined, near = of.reach > 0 ? Math.max(0, 1 - v3.dist(vp, this.listener) / of.reach) : 1;
      if (kind.room) b.setVoice(v.v, { gain: of.gain * near * (1 - 0.6 * h.occ) * kind.gain(of) }, 0.5);
      else b.setVoice(v.v, { gain: t - this.moved > 0.2 ? 0 : occludedGain(of.gain * near * (trim?.gain ?? 1), h.occ) * kind.gain(of), rate: h.rate * (trim?.rate ?? 1) * kind.rate(of) }, 0.07);
    }
    // environment probe (4×/s): mean free path around the listener → reverb amount; outdoor → less
    if (time - this.lastEnvT > 0.25 || this.lastEnvT === 0 || this.jumped) {
      let sum = 0; const N = 10, L = this.listener;
      for (let i = 0; i < N; i++) { const a = (i / N) * Math.PI * 2; const hit = this.ask.ray(L, [Math.cos(a), 0, Math.sin(a)], 16); sum += hit ? hit.t : 16; }
      const mfp = sum / N, wet = this.outdoors(L[0], L[2]) ? 0.08 : Math.min(0.55, 0.12 + mfp * 0.045);
      b.setReverb(wet, 0.3); this.stats.wet = wet;
      this.lastEnvT = time || 1e-3;
      // what is held goes through the same propagation as one-shots: the panner at the apparent position, the low-pass and the room's share (at once, where the game has jumped: it is another place, not a way there)
      const tc = this.jumped ? 0.02 : 0.25; this.jumped = false;
      for (const v of this.voices) if (v.held && !v.dying) { const { of } = v.held, room = LASTING[of.name].room;
        if (of.reach > 0 && v3.dist(v.pos!, L) > of.reach) continue;
        const pr = this.ask.hear(v.pos!, L); b.setVoice(v.v, { pos: pr.apparent, cutoffHz: (room ? 300 : 0) + cutoffOf(pr.muffle), send: room ? undefined : sendOf(pr.muffle, pr.pathLen) }, tc); v.held.occ = pr.muffle; }
    }
    void dt;
  }

  setVolumes() {
    const b = this.backend; if (!b) return;
    b.setBus('master', this.enabled ? this.volume.master : 0, 0.1);
    b.setBus('sfx', this.volume.sfx, 0.1); b.setBus('ambience', this.volume.ambience, 0.1);
    this.music?.setVolume(this.volume.music);
  }
}
