// THE GUN IS PUT AWAY FOR A TWO-HANDED JOB (the maintainer, 10-23: "characters should fully unequip their weapon when planting door charges, etc. -- they can
// reequip after they're done"). A charge went on with the gun still in the man's hands: you set one with your left while the pistol stayed in your right (the
// rifle hung low in both and a hand left it), and a teammate pressed his on with one hand, the carbine in the other, free to fire the whole two seconds.
//
//   YOU      (door charge by hand, wall charge by hand — doortools.ts / wallcharge.ts samTick): what is in your hand is put away first (the pistol to its
//            holster, the rifle onto its sling, a can to its pouch — player.ts setHolstered, carried there by kit handling's own errand where that is on), the
//            job's clock starts once your hands are free of it, BOTH hands go on the charge, and when it is set — or you let go of it — it is brought back up.
//            No round leaves the gun until it is up again: that is the kit's own draw (Player.equipT), nothing added here. Hands that were already empty stay
//            empty. Something that wants your hands empty and took you off the job (a man in your arm, a lock, a wall you are pressed to) leaves it away.
//   THE TEAM (the 'place' stage of a door charge and of a wall charge on order): the setter's gun leaves his hands as he comes onto the job — a long gun hangs
//            down his front on its sling, a sidearm goes to his thigh (combat.ts guardKit's `hang`, as for the hammer) — both hands go on the charge `mateDown`
//            later, and it is back in his hands `mateUp` after the job ends. NO ROUND LEAVES IT from the moment it is let go until it is back up (gunslots.ts
//            sideLocked, the one lock every trigger a teammate has asks; his fire clock is held as well). A man the fight has (his order yielded to it, he is
//            blind or doubled over, the room fight has his gun on a man) DROPS the job: nothing is spent, the gun comes up, and he starts over when the fight
//            lets him go — or after `fightCap` of it at a stretch, with it still on him.
//
// WHAT IT COSTS: the plant is longer by the time the gun takes to put away, and the first round after one waits for the draw — a man planting cannot shoot.
// NOT HERE (other two-handed jobs, their owners'): a timed charge on its bench, the drive in its rack, the optical wand, a lock, a thing a teammate works.
//
// SWITCH: WORKSTOW.on — ON in the page (`?noworkstow` off), OFF in the module and on the frozen stand (`TPS_WORKSTOW=1`): off — or with the hands that
// do the work off (HANDWORK) — nothing here is called and every plant is what it was. State: Player.workStow, Guard.workStow (on the men: a rollback snapshot holds them; a checkpoint does not — no save is written while a
// charge goes on, his gun is away for one or a kit errand runs (savestate.ts playerBusy), and a teammate loaded mid-job has his gun in his hands). No dice, no clock of its own.
import type { Game, Guard } from './game';
import './handwork';   // (asked inside the functions below only)
import { RUST_PASS, RUST_SURFACE } from './crossing';

export const WORKSTOW = {
  /** THE switch (see the head) */
  on: false,
  /** a teammate: s from his gun let go of to both hands on the job · s from the job's end to the gun back up (no round inside either) · the share of `mateUp`
   *  after which the gun is drawn in his hands again */
  mateDown: 0.3, mateUp: 0.45, show: 0.6,
  /** you: s at most the job's clock waits for your hand to put the thing away (kit handling's errand; an errand something else took the arm from waits no longer) ·
   *  s at least before a round leaves what comes back into your hand (kit handling's own draw is longer and is the lock where it is on; the rifle's own beat too) */
  downMax: 1.6, samUp: 0.3,
  /** a teammate the fight has: s at most, at a stretch, his charge waits on it — then he sets it anyway (door tools' own cap for a lock under fire: kickCap + 3) */
  fightCap: 8,
};
/** QA: how often each happened (the check reads them; nothing else does) */
export const WORKSTOW_STATS = { samDown: 0, samUp: 0, samKept: 0, mateDown: 0, mateUp: 0, mateFired: 0 };


/** combat.ts guardProps: his gun is drawn out of his hands this frame (hung on its sling, or on his thigh) */
export const stowHang = (g: Game, gd: Guard): boolean => { const s = gd.workStow; return !!s && gd.state !== 'dead' && (s.upT < 0 || s.upT - g.time > WORKSTOW.mateUp * (1 - WORKSTOW.show)); };
/** combat.ts fireWeapon, the net under every other fire site: a round that leaves a gun this module has out of his hands puts it in them at once */
export function stowFired(gd: Guard) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('stowFired', undefined, [gd]); if (__r !== RUST_PASS) return __r as any; } } gd.workStow = null; gd.char.anim.stowed = false; WORKSTOW_STATS.mateFired++; }
