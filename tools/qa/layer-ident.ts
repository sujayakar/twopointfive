// THE ANIMATOR'S LAYERS IN RUST, ASKED BOTH WAYS (rust/sim/src/layers.rs, the row RUST.walk; docs/agents/rust.md "A layer of the animator") — a scripted
// mission is played on the server's stand with the poses at home and the row up, and EVERY call of a layer that has moved is run twice on the same pose from
// the same start, once by the module and once by the TypeScript method (anim/skeleton.ts layerBoth), and the two poses compared single for single, and the run-forward bones and the state each leaves double for double. So the
// questions are the ones a game asks, with the numbers a game has. With it: the two lists Rust code reads by name (rust/sim/src/knobs.rs: the knobs, the
// rig's named nodes) are held to the game's own tables.
//   bun run tools/qa/layer-ident.ts [--run warehouse-four:1]   exit 0 = every line passes · 1 = one failed (named)
//   RUST_BREAK=1 bun run tools/qa/layer-ident.ts               the red path: every knob is handed to the module a hundredth large, and so is the first of every
//                                                              layer's own numbers (a layer that reads no knob has a break of its own) — `both` and `changed` must FAIL
// WHAT IS HELD:
//   list      every path of the knob list is a number of a registered knob table or of the animator's tables (a path that is not is an error as the list is first read)
//   read      every name of the three lists (knobs, named nodes, lists of nodes) is read by some line of the crate outside knobs.rs (a line nobody reads is rot)
//   both      every layer in LAYERS_MOVED was asked at least FLOOR times in the run and the quiet walk after it (Sam, hands empty, round a standing guard: the
//             layers of quiet play, which no scripted mission reaches), and no answer of any layer differs
//   changed   a knob changed between two ticks is the module's by the next tick's opening, and back again; one changed outside a tick is the module's at the
//             next pose made there (a teleport's)
// Not game code; nothing here needs a GPU, and nothing here has been seen on one.
import { resolve } from 'node:path';
import { readFileSync, readdirSync } from 'node:fs';
const ROOT = resolve(import.meta.dir, '../..');
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.TPS_RUSTPOSES = '1'; process.env.TPS_RUSTWALK = '1';
const BREAK = process.env.RUST_BREAK === '1', args = process.argv.slice(2), flag = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
/** the layers that have moved (rust/sim/src/layers.rs, a `pub fn` each), by the name game/character.ts asks them both ways under */
const LAYERS_MOVED = ['standPose', 'crouchPose', 'softKnees', 'runPlant', 'steadyLayer', 'sneakBody', 'creepBody', 'watchSlip', 'watchLook', 'mvPostBody', 'kickBody', 'toesUp', 'wallBody', 'stanceBody', 'mvPre', 'mvTrunk', 'longBody', 'lifeLayer', 'leanBody', 'gazeKeep', 'turnWeigh', 'sneakArms', 'carryArms', 'wallArms', 'actPose', 'woundBody', 'woundLimbs', 'longArms', 'mvPostArms', 'watchArm', 'creepArms', 'clutchArm', 'boundArms', 'reloadArm', 'pistolReloadArm'], FLOOR = 100;
const { standUp } = await import('./headless.ts');
const S: any = await import(`${ROOT}/src/sim/sim.ts`), SK: any = await import(`${ROOT}/src/anim/skeleton.ts`), RK: any = await import(`${ROOT}/src/game/rustknobs.ts`);
const KN: any = await import(`${ROOT}/src/game/knobs.ts`), PT: any = await import(`${ROOT}/tools/qa/playthrough.ts`);
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
console.log(`the animator's layers in Rust · profile ${process.env.TPS_PROFILE}${BREAK ? ' · RUST_BREAK=1 (the red path)' : ''}`);
const no = S.simWhyNot();
if (no) { console.log(`- FAIL — the module does not start here: ${no}`); process.exit(1); }
if (BREAK) RK.RUST_KNOBS.hand = (_i: number, v: number) => v * 1.01;
// (…and the first of a layer's own numbers: the module's exports as the animator calls them — POSE_BENCH.x, set the first time the bench is used — are
//  stood in for by a copy whose `layer_…` functions make the bench's first argument a hundredth large before they call the module's)
if (BREAK) { const B = SK.POSE_BENCH; let real: any = B.x, bent: any = null;
  const bend = (x: any) => x && Object.fromEntries(Object.entries(x).map(([k, f]) => [k, k.startsWith('layer_') ? (...a: number[]) => { S.f64()[B.args] *= 1.01; return (f as (...a: number[]) => number)(...a); } : f]));
  bent = bend(real); Object.defineProperty(B, 'x', { get: () => bent, set: (v: any) => { real = v; bent = bend(v); }, configurable: true }); }

// ---- list
let lines: { path: string; value: number | null }[] = [], listErr = '';
try { lines = RK.rustKnobLines(); } catch (e) { listErr = String((e as Error).message); }
verdict(!listErr && lines.length > 0 && lines.every(l => typeof l.value === 'number'), 'list: every knob Rust code reads is a number of a registered knob table',
  listErr || `${lines.length} knobs: ${lines.map(l => `${l.path} ${l.value}`).join(' · ')} · ${RK.rustList(3).length} named nodes: ${RK.rustList(3).join(' ')}`);

// ---- read
{ const dir = `${ROOT}/rust/sim/src`, src = readFileSync(`${dir}/knobs.rs`, 'utf8'), names = [...src.matchAll(/^\s+([A-Z][A-Z0-9_]*) = "([^"]+)",$/gm)].map(m => m[1]);
  const others = readdirSync(dir).filter(f => f.endsWith('.rs') && f !== 'knobs.rs').map(f => readFileSync(`${dir}/${f}`, 'utf8')).join('\n');
  const unread = names.filter(n => !new RegExp(`\\b${n}\\b`).test(others));
  verdict(names.length === lines.length + RK.rustList(3).length + RK.rustList(5).length + RK.rustList(8).length && unread.length === 0, 'read: every knob, every named node, every list of nodes and every table of the rig\'s numbers of the lists is read by a line of the crate',
    unread.length ? `NOBODY READS ${unread.join(', ')}` : `${names.length} names in rust/sim/src/knobs.rs, the module lists ${lines.length} + ${RK.rustList(3).length} + ${RK.rustList(5).length} + ${RK.rustList(8).length}, each read outside it`); }

// ---- the quiet walk: no scripted mission sneaks, crouches behind a man or closes in on one, so the layers of Sam's quiet play would never be asked. Sam, hands
// empty and in the dark, is put a couple of metres behind a guard who stands where he is, and walks and crouch-walks round him for `secs`, set down on a new
// bearing every second: the close-in mode comes and goes with the distance, the crouch with the key. The carriage's arms have three ways to be: bare hands
// let out wide and low (the first half), a knife at his chest that the right hand goes up to hover by (the third quarter), and the carriage as it was before
// the distance was dealt (the last): the switches that make each are the game's own, set as a stand sets them.
async function quietWalk(secs: number): Promise<void> {
  const W: any = await standUp({ seed: 11 }), g = W.game, DT = 1 / 60, CL: any = await import(`${ROOT}/src/game/close.ts`), KA: any = await import(`${ROOT}/src/game/kitanim.ts`);
  const was = CL.CLOSE.on, wasBare = CL.CLOSE.noKnife, wasClose = KA.KITANIM.close; CL.CLOSE.on = true; CL.CLOSE.noKnife = true;
  try {
    g.godMode = true; g.player.slot = 'pistol'; g.player.holstered = true;
    for (let i = 0; i < 5; i++) W.step(DT);
    const A = g.guards[0], home = [...g.player.char.pos]; A.hold = true;
    for (let f = 0; f < secs * 60; f++) {
      if (f === secs * 30) CL.CLOSE.noKnife = false; if (f === secs * 45) KA.KITANIM.close = false;
      if (f % 60 === 0) { const a = A.char.bodyYaw + Math.PI + (f / 60) * 0.9, d = 1.2 + ((f / 60) % 4) * 0.9; g.teleportPlayer([A.char.pos[0] + Math.sin(a) * d, 0, A.char.pos[2] + Math.cos(a) * d]); g.player.crouch = (f / 60) % 3 !== 0; }
      if (f % 60 < 40) W.input.keys.add(f % 120 < 60 ? 'KeyW' : 'KeyA'); else { W.input.keys.delete('KeyW'); W.input.keys.delete('KeyA'); }
      g.player.visibility = 0.02; W.step(DT);
    }
    CL.CLOSE.noKnife = wasBare; KA.KITANIM.close = wasClose;
    // …and four kicks at the air, since a mission's doors are too few to count on: the right leg and the left (the same kick in a mirror), standing, off a walk
    // and off a crouch (the planted foot found off the idle's spot, and set down on it)
    W.input.keys.delete('KeyW'); W.input.keys.delete('KeyA');
    for (let k = 0; k < 4; k++) {
      g.player.crouch = k === 3; if (k === 2) W.input.keys.add('KeyW');
      for (let f = 0; f < 30; f++) W.step(DT);
      g.player.char.anim.kickDoor({ leg: k % 2 ? 'left' : 'right' }); W.input.keys.delete('KeyW');
      for (let f = 0; f < 80; f++) W.step(DT);
    }
    // …and the reactions, which a mission deals too few of to count on. Sam and the guard he stands behind are each, a second apart, ducked (hard, by half, soft),
    // started and hit from the front, a side and behind, blinded, staggered, hurt, shaken, and then all of it at once; Sam tosses, and walks while he is hurt
    // (the short heavy step). The game's own state is what is set, so each reaches the figure the way a mission deals it.
    g.player.crouch = false;
    const dirs: [number, number, number][] = [[0, 0, 1], [1, 0, 0.2], [-0.3, 0, -1]];
    for (let f = 0; f < 15 * 60; f++) {
      const s = Math.floor(f / 60), d = dirs[s % 3];
      for (const m of [g.player, A]) {
        const an = m.char.anim;
        if (f % 60 === 0) {
          if (s < 3) an.flinchFrom(d, s === 1 ? 0.5 : 1, s === 2);
          else if (s < 6) an.startleFrom(d);
          else if (s < 9) an.hitFrom(d);
          else if (s === 9) { m.dazzledUntil = g.time + 0.8; m.dazzledAt = [m.char.pos[0] + d[0], 0, m.char.pos[2] + d[2]]; }
          else if (s === 10) m.staggerUntil = g.time + 0.8;
          else if (s === 11) m.hurtAt = g.time;
          else if (s === 13) an.tossItem();
          else if (s === 14) { m.dazzledUntil = g.time + 0.5; m.staggerUntil = g.time + 0.9; an.flinchFrom(d); an.startleFrom(d); an.hitFrom(d); }
        }
        if (s === 12 || s === 14) m.sup.v = 0.7;
      }
      if (s >= 11 && f % 60 < 40) W.input.keys.add('KeyW'); else W.input.keys.delete('KeyW');
      W.step(DT);
    }
    // …and a lean round an edge, which only a teammate at a threshold is dealt: Sam is leaned right, half left, let up and leaned again on the move (nothing
    // deals the player a lean, so the animator's own input is what is set)
    W.input.keys.delete('KeyW');
    for (let f = 0; f < 5 * 60; f++) {
      const s = Math.floor(f / 60); g.player.char.anim.lean = [1, -0.5, 0, 1, 0][s];
      if (s === 3) W.input.keys.add('KeyW'); else W.input.keys.delete('KeyW');
      W.step(DT);
    }
    // …and the wounded man's pose, which a mission deals only by a wounding round: the guard is laid down wounded through the animator's own input, on a knee
    // with the hand to his ribs, let up, then on his back with the hand to his thigh, and last as a save restores him (settled, his clock given)
    W.input.keys.delete('KeyW');
    for (let f = 0; f < 12 * 60; f++) {
      const s = Math.floor(f / 60);
      A.char.anim.wound = s < 5 ? { pose: 'knee', at: 'ribs' } : s < 6 ? null : s < 10 ? { pose: 'back', at: 'thigh' } : s < 11 ? null : { pose: 'knee', at: 'thigh', settled: true, clock: 3 };
      W.step(DT);
    }
    A.char.anim.wound = null;
    // …and a man kept down, which only a hostage or a man told to stay is dealt (civilians.ts feeds it): Sam, his hands empty, has his wrists put behind his
    // back, is let go, puts his hands on his head and is let go again, through the animator's own input (set down where the mission began: the walk left
    // him with his back to a wall, and the wall hold has the whole of his arms)
    g.teleportPlayer(home); for (let f = 0; f < 60; f++) W.step(DT);
    for (let f = 0; f < 9 * 60; f++) { const s = f / 60; g.player.char.anim.bound = s < 3 ? 1 : s < 4.5 ? 0 : s < 8 ? 2 : 0; W.step(DT); }
    g.player.char.anim.bound = 0;
    // …and the magazine changes, which a mission's fights are too short to count on. Sam changes his rifle's magazine with rounds left in it and with none
    // (a round still chambered, then none: the handle run), then his pistol's the same three ways. The guard, an untrained man, is dealt each long gun in turn off his route (the row the game arms
    // him by every frame) and begins his beat on it: a box under the carbine, the MP7's up the grip, the belt gun's cover and box, the tube guns a shell
    // at a time. His beat is counted down by the fight he is in; standing here he is in none, so the drive counts it with the game's own tick.
    { const pl = g.player, GD: any = await import(`${ROOT}/src/game/guards.ts`);
      for (const slot of ['rifle', 'pistol']) for (const [mag, chamber] of [[5, 1], [0, 1], [0, 0]]) {
        const gun = pl[slot]; pl.slot = slot; pl.holstered = false; for (let f = 0; f < 90; f++) W.step(DT);
        gun.cancelReload(); gun.mag = mag; gun.chamber = chamber; gun.spare = [gun.def.magCapacity, gun.def.magCapacity];
        W.input.pressed.add('KeyR'); for (let f = 0; f < 270; f++) W.step(DT);
      }
      pl.slot = 'pistol'; pl.holstered = true;
      const own = A.route.weapon;
      for (const id of ['aks74u', 'mk12', 'mp7', 'mk46', 'm1014', 'm870']) {
        A.route.weapon = id; for (let f = 0; f < 45; f++) W.step(DT);
        GD.startReloadBeat(g, A);
        for (let f = 0; f < 900 && A.reloadT >= 0; f++) { const was = A.reloadT; W.step(DT); if (A.reloadT === was) GD.tickReloadBeat(g, A, DT); }
        for (let f = 0; f < 45; f++) W.step(DT);
      }
      A.route.weapon = own; }
  } finally { CL.CLOSE.on = was; CL.CLOSE.noKnife = wasBare; KA.KITANIM.close = wasClose; }
}

// ---- the wall hold: no scripted mission puts a back to a wall either. The wall-pose probe's own two drives are run for their calls alone (its verdicts are
// its own check's, and are not printed here): the hold taken standing and crouched, parked, sidled both ways, flipped, peeked and left, with the mend
// (WALLSNAP.pose: the hold's own step) on and off.
async function wallHold(): Promise<void> {
  const WP: any = await import('./wall-pose-probes.ts'), log = console.log;
  console.log = () => {};
  try { await WP.wallPose(); await WP.wallPoseMend(); } finally { console.log = log; }
}

// ---- both
{ const [scn, seed] = (flag('--run') ?? 'warehouse-four:1').split(':'); SK.LAYERS.both = true;
  const t0 = performance.now(), r = await PT.play(scn, Number(seed), 300, false, {}); await quietWalk(80); await wallHold(); SK.LAYERS.both = false;
  const by: Record<string, { asked: number; differ: number }> = SK.LAYERS.by, thin = LAYERS_MOVED.filter(n => (by[n]?.asked ?? 0) < FLOOR), off = Object.keys(by).filter(n => by[n].differ > 0), stray = Object.keys(by).filter(n => !LAYERS_MOVED.includes(n));
  console.log('LAYER-IDENT ' + JSON.stringify({ by }));   // (what tools/qa/rust-check.ts reads: under RUST_BREAK=1 every layer's own answers must have differed)
  verdict(thin.length === 0 && off.length === 0 && stray.length === 0, 'both: every call of a moved layer in a mission, run by the module and by the TypeScript on the same pose, leaves the same pose single for single',
    `${scn}:${seed} ends ${String(r.end)}, then a quiet walk of 80 s, four kicks, the reactions, a lean, the wounds, a man kept down, the magazine changes and the wall hold's drives · ${LAYERS_MOVED.map(n => `${n} ${by[n]?.asked ?? 0} asked, ${by[n]?.differ ?? 0} differ`).join(' · ')}${thin.length ? ` · ASKED TOO LITTLE: ${thin.join(', ')}` : ''}${stray.length ? ` · asked and not in LAYERS_MOVED: ${stray.join(', ')}` : ''}${SK.LAYERS.first ? ` · first: ${SK.LAYERS.first}` : ''} · ${((performance.now() - t0) / 1000).toFixed(1)} s`); }

// ---- changed
{ const W: any = await standUp({ seed: 7 }), DT = 1 / 60, at = S.sim().knob_addr(0) / 8, i = lines.findIndex(l => l.path === 'MV.gait.lean'), MV = KN.knobRegistry().MV;
  const was = MV.gait.lean, mod = () => S.f64()[at + i];
  W.step(DT); const a = mod() === was; MV.gait.lean = was + 1.25; const stale = mod() === was; W.step(DT); const b = mod() === was + 1.25; MV.gait.lean = was; W.step(DT); const c = mod() === was;
  // …and outside a tick: a pose made there (a teleport's re-pose) reads the table as it stands too
  MV.gait.lean = was + 2.5; W.game.teleportPlayer([...W.game.player.char.pos]); const out = mod() === was + 2.5; MV.gait.lean = was; W.step(DT); const e = mod() === was;
  verdict(i >= 0 && a && stale && b && c && out && e, 'changed: a knob changed between two ticks is the module\'s as the next tick opens, and at a pose made outside a tick',
    `MV.gait.lean ${was}: the module's after a tick ${a} · the table changed, the module's not yet ${stale} · after the next tick ${b} · put back, and a tick ${c} · changed and a teleport, no tick ${out} · put back, and a tick ${e}`); }

console.log(fails ? `=== layer-ident: ${fails} FAILED ===` : '=== layer-ident: PASS — every layer the same pose both ways, every knob the table\'s ===');
process.exit(fails ? 1 : 0);
