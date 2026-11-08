// THE PAGE THAT DRAWS A GAME IT DOES NOT HOLD (draw.html; `?draw=1` on the game's page). The game is the Rust module's (host/drawgame.ts): started from the
// level's file and the address's words, handed what the player did, stepped, and drawn from what each tick leaves to be drawn (src/draw). Nothing this
// page's bundle holds is the game's TypeScript: tools/draw/ban.ts stops the build on an import of it.
import { initGpu, laterIdle, pipeQueue, showFatal, showFatalError, pipelineProgress } from '../gpu/device';
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
import { machineAfter, machineBefore } from './machine';
import { EVENPACE, FLIGHT_PAGE, pacer, pageRoom, rateMeter } from './pace';
import { PLAN_PACE } from '../render/planPace';
import { RENDER_PASSES } from '../gpu/timer';
import { DrawGame, GameLost } from './drawgame';
import { debuggerPanel } from './debugger';
import { lostCard } from './lostcard';
import { Reel } from './reel';
import { Measure } from './measure';
import { urlFlagsOnEngine } from './engineflags';
import { FRAMECAP, watchTextures } from '../render/framecap';
import { SavePics } from './savepics';
import { Readout } from '../ui/readout';
import { PauseCard } from '../ui/pausecard';
import { agoWords, type RunSaves } from '../ui/runcard';
import { gameKeysBoot } from './keymap';
import { settingsOf } from './settings';
import { closeOptions, openOptions, optionsOpen, refreshOptions, setOptionsDeps, walkInNow } from '../ui/options';
import { LOADBRIEF, loadBrief } from '../ui/loadbrief';
import { settleOfQuery, startWaits } from '../ui/startwait';
import { TITLE, bootCard, cheatWords, titleScreen } from '../ui/title';
import { missionsBusy, missionsReopenAsked, reopenMissionsAfterLoad } from '../ui/missions';
import { openStartPage, startPage, startPageWanted } from '../ui/startpage';
import { RunBooks } from '../ui/runBooks';
import { watchMissionRows } from '../ui/verdictRows';
import { KIT_VIEW } from '../ui/kitView';
import { Desk, desk, setDesk, type DeskFace } from '../view/desk';
import { Commands } from '../draw/commands';
import { pickRedirect } from '../scene/picks';
import { keepPageBoot } from './keeppage';
import { ownPage, runPage } from './runpage';
import { bootTextSize } from '../ui/textsize';
import { Heard, Man, Section, Sky, View } from '../draw/sections';
import { AudioEngine, SFX_TABLE, warmSfx } from '../audio/audio';
import { setlistOfQuery } from '../audio/setlist';
import { MUSIC } from '../audio/pieces';
import { askOf } from '../draw/ask';
import { rimsFrom } from '../draw/rims';
import { makeOutlineFrame } from '../render/outline';
import { DrawEvents } from '../draw/events';
import { Particles } from '../render/particles';
import { DrawReach, doorways } from '../draw/reach';
import { DrawMen } from '../draw/men';
import { DrawLights } from '../draw/lights';
import { DrawDecals } from '../draw/marks';
import { dustFrom, trafficFrom } from '../draw/levellook';
import { DrawWorld } from '../draw/world';
import { lookNumbers } from '../draw/look';
import { sampleHand } from '../draw/hand';
import { HOURS_OF, skyFrom } from '../draw/sky';
import { fogFrom } from '../draw/fog';
import { makeFogFrame } from '../render/fogofwar';
import { WeatherPicture } from '../picture/weather';
import { SNOW_LOOK, snowLookOfQuery } from '../picture/snow/look';
import type { Vec3 } from '../math/vec';

const LOW_PITCH = 24 * Math.PI / 180, HIGH_PITCH = 55 * Math.PI / 180;
const bytesOf = async (url: string): Promise<Uint8Array> => { const r = await fetch(url, { cache: 'no-store' }); if (!r.ok) throw new Error(`${url}: HTTP ${r.status}`); return new Uint8Array(await r.arrayBuffer()); };
/** the level's file in the look that is on, or the file itself where it has none */
async function levelBytes(name: string): Promise<Uint8Array> {
  const look = lookLevelFile(name);
  try { return await bytesOf(`levels/${look}`); } catch (e) { if (look === name) throw e; return bytesOf(`levels/${name}`); }
}

async function main(): Promise<void> {
  const q = new URLSearchParams(location.search), loading = document.getElementById('loading')!; let say = (line: string): void => { loading.textContent = line; };
  // THE KEPT PAGE (gpu/keep.ts): the tab's own page keeps the GPU device and shows the game in a frame of this address, so a mission's start compiles nothing this visit has compiled
  const keep = keepPageBoot(q, ownPage(q)); if (keep.role === 'shell' || keep.role === 'leave') return;
  { const to = pickRedirect(location.search); if (to !== null) { location.replace(location.pathname + to + location.hash); return; } }
  const canvas = document.getElementById('view') as HTMLCanvasElement;
  // (of the page's markup, what this page drives: the rest of the HUD comes as it is told of the game)
  for (const el of document.getElementById('app')!.children) if (!['view', 'panel', 'lcol', 'veil', 'loading', 'menu', 'missions', 'fatal'].includes(el.id)) (el as HTMLElement).style.display = 'none';
  gameKeysBoot(q, walkInNow); bootTextSize(q);
  LOOK.on = lookOfQuery(q); setLookPlace(placeOfQuery(q));
  const name = /^[\w.-]+\.json$/.test(q.get('levelFile') ?? '') ? q.get('levelFile')! : 'office.json', at = (p: string) => new URL(p, document.baseURI).href;

  SFX_TABLE.was = q.has('oldsounds'); SFX_TABLE.redone = q.has('newsounds'); if (!q.has('mute')) warmSfx();   // (the sounds, a voice at a time in idle moments)

  // ---- the game
  const chP = loadCharacterSet(at('assets/ual/AnimationLibrary_Godot_Standard.gltf'), castUrlFor(location.search, document.baseURI)); chP.catch(() => { /* awaited below */ });
  const filesP = Promise.all([levelBytes(name), bytesOf(at('assets/ual/AnimationLibrary_Godot_Standard.gltf')), bytesOf(at('assets/ual/AnimationLibrary_Godot_Standard.bin')), bytesOf(at(`assets/chars/${CAST_FILE}`)), bytesOf(at(`assets/chars/${CAST_FILE_LOOK}`))]); filesP.catch(() => { /* awaited below */ });
  const face = await fetchFace(say, () => { throw new Error('the game asked a die the page never hands in'); });
  // THE DESK: the cards before a game ask the module, and what a press keeps goes in this machine's store, where the next page's start reads it
  { const C = new Commands(); setDesk(new Desk(face.exports as unknown as DeskFace, face.exports.memory as WebAssembly.Memory, (key, text) => { try { if (text) localStorage.setItem(key, text); else localStorage.removeItem(key); } catch { /* no store: nothing is kept */ } }));
    machineBefore(q, name, C); desk().do(C); }
  // (an address that plays a mission comes up on that mission's brief, one that plays nothing on the title; either is live from here, and the game is made behind it)
  LOADBRIEF.on = q.get('loadbrief') !== '0'; settleOfQuery(q);
  LOADBRIEF.options = { open: () => openOptions({ tab: 'controls' }), close: closeOptions, up: optionsOpen };
  TITLE.ownKeys = false; KIT_VIEW.page = new URL('./', document.baseURI).pathname;   // (no benchmark on this page; the Team & kit picture is the game's page in a frame)
  watchMissionRows();   // (each mission's card shows the ways this machine has won it)
  let title = startPageWanted(); const brief = q.has('paused') || runPage(q) ? null : loadBrief(location.search), card = brief ?? bootCard();
  loading.textContent = ''; say = line => card.status(line);
  const [level, mannequin, mannequinBin, cast, castLook] = await filesP;
  say('starting the game …');
  const game = new DrawGame(face), seed = Number(q.get('seed') ?? NaN);
  machineBefore(q, name, game.commands);
  game.start({ level, words: location.search, mannequin, mannequinBin, cast, castLook, seed: Number.isFinite(seed) ? seed >>> 0 : crypto.getRandomValues(new Uint32Array(1))[0] });
  machineAfter(game.commands); game.tell();   // (the title and Options read what is kept before the first step)
  const D = game.drawn, sky = () => D.at[Section.sky], view = () => D.at[Section.view];

  // ---- the picture's constants: what the shaders are compiled with is said before any of them is
  wallSidesConfig.on = lookAmounts.walls > 0;
  lookConfig.kept = lookKeptOf(D.f[sky() + Sky.kept] < 0 ? undefined : D.f[sky() + Sky.kept], q.get('kept'));
  weatherKindConfig.on = true; snowConfig.on = D.u[sky() + Sky.snowLevel] !== 0;
  if (snowConfig.on) { snowLookOfQuery(q); snowLookConfig.surface = SNOW_LOOK.on; snowLookConfig.wisps = SNOW_LOOK.wisps; snowLookConfig.tracks = SNOW_LOOK.on && SNOW_LOOK.tracks; snowLookConfig.glow = SNOW_LOOK.on && SNOW_LOOK.glow; snowLookConfig.sparkle = SNOW_LOOK.on && SNOW_LOOK.sparkle; snowLookConfig.under = SNOW_LOOK.under; snowLookConfig.grains = SNOW_LOOK.grains || SNOW_LOOK.oneAir; }
  glassBreakConfig.on = true; initCutSwitch();
  COATFINISH.on = true; SOLIDGLASS.on = true;

  const gpu = await initGpu(canvas, { retry: true, canvasRead: true, keeper: keep.keeper, seat: keep.seat });   // (the module's game has stood by now)
  if (q.get('framecap') === '0') FRAMECAP.on = false;
  if (FRAMECAP.on) watchTextures(gpu.device);   // (a frame capture knows every texture made from here on)
  let world!: DrawWorld; const engine = new Engine(gpu, undefined, d => world = new DrawWorld(d, D)), smoke = new SmokeSystem(engine);
  urlFlagsOnEngine(engine);
  engine.setParticles(new Particles(engine.device, engine.sceneLayout));
  engine.setSmoke(smoke.binding); engine.afterSubmit.push(() => smoke.afterSubmit());
  const decals = new DrawDecals(engine.device, D); engine.setDecals(decals); trafficFrom(engine.gbuffer, D);
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
  pipelineProgress.onChange = null; pipeQueue.settle(); engine.warmFirstUse();
  const quality = new Quality(engine, smoke); quality.visit(location.search); quality.onCalibrated = refreshOptions; (window as unknown as { __quality: Quality }).__quality = quality;
  lookSync();
  lookConfig.exposure = () => { const h = HOURS_OF[D.u[sky() + Sky.hour]]; return h === 'night' ? null : hourExposure(h, D.f[sky() + Sky.hourSun]); };
  { const own = { zenith: [0, 0, 0] as Vec3, horizon: [0, 0, 0] as Vec3 }; for (let i = 0; i < 3; i++) { own.zenith[i] = D.f[sky() + Sky.ownZenith + i]; own.horizon[i] = D.f[sky() + Sky.ownHorizon + i]; } lookConfig.nightSky = () => own; }
  lookConfig.snow = () => snowConfig.on;
  { const eye: [number, number] = [0, 0]; lookConfig.eyeAt = () => { const o = view() + View.at; eye[0] = D.f[o]; eye[1] = D.f[o + 2]; return eye; }; }
  const gaps = doorways(D); { const lines = gaps.map(d => [d.hinge[0], d.hinge[1], d.hinge[0] + Math.cos(d.closedDir) * d.width, d.hinge[1] + Math.sin(d.closedDir) * d.width] as [number, number, number, number]); lookConfig.doorGaps = () => lines; }
  const fog = makeFogFrame(), ask = askOf(game), audio = new AudioEngine(ask), events = new DrawEvents(D, engine.particles, smoke, audio, i => game.nameOf(i));
  MUSIC.pieces = MUSIC.byLevel = MUSIC.theirs = true; audio.scoreSet = setlistOfQuery(location.search); let run = -1;   // (the level's own music, as tense as those who are against him)
  if (q.has('mute')) { audio.volume.master = 0; audio.start = () => { /* silent */ }; }
  const settings = settingsOf(game); setOptionsDeps({ engine, quality, audio, game: settings });   // (what this machine keeps of the volumes and the wall hold is put on them)
  for (const ev of ['pointerdown', 'keydown'] as const) window.addEventListener(ev, () => audio.start(), { capture: true });   // (a browser's sound begins with a hand on the page)
  engine.fogFrame = fog; const rims = engine.outline = makeOutlineFrame(); engine.beams = world.beams;
  const lights = new DrawLights(engine.lights, D); lights.from();   // (the lights stand too: their reach is worked out of the level as it opens)
  const reach = new DrawReach(engine, world, gaps);
  const weather = new WeatherPicture(engine, D, world, engine.particles, smoke);

  // ---- the player
  const input = new Input(canvas); input.saveKey = () => !title && !brief?.up;   // (F5 is the game's while a run is on: a save by hand)
  const cam = new FollowCamera(), follow: Vec3 = [0, 0, 0], aim: Vec3 = [0, 0, 0];
  FINDTUNED.on = true; findMeStarts(); bindFindMe(cam);   // (how the view feels: the player's own rows, over the tuned starts)
  let cuts = -1, stood = -1; const lag = new Lag(), pair = new CameraPair();
  const three = (v: Vec3, o: number): Vec3 => { v[0] = D.f[o]; v[1] = D.f[o + 1]; v[2] = D.f[o + 2]; return v; };
  const peek: Vec3 = [0, 0, 0], kick: Vec3 = [0, 0, 0], any = (v: Vec3, o: number): Vec3 | null => D.f[o] || D.f[o + 1] || D.f[o + 2] ? three(v, o) : null;
  /** what the tick left for the camera that follows the shown man */
  const lens = (): void => {
    const o = view(), n = D.u[o + View.cuts];
    cam.peekOffset = any(peek, o + View.peek); cam.kick = any(kick, o + View.kick);
    // (what a rule did to the view itself: a level's script holds it, a lesson opens on its first look)
    if (D.u[o + View.pinned]) { const P = cam.pin ??= { look: [0, 0, 0], yaw: 0, pitch: 0, dist: 0 }; three(P.look, o + View.pin); P.yaw = D.f[o + View.pinYaw]; P.pitch = D.f[o + View.pinPitch]; P.dist = D.f[o + View.pinDistance]; } else cam.pin = null;
    { const t = D.u[o + View.turned]; if (t & 8) three(cam.target, o + View.target); if (t & 1) cam.yaw = D.f[o + View.yaw]; if (t & 2) cam.pitch = D.f[o + View.pitch]; if (t & 4) cam.distance = cam.desiredDistance = D.f[o + View.distance]; }
    if (cuts < 0) { three(cam.target, o + View.follow); cuts = n; } else if (n !== cuts) { cuts = n; three(cam.target, o + View.cut); }
  };
  const books = new RunBooks(), unkept = ownPage(q);   // (what this machine keeps of the runs played: a page that plays a run of its own is nobody's)
  /** the game is another than the one last drawn (a restart, a load): what the picture and the sound carry over from frame to frame starts over, before anything of the new one is played */
  const anew = (): void => {
    const o = view(), first = stood < 0; stood = D.u[o + View.stood]; if (first) return;
    engine.restartPicture(); engine.particles?.clear(); smoke.clearAll(); events.cut(); weather.cut(); audio.hush(); game.turnAgain(); pair.cut(); lag.reset(); books.anew(); three(cam.target, o + View.follow);
  };
  anew(); lens(); cam.aspect = Math.max(1, canvas.clientWidth) / Math.max(1, canvas.clientHeight); cam.rebuild();

  const told = { update: 0, render: 0, tick: 0, draw: 0, frames: 0, fps: 0 }, readout = new Readout({ engine, smoke, quality, adapter: gpu.adapterInfo, men: () => D.n[Section.men], told }, q);
  const ear: Vec3 = [0, 0, 0];
  const driven = q.has('paused') || navigator.webdriver && !q.has('reel');   // (a page a script drives stands still, unless its address asks)
  const veil = document.getElementById('veil')!, reel = new Reel(cam, {
    may: () => title && !driven && name === 'office.json' && !missionsBusy() && !optionsOpen() && !quality.calibrating,
    begin: () => { input.keys.clear(); input.buttons = 0; input.endFrame(); thaw = true; game.commands.aside(1, 1.5, 1.5); },   // (the grass off the lot's north-west corner: in no shot, on no route)
    back: () => { game.commands.aside(0, 0, 0); game.tell(); anew(); lens(); three(cam.target, view() + View.follow); cam.desired = [cam.target[0], cam.target[1], cam.target[2]]; pair.cut(); cam.rebuild(); },
    man: () => { const o = D.at[Section.men]; for (let i = 0; i < D.n[Section.men]; i++) { const r = o + i * Man.row; if (D.u[r + Man.side] === 2) return three([0, 0, 0], r + Man.at); } return null; },
    veil: (black, on) => { veil.style.opacity = String(black); document.body.classList.toggle('attract', on); } });
  // a save's picture, by its run and the save as the pause card names it
  const ofRun = (save: string): string => `${D.u[D.at[Section.heard] + Heard.run]}:${save}`, nameOf = (r: { key: string; savedAt: number }): string => `${r.key}@${r.savedAt}`;
  const pictures = new Map<string, string>(), pics = new SavePics<string>(gpu, canvas, () => 256, () => 0.6, (key, url) => { pictures.set(key, url); if (key === ofRun(key.slice(key.indexOf(':') + 1))) pause.pictured(key.slice(key.indexOf(':') + 1), url); }, e => console.warn('save picture:', e)); let saves = 0;
  let time = 0, acc = 0, drift = 0, paused = false, settling = false, thaw = false, ended = 0, onEnded = (): void => { /* no card is made yet */ };
  const tick = (dt: number): void => {
    if (thaw) { thaw = false; input.pressed.add('Paused'); input.pressed.add('Thawed'); }   // (the first tick after a card is told: what was held when it came up is let go unsent, in the tick)
    const keys = sampleHand(game.numbersOf(), input, cam, canvas);
    game.step(dt, keys);
    if (D.u[view() + View.stood] !== stood) anew();
    weather.told();
    events.from();
    D.fresh();   // (a sound asks the module, whose memory may have grown)
    lens();
    const o = view(), reeled = reel.step(dt), spectating = !reeled && D.u[o + View.spectating] !== 0;
    if (reeled) { follow[0] = reeled[0]; follow[1] = reeled[1]; follow[2] = reeled[2]; } else three(follow, o + View.follow);
    // come round a man spectated to a side he can be seen from, low where the view is close; else the usual tilt
    if (spectating) cam.autoYaw(dt, follow, ask.blocked);
    cam.easeYaw(dt, spectating);
    if (!cam.pin) cam.pitch += ((spectating && cam.desiredDistance < 12 ? LOW_PITCH : HIGH_PITCH) - cam.pitch) * Math.min(1, dt * 3);
    cam.step(dt, follow, spectating || reeled ? null : three(aim, o + View.aim));
    pair.capture(cam);
    weather.tick(dt, cam);
    { const n = D.u[o + View.saves]; if (n !== saves) { saves = n; const rows = game.saves().rows, all = new Set(rows.map(r => ofRun(nameOf(r)))); for (const k of pictures.keys()) if (!all.has(k)) pictures.delete(k); const newest = rows.reduce<typeof rows[number] | null>((a, r) => a && a.savedAt > r.savedAt ? a : r, null); if (newest && !pictures.has(ofRun(nameOf(newest)))) pics.want(ofRun(nameOf(newest))); } }
    told.tick = game.ms.step; told.draw = game.ms.draw;
  };
  const measure = new Measure({ gpu, engine, smoke, quality, cam, input, canvas, game, marks: () => decals.count, blocked: () => quality.calibrating ? 'auto quality is still measuring this machine' : title || brief?.held ? 'start a run first' : null, say: words => readout.say(words) });
  /** the world holds still: a card, the title, a mission not yet started */
  const stopped = (): boolean => paused || measure.freezing || title && !reel.live || !!brief?.held;
  /** one drawn frame: `dt` seconds of the display's, in whole ticks (`drawn` false: a script's, stepped and not drawn) */
  const frame = (dt: number, ticks?: number, drawn = true): void => {
    const still = ticks === undefined && stopped(); if (still) dt = 0;   // (the world holds still under a card: what only the picture moves with it)
    time += dt;
    const t0 = performance.now(), plan = ticks !== undefined ? { ticks, acc, drift } : still ? { ticks: 0, acc, drift } : planTicks(acc, dt, FIXEDSTEP, drift); acc = plan.acc; drift = plan.drift;
    const locks = D.u[view() + View.locks];
    if (input.wheelCtrl) { if (!still && !(locks & 2)) cam.zoom(input.wheelCtrl); input.wheelCtrl = 0; }
    if (input.orbitDX && plan.ticks && !(locks & 1)) cam.rotate(ctrlTurn(input.orbitDX));   // (in a frame's first tick: the next tick's hand looks the way it left the view)
    cam.frame();
    for (let i = 0; i < plan.ticks; i++) { if (i) input.endTick(); tick(1 / FIXEDSTEP.hz); }
    // (what this machine keeps of the run: its try as it begins, its end before the card of it)
    { const on = D.u[view() + View.run]; if (!unkept && books.due((on & 1) !== 0, (on & 2) !== 0)) books.tell(game.run()); }
    // (the run's end card, once each time one is due; a restart or a load is another end to come)
    { const e = D.u[view() + View.ended]; if (e !== ended) { ended = e; if (e) onEnded(); } }
    D.fresh();
    if (!measure.freezing) { skyFrom(engine, D); fogFrom(fog, D); }   // (a take over a held world flips what these write)
    rimsFrom(rims, D); weather.frame(cam); dustFrom(engine.particles!, D);
    { const o = view(); engine.settings.nvAmount = D.f[o + View.night]; engine.settings.hitFlash = D.f[o + View.hit]; }
    game.look(lookNumbers(engine.settings, engine.world, !!ch.cast));
    const t1 = performance.now();
    if (!drawn) { input.endFrame(); if (still) thaw = true; return; }
    { if (reel.live) { ear[0] = cam.target[0]; ear[1] = cam.target[1] + 1.6; ear[2] = cam.target[2]; } else { const o = view() + View.at; ear[0] = D.f[o]; ear[1] = D.f[o + 1] + 1.6; ear[2] = D.f[o + 2]; } audio.setListener(ear, cam.forward); audio.update(dt, time); audio.scoreMenu = title;
      D.fresh(); const h = D.at[Section.heard]; if (audio.music) { audio.music.intensity = D.f[h + Heard.tension]; if (D.u[h + Heard.run] !== run) audio.music.mission(); } run = D.u[h + Heard.run]; }
    smoke.update(dt);
    const a = ticks === undefined ? lag.at(acc * FIXEDSTEP.hz) : 1, pose = pair.at(a, cam);
    if (pose) cam.drawIn(pose);
    game.frame(a, cam.focus); lights.from(); men.from(engine.device.queue); reach.from(); world.from();
    cam.kickIn(); try { engine.render(cam, time, dt, true); } finally { cam.kickOut(); cam.drawOut(); }
    if (plan.ticks) input.endFrame(); else if (still) { input.endFrame(); thaw = true; }   // (pressed over a held world: nobody's, and the first tick after it is told)
    told.update = t1 - t0; told.render = performance.now() - t1; told.frames++;
    if (pics.wanted) pics.take();
    readout.frame();
  };
  // a page over the whole view (the brief, the start page): the still behind it is drawn until its histories are full, and then not again while it is up
  let covered = -1; const unseen = (): boolean => { if (settling || quality.measuring || measure.rec.recording || !(brief?.held || startPage())) { covered = -1; return false; } if (covered < 0) covered = told.frames; return told.frames - covered >= 90; };
  let last = performance.now(), held = q.has('paused');
  // THE PAGE'S PACE (host/pace.ts): a frame is not begun while two are still with the GPU, and a fast display's frames between the ladder's are passed over. A tab
  // nobody sees is not drawn; the time it was away is not the game's
  const pace = pacer(), rate = rateMeter(), rendered = [...RENDER_PASSES]; let resolved = -1;
  EVENPACE.on = true; EVENPACE.flight = FLIGHT_PAGE; PLAN_PACE.on = true;
  document.addEventListener('visibilitychange', () => { last = performance.now(); pace.reset(); rate.reset(); });
  /** a game lost to a panic stands again, and nothing of the lost one's last frame goes with it */
  let backAt = -1e9;
  const standAgain = (newest: boolean): string | null => {
    const label = game.recover(newest); if (label === null) return null;
    input.keys.clear(); input.buttons = 0; input.endFrame(); acc = 0; drift = 0;
    cam.kickOut(); cam.drawOut(); if (pause.up) pause.close(); paused = false; thaw = false; input.lockExcept = null; stood = 0; anew(); lens(); cam.rebuild();
    backAt = newest ? game.ticks : -1e9; last = performance.now(); requestAnimationFrame(loop);
    return label;
  };
  const loop = (ts: number): void => {
    try {
      const room = pageRoom({ calibrating: quality.calibrating, measuring: measure.active, kitView: false });
      if (held || document.hidden) { last = performance.now(); pace.reset(); rate.reset(); requestAnimationFrame(loop); return; }
      if (pace.skip(ts, !quality.calibrating && !measure.active && (quality.adaptive || quality.auto) ? quality.bands.target : 0, false, room)) { requestAnimationFrame(loop); return; }
      const now = performance.now(), raw = (now - last) / 1000; last = now; told.fps = rate.push(now);
      if (quality.early) quality.remeasure();
      if (quality.calibrating) quality.calibrateTick(raw);
      else if (raw < 0.5 && !stopped() && !measure.active) { const fresh = engine.timer.resolved !== resolved; resolved = engine.timer.resolved; quality.sample(raw, !engine.timer.enabled ? 0 : fresh ? engine.timer.unionMs(rendered) : NaN); }
      measure.begins(); reel.frame(raw); frame(Math.min(1 / 20, raw), undefined, !unseen()); measure.drawn(raw, told.fps, told.update, told.render);
      if (room > 0) { const w = pace.sent(performance.now()), end = (): void => pace.done(w, performance.now(), 0); gpu.device.queue.onSubmittedWorkDone().then(end, end); }
      requestAnimationFrame(loop);
    } catch (e) {
      // (lost to a panic: the picture stands as it was, and the card says what Enter puts back)
      if (e instanceof GameLost) { lostCard(document.getElementById('app')!, e, { kept: () => game.kept(), back: standAgain }, e.tick - backAt < 120); return; }
      console.error(e); showFatal(`The frame loop threw and has been stopped:\n\n${(e as Error)?.stack ?? e}`); }
  };
  // ---- the pause card: the world holds still under it
  const runSaves: RunSaves = { elapsed: () => game.saves().elapsed, timeline: () => game.saves().rows, offer: () => { const s = game.saves(); return s.rows.find(r => r.key === s.offer) ?? null; }, age: row => agoWords(game.saves().elapsed - row.missionT), picture: row => pictures.get(ofRun(nameOf(row))) ?? null, load: key => { game.commands.load(key); return null; }, due: () => false };
  const pause = new PauseCard({ title: desk().brief(location.search).title || name.replace(/\.json$/, ''), saves: runSaves, verdict: () => game.verdict(), may: () => !game.lost && !title && !brief?.up, restart: () => game.commands.restart('restarted'), quit: () => { reopenMissionsAfterLoad(); location.assign(location.pathname + desk().leave(location.search) + location.hash); }, turned: up => { paused = up; if (up) game.commands.letGo(); else thaw = true; input.keys.clear(); input.buttons = 0; input.endFrame(); input.lockExcept = up ? new Set() : null; last = performance.now(); } });
  onEnded = () => pause.end();
  // ---- the title stays up over its floor, live; a mission's run begins when the player starts it
  const drawTitle = (): void => { const root = document.getElementById('menu')!, c = document.createElement('div'); c.className = 'card'; root.textContent = ''; root.classList.add('on', 'title'); root.appendChild(c); titleScreen(c, { cheats: () => cheatWords(settings), closed: moved => { if (title && moved) drawTitle(); } }); };
  if (title) { drawTitle(); if (missionsReopenAsked() && !startPage()) openStartPage(); }
  else if (brief) startWaits(brief, { drawn: () => told.frames, measuring: () => quality.calibrating, settling: on => { settling = on; }, settled: () => { /* the block is live */ }, run: () => { input.keys.clear(); input.buttons = 0; input.endFrame(); thaw = true; audio.start(); input.focusGame(); last = performance.now(); } });
  else card.off();
  loading.style.display = 'none'; debuggerPanel({ game, engine, smoke, cam, quality, input, chars: ch, marks: decals, hud: () => ({ fps: told.fps, union: engine.timer.unionMs(rendered) }) });
  // (for a tool: the page's parts, and frames stepped by hand)
  Object.assign(window, { __draw: { game, books, measure, reel, pics, engine, world, audio, pause, cam, input, quality, told, fog, weather, smoke, get held() { return held; }, set held(v: boolean) { held = v; last = performance.now(); }, run: (n: number, keys: string[] = [], drawn: 'none' | 'last' | 'all' = 'last') => { if (title || brief?.up) { title = false; reel.poke(); brief?.off(); card.off(); } for (const k of keys) input.keys.add(k); try { for (let i = 0; i < n; i++) frame(1 / 60, 1, drawn === 'all' || (drawn === 'last' && i === n - 1)); } finally { for (const k of keys) input.keys.delete(k); } } } });
  requestAnimationFrame(loop);
}
// (a tool waits for the programs made behind the start too: until then a pass draws its stand-in, and the fog of war is the lit one)
const ready = main(); (window as unknown as { __ready: Promise<void> }).__ready = ready.then(() => laterIdle());
ready.catch(e => { console.error(e); if ((e as Error)?.name === 'RustGameError') showFatal((e as Error).message); else showFatalError(e); });
