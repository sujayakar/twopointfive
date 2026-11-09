//! The numbers of pressure.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PressureK {
    /// the master switch: off = none of this file runs
    #[k(false)] pub on: bool,
    /// the three behaviours, each measurable alone (all need `on`)
    #[k(true)] pub press: bool,
    #[k(true)] pub flank: bool,
    /// HARD ON A LEVEL THAT DECLARES SQUADS (enemysquads.ts): the press above stands down there — the squads own who moves — so harder defenders are the
    ///  squads themselves as a DRILLED GARRISON: enemysquads.ts SQUADS.drill* (each rule its own switch there), read only while `on` and this row are both
    ///  up and squads are staged. Off = hard on a squads level is standard run for run, as it was
    #[k(true)] pub squad_drill: bool,
    /// stand and fight: a hunter the press called who meets the team out of the room he was called to stops and fights from where he stands (needs press;
    ///  nothing under `gather`, whose men are set outside the doors already)
    #[k(false)] pub second: bool,
    /// men to one press (a far room that goes together — `byWay` — is not cut in two by it: its men are over the count)
    #[k(4.0)] pub max_men: f64,
    /// s a fight he heard stays worth going to
    #[k(15.0)] pub memory_secs: f64,
    /// × his beat between hearing the fight and calling it and going (1 = the stock rally's own beat, FIGHT_SKILL rally: 1.5–4 s). The dial for how long a team
    ///  that keeps moving has before the press is on it
    #[k(1.0)] pub organise_k: f64,
    /// STAND TO — surprise lost before the first entry. The first loud thing they hear is not a room of theirs being hit (the team's gunfire on the way in): the
    ///  man best placed calls it on the net ('shots outside — they're coming. stand to!'). While the report stands nobody walks off to it by himself, and the
    ///  men of every room but the one the noise is nearest keep their hands off the furniture and stand ready. When a room IS hit ('contact! …' on the net)
    ///  it is ALL HANDS, for the rest of the fight: no man is left at home to keep a room and a press takes every man who hears it (maxMen is not asked).
    ///  Nobody comes, and nothing more is heard, for `standSecs`: they stand down, on the net, and the stock rules are back (a later report starts it again).
    ///  Off = a report is a fight at the nearest mate's post, as it was. A fight that starts as a fight (the first loud thing is a boot, a charge, a round in a
    ///  room) never comes through here: a team that kept surprise meets the defenders it met with this off. Off in the build, ON under the
    ///  difficulty switch's hard (difficulty.ts HARD.warned)
    #[k(false)] pub warned: bool,
    /// stood to, ALL HANDS GO THROUGH THE PRESS (needs warned): the stock rally's clock does not run while the report stands, so at the hit every man takes his
    ///  beat and joins the press (round by another door, stand and fight). Off = as the rule was first measured: the clocks armed at the report have run out by
    ///  the hit, and three men in four leave that frame by the stock rally, each by himself, the rest through the press. OFF: measured easier for the team
    ///  (The block with surprise lost, seeds 145-288: +18 runs won of 144, +3 to +33; +17 teammates of 576, −17 to +51) — men who all arrive together
    ///  at the bang meet the team at its strongest
    #[k(false)] pub stand_press: bool,
    /// s between the first man hearing it and his call
    #[k(1.4)] pub report_secs: f64,
    /// s stood to with nobody coming and nothing more heard before they stand down
    #[k(75.0)] pub stand_secs: f64,
    /// s after the room is hit that a man who stood ready is still the press's, not the furniture's (his beat toward the fight starts at the hit)
    #[k(5.0)] pub stand_grace: f64,
    /// s before a man who has been in a press joins another
    #[k(14.0)] pub again_secs: f64,
    /// A CHARGE EACH (0 = none: f6cf9316's press run for run; on under hard on The block only — difficulty.ts HARD_AT; record
    ///  rebuild/hard-everywhere/round2-block.md): every defender carries this many stun charges, the team's own kind, and a man of a press let
    ///  go at the fight takes the door he was put down for as a DRILL instead of walking in (tactics.ts's kill-zone doorway, built for the night staff's response
    ///  team and never yet handed a charge): on the jamb out of the opening, the call on their net ('they're in there — … stack on me, charge first'), his second
    ///  fetched or the wait run out, the leaf shoved, 'bang out', the can in round the jamb and heard on the floor before it goes, 'going in'. All of that is what
    ///  the player is given to answer it with: a voice at the door, an arm round the jamb, a can bouncing. Handed to each man of a fight once, the first frame the press sees him (a charge spent is not replaced); what it does to a man
    ///  is combat.ts FLASH, the same numbers both ways
    #[k(0.0)] pub stuns: f64,
    /// A CHARGE GOES ONLY WHERE THEY ARE KNOWN TO BE (on; off = a man let go with a charge marked whatever door the press put him down for — the 10-13 tapes:
    ///  eight charges burst in rooms no teammate was in, each after a man of that room was shot through a window from outside, two of them after their own net
    ///  had said the team was in the yard, one dazzling their own man). 'A fight at a mate's post' is what sends a press; it does not say the team is IN that
    ///  room. A door is taken charge first only on something that does: a living man of theirs who has NOTICED them has had one of the team in his sight in
    ///  that room, the team's own unsuppressed gunfire came from inside it, they booted a door of it, or the man at the door has their boots in his ears through it — inside `stunKnowSecs`, and
    ///  with nothing newer (by more than `stunMovedSecs`) putting them somewhere else. A man let go without it walks on as the fight AI's man, as he did before
    ///  anybody carried a charge, and takes the door as the drill the moment his side has it (`stunLate`); a door already marked and not yet banged is let be the
    ///  moment the newest thing they know puts the team elsewhere (`stunOff`: tactics.ts's 'the doorway is plain again').
    #[k(20.0)] pub stun_know_secs: f64,
    #[k(3.0)] pub stun_moved_secs: f64,
    /// A LAST MAN ALONE KEEPS HIS ROOM (on; off = the last of them alive pressed like anybody — 'they've hit the others — get in there!' on a net with
    ///  nobody left on it, and off alone at four rifles; the 10-13 tapes have it five times, each stopped short by a covered lane a moment later). With it on
    ///  he neither starts a press nor walks off on the stock rally: his room, his doors, and the rules for a room's last man (fall back, dig in) as they are.
    #[k(true)] pub last_holds: bool,
    /// THE PICKET (off: every staging as it was; off under hard too — difficulty.ts HARD.picket, built and measured, not shipped; record rebuild/hard-everywhere/depot-and-pairs.md):
    ///  NEVER TWO MEN IN ONE FLASH, in the room the team comes to first. Its men are the ones no rule above reaches: no fight comes before theirs, so they hear
    ///  nothing, go nowhere, and one can through one door folds the pair of them where they stand (measured: worth next to nothing, at standard and hard alike).
    ///  Where that room has a second door, one of the pair stands OUTSIDE it instead — beyond the door farthest on foot from the team's start, off its line, his gun
    ///  on it (where that door opens into the next room, as on the depot and The block, he stands in that room, beside its men). The can does not reach him; he hears the room go, says so, and comes in by his door behind the men who took it. Only that room (a man of any other
    ///  room is one who would have come hunting: posting him outside was measured to make the big floor far easier), only when the two would share a can (burst
    ///  1.6 m inside a door, both within its reach with a line to it), never a man the scenario pins. He can be seen, and dealt with, before the entry
    #[k(false)] pub picket: bool,
    /// READS THE DOORS BETWEEN (off: 51ce9c3's hard run for run; ON under hard — HARD.byWay): where his room stands to the fight decides who goes. NEXT DOOR
    ///  (his room shares a door with the room being fought over): as ever — he goes on his own beat and the farthest man of his room keeps it, because that door
    ///  is the team's next. NOT next door (a corridor, a hall or another room between): nobody is coming for his room next, so nobody is kept and the men of one
    ///  room go TOGETHER. The first to place the noise says so and the men of his room know it from him (a noise heard and not placed is otherwise a man who
    ///  turns to his furniture), the place stays put for `memorySecs` (not lost when the next shot comes to him by another doorway), and the first whose beat is
    ///  up waits for the last of them — no beat is shortened; they leave on the slower man's — and they go by one door (the second takes the first's door and
    ///  role: the other door stays free for a man who goes round), so they are let go the same moment and the fight AI has two men on
    ///  one fix instead of one man walking twenty metres to land alone on a team that has had fifteen seconds to set. Measured (the depot's stores, 19 m and three
    ///  doors from the crew room): the pair that left together was worth three times the two that left one at a time
    #[k(false)] pub by_way: bool,
    /// HARD'S RESCUE CREW (off; ON under hard — HARD.rescueCrew): on a hostage rescue the captor's two men act on his word (rescue.ts crew(): a gun on the
    ///  hostage door from a side room · the door man inside · a roamer who hides and comes up behind — each its own RESCUE row). Nothing in this file reads it
    #[k(false)] pub rescue_crew: bool,
    /// HARD'S DRILLED DEFENDERS (off; ON under hard — HARD.drilled): on the night breach by itself, a defender is half as shaken by a stun canister or a
    ///  boot, so the burst he fires blind down the door he was watching is partly fired instead of lost to the seconds he spent doubled over (roomfight.ts
    ///  DRILLED). Nothing in this file reads it; like every row here it does nothing with `on` off
    #[k(false)] pub drilled: bool,
    /// the way round a flanker will take: metres at most, and against his shortest way to that room
    #[k(46.0)] pub flank_max_m: f64,
    #[k(3.0)] pub flank_k: f64,
    #[k(14.0)] pub flank_slack_m: f64,
    /// the last man's beat before he gives a room up: s, with two of the enemy in view the whole of it
    #[k([0.35, 0.9])] pub last_man_secs: [f64; 2],
    /// s a fight heard elsewhere on the floor keeps a man that ready to give his room up · his beat between the boots at his door and his move
    #[k(90.0)] pub loud_secs: f64,
    #[k([0.8, 1.6])] pub out_secs: [f64; 2],
    /// s from a press's first call until its men stop waiting for more of them (two alive: set; one: he holds the door from outside)
    #[k(8.0)] pub gather_secs: f64,
    /// s more they wait for a man still on his way round
    #[k(6.0)] pub flank_secs: f64,
    /// s a press set on a quiet room's doors holds them from outside before it goes in (0 = in as soon as it is set)
    #[k(25.0)] pub siege_secs: f64,
    /// s between the word and the move: the cue's own length
    #[k(0.9)] pub word_secs: f64,
    /// THE WAY OUT (off: every level as it was; ON under hard — difficulty.ts HARD.wayOut; record rebuild/hard-everywhere/round2-records-pair.md).
    ///  Only on a floor whose structure asks for it (tools/levels/lib/scenarios.ts StructureData.wayOut — levels/records.json: a pair's building, one man in the first room,
    ///  every hunter's way in through the one door two rifles are already on). There the men a lost room draws do not walk into it one at a time: they FORM UP
    ///  on ONE door of it — the nearest to the first of them (the `gather` road above, whatever that row says) — and hold it from back off it: `wayOutM` past
    ///  the reach of the can that comes out of a door first (effects.ts bang.dazzleRadius round where it goes off), each with a line to the door, none in
    ///  another's, out of the place's own light where the floor allows — guns on the door the team has to leave by. Nobody goes round, nobody goes in at the
    ///  noise, a man with no such spot stays home; the lead says it on their net ('back off the doors — out of their flash! let them come out to us!'): that
    ///  line is the cue. They go in on the word only when `wayOutSecs` have brought nobody out. The measured answer is SPEED: they need some eight
    ///  seconds from the door going to be set, and a pair that goes straight on through that door meets them walking (the record's rush row); a can thrown
    ///  long, another door with a can for the hall, or no light in the doorway are a player's to try — not measured
    #[k(false)] pub way_out: bool,
    #[k(0.6)] pub way_out_m: f64,
    #[k(60.0)] pub way_out_secs: f64,
    /// GO AND LOOK (off: every level as it was; ON under hard where the page says so — difficulty.ts HARD.sweep. The maintainer, 10-13: 'harder settings have
    ///  better trained guys, more proactive, higher morale, better kit'). Their side has had word of the team — one of them properly in a man's sight, a loud
    ///  thing in a man's ears — and then NOTHING for `sweepSecs`. On the floor as it was nobody moves again: each man holds the spot the noise left him on
    ///  until the team comes to him, however long they take over it. With this on two of them go and look, together: the holding man nearest the place that
    ///  word put the team who has a mate within `sweepPairM` of him, and that mate, on his word on their net ('gone quiet. with me — we go and look.': that
    ///  line is the cue, `sweepBeat` before either moves), let go at that place as the stock rally lets a man go — the fight AI's own hunt, its doorway drill
    ///  and its search; where they end up calm, they hold. ONE look per word: a place looked at is not looked at again until the team is seen or heard anew,
    ///  and never oftener than every `sweepSecs`. Nobody goes to a room a man of theirs holds (he is looking at it), while a man is still out, a press or a
    ///  fall-back is live or a report stands; somebody always stays (`sweepMen` is never fewer than two nor all of them); and A LOOK THAT LOSES A MAN IS THE
    ///  LAST: nobody else walks out that fight. Never a squad's man (enemysquads.ts has its own form of this), a last stand's, a hostage rescue's, one on
    ///  his way to cover or wedging a door (digin.ts; a man SET on his door may go), or one who knows a covered lane lies that way (lanes.ts). The dead say
    ///  nothing: a man dropped without a sound is no word. No die is drawn.
    #[k(false)] pub sweep: bool,
    #[k(20.0)] pub sweep_secs: f64,
    #[k(2.0)] pub sweep_men: f64,
    #[k(1.5)] pub sweep_beat: f64,
    #[k(12.0)] pub sweep_pair_m: f64,
}
