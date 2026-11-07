// SAM WATCHES THE MAN WHO MATTERS, AND GIVES A MAN ROOM AS HE PASSES HIM (the maintainer, 10-23: "another thing CT did really well is having sam look at
// enemies! curious how that might fit with our control scheme" · "i'm also curious if we can make the experience of 'slithering' through guards also feel
// just as amazing as approaching them for a grab").
//
// HOW IT FITS THE CONTROLS: the cursor is still where he aims and which way his body faces; nothing here moves either, and no key is added. It is his HEAD
// that goes to a man — and the man it goes to is picked by the game, so the head is something to READ: who is nearest, and above all who is beginning to
// notice him (that man is looked at before a nearer one who is not). With a gun up and aimed the head is the gun's, as it was.
//
//   SAMLOOK.on    THE LOOK. Each frame one man is his (`samLookMan`): the man a held click is creeping up on (stalk.ts), else — of the men the fog shows, inside
//                 `nearM`, no more than `backDeg`° round from the way he aims (the cursor's side, which his chest turns to), with nothing solid between the two heads — the one with the lowest
//                 score: his distance, less `noticeM` for a man who is onto him or has him in view with his notice risen (so a watcher a few metres further
//                 off wins), less `swapM` for the man he is already watching (two men at the same distance do not trade him back and forth), and the man
//                 already his is kept out to `keepDeg`°. Nobody while he is down, sprinting, has a man in his arm or in a takedown, kicks, picks, works a
//                 wand or a keypad, has his back on a wall, a body in tow or a hand on a door or a breaker, or has a gun in the aim. The head's strength is whole inside `fullM` and nothing at `nearM`.
//   SAMLOOK.slip  THE SLIP. Crouched, hands empty, off a wall: the nearest such man who is not onto him, at any bearing, inside `slipFrom` m — he tips away
//                 from him, sinks, and the arm on that side goes out low toward him (character.ts WATCH), whole by `slipBy` m — over whatever the close-in's
//                 carriage has him in (close.ts). Behind a man's back it lets go (`backCos`: the cosine of Sam's bearing off the man's nose where it is
//                 gone, and where it is whole): there the creep's own body has him (stalk.ts). The slip is the pass, the creep the back going by.
// Both are LOOKS, and no rule reads either back; his head and chest joints move with them by a few centimetres, and those are what the eyes that look for
// him read (character.ts bones), so a run with them on is not the same run to the digit. Both are off here and on the frozen stand (TPS_SAMLOOK=1,
// TPS_SAMSLIP=1); the page turns them on (?samlook=0, ?slip=0). The pick is exported for a camera that wants the same man.
import type { Game, Guard } from './game';
import { SideMap } from '../math/side';
import { closeSubject } from './close';
import './stalk';
import { crossToRust } from './crossing';

export const SAMLOOK = {
  on: false, nearM: 8, fullM: 5.5, backDeg: 120, keepDeg: 140, noticeM: 3, swapM: 0.75,
  slip: false, slipFrom: 2.6, slipBy: 1.2, backCos: [-0.45, -0.1] as [number, number],
};
interface St { on: Guard | null }
const STATE = new SideMap<Game, St>('samlook.STATE');
const wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));

/** metres to a man his head could go to (Infinity: not one) — `deg` = how far round from his body's facing, 180 = anywhere */
function reach(g: Game, gd: Guard, maxM: number, deg: number): number {
  if (!closeSubject(g, gd)) return Infinity;
  const c = g.player.char, p = c.pos, q = gd.char.pos, dx = q[0] - p[0], dz = q[2] - p[2], d = Math.hypot(dx, dz);
  if (d > maxM || d < 0.15) return Infinity;
  if (deg < 180 && Math.abs(wrap(Math.atan2(dx, dz) - c.aimYaw)) > deg * Math.PI / 180) return Infinity;   // (his aim's line, which his chest and head turn to: with the cursor off to one side that, not his feet's, is where his head starts from)
  if (g.col.solidBlocked([p[0], p[1] + 1.2, p[2]], [q[0], q[1] + 1.4, q[2]])) return Infinity;
  return d;
}
/** the man his head is on (null: nobody) — as of this frame's samLookStep */
export function samLookMan(g: Game): Guard | null { return SAMLOOK.on ? STATE.get(g)?.on ?? null : null; }
/** THE SLIP's man: crouched with empty hands, the man who asks the most room of him (the nearest off a man's back), and how much (0‥1) */
export function samSlipMan(g: Game): { gd: Guard; k: number } | null { return crossToRust('samSlipMan', undefined, [g]) as any; }
