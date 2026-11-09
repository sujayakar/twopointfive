# The player

Sam in the player's hands: his feet, aim, guns, throws and kit, what he does to his own side, what hits do to him, and the view. The wall hold,
takedowns and the creep are `stealth.md`, a burst on a wall `destruction.md`, night vision's rules `team-feel.md`, door orders `door-tools.md`, a hand
on a leaf `doors.md`, poses `anim.md`, the kit screens `menus.md`.

## What the player gets

- **Three gears.** A walk, a run on Shift, a crouch on C. Shift in a crouch is a quick crouch walk, louder than a walk. Lighter kit is faster and
  quieter.
- **A gun with weight.** The ring follows the cursor at the rate the gun allows, the round goes where the gun points, and moving opens the cone.
- **One round is enough,** from any gun on any man. Guns differ by reach, noise, capacity and handling.
- **Digits.** 1 the sidearm, 2 the long gun, 3 smoke, 4 stun, 5 frag. A gun's digit again opens its setup: selector, load, light.
- **A launcher** on the right button of a long gun with a tube: a sponge round, a 40 mm grenade or a sticky camera.
- **A card.** Four pouches and one big slot a man. What is on it is what he deploys with, and each thing costs pace and notice.
- **His own actions on a ring.** E tapped does the default, E held puts every row up.
- **His gun knows his friends.** No ring on a teammate, and it comes down off one on a quiet floor. In a fight his rounds can hit them.
- **A blast counts.** It folds, blinds or hurts him for the mission. The end card shows how he died.

## Where the code is

In `rust/game_rules/src/`, with types and numbers under the same names in `rust/game_types/src/` and its `knobs/`:

| module | what it does |
|---|---|
| `player/` | `update_player`, one tick of his keys, feet, aim, guns, throws and footfalls; the cursor (`update_aim`); his rows (`build_interactables`) |
| `noshove.rs` | a teammate falling in goes round Sam |
| `aim.rs`, `aimweight.rs`, `aimcost.rs` | how settled a shot is, both sides; turn rate and the lagging ring; lamps need a real hit, feet cost the shot |
| `combat/` | a round's flight (`fire_weapon_raw`), who it finds, `hit_player`, held things as boxes, cans going off |
| `weapons.rs` | a gun's state: magazine, chamber, modes, reload, the launcher's loads, the OCP. The rows are `rows` in `game_types/src/weapons/` |
| `tracers.rs`, `thinline.rs` | which rounds show a streak; boxes the page draws as thin lines |
| `gunreload.rs`, `autoreload.rs` | reload beats and cues, shell by shell on a tube gun; the dry gun starting its own |
| `gunwall.rs` | a raised gun comes down at a wall, for every man |
| `gunslots.rs`, `gunroles.rs` | the picks a card offers, a teammate's sidearm when dry; what a teammate's gun is for |
| `slotsetup.rs` | the setup a gun's digit opens |
| `launcher.rs`, `he40.rs`, `shotgun.rs` | the sponge round; the 40 mm; a shell's pellets and a lock taken with them |
| `throwables/`, `throwplan.rs`, `frag/` | things in the air and `throwables_solve`; a flight asked ahead; the frag, `blast`, `throw_reach` |
| `kit/`, `kitload.rs`, `loadout/`, `issue.rs`, `gear.rs` | the card (`kit_player`); the two scores; a mission's size; a level's wall charges; the issued kit's words |
| `bigkit.rs`, `packside.rs`, `inhand/` | hammer and breaching shotgun, Sam's beat with them; what stands where on a pack; what sits where in a hand |
| `hands.rs`, `handwork.rs`, `workstow.rs`, `workthings.rs` | bodies and a leaf by hand; a hand on a thing; the gun put away for a job; a teammate's jobs |
| `handslot/`, `handring/` | the empty-hands key; the ring |
| `friends.rs`, `friendlyfire.rs`, `fireline.rs` | ring and low ready; rounds among friends; a teammate waiting out Sam's lane of fire |
| `condition.rs`, `samstun.rs`, `flinch.rs`, `deathrecap.rs` | hurt; folded and blind; a man ducking at a round; the record of a death |
| `ownblow.rs`, `cleanrestart/` | Sam's own charge as his own row; the restart's counters |
| `camera.rs` | the follow camera and the rays off it |

The page's half: `src/host/keymap.ts` (which key gives which code), `src/host/camera.ts`, `src/host/sandbox.ts`, `src/ui/handbox.ts` (kit column),
`src/ui/recap.ts` (death panel). The HUD reads `kit_view` and `rust/game_rules/src/tick/view/`.

## How it works

**A tick of Sam** is `update_player`, read from the top. Enter restarts. Down, he stops there. A beat that has him takes the whole tick and returns: a
takedown, a hold, a kick, picking, the wand, a job (`sam_work`). Only then come digits, reload, feet, aim, triggers, throws and the footfall.

**Keys are codes.** The game asks for `KeyR` or `Digit1`, and `keymap.ts` decides which key gives each. On the page E gives the interaction's code
(`Space`), Z and L give nothing (they are in the setup), backtick gives `Digit0` (`HANDSLOT_KEY`), and the Space bar is the camera's.

**Feet.** `sprint_stance` turns Shift into `sprinting` or `dash`, `free_pace` picks the pace and `sam_load_pace` weighs it by his card, `stride_step`
moves him, then walls, then `keep_off_guards`. `footfall_due` files a step, its level by gear, `load_heard` and floor (`tread`).

**The aim.** `update_aim` makes the cursor `aim_point` and snaps it to a light (`aim_target`) or an enemy (`aim_guard`). A teammate only sets
`aim_friend`. `man_aim` aims at where a low man really is. Three things then stand apart: the cursor, the gun's heading (`aim_weigh_turn`, slowed by
`handling_of`) and the settled `track` (`track_player`).

**A round.** `shot_target` gives the point and `aim_weigh_shot` turns it to the heading. The cone adds `firearm_auto_spread`, `sam_spread` (his
suppression) and `sam_aim` (`player_aim`: hands, settle, target speed, feet), and on the long gun `rifle_spread`. `fire_weapon_raw` flies it through
what it pierces and scans bodies.

**Slots.** `slot` is a `SlotKind`, `holstered` stands apart, and `player_gun_in_hand` asks both. To or from the long gun sets `equip_t`, and no round
goes until it ends. `slot_setup_keys` runs before the digits and takes them while the setup is up.

**The launcher.** `launch_held` is the load whose button is down, with a solve each (`ring_solve`, `shell_solve`, `cam_solve`) and its arc in
`throw_preview`. Letting go asks `aim_weigh_launch`, which waits up to `launch_wait` for the ring. `ring_cd` is the one tube's clock. The sponge round
puts a man out at the head, folds him in the body, and off a wall is a noise there. The 40 mm is a dud short of `arm_m`, and its burst is `blast`.

**A throw.** `do_throw` takes one off the count and `pending_throw` holds it until the arm lets go (`THROW_RELEASE`). The preview shows where it comes
to rest (`throw_rest`, `rest_of_throw`, which moves nothing) and the burst's rings (`throw_reach`).

**The card.** `MemberKit` is one card, priced by `SlotTable`. `kit_player` writes it on the Player at staging, and `kit_fresh` after a restart on a
solo card. `big_used` counts the big slot. `kit_alone` makes the charges Sam's own count when nobody came. A mission's `issue` block is `kit_issue`.
`load_worn` makes the two scores, read against his issue.

**The rows.** `build_interactables` lists what he can do, `update_interact` makes E a tap or the ring (`whistle_wedge` the last wedge), and a tap with
no row says why (`empty_thing`). So does any other press that did nothing, the trigger, a slot key on a wall and the wall key among them (`refuse`). A row performed is `g.act`.

**His own charges.** `door_tools_sam_rows` and `wall_charge_sam_rows` offer them, `door_tools_sam_tick` and `wall_charge_sam_tick` run the job,
`update_work_plant` plants him. `sam_stow_tick` puts the gun away first, and the job's clock waits on `sam_stowing`. A step, a sprint or a click ends
it (`sam_work_let_go`). `own_blow_row` is the blow from anywhere, and `own_blow_held` refuses one that would put him down. `door_tools_key_is` and
`wall_charge_key_is` say when the squad's key has a charge instead.

**Being hit.** A round is `sam_hit`, then `hit_player`: `hits_left` is one, so he goes down, his gun falls and he ragdolls. A blast is `seat_blast`:
killed is the same, wounded calls `condition_hurt`, wounded or stunned sets `stagger_until`. A can is `dazzle_player` and `stagger_player`, and
`fold_cancels` empties his hands. Folded he is rooted (`sam_rooted`). Blind he keeps his feet and takes no ring.

**The view.** The page steps `FollowCamera` in `src/host/camera.ts` and hands it in with each hand (`game_face/src/hand.rs`); native hosts step
`camera.rs` (`follow_camera_update`). Rules only ask it: `follow_camera_planar_basis`, `follow_camera_ray`. It leads the aim by `aim_lead`;
`peek_offset` is the peek at a wall's end.

**Other keys.** N is `nv_key`, only if `kit_you_nvg`. The weapon light is `light_on` on each gun, row 3 of the setup, one lamp posed by
`pose_gun_light`, only if `kit_you_light`.

## Rules that hold

- **One round.** No gun's row has a damage of its own. Nothing is harder by hit points.
- **Two gears, no dial.** "let's start by just removing the pace wheel entirely with the faster crouch sneak": `wheel_pace` does nothing.
- **The sprint is the standing man's.** "we can make sprint when crouching a really fast crouch walk, but you have to stand up to sprint properly". A
  hurt man does neither.
- **The aim has weight.** "we should have limited turning rate + the aiming circle can lag behind the cursor (like helldivers 2)". A round
  goes along the heading.
- **Lamps and feet.** "yeah, needs a real hit. we should also really penalize accuracy while moving" (`lamp_struck`, `feet_hold`).
- **A dry gun reloads itself** (`auto_reload_due`) a beat after the last round. It asks the chamber, so a last magazine seated and not racked still
  reloads.
- **A gun comes down at a wall** and no round goes (`gun_walled`), but the breaching shotgun laid on a lock stays up and fires (`breach_aim`). An AI
  man in a fight (`fight_s`) lowers only a gun standing in the panel; Sam's test is always the calm one.
- **No ring on a friend** (`friend_no_snap`), and an enemy behind a friend under the cursor is not ringed through him.
- **Off a friend out of a fight** (`sam_off_friend`, `friends_in_fight`) the gun comes down, for a hostage in a rescue too. In a fight it stays up.
  Any round of his can hit a friend, as an enemy's can.
- **A teammate does not press with a friend where his round could go** (`friend_in_cone`, `ff_unpressed`), and waits out Sam's lane of fire or finds a
  way round (`fire_line_feet`).
- **Flash yourself.** "it should totally be possible to flash yourself with a bad entry": `own_seen`. A teammate is spared his own side's can.
- **Kit has a price.** "we should also model a mobility / concealment score like door kickers 2 does -- there should be legitimate reasons to bring
  less kit". A gun's `weight_kg` and `bulk` count.
- **One big slot a man.** A wall charge, the hammer or the breaching shotgun fills it and costs no pouch (`price_of`).
- **The setup is a mode.** "the timer is bad. we should just have it switch a mode". A round leaving shuts it.
- **His own actions are his key.** "radial menu for our own actions, remove existing menu in top left and awkward scrolling thing". A tap never
  whistles.
- **The squad's key is the squad's.** "i think we shouldn't use F for blow it, F should be reserved for squad. instead it should be one of our own
  actions".
- **Hands clear for a charge.** "characters should fully unequip their weapon when planting door charges, etc. -- they can reequip after they're
  done". `sam_up` passes before a round.
- **A can not yet thrown goes back** when a fold or a job takes his hands (`throw_off`). Unlimited ammo covers cans, launcher rounds and wall charges.
- **A restart is clean.** "after resetting a level … the lights kind of flicker on in engine. Like, we should reset the level to a clean state".
  `game_restart_encounter` makes a fresh Player and floor. A load is not a restart. Enter when down goes to the run's newest checkpoint, else
  restarts; Shift and Enter always restarts.
- **A recap tells nobody still playing** (`recap_over`).
- **The sandbox row is no clean run.** God, no AI or unlimited ammo sets `verdict.cheat`. They are commands (`God`, `Ai`, `Ammo`), kept per machine by
  `src/host/sandbox.ts`.
- **The camera is on the Space bar.** "move camera pan from ctrl to space". The board's rule (`camKey`).

## To change it

- **A number:** its table in `knobs/`, by the panel or `?knob.aimweight.heft=2`. Paces are `player_walk`, `player_sprint`, `player_crouch` in
  `GameTune`, and `knobs/player.rs`.
- **A gun:** a row in `rows`, a `WeaponId`, a pick in `picks` (`game_types/src/gunslots/`), its look in `gunlook.rs`. `Fn2000` and `Fiveseven` are the
  ids of the Mk 18 and the Glock 19, and a line written with the old names goes through `gear_text`.
- **A thing on the card:** `MemberKit`, `SlotTable` in `knobs/kit.rs`, its row in `kit/`, its weight in `knobs/kitload.rs`, its write in `kit_player`.
- **A row for Sam:** `build_interactables`, its `Verb`, the rule that reads `g.act`. A job that plants him is an arm of `sam_work`. A key: its code in
  `update_player`, its row in `KEY_ACTS`.
- **A new reason his gun cannot fire:** a term of `walled` in `update_player` with its line on a click, and `gun_shut` so that the ring shows the bar.
- **New state:** on the Player it dies with the fresh Player in `game_restart_encounter`. Elsewhere it needs a reset there, and `restarts.rs` shows if
  it was forgotten.

## What holds it

Native tests in `rust/game_start/tests/`. `friends.rs`, `samstun.rs`, `aimweight.rs` and `condition.rs` carry unit tests too.

| file | what it holds |
|---|---|
| `held_gears.rs`, `held_aim.rs` | paces, the dash, one footfall a step; a shot's settle, a lamp's hit, feet and the cone |
| `held_gunhands.rs` | a click at a wall fires nothing, back from it a click fires |
| `kit.rs`, `kit_view.rs` | the card each run wears, the setup's load row, carriages; the kit column |
| `held_sidearm.rs`, `hands.rs` | a teammate's sidearm when dry; a magazine change goes on, nobody fires with it out |
| `guns_and_debris.rs`, `rounds_and_saves.rs` | a dry gun reloads, unlimited ammo covers wall charges; what a round goes through |
| `held_forty.rs`, `held_ring.rs`, `held_frag.rs`, `held_flash.rs` | the 40 mm, the sponge round, the frag, the stun can |
| `held_bigslot.rs`, `held_charges.rs` | his hammer; his charges, their count, his own blow |
| `held_friends.rs`, `men_by_sam.rs` | a teammate not pressing; a man falling in going round Sam |
| `wall_kills.rs`, `wallcharge.rs` | a wall charge on Sam: down, blind or nothing by distance |
| `recap.rs`, `restarts.rs`, `rings.rs` | every death explained; a restart equals a fresh staging; his ring told to the HUD |

`keybinds-page.cjs` in `bun run tools/qa/pages.ts menus` holds the page's keys.

## Not seen

How aim weight, the low ready, the dash, the stow, the fold and the blind view feel and look has been checked by numbers only, never on a GPU.
`hit_player` still words a second hit that no Player reaches, since `hits_left` starts at one.
