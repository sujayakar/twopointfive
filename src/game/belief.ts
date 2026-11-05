// WHAT A MAN BELIEVES AND WHAT HE COVERS ARE TWO THINGS (BELIEF.on: on in the page, ?belief=0; off in this module and on the frozen stand, TPS_BELIEF=1).
//   Guard.lastKnown is where he believes the enemy to be: only what reached him (his eyes, his ears, a mate's word, a fix he was handed) ever writes it.
//   Guard.covers is the point his duty has his gun on while no man is in his eyes: the room fight's to write (cover() below) for a man it has holding a post.
// Before this rule the two were one field. The room fight wrote the door he watches into lastKnown at the top of every tick and his eyes wrote the man back
// later in the same tick, and everything that read the field in between, or after his eyes had nothing, took a door for the enemy: where a grenade goes, how
// fresh his fix is, the picture he has of his man, the alarm's place. With the rule off cover() writes lastKnown as it always did and `covers` stays null, so
// gunPoint() is lastKnown, eyesFix() asks the old question and the build is the build as it was (a digest does not see the field while it is null).
// docs/agents/enemies.md has the rule's words, stands and measurements; tools/qa/belief-check.ts is its check. No top-level statement here uses another module.
import type { Game, Guard } from './game';
import { v3, type Vec3 } from '../math/vec';

export const BELIEF = {
  on: false,
};

/** the room fight (and the rules that hand a man back to it): his duty has his gun on `pt`, kept by reference, moved in place as his watch moves */
export function cover(gd: Guard, pt: Vec3) {
  if (BELIEF.on) gd.covers = pt; else gd.lastKnown = pt;
}
/** his duty has nothing for his gun now: off the room fight's roster, or his eyes have it */
export function uncover(gd: Guard) { gd.covers = null; }

/** his belief is what his eyes last had (a noise or a word since has moved it off that) */
function fixIsSight(gd: Guard): boolean { return !!gd.lastKnown && v3.distXZ(gd.lastKnown, gd.sightPos) < 0.01; }
/** where his own eyes have the man, for the rules that ask only that of a man on a post (null: a noise, a word, or nothing). Rule off: whatever in the one field
 *  is not `pt` he watches, as those rules always asked it */
export function eyesFix(gd: Guard, pt: Vec3): Vec3 | null {
  const lk = gd.lastKnown; if (!lk) return null;
  return (BELIEF.on ? fixIsSight(gd) : lk !== pt) ? lk : null;
}

/** where his gun goes with no man to shoot at: what his own eyes put in his belief this very tick, else what his duty covers, else what he believes. A man off
 *  his post has no duty, whatever was left in the field */
export function gunPoint(g: Game, gd: Guard): Vec3 | null {
  const lk = gd.lastKnown, cv = gd.covers;
  if (!cv || !gd.hold || !gd.pinned) return lk;
  return lk && g.time - gd.sightT < 1e-6 && fixIsSight(gd) ? lk : cv;
}

/** a report that reaches him outside his ears and eyes (a blast on him, a mate's word on the net) places his belief: what he hears of the same moment, later in
 *  the same tick, does not write over it (hearingCheck, whistle.ts, attention.ts ask wordNow). His eyes still do: they are the newer */
export function word(g: Game, gd: Guard, at: Vec3) { gd.lastKnown = at; gd.wordT = g.time; }
export function wordNow(g: Game, gd: Guard): boolean { return BELIEF.on && gd.wordT === g.time; }
