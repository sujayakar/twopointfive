// THE ROWS OF HOST/FINDME.TS (Options > Game): finding your own man on the screen, and how the camera answers the hand. What a page draws for them is
// ui/findring.ts's and draw/rims.ts's.
import { camCap } from '../host/keymap';
import { FINDME, setFindMe, resetFindMe } from '../host/findme';

/** the screen's own row makers handed in, so the rows look and key as every other row there */
export interface RowKit {
  sec(parent: HTMLElement, text: string): unknown;
  flip(parent: HTMLElement, label: string, get: () => boolean, set: (v: boolean) => void, describe: string, words?: [string, string]): void;
  seg<T extends string>(parent: HTMLElement, label: string, opts: { v: T; label: string; line?: string }[], get: () => T, set: (v: T) => void, describe: () => string): void;
  range(parent: HTMLElement, label: string, min: number, max: number, step: number, get: () => number, set: (v: number) => void, fmt: (v: number) => string, describe: string): void;
}
const TURNS: { v: 'as' | 'rev'; label: string }[] = [{ v: 'as', label: 'Normal' }, { v: 'rev', label: 'Reversed' }];
/** the rows under their headings; returns what Reset does for them */
export function findMeRows(K: RowKit, body: HTMLElement): () => void {
  K.sec(body, 'Finding yourself · trying out');
  K.flip(body, 'Ring at your feet', () => FINDME.ring, v => setFindMe('ring', v), 'Keep the gold ring from the orders menu at your feet all the time.', ['On', 'Off']);
  K.flip(body, 'Outline in the dark', () => FINDME.outline, v => setFindMe('outline', v), 'Draw a rim around anyone on your team, including you, who\'s standing in the dark.', ['On', 'Off']);
  K.sec(body, 'Camera · trying out');
  K.range(body, 'Aim pan', 0, 100, 10, () => Math.round(FINDME.pan * 100), v => setFindMe('pan', v / 100), v => !v ? 'None' : `${v.toFixed(0)} %`, 'How far the view leans toward where you aim. At none you stay in the middle of the screen.');
  K.seg(body, 'Turn direction', TURNS, () => FINDME.turn < 0 ? 'rev' : 'as', v => setFindMe('turn', v === 'rev' ? -1 : 1), () => `Which way the view turns when you hold ${camCap()} and move the mouse.`);
  return resetFindMe;
}
