// WHAT A CARD OF A RUN IS DRAWN FROM (ui/debrief.ts), whichever page holds the run: how it ended, what this machine kept of it, the debrief's own record, and the floor its map is of. Nothing here imports the game.
import type { DebriefView } from './debriefview';

/** m:ss */
export function clock(s: number): string { const n = Math.max(0, Math.floor(s)); return `${Math.floor(n / 60)}:${String(n % 60).padStart(2, '0')}`; }

/** the way a run was won, read off the run */
export type Approach = 'doors' | 'walls' | 'dark' | 'quiet';
export type ManState = 'up' | 'out' | 'dead';
export type ManTone = 'you' | 'purple' | 'red' | 'blue' | null;

/** one man of the team at the run's end. Times are seconds from the try's start; `path` is x, z, t triples sampled as he walked. */
export interface VerdictMan {
  id: number; name: string; tone: ManTone; human: boolean;
  state: ManState; downAt: number; downRoom: string; downBy: string; downPos: [number, number] | null;
  drops: number[]; path: number[];
}
export interface RunVerdict {
  kind: 'nightbreach' | 'roomclear' | 'rescue' | 'quiet' | null;
  won: boolean; word: string; why: string;
  /** seconds from the try's start to the end */
  secs: number;
  approach: Approach | null;
  /** a cheat was on during the try: nothing of it is kept */
  void: boolean;
  /** nobody of the team was lost */
  clean: boolean;
  men: VerdictMan[]; lost: number; of: number;
  /** the defenders: how many went down of how many, and where (x, z pairs) */
  foes: { down: number; of: number; fell: number[] };
  hostages: { alive: number; of: number } | null;
  /** where the team started (the map's mark) */
  start: [number, number];
}
/** the page's note to the card: which try of the streak this was (0 = not a counted try), the ways won on this mission so far */
export interface KeptNote { streak: number; fresh: boolean; gold: boolean; ways: Partial<Record<Approach, { first: boolean; clean: boolean }>>; /** a Training lesson's run: nothing is kept of it and the card grades nothing — no ways, no tries */ lesson?: boolean }

/** the verdict as one line (the copy button, a console read-out) */
export function verdictLine(R: RunVerdict, kept: KeptNote | null = null): string {
  const parts = [`${R.word} ${clock(R.secs)}`];
  if (R.approach && !kept?.lesson) parts.push(R.approach);
  if (R.of > 1) parts.push(`${R.of - R.lost} of ${R.of} up`);
  if (R.foes.of) parts.push(`${R.foes.down} of ${R.foes.of} defenders down`);
  if (R.hostages) parts.push(`${R.hostages.alive} of ${R.hostages.of} hostages alive`);
  if (kept?.streak) parts.push(`try ${kept.streak}`);
  if (R.void) parts.push('cheats on: not kept');
  return parts.join(' · ');
}

/** a mark of the map: where he came in, what he came for, the ways out */
export interface RunMark { glyph: string; cls: string; x: number; z: number; title: string }
export interface RunEnd {
  view: DebriefView;
  verdict: RunVerdict | null; kept: KeptNote | null;
  /** this page's own man is down now */
  down: boolean;
  /** the course's par, where the level has one and the run was timed against it: its medal, its line, and the line with how it stands on this machine */
  par: { medal: string; line: string; shown: string } | null;
  /** what the thing to take is in, as the mission calls it ('': a fight, with no errand) */
  holder: string;
  marks: RunMark[];
  /** the walked floor, a cell at a time: does nothing walk there, is it indoors */
  floor: { w: number; h: number; cell: number; blocked(x: number, z: number): boolean; indoors(x: number, z: number): boolean };
  /** the recap's floor */
  map(): { boxes: number[] };
  /** what this page calls the man with this id: a player's picked name, or `was` */
  name(id: number, was: string): string;
  /** a line that names the kit, in the kit's words as issued */
  kit(text: string): string;
}
