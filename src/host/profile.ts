// THE STAND OF THE QA HARNESS, THE OLD ONE, and the one list of what separates them. Since 2026-11-05 a tool with no word stands SHIPPED (harnessProfile below).
//
//   frozen   (TPS_PROFILE=frozen; the default until 2026-11-05)   the stand every frozen digest, tally, probe bound and session fixture was recorded on:
//                            tools/qa/headless.ts and standGame's harness block hold the newer rules off, each by its own line (`pinned()` below is asked by
//                            every one of them). Kept by its one word for the switch cleanup until the off paths are gone.
//   shipped  (no word, or TPS_PROFILE=shipped)   the stand the PAGE gives a player who opens it with no URL flag on a machine that has kept nothing: none of
//                            those lines runs, so every table is left at its module's own default — which is what the page plays — and `shippedBoot()` writes the few
//                            rows the page's boot (src/main.ts) writes over those defaults. A pin whose own env word IS given still holds (TPS_PROFILE=shipped
//                            TPS_MOVES=0 = as shipped but for the movement pass), and `--set` rows land after either stand. One exception: a PLAY-THROUGH
//                            (tools/qa/playthrough.ts) writes the door rules and the chaos itself after the stand-up, on either stand — there those rows are
//                            changed with `--set`, never with ONEWAY / QA_DOORSTAY / QA_FRAMEHOLDS / QA_DOORMANNERS / QA_DOORBLAST / QA_EXITSOUT.
//
// The page's own boot writes its URL switches through `pageSwitches` below, the same function the shipped stand calls. What a game module reads off the URL as it
// loads, or keeps per machine, cannot be shared that way, so the page is also HELD to the stand: `node tools/qa/shipped-page.cjs` boots the real page, reads every
// knob table off it (window.__knobs) and compares them with the harness stood up under TPS_PROFILE=shipped (about three minutes a page on the software adapter:
// run it when a boot switch or a page-only default is added); `probes.ts shipped` checks, without a page, that the two harness stands differ in exactly the rows
// FROZEN_HOLDS names. A rule that ships on and is pinned off for the frozen checks is added to
// FROZEN_HOLDS in the same change, or that probe is red.
//
// What 'shipped' still is not: the player (the play-through's commander is scripted and cannot be hurt), the GPU's light meter and smoke sight lines (headless: the
// CPU stand-in, no smoke), a machine's own kept choices (localStorage), V8 (bun runs JavaScriptCore), the game's own die (the page's game owns a seeded die,
// src/math/dice.ts; a harness game on either stand draws the tool's Math.random as every recorded tally did, unless TPS_SEED=n / standUp({ seed }) asks).
//
// Host-neutral, and every game import below is used inside a function only (src/game/ is one import cycle: nothing here may run a game import as this module loads).
import { WALLCHARGE } from '../game/wallcharge';
import { SNEAK } from '../game/sneak';
import { AIR_SIM } from '../game/airSim';
import { DOORSHOT } from '../game/doorblast';
import { DOORPATH } from '../game/doorpath';
import { WALLSNAP, LEAN_GUN_M } from '../game/wallsnap';
import { STALK } from '../game/stalk';
import { CMDRING } from '../game/cmdring';
import { SQUADTHROW } from '../game/squadthrow';
import { moraleFromQuery } from '../game/morale';
import { FIRE, FIRE_SIZE, FIRE_SHIMMER, FIRE_PLUME, FIRE_EMBERS, FIRE_BRIGHT, FIRE_REACH, FIRE_LIFT, FIRE_LIFT_WAS, FIRE_WARM, FIRE_WANDER, FIRE_WANDER_UP, FIRE_WANDER_WAS, FIRE_WANDER_UP_WAS, FIRE_REACT, FIRE_LEAN, FIRE_LEAN_WAS, FIRE_STEPS } from '../game/fire';
import { POSEMATH } from '../anim/skeleton';
import { rowsUp, mathsUp, retiredAddressWord, fellWord } from '../sim/sim';
import { netQuietFromQuery } from '../game/netquiet';
import { PICTURE } from '../game/picture';
import { TEAMPACE } from '../game/teampace';
import { MARKSIT } from '../render/decals';
import { PLAN_PACE } from '../render/planPace';
import { REACH_PACE } from '../render/reach';
import { COATFINISH, SOLIDGLASS } from '../scene/stuff';
import { WALLSKIN } from '../game/wallskin';
import { gearFromQuery } from './gear';
import { wordsFromQuery } from '../game/words';
import { kitLoad, KIT_SOLO, BIGSLOT } from '../game/kit';
import { squadSenses } from '../game/enemysquads';
import { LOOK, suitOfQuery } from '../scene/look';
import { LIVED, livedOfQuery } from '../scene/lived';
import { SHAPES } from '../scene/shapes';
import { ROOMCLEAR } from '../game/roomclear';
import { FIXEDSTEP } from './loop';
import { TWEEN } from './tween';
import { EVENPACE, FLIGHT_PAGE } from './pace';
import { findMeStarts } from './findme';
import { settledUp } from './settled';

export type Profile = Stand;
const envOf = (): Record<string, string | undefined> => (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env ?? {};

/** the stand this harness run asked for (TPS_PROFILE; the rule is standEnv.ts standOf: no word or 'shipped' = the page's stand, 'frozen' = the old one, by
 *  that one word, anything else stops the tool). tools/qa/stand.ts writes the word down for a tool as it loads and says a run on the old stand on stderr. */
export function harnessProfile(): Profile { return standOf(envOf().TPS_PROFILE); }
/** does a harness pin hold on this run? frozen: always. shipped: only where the run's environment names the pin's own word (`env`; a pin with none is the frozen stand's alone) */
export function pinned(env?: string): boolean { return harnessProfile() === 'frozen' || (env !== undefined && envOf()[env] !== undefined); }

/** THE SWITCHES THE PAGE'S BOOT WRITES FROM ITS URL, in one place: src/main.ts calls this with the page's query before the level is built, the harness's shipped
 *  stand calls it with an empty one (shippedBoot below) — so 'the page with no flag' is this function's answer to '', by construction.
 *  (Switches a game module reads off the URL itself as it loads are shared already: under bun there is no `location`, and the module's own default stands.) */
/** THE RUST MODULE'S ROWS AND THE GAME'S OWN MATHS ON THE PAGE (src/sim/sim.ts): every row of RUST_ON and the maths, always. The address has no word
 *  for either any more (?rust, ?rust<row>, ?noexact were the switches while the TypeScript was a second way to play): one that still says so is told on
 *  the console, once a page and a word, as a note and not a warning (a visitor's old bookmark is no fault of the page's), and is not obeyed. */
export const RETIRED_SAID = new Set<string>();
export function pageRows(q: URLSearchParams): void {
  mathsUp(); rowsUp();
  for (const k of q.keys()) if (retiredAddressWord(k) && !(k === 'rust' && q.get('rust') === '1') && !RETIRED_SAID.has(k)) { RETIRED_SAID.add(k); console.info(`?${k}: the Rust module's rows and the game's own maths are not switches any more; the word does nothing`); }
  { const f = q.get('fell'); if (f !== null) fellWord(f); }   // (?fell: the count of questions the TypeScript answered while its row was up, from the first box asked as the level is built — sim.ts FELL; `window.__sim.fell()` reads it)
}
export function pageSwitches(q: URLSearchParams): void {
  settledUp();   // every settled row at its one value (host/settled.ts): below are the rows that are still switches, each by its address word
  SHAPES.on = q.get('shapes') !== '0';   // a box says its shape (scene/shapes.ts): a level's boxes that say one — a vehicle's rounded corners, crowned bonnet, bowed roof, leaning glass and round wheels — stand as it to the picture and to every ray, and their pieces of trim (hubs, arches, window posts: drawn only) are added; what walks meets the plain level: ON in the page; ?shapes=0 = the plain boxes, as they were (the game's own rays meet a shape whatever the cut-pieces switch says: boxes.ts cutMet)
  COATFINISH.on = q.get('coatfinish') !== '0';   // a named coat's finish goes wherever its word goes (scene/stuff.ts COATFINISH; a picture only): a box that says car paint is as smooth as a clear coat, so a vehicle shows what stands round it — ON in the page; ?coatfinish=0 = the finish its file gives it, as it was
  SOLIDGLASS.on = q.get('solidglass') !== '0';   // a thing of solid glass is dark and smooth (scene/stuff.ts SOLIDGLASS; a picture only): a box that says glass and is no pane — a vehicle's window, a mirror's face — is sent near black and polished: ON in the page; ?solidglass=0 = the grey its file gives it, as it was
  { const n = Number(q.get('seenrate')); if (q.has('seenrate') && Number.isFinite(n) && n > 0) SNEAK.seenRate = Math.min(1, n); }   // a lit step with a line is SEEN only where that defender's eye would gain on a man there (src/game/sneak.ts; `?seenrate=<n>`: the rate that counts, the guards' own floor 0.05 by default)
  AIR_SIM.on = q.get('air') !== '0';   // THE AIR AS ONE SIMULATED FIELD (game/airSim.ts, render/airField.ts): the wind as a small fluid simulation over every level's floor plan, fed by the wind's own sources, walls and door leaves standing in it — a picture only (today the blown snow rides it): ON in the page; ?air=0 = no field is built or stepped, the build as it was
  if (q.get('doorcost') === '0') { DOORSHOT.toll = 1; }   // shooting a door out is a cost — twelve rifle rounds, twenty-four pistol, whatever the destruction scale (DOORSHOT.toll 2 and .flat, the module's own words): ?doorcost=0 = drywall's own six and twelve, riding the scale, as it was
  if (q.get('wallmenu') === 'direct') WALLCHARGE.menu = 'direct';   // the wall charge's own switch (game/wallcharge.ts; it also needs the destruction's) · ?wallmenu=direct: X, 5 instead of X, 5, 3
  WALLSNAP.on = !q.has('nowallsnap');   // back to the wall, the feel pass (game/wallsnap.ts): the take on one eased clock with stepping feet and a settle, the way off keeping his momentum, and Q with the rifle in hand STACKS on the wall (low ready; the cursor round an open end steps him out): ON in the page; ?nowallsnap = the hold as it was
  WALLSNAP.press = q.get('wallpress') !== '0';   // …taken without the key (game/wallsnap.ts `press`, player.ts wallPress): a move key held into a wall takes it as its key does (while this is on the hold has no key on the page's board: host/keymap.ts qFree) — ON in the page (Options ▸ Game has its row, kept per machine); ?wallpress=0 = Q alone, as it was
  WALLSNAP.leanGun = q.has('noleangun') ? 0 : LEAN_GUN_M;   // …and the step-out round an end goes as far as the RIFLE needs (wallsnap.ts `leanGun`): round an end on his gun side the eyes' 0.6 m left the rifle in the jamb and its rounds in the wood; ?noleangun = 0.6 m whichever side, as it was
  TEAMPACE.doorBrisk = q.get('doorbrisk') !== '0';   // nobody sprints to a stack: a teammate on a stack order who would run to cover on his way to the door goes at the fight's pace with his gun up (game/teampace.ts teamDash). ?doorbrisk=0 = he sprints there with his gun low, as he did
  TEAMPACE.pacing = q.get('pacing') !== '0';   // a teammate paces himself on you: he walks to an order's mark at your walk, slows only while you are behind him and walking after him, and steps up to a brisk walk when you are ahead and bound the same way (game/teampace.ts teamWalkOf). ?pacing=0 = the flat four fifths of your walk, as it was
  findMeStarts();   // the outline in the dark is ON for a machine that has kept nothing, at the maintainer's own numbers for its width, his teammates' and its colour, and the camera starts as he plays it: the aim pan at none and the turn the other way (host/findme.ts FINDME_TUNED; a row a hand has moved stays where the hand put it) — ON in the page; ?findtuned=0 = those rows start as built: no rim until Options > Game asks, the view leaning toward the aim point and turning as it did
  MARKSIT.on = q.get('marksit') !== '0';   // a blast's marks sit in the surface (render/decals.ts MARKSIT; a picture only, one word of the decal buffer's header): ON in the page; ?marksit=0 = every mark as it was drawn
  PLAN_PACE.on = q.get('planpace') !== 'old';   // the picture's planners are made a little a frame (render/planPace.ts; a picture only: engine.ts reads it as it draws, and nothing in the update does): ON in the page; ?planpace=old = the wall sides, the openings and the statics' draw order each made whole on the frame that asks, as they were
  REACH_PACE.whole = q.get('reachpace') === 'old';   // a reach mask decided whole after a wall changes, the pace before the per-frame shares (render/reach.ts REACH_PACE; the lamps' masks are the picture's and are kept by it on drawn frames, render/reachKeep.ts: nothing in the tick reads the word): OFF in the page and on both stands; ?reachpace=old = on. Read here and not by main.ts alone, so a recipe carries it and a server's stand paces as its clients do
  WALLSKIN.on = q.get('wallskin') !== '0';   // what is painted on a wall goes with the wall (game/wallskin.ts): a band, a skirt or a dado on a wall's face is drawn only where the wall still stands behind it, cut at the edge of a hole as the wall is — picture only, no box's flags written, the same run by digest: ON in the page; ?wallskin=0 = the band stays whole across the hole, as it was
  EVENPACE.on = q.get('pace') !== '0';   // the page draws at the rate the quality ladder holds, so a fast display's frames between are passed over and the pace is even (host/pace.ts): ON in the page; ?pace=0 = every display frame worked, as it was
  EVENPACE.giveway = q.get('giveway') !== '0';   // a window without the system's focus keeps one frame with the GPU at a time and leaves it as long again, so a second window of the browser can start and play beside it (host/pace.ts): ON in the page; ?giveway=0 = an unfocused window draws as a focused one, as it was
  EVENPACE.flight = q.get('flight') === '0' ? 0 : q.get('flight') === '1' ? 1 : FLIGHT_PAGE;   // the page waits for the GPU: a frame is not begun while two are still with it, so frames reach the GPU as it finishes them and not in bursts (host/pace.ts): ON in the page at two; ?flight=1 = one at a time; ?flight=0 = nothing waits, as it was
  moraleFromQuery(q);   // shaken (game/morale.ts): a blast makes the men near it duck, and breaks a man alone in the dark — down, back off the hole, a burst into the dust, a call nobody answers: ON in the page; ?nomorale = every man as steady as he was. With it who is green and how brittle they all are: ?training=green|mixed|trained (absent: by the squad's own key and the difficulty), ?nerve=brittle|steady — and a choice made by hand in the debugger's Enemy nerve section, kept for the visit
  FIRE.size = q.get('firesize') === '0' ? 0 : FIRE_SIZE;   // … its lamp the pinpoint the level made it, not the flame's size: the drum's shadow on the ground with a drawn edge, as it was (to compare on a GPU)
  FIRE.look.shimmer = q.get('fireheat') === '0' ? 0 : FIRE_SHIMMER; FIRE.look.plume = q.get('fireplume') === '0' ? 0 : FIRE_PLUME;   // … the shimmer of the hot air over it, and the smoke drawn off its tips (both the picture only): ?fireheat=0, ?fireplume=0 = without
  FIRE.embers = q.get('fireembers') === '0' ? 0 : FIRE_EMBERS;   // … and its embers, which are GPU particles: ?fireembers=0 = none
  { const plain = q.get('firebright') === '0'; FIRE.bright = plain ? 1 : FIRE_BRIGHT; FIRE.reach = plain ? 1 : FIRE_REACH; FIRE.lift = plain ? FIRE_LIFT_WAS : FIRE_LIFT; FIRE.warm = plain ? [1, 1, 1] : [...FIRE_WARM]; }   // … its lamp a fire's: a quarter stronger than the level's, reaching further, a deeper orange, in the body of the flame (the light meter reads it): ?firebright=0 = the lamp as the level hung it
  { const still = q.get('firesway') === '0'; FIRE.wander = still ? FIRE_WANDER_WAS : FIRE_WANDER; FIRE.wanderUp = still ? FIRE_WANDER_UP_WAS : FIRE_WANDER_UP; }   // … its place shifting with the flames far enough to move a shadow's edge: ?firesway=0 = the few centimetres it was
  { const n = Math.round(+(q.get('firesteps') ?? 0)); FIRE.look.steps = n >= 4 && n <= 64 ? n : FIRE_STEPS; }   // … how many samples of the flame a pixel's ray takes (the picture only, its price on a GPU): ?firesteps=N, 4 to 64, for a timing; 40 as it opens
  FIRE.look.react = q.get('firereact') === '0' ? 0 : FIRE_REACT; FIRE.look.lean = q.get('firelean') === '0' ? FIRE_LEAN_WAS : FIRE_LEAN;   // … its pixels taken from this frame by the temporal resolve (?firereact=0 = averaged over the picture's memory, as it was), and the wind leaning the flame (?firelean=0 = hardly, as it was)
  POSEMATH.singles = q.get('posesingles') !== '0';   // a pose's bulk arithmetic in singles (anim/skeleton.ts): blendPose and sampleClip's step worked in singles, a rotation's length a plain root, in both languages — what lets the module work four nodes a step. ON in the page; ?posesingles=0 = in doubles, as it was
  pageRows(q);   // which systems run in the Rust module (src/sim/sim.ts RUST): the rows of RUST_ON are ON in the page — pageRows below has the words
  netQuietFromQuery(q);   // the net goes quiet (game/netquiet.ts): a radio check nobody answers and a body found shake the men, the asker's calls get worse, a jumpy man's beam whips to a noise, and the last man backs to the spot his doors sit in one arc from and goes low: ON in the page; ?nonetquiet = the men as they were
  if (q.has('picture')) PICTURE.show = q.get('picture') === 'all' ? 'all' : 'cursor';   // (?picture = the floor shows the picture of the man under the cursor; ?picture=all = every teammate's ideas)
  CMDRING.goalHalf = q.get('goalhalf') !== '0';   // …and the first of them, with nothing going, sends half the squad (the pair nearer what the cursor is on) and leaves the rest with him (`?goalhalf=0` = with nothing going the goal is everyone's, as it was)
  CMDRING.twice = q.get('qtwice') !== '0';   // …and the key tapped twice on the same thing is 'fall in' for everyone, every goal gone with it; the first tap still gives its order as it comes up (`?qtwice=0` = the second tap is the key's order again, as it was)
  SQUADTHROW.on = !q.has('nosquadthrow');   // teammates throw a frag or a stun canister where he points (game/squadthrow.ts): X on a spot of floor, G, then 1 or 2. ON in the page, ?nosquadthrow = no such row; off on the frozen stand (TPS_SQUADTHROW=1)
  BIGSLOT.on = !q.has('nobigslot');   // the big slot (game/kit.ts BIGSLOT, game/bigkit.ts): every card has ONE big slot beside its pouches — the wall charge, the breaching shotgun (kit now, not a gun: the man keeps his rifle) or the dynamic hammer — and the last two take a lock from either face, yours on a locked door's row, a teammate's on a door order. ON in the page, ?nobigslot = the cards and the weapon row as they were; off on the frozen stand (TPS_BIGSLOT=1). Before the kit is read from the store: a stored shotgun-as-gun card is read as rifle + shotgun in the big slot.
  gearFromQuery(q);   // the 2010s issue kit (host/gear.ts): Sam's rifle and pistol named a Mk 18 and a Glock 19 (the rifle drawn as an M4-pattern carbine), two-tube goggles that do not glow, the ring worded and drawn a 40 mm sponge round — ON in the page; ?oldgear = the Splinter Cell kit, as it was. Words and boxes, and one number: the Glock's magazines hold fifteen
  wordsFromQuery(q);   // the dialogue pass (game/words.ts): hostages with a voice of their own, and every line said by the side and in the place it was written for — ON in the page; ?oldwords = every line as it was. A bubble's time is its length, so a reworded line can move who talks next: the frozen stand keeps the old words
  LOOK.on = true;   // the default look, 2010s GWOT (scene/look.ts): the men's dress (the cast file and its material tables; weapons and kit are GEAR's) and a leaf's paint
  suitOfQuery(q);   // …and under it the black sneaking suit Sam wears on a mission that is not a squad's loud one (game/cast.ts sneakSuit): ?suit=0 = the operators' issue dress on every mission
  LIVED.on = livedOfQuery(q);   // lived-in rooms (scene/lived.ts): the furniture, clutter, floor pieces and lamps that make each room of a level its own place, as the office's rooms are — ON in the page, raised before the level loads; ?lived=0 = every level as it was before there were any
  { const w = q.get('way'); ROOMCLEAR.way = w === 'any' ? -1 : w !== null && /^\d+$/.test(w) ? +w : 0; }   // …and the way in (ROOMCLEAR.way; the mission sheet's 'Way in' row writes it on the link): absent or ?way=1 = the level's own start, ?way=2 … = its other ones, ?way=any = dealt off this visit's die with the marks
  FIXEDSTEP.on = !/^(0|off)$/.test(q.get('fixedstep') ?? '');   // the game runs in whole 1/60 s ticks whatever the display's rate (host/loop.ts FIXEDSTEP — the same game on a 60, a 120 and a 144 Hz display): ON in the page; ?fixedstep=0 = one update a frame on the frame's own time, as before (?fixedstep=<hz> = another tick rate: main.ts)
  TWEEN.on = !/^(0|off)$/.test(q.get('tween') ?? '');   // …and a frame between two ticks is drawn at the blend of them (host/tween.ts: keyed boxes, the men, carried lights, the camera's look-at — a part of a tick behind the game, none on a 60 Hz display; acts only with the line above): ON in the page; ?tween=0 = every frame draws the last tick, as the fixed step did
}

/** the harness stood up as the page with no URL flag on a machine that has kept nothing: the page's own boot switches for an empty query, the kit read from an empty
 *  store (the page: ui/kitPreview.ts kitBoot → kitLoad(localStorage, search)), the solo night's Sam in his own card (the page on ?solo), and the one default a game module keeps for the page alone (the defender squads'
 *  senses). Called once, as tools/qa/headless.ts loads and before any level is built, under the shipped profile only. */
export function shippedBoot(): void {
  pageSwitches(new URLSearchParams(''));
  kitLoad(null, '');
  KIT_SOLO.stand = true;   // (kit.ts kitSoloStand: a game stood up plain, or with a quiet mission alone started on it, wears your card as the page's ?solo / quiet-alone Sam does — one of each under the pouches rule, not the eleven things a Player is built with. A flag of the stand, in no knob table)
  if (envOf().SQUADS === undefined) squadSenses('call');   // (enemysquads.ts: the page opens as ?radiocheck=call; a run that names its own SQUADS word keeps what that module made of it as it loaded)
}

/** WHAT THE FROZEN STAND HOLDS OFF: every row the two harness stands differ in, as `Table.key` (or `tune.key` for Game.tune), with the word that turns it on for a
 *  frozen run where there is one. `probes.ts shipped` stands the game up both ways and is red when this list and the measured difference are not the same list. */
export const FROZEN_HOLDS: Record<string, string> = {
  // standGame's harness block (src/host/standup.ts)
  'OPENING.on': 'TPS_OPENING=1',
  'tune.qrfSilent': 'TPS_QRFSILENT=1',
  'tune.qrfSquad': 'TPS_QRFSQUAD=1',
  'tune.chaos': "the tool's own (playthrough.ts plays 0.5, soak.ts --chaos K)",
  'AIM.player.lightSnapPx': 'none: probes.ts aim-snap sets it',
  'AIM.player.level': 'none: probes.ts aimlevel sets it',
  'AIM.player.low': 'none: probes.ts aimlow sets it',
  'MOVES.on': 'TPS_MOVES=1',
  'TURN.on': 'TPS_TURN=1',
  'DOORPATH.on': 'TPS_DOORPATH=1',
  'FOG.things': 'TPS_FOGTHINGS=1',
  'FOG.flat': 'TPS_FOGFLAT=1',
  'FOG.lit': 'TPS_FOGLIT=1',
  'FOG.full': 'TPS_FOGFULL=1',
  'FOG.air': 'TPS_FOGAIR=1',
  'FOG.ours': 'TPS_FOGOURS=1',
  'FOG.thin': 'TPS_FOGTHIN=1',
  'FOG.eyes': 'TPS_FOGEYES=1',
  'FOG.kept': 'TPS_FOGKEPT=1',
  'FOG.filed': 'TPS_FOGFILED=1',
  'FOG.tracks': 'TPS_FOGTRACKS=1',
  'FOG.mates': 'TPS_FOGMATES=1 (acts only under FOG.thin: with TPS_FOGTHIN=1)',
  'FOG.edge': 'TPS_FOGEDGE=1',
  'FOG.desat': 'TPS_FOGCOLOUR=1',
  'FOG.squadDoors': 'TPS_SQUADDOORS=1',
  'FOG.squadFire': 'TPS_SQUADFIRE=1',
  'SILENT.on': 'none: probes.ts silent sets it',
  'tune.glassBreak': 'GLASSFX=on',
  'tune.glassShards': 'GLASSFX=on',
  'GLASSPEN.on': 'GLASSFX=on',
  'FRAG.defenders': `--set '{"FRAG":{"defenders":true}}'`,
  'FRAG.fleeWise': 'TPS_FLEEWISE=1',
  'FRAG.restProps': 'TPS_FLEEMATES=1',
  'FRAG.fleeApart': 'TPS_FLEEMATES=1',
  'FRAG.fleeSam': 'TPS_FLEEMATES=1',
  'FRAG.fleeLook': 'TPS_FLEEMATES=1',
  'FRAG.wallFlee': 'TPS_WALLFLEE=1',
  'FRAG.downKill': 'TPS_DOWNKILL=1',
  'FRAG.fleeDoor': 'TPS_FLEEDOOR=1',
  'DOORSTAY.on': 'QA_DOORSTAY=1 (playthrough.ts plays it on)',
  'LIGHTWALL.on': 'TPS_LIGHTWALL=1',
  'IR.teamOff': 'TPS_NOTEAMLASER=1 (a picture only)',
  'LIGHTS.wallOff': 'TPS_WALLLIGHT=1',
  'IRLIGHT.on': 'TPS_IRLIGHT=1',
  'IRLIGHT.auto': 'TPS_IRAUTO=1',
  'IRLIGHT.meter': 'TPS_IRMETER=1',
  'IRLIGHT.near': 'TPS_IRNEAR=1',
  'AIRDUST.on': 'TPS_AIRDUST=1',
  'WALLSNAP.on': 'TPS_WALLSNAP=1',
  'WALLSNAP.press': 'TPS_WALLPRESS=1',
  'WALLSNAP.pressRun': 'TPS_WALLRUN=1',
  'WALLSNAP.voice': 'TPS_WALLVOICE=1',
  'WALLSNAP.slow': 'TPS_WALLSLOW=1',
  'WALLSNAP.leanGun': 'TPS_LEANGUN=1',
  'HANDSLOT.on': 'TPS_HANDSLOT=1',
  'HANDRING.on': 'TPS_HANDRING=1',
  'HANDRING.empty': 'TPS_HANDRING=1',
  'HANDRING.near': 'TPS_HANDRING=1',
  'HANDRING.lone': 'TPS_HANDRING=1',
  'HANDRING.loneWrap': 'TPS_HANDRING=1',
  'HANDRING.bare': 'TPS_HANDRING=1',
  'HANDRING.turn': 'TPS_HANDRING=1',
  'HANDRING.corner': 'TPS_HANDRING=1',
  'THROWBACK.on': 'TPS_THROWBACK=1',
  'OWNBLOW.on': 'TPS_OWNBLOW=1',
  'THROWBACK.restMark': 'none: settled, the Rust view asks it no more',
  'THROWBACK.reach': 'none: the Rust view reads it',
  'GUNWALL.on': 'TPS_GUNWALL=1',
  'GUNWALL.mark': 'none: settled, the Rust view asks it no more',
  'REARWATCH.on': 'TPS_REARWATCH=1',
  'STACKLINE.on': 'TPS_STACKLINE=1',
  'LANES.on': 'TPS_LANES=1',
  'HARD.sweep': 'TPS_SWEEP=1',
  'HAULHOME.on': 'TPS_HAULHOME=1',
  'GUNHOLD.on': 'TPS_GUNHOLD=1',
  'HOLD.dropGun': 'TPS_HOLDDROP=1',
  'HOLD.disarm': 'TPS_DISARM=1',
  'HOLD.stripFirst': 'TPS_STRIPFIRST=1 (acts only under HOLD.disarm: with TPS_HOLDDROP=1 TPS_DISARM=1)',
  'HOLD.oneFacing': 'TPS_HOLDYAW=1',
  'BACKKEY.tab': 'TPS_TABBACK=1',
  'WALLSHOT.on': 'TPS_WALLSHOT=1',
  'WALLKILL.on': 'TPS_WALLKILL=1',
  'HARDBREATH.on': 'TPS_HARDBREATH=1',
  'FIXEDSTEP.on': 'TPS_FIXEDSTEP=1 (read only by a tool that steps through host/loop.ts stepFrame: the harness\'s own W.step names its dt)',
  'TWEEN.on': 'TPS_TWEEN=1 (the picture between two ticks: the same)',
  'REARWATCH.room': 'TPS_ROOMKEEP=1',
  'CLOSE.on': 'TPS_CLOSE=1',
  'CLOSE.heldClicks': 'TPS_HELDCLICKS=1',
  'CLOSE.reachUp': 'TPS_REACHUP=1',
  'CLOSE.noKnife': 'TPS_NOKNIFE=1',
  'CLOSE.rmbHold': 'TPS_RMBHOLD=1',
  'FOOTFALL.once': 'TPS_FOOTFALL=1',
  'FOOTFALL.owed': 'TPS_FOOTOWED=1 (acts only under FOOTFALL.once: with TPS_FOOTFALL=1)',
  'TWOGEARS.on': 'TPS_TWOGEARS=1',
  'CROUCHRUN.on': 'TPS_CROUCHRUN=1',
  'RUNREADY.on': 'TPS_RUNREADY=1',
  'RUNARMS.on': 'TPS_RUNARMS=1',
  'SAMLOOK.on': 'TPS_SAMLOOK=1',
  'SAMLOOK.slip': 'TPS_SAMSLIP=1',
  'STALK.on': 'TPS_STALK=1',
  'STALK.key': 'TPS_STALKKEY=1 (acts only under STALK.on: with TPS_STALK=1)',
  'STALK.low': 'TPS_STALKLOW=1 (acts only under STALK.on: with TPS_STALK=1)',
  'STALK.wide': 'TPS_STALKWIDE=1 (acts only under STALK.low: with TPS_STALK=1 TPS_STALKLOW=1)',
  'STALK.orbit': 'TPS_STALKORBIT=1 (acts only under STALK.on: with TPS_STALK=1)',
  'STALK.heft': 'TPS_STALKHEFT=1 (acts only under STALK.wide: with TPS_STALK=1 and its row)',
  'STALK.turnIn': 'TPS_STALKPOLISH=1 (acts only under STALK.on: with TPS_STALK=1)',
  'STALK.cornerShow': 'TPS_STALKPOLISH=1 (acts only under STALK.on: with TPS_STALK=1)',
  'STALK.catchLong': 'TPS_STALKPOLISH=1 (acts only under STALK.on: with TPS_STALK=1)',
  'STALK.jolt': 'TPS_STALKPOLISH=1 (acts only under STALK.on: with TPS_STALK=1)',
  'STALK.yankWith': 'TPS_STALKPOLISH=1 (acts only under STALK.on: with TPS_STALK=1)',
  'HOLD.wayShape': 'TPS_STALKPOLISH=1 (acts only with bare hands, wherever the reach shows: with TPS_NOKNIFE=1)',
  'HOLD.catchShape': 'TPS_STALKPOLISH=1 (acts only in a bare-handed hold: with TPS_NOKNIFE=1)',
  'HOLD.hitShape': 'TPS_STALKPOLISH=1 (acts only in the grab a stalk ends in, bare-handed: with TPS_STALK=1 TPS_NOKNIFE=1)',
  'HOLD.layAhead': 'TPS_STALKPOLISH=1 (acts only at a choke\'s end, in the takedown\'s own fall: with TPS_TAKEDOWN=1)',
  'HOLD.lunge.dip': 'TPS_STALKPOLISH=1 (acts only under STALK.on: with TPS_STALK=1)',
  'HOLD.lunge.back': 'TPS_STALKPOLISH=1 (acts only under STALK.on: with TPS_STALK=1)',
  'HOLD.sway.m': 'TPS_STALKPOLISH=1 (acts only in a bare-handed hold: with TPS_NOKNIFE=1)',
  'HOLD.sway.twist': 'TPS_STALKPOLISH=1 (acts only in a bare-handed hold: with TPS_NOKNIFE=1)',
  'HOLD.apart.side': 'TPS_STALKPOLISH=1 (acts only in a bare-handed hold: with TPS_NOKNIFE=1)',
  'HOLD.apart.tip': 'TPS_STALKPOLISH=1 (acts only in a bare-handed hold: with TPS_NOKNIFE=1)',
  'HOLD.yank.arch': 'TPS_STALKPOLISH=1 (acts only with bare hands on a stalk: with TPS_STALK=1 and TPS_NOKNIFE=1)',
  'GUARD_EYE.far': 'TPS_FARSIGHT=1',
  'KITANIM.on': 'TPS_KITANIM=1',
  'THROW_BY.on': 'TPS_THROWBY=1',
  'START.fallIn': 'TPS_FALLIN=1',
  'SAMESTART.on': 'TPS_SAMESTART=1',
  'TAKE.wired': 'TPS_TAKECHECK=1',
  'QUIETFOG.on': 'TPS_QUIETFOG=1',
  'QUIETFOG.def': 'TPS_QUIETDEF=team',
  'QUIETFOG.wand': 'TPS_QUIETWAND=1',
  'QUIETFOG.solo': 'TPS_SOLOFOG=1',
  'TOLD.on': 'TPS_TOLD=1',
  'TOLD.fogOff': 'TPS_FOGOFF=1',
  'TORCH.calmOff': 'TPS_CALMTORCH=1',
  'TORCH.wallLow': 'TPS_TORCHWALL=1',
  'MORALE.on': 'TPS_MORALE=1',
  'RUNNER.on': 'TPS_RUNNER=1',
  'RUNNER.anyway': 'TPS_RUNANYWAY=1',
  'RUNNER.nobody': 'TPS_RUNNOBODY=1',
  'ROWSET.on': 'TPS_ROWSET=1',
  'SMOKEBREAK.on': 'TPS_SMOKEBREAK=1',
  'SHOWWHY.on': 'TPS_SHOWWHY=1',
  'FIRE.on': 'TPS_FIRE=1',
  'FLINCH.on': 'TPS_FLINCH=1',
  'FLINCH.run': 'TPS_FLINCHRUN=1',
  'VERDICT.loudCard': 'TPS_LOUDCARD=1',
  'DARKDOOR.on': 'TPS_DARKDOOR=1',
  'DARKFOE.on': 'TPS_DARKFOE=1',
  'POWEROUT.on': 'TPS_POWEROUT=1',
  'SQUADDEAL.on': 'TPS_SQUADDEAL=1',
  'STAYASK.on': 'TPS_STAYASK=1',
  'KITLOAD.on': 'TPS_KITLOAD=1',
  'RESCUECHECK.on': 'TPS_RESCUECHECK=1',
  'RESCUEEND.on': 'TPS_RESCUEEND=1',
  'RESCUEFLUSH.on': 'TPS_RESCUEFLUSH=1',
  'POSEMATH.singles': 'TPS_POSESINGLES=1',
  'CMDCHECK.on': 'TPS_CMDCHECK=1',
  'DOZE.on': 'TPS_DOZE=1',
  'NETQUIET.on': 'TPS_NETQUIET=1',
  'ATTENTION.on': 'TPS_ATTENTION=1',
  'PICTURE.on': 'TPS_PICTURE=1',
  'RESPONSE.on': 'TPS_RESPONSE=1',
  'SAMSTUN.on': 'TPS_SAMSTUN=1',
  'TEAMTALLY.on': 'TPS_TEAMTALLY=1',
  'AIMWEIGHT.on': 'TPS_AIMWEIGHT=1',
  'SPRINTDOOR.on': 'TPS_SPRINTDOOR=1',
  'TEAMPACE.on': 'TPS_TEAMPACE=1',
  'TEAMPACE.doorBrisk': 'TPS_DOORBRISK=1',
  'TEAMPACE.pacing': 'TPS_PACING=1',
  'NOSHOVE.on': 'TPS_NOSHOVE=1',
  'RAILS.on': 'TPS_RAILS=1',
  'RAILS.cardBeat': 'TPS_CARDBEAT=1 (acts only with the frame up: TPS_RAILS=1)',
  'RAILS.flatRing': 'TPS_RAILFLAT=1 (acts only with the frame and the flat tree up: TPS_RAILS=1 TPS_QRING=1 TPS_QONLY=1 TPS_RINGFLAT=1)',
  'RAILS.spareDown': 'TPS_SPAREDOWN=1 (acts only with the frame up: TPS_RAILS=1)',
  'BELIEF.on': 'TPS_BELIEF=1',
  'DOORFORCE.postEnds': 'TPS_POSTENDS=1',
  'DOORFORCE.on': 'TPS_DOORFORCE=1',
  'SLOTSETUP.on': 'TPS_SLOTSETUP=1',
  'SLOTSETUP.pop': 'TPS_SLOTSETUP=1',
  'SLOTSETUP.draw': 'TPS_SETUPDRAW=1 (acts only with the rule up: TPS_SLOTSETUP=1)',
  'SLOTSETUP.mags': 'none: nothing reads it',
  'GLASSREAL.drop': 'TPS_GLASSDROP=1 (acts only with the glass up: GLASSFX=on)',
  'GLASSREAL.place': 'TPS_GLASSPLACE=1 (acts only with the glass up: GLASSFX=on)',
  'AIMCOST.lamp': 'TPS_LAMPHIT=1',
  'AIMCOST.move': 'TPS_MOVECOST=1',
  'QUICKTHROW.on': 'TPS_QUICKTHROW=0',   // (the one row the FROZEN stand has on and the page does not: the word takes it away)
  'FRIENDS.noSnap': 'TPS_FRIENDS=1',
  'FRIENDS.lowReady': 'TPS_MUZZLE=1',
  'FRIENDLYFIRE.on': 'TPS_FRIENDLYFIRE=1',
  'LIGHTSWITCH.on': 'TPS_LIGHTSWITCH=1',
  'CARLAMPS.on': 'TPS_CARLAMPS=1',
  'HITSTUFF.on': 'TPS_HITSTUFF=1',
  'MARKCUT.on': 'TPS_MARKCUT=1',
  'MARKSIT.on': 'TPS_MARKSIT=1 (a picture only)',
  'PLAN_PACE.on': 'TPS_PLANPACE=1 (a picture only)',
  'WALLSKIN.on': 'TPS_WALLSKIN=1',
  'CLEANRESTART.on': 'TPS_CLEANRESTART=1',
  'FIRELINE.on': 'TPS_FIRELINE=1',
  'SAVETIMELINE.on': 'TPS_SAVETIMELINE=1',
  'SAVETIMELINE.holds': 'TPS_SAVEHOLDS=1',
  'SAVETIMELINE.turn': 'TPS_SAVETURN=1',
  'SAVETIMELINE.since': 'TPS_SAVESINCE=1',
  'SAVETIMELINE.base': 'TPS_SAVEBASE=1',
  'SAVEPIC.on': 'TPS_SAVEPIC=1 (the page\'s alone: no picture is taken headless)',
  'EVENPACE.on': 'TPS_EVENPACE=1 (the page\'s alone: no display asks a harness for a frame)',
  'EVENPACE.giveway': 'TPS_GIVEWAY=1 (the page\'s alone: a harness has no window and nothing to give way to)',
  'EVENPACE.flight': 'TPS_FLIGHT=1 (= the page\'s number, pace.ts FLIGHT_PAGE; the page\'s alone: a harness hands no frame to a GPU)',
  'FINDTUNED.on': 'TPS_FINDTUNED=1 (the page\'s own starts for the outline and the camera: nothing headless draws the rim or hands a camera over for the pan; a scripted turn under the camera\'s key goes the other way)',
  'WALLCHARGE.outer': 'TPS_OUTERWALL=1',
  'WALLCHARGE.safe': 'TPS_WALLSAFE=1',
  'WALLCHARGE.farFurn': 'TPS_WALLFURN=1',
  'WALLCHARGE.hung': 'TPS_HUNGCHARGE=1',
  'WALLCHARGE.outOfFire': 'TPS_STANDFIRE=1',
  'WALLCHARGE.menClear': 'TPS_MENCLEAR=1',
  'CHAOS.pall.on': 'TPS_PALL=1',
  'CHAOS.mess.on': 'TPS_MESS=1',
  'WALLREACT.everyGone': 'TPS_EVERYGONE=1',
  'WALLREACT.blastFlood': 'TPS_BLASTFLOOD=1',
  'CASTOR.on': 'TPS_CHAIRROLL=1',
  'ORDERS.fileSpread': 'TPS_FILESPREAD=1',
  'ORDERS.asideClear': 'TPS_ASIDECLEAR=1',
  'ORDERS.fileQueue': 'TPS_FILEQUEUE=1',
  'ORDERS.propRound': 'TPS_PROPROUND=1',
  'DOORMANNERS.leafStall': 'TPS_LEAFSTALL=1',
  'ORDERS.fileHold': 'TPS_FILEHOLD=1',
  'ORDERS.backEnd': 'TPS_BACKEND=1',
  'ORDERS.inWay': 'TPS_INWAY=1',
  'ORDERS.retakeGoal': 'TPS_RETAKEGOAL=1',
  'ORDERS.walkTries': 'TPS_WALKTRIES=1',
  'ORDERS.roundCap': 'TPS_ROUNDCAP=1',
  'ORDERS.sameWay': 'TPS_SAMEWAY=1',
  'ORDERS.trailDoors': 'TPS_TRAILDOORS=1',
  'ROOMFIGHT.doorShots': 'TPS_DOORSHOTS=1',
  'COVER.rearRead': 'TPS_REARREAD=1',
  'COVER.backTurn': 'TPS_BACKTURN=1',
  'UNDERFOOT.on': 'TPS_UNDERFOOT=1',
  'SHAPES.on': 'TPS_SHAPES=1',
  'COATFINISH.on': 'TPS_COATFINISH=1 (a picture only)',
  'SOLIDGLASS.on': 'TPS_SOLIDGLASS=1 (a picture only)',
  'SNEAK.noiseWalk': 'TPS_NOISEWALK=1',
  'DOORTOOLS.testDoor': 'TPS_DOORTEST=1',
  'DOORTOOLS.thrPie': 'TPS_THRPIE=1',
  'DOORTOOLS.thrLean': 'TPS_THRLEAN=1',
  'DOORTOOLS.thrPieOne': 'TPS_THRPIEONE=1',
  'DOORTOOLS.outsidePlain': 'TPS_OUTSIDEPLAIN=1',
  'DOORTOOLS.wordNoCan': 'TPS_WORDNOCAN=1',
  'DOORTOOLS.wordCuts': 'TPS_WORDCUTS=1',
  'DOORTOOLS.wordSmall': 'TPS_WORDSMALL=1',
  'CMDRING.on': 'TPS_QRING=1',
  'CMDRING.tapNow': 'TPS_QRING=1 (with the ring)',
  'CMDRING.stay': 'TPS_QSTAY=1 (needs TPS_QRING=1)',
  'CMDRING.stayDigit': 'TPS_QSTAY=1 (with the ring that stays)',
  'CMDRING.pin': 'TPS_RINGPIN=1 (needs TPS_QRING=1)',
  'CMDRING.wholeSquad': 'TPS_WHOLESQUAD=1 (needs TPS_QRING=1)',
  'CMDRING.goals': 'TPS_GOALS=1 (needs TPS_QRING=1)',
  'CMDRING.goalRing': 'TPS_GOALRING=1 (needs TPS_GOALS=1 and TPS_QRING=1)',
  'CMDRING.goalHalf': 'TPS_GOALHALF=1 (needs TPS_GOALS=1 and TPS_QRING=1)',
  'CMDRING.twice': 'TPS_QTWICE=1 (needs TPS_QRING=1)',
  'CMDRING.flat': 'TPS_RINGFLAT=1 (needs TPS_QRING=1 TPS_QONLY=1)',
  'CMDRING.plainHint': 'TPS_PLAINHINT=1',
  'CMDRING.bareHint': 'TPS_BAREHINT=1',
  'CMDRING.wheelIdle': 'TPS_WHEELIDLE=1',
  'CMDRING.only': 'TPS_QONLY=1 (needs TPS_QRING=1)',
  'DOORTOOLS.bangInto': 'TPS_BANGINTO=1',
  'SEENWALK.on': 'TPS_SEENWALK=1',
  'DOORTOOLS.haltBack': 'TPS_HALTBACK=1',
  'DOORTOOLS.behindSam': 'TPS_BEHINDSAM=1',
  'DOORTOOLS.chargeAnswer': 'TPS_CHARGEANSWER=1',
  'DOORTOOLS.podFrames': 'TPS_PODFRAMES=1',
  'DOORTOOLS.thrHand': 'TPS_THRHAND=1',
  'DOORTOOLS.thrFragHold': 'TPS_THRFRAGHOLD=1',
  'DOORTOOLS.fragHold': 'TPS_FRAGHOLD=1 (this rule alone: the threshold\'s half has its own word above)',
  'DOORWAY.sideKick': 'TPS_SIDEKICK=1',
  'DOORWAY.kickGlass': 'TPS_KICKGLASS=1 (acts only with the side kick and rounds through glass up as well: TPS_SIDEKICK=1 GLASSFX=on)',
  'DOORWAY.solid': 'TPS_DOORSOLID=1',
  'SHARDS.byFall': 'TPS_GLASSFALL=1 (acts only where glass breaks and falls by its model: GLASSFX=on)',
  'DOORWAY.leafWay': 'TPS_LEAFWAY=1 (acts only with one-way doors, ONEWAY=1, and turns a door only where furniture stands in its way: TPS_LIVED=1)',
  'FURN.mouth.on': 'TPS_HOLECLEAR=1',
  'FURN.mouth.snow': 'TPS_HOLESNOW=1 (acts only with a blown hole cleared: TPS_HOLECLEAR=1)',
  'FURN.mouth.base': 'TPS_HOLEBASE=1 (acts only with a blown hole cleared: TPS_HOLECLEAR=1)',
  'WX_OWN.roofs': 'TPS_OWNROOFS=1',
  'WX_OWN.wind': 'TPS_WXWIND=1 (acts only on a level of three or four roofs of its own in force: a picked weather, a snow level with TPS_SNOW=1, or TPS_OWNROOFS=1)',
  'SNOW.on': 'TPS_SNOW=1',
  'SNOW_LOOK.on': 'TPS_SNOWLOOK=1 (shows only on a snow level: with TPS_SNOW=1)',
  'SNOW_LOOK.wisps': 'TPS_SNOWWISPS=1 (shows only on a snow level: with TPS_SNOW=1)',
  'SNOW_LOOK.tracks': 'TPS_SNOWTRACKS=1 (shows only on a snow level with its surface: TPS_SNOW=1 TPS_SNOWLOOK=1)',
  'SNOW_LOOK.glow': 'TPS_SNOWGLOW=1 (shows only on a snow level with its surface: TPS_SNOW=1 TPS_SNOWLOOK=1)',
  'SNOW_LOOK.sparkle': 'TPS_SNOWSPARKLE=1 (shows only on a snow level with its surface: TPS_SNOW=1 TPS_SNOWLOOK=1)',
  'SNOW_LOOK.breath': 'TPS_SNOWBREATH=1 (shows only on a snow level: with TPS_SNOW=1)',
  'SNOW_LOOK.steam': 'TPS_BREATHSTEAM=1 (the breath as gas in the smoke solver: with TPS_SNOWBREATH=1 on a snow level, and only where a solver draws — the harness has none)',
  'SNOW_LOOK.grains': 'TPS_SNOWGRAINS=1 (shows only on a snow level: with TPS_SNOW=1; they fly only where the air field runs: TPS_AIR=1 on a page with a GPU)',
  'SNOW_LOOK.oneAir': 'TPS_SNOWAIR=1 (shows only on a snow level: with TPS_SNOW=1; it wants the air field: TPS_AIR=1 on a page with a GPU; with it the grains stand down)',
  'SNOWBOG.on': 'TPS_SNOWBOG=1 (acts only on a snow level: with TPS_SNOW=1)',
  'SNOW_LOOK.marks': 'TPS_SNOWMARKS=1 (shows only on a snow level: with TPS_SNOW=1; it wants the prints: TPS_SNOWTRACKS=1)',
  'SNOW_LOOK.past': 'TPS_SNOWPAST=1 (shows only on a snow level: with TPS_SNOW=1)',
  'SNOW_LOOK.under': 'TPS_SNOWUNDER=1 (a prelude const of the page: nothing headless draws it; shows only on a snow level, TPS_SNOW=1)',
  'SNOW_LOOK.blast': 'TPS_SNOWBLAST=1 (shows only on a snow level: with TPS_SNOW=1; the crater wants the prints: TPS_SNOWTRACKS=1; the throw wants the grains: TPS_SNOWGRAINS=1)',
  'AIR_SIM.on': 'TPS_AIR=1 (a picture only, and the harness has no field to feed: it changes what the game would hand one)',
  'GLASS.bothFaces': 'TPS_GLASSEARS=1',
  'WALLPEN.on': 'TPS_WALLPEN=1',
  'DOORSHOT.on': 'TPS_DOORSHOT=1',
  'WINBARS.on': 'TPS_WINBARS=1',
  'DOORTHROW.on': 'TPS_DOORTHROW=1',
  'HANDS.on': 'TPS_HANDS=1',
  'WORK.on': 'TPS_WORKTHINGS=1',
  'WAND.on': 'TPS_WAND=1',
  'WAND.mate': 'TPS_WANDMATE=1',
  'WAND.quick': 'TPS_WANDQUICK=1',
  'PEEKSIGHT.on': 'TPS_PEEKSIGHT=1',
  'STICKY.on': 'TPS_STICKY=1',
  'STICKY.take': 'TPS_CAMTAKE=1',
  'CUES.overheard': 'TPS_OVERHEARD=1',
  'STICKY.mark': 'TPS_CAMMARK=1',
  'OWNCHARGE.on': 'TPS_OWNCHARGE=1',
  'GUNSLOTS.on': 'TPS_GUNSLOTS=1 (rides on the card of the big slot: TPS_BIGSLOT=1 with it)',
  'KITTREE.on': 'TPS_KITTREE=1 (rides on the two gun slots\' card: TPS_GUNSLOTS=1 and TPS_BIGSLOT=1 with it)',
  'GUNLOOK.on': 'TPS_GUNLOOK=1 (nothing to see without a man who carries one of the guns: TPS_GUNSLOTS=1 and TPS_BIGSLOT=1 with it)',
  'GUNROLES.on': 'TPS_GUNROLES=1',
  'BIGSLOT.on': 'TPS_BIGSLOT=1',
  'KITDEAL.on': 'TPS_KITDEAL=1 (reads only with the pouches rule on: SLOTS=on)',
  'KITDEAL.more': 'TPS_KITMORE=1 (reads only with the deal and the big slot on: TPS_KITDEAL=1, TPS_BIGSLOT=1)',
  'KITDEAL.stealth': 'TPS_KITSTEALTH=1 (reads only under the card as a tree: TPS_KITTREE=1 and what it needs)',
  'ISSUE.whole': 'TPS_ISSUEWHOLE=1 (the sponge rounds are a row of the card as a tree: TPS_KITTREE=1 and what it needs)',
  'ISSUE.picked': 'TPS_KITPACK=1 (the page\'s and a recording\'s rule: no headless lesson run raises the issued page)',
  'SQUADTHROW.on': 'TPS_SQUADTHROW=1',
  'SQUAD40.on': 'TPS_SQUAD40=1',
  'AUTOPILOT.on': 'TPS_AUTOPILOT=1',
  'AUTOPILOT.plain': 'TPS_PLAINENTRY=1 (needs TPS_AUTOPILOT=1)',
  'AUTOPILOT.restack': 'TPS_RESTACK=1 (needs TPS_AUTOPILOT=1)',
  'CONDITION.on': 'TPS_CONDITION=1',
  'GUNHAND.on': 'TPS_GUNHAND=1',
  'INHAND.on': 'TPS_INHAND=1',
  'PACKSIDE.on': 'TPS_PACKSIDE=1',
  'STACKROOM.on': 'TPS_STACKROOM=1',
  'HANDLE.toolNear': 'TPS_TOOLNEAR=1',
  'HANDWORK.on': 'TPS_HANDWORK=1',
  'WORKSTOW.on': 'TPS_WORKSTOW=1',
  'SQUADNET.on': 'TPS_SQUADNET=1',
  'SQUADNET.why': 'TPS_QWHY=1',
  'SQUADNET.clear': 'TPS_HONESTCLEAR=1',
  'SQUADNET.endCall': 'TPS_ENDCALL=1',
  'SQUADNET.edge': 'TPS_NETEDGE=1',
  'TAKEDOWN.on': 'TPS_TAKEDOWN=1',
  'TAKEDOWN.haul': 'TPS_HAUL=1',
  'STEALTHPOSE.on': 'TPS_STEALTHPOSE=1',
  'GEAR.on': 'TPS_GEAR=1',
  'WORDS.on': 'TPS_WORDS=1',
  'WORDS.doors': 'TPS_DOORNAMES=1',
  'WORDS.radioHome': 'TPS_RADIOHOME=1',
  'WORDS.guide': 'TPS_GUIDEWORDS=1',
  'UNSEEN.on': 'TPS_UNSEEN=1',
  'UNSEEN.wallBeat': 'TPS_WALLBEAT=1',
  'HOLDFIRE.on': 'TPS_HOLDFIRE=1',
  'SILENT.stackSet': 'TPS_SILENTSET=1',
  'LOOK.on': 'TPS_LOOK=1',
  'MUSIC.pieces': 'TPS_MUSIC=1',
  'MUSIC.byLevel': 'TPS_MUSIC=1 (with the pieces: the one word raises both)',
  'MUSIC.theirs': 'TPS_MUSIC=1 (the same word again: whose nerves the tension reads)',
  'LIVED.on': 'TPS_LIVED=1',
  'ROOMCLEAR.fresh': 'none: tools/qa/fresh-start-check.ts sets it (it needs a Game with a die of its own, and playthrough.ts names every seed)',
  // tools/qa/headless.ts, as it loads
  'DOORWAY.oneWay': 'ONEWAY=1 (playthrough.ts plays it on)',
  'DOORWAY.frameHolds': 'QA_FRAMEHOLDS=1 (playthrough.ts plays it on)',
  'DOORMANNERS.on': 'QA_DOORMANNERS=1 (playthrough.ts plays it on)',
  'DOORBLAST.on': 'QA_DOORBLAST=1 (playthrough.ts plays it on)',
  'SLOTS.on': 'SLOTS=on',
  'CUT.on': 'CUTBOXES=1',
  // set by the page's boot, or by a game module for the page alone — never by the harness (shippedBoot above)
  'DESTRUCTION.on': 'PT_DESTR=1 (playthrough.ts)',
  'RUBBLE.on': 'PT_KICK=1 (playthrough.ts)',
  'EVICT.on': 'PT_POP=1 (playthrough.ts)',
  'DEBRIS.dress': 'TPS_DEBRISFALL=1 (acts only where things break: PT_DESTR=1)',
  'SQUADS.radioCheck': 'SQUADS=radiocheck-call',
  'SQUADS.noticedEyes': 'SQUADS=radiocheck-call',
  'SQUADS.callBeat': 'SQUADS=radiocheck-call',
};
export { STAND_ENV } from './standEnv';
import { standOf, type Stand } from './standEnv';   // (every environment word the two stands read — its own import-free file: a check clears them before the game loads)
