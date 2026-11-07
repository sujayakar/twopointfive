// A PARTNER'S MEN ARE ON SAM'S CARD — a quiet mission as two. killGuard (combat.ts) prices a man only when the PLAYER put him down: his partner's round arrives
// as 'guard', and so does the takedown the partner was ordered to (pairorders.ts) or took on his own when a man had seen them (spotted.ts). So a run where the
// partner shot the whole shift read 'kills 0 · no kills · pacifist'. With the switch on, a defender the partner put down this step is tallied as the player's
// would be — a round that kills is a kill, anything else a knockout, a round into a man is a round near a man — and gets its row on the timeline.
//   THE MODULE READS THE FLOOR AND WRITES ONLY THE RUN'S NUMBERS (mission.stats, the chronicle): it moves nobody and draws no die. Stateless: a man is counted
//   the step he goes down (`diedAt` is this step's clock), so a checkpoint carries nothing of it.
//   WHOSE: a round is the partner's when the man's `killerId` is his (combat.ts sets it the frame the round lands); a man STRUCK or CHOKED by 'guard' is the
//   partner's whoever is named — no defender lays a hand on his own. A defender's stray round into his own side stays on nobody's card, as it was.
//   SWITCH: TEAMTALLY.on — ON in the page (`?noteamtally` = only the player's own are counted, as it was), OFF here and on the frozen stand
//   (TPS_TEAMTALLY=1 raises it there): off = the card as it was, by run digest. The maintainer's yes: 15:33Z 10-13.
import './teams';
import './mission';
import './chronicle';

export const TEAMTALLY = { on: false };

