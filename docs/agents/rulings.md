# Rulings

What the maintainer has ruled about how the game, its look, its code and its text should be, in the maintainer's own words, where the code follows it today.
A ruling that stands on a system's own page is there and not here.

## The GPU and the picture

- "we should remove any mitigations we have unless they're explicitly confirmed to work": a lost device gets no gentler second try in `src/gpu/`.
- "let's just assume we only support nvidia and apple for now": nothing under `src/gpu/` or `src/render/` branches on a vendor.
- "i'd generally prefer to have a single path when possible": one set of ray programs, with `CUT_BOXES` always in the prelude (`frame.ts`).
- "let's just make sure it's logged": a GPU too small for the gather's buffer is reported by `splitRefused` and the row stays on.
- "weird that the men are covered in snow, shouldn't happen": `lookMan` puts no snow on a man, and "let's also remove the snow on boots" took the cake off too.
- "we also need to get away from the pixel art textures": a man's camouflage is the print on the bolt (`manCamo2`), never noise cut at a level.
- "we don't need to go full hyperrealistic but we should sell it more": a car's body is the `carpaint` stuff in `lookTex.ts`, and no more than that.
- "i don't think we need the NVG to glow in the dark": `goggles_boxes` builds the tubes from `goggle_part`, with nothing lit.
- "shooting metal shouldn't have particle debris, maybe just sparks?": `hit_look` gives `Metal` no chips.
- "for most glass we do want to simulate the fractures -- I think props like cars should be an exception": solid glass cracks and throws `SOLID_GLASS_CHIPS`.

## Words on the screen

- "let's rename "sam" to just "player"": a man's name is `player_name`, else `Player`. Sam is the code's word.
- "yeah switch to 15 rounds for the glock, 40mm sponge round sgtm (and redo the assets), and just call it the mk18": `mag_capacity` and the pick `mk18` (`picks`).
- "weird that we show a score (and a low one) for loud missions": `loud_row` keeps the sneak's rating off a loud row's end card.
- "we shouldn't show the arrows, feels unrealistic": a lost teammate's row is struck, and nothing draws the way the round came (`lost_live`).

## Sam's kit

- "we should just remove the max one wall charge per squad limitation": a man's slots are the only limit (`slot_max`), and no count runs across the squad.
- "we should also probably just get rid of the ballistic shield, doesn't match the vibes (feels more like swat)": `levelData.ts` refuses the key on a card.
- "attachments like silencer should be per gun": `suppressed` for the long gun and `side_can` for the sidearm, on `MemberKit`.
- "ideally rifle attachments should be hierarchical under the rifle": the card's rows are a tree (`tree_up`, `kittree`).
- "sponge should show up, should only be part of stealth builds by default": `issued` cards carry no `rings`.
- "not sure it makes sense to have "mission's" for equipment. that should just be defaults. and if we want to have required equipment, that should just be a constraint
  on the stuff you pick.": a quiet mission starts from your own stored card if it fits (`stealth_card`).
- "as issued still has blue team pretty empty": the kit is dealt over who comes, and "A tool each pair" lends the pair's second man a big tool (`lend`, `deal_tool`).
- "we should say 31/30 and then two spare magazines on the side": `handbox.ts` shows one magazine's size and a little box for each spare (`mags`).
- "we should have different animations based on how far you're throwing something": `throw_by_read` sets the wind-up by the distance (`k.throw_by`).
- "the swaps when taking out the hammer and putting it back feel too fast -- swapping the guns feels good, can we make sure they all feel similarly physical?":
  a tool changes hands with a gun swap's weight (`k.bigslot.equip_w`).
- "like the character should have a hammer on their back, they put away their gun, equip the hammer, wind up, and then hit the door in one motion": `bigmove`,
  `hammer_blow`.
- "using the hammer shouldn't require being directly in front of the door?": `big_spots` deals a tool its spots off the door's line.

## Kit screens and briefs

- "this menu is pretty unwieldy, door kickers is a lot better. we shouldn't copy it but maybe we have a popup menu per gun, more information density, etc.": `kitDense.ts`.
- "clicking outside the map should close it": `KITBRIEF` `outside`, and that click does nothing else.
- "we should have copy and paste for loadouts": `clipKey`, on the picked man's card from any row.
- "our art style is also AWESOME so if we could highlight it": a mission's card on the list wears a picture (`menuStills.ts`).
- "better but let's show the map vs. the (kinda low res) level art": the brief draws `drawMissionMap`, not the mission's picture (`loadbrief.ts`).
- "this tutorial is good but we should attach it to a tutorial mission (and make it bespoke and explain how to plan), not for the regular ones": a regular mission's
  Team & kit has no walkthrough.
- "i think we can rip out the auto camera for now btw": the camera is the hand's alone, and a tap of its key does nothing.
- "i think aim pan zero and turn direction reversed feel good": the page starts so (`FINDME_TUNED`).

## The squad

- "radial menu should stick on the point for the action, not move with the camera": `ring_pin`.
- "let's remove the icons on mouse position from squad orders entirely": the cursor carries no order icon.
- "pressing F only releases the people blocked on it": `door_tools_word_men`.
- "the guys shouldn't split up in situations like this -- buddy teams should try to stay together": a pair goes round one way (`squad_orders_same_ways`).
- "a, but yeah you have to order it": a teammate looks under a door only on the row `wandLook`, and nothing in `doortools/` or `autopilot/` starts it.
- "ordering your squad to throw a grenade is still pretty useless. they should figure out how to throw to a spot": `walk_spot` finds the man and where he walks to.
- "the point man here saw his buddy go down, knows roughly where the enemy is in the room and looks the wrong way going in": `door_tools_threat_note`.
- "yeah maybe this should force kicking from the front?": `door_tools_boot_spot` goes square on where a pane has eyes on the side spot (`stack_eyes`).

## Defenders

- "each actor has ideas of where other enemies are (which may be out of date), where is safe, what good cover points are": `picture/`, told by `picture_tell`, never
  from the truth.
- "effectively just a health system ... but with very high lethality": one model for everybody, `condition_of`.
- "both quiet and maybe even some loud": a response team on a loud level is a `raid` in the level file.

## Fog of war

- "given how pretty the game is, i think having fog of war mute the colors is unfortunate": `desat` is 0 (`FogK`).
- "fog of war edges shouldn't be so sharp": `edge`.
- "I think we should at least show everything in the environment visible at the start of the level all the time?": the place's fires and smoke draw under it (`air`).
- "nit, you should be able to see your sticky cameras in fog of war": `sticky_marks`.
- "I won't see tracks forming in fog of war, right? but if an enemy off screen makes tracks and then I see snow, I'll see the tracks?": `unseen` in
  `src/picture/snow/tracks.ts` keeps a hidden man's marks off the map until the team looks.

## Sam's hands and the wall

- "setting up the optical wand should be faster. pretty much just the time for the animation and none of the waiting time with the spinner (which we can probably just
  remove)": `quick_secs`, never before `quick_down`.
- "should be able to use night vision with wand": `nv_key` runs while `wanding`.
- "having the hands up just doesn't look cool": the creep's arms hang low and go out only by `wide_by`.
- "sam springs into action for the final grab ... you really want a disarm and then a grab, right?": the strip goes first (`leads`).
- "sprinting straight into a wall should snap to cover": `wall_ran_on`.
- "we should have some audio feedback for when you snap/unsnap from a wall. the transition animation is also too fast, should probably be 2x slower": `voice_on`,
  `voice_off`, `slow_take`.
- "weird that the line here is a straight line, not a pathed one. i'd expected pathed from squad commands": a man walking to a noise has his own way drawn
  (`attentionWay`).

## Weather

- "sprinting should leave messier trails": `runMess`, by the man's speed as the foot lands (`tracks.ts`).
- "footprints can also be a bit messier": `mess` scales how untidy any print is.
- "the low res rain isn't scale invariant": a splash is drawn for how far off the camera is (`rainScaleConfig`).
- "wet rain simulates droplets which don't make sense when wet indoors. also only models being completely wet": a coat has a line it reaches up him (`wet_line`).

## Sound

- "the jazzy breakbeats and phenomenal bass work as the foundation are really what pull me in": what `src/audio/pieces.ts` stands on.
- "we should also make the grab super satisfying. cool animation where sam disarms him and satisfying sounds": `grabCatch`, `gunStrip`, `gunDown`.

## Lessons

- "we show the modals too quickly. like it should happen a beat after something happens on screen": `lesson_settling`, `card_beat_secs`.
- "we should block quick Q here when we want to teach something else": a step's `no_tap`.
- "i think this needs more explanation for the systems. maybe even a modal that pauses the game and explains it … and then the player can try it out": a step's `explain`.
- "guys should announce that they're throwing out a smoke grenade": `SMOKE_CALLS`, each can called by its thrower.
- "guy taking a smoke would have turned his flashlight off": the smoker's script orders his light out at his chair (`script_tut_quiet`).

## Levels and dress

- "middle eastern enemies also don't make sense for the office level": `office_dress`.
- "sam should also be in a black sneaking suit in stealth missions (probs not loud ones)": `sneak_suit`.
- "would expect a blacked out rifle build for stealth loadouts": `rifle_black`, on the same test.
- "could we make it so the cars have rounded surfaces? look really blocky right now": `carShapes.ts`.

## Tools and code

- "yep breaking replays is fine": a change may part an old tape, and a tape that cannot play is refused (`TapeRefused`).
- "let's do an audit of all our errors and have them use some reusable error reporter that sends up to convex": `reportError`.
- "feel free to delete them if they're no longer relevant or fix them": what the maintainer said of old checks, so a stale row in `checks.ts` goes.
- "reusing the very old scenes from the showcases looks very hokey now. we should just remove the two showcases and make a new benchmark": `BENCH_SCENES`.
- "we also don't really use the level viewer and editor right? if so we could remove those?": no viewer or editor stands, and a level is changed in `tools/levels/`.
