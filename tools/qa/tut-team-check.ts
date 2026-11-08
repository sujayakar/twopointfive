// TUT-TEAM-CHECK: the squad's tutorial mission on the ground (tools/levels/tut-team.ts → levels/tut-team.json: the tunnel station of tut-sky with the player
// on the road behind his own truck). What this mission leans on, held as numbers:
//   2. ONE ORDER AND ONE WORD. From the first frame he has what he learnt alone and nothing of the squad's; the steps add the zoom, the squad's key and the go
//      key, in that order, and nothing else; the squad's key gives move, stack, the go and fall in and no other order (the mains box's own row is not among them: read live, a tap on the box has fall in for its first row). No line asks for the ring, a charge, the mains box or the goggles, and none asks for the squad's key a second
//      time on a door they stand on. The store is no room of the fight.
//   3. HE STARTS IN THE TRUCK'S SHADOW. From each post of the roadblock (the script's own POSTS, at a standing man's eye) a line to his knee, his chest and
//      his chin where he starts is stopped by something solid (a pickup hides a man to 1.6 m: the top of his head is over it, and is lines 4 and 5's).
//   4. LIVE, NOBODY BEHIND THE TRUCKS IS HIT WHILE THEY ARE PINNED. On one seed (TUTTEAM_SEEDS for more) the mission is stood up as the page stages it and left alone for
//      forty seconds (no key: the roadblock's fire goes on until the smoke, and the smoke waits for the zoom): he is not hit on his feet (one round puts
//      him down), the roadblock's three are on their posts and the squad's four are still on their marks behind the tails, none of them hit.
//   bun run tools/qa/tut-team-check.ts          and again with TPS_PROFILE=shipped in front (the stand the page opens on)
//   TUTTEAM_BREAK=1 …   he is stood on the road in front of the trucks (the north one's nose, then the south one's where the blind fire goes): lines 3, 4, 5
//                       and 6 must be RED (the fire is real, and the shadow is the truck's)
// EXIT: 0 = every line holds · 1 = a fault, each a line.
// Headless: no GPU, and no smoke between anyone (every man's sight through smoke is 1 here), so what happens after the smoke is the page's to show.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

process.env.TPS_RAILS ??= '1';   // (the level has a lesson only on the frame: off on the frozen stand unless raised)

const ROOT = resolve(import.meta.dir, '../..'), BROKEN = process.env.TUTTEAM_BREAK === '1', SEEDS = process.env.TUTTEAM_SEEDS ? process.env.TUTTEAM_SEEDS.split(',').map(Number) : [1], SECS = 40;   // (TUTTEAM_SEEDS=7,8,9: other dice, for a longer measure)
/** where the break stands him: on the road in front of the north truck's nose, in the roadblock's line · and, once the smoke is out, in front of the
 *  south truck's nose where the blind fire goes (the lane in front of the north truck is not fired down blind: his way to the gate goes by it) */
const OPEN = [13.6, 0, 22.3];
const faults: string[] = [];
const line = (ok: boolean, say: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${say}: ${got}`); if (!ok) faults.push(`${say} (${got})`); };

const TEXT = readFileSync(resolve(ROOT, 'levels/tut-team.json'), 'utf8'), L = JSON.parse(TEXT);
// 2 ---- one order and one word
{ const R = L.scenarios.lesson.rails, steps: any[] = L.scenarios.lesson.steps, SQUADS = ['order', 'word', 'menu', 'ring', 'who', 'tools', 'plan', 'goal', 'nvg', 'zoom'];
  const early = (R.allow as string[]).filter(a => SQUADS.includes(a)), added = steps.flatMap(s => (s.allow ?? []) as string[]);
  const words = steps.flatMap(s => [s.line, s.after?.line, s.after?.word, s.explain?.text, s.flatLine, s.menuLine, s.pageLine, s.noTap].filter(Boolean) as string[]);
  const asks = words.filter(w => /\[X\]|\[N\]|\[[1-5]\]|charge|breaker|night vision/i.test(w)), again = words.filter(w => /\[B\][^.]*\bagain\b|\bagain\b[^.]*\[B\]/i.test(w));
  const rooms: string[] = L.scenarios.roomClear.rooms.map((r: any) => r.id);
  const gives = [...((R.orders ?? []) as string[])].sort().join();
  line(early.length === 0 && added.join() === 'zoom,order,word' && gives === 'fallIn,go,move,stack' && asks.length === 0 && again.length === 0 && !rooms.includes('store'),
    'he starts with what he learnt alone; the steps add the zoom, the squad\'s key and the go key and nothing else; the squad\'s key gives move, stack, the go and fall in only (a tap on the mains box sends nobody to it); no line asks for the ring, a charge, the mains box, the goggles or a second tap on a stacked door; the store is no room of the fight',
    `the frame's own: ${early.join(', ') || 'none of the squad\'s'} · added: ${added.join(', ')} · orders: ${gives || 'EVERY ORDER (no list)'} · ${asks.length + again.length ? `ASKS: ${[...asks, ...again].join(' | ')}` : `${words.length} texts read`} · rooms ${rooms.join(', ')}`); }

const { standUp } = await import('./headless');
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`), { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
/** the mission stood up as the page stages it on `seed` (he on the break's spot when it is asked), then `fn` on it, the console muted meanwhile. Answers the fault ('' = none) */
const live = async (seed: number, fn: (W: any, g: any) => void, broken = OPEN): Promise<string> => {
  const log = console.log, warn = console.warn, err = console.error;
  try { console.log = console.warn = console.error = () => { };
    const W: any = await standUp({ level: loadLevel(parseLevelData(TEXT)), seed }), g = W.game;
    const r = startFromQuery(g, '?levelFile=tut-team.json&roomclear=1&squad4'); if (r.why) return String(r.why);
    if (BROKEN) g.teleportPlayer(broken);
    fn(W, g); return '';
  } catch (x) { return String((x as Error).message); } finally { console.log = log; console.warn = warn; console.error = err; }
};
const POSTS: [string, number[]][] = [...readFileSync(resolve(ROOT, 'levels/tut-team.script.ts'), 'utf8').matchAll(/\{ name: '(Tango \d+)', at: \[([\d., ]+)\] \}/g)].map(m => [m[1], m[2].split(',').map(Number)]);

// 3 ---- the truck's shadow
{ const open: string[] = []; let at: number[] = [];
  const why = await live(1, (_W, g) => { at = [...g.player.char.pos];
    for (const [name, p] of POSTS) for (const [part, y] of [['knee', 0.5], ['chest', 1.2], ['chin', 1.5]] as [string, number][]) {
      const o = [p[0], 1.5, p[2]], to = [at[0], y, at[2]], d = [to[0] - o[0], to[1] - o[1], to[2] - o[2]], len = Math.hypot(d[0], d[1], d[2]);
      if (!g.col.raycast(o, d.map(v => v / len), len - 0.3)) open.push(`${name} → his ${part}`); } });
  line(!why && POSTS.length === 3 && open.length === 0, 'where he starts, a line from each post of the roadblock to his knee, his chest and his chin is stopped by something solid (a pickup hides a man to 1.6 m: his head is line 4\'s and 5\'s)', why || `he at (${at[0]?.toFixed(2)}, ${at[2]?.toFixed(2)}) · ${POSTS.length} posts · ${open.length ? `OPEN: ${open.join(', ')}` : 'every line stopped'}`); }

// 4, 5 ---- live: pinned, then under the blind fire, and nobody hit
const stood = async (say: string) => { const rows: string[] = []; let bad = 0;
  for (const seed of SEEDS) {
    let hits = 0, down = false, posts = 0, off = 0, step = '', who = ''; const knees: string[] = [];
    const why = await live(seed, (W, g) => {
      const marks: number[][] = L.scenarios.roomClear.team.ops, full = g.player.hitsLeft;
      for (let i = 0; i < SECS * 60; i++) { W.step(1 / 60); for (const m of g.guards) if (m.unit && m.team === 'operative' && g.rails.isSpared(m) && !knees.some(k => k.startsWith(m.callsign))) knees.push(`${m.callsign} at ${g.time.toFixed(1)} s (${m.char.pos[0].toFixed(1)}, ${m.char.pos[2].toFixed(1)})`); }
      hits = full - g.player.hitsLeft; down = !!g.player.down; step = g.view.lesson?.id ?? '';
      posts = g.guards.filter((x: any) => POSTS.some(([n]) => n === x.callsign) && x.state !== 'dead').length;
      const men = g.guards.filter((x: any) => x.unit && x.team === 'operative' && x.state !== 'dead');
      const away = men.filter((m: any) => !marks.some(k => Math.hypot(m.char.pos[0] - k[0], m.char.pos[2] - k[2]) < 1.0)); off = 4 - men.length + away.length;
      who = away.map((m: any) => `${m.callsign} at (${m.char.pos[0].toFixed(1)}, ${m.char.pos[2].toFixed(1)}) ${g.orders.of(m)?.order ?? 'no order'}${m.script ? ', the script\'s' : ', NOT the script\'s'}`).join(', ');
    }, OPEN);
    const want = 'zoom', ok = !why && hits === 0 && !down && posts === 3 && off === 0 && knees.length === 0 && step === want; if (!ok) bad++;
    rows.push(`seed ${seed}: ${why || `${hits ? `HIT ${hits}×${down ? ', DOWN' : ''}` : 'not hit'} · posts ${posts} · ${off ? `${off} OFF THEIR MARKS (${who})` : 'the four on their marks'}${knees.length ? ` · PUT ON A KNEE: ${knees.join(', ')}` : ''}${step === want ? '' : ` · step '${step}'`}`}`);
  }
  line(bad === 0, say, rows.join(' | ')); };
await stood(`pinned for ${SECS} s with no key pressed, on ${SEEDS.length} seeds: he is not hit on his feet, the roadblock's three are on their posts and the squad's four are on their marks behind the tails, none of them hit`);

console.log(faults.length ? `\n=== tut-team-check: ${faults.length} FAULT${faults.length > 1 ? 'S' : ''} ===` : '\n=== tut-team-check: ALL PASS ===');
process.exit(faults.length ? 1 : 0);
