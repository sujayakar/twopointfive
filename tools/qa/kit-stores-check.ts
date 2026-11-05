// THE LOOK KIT'S DRUMS (src/scene/gwotKit.ts drum, drums, drumDown): is a drum steel whatever it is painted, and is the one lying on its side a round?
//   bun run tools/qa/kit-stores-check.ts                       exit 0 = all pass, 1 = a line failed (named)
//   KITSTORES_BREAK=1 bun run tools/qa/kit-stores-check.ts     the red path: each line is asked of the thing a drum once was (a crate in the same paint for
//                                                              the drum standing, the wood pile's plain boxes for the drum lying) — every line must FAIL
// Each piece is laid alone in a kit stood up empty (tools/qa/kit-stand.ts). WHAT IS HELD, over EVERY row of the palette (a row added tomorrow is asked
// the same; a level may paint a drum in any of them):
//   1 steel      a drum standing and a drum lying, in any row, is STEEL to the rules: it says it does what steel does (Box.does, the row 'steel' of
//                scene/materials.ts), so a round's hit knocks out of it what metal gives and a blast breaks it as metal (never the wood a wall
//                charge chews), and with a hit read from what a thing is DRAWN as (particleFx.ts HITSTUFF, raised here) it is metal or leaves the hit
//                to what it does. It wears the row to the eye (the row's own numbers) and is known as the row's paint (scene/stuff.ts paintOf). WHAT IT SAYS: the row's own word where the
//                row is a coat or metal already (`paint`, `enamel`, a row the table reads as metal); in every other row `paint`, with a drum enamel's
//                finish, never the row's stuff (a drum in the colour of mud is not made of mud). Until 11-05 an olive drum was an ammunition crate's
//                dark wood to a round, and a level that wanted a steel one picked another colour
//   2 the dealt  `drums` deals from its own paints, one of which is NOT metal by its row (olive: the line asks nothing without one), and each drum it
//                deals is steel
//   3 lying      the drum on its side is ONE solid box that says `round: 1` (scene/shapes.ts), square across its length and standing on the ground, with
//                its two ends and its two ribs as trim (drawn only, each round about the same axis, none over 2 cm out of the box, bare metal where the
//                drum's paint is), on two chocks that reach in under the round's flank; it carries no markings (a box that says a form cannot); and it
//                hands back its top, along which the kit's `lay` puts a sheet that follows the round (scene/carShapes.ts layOn)
// The files that follow from the kit are held by the levels' own checks (tools/levels/export.ts --check, tools/qa/lived-check.ts <level>). No picture is
// judged here; the shape rule's own check (tools/qa/shapes-check.ts) holds what a round box is to a ray and a walker.
import type { Vec3 } from '../../src/math/vec';
import { BoxFlag, type Box } from '../../src/scene/boxes';
import { GWOT_PALETTE, GWOT_FINISH } from '../../src/scene/gwotPalette';
import { paintOf, stuffOfName, doesOfName } from '../../src/scene/stuff';
import { doesOf, DOES_DEFAULT } from '../../src/scene/materials';
import { saidSurf, HITSTUFF } from '../../src/game/particleFx';
import { laid, lines, ROWS, type Kit } from './kit-stand';

const BREAK = process.env.KITSTORES_BREAK === '1', { check, failed } = lines();
const solid = (b: Box) => !(b.flags & BoxFlag.NoShadow), top = (b: Box) => b.c[1] + b.h[1];
const near = (a: readonly number[], b: readonly number[]) => Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1]), Math.abs(a[2] - b[2])) <= 0;   // (a stand-in is the row's own numbers: gwotPalette.ts gwotAs)
/** does this box do what steel does, to everything of the game that asks? and what it does where it does not */
HITSTUFF.on = true;   // (what a hit reads from a thing's word is asked too, whatever the stand)
const readAs = (b: Box): string[] => { const d = doesOf(b), said = saidSurf(b); return [(b.does ?? DOES_DEFAULT) === 'steel' ? '' : `it says it does '${b.does ?? DOES_DEFAULT}'`, d.surf === 'metal' ? '' : `a hit: ${d.surf}`, said === undefined || said === 'metal' ? '' : `a hit by its word '${b.stuff}': ${said}`, d.breaks === 'metal' ? '' : `a blast: ${d.breaks}`].filter(x => x); };
console.log(`\n=== kit stores: a drum is steel in any paint, and the one on its side is a round${BREAK ? ' (KITSTORES_BREAK: each line is asked of what a drum once was)' : ''} ===`);

// 1. steel in every row, standing and lying; the row to the eye, the row's word to the picture
{ const wrong: string[] = []; let stoodIn = 0, coated = 0;
  for (const row of ROWS) for (const [how, lay] of [['standing', (G: Kit) => BREAK ? G.crates(0, 0, 0, 1, row) : G.drum(10, 10, row)], ['lying', (G: Kit) => BREAK ? G.crates(0, 0, 0, 1, row) : G.drumDown(10, 10, 0.3, row)]] as const) {
    const body = laid(lay).boxes.filter(b => solid(b) && top(b) > 0.2 && near(b.albedo, GWOT_PALETTE[row]));   // (the drum's own boxes: its chocks, under a hand high, are wood and are not asked)
    const notSteel = [...new Set(body.flatMap(readAs))], rowWord = stuffOfName(row), own = rowWord === 'paint' || rowWord === 'enamel' || doesOfName(row) === 'steel';
    const word = own ? (rowWord === 'plain' || rowWord === 'waits' ? undefined : rowWord) : 'paint', rough = own ? GWOT_FINISH[row]?.rough : GWOT_FINISH.drumBlue?.rough; if (!own) coated++;
    const unnamed = body.filter(b => paintOf(b.albedo)?.name !== row).length, unsaid = body.filter(b => b.stuff !== word).length, unfinished = rough === undefined ? 0 : body.filter(b => b.rough !== rough || (!own && paintOf(b.albedo)?.rough !== rough)).length;   // (the box's own roughness, and for a coat the finish its paint was said with: stuff.ts saidAs)
    if (!body.length || notSteel.length || unnamed || unsaid || unfinished) wrong.push(`${row} ${how}: ${body.length} boxes${notSteel.length ? `, not metal to ${notSteel.join(' · ')}` : ''}${unnamed ? `, ${unnamed} not known as the row's paint` : ''}${unsaid ? `, ${unsaid} not saying ${word ? `'${word}'` : 'nothing'} (says '${body.find(b => b.stuff !== word)?.stuff}')` : ''}${unfinished ? `, ${unfinished} not of roughness ${rough}` : ''}`);
    if (body.some(b => (b.albedo as readonly number[]) !== GWOT_PALETTE[row] && paintOf(b.albedo)?.does !== doesOfName(row))) stoodIn++;   // (a stand-in: the row's numbers as a paint of their own, that does another thing than the row)
  }
  check(wrong.length === 0 && stoodIn > 0 && coated > 0 && coated < ROWS.length * 2, `1 a drum, standing or lying, in any of the palette's ${ROWS.length} rows says it does what steel does: metal to a round's hit, by what it does and by its word, and to a blast; it wears the row to the eye and is known as the row's paint; it says the row's word where the row is a coat or metal and 'paint' with an enamel's finish in any other`,
    `${ROWS.length * 2} drums laid, ${stoodIn} of them in a stand-in, ${coated} of them a coat over a row that is neither a coat nor metal${wrong.length ? ' · ' + wrong.slice(0, 5).join(' · ') + (wrong.length > 5 ? ` · and ${wrong.length - 5} more` : '') : ''}`); }

// 2. the drums a level is dealt (dealt at several places: a place deals by its own seed)
{ const body: Box[] = []; for (const [x, z] of [[10, 10], [14.5, 7.25], [22, 31], [3.3, 18.9], [27.1, 5.6], [8.9, 20.3]]) body.push(...laid(G => BREAK ? G.crates(x, z, 0, 3, 'olive') : G.drums(x, z, 6)).boxes.filter(solid));
  const paints = [...new Set(body.map(b => paintOf(b.albedo)?.name ?? '?'))].sort(), notMetalRow = paints.filter(n => n in GWOT_PALETTE && doesOfName(n) !== 'steel');
  const notSteel = body.filter(b => readAs(b).length);
  check(body.length >= 36 * 4 && notSteel.length === 0 && paints.length >= 3 && notMetalRow.length > 0, '2 the drums `drums` deals are steel, each of them, in the paints it deals from, one at least of which is not metal by its row',
    `${body.length} boxes of thirty-six drums in ${paints.join(', ')} (not metal by the row: ${notMetalRow.join(', ') || 'NONE: the line asks nothing'}) · ${notSteel.length} not metal to a round${notSteel.length ? ` (${[...new Set(notSteel.flatMap(readAs))].join(' · ')})` : ''}`); }

// 3. the drum on its side (in a drum's enamel; in a bare paint, so that the trim's bare metal is asked; in a row that is no coat, so that the trim's word is)
{ const BARE = ROWS.find(r => GWOT_FINISH[r]?.metal) ?? 'galv';
  for (const colour of ['drumRed', BARE, 'mud'] as const) { const L = laid(G => BREAK ? (G.woodPile(10, 10, 0, 1.4), undefined) : G.drumDown(10, 10, 0, colour)), { K, G, boxes } = L, topOf: any = L.back, n0 = K.boxes.length - boxes.length;
    const body = boxes.filter(b => solid(b) && top(b) > 0.2), chocks = boxes.filter(b => solid(b) && top(b) <= 0.2);
    const d: any = body[0], sh = d?.shape, trim: any[] = [...(sh?.with ?? [])], r = d ? d.h[1] : 0, bare = !!d?.metal;
    const ends = trim.filter(t => Math.abs(Math.abs(t.off[0]) - d.h[0]) < 0.01), ribs = trim.filter(t => Math.abs(t.off[0]) < d.h[0] - 0.05);
    const trimOk = trim.length === 4 && ends.length === 2 && ribs.length === 2 && trim.every(t => t.round === 1 && t.albedo && Math.max(t.h[1], t.h[2]) <= r + 0.02 && t.h[1] === t.h[2] && !!t.metal === bare && t.stuff === d.stuff);
    const one = body.length === 1 && sh?.round === 1 && !sh.arc && !sh.cut && Math.abs(d.h[1] - d.h[2]) < 1e-9 && Math.abs(d.c[1] - d.h[1]) < 1e-9 && d.h[0] > d.h[1] && !d.marks?.length && !d.rot;
    // a chock wedges: at its top, the round's flank is no further out than the chock reaches in (the round's flank at height y: sqrt(r² − (r − y)²))
    const wedged = chocks.length === 2 && d && chocks.every(c => { const y = top(c), flank = Math.sqrt(Math.max(0, r * r - (r - y) * (r - y))), lz = Math.abs(c.c[2] - d.c[2]); return y < 0.1 && lz - c.h[2] < flank - 0.01 && lz + c.h[2] > d.h[2]; });
    const handed = !!topOf && topOf.shape === sh && topOf.h[1] === r && topOf.c[1] === r;
    let follows = false, laidN = 0;
    if (handed) { const before = (sh.with ?? []).length; G.lay(10, 10, 0, topOf, 0, 0, 2 * r, 2 * r + 0.04, 0.37, r * 0.6, G.M.dust); const added = (sh.with as any[]).slice(before); laidN = K.boxes.length - n0 - boxes.length;
      follows = added.length === 1 && added[0].arc?.axis === 1 && added[0].arc.rise > 0 && laidN === 1; }
    check(one && trimOk && wedged && handed && follows && (colour !== BARE || bare),
      `3 the drum on its side${colour === BARE ? `, in a bare paint (${BARE}),` : colour === 'mud' ? ', in a row that is neither a coat nor metal (mud),' : ''} is one solid box that says round about its length, square across it and standing on the ground, with two ends and two ribs of trim that say what the drum does and are bare where it is, on two chocks that reach in under its flank, and no markings; it hands back its top, and what is laid along that top is a ridge and a sheet that follows the round`,
      `${body.length} box${body.length === 1 ? '' : 'es'} in the drum's paint, shape ${JSON.stringify(sh ? { round: sh.round, with: trim.length } : null)}, ${d ? `${(d.h[0] * 2).toFixed(2)} long, ${(d.h[1] * 2).toFixed(2)} by ${(d.h[2] * 2).toFixed(2)} across, foot at ${(d.c[1] - d.h[1]).toFixed(3)}, ${bare ? 'bare' : 'painted'}` : 'none'} · trim: ${ends.length} ends, ${ribs.length} ribs, ${trim.filter(t => t.metal).length} bare · ${chocks.length} chocks${wedged ? ', wedged' : ', NOT under the flank'} · top handed back ${handed} · laid along it: ${laidN} ridge${follows ? ' and a sheet about the same axis' : ', no sheet that follows'}`); } }

console.log(failed() ? `=== kit stores: ${failed()} FAILED ===` : '=== kit stores: ALL PASS ===');
process.exit(failed() ? 1 : 0);
