//! THE MISSIONS: every row of the list, the tutorial's, the squad's and the quiet ones. The one table: the page's lists and cards, the music page, the level tools and the checks ask the game for
//! these rows (`desk.rs` Missions). A mission on a level of its own is one row here: `level` names its file under levels/, whose own `scenarios` block carries the marks.
//!
//! A row may name its group; without one a quiet mission is 'quiet, alone' or 'quiet, as two' by its team, a rescue is 'on the clock' and any other squad's mission a raid (start_list `group_of`).
//!
//! The list draws seventeen (the maintainer's "Keep my 13", 2026-11-05, and the four he brought back 2026-11-08). Every other row is `off_list`: it keeps its place in its table, its level file and
//! its address, no list draws it, and `?offlist=0` draws it again.
use crate::hours::HourId;
use crate::scene_scenarios::{MissionGroup, ScenarioEntry, ScenarioEntryKind, ScenarioEntryWeather};
use crate::ui_missions::Lean;
use alloc::{string::String, vec, vec::Vec};

fn said(words: &str) -> Option<String> { Some(String::from(words)) }
fn list(lines: &[&str]) -> Option<Vec<String>> { Some(lines.iter().map(|l| String::from(*l)).collect()) }
/// a row with nothing said of it yet: on the office, drawn, balanced
fn row(id: &str, title: &str, kind: ScenarioEntryKind, squad: f64) -> ScenarioEntry { ScenarioEntry { id: String::from(id), title: String::from(title), kind, squad, ..ScenarioEntry::default() } }

/// Every mission: the tutorial's, the squad's, the quiet ones.
pub fn all_missions() -> Vec<ScenarioEntry> {
    let mut all = tutorial_missions();
    all.extend(scenarios());
    all.extend(quiet_missions());
    all
}

/// THE TUTORIAL: missions played on rails, one new thing a step, everything not yet met switched off, each step's line the one instruction on screen. The list and a won lesson's next are the
/// drawn rows in this order.
pub fn tutorial_missions() -> Vec<ScenarioEntry> {
    vec![
        // one man and his rifle: one snowed-in mountain road, where every wreck and wall on the way up is the cover for the next key. Lit for the night: three
        // fires, two trucks' lamps and a thin moon on snow
        ScenarioEntry {
            level: said("tut-road.json"), group: Some(MissionGroup::Tutorial), hour: Some(HourId::Night),
            place: said("A snowed-in road below the pass, at night"), lean: Some(Lean::Loud), defenders: Some(6.0),
            blurb: said("You're alone on the pass road with your rifle. Get past a sentry, a locked hut and a machine gun, then go out through the upper gate."),
            scene: said("A fuel convoy was hit on the pass road, and its escort stopped answering the radio. You're going up alone to see what's left, and the people who hit it are still on the road."),
            teaches: list(&["Move, sprint and crouch", "Take cover on a wall and lean out", "Aim, shoot and reload", "Switch to full auto and shoot a door down", "Shoot through a thin wall"]),
            ..row("tut-road", "The pass road, alone", ScenarioEntryKind::Quiet, 0.0)
        },
        // the squad, with him on the ground: one order and one word
        ScenarioEntry {
            level: said("tut-team.json"), group: Some(MissionGroup::Tutorial), hour: Some(HourId::Night),
            place: said("A road tunnel's station, under snow"), lean: Some(Lean::Loud), defenders: Some(5.0), squads: Some(true),
            blurb: said("Your squad is pinned down on the road with you. Get them through the gate, then lead them door by door."),
            scene: said("The people who hit the convoy fell back to a road tunnel. You and your squad went after them and ran straight into their roadblock at the station by the tunnel's mouth."),
            teaches: list(&["Zoom out to see your squad", "Move the squad with one key", "Stack the squad up on a door, then send them in", "Call the squad back to you"]),
            ..row("tut-team", "The tunnel", ScenarioEntryKind::Roomclear, 4.0)
        },
        // his kit, then the squad's: the fuel halt at night. He goes in alone and uses each thing he carries once, then calls his four up and has them use theirs. Built for
        // the dark: its lamps are the photocell's, one of them is his to shoot out and the rest are the breaker box's. The count is of the men on the floor as it opens: its script stands five more up
        ScenarioEntry {
            level: said("tut-kit.json"), group: Some(MissionGroup::Tutorial), hour: Some(HourId::Night), weather: Some(ScenarioEntryWeather::Clear), seed: Some(1.0),
            place: said("A walled fuel stop on a dust road"), lean: Some(Lean::Loud), defenders: Some(6.0), squads: Some(true),
            blurb: said("Go in alone and use everything you carry. Then call your squad up and have them use theirs on your order."),
            scene: said("The enemies who hit the convoy fuel their trucks at a stop down the valley, and there's a machine gun behind its gate. You'll go in ahead of your squad to take out the gun, then call them up to clear the rest."),
            teaches: list(&["Throw smoke, a frag and a stun grenade", "Blow a locked door with your own charge", "Use your rifle's launcher and your pistol", "Give your squad orders from their ring", "Cut the lights and fight in the dark"]),
            ..row("tut-kit", "The fuel stop", ScenarioEntryKind::Roomclear, 4.0)
        },
        // the squad in two pairs: every stop is a thing one order to all four loses and two orders win. The last light: a low sun and long shadows across the yard
        ScenarioEntry {
            level: said("tut-queue.json"), group: Some(MissionGroup::Tutorial), hour: Some(HourId::Dusk), weather: Some(ScenarioEntryWeather::Clear), seed: Some(1.0),
            place: said("A walled relay post in the hills"), lean: Some(Lean::Loud), defenders: Some(8.0), squads: Some(true),
            blurb: said("Split your squad to do two things at once. Hit two doors together, get behind a gunner, and cover two gates."),
            scene: said("The enemies who hit the convoy use this relay post to talk to each other. Every room in it has two doors, and one group going through one door won't be enough."),
            teaches: list(&["Give part of your squad a second order", "Send both pairs in at once", "Get behind a gunner who's watching a door", "Watch two gates at once", "Open the wheel for part of your squad"]),
            ..row("tut-queue", "The relay post, in pairs", ScenarioEntryKind::Roomclear, 4.0)
        },
        // (its light is the lesson: a midday left on the link would put the lamps out and the lane in sun)
        ScenarioEntry {
            level: said("tut-quiet.json"), group: Some(MissionGroup::Tutorial), hour: Some(HourId::Night), off_list: Some(true),
            place: said("A money changer's shop on a snowy night"), lean: Some(Lean::Quiet), defenders: Some(3.0),
            blurb: said("You go in alone. Slip through the dark to the strongroom, then leave the way you came in."),
            scene: said("The people who hit the convoy were paid through a money changer's shop in town, and his ledger shows who paid them. The ledger is in the strongroom. Nobody can know you were there."),
            teaches: list(&["Stay low and out of the light", "Whistle to lure a guard, then grab him", "Knock a guard out with a sponge round", "Locks, the breaker box and night vision", "Leave the way you came in"]),
            ..row("tut-quiet", "The ledger", ScenarioEntryKind::Quiet, 0.0)
        },
        // quiet, as two: each stretch of a truck park is one thing a second man makes possible
        ScenarioEntry {
            level: said("tut-pair.json"), group: Some(MissionGroup::Tutorial), off_list: Some(true),
            place: said("A walled truck park, at night"), lean: Some(Lean::Quiet), defenders: Some(4.0),
            blurb: said("There are two of you, and nobody can know you were here. Take a guard each at the same moment, cut the lights, and leave together through the far gate."),
            scene: said("The ledger shows payments to a truck park at the edge of town, and the convoy's fuel went through it. Two of you are going in to get the manifest from the office, and nobody can know."),
            teaches: list(&["A partner who follows you quietly", "Send him around behind a guard", "Take down two guards at once", "Have him cut the power while your hands are full", "Move through the dark together"]),
            ..row("tut-pair", "The truck park, as two", ScenarioEntryKind::Quiet, 1.0)
        },
    ]
}

/// THE SQUAD'S MISSIONS.
pub fn scenarios() -> Vec<ScenarioEntry> {
    vec![
        // the office. The night breach was built and tuned on this floor, and its idea is picking your entry. It is cut from the list (the maintainer, 10-24: "let's also remove "night breach"
        // since it's unnecessary now"): the row, its address (`?nightbreach`) and every check that stages it stay, for the room clear and the rescue are built on its staging
        ScenarioEntry {
            group: Some(MissionGroup::Raids), off_list: Some(true),
            place: said("The office · the break room"), lean: Some(Lean::Either), defenders: Some(3.0),
            blurb: said("Enemies hold a break room with four doors. Stack your pair up on one door and send them in while you come through another."),
            ..row("night-breach", "Night breach", ScenarioEntryKind::Nightbreach, 2.0)
        },
        ScenarioEntry {
            place: said("The office · the east wing, room by room"), lean: Some(Lean::Loud), defenders: Some(5.0),
            blurb: said("Clear four held rooms in the office one at a time with the order menu and the door tools. This is the careful way to play."),
            ..row("room-clear-pair", "Room clear", ScenarioEntryKind::Roomclear, 2.0)
        },
        ScenarioEntry {
            untuned: Some(true), off_list: Some(true),
            place: said("The office · the east wing, two pairs"), lean: Some(Lean::Loud), defenders: Some(5.0),
            blurb: said("Clear the same four rooms with a red pair and a blue pair. Hit two doors at once, or have one pair cover the other."),
            ..row("room-clear-four", "Room clear, four-man squad", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            place: said("The office · the manager's office"), lean: Some(Lean::Loud), defenders: Some(3.0),
            blurb: said("Two hostages are held by a captor on a clock. Move down the corridor, clear only what you have to, and get in before time runs out."),
            ..row("hostage-rescue", "Hostage rescue", ScenarioEntryKind::Rescue, 2.0)
        },
        // a level a scenario
        ScenarioEntry {
            level: said("records.json"), group: Some(MissionGroup::Raids), untuned: Some(true),
            place: said("Three small rooms and a strong room, a dark lot outside"), lean: Some(Lean::Loud), defenders: Some(5.0),
            ways: list(&["the dark lot, on the front door", "the dark side yard, on the records room"]),
            blurb: said("The choice here is which door. Most rooms have two or three, but the strong room has one locked door off a room you have to take first."),
            ..row("records-pair", "The records office", ScenarioEntryKind::Roomclear, 2.0)
        },
        ScenarioEntry {
            level: said("depot.json"), untuned: Some(true),
            place: said("A motor pool at night: three huts round a floodlit yard"), lean: Some(Lean::Loud), defenders: Some(5.0),
            ways: list(&["the main gate", "the back gate"]),
            blurb: said("The yard is floodlit, and enemies cover it from two dark openings in the comms room and the store. The breaker is behind the comms room."),
            ..row("depot-four", "The depot", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            level: said("block.json"), group: Some(MissionGroup::Strongholds), untuned: Some(true),
            place: said("Four rooms off one hall, a turn in each"), lean: Some(Lean::Loud), defenders: Some(8.0), squads: Some(true),
            ways: list(&["the loading hall, on the main hall", "the loading hall, on the tool crib"]),
            blurb: said("Every held room turns a corner, so the enemies wait in the dark past a lit doorway. Each has a second door from the small room beside it."),
            ..row("block-four", "The block", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            level: said("warehouse.json"), group: Some(MissionGroup::Strongholds), untuned: Some(true),
            place: said("A racked hall, a loading dock, a ring of small rooms"), lean: Some(Lean::Loud), defenders: Some(11.0), squads: Some(true),
            ways: list(&["the staff parking, by the staff door", "the dark corner by the loading dock"]),
            blurb: said("The building is laid out as a ring, so whichever door you fight at, someone can walk around behind you."),
            ..row("warehouse-four", "The warehouse", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            level: said("compound.json"), group: Some(MissionGroup::Strongholds), untuned: Some(true),
            place: said("A walled yard, a house of eight rooms, three outbuildings"), lean: Some(Lean::Loud), defenders: Some(11.0), squads: Some(true),
            ways: list(&["the side yard gate", "the back gate"]),
            blurb: said("The enemies are behind a wall. Two of its three gates are dark, two windows are open, and the breaker is in a pantry nobody's guarding."),
            ..row("compound-four", "The compound", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            level: said("barracks.json"), group: Some(MissionGroup::Strongholds), untuned: Some(true), off_list: Some(true),
            place: said("Two wings and a mess hall round an open yard"), lean: Some(Lean::Loud), defenders: Some(11.0), squads: Some(true),
            ways: list(&["the south-east corner", "the galley back door"]),
            blurb: said("Enemies hold two galleries that look across an open yard at each other. Six rooms are empty, and the breaker is in the duty office."),
            ..row("barracks-four", "The barracks", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            level: said("registry.json"), untuned: Some(true), off_list: Some(true),
            place: said("A lit gallery to the hostage room, a dark way round"), lean: Some(Lean::Loud), defenders: Some(3.0),
            blurb: said("You're on a clock. A lit gallery passes two rooms on the way to the hostages, and a gunman waits in one of them. Check the glass."),
            ..row("rescue-registry", "The registry", ScenarioEntryKind::Rescue, 2.0)
        },
        ScenarioEntry {
            level: said("consulate.json"), group: Some(MissionGroup::Clock), untuned: Some(true),
            place: said("A corridor past four rooms to an office in two halves"), lean: Some(Lean::Loud), defenders: Some(6.0),
            blurb: said("Three hostages are behind two doors, and you'll want to hit both at once. There's no time to clear the four rooms you pass, so cover them."),
            ..row("rescue-consulate", "The consulate", ScenarioEntryKind::Rescue, 4.0)
        },
        // small missions, each built on one idea, on a level made for it: a room clear keyed on a block of the level file
        ScenarioEntry {
            level: said("floodlit.json"), group: Some(MissionGroup::Raids), untuned: Some(true), off_list: Some(true),
            place: said("A walled fuel post, eleven metres of lit yard to its door"), lean: Some(Lean::Either), defenders: Some(7.0),
            blurb: said("The yard is floodlit, with guns in two open windows. The breaker is in a shed outside the wall, and the enemies don't have night vision."),
            ..row("floodlit-four", "The floodlit yard", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            level: said("killhouse.json"), group: Some(MissionGroup::Clock), untuned: Some(true),
            place: said("A five-room shoot house on a range"), lean: Some(Lean::Loud), defenders: Some(6.0),
            ways: list(&["the ready line, on the front door", "the dark side, on the back door"]),
            blurb: said("The clock starts on your first move. Gold is 1:05, silver 1:30 and bronze 2:00. Every teammate you lose adds 25 seconds."),
            ..row("killhouse-four", "The kill house", ScenarioEntryKind::Roomclear, 4.0)
        },
        // the walls are the way in: the level's own room clear issues the squad wall charges for the run (scenarios.roomClear.wallCharges), and every room's door opens on one lit corridor a
        // duty-room gun looks down
        ScenarioEntry {
            level: said("mousehole.json"), group: Some(MissionGroup::Strongholds), untuned: Some(true),
            place: said("A clinic: three rooms and a duty room off one lit corridor"), lean: Some(Lean::Loud), defenders: Some(9.0), squads: Some(true),
            ways: list(&["the dark yard, on the blank wall", "the dark side, on the fire exit"]),
            blurb: said("Every door opens onto one lit corridor, and a gunner behind a hatch covers it. A flashbang can't reach him, but you have wall charges."),
            ..row("mousehole-four", "Mousehole", ScenarioEntryKind::Roomclear, 4.0)
        },
        ScenarioEntry {
            level: said("hall.json"), group: Some(MissionGroup::Raids), untuned: Some(true), off_list: Some(true),
            place: said("One wing: a bare hall 23 metres long, four rooms off it"), lean: Some(Lean::Loud), defenders: Some(7.0),
            only: said("Your kit is issued for this one, so it's played with the full squad of four."),
            blurb: said("Two enemies at the dim far end cover the whole hall, and the only door in opens onto it. Your kit is issued for this one."),
            ..row("hall-four", "The long hall", ScenarioEntryKind::Roomclear, 4.0)
        },
        // glass is not cover: a floodlit forecourt, a glazed front and glass partitions with dark rooms behind, a blank back wall with the back door into the store; both starts are dark and out of
        // sight. The blurb holds for both ways in. Two squads hold it (Front stays in the glasshouse, Back moves), and the level issues one wall charge for the blank end wall
        ScenarioEntry {
            level: said("glasshouse.json"), group: Some(MissionGroup::Raids), untuned: Some(true), off_list: Some(true),
            place: said("A seed station: a glazed front, glass between its rooms"), lean: Some(Lean::Loud), defenders: Some(7.0), squads: Some(true),
            ways: list(&["the dark beside the building, round the corner from the glass", "the dark back lot, on the back door"]),
            blurb: said("The enemies in three dark, glass-walled rooms will shoot right through the glass. Only the back store has none. You get one wall charge."),
            ..row("glasshouse-four", "The glasshouse", ScenarioEntryKind::Roomclear, 4.0)
        },
        // made for snow (tools/levels/pass.ts): what falls is the weather's, keyed on the level's name, so the row names none. The one gate is lit and watched through windows from two sides of the
        // yard; the other way in is the kitchen's back door, outside the wall
        ScenarioEntry {
            level: said("pass.json"), group: Some(MissionGroup::Strongholds), untuned: Some(true),
            place: said("A teahouse and truck halt on a pass road, under snow"), lean: Some(Lean::Loud), defenders: Some(8.0), squads: Some(true),
            ways: list(&["the road, in the dark beside the gate", "the back of the halt, on the kitchen's door"]),
            blurb: said("The only gate is lit, with a truck snowed in just inside. Lit windows watch the yard, the kitchen has a back door, and there's a generator."),
            ..row("pass-four", "The pass", ScenarioEntryKind::Roomclear, 4.0)
        },
    ]
}

/// THE QUIET MISSIONS, in a table of their own: a level file with a `scenarios.quiet` block, squad 0 alone, squad 1 with one partner under the pair rules. Each start deals one of the block's
/// starts, spots and patrol sets off the run's seed. Whoever walks `scenarios` means a team to command, so they are not among those.
pub fn quiet_missions() -> Vec<ScenarioEntry> {
    vec![
        ScenarioEntry {
            level: said("bazaar.json"), untuned: Some(true),
            place: said("A shuttered bazaar under curfew: two blocks, one passage"), lean: Some(Lean::Quiet), defenders: Some(5.0),
            blurb: said("Two patrols circle the blocks and share one passage. Move behind their flashlights or cut through the shops. Take the sat phone and leave through any gate."),
            ..row("bazaar-quiet", "The bazaar row", ScenarioEntryKind::Quiet, 0.0)
        },
        ScenarioEntry {
            level: said("police-post.json"), untuned: Some(true), off_list: Some(true),
            place: said("A walled district police post"), lean: Some(Lean::Quiet), defenders: Some(5.0),
            blurb: said("The desk sergeant carries the keys, and the guards know the armory code. Grab one and ask, or pick a lock. Take the case file and leave through a side gate."),
            ..row("police-post-quiet", "The police post", ScenarioEntryKind::Quiet, 0.0)
        },
        // (tools/levels/vault.ts: built round the devices: a lens on each gate and one on a keypad door, and two tripwire beams)
        ScenarioEntry {
            level: said("vault.json"), group: Some(MissionGroup::QuietAlone), untuned: Some(true),
            place: said("A money changer's house in a walled yard"), lean: Some(Lean::Quiet), defenders: Some(3.0),
            blurb: said("There's a camera on the lit gate and a keypad on the strongroom. The clerk who knows the code smokes alone in the dark. Two tripwires guard the back door."),
            ..row("vault-quiet", "The counting house", ScenarioEntryKind::Quiet, 0.0)
        },
        ScenarioEntry {
            level: said("crossing.json"), untuned: Some(true), off_list: Some(true),
            place: said("A border checkpoint, the customs house across the road"), lean: Some(Lean::Quiet), defenders: Some(6.0),
            blurb: said("The manifest is across a lit road, past two sentries who watch each other. Put your partner behind one and grab the other, and he'll go when you do."),
            ..row("quiet-crossing-two", "The border crossing, as two", ScenarioEntryKind::Quiet, 1.0)
        },
        ScenarioEntry {
            level: said("quiet-relay.json"), untuned: Some(true), off_list: Some(true),
            place: said("A hilltop radio relay: mast, comms hut, ops room"), lean: Some(Lean::Quiet), defenders: Some(6.0),
            blurb: said("The crypto drive is in the comms hut. Its door is lit, locked and on camera, and a guard watches it from the dark behind the mast. Send your partner behind him."),
            ..row("quiet-relay-two", "The relay station, as two", ScenarioEntryKind::Quiet, 1.0)
        },
        // (built for two: the breaker and the door it darkens are a courtyard apart, and the dark lasts as long as one man's walk to the panel: the man at the hall's window, whom the game sends)
        ScenarioEntry {
            level: said("lights-out.json"), group: Some(MissionGroup::QuietTwo), untuned: Some(true),
            place: said("A walled compound: the generator shed, the radio house"), lean: Some(Lean::Quiet), defenders: Some(3.0),
            blurb: said("The codebook is in the radio house, behind a camera, beams and a sentry. The breaker is in a shed, and the dark only lasts until he walks over to it."),
            ..row("lights-out-two", "Lights out, as two", ScenarioEntryKind::Quiet, 1.0)
        },
    ]
}
