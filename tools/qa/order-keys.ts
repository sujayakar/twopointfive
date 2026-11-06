// WHAT A MENU'S ROW MAY BE ON, and which row the door tools' fourth is: one copy for the checks that only need that (orderkeys, shutdoor, shotgun-card,
// orders-menu). doortools-probes, pairs-probes and doormenu-probes still branch on DOORTOOLS.outsidePlain themselves, for the row's words and its order as
// well as its id. It imports nothing (the game's own tables are handed in), so a probe file can load it beside its own stood game.

/** the letters an order may be on: its own four, and the two a full page spills onto */
export const LETTERS = ['X', 'T', 'B', 'F', 'G', 'H'];   // (G: the door tools' frag row — the key beside F, where the page's five digits are taken.
// H: the big slot's row on that same page and squad smoke's on the floor's — one key past G and a stretch from W A S D, which is an open call
// with the maintainer, not a walk's to refuse. No walk met it before a team of four was lent a tool as issued — KITDEAL.more, menus.md.)

/** a key of the left hand: one of LETTERS, or a digit from 1 to `digits` (orders.ts ORDER_DIGITS) */
export const leftKey = (k: string, digits: number): boolean => LETTERS.includes(k) || (/^[1-9]$/.test(k) && Number(k) <= digits);

/** the door tools' fourth row: the turn to the threshold's page, or 'clear from outside' itself where the rule has it plain (doortools.ts
 *  DOORTOOLS.outsidePlain). Read when asked, never kept: a rig puts the table back as it stands a game */
export const toolsRow4 = (DOORTOOLS: { outsidePlain?: boolean }): string => DOORTOOLS.outsidePlain ? 'thresholdHold' : 'thr';
