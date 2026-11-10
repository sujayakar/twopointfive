// Written by tools/draw/gen.ts from rust/game_draw. To change a section, change it there.

/** the stamp of the sections this was made from: a module of another stamp is refused (drawn.ts) */
export const LAYOUT = 1273517202;
/** what a host may ask for: what is made together is asked for together */
export const enum Want { fog = 1, view = 2, sky = 4, boxes = 8, lights = 16, men = 32, doorways = 64, rims = 128, events = 256, fires = 512, decals = 1024, air = 2048, levelLook = 4096, snow = 8192, smoke = 16384, all = 32767 }
/** each section's place in the table the module answers with */
export const enum Section { fog = 0, view = 1, sky = 2, head = 3, geo = 4, mat = 5, cut = 6, visible = 7, marks = 8, cells = 9, items = 10, info = 11, panes = 12, leaves = 13, lights = 14, coming = 15, men = 16, insts = 17, bones = 18, caps = 19, doorways = 20, emits = 21, kicks = 22, pushes = 23, sources = 24, anchors = 25, beams = 26, sounds = 27, footfalls = 28, lasting = 29, heard = 30, rims = 31, fires = 32, fireTable = 33, decals = 34, decalKeys = 35, decalHead = 36, winds = 37, gusts = 38, heat = 39, air = 40, windRoofs = 41, roofs = 42, ceilings = 43, doorBoxes = 44, openPanes = 45, resets = 46, traffic = 47, dust = 48, feet = 49, things = 50, blasts = 51, snowWords = 52, smoke = 53, smokeAt = 54, smokeBricks = 55, count = 56 }

/** the fog: one row */
export const enum Fog {
  /** u32: the fog is drawn; 0 = every other field is 0 */
  on = 0,
  /** u32: live eyes */
  count = 1,
  /** f32 × 28: an eye: x y z, then the man's id (0 = none) */
  eyes = 2,
  /** f32 × 28: where it looks in plan (x, z; unit), the cosine of its half field (-2 = all round), its all-round near field in metres */
  looks = 30,
  /** f32 */
  dim = 58,
  /** f32 */
  desat = 59,
  /** f32 */
  topPad = 60,
  /** f32: the band the fog comes in over across a sight edge, metres */
  edge = 61,
  /** f32: the wedge's soft edge: the cosines of the half field less and plus half the soft band, and the metres the near field's rim comes in over */
  softIn = 62,
  /** f32 */
  softOut = 63,
  /** f32 */
  nearSoft = 64,
  /** f32 */
  flat = 65,
  /** f32: the lamps' strength in the fog (0 = none), the fill, the level under which a lamp is not traced, and the fill's corner shade and its reach */
  lit = 66,
  /** f32 */
  litFill = 67,
  /** f32 */
  litCut = 68,
  /** f32 */
  litAo = 69,
  /** f32 */
  litAoM = 70,
  /** u32 */
  lightCount = 71,
  /** u32 × 768: the lamps, as the scene's own light record is packed: 16 words a lamp */
  lights = 72,
  /** u32: the full picture under the fog, and its fill */
  full = 840,
  /** f32 */
  fullFlat = 841,
  /** u32: smoke and dust are drawn over floor nobody sees */
  air = 842,
  /** u32: thin things do not cut a sight line */
  thin = 843,
  /** u32: what lies still stays as last seen */
  kept = 844,
  /** u32 × 128: which statics are thin, a bit a box, and the list they were read off: its geometry serial and its length */
  thinBits = 845,
  /** u32 */
  thinSerial = 973,
  /** u32 */
  thinCount = 974,
  /** words a row */
  row = 975,
}

/** the seat this machine shows: one row */
export const enum View {
  /** u32: the tick this is of: sixtieths of a second on the game's clock */
  tick = 0,
  /** u32: moves when the game is another than the one last drawn (a restart, a load, another seat): nothing is drawn or heard through it */
  stood = 1,
  /** u32: the shown man's id */
  man = 2,
  /** f32 × 3: where he stands, and where the view follows (him, or a man spectated) */
  at = 3,
  /** f32 × 3 */
  follow = 6,
  /** f32 × 3: where his aim meets the world */
  aim = 9,
  /** u32: the view is another man's: nothing leans on the cursor */
  spectating = 12,
  /** f32 × 3: the view is held off the follow point by this much (a peek round a wall's end), and kicked for this frame's picture by that much */
  peek = 13,
  /** f32 × 3 */
  kick = 16,
  /** u32: the tick cut the view to here: once each time the count moves */
  cuts = 19,
  /** f32 × 3 */
  cut = 20,
  /** u32: the kit picture holds the view here: where it looks, and from how far and which way (0: nobody does) */
  pinned = 23,
  /** f32 × 3 */
  pin = 24,
  /** f32 */
  pinYaw = 27,
  /** f32 */
  pinPitch = 28,
  /** f32 */
  pinDistance = 29,
  /** u32: the tick turned the host's own view: 1 its yaw · 2 its pitch · 4 its distance · 8 where it looks, to these */
  turned = 30,
  /** f32 */
  yaw = 31,
  /** f32 */
  pitch = 32,
  /** f32 */
  distance = 33,
  /** f32 × 3 */
  target = 34,
  /** u32: what a lesson does not let the hand do to the view: 1 turn it · 2 zoom it */
  locks = 37,
  /** u32: the back key is the game's now: it would put something away (a list, an open order menu, a running verb, a held throw, a gun's setup) */
  backs = 38,
  /** u32: saves made of this game so far: moves when there is another */
  saves = 39,
  /** u32: what the game is set to, for a screen that sets it: hits do nothing to him, the men think, unlimited ammunition, it saves by itself, the wall hold's feel, walking into a wall takes it, the command key tapped twice calls the squad in */
  god = 40,
  /** u32 */
  thinks = 41,
  /** u32 */
  ammo = 42,
  /** u32 */
  autosave = 43,
  /** u32 */
  wallFeel = 44,
  /** u32 */
  wallPress = 45,
  /** u32 */
  fallIn = 46,
  /** u32: what each man is thinking is said over him */
  labels = 47,
  /** u32: the run's end card is due: 1 it was lost, 2 won (0: not yet) */
  ended = 48,
  /** u32: this run, for whoever keeps what was won: 1 its try has begun · 2 it is over (what there is to keep is asked: face_run_view) */
  run = 49,
  /** f32: night vision's amount, and how hard he has just been hit */
  night = 50,
  /** f32 */
  hit = 51,
  /** u32: this game is the Team & kit picture: 1 staged, 2 nothing to stage (0: it is no picture) */
  kit = 52,
  /** u32: the Team & kit picture asks for a copy of the last picture over what it is about to stage again: moves each time; answered by a command */
  veil = 53,
  /** words a row */
  row = 54,
}

/** the sky: one row */
export const enum Sky {
  /** f32 × 3: the sky's colours up and level, the haze and the exposure the picture is taken at */
  zenith = 0,
  /** f32 × 3 */
  horizon = 3,
  /** f32 */
  haze = 6,
  /** f32 */
  exposure = 7,
  /** u32: polished floors mirror */
  gloss = 8,
  /** u32: the hour that is up (0 night, 1 dawn, 2 midday, 3 overcast, 4 dusk) and how strong its sun is in the open */
  hour = 9,
  /** f32 */
  hourSun = 10,
  /** f32: the cover of the weather that is picked (under 0: none is), and the share of the hour's own sun that comes through */
  cover = 11,
  /** f32 */
  sunShare = 12,
  /** u32: the level's moon: 0 it has none, 1 it has, 2 it is up; its strength, its height and bearing in degrees */
  moon = 13,
  /** f32 */
  moonK = 14,
  /** f32 */
  moonElev = 15,
  /** f32 */
  moonBearing = 16,
  /** u32: snow lies on this level */
  snowLevel = 17,
  /** f32 × 3: the level's own sky, how kept the place is (under 0: it does not say) and its marks' texel (0: the usual) */
  ownZenith = 18,
  /** f32 × 3 */
  ownHorizon = 21,
  /** f32 */
  kept = 24,
  /** f32 */
  markTexel = 25,
  /** u32: the weather in the open air */
  weather = 26,
  /** f32 */
  rain = 27,
  /** f32 */
  snow = 28,
  /** f32 × 2 */
  shear = 29,
  /** f32 */
  wet = 31,
  /** f32 × 3 */
  flash = 32,
  /** f32 */
  rainTop = 35,
  /** f32 × 4 */
  interior = 36,
  /** f32 × 4 */
  annex = 40,
  /** u32 */
  roofC = 44,
  /** u32 */
  roofD = 45,
  /** u32 */
  fog = 46,
  /** f32 × 4 */
  nearFade = 47,
  /** f32 × 4: the streaks' phase function; its last number 0 = none said */
  phase = 51,
  /** words a row */
  row = 55,
}

/** what the page is told of the boxes' tables this frame: one row */
export const enum Head {
  /** u32: how many boxes, how many of them statics */
  count = 0,
  /** u32 */
  statics = 1,
  /** u32: the statics' rows that changed since the last frame the host took: from, and to the one after (from = to: none) */
  from = 2,
  /** u32 */
  to = 3,
  /** u32: plain boxes drawn, and after them in the list the cut ones, the first of which a ray can meet */
  visible = 4,
  /** u32 */
  visibleCut = 5,
  /** u32 */
  visibleCutTraced = 6,
  /** u32: cut boxes a ray can meet */
  cutTraced = 7,
  /** u32: the rows of the cut table written this frame: the statics' and the frame's own */
  cutFrom = 8,
  /** u32 */
  cutTo = 9,
  /** u32 */
  cutFromOwn = 10,
  /** u32 */
  cutToOwn = 11,
  /** u32: the marking table grew */
  marksMoved = 12,
  /** u32: the grid is another than the last frame's, and how many items it holds */
  gridMoved = 13,
  /** u32 */
  items = 14,
  /** u32: the panes' broken cells are to be sent */
  panesMoved = 15,
  /** u32: the game's own count of what broke of the statics (the fog's thin bits are read off the list of that count) */
  serial = 16,
  /** u32: which stuffs some box has been, a bit a stuff's number */
  stuffs = 17,
  /** f32: the boxes a ray's grid reaches past the cells they are registered in (metres) */
  slack = 18,
  /** u32: keyed boxes found a frame ago, met for the first time, and boxes with no key */
  matched = 19,
  /** u32 */
  fresh = 20,
  /** u32 */
  unkeyed = 21,
  /** u32: the order the statics are drawn in is of the list as it stood some frames ago */
  orderStale = 22,
  /** words a row */
  row = 23,
}

/** a box's place: cx cy cz, yaw's cosine and sine as halves · hx hy hz, flags */
export const enum Geo {
  /** u32 × 8 */
  w = 0,
  /** words a row */
  row = 8,
}

/** its material: colour and cut height · emission and finish · rotation · the colour a secondary ray finds and its marks' gain */
export const enum Mat {
  /** f32 × 16 */
  w = 0,
  /** words a row */
  row = 16,
}

/** its planes, where it is cut: eight of n and d, then the core's colour */
export const enum Cut {
  /** f32 × 36 */
  w = 0,
  /** words a row */
  row = 36,
}

/** one word of a list */
export const enum Word {
  /** u32 */
  w = 0,
  /** words a row */
  row = 1,
}

/** four floats of a table */
export const enum Row4 {
  /** f32 × 4 */
  w = 0,
  /** words a row */
  row = 4,
}

/** a door's leaf where the tick left it: what the picture's own plans ask of it (openings, places, what shuts smoke in) */
export const enum Leaf {
  /** f32 × 3 */
  c = 0,
  /** f32 × 3 */
  h = 3,
  /** f32 */
  yaw = 6,
  /** u32 */
  flags = 7,
  /** u32 */
  key = 8,
  /** words a row */
  row = 9,
}

/** a light */
export const enum Light {
  /** u32: which light it is, for as long as it burns */
  key = 0,
  /** u32: 0 a point, 1 a spot, 2 the sky's, 3 a panel */
  kind = 1,
  /** u32: what it is by day: 0 drawn as it is, 1 a fixture, 2 in a hand, 3 a vehicle's, 4 a flash, 5 a glowing face */
  class = 2,
  /** u32: 1 it burns · 2 transient · 4 broad · 8 a carried spot · 16 infrared · 32 it moves (a hand's, Sam's own) */
  says = 3,
  /** u32 */
  owner = 4,
  /** i32: the marked box and the mark it is the light of (under 0: none) */
  mark = 5,
  /** f32 × 3 */
  pos = 6,
  /** f32 × 3 */
  dir = 9,
  /** f32 × 3 */
  color = 12,
  /** f32 */
  intensity = 15,
  /** f32 */
  peak = 16,
  /** f32 */
  range = 17,
  /** f32 */
  inner = 18,
  /** f32 */
  outer = 19,
  /** f32 */
  radius = 20,
  /** f32 */
  volumetric = 21,
  /** f32 */
  flicker = 22,
  /** f32 */
  ttl = 23,
  /** f32 */
  age = 24,
  /** f32: its class under the fog (0 none, 1 only what the team sees, 2 only what it does not), and the strength it is remembered at where it is out */
  fog = 25,
  /** f32 */
  ghost = 26,
  /** words a row */
  row = 27,
}

/** a light of the sky's that has not come yet: the way it will shine and how wide it is (none, or the one that is next) */
export const enum Coming {
  /** f32 × 3 */
  dir = 0,
  /** f32 */
  radius = 3,
  /** words a row */
  row = 4,
}

/** a man drawn: what the page's own tables dress him by */
export const enum Man {
  /** u32 */
  id = 0,
  /** u32: whose table he wears: 0 a civilian's, 1 an operator's, 2 a defender's, 3 a responder's; and whether he is in the sneaking suit */
  side = 1,
  /** u32 */
  suit = 2,
  /** u32: the cast's parts he wears, a bit a part (0: every one) */
  parts = 3,
  /** u32 */
  alive = 4,
  /** f32 × 3 */
  tint = 5,
  /** f32 × 3 */
  tint2 = 8,
  /** f32: his height beside the skeleton's */
  stature = 11,
  /** f32: how much his proxies are darkened by a wet coat (1: dry) */
  dry = 12,
  /** f32 × 3 */
  at = 13,
  /** f32 × 3: where he is drawn standing: between the last two ticks, as his bones are */
  feet = 16,
  /** f32 × 3: … and where his head is drawn */
  head = 19,
  /** f32 × 4: the sphere round his proxies */
  bound = 22,
  /** words a row */
  row = 26,
}

/** a man's instance, as the skinning reads it: tint, bones' place · tint2, owner · wet coat · its knobs · the waterline, the parts he wears */
export const enum Inst {
  /** u32 × 24 */
  w = 0,
  /** words a row */
  row = 24,
}

/** a man's bones: a matrix a bone */
export const enum Bones {
  /** f32 × 1024 */
  w = 0,
  /** words a row */
  row = 1024,
}

/** one of his proxies: an end and its radius, the other end, and whose it is */
export const enum Cap {
  /** f32 × 7 */
  w = 0,
  /** u32 */
  owner = 7,
  /** words a row */
  row = 8,
}

/** a doorway: its hinge on the plan, which way the shut leaf lies from it, and how wide it is */
export const enum Doorway {
  /** f32 × 2 */
  hinge = 0,
  /** f32 */
  closedDir = 2,
  /** f32 */
  width = 3,
  /** words a row */
  row = 4,
}

/** particles thrown */
export const enum Emit {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** u32: render/particles.ts kinds, by their number in PKINDS; how many; and 1 = the team's own doing */
  kind = 2,
  /** u32 */
  count = 3,
  /** u32 */
  ours = 4,
  /** u32: EMIT_SHAPES, one more than its number (0: not said); the owner, and the seed (not said: 0xffffffff) */
  shape = 5,
  /** u32 */
  owner = 6,
  /** u32 */
  seed = 7,
  /** f32 × 3 */
  pos = 8,
  /** f32 × 3 */
  dir = 11,
  /** f32 */
  speed = 14,
  /** f32 */
  life = 15,
  /** f32 */
  size = 16,
  /** f32 × 13: spread, speed_jit, life_jit, size_jit, lift, radius, pos_jit, top, aspect, tumble, nrm_jit, steam, ember (NaN: not said) */
  more = 17,
  /** f32 × 3 */
  vadd = 30,
  /** f32 × 3 */
  color = 33,
  /** words a row */
  row = 36,
}

/** what lies about is kicked */
export const enum Kick {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** u32: a bit a kind (0: the usual); 1 = a wall between stops it */
  kinds = 2,
  /** u32 */
  occlude = 3,
  /** u32 */
  seed = 4,
  /** f32 × 3 */
  pos = 5,
  /** f32 */
  radius = 8,
  /** f32 */
  strength = 9,
  /** f32 */
  up = 10,
  /** f32 × 3 */
  bias = 11,
  /** f32 × 3 */
  eye = 14,
  /** words a row */
  row = 17,
}

/** the air is pushed */
export const enum AirPush {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** u32: 0 at a point, 1 along a line, 2 by a standing body, 3 a jet, 4 outward, 5 a round's path, 6 a walking man's stride */
  shape = 2,
  /** f32 × 3 */
  pos = 3,
  /** f32 × 3: the velocity the air is taken to, and which way the shape lies */
  vel = 6,
  /** f32 × 3 */
  dir = 9,
  /** f32 */
  radius = 12,
  /** f32: half the line, or half the body's height */
  len = 13,
  /** f32: how hard (a second, at the middle), and how much the air is made to part there (NaN: not said) */
  hard = 14,
  /** f32 */
  expand = 15,
  /** f32: how long it goes on (0: the one tick) */
  ttl = 16,
  /** u32: 0 every lattice, 1 the fine one, 2 the coarse */
  lattice = 17,
  /** words a row */
  row = 18,
}

/** smoke comes from somewhere, for a while */
export const enum SmokeSource {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** u32: SMOKE_KINDS */
  kind = 2,
  /** u32: the push that goes with it, one more than its serial in this tick (0: none) */
  push = 3,
  /** u32: what it rides, if anything (`anchors`; 0: it stays) */
  anchor = 4,
  /** u32: 1 the picture's alone · 2 gone when what it rides is · 4 its ride's direction is the smoke's velocity · 8 the men see through it too · 16 a fire's, for the preset that sheds those */
  says = 5,
  /** u32 */
  lattice = 6,
  /** f32 × 3 */
  pos = 7,
  /** f32 */
  radius = 10,
  /** f32 */
  density = 11,
  /** f32 */
  heat = 12,
  /** f32 */
  ttl = 13,
  /** f32 */
  age = 14,
  /** f32: NaN: not said */
  attack = 15,
  /** f32 */
  jitter = 16,
  /** f32 */
  prio = 17,
  /** f32 */
  minCells = 18,
  /** f32 × 3 */
  tint = 19,
  /** words a row */
  row = 22,
}

/** where something rides, for as long as a source rides it */
export const enum Anchor {
  /** u32 */
  key = 0,
  /** f32 × 3 */
  pos = 1,
  /** f32 × 3 */
  dir = 4,
  /** words a row */
  row = 7,
}

/** a line of light (a weapon's laser, a tracer's streak): its box, along its own z, and what it emits */
export const enum Beam {
  /** f32 × 3 */
  c = 0,
  /** f32 × 3 */
  h = 3,
  /** f32 × 4 */
  rot = 6,
  /** f32 × 3 */
  emits = 10,
  /** u32: 0 a mirror shows it, 1 the air does, 2 the air does and it ends on a dot */
  air = 13,
  /** words a row */
  row = 14,
}

/** a sound */
export const enum Sound {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** u32: its name's number (`face_draw_name`) */
  name = 2,
  /** u32: it has a place; 0 = it is in the ears */
  placed = 3,
  /** f32 × 3 */
  pos = 4,
  /** f32: NaN: not said */
  gain = 7,
  /** f32 */
  rate = 8,
  /** f32 */
  lowpass = 9,
  /** f32 */
  pan = 10,
  /** f32 */
  wet = 11,
  /** words a row */
  row = 12,
}

/** a footfall */
export const enum Footfall {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** u32 */
  own = 2,
  /** u32: what it landed on, by its number in FLOORS */
  floor = 3,
  /** f32 × 3 */
  pos = 4,
  /** f32 */
  loud = 7,
  /** words a row */
  row = 8,
}

/** a thing that sounds for as long as it goes on, as it stands: a chair that rolls, a fire that burns, a can that pours, what hums in a room */
export const enum Lasting {
  /** u32: the sound's name's number (`face_draw_name`), and which of that name it is */
  name = 0,
  /** u32 */
  id = 1,
  /** f32 × 3 */
  at = 2,
  /** f32 */
  gain = 5,
  /** f32: how hard it goes, 0 to 1: its level and pitch ride it (a fire at full heat two thirds) */
  hard = 6,
  /** f32: how long it has gone on, and how much longer it will (NaN: nobody knows) */
  age = 7,
  /** f32 */
  left = 8,
  /** f32: how far off there is nothing of it (0: as far as it carries) */
  reach = 9,
  /** words a row */
  row = 10,
}

/** what is heard all the while: one row */
export const enum Heard {
  /** f32: the game's clock */
  time = 0,
  /** f32: the rain's level where the listener stands */
  rain = 1,
  /** f32: how tense the floor is, by the nerves of those who are not the player's own: what the score follows */
  tension = 2,
  /** u32: which run this is (a fresh one is the score's cue) */
  run = 3,
  /** words a row */
  row = 4,
}

/** a man of the team: the first row is the shown player's own */
export const enum Rim {
  /** u32 */
  id = 0,
  /** u32: 0 the first player's, 1 the red pair's, 2 the blue pair's, 3 the second player's */
  hue = 1,
  /** f32 × 3 */
  at = 2,
  /** words a row */
  row = 5,
}

/** a fire that burns, or that the fog's picture has burning still */
export const enum Fire {
  /** u32: its place among the level's fires, which is who it is */
  ix = 0,
  /** u32: it burns (else it is out, and only the fog's picture has it) */
  lit = 1,
  /** u32: its class under the fog: 0 drawn anywhere · 1 only where the team sees · 2 only where it does not */
  fog = 2,
  /** f32: its own number 0‥1 off where it stands */
  seed = 3,
  /** f32 × 3: its mouth and how wide: the middle of the glowing box's top, the flame's radius, the bed of coals' */
  pos = 4,
  /** f32 */
  r = 7,
  /** f32 */
  bed = 8,
  /** f32: how high a calm flame stands, how hot it burns, the clock it is drawn at, how its lamp stands to the authored one */
  h = 9,
  /** f32 */
  heat = 10,
  /** f32 */
  time = 11,
  /** f32 */
  flick = 12,
  /** f32 × 2: what leans the flame (capped), and the air at it as it blows */
  wind = 13,
  /** f32 × 3 */
  blows = 15,
  /** words a row */
  row = 18,
}

/** the numbers every fire is drawn by: one row, on a level that has a fire */
export const enum FireTable {
  /** f32 × 16: fire.wgsl FireLook, in its order */
  look = 0,
  /** f32: how far off the view a flame is drawn, how many, the flame's height and how far the wind leans it */
  drawR = 16,
  /** f32 */
  max = 17,
  /** f32 */
  height = 18,
  /** f32 */
  tall = 19,
  /** f32 */
  lean = 20,
  /** f32 × 12: its smoke: on, up, radius, cells, density, temp, speed, push, jitter, every, view_r, lean */
  smoke = 21,
  /** f32 × 14: one column: up, radius, density, soot_ease, puff_every, puff_secs, puff_up, puff_radius, puff_off, puff_density, puff_temp, puff_speed, puff_push, puff_expand */
  column = 33,
  /** f32 × 3 */
  soot = 47,
  /** words a row */
  row = 50,
}

/** who a record is, in the same order: the id of what made it (its tick; the arm in the top byte and its place among that tick's) */
export const enum DecalKey {
  /** u32 */
  tick = 0,
  /** u32 */
  made = 1,
  /** words a row */
  row = 2,
}

/** what the page is told of the marks' buffer this frame: one row */
export const enum DecalHead {
  /** u32: a word of it moved: send it */
  moved = 0,
  /** u32: marks in the list, and of them drawn this frame */
  count = 1,
  /** u32 */
  drawn = 2,
  /** u32: entries of the cells' lists, and left out of them by their caps */
  listed = 3,
  /** u32 */
  dropped = 4,
  /** words a row */
  row = 5,
}

/** a source of moving air */
export const enum Wind {
  /** u32: 0 a jet, 1 the storm */
  kind = 0,
  /** u32: 1 the storm is a field cut roof by roof · 2 the air of a fire */
  says = 1,
  /** u32: which particles it moves, a bit a kind (0: the usual) */
  kinds = 2,
  /** u32: the roofs it blows between: its first row in `wind_roofs`, and how many (0: its two rectangles) */
  roofsAt = 3,
  /** u32 */
  roofs = 4,
  /** f32 × 3 */
  pos = 5,
  /** f32 × 3 */
  dir = 8,
  /** f32 */
  speed = 11,
  /** f32 */
  reach = 12,
  /** f32 */
  tanSpread = 13,
  /** f32 */
  r0 = 14,
  /** f32 */
  dMax = 15,
  /** f32: NaN: not said */
  floorY = 16,
  /** f32 × 4 */
  interior = 17,
  /** f32 × 4 */
  annex = 21,
  /** f32 × 4: the sphere it reaches: middle, radius */
  bound = 25,
  /** words a row */
  row = 29,
}

/** the weather's air: one row */
export const enum Air {
  /** f32: the wind's master (0: still air), how fast the solver's air answers it, and the particles' three numbers */
  master = 0,
  /** f32 */
  relax = 1,
  /** f32 × 3 */
  knobs = 2,
  /** f32: the storm: how fast it blows now, which way, its gust, and the share of its speed at the ground (0: no storm source) */
  stormSpeed = 5,
  /** f32 × 3 */
  stormDir = 6,
  /** f32 */
  stormGust = 9,
  /** f32: how far from a roof the storm's field has come in whole (m) */
  stormLee = 10,
  /** u32: a rain falls by the level's own roofs (`roofs`), else by the two rectangles */
  levelRoofs = 11,
  /** f32 × 4: the two rectangles */
  interior = 12,
  /** f32 × 4 */
  annex = 16,
  /** words a row */
  row = 20,
}

/** the corners of a rectangle on the plan: x0 z0 x1 z1 */
export const enum Rect {
  /** f32 × 4 */
  r = 0,
  /** words a row */
  row = 4,
}

/** the tick began a part of the picture again */
export const enum Reset {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** u32: 1 the smoke · 2 the cans venting stop, what they let out left in the air */
  what = 2,
  /** words a row */
  row = 3,
}

/** peak opacity at its centre · × radius over its life · seconds to full opacity · × the softening where it meets a surface · the share of a crossing beam scattered toward the eye · × the cache's ambient on it */
export const enum DustLook {
  /** f32 */
  opacity = 0,
  /** f32 */
  growth = 1,
  /** f32 */
  fadeIn = 2,
  /** f32 */
  soft = 3,
  /** f32 */
  wrap = 4,
  /** f32 */
  ambient = 5,
  /** words a row */
  row = 6,
}

/** a man, as what stands on the snow and breathes over it */
export const enum Foot {
  /** u32 */
  id = 0,
  /** u32 */
  says = 1,
  /** f32 × 3 */
  pos = 2,
  /** f32 × 3 */
  vel = 5,
  /** f32 */
  yaw = 8,
  /** f32 × 3 */
  hips = 9,
  /** f32 × 3 */
  head = 12,
  /** f32 × 3 */
  chest = 15,
  /** f32 × 3 */
  footL = 18,
  /** f32 × 3 */
  footR = 21,
  /** f32 × 3 */
  toeL = 24,
  /** f32 × 3 */
  toeR = 27,
  /** f32 × 3: where his head faces */
  facing = 30,
  /** words a row */
  row = 33,
}

/** a part of a thing that moves over the ground: each of a prop's parts, a thrown thing, a chunk of debris */
export const enum Thing {
  /** u32: 0 a prop's part · 1 a thrown thing · 2 a chunk */
  kind = 0,
  /** u32 */
  key = 1,
  /** u32 */
  part = 2,
  /** u32: 1 at rest · 2 lying on something that is not the ground */
  says = 3,
  /** f32 × 3 */
  c = 4,
  /** f32 × 3 */
  h = 7,
  /** f32 */
  yaw = 10,
  /** f32 × 2: where the whole thing stands, its velocity and its turn (a prop's) */
  at = 11,
  /** f32 × 3 */
  vel = 13,
  /** f32 */
  spin = 16,
  /** f32: the whole thing's radius and weight, and how far over the floor this part's underside hangs */
  radius = 17,
  /** f32 */
  mass = 18,
  /** f32 */
  foot = 19,
  /** words a row */
  row = 20,
}

/** a blast went off */
export const enum SnowBlast {
  /** u32 */
  tick = 0,
  /** u32 */
  serial = 1,
  /** f32 × 3 */
  at = 2,
  /** f32 */
  radius = 5,
  /** words a row */
  row = 6,
}

/** what snow a static is, by its place: 0 none · 1 snow · 2 trodden · 3 old */
export const enum SnowWord {
  /** u32 */
  w = 0,
  /** words a row */
  row = 1,
}

/** the grid's plan: one row, on a level that has one and knows its ways */
export const enum SmokeHead {
  /** u32: how many times the rows were made */
  made = 0,
  /** u32: the plan in bricks, along x and along z */
  wide = 1,
  /** u32 */
  deep = 2,
  /** words a row */
  row = 3,
}

/** a brick that stands: four half-metre cells along x, then along z, then eight up */
export const enum SmokeBrick {
  /** f32 × 128: how much smoke is in each cell, in the solver's own unit as it is drawn */
  rho = 0,
  /** u32 × 32: a byte a cell: 1 = open toward +x · 2 = up · 4 = toward +z · 8 = asked */
  open = 128,
  /** u32: what is missing from its color: a byte of nothing, then 1 − tint as r, g, b (smoke.wgsl WHAT IS IN A CELL) */
  dye = 160,
  /** words a row */
  row = 161,
}

/** what a box may say it is made of, by its number in Rust */
export const STUFFS = ['carpet', 'grass', 'rubber', 'metal', 'glass', 'wood', 'plastic', 'fabric', 'steel', 'enamel', 'tarmac', 'mud wall', 'plaster', 'block', 'earth', 'gravel', 'slab', 'sheet', 'cloth', 'paint', 'veneer', 'carpaint', 'stucco', 'skim', 'gloss block', 'board concrete', 'cast panel', 'paver', 'brushed', 'louvre', 'plain', 'waits'] as const;

/** what a particle is, by its number in Rust */
export const PKINDS = ['rain', 'dust', 'casing', 'sheet', 'spark', 'debris', 'shard'] as const;

/** how particles are thrown, by its number in Rust */
export const EMIT_SHAPES = ['cone', 'burst', 'ring', 'puff', 'rainDisc'] as const;

/** what smoke comes from, by its number in Rust */
export const SMOKE_KINDS = ['flash', 'wisp', 'canister', 'burst', 'cloud', 'jet', 'trail', 'vent', 'puff', 'breath'] as const;

/** what a floor is, by its number in Rust */
export const FLOORS = ['carpet', 'lino', 'stone', 'tile', 'asphalt', 'grass'] as const;
