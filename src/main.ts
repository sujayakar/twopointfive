import { Menu } from './ui/menu';
import { initGpu, showFatal } from './gpu/device';
import { Engine } from './engine';
import { buildLevel } from './scene/level';
import { lightFromDef } from './render/lights';
import { FollowCamera } from './game/camera';
import { Input } from './game/input';
import { v3 } from './math/vec';
import { loadGltfCharacter } from './anim/gltf';
import { scenePrelude } from './render/frame';
import lightingCommonSrc from './render/shaders/lighting_common.wgsl' with { type: 'text' };
import { Game } from './game/game';
import { Overlay } from './ui/overlay';
import { Bubbles } from './ui/bubbles';
import { SmokeSystem } from './smoke/smoke';
import { Panel } from './ui/panel';
import { AudioEngine } from './audio/audio';
import { Demo } from './game/demo';
import { Quality, QUALITY_NAMES, QualityName } from './ui/quality';
import { buildPanel } from './ui/bindings';
import { frameCanvas, postShot } from './ui/capture';

/** The top-left GPU / pass / guard read-out: opt-in in play (persisted), always off during the tour. */
const readout = { on: localStorage.getItem('twopointsix.readout') === '1', set(v: boolean) { this.on = v; try { localStorage.setItem('twopointsix.readout', v ? '1' : '0'); } catch { /* */ } } };
(window as any).__readout = readout;

async function main() {
  const canvas = document.getElementById('view') as HTMLCanvasElement;
  const loading = document.getElementById('loading')!;
  const gpu = await initGpu(canvas);
  const engine = new Engine(gpu);
  const level = buildLevel();
  for (const b of level.boxes) engine.world.addStatic(b);
  for (const ld of level.lights) engine.lights.add(lightFromDef(ld));
  engine.settings.skyZenith = level.sky.zenith; engine.settings.skyHorizon = level.sky.horizon;

  loading.textContent = 'loading character rig…';
  const ch = await loadGltfCharacter(new URL('assets/ual/AnimationLibrary_Godot_Standard.gltf', document.baseURI).href);
  const smoke = new SmokeSystem(engine);
  engine.setSmoke(smoke.binding);
  const game = new Game(engine, level, ch, smoke);
  engine.extraPasses.push((enc, sceneBG, diceView, ts) => { if (game.probe.hasPending) game.probe.encode(enc, sceneBG, diceView, ts('probe')); });
  engine.afterSubmit.push(() => game.probe.afterSubmit());
  engine.afterSubmit.push(() => smoke.afterSubmit());
  window.__smoke = smoke;
  loading.style.display = 'none';

  const audio = new AudioEngine(game.col); game.audio = audio;   // col includes the door leaves as dynamic occluders
  // test-rig mute: '?mute' or localStorage twopointsix.testMute=1 (set once in the automation browser's profile) keeps this page silent —
  // master pinned at 0 and the context never started — so scripted / headless runs stay silent
  const testMute = new URLSearchParams(location.search).has('mute') || (() => { try { return localStorage.getItem('twopointsix.testMute') === '1'; } catch { return false; } })();
  if (testMute) { audio.volume.master = 0; const sv = audio.setVolumes.bind(audio); audio.setVolumes = () => { audio.volume.master = 0; sv(); }; audio.start = () => {}; }
  // audio needs a user gesture: any click / key starts it (the start card's button is usually the first)
  window.addEventListener('pointerdown', () => audio.start(), { once: false }); window.addEventListener('keydown', () => audio.start(), { capture: true });   // (capture: Input claims game keys at window-capture and stops them there)
  window.__engine = engine; window.__game = game; window.__audio = audio;
  const input = new Input(canvas);
  const cam = new FollowCamera();
  cam.target = v3.copy(level.playerSpawn);
  window.__cam = cam;
  const overlay = new Overlay();
  const bubbles = new Bubbles(document.getElementById('overlay')!);
  const demo = new Demo(game, cam, smoke, input); window.__demo = demo;
  // WGSL compile check against the live device (Chrome/Tint is the compiler we ship to; naga is not available on this machine):
  // __wgsl(source, { prelude: 'scene' | 'lighting' }) prepends what the scene passes get (world consts + common.wgsl [+ lighting_common.wgsl]),
  // so a worktree's edited pass shader can be checked in seconds without booting its build. Resolves to the compiler messages.
  window.__wgsl = async (code: string, opts: { prelude?: boolean | 'scene' | 'lighting'; label?: string } = {}) => {
    const pre = opts.prelude === 'scene' ? scenePrelude() : opts.prelude ? scenePrelude() + lightingCommonSrc : '';   // 'scene' = what direct/softblur/temporal get; true|'lighting' = + lighting_common (gather, rc, probe, volumetrics)
    const full = pre + code;
    const mod = gpu.device.createShaderModule({ code: full, label: opts.label ?? 'lint' }); const info = await mod.getCompilationInfo();
    const off = pre ? pre.split('\n').length - 1 : 0;
    return info.messages.map(x => ({ type: x.type, line: x.lineNum - off, col: x.linePos, msg: x.message, src: (full.split('\n')[x.lineNum - 1] ?? '').trim() }));
  };
  { let lastHint = -10; input.onLockedInput = () => { const now = performance.now() / 1000; if (demo.active && now - lastHint > 3) { lastHint = now; game.msg('hands-off tour — P to stop · Esc to pause'); } }; }
  const quality = new Quality(engine, smoke); window.__quality = quality;
  { const qp = new URLSearchParams(location.search).get('quality'); if (qp && (QUALITY_NAMES as string[]).includes(qp)) { quality.auto = false; quality.apply(qp as QualityName); } }   // ?quality= pins for this visit without touching the remembered choice
  const panel = new Panel();
  buildPanel(panel, engine, game, smoke, cam, overlay, audio, quality);
  const hud = document.getElementById('hud')!;
  const tourVeil = document.getElementById('veil')!;   // black fade the tour raises around its cuts
  // start card now, pause menu on Esc (settings + controls); the simulation freezes while it is open, the frame keeps rendering
  const menu = new Menu({ engine, game, quality, audio, overlay, startTour: () => { if (!demo.active) demo.start(); }, stopTour: () => { if (demo.active) demo.stop(); }, focusGame: () => input.focusGame(), onStart: () => { audio.start(); } });
  window.__menu = menu;
  window.addEventListener('keydown', e => { if (e.code === 'Escape' && !e.repeat) { if (!menu.isOpen && panelWantsEsc()) { (document.activeElement as HTMLElement).blur(); return; } menu.toggle(); e.preventDefault(); } }, { capture: true });
  const panelWantsEsc = () => { const a = document.activeElement; return !!a && (a instanceof HTMLInputElement || a instanceof HTMLSelectElement) && !!a.closest('#panel'); };   // first Esc just releases a focused tweak-panel control; controls inside the menu never block it
  if (!new URLSearchParams(location.search).has('demo')) menu.open('start');

  let last = performance.now(); let timeAcc = 0; let fpsSm = 60; let frameCount = 0;
  const debugNames = ['final', 'albedo', 'normals', 'direct', 'indirect', 'volumetrics', 'depth', 'dice', 'lighting', 'denoise hints'];

  // scripted stepping for automated testing: __runFrames(n, dt, keys[]) runs n fixed steps with the given keys held
  let scripted = false;
  window.__runFrames = (n: number, dtFixed = 1 / 60, keys: string[] = []) => { scripted = true; for (const k of keys) input.keys.add(k); for (let i = 0; i < n; i++) step(dtFixed, i === n - 1); for (const k of keys) input.keys.delete(k); scripted = false; };
  window.__mouse = (x: number, y: number, buttons?: number) => { input.mouseX = x; input.mouseY = y; if (buttons !== undefined) input.buttons = buttons; };   // buttons: 1 LMB, 4 RMB (held)
  window.__click = (button = 0) => { input.clicked |= 1 << button; };
  window.__input = input;

  // __shot(name): render one offscreen frame, read it back and POST a JPEG to the capture server (tools/shot-server.ts) — for docs/board images
  window.__shot = async (name: string, quality = 0.86, scale = 1) => {
    engine.render(cam, timeAcc, 1 / 60, false);
    const c2 = await frameCanvas(engine); if (!c2) return 'no frame';
    let out: HTMLCanvasElement = c2;
    if (scale !== 1) { out = document.createElement('canvas'); out.width = Math.round(c2.width * scale); out.height = Math.round(c2.height * scale); const cx = out.getContext('2d')!; cx.imageSmoothingQuality = 'high'; cx.drawImage(c2, 0, 0, out.width, out.height); }
    const blob: Blob = await new Promise(res => out.toBlob(b => res(b!), 'image/jpeg', quality));
    return `${c2.width}x${c2.height} ${(blob.size / 1024).toFixed(0)} KB → ${await postShot(name, blob)}`;
  };
  // __png(name): like __shot but lossless PNG (for pixel-exact A/B of renderer changes; pair with __pauseLoop for determinism)
  window.__png = async (name: string) => {
    engine.render(cam, timeAcc, 1 / 60, false);
    const c2 = await frameCanvas(engine); if (!c2) return 'no frame';
    const blob: Blob = await new Promise(res => c2.toBlob(b => res(b!), 'image/png'));
    return `${c2.width}x${c2.height} png ${(blob.size / 1024).toFixed(0)} KB → ${await postShot(name, blob)}`;
  };
  // __resetClock(): zero every per-frame counter that feeds jitter / seeds / flicker, snap the camera — so a scripted capture sequence
  // renders the same frames whatever ran during boot (pair with __pauseLoop)
  window.__resetClock = () => { engine.frameIdx = 0; engine.passes.resetClocks(); cam.distance = cam.desiredDistance; cam.target = v3.copy(game.followPos()); };
  if (new URLSearchParams(location.search).has('paused')) window.__pauseLoop = true;   // ?paused: nothing runs until a script steps it (deterministic captures from frame 0)
  window.__timeScale = 1;
  if (new URLSearchParams(location.search).has('demo')) setTimeout(() => { demo.start(); }, 1500);
  if (new URLSearchParams(location.search).has('hud')) readout.set(true);
  // attract mode: a demo link that gets opened and left on the start card starts showing off by itself after a while (arcade-style); any key,
  // click or mouse movement puts the clock back, and it only ever fires from the start card (never from pause / debrief / play)
  { let idle = 0, last = performance.now(); const reset = () => { idle = 0; }; for (const ev of ['keydown', 'mousedown', 'mousemove', 'wheel', 'touchstart']) window.addEventListener(ev, reset, { capture: true, passive: true });
    setInterval(() => { const now = performance.now(); const dt = Math.min(1.5, (now - last) / 1000); last = now; if (menu.mode !== 'start' || demo.active || (document.hidden && !(window as any).__testVisible) || (window as any).__pauseLoop) { idle = 0; return; } idle += dt; if (idle > 25) { idle = 0; try { if (!demo.active) demo.start(); menu.close(); } catch (e) { console.error('attract mode:', e); } } }, 500); }   // (tour first, then the card: no live frame in between for a guard to find Sam standing at the insertion point)
  function frame() {
    try {
      if (window.__pauseLoop) { last = performance.now(); requestAnimationFrame(frame); return; }   // test hook: only scripted __runFrames advance anything (deterministic captures)
      const now = performance.now(); const rawDt = (now - last) / 1000; const dt = Math.min(1 / 20, rawDt) * (window.__timeScale ?? 1); last = now;
      if (rawDt > 0) fpsSm = fpsSm * 0.95 + (1 / rawDt) * 0.05;   // HUD frame rate from wall-clock, not the clamped / time-scaled game dt
      if (quality.calibrating) quality.calibrateTick(rawDt);                            // boot calibration reads GPU pass timings behind the start card (the live scene renders there)
      else if (!(document.hidden && !window.__testVisible) && rawDt < 0.5 && !menu.isOpen) quality.sample(rawDt);   // a frozen pause frame says nothing about gameplay load
      step(dt);
      requestAnimationFrame(frame);
    } catch (e) { console.error(e); fatalOnce('The frame loop threw and has been stopped:', e); }   // no re-arm: stepping on with broken state would only bury this message under a cascade (scripted __runFrames still works for poking at it)
  }
  function step(dt: number, lastOfBurst = true) {
    timeAcc += dt;
    const frozen = menu.isOpen && !scripted;   // paused / start card: the world holds still, the picture stays live behind the card, hotkeys are off
    if (frozen) { dt = 0; input.endFrame(); }
    if (!frozen && input.wheelCtrl) cam.zoom(input.wheelCtrl);                 // Ctrl + wheel (or a pinch) zooms; the plain wheel is Sam's walking pace (player.ts)
    if (!frozen && input.orbitDX) cam.rotate(input.orbitDX * 0.006);          // hold Ctrl and move the mouse to orbit (Q / E are the wall hold and the holster now)
    if (!frozen && input.hit('Backquote')) engine.settings.debugView = (engine.settings.debugView + (input.down('ShiftLeft') ? debugNames.length - 1 : 1)) % debugNames.length;
    if (!frozen && input.hit('BracketLeft')) engine.settings.exposure /= 1.25;
    if (!frozen && input.hit('BracketRight')) engine.settings.exposure *= 1.25;
    if (!frozen && input.hit('Tab')) panel.toggle();
    if (!frozen && input.hit('KeyP')) { if (demo.active) demo.stop(); else demo.start(); }
    if (!frozen && demo.active) { if (input.hit('ArrowRight')) demo.jump(+1); if (input.hit('ArrowLeft')) demo.jump(-1); }   // tour: arrows step between stops (the timeline at the bottom is clickable too)

    if (canvas.clientWidth > 0 && canvas.clientHeight > 0) {
      const demoPos = frozen ? null : demo.update(dt);   // before the game step: the tour's scripted beats press keys / move the puppet's aim for THIS frame's update
      if (!frozen) game.update(dt, input, cam, canvas);
      if (game.mission.debrief) { game.mission.debrief = false; menu.open('debrief'); }   // out with the drive: the end card (freezes the world like the pause menu; Esc / Keep playing resume, Again restarts)
      engine.settings.nvAmount = game.player.nvAmount; engine.settings.hitFlash = game.player.hitFlash;
      const lp = game.player.char.pos; audio.setListener([lp[0], lp[1] + 1.6, lp[2]], cam.forward); audio.update(dt, timeAcc);
      if (audio.music && !window.__forcedTension) audio.music.intensity = game.tension();
      smoke.update(dt);
      { const spect = !demoPos && game.spectate >= 0; if (spect) cam.autoYaw(dt, game.followPos(), (a, b) => game.col.segmentBlocked(a, b)); cam.easeYaw(dt, spect);   // spectating a guard: come round to the side he can be seen from,
        const lowP = 24 * Math.PI / 180, highP = 55 * Math.PI / 180; const want = spect && cam.desiredDistance < 12 ? lowP : highP; cam.pitch += (want - cam.pitch) * Math.min(1, dt * 3); }   // and drop to a low three-quarter angle up close (from the usual height the ceiling panels sit between the eye and a man six metres off)
      cam.update(dt, demoPos ?? game.followPos(), demoPos || game.spectate >= 0 ? null : game.aimPoint);
      overlay.setCaption(demo.active ? demo.caption : '', demo.active ? demo.sub : '');
      overlay.setTimeline(demo.active ? demo.chapters() : null, k => demo.goto(k));
      engine.lights.update(dt, timeAcc);
      engine.render(cam, timeAcc, dt, !scripted || (lastOfBurst && !document.hidden));
      overlay.update(game, cam, canvas); bubbles.update(game, cam, canvas);
    }
    input.endFrame();

    frameCount++;
    if (frameCount % 10 === 0) {
      const t = engine.timer; const lines: string[] = [];
      // one stat per line (a narrow column down the left edge reads better than three long rows across the picture)
      const row = (k: string, v: string) => k.padEnd(11) + v;
      lines.push(gpu.adapterInfo);
      lines.push(row('res', `${engine.width}×${engine.height} ×${engine.effectiveScale.toFixed(2)}${engine.effectiveScale < engine.settings.renderScale - 0.005 ? ' cap' : ''}`));
      lines.push(row('quality', `${quality.label}${quality.adaptive ? ' +adapt' : ''}`));
      lines.push(row('fps', `${fpsSm.toFixed(0)}`));
      lines.push(row('gpu ms', `${t.total(['composite', 'post', 'fxaa', 'gbuffer']).toFixed(2)}`));
      const order = ['smoke', 'rc c4', 'rc c3', 'rc c2', 'rc c1', 'rc c0', 'rc dice', 'direct', 'dtemporal', 'softblur', 'gather', 'temporal', 'volumetrics', 'bloom', 'probe'];   // compute passes only: render-pass timestamps (gbuffer / composite / post / fxaa) on Metal span the wait for the drawable and read as ~one frame each, so they are left out rather than mislead
      for (const k of order) if (t.results.has(k)) lines.push(row('  ' + k, t.results.get(k)!.toFixed(2)));
      lines.push(row('boxes', `${engine.world.all.length} (dyn ${engine.world.dynamics.length})`));
      lines.push(row('lights', `${engine.lights.activeCount}`));
      lines.push(row('smoke', `${smoke.stats.live} dom · ${smoke.stats.emitters} em · ${smoke.stats.steps} disp${smoke.stats.bricksTotal ? ` · ${Math.round(100 * smoke.stats.bricksActive / smoke.stats.bricksTotal)}% bricks` : ''}`));
      lines.push(row('rc rays', `${(engine.rc.stats.intervals / 1e6).toFixed(2)}M`));
      lines.push(row('view', `${debugNames[engine.settings.debugView]} · exp ${engine.settings.exposure.toFixed(2)}`));
      lines.push(row('player', `vis ${(game.player.visibility * 100) | 0}% E=${game.player.irradiance.toFixed(3)}`));
      game.guards.forEach((gd, i) => lines.push(row(`guard ${i}`, `${gd.state} ${(gd.awareness * 100) | 0}`)));
      lines.push(row('AI', game.aiEnabled ? 'on' : 'off'));
      hud.textContent = demo.active || !readout.on ? '' : lines.join('\n');   // the showcase keeps the screen clean; in play the read-out is opt-in (Tab ▸ Frame ▸ debug read-out, or ?hud) now that the kit and the interaction panel live on the left
    }
    tourVeil.style.opacity = demo.active ? String(demo.fade) : '0'; document.body.classList.toggle('touring', demo.active);
  }
  requestAnimationFrame(frame);
}

// Runtime error surface. Boot failures already land on the #fatal card (main().catch below, device.lost in gpu/device.ts); this catches what
// happens afterwards — a throw inside the frame loop (frame()'s try/catch, which also stops the loop), or an uncaught exception / rejected
// promise from any other callback — and puts the FIRST one on the same card instead of leaving a silently frozen picture. Later ones, and
// anything arriving while a card is already up, only reach the console; the browser's ResizeObserver loop notices are not errors at all.
let crashed = false;
function fatalOnce(head: string, err: unknown) {
  if (crashed) return; crashed = true;
  const card = document.getElementById('fatal'); if (card && card.style.display === 'flex') return;   // showFatal already has something up (device loss…): that is the message that matters
  showFatal(`${head}\n\n${err instanceof Error ? (err.stack || String(err)) : String(err)}`);
}
window.addEventListener('error', ev => { if (/ResizeObserver loop/.test(ev.message)) return; fatalOnce('Uncaught error:', ev.error ?? ev.message); });
window.addEventListener('unhandledrejection', ev => fatalOnce('Unhandled promise rejection:', ev.reason));

main().catch(e => { console.error(e); showFatal(String(e?.stack ?? e)); });
