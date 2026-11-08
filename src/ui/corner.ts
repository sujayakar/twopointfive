// THE CORNER PANEL IS THREE THINGS (game/handring.ts HANDRING.corner): the objective, what a quick press of his own key does, what a quick press of
// the squad's key does. Each is there only while it has something to say, and nothing else is: no heading over a row (the thing's name and state),
// no line per man of the squad, no counts of what the squad carries, no alternative under a key's line.
//
// This file is the reduction, with no page under it (ui/overlay.ts feeds it the rows it would have listed and draws what comes back;
// handring-check asks it directly). Whether the panel is these three things at all is the view's (`interact.corner`: a tour's puppet keeps the list as it was).

import type { ThingKind } from '../view/read';

/** One block of the interaction panel — one ROW of the list (a verb Space can do, or a running one): what the thing is (`label`, shown only on the first row of a thing — `cont` marks the rows of the same
 *  thing under it), the key → action pair plus hint lines (`opts`; key '' = a status / hint line such as 'picking… 40%' or 'or sprint into it'), whether it is within reach (else greyed, with `dist` in
 *  metres when known), `off` (a thrown breaker), and `progress` 0‥1 for a bar (a lock being picked, the drive coming out, the leaf easing round). */
export interface InteractRow { kind: ThingKind; label: string; opts: { key: string; text: string; tone?: string; /** a squad man down: his line is struck */ gone?: boolean }[]; inReach: boolean; off: boolean; dist?: number; progress?: number; cont?: boolean; /** the squad's element this row is about (orders.ts Tone: gold / red / blue) — #interact .t-<tone> */ tone?: string;
  /** a block under no heading: a line a key gives, or the lines no key gives (its label alone where it has none) · the objective's section · the clock in front of the objective */ one?: boolean; sec?: boolean; note?: string; /** … and the mission's stage, for its colour */ stage?: string }

/** The objective line in its parts (null: none). The line's own word ends on '▸' ('objective ▸ …', 'training 2/7 ▸ …' — the Rust game's
 *  `mission::objective`, in the view): `label` is that word, `note` what stands in front of it (a par or a clock), `text` the rest. A line with no such word (a run's end:
 *  'structure taken · …', 'mission failed — …') is all text, under no label. */
export function objectiveParts(line: string): { label: string; note: string; text: string } | null {
  if (!line.trim()) return null; const cut = line.indexOf('▸'); if (cut < 0) return { label: '', note: '', text: line.trim() };
  const head = line.slice(0, cut).trim(), at = Math.max(head.lastIndexOf('·'), head.lastIndexOf('—'));
  return { label: head.slice(at + 1).trim(), note: at < 0 ? '' : head.slice(0, at).trim(), text: line.slice(cut + 1).trim() };
}
type Opt = InteractRow['opts'][number];
/** The corner as HANDRING.corner has it. `mine`: his own rows as the panel listed them (the highlighted row, a running verb's, a line no key gives,
 *  the word an empty tap left) · `primary`: which of them is the highlighted one (-1 none) · `squad`: the squad's block (a man's line, the notes, the quick key's: game.view.squad) ·
 *  `notes`: what the block and a door's heading say that the corner would lose, as a key's line or a warning under none (game.view.corner) · `objective`: the mission's line ('' =
 *  none, or a lesson's own line is up) · `stage`: the mission's stage, for the section's colour ('done', 'failed').
 *  Back: the objective as a section; then of his rows every line a key gives (the key's own, the two clicks on a man in reach) each by itself, and a
 *  row no key gives whole under no heading (a running verb's words and its bar, a committed move's, the word an empty tap left with how far); then
 *  the squad's lines a key gives (the quick key's, a plan's go, a charge to blow) and a set charge's warning; then its men lost, struck as they were (deathrecap.ts). */
export function cornerRows(mine: InteractRow[], primary: number, squad: InteractRow[], objective: string, notes: Opt[] = [], stage = ''): { rows: InteractRow[]; primary: number } {
  const rows: InteractRow[] = []; let lit = -1; const O = objectiveParts(objective);
  if (O) rows.push({ kind: 'objective', label: O.label, opts: [{ key: '', text: O.text }], inReach: true, off: false, sec: true, note: O.note, stage });
  mine.forEach((r, i) => {
    if (r.kind === 'cancel') return;
    const keyed = r.opts.filter(o => o.key), base = { kind: r.kind, inReach: r.inReach, off: r.off, one: true };
    if (!keyed.length) { rows.push({ ...base, label: r.opts.length ? '' : r.label, opts: r.opts, progress: r.progress, dist: r.dist }); return; }
    keyed.forEach((o, j) => { if (i === primary && j === 0) lit = rows.length; rows.push({ ...base, label: '', opts: [o], progress: j === 0 ? r.progress : undefined }); });
  });
  const sq = squad.flatMap(r => r.opts), line = (o: Opt): InteractRow => ({ kind: 'objective', label: '', opts: [{ key: o.key, text: o.text }], inReach: true, off: false, one: true });
  for (const o of notes) rows.push(line(o));
  for (const o of sq) if (o.key && !o.gone) rows.push(line(o));
  const lost = sq.filter(o => o.gone); if (lost.length) rows.push({ kind: 'objective', label: '', opts: lost, inReach: true, off: false, one: true });
  return { rows, primary: lit };
}
