// A SMOKE SOURCE RIDES WHAT THE GAME SAYS IT RIDES (src/smoke/types.ts SmokeEmitter.track; tools/blitz/track.ts; PAGE.md "what it rides").
// The solver asks a source every frame where the thing it hangs on is now: the gun's barrel, its ejection port, a flying spark. On the Rust game that
// thing is named by the tick (effects::Track) and crosses in the out event; the page makes the question from the name. This holds the question to the
// thing, frame after frame, with a man walking and the sparks flying.
//   tools/blitz/rust.sh rust bun run tools/blitz/track-check.ts      (both stands: TPS_PROFILE=shipped in front for the page's)
//   TRACK_BREAK=1 …   = every source taken as the import handed it over before this was mended (what it rides dropped): must fail
// exit 0 = all pass, 1 = a FAIL line. No solver stands here: what is held is what the solver would be told, never smoke seen.
import { standUp, ROOT } from '../qa/headless.ts';
import { readFileSync } from 'node:fs';
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const fx = await import(`${ROOT}/src/game/effects.ts`);
const DT = 1 / 60, BREAK = process.env.TRACK_BREAK === '1', say = console.log;
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
let fails = 0; const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; say(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const far = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);

const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8'))), seed: 7 }));
const g = W.game;
await quiet(() => startFromQuery(g, '?levelFile=pass.json&roomclear=0&squad4&difficulty=hard'));
g.aiEnabled = false; g.godMode = true; g.playerInvisible = true;
const filed: any[] = [], emit0 = g.smoke.emit?.bind(g.smoke);
g.smoke.emit = (e: any) => { if (BREAK) e.track = undefined; filed.push(e); emit0?.(e); };
/** where the solver would put a source this frame (smoke.ts update: its track's answer, else where it was filed) */
const at = (e: any): number[] | null => { if (!e.track) return e.pos; const t = e.track(); return t ? t.pos : null; };
say(`\n=== track: a smoke source rides what the game says it rides · the ${g.gameWord ?? 'TypeScript'} game${BREAK ? ' (what it rides dropped: must fail)' : ''} ===`);

// ---- the barrel and the port: a shot, then the man walks for a second
{ W.step(DT); W.step(DT);
  const c = g.player.char, p0 = [c.pos[0], c.pos[1], c.pos[2]];
  fireWeapon(g, c, [p0[0] + 4, 1.2, p0[2]], true, { spread: 0 }); W.step(DT);
  const wisps = filed.filter(e => e.kind === 'wisp'), barrel = wisps[0], port = wisps[1];
  let worst = 0, worstPort = 0;
  for (let i = 0; i < 60; i++) { g.puppet = { goal: [p0[0], 0, p0[2] + 3], aim: [p0[0] + 4, 1.2, p0[2] + 3] }; W.step(DT);
    const a = barrel && at(barrel), b = port && at(port); worst = Math.max(worst, a ? far(a, c.muzzle) : 99); if (b) worstPort = Math.max(worstPort, far(b, c.muzzle)); }
  const walked = far(c.pos, p0);
  verdict(!!barrel && walked > 0.5 && worst < 1e-9, 'barrel — the wisp a shot leaves is at the muzzle every frame while the man walks',
    `${wisps.length} wisps filed · the man walked ${walked.toFixed(2)} m in a second · the barrel's source at most ${worst.toFixed(3)} m from the muzzle`);
  verdict(!port || (walked > 0.5 && worstPort < 0.9), 'port — the curl off the ejection port stays on the gun',
    port ? `at most ${worstPort.toFixed(3)} m from the muzzle (the port is a forearm behind it)` : 'no port wisp is filed on this stand (effects.ts flags.ejectionWisp)'); }

// ---- the sparks: a stun's trails, each on its own spark until that spark is gone
{ filed.length = 0;
  fx.spawnBurst(g.smoke, g.sparks, [16, 0.1, 16], 3); W.step(DT);
  const trails = filed.filter(e => e.kind === 'trail'), born = trails.map(e => [e.pos[0], e.pos[1], e.pos[2]]);
  for (let i = 0; i < 12; i++) W.step(DT);
  const now = trails.map(at), live = now.filter(p => p).length, moved = now.filter((p, i) => p && far(p, born[i]) > 0.15).length;
  const onSpark = now.filter(p => p && g.sparks.pool.some((s: any) => s.alive && far(s.pos, p) < 1e-9)).length;
  verdict(trails.length > 0 && live > 0 && moved === live && onSpark === live, 'sparks — a burst\'s trails fly with their sparks',
    `${trails.length} trails filed · after 12 frames ${live} still ride a live spark, ${moved} of them over 15 cm from where they were filed, ${onSpark} exactly on a spark of the pool`); }

say(fails ? `\ntrack-check: ${fails} FAILED` : '\ntrack-check: all pass');
process.exit(fails ? 1 : 0);
