// THE MEN'S BREATH STEAMS ON A SNOW LEVEL, AND IS THE PICTURE'S ALONE (src/game/weatherKinds.ts SNOW_LOOK.breath, src/game/snowBreath.ts SNOWBREATH /
// updateSnowBreath; the particle system's dust puff with one byte of its colour word spent: render/particles.ts EmitOpts.steam, the shaders' steamIs / steamOf).
//
// The maintainer, 10-14: "we should have steam from breaths in the cold levels too" — and of the snow as a whole, "it does need to feel more organic".
//
//   bun run tools/qa/snowbreath-check.ts [--verbose] [--fast]      exit 0 = all pass, 1 = a FAIL line   (--fast: without the played runs)
//   SNOWBREATH_BREAK=1 …   = the switch never raised and the same lines asked: must fail
//   bun run tools/qa/probes.ts snowbreath      (also under `snow` and `weather`)
//   Run it on BOTH stands (TPS_PROFILE=shipped in front for the page's): it puts the switch where each line needs it by the stand's own word, TPS_SNOWBREATH.
//
// The harness has no GPU: a puff is thrown, moved, lit and drawn on the GPU, so what is proven HERE is what the CPU files and what the shader text says of it —
// never a breath seen.
//   runs      the pass's room clear, a squad of four, switch on: every living man on his feet under the open sky breathes — dust records, a cone from his own
//             mouth, his id as their owner, a steam share under 1, seeds of their own — two to four breaths each in ten seconds
//   rhythm    … and not together: the men's first breaths start on different frames
//   effort    a man made to run breathes more often, and bigger, than the same man standing
import { standUp, ROOT } from './headless.ts';
import { readFileSync, existsSync, mkdtempSync, rmSync } from 'node:fs';
import { resolve } from 'node:path';
import { tmpdir } from 'node:os';

const DT = 1 / 60;

export async function snowBreathCheck(o: { verbose?: boolean; fast?: boolean } = {}): Promise<boolean> {
  const WK = await import(`${ROOT}/src/game/weatherKinds.ts`);
  const { SNOW, SNOW_LOOK } = WK;
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { updateSnowBreath, breathStats, seedBreath, SNOWBREATH } = await import(`${ROOT}/src/game/snowBreath.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
  const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
  const J = await import(`${ROOT}/src/game/journal.ts`);
  const BREAK = process.env.SNOWBREATH_BREAK === '1';
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (o.verbose) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  let fails = 0, checks = 0;
  const verdict = (ok: boolean, label: string, detail: string) => { checks++; if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const was = { snow: SNOW.on, breath: SNOW_LOOK.breath, envSnow: process.env.TPS_SNOW, envBreath: process.env.TPS_SNOWBREATH };

  /** a game stood up on `file` in snow, the breath's switch where `on` says (by the stand's own word, so the frozen stand's pin and the shipped stand's boot both
   *  obey); on the pass, its room clear begun with a squad of four */
  async function rig(file: string, on: boolean) {
    const sw = on && !BREAK;
    process.env.TPS_SNOW = '1'; process.env.TPS_SNOWBREATH = sw ? '1' : '0'; SNOW_LOOK.breath = !sw;   // (the module's switch left the WRONG way round: only the stand's word can put it right)
    const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
    const W: any = await quiet(() => standUp({ level: lvl, seed: 7 }));
    const g = W.game; const stood = SNOW_LOOK.breath; SNOW_LOOK.breath = sw;
    const why = file === 'pass.json' ? await quiet(() => startFromQuery(g, '?levelFile=pass.json&roomclear=0&squad4').why) : null;
    const step = (n = 1) => { for (let i = 0; i < n; i++) { SNOW_LOOK.breath = sw; W.step(DT); } };
    return { W, g, step, stood, sw, why };
  }
  /** a recording stand-in for the particle pool (the harness's engine has none: no GPU) */
  const mk = () => { const recs: any[] = []; let frame = 0; const P: any = { emit(kind: string, pos: number[], dir: number[], count: number, opts: any) { recs.push({ kind, pos: [...pos], dir: [...dir], count, opts: { ...opts }, frame }); return count; }, tick() { frame++; } }; return { P, recs }; };
  /** the emitter run `frames` frames against a fresh pool, reseeded — the pool on the engine only for the emitter's own call (the rest of the game has no pool to
   *  file into on this harness, as ever). `play`: the GAME is stepped a frame before each (its own poolless pass over the emitter runs the rhythm on a frame too:
   *  the men breathe twice as fast in that line, and nothing else of it changes); else the game stands still. `pre` = what the line holds the men to, each frame */
  const fly = (R: any, frames: number, f: { play?: boolean; pre?: () => void } = {}) => {
    const { P, recs } = mk(); const e = R.g.engine as any; const had = { p: e.particles, gp: e.settings.gpuParticles };
    seedBreath(R.g, 1);
    for (let i = 0; i < frames; i++) {
      if (f.play) R.step(1);
      f.pre?.();
      e.particles = P; e.settings.gpuParticles = true; SNOW_LOOK.breath = R.sw;
      updateSnowBreath(R.g, DT); P.tick();
      e.particles = had.p; e.settings.gpuParticles = had.gp;
    }
    const S = breathStats(R.g); const by = (id: number) => recs.filter(r => r.opts.owner === id);
    return { recs, by, S: { ...S, by: { ...S.by } } };
  };
  const key = (r: any) => `${r.frame}:${r.opts.owner}:${r.opts.seed}:${r.count}:${r.pos.map((v: number) => v.toFixed(4)).join(',')}:${r.opts.steam}:${r.opts.size}:${r.opts.speed.toFixed(4)}`;
  /** breaths = runs of a man's records (a breath's records come `out` ÷ `puffs` apart; breaths seconds apart) */
  const breathsOf = (rs: any[]) => { let n = 0, last = -1e9; for (const r of rs) { if (r.frame - last > 30) n++; last = r.frame; } return n; };

  try {
    const A = await rig('pass.json', true); A.step(120);
    const g = A.g; const K = SNOWBREATH;
    const ops: any[] = g.guards.filter((gd: any) => gd.team === 'operative'), foes: any[] = g.guards.filter((gd: any) => gd.team !== 'operative');
    const open = (c: any) => !!c.bones.head && !g.weather.underRoof(c.bones.head[0], c.bones.head[2]);
    const out: any[] = [g.player.char, ...ops.map(gd => gd.char)].filter(open);   // (the men this check expects to steam: Sam and his four, at the gate under the sky)
    const outIds = new Set(out.map(c => c.id));
    // ---- runs
    const on = fly(A, 600);
    { const per = out.map(c => ({ id: c.id, recs: on.by(c.id), n: breathsOf(on.by(c.id)) }));
      const shape = on.recs.every(r => r.kind === 'dust' && r.opts.shape === 'cone' && r.count === K.count && r.opts.steam > 0 && r.opts.steam < 1 && outIds.has(r.opts.owner) && r.opts.life === K.life && r.dir[1] < 0.3 && Math.abs(Math.hypot(r.dir[0], r.dir[1], r.dir[2]) - 1) < 1e-6);
      const mouth = on.recs.every(r => { const h = out.find(c => c.id === r.opts.owner)?.bones.head; return !!h && Math.hypot(r.pos[0] - h[0], r.pos[1] - h[1], r.pos[2] - h[2]) < 0.2 && r.pos[1] < h[1]; });
      const own = new Set(on.recs.map(r => r.opts.seed)).size === on.recs.length;
      const each = per.every(p => p.n >= 2 && p.n <= 4 && p.recs.length >= p.n * K.puffs - (K.puffs - 1) && p.recs.length <= p.n * K.puffs);
      verdict(A.stood === true && !A.why && out.length >= 5 && on.S.active && on.S.shown === out.length && on.recs.length > 0 && shape && mouth && own && each && on.S.records === on.recs.length,
        'runs — on the pass every living man under the open sky breathes steam', `the stand's word raised the switch before the Game was built: ${A.stood} · ${on.S.why}: ${on.S.shown} of ${on.S.men} men show (${out.length} stand under the sky: Sam and ${ops.filter(gd => open(gd.char)).length} of his) · ${on.recs.length} records in 600 frames, ${on.S.births} puffs · breaths each in ten seconds: ${per.map(p => `#${p.id} ${p.n} (${p.recs.length} records)`).join(', ')} — two to four, ${K.puffs} records a breath: ${each} · every one a dust cone of ${K.count}, a steam share ${on.recs[0]?.opts.steam ?? '—'}, its owner one of them: ${shape} · filed inside 0.2 m of his head bone and under it: ${mouth} · seeds their own: ${own}`); }
    // ---- rhythm
    { const first = out.map(c => on.by(c.id)[0]?.frame ?? -1); const distinct = new Set(first).size; const gap = Math.max(...first) - Math.min(...first);
      verdict(distinct >= out.length - 1 && gap > 30 && first.every(f => f >= 0), 'rhythm — no two start together', `the frame each man's first breath starts on: ${first.join(', ')} (${distinct} different, ${(gap * DT).toFixed(1)} s between the first and the last)`); }
    // ---- effort
    { const m = ops[0].char; const v0 = [...m.vel];
      const run = fly(A, 600, { pre: () => { m.vel[0] = 4.5; m.vel[2] = 0; } }); m.vel[0] = v0[0]; m.vel[1] = v0[1]; m.vel[2] = v0[2];
      const rest = on.by(m.id), hard = run.by(m.id); const nr = breathsOf(rest), nh = breathsOf(hard); const late = hard[hard.length - 1]; const r0 = rest[0]?.opts ?? { steam: 0, size: 0 };
      const others = out.filter(c => c !== m).every(c => run.by(c.id).map(key).join('|') === on.by(c.id).map(key).join('|'));
      verdict(nh >= nr + 2 && !!late && nr > 0 && late.opts.steam > r0.steam * 1.2 && late.opts.size > r0.size * 1.2 && late.opts.steam <= 1 && others,
        'effort — a man who runs breathes faster and bigger', `#${m.id} held at 4.5 m/s for ten seconds: ${nh} breaths against ${nr} standing · his last record's steam share ${late?.opts.steam.toFixed(2)} against ${r0.steam.toFixed(2)}, its size ${late?.opts.size.toFixed(3)} m against ${r0.size.toFixed(3)} · the others' records untouched by it: ${others}`); }
  } finally { SNOW.on = was.snow; SNOW_LOOK.breath = was.breath; if (was.envSnow === undefined) delete process.env.TPS_SNOW; else process.env.TPS_SNOW = was.envSnow; if (was.envBreath === undefined) delete process.env.TPS_SNOWBREATH; else process.env.TPS_SNOWBREATH = was.envBreath; }
  console.log(`\nsnowbreath-check: ${checks - fails}/${checks} pass${BREAK ? ' (SNOWBREATH_BREAK=1: the switch never raised — this run must fail)' : ''}`);
  return fails === 0;
}

if (import.meta.main) { const ok = await snowBreathCheck({ verbose: process.argv.includes('--verbose'), fast: process.argv.includes('--fast') }); process.exit(ok ? 0 : 1); }
