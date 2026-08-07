// Character / prop viewer (viewer.html). Boots the game's Engine on a procedural stage (scene/stage.ts) with ONE mannequin, an orbit
// camera and a panel that (a) drives the animation graph or plays any library clip raw, (b) toggles the kit (goggles / rail light /
// slot / guard torch) and (c) live-edits every number in rigProps.RIG, with a "copy constants" button that emits the `export const RIG`
// statement to paste back into src/game/rigProps.ts. No Game / AI / audio: the props come from rigBoxes() and the rail light from
// rigLightPose() — the very functions game.ts calls — and the frame is fed to the Engine the way main.ts + Game.update do it
// (world statics, dynamic prop boxes, capsule shadow proxies, skinned instance, LightSet, SmokeSystem, FollowCamera).
import { initGpu, showFatal } from './gpu/device';
import { Engine } from './engine';
import { buildStage } from './scene/stage';
import { lightFromDef } from './render/lights';
import { FollowCamera } from './game/camera';
import { Input } from './game/input';
import { Vec3, v3, DEG, clamp, wrapAngle } from './math/vec';
import { StaticCollision } from './scene/collision';
import { loadGltfCharacter } from './anim/gltf';
import { CharacterRenderer } from './anim/characters';
import { Character, UpperMode, AnimParams, Stance, SignalKind, KICK_IMPACT } from './game/character';
import { SmokeSystem } from './smoke/smoke';
import { Panel } from './ui/panel';
import { Quality, QUALITY_NAMES, QualityName } from './ui/quality';
import { Box, BoxFlag, makeBox } from './scene/boxes';
import { RIG, RIG_DEFAULTS, Rig, assignRig, clearAdjs, rigBoxes, holsterBoxes, rigLightPose, rigTorchPos, rigSource, torchCarryDir, railLightDef, torchLightDef, RigNodeMeta } from './game/rigProps';
import { RigGizmo } from './viewerGizmo';
import * as fxm from './game/effects';
import { Sparks } from './game/sparks';
import { Throwables } from './game/throwables';
import { PropSystem } from './game/props';
import { frameCanvas, postShot } from './ui/capture';

// the game's character tints (game.ts Player / Guard) so the props read against the right suit
const PLAYER_TINT = { tint: [0.05, 0.06, 0.07] as Vec3, tint2: [0.1, 0.32, 0.18] as Vec3 };
const GUARD_TINT = { tint: [0.42, 0.36, 0.25] as Vec3, tint2: [0.02, 0.02, 0.025] as Vec3 };
const CHAR_ID = 1;   // = PLAYER_ID: the rail light / torch are owned by the character so its own capsules never shadow them

/** locomotion states of the graph = (ground speed, crouch, reverse); speeds are the animator's natural clip speeds so ×1 plays each cycle at 1:1 */
const DRIVES: { name: string; speed: (p: AnimParams) => number; crouch: number; reverse: boolean }[] = [
  { name: 'idle', speed: () => 0, crouch: 0, reverse: false },
  { name: 'walk', speed: p => p.walkSpeed, crouch: 0, reverse: false },
  { name: 'jog', speed: p => p.jogSpeed, crouch: 0, reverse: false },
  { name: 'sprint', speed: p => p.sprintSpeed, crouch: 0, reverse: false },
  { name: 'crouch idle', speed: () => 0, crouch: 1, reverse: false },
  { name: 'crouch walk', speed: p => p.crouchSpeed, crouch: 1, reverse: false },
  { name: 'backpedal (walk)', speed: p => p.walkSpeed, crouch: 0, reverse: true },
  { name: 'backpedal (crouch)', speed: p => p.crouchSpeed, crouch: 1, reverse: true },
];
const UPPERS: { name: string; mode: UpperMode; autoFire: boolean }[] = [
  { name: 'none (arms follow legs)', mode: 'none', autoFire: false },
  { name: 'relaxed (pistol idle, two-hand)', mode: 'relaxed', autoFire: false },
  { name: 'aim (two-hand, pitch blend)', mode: 'aim', autoFire: false },
  { name: 'shoot (aim + fire every 0.6 s)', mode: 'aim', autoFire: true },
  { name: 'torch (Idle_Torch_Loop)', mode: 'torch', autoFire: false },
];
const TRACK = ['— (free focus)', 'root + 1 m', 'head', 'right hand', 'chest'];
/** the animator's procedural vocabulary (character.ts): stances are continuous, the rest are one-shots the 'loop' box can keep re-firing */
const STANCES: Stance[] = ['none', 'highReady', 'lowReady', 'stack'];
const TAC_SHOTS = ['kick door', 'signal: hold', 'signal: go', 'signal: rally', 'flinch'];
const DEBUG_VIEWS = ['final', 'albedo', 'normals', 'direct', 'indirect', 'volumetrics', 'depth', 'RC dice', 'lighting only', 'denoise hints'];   // engine.settings.debugView indices (same order as main.ts)

async function main() {
  const canvas = document.getElementById('view') as HTMLCanvasElement;
  const loading = document.getElementById('loading')!;
  const hud = document.getElementById('hud')!, help = document.getElementById('help')!, logEl = document.getElementById('log')!, rigout = document.getElementById('rigout')!;
  const gpu = await initGpu(canvas);
  const engine = new Engine(gpu);
  const stage = buildStage();
  for (const b of stage.boxes) engine.world.addStatic(b);
  const stageLights = stage.lights.map(ld => engine.lights.add(lightFromDef(ld)));
  engine.settings.skyZenith = stage.sky.zenith; engine.settings.skyHorizon = stage.sky.horizon;
  engine.settings.exposure = 1.4; engine.settings.hazeDensity = 0.006;   // a studio, not the office at night: little haze (the beam still reads), moderate exposure — otherwise a rail light pointed at the camera whites the stage out
  const wallBox = stage.boxes[stage.wall];   // same object the BoxWorld holds (addStatic keeps the reference), so cutHeight edits take effect on the next frame
  const stageCol = new StaticCollision(stage.boxes);   // what a dying mannequin's ragdoll collides with: the floor, the back wall, the cube and the post (the game hands it the office's)
  const stageItems = new Throwables(stageCol, new PropSystem([], stageCol, () => {}));   // the game's throwable / dropped-item physics + models on the stage (dropped pistol / torch, canister bodies)

  loading.textContent = 'loading character rig…';
  const ch = await loadGltfCharacter(new URL('assets/ual/AnimationLibrary_Godot_Standard.gltf', document.baseURI).href);
  const smoke = new SmokeSystem(engine); engine.setSmoke(smoke.binding); engine.afterSubmit.push(() => smoke.afterSubmit());   // same attachments as main.ts (gun smoke on 'shoot')
  const chars = new CharacterRenderer(engine.device, ch); engine.skin = chars;
  const clipNames = [...ch.clips.keys()].sort((a, b) => a.localeCompare(b));
  loading.style.display = 'none';

  // the two hand lights from the same definitions game.ts uses (its tune defaults: playerLight 60 · flashIntensity 70, inner 7°, outer 19°)
  const railLight = engine.lights.add(railLightDef(CHAR_ID, 60));
  const torchLight = engine.lights.add(torchLightDef('flashlight0', CHAR_ID, 70, 7, 19)); torchLight.enabled = false;

  const st = {
    drive: 0, speedMul: 1, upper: 1, aimPitchDeg: 0, aimYawDeg: 0, lookYawDeg: 0,
    emitterGizmos: true,   // shot emitter anchors (bore jet / side ports / muzzle wisp) as gizmo nodes on the muzzle frame
    clip: 0, rate: 1, paused: false, path: 0,          // clip 0 = the game's graph, k>0 = clipNames[k-1] raw · path 0 treadmill, 1 walk a circle
    turntable: false, turnRate: 30, bodyYawDeg: 0,
    goggles: true, nv: false, lightOn: false, slot: 1, guard: false, guardTorch: true, nvScreen: false, showProps: true, gunSmoke: true,   // light off by default: facing the default camera it would shine straight into the lens (L toggles)
    track: 1, wall: true,
    stance: 0, stackSide: 1 as -1 | 1, lockpick: false, tacShot: 0, tacLoop: false, tacEvery: 1.5,   // Tactical (procedural) section
  };
  let tacT = 0; let kickFlashT = 0;
  let autoFireT = 0; let beamDir: Vec3 = [0, -0.3, 1]; let circPhi = Math.PI / 2;   // circle path parameter (starts due 'south' of the centre, nearest the default camera)
  const CIRCLE_R = 2;
  const fx: { box: Box; ttl: number }[] = []; const dyn: Box[] = [];
  const sparks = new Sparks(fxm.sparks); let fxSeed = 1;   // the game's powder-spark particles (emissive streak boxes)
  let canTick: (dt: number) => void = () => {};              // canister loop (set up by the Canisters section)
  const messages: { text: string; t: number }[] = []; const msg = (text: string) => { messages.push({ text, t: performance.now() / 1000 }); if (messages.length > 4) messages.shift(); };

  let char!: Character;
  const spawn = () => { const keepYaw = char ? char.bodyYaw : st.bodyYawDeg * DEG; char = new Character(ch, { id: CHAR_ID, tint: v3.copy(PLAYER_TINT.tint), tint2: v3.copy(PLAYER_TINT.tint2), pos: v3.copy(stage.center), yaw: keepYaw }); char.aimYaw = keepYaw; char.update(0); };
  spawn();

  // ---------------------------------------------------------------- camera: the game's FollowCamera driven as an orbit camera
  const cam = new FollowCamera();
  cam.fov = 40 * DEG; cam.near = 0.1; cam.far = 80; cam.aimLead = 0;
  let focus: Vec3 = v3.add(stage.center, [0, 1, 0]);
  const frameShot = (kind: 'body' | 'head' | 'hand' | 'feet' | 'back') => {
    if (kind === 'body') { st.track = 1; cam.yaw = 0; cam.pitch = 12 * DEG; cam.desiredDistance = 2.8; }
    if (kind === 'back') { st.track = 1; cam.yaw = Math.PI; cam.pitch = 14 * DEG; cam.desiredDistance = 2.8; }
    if (kind === 'head') { st.track = 2; cam.pitch = 8 * DEG; cam.desiredDistance = 0.9; }
    if (kind === 'hand') { st.track = 3; cam.pitch = 25 * DEG; cam.desiredDistance = 0.9; }
    if (kind === 'feet') { st.track = 0; focus = v3.add(char.pos, [0, 0.25, 0]); cam.pitch = 20 * DEG; cam.desiredDistance = 1.8; }
  };
  frameShot('body'); cam.distance = cam.desiredDistance; cam.target = v3.copy(focus); cam.desired = v3.copy(focus); cam.rebuild();

  const input = new Input(canvas);
  canvas.addEventListener('mousedown', () => { const ae = document.activeElement; if (ae instanceof HTMLElement && ae.closest('#panel')) ae.blur(); input.focusGame(); });   // a slider that had focus lets go of the keyboard; focus goes to the game (its sink when framed)   // Input ignores keys while a slider/select has focus (and preventDefaults the click that would blur it): a click in the view hands the hotkeys back
  let lastMX = input.mouseX, lastMY = input.mouseY;

  // ---------------------------------------------------------------- one-shots
  const shoot = () => {
    if (!char.anim.rawClip) char.anim.fire();   // (the raw-clip bypass never advances the shot layer, so don't arm it there — the flash still goes off at the muzzle)
    const muzzle = v3.copy(char.muzzle), dir = v3.copy(char.gunDir);
    // cosmetic copy of the player branch of Game.fireWeapon (k = 0.45, suppressed): hot core + afterglow lights, a 1-frame flame card, the
    // gas recipe. Not a tunable — if fireWeapon's numbers move, this preview is allowed to lag.
    // the game's own graded shot (src/game/effects.ts spawnShot + Sparks): bore + port gas, wisps, powder sparks, the warm flash lights —
    // driven by the same numbers the Effects section below edits, so what you tune here IS what the game fires
    const track = () => ({ pos: char.muzzle, dir: char.gunDir });
    const port = () => ({ pos: v3.add(v3.mad(char.bones.handR ?? char.muzzle, char.gunDir, 0.06), [0, 0.06, 0]), dir: [0, 1, 0] as Vec3 });
    fx.push({ box: makeBox({ c: v3.mad(muzzle, dir, 0.05), h: [0.018, 0.018, 0.045], yaw: v3.yawOf(dir), albedo: [1, 0.8, 0.5], emissive: [40, 22, 8], flags: BoxFlag.Dynamic | BoxFlag.NoShadow }), ttl: 0.035 });   // 1-frame flame card (fireWeapon's)
    for (const L of fxm.spawnShot(st.gunSmoke ? smoke : { emit: () => {} } as unknown as typeof smoke, sparks, muzzle, dir, { seed: fxSeed++, lightGain: st.guard ? 1 : fxm.shot.suppressedGain, track, eject: port }))
      engine.lights.flash(L.pos, L.color, L.intensity, L.range, L.ttl, CHAR_ID);
  };

  // ---------------------------------------------------------------- rig constants: emit / copy / reset
  const showSource = () => { rigout.textContent = rigSource(); rigout.classList.remove('hidden'); };
  const copyConstants = async () => {
    const src = rigSource(); showSource();
    try { await navigator.clipboard.writeText(src); msg(`copied ${src.split('\n').length} lines — paste over the RIG block in src/game/rigProps.ts`); }
    catch { const sel = window.getSelection(); if (sel) { const r = document.createRange(); r.selectNodeContents(rigout); sel.removeAllRanges(); sel.addRange(r); } msg('clipboard blocked by the browser — the text is selected in the box, ⌘/Ctrl+C it'); }
  };
  const resetConstants = () => { clearAdjs(RIG); assignRig(RIG, RIG_DEFAULTS); if (!rigout.classList.contains('hidden')) showSource(); msg('RIG reset to the shipped numbers'); refreshSel(); };

  // ---------------------------------------------------------------- panel
  const quality = new Quality(engine, smoke);
  { const qp = new URLSearchParams(location.search).get('quality'); if (qp && (QUALITY_NAMES as string[]).includes(qp)) { quality.auto = false; quality.apply(qp as QualityName); } }
  const panel = new Panel(); panel.toggle();   // open by default: it IS the tool
  // ---- selection: hierarchy list + Blender-style handles (viewerGizmo) editing the selected node's free Adj in RIG
  const gizmo = new RigGizmo(canvas, cam, input); const meta: RigNodeMeta[] = [];
  // emitter gizmo nodes: the Adj's `off` is the live fxm.shot vector itself, so dragging an arrow edits the effect directly (rot is accepted and ignored)
  const emitterAdjs = new Map<string, { off: Vec3; rot: Vec3 }>(); const emitterRootAdj = { off: [0, 0, 0] as Vec3, rot: [0, 0, 0] as Vec3 };
  const emitterAdj = (key: string, off: Vec3) => { let a = emitterAdjs.get(key); if (!a || a.off !== off) { a = { off, rot: [0, 0, 0] }; emitterAdjs.set(key, a); } return a; };
  const hier = panel.section('HIERARCHY · click a prop or a row', true);
  const tree = document.createElement('div'); tree.style.cssText = 'margin:2px 0 4px'; hier.appendChild(tree);
  const selBox = document.createElement('div'); hier.appendChild(selBox);
  let treeKey = '';
  const rebuildTree = () => {   // rows for the nodes drawn this frame (changes with kit / slot / NV); cheap, keyed so it only rebuilds when the set changes
    const key = meta.map(m => m.path).join('|') + '#' + (gizmo.selected ?? '');
    if (key === treeKey) return; treeKey = key; tree.innerHTML = '';
    for (const m of meta) {
      const row = document.createElement('div');
      row.textContent = m.label; row.title = m.path;
      row.style.cssText = `padding:1px 4px 1px ${4 + (m.depth - 1) * 14}px;cursor:pointer;border-radius:3px;color:${m.path === gizmo.selected ? '#111' : '#cfe'};background:${m.path === gizmo.selected ? '#f0d878' : 'transparent'}`;
      row.onclick = () => gizmo.select(m.path);
      tree.appendChild(row);
    }
    if (!meta.length) tree.textContent = '(no props drawn — check kit / slot)';
  };
  const refreshSel = () => {    // six sliders for the selected node's Adj (precise entry; the handles do the same by dragging)
    selBox.innerHTML = ''; panel.prune(); const n = gizmo.node();
    const head = document.createElement('div'); head.style.cssText = 'color:#9bb;margin:4px 0 2px'; selBox.appendChild(head);
    if (!n) { head.textContent = gizmo.selected ? `selected: ${gizmo.selected} (not drawn right now)` : 'nothing selected · click a prop in the view'; return; }
    head.textContent = `selected: ${n.path} — offset (m) / rotation (°) in its parent frame`;
    const adj = n.getAdj();
    const mk = (label: string, arr: number[], i: number, min: number, max: number, step: number) => panel.slider(selBox, label, min, max, step, () => arr[i], v => { arr[i] = v; if (!rigout.classList.contains('hidden')) rigout.textContent = rigSource(); });
    ['off.x (side)', 'off.y (up)', 'off.z (along)'].forEach((l, i) => mk(l, adj.off, i, -0.3, 0.3, 0.001));
    ['rot.x (pitch)', 'rot.y (yaw)', 'rot.z (roll)'].forEach((l, i) => mk(l, adj.rot, i, -180, 180, 0.5));
    panel.slider(selBox, 'handle opacity', 0.1, 1, 0.05, () => gizmo.opacity, v => { gizmo.opacity = v; });
    panel.buttons(selBox, '', [{ text: 'parent (U)', run: () => gizmo.selectParent() }, { text: 'zero this node', run: () => { adj.off.fill(0); adj.rot.fill(0); panel.refresh(); if (!rigout.classList.contains('hidden')) rigout.textContent = rigSource(); } }, { text: 'deselect (Esc)', run: () => gizmo.select(null) }]);
    const note = document.createElement('div'); note.style.cssText = 'color:#788;margin:3px 0'; note.textContent = 'off / rot act in the parent frame drawn by the handles; props carry a full 3D rotation (Box.rot) and are traced like everything else'; selBox.appendChild(note);
  };
  gizmo.onSelect = () => { treeKey = ''; rebuildTree(); refreshSel(); };
  gizmo.onChange = () => { panel.refresh(); if (!rigout.classList.contains('hidden')) rigout.textContent = rigSource(); };
  refreshSel();
  const S = engine.settings;

  const a = panel.section('Animation');
  panel.select(a, 'locomotion', DRIVES.map(d => d.name), () => st.drive, v => { st.drive = v; });
  panel.slider(a, 'speed ×', 0, 1.5, 0.05, () => st.speedMul, v => { st.speedMul = v; }, 'ground speed as a multiple of the state\'s natural clip speed (playback rate follows, like in game)');
  panel.select(a, 'path', ['treadmill (in place)', 'walk a 2 m circle'], () => st.path, v => { st.path = v; if (v === 0) { char.pos = v3.copy(stage.center); } });
  panel.select(a, 'upper body', UPPERS.map(u => u.name), () => st.upper, v => { st.upper = v; autoFireT = 0.3; });
  panel.slider(a, 'aim pitch °', -60, 70, 1, () => st.aimPitchDeg, v => { st.aimPitchDeg = v; }, 'blends Pistol_Aim_Down/Neutral/Up + spine twist; also tilts the guard torch');
  panel.slider(a, 'aim yaw offset °', -100, 100, 1, () => st.aimYawDeg, v => { st.aimYawDeg = v; }, 'chest twist toward the aim relative to the hips');
  panel.slider(a, 'look yaw °', -80, 80, 1, () => st.lookYawDeg, v => { st.lookYawDeg = v; }, 'extra neck/head yaw (guards looking around) — the goggles must follow');
  panel.buttons(a, 'one-shots', [{ text: 'shoot', run: shoot, title: 'K' }, { text: 'reload', run: () => char.anim.reload() }, { text: 'hit', run: () => char.anim.hit() }, { text: 'throw', run: () => char.anim.throwItem() }]);
  // death = the game's ragdoll (Character.ragdollize): 'shot' throws it along the camera's view direction like Game.killGuard does along the round, 'takedown' folds it forward
  const camShotDir = (): Vec3 => { const d = v3.sub(char.pos, cam.pos); d[1] = 0; return v3.normalize(d); };
  panel.buttons(a, '', [{ text: 'die (shot)', run: () => char.ragdollize(stageCol, camShotDir(), false), title: 'X' }, { text: 'die (takedown)', run: () => char.ragdollize(stageCol, char.forward(), true) }, { text: 'respawn', run: () => spawn() }]);
  panel.select(a, 'raw clip', ['— (game graph)', ...clipNames], () => st.clip, v => { st.clip = v; }, 'bypass the graph and loop one library clip over the rest pose (props stay attached)');
  panel.slider(a, 'scrub', 0, 1, 0.005, () => { const r = char.anim.rawClip; return r ? ((r.time % r.clip.duration) + r.clip.duration) % r.clip.duration / r.clip.duration : char.anim.phase; }, v => { const r = char.anim.rawClip; if (r) r.time = v * r.clip.duration; else char.anim.phase = v; }, 'normalized clip time (raw clip) / locomotion phase (graph) — pause first');
  panel.toggleBox(a, 'paused (P)', () => st.paused, v => { st.paused = v; });
  panel.slider(a, 'playback rate', 0, 2, 0.05, () => st.rate, v => { st.rate = v; }, 'time scale for the character only');
  panel.toggleBox(a, 'turntable (T)', () => st.turntable, v => { st.turntable = v; });
  panel.slider(a, 'turntable °/s', -90, 90, 5, () => st.turnRate, v => { st.turnRate = v; });
  panel.slider(a, 'body yaw °', -180, 180, 1, () => st.bodyYawDeg, v => { st.bodyYawDeg = v; char.bodyYaw = v * DEG; }, '0 = facing the default camera');
  panel.slider(a, 'anim walk speed', 0.8, 2.5, 0.05, () => char.anim.params.walkSpeed, v => { char.anim.params.walkSpeed = v; }, 'ground speed at which Walk_Loop plays 1:1 (foot-slide tuning: use "walk a circle" and the floor marks)');
  panel.slider(a, 'anim jog speed', 2, 5, 0.05, () => char.anim.params.jogSpeed, v => { char.anim.params.jogSpeed = v; });

  // ---- the procedural tactical vocabulary layered on the graph (character.ts: stance / kickDoor / signal / lockpick) — same animator for player and guard kit
  const fireTac = (i = st.tacShot) => { const an = char.anim; if (an.rawClip) return; if (i === 0) an.kickDoor(); else if (i === 4) an.flinch(); else an.signal((['hold', 'go', 'rally'] as SignalKind[])[i - 1]); };
  { const tc = panel.section('Tactical (procedural)');
    panel.select(tc, 'stance', ['none', 'high ready (pieing / covering)', 'low ready (moving)', 'stack (on a door)'], () => st.stance, v => { st.stance = v; }, 'continuous upper-body carry layered under aim: composes with the aim yaw offset, walking and crouch; aim / shoot still win');
    panel.select(tc, 'stack side (wall on…)', ['left', 'right'], () => st.stackSide > 0 ? 1 : 0, v => { st.stackSide = v ? 1 : -1; }, 'mirrors the stack: pelvis blades toward the wall, head turns to the door');
    panel.buttons(tc, 'one-shots', [{ text: 'kick', run: () => fireTac(0), title: 'front kick at handle height; the door should move at the impact instant (flash on the HUD line)' }, { text: 'hold', run: () => fireTac(1) }, { text: 'go', run: () => fireTac(2) }, { text: 'rally', run: () => fireTac(3) }, { text: 'flinch', run: () => fireTac(4) }]);
    panel.toggleBox(tc, 'lockpick (player)', () => st.lockpick, v => { st.lockpick = v; }, 'forces the crouch; hands work a keyway 0.45 m ahead at 0.96 m (anim.lockpickAt); once the hands are half way there the animator raises hideHeldItem and the pistol / can leaves the hand (sidearm shown holstered, hand light off)');
    panel.select(tc, 'loop this one-shot', TAC_SHOTS, () => st.tacShot, v => { st.tacShot = v; tacT = 0.2; });
    panel.toggleBox(tc, 'loop every N s', () => st.tacLoop, v => { st.tacLoop = v; tacT = 0.2; }, 're-fires the selected one-shot for inspection; playback rate and pause apply (the one-shot clocks run inside the animator)');
    panel.slider(tc, 'N', 1, 4, 0.1, () => st.tacEvery, v => { st.tacEvery = v; });
    panel.text(tc, () => { const an = char.anim; return `stance ${STANCES[st.stance]}${st.stance === 3 ? (st.stackSide > 0 ? ' R' : ' L') : ''} · ${an.kicking ? `kick t=${an.kickTime.toFixed(2)}${an.kickTime >= KICK_IMPACT ? ' (past impact)' : ''}` : an.signalling ? 'signalling' : an.lockpick ? 'picking' : 'idle'}${an.hideHeldItem ? ' · held item hidden' : ''} · impact instant = ${KICK_IMPACT}s`; });
    /** Pose the figure at an exact instant of a one-shot for a still: pauses the viewer's character clock, pushes the panel state into the animator, runs out
     *  anything still playing and lets the layers settle (0.8 s), fires `shot` (if given) and advances the animator alone to `t` in 1/120 s steps. The frame
     *  loop keeps rendering the frozen pose (props, lights, camera live), so orbit / __shot afterwards; un-pause to carry on. */
    const poseAt = (t: number, shot?: 'kick' | SignalKind | 'flinch') => {
      const an = char.anim; st.paused = true;
      an.stance = STANCES[st.stance]; an.stackSide = st.stackSide; an.lockpick = st.lockpick; an.speed = DRIVES[st.drive].speed(an.params) * st.speedMul; an.crouchTarget = DRIVES[st.drive].crouch; an.upper = UPPERS[st.upper].mode;
      for (let i = 0; i < 400 && (an.kicking || an.signalling); i++) char.update(1 / 60);
      for (let i = 0; i < 48; i++) char.update(1 / 60);
      if (shot) fireTac(shot === 'kick' ? 0 : shot === 'flinch' ? 4 : ['hold', 'go', 'rally'].indexOf(shot) + 1);
      for (let i = 0, n = Math.max(0, Math.round(t * 120)); i < n; i++) char.update(1 / 120);
      return `${shot ?? 'settled'} @ ${t.toFixed(3)} s${an.kicking ? ` (kick t=${an.kickTime.toFixed(3)})` : ''} — viewer paused`;
    };
    window.__tac = {   // scripted capture: __tac.stance('stack', -1); __tac.kick(); __tac.signal('go'); __tac.lockpick(true); __tac.at(0.38, 'kick')
      stance: (name: Stance, side?: -1 | 1) => { const i = STANCES.indexOf(name); if (i < 0) throw new Error(`stance: one of ${STANCES.join(' ')}`); st.stance = i; if (side) st.stackSide = side; },
      kick: () => fireTac(0), signal: (kname: SignalKind) => { const i = ['hold', 'go', 'rally'].indexOf(kname); if (i < 0) throw new Error('signal: hold | go | rally'); fireTac(i + 1); }, flinch: () => fireTac(4),
      lockpick: (on: boolean) => { st.lockpick = on; }, loop: (on: boolean, shot?: number, every?: number) => { st.tacLoop = on; if (shot !== undefined) st.tacShot = shot; if (every) st.tacEvery = every; tacT = 0.2; },
      at: poseAt, resume: () => { st.paused = false; },
    };
  }

  const k = panel.section('Kit');
  panel.toggleBox(k, 'goggles worn', () => st.goggles, v => { st.goggles = v; });
  panel.toggleBox(k, 'night vision on (N)', () => st.nv, v => { st.nv = v; }, 'on: lenses down over the eyes, blazing · off: parked on the forehead, dim');
  panel.toggleBox(k, 'NV screen effect too', () => st.nvScreen, v => { st.nvScreen = v; }, 'also run the game\'s night-vision post (you will not see the lens colours through it)');
  panel.select(k, 'slot', ['1 · Five-seveN', '2 · empty hand (canister)'], () => st.slot - 1, v => { st.slot = v + 1; });
  panel.toggleBox(k, 'rail light / torch on (L)', () => st.lightOn, v => { st.lightOn = v; });
  panel.toggleBox(k, 'show as guard (G)', () => st.guard, v => { st.guard = v; }, 'guard tint + guard kit: hand torch (patrol carry) or drawn pistol with weapon light');
  panel.select(k, 'guard carry', ['torch (patrol)', 'pistol drawn'], () => st.guardTorch ? 0 : 1, v => { st.guardTorch = v === 0; });
  panel.toggleBox(k, 'draw props', () => st.showProps, v => { st.showProps = v; }, 'off = bare mannequin (to see what the boxes hide)');
  panel.toggleBox(k, 'gun smoke on shoot', () => st.gunSmoke, v => { st.gunSmoke = v; });
  panel.slider(k, 'rail light intensity', 0, 300, 5, () => railLight.peakIntensity, v => { railLight.peakIntensity = railLight.intensity = v; });
  panel.slider(k, 'torch intensity', 0, 300, 5, () => torchLight.peakIntensity, v => { torchLight.peakIntensity = torchLight.intensity = v; });

  // ---- dropped items: the game's item models + physics (Throwables) tossed onto the stage — the pistol / torch a dying hand lets go of, the canister bodies, a magazine
  { const ds = panel.section('Dropped items (tossed from the hand)', false);
    const toss = (kind: 'pistol' | 'torch' | 'smoke' | 'flash' | 'mag', suppressed = false) => {
      const h = char.bones.handR ?? v3.add(char.pos, [0, 1.1, 0]); const f = char.forward();
      const it = stageItems.spawn(kind, [h[0], Math.max(0.3, h[1]), h[2]], [f[0] * 1.4 + (Math.random() - 0.5) * 0.5, 1.0, f[2] * 1.4 + (Math.random() - 0.5) * 0.5], { fill: suppressed ? 1 : 0, fuse: 1e9, life: 600 });
      if (kind === 'pistol') it.rounds = 5; msg(`tossed: ${kind}${suppressed ? ' (suppressed)' : ''}`); };
    panel.buttons(ds, 'toss', [{ text: 'pistol', run: () => toss('pistol') }, { text: 'Five-seveN (can)', run: () => toss('pistol', true) }, { text: 'torch', run: () => toss('torch') }, { text: 'smoke can', run: () => toss('smoke') }, { text: 'stun can', run: () => toss('flash') }, { text: 'magazine', run: () => toss('mag') }]);
    panel.buttons(ds, '', [{ text: 'clear items', run: () => { stageItems.items.length = 0; } }]);
    panel.text(ds, () => `${stageItems.items.length} item(s) · models live in throwables.ts boxes() (pistol: slide + raked grip + can; torch: body + dark lens)`);
    window.__toss = toss;
  }

  // ---- canisters: the two throwables on the stage floor (the game's own effect functions), for grading their plume / burst
  { const cs = panel.section('Canisters (on the floor mark, 1.6 m ahead)', false);
    const canState = { loop: false, loopEvery: 8, plume: true, t: 0 };
    const dropPoint = (): Vec3 => { const f = char.forward(); return [char.pos[0] + f[0] * 1.6, 0.05, char.pos[2] + f[2] * 1.6]; };
    const smokeCan = () => { const at = dropPoint(); const yaw = char.bodyYaw;
      if (canState.plume) smoke.spawnCanister(at);                              // the shipped plume (upward jet: Smoke panel numbers below)
      else fxm.spawnVent(smoke, fxm.ventMouth(at, yaw), yaw);                   // the ported floor vent (effects.ts `vent`)
      msg(canState.plume ? 'smoke canister: plume' : 'smoke canister: floor vent'); };
    const stunCan = () => { const at = dropPoint(); const B = fxm.bang; const L = fxm.spawnBurst(smoke, sparks, [at[0], at[1] + B.height, at[2]], fxSeed++);
      const light = engine.lights.flash(L.pos, L.color, L.intensity, L.range, L.ttl, 0); if (L.radius) light.radius = L.radius; msg('stun canister'); };
    window.__cans = { smokeCan, stunCan };
    panel.buttons(cs, 'fire', [{ text: 'smoke can', run: smokeCan }, { text: 'stun can', run: stunCan }, { text: 'clear smoke', run: () => smoke.clearAll() }]);
    panel.toggleBox(cs, 'smoke can = shipped plume', () => canState.plume, v => { canState.plume = v; }, 'off = the ported floor-hugging vent (effects.ts vent)');
    panel.toggleBox(cs, 'loop (alternate every N s)', () => canState.loop, v => { canState.loop = v; canState.t = 0; });
    panel.slider(cs, 'loop period s', 3, 30, 1, () => canState.loopEvery, v => { canState.loopEvery = v; });
    const P = smoke.params; const V = fxm.vent; const B = fxm.bang; const SP2 = fxm.sparks;
    panel.slider(cs, 'plume density /s', 0, 200, 1, () => P.canisterDensity, v => { P.canisterDensity = v; });
    panel.slider(cs, 'plume seconds', 2, 60, 1, () => P.canisterDuration, v => { P.canisterDuration = v; });
    panel.slider(cs, 'plume radius m', 0.04, 0.4, 0.01, () => P.canisterRadius, v => { P.canisterRadius = v; });
    panel.slider(cs, 'plume heat', 0, 8, 0.1, () => P.canisterTemp, v => { P.canisterTemp = v; });
    panel.slider(cs, 'plume jet m/s', 0, 6, 0.1, () => P.canisterSpeed, v => { P.canisterSpeed = v; });
    panel.slider(cs, 'vent density /s', 0, 60, 0.5, () => V.density, v => { V.density = v; });
    panel.slider(cs, 'vent speed m/s', 0, 9, 0.1, () => V.speed, v => { V.speed = v; });
    panel.slider(cs, 'vent radius m', 0.05, 0.6, 0.01, () => V.radius, v => { V.radius = v; });
    panel.slider(cs, 'vent push', 0, 90, 1, () => V.push, v => { V.push = v; });
    panel.slider(cs, 'vent seconds', 2, 60, 1, () => V.seconds, v => { V.seconds = v; });
    panel.slider(cs, 'stun density ×', 0, 4, 0.05, () => B.densityGain, v => { B.densityGain = v; });
    panel.slider(cs, 'stun heat ×', 0, 3, 0.05, () => B.tempGain, v => { B.tempGain = v; });
    panel.slider(cs, 'stun expand', 0, 200, 1, () => B.expand, v => { B.expand = v; }, 'divergence source: how hard the burst shoves its neighbourhood apart');
    panel.slider(cs, 'stun end-jet m/s', 0, 14, 0.5, () => B.ventSpeed, v => { B.ventSpeed = v; });
    panel.slider(cs, 'stun asymmetry', 0, 1, 0.05, () => B.asymmetry, v => { B.asymmetry = v; });
    panel.slider(cs, 'stun body vents', 0, 8, 1, () => B.bodyVents, v => { B.bodyVents = v; });
    panel.slider(cs, 'stun wisp density', 0, 5, 0.1, () => B.wispDensity, v => { B.wispDensity = v; });
    panel.slider(cs, 'stun wisp life s', 0, 12, 0.5, () => B.wispLife, v => { B.wispLife = v; });
    panel.slider(cs, 'stun light power', 0, 3000, 20, () => B.lightPower, v => { B.lightPower = v; });
    panel.slider(cs, 'stun sparks', 0, 96, 1, () => B.sparkCount, v => { B.sparkCount = v; });
    panel.slider(cs, 'stun spark speed', 2, 30, 0.5, () => SP2.speed, v => { SP2.speed = v; });
    panel.slider(cs, 'stun spark spread', 0, 1.5, 0.05, () => SP2.spread, v => { SP2.spread = v; });
    panel.slider(cs, 'stun spark lift', 0, 1.5, 0.05, () => SP2.lift, v => { SP2.lift = v; });
    panel.slider(cs, 'stun spark life s', 0.1, 3, 0.05, () => SP2.life, v => { SP2.life = v; });
    panel.slider(cs, 'stun smoke trails', 0, 20, 1, () => B.trailCount, v => { B.trailCount = v; });
    panel.text(cs, () => `numbers are src/game/effects.ts (vent / bang / sparks) + the Smoke panel's canister block — note the ones you like`);
    canTick = (dt: number) => { if (!canState.loop) return; canState.t -= dt; if (canState.t <= 0) { canState.t = canState.loopEvery; (Math.floor(performance.now() / 1000 / canState.loopEvery) % 2 === 0 ? smokeCan : stunCan)(); } };
  }
  // ---- effects: the same live numbers the game's Effects panel edits (src/game/effects.ts + spark params) — press K / 'shoot' to see them
  { const ef = panel.section('Effects (shot: K)', false); const SH = fxm.shot; const SP = sparks.params;
    panel.toggleBox(ef, 'emitter gizmos', () => st.emitterGizmos, v => { st.emitterGizmos = v; }, 'bore jet / side ports / muzzle wisp anchors as draggable nodes on the muzzle frame (hierarchy: emitters ▸ …); offsets are [right, up, forward] m in fxm.shot.jetOff / portOff / wispOff');
    panel.text(ef, () => `jetOff [${SH.jetOff.map(v => v.toFixed(3)).join(', ')}] · portOff [${SH.portOff.map(v => v.toFixed(3)).join(', ')}] · wispOff [${SH.wispOff.map(v => v.toFixed(3)).join(', ')}]`);
    panel.slider(ef, 'shot light power', 0, 400, 5, () => SH.lightPower, v => { SH.lightPower = v; });
    panel.slider(ef, 'shot light s', 0.02, 0.3, 0.01, () => SH.lightDuration, v => { SH.lightDuration = v; });
    panel.slider(ef, 'jet density /s', 0, 120, 1, () => SH.jetDensity, v => { SH.jetDensity = v; }, 'bore jet; the two side ports carry portFraction of it');
    panel.slider(ef, 'jet heat /s', 0, 400, 5, () => SH.jetTemp, v => { SH.jetTemp = v; });
    panel.slider(ef, 'wisp density /s', 0, 30, 0.5, () => SH.wispDensity, v => { SH.wispDensity = v; });
    panel.slider(ef, 'wisp heat /s', 0, 150, 1, () => SH.wispTemp, v => { SH.wispTemp = v; });
    panel.toggleBox(ef, 'ejection-port curl', () => fxm.flags.ejectionWisp, v => { fxm.flags.ejectionWisp = v; });
    panel.toggleBox(ef, 'powder sparks', () => fxm.flags.shotSparks, v => { fxm.flags.shotSparks = v; });
    panel.slider(ef, 'sparks per shot', 0, 48, 1, () => SH.sparkCount, v => { SH.sparkCount = v; });
    panel.slider(ef, 'spark speed m/s', 2, 40, 0.5, () => SH.sparkSpeed, v => { SH.sparkSpeed = v; }, 'streak length = speed × shutter');
    panel.slider(ef, 'spark cone rad', 0.05, 1.2, 0.01, () => SH.sparkCone, v => { SH.sparkCone = v; });
    panel.slider(ef, 'spark life s', 0.03, 0.6, 0.01, () => SH.sparkLife, v => { SH.sparkLife = v; });
    panel.slider(ef, 'spark gravity', 0, 30, 0.5, () => SP.gravity, v => { SP.gravity = v; });
    panel.slider(ef, 'spark drag /s', 0, 6, 0.1, () => SP.drag, v => { SP.drag = v; });
    panel.slider(ef, 'spark brightness ×', 0.1, 8, 0.1, () => SP.brightness, v => { SP.brightness = v; });
    panel.slider(ef, 'spark hold', 0, 0.9, 0.02, () => SP.hold, v => { SP.hold = v; }, 'fraction of its life a spark stays white-hot before fading orange → red');
    panel.slider(ef, 'streak thickness mm', 2, 30, 1, () => SP.thickness * 1000, v => { SP.thickness = v / 1000; });
    panel.slider(ef, 'shutter ms', 2, 50, 1, () => SP.shutter * 1000, v => { SP.shutter = v / 1000; });
    const P = smoke.params;
    panel.slider(ef, 'smoke buoyancy', 0, 6, 0.05, () => P.buoyancy, v => { P.buoyancy = v; });
    panel.slider(ef, 'smoke vorticity', 0, 12, 0.1, () => P.vortConf, v => { P.vortConf = v; });
    panel.slider(ef, 'smoke turbulence', 0, 8, 0.1, () => P.turbulence, v => { P.turbulence = v; });
    panel.slider(ef, 'density decay /s', 0, 1, 0.01, () => P.densDecay, v => { P.densDecay = v; });
    panel.slider(ef, 'temp decay /s', 0, 4, 0.05, () => P.tempDecay, v => { P.tempDecay = v; });
    panel.slider(ef, 'velocity damping', 0, 2, 0.02, () => P.velDamp, v => { P.velDamp = v; });
    panel.text(ef, () => `sparks live ${sparks.live} · smoke domains ${smoke.stats.live} emitters ${smoke.stats.emitters} — numbers are shared with the game's Effects / Smoke panels; note them down or edit src/game/effects.ts`);
  }
  const rc = panel.section('Rig constants');
  panel.text(rc, () => 'every slider below writes straight into rigProps.RIG (metres along / across the bone axes, 1 mm steps). "copy" puts the whole `export const RIG` statement on the clipboard and in the box bottom-left.');
  panel.buttons(rc, '', [{ text: 'copy constants', run: () => { void copyConstants(); } }, { text: 'show', run: showSource }, { text: 'hide', run: () => rigout.classList.add('hidden') }, { text: 'reset', run: resetConstants }]);
  const rigSection = (group: keyof Rig, open: boolean) => {
    const sec = panel.section(`RIG.${group}`, open);
    const leaf = (label: string, path: string, get: () => number, set: (v: number) => void) => {
      let min = -0.2, max = 0.45, step = 0.001; let title: string | undefined;      // offsets along / across an axis (m)
      if (/thick|tall|half/i.test(path)) { min = 0.002; max = 0.12; }             // half extents (m)
      if (/glow/i.test(path)) { min = 0; max = /lensGlow/.test(path) ? 100 : 40; step = /lensGlow/.test(path) ? 1 : 0.1; title = 'screen glow + bloom only: the lit lens boxes are raster-only, no ray sees them, so this never lights the face or the wall'; }
      if (/follow/i.test(path)) { min = 0; max = 1; step = 1; title = '0: offsets in a yaw-only head frame (shipped) · 1: offsets ride the skull\'s full rotation (crouch / aim-down / death keep them on the face); boxes still only yaw'; }
      panel.slider(sec, label, min, max, step, get, v => { set(v); if (!rigout.classList.contains('hidden')) rigout.textContent = rigSource(); }, title);
    };
    const walk = (obj: Record<string, unknown>, path: string) => {
      for (const key of Object.keys(obj)) {
        if (/adj$/i.test(key)) continue;                          // free adjustments belong to the gizmo / selection sliders (degrees, not metres; recreated on reset)
        const v = obj[key]; const p = path ? `${path}.${key}` : key;
        if (typeof v === 'number') leaf(p, p, () => obj[key] as number, x => { obj[key] = x; });
        else if (Array.isArray(v)) { const comps = /glow/i.test(p) ? 'rgb' : 'xyz'; v.forEach((_, i) => leaf(`${p}.${comps[i] ?? i}`, p, () => v[i] as number, x => { v[i] = x; })); }   // assignRig() writes arrays in place, so these closures survive a reset
        else if (v && typeof v === 'object') walk(v as Record<string, unknown>, p);
      }
    };
    walk(RIG[group] as unknown as Record<string, unknown>, '');
  };
  rigSection('goggles', true); rigSection('pistol', true); rigSection('torch', false); rigSection('guardPistol', false);

  const sc = panel.section('Stage & camera');
  panel.buttons(sc, 'frame', [{ text: 'body', run: () => frameShot('body'), title: 'F' }, { text: 'back', run: () => frameShot('back') }, { text: 'head', run: () => frameShot('head'), title: 'H' }, { text: 'hand', run: () => frameShot('hand') }, { text: 'feet', run: () => frameShot('feet') }]);
  panel.select(sc, 'track', TRACK, () => st.track, v => { st.track = v; }, 'what the orbit centre follows (right-drag pans and frees it)');
  panel.slider(sc, 'distance', 0.5, 12, 0.05, () => cam.desiredDistance, v => { cam.desiredDistance = v; });
  panel.slider(sc, 'pitch °', -8, 85, 1, () => cam.pitch / DEG, v => { cam.pitch = v * DEG; });
  panel.slider(sc, 'yaw °', -180, 180, 1, () => wrapAngle(cam.yaw) / DEG, v => { cam.yaw = v * DEG; });
  panel.slider(sc, 'fov °', 15, 70, 1, () => cam.fov / DEG, v => { cam.fov = v * DEG; });
  panel.toggleBox(sc, 'back wall', () => st.wall, v => { st.wall = v; }, 'off = cut down to a lip (rays still see it: the bounce stays)');
  for (const [i, L] of stageLights.entries()) panel.slider(sc, `${L.name} light`, 0, i === 0 ? 200 : 80, 1, () => L.enabled ? L.intensity : 0, v => { L.intensity = L.peakIntensity = v; L.enabled = v > 0; });
  let skyScale = 1; const z0 = [...S.skyZenith], h0 = [...S.skyHorizon];
  panel.slider(sc, 'sky brightness', 0, 20, 0.1, () => skyScale, v => { skyScale = v; S.skyZenith = [z0[0] * v, z0[1] * v, z0[2] * v]; S.skyHorizon = [h0[0] * v, h0[1] * v, h0[2] * v]; }, 'the stage has no ceiling: rays leaving upward take the (night) sky');

  const r = panel.section('Renderer', false);
  panel.text(r, () => { const t = engine.timer; return `gpu: ${[...t.results.entries()].filter(([n]) => !['composite', 'fxaa'].includes(n)).map(([n, v]) => `${n} ${v.toFixed(2)}`).join(' · ')}`; });
  panel.select(r, 'quality preset', QUALITY_NAMES.map(q => q), () => QUALITY_NAMES.indexOf(quality.current), v => quality.apply(QUALITY_NAMES[v]));
  panel.toggleBox(r, 'adaptive resolution', () => quality.adaptive, v => { quality.adaptive = v; });
  panel.slider(r, 'resolution scale', 0.3, 1, 0.05, () => S.renderScale, v => { S.renderScale = v; });
  panel.slider(r, 'exposure', 0.2, 8, 0.05, () => S.exposure, v => { S.exposure = v; });
  panel.select(r, 'debug view', DEBUG_VIEWS, () => S.debugView, v => { S.debugView = v; });
  panel.slider(r, 'haze density', 0, 0.2, 0.005, () => S.hazeDensity, v => { S.hazeDensity = v; }, 'makes the rail light / torch beam visible');
  panel.slider(r, 'bloom', 0, 0.3, 0.005, () => engine.passes.bloom, v => { engine.passes.bloom = v; });
  panel.slider(r, 'emissive scale', 0, 4, 0.05, () => S.emissiveScale, v => { S.emissiveScale = v; }, 'global multiplier on emissive surfaces (for the lit lenses that is purely their on-screen glow)');
  panel.toggleBox(r, 'fxaa', () => engine.passes.fxaaEnabled, v => engine.passes.setFxaa(v));

  help.textContent = 'click a prop: select (handles: arrows move, rings rotate · U parent · Esc deselect) · drag: orbit · shift-drag / right-drag: pan · wheel: zoom · F frame body · H head close-up · T turntable · P pause · K shoot · X die (ragdoll, shot from the camera) · N night vision · L light · G guard · 1/2 slot · Tab panel · ` debug views · [ ] exposure';
  // __shot(name): engine readback → JPEG → capture sink (:5174), like the game page's (headless grading of the stage)
  window.__char = char;
  window.__shot = async (name: string, quality = 0.86) => {
    engine.render(cam, window.__vtime ?? 0, 1 / 60, false);
    const c2 = await frameCanvas(engine); if (!c2) return 'no frame';
    const blob: Blob = await new Promise(res => c2.toBlob(b => res(b!), 'image/jpeg', quality));
    return `${c2.width}x${c2.height} ${(blob.size / 1024).toFixed(0)} KB → ${await postShot(name, blob)}`;
  };
  window.__engine = engine; window.__cam = cam; window.__rig = RIG; window.__gizmo = gizmo; window.__step = (n = 1) => { for (let i = 0; i < n; i++) step(1 / 60); }; window.__rigSource = rigSource; window.__viewer = st;
  Object.defineProperty(window, '__char', { get: () => char, configurable: true });

  // ---------------------------------------------------------------- frame
  let last = performance.now(); let time = 0; let fpsSm = 60; let frameCount = 0;
  const driveSpeed = () => DRIVES[st.drive].speed(char.anim.params) * st.speedMul;
  function frame() {
    const now = performance.now(); const rawDt = (now - last) / 1000; const dt = Math.min(1 / 20, rawDt); last = now;
    if (!document.hidden && rawDt < 0.5) quality.sample(rawDt);
    try { step(dt); } catch (e) { console.error(e); showFatal(String((e as Error)?.stack ?? e)); return; }
    requestAnimationFrame(frame);
  }
  function step(dt: number) {
    time += dt;
    // ---- keys
    if (input.hit('Tab')) panel.toggle();
    if (input.hit('KeyP')) st.paused = !st.paused;
    if (input.hit('KeyU')) gizmo.selectParent();                 // up the hierarchy
    if (input.hit('Escape')) gizmo.select(null);
    if (input.hit('KeyT')) st.turntable = !st.turntable;
    if (input.hit('KeyN')) { st.nv = !st.nv; msg(st.nv ? 'night vision on (goggles down)' : 'night vision off (goggles up)'); }
    if (input.hit('KeyL')) st.lightOn = !st.lightOn;
    if (input.hit('KeyG')) st.guard = !st.guard;
    if (input.hit('KeyK')) shoot();
    if (input.hit('KeyX')) char.ragdollize(stageCol, camShotDir(), false);
    if (input.hit('KeyF')) frameShot('body');
    if (input.hit('KeyH')) frameShot('head');
    if (input.hit('Digit1')) st.slot = 1;
    if (input.hit('Digit2')) st.slot = 2;
    if (input.hit('Backquote')) S.debugView = (S.debugView + (input.down('ShiftLeft') ? DEBUG_VIEWS.length - 1 : 1)) % DEBUG_VIEWS.length;
    if (input.hit('BracketLeft')) S.exposure /= 1.25;
    if (input.hit('BracketRight')) S.exposure *= 1.25;
    // ---- orbit camera (mouse deltas from the game's Input; the panel swallows its own events)
    const dx = input.mouseX - lastMX, dy = input.mouseY - lastMY; lastMX = input.mouseX; lastMY = input.mouseY;
    const shiftPan = input.lmb() && (input.down('ShiftLeft') || input.down('ShiftRight'));
    if (input.lmb() && !gizmo.busy && !shiftPan) { cam.yaw -= dx * 0.008; cam.pitch = clamp(cam.pitch + dy * 0.006, -8 * DEG, 85 * DEG); }   // 'grab the mannequin' sense (unless a gizmo handle has the drag)
    if (input.rmb() || input.mmb() || (shiftPan && !gizmo.busy)) { const { right } = cam.planarBasis(); const kp = cam.distance * 0.0017; focus = v3.mad(focus, right, -dx * kp); focus[1] = clamp(focus[1] + dy * kp, 0.03, 2.8); st.track = 0; }
    if (input.wheel) cam.desiredDistance = clamp(cam.desiredDistance * Math.exp(input.wheel * 0.0012), 0.5, 12);

    if (canvas.clientWidth > 0 && canvas.clientHeight > 0) {
      // ---- drive the character exactly through the fields game.ts writes
      const c = char; const D = DRIVES[st.drive], U = UPPERS[st.upper];
      const cdt = st.paused ? 0 : dt * st.rate;
      const speed = driveSpeed();
      if (st.clip > 0) { const clip = ch.clips.get(clipNames[st.clip - 1])!; if (c.anim.rawClip?.clip !== clip) c.anim.rawClip = { clip, time: 0, rate: 1, loop: true }; }
      else c.anim.rawClip = null;
      c.anim.speed = speed; c.anim.crouchTarget = D.crouch; c.anim.reverse = D.reverse;
      c.anim.upper = c.anim.rawClip ? 'none' : U.mode; c.anim.lookYawExtra = st.lookYawDeg * DEG;   // ('aim' would pull gunDir toward the analytic aim while the raw clip's hand is elsewhere)
      c.anim.stance = STANCES[st.stance]; c.anim.stackSide = st.stackSide; c.anim.lockpick = st.lockpick;
      if (st.tacLoop && !st.paused) { tacT -= cdt; if (tacT <= 0) { tacT = st.tacEvery; fireTac(); } }
      if (U.autoFire && !st.paused && !c.anim.rawClip) { autoFireT -= cdt; if (autoFireT <= 0) { autoFireT = 0.6; shoot(); } }
      if (st.turntable && !st.paused) { c.bodyYaw = wrapAngle(c.bodyYaw + st.turnRate * DEG * cdt); st.bodyYawDeg = Math.round(c.bodyYaw / DEG); }
      if (st.path === 1) {   // walk a 2 m circle about the stage centre (analytic, no drift): real foot planting to judge against the floor marks; a backpedal faces against
        circPhi += (speed / CIRCLE_R) * cdt;   // the travel; heading comes from the tangent itself so it holds when the speed drops to 0 (turntable / body yaw only apply on the treadmill)
        c.pos = [stage.center[0] + CIRCLE_R * Math.cos(circPhi), 0, stage.center[2] + CIRCLE_R * Math.sin(circPhi)];
        c.vel = [-Math.sin(circPhi) * speed, 0, Math.cos(circPhi) * speed];             // tangent × speed (d pos / dt)
        c.bodyYaw = wrapAngle(Math.atan2(-Math.sin(circPhi), Math.cos(circPhi)) + (D.reverse ? Math.PI : 0)); st.bodyYawDeg = Math.round(c.bodyYaw / DEG);
      } else c.vel = [0, 0, 0];
      c.aimYaw = c.bodyYaw + st.aimYawDeg * DEG; c.aimPitch = st.aimPitchDeg * DEG;
      const T = st.guard ? GUARD_TINT : PLAYER_TINT; c.tint = T.tint; c.tint2 = T.tint2;
      c.update(cdt);
      if (c.anim.kickImpact) { kickFlashT = 0.25; msg('kick: impact (door goes here)'); } kickFlashT = Math.max(0, kickFlashT - dt);

      // ---- props + hand lights: the functions game.ts uses, fed from the panel state
      dyn.length = 0; meta.length = 0;
      const handsBusy = c.anim.hideHeldItem;   // the animator says the hands are on the lock tools: draw an empty hand (sidearm holstered, no can, no hand light) whatever the kit says
      if (st.guard) {
        let dir = c.gunDir; const torch = st.guardTorch && c.alive;   // game: only a patrolling (hence living) guard carries the torch; a body keeps a lit weapon light
        if (torch) { beamDir = torchCarryDir(c, beamDir); dir = beamDir; }   // attachFlashlight's relaxed carry (same helper)
        else beamDir = v3.copy(dir);
        torchLight.enabled = st.lightOn && !handsBusy; torchLight.pos = rigTorchPos(c, dir, torch); torchLight.dir = dir;
        railLight.enabled = false;
        if (st.showProps) { if (handsBusy) holsterBoxes(c, dyn, { slide: RIG.guardPistol.slide, grip: RIG.guardPistol.grip }, meta); else rigBoxes(c, { goggles: false, nv: false, slot: 1, lightOn: st.lightOn, isGuard: true, torch, beamDir, lensColor: torchLight.color }, dyn, RIG, meta); }
      } else {
        torchLight.enabled = false;
        railLight.enabled = st.lightOn && st.slot === 1 && !handsBusy;
        if (railLight.enabled) { const lp = rigLightPose(c); railLight.pos = lp.pos; railLight.dir = lp.dir; }
        if (st.showProps) rigBoxes(c, { goggles: st.goggles, nv: st.nv, slot: handsBusy ? 0 : st.slot, lightOn: st.lightOn, isGuard: false, holster: handsBusy || c.alive || st.slot !== 1, held: handsBusy ? null : st.slot === 2 ? 'smoke' : st.slot === 3 ? 'flash' : null }, dyn, RIG, meta);   // (as game.ts: can up → gun on the thigh + can in the fist; picking → empty hands, gun on the thigh)
      }
      if (st.emitterGizmos && !st.guard && st.slot === 1 && !handsBusy && st.showProps) {   // shot emitter anchors ride the muzzle frame; a tiny bright cube marks each so it can be picked like a rig part
        const MF = fxm.muzzleFrame(c.muzzle, c.gunDir); const SH = fxm.shot;
        meta.push({ path: 'emitters', label: 'shot emitters (muzzle frame)', depth: 0, parent: MF, frame: MF, boxStart: dyn.length, boxCount: 0, getAdj: () => emitterRootAdj });
        for (const [key, label, col] of [['jetOff', 'bore jet', [1, 0.55, 0.2]], ['portOff', 'side ports (base)', [0.3, 0.7, 1]], ['wispOff', 'muzzle wisp', [0.7, 0.7, 0.7]]] as [keyof typeof SH, string, Vec3][]) {
          const off = SH[key] as Vec3; const o = v3.mad(v3.mad(v3.mad(MF.o, MF.x, off[0]), MF.y, off[1]), MF.z, off[2]);
          meta.push({ path: 'emitters.' + key, label, depth: 1, parent: MF, frame: { ...MF, o }, boxStart: dyn.length, boxCount: 1, getAdj: () => emitterAdj(key as string, off) });
          dyn.push(makeBox({ c: o, h: [0.006, 0.006, 0.006], yaw: 0, albedo: col, emissive: v3.scale(col, 3), flags: BoxFlag.Dynamic | BoxFlag.NoShadow }));
        }
      }
      gizmo.update(meta, dyn); rebuildTree();
      canTick(dt); sparks.update(dt, stageCol); sparks.boxes(dyn); stageItems.update(dt); stageItems.boxes(dyn);   // (fast sparks glance off the back wall / the cube like they do off the office's walls; slow ones only see the floor)
      for (let i = fx.length - 1; i >= 0; i--) { const f = fx[i]; dyn.push(f.box); f.ttl -= dt; if (f.ttl <= 0) fx.splice(i, 1); }
      engine.world.dynamics = dyn;
      // ---- skin instance + capsule shadow proxies (Game.update's per-character block)
      chars.setInstance(0, c.skel.jointData, c.tint, c.tint2, c.id); chars.finish(1);
      engine.world.resetCapsules(); { const { caps, bound } = c.capsules(); engine.world.addCharacterCapsules(c.id, v3.lerp(c.tint, [0.5, 0.5, 0.5], 0.3), caps, bound); }

      // ---- camera + stage
      if (st.track === 1) focus = v3.add(c.pos, [0, 1, 0]);
      else if (st.track === 2 && c.bones.head) focus = v3.add(c.bones.head, [0, 0.07, 0]);
      else if (st.track === 3 && c.bones.handR) focus = v3.mad(c.bones.handR, c.gunDir, 0.12);
      else if (st.track === 4 && c.bones.chest) focus = v3.copy(c.bones.chest);
      { const minP = Math.asin(clamp((0.06 - focus[1]) / Math.max(cam.distance, 0.1), -0.99, 0.99)); if (cam.pitch < minP) cam.pitch = minP; }   // never orbit into the floor slab
      cam.update(dt, focus, null);
      wallBox.cutHeight = st.wall && cam.pos[2] > stage.wallZ + 0.15 ? 100 : 0.12;   // cut the back wall down to a lip when the orbit goes behind it (or when switched off)
      S.nvAmount = st.nvScreen && st.nv ? Math.min(1, S.nvAmount + dt * 5) : Math.max(0, S.nvAmount - dt * 9);
      smoke.update(dt);
      engine.lights.update(dt, time);
      window.__vtime = time; engine.render(cam, time, dt, true);
      gizmo.draw();
    }
    input.endFrame();

    frameCount++; fpsSm = fpsSm * 0.95 + (1 / Math.max(dt, 1e-4)) * 0.05;
    if (frameCount % 10 === 0) {
      const t = engine.timer; const lines: string[] = [];
      lines.push(`character / prop viewer · ${gpu.adapterInfo} | ${engine.width}x${engine.height} (${quality.current}${quality.adaptive ? ', adaptive' : ''} ×${S.renderScale.toFixed(2)}) | ${fpsSm.toFixed(0)} fps | gpu ~${t.total(['composite', 'post', 'fxaa', 'gbuffer']).toFixed(2)} ms`);
      const r = char.anim.rawClip;
      const animLine = r ? `raw clip ${r.clip.name} (${r.clip.duration.toFixed(2)} s) t=${(((r.time % r.clip.duration) + r.clip.duration) % r.clip.duration).toFixed(2)}` : `graph: ${DRIVES[st.drive].name} ${driveSpeed().toFixed(2)} m/s${st.path === 1 ? ' (circle)' : ''} · upper ${UPPERS[st.upper].mode} · phase ${char.anim.phase.toFixed(2)}`;
      lines.push(`${animLine} · stance ${STANCES[st.stance]}${st.lockpick ? ' · lockpick' : ''}${kickFlashT > 0 ? ' · ▮▮ KICK IMPACT ▮▮' : ''} · aim pitch ${st.aimPitchDeg}° · body yaw ${Math.round(wrapAngle(char.bodyYaw) / DEG)}°${st.paused ? ' · PAUSED' : ''}`);
      lines.push(`kit: ${st.guard ? `guard, ${st.guardTorch ? 'torch' : 'pistol drawn'}` : `player, slot ${st.slot}, goggles ${st.goggles ? (st.nv ? 'down (NV on)' : 'up') : 'off'}`} · light ${st.lightOn ? 'on' : 'off'} · props ${dyn.length} boxes · lights ${engine.lights.activeCount} · cam yaw ${Math.round(wrapAngle(cam.yaw) / DEG)}° pitch ${Math.round(cam.pitch / DEG)}° dist ${cam.distance.toFixed(2)} m · view ${S.debugView} · exp ${S.exposure.toFixed(2)}`);
      hud.textContent = lines.join('\n');
      const nowS = performance.now() / 1000; logEl.textContent = messages.filter(m => nowS - m.t < 5).map(m => m.text).join('\n');
    }
  }
  requestAnimationFrame(frame);
}

main().catch(e => { console.error(e); showFatal(String(e?.stack ?? e)); });
