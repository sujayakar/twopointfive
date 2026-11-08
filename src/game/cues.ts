// CUES — what the defenders decide, told to the player (the maintainer, 10-07, of defenders that hold their rooms after he is down: "maybe we just need more
// telegraphing from the NPCs then"; earlier "lean into unpredictability but with cues to be able to know what's happening"). Game.cues.
//
// THE RULES BELOW ARE THE RUST GAME'S (rust/game_rules/src/cues), and what a HUD is told of them is the view's (game.view.radio, game.view.bubbles). What is left here is
// the words' tables and the record the bridge fills.
//
// They write nothing a man's AI, the dice or a digest reads: the rows and the bubbles live on the Cues record, never on a Guard
// (Guard.bubble is read by the bark clocks — suppression.ts, the stand-off's — and a line put there would move a die). A cue never moves a man.
//
//   1 · THE NET IS DELIVERED. A unit's radio line went into the log as '<callsign> (radio): …', which the strip (ui/overlay.ts)
//       never drew: no scenario defender's call ever reached the HUD. say() (guards.ts) now hands a room fight's defenders' radio lines here as well, and the
//       strip draws this file's rows (the team's own net, a showcase's cast and the night game's response are as they were). The rows are their own short
//       list: a line nothing draws can no longer push a drawn one out of the five-row message ring.
//   2 · PRIORITY AND RATE. About forty defender lines go out in a loud run and the strip has three rows of eight seconds. Every row has a CLASS (the order is
//       what a player would most want to know): 1 men coming round or coming to help · 2 stood down · 3 holding on purpose · 4 the net knows · 5 fell back
//       and planted · 6 alert but staying put (CLASSES for the lines they already had, by their words; LINES carries each new line's own). A squad's word
//       ranks a step ahead of a man's in its class. A row is NOT drawn when: the same words (whoever says them) went out inside `dupSecs`; two rows are up
//       and its class spoke inside its gap (`gaps`); the strip is full and every row up is better, or as good and not yet `minShow` old. A full strip gives
//       up, in order: the oldest row of a class holding more than `perClass` rows, the older of the new row's own class once it holds `perClass`, else the
//       worst-ranked row's oldest. A row that arrives squelches (once a frame).
//   3 · THE SILENT DECISIONS GET WORDS (LINES — one table). Each is read off what the man has already done, the frame he did it: the room fight's own notes
//       (RoomFight.log — the squads' and the press's come through it) and each defender's state. Nothing is said that he is not doing.
//         help       he leaves his room for the fight next door (he shouts it, too)   net   'moving to the fight — the mess'
//         stayOurs   he hears his side hit next door and the dice say stay            net   'they're hit in the mess — staying on my door'
//         stay       … or nobody of his side is posted where it came from             net   'shots outside the store — holding my room'
//         huntOver   his hunt is over, he holds where it ended                        net   'lost them — holding outside the store'
//         calm       he lowers his gun (state → patrol), still on his post            net   'quiet now — staying put'
//         cover      Sam is down at his feet: he keeps a gun on him                   net   'one of them down in the mess — covering him'
//         still      alert these `stillSecs` with nobody in sight, not moving         net   'they're still out there — holding the mess'
//         fellBack   he gives his room up (he shouts it, too)                         net   'the mess is lost — falling back'
//         planted    … and where he has set up again (pinnedIn: back in that room)    net   'fell back — set outside the mess'
//         squadHears a squad has word of the team and has not yet said what it does   net   'Bravo: copy, Alpha — stand by'
//         squadWaits … on Hard, a squad that could send men holds on purpose (drillNet)  net   'Bravo: copy, Alpha — hold, let them come in. Wait for my word!'
//         squadStays asked again, it stays on its doors                               net   'Bravo: still on the doors — nobody goes'
//         squadQuiet its rooms have gone quiet                                        net   'Bravo: gone quiet here — still holding'
//         surprised  caught looking the wrong way · flinch at a booted door           spoken (a bubble: he is in the room the team is entering)
//       A cue's bubble is drawn like any line's (ui/bubbles.ts: over a man the team sees, in the view). One man: a cue at most every `manSecs`, none on top
//       of a line of his own (bar the two he also shouts); the floor: KIND_GAP between two of a kind.
//   The one line that said what nobody then did — 'back to your posts', from a man whose side then stands where it is — is reworded for a unit (guards.ts),
//   and a place on the net is named by the level's own rooms (dialogue.ts placeName: 'coming at him from the car park' on a floor with no car park).
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import type { Game, Guard } from './game';
import { bubbleSecs } from './guards';
import { w, wg } from './words';
import './owner';
import './pressure';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

export const CUES = {
  /** the switch: off = none of this file runs, the strip and every line as they were */
  on: true,
  /** the strip: rows drawn at most · s a row stays · s a row is owed before one of its own class may take its place · rows one class may hold at once */
  rows: 3, secs: 8, minShow: 2.5, perClass: 2,
  /** s inside which the same words are not drawn twice */
  dupSecs: 10,
  /** s between two rows of one class (index = class; 0 = a row that was always on the strip: control, the staff's net) */
  gaps: [0, 1.2, 3, 3, 2.5, 3, 12],
  /** every row of their net that reaches the strip squelches as it arrives (gain; 0 = silent) */
  squelch: 0.35,
  /** s between two cues from one man · s between two of his 'stay' / 'still' lines */
  manSecs: 2.5, againSecs: 28,
  /** s alert with nobody in his sight, standing, before he says he is still holding · … and with no shot in his ears */
  stillSecs: 8, stillQuiet: 5,
  /** s after a loud thing outside his room before a man who has not gone says he is staying */
  staySecs: 2,
  /** A SHOUT HEARD (overheard(), below): a defender the fog hides who SHOUTS a line — not on his net — where the player's ears or a living teammate's really
   *  get it is a quiet row of its own on the strip: his words alone, no name, no place. OFF here and on the frozen stand (`TPS_OVERHEARD=1` raises it); the
   *  page has it on (host/profile.ts: `?overheard=0` = such a shout is no row, as it was) · s between two such rows. How far a shout carries is not a number
   *  of this table's: digin.ts DIGIN.shoutM, the one the defenders' own shouted cues are heard by. */
  overheard: false, overheardGap: 4,
};
/** `?nocues` / `?cues` set the switch for the visit */
try { const q = new URLSearchParams(location.search); if (q.has('cues')) CUES.on = q.get('cues') !== '0'; if (q.has('nocues')) CUES.on = false; } catch { /* no page (the harness) */ }

/** THE RANKING of the lines the defenders already had: a radio line's class by its words, first match (what a player would most want to know first —
 *  rebuild/telegraphing/decisions.md §7); a line no row knows is the net knowing something (4). The new lines below carry their class themselves. */
const CLASSES: [RegExp, number][] = [
  [/fall(ing)? back|fell back|is lost|out the back|get back|behind me|i'm coming to you/i, 5],
  [/still out there/i, 6],
  [/hold where you are|lost them|quiet now|standing down|gone quiet|covering him|not answering — going back/i, 2],
  [/going round|going around|moving round|moving around|locked down\. box him in|moving to|moved on — after them|coming to you|coming at|coming up behind|with you|get round|get around|hit the others|on that door|get in there|in position|^set\b|go go go|go — go|go, go|pushing|on my way|two on this room|everybody, go|stand and fight|charge first|moving up behind|coming in — i|bang out|move up|putting fire|covering — move|^moving!/i, 1],   // (… and Hard's own men on the move: stand to's word, the block's charge, the rescue crew's two — pressure.ts, tactics.ts, rescue.ts; a covered lane's tells — the can going at its guns, the men moving up once it is found empty, fire from a corner and, with `pinGo`, the run-in under it: lanes.ts)
  [/lock the doors|on the doors|back on our doors|stay on your doors|digging in|getting ready|ready for them|room ready|staying on my door|holding my room|staying on my post|holding my post|let them come|not coming out|holding my corner|holding here|i've got this door|on my own/i, 3],
];
export function cueClass(text: string): number { for (const [re, c] of CLASSES) if (re.test(text)) return c; return 4; }

export type CueKind = 'help' | 'stay' | 'stayOurs' | 'huntOver' | 'calm' | 'cover' | 'still' | 'fellBack' | 'planted' | 'pinnedIn' | 'squadHears' | 'squadCopies' | 'squadWaits' | 'squadCopiesWaits' | 'squadStays' | 'squadQuiet' | 'surprised' | 'flinch';
/** THE NEW LINES: {at} = 'in the mess' / 'outside the mess', {the} = 'the mess' / 'outside the mess', {sq} / {ally} = squad names. cls = its row of the ranking
 *  on their net (the strip) — 0 = spoken only, not a net line; every one is also a bubble over a man the team can see. `now` = the same lines, place for place, as
 *  the dialogue pass has them (words.ts; read through `said()`): what he heard may have been a boot or a charge, so he does not call it 'shots', and a man told to stay
 *  is on his POST — he may have no door and no room of his own. */
const LINES: Record<CueKind, { cls: number; say: string[]; now?: string[]; /** (WORDS.guide) the same lines as the writing guide has them, place for place */ guide?: string[] }> = {
  help: { cls: 1, say: ['moving to the fight — {the}', "they're hit {at} — moving to them"] },
  huntOver: { cls: 2, say: ['lost them — holding {the}', 'lost them — staying {at}'] },
  calm: { cls: 2, say: ['quiet now — staying put', 'standing down — holding here'] },
  cover: { cls: 2, say: ['one of them down {at} — covering him'] },
  squadQuiet: { cls: 2, say: ['{sq}: gone quiet here — still holding'] },
  stayOurs: { cls: 3, say: ["they're hit {at} — staying on my door", 'shots {at} — staying on my door'], now: ["they're hit {at} — staying on my post", 'that was {at} — staying on my post'] },
  stay: { cls: 3, say: ['shots {at} — holding my room', 'heard that, {the} — holding my room'], now: ['that was {at} — holding my post', 'heard that, {the} — holding my post'] },
  squadStays: { cls: 3, say: ['{sq}: still on the doors — nobody goes'] },
  squadHears: { cls: 4, say: ['{sq}: heard that — stand by'] },
  squadCopies: { cls: 4, say: ['{sq}: copy, {ally} — stand by'] },
  squadWaits: { cls: 3, say: ['{sq}: hold — let them come in. Wait for my word!'], now: ['{sq}: hold — let them come in. wait for my word!'], guide: ['{sq}: hold — let them come in. wait for my call!'] },   // (enemysquads.ts drillNet: a drilled garrison's squad that could go, holding on purpose — its word comes later than a 'stand by' squad's, and may be a pair going round)
  squadCopiesWaits: { cls: 3, say: ['{sq}: copy, {ally} — hold, let them come in. Wait for my word!'], now: ['{sq}: copy, {ally} — hold, let them come in. wait for my word!'], guide: ['{sq}: copy, {ally} — hold, let them come in. wait for my call!'] },
  fellBack: { cls: 5, say: ['{the} is lost — falling back', 'falling back — {the} is lost'] },
  planted: { cls: 5, say: ['fell back — set {at}'] },
  pinnedIn: { cls: 5, say: ["can't get out — fell back to cover {at}"] },
  still: { cls: 6, say: ["they're still out there — holding {the}", 'still out there — staying on this door'], now: ["they're still out there — holding {the}", 'still out there — staying on my post'] },
  surprised: { cls: 0, say: ['behind—!', 'there—!'] },
  flinch: { cls: 0, say: ['the door—!'] },
};
/** a kind's lines as they are said now: the pass's where it has any — as many as before, so the pick falls on the same place in the list either way */
const said = (k: CueKind): string[] => w(wg(LINES[k].guide, undefined) ?? LINES[k].now ?? LINES[k].say, LINES[k].say);
/** every new line's words (the QA probe: each falls in a row of CLASSES, none is said twice by the table) */
export function cueLines(): { kind: CueKind; cls: number; text: string }[] { return (Object.keys(LINES) as CueKind[]).flatMap(k => said(k).map(text => ({ kind: k, cls: LINES[k].cls, text }))); }

export interface CueRow { t: number; text: string; cls: number; /** class × 2, a squad's word a step ahead of a man's */ rank: number; who: string }
/** a line is SHOUTED when its writer put an exclamation mark in it ('find them!', 'grenade!'): the tables' own mark, and the only lines that carry through a wall */
export const isShout = (text: string) => text.includes('!');
type Bubble = { text: string; t: number; dur: number; radio: boolean };
interface Man { state: string; alertT: number; saidT: number; againT: number; stoodT: number; moved: 'fallback' | 'cover' | null; from: string; noiseT: number; ep: number; rallyAt: number; covered: boolean }
const newStats = () => ({ /** unit radio lines handed over by say() · of them drawn */ heard: 0, heardDrawn: 0, /** shouts of hidden defenders offered (CUES.overheard) · of them drawn as a row heard */ shouts: 0, shoutsDrawn: 0, /** new lines said · of them on the net · of those drawn */ cues: 0, cueNet: 0, cueDrawn: 0,
  /** rows not drawn: the same words just went out · its class had just spoken · the strip was full of better rows */ dup: 0, gap: 0, full: 0, /** rows that had a drawn row's place · of them rows not yet 1.5 s old */ bumped: 0, bumpedYoung: 0,
  /** rows offered · drawn, by class (index 0 = the rows that were always on the strip) */ filed: [0, 0, 0, 0, 0, 0, 0], drawn: [0, 0, 0, 0, 0, 0, 0] });

export class Cues {
  /** the strip's rows, oldest first (at most CUES.rows alive) */
  rows: CueRow[] = [];
  stats = newStats();
  /** every new line said, by kind (the QA tally) · every row filed, newest last (capped): what was drawn and what was not, and why */
  kinds: Partial<Record<CueKind, number>> = {};
  log: { t: number; who: string; text: string; cls: number; kind: CueKind | 'heard' | 'raw' | 'call'; drawn: boolean; why: string }[] = [];
  private bubbles = new SideMap<Guard, Bubble>();
  private men = new SideMap<Guard, Man>();
  private lastNote: unknown = null; private lastT = -1; private said = new Map<string, number>(); private classT: number[] = []; private squadT: number[] = []; private kindT: Partial<Record<CueKind, number>> = {}; private n = 0; private squelchDue = false;
  /** the one shout heard on the strip, if any (overheard): a field of its own, never one of `rows` — nothing that files, ranks or reads the net's rows meets it */
  heardRow: { t: number; text: string } | null = null;
  /** every hidden defender's shout offered, newest last (capped): drawn or not, and why not ('dup' · 'gap' · 'full' = no free row · 'far' = no ear in reach) */
  heardLog: { t: number; text: string; drawn: boolean; why: string }[] = [];
  private heardSaid = new Map<string, number>(); private shoutT: number | undefined = undefined;
  constructor(private g: Game) {}

  /** the encounter is another one (a restart, a checkpoint loaded: the clock may have gone back) */
  reset() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Cues.reset', this, []); if (__r !== RUST_PASS) return __r as any; } } this.heardRow = null; this.heardLog = []; this.heardSaid.clear(); this.shoutT = undefined; this.rows = []; this.bubbles = new SideMap(); this.men = new SideMap(); this.lastNote = null; this.said.clear(); this.classT = []; this.squadT = []; this.kindT = {}; this.squelchDue = false; this.stats = newStats(); this.kinds = {}; this.log = []; }

  // ---------------------------------------------------------------- the strip
  /** Game.msg: a row written 'radio: …' (control, the staff's net, Sam's own call) — on the strip as it always was */
  raw(text: string) { RUST_SURFACE?.call('Cues.raw', this, [text]); }
  /** guards.ts say(): a UNIT's line went out on his side's net — the strip gets it, under his callsign (a squad's word is already under the squad's name) */
  heard(gd: Guard, text: string) { RUST_SURFACE?.call('Cues.heard', this, [gd, text]); }
  /** laststand.ts, enemysquads.ts's radio check: a defender's line on his side's net that is TELLING only — the strip's row under his callsign (a squad's
   *  word is already under the squad's name) and this file's own bubble over him, never Guard.bubble (the bark clocks and the digest read that) */
  net(gd: Guard, text: string, cls: number): void { if (RUST_SURFACE?.tapFiring('Cues.net')) return RUST_SURFACE.tapAnswer() as any; return goneWithTick('Cues.net') as any; }
  /** fogofwar.ts: a TEAMMATE calls what the fog's cone hides ('contact left!') — this file's own bubble over him (never Guard.bubble) and a row on the strip
   *  under his callsign, in the first class (men coming), logged under a kind of its own ('call': a tally of the defenders' 'heard' lines never counts the team's).
   *  Returns whether the strip drew it */
  call(gd: Guard, text: string): boolean { return crossToRust('Cues.call', this, [gd, text]) as any; }
  /** guards.ts say(): a line a man SPOKE, not one on a net. A defender the fog hides who SHOUTS it (isShout) where an ear of the team's really gets it — the
   *  shout's own law (digin.ts heardBy: the path under DIGIN.shoutM × the propagation's carry) — is a row of its own kind on the strip: HIS WORDS ALONE, no
   *  name and no place, drawn as something heard (stripRows: 'heard: …'). It never takes a row's place: no free row, no row, and the first row filed onto a
   *  full strip ends it (file()). One such row at a time; the same words not twice inside `dupSecs`, two of them `overheardGap` s apart; no squelch (it is
   *  not on the net), no class clock (laststand.ts netWait never waits on it). The cheap questions come first, and the hearing model is asked WITHOUT keeping
   *  the answer (hearPath's `keep`): the row leaves no route in the cache the AI's own hearing reads. Returns whether the strip drew it. */
  overheard(gd: Guard, text: string): boolean { return RUST_SURFACE?.call('Cues.overheard', this, [gd, text]) === true; }
  /** deathrecap.ts: a word over a man for the player's eyes alone — the bubble, no strip row, no class clock: nothing the net's own lines wait on (laststand.ts netWait); false with the cues off (nothing is drawn) */
  aside(gd: Guard, text: string): boolean { if (!CUES.on) return false; this.bubbles.set(gd, { text, t: this.g.time, dur: bubbleSecs(text), radio: false }); return true; }
  /** laststand.ts: seconds until the strip would take a line of this class without dropping it for the class's gap (0 = now, and with the cues off) */
  netWait(cls: number): number { return goneWithTick('Cues.netWait') as any; }
  /** savestate.ts: the message ring has been put back as it was saved — its 'radio: ' rows are the strip's again */
  adopt(msgs: { text: string; t: number }[]) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Cues.adopt', this, [msgs]); if (__r !== RUST_PASS) return __r as any; } } this.heardRow = null; if (CUES.on) this.rows = msgs.filter(m => m.text.startsWith('radio: ')).slice(-CUES.rows).map(m => ({ t: m.t, text: m.text.slice(7), cls: 0, rank: 1, who: '' })); }
  /** what a man is saying on this copy (deathrecap.ts witness alone: the HUD is told by the game, game.view.bubbles) */
  bubbleOf(gd: Guard): Bubble | null {
    const b = gd.bubble, c = this.bubbles.get(gd), t = this.g.time;
    return !c || t < c.t || t - c.t > c.dur || (b && b.t >= c.t) ? b : c;
  }

  // ---------------------------------------------------------------- once a frame, after everybody has moved
  tick(): void { return crossToRust('Cues.tick', this, []) as any; }

  /** 'on · 14 rows drawn of 41 · 9 new lines' — the panel's line */
  summary(): string {
    if (!CUES.on) return 'off';
    const S = this.stats, drawn = S.heardDrawn + S.cueDrawn, of = S.heard + S.cueNet;
    return `${drawn} of ${of} net lines drawn · ${S.cues} new lines${S.dup + S.gap + S.full ? ` · held back: ${S.dup} said already, ${S.gap} too soon, ${S.full} outranked` : ''}`;
  }
}
