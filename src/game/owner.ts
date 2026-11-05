// WHO HAS THIS MAN: THE ONE RECORD. Four columns of the men's table (src/sim/men/schema.ts) and nothing else writes them:
//   owner   a word of claims, a bit a rule. A man is held by more than one rule at once all the time (his squad and a lane, a press and a last stand), so the word is every rule
//           that has a hand on him, and the highest bit set is the rule that HAS him.
//   rank    that top bit's place (1 the weakest; 0 nobody).
//   reason  which of the top rule's reasons it gave (an index into its `why`).
//   since   the game time that rule came to the top.
// A rule says so as it takes him and as it lets go (`claimMan`, `releaseMan`), or as the one tick its lists can change in ends, and whoever wants to know reads the word (`hasHim`, `ownerOf`). What
// a rule keeps about what it is doing with him stays the rule's own, and an answer that turns on what moves between its ticks (who is alive, what he just heard, a time) is the rule's own read, not the word. Nothing here decides anything: an asker's own list of who it yields to is a mask written where it asks.
//
// THE RANKS ARE THIS TABLE AND ONLY THIS TABLE, strongest first, in the order the game decides by: `updateGuard` for his body, then who may move him off his post. A rank decides who
// is shown as having him, never who may take him. Where two modules disagree today about who outranks whom the table cannot say so and does not settle it: the list is
// docs/agents/enemies.md, "who has a man".
//
// Imports nothing at run time (the module is a leaf of src/game's cycle, so a top-level `ownerWitness` line in a rule's file is safe wherever the cycle starts). TypeScript for now;
// `claimMan` and `releaseMan` move to Rust with the behaviours (wave 3 of the port), and this file is small so that it can.
import type { Guard } from './game';

/** the rules that can hold a man, strongest first. `why[0]` is what is shown when nothing more is said, a lower claim coming back to the top included */
const RULES = [
  // HIS BODY: as `updateGuard` tries them (dead, held and script are his own fields, not claims: `ownerOf` reads them in their places)
  { name: 'frag', why: ['a live grenade he knows of has a line to him'] },
  { name: 'runner', why: ['has said he is done', 'dropped it and ran'] },
  { name: 'morale', why: ['a blast has him ducked or broken'] },
  { name: 'startle', why: ['caught alone at first sight, running for cover'] },
  // WHO MAY MOVE HIM OFF HIS POST
  { name: 'stand', why: ['one of the last stand: in its room, on his way to it, or told to hold where he is'] },
  { name: 'squadLeg', why: ['his squad has sent him, or is about to'] },
  { name: 'squadWord', why: ['his squad has heard and not yet said what it does, or has him going round or looking'] },
  { name: 'lastMan', why: ['the last man up, at his act'] },
  { name: 'lane', why: ['held short of a covered lane, or looking round its corner'] },
  { name: 'press', why: ['the press has him, or he stays to keep the room it leaves'] },
  { name: 'walk', why: ['walking to the post of a mate who does not answer, nothing learned of the team by it'] },
  { name: 'breaker', why: ['out for the lights, or waiting to be sent'] },
  { name: 'buddy', why: ['a colleague is on his way to him, or the two have just paired up'] },
  { name: 'needs', why: ['off for a coffee, a smoke, some water or a stretch'] },
  { name: 'squad', why: ['at rest, his squad\'s call'] },
] as const;
export type OwnerRule = typeof RULES[number]['name'];

const TOP = RULES.length - 1;
/** a rule's bit: higher is stronger */
const bit = (i: number): number => 1 << (TOP - i);
if (RULES.length > 31) throw new Error('owner.ts: more rules than the word has bits');

/** each rule's bit, for `claimMan`, `releaseMan` and `hasHim` (several `|`d together for an asker's own list) */
export const OWN: Readonly<Record<OwnerRule, number>> = Object.fromEntries(RULES.map((r, i) => [r.name, bit(i)])) as Record<OwnerRule, number>;

/** the rank a word of claims comes to: 1 the weakest rule, 0 nobody */
const rankOf = (word: number): number => 32 - Math.clz32(word);
/** the table's row of a rank */
const rowOf = (rank: number) => RULES[TOP + 1 - rank];

/** the checks' hands on the record (tools/qa/owner-check.ts): nothing with them unset */
export const OWNER_QA: { read: ((gd: Guard, rules: number) => void) | null; drop: number } = { read: null, drop: 0 };

/** THE WORD CHANGED: if the rule on top changed, it has him from now, for its first reason */
function settle(gd: Guard, word: number, t: number, why: number): void {
  gd.owner = word;
  const rank = rankOf(word);
  if (rank === gd.rank) { if (why >= 0 && rank > 0) gd.reason = why; return; }
  gd.rank = rank; gd.reason = why >= 0 ? why : 0; gd.since = t;
}
/** RULE `rule` TAKES HIM (nothing changes if it has him already, but for the reason it gives). `t` the game's clock; `why` an index into its row's `why` */
export function claimMan(gd: Guard, rule: number, t: number, why = 0): void {
  if (OWNER_QA.drop & rule) return;
  const was = gd.owner, word = was | rule;
  if (was === word) { if (rankOf(word) === rankOf(rule)) gd.reason = why; return; }
  settle(gd, word, t, rankOf(rule) === rankOf(word) ? why : -1);
}
/** RULE `rule` LETS GO. The rule below it, if any, has him again from now */
export function releaseMan(gd: Guard, rule: number, t: number): void {
  const was = gd.owner; if (!(was & rule)) return;
  settle(gd, was & ~rule, t, -1);
}
/** does any of these rules have a hand on him? */
export function hasHim(gd: Guard, rules: number): boolean {
  if (OWNER_QA.read) OWNER_QA.read(gd, rules);
  return (gd.owner & rules) !== 0;
}
/** WHO HAS HIM, as `updateGuard` would have it: the name, or null when he is his own man */
export function ownerOf(gd: Guard): OwnerRule | 'dead' | 'held' | 'script' | null {
  if (gd.state === 'dead') return 'dead';
  if (gd.held) return 'held';
  const r = rankOf(gd.owner);
  if (r >= SCRIPT_RANK) return rowOf(r).name;
  if (gd.script) return 'script';
  return r ? rowOf(r).name : null;
}
/** the rules of his body stand above `script`, the rest below it */
const SCRIPT_RANK = rankOf(OWN.startle);
/** what the panel says of one man */
function ownerWords(gd: Guard, t: number): string {
  const who = ownerOf(gd);
  if (!who) return '';
  if (who === 'dead' || who === 'held' || who === 'script') return `${gd.callsign} ${who}`;
  const row = rowOf(gd.rank) as { name: string; why: readonly string[] };
  const under = RULES.filter(r => r.name !== who && (gd.owner & OWN[r.name])).map(r => r.name);
  return `${gd.callsign} ${who}: ${row.why[gd.reason] ?? row.why[0]}, ${Math.max(0, t - gd.since).toFixed(1)} s${under.length ? ` (over ${under.join(', ')})` : ''}`;
}
/** THE DEBUGGER'S LINE (ui/bindings.ts): who has each man who is not his own, and why */
export function ownerSummary(men: readonly Guard[], t: number): string {
  const said = men.filter(gd => gd.state !== 'dead').map(gd => ownerWords(gd, t)).filter(Boolean);
  return `who has them ▸ ${said.length ? said.join(' · ') : 'each his own man'}`;
}

// ---------------------------------------------------------------- the check's
/** what a rule's own state says, one line in the rule's file: tools/qa/owner-check.ts holds the record to it at every read. Nothing in the game calls these */
const WITNESSES: { rule: number; name: string; says: (gd: Guard, g: never) => boolean; atRead: boolean }[] = [];
/** `atRead`: the rule's answer turns on what moves between its reads (a man's awareness), so the rule says it just before they come and it is held to its state only there */
export function ownerWitness<G>(name: OwnerRule, says: (gd: Guard, g: G) => boolean, atRead = false): void { WITNESSES.push({ rule: OWN[name], name, says: says as (gd: Guard, g: never) => boolean, atRead }); }
/** every rule the record and its own state disagree about for this man */
export function ownerDisagrees<G>(gd: Guard, g: G, rules = ~0, reading = true): { name: string; record: boolean; rule: boolean }[] {
  const out: { name: string; record: boolean; rule: boolean }[] = [];
  for (const W of WITNESSES) { if (!(W.rule & rules) || (W.atRead && !reading)) continue; const record = (gd.owner & W.rule) !== 0, rule = W.says(gd, g as never); if (record !== rule) out.push({ name: W.name, record, rule }); }
  return out;
}
/** the names of the rules with a witness, and the table's order (the check's) */
export const ownerWitnessed = (): string[] => WITNESSES.map(W => W.name);
export const ownerRanks = (): string[] => RULES.map(r => r.name);
