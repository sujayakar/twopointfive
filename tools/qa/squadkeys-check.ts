// THE CONTROLS LIST SAYS THE SQUAD'S KEYS AS THE PAGE HAS THEM, AND NAMES NO PLAN MODE (src/ui/options.ts squadRows; game/cmdring.ts `only`, `goals`).
// The maintainer, 10-26: "let's remove plan mode and update the tutorials accordingly". On the page the squad's keys are the command key and the word's;
// Shift with the command key sends a part of the squad to one more thing. Reads the module and the screen's source as text: no stand, no page.
//   bun run tools/qa/squadkeys-check.ts      (also `probes.ts squadkeys`, family ui)
//   as built   with `only` off the rows are handed back untouched
//   keys       with `only` and `goals`: the command key's row says its tap and its hold, a row of Shift with that key follows it, the word's row says
//              what the word does, and the orders menu's row is listed only with a key
//   no part    with `goals` off there is no row for a part of the squad
//   words      nothing the rows say with `goals` on names plan mode; in the screen's source the "Plan mode circles" row and the who row's
//              plan mode words are behind `goals` being off
// SQUADKEYS_BREAK=1 asks for the rows with `goals` off where the page has it on: 'keys' must FAIL.
// The page's half (Options ▸ Controls as drawn, the older card's key list): tools/qa/squadkeys-page.cjs.
const ROOT = new URL('../..', import.meta.url).pathname.replace(/\/$/, '');
import { readFileSync } from 'node:fs';

type Row = [string[], string, string?];
export async function squadKeysCheck(): Promise<boolean> {
  const BREAK = process.env.SQUADKEYS_BREAK === '1';
  console.log(`\n=== squadkeys: the Controls list says the squad's keys as the page has them${BREAK ? ' — SQUADKEYS_BREAK: the rows asked for with goals off' : ''} ===`);
  let all = true; const line = (ok: boolean, label: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
  const { squadRows } = await import(`${ROOT}/src/ui/options.ts`), src = readFileSync(`${ROOT}/src/ui/options.ts`, 'utf8');
  const built: Row[] = [[['X'], 'Orders, on what the cursor is on', 'or use the middle mouse button'], [['B'], 'The usual order for that spot, at once'], [['T'], 'Who takes the next order', 'everyone, red, blue or one teammate'], [['F'], 'Go', 'tells a waiting squad to go'], [['H'], 'With the orders up: the big tool'], [['G'], 'With the orders up: frag and clear']];
  const say = (rows: Row[]) => rows.map(r => `[${r[0].join(' + ')}] ${r[1]}`).join(' · ');
  // ---- as built
  const a = squadRows(built, false, true, false) as Row[];
  line(a === built, 'as built: with the squad on its older keys the rows are handed back untouched', say(a));
  // ---- keys
  const k = squadRows(built, true, !BREAK, false) as Row[], cmd = k.findIndex(r => r[0].length === 1 && r[0][0] === 'B'), part = k[cmd + 1], word = k.find(r => r[0][0] === 'F');
  const withKey = squadRows(built, true, true, true) as Row[];
  line(cmd === 0 && /^Order the squad$/.test(k[0][1]) && /tap it/.test(k[0][2] ?? '') && /hold it a moment for the wheel/.test(k[0][2] ?? '') && !!part && part[0].length === 2 && part[0][1] === 'B' && part[0][0] !== 'Shift' && /part of the squad/.test(part[1]) && /wheel/.test(part[2] ?? '')
    && !!word && /everyone who's waiting/.test(word[1]) && !k.some(r => r[0][0] === 'X') && withKey.some(r => r[0][0] === 'X') && k.some(r => r[0][0] === 'T') && k.some(r => r[0][0] === 'H'),
    "keys: the command key's row says its tap and its hold, Shift with it is the next row, the word's row says what it does, the menu's row needs a key", say(k));
  // ---- no part
  const n = squadRows(built, true, false, false) as Row[];
  line(!n.some(r => r[0].length > 1) && n.length === k.length - (BREAK ? 0 : 1), 'no part: with goals off there is no row for a part of the squad', say(n));
  // ---- words
  const said = (squadRows(built.filter(r => r[0][0] !== 'T'), true, true, false) as Row[]).filter(r => /plan mode/i.test(`${r[1]} ${r[2] ?? ''}`)).map(r => r[1]);
  const circles = /if \(CMDRING\.plan && !CMDRING\.goals\) range\(body, 'Plan mode circles'/.test(src), who = /CMDRING\.goals \? WHO_DEAD : WHO_PLAN/.test(src) && (src.match(/\bWHO_PLAN\b/g) ?? []).length === 2 && !/plan mode/i.test(/WHO_DEAD = '([^']*)'/.exec(src)?.[1] ?? 'plan mode');
  line(said.length === 0 && circles && who, 'words: no row names plan mode with goals on, and the circles row and the who row\'s plan mode words wait on goals being off', `rows naming it: ${said.join(' · ') || 'none'} · the circles row guarded ${circles} · the who row's words guarded ${who}`);
  console.log(`=== squadkeys: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await squadKeysCheck()) ? 0 : 1);
