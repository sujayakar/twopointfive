// THE DESK, as a page has it (rust/game_start/src/host/desk.rs): what the cards before a game show and press. The page holds no rule of a card and no card: it keeps
// the text a press says is kept, and asks again.
import { Asks } from '../draw/asks';
import type { Face } from '../draw/face';
import type { Pressed } from './read';

export const KIT_KEPT = 'twopointsix.kit';

export class Desk extends Asks {
  /** `keeps`: told a text this machine keeps after a press that was taken, by the name it is kept under (empty: kept no longer) */
  constructor(private readonly face: Face, private readonly keeps: (key: string, text: string) => void) { super(asked => face.deskAsk(asked)); }
  /** what this machine keeps and what the address says, laid as a start lays them */
  machine(kept: string, address: string): void { this.face.deskMachine(kept, address); }
  protected override told(p: Pressed): Pressed { if (p.ok) for (const k of p.kept) this.keeps(k.key, k.text); return p; }
}

let standing: Desk | null = null;
/** the page's one desk */
export function desk(): Desk { if (!standing) throw new Error('the desk is asked before the page has set it'); return standing; }
export function setDesk(d: Desk): void { standing = d; }
/** a desk stands */
export function deskStands(): boolean { return standing !== null; }
