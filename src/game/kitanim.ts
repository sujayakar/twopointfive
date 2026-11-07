// KIT ANIMATIONS — Sam handles his kit with his hands (the maintainer, 10-10: "we should have better animations for equipping and unequipping kit. like rifle for
// sam goes on his back, pistol in holster, knife, etc. reloads can be more detailed too (and vary based on tactical reload vs. not)" · "the closer than ever
// animation could use some work. sam's hand looks pretty static. and it should be reaching for his knife. one thing chaos theory did well was having sam's
// state vary continuously with his distance to the enemy").
//
// This file is the GAME's half: it decides WHEN, off the game's own state, and hands the animator its inputs. The poses are character.ts (KIT: kitSwap /
// kitLayer, pistolReloadArm, the `keep` variant of reloadArm, closeK in creepLayer / readyLayer); the props are rigProps.ts (KitDraw).
//
//   EQUIP    kitTick() watches what is in Sam's right hand by the game's rules (kitInHand: the pistol, the rifle, a can, the knife of an arm hold, nothing) and
//            on a change starts a swap: the old thing is carried to where it lives (pistol → thigh holster, rifle → over the shoulder onto the sling across his
//            back, can → belt pouch, knife → the sheath on his chest), the hand goes on to the next thing and brings it up. While it runs the drawer shows what
//            the HAND has (kitShow), not what the slot says, and a gun coming up cannot fire until it is up (Player.equipT, the lock-out every fire site already
//            honours — it was 0.45 s for anything involving the rifle and nothing for the pistol; now it is the draw's own length).
//   RELOAD   kitReload() picks the magazine change off the gun's state — EMPTY (dry magazine, nothing chambered: the magazine falls, a fresh one, the slide /
//            handle run), DRY (dry magazine, a round still up: the same without the slide) or TACTICAL (rounds left in the magazine: it is taken out and PUT
//            AWAY, then the fresh one; slower) — and gives the weapon and the animator that change's own beat (KITANIM.beats, scaled to the weapon's row).
//   CLOSE    kitTick() also deals the animator how close the nearest man of the close-in mode is, 0‥1: the carriage deepens and the knife hand travels to the
//            hilt with it, continuously (KIT.close).
//
// WHILE A SWAP RUNS (anim.kitBusy): no reload starts (player.ts reloadInHand), a can not yet in the fist is not thrown by LMB, no checkpoint is written
// (savestate.ts playerBusy), and a gun's light, lit lens and IR laser wait until it is UP (kitGunUp). A gun leaving the hand abandons its magazine change.
// Needs the MOVES' reload layer for the magazine hands (anim.kitHandsOn): without it a reload is the weapon row's, as ever.
//
// SWITCH: KITANIM.on — ON in the page (`?nokitanim` = the build as it was), OFF in the harness's frozen stand (TPS_KITANIM=1 turns it on there): off, nothing in
// this file calls anything and every pose, prop, beat and lock-out is what it was. Sam only (the one Player); nothing under the tour.
// Per-player state lives on the Player (kitWas / kitCan); a fresh Player (a restart, a loaded checkpoint) starts with no swap pending — the kit simply is where
// the game says. No dice, no clocks of its own, nothing read from the renderer.
import type { Game } from './game';
import type { KitItem, ReloadBeat } from './character';
import type { Firearm, WeaponDef } from './weapons';
import type { KitDraw } from './rigProps';
import type { ThrowKind } from './game';
import './close';   // (read inside functions only — the game's one import cycle)
import { activeScenario } from './scenarioRun';   // (the same)
import './rigProps';   // (the same)
import './workstow';   // (the same: the thing in his hand is on its way to where it lives before a two-handed job — that errand is this file's)
import './handwork';   // (the same: while his hands are at work on a thing the work has them)
import { samBigCarry, bigWeighed } from './bigkit';   // (the same)
import { bigOff, type Character } from './character';   // (the same)
import { crossToRust, goneWithTick } from './crossing';

export type ReloadKind = 'empty' | 'dry' | 'tactical';
type BeatRow = { dur: number; out: number; in_: number; rack?: number; keep?: number };
export const KITANIM = {
  /** THE switch (see the head). The three parts can be turned off one by one under it. */
  on: false,
  equip: true, reload: true, close: true,
  /** held things are DRAWN in his hands (rigProps.ts GRIPFIT: the can in the fist, the guns' grips in the palm, the foregrip in the fingers, a magazine across
   *  the palm) and his fingers stand open round a can. Picture only — the muzzle, the bore and the gun light are where they were. Off = every box off the wrist as before. */
  grips: true,
  /** where the rifle lives when it is not in his hands (the maintainer, 10-10: "for "loud builds" … unequipping rifle should be putting it down in a sling, not
   *  over the back, right?"): 0 = by the mission — a squad's loud mission: let down on its sling in FRONT of him (quicker both ways: no trip over the shoulder);
   *  a quiet mission, the plain night: across his BACK, out of the way of his hands — the very test the sneaking suit is worn by (cast.ts sneakSuit, less the
   *  look's own switches) · 1 = always in front · 2 = always on his back. Part of `equip`: with that off (or the switch) it is the back, as the build had it. */
  sling: 0 as 0 | 1 | 2,
  /** the rifle hanging in front rests on a thigh that comes up under it (rigProps SLING_REST): how fast it is pushed up onto one / settles back down, 1/s (an
   *  ease toward what the pose wants — a stride moves that a quarter turn in a few frames, and a hung rifle has weight); 0 = no ease, the pose's own answer */
  slingUp: 18, slingDown: 4,
  /** s before a draw's end at which the gun may already fire (it is up; the support hand is still settling) */
  early: 0.05,
  /** A CLICK MADE WHILE A GUN IS STILL COMING UP IS KEPT: the left button pressed inside the last `clickKeep` s of a kit change's lock-out fires the round on the
   *  frame the gun is up (kitClick — the wall stack's own rule for its rise). 0 = dropped, as the build had it: the click of a drawn pistol did nothing. */
  clickKeep: 0.35,
  /** …and A CLICK MADE WHILE A CAN IS STILL COMING TO THE FIST IS KEPT for the whole fetch ('5, click' in one breath throws as the can arrives — it used to throw
   *  nothing for up to two thirds of a second): the most such a click may wait, s. 0 = dropped, as it was. */
  throwKeep: 1.0,
  /** the most a lock-out may be once no errand is running (= player.ts RIFLE_EQUIP_SECS, the build's own rifle beat; the probes hold the two equal) */
  plainLock: 0.45,
  /** the magazine changes, in seconds on the weapon's clock — written for the pistol's 1.6 s row and the rifle's 2.4 s row and scaled to whatever row the gun in
   *  hand has. out = the old magazine leaves the gun · keep = it is in the dump pouch (tactical only) · in_ = the fresh one is seated · rack = slide / handle */
  beats: {
    pistol: { ref: 1.6, empty: { dur: 1.6, out: 0.25, in_: 0.95, rack: 1.3 } as BeatRow, dry: { dur: 1.45, out: 0.25, in_: 0.95 } as BeatRow, tactical: { dur: 2.0, out: 0.45, keep: 0.95, in_: 1.5 } as BeatRow },
    rifle: { ref: 2.4, empty: { dur: 2.4, out: 0.4, in_: 1.45, rack: 1.95 } as BeatRow, dry: { dur: 1.95, out: 0.4, in_: 1.45 } as BeatRow, tactical: { dur: 2.75, out: 0.55, keep: 1.2, in_: 2.1 } as BeatRow },
  },
  /** m to the close-in mode's man at which closeness is 1 (the knife's reach); it is 0 at the mode's own edge (close.ts CLOSE.modeM) */
  closeNear: 1.1,
};
/** for the probes and the panel: what has been played */
export const KITANIM_STATS = { swaps: 0, draws: { pistol: 0, rifle: 0, can: 0, knife: 0 }, stows: { pistol: 0, rifle: 0, can: 0, knife: 0 }, reloads: { empty: 0, dry: 0, tactical: 0 }, closeFrames: 0, nv: { hand: 0, nod: 0 }, owed: 0 };
export const KITANIM_LOG: { t: number; what: 'swap' | 'reload'; from?: KitItem | null; to?: KitItem | null; kind?: ReloadKind; secs: number }[] = [];

function live(g: Game): boolean { return KITANIM.on && !g.touring; }


/** What the game's own rules put in Sam's right hand this frame (the drawer's rules, combat.ts handProps, said once): the pistol up, the rifle in the hands, a
 *  can of the selected kind, the knife of a man held with the hands empty — or nothing. */
/** a pose has his hands off his own gun: the animator's word, as it stood after its last update — and, the big slot's tool with BIGSLOT.weight on
 *  (bigkit.ts), the word it WILL give this frame, read off the beat's record as it stands now (character.ts bigOff): the kit's draw then starts the frame the
 *  tool lets go of his hands, not one later with the gun shown in them for a frame between */
function handsOff(an: Character['anim']): boolean { return an.bigTool && bigWeighed() ? bigOff(an.bigTool) : an.hideHeldItem; }
export function kitInHand(g: Game): KitItem | null {
  const pl = g.player, an = pl.char.anim;
  if (pl.down) return null;
  if (pl.holding) return pl.holding.variant === 'arm' && !an.holdGunSide && !an.noKnife ? 'knife' : pl.pistolUp ? 'pistol' : null;
  if (handsOff(an)) return null;
  if (samBigCarry(g)) return null;   // (bigkit.ts BIGSLOT.weight: the big slot's tool is asked for — his gun goes to its place by hand before the hand goes for the tool)
  if (pl.pistolUp) return 'pistol';
  if (pl.rifleUp) return 'rifle';
  const k = pl.canisterSel;
  return k && !pl.holstered && !pl.dragging && g.throwCount(k) > 0 ? 'can' : null;
}

/** KIT: the rifle hangs in FRONT of him on its sling when it is not in his hands (KITANIM.sling) — the hand's errands (anim.kitFront) and the drawer
 *  (RigOpts.slingFront / KitDraw.front) both ask here. A fact of the mission, not of the picture: the times of his kit changes hang on it. */
export function kitSlingFront(g: Game): boolean {
  if (!live(g) || !KITANIM.equip) return false;
  if (KITANIM.sling) return KITANIM.sling === 1;
  const k = activeScenario(g); return !!k && k !== 'quiet';
}

/** KIT: how far the rifle hanging in front is swung forward onto his thighs this frame, rad — eased toward what his pose wants (rigProps slingRestWant) on the
 *  game's clock, so asked twice in a frame it answers the same; the state is the Player's (a fresh Player starts on the pose's own answer). undefined = not in front. */
export function kitSlingRest(g: Game): number | undefined { return crossToRust('kitSlingRest', undefined, [g]) as any; }

/** how close a man `d` metres off is, 0 (the close-in mode's edge) … 1 (in the knife's reach) */
export function kitCloseness(d: number): number { return goneWithTick('kitCloseness') as any; }



/** The longest a kit change can hold a gun's fire (the soak's bound on Player.equipT with the switch on). */
export function kitLockMax(KIT: { stow: Record<KitItem, number>; reach: Record<KitItem, number>; draw: Record<KitItem, number>; far: number; swapK: number; front?: { stow: number; reach: number; draw: number } }): number {
  let m = 0;
  for (const fr of KIT.front ? [false, true] : [false]) {   // (the rifle on its back sling, then on the front one: its own three times — character.ts KIT.front)
    const F = KIT.front, t = (T: Record<KitItem, number>, it: KitItem, f: number) => fr && it === 'rifle' ? f : T[it];
    for (const to of ['pistol', 'rifle'] as const) {
      const reach = t(KIT.reach, to, F?.reach ?? 0), draw = t(KIT.draw, to, F?.draw ?? 0); m = Math.max(m, reach + draw);
      for (const from of ['pistol', 'rifle', 'can', 'knife'] as const) m = Math.max(m, (t(KIT.stow, from, F?.stow ?? 0) + reach + (fr ? 0 : KIT.far) + draw) * KIT.swapK);
    }
  }
  return m;
}

/** What the drawer shows in his hands this frame: `v` is what the game's state says (combat.ts handProps computes it as ever); while a swap runs the HAND's
 *  word replaces it — the pistol is in the fist until the fist is at the holster, the rifle in the hands until it is over his shoulder, and so on — and `kit`
 *  carries the in-between for rigProps. Off / idle: `v` untouched, no `kit`. */
export function kitShow(g: Game, v: { pistolInHand: boolean; rifle: 'hand' | 'slung' | null; held: ThrowKind | null; knife: boolean }): KitDraw | undefined { return crossToRust('kitShow@src/game/kitanim.ts', undefined, [g, v]) as any; }

/** The magazine change for the gun in his hands, off its state: which kind, the weapon's beat for it (`row`: Firearm.startReload's) and the animator's
 *  (`beat`: CharacterAnimator.reload's). null = the switch is off / a shell-fed gun: the row's own reload, as ever. */
export function kitReload(g: Game, gun: Firearm, long: boolean): { kind: ReloadKind; row: WeaponDef['reload']; beat: ReloadBeat } | null { return goneWithTick('kitReload') as any; }
/** one line for the panel */
export function kitSummary(): string { const S = KITANIM_STATS, r = S.reloads; return KITANIM.on ? `on · ${S.swaps} swaps · reloads ${r.empty} empty / ${r.dry} dry / ${r.tactical} tactical` : 'off'; }
