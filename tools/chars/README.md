# tools/chars: the character cast

Builds `public/assets/chars/cast.gltf` and `public/assets/chars/cast-gwot.gltf` (the operators, the defenders, the responders and the civilians as
swappable PARTS on the mannequin's skeleton) from nothing but the mannequin already in the repo. No downloads, no Blender needed to build.

```bash
python3 tools/chars/build_cast.py && python3 tools/chars/check_cast.py     # needs numpy only
python3 tools/chars/build_cast.py --look gwot && python3 tools/chars/check_cast.py public/assets/chars/cast-gwot.gltf
```

Renders and deformation checks (headless Blender as a Python module:
`python3.11 -m venv ~/bvenv && ~/bvenv/bin/python -m pip install bpy numpy pillow`, `apt-get install -y --no-install-recommends libegl1`,
ImageMagick). `CAST_GLTF=<path>` in front renders a scratch build or the second file instead of `cast.gltf`:

```bash
~/bvenv/bin/python -I tools/chars/render_cast.py <outdir> lineup turn topdown --spp 48   # Cycles stills
~/bvenv/bin/python -I tools/chars/render_cast.py <outdir> strips --quick                 # Workbench animation strips
~/bvenv/bin/python -I tools/chars/render_cast.py <outdir> turn --quick --only operator_1 # fast look at one variant
```

## Files

| file | what |
|---|---|
| `kit.py` | mannequin loader (welds, islands, weights), ray casting, `Master` lofts, primitives, normals / top-4 weights, the glTF writer |
| `head.py` | the parametric head every hair style and piece of headgear is sampled from |
| `build_cast.py` | the parts and the variants table; run it to write the asset |
| `check_cast.py` | the file-contract check (plain python3 + numpy); exit 1 on any failure |
| `render_cast.py` | bpy: reads the cast with its own parser, skins it to the mannequin's armature, renders |
| `look/` | the look sheets: the cast shaded with the game's own colors (`look/README.md`) |

## The file contract (what `check_cast.py` asserts)

One glTF 2.0 JSON, buffer embedded as base64, no textures, no animations. Each PART = one node with `mesh` + `skin: 0`, unique lowercase name, at most
32 parts. `skins[0].joints` are nodes named as the mannequin's 53 joints (the file carries the mannequin's own node TRS and inverse bind matrices
verbatim, but the loader maps by NAME and uses the mannequin's). Vertex positions are in the mannequin's bind space (meters, +Y up, +Z front, +X = the
character's left, feet on Y=0, T-pose). Every primitive: POSITION, NORMAL, JOINTS_0 (u8 x4), WEIGHTS_0 (f32 x4, normalized, at most 4 non-zero), u16
indices, triangles, one primitive per SLOT per part, material NAME = the slot. `extras.cast = { version: 1, variants: { name: { side, parts[] } } }`.

The slots, eleven, in an order that is never changed (`SLOTS` in `kit.py` and `check_cast.py`):
`skin cloth_uniform cloth_plain webbing boot helmet metal_dark rubber lens accent eye`. `accent` (slot id 9) is the member's marking color: the
operators' pair marks, the response team's white mark, and also the tucked shirt's tie, the straw hat and the tee under the open jacket; what each of
those is drawn as is the side's material table's business (`src/render/charMaterials.ts`). `eye` (slot id 10) is the whites of the eyes, the same
off-white on every side.

Also asserted: index ranges, no NaN, no degenerate triangles, the bounding box, every variant's lowest vertex on the floor, the variants table (every
part worn, no two variants the same set), and the budgets: `MAX_VARIANT_TRIS`, `MAX_UNION_TRIS`, `MAX_UNION_VERTS`, `MAX_PARTS`. A part is one bit of
a 32-bit mask, and all 32 bits are in use in both files, so a new part needs another to leave. The variant `role` strings live in `build_cast.py`
only; the file's `extras.cast.variants` carries `side` and `parts` and nothing else.

`bun run tools/qa/cast-probes.ts` (a row of `tools/qa/checks.ts`) holds the loader and the files from the game's side: `loadGltfCast` of
`src/anim/gltf.ts` against a synthetic cast and files off contract, the shipped files, and the proxy fit per variant (`fitRig` in
`src/render/sdfrig.ts`).

## Method

- **Garments are lofts cast onto the mannequin.** `kit.Master` shoots rings of rays from a limb's axis out to the mannequin's own island (torso, arm,
  leg: the mannequin is five closed islands plus 28 finger pieces), every centimeter, keeps the FARTHEST hit, smooths the radii along the limb (never
  below the raw surface: this is what fills the mannequin's joint bands) and stores the mannequin's skin weights at each hit. A garment samples that
  field at chosen heights with an offset per ring (`tube_of`), so it carries the mannequin's weights at the very points it was cast from and layers
  (skin < trousers < blouse < vest < pouches) never cross. Why lofts rather than duplicating the mannequin's faces: its body is few triangles but in
  bands and steps, and its fingers are many; a loft puts the rings where a hem, cuff, roll or blouse needs them.
- **Hems follow the thighs.** Below the hip joint a growing share of each vertex goes to the thigh on its side (`tail_weights`), less behind than in
  front so that the seat stays with the hips, so a raised thigh carries the hem instead of coming through it. A hanging hem (blouse skirt, shirt tail,
  jacket) is also kept outside both thighs and the trouser seat (`over_legs` in `tube_of`, `leg_envelope`).
- **Hands:** the mannequin's own palm faces (its weights), cut at the hand's own wrist ring, plus new six-sided finger tubes on its finger joints.
- **The head** is a new parametric surface (`head.py`), rigid on `DEF-head`; the neck is the torso loft cut back from the mannequin's bull neck to a
  human one. Hair, caps, hats and the helmet are shells of that same surface. `over_neck` keeps a hair shell outside the neck under the skull.
- **Faces.** `head.py` has three looks, a a man, b a second man, c a woman: `head` is a, `head_b` is b, and c is built into `cleaner`. `chin_strap`
  and the full beard are fitted to faces a and b, and the build refuses a variant that wears them on another.
- **Hard gear** (pouches, pack, holster, goggles, buckles) = chamfered solids rigid on ONE joint each (`stuck`).
- **Straps over the shoulder** are 3D bands (`band3d`). The chest rig's, the tank top's and the bandolier's run along `over_shoulder`: a line of the
  NECK master at a fixed distance off the midline, each cross-section flat on the surface under it and carrying that master's weights, so it lies on
  the slope of the shoulder and moves with the cloth.
- **Body builds.** `Build` scales the masters' ring RADII per (height, angle) and leaves the weights alone, so one skeleton and one set of skin
  weights carry four bodies: `human` (every part unless it names a build), `wiry` (`tank_top`, `bandolier`), `big` (`jacket_open`), `woman`
  (`cleaner`). Nothing is scaled above the point where the head part's neck starts. A part is built inside ONE build (`built(name, fn)` in `PARTS`),
  so a part worn by two variants is the same body on both.
- **Feet.** Soles are flat on the floor (`SOLE`), and `blunt_toe` eases the toe back.
- A defender's sleeve head is scaled down (`SOFT`) and its offsets are small at the shoulder: no squared cap. Operators keep the squared vest
  shoulders.
- Right side = the left side mirrored with .L/.R weights swapped.
- The file is written through a temp file and a rename, so the file on disk is always whole. `build_cast.py <path>` writes elsewhere.

## The two files

The game draws the men out of one of two files by where the page booted (`lookDress` in `src/scene/look.ts`, `look_dress` on the Rust side):
`cast-gwot.gltf` in every place but the office family, `cast.gltf` there. Both keep the same contract, with the same 14 variant names on the same
sides in the same order, so `rust/game_rules/src/outfit/cast.rs` deals a man the same variant name under either. Operators, responders and civilians have the
same part lists in both. Colors are in `src/render/charMaterials.ts` (`CHAR_MATS`, `CHAR_MATS_LOOK`).

### Variants of `cast.gltf`

| variant | side | what tells him apart | parts |
|---|---|---|---|
| operator_1 | operator | team leader: radio box on the back, tape antenna, face a | head, helmet, hair_short, hands_glove, op_suit, nvg_up, kit_lead |
| operator_2 | operator | breacher: ram slung across the back, charge satchel, shell loops, face a | head, helmet, hair_short, hands_glove, op_suit, nvg_up, kit_breacher |
| operator_3 | operator | rifleman: small assault pack, face b | head_b, helmet, hair_short, hands_glove, op_suit, nvg_up, kit_rifleman |
| operator_4 | operator | rifleman: slick, hydration carrier, moustache, face a | head, helmet, hair_short, hands_glove, op_suit, nvg_up, kit_slick, moustache |
| operator_5 | operator | medic: big pack and roll, face b | head_b, helmet, hair_short, hands_glove, op_suit, nvg_up, kit_medic |
| defender_1 | defender | ball cap, loose shirt, fatigues, boots, face a | head, hands_skin, hair_short, cap_ball, shirt_loose, trousers_fatigue, boots_low |
| defender_2 | defender | wiry: straw hat, tank top, bandolier, jeans, face b | head_b, hands_skin, hair_full, hat_straw, tank_top, bandolier, trousers_jeans, shoes |
| defender_3 | defender | big: bandana, open jacket over a tee, moustache, face a | head, hands_skin, moustache, hair_short, bandana, jacket_open, trousers_fatigue, boots_low |
| defender_4 | defender | bare head, shirt and chest rig, jeans, face a | head, hands_skin, hair_full, shirt_loose, chest_rig, trousers_jeans, boots_low |
| defender_5 | defender | boonie hat, shirt and chest rig, face b | head_b, hands_skin, hair_short, hat_boonie, shirt_loose, chest_rig, trousers_fatigue, shoes |
| responder_1 | responder | steel helmet, flak vest, back roll, gloves; the radio man, face b | head_b, hands_glove, hair_short, shirt_tucked, rsp_kit, trousers_fatigue, boots_low, rsp_radio |
| responder_2 | responder | the same kit without the radio, face a | head, hands_glove, hair_short, shirt_tucked, rsp_kit, trousers_fatigue, boots_low |
| civilian_1 | civilian | the cleaner: woman build, tunic, apron, kerchief, her own face inside the part | hands_skin, cleaner, shoes |
| civilian_2 | civilian | office worker: shirt, tie, belt, fuller hair, moustache, face a | head, hands_skin, hair_full, moustache, shirt_tucked, trousers_fatigue, shoes |

### What `cast-gwot.gltf` changes (`--look gwot`)

- **Operators** keep their part lists. `helmet` is `part_helmet_gwot`: the same shell plus two side rails, a counterweight pouch on the back and a
  marker-light block. The rest of their read is color (`CHAR_MATS_LOOK`). The pair marking is small in this file (`part_helmet('gwot')`,
  `part_op_suit('gwot')`): one solid panel on the crown (`crown_panel`, `CROWN_HALF`), a short band across the back of the shell (`cat_eye`), and a
  patch on the top of each deltoid, in place of `cast.gltf`'s band, stripe (`crown_stripe`), brassards and shoulder panels (`op_shoulder_marks`).
- **Defenders** are re-dressed. Nine parts of `cast.gltf` are dropped (`_GONE`:
  `cap_ball hat_boonie hat_straw bandana shirt_loose tank_top jacket_open bandolier trousers_jeans`) and nine new ones take their bits: `local_suit`
  (a knee-length loose shirt over loose trousers, slot `cloth_plain`) · `waistcoat` (`cloth_uniform`) · `patu` (a shawl over the shoulders, `lens`) ·
  `scarf` (a shemagh, `lens`) · `pakol` (a flat wool cap, `helmet`) · `turban` (`accent`) · `kufi` (`accent`) · `beard_full` (`rubber`) · `sandals`
  (skin foot, `metal_dark` sole, `boot` straps). `chest_rig`, `shoes`, `boots_low`, `hair_short`, `hair_full` and `moustache` are the first file's.
  The five (`VARIANTS_GWOT`): pakol, waistcoat, beard · turban, chest rig, beard, sandals · kufi, shawl, sandals · bare head, shemagh, chest rig,
  beard · pakol, shemagh, waistcoat. All five stand on the one `human` build.
- **Slots re-used for cloth:** the shawl and the shemagh ride `lens`, the turban and the kufi `accent`, slots no other defender part uses, so each
  takes its own pick from its own palette (`CHAR_MATS_LOOK`).

Look sheets of this file: `look/README.md` says how to name it.

## The goggle mount on the operators' helmet

What is in the hands and on the heads (rifles, pistols, goggles) is not this file's. The helmet (`helmet`, both files, every operator variant) carries
a brow plate rigid to `DEF-head`: a 4.8 × 4.4 cm block on the shell's center line (`part_helmet`, "NVG mount plate"). The plate, the shell and the
brow are the same in both files, and the two files' `DEF-head` bind matrices are the same, so goggles hung off the head bone at `Rig.goggles` sit on
either helmet alike.

The center of its OUTER FACE, the point to hang a goggle arm on, in meters, bind pose (x the figure's left, y up, z ahead): (0, 1.775, 0.142), facing
(0, 0.469, 0.883), which is 28° above straight ahead, with the plate's own up (0, 0.883, −0.469). `DEF-head`'s bind origin is (0, 1.5687, 0.0051).

Goggles today: the rig's box pair (`goggles_boxes`, `Rig.goggles` in `rust/game_types/src/rig_props/types.rs`) and the mesh part `nvg_up` (a
flipped-up pair with an arm to this plate, drawn only on an operator the game gives no box goggles: `castGoggles` in `src/anim/gltf.ts`). The sneaking
suit (`castSuit`) takes off the helmet, every `kit_*` and the mesh goggles.

## Known rough

Faces are mask-like. The short sleeve has a squared cap and a step at the sleeve head. The trousers' crotch shows a crease. The goggle block is small.
The tank top's neckline is a straight edge between its straps. The shawl lies as close as a yoke, and where a man's pick of wrap is near his shirt's
color it is hard to see. The turban is a smooth dome with shallow ridges. The long shirt's hem does not swing (it is skinned to the thighs). Nobody
has a rolled sleeve. None of the cast has been judged on a GPU by this folder's tools: the sheets are Blender's.
