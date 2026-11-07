// THE PAGE THAT DRAWS A GAME IT DOES NOT HOLD (draw.html; `?draw=1` on the game's page). The game is the Rust module's (host/drawgame.ts): started from the
// level's file and the address's words, handed what the player did, stepped, and drawn from what each tick leaves to be drawn (src/draw). Nothing this
// page's bundle holds is the game's TypeScript: tools/draw/ban.ts stops the build on an import of it.
import { initGpu, showFatal, showFatalError, pipelineProgress } from '../gpu/device';
import { Engine } from '../engine';
import { SmokeSystem } from '../smoke/smoke';
import { Quality } from '../ui/quality';
import { glassBreakConfig, lookConfig, snowConfig, snowLookConfig, wallSidesConfig, weatherKindConfig } from '../render/frame';
import { lookAmounts, lookParts, lookSync } from '../render/gbuffer';
import { hourExposure } from '../render/photometry';
import { charSlots, meshAuthored } from '../render/charMaterials';
import { MeshShadows } from '../render/meshShadows';
import { fitRig, fitRigCast } from '../render/sdfrig';
import { cutConfig, initCutSwitch } from '../scene/cut';
import { roundConfig } from '../scene/round';
import { CAST_FILE, CAST_FILE_LOOK, castUrlFor, loadCharacterSet } from '../anim/gltf';
import { CharacterRenderer } from '../anim/characters';
import { COATFINISH, SOLIDGLASS } from '../scene/stuff';
import { MARKING_TEXEL } from '../scene/markings';
import { LOOK, lookLevelFile, lookKeptOf, lookOfQuery, placeOfQuery, setLookPlace } from '../scene/look';
import { Input } from '../game/input';
import { FollowCamera } from '../game/camera';
import { FIXEDSTEP, planTicks } from './fixedstep';
import { CameraPair, Lag } from './between';
import { FINDTUNED, bindFindMe, ctrlTurn, findMeStarts } from './findme';
import { fetchFace } from './face';
import { EVENPACE, FLIGHT_PAGE, pacer, pageRoom, rateMeter } from './pace';
import { PLAN_PACE } from '../render/planPace';
import { RENDER_PASSES } from '../gpu/timer';
import { DrawGame } from './drawgame';
import { keysBoot } from './keymap';
import { Section, Sky, View } from '../draw/sections';
import { AudioEngine } from '../audio/audio';
import { askOf } from '../draw/ask';
import { DrawEvents } from '../draw/events';
import { Particles } from '../render/particles';
import { DrawReach, doorways } from '../draw/reach';
import { DrawMen } from '../draw/men';
import { DrawLights } from '../draw/lights';
import { DrawWorld } from '../draw/world';
import { lookNumbers } from '../draw/look';
import { sampleHand } from '../draw/hand';
import { HOURS_OF, skyFrom } from '../draw/sky';
import { fogFrom } from '../draw/fog';
import { makeFogFrame } from '../render/fogofwar';
import type { Vec3 } from '../math/vec';

const bytesOf = async (url: string): Promise<Uint8Array> => { const r = await fetch(url, { cache: 'no-store' }); if (!r.ok) throw new Error(`${url}: HTTP ${r.status}`); return new Uint8Array(await r.arrayBuffer()); };
/** the level's file in the look that is on, or the file itself where it has none */
async function levelBytes(name: string): Promise<Uint8Array> {
  const look = lookLevelFile(name);
  try { return await bytesOf(`levels/${look}`); } catch (e) { if (look === name) throw e; return bytesOf(`levels/${name}`); }
}

async function main(): Promise<void> {
  const q = new URLSearchParams(location.search), loading = document.getElementById('loading')!, say = (line: string) => { loading.textContent = line; };
  const canvas = document.getElementById('view') as HTMLCanvasElement;
  keysBoot(q);
  LOOK.on = lookOfQuery(q); setLookPlace(placeOfQuery(q));
  const name = /^[\w.-]+\.json$/.test(q.get('levelFile') ?? '') ? q.get('levelFile')! : 'office.json', at = (p: string) => new URL(p, document.baseURI).href;

  // ---- the game
  const chP = loadCharacterSet(at('assets/ual/AnimationLibrary_Godot_Standard.gltf'), castUrlFor(location.search, document.baseURI)); chP.catch(() => { /* awaited below */ });
  const [face, level, mannequin, mannequinBin, cast, castLook] = await Promise.all([
    fetchFace(say, () => { throw new Error('the game asked a die the page never hands in'); }), levelBytes(name),
    bytesOf(at('assets/ual/AnimationLibrary_Godot_Standard.gltf')), bytesOf(at('assets/ual/AnimationLibrary_Godot_Standard.bin')), bytesOf(at(`assets/chars/${CAST_FILE}`)), bytesOf(at(`assets/chars/${CAST_FILE_LOOK}`))]);
  say('starting the game …');
  const game = new DrawGame(face), seed = Number(q.get('seed') ?? NaN);
  game.start({ level, words: location.search, mannequin, mannequinBin, cast, castLook, seed: Number.isFinite(seed) ? seed >>> 0 : crypto.getRandomValues(new Uint32Array(1))[0] });
  const D = game.drawn, sky = () => D.at[Section.sky], view = () => D.at[Section.view];

  // ---- the picture's constants: what the shaders are compiled with is said before any of them is
  wallSidesConfig.on = lookAmounts.walls > 0;
  lookConfig.kept = lookKeptOf(D.f[sky() + Sky.kept] < 0 ? undefined : D.f[sky() + Sky.kept], q.get('kept'));
  weatherKindConfig.on = true; snowConfig.on = D.u[sky() + Sky.snowLevel] !== 0;
  if (snowConfig.on) { snowLookConfig.surface = true; snowLookConfig.glow = true; snowLookConfig.sparkle = true; snowLookConfig.under = true; }
  glassBreakConfig.on = true; initCutSwitch(); roundConfig.gpu = cutConfig.on;
  COATFINISH.on = true; SOLIDGLASS.on = true;

  const gpu = await initGpu(canvas, { retry: true });   // (the module's game has stood by now)
  let world!: DrawWorld; const engine = new Engine(gpu, undefined, d => world = new DrawWorld(d, D)), smoke = new SmokeSystem(engine);
  engine.setParticles(new Particles(engine.device, engine.sceneLayout));
  engine.setSmoke(smoke.binding); engine.afterSubmit.push(() => smoke.afterSubmit());
  engine.settings.markingTexel = D.f[sky() + Sky.markTexel] || MARKING_TEXEL;
  pipelineProgress.onChange = () => say(`compiling shaders ${pipelineProgress.done} / ${pipelineProgress.total} …`);
  // the men: the mesh and its skinning, the tree that shadows him, the cones the rays meet of him
  const ch = await chP, chars = new CharacterRenderer(engine.device, ch), shadows = new MeshShadows(engine.device, ch, chars.jointMats, chars.charInsts);
  engine.skin = chars; engine.setMeshShadows(shadows);
  if (ch.cast) { lookParts.suit = ch.cast.parts.indexOf('op_suit') + 1; lookParts.rig = ch.cast.parts.indexOf('chest_rig') + 1; engine.gbuffer.setLook(); }
  if (meshAuthored(ch.mesh)) { engine.settings.charMats = true; charSlots(ch); }
  const men = new DrawMen(ch, chars, engine.world, D, () => engine.settings.charMats);
  { const fit = fitRig(ch); engine.setRigFit(ch.cast ? { ...fit, byOwner: men.fits } : fit); if (ch.cast) fitRigCast(ch); }
  game.look(lookNumbers(engine.settings, engine.world, !!ch.cast)); game.frame(1, [20, 0, 14]); engine.world.upload();   // (every stuff the level is made of is known from here: its maps are made with the programs)
  await Promise.all([engine.init({ sharp: true }), smoke.init(), engine.particles!.init(), engine.gbuffer.needLook(), shadows.init(), engine.gbuffer.prepareSkin(chars.jointsFormat)]);
  pipelineProgress.onChange = null; engine.warmFirstUse();
  const quality = new Quality(engine, smoke); quality.visit(location.search);
  lookSync();
  lookConfig.exposure = () => { const h = HOURS_OF[D.u[sky() + Sky.hour]]; return h === 'night' ? null : hourExposure(h, D.f[sky() + Sky.hourSun]); };
  { const own = { zenith: [0, 0, 0] as Vec3, horizon: [0, 0, 0] as Vec3 }; for (let i = 0; i < 3; i++) { own.zenith[i] = D.f[sky() + Sky.ownZenith + i]; own.horizon[i] = D.f[sky() + Sky.ownHorizon + i]; } lookConfig.nightSky = () => own; }
  lookConfig.snow = () => snowConfig.on;
  { const eye: [number, number] = [0, 0]; lookConfig.eyeAt = () => { const o = view() + View.at; eye[0] = D.f[o]; eye[1] = D.f[o + 2]; return eye; }; }
  const gaps = doorways(D); { const lines = gaps.map(d => [d.hinge[0], d.hinge[1], d.hinge[0] + Math.cos(d.closedDir) * d.width, d.hinge[1] + Math.sin(d.closedDir) * d.width] as [number, number, number, number]); lookConfig.doorGaps = () => lines; }
  const fog = makeFogFrame(), audio = new AudioEngine(askOf(game)), events = new DrawEvents(D, engine.particles, smoke, audio, i => game.nameOf(i));
  if (q.has('mute')) { audio.volume.master = 0; audio.start = () => { /* silent */ }; }
  for (const ev of ['pointerdown', 'keydown'] as const) window.addEventListener(ev, () => audio.start(), { capture: true });   // (a browser's sound begins with a hand on the page)
  engine.fogFrame = fog; engine.beams = world.beams;
  const lights = new DrawLights(engine.lights, D); lights.from();   // (the lights stand too: their reach is worked out of the level as it opens)
  const reach = new DrawReach(engine, world, gaps);

  // ---- the player
  const input = new Input(canvas); input.saveKey = () => true;   // (F5 is the game's: a save by hand)
  const cam = new FollowCamera(), follow: Vec3 = [0, 0, 0], aim: Vec3 = [0, 0, 0];
  FINDTUNED.on = true; findMeStarts(); bindFindMe(cam);   // (how the view feels: the player's own rows, over the tuned starts)
  let cuts = -1, stood = -1; const lag = new Lag(), pair = new CameraPair();
  const three = (v: Vec3, o: number): Vec3 => { v[0] = D.f[o]; v[1] = D.f[o + 1]; v[2] = D.f[o + 2]; return v; };
  const peek: Vec3 = [0, 0, 0], kick: Vec3 = [0, 0, 0], any = (v: Vec3, o: number): Vec3 | null => D.f[o] || D.f[o + 1] || D.f[o + 2] ? three(v, o) : null;
  /** what the tick left for the camera that follows the shown man */
  const lens = (): void => {
    const o = view(), n = D.u[o + View.cuts];
    cam.peekOffset = any(peek, o + View.peek); cam.kick = any(kick, o + View.kick);
    if (cuts < 0) { three(cam.target, o + View.follow); cuts = n; } else if (n !== cuts) { cuts = n; three(cam.target, o + View.cut); }
  };
  /** the game is another than the one last drawn (a restart, a load): what the picture and the sound carry over from frame to frame starts over, before anything of the new one is played */
  const anew = (): void => {
    const o = view(), first = stood < 0; stood = D.u[o + View.stood]; if (first) return;
    engine.restartPicture(); engine.particles?.clear(); smoke.clearAll(); events.cut(); audio.hush(); pair.cut(); lag.reset(); three(cam.target, o + View.follow);
  };
  anew(); lens(); cam.aspect = Math.max(1, canvas.clientWidth) / Math.max(1, canvas.clientHeight); cam.rebuild();

  const told = { update: 0, render: 0, tick: 0, draw: 0, frames: 0, fps: 0 };
  const ear: Vec3 = [0, 0, 0];
  let time = 0, acc = 0, drift = 0;
  const tick = (dt: number): void => {
    const keys = sampleHand(game.numbersOf(), input, cam, canvas);
    game.step(dt, keys);
    if (D.u[view() + View.stood] !== stood) anew();
    events.from();
    D.fresh();   // (a sound asks the module, whose memory may have grown)
    lens();
    const o = view();
    cam.step(dt, three(follow, o + View.follow), D.u[o + View.spectating] ? null : three(aim, o + View.aim));
    pair.capture(cam);
    told.tick = game.ms.step; told.draw = game.ms.draw;
  };
  /** one drawn frame: `dt` seconds of the display's, in whole ticks (`drawn` false: a script's, stepped and not drawn) */
  const frame = (dt: number, ticks?: number, drawn = true): void => {
    time += dt;
    const t0 = performance.now(), plan = ticks === undefined ? planTicks(acc, dt, FIXEDSTEP, drift) : { ticks, acc, drift }; acc = plan.acc; drift = plan.drift;
    if (input.wheelCtrl) { cam.zoom(input.wheelCtrl); input.wheelCtrl = 0; }
    if (input.orbitDX && plan.ticks) cam.rotate(ctrlTurn(input.orbitDX));   // (in a frame's first tick: the next tick's hand looks the way it left the view)
    cam.frame();
    for (let i = 0; i < plan.ticks; i++) { if (i) input.endTick(); tick(1 / FIXEDSTEP.hz); }
    D.fresh();
    skyFrom(engine, D); fogFrom(fog, D);
    { const o = view(); engine.settings.nvAmount = D.f[o + View.night]; engine.settings.hitFlash = D.f[o + View.hit]; }
    game.look(lookNumbers(engine.settings, engine.world, !!ch.cast));
    const t1 = performance.now();
    if (!drawn) { input.endFrame(); return; }
    { const o = view() + View.at; ear[0] = D.f[o]; ear[1] = D.f[o + 1] + 1.6; ear[2] = D.f[o + 2]; audio.setListener(ear, cam.forward); audio.update(dt, time); }
    smoke.update(dt);
    const a = ticks === undefined ? lag.at(acc * FIXEDSTEP.hz) : 1, pose = pair.at(a, cam);
    if (pose) cam.drawIn(pose);
    game.frame(a, cam.focus); lights.from(); men.from(engine.device.queue); reach.from(); world.from();
    cam.kickIn(); try { engine.render(cam, time, dt, true); } finally { cam.kickOut(); cam.drawOut(); }
    if (plan.ticks) input.endFrame();
    told.update = t1 - t0; told.render = performance.now() - t1; told.frames++;
  };
  let last = performance.now(), held = q.has('paused');
  // THE PAGE'S PACE (host/pace.ts): a frame is not begun while two are still with the GPU, and a fast display's frames between the ladder's are passed over. A tab
  // nobody sees is not drawn; the time it was away is not the game's
  const pace = pacer(), rate = rateMeter(), rendered = [...RENDER_PASSES]; let resolved = -1;
  EVENPACE.on = true; EVENPACE.flight = FLIGHT_PAGE; PLAN_PACE.on = true;
  document.addEventListener('visibilitychange', () => { last = performance.now(); pace.reset(); rate.reset(); });
  const loop = (ts: number): void => {
    try {
      const room = pageRoom({ calibrating: quality.calibrating, measuring: false, kitView: false });
      if (held || document.hidden) { last = performance.now(); pace.reset(); rate.reset(); requestAnimationFrame(loop); return; }
      if (pace.skip(ts, !quality.calibrating && (quality.adaptive || quality.auto) ? quality.bands.target : 0, false, room)) { requestAnimationFrame(loop); return; }
      const now = performance.now(), raw = (now - last) / 1000; last = now; told.fps = rate.push(now);
      if (quality.early) quality.remeasure();
      if (quality.calibrating) quality.calibrateTick(raw);
      else if (raw < 0.5) { const fresh = engine.timer.resolved !== resolved; resolved = engine.timer.resolved; quality.sample(raw, !engine.timer.enabled ? 0 : fresh ? engine.timer.unionMs(rendered) : NaN); }
      frame(Math.min(1 / 20, raw));
      if (room > 0) { const w = pace.sent(performance.now()), end = (): void => pace.done(w, performance.now(), 0); gpu.device.queue.onSubmittedWorkDone().then(end, end); }
      requestAnimationFrame(loop);
    } catch (e) { console.error(e); showFatal(`The frame loop threw and has been stopped:\n\n${(e as Error)?.stack ?? e}`); }
  };
  loading.style.display = 'none';
  // (for a tool: the page's parts, and frames stepped by hand)
  Object.assign(window, { __draw: { game, engine, world, audio, cam, input, quality, told, fog, get held() { return held; }, set held(v: boolean) { held = v; last = performance.now(); }, run: (n: number, keys: string[] = [], drawn: 'none' | 'last' | 'all' = 'last') => { for (const k of keys) input.keys.add(k); try { for (let i = 0; i < n; i++) frame(1 / 60, 1, drawn === 'all' || (drawn === 'last' && i === n - 1)); } finally { for (const k of keys) input.keys.delete(k); } } } });
  requestAnimationFrame(loop);
}
const ready = main(); (window as unknown as { __ready: Promise<void> }).__ready = ready;
ready.catch(e => { console.error(e); if ((e as Error)?.name === 'RustGameError') showFatal((e as Error).message); else showFatalError(e); });
