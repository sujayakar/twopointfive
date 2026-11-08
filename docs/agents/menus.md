# Menus, HUD screens, the debugger

One of the area files CLAUDE.md indexes. Everything below was moved here word for word from CLAUDE.md on 2026-10-13, in its old order;
"the bullet above / below" in a bullet means its neighbour there (each block says which lines it was), and a bullet named by its bold
title may now live in another file of this folder: `grep -l '<title>' docs/agents/*.md`. New bullets for this area go at the foot of this file.

<!-- was CLAUDE.md lines 16–36 -->
- **The menus are one game from the title to the debrief** (the look is ONE set of tokens, `src/ui/theme.ts menuTheme()`: warm black ground, sand text, flat fills, one orange
  accent `--m-accent`, the condensed heading stack `--m-head`; a canvas — the floor plans, the debrief's map — and the kit silhouettes repeat `--m-base` as literals).
  A fresh visit opens on the TITLE (`#menu.title`: **Play ›** · Options · Controls · Extras; Esc does nothing there; W S walk the buttons; EXTRAS swaps the title's own column for the list `ui/title.ts EXTRAS` — the replay viewer `replay.html`, the listening page `sounds.html`, each a link, a NAME only (its sentence is the row's tooltip), Esc back; the BENCHMARK row starts in place from the live title (no page load), and its link — what it is while the page boots, or in a new tab — is `?bench` with the visitor's other flags kept; the list's first section was the two SHOWCASES, removed 2026-10-17 with their `?demo` / `?breach` pages (docs/agents/showcases.md); the one-file build lists no tool page (tools/artifact-page.py marks its page, `onePage`); the replay viewer's bar carries a '‹ Game' link back; a row added to that table is a row on the title; what is kept and not listed, and why: rebuild/menus/tools.md); Play opens the ONE STARTING PAGE
  (`src/ui/startpage.ts` over the mission list — its rows in the list's six groups (`scenarios.ts MISSION_GROUPS`: training · raids · strongholds · against the clock · quiet alone · quiet as two), drawn the way Door Kickers 2 draws them (the maintainer, 10-11: "i think door kickers did a good job of this btw"): the groups are PICTURE TILES in a rail down the left (`.mgroups .msec`, each its first mission's picture and its count), the open group's missions a grid of CARDS, a picture and a name each (`.mgrid .mrow`, canvas `.mthumb`), the short list's missions first inside a group, a fresh visit on the compound — in TWO STEPS named on its head, MISSION › TEAM & KIT — Enter goes on, Esc back, Enter on the second deploys. A CARD'S PICTURE IS KEY ART (the maintainer: "our art style is also AWESOME so if we could highlight it" · "use your artistic judgment though. and it can be synthetic, doesn't have to literally be a rendering of the level. make it awesome!"): `public/assets/menu/<mission id>.jpg` + `stills.json`, read by `src/ui/menuStills.ts` (a list that is not there is the answer for the page's life; a server that did not answer, a 5xx or a picture that failed to load is asked for again when the Play page is next opened: `bun run tools/qa/menustills-check.ts`); since 10-12 (the maintainer on the composed set: "they look somewhat staged and repetitive … our actual gameplay looks a lot cooler!") a mission that has a RECIPE in `tools/menu-shots/recipes.cjs` is a frame of the REAL page with the mission being PLAYED — `PORT=<dev server> node tools/menu-shots/shots.cjs make [id …]` boots the mission's own link in headless Chromium on the software adapter, the in-page director (`director.js`, `window.__ms`: Sam on the walk keys, the squad on the menu's own orders, the fight the game's) plays it undrawn until the recipe's moment (a can going off, a round, a wall coming down), then the frame is drawn from a pinned camera (minutes a picture; `shots.cjs serve` keeps pages alive for finding a moment; a recipe gives that KIND of moment again, not the frame to the pixel; across a group no two recipes share a moment, an hour and a camera) — every other mission's picture is still the first set, COMPOSED offline by `bun run tools/menu-art.ts [id …]` — a small CPU path tracer over the mission's own level file (the look's file where there is one: boxes, paint, lamps, door leaves) with a cast placed for the picture (weapon lights and lasers in haze on a loud mission, one man low in the dark and the guards' torches on a quiet one, the captor and hostages on a rescue), its mood by GROUP (`MOODS`) and its place by mission where the level's own start does not make a picture (`PIN`: the office is five missions on one floor, each its own corner); not a capture of play, nothing is marked on it, no GPU; a mission added since gets its picture by running the tool again (two to three minutes a picture a core at the shipped 760 × 475, 40 samples: about half an hour for all of them on four cores; do not edit the tool while it runs — each picture is a fresh bun that imports it. A picture is aimed by its `PIN` row: who stands where — `at`, `ops`, `guards`, `aim` —, the camera — `look`, `yaw`, `pitch`, `dist` —, doors `ajar`, and any field of the mood) and until then — and in the one-file build, which has no folder beside it — its card shows the menu's own drawn model of the place, `src/ui/planArt.ts` (the level's boxes tilted as the game's camera sees them, walls cut, flat fills: also THE MAP SHEET beside the list, where the way in is marked). MISSION: the
  cards · a briefing · a SETUP rail of one row a setting with its VALUE (Difficulty, its three levers hung under it (a fourth since 10-23, `You're told`: stealth.md), Hour,
  Weather, Way in; a click or its key steps it; the list's own setting rows are parked unseen in `.mstore` and the rail's rows press THEIR buttons, so what a setting may
  be is still `ui/missions.ts`'s) · a Team & kit row · the place as that map sheet under the rail (with the tiles; not drawn in a window under 730 px tall). SHOW, NOT TELL (the maintainer, 10-11: "we should generally show, not tell. e.g. no \"play this first\", wordy explanations of different options, etc."): a group's heading is a word or two with its row count, no sentence explains a setting (a row's sentence is its `title` tooltip and the screen reader's live region `.mread`; a Custom difficulty arms in its own row as 'Custom → <level>'; the place line shows only where a mission has no plan; the kit step repeats no setting, the Approach fact and a chip's 'Sam' / 'Red pair' line are not drawn), the way in is an accent dot on the plan at the picked start — rings at every start for 'Dealt' (`ui/missions.ts drawPlan(…, mark)` off the level file's `scenarios`; none for the office's rows or a quiet mission), a roster chip shows what differs man to man as icons — his gun, what he carries a count of, the wand (`.lo-ics`, `loadout.ts CHIP_SHOWS`), a kit tile speaks only when a step is refused, and a carried wand or card camera on a mission with no fog of war — nothing for it to look into: the solo night always, a quiet mission with its fog lever off — says so on its own tile: the value reads 'no use here' in the dim grey with a faint icon, the label lit and the arrows live (`.kc-row.idle` ← kitCard.ts `MemberCardOpts.idle`, asked by loadout.ts `noFog()` off the mission's own link, redrawn when the screen's fog lever moves; an empty tile is like any other. It was the whole tile at 40 %, still reading CARRIED: the maintainer, 10-12, "wand greyed out"). A new line on a menu screen needs the same test: can it be shown instead. TEAM & KIT: team size, squad kit, the roster chips, the team at the mission's door in the column where the list was (`src/ui/kitView.ts`: below); THE LEVEL IS NOT DRAWN BEHIND A PAGE THAT HIDES ALL OF IT (the maintainer, 10-12, on Team & kit: "framerate isn't great here -- are we rendering this + the level?" — we were: the world under the start page and the options layer is frozen and both paint over the whole view, yet the level's whole frame was drawn every display frame; main.ts `viewHidden` now skips `engine.render` `behind.warm` 90 frames after such a page goes up — never while calibrating, under a sweep or a recording, a script's `__runFrames` or a tape; the title, the pause card and the options layer opened over a paused run — it lets the frame show through: `.over-game` — show the level and are drawn as ever; `?menurender` = drawn every frame as it was, `__behind {on, warm, drawn, skipped, hidden}`; `PORT=<dev server> node tools/qa/menu-idle-page.cjs` = the real page on the software adapter); `PORT=<dev server> node tools/qa/menu-art-page.cjs [outDir]` = the real page on the software adapter: the key art on the cards, and the drawn plan in its place on a page marked as the one-file build, ~4 min; the picked man's kit as tiles (`ui/kitCard.ts`,
  icons `src/ui/kitIcons.ts`). A sticky foot bar on both with the page's ONE filled block, which leads on: 'Team & kit ›' on MISSION (the page's own button on the loadout's bar, `.sp-next`; Deploy a quiet word beside it — the maintainer, 10-12: "confusing that CTA is to deploy but that skips the kit selection screen"), DEPLOY on TEAM & KIT; a mission played with the kit as issued (a Training lesson, root `.nokit`) has no kit to pick and keeps Deploy as the block on both). A man's kit holds ONE LINE on his roster chip however full his card is (the things are sized in ems of ONE icon height for all five chips: 17 px, or the chip's width over the fullest man's row — `--u` on `.lo-slots`, loadout.ts `drawSlots`; `PORT=<dev server> node tools/qa/kitline-page.cjs`, no GPU needed, `KITLINE_BREAK=1` must fail). The page comes up by itself only for
  `?loadout=<id>` or after a quit by page load (sessionStorage `twopointsix.missions.reopen`, read once by `missions.ts missionsReopenAsked`). OPTIONS is a full-screen
  layer of its own (`src/ui/options.ts`, `#options`: tabs GRAPHICS · AUDIO · GAME · CONTROLS on 1‥4, W S rows, A D change, Esc back), opened from the title and from the
  pause card (which is a narrow card: Resume · Restart · Options · Run so far · Missions · Quit); it took the place of the pause card's Settings and Controls tabs, so
  an older 'pause card ▸ Settings ▸ …' path in this file is Options ▸ that tab. The three cheats live on Options ▸ Game (a 'Cheats on: …' chip on the title while one is
  on); the four volumes and the hints switch are kept per machine in localStorage `twopointsix.options` (never the 0 a `?mute` page pins the master at). TAB IS THE OBJECTIVES CARD, every page (`ui/tabcard.ts`, `main.ts tabKey`; the trial of Nothing, Run so far, Order who and Controls, its Options row and `?tab=` went 2026-11-08), and Tab backs out first (`BACKKEY.tab`, docs/agents/squad.md).
- **The fight screen wears the menus' look** (2026-10-16; the maintainer's "sure a" to the call "The fight screen in the menus' colours: move the mint green
  monospace over to the menus' sand and orange"; before it the two read as two games). NO SWITCH, on purpose: it is a look, not a rule of the game (STYLE.md: no
  player-facing switches for art constants), nothing of the simulation reads it, so a run's digest is the same on both stands. WHERE: every rule of the fight screen is
  in `index.html`'s one sheet, and that sheet now opens with a `:root` block of `--h-…` tokens, the fight screen's ROLES (ink, plate, edge …), each
  `var(--m-…, <that token's value>)`: the menus' own token once `menuTheme()` has run (the Overlay's constructor calls it too now), the literal for the moments before
  the first script. So `--m-accent` moves the fight screen as it moves a menu (theme.ts gained `--m-bright-rgb` for the marks drawn in bright sand at an alpha).
  WHAT MEANS WHAT: sand is the ink (`--h-ink`; values and lit things `--h-hi`; hints `--h-soft` / `--h-dim` / `--h-faint`; hairlines `rgba(var(--h-edge), a)`; backings
  `rgba(var(--h-plate), a)`, warm black where they were blue-black). ORANGE (`--h-acc`) is spent on what leads on and nothing else: the lit interaction row's left bar
  and its OWN key cap (`.it.primary > .what + .opt kbd`: not the hint keys under it, and a row that cannot be done keeps its dim cap; the row's FRAME is the selected
  kit slot's sand frame, so 'chosen' is one thing in both columns; every other key cap is a plain sand cap; the cancel row lit is bright sand), the objective (its
  diamond, its reticle, its dot, and the line's own word: `overlay.ts` sets 'objective ▸' apart as a `<b>` label, caps in the orange, the text read back from
  `#objective` unchanged and a par clock in front of it left plain), the LIGHT bar while Sam is lit (`#meters.hot`), a lost man's way (it was the accent as a literal
  already), the debugger's chosen system / focus / ticked box, the showcase timeline's playhead. NOT orange: a row header of kind 'objective', because `orders.ts`
  uses that kind for every squad row that is not a door's; it is the bright ink. EVERY OTHER HUE KEPT ITS MEANING AND ITS VALUE: gold / red / blue = who an order
  goes to, amber then red = alarm and REC, blue = a door, salmon = a body, red = a man, amber = a breaker / RING / a key / an off thing, yellow = a changed row in
  the debugger; the one hue dropped is the cancel row's salmon (it sat a few degrees from the orange). NO GLOW, FLAT BARS: every coloured halo, the amber, blue and
  red ones too, became a dark one or none, and the bars' fills are flat (the menus are flat); text that sits on the picture takes `--h-shadow` (the old 1 px drop plus a
  5 px dark halo), since sand has no hue to stand off a sunlit floor with; for the same reason the objective line has a plate under it (and is not drawn while empty),
  unlit kit rows went from 0.55 to 0.7 opacity and the kit's key hints from the faintest ink to `--h-soft`. TYPE: labels (`.what`, LIGHT / SOUND, ALARM, CHECKPOINT,
  REC, kit names, the marks' words) are `--h-head` 700 at 11 or 12 px, kit tags 10 px; rows (`.opt`, `.hint`, radio, speech, the objective's text) are `--h-font`; key
  caps and the bars' numbers are `--h-mono`. `body` is still mono and so is `#hud` (the debug read-out is columns) and the debugger (`#panel`, `#ptip`): they are
  recoloured, not reset. ROW HEIGHTS WERE HELD: line heights are set so the open five-order door menu is 294 px tall (307 before) and a five-slot kit column 206
  (205), measured in Chromium on the page's own markup; the kit column is 326 px wide against 309 where `--h-head` falls back to Inter, and narrower than that on a
  Mac, whose 'Avenir Next Condensed' is the face the labels were sized for. FROM CODE: `ui/overlay.ts` gives the throw arc's two shapes a class (`.trajpath`,
  `.trajland`: an attribute cannot hold a `var()`) and colours the objective's dot from `--h-acc-rgb`. A NEW ROW ON THE FIGHT SCREEN takes these tokens, never a
  literal green or a typeface by name. CHECK: `bun run tools/qa/hud-look-check.ts` (reads files only, so there is no stand to run it on: no green-to-teal literal,
  pale mints included, and no green or blue near-black in any rule of the sheet bar the named things that ARE green, a can's paint and two keys of the debrief's
  map; each `--h-` fallback equal to theme.ts's value; every `font` in the fight screen's rules a token; no green in overlay.ts); `HUDLOOK_BREAK=1` adds one rule in
  the old look and 'no green' and 'type' must both fail ('tally' too: the next bullet). SEEN, all on the software adapter in headless Chromium at 1440 × 900, never on a GPU and never on a Mac:
  the real page on `?roomclear&squad4` (deployed, the order menu on a door, after a stack order, the debugger open) and the solo night at a door, each beside the
  same DOM under the old sheet, with no console error; the alarm line, CHECKPOINT, REC, radio rows, speech, head marks, both reticles and the order mark only as
  static markup under the real sheet. NOT SEEN BY ANYONE: the labels in the condensed face they were sized for, any of it over a sunlit floor at play speed, a lit
  LIGHT bar, a lost man's row, a par clock in front of the objective, the showcase timeline, the close-in windows, the throw arc. NOT TOUCHED, each its owner's and
  still mint mono on this screen: the F7 tag (`ui/replaykey.ts TAG_CSS`), two edges of the engine replay's bar (`replay/engineplay.ts`), the one-off tools
  (`ui/perfsweep.ts`), the debug floor views (`windviz`, `pictureviz`). TWO OVERLAPS REPORTED ON THIS SCREEN ARE NOT MENDED and were not seen
  in any of the pictures above: a long order menu reaching the kit column in a window under about 640 px tall (it reached it before; the menu is now 13 px
  shorter), and the objective's diamond drawn over the squad block when the objective lies up and to the left.
- **The run tally cuts no label** (10-20, a look and one word, no switch: `index.html` `#menu .db-stats`, `#menu .db-grid`; `src/ui/debrief.ts`). The tally on the pause
  card's *Run so far* and on the debrief ended three labels in an ellipsis at every window and text size ('bodies found' and 'doors kicked' 2 px over a tile's
  83, 'called in by security' 55 over). Now a label too long for its tile takes a second line and the tile holds its number at its foot, so a row's numbers stay
  on one line; a tile's side padding is 6 px (it was 8), which puts every label of the plain night on one line on the pause card at its full width (on the debrief, 12 px narrower, or with a scroll bar drawn, the two 85 px ones take two); the long one reads
  'called in' (its hover line still says a camera or a tripwire radioed you in; the summary line's words, chronicle.ts, are as they were). THE TALLY AND ITS MAP
  wrap by the CARD's width, not the window's: side by side while the card holds 360 + 18 + 420 px, and under that the map goes below the tally and takes the
  row's whole width, as the old one-column sheet gave it (the tally grows 999 parts to the map's 1, so beside it the map stays 420 px). It was a window rule at
  820 px: on the pause card the tally's column ran from nothing just over 820 px to its full width near 1260 px (136 px at a 1000 px window, every label cut).
  The 360 is chosen for SCROLL BARS: the debrief card holds 842 px and the pause card 854, so the pair stays side by side with a bar of 15 to 17 px drawn, as
  it did before (a first cut at 400 dropped the map under the timeline whenever a drawn bar appeared). In the narrowest band a few more labels take two lines,
  and a single word too long for its tile breaks, never spills. CHECK: two lines of `tools/qa/hud-look-check.ts`, both red under `HUDLOOK_BREAK=1`: 'tally' (no
  rule of the tally holds an ellipsis, `nowrap` or a line clamp; a tile is a column with its number at the foot) and 'tally pair' (the pair wraps, and its
  least widths fit the debrief card less a 17 px bar). SEEN: measured on the real pause card of the solo night, booted on the software adapter, before the
  change (the three cut, tile 101 px); after it only as markup laid out again under the new sheet, never a booted game: the pause card's own markup at 700 to
  1920 px wide with the five rarer long labels added (nothing cut; scroll bars hidden, as the headless browser has them), and a made-up debrief and pause card
  with 15 px scroll bars drawn at 400 to 1920 px wide (side by side wherever the old sheet was, nothing spilt). NOT SEEN BY ANYONE: on a GPU, on a Mac, on
  Windows, at a text size other than the first, on the debrief of a finished squad mission, or the squad-down card's replay picture where the pair wraps.
- **A quiet mission's gear screen** (2026-10-17; the maintainer: "quiet missions should have different gear screen stuff i think. be creative!", with
  show, not tell). The PANEL side of Team & kit; the picture beside it is the bullet below. WHEN: the mission's kind is quiet or it is the solo night
  (`ui/startpage.ts quietGear`), on the Team & kit step only: the root wears `quiet` (`ui/startpage.ts`, where `nokit` is set). `kitloud` on the address = the
  panel as it was. No FROZEN row and no knob: it is DOM and a look, nothing of the simulation reads it. WHAT: (1) THE LIGHTS ARE OUT: on that step
  `--m-accent` is the bright sand, so nothing on it is orange and the night picture is the brightest thing in the window. The cursor is told from 'on' by
  weight, not hue: 4 px of bright sand under a tab or beside a tile where 'on' is 2 px of the soft ink, and the roster's standing bar a step dimmer. Deploy is
  a hollow block: a 2 px bright frame over a faint fill. (2) THE TILES ARE SOLID PLATES (the panel colour at 0.94; they were the edge colour at 0.06), so the
  room does not show through them. (3) THE KIT READS BY NOISE, no word added: every tile carries `data-noise` 0‥3 (`ui/kitCard.ts`, the row's `noise`: `kit/card.rs kit_noise`), and
  on this screen a tile above 0 wears a sound mark after its VALUE (the name's line is cut with an ellipsis in a narrow tile and holds the arrows): a source
  and one arc = heard nearby and no alarm (an event with `loud: false`: the suppressed pistol's report, a smoke can's pop, a sticky camera fired and
  striking), two arcs = the floor hears it (`loud: true`: every other gun, a stun can, a charge, a hammer), three = the blast (`level: 2`, frag.ts blast(): a
  frag and a 40 mm grenade). Colour is spent on what he CARRIES: two arcs in the alarm's amber, three in its red (index.html `#alarm.l1` / `.l2`); one arc,
  and every mark on a thing he has none of, in the ink. A gun's step is read off its `WEAPONS` row when asked; the fixed ones are a table that cites the
  event each copies. So the quiet set reads at a glance: one arc on the pistol, and its one frag is the only colour on the card. (4) NONE CARRIED SINKS:
  a tile with a count of none, an empty big slot or no wand (`.zero`; never a fitting switched off, such as a teammate's suppressor) goes to the
  ground colour with its words at 0.65 (0.9 under the cursor). (5) AN EMPTY BIG SLOT is a tile like the rest, not the card's widest; with something in
  it, it is wide as before. NOT DONE: the tiles keep their order (the keys walk the card's own order); the picture's fade under the panel is
  `ui/kitView.ts`'s. A loud mission's screen is untouched (every rule is under `#missions.one.quiet.pane-loadout`). CHECKS:
  the walk (`tools/qa/cards-page.ts`: every tile's `noise`) and `PORT=<port> node tools/qa/quietgear-page.cjs [outDir]`
  (the real page in headless Chromium, no adapter needed: the look, the steps, a mark on every noisy tile and no other, the big slot, the quiet set's gun at
  one arc, `kitloud`, the solo night, a loud mission as it was, no page error; `QUIETGEAR_BREAK=1` must fail). SEEN: stills of the real page's DOM at
  1440 × 900 with no picture behind it, looked at twice by a reviewer. NOT SEEN BY ANYONE: the panel over the real night picture, on a GPU, in a Mac's
  condensed face, with a pair (a teammate's card), in a window under 1180 px wide.
- **The Team & kit picture is the Rust game's** (2026-11-08; the page's own TypeScript picture went the same day). WHAT IT SHOWS: on the Team & kit step, beside the cards, the real renderer's top-down picture of the team as picked, stood at the mission's own way-in door, and the man whose card is open using the thing under its highlight there, again and again. Anywhere else (a mission whose level file has no block for the way, a dealt way, a window under 720 px, a device refused) the column is empty: nobody stands in.
  HOW: `src/ui/kitView.ts` puts this page again in an iframe, on the very address Deploy would load plus `kitview&mute&noreplay`. The module starts that mission and STAGES it: `rust/game_start/src/kit_stage.rs` reads the words (way, set, `add`, dress) and makes the plan (`kit_plan.rs`, `kit_set.rs`), `rust/game_rules/src/kitstage` stands the team, parks everybody else, hangs the lamps and pins the camera, round the tick (`game_step_seats`); no rule reads it (`Game.kit_stage`, None in every other game). The level's block is `scenarios.kitView` (`docs/level-format.md`).
  THE MODULE FILES: `acts.rs` (the rows, one `start` and one `tick` a row in a plain match; the driver: `begin`, `finish`, `cut`, `tidy`, `waits`, `direct`), `walk.rs` (the walks; the card switch walks the same men; home), `aim.rs` (the aim channels; Sam's two yaws stepped in `after`; idle life), `quiet.rs` (the quiet picture's rows, loud kit's cost, life at rest), `cards.rs` (command 30 compares the cards and writes what moved live), `stage.rs`, `plan.rs`, `light.rs` (every number); state on `KitStage.a`. Only the game's own ways: the squad's orders, `g.puppet`, Sam's keys on the record, doors crack / pull_to, the door tools, the wall charge, `fire_weapon`.
  THE ROWS (`acts.rs` `beats`; a thing with no row plays the gun's): `weapon` the gun fires a short burst through a pane or the opened door · `light` his weapon light · `swap` holster, new gun, unholster · `bangs` / `frags` a teammate's on order, Sam's by his own hand · `smoke` a can through the door · `charges` a door charge set and blown (a teammate's, Sam's) · `wand` the optical wand at the door (Sam's, a teammate's) · `cams` a sticky camera, `shells` / `rings` the launcher's rounds · `walls` the wall charge on the wall · `bigtool` the hammer or the breaching shotgun · `turn` (the suit, the trim, what is worn) a turn that shows him. On a quiet kind (`quiet.rs`, Sam's card only): one pool of light with a crouched man beside it; `weapon` a suppressed round puts the lamp out, `turn` he leans a shoulder into the pool, `light` one blink of the beam across the door, and a loud thing shows its cost (the leaf thrown wide, the lamp).
  ONE WAY TO PUT BACK: what has no undo (a blown door, a frag's marks, a wall's hole, the lamp shot out) is held for `hold`, then staged again, the file's order kept and every man ON his place; `mend()` puts back what a burst leaves, plain. A restage the page should cover asks through the view's `veil` (the page lays its copy and answers 33 `KitStageUnder`; half a second of game time unanswered and it goes on). Nothing begins before the page has shown the picture: 34 `KitStageShown`, sent with its `ready`. The wand's rows have the fog on for the game: `game_draw` `fog.rs` draws none while a stage is up. The view's `run` and `ended` stay 0 under a stage. REFUSED, in the rows' own words (the gun's row plays instead; `kit_acts` holds the list): steel doors (`steel: a charge will not cut it`, `steel: no gap under it`, `steel: it holds`), a leafless way (`no leaf at this way in`), a blank wall until the charge has opened it, a card with no launcher or no wand, an empty big slot, a loud gun's quiet row (`not a suppressed gun`).
  THE PAGE'S HALF: Commands 30 `KitStageCards`, 31 `KitStageOpen`, 32 `KitStageFrame`, 33, 34; the View section's `kit` (1 staged, 2 nothing to stage). `src/host/kitpage.ts` (messages to commands; a team staged again is staged under a copy of the last picture), `src/host/kitguard.ts` (this window writes no store and hears no key), `src/host/kitViewQuiet.ts` (the kit screen's reading of a link's quiet kind, for the fade), `index.html` `body.kitview` (the canvas and nothing else).
  THE WORDS: `?stillkit` holds the picture still (`KitStage.still`: staged, no acts; the kit screen opens its frame so and sends no `sel`) · `?kitloud` puts the quiet picture off (the loud picture on a quiet kind).
  THE CHECKS: `bun run tools/qa/kitview-plan.ts [level …] [--quiet]` (what a level's author runs: every block over the planner in the checked-in module, `face_kit_plan`; no problems, no steel anchor, five places, sets and `add` laid; `KITPLAN_BREAK=1` must exit 1) · `game_start --test kit_levels --release` (every level with a block, every way, each kind the list has: it stages, lamps and pin finite, 5 s of ticks, nobody roused; 20 s, 2 minutes unoptimised) · `game_start --test kit_stage`, `game_start --test kit_acts` (every row on records, mousehole both ways, a wall span, a gap, a set, bazaar, vault, solo: its mark or its refusal, then every man home inside 8 s, nobody roused or hit; a pick cuts inside 0.1 s; the card switch; live cards; a minute left alone; `KITACTS_BREAK=1` on the red path) · `game_run --test kit_commands`. Played headless only: never in a browser at speed, never seen on a GPU. Sam's drawn bones come from the game's own write of his yaws within a step, the channel from the step after: never judged for jerk.
  WHAT THE MAINTAINER ASKED OF IT, each still held:
  "after selecting something, it should start the demo animation immediately" · "i don't like the fade to black animations": the iframe is never faded out, a restage is under its own copy.
  "switching between squad members should have them swap places in the stack" · "okay for switching weapons to have them holster what they've got, instantly get the new stuff, and then unholster".
  "animations for all the pieces of gear" · "guns when selected should do something! like shooting through window" · "selecting an item should still demo it even if it's at zero" · "big slots don't seem to do anything": every pick plays.
  "nobody off a slot": a row of Sam's waits for the men to be home.
  "some demos still don't really do anything (e.g. dynamic hammer here)": every way anchors on a door that is not steel, so every door tool has a leaf to work (`docs/level-format.md`, section C of `kitview-plan`).
  "we show the original gear screen before the new one fades in. let's just remove the old one with the figure?": no standing man; the column waits empty.
  "can we make it nicely abstracted so tweaks to one of them don't require churn elsewhere?" / "feel free to make a bespoke set for the gear screen … it should then just show off the vibe of the level / be recognizable": a level writes its anchor and whatever it pins, the planner measures the rest; a way may name a `set`.
  "quiet missions should have different gear screen stuff": the quiet picture, off the link's kind.
  EVERY MAN IS TURNED TO THE MIDDLE OF THE WAY IN, wherever he is put (`walk.rs faces`), on every mission: a place's written yaw turns nobody (the maintainer: "weird to have the guy by the door looking away"). Held by `kit_stage turned`.
- **The way in**: the title's Play opens the one starting page (`src/ui/startpage.ts`; the older start card and its short list of rows were removed 2026-11-07; `src/scene/picks.ts PICKS` still orders the page's first missions, carries `?pick=<id>` and the pause card's 'Next'). On the page a row's Team & kit is the **Loadout** (`src/ui/loadout.ts` + `src/ui/kitCard.ts`; `?loadout=<mission id>`), and Deploy is a PAGE LOAD of the mission's URL. An address that names a mission and draws no load brief (`?paused`, `?loadbrief=0`) boots behind the title's head and the status line with no buttons; a mission refused at boot opens the title with the refusal on it, and Options closed with the no-AI cheat off loads the page again. CHECKS: `probes.ts picker-card`, `missions-screen`; `tools/qa/startpage-look.cjs`, `loadbrief-page.cjs` on the software adapter. SEEN: on the software adapter only; never seen on a GPU, nobody has played it.

<!-- was CLAUDE.md lines 759–770 -->
- **The debrief = the chronicle** (`rust/game_rules/src/chronicle`, `Game.chronicle`, replaced by `restartEncounter`; `src/game/chronicle.ts` keeps only its record's shape and the two tables the knob registry reads): a capped timeline `events[]` of `{ t (mission
  clock), kind, room, pos, data }` — kinds `objective spotted contact heard lost calm alarm radio shot lightShot ocp blackout thrown grab interrogated keyTaken ko kill
  bodyFound doorKicked lockPicked hit down checkpoint` — Sam's PATH sampled every 0.5 s (flat Float32Array, thins itself at PATH_CAP), and the counters MissionStats
  lacks (`counts`). Written by ONE-LINE hooks at the true sites (guards.ts hearingCheck ×5 / the alert transition / dropToSearch / the search's end / raiseEscalation +
  stepDown / say(radio) / both body-found sites; combat.ts fireWeapon ×3 + killGuard; lighting.ts setBlackout + ocp ×2; player.ts takeKey / kick impact / pick done /
  throw release / startGrab; game.ts interrogate; mission.ts spotted / objectives / `tickChronicle`) through the SAME gate as the tallies (`missionLive`: nothing under
  the tour / puppet, nothing once done or failed). New verbs: call `chronicle` or add a `chron_x` helper there; a save's or a load's row is the game's own, written under the save's label (`face_checkpoint`, `face_loaded`). What the page's own books ask of a floor point (the autosave's 'first time in a room' and its label, the verdict's rows, the devices) is `src/game/rooms.ts`. Reading: the page asks ONCE, as the card is built, `face_debrief_view` → `view::debrief_view` (`game.debriefView()`, the record `src/view/debriefview.ts`): `debrief_of`
  → counts + `stealth_score` (100 minus SCORE's price list — every term only subtracts; `the_score_only_goes_down` holds it) + `rating_of` (bands: GHOST ≥ 90 · SHADOW ≥ 70 ·
  OPERATIVE ≥ 40 · ASSAULT) + `badges_of` (BADGE knobs) + the summary line; `rows` = each row in words (`event_text`), `path`, `recap`. A tool reads it with `tools/qa/debrief-read.ts`. The card is `ui/debrief.ts renderDebrief(host, end, actions)` over a `RunEnd` (`view/runend.ts`), made by `host/gameend.ts` on the page that holds a game and by `host/drawend.ts` on the page that draws (`face_end`, `face_debrief_view`, `face_floor`; there `ui/pausecard.ts` is the end card, its W S walk and 'Run so far'; picked names not shown on it yet; `hands-check` holds the record to the verdict's lines at a run's own end; seen in software only)
  (menu.ts: mode 'debrief' on the game's out event EndCard — sent on exfil AND `DOWN_CARD_SECS` after Sam drops, `END_CARD.due` for the page's frame — plus the pause card's *Run so far* tab; CSS `#menu .db-*`);
  the map is a canvas at nav-grid resolution + percent-placed DOM pins (PIN table). QA: `rust/game_start/tests/debrief.rs` and chronicle's own tests natively, `probes.ts debrief` (run / rooms / loud / dom) for the card as the page reads it, soak's 'leak chronicle' invariant.

<!-- was CLAUDE.md lines 1735–1757 -->
- **The sandbox row** (Options ▸ Game, `SANDBOX_LS` = localStorage `twopointsix.sandbox`, applied by main.ts `loadSandbox` at boot; a mission refused for 'no AI' loads again the moment that cheat goes off: the title's `again`): god mode ·
  no AI · unlimited ammo — the same `Game.godMode / aiEnabled / infiniteAmmo` the pause card and Tab ▸ Sandbox flip; unlimited ammo covers magazines, canisters AND ring airfoils.
- **The tweak panel = the debugger (ui/panel.ts + bindings.ts), on BACKTICK since 10-10 (Tab is left to the game; no key steps the debug views: they are the 'debug view' row on its Frame page)**:
  a two-level tool since 08-12 — SYSTEMS down a left rail (since 10-10 the GAME's pages first: Sandbox (cheats, the kit, the chaos dial) · Missions (difficulty,
  the scenarios, fog of war) · Squad (orders, door tools, Team feel split by subject) · Enemy AI · Stealth · Player & weapons · Destruction · Effects · Weather & wind, then the renderer's:
  Frame · Direct · Indirect · Volumetrics · Materials · Denoiser · Renderer fx · Audio · Checkpoints; `bindings.ts SYS` is the list, a section is filed on a page by the
  `panel.system(SYS.x)` line above it, and a Team feel block in the Squad section the `tf = TF.<subject>` line above its `feelRows(table, rows)` call last set — the table's name does NOT pick the section) and collapsible SECTIONS on each page (live `summary()` in the header, ↺ where a defaults
  table exists); a status strip; search across every system (results carry their 'System ▸ Section' path); 'all knobs' reveals the A/B rows (`adv`) and rare sections; ← / → or
  [ / ] walk the rail (keys reach the panel FIRST through `Input.keyVeto` = `panel.onKey` while the pointer hovers it or focus sits inside it: the nav keys step it. A PLAIN LETTER IS THE GAME'S
  (2026-11-01; it used to open the search whenever the pointer was over the panel, and the maintainer, playing with the panel open: "the search box in the debug view grabs input really
  aggressively ... make it so it only takes input when explicitly clicked on"): the search types from a click on it until Enter (which jumps to the first hit), Esc, a click elsewhere
  or the pointer leaving the panel. Only the search is blurred by the pointer leaving: an open list keeps the keyboard until it is done. While the search types the panel keeps its
  height, so a query with few rows does not pull it from under a pointer resting low on it (a tall window, a long page). (A scrubbed number row takes no focus at its press and would
  leave the keyboard with the search; the game's panel has no number row today, they are the editor's and the viewer's.) No switch (the panel is
  a debug tool, and the headless harness never comes through `onKey`). Checked by `tools/qa/panel-keys.cjs` with real key events on the software adapter (`PANELKEYS_BREAK=1` must
  fail one line); never tried on his machine. BACKTICK stays the panel's own: it is main.ts's
  hotkey (`panel.toggle()`, not while a card has the game frozen), `onKey` hands it back and it is never typed into the search, and a FIELD inside the panel that holds the keyboard — the
  search box, a slider just dragged, a list, a number — shuts the panel on backtick through the panel's own capture listener, since Input never sees a key a field keeps. Tab is only
  `preventDefault`ed by game/input.ts and handed to the game like any key — nothing in src/game reads it yet; the mission list walks its own focus with
  it while it is up. The WORKBENCH: backtick hides / shows its right-hand panel (the inspector) in every mode, Tab does nothing, and its debug views are ⚙ ▸ Renderer ▸ 'debug view'
  in the Characters / Effects modes and the 'view' list in the level top bar's second row in the Level / Materials modes; `[` `]` are still exposure. The native host: backtick
  toggles its panel, no debug-view key); open page / section / scroll kept in
  localStorage `twopointsix.panel`. Rows for WIND come from
  `knobRows(spec)` (which builds no row for a key the page holds at one value: the bullet 'THE DEBUGGER HAS NO BOX FOR A SETTLED SWITCH' below); everything with custom logic stays hand-written. `tools/qa/panel-inventory.ts` (`probes.ts panel-inventory`, family `ui`) freezes the row inventory against
  fixtures/panel-inventory-main.json — a row may move but never vanish by accident (a row goes on purpose by `--gone` or `--write`, as the game's pages did on 2026-11-07; and a box of a SETTLED switch, taken out on purpose: its row leaves the fixture too, as on 2026-11-07, the bullet 'THE DEBUGGER HAS NO BOX FOR A SETTLED SWITCH' below; a row REWORDED on purpose: `--reword [map.json]` rewrites only the fixture rows that went missing, each from the live row of the same kind and label, or the label the map gives); `PORT=<dev server port> node tools/qa/panel-keys.cjs` = the keys pressed for real in headless Chromium (the software
  adapter): backtick opens / shuts the panel from the game, from its search box and from a focused slider, Tab reaches the game, no debug view is stepped (exit 0 / 1, 2 = the script died).
  SINCE 2026-11-07 THE PAGES ARE Sandbox (with the extra enemies), Missions (difficulty, enemy nerve), Weather & wind, Frame and the graphics pages, Audio, Saves: Squad, Enemy, Stealth, Player, Destruction and Effects went, with Missions' staging, room clear, fog of war and hostage rescue rows.
  NOTHING KEEPS THE GONE PAGES' NUMBERS (2026-11-08): nobody can write them any more, so nobody reads them, and every machine plays the shipped numbers, as a server does. Their stores, and `teamfeel.ts`, are deleted: aim, blastfeel, buddy, chaos, civilians, clearline, cues, devices, bodyLight, dust.v2, glassfx, glassfracture, glasspen, laststand, needs, pair, playerCrouch, qrf, shards, sneak, spotted, tactics, teamfeel, whistle. An older bullet that says one of these is kept per machine is older than this. KEPT, because a page that stays writes them: difficulty, levers, pressure, digin, sandbox, wet, wind, prain, quietfog, quietcams, and the visit's morale and netquiet.
  Every 'Tab ▸ X ▸ Y' path in this file, in code comments and in docs/internal means the debugger's page X, section Y (where the page still stands: most of these paths name pages that went) — the key moved, and on 10-10 the old Sandbox page's sections were dealt to the six game pages above (a 'Tab ▸ Sandbox ▸ …' path is that section on Missions / Squad / Enemy AI / Stealth / Player & weapons now, 'Team feel' the Squad page's sections; sections renamed that day: the two called 'Glass' are 'Glass (the picture)' on Renderer fx and 'Glass breaking' on Destruction, 'Renderer — decals / particles' are 'Decals' / 'Particles', the weather's is 'Weather'); a row's label is unchanged unless the page rebuild/debug-tools/read-first.md lists it (some forty were reworded), so the panel's search finds a row by the words after the last ▸.
  `Weather & wind ▸ Wind — field view` (ui/windviz.ts, drawn by overlay.ts on the 2D canvas): arrows on a floor lattice from `wind.at()`, the sources' cones / gusts, the ≤ 8
  records the particles get, a probe at the cursor; ~0.35 ms CPU at its 534-sample maximum, nothing when off, the frame untouched.

<!-- added 2026-10-13, after the cut -->
- **THE RECAP'S RECORD IS THE RUST TICK'S (2026-11-07).** `rust/game_rules/src/deathrecap` keeps who of ours fell, to whom, from where, with what, the floor's last seconds and what the squad knows of a man lost (`Game.recap`; only `lost_live` is on the wire), last in the tick, with `rust/game_start/tests/recap.rs` holding each rule. A kept game has none of it: a load starts it over. The bullet below is the rules as they were written and the page's side of them.
- **The down card says who shot you, and from where** (the death recap goal, 'Every death explains itself'; `rust/game_rules/src/deathrecap`, kept by the tick and read through `face_debrief_view`; `src/ui/recap.ts` draws it; `src/game/deathrecap.ts` keeps `RECAP`, the record's shape and what the squad block reads; step 1 = commit
  456c2e1b, in the build since 8d2c2785; its page is rebuild/death-recap/read-first.md): when Sam goes down the down
  card has a small plan above the route map — the man who shot him in the menu's orange with his name, the line of the round with an arrowhead, Sam as a white cross, a faint wedge for where
  he was looking — and under it the man's name, his gun, the distance. The same record is kept for every teammate and hostage who goes down (rounds, fragmentation bursts, a hostage shot by
  her captor; a stun canister is never named a killer, a knocked-out civilian is not a death). NOTHING IS SHOWN WHILE ANYBODY IS STILL PLAYING: the record is handed to the card only once the
  run is over (Sam down, or the mission ended either way); played on past the end ('Back to the floor'), what happens after the end stays hidden until he is down. It only READS the game: a
  run digests the same with it on and off. Off: `?norecap`, or Tab ▸ Player & weapons ▸ Player, doors & props ▸ 'end card: who shot whom'. `bun run tools/qa/deathrecap-check.ts` (`probes.ts
  deathrecap`, in family `ui`; both stands; `RECAP_BREAK=1` must fail), `PORT=<dev server> node tools/qa/deathrecap-page.cjs <outDir>` (the real page: a guard shoots Sam, the card's panel
  names the man the game's own log names; no panel under `?norecap`). STEP 2 (commit 4dcb3322, in the build since
  ecfbfd0a): THE PLAN PLAYS THE TAKE — the last `RECAP.secs` 8 seconds before he fell and the `after` 1.2 past it, at the speed they happened, a 1.8 s rest on who did it, then again
  (`src/ui/recap.ts mountRecap`: the DISPLAY's clock, never the game's; every man placed between the kept samples by `recapMenAt`; asleep while the card is put away, stopped when the
  panel leaves the page; a still of the fall alone where the viewer asks for less motion or the host has no frame clock; a chip a man, by name, when more than one of ours was lost;
  `window.__recap` {show, seek, stop, take}). The software adapter fires no animation frames once the game is up, so `deathrecap-page.cjs` steps it with `seek(k)` and
  `deathrecap-check.ts` holds the clock on a fake frame clock (`RECAP_BREAK=snap` — every man read off the last sample alone — must fail; a `look` argument after the page tool's
  outDir adds a squad page with no verdict, stills only). STEP 3 (commit 1743d599, in the build since 09e6b525): WHILE THE RUN IS ON (`RECAP.live`,
  `?recaplive=0`; the game's `recap.lostLive`) a squad man down has his squad-block row struck (`game.view.squad`, a mate's `down`, `overlay.ts`), and only if a man still up saw
  him fall or heard the shot does the squad get, a beat later, the way it came from, to an eighth of a turn, in the
  witness's side word ('from the left!', 'from behind us!' — the fog's own sides, `cues.aside`, never a compass word); never the shooter. NOTHING OF IT IS DRAWN since 10-14, and the drawing is deleted (the bullet 'A lost teammate's way is no longer drawn' below). `deathrecap-page.cjs` holds the one squad page, `live`. The hearing test repeats
  `SoundPaths.propagate`'s rule WITHOUT its cache. Check = section 11 of `tools/qa/deathrecap-check.ts` (`RECAP_BREAK=seen` must fail) and the live pages of `deathrecap-page.cjs`. On
  a squad's card the recap is a `verdictPanels` panel (`debrief.ts recapPanel`), under the men's rows; over the route map otherwise. Seen only on the software adapter.
- **Unplayed mission rows are drawn apart on the list** (2026-10-13; the owner: 'The 25 untuned rows are the backlog: play each once, say keep, fix or cut. Unplayed rows sit apart and
  earn no medals' — this is 'sit apart' only; medals are not built here). THE RULE, `unplayed(m)`: the row is `untuned` and the short list does
  not name it (`onShortList`) — exported for whatever a mission earns to ask, and the sheet's 'untuned' tag asks it. Today 25 untuned, 20 unplayed.
  DRAWN: inside each group the unplayed rows come after the others (`unplayedLast`, a stable partition, taken by `missionList`; the start page's `startPageList` puts the short list's
  rows in front of both halves; the keys walk the same list); each wears `.mrow.unplayed` (picture 0.45, title and tags 0.5, full again under the pointer or the highlight) and the first
  of a group's wears `.first` when a played row stands before it: on the older list a thin rule in 15 px of room above the row, outside the row's own box. ON THE CARDS NO LINE IS BROKEN
  (tried and taken out the same day: a line of their own for the unplayed added a row of cards to two groups, and at 1366 x 650 and at two columns the grid squeezed every card of the
  group until titles sat on the rule) — there the unplayed stand last and quieter, nothing else. No word is added (the page's words are the same on and off). The loadout screen's
  'untuned' tag asks the same `unplayed()`. A group of nothing but unplayed rows (Quiet, as two) has no break. On the start page the order
  did not move today (its short list already stood first and no other played row stood behind an unplayed one); on the older list 8 rows stand elsewhere. CHECK:
  `probes.ts unplayed` (family missions; `tools/qa/unplayed-check.ts`, lists as data and both screens on the fake DOM; the switch-off order is written out again in the check, with no literal,
  so a row added to the table or taken off it moves nothing) — ALL PASS on both stands, `UNPLAYED_BREAK=1` fails on both. `missions-list`'s order line holds the switch off (it pins the order before this). LOOKED AT:
  `tools/qa/unplayed-page.cjs` on the software adapter at 1280 x 720 (classes, order, computed opacity, the list's rule; no page
  error) and stills taken with no GPU at all; the page script also holds the open group's cards to the SAME BOXES with the switch on and off. Never seen on a GPU, nobody has played with it.
- **Every run ends on a verdict** (`src/game/verdict.ts`, `Game.verdict`, ticked in `Game.step` after the scenarios; `VERDICT {on, winSecs, lossSecs}` in the knob registry,
  settled on; READS ONLY — it moves no run's digest). A TRY starts at the par clock's rule (the first order logged, Sam
  1 m off his mark, a round or a throw); `ended()` names each mission kind's end (CLEAR / RESCUED / OUT · DOWN / WIPED / LOST / GOT AWAY /
  FAILED); `result: RunVerdict` = word, clock, a row per man (path, where he fell, the defenders he dropped), the way a WON run was won — `Approach` doors / walls / dark /
  quiet, one a run, the rarest it earned: nobody ever hunted the team (the stealth game: no alarm, nothing spotted either) = quiet, the mains cut before it was found =
  dark, a man by a hole a blast made = walls, else doors — `clean` (nobody lost) and `void` (a cheat on during the try; its own latch — `m.stats.sandbox` latches before
  the try too). The mission's own card flag (`m.debrief`) raises the card where it has one; every other end is polled by main.ts `postUpdate` (`takeDue()`, a beat after).
  THE PAGE keeps what a machine has won (`src/ui/verdictStore.ts`, localStorage `twopointsix.verdicts`, key = mission id | difficulty: tries, the streak since the last
  win, per way the fewest tries and whether clean): `src/ui/runBooks.ts` counts a try the frame it starts (a checkpoint load is another try; never under a cheat or a
  tape) and files the end BEFORE the card opens (`Verdict.kept`); the game says what there is to keep (`face_run_view`, view `RunView`) and either page tells the books (`runbooks-check.ts`; `RUNBOOKS_BREAK=1` must fail). The card: `src/ui/verdictCard.ts` (head, men, `verdictPanels` = the slot other goals' panels fill)
  hosted by ui/debrief.ts (`.vd-squad` hides the stealth game's rating / numbers / timeline, `.vd-solo` keeps them), pips `src/ui/verdictPips.ts`; the START PAGE's
  mission rows get the same pips from `src/ui/verdictRows.ts` (`watchMissionRows()`, a MutationObserver on `#missions` — the list's own files are not edited; no pips on
  Training rows). QA: `bun run tools/qa/verdict-check.ts` (`probes.ts verdict`; `VERDICT_BREAK=1` must fail), `node tools/qa/verdict-look.cjs <dir> [win|down|solo|off|cards]`
  (the real page, software adapter; `cards` needs no adapter). Page: rebuild/run-verdict/read-first.md. Never seen on a GPU.
- **Three rows are cut from the mission list** (2026-10-13; the maintainer's tap 'Three' on the card 'Cut which untuned rows from the mission list?'): `room-clear-four` (the office a third
  time), `quiet-yard-alone` and `quiet-yard-two` (the quiet engine's test floor). Each carries `offList: true` in `scene/scenarioList.ts` and STAYS A ROW OF THE TABLE: its id, its own
  address, its level file, `?loadout=<id>` and every tool that names it hold as they were. The list is the desk's (`start_list.rs start_rows`): `?offlist=0` = every row drawn, as before the cut; a row the visit names is drawn that once. (2026-11-08: `OFFLIST_UI`, `listedMissions` and their check went with the page's list; `rust/game_run/tests/desk.rs` holds this.) The real page by
  `tools/qa/unplayed-page.cjs` (no card for a cut row; cards again under `?offlist=0`). To cut another row: `offList: true` on it and its id in BOTH `CUT` lists
  (`tools/qa/offlist-check.ts`, `tools/qa/unplayed-page.cjs`) — only on the maintainer's yes. Of the block pair, `block-squads` was cut later the same day (the bullet at the
  foot of this file); `block-four` stays. Never seen on a GPU.
- **The cap of eight rows a group is asked of what the page draws (2026-10-27).** The list's rule ("no group holds more than eight") was counted over the
  whole table in three probes (`missions-list`, `unplayed`, `picks-list`). The tutorial's table holds nine rows since "The tunnel" (`tut-team`) joined, six
  of them drawn (`tut-sky`, `tut-solo` and `tut-squad` are `offList`), and all three probes went red on both stands. No rule of the page changed and there is
  no switch: the bar is one helper now, `tools/qa/listed.ts` (`groupsFit`: `listedMissions` with `OFFLIST_UI` on, put back after; every group draws at least
  one row and no more than eight). A cut row stays a row of the table and is not counted. NOT HELD by the cap, and nine cards can stand there: `?offlist=0`
  (every row of the table drawn: the Tutorial group draws nine today) and a visit that names a cut row (`?loadout=<id>` draws that one row that once; a group
  with eight drawn rows and a cut one would draw nine). `startpage-look.cjs` asks its own count of eight on the real page with the cut rows off. RUN: the three
  probes ALL PASS on both stands; `UNPLAYED_BREAK=1` still fails `unplayed`. Nothing here was seen on a GPU.
- **A run's page is nobody's try** (2026-11-08; `main.ts offBooks`, `host/runpage.ts ownPage`: `benchrun`, `pricerun`, `bench`, `pricelevels` on the address; no switch, a rule of the page). The fight on such
  an address files no try and no par best, opens no end card, for a win or for Sam down, and writes no `run` on the cloud. The address is what it goes by, so a reloaded scene (its record dropped) and a refused
  `?bench` (no session store) play on with no card either. It stands where `?noverdict` once did, a word that stopped doing anything when `VERDICT.on` settled, so that for a while a benchmark's
  won scene froze behind a card and filed a try. CHECK: `tools/qa/runbooks-page.cjs`, the kill house on the page by `__killGuard`, seed 1: won on a plain address (card, try and time filed), won on all three other addresses, and Sam shot down on a plain and a scene's address.
  Without the rule (the head before it) every run-page line of card and books is red, but the par best on a loss, which files no time. NOT RUN: `bench-page.cjs` (a whole run is hours); the cloud line, read only (the check boots `cloud=0`); the stealth game's own end. Never seen on a GPU.
- **A Training lesson ends on the verdict card too, graded as a lesson** (`VERDICT`): lessons 5 and 6 are room clears, so their end is the
  room clear's `'taken'` / `'wiped'` and the card comes up a beat later like any squad mission's (before the verdict a room clear had no card at all: only the row 'structure taken
  — shift+enter stages it again'); lessons 1 to 4 and 7 end at the exfil on the stealth mission's own card. On a lesson's card NOTHING IS GRADED OR KEPT: no ways pips, no tries
  ticks, no record under `twopointsix.verdicts` (`ui/runBooks.ts`, by `RunView.lesson`: the running row's group → `KeptNote.lesson`; `verdictCard.ts verdictHead` draws the word and the
  clock only; `verdictLine` leaves the approach out). A lesson WON leads with `Next: <the next Training row's title> ›` (`src/ui/verdictNext.ts nextLessonOf`, wired in
  `menu.ts debrief`: the first button, primary, the focus; the row's own page load by `missionHref`; none after lesson 7 or on a loss; since 2026-10-26 it is the next lesson the lists draw, a row cut from the list being skipped: `missions.md`, the two missions off the tutorial's list). The room can fall
  before the lesson's last step is done (lesson 5's scripted run: the card was due 5.7 s before the last line went by): the card wins, as the row already did. QA:
  `bun run tools/qa/verdict-lessons.ts [name …]` (the scripted player of `lesson-run.ts` plays each of the seven to its end, input only; the run must end WON with a card due;
  ~1.5 min; `VERDICT_BREAK=1` must fail), `node tools/qa/verdict-look.cjs <dir> lesson` (the real page, software adapter: lesson 6 taken, the card up by itself, no pips, no
  ticks, nothing kept, `Next: 7 · …` first and focused, its link the next row's). Never seen on a GPU; never played by a person to the card.
- **Four missions are back on the list** (2026-11-08, his "can we bring back records office, block, warehouse, and kill house?"): `records-pair` (Raids), `block-four` and `warehouse-four` (Strongholds) and
  `killhouse-four` (On the clock) lose `offList` in both tables (`scene/scenarioList.ts`; `rust/game_types/src/scene_scenario_list/types.rs`, which is kept by hand). The list draws seventeen levels on eighteen
  rows, and the solo night. The short list (`PICKS`) is not touched. The records office's and the warehouse's ways in are named without a bearing, as the sheet draws them. The kill house's clock and medals
  (`par`) never left. The bullet below is the cut as it was made: of its sixteen, these four are back and two (`snatch-four`, `defuse-four`) have no row left. CHECK: `rust/game_run/tests/desk.rs` (a drawn row is no cut row, six groups, eight rows a group at most).
- **The list draws his thirteen** (2026-11-07, doing his "Keep my 13" of 2026-11-05): `tut-road`, `tut-team`, `tut-kit`, `tut-queue`; `pass-four`, `compound-four`, `rescue-consulate`, `depot-four`,
  `mousehole-four`; the office's `room-clear-pair` and `hostage-rescue`; `vault-quiet`, `bazaar-quiet`, `lights-out-two`; and the solo night. The other sixteen carry `offList: true`: `tut-quiet`, `tut-pair`,
  `records-pair`, `block-four`, `warehouse-four`, `barracks-four`, `rescue-registry`, `snatch-four`, `defuse-four`, `floodlit-four`, `killhouse-four`, `hall-four`, `glasshouse-four`, `police-post-quiet`,
  `quiet-crossing-two`, `quiet-relay-two`. Their level files, rows and addresses stay; `?offlist=0` draws them. The tutorial now ends after `tut-queue`. The short list (`scene/picks.ts`) names only drawn rows, so
  it is three: the compound, the depot, the solo night; no row sets an hour any more, and `Pick.hour` stays as the mechanism. The pricing run and the level tools read `listedLevels`, so they walk the thirteen
  levels. Page checks whose question is one cut row open with `offlist=0` (`sticky-page.cjs`); `probes.ts offlist` and `unplayed-page.cjs` name all twenty-four. Never seen on a GPU.
- **The block's three-squads row is cut from the mission list too** (2026-10-13, the fourth of the maintainer's tap 'Three': "One of the two block rows goes later, when the block's
  rebuild says which"). The rebuilt block is for `block-four` (eight men who hold their rooms); `block-squads` carries `offList: true` and stays a row of the table, as the other
  three do: its own address, `block-squads.json` and `?loadout=block-squads` hold (its `ways` now name the rebuilt floor's two starts, the same as `block-four`'s: the two files
  share them). The cut changes no check on the level file: what is red on the rebuilt block floor is the floor's (docs/agents/levels.md), not this row's. `?offlist=0` draws it again. `probes.ts offlist` and
  `tools/qa/unplayed-page.cjs` name the four. Checked headless and on the real page with the software adapter; never seen on a GPU.
- **A partner's men are on Sam's stealth card** (`src/game/teamtally.ts`, `TEAMTALLY.on`; the maintainer's yes 15:33Z 10-13 to the quiet levels page's call 35. ON in the
  page — `?noteamtally` (or `?teamtally=0`) = only the player's own are counted, as it was; OFF in the module and on the frozen stand, `TPS_TEAMTALLY=1` there;
  `FROZEN_HOLDS`, `standup.ts`, `standEnv.ts`, `knobs.ts` + `JOURNAL_KNOBS`, `journal.ts applyKnobs`: a file without the row replays off): `combat.ts killGuard` prices
  a man only when `by === 'player'`, and the partner's round (combat.ts, `'guard'` + `killerId`), his ordered takedown (`pairorders.ts`, no `killerId`) and his own
  (`spotted.ts`) all arrive as `'guard'` — so a quiet mission as two where he shot the shift read 'kills 0 · no kills · pacifist'. On, `teamTallyTick` (game.ts, the
  step's tail, before the checkpoint and the journal close the frame; quiet missions with a partner only) tallies each defender the partner put down THAT step as the
  player's would be: a round that kills = a kill, anything else = a knockout, a round into a man = a round near a man (so no 'pacifist'), with its timeline row
  (`… shot dead (Blue One's round)`). A round `killerId` does not give to a teammate stays on nobody's card (a defender's stray); a man struck or choked by `'guard'`
  is the partner's whoever is named. Stateless (`diedAt === g.time`), nothing in a checkpoint. Squad missions are untouched (their card has no stealth numbers).
  Unswitched, with it: on the verdict's own rows an unnamed takedown ticks the partner's row where the team has one AI man (`verdict.ts onlyMate`). QA:
  `bun run tools/qa/teamtally-check.ts` (`probes.ts teamtally`, family `quiet`; seven lines; `TEAMTALLY_BREAK=1` must fail; both stands — the check sets the table
  itself after the stand-up). Never played by a person; the card with a partner's kill on it never seen on a page.
- **A lost teammate's way is no longer drawn (`RECAP.liveArrows`, 10-14; the maintainer: "we shouldn't show the arrows, feels unrealistic").** The live recap
  (`RECAP.live`, the game's `recap.lostLive`) drew the way the round came from twice: an arrow on the man's struck row in the squad block and an orange stroke with
  a head on the ground at him (`.lostfrom`, `ui/overlay.ts`). Both are deleted (2026-11-08): the row is still struck and the man who saw still
  says which side — the squad's own word is all the player gets. The key is
  false in the module and on both stands (the overlay only: no step reads it, no digest moves, not in the journal). Check: `probes.ts deathrecap` (28 lines; the
  new one reads the staged 'mate' run, the key both ways, and that overlay.ts asks before the stroke and before the row's arrow; `RECAP_BREAK=arrows` red) on
  both stands. Never seen on a GPU, never played.
- **The end card opens the run's replay (10-14; the maintainer, of the verdict card: "would be nice to be able to open the replay immediately here").** The card
  (`ui/menu.ts debrief()`: the verdict card, the down card, a lesson's) has a **Replay** button after its own buttons and before Copy summary. One press does
  Shift+F7's work (`ui/replaykey.ts openViewer('card')`, the key goes the same road now): the last stretch saved to
  the shelf, then THIS WINDOW sent to `replay.html#id=<n>` (no tab: 'One window' in replays.md), staying on the game if nothing was shelved. Armed half a second after the card is up, like
  its neighbours. NO button where nothing could be viewed (`ReplayKey.canView`: the one-file build has no viewer page; `?noreplay` or the stored switch has no
  buffer) — the page hands the card its opener through `MenuDeps.replay`, asked each time the card is drawn. No switch (a button, not a rule); nothing in the
  game reads it. `__replay.view()` / `__replay.canView` are the hook's. CHECK `probes.ts cardreplay` (family `replay`; `CARDREPLAY_BREAK=1` = the browser hands
  back no tab: must fail), 4 lines, both stands. What the replay holds is the buffer's last stretch of PLAY, as F7's (`REPLAY.seconds`, 300: the card
  up stops the clock, so waiting on it loses nothing): a run longer than that has lost its start and opens at the buffer's own, and after restarts the file
  still holds the earlier attempts but the viewer opens it at the run the card is for (the last restart's mark, `replay/format.ts lastRun`, for a file whose
  why is 'card': the F7 bullet in replays.md). NEVER SEEN ON A GPU; the page look is on the software adapter.
- **The end card's Replay plays the run in the engine (10-15; the maintainer: "clicking on replay after a mission should start in engine").** The card's press
  (`ReplayKey.openViewer('card')`) still saves the last stretch, and the window (no new tab since 10-15: 'One window' in replays.md) now goes to the GAME PAGE on the file's run
  tape (`replaykey.ts engineUrl`: `index.html?playtape=<shelf id>`, the address the map viewer's 'Play in engine' has), with `&at=<step>` at the first steps of
  the LAST try so it opens on the run the card is about (the tries before it are run through unseen first; one try = from the top). That step is read off both
  halves of the file: the trace's last restart mark (`format.ts lastRun`, the map viewer's rule for a card's file; it also has a restart made INSIDE a step —
  Shift+Enter, Enter once down — which is on no tape row) and the tape's last restart row + 1 (one older than the trace's 300 s). A file with NO TAPE TO PLAY goes
  to the map viewer as before, and so does Shift+F7: `rerun` absent or `{dead}` — a checkpoint loaded (the down card's default 'Restart from checkpoint' on the
  solo and quiet missions), a scenario started or quit from a menu inside the visit, an F10 recording, past `RUNTAPE.minutes`. No switch (a button's address;
  nothing a step reads). The engine page opens standing on its big ▶ (the player's own: a page just loaded has no gesture for sound), so it is one more press to roll; its
  bar's 'Map' goes to the 2D viewer. Check: `cardreplay` (family `replay`; a real tape off the key's own buffer with a restart in it; `CARDREPLAY_BREAK=1`), both
  stands. Never seen on a GPU.
- **Copy and paste for loadouts** (2026-10-15; the maintainer: "we should have copy and paste for loadouts"). On Team & kit, **C** copies the picked
  man's kit and **V** pastes the copied kit onto the picked man, from any row of the screen (Ctrl / Cmd with either is the same press while no text on the
  page is selected and no text field has the keys; the two go by the LETTER on the key, `kitCard.ts clipKey`, the screen's other keys by their place);
  the mouse has two buttons, Copy and Paste, at the end of the card's sets line (`.kc-clip`, not `.kc-set`: the sets are still three). The desk
  holds the one copied card for the page's life (`rust/game_run/src/desk.rs`: nothing in the game reads it, so it is no knob table and no journal row). THE
  PASTE IS `src/game/kit.ts kitPaste(i, card)` (`kitPastePeek` = the same, nothing written): every ROW of the man's own card takes the copied card's
  value where his row offers it, a count over his row's most comes down to it, a row that cannot show the copied value at all keeps what it had (a
  teammate's wall charge onto you with your own charges off: your big slot is left alone; your cans as one of four splits with the pouches rule off), a field his card has no row for is never
  written, and `cardWhy` is asked as for a set: so a pasted card is one his own tiles could have made, and a paste the squad has no room for (a table
  with a squad limit) is refused with the reason under the rows. HIS COLOURS STAY HIS OWN (suit and trim, as under a squad kit: the trim is how the men
  are told apart). ONE write of his card and of the store per paste (the door picture's one `kit` message), none when nothing would move; the highlight
  is not moved by the keys. SHOWN, NOT TOLD: Copy is lit on every card that WEARS the copied kit (`kitWears`: a paste there would move no row, so Paste
  is dim), the tiles a paste moved flash once (`.kc-row.pasted`), and the roster marks each man who wears it (`.lo-slot.clip`, two sheets in the chip's
  corner). The one start page's key line gains "C copy · V paste" while there are cards (`ui/startpage.ts say`, asked again at every paint: a mission's
  own issue takes the cards away after the screen is up). A Ctrl / Cmd press is not kept for its release (`eaten`): macOS sends no keyup for a key let
  go under Cmd. CHECKS: `bun run tools/qa/kitpaste-check.ts` (`probes.ts kitpaste`, also in `kit`; it sets the pouches rule, the big slot and your own
  charges / wand / cameras each way itself, so it asks the same on both stands: 8,000 pastes over seeded squads the tiles made, each a legal squad and a
  card of the man's own tiles, plus named cases; `KITPASTE_BREAK=1` = the copied card written unfitted, 13 lines fail on both stands) and
  `PORT=<dev server> node tools/qa/kitpaste-page.cjs` (the real page in headless Chromium, no GPU needed: the keys, the button, Ctrl+C / Ctrl+V, one
  store write a paste, the flash, the marks, a mission that issues its own kit). WITH THE SHIELD GONE FROM THE
  GAME (10-18) a stored card that says shield reads as a card without one, and a paste hands over what the card SHOWS: the empty big
  slot, never a shield. A
  DEPLOY WITH A PASTED KIT, end to end: `PORT=<dev server> node tools/qa/kitpaste-deploy-page.cjs` (SLOW, about eight minutes: the real page on the software
  adapter; C, V, the Deploy button's page load, then the mission's own kit table — four staged, the pasted man's card and the squad's count; first run
  10-15 on the build of that day, 6 lines pass; `KITPASTE_BREAK=1` = nothing pasted, 3 lines fail). It reads the table, not the men: nobody has looked at the staged man's
  hammer. The Cmd key of a Mac (no keyup for the letter) and a Dvorak board are two lines of the page check, made of key events built by hand: no
  such keyboard has pressed them. Never seen on a GPU, and never with the door picture up behind the tiles; nobody has played it.
- **What a mission hands you shows on your card** (2026-10-16; a claim from the stealth goal's mission wand). A quiet mission played under the fog hands Sam
  its sticky cameras for the visit (`scenarioRun.ts` → `stickyVisitSet`, to a card with the rifle) and, where `quietfog.ts QUIETFOG.wand` is up (`?quietwand=0` off), an
  optical wand with them; the Team & kit card read his CARD alone and said 'none' beside both. Now `CardRow.lent` (how many the mission
  hands him; `kit/card.rs card_row`) marks a tile `lent` where that is more than his card carries: the thing drawn lit, the mission's count and `LENT_WORD` ('mission’s') where the
  value is (his own count stays in front where he carries some: '1 · mission’s 2', so an arrow pressed still moves a number he can see), the row's line for
  a man who has it, no pouch rate on a tile the mission alone fills, the reason on the tile (`LENT_WHY`). The desk answers it from the link
  the screen would deploy, by the game's own rule (`scenario_run quiet_cams_of_link`; the fog lever and
  the Sticky cameras row are on that screen: moved, the desk is told and the card is drawn again); you only, quiet missions only, never a lesson, no cameras beside a pistol.
  `worn()` lays the same kit over the card it hands the drawn man, so the figure beside the card is dressed in what he will deploy with. NOTHING OF THE
  CARD OR THE POUCHES IS WRITTEN: whether lent kit stays free is the maintainer's open call on the stealth page, and a card that carries its own wand still
  pays its pouch. CHECK: `bun run tools/qa/kitlent-check.ts` (`probes.ts kitlent`, also in `kit`; a headless DOM, it sets its
  switches itself so it asks the same on both stands; `KITLENT_BREAK=1` must fail), 10 lines, one of them THE GAME'S OWN WORD: the screen's own link
  started in a headless game, the cameras (and the wand) the tiles promised are what is on him. The real page: `PORT=<port> node tools/qa/kitlent-page.cjs` (the
  software adapter, about four minutes a page: the screen as it opens, the fog kept off). Not on the roster's chips or the one-line card. Never seen
  on a GPU; nobody has played it.

- **A stealth mission's kit is what your card starts at there** (2026-10-20; the maintainer on Team & kit of the solo night: "not sure it makes sense to
  have "mission's" for equipment. that should just be defaults. and if we want to have required equipment, that should just be a constraint on the stuff
  you pick." and "quiet default also looks jank"). The build before HANDED a quiet mission's sponge rounds, sticky cameras and optical wand to you over
  your card at the staging, for no pouch, and the tiles above read 'mission's': a second kind of gear no row could change. Now NOTHING IS HANDED OVER A
  CARD. On a stealth mission (a quiet mission; the night alone) you wear a STEALTH CARD of your own, kept apart from the loud one, and what it is issued as
  is the mission's kit (`kit.ts stealthIssue`): the launcher's five sponge rounds, the cameras the link carries, the wand, nothing loud (no frag, no HE),
  and a smoke / stun canister in turn where a pouch is left (no fog: five rounds, two smoke, one stun). Every row is yours to change and is priced like any
  other; the sponge rounds ride FIVE TO A POUCH (`slots.json` sponge `holds: 5`, read only under the switch), so the kit as issued fills the four pouches:
  1 + 2 cameras + the wand. 'As issued' on that screen puts the mission's kit back. A stealth card still as issued is not stored and FOLLOWS the mission
  (the fog lever moved on the screen, another link's cameras); one changed by hand is kept (`twopointsix.kit` gains `stealth`, the card; `members[0]` stays
  the loud card) and is worn on every stealth mission while it is a legal fill. `kitMission(hand)` says which mission the table's first card is for: the
  loadout screen as its card is mounted and as the fog lever moves (`loadout.ts syncHand`), null as it closes; a staging puts the run's card first for the
  length of its own call and puts the table back (`forRun`), so nothing read off the table between stagings moves with the last run played. THE QUIET KIT'S
  CARD FOR YOU is that same kit under the switch (the table's, a pistol alone with three smoke and a frag, predates the rounds, the cameras and the wand),
  so on a stealth mission the row of kits does not list Quiet beside the same card as issued; the teammates' Quiet cards are the table's. NOT COVERED: a
  mission's own issue and a lesson's page wear the issue cards (nobody's card: a lesson's page with the hand-overs as they were, a mission's own card for
  you with none since `ISSUE.whole`, `docs/agents/missions.md`); the tile seam (`lent`, `LENT_WORD`,
  the lent tile) is kept and inert under the switch, and is the page as it was under `?nokitstealth`. REQUIRED EQUIPMENT: no mission requires a thing today;
  one that did would be a refusal with its reason (`kitRowCan`), never a second kind of gear. SWITCH: `KITDEAL.stealth` (a key of the registered table,
  journaled; a recording from before the key replays with it off), OFF in the module and on the frozen stand (`FROZEN_HOLDS` row, `TPS_KITSTEALTH=1`; reads
  only under the card as a tree), ON in the page (`?nokitstealth`; a box in the debugger). Off = the build before: the hand-overs, the tiles that say
  whose, a sponge round a pouch each. CHECK: `bun run tools/qa/probes.ts kitstealth` on BOTH stands (13 lines: off to the count, the card, a reload, the follow, the
  store, As issued, the Quiet kit, four staged runs, the journal); `KITSTEALTH_BREAK=1` fails 6 on each. The real page: `PORT=<port> node
  tools/qa/kitstealth-page.cjs` (`STEALTH_BREAK=1` must fail). Probes written against the hand-overs run with the key held off by the runner
  (`STEALTH_AS_IT_WAS` in `tools/qa/holds.ts`; `kitanim-midswap` beside it, whose night alone throws a frag; `lesson-check.ts` holds it for its kit block, whose 'same start off a lesson's page' is a quiet run alone — a lesson's page under a kept stealth card is kitstealth's own line). A run tape carries the kept stealth card beside the cards (`kit.stealth`). Never seen on a GPU; nobody has played
  it; whether five rounds, two cameras and a wand is the right kit to start a quiet night with is the maintainer's to say.

- **A suppressed gun is drawn and named as one on the kit card** (2026-10-17; the maintainer on the Team & kit screen of a quiet mission: "quiet missions
  should have different gear screen stuff i think. be creative!"; his shot showed the quiet set's guns tile reading 'GLOCK 19 ONLY' beside a bare pistol).
  THIS IS ONE PART OF THAT ASK, the guns tile's; nothing in it asks whether the mission is quiet, and the rest of the ask belongs to the screen's picture
  and panel. Both of your guns wear a suppressor (`weapons.ts suppressed`, drawn so in the game), and nothing drawn on the card showed it. Now
  `game/kit.ts kitGunCan(i, m)` (the gun in his hands is a suppressed one: both of yours whatever the card's `suppressed` field holds, a teammate's by his
  Suppressor row, never the shotgun) puts the can on the gun's drawing (`ui/kitIcons.ts GUN_CAN`, `kitIcon(field, value, can)` / `gunIcon(weapon, can)`:
  the tube on the muzzle, one dark band where it screws on) in the three places a gun is drawn, all asked through `ui/kitCard.ts gunCanOf`: the guns
  tile, the roster's chip (`.lo-ics`, the drawing the one start page shows) and the roster's gun over its words (`.lo-gun`, the loadout's screen of its own). Your
  pistol carried alone reads 'Suppressed Glock 19' on its tile (`youGunsCan`); the rifle's words stay 'Mk 18 + Glock 19' (two guns on one tile, and the
  floor hears the rifle as gunfire all the same); a teammate's tile keeps his row's word, the Suppressor row beside it says the rest. `kitWords` and
  `YOU_GUNS` are NOT changed (the roster's line, the card's line under the tiles and several checks read them), so on that screen those two
  lines still say 'Glock 19 only' under a tile that says 'Suppressed Glock 19'; the one start page shows neither line. CHECK: `bun run tools/qa/kitcan-check.ts` (`probes.ts kitcan`, also in `kit`; a
  headless DOM, it sets its switches itself so it asks the same on both stands; `KITCAN_BREAK=1` must fail), 7 lines. Looked at on the real page on the
  software adapter at 1280 x 720 only (the tile's words fit uncut, no page error); under 1180 wide the one page hides the tile's drawing and only the
  roster's chip carries the can, not looked at; that screen's narrower tile not measured. THE MAINTAINER'S CALL, not taken: the quiet set still
  carries a frag (`setCard` builds it on the issued card). Never seen on a GPU; nobody has played it.

- **The squad's kit is dealt over the men who come** (2026-10-18; the maintainer, over Team & kit on The pass with a team of four as issued: "as
  issued still has blue team pretty empty"). The issued kit (`src/game/kit.ts ISSUED`) and every named squad kit (`src/game/slots.json` loadouts) are
  written down as the FIRST TWO teammates carry them, so that a pair, a three and a four add up the same; with three or four coming the others stood
  on the roster with a gun and `pouches 0 / 4`. No level deals a kit: the table is one for every mission (a level changes it only by
  `scenarios.issue.members`, today the long hall's alone, or by `scenarios.roomClear.wallCharges`), so the fault was on every squad mission at +3 and
  +4 under every kit. ON: the cards are WORN shared out (`kitWorn(n)`, `kitShown(i)`): while the teammates' counted small things (door charges, stun,
  frags, smoke) are the issue's or a named kit's, `deal()` hands them one at a time to the man of those who come with the fewest, a man without that
  thing first, Red and Blue by turns. As issued, of four: Red One 1 stun + 1 charge, Red Two 1 stun + 1 frag, Blue One 1 charge + 1 frag, Blue Two 1
  stun, so each pair has a stun and a charge of its own. THE TABLE'S CARDS ARE NEVER REWRITTEN FOR IT: `KIT.members` and the store stay in the table's
  layout; the deal is what the roster, a man's card and the drawn man show, and the card a staged man carries (`kitStage`, `kitOf`). The squad's count
  is a sum over the men who come (`kitPool`), so it does not move at any size and a run is the run it was by digest; nothing in the simulation reads
  which man holds a counted thing. A CHANGE BY HAND is made to the card as worn (`onWorn`): a counted thing changed on a teammate makes the worn cards
  the player's own, kept as they stand at every size (so a smaller team then leaves behind what its men at home hold, as hand-made cards always did);
  any other change (a gun, a look, a big thing, your own card) is written onto the table's cards and the layout stays, unless it costs a pouch that
  man's card in the table has not got (a wand on Red One, full as issued): then the worn cards are kept too, so no stored card is ever over its
  pouches; a refused change and a peek write nothing; men put back on their shares are the named kit again (`dealSettle`, only into a layout their
  other things leave room for). Cards kept as the player's own are marked so (`dealOwn`, saved as `own` in the store, dropped by a squad kit put on by
  name or a reset): they are worn as written even where they come to read as a kit's table (Blue One's last canister given up on purpose), and the
  squad kit row then names no kit. The Team & kit picture is sent the worn cards and wears them as written (`ui/kitView.ts`, `kitstage/cards.rs`). A man's own set (As issued, Quiet, Loud) at three or four is HIS
  SHARE of that kit, so Loud on a man of four may add no stun to him. A mission's own issue that changes a teammate's counts is not a named kit and is
  worn as its file writes it. A TOOL EACH PAIR (`KITDEAL.more`, 10-18; the maintainer's pick on the card that asked whether a four should carry more
  as issued: "A tool each pair"): a team of three or four is LENT a big tool each pair, Red Two the breaching shotgun and Blue Two the hammer, in a
  big slot that stands empty; pouches as the deal gave them. THE PAIR'S SECOND MAN CARRIES IT, NOT ITS FIRST (so of three, where Blue One comes
  alone, only Red Two is lent one). The card the maintainer picked on said Red One and Blue One, and that was built first and MEASURED WORSE THAN NO
  TOOL (the shipped stand, the office room clear, the blue pair told 'open and clear' at the manager's door with red stacked on it, seeds 1 to 8):
  with the hammer on Blue One the pair lost 10 men of 16 and was wiped twice, where with no tool it lost 4 and with the hammer on Blue Two it lost 2
  and went in every time. Why: `doortools.ts toGun` sends the order's lead to the lock when he carries the tool, the door goes with the hammer still
  in his hands, and the second man waits on the lead for the threat in the doorway ('until the first man has it'), so nobody fires for a second.
  With the tool on the second man the lead keeps his gun up and has that threat down inside 0.3 s. STILL OPEN, the door tools' own: a carrier who
  is also the order's lead (the tool put on him by hand, or the second man standing on the latch jamb) goes in as before; no rule yet hands the lead
  to his mate. THE TOOLS ARE WORN, NEVER WRITTEN (`lend`, applied where the cards are worn: `kitWorn`,
  `kitIssuedShown`, a man's own As issued set): no card of the table or the store ever holds one, not after another row is changed, not after a
  counted thing is (`onWorn` writes a slot empty where it holds the very tool the deal would lend back), so one or two teammates come as they did and
  the key off is the deal alone at once. A big thing put on that man by hand is his, written, at every size. HIS SLOT EMPTIED BY HAND IS MARKED ON
  HIS CARD (`MemberKit.bare`, kept in the store, dropped when something is put in the slot): the lent tool stepped off, or a thing put on by hand and
  cleared again, leaves the slot empty until he is given something, and moves nothing a smaller team carries (a first pass kept it by making the
  cards the player's own, which halved a pair's grenades and wrote the other man's tool; and a cleared slot showed the lent tool at once). WHICH CARDS
  ARE LENT (`lendable`): the teammates' counted things are the issue's, or the cards are the player's own and no named kit's. NO NAMED SQUAD KIT IS
  LENT ONE (each says what its men carry, and the Quiet kit's men would pay for a loud tool they keep slung), cards that leave a named kit by a
  counted change keep its empty slots (the same mark, dropped again when they settle back into it), and a mission's issue that writes its own counts
  is worn as its file writes it; a mission's plain issue lends them (the long hall). `dealSettle` never settles into a layout that would lend a man
  a tool or stop lending him one. What it gives the four: the big slot's row on a door's tools (H) and a lock taken by the shotgun where the lead
  booted it. NOBODY COMES WITH A GUN ALONE, the same key: a man the deal leaves with nothing counted is handed one stun canister more (while his
  pouches and the table's most-a-man and most-a-squad allow), which `kitPool` (it now reads the cards as worn) counts: only the Quiet kit of four has
  such a man, so it carries four stun where it carried three. THAT ONE CANISTER IS THE ONLY THING OF THE DEAL THAT MOVES THE SQUAD'S COUNT. SWITCH OF
  ITS OWN: `KITDEAL.more`, OFF in the module and on the frozen stand (a `FROZEN_HOLDS` row, `TPS_KITMORE=1`; reads only with the deal and the big
  slot on), ON in the page and on the shipped stand (`?nokitmore`; the debugger's Big slot section, 'a tool each pair as issued'); a recording
  whose KITDEAL table has no such key replays with it off (`journal.ts applyKnobs`). Its check is the `kitdeal-more` family and the lines it adds to
  the run, the screen and the stands (`KITMORE_BREAK=1` fails 13 on both stands); the older families hold the key off and ask of the deal alone, but
  for the seeded walk, which now walks the whole rule and reads the big row after every press. MEASURED (the shipped stand, the office room clear,
  four as issued, the manager's door locked against Sam, seeds 8, 9, 10, the same on each): 'open and clear' takes the lock in 14.6 s with the
  shotgun drawn once and no boot, where the key off it took the same 14.6 s with one boot; the big slot's own row takes it in 14.2 s where the key
  off there is no such row; both door charges are still carried either way. (So the tool buys no time at a door a boot opens: it buys the doors a
  boot is refused at, and a lead who goes in with his gun up.) Not measured as issued: a door locked on the face it opens toward (the boot's
  refusal, the tools' answer: `bigslot-shut` asks it with cards set by hand). PROBES WRITTEN FOR A FOUR WITH EMPTY BIG SLOTS are held to that on
  every stand by the runner (`tools/qa/holds.ts MORE_AS_IT_WAS`: the word and the table, unless whoever runs the tool set the word himself), as the deal's
  own older probes are: `frameholds-tools`, `doortest`, `silent-pick`, `silent-boot` (a locked or wedged door such a four boots is one the tool
  takes), the ones that read a door's tools page row for row against the pair's or count its rows by their digits (`doormenu-rows`,
  `thresholdmenu-rows` (deleted 11-06), `shutdoor-menu`, `silent-tools`), and `kitpaste` (its squads' big slots
  are the table's; a paste over a lent tool is a `kitdeal-more` line); the page as it opens is walked, in every scenario, by `orderkeys-walk`. THE PASTE'S TWO PAGE CHECKS READ THE RULE (10-19): `tools/qa/kitpaste-page.cjs` and the slow `kitpaste-deploy-page.cjs` paste a hammer card onto Blue Two of four, the one man the deal lends that hammer, so the store holds his counts and big 'none', his Big slot tile and his card's line read the hammer before and after, and three tiles flash; the first runs its opening part again under `?nokitmore`, where the hammer is written and four tiles flash (both were written to the store's hammer and went red on the page as it opens; headless Chromium, never seen on a GPU). ONE THING THE PLAYER NOW MEETS ON EVERY DOOR, OF FOUR: the big slot's row is on H, a key of its own beside G, since the
  page's five digits are taken; `orderkeys` knows the key now (it never met the row: nobody was issued a tool). The maintainer's call whether H
  is near enough to the left hand.
  STILL LEFT: a four as issued fills 2, 2, 2 and 1 of its sixteen pouches. SWITCH: `KITDEAL.on`, OFF in the module and on the frozen stand (a `FROZEN_HOLDS` row, `TPS_KITDEAL=1`, reads only with `SLOTS=on`), ON in the
  page and on the shipped stand (`?nokitdeal`; the debugger's Big slot section, 'squad kit dealt over who comes'); in the knob registry and
  `JOURNAL_KNOBS`, and a recording from before the table replays with it off. Off at any moment = every card shown and staged as the table writes it.
  CHECK: `bun run tools/qa/kitdeal-check.ts` (`probes.ts kitdeal`, also in `kit`; 39 lines on both stands, `KITDEAL_BREAK=1` fails 3 on both: nobody
  comes bare, a pair a door, the roster of four); the real page: `PORT=<port> node tools/qa/kitdeal-page.cjs` (with `more`: the roster's lines
  name the two tools, the staged four have the big row, `?nokitmore` is the deal alone). The older card probes (`kit-*`,
  `pouch-*`, `squadsmoke-*`, `bigslot-card`, `wand-mate`) press a pouch too many on the table's own layout at four, so `tools/qa/holds.ts DEAL_AS_IT_WAS`
  holds the deal off for them on every stand; the worn cards are this check's, with 3000 seeded presses of its own. `bigslot-check.ts` run BY ITS
  OWN FILE takes the same holds itself (the deal for its card part, the two gun rows for every part: it was red on the shipped stand alone while
  green inside the gate), and now runs its `weight` part too. A MISSION'S ISSUE IS NOBODY'S OWN LAYOUT: `asIssued` holds the player's own-layout
  mark off for the staging, so cards changed by hand never stage a mission's issue undealt (the check's hall line, red without it). LISTING: `bun run
  tools/qa/kit-audit.ts` (`--deal=off` for the cards before, `--more=off` for the deal alone) prints every kit at every size and one row a mission. MEASURED: The pass, four, 240
  frames, one seed: the same digest on as off; a pair and one man count what they counted; the long hall, whose block names nobody since the shield left the game, counts the same;
  60,000 seeded presses (eight seeds, sizes stepped) left no stored or worn card outside the rule on either stand. Never seen on a GPU; nobody has
  played it.
- **The gear screen is a tree, the same for every man** (`src/game/kittree.ts KITTREE`, `kit.ts buildRowsTree`, `ui/kitCard.ts`, `kit/card.rs`; the
  maintainer on Team & kit, 10-19: "have some way of distinguishing different guns (some stats?), attachments like silencer should be per gun", "should
  be symmetric between self and teammates (e.g. 40mm should work for squad)", "sponge should show up, should only be part of stealth builds by default",
  "ideally rifle attachments should be hierarchical under the rifle", "weird that we have both per person defaults (as issued, quiet set, etc.) and
  then squad kits too"). With the switch on a card is ONE list of rows for you and a teammate. Each gun is a box: its tile, FOUR BARS of what it is
  good at (Reach, Volume, Handling, Quiet: `kit/card.rs gun_stats` read off its weapon's row, no number shown), and under it, set in off a rule, what
  rides on it (`KitRow.under`): its own suppressor (`suppressed` the primary's, `sideCan` the secondary's, new), the weapon light, and on a gun with
  the tube (`KitRow.has`) sponge rounds (`rings`, a row for the first time), 40 mm grenades and sticky cameras. A suppressor row on a gun that has no
  choice reads what it is and has no arrows (`KitRow.fixed`), and its flag is put back on when the gun leaves the card (`homeCans`): a paste or a kit
  never carries an 'off' nobody could see. YOUR card has the suppressor, light and goggles rows a teammate's always had, and you deploy with them: a
  suppressor off is the same gun with a bare muzzle (the pick's own bare row, `gunslots.ts PICKS`: the row a teammate's card makes of it), no light is
  a dead L, no goggles a dead N and no goggles drawn (`kitYouLight`, `kitYouNvg`, asked every frame, so a checkpoint cannot bring
  one back). SPONGE ROUNDS are a count, none as issued; a quiet or solo mission hands you `KITTREE.lend` (5) over your card as it hands its cameras
  (`kitRingsLent`; the tile reads `5 · mission's`), so a stealth run starts with the five every launcher used to carry and a loud one with none. ONE
  PLACE FOR NAMED KITS (the maintainer's tap, 'Squad row only'): the card's row of sets is one `Reset` beside Copy and Paste, and the screen's row of
  kits (labelled `Kit`) is shown and live at every team size; alone it lists the kits that put a different card on YOU and a press writes your card
  only (`kitLoadout(id, only)`, `kitLoadouts(only)`, `kitLoadoutOf(only)`). A stored card from before the tree: yours takes its suppressors, light and
  goggles as issued, a teammate's one Suppressor row is both his guns'; a card written under the tree keeps its new fields through a visit with the
  switch off. A TEAMMATE'S 40 mm: his card has the `40 mm grenades` tile under his primary where yours is, none as issued and one at
  most (`SHELLS_MATE_MAX`; `slots.json shell40` names a teammate with a most of 1, a pouch), only where the round can be fired: the card a tree and
  the order in the build (`mateShells` = the tree and `squad40.ts squad40Here`; the order is X on a spot of floor, G, 3, switch `SQUAD40.on`,
  `?nosquad40`). A primary with nothing under its barrel takes the tile and the round off the card (`homeShells`), his line under his name says
  `1 × 40 mm`, and `kitStage` hands the round to `squad40Stage` as it always did. No sponge round and no sticky camera for a teammate.
  SWITCH: `KITTREE.on`, OFF in the module and
  on the frozen stand (a `FROZEN_HOLDS` row, `TPS_KITTREE=1`; it reads only with `GUNSLOTS` and `BIGSLOT` up), ON in the page and on the shipped
  stand (`?nokittree`); in the knob registry and `JOURNAL_KNOBS`, and a recording from before the table replays with it off. Off = the card of the
  build before, row for row: one Suppressor row for both a teammate's guns, your guns always suppressed, five sponge rounds with every launcher, the
  two rows of sets. CHECK: `bun run tools/qa/kittree-check.ts` (`probes.ts kittree`: `kittree-rows`, `-guns`, `-sponge`, `-off`, `-mate40`; 31 lines on both
  stands, `KITTREE_BREAK=1` fails 18 on the frozen stand and 19 on the shipped one); a deploy off the card,
  `tools/qa/kittree-deploy-page.cjs` (two mission boots on the software adapter, slow). The older card probes that read a row's place (`wand-kit`) hold the tree off on every stand (`tools/qa/holds.ts ROWS_BEFORE_THE_TREE`), as `gunslots-check.ts` does in its own `raise`, and `kitpaste-page.cjs` counts the one Reset on a tree card. A MAN'S LINE under his
  name (`kitWords`) names his guns by their picks under the tree, with `bare` after one whose suppressor the card has taken off. LEFT, the
  maintainer's calls: a sponge round costs a pouch and on a quiet run buys nothing over the lent five; the Quiet kit names no sponge; which four
  bars, and their scale (untuned, nobody has compared guns by them in play); a bare Mk 18 in a teammate's hands is the old unsuppressed row, in yours
  the suppressed row with a bare report; the empty sponge load is still a stop on the launcher's cycle. NOT DONE: the player's suppressor is still
  DRAWN when the card has it off (the guns' look), the Team & kit picture does not know `sideCan` / `rings`, a bare gun weighs
  what it did suppressed and sponge rounds weigh nothing (`kitload`). Never seen on a GPU; nobody has played it.
- **The dense kit card: guns side by side, a pop-up per gun, the kit in one line of tiles** (2026-10-20; `src/ui/kitDense.ts`, `ui/kitCard.ts`
  `MemberCardOpts.dense`; the maintainer on Team & kit beside a loadout card of Door Kickers 2: "this menu is pretty unwieldy, door kickers is a lot
  better. we shouldn't copy it but maybe we have a popup menu per gun, more information density, etc."). The picked man's card on the one starting page
  is the SAME card (the tree's rows, values, refusals, lent and idle tiles, copy and paste: nothing of the kit's rules moved) in another lay-out:
  one line of pouches and the two weight bars; THE TWO GUNS as tiles side by side (`.kc-guns`), each its drawing large, its name, the four bars, the
  mark that the tile opens beside the slot's name (`.kc-label::after`: the gun's own line is the sound mark's on a quiet mission), and
  what rides on it as CHIPS in the tile's corner (`.kc-chips`: lit where fitted or carried, a count on a round, faint where not, `lent` where the
  mission hands it); THE REST OF THE KIT as one line of small tiles (`.kc-gear`: the thing drawn, how many, its name; the arrows stand either side of the
  drawing on the tile the mouse or the keys are on); the colours and Reset · Copy · Paste a strip under it (`.kc-look`). A GUN'S TILE OPENS ITS OWN
  POP-UP (`.kc-pop[data-pop]`, under both tiles, over the kit): the guns the row offers side by side (`.kc-pick`), each with its drawing, name and the
  same four bars under one another, the one on the card lit, one the pouches have no room for dimmed with the reason as its tooltip; under them the
  gun's fittings and rounds, the tree's own tiles (`.kc-kids`; a tile with words beside its count, `lent` or `idle`, is two tiles wide). A press on a gun puts it on the card at once through `kit.ts kitPaste` (so it is fitted
  and refused as a paste is, and never steps through the guns between). SHUT BY: a press anywhere off it (a `pointerdown`: the kit figure eats the mouse event), Esc, Enter on the gun, the keys leaving the
  card. IT NEVER COVERS THE FOOT: `MemberCardOpts.floor` is the top of the host's foot (the loadout's buttons), and a pop-up that would pass it is
  drawn that much higher, over the foot of the guns' tiles (`kitCard.ts fit`, again when the window is sized or the page scrolls) — the maintainer's own window is about
  1000 x 673, where a Mk 18's five things are two lines. THE KEYS are the card's (W S move, A D change): shut, W S walk the tiles as they lie (guns, kit, colours, sets) and A D on a gun's tile is the
  next gun; Enter on a gun's tile opens its pop-up, where W S walk the gun and its things and stop at the ends, A D change. A gun's refusal is said on
  the card's own line, or in the pop-up while it is open (`.kc-pop-note`, which also takes a set's or a paste's refusal: the pop-up lies over the
  card's line). The chips and the tiles ask ONE rule what the mission makes of a row (`kitCard.ts given`: idle, handed, lent). A QUIET mission's
  wider cursor (`startpage.ts QUIET_CSS`) is drawn inside a dense tile, so no tile moves under the keys. The whole screen fits with nothing scrolled from 1200 x 675 up; a short
  window lowers the guns' tiles, one with room to spare (900 px tall and 1500 wide) draws everything larger. `MemberCard.pop()` / `.open(slot)` for a
  host or a check. CHECK: the real page,
  `PORT=<port> node tools/qa/kitdense-page.cjs [outDir]` (20 lines: the lay-out, the chips, the pop-up by the mouse and by the keys alone, a gun put on
  and kept, the walk to Deploy, four window sizes for you and a teammate, a quiet mission's lent rounds, a quiet mission at 1000 x 673 and
  1200 x 675 — no tile moves under the keys, the pop-up ends over the foot —, the largest text size). `kitpaste-page.cjs` takes the pouches on a line of their own as clear of Copy and Paste; `bigslot-page.cjs` reads the big slot as the last of the
  KIT's rows on a dense card (the colours' strip follows it); LEFT, the maintainer's calls: W S walk a line of tiles
  that lies left to right (the card's standing keys; a walk by the four directions would be a change of the screen's keys); a count's tile has no
  pop-up, only its arrows; in a short window the pop-up lies over the lower part of the guns' tiles. Seen only in
  headless Chromium on the software adapter: never on a GPU, nobody has played it.
- **The kit step says the job, and a tutorial mission teaches planning on it** (2026-10-22, the walkthrough moved onto a tutorial mission 10-23; `src/ui/kitBrief.ts KITBRIEF`, `src/ui/kitGuide.ts KITGUIDE`; the maintainer: "we should
  teach how to pick squad layouts! so maybe a tutorial on the gear page for a mission (that's skippable). honestly it doesn't make a ton of sense right
  now since you don't know anything about the objective or map when picking kit"). TWO THINGS, EACH ITS OWN SWITCH, both UI only (nothing under
  `src/game` reads either, no knob table, no journal key, no `FROZEN_HOLDS` row: they are mounted by `ui/startpage.ts` alone, which no headless stand
  opens; the one game rule beside them, a lesson's kit that can be picked, is `ISSUE.picked`). **THE BRIEF** (`?kitbrief=0` = the step's head as it was, the name alone): Team & kit keeps a band across its top — the mission list's own map
  sheet at the left with the way in marked (`.msheet > .mmap`; a click or M makes it large over the step with the name and the job under it, `.kb-cap`;
  again, Esc or Enter puts it back; the foot's key line gains 'M map' where there is a map to open), and beside it the name, what is set (`.mcsum`), the
  facts and the job (`.mblurb`), the list's own elements shown by this file's rules and no new sentence. It adds to the facts row WHAT THE PLACE ASKS OF
  A KIT, counted off the level's file by the plan's own fetch AS THE MISSION IS STAGED (`ui/missions.ts placeNeeds`, `Plan.needs`): the doors found
  locked, those of them that are steel, a rescue's hostages — a rescue's side door (`scenarios.rescue.side.door`) is unlocked by its staging and is not
  counted; the bundled office's rescue is read from its own file, which is in the build. The cells are `.mfact.kb`, drawn on this step only; a count is a digit
  to five and a word past it (`countWords`), and the list's Defenders cell was drawn AGAIN by this file with its count in words ('eleven'; since 10-23 not drawn), the list's
  own cell hidden on this step. IN A SHORT WINDOW (800 px tall and under: the maintainer's own is about 1000 x 673) THE BAND IS NOT A PIXEL TALLER THAN
  THE NAME ALONE WAS: the map a small mark, the name, the facts under it, and no job sentence (no text is cut to fit: `textsize-page` holds that) — the
  job is read on the large sheet. `kitdense-page`'s bars (a gun's pop-up ends over the foot at 1317 x 775, nothing scrolls at 1200 x 675) are why.
  In a window under 900 px wide, or where the plan did not come there is no map on the step, only the words, and M is not taken.
  **THE WALKTHROUGH IS A TUTORIAL MISSION'S OWN PLAN** (2026-10-23; `src/ui/kitPlan.ts KIT_PLANS`, the engine `src/ui/kitGuide.ts`; the maintainer, of
  the walkthrough that first came up on every mission: "this tutorial is good but we should attach it to a tutorial mission (and make it bespoke and
  explain how to plan), not for the regular ones"). A REGULAR MISSION PLAYS NONE and has no word for one. The walkthrough is a SCRIPT (`GuideScript`:
  beats as data — the row a card sits by, its words for the step as it stands, what counts as done, the key it is remembered under, the word on the
  head, `auto` = played unasked), and the one played unasked is the plan written for one mission, the tutorial's squad mission ('A harder fight',
  `tut-squad`): the first the player walks into with the four, after the mission that had him throw each grenade by hand, on a street with what a kit
  is chosen for. Its row is the one lesson NOT played with the kit as issued (`game/kit.ts ISSUE.picked`, `docs/agents/missions.md`), so its cards are
  on the step. THE BEATS, each a small card beside ONE lit row, the rest of the step dimmed and still live, the keys put on the lit row, gone when its
  thing is DONE or on its 'Next': **Plan it first** (the map opened and put back — large, it names the street's rooms, the machine gun and the locked
  hall as words pinned on the sheet, `.kb-pin`, placed through the sheet's own projection: `planArt.ts sheetView`, `missions.ts sheetSpots`; and the
  line under it says what to look for) · **Pack for the rooms** (Stun heavy put on: the squad throws one into a room first when it has one to spare,
  it is sent into three rooms on that street by the lesson's own door lines, and the card counts what they carry now, three as issued, so nothing to
  spare — the step's own 'Squad carries' line stays in view under the row) · **The machine
  gun** (your own card opened: a frag goes over sandbags; Stun heavy took yours and one will be handed over, but pack your own, so a smoke canister is traded for a frag grenade — or, with one
  carried, 'keep it') · **The four** (a teammate's card opened) · **That's a plan** (Deploy; packed with no frag it says one will be handed over, which
  the page does, `kit.ts kitLessonPage`, and his card's frag tile reads '1 · mission's' on that step: `loadout.ts lentNow`). A beat whose row the step does not have, or whose thing is already so (Stun heavy on), is left out. THE WORDS
  follow the writing rules (`docs/agents/writing.md`): to "you", who presses the key; a count in a sentence is written out; no digit, no compass word.
  ENTER UNDER A CARD IS THE CARD'S 'NEXT' wherever it would otherwise deploy (every row but the named kits, the man's card and Deploy's own): a first
  press of Enter on a card never starts the mission. 'Skip' on every card, or Esc: the first Esc is the walkthrough's, the next is the step's own.
  Finished, skipped or deployed UNDER A CARD, it is not shown again, under ITS OWN key (localStorage `twopointsix.kitplan.tut-squad`: a browser that
  went through the general walkthrough has not been through this); THE WAY BACK is the word 'How to plan' on the step's head. NOT PLAYED UNASKED IN A
  BROWSER A SCRIPT DRIVES (`navigator.webdriver`, `KITGUIDE.driven`): every other page tool measures the step as it stands; the brief and the word ARE
  on their pages. THE SWITCHES: `?kitguide=0` = no walkthrough and no word on any step (that mission's cards stay) · `?kitguide=1` = played on this
  visit whatever the browser remembers, and ON A REGULAR MISSION the step's GENERAL walkthrough (`kitGuide.ts GENERIC`: the job · who comes · a kit for
  the squad · each man · his card · go; word 'How to pick', key `twopointsix.kitguide`), kept for comparing and played only so · `?kitpack=0` = the
  game's rule off: that mission as issued again, no cards on its step and so no plan. UI only but for that rule. THE SEAMS, in `ui/loadout.ts`:
  `LoadoutOpts.key(code, repeat)`, asked before any row of the kit screen (the map's M, and the keys that put the large sheet back);
  `LoadoutOpts.rest(code, repeat)`, asked after the highlighted row and before the screen's own Enter / Esc (the walkthrough: an open gun pop-up's Esc
  shuts the pop-up first); `LoadoutScreen.kitName`, the named kit that is on ('' = a mix of the player's own). WHAT IT CHANGES AND KEEPS: the things it
  asks the player to do are real changes to the kit this machine keeps, and that kit is what the mission is then played with. CHECK:
  `PORT=<dev server> node tools/qa/kitguide-page.cjs [outDir]` — the real page in headless Chromium, no GPU needed, real keys and clicks at 1440 x 900,
  1353 x 770 and 1000 x 673: the brief's map, words and counts on the registry and the office's rescue, no digit above 5 among the facts, M, the short
  band no taller than the old head; a regular mission with no card, no word and a bare map; the squad mission's cards at its own four, the five words
  pinned on its large map (each on the sheet, clear of the others), every beat's card up beside its lit row, whole in the window and clear of the row,
  no digit and no compass word on it, put away by its own thing and not by another, the squad's line in view and counted as the card counts it, its own
  key, the word, both Esc, a browser that finished the general walkthrough still walked through, Enter as Next, a pop-up's Esc, the words with the kit
  as issued and with no frag packed, each switch, the general walkthrough under `?kitguide=1`, the handed frag read whole on its tile, a driven browser (148 PASS); `KITGUIDE_BREAK=1` = the
  walkthrough and the brief off: FAILS. The game's side has its own check (`probes.ts kitpack`, `docs/agents/missions.md`): the plan's marks lie in the
  level's rooms of those names, what its cards say of the kits is true of the kits, and the mission is played through on five kits. NOT SEEN: on a GPU
  (so never with the team's picture in the left column behind it, and the pinned words never over the sheet as a GPU's browser draws it), by a
  first-time player. (The Mission step's facts row wrote 'Defenders 11' as digits: since 10-23 no screen draws that cell.) OPEN, for the menus' owners: a named kit a mission
  recommends (a field on its row, a mark on the Kit row's button) is not built for regular missions — only what the place asks is said. OPEN, for the
  squad mission's owner: its frame refuses the player's smoke and stun slots all mission, so of what he packs for himself only the frag is used.
- **THE DEBUGGER HAS NO BOX FOR A SETTLED SWITCH (2026-11-07; both halves, and a probe that holds it).** On the page every row the switch list calls settled is held at its
  page value (`src/host/rustgame.ts rustSettledHold`: a read answers that value, a write is refused), so a box that showed one could not be moved. OUT:
  the 256 boxes written as `panel.toggleBox(…, () => ROW, …)` in `src/ui/bindings.ts` whose getter reads only settled rows of `tools/qa/switches.json`,
  with the imports only they used. A click test of every row (the review's, headless, the hold laid) found none of the 256 could move. Among them: 'Tab ▸
  the orders ring', which could not move but, clicked off, put the maintainer's two open ring rows off for good; and the two '(reloads)' boxes.
  THE SECOND HALF, the rows built from data. THE HOLD SAYS WHAT IT HOLDS: `src/host/held.ts` (import-free), `markHeld` called by the page's hold and by
  the tools' fold as each lays a row, `rowHeld(table, key)` true for a laid row and for a key the table has not got. Nothing is guessed from a row's
  shape (`SLOTS.on` and `BIGSLOT.on` answer through accessors of their own and are live). WHO ASKS: `knobRows` (and `feelRows` through it) builds no
  row, yes/no or number, for a held key, and says so on the console for a key that is not there; the sneak lab's switches (now data, `t` and `key`,
  like its numbers) and its numbers; the fire lab's one box. A panel built BEFORE the hold is laid would get the rows after all: on the page the
  hold is laid first (`main.ts`). Out by hand: the nine 'Enemy nerve' rule boxes (all settled; the first, unticked, was refused and still reset every
  man's morale) and the slider 'way round a room ×'. COUNTS: 283 more boxes and 9 sliders over settled numbers gone from the debugger (1,367 rows to
  1,075 on the page's stand; 190 boxes left). `panel-keys.cjs` wants more than 1,000 rows: 75 to spare.
  HELD BY A PROBE: `probes.ts panel-dead` (family `ui`; `tools/qa/panel-inventory.ts panelDead`, also `panel-inventory.ts --dead`) builds the live
  debugger under the fold, clicks every yes/no box and fails on one whose answer does not change. 190 boxes, 168 judged: three are named and not
  clicked (`NOT_CLICKED`: two show a page object the harness fakes; the one box that reloads the page is judged by the row it shows, and a
  '(reloads)' box that is not named there fails the line) and 19 answer no yes or no in the harness (a stand-in of the engine's: not judged).
  `PANELDEAD_BREAK=1` (the probe adds a box over a settled row itself) fails naming it. On the old stand nothing is held and the line is not asked.
  THE SCENE PANEL ASKS TOO (`?tweak`, `src/ui/sceneLayers.ts`): its `table()` makes no slider for a held row, by the same `rowHeld`. One row went,
  the fog shade's `FOG.desat` (370 numbers to 369 where rows are held), and `scenetweak` 'round' is green again with no word changed: nothing is
  left that will not move. Its 'breath' line wrote the hard breath rule's switch both ways; that switch is a settled row, so where it is held the
  line asks the one side the page plays and writes nothing (both sides where it is still a switch, which no stand that can step a game is today).
  It is still RED on this tree: the stand-in files no breath record on the Rust game yet (a mend of the call surface, not of this line), as are
  'oneair' and 'digest'. NOT DONE: a row that is only ever dimmed because its `readWhen` asks a held switch (the breath's four soft rows) still
  has its slider.
  NOT HELD BY IT: a slider or a select over a held number (it clicks boxes only); the two labs' panels (they ask the same query, unprobed here; the
  sneak lab's page check `stalk-page.cjs` now asks that two of its settled switches have NO box, and was NOT run).
  NUMBERS LEFT INERT, with hover texts pointing at a box that left: the five rows that say 'Does nothing while every mission starts the same is on'
  (it is held on, so two sliders there move nothing), 'setup ▸ seconds it stays up unused' (the only row left in Gun setup), 'far line: distance from
  you (edge off)'; section summaries that print a constant 'on'. THE NOTES WERE NOT SWEPT: where an area note names a debugger box for a settled
  switch, that box is gone.
  KNOWN BESIDE IT, NOT THIS CHANGE'S, AND GONE SINCE 2026-11-07 WITH THE WORD: `?round=0` put the PICTURE's half of round things off (`main.ts` reads `ROUND.on` for the GPU before the hold
  is laid) while the game holds them round; a machine that once unticked a glass box keeps `{on:false}` in its store, read back and refused at every
  visit, with no box left to rewrite it ('1 kept row … is not applied' on the panel's first line); `wallpen-page.cjs` cannot pass on this tree for its
  `?wallpen=0` arm, a settled word the hold turns away (its panel line now reads the rule's status line where it read the box; NOT run).
  CHECKS: `panel-inventory --print` before and after: 256 toggles fewer and nothing else moved (1,622 rows to 1,366, and 1,367 with the kit weight's line; the civilians' pleasantries box left
  the fixture with it, so the same 5 reference rows are missing as before; the second half took four more with it, the wind's four boxes); `panel-keys.cjs` on the real page
  in headless Chromium, software adapter: ALL PASS after the first half (the page boots, every page of the panel has rows, no page error); NOT run
  again after the second. Both typechecks, the knob check and the build clean. 'Kit weight' keeps its
  read-out as a row of its own (a section with no row is hidden). The one-off script: project files `rebuild/switch-cleanup/scripts/deadbox.cjs`.
  Never seen on a GPU.
- **EIGHTEEN SWITCHES RULED SETTLED, TAKEN OFF THE PANELS (2026-11-07).** The port ruled these rows settled at their page value (project files
  `rebuild/rust-sim/port/cleanup/OPEN-ROWS-RULING.md`); a row with a setter on the page could not be folded, so the setters go first. THIS CHANGE ALONE DOES NOT
  SETTLE THEM: the rows turn `settled` in `tools/qa/switches.json`, and constants in the Rust game, in the fold this change travels with; until that is in
  the tree a row here is still a switch its address word can put off, with no box to show it. Fifteen of them the Rust game branches on; the three snow
  rows are read by the page's own snow code and by nothing in Rust, and their other side stays in that code until the rows are retired. Gone from the debugger
  (`src/ui/bindings.ts`), fifteen boxes: 'running ▸ arms carried close' (`RUNARMS.on`); 'throw order ▸' a man walks to a throw, works his throw out, frag
  where men can duck, a frag near you is your call, drawn on the ground (`SQUADTHROW.walk`, `lob`, `duck`, `samCall`, `draw`; THE switch 'teammates throw
  frag / stun on order' stays); 'teammates ▸' slower than you, run with you, run for a reason, look where they walk (`TEAMPACE.on`, `follow`, `own`,
  `walkAim`; 'no sprint to a stack' and 'pace themselves on you' stay, the maintainer's to try, and the pace's numbers stay); 'charge order: carried out
  or answered' (`DOORTOOLS.chargeAnswer`); 'wall snap:' the rifle stacks, sprint into a wall to take it, heard on and off the wall, the take is unhurried
  (`WALLSNAP.stack`, `pressRun`, `voice`, `slow`; 'wall snap: feel pass + rifle stack' and 'walk into a wall to take it' stay, and every number). Gone from
  the scene tweak panel (`src/ui/sceneLayers.ts`), three switches: `Snow in one air`, `Things mark the snow` and `Blast marks` keep their layer, numbers,
  status line and the blast's button, with no box (`SNOW_LOOK.oneAir`, `marks`, `blast`). The rows' address words and stand-up words are not this change's.
  CHECKS: `scenetweak` 'oneair' now asks that the three layers have no switch and no longer puts one air off and on again, nor tells a blast with its
  switch off (the off side is asked by nothing now); `scenetweak-page.cjs` step 4b the same, and its step 5 reads the wind sim, the one `mode` switch left,
  for what `none` leaves on. `panel-inventory`: the same 5 reference rows missing before and after (none of the fifteen is in
  its fixture). On the tree this was cut on, `scenetweak` is red on 4 lines before and after for the tree's own reasons (the blast button tells the Rust
  game no blast, among them). Typechecks clean. Never seen on a GPU, and the panel was not opened in a browser for it.
- **The debrief calls a door what its level calls it (10-19, rides `WORDS.doors`).** A kicked, picked or keyed door's row was worded from the door's
  id ('kicked in the breakroom w door') while the squad's rows said the name its level wrote (`scenarios.doorNames`, `doornames.ts`). Now
  `chronicle.ts eventText(e, g)` and `fmtEvent(e, g)` take the game the row is of, and a door row asks `doorWrittenName` of that door WHEN THE ROW IS
  READ: 'kicked in the cubicles-side break room door', 'picked the lock on the …', 'keyed the code — the …', '… gave up the code to the …' (the written
  name is the whole name: no ' door' added to a 'yard gate', no possessive hung on 'double doors'). The row's DATA is untouched (the id, as ever): nothing
  in a checkpoint, a recording or a digest moves, and a row from before, a loaded checkpoint or a flipped key all read by the keys as they are now. No
  written name, the key down (`?doornames=0`), the words pass off (`?oldwords`), or no game behind the row (a tool printing a timeline with
  `fmtEvent(e)`): the line exactly as it was. So the frozen stand reads as it did (`TPS_WORDS=1 TPS_DOORNAMES=1` raise it), and the door names key is the
  switch: off = the card as it was. The card's two readers pass the game (`ui/debrief.ts`: the timeline's row, the map pin's hover line). NOT DONE: a
  door with only a MADE name ('hall-side mess door') still reads its id on the card: `doorWords` would also spell an unnamed door's bearing out ('vault
  west door'), and the card has no compass. Check: `a_door_row_calls_its_door_what_its_level_calls_it` in `rust/game_start/tests/debrief.rs`. Only the office's two break room doors have written names so far, so that is where it shows.
- **A wall charge's rows on the debrief say a charge (10-19, rides `WALLKILL`).** A man at the charge when it went read 'shot dead (a wall charge)' and
  the player 'DOWN — the wall charge's round'. `chronicle.ts eventText` now words them 'killed — the wall charge' and 'DOWN — the wall charge' (a `by`
  that ends in 'charge' takes no 'round'); the rows' data is as the charge files it, every other kill and down row reads as it did, and the end card's
  own line ('by the wall charge') needed nothing. Both rows exist only with `WALLKILL` up, so its switch is theirs. THE KNOCK THROUGH THE WALL (older than
  `WALLKILL`, on both stands): a man or a civilian knocked out behind the wall it opened was filed and read as 'the takedown'. `chronDown` and
  `civilians.ts` now file the row's `method` as 'charge' from the charge's own line ('… behind the wall when it went') and the row reads 'out cold —
  the wall charge'. That is a row's data and words only (no tally, no digest: a digest holds a row's kind and time), and a Training lesson's
  'downed how: takedown' step no longer counts a charge's knock. A checkpoint saved before keeps its 'takedown' rows. Check
  `a_charge_files_its_own_rows` in `rust/game_start/tests/debrief.rs` and the `words` test in `rust/game_rules/src/chronicle`. Text only; nobody has played to it.
- **Options ▸ Game ▸ 'Walk into a wall to take it'** (10-20; `ui/options.ts`, shown only with the wall hold's feel pass on): flips `WALLSNAP.press`, kept in
  `twopointsix.options` as `wallpress`, a `?wallpress` in the URL wins over the kept value, Reset puts it back on. The rule: `docs/agents/stealth.md`, the
  wall taken without the key. The Controls list and the Options key list do not mention it.
- **THE OUTLINE IS SETTLED (2026-11-08; his card "Cut as marked").** One row stays: 'Outline in the dark', on as the page starts it. The rows Outline width, strength, fades by, shows under light, full under light, teammate width and colour are gone, a kept line that names them reads as if it did not (`findMeRead`), and the numbers are his hand's: `render/outline.ts OUTLINE` (`wide` 0.25, `opacity` 0.6, `lit` 0.3, `dark` 0.12) and `host/findme.ts OUTLINE_LOOK` (`mates` 0.8, `colour` 0.5). The light meter's mode is gone with `outlineWant`, the frame's `strength` and the shader's blank picture: the rim goes by the picture alone, and a frame with no copy of the composite (none but the first) draws none. A second player down still has his own rim (`findMeOutline`: the line that meant to take it out was read only by the meter, so nothing drawn moved). No switch: a cut. CHECKS: `findme` (`FINDME_BREAK=1` must fail), `teams`, `tools/qa/draw-equal.cjs` base against tip. The bullets below say how the rows came about, and every sentence in them that names a row is history.
- **Finding your own man, and the camera under the hand: twelve rows on trial in Options > Game (10-19; six, cut to four and back to five 10-20; the outline's width row 10-21, its strength, two light rows, teammates and colour 10-22;
  `src/host/findme.ts` the table, `src/ui/findme.ts` the rows, `src/ui/findmeviz.ts` the ring and the outline's frame, `src/render/outline.ts` the outline's pass).** From a play test: a new player kept losing his own man on the screen, in the dark and behind
  walls, and reached for a camera that turns the other way and one that snaps behind him. The maintainer asked for settings to play with, none a new
  default THEN (since 10-26 the outline's four rows start at the maintainer's own numbers ON THE PAGE, behind a switch with a `FROZEN_HOLDS` row: THE OUTLINE IS ON, below; what follows is the module and the frozen stand), so there was no switch and no `FROZEN_HOLDS` row: `FINDME` starts as `{ pan: 1, ring: false, outline: false, width: 1, opacity: 0.6, picture: true, lit: 0.3, dark: 0.12, picLit: 0.3, picDark: 0.12, mates: 1, colour: 0.6, turn: 1, behind: false }` on the page and on
  both stands, and at those starts the build is the build as it was (measured against the build head each time: the same game digest and the same
  view matrix over 15 s of a scripted hand, two seeds, free step and fixed, frozen and shipped). Kept per machine in localStorage
  `twopointsix.findme`, read at load and again when the page says which starts are in force; Reset to defaults on the Game tab puts all twelve back. THE ROWS: `Aim pan` (a slider, none to today's: it
  scales `FollowCamera.aimLead` as built; none = he is the middle of the screen wherever the mouse goes), `Ring at your feet` (the gold ring the order
  menu draws, `PLAN.ringM * 1.35`, on at all times; it stands down while the menu's own is up: `PlanFloor.youOn`), `Turn direction` (the camera's key + mouse
  turns the view the other way: one function, `host/findme.ts ctrlTurn`, asked by the loop's orbit and by the engine replay's own Ctrl turn,
  `replay/engineplay.ts ctrlTurn`). (A `Double tap` row brought the view round to where he looked, his aim carried with it, until the auto camera took the tap; it left with that mode, 2026-11-08.) The maintainer played them 10-20: "aim pan zero and turn
  direction reversed feel good"; whether those become the starts is his to say.
  THE OUTLINE IN THE DARK IS THE RENDERER'S (10-20). The first one, and a `Shape through walls` beside it, were drawn in the overlay's SVG from his
  joints, each limb a stroke as wide as a bare limb with a filter's rim round the lot, and were only ever looked at close in on the office level. From
  a squad level's distance an arm was two pixels wide and the outline was a stick figure's (the loop of his arms a box, his legs a zigzag); the
  maintainer's shots, and his word: "it should be a proper silhouette. if it's not really possible given our rendering pipeline we can just shelve the
  idea". It is possible: the G-buffer's id target (`gbuffer.wgsl`: `owner << 20`, on his skinned mesh and on every box emitted for him) says whose
  each pixel is. So `Outline in the dark` (off at its start) is now ONE PASS over the composite, after the fog of war and before the resolve
  (`render/outline.ts OutlinePass`, `shaders/outline.wgsl`; the one call in `render/passes.ts`): a blended triangle inside a scissor round him, in which
  a pixel that is not his takes a pale ink by how near the nearest of his pixels is (`OUTLINE`: 1.125 to 2.25 pixels of the composite by its height since 10-21, the
  outer pixel fading) and only from one of his that stands in front of it by the G-buffer's depth, so a wall top, a table or a teammate between him
  and the camera is not drawn on. His body, kit and gun are in it because they carry his owner byte; so do a round of his for the frame it leaves the
  muzzle and a charge while his hands set it, and they take the rim too. HOW STRONG is the light meter's word (`host/findme.ts outlineWant`: full at
  `Player.visibility` 0.12 and under, none at 0.3 and over as the rows start, none under the goggles), eased at 6 a second, taken whole on a frame with no time in it
  (the row flipped under the pause card shows at once). The page fills the frame once a frame (`ui/findmeviz.ts findMeOutline`: off with the row off,
  the man down, and where the HUD's own marks of him stand down: a stun's white-out, attract mode, the kit's picture); `Engine.render` adds the box
  on the picture from the camera it draws with (`outlineBox`; he behind the lens = no pass), so a still or a card's settled frame has its own. WHAT
  IT COSTS: with the row off, nothing: the pass object is not made and `encode` is not called. First frame on: one bind group layout, a small
  uniform buffer, one small module compiled off the frame; until that is in, and for good if it fails (said once), nothing is drawn. On and he is in the
  dark: one render pass on the composite (load and store), a scissor of about a tenth of the picture close in and a fiftieth from a squad level's
  distance, up to 49 id reads a pixel inside it (81 with the width row at its most). No scene group, no storage buffer, no timer row (a render pass's stamp
  is left off here as the particles' draw's is). On and he is lit: no pass (BY THE METER; by the picture, below, it runs lit or dark and reads a
  copy of the composite). A kept line from the stick figure's days that says `outline: true`
  turns this one on. UNDER THE RESOLVE: the rim is drawn before it, so it is anti-aliased with the picture, and the pixel next to him rides his
  motion vector; the outer pixel has the floor's and may trail him by a few frames as he moves. Nobody has seen that on a GPU.
  ITS WIDTH (10-21, 10-22). The maintainer, having seen the rim on his GPU: "outline in the dark is good but can you add a slider to the options
  for the width? it's just a touch too fat"; and a day on, with the slider at its least: "can you make the outline slider go to zero? still large
  at 50%". THE RIM ITSELF IS A QUARTER THINNER than first built: `OUTLINE` is 1.125 to 2.25 pixels by the composite's height (it was 1.5 to 3);
  the quarter is a guess at "a touch", and it is in the rule, not in the row. `Outline width` IS A ROW (a slider under the outline's row, none to
  150 % in steps of 5; `FINDME.width`, starts at 1). Over 100 % the row multiplies the radius. UNDER 100 % IT SCALES WHAT THE RIM COVERS, not its
  radius: the shader fades the outer pixel (radius + 0.5 less the distance), so a rim of radius r covers r − 0.5 of a pixel beside him, and
  `outlineRadius(h, wide)` = 0.5 + (the rule − 0.5) × the row squared. Every step of the row is a different rim on any composite, finest where
  it is thinnest, down to none, where the frame is off and nothing is drawn; and no kept row under 100 % is drawn wider than before (the row
  first multiplied the radius and stopped at a floor of 0.9: on a short composite its low end did nothing, and on a tall one 50 % was still a
  whole pixel and more). A LINE IS NEVER NARROWER THAN ITS PIXEL: under one pixel covered, thinner is that pixel fainter, and a composite pixel is more than one
  of the screen's wherever the canvas is capped under the window. At most 3.4 pixels for him and 4 for anyone (`OUTLINE.capPx`): up to 81 id reads
  a pixel inside a man's scissor.
  HOW VISIBLE, AND BY WHAT LIGHT, ARE ROWS TOO (10-22): "can you also add a slider for how visible it is? i think it should probably fade in and
  out smoothly based on how lit up sam is -- sliders to calibrate that". `Outline strength` (`FINDME.opacity`, the ink's opacity at the full,
  none to 100 %, starts at 60 %: the 0.6 that was `OUTLINE.alpha`; none = the frame off), `Outline shows under light` (`FINDME.lit`, starts at
  30 %) and `Outline full under light` (`FINDME.dark`, starts at 12 %): the two ends `outlineWant` fades between, by the light meter, in the
  HUD's own LIGHT number; neither row crosses the other (each stops at it, so the number a row shows is the one in force; Reset writes both
  whole). The three start at the numbers the rim was built with; what he
  settles on is to become their starts, and FOR THAT THE STORE KEEPS ONLY THE ROWS THAT ARE NOT AT THEIR STARTS (`keep`; Reset leaves `{}`), so
  a start tuned later reaches every machine whose row stands at the old start (one put back there by hand too: rows on trial). A full line kept
  by an older build reads as before and is written again that way as it is read. WHAT HE CALLS ITS PULSING ("on levels like the block with the
  breaker thrown, the current pulsing of the outline really ruins the lighting. we'll tweak it") is the meter crossing those two ends as he
  moves: nothing in the pass moves with time but the ease. Both light rows at 100 % hold the rim steady; the strength row dims it.
  THE METER IS NOT THE PICTURE (his next line: "the light meter is a little funky. is it just measuring at a single point?"). It is asked at his
  chest and his head, the brighter kept; each is the brightest of six facings of shadowed direct light (one ray to each light's centre) plus the
  bounce (`probe.wgsl`, `Game.lightMeter`), eased at 6 a second, a frame or two late. So window light on one side of him reads lit while the side
  the camera sees is dark (no rim where one is wanted), and a dark man on a lit floor reads dark (a rim where none is needed). The meter is the
  guards' and is not to be bent for this; a rim that fades by how he is DRAWN against what is behind him is the picture's own job and is NOT
  BUILT (put to the maintainer as a choice).
  BY THE PICTURE, PART BY PART (10-22, an hour on, the rims in colour on his GPU: "these feel a lot better already"): "is there a way to make it
  so it naturally turns on for dark parts of the character and off for bright parts? that might make it feel less pulsey". `Outline fades by`
  IS A ROW (`FINDME.picture`: `Picture`, where it starts, or `Light meter`, the rim as first built). By the picture the meter is not asked at
  all: `outline.wgsl`, at every pixel beside a man, takes the mean light of the few pixels of HIS within the rim's reach off this frame's
  composite, makes of it a brightness on the screen (the exposure on, l / (1 + l), a square root: 0 black … 1 white) and fades the rim between
  two ends. A lit shoulder has no rim and the leg in shadow under it has one; nothing swings as the meter's one number crosses a line, and
  nothing is eased (the frame's `whole`: 1 but for the goggles). THE PICTURE HAS ITS OWN TWO ENDS (`FINDME.picLit`, `picDark`): the two light
  rows show and move the pair of whichever the row above says, so the meter's numbers are never read as brightness on the screen nor the other
  way; both pairs start at 30 % and 12 %, which were tuned for the meter and are a GUESS for the picture (a part darker than 12 % of white has
  the whole rim, one brighter than 30 % none). THE COMPOSITE IS READ FROM A COPY (the pass cannot read the target it draws on): `hdrCopy`, which
  GBAA already fills each frame before its edge blends and the fog; where GBAA does not run (render resolution at 100 %, TAAU off, its own
  toggle) `passes.setOutlineCopy` keeps that texture for the outline alone (the engine mirrors the frame's `on` and `picture` before resize, as
  it does the fog's switch; the composite's set is made again only when that changes what it holds) and `passes.ts` fills it just before the
  pass. A frame handed no copy goes by the meter. WHAT IT COSTS: by the picture the pass runs every frame the outline's row is on, lit or dark
  (by the meter it ran only in the dark), reads the copy only at pixels that are a man's, and without GBAA adds one copy of the composite a
  frame and its 8 bytes a pixel. NOT EASED, AND THAT MAY SHOW: a muzzle flash that lights him takes the rim for the frames it lasts; whether
  that reads as right or as a blink nobody has seen. Line (10) of the check holds the frame, the slot, what is bound and the two pairs; what
  the shader makes of the copy only a device can say: `findme-squad-page.cjs` counts the rims' ink on the software adapter with the rows at
  their most, at none, at their starts, and at their most again with GBAA put off. MEASURED on that adapter, one run, a squad of four stood in a dark room, strongly coloured pixels in the men's
  boxes: 1771 with the rows at their most, 65 at none, 1591 at their starts (men that dark have nearly the whole rim there too), 2419 at
  their most with GBAA off (the copy kept for the outline; that picture is not anti-aliased, so more of it counts), no
  GPU error in any. (The first run, 10-22, read 1501 / 66 / 1629: its picture at most was drawn two frames younger than the one at the starts, and
  the resolve gathers a new line over several frames, so the starts read over the most. The check draws the same frames for both now and holds
  the starts at no more than a tenth over the most, the room it always gave.) That is the ends and the two copies; NOT MEASURED: a man half lit, which is the case the row is for. `findme` is green
  on both stands, its break word red with seven lines. THE TWO COPIES ARE NOT THE SAME PICTURE: GBAA's is taken before its edge blends and
  the fog of war, the outline's own after both; the team's own men are never fogged and the blend moves edge pixels only, so the mean over
  a man's pixels is near enough the same. Nobody has seen any of it on a GPU, and nobody has timed the pass by the picture.
  HIS OWN ROWS when he said so (a screenshot, 10-22): width 25 %, strength 60 %, the light rows at their starts, teammates 80 %, colour 50 %.
  Not made the starts: he has not called them settled.
  HIS TEAMMATES HAVE RIMS TOO, AND THE RIMS HAVE COLOURS (10-22): "can you also add outlines for teammates + have a separate slider for thickness
  multiplier relative to the player? make the outline for the player yellow and for teammates red/blue and have a slider for saturation".
  `Teammate outline width` (`FINDME.mates`, none to 200 %, starts at 100 %; none = no teammate has a rim) and `Outline colour`
  (`FINDME.colour`, none = the pale ink the rim was first drawn in, 100 % = `OUTLINE_INK`: yellow for him, red for the red pair, blue for the
  blue pair, a duo's partner and a squad not in pairs; starts at 60 %, a guess; IN A GAME OF TWO PLAYERS the whole team is rimmed and the second
  player's own man is purple on both screens: `docs/agents/net.md`, 'Two players, two teams to the eye'). THE TEAMMATES' ROW IS A SHARE OF WHAT HIS RIM COVERS on the
  same picture, whatever his own width row made that (`OutlineMan.times`, `outlineRadius(h, wide, times)`: 50 % is half his, at any width; it
  first multiplied his row, which the curve under 100 % squared), never over `OUTLINE.capPx` 4, so his row at its most with theirs at its most
  stops there. WHO: the living men of his squad (`SquadOrders.men()`, the roster, whatever the orders menu is doing: a show's men too), each
  BY HIS OWN LIGHT METER (`Guard.visibility`, the unit's meter the hostiles' eyes read) through the same two light rows, each eased on his own;
  nobody else's man ever. The frame is the player's man plus `mates` (`OutlineMan`, a pool keyed by owner byte: a man dead gives his place
  up); `outlineBoxes` boxes each for the render's camera (a man off to the side whose box crosses the lens's plane gets an empty box, not the
  whole picture). THE PASS is still one render pass: a scissored triangle a man, each from his own 256-byte slot of one uniform buffer (a
  dynamic offset; `OUTLINE_MEN` 8 slots), THE TEAMMATES FIRST AND THE PLAYER LAST, so where two rims meet his is the one on top and in a crowd
  his place is kept. WHAT IT COSTS: five men are five small draws and one bind group; men stood close are each looked for in their own box, so
  where boxes overlap the id target is read once a man (a stack of five at a door: five times the reads over that patch, a few per cent of the
  picture). A machine that kept the outline on gets the teammates' rims and the colours at once (that is the ask: with `picture`, the only rows
  whose start is not the outline as first built); with the outline off nothing is asked of the squad's list. Line (11) of the check stands a
  squad of four up and holds all of it, on a device that writes down what it is asked (the layout's dynamic offset, a slot and a scissor a
  man), and that a squad stepped with the outline on ends digest for digest where it ends with it off. `tools/qa/findme-squad-page.cjs` is
  the real page's word on it (the four rimmed, the program in, a draw, no console line; stills to look at). Nobody has seen the colours on a
  GPU; the three inks were picked by number.
  Nothing reads these rows with the outline off; the page reads them every frame, so they move live under Options. Line (1) of the check holds
  what is kept; line (10) the rim's own width inside its two pixel counts, every step of the width row, the strength row and the light rows'
  ends; the page check moves every slider and takes width and strength to None and back. Nobody has seen any of them on a GPU.
  THE OUTLINE IS ON FOR A MACHINE THAT HAS KEPT NOTHING, AT THE MAINTAINER'S OWN NUMBERS (10-26). He watched a first-time player on a fresh
  browser profile: "i don't think the outlines showed up for him?" (they were off, as every row here started), then "we should take the numbers
  i tuned ... and use those as the defaults". His rows as he left them on his GPU on 10-22 ("overall these feel a lot better already"):
  `FINDME_TUNED = { outline: true, width: 0.25, mates: 0.8, colour: 0.5 }` (and, later the same day, `pan: 0, turn: -1`: the paragraph after
  this one); the strength, the fade by the picture and the light rows he left at their starts. SWITCH: `FINDTUNED.on` (`host/findme.ts`), ON in the page (`?findtuned=0` = every row starts as built, no rim until Options
  asks, as it was), OFF in the module and on the frozen stand (`FROZEN_HOLDS['FINDTUNED.on']`, `TPS_FINDTUNED=1`; the shipped stand boots with
  it, `TPS_FINDTUNED=0` there is the page's word). In the knob registry as THE VISIT'S OWN (`knobs.ts VISIT_OWN`, beside `SAVEPIC`; its word on
  `ui/replaykey.ts VISIT_FLAGS`): the outline's rows are a picture on this machine's screen, in no recording, and no digest can tell them on
  from off (line (3) of the check); THE CAMERA'S TWO, added later the same day, are not so on a harness — the next paragraph. The table is
  not read off the switch frame by frame: WHOEVER WRITES `FINDTUNED.on` CALLS `findMeStarts()` (`pageSwitches` and the stand do, and
  `knobs.ts knobsBack` when the copy it puts back moves the switch: before that a probe that booted the page's switches on the frozen stand
  and put its knobs back left the tuned rows in the table until the next game was stood up, which mattered the day the turn joined them —
  the loop reads `FINDME.turn`).
  `FINDME_START` IS NOW THE STARTS IN FORCE (`FINDME_BUILT` under `FINDME_TUNED`): what Reset puts back and what a kept line is written
  against, so the rule above (only moved rows are kept) does the work. `findMeStarts()` moves a row standing at its old start to its new one,
  leaves a row a hand moved, and where there is a store reads the kept line again: a machine that kept nothing gets the rim; one whose line
  names a row keeps that row, Off included; a line that kept these very numbers by hand is the same table and is written again as only what
  still differs. THREE TRAPS IN THE KEPT LINE, each held by line (12): (a) the module's own read as it loads no longer tidies the line —
  tidied against the built starts a kept `outline: false` is "at its start", written away, and drawn again on the next visit; the line is
  tidied only by the page's call under the tuned starts; (b) a row moved on a `?findtuned=0` visit is written INTO the line as it lies
  (`keep(k)`), not the table whole against the built starts, for the same reason; (c) a store whose name throws (a frame with storage
  blocked) still gets the page's starts: the read is tried apart from the starts. KNOWN AND LEFT: a line an older build kept WHOLE (all
  fourteen keys; the builds of 10-20 and 10-21 wrote it so) on a machine not opened since reads as fourteen rows moved by hand, so it keeps
  the outline off; a line `{"outline":true}` from before this change keeps the rim and takes the new width, teammates and colour (rows at
  the old starts follow the new, by the rule above); Reset on a `?findtuned=0` visit writes the line empty.
  WHAT A PLAYER SEES THAT IS NEW: on any machine where the row was never touched, a thin rim round him and his teammates where they are dark
  on the picture, in half colour. AT 25 % THE RIM IS A FEW PER CENT OF ONE PIXEL (`outlineRadius(h, 0.25)`: 0.54 px on a composite 630 rows
  high or less, 0.55 at 720, 0.58 at 1000, 0.61 at 1260 and over; the pixel beside him gets the strength row's 60 % of radius − 0.5, so
  about 2 % to 7 % of the ink): his pick on his own high-density screen, and on a coarse screen or a low quality tier it may not be seen at
  all. Whether the start should be a share of a pixel covered and not a share of the rim's width is not settled; nobody has looked at it on a
  second machine. WHAT IT COSTS, read not measured: by the picture the pass runs every frame the row is on, lit or dark (a scissored draw a
  man, five on a squad level), and where GBAA does not run (render resolution at 100 %) the renderer keeps a copy of the composite for it:
  one copy a frame and 8 bytes a pixel that a fresh machine did not pay before, and the Auto ladder and the benchmark now run with it.
  Checks then: `probes.ts findme` line (12, gone with the cut of the checks and the rows 2026-11-08), ALL PASS on both stands (`FINDTUNED_BREAK=1` = the boot leaves the starts as built: that line failed
  on both; `FINDME_BREAK=1` still fails its seven); lines (1) to (11) are the rows' own and run on the built starts whatever the stand
  booted with. `tools/qa/findtuned-page.cjs` is the real page's word (software adapter, a squad of four: a browser that kept nothing, the
  word, a kept Off; the Options screen read off its own rows, a row put Off, Reset): RUN ONCE, 1024 × 576 on `verylow`, ALL PASS, six lines,
  no page or console error; the fresh browser's pass drew 7 frames once its program was in. `findme-page.cjs` and `findme-squad-page.cjs` open the
  page with `findtuned=0`: they are the rows' own checks, and NEITHER WAS RUN on this change (35 minutes each on the software adapter).
  NOBODY HAS SEEN the tuned rim on a GPU but the maintainer on his own machine, and nobody has measured what the pass costs a machine that
  did not ask for it.
  THE CAMERA'S TWO ARE STARTS TOO: AIM PAN NONE, TURN DIRECTION REVERSED (10-26, the same day). His own rows since 10-20 ("i think aim pan
  zero and turn direction reversed feel good"); asked whether they should be defaults as well: "yeah let's do those as defaults to".
  `FINDME_TUNED` holds `pan: 0, turn: -1` beside the outline's four, under THE SAME SWITCH and the same words (`FINDTUNED.on`,
  `?findtuned=0` = the view leans toward the aim point and turns as it did, `TPS_FINDTUNED=1` on the frozen stand): one way back for all six.
  WHAT A PLAYER SEES THAT IS NEW, on any machine where the rows were never touched: he stays in the middle of the screen wherever the mouse
  goes (as far to see behind him as ahead: the view no longer leans 18 % of the way to the aim point), and the view turns the other way round
  under the camera's key, in play and in the engine replay's Ctrl turn. Options > Game opens with Aim pan at None and Turn direction at
  Reversed; a player who wants them as they were puts the pan to its full and the turn to Normal once, and that is kept
  (`{"pan":1,"turn":1}`: rows moved off the starts). `findMeRead` now reads a line with no `turn` as THE START IN FORCE (it read 1); under
  `?findtuned=0` that start is 1, so no kept line reads differently there. His own line (`{"pan":0,"turn":-1}` since the outline's starts)
  is the same table and is written again as `{}`. A MACHINE THAT SET EITHER ROW BACK TO ITS OLD START BY HAND has no line that says so
  (only moved rows are kept) and gets the new start, unasked: the rule at the top of this section, and the same for the outline. A line an
  older build kept WHOLE (the builds of 10-20 and 10-21) names `"pan":1` and `"turn":1` and keeps them, as it keeps the outline off.
  KNOWN AND LEFT, under `?findtuned=0` only: a row moved there is written against the built starts (`keep(k)`), so Normal chosen on the
  plain page, then flipped and flipped back under the word, is "at its start" and leaves the line, and the next plain visit is Reversed
  again; and Reversed chosen under the word is tidied out of the line by the next plain visit (it is the start there), so the word visit
  after reads Normal. The same holds for the pan and, since the outline's starts, for the outline. One line cannot be read against two sets
  of starts without that; the word is a way back for a look, not a second profile.
  `bindFindMe` is called after `pageSwitches` (`main.ts`), so the page's camera is handed over with the lead already none; `findMeStarts`
  writes no camera.
  THE HARNESS: the frozen stand is as built. THE SHIPPED STAND boots with the switch, and there the word reaches exactly one thing — a
  scripted turn under the camera's key (`Input.orbitDX` through `ctrlTurn`) goes the other way round. NO STAND HANDS ITS CAMERA OVER
  (`bindFindMe` is the page's alone), so a shipped stand's camera still leads 0.18 toward the aim point while the page's leads none: in
  this one number the shipped stand is not the page as it opens (the lead rides every tick's `cmd.view.aimLead`, and the wall hold's camera
  reads it). Left so on purpose — binding it would move the view of every run on the shipped stand — and for whoever owns the stands to
  settle. No rule of the game reads the pan or the turn; a recording carries the view it was made with.
  NEIGHBOURS' CHECKS MENDED FOR IT: `tween-check.ts` turned its reference camera by a sign written out (`ORBIT_PX * 0.006`) and was red on
  the shipped stand, three lines; the rate is still written out and the way round is now the stand's row (`* FINDME.turn`). `replayturn-check.ts` asked that the row START at 1: it asks
  `FINDME_BUILT`. Three page checks assert the camera as built and now open with `findtuned=0`: `engine-replay-page.cjs` `fog` (Ctrl's turn,
  its sign) and `turn` (a kept `{turn:-1}` is a row moved off the built start; under the page's own it is the start and is tidied away), and `findme-page.cjs` and
  `findme-squad-page.cjs` already did.
  CHECKS: `probes.ts findme` line (12) now also holds the camera handed over (lead none and 100 px turning the view −0.6 rad on a machine that
  kept nothing; lead 0.18 and +0.6 under the word), Normal and the full pan chosen by hand written and still the hand's on the visit after,
  an older line's pan none and reversed read under the word, and the copy of the knobs put back. ALL PASS on both stands, each plain and with
  the word both ways (`TPS_FINDTUNED=1` frozen, `=0` shipped); `FINDTUNED_BREAK=1` failed that line on both (word and line gone). `tween-check` four ways,
  `replayturn-check`, `camspace-check`, `probes.ts shipped` and the `ui` gate (143 lines) pass on both stands.
  `probes.ts all-quick` was NOT run through on either stand (the frozen run was stopped at forty minutes with no FAIL in what it had done).
  `findtuned-page.cjs`, the real page on the software adapter, RUN ONCE at 1024 × 576 on `verylow`: a browser that kept nothing has the
  table at the six tuned rows and its follow camera's lead at 0; the camera's key (Space) held and the mouse really dragged 100 px turns the
  view −0.6 rad there and +0.6 under `?findtuned=0`; Options > Game reads Aim pan None and Turn direction Reversed and Reset keeps them.
  NOT RUN on this change: `engine-replay-page.cjs`, `findme-page.cjs`, `findme-squad-page.cjs` (the first was
  edited: one word on a URL), and every other page check that opens a fresh browser now does so with no aim lead and the turn
  reversed — any that asserts where the look-at sits against the cursor, or which way a held camera key turns, was not looked for beyond a
  grep. NOT MEASURED: nothing of play; no tally moves by reading (no rule reads the two rows) and none was run. NEVER SEEN ON A GPU as a
  fresh browser's start; it is how the maintainer has played on his own machine since 10-20. NOBODY HAS ASKED a first-time player whether
  a view that never leans toward the aim is easier or harder to shoot from, and the row a new player opens on reads "Reversed": the
  labels name the built direction, not the page's.
  SHELVED STILL: `Shape through walls` (no row, no key; a kept line that names it reads without it). A proper one is him drawn once more with the
  depth test off into a mask and filled where a wall hides him: a pass of its own, to be built only once the outline has been seen on a GPU.
  ONE MODEL HOLDS: pan and turn change the seat's view, which is the player's input and rides the record as data (`cmd.ts`), so a recording
  replays the view it was made with; the ring and the outline read his position and his light and write nothing of the game's (a run with both on
  and asked every frame is the run with them off, digest for digest: the picture's own, as the fog of war's pass is; no sense and no meter reads
  the composite after it). Checks: `bun run tools/qa/probes.ts findme` (family `ui`; `FINDME_BREAK=1` = the rows moved and the table not:
  the ring, the outline and the teammates' must fail), the same on both stands: its line (10) runs the outline's pass on a stand-in device
  that writes down what it is asked (nothing while off, one program, no draw until it is in, a scissored draw, never after a failed program);
  `PORT=… node tools/qa/findme-page.cjs` (the real page on the software adapter: the rows exist, move live, are kept over a reload and reset; the
  outline's program compiles there and draws with no GPU error; a real drag with the camera's key held, a real held press, and real double taps, which do nothing; the check asks the page which key that is, `__keys.camCap()`, and
  presses that key, so it follows the page as it opens). The camera's key is the Space bar
  on the page while the board leaves it spare (`KEYMAP.camSpace`, docs/agents/player.md); under `?camspace=0` it is Ctrl as it was. NEVER SEEN ON A GPU by anyone here.
- **The auto camera is shelved (2026-11-08; the maintainer: "i think we can rip out the auto camera for now btw", "we can put that on the queue for a future release").** A double tap of the
  camera's key turned on a camera that framed the door, the squad or the fight by itself and came in as he sneaked up on someone. It left whole, with the double tap: the camera is the hand's, and a tap of its key
  does nothing. Its design, its numbers and its checks last lived at step 2197, commit 069dfa6a511563d3ba394354cbc5fc9945135f1e: `src/host/autocam.ts` (its head is the design), `src/host/loop.ts`, `src/host/findme.ts`
  `AUTOVIEW`, `src/ui/findme.ts` `autoRows`, `tools/qa/autocam-check.ts`, `tools/qa/autocam-page.cjs`, and this file's bullets at that commit. A machine's kept `twopointsix.autocam` and `twopointsix.autotune` are not read.
  NO SWITCH: it is deleted, not held off, so there is no word on either stand and no `FROZEN_HOLDS` row; `?autocam`, `?autoturn`, `?autozoom`, `?autosway`, `?autotune` and `?autolock` do nothing. CHECKS: `camspace` and `findme` re-aimed
  (`CAMSPACE_BREAK=1`, `FINDME_BREAK=1` red on both stands), `tools/qa/findme-page.cjs` holds on the real page that a held press and three double taps turn nothing and leave the cursor on the mouse. MEASURED, on the Rust game, one seed: the hand's camera
  (yaw, pitch, distance, look-at, the cursor) through `stepFrame` over 900 frames of a scripted hand, free step and fixed, both stands, equal to the bit with the head with the mode off; thrice and night-breach:1 equal to the head. NOBODY HAS PLAYED IT, and it has never been seen on a GPU; `tools/qa/autocam-check.ts` and `autocam-page.cjs` left with it, so nothing asks about the mode any more.
- **A mission's page loads behind that mission's brief and waits for Start** (2026-10-19; the maintainer, after watching a first-time player: "having
  the loading screen make more sense (e.g. being a pre mission brief) that you then click through rather than flashing the controls"; that player's
  notes: "stay on loading screen longer, would like to read"). `src/ui/loadbrief.ts`. BEFORE: a page whose address names a mission booted behind the
  old start card's look-alike (`ui/title.ts bootCard`: a grey Start, the notes, the whole controls list, the shader count) and the mission began under
  it the moment the boot was done. NOW that page shows the mission, in a layer of its own (`#brief`, z 30: over the menu and the list, under the
  options and the fatal card): the heading it is listed under, its title, place and one line, the sheet's three cells (Team AS DEPLOYED, Defenders,
  Approach), THE MAP of its level (below; the file did not come = the words alone), and ONE block, START, which fills by the boot's own status line and is
  live when the boot is done. Every word is the mission list's (`desk().brief`, `start_list.rs brief_of`: the row its address boots; deployed at a size no row has, the row of
  that kind on that level; `?solo` alone = the solo night; NONE for an address that picks its level by `?level=` — the editor's own level plays the
  moment it loads, under no other level's words); the only words that are the screen's own are Start, Enter, Controls and Back. IT STAYS
  UNTIL STARTED (a click on the block, Enter or Space) with the world stood up and held still behind it — `main.ts`: `briefHolds()` is one more term
  of the loop's `frozen`, as under the pause card, the quality ladder takes no sample under it, and it counts as a page that hides the view
  (`viewCovered`: the frame is not drawn behind it once the warm-up frames are) — so the run's first tick, its try and its par
  clock are the player's, and the first frames' warm-up is spent behind the brief. A PRESS WHILE IT BOOTS IS KEPT (the block sits pressed in) and the
  run starts the moment it can: the brief never adds wait. A second press takes it back, and so does opening Controls: the run never starts under a
  page the player is reading. The keys are the brief's while it is up (its listener is first at the window: no game key
  and no pause-card Esc is read under it); CONTROLS opens Options ▸ Controls over it (where the keys are from every other screen; shut, the focus is START's), BACK / Esc is a
  page load of the address a quit of that mission leaves, onto the list. It is a layer of the game's text size (`#brief` in `ui/textsize.ts LAYERS`:
  drawn at the window's factor and the player's step like the menus; its viewport units are divided by the factor and its `@media` are px, by that
  module's rule — it had a scale of its own for a day, `--u`, which is gone). NOT FOR A PAGE NOBODY SITS AT: none is made under `?paused`, a replay's tape or a benchmark's scene; one that is up is
  dropped unstarted at the boot's end on a muted test rig (`?mute`), for a show, for a mission whose start was refused (the start card says why), and
  at the loop's next step once a hook that drives the game is called (`__runFrames`, `__mouse`, `__click`, a save or load; NOT one that only
  shoots the page: `docs/agents/qa.md`); the headless harness never comes through `main.ts`. A restart, a
  checkpoint and the next try are not page loads and show no brief. A TUTORIAL mission's brief also carries its row's `scene` and a 'You will learn' list (`teaches`): `docs/agents/missions.md`, the opening card. SWITCH `LOADBRIEF.on` (`ui/loadbrief.ts`): ON in the page, `?loadbrief=0` = the
  boot card and the straight start as the build was; OFF in the module, a page-only screen — no game rule reads it, no knob table
  or journal carries it, no `FROZEN_HOLDS` row: both stands are the build as it was, there being no code of it on either. WHO OWNS WHAT IN THE BOOT:
  this screen is the card line, its branch of the READY block, the `frozen` term, the ladder's guard and its word in `viewCovered` in `main.ts`; when the level is READY
  (`gpu/device.ts`, `engine.init`, `ui/boot.ts`, the status text — keep 'N / M' in it, the brief's fill reads it — `__ready`) is the boot's own.
  CHECKS: `bun run tools/qa/probes.ts loadbrief` (family `ui`; `tools/qa/loadbrief-check.ts`: all 37 rows' words off their own addresses, the screen
  and its hand-over on a fake DOM, `main.ts`'s wiring as text, the switch off on the stand; green on both stands, `LOADBRIEF_BREAK=1` red on both) and
  `PORT=… node tools/qa/loadbrief-page.cjs` (the real page, one job for each kind of mission and one Training lesson, each started another way, then
  `?loadbrief=0` and the automation boot; it runs the page's frames undrawn unless `REAL=1`). MEASURED 10-19 on the software adapter
  (frames undrawn, 1024 x 576, verylow, about 5 minutes a boot): 8 of 8 PASS, no page error — night breach (Enter), the depot room clear (a click),
  the registry rescue (a click made while it booted: kept, started by itself), the counting house quiet (Space), Training 1 (Controls opened, Esc,
  the focus on START, Enter), the solo night (a script's `__runFrames`: the brief gone, its 3 frames run), `?loadbrief=0` (no brief, the old card's
  14 control rows, the run under way unpressed) and `?paused&mute` (no brief at any point); in each brief job the game's clock stood at 0 across
  the hold and ran after the start, with no key left held and no pause card. Night breach was also run once with its frames DRAWN (`REAL=1`,
  about 9 minutes after the boot) before the review's mends: the same verdict. The brief's buttons were also driven in a page with no GPU boot at
  all (a press kept, taken back by a second, taken back by Controls; a chorded Esc opens nothing; `?level=local` and `?loadbrief=0` show the old card).
  NOBODY HAS SEEN IT ON A GPU OR PLAYED IT: the brief itself is plain DOM and was looked at as Chromium lays it out at 960 x 540, 1280 x 720, 1920 x
  1080, 2560 x 1440, 3440 x 1440, 3840 x 2160 and a 700 px wide window; how long the hold feels, the fade into the first frames, and the sound that
  starts on the press are unjudged.
  THE MAP, NOT THE PICTURE (10-20; the maintainer on the first cut, seen on his own machine: "better but let's show the map vs. the (kinda low res)
  level art"): beside the words is the mission list's own map sheet of the level (`ui/missions.ts drawMissionMap` → `drawPlan` / `ui/planArt.ts
  drawSheet`: the place as a model, a 10 m scale, the way in the address asks for marked as the list marks it — `wayOfLink`, one fold for the list
  and the brief), chosen over the flat plan and the maps of play because it is the one that shows the ground and where the team starts before a run
  exists. It is the level AS THE PAGE LOADS IT, so the furniture on the map is the furniture of the run, and the file is the one
  the boot fetches anyway. It is drawn in the screen's own pixels (the canvas is its shown width × the text size factor × the device's pixel ratio,
  480 to 2880, redrawn on a resize or a text size step), so it is sharp on a dense screen where the 760 px picture was not; its box is the sheet's
  40 : 28 at the largest width the column and the window's height leave, so nothing of it is cut in a short window or letterboxed in a narrow one;
  a file that did not come is asked for again at the next resize; the canvas's pixels are let go when the brief goes. The mission pictures
  (`ui/menuStills.ts`) are no longer shown here; the cards keep them. KNOWN: a window dragged to a screen of another pixel density with no resize
  keeps the old density until one; the parsed level stays in the list's plan cache for the page's life, as it does after a visit to the list.
  CHECKED 10-20: the probe's map line on both stands (the level's file by the stand's look, the way in exact, the sheet's shape, its pixels let
  go); laid out in a real browser page (no GPU boot) at 1300 x 780 at 2 device pixels, 1920 x 1080 (dressed and `?look=plain`), 3840 x 2160, the
  largest text step, 1920 x 700, 1400 x 480, 700 x 900, 820 x 600, and with the level's first fetch failed then a resize — the canvas is the shown
  size in device pixels (or its 480 floor) each time, inked, inside the window, nothing scrolls; `loadbrief-page.cjs` holds the same on the booted
  page (software adapter, 1024 x 576: night breach, the depot room clear, the counting house and Training 1 PASS with the map of their own level
  drawn at the shown size; the registry rescue, started early, PASS). The map on the brief has not been seen on a GPU page or by the maintainer yet.
- **The brief's Start waits for the picture** (2026-10-28; the maintainer, after an evening on the hosted build with the first-frame start on: "i also
  quite dislike the 'popin' that happens from starting the game too early when some shaders haven't finished"). `src/ui/settle.ts`, `main.ts
  briefSettle`, `LoadBrief.hold` in `ui/loadbrief.ts`. BEFORE: the block was live the moment the level stood (`__boot.marks.interactive`), before one
  frame of the level had been drawn, and a press kept from the boot started the run there. The run's first seconds were then the renderer's: a pass on
  its stand-in while its program compiled (the row 'Start on first frame shaders'), the second (cut) set of the tracing programs compiling in the
  background (it always did, on every load), every history empty, and auto quality measuring in view and changing preset and render scale in one step
  about a second in. NOW the brief HOLDS from the level standing until (1) every program is in (no pipeline asked for and not made, none of the later set
  out, the cut set built or failed: on such a page the cut set's build is one of the later set's holds, so `__ready`, `laterReady` and auto quality's
  wait include it), then (2) auto quality has made its pick and 45 frames have been drawn AFTER it (a pinned preset: the 45 alone). The frames are
  drawn behind the brief only in turn (2); before it the still under the brief stops after `behind.warm` as under any cover. The world is still, the block is not live, the status line
  goes on ('compiling shaders N / M …', then 'getting the picture ready …'), a press is kept and starts the run when the block goes live. Ways out: 120 s
  on programs (a compile that never returns; `?settlewait=N` for a page tool on the software adapter), 12 s on the picture (a page too slow to
  measure), both by the wall clock; the time a tab spent in the background is not counted (the clock starts again on `visibilitychange`). While it holds, a Windows browser's compile queue runs at the boot's number, background set included (`pipeQueue.rush`):
  nobody sees the frames the two smaller numbers spare, but the brief's own page is as busy as the boot was (unmeasured on Windows). `__boot.marks.settled` = when the block went live. A VISIT'S RECORDS: `cloud.frame` and the visit's `live` fact no
  longer count the frames behind a held brief as play (they did, before this too: a perf record's first minute held the brief's seconds).
  SWITCH: `SETTLE.on`, ON in the page where a person sits
  (`?settle=0` = live the moment the level stands, as it was), OFF in the module and on a page a script drives (`navigator.webdriver`: the page
  tools press START the moment the level stands; `?settle=1` raises it there, a number over 1 is the frames); a page switch like the brief
  (no knob, no FROZEN_HOLDS row, no stand's word: the harness has no page). ONLY the brief: the title's start
  card, `?loadbrief=0` and an address that names no mission of the list start as they did. CHECKS: `bun run tools/qa/settle-check.ts` (the rule on a
  clock moved by hand and main.ts's places, 11 lines; `SETTLE_BREAK=1` fails 9), `loadbrief` (two lines for `hold`), `pipequeue-check` (three for the
  rush), all on both stands; `PORT=… node tools/qa/settle-page.cjs` (the real page in a browser that does not say it is driven, `?settle=2&settlewait=3600`:
  not live at the level standing, a press kept, live only with the cut set ready and the later set in, whose own mark `laterReady` is a timer's turn behind the count the hold goes by and so may land just after `settled`; `SETTLE_BREAK=1` must fail). WHAT IT COSTS:
  a mission's Start is live later by the cut set's build on a visit's first load (on the kept page a later mission has it already) plus about a second of
  frames on every load. UNMEASURED: both numbers on any GPU (the software adapter takes minutes over a frame); how long a Windows machine holds the
  first brief with the queue rushed. WHY IT WAS BUILT ON INFERENCE: his three loads that evening (an M4 Max) had every later program in at the moment the
  block went live, so the pop-in he saw was not a stand-in; what the records do show is the first seconds of each mission slow (32 frames over 100 ms in
  the first minute of one, 16 fps for the 6 s he stayed in another) with the GPU's own time a frame at 7 to 10 ms; and those counts held the frames
  behind the brief, so how much of it was play is not known. Nobody has seen this on a GPU or
  played it.
- **The game has its own text size, and the page no longer zooms** (`src/ui/textsize.ts`, `TEXTSIZE.on`, 10-19; a play test on a 3840 × 2160 screen:
  the words were too small to read, the browser's zoom keys grew the page until the layout broke, and the mission select's names were cut). ONE FACTOR,
  `--tz` on the root (which wears the class `tsz` while the switch is on), drawn as CSS `zoom` on every layer of the page that carries words (`LAYERS`:
  the menu, the Play page, the options, the fight screen's read-outs, the marks over the picture, the debugger; never the picture's canvas). WITH NOTHING
  CHOSEN it is the window over 1920 × 1080, never under 1 and never over 3: 3840 × 2160 draws at 2, 2560 × 1440 at 1.33, 1920 × 1080 and anything
  smaller as it always did. A page zoomed from the browser's own menu is a smaller window, so the words stay the size they were. THE SETTING is Options ▸
  Graphics ▸ Display ▸ Text size: five steps drawn as the letter at each size (0.85, 1, 1.15, 1.3, 1.5 times the window's), live while the page boots,
  kept per machine (localStorage `twopointsix.textsize`, the step's index; nothing kept = the window's; Graphics ▸ Reset forgets it). A step never
  takes the layout under 1200 × 675 of its own pixels (the smallest window the screens hold whole at, measured with the page check on the build before):
  where the largest step asks for more room than there is, the larger steps share what there is (1366 × 768: 1.04, 1.08, 1.14; 1280 × 720: up to
  1.07), and on a window of 1200 × 675 or less they do nothing. THE KEYS: Ctrl or ⌘ with + and − step it from anywhere, with 0 it is the window's
  again; those chords, Ctrl + wheel and a pinch are taken from the browser (over the picture a wheel that says Ctrl is still `game/input.ts`'s: a pinch zooms the camera, the wheel under a held Ctrl key is nothing, `player.md` CTRL AND THE WHEEL).
  FOUR THINGS A ZOOMED LAYER DOES NOT DO BY ITSELF, and a new screen has to: a viewport unit in a layer is written `calc(25vw / var(--tz, 1))` (the
  check is red on a bare one in `index.html` or `src/ui`); an `@media` width or height is rewritten by the module to ask for px × factor (also when a
  style sheet joins later, one microtask after it joins), and a script asks `uiMatch('(max-width: 720px)')` in place of `matchMedia`; a rectangle a
  script measures off a zoomed element is in window pixels and one it writes into a layer is in the layer's, so divide by `rectZoom()` (the speech
  bubbles' blockers, the debugger's tooltip, the kit picture's width), and a size read off the window or the picture's canvas by `textZoom()` (the
  overlay's marks, `uiSize()`); a 2D canvas inside a layer multiplies its device pixels by `textZoom()` or it is stretched (the
  debrief's map, the wind graph do). A new layer of the page joins `LAYERS`. The replay player's own bar, map and sound lanes are NOT layers (they
  stay the size they were); its rules that move the fight screen's read-outs clear of them divide the bar's window pixels by the factor.
  THE MISSION SELECT'S CUT NAMES (the play test's other note) had a cause of their own: the grid of cards has a fixed height and its lines shared it,
  so on a tall window with few lines the picture (16:9 of a wide card) took the whole line and the name and tags hung out under the next line's picture;
  at 3840 × 2160 no name could be read, at 1920 × 1080 the tags were cut. With the switch on a card's line is `minmax(0, 1fr) auto auto` with a floor of
  the picture cropped to 2:1 plus its captions: the picture gives way before the words do, then the grid scrolls. The screen's look at 1920 × 1080 and
  under is otherwise as it was; with the switch off it is the build before to the rectangle.
  SWITCH: ON in the page, `?textsize=0` off (no factor, no class, no row, the browser's own zoom keys, every rule as written; `var(--tz, 1)` is 1), OFF
  in the module, never in the Team & kit picture's own window. It is a look of the page, not a rule of the game: nothing under `src/game` reads it, it
  is in no knob registry, journal or digest, the headless harness never boots it, so there is no stand word and no run can move. Hook `window.__textsize`
  (`on`, `zoom`, `step`, `set(i | null)`, `layers`, `table`, `want(w, h, i)`, `media()`). CHECKS: `bun run tools/qa/probes.ts textsize` (family `ui`:
  the factor's table, the browser-zoom invariance, the floor and the shared room, the media rewrite, the keys, the lints; `TEXTSIZE_BREAK=1` fails,
  the same on both stands) and the real page, `PORT=<dev server> node tools/qa/textsize-page.cjs [outDir]` (`WAS=<port of a server of the build
  before>` holds `?textsize=0` to it; `GPU=1` adds the pause card and the fight screen on the software adapter; `TEXTSIZE_BREAK=1` fails).
  MEASURED 10-19 in headless Chromium with no adapter: at 3840 × 2160, 2560 × 1440, 1280 × 720, 2560 × 1440 at 1.5 device pixels and 1920 × 1080 at
  2, and at the largest and smallest step, no text cut or covered on the title, the six groups of the mission select (33 cards), Team & kit or the
  options, and every `@media` rule asked at its px × factor; the row, the keys, the wheel, the kept step across a load; `?textsize=0` at 3840 × 2160
  equal to the build before in 588 text rectangles. On the SOFTWARE ADAPTER, one boot of the solo night at the three sizes: the kit, the meters and the
  objective inside the window at the window's factor, the aim ring over the same point of the picture at the largest step as at none, the options
  over the pause card whole (the pause card's run tally already ends three long labels in an ellipsis at 1280 × 720 on the build before; the check
  holds the card to that). NEVER SEEN ON A GPU, and nobody has played it: not the sharpness of the words or of the three canvases, not the kit
  picture's window inside the zoomed Play page (no adapter here draws it), not a real pinch or a real Ctrl + wheel. Chromium only: Safari was never
  run (the boot asks the engine which pixels it measures a zoomed element in, and that answer has only ever been Chromium's). The replay player was
  not run with a tape after its six rules changed.
- **Options ▸ Controls rebinds the keys (10-21; `src/host/keymap.ts` the table, `src/ui/keybinds.ts` the caps, the remap itself in
  `src/game/input.ts`'s keyboard listener).** The maintainer: "could you add key remapping (plus reset) to settings? that way i could tweak
  settings without a rebuild". WHERE IT SITS: at the keyboard and nowhere else. The listener asks `gameCode()` what a key IS before it files it,
  so the game still asks for the codes it always did and the tick's record, the session journal, a run's tape and a replay hold those codes:
  a recording made on a rebound board plays the same on any machine. It is no rule of the game: no knob table, no `FROZEN_HOLDS` row (the
  harness's Input has no keyboard), and OFF in the module, so a headless tool, the level editor, the viewer and the workbench keep the
  build's keys whatever the machine has kept; the game page turns it on in `main.ts`'s first line (`keysBoot`). OFF ON THE PAGE:
  `?keybinds=0` (the build's keys and the plain list for the visit, the kept table left alone). KEPT in localStorage `twopointsix.keys`
  (only rows not as built; cleaned row by row as it is read). PAGE HOOK `__keys` (the module: `bindKey(id, code)`, `resetKeys()`,
  `keysKept()`, `setKeys()`). WHAT MOVES: the 26 rows of `KEY_ACTS` (W A S D, Shift, C, Q, Space, V, N, R, Z, L, E, 1 to 5, X, B, T, F, G, H,
  the backtick). It is the KEY that moves, not the verb: E still puts away and swaps a hold, V still whistles and is the order menu's wand
  row, the digits are the kit and the menu's rows. H and G (rows of the order menu on letters) got rows on the list so that a key given to
  another row is never theirs unseen. WHAT STAYS: the mouse and the wheel, the camera's Ctrl, and the page's own keys, which no row can take
  (Esc, Enter, Tab, the arrows, the F row, Ctrl / Alt / Cmd, P [ ], Caps Lock). THE SCREEN: a movable cap is a button (the fixed ones are
  drawn hollow); a click, or Enter on it (W S walk the caps), and it waits for the next key. A key another row has SWAPS the two and the
  other cap flashes (a row with two keys in the build, Shift, keeps its other one); with no key to give back the other row is left with
  none and reads — , dashed. Backspace or Delete = no key. Esc or a click elsewhere = nothing changed. A page's own key shakes the cap,
  which goes on waiting; the F row passes through. Reset to defaults (that tab's button) = every row as built, nothing kept. THE NAMES
  FOLLOW: `keyCap` for a cap the HUD is handed (the Space rows and the order menu's keys, the kit column's slot numbers), `keyText` for keys
  named in words (the rows' hints, the order menu's hint, the objective line and so Training's steps, the kit column's L and N tags, the
  order mark, captions, the pause and down cards' foot lines, the old start card's list). `keyText` reads finished words and that is a
  guess: a moved cap standing alone is the key unless the words round it make it a place's letter ('room B', 'Vault B', 'B wing',
  'T-junction'); a digit only where the words say key ('then 3', '2 again'); W A S D only in a list of keys. KNOWN TO BE LEFT AS BUILT:
  '1 to 5 picks an order', the tour's 'WASD and mouse', speech bubbles, the debugger's own tooltips, a thing's name on the Space rows, and
  the Tab trial's 'as T' follows only on the Options list. Lines that handed the HUD a code instead of a letter would need no guessing;
  they are written all over the game and that is a pass of its own. NOT THE KEY'S LETTER ON ANOTHER LAYOUT: a cap is named by where the
  key sits on a US board, as the build's own list always was. CHECK `bun run tools/qa/probes.ts keybinds` (family `player`), on BOTH stands
  (`TPS_PROFILE=shipped` in front for the page's): the table (off in the module, the boot and `?keybinds=0`, a swap, 4,000 moves by a
  die leave no key with two rows, the page's keys refused, no key, Reset, the store written and read back, a bad kept table cleaned, the
  words); the keyboard listener on a window that is a list of listeners (nothing rebound = exactly the codes the build filed; rows moved
  = the same codes off the new keys; a dead key lets go of nothing; a key rebound while down lets go of what it put down); and one script
  of hands (330 ticks: walk, crouch, sprint, the rifle, a reload, the wall, holster, Space) played three ways into a fresh game — codes
  written straight in, the keyboard as built, a board with 12 rows moved — ending in ONE digest on each stand. `KEYBINDS_BREAK=1` (a stray
  kept table while the 'nothing rebound' lines are asked) fails on both. ON A PAGE `PORT=<port> node tools/qa/keybinds-page.cjs [outDir]`
  (headless Chromium, real key presses; the screen needs no adapter, `GPU=1` adds the game on the software adapter, two boots,
  about twenty minutes; `ONLY=game` leaves the screen out). NEVER SEEN
  ON A GPU, and nobody has played it: not the feel of any moved key, not a Mac's keys (Cmd chords, a trackpad), not Safari or Firefox.
  ROWS THE BUILD HAS ELSEWHERE WHILE EMPTY HANDS ARE A SLOT (`KEYMAP.slotWhen`, `SLOT`, `ALIAS`, 10-22): the slot's row (code `'Digit0'`) on backtick,
  the debugger's on backslash, E's row with no key and its letter in a line read as the slot's key. The bullet is in `player.md`.
  A ROW THE BUILD GIVES NO KEY (`KEYMAP.qFree`, the wall hold's, 10-21): `home(a)` is a row's keys in the build, and 'as built', the swap, the kept
  table and Reset all measure against that, not against the row's code. The bullet is in `stealth.md` ('Q is free').
- **A mission that leans loud is not graded as a sneak (10-22, `VERDICT.loudCard`).** The maintainer, off the solo tutorial's end card ('ASSAULT 14'
  on a won fight): 'weird that we show a score (and a low one) for loud missions'. The stealth game's rating prices every man shot and every time
  he was seen; a row of the list whose `lean` is `'loud'` but whose kind is the stealth game's (`tut-solo`, `train-kit` today) ended on that card.
  `game/verdict.ts loudRow(g)` = the key up, a quiet mission up, and its row leans loud. The game is not told the page's `levelFile`, so the row
  is found off the level's own name (`rowFile`: `<name>.json`). `chronicle.ts
  debriefOf` carries it as `Debrief.scored` and `Debrief.grade` ('GHOST 94', '' when not scored). Not scored: the card (`ui/debrief.ts`, class
  `db-loud`, which nothing styles) builds no rating word, no score, no line of what was docked, no badges, and the sneak's cells (seen, alarms,
  floor up, bodies found, kills, doors kicked, called in, checkpoint loads, the civilians' saw / told) stand untoned — a hit taken and a civilian
  hurt keep their red; the copied line opens on the clock; `mission.ts`'s two end lines ('exfiltrated', 'mission complete') read `grade` and stop
  at the clock. The numbers, the timeline and the map stay. A row that leans quiet or either way, the plain night, and every squad kind (its end
  card never showed the rating; its 'run so far' still does) are as they were. Reads only: no run's digest differs.
  ON in the page (`?loudcard=0` = the rating on every card it stood on); off in the module and on the frozen stand (`TPS_LOUDCARD=1`); a
  `FROZEN_HOLDS` row; a recording carries it (`VERDICT` is the last name of `JOURNAL_KNOBS`: `mission.ts`'s two end lines are worded by it), and
  one from before the key, or the table, replays with it off (`journal.ts applyKnobs`). CHECK `probes.ts debrief-loud` (the
  rule, the key down, a quiet row, the plain night, a room clear on a loud row, every row's level file and its look's file read back as the row's
  file, a recording with and without the key, mission.ts's two lines) and a block of `debrief-dom` (the card on a fake DOM, key up and down, counts that would tone every cell);
  `LOUDCARD_BREAK=1` fails both on both stands. NEVER SEEN ON A GPU; nobody has played to the card since. Left as it was: the stealth game's
  cells and timeline still stand on a loud row's card (a loud card of its own would be new work).
- **A row with its own hour or weather draws no Hour or Weather row** (2026-10-22; `src/game/rowset.ts` `rowHour` / `rowWeather`, `ROWSET.on`,
  `?rowset=0`; the rule and its check are in `missions.md`, "A mission row can name its own hour and weather"). What does not apply is not drawn: H and J
  do nothing on such a row, the start page's Setup rail follows (it already skips a row that is not drawn). A row that names only
  one keeps the other row, whose note reads the mission's own for the hidden one. Its LINK is any row's link: the visitor's Hour and Weather picks stay
  on it and on every later link, and the row's own win when the page boots. The debugger's 'weather kind' buttons say so and do nothing on a row with
  its own weather. No row names one yet.
- **How many are inside is a mystery, and the large map is left by a click beside it** (10-23; `src/ui/kitBrief.ts`
  `KITBRIEF.outside`). The maintainer, off the Team & kit step with its map opened large: "clicking outside the map should close it. we should also
  remove all defender counts from the menus, etc., it should be a mystery!" **THE COUNT**: no screen draws a
  Defenders cell: the Mission step's sheet, the same cell on the Team & kit step (`kitBrief.ts`) and the fact on a mission's load page
  (`loadbrief.ts`), and the pause card's intel cell says how many were
  questioned without "of N". The facts row loses the cell and not a gap (`.mfacts.nodef`: three columns on the old sheet, no row at all on the one
  starting page's Mission step, where Defenders was its only cell, nor on a kit band with nothing to say of locks or hostages). A row's `defenders`
  number is untouched: the stand-up, the picks probe and the levers still count on it. THE WORDS
  are changed in the data and have no switch (a wording change, like the writing guide's): the rows that said a head count say the place instead
  (`scene/scenarioList.ts`: the pair's break room, the block's squads, the warehouse, the compound, the barracks, the kill house; `scene/picks.ts`
  the warehouse's line), Hard's sentence names no number of guards (`game/difficulty.ts`), and the kit lesson's first card no longer says the band "shows who holds the
  floor": it sends the player to the map, where the job is read (`ui/kitGuide.ts`; true in a short window too, where the band has no room for the job). KEPT ON PURPOSE: a smaller number that is the puzzle's premise ("two sentries who watch each
  other", "a gunman waits in one of them"), every Training and tutorial row (the number is the exercise; the lessons' own steps are their
  owners'), and the end-of-run card's "N of M down" (after the fact). NOT TOUCHED, and claimed to their owners: what the enemy SAYS during a
  mission (the last-stand call "{n} of us left", the staff radio check's roll of callsigns, a questioned guard's "{N} of us tonight", the response
  team's "{n} of us", the numbered Tango callsigns on the radio strip). **THE MAP** (`?mapout=0` = as it was: a click on the map, M, Esc, Enter or
  Space): large, the sheet reads as a card laid over the step, so a click anywhere off it puts it back. The click, and the press that begins it, are caught on the document before
  the step hears them and do nothing else: a teammate's slot under the pointer is not picked, a kit card does not take the keys, the figure does not
  start to turn. Small, a click on the step is the step's own; so is a click made after the window was narrowed until the sheet is no longer drawn. The
  map's switch is UI only: nothing under `src/game` reads it, no recording carries it, no frozen row. (The sheet still makes the Defenders cell, empty and hidden, only because the walk `tools/qa/cards-page.ts` counts it.) CHECKS: `bun run tools/qa/mystery-check.ts`
  (also `probes.ts mystery`, families ui and words: no row, pick line or Hard sentence says a head count, no file under
  `src/ui` reads a row's `defenders`; `--list` prints the lessons' numbers it does not judge; `MYSTERY_BREAK=1` must fail), the
  missions-screen probe (the sheet's cell with the switch on and off) and `PORT=<dev server> node tools/qa/mystery-page.cjs [outDir]` (the real
  page in headless Chromium, real keys and clicks at 1440 x 900 and 1000 x 673: no cell and no count word on the kit step, the large map's
  caption, the Mission step and three load pages; the click beside the large map on a teammate's slot; each switch off; `MYSTERY_BREAK=1` must
  fail). SEEN: those two runs and stills of the three screens, on the software adapter. NOBODY has seen it on a GPU or played it, and the pause
  card's intel tooltip was read in the code only.

- **Tab shows the objectives and the squad** (2026-10-23; `ui/tabcard.ts`, `main.ts tabKey`, `index.html #tabcard`; the maintainer, with
  the corner panel cut down to what the two keys do: "if you don't have a menu open, tab opens up an objectives modal that also has your squad
  equipment?"). With nothing to back out of (Tab is the back key first: `game/backkey.ts`; the card also stays down over anything `game.backable()`
  names where `?tabback=0` has the back key off) and no menu open, Tab HOLDS THE GAME (`main.ts frozen`, as a pausing card does) on a card in the
  middle of the screen: the OBJECTIVE under its own word with the par or clock at the right, its sentence, and where it lists (`objectiveList`,
  only under the word 'objective': a room clear's rooms) each thing with a tick (taken), an arrow (the one being called) or a ring (to do); the
  SQUAD with the block's own title note at the right (loud or quiet, the formation), a line a man in the squad's order with what he is doing
  and what he covers (`orders.statusOf` and `cover.tail`, the block's own line past his name) and under it his kit in a line, a man lost struck, then the block's plain notes (what the squad carries between them, a
  charge set, who waits on him). A man's kit is the STAGED card (`kit.ts kitOf`, then `kitWords(i, m)`): what this run gave him, not this machine's
  table; its counted things read 'started with 1 stun, 1 charge' (`tabcard.ts staged`), because throwables and charges are one squad
  pool once the run is on and the live count is the note under the men. His own kit has no section (the slot bar shows it). Tab again or Esc goes back (Esc alone does not open the pause card over it); a menu,
  Options, a lesson's card or a script that steps the page puts it away. F5 under it puts it away and saves by hand, as on the pause card. Every word is one the game already makes, through `keyText`; the card lays them out once
  as it comes up (not a live readout) and scales with the text size (`textsize.ts LAYERS`). It is the home of what the corner panel stopped saying
  (`docs/agents/player.md`, the corner panel). NO ROW AND NO WORD SINCE 2026-11-08: it is what Tab does on every page, a page a script drives too (a check that presses Tab with nothing to back out of gets the card, put away on the next `__runFrames`). A kept `tab` or `tabPicked` in `twopointsix.options` is read past and dropped when a row is next moved. No rule of the game reads it and nothing of it is in a recording: under a replay or a script that
  steps the page (`__runFrames`) the hold does not apply and the card is put away on the next frame. CHECK: `tabcard` (family `ui`), both
  stands, `TABCARD_BREAK=1` red: the choice, what settles it and what is kept, the list, the card of a room clear with the squad of four (a
  staged kit, a man lost), the page's wiring read as text. DRIVEN on the real page on the software adapter (`tut-squad`, a room clear, four men,
  `?tab=brief`, no picture drawn): Tab put the card up at 560 x 422 in the middle with the six rooms, the four men each with his words and kit and
  the two carry notes, the clock stood still under it, Tab took it down and the clock ran, Tab put it up again, Esc took it down with no pause
  card; the same page without the word showed nothing on Tab; no console errors. MEASURED: nothing; it moves no tally. DECLINED, from review: the card as one of
  `overlay.card`'s (those are a lesson's, with its own key and locks) or a page of `#menu` (the menu is the pause, with its own Tab); the
  objective handed over in parts by the Rust game's `mission::objective` (the line is the one source the top line, the corner and this card all read; the list is cut
  from it only under 'objective'); a place in `bubbles.ts BLOCKER_IDS` (the card is above the bubbles by z-index and holds the game); one page-level 'what holds the game' reader
  for the load brief, a lesson's card, the menu and this card (a change to `main.ts` wider than this card; the check pins each site it added);
  `picked` and `kept` as one field (the kept file's two keys are already on machines); the menu's HTML escaper shared (a private one-liner of a
  neighbour's file); the break reaching the list and the page lines (both are fixed inputs: a break there would be by fiat). NOT DONE: a
  lesson's frame does not teach or lock it (the frame's key locks do not know Tab's card); nobody tracks what a man himself has left (the pool is the squad's), so a man's counts stay
  what he started with; a run's end line still says to press Enter while the card holds the game (Tab or Esc first). Never seen on a GPU. NOBODY HAS PLAYED IT.
- **Enter deploys a mission that has no kit step (2026-10-23; `ui/startpage.ts` `noKit` / `onward`).** The maintainer: "bug for tutorial 1 where hitting
  enter (rather than clicking deploy) goes to an empty loadout screen rather than starting". A mission played with the kit as issued (`kitAsIssued`: a
  tutorial mission with no kit plan of its own, `.nokit` on the page) draws Deploy as the block on the MISSION step, but Enter on its row still stepped
  into TEAM & KIT, where there is nothing to pick. Now nothing on the page leads there for such a mission: Enter or Space on its row is Deploy, the key
  line under the list reads "Enter deploy", the head names one step, the rail has no Team & kit row, and `setPane` itself refuses the kit step, so
  `?loadout=<that mission>` opens on its rows. A mission with a kit steps into it as before. No switch (a page's navigation; nothing a run reads).
  CHECK: `tools/qa/startpage-look.cjs`, two lines ("a mission with no kit to pick", "a mission WITH a kit still steps into it"), on the real page with no
  adapter under it; the look's group name 'Training' was stale since the old lessons went and reads 'Tutorial' now. Nobody has pressed it on a GPU page.
- **The night breach is cut from the mission list and from the short list** (2026-10-24; the maintainer: "let's also remove "night breach" since it's
  unnecessary now"). `night-breach` carries `offList: true` (`scene/scenarioList.ts`) and its row is out of `PICKS` (`scene/picks.ts`: six rows now —
  compound, registry, warehouse, depot, barracks-day, solo; the barracks' "next" is the solo night). THE SWITCH is the cut rows' own, `OFFLIST_UI.on`:
  `?offlist=0` draws the row again (the short list's row does not come back with it); off in the module and in every harness, so every list probe still
  pins the whole table. NOTHING IS DELETED, and little could be: the row, its address `?nightbreach`, `?loadout=night-breach`, its picture and its
  `scenarios.nightBreach` block stay; `game/nightbreach.ts` is the staging the room clear and the rescue are built on (the team, the pairs, the stack);
  about a hundred files under `tools/` stage the one-room fight by its id (moves, wounds, kit, restart, the co-op and threshold fixtures), and a kept run
  or a recording of it still loads. WHAT STILL NAMES IT TO A PLAYER, left as it was (not this step's): the Hard difficulty's long line
  (`game/difficulty.ts`: "On the night breach the defenders are drilled…"), and the debugger's section 'Room fight staging (night breach · every room
  clear)' with its start button. The Raids tile's picture is its first listed mission's, as it always was. CHECK: `probes.ts offlist` (the table's cut
  rows are the five named, and line 6, new: no row of the short list starts a cut row; `OFFLIST_BREAK=1` fails it), both stands; the real page by
  `PORT=<port> node tools/qa/unplayed-page.cjs` (no adapter under it: 31 cards, none the night breach's; 36 under `?offlist=0`; no page error) and its
  still of the Raids group, six cards. `startpage-look.cjs` and the verify
  notes list six. Nobody has opened the page on a GPU since.
- **Options offers no dead order who control** (10-25; `src/ui/options.ts` the who row). It follows the squad's rule and has no switch of its own: `game/cmdring.ts` `wholeSquad` (the maintainer, 10-24: "quick Q mode should always do the whole squad. you should only be able to issue commands to parts of the squad in plan mode"; `?wholesquad=0`). THE WHO ROW on Options ▸ Controls ("Who takes the next order", key id `KeyT`, a dashed cap since Q and F became the squad's only keys) is not listed while it has no key: there is no cap to put a dead key on. A row the player gave a key earlier stays his to see and move, and its small words say the key does nothing now (the list is built again when the row loses its key: Reset, Backspace on its cap). CHECK: `PORT=<dev server> node tools/qa/tabwho-page.cjs` (Options ▸ Controls on the real page: no row, a hand-bound row, the row under `?wholesquad=0`; `TABWHO_BREAK=1` must fail). SEEN: the Controls tab in headless Chromium with no adapter. NOT SEEN: a GPU, a played game.
- **The menus name no plan mode, and the Controls list says the squad's keys as the page has them** (10-26; `src/ui/options.ts` `squadRows`,
  `listNow`). The maintainer, 10-26: "let's remove plan mode and update the tutorials accordingly". No switch of its own: it follows
  `game/cmdring.ts` `only` (`?qonly=0`: the Squad rows as they were) and `goals` (`?goals=0`: no row for a part of the squad, the "Plan mode
  circles" row back, the who row's plan mode words back); both are off on the frozen stand. (With plan mode out of the game
  altogether, the squad's own change, `?goals=0` brings neither of those back: a hand-bound who key then says every order goes to the whole squad.
  Both checks below pass on either build.) OPTIONS ▸ CONTROLS, Squad, on the page's own board is
  three rows: the command key's, "Order the squad" (its tap gives the usual order where the cursor is, kept down a moment it opens the wheel with
  every order); then, with `goals`, the sprint key with that key, "Send part of the squad to one more thing" (both caps are those rows' own, so a
  moved sprint key is named there: the game reads the sprint row's code, `game/input.ts` gameCode); then the word's, "Send in everyone who's
  waiting". The orders menu's row (code `KeyX`, no key and no middle button on this board) and the two rows that are keys of that menu alone (H
  and G: nobody's while a wheel is up, nothing with no menu up) are listed only while the menu has a key, kept from before; its words then leave
  the middle button out. A who row given a key by hand says the key does nothing now. The list is built again, at the same scroll, when one of
  those facts changes under it. OPTIONS ▸ GAME: "Plan mode circles" is shown only while `goals` is off. KNOWN GAPS: a menu, a who
  key or the menu's two letters cannot be given a key from Options on a fresh machine (a kept one keeps its row), and H and G are no rows at
  all while the menu has no key (the next bullet). A tutorial mission
  still has plan mode until its lines are cut, and Options says nothing of it there. CHECKS: `bun run tools/qa/squadkeys-check.ts` (also
  `probes.ts squadkeys`, family ui; the screen's own rows through the pure function, and its source read for the two guards;
  `SQUADKEYS_BREAK=1` must fail) and `PORT=<dev server> node tools/qa/squadkeys-page.cjs` (the real page: the three rows, a hand-bound who row, a
  moved sprint key, a menu on a kept key, `?goals=0`, `?qonly=0`; `SQUADKEYS_BREAK=1` must fail); `tools/qa/tabwho-page.cjs` and
  `tools/qa/keybinds-page.cjs` each have a line brought up to date. SEEN: the Controls tab in headless Chromium with no
  adapter, and one picture of the tab at 1440 wide. NOT SEEN: Options ▸ Game as drawn, a GPU, a played game.
- **The old menu's letters go with its key (10-26; `host/keymap.ts` `KEYMAP.letGo`, `LETTERS`; only with `qOnly`).** With the G and H rows off the
  Controls list the two rows still sat on G and H in the key table, so a key put on H was a SWAP with a row no list shows: crouch put on H left
  the hidden row on C, kept, and C gave the game the menu's letter from then on. Now, while the ring carries every order and the orders menu's
  row has no key, those two are ASLEEP: no key, nothing to bind (`bindKey` says 'off'), G and H on the board spare (they give the game nothing
  until a row is put on one, and that takes it from nobody); a line of words that names one is left as written. The menu on a key (kept from
  before): both are rows again on G and H or where he put them, listed, rebound and swapped like any other, never onto the menu's own key.
  KEPT AS HE SET IT: what is kept for a sleeping letter stays kept, through a load and a visit on another board, and means the same when the
  menu is back; a key given to another row while the letter slept is that row's and the letter's entry goes with it. THE FACTS ARE LIVE
  (`onlyWhen`, `throwWhen`, handed by `main.ts`): the debugger's "the ring ▸ carries every order" unticked, or an older recording played, and
  the letters wake with the old menu; the quick throw on (`?quickthrow`, the debugger), and G is awake as its key. SWITCH: ON in the page,
  `?menuletters=0` = both rows on G and H as they were; OFF in the module, so the headless stands and every recording are untouched (the remap
  is at the keyboard; no knob table, no `FROZEN_HOLDS` row, like the rest of the keymap). THE HOOK `window.__keys` is set as the keys boot, not
  when the boot ends: the Controls screen is up before that and its check asks the table. A waiting cap whose row goes away under it lets go.
  CHECK: `probes.ts keybinds` has the line on both stands (`LETTERS_BREAK=1` must fail; 4,000 moves by a die with the menu's key and the two
  live facts going on and off leave no key with two rows, no sleeping letter keyed, and every kept table read back the same board), and
  `tools/qa/keybinds-page.cjs` asks the real page that no row with a key is missing from the list. `qkey-check` and `qonly-check` count G and
  H among the keys that give nothing. KNOWN GAPS: with the quick throw on, G is a live key that no list shows while the menu has no key (the
  old flaw, on that key, on that visit: the list is `ui/options.ts squadRows`); a row a letter took a key from stays without one when the
  letter goes to sleep (its dashed cap is on the list, and Reset is offered). NOT SEEN ON A GPU, not played: nothing here draws, and the
  game's part of the page driver was not rerun.
- **The title's CO-OP row and the starting page's co-op dress** (`ui/coop.ts`, `StartPageOpts.coop`, `LoadoutOpts.deployWords`): `docs/agents/net.md`,
  "The lobby". In `ui/menu.ts` the title keeps which of its pages is up (`titleCoop` beside `titleExtras`), its key walk stops at a text field with
  the arrows, and a letter typed into a field is the field's (`keymap.ts typedInto`).
- **The software-drawn kit figure and the `?kit` preview are gone** (2026-11-07, the maintainer's list). The start page's column is the team at its door or empty; `ui/kitBoot.ts` reads the
  kit store and puts up the `__kit` hook. What the figure's check held of the game stays in `tools/qa/cast-deal-check.ts` (a teammate wears his place's variant, not his id's; `ROSTER_BREAK=1` must fail).

- **THE DESK** (the cards before a game hold no rule on the page). Where the rules are: `rust/game_rules/src/kit/card.rs` (a man's card, `CardView`), `team.rs` (the team, `TeamView`) and `start_list` (the mission list: `StartView`, a mission's `SheetView` with its rows of `Setting`, the short list's `ShortView`, the load brief's `BriefView`, every link),
  asked of one blank game by `rust/game_run/src/desk.rs`, which is done the same commands a start is done (`src/host/machine.ts machineBefore`), so a man is staged with what his card shows.
  HOW A CARD ASKS: `src/view/desk.ts` (`desk()`, set at the page's boot beside `kitBoot`, on the module the page has booted) and its generated caller `src/draw/asks.ts`. `ui/kitCard.ts memberCard` asks
  `desk().card(i)` at each draw that can have changed the card and keeps that view for a move of the highlight (a row's `takes` and `would` are already on it; a move asks nothing); `ui/loadout.ts` says
  `desk().for(mission, '', issue, walls)` as it opens and when the level's file has come (an empty link: the desk works out the link and asks again when a setting moves it), and asks `desk().team()` at each paint; `TeamView.link` is what Deploy loads, and `desk().same(link, page)` says whether that is what the plain page already stands (the solo night's start with no load).
  THE LIST: `ui/missions.ts openMissions` draws `desk().start(address).rows` and, for the highlighted row, `desk().sheet(id)`: every setting row (difficulty, the levers and the cams row, hour, weather, way) by ONE loop over `settings`, a press and the keys A D, H, J, K by `desk().set(id, key, to)`. `ui/startpage.ts` is its layout and what is its own: the rail reads those rows' DOM, `sheet.base` and `sheet.asPick`, the Hour row put at a short-list row's own night and the player's put back, pressed through `set` (on a mission that always draws the row: the hour is the list's). The load brief is `desk().brief(address)`; its Controls button is what `LOADBRIEF.options` hands it (main.ts: ui/options.ts, which holds game rows); no options, no button. The cards' Next is `desk().next(address)`, a won lesson's next the next `lesson` row of `start`. The page's: the plan sheet (`issuedWalls`, `issuedKit`, `placeNeeds`, `kitViewWays` read the level's file), pictures, DOM, keys, focus, the row last started (`twopointsix.mission`) and the size last deployed (`twopointsix.loadout`). `tools/draw/ban.ts` walks `ui/startpage.ts` and `ui/loadbrief.ts` too: nothing under them imports src/game.
  THE OLD PAGE IN STEP: `main.ts`'s `keeps` writes the text a press says is kept, then what its own start reads re-reads it: `kitLoad`, `difficultyFromStore` (difficulty.ts), `quietFogForget`. A press (`step`, `choose`, `reset`, `copy`, `paste`, `size`, `named`) answers `Pressed`: a refusal's
  words, and what is kept, which the desk's owner stores (the old page then runs `kitLoad`, which is all its own start reads). The page's are only its layout, the labels of buttons, tooltips and how a value is put together with `IDLE_WORD` and `LENT_WORD`.
  ADD A ROW: `kit/p_02.rs` rows, its note in `card.rs row_note`, a drawing in `ui/kitIcons.ts`. ADD AN ASK: one line of `asks!` in `desk.rs`, a struct by `tools/wire` if it answers a new one, then `bun run tools/draw/gen.ts`.
  THE WALK: `CHROME=… bun run tools/qa/cards-page.ts OUT [--face PATH]`, then `--diff A B`: every mission, size, man, step, pick, copy and paste written down in about 245,000 lines, and two trees compared; it replaced the fake-page probes `kit-card`, `pouch-card`, `loadout-screen`, `kitload-card`, `wand-tile`, `kitdeal-screen`, `kitnoise-check`, the bars' verdicts of `kittree-guns`, and of the list `missions-list`, `missions-screen`, `levers-screens`, `picks-card`, `hourwx-sheet`, `offlist`, `unplayed` and the sheet's verdicts of `hours-sheet`, `rowset`, `quietfog`, `told`, `sticky-default`, `picker-card`, `picker-level`, `map` (the module's own tests: `rust/game_run` desk tests; the eight-a-group cap too). The picture beside the card (`ui/kitView.ts`) is stubbed there: never seen against the desk on a GPU.
