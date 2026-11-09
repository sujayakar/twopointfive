// HOW MANY ARE INSIDE IS A MYSTERY. The maintainer, 10-23: "we should also remove all defender counts from the
// menus, etc., it should be a mystery!" What a player reads BEFORE a mission, held to that: the words of
// every mission row, start-page pick and difficulty sentence. The rows are the module's (tools/desk.ts): no game is stood.
//   bun run tools/qa/mystery-check.ts [--list]
//   total    no row's place, job or pick line says its own head count: a number beside "enemies", "defenders", "guards", "men" … that IS the row's
//            `defenders`, any such number past five, "against eleven", "eleven hold". A smaller number is a part of the puzzle and stays ("two sentries
//            who watch each other"). The training and tutorial rows are lessons: their numbers are the exercise, listed and not judged
//   asks     no screen reads a row's `defenders` or the tally's roster for itself: nothing under src/ui names `.defenders` or `.roster`
// MYSTERY_BREAK=1 reads the old rows as theirs: 'total' must FAIL. What Hard says is the desk's (rust/game_run), and no check reads it for a number today.
const ROOT = new URL('../..', import.meta.url).pathname.replace(/\/$/, '');
import { readFileSync, readdirSync } from 'node:fs';

const NUM = ['zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'ten', 'eleven', 'twelve', 'thirteen', 'fourteen', 'fifteen', 'sixteen'];
const N = `(${NUM.join('|')}|\\d+)`, FOE = '(?:enemies|enemy|defenders?|guards?|gunmen|gunman|men|sentries|sentry|captors?|hostiles?|shooters?)';
const valueOf = (w: string) => /^\d+$/.test(w) ? Number(w) : NUM.indexOf(w.toLowerCase());
/** every number a sentence puts on the enemy: [the words, the number, said as the whole floor ("against eleven", "eleven hold")] */
export function countsIn(text: string): [string, number, boolean][] {
  const out: [string, number, boolean][] = [];
  for (const m of text.matchAll(new RegExp(`\\b${N}\\s+(?:armed\\s+|more\\s+|other\\s+)?${FOE}\\b`, 'gi'))) out.push([m[0], valueOf(m[1]), false]);
  for (const m of text.matchAll(new RegExp(`\\bagainst\\s+${N}\\b|\\b${N}\\s+(?:hold|holds|holding|of\\s+us|inside|on\\s+shift|on\\s+patrol)\\b`, 'gi'))) out.push([m[0], valueOf(m[1] ?? m[2]), true]);
  return out;
}
/** the old rows, as they read before the counts came out: what MYSTERY_BREAK reads as a row's */
const OLD: [string, number][] = [["It's five against eleven. Three squads hold the block, each in its own rooms.", 11], ['Eleven enemies hold a building laid out as a ring.', 11],
  ['Three enemies hold a break room with four doors.', 3], ['Six enemies, and the clock starts on your first move.', 6], ['The hard one. Eleven hold a ring of rooms.', 11]];

export async function mysteryCheck(): Promise<boolean> {
  const BREAK = process.env.MYSTERY_BREAK === '1', LIST = process.argv.includes('--list');
  console.log(`\n=== mystery: no screen says how many are inside${BREAK ? ' — MYSTERY_BREAK: the old sentences read as rows' : ''} ===`);
  let all = true; const line = (ok: boolean, label: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
  const { missions } = await import(`${ROOT}/tools/desk.ts`) as typeof import('../desk.ts'), list = missions();
  {
    // ---- total
    const total = (e: typeof list[number]): number => e.kind === 'solo' ? 3 : e.defenders;   // (the solo night's three are the level's own)
    const rows: { id: string; text: string; total: number; lesson: boolean }[] = list.map(e => ({ id: e.id, text: [e.place, e.blurb, e.scene].filter(Boolean).join(' '), total: total(e), lesson: e.group === 'tutorial' }));
    for (const e of list) if (e.line) rows.push({ id: `${e.id}'s line on the short list`, text: e.line, total: total(e), lesson: false });
    if (BREAK) for (const [text, total] of OLD) rows.push({ id: 'an old row', text, total, lesson: false });
    const bad: string[] = [], part: string[] = [], lessons: string[] = [];
    for (const r of rows) for (const [words, n, whole] of countsIn(r.text)) (r.lesson ? lessons : whole || n === r.total || n > 5 ? bad : part).push(`${r.id} "${words}"`);
    line(bad.length === 0, "total: no mission row or pick line says its own head count", bad.length ? bad.join(' · ') : `${rows.length} rows read · a part of the puzzle, kept: ${part.join(' · ') || 'none'}`);
    if (LIST) console.log(`    the lessons' own numbers (the exercise, not judged): ${lessons.join(' · ') || 'none'}`);
    // ---- asks
    const ui = readdirSync(`${ROOT}/src/ui`).filter(f => f.endsWith('.ts')), own: string[] = [];
    for (const f of ui) readFileSync(`${ROOT}/src/ui/${f}`, 'utf8').split('\n').forEach((l, i) => { const code = l.replace(/\/\/.*$/, '').replace(/\/\*.*?\*\//g, '');
      if (/\.defenders\b|\.roster\b/.test(code)) own.push(`${f}:${i + 1}`); });
    line(own.length === 0, "asks: no screen reads a row's count or the tally's roster for itself", own.length ? own.join(' · ') : `${ui.length} files under src/ui read`);
  }
  console.log(`=== mystery: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await mysteryCheck()) ? 0 : 1);
