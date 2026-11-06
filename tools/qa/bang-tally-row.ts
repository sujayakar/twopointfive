// THE DOOR TALLY'S ROW — what bang-tally.ts writes of one staged entry, and the rules that read it. No game is stood here: bang-tally-diff.ts and its
// self test read rows without one, and a tool that counts a tally's rows can ask `counted` and not read the note's words.
export type Arm = 'open' | 'bang' | 'hand';
/** `at` on a spec = a staging to stand the entry AS (another tree's `Result.at` for it: bang-tally-diff.ts), taken where every spot of it is free floor here */
export interface Spec { scn: string; door: string; room: string; arm: Arm; seed: number; at?: string }
/** `still`: the room's own men alive and STILL IN IT at the end with the team alive — walked past, not gone off to another room */
/** `at` = the staging as one word: where the team and Sam were stood (Sam last), and how many others were taken off the floor before a sound. Two runs of an
 *  entry with another `at` are not the same entry: bang-tally-diff.ts says so of a door before anyone blames the walk to it. */
export interface Result extends Spec { lost: number; team: number; down: number; of: number; taken: boolean; held: boolean; still: number; after: number; secs: number; bangs: number; pre: number; outside: number; note: string; at: string;
  /** what became of the entry, as a word a reader can hold a tally to (the note is for people): played · refused (the game refused the order: the team is left
   *  outside a hot room, and that is counted as it falls out) · no-floor (nowhere to stand the team) · nobody (no defender dealt to the room, or no such entry) · threw */
  outcome: 'played' | 'refused' | 'no-floor' | 'nobody' | 'threw';
  /** the game's own reason when it refused the entry's order, '' when it took it. A field of its own and never in the note: other tools count rows by
   *  whether they have a note, and a refusal was always a row with none */
  refused: string;
  /** men of the team down before the order was asked (the hot room's men come round to the door while the team waits): the note says it for people, this
   *  says it for a reader. An entry with any is a fight already begun, and its refusal (`no squad`, when it is all of them) is that fight's doing */
  lostBefore: number }
/** the game refused the entry's order: said in `outcome` and `refused`, and the note is left as it was */
export function refuse(out: Result, why: string): void { out.refused = why; out.outcome = 'refused'; }
/** a row the tally's own table counts: an entry that was fought with the team whole at the order (nobody to fight, nowhere to stand, a run that threw
 *  and men lost before the order are set aside, as they always were when this was read off the note) */
export const counted = (r: Result): boolean => (r.outcome === 'played' || r.outcome === 'refused') && !r.lostBefore;
