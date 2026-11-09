//! The numbers of roomfight.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RoomfightK {
    /// s for a piece of evidence at an entry to fade to a third
    #[k(12.0)] pub evidence_secs: f64,
    /// an entry already being watched keeps this much over a rival (no flicking between two doors) / each mate already on an entry makes it this much less his
    #[k(0.25)] pub sticky: f64,
    #[k(0.2)] pub crowd: f64,
    /// below this no entry is feared: he faces the way he was put
    #[k(0.15)] pub fear_min: f64,
    /// operatives step out of an open doorway they have stopped in / to the side when a friend stands in their line
    /// who he shoots first: a man he sees inside `closeM` with his muzzle ON him comes before the man he is already on, and a man he has had no clear round at
    ///  for `noShot` s (furniture, a jamb, a leaf in the line) has no hold on him — the next man he sees is his (false = as it was: once on a man he stayed on him
    ///  while he could see him, whoever else walked up)
    #[k(2.5)] pub close_m: f64,
    #[k(0.4)] pub no_shot: f64,
    #[k(2.0)] pub point_blank: f64,
    /// s a teammate WITH SOMEWHERE TO BE (walking on an order: following, to a stack, to a mark) keeps his gun on a man he has no round at — glass, a jamb,
    ///  furniture in the line — before he says so, lets him go for `noShotPass` s and walks on with his gun where his order has it; a man stood on his post keeps
    ///  him (that is what he is there for), and the moment there IS a round at the man he is his again. 0 = as it was: he walked on backwards with his muzzle (and, once
    ///  the lights were free, his light) on a man behind a pane for as long as he could see him — with night vision, the whole length of a corridor
    #[k(1.5)] pub no_shot_go: f64,
    #[k(4.0)] pub no_shot_pass: f64,
    /// how readily a defender goes to a fight next door (`rally`): × his side's chance to go each time he weighs it (FIGHT_SKILL rallyChance, 0.75 a roll). 1 = as it
    ///  was; under 1 more of them are still in their own rooms when the team gets there; 0 = nobody leaves (the `rally` switch off). The dice are drawn either way
    #[k(1.0)] pub rally_k: f64,
    /// a man HOLDING an arc he was given (orders.ts holdArcs: a watch point, on his mark) answers a mate's call or a footfall only inside this many degrees of it
    ///  — the other half is the other man's; 180 = as it was (he turned to any call he had a line to, and the two of them ended facing one way)
    #[k(100.0)] pub arc_half_deg: f64,
    /// A MAN TOLD TO COVER KEEPS HIS ANGLE (the maintainer, of a man covering a lane who swung a quarter turn to a man two mates were already shooting, the
    ///  lane's own men still coming: "doesn't watch his angle even though red team has it covered"). With `coverKeep` a man stood at the post of a cover order
    ///  passes over an enemy who is more than `coverHalfDeg` off the way he was told to watch WHILE a mate's gun is on that man with a round at him: the man is
    ///  seen (his meter runs) and is not his — not to his eyes, and no call, shot or footfall of that man's turns his look. The man is his again the moment no
    ///  mate has him (the mate is behind a jamb, reloading, has a friend in his line or his muzzle down), his muzzle comes onto this man, or he is inside
    ///  `coverNearM`; a man he is already on he hands over only when the mate is past his own beat to fire. A look he had begun at such a man is dropped.
    ///  Not covered: coverfire.ts's rounds into cover at a man a mate knows of. Off = as it was: his eyes took any man they saw.
    #[k(45.0)] pub cover_half_deg: f64,
    #[k(6.0)] pub cover_near_m: f64,
    /// s a teammate stays ROUSED after a shot, a charge or a boot last reached his ears: a sighting alone is a man to him (no making the shape out first) and he
    ///  reads at the roused rate. 10 = as it was — a quarter of a minute after a room was taken the team looked as calm men look, and a defender walking in through
    ///  a connecting door two metres off had three tenths of a second on the man watching it
    #[k(45.0)] pub team_hot: f64,
    /// s a teammate holds off a man for a friend in his line, with nowhere to step, before a hostile's round from somewhere else takes his eyes: he lets the
    ///  masked man go for a couple of seconds and turns to the one shooting (0 = never, as it was: he stood on the masked man until he was shot from behind)
    #[k(0.8)] pub let_go: f64,
    /// the man whose look a footfall is, with NO line to the walker (a shut door, a corner), brings his gun onto where the sound comes from — the footfall's own
    ///  spot through a leaf, the doorway or corner it comes round otherwise — and holds it there while the steps keep coming, `stepLookSecs` at most a look; a shot or a mate's call takes
    ///  the look over, and a man on his way to a door's mark or through it keeps his corner (off = as it was: he looked only for a man he already had a line
    ///  to, and boots running up to a shut door two metres behind him turned nobody)
    #[k(5.0)] pub step_look_secs: f64,
    #[k(3.5)] pub door_look_secs: f64,
    #[k(8.0)] pub door_shot_m: f64,
    /// the feet half of guards.ts LIGHTS.litMan (that switch is this rule's too — see there): a teammate WALKING on an order who stands in the dark on a man
    ///  standing in the place's light, with SAM in his line to him — his gun is down for it (samLine) — stops where he is until the line is clear, this many
    ///  seconds at most a hold (0 = his feet are never held). Only a walk that closes on the man (the leg he is on ends nearer the man than he stands: a word that pulls
    ///  him back is walked), never while he stands in an open doorway's funnel, and only for the player: a mate in the line is the side step's, and moves on by
    ///  himself. Only from the dark, by measurement: the same hold by a man whose light was on
    ///  stood him still in front of an aimed enemy and lost both teammates most of the time
    #[k(4.0)] pub hold_short_secs: f64,
    /// a moving defender stops to fight an enemy seen inside this many metres
    #[k(7.0)] pub move_engage_m: f64,
    /// the light read (0‥1, as a meter reads) for a man whose lit weapon light has the watching eye inside its cone: the lens is the brightest thing in view
    #[k(0.7)] pub lamp: f64,
    /// a man who shows within this many degrees of where a raised gun already points is shot at from that side's `preaim` track, not from nothing
    #[k(12.0)] pub preaim_deg: f64,
}

/// DRILLED DEFENDERS — hard on the night breach (difficulty.ts HARD.drilled → pressure.ts PRESSURE.drilled).
///  One room, three men, no warning: the canister blinds all three and the room is taken in
///  two seconds, so nothing a defender could do smarter comes before he is shot. What is left is how a man takes a bang, and that is training — nerves only, no
///  reaction, aim or hit-point number is in it. A drilled man is blind exactly as long as anybody (combat.ts FLASH.secs, untouched) and breaks as often
///  (bangBreak, untouched); he is HALF AS SHAKEN — `shaken` × the seconds a stun canister doubles him over (FLASH.stagger 2.5‥1.0 s → 1.25‥0.5: combat.ts
///  detonateStun asks `drilledK`), × his flinch at a boot (startle 0.4‥0.65 s → 0.2‥0.33) and × the daze once his eyes clear (daze 1‥2 s → 0.5‥1).
///  So his blind burst — the room fight's own, down the door HE WAS WATCHING, twelve turns from three tenths after the flash — is partly fired instead of
///  lost whole to the seconds he spent doubled over, and it starts AFTER the entry's own beat of silence (doortools.ts burstDry: 0.6 s) has sent the lead in:
///  that is the bite. The first of them to fire shouts it. A breaching charge folds him as long as anybody (doortools.ts staggerSecs, untouched: still no
///  burst back through that doorway), and the door worked from the threshold is the other answer.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DrilledK {
    #[k(0.5)] pub shaken: f64,
}
