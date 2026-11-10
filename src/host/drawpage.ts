// THE GAME'S PAGE (index.html), which draws a game it does not hold. The game is the Rust module's (host/drawgame.ts): started from the
// level's file and the address's words, handed what the player did, stepped, and drawn from what each tick leaves to be drawn (src/draw). Nothing this
// page's bundle holds is the game's TypeScript: tools/draw/ban.ts stops the build on an import of it.
import { kitPicture } from './kitguard';   // (FIRST: in the Team & kit picture's window it makes the stores read-only before any module below reads or writes one)
import { PIPE_QUEUE, gpuStory, initGpu, laterIdle, pipeQueue, showFatal, showFatalError, pipelineProgress } from '../gpu/device';
import { Engine } from '../engine';
import { SmokeSystem } from '../smoke/smoke';
import { Quality } from '../ui/quality';
import { glassBreakConfig, lookConfig, snowConfig, snowLookConfig, weatherKindConfig } from '../render/frame';
import { lookParts, lookSync, nextTask } from '../render/gbuffer';
import { hourExposure } from '../render/photometry';
import { charSlots, meshAuthored } from '../render/charMaterials';
import { MeshShadows } from '../render/meshShadows';
import { fitRig, fitRigCast } from '../render/sdfrig';
import { initCutSwitch } from '../scene/cut';
import { CAST_FILE, CAST_FILE_LOOK, castUrlFor, loadCharacterSet } from '../anim/gltf';
import { CharacterRenderer } from '../anim/characters';
import { COATFINISH, SOLIDGLASS } from '../scene/stuff';
import { MARKING_TEXEL } from '../scene/markings';
import { lookKeptOf, placeOfQuery, setLookPlace } from '../scene/look';
import { Input } from './input';
import { FollowCamera } from './camera';
import { FIXEDSTEP, planTicks } from './fixedstep';
import { CameraPair, Lag } from './between';
import { FINDTUNED, bindFindMe, ctrlTurn, findMeStarts } from './findme';
import { fetchFace } from './face';
import { machineAfter, machineKept, machineStood } from './machine';
import { EVENPACE, pacer, pageRoom, rateMeter } from './pace';
import { RENDER_PASSES } from '../gpu/timer';
import { DrawGame, GameLost } from './drawgame';
import { debuggerPanel } from './debugger';
import { lostCard } from './lostcard';
import { Reel } from './reel';
import { Measure } from './measure';
import { oneAirFlip, wispMotesFlip } from '../ui/sweepSnow';
import { drawBench } from './drawbench';
import { DrawHud } from './drawhud';
import { drawEnd } from './drawend';
import { nextLessonOf } from '../ui/verdictNext';
import { DrawPlay } from './drawplay';
import { DrawNet, netAsked } from './drawnet';
import { COOP } from '../net/lobby';
import { HOSTED, coopFromUrl, coopInvite, coopRows } from '../ui/coop';
import { netKeyed, netLeave } from '../net/door';
import { NETPLAY, NET_JOIN } from '../net/address';
import { keptBeside, keptName, tapeAddress, tapeAsked } from './tapepage';
import { sessionId } from './session';
import { TapeBar, type Played, type Sees } from '../tape/bar';
import { TapeMap } from '../tape/map';
import { soundLanes, type LaneTape } from '../tape/soundlanes';
import { FreeCam } from '../tape/freecam';
import { PLAYTAPE } from '../tape/address';
import { RunKeep, keeps } from '../tape/run';
import { backToGame, gameUp } from '../tape/format';
import { keptStore } from '../tape/keptui';
import { cloudNotes } from '../cloud/cloud';
import { buildName } from './buildname';
import { HandTape, TapeRefused } from '../tape/handtape';
import { MODULE } from '../tape/module';
import { refusedCard } from '../tape/refused';
import { drawCloud } from './drawcloud';
import { cloudAdopt } from '../cloud/cloud';
import { tellRedCards } from '../cloud/redcards';
import { tapConsole } from '../cloud/report';
import { cloudNote } from '../ui/cloudnote';
import { keptFreeze, startCallClock } from '../gpu/keep';
import * as boot from '../ui/boot';
import { BENCH_RUN_KEY, benchBegin, benchSceneWaits } from '../ui/benchrun';
import { PRICE_KEY, PRICE_RUN_KEY, priceLevelsPage } from '../ui/pricerun';
import { urlFlagsOnEngine } from './engineflags';
import { pictureKeepsPace, pictureMakes, pictureOpens, pictureSolves, pictureWords } from './pictureopens';
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
import { KitPage, kitNone } from './kitpage';
import { Desk, desk, setDesk, type DeskFace } from '../view/desk';
import type { BriefView } from '../view/read';
import { nextPickOf } from '../ui/picks';
import { keepPageBoot } from './keeppage';
import { heldPage, ownPage, runPage } from './runpage';
import { bootTextSize } from '../ui/textsize';
import { Fog, Heard, Man, Section, Sky, View } from '../draw/sections';
import { AudioEngine, SFX_TABLE, SFX_WARM, warmSfx } from '../audio/audio';
import { setlistLevel, setlistOf } from '../audio/setlist';
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
import { HAND, sampleHand } from '../draw/hand';
import { HOURS_OF, skyFrom } from '../draw/sky';
import { fogFrom } from '../draw/fog';
import { smokeFrom } from '../draw/smoke';
import { makeFogFrame } from '../render/fogofwar';
import { WeatherPicture } from '../picture/weather';
import { SNOW_LOOK, snowLookOfQuery } from '../picture/snow/look';
import type { Vec3 } from '../math/vec';

/** ticks played and thrown away before the first */
const WARM = 36;
const LOW_PITCH = 24 * Math.PI / 180, HIGH_PITCH = 55 * Math.PI / 180;
const bytesOf = async (url: string): Promise<Uint8Array> => { const r = await fetch(url, { cache: 'no-store' }); if (!r.ok) throw new Error(`${url}: HTTP ${r.status}`); return new Uint8Array(await r.arrayBuffer()); };
/** the level's file */
const levelBytes = (name: string): Promise<Uint8Array> => bytesOf(`levels/${name}`);

const NO_SAVE = 'You can\u2019t save a co-op game.';

async function main(): Promise<void> {
  const q = new URLSearchParams(location.search), loading = document.getElementById('loading')!; let say = (line: string): void => { loading.textContent = line; };
  // THE KEPT PAGE (gpu/keep.ts): the tab's own page keeps the GPU device and shows the game in a frame of this address, so a mission's start compiles nothing this visit has compiled
  const keep = keepPageBoot(q, ownPage(q), kitPicture); if (keep.role === 'shell' || keep.role === 'leave') { boot.stopRafProbe(); return; }
  if (!kitPicture && await cloudAdopt(q)) return;   // (a browser handed another's id takes it and its settings before a key is read, then loads once more without the word)
  const tape = kitPicture ? null : await tapeAsked(q); if (tape) { const to = tapeAddress(tape, q); if (to !== null) { location.replace(location.pathname + to + location.hash); return; } }   // (a tape plays at its run's own address)
  // (a co-op game's code is given a seat and sent on to its server; a server's page comes up at its game's own address)
  if (COOP.on && !tape && !kitPicture && q.has(NET_JOIN) && !(q.has(NETPLAY) && netKeyed(q))) { if (await coopFromUrl(q) === 'moved') return; q.delete(NET_JOIN); }
  const coop = tape || kitPicture ? null : await netAsked(q); if (coop === 'moved') return;
  TITLE.coop = () => COOP.on ? coopRows : null;
  // (a game hosted from here stands on this machine's settings; both players play the kit as issued)
  HOSTED.setup = () => ({ kept: machineKept(true) });
  const canvas = document.getElementById('view') as HTMLCanvasElement;
  // (the Team & kit picture has no HUD)
  if (kitPicture) for (const el of document.getElementById('app')!.children) if (!['view', 'panel', 'lcol', 'veil', 'loading', 'menu', 'missions', 'fatal'].includes(el.id)) (el as HTMLElement).style.display = 'none';
  gameKeysBoot(q, walkInNow); bootTextSize(q, kitPicture);
  setLookPlace(placeOfQuery(q));
  const name = coop?.start.file ?? (/^[\w.-]+\.json$/.test(q.get('levelFile') ?? '') ? q.get('levelFile')! : 'office.json'), at = (p: string) => new URL(p, document.baseURI).href;

  // ---- the game's module
  const faceP = fetchFace(say).then(face => { MODULE.name = face.name ?? '';
    // THE DESK: the cards before a game ask the module, and what a press keeps goes in this machine's store, where the next page's start reads it
    setDesk(new Desk(face.exports as unknown as DeskFace, face.exports.memory as WebAssembly.Memory, (key, text) => { try { if (text) localStorage.setItem(key, text); else localStorage.removeItem(key); } catch { /* no store: nothing is kept */ } }));
    desk().machine(machineKept(), location.search); return face; });
  faceP.catch(() => { /* told where it is awaited, below */ });
  // (an address that may only say where to go waits for the module before anything else is fetched: a row of the short list named and nothing to play goes to that row's own page, the benchmark asked
  //  for to its first scene's, the pricing run to its first level's. A page of a pricing run under way takes its part here, before the picture boots)
  let refused: string | null = null;
  if (q.has('bench') || q.has(PRICE_KEY) || q.has(PRICE_RUN_KEY) || (q.has('pick') && !q.has('levelFile') && !q.has('solo'))) { await faceP;
    const to = tape || coop ? '' : desk().pick(location.search); if (to) { location.replace(location.pathname + to + location.hash); return; }
    if (q.has('bench') && !q.has(BENCH_RUN_KEY)) { refused = benchBegin(false); if (refused === null) return; console.info(`benchmark: ${refused}`); }
    if (priceLevelsPage(q)) return; }

  SFX_TABLE.was = q.has('oldsounds'); SFX_TABLE.redone = q.has('newsounds'); if (q.has('bench') || q.has(BENCH_RUN_KEY)) SFX_WARM.on = false; else if (!q.has('mute')) warmSfx();   // (the sounds, a voice at a time in idle moments)

  // ---- the game
  const tBoot = performance.now();
  const chP = loadCharacterSet(at('assets/ual/AnimationLibrary_Godot_Standard.gltf'), castUrlFor(location.search, document.baseURI)); chP.then(() => boot.span('gltf', tBoot), () => { /* awaited below */ });
  const tFiles = performance.now(), filesP = Promise.all([tape ? new Uint8Array(0) : coop ? bytesOf(`levels/${name}`) : levelBytes(name).then(b => { boot.span('level', tFiles); return b; }), bytesOf(at('assets/ual/AnimationLibrary_Godot_Standard.gltf')), bytesOf(at('assets/ual/AnimationLibrary_Godot_Standard.bin')), bytesOf(at(`assets/chars/${CAST_FILE}`)), bytesOf(at(`assets/chars/${CAST_FILE_LOOK}`))]); filesP.catch(() => { /* awaited below */ });
  boot.expose();
  const face = await faceP;
  // (an address that plays a mission comes up on that mission's brief, one that plays nothing on the title; either is live from here, and the game is made behind it)
  LOADBRIEF.on = q.get('loadbrief') !== '0'; settleOfQuery(q);
  LOADBRIEF.options = { open: () => openOptions({ tab: 'controls' }), close: closeOptions, up: optionsOpen };
  watchMissionRows();   // (each mission's card shows the ways this machine has won it)
  let title = !kitPicture && !tape && !coop && startPageWanted(); const brief = kitPicture || tape || coop || q.has('paused') || runPage(q) ? null : loadBrief(location.search), card = kitPicture ? { status: () => { /* nobody reads it */ }, off: () => { /* */ } } : brief ?? bootCard();
  loading.textContent = ''; say = line => card.status(line);
  const [level, mannequin, mannequinBin, cast, castLook] = await filesP;
  say('starting the game …');
  boot.mark('module'); const tGame = performance.now();
  const game = new DrawGame(face), asked0 = Number(q.get('seed') ?? NaN), seed = Number.isFinite(asked0) ? asked0 >>> 0 : crypto.getRandomValues(new Uint32Array(1))[0], sid = sessionId(); let built = '';
  const unkept = kitPicture || !!tape || !!coop || ownPage(q);   // (what this machine keeps of the runs played: a page that plays a run of its own is nobody's)
  const named = q.get(PLAYTAPE) ?? '', beside = tape && keptName(named) ? await keptBeside(named) : null;   // (a file says of no module that it was played on it)
  const play = tape ? new DrawPlay(game, tape, !!beside && !beside.other) : null;
  const net = coop ? new DrawNet(coop.door, coop.start, game, () => deaf()) : null; let deaf = (): void => { /* nothing sounds yet */ };
  if (play) play.start({ mannequin, mannequinBin, cast, castLook });
  else if (net) net.start({ level, mannequin, mannequinBin, cast, castLook });
  else {
    // (the run's tape: not of a title's floor, which nobody plays, nor of a page that measures)
    if (!title && !unkept && !q.has('noreplay')) game.tape = new HandTape(HAND.count);
    if (game.tape) built = await buildName();
    game.start({ level, words: location.search, kept: machineKept(), mannequin, mannequinBin, cast, castLook, seed, named: { level: name, build: built } });
    machineStood(game);
    machineAfter(game.commands, !benchSceneWaits() && !heldPage(q));
  }
  game.tell();   // (the title and Options read what is kept before the first step)
  boot.span('gameCtor', tGame); boot.mark('game');
  const D = game.drawn, sky = () => D.at[Section.sky], view = () => D.at[Section.view];

  // ---- the picture's constants: what the shaders are compiled with is said before any of them is
  pictureWords(q);
  lookConfig.kept = lookKeptOf(D.f[sky() + Sky.kept] < 0 ? undefined : D.f[sky() + Sky.kept], q.get('kept'));
  weatherKindConfig.on = true; snowConfig.on = D.u[sky() + Sky.snowLevel] !== 0;
  if (snowConfig.on) { snowLookOfQuery(q); snowLookConfig.surface = SNOW_LOOK.on; snowLookConfig.wisps = SNOW_LOOK.wisps; snowLookConfig.tracks = SNOW_LOOK.on && SNOW_LOOK.tracks; snowLookConfig.glow = SNOW_LOOK.on && SNOW_LOOK.glow; snowLookConfig.sparkle = SNOW_LOOK.on && SNOW_LOOK.sparkle; snowLookConfig.under = SNOW_LOOK.under; snowLookConfig.grains = SNOW_LOOK.grains || SNOW_LOOK.oneAir; }
  glassBreakConfig.on = true; initCutSwitch();
  COATFINISH.on = true; SOLIDGLASS.on = true;

  if (kitPicture) PIPE_QUEUE.later = false;   // (the picture waits for every program: it is shown once, and nothing of it may change as one comes)
  const tGpu = performance.now();
  const gpu = await initGpu(canvas, { retry: !(kitPicture || q.has('bench') || runPage(q) || navigator.webdriver === true), canvasRead: !kitPicture, keeper: keep.keeper, seat: keep.seat });   // (the module's game has stood by now)
  if (kitPicture) void gpu.device.lost.then(kitNone);
  if (q.get('framecap') === '0') FRAMECAP.on = false;
  if (FRAMECAP.on) watchTextures(gpu.device);   // (a frame capture knows every texture made from here on)
  boot.span('gpu', tGpu); boot.mark('gpu'); startCallClock(gpu.device);
  let world!: DrawWorld; const engine = new Engine(gpu, undefined, d => world = new DrawWorld(d, D)), smoke = new SmokeSystem(engine);
  urlFlagsOnEngine(engine); pictureSolves(q, engine, smoke);
  engine.setParticles(new Particles(engine.device, engine.sceneLayout));
  engine.setSmoke(smoke.binding); engine.afterSubmit.push(() => smoke.afterSubmit());
  const decals = new DrawDecals(engine.device, D); engine.setDecals(decals); trafficFrom(engine.gbuffer, D);
  engine.settings.markingTexel = D.f[sky() + Sky.markTexel] || MARKING_TEXEL;
  pipelineProgress.onChange = () => { gpuStory.stage = `while compiling shaders, ${pipelineProgress.done} of ${pipelineProgress.total}`; say(`compiling shaders ${pipelineProgress.done} / ${pipelineProgress.total} …`); };
  // the men: the mesh and its skinning, the tree that shadows him, the cones the rays meet of him
  const ch = await chP, chars = new CharacterRenderer(engine.device, ch), shadows = new MeshShadows(engine.device, ch, chars.jointMats, chars.charInsts);
  engine.skin = chars; engine.setMeshShadows(shadows);
  if (ch.cast) { lookParts.suit = ch.cast.parts.indexOf('op_suit') + 1; lookParts.rig = ch.cast.parts.indexOf('chest_rig') + 1; engine.gbuffer.setLook(); }
  if (meshAuthored(ch.mesh)) { engine.settings.charMats = true; charSlots(ch); }
  const men = new DrawMen(ch, chars, engine.world, D, () => engine.settings.charMats);
  { const fit = fitRig(ch); engine.setRigFit(ch.cast ? { ...fit, byOwner: men.fits } : fit); if (ch.cast) fitRigCast(ch); }
  game.look(lookNumbers(engine.settings, engine.world, !!ch.cast)); game.frame(1, [20, 0, 14]); engine.world.upload();   // (every stuff the level is made of is known from here: its maps are made with the programs)
  const tPipe = performance.now();
  const group = (name: string): void => { boot.span('pipelines_' + name, tPipe); }, timed = <T>(name: string, p: Promise<T>): Promise<T> => p.then(v => { group(name); return v; });
  const made = Promise.all([engine.init({ ...pictureMakes(q), onGroup: group }), timed('smoke', smoke.init()), timed('particles', engine.particles!.init()), timed('materials', engine.gbuffer.needLook()), timed('meshShadows', shadows.init()), timed('skin', engine.gbuffer.prepareSkin(chars.jointsFormat))]);
  made.catch(() => { /* awaited below */ });
  // (while the programs compile: the ticks a run begins with, still, walking and firing)
  if (!play && !kitPicture && !title) { const eye = new FollowCamera(), o = view() + View.follow, none = new Set<string>(), walk = new Set(['KeyW']), w = Math.max(1, canvas.clientWidth), h = Math.max(1, canvas.clientHeight);
    eye.target = [D.f[o], D.f[o + 1], D.f[o + 2]]; eye.aspect = w / h; eye.rebuild();
    await game.warm(WARM, (N, i) => sampleHand(N, { keys: i >= WARM / 3 ? walk : none, pressed: none, buttons: i >= WARM * 2 / 3 ? 1 : 0, clicked: 0, wheel: 0, mouseX: w * 0.6, mouseY: h * 0.4 }, eye, canvas), nextTask); }
  await made;
  boot.span('pipelines', tPipe); boot.mark('pipelines'); boot.freezePipelines(); keptFreeze(); gpuStory.stage = 'while setting up'; pipelineProgress.onChange = null; pipeQueue.settle(); engine.warmFirstUse();
  const quality = new Quality(engine, smoke); quality.visit(location.search); pictureOpens(q, engine, quality); quality.onCalibrated = refreshOptions; (window as unknown as { __quality: Quality }).__quality = quality;
  lookSync();
  lookConfig.exposure = () => { const h = HOURS_OF[D.u[sky() + Sky.hour]]; return h === 'night' ? null : hourExposure(h, D.f[sky() + Sky.hourSun]); };
  { const own = { zenith: [0, 0, 0] as Vec3, horizon: [0, 0, 0] as Vec3 }; for (let i = 0; i < 3; i++) { own.zenith[i] = D.f[sky() + Sky.ownZenith + i]; own.horizon[i] = D.f[sky() + Sky.ownHorizon + i]; } lookConfig.nightSky = () => own; }
  lookConfig.snow = () => snowConfig.on;
  { const eye: [number, number] = [0, 0]; lookConfig.eyeAt = () => { const o = view() + View.at; eye[0] = D.f[o]; eye[1] = D.f[o + 2]; return eye; }; }
  const gaps = doorways(D); { const lines = gaps.map(d => [d.hinge[0], d.hinge[1], d.hinge[0] + Math.cos(d.closedDir) * d.width, d.hinge[1] + Math.sin(d.closedDir) * d.width] as [number, number, number, number]); lookConfig.doorGaps = () => lines; }
  const fog = makeFogFrame(), ask = askOf(game), audio = new AudioEngine(ask), events = new DrawEvents(D, engine.particles, smoke, audio, i => game.nameOf(i));
  MUSIC.pieces = MUSIC.byLevel = true; audio.scoreSet = setlistOf(setlistLevel(q.get('levelFile')), desk().kind(location.search) || null);   // (the level's own music)
  if (q.has('mute')) { audio.volume.master = 0; audio.start = () => { /* silent */ }; }
  const hud = kitPicture ? null : new DrawHud(game, ask); if (hud && (play || q.get('cards') === '0')) hud.overlay.cards = false;   // (a run being played again waits for nobody)
  deaf = () => audio.hush();
  const settings = settingsOf(game, hud?.overlay); setOptionsDeps({ engine, quality, audio, game: settings });   // (what this machine keeps of the volumes and the wall hold is put on them)
  for (const ev of ['pointerdown', 'keydown'] as const) window.addEventListener(ev, () => audio.start(), { capture: true });   // (a browser's sound begins with a hand on the page)
  engine.fogFrame = fog; const rims = engine.outline = makeOutlineFrame(); engine.beams = world.beams;
  const lights = new DrawLights(engine.lights, D); lights.from();   // (the lights stand too: their reach is worked out of the level as it opens)
  const reach = new DrawReach(engine, world, D, gaps);
  const weather = new WeatherPicture(engine, D, world, engine.particles, smoke);
  audio.outdoors = weather.roofs.outdoors;

  // ---- the player
  const input = new Input(canvas); if (hud) hud.overlay.cardClosed = () => input.focusGame(); input.saveKey = () => !title && !play && !brief?.up;   // (F5 is the game's while a run is on: a save by hand)
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
  const books = new RunBooks();   // (what this machine keeps of the runs played: a page that plays a run of its own is nobody's)
  /** the game is another than the one last drawn (a restart, a load): what the picture and the sound carry over from frame to frame starts over, before anything of the new one is played */
  const anew = (): void => {
    const o = view(), first = stood < 0; stood = D.u[o + View.stood]; if (first) { events.stands(); return; }
    engine.restartPicture(); engine.particles?.clear(); smoke.anew(); events.cut(); events.stands(); weather.cut(); audio.hush(); game.turnAgain(); pair.cut(); lag.reset(); books.anew(); three(cam.target, o + View.follow);
  };
  anew(); lens(); cam.aspect = Math.max(1, canvas.clientWidth) / Math.max(1, canvas.clientHeight); cam.rebuild();

  const told = { update: 0, render: 0, tick: 0, draw: 0, hud: 0, frames: 0, fps: 0 }, readout = new Readout({ engine, smoke, quality, adapter: gpu.adapterInfo, men: () => D.n[Section.men], told }, q);
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
  let time = 0, acc = 0, drift = 0, paused = false, settling = false, thaw = false, owed = false, ended = 0, onEnded = (): void => { /* no card is made yet */ };
  // ---- a tape being played: what a bar over it turns
  const firstAt = Math.max(0, Math.floor(Number(q.get('at')) || 0));
  const played: Played & { to: number; readonly ends: string } = { paused: !!play, pace: 1, to: firstAt > 0 ? firstAt : -1, get seeking() { return this.to >= 0; }, get at() { return play?.at ?? 0; }, get total() { return play?.total ?? 0; }, get ends() { return play?.ends ?? ''; }, get parted() { return play?.parted?.tick ?? null; }, get lost() { return play ? (play.lost ?? game.lost)?.words.split('\n')[0] ?? null : null; }, get tries() { return play?.tries ?? []; }, seek(t: number): void { played.to = Math.max(0, Math.min(played.total, Math.round(t))); } };
  if (!play && !kitPicture) gameUp();
  let bar: TapeBar | null = null, laneTape: LaneTape | null = null; const runView = (): { look: [number, number, number]; yaw: number; pitch: number; dist: number; fov: number } => { const n = play?.next; return n && play!.total ? { look: [n[HAND.target], n[HAND.target + 1], n[HAND.target + 2]], yaw: n[HAND.yaw], pitch: n[HAND.pitch], dist: n[HAND.distance], fov: n[HAND.fov] } : { look: [cam.target[0], cam.target[1], cam.target[2]], yaw: cam.yaw, pitch: cam.pitch, dist: cam.distance, fov: cam.fov }; };
  // whoever watches a run sees it his own way: the fog off his picture, a tube or none whatever the run's goggles were (null: as the run had it), eased by the page's clock
  const watched = { fog: true, tube: null as boolean | null, k: 0 }, seesAs = (fog: boolean, tube: boolean | null): void => { watched.fog = fog; watched.tube = tube; if (hud) hud.page.bared = !fog; game.viewAs(!fog, tube); };
  const tubeOf = (runs: number, dt: number): number => { if (!play) return runs; const to = watched.tube === null ? runs : watched.tube ? 1 : 0, by = dt * 4; if (watched.tube === null && Math.abs(runs - watched.k) <= by) return watched.k = runs; return watched.k += Math.max(-by, Math.min(by, to - watched.k)); };
  if (play) seesAs(true, null);
  const freeCam = play ? new FreeCam(canvas, runView, () => cam.aspect, () => !!bar?.busy, ctrlTurn) : null;
  /** the run's own view of the next tick, or wherever the picture's camera was taken; false = the page follows him itself */
  const posed = (): boolean => {
    if (!freeCam || !play!.total) return false;
    const v = freeCam.view(); cam.target[0] = v.look[0]; cam.target[1] = v.look[1]; cam.target[2] = v.look[2]; cam.yaw = v.yaw; cam.pitch = v.pitch; cam.distance = cam.desiredDistance = v.dist; cam.fov = v.fov; cam.rebuild(); return true;
  };
  posed();
  /** a place on the tape asked for: played to unseen, some of it a frame; true = it is there, and the picture is of it */
  const sought = (): boolean => {
    if (!play || played.to < 0) return true;
    if (!play.seek(played.to, 24)) return false;
    played.to = -1; if (game.lost) return false; anew(); lens(); posed(); pair.cut(); acc = 0; drift = 0; return true;
  };
  /** the walk was lost to a panic: the picture stands as it was, and any place before it can be gone back to */
  const lostWalk = (): void => { console.error(play!.lost!.message); readout.say(`${play!.ends === play!.lost!.message ? 'the run stopped here when it was played' : 'the replay stopped here'}: ${play!.lost!.words.split('\n')[0]}`); };
  /** what a tick of the game's did once is played, and what it left for the view is taken */
  const told1 = (): void => {
    if (D.u[view() + View.stood] !== stood) anew();
    weather.told();
    events.from();
    D.fresh();   // (a sound asks the module, whose memory may have grown)
    lens();
  };
  const tick = (dt: number): void => {
    if (thaw) { thaw = false; if (!net) { input.pressed.add('Paused'); input.pressed.add('Thawed'); } }   // (the first tick after a card is told: what was held when it came up is let go unsent, in the tick)
    owed = false;   // (a card asked for another game, and this tick stands it, whatever else holds the world still)
    if (play) { if (!play.tick()) { played.paused = true; if (game.lost) lostWalk(); return; } }
    else if (net) { if (net.tick(input, cam, canvas, stopped(), told1)) { anew(); lens(); } }
    else game.step(dt, sampleHand(game.numbersOf(), input, cam, canvas));
    if (!net) told1();
    const o = view(), reeled = reel.step(dt), spectating = !reeled && D.u[o + View.spectating] !== 0;
    if (reeled) { follow[0] = reeled[0]; follow[1] = reeled[1]; follow[2] = reeled[2]; } else three(follow, o + View.follow);
    const led = net?.lead ?? null; if (led) for (let i = 0; i < 3; i++) follow[i] += led.by[i];   // (his own man is drawn ahead of the game, and looked at where he is drawn)
    // come round a man spectated to a side he can be seen from, low where the view is close; else the usual tilt
    if (spectating) cam.autoYaw(dt, follow, ask.blocked);
    cam.easeYaw(dt, spectating);
    if (!cam.pin) cam.pitch += ((spectating && cam.desiredDistance < 12 ? LOW_PITCH : HIGH_PITCH) - cam.pitch) * Math.min(1, dt * 3);
    if (!posed()) cam.step(dt, follow, spectating || reeled ? null : led?.aim ?? three(aim, o + View.aim));
    pair.capture(cam);
    weather.tick(dt, cam);
    { const n = D.u[o + View.saves]; if (n !== saves) { saves = n; const rows = game.saves().rows, all = new Set(rows.map(r => ofRun(nameOf(r)))); for (const k of pictures.keys()) if (!all.has(k)) pictures.delete(k); const newest = rows.reduce<typeof rows[number] | null>((a, r) => a && a.savedAt > r.savedAt ? a : r, null); if (newest && !pictures.has(ofRun(nameOf(newest)))) pics.want(ofRun(nameOf(newest))); } }
    told.tick = game.ms.step; told.draw = game.ms.draw;
  };
  const measure = new Measure({ gpu, engine, smoke, quality, cam, input, canvas, game, marks: () => decals.count, blocked: mine => quality.calibrating ? 'auto quality is still measuring this machine' : (title || brief?.held) && !mine ? 'start a run first' : null, say: words => readout.say(words) });
  { const s = weather.snow?.fx.swept; if (s) { measure.sweep.oneAir = () => oneAirFlip(s); measure.sweep.wispMotes = () => wispMotesFlip(s); } }
  /** the world holds still: a card, the title, a mission not yet started */
  const stopped = (): boolean => paused || !!play && played.paused || measure.freezing || title && !reel.live || !!brief?.held || !!hud?.holds && !owed;
  /** the page's own keys, ahead of the tick: a press that is one of them is nobody else's (a frame with no tick keeps the presses for the next) */
  const pageKeys = (): void => {
    if (!hud || play) return;
    if (hud.tabUp && (paused || optionsOpen() || hud.overlay.cardUp)) hud.tabShow(false);   // (a card or a screen over Tab's puts it away)
    const still = stopped();
    if (!title && input.hit('Tab') && hud.tabKey(!still)) input.pressed.delete('Tab');
    if (net && input.hit('F5')) { input.pressed.delete('F5'); net.door.pass(NO_SAVE); }
    if (hud.tabUp && input.hit('F5') && input.saveKey?.()) { input.pressed.delete('F5'); game.commands.save(); hud.tabShow(false); }   // (written as play goes on)
    if (still) return;
    if (input.hit('BracketLeft')) { engine.settings.exposure /= 1.25; input.pressed.delete('BracketLeft'); }
    if (input.hit('BracketRight')) { engine.settings.exposure *= 1.25; input.pressed.delete('BracketRight'); }
  };
  const kit = kitPicture ? new KitPage(canvas, game.commands) : null;
  /** one drawn frame: `dt` seconds of the display's, in whole ticks (`drawn` false: a script's, stepped and not drawn) */
  let beat = -1;   // (the display's own time for this frame, s; -1 = not known)
  const frame = (dt: number, ticks?: number, drawn = true): void => {
    if (freeCam) { freeCam.frame(dt); bar?.paint(); }
    if (ticks === undefined) pageKeys(); else hud?.tabShow(false);
    if (!sought()) return;
    if (play && game.lost) { input.endFrame(); return; }
    const wall = dt; dt *= played.pace; if (freeCam && played.paused) { posed(); pair.cut(); }
    const still = ticks === undefined && !net && stopped(); if (still) dt = 0;   // (the world holds still under a card: what only the picture moves with it)
    time += dt;
    const t0 = performance.now(), plan = ticks !== undefined ? { ticks, acc, drift } : still ? { ticks: 0, acc, drift } : planTicks(acc, ticks === undefined && beat >= 0 ? beat * ((window as unknown as { __timeScale?: number }).__timeScale ?? 1) * played.pace : dt, FIXEDSTEP, drift); acc = plan.acc; drift = plan.drift;
    const locks = D.u[view() + View.locks];
    if (input.wheelCtrl) { if (!still && !(locks & 2)) cam.zoom(input.wheelCtrl); input.wheelCtrl = 0; }
    if (input.orbitDX && plan.ticks && !(locks & 1)) cam.rotate(ctrlTurn(input.orbitDX));   // (in a frame's first tick: the next tick's hand looks the way it left the view)
    cam.frame(); kit?.before(Math.max(1, canvas.clientWidth) / Math.max(1, canvas.clientHeight));
    for (let i = 0; i < plan.ticks && !game.lost; i++) { if (i) input.endTick(); tick(1 / FIXEDSTEP.hz); }
    if (net?.door.far) { if (plan.ticks) input.endFrame(); return; }   // (ticks nobody sees: the picture stands until the page is level)
    if (play && game.lost) { input.endFrame(); return; }   // (the picture stands as it was: any place before it can be gone back to)
    // (what this machine keeps of the run: its try as it begins, its end before the card of it)
    { const on = D.u[view() + View.run]; if (!unkept && books.due((on & 1) !== 0, (on & 2) !== 0)) books.tell(game.run()); }
    // (the run's end card, once each time one is due; a restart or a load is another end to come)
    { const e = D.u[view() + View.ended]; if (e !== ended) { ended = e; if (e) onEnded(); } }
    D.fresh();
    if (!measure.freezing) { skyFrom(engine, D); fogFrom(fog, D); }   // (a take over a held world flips what these write)
    smokeFrom(smoke, D);
    weather.frame(cam); dustFrom(engine.particles!, D);
    { const o = view(); engine.settings.nvAmount = tubeOf(D.f[o + View.night], wall); engine.settings.hitFlash = D.f[o + View.hit]; }
    game.look(lookNumbers(engine.settings, engine.world, !!ch.cast));
    const t1 = performance.now();
    if (!drawn) { input.endFrame(); if (still) thaw = true; return; }
    { if (reel.live) { ear[0] = cam.target[0]; ear[1] = cam.target[1] + 1.6; ear[2] = cam.target[2]; } else { const o = view() + View.at; ear[0] = D.f[o]; ear[1] = D.f[o + 1] + 1.6; ear[2] = D.f[o + 2]; } audio.setListener(ear, cam.forward); audio.update(dt, time); audio.scoreMenu = title;
      D.fresh(); const h = D.at[Section.heard]; if (audio.music) audio.music.intensity = D.f[h + Heard.tension]; }
    for (const s of world.stale) smoke.obstaclesChanged(s.c, s.r);   // (what stands is another thing there: the last frame's rows say where)
    world.stale.length = 0; smoke.update(dt);
    const a = ticks === undefined ? lag.at(acc * FIXEDSTEP.hz) : 1, pose = pair.at(a, cam);
    if (pose) cam.drawIn(pose);
    game.frame(a, cam.focus); if (!kit) rimsFrom(rims, D); lights.from(); men.from(engine.device.queue); reach.from(); world.from();
    cam.kickIn(); try { engine.render(cam, time, dt, true); const tH = performance.now(); if (hud) { hud.blank = title && !reel.live; hud.told(); hud.draw(cam, canvas); } told.hud = performance.now() - tH; laneTape?.drawn?.(); } finally { cam.kickOut(); cam.drawOut(); }
    kit?.after(D.u[view() + View.kit], D.u[view() + View.veil]);
    if (plan.ticks) input.endFrame(); else if (still) { input.endFrame(); thaw = true; }   // (pressed over a held world: nobody's, and the first tick after it is told)
    told.update = t1 - t0; told.render = performance.now() - t1 - told.hud; told.frames++;
    if (told.frames === 1) { boot.span('firstFrameCpu', t1); boot.mark('firstSubmit'); gpu.device.queue.onSubmittedWorkDone().then(() => { boot.span('firstFrameGpu', t1); boot.mark('gpuReady'); }, () => { /* the device went */ }); }
    if (pics.wanted) pics.take();
    readout.frame();
  };
  // a page over the whole view (the brief, the start page): the still behind it is drawn until its histories are full, and then not again while it is up
  let covered = -1; const unseen = (): boolean => { if (settling || quality.measuring || measure.rec.recording || !(brief?.held || startPage())) { covered = -1; return false; } if (covered < 0) covered = told.frames; return told.frames - covered >= 90; };
  let last = performance.now(), held = !kitPicture && q.has('paused');
  // THE PAGE'S PACE (host/pace.ts): a frame is not begun while two are still with the GPU, and a fast display's frames between the ladder's are passed over. A tab
  // nobody sees is not drawn; the time it was away is not the game's
  const pace = pacer(), rate = rateMeter(), rendered = [...RENDER_PASSES]; let resolved = -1, gpuMs = NaN;
  pictureKeepsPace(q);
  /** is somebody at this window? A kept page is asked of the tab's own (a top that refused once is not asked again) */
  let topAsks = true; const attended = (): boolean => { const t = (window as unknown as { __testFocus?: unknown }).__testFocus; if (typeof t === 'boolean') return t; if (topAsks) try { return (window.top ?? window).document.hasFocus(); } catch { topAsks = false; } return true; };
  let gave = 0, stamp = -1;
  /** what was not played is not the game's time, and no gap of the display's */
  const afresh = (): void => { last = performance.now(); stamp = -1; pace.reset(); rate.reset(); };
  document.addEventListener('visibilitychange', afresh);
  /** a game lost to a panic stands again, and nothing of the lost one's last frame goes with it */
  let backAt = -1e9;
  const standAgain = (newest: boolean): string | null => {
    const label = game.recover(newest); if (label === null) return null;
    input.keys.clear(); input.buttons = 0; input.endFrame(); acc = 0; drift = 0;
    cam.kickOut(); cam.drawOut(); if (pause.up) pause.close(); paused = false; thaw = false; input.lockExcept = null; stood = 0; anew(); lens(); cam.rebuild();
    backAt = newest ? game.ticks : -1e9; afresh(); requestAnimationFrame(loop);
    return label;
  };
  const loop = (ts: number): void => {
    try {
      const room = pageRoom({ calibrating: quality.calibrating, measuring: measure.active }), away = EVENPACE.giveway && !kit && !quality.calibrating && !measure.active && !measure.rec.recording && !attended();
      if (kit?.rests || held || document.hidden) { afresh(); requestAnimationFrame(loop); return; }
      if (pace.skip(ts, kit ? 60 : !quality.calibrating && !measure.active && (quality.adaptive || quality.auto) ? quality.bands.target : 0, away, room)) { requestAnimationFrame(loop); return; }
      const now = performance.now(), raw = (now - last) / 1000; last = now; told.fps = rate.push(now);
      // (a tick is owed by the display's own frames, which are steadier than a clock read in one; a window that gave way is no measure of the machine)
      beat = stamp >= 0 && ts > stamp ? Math.min(1 / 20, (ts - stamp) / 1000) : -1; stamp = ts; const rested = pace.gave !== gave; gave = pace.gave;
      if (quality.early) quality.remeasure();
      if (quality.calibrating) { if (measure.calibrates(engine)) quality.calibrateTick(raw); }
      else if (raw < 0.5 && !rested && !stopped() && !measure.active) { const fresh = engine.timer.resolved !== resolved; resolved = engine.timer.resolved; gpuMs = !engine.timer.enabled ? 0 : fresh ? engine.timer.unionMs(rendered) : NaN; quality.sample(raw, gpuMs); }
      measure.begins(); cloud?.begins(); reel.frame(raw); frame(Math.min(1 / 20, raw) * ((window as unknown as { __timeScale?: number }).__timeScale ?? 1), undefined, !unseen()); measure.timed(told.update, told.render); cloud?.drawn(raw, gpuMs, rested); gpuMs = NaN; measure.drawn(raw, told.fps);
      if (away || room > 0) { const w = pace.sent(performance.now()), end = (): void => pace.done(w, performance.now(), away && engine.timer.enabled ? engine.timer.unionMs(rendered) : 0); gpu.device.queue.onSubmittedWorkDone().then(end, end); }
      requestAnimationFrame(loop);
    } catch (e) {
      // (lost to a panic: the picture stands as it was, and the card says what Enter puts back)
      if (kit) { console.error(e); kit.none(); return; }
      if (e instanceof GameLost && play) { console.error(e); requestAnimationFrame(loop); return; }
      if (e instanceof GameLost) { kept0?.lost(); cloud?.cloud.fatal({ message: e.message.slice(0, 400), stack: e.words, stage: 'game', data: { what: e.what, tick: e.tick } }); lostCard(document.getElementById('app')!, e, { kept: () => game.kept(), back: standAgain }, e.tick - backAt < 120); return; }
      console.error(e); showFatal(`The frame loop threw and has been stopped:\n\n${(e as Error)?.stack ?? e}`); }
  };
  // ---- the pause card: the world holds still under it
  const runSaves: RunSaves = { elapsed: () => game.saves().elapsed, timeline: () => game.saves().rows, offer: () => { const s = game.saves(); return s.rows.find(r => r.key === s.offer) ?? null; }, age: row => agoWords(game.saves().elapsed - row.missionT), picture: row => pictures.get(ofRun(nameOf(row))) ?? null, load: key => { if (net) return 'You can\u2019t load a checkpoint in a co-op game.'; game.commands.load(key); owed = true; return null; }, due: () => asked };
  let asked = false;   // (a save by hand asked for on the card that is up)
  const kept0 = game.tape && keeps(q) ? new RunKeep({ sid, tape: () => game.tape ?? game.ended, on: () => !!game.tape, head: () => ({ level: { file: name, scenario: desk().brief(location.search).title || '' }, seed, ...(built ? { commit: built } : {}) }),
    over: () => !game.lost && (D.u[view() + View.run] & 2) !== 0, live: () => !title && !stopped() && !game.lost && game.view().live, began: () => (game.tape ?? game.ended)?.began ?? 0, say: words => readout.say(words), ended: () => { void cloud?.cloud.tapes?.round(); } }) : null;
  const inPanel = (): HTMLElement | null => { const a = document.activeElement; return (a instanceof HTMLInputElement || a instanceof HTMLSelectElement) && a.closest('#panel') ? a : null; };
  const pause = new PauseCard({ title: ((b: BriefView) => b.title ? `${b.title}${b.team ? ` · ${b.team}` : ''}` : name.replace(/\.json$/, ''))(desk().brief(location.search)), saves: runSaves, end: () => drawEnd(game, books, net?.door.client.names), lesson: () => nextLessonOf(game.run().mission), pick: () => { const p = nextPickOf(); return p && p.scenario === (game.run().mission !== '') ? p : null; }, may: () => !game.lost && !title && !play && !brief?.up && !measure.bench?.active, restart: () => { if (net) net.door.pass('You can\u2019t restart a co-op game from here.'); else { game.commands.restart('restarted'); owed = true; } }, replay: () => kept0?.whole ? () => kept0.watch() : null, quit: () => { reopenMissionsAfterLoad(); location.assign(location.pathname + desk().leave(net ? netLeave(location.search) : location.search) + location.hash); }, 
    // (Escape: Tab's card goes, or a field of the debugger's is let go, and that is the press's whole job)
    away: () => { const field = inPanel(); if (hud?.tabUp) hud.tabShow(false); else if (field && !pause.up) { field.dataset.esc = '1'; field.blur(); delete field.dataset.esc; } else return false; input.pressed.delete('Escape'); return true; },
    save: () => { if (optionsOpen() || missionsBusy() || !input.saveKey?.()) return false; if (net) { net.door.pass(NO_SAVE); return false; } game.commands.save(); return asked = game.saves().hand; },
    turned: up => { paused = up; if (up) { game.commands.letGo(); hud?.tabShow(false); } else { thaw = true; asked = false; } input.keys.clear(); input.buttons = 0; input.endFrame(); input.lockExcept = up ? new Set() : null; afresh(); } });
  onEnded = () => { if (!kit && !play && !ownPage(q)) pause.end(); };   // (a run's page has no end card: the next is its own)
  const cloud = kitPicture || play ? null : drawCloud(q, { sid, store: !unkept && keeps(q) ? keptStore() : null, gpu, engine, quality, game, measure, mission: desk().brief(location.search).title || (q.get('levelFile') ?? ''), offBooks: ownPage(q) || !!net, live: () => !title && !stopped() && !document.hidden && game.view().live, still: () => stopped() || measure.sweep.active });
  if (play && freeCam) { const mine = !!beside, { notes, other } = beside ?? { notes: [], other: false };
    const map = new TapeMap({ floor: () => game.floor(), now: () => game.map(), view: () => freeCam.view(), notes, to: (x, z) => freeCam.to([x, freeCam.view().look[1], z]), at: () => played.at });
    const sees: Sees = { get fog() { return watched.fog; }, get tube() { return watched.tube; }, get fogged() { return D.u[D.at[Section.fog] + Fog.on] !== 0; }, get runs() { return D.f[view() + View.night] > 0.5; }, get lit() { return watched.tube ?? this.runs; }, set: seesAs };
    bar = new TapeBar({ played, sees, map, other, cam: freeCam, canvas, title: desk().brief(location.search).title || name.replace(/\.json$/, ''), notes, keep: mine ? async all => { const ok = await keptStore().patch(named, { notes: all }); if (ok) void cloudNotes(q, keptStore()).catch(() => { /* on the next round */ }); return ok; } : null,
      back: () => backToGame(named, location.pathname), begun: () => { audio.start(); input.focusGame(); } });
    laneTape = { ticks: played.total, key: named, label: named.split('/').pop() ?? named, get f() { return played.at; }, get speed() { return played.pace; }, get seeking() { return played.seeking; }, get playing() { return !played.paused; }, play: () => { if (played.paused) bar!.toggle(); }, pause: () => { played.paused = true; }, seek: f => played.seek(f), drawn: null };
    soundLanes(laneTape, audio); }
  const bencher = drawBench({ idle: capMs => cloud ? cloud.cloud.filesIdle(capMs) : Promise.resolve(true), gpu, engine, smoke, quality, input, game, settings, measure, say: words => readout.say(words) }); bencher.go(); if (refused) readout.say(`benchmark not now: ${refused}`);
  // ---- the title stays up over its floor, live; a mission's run begins when the player starts it
  const drawTitle = (): void => { const root = document.getElementById('menu')!, c = document.createElement('div'); c.className = 'card'; root.textContent = ''; root.classList.add('on', 'title'); root.appendChild(c); titleScreen(c, { note: cloudNote, shows: { bench: () => { bencher.begin(); return true; } }, cheats: () => cheatWords(settings), closed: moved => { if (title && moved) drawTitle(); } }); };
  if (net) { net.ready(); coopInvite(net.door, q); }
  if (title) { drawTitle(); if (missionsReopenAsked() && !startPage()) openStartPage(); }
  else if (brief) startWaits(brief, { drawn: () => told.frames, measuring: () => quality.calibrating, settling: on => { settling = on; }, settled: () => { boot.mark('settled'); }, run: () => { input.keys.clear(); input.buttons = 0; input.endFrame(); thaw = true; audio.start(); input.focusGame(); afresh(); } });
  else card.off();
  loading.style.display = 'none'; debuggerPanel({ game, engine, smoke, cam, quality, input, chars: ch, marks: decals, over: hud, net, card: e => !pause.up && !!hud?.overlay.cardUp && hud.overlay.cardKey(e), hud: () => ({ fps: told.fps, union: engine.timer.unionMs(rendered) }) });
  // (for a tool: the page's parts, and frames stepped by hand)
  const steps = (n: number, keys: string[] = [], drawn: 'none' | 'last' | 'all' = 'last'): void => { if (title || brief?.up) { title = false; reel.poke(); brief?.off(); card.off(); } for (const k of keys) input.keys.add(k); try { for (let i = 0; i < n; i++) frame(1 / 60, 1, drawn === 'all' || (drawn === 'last' && i === n - 1)); } finally { for (const k of keys) input.keys.delete(k); } };
  Object.assign(window, { __draw: { game, hud, played: play ? played : null, seesAs, get watched() { return watched; }, books, kit, measure, reel, pics, engine, world, audio, pause, cam, input, quality, told, fog, weather, smoke, get held() { return held; }, set held(v: boolean) { held = v; afresh(); }, run: steps } });
  // (…and by the names every page of the game has them under)
  const G = window as unknown as Record<string, unknown>; G.__engine = engine; G.__prewarmed = true; G.__timeScale = 1;
  Object.defineProperty(window, '__pauseLoop', { configurable: true, get: () => held, set: (v: boolean) => { held = v; afresh(); } });
  G.__runFrames = (n: number, _dt = 1 / 60, keys: string[] = [], draw: boolean | 'none' = true): void => steps(n, keys, draw === 'none' ? 'none' : draw ? 'all' : 'last');
  G.__game = { get time() { return game.time; }, get mission() { const t = game.tally(); return { stage: t.stage, alarmOn: t.alarm, stats: { alerts: t.alerts } }; } };
  G.__pace = () => ({ on: EVENPACE.on, hz: pace.hz, every: pace.every, skipped: pace.skipped, worked: pace.worked, target: quality.adaptive || quality.auto ? quality.bands.target : 0, fps: told.fps, flight: EVENPACE.flight, flying: pace.flying, held: pace.held, lost: pace.lost });
  boot.mark('interactive'); gpuStory.stage = ''; boot.span('toInteractive', 0); boot.stopRafProbe();
  void laterIdle().then(() => { if (PIPE_QUEUE.later) boot.mark('laterReady'); }); setTimeout(() => console.log(boot.line()), 5000);
  requestAnimationFrame(loop);
}
tapConsole();   // (before the page boots: its own complaints are held until the backend's side is up, and nothing on a build with none)
tellRedCards(() => kitPicture);
const uncaught = (head: string, e: unknown): void => { if (document.getElementById('fatal')?.style.display !== 'flex') showFatal(`${head}\n\n${(e as Error)?.stack ?? e}`); };
window.addEventListener('error', ev => { if (!/ResizeObserver loop/.test(ev.message) && !(ev.error instanceof GameLost)) uncaught('Uncaught error:', ev.error ?? ev.message); });
window.addEventListener('unhandledrejection', ev => { if (!(ev.reason instanceof GameLost) && !(ev.reason instanceof TapeRefused)) uncaught('Unhandled promise rejection:', ev.reason); });
// (a tool waits for the programs made behind the start too: until then a pass draws its stand-in, and the fog of war is the lit one)
const ready = main(); (window as unknown as { __ready: Promise<void> }).__ready = ready.then(() => laterIdle());
ready.catch(e => { if (e instanceof TapeRefused) { refusedCard(e.message); return; } console.error(e); if (kitPicture) kitNone(); if ((e as Error)?.name === 'RustGameError') showFatal((e as Error).message, { report: { message: (e as Error).message.slice(0, 400), stack: (e as Error).stack, stage: 'rust game' } }); else showFatalError(e); });
