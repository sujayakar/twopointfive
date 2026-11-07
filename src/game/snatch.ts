// THE SNATCH: one unarmed man the team has to take ALIVE — an overlay on a level's ROOM CLEAR, switched on by the block `scenarios.snatch` in the level file
// (scene/scenarios.ts SnatchData; levels/snatch.json is the level built for it). A level without the block returns on the first line of every entry point:
// the game as it was.
//
//   WHO HE IS      a civilian of the usual make (civilians.ts spawnTarget: on Game.civilians, so his eyes and ears, his run, the door he pushes, the round that
//                  finds him and Sam's own grab are the civilians' — nothing of that is forked here). He is put on the floor at every staging of the room clear.
//   QUIET          he keeps to his marks. Gunfire or a charge in his ears (civilians.ts hear) frightens him as it does anybody, and he runs for the nearest of
//                  the block's WAYS OUT by walking (civSpots hands the civilians' own flee those). A teammate or Sam he SEES frightens him the same.
//   WHAT HE SEES  one model with everybody else's eyes: a man of the team is SEEN by him only through the civilians' own eye (civilians.ts civilianSees: its
//                  cone and range, the guards' sight gain on the light that falls on that man, over `seeRate`). A man in the dark on his way out is not
//                  seen: he runs into him. What the TEAM has of him (the challenge, the tie, 'nobody on him') is a plain line to him inside `seeM`.
//   HE GIVES UP    (hands on his head, where he stands — the civilians' complied cower, with no end to it) to a teammate or Sam within `giveUpM` with a line
//                  to him, from any side; and, running, to one who stands BETWEEN him and the way out he is running at within `haltM` (ships equal to
//                  `giveUpM`: he stops only for a man he all but runs into; raise it for a tamer man — at 9 m the covered play won 24 of 24). One who is
//                  between him and it but farther than that only TURNS him: that way out is struck off and he runs for another, or goes to ground.
//   IN HAND        hands up with one of the team within `cuffM` and a line for `cuffSecs` = TIED (the wrists-behind pose; for good). Hands up and nobody in
//                  sight for `boltSecs` = he bolts again. In Sam's arm, or put out cold by Sam's hands, is in hand too.
//   THE END        WON = every defender down (the room clear's own 'taken') with him alive and in hand. LOST = he reached a way out, or a round killed him
//                  (anybody's: `killer` says whose, for the tally). The loss fails the mission the way a civilian's death can (the down card); the room
//                  clear's own 'wiped' is Sam's, as ever.
//
// Seams: game.ts (the field; `tick` right after updateMission), the Rust game's `mission::objective` (his line stands over the room clear's while he is
// loose), savestate.ts (the slice — a checkpoint is refused with a squad on the floor, so today nothing reaches it), knobs.ts, civilians.ts (civSpots, the
// tied pose, civilianSees, no 'control will ring' call and no knockout row for him), combat.ts (whose round found him), roomclear.ts (its 'structure
// taken' line says he is still loose). The enemies' decision code is not touched: the defenders do not know he
// exists, and no teammate holds his fire for him — a round through him is the team's to avoid by where it stands.
// The team = Sam and every living operative unit; with a second player it is each player's character, asked the same way (teamOf below is the one place).
import type { Game } from './game';
import type { Guard } from './game';
import type { SnatchData } from '../scene/scenarios';
import { v3, type Vec3 } from '../math/vec';
import { spawnTarget, civilianTie } from './civilians';
import './mission';
import { SNATCH_LINES, snatchLines } from './dialogue';
import { SnatchOut, type SnatchOutSnap } from './snatchout';
import { RUST_PASS, RUST_SURFACE } from './crossing';

export const SNATCH = {
  on: true,
  /** he gives up to one of the team this near with a line to him, whichever way he is going */
  giveUpM: 3.5,
  /** running at a way out with one of the team between him and it, in his sight, this near: he gives up; farther, he turns for another way */
  haltM: 3.5,
  /** how far the TEAM's hold on him reaches: one of the team within this with a line to him covers him (the challenge, the tie, 'nobody on him') */
  seeM: 14,
  /** what HE makes out is the civilians' own eye on the light that falls on the man (civilians.ts civilianSees: cone, range, the guards' sight gain); this is the
   *  gain at which a man of the team counts as seen — the guards' 'it moves his fix' bar. A man stood in the dark on his way out is not seen until he runs into him */
  seeRate: 0.05,
  /** hands up: one of the team this near with a line for `cuffSecs` and he is tied */
  cuffM: 4.5, cuffSecs: 3,
  /** hands up, not tied, and nobody of the team in sight for this long: he runs again */
  boltSecs: 6,
};
export type SnatchEnd = 'won' | 'lost';
export interface SnatchSnap { handsUp: boolean; tied: boolean; nearT: number; aloneT: number; end: SnatchEnd | null; why: string; seen: Vec3 | null; fear: Vec3 | null; lost: 'gone' | 'dead' | null; killer: { side: string; who: string } | null; times: { ran: number; up: number; tied: number; end: number }; stats: { turned: number; bolted: number }; lastExit: string | null; at: Vec3; yaw: number; out?: SnatchOutSnap }

export class Snatch {
  /** the man (null: no block, or not staged) */
  man: Guard | null = null;
  end: SnatchEnd | null = null; why = '';
  /** how it was lost: 'gone' | 'dead'; whose round, when dead */
  lost: 'gone' | 'dead' | null = null; killer: { side: string; who: string } | null = null;
  handsUp = false; tied = false;
  /** game time of: his first fright · his hands going up (the first time) · the tie · the end */
  times = { ran: -1, up: -1, tied: -1, end: -1 };
  /** ways out he was turned from · times he bolted with his hands up */
  stats = { turned: 0, bolted: 0 };
  log: string[] = [];
  private run: object | null = null;
  private nearT = 0; private aloneT = 0; private seen: Vec3 | null = null; private fear: Vec3 | null = null; private saidOut = false;
  /** how it ends while SNATCHOUT is up and the block names a van (snatchout.ts): he is led to it, and in it the mission is won */
  out: SnatchOut;
  constructor(private g: Game) { this.out = new SnatchOut(g, this); }

  /** this level's block while the overlay's switch is up (snatchout.ts reads the van off it) */
  get block(): SnatchData | null { return this.data; }
  /** the mission still hangs on him: loose — or, where he is to be driven off (snatchout.ts), not yet in the van (roomclear.ts: the house taken is not the end) */
  get owes(): boolean { return !this.end && (this.out.ruled || !this.inHand); }
  private get data(): SnatchData | null { return SNATCH.on ? this.g.level.scenarios?.snatch ?? null : null; }
  get active(): boolean { return !!this.man && !!this.data && !!this.g.roomClear?.active; }
  isMan(gd: Guard): boolean { return gd === this.man; }
  /** alive and in the team's hands: tied, in Sam's arm, or out cold by Sam's hands */
  get inHand(): boolean { const m = this.man; return !!m && (m.state === 'dead' ? m.downKind !== 'shot' : this.tied || !!m.held); }
  /** civilians.ts civSay: what HE says in place of a civilian's line (dialogue.ts SNATCH_LINES) — by what he is doing; null = nothing (calm, he is silent: the
   *  glimpse, the shrug and the settling are the office worker's). A line that is already his passes as it is. No die: the pick is off the clock. */
  voice(gd: Guard, text: string): string | null {
    if (snatchLines().includes(text)) return text;
    const L = SNATCH_LINES, s = gd.civ!.state;
    const pool = this.tied ? L.tied : this.handsUp ? L.handsUp : s === 'cower' ? L.cornered : s === 'alarmed' || s === 'flee' ? L.running : null;
    return pool ? pool[Math.floor(this.g.time * 3 + text.length) % pool.length] : null;
  }
  /** combat.ts: a round has found him (before he goes down) — whose, for the tally */
  roundOn(gd: Guard, shooter: Guard | null) { if (gd === this.man) this.killer = shooter ? { side: shooter.team, who: shooter.callsign } : { side: 'player', who: 'Player' }; }

  /** notes made this run (the log itself is cut to its last rows) */
  notes = 0;
  note(what: string) { this.notes++; this.log.push(`${this.g.time.toFixed(1)} ${what}`); if (this.log.length > 60) this.log.splice(0, 20); }

  private stage(N: SnatchData) {
    const g = this.g;
    if (this.man) g.civilians = g.civilians.filter(c => c !== this.man);
    this.run = g.mission; this.end = null; this.why = ''; this.lost = null; this.killer = null; this.handsUp = false; this.tied = false;
    this.times = { ran: -1, up: -1, tied: -1, end: -1 }; this.stats = { turned: 0, bolted: 0 }; this.nearT = 0; this.aloneT = 0; this.seen = null; this.fear = null; this.saidOut = false; this.lastExit = null; this.log.length = 0; this.notes = 0; this.out.reset();
    this.man = spawnTarget(g, N.man);
    this.note(`${N.man.name} staged in his room`);
  }

  private lastExit: string | null = null;


  summary(): string {
    const gd = this.man; if (!gd || !this.data) return 'snatch: none on this level';
    const st = this.end ? `${this.end}${this.why ? ` (${this.why})` : ''}` : this.tied ? 'tied' : this.handsUp ? 'hands up' : gd.civ!.state;
    return `snatch: ${gd.civ!.name} · ${st} · turned ${this.stats.turned} · bolted ${this.stats.bolted}`;
  }
  snapshot(): SnatchSnap | undefined {
    const m = this.man; if (!this.active || !m) return undefined;
    return { handsUp: this.handsUp, tied: this.tied, nearT: this.nearT, aloneT: this.aloneT, end: this.end, why: this.why, seen: this.seen ? v3.copy(this.seen) : null, fear: this.fear ? v3.copy(this.fear) : null,
      lost: this.lost, killer: this.killer ? { ...this.killer } : null, times: { ...this.times }, stats: { ...this.stats }, lastExit: this.lastExit, at: v3.copy(m.char.pos), yaw: m.char.bodyYaw, ...(this.out.ruled ? { out: this.out.snapshot() } : {}) };
  }
  /** savestate.ts, after the civilians are back. TODAY NOTHING REACHES THIS: a checkpoint is refused with a squad on the floor, and a snatch is never staged without one. Written
   *  for the day that changes: a load is a restart, and the restart's room clear has not staged him yet, so the civilians' own slice found nobody to put back — he is staged here,
   *  stood where the file had him, and the overlay's flags (and the pose they mean) are put back. What is NOT carried: his fright mid-run (he comes back as his flags say: at
   *  work, hands up, or tied) and a man out cold (he comes back standing). */
  restore(s: SnatchSnap | undefined) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('Snatch.restore', this, [s]); if (__r !== RUST_PASS) return __r as any; } }
    const N = this.data; if (!s || !N) return;
    if (!this.man || !this.g.civilians.includes(this.man)) { this.stage(N); const c = this.man!.char; c.pos[0] = s.at[0]; c.pos[2] = s.at[2]; c.bodyYaw = s.yaw; }
    this.run = this.g.mission;
    this.handsUp = s.handsUp; this.tied = s.tied; this.nearT = s.nearT; this.aloneT = s.aloneT; this.end = s.end; this.why = s.why; this.seen = s.seen ? v3.copy(s.seen) : null; this.fear = s.fear ? v3.copy(s.fear) : null;
    this.lost = s.lost ?? null; this.killer = s.killer ? { ...s.killer } : null; if (s.times) this.times = { ...s.times }; if (s.stats) this.stats = { ...s.stats }; this.lastExit = s.lastExit ?? null;
    if (this.tied || this.handsUp) civilianTie(this.g, this.man!);   // (the complied cower with no end to it; the wrists-behind pose is asked of isTied)
    this.out.restore(s.out);
  }
}

/** the block against the built floor: he and his ways out stand on floor a man can walk, and every way out can be walked to from where he keeps */
export function checkSnatch(g: Game): string[] { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('checkSnatch', undefined, [g]); if (__r !== RUST_PASS) return __r as any; } }
  const N = g.level.scenarios?.snatch; if (!N) return [];
  const out: string[] = [], nav = g.col.nav;
  const pts: [string, Vec3][] = [['man.pos', N.man.pos], ...(N.man.points ?? []).map((p, i): [string, Vec3] => [`man.points[${i}]`, p]), ...N.exits.map((e, i): [string, Vec3] => [`exits[${i}] ${e.name}`, e.pos])];
  for (const [w, p] of pts) if (nav.isBlocked(p[0], p[2])) out.push(`scenarios.snatch.${w}: (${p[0]}, ${p[2]}) is not floor a man can stand on`);
  const walk = N.exits.map(e => { const w = nav.findPath(N.man.pos, e.pos); if (!w?.length) { out.push(`scenarios.snatch: no way on foot from where he keeps to ${e.name}`); return Infinity; } let l = 0, q: Vec3 = N.man.pos; for (const p of w) { l += v3.distXZ(q, p); q = p; } return l; });
  if (N.van) { if (nav.isBlocked(N.van.pos[0], N.van.pos[2])) out.push(`scenarios.snatch.van.pos: (${N.van.pos[0]}, ${N.van.pos[2]}) is not floor a man can stand on`); else if (!nav.findPath(N.man.pos, N.van.pos)?.length) out.push(`scenarios.snatch: no way on foot from where he keeps to ${N.van.name}`); }
  if (walk.some(l => l < walk[0] - 0.5)) out.push(`scenarios.snatch.exits[0] (${N.exits[0].name}) is not his nearest way out on foot: the objective's line and the page name the first as the one he runs for`);
  return out;
}
