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
import { AUTOVIEW, CARRY_KEYS, FINDME, FINDTUNED, aimCarry, behindStep, behindYaw, bindFindMe, carryStep, ctrlTurn, findMeStarts, setAutoView, watchCtrlTaps, type AimCarry } from './findme';
import { AUTOCAM, AUTOCAM_STATS, autoFrame, autoHand, autoSway, autoTuneApply, autoZoom, frameBegin, frameRest, frameState, swayCoast, swayDrop, turnCoast, turnEase, turnGoal, walkKey } from './autoframe';
import { fetchFace } from './face';
import { machineAfter, machineBefore } from './machine';
import { EVENPACE, FLIGHT_PAGE, pacer, pageRoom, rateMeter } from './pace';
import { PLAN_PACE } from '../render/planPace';
import { RENDER_PASSES } from '../gpu/timer';
import { DrawGame, GameLost } from './drawgame';
import { knobPanel } from './knobpanel';
import { lostCard } from './lostcard';
import { PauseCard } from '../ui/pausecard';
import { agoWords, type RunSaves } from '../ui/runcard';
import { camKey, keysBoot } from './keymap';
import { Section, Sky, View, Want } from '../draw/sections';
import { shotFrom, shotScene } from '../draw/shot';
import { AudioEngine } from '../audio/audio';
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
import type { Vec3 } from '../math/vec';

const LOW_PITCH = 24 * Math.PI / 180, HIGH_PITCH = 55 * Math.PI / 180;
const bytesOf = async (url: string): Promise<Uint8Array> => { const r = await fetch(url, { cache: 'no-store' }); if (!r.ok) throw new Error(`${url}: HTTP ${r.status}`); return new Uint8Array(await r.arrayBuffer()); };
/** the level's file in the look that is on, or the file itself where it has none */
async function levelBytes(name: string): Promise<Uint8Array> {
  const look = lookLevelFile(name);
  try { return await bytesOf(`levels/${look}`); } catch (e) { if (look === name) throw e; return bytesOf(`levels/${name}`); }
}

async function main(): Promise<void> {
  const q = new URLSearchParams(location.search), loading = document.getElementById('loading')!, say = (line: string) => { loading.textContent = line; };
  const canvas = document.getElementById('view') as HTMLCanvasElement;
  // (of the page's markup, what this page drives: the rest of the HUD comes as it is told of the game)
  for (const el of document.getElementById('app')!.children) if (!['view', 'loading', 'menu', 'fatal'].includes(el.id)) (el as HTMLElement).style.display = 'none';
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
  game.drawn.want = Want.all & ~Want.shot;   // (the mode, if it is on, asks from its first frame)
  machineBefore(q, name, game.commands);
  game.start({ level, words: location.search, mannequin, mannequinBin, cast, castLook, seed: Number.isFinite(seed) ? seed >>> 0 : crypto.getRandomValues(new Uint32Array(1))[0] });
  machineAfter(game.commands);
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
  engine.setDecals(new DrawDecals(engine.device, D)); trafficFrom(engine.gbuffer, D);
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
  const fog = makeFogFrame(), ask = askOf(game), audio = new AudioEngine(ask), events = new DrawEvents(D, engine.particles, smoke, audio, i => game.nameOf(i));
  if (q.has('mute')) { audio.volume.master = 0; audio.start = () => { /* silent */ }; }
  for (const ev of ['pointerdown', 'keydown'] as const) window.addEventListener(ev, () => audio.start(), { capture: true });   // (a browser's sound begins with a hand on the page)
  engine.fogFrame = fog; const rims = engine.outline = makeOutlineFrame(); engine.beams = world.beams;
  const lights = new DrawLights(engine.lights, D); lights.from();   // (the lights stand too: their reach is worked out of the level as it opens)
  const reach = new DrawReach(engine, world, gaps);
  const weather = new WeatherPicture(engine, D, world, engine.particles, smoke);

  // ---- the player
  const input = new Input(canvas); input.saveKey = () => true;   // (F5 is the game's: a save by hand)
  const cam = new FollowCamera(), follow: Vec3 = [0, 0, 0], aim: Vec3 = [0, 0, 0];
  FINDTUNED.on = true; findMeStarts(); bindFindMe(cam);   // (how the view feels: the player's own rows, over the tuned starts)
  AUTOCAM.tune = true; autoTuneApply();   // (the camera that runs itself: the player's own mode and rows)
  const auto = frameState(), scene = shotScene();
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
  /** the game is another than the one last drawn (a restart, a load): what the picture and the sound carry over from frame to frame starts over, before anything of the new one is played */
  const anew = (): void => {
    const o = view(), first = stood < 0; stood = D.u[o + View.stood]; if (first) return;
    engine.restartPicture(); engine.particles?.clear(); smoke.clearAll(); events.cut(); weather.cut(); audio.hush(); game.turnAgain(); pair.cut(); lag.reset(); three(cam.target, o + View.follow);
  };
  anew(); lens(); cam.aspect = Math.max(1, canvas.clientWidth) / Math.max(1, canvas.clientHeight); cam.rebuild();

  const told = { update: 0, render: 0, tick: 0, draw: 0, frames: 0, fps: 0 };
  const ear: Vec3 = [0, 0, 0];
  let time = 0, acc = 0, drift = 0, paused = false, thaw = false, ended = 0, onEnded = (): void => { /* no card is made yet */ }, behind: number | null = null, behindAuto = false, carry: AimCarry | null = null; const feet: Vec3 = [0, 0, 0], twice = watchCtrlTaps(window, () => false, camKey);
  const viewFree = (): boolean => { const o = view(); return D.u[o + View.spectating] === 0 && D.u[o + View.showing] === 0 && !cam.pin; };
  /** his aim goes round with a view that comes round quickly: the point he faces now */
  const carryOn = (): void => { const o = view(), was = carry; carry = aimCarry(D.f[o + View.aimYaw], three(feet, o + View.at), three(aim, o + View.aim), was ? was.realX : input.mouseX, was ? was.realY : input.mouseY); if (was) { carry.putX = was.putX; carry.putY = was.putY; } };
  /** the view coming round, a step with each tick: the double tap's, or a turn of the auto camera's own (its speed dying away where it was ended) */
  const comeRound = (dt: number): void => {
    if (behind !== null && !viewFree()) behind = null;
    if (behind === null) { if (auto.turnV) { const t = AUTOVIEW.on && viewFree() ? turnCoast(auto, dt) : (auto.turnV = 0); if (t) cam.rotate(t); } return; }
    const cap = behindAuto ? AUTOCAM.turnCap : AUTOVIEW.on ? AUTOCAM.swingCap : 0;
    if (cap > 0) { const s = turnEase(auto, cam.yaw, behind, dt, cap); cam.rotate(s.turn); if (s.done) behind = null; return; }
    auto.turnV = 0;
    const s = behindStep(cam.yaw, behind, dt, behindAuto ? AUTOCAM.turnRate : undefined); cam.rotate(s.turn); if (s.done) behind = null;
  };
  /** the point a cursor there aims at, for a view that comes round a long way (host/findme.ts carryStep) */
  const aimOf = (sx: number, sy: number): Vec3 | null => { const u = sx / Math.max(1, canvas.clientWidth), v = sy / Math.max(1, canvas.clientHeight), r = cam.ray(u, v), hit = ask.ray(r.ro, r.rd, 200); return hit ? [r.ro[0] + r.rd[0] * hit.t, r.ro[1] + r.rd[1] * hit.t, r.ro[2] + r.rd[2] * hit.t] : cam.groundPoint(u, v, 0); };
  /** his aim carried round: the game's cursor put where the point he faced stands now, or the moment the hand uses the mouse or a key that may read it, back where the mouse really is */
  const carryAim = (): void => {
    if (carry) { let used = paused || !!input.clicked || !viewFree() || !(FINDME.behind || AUTOVIEW.on); for (const k of input.pressed) if (!CARRY_KEYS.has(k)) used = true; const s = carryStep(carry, input.mouseX, input.mouseY, used, three(feet, view() + View.at), (x, y, z) => cam.project([x, y, z], canvas.clientWidth, canvas.clientHeight, Infinity, 1e-3), AUTOVIEW.on ? aimOf : undefined); input.mouseX = s.x; input.mouseY = s.y; if (!s.held) { carry = null; if (behindAuto && !(AUTOCAM.turnCap > 0)) behind = null; } }
  };
  const tick = (dt: number): void => {
    comeRound(dt);
    carryAim();
    if (thaw) { thaw = false; input.pressed.add('Paused'); input.pressed.add('Thawed'); }   // (the first tick after a card is told: what was held when it came up is let go unsent, in the tick)
    const keys = sampleHand(game.numbersOf(), input, cam, canvas);
    game.step(dt, keys);
    if (D.u[view() + View.stood] !== stood) anew();
    weather.told();
    events.from();
    D.fresh();   // (a sound asks the module, whose memory may have grown)
    lens();
    const o = view(), spectating = D.u[o + View.spectating] !== 0;
    three(follow, o + View.follow);
    // come round a man spectated to a side he can be seen from, low where the view is close; else the usual tilt
    if (spectating) cam.autoYaw(dt, follow, ask.blocked);
    cam.easeYaw(dt, spectating);
    if (!cam.pin) cam.pitch += ((spectating && cam.desiredDistance < 12 ? LOW_PITCH : HIGH_PITCH) - cam.pitch) * Math.min(1, dt * 3);
    // THE CAMERA RUNNING ITSELF (host/autoframe.ts): with the mode on and the view his own, the look-at is the frame's (his feet, shifted toward what the game knows he is about), nothing leans on the cursor, and a turn of its own may begin
    if (AUTOVIEW.on && shotFrom(scene, D)) {
      if (!(auto.base > 0)) frameBegin(auto, cam);   // (kept on from an earlier visit: the zoom as the page opened is the hand's own)
      const look = autoFrame(auto, scene, cam, dt), yaw = turnGoal(auto, scene, cam, dt, ask.blocked);
      if (behindAuto && behind !== null && (scene.shown || scene.hold || !scene.free)) behind = null;   // (a turn of its own under way ends where it is the moment it could not have begun)
      if (Math.abs(auto.turnV) > AUTOCAM.turnSeen) auto.turnT = 0;   // (`gap` is counted from where an eased turn is seen to stop)
      else if (yaw != null && behind === null && !auto.sway && !auto.swayV) { behind = yaw; behindAuto = true; auto.turnT = 0; AUTOCAM_STATS.turns++; if (!(AUTOCAM.turnCap > 0)) carryOn(); }   // (his aim goes round with a quick turn; an eased one is slow enough for the hand to follow)
      if (behind !== null) swayDrop(auto); else { const t = autoSway(auto, scene, cam, dt, ask.blocked); if (t) { cam.rotate(t); AUTOCAM_STATS.swayed += Math.abs(t); } }
      cam.step(dt, look, null);
    } else {
      if (auto.turnV) { auto.turnV = 0; behind = null; }   // (an eased turn under way as the view goes to somebody else: it ends there)
      frameRest(auto, cam); if (auto.swayV) { const t = AUTOVIEW.on ? swayCoast(auto, dt) : 0; if (t) cam.rotate(t); }   // (a lean under way dies away, and is where it was when the view is his again)
      cam.step(dt, follow, spectating ? null : three(aim, o + View.aim));
    }
    pair.capture(cam);
    weather.tick(dt, cam);
    told.tick = game.ms.step; told.draw = game.ms.draw;
  };
  /** one drawn frame: `dt` seconds of the display's, in whole ticks (`drawn` false: a script's, stepped and not drawn) */
  const frame = (dt: number, ticks?: number, drawn = true): void => {
    if (paused) dt = 0;   // (the world holds still under a card: what only the picture moves with it)
    time += dt;
    const t0 = performance.now(), plan = ticks !== undefined ? { ticks, acc, drift } : paused ? { ticks: 0, acc, drift } : planTicks(acc, dt, FIXEDSTEP, drift); acc = plan.acc; drift = plan.drift;
    const tapped = twice();   // (asked every frame: a tap under a card is not kept for the moment it lifts)
    const locks = D.u[view() + View.locks];
    if (input.wheelCtrl) { if (!(locks & 2)) { cam.zoom(input.wheelCtrl); if (AUTOVIEW.on) autoZoom(auto, cam, input.wheelCtrl); } input.wheelCtrl = 0; }
    // THE AUTO CAMERA'S MODE: the double tap puts it on and brings the view round, whatever the Options row says, so the hand sees it took (in the mode, a double tap is that turn again); the hand on the camera puts it off
    if (!AUTOVIEW.on) { if (auto.base > 0 && !(locks & 2)) cam.desiredDistance = auto.base; auto.base = NaN; auto.turnV = 0; if (auto.sway || auto.swayV) swayDrop(auto); }   // (off: the zoom goes back to the hand's own, and that is what it comes on with next)
    if (!paused) {
      const camDown = !!input.camHeld || (!camKey() && (input.down('ControlLeft') || input.down('ControlRight')));
      if (autoHand(auto, input, locks & 1 || !plan.ticks ? 0 : input.orbitDX, camDown, dt) && AUTOVIEW.on) { setAutoView(false); frameRest(auto); AUTOCAM_STATS.off++; if (behindAuto || auto.turnV) behind = null; }
      if (behind !== null && (behindAuto ? walkKey(input) || input.clicked : !!auto.turnV && input.clicked)) behind = null;   // (a walk key or a button ends a turn of its own where it is; a shot ends a slow swing)
    }
    if (behind === null) behindAuto = false;
    const orbit = !paused && input.orbitDX && !(locks & 1) ? ctrlTurn(input.orbitDX) : 0, autoTap = !(locks & 1) && D.u[view() + View.own] !== 0;   // (where the mode would frame nothing, the tap is not its)
    if (paused) carryAim();   // (no tick will ask: a card up gives the mouse back at once)
    if (paused || !viewFree()) behind = null;
    else if (tapped && (FINDME.behind || autoTap)) { const put = autoTap && !AUTOVIEW.on; if (put) { setAutoView(true); frameBegin(auto, cam); } if (!put || AUTOCAM.swing) { behindAuto = false; behind = behindYaw(D.f[view() + View.aimYaw]); carryOn(); } }   // (the swing off: the tap that puts the mode on turns nothing)
    else if (orbit) behind = null;
    if (orbit && plan.ticks) cam.rotate(orbit);   // (in a frame's first tick: the next tick's hand looks the way it left the view)
    D.want = AUTOVIEW.on ? Want.all : Want.all & ~Want.shot;   // (what there is to frame is read only for the mode)
    cam.frame();
    for (let i = 0; i < plan.ticks; i++) { if (i) input.endTick(); tick(1 / FIXEDSTEP.hz); }
    // (the run's end card, once each time one is due; a restart or a load is another end to come)
    { const e = D.u[view() + View.ended]; if (e !== ended) { ended = e; if (e) onEnded(); } }
    D.fresh();
    skyFrom(engine, D); fogFrom(fog, D); rimsFrom(rims, D, dt); weather.frame(cam); dustFrom(engine.particles!, D);
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
  /** a game lost to a panic stands again, and nothing of the lost one's last frame goes with it */
  let backAt = -1e9;
  const standAgain = (newest: boolean): string | null => {
    const label = game.recover(newest); if (label === null) return null;
    input.keys.clear(); input.buttons = 0; input.endFrame(); acc = 0; drift = 0; behind = carry = null;
    cam.kickOut(); cam.drawOut(); if (pause.up) pause.close(); paused = false; thaw = false; input.lockExcept = null; stood = 0; anew(); lens(); cam.rebuild();
    backAt = newest ? game.ticks : -1e9; last = performance.now(); requestAnimationFrame(loop);
    return label;
  };
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
    } catch (e) {
      // (lost to a panic: the picture stands as it was, and the card says what Enter puts back)
      if (e instanceof GameLost) { lostCard(document.getElementById('app')!, e, { kept: () => game.kept(), back: standAgain }, e.tick - backAt < 120); return; }
      console.error(e); showFatal(`The frame loop threw and has been stopped:\n\n${(e as Error)?.stack ?? e}`); }
  };
  // ---- the pause card: the world holds still under it
  const runSaves: RunSaves = { elapsed: () => game.saves().elapsed, timeline: () => game.saves().rows, offer: () => { const s = game.saves(); return s.rows.find(r => r.key === s.offer) ?? null; }, age: row => agoWords(game.saves().elapsed - row.missionT), picture: () => null, load: key => { game.commands.load(key); return null; }, due: () => false };
  const pause = new PauseCard({ title: name.replace(/\.json$/, ''), saves: runSaves, verdict: () => game.verdict(), may: () => !game.lost, restart: () => game.commands.restart('restarted'), quit: () => location.assign('./'), turned: up => { paused = up; if (up) game.commands.letGo(); else thaw = true; input.keys.clear(); input.buttons = 0; input.endFrame(); input.lockExcept = up ? new Set() : null; last = performance.now(); } });
  onEnded = () => pause.end();
  loading.style.display = 'none'; knobPanel(document.getElementById('app')!, game);
  // (for a tool: the page's parts, and frames stepped by hand)
  Object.assign(window, { __draw: { game, engine, world, audio, pause, cam, input, quality, told, fog, weather, smoke, get held() { return held; }, set held(v: boolean) { held = v; last = performance.now(); }, run: (n: number, keys: string[] = [], drawn: 'none' | 'last' | 'all' = 'last') => { for (const k of keys) input.keys.add(k); try { for (let i = 0; i < n; i++) frame(1 / 60, 1, drawn === 'all' || (drawn === 'last' && i === n - 1)); } finally { for (const k of keys) input.keys.delete(k); } } } });
  requestAnimationFrame(loop);
}
const ready = main(); (window as unknown as { __ready: Promise<void> }).__ready = ready;
ready.catch(e => { console.error(e); if ((e as Error)?.name === 'RustGameError') showFatal((e as Error).message); else showFatalError(e); });
