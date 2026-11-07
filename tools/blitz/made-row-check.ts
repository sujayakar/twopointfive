// A ROW THE APPLY MAKES HAS EVERY VALUE IT WAS SENT. An object the apply makes has no field yet, so a zero read onto it is a zero and not "this optional field
// stays unset": a field reached by a path (`o["head"]`) lost its zero, and `anim.nodes.head`, node 0 of the rig, was missing on every man made at run time.
//   TPS_GAME=rust bun run tools/blitz/made-row-check.ts [address]     exit 0 every made man whole · 1 a hole · 3 no man was made, so nothing was checked
// MADE_ROW_BREAK=1 stands a man's node table with a hole in it, and the check must fail; MADE_CLASS_BREAK=1 takes a spring's class and a typed leaf off him, and the check must fail.
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
// A MADE ROW IS ITS CLASS'S, HELD AGAINST ONE THE CLASS BUILT: a man of the start, path by path. FIELD: the class has it and the row has not. CLASS: a part of another class than the class's. SHAPE: another kind of value (a plain list where the class keeps a typed array).
// LEFT, on purpose: the picture's own half (`picture()`, the page's made hook, which a headless game does not run) and the seven named poses, which a made man holds apart from his set (MADE-ROW-APPLY's item three). The record says what the rest should be: tools/wire/class-defaults.ts.
const kind = (v: any): string => v === null ? 'null' : v === undefined ? 'undefined' : Array.isArray(v) ? 'list' : ArrayBuffer.isView(v) ? v.constructor.name : typeof v === 'object' ? (v instanceof Map ? 'Map' : v instanceof Set ? 'Set' : 'object') : typeof v;
const FOR_THE_PICTURE = /(^|\.)(g|ch|cast|rig|ibm|ibms|ownRot|jointViews|jointData|fk|rlSrc|posIn|ev|tmp\w*|scratch\w*)(\.|$)/, NAMED_POSES = /^anim\.(pose|pA|pB|pC|pUpper|rest|pAim|poses)(\.|$)/;
const LINES = new Set<string>();
function hold(a: any, b: any, path: string, depth: number, seen: Set<object>): void { if (!b || typeof b !== 'object' || seen.has(b) || depth > 5 || ArrayBuffer.isView(b)) return; seen.add(b);
  if (!a || typeof a !== 'object') return; if (Array.isArray(b)) { if (Array.isArray(a) && a.length && b.length) hold(a[0], b[0], `${path}[0]`, depth + 1, seen); return; } if (b instanceof Map || b instanceof Set) return;
  const P = Object.getPrototypeOf(b); if (P !== Object.getPrototypeOf(a) && !FOR_THE_PICTURE.test(path) && !NAMED_POSES.test(path)) LINES.add(`CLASS ${path}: the row's ${Object.getPrototypeOf(a)?.constructor?.name}, the class's ${P?.constructor?.name}`);
  for (const k of Object.keys(b)) { const p = path ? `${path}.${k}` : k; if (FOR_THE_PICTURE.test(p) || NAMED_POSES.test(p)) continue; const v = b[k]; if (typeof v === 'function') continue; if (!(k in a)) { if (v !== undefined) LINES.add(`FIELD ${p}: ${kind(v)}`); continue; } const u = a[k], K = kind(u), L = kind(v); if (K !== L && !(K === 'null' || L === 'null' || K === 'undefined' || L === 'undefined')) { LINES.add(`SHAPE ${p}: the row's ${K}, the class's ${L}`); continue; } hold(u, v, p, depth + 1, seen); } }
if (process.env.MADE_CLASS_BREAK) { const st = made[0].anim.st; Object.setPrototypeOf(st.w, Object.prototype); made[0].anim.rd = Array.from(made[0].anim.rd); }
const built = chars()[0]; for (const c of made.slice(0, 1)) hold(c, built, '', 0, new Set());
for (const l of [...LINES].slice(0, 40)) console.log(`FAIL: a made man: ${l}`);
console.log(`=== made-row-check: ${made.length} men made in ${q}, ${nodes.length} nodes each: ${holes ? `${holes} with a hole` : 'whole'}; ${LINES.size} lines against a man the class built ===`);
process.exit(holes || LINES.size ? 1 : 0);
