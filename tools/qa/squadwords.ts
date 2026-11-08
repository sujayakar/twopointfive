// WHAT A CHECK READS OF THE SQUAD'S WORDS: the game's own record (`game.view.squad`, rust/game_rules/src/orders/hud.rs), by a man's object, and the open menu's rows
// as the order table has them. Nothing here works out a word. `doing` ends on what the man covers (' · on the rear'), so a line about his order asks how it starts.
const keyName = (code: string): string => code.replace(/^Key|^Digit/, '');
/** the listed menu's key is gone from the game: a check that wants a menu up on what the cursor is on has it put up by the menu's own call, and a second time down */
export function menuKey(O: any): void { if (O.menu) { O.close(); return; } O.open(O.hover() ?? undefined); }
/** one man's line */
export function mateOf(g: any, gd: any): any { return g.view.squad?.mates.find((m: any) => m.man === gd.char.id) ?? null; }
/** what he is doing and what he covers, as the HUD is told it ('' = no squad answers) */
export function doing(g: any, gd: any): string { return mateOf(g, gd)?.doing ?? ''; }
/** the squad's sneak in his line (null: nothing said). It is there only for a man with no post of his own (a follower), under pairUp('word') and while the sneak is live, as `squad_orders_status_of` has it */
export function sneakWordOf(g: any, gd: any): string | null { return doing(g, gd).match(/clear of the door|waiting on eyes|waiting on the light|crossing|backing off/)?.[0] ?? null; }
/** his line is this word, whatever he covers: the tail ' · on …' left out of the comparison and nothing else */
export function sameWord(line: string, word: string): boolean { return line === word || line.startsWith(`${word} · on `); }
/** the notes' words, keys left off */
export function notesOf(g: any): string[] { return (g.view.squad?.notes ?? []).map((n: any) => n.text as string); }
/** who the nearest of the wall charges would catch beyond its wall, as the note says it */
export function behindWall(g: any): string[] { for (const t of notesOf(g)) { const m = t.match(/ · BEHIND THE WALL: (.*)$/); if (m) return m[1].split(', '); } return []; }
/** the corner's notes */
export function cornerOf(g: any): { key: string; text: string }[] { return g.view.corner; }
/** the squad's block, a line a man, the notes and the quick key's line, under its title: what the page lays out (ui/overlay.ts), as one row */
export function blockOf(O: any): any[] {
  const V = O.g.view.squad; if (!V) return [];
  const lk = (act: string) => !!O.g.rails?.locks(act), quick = O.quickLine();
  return [{ kind: 'objective', tone: O.toneOf(O.recipients()), label: lk('who') ? 'squad' : `squad · ${V.posture ? `${V.posture} · ` : ''}orders to ${V.who}`, inReach: true, off: false,
    opts: [...V.mates.map((m: any) => ({ key: '', text: `${V.picked ? (m.addressed ? '▸ ' : '· ') : ''}${m.name} · ${m.doing}`, tone: m.pair ?? undefined, gone: m.down })), ...(lk('menu') ? [] : V.notes), ...(quick ? [quick] : [])] }];
}
/** The open menu's rows off the order table, each with its key and words (null: no menu): a check's way to read what a ring's wedges and digits say; no check of the rows' own keys against this */
export function listedOf(O: any): any {
  const M = O.menu; if (!M) return null;
  const mt = O.menuTarget(), kind = M.target.kind === 'door' ? 'door' : 'objective', n = M.men.length, far = mt.kind === 'door' && M.target.kind === 'door' && mt.side !== M.target.side;
  const opener = M.page !== null ? O.defs.find((d: any) => d.opens === M.page) : undefined, where = `on ${O.hoverWords(M.target)}${far ? ' · far side' : ''}`;
  const head = opener ? `${O.labelOf(opener, mt)} · ${where}` : where, headOf = (d: any) => d.head ?? head;
  const words = (d: any) => d.opens ? `${O.labelOf(d, mt)} ▸` : (M.onWord && d.onWord ? 'wait · ' : '') + O.labelOf(d, mt);
  const rows: any[] = M.rows.map((d: any, i: number) => ({ kind, label: headOf(d), opts: [{ key: d.key ? String(d.key) : d.hotKey ? keyName(d.hotKey) : '', text: words(d) + (O.posture?.mark(d) ?? '') + (M.why[i] ? ` — ${M.why[i]}` : '') }], inReach: n > 0 && !M.why[i], off: !!M.why[i], cont: i > 0 && headOf(M.rows[i - 1]) === headOf(d) }));
  if (!rows.length) rows.push({ kind, label: head, opts: [{ key: '', text: 'no order fits' }], inReach: false, off: false });
  if (M.wordRow) { const digit = /^Digit([1-5])$/.exec(O.knobs.waitKey)?.[1], free = digit && !M.rows.some((d: any) => d.key === +digit), alt = O.knobs.waitAlt ? keyName(O.knobs.waitAlt) : ''; rows.push({ kind, label: 'on your word', opts: [{ key: free ? digit : alt, text: `wait for my word${free && alt ? ` (${alt})` : ''} · ${M.onWord ? 'ON' : 'off'}` }], inReach: n > 0, off: false }); }
  return { rows };
}
