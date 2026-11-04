// CUT BOXES — A CALM ROOM PAYS NOTHING (scene/cut.ts cutConfig.swap).
//
// The cut rule is one extra branch in the ray-box test of every tracing program (common.wgsl isectBox behind the scene prelude's `const CUT_BOXES`). Whether a GPU
// pays for a branch nobody takes cannot be measured where this was written, so the build does not ask it to: every pipeline BOOTS from the prelude without the rule
// (the text `?nocutboxes` compiles — the calm frame is that build's, program for program), a SECOND set of the tracing pipelines is compiled behind the rule in the
// background once the boot is over, and the frame takes that set only while a cut box a ray can meet is alive (BoxWorld.cutTraced), going back a moment after the
// last one is gone. Both sets share every layout, bind group, buffer and history: a swap is a handful of field assignments between two frames.
//
// Who is in it: every owner of a pipeline that traces boxes — render/passes.ts (direct, restir ×2, the four gathers, both reflection kernels, the bounce cards, the
// volumetric march + its dust twin, the composite ×4 for its side ray, the traced sharp kernel), rc.ts (the cascades, every instance), froxels.ts, probe.ts (the
// light meter: ONE MODEL — guards read the cut shape's shadow too), particles.ts (the sim's collision / light rays), fogofwar.ts. The g-buffer's cut pipeline is
// its own module and always carries the rule — so the ENGINE draws a traceable cut box with it only on a frame whose ray programs carry the rule too (`drawsCut`): until
// the second set exists (the first seconds after boot), if it failed to compile, or while a dev hook pins the boot's programs, such a piece is WHOLE for the raster
// and for every GPU ray alike — never a cut shape in the picture over a box's shadow, or the reverse. (The CPU's lines — rounds, sight, sound — meet the cut shape
// from the frame the piece is born: the simulation never waits on a compile. A glass plate, picture only, is drawn cut always: no ray could disagree with it.)
//
// WHEN NEEDED (2026-10-26; ui/shaderrows.ts row `cutWhenNeeded`, `?cutwhenneeded=1`, OFF by default and read once at this module's import). On D3D12 the second set is
// 10–25 s of DXC compiling behind every start (gpu/device.ts THE COMPILE QUEUE has the measure), paid by a visit that never breaks a thing. With the row on the start
// builds nothing (`start()` only ARMS the asking) and `need(why)` builds the set the first time one of these is true:
//   'piece'  frame() is handed a cut box a ray can meet: the first break, or the first frame after a save with pieces in it was loaded (the world's own upload says so:
//            no load path has a hook of its own)
//   'sign'   cutComing(game) — main.ts asks it once a frame while the set is unasked: a round fired by anyone, a bang, a frag or a launcher's round in the air, or a
//            charge set on a wall or a door (or being set on a wall). Each comes before the first piece it can make: a round by a frame, a throw by its fuse, a
//            charge by its hand work and fuse. Not asked on the Team & kit picture's page, whose staged man fires for show: there only a piece asks.
//   'ready'  somebody read window.__cut.programs.ready (the page tools that stage a cut piece await it): asking when the set is ready is asking for the set
// Until the set is in, a traceable cut piece is drawn and traced WHOLE, exactly as in the seconds after a boot today — but that window now opens at the first break and
// lasts as long as the compile (two in flight: the queue's background lane), so on a machine that compiles slowly the row changes what a fight looks like for that long,
// and what the GPU's rays report (the guards' light meter, the fog pass) with it; the CPU's lines never wait. The asking itself costs the page's thread the making of
// the set's shader modules (their text is cut and handed over there: tens of ms at the start today, behind the card) — under the row one owner a frame, from the frame
// of the first shot. Off = build() at the start, every owner at once, as it was. A switch of the page, not a rule of the game. tools/qa/cutneed-check.ts
// (CUTNEED_BREAK=1 must fail); on the page tools/qa/cutneed-page.cjs. Never seen on a GPU.
import { cutConfig } from '../scene/cut';
import { pipelineProgress } from '../gpu/device';
import { useShaderRow } from '../ui/shaderrows';

export interface CutSwappable {
  /** build this owner's tracing pipelines behind the cut prelude (scenePrelude(true)); resolves when they exist. May be called again (a rebuild). */
  buildCut(): Promise<void>;
  /** take the cut programs (true) or the boot's (false); a no-op until buildCut has resolved */
  useCut(on: boolean): void;
}

/** one owner's two sets: `cut` = field name → pipeline, `base` = what those fields held when the cut set went live */
export class ProgramSet {
  cut: Record<string, unknown> | null = null; private base: Record<string, unknown> | null = null; live = false;
  /** install a (re)built cut set; if the old one is live it is taken out first and the new one put in */
  set(owner: object, cut: Record<string, unknown>) { const was = this.live; this.use(owner, false); this.cut = cut; this.use(owner, was); }
  use(owner: object, on: boolean) {
    if (!this.cut || on === this.live) return;
    const o = owner as Record<string, unknown>;
    if (on) { const b: Record<string, unknown> = {}; for (const k in this.cut) b[k] = o[k]; this.base = b; Object.assign(o, this.cut); }
    else Object.assign(o, this.base!);
    this.live = on;
  }
  /** replace members of BOTH sets in place (a rebuild of some programs with their cut twins: render/shaderTune.ts), whichever set is live staying live; with no cut set only the owner's fields are written */
  patch(owner: object, base: Record<string, unknown>, cut: Record<string, unknown> | null) {
    const was = this.live; this.use(owner, false); Object.assign(owner, base);
    if (this.cut && cut) Object.assign(this.cut, cut);
    this.use(owner, was);
  }
  /** the BOOT set's member under a key, whichever set is live (a pipeline that arrives late asks whether its place was filled meanwhile: render/passes.ts setL) */
  baseOf(owner: object, key: string): unknown { return this.live && this.base && key in this.base ? this.base[key] : (owner as Record<string, unknown>)[key]; }
}

const HOLD_FRAMES = 240;   // frames without a traced cut box before the boot's programs come back (4 s at 60 Hz: pieces coming and going round zero must not flap the two sets)

export const cutSwap = {
  /** is the swap what this boot does? (the switch on AND cutConfig.swap) */
  get enabled(): boolean { return cutConfig.on && cutConfig.swap; },
  members: new Set<CutSwappable>(),
  /** 'none' = nothing asked yet · 'building' · 'ready' = the second set exists · 'failed' = it did not compile (the boot's programs stay: cut pieces trace whole) */
  state: 'none' as 'none' | 'building' | 'ready' | 'failed',
  /** the cut programs are the ones dispatched this frame */
  live: false,
  /** by hand (the cost lab, `__cut.programs.force`): 'cut' / 'base' pin a set whatever is alive; null = the rule */
  force: null as 'cut' | 'base' | null,
  /** ms the background build took, and how many owners it covered */
  buildMs: 0, built: 0, pipelines: 0,
  /** swaps since boot (in + out) */
  swaps: 0,
  error: '' as string,
  idle: 0,
  /** a dev hook put a hot-swapped program on some owner (hot()): the boot's set is pinned until the page reloads */
  hotPinned: false,
  /** the g-buffer's own cut pipeline did not compile (rasterFailed()): the picture can only show cut pieces whole, so no ray program with the cut rule goes on the frame either */
  noRaster: false,
  /** does this frame's g-buffer draw TRACEABLE cut boxes cut? Only when the ray programs dispatched this frame carry the cut rule (a build without the swap: always) */
  get drawsCut(): boolean { return !this.noRaster && (!this.enabled || this.live); },
  ready: null as Promise<boolean> | null,
  /** WHEN NEEDED (the head of this file): this load's word of the row — the second set is built when first needed, not at the start */
  whenNeeded: useShaderRow('cutWhenNeeded'),
  /** start() has run under whenNeeded: need() may build (never before the boot's own pipelines are in, and never on a page that does not start the set at all) */
  armed: false,
  /** what asked for the set under whenNeeded ('' = nothing yet, or the row is off) */
  asked: '' as '' | 'piece' | 'sign' | 'ready',
  /** the row holds the set back and nothing has asked for it yet (true from the import on: a pass that would compile a cut twin of its own beside the set holds that too) */
  get held(): boolean { return this.whenNeeded && this.state === 'none' && this.enabled; },
  /** …and the start has armed the asking: the next piece, sign or reader of `ready` builds it */
  get waiting(): boolean { return this.armed && this.held; },
  /** main.ts, once the boot's own pipelines are in: build the second set in the background now — or, under whenNeeded, only arm need() */
  start() { if (this.whenNeeded) this.armed = true; else void this.build(); },
  /** the set is wanted: build it if it was being held back. Resolves like build(); with nothing held back it is the build already asked for (false if none was) */
  need(why: 'piece' | 'sign' | 'ready'): Promise<boolean> {
    if (!this.waiting) return this.ready ?? Promise.resolve(false);
    if (why !== 'ready' && (this.noRaster || this.hotPinned)) return Promise.resolve(false);   // (the picture can only draw pieces whole, or a dev hook pinned the boot's programs: the frame would never take the set)
    this.asked = why;
    console.log(`cut programs: asked for now (${why === 'piece' ? 'a cut piece is in the world' : why === 'sign' ? 'a cut is coming' : 'a tool asked when they are ready'})`);
    return this.build();
  },
  add(m: CutSwappable) {
    if (!this.enabled) return;
    this.members.add(m);
    if (this.state === 'building' || this.state === 'ready') {
      const P = pipelineProgress, failed0 = P.failed;   // (gpu/device.ts never rejects: a failure is a count, as in build())
      m.buildCut().then(() => { if (P.failed > failed0) throw new Error('a late owner\'s pipelines failed to compile'); m.useCut(this.live); }).catch(e => this.fail(e));
    }
  },
  /** the second set is not whole: every owner goes back to the boot's programs and stays there — cut pieces are traced (and so drawn) as whole boxes, picture and rays agreeing */
  fail(e: unknown) {
    this.state = 'failed'; this.error = String((e as Error)?.message ?? e);
    if (this.live) { this.live = false; this.swaps++; }
    for (const m of this.members) m.useCut(false);
    console.error('[cut programs] the second set did not compile — cut pieces are traced as whole boxes', e);
  },
  /** render/gbuffer.ts: the picture's cut pipeline failed to compile (its one console line is the pipeline helper's). The g-buffer draws every cut box whole from here, so
   *  the boot's ray programs are pinned for good. (Under `?cutprograms=boot` the one set carries the cut rule and cannot be taken back: there the rays stay cut.) */
  rasterFailed() {
    this.noRaster = true; this.error = this.error || 'the g-buffer\'s cut pipeline did not compile';
    if (this.live) { this.live = false; this.swaps++; }
    for (const m of this.members) m.useCut(false);
  },
  remove(m: CutSwappable) { this.members.delete(m); },
  /** compile the second set now (main.ts: after the boot's own pipelines resolved, not awaited by the start card). Resolves true when every owner has it. */
  build(): Promise<boolean> {
    if (!this.enabled) return Promise.resolve(false);
    if (this.ready) return this.ready;
    this.state = 'building'; const t0 = performance.now(); const ms = [...this.members]; const P = pipelineProgress, total0 = P.total, failed0 = P.failed;
    // (WHEN NEEDED: asked in the middle of play, so the owners make their modules a frame apart, not all inside the frame of the first shot)
    const jobs = this.whenNeeded ? ms.map((m, i) => i === 0 ? m.buildCut() : new Promise<void>(r => setTimeout(r, i * 16)).then(() => m.buildCut())) : ms.map(m => m.buildCut());
    return this.ready = Promise.all(jobs).then(() => {
      this.buildMs = performance.now() - t0; this.built = ms.length; this.pipelines = P.total - total0;
      if (P.failed > failed0) throw new Error(`${P.failed - failed0} of its pipelines failed to compile`);   // (gpu/device.ts hands back an invalid pipeline on a failure: never dispatched from here)
      if (this.state === 'failed') return false;   // (a late owner failed while this was compiling: fail() has said so)
      this.state = 'ready';
      console.log(`cut programs ready: ${this.pipelines} pipelines of ${ms.length} owners in ${this.buildMs.toFixed(0)} ms (the frame takes them while a cut piece is alive)`);
      return true;
    }).catch(e => { this.fail(e); return false; });
  },
  /** once a frame, after the world's upload: `traced` = cut boxes a ray can meet this frame */
  frame(traced: number) {
    if (traced > 0 && this.waiting) void this.need('piece');   // (WHEN NEEDED: the first piece a ray can meet — a break, or a loaded save's — asks for the set)
    if (this.state !== 'ready') return;
    let want: boolean;
    if (this.force) { want = this.force === 'cut'; this.idle = HOLD_FRAMES; }   // (a pin let go leaves no hold behind it: the next unpinned frame is the rule's own — the cut set only if a piece is alive)
    else if (traced > 0) { want = true; this.idle = 0; }
    else want = this.live && ++this.idle < HOLD_FRAMES;
    if (this.noRaster) want = false;
    if (want !== this.live) { this.live = want; this.swaps++; }
    for (const m of this.members) m.useCut(want);
  },
  /** a dev hook is about to write a hot-swapped pipeline (compiled from the prelude WITHOUT the cut rule) into an owner's fields: put the boot's set back on every
   *  owner first and pin it, so the two mechanisms never overwrite each other — from here cut pieces are whole for picture and rays until a reload (dev pages only) */
  hot() {
    if (!this.enabled) return;
    if (!this.hotPinned) console.warn('[cut programs] a hot-swapped program is in: the boot set is pinned, cut pieces are drawn and traced whole until the page reloads');
    this.hotPinned = true; this.force = 'base';
    if (this.live) { this.live = false; this.swaps++; }
    for (const m of this.members) m.useCut(false);
  },
};

/** WHEN NEEDED's early sign, read off the game as it stands (main.ts, once a frame while the set is unasked; nothing is written): a cut is coming when a round has been
 *  fired by anyone this half second (every round's event names its gun), a bang has gone off, a stun grenade or a frag is in the world, a launcher's round is in the
 *  air, a wall charge is being set or stands on a wall, or a charge stands on a door. A list kept by hand: what it does not know (a scripted blast, the next thing
 *  that learns to break a wall) is asked for by its first piece, a frame later than a sign would. */
export function cutComing(g: { events: readonly { kind: string; weapon?: unknown }[]; items: { items: readonly { kind: string }[] }; shells: readonly unknown[]; wallCharge: { ops: readonly unknown[]; set: readonly unknown[] }; doorTools: { set: readonly unknown[] } }): boolean {
  if (g.wallCharge.ops.length > 0 || g.wallCharge.set.length > 0 || g.doorTools.set.length > 0 || g.shells.length > 0) return true;
  for (const e of g.events) if (e.weapon !== undefined || e.kind === 'bang') return true;
  for (const it of g.items.items) if (it.kind === 'frag' || it.kind === 'flash') return true;
  return false;
}
