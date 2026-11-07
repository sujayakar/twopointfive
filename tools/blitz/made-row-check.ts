// A ROW THE APPLY MAKES HAS EVERY VALUE IT WAS SENT. An object the apply makes has no field yet, so a zero read onto it is a zero and not "this optional field
// stays unset": a field reached by a path (`o["head"]`) lost its zero, and `anim.nodes.head`, node 0 of the rig, was missing on every man made at run time.
//   TPS_GAME=rust bun run tools/blitz/made-row-check.ts [address]     exit 0 every made man whole · 1 a hole · 3 no man was made, so nothing was checked
// MADE_ROW_BREAK=1 stands a man's node table with a hole in it, and the check must fail.
const { standUp } = await import('../qa/headless.ts');
const { SCHEMA, bookOf } = await import('./export.ts');
const { spawnUnit } = await import('../../src/game/guards.ts');
const q = process.argv[2] ?? 'nightbreach', W: any = await standUp({ search: `?${q}` } as any), g = W.game;
const nodes = SCHEMA.types['crate::character::AnimNodes'].fields.map((f: any) => f.expr.match(/\["(.+)"\]/)[1]) as string[];
W.step(1 / 60);   // (the Rust game stands at the first tick)
const chars = () => bookOf(g)!.listOf('CharId') as any[], was = chars().length;
spawnUnit(g, { team: 'enemy', pos: [...g.player.char.pos], budget: 16 });   // (a man the Rust game makes at run time)
W.step(1 / 60);
const made = chars().slice(was);
if (!made.length) { console.log(`made-row-check: no man was made in ${q}, so nothing was checked`); process.exit(3); }
if (process.env.MADE_ROW_BREAK) delete made[0].anim.nodes[nodes[0]];
let holes = 0;
for (const c of made) { const N = c.anim?.nodes; const lost = nodes.filter(k => typeof N?.[k] !== 'number'); if (lost.length) { holes++; console.log(`FAIL: a made man's node table has no ${lost.join(', ')}`); } }
console.log(`=== made-row-check: ${made.length} men made in ${q}, ${nodes.length} nodes each: ${holes ? `${holes} with a hole` : 'whole'} ===`);
process.exit(holes ? 1 : 0);
