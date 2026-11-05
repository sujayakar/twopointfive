# The library: every object and every paint, once

WRITTEN BY `bun run tools/levels/library.ts --write` FROM THE KITS. Do not edit: change the kit, then write it again (`--check` is red when this page is stale).

186 pieces in 6 kits, 115 named paints in 6 tables.
A piece is something a level asks a kit for by name. Each row is one call laid alone: the call a showroom makes.

- **To a man**, read off the boxes as laid: *solid to h* = feet, sight, rounds and sound meet it, up to h metres; *off the floor* = solid, but nothing of it comes
  down to the floor (it is laid on furniture or on a wall); *glass* = sight passes it, feet and sound do not; *walked over* = solid but under 0.2 m;
  *loose* = a body a man shoves and a blast throws; *drawn only* = no ray and no foot meets it; *lit* = it adds a lamp or a lit fixture.
- **Size** is length × width × height in metres of the call that was laid. A wall, a rug or a line is as long as it was asked for.
- **Boxes** is what the piece costs: solid boxes, drawn-only boxes, loose things and their parts, round things the round rule finds, marking bitmaps, lamps.
- **Paints** are rows of the tables under Paints below, most boxes first; `row × 0.60` is a row darkened; a colour a kit writes as numbers shows as its numbers.
- **The showrooms**: `?levelFile=gwot-yard.json&solo&hour=midday` and `?levelFile=library-hall.json&solo&hour=midday`. Between them one of each piece stands, and its line in the
  showroom's generator is the call to copy.
- Which level lays which piece is not on this page (it moves with every generator): `bun run tools/levels/library.ts --uses FILE` writes it.

## The kits

| Kit | File | Pieces | Its sections |
|---|---|---|---|
| gwotKit | `src/scene/gwotKit.ts` | 79 | walls · what is done to a wall's face · ground · cloth · furniture · stores & cover · machines & vehicles · lights |
| officeKit | `src/scene/officeGen.ts` | 23 | glass · walls · props · loose props · set dressing helpers · lights · doors · what is fixed to a wall's face |
| livedKit | `tools/levels/livedKit.ts` | 18 | the floor · furniture · clutter · loose things |
| livedPieces | `tools/levels/livedPieces.ts` | 37 | carcasses · paper, plans, books · instruments and tools · safes · on a wall · on a desk · on the floor · loose |
| livedQuiet | `tools/levels/lived-quiet.ts` | 11 |  |
| livedTrain | `tools/levels/lived-train.ts` | 18 | furniture · loose things · the seven signature pieces |

## One name, more than one piece

The same word in more than one kit: each is built its own way, in its own paint. This is the list the library has to bring down to one piece a name.

| Name | Kits | Sizes |
|---|---|---|
| bench | officeKit, livedQuiet | 1.4 × 0.4 × 0.45 · 1.3 × 0.32 × 0.44 |
| bucket | livedKit, livedPieces, livedTrain | 0.28 × 0.28 × 0.3 · 0.3 × 0.26 × 0.3 · 0.26 × 0.26 × 0.29 |
| carton | livedKit (officeKit.cardboardBox), livedQuiet | 0.45 × 0.45 × 0.45 · 0.5 × 0.36 × 0.31 |
| chest | livedPieces, livedTrain | 1.06 × 0.66 × 0.9 · 0.58 × 0.38 × 0.32 |
| cone | livedKit, livedPieces | 0.34 × 0.34 × 0.56 · 0.3 × 0.3 × 0.49 |
| crate | livedKit, livedPieces | 0.8 × 0.5 × 0.47 · 0.5 × 0.5 × 0.42 |
| drum | gwotKit, livedKit | 0.56 × 0.56 × 0.88 · 0.56 × 0.56 × 0.89 |
| ladder | gwotKit, livedPieces | 0.6 × 0.44 × 2.84 · 2.4 × 0.46 × 0.07 |
| mat | livedKit, livedQuiet | 1.4 × 1 × 0.02 · 1.6 × 0.9 × 0.01 |
| mug | livedKit, livedQuiet | 0.07 × 0.07 × 0.09 · 0.08 × 0.08 × 0.1 |
| pot | gwotKit, livedTrain | 0.34 × 0.34 × 0.14 · 0.4 × 0.4 × 0.34 |
| rug | gwotKit, officeKit | 3.2 × 3 × 0.01 · 2.4 × 1.6 × 0.01 |
| shelf | officeKit, livedQuiet | 1.6 × 0.4 × 1.9 · 2 × 0.36 × 1.5 |
| stool | livedKit, livedPieces, livedQuiet, livedTrain | 0.34 × 0.34 × 0.47 · 0.36 × 0.36 × 0.48 · 0.34 × 0.34 × 0.34 · 0.34 × 0.34 × 0.42 |
| table | gwotKit, livedKit, livedPieces, livedTrain | 1.6 × 0.75 × 0.75 · 1.8 × 0.7 × 0.74 · 2 × 0.9 × 0.74 · 1.8 × 0.8 × 0.75 |
| tyres | gwotKit, livedKit | 0.68 × 0.64 × 0.6 · 0.68 × 0.68 × 0.6 |

## One piece, handed out by more than one kit

A piece that lays nothing of its own, only one piece of another kit: the same piece, taken into that kit (a lived kit names it as one of a room's lived things).
It is no second way of building the name.

| Piece | Is |
|---|---|
| gwotKit.wallX | officeKit.wallX |
| gwotKit.wallZ | officeKit.wallZ |
| gwotKit.partX | officeKit.wallX |
| gwotKit.partZ | officeKit.wallZ |
| livedKit.papers | officeKit.papers |
| livedKit.chair | officeKit.chair |
| livedKit.carton | officeKit.cardboardBox |

## gwotKit

`src/scene/gwotKit.ts`

### gwotKit: walls

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **wallX**(z, x0, x1, kind = 'mud', openings = [], height = H) | a SHELL wall along X at fixed z (0.24 thick: mud-brick, render, whitewash, bare block); openings as the kits' own (doors 0‥2.2, windows 0.9‥2.4) *(is `officeKit.wallX`)* | solid to 3 | 10.24 × 0.24 × 3 | 1 solid | mud | gwot-yard, library-hall |
| **wallZ**(x, z0, z1, kind = 'mud', openings = [], height = H) | a SHELL wall along X at fixed z (0.24 thick: mud-brick, render, whitewash, bare block); openings as the kits' own (doors 0‥2.2, windows 0.9‥2.4) *(is `officeKit.wallZ`)* | solid to 3 | 5.76 × 0.24 × 3 | 1 solid | mud | gwot-yard |
| **partX**(z, x0, x1, kind = 'mudLight', openings = [], height = H) | an interior PARTITION (0.12 thick) *(is `officeKit.wallX`)* | solid to 3 | 4 × 0.12 × 3 | 1 solid | whitewash | library-hall |
| **partZ**(x, z0, z1, kind = 'mudLight', openings = [], height = H) | an interior PARTITION (0.12 thick) *(is `officeKit.wallZ`)* | solid to 3 | 5.76 × 0.12 × 3 | 3 solid | mudLight | gwot-yard |
| **yardWallX**(z, x0, x1, kind = 'mud', openings = [], height = 2.4) | a COMPOUND (yard) wall along X: the shell wall to `height` (2.4: over a man, under the roofs) with a paler coping course overhanging it, open to the sky | solid to 2.4 | 2 × 0.3 × 2.47 | 2 solid | mud, mudLight | gwot-yard |
| **yardWallZ**(x, z0, z1, kind = 'mud', openings = [], height = 2.4) | a COMPOUND (yard) wall along Z at fixed x: as yardWallX | solid to 2.4 | 16 × 0.3 × 2.47 | 4 solid | mud, mudLight | gwot-yard |
| **pier**(x, z, height = 2.4, kind = 'mud') | a buttress PIER on a wall line (0.44 square, capped) — the rhythm of a long mud wall, and a corner to stand behind | solid to 2.44 | 0.52 × 0.52 × 2.52 | 2 solid | mud, mudLight | gwot-yard |
| **skirt**(axis, line, a, b, side, mat = 'mudDark', top = 0.38, t = GW_SHELL) | DECAL: a band on a wall face from the floor to `top` — the damp, darker foot of a mud wall (default), or a painted DADO (`mat` a paint, top 1.1). `axis` 'x' = the wall runs along X at `line`; a‥b along it; `side` ±1 = which face (toward +z / +x is +1); `t` = that wall's thickness | drawn only | 10.3 × 0.01 × 0.38 | 1 drawn | mudDark | gwot-yard |
| **dado**(axis, line, a, b, side, paint = 'paintBlue', t = GW_SHELL) | DECAL: a painted DADO: the band of `skirt` in a paint, to 1.1 m up the wall's face | drawn only | 5.32 × 0.01 × 1.1 | 1 drawn | paintBlue | gwot-yard |
| **wallHanging**(axis, line, at, side, style = 'baluch', w = 1.2, hgt = 0.8, t = GW_SHELL) | DECAL: a small rug / cloth hung flat on a wall face, its middle at `at` along the wall, 1.2‥2.0 m up | drawn only | 1.2 × 0.01 × 0.8 | 1 drawn, 1 marking | rugMadder | gwot-yard |

### gwotKit: what is done to a wall's face

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **gate**(x, z, axis, width = 3.0, side = 1, paint = 'paintBlue', openDeg = 78, height = 2.4) | a sheet-metal GATE in an opening `width` wide centred at (x, z) of a wall along `axis`: two capped posts and two painted leaves standing open by `openDeg` toward `side` (static: a gate that is open tonight). The wall's opening is the caller's (a‥b = centre ∓ width / 2, y1 ≥ the wall's height). | solid to 2.5 | 3.74 × 1.64 × 2.58 | 6 solid, 8 markings | mudLight, paintBlue, render | gwot-yard |
| **boards**(axis, line, a, b, side, form = 'shut', y0 = 0.9, y1 = 1.85, t = GW_SHELL) | a window BOARDED UP (the opening a‥b of a wall along `axis`, y0‥y1): shut = two planks nailed across, one across those, and one hanging by its last nail — a look and a round pass BETWEEN the planks, not through them. Open = the boards pulled off the opening: one left nailed over the lintel and two swung down on their last nails to hang on the wall beside the jambs. Neither form puts anything on the ground | off the floor | 1.69 × 0.05 × 0.85 | 4 solid | woodGrey, ply, poplar | library-hall |
| **shutters**(axis, line, a, b, side, form = 'shut', paint = 'paintGreen', y0 = 0.9, y1 = 1.85, t = GW_SHELL) | a window's SHUTTERS, two leaves in a paint: shut = one leaf shut over its half and the other swung out on its hinge (the half behind the shut leaf is blind; the leaves are real). Open = both folded back flat on the wall beside the opening | off the floor | 1.49 × 0.54 × 1.01 | 2 solid, 2 drawn | paintGreen, paintGreen × 0.90 | library-hall |
| **windowBlanket**(axis, line, a, b, side, form = 'shut', cloth = 'blanketGrey', y0 = 0.9, y1 = 1.85, t = GW_SHELL) | a BLANKET over a window: shut = nailed over the opening under a batten, one lower corner come loose (it is real: a look stops at it). Open = rolled up and tied over the lintel, the opening clear | off the floor | 1.48 × 0.03 × 1.2 | 1 solid, 1 drawn | blanketGrey, woodGrey | library-hall |
| **wallPatch**(axis, line, at, side, y, w, hgt, mat = 'mudLight', tone = 0.9, t = GW_SHELL) | DECAL: a MEND on a wall's face — a patch of newer or older mud, w × h about `y`, with a rough outline (said as a shape: with shapes off it is the plain rectangle). `tone` darkens the wall paint `mat` in one piece (0.55‥0.99; 1 = the paint as it is). It lies OVER a dado or a skirt (a painted band plastered over) | drawn only | 1 × 0.01 × 1.1 | 1 drawn | mudLight × 0.90 | library-hall |
| **spout**(axis, line, at, side, y = 2.36, wall = 'mud', t = GW_SHELL) | a roof SPOUT through the wall at `y`, tipped to throw its water clear, and the STREAK the water left down the face (a decal in the wall's own paint, darker) | overhead | 0.26 × 0.2 × 2.36 | 1 solid, 1 drawn | GWOT_PALETTE.iron, mud × 0.60 | library-hall |
| **beamEnds**(axis, line, a, b, side, y = 2.64, pitch = 1.9, t = GW_SHELL) | the ENDS OF THE ROOF'S BEAMS through a wall a‥b under the roof line (over every head): round poles, unevenly spaced and of uneven reach, one snapped short | overhead | 14.08 × 0.4 × 0.18 | 8 solid | woodGrey | library-hall |
| **splitWood**(axis, line, a, b, side, rows = 3, t = GW_SHELL) | COVER (knee): SPLIT WOOD stacked against a wall from a to b, `rows` high: round billets end-on to the yard, pale grain showing, each a box of its own. No two billets that touch are as thick, as long or as far out (their girth, length and set-back step through small tables by place and row, and every face lands on an odd millimetre, off the centimetre grid levels are written on): with shapes off they are boxes that overlap, and two of their faces in one plane would flicker | solid to 0.48 | 0.69 × 0.5 × 0.48 | 9 solid | woodGrey, poplar | library-hall |
| **ladder**(axis, line, at, side, len = 2.84, leanDeg = 15, t = GW_SHELL) | a LADDER leaned on a wall at `at`: two stiles and the rungs between them, one rung gone; its feet stand `len` × sin(lean) out from the face | solid to 2.79 | 0.6 × 0.44 × 2.84 | 8 solid | woodGrey, poplar | library-hall |

### gwotKit: ground

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **ground**(x0, z0, x1, z1, mat = 'dust', y = lie.finish) | a GROUND finish over a rectangle (a thin solid slab like the kits' floor finishes: feet, rounds and light land on it): dust \| dirt \| gravel \| mudFloor \| slab | walked over | 24 × 16 × 0 | 1 solid | dirt | gwot-yard, library-hall |
| **patch**(x, z, w, d, mat = 'damp', yaw = 0, y = flatAt(lie.patch)) | DECAL: a patch on the ground — a damp spot, an oil stain, a drift of dust, a scatter of gravel (w × d, any yaw). ANOTHER row's stuff lying on the ground; for the same ground darker (wet, worn) use `stain` | drawn only | 3.6 × 2.6 × 0.01 | 1 drawn | gravel | gwot-yard |
| **ruts**(x0, z0, x1, z1, mat = 'dirt') | DECAL: a vehicle's two wheel RUTS from (x0, z0) to (x1, z1), 1.5 m apart | drawn only | 6.6 × 1.82 × 0.01 | 2 drawn | dirt | gwot-yard |
| **rubble**(x, z, r = 0.8, n = 7, mat = 'mud') | WALK-OVER: a scatter of rubble / field stones within r of (x, z) — n small blocks under 0.19 m (lit and shadowed; nobody trips on them). ALL OF IT IS `mat`: three stones in ten are its darker row where the palette holds one (gwotPalette.ts GWOT_DARKER: mud's is mudDark), standing for what `mat` is to a round, so a scatter is one thing in two tones and never a second paint; a stone has the row's whole finish (bits of tin sheet are metal). (Those three were mudDark whatever was asked: pale clods among dark gravel, mud among offcuts of poplar.) | walked over | 1.54 × 1.41 × 0.17 | 9 solid | mud, mudDark | gwot-yard |
| **stain**(x, z, w, d, ground, yaw = 0, k = 0.66, y = flatAt(lie.stain)) | DECAL: a STAIN that is its ground's own paint darker — wet concrete under a tap, a floor walked dark under a bench. `ground` = the finish it lies on, `k` of it (STYLE.md: a patch is darker than its ground and never under a third of it; see `toned`). Wet reads smoother than dry. IT LIES UNDER A VEHICLE'S RUTS AND UNDER A PATCH (`lie`: a stain is the ground, and what was driven or laid over the ground shows over it; at the patch's height it painted the ruts out where they crossed). `y` = its centre, for the one that is meant to lie on something: `flatAt(lie.patch)` is soot over a damp spot | drawn only | 1.8 × 1.2 × 0.01 | 1 drawn | dust × 0.66 | library-hall |
| **seep**(x0, z0, x1, z1, mat = 'dirt') | DECAL: a SEEP — where water runs out under a wall and stands: darker earth from (x0, z0) to (x1, z1), in lengths that overlap and wander. The line is the stain's MIDDLE and a length is up to 1.15 m across: lay it 0.6 m out from the wall's face, not on the wall's own line. `mat` is darker than the ground it lies on (dirt on dust, damp on dirt) | drawn only | 3.12 × 1.15 × 0.01 | 2 drawn | dirt | library-hall |
| **kerbStones**(x0, z0, x1, z1, paint = 'whitewash', pitch = 1.15) | WALK-OVER: KERB STONES — painted stones from (x0, z0) to (x1, z1), `pitch` metres from one to the next (a stride by default; 0.3 = stone against stone): the edge of a government yard, the side of a track (each 0.26 long, under 0.14 m high) | walked over | 3.65 × 0.44 × 0.11 | 4 solid | whitewash | library-hall |
| **kerb**(x0, z0, x1, z1, mat = 'block', gaps = [], len = 1.9) | WALK-OVER: a KERB from (x0, z0) to (x1, z1) — precast stones about `len` long, 0.3 wide and 0.14 high, laid end to end a finger apart and none quite in line (lit and shadowed; stepped over). The run is cut into round(length / len) stones, at least one; `gaps` = the stones left out, counted from the first end (a broken one, a dropped kerb where wheels cross). A run under 0.3 m, or a `len` under 0.3, throws: that is no kerb | walked over | 1.83 × 0.3 × 0.14 | 1 solid | block | library-hall |
| **plantBed**(x, z, len = 4.4, wid = 1.3, yaw = 0, rows = 3) | WALK-OVER: a PLANT BED, `len` along local x by `wid`: a kerb of blocks round turned earth, `rows` of seedlings with a gap in each where some failed (all of it under 0.19 m) — a household's vegetables, a station's trial plot | walked over | 4.4 × 1.3 × 0.16 | 11 solid | prodGreen, block, dirt | library-hall |
| **hearth**(x, z, yaw = 0, ground = groundAt(x, z)) | WALK-OVER: a cold HEARTH — a ring of stones, two charred lengths, the ash trodden round it: where a watch sat last night. The trodden ash is a `stain`: the ground's own paint, darker. `ground` = the finish the fire was lit on: what the kit laid there (`groundAt`). Where the kit laid none (a level's own slab) the caller names it, and a hearth that is told nothing there THROWS: it does not guess. (It was a patch of `damp` whatever it lay on: a black square on moonlit dust.) | walked over | 1.5 × 1.3 × 0.15 | 9 solid, 1 drawn | mudDark, woodGrey, dust × 0.66 | library-hall |

### gwotKit: cloth

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **rug**(x, z, w, d, style = 'afghan', yaw = 0) | DECAL: a patterned RUG, w along local x × d along local z (each ≤ 3.2 m for the whole pattern; snapped to 10 cm): 'afghan' red field with navy guls, 'baluch' navy with red, 'kilim' striped cream, 'mat' a woven plastic mat, 'felt' a grey namad | drawn only | 3.2 × 3 × 0.01 | 1 drawn, 4 markings | rugRed | gwot-yard |
| **toshak**(x, z, len = 1.9, yaw = 0, cover = 'rugRed', cushion = 'rugNavy') | WALK-OVER: a floor mattress (toshak) `len` long by 0.7 with two flat cushions on it — what a guest room is lined with; all of it under 0.19 m | walked over | 1.9 × 0.7 × 0.19 | 3 solid, 4 markings | rugNavy, rugRed | gwot-yard |
| **cot**(x, z, yaw = 0, made = true, blanket = 'blanketGrey') | COVER (knee high): a steel-framed army COT, 1.9 × 0.7, the canvas at 0.42 m; `made` = a folded blanket and a pillow on it. Head toward local +X. | solid to 0.5 | 1.9 × 0.7 × 0.5 | 6 solid, 1 marking | galv, blanketGrey, GWOT_PALETTE.canvas, sack | gwot-yard |
| **charpoy**(x, z, yaw = 0, quilt = 'rugRed') | COVER (knee high): a CHARPOY — the rope bed: four turned legs, a poplar frame 1.9 × 0.9, the woven top, a quilt folded at the foot | solid to 0.52 | 1.9 × 0.9 × 0.52 | 6 solid, 3 markings | woodGrey, poplar, rugRed | gwot-yard |
| **washLine**(x0, z0, x1, z1, n = 4, posts = true) | DECAL: washing on a LINE between two posts (the posts are thin statics; the line and the cloths are decals at 1.3‥1.95 m) | solid to 2 | 4.36 × 0.06 × 2 | 2 solid, 6 drawn | rugCream, woodGrey, blanketBrown, GWOT_PALETTE.iron, paintGreen | gwot-yard |

### gwotKit: furniture

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **lowTable**(x, z, yaw = 0, tea = true) | COVER (shin high): a LOW TABLE 1.0 × 0.6 × 0.3 with a tea tray, a pot and two glasses | solid to 0.43 | 1 × 0.6 × 0.43 | 7 solid | poplar, whitewash, GWOT_PALETTE.iron, GWOT_PALETTE.tin | gwot-yard |
| **table**(x, z, w = 1.6, d = 0.75, yaw = 0, top = 'ply') | COVER: a plain TABLE w × d at 0.75 m on four legs (plywood on a timber frame — the FOB's desk, a kitchen table) | solid to 0.75 | 1.6 × 0.75 × 0.75 | 5 solid | woodGrey, ply | gwot-yard, library-hall |
| **seedTray**(x, z, baseY = 0.75, w = 0.62, d = 0.3, yaw = 0) | COVER (none of its own: it stands on a bench and adds a hand to its height): a SEED TRAY w × d on something `baseY` high (a table, 0.75) — a black plastic tray of seedlings | off the floor | 0.62 × 0.3 × 0.11 | 2 solid | plasticBlack, prodGreen | library-hall |
| **plasticChair**(x, z, yaw = 0, colour = 'plasticWhite') | PROP (chair): a moulded PLASTIC CHAIR — seat, back, two arms, four splayed legs; the sitter faces local −Z | loose | 0.49 × 0.45 × 0.82 | 1 loose (8 parts) | plasticGreen | gwot-yard |
| **chairAs**(x, z, yaw, seat = 'plasticWhite', foot = seat) | PROP (chair): the OFFICE chair's own five boxes in other paint — same footprint, mass and shape, so a finished level can swap its chairs' look and keep every run as it was (the pedestal reads as a plastic stool's foot) | loose | 0.51 × 0.5 × 1.1 | 1 loose (5 parts) | plasticBlue | gwot-yard |
| **trunk**(x, z, yaw = 0, paint = 'paintGreen') | COVER (knee high): a painted tin TRUNK 0.8 × 0.45 × 0.4 with strap bands and a hasp | solid to 0.4 | 0.8 × 0.45 × 0.4 | 1 solid, 2 markings | paintGreen | gwot-yard |
| **stove**(x, z) | COVER: a cast-iron STOVE (bukhari) with its flue to the roof — the winter corner of a room | solid to 3 | 0.4 × 0.4 × 3 | 8 solid, 1 round | rust, GWOT_PALETTE.iron, GWOT_PALETTE.tin | gwot-yard |
| **tandoor**(x, z) | COVER: a clay bread OVEN (tandoor), a mud drum 0.9 across and 0.55 high with a black mouth | solid to 0.55 | 1.5 × 1.4 × 0.55 | 4 solid, 1 drawn, 1 round, 1 marking | mudDark, damp | gwot-yard |

### gwotKit: stores & cover

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **drum**(x, z, colour = 'drumBlue', yaw = 0) | COVER: an OIL DRUM (0.58 across, 0.88 high) standing; ribs round it, a bung on the lid | solid to 0.88 | 0.56 × 0.56 × 0.88 | 4 solid, 1 round, 16 markings | drumRed | library-hall |
| **drums**(x, z, n = 3, yaw = 0) | a group of drums: n standing in a tight cluster from (x, z), colours dealt by position | solid to 0.88 | 1.54 × 1.18 × 0.88 | 16 solid, 4 round, 64 markings | rust, drumBlue, olive | gwot-yard |
| **drumDown**(x, z, yaw = 0, colour = 'rust') | COVER (to the chest lying, the knee standing): a drum on its SIDE, chocked | solid to 0.58 | 0.89 × 0.64 × 0.58 | 4 solid, 1 marking | rust, woodGrey | gwot-yard |
| **jerryCan**(x, z, yaw = 0, colour = 'olive') | PROP (bin): a JERRY CAN, 20 l — olive steel, or the yellow / blue plastic ones that carry everything | loose | 0.34 × 0.16 × 0.45 | 1 loose (3 parts), 2 markings | olive, GWOT_PALETTE.iron | gwot-yard |
| **gasBottle**(x, z, colour = 'drumRed') | PROP (bin): a GAS BOTTLE (LPG), 0.3 across with its collar | loose | 0.4 × 0.4 × 0.62 | 1 loose (3 parts) | drumRed, GWOT_PALETTE.iron | gwot-yard |
| **crates**(x, z, yaw = 0, n = 2, colour = 'olive') | COVER: wooden AMMUNITION CRATES, 0.9 × 0.4 × 0.3 each, n stacked (1 = shin, 3 = a waist-high rest): olive, battens across the lid, a stencil block | solid to 0.9 | 0.94 × 0.47 × 0.9 | 3 solid, 12 markings | olive | gwot-yard |
| **sacks**(x, z, yaw = 0, n = 3) | COVER: a pile of grain / flour SACKS, n layers of 0.2 m (3 = a kneeling man's cover) | solid to 0.8 | 1.01 × 0.99 × 0.8 | 8 solid, 8 markings | sack | gwot-yard |
| **sandbags**(x, z, len, yaw = 0, courses = 4) | COVER: a SANDBAG wall, `len` along local x, `courses` of 0.2 m (4 = 0.8 m, a crouched man's cover; 6 = 1.2; 7+ blocks a standing man's sight), 0.5 thick, laid in bond; long runs are cut into ≤ 1.6 m lengths | solid to 0.8 | 2.4 × 0.5 × 0.8 | 8 solid, 18 markings | sandbag | gwot-yard |
| **hesco**(x, z, n = 2, yaw = 0) | COVER (full): HESCO bastions — n wire-and-liner cells of 1.1 m, 1.37 high, filled with earth, in a row along local x | solid to 1.41 | 2.19 × 1.1 × 1.41 | 4 solid, 8 markings | dirt, hesco | gwot-yard |
| **jersey**(x, z, n = 1, yaw = 0) | COVER (crouch): JERSEY barriers — n precast 2 m lengths, 0.81 high, in a row along local x | solid to 0.81 | 2 × 0.6 × 0.81 | 2 solid, 2 markings | tWall | gwot-yard |
| **tWall**(x, z, n = 3, yaw = 0, height = 2.9) | COVER (full): T-WALLS — n precast blast-wall segments 1.2 m wide and `height` tall, each on its foot, in a row along local x | solid to 2.9 | 4.78 × 0.84 × 2.9 | 8 solid | tWall | gwot-yard |
| **tyres**(x, z, n = 3) | COVER (waist): a stack of TYRES, n high (0.2 m each). One lying alone is a fat one, 0.27 m (as the older kit's is): a solid that tops out between 0.20 and 0.25 m stops a body on a cell the nav grid calls free | solid to 0.6 | 0.68 × 0.64 × 0.6 | 12 solid, 3 round, 1 marking | GWOT_PALETTE.rubber | gwot-yard |
| **blocks**(x, z, yaw = 0, layers = 2) | COVER (knee): a stack of concrete BLOCKS 0.8 × 0.4 × 0.4, cores up | solid to 0.4 | 0.8 × 0.4 × 0.4 | 1 solid, 3 markings | block | gwot-yard |
| **woodPile**(x, z, yaw = 0, len = 1.4) | COVER (knee): a WOOD PILE — split poplar, three layers | solid to 0.42 | 1.41 × 0.6 × 0.42 | 3 solid, 9 markings | woodGrey | gwot-yard |
| **pipes**(x, z, yaw = 0, n = 3, len = 1.8, r = 0.16) | COVER (knee to thigh): steel PIPES stacked along local X — n = 1‥3: two on the ground side by side and the third in the notch between them; one of the ground pair is rust. Their bores are dark at both ends. Hands back each pipe's top (`lay` puts snow along it) | solid to 0.59 | 1.89 × 0.65 × 0.59 | 3 solid | GWOT_PALETTE.iron, rust | library-hall |
| **log**(x, z, yaw = 0, len = 1.0, r = 0.095, baseY = 0, bark = 'woodGrey', grain = 'poplar') | one LOG lying along local X, bark on, its ends pale grain. WALK-OVER at the default (0.19 across: a top between 0.20 and 0.25 m stops a body on a cell the nav grid calls free, so a single log is under that or over it); `baseY` lifts it onto a pile. Hands back its top | walked over | 1 × 0.19 × 0.19 | 1 solid | woodGrey | library-hall |
| **choppingBlock**(x, z, yaw = 0) | COVER (knee): a CHOPPING BLOCK — a round of poplar 0.44 across and 0.32 high, its top scored | solid to 0.32 | 0.42 × 0.42 × 0.32 | 4 solid, 1 round, 1 marking | poplar | library-hall |
| **plankBench**(x, z, yaw = 0, len = 1.12) | COVER (knee): a PLANK BENCH — a board across two stacks of mud brick, `len` long along local X | solid to 0.41 | 1.15 × 0.35 × 0.41 | 3 solid | mud, mudLight, poplar | library-hall |
| **bricks**(x, z, yaw = 0, n = 5) | WALK-OVER: loose MUD BRICKS left where the work stopped, n of them, some on the others | walked over | 0.95 × 0.49 × 0.18 | 6 solid | mud, mudLight | library-hall |
| **pot**(x, z, baseY = 0) | a cooking POT, sooted iron, 0.26 across (under 0.2 m of radius a round is two squares: `roundSlabs`); `baseY` stands it on a bench or a stove | off the floor | 0.34 × 0.34 × 0.14 | 2 solid | GWOT_PALETTE.iron | library-hall |

### gwotKit: machines & vehicles

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **generator**(x, z, yaw = 0, colour = 'genYellow') | COVER (waist): a diesel GENERATOR on its skid, 1.5 × 0.75 × 1.0: louvred sides, a control panel on local −Z, the exhaust stack | solid to 1.32 | 1.6 × 0.8 × 1.32 | 4 solid, 4 markings | GWOT_PALETTE.iron, genYellow, rust | gwot-yard |
| **waterTank**(x, z, colour = 'plasticBlue', plinth = 1.1) | COVER (full): a WATER TANK — a 1 000 l plastic tank on a mud-brick plinth (`plinth` m high; 0 = on the ground) | solid to 2.2 | 1.32 × 1.32 × 2.26 | 7 solid, 1 round, 16 markings | plasticBlue, galv, mud, plasticBlack | gwot-yard |
| **pump**(x, z, yaw = 0) | COVER: a hand PUMP on its concrete pad (the pad a walk-over) with a bucket | solid to 1.08 | 2.82 × 2.4 × 1.08 | 6 solid, 2 drawn | paintGreen, damp, block, GWOT_PALETTE.iron, plasticBlue | gwot-yard |
| **pickup**(x, z, yaw = 0, body = 'truckWhite', opts = {}) | COVER (full height at the cab): a PICKUP, 5.0 × 1.8, nose toward local +X: cab, open bed, tailgate. `load`: what rides in the bed; `mount`: a pedestal gun in the bed (the "technical") | solid to 1.71 | 5.04 × 1.82 × 1.71 | 32 solid, 3 round, 2 markings | drumBlue, truckWhite, GWOT_PALETTE.rubber, rust, plasticBlack, plasticWhite, truckRed, glassDark | gwot-yard |
| **van**(x, z, yaw = 0, body = 'truckWhite') | COVER (head high): a PANEL VAN 4.9 × 1.8 × 1.95, nose toward local +X, its two back doors standing open on the load bay (the spot a stride behind them is local (−3.2, 0): where a man is put in) | solid to 1.95 | 5.63 × 2.65 × 1.95 | 23 solid | truckWhite, GWOT_PALETTE.rubber, plasticBlack, plasticWhite, truckRed, glassDark | library-hall |
| **lorry**(x, z, yaw = 0) | COVER (full height): a TWO-AXLE LORRY with a canvas tilt, 7 m, nose toward local +X (the pass's, where it stopped when the road shut). Like every vehicle here it hands back the shapes its boxes say (scene/carShapes.ts), `tops` among them: what a level lays on its roof goes by `lay` | solid to 1.95 | 6.68 × 2.34 × 2.2 | 20 solid | GWOT_PALETTE.rubber, poplar, truckRed, GWOT_PALETTE.iron, plasticWhite, glassDark, GWOT_PALETTE.canvas, woodGrey | gwot-yard |
| **motorbike**(x, z, yaw = 0, tank = 'truckRed') | COVER (waist, thin): a small MOTORBIKE on its stand, nose toward local +X | solid to 1.04 | 1.96 × 0.64 × 1.04 | 10 solid | GWOT_PALETTE.iron, GWOT_PALETTE.rubber, galv, plasticBlack, truckRed | gwot-yard |
| **handcart**(x, z, yaw = 0, load = 'none') | COVER (waist): a flat HANDCART (karachi) 1.7 × 0.95 on two wheels, shafts toward local +X; `load`: sacks, produce crates or nothing | solid to 0.97 | 2.52 × 1.11 × 0.97 | 9 solid, 2 markings | sack, woodGrey, GWOT_PALETTE.rubber, poplar | gwot-yard |
| **stall**(x, z, yaw = 0, w = 2.2, awning = 'tarpBlue') | COVER (waist at the counter): a MARKET STALL `w` wide: a skirted counter of produce trays on local −Z, the keeper's striped awning on four poles behind it | solid to 2.15 | 2.5 × 2.13 × 2.18 | 15 solid, 10 markings | ply, woodGrey, GWOT_PALETTE.canvas, poplar, tarpBlue | gwot-yard |
| **container**(x, z, yaw = 0, colour = 'paintRust') | COVER (full): a 20 ft shipping CONTAINER 6.06 × 2.44 × 2.59, its doors on local +X | solid to 2.59 | 6.08 × 2.44 × 2.59 | 6 solid, 4 markings | galv, GWOT_PALETTE.iron, paintRust | gwot-yard |
| **satDish**(x, z, yaw = 0, height = 1.6) | a SATELLITE DISH on a pole (thin: no cover) — the one modern thing on every roofline, here stood in the yard | solid to 1.6 | 0.72 × 0.53 × 2.2 | 3 solid | galv, GWOT_PALETTE.iron, GWOT_PALETTE.tin | gwot-yard |
| **bush**(x, z, s = 1) | PROP (plant): a dry BUSH (camelthorn / scrub), scale s | loose | 0.63 × 0.6 × 0.58 | 1 loose (2 parts) | scrub, straw | gwot-yard |

### gwotKit: lights

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **wire**(x0, z0, x1, z1, y = 2.7) | DECAL: overhead WIRE from (x0, z0) at y0 to (x1, z1) (level: the sag is not drawn) | drawn only | 3.54 × 0.03 × 0.01 | 1 drawn | plasticBlack | gwot-yard |
| **bulb**(x, z, name, group, on = true, intensity = 26) | a bare BULB on its flex under a roof: a warm point light on the mains (group = the room's lighting group) | drawn only, lit | 0.08 × 0.08 × 0.69 | 2 drawn, 1 lamp | plasticBlack, plasticWhite | gwot-yard |
| **lantern**(x, z, name, group, baseY = 0, on = true) | a hurricane LANTERN standing on something `baseY` high (0 = the floor; a low table 0.3, a table 0.75): a small warm light, its own fuel (not the mains) | off the floor, lit | 0.14 × 0.14 × 0.29 | 1 solid, 2 drawn, 1 lamp | paintBlue, plasticWhite | gwot-yard |
| **tube**(x, z, alongX, group, on = true, flicker = false) | a bare fluorescent TUBE under a roof (light-only, like the kits' ceiling panels — the camera looks through the roof): a switchable in `group` | overhead, lit | 1.2 × 0.1 × 0.04 | 1 solid, 1 fixture | plasticWhite | gwot-yard |
| **floodLight**(x, z, dir, name, on = true, height = 3.4, mains = false) | a work FLOODLIGHT on a pole `height` tall at (x, z), thrown along `dir` (an XZ direction): white, wide, off the yard's own supply (group 'exterior') | solid to 3.4, lit | 0.32 × 0.23 × 3.52 | 2 solid, 1 drawn, 1 lamp | galv, plasticBlack, plasticWhite | gwot-yard, library-hall |

## officeKit

`src/scene/officeGen.ts`

### officeKit: glass

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **paneX**(z, x0, x1, y0, y1, tint) | a GLASS pane in a wall along X at fixed z, x0‥x1 by y0‥y1, in the tint given: sight passes it, feet and sound do not, a round breaks it | glass | 1.4 × 0.01 × 1.5 | 1 solid | glassExt | library-hall |
| **paneZ**(x, z0, z1, y0, y1, tint) | a GLASS pane in a wall along Z at fixed x: as paneX | glass | 1.6 × 0.01 × 1.5 | 1 solid | glassExt | library-hall |

### officeKit: walls

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **wallX**(z, x0, x1, t, albedo, openings = [], extra = {}, height = H) | Wall running along X at fixed z (thickness t), from x0..x1, with openings (doors: y0=0,y1=2.2; windows: y0=0.9,y1=2.4). | solid to 3 | 7.24 × 0.24 × 3 | 1 solid | concrete | library-hall |
| **wallZ**(x, z0, z1, t, albedo, openings = [], extra = {}, height = H) | Wall running along Z at fixed x. | solid to 3 | 4.76 × 0.24 × 3 | 1 solid | concrete | library-hall |

### officeKit: props

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **desk**(x, z, w, d, yaw = 0, albedo = C.wood) | COVER: a DESK w × d, its top at 0.76 m on panel ends | solid to 0.76 | 1.6 × 0.8 × 0.76 | 4 solid | wood | library-hall |
| **monitor**(x, z, yaw, on, baseY = 0.76, keyboard = true, standby = false) | a MONITOR on a surface at baseY (a desk 0.76, a counter 1.15), lit when `on`, its keyboard in front of it | off the floor, lit | 0.54 × 0.43 × 0.49 | 5 solid, 2 markings, 1 fixture | black, screenOff, (0.08, 0.08, 0.09) | library-hall |

### officeKit: loose props

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **chair**(x, z, yaw, albedo = C.chair, made) | PROP (chair): an office CHAIR on casters: seat, back, gas lift, star base | loose | 0.5 × 0.5 × 1.1 | 1 loose (5 parts) | black, chair | library-hall |
| **cardboardBox**(x, z, s, yaw, y = 0) | PROP (cardboard): a CARTON of side s; with y over 0 it is stacked on the carton already standing there and moves with it | loose | 0.5 × 0.5 × 0.5 | 1 loose (1 part), 4 markings | cardboard | library-hall |

### officeKit: set dressing helpers

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **rug**(x, z, w, d, albedo, yaw = 0) | DECAL: a plain RUG w × d in the colour given: cloth. Floor paint and a dirt track are no rugs and are laid as their own flat boxes | drawn only | 2.4 × 1.6 × 0.01 | 1 drawn | carpetBlue | library-hall |
| **plant**(x, z, s = 1) | PROP (plant): a potted PLANT, scale s: a pot and two masses of leaves | loose | 0.58 × 0.56 × 1.26 | 1 loose (3 parts) | plant, (0.138, 0.345, 0.11), (0.36, 0.3, 0.26) | library-hall |
| **bin**(x, z) | PROP (bin): a waste BIN | loose | 0.26 × 0.26 × 0.4 | 1 loose (1 part) | (0.12, 0.12, 0.13) | library-hall |
| **cabinet**(x, z, yaw) | COVER: a FILING CABINET 0.46 × 0.6 × 1.32, drawer fronts on local −Z | solid to 1.32 | 0.6 × 0.46 × 1.32 | 1 solid, 1 marking | (0.58, 0.58, 0.56) | library-hall |
| **papers**(x, y, z, yaw = 0, scribble = false) | DECAL: a small pile of PAPERS on a surface at y | drawn only | 0.32 × 0.27 × 0.06 | 2 drawn, 2 markings | LIVED_PAINT.paper, (0.9, 0.88, 0.8) | library-hall |
| **wallArt**(x, y, z, w, h, albedo, alongX, marks = []) | DECAL: a flat panel on a wall (a picture, a board, a notice) w × h, its centre at y, carrying the markings given | drawn only | 1.2 × 0.03 × 0.8 | 1 drawn | white | library-hall |
| **bench**(x, z, yaw) | COVER (knee high): a BENCH 1.4 long, a plank seat on two steel ends | solid to 0.45 | 1.4 × 0.4 × 0.45 | 3 solid | metal, woodDark | library-hall |
| **car**(x, z, yaw, body) | COVER: a parked CAR 4.4 × 1.7, nose toward local +X, in the body colour given | solid to 1.52 | 4.44 × 1.84 × 1.52 | 7 solid, 4 drawn | black, partBlue, (0.07, 0.08, 0.1), (0.3, 0.02, 0.02), (0.8, 0.8, 0.75) | library-hall |
| **shelf**(x, z, w, d, yaw, levels = 4, hgt = 2.0) | COVER: open SHELVING w × d, `levels` boards up to `hgt` | solid to 1.9 | 1.6 × 0.4 × 1.9 | 8 solid, 6 markings | metal, cardboard | library-hall |

### officeKit: lights

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **ceilingPanel**(x, z, on, group, flicker = false, w = 1.2, d = 0.6, radiance = [11, 10.5, 9.5]) | a lit CEILING PANEL w × d (light only: the camera looks through the roof), a switchable in `group` | overhead, lit | 1.2 × 0.6 × 0.04 | 1 solid, 1 fixture | white | library-hall |

### officeKit: doors

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **doorFrame**(d) | the frame round one doorway that gets a leaf: jambs + head slightly proud of the wall, and the frame's stops (the comment at the office's door list says why) | solid to 2.05 | 0.98 × 0.26 × 2.12 | 5 solid | (0.2, 0.21, 0.23) | gwot-yard, library-hall |

### officeKit: what is fixed to a wall's face

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **band**(axis, line, a, b, side, y0, y1, paint, t = 0.24, proud = 0.012, solid = false) | DECAL: a BAND on a wall's face from a to b, y0‥y1, a finger proud: a painted dado or plinth (hand it the WALL'S OWN stuff in the new colour, `saidAs`: the wall's joints run on under the colour's edge), a plate, a door's skin. Returns its box, for what is painted on it | drawn only | 2.3 × 0.01 × 1 | 1 drawn | partBlue | library-hall |
| **vent**(axis, line, a, b, side, y0, y1, paint, t = 0.24, proud = 0.02, solid = false) | DECAL: a VENT on a wall's face from a to b, y0‥y1: a louvred panel two fingers proud (hand it a paint that says `louvre`) | drawn only | 0.6 × 0.02 × 0.4 | 1 drawn | (0.5, 0.51, 0.52) | library-hall |
| **conduit**(axis, line, a, b, side, y, paint, clip, drop, t = 0.24, solid = false) | DECAL: a run of CONDUIT on a wall's face from a to b at height y, a clip (`clip`: its paint) every metre and a half; `drop` = a branch down to that height at b, into a box that hangs under it (the box stands 11 cm past b) | drawn only | 4.61 × 0.09 × 0.75 | 6 drawn | metal, (0.7, 0.71, 0.72) | library-hall |
| **pipe**(x, z, y0, y1, r, paint, rings = [], solid = false) | a PIPE standing at (x, z) from y0 to y1, radius r: four slabs turned an eighth each, which the game stands up as one round (scene/round.ts). `rings` = what is round it, each [y0, y1, r]: a flange at its foot, a collar half way | drawn only | 0.23 × 0.23 × 3 | 12 drawn | (0.7, 0.71, 0.72) | library-hall |

## livedKit

`tools/levels/livedKit.ts`

### livedKit: the floor

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **mat**(x0, z0, x1, z1, albedo, top = 0.022) | a floor MAT over a rectangle in the colour given (drawn only) | drawn only | 1.4 × 1 × 0.02 | 1 drawn | rubberMat | library-hall |
| **duckboard**(x0, z0, x1, z1, n = 4) | a duckboard: a dark base and pale slats across its short side | drawn only | 1.4 × 0.7 × 0.04 | 5 drawn | wood, woodDark | library-hall |

### livedKit: furniture

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **table**(x, z, w, d, yaw = 0, top = C.wood, h = 0.74, legs = C.metal) | a trestle table: a top on two end frames (rays and rounds pass under it: it stops feet, not fire) | solid to 0.74 | 1.8 × 0.7 × 0.74 | 3 solid | metal, wood | library-hall |
| **crate**(x, z, w, d, h, yaw = 0, albedo = C.wood, y = 0) | a wooden crate with two battens on its lid (a solid block: cover to its height) | solid to 0.47 | 0.8 × 0.5 × 0.47 | 3 solid | woodDark, wood | library-hall |
| **tyres**(x, z, n, yaw = 0, r = 0.34) | a stack of n tyres (0.2 m each), each a `round` turned 0.1 rad on the one below, the top one's well drawn on it. A layer turned θ reaches at most r·(cos θ + sin θ) along a wall: a four-high stack (0.3 rad at the top) 8.5 cm past r, so keep the centre r + 0.1 off a wall. Stacks side by side: give them the SAME yaw and 0.72 m between centres at r 0.34 (turned differently they bite into each other until 0.77 m). One lying alone is a fat one, 0.27 m: a solid topping out between 0.20 and 0.25 m stops a body on a cell the nav grid calls free | solid to 0.6 | 0.68 × 0.68 × 0.6 | 12 solid, 3 round, 1 marking | black | library-hall |
| **jerryCans**(x, z, n, yaw = 0, albedo = LIVED_PAINT.oliveDrab, y = 0) | a row of n jerry cans along local X, standing | solid to 0.47 | 0.56 × 0.34 × 0.47 | 3 solid | oliveDrab | library-hall |
| **drumFixed**(x, z, albedo = LIVED_PAINT.oxide, y = 0) | a drum standing where it is (static; `drum` is the one a man can shove) | solid to 0.88 | 0.54 × 0.54 × 0.88 | 4 solid, 1 round, 9 markings | oxide | library-hall |
| **hung**(x, z, alongX, w, y0, y1, albedo, t = 0.04) | a thing hung on a wall (drawn only: a coat, a board, a hose): `alongX` = on a wall that runs along X, at the wall's face `x, z` | drawn only | 0.5 × 0.04 × 0.8 | 1 drawn | LIVED_PAINT.canvas | library-hall |

### livedKit: clutter

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **mug**(x, y, z, albedo = C.white) | a MUG on a surface at y (clutter: a firefight knocks it off) | off the floor | 0.07 × 0.07 × 0.09 | 1 solid | white | library-hall |
| **papers**(x, y, z, yaw = 0) | the office kit's pile of PAPERS on a surface at y *(is `officeKit.papers`)* | drawn only | 0.32 × 0.27 × 0.06 | 2 drawn, 2 markings | LIVED_PAINT.paper, (0.9, 0.88, 0.8) | library-hall |
| **sheet**(x, y, z, w = 0.21, d = 0.3, yaw = 0, albedo = LIVED_PAINT.paper) | one loose sheet (a card, a form, a map corner) | drawn only | 0.3 × 0.21 × 0.01 | 1 drawn | LIVED_PAINT.paper | library-hall |

### livedKit: loose things

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **stool**(x, z, albedo = C.woodDark) | PROP (chair): a STOOL, a seat on a post | loose | 0.34 × 0.34 × 0.47 | 1 loose (2 parts) | metal, woodDark | library-hall |
| **chair**(x, z, yaw, albedo = C.chair) | PROP (chair): the office CHAIR, as one of this room's lived things *(is `officeKit.chair`)* | loose | 0.5 × 0.5 × 1.1 | 1 loose (5 parts) | black, chair | library-hall |
| **drum**(x, z, albedo = LIVED_PAINT.oxide, mass = 22) | PROP (bin): an oil DRUM a man can shove, 0.56 across and 0.88 high (`drumFixed` is the one that stands where it is) | loose | 0.56 × 0.56 × 0.89 | 1 loose (2 parts) | oxide, oxide × 0.60 | library-hall |
| **bucket**(x, z, albedo = C.metal) | PROP (bin): a BUCKET | loose | 0.28 × 0.28 × 0.3 | 1 loose (1 part) | metal | library-hall |
| **cone**(x, z) | PROP (cardboard): a traffic CONE | loose | 0.34 × 0.34 × 0.56 | 1 loose (3 parts) | black, hiVis, white | library-hall |
| **carton**(x, z, s, yaw) | PROP (cardboard): the office kit's CARTON of side s, as one of this room's lived things *(is `officeKit.cardboardBox`)* | loose | 0.45 × 0.45 × 0.45 | 1 loose (1 part), 4 markings | cardboard | library-hall |
| **looseCrate**(x, z, w, d, h, yaw = 0, albedo = C.wood) | a small wooden box a man can shove (ammunition, spares, fruit) | loose | 0.5 × 0.36 × 0.32 | 1 loose (2 parts) | wood, woodDark | library-hall |

## livedPieces

`tools/levels/livedPieces.ts`

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **floor**(x, z, w, d, albedo, yaw = 0.014, y = 0.016) | a floor covering (drawn only, no step), laid a degree off square — by hand. At its default height (top 0.0195) it lies OVER the look's floor marks (tops 0.012‥0.0143) and under the kit's rugs: a mat, a patch. A wall-to-wall floor passes y 0.006: the look's marks and the kit's rugs then draw over it | drawn only | 1.81 × 1.41 × 0.01 | 1 drawn | baize | library-hall |

### livedPieces: carcasses

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **table**(x, z, w, d, yaw = 0, top = C.wood, leg = C.woodDark, h = 0.74) | a four-legged table: top w × d at height h | solid to 0.74 | 2 × 0.9 × 0.74 | 5 solid | woodDark, wood | library-hall |
| **counter**(x, z, w, d, yaw = 0, h = 1.0, body = C.woodDark, top = C.wood) | a counter or a plinth: a solid body under an overhanging top | solid to 1 | 2.46 × 0.76 × 1 | 2 solid | wood, woodDark | library-hall |
| **chest**(x, z, w, d, yaw = 0, h = 0.9, rows = 5, body = C.wood, top = C.woodDark) | a chest of wide shallow drawers (a plan chest, a map chest): `rows` drawer seams and a brass pull each on the front | solid to 0.9 | 1.06 × 0.66 × 0.9 | 2 solid, 9 drawn | INK.brass, wood × 0.45, wood, woodDark | library-hall |

### livedPieces: paper, plans, books

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **rolls**(x, y, z, yaw = 0, k = 4, len = 0.8, pitch = 0.085) | `k` rolled plans lying side by side on a surface at y, each `len` long along local X | off the floor | 0.76 × 0.31 × 0.06 | 4 solid | blueprint, cream × 0.95, cream × 0.98, cream × 0.99 | library-hall |
| **rollBin**(x, z) | a tall bin of rolled plans standing on end (a loose prop: it tips) | loose | 0.32 × 0.32 × 1.08 | 1 loose (6 parts) | blueprint, cream × 0.93, cream × 0.95, cream × 0.97, cream × 0.98, metal | library-hall |
| **plan**(x, y, z, w, d, yaw = 0, albedo = INK.cream) | a big sheet spread on a surface (a map, a plan, a blueprint: too big to fly — drawn only) with a darker border line | drawn only | 0.9 × 0.6 × 0.02 | 3 drawn | cream, cream × 0.74, cream × 0.86 | library-hall |
| **sheets**(x, y, z, yaw = 0, k = 3, albedo = INK.paper) | loose sheets on a surface (PAPER: they fly) | drawn only | 0.53 × 0.32 × 0.02 | 3 drawn | INK.paper × 0.98, INK.paper × 0.95 | library-hall |
| **ledger**(x, y, z, yaw = 0, open = true, cover = INK.oxblood) | a ledger: open (two pale pages on a dark cover) or shut | off the floor | 0.56 × 0.4 × 0.04 | 3 solid | cream, INK.paper, oxblood | library-hall |
| **planBook**(x, y, z, yaw = 0, w = 0.45, d = 0.46, cover = INK.baize) | a huge plan book open on a lectern: two pale pages w each, thin parcel lines drawn on them (read from above) | off the floor | 0.96 × 0.5 × 0.06 | 3 solid, 4 drawn | manila × 0.60, baize, cream, INK.paper | library-hall |
| **books**(x, y, z, yaw = 0, k = 6, palette = [INK.oxblood, INK.baize, INK.navy, C.woodDark, INK.manila]) | a row of `k` books or binders standing on a surface (CLUTTER: they are knocked off) | off the floor | 0.33 × 0.18 × 0.29 | 6 solid | woodDark, baize, manila, navy | library-hall |
| **tins**(x, y, z, yaw = 0, k = 3, pitch = 0.38, lid = C.white, tape = C.red) | `k` tin deed boxes in a row on a surface: black japanned, a WHITE lid, red tape crossed over it — the white square with the red cross reads from above (all solid, so a tin knocked off takes its lid and tape with it: CLUTTER) | off the floor | 1.09 × 0.25 × 0.15 | 12 solid | red, INK.tin, white | library-hall |
| **tray**(x, y, z, yaw = 0) | an in-tray with papers in it (the tray CLUTTER, the top sheet PAPER) | off the floor | 0.34 × 0.26 × 0.06 | 1 solid, 1 drawn | INK.paper, metal | library-hall |
| **stamps**(x, y, z, yaw = 0) | an ink pad and three stamps (CLUTTER) | off the floor | 0.4 × 0.1 × 0.08 | 4 solid | woodDark, navy | library-hall |

### livedPieces: instruments and tools

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **tripod**(x, z, yaw = 0, body = INK.yellow, legs = C.wood, top = 1.25) | an instrument on a tripod (a theodolite, a level, a camera): three legs, a head, a scope along local Z | solid to 1.39 | 0.54 × 0.51 × 1.39 | 9 solid | wood, asphalt, yellow | library-hall |
| **poles**(x, y, z, yaw = 0, k = 3, len = 2.0, pitch = 0.09) | `k` ranging poles lying on two brackets at height y along local X: red and white by the half metre | off the floor | 2.06 × 0.37 × 0.08 | 14 solid | red, white, metal | library-hall |
| **drafting**(x, z, yaw = 0, w = 1.5, d = 1.0) | a drawing board on a pedestal: a tilted-looking pale board, the sheet pinned on it, a T-square across, a set square, a pot of pencils | solid to 1.01 | 1.59 × 1 × 1.01 | 8 solid, 3 drawn | asphalt, woodDark, cream, enamel, INK.paper, metal, wood | library-hall |
| **ladder**(x, y, z, yaw = 0, len = 3.0, wid = 0.42, rail = C.wood) | a ladder lying along local X at height y (on the floor against a wall, on trestles): two rails and its rungs | walked over | 2.4 × 0.46 × 0.07 | 9 solid | wood × 0.80, wood | library-hall |

### livedPieces: safes

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **safe**(x, z, yaw = 0, w = 0.8, d = 0.7, h = 1.25, body = INK.enamel) | a safe: a dark body, its door proud on the front with a brass dial and a handle | solid to 1.25 | 0.8 × 0.77 × 1.25 | 4 solid | INK.brass, enamel × 0.50, enamel × 0.62 | library-hall |

### livedPieces: on a wall

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **hooks**(x, z, yaw, k = 4, y = 1.6, items = [INK.hivis, INK.navy, INK.canvas], pitch = 0.36) | a rail of hooks on the wall behind the piece at height y with `k` things hanging from it (coats, vests, belts: slabs 0.5 tall in the colours given) | off the floor | 1.44 × 0.12 × 0.64 | 5 solid | hivis, INK.canvas, navy, woodDark | library-hall |
| **rail**(x, z, yaw, k, y, w, h, d, albedo, pitch = 0.34) | a rail on the wall behind the piece at height y carrying `k` small things (hard hats, fire buckets, mugs on hooks: CLUTTER between 0.7 and 1.6 m) | off the floor | 1.02 × 0.24 × 0.29 | 4 solid | hivis, metal | library-hall |

### livedPieces: on a desk

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **hat**(x, y, z, yaw = 0, albedo = INK.yellow) | a hard hat on a surface (CLUTTER) | off the floor | 0.31 × 0.22 × 0.1 | 2 solid | yellow | library-hall |
| **mugs**(x, y, z, yaw = 0, k = 3, palette = [C.white, INK.enamel, INK.navy], pitch = 0.13) | `k` mugs or glasses in a row on a surface (CLUTTER) | off the floor | 0.34 × 0.09 × 0.09 | 3 solid | enamel, navy, white | library-hall |
| **kettle**(x, y, z, yaw = 0, albedo = C.metal) | a kettle (CLUTTER) | off the floor | 0.25 × 0.17 × 0.16 | 2 solid | metal | library-hall |
| **typewriter**(x, y, z, yaw = 0) | a typewriter with a sheet in it | off the floor | 0.4 × 0.3 × 0.28 | 2 solid, 1 drawn | asphalt, INK.paper, metal | library-hall |
| **phone**(x, y, z, yaw = 0, albedo = INK.iron) | a desk telephone (CLUTTER) | off the floor | 0.22 × 0.15 × 0.09 | 2 solid | asphalt | library-hall |

### livedPieces: on the floor

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **coatStand**(x, z, yaw = 0, coat = INK.navy) | a coat stand with a coat and a hat on it | solid to 1.74 | 0.58 × 0.52 × 1.76 | 6 solid | woodDark, navy, navy × 0.60 | library-hall |
| **boots**(x, z, yaw = 0, k = 3, pitch = 0.4) | `k` pairs of boots in a row on a mat along local X | solid to 0.33 | 1.3 × 0.42 × 0.31 | 6 solid, 1 drawn | INK.rubber, (0.2, 0.13, 0.08) | library-hall |
| **sack**(x, z, yaw = 0, albedo = INK.canvas, y = 0) | a sack lying on the floor (mail, rice, cement): a body, a bulge, a tied neck | solid to 0.36 | 0.72 × 0.4 × 0.36 | 3 solid | INK.canvas, INK.canvas × 0.80, (0.594, 0.54, 0.41) | library-hall |

### livedPieces: loose

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **stool**(x, z, albedo = C.woodDark) | a stool | loose | 0.36 × 0.36 × 0.48 | 1 loose (4 parts) | black, woodDark | library-hall |
| **armchair**(x, z, yaw = 0, albedo = INK.oxblood) | an armchair (heavy: it is cover you can shove) | loose | 0.7 × 0.7 × 0.9 | 1 loose (5 parts) | oxblood, oxblood × 0.85, woodDark | library-hall |
| **crate**(x, z, s = 0.5, yaw = 0, albedo = C.wood, tops = null, thing = 'crate') | a wooden crate of side s (a tea chest, a parts crate): slats across its lid; `tops` stand on it (bottles, insulators) | loose | 0.5 × 0.5 × 0.42 | 1 loose (3 parts) | wood × 0.75, wood | library-hall |
| **trolley**(x, z, yaw = 0, load = [INK.oxblood, INK.baize, INK.manila, INK.navy], frame = C.metal) | a trolley on casters, two shelves, a load of books or boxes in the colours given | loose | 0.81 × 0.42 × 0.94 | 1 loose (8 parts) | metal, baize, manila, navy, oxblood | library-hall |
| **peg**(x, z, top = C.red, body = C.white, h = 0.28) | a peg in the ground (a survey peg, a tent peg): a short post with a painted top | solid to 0.32 | 0.07 × 0.07 × 0.32 | 2 solid | red, white | library-hall |
| **rod**(x0, z0, x1, z1, len = 2.0, y = 0.012) | one ranging pole lying on the ground from (x0, z0) toward (x1, z1): red and white by the half metre, `len` long from its first end | walked over | 2 × 0.05 × 0.05 | 4 solid | red, white | library-hall |
| **cone**(x, z) | a traffic cone | loose | 0.3 × 0.3 × 0.49 | 1 loose (4 parts) | hivis, white | library-hall |
| **bucket**(x, z, albedo = C.metal, thing = 'bucket') | a bucket (a mop bucket, a fire bucket on the floor, a coal scuttle) | loose | 0.3 × 0.26 × 0.3 | 1 loose (2 parts) | metal, metal × 0.60 | library-hall |

## livedQuiet

`tools/levels/lived-quiet.ts`

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **mat**(x, z, w, dd, m, yaw = 0, y = 0.016) | a FLOOR PIECE w × d (a mat, a stain, a painted panel) | drawn only | 1.6 × 0.9 × 0.01 | 1 drawn | GWOT_PALETTE.rubber | library-hall |
| **line**(x0, z0, x1, z1, wid, m, y = 0.03) | a painted LINE on the ground from (x0, z0) to (x1, z1), `wid` wide | drawn only | 1.6 × 0.06 × 0 | 1 drawn | whitewash | library-hall |
| **paper**(x, z, y, yaw = 0, w = 0.3, dd = 0.21, m = 'whitewash') | PAPER lying on a surface whose top is at y | drawn only | 0.3 × 0.21 × 0.01 | 1 drawn | whitewash | library-hall |
| **pail**(what, x, z, m = 'galv', fill = null) | PROP (bin): a pail / bucket, 0.28 across | loose | 0.37 × 0.37 × 0.26 | 1 loose (2 parts) | galv | library-hall |
| **carton**(what, x, z, yaw, w, dd, h, m = 'coyote', band = 'sack', mass = 5) | PROP (cardboard): a carton / case w × d × h with a band of another paint across its lid | loose | 0.5 × 0.36 × 0.31 | 1 loose (2 parts) | coyote, sack | library-hall |
| **stool**(what, x, z, yaw = 0, seat = 'poplar') | PROP (chair): a three-legged wooden STOOL, 0.34 across | loose | 0.34 × 0.34 × 0.34 | 1 loose (2 parts) | poplar, woodGrey | library-hall |
| **chock**(what, x, z, yaw = 0, m = 'prodYellow') | PROP (bin): a WHEEL CHOCK, a stepped wedge 0.24 long | loose | 0.24 × 0.16 × 0.15 | 1 loose (2 parts) | prodYellow | library-hall |
| **tyre**(what, x, z, yaw = 0) | PROP (bin): a loose TYRE lying flat (an octagon, 0.6 across), its rim showing | loose | 0.58 × 0.58 × 0.22 | 1 loose (5 parts) | GWOT_PALETTE.rubber, galv | library-hall |
| **mug**(x, z, y, m = 'whitewash', r = 0.04, h = 0.1) | a mug / glass / tin standing on a surface at y | off the floor | 0.08 × 0.08 × 0.1 | 1 solid | whitewash | library-hall |
| **bench**(x, z, len, alongX, top = 'poplar', foot = 'block') | a BENCH: a plank `len` long on two block feet (seat at 0.44) | solid to 0.44 | 1.3 × 0.32 × 0.44 | 3 solid | block, poplar | library-hall |
| **shelf**(x, z, len, depth, h, alongX, boards = 3, frame = 'galv', board = 'woodGrey') | an open-backed SHELF UNIT against a wall: two ends, `boards` boards, top at `h`; returns each board's top y (lowest first) | solid to 1.5 | 2 × 0.36 × 1.5 | 5 solid | woodGrey, galv | library-hall |

## livedTrain

`tools/levels/lived-train.ts`

### livedTrain: furniture

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **table**(x0, z0, x1, z1, top = 0.75, m = 'ply', leg = 'woodGrey') | a table: a top on four legs | solid to 0.75 | 1.8 × 0.8 × 0.75 | 5 solid | woodGrey, ply | library-hall |
| **plank**(x0, z0, x1, z1, h = 0.42, m = 'poplar', foot = 'block') | a plank on two blocks: a bench, a low stand | solid to 0.42 | 1.8 × 0.28 × 0.42 | 3 solid | block, poplar | library-hall |
| **rack**(x0, z0, x1, z1, height, levels, m = 'woodGrey') | open shelving: two ends and a board at each of `levels` | solid to 1.7 | 2.2 × 0.36 × 1.7 | 5 solid | woodGrey | library-hall |
| **row**(x, z, axis, n, w, d, g, y0, y1, ms, flags = 0) | a row of things standing along a line: n boxes w × d, from y0 to y1, gap g, colours in turn | solid to 0.5 | 1.38 × 0.3 × 0.5 | 4 solid | coyote, olive | library-hall |

### livedTrain: loose things

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **stool**(x, z, m = 'poplar', yaw = 0) | PROP (chair): a four-legged STOOL | loose | 0.34 × 0.34 × 0.42 | 1 loose (5 parts) | woodGrey, poplar | library-hall |
| **bucket**(x, z, m = 'tin', fill = 'rubber') | PROP (bin): a BUCKET, with what is in it (`fill`) | loose | 0.26 × 0.26 × 0.29 | 1 loose (2 parts) | GWOT_PALETTE.rubber, GWOT_PALETTE.tin | library-hall |
| **pot**(x, z, m = 'mudDark') | a clay pot (or, in straw, a basket): round, a neck on it | loose | 0.4 × 0.4 × 0.34 | 1 loose (3 parts) | mudDark | library-hall |
| **jerry**(x, z, yaw, m) | a jerry can | loose | 0.34 × 0.14 × 0.46 | 1 loose (2 parts) | olive, plasticBlack | library-hall |
| **helmet**(x, z, yaw = 0, m = 'olive') | a helmet set down on its rim | loose | 0.26 × 0.24 × 0.16 | 1 loose (2 parts) | olive | library-hall |
| **toolRoll**(x, z, yaw = 0) | a canvas roll of tools or cleaning kit, tied | loose | 0.48 × 0.15 × 0.1 | 1 loose (2 parts) | GWOT_PALETTE.canvas, GWOT_PALETTE.rubber | library-hall |
| **chest**(x, z, yaw, m, hx = 0.28, hy = 0.15, hz = 0.18, lid = 'tin') | PROP (cardboard): a small CHEST with a lid in another paint | loose | 0.58 × 0.38 × 0.32 | 1 loose (2 parts) | GWOT_PALETTE.tin, olive | library-hall |

### livedTrain: the seven signature pieces

| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |
|---|---|---|---|---|---|---|
| **hoist**(x, z, turn = 0) | MOTOR POOL: an engine hoist, the engine hanging on its chain over a drip tray (beam along the frame's x) | solid to 2.3 | 1.6 × 0.96 × 2.42 | 11 solid, 3 drawn | rust, GWOT_PALETTE.iron, galv, genYellow, GWOT_PALETTE.tin, oil | library-hall |
| **cauldrons**(x, z, turn, n) | KITCHEN COURT: n black cauldrons in a row on a mud hearth | solid to 0.72 | 2.34 × 0.88 × 0.77 | 13 solid, 7 drawn, 3 round | GWOT_PALETTE.iron, GWOT_PALETTE.rubber, GWOT_PALETTE.tin, mud | library-hall |
| **spools**(x, z, turn = 0) | GENERATOR YARD: a cable spool stood on its rim and one laid flat | solid to 1.2 | 2.58 × 1.18 × 1.2 | 15 solid, 3 round | woodGrey, plasticBlack | library-hall |
| **flag**(x, z) | THE RANGE: the red flag on its pole, a red and white square painted round the foot | solid to 3.5 | 1.5 × 1 × 3.5 | 2 solid, 3 drawn | truckRed, block, galv, whitewash | library-hall |
| **basins**(x, z, turn = 0) | QUARTERS: the wash bench — tin basins in a row, towels on a rail behind | solid to 1.3 | 2 × 0.55 × 1.3 | 8 solid, 6 drawn | GWOT_PALETTE.tin, plasticBlue, woodGrey, block, blanketGrey, poplar, rugCream | library-hall |
| **loft**(x, z) | THE WHITE HOUSE: the headman's pigeon loft — a white box with a green roof up a pole (not blue: BLUE is a pair in the lesson it stands in) | solid to 1.9 | 1.34 × 1.08 × 2.51 | 3 solid, 5 drawn | GWOT_PALETTE.rubber, woodGrey, paintGreen, straw, whitewash | library-hall |
| **waterCart**(x, z, turn = 0) | THE WELL COURT: the water cart — two barrels on a flat cart, shafts down on a prop | solid to 1.14 | 2.56 × 1.12 × 1.14 | 8 solid, 1 drawn | woodGrey, GWOT_PALETTE.rubber, drumBlue, GWOT_PALETTE.tin, plasticBlue | library-hall |

## Paints

Every colour with a name, by table. *Stands for* is what the simulation reads the paint as (a round, a charge and a blast go by it); *underfoot* is its footstep surface.
Pieces counts the catalogue's pieces that wear the row, darkened or not, as laid above.

### GWOT_PALETTE

`src/scene/gwotPalette.ts`: the look's own paints: the only table the look draws as materials.

| Paint | Colour (linear) | Finish | Stands for | Underfoot | Pieces |
|---|---|---|---|---|---|
| mud | 0.42, 0.32, 0.21 | rough 0.97 | concrete |  | 11 |
| mudDark | 0.29, 0.22, 0.15 | rough 0.97 | concrete |  | 5 |
| mudLight | 0.56, 0.46, 0.33 | rough 0.95 | wall |  | 8 |
| render | 0.57, 0.53, 0.45 | rough 0.9 | concrete |  | 1 |
| whitewash | 0.72, 0.69, 0.61 | rough 0.88 | wall |  | 8 |
| block | 0.4, 0.39, 0.37 | rough 0.92 | concrete |  | 8 |
| tWall | 0.46, 0.45, 0.43 | rough 0.9 | concrete |  | 2 |
| paintBlue | 0.06, 0.19, 0.32 | rough 0.6 | metal |  | 3 |
| paintGreen | 0.08, 0.24, 0.16 | rough 0.6 | metal |  | 5 |
| paintRust | 0.3, 0.075, 0.04 | rough 0.65 | metal |  | 1 |
| paintCream | 0.66, 0.6, 0.44 | rough 0.7 | wall |  | 0 |
| dust | 0.205, 0.145, 0.085 | rough 1 | asphalt | asphalt | 2 |
| dirt | 0.135, 0.09, 0.05 | rough 1 | asphalt | asphalt | 5 |
| gravel | 0.15, 0.135, 0.115 | rough 1 | asphalt | asphalt | 1 |
| mudFloor | 0.17, 0.115, 0.07 | rough 0.97 | stone | stone | 0 |
| slab | 0.3, 0.285, 0.26 | rough 0.9 | concrete | stone | 0 |
| damp | 0.075, 0.052, 0.034 | rough 0.7 | asphalt |  | 2 |
| oil | 0.11, 0.095, 0.08 | rough 0.35 | asphalt |  | 1 |
| rugRed | 0.3, 0.052, 0.04 | rough 0.95 | carpetRed | carpet | 3 |
| rugNavy | 0.04, 0.06, 0.15 | rough 0.95 | carpetBlue | carpet | 1 |
| rugCream | 0.6, 0.52, 0.38 | rough 0.95 | carpet | carpet | 2 |
| rugOchre | 0.5, 0.33, 0.1 | rough 0.95 | carpet |  | 0 |
| rugGreen | 0.09, 0.22, 0.13 | rough 0.95 | carpet |  | 0 |
| canvas | 0.33, 0.31, 0.2 | rough 0.95 | chair |  | 4 |
| blanketGrey | 0.3, 0.3, 0.31 | rough 0.95 | chair |  | 3 |
| blanketBrown | 0.27, 0.18, 0.12 | rough 0.95 | chair |  | 1 |
| tarpBlue | 0.05, 0.17, 0.48 | rough 0.95 | chair |  | 1 |
| sack | 0.62, 0.58, 0.48 | rough 0.95 | chair |  | 4 |
| sandbag | 0.45, 0.39, 0.27 | rough 0.95 | concrete |  | 1 |
| hesco | 0.47, 0.44, 0.36 | rough 0.95 | concrete |  | 1 |
| olive | 0.15, 0.17, 0.09 | rough 0.75 | woodDark |  | 7 |
| coyote | 0.4, 0.31, 0.2 | rough 0.85 | chair |  | 2 |
| stencil | 0.62, 0.5, 0.1 | rough 0.7 | metal |  | 0 |
| rust | 0.28, 0.12, 0.06 | rough 0.85 | metal |  | 7 |
| iron | 0.098, 0.1, 0.112 | rough 0.5, bare metal | metal |  | 16 |
| galv | 0.5, 0.52, 0.53 | rough 0.38, bare metal | metal |  | 11 |
| tin | 0.56, 0.55, 0.52 | rough 0.42, bare metal | metal |  | 9 |
| drumBlue | 0.06, 0.15, 0.33 | rough 0.5 | metal |  | 3 |
| drumRed | 0.4, 0.07, 0.05 | rough 0.5 | metal |  | 2 |
| genYellow | 0.6, 0.42, 0.04 | rough 0.5 | metal |  | 2 |
| plasticWhite | 0.7, 0.7, 0.66 | rough 0.45 | ~plastic |  | 7 |
| plasticGreen | 0.08, 0.28, 0.14 | rough 0.45 | ~plastic |  | 1 |
| plasticBlue | 0.03, 0.12, 0.36 | rough 0.45 | ~plastic |  | 5 |
| plasticBlack | 0.045, 0.045, 0.05 | rough 0.5 | ~plastic |  | 10 |
| jerryYellow | 0.62, 0.46, 0.05 | rough 0.45 | ~plastic |  | 0 |
| rubber | 0.03, 0.03, 0.033 | rough 0.85 | ~plastic |  | 13 |
| poplar | 0.47, 0.36, 0.22 | rough 0.8 | wood |  | 15 |
| woodGrey | 0.33, 0.29, 0.23 | rough 0.85 | wood |  | 24 |
| ply | 0.55, 0.43, 0.27 | rough 0.75 | wood |  | 4 |
| truckWhite | 0.68, 0.68, 0.65 | rough 0.35 | metal |  | 2 |
| truckTan | 0.5, 0.42, 0.29 | rough 0.4 | metal |  | 0 |
| truckRed | 0.42, 0.06, 0.05 | rough 0.35 | metal |  | 5 |
| glassDark | 0.05, 0.06, 0.08 | rough 0.1 | ~glass |  | 3 |
| prodOrange | 0.7, 0.3, 0.04 | rough 0.95 | plant |  | 0 |
| prodGreen | 0.22, 0.4, 0.1 | rough 0.95 | plant |  | 2 |
| prodRed | 0.5, 0.07, 0.05 | rough 0.95 | plant |  | 0 |
| prodYellow | 0.7, 0.55, 0.1 | rough 0.95 | plant |  | 1 |
| straw | 0.51, 0.43, 0.22 | rough 0.95 | plant |  | 2 |
| scrub | 0.24, 0.26, 0.13 | rough 0.95 | plant |  | 1 |
| rugMadder | 0.1, 0.03, 0.035 | rough 0.95 | carpetRed | carpet | 1 |

### SNOW_PALETTE

`src/scene/gwotPalette.ts`: lying snow as paint, for a level built in winter.

| Paint | Colour (linear) | Finish | Stands for | Underfoot | Pieces |
|---|---|---|---|---|---|
| snow | 0.66, 0.71, 0.78 |  | asphalt |  | 0 |
| snowPacked | 0.43, 0.47, 0.53 |  | asphalt |  | 0 |
| snowOld | 0.27, 0.3, 0.34 |  | asphalt |  | 0 |

### OFFICE_PALETTE

`src/scene/officeGen.ts`: the office's paints: the names the simulation reads a box's stuff by.

| Paint | Colour (linear) | Finish | Stands for | Underfoot | Pieces |
|---|---|---|---|---|---|
| carpet | 0.13, 0.14, 0.17 |  |  |  | 0 |
| carpetBlue | 0.09, 0.11, 0.2 |  |  |  | 1 |
| carpetRed | 0.22, 0.07, 0.07 |  |  |  | 0 |
| lino | 0.33, 0.33, 0.31 |  |  |  | 0 |
| stone | 0.38, 0.37, 0.35 |  |  |  | 0 |
| asphalt | 0.09, 0.09, 0.1 |  |  |  | 4 |
| grass | 0.1, 0.16, 0.08 |  |  |  | 0 |
| wall | 0.66, 0.64, 0.6 |  |  |  | 0 |
| wallDark | 0.36, 0.38, 0.4 |  |  |  | 0 |
| concrete | 0.42, 0.41, 0.4 |  |  |  | 2 |
| ceiling | 0.72, 0.72, 0.7 |  |  |  | 0 |
| partRed | 0.5, 0.13, 0.1 |  |  |  | 0 |
| partBlue | 0.13, 0.17, 0.42 |  |  |  | 2 |
| partTan | 0.55, 0.47, 0.35 |  |  |  | 0 |
| wood | 0.52, 0.4, 0.27 |  |  |  | 12 |
| woodDark | 0.25, 0.18, 0.12 |  |  |  | 15 |
| metal | 0.3, 0.31, 0.33 |  |  |  | 15 |
| black | 0.04, 0.04, 0.045 |  |  |  | 7 |
| chair | 0.14, 0.2, 0.45 |  |  |  | 2 |
| cardboard | 0.58, 0.44, 0.26 |  |  |  | 3 |
| white | 0.8, 0.8, 0.78 |  |  |  | 10 |
| plant | 0.12, 0.3, 0.1 |  |  |  | 1 |
| glassFrame | 0.2, 0.22, 0.25 |  |  |  | 0 |
| screenOff | 0.02, 0.02, 0.025 |  |  |  | 1 |
| red | 0.55, 0.08, 0.06 |  |  |  | 4 |
| glass | 0.8, 0.87, 0.83 |  |  |  | 0 |
| glassExt | 0.7, 0.75, 0.79 |  |  |  | 2 |

### KIT_PALETTE

`src/scene/levelKit.ts`: the map kit's copy of the office's paints.

| Paint | Colour (linear) | Finish | Stands for | Underfoot | Pieces |
|---|---|---|---|---|---|
| carpet | 0.13, 0.14, 0.17 |  |  |  | the same colour as OFFICE_PALETTE.carpet |
| carpetBlue | 0.09, 0.11, 0.2 |  |  |  | the same colour as OFFICE_PALETTE.carpetBlue |
| carpetRed | 0.22, 0.07, 0.07 |  |  |  | the same colour as OFFICE_PALETTE.carpetRed |
| lino | 0.33, 0.33, 0.31 |  |  |  | the same colour as OFFICE_PALETTE.lino |
| stone | 0.38, 0.37, 0.35 |  |  |  | the same colour as OFFICE_PALETTE.stone |
| asphalt | 0.09, 0.09, 0.1 |  |  |  | the same colour as OFFICE_PALETTE.asphalt |
| grass | 0.1, 0.16, 0.08 |  |  |  | the same colour as OFFICE_PALETTE.grass |
| wall | 0.66, 0.64, 0.6 |  |  |  | the same colour as OFFICE_PALETTE.wall |
| wallDark | 0.36, 0.38, 0.4 |  |  |  | the same colour as OFFICE_PALETTE.wallDark |
| concrete | 0.42, 0.41, 0.4 |  |  |  | the same colour as OFFICE_PALETTE.concrete |
| ceiling | 0.72, 0.72, 0.7 |  |  |  | the same colour as OFFICE_PALETTE.ceiling |
| partRed | 0.5, 0.13, 0.1 |  |  |  | the same colour as OFFICE_PALETTE.partRed |
| partBlue | 0.13, 0.17, 0.42 |  |  |  | the same colour as OFFICE_PALETTE.partBlue |
| partTan | 0.55, 0.47, 0.35 |  |  |  | the same colour as OFFICE_PALETTE.partTan |
| wood | 0.52, 0.4, 0.27 |  |  |  | the same colour as OFFICE_PALETTE.wood |
| woodDark | 0.25, 0.18, 0.12 |  |  |  | the same colour as OFFICE_PALETTE.woodDark |
| metal | 0.3, 0.31, 0.33 |  |  |  | the same colour as OFFICE_PALETTE.metal |
| black | 0.04, 0.04, 0.045 |  |  |  | the same colour as OFFICE_PALETTE.black |
| chair | 0.14, 0.2, 0.45 |  |  |  | the same colour as OFFICE_PALETTE.chair |
| cardboard | 0.58, 0.44, 0.26 |  |  |  | the same colour as OFFICE_PALETTE.cardboard |
| white | 0.8, 0.8, 0.78 |  |  |  | the same colour as OFFICE_PALETTE.white |
| plant | 0.12, 0.3, 0.1 |  |  |  | the same colour as OFFICE_PALETTE.plant |
| glassFrame | 0.2, 0.22, 0.25 |  |  |  | the same colour as OFFICE_PALETTE.glassFrame |
| screenOff | 0.02, 0.02, 0.025 |  |  |  | the same colour as OFFICE_PALETTE.screenOff |
| red | 0.55, 0.08, 0.06 |  |  |  | the same colour as OFFICE_PALETTE.red |
| glass | 0.8, 0.87, 0.83 |  |  |  | the same colour as OFFICE_PALETTE.glass |
| glassExt | 0.7, 0.75, 0.79 |  |  |  | the same colour as OFFICE_PALETTE.glassExt |

### LIVED_PAINT

`tools/levels/livedKit.ts`: colours of the older levels' lived things that the office has no name for.

| Paint | Colour (linear) | Finish | Stands for | Underfoot | Pieces |
|---|---|---|---|---|---|
| oliveDrab | 0.24, 0.235, 0.14 |  |  |  | 1 |
| oxide | 0.42, 0.15, 0.1 |  |  |  | 2 |
| rubberMat | 0.07, 0.08, 0.075 |  |  |  | 1 |
| oilDark | 0.06, 0.058, 0.055 |  |  |  | 0 |
| canvas | 0.44, 0.39, 0.27 |  |  |  | 1 |
| enamelBlue | 0.12, 0.25, 0.5 |  |  |  | 0 |
| brass | 0.6, 0.47, 0.16 |  |  |  | 0 |
| hiVis | 0.85, 0.36, 0.05 |  |  |  | 1 |
| paper | 0.86, 0.86, 0.82 |  |  |  | 3 |

### INK

`tools/levels/livedPieces.ts`: accents of the older levels' furniture.

| Paint | Colour (linear) | Finish | Stands for | Underfoot | Pieces |
|---|---|---|---|---|---|
| cream | 0.86, 0.84, 0.76 |  |  |  | 6 |
| paper | 0.9, 0.89, 0.84 |  |  |  | 6 |
| manila | 0.78, 0.66, 0.42 |  |  |  | 3 |
| brass | 0.62, 0.48, 0.2 |  |  |  | 2 |
| iron | 0.09, 0.09, 0.1 |  |  |  | the same colour as OFFICE_PALETTE.asphalt |
| blueprint | 0.16, 0.3, 0.6 |  |  |  | 2 |
| hivis | 0.9, 0.42, 0.06 |  |  |  | 3 |
| yellow | 0.85, 0.68, 0.1 |  |  |  | 2 |
| baize | 0.1, 0.3, 0.2 |  |  |  | 4 |
| canvas | 0.55, 0.5, 0.38 |  |  |  | 2 |
| oxblood | 0.32, 0.09, 0.07 |  |  |  | 3 |
| navy | 0.08, 0.1, 0.3 |  |  |  | 6 |
| bottle | 0.25, 0.45, 0.7 |  |  |  | 0 |
| rubber | 0.06, 0.06, 0.065 |  |  |  | 1 |
| copper | 0.6, 0.3, 0.15 |  |  |  | 0 |
| tin | 0.05, 0.055, 0.06 |  |  |  | 1 |
| enamel | 0.2, 0.36, 0.3 |  |  |  | 3 |

### One paint name, more than one colour

| Name | Tables and colours |
|---|---|
| brass | LIVED_PAINT (0.6, 0.47, 0.16) · INK (0.62, 0.48, 0.2) |
| canvas | GWOT_PALETTE (0.33, 0.31, 0.2) · LIVED_PAINT (0.44, 0.39, 0.27) · INK (0.55, 0.5, 0.38) |
| iron | GWOT_PALETTE (0.098, 0.1, 0.112) · INK (0.09, 0.09, 0.1) |
| paper | LIVED_PAINT (0.86, 0.86, 0.82) · INK (0.9, 0.89, 0.84) |
| rubber | GWOT_PALETTE (0.03, 0.03, 0.033) · INK (0.06, 0.06, 0.065) |
| tin | GWOT_PALETTE (0.56, 0.55, 0.52) · INK (0.05, 0.055, 0.06) |

### Colours written as numbers

A colour a kit writes by hand, in no table. The look can give such a box no material.

| Colour (linear) | Pieces |
|---|---|
| (0.7, 0.71, 0.72) | officeKit.conduit, officeKit.pipe |
| (0.9, 0.88, 0.8) | officeKit.papers, livedKit.papers |
| (0.07, 0.08, 0.1) | officeKit.car |
| (0.08, 0.08, 0.09) | officeKit.monitor |
| (0.12, 0.12, 0.13) | officeKit.bin |
| (0.138, 0.345, 0.11) | officeKit.plant |
| (0.2, 0.13, 0.08) | livedPieces.boots |
| (0.2, 0.21, 0.23) | officeKit.doorFrame |
| (0.3, 0.02, 0.02) | officeKit.car |
| (0.36, 0.3, 0.26) | officeKit.plant |
| (0.5, 0.51, 0.52) | officeKit.vent |
| (0.58, 0.58, 0.56) | officeKit.cabinet |
| (0.594, 0.54, 0.41) | livedPieces.sack |
| (0.8, 0.8, 0.75) | officeKit.car |

## What the rules say today

- Nothing: every piece stands in a showroom, says what it is and tops out clear of the band the nav grid cannot see.
