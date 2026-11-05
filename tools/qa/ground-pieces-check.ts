// THE LOOK KIT'S GROUND PIECES (src/scene/gwotKit.ts rubble, stain, hearth, ruts, patch, ground, groundAt, lie) and the lived tail's closing
// (tools/levels/livedKit.ts livedTail): is what lies on the ground the ground's own, and does each flat thing lie where the table says?
//   bun run tools/qa/ground-pieces-check.ts                         exit 0 = all pass, 1 = a line failed (named)
//   GROUNDPIECES_BREAK=1 bun run tools/qa/ground-pieces-check.ts    the red path: each line asks the KIT for the thing as it was laid before (a scatter judged
//                                                                   as another row's, the ground asked before it is laid, a patch of damp for the ash, a
//                                                                   stain at a patch's height, a box laid inside the tail) — every line must FAIL
// Each piece is laid alone in a kit stood up empty, as tools/levels/library.ts lays a piece for the catalogue. WHAT IS HELD, over EVERY row of the palette
// (a row added tomorrow is asked the same):
//   1 a scatter   rubble in a row is that row: every stone wears the row's paint or its darker row's (gwotPalette.ts GWOT_DARKER) with the row's finish,
//                 bare metal included, and to a round every stone is what the row is (gwotSimOf); a row with a darker row shows both tones
//   2 the ground  groundAt answers what `ground` laid: the higher finish where two overlap, the later of two as high, nothing where it laid none
//   3 a hearth    on every finish `ground` lays the trodden ash is that ground's own paint darker (no channel under half of it, none over it: never a
//                 black square on pale earth); on ground the kit did not lay it takes the ground it is told, and told nothing it THROWS
//   4 heights     the table `lie` is the order finish < stain < ruts < patch, a millimetre or more apart, and the pieces lie where it says:
//                 on the kit's default finish a stain lies over the finish and under ruts, and a patch over them
//   5 the tail    when a lived tail is done the generator's group is no lived group, so a box laid after it is one the exporter refuses by name
// The files that follow from the kit are held by the levels' own checks (tools/levels/export.ts --check, tools/qa/lived-check.ts <level>). No picture is
// judged here.
import type { Vec3 } from '../../src/math/vec';
import { BoxFlag, type Box } from '../../src/scene/boxes';
import { officeKit } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice } from '../../src/scene/gwotKit';
import { GWOT_PALETTE, GWOT_FINISH, GWOT_SIM, GWOT_DARKER, gwotAs, gwotSimOf, type GwotMat } from '../../src/scene/gwotPalette';
import { isLived } from '../../src/scene/lived';
import { livedTail } from '../levels/livedKit';

const BREAK = process.env.GROUNDPIECES_BREAK === '1';
let failed = 0;
const check = (ok: boolean, what: string, detail = '') => { if (!ok) failed++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
type Kit = ReturnType<typeof gwotKit>;
/** a kit stood up empty, with the group of every box it is handed */
const stand = () => { const groups: string[] = [], K = officeKit({ groups }); return { K, G: gwotKit(gwotOfOffice(K)), groups }; };
/** what one call of `lay` adds to a kit stood up empty (after `before`, which is not counted) */
const laidBy = (lay: (G: Kit) => void, before?: (G: Kit) => void): Box[] => { const { K, G } = stand(); before?.(G); const n = K.boxes.length; lay(G); return K.boxes.slice(n); };
const same = (a: readonly number[], b: readonly number[]) => a[0] === b[0] && a[1] === b[1] && a[2] === b[2];
const top = (b: Box) => b.c[1] + b.h[1], mm = (y: number) => `${(y * 1000).toFixed(1)} mm`;
const ROWS = Object.keys(GWOT_PALETTE) as GwotMat[];

// 1. a scatter is one row in two tones at most, with the row's finish, and one thing to a round
{ const wrong: string[] = []; let two = 0, metal = 0;
  for (const row of ROWS) {
    const stones = laidBy(G => G.rubble(0, 0, 0.8, 40, BREAK && row !== 'mud' && row !== 'mudDark' ? 'mud' : row));   // (the red path: a scatter of mud judged as this row's, as three stones in ten once were)
    const darker = GWOT_DARKER[row], second: Vec3 | undefined = darker ? gwotAs(darker, GWOT_SIM[row]) : undefined, isMetal = !!GWOT_FINISH[row]?.metal;
    const odd = stones.filter(b => !same(b.albedo, GWOT_PALETTE[row]) && !(second && same(b.albedo, second))), mis = stones.filter(b => gwotSimOf(b.albedo) !== GWOT_SIM[row]);
    const dull = stones.filter(b => !!b.metal !== (same(b.albedo, GWOT_PALETTE[row]) ? isMetal : !!(darker && GWOT_FINISH[darker]?.metal)));
    if (stones.length !== 40 || odd.length || mis.length || dull.length) wrong.push(`${row}: ${stones.length} stones, ${odd.length} in another paint, ${mis.length} another thing to a round, ${dull.length} with another finish`);
    if (second && stones.some(b => same(b.albedo, second)) && stones.some(b => same(b.albedo, GWOT_PALETTE[row]))) two++;
    if (isMetal) metal++;
  }
  const families = ROWS.filter(r => GWOT_DARKER[r]);
  check(wrong.length === 0 && two === families.length && families.length > 0 && metal > 0, `1 a scatter of rubble in any of the palette's ${ROWS.length} rows is that row: every stone wears its paint or its darker row's, has the row's finish, and to a round is what the row is`,
    wrong.length ? `${wrong.length} rows wrong: ${wrong.slice(0, 4).join(' · ')}` : `${families.length} rows have a darker row (${families.map(r => `${r} → ${GWOT_DARKER[r]}`).join(', ')}) and show both tones; the other ${ROWS.length - families.length} are laid in one; ${metal} rows are bare metal and their stones are`); }

// 2. the kit knows the ground it laid, and only that
{ const { G } = stand(), ask = () => [G.groundAt(0, 0), G.groundAt(2, 0), G.groundAt(4.5, 4.5), G.groundAt(-2, -2), G.groundAt(9, 9)].map(g => g ?? 'none').join(',');
  const early = ask(); G.ground(-3, -3, 3, 3, 'dirt'); G.ground(-1, -1, 1, 1, 'gravel', 0.006); G.ground(5, 5, 4, 4, 'slab'); G.ground(-3, -3, -1, -1, 'mudFloor');
  const got = BREAK ? early : ask();   // (the red path: asked before anything is laid)
  check(got === 'gravel,dirt,slab,mudFloor,none' && early === 'none,none,none,none,none', '2 the ground at a spot is what the kit laid there: the higher finish where two overlap, the later of two as high, a rectangle given either way round, and nothing where it laid none or has not laid yet', `${got.split(',').join(' · ')} · before laying: ${early.split(',').join(' · ')}`); }

// 3. a hearth's ash is its ground, darker, and a hearth does not guess its ground
{ const FINISHES: GwotMat[] = ['dust', 'dirt', 'gravel', 'mudFloor', 'slab'], lines: string[] = []; let bad = 0;
  const ashOn = (g: GwotMat, how: 'laid' | 'told') => {
    const made = laidBy(G => { if (BREAK) G.patch(0, 0, 1.5, 1.3, 'damp', 0.6); else G.hearth(0, 0, 0.3, how === 'told' ? g : undefined); }, G => { if (how === 'laid') G.ground(-3, -3, 3, 3, g); });   // (the red path: the ash as it was, a patch of damp)
    const ash = made.find(b => (b.flags & BoxFlag.NoShadow) !== 0), k = ash ? ash.albedo.map((x, i) => x / GWOT_PALETTE[g][i]) : [];
    const ok = !!ash && k.every(x => x >= 0.5 && x < 1); if (!ok) bad++;
    lines.push(`${g} ${how}: ${ash ? k.map(x => x.toFixed(2)).join(' ') : 'no ash'}`);
  };
  for (const g of FINISHES) { ashOn(g, 'laid'); ashOn(g, 'told'); }
  let threw = ''; try { laidBy(G => G.hearth(0, 0, 0.3)); } catch (e) { threw = String((e as Error).message); }
  check(bad === 0 && /stands on no ground/.test(threw), '3 a hearth\'s trodden ash is the ground\'s own paint darker on every finish the kit lays, laid or told: no channel under half of the ground, none over it; on ground the kit did not lay and is not told, it throws', `${lines.join(' · ')} · told nothing on bare ground: ${threw ? 'throws' : 'LAYS ONE'}`); }

// 4. the table's order, and the pieces lie where it says
{ const { G } = stand(), L = G.lie, order = [L.finish, L.stain, L.ruts, L.patch], apart = order.slice(1).map((y, i) => y - order[i]);
  const g = laidBy(K => K.ground(-3, -3, 3, 3, 'dust'))[0], st = laidBy(K => K.stain(0, 0, 1, 1, 'dust', 0, 0.66, ...(BREAK ? [K.flatAt(K.lie.patch)] as const : [] as const)))[0];   // (the red path: a stain at a patch's height, as it was)
  const ru = laidBy(K => K.ruts(-2, 0, 2, 0)), pa = laidBy(K => K.patch(0, 0, 1, 1))[0], se = laidBy(K => K.seep(-3, 0, 3, 0));
  const near = (a: number, b: number) => Math.abs(a - b) < 1e-9;
  const ok = apart.every(d => d >= 0.001 - 1e-12) && near(top(g), L.finish) && near(top(st), L.stain) && ru.every(b => near(top(b), L.ruts)) && near(top(pa), L.patch) && se.every(b => top(b) >= L.patch - 1e-9) && top(g) < top(st) && top(st) < Math.min(...ru.map(top)) && Math.max(...ru.map(top)) < top(pa);
  check(ok, '4 what lies on what: the table is finish, stain, ruts, patch, each a millimetre or more over the one before (the exporter parts two flats by 0.7 mm), and every piece lies where it says: a stain over the default finish and under a vehicle\'s ruts, a patch and a seep over them',
    `finish ${mm(top(g))} · stain ${mm(top(st))} · ruts ${mm(Math.min(...ru.map(top)))} · patch ${mm(top(pa))} · a seep's lengths ${mm(Math.min(...se.map(top)))} to ${mm(Math.max(...se.map(top)))}`); }

// 5. a lived tail, done, leaves the generator in no lived group
{ const { K, groups } = stand(), T = livedTail(K.props, K.grp);
  T.room('yard'); const a = K.add({ c: [0, 0.1, 0], h: [0.1, 0.1, 0.1] }); if (!BREAK) T.done(); const b = K.add({ c: [1, 0.1, 0], h: [0.1, 0.1, 0.1] });   // (the red path: the second box laid before the tail is done, as one laid after it once was)
  const stray = groups.findIndex((t, i) => i > groups.findIndex(isLived) && !isLived(t));   // (the exporter's own question: tools/levels/export.ts "lived things must come last")
  check(isLived(groups[a]) && !isLived(groups[b]) && stray === b, '5 when a lived tail is done the generator is in no lived group: a box laid after it is not a lived thing by accident, and is the stray the exporter refuses',
    `in the room: ${groups[a]} · after done(): ${groups[b]}`); }

console.log(failed ? `=== ground pieces: ${failed} FAILED ===` : '=== ground pieces: ALL PASS ===');
process.exit(failed ? 1 : 0);
