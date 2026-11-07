// A GUN FROM BEHIND A HELD MAN (the maintainer, 10-12: "when you're holding a hostage, enemies pretty much ignore that you're shooting at them and you can just take
// them down. they should probably run for cover when you take out a gun and actively work on flanking you").
//
// What was there (guards.ts, the human-shield standoff): the men who see a colleague in Sam's arm walk a ring round the pair, 3.5‥7 m off, every ring point
// chosen so the pair stays IN SIGHT, at a walk, chest to him — and under muzzle discipline nobody fires while the man is up. Right for a knife at a throat. With
// the PISTOL at the man's head Sam fires past him: one round a man, at men standing in the open who may not answer, and suppression only roots a standoff man
// where he stands. That is the free win.
//
// The rule here, for the men of that standoff, from the moment one of them has SEEN the pistol in the hold (or a round has left it):
//   COVER   a man in Sam's line of fire, forward of his rear arc, RUNS for the nearest floor where something solid stands between him and Sam's gun (a crouch
//           behind low cover counts; glass does not), and holds there — the standoff is kept without eyes on the pair while somebody of his side has seen
//           them lately;
//   ROUND   the men go round: each plans a way to Sam's back that keeps out of his line of fire (the nav search at a price on floor his gun covers, dearest in
//           front of him), two men opposite ways, bent double behind anything low; with three or more, the man most in front of Sam stays in cover and keeps
//           him talking. A way may cross open floor in his line only in short dashes — a couple of strides with the pistol turned on this man, a longer one
//           while it is on somebody else — else it is no way: he stays behind his cover and is asked again; and a man caught in the open with the pistol
//           coming round onto him breaks for the nearest cover;
//   LOOK    with nobody's eyes on the pair for a few seconds a man in cover steps to its edge (or stands up from behind low cover), looks, and steps back; the
//           pair gone from where they were left = the standoff is lost, and it is the plain hunt on where they were;
//   LINE    a man who gains the clean line at Sam's back (guards.ts flankShot: within 70° of dead astern, the held man well clear of the round, inside 5 m)
//           plants on it — and once Sam has FIRED from behind the man, takes it: single aimed rounds on the standoff's own slow cadence. Never past a civilian,
//           never from anywhere but that line; before Sam fires the line is held silent, as it always was.
// Nothing here adds a man, a hit point or a steadier hand: the same men, walking somewhere else.
//
// ONE MODEL: where Sam is, is where a man of the side last SAW the pair (`Known.sam` — any standoff man's eyes this frame, or his own); nobody is steered by the
// pair's true position through a wall. A round from the hold is known to every man of the standoff (they are inside a room's reach of it by construction).
//
// SWITCH: `GUNHOLD.on` — ON in the page (`?nogunhold` = the ring and the silence exactly as they were), OFF in the module and on the frozen harness stand
// (`TPS_GUNHOLD=1`); a FROZEN_HOLDS row, in the knob registry and a journal's tables (a recording from before the key replays with it off). Off = nothing in this
// file runs: every seam in guards.ts returns on its first clause. Never under a show (`touring`) or `quietUtility`.
//
// STATE: per hold (`Known`, keyed by the Hold object) and per man (`St`, keyed by his Standoff record) in WeakMaps — it dies with the hold / the record, as the
// standoff record itself does; no checkpoint is written with a man in Sam's arm (savestate.ts playerBusy), so nothing here is saved. The die is the game's
// (simRandom). Seams: guards.ts standoffPerceive (gunHoldSee), standoffAlert (gunHoldFeet, gunHoldFree), standoffUpkeep (standoffFixKept), updateGuard's
// suppression line (standoffOnFeet). Check: `bun run tools/qa/gunhold-check.ts` (`probes.ts gunhold`; `GUNHOLD_BREAK=1` must fail).
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Standoff } from './guards';
import './guards';
import './suppression';
import './teams';
import { goneWithTick } from './crossing';

export const GUNHOLD = {
  /** the whole rule. Module default OFF (the frozen stand); the page raises it (src/host/profile.ts pageSwitches; `?nogunhold` holds it off) */
  on: false,
  /** once Sam has fired from behind the man, a man on the clean line at his back takes it. false = cover and the way round as ever, the line held silent */
  fire: true,
  /** how far a man will run for cover, metres of floor */
  coverM: 10,
  /** nobody's cover, and nobody's way round, inside this of the pair */
  nearM: 3.0,
  /** a man this close to dead astern of Sam (degrees) is where he wants to be: he does not run from there */
  rearDeg: 70,
  /** the standoff is kept this long with nobody of the side having seen the pair */
  keepSecs: 14,
  /** nobody has seen them for this long: a man in cover takes a look … */
  peekEvery: 4,
  /** … for this long, then back */
  peekSecs: 0.8,
  /** the longest stretch of a way round across floor his gun covers in front of him: with the pistol turned this man's way … */
  showM: 2.5,
  /** … seconds after a round from the hold that a man the pistol is turned on will cross next to none of it */
  shyS: 6,
  /** … and with it on somebody else (a dash between covers while he looks the other way) */
  dashM: 3.5,
  /** 'turned his way' = the man within this of where the pair faces, degrees */
  faceDeg: 50,
  /** the longest way round, metres */
  flankMaxM: 40,
  /** a man's own beat before his legs answer the gun, seconds [least, most] */
  react: [0.1, 0.4] as [number, number],
};
export const GUNHOLD_DEFAULTS: typeof GUNHOLD = JSON.parse(JSON.stringify(GUNHOLD));

/** what the men of one side know of one hold: when the pistol was first seen in it (−1 never), the last of Sam's rounds from it they know of (−1 none), where
 *  the pair was last seen and which way it faced, when; when a look found them gone; the floor his gun covers from there (lazy, per nav cell: 0 none, 1 standing
 *  only — a crouch hides, 2 open), and where that was worked out from; the man of three or more who stays in front; the side's last line */
interface Known { gunT: number; shotT: number; sam: Vec3; facing: number; fixT: number; goneT: number; expo: Map<number, number>; expoAt: Vec3; expoT: number; talker: Guard | null; saidT: number; saidGun: boolean; saidShot: boolean; }
type Phase = 'react' | 'plan' | 'run' | 'cover' | 'peek' | 'back' | 'flank' | 'line';
/** one man's part: what his feet are doing; the beat before they answer; his cover (and whether it is low: he crouches), the cell at its edge he looks from;
 *  his way round's goal and what the pair looked like when it was planned; the clocks */
interface St { phase: Phase; goT: number; spot: Vec3 | null; low: boolean; edge: Vec3 | null; goal: Vec3 | null; planSam: Vec3 | null; planFacing: number; planT: number; peekT: number; peekEnd: number; peekFrom: number; checkT: number; runT: number; lineT: number; retryT: number; coverBad: boolean; shownT: number; bad: Vec3 | null; saidRound: boolean; }
export interface GunHoldStats { armed: number; ran: number; noCover: number; rounds: number; noWay: number; looks: number; gone: number; lines: number; freeT: number; }

const knowns = new SideMap<object, Known>('gunhold.knowns'); const states = new SideMap<Standoff, St>('gunhold.states'); const statsOf = new SideMap<Game, GunHoldStats>('gunhold.statsOf'); const planned = new SideMap<Game, number>('gunhold.planned');
export function gunHoldStats(g: Game): GunHoldStats { let s = statsOf.get(g); if (!s) { s = { armed: 0, ran: 0, noCover: 0, rounds: 0, noWay: 0, looks: 0, gone: 0, lines: 0, freeT: -1 }; statsOf.set(g, s); } return s; }
export const GUNHOLD_DEBUG: { log: ((s: string) => void) | null } = { log: null };

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** the men of the standoff on this held man, living, in roster order */
function menOn(g: Game, held: Guard): Guard[] { const out: Guard[] = []; for (const o of g.guards) if (o.state !== 'dead' && o.standoff && o.standoff.pair === held) out.push(o); return out; }





// ---------------------------------------------------------------- the lines
const LINES = {
  gun: ['gun — he\'s got a gun! get behind something!', 'he\'s armed — cover, get to cover!'],
  shot: ['he\'s shooting — out of his line, move!', 'shots! get off his line — go round him!'],
  round: ['going round him — keep him talking', 'I\'ll get behind him. keep his eyes on you'],
  gone: ['he\'s moved — they\'re not there!'],
};
/** The same four moments from a DEFENDER of a squad mission (teams.ts defenderWords: a hostile unit under the dialogue pass) — a room fight's man who has
 *  left his post (roomfight.ts mode 'free': neither held nor pinned) comes into the standoff like anybody. The man with the pistol is 'he', as in every other
 *  line of the standoff (guards.ts hostageCall, STANDOFF_LINES); the one in his arm is never called theirs — it can be a civilian. A fighting man's news is
 *  not that there is a gun but where it is: in the hold. The two `round` lines read as class 1 ('coming round') on the cue strip, `gone` as no class of its
 *  own. The pick is made where the line is read (gunHoldPool); the one roll of the die is drawn either way. probes.ts words-lint holds all of it. */
export const GUNHOLD_DEFENDER_LINES: Record<keyof typeof LINES, string[]> = {
  gun: ['pistol\'s out behind his shield — get to cover!', 'he\'s drawn from the hold — get behind something!'],
  shot: ['he\'s firing past his shield — off that line, move!', 'rounds from the hold — get off it, go round him!'],
  round: ['going round behind him — keep his eyes on you', 'coming at his back. hold him where he is'],
  gone: ['he\'s moved — nobody where we left them!'],
};
/** (WORDS) a line of either table as it is said now, by the line it stands for (read in bark, where the line is said; cues.ts classes the new words as it classed the old) */
export const GUNHOLD_NOW: Record<string, string> = {
  'shots! get off his line — go round him!': 'shots! get out of his line of fire — go around him!', 'going round him — keep him talking': 'going around him — keep him talking',
  'he\'s drawn from the hold — get behind something!': 'he\'s pulled a pistol — get behind something!', 'rounds from the hold — get off it, go round him!': 'he\'s shooting — get out of his line of fire, go around him!',
  'going round behind him — keep his eyes on you': 'going around behind him — keep his eyes on you', 'coming at his back. hold him where he is': 'coming up behind him. keep him where he is',
  'he\'s moved — nobody where we left them!': 'he\'s moved — they\'re not where we left them!',
};
/** The pool a man draws one of the four moments from: a defender's own where the dialogue pass is on, else the staff's. */
export function gunHoldPool(gd: Guard, kind: keyof typeof LINES): string[] { return goneWithTick('gunHoldPool') as any; }


/** Panel / QA: 'gun in the hold ▸ fired · Kowalski: behind cover (low) · Reyes: going round, 6.2 m to go' — empty with nothing to say */
export function gunHoldSummary(g: Game): string {
  const H = g.player.holding, K = H ? knowns.get(H) : undefined; const s = gunHoldStats(g);
  const tail = `${s.ran} ran for cover · ${s.rounds} went round · ${s.lines} lines · ${s.looks} looks`;
  if (!GUNHOLD.on) return `gun in the hold ▸ off`;
  if (!H || !K || K.gunT < 0) return `gun in the hold ▸ — (${tail})`;
  const parts: string[] = [K.shotT >= 0 ? 'he has fired' : 'pistol seen'];
  for (const gd of menOn(g, H.g)) { const st = gd.standoff && states.get(gd.standoff); if (!st) continue;
    const what = st.phase === 'run' ? 'running for cover' : st.phase === 'cover' ? `behind cover${st.low ? ' (low)' : ''}` : st.phase === 'peek' || st.phase === 'back' ? 'taking a look' : st.phase === 'flank' ? `going round${st.goal ? `, ${flat(gd.char.pos, st.goal).toFixed(1)} m off his spot` : ''}` : st.phase === 'line' ? 'on the line at his back' : 'reacting';
    parts.push(`${gd.callsign}: ${what}`); }
  return `gun in the hold ▸ ${parts.join(' · ')} (${tail})`;
}
/** QA: this man's part, by name (null = none) */
export function gunHoldPhase(g: Game, gd: Guard): Phase | null { return goneWithTick('gunHoldPhase') as any; }
